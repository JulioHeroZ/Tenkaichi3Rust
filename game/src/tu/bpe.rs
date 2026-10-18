#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Bpe_Decode(c: &mut Ctx, mut p_src_1: u32, mut p_dst_2: u32, mut p_rawSize_3: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Bpe_Decode", &PROBE_N, &[(p_src_1) as u32, (p_dst_2) as u32, (p_rawSize_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(768) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 768);
    let mut l_size_4: i32 = 0;
    let mut l_packSize_5: i32 = 0;
    let mut l_in_6: u32 = 0;
    let mut l_out_7: u32 = 0;
    let mut l_c_8: i16 = 0;
    let mut l_count_9: i16 = 0;
    let mut l_i_10: i16 = 0;
    let mut l_blockSize_11: i16 = 0;
    let mut l_hi_12: i32 = 0;
    let mut l_buf_13: u32 = 0;
    let r_ = 'f: {
        l_size_4 = (c.m.r32(p_src_1) as i32);
        p_src_1 = (p_src_1).wrapping_add((((1i32) as u32)).wrapping_mul(4));
        l_packSize_5 = (c.m.r32(p_src_1) as i32);
        p_src_1 = (p_src_1).wrapping_add((((1i32) as u32)).wrapping_mul(4));
        if (((((p_rawSize_3) != (((0i32) as u32))) as i32)) != 0) {
            { let v_ = l_size_4; let a_ = p_rawSize_3; c.m.w32(a_, (v_) as u32); };
        }
        if (((((p_dst_2) != (((0i32) as u32))) as i32)) != 0) {
            l_buf_13 = p_dst_2;
        } else {
            l_buf_13 = F::Heap_Alloc(c, l_size_4, ((32i32) as u32), 0i32, 2i32);
        }
        l_out_7 = l_buf_13;
        l_in_6 = p_src_1;
        'l1: while (((((((l_in_6).wrapping_sub(p_src_1) as i32)) != (l_packSize_5)) as i32)) != 0) {
            l_count_9 = ((c.m.r8({ let o_ = l_in_6; let n_ = (o_).wrapping_add(1); l_in_6 = n_; o_ })) as i16);
            {
                l_i_10 = ((0i32) as i16);
                'l2: loop {
                    if !(((((((l_i_10) as i32)) < (256i32)) as i32)) != 0) { break 'l2; }
                    'c3: {
                        { let v_ = ((l_i_10) as u8); let a_ = (fp).wrapping_add(((l_i_10) as u32)); c.m.w8(a_, v_); };
                    }
                    { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; };
                }
            }
            l_c_8 = ((0i32) as i16);
            {
                'l4: loop {
                    'c5: {
                        if (((((((l_count_9) as i32)) > (127i32)) as i32)) != 0) {
                            { let r_ = (((l_count_9) as i32)).wrapping_sub(127i32); let n_ = (((((l_c_8) as i32)).wrapping_add(r_)) as i16); l_c_8 = n_; };
                            l_count_9 = ((0i32) as i16);
                        }
                        if (((((((l_c_8) as i32)) == (256i32)) as i32)) != 0) {
                            break 'l4;
                        }
                        {
                            l_i_10 = ((0i32) as i16);
                            'l6: loop {
                                if !(((((((l_i_10) as i32)) <= (((l_count_9) as i32))) as i32)) != 0) { break 'l6; }
                                'c7: {
                                    { let v_ = c.m.r8({ let o_ = l_in_6; let n_ = (o_).wrapping_add(1); l_in_6 = n_; o_ }); let a_ = (fp).wrapping_add(((l_c_8) as u32)); c.m.w8(a_, v_); };
                                    if (((((((l_c_8) as i32)) != (((c.m.r8((fp).wrapping_add(((l_c_8) as u32)))) as i32))) as i32)) != 0) {
                                        { let v_ = c.m.r8({ let o_ = l_in_6; let n_ = (o_).wrapping_add(1); l_in_6 = n_; o_ }); let a_ = (fp.wrapping_add(256)).wrapping_add(((l_c_8) as u32)); c.m.w8(a_, v_); };
                                    }
                                }
                                { { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; }; { let o_ = l_c_8; let n_ = (o_).wrapping_add(1); l_c_8 = n_; }; };
                            }
                        }
                        if (((((((l_c_8) as i32)) == (256i32)) as i32)) != 0) {
                            break 'l4;
                        }
                        l_count_9 = ((c.m.r8({ let o_ = l_in_6; let n_ = (o_).wrapping_add(1); l_in_6 = n_; o_ })) as i16);
                    }
                }
            }
            l_hi_12 = ((c.m.r8({ let o_ = l_in_6; let n_ = (o_).wrapping_add(1); l_in_6 = n_; o_ })) as i32);
            l_blockSize_11 = (((((c.m.r8({ let o_ = l_in_6; let n_ = (o_).wrapping_add(1); l_in_6 = n_; o_ })) as i32)).wrapping_add((l_hi_12).wrapping_shl((8i32) as u32))) as i16);
            l_i_10 = ((0i32) as i16);
            {
                'l8: loop {
                    'c9: {
                        if (((((((l_i_10) as i32)) != (0i32)) as i32)) != 0) {
                            l_c_8 = ((c.m.r8((fp.wrapping_add(512)).wrapping_add((({ let o_ = l_i_10; let n_ = (o_).wrapping_sub(1); l_i_10 = n_; n_ }) as u32)))) as i16);
                        } else {
                            if ((((((({ let o_ = l_blockSize_11; let n_ = (o_).wrapping_sub(1); l_blockSize_11 = n_; o_ }) as i32)) == (0i32)) as i32)) != 0) {
                                break 'l8;
                            }
                            l_c_8 = ((c.m.r8({ let o_ = l_in_6; let n_ = (o_).wrapping_add(1); l_in_6 = n_; o_ })) as i16);
                        }
                        if (((((((l_c_8) as i32)) == (((c.m.r8((fp).wrapping_add(((l_c_8) as u32)))) as i32))) as i32)) != 0) {
                            { let v_ = ((l_c_8) as u8); let a_ = { let o_ = l_out_7; let n_ = (o_).wrapping_add(1); l_out_7 = n_; o_ }; c.m.w8(a_, v_); };
                        } else {
                            { let v_ = c.m.r8((fp.wrapping_add(256)).wrapping_add(((l_c_8) as u32))); let a_ = (fp.wrapping_add(512)).wrapping_add((({ let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; o_ }) as u32)); c.m.w8(a_, v_); };
                            { let v_ = c.m.r8((fp).wrapping_add(((l_c_8) as u32))); let a_ = (fp.wrapping_add(512)).wrapping_add((({ let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; o_ }) as u32)); c.m.w8(a_, v_); };
                        }
                    }
                }
            }
        }
        break 'f l_buf_13;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Bpe_Decode", &PROBE_N, &[], true); }
    r_
}
