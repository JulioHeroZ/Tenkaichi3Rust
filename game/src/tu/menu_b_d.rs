#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn HistSel_PlayVoice(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistSel_PlayVoice", &PROBE_N, &[], false); }
    let r_ = 'f: {
    's1: {
        let sv1: i32 = { let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(484); (c.m.r32(a_) as i32) };
        match sv1 {
            0i32 => {
                { let a0_ = { let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(44)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(484); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(480); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34103i32, a2_) };
                break 's1;
            }
            1i32 => {
                { let a0_ = { let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(44)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(484); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(480); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 33827i32, a2_) };
                break 's1;
            }
            2i32 => {
                { let a0_ = { let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(44)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(484); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(480); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 33716i32, a2_) };
                break 's1;
            }
            3i32 => {
                { let a0_ = { let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(44)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(484); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(480); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 33755i32, a2_) };
                break 's1;
            }
            4i32 => {
                { let a0_ = { let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(44)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(484); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(480); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 33924i32, a2_) };
                break 's1;
            }
            5i32 => {
                { let a0_ = { let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(44)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(484); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(480); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 33856i32, a2_) };
                break 's1;
            }
            6i32 => {
                { let a0_ = { let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(44)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(484); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(480); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 33794i32, a2_) };
                break 's1;
            }
            7i32 => {
                { let a0_ = { let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(44)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(484); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(480); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 33894i32, a2_) };
                break 's1;
            }
            8i32 => {
                { let a0_ = { let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(44)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(484); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(480); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34139i32, a2_) };
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HistSel_PlayVoice", &PROBE_N, &[], true); }
    r_
}

pub fn HistSel_Idle(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistSel_Idle", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(488); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    if ((((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(488); (c.m.r32(a_) as i32) }) == (3600i32)) as i32)) != 0) {
        { let v_ = 8i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(484); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
        { let v_ = (((F::Rand_Range(c, ((4i32) as u32))).wrapping_add(((10i32) as u32))) as i32); let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
        F::HistSel_PlayVoice(c);
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(488); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HistSel_Idle", &PROBE_N, &[], true); }
    r_
}

pub fn HistSel_SayItem(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistSel_SayItem", &PROBE_N, &[], false); }
    let r_ = 'f: {
    's1: {
        let sv1: i32 = { let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(500)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
        match sv1 {
            0i32 => {
                { let v_ = (((F::Rand_Range(c, ((2i32) as u32))).wrapping_add(((22i32) as u32))) as i32); let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                F::HistSel_PlayVoice(c);
                break 's1;
            }
            1i32 => {
                { let v_ = (((F::Rand_Range(c, ((2i32) as u32))).wrapping_add(((24i32) as u32))) as i32); let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                F::HistSel_PlayVoice(c);
                break 's1;
            }
            2i32 => {
                { let v_ = (((F::Rand_Range(c, ((2i32) as u32))).wrapping_add(((26i32) as u32))) as i32); let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                F::HistSel_PlayVoice(c);
                break 's1;
            }
            3i32 => {
                { let v_ = (((F::Rand_Range(c, ((2i32) as u32))).wrapping_add(((28i32) as u32))) as i32); let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                F::HistSel_PlayVoice(c);
                break 's1;
            }
            4i32 => {
                { let v_ = (((F::Rand_Range(c, ((2i32) as u32))).wrapping_add(((32i32) as u32))) as i32); let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                F::HistSel_PlayVoice(c);
                break 's1;
            }
            5i32 => {
                { let v_ = (((F::Rand_Range(c, ((2i32) as u32))).wrapping_add(((30i32) as u32))) as i32); let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                F::HistSel_PlayVoice(c);
                break 's1;
            }
            6i32 => {
                { let v_ = (((F::Rand_Range(c, ((2i32) as u32))).wrapping_add(((34i32) as u32))) as i32); let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                F::HistSel_PlayVoice(c);
                break 's1;
            }
            7i32 => {
                { let v_ = (((F::Rand_Range(c, ((2i32) as u32))).wrapping_add(((36i32) as u32))) as i32); let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                F::HistSel_PlayVoice(c);
                break 's1;
            }
            8i32 => {
                { let v_ = 47i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                F::HistSel_PlayVoice(c);
                break 's1;
            }
            _ => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                F::Voice_StopWithLip(c);
                break 's1;
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HistSel_SayItem", &PROBE_N, &[], true); }
    r_
}

pub fn HistSel_RollEvent(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistSel_RollEvent", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_slot_2: u32 = 0;
    let mut l_flags_3: i32 = 0;
    let mut l_done_4: i32 = 0;
    let r_ = 'f: {
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (8i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                l_slot_2 = ((c.m.r32(sym::gSaveData)).wrapping_add(16)).wrapping_add((((l_i_1) as u32)).wrapping_mul(16));
                l_flags_3 = (c.m.r32(l_slot_2) as i32);
                l_done_4 = ((l_flags_3) & (32i32));
                { let r_ = 4i32; let n_ = ((l_flags_3) & (r_)); l_flags_3 = n_; };
                if ((((((((((l_flags_3) != 0) && ((((!((l_done_4) != 0)) as i32)) != 0)) as i32)) != 0) && (((((((F::Rand_Range(c, ((100i32) as u32))) as i32)) < (4i32)) as i32)) != 0)) as i32)) != 0) {
                    { let v_ = l_i_1; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(556); c.m.w32(a_, (v_) as u32); };
                    break 'f;
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HistSel_RollEvent", &PROBE_N, &[], true); }
    r_
}

pub fn HistSel_Init(c: &mut Ctx, mut p_section_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistSel_Init", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_i_2: i32 = 0;
    let mut l_j_3: i32 = 0;
    let mut l_cleared_4: i32 = 0;
    let mut l_slot_5: u32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = F::Heap_Alloc(c, 596i32, ((32i32) as u32), 0i32, 2i32); let a_ = sym::gHistSel; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gHistSel); F::memset(c, a0_, 0i32, ((596i32) as u32)) }; };
        { let v_ = (c.m.r32(sym::gMenuArc2)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc2)).wrapping_add((((p_section_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = c.m.r32(sym::gHistSel); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gHistSel); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.w32(a_, v_); };
        F::StreamSe_PlayDefault(c, 0i32, 68571i32);
        { let v_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = c.m.r32(fp); let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(124); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((30i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((74i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((75i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((71i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((70i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((73i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((72i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (16i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = (({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((l_i_2).wrapping_mul(64i32)) as u32))).wrapping_add((((7i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add(((((6i32).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((31i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((32i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((33i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((36i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((44i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((40i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((43i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((42i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((37i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((76i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((79i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((7i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((78i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((8i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((81i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((39i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((82i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((34i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((35i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((41i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((45i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((38i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((8i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((77i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        {
            l_i_2 = 0i32;
            'l3: loop {
                if !(((((l_i_2) < (8i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let v_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(((((33i32).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
                    's5: {
                        let sv6: i32 = l_i_2;
                        match sv6 {
                            0i32 => {
                                { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((46i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((47i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((48i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                break 's5;
                            }
                            1i32 => {
                                { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((49i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((50i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((51i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                break 's5;
                            }
                            2i32 => {
                                { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((52i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((53i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((54i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                break 's5;
                            }
                            3i32 => {
                                { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((55i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((57i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((56i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                break 's5;
                            }
                            4i32 => {
                                { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((58i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((59i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((60i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                break 's5;
                            }
                            5i32 => {
                                { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((61i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((62i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((63i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                break 's5;
                            }
                            6i32 => {
                                { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((64i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((65i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((66i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                break 's5;
                            }
                            7i32 => {
                                { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((67i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((68i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((69i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                                break 's5;
                            }
                            _ => {
                            }
                        }
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(128)).wrapping_add((((80i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gHistSel)).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gHistSel)).wrapping_add(128); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gHistSel)).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let v_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = c.m.r32(fp); F::IconWin_Init(c, a0_, a1_) };
        F::IconWin_Open(c);
        {
            l_i_2 = 0i32;
            'l6: loop {
                if !(((((l_i_2) < (9i32)) as i32)) != 0) { break 'l6; }
                'c7: {
                    if (((((l_i_2) == (8i32)) as i32)) != 0) {
                        { let v_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(8)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        { let v_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(44)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    } else {
                        { let v_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(((((16i32).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(8)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        { let v_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add(((((25i32).wrapping_add(l_i_2)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(44)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let a0_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let _ = 0i32; F::MsgWin_Init(c, a0_, ((0i32) as u32), 1i32) };
        F::MsgWin_Open(c);
        {
            rt::memset(c, fp.wrapping_add(4), 0, 32);
            { let v_ = 3i32; c.m.w32(fp.wrapping_add(4).wrapping_add(0), (v_) as u32); }
            { let v_ = 4i32; c.m.w32(fp.wrapping_add(4).wrapping_add(4), (v_) as u32); }
            { let v_ = 7i32; c.m.w32(fp.wrapping_add(4).wrapping_add(8), (v_) as u32); }
            { let v_ = 5i32; c.m.w32(fp.wrapping_add(4).wrapping_add(12), (v_) as u32); }
            { let v_ = 16i32; c.m.w32(fp.wrapping_add(4).wrapping_add(16), (v_) as u32); }
            { let v_ = 5i32; c.m.w32(fp.wrapping_add(4).wrapping_add(20), (v_) as u32); }
            { let v_ = 4i32; c.m.w32(fp.wrapping_add(4).wrapping_add(24), (v_) as u32); }
            { let v_ = 4i32; c.m.w32(fp.wrapping_add(4).wrapping_add(28), (v_) as u32); }
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(536); c.m.w32(a_, (v_) as u32); };
            l_cleared_4 = 0i32;
            {
                l_i_2 = 0i32;
                'l8: loop {
                    if !(((((l_i_2) < (9i32)) as i32)) != 0) { break 'l8; }
                    'c9: {
                        l_slot_5 = ((c.m.r32(sym::gSaveData)).wrapping_add(16)).wrapping_add((((l_i_2) as u32)).wrapping_mul(16));
                        if (((((c.m.r32(l_slot_5) as i32)) & (1i32))) != 0) {
                            { let v_ = l_i_2; let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(500)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(536); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); o_ }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        if (((((l_i_2) < (8i32)) as i32)) != 0) {
                            {
                                l_j_3 = 0i32;
                                'l10: loop {
                                    if !(((((l_j_3) < ((c.m.r32((fp.wrapping_add(4)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32))) as i32)) != 0) { break 'l10; }
                                    'c11: {
                                        if (((({ let a_ = ((((c.m.r32(sym::gSaveData)).wrapping_add(16)).wrapping_add((((l_i_2) as u32)).wrapping_mul(16))).wrapping_add(4)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) & ((((1u32).wrapping_shl((l_j_3) as u32)) as i32)))) != 0) {
                                            { let o_ = l_cleared_4; let n_ = (o_).wrapping_add(1); l_cleared_4 = n_; };
                                        }
                                    }
                                    { let o_ = l_j_3; let n_ = (o_).wrapping_add(1); l_j_3 = n_; };
                                }
                            }
                        }
                    }
                    { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
                }
            }
        }
        {
            l_i_2 = { let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(536); (c.m.r32(a_) as i32) };
            'l12: loop {
                if !(((((l_i_2) < (3i32)) as i32)) != 0) { break 'l12; }
                'c13: {
                    { let v_ = 9i32; let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(500)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(536); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = rt::f2i(rt::fmul(rt::fdiv(rt::i2f(l_cleared_4), F32(0x42400000)), F32(0x42c80000))); let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(552); c.m.w32(a_, (v_) as u32); };
        if (((((((((l_cleared_4) != (0i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(552); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0)) as i32)) != 0) {
            { let v_ = 1i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(552); c.m.w32(a_, (v_) as u32); };
        }
        if ((((((((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(552); (c.m.r32(a_) as i32) }) == (100i32)) as i32)) != 0) && ((((!(((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(8); (c.m.r32(a_) as i32) }) & (256i32))) != 0)) as i32)) != 0)) as i32)) != 0) {
            { let r_ = 64i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(460); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        {
            l_i_2 = 0i32;
            'l14: loop {
                if !(((((l_i_2) < ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(536); (c.m.r32(a_) as i32) })) as i32)) != 0) { break 'l14; }
                'c15: {
                    if ((((({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(500)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == ({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(52); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        { let v_ = l_i_2; let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        break 'l14;
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = ({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(540); c.m.w32(a_, (v_) as u32); };
        if ((((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(540); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
            { let r_ = { let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(536); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(540); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
        }
        { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
        { let v_ = 8i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(484); c.m.w32(a_, (v_) as u32); };
        { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(556); c.m.w32(a_, (v_) as u32); };
        {
            l_i_2 = 0i32;
            'l16: loop {
                if !(((((l_i_2) < (2i32)) as i32)) != 0) { break 'l16; }
                'c17: {
                    { let v_ = ((F::Rand_Range(c, ((32i32) as u32))) as i32); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(564)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        if (((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(8); (c.m.r32(a_) as i32) }) & (128i32))) != 0) {
            F::HistSel_RollEvent(c);
            if ((((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(556); (c.m.r32(a_) as i32) }) >= (0i32)) as i32)) != 0) {
                { let r_ = 32i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(460); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
            }
        } else {
            { let r_ = 16i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(460); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        { let v_ = ((((F::Rand_Range(c, ((10i32) as u32))).wrapping_mul(((60i32) as u32))).wrapping_add(((300i32) as u32))) as i32); let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(592); c.m.w32(a_, (v_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HistSel_Init", &PROBE_N, &[], true); }
    r_
}

pub fn HistSel_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistSel_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    F::MsgWin_Term(c);
    F::IconWin_Term(c);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gHistSel)).wrapping_add(80)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Destroy(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gHistSel)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gHistSel); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gHistSel; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HistSel_Term", &PROBE_N, &[], true); }
    r_
}

pub fn HistSel_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistSel_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(176) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 176);
    let mut l_i_1: i32 = 0;
    let mut l_flash_2: u32 = 0;
    let r_ = 'f: {
        { let a0_ = { let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(124); c.m.r32(a_) }; F::Sprite_DrawPicture(c, a0_, 0i32, 0i32, 128i32) };
        l_flash_2 = ((c.m.r32(sym::gHistSel)).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        F::Flash_FindLabel(c, l_flash_2, sym::menu_b_d__str0, sym::menu_b_d__str1, fp);
        { let a2_ = ((c.m.r32(sym::gHistSel)).wrapping_add(564)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F::FlashAnim_Blink(c, l_flash_2, fp, a2_, 0i32) };
        F::Flash_FindLabel(c, l_flash_2, sym::menu_b_d__str2, sym::menu_b_d__str3, fp);
        if ((((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(560); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            { let a2_ = ((c.m.r32(sym::gHistSel)).wrapping_add(572)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); F::FlashAnim_Talk(c, l_flash_2, fp, a2_, 0i32) };
        } else {
            F::FlashAnim_ShowNext2(c, l_flash_2, fp, 0i32);
        }
        if ((((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(556); (c.m.r32(a_) as i32) }) >= (0i32)) as i32)) != 0) {
            { let _ = { let a2_ = { let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(556); (c.m.r32(a_) as i32) }; F::sprintf(c, fp.wrapping_add(28), sym::menu_b_d__str4, &[((a2_) as i64 as u64)]) }; };
            { let _ = { let a2_ = { let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(556); (c.m.r32(a_) as i32) }; F::sprintf(c, fp.wrapping_add(92), sym::menu_b_d__str5, &[((a2_) as i64 as u64)]) }; };
            F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), fp.wrapping_add(92), fp);
            { let a2_ = ((c.m.r32(sym::gHistSel)).wrapping_add(564)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); F::FlashAnim_Blink(c, l_flash_2, fp, a2_, 0i32) };
            { let _ = { let a2_ = { let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(556); (c.m.r32(a_) as i32) }; F::sprintf(c, fp.wrapping_add(92), sym::menu_b_d__str6, &[((a2_) as i64 as u64)]) }; };
            F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), fp.wrapping_add(92), fp);
            if ((((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(560); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) {
                { let a2_ = ((c.m.r32(sym::gHistSel)).wrapping_add(572)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); F::FlashAnim_Talk(c, l_flash_2, fp, a2_, 0i32) };
            } else {
                F::FlashAnim_ShowNext2(c, l_flash_2, fp, 0i32);
            }
        }
        { let a4_ = { let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(552); (c.m.r32(a_) as i32) }; F::Num_Draw(c, l_flash_2, sym::menu_b_d__str7, 0i32, 3i32, a4_, 32i32, 32i32, 0i32) };
        {
            l_i_1 = 0i32;
            'l1: loop {
                if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = (l_i_1).wrapping_mul(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    's3: {
                        let sv3: i32 = l_i_1;
                        match sv3 {
                            0i32 => {
                                F::Flash_FindLabel(c, l_flash_2, sym::menu_b_d__str8, sym::menu_b_d__str9, fp);
                                if ((((({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(500)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(540); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (9i32)) as i32)) != 0) {
                                    { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                                } else {
                                    { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                                }
                                break 's3;
                            }
                            1i32 => {
                                F::Flash_FindLabel(c, l_flash_2, sym::menu_b_d__str10, sym::menu_b_d__str11, fp);
                                if ((((({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(500)).wrapping_add((((rt::rem_i32(({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32), { let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(536); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (9i32)) as i32)) != 0) {
                                    { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                                } else {
                                    { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                                }
                                break 's3;
                            }
                            _ => {
                            }
                        }
                    }
                    F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        {
            l_i_1 = 0i32;
            'l4: loop {
                if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l4; }
                'c5: {
                    { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (l_i_1).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_b_d__str12, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_b_d__str13, fp);
                    F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                    F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_b_d__str14, fp);
                    F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        {
            l_i_1 = 0i32;
            'l6: loop {
                if !(((((l_i_1) < (3i32)) as i32)) != 0) { break 'l6; }
                'c7: {
                    { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = (l_i_1).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_b_d__str15, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_b_d__str16, fp);
                    F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                    F::Flash_FindLabel(c, l_flash_2, fp.wrapping_add(28), sym::menu_b_d__str17, fp);
                    F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        {
            l_i_1 = 0i32;
            'l8: loop {
                if !(((((l_i_1) < (3i32)) as i32)) != 0) { break 'l8; }
                'c9: {
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_b_d__str18, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(28), fp);
                    { let a2_ = { let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(500)).wrapping_add((((rt::rem_i32(({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(540); (c.m.r32(a_) as i32) }).wrapping_add(l_i_1), { let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(536); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::Flash_ClipSetTex(c, l_flash_2, fp, a2_) };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_b_d__str19, fp);
        { let a2_ = { let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(500)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(544); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::Flash_ClipSetTex(c, l_flash_2, fp, a2_) };
        {
            l_i_1 = 0i32;
            'l10: loop {
                if !(((((l_i_1) < (2i32)) as i32)) != 0) { break 'l10; }
                'c11: {
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_b_d__str20, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(28), fp);
                    if (((((l_i_1) != (0i32)) as i32)) != 0) {
                        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = (rt::rem_i32({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(500)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(548); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }, 8i32)).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        { let a2_ = rt::div_i32({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(500)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(548); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }, 8i32); F::Flash_ClipSetTex(c, l_flash_2, fp, a2_) };
                    } else {
                        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = (rt::rem_i32({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(500)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }, 8i32)).wrapping_mul(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 512i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r32((fp.wrapping_add(8)).wrapping_add(4)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        { let a2_ = rt::div_i32({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(500)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }, 8i32); F::Flash_ClipSetTex(c, l_flash_2, fp, a2_) };
                    }
                    F::Flash_ClipSetUv(c, l_flash_2, fp, fp.wrapping_add(8));
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), sym::menu_b_d__str21, fp);
        if (((({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(8)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(500)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16)); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
        } else {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_2, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
        }
        {
            rt::memset(c, fp.wrapping_add(156), 0, 12);
            { let v_ = F32(0x3eb60b60); c.m.w32(fp.wrapping_add(156).wrapping_add(0), (v_).0); }
            { let v_ = F32(0x3eb60b60); c.m.w32(fp.wrapping_add(156).wrapping_add(4), (v_).0); }
            { let v_ = F32(0x3f088888); c.m.w32(fp.wrapping_add(156).wrapping_add(8), (v_).0); }
            { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
            {
                l_i_1 = 0i32;
                'l12: loop {
                    if !(((((l_i_1) < (3i32)) as i32)) != 0) { break 'l12; }
                    'c13: {
                        { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_b_d__str22, &[(((l_i_1).wrapping_add(1i32)) as i64 as u64)]); };
                        F::Flash_FindLabel(c, l_flash_2, ((0i32) as u32), fp.wrapping_add(28), fp);
                        { let a4_ = ((c.m.r32(sym::gHistSel)).wrapping_add(580)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)); let a6_ = F32(c.m.r32((fp.wrapping_add(156)).wrapping_add((((l_i_1) as u32)).wrapping_mul(4)))); F::FlashAnim_Scroll(c, l_flash_2, fp, fp.wrapping_add(8), ((0i32) as u32), a4_, F32(0x00000000), a6_) };
                    }
                    { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
                }
            }
        }
        {
            l_i_1 = 0i32;
            'l14: loop {
                if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l14; }
                'c15: {
                    { let a0_ = ((c.m.r32(sym::gHistSel)).wrapping_add(80)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        F::IconWin_Draw(c);
        { let a0_ = { let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(8)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(484); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::MsgWin_SetText(c, a0_) };
        { let a2_ = { let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(480); (c.m.r32(a_) as i32) }; F::MsgWin_Draw(c, 0i32, 0i32, a2_) };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HistSel_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn HistSel_PlateGoto(c: &mut Ctx, mut p_idx_1: i32, mut p_kind_2: i32, mut p_label_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistSel_PlateGoto", &PROBE_N, &[(p_idx_1) as u32, (p_kind_2) as u32, (p_label_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_flash_4: u32 = 0;
    let r_ = 'f: {
        l_flash_4 = ((c.m.r32(sym::gHistSel)).wrapping_add(80)).wrapping_add((((p_idx_1) as u32)).wrapping_mul(44));
        's1: {
            let sv5: i32 = p_kind_2;
            match sv5 {
                0i32 => {
                    { let _ = { let a2_ = ({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_b_d__str23, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                2i32 => {
                    { let _ = { let a2_ = ({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_b_d__str24, &[((a2_) as i64 as u64)]) }; };
                    break 's1;
                }
                _ => {
                    break 'f;
                }
            }
        }
        F::Flash_FindLabel(c, l_flash_4, ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, l_flash_4, fp, p_label_3);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HistSel_PlateGoto", &PROBE_N, &[], true); }
    r_
}

pub fn HistSel_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistSel_Update", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(460); (c.m.r32(a_) as i32) }) & (8i32))) != 0) {
        if ((((((((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(480); (c.m.r32(a_) as i32) }) >= (14i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(480); (c.m.r32(a_) as i32) }) <= (19i32)) as i32)) != 0)) as i32)) != 0) {
            if (((((F::Voice_GetStat(c, 0i32)) == (5i32)) as i32)) != 0) {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                { let v_ = ({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(5i32); let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                F::HistSel_PlayVoice(c);
            }
        }
    }
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gHistSel)).wrapping_add(80)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Advance(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if (((({ let a_ = (((c.m.r32(sym::gHistSel)).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(16); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 15i32); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(592); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) == ((1i32).wrapping_neg())) as i32)) != 0) {
        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 13i32); };
        { let v_ = ((((F::Rand_Range(c, ((10i32) as u32))).wrapping_mul(((60i32) as u32))).wrapping_add(((300i32) as u32))) as i32); let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(592); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "HistSel_Update", &PROBE_N, &[], true); }
    r_
}

pub fn HistSel_Input(c: &mut Ctx, mut p_result_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistSel_Input", &PROBE_N, &[(p_result_1) as u32], false); }
    let r_ = 'f: {
    if ((((!(((({ let a_ = (((c.m.r32(sym::gHistSel)).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f;
    }
    if ((((!(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(460); (c.m.r32(a_) as i32) }) & (4i32))) != 0)) as i32)) != 0) {
        F::HistSel_PlateGoto(c, 0i32, 0i32, sym::menu_b_d__str25);
        { let r_ = 4i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(460); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    's1: {
        let sv2: i32 = { let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); (c.m.r32(a_) as i32) };
        match sv2 {
            0i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(488); c.m.w32(a_, (v_) as u32); };
                    F::HistSel_PlateGoto(c, 0i32, 0i32, sym::menu_b_d__str26);
                    { let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    if ((((({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                        { let v_ = 1i32; let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    F::HistSel_PlateGoto(c, 0i32, 0i32, sym::menu_b_d__str27);
                    { let v_ = ({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(5i32); let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                    F::HistSel_PlayVoice(c);
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(488); c.m.w32(a_, (v_) as u32); };
                        F::HistSel_PlateGoto(c, 0i32, 0i32, sym::menu_b_d__str28);
                        { let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= (2i32)) as i32)) != 0) {
                            { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        F::HistSel_PlateGoto(c, 0i32, 0i32, sym::menu_b_d__str29);
                        { let v_ = ({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(5i32); let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                        F::HistSel_PlayVoice(c);
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(488); c.m.w32(a_, (v_) as u32); };
                            F::HistSel_PlateGoto(c, 0i32, 0i32, sym::menu_b_d__str30);
                            's2: {
                                let sv3: i32 = { let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                                match sv3 {
                                    0i32 => {
                                        { let a0_ = ((c.m.r32(sym::gHistSel)).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_b_d__str31, ((((1i32) as i64 as u64)) as u8)) };
                                        if (((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(8); (c.m.r32(a_) as i32) }) & (512i32))) != 0) {
                                            { let r_ = (!(512i32)); let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(8); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                                            { let v_ = 42i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                                        } else {
                                            { let v_ = (((F::Rand_Range(c, ((3i32) as u32))).wrapping_add(((19i32) as u32))) as i32); let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                                        }
                                        F::HistSel_PlayVoice(c);
                                        { let v_ = 1i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); c.m.w32(a_, (v_) as u32); };
                                        break 's2;
                                    }
                                    1i32 => {
                                        { let a0_ = ((c.m.r32(sym::gHistSel)).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_b_d__str32, ((((1i32) as i64 as u64)) as u8)) };
                                        { let v_ = { let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(4); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                        F::HistSel_PlateGoto(c, 0i32, 2i32, sym::menu_b_d__str33);
                                        { let v_ = ({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(7i32); let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                                        F::HistSel_PlayVoice(c);
                                        { let v_ = 2i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); c.m.w32(a_, (v_) as u32); };
                                        break 's2;
                                    }
                                    _ => {
                                    }
                                }
                            }
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(488); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 51i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(496); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 0i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                            } else {
                                F::HistSel_Idle(c);
                            }
                        }
                    }
                }
                break 's1;
            }
            1i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) != 0) {
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(488); c.m.w32(a_, (v_) as u32); };
                    if ((((({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(500)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(540); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (9i32)) as i32)) != 0) {
                        break 'f;
                    }
                    { let a0_ = ((c.m.r32(sym::gHistSel)).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_b_d__str34, ((((1i32) as i64 as u64)) as u8)) };
                    { let v_ = rt::rem_i32(({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32), { let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(536); (c.m.r32(a_) as i32) }); let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(544); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(548); c.m.w32(a_, (v_) as u32); };
                    { let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    if ((((({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                        { let v_ = ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(536); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    { let v_ = ({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(540); c.m.w32(a_, (v_) as u32); };
                    if ((((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(540); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                        { let r_ = { let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(536); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(540); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                    }
                    F::HistSel_SayItem(c);
                    { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 10i32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) != 0) {
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(488); c.m.w32(a_, (v_) as u32); };
                        if ((((({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(500)).wrapping_add((((rt::rem_i32(({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32), { let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(536); (c.m.r32(a_) as i32) })) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (9i32)) as i32)) != 0) {
                            break 'f;
                        }
                        { let a0_ = ((c.m.r32(sym::gHistSel)).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_b_d__str35, ((((1i32) as i64 as u64)) as u8)) };
                        { let v_ = { let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(540); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(544); c.m.w32(a_, (v_) as u32); };
                        { let v_ = { let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(548); c.m.w32(a_, (v_) as u32); };
                        { let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= ({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(536); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                            { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        { let v_ = ({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(540); c.m.w32(a_, (v_) as u32); };
                        if ((((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(540); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                            { let r_ = { let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(536); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(540); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                        }
                        F::HistSel_SayItem(c);
                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 10i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(488); c.m.w32(a_, (v_) as u32); };
                            's3: {
                                let sv4: i32 = { let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(500)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                                match sv4 {
                                    9i32 => {
                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 7i32); };
                                        break 's3;
                                    }
                                    8i32 => {
                                        { let r_ = (!(2i32)); let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(8)).wrapping_add((((8i32) as u32)).wrapping_mul(16)); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                                        { let v_ = 2i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
                                        { let r_ = 1i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(460); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                        { let r_ = 2i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(460); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                        { let v_ = 15i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(476); c.m.w32(a_, (v_) as u32); };
                                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 11i32); };
                                        break 's3;
                                    }
                                    _ => {
                                        if (((({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(8)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(500)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(16)); (c.m.r32(a_) as i32) }) & (8i32))) != 0) {
                                            { let v_ = 101i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(496); c.m.w32(a_, (v_) as u32); };
                                        } else {
                                            { let v_ = 151i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(496); c.m.w32(a_, (v_) as u32); };
                                        }
                                        { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 11i32); };
                                        break 's3;
                                    }
                                }
                            }
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(488); c.m.w32(a_, (v_) as u32); };
                                { let a0_ = ((c.m.r32(sym::gHistSel)).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_b_d__str36, ((((1i32) as i64 as u64)) as u8)) };
                                F::HistSel_PlateGoto(c, 0i32, 0i32, sym::menu_b_d__str37);
                                { let v_ = ({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(5i32); let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                                F::HistSel_PlayVoice(c);
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                            } else {
                                F::HistSel_Idle(c);
                            }
                        }
                    }
                }
                break 's1;
            }
            2i32 => {
                if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) != 0) {
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(488); c.m.w32(a_, (v_) as u32); };
                    F::HistSel_PlateGoto(c, 0i32, 2i32, sym::menu_b_d__str38);
                    { let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    if ((((({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < (0i32)) as i32)) != 0) {
                        { let v_ = 2i32; let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    }
                    F::HistSel_PlateGoto(c, 0i32, 2i32, sym::menu_b_d__str39);
                    { let v_ = ({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(7i32); let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                    F::HistSel_PlayVoice(c);
                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                } else {
                    if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) != 0) {
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(488); c.m.w32(a_, (v_) as u32); };
                        F::HistSel_PlateGoto(c, 0i32, 2i32, sym::menu_b_d__str40);
                        { let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        if ((((({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) >= (3i32)) as i32)) != 0) {
                            { let v_ = 0i32; let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        F::HistSel_PlateGoto(c, 0i32, 2i32, sym::menu_b_d__str41);
                        { let v_ = ({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(7i32); let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                        F::HistSel_PlayVoice(c);
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                    } else {
                        if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(488); c.m.w32(a_, (v_) as u32); };
                            { let v_ = { let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                            { let a0_ = ((c.m.r32(sym::gHistSel)).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_b_d__str42, ((((1i32) as i64 as u64)) as u8)) };
                            F::HistSel_PlateGoto(c, 0i32, 2i32, sym::menu_b_d__str43);
                            F::HistSel_PlateGoto(c, 0i32, 0i32, sym::menu_b_d__str44);
                            { let v_ = ({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(5i32); let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                            F::HistSel_PlayVoice(c);
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); c.m.w32(a_, (v_) as u32); };
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                        } else {
                            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) != 0) {
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(488); c.m.w32(a_, (v_) as u32); };
                                { let a0_ = ((c.m.r32(sym::gHistSel)).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_b_d__str45, ((((1i32) as i64 as u64)) as u8)) };
                                F::HistSel_PlateGoto(c, 0i32, 2i32, sym::menu_b_d__str46);
                                F::HistSel_PlateGoto(c, 0i32, 0i32, sym::menu_b_d__str47);
                                { let v_ = ({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(5i32); let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                                F::HistSel_PlayVoice(c);
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(492); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                            } else {
                                F::HistSel_Idle(c);
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
    if rt::probing() { crate::host::harness::probe(c, "HistSel_Input", &PROBE_N, &[], true); }
    r_
}

pub fn HistSel_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "HistSel_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = 1i32; c.m.w32(fp, (v_) as u32); }
        F::HistSel_Init(c, p_section_1);
        F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
        'l1: while ((1i32) != 0) {
            F::Gfx_BeginFrame(c);
            F::Pad_Update(c);
            F::Snd_Update(c);
            F::ColorFade_Update(c);
            if ((((!(((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0)) as i32)) != 0) {
                F::HistSel_Update(c);
                { let a0_ = c.m.r32(sym::gHistSel); F::HistGuide_Update(c, a0_, fp) };
            }
            F::HistSel_Draw(c);
            F::ColorFade_Draw(c);
            F::Gfx_EndFrame(c, 1i32);
            F::Dma_Flush(c);
            F::File_Stub264D90(c);
            if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
                continue 'l1;
            }
            if ((F::ColorFade_IsInDone(c)) != 0) {
                if ((((((((!(((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(460); (c.m.r32(a_) as i32) }) & (8i32))) != 0)) as i32)) != 0) && (((({ let a_ = (((c.m.r32(sym::gHistSel)).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
                    { let r_ = 8i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(460); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    if (((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(460); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
                        { let v_ = 1151i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(496); c.m.w32(a_, (v_) as u32); };
                    } else {
                        if (((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(460); (c.m.r32(a_) as i32) }) & (16i32))) != 0) {
                            { let v_ = 1i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(496); c.m.w32(a_, (v_) as u32); };
                        } else {
                            if (((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(460); (c.m.r32(a_) as i32) }) & (32i32))) != 0) {
                                { let v_ = 651i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(496); c.m.w32(a_, (v_) as u32); };
                            } else {
                                { let a0_ = ((c.m.r32(sym::gHistSel)).wrapping_add(80)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_b_d__str48, ((((1i32) as i64 as u64)) as u8)) };
                                { let v_ = 8i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(484); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(560); c.m.w32(a_, (v_) as u32); };
                                { let v_ = (((F::Rand_Range(c, ((5i32) as u32))).wrapping_add(((14i32) as u32))) as i32); let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(480); c.m.w32(a_, (v_) as u32); };
                                F::HistSel_PlayVoice(c);
                                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 14i32); };
                            }
                        }
                    }
                }
            }
            if ((F::ColorFade_IsFadingOut(c)) != 0) {
                F::Voice_FadeOutStep(c, 0i32);
                F::Bgm_FadeOutStep(c);
                F::StreamSe_FadeOutStep(c, 0i32);
                continue 'l1;
            }
            if ((F::ColorFade_IsOutDone(c)) != 0) {
                break 'l1;
            }
            if (((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(460); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(476); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) == ((1i32).wrapping_neg())) as i32)) != 0) {
                    if ((((((c.m.r32(fp) as i32)) != (1i32)) as i32)) != 0) {
                        F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                    } else {
                        F::ColorFade_StartOut(c, 255i32, 255i32, 255i32, 20i32);
                    }
                    { let v_ = { let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(500)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gHistSel)).wrapping_add(464)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(52); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(8)).wrapping_add(4); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(60); c.m.w32(a_, (v_) as u32); };
                }
            } else {
                if ((((({ let a_ = (c.m.r32(sym::gHistSel)).wrapping_add(496); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::HistSel_Input(c, fp);
                }
            }
        }
        F::HistSel_Term(c);
        F::Dma_ResetBuffers(c);
        break 'f (c.m.r32(fp) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "HistSel_Run", &PROBE_N, &[], true); }
    r_
}
