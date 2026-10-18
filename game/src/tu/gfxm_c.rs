#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Flash_CxformIdentity(c: &mut Ctx, mut p_cx_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_CxformIdentity", &PROBE_N, &[(p_cx_1) as u32], false); }
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = 0i32;
        'l1: loop {
            if !(((((l_i_2) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = F32(0x3f800000); let a_ = (p_cx_1).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                { let v_ = ((0i32) as i16); let a_ = ((p_cx_1).wrapping_add(16)).wrapping_add((((l_i_2) as u32)).wrapping_mul(2)); c.m.w16(a_, (v_) as u16); };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Flash_CxformIdentity", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_ReadCxform(c: &mut Ctx, mut p_data_1: u32, mut p_cx_2: u32, mut p_ofs_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_ReadCxform", &PROBE_N, &[(p_data_1) as u32, (p_cx_2) as u32, (p_ofs_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_flags_4: u8 = 0;
    let mut l_bits_5: u8 = 0;
    let mut l_i_6: u32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        F::Flash_CxformIdentity(c, p_cx_2);
        l_flags_4 = { let a_ = (p_data_1).wrapping_add((((c.m.r32(p_ofs_3) as i32)) as u32)); c.m.r8(a_) };
        { let r_ = 1i32; let a_ = p_ofs_3; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
        if (((((((l_flags_4) as i32)) != (0i32)) as i32)) != 0) {
            l_bits_5 = (({ let a0_ = (p_data_1).wrapping_add((((c.m.r32(p_ofs_3) as i32)) as u32)); F::Flash_ReadBits(c, a0_, fp, ((4i32) as u8)) }) as u8);
            if ((((((l_flags_4) as i32)) & (15i32))) != 0) {
                {
                    l_i_6 = ((0i32) as u32);
                    'l1: loop {
                        if !(((((l_i_6) < (((4i32) as u32))) as i32)) != 0) { break 'l1; }
                        'c2: {
                            if (((((((l_flags_4) as i32)).wrapping_shr((l_i_6) as u32)) & (1i32))) != 0) {
                                { let v_ = rt::fmul(rt::i2f((((({ let a0_ = (p_data_1).wrapping_add((((c.m.r32(p_ofs_3) as i32)) as u32)); F::Flash_ReadSBits(c, a0_, fp, l_bits_5) }) as i16)) as i32)), F32(0x3b800000)); let a_ = (p_cx_2).wrapping_add((l_i_6).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                            }
                        }
                        { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                    }
                }
            }
            if ((((((l_flags_4) as i32)) & (240i32))) != 0) {
                {
                    l_i_6 = ((0i32) as u32);
                    'l3: loop {
                        if !(((((l_i_6) < (((4i32) as u32))) as i32)) != 0) { break 'l3; }
                        'c4: {
                            if ((((((l_flags_4) as u32)) & ((16u32).wrapping_shl((l_i_6) as u32)))) != 0) {
                                { let v_ = (({ let a0_ = (p_data_1).wrapping_add((((c.m.r32(p_ofs_3) as i32)) as u32)); F::Flash_ReadSBits(c, a0_, fp, l_bits_5) }) as i16); let a_ = ((p_cx_2).wrapping_add(16)).wrapping_add((l_i_6).wrapping_mul(2)); c.m.w16(a_, (v_) as u16); };
                            }
                        }
                        { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                    }
                }
            }
        }
        { let r_ = (c.m.r32(fp)).wrapping_shr((3i32) as u32); let a_ = p_ofs_3; let n_ = ((((((c.m.r32(a_) as i32)) as u32)).wrapping_add(r_)) as i32); c.m.w32(a_, (n_) as u32); };
        if ((((c.m.r32(fp)) & (((7i32) as u32)))) != 0) {
            { let r_ = 1i32; let a_ = p_ofs_3; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Flash_ReadCxform", &PROBE_N, &[], true); }
    r_
}

pub fn FlashTl_RunFrame(c: &mut Ctx, mut p_tl_1: u32, mut p_frame_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashTl_RunFrame", &PROBE_N, &[(p_tl_1) as u32, (p_frame_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut sv_3: i32 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
        'fl1: loop {
            match bb_ {
                0 => {
                    { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((0i32) as u16); let a_ = fp; c.m.w16(a_, v_); };
                    { let v_ = ((0i32) as u16); let a_ = fp.wrapping_add(4); c.m.w16(a_, v_); };
                    if (((((p_frame_2) == (((0i32) as u32))) as i32)) != 0) { bb_ = 1; continue 'fl1; } else { bb_ = 2; continue 'fl1; }
                }
                1 => {
                    bb_ = 3; continue 'fl1;
                }
                2 => {
                    bb_ = 5; continue 'fl1;
                }
                3 => {
                    { let r_ = (!(1i32)); let a_ = (p_tl_1).wrapping_add(21); let n_ = ((((((c.m.r8(a_)) as i32)) & (r_))) as u8); c.m.w8(a_, n_); };
                    if (((((c.m.r32((p_tl_1).wrapping_add(12))) != (((0i32) as u32))) as i32)) != 0) { bb_ = 10; continue 'fl1; } else { bb_ = 11; continue 'fl1; }
                }
                4 => {
                    bb_ = 2; continue 'fl1;
                }
                5 => {
                    if ((((((({ let a_ = (p_frame_2).wrapping_add((({ let a_ = fp.wrapping_add(8); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); o_ }) as u32)); c.m.r8(a_) }) as i32)) != (0i32)) as i32)) != 0) { bb_ = 6; continue 'fl1; } else { bb_ = 7; continue 'fl1; }
                }
                6 => {
                    bb_ = 5; continue 'fl1;
                }
                7 => {
                    { let _ = { let a1_ = (p_frame_2).wrapping_add((((c.m.r32(fp.wrapping_add(8)) as i32)) as u32)); F::memcpy(c, fp, a1_, ((2i32) as u32)) }; };
                    { let r_ = 2i32; let a_ = fp.wrapping_add(8); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                    if (((((((c.m.r16(fp)) as i32)) == (0i32)) as i32)) != 0) { bb_ = 8; continue 'fl1; } else { bb_ = 9; continue 'fl1; }
                }
                8 => {
                    bb_ = 3; continue 'fl1;
                }
                9 => {
                    bb_ = 13; continue 'fl1;
                }
                10 => {
                    { let a_ = (p_tl_1).wrapping_add(12); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, n_); };
                    bb_ = 11; continue 'fl1;
                }
                11 => {
                    break 'f;
                }
                12 => {
                    bb_ = 9; continue 'fl1;
                }
                13 => {
                    { let _ = { let a1_ = (p_frame_2).wrapping_add((((c.m.r32(fp.wrapping_add(8)) as i32)) as u32)); F::memcpy(c, fp.wrapping_add(4), a1_, ((2i32) as u32)) }; };
                    { let r_ = 2i32; let a_ = fp.wrapping_add(8); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                    sv_3 = ((c.m.r16(fp.wrapping_add(4))) as i32);
                    bb_ = match sv_3 { 4i32 => 18, 26i32 => 20, 5i32 => 22, 28i32 => 24, 12i32 => 26, 1i32 => 28, _ => 16, }; continue 'fl1;
                }
                14 => {
                    if (((((((c.m.r16(fp.wrapping_add(4))) as i32)) != (1i32)) as i32)) != 0) { bb_ = 13; continue 'fl1; } else { bb_ = 15; continue 'fl1; }
                }
                15 => {
                    break 'fl1;
                }
                16 => {
                    bb_ = 14; continue 'fl1;
                }
                17 => {
                    bb_ = 18; continue 'fl1;
                }
                18 => {
                    F::FlashTl_TagPlace(c, p_tl_1, p_frame_2, fp.wrapping_add(8));
                    bb_ = 16; continue 'fl1;
                }
                19 => {
                    bb_ = 20; continue 'fl1;
                }
                20 => {
                    F::FlashTl_TagPlace2(c, p_tl_1, p_frame_2, fp.wrapping_add(8));
                    bb_ = 16; continue 'fl1;
                }
                21 => {
                    bb_ = 22; continue 'fl1;
                }
                22 => {
                    F::FlashTl_TagRemove(c, p_tl_1, p_frame_2, fp.wrapping_add(8));
                    bb_ = 16; continue 'fl1;
                }
                23 => {
                    bb_ = 24; continue 'fl1;
                }
                24 => {
                    F::FlashTl_TagRemove2(c, p_tl_1, p_frame_2, fp.wrapping_add(8));
                    bb_ = 16; continue 'fl1;
                }
                25 => {
                    bb_ = 26; continue 'fl1;
                }
                26 => {
                    F::FlashTl_TagActions(c, p_tl_1, p_frame_2, fp.wrapping_add(8));
                    bb_ = 16; continue 'fl1;
                }
                27 => {
                    bb_ = 28; continue 'fl1;
                }
                28 => {
                    bb_ = 16; continue 'fl1;
                }
                29 => {
                    bb_ = 16; continue 'fl1;
                }
                _ => unreachable!(),
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "FlashTl_RunFrame", &PROBE_N, &[], true); }
    r_
}

pub fn FlashTl_TagPlace(c: &mut Ctx, mut p_tl_1: u32, mut p_data_2: u32, mut p_ofs_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashTl_TagPlace", &PROBE_N, &[(p_tl_1) as u32, (p_data_2) as u32, (p_ofs_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_kind_4: u8 = 0;
    let r_ = 'f: {
        { let _ = { let a1_ = (p_data_2).wrapping_add((((c.m.r32(p_ofs_3) as i32)) as u32)); F::memcpy(c, fp.wrapping_add(60), a1_, ((2i32) as u32)) }; };
        { let r_ = 2i32; let a_ = p_ofs_3; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
        l_kind_4 = { let a_ = (p_data_2).wrapping_add((((c.m.r32(p_ofs_3) as i32)) as u32)); c.m.r8(a_) };
        { let r_ = 1i32; let a_ = p_ofs_3; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
        { let _ = { let a1_ = (p_data_2).wrapping_add((((c.m.r32(p_ofs_3) as i32)) as u32)); F::memcpy(c, fp.wrapping_add(64), a1_, ((2i32) as u32)) }; };
        { let r_ = 2i32; let a_ = p_ofs_3; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
        's1: {
            let sv5: i32 = ((l_kind_4) as i32);
            match sv5 {
                4i32 => {
                    F::Flash_ReadMtx(c, p_data_2, fp, p_ofs_3);
                    F::Flash_CxformIdentity(c, fp.wrapping_add(36));
                    { let a0_ = (c.m.r32((p_tl_1).wrapping_add(4))).wrapping_add(((((((c.m.r16(fp.wrapping_add(60))) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(8)); let a1_ = c.m.r16(fp.wrapping_add(64)); let a4_ = c.m.r32(p_tl_1); F::FlashSlot_SetShape(c, a0_, a1_, fp, fp.wrapping_add(36), a4_) };
                    break 's1;
                }
                6i32 => {
                    F::Flash_ReadMtx(c, p_data_2, fp, p_ofs_3);
                    F::Flash_ReadCxform(c, p_data_2, fp.wrapping_add(36), p_ofs_3);
                    { let a0_ = (c.m.r32((p_tl_1).wrapping_add(4))).wrapping_add(((((((c.m.r16(fp.wrapping_add(60))) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(8)); let a1_ = c.m.r16(fp.wrapping_add(64)); let a5_ = c.m.r32(p_tl_1); F::FlashSlot_SetClip(c, a0_, a1_, ((0i32) as u32), fp, fp.wrapping_add(36), a5_) };
                    break 's1;
                }
                _ => {
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "FlashTl_TagPlace", &PROBE_N, &[], true); }
    r_
}

pub fn FlashTl_TagPlace2(c: &mut Ctx, mut p_tl_1: u32, mut p_data_2: u32, mut p_ofs_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashTl_TagPlace2", &PROBE_N, &[(p_tl_1) as u32, (p_data_2) as u32, (p_ofs_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(144) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 144);
    let mut l_flags_4: u8 = 0;
    let mut l_kind_5: u8 = 0;
    let mut l_pm_6: u32 = 0;
    let mut l_pc_7: u32 = 0;
    let r_ = 'f: {
        F::Flash_MtxIdentity(c, fp);
        F::Flash_CxformIdentity(c, fp.wrapping_add(36));
        l_flags_4 = { let a_ = (p_data_2).wrapping_add((((c.m.r32(p_ofs_3) as i32)) as u32)); c.m.r8(a_) };
        { let r_ = 1i32; let a_ = p_ofs_3; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
        { let _ = { let a1_ = (p_data_2).wrapping_add((((c.m.r32(p_ofs_3) as i32)) as u32)); F::memcpy(c, fp.wrapping_add(124), a1_, ((2i32) as u32)) }; };
        { let r_ = 2i32; let a_ = p_ofs_3; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
        if ((((((l_flags_4) as i32)) & (2i32))) != 0) {
            l_kind_5 = { let a_ = (p_data_2).wrapping_add((((c.m.r32(p_ofs_3) as i32)) as u32)); c.m.r8(a_) };
            { let r_ = 1i32; let a_ = p_ofs_3; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
            { let _ = { let a1_ = (p_data_2).wrapping_add((((c.m.r32(p_ofs_3) as i32)) as u32)); F::memcpy(c, fp.wrapping_add(128), a1_, ((2i32) as u32)) }; };
            { let r_ = 2i32; let a_ = p_ofs_3; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
            if ((((((l_flags_4) as i32)) & (4i32))) != 0) {
                F::Flash_ReadMtx(c, p_data_2, fp, p_ofs_3);
            }
            if ((((((l_flags_4) as i32)) & (8i32))) != 0) {
                F::Flash_ReadCxform(c, p_data_2, fp.wrapping_add(36), p_ofs_3);
            }
            if ((((((l_flags_4) as i32)) & (32i32))) != 0) {
                { let _ = F::memset(c, fp.wrapping_add(60), 0i32, 64u32); };
                F::Flash_ReadString(c, fp.wrapping_add(60), p_data_2, p_ofs_3);
            }
            's1: {
                let sv8: i32 = ((l_kind_5) as i32);
                match sv8 {
                    4i32 => {
                        { let a0_ = (c.m.r32((p_tl_1).wrapping_add(4))).wrapping_add(((((((c.m.r16(fp.wrapping_add(124))) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(8)); let a1_ = c.m.r16(fp.wrapping_add(128)); let a4_ = c.m.r32(p_tl_1); F::FlashSlot_SetShape(c, a0_, a1_, fp, fp.wrapping_add(36), a4_) };
                        break 's1;
                    }
                    6i32 => {
                        { let a0_ = (c.m.r32((p_tl_1).wrapping_add(4))).wrapping_add(((((((c.m.r16(fp.wrapping_add(124))) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(8)); let a1_ = c.m.r16(fp.wrapping_add(128)); let a5_ = c.m.r32(p_tl_1); F::FlashSlot_SetClip(c, a0_, a1_, fp.wrapping_add(60), fp, fp.wrapping_add(36), a5_) };
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
        } else {
            l_pm_6 = ((0i32) as u32);
            l_pc_7 = ((0i32) as u32);
            if ((((((l_flags_4) as i32)) & (4i32))) != 0) {
                F::Flash_ReadMtx(c, p_data_2, fp, p_ofs_3);
                l_pm_6 = fp;
            }
            if ((((((l_flags_4) as i32)) & (8i32))) != 0) {
                F::Flash_ReadCxform(c, p_data_2, fp.wrapping_add(36), p_ofs_3);
                l_pc_7 = fp.wrapping_add(36);
            }
            { let a0_ = (c.m.r32((p_tl_1).wrapping_add(4))).wrapping_add(((((((c.m.r16(fp.wrapping_add(124))) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(8)); F::FlashSlot_Move(c, a0_, l_pm_6, l_pc_7) };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "FlashTl_TagPlace2", &PROBE_N, &[], true); }
    r_
}

pub fn FlashTl_TagRemove(c: &mut Ctx, mut p_tl_1: u32, mut p_data_2: u32, mut p_ofs_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashTl_TagRemove", &PROBE_N, &[(p_tl_1) as u32, (p_data_2) as u32, (p_ofs_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_kind_4: u8 = 0;
    let mut l_slot_5: u32 = 0;
    let r_ = 'f: {
        { let _ = { let a1_ = (p_data_2).wrapping_add((((c.m.r32(p_ofs_3) as i32)) as u32)); F::memcpy(c, fp, a1_, ((2i32) as u32)) }; };
        { let r_ = 2i32; let a_ = p_ofs_3; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
        l_kind_4 = { let a_ = (p_data_2).wrapping_add((((c.m.r32(p_ofs_3) as i32)) as u32)); c.m.r8(a_) };
        { let r_ = 1i32; let a_ = p_ofs_3; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
        { let _ = { let a1_ = (p_data_2).wrapping_add((((c.m.r32(p_ofs_3) as i32)) as u32)); F::memcpy(c, fp.wrapping_add(4), a1_, ((2i32) as u32)) }; };
        { let r_ = 2i32; let a_ = p_ofs_3; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
        l_slot_5 = (c.m.r32((p_tl_1).wrapping_add(4))).wrapping_add(((((((c.m.r16(fp)) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(8));
        's1: {
            let sv6: i32 = ((l_kind_4) as i32);
            match sv6 {
                4i32 => {
                    if (((((c.m.r32(l_slot_5)) != (((0i32) as u32))) as i32)) != 0) {
                        { let a0_ = c.m.r32(l_slot_5); F::FlashShape_Free(c, a0_) };
                        { let v_ = ((0i32) as u32); let a_ = l_slot_5; c.m.w32(a_, v_); };
                    }
                    break 's1;
                }
                6i32 => {
                    if (((((c.m.r32((l_slot_5).wrapping_add(4))) != (((0i32) as u32))) as i32)) != 0) {
                        { let a0_ = c.m.r32((l_slot_5).wrapping_add(4)); F::FlashClip_Reset(c, a0_, 1i32) };
                        { let v_ = ((0i32) as u32); let a_ = (l_slot_5).wrapping_add(4); c.m.w32(a_, v_); };
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "FlashTl_TagRemove", &PROBE_N, &[], true); }
    r_
}

pub fn FlashTl_TagRemove2(c: &mut Ctx, mut p_tl_1: u32, mut p_data_2: u32, mut p_ofs_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashTl_TagRemove2", &PROBE_N, &[(p_tl_1) as u32, (p_data_2) as u32, (p_ofs_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let _ = { let a1_ = (p_data_2).wrapping_add((((c.m.r32(p_ofs_3) as i32)) as u32)); F::memcpy(c, fp, a1_, ((2i32) as u32)) }; };
        { let r_ = 2i32; let a_ = p_ofs_3; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
        { let a0_ = (c.m.r32((p_tl_1).wrapping_add(4))).wrapping_add(((((((c.m.r16(fp)) as i32)).wrapping_sub(1i32)) as u32)).wrapping_mul(8)); F::FlashSlot_Clear(c, a0_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "FlashTl_TagRemove2", &PROBE_N, &[], true); }
    r_
}

pub fn FlashTl_SkipActions(c: &mut Ctx, mut p_tl_1: u32, mut p_data_2: u32, mut p_ofs_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashTl_SkipActions", &PROBE_N, &[(p_tl_1) as u32, (p_data_2) as u32, (p_ofs_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(448) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 448);
    let mut l_code_4: u8 = 0;
    let r_ = 'f: {
        'l1: loop {
            'c2: {
                l_code_4 = { let a_ = (p_data_2).wrapping_add((((c.m.r32(p_ofs_3) as i32)) as u32)); c.m.r8(a_) };
                { let r_ = 1i32; let a_ = p_ofs_3; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                's3: {
                    let sv5: i32 = ((l_code_4) as i32);
                    match sv5 {
                        129i32 => {
                            { let r_ = 2i32; let a_ = p_ofs_3; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                            break 's3;
                        }
                        131i32 => {
                            { let _ = F::memset(c, fp, 0i32, 64u32); };
                            { let _ = F::memset(c, fp.wrapping_add(64), 0i32, 64u32); };
                            F::Flash_ReadString(c, fp, p_data_2, p_ofs_3);
                            F::Flash_ReadString(c, fp.wrapping_add(64), p_data_2, p_ofs_3);
                            if (((((F::strcmp(c, fp, sym::gfxm_c__str0)) == (0i32)) as i32)) != 0) {
                                if (((((F::strcmp(c, fp.wrapping_add(64), sym::gfxm_c__str1)) == (0i32)) as i32)) != 0) {
                                    { let r_ = ((2i32) as u32); let a_ = (c.m.r32(p_tl_1)).wrapping_add(8); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                                } else {
                                    if (((((F::strcmp(c, fp.wrapping_add(64), sym::gfxm_c__str2)) == (0i32)) as i32)) != 0) {
                                        { let r_ = (((!(2i32))) as u32); let a_ = (c.m.r32(p_tl_1)).wrapping_add(8); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
                                    }
                                }
                            }
                            break 's3;
                        }
                        138i32 => {
                            { let r_ = 3i32; let a_ = p_ofs_3; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                            break 's3;
                        }
                        139i32 => {
                            { let _ = F::memset(c, fp.wrapping_add(128), 0i32, 64u32); };
                            F::Flash_ReadString(c, fp.wrapping_add(128), p_data_2, p_ofs_3);
                            break 's3;
                        }
                        140i32 => {
                            { let _ = F::memset(c, fp.wrapping_add(192), 0i32, ((255i32) as u32)); };
                            F::Flash_ReadString(c, fp.wrapping_add(192), p_data_2, p_ofs_3);
                            break 's3;
                        }
                        4i32 | 5i32 | 6i32 | 7i32 => {
                            break 's3;
                        }
                        _ => {
                        }
                    }
                }
            }
            if !(((((((l_code_4) as i32)) != (0i32)) as i32)) != 0) { break 'l1; }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "FlashTl_SkipActions", &PROBE_N, &[], true); }
    r_
}

pub fn FlashTl_TagActions(c: &mut Ctx, mut p_tl_1: u32, mut p_data_2: u32, mut p_ofs_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashTl_TagActions", &PROBE_N, &[(p_tl_1) as u32, (p_data_2) as u32, (p_ofs_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(464) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 464);
    let mut l_cur_4: u32 = 0;
    let mut l_code_5: u8 = 0;
    let mut l_clip_6: u32 = 0;
    let mut l_index_7: i32 = 0;
    let r_ = 'f: {
        if ((((((c.m.r8((p_tl_1).wrapping_add(21))) as i32)) & (8i32))) != 0) {
            F::FlashTl_SkipActions(c, p_tl_1, p_data_2, p_ofs_3);
            break 'f;
        }
        l_cur_4 = p_tl_1;
        'l1: loop {
            'c2: {
                l_code_5 = { let a_ = (p_data_2).wrapping_add((((c.m.r32(p_ofs_3) as i32)) as u32)); c.m.r8(a_) };
                { let r_ = 1i32; let a_ = p_ofs_3; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                's3: {
                    let sv8: i32 = ((l_code_5) as i32);
                    match sv8 {
                        4i32 => {
                            { let v_ = (c.m.r32((l_cur_4).wrapping_add(12))).wrapping_add(((1i32) as u32)); let a_ = (l_cur_4).wrapping_add(12); c.m.w32(a_, v_); };
                            { let r_ = 2i32; let a_ = (l_cur_4).wrapping_add(21); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
                            break 's3;
                        }
                        5i32 => {
                            if (((((c.m.r32((l_cur_4).wrapping_add(12))) != (((0i32) as u32))) as i32)) != 0) {
                                { let a_ = (l_cur_4).wrapping_add(12); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, n_); };
                            }
                            { let r_ = 2i32; let a_ = (l_cur_4).wrapping_add(21); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
                            break 's3;
                        }
                        6i32 => {
                            { let r_ = 1i32; let a_ = (l_cur_4).wrapping_add(21); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
                            break 's3;
                        }
                        7i32 => {
                            { let r_ = (!(1i32)); let a_ = (l_cur_4).wrapping_add(21); let n_ = ((((((c.m.r8(a_)) as i32)) & (r_))) as u8); c.m.w8(a_, n_); };
                            break 's3;
                        }
                        129i32 => {
                            {
                                { let _ = { let a1_ = (p_data_2).wrapping_add((((c.m.r32(p_ofs_3) as i32)) as u32)); F::memcpy(c, fp, a1_, ((2i32) as u32)) }; };
                                { let r_ = 2i32; let a_ = p_ofs_3; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                                { let v_ = (((((c.m.r16(fp)) as i32)).wrapping_sub(1i32)) as u32); let a_ = (l_cur_4).wrapping_add(12); c.m.w32(a_, v_); };
                                { let r_ = 4i32; let a_ = (l_cur_4).wrapping_add(21); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
                                break 's3;
                            }
                        }
                        131i32 => {
                            {
                                { let _ = F::memset(c, fp.wrapping_add(4), 0i32, 64u32); };
                                { let _ = F::memset(c, fp.wrapping_add(68), 0i32, 64u32); };
                                F::Flash_ReadString(c, fp.wrapping_add(4), p_data_2, p_ofs_3);
                                F::Flash_ReadString(c, fp.wrapping_add(68), p_data_2, p_ofs_3);
                                if (((((F::strcmp(c, fp.wrapping_add(4), sym::gfxm_c__str3)) == (0i32)) as i32)) != 0) {
                                    if (((((F::strcmp(c, fp.wrapping_add(68), sym::gfxm_c__str4)) == (0i32)) as i32)) != 0) {
                                        { let r_ = ((2i32) as u32); let a_ = (c.m.r32(p_tl_1)).wrapping_add(8); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                                    } else {
                                        if (((((F::strcmp(c, fp.wrapping_add(68), sym::gfxm_c__str5)) == (0i32)) as i32)) != 0) {
                                            { let r_ = (((!(2i32))) as u32); let a_ = (c.m.r32(p_tl_1)).wrapping_add(8); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
                                        }
                                    }
                                } else {
                                    if (((((F::strcmp(c, fp.wrapping_add(4), sym::gfxm_c__str6)) == (0i32)) as i32)) != 0) {
                                        { let r_ = (((1i32).wrapping_shl((F::atoi(c, fp.wrapping_add(68))) as u32)) as u32); let a_ = (c.m.r32(p_tl_1)).wrapping_add(12); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                                    } else {
                                        if (((((F::strcmp(c, fp.wrapping_add(4), sym::gfxm_c__str7)) == (0i32)) as i32)) != 0) {
                                            { let r_ = (((1i32).wrapping_shl((F::atoi(c, fp.wrapping_add(68))) as u32)) as u32); let a_ = (c.m.r32(p_tl_1)).wrapping_add(16); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                                        } else {
                                            if (((((F::strcmp(c, fp.wrapping_add(4), sym::gfxm_c__str8)) == (0i32)) as i32)) != 0) {
                                                if (((((F::strcmp(c, fp.wrapping_add(68), sym::gfxm_c__str9)) == (0i32)) as i32)) != 0) {
                                                    { let r_ = ((8i32) as u32); let a_ = (c.m.r32(p_tl_1)).wrapping_add(8); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                                                }
                                            }
                                        }
                                    }
                                }
                                break 's3;
                            }
                        }
                        138i32 => {
                            { let r_ = 3i32; let a_ = p_ofs_3; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                            break 's3;
                        }
                        139i32 => {
                            {
                                { let _ = F::memset(c, fp.wrapping_add(132), 0i32, 64u32); };
                                F::Flash_ReadString(c, fp.wrapping_add(132), p_data_2, p_ofs_3);
                                if ((((((((c.m.r8((fp.wrapping_add(132)).wrapping_add(((0i32) as u32))) as i8)) as i32)) == (0i32)) as i32)) != 0) {
                                    l_cur_4 = p_tl_1;
                                } else {
                                    l_clip_6 = { let a0_ = { let a_ = (c.m.r32(p_tl_1)).wrapping_add(36); c.m.r32(a_) }; F::FlashClipList_FindByName(c, a0_, fp.wrapping_add(132)) };
                                    l_cur_4 = (if (((((l_clip_6) != (((0i32) as u32))) as i32)) != 0) { (l_clip_6).wrapping_add(204) } else { p_tl_1 });
                                }
                                break 's3;
                            }
                        }
                        140i32 => {
                            {
                                { let _ = F::memset(c, fp.wrapping_add(196), 0i32, ((255i32) as u32)); };
                                F::Flash_ReadString(c, fp.wrapping_add(196), p_data_2, p_ofs_3);
                                l_index_7 = { let a0_ = c.m.r32((l_cur_4).wrapping_add(8)); F::Flash_FindName(c, a0_, fp.wrapping_add(196)) };
                                if (((((l_index_7) >= (0i32)) as i32)) != 0) {
                                    { let v_ = ((l_index_7) as u32); let a_ = (l_cur_4).wrapping_add(12); c.m.w32(a_, v_); };
                                    { let r_ = 4i32; let a_ = (l_cur_4).wrapping_add(21); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
                                }
                                break 's3;
                            }
                        }
                        _ => {
                        }
                    }
                }
            }
            if !(((((((l_code_5) as i32)) != (0i32)) as i32)) != 0) { break 'l1; }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "FlashTl_TagActions", &PROBE_N, &[], true); }
    r_
}

pub fn FlashTl_Init(c: &mut Ctx, mut p_tl_1: u32, mut p_record_2: u32, mut p_owner_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashTl_Init", &PROBE_N, &[(p_tl_1) as u32, (p_record_2) as u32, (p_owner_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let _ = F::memset(c, fp, 0i32, 8u32); };
        { let _ = F::memset(c, p_tl_1, 0i32, 24u32); };
        { let _ = F::memcpy(c, fp, p_record_2, 8u32); };
        { let r_ = 8i32; let n_ = (p_record_2).wrapping_add(((r_) as u32)); p_record_2 = n_; };
        { let v_ = p_owner_3; let a_ = p_tl_1; c.m.w32(a_, v_); };
        { let v_ = c.m.r16((fp).wrapping_add(2)); let a_ = (p_tl_1).wrapping_add(22); c.m.w16(a_, v_); };
        { let v_ = p_record_2; let a_ = (p_tl_1).wrapping_add(8); c.m.w32(a_, v_); };
        if (((((c.m.r32((p_tl_1).wrapping_add(4))) == (((0i32) as u32))) as i32)) != 0) {
            { let v_ = { let a0_ = (((c.m.r16((p_tl_1).wrapping_add(22))) as i32)).wrapping_mul(8i32); F::Heap_Alloc(c, a0_, ((32i32) as u32), 0i32, 2i32) }; let a_ = (p_tl_1).wrapping_add(4); c.m.w32(a_, v_); };
            { let _ = { let a0_ = c.m.r32((p_tl_1).wrapping_add(4)); let a2_ = (((((c.m.r16((p_tl_1).wrapping_add(22))) as i32)).wrapping_mul(8i32)) as u32); F::memset(c, a0_, 0i32, a2_) }; };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "FlashTl_Init", &PROBE_N, &[], true); }
    r_
}

pub fn FlashTl_Term(c: &mut Ctx, mut p_tl_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashTl_Term", &PROBE_N, &[(p_tl_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32((p_tl_1).wrapping_add(4))) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32((p_tl_1).wrapping_add(4)); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (p_tl_1).wrapping_add(4); c.m.w32(a_, v_); };
    }
    { let _ = F::memset(c, p_tl_1, 0i32, 24u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashTl_Term", &PROBE_N, &[], true); }
    r_
}

pub fn FlashTl_Rewind(c: &mut Ctx, mut p_tl_1: u32, mut p_unused_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashTl_Rewind", &PROBE_N, &[(p_tl_1) as u32, (p_unused_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    { let v_ = ((0i32) as u32); let a_ = (p_tl_1).wrapping_add(16); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (p_tl_1).wrapping_add(12); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u8); let a_ = (p_tl_1).wrapping_add(20); c.m.w8(a_, v_); };
    { let v_ = ((0i32) as u8); let a_ = (p_tl_1).wrapping_add(21); c.m.w8(a_, v_); };
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (((c.m.r16((p_tl_1).wrapping_add(22))) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = (c.m.r32((p_tl_1).wrapping_add(4))).wrapping_add((((l_i_3) as u32)).wrapping_mul(8)); F::FlashSlot_Clear(c, a0_) };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashTl_Rewind", &PROBE_N, &[], true); }
    r_
}

pub fn FlashTl_Advance(c: &mut Ctx, mut p_tl_1: u32, mut p_count_2: u32, mut p_children_3: u8, mut p_force_4: u8) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashTl_Advance", &PROBE_N, &[(p_tl_1) as u32, (p_count_2) as u32, (p_children_3) as u32, (p_force_4) as u32], false); }
    let mut l_n_5: u32 = 0;
    let mut l_i_6: u32 = 0;
    let mut l_target_7: u32 = 0;
    let mut l_start_8: u32 = 0;
    let r_ = 'f: {
    {
        l_n_5 = ((0i32) as u32);
        'l1: loop {
            if !(((((l_n_5) < (p_count_2)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((c.m.r8((p_tl_1).wrapping_add(21))) as i32)) & (((1i32) | (4i32))))) != 0) {
                    if ((((((c.m.r8((p_tl_1).wrapping_add(21))) as i32)) & (4i32))) != 0) {
                        l_target_7 = c.m.r32((p_tl_1).wrapping_add(12));
                        if ((((((c.m.r8((p_tl_1).wrapping_add(21))) as i32)) & (16i32))) != 0) {
                            l_start_8 = c.m.r32((p_tl_1).wrapping_add(16));
                        } else {
                            F::FlashTl_Rewind(c, p_tl_1, 1i32);
                            l_start_8 = ((0i32) as u32);
                        }
                        { let r_ = ((1i32) | (8i32)); let a_ = (p_tl_1).wrapping_add(21); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
                        {
                            l_i_6 = l_start_8;
                            'l3: loop {
                                if !(((((l_i_6) < (l_target_7)) as i32)) != 0) { break 'l3; }
                                'c4: {
                                    { let a1_ = { let a0_ = c.m.r32((p_tl_1).wrapping_add(8)); F::Flash_SkipNamed(c, a0_, l_i_6) }; F::FlashTl_RunFrame(c, p_tl_1, a1_) };
                                }
                                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                            }
                        }
                        { let v_ = l_target_7; let a_ = (p_tl_1).wrapping_add(12); c.m.w32(a_, v_); };
                        { let r_ = (!(((((4i32) | (8i32))) | (16i32)))); let a_ = (p_tl_1).wrapping_add(21); let n_ = ((((((c.m.r8(a_)) as i32)) & (r_))) as u8); c.m.w8(a_, n_); };
                    }
                    { let a1_ = { let a0_ = c.m.r32((p_tl_1).wrapping_add(8)); let a1_ = c.m.r32((p_tl_1).wrapping_add(12)); F::Flash_SkipNamed(c, a0_, a1_) }; F::FlashTl_RunFrame(c, p_tl_1, a1_) };
                    if ((((!((((((c.m.r8((p_tl_1).wrapping_add(21))) as i32)) & (4i32))) != 0)) as i32)) != 0) {
                        if ((((!((((((c.m.r8((p_tl_1).wrapping_add(21))) as i32)) & (2i32))) != 0)) as i32)) != 0) {
                            if ((((((((c.m.r8((p_tl_1).wrapping_add(21))) as i32)) & (1i32))) as u8)) != 0) {
                                { let a_ = (p_tl_1).wrapping_add(12); let o_ = c.m.r32(a_); let n_ = (o_).wrapping_add(1); c.m.w32(a_, n_); };
                            }
                        }
                    }
                    if ((p_children_3) != 0) {
                        {
                            l_i_6 = ((0i32) as u32);
                            'l5: loop {
                                if !(((((l_i_6) < (((c.m.r16((p_tl_1).wrapping_add(22))) as u32))) as i32)) != 0) { break 'l5; }
                                'c6: {
                                    { let a0_ = (c.m.r32((p_tl_1).wrapping_add(4))).wrapping_add((l_i_6).wrapping_mul(8)); F::FlashSlot_Advance(c, a0_, p_count_2) };
                                }
                                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                            }
                        }
                    }
                } else {
                    if ((((((((p_children_3) as i32)) != 0) && ((((p_force_4) as i32)) != 0)) as i32)) != 0) {
                        {
                            l_i_6 = ((0i32) as u32);
                            'l7: loop {
                                if !(((((l_i_6) < (((c.m.r16((p_tl_1).wrapping_add(22))) as u32))) as i32)) != 0) { break 'l7; }
                                'c8: {
                                    { let a0_ = (c.m.r32((p_tl_1).wrapping_add(4))).wrapping_add((l_i_6).wrapping_mul(8)); F::FlashSlot_Advance(c, a0_, p_count_2) };
                                }
                                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                            }
                        }
                    }
                }
            }
            { let o_ = l_n_5; let n_ = (o_).wrapping_add(1); l_n_5 = n_; };
        }
    }
    if ((((!((p_children_3) != 0)) as i32)) != 0) {
        {
            l_i_6 = ((0i32) as u32);
            'l9: loop {
                if !(((((l_i_6) < (((c.m.r16((p_tl_1).wrapping_add(22))) as u32))) as i32)) != 0) { break 'l9; }
                'c10: {
                    { let a0_ = (c.m.r32((p_tl_1).wrapping_add(4))).wrapping_add((l_i_6).wrapping_mul(8)); F::FlashSlot_Advance(c, a0_, p_count_2) };
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashTl_Advance", &PROBE_N, &[], true); }
    r_
}

pub fn FlashTl_Draw(c: &mut Ctx, mut p_tl_1: u32, mut p_prop_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashTl_Draw", &PROBE_N, &[(p_tl_1) as u32, (p_prop_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (((c.m.r16((p_tl_1).wrapping_add(22))) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = (c.m.r32((p_tl_1).wrapping_add(4))).wrapping_add((((l_i_3) as u32)).wrapping_mul(8)); let a2_ = c.m.r32(p_tl_1); F::FlashSlot_Draw(c, a0_, p_prop_2, a2_) };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashTl_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn FlashTl_GotoLabel(c: &mut Ctx, mut p_tl_1: u32, mut p_label_2: u32, mut p_restart_3: u8) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashTl_GotoLabel", &PROBE_N, &[(p_tl_1) as u32, (p_label_2) as u32, (p_restart_3) as u32], false); }
    let mut l_index_4: i32 = 0;
    let r_ = 'f: {
    l_index_4 = { let a0_ = c.m.r32((p_tl_1).wrapping_add(8)); F::Flash_FindName(c, a0_, p_label_2) };
    if (((((l_index_4) >= (0i32)) as i32)) != 0) {
        { let v_ = c.m.r32((p_tl_1).wrapping_add(12)); let a_ = (p_tl_1).wrapping_add(16); c.m.w32(a_, v_); };
        { let v_ = ((l_index_4) as u32); let a_ = (p_tl_1).wrapping_add(12); c.m.w32(a_, v_); };
        { let r_ = 4i32; let a_ = (p_tl_1).wrapping_add(21); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
        if ((((!((p_restart_3) != 0)) as i32)) != 0) {
            { let r_ = 16i32; let a_ = (p_tl_1).wrapping_add(21); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashTl_GotoLabel", &PROBE_N, &[], true); }
    r_
}

pub fn FlashTl_StepFrames(c: &mut Ctx, mut p_tl_1: u32, mut p_step_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashTl_StepFrames", &PROBE_N, &[(p_tl_1) as u32, (p_step_2) as u32], false); }
    let mut l_frame_3: i32 = 0;
    let r_ = 'f: {
    l_frame_3 = (((c.m.r32((p_tl_1).wrapping_add(12))).wrapping_add(((p_step_2) as u32))) as i32);
    if (((((((((l_frame_3) >= (0i32)) as i32)) != 0) && ((((({ let a0_ = c.m.r32((p_tl_1).wrapping_add(8)); F::Flash_SkipNamed(c, a0_, ((l_frame_3) as u32)) }) != (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        { let v_ = ((l_frame_3) as u32); let a_ = (p_tl_1).wrapping_add(12); c.m.w32(a_, v_); };
        { let r_ = 1i32; let a_ = (p_tl_1).wrapping_add(21); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
        if (((((p_step_2) != (1i32)) as i32)) != 0) {
            { let r_ = 4i32; let a_ = (p_tl_1).wrapping_add(21); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashTl_StepFrames", &PROBE_N, &[], true); }
    r_
}

pub fn FlashSlot_SetShape(c: &mut Ctx, mut p_slot_1: u32, mut p_id_2: u16, mut p_mtx_3: u32, mut p_cx_4: u32, mut p_flash_5: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashSlot_SetShape", &PROBE_N, &[(p_slot_1) as u32, (p_id_2) as u32, (p_mtx_3) as u32, (p_cx_4) as u32, (p_flash_5) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(p_slot_1)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(p_slot_1); F::FlashShape_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = p_slot_1; c.m.w32(a_, v_); };
    }
    { let v_ = { let a0_ = c.m.r32((p_flash_5).wrapping_add(40)); F::FlashShape_Create(c, a0_, p_id_2, p_mtx_3, p_cx_4) }; let a_ = p_slot_1; c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashSlot_SetShape", &PROBE_N, &[], true); }
    r_
}

pub fn FlashSlot_SetClip(c: &mut Ctx, mut p_slot_1: u32, mut p_id_2: u16, mut p_name_3: u32, mut p_mtx_4: u32, mut p_cx_5: u32, mut p_flash_6: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashSlot_SetClip", &PROBE_N, &[(p_slot_1) as u32, (p_id_2) as u32, (p_name_3) as u32, (p_mtx_4) as u32, (p_cx_5) as u32, (p_flash_6) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32((p_slot_1).wrapping_add(4))) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32((p_slot_1).wrapping_add(4)); F::FlashClip_Reset(c, a0_, 1i32) };
        { let v_ = ((0i32) as u32); let a_ = (p_slot_1).wrapping_add(4); c.m.w32(a_, v_); };
    }
    { let v_ = { let a0_ = c.m.r32((p_flash_6).wrapping_add(36)); F::FlashClip_Start(c, a0_, p_id_2, p_name_3, p_mtx_4, p_cx_5) }; let a_ = (p_slot_1).wrapping_add(4); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashSlot_SetClip", &PROBE_N, &[], true); }
    r_
}

pub fn FlashSlot_Clear(c: &mut Ctx, mut p_slot_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashSlot_Clear", &PROBE_N, &[(p_slot_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(p_slot_1)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(p_slot_1); F::FlashShape_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = p_slot_1; c.m.w32(a_, v_); };
    }
    if (((((c.m.r32((p_slot_1).wrapping_add(4))) != (((0i32) as u32))) as i32)) != 0) {
        { let r_ = (((!(1i32))) as u32); let a_ = (c.m.r32((p_slot_1).wrapping_add(4))).wrapping_add(68); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
        { let v_ = ((0i32) as u32); let a_ = (p_slot_1).wrapping_add(4); c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashSlot_Clear", &PROBE_N, &[], true); }
    r_
}

pub fn FlashSlot_Move(c: &mut Ctx, mut p_slot_1: u32, mut p_mtx_2: u32, mut p_cx_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashSlot_Move", &PROBE_N, &[(p_slot_1) as u32, (p_mtx_2) as u32, (p_cx_3) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(p_slot_1)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(p_slot_1); F::FlashShape_Set(c, a0_, p_mtx_2, p_cx_3) };
    }
    if (((((c.m.r32((p_slot_1).wrapping_add(4))) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32((p_slot_1).wrapping_add(4)); F::FlashClip_Set(c, a0_, p_mtx_2, p_cx_3) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashSlot_Move", &PROBE_N, &[], true); }
    r_
}

pub fn FlashSlot_Advance(c: &mut Ctx, mut p_slot_1: u32, mut p_count_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashSlot_Advance", &PROBE_N, &[(p_slot_1) as u32, (p_count_2) as u32], false); }
    let mut l_clip_3: u32 = 0;
    let r_ = 'f: {
    l_clip_3 = c.m.r32((p_slot_1).wrapping_add(4));
    if (((((l_clip_3) != (((0i32) as u32))) as i32)) != 0) {
        if ((((c.m.r32((l_clip_3).wrapping_add(68))) & (((8i32) as u32)))) != 0) {
            F::FlashTl_Advance(c, (l_clip_3).wrapping_add(204), p_count_2, ((1i32) as u8), ((1i32) as u8));
        } else {
            F::FlashTl_Advance(c, (l_clip_3).wrapping_add(204), p_count_2, ((1i32) as u8), ((0i32) as u8));
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashSlot_Advance", &PROBE_N, &[], true); }
    r_
}

pub fn gfxm_c__Flash_Clamp255(c: &mut Ctx, mut p_v_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "gfxm_c__Flash_Clamp255", &PROBE_N, &[(p_v_1) as u32], false); }
    let r_ = 'f: {
    break 'f (if (((((p_v_1) < (0i32)) as i32)) != 0) { 0i32 } else { (if (((((p_v_1) > (255i32)) as i32)) != 0) { 255i32 } else { p_v_1 }) });
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "gfxm_c__Flash_Clamp255", &PROBE_N, &[], true); }
    r_
}

pub fn gfxm_c__Flash_ClampAdd(c: &mut Ctx, mut p_v_1: i32) -> i16 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "gfxm_c__Flash_ClampAdd", &PROBE_N, &[(p_v_1) as u32], false); }
    let r_ = 'f: {
    if (((((p_v_1) >= (0i32)) as i32)) != 0) {
        if (((((p_v_1) > (255i32)) as i32)) != 0) {
            p_v_1 = 255i32;
        }
        break 'f ((p_v_1) as i16);
    }
    break 'f ((0i32) as i16);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "gfxm_c__Flash_ClampAdd", &PROBE_N, &[], true); }
    r_
}

pub fn FlashSlot_Draw(c: &mut Ctx, mut p_slot_1: u32, mut p_parent_2: u32, mut p_flash_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashSlot_Draw", &PROBE_N, &[(p_slot_1) as u32, (p_parent_2) as u32, (p_flash_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(144) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 144);
    let mut l_clip_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let mut l_j_6: i32 = 0;
    let r_ = 'f: {
        l_clip_4 = c.m.r32((p_slot_1).wrapping_add(4));
        if (((((c.m.r32(p_slot_1)) != (((0i32) as u32))) as i32)) != 0) {
            { let a0_ = c.m.r32(p_slot_1); F::FlashShape_Draw(c, a0_, p_parent_2, p_flash_3) };
        }
        if (((((((((l_clip_4) != (((0i32) as u32))) as i32)) != 0) && ((((c.m.r32((l_clip_4).wrapping_add(68))) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
            { let s_ = (l_clip_4).wrapping_add(72); let d_ = fp; rt::memcpy(c, d_, s_, 108); };
            if (((((p_parent_2) != (((0i32) as u32))) as i32)) != 0) {
                { let r_ = ((c.m.r32(p_parent_2)) & (((((4096i32) | (8192i32))) as u32))); let a_ = fp; let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                {
                    l_i_5 = 0i32;
                    'l1: loop {
                        if !(((((l_i_5) < (3i32)) as i32)) != 0) { break 'l1; }
                        'c2: {
                            {
                                l_j_6 = 0i32;
                                'l3: loop {
                                    if !(((((l_j_6) < (3i32)) as i32)) != 0) { break 'l3; }
                                    'c4: {
                                        { let v_ = rt::fadd(rt::fadd(rt::fmul(F32(c.m.r32(((fp).wrapping_add(48)).wrapping_add((((l_j_6) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_parent_2).wrapping_add(48)).wrapping_add(((((l_i_5).wrapping_mul(3i32)) as u32)).wrapping_mul(4))))), rt::fmul(F32(c.m.r32(((fp).wrapping_add(48)).wrapping_add(((((3i32).wrapping_add(l_j_6)) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_parent_2).wrapping_add(48)).wrapping_add((((((l_i_5).wrapping_mul(3i32)).wrapping_add(1i32)) as u32)).wrapping_mul(4)))))), rt::fmul(F32(c.m.r32(((fp).wrapping_add(48)).wrapping_add(((((6i32).wrapping_add(l_j_6)) as u32)).wrapping_mul(4)))), F32(c.m.r32(((p_parent_2).wrapping_add(48)).wrapping_add((((((l_i_5).wrapping_mul(3i32)).wrapping_add(2i32)) as u32)).wrapping_mul(4)))))); let a_ = (fp.wrapping_add(108)).wrapping_add((((((l_i_5).wrapping_mul(3i32)).wrapping_add(l_j_6)) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                    }
                                    { let o_ = l_j_6; let n_ = (o_).wrapping_add(1); l_j_6 = n_; };
                                }
                            }
                        }
                        { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
                    }
                }
                { let s_ = fp.wrapping_add(108); let d_ = (fp).wrapping_add(48); rt::memcpy(c, d_, s_, 36); };
                if ((((c.m.r32(p_parent_2)) & (((2i32) as u32)))) != 0) {
                    { let r_ = F32(c.m.r32((p_parent_2).wrapping_add(4))); let a_ = ((fp).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    { let r_ = F32(c.m.r32((p_parent_2).wrapping_add(8))); let a_ = ((fp).wrapping_add(48)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                }
                {
                    l_i_5 = 0i32;
                    'l5: loop {
                        if !(((((l_i_5) < (4i32)) as i32)) != 0) { break 'l5; }
                        'c6: {
                            { let r_ = F32(c.m.r32(((p_parent_2).wrapping_add(84)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)))); let a_ = ((fp).wrapping_add(84)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            if (((((l_i_5) < (3i32)) as i32)) != 0) {
                                if ((((c.m.r32(p_parent_2)) & (((32i32) as u32)))) != 0) {
                                    { let r_ = F32(c.m.r32((p_parent_2).wrapping_add(24))); let a_ = ((fp).wrapping_add(84)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                }
                            } else {
                                if ((((c.m.r32(p_parent_2)) & (((8i32) as u32)))) != 0) {
                                    { let r_ = F32(c.m.r32((p_parent_2).wrapping_add(12))); let a_ = ((fp).wrapping_add(84)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); let n_ = rt::fmul(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                                }
                            }
                            if (((rt::fgt(F32(c.m.r32(((fp).wrapping_add(84)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)))), F32(0x3f800000)) as i32)) != 0) {
                                { let v_ = F32(0x3f800000); let a_ = ((fp).wrapping_add(84)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                            } else {
                                if (((rt::flt(F32(c.m.r32(((fp).wrapping_add(84)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)))), F32(0x00000000)) as i32)) != 0) {
                                    { let v_ = F32(0x00000000); let a_ = ((fp).wrapping_add(84)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                                }
                            }
                            { let v_ = {  { let a0_ = ((((c.m.r16((((fp).wrapping_add(84)).wrapping_add(16)).wrapping_add((((l_i_5) as u32)).wrapping_mul(2))) as i16)) as i32)).wrapping_add((((c.m.r16((((p_parent_2).wrapping_add(84)).wrapping_add(16)).wrapping_add((((l_i_5) as u32)).wrapping_mul(2))) as i16)) as i32)); F::gfxm_c__Flash_ClampAdd(c, a0_) } }; let a_ = (((fp).wrapping_add(84)).wrapping_add(16)).wrapping_add((((l_i_5) as u32)).wrapping_mul(2)); c.m.w16(a_, (v_) as u16); };
                        }
                        { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
                    }
                }
            }
            if (((((c.m.r32((l_clip_4).wrapping_add(180))) != (((0i32) as u32))) as i32)) != 0) {
                { let fp_ = c.m.r32((l_clip_4).wrapping_add(180)); let a0_ = c.m.r32((l_clip_4).wrapping_add(188)); { let _ = D::call(c, fp_, &[((a0_) as u64)]); } };
            }
            F::FlashTl_Draw(c, (l_clip_4).wrapping_add(204), fp);
            if (((((c.m.r32((l_clip_4).wrapping_add(196))) != (((0i32) as u32))) as i32)) != 0) {
                { let fp_ = c.m.r32((l_clip_4).wrapping_add(196)); let a0_ = c.m.r32((l_clip_4).wrapping_add(200)); { let _ = D::call(c, fp_, &[((a0_) as u64), ((fp) as u64)]); } };
            }
            if (((((c.m.r32((l_clip_4).wrapping_add(184))) != (((0i32) as u32))) as i32)) != 0) {
                { let fp_ = c.m.r32((l_clip_4).wrapping_add(184)); let a0_ = c.m.r32((l_clip_4).wrapping_add(192)); { let _ = D::call(c, fp_, &[((a0_) as u64)]); } };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "FlashSlot_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn FlashDraw_AdjustY(c: &mut Ctx, mut p_y_1: F32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashDraw_AdjustY", &PROBE_N, &[(p_y_1).0], false); }
    let r_ = 'f: {
    break 'f p_y_1;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashDraw_AdjustY", &PROBE_N, &[], true); }
    r_
}

pub fn FlashDraw_UploadTex(c: &mut Ctx, mut p_tex_1: u32, mut p_index_2: i32, mut p_tbp_3: i32) -> u64 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashDraw_UploadTex", &PROBE_N, &[(p_tex_1) as u32, (p_index_2) as u32, (p_tbp_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_clut_4: u32 = 0;
    let mut l_pix_5: u32 = 0;
    let mut l_base_6: i64 = 0;
    let mut l_tex0_7: u64 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 48);
        rt::memset(c, fp.wrapping_add(0), 0, 16);
        { let v_ = ((268435458i32) as u32); c.m.w32(fp.wrapping_add(0).wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(0).wrapping_add(4), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(0).wrapping_add(8), v_); }
        { let v_ = ((1342177282i32) as u32); c.m.w32(fp.wrapping_add(0).wrapping_add(12), v_); }
        rt::memset(c, fp.wrapping_add(16), 0, 16);
        { let v_ = ((1152921504606879745i64) as u64); c.m.w64(fp.wrapping_add(16).wrapping_add(0), v_); }
        { let v_ = ((14i32) as u64); c.m.w64(fp.wrapping_add(16).wrapping_add(8), v_); }
        { let v_ = ((0i32) as u64); c.m.w64(fp.wrapping_add(32), v_); }
        { let v_ = ((80i32) as u64); c.m.w64(fp.wrapping_add(40), v_); }
        l_clut_4 = ((((p_index_2).wrapping_shl((6i32) as u32)) as u32)).wrapping_add(p_tex_1);
        l_base_6 = ((p_tbp_3) as i64);
        if ((((((c.m.r32((l_clut_4).wrapping_add(8)) as i32)) == (0i32)) as i32)) != 0) {
            p_index_2 = 0i32;
        }
        l_pix_5 = ((((p_index_2).wrapping_shl((6i32) as u32)) as u32)).wrapping_add(p_tex_1);
        { let v_ = (((((l_base_6).wrapping_shl((32i32) as u32)) as u64)) | ((((c.m.r32((l_pix_5).wrapping_add(24))) as u64)).wrapping_shl((48i32) as u32))); let a_ = (fp).wrapping_add(32); c.m.w64(a_, v_); };
        F::Dma_AddData(c, fp, ((48u32) as i32));
        { let a0_ = c.m.r32((l_pix_5).wrapping_add(56)); let a1_ = (c.m.r32((l_pix_5).wrapping_add(8)) as i32); F::Dma_AddRef(c, a0_, a1_) };
        { let r_ = (c.m.r32((l_pix_5).wrapping_add(16)) as i32); let n_ = (p_tbp_3).wrapping_add(r_); p_tbp_3 = n_; };
        { let r_ = 64i32; let n_ = (p_tbp_3).wrapping_add(r_); p_tbp_3 = n_; };
        { let v_ = (((((((p_tbp_3) as i64)).wrapping_shl((32i32) as u32)) as u64)) | ((((c.m.r32((l_clut_4).wrapping_add(28))) as u64)).wrapping_shl((48i32) as u32))); let a_ = (fp).wrapping_add(32); c.m.w64(a_, v_); };
        F::Dma_AddData(c, fp, ((48u32) as i32));
        { let a0_ = c.m.r32((l_clut_4).wrapping_add(60)); let a1_ = (c.m.r32((l_clut_4).wrapping_add(12)) as i32); F::Dma_AddRef(c, a0_, a1_) };
        l_tex0_7 = c.m.r64((l_pix_5).wrapping_add(48));
        { let r_ = (((((p_tbp_3) as i64)).wrapping_shl((37i32) as u32)) as u64); let n_ = ((l_tex0_7) | (r_)); l_tex0_7 = n_; };
        { let r_ = ((l_base_6) as u64); let n_ = ((l_tex0_7) | (r_)); l_tex0_7 = n_; };
        { let r_ = (((1i64).wrapping_shl((34i32) as u32)) as u64); let n_ = ((l_tex0_7) | (r_)); l_tex0_7 = n_; };
        break 'f l_tex0_7;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "FlashDraw_UploadTex", &PROBE_N, &[], true); }
    r_
}

pub fn FlashDraw_PutSprite(c: &mut Ctx, mut p_p_1: u32, mut p_q_2: u32, mut p_tex0_3: u64) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashDraw_PutSprite", &PROBE_N, &[(p_p_1) as u32, (p_q_2) as u32, (p_tex0_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_oy_4: i32 = 0;
    let mut l_ox_5: i32 = 0;
    let r_ = 'f: {
        { let v_ = 14123288431433908225u64; let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((91617345341153i64) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
        { let v_ = (((((((((((c.m.r32(((p_q_2).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as i64)) | (((((c.m.r32(((p_q_2).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as i64)).wrapping_shl((8i32) as u32)))) | (((((c.m.r32(((p_q_2).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as i64)).wrapping_shl((16i32) as u32)))) | (((((c.m.r32(((p_q_2).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) as i64)).wrapping_shl((24i32) as u32)))) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((0i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
        { let v_ = p_tex0_3; let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = ((340i32) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
        { let v_ = ((c.m.r32((p_q_2).wrapping_add(36)) as i32)).wrapping_add(1i32); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32(((p_q_2).wrapping_add(36)).wrapping_add(8)) as i32)).wrapping_sub(1i32); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32(((p_q_2).wrapping_add(36)).wrapping_add(4)) as i32)).wrapping_add(1i32); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32(((p_q_2).wrapping_add(36)).wrapping_add(12)) as i32)).wrapping_sub(1i32); let a_ = (fp).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        if ((((c.m.r32(p_q_2)) & (((1i32) as u32)))) != 0) {
            { let v_ = (c.m.r32(((p_q_2).wrapping_add(36)).wrapping_add(8)) as i32); let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (c.m.r32((p_q_2).wrapping_add(36)) as i32); let a_ = (fp).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
        if ((((c.m.r32(p_q_2)) & (((2i32) as u32)))) != 0) {
            { let v_ = (c.m.r32(((p_q_2).wrapping_add(36)).wrapping_add(12)) as i32); let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = (c.m.r32(((p_q_2).wrapping_add(36)).wrapping_add(4)) as i32); let a_ = (fp).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
        l_oy_4 = 29184i32;
        l_ox_5 = 28672i32;
        { let v_ = ((((((((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)).wrapping_shl((4i32) as u32)) as i64)) | ((((((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)).wrapping_shl((4i32) as u32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = (((((((rt::f2i(rt::fmul(F32(c.m.r32((((p_q_2).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0x41800000)))).wrapping_add(l_ox_5)) as i64)) | (((((rt::f2i(rt::fmul({ let a0_ = F32(c.m.r32((((p_q_2).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); F::FlashDraw_AdjustY(c, a0_) }, F32(0x41800000)))).wrapping_add(l_oy_4)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
        { let v_ = ((((((((c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)).wrapping_shl((4i32) as u32)) as i64)) | ((((((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)).wrapping_shl((4i32) as u32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = (((((((rt::f2i(rt::fmul(F32(c.m.r32((((p_q_2).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0x41800000)))).wrapping_add(l_ox_5)) as i64)) | (((((rt::f2i(rt::fmul({ let a0_ = F32(c.m.r32((((p_q_2).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); F::FlashDraw_AdjustY(c, a0_) }, F32(0x41800000)))).wrapping_add(l_oy_4)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
        { let v_ = ((((((((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)).wrapping_shl((4i32) as u32)) as i64)) | ((((((c.m.r32((fp).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)).wrapping_shl((4i32) as u32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = (((((((rt::f2i(rt::fmul(F32(c.m.r32((((p_q_2).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0x41800000)))).wrapping_add(l_ox_5)) as i64)) | (((((rt::f2i(rt::fmul({ let a0_ = F32(c.m.r32((((p_q_2).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); F::FlashDraw_AdjustY(c, a0_) }, F32(0x41800000)))).wrapping_add(l_oy_4)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
        { let v_ = ((((((((c.m.r32((fp).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)).wrapping_shl((4i32) as u32)) as i64)) | ((((((c.m.r32((fp).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)).wrapping_shl((4i32) as u32)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (p_p_1).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let v_ = (((((((rt::f2i(rt::fmul(F32(c.m.r32((((p_q_2).wrapping_add(4)).wrapping_add((((3i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0x41800000)))).wrapping_add(l_ox_5)) as i64)) | (((((rt::f2i(rt::fmul({ let a0_ = F32(c.m.r32((((p_q_2).wrapping_add(4)).wrapping_add((((3i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); F::FlashDraw_AdjustY(c, a0_) }, F32(0x41800000)))).wrapping_add(l_oy_4)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (p_p_1).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        { let r_ = 2i32; let n_ = (p_p_1).wrapping_add((((r_) as u32)).wrapping_mul(8)); p_p_1 = n_; };
        break 'f p_p_1;
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "FlashDraw_PutSprite", &PROBE_N, &[], true); }
    r_
}

pub fn FlashDraw_FlatQuad(c: &mut Ctx, mut p_q_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashDraw_FlatQuad", &PROBE_N, &[(p_q_1) as u32], false); }
    let mut l_p_2: u32 = 0;
    let mut l_oy_3: i32 = 0;
    let mut l_ox_4: i32 = 0;
    let r_ = 'f: {
    l_p_2 = F::Dma_BeginDirect(c);
    { let v_ = ((1152921504606879748i64) as u64); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
    { let v_ = (if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { ((((((((112i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) } else { ((((((((0i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) }); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((76i32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
    { let v_ = ((1i32) as u64); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((70i32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
    if ((((c.m.r32(p_q_1)) & (((8i32) as u32)))) != 0) {
        { let v_ = ((72i32) as u64); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    } else {
        if ((((c.m.r32(p_q_1)) & (((16i32) as u32)))) != 0) {
            { let v_ = ((66i32) as u64); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        } else {
            { let v_ = ((68i32) as u64); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        }
    }
    { let v_ = ((66i32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
    { let v_ = ((196608i32) as u64); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((71i32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
    { let v_ = ((7205759403792826369i64) as u64); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((5592336i32) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
    { let v_ = ((68i32) as u64); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    l_oy_3 = 29184i32;
    l_ox_4 = 28672i32;
    { let v_ = (((((((((((c.m.r32(((p_q_1).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) as i64)) | (((((c.m.r32(((p_q_1).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) as i64)).wrapping_shl((8i32) as u32)))) | (((((c.m.r32(((p_q_1).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) as i64)).wrapping_shl((16i32) as u32)))) | (((((c.m.r32(((p_q_1).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) as i64)).wrapping_shl((24i32) as u32)))) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
    { let v_ = (((((((rt::f2i(rt::fmul(F32(c.m.r32((((p_q_1).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0x41800000)))).wrapping_add(l_ox_4)) as i64)) | (((((rt::f2i(rt::fmul({ let a0_ = F32(c.m.r32((((p_q_1).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); F::FlashDraw_AdjustY(c, a0_) }, F32(0x41800000)))).wrapping_add(l_oy_3)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = (((((((rt::f2i(rt::fmul(F32(c.m.r32((((p_q_1).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0x41800000)))).wrapping_add(l_ox_4)) as i64)) | (((((rt::f2i(rt::fmul({ let a0_ = F32(c.m.r32((((p_q_1).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); F::FlashDraw_AdjustY(c, a0_) }, F32(0x41800000)))).wrapping_add(l_oy_3)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
    { let v_ = (((((((rt::f2i(rt::fmul(F32(c.m.r32((((p_q_1).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0x41800000)))).wrapping_add(l_ox_4)) as i64)) | (((((rt::f2i(rt::fmul({ let a0_ = F32(c.m.r32((((p_q_1).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); F::FlashDraw_AdjustY(c, a0_) }, F32(0x41800000)))).wrapping_add(l_oy_3)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_2).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = (((((((rt::f2i(rt::fmul(F32(c.m.r32((((p_q_1).wrapping_add(4)).wrapping_add((((3i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(0x41800000)))).wrapping_add(l_ox_4)) as i64)) | (((((rt::f2i(rt::fmul({ let a0_ = F32(c.m.r32((((p_q_1).wrapping_add(4)).wrapping_add((((3i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))); F::FlashDraw_AdjustY(c, a0_) }, F32(0x41800000)))).wrapping_add(l_oy_3)) as i64)).wrapping_shl((16i32) as u32)))) as u64); let a_ = (l_p_2).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_2).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_2 = n_; };
    F::Dma_EndDirect(c, l_p_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashDraw_FlatQuad", &PROBE_N, &[], true); }
    r_
}

pub fn FlashDraw_MaskQuad(c: &mut Ctx, mut p_q_1: u32, mut p_tex0_2: u64, mut p_set_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashDraw_MaskQuad", &PROBE_N, &[(p_q_1) as u32, (p_tex0_2) as u32, (p_set_3) as u32], false); }
    let mut l_p_4: u32 = 0;
    let mut l_frame_5: u64 = 0;
    let mut l_reg_6: u64 = 0;
    let r_ = 'f: {
    l_p_4 = F::Dma_BeginDirect(c);
    { let v_ = ((1152921504606879751i64) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    l_frame_5 = (if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { ((((((((112i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) } else { ((((((((0i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) });
    if ((p_set_3) != 0) {
        l_reg_6 = ((l_frame_5) | (((72057589742960640i64) as u64)));
    } else {
        l_reg_6 = ((l_frame_5) | (18374686479671623680u64));
    }
    { let v_ = l_reg_6; let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((76i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((1i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((70i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    if ((p_set_3) != 0) {
        { let v_ = ((327680i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    } else {
        { let v_ = ((328653i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    }
    { let v_ = ((71i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    if ((((c.m.r32(p_q_1)) & (((8i32) as u32)))) != 0) {
        { let v_ = (((if ((((!((p_set_3) != 0)) as i32)) != 0) { 88i32 } else { 72i32 })) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    } else {
        if ((((c.m.r32(p_q_1)) & (((16i32) as u32)))) != 0) {
            { let v_ = (((if ((((!((p_set_3) != 0)) as i32)) != 0) { 82i32 } else { 66i32 })) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        } else {
            { let v_ = (((if ((((!((p_set_3) != 0)) as i32)) != 0) { 84i32 } else { 68i32 })) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        }
    }
    { let v_ = ((66i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((0i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((63i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((96i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((20i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    if ((((c.m.r32(p_q_1)) & (((4i32) as u32)))) != 0) {
        { let v_ = ((0i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    } else {
        { let v_ = ((5i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    }
    { let v_ = ((8i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let a0_ = F::FlashDraw_PutSprite(c, l_p_4, p_q_1, p_tex0_2); F::Dma_EndDirect(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashDraw_MaskQuad", &PROBE_N, &[], true); }
    r_
}

pub fn FlashDraw_Quad(c: &mut Ctx, mut p_tex_1: u32, mut p_q_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashDraw_Quad", &PROBE_N, &[(p_tex_1) as u32, (p_q_2) as u32], false); }
    let mut l_tex0_3: u64 = 0;
    let mut l_p_4: u32 = 0;
    let r_ = 'f: {
    if ((((((c.m.r32(((p_q_2).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) == (0i32)) as i32)) != 0) {
        break 'f;
    }
    if (((((p_tex_1) != (((0i32) as u32))) as i32)) != 0) {
        l_tex0_3 = { let a1_ = (c.m.r32(((p_q_2).wrapping_add(36)).wrapping_add(16)) as i32); F::FlashDraw_UploadTex(c, p_tex_1, a1_, 12288i32) };
        F::Dma_AddTexFlush(c);
    } else {
        F::FlashDraw_FlatQuad(c, p_q_2);
        break 'f;
    }
    if ((((c.m.r32(p_q_2)) & (((64i32) as u32)))) != 0) {
        F::Gfx_ClearScreen(c, 16777215i32, ((0i32) as u32));
        F::FlashDraw_MaskQuad(c, p_q_2, l_tex0_3, 1i32);
        break 'f;
    }
    if ((((c.m.r32(p_q_2)) & (((32i32) as u32)))) != 0) {
        F::FlashDraw_MaskQuad(c, p_q_2, l_tex0_3, 0i32);
        break 'f;
    }
    l_p_4 = F::Dma_BeginDirect(c);
    { let v_ = ((1152921504606879750i64) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((14i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = (if ((((!(((((c.m.r32((sym::gGfx).wrapping_add(564)) as i32)) & (1i32))) != 0)) as i32)) != 0) { ((((((((112i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) } else { ((((((((0i32) as u64)) | ((((8i32) as u64)).wrapping_shl((16i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((24i32) as u32)))) | ((((0i32) as u64)).wrapping_shl((32i32) as u32))) }); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((76i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((1i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((70i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    if ((((c.m.r32(p_q_2)) & (((8i32) as u32)))) != 0) {
        { let v_ = ((72i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    } else {
        if ((((c.m.r32(p_q_2)) & (((16i32) as u32)))) != 0) {
            { let v_ = ((66i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        } else {
            { let v_ = ((68i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
        }
    }
    { let v_ = ((66i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((96i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((20i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let v_ = ((196608i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let v_ = ((71i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    if ((((c.m.r32(p_q_2)) & (((4i32) as u32)))) != 0) {
        { let v_ = ((0i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    } else {
        { let v_ = ((5i32) as u64); let a_ = (l_p_4).wrapping_add((((0i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    }
    { let v_ = ((8i32) as u64); let a_ = (l_p_4).wrapping_add((((1i32) as u32)).wrapping_mul(8)); c.m.w64(a_, v_); };
    { let r_ = 2i32; let n_ = (l_p_4).wrapping_add((((r_) as u32)).wrapping_mul(8)); l_p_4 = n_; };
    { let a0_ = F::FlashDraw_PutSprite(c, l_p_4, p_q_2, l_tex0_3); F::Dma_EndDirect(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashDraw_Quad", &PROBE_N, &[], true); }
    r_
}

pub fn FlashDraw_TransformQuad(c: &mut Ctx, mut p_q_1: u32, mut p_mtx_2: u32, mut p_cx_3: u32, mut p_prop_4: u32, mut p_ofs_5: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashDraw_TransformQuad", &PROBE_N, &[(p_q_1) as u32, (p_mtx_2) as u32, (p_cx_3) as u32, (p_prop_4) as u32, (p_ofs_5) as u32], false); }
    let mut l_i_6: i32 = 0;
    let mut l_x_7: F32 = F32(0);
    let mut l_y_8: F32 = F32(0);
    let mut l_sx_9: F32 = F32(0);
    let mut l_sy_10: F32 = F32(0);
    let mut l_xs_11: u32 = 0;
    let mut l_ys_12: u32 = 0;
    let mut l_px_13: u32 = 0;
    let mut l_py_14: u32 = 0;
    let mut l_xs_15: u32 = 0;
    let mut l_ys_16: u32 = 0;
    let mut l_px_17: u32 = 0;
    let mut l_py_18: u32 = 0;
    let r_ = 'f: {
    {
        {
            l_i_6 = 0i32;
            'l1: loop {
                if !(((((l_i_6) < (3i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_xs_11 = (((p_q_1).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4));
                    l_ys_12 = (((p_q_1).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4));
                    l_px_13 = (l_xs_11).wrapping_add(((((l_i_6).wrapping_mul(2i32)) as u32)).wrapping_mul(4));
                    l_py_14 = (l_ys_12).wrapping_add(((((l_i_6).wrapping_mul(2i32)) as u32)).wrapping_mul(4));
                    l_x_7 = F32(c.m.r32(l_px_13));
                    l_y_8 = F32(c.m.r32(l_py_14));
                    { let v_ = rt::fadd(rt::fadd(rt::fmul(l_x_7, F32(c.m.r32((p_mtx_2).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), rt::fmul(l_y_8, F32(c.m.r32((p_mtx_2).wrapping_add((((3i32) as u32)).wrapping_mul(4)))))), F32(c.m.r32((p_mtx_2).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let a_ = l_px_13; c.m.w32(a_, (v_).0); };
                    { let v_ = rt::fadd(rt::fadd(rt::fmul(l_x_7, F32(c.m.r32((p_mtx_2).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), rt::fmul(l_y_8, F32(c.m.r32((p_mtx_2).wrapping_add((((4i32) as u32)).wrapping_mul(4)))))), F32(c.m.r32((p_mtx_2).wrapping_add((((5i32) as u32)).wrapping_mul(4))))); let a_ = l_py_14; c.m.w32(a_, (v_).0); };
                    if (((((p_prop_4) == (((0i32) as u32))) as i32)) != 0) {
                        { let r_ = rt::i2f((c.m.r32((p_ofs_5).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)); let a_ = l_px_13; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        { let r_ = rt::i2f((c.m.r32((p_ofs_5).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)); let a_ = l_py_14; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                    }
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
    }
    { let v_ = rt::fsub(rt::fadd(F32(c.m.r32((((p_q_1).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((p_q_1).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), F32(c.m.r32((((p_q_1).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (((p_q_1).wrapping_add(4)).wrapping_add((((3i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fsub(rt::fadd(F32(c.m.r32((((p_q_1).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((p_q_1).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), F32(c.m.r32((((p_q_1).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = (((p_q_1).wrapping_add(4)).wrapping_add((((3i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
    { let v_ = { let a0_ = (rt::f2i(rt::fmul(rt::i2f((c.m.r32(((p_q_1).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32((p_cx_3).wrapping_add((((0i32) as u32)).wrapping_mul(4))))))).wrapping_add((((c.m.r16(((p_cx_3).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(2))) as i16)) as i32)); F::gfxm_c__Flash_Clamp255(c, a0_) }; let a_ = ((p_q_1).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a0_ = (rt::f2i(rt::fmul(rt::i2f((c.m.r32(((p_q_1).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32((p_cx_3).wrapping_add((((1i32) as u32)).wrapping_mul(4))))))).wrapping_add((((c.m.r16(((p_cx_3).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(2))) as i16)) as i32)); F::gfxm_c__Flash_Clamp255(c, a0_) }; let a_ = ((p_q_1).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a0_ = (rt::f2i(rt::fmul(rt::i2f((c.m.r32(((p_q_1).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32((p_cx_3).wrapping_add((((2i32) as u32)).wrapping_mul(4))))))).wrapping_add((((c.m.r16(((p_cx_3).wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(2))) as i16)) as i32)); F::gfxm_c__Flash_Clamp255(c, a0_) }; let a_ = ((p_q_1).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = { let a0_ = (rt::f2i(rt::fmul(rt::i2f((c.m.r32(((p_q_1).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32((p_cx_3).wrapping_add((((3i32) as u32)).wrapping_mul(4))))))).wrapping_add((((c.m.r16(((p_cx_3).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(2))) as i16)) as i32)); F::gfxm_c__Flash_Clamp255(c, a0_) }; let a_ = ((p_q_1).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    if (((((p_prop_4) != (((0i32) as u32))) as i32)) != 0) {
        l_sx_9 = F32(c.m.r32(((p_prop_4).wrapping_add(48)).wrapping_add((((0i32) as u32)).wrapping_mul(4))));
        l_sy_10 = F32(c.m.r32(((p_prop_4).wrapping_add(48)).wrapping_add((((4i32) as u32)).wrapping_mul(4))));
        if ((((c.m.r32(p_prop_4)) & (((16i32) as u32)))) != 0) {
            { let r_ = F32(c.m.r32((p_prop_4).wrapping_add(16))); let n_ = rt::fmul(l_sx_9, r_); l_sx_9 = n_; };
            { let r_ = F32(c.m.r32((p_prop_4).wrapping_add(20))); let n_ = rt::fmul(l_sy_10, r_); l_sy_10 = n_; };
        }
        {
            {
                l_i_6 = 0i32;
                'l3: loop {
                    if !(((((l_i_6) < (3i32)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        l_xs_15 = (((p_q_1).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4));
                        l_ys_16 = (((p_q_1).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4));
                        l_px_17 = (l_xs_15).wrapping_add(((((l_i_6).wrapping_mul(2i32)) as u32)).wrapping_mul(4));
                        l_py_18 = (l_ys_16).wrapping_add(((((l_i_6).wrapping_mul(2i32)) as u32)).wrapping_mul(4));
                        l_x_7 = F32(c.m.r32(l_px_17));
                        l_y_8 = F32(c.m.r32(l_py_18));
                        { let v_ = rt::fadd(rt::fadd(rt::fmul(l_x_7, l_sx_9), rt::fmul(l_y_8, F32(c.m.r32(((p_prop_4).wrapping_add(48)).wrapping_add((((3i32) as u32)).wrapping_mul(4)))))), F32(c.m.r32(((p_prop_4).wrapping_add(48)).wrapping_add((((2i32) as u32)).wrapping_mul(4))))); let a_ = l_px_17; c.m.w32(a_, (v_).0); };
                        { let v_ = rt::fadd(rt::fadd(rt::fmul(l_x_7, F32(c.m.r32(((p_prop_4).wrapping_add(48)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), rt::fmul(l_y_8, l_sy_10)), F32(c.m.r32(((p_prop_4).wrapping_add(48)).wrapping_add((((5i32) as u32)).wrapping_mul(4))))); let a_ = l_py_18; c.m.w32(a_, (v_).0); };
                        if ((((c.m.r32(p_prop_4)) & (((2i32) as u32)))) != 0) {
                            { let r_ = F32(c.m.r32((p_prop_4).wrapping_add(4))); let a_ = l_px_17; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            { let r_ = F32(c.m.r32((p_prop_4).wrapping_add(8))); let a_ = l_py_18; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        }
                        if ((((!((((c.m.r32(p_prop_4)) & (((512i32) as u32)))) != 0)) as i32)) != 0) {
                            { let r_ = rt::i2f((c.m.r32((p_ofs_5).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)); let a_ = l_px_17; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                            { let r_ = rt::i2f((c.m.r32((p_ofs_5).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)); let a_ = l_py_18; let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                        }
                    }
                    { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
                }
            }
        }
        { let v_ = rt::fsub(rt::fadd(F32(c.m.r32((((p_q_1).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((p_q_1).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))), F32(c.m.r32((((p_q_1).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4))))); let a_ = (((p_q_1).wrapping_add(4)).wrapping_add((((3i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = rt::fsub(rt::fadd(F32(c.m.r32((((p_q_1).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)))), F32(c.m.r32((((p_q_1).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))), F32(c.m.r32((((p_q_1).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4))))); let a_ = (((p_q_1).wrapping_add(4)).wrapping_add((((3i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
        { let v_ = { let a0_ = (rt::f2i(rt::fmul(rt::i2f((c.m.r32(((p_q_1).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32(((p_prop_4).wrapping_add(84)).wrapping_add((((0i32) as u32)).wrapping_mul(4))))))).wrapping_add((((c.m.r16((((p_prop_4).wrapping_add(84)).wrapping_add(16)).wrapping_add((((0i32) as u32)).wrapping_mul(2))) as i16)) as i32)); F::gfxm_c__Flash_Clamp255(c, a0_) }; let a_ = ((p_q_1).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = (rt::f2i(rt::fmul(rt::i2f((c.m.r32(((p_q_1).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32(((p_prop_4).wrapping_add(84)).wrapping_add((((1i32) as u32)).wrapping_mul(4))))))).wrapping_add((((c.m.r16((((p_prop_4).wrapping_add(84)).wrapping_add(16)).wrapping_add((((1i32) as u32)).wrapping_mul(2))) as i16)) as i32)); F::gfxm_c__Flash_Clamp255(c, a0_) }; let a_ = ((p_q_1).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = (rt::f2i(rt::fmul(rt::i2f((c.m.r32(((p_q_1).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32(((p_prop_4).wrapping_add(84)).wrapping_add((((2i32) as u32)).wrapping_mul(4))))))).wrapping_add((((c.m.r16((((p_prop_4).wrapping_add(84)).wrapping_add(16)).wrapping_add((((2i32) as u32)).wrapping_mul(2))) as i16)) as i32)); F::gfxm_c__Flash_Clamp255(c, a0_) }; let a_ = ((p_q_1).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = (rt::f2i(rt::fmul(rt::i2f((c.m.r32(((p_q_1).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32(((p_prop_4).wrapping_add(84)).wrapping_add((((3i32) as u32)).wrapping_mul(4))))))).wrapping_add((((c.m.r16((((p_prop_4).wrapping_add(84)).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(2))) as i16)) as i32)); F::gfxm_c__Flash_Clamp255(c, a0_) }; let a_ = ((p_q_1).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        if ((((c.m.r32(p_prop_4)) & (((32i32) as u32)))) != 0) {
            { let v_ = { let a0_ = rt::f2i(rt::fmul(rt::i2f((c.m.r32(((p_q_1).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32((p_prop_4).wrapping_add(24))))); F::gfxm_c__Flash_Clamp255(c, a0_) }; let a_ = ((p_q_1).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = { let a0_ = rt::f2i(rt::fmul(rt::i2f((c.m.r32(((p_q_1).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32((p_prop_4).wrapping_add(24))))); F::gfxm_c__Flash_Clamp255(c, a0_) }; let a_ = ((p_q_1).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            { let v_ = { let a0_ = rt::f2i(rt::fmul(rt::i2f((c.m.r32(((p_q_1).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32((p_prop_4).wrapping_add(24))))); F::gfxm_c__Flash_Clamp255(c, a0_) }; let a_ = ((p_q_1).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
        if ((((c.m.r32(p_prop_4)) & (((8i32) as u32)))) != 0) {
            { let v_ = { let a0_ = rt::f2i(rt::fmul(rt::i2f((c.m.r32(((p_q_1).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)), F32(c.m.r32((p_prop_4).wrapping_add(12))))); F::gfxm_c__Flash_Clamp255(c, a0_) }; let a_ = ((p_q_1).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        }
        if ((((c.m.r32(p_prop_4)) & (((4i32) as u32)))) != 0) {
            { let s_ = (p_prop_4).wrapping_add(28); let d_ = (p_q_1).wrapping_add(36); rt::memcpy(c, d_, s_, 20); };
            { let v_ = 0i32; let a_ = ((p_q_1).wrapping_add(36)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
        }
        if ((((c.m.r32(p_prop_4)) & (((1i32) as u32)))) != 0) {
            { let v_ = (c.m.r32(((p_prop_4).wrapping_add(28)).wrapping_add(16)) as i32); let a_ = ((p_q_1).wrapping_add(36)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
        }
        if ((((c.m.r32(p_prop_4)) & (((64i32) as u32)))) != 0) {
            { let r_ = ((1i32) as u32); let a_ = p_q_1; let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        }
        if ((((c.m.r32(p_prop_4)) & (((128i32) as u32)))) != 0) {
            { let r_ = ((2i32) as u32); let a_ = p_q_1; let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        }
        if ((((c.m.r32(p_prop_4)) & (((256i32) as u32)))) != 0) {
            { let r_ = ((4i32) as u32); let a_ = p_q_1; let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        }
        if ((((c.m.r32(p_prop_4)) & (((1024i32) as u32)))) != 0) {
            { let r_ = ((8i32) as u32); let a_ = p_q_1; let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        }
        if ((((c.m.r32(p_prop_4)) & (((2048i32) as u32)))) != 0) {
            { let r_ = ((16i32) as u32); let a_ = p_q_1; let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        }
        if ((((c.m.r32(p_prop_4)) & (((4096i32) as u32)))) != 0) {
            { let r_ = ((32i32) as u32); let a_ = p_q_1; let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        }
        if ((((c.m.r32(p_prop_4)) & (((8192i32) as u32)))) != 0) {
            { let r_ = ((64i32) as u32); let a_ = p_q_1; let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashDraw_TransformQuad", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_Create(c: &mut Ctx, mut p_flash_1: u32, mut p_file_2: u32, mut p_tex_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_Create", &PROBE_N, &[(p_flash_1) as u32, (p_file_2) as u32, (p_tex_3) as u32], false); }
    let r_ = 'f: {
    { let _ = F::Flash_CheckHeader(c, p_file_2); };
    { let _ = F::memset(c, p_flash_1, 0i32, 44u32); };
    { let v_ = p_tex_3; let a_ = (p_flash_1).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = (p_file_2).wrapping_add(((8i32) as u32)); let a_ = p_flash_1; c.m.w32(a_, v_); };
    { let v_ = 0i32; let a_ = (p_flash_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = ((p_flash_1).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = ((p_flash_1).wrapping_add(24)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    if (((((c.m.r32((p_flash_1).wrapping_add(32))) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = F::Heap_Alloc(c, ((24u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (p_flash_1).wrapping_add(32); c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32((p_flash_1).wrapping_add(32)); F::memset(c, a0_, 0i32, 24u32) }; };
    }
    { let a0_ = c.m.r32((p_flash_1).wrapping_add(32)); let a1_ = ({ let a0_ = c.m.r32(p_flash_1); F::Flash_FindTag(c, a0_, ((7i32) as u8)) }).wrapping_add(((8i32) as u32)); F::FlashTl_Init(c, a0_, a1_, p_flash_1) };
    { let r_ = 1i32; let a_ = (c.m.r32((p_flash_1).wrapping_add(32))).wrapping_add(21); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
    if (((((c.m.r32((p_flash_1).wrapping_add(36))) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = F::Heap_Alloc(c, ((12u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (p_flash_1).wrapping_add(36); c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32((p_flash_1).wrapping_add(36)); F::memset(c, a0_, 0i32, 12u32) }; };
    }
    { let a0_ = c.m.r32((p_flash_1).wrapping_add(36)); F::FlashClipList_Init(c, a0_, p_flash_1) };
    if (((((c.m.r32((p_flash_1).wrapping_add(40))) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = F::Heap_Alloc(c, ((8u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = (p_flash_1).wrapping_add(40); c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32((p_flash_1).wrapping_add(40)); F::memset(c, a0_, 0i32, 8u32) }; };
    }
    { let a0_ = c.m.r32((p_flash_1).wrapping_add(40)); let a1_ = ((({ let a_ = (c.m.r32((p_flash_1).wrapping_add(32))).wrapping_add(22); c.m.r16(a_) }) as u32)).wrapping_add({ let a_ = (c.m.r32((p_flash_1).wrapping_add(36))).wrapping_add(8); c.m.r32(a_) }); F::FlashShapePool_Init(c, a0_, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Flash_Create", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_Destroy(c: &mut Ctx, mut p_flash_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_Destroy", &PROBE_N, &[(p_flash_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32((p_flash_1).wrapping_add(40))) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32((p_flash_1).wrapping_add(40)); F::FlashShapePool_Term(c, a0_) };
        if (((((c.m.r32((p_flash_1).wrapping_add(40))) != (((0i32) as u32))) as i32)) != 0) {
            { let a0_ = c.m.r32((p_flash_1).wrapping_add(40)); F::Heap_Free(c, a0_) };
            { let v_ = ((0i32) as u32); let a_ = (p_flash_1).wrapping_add(40); c.m.w32(a_, v_); };
        }
    }
    if (((((c.m.r32((p_flash_1).wrapping_add(36))) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32((p_flash_1).wrapping_add(36)); F::FlashClipList_Term(c, a0_) };
        if (((((c.m.r32((p_flash_1).wrapping_add(36))) != (((0i32) as u32))) as i32)) != 0) {
            { let a0_ = c.m.r32((p_flash_1).wrapping_add(36)); F::Heap_Free(c, a0_) };
            { let v_ = ((0i32) as u32); let a_ = (p_flash_1).wrapping_add(36); c.m.w32(a_, v_); };
        }
    }
    if (((((c.m.r32((p_flash_1).wrapping_add(32))) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32((p_flash_1).wrapping_add(32)); F::FlashTl_Term(c, a0_) };
        if (((((c.m.r32((p_flash_1).wrapping_add(32))) != (((0i32) as u32))) as i32)) != 0) {
            { let a0_ = c.m.r32((p_flash_1).wrapping_add(32)); F::Heap_Free(c, a0_) };
            { let v_ = ((0i32) as u32); let a_ = (p_flash_1).wrapping_add(32); c.m.w32(a_, v_); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Flash_Destroy", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_Advance(c: &mut Ctx, mut p_flash_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_Advance", &PROBE_N, &[(p_flash_1) as u32], false); }
    let mut l_speed_2: u32 = 0;
    let mut l_flags_3: u32 = 0;
    let r_ = 'f: {
    l_speed_2 = (p_flash_1).wrapping_add(20);
    { let v_ = ((0i32) as u32); let a_ = (p_flash_1).wrapping_add(12); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (p_flash_1).wrapping_add(16); c.m.w32(a_, v_); };
    { let r_ = (((!(8i32))) as u32); let a_ = (p_flash_1).wrapping_add(8); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
    l_flags_3 = (p_flash_1).wrapping_add(8);
    if ((((c.m.r32(l_flags_3)) & (((1i32) as u32)))) != 0) {
        { let a0_ = c.m.r32((p_flash_1).wrapping_add(32)); let a1_ = (((c.m.r32(l_speed_2) as i32)) as u32); F::FlashTl_Advance(c, a0_, a1_, ((0i32) as u8), ((0i32) as u8)) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Flash_Advance", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_Draw(c: &mut Ctx, mut p_flash_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_Draw", &PROBE_N, &[(p_flash_1) as u32], false); }
    let r_ = 'f: {
    F::Gfx_AddDefaultEnv(c);
    if ((((!((((c.m.r32((p_flash_1).wrapping_add(8))) & (((4i32) as u32)))) != 0)) as i32)) != 0) {
        { let a0_ = c.m.r32((p_flash_1).wrapping_add(32)); F::FlashTl_Draw(c, a0_, ((0i32) as u32)) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Flash_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_Reset(c: &mut Ctx, mut p_flash_1: u32, mut p_keepClips_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_Reset", &PROBE_N, &[(p_flash_1) as u32, (p_keepClips_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((1i32) as u32); let a_ = (p_flash_1).wrapping_add(8); c.m.w32(a_, v_); };
    { let a0_ = c.m.r32((p_flash_1).wrapping_add(32)); F::FlashTl_Rewind(c, a0_, p_keepClips_2) };
    { let r_ = 1i32; let a_ = (c.m.r32((p_flash_1).wrapping_add(32))).wrapping_add(21); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
    if ((((!((p_keepClips_2) != 0)) as i32)) != 0) {
        { let a0_ = c.m.r32((p_flash_1).wrapping_add(36)); F::FlashClipList_Reset(c, a0_) };
        { let a0_ = c.m.r32((p_flash_1).wrapping_add(40)); F::FlashShapePool_Clear(c, a0_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Flash_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_Play(c: &mut Ctx, mut p_flash_1: u32, mut p_speed_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_Play", &PROBE_N, &[(p_flash_1) as u32, (p_speed_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_speed_2; let a_ = (p_flash_1).wrapping_add(20); c.m.w32(a_, (v_) as u32); };
    { let r_ = ((1i32) as u32); let a_ = (p_flash_1).wrapping_add(8); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Flash_Play", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_Stop(c: &mut Ctx, mut p_flash_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_Stop", &PROBE_N, &[(p_flash_1) as u32], false); }
    let r_ = 'f: {
    { let r_ = (((!(1i32))) as u32); let a_ = (p_flash_1).wrapping_add(8); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Flash_Stop", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_SetOffset(c: &mut Ctx, mut p_flash_1: u32, mut p_x_2: i32, mut p_y_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_SetOffset", &PROBE_N, &[(p_flash_1) as u32, (p_x_2) as u32, (p_y_3) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_x_2; let a_ = ((p_flash_1).wrapping_add(24)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    { let v_ = p_y_3; let a_ = ((p_flash_1).wrapping_add(24)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Flash_SetOffset", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_SetFlag(c: &mut Ctx, mut p_flash_1: u32, mut p_mask_2: u32, mut p_on_3: u8) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_SetFlag", &PROBE_N, &[(p_flash_1) as u32, (p_mask_2) as u32, (p_on_3) as u32], false); }
    let r_ = 'f: {
    if ((p_on_3) != 0) {
        { let r_ = p_mask_2; let a_ = (p_flash_1).wrapping_add(8); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    } else {
        { let r_ = (!(p_mask_2)); let a_ = (p_flash_1).wrapping_add(8); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Flash_SetFlag", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_GotoLabel(c: &mut Ctx, mut p_flash_1: u32, mut p_label_2: u32, mut p_restart_3: u8) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_GotoLabel", &PROBE_N, &[(p_flash_1) as u32, (p_label_2) as u32, (p_restart_3) as u32], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32((p_flash_1).wrapping_add(32)); F::FlashTl_GotoLabel(c, a0_, p_label_2, p_restart_3) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Flash_GotoLabel", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_StepFrames(c: &mut Ctx, mut p_flash_1: u32, mut p_step_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_StepFrames", &PROBE_N, &[(p_flash_1) as u32, (p_step_2) as u32], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32((p_flash_1).wrapping_add(32)); F::FlashTl_StepFrames(c, a0_, p_step_2) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Flash_StepFrames", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_FindLabel(c: &mut Ctx, mut p_flash_1: u32, mut p_parent_2: u32, mut p_name_3: u32, mut p_out_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_FindLabel", &PROBE_N, &[(p_flash_1) as u32, (p_parent_2) as u32, (p_name_3) as u32, (p_out_4) as u32], false); }
    let r_ = 'f: {
    { let _ = F::memset(c, p_out_4, 0i32, 8u32); };
    { let a0_ = c.m.r32((p_flash_1).wrapping_add(36)); F::FlashClipList_FindRef(c, a0_, p_parent_2, p_name_3, p_out_4) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Flash_FindLabel", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_ClipGotoLabel(c: &mut Ctx, mut p_flash_1: u32, mut p_ref_2: u32, mut p_label_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_ClipGotoLabel", &PROBE_N, &[(p_flash_1) as u32, (p_ref_2) as u32, (p_label_3) as u32], false); }
    let r_ = 'f: {
    if ((((((c.m.r32(p_ref_2) as i32)) >= (0i32)) as i32)) != 0) {
        { let a0_ = c.m.r32((p_flash_1).wrapping_add(36)); F::FlashClipList_GotoLabel(c, a0_, p_ref_2, p_label_3) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Flash_ClipGotoLabel", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_ClipSetCallbackA(c: &mut Ctx, mut p_flash_1: u32, mut p_ref_2: u32, mut p_fn_3: u32, mut p_arg_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_ClipSetCallbackA", &PROBE_N, &[(p_flash_1) as u32, (p_ref_2) as u32, (p_fn_3) as u32, (p_arg_4) as u32], false); }
    let r_ = 'f: {
    if ((((((c.m.r32(p_ref_2) as i32)) >= (0i32)) as i32)) != 0) {
        { let a0_ = c.m.r32((p_flash_1).wrapping_add(36)); F::FlashClipList_SetPreDraw(c, a0_, p_ref_2, ((((p_fn_3) as u64)) as i32), ((((p_arg_4) as u64)) as i32)) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Flash_ClipSetCallbackA", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_ClipSetCallbackB(c: &mut Ctx, mut p_flash_1: u32, mut p_ref_2: u32, mut p_fn_3: u32, mut p_arg_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_ClipSetCallbackB", &PROBE_N, &[(p_flash_1) as u32, (p_ref_2) as u32, (p_fn_3) as u32, (p_arg_4) as u32], false); }
    let r_ = 'f: {
    if ((((((c.m.r32(p_ref_2) as i32)) >= (0i32)) as i32)) != 0) {
        { let a0_ = c.m.r32((p_flash_1).wrapping_add(36)); F::FlashClipList_SetPostDraw(c, a0_, p_ref_2, ((((p_fn_3) as u64)) as i32), ((((p_arg_4) as u64)) as i32)) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Flash_ClipSetCallbackB", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_ClipSetCallbackC(c: &mut Ctx, mut p_flash_1: u32, mut p_ref_2: u32, mut p_fn_3: u32, mut p_arg_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_ClipSetCallbackC", &PROBE_N, &[(p_flash_1) as u32, (p_ref_2) as u32, (p_fn_3) as u32, (p_arg_4) as u32], false); }
    let r_ = 'f: {
    if ((((((c.m.r32(p_ref_2) as i32)) >= (0i32)) as i32)) != 0) {
        { let a0_ = c.m.r32((p_flash_1).wrapping_add(36)); F::FlashClipList_SetDrawOver(c, a0_, p_ref_2, ((((p_fn_3) as u64)) as i32), ((((p_arg_4) as u64)) as i32)) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Flash_ClipSetCallbackC", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_ClipSetFlags(c: &mut Ctx, mut p_flash_1: u32, mut p_ref_2: u32, mut p_props_3: i32, mut p_on_4: u8) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_ClipSetFlags", &PROBE_N, &[(p_flash_1) as u32, (p_ref_2) as u32, (p_props_3) as u32, (p_on_4) as u32], false); }
    let r_ = 'f: {
    if ((((((c.m.r32(p_ref_2) as i32)) >= (0i32)) as i32)) != 0) {
        if ((((p_props_3) & (1i32))) != 0) {
            if ((p_on_4) != 0) {
                { let a0_ = c.m.r32((p_flash_1).wrapping_add(36)); F::FlashClipList_Play(c, a0_, p_ref_2) };
            } else {
                { let a0_ = c.m.r32((p_flash_1).wrapping_add(36)); F::FlashClipList_Stop(c, a0_, p_ref_2) };
            }
        }
        if ((((p_props_3) & (2i32))) != 0) {
            { let a0_ = c.m.r32((p_flash_1).wrapping_add(36)); F::FlashClipList_SetFlags(c, a0_, p_ref_2, 2i32, p_on_4) };
        }
        if ((((p_props_3) & (1024i32))) != 0) {
            { let a0_ = c.m.r32((p_flash_1).wrapping_add(36)); F::FlashClipList_SetFlags(c, a0_, p_ref_2, 8i32, p_on_4) };
        }
        if ((((p_props_3) & (4i32))) != 0) {
            { let a0_ = c.m.r32((p_flash_1).wrapping_add(36)); F::FlashClipList_SetOverride(c, a0_, p_ref_2, 64i32, p_on_4) };
        }
        if ((((p_props_3) & (8i32))) != 0) {
            { let a0_ = c.m.r32((p_flash_1).wrapping_add(36)); F::FlashClipList_SetOverride(c, a0_, p_ref_2, 128i32, p_on_4) };
        }
        if ((((p_props_3) & (16i32))) != 0) {
            { let a0_ = c.m.r32((p_flash_1).wrapping_add(36)); F::FlashClipList_SetOverride(c, a0_, p_ref_2, 512i32, p_on_4) };
        }
        if ((((p_props_3) & (32i32))) != 0) {
            { let a0_ = c.m.r32((p_flash_1).wrapping_add(36)); F::FlashClipList_SetOverride(c, a0_, p_ref_2, 1024i32, p_on_4) };
        }
        if ((((p_props_3) & (64i32))) != 0) {
            { let a0_ = c.m.r32((p_flash_1).wrapping_add(36)); F::FlashClipList_SetOverride(c, a0_, p_ref_2, 2048i32, p_on_4) };
        }
        if ((((p_props_3) & (128i32))) != 0) {
            { let a0_ = c.m.r32((p_flash_1).wrapping_add(36)); F::FlashClipList_SetOverride(c, a0_, p_ref_2, 4096i32, p_on_4) };
        }
        if ((((p_props_3) & (256i32))) != 0) {
            { let a0_ = c.m.r32((p_flash_1).wrapping_add(36)); F::FlashClipList_SetOverride(c, a0_, p_ref_2, 8192i32, p_on_4) };
        }
        if ((((p_props_3) & (512i32))) != 0) {
            { let a0_ = c.m.r32((p_flash_1).wrapping_add(36)); F::FlashClipList_SetOverride(c, a0_, p_ref_2, 256i32, p_on_4) };
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Flash_ClipSetFlags", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_ClipSetOffset(c: &mut Ctx, mut p_flash_1: u32, mut p_ref_2: u32, mut p_x_3: i32, mut p_y_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_ClipSetOffset", &PROBE_N, &[(p_flash_1) as u32, (p_ref_2) as u32, (p_x_3) as u32, (p_y_4) as u32], false); }
    let r_ = 'f: {
    if ((((((c.m.r32(p_ref_2) as i32)) >= (0i32)) as i32)) != 0) {
        { let a0_ = c.m.r32((p_flash_1).wrapping_add(36)); F::FlashClipList_SetOffset(c, a0_, p_ref_2, p_x_3, p_y_4) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Flash_ClipSetOffset", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_ClipSetScale(c: &mut Ctx, mut p_flash_1: u32, mut p_ref_2: u32, mut p_x_3: F32, mut p_y_4: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_ClipSetScale", &PROBE_N, &[(p_flash_1) as u32, (p_ref_2) as u32, (p_x_3).0, (p_y_4).0], false); }
    let r_ = 'f: {
    if ((((((((((((((c.m.r32(p_ref_2) as i32)) >= (0i32)) as i32)) != 0) && ((((!(((rt::flt(p_x_3, F32(0x00000000)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) && ((((!(((rt::flt(p_y_4, F32(0x00000000)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
        { let a0_ = c.m.r32((p_flash_1).wrapping_add(36)); F::FlashClipList_SetScale(c, a0_, p_ref_2, p_x_3, p_y_4) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Flash_ClipSetScale", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_ClipSetAlpha(c: &mut Ctx, mut p_flash_1: u32, mut p_ref_2: u32, mut p_alpha_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_ClipSetAlpha", &PROBE_N, &[(p_flash_1) as u32, (p_ref_2) as u32, (p_alpha_3).0], false); }
    let r_ = 'f: {
    if ((((((((((c.m.r32(p_ref_2) as i32)) >= (0i32)) as i32)) != 0) && ((((!(((rt::flt(p_alpha_3, F32(0x00000000)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
        { let a0_ = c.m.r32((p_flash_1).wrapping_add(36)); F::FlashClipList_SetAlpha(c, a0_, p_ref_2, p_alpha_3) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Flash_ClipSetAlpha", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_ClipSetColor(c: &mut Ctx, mut p_flash_1: u32, mut p_ref_2: u32, mut p_color_3: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_ClipSetColor", &PROBE_N, &[(p_flash_1) as u32, (p_ref_2) as u32, (p_color_3).0], false); }
    let r_ = 'f: {
    if ((((((((((c.m.r32(p_ref_2) as i32)) >= (0i32)) as i32)) != 0) && ((((!(((rt::flt(p_color_3, F32(0x00000000)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
        { let a0_ = c.m.r32((p_flash_1).wrapping_add(36)); F::FlashClipList_SetColor(c, a0_, p_ref_2, p_color_3) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Flash_ClipSetColor", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_ClipSetTex(c: &mut Ctx, mut p_flash_1: u32, mut p_ref_2: u32, mut p_tex_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_ClipSetTex", &PROBE_N, &[(p_flash_1) as u32, (p_ref_2) as u32, (p_tex_3) as u32], false); }
    let r_ = 'f: {
    if ((((((c.m.r32(p_ref_2) as i32)) >= (0i32)) as i32)) != 0) {
        { let a0_ = c.m.r32((p_flash_1).wrapping_add(36)); F::FlashClipList_SetTex(c, a0_, p_ref_2, p_tex_3) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Flash_ClipSetTex", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_ClipSetUv(c: &mut Ctx, mut p_flash_1: u32, mut p_ref_2: u32, mut p_rect_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_ClipSetUv", &PROBE_N, &[(p_flash_1) as u32, (p_ref_2) as u32, (p_rect_3) as u32], false); }
    let r_ = 'f: {
    if ((((((c.m.r32(p_ref_2) as i32)) >= (0i32)) as i32)) != 0) {
        { let a0_ = c.m.r32((p_flash_1).wrapping_add(36)); F::FlashClipList_SetUv(c, a0_, p_ref_2, p_rect_3) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Flash_ClipSetUv", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_ClipGetPos(c: &mut Ctx, mut p_flash_1: u32, mut p_ref_2: u32, mut p_x_3: u32, mut p_y_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_ClipGetPos", &PROBE_N, &[(p_flash_1) as u32, (p_ref_2) as u32, (p_x_3) as u32, (p_y_4) as u32], false); }
    let r_ = 'f: {
    if ((((((c.m.r32(p_ref_2) as i32)) >= (0i32)) as i32)) != 0) {
        { let a0_ = c.m.r32((p_flash_1).wrapping_add(36)); F::FlashClipList_GetPos(c, a0_, p_ref_2, p_x_3, p_y_4) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Flash_ClipGetPos", &PROBE_N, &[], true); }
    r_
}

pub fn Flash_ClipGetAlpha(c: &mut Ctx, mut p_flash_1: u32, mut p_ref_2: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Flash_ClipGetAlpha", &PROBE_N, &[(p_flash_1) as u32, (p_ref_2) as u32], false); }
    let mut l_alpha_3: F32 = F32(0);
    let r_ = 'f: {
    l_alpha_3 = F32(0x00000000);
    if ((((((c.m.r32(p_ref_2) as i32)) < (0i32)) as i32)) != 0) {
        break 'f l_alpha_3;
    }
    break 'f { let a0_ = c.m.r32((p_flash_1).wrapping_add(36)); F::FlashClipList_GetAlpha(c, a0_, p_ref_2) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "Flash_ClipGetAlpha", &PROBE_N, &[], true); }
    r_
}

pub fn FlashShapePool_Init(c: &mut Ctx, mut p_pool_1: u32, mut p_count_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashShapePool_Init", &PROBE_N, &[(p_pool_1) as u32, (p_count_2) as u32], false); }
    let r_ = 'f: {
    { let v_ = p_count_2; let a_ = (p_pool_1).wrapping_add(4); c.m.w32(a_, v_); };
    if (((((c.m.r32(p_pool_1)) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = F::Heap_Alloc(c, (((p_count_2).wrapping_mul(64u32)) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = p_pool_1; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(p_pool_1); let a2_ = (c.m.r32((p_pool_1).wrapping_add(4))).wrapping_mul(64u32); F::memset(c, a0_, 0i32, a2_) }; };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashShapePool_Init", &PROBE_N, &[], true); }
    r_
}

pub fn FlashShapePool_Term(c: &mut Ctx, mut p_pool_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashShapePool_Term", &PROBE_N, &[(p_pool_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(p_pool_1)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(p_pool_1); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = p_pool_1; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashShapePool_Term", &PROBE_N, &[], true); }
    r_
}

pub fn FlashShapePool_Clear(c: &mut Ctx, mut p_pool_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashShapePool_Clear", &PROBE_N, &[(p_pool_1) as u32], false); }
    let r_ = 'f: {
    if (((((c.m.r32(p_pool_1)) != (((0i32) as u32))) as i32)) != 0) {
        { let _ = { let a0_ = c.m.r32(p_pool_1); let a2_ = (c.m.r32((p_pool_1).wrapping_add(4))).wrapping_mul(64u32); F::memset(c, a0_, 0i32, a2_) }; };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashShapePool_Clear", &PROBE_N, &[], true); }
    r_
}

pub fn FlashShapePool_Alloc(c: &mut Ctx, mut p_pool_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashShapePool_Alloc", &PROBE_N, &[(p_pool_1) as u32], false); }
    let mut l_i_2: u32 = 0;
    let mut l_shape_3: u32 = 0;
    let r_ = 'f: {
    l_shape_3 = c.m.r32(p_pool_1);
    {
        l_i_2 = ((0i32) as u32);
        'l1: loop {
            if !(((((l_i_2) < (c.m.r32((p_pool_1).wrapping_add(4)))) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((((c.m.r16(l_shape_3)) as i32)) == (0i32)) as i32)) != 0) {
                    { let _ = F::memset(c, l_shape_3, 0i32, 64u32); };
                    break 'f l_shape_3;
                }
            }
            { { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; }; { let o_ = l_shape_3; let n_ = (o_).wrapping_add(64); l_shape_3 = n_; }; };
        }
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashShapePool_Alloc", &PROBE_N, &[], true); }
    r_
}

pub fn FlashShape_Create(c: &mut Ctx, mut p_pool_1: u32, mut p_id_2: u16, mut p_mtx_3: u32, mut p_cx_4: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashShape_Create", &PROBE_N, &[(p_pool_1) as u32, (p_id_2) as u32, (p_mtx_3) as u32, (p_cx_4) as u32], false); }
    let mut l_shape_5: u32 = 0;
    let r_ = 'f: {
    l_shape_5 = F::FlashShapePool_Alloc(c, p_pool_1);
    if (((((l_shape_5) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    { let v_ = p_id_2; let a_ = (l_shape_5).wrapping_add(2); c.m.w16(a_, v_); };
    { let v_ = ((1i32) as u16); let a_ = l_shape_5; c.m.w16(a_, v_); };
    if (((((p_mtx_3) != (((0i32) as u32))) as i32)) != 0) {
        { let s_ = p_mtx_3; let d_ = (l_shape_5).wrapping_add(4); rt::memcpy(c, d_, s_, 36); };
    } else {
        F::Flash_MtxIdentity(c, (l_shape_5).wrapping_add(4));
    }
    if (((((p_cx_4) != (((0i32) as u32))) as i32)) != 0) {
        { let s_ = p_cx_4; let d_ = (l_shape_5).wrapping_add(40); rt::memcpy(c, d_, s_, 24); };
    } else {
        F::Flash_CxformIdentity(c, (l_shape_5).wrapping_add(40));
    }
    break 'f l_shape_5;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashShape_Create", &PROBE_N, &[], true); }
    r_
}

pub fn FlashShape_Free(c: &mut Ctx, mut p_shape_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashShape_Free", &PROBE_N, &[(p_shape_1) as u32], false); }
    let r_ = 'f: {
    { let _ = F::memset(c, p_shape_1, 0i32, 64u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashShape_Free", &PROBE_N, &[], true); }
    r_
}

pub fn FlashShape_Set(c: &mut Ctx, mut p_shape_1: u32, mut p_mtx_2: u32, mut p_cx_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashShape_Set", &PROBE_N, &[(p_shape_1) as u32, (p_mtx_2) as u32, (p_cx_3) as u32], false); }
    let r_ = 'f: {
    if (((((p_mtx_2) != (((0i32) as u32))) as i32)) != 0) {
        { let s_ = p_mtx_2; let d_ = (p_shape_1).wrapping_add(4); rt::memcpy(c, d_, s_, 36); };
    }
    if (((((p_cx_3) != (((0i32) as u32))) as i32)) != 0) {
        { let s_ = p_cx_3; let d_ = (p_shape_1).wrapping_add(40); rt::memcpy(c, d_, s_, 24); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashShape_Set", &PROBE_N, &[], true); }
    r_
}

pub fn FlashShape_Draw(c: &mut Ctx, mut p_shape_1: u32, mut p_prop_2: u32, mut p_flash_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashShape_Draw", &PROBE_N, &[(p_shape_1) as u32, (p_prop_2) as u32, (p_flash_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(112) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 112);
    let mut l_data_4: u32 = 0;
    let mut l_img_5: u32 = 0;
    let mut l_tex_6: u32 = 0;
    let mut l_i_7: i32 = 0;
    let mut l_pq_8: u32 = 0;
    let mut l_pm_9: u32 = 0;
    let r_ = 'f: {
        l_data_4 = { let a0_ = c.m.r32(p_flash_3); let a2_ = c.m.r16((p_shape_1).wrapping_add(2)); F::Flash_GetRecord(c, a0_, ((4i32) as u8), a2_) };
        { let _ = F::memcpy(c, fp.wrapping_add(96), (l_data_4).wrapping_add(((8i32) as u32)), ((2i32) as u32)); };
        {
            l_i_7 = 0i32;
            'l1: loop {
                if !(((((l_i_7) < (((c.m.r16(fp.wrapping_add(96))) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_pm_9 = fp.wrapping_add(24);
                    l_pq_8 = l_pm_9;
                    l_tex_6 = ((0i32) as u32);
                    { let _ = F::memset(c, l_pm_9, 0i32, 72u32); };
                    { let _ = F::memset(c, fp, 0i32, 24u32); };
                    { let _ = F::memcpy(c, fp, ((l_data_4).wrapping_add(((10i32) as u32))).wrapping_add((((l_i_7) as u32)).wrapping_mul(24u32)), 24u32); };
                    if ((((((((c.m.r16(fp) as i16)) as i32)) >= (0i32)) as i32)) != 0) {
                        l_img_5 = { let a0_ = c.m.r32(p_flash_3); let a2_ = (((c.m.r16(fp) as i16)) as u16); F::Flash_GetRecord(c, a0_, ((3i32) as u8), a2_) };
                        l_tex_6 = { let a_ = (c.m.r32((p_flash_3).wrapping_add(4))).wrapping_add(((((c.m.r16(fp) as i16)) as u32)).wrapping_mul(4)); c.m.r32(a_) };
                        if (((((l_tex_6) == (((0i32) as u32))) as i32)) != 0) {
                            break 'c2;
                        }
                        { let v_ = 0i32; let a_ = ((fp.wrapping_add(24)).wrapping_add(36)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = (fp.wrapping_add(24)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = ((fp.wrapping_add(24)).wrapping_add(36)).wrapping_add(16); c.m.w32(a_, (v_) as u32); };
                        { let _ = F::memcpy(c, fp.wrapping_add(100), (l_img_5).wrapping_add(((8i32) as u32)), ((2i32) as u32)); };
                        { let _ = F::memcpy(c, fp.wrapping_add(104), (l_img_5).wrapping_add(((10i32) as u32)), ((2i32) as u32)); };
                        { let v_ = ((c.m.r16(fp.wrapping_add(100))) as i32); let a_ = ((fp.wrapping_add(24)).wrapping_add(36)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r16(fp.wrapping_add(104))) as i32); let a_ = ((fp.wrapping_add(24)).wrapping_add(36)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    }
                    { let v_ = rt::i2f((c.m.r32((fp).wrapping_add(8)) as i32)); let a_ = (((fp.wrapping_add(24)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::i2f((c.m.r32((fp).wrapping_add(12)) as i32)); let a_ = (((fp.wrapping_add(24)).wrapping_add(4)).wrapping_add((((3i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::i2f((c.m.r32((fp).wrapping_add(16)) as i32)); let a_ = (((fp.wrapping_add(24)).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::i2f((c.m.r32((fp).wrapping_add(20)) as i32)); let a_ = (((fp.wrapping_add(24)).wrapping_add(4)).wrapping_add((((3i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::i2f((c.m.r32((fp).wrapping_add(8)) as i32)); let a_ = (((fp.wrapping_add(24)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::i2f((c.m.r32((fp).wrapping_add(12)) as i32)); let a_ = (((fp.wrapping_add(24)).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(8))).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::i2f((c.m.r32((fp).wrapping_add(16)) as i32)); let a_ = (((fp.wrapping_add(24)).wrapping_add(4)).wrapping_add((((0i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = rt::i2f((c.m.r32((fp).wrapping_add(20)) as i32)); let a_ = (((fp.wrapping_add(24)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(8))).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    { let v_ = (((c.m.r32((fp).wrapping_add(4))).wrapping_shr((24i32) as u32)) as i32); let a_ = ((fp.wrapping_add(24)).wrapping_add(56)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (((((c.m.r32((fp).wrapping_add(4))).wrapping_shr((16i32) as u32)) & (((255i32) as u32)))) as i32); let a_ = ((fp.wrapping_add(24)).wrapping_add(56)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (((((c.m.r32((fp).wrapping_add(4))) & (((65280i32) as u32)))).wrapping_shr((8i32) as u32)) as i32); let a_ = ((fp.wrapping_add(24)).wrapping_add(56)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((((c.m.r32((fp).wrapping_add(4))) & (((255i32) as u32)))) as i32); let a_ = ((fp.wrapping_add(24)).wrapping_add(56)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    F::FlashDraw_TransformQuad(c, l_pq_8, (p_shape_1).wrapping_add(4), (p_shape_1).wrapping_add(40), p_prop_2, (p_flash_3).wrapping_add(24));
                    F::FlashDraw_Quad(c, l_tex_6, l_pq_8);
                }
                { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "FlashShape_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn FlashClipList_Init(c: &mut Ctx, mut p_list_1: u32, mut p_flash_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_Init", &PROBE_N, &[(p_list_1) as u32, (p_flash_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_tag_3: u32 = 0;
    let mut l_rec_4: u32 = 0;
    let mut l_clip_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let r_ = 'f: {
        { let _ = F::memset(c, fp, 0i32, 8u32); };
        l_tag_3 = { let a0_ = c.m.r32(p_flash_2); F::Flash_FindTag(c, a0_, ((2i32) as u8)) };
        if (((((l_tag_3) == (((0i32) as u32))) as i32)) != 0) {
            break 'f;
        }
        { let _ = F::memcpy(c, fp, l_tag_3, 8u32); };
        if (((((c.m.r32(p_list_1)) == (((0i32) as u32))) as i32)) != 0) {
            { let v_ = { let a0_ = (((((c.m.r16((fp).wrapping_add(2))) as u32)).wrapping_mul(228u32)) as i32); F::Heap_Alloc(c, a0_, ((32i32) as u32), 0i32, 2i32) }; let a_ = p_list_1; c.m.w32(a_, v_); };
            { let _ = { let a0_ = c.m.r32(p_list_1); let a2_ = (((c.m.r16((fp).wrapping_add(2))) as u32)).wrapping_mul(228u32); F::memset(c, a0_, 0i32, a2_) }; };
        }
        { let v_ = ((c.m.r16((fp).wrapping_add(2))) as u32); let a_ = (p_list_1).wrapping_add(4); c.m.w32(a_, v_); };
        {
            l_i_6 = 0i32;
            'l1: loop {
                if !(((((l_i_6) < (((c.m.r16((fp).wrapping_add(2))) as i32))) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_clip_5 = (c.m.r32(p_list_1)).wrapping_add((((l_i_6) as u32)).wrapping_mul(228));
                    { let v_ = ((l_i_6) as u16); let a_ = (l_clip_5).wrapping_add(66); c.m.w16(a_, v_); };
                    l_rec_4 = { let a1_ = c.m.r16((l_clip_5).wrapping_add(66)); F::Flash_SkipRecords(c, l_tag_3, a1_) };
                    { let _ = F::memcpy(c, (l_clip_5).wrapping_add(64), (l_rec_4).wrapping_add(((8i32) as u32)), ((2i32) as u32)); };
                    { let v_ = ((0i32) as u32); let a_ = (l_clip_5).wrapping_add(68); c.m.w32(a_, v_); };
                    { let _ = F::memset(c, (l_clip_5).wrapping_add(72), 0i32, 108u32); };
                    F::Flash_MtxIdentity(c, ((l_clip_5).wrapping_add(72)).wrapping_add(48));
                    F::Flash_CxformIdentity(c, ((l_clip_5).wrapping_add(72)).wrapping_add(84));
                    { let v_ = F32(0x3f800000); let a_ = ((l_clip_5).wrapping_add(72)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                    { let a1_ = { let a0_ = c.m.r32(p_flash_2); let a2_ = c.m.r16((l_clip_5).wrapping_add(64)); F::Flash_GetRecord(c, a0_, ((6i32) as u8), a2_) }; F::FlashTl_Init(c, (l_clip_5).wrapping_add(204), a1_, p_flash_2) };
                    { let r_ = ((c.m.r16(((l_clip_5).wrapping_add(204)).wrapping_add(22))) as u32); let a_ = (p_list_1).wrapping_add(8); let n_ = (c.m.r32(a_)).wrapping_add(r_); c.m.w32(a_, n_); };
                }
                { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_Init", &PROBE_N, &[], true); }
    r_
}

pub fn FlashClipList_Term(c: &mut Ctx, mut p_list_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_Term", &PROBE_N, &[(p_list_1) as u32], false); }
    let mut l_i_2: u32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = ((0i32) as u32);
        'l1: loop {
            if !(((((l_i_2) < (c.m.r32((p_list_1).wrapping_add(4)))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(p_list_1)).wrapping_add((l_i_2).wrapping_mul(228))).wrapping_add(204); F::FlashTl_Term(c, a0_) };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    if (((((c.m.r32(p_list_1)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(p_list_1); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = p_list_1; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_Term", &PROBE_N, &[], true); }
    r_
}

pub fn FlashClipList_Advance(c: &mut Ctx, mut p_list_1: u32, mut p_count_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_Advance", &PROBE_N, &[(p_list_1) as u32, (p_count_2) as u32], false); }
    let mut l_i_3: u32 = 0;
    let r_ = 'f: {
    {
        l_i_3 = ((0i32) as u32);
        'l1: loop {
            if !(((((l_i_3) < (c.m.r32((p_list_1).wrapping_add(4)))) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((({ let a_ = ((c.m.r32(p_list_1)).wrapping_add((l_i_3).wrapping_mul(228))).wrapping_add(68); c.m.r32(a_) }) & (((8i32) as u32)))) != 0) {
                    { let a0_ = ((c.m.r32(p_list_1)).wrapping_add((l_i_3).wrapping_mul(228))).wrapping_add(204); F::FlashTl_Advance(c, a0_, p_count_2, ((1i32) as u8), ((1i32) as u8)) };
                } else {
                    { let a0_ = ((c.m.r32(p_list_1)).wrapping_add((l_i_3).wrapping_mul(228))).wrapping_add(204); F::FlashTl_Advance(c, a0_, p_count_2, ((1i32) as u8), ((0i32) as u8)) };
                }
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_Advance", &PROBE_N, &[], true); }
    r_
}

pub fn FlashClipList_Reset(c: &mut Ctx, mut p_list_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_Reset", &PROBE_N, &[(p_list_1) as u32], false); }
    let mut l_i_2: u32 = 0;
    let r_ = 'f: {
    {
        l_i_2 = ((0i32) as u32);
        'l1: loop {
            if !(((((l_i_2) < (c.m.r32((p_list_1).wrapping_add(4)))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = (c.m.r32(p_list_1)).wrapping_add((l_i_2).wrapping_mul(228)); F::FlashClip_Reset(c, a0_, 0i32) };
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn FlashClipList_FindFree(c: &mut Ctx, mut p_list_1: u32, mut p_id_2: u16) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_FindFree", &PROBE_N, &[(p_list_1) as u32, (p_id_2) as u32], false); }
    let mut l_i_3: u32 = 0;
    let mut l_clip_4: u32 = 0;
    let r_ = 'f: {
    l_clip_4 = c.m.r32(p_list_1);
    {
        l_i_3 = ((0i32) as u32);
        'l1: loop {
            if !(((((l_i_3) < (c.m.r32((p_list_1).wrapping_add(4)))) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((((((!((((c.m.r32((l_clip_4).wrapping_add(68))) & (((1i32) as u32)))) != 0)) as i32)) != 0) && (((((((c.m.r16((l_clip_4).wrapping_add(64))) as i32)) == (((p_id_2) as i32))) as i32)) != 0)) as i32)) != 0) {
                    break 'f l_clip_4;
                }
            }
            { { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; }; { let o_ = l_clip_4; let n_ = (o_).wrapping_add(228); l_clip_4 = n_; }; };
        }
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_FindFree", &PROBE_N, &[], true); }
    r_
}

pub fn FlashClip_Start(c: &mut Ctx, mut p_list_1: u32, mut p_id_2: u16, mut p_name_3: u32, mut p_mtx_4: u32, mut p_cx_5: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashClip_Start", &PROBE_N, &[(p_list_1) as u32, (p_id_2) as u32, (p_name_3) as u32, (p_mtx_4) as u32, (p_cx_5) as u32], false); }
    let mut l_clip_6: u32 = 0;
    let r_ = 'f: {
    l_clip_6 = F::FlashClipList_FindFree(c, p_list_1, p_id_2);
    if (((((l_clip_6) == (((0i32) as u32))) as i32)) != 0) {
        break 'f ((0i32) as u32);
    }
    { let _ = F::memset(c, l_clip_6, 0i32, 64u32); };
    { let _ = { let a2_ = F::strlen(c, p_name_3); F::memcpy(c, l_clip_6, p_name_3, a2_) }; };
    { let r_ = ((((1i32) | (2i32))) as u32); let a_ = (l_clip_6).wrapping_add(68); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
    { let _ = F::memset(c, (l_clip_6).wrapping_add(72), 0i32, 108u32); };
    { let v_ = F32(0x3f800000); let a_ = ((l_clip_6).wrapping_add(72)).wrapping_add(12); c.m.w32(a_, (v_).0); };
    if (((((p_mtx_4) != (((0i32) as u32))) as i32)) != 0) {
        { let s_ = p_mtx_4; let d_ = ((l_clip_6).wrapping_add(72)).wrapping_add(48); rt::memcpy(c, d_, s_, 36); };
    } else {
        F::Flash_MtxIdentity(c, ((l_clip_6).wrapping_add(72)).wrapping_add(48));
    }
    if (((((p_cx_5) != (((0i32) as u32))) as i32)) != 0) {
        { let s_ = p_cx_5; let d_ = ((l_clip_6).wrapping_add(72)).wrapping_add(84); rt::memcpy(c, d_, s_, 24); };
    } else {
        F::Flash_CxformIdentity(c, ((l_clip_6).wrapping_add(72)).wrapping_add(84));
    }
    { let v_ = ((0i32) as u32); let a_ = (l_clip_6).wrapping_add(180); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_clip_6).wrapping_add(184); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_clip_6).wrapping_add(188); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (l_clip_6).wrapping_add(192); c.m.w32(a_, v_); };
    { let r_ = 1i32; let a_ = ((l_clip_6).wrapping_add(204)).wrapping_add(21); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
    if (((((c.m.r32(((l_clip_6).wrapping_add(204)).wrapping_add(16))) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = c.m.r32(((l_clip_6).wrapping_add(204)).wrapping_add(16)); let a_ = ((l_clip_6).wrapping_add(204)).wrapping_add(12); c.m.w32(a_, v_); };
        { let r_ = 4i32; let a_ = ((l_clip_6).wrapping_add(204)).wrapping_add(21); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
        { let v_ = ((0i32) as u32); let a_ = ((l_clip_6).wrapping_add(204)).wrapping_add(16); c.m.w32(a_, v_); };
    }
    break 'f l_clip_6;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashClip_Start", &PROBE_N, &[], true); }
    r_
}

pub fn FlashClip_Set(c: &mut Ctx, mut p_clip_1: u32, mut p_mtx_2: u32, mut p_cx_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashClip_Set", &PROBE_N, &[(p_clip_1) as u32, (p_mtx_2) as u32, (p_cx_3) as u32], false); }
    let r_ = 'f: {
    if (((((p_mtx_2) != (((0i32) as u32))) as i32)) != 0) {
        { let s_ = p_mtx_2; let d_ = ((p_clip_1).wrapping_add(72)).wrapping_add(48); rt::memcpy(c, d_, s_, 36); };
    }
    if (((((p_cx_3) != (((0i32) as u32))) as i32)) != 0) {
        { let s_ = p_cx_3; let d_ = ((p_clip_1).wrapping_add(72)).wrapping_add(84); rt::memcpy(c, d_, s_, 24); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashClip_Set", &PROBE_N, &[], true); }
    r_
}

pub fn FlashClip_Reset(c: &mut Ctx, mut p_clip_1: u32, mut p_arg_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashClip_Reset", &PROBE_N, &[(p_clip_1) as u32, (p_arg_2) as u32], false); }
    let r_ = 'f: {
    { let _ = F::memset(c, p_clip_1, 0i32, 64u32); };
    { let v_ = ((0i32) as u32); let a_ = (p_clip_1).wrapping_add(68); c.m.w32(a_, v_); };
    { let _ = F::memset(c, (p_clip_1).wrapping_add(72), 0i32, 108u32); };
    { let v_ = F32(0x3f800000); let a_ = ((p_clip_1).wrapping_add(72)).wrapping_add(12); c.m.w32(a_, (v_).0); };
    F::Flash_MtxIdentity(c, ((p_clip_1).wrapping_add(72)).wrapping_add(48));
    F::Flash_CxformIdentity(c, ((p_clip_1).wrapping_add(72)).wrapping_add(84));
    { let v_ = ((0i32) as u32); let a_ = (p_clip_1).wrapping_add(180); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (p_clip_1).wrapping_add(184); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (p_clip_1).wrapping_add(188); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (p_clip_1).wrapping_add(192); c.m.w32(a_, v_); };
    F::FlashTl_Rewind(c, (p_clip_1).wrapping_add(204), p_arg_2);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashClip_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn FlashClipList_FindByName(c: &mut Ctx, mut p_list_1: u32, mut p_name_2: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_FindByName", &PROBE_N, &[(p_list_1) as u32, (p_name_2) as u32], false); }
    let mut l_i_3: u32 = 0;
    let mut l_clip_4: u32 = 0;
    let r_ = 'f: {
    l_clip_4 = c.m.r32(p_list_1);
    {
        l_i_3 = ((0i32) as u32);
        'l1: loop {
            if !(((((l_i_3) < (c.m.r32((p_list_1).wrapping_add(4)))) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((F::strcmp(c, l_clip_4, p_name_2)) == (0i32)) as i32)) != 0) {
                    break 'f l_clip_4;
                }
            }
            { { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; }; { let o_ = l_clip_4; let n_ = (o_).wrapping_add(228); l_clip_4 = n_; }; };
        }
    }
    break 'f ((0i32) as u32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_FindByName", &PROBE_N, &[], true); }
    r_
}

pub fn FlashClipList_FindRef(c: &mut Ctx, mut p_list_1: u32, mut p_parent_2: u32, mut p_name_3: u32, mut p_out_4: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_FindRef", &PROBE_N, &[(p_list_1) as u32, (p_parent_2) as u32, (p_name_3) as u32, (p_out_4) as u32], false); }
    let mut l_found_5: i32 = 0;
    let mut l_i_6: u32 = 0;
    let mut l_clip_7: u32 = 0;
    let mut l_slot_8: u32 = 0;
    let mut l_j_9: i32 = 0;
    let mut l_child_10: u32 = 0;
    let r_ = 'f: {
    l_found_5 = 0i32;
    { let _ = F::memset(c, p_out_4, 0i32, 8u32); };
    if (((((p_parent_2) != (((0i32) as u32))) as i32)) != 0) {
        l_clip_7 = c.m.r32(p_list_1);
        {
            l_i_6 = ((0i32) as u32);
            'l1: loop {
                if !(((((l_i_6) < (c.m.r32((p_list_1).wrapping_add(4)))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((F::strcmp(c, l_clip_7, p_parent_2)) == (0i32)) as i32)) != 0) {
                        l_slot_8 = c.m.r32(((l_clip_7).wrapping_add(204)).wrapping_add(4));
                        {
                            l_j_9 = 0i32;
                            'l3: loop {
                                if !(((((l_j_9) < (((c.m.r16(((l_clip_7).wrapping_add(204)).wrapping_add(22))) as i32))) as i32)) != 0) { break 'l3; }
                                'c4: {
                                    l_child_10 = c.m.r32((l_slot_8).wrapping_add(4));
                                    { let o_ = l_slot_8; let n_ = (o_).wrapping_add(8); l_slot_8 = n_; };
                                    if (((((((((l_child_10) != (((0i32) as u32))) as i32)) != 0) && (((((F::strcmp(c, l_child_10, p_name_3)) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                                        l_found_5 = 1i32;
                                        { let v_ = ((c.m.r16((l_child_10).wrapping_add(66))) as i32); let a_ = p_out_4; c.m.w32(a_, (v_) as u32); };
                                        break 'l3;
                                    }
                                }
                                { let o_ = l_j_9; let n_ = (o_).wrapping_add(1); l_j_9 = n_; };
                            }
                        }
                        break 'l1;
                    }
                }
                { { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; }; { let o_ = l_clip_7; let n_ = (o_).wrapping_add(228); l_clip_7 = n_; }; };
            }
        }
    } else {
        l_clip_7 = c.m.r32(p_list_1);
        {
            l_i_6 = ((0i32) as u32);
            'l5: loop {
                if !(((((l_i_6) < (c.m.r32((p_list_1).wrapping_add(4)))) as i32)) != 0) { break 'l5; }
                'c6: {
                    if (((((F::strcmp(c, l_clip_7, p_name_3)) == (0i32)) as i32)) != 0) {
                        if ((l_found_5) != 0) {
                            { let a_ = (p_out_4).wrapping_add(4); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        } else {
                            { let v_ = ((l_i_6) as i32); let a_ = p_out_4; c.m.w32(a_, (v_) as u32); };
                            l_found_5 = 1i32;
                        }
                    }
                }
                { { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; }; { let o_ = l_clip_7; let n_ = (o_).wrapping_add(228); l_clip_7 = n_; }; };
            }
        }
    }
    if ((((!((l_found_5) != 0)) as i32)) != 0) {
        { let v_ = 0i32; let a_ = (p_out_4).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = (1i32).wrapping_neg(); let a_ = p_out_4; c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_FindRef", &PROBE_N, &[], true); }
    r_
}

pub fn FlashClipList_Play(c: &mut Ctx, mut p_list_1: u32, mut p_ref_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_Play", &PROBE_N, &[(p_list_1) as u32, (p_ref_2) as u32], false); }
    let mut l_clip_3: u32 = 0;
    let mut l_n_4: i32 = 0;
    let mut l_i_5: u32 = 0;
    let r_ = 'f: {
    l_clip_3 = (c.m.r32(p_list_1)).wrapping_add(((((c.m.r32(p_ref_2) as i32)) as u32)).wrapping_mul(228));
    { let r_ = 1i32; let a_ = ((l_clip_3).wrapping_add(204)).wrapping_add(21); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
    l_n_4 = (c.m.r32((p_ref_2).wrapping_add(4)) as i32);
    if (((((l_n_4) != (0i32)) as i32)) != 0) {
        {
            l_i_5 = ((1i32) as u32);
            'l1: loop {
                if !(((((l_i_5) < ((c.m.r32((p_list_1).wrapping_add(4))).wrapping_sub((((c.m.r32(p_ref_2) as i32)) as u32)))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((F::strcmp(c, (l_clip_3).wrapping_add((l_i_5).wrapping_mul(228)), l_clip_3)) == (0i32)) as i32)) != 0) {
                        { let r_ = 1i32; let a_ = (((l_clip_3).wrapping_add((l_i_5).wrapping_mul(228))).wrapping_add(204)).wrapping_add(21); let n_ = ((((((c.m.r8(a_)) as i32)) | (r_))) as u8); c.m.w8(a_, n_); };
                        { let o_ = l_n_4; let n_ = (o_).wrapping_sub(1); l_n_4 = n_; };
                        if (((((l_n_4) == (0i32)) as i32)) != 0) {
                            break 'l1;
                        }
                    }
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_Play", &PROBE_N, &[], true); }
    r_
}

pub fn FlashClipList_Stop(c: &mut Ctx, mut p_list_1: u32, mut p_ref_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_Stop", &PROBE_N, &[(p_list_1) as u32, (p_ref_2) as u32], false); }
    let mut l_clip_3: u32 = 0;
    let mut l_n_4: i32 = 0;
    let mut l_i_5: u32 = 0;
    let r_ = 'f: {
    l_clip_3 = (c.m.r32(p_list_1)).wrapping_add(((((c.m.r32(p_ref_2) as i32)) as u32)).wrapping_mul(228));
    { let r_ = (!(1i32)); let a_ = ((l_clip_3).wrapping_add(204)).wrapping_add(21); let n_ = ((((((c.m.r8(a_)) as i32)) & (r_))) as u8); c.m.w8(a_, n_); };
    l_n_4 = (c.m.r32((p_ref_2).wrapping_add(4)) as i32);
    if (((((l_n_4) != (0i32)) as i32)) != 0) {
        {
            l_i_5 = ((1i32) as u32);
            'l1: loop {
                if !(((((l_i_5) < ((c.m.r32((p_list_1).wrapping_add(4))).wrapping_sub((((c.m.r32(p_ref_2) as i32)) as u32)))) as i32)) != 0) { break 'l1; }
                'c2: {
                    if (((((F::strcmp(c, (l_clip_3).wrapping_add((l_i_5).wrapping_mul(228)), l_clip_3)) == (0i32)) as i32)) != 0) {
                        { let r_ = (!(1i32)); let a_ = (((l_clip_3).wrapping_add((l_i_5).wrapping_mul(228))).wrapping_add(204)).wrapping_add(21); let n_ = ((((((c.m.r8(a_)) as i32)) & (r_))) as u8); c.m.w8(a_, n_); };
                        { let o_ = l_n_4; let n_ = (o_).wrapping_sub(1); l_n_4 = n_; };
                        if (((((l_n_4) == (0i32)) as i32)) != 0) {
                            break 'l1;
                        }
                    }
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlashClipList_Stop", &PROBE_N, &[], true); }
    r_
}
