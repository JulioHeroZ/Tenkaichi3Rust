#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn DiscFusion_UpdateHave(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DiscFusion_UpdateHave", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1668); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
        { let v_ = 1i32; let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(604)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    } else {
        { let v_ = 0i32; let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(604)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    }
    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1668); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
        { let v_ = 1i32; let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(604)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    } else {
        { let v_ = 0i32; let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(604)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DiscFusion_UpdateHave", &PROBE_N, &[], true); }
    r_
}

pub fn DiscFusion_Identify(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DiscFusion_Identify", &PROBE_N, &[], false); }
    let mut l_result_1: i32 = 0;
    let r_ = 'f: {
    's1: {
        let sv2: i32 = F::Disc_Identify(c);
        match sv2 {
            0i32 => {
                l_result_1 = 0i32;
                break 's1;
            }
            1i32 => {
                l_result_1 = 1i32;
                break 's1;
            }
            2i32 => {
                l_result_1 = 2i32;
                break 's1;
            }
            (-2i32) => {
                l_result_1 = 99i32;
                break 's1;
            }
            _ => {
                l_result_1 = (1i32).wrapping_neg();
                break 's1;
            }
        }
    }
    break 'f l_result_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "DiscFusion_Identify", &PROBE_N, &[], true); }
    r_
}

pub fn DiscFusion_Init(c: &mut Ctx, mut p_section_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DiscFusion_Init", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = F::Heap_Alloc(c, ((644u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gDiscFusion; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gDiscFusion); F::memset(c, a0_, 0i32, 644u32) }; };
        { let v_ = (c.m.r32(sym::gMenuArc3)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc3)).wrapping_add((((p_section_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = c.m.r32(sym::gDiscFusion); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gDiscFusion); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = c.m.r32(fp); let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(108); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(112)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(112)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(112)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(112)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(112)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(112)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((7i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(112)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((8i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(112)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((9i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(112)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((10i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(112)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((11i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(112)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((13i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(112)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((14i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(112)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((15i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(112)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((16i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(112)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((17i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(112)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((18i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(112)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(112)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(184)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(184)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(184)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(184)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(112); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let a0_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(184); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = rt::rem_i32(F::Rand_Libc(c), 32i32); let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(528)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = ({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = c.m.r32(fp); F::IconWin_Init(c, a0_, a1_) };
        F::IconWin_Open(c);
        { let a0_ = ({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = { let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(8); c.m.r32(a_) }; let a3_ = (((c.m.r32(sym::gDiscFusion)).wrapping_add(544)) as i32); let _ = a3_; F::MsgWin_Init(c, a0_, a1_, 0i32) };
        F::MsgWin_Open(c);
        { let v_ = ({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(16); c.m.w32(a_, v_); };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = { let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(16); c.m.r32(a_) }; F::Dialog_Init(c, a0_, a1_, 0i32) };
        {
            l_i_2 = 0i32;
            'l3: loop {
                if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let a0_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(200)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); let a1_ = { let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(16); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, ((0i32) as u32)) };
                    { let a0_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(200)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); F::TextBox_SetUnk50(c, a0_, 0i32) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        F::DiscFusion_UpdateHave(c);
        { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(520); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(616); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(512); c.m.w32(a_, (v_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DiscFusion_Init", &PROBE_N, &[], true); }
    r_
}

pub fn DiscFusion_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DiscFusion_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    F::MsgWin_Term(c);
    F::IconWin_Term(c);
    F::Dialog_Term(c);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(20)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Destroy(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gDiscFusion)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gDiscFusion); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gDiscFusion; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DiscFusion_Term", &PROBE_N, &[], true); }
    r_
}

pub fn DiscFusion_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DiscFusion_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_flash_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_x_3: i32 = 0;
    let mut l_y_4: i32 = 0;
    let r_ = 'f: {
        { let a0_ = { let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(108); c.m.r32(a_) }; F::Sprite_DrawPicture(c, a0_, 0i32, 0i32, 128i32) };
        l_flash_1 = ((c.m.r32(sym::gDiscFusion)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        F::Flash_FindLabel(c, l_flash_1, sym::menu_o_b__str0, sym::menu_o_b__str1, fp);
        { let a2_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(528)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F::FlashAnim_Blink(c, l_flash_1, fp, a2_, 0i32) };
        F::Flash_FindLabel(c, l_flash_1, sym::menu_o_b__str2, sym::menu_o_b__str3, fp);
        { let a2_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(536)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F::FlashAnim_Talk(c, l_flash_1, fp, a2_, 0i32) };
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_o_b__str4, &[((l_i_2) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_o_b__str5, fp);
                    if (({ let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(604)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != 0) {
                        { let a5_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(200)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, (l_i_2).wrapping_add(348i32), a5_) };
                    } else {
                        { let a5_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(200)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, (l_i_2).wrapping_add(346i32), a5_) };
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        l_x_3 = 0i32;
        l_y_4 = 0i32;
        's3: {
            let sv5: i32 = { let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(484)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(500); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
            match sv5 {
                0i32 => {
                    if (({ let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(604)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != 0) {
                        l_x_3 = 0i32;
                        l_y_4 = 64i32;
                    } else {
                        l_x_3 = 0i32;
                        l_x_3 = ({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(628); (c.m.r32(a_) as i32) }).wrapping_mul(64i32);
                    }
                    break 's3;
                }
                1i32 => {
                    if (({ let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(604)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != 0) {
                        l_x_3 = 0i32;
                        l_y_4 = 128i32;
                    } else {
                        l_x_3 = 0i32;
                        l_x_3 = ({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(628); (c.m.r32(a_) as i32) }).wrapping_mul(64i32);
                    }
                    break 's3;
                }
                _ => {
                }
            }
        }
        { let v_ = l_x_3; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = l_y_4; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = (l_x_3).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = (l_y_4).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_o_b__str6, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        { let r_ = 1i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(628); let n_ = (((c.m.r32(a_) as i32)) ^ (r_)); c.m.w32(a_, (n_) as u32); };
        { let a0_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
        F::IconWin_Draw(c);
        { let a2_ = { let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(520); (c.m.r32(a_) as i32) }; F::MsgWin_Draw(c, 0i32, 0i32, a2_) };
        F::Dialog_Draw(c, 1i32);
        { let a0_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DiscFusion_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn DiscFusion_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DiscFusion_Update", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(488); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
        { let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(488); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    }
    if (((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(480); (c.m.r32(a_) as i32) }) & (8i32))) != 0) {
        if ((((((((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(516); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) && ((((((((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(520); (c.m.r32(a_) as i32) }) == (64i32)) as i32)) != 0) || ((((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(520); (c.m.r32(a_) as i32) }) == (65i32)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
            if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                { let v_ = ({ let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(484)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(500); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(3i32); let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(504); c.m.w32(a_, (v_) as u32); };
            }
        }
    }
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(20)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Advance(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DiscFusion_Update", &PROBE_N, &[], true); }
    r_
}

pub fn DiscFusion_UpdateDisc(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DiscFusion_UpdateDisc", &PROBE_N, &[], false); }
    let mut l_state_1: i32 = 0;
    let mut l_paused_2: i32 = 0;
    let mut l_open_3: i32 = 0;
    let mut l_noType_4: i32 = 0;
    let mut l_stopped_5: i32 = 0;
    let mut l_answer_6: i32 = 0;
    let mut l_r_7: i32 = 0;
    let mut l_r_8: i32 = 0;
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(516); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        break 'f;
    }
    l_state_1 = F::Disc_GetDriveState(c);
    l_stopped_5 = (((l_state_1) == (0i32)) as i32);
    l_open_3 = (((l_state_1) == (1i32)) as i32);
    l_paused_2 = (((l_state_1) == (4i32)) as i32);
    l_noType_4 = (((l_state_1) == (7i32)) as i32);
    's1: {
        let sv9: i32 = { let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(516); (c.m.r32(a_) as i32) };
        match sv9 {
            1i32 => {
                F::Dialog_SetChoices(c, 1i32);
                F::Dialog_SetCursor(c, 1i32);
                F::Dialog_Start(c, 0i32);
                { let v_ = 2i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(516); c.m.w32(a_, (v_) as u32); };
                F::Voice_FadeOutStep(c, 0i32);
                F::Bgm_FadeOutStep(c);
                break 's1;
            }
            2i32 => {
                { let a0_ = ({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(612); (c.m.r32(a_) as i32) }).wrapping_add(20i32); F::Dialog_SetMsg(c, a0_) };
                l_answer_6 = F::Dialog_Input(c, 0i32);
                if (((((l_answer_6) < (0i32)) as i32)) != 0) {
                    { let v_ = 14i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(516); c.m.w32(a_, (v_) as u32); };
                    F::Dialog_Start(c, 1i32);
                } else {
                    if (((((l_answer_6) > (0i32)) as i32)) != 0) {
                        F::Dialog_SetChoices(c, 0i32);
                        { let v_ = 3i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(516); c.m.w32(a_, (v_) as u32); };
                    }
                }
                break 's1;
            }
            3i32 => {
                F::Dialog_SetMsg(c, 22i32);
                if ((l_open_3) != 0) {
                    { let v_ = 4i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(516); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            4i32 => {
                { let a0_ = ({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(612); (c.m.r32(a_) as i32) }).wrapping_add(23i32); F::Dialog_SetMsg(c, a0_) };
                if ((l_stopped_5) != 0) {
                    { let v_ = 5i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(516); c.m.w32(a_, (v_) as u32); };
                    F::Dialog_SetMsg(c, 25i32);
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(620); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 240i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(636); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            5i32 => {
                if ((l_paused_2) != 0) {
                    if ((((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(632); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                        l_r_7 = F::DiscFusion_Identify(c);
                        if (((((l_r_7) == (99i32)) as i32)) != 0) {
                            break 's1;
                        }
                        if ((((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(612); (c.m.r32(a_) as i32) }) == (l_r_7)) as i32)) != 0) {
                            { let v_ = 6i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(516); c.m.w32(a_, (v_) as u32); };
                            F::Dialog_Start(c, 1i32);
                            break 's1;
                        }
                        if ((((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(620); (c.m.r32(a_) as i32) }) == (64i32)) as i32)) != 0) {
                            F::Dialog_SetChoices(c, 1i32);
                            F::Dialog_SetCursor(c, 1i32);
                            { let v_ = 11i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(516); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(640); c.m.w32(a_, (v_) as u32); };
                            break 's1;
                        }
                        { let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(620); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let v_ = 3i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(632); c.m.w32(a_, (v_) as u32); };
                    } else {
                        { let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(632); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    }
                }
                if ((l_open_3) != 0) {
                    { let v_ = 4i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(516); c.m.w32(a_, (v_) as u32); };
                } else {
                    if ((l_noType_4) != 0) {
                        if ((((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(636); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                            F::Dialog_SetChoices(c, 1i32);
                            F::Dialog_SetCursor(c, 1i32);
                            { let v_ = 11i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(516); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 1i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(640); c.m.w32(a_, (v_) as u32); };
                        } else {
                            { let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(636); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                        }
                    }
                }
                break 's1;
            }
            6i32 => {
                if ((((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(612); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    { let r_ = 1i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1668); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                } else {
                    { let r_ = 2i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1668); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
                F::DiscFusion_UpdateHave(c);
                { let a0_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_o_b__str7, ((((1i32) as i64 as u64)) as u8)) };
                { let v_ = 7i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(516); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            7i32 => {
                if (((({ let a_ = (((c.m.r32(sym::gDiscFusion)).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0) {
                    F::Dialog_Start(c, 0i32);
                    F::Dialog_SetMsg(c, 27i32);
                    { let v_ = 8i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(516); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            8i32 => {
                if ((l_open_3) != 0) {
                    { let v_ = 9i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(516); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            9i32 => {
                F::Dialog_SetMsg(c, 28i32);
                if ((l_stopped_5) != 0) {
                    { let v_ = 10i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(516); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(620); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 240i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(636); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            10i32 => {
                F::Dialog_SetMsg(c, 25i32);
                if ((l_paused_2) != 0) {
                    if ((((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(632); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                        l_r_8 = F::DiscFusion_Identify(c);
                        if (((((l_r_8) == (99i32)) as i32)) != 0) {
                            break 's1;
                        }
                        if (((((l_r_8) == (2i32)) as i32)) != 0) {
                            { let v_ = 14i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(516); c.m.w32(a_, (v_) as u32); };
                            F::Dialog_Start(c, 1i32);
                            break 's1;
                        }
                        if ((((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(620); (c.m.r32(a_) as i32) }) == (64i32)) as i32)) != 0) {
                            { let v_ = 13i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(516); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(640); c.m.w32(a_, (v_) as u32); };
                            break 's1;
                        }
                        { let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(620); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        { let v_ = 3i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(632); c.m.w32(a_, (v_) as u32); };
                    } else {
                        { let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(632); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    }
                }
                if ((l_open_3) != 0) {
                    { let v_ = 9i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(516); c.m.w32(a_, (v_) as u32); };
                } else {
                    if ((l_noType_4) != 0) {
                        if ((((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(636); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                            { let v_ = 13i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(516); c.m.w32(a_, (v_) as u32); };
                            { let v_ = 1i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(640); c.m.w32(a_, (v_) as u32); };
                        } else {
                            { let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(636); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                        }
                    }
                }
                break 's1;
            }
            11i32 => {
                if (({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(640); (c.m.r32(a_) as i32) }) != 0) {
                    F::Dialog_SetMsg(c, 30i32);
                } else {
                    F::Dialog_SetMsg(c, 26i32);
                }
                l_answer_6 = F::Dialog_Input(c, 0i32);
                if (((((l_answer_6) < (0i32)) as i32)) != 0) {
                    { let v_ = 3i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(516); c.m.w32(a_, (v_) as u32); };
                    F::Dialog_SetChoices(c, 0i32);
                } else {
                    if (((((l_answer_6) > (0i32)) as i32)) != 0) {
                        F::Dialog_SetChoices(c, 0i32);
                        { let v_ = 12i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(516); c.m.w32(a_, (v_) as u32); };
                    }
                }
                break 's1;
            }
            12i32 => {
                F::Dialog_SetMsg(c, 22i32);
                if ((l_open_3) != 0) {
                    { let v_ = 9i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(516); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            13i32 => {
                if (({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(640); (c.m.r32(a_) as i32) }) != 0) {
                    F::Dialog_SetMsg(c, 31i32);
                } else {
                    F::Dialog_SetMsg(c, 29i32);
                }
                if ((l_open_3) != 0) {
                    { let v_ = 9i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(516); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            14i32 => {
                if ((F::Dialog_IsClosed(c)) != 0) {
                    F::Adx_StopAll(c);
                    F::Bgm_Play(c, 68379i32);
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(516); c.m.w32(a_, (v_) as u32); };
                    if ((((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(612); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                        if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1668); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
                            { let v_ = 6i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(504); c.m.w32(a_, (v_) as u32); };
                        } else {
                            { let v_ = 7i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(504); c.m.w32(a_, (v_) as u32); };
                        }
                    } else {
                        if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1668); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
                            { let v_ = 6i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(504); c.m.w32(a_, (v_) as u32); };
                        } else {
                            { let v_ = 7i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(504); c.m.w32(a_, (v_) as u32); };
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
    if rt::probing() { crate::host::harness::probe(c, "DiscFusion_UpdateDisc", &PROBE_N, &[], true); }
    r_
}

pub fn DiscFusion_UpdateTalk(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DiscFusion_UpdateTalk", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(512); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        if ((((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(504); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            break 'f;
        }
        if ((((((((((((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(520); (c.m.r32(a_) as i32) }) != ((1i32).wrapping_neg())) as i32)) != 0) && (((((F::Voice_GetStat(c, 0i32)) != (5i32)) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(504); (c.m.r32(a_) as i32) }) == ({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(508); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                { let v_ = 1i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(512); c.m.w32(a_, (v_) as u32); };
            }
            break 'f;
        }
    } else {
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(512); c.m.w32(a_, (v_) as u32); };
    }
    's1: {
        let sv1: i32 = { let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(504); (c.m.r32(a_) as i32) };
        match sv1 {
            1i32 => {
                { let v_ = 64i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(520); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = 65i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(520); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            3i32 => {
                if (({ let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(604)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != 0) {
                    { let v_ = 82i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(520); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let v_ = 66i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(520); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            4i32 => {
                if (({ let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(604)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != 0) {
                    { let v_ = 83i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(520); c.m.w32(a_, (v_) as u32); };
                } else {
                    { let v_ = 67i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(520); c.m.w32(a_, (v_) as u32); };
                }
                break 's1;
            }
            5i32 => {
                { let v_ = 70i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(520); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            6i32 => {
                { let v_ = 71i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(520); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            7i32 => {
                { let v_ = 72i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(520); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(504); (c.m.r32(a_) as i32) }) == (6i32)) as i32)) != 0) {
        { let v_ = ({ let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(484)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(500); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(3i32); let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(504); c.m.w32(a_, (v_) as u32); };
    } else {
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(504); c.m.w32(a_, (v_) as u32); };
    }
    { let a0_ = { let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(520); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34661i32, a2_) };
    { let v_ = { let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(504); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(508); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "DiscFusion_UpdateTalk", &PROBE_N, &[], true); }
    r_
}

pub fn DiscFusion_Input(c: &mut Ctx, mut p_result_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DiscFusion_Input", &PROBE_N, &[(p_result_1) as u32], false); }
    let mut l_up_2: i32 = 0;
    let mut l_down_3: i32 = 0;
    let mut l_ok_4: i32 = 0;
    let mut l_cancel_5: i32 = 0;
    let r_ = 'f: {
    l_up_2 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) as i32);
    l_down_3 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) as i32);
    l_ok_4 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) as i32);
    l_cancel_5 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) as i32);
    if ((((!(((({ let a_ = (((c.m.r32(sym::gDiscFusion)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f;
    }
    if ((((!(((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(480); (c.m.r32(a_) as i32) }) & (4i32))) != 0)) as i32)) != 0) {
        if (((((({ let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(604)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != 0) && (({ let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(604)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != 0)) as i32)) != 0) {
            { let v_ = 2i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(504); c.m.w32(a_, (v_) as u32); };
        } else {
            { let v_ = 1i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(504); c.m.w32(a_, (v_) as u32); };
        }
        F::DiscFusion_PlateGoto(c, 0i32, 0i32, sym::menu_o_b__str8);
        { let r_ = 4i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(480); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    's1: {
        let sv6: i32 = { let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(500); (c.m.r32(a_) as i32) };
        match sv6 {
            0i32 => {
                if ((l_up_2) != 0) {
                    F::DiscFusion_PlateGoto(c, 0i32, 0i32, sym::menu_o_b__str9);
                    if ((((({ let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(484)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(500); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) < (0i32)) as i32)) != 0) {
                        { let v_ = 1i32; let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(484)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(500); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    F::DiscFusion_PlateGoto(c, 0i32, 0i32, sym::menu_o_b__str10);
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(624); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ({ let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(484)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(500); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(3i32); let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(504); c.m.w32(a_, (v_) as u32); };
                } else {
                    if ((l_down_3) != 0) {
                        F::DiscFusion_PlateGoto(c, 0i32, 0i32, sym::menu_o_b__str11);
                        if ((((({ let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(484)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(500); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) >= (2i32)) as i32)) != 0) {
                            { let v_ = 0i32; let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(484)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(500); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        F::DiscFusion_PlateGoto(c, 0i32, 0i32, sym::menu_o_b__str12);
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(624); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ({ let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(484)).wrapping_add(((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(500); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(3i32); let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(504); c.m.w32(a_, (v_) as u32); };
                    } else {
                        if ((l_ok_4) != 0) {
                            if (({ let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(604)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(484)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) != 0) {
                                { let r_ = 1i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(480); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                { let r_ = 2i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(480); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                { let v_ = 15i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(488); c.m.w32(a_, (v_) as u32); };
                            } else {
                                { let v_ = { let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(484)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(612); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 1i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(516); c.m.w32(a_, (v_) as u32); };
                                F::Voice_StopWithLip(c);
                            }
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(624); c.m.w32(a_, (v_) as u32); };
                        } else {
                            if ((l_cancel_5) != 0) {
                                F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                                { let v_ = 0i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
                                { let a0_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_o_b__str13, ((((1i32) as i64 as u64)) as u8)) };
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(624); c.m.w32(a_, (v_) as u32); };
                            } else {
                                { let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(624); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                if ((((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(624); (c.m.r32(a_) as i32) }) == (1800i32)) as i32)) != 0) {
                                    { let v_ = 5i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(504); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(624); c.m.w32(a_, (v_) as u32); };
                                }
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
    if rt::probing() { crate::host::harness::probe(c, "DiscFusion_Input", &PROBE_N, &[], true); }
    r_
}

pub fn DiscFusion_PlateGoto(c: &mut Ctx, mut p_movie_1: i32, mut p_level_2: i32, mut p_label_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DiscFusion_PlateGoto", &PROBE_N, &[(p_movie_1) as u32, (p_level_2) as u32, (p_label_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_flash_4: u32 = 0;
    let r_ = 'f: {
        l_flash_4 = ((c.m.r32(sym::gDiscFusion)).wrapping_add(20)).wrapping_add((((p_movie_1) as u32)).wrapping_mul(44));
        { let _ = { let a2_ = { let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(484)).wrapping_add((((p_level_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::sprintf(c, fp.wrapping_add(8), sym::menu_o_b__str14, &[((a2_) as i64 as u64)]) }; };
        F::Flash_FindLabel(c, l_flash_4, ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, l_flash_4, fp, p_label_3);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DiscFusion_PlateGoto", &PROBE_N, &[], true); }
    r_
}

pub fn DiscFusion_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "DiscFusion_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = 1i32; c.m.w32(fp, (v_) as u32); }
        F::DiscFusion_Init(c, p_section_1);
        F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
        'l1: while ((1i32) != 0) {
            F::Gfx_BeginFrame(c);
            F::Pad_Update(c);
            F::Snd_Update(c);
            F::ColorFade_Update(c);
            F::DiscFusion_Update(c);
            F::DiscFusion_UpdateDisc(c);
            F::DiscFusion_UpdateTalk(c);
            F::DiscFusion_Draw(c);
            F::ColorFade_Draw(c);
            F::Gfx_EndFrame(c, 1i32);
            F::Dma_Flush(c);
            F::File_Stub264D90(c);
            if ((F::ColorFade_IsInDone(c)) != 0) {
                if ((((((((!(((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(480); (c.m.r32(a_) as i32) }) & (8i32))) != 0)) as i32)) != 0) && (((({ let a_ = (((c.m.r32(sym::gDiscFusion)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
                    { let r_ = 8i32; let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(480); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
            if ((F::ColorFade_IsFadingOut(c)) != 0) {
                F::Voice_FadeOutStep(c, 0i32);
                F::Bgm_FadeOutStep(c);
                continue 'l1;
            }
            if ((F::ColorFade_IsOutDone(c)) != 0) {
                break 'l1;
            }
            if (((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(480); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(488); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                    { let v_ = { let a_ = ((c.m.r32(sym::gDiscFusion)).wrapping_add(484)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1600); c.m.w32(a_, (v_) as u32); };
                }
            } else {
                if ((((((((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(516); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gDiscFusion)).wrapping_add(504); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                    F::DiscFusion_Input(c, fp);
                }
            }
        }
        F::DiscFusion_Term(c);
        F::Dma_ResetBuffers(c);
        break 'f (c.m.r32(fp) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "DiscFusion_Run", &PROBE_N, &[], true); }
    r_
}
