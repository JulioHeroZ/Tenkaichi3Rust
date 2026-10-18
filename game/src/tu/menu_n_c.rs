#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn UbRank_SetupBattle(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbRank_SetupBattle", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_rule_1: u32 = 0;
    let mut l_unk14_2: i32 = 0;
    let mut l_unk4_3: i32 = 0;
    let mut l_announcer_4: i32 = 0;
    let mut l_stage_5: i32 = 0;
    let mut l_bgm_6: i32 = 0;
    let mut l_timeLimit_7: i32 = 0;
    let mut l_health_8: F32 = F32(0);
    let mut l_n_9: i32 = 0;
    let mut l_j_10: i32 = 0;
    let mut l_item_11: i32 = 0;
    let r_ = 'f: {
        l_rule_1 = { let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1388); c.m.r32(a_) };
        l_unk14_2 = ((((c.m.r32((l_rule_1).wrapping_add(20)) as i32)) != (0i32)) as i32);
        l_unk4_3 = ((((c.m.r32((l_rule_1).wrapping_add(4)) as i32)) != (0i32)) as i32);
        l_announcer_4 = (c.m.r32(l_rule_1) as i32);
        l_stage_5 = (c.m.r32((l_rule_1).wrapping_add(12)) as i32);
        l_bgm_6 = (c.m.r32((l_rule_1).wrapping_add(16)) as i32);
        l_timeLimit_7 = (c.m.r32((l_rule_1).wrapping_add(8)) as i32);
        if (((((l_announcer_4) == (998i32)) as i32)) != 0) {
            l_announcer_4 = ((F::Rand_Range(c, ((8i32) as u32))) as i32);
        }
        if (((((l_stage_5) == (998i32)) as i32)) != 0) {
            l_stage_5 = { let a_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(52); c.m.r32(a_) }).wrapping_add((F::Rand_Range(c, ((28i32) as u32))).wrapping_mul(4)); (c.m.r32(a_) as i32) };
        }
        if (((((l_bgm_6) == (998i32)) as i32)) != 0) {
            l_bgm_6 = { let a_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(56); c.m.r32(a_) }).wrapping_add((F::Rand_Range(c, ((11i32) as u32))).wrapping_mul(4)); (c.m.r32(a_) as i32) };
        }
        { let _ = F::memset(c, fp, 0i32, 16u32); };
        l_n_9 = 0i32;
        {
            l_j_10 = 0i32;
            'l1: loop {
                if !(((((l_j_10) < (7i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_item_11 = { let a_ = (({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1392); c.m.r32(a_) }).wrapping_add(16)).wrapping_add((((l_j_10) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) };
                    if (((((l_item_11) != (999i32)) as i32)) != 0) {
                        { let v_ = (((l_item_11).wrapping_add(1i32)) as u16); let a_ = (fp).wrapping_add(((({ let o_ = l_n_9; let n_ = (o_).wrapping_add(1); l_n_9 = n_; o_ }) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
                    }
                }
                { let o_ = l_j_10; let n_ = (o_).wrapping_add(1); l_j_10 = n_; };
            }
        }
        if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1392); c.m.r32(a_) }).wrapping_add(12); (c.m.r32(a_) as i32) }) == (999i32)) as i32)) != 0) {
            { let v_ = ((0i32) as u16); let a_ = (fp).wrapping_add((((7i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
        } else {
            { let v_ = ((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1392); c.m.r32(a_) }).wrapping_add(12); (c.m.r32(a_) as i32) }).wrapping_add(1i32)) as u16); let a_ = (fp).wrapping_add((((7i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
        }
        F::Battle_ClearWork(c);
        l_health_8 = F32(0x42c80000);
        F::BattleSetup_SetRule(c, 0i32, 2i32, l_bgm_6, l_timeLimit_7, l_announcer_4, l_stage_5, l_unk4_3);
        F::BattleSetup_SetSide(c, 0i32, 0i32, 0i32, 1i32, 1i32, 0i32, 0i32, ((0i32) as u32));
        F::BattleSetup_SetSide(c, 1i32, 2i32, 1i32, 1i32, l_unk14_2, 0i32, 0i32, ((0i32) as u32));
        { let a2_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1116); (c.m.r32(a_) as i32) }; let a3_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1112); (c.m.r32(a_) as i32) }; let a7_ = (c.m.r32(sym::gProgress)).wrapping_add(1120); F::BattleSetup_SetMember(c, 0i32, 0i32, a2_, a3_, 0i32, 0i32, l_health_8, a7_) };
        { let a2_ = { let a_ = { let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1392); c.m.r32(a_) }; (c.m.r32(a_) as i32) }; let a3_ = { let a_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1392); c.m.r32(a_) }).wrapping_add(4); (c.m.r32(a_) as i32) }; let a5_ = { let a_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1392); c.m.r32(a_) }).wrapping_add(8); (c.m.r32(a_) as i32) }; F::BattleSetup_SetMember(c, 1i32, 0i32, a2_, a3_, 0i32, a5_, l_health_8, fp) };
        F::BattleSetup_Finish(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "UbRank_SetupBattle", &PROBE_N, &[], true); }
    r_
}

pub fn UbRank_PlaceToRule(c: &mut Ctx, mut p_place_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbRank_PlaceToRule", &PROBE_N, &[(p_place_1) as u32], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(1916); (c.m.r32(a_) as i32) }) < (p_place_1)) as i32)) != 0) {
        break 'f (p_place_1).wrapping_sub(1i32);
    }
    break 'f p_place_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "UbRank_PlaceToRule", &PROBE_N, &[], true); }
    r_
}

pub fn UbRank_SetFoe(c: &mut Ctx, mut p_idx_1: i32, mut p_intruder_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbRank_SetFoe", &PROBE_N, &[(p_idx_1) as u32, (p_intruder_2) as u32], false); }
    let r_ = 'f: {
    if ((((!((p_intruder_2) != 0)) as i32)) != 0) {
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(36); c.m.r32(a_) }).wrapping_add((((p_idx_1) as u32)).wrapping_mul(28)); let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1388); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(44); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1388); c.m.r32(a_) }).wrapping_add(24); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(44)); let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1392); c.m.w32(a_, v_); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1396); c.m.w32(a_, (v_) as u32); };
    } else {
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(40); c.m.r32(a_) }).wrapping_add((((p_idx_1) as u32)).wrapping_mul(28)); let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1388); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(48); c.m.r32(a_) }).wrapping_add(((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1388); c.m.r32(a_) }).wrapping_add(24); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(44)); let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1392); c.m.w32(a_, v_); };
        { let v_ = 1i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1396); c.m.w32(a_, (v_) as u32); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "UbRank_SetFoe", &PROBE_N, &[], true); }
    r_
}

pub fn UbRank_PickFoe(c: &mut Ctx, mut p_place_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbRank_PickFoe", &PROBE_N, &[(p_place_1) as u32], false); }
    let mut l_intrude_2: i32 = 0;
    let r_ = 'f: {
    l_intrude_2 = (((((F::Rand_Range(c, ((65535i32) as u32))) as i32)) < (6554i32)) as i32);
    'l1: loop {
        'c2: {
            if (((((((((((((p_place_1) < (10i32)) as i32)) != 0) || ((((!((l_intrude_2) != 0)) as i32)) != 0)) as i32)) != 0) || ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(1916); (c.m.r32(a_) as i32) }) < (p_place_1)) as i32)) != 0)) as i32)) != 0) {
                { let a0_ = F::UbRank_PlaceToRule(c, p_place_1); F::UbRank_SetFoe(c, a0_, 0i32) };
                { let r_ = (!(16i32)); let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1668); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
        }
        if !((0i32) != 0) { break 'l1; }
    }
    { let a0_ = ((F::Rand_Range(c, ((37i32) as u32))) as i32); F::UbRank_SetFoe(c, a0_, 1i32) };
    { let r_ = 16i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1668); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "UbRank_PickFoe", &PROBE_N, &[], true); }
    r_
}

pub fn UbRank_ScrollUp(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbRank_ScrollUp", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if ((((({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1380); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
        { let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1380); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
        { let a0_ = ((c.m.r32(sym::gUbRank)).wrapping_add(60)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_n_c__str0, ((((1i32) as i64 as u64)) as u8)) };
        { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1380); (c.m.r32(a_) as i32) }).wrapping_add(5i32); let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1372); c.m.w32(a_, (v_) as u32); };
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "UbRank_ScrollUp", &PROBE_N, &[], true); }
    r_
}

pub fn UbRank_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbRank_Init", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = F::Heap_Alloc(c, ((1412u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gUbRank; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gUbRank); F::memset(c, a0_, 0i32, 1412u32) }; };
        { let v_ = { let a0_ = ({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_add(27i32); F::File_LoadSync(c, a0_, ((0i32) as u32), 0i32) }; let a_ = c.m.r32(sym::gUbRank); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gUbRank); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = F::Heap_Alloc(c, 92160i32, ((64i32) as u32), 0i32, 2i32); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::Heap_Alloc(c, 92160i32, ((64i32) as u32), 0i32, 2i32); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::Heap_Alloc(c, 133120i32, ((32i32) as u32), 0i32, 2i32); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = F::Heap_Alloc(c, 133120i32, ((32i32) as u32), 0i32, 2i32); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(36); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(40); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(44); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(48); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(52); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(56); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = c.m.r32(fp); let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(192); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((31i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((32i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((33i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((7i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((8i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((9i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((10i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((11i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((12i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((13i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((14i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((15i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((16i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((17i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((18i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((19i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((30i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((22i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((23i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((24i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((25i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((27i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((34i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((28i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((35i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((29i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((36i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((11i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(344)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((12i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(344)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((20i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(344)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((21i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(344)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(344)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(344)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((26i32).wrapping_mul(64i32)) as u32)); let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(420); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(368)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(368)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(368)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(368)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(368)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(368)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(368)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((7i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(368)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((8i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(368)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((9i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(368)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((10i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(368)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(8); c.m.w32(a_, v_); };
        {
            l_i_1 = 0i32;
            'l1: loop {
                if !(((((l_i_1) < (165i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(8); c.m.r32(a_) }).wrapping_add(((((l_i_1).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        { let a0_ = ((c.m.r32(sym::gUbRank)).wrapping_add(60)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gUbRank)).wrapping_add(196); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gUbRank)).wrapping_add(60)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let a0_ = ((c.m.r32(sym::gUbRank)).wrapping_add(60)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gUbRank)).wrapping_add(344); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gUbRank)).wrapping_add(60)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let a0_ = ((c.m.r32(sym::gUbRank)).wrapping_add(60)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gUbRank)).wrapping_add(368); F::Flash_Create(c, a0_, a1_, a2_) };
        { let v_ = rt::rem_i32(F::Rand_Libc(c), 32i32); let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1360); c.m.w32(a_, (v_) as u32); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = c.m.r32(fp); F::IconWin_Init(c, a0_, a1_) };
        F::IconWin_Open(c);
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(28); c.m.w32(a_, v_); };
        {
            l_i_1 = 0i32;
            'l3: loop {
                if !(((((l_i_1) < (6i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let a0_ = ((c.m.r32(sym::gUbRank)).wrapping_add(424)).wrapping_add((((l_i_1) as u32)).wrapping_mul(140)); let a1_ = { let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(28); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, ((0i32) as u32)) };
                    { let a0_ = ((c.m.r32(sym::gUbRank)).wrapping_add(424)).wrapping_add((((l_i_1) as u32)).wrapping_mul(140)); F::TextBox_SetUnk50(c, a0_, 0i32) };
                }
                { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
            }
        }
        { let v_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(32); c.m.w32(a_, v_); };
        { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1352); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1348); c.m.w32(a_, (v_) as u32); };
        if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1668); (c.m.r32(a_) as i32) }) & (4i32))) != 0) {
            { let r_ = (!(4i32)); let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1668); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            l_i_1 = { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(1916); (c.m.r32(a_) as i32) };
            if (((((F::BattleResult_GetFlags(c)) == (1i32)) as i32)) != 0) {
                { let v_ = 1i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1400); c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1400); c.m.w32(a_, (v_) as u32); };
            }
            if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1668); (c.m.r32(a_) as i32) }) & (8i32))) != 0) {
                { let r_ = 256i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a0_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1672); (c.m.r32(a_) as i32) }; F::UbRank_SetFoe(c, a0_, 0i32) };
                if (({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1400); (c.m.r32(a_) as i32) }) != 0) {
                    if (((((l_i_1) > (0i32)) as i32)) != 0) {
                        { let o_ = l_i_1; let n_ = (o_).wrapping_sub(1); l_i_1 = n_; };
                    }
                    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1668); (c.m.r32(a_) as i32) }) & (16i32))) != 0) {
                        { let v_ = 5i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1404); c.m.w32(a_, (v_) as u32); };
                    }
                }
            }
        } else {
            { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1336); c.m.w32(a_, (v_) as u32); };
            l_i_1 = { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(1916); (c.m.r32(a_) as i32) };
        }
        if (((((l_i_1) >= (98i32)) as i32)) != 0) {
            { let v_ = (100i32).wrapping_sub(5i32); let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1380); c.m.w32(a_, (v_) as u32); };
        } else {
            if (((((l_i_1) < (2i32)) as i32)) != 0) {
                { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1380); c.m.w32(a_, (v_) as u32); };
            } else {
                { let v_ = (l_i_1).wrapping_sub(2i32); let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1380); c.m.w32(a_, (v_) as u32); };
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "UbRank_Init", &PROBE_N, &[], true); }
    r_
}

pub fn UbRank_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbRank_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    F::IconWin_Term(c);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (3i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gUbRank)).wrapping_add(60)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Destroy(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if ((((({ let a_ = c.m.r32(sym::gUbRank); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = c.m.r32(sym::gUbRank); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = c.m.r32(sym::gUbRank); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gUbRank)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gUbRank); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gUbRank; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "UbRank_Term", &PROBE_N, &[], true); }
    r_
}

pub fn UbRank_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbRank_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_flash_1: u32 = 0;
    let mut l_res_2: u32 = 0;
    let mut l_i_3: i32 = 0;
    let mut l_place_4: i32 = 0;
    let mut l_chara_5: i32 = 0;
    let mut l_chara_6: i32 = 0;
    let mut l_chara_7: i32 = 0;
    let r_ = 'f: {
        { let a0_ = { let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(192); c.m.r32(a_) }; F::Sprite_DrawPicture(c, a0_, 0i32, 0i32, 128i32) };
        l_flash_1 = ((c.m.r32(sym::gUbRank)).wrapping_add(60)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        F::Flash_FindLabel(c, l_flash_1, sym::menu_n_c__str1, sym::menu_n_c__str2, fp);
        { let a2_ = (c.m.r32(sym::gUbRank)).wrapping_add(1360); F::FlashAnim_Blink(c, l_flash_1, fp, a2_, 0i32) };
        F::Flash_FindLabel(c, l_flash_1, sym::menu_n_c__str3, sym::menu_n_c__str4, fp);
        { let a2_ = (c.m.r32(sym::gUbRank)).wrapping_add(1364); F::FlashAnim_Talk(c, l_flash_1, fp, a2_, 0i32) };
        {
            l_i_3 = 0i32;
            'l1: loop {
                if !(((((l_i_3) < (5i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_place_4 = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1380); (c.m.r32(a_) as i32) }).wrapping_add(l_i_3);
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_n_c__str5, &[((l_i_3) as i64 as u64)]); };
                    F::Num_DrawChild(c, l_flash_1, fp.wrapping_add(28), sym::menu_n_c__str6, 0i32, 3i32, (l_place_4).wrapping_add(1i32), 64i32, 64i32, 0i32, 0i32);
                    if (((((l_place_4) == ({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(1916); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        { let v_ = { let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(420); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add(((((c.m.r32((sym::menu_n_c__sUbRankChipTex).wrapping_add((((l_i_3) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_n_c__str7, fp);
                        { let a4_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1116); (c.m.r32(a_) as i32) }; let a5_ = ((c.m.r32(sym::gUbRank)).wrapping_add(424)).wrapping_add((((l_i_3) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, a4_, a5_) };
                    } else {
                        l_chara_5 = { let a_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(44); c.m.r32(a_) }).wrapping_add(((({ let a_ = (({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(36); c.m.r32(a_) }).wrapping_add((((F::UbRank_PlaceToRule(c, l_place_4)) as u32)).wrapping_mul(28))).wrapping_add(24); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(44)); (c.m.r32(a_) as i32) };
                        l_res_2 = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(8); c.m.r32(a_) }).wrapping_add(((((l_chara_5).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
                        { let v_ = (c.m.r32((l_res_2).wrapping_add(16))).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add(((((c.m.r32((sym::menu_n_c__sUbRankChipTex).wrapping_add((((l_i_3) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_n_c__str8, fp);
                        { let a5_ = ((c.m.r32(sym::gUbRank)).wrapping_add(424)).wrapping_add((((l_i_3) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, l_chara_5, a5_) };
                    }
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_n_c__str9, &[((l_i_3) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(28), fp);
                    's3: {
                        let sv8: i32 = { let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1336); (c.m.r32(a_) as i32) };
                        match sv8 {
                            2i32 | 3i32 | 4i32 | 5i32 => {
                                if (((((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1668); (c.m.r32(a_) as i32) }) & (8i32))) != 0) && ((((((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(1916); (c.m.r32(a_) as i32) }) == (l_place_4)) as i32)) != 0) || ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1672); (c.m.r32(a_) as i32) }) == (l_place_4)) as i32)) != 0)) as i32)) != 0)) as i32)) != 0) {
                                    { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                                } else {
                                    { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                                }
                                break 's3;
                            }
                            _ => {
                                { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
                                break 's3;
                            }
                        }
                    }
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_n_c__str10, &[((5i32) as i64 as u64)]); };
        { let a5_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1372); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::Num_DrawChild(c, l_flash_1, fp.wrapping_add(28), sym::menu_n_c__str11, 0i32, 3i32, a5_, 64i32, 64i32, 0i32, 0i32) };
        if ((((({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1372); (c.m.r32(a_) as i32) }) == ({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(1916); (c.m.r32(a_) as i32) })) as i32)) != 0) {
            { let v_ = { let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(420); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add(((((c.m.r32((sym::menu_n_c__sUbRankChipTex).wrapping_add((((5i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_n_c__str12, fp);
            { let a4_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1116); (c.m.r32(a_) as i32) }; let a5_ = ((c.m.r32(sym::gUbRank)).wrapping_add(424)).wrapping_add((((5i32) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, a4_, a5_) };
        } else {
            l_chara_6 = { let a_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(44); c.m.r32(a_) }).wrapping_add(((({ let a_ = (({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(36); c.m.r32(a_) }).wrapping_add(((({ let a0_ = { let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1372); (c.m.r32(a_) as i32) }; F::UbRank_PlaceToRule(c, a0_) }) as u32)).wrapping_mul(28))).wrapping_add(24); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(44)); (c.m.r32(a_) as i32) };
            l_res_2 = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(8); c.m.r32(a_) }).wrapping_add(((((l_chara_6).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
            { let v_ = (c.m.r32((l_res_2).wrapping_add(16))).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(196)).wrapping_add(((((c.m.r32((sym::menu_n_c__sUbRankChipTex).wrapping_add((((5i32) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
            F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_n_c__str13, fp);
            { let a5_ = ((c.m.r32(sym::gUbRank)).wrapping_add(424)).wrapping_add((((5i32) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, l_chara_6, a5_) };
        }
        F::Flash_FindLabel(c, l_flash_1, sym::menu_n_c__str14, sym::menu_n_c__str15, fp);
        if ((((({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1380); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
        } else {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
        }
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        F::Flash_FindLabel(c, l_flash_1, sym::menu_n_c__str16, sym::menu_n_c__str17, fp);
        if ((((({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1380); (c.m.r32(a_) as i32) }) < ((100i32).wrapping_sub(5i32))) as i32)) != 0) {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((1i32) as i64 as u64)) as u8)); };
        } else {
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
        }
        { let v_ = 32i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        l_flash_1 = ((c.m.r32(sym::gUbRank)).wrapping_add(60)).wrapping_add((((1i32) as u32)).wrapping_mul(44));
        's4: {
            let sv9: i32 = { let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1336); (c.m.r32(a_) as i32) };
            match sv9 {
                1i32 => {
                    if (((({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1264); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
                        l_chara_7 = { let a_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(44); c.m.r32(a_) }).wrapping_add(((({ let a_ = (({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(36); c.m.r32(a_) }).wrapping_add(((({ let a0_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1672); (c.m.r32(a_) as i32) }; F::UbRank_PlaceToRule(c, a0_) }) as u32)).wrapping_mul(28))).wrapping_add(24); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(44)); (c.m.r32(a_) as i32) };
                        { let a2_ = ((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1672); (c.m.r32(a_) as i32) }).wrapping_sub({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1380); (c.m.r32(a_) as i32) })).wrapping_mul(52i32)).wrapping_add(132i32); F::Flash_SetOffset(c, l_flash_1, 197i32, a2_) };
                        l_res_2 = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(8); c.m.r32(a_) }).wrapping_add(((((l_chara_7).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
                        { let v_ = (c.m.r32((l_res_2).wrapping_add(16))).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(344)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        l_res_2 = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((({ let a_ = { let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1392); c.m.r32(a_) }; (c.m.r32(a_) as i32) }).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
                        { let v_ = (c.m.r32((l_res_2).wrapping_add(16))).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(344)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        F::Flash_SetFlag(c, l_flash_1, ((4i32) as u32), ((0i32) as u8));
                    }
                    break 's4;
                }
                2i32 | 3i32 | 4i32 | 5i32 => {
                    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1668); (c.m.r32(a_) as i32) }) & (8i32))) != 0) {
                        { let a2_ = ((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1672); (c.m.r32(a_) as i32) }).wrapping_sub({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1380); (c.m.r32(a_) as i32) })).wrapping_mul(52i32)).wrapping_add(184i32); F::Flash_SetOffset(c, l_flash_1, 197i32, a2_) };
                        l_res_2 = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((({ let a_ = { let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1392); c.m.r32(a_) }; (c.m.r32(a_) as i32) }).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
                        if ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(1916); (c.m.r32(a_) as i32) }) > ({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1672); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                            { let v_ = (c.m.r32((l_res_2).wrapping_add(16))).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(344)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                            { let v_ = { let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(420); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(344)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        } else {
                            { let v_ = (c.m.r32((l_res_2).wrapping_add(16))).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(344)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                            { let v_ = { let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(420); c.m.r32(a_) }; let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(344)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                        }
                        F::Flash_SetFlag(c, l_flash_1, ((4i32) as u32), ((0i32) as u8));
                    } else {
                        F::Flash_SetFlag(c, l_flash_1, ((4i32) as u32), ((1i32) as u8));
                    }
                    break 's4;
                }
                _ => {
                    F::Flash_SetFlag(c, l_flash_1, ((4i32) as u32), ((1i32) as u8));
                    break 's4;
                }
            }
        }
        {
            l_i_3 = 0i32;
            'l5: loop {
                if !(((((l_i_3) < (3i32)) as i32)) != 0) { break 'l5; }
                'c6: {
                    if ((((((((({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1336); (c.m.r32(a_) as i32) }) != (4i32)) as i32)) != 0) || (((((l_i_3) == (2i32)) as i32)) != 0)) as i32)) != 0) {
                        { let a0_ = ((c.m.r32(sym::gUbRank)).wrapping_add(60)).wrapping_add((((l_i_3) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
                    }
                }
                { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
            }
        }
        if ((((({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1336); (c.m.r32(a_) as i32) }) != (4i32)) as i32)) != 0) {
            F::IconWin_Draw(c);
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "UbRank_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn UbRank_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbRank_Update", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1324); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
        { let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1324); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    }
    if (((((((({ let a_ = (((c.m.r32(sym::gUbRank)).wrapping_add(60)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) && ((((({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1336); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0)) as i32)) != 0) {
        { let a0_ = ((c.m.r32(sym::gUbRank)).wrapping_add(60)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_n_c__str18, ((((1i32) as i64 as u64)) as u8)) };
        { let r_ = 64i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
    }
    if (((((((({ let a_ = (((c.m.r32(sym::gUbRank)).wrapping_add(60)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((2i32) as u32)))) != 0) && ((((({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1336); (c.m.r32(a_) as i32) }) == (6i32)) as i32)) != 0)) as i32)) != 0) {
        { let a0_ = ((c.m.r32(sym::gUbRank)).wrapping_add(60)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_n_c__str19, ((((1i32) as i64 as u64)) as u8)) };
        { let v_ = 5i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1336); c.m.w32(a_, (v_) as u32); };
    }
    if (((({ let a_ = (((c.m.r32(sym::gUbRank)).wrapping_add(60)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add(12); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
        's1: {
            let sv2: i32 = { let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1336); (c.m.r32(a_) as i32) };
            match sv2 {
                1i32 => {
                    { let r_ = 128i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let r_ = (!(64i32)); let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1264); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                    break 's1;
                }
                2i32 => {
                    { let r_ = 32i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    break 's1;
                }
                5i32 => {
                    { let r_ = 32i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    break 's1;
                }
                _ => {
                }
            }
        }
    }
    if (((({ let a_ = (((c.m.r32(sym::gUbRank)).wrapping_add(60)).wrapping_add((((1i32) as u32)).wrapping_mul(44))).wrapping_add(16); c.m.r32(a_) }) & (((1i32) as u32)))) != 0) {
        { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((2i32) as u32)) as u64), ((58i32) as i64 as u64)])) as i32); };
    }
    {
        l_i_1 = 0i32;
        'l2: loop {
            if !(((((l_i_1) < (3i32)) as i32)) != 0) { break 'l2; }
            'c3: {
                { let a0_ = ((c.m.r32(sym::gUbRank)).wrapping_add(60)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Advance(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "UbRank_Update", &PROBE_N, &[], true); }
    r_
}

pub fn UbRank_UpdateVoice(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbRank_UpdateVoice", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
        break 'f;
    }
    if ((((!(({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1348); (c.m.r32(a_) as i32) }) != 0)) as i32)) != 0) {
        if ((((({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1340); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            break 'f;
        }
        if ((((((((((((({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1352); (c.m.r32(a_) as i32) }) != ((1i32).wrapping_neg())) as i32)) != 0) && (((((F::Voice_GetStat(c, 0i32)) != (5i32)) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1340); (c.m.r32(a_) as i32) }) == ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1344); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((1i32) as i64 as u64)])) as i32); };
                { let v_ = 1i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1348); c.m.w32(a_, (v_) as u32); };
            }
            break 'f;
        }
    } else {
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1348); c.m.w32(a_, (v_) as u32); };
    }
    's1: {
        let sv1: i32 = { let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1340); (c.m.r32(a_) as i32) };
        match sv1 {
            1i32 => {
                { let v_ = 74i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1352); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = 75i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1352); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            3i32 => {
                { let v_ = 76i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1352); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            4i32 => {
                { let v_ = 80i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1352); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            5i32 => {
                { let v_ = 81i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1352); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            6i32 => {
                { let v_ = 53i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1352); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1340); c.m.w32(a_, (v_) as u32); };
    { let a0_ = { let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(32); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1352); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34661i32, a2_) };
    { let v_ = { let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1340); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1344); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "UbRank_UpdateVoice", &PROBE_N, &[], true); }
    r_
}

pub fn UbRank_Input(c: &mut Ctx, mut p_result_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbRank_Input", &PROBE_N, &[(p_result_1) as u32], false); }
    let mut l_state_2: i32 = 0;
    let mut l_top_3: i32 = 0;
    let mut l_pos_4: i32 = 0;
    let mut l_up_5: i32 = 0;
    let mut l_down_6: i32 = 0;
    let mut l_ok_7: i32 = 0;
    let mut l_cancel_8: i32 = 0;
    let r_ = 'f: {
    l_state_2 = { let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1336); (c.m.r32(a_) as i32) };
    l_top_3 = { let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1380); (c.m.r32(a_) as i32) };
    l_pos_4 = (l_top_3).wrapping_add({ let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(1268)).wrapping_add((((l_state_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) });
    l_up_5 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) as i32);
    l_down_6 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) as i32);
    l_ok_7 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) as i32);
    l_cancel_8 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) as i32);
    if ((((!(((({ let a_ = (((c.m.r32(sym::gUbRank)).wrapping_add(60)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
        break 'f;
    }
    if ((((!(((({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1264); (c.m.r32(a_) as i32) }) & (4i32))) != 0)) as i32)) != 0) {
        { let v_ = ({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(1916); (c.m.r32(a_) as i32) }).wrapping_sub(l_top_3); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(1268)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        F::UbRank_ClipGoto(c, 0i32, 0i32, sym::menu_n_c__str20);
        { let r_ = 4i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        if (((({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1264); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
            { let v_ = 5i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1336); c.m.w32(a_, (v_) as u32); };
            if (({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1400); (c.m.r32(a_) as i32) }) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(1916); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) {
                    { let v_ = 5i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1340); c.m.w32(a_, (v_) as u32); };
                } else {
                    if ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(1916); (c.m.r32(a_) as i32) }) == (2i32)) as i32)) != 0) {
                        { let v_ = 3i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1340); c.m.w32(a_, (v_) as u32); };
                    } else {
                        if ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(1916); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                            { let v_ = (((F::Rand_Range(c, ((2i32) as u32))).wrapping_add(((1i32) as u32))) as i32); let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1340); c.m.w32(a_, (v_) as u32); };
                        }
                    }
                }
                { let a0_ = ((c.m.r32(sym::gUbRank)).wrapping_add(60)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_n_c__str21, ((((1i32) as i64 as u64)) as u8)) };
                break 'f;
            }
            { let a0_ = ((c.m.r32(sym::gUbRank)).wrapping_add(60)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_n_c__str22, ((((1i32) as i64 as u64)) as u8)) };
            break 'f;
        }
        break 'f;
    }
    's1: {
        let sv9: i32 = l_state_2;
        match sv9 {
            0i32 => {
                if ((((((l_up_5) != 0) && (((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(1916); (c.m.r32(a_) as i32) }).wrapping_sub(1i32)) < (l_pos_4)) as i32)) != 0)) as i32)) != 0) {
                    if ((((((((({ let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(1268)).wrapping_add(((({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1336); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1380); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0)) as i32)) != 0) {
                        { let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1380); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                        { let a0_ = ((c.m.r32(sym::gUbRank)).wrapping_add(60)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_n_c__str23, ((((1i32) as i64 as u64)) as u8)) };
                        { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                        { let v_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1380); (c.m.r32(a_) as i32) }).wrapping_add(5i32); let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1372); c.m.w32(a_, (v_) as u32); };
                    } else {
                        if ((((({ let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(1268)).wrapping_add(((({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1336); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
                            F::UbRank_ClipGoto(c, 0i32, 0i32, sym::menu_n_c__str24);
                            { let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(1268)).wrapping_add(((({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1336); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                            F::UbRank_ClipGoto(c, 0i32, 0i32, sym::menu_n_c__str25);
                            { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                        }
                    }
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1368); c.m.w32(a_, (v_) as u32); };
                } else {
                    if ((l_down_6) != 0) {
                        if ((((({ let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(1268)).wrapping_add(((({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1336); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) < ((5i32).wrapping_sub(1i32))) as i32)) != 0) {
                            F::UbRank_ClipGoto(c, 0i32, 0i32, sym::menu_n_c__str26);
                            { let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(1268)).wrapping_add(((({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1336); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                            F::UbRank_ClipGoto(c, 0i32, 0i32, sym::menu_n_c__str27);
                            { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                        } else {
                            if ((((({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1380); (c.m.r32(a_) as i32) }) < ((100i32).wrapping_sub(5i32))) as i32)) != 0) {
                                { let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1380); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                { let a0_ = ((c.m.r32(sym::gUbRank)).wrapping_add(60)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_n_c__str28, ((((1i32) as i64 as u64)) as u8)) };
                                { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((0i32) as i64 as u64)])) as i32); };
                                { let v_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1380); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1372); c.m.w32(a_, (v_) as u32); };
                            }
                        }
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1368); c.m.w32(a_, (v_) as u32); };
                    } else {
                        if ((l_ok_7) != 0) {
                            { let v_ = ({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1380); (c.m.r32(a_) as i32) }).wrapping_add({ let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(1268)).wrapping_add(((({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1336); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }); let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1672); c.m.w32(a_, (v_) as u32); };
                            if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1672); (c.m.r32(a_) as i32) }) == ({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(1916); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((7i32) as i64 as u64)])) as i32); };
                            } else {
                                if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1672); (c.m.r32(a_) as i32) }) < ({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(1916); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                                    { let r_ = 8i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1668); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                } else {
                                    { let r_ = (!(8i32)); let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1668); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                                }
                                if (({ let a0_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1672); (c.m.r32(a_) as i32) }; F::UbRank_PickFoe(c, a0_) }) != 0) {
                                    { let a0_ = ((c.m.r32(sym::gUbRank)).wrapping_add(60)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_n_c__str29, ((((1i32) as i64 as u64)) as u8)) };
                                    { let v_ = 1i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1336); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 4i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1340); c.m.w32(a_, (v_) as u32); };
                                } else {
                                    { let v_ = 2i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1336); c.m.w32(a_, (v_) as u32); };
                                    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1668); (c.m.r32(a_) as i32) }) & (8i32))) != 0) {
                                        { let a0_ = ((c.m.r32(sym::gUbRank)).wrapping_add(60)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_n_c__str30, ((((1i32) as i64 as u64)) as u8)) };
                                    } else {
                                        { let r_ = 32i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                                    }
                                }
                                { let v_ = 1i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1376); c.m.w32(a_, (v_) as u32); };
                                { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((1i32) as i64 as u64)])) as i32); };
                            }
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1368); c.m.w32(a_, (v_) as u32); };
                        } else {
                            if ((l_cancel_8) != 0) {
                                F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                                { let v_ = 0i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
                                { let a0_ = ((c.m.r32(sym::gUbRank)).wrapping_add(60)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_n_c__str31, ((((1i32) as i64 as u64)) as u8)) };
                                { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((2i32) as i64 as u64)])) as i32); };
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1368); c.m.w32(a_, (v_) as u32); };
                            } else {
                                { let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1368); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                if ((((({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1368); (c.m.r32(a_) as i32) }) == (1800i32)) as i32)) != 0) {
                                    { let v_ = 6i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1340); c.m.w32(a_, (v_) as u32); };
                                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1368); c.m.w32(a_, (v_) as u32); };
                                }
                            }
                        }
                    }
                }
                break 's1;
            }
            1i32 => {
                if (((({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1264); (c.m.r32(a_) as i32) }) & (128i32))) != 0) {
                    { let v_ = 2i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1336); c.m.w32(a_, (v_) as u32); };
                    { let a0_ = ((c.m.r32(sym::gUbRank)).wrapping_add(60)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_n_c__str32, ((((1i32) as i64 as u64)) as u8)) };
                    { let r_ = (!(128i32)); let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1264); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                }
                break 's1;
            }
            2i32 => {
                if ((((((({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1264); (c.m.r32(a_) as i32) }) & (((16i32) | (32i32))))) == (((16i32) | (32i32)))) as i32)) != 0) {
                    { let v_ = 3i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1336); c.m.w32(a_, (v_) as u32); };
                    { let a0_ = ((c.m.r32(sym::gUbRank)).wrapping_add(60)).wrapping_add((((2i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
                    { let r_ = (!(32i32)); let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1264); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                }
                break 's1;
            }
            3i32 => {
                { let v_ = 4i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1336); c.m.w32(a_, (v_) as u32); };
                { let v_ = 300i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1408); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            4i32 => {
                { let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1408); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                if ((((((l_ok_7) != 0) || ((((({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1408); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0)) as i32)) != 0) {
                    F::Bgm_FadeOutStep(c);
                    { let r_ = 1i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let r_ = 2i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 15i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1324); c.m.w32(a_, (v_) as u32); };
                    if ((l_ok_7) != 0) {
                        { let _ = ((D::call(c, fid::Snd_PlaySe, &[((((1i32) as u32)) as u64), ((1i32) as i64 as u64)])) as i32); };
                    }
                    { let r_ = 4i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1668); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
                break 's1;
            }
            5i32 => {
                if (((({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1264); (c.m.r32(a_) as i32) }) & (32i32))) != 0) {
                    { let r_ = (!(32i32)); let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1264); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
                    if (((((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1668); (c.m.r32(a_) as i32) }) & (8i32))) != 0) && (({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1400); (c.m.r32(a_) as i32) }) != 0)) as i32)) != 0) {
                        { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(1916); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                    }
                    if ((((((((({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1404); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(1916); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0)) as i32)) != 0) {
                        { let a0_ = { let a0_ = ({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(1916); (c.m.r32(a_) as i32) }).wrapping_sub(1i32); F::UbRank_PlaceToRule(c, a0_) }; F::UbRank_SetFoe(c, a0_, 0i32) };
                        { let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1404); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                        if ((((!((F::UbRank_ScrollUp(c)) != 0)) as i32)) != 0) {
                            { let a0_ = ((c.m.r32(sym::gUbRank)).wrapping_add(60)).wrapping_add((((1i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_n_c__str33, ((((1i32) as i64 as u64)) as u8)) };
                        } else {
                            { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1672); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                            { let v_ = 6i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1336); c.m.w32(a_, (v_) as u32); };
                        }
                    } else {
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1336); c.m.w32(a_, (v_) as u32); };
                    }
                }
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "UbRank_Input", &PROBE_N, &[], true); }
    r_
}

pub fn UbRank_ClipGoto(c: &mut Ctx, mut p_movie_1: i32, mut p_state_2: i32, mut p_label_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbRank_ClipGoto", &PROBE_N, &[(p_movie_1) as u32, (p_state_2) as u32, (p_label_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_flash_4: u32 = 0;
    let r_ = 'f: {
        l_flash_4 = ((c.m.r32(sym::gUbRank)).wrapping_add(60)).wrapping_add((((p_movie_1) as u32)).wrapping_mul(44));
        { let _ = { let a2_ = { let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(1268)).wrapping_add((((p_state_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; F::sprintf(c, fp.wrapping_add(8), sym::menu_n_c__str34, &[((a2_) as i64 as u64)]) }; };
        F::Flash_FindLabel(c, l_flash_4, ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, l_flash_4, fp, p_label_3);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "UbRank_ClipGoto", &PROBE_N, &[], true); }
    r_
}

pub fn UbRank_UpdateFaceLoad(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbRank_UpdateFaceLoad", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_p_1: u32 = 0;
    let r_ = 'f: {
        l_p_1 = fp;
        { let _ = F::memset(c, l_p_1, 0i32, ((8i32) as u32)); };
        's1: {
            let sv2: i32 = { let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1376); (c.m.r32(a_) as i32) };
            match sv2 {
                1i32 => {
                    { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(368)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(368)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1116); (c.m.r32(a_) as i32) }; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = { let a_ = { let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1392); c.m.r32(a_) }; (c.m.r32(a_) as i32) }; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    F::File_CancelRequests(c);
                    { let _ = { let a0_ = ((c.m.r32((fp).wrapping_add((((0i32) as u32)).wrapping_mul(4))) as i32)).wrapping_add(761i32); let a1_ = { let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let _ = 92160i32; F::File_Request(c, a0_, a1_) }; };
                    { let _ = { let a0_ = ((c.m.r32((fp).wrapping_add((((1i32) as u32)).wrapping_mul(4))) as i32)).wrapping_add(761i32); let a1_ = { let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let _ = 92160i32; F::File_Request(c, a0_, a1_) }; };
                    { let v_ = 2i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1376); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                2i32 => {
                    if ((F::File_UpdateRequests(c)) != 0) {
                        { let v_ = 3i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1376); c.m.w32(a_, (v_) as u32); };
                    }
                    break 's1;
                }
                3i32 => {
                    { let _ = { let a0_ = { let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(12)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a1_ = { let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, a1_, ((0i32) as u32)) }; };
                    { let _ = { let a0_ = { let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(12)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a1_ = { let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, a1_, ((0i32) as u32)) }; };
                    { let v_ = { let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(20)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = fp.wrapping_add(8); c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp.wrapping_add(8)); let a2_ = c.m.r32(fp.wrapping_add(8)); F::Res_RelocateOffsets(c, fp.wrapping_add(8), a1_, a2_) };
                    { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(8))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(368)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let v_ = { let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(20)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }; let a_ = fp.wrapping_add(8); c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp.wrapping_add(8)); let a2_ = c.m.r32(fp.wrapping_add(8)); F::Res_RelocateOffsets(c, fp.wrapping_add(8), a1_, a2_) };
                    { let v_ = ({ let a_ = (c.m.r32(fp.wrapping_add(8))).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gUbRank)).wrapping_add(368)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                    { let r_ = 16i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1376); c.m.w32(a_, (v_) as u32); };
                    break 's1;
                }
                _ => {
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "UbRank_UpdateFaceLoad", &PROBE_N, &[], true); }
    r_
}

pub fn UbRank_Run(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "UbRank_Run", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = 1i32; c.m.w32(fp, (v_) as u32); }
        F::UbRank_Init(c);
        F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
        'l1: while ((1i32) != 0) {
            F::Gfx_BeginFrame(c);
            F::Pad_Update(c);
            F::Snd_Update(c);
            F::ColorFade_Update(c);
            F::UbRank_UpdateFaceLoad(c);
            F::UbRank_Update(c);
            F::UbRank_UpdateVoice(c);
            F::UbRank_Draw(c);
            F::ColorFade_Draw(c);
            F::Gfx_EndFrame(c, 1i32);
            F::Dma_Flush(c);
            F::File_Stub264D90(c);
            if ((F::ColorFade_IsInDone(c)) != 0) {
                if ((((((((!(((({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1264); (c.m.r32(a_) as i32) }) & (8i32))) != 0)) as i32)) != 0) && (((({ let a_ = (((c.m.r32(sym::gUbRank)).wrapping_add(60)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
                    { let r_ = 8i32; let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1264); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
            }
            if ((F::ColorFade_IsFadingOut(c)) != 0) {
                F::Bgm_FadeOutStep(c);
                F::Voice_FadeOutStep(c, 0i32);
                continue 'l1;
            }
            if ((F::ColorFade_IsOutDone(c)) != 0) {
                break 'l1;
            }
            if (((({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1264); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1324); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                }
            } else {
                if ((((({ let a_ = (c.m.r32(sym::gUbRank)).wrapping_add(1340); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::UbRank_Input(c, fp);
                }
            }
        }
        if (((c.m.r32(fp) as i32)) != 0) {
            F::UbRank_SetupBattle(c);
        }
        F::UbRank_Term(c);
        F::Dma_ResetBuffers(c);
        break 'f (c.m.r32(fp) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "UbRank_Run", &PROBE_N, &[], true); }
    r_
}
