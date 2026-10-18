#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn eft_h__EftShot_TypeRow(c: &mut Ctx, mut p_type_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "eft_h__EftShot_TypeRow", &PROBE_N, &[(p_type_1) as u32], false); }
    let mut l_row_2: i32 = 0;
    let r_ = 'f: {
    l_row_2 = (p_type_1).wrapping_add(1i32);
    if (((((p_type_1) == ((1i32).wrapping_neg())) as i32)) != 0) {
        l_row_2 = 0i32;
    }
    break 'f l_row_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "eft_h__EftShot_TypeRow", &PROBE_N, &[], true); }
    r_
}

pub fn EftShot_BuildParam(c: &mut Ctx, mut p_chr_1: i32, mut p_slot_2: i32, mut p_p_3: u32, mut p_blank_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShot_BuildParam", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32, (p_p_3) as u32, (p_blank_4) as u32], false); }
    let mut l_i_5: i32 = 0;
    let mut l_d_6: u32 = 0;
    let mut l_n_7: i32 = 0;
    let mut l_d_8: u32 = 0;
    let mut l_n_9: i32 = 0;
    let r_ = 'f: {
    { let _ = F::memset(c, p_p_3, 0i32, 140u32); };
    if ((p_blank_4) != 0) {
        { let v_ = ((0i32) as i16); let a_ = p_p_3; c.m.w16(a_, (v_) as u16); };
        { let v_ = ((100i32) as i16); let a_ = (p_p_3).wrapping_add(2); c.m.w16(a_, (v_) as u16); };
        { let v_ = ((1i32) as i8); let a_ = (p_p_3).wrapping_add(4); c.m.w8(a_, (v_) as u8); };
        { let v_ = ((0i32) as i8); let a_ = (p_p_3).wrapping_add(5); c.m.w8(a_, (v_) as u8); };
        { let v_ = ((0i32) as i8); let a_ = (p_p_3).wrapping_add(6); c.m.w8(a_, (v_) as u8); };
        { let v_ = ((0i32) as i8); let a_ = (p_p_3).wrapping_add(7); c.m.w8(a_, (v_) as u8); };
        { let v_ = ((0i32) as i8); let a_ = (p_p_3).wrapping_add(8); c.m.w8(a_, (v_) as u8); };
        { let v_ = ((1i32) as i8); let a_ = (p_p_3).wrapping_add(9); c.m.w8(a_, (v_) as u8); };
        { let v_ = ((0i32) as i8); let a_ = (p_p_3).wrapping_add(10); c.m.w8(a_, (v_) as u8); };
        { let v_ = ((0i32) as i8); let a_ = (p_p_3).wrapping_add(11); c.m.w8(a_, (v_) as u8); };
        { let v_ = ((0i32) as i8); let a_ = (p_p_3).wrapping_add(12); c.m.w8(a_, (v_) as u8); };
        { let v_ = ((0i32) as i8); let a_ = (p_p_3).wrapping_add(13); c.m.w8(a_, (v_) as u8); };
        { let v_ = ((3i32) as i8); let a_ = ((p_p_3).wrapping_add(14)).wrapping_add(((0i32) as u32)); c.m.w8(a_, (v_) as u8); };
        { let v_ = ((3i32) as i8); let a_ = ((p_p_3).wrapping_add(14)).wrapping_add(((1i32) as u32)); c.m.w8(a_, (v_) as u8); };
        { let v_ = ((3i32) as i8); let a_ = ((p_p_3).wrapping_add(14)).wrapping_add(((2i32) as u32)); c.m.w8(a_, (v_) as u8); };
        { let v_ = ((3i32) as i8); let a_ = ((p_p_3).wrapping_add(14)).wrapping_add(((3i32) as u32)); c.m.w8(a_, (v_) as u8); };
        { let v_ = ((3i32) as i8); let a_ = ((p_p_3).wrapping_add(14)).wrapping_add(((4i32) as u32)); c.m.w8(a_, (v_) as u8); };
        { let v_ = ((3i32) as i8); let a_ = ((p_p_3).wrapping_add(14)).wrapping_add(((5i32) as u32)); c.m.w8(a_, (v_) as u8); };
        { let v_ = ((0i32) as i16); let a_ = ((p_p_3).wrapping_add(22)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.w16(a_, (v_) as u16); };
        { let v_ = ((10i32) as i16); let a_ = ((p_p_3).wrapping_add(22)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.w16(a_, (v_) as u16); };
        { let v_ = ((20i32) as i16); let a_ = ((p_p_3).wrapping_add(22)).wrapping_add((((2i32) as u32)).wrapping_mul(2)); c.m.w16(a_, (v_) as u16); };
        { let v_ = (((1i32).wrapping_neg()) as i16); let a_ = ((p_p_3).wrapping_add(22)).wrapping_add((((3i32) as u32)).wrapping_mul(2)); c.m.w16(a_, (v_) as u16); };
        { let v_ = (((1i32).wrapping_neg()) as i16); let a_ = ((p_p_3).wrapping_add(22)).wrapping_add((((4i32) as u32)).wrapping_mul(2)); c.m.w16(a_, (v_) as u16); };
        { let v_ = (((1i32).wrapping_neg()) as i16); let a_ = ((p_p_3).wrapping_add(22)).wrapping_add((((5i32) as u32)).wrapping_mul(2)); c.m.w16(a_, (v_) as u16); };
        { let v_ = ((0i32) as i16); let a_ = (p_p_3).wrapping_add(34); c.m.w16(a_, (v_) as u16); };
        { let v_ = ((0i32) as i16); let a_ = (p_p_3).wrapping_add(36); c.m.w16(a_, (v_) as u16); };
        { let v_ = 120i32; let a_ = (p_p_3).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
        { let v_ = F32(0x00000000); let a_ = (p_p_3).wrapping_add(44); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (p_p_3).wrapping_add(48); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x42200000); let a_ = (p_p_3).wrapping_add(52); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x00000000); let a_ = (p_p_3).wrapping_add(56); c.m.w32(a_, (v_).0); };
        { let v_ = 0i32; let a_ = (p_p_3).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((0i32) as i16); let a_ = (p_p_3).wrapping_add(64); c.m.w16(a_, (v_) as u16); };
        { let v_ = ((0i32) as i16); let a_ = (p_p_3).wrapping_add(66); c.m.w16(a_, (v_) as u16); };
        { let v_ = ((1i32) as i16); let a_ = (p_p_3).wrapping_add(68); c.m.w16(a_, (v_) as u16); };
        { let v_ = F32(0x3f800000); let a_ = (p_p_3).wrapping_add(80); c.m.w32(a_, (v_).0); };
        { let v_ = (((1i32).wrapping_neg()) as i8); let a_ = (p_p_3).wrapping_add(20); c.m.w8(a_, (v_) as u8); };
        { let v_ = ((0i32) as i16); let a_ = (p_p_3).wrapping_add(72); c.m.w16(a_, (v_) as u16); };
        { let v_ = ((0i32) as i16); let a_ = (p_p_3).wrapping_add(74); c.m.w16(a_, (v_) as u16); };
        { let v_ = F32(0x00000000); let a_ = (p_p_3).wrapping_add(76); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (p_p_3).wrapping_add(84); c.m.w32(a_, (v_).0); };
        {
            l_i_5 = 0i32;
            'l1: loop {
                if !(((((l_i_5) < (8i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = (((1i32).wrapping_neg()) as i8); let a_ = ((p_p_3).wrapping_add(88)).wrapping_add(((l_i_5) as u32)); c.m.w8(a_, (v_) as u8); };
                    { let v_ = F32(0x00000000); let a_ = ((p_p_3).wrapping_add(96)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                }
                { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
            }
        }
        { let v_ = F32(0x3dcccccc); let a_ = (p_p_3).wrapping_add(132); c.m.w32(a_, (v_).0); };
        { let v_ = F32(0x3f800000); let a_ = (p_p_3).wrapping_add(136); c.m.w32(a_, (v_).0); };
        { let v_ = ((0i32) as i16); let a_ = (p_p_3).wrapping_add(128); c.m.w16(a_, (v_) as u16); };
    } else {
        if (((((p_slot_2) < (2i32)) as i32)) != 0) {
            l_d_6 = F::BtlCharApi_GetPlayerSkillData(c, p_chr_1);
            l_n_7 = p_slot_2;
            { let v_ = ((c.m.r16(((l_d_6).wrapping_add(16)).wrapping_add((((l_n_7) as u32)).wrapping_mul(2)))) as i16); let a_ = p_p_3; c.m.w16(a_, (v_) as u16); };
            { let v_ = ((c.m.r16(((l_d_6).wrapping_add(20)).wrapping_add((((l_n_7) as u32)).wrapping_mul(2)))) as i16); let a_ = (p_p_3).wrapping_add(2); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((0i32) as i8); let a_ = (p_p_3).wrapping_add(4); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((c.m.r8(((l_d_6).wrapping_add(98)).wrapping_add(((l_n_7) as u32)))) as i8); let a_ = (p_p_3).wrapping_add(5); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((c.m.r8(((l_d_6).wrapping_add(56)).wrapping_add(((l_n_7) as u32)))) as i8); let a_ = (p_p_3).wrapping_add(6); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((0i32) as i8); let a_ = (p_p_3).wrapping_add(7); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((c.m.r8(((l_d_6).wrapping_add(62)).wrapping_add(((l_n_7) as u32)))) as i8); let a_ = (p_p_3).wrapping_add(8); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((c.m.r8(((l_d_6).wrapping_add(58)).wrapping_add(((l_n_7) as u32)))) as i8); let a_ = (p_p_3).wrapping_add(9); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((c.m.r8(((l_d_6).wrapping_add(64)).wrapping_add(((l_n_7) as u32)))) as i8); let a_ = (p_p_3).wrapping_add(10); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((c.m.r8(((l_d_6).wrapping_add(66)).wrapping_add(((l_n_7) as u32)))) as i8); let a_ = (p_p_3).wrapping_add(11); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((0i32) as i8); let a_ = (p_p_3).wrapping_add(12); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((c.m.r8(((l_d_6).wrapping_add(96)).wrapping_add(((l_n_7) as u32)))) as i8); let a_ = (p_p_3).wrapping_add(13); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((c.m.r8((((l_d_6).wrapping_add(72)).wrapping_add((((0i32) as u32)).wrapping_mul(2))).wrapping_add(((l_n_7) as u32)))) as i8); let a_ = ((p_p_3).wrapping_add(14)).wrapping_add(((0i32) as u32)); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((c.m.r8((((l_d_6).wrapping_add(72)).wrapping_add((((1i32) as u32)).wrapping_mul(2))).wrapping_add(((l_n_7) as u32)))) as i8); let a_ = ((p_p_3).wrapping_add(14)).wrapping_add(((1i32) as u32)); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((c.m.r8((((l_d_6).wrapping_add(72)).wrapping_add((((2i32) as u32)).wrapping_mul(2))).wrapping_add(((l_n_7) as u32)))) as i8); let a_ = ((p_p_3).wrapping_add(14)).wrapping_add(((2i32) as u32)); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((c.m.r8((((l_d_6).wrapping_add(72)).wrapping_add((((3i32) as u32)).wrapping_mul(2))).wrapping_add(((l_n_7) as u32)))) as i8); let a_ = ((p_p_3).wrapping_add(14)).wrapping_add(((3i32) as u32)); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((c.m.r8((((l_d_6).wrapping_add(72)).wrapping_add((((4i32) as u32)).wrapping_mul(2))).wrapping_add(((l_n_7) as u32)))) as i8); let a_ = ((p_p_3).wrapping_add(14)).wrapping_add(((4i32) as u32)); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((c.m.r8((((l_d_6).wrapping_add(72)).wrapping_add((((5i32) as u32)).wrapping_mul(2))).wrapping_add(((l_n_7) as u32)))) as i8); let a_ = ((p_p_3).wrapping_add(14)).wrapping_add(((5i32) as u32)); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((((c.m.r8((((l_d_6).wrapping_add(84)).wrapping_add((((0i32) as u32)).wrapping_mul(2))).wrapping_add(((l_n_7) as u32)))) as i8)) as i16); let a_ = ((p_p_3).wrapping_add(22)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((((c.m.r8((((l_d_6).wrapping_add(84)).wrapping_add((((1i32) as u32)).wrapping_mul(2))).wrapping_add(((l_n_7) as u32)))) as i8)) as i16); let a_ = ((p_p_3).wrapping_add(22)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((((c.m.r8((((l_d_6).wrapping_add(84)).wrapping_add((((2i32) as u32)).wrapping_mul(2))).wrapping_add(((l_n_7) as u32)))) as i8)) as i16); let a_ = ((p_p_3).wrapping_add(22)).wrapping_add((((2i32) as u32)).wrapping_mul(2)); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((((c.m.r8((((l_d_6).wrapping_add(84)).wrapping_add((((3i32) as u32)).wrapping_mul(2))).wrapping_add(((l_n_7) as u32)))) as i8)) as i16); let a_ = ((p_p_3).wrapping_add(22)).wrapping_add((((3i32) as u32)).wrapping_mul(2)); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((((c.m.r8((((l_d_6).wrapping_add(84)).wrapping_add((((4i32) as u32)).wrapping_mul(2))).wrapping_add(((l_n_7) as u32)))) as i8)) as i16); let a_ = ((p_p_3).wrapping_add(22)).wrapping_add((((4i32) as u32)).wrapping_mul(2)); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((((c.m.r8((((l_d_6).wrapping_add(84)).wrapping_add((((5i32) as u32)).wrapping_mul(2))).wrapping_add(((l_n_7) as u32)))) as i8)) as i16); let a_ = ((p_p_3).wrapping_add(22)).wrapping_add((((5i32) as u32)).wrapping_mul(2)); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((0i32) as i16); let a_ = (p_p_3).wrapping_add(34); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((0i32) as i16); let a_ = (p_p_3).wrapping_add(36); c.m.w16(a_, (v_) as u16); };
            { let v_ = rt::f2i(rt::fmul(F32(c.m.r32(((l_d_6).wrapping_add(24)).wrapping_add((((l_n_7) as u32)).wrapping_mul(4)))), F32(0x41f00000))); let a_ = (p_p_3).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            { let v_ = F32(0x00000000); let a_ = (p_p_3).wrapping_add(44); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32(((l_d_6).wrapping_add(48)).wrapping_add((((l_n_7) as u32)).wrapping_mul(4)))); let a_ = (p_p_3).wrapping_add(48); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32(((l_d_6).wrapping_add(32)).wrapping_add((((l_n_7) as u32)).wrapping_mul(4)))); let a_ = (p_p_3).wrapping_add(52); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32(((l_d_6).wrapping_add(40)).wrapping_add((((l_n_7) as u32)).wrapping_mul(4)))); let a_ = (p_p_3).wrapping_add(56); c.m.w32(a_, (v_).0); };
            { let v_ = (c.m.r32((l_d_6).wrapping_add((((l_n_7) as u32)).wrapping_mul(4))) as i32); let a_ = (p_p_3).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
            { let v_ = ((((c.m.r8(((l_d_6).wrapping_add(70)).wrapping_add(((l_n_7) as u32)))) as i8)) as i16); let a_ = (p_p_3).wrapping_add(64); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((((c.m.r8(((l_d_6).wrapping_add(68)).wrapping_add(((l_n_7) as u32)))) as i8)) as i16); let a_ = (p_p_3).wrapping_add(66); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((((c.m.r8(((l_d_6).wrapping_add(60)).wrapping_add(((l_n_7) as u32)))) as i8)) as i16); let a_ = (p_p_3).wrapping_add(68); c.m.w16(a_, (v_) as u16); };
            { let v_ = F32(0x00000000); let a_ = (p_p_3).wrapping_add(76); c.m.w32(a_, (v_).0); };
            { let v_ = ((((c.m.r8(((l_d_6).wrapping_add(110)).wrapping_add(((l_n_7) as u32)))) as i8)) as i16); let a_ = (p_p_3).wrapping_add(72); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((((c.m.r8(((l_d_6).wrapping_add(112)).wrapping_add(((l_n_7) as u32)))) as i8)) as i16); let a_ = (p_p_3).wrapping_add(74); c.m.w16(a_, (v_) as u16); };
            { let v_ = F32(0x3f800000); let a_ = (p_p_3).wrapping_add(80); c.m.w32(a_, (v_).0); };
            { let v_ = (((1i32).wrapping_neg()) as i8); let a_ = (p_p_3).wrapping_add(20); c.m.w8(a_, (v_) as u8); };
            { let v_ = F32(0x3f800000); let a_ = (p_p_3).wrapping_add(84); c.m.w32(a_, (v_).0); };
            {
                l_i_5 = 0i32;
                'l3: loop {
                    if !(((((l_i_5) < (8i32)) as i32)) != 0) { break 'l3; }
                    'c4: {
                        { let v_ = (((1i32).wrapping_neg()) as i8); let a_ = ((p_p_3).wrapping_add(88)).wrapping_add(((l_i_5) as u32)); c.m.w8(a_, (v_) as u8); };
                        { let v_ = F32(0x00000000); let a_ = ((p_p_3).wrapping_add(96)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    }
                    { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
                }
            }
            { let v_ = F32(0x3dcccccc); let a_ = (p_p_3).wrapping_add(132); c.m.w32(a_, (v_).0); };
            { let v_ = F32(0x3f800000); let a_ = (p_p_3).wrapping_add(136); c.m.w32(a_, (v_).0); };
            { let v_ = ((0i32) as i16); let a_ = (p_p_3).wrapping_add(128); c.m.w16(a_, (v_) as u16); };
        } else {
            l_d_8 = F::BtlCharApi_GetPlayerSuperData(c, p_chr_1);
            l_n_9 = (p_slot_2).wrapping_sub(2i32);
            { let v_ = ((c.m.r16(((l_d_8).wrapping_add(24)).wrapping_add((((l_n_9) as u32)).wrapping_mul(2)))) as i16); let a_ = p_p_3; c.m.w16(a_, (v_) as u16); };
            { let v_ = ((c.m.r16(((l_d_8).wrapping_add(30)).wrapping_add((((l_n_9) as u32)).wrapping_mul(2)))) as i16); let a_ = (p_p_3).wrapping_add(2); c.m.w16(a_, (v_) as u16); };
            { let v_ = (((if (((((p_slot_2) == (4i32)) as i32)) != 0) { 2i32 } else { 1i32 })) as i8); let a_ = (p_p_3).wrapping_add(4); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((c.m.r8(((l_d_8).wrapping_add(315)).wrapping_add(((l_n_9) as u32)))) as i8); let a_ = (p_p_3).wrapping_add(5); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((c.m.r8(((l_d_8).wrapping_add(144)).wrapping_add(((l_n_9) as u32)))) as i8); let a_ = (p_p_3).wrapping_add(6); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((0i32) as i8); let a_ = (p_p_3).wrapping_add(7); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((c.m.r8(((l_d_8).wrapping_add(153)).wrapping_add(((l_n_9) as u32)))) as i8); let a_ = (p_p_3).wrapping_add(8); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((c.m.r8(((l_d_8).wrapping_add(147)).wrapping_add(((l_n_9) as u32)))) as i8); let a_ = (p_p_3).wrapping_add(9); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((c.m.r8(((l_d_8).wrapping_add(156)).wrapping_add(((l_n_9) as u32)))) as i8); let a_ = (p_p_3).wrapping_add(10); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((c.m.r8(((l_d_8).wrapping_add(159)).wrapping_add(((l_n_9) as u32)))) as i8); let a_ = (p_p_3).wrapping_add(11); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((0i32) as i8); let a_ = (p_p_3).wrapping_add(12); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((c.m.r8(((l_d_8).wrapping_add(312)).wrapping_add(((l_n_9) as u32)))) as i8); let a_ = (p_p_3).wrapping_add(13); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((c.m.r8((((l_d_8).wrapping_add(168)).wrapping_add((((0i32) as u32)).wrapping_mul(3))).wrapping_add(((l_n_9) as u32)))) as i8); let a_ = ((p_p_3).wrapping_add(14)).wrapping_add(((0i32) as u32)); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((c.m.r8((((l_d_8).wrapping_add(168)).wrapping_add((((1i32) as u32)).wrapping_mul(3))).wrapping_add(((l_n_9) as u32)))) as i8); let a_ = ((p_p_3).wrapping_add(14)).wrapping_add(((1i32) as u32)); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((c.m.r8((((l_d_8).wrapping_add(168)).wrapping_add((((2i32) as u32)).wrapping_mul(3))).wrapping_add(((l_n_9) as u32)))) as i8); let a_ = ((p_p_3).wrapping_add(14)).wrapping_add(((2i32) as u32)); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((c.m.r8((((l_d_8).wrapping_add(168)).wrapping_add((((3i32) as u32)).wrapping_mul(3))).wrapping_add(((l_n_9) as u32)))) as i8); let a_ = ((p_p_3).wrapping_add(14)).wrapping_add(((3i32) as u32)); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((c.m.r8((((l_d_8).wrapping_add(168)).wrapping_add((((4i32) as u32)).wrapping_mul(3))).wrapping_add(((l_n_9) as u32)))) as i8); let a_ = ((p_p_3).wrapping_add(14)).wrapping_add(((4i32) as u32)); c.m.w8(a_, (v_) as u8); };
            { let v_ = ((c.m.r8((((l_d_8).wrapping_add(168)).wrapping_add((((5i32) as u32)).wrapping_mul(3))).wrapping_add(((l_n_9) as u32)))) as i8); let a_ = ((p_p_3).wrapping_add(14)).wrapping_add(((5i32) as u32)); c.m.w8(a_, (v_) as u8); };
            { let v_ = (((1i32).wrapping_neg()) as i16); let a_ = ((p_p_3).wrapping_add(22)).wrapping_add((((0i32) as u32)).wrapping_mul(2)); c.m.w16(a_, (v_) as u16); };
            { let v_ = (((1i32).wrapping_neg()) as i16); let a_ = ((p_p_3).wrapping_add(22)).wrapping_add((((5i32) as u32)).wrapping_mul(2)); c.m.w16(a_, (v_) as u16); };
            { let v_ = (((1i32).wrapping_neg()) as i16); let a_ = ((p_p_3).wrapping_add(22)).wrapping_add((((1i32) as u32)).wrapping_mul(2)); c.m.w16(a_, (v_) as u16); };
            { let v_ = (((1i32).wrapping_neg()) as i16); let a_ = ((p_p_3).wrapping_add(22)).wrapping_add((((2i32) as u32)).wrapping_mul(2)); c.m.w16(a_, (v_) as u16); };
            { let v_ = (((1i32).wrapping_neg()) as i16); let a_ = ((p_p_3).wrapping_add(22)).wrapping_add((((3i32) as u32)).wrapping_mul(2)); c.m.w16(a_, (v_) as u16); };
            { let v_ = (((1i32).wrapping_neg()) as i16); let a_ = ((p_p_3).wrapping_add(22)).wrapping_add((((4i32) as u32)).wrapping_mul(2)); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((0i32) as i16); let a_ = (p_p_3).wrapping_add(34); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((0i32) as i16); let a_ = (p_p_3).wrapping_add(36); c.m.w16(a_, (v_) as u16); };
            { let v_ = rt::f2i(rt::fmul(F32(c.m.r32(((l_d_8).wrapping_add(36)).wrapping_add((((l_n_9) as u32)).wrapping_mul(4)))), F32(0x41f00000))); let a_ = (p_p_3).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
            { let v_ = F32(c.m.r32(((l_d_8).wrapping_add(48)).wrapping_add((((l_n_9) as u32)).wrapping_mul(4)))); let a_ = (p_p_3).wrapping_add(44); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32(((l_d_8).wrapping_add(108)).wrapping_add((((l_n_9) as u32)).wrapping_mul(4)))); let a_ = (p_p_3).wrapping_add(48); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32(((l_d_8).wrapping_add(84)).wrapping_add((((l_n_9) as u32)).wrapping_mul(4)))); let a_ = (p_p_3).wrapping_add(52); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32(((l_d_8).wrapping_add(96)).wrapping_add((((l_n_9) as u32)).wrapping_mul(4)))); let a_ = (p_p_3).wrapping_add(56); c.m.w32(a_, (v_).0); };
            { let v_ = (c.m.r32((l_d_8).wrapping_add((((l_n_9) as u32)).wrapping_mul(4))) as i32); let a_ = (p_p_3).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
            { let v_ = ((((c.m.r8(((l_d_8).wrapping_add(165)).wrapping_add(((l_n_9) as u32)))) as i8)) as i16); let a_ = (p_p_3).wrapping_add(64); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((((c.m.r8(((l_d_8).wrapping_add(162)).wrapping_add(((l_n_9) as u32)))) as i8)) as i16); let a_ = (p_p_3).wrapping_add(66); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((((c.m.r8(((l_d_8).wrapping_add(150)).wrapping_add(((l_n_9) as u32)))) as i8)) as i16); let a_ = (p_p_3).wrapping_add(68); c.m.w16(a_, (v_) as u16); };
            { let v_ = F32(c.m.r32(((l_d_8).wrapping_add(532)).wrapping_add((((l_n_9) as u32)).wrapping_mul(4)))); let a_ = (p_p_3).wrapping_add(76); c.m.w32(a_, (v_).0); };
            { let v_ = ((((c.m.r8(((l_d_8).wrapping_add(348)).wrapping_add(((l_n_9) as u32)))) as i8)) as i16); let a_ = (p_p_3).wrapping_add(72); c.m.w16(a_, (v_) as u16); };
            { let v_ = ((((c.m.r8(((l_d_8).wrapping_add(351)).wrapping_add(((l_n_9) as u32)))) as i8)) as i16); let a_ = (p_p_3).wrapping_add(74); c.m.w16(a_, (v_) as u16); };
            { let v_ = F32(c.m.r32(((l_d_8).wrapping_add(120)).wrapping_add((((l_n_9) as u32)).wrapping_mul(4)))); let a_ = (p_p_3).wrapping_add(84); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32(((l_d_8).wrapping_add(132)).wrapping_add((((l_n_9) as u32)).wrapping_mul(4)))); let a_ = (p_p_3).wrapping_add(80); c.m.w32(a_, (v_).0); };
            { let v_ = ((c.m.r8(((l_d_8).wrapping_add(354)).wrapping_add(((l_n_9) as u32)))) as i8); let a_ = (p_p_3).wrapping_add(20); c.m.w8(a_, (v_) as u8); };
            {
                l_i_5 = 0i32;
                'l5: loop {
                    if !(((((l_i_5) < (8i32)) as i32)) != 0) { break 'l5; }
                    'c6: {
                        { let v_ = ((c.m.r8((((l_d_8).wrapping_add(186)).wrapping_add((((l_i_5) as u32)).wrapping_mul(3))).wrapping_add(((l_n_9) as u32)))) as i8); let a_ = ((p_p_3).wrapping_add(88)).wrapping_add(((l_i_5) as u32)); c.m.w8(a_, (v_) as u8); };
                        { let v_ = F32(c.m.r32((((l_d_8).wrapping_add(212)).wrapping_add((((l_i_5) as u32)).wrapping_mul(12))).wrapping_add((((l_n_9) as u32)).wrapping_mul(4)))); let a_ = ((p_p_3).wrapping_add(96)).wrapping_add((((l_i_5) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
                    }
                    { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
                }
            }
            { let v_ = ((((c.m.r8(((l_d_8).wrapping_add(308)).wrapping_add(((l_n_9) as u32)))) as i8)) as i16); let a_ = (p_p_3).wrapping_add(128); c.m.w16(a_, (v_) as u16); };
            { let v_ = F32(c.m.r32(((l_d_8).wrapping_add(72)).wrapping_add((((l_n_9) as u32)).wrapping_mul(4)))); let a_ = (p_p_3).wrapping_add(132); c.m.w32(a_, (v_).0); };
            { let v_ = F32(c.m.r32(((l_d_8).wrapping_add(60)).wrapping_add((((l_n_9) as u32)).wrapping_mul(4)))); let a_ = (p_p_3).wrapping_add(136); c.m.w32(a_, (v_).0); };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShot_BuildParam", &PROBE_N, &[], true); }
    r_
}

pub fn EftShot_GetSlot(c: &mut Ctx, mut p_chr_1: i32, mut p_slot_2: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShot_GetSlot", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let mut l_c_3: u32 = 0;
    let r_ = 'f: {
    l_c_3 = ({ let a_ = (c.m.r32(sym::gEftShot)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((p_chr_1) as u32)).wrapping_mul(1344));
    break 'f (l_c_3).wrapping_add((((p_slot_2) as u32)).wrapping_mul(80));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShot_GetSlot", &PROBE_N, &[], true); }
    r_
}

pub fn EftShot_CreateSlotTask(c: &mut Ctx, mut p_chr_1: i32, mut p_slot_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShot_CreateSlotTask", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let mut l_c_3: u32 = 0;
    let mut l_s_4: u32 = 0;
    let mut l_cls_5: u32 = 0;
    let r_ = 'f: {
    l_c_3 = ({ let a_ = (c.m.r32(sym::gEftShot)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((p_chr_1) as u32)).wrapping_mul(1344));
    l_s_4 = (l_c_3).wrapping_add((((p_slot_2) as u32)).wrapping_mul(80));
    l_cls_5 = { let a_ = (sym::gEftShotClass).wrapping_add(((({ let a0_ = (({ let a_ = (c.m.r32((l_s_4).wrapping_add(36))).wrapping_add(13); (c.m.r8(a_) as i8) }) as i32); F::eft_h__EftShot_TypeRow(c, a0_) }) as u32)).wrapping_mul(12)); c.m.r32(a_) };
    if (((((l_cls_5) != (((0i32) as u32))) as i32)) != 0) {
        { let v_ = p_chr_1; let a_ = (l_s_4).wrapping_add(32); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a0_ = c.m.r32((l_c_3).wrapping_add(1320)); F::BtlTaskList_AddTail(c, a0_, l_cls_5, l_s_4) }; let a_ = (l_s_4).wrapping_add(40); c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShot_CreateSlotTask", &PROBE_N, &[], true); }
    r_
}

pub fn EftShot_Start(c: &mut Ctx, mut p_arg_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShot_Start", &PROBE_N, &[(p_arg_1) as u32], false); }
    let mut l_s_2: u32 = 0;
    let mut l_cls_3: u32 = 0;
    let mut l_group_4: u32 = 0;
    let r_ = 'f: {
    l_s_2 = { let a0_ = (c.m.r32(p_arg_1) as i32); let a1_ = (c.m.r32((p_arg_1).wrapping_add(4)) as i32); F::EftShot_GetSlot(c, a0_, a1_) };
    if (((((l_s_2) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    if (((((c.m.r32((l_s_2).wrapping_add(28))) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    l_cls_3 = { let a_ = ((sym::gEftShotClass).wrapping_add(((({ let a0_ = (({ let a_ = (c.m.r32((l_s_2).wrapping_add(36))).wrapping_add(13); (c.m.r8(a_) as i8) }) as i32); F::eft_h__EftShot_TypeRow(c, a0_) }) as u32)).wrapping_mul(12))).wrapping_add(4); c.m.r32(a_) };
    if (((((l_cls_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    l_group_4 = c.m.r32((l_s_2).wrapping_add(40));
    { let s_ = p_arg_1; let d_ = l_s_2; rt::memcpy(c, d_, s_, 28); };
    { let v_ = 0i32; let a_ = (l_s_2).wrapping_add(64); c.m.w32(a_, (v_) as u32); };
    { let v_ = rt::f2i(rt::fmul(F32(c.m.r32((p_arg_1).wrapping_add(16))), F32(0x41f00000))); let a_ = (c.m.r32((l_s_2).wrapping_add(36))).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
    { let v_ = F32(c.m.r32((p_arg_1).wrapping_add(20))); let a_ = (c.m.r32((l_s_2).wrapping_add(36))).wrapping_add(52); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((p_arg_1).wrapping_add(24))); let a_ = (c.m.r32((l_s_2).wrapping_add(36))).wrapping_add(56); c.m.w32(a_, (v_).0); };
    if ((((((({ let a_ = (c.m.r32((l_s_2).wrapping_add(36))).wrapping_add(4); (c.m.r8(a_) as i8) }) as i32)) == (0i32)) as i32)) != 0) {
        { let v_ = 60i32; let a_ = (c.m.r32((l_s_2).wrapping_add(36))).wrapping_add(40); c.m.w32(a_, (v_) as u32); };
    }
    { let a0_ = (c.m.r32(p_arg_1) as i32); let a1_ = (c.m.r32((p_arg_1).wrapping_add(4)) as i32); F::EftShot_SetCurSlot(c, a0_, a1_) };
    { let a1_ = rt::f2i(F::EftShot_GetLeadTime(c, l_s_2)); let a2_ = { let a_ = (c.m.r32((l_s_2).wrapping_add(36))).wrapping_add(40); (c.m.r32(a_) as i32) }; { let _ = F::EftTechEvt_Start(c, l_s_2, a1_, a2_); } };
    { let _ = { let a0_ = c.m.r32((l_group_4).wrapping_add(36)); F::BtlTaskList_AddTail(c, a0_, l_cls_3, l_s_2) }; };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShot_Start", &PROBE_N, &[], true); }
    r_
}

pub fn EftShot_DestroyChar(c: &mut Ctx, mut p_chr_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShot_DestroyChar", &PROBE_N, &[(p_chr_1) as u32], false); }
    let mut l_c_2: u32 = 0;
    let r_ = 'f: {
    l_c_2 = ({ let a_ = (c.m.r32(sym::gEftShot)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((p_chr_1) as u32)).wrapping_mul(1344));
    if (((((c.m.r32((l_c_2).wrapping_add(1324))) != (((0i32) as u32))) as i32)) != 0) {
        F::BtlScene_Reset(c, (if (((((p_chr_1) == (0i32)) as i32)) != 0) { 2i32 } else { 3i32 }));
        F::EftShot_SetCharPool(c, p_chr_1);
        { let a0_ = c.m.r32((l_c_2).wrapping_add(1324)); F::BtlTask_Kill(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (l_c_2).wrapping_add(1324); c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShot_DestroyChar", &PROBE_N, &[], true); }
    r_
}

pub fn EftShot_CreateChar(c: &mut Ctx, mut p_chr_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShot_CreateChar", &PROBE_N, &[(p_chr_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_c_2: u32 = 0;
    let r_ = 'f: {
        l_c_2 = ({ let a_ = (c.m.r32(sym::gEftShot)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((p_chr_1) as u32)).wrapping_mul(1344));
        if (((((c.m.r32((l_c_2).wrapping_add(1324))) == (((0i32) as u32))) as i32)) != 0) {
            { let v_ = p_chr_1; let a_ = fp; c.m.w32(a_, (v_) as u32); };
            { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gEftShot); c.m.r32(a_) }; F::BtlTaskList_AddTail(c, a0_, sym::gEftShotCharClass, fp) }; let a_ = (l_c_2).wrapping_add(1324); c.m.w32(a_, v_); };
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftShot_CreateChar", &PROBE_N, &[], true); }
    r_
}

pub fn EftShot_GetLeadTime(c: &mut Ctx, mut p_slot_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShot_GetLeadTime", &PROBE_N, &[(p_slot_1) as u32], false); }
    let mut l_t_2: F32 = F32(0);
    let mut l_p_3: u32 = 0;
    let r_ = 'f: {
    l_t_2 = F32(0x41cbffff);
    l_p_3 = c.m.r32((p_slot_1).wrapping_add(36));
    if (((((l_p_3) == (((0i32) as u32))) as i32)) != 0) {
        break 'f l_t_2;
    }
    's1: {
        let sv4: i32 = (((c.m.r8((l_p_3).wrapping_add(4)) as i8)) as i32);
        match sv4 {
            0i32 => {
                break 's1;
            }
            1i32 => {
                if ((((((((c.m.r8((l_p_3).wrapping_add(5)) as i8)) as i32)) == (5i32)) as i32)) != 0) {
                    l_t_2 = F32(0x41bfffff);
                }
                break 's1;
            }
            2i32 => {
                l_t_2 = F32(0x41bfffff);
                if ((((((((c.m.r8((l_p_3).wrapping_add(5)) as i8)) as i32)) != (5i32)) as i32)) != 0) {
                    l_t_2 = F32(0x421bffff);
                }
                if ((((((((c.m.r16(l_p_3) as i16)) as i32)) == (616i32)) as i32)) != 0) {
                    l_t_2 = F32(0x41a7ffff);
                }
                if ((((((((c.m.r16(l_p_3) as i16)) as i32)) == (656i32)) as i32)) != 0) {
                    l_t_2 = F32(0x00000000);
                }
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f l_t_2;
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShot_GetLeadTime", &PROBE_N, &[], true); }
    r_
}

pub fn EftShot_GetCharPack(c: &mut Ctx, mut p_chr_1: i32, mut p_slot_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShot_GetCharPack", &PROBE_N, &[(p_chr_1) as u32, (p_slot_2) as u32], false); }
    let r_ = 'f: {
    break 'f F::BtlCharApi_GetPlayerObjUnk9C(c, p_chr_1, ((p_slot_2) as u32));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShot_GetCharPack", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotNullGroup_Init(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotNullGroup_Init", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let _ = F::BtlTask_CreateChildList(c, p_task_1, ((((2i32) as i64 as u64)) as u32), ((((((8u32) as i32)) as i64 as u64)) as u32)); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotNullGroup_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotNullGroup_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotNullGroup_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotNullGroup_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotNullGroup_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotNullGroup_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotNullGroup_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotNull_Init(c: &mut Ctx, mut p_task_1: u32, mut p_slot_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotNull_Init", &PROBE_N, &[(p_task_1) as u32, (p_slot_2) as u32], false); }
    let mut l_w_3: u32 = 0;
    let r_ = 'f: {
    l_w_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let _ = F::memset(c, l_w_3, 0i32, 8u32); };
    { let v_ = p_slot_2; let a_ = (l_w_3).wrapping_add(4); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotNull_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotNull_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotNull_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotNull_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotNull_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotNull_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotNull_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotNull_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotNull_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_slot_2: u32 = 0;
    let r_ = 'f: {
    l_slot_2 = { let a_ = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(4); c.m.r32(a_) };
    if (((((l_slot_2) == (((0i32) as u32))) as i32)) != 0) {
        F::BtlTask_SetDead(c, p_task_1);
        break 'f;
    }
    if (((((c.m.r32((l_slot_2).wrapping_add(36))) == (((0i32) as u32))) as i32)) != 0) {
        F::BtlTask_SetDead(c, p_task_1);
        break 'f;
    }
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotNull_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotNull_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotNull_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotNull_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftShotNull_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftShotNull_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftShotNull_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftVolley_AllocShot(c: &mut Ctx, mut p_w_1: u32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVolley_AllocShot", &PROBE_N, &[(p_w_1) as u32], false); }
    let mut l_shot_2: u32 = 0;
    let mut l_found_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_shot_2 = (p_w_1).wrapping_add(1472);
    l_found_3 = ((0i32) as u32);
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (30i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((c.m.r32((l_shot_2).wrapping_add(72))) == (((0i32) as u32))) as i32)) != 0) {
                    l_found_3 = l_shot_2;
                    { let _ = F::memset(c, l_found_3, 0i32, 80u32); };
                }
            }
            { { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; }; { let o_ = l_shot_2; let n_ = (o_).wrapping_add(80); l_shot_2 = n_; }; };
        }
    }
    break 'f l_found_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftVolley_AllocShot", &PROBE_N, &[], true); }
    r_
}

pub fn EftVolley_Fire(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32, mut p_phase_3: i32, mut p_sub_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVolley_Fire", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32, (p_phase_3) as u32, (p_sub_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_w_5: u32 = 0;
    let mut l_slot_6: u32 = 0;
    let mut l_n_7: i32 = 0;
    let mut l_i_8: i32 = 0;
    let mut l_shot_9: u32 = 0;
    let mut l_count_10: i32 = 0;
    let mut l_node_11: i32 = 0;
    let mut l_sp_12: u32 = 0;
    let r_ = 'f: {
        l_w_5 = c.m.r32((p_task_2).wrapping_add(56));
        l_slot_6 = c.m.r32((l_w_5).wrapping_add(144));
        l_n_7 = 1i32;
        if (((((p_phase_3) == (1i32)) as i32)) != 0) {
            l_n_7 = (({ let a_ = (c.m.r32((l_slot_6).wrapping_add(36))).wrapping_add(68); (c.m.r16(a_) as i16) }) as i32);
            if (((((l_n_7) <= (0i32)) as i32)) != 0) {
                l_n_7 = 1i32;
            }
        }
        {
            l_i_8 = 0i32;
            'l1: loop {
                if !(((((l_i_8) < (l_n_7)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_shot_9 = F::EftVolley_AllocShot(c, l_w_5);
                    if (((((l_shot_9) != (((0i32) as u32))) as i32)) != 0) {
                        l_count_10 = (({ let a_ = (c.m.r32((l_slot_6).wrapping_add(36))).wrapping_add(9); (c.m.r8(a_) as i8) }) as i32);
                        rt::memset(c, fp, 0, 48);
                        { let v_ = l_slot_6; c.m.w32(fp.wrapping_add(0), v_); }
                        { let v_ = c.m.r32((l_w_5).wrapping_add(1456)); c.m.w32(fp.wrapping_add(4), v_); }
                        { let v_ = (l_w_5).wrapping_add(864); c.m.w32(fp.wrapping_add(8), v_); }
                        { let v_ = l_shot_9; c.m.w32(fp.wrapping_add(12), v_); }
                        { let v_ = (l_shot_9).wrapping_add(16); c.m.w32(fp.wrapping_add(16), v_); }
                        { let v_ = (c.m.r32((l_w_5).wrapping_add(5056)) as i32); c.m.w32(fp.wrapping_add(20), (v_) as u32); }
                        { let v_ = l_count_10; c.m.w32(fp.wrapping_add(24), (v_) as u32); }
                        { let v_ = p_phase_3; c.m.w32(fp.wrapping_add(28), (v_) as u32); }
                        { let v_ = p_sub_4; c.m.w32(fp.wrapping_add(32), (v_) as u32); }
                        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(36), (v_).0); }
                        { let v_ = F32(c.m.r32((l_w_5).wrapping_add(16))); c.m.w32(fp.wrapping_add(40), (v_).0); }
                        { let v_ = F32(c.m.r32((l_w_5).wrapping_add(4))); c.m.w32(fp.wrapping_add(44), (v_).0); }
                        if (((((c.m.r32(l_w_5) as i32)) & (512i32))) != 0) {
                            { let r_ = 2i32; let a_ = (l_shot_9).wrapping_add(64); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            l_node_11 = (if (((((l_i_8) == (0i32)) as i32)) != 0) { 31i32 } else { 45i32 });
                            l_sp_12 = ((l_w_5).wrapping_add(3872)).wrapping_add((((l_i_8) as u32)).wrapping_mul(592));
                            { let s_ = (l_w_5).wrapping_add(864); let d_ = l_sp_12; rt::memcpy(c, d_, s_, 592); };
                            { let v_ = l_node_11; let a_ = (((l_w_5).wrapping_add(3872)).wrapping_add((((l_i_8) as u32)).wrapping_mul(592))).wrapping_add(80); c.m.w32(a_, (v_) as u32); };
                            F::BtlCharApi_GetNodePos(c, p_objId_1, l_node_11, (((l_w_5).wrapping_add(3872)).wrapping_add((((l_i_8) as u32)).wrapping_mul(592))).wrapping_add(96));
                            { let v_ = ((l_w_5).wrapping_add(3872)).wrapping_add((((l_i_8) as u32)).wrapping_mul(592)); let a_ = (fp).wrapping_add(8); c.m.w32(a_, v_); };
                        } else {
                            if (((((F::BtlCharApi_ObjQuery24D610(c, p_objId_1, 1024i32, 4i32)) <= (0i32)) as i32)) != 0) {
                                { let r_ = 2i32; let a_ = (l_shot_9).wrapping_add(64); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                            }
                            l_node_11 = (c.m.r32(((l_w_5).wrapping_add(864)).wrapping_add(80)) as i32);
                        }
                        if (((((c.m.r32(l_w_5) as i32)) & (4096i32))) != 0) {
                            F::BtlCharApi_GetNodePos(c, p_objId_1, 54i32, l_shot_9);
                            F::Vec4_Copy(c, (l_shot_9).wrapping_add(16), (l_w_5).wrapping_add(48));
                            { let a5_ = (c.m.r32((l_w_5).wrapping_add(5056)) as i32); { let _ = F::EftVolleyAim_Spread(c, p_objId_1, (l_shot_9).wrapping_add(16), 11i32, 1i32, 0i32, a5_, l_count_10); } };
                        } else {
                            F::BtlCharApi_GetNodePos(c, p_objId_1, l_node_11, l_shot_9);
                            F::Vec4_Copy(c, (l_shot_9).wrapping_add(16), (l_w_5).wrapping_add(48));
                        }
                        if (((((p_phase_3) == (1i32)) as i32)) != 0) {
                            { let v_ = rt::fdiv(rt::i2f({ let a_ = (c.m.r32((l_slot_6).wrapping_add(36))).wrapping_add(40); (c.m.r32(a_) as i32) }), F32(0x41f00000)); let a_ = (fp).wrapping_add(36); c.m.w32(a_, (v_).0); };
                            if ((((!(((((c.m.r32(l_w_5) as i32)) & (512i32))) != 0)) as i32)) != 0) {
                                if ((((!(((((c.m.r32((l_shot_9).wrapping_add(64)) as i32)) & (4i32))) != 0)) as i32)) != 0) {
                                    { let v_ = { let a0_ = (c.m.r32(((l_w_5).wrapping_add(864)).wrapping_add(80)) as i32); F::EftVolleyAim_GetNodeSide(c, a0_) }; let a_ = (l_w_5).wrapping_add(5064); c.m.w32(a_, (v_) as u32); };
                                }
                                { let r_ = 4i32; let a_ = (l_shot_9).wrapping_add(64); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                { let a2_ = { let a0_ = c.m.r32((l_w_5).wrapping_add(1456)); F::EftEmit_GetAimKind(c, a0_) }; let a3_ = (c.m.r32((l_w_5).wrapping_add(5064)) as i32); let a4_ = (c.m.r32((l_w_5).wrapping_add(5056)) as i32); let a6_ = F32(c.m.r32((l_w_5).wrapping_add(4))); let a7_ = F32(c.m.r32((l_w_5).wrapping_add(8))); F::EftVolleyAim_InitShot(c, p_objId_1, l_shot_9, a2_, a3_, a4_, l_count_10, a6_, a7_) };
                            }
                        }
                        if (((((c.m.r32(l_w_5) as i32)) & (1024i32))) != 0) {
                            { let v_ = { let a1_ = { let a_ = ({ let a_ = (F::BtlTask_GetParent(c, p_task_2)).wrapping_add(56); c.m.r32(a_) }).wrapping_add(800); c.m.r32(a_) }; F::EftBlastObj_CreateWithModel(c, fp, a1_) }; let a_ = (l_shot_9).wrapping_add(72); c.m.w32(a_, v_); };
                        } else {
                            { let v_ = F::EftBlastObj_Create(c, fp); let a_ = (l_shot_9).wrapping_add(72); c.m.w32(a_, v_); };
                        }
                        { let r_ = 1i32; let a_ = (l_shot_9).wrapping_add(64); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        if (((((c.m.r32(l_w_5) as i32)) & (2048i32))) != 0) {
                            { let a0_ = c.m.r32((l_shot_9).wrapping_add(72)); { let _ = F::EftBlastObj_SetSel(c, a0_, 1i32, 5i32, 4i32); } };
                        }
                        if (((((c.m.r32(l_w_5) as i32)) & (4096i32))) != 0) {
                            { let a0_ = c.m.r32((l_shot_9).wrapping_add(72)); { let _ = F::EftBlastObj_SetModelAnim(c, a0_, 1i32); } };
                            if ((((!((((((c.m.r32((l_w_5).wrapping_add(5056)) as i32)) < ((l_count_10).wrapping_sub(1i32))) as i32)) != 0)) as i32)) != 0) {
                                { let a0_ = c.m.r32((l_shot_9).wrapping_add(72)); { let _ = F::EftBlastObj_MarkLast(c, a0_); } };
                            }
                        }
                        if (((((p_phase_3) == (1i32)) as i32)) != 0) {
                            if (((((c.m.r32((l_shot_9).wrapping_add(64)) as i32)) & (2i32))) != 0) {
                                { let a0_ = c.m.r32((l_shot_9).wrapping_add(72)); { let _ = F::EftBlastObj_MarkLast(c, a0_); } };
                            }
                        } else {
                            { let a0_ = c.m.r32((l_shot_9).wrapping_add(72)); { let _ = F::EftBlastObj_SetNoHit(c, a0_, 1i32); } };
                            { let a0_ = c.m.r32((l_shot_9).wrapping_add(72)); { let _ = F::EftBlastObj_SetFrozen(c, a0_, 1i32); } };
                        }
                        { let r_ = 256i32; let a_ = l_w_5; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        { let a_ = (l_w_5).wrapping_add(5056); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        if (((((p_phase_3) == (1i32)) as i32)) != 0) {
                            { let a_ = (l_w_5).wrapping_add(5060); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        }
                    }
                }
                { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftVolley_Fire", &PROBE_N, &[], true); }
    r_
}

pub fn EftVolley_PruneShots(c: &mut Ctx, mut p_task_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVolley_PruneShots", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_alive_2: i32 = 0;
    let mut l_shot_3: u32 = 0;
    let mut l_i_4: i32 = 0;
    let r_ = 'f: {
    l_alive_2 = 0i32;
    l_shot_3 = (c.m.r32((p_task_1).wrapping_add(56))).wrapping_add(1472);
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (30i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (({ let a0_ = c.m.r32((l_shot_3).wrapping_add(72)); F::EftBlastObj_IsAlive(c, a0_) }) != 0) {
                    l_alive_2 = 1i32;
                } else {
                    { let v_ = ((0i32) as u32); let a_ = (l_shot_3).wrapping_add(72); c.m.w32(a_, v_); };
                }
            }
            { { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; }; { let o_ = l_shot_3; let n_ = (o_).wrapping_add(80); l_shot_3 = n_; }; };
        }
    }
    break 'f l_alive_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftVolley_PruneShots", &PROBE_N, &[], true); }
    r_
}

pub fn EftVolley_UpdateShots(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVolley_UpdateShots", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32], false); }
    let mut l_phase_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_w_5: u32 = 0;
    let mut l_shot_6: u32 = 0;
    let mut l_flag2_7: i32 = 0;
    let r_ = 'f: {
    l_phase_3 = (1i32).wrapping_neg();
    l_i_4 = 0i32;
    l_w_5 = c.m.r32((p_task_2).wrapping_add(56));
    if ((F::EftShot_TestBits(c, p_objId_1, 2i32)) != 0) {
        l_phase_3 = 0i32;
    } else {
        if ((F::EftShot_TestBits(c, p_objId_1, 4i32)) != 0) {
            l_phase_3 = 1i32;
            l_i_4 = 1i32;
            F::EftTechEvt_RequestRestart(c, p_objId_1);
        } else {
            if ((F::EftShot_TestBits(c, p_objId_1, 16i32)) != 0) {
                l_phase_3 = 3i32;
                l_i_4 = 2i32;
            } else {
                if ((F::EftShot_TestBits(c, p_objId_1, 32i32)) != 0) {
                    l_phase_3 = 4i32;
                    l_i_4 = 3i32;
                } else {
                    if ((F::EftShot_TestBits(c, p_objId_1, 64i32)) != 0) {
                        l_phase_3 = 5i32;
                        l_i_4 = 4i32;
                    }
                }
            }
        }
    }
    if (((((((((l_phase_3) >= (0i32)) as i32)) != 0) && ((((({ let a0_ = c.m.r32((l_w_5).wrapping_add(1456)); F::EftEmit_GetPhaseMask(c, a0_) }).wrapping_shr((l_phase_3) as u32)) & (1i32))) != 0)) as i32)) != 0) {
        F::EftVolley_Fire(c, p_objId_1, p_task_2, l_phase_3, l_i_4);
    }
    {
        l_i_4 = 0i32;
        'l1: loop {
            if !(((((l_i_4) < (30i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_shot_6 = ((l_w_5).wrapping_add(1472)).wrapping_add((((l_i_4) as u32)).wrapping_mul(80));
                if (({ let a0_ = c.m.r32((l_shot_6).wrapping_add(72)); F::EftBlastObj_IsAlive(c, a0_) }) != 0) {
                    l_flag2_7 = 0i32;
                    if (((((c.m.r32((l_shot_6).wrapping_add(64)) as i32)) & (2i32))) != 0) {
                        l_flag2_7 = 1i32;
                    }
                    if (((((c.m.r32((l_shot_6).wrapping_add(64)) as i32)) & (4i32))) != 0) {
                        { let a4_ = F32(c.m.r32((l_shot_6).wrapping_add(68))); let a5_ = F32(c.m.r32((l_w_5).wrapping_add(4))); let a6_ = F32(c.m.r32((l_w_5).wrapping_add(8))); F::EftVolleyAim_Update(c, p_objId_1, l_shot_6, (l_shot_6).wrapping_add(48), l_flag2_7, a4_, a5_, a6_) };
                        { let a0_ = c.m.r32((l_shot_6).wrapping_add(72)); { let _ = F::EftBlastObj_SetTarget(c, a0_, 1i32, l_shot_6); } };
                    }
                    if (((((c.m.r32(l_w_5) as i32)) & (1i32))) != 0) {
                        { let a0_ = c.m.r32((l_shot_6).wrapping_add(72)); { let _ = F::EftBlastObj_Stop(c, a0_); } };
                    }
                    { let r_ = F32(0x3f800000); let a_ = (l_shot_6).wrapping_add(68); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
                }
            }
            { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
        }
    }
    break 'f F::EftVolley_PruneShots(c, p_task_2);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftVolley_UpdateShots", &PROBE_N, &[], true); }
    r_
}

pub fn EftVolley_Aim(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32, mut p_target_3: u32, mut p_bySlot_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVolley_Aim", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32, (p_target_3) as u32, (p_bySlot_4) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_w_5: u32 = 0;
    let mut l_slot_6: u32 = 0;
    let mut l_kind_7: i32 = 0;
    let mut l_pitch_8: F32 = F32(0);
    let mut l_yaw_9: F32 = F32(0);
    let r_ = 'f: {
        l_w_5 = c.m.r32((p_task_2).wrapping_add(56));
        l_slot_6 = c.m.r32((l_w_5).wrapping_add(144));
        l_kind_7 = { let a0_ = c.m.r32((l_w_5).wrapping_add(1456)); F::EftEmit_GetAimKind(c, a0_) };
        if ((p_bySlot_4) != 0) {
            { let _ = F::EftAim_GetDirKeep(c, l_slot_6, (l_w_5).wrapping_add(48), ((((p_target_3) as u64)) as i32), p_objId_1); };
        } else {
            { let _ = F::EftAim_GetDir(c, (l_w_5).wrapping_add(48), ((((p_target_3) as u64)) as i32), p_objId_1); };
        }
        F::Vec4_Copy(c, (l_w_5).wrapping_add(64), (l_w_5).wrapping_add(48));
        's1: {
            let sv10: i32 = l_kind_7;
            match sv10 {
                7i32 => {
                    F::Vec4_Set(c, (l_w_5).wrapping_add(64), F32(0x00000000), F32(0xbf800000), F32(0x00000000), F32(0x3f800000));
                    break 's1;
                }
                8i32 => {
                    {
                        { let s_ = sym::eft_h__sAimForward; rt::memcpy(c, fp.wrapping_add(64), s_, 16); }
                        F::Vec3_Clamp(c, (l_w_5).wrapping_add(64), (l_w_5).wrapping_add(64), F32(0xbf800000), F32(0x3f800000));
                        l_pitch_8 = { let a0_ = rt::fneg(F32(c.m.r32(((l_w_5).wrapping_add(64)).wrapping_add(4)))); F::Mathf_Asin(c, a0_) };
                        l_yaw_9 = { let a0_ = F32(c.m.r32((l_w_5).wrapping_add(64))); let a1_ = F32(c.m.r32(((l_w_5).wrapping_add(64)).wrapping_add(8))); F::Port_atan2f(c, a0_, a1_) };
                        l_pitch_8 = F::EftMath_WrapAngle(c, rt::fadd(l_pitch_8, F32(0x3f490fda)));
                        l_yaw_9 = F::EftMath_WrapAngle(c, l_yaw_9);
                        F::Mtx_StoreIdentity(c, fp);
                        F::Mtx_RotateX(c, fp, fp, l_pitch_8);
                        F::Mtx_RotateY(c, fp, fp, l_yaw_9);
                        F::Mtx_MulVec4(c, (l_w_5).wrapping_add(64), fp, fp.wrapping_add(64));
                        { let v_ = F32(0x3f800000); let a_ = ((l_w_5).wrapping_add(64)).wrapping_add(12); c.m.w32(a_, (v_).0); };
                        F::Vec3_Normalize(c, (l_w_5).wrapping_add(64), (l_w_5).wrapping_add(64));
                        break 's1;
                    }
                }
                _ => {
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftVolley_Aim", &PROBE_N, &[], true); }
    r_
}

pub fn EftVolley_UpdateParts(c: &mut Ctx, mut p_objId_1: i32, mut p_task_2: u32, mut p_set_3: u32, mut p_reset_4: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVolley_UpdateParts", &PROBE_N, &[(p_objId_1) as u32, (p_task_2) as u32, (p_set_3) as u32, (p_reset_4) as u32], false); }
    let mut l_w_5: u32 = 0;
    let mut l_type_6: i32 = 0;
    let mut l_i_7: i32 = 0;
    let mut l_def_8: u32 = 0;
    let mut l_flags_9: i32 = 0;
    let mut l_wf_10: i32 = 0;
    let r_ = 'f: {
    l_w_5 = c.m.r32((p_task_2).wrapping_add(56));
    {
        l_type_6 = 0i32;
        'l1: loop {
            if !(((((l_type_6) < (19i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((({ let a_ = c.m.r32(p_set_3); c.m.r32(a_) }) & ((1u32).wrapping_shl((l_type_6) as u32)))) != 0) {
                    l_def_8 = c.m.r32(((p_set_3).wrapping_add(12)).wrapping_add((((l_type_6) as u32)).wrapping_mul(16)));
                    {
                        l_i_7 = 0i32;
                        'l3: loop {
                            if !(((((l_i_7) < (((c.m.r8((l_def_8).wrapping_add(1))) as i32))) as i32)) != 0) { break 'l3; }
                            'c4: {
                                if (({ let a1_ = ((c.m.r8(l_def_8)) as i32); F::EftEmit_IsPartDeferred(c, p_set_3, a1_, l_i_7) }) != 0) {
                                    break 'c4;
                                }
                                l_wf_10 = (c.m.r32(l_w_5) as i32);
                                if ((((((((l_wf_10) & (2048i32))) != 0) && ((((((({ let a_ = ((c.m.r32((p_set_3).wrapping_add(8))).wrapping_add((((((({ let a_ = (((p_set_3).wrapping_add(12)).wrapping_add((((c.m.r8(l_def_8)) as u32)).wrapping_mul(16))).wrapping_add(8); c.m.r8(a_) }) as i32)).wrapping_add(l_i_7)) as u32)).wrapping_mul(64))).wrapping_add(8); c.m.r8(a_) }) as i32)) == (5i32)) as i32)) != 0)) as i32)) != 0) {
                                    break 'c4;
                                }
                                if (((((p_reset_4) == (0i32)) as i32)) != 0) {
                                    l_flags_9 = F::EftEmit_GetFlagsFromReq(c, p_set_3, (l_w_5).wrapping_add(148), p_objId_1, l_type_6, l_i_7, ((l_wf_10) & (1i32)), ((l_wf_10) & (4i32)));
                                } else {
                                    l_flags_9 = F::EftEmit_GetResetFlags(c, p_set_3, (l_w_5).wrapping_add(148), l_type_6, l_i_7);
                                }
                                if (((((c.m.r32(l_w_5) as i32)) & (128i32))) != 0) {
                                    l_flags_9 = 2i32;
                                }
                                if (((((l_flags_9) != (0i32)) as i32)) != 0) {
                                    { let a8_ = F32(c.m.r32((l_w_5).wrapping_add(16))); F::EftEmit_SpawnOwn(c, p_set_3, (l_w_5).wrapping_add(148), (l_w_5).wrapping_add(864), (l_w_5).wrapping_add(96), (l_w_5).wrapping_add(64), l_type_6, l_i_7, l_flags_9, a8_) };
                                }
                            }
                            { let o_ = l_i_7; let n_ = (o_).wrapping_add(1); l_i_7 = n_; };
                        }
                    }
                }
            }
            { let o_ = l_type_6; let n_ = (o_).wrapping_add(1); l_type_6 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftVolley_UpdateParts", &PROBE_N, &[], true); }
    r_
}

pub fn EftVolley_Init(c: &mut Ctx, mut p_task_1: u32, mut p_arg_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVolley_Init", &PROBE_N, &[(p_task_1) as u32, (p_arg_2) as u32], false); }
    let mut l_set_3: u32 = 0;
    let mut l_w_4: u32 = 0;
    let mut l_slot_5: u32 = 0;
    let mut l_p_6: u32 = 0;
    let mut l_id_7: i32 = 0;
    let r_ = 'f: {
    l_set_3 = { let a_ = (F::BtlTask_GetParent(c, p_task_1)).wrapping_add(56); c.m.r32(a_) };
    l_w_4 = c.m.r32((p_task_1).wrapping_add(56));
    l_slot_5 = p_arg_2;
    { let _ = F::memset(c, l_w_4, 0i32, 5072u32); };
    { let v_ = l_slot_5; let a_ = (l_w_4).wrapping_add(144); c.m.w32(a_, v_); };
    l_p_6 = c.m.r32((l_slot_5).wrapping_add(36));
    { let v_ = F32(c.m.r32((l_p_6).wrapping_add(52))); let a_ = (l_w_4).wrapping_add(4); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_p_6).wrapping_add(56))); let a_ = (l_w_4).wrapping_add(8); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_p_6).wrapping_add(48))); let a_ = (l_w_4).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_p_6).wrapping_add(48))); let a_ = (l_w_4).wrapping_add(16); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_w_4).wrapping_add(12))); let a_ = (l_w_4).wrapping_add(20); c.m.w32(a_, (v_).0); };
    { let v_ = l_set_3; let a_ = (l_w_4).wrapping_add(1456); c.m.w32(a_, v_); };
    F::EftEmit_InitState(c, l_set_3, (l_w_4).wrapping_add(148));
    { let v_ = rt::i2f({ let a0_ = c.m.r32((l_w_4).wrapping_add(1456)); F::EftEmit_GetEndFrames(c, a0_) }); let a_ = (l_w_4).wrapping_add(32); c.m.w32(a_, (v_).0); };
    l_id_7 = (({ let a_ = c.m.r32((l_slot_5).wrapping_add(36)); (c.m.r16(a_) as i16) }) as i32);
    if (((((l_id_7) == (454i32)) as i32)) != 0) {
        { let r_ = 512i32; let a_ = l_w_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if (((({ let a_ = (c.m.r32((l_slot_5).wrapping_add(36))).wrapping_add(60); (c.m.r32(a_) as i32) }) & (131072i32))) != 0) {
        { let r_ = 1024i32; let a_ = l_w_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if (((((((((((({ let a_ = c.m.r32((l_slot_5).wrapping_add(36)); (c.m.r16(a_) as i16) }) as u16)) as i32)).wrapping_sub(622i32)) as u32)) < (2u32)) as i32)) != 0) {
        { let r_ = ((2048i32) | (4096i32)); let a_ = l_w_4; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    { let a1_ = (if ((((((c.m.r32(l_slot_5) as i32)) == (0i32)) as i32)) != 0) { 2048i32 } else { 4096i32 }); F::BtlTask_SetOwnerTag(c, p_task_1, a1_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftVolley_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftVolley_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVolley_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_slot_3: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_slot_3 = c.m.r32((l_w_2).wrapping_add(144));
    { let a0_ = c.m.r32((l_w_2).wrapping_add(1456)); F::EftEmit_TermState(c, a0_, (l_w_2).wrapping_add(148)) };
    if ((((((({ let a_ = (c.m.r32((l_slot_3).wrapping_add(36))).wrapping_add(4); (c.m.r8(a_) as i8) }) as i32)) != (0i32)) as i32)) != 0) {
        { let a0_ = (c.m.r32(l_slot_3) as i32); F::EftShot_SetHeldFlagA8(c, a0_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftVolley_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftVolley_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVolley_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let mut l_slot_3: u32 = 0;
    let mut l_next_4: i32 = 0;
    let mut l_alive_5: i32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    l_slot_3 = c.m.r32((l_w_2).wrapping_add(144));
    l_next_4 = 0i32;
    if (({ let a0_ = (c.m.r32(l_slot_3) as i32); F::BtlScene_IsCharStopped(c, a0_) }) != 0) {
        break 'f;
    }
    F::EftEmit_UpdateNodesReq(c, l_slot_3, (l_w_2).wrapping_add(864));
    if ((((!(((((c.m.r32(l_w_2) as i32)) & (64i32))) != 0)) as i32)) != 0) {
        if (({ let a0_ = (c.m.r32(l_slot_3) as i32); F::EftShot_TestBits(c, a0_, 2i32) }) != 0) {
            { let a0_ = (c.m.r32(l_slot_3) as i32); F::EftVolley_Aim(c, a0_, p_task_1, ((l_w_2).wrapping_add(864)).wrapping_add(32), 0i32) };
            { let v_ = ((0i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
            { let r_ = 64i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        } else {
            if (({ let a0_ = (c.m.r32(l_slot_3) as i32); F::EftShot_TestBits(c, a0_, 4i32) }) != 0) {
                { let v_ = ((0i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                { let r_ = 64i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        }
    }
    if (((((c.m.r32(l_w_2) as i32)) & (64i32))) != 0) {
        's1: {
            let sv6: i32 = ((c.m.r8((p_task_1).wrapping_add(1))) as i32);
            match sv6 {
                0i32 => {
                    if (({ let a0_ = (c.m.r32(l_slot_3) as i32); F::EftShot_TestBits(c, a0_, 4i32) }) != 0) {
                        { let r_ = 16i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        F::Vec4_Copy(c, (l_w_2).wrapping_add(80), ((l_w_2).wrapping_add(864)).wrapping_add(96));
                        F::Vec4_Copy(c, (l_w_2).wrapping_add(96), (l_w_2).wrapping_add(80));
                        { let a0_ = (c.m.r32(l_slot_3) as i32); F::EftVolley_Aim(c, a0_, p_task_1, (l_w_2).wrapping_add(96), 1i32) };
                        { let v_ = ((1i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                    }
                    break 's1;
                }
                1i32 => {
                    l_next_4 = 1i32;
                    break 's1;
                }
                2i32 => {
                    if (((((({ let a0_ = (c.m.r32(l_slot_3) as i32); F::EftShot_TestBits(c, a0_, 4i32) }) != 0) && (((({ let a_ = (c.m.r32((l_slot_3).wrapping_add(36))).wrapping_add(60); (c.m.r32(a_) as i32) }) & (2048i32))) != 0)) as i32)) != 0) {
                        F::Vec4_Copy(c, (l_w_2).wrapping_add(80), ((l_w_2).wrapping_add(864)).wrapping_add(96));
                        F::Vec4_Copy(c, (l_w_2).wrapping_add(96), (l_w_2).wrapping_add(80));
                        { let a0_ = (c.m.r32(l_slot_3) as i32); F::EftVolley_Aim(c, a0_, p_task_1, (l_w_2).wrapping_add(96), 1i32) };
                    }
                    if (({ let a0_ = (c.m.r32(l_slot_3) as i32); F::EftShot_TestBits(c, a0_, 8i32) }) != 0) {
                        { let r_ = ((1i32) | (8i32)); let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        { let v_ = ((3i32) as u8); let a_ = (p_task_1).wrapping_add(1); c.m.w8(a_, v_); };
                    }
                    break 's1;
                }
                3i32 => {
                    l_next_4 = 1i32;
                    break 's1;
                }
                _ => {
                }
            }
        }
    }
    l_alive_5 = { let a0_ = (c.m.r32(l_slot_3) as i32); F::EftVolley_UpdateShots(c, a0_, p_task_1) };
    if (((((((((((((l_alive_5) == (0i32)) as i32)) != 0) && (((((c.m.r32(l_w_2) as i32)) & (256i32))) != 0)) as i32)) != 0) && ((((!((((((c.m.r32((l_w_2).wrapping_add(5060)) as i32)) < ((({ let a_ = (c.m.r32((l_slot_3).wrapping_add(36))).wrapping_add(9); (c.m.r8(a_) as i8) }) as i32))) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
        { let r_ = 1i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if (({ let a0_ = (c.m.r32(l_slot_3) as i32); F::EftShot_TestBits(c, a0_, 1024i32) }) != 0) {
        if ((((!(((((c.m.r32(l_w_2) as i32)) & (8i32))) != 0)) as i32)) != 0) {
            { let r_ = 4i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        { let r_ = 1i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    { let a0_ = (c.m.r32(l_slot_3) as i32); let a2_ = c.m.r32((l_w_2).wrapping_add(1456)); F::EftVolley_UpdateParts(c, a0_, p_task_1, a2_, 0i32) };
    if ((l_next_4) != 0) {
        { let a_ = (p_task_1).wrapping_add(1); let o_ = c.m.r8(a_); let n_ = (o_).wrapping_add(1); c.m.w8(a_, n_); };
    }
    if (((((c.m.r32(l_w_2) as i32)) & (1i32))) != 0) {
        { let r_ = F32(0x3f800000); let a_ = (l_w_2).wrapping_add(28); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
    }
    if (((((((((l_alive_5) == (0i32)) as i32)) != 0) && (((((c.m.r32(l_w_2) as i32)) & (2i32))) != 0)) as i32)) != 0) {
        F::BtlTask_SetDead(c, p_task_1);
    } else {
        if (((((c.m.r32(l_w_2) as i32)) & (1i32))) != 0) {
            if (((((((((c.m.r32(l_w_2) as i32)) & (4i32))) != 0) || (((rt::fge(F32(c.m.r32((l_w_2).wrapping_add(28))), F32(c.m.r32((l_w_2).wrapping_add(32)))) as i32)) != 0)) as i32)) != 0) {
                { let r_ = 2i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        } else {
            if (((({ let a_ = (c.m.r32((l_slot_3).wrapping_add(36))).wrapping_add(60); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                if (((((c.m.r32(l_w_2) as i32)) & (16i32))) != 0) {
                    { let a0_ = c.m.r32((l_w_2).wrapping_add(1456)); F::EftEmit_UpdateTrailWidth(c, a0_, (l_w_2).wrapping_add(148)) };
                }
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftVolley_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftVolley_PostUpdate(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVolley_PostUpdate", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if (({ let a0_ = { let a_ = c.m.r32((l_w_2).wrapping_add(144)); (c.m.r32(a_) as i32) }; F::BtlScene_IsCharStopped(c, a0_) }) != 0) {
        break 'f;
    }
    { let _ = { let a0_ = c.m.r32((l_w_2).wrapping_add(1456)); F::EftEmit_UpdateAlive(c, a0_, (l_w_2).wrapping_add(148)) }; };
    { let _ = F::EftVolley_PruneShots(c, p_task_1); };
    if ((((((c.m.r16((p_task_1).wrapping_add(8))) as i32)) & (4i32))) != 0) {
        { let r_ = 1i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftVolley_PostUpdate", &PROBE_N, &[], true); }
    r_
}

pub fn EftVolley_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVolley_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let mut l_w_2: u32 = 0;
    let r_ = 'f: {
    l_w_2 = c.m.r32((p_task_1).wrapping_add(56));
    if ((((!(((((c.m.r32(l_w_2) as i32)) & (128i32))) != 0)) as i32)) != 0) {
        { let r_ = 128i32; let a_ = l_w_2; let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        { let a0_ = c.m.r32((l_w_2).wrapping_add(1456)); F::EftEmit_KillAll(c, a0_, (l_w_2).wrapping_add(148)) };
    }
    F::BtlTask_SetDead(c, p_task_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftVolley_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftVolley_Draw(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVolley_Draw", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftVolley_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn EftVolleyGroup_Init(c: &mut Ctx, mut p_task_1: u32, mut p_slot_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVolleyGroup_Init", &PROBE_N, &[(p_task_1) as u32, (p_slot_2) as u32], false); }
    let mut l_set_3: u32 = 0;
    let r_ = 'f: {
    l_set_3 = c.m.r32((p_task_1).wrapping_add(56));
    { let _ = ((804u32) as i32); F::EftShot_Nop(c) };
    { let _ = F::memset(c, l_set_3, 0i32, 804u32); };
    { let a3_ = c.m.r32((p_slot_2).wrapping_add(28)); F::EftEmit_LoadSet(c, p_slot_2, l_set_3, ((0i32) as u32), a3_, 0i32, 4i32) };
    { let _ = F::BtlTask_CreateChildList(c, p_task_1, ((((2i32) as i64 as u64)) as u32), ((((((5072u32) as i32)) as i64 as u64)) as u32)); };
    if (((({ let a_ = (c.m.r32((p_slot_2).wrapping_add(36))).wrapping_add(60); (c.m.r32(a_) as i32) }) & (131072i32))) != 0) {
        { let v_ = { let a0_ = c.m.r32((p_slot_2).wrapping_add(28)); F::BtlScene_GetPackEntry(c, a0_, 1i32) }; let a_ = (l_set_3).wrapping_add(800); c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftVolleyGroup_Init", &PROBE_N, &[], true); }
    r_
}

pub fn EftVolleyGroup_Term(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVolleyGroup_Term", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32((p_task_1).wrapping_add(56)); F::EftEmit_FreeSet(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftVolleyGroup_Term", &PROBE_N, &[], true); }
    r_
}

pub fn EftVolleyGroup_Update(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVolleyGroup_Update", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
    { let a0_ = c.m.r32((p_task_1).wrapping_add(56)); F::EftEmit_BeginFrame(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftVolleyGroup_Update", &PROBE_N, &[], true); }
    r_
}

pub fn EftVolleyGroup_Reset(c: &mut Ctx, mut p_task_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftVolleyGroup_Reset", &PROBE_N, &[(p_task_1) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftVolleyGroup_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_TypeToResKind(c: &mut Ctx, mut p_type_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_TypeToResKind", &PROBE_N, &[(p_type_1) as u32], false); }
    let mut l_kind_2: i32 = 0;
    let r_ = 'f: {
    l_kind_2 = 0i32;
    's1: {
        let sv3: i32 = p_type_1;
        match sv3 {
            2i32 => {
                l_kind_2 = 3i32;
                break 's1;
            }
            5i32 | 10i32 | 12i32 | 15i32 | 16i32 | 17i32 | 18i32 => {
                l_kind_2 = 1i32;
                break 's1;
            }
            9i32 => {
                l_kind_2 = 2i32;
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f l_kind_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_TypeToResKind", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_LoadSet(c: &mut Ctx, mut p_owner_1: u32, mut p_set_2: u32, mut p_given_3: u32, mut p_base_4: u32, mut p_common_5: i32, mut p_idx_6: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_LoadSet", &PROBE_N, &[(p_owner_1) as u32, (p_set_2) as u32, (p_given_3) as u32, (p_base_4) as u32, (p_common_5) as u32, (p_idx_6) as u32], false); }
    let mut l_nPart_7: i32 = 0;
    let mut l_nPair_8: i32 = 0;
    let mut l_nRes_9: i32 = 0;
    let mut l_i_10: i32 = 0;
    let mut l_head_11: u32 = 0;
    let mut l_defs_12: u32 = 0;
    let mut l_off_13: i32 = 0;
    let mut l_parts_14: u32 = 0;
    let mut l_g_15: u32 = 0;
    let mut l_next_16: i32 = 0;
    let mut l_tbl_17: u32 = 0;
    let mut l_j_18: i32 = 0;
    let mut l_def_19: u32 = 0;
    let mut l_count_20: u32 = 0;
    let mut l_cOff_21: i32 = 0;
    let mut l_n_22: i32 = 0;
    let mut l_def_23: u32 = 0;
    let mut l_two_24: i32 = 0;
    let mut l_n_25: i32 = 0;
    let mut l_m_26: i32 = 0;
    let mut l_n_27: i32 = 0;
    let r_ = 'f: {
    l_nPart_7 = 0i32;
    l_nPair_8 = 0i32;
    l_nRes_9 = 0i32;
    { let v_ = p_owner_1; let a_ = (p_set_2).wrapping_add(796); c.m.w32(a_, v_); };
    if (((((p_given_3) != (((0i32) as u32))) as i32)) != 0) {
        l_head_11 = p_given_3;
    } else {
        if (((((p_common_5) == (1i32)) as i32)) != 0) {
            l_head_11 = F::BtlScene_GetCommonEntry(c, p_idx_6);
        } else {
            l_head_11 = F::BtlScene_GetPackEntry(c, p_base_4, p_idx_6);
        }
    }
    l_off_13 = ((64u32) as i32);
    { let v_ = l_head_11; let a_ = p_set_2; c.m.w32(a_, v_); };
    l_defs_12 = (l_head_11).wrapping_add(((l_off_13) as u32));
    { let v_ = l_defs_12; let a_ = (p_set_2).wrapping_add(4); c.m.w32(a_, v_); };
    l_off_13 = ((((((c.m.r8((l_head_11).wrapping_add(4))) as u32)).wrapping_mul(32u32)).wrapping_add(((l_off_13) as u32))) as i32);
    l_parts_14 = (l_head_11).wrapping_add(((l_off_13) as u32));
    { let v_ = l_parts_14; let a_ = (p_set_2).wrapping_add(8); c.m.w32(a_, v_); };
    l_tbl_17 = (p_set_2).wrapping_add(12);
    {
        l_i_10 = 3i32;
        'l1: loop {
            if !(((((l_i_10) >= (0i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = 0i32; let a_ = (((p_set_2).wrapping_add(12)).wrapping_add(768)).wrapping_add((((l_i_10) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
            }
            { let o_ = l_i_10; let n_ = (o_).wrapping_sub(1); l_i_10 = n_; };
        }
    }
    {
        l_i_10 = 0i32;
        'l3: loop {
            if !(((((l_i_10) < (((c.m.r8((l_head_11).wrapping_add(4))) as i32))) as i32)) != 0) { break 'l3; }
            'c4: {
                l_def_19 = (l_defs_12).wrapping_add((((l_i_10) as u32)).wrapping_mul(32));
                l_g_15 = ((p_set_2).wrapping_add(12)).wrapping_add((((c.m.r8(l_def_19)) as u32)).wrapping_mul(16));
                { let v_ = (l_parts_14).wrapping_add((((l_nPart_7) as u32)).wrapping_mul(64)); let a_ = (l_g_15).wrapping_add(4); c.m.w32(a_, v_); };
                { let v_ = l_def_19; let a_ = l_g_15; c.m.w32(a_, v_); };
                { let v_ = ((l_nPart_7) as u8); let a_ = (l_g_15).wrapping_add(8); c.m.w8(a_, v_); };
                { let v_ = ((l_nPair_8) as u8); let a_ = (l_g_15).wrapping_add(9); c.m.w8(a_, v_); };
                { let v_ = ((l_nRes_9) as u8); let a_ = (l_g_15).wrapping_add(10); c.m.w8(a_, v_); };
                { let v_ = (({ let a0_ = ((c.m.r8(l_def_19)) as i32); F::EftEmit_TypeToResKind(c, a0_) }) as u8); let a_ = (l_g_15).wrapping_add(11); c.m.w8(a_, v_); };
                l_cOff_21 = ((((((c.m.r8((l_g_15).wrapping_add(11))) as u32)).wrapping_mul(4u32)).wrapping_add((((192i32) as u32)).wrapping_mul(4u32))) as i32);
                l_count_20 = (l_tbl_17).wrapping_add(((l_cOff_21) as u32));
                { let v_ = (((c.m.r32(l_count_20) as i32)) as u8); let a_ = (l_g_15).wrapping_add(12); c.m.w8(a_, v_); };
                l_n_22 = (({ let a_ = (c.m.r32(l_g_15)).wrapping_add(1); c.m.r8(a_) }) as i32);
                { let r_ = l_n_22; let n_ = (l_nPart_7).wrapping_add(r_); l_nPart_7 = n_; };
                if (((((((c.m.r8(l_def_19)) as i32)) != (0i32)) as i32)) != 0) {
                    { let r_ = l_n_22; let n_ = (l_nPair_8).wrapping_add(r_); l_nPair_8 = n_; };
                    { let r_ = (({ let a_ = (c.m.r32(l_g_15)).wrapping_add(2); c.m.r8(a_) }) as i32); let n_ = (l_nRes_9).wrapping_add(r_); l_nRes_9 = n_; };
                    { let r_ = (({ let a_ = (c.m.r32(l_g_15)).wrapping_add(2); c.m.r8(a_) }) as i32); let a_ = l_count_20; let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                }
            }
            { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; };
        }
    }
    if (((((p_given_3) != (((0i32) as u32))) as i32)) != 0) {
        break 'f;
    }
    l_next_16 = (p_idx_6).wrapping_add(1i32);
    if ((((((c.m.r32((((p_set_2).wrapping_add(12)).wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) > (0i32)) as i32)) != 0) {
        { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = ((c.m.r32((((p_set_2).wrapping_add(12)).wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)).wrapping_mul(520i32); F::BtlPool_Alloc(c, a0_, a1_) }; let a_ = (((p_set_2).wrapping_add(12)).wrapping_add(752)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    if ((((((c.m.r32((((p_set_2).wrapping_add(12)).wrapping_add(768)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) > (0i32)) as i32)) != 0) {
        { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = ((c.m.r32((((p_set_2).wrapping_add(12)).wrapping_add(768)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)).wrapping_mul(264i32); F::BtlPool_Alloc(c, a0_, a1_) }; let a_ = (((p_set_2).wrapping_add(12)).wrapping_add(752)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    if ((((((c.m.r32((((p_set_2).wrapping_add(12)).wrapping_add(768)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) > (0i32)) as i32)) != 0) {
        { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = ((c.m.r32((((p_set_2).wrapping_add(12)).wrapping_add(768)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)).wrapping_mul(136i32); F::BtlPool_Alloc(c, a0_, a1_) }; let a_ = (((p_set_2).wrapping_add(12)).wrapping_add(752)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    if ((((((c.m.r32((((p_set_2).wrapping_add(12)).wrapping_add(768)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) > (0i32)) as i32)) != 0) {
        { let v_ = { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = ((c.m.r32((((p_set_2).wrapping_add(12)).wrapping_add(768)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)).wrapping_mul(72i32); F::BtlPool_Alloc(c, a0_, a1_) }; let a_ = (((p_set_2).wrapping_add(12)).wrapping_add(752)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    {
        l_i_10 = 0i32;
        'l5: loop {
            if !(((((l_i_10) < (((c.m.r8((l_head_11).wrapping_add(4))) as i32))) as i32)) != 0) { break 'l5; }
            'c6: {
                l_def_23 = (l_defs_12).wrapping_add((((l_i_10) as u32)).wrapping_mul(32));
                if (((((((c.m.r8(l_def_23)) as i32)) == (0i32)) as i32)) != 0) {
                    break 'c6;
                }
                l_two_24 = 1i32;
                l_g_15 = ((p_set_2).wrapping_add(12)).wrapping_add((((c.m.r8(l_def_23)) as u32)).wrapping_mul(16));
                if (((((((((((c.m.r8(l_def_23)) as i32)) == (2i32)) as i32)) != 0) || (((((((c.m.r8(l_def_23)) as i32)) == (14i32)) as i32)) != 0)) as i32)) != 0) {
                    l_two_24 = 0i32;
                }
                {
                    l_j_18 = 0i32;
                    'l7: loop {
                        if !(((((l_j_18) < (((c.m.r8((l_def_23).wrapping_add(2))) as i32))) as i32)) != 0) { break 'l7; }
                        'c8: {
                            l_n_25 = (((c.m.r8((l_g_15).wrapping_add(10))) as i32)).wrapping_add(l_j_18);
                            if (((((p_common_5) == (1i32)) as i32)) != 0) {
                                { let v_ = F::BtlScene_GetCommonEntry(c, { let o_ = l_next_16; let n_ = (o_).wrapping_add(1); l_next_16 = n_; o_ }); let a_ = ((p_set_2).wrapping_add(12)).wrapping_add(((((76i32).wrapping_add(l_n_25)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                            } else {
                                { let v_ = F::BtlScene_GetPackEntry(c, p_base_4, { let o_ = l_next_16; let n_ = (o_).wrapping_add(1); l_next_16 = n_; o_ }); let a_ = ((p_set_2).wrapping_add(12)).wrapping_add(((((76i32).wrapping_add(l_n_25)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                            }
                            l_m_26 = (((c.m.r8((l_g_15).wrapping_add(12))) as i32)).wrapping_add(l_j_18);
                            's9: {
                                let sv28: i32 = ((c.m.r8((l_g_15).wrapping_add(11))) as i32);
                                match sv28 {
                                    0i32 => {
                                        { let a0_ = (c.m.r32((((p_set_2).wrapping_add(12)).wrapping_add(752)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))).wrapping_add((((l_m_26).wrapping_mul(520i32)) as u32)); let a1_ = c.m.r32(((p_set_2).wrapping_add(12)).wrapping_add(((((76i32).wrapping_add(l_n_25)) as u32)).wrapping_mul(4))); F::EftTexSet_Load32(c, a0_, a1_) };
                                        break 's9;
                                    }
                                    1i32 => {
                                        { let a0_ = (c.m.r32((((p_set_2).wrapping_add(12)).wrapping_add(752)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))).wrapping_add((((l_m_26).wrapping_mul(264i32)) as u32)); let a1_ = c.m.r32(((p_set_2).wrapping_add(12)).wrapping_add(((((76i32).wrapping_add(l_n_25)) as u32)).wrapping_mul(4))); F::EftTexSet_Load16(c, a0_, a1_) };
                                        break 's9;
                                    }
                                    2i32 => {
                                        { let a0_ = (c.m.r32((((p_set_2).wrapping_add(12)).wrapping_add(752)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))).wrapping_add((((l_m_26).wrapping_mul(136i32)) as u32)); let a1_ = c.m.r32(((p_set_2).wrapping_add(12)).wrapping_add(((((76i32).wrapping_add(l_n_25)) as u32)).wrapping_mul(4))); F::EftTexSet_Load8(c, a0_, a1_) };
                                        break 's9;
                                    }
                                    3i32 => {
                                        { let a0_ = (c.m.r32((((p_set_2).wrapping_add(12)).wrapping_add(752)).wrapping_add((((3i32) as u32)).wrapping_mul(4)))).wrapping_add((((l_m_26).wrapping_mul(72i32)) as u32)); let a1_ = c.m.r32(((p_set_2).wrapping_add(12)).wrapping_add(((((76i32).wrapping_add(l_n_25)) as u32)).wrapping_mul(4))); F::EftTexSet_Load4(c, a0_, a1_) };
                                        break 's9;
                                    }
                                    _ => {
                                    }
                                }
                            }
                        }
                        { let o_ = l_j_18; let n_ = (o_).wrapping_add(1); l_j_18 = n_; };
                    }
                }
                {
                    l_j_18 = 0i32;
                    'l10: loop {
                        if !(((((l_j_18) < (((c.m.r8((l_def_23).wrapping_add(1))) as i32))) as i32)) != 0) { break 'l10; }
                        'c11: {
                            l_n_27 = (((c.m.r8((l_g_15).wrapping_add(9))) as i32)).wrapping_add(l_j_18);
                            if (((((p_common_5) == (1i32)) as i32)) != 0) {
                                { let v_ = F::BtlScene_GetCommonEntry(c, { let o_ = l_next_16; let n_ = (o_).wrapping_add(1); l_next_16 = n_; o_ }); let a_ = ((p_set_2).wrapping_add(12)).wrapping_add(((((54i32).wrapping_add(l_n_27)) as u32)).wrapping_mul(8)); c.m.w32(a_, v_); };
                            } else {
                                { let v_ = F::BtlScene_GetPackEntry(c, p_base_4, { let o_ = l_next_16; let n_ = (o_).wrapping_add(1); l_next_16 = n_; o_ }); let a_ = ((p_set_2).wrapping_add(12)).wrapping_add(((((54i32).wrapping_add(l_n_27)) as u32)).wrapping_mul(8)); c.m.w32(a_, v_); };
                            }
                            if ((l_two_24) != 0) {
                                if (((((p_common_5) == (1i32)) as i32)) != 0) {
                                    { let v_ = F::BtlScene_GetCommonEntry(c, { let o_ = l_next_16; let n_ = (o_).wrapping_add(1); l_next_16 = n_; o_ }); let a_ = (((p_set_2).wrapping_add(12)).wrapping_add(((((54i32).wrapping_add(l_n_27)) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, v_); };
                                } else {
                                    { let v_ = F::BtlScene_GetPackEntry(c, p_base_4, { let o_ = l_next_16; let n_ = (o_).wrapping_add(1); l_next_16 = n_; o_ }); let a_ = (((p_set_2).wrapping_add(12)).wrapping_add(((((54i32).wrapping_add(l_n_27)) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.w32(a_, v_); };
                                }
                            }
                        }
                        { let o_ = l_j_18; let n_ = (o_).wrapping_add(1); l_j_18 = n_; };
                    }
                }
            }
            { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_LoadSet", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_FreeSet(c: &mut Ctx, mut p_set_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_FreeSet", &PROBE_N, &[(p_set_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((0i32) as u32); let a_ = p_set_1; c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (p_set_1).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = ((0i32) as u32); let a_ = (p_set_1).wrapping_add(8); c.m.w32(a_, v_); };
    if ((((((c.m.r32((((p_set_1).wrapping_add(12)).wrapping_add(768)).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)) > (0i32)) as i32)) != 0) {
        { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32((((p_set_1).wrapping_add(12)).wrapping_add(752)).wrapping_add((((0i32) as u32)).wrapping_mul(4))); F::BtlPool_Free(c, a0_, a1_) };
    }
    if ((((((c.m.r32((((p_set_1).wrapping_add(12)).wrapping_add(768)).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)) > (0i32)) as i32)) != 0) {
        { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32((((p_set_1).wrapping_add(12)).wrapping_add(752)).wrapping_add((((1i32) as u32)).wrapping_mul(4))); F::BtlPool_Free(c, a0_, a1_) };
    }
    if ((((((c.m.r32((((p_set_1).wrapping_add(12)).wrapping_add(768)).wrapping_add((((2i32) as u32)).wrapping_mul(4))) as i32)) > (0i32)) as i32)) != 0) {
        { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32((((p_set_1).wrapping_add(12)).wrapping_add(752)).wrapping_add((((2i32) as u32)).wrapping_mul(4))); F::BtlPool_Free(c, a0_, a1_) };
    }
    if ((((((c.m.r32((((p_set_1).wrapping_add(12)).wrapping_add(768)).wrapping_add((((3i32) as u32)).wrapping_mul(4))) as i32)) > (0i32)) as i32)) != 0) {
        { let a0_ = F::BtlPool_GetCurrent(c); let a1_ = c.m.r32((((p_set_1).wrapping_add(12)).wrapping_add(752)).wrapping_add((((3i32) as u32)).wrapping_mul(4))); F::BtlPool_Free(c, a0_, a1_) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_FreeSet", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_BeginFrame(c: &mut Ctx, mut p_set_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_BeginFrame", &PROBE_N, &[(p_set_1) as u32], false); }
    let mut l_kind_2: i32 = 0;
    let mut l_i_3: i32 = 0;
    let r_ = 'f: {
    {
        l_kind_2 = 0i32;
        'l1: loop {
            if !(((((l_kind_2) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                {
                    l_i_3 = 0i32;
                    'l3: loop {
                        if !(((((l_i_3) < ((c.m.r32((((p_set_1).wrapping_add(12)).wrapping_add(768)).wrapping_add((((l_kind_2) as u32)).wrapping_mul(4))) as i32))) as i32)) != 0) { break 'l3; }
                        'c4: {
                            's5: {
                                let sv4: i32 = l_kind_2;
                                match sv4 {
                                    0i32 => {
                                        if (((((c.m.r32((((p_set_1).wrapping_add(12)).wrapping_add(752)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
                                            { let v_ = 0i32; let a_ = ((c.m.r32((((p_set_1).wrapping_add(12)).wrapping_add(752)).wrapping_add((((0i32) as u32)).wrapping_mul(4)))).wrapping_add((((l_i_3).wrapping_mul(520i32)) as u32))).wrapping_add(((516i32) as u32)); c.m.w32(a_, (v_) as u32); };
                                        }
                                        break 's5;
                                    }
                                    1i32 => {
                                        if (((((c.m.r32((((p_set_1).wrapping_add(12)).wrapping_add(752)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
                                            { let v_ = 0i32; let a_ = ((c.m.r32((((p_set_1).wrapping_add(12)).wrapping_add(752)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))).wrapping_add((((l_i_3).wrapping_mul(264i32)) as u32))).wrapping_add(((260i32) as u32)); c.m.w32(a_, (v_) as u32); };
                                        }
                                        break 's5;
                                    }
                                    2i32 => {
                                        if (((((c.m.r32((((p_set_1).wrapping_add(12)).wrapping_add(752)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
                                            { let v_ = 0i32; let a_ = ((c.m.r32((((p_set_1).wrapping_add(12)).wrapping_add(752)).wrapping_add((((2i32) as u32)).wrapping_mul(4)))).wrapping_add((((l_i_3).wrapping_mul(136i32)) as u32))).wrapping_add(((132i32) as u32)); c.m.w32(a_, (v_) as u32); };
                                        }
                                        break 's5;
                                    }
                                    3i32 => {
                                        if (((((c.m.r32((((p_set_1).wrapping_add(12)).wrapping_add(752)).wrapping_add((((3i32) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
                                            { let v_ = 0i32; let a_ = ((c.m.r32((((p_set_1).wrapping_add(12)).wrapping_add(752)).wrapping_add((((3i32) as u32)).wrapping_mul(4)))).wrapping_add((((l_i_3).wrapping_mul(72i32)) as u32))).wrapping_add(((68i32) as u32)); c.m.w32(a_, (v_) as u32); };
                                        }
                                        break 's5;
                                    }
                                    _ => {
                                    }
                                }
                            }
                        }
                        { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                    }
                }
            }
            { let o_ = l_kind_2; let n_ = (o_).wrapping_add(1); l_kind_2 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_BeginFrame", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_GetHead21(c: &mut Ctx, mut p_set_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_GetHead21", &PROBE_N, &[(p_set_1) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (c.m.r32(p_set_1)).wrapping_add(33); c.m.r8(a_) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_GetHead21", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_GetHead26(c: &mut Ctx, mut p_set_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_GetHead26", &PROBE_N, &[(p_set_1) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (c.m.r32(p_set_1)).wrapping_add(38); c.m.r8(a_) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_GetHead26", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_GetAimKind(c: &mut Ctx, mut p_set_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_GetAimKind", &PROBE_N, &[(p_set_1) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (c.m.r32(p_set_1)).wrapping_add(6); c.m.r8(a_) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_GetAimKind", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_GetHead22(c: &mut Ctx, mut p_set_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_GetHead22", &PROBE_N, &[(p_set_1) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (c.m.r32(p_set_1)).wrapping_add(34); c.m.r8(a_) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_GetHead22", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_GetHead25(c: &mut Ctx, mut p_set_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_GetHead25", &PROBE_N, &[(p_set_1) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (c.m.r32(p_set_1)).wrapping_add(37); c.m.r8(a_) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_GetHead25", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_GetHead28(c: &mut Ctx, mut p_set_1: u32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_GetHead28", &PROBE_N, &[(p_set_1) as u32], false); }
    let r_ = 'f: {
    break 'f { let a_ = (c.m.r32(p_set_1)).wrapping_add(40); F32(c.m.r32(a_)) };
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_GetHead28", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_GetEndFrames(c: &mut Ctx, mut p_set_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_GetEndFrames", &PROBE_N, &[(p_set_1) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (c.m.r32(p_set_1)).wrapping_add(32); c.m.r8(a_) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_GetEndFrames", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_GetPhaseMask(c: &mut Ctx, mut p_set_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_GetPhaseMask", &PROBE_N, &[(p_set_1) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (c.m.r32(p_set_1)).wrapping_add(39); c.m.r8(a_) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_GetPhaseMask", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_GetPartFlags(c: &mut Ctx, mut p_set_1: u32, mut p_type_2: i32, mut p_idx_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_GetPartFlags", &PROBE_N, &[(p_set_1) as u32, (p_type_2) as u32, (p_idx_3) as u32], false); }
    let r_ = 'f: {
    break 'f (({ let a_ = (c.m.r32((p_set_1).wrapping_add(8))).wrapping_add(((((((c.m.r8((((p_set_1).wrapping_add(12)).wrapping_add((((p_type_2) as u32)).wrapping_mul(16))).wrapping_add(8))) as i32)).wrapping_add(p_idx_3)) as u32)).wrapping_mul(64)); c.m.r8(a_) }) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_GetPartFlags", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_IsPartDeferred(c: &mut Ctx, mut p_set_1: u32, mut p_type_2: i32, mut p_idx_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_IsPartDeferred", &PROBE_N, &[(p_set_1) as u32, (p_type_2) as u32, (p_idx_3) as u32], false); }
    let mut l_n_4: i32 = 0;
    let mut l_ret_5: i32 = 0;
    let mut l_phase_6: i32 = 0;
    let mut l_a_7: i32 = 0;
    let r_ = 'f: {
    l_n_4 = (((c.m.r8((((p_set_1).wrapping_add(12)).wrapping_add((((p_type_2) as u32)).wrapping_mul(16))).wrapping_add(8))) as i32)).wrapping_add(p_idx_3);
    l_ret_5 = 0i32;
    l_phase_6 = (({ let a_ = ((c.m.r32((p_set_1).wrapping_add(8))).wrapping_add((((l_n_4) as u32)).wrapping_mul(64))).wrapping_add(8); c.m.r8(a_) }) as i32);
    l_a_7 = (({ let a_ = ((c.m.r32((p_set_1).wrapping_add(8))).wrapping_add((((l_n_4) as u32)).wrapping_mul(64))).wrapping_add(10); c.m.r8(a_) }) as i32);
    if ((((((({ let a_ = (c.m.r32(p_set_1)).wrapping_add(39); c.m.r8(a_) }) as i32)).wrapping_shr((l_phase_6) as u32)) & (1i32))) != 0) {
        if (((((l_a_7) == (5i32)) as i32)) != 0) {
            l_ret_5 = 1i32;
        } else {
            's1: {
                let sv8: i32 = l_phase_6;
                match sv8 {
                    0i32 => {
                        if (((((l_a_7) == (0i32)) as i32)) != 0) {
                            l_ret_5 = 1i32;
                        }
                        break 's1;
                    }
                    1i32 => {
                        if (((((l_a_7) == (1i32)) as i32)) != 0) {
                            l_ret_5 = 1i32;
                        }
                        break 's1;
                    }
                    3i32 => {
                        if (((((l_a_7) == (2i32)) as i32)) != 0) {
                            l_ret_5 = 1i32;
                        }
                        break 's1;
                    }
                    4i32 => {
                        if (((((l_a_7) == (3i32)) as i32)) != 0) {
                            l_ret_5 = 1i32;
                        }
                        break 's1;
                    }
                    5i32 => {
                        l_ret_5 = (((l_a_7) == (4i32)) as i32);
                        break 's1;
                    }
                    _ => {
                    }
                }
            }
        }
    }
    break 'f l_ret_5;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_IsPartDeferred", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_InitState(c: &mut Ctx, mut p_set_1: u32, mut p_st_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_InitState", &PROBE_N, &[(p_set_1) as u32, (p_st_2) as u32], false); }
    let mut l_i_3: i32 = 0;
    let mut l_head_4: u32 = 0;
    let mut l_parts_5: u32 = 0;
    let mut l_h_6: u32 = 0;
    let mut l_j_7: i32 = 0;
    let r_ = 'f: {
    { let v_ = c.m.r32((p_set_1).wrapping_add(796)); let a_ = (p_st_2).wrapping_add(712); c.m.w32(a_, v_); };
    l_head_4 = c.m.r32(p_set_1);
    l_parts_5 = c.m.r32((p_set_1).wrapping_add(8));
    {
        l_i_3 = 0i32;
        'l1: loop {
            if !(((((l_i_3) < (((c.m.r8((l_head_4).wrapping_add(5))) as i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                { let v_ = ((0i32) as u8); let a_ = ((p_st_2).wrapping_add(320)).wrapping_add(((l_i_3) as u32)); c.m.w8(a_, v_); };
                { let v_ = F32(c.m.r32(((l_parts_5).wrapping_add((((l_i_3) as u32)).wrapping_mul(64))).wrapping_add(28))); let a_ = ((p_st_2).wrapping_add(360)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_).0); };
            }
            { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
        }
    }
    { let v_ = F32(c.m.r32((l_head_4).wrapping_add(8))); let a_ = (p_st_2).wrapping_add(680); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(F32(c.m.r32((l_head_4).wrapping_add(20))), F32(0x41f00000)); let a_ = (p_st_2).wrapping_add(688); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(F32(c.m.r32((p_st_2).wrapping_add(688))), F32(c.m.r32((l_head_4).wrapping_add(24)))); let a_ = (p_st_2).wrapping_add(684); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (p_st_2).wrapping_add(692); c.m.w32(a_, (v_).0); };
    { let v_ = F32(c.m.r32((l_head_4).wrapping_add(44))); let a_ = (p_st_2).wrapping_add(696); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(F32(c.m.r32((l_head_4).wrapping_add(56))), F32(0x41f00000)); let a_ = (p_st_2).wrapping_add(704); c.m.w32(a_, (v_).0); };
    { let v_ = rt::fmul(F32(c.m.r32((p_st_2).wrapping_add(704))), F32(c.m.r32((l_head_4).wrapping_add(60)))); let a_ = (p_st_2).wrapping_add(700); c.m.w32(a_, (v_).0); };
    { let v_ = F32(0x00000000); let a_ = (p_st_2).wrapping_add(708); c.m.w32(a_, (v_).0); };
    l_h_6 = (p_st_2).wrapping_add((((0i32) as u32)).wrapping_mul(160));
    {
        l_j_7 = 0i32;
        'l3: loop {
            if !(((((l_j_7) < (2i32)) as i32)) != 0) { break 'l3; }
            'c4: {
                {
                    l_i_3 = 39i32;
                    'l5: loop {
                        if !(((((l_i_3) >= (0i32)) as i32)) != 0) { break 'l5; }
                        'c6: {
                            { let v_ = ((0i32) as u32); let a_ = (l_h_6).wrapping_add((((l_i_3) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        }
                        { let o_ = l_i_3; let n_ = (o_).wrapping_sub(1); l_i_3 = n_; };
                    }
                }
            }
            { { let o_ = l_j_7; let n_ = (o_).wrapping_add(1); l_j_7 = n_; }; { let r_ = 40i32; let n_ = (l_h_6).wrapping_add((((r_) as u32)).wrapping_mul(4)); l_h_6 = n_; }; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_InitState", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_TermState(c: &mut Ctx, mut p_set_1: u32, mut p_st_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_TermState", &PROBE_N, &[(p_set_1) as u32, (p_st_2) as u32], false); }
    let r_ = 'f: {
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_TermState", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_GetFlagsFromReq(c: &mut Ctx, mut p_set_1: u32, mut p_st_2: u32, mut p_objId_3: i32, mut p_type_4: i32, mut p_idx_5: i32, mut p_ending_6: i32, mut p_kill_7: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_GetFlagsFromReq", &PROBE_N, &[(p_set_1) as u32, (p_st_2) as u32, (p_objId_3) as u32, (p_type_4) as u32, (p_idx_5) as u32, (p_ending_6) as u32, (p_kill_7) as u32], false); }
    let mut l_req_8: i32 = 0;
    let r_ = 'f: {
    l_req_8 = 2i32;
    if ((((!((F::EftShot_TestBits(c, p_objId_3, 2i32)) != 0)) as i32)) != 0) {
        l_req_8 = 0i32;
    }
    if ((F::EftShot_TestBits(c, p_objId_3, 4i32)) != 0) {
        { let r_ = 4i32; let n_ = ((l_req_8) | (r_)); l_req_8 = n_; };
    }
    if ((F::EftShot_TestBits(c, p_objId_3, 8i32)) != 0) {
        { let r_ = 8i32; let n_ = ((l_req_8) | (r_)); l_req_8 = n_; };
    }
    if ((F::EftShot_TestBits(c, p_objId_3, 16i32)) != 0) {
        { let r_ = 16i32; let n_ = ((l_req_8) | (r_)); l_req_8 = n_; };
    }
    if ((F::EftShot_TestBits(c, p_objId_3, 32i32)) != 0) {
        { let r_ = 32i32; let n_ = ((l_req_8) | (r_)); l_req_8 = n_; };
    }
    if ((F::EftShot_TestBits(c, p_objId_3, 64i32)) != 0) {
        { let r_ = 64i32; let n_ = ((l_req_8) | (r_)); l_req_8 = n_; };
    }
    break 'f F::EftEmit_GetFlags(c, p_set_1, p_st_2, p_objId_3, p_type_4, p_idx_5, p_ending_6, p_kill_7, l_req_8);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_GetFlagsFromReq", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_GetFlagsFromMask(c: &mut Ctx, mut p_set_1: u32, mut p_st_2: u32, mut p_objId_3: i32, mut p_type_4: i32, mut p_idx_5: i32, mut p_ending_6: i32, mut p_kill_7: i32, mut p_mask_8: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_GetFlagsFromMask", &PROBE_N, &[(p_set_1) as u32, (p_st_2) as u32, (p_objId_3) as u32, (p_type_4) as u32, (p_idx_5) as u32, (p_ending_6) as u32, (p_kill_7) as u32, (p_mask_8) as u32], false); }
    let mut l_req_9: i32 = 0;
    let r_ = 'f: {
    l_req_9 = (((p_mask_8) & (1i32))).wrapping_shl((1i32) as u32);
    if ((((p_mask_8) & (2i32))) != 0) {
        { let r_ = 4i32; let n_ = ((l_req_9) | (r_)); l_req_9 = n_; };
    }
    if ((((p_mask_8) & (4i32))) != 0) {
        { let r_ = 8i32; let n_ = ((l_req_9) | (r_)); l_req_9 = n_; };
    }
    if ((((p_mask_8) & (8i32))) != 0) {
        { let r_ = 16i32; let n_ = ((l_req_9) | (r_)); l_req_9 = n_; };
    }
    if ((((p_mask_8) & (16i32))) != 0) {
        { let r_ = 32i32; let n_ = ((l_req_9) | (r_)); l_req_9 = n_; };
    }
    if ((((p_mask_8) & (32i32))) != 0) {
        { let r_ = 64i32; let n_ = ((l_req_9) | (r_)); l_req_9 = n_; };
    }
    break 'f F::EftEmit_GetFlags(c, p_set_1, p_st_2, p_objId_3, p_type_4, p_idx_5, p_ending_6, p_kill_7, l_req_9);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_GetFlagsFromMask", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_GetFlags(c: &mut Ctx, mut p_set_1: u32, mut p_st_2: u32, mut p_objId_3: i32, mut p_type_4: i32, mut p_idx_5: i32, mut p_ending_6: i32, mut p_kill_7: i32, mut p_req_8: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_GetFlags", &PROBE_N, &[(p_set_1) as u32, (p_st_2) as u32, (p_objId_3) as u32, (p_type_4) as u32, (p_idx_5) as u32, (p_ending_6) as u32, (p_kill_7) as u32, (p_req_8) as u32], false); }
    let mut l_g_9: u32 = 0;
    let mut l_flags_10: i32 = 0;
    let mut l_def_11: u32 = 0;
    let mut l_part_12: u32 = 0;
    let mut l_n_13: i32 = 0;
    let mut l_f_14: i32 = 0;
    let mut l_started_15: i32 = 0;
    let mut l_pf_16: i32 = 0;
    let mut l_f2_17: i32 = 0;
    let mut l_a_18: i32 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
    'fl1: loop {
        match bb_ {
            0 => {
                l_g_9 = ((p_set_1).wrapping_add(12)).wrapping_add((((p_type_4) as u32)).wrapping_mul(16));
                l_flags_10 = 0i32;
                l_n_13 = (((c.m.r8((l_g_9).wrapping_add(8))) as i32)).wrapping_add(p_idx_5);
                l_def_11 = c.m.r32(l_g_9);
                l_part_12 = (c.m.r32((p_set_1).wrapping_add(8))).wrapping_add((((l_n_13) as u32)).wrapping_mul(64));
                if (((((p_kill_7) == (0i32)) as i32)) != 0) { bb_ = 1; continue 'fl1; } else { bb_ = 3; continue 'fl1; }
            }
            1 => {
                if ((((p_req_8) & (2i32))) != 0) { bb_ = 4; continue 'fl1; } else { bb_ = 5; continue 'fl1; }
            }
            2 => {
                if (((((p_kill_7) != (0i32)) as i32)) != 0) { bb_ = 36; continue 'fl1; } else { bb_ = 38; continue 'fl1; }
            }
            3 => {
                l_flags_10 = 4i32;
                bb_ = 2; continue 'fl1;
            }
            4 => {
                l_flags_10 = (((((c.m.r8((l_part_12).wrapping_add(8))) as i32)) == (0i32)) as i32);
                if (((((((c.m.r8((l_part_12).wrapping_add(9))) as i32)) == (1i32)) as i32)) != 0) { bb_ = 6; continue 'fl1; } else { bb_ = 7; continue 'fl1; }
            }
            5 => {
                if ((((p_req_8) & (4i32))) != 0) { bb_ = 8; continue 'fl1; } else { bb_ = 9; continue 'fl1; }
            }
            6 => {
                { let r_ = 2i32; let n_ = ((l_flags_10) | (r_)); l_flags_10 = n_; };
                bb_ = 7; continue 'fl1;
            }
            7 => {
                bb_ = 5; continue 'fl1;
            }
            8 => {
                if (((((((c.m.r8((l_part_12).wrapping_add(8))) as i32)) == (1i32)) as i32)) != 0) { bb_ = 10; continue 'fl1; } else { bb_ = 11; continue 'fl1; }
            }
            9 => {
                if ((((p_req_8) & (16i32))) != 0) { bb_ = 14; continue 'fl1; } else { bb_ = 15; continue 'fl1; }
            }
            10 => {
                { let r_ = 1i32; let n_ = ((l_flags_10) | (r_)); l_flags_10 = n_; };
                bb_ = 11; continue 'fl1;
            }
            11 => {
                if (((((((c.m.r8((l_part_12).wrapping_add(9))) as i32)) == (2i32)) as i32)) != 0) { bb_ = 12; continue 'fl1; } else { bb_ = 13; continue 'fl1; }
            }
            12 => {
                { let r_ = 2i32; let n_ = ((l_flags_10) | (r_)); l_flags_10 = n_; };
                bb_ = 13; continue 'fl1;
            }
            13 => {
                bb_ = 9; continue 'fl1;
            }
            14 => {
                if (((((((c.m.r8((l_part_12).wrapping_add(8))) as i32)) == (3i32)) as i32)) != 0) { bb_ = 16; continue 'fl1; } else { bb_ = 17; continue 'fl1; }
            }
            15 => {
                if ((((p_req_8) & (32i32))) != 0) { bb_ = 20; continue 'fl1; } else { bb_ = 21; continue 'fl1; }
            }
            16 => {
                { let r_ = 1i32; let n_ = ((l_flags_10) | (r_)); l_flags_10 = n_; };
                bb_ = 17; continue 'fl1;
            }
            17 => {
                if (((((((c.m.r8((l_part_12).wrapping_add(9))) as i32)) == (4i32)) as i32)) != 0) { bb_ = 18; continue 'fl1; } else { bb_ = 19; continue 'fl1; }
            }
            18 => {
                { let r_ = 2i32; let n_ = ((l_flags_10) | (r_)); l_flags_10 = n_; };
                bb_ = 19; continue 'fl1;
            }
            19 => {
                bb_ = 15; continue 'fl1;
            }
            20 => {
                if (((((((c.m.r8((l_part_12).wrapping_add(8))) as i32)) == (4i32)) as i32)) != 0) { bb_ = 22; continue 'fl1; } else { bb_ = 23; continue 'fl1; }
            }
            21 => {
                if ((((p_req_8) & (64i32))) != 0) { bb_ = 26; continue 'fl1; } else { bb_ = 27; continue 'fl1; }
            }
            22 => {
                { let r_ = 1i32; let n_ = ((l_flags_10) | (r_)); l_flags_10 = n_; };
                bb_ = 23; continue 'fl1;
            }
            23 => {
                if (((((((c.m.r8((l_part_12).wrapping_add(9))) as i32)) == (5i32)) as i32)) != 0) { bb_ = 24; continue 'fl1; } else { bb_ = 25; continue 'fl1; }
            }
            24 => {
                { let r_ = 2i32; let n_ = ((l_flags_10) | (r_)); l_flags_10 = n_; };
                bb_ = 25; continue 'fl1;
            }
            25 => {
                bb_ = 21; continue 'fl1;
            }
            26 => {
                if (((((((c.m.r8((l_part_12).wrapping_add(8))) as i32)) == (5i32)) as i32)) != 0) { bb_ = 28; continue 'fl1; } else { bb_ = 29; continue 'fl1; }
            }
            27 => {
                if ((((((c.m.r8(((p_st_2).wrapping_add(320)).wrapping_add(((l_n_13) as u32)))) as i32)) & (64i32))) != 0) { bb_ = 32; continue 'fl1; } else { bb_ = 33; continue 'fl1; }
            }
            28 => {
                { let r_ = 1i32; let n_ = ((l_flags_10) | (r_)); l_flags_10 = n_; };
                bb_ = 29; continue 'fl1;
            }
            29 => {
                if (((((((c.m.r8((l_part_12).wrapping_add(9))) as i32)) == (6i32)) as i32)) != 0) { bb_ = 30; continue 'fl1; } else { bb_ = 31; continue 'fl1; }
            }
            30 => {
                { let r_ = 2i32; let n_ = ((l_flags_10) | (r_)); l_flags_10 = n_; };
                bb_ = 31; continue 'fl1;
            }
            31 => {
                bb_ = 27; continue 'fl1;
            }
            32 => {
                { let r_ = 1i32; let n_ = ((l_flags_10) | (r_)); l_flags_10 = n_; };
                bb_ = 33; continue 'fl1;
            }
            33 => {
                if ((((((((((c.m.r8(((p_st_2).wrapping_add(320)).wrapping_add(((l_n_13) as u32)))) as i32)) & (128i32))) != 0) && (((((((c.m.r8(l_def_11)) as i32)) == (17i32)) as i32)) != 0)) as i32)) != 0) { bb_ = 34; continue 'fl1; } else { bb_ = 35; continue 'fl1; }
            }
            34 => {
                { let r_ = 256i32; let n_ = ((l_flags_10) | (r_)); l_flags_10 = n_; };
                { let r_ = 127i32; let a_ = ((p_st_2).wrapping_add(320)).wrapping_add(((l_n_13) as u32)); let n_ = ((((((c.m.r8(a_)) as i32)) & (r_))) as u8); c.m.w8(a_, n_); };
                bb_ = 35; continue 'fl1;
            }
            35 => {
                bb_ = 2; continue 'fl1;
            }
            36 => {
                { let r_ = 2i32; let n_ = ((l_flags_10) | (r_)); l_flags_10 = n_; };
                bb_ = 37; continue 'fl1;
            }
            37 => {
                l_f_14 = ((c.m.r8(((p_st_2).wrapping_add(320)).wrapping_add(((l_n_13) as u32)))) as i32);
                l_started_15 = ((l_f_14) & (2i32));
                if ((l_started_15) != 0) { bb_ = 46; continue 'fl1; } else { bb_ = 47; continue 'fl1; }
            }
            38 => {
                if (((((p_ending_6) != (0i32)) as i32)) != 0) { bb_ = 39; continue 'fl1; } else { bb_ = 40; continue 'fl1; }
            }
            39 => {
                if (((((((c.m.r8((l_part_12).wrapping_add(8))) as i32)) == (2i32)) as i32)) != 0) { bb_ = 41; continue 'fl1; } else { bb_ = 43; continue 'fl1; }
            }
            40 => {
                bb_ = 37; continue 'fl1;
            }
            41 => {
                { let r_ = 1i32; let n_ = ((l_flags_10) | (r_)); l_flags_10 = n_; };
                bb_ = 42; continue 'fl1;
            }
            42 => {
                bb_ = 40; continue 'fl1;
            }
            43 => {
                if (((((((c.m.r8((l_part_12).wrapping_add(8))) as i32)) != (6i32)) as i32)) != 0) { bb_ = 44; continue 'fl1; } else { bb_ = 45; continue 'fl1; }
            }
            44 => {
                { let r_ = 2i32; let n_ = ((l_flags_10) | (r_)); l_flags_10 = n_; };
                bb_ = 45; continue 'fl1;
            }
            45 => {
                bb_ = 42; continue 'fl1;
            }
            46 => {
                { let r_ = (!(1i32)); let n_ = ((l_flags_10) & (r_)); l_flags_10 = n_; };
                bb_ = 47; continue 'fl1;
            }
            47 => {
                if ((((l_f_14) & (4i32))) != 0) { bb_ = 48; continue 'fl1; } else { bb_ = 49; continue 'fl1; }
            }
            48 => {
                { let r_ = (!(2i32)); let n_ = ((l_flags_10) & (r_)); l_flags_10 = n_; };
                bb_ = 49; continue 'fl1;
            }
            49 => {
                l_pf_16 = ((c.m.r8(l_part_12)) as i32);
                if ((((l_pf_16) & (2i32))) != 0) { bb_ = 50; continue 'fl1; } else { bb_ = 52; continue 'fl1; }
            }
            50 => {
                if (((((((((((c.m.r8(l_def_11)) as i32)) == (17i32)) as i32)) != 0) || (((((((c.m.r8(l_def_11)) as i32)) == (15i32)) as i32)) != 0)) as i32)) != 0) { bb_ = 53; continue 'fl1; } else { bb_ = 55; continue 'fl1; }
            }
            51 => {
                l_f2_17 = ((c.m.r8(((p_st_2).wrapping_add(320)).wrapping_add(((l_n_13) as u32)))) as i32);
                if ((((l_f2_17) & (2i32))) != 0) { bb_ = 62; continue 'fl1; } else { bb_ = 63; continue 'fl1; }
            }
            52 => {
                if ((((!((l_started_15) != 0)) as i32)) != 0) { bb_ = 56; continue 'fl1; } else { bb_ = 57; continue 'fl1; }
            }
            53 => {
                { let r_ = (!(16i32)); let n_ = ((l_flags_10) & (r_)); l_flags_10 = n_; };
                { let r_ = 512i32; let n_ = ((l_flags_10) | (r_)); l_flags_10 = n_; };
                bb_ = 54; continue 'fl1;
            }
            54 => {
                bb_ = 51; continue 'fl1;
            }
            55 => {
                { let r_ = (!(16i32)); let n_ = ((l_flags_10) & (r_)); l_flags_10 = n_; };
                bb_ = 54; continue 'fl1;
            }
            56 => {
                bb_ = 58; continue 'fl1;
            }
            57 => {
                if (((((((c.m.r8((l_part_12).wrapping_add(8))) as i32)) != (2i32)) as i32)) != 0) { bb_ = 60; continue 'fl1; } else { bb_ = 61; continue 'fl1; }
            }
            58 => {
                break 'f l_flags_10;
            }
            59 => {
                bb_ = 57; continue 'fl1;
            }
            60 => {
                { let r_ = 16i32; let n_ = ((l_flags_10) | (r_)); l_flags_10 = n_; };
                bb_ = 61; continue 'fl1;
            }
            61 => {
                bb_ = 51; continue 'fl1;
            }
            62 => {
                if ((((!((((l_f2_17) & (4i32))) != 0)) as i32)) != 0) { bb_ = 64; continue 'fl1; } else { bb_ = 65; continue 'fl1; }
            }
            63 => {
                bb_ = 58; continue 'fl1;
            }
            64 => {
                { let r_ = 64i32; let n_ = ((l_flags_10) | (r_)); l_flags_10 = n_; };
                bb_ = 65; continue 'fl1;
            }
            65 => {
                if ((((!((((l_f2_17) & (4i32))) != 0)) as i32)) != 0) { bb_ = 66; continue 'fl1; } else { bb_ = 67; continue 'fl1; }
            }
            66 => {
                l_a_18 = ((c.m.r8((l_part_12).wrapping_add(10))) as i32);
                if ((((l_f2_17) & (32i32))) != 0) { bb_ = 68; continue 'fl1; } else { bb_ = 69; continue 'fl1; }
            }
            67 => {
                bb_ = 63; continue 'fl1;
            }
            68 => {
                l_a_18 = ((c.m.r8((l_part_12).wrapping_add(54))) as i32);
                bb_ = 69; continue 'fl1;
            }
            69 => {
                if (((((l_a_18) == (5i32)) as i32)) != 0) { bb_ = 70; continue 'fl1; } else { bb_ = 72; continue 'fl1; }
            }
            70 => {
                { let r_ = 128i32; let n_ = ((l_flags_10) | (r_)); l_flags_10 = n_; };
                bb_ = 71; continue 'fl1;
            }
            71 => {
                bb_ = 67; continue 'fl1;
            }
            72 => {
                if ((((l_pf_16) & (128i32))) != 0) { bb_ = 73; continue 'fl1; } else { bb_ = 74; continue 'fl1; }
            }
            73 => {
                { let r_ = 128i32; let n_ = ((l_flags_10) | (r_)); l_flags_10 = n_; };
                bb_ = 74; continue 'fl1;
            }
            74 => {
                bb_ = 71; continue 'fl1;
            }
            75 => {
                break 'fl1;
            }
            _ => unreachable!(),
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_GetFlags", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_GetResetFlags(c: &mut Ctx, mut p_set_1: u32, mut p_st_2: u32, mut p_type_3: i32, mut p_idx_4: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_GetResetFlags", &PROBE_N, &[(p_set_1) as u32, (p_st_2) as u32, (p_type_3) as u32, (p_idx_4) as u32], false); }
    let mut l_n_5: i32 = 0;
    let mut l_flags_6: i32 = 0;
    let mut l_f_7: i32 = 0;
    let mut l_part_8: u32 = 0;
    let mut l_a_9: i32 = 0;
    let r_ = 'f: {
    l_n_5 = (((c.m.r8((((p_set_1).wrapping_add(12)).wrapping_add((((p_type_3) as u32)).wrapping_mul(16))).wrapping_add(8))) as i32)).wrapping_add(p_idx_4);
    l_flags_6 = 0i32;
    l_f_7 = ((c.m.r8(((p_st_2).wrapping_add(320)).wrapping_add(((l_n_5) as u32)))) as i32);
    l_part_8 = (c.m.r32((p_set_1).wrapping_add(8))).wrapping_add((((l_n_5) as u32)).wrapping_mul(64));
    if ((((l_f_7) & (2i32))) != 0) {
        if ((((!((((l_f_7) & (4i32))) != 0)) as i32)) != 0) {
            l_a_9 = ((c.m.r8((l_part_8).wrapping_add(10))) as i32);
            if ((((l_f_7) & (32i32))) != 0) {
                l_a_9 = ((c.m.r8((l_part_8).wrapping_add(54))) as i32);
            }
            l_flags_6 = 48i32;
            if (((((((((p_type_3) != (17i32)) as i32)) != 0) && (((((l_a_9) != (5i32)) as i32)) != 0)) as i32)) != 0) {
                l_flags_6 = 0i32;
            }
        }
    }
    break 'f l_flags_6;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_GetResetFlags", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_TagTask(c: &mut Ctx, mut p_task_1: u32, mut p_chr_2: i32, mut p_type_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_TagTask", &PROBE_N, &[(p_task_1) as u32, (p_chr_2) as u32, (p_type_3) as u32], false); }
    let mut l_flags_4: i32 = 0;
    let r_ = 'f: {
    if (((((((((((((p_task_1) != (((0i32) as u32))) as i32)) != 0) && (((((p_type_3) != (2i32)) as i32)) != 0)) as i32)) != 0) && (((((p_type_3) != (5i32)) as i32)) != 0)) as i32)) != 0) {
        l_flags_4 = (if (((((p_chr_2) == (0i32)) as i32)) != 0) { 2048i32 } else { 4096i32 });
        F::BtlTask_SetOwnerTag(c, p_task_1, l_flags_4);
        l_flags_4 = 0i32;
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_TagTask", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_SpawnType0(c: &mut Ctx, mut p_set_1: u32, mut p_handles_2: u32, mut p_flags_3: i32, mut p_type_4: i32, mut p_chr_5: i32, mut p_idx_6: i32, mut p_pos_7: u32, mut p_size_8: F32, mut p_scale_9: F32, mut p_rate_10: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_SpawnType0", &PROBE_N, &[(p_set_1) as u32, (p_handles_2) as u32, (p_flags_3) as u32, (p_type_4) as u32, (p_chr_5) as u32, (p_idx_6) as u32, (p_pos_7) as u32, (p_size_8).0, (p_scale_9).0, (p_rate_10).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(192) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 192);
    let mut l_n_11: i32 = 0;
    let mut l_part_12: u32 = 0;
    let mut l_kind_13: i32 = 0;
    let mut l_h_14: u32 = 0;
    let mut sv_15: i32 = 0;
    let mut bb_: u32 = 0;
    let r_ = 'f: {
        'fl1: loop {
            match bb_ {
                0 => {
                    l_n_11 = (((c.m.r8((((p_set_1).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(16))).wrapping_add(8))) as i32)).wrapping_add(p_idx_6);
                    l_part_12 = (c.m.r32((p_set_1).wrapping_add(8))).wrapping_add((((l_n_11) as u32)).wrapping_mul(64));
                    l_kind_13 = ((c.m.r8((l_part_12).wrapping_add(4))) as i32);
                    sv_15 = l_kind_13;
                    bb_ = match sv_15 { 0i32 => 3, 1i32 => 11, _ => 15, }; continue 'fl1;
                }
                1 => {
                    if ((((p_flags_3) & (((16i32) | (2i32))))) != 0) { bb_ = 17; continue 'fl1; } else { bb_ = 18; continue 'fl1; }
                }
                2 => {
                    bb_ = 3; continue 'fl1;
                }
                3 => {
                    if ((((p_flags_3) & (1i32))) != 0) { bb_ = 4; continue 'fl1; } else { bb_ = 5; continue 'fl1; }
                }
                4 => {
                    rt::memset(c, fp, 0, 96);
                    rt::memset(c, fp.wrapping_add(0), 0, 16);
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(0), (v_).0); }
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(4), (v_).0); }
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(8), (v_).0); }
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(0).wrapping_add(12), (v_).0); }
                    rt::memset(c, fp.wrapping_add(16), 0, 16);
                    { let v_ = 192i32; c.m.w32(fp.wrapping_add(16).wrapping_add(0), (v_) as u32); }
                    { let v_ = 192i32; c.m.w32(fp.wrapping_add(16).wrapping_add(4), (v_) as u32); }
                    { let v_ = 192i32; c.m.w32(fp.wrapping_add(16).wrapping_add(8), (v_) as u32); }
                    { let v_ = 112i32; c.m.w32(fp.wrapping_add(16).wrapping_add(12), (v_) as u32); }
                    { let v_ = p_rate_10; c.m.w32(fp.wrapping_add(32), (v_).0); }
                    { let v_ = rt::fmul(p_scale_9, F32(0x42c80000)); c.m.w32(fp.wrapping_add(36), (v_).0); }
                    { let v_ = p_size_8; c.m.w32(fp.wrapping_add(40), (v_).0); }
                    { let v_ = rt::fmul(F32(c.m.r32((l_part_12).wrapping_add(16))), p_scale_9); c.m.w32(fp.wrapping_add(44), (v_).0); }
                    { let v_ = rt::fmul(F32(c.m.r32((l_part_12).wrapping_add(20))), p_scale_9); c.m.w32(fp.wrapping_add(48), (v_).0); }
                    { let v_ = 1i32; c.m.w32(fp.wrapping_add(52), (v_) as u32); }
                    { let v_ = ((c.m.r8((l_part_12).wrapping_add(3))) as i32); c.m.w32(fp.wrapping_add(56), (v_) as u32); }
                    { let v_ = p_chr_5; c.m.w32(fp.wrapping_add(60), (v_) as u32); }
                    { let v_ = 1i32; c.m.w32(fp.wrapping_add(64), (v_) as u32); }
                    { let v_ = 0i32; c.m.w32(fp.wrapping_add(68), (v_) as u32); }
                    { let v_ = rt::f2i(rt::i2f((c.m.r8((l_part_12).wrapping_add(5))) as i32)); c.m.w32(fp.wrapping_add(72), (v_) as u32); }
                    { let v_ = rt::f2i(rt::i2f((c.m.r8((l_part_12).wrapping_add(6))) as i32)); c.m.w32(fp.wrapping_add(76), (v_) as u32); }
                    { let v_ = 0i32; c.m.w32(fp.wrapping_add(80), (v_) as u32); }
                    if (((((((c.m.r8((l_part_12).wrapping_add(8))) as i32)) == (2i32)) as i32)) != 0) { bb_ = 6; continue 'fl1; } else { bb_ = 7; continue 'fl1; }
                }
                5 => {
                    bb_ = 1; continue 'fl1;
                }
                6 => {
                    { let v_ = 1i32; let a_ = (fp).wrapping_add(80); c.m.w32(a_, (v_) as u32); };
                    bb_ = 7; continue 'fl1;
                }
                7 => {
                    F::Vec4_Copy(c, fp, p_pos_7);
                    l_h_14 = (p_handles_2).wrapping_add((((l_n_11) as u32)).wrapping_mul(4));
                    { let v_ = F::EftRay_Create(c, fp); let a_ = l_h_14; c.m.w32(a_, v_); };
                    { let a0_ = c.m.r32(l_h_14); { let _ = F::EftRay_SetType(c, a0_, p_type_4); } };
                    bb_ = 8; continue 'fl1;
                }
                8 => {
                    { let a0_ = c.m.r32(l_h_14); F::EftEmit_TagTask(c, a0_, p_chr_5, p_type_4) };
                    bb_ = 13; continue 'fl1;
                }
                9 => {
                    bb_ = 5; continue 'fl1;
                }
                10 => {
                    bb_ = 11; continue 'fl1;
                }
                11 => {
                    if ((((p_flags_3) & (1i32))) != 0) { bb_ = 12; continue 'fl1; } else { bb_ = 13; continue 'fl1; }
                }
                12 => {
                    rt::memset(c, fp.wrapping_add(96), 0, 96);
                    rt::memset(c, fp.wrapping_add(96).wrapping_add(0), 0, 16);
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(96).wrapping_add(0).wrapping_add(0), (v_).0); }
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(96).wrapping_add(0).wrapping_add(4), (v_).0); }
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(96).wrapping_add(0).wrapping_add(8), (v_).0); }
                    { let v_ = F32(0x3f800000); c.m.w32(fp.wrapping_add(96).wrapping_add(0).wrapping_add(12), (v_).0); }
                    rt::memset(c, fp.wrapping_add(96).wrapping_add(16), 0, 16);
                    { let v_ = 128i32; c.m.w32(fp.wrapping_add(96).wrapping_add(16).wrapping_add(0), (v_) as u32); }
                    { let v_ = 128i32; c.m.w32(fp.wrapping_add(96).wrapping_add(16).wrapping_add(4), (v_) as u32); }
                    { let v_ = 128i32; c.m.w32(fp.wrapping_add(96).wrapping_add(16).wrapping_add(8), (v_) as u32); }
                    { let v_ = 128i32; c.m.w32(fp.wrapping_add(96).wrapping_add(16).wrapping_add(12), (v_) as u32); }
                    { let v_ = p_rate_10; c.m.w32(fp.wrapping_add(96).wrapping_add(32), (v_).0); }
                    { let v_ = rt::fmul(p_scale_9, F32(0x44480000)); c.m.w32(fp.wrapping_add(96).wrapping_add(36), (v_).0); }
                    { let v_ = F32(0x40000000); c.m.w32(fp.wrapping_add(96).wrapping_add(40), (v_).0); }
                    { let v_ = rt::fmul(F32(c.m.r32((l_part_12).wrapping_add(16))), p_scale_9); c.m.w32(fp.wrapping_add(96).wrapping_add(44), (v_).0); }
                    { let v_ = rt::fmul(F32(c.m.r32((l_part_12).wrapping_add(20))), p_scale_9); c.m.w32(fp.wrapping_add(96).wrapping_add(48), (v_).0); }
                    { let v_ = 3i32; c.m.w32(fp.wrapping_add(96).wrapping_add(52), (v_) as u32); }
                    { let v_ = ((c.m.r8((l_part_12).wrapping_add(3))) as i32); c.m.w32(fp.wrapping_add(96).wrapping_add(56), (v_) as u32); }
                    { let v_ = p_chr_5; c.m.w32(fp.wrapping_add(96).wrapping_add(60), (v_) as u32); }
                    { let v_ = 1i32; c.m.w32(fp.wrapping_add(96).wrapping_add(64), (v_) as u32); }
                    { let v_ = 1i32; c.m.w32(fp.wrapping_add(96).wrapping_add(68), (v_) as u32); }
                    { let v_ = rt::f2i(rt::i2f((c.m.r8((l_part_12).wrapping_add(5))) as i32)); c.m.w32(fp.wrapping_add(96).wrapping_add(72), (v_) as u32); }
                    { let v_ = rt::f2i(rt::i2f((c.m.r8((l_part_12).wrapping_add(6))) as i32)); c.m.w32(fp.wrapping_add(96).wrapping_add(76), (v_) as u32); }
                    { let v_ = 1i32; c.m.w32(fp.wrapping_add(96).wrapping_add(80), (v_) as u32); }
                    F::Vec4_Copy(c, fp.wrapping_add(96), p_pos_7);
                    l_h_14 = (p_handles_2).wrapping_add((((l_n_11) as u32)).wrapping_mul(4));
                    { let v_ = F::EftRay_CreateByValue(c, fp.wrapping_add(96)); let a_ = l_h_14; c.m.w32(a_, v_); };
                    { let a0_ = c.m.r32(l_h_14); { let _ = F::EftRay_SetType(c, a0_, p_type_4); } };
                    bb_ = 8; continue 'fl1;
                }
                13 => {
                    bb_ = 1; continue 'fl1;
                }
                14 => {
                    bb_ = 15; continue 'fl1;
                }
                15 => {
                    break 'f;
                }
                16 => {
                    bb_ = 1; continue 'fl1;
                }
                17 => {
                    { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_11) as u32)).wrapping_mul(4))); { let _ = F::EftRay_SetPos(c, a0_, p_pos_7); } };
                    bb_ = 18; continue 'fl1;
                }
                18 => {
                    if ((((p_flags_3) & (2i32))) != 0) { bb_ = 19; continue 'fl1; } else { bb_ = 20; continue 'fl1; }
                }
                19 => {
                    { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_11) as u32)).wrapping_mul(4))); { let _ = F::EftRay_Kill(c, a0_); } };
                    { let v_ = ((0i32) as u32); let a_ = (p_handles_2).wrapping_add((((l_n_11) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    bb_ = 20; continue 'fl1;
                }
                20 => {
                    break 'fl1;
                }
                _ => unreachable!(),
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_SpawnType0", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_SpawnType2(c: &mut Ctx, mut p_set_1: u32, mut p_handles_2: u32, mut p_flags_3: i32, mut p_type_4: i32, mut p_chr_5: i32, mut p_idx_6: i32, mut p_pos_7: u32, mut p_dir_8: u32, mut p_size_9: F32, mut p_scale_10: F32, mut p_rate_11: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_SpawnType2", &PROBE_N, &[(p_set_1) as u32, (p_handles_2) as u32, (p_flags_3) as u32, (p_type_4) as u32, (p_chr_5) as u32, (p_idx_6) as u32, (p_pos_7) as u32, (p_dir_8) as u32, (p_size_9).0, (p_scale_10).0, (p_rate_11).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(32) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 32);
    let mut l_g_12: u32 = 0;
    let mut l_n_13: i32 = 0;
    let mut l_part_14: u32 = 0;
    let mut l_res_15: u32 = 0;
    let mut l_tex_16: u32 = 0;
    let r_ = 'f: {
        l_g_12 = ((p_set_1).wrapping_add(12)).wrapping_add((((2i32) as u32)).wrapping_mul(16));
        l_n_13 = (((c.m.r8((l_g_12).wrapping_add(8))) as i32)).wrapping_add(p_idx_6);
        l_part_14 = (c.m.r32((p_set_1).wrapping_add(8))).wrapping_add((((l_n_13) as u32)).wrapping_mul(64));
        { let a3_ = rt::fmul(F32(c.m.r32((l_part_14).wrapping_add(16))), p_scale_10); F::Vec3_ScaleAdd(c, fp, p_dir_8, p_pos_7, a3_) };
        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        if ((((p_flags_3) & (1i32))) != 0) {
            if (((((c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4)))) == (((0i32) as u32))) as i32)) != 0) {
                l_res_15 = (c.m.r32((((p_set_1).wrapping_add(12)).wrapping_add(752)).wrapping_add((((3i32) as u32)).wrapping_mul(4)))).wrapping_add(((((((c.m.r8((l_g_12).wrapping_add(12))) as i32)).wrapping_add(((c.m.r8((l_part_14).wrapping_add(1))) as i32))).wrapping_mul(72i32)) as u32));
                l_tex_16 = { let a_ = (((p_set_1).wrapping_add(12)).wrapping_add(432)).wrapping_add(((((((c.m.r8((l_g_12).wrapping_add(9))) as i32)).wrapping_add(p_idx_6)) as u32)).wrapping_mul(8)); c.m.r32(a_) };
                rt::memset(c, fp.wrapping_add(16), 0, 16);
                { let v_ = ((p_chr_5) as u8); c.m.w8(fp.wrapping_add(16).wrapping_add(0), v_); }
                { let v_ = ((p_type_4) as u8); c.m.w8(fp.wrapping_add(16).wrapping_add(1), v_); }
                { let v_ = c.m.r8((l_part_14).wrapping_add(2)); c.m.w8(fp.wrapping_add(16).wrapping_add(2), v_); }
                { let v_ = ((0i32) as u8); c.m.w8(fp.wrapping_add(16).wrapping_add(3), v_); }
                { let v_ = l_tex_16; c.m.w32(fp.wrapping_add(16).wrapping_add(4), v_); }
                { let v_ = l_res_15; c.m.w32(fp.wrapping_add(16).wrapping_add(8), v_); }
                { let v_ = p_rate_11; c.m.w32(fp.wrapping_add(16).wrapping_add(12), (v_).0); }
                { let v_ = F::EftRays_Create(c, fp.wrapping_add(16)); let a_ = (p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftRays_SetSize(c, a0_, p_size_9) };
                if ((((((c.m.r8(l_part_14)) as i32)) & (64i32))) != 0) {
                    { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftRays_SetPos(c, a0_, p_pos_7) };
                } else {
                    { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftRays_SetPos(c, a0_, fp) };
                }
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); let a1_ = rt::i2f((c.m.r8((l_part_14).wrapping_add(5))) as i32); F::EftRays_SetDelay(c, a0_, a1_) };
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); let a1_ = rt::i2f((c.m.r8((l_part_14).wrapping_add(6))) as i32); F::EftRays_SetHold(c, a0_, a1_) };
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); let a1_ = rt::i2f((c.m.r8((l_part_14).wrapping_add(7))) as i32); F::EftRays_SetFade(c, a0_, a1_) };
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftEmit_TagTask(c, a0_, p_chr_5, p_type_4) };
            }
        }
        if (((((c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
            if ((((p_flags_3) & (((16i32) | (2i32))))) != 0) {
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftRays_SetPos(c, a0_, fp) };
            }
            if ((((p_flags_3) & (64i32))) != 0) {
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftRays_SetSize(c, a0_, p_size_9) };
            }
            if ((((p_flags_3) & (8i32))) != 0) {
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftRays_Kill(c, a0_) };
            } else {
                if ((((p_flags_3) & (2i32))) != 0) {
                    { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftRays_Stop(c, a0_) };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_SpawnType2", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_SpawnType16(c: &mut Ctx, mut p_set_1: u32, mut p_handles_2: u32, mut p_flags_3: i32, mut p_type_4: i32, mut p_chr_5: i32, mut p_idx_6: i32, mut p_pos_7: u32, mut p_dir_8: u32, mut p_size_9: F32, mut p_scale_10: F32, mut p_rate_11: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_SpawnType16", &PROBE_N, &[(p_set_1) as u32, (p_handles_2) as u32, (p_flags_3) as u32, (p_type_4) as u32, (p_chr_5) as u32, (p_idx_6) as u32, (p_pos_7) as u32, (p_dir_8) as u32, (p_size_9).0, (p_scale_10).0, (p_rate_11).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_g_12: u32 = 0;
    let mut l_n_13: i32 = 0;
    let mut l_part_14: u32 = 0;
    let mut l_res_15: u32 = 0;
    let mut l_ri_16: i32 = 0;
    let mut l_texA_17: u32 = 0;
    let mut l_texB_18: u32 = 0;
    let r_ = 'f: {
        l_g_12 = ((p_set_1).wrapping_add(12)).wrapping_add((((16i32) as u32)).wrapping_mul(16));
        l_n_13 = (((c.m.r8((l_g_12).wrapping_add(8))) as i32)).wrapping_add(p_idx_6);
        l_part_14 = (c.m.r32((p_set_1).wrapping_add(8))).wrapping_add((((l_n_13) as u32)).wrapping_mul(64));
        { let a3_ = rt::fmul(F32(c.m.r32((l_part_14).wrapping_add(16))), p_scale_10); F::Vec3_ScaleAdd(c, fp, p_dir_8, p_pos_7, a3_) };
        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        if ((((p_flags_3) & (1i32))) != 0) {
            if (((((c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4)))) == (((0i32) as u32))) as i32)) != 0) {
                l_res_15 = c.m.r32((((p_set_1).wrapping_add(12)).wrapping_add(752)).wrapping_add((((1i32) as u32)).wrapping_mul(4)));
                l_ri_16 = (((c.m.r8((l_g_12).wrapping_add(12))) as i32)).wrapping_add(((c.m.r8((l_part_14).wrapping_add(1))) as i32));
                l_texA_17 = { let a_ = ((p_set_1).wrapping_add(12)).wrapping_add((((((54i32).wrapping_add(((c.m.r8((l_g_12).wrapping_add(9))) as i32))).wrapping_add(p_idx_6)) as u32)).wrapping_mul(8)); c.m.r32(a_) };
                { let r_ = (l_ri_16).wrapping_mul(264i32); let n_ = (l_res_15).wrapping_add(((r_) as u32)); l_res_15 = n_; };
                l_texB_18 = { let a_ = (((p_set_1).wrapping_add(12)).wrapping_add((((((54i32).wrapping_add(((c.m.r8((l_g_12).wrapping_add(9))) as i32))).wrapping_add(p_idx_6)) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.r32(a_) };
                {
                    rt::memset(c, fp.wrapping_add(16), 0, 64);
                    rt::memset(c, fp.wrapping_add(16).wrapping_add(0), 0, 16);
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(0), (v_).0); }
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(4), (v_).0); }
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(8), (v_).0); }
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(12), (v_).0); }
                    rt::memset(c, fp.wrapping_add(16).wrapping_add(16), 0, 16);
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(0), (v_).0); }
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(4), (v_).0); }
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(8), (v_).0); }
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(12), (v_).0); }
                    { let v_ = p_chr_5; c.m.w32(fp.wrapping_add(16).wrapping_add(32), (v_) as u32); }
                    { let v_ = ((c.m.r8((l_part_14).wrapping_add(2))) as i32); c.m.w32(fp.wrapping_add(16).wrapping_add(36), (v_) as u32); }
                    { let v_ = p_rate_11; c.m.w32(fp.wrapping_add(16).wrapping_add(40), (v_).0); }
                    { let v_ = p_size_9; c.m.w32(fp.wrapping_add(16).wrapping_add(44), (v_).0); }
                    { let v_ = l_res_15; c.m.w32(fp.wrapping_add(16).wrapping_add(48), v_); }
                    { let v_ = l_texA_17; c.m.w32(fp.wrapping_add(16).wrapping_add(52), v_); }
                    { let v_ = l_texB_18; c.m.w32(fp.wrapping_add(16).wrapping_add(56), v_); }
                    if ((((((c.m.r8(l_part_14)) as i32)) & (64i32))) != 0) {
                        F::Vec4_Copy(c, fp.wrapping_add(16), p_pos_7);
                    } else {
                        F::Vec4_Copy(c, fp.wrapping_add(16), fp);
                    }
                    F::Vec4_Copy(c, (fp.wrapping_add(16)).wrapping_add(16), p_dir_8);
                    { let v_ = F::EftBill_Create(c, fp.wrapping_add(16)); let a_ = (p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); let a1_ = rt::i2f((c.m.r8((l_part_14).wrapping_add(5))) as i32); F::EftBill_SetDelay(c, a0_, a1_) }; };
                    { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); let a1_ = rt::i2f((c.m.r8((l_part_14).wrapping_add(6))) as i32); F::EftBill_SetEndDelay(c, a0_, a1_) }; };
                    if ((((((c.m.r8(l_part_14)) as i32)) & (32i32))) != 0) {
                        { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftBill_SetFront(c, a0_) }; };
                    }
                    { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftBill_SetType(c, a0_, p_type_4) }; };
                    { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftEmit_TagTask(c, a0_, p_chr_5, p_type_4) };
                }
            }
        }
        if (((((c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
            if ((((p_flags_3) & (16i32))) != 0) {
                { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftBill_SetPos(c, a0_, fp) }; };
            }
            if ((((p_flags_3) & (64i32))) != 0) {
                { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftBill_SetSize(c, a0_, p_size_9) }; };
            }
            if ((((p_flags_3) & (128i32))) != 0) {
                { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftBill_SetDir(c, a0_, p_dir_8) }; };
            }
            if ((((p_flags_3) & (8i32))) != 0) {
                { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftBill_Kill(c, a0_) }; };
                { let v_ = ((0i32) as u32); let a_ = (p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            } else {
                if ((((p_flags_3) & (2i32))) != 0) {
                    if ((((p_flags_3) & (4i32))) != 0) {
                        { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftBill_SetEndDelay(c, a0_, F32(0x00000000)) }; };
                    }
                    { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftBill_Stop(c, a0_) }; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_SpawnType16", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_SpawnType17(c: &mut Ctx, mut p_set_1: u32, mut p_handles_2: u32, mut p_flags_3: i32, mut p_type_4: i32, mut p_chr_5: i32, mut p_arg5_6: i32, mut p_idx_7: i32, mut p_pos_8: u32, mut p_pos2_9: u32, mut p_dir_10: u32, mut p_size_11: F32, mut p_scale_12: F32, mut p_rate_13: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_SpawnType17", &PROBE_N, &[(p_set_1) as u32, (p_handles_2) as u32, (p_flags_3) as u32, (p_type_4) as u32, (p_chr_5) as u32, (p_arg5_6) as u32, (p_idx_7) as u32, (p_pos_8) as u32, (p_pos2_9) as u32, (p_dir_10) as u32, (p_size_11).0, (p_scale_12).0, (p_rate_13).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_g_14: u32 = 0;
    let mut l_create_15: i32 = 0;
    let mut l_n_16: i32 = 0;
    let mut l_part_17: u32 = 0;
    let mut l_res_18: u32 = 0;
    let mut l_texA_19: u32 = 0;
    let mut l_texB_20: u32 = 0;
    let mut l_warp_21: i32 = 0;
    let r_ = 'f: {
        l_g_14 = ((p_set_1).wrapping_add(12)).wrapping_add((((17i32) as u32)).wrapping_mul(16));
        l_create_15 = 0i32;
        l_n_16 = (((c.m.r8((l_g_14).wrapping_add(8))) as i32)).wrapping_add(p_idx_7);
        l_part_17 = (c.m.r32((p_set_1).wrapping_add(8))).wrapping_add((((l_n_16) as u32)).wrapping_mul(64));
        { let a3_ = rt::fmul(F32(c.m.r32((l_part_17).wrapping_add(16))), p_scale_12); F::Vec3_ScaleAdd(c, fp, p_dir_10, p_pos_8, a3_) };
        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        { let a3_ = rt::fmul(F32(c.m.r32((l_part_17).wrapping_add(20))), p_scale_12); F::Vec3_ScaleAdd(c, fp.wrapping_add(16), p_dir_10, p_pos2_9, a3_) };
        { let v_ = F32(0x3f800000); let a_ = (fp.wrapping_add(16)).wrapping_add(12); c.m.w32(a_, (v_).0); };
        if ((((((((p_flags_3) & (1i32))) != 0) && (((((c.m.r32((p_handles_2).wrapping_add((((l_n_16) as u32)).wrapping_mul(4)))) == (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
            l_create_15 = 1i32;
        } else {
            if ((((p_flags_3) & (256i32))) != 0) {
                if (((((c.m.r32((p_handles_2).wrapping_add((((l_n_16) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
                    { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_16) as u32)).wrapping_mul(4))); F::EftRibbon_Kill(c, a0_) }; };
                    l_create_15 = 1i32;
                    { let v_ = ((0i32) as u32); let a_ = (p_handles_2).wrapping_add((((l_n_16) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
            }
        }
        if ((l_create_15) != 0) {
            l_res_18 = (c.m.r32((((p_set_1).wrapping_add(12)).wrapping_add(752)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))).wrapping_add(((((((c.m.r8((l_g_14).wrapping_add(12))) as i32)).wrapping_add(((c.m.r8((l_part_17).wrapping_add(1))) as i32))).wrapping_mul(264i32)) as u32));
            l_texA_19 = { let a_ = ((p_set_1).wrapping_add(12)).wrapping_add((((((54i32).wrapping_add(((c.m.r8((l_g_14).wrapping_add(9))) as i32))).wrapping_add(p_idx_7)) as u32)).wrapping_mul(8)); c.m.r32(a_) };
            l_texB_20 = { let a_ = (((p_set_1).wrapping_add(12)).wrapping_add((((((54i32).wrapping_add(((c.m.r8((l_g_14).wrapping_add(9))) as i32))).wrapping_add(p_idx_7)) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.r32(a_) };
            rt::memset(c, fp.wrapping_add(32), 0, 64);
            rt::memset(c, fp.wrapping_add(32).wrapping_add(0), 0, 16);
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(32).wrapping_add(0).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(32).wrapping_add(0).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(32).wrapping_add(0).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(32).wrapping_add(0).wrapping_add(12), (v_).0); }
            rt::memset(c, fp.wrapping_add(32).wrapping_add(16), 0, 16);
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(32).wrapping_add(16).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(32).wrapping_add(16).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(32).wrapping_add(16).wrapping_add(8), (v_).0); }
            { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(32).wrapping_add(16).wrapping_add(12), (v_).0); }
            { let v_ = p_type_4; c.m.w32(fp.wrapping_add(32).wrapping_add(32), (v_) as u32); }
            { let v_ = p_chr_5; c.m.w32(fp.wrapping_add(32).wrapping_add(36), (v_) as u32); }
            { let v_ = ((c.m.r8((l_part_17).wrapping_add(2))) as i32); c.m.w32(fp.wrapping_add(32).wrapping_add(40), (v_) as u32); }
            { let v_ = p_rate_13; c.m.w32(fp.wrapping_add(32).wrapping_add(44), (v_).0); }
            { let v_ = p_size_11; c.m.w32(fp.wrapping_add(32).wrapping_add(48), (v_).0); }
            { let v_ = l_res_18; c.m.w32(fp.wrapping_add(32).wrapping_add(52), v_); }
            { let v_ = l_texA_19; c.m.w32(fp.wrapping_add(32).wrapping_add(56), v_); }
            { let v_ = l_texB_20; c.m.w32(fp.wrapping_add(32).wrapping_add(60), v_); }
            F::Vec4_Copy(c, (fp.wrapping_add(32)).wrapping_add(16), p_dir_10);
            if ((((((c.m.r8(l_part_17)) as i32)) & (64i32))) != 0) {
                F::Vec4_Copy(c, fp.wrapping_add(32), p_pos_8);
            } else {
                F::Vec4_Copy(c, fp.wrapping_add(32), fp);
            }
            { let v_ = F::EftRibbon_Create(c, fp.wrapping_add(32)); let a_ = (p_handles_2).wrapping_add((((l_n_16) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_16) as u32)).wrapping_mul(4))); F::EftRibbon_SetUnkD8(c, a0_, p_arg5_6) }; };
            { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_16) as u32)).wrapping_mul(4))); let a1_ = rt::i2f((c.m.r8((l_part_17).wrapping_add(5))) as i32); F::EftRibbon_SetDelay(c, a0_, a1_) }; };
            { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_16) as u32)).wrapping_mul(4))); let a1_ = rt::i2f((c.m.r8((l_part_17).wrapping_add(6))) as i32); F::EftRibbon_SetFadeDelay(c, a0_, a1_) }; };
            { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_16) as u32)).wrapping_mul(4))); let a1_ = rt::i2f((c.m.r8((l_part_17).wrapping_add(7))) as i32); F::EftRibbon_SetFadeTime(c, a0_, a1_) }; };
            if ((((((c.m.r8((l_part_17).wrapping_add(52))) as i32)) & (2i32))) != 0) {
                { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_16) as u32)).wrapping_mul(4))); let a1_ = ((c.m.r8((l_part_17).wrapping_add(3))) as i32); F::EftRibbon_SetMaxNodes(c, a0_, a1_) }; };
            }
            { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_16) as u32)).wrapping_mul(4))); F::EftEmit_TagTask(c, a0_, p_chr_5, p_type_4) };
        }
        if (((((c.m.r32((p_handles_2).wrapping_add((((l_n_16) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
            l_warp_21 = 0i32;
            if ((((p_flags_3) & (16i32))) != 0) {
                if ((((!((((p_flags_3) & (32i32))) != 0)) as i32)) != 0) {
                    { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_16) as u32)).wrapping_mul(4))); F::EftRibbon_SetPos(c, a0_, fp) }; };
                } else {
                    l_warp_21 = 1i32;
                }
                { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_16) as u32)).wrapping_mul(4))); F::EftRibbon_SetEnd(c, a0_, fp.wrapping_add(16), l_warp_21) }; };
            } else {
                if ((((p_flags_3) & (512i32))) != 0) {
                    { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_16) as u32)).wrapping_mul(4))); F::EftRibbon_SetEnd(c, a0_, fp.wrapping_add(16), 0i32) }; };
                }
            }
            if ((((p_flags_3) & (64i32))) != 0) {
                { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_16) as u32)).wrapping_mul(4))); F::EftRibbon_SetSize(c, a0_, p_size_11) }; };
            }
            if ((((p_flags_3) & (8i32))) != 0) {
                { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_16) as u32)).wrapping_mul(4))); F::EftRibbon_Kill(c, a0_) }; };
                { let v_ = ((0i32) as u32); let a_ = (p_handles_2).wrapping_add((((l_n_16) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            } else {
                if ((((p_flags_3) & (2i32))) != 0) {
                    { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_16) as u32)).wrapping_mul(4))); F::EftRibbon_Stop(c, a0_) }; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_SpawnType17", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_SpawnType18(c: &mut Ctx, mut p_set_1: u32, mut p_handles_2: u32, mut p_flags_3: i32, mut p_type_4: i32, mut p_chr_5: i32, mut p_arg5_6: i32, mut p_idx_7: i32, mut p_pos_8: u32, mut p_dir_9: u32, mut p_size_10: F32, mut p_scale_11: F32, mut p_rate_12: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_SpawnType18", &PROBE_N, &[(p_set_1) as u32, (p_handles_2) as u32, (p_flags_3) as u32, (p_type_4) as u32, (p_chr_5) as u32, (p_arg5_6) as u32, (p_idx_7) as u32, (p_pos_8) as u32, (p_dir_9) as u32, (p_size_10).0, (p_scale_11).0, (p_rate_12).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_g_13: u32 = 0;
    let mut l_n_14: i32 = 0;
    let mut l_part_15: u32 = 0;
    let mut l_res_16: u32 = 0;
    let mut l_texA_17: u32 = 0;
    let mut l_texB_18: u32 = 0;
    let r_ = 'f: {
        l_g_13 = ((p_set_1).wrapping_add(12)).wrapping_add((((18i32) as u32)).wrapping_mul(16));
        l_n_14 = (((c.m.r8((l_g_13).wrapping_add(8))) as i32)).wrapping_add(p_idx_7);
        l_part_15 = (c.m.r32((p_set_1).wrapping_add(8))).wrapping_add((((l_n_14) as u32)).wrapping_mul(64));
        { let a3_ = rt::fmul(F32(c.m.r32((l_part_15).wrapping_add(16))), p_scale_11); F::Vec3_ScaleAdd(c, fp, p_dir_9, p_pos_8, a3_) };
        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        if ((((p_flags_3) & (1i32))) != 0) {
            if (((((c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4)))) == (((0i32) as u32))) as i32)) != 0) {
                l_res_16 = (c.m.r32((((p_set_1).wrapping_add(12)).wrapping_add(752)).wrapping_add((((1i32) as u32)).wrapping_mul(4)))).wrapping_add(((((((c.m.r8((l_g_13).wrapping_add(12))) as i32)).wrapping_add(((c.m.r8((l_part_15).wrapping_add(1))) as i32))).wrapping_mul(264i32)) as u32));
                l_texA_17 = { let a_ = ((p_set_1).wrapping_add(12)).wrapping_add((((((54i32).wrapping_add(((c.m.r8((l_g_13).wrapping_add(9))) as i32))).wrapping_add(p_idx_7)) as u32)).wrapping_mul(8)); c.m.r32(a_) };
                l_texB_18 = { let a_ = (((p_set_1).wrapping_add(12)).wrapping_add((((((54i32).wrapping_add(((c.m.r8((l_g_13).wrapping_add(9))) as i32))).wrapping_add(p_idx_7)) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.r32(a_) };
                rt::memset(c, fp.wrapping_add(16), 0, 64);
                rt::memset(c, fp.wrapping_add(16).wrapping_add(0), 0, 16);
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(0), (v_).0); }
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(4), (v_).0); }
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(8), (v_).0); }
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(12), (v_).0); }
                rt::memset(c, fp.wrapping_add(16).wrapping_add(16), 0, 16);
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(0), (v_).0); }
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(4), (v_).0); }
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(8), (v_).0); }
                { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(12), (v_).0); }
                { let v_ = p_chr_5; c.m.w32(fp.wrapping_add(16).wrapping_add(32), (v_) as u32); }
                { let v_ = ((c.m.r8((l_part_15).wrapping_add(2))) as i32); c.m.w32(fp.wrapping_add(16).wrapping_add(36), (v_) as u32); }
                { let v_ = p_rate_12; c.m.w32(fp.wrapping_add(16).wrapping_add(40), (v_).0); }
                { let v_ = p_size_10; c.m.w32(fp.wrapping_add(16).wrapping_add(44), (v_).0); }
                { let v_ = l_res_16; c.m.w32(fp.wrapping_add(16).wrapping_add(48), v_); }
                { let v_ = l_texA_17; c.m.w32(fp.wrapping_add(16).wrapping_add(52), v_); }
                { let v_ = l_texB_18; c.m.w32(fp.wrapping_add(16).wrapping_add(56), v_); }
                F::Vec4_Copy(c, (fp.wrapping_add(16)).wrapping_add(16), p_dir_9);
                if ((((((c.m.r8(l_part_15)) as i32)) & (64i32))) != 0) {
                    F::Vec4_Copy(c, fp.wrapping_add(16), p_pos_8);
                } else {
                    F::Vec4_Copy(c, fp.wrapping_add(16), fp);
                }
                { let v_ = F::EftChain_Create(c, fp.wrapping_add(16)); let a_ = (p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); let a1_ = ((c.m.r8((l_part_15).wrapping_add(3))) as i32); F::EftChain_SetParam3(c, a0_, a1_) }; };
                { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); let a1_ = rt::i2f((c.m.r8((l_part_15).wrapping_add(5))) as i32); F::EftChain_SetParam5(c, a0_, a1_) }; };
                { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); let a1_ = rt::i2f((c.m.r8((l_part_15).wrapping_add(6))) as i32); F::EftChain_SetParam6(c, a0_, a1_) }; };
                if ((((((c.m.r8(l_part_15)) as i32)) & (32i32))) != 0) {
                    { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); F::EftChain_SetViewOnly(c, a0_) }; };
                }
                { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); F::EftChain_SetType(c, a0_, p_type_4) }; };
                { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); F::EftEmit_TagTask(c, a0_, p_chr_5, p_type_4) };
            }
        }
        if (((((c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
            if ((((p_flags_3) & (16i32))) != 0) {
                if ((((p_flags_3) & (32i32))) != 0) {
                    { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); F::EftChain_Warp(c, a0_, fp) }; };
                } else {
                    { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); F::EftChain_SetPos(c, a0_, fp) }; };
                }
            }
            if ((((p_flags_3) & (64i32))) != 0) {
                { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); F::EftChain_SetSize(c, a0_, p_size_10) }; };
            }
            if ((((p_flags_3) & (128i32))) != 0) {
                { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); F::EftChain_SetDir(c, a0_, p_dir_9) }; };
            }
            if ((((p_flags_3) & (8i32))) != 0) {
                { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); F::EftChain_Kill(c, a0_) }; };
                { let v_ = ((0i32) as u32); let a_ = (p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            } else {
                if ((((p_flags_3) & (2i32))) != 0) {
                    if ((((p_flags_3) & (4i32))) != 0) {
                        { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); F::EftChain_SetParam6(c, a0_, F32(0x00000000)) }; };
                    }
                    { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_14) as u32)).wrapping_mul(4))); F::EftChain_Stop(c, a0_) }; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_SpawnType18", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_SpawnType14(c: &mut Ctx, mut p_set_1: u32, mut p_handles_2: u32, mut p_flags_3: i32, mut p_type_4: i32, mut p_chr_5: i32, mut p_idx_6: i32, mut p_pos_7: u32, mut p_dir_8: u32, mut p_size_9: F32, mut p_scale_10: F32, mut p_rate_11: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_SpawnType14", &PROBE_N, &[(p_set_1) as u32, (p_handles_2) as u32, (p_flags_3) as u32, (p_type_4) as u32, (p_chr_5) as u32, (p_idx_6) as u32, (p_pos_7) as u32, (p_dir_8) as u32, (p_size_9).0, (p_scale_10).0, (p_rate_11).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_g_12: u32 = 0;
    let mut l_n_13: i32 = 0;
    let mut l_part_14: u32 = 0;
    let mut l_res_15: u32 = 0;
    let mut l_tex_16: u32 = 0;
    let r_ = 'f: {
        l_g_12 = ((p_set_1).wrapping_add(12)).wrapping_add((((14i32) as u32)).wrapping_mul(16));
        l_n_13 = (((c.m.r8((l_g_12).wrapping_add(8))) as i32)).wrapping_add(p_idx_6);
        l_part_14 = (c.m.r32((p_set_1).wrapping_add(8))).wrapping_add((((l_n_13) as u32)).wrapping_mul(64));
        { let a3_ = rt::fmul(F32(c.m.r32((l_part_14).wrapping_add(16))), p_scale_10); F::Vec3_ScaleAdd(c, fp, p_dir_8, p_pos_7, a3_) };
        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        if ((((p_flags_3) & (1i32))) != 0) {
            if (((((c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4)))) == (((0i32) as u32))) as i32)) != 0) {
                l_res_15 = c.m.r32((((p_set_1).wrapping_add(12)).wrapping_add(752)).wrapping_add((((0i32) as u32)).wrapping_mul(4)));
                { let r_ = ((((c.m.r8((l_g_12).wrapping_add(12))) as i32)).wrapping_add(((c.m.r8((l_part_14).wrapping_add(1))) as i32))).wrapping_mul(520i32); let n_ = (l_res_15).wrapping_add(((r_) as u32)); l_res_15 = n_; };
                l_tex_16 = { let a_ = (((p_set_1).wrapping_add(12)).wrapping_add(432)).wrapping_add(((((((c.m.r8((l_g_12).wrapping_add(9))) as i32)).wrapping_add(p_idx_6)) as u32)).wrapping_mul(8)); c.m.r32(a_) };
                {
                    rt::memset(c, fp.wrapping_add(16), 0, 64);
                    { let v_ = p_chr_5; c.m.w32(fp.wrapping_add(16).wrapping_add(0), (v_) as u32); }
                    { let v_ = ((c.m.r8((l_part_14).wrapping_add(4))) as i32); c.m.w32(fp.wrapping_add(16).wrapping_add(4), (v_) as u32); }
                    { let v_ = p_rate_11; c.m.w32(fp.wrapping_add(16).wrapping_add(8), (v_).0); }
                    rt::memset(c, fp.wrapping_add(16).wrapping_add(16), 0, 16);
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(0), (v_).0); }
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(4), (v_).0); }
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(8), (v_).0); }
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(12), (v_).0); }
                    rt::memset(c, fp.wrapping_add(16).wrapping_add(32), 0, 16);
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(32).wrapping_add(0), (v_).0); }
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(32).wrapping_add(4), (v_).0); }
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(32).wrapping_add(8), (v_).0); }
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(32).wrapping_add(12), (v_).0); }
                    { let v_ = p_size_9; c.m.w32(fp.wrapping_add(16).wrapping_add(48), (v_).0); }
                    { let v_ = l_tex_16; c.m.w32(fp.wrapping_add(16).wrapping_add(52), v_); }
                    { let v_ = l_res_15; c.m.w32(fp.wrapping_add(16).wrapping_add(56), v_); }
                    if ((((((c.m.r8(l_part_14)) as i32)) & (64i32))) != 0) {
                        F::Vec4_Copy(c, (fp.wrapping_add(16)).wrapping_add(16), p_pos_7);
                    } else {
                        F::Vec4_Copy(c, (fp.wrapping_add(16)).wrapping_add(16), fp);
                    }
                    F::Vec4_Copy(c, (fp.wrapping_add(16)).wrapping_add(32), p_dir_8);
                    { let v_ = F::EftAnimPart_Create(c, fp.wrapping_add(16)); let a_ = (p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); let a1_ = rt::i2f((c.m.r8((l_part_14).wrapping_add(5))) as i32); F::EftAnimPart_SetDelay(c, a0_, a1_) }; };
                    { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); let a1_ = rt::i2f((c.m.r8((l_part_14).wrapping_add(6))) as i32); F::EftAnimPart_SetHold(c, a0_, a1_) }; };
                    { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); let a1_ = rt::i2f((c.m.r8((l_part_14).wrapping_add(7))) as i32); F::EftAnimPart_SetFade(c, a0_, a1_) }; };
                    { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftAnimPart_SetType(c, a0_, p_type_4) }; };
                    { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftEmit_TagTask(c, a0_, p_chr_5, p_type_4) };
                }
            }
        }
        if (((((c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
            if ((((p_flags_3) & (16i32))) != 0) {
                { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftAnimPart_SetPos(c, a0_, fp) }; };
            }
            if ((((p_flags_3) & (64i32))) != 0) {
                { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftAnimPart_SetSize(c, a0_, p_size_9) }; };
            }
            if ((((p_flags_3) & (128i32))) != 0) {
                { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftAnimPart_SetDir(c, a0_, p_dir_8) }; };
            }
            if ((((p_flags_3) & (8i32))) != 0) {
                { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftAnimPart_Kill(c, a0_) }; };
                { let v_ = ((0i32) as u32); let a_ = (p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            } else {
                if ((((p_flags_3) & (2i32))) != 0) {
                    if ((((p_flags_3) & (4i32))) != 0) {
                        { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftAnimPart_SetHold(c, a0_, F32(0x00000000)) }; };
                    }
                    { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftAnimPart_Stop(c, a0_) }; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_SpawnType14", &PROBE_N, &[], true); }
    r_
}

pub fn EftEmit_SpawnType5(c: &mut Ctx, mut p_set_1: u32, mut p_handles_2: u32, mut p_flags_3: i32, mut p_type_4: i32, mut p_chr_5: i32, mut p_idx_6: i32, mut p_pos_7: u32, mut p_dir_8: u32, mut p_size_9: F32, mut p_scale_10: F32, mut p_rate_11: F32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_SpawnType5", &PROBE_N, &[(p_set_1) as u32, (p_handles_2) as u32, (p_flags_3) as u32, (p_type_4) as u32, (p_chr_5) as u32, (p_idx_6) as u32, (p_pos_7) as u32, (p_dir_8) as u32, (p_size_9).0, (p_scale_10).0, (p_rate_11).0], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_g_12: u32 = 0;
    let mut l_n_13: i32 = 0;
    let mut l_part_14: u32 = 0;
    let mut l_res_15: u32 = 0;
    let mut l_ri_16: i32 = 0;
    let mut l_texA_17: u32 = 0;
    let mut l_texB_18: u32 = 0;
    let r_ = 'f: {
        l_g_12 = ((p_set_1).wrapping_add(12)).wrapping_add((((5i32) as u32)).wrapping_mul(16));
        l_n_13 = (((c.m.r8((l_g_12).wrapping_add(8))) as i32)).wrapping_add(p_idx_6);
        l_part_14 = (c.m.r32((p_set_1).wrapping_add(8))).wrapping_add((((l_n_13) as u32)).wrapping_mul(64));
        { let a3_ = rt::fmul(F32(c.m.r32((l_part_14).wrapping_add(16))), p_scale_10); F::Vec3_ScaleAdd(c, fp, p_dir_8, p_pos_7, a3_) };
        { let v_ = F32(0x3f800000); let a_ = (fp).wrapping_add(12); c.m.w32(a_, (v_).0); };
        if ((((p_flags_3) & (1i32))) != 0) {
            if (((((c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4)))) == (((0i32) as u32))) as i32)) != 0) {
                l_res_15 = c.m.r32((((p_set_1).wrapping_add(12)).wrapping_add(752)).wrapping_add((((1i32) as u32)).wrapping_mul(4)));
                l_ri_16 = (((c.m.r8((l_g_12).wrapping_add(12))) as i32)).wrapping_add(((c.m.r8((l_part_14).wrapping_add(1))) as i32));
                l_texA_17 = { let a_ = ((p_set_1).wrapping_add(12)).wrapping_add((((((54i32).wrapping_add(((c.m.r8((l_g_12).wrapping_add(9))) as i32))).wrapping_add(p_idx_6)) as u32)).wrapping_mul(8)); c.m.r32(a_) };
                { let r_ = (l_ri_16).wrapping_mul(264i32); let n_ = (l_res_15).wrapping_add(((r_) as u32)); l_res_15 = n_; };
                l_texB_18 = { let a_ = (((p_set_1).wrapping_add(12)).wrapping_add((((((54i32).wrapping_add(((c.m.r8((l_g_12).wrapping_add(9))) as i32))).wrapping_add(p_idx_6)) as u32)).wrapping_mul(8))).wrapping_add(4); c.m.r32(a_) };
                {
                    rt::memset(c, fp.wrapping_add(16), 0, 64);
                    rt::memset(c, fp.wrapping_add(16).wrapping_add(0), 0, 16);
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(0), (v_).0); }
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(4), (v_).0); }
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(8), (v_).0); }
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(0).wrapping_add(12), (v_).0); }
                    rt::memset(c, fp.wrapping_add(16).wrapping_add(16), 0, 16);
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(0), (v_).0); }
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(4), (v_).0); }
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(8), (v_).0); }
                    { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(16).wrapping_add(16).wrapping_add(12), (v_).0); }
                    { let v_ = p_chr_5; c.m.w32(fp.wrapping_add(16).wrapping_add(32), (v_) as u32); }
                    { let v_ = ((c.m.r8((l_part_14).wrapping_add(2))) as i32); c.m.w32(fp.wrapping_add(16).wrapping_add(36), (v_) as u32); }
                    { let v_ = p_rate_11; c.m.w32(fp.wrapping_add(16).wrapping_add(40), (v_).0); }
                    { let v_ = p_size_9; c.m.w32(fp.wrapping_add(16).wrapping_add(44), (v_).0); }
                    { let v_ = l_res_15; c.m.w32(fp.wrapping_add(16).wrapping_add(48), v_); }
                    { let v_ = l_texA_17; c.m.w32(fp.wrapping_add(16).wrapping_add(52), v_); }
                    { let v_ = l_texB_18; c.m.w32(fp.wrapping_add(16).wrapping_add(56), v_); }
                    if ((((((c.m.r8(l_part_14)) as i32)) & (64i32))) != 0) {
                        F::Vec4_Copy(c, fp.wrapping_add(16), p_pos_7);
                    } else {
                        F::Vec4_Copy(c, fp.wrapping_add(16), fp);
                    }
                    F::Vec4_Copy(c, (fp.wrapping_add(16)).wrapping_add(16), p_dir_8);
                    { let v_ = F::EftPtcl_Create(c, fp.wrapping_add(16)); let a_ = (p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); let a1_ = rt::i2f((c.m.r8((l_part_14).wrapping_add(5))) as i32); F::EftPtcl_SetStartDelay(c, a0_, a1_) }; };
                    { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); let a1_ = rt::i2f((c.m.r8((l_part_14).wrapping_add(6))) as i32); F::EftPtcl_SetStopDelay(c, a0_, a1_) }; };
                    { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); let a1_ = rt::i2f((c.m.r8((l_part_14).wrapping_add(7))) as i32); F::EftPtcl_SetLinger(c, a0_, a1_) }; };
                    if ((((((c.m.r8(l_part_14)) as i32)) & (32i32))) != 0) {
                        { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftPtcl_SetFront(c, a0_) }; };
                    }
                    if ((((((c.m.r8((l_part_14).wrapping_add(52))) as i32)) & (8i32))) != 0) {
                        { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftPtcl_SetFlag40(c, a0_, 1i32) }; };
                    }
                    { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftPtcl_SetType(c, a0_, p_type_4) }; };
                    { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftEmit_TagTask(c, a0_, p_chr_5, p_type_4) };
                }
            }
        }
        if (((((c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4)))) != (((0i32) as u32))) as i32)) != 0) {
            if ((((p_flags_3) & (16i32))) != 0) {
                if ((((p_flags_3) & (32i32))) != 0) {
                    { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftPtcl_Warp(c, a0_, fp) }; };
                } else {
                    { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftPtcl_SetPos(c, a0_, fp) }; };
                }
            }
            if ((((p_flags_3) & (64i32))) != 0) {
                { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftPtcl_SetSize(c, a0_, p_size_9) }; };
            }
            if ((((p_flags_3) & (128i32))) != 0) {
                { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftPtcl_SetDir(c, a0_, p_dir_8) }; };
            }
            if ((((p_flags_3) & (8i32))) != 0) {
                { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftPtcl_Kill(c, a0_) }; };
                { let v_ = ((0i32) as u32); let a_ = (p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            } else {
                if ((((p_flags_3) & (2i32))) != 0) {
                    if ((((p_flags_3) & (4i32))) != 0) {
                        { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftPtcl_SetStopDelay(c, a0_, F32(0x00000000)) }; };
                    }
                    { let _ = { let a0_ = c.m.r32((p_handles_2).wrapping_add((((l_n_13) as u32)).wrapping_mul(4))); F::EftPtcl_Stop(c, a0_) }; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "EftEmit_SpawnType5", &PROBE_N, &[], true); }
    r_
}
