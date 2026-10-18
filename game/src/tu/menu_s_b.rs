#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn SurvSel_SetupBattle(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SurvSel_SetupBattle", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_def_1: u32 = 0;
    let mut l_unk10_2: i32 = 0;
    let mut l_unk1FC_3: i32 = 0;
    let mut l_announcer_4: i32 = 0;
    let mut l_timeLimit_5: i32 = 0;
    let mut l_stage_6: i32 = 0;
    let mut l_bgm_7: i32 = 0;
    let mut l_i_8: i32 = 0;
    let mut l_opp_9: u32 = 0;
    let mut l_chara_10: i32 = 0;
    let mut l_n_11: i32 = 0;
    let mut l_j_12: i32 = 0;
    let mut l_id_13: i32 = 0;
    let r_ = 'f: {
        l_def_1 = ({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(740); c.m.r32(a_) }).wrapping_add(((({ let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(756)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(224));
        l_unk10_2 = ((((c.m.r32((l_def_1).wrapping_add(4)) as i32)) != (0i32)) as i32);
        l_unk1FC_3 = ((((c.m.r32((l_def_1).wrapping_add(20)) as i32)) != (0i32)) as i32);
        l_announcer_4 = (c.m.r32(l_def_1) as i32);
        l_timeLimit_5 = (c.m.r32((l_def_1).wrapping_add(8)) as i32);
        l_stage_6 = (c.m.r32((l_def_1).wrapping_add(12)) as i32);
        l_bgm_7 = (c.m.r32((l_def_1).wrapping_add(16)) as i32);
        if (((((l_announcer_4) == (998i32)) as i32)) != 0) {
            l_announcer_4 = ((F::Rand_Range(c, ((8i32) as u32))) as i32);
        }
        if (((((l_stage_6) == (998i32)) as i32)) != 0) {
            l_stage_6 = ((F::Rand_Range(c, ((35i32) as u32))) as i32);
        }
        if (((((l_bgm_7) == (998i32)) as i32)) != 0) {
            l_bgm_7 = 24i32;
        }
        F::Battle_ClearWork(c);
        F::BattleSetup_SetRule(c, 0i32, 3i32, l_bgm_7, l_timeLimit_5, l_announcer_4, l_stage_6, l_unk10_2);
        F::BattleSetup_SetSide(c, 0i32, 0i32, 0i32, 1i32, 1i32, 1i32, 0i32, ((0i32) as u32));
        F::BattleSetup_SetSide(c, 1i32, 2i32, 1i32, 1i32, l_unk1FC_3, 1i32, 0i32, ((0i32) as u32));
        F::BattleSetup_SetMember(c, 1i32, 0i32, 0i32, 0i32, 0i32, 0i32, F32(0x42c80000), ((0i32) as u32));
        {
            l_i_8 = 0i32;
            'l1: loop {
                if !(((((l_i_8) < (50i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_opp_9 = ({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(748); c.m.r32(a_) }).wrapping_add(((((c.m.r32(((l_def_1).wrapping_add(24)).wrapping_add((((l_i_8) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(44));
                    l_chara_10 = (c.m.r32(l_opp_9) as i32);
                    l_chara_10 = (if (((((l_chara_10) == (998i32)) as i32)) != 0) { { let a_ = ({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(744); c.m.r32(a_) }).wrapping_add((F::Rand_Range(c, ((102i32) as u32))).wrapping_mul(4)); (c.m.r32(a_) as i32) } } else { l_chara_10 });
                    { let _ = F::memset(c, fp, 0i32, 16u32); };
                    l_n_11 = 0i32;
                    {
                        l_j_12 = 0i32;
                        'l3: loop {
                            if !(((((l_j_12) < (7i32)) as i32)) != 0) { break 'l3; }
                            'c4: {
                                l_id_13 = (c.m.r32(((l_opp_9).wrapping_add(16)).wrapping_add((((l_j_12) as u32)).wrapping_mul(4))) as i32);
                                if (((((l_id_13) != (999i32)) as i32)) != 0) {
                                    { let v_ = (((l_id_13).wrapping_add(1i32)) as u16); let a_ = (fp).wrapping_add(((({ let o_ = l_n_11; let n_ = (o_).wrapping_add(1); l_n_11 = n_; o_ }) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
                                }
                            }
                            { let o_ = l_j_12; let n_ = (o_).wrapping_add(1); l_j_12 = n_; };
                        }
                    }
                    if ((((((c.m.r32((l_opp_9).wrapping_add(12)) as i32)) == (999i32)) as i32)) != 0) {
                        { let v_ = ((0i32) as u16); let a_ = (fp).wrapping_add((((7i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
                    } else {
                        { let v_ = ((((c.m.r32((l_opp_9).wrapping_add(12)) as i32)).wrapping_add(1i32)) as u16); let a_ = (fp).wrapping_add((((7i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
                    }
                    { let a3_ = (c.m.r32((l_opp_9).wrapping_add(4)) as i32); let a5_ = (c.m.r32((l_opp_9).wrapping_add(8)) as i32); F::BattleSetup_SetPoolMember(c, 50i32, l_i_8, l_chara_10, a3_, 0i32, a5_, F32(0x42c80000), fp) };
                }
                { let o_ = l_i_8; let n_ = (o_).wrapping_add(1); l_i_8 = n_; };
            }
        }
        { let v_ = 1i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1604); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1608); c.m.w32(a_, (v_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SurvSel_SetupBattle", &PROBE_N, &[], true); }
    r_
}

pub fn SurvSel_Init(c: &mut Ctx, mut p_section_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SurvSel_Init", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = F::Heap_Alloc(c, ((812u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gSurvSel; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gSurvSel); F::memset(c, a0_, 0i32, 812u32) }; };
        { let v_ = (c.m.r32(sym::gMenuArc3)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc3)).wrapping_add((((p_section_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = c.m.r32(sym::gSurvSel); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gSurvSel); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(740); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(744); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(748); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = c.m.r32(fp); let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(620); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(624)).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(624)).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(624)).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(624)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(624)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(624)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(624)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((7i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(624)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((8i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(624)).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((10i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(624)).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((11i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(624)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((12i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(624)).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((13i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(624)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((14i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(624)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((15i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(624)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((16i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(624)).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((17i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(624)).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((18i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(624)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((19i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(624)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((20i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(624)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((21i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(624)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((22i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(624)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((23i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(624)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((25i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(624)).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((26i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(624)).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((27i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(624)).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(624)).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(624)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(624)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let a0_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(576)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gSurvSel)).wrapping_add(624); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(576)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let v_ = rt::rem_i32(F::Rand_Libc(c), 32i32); let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(800); c.m.w32(a_, (v_) as u32); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((9i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = c.m.r32(fp); F::IconWin_Init(c, a0_, a1_) };
        F::IconWin_Open(c);
        { let v_ = ({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(8); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(12); c.m.w32(a_, v_); };
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (4i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let a0_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(16)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); let a1_ = { let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(8); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, ((0i32) as u32)) };
                    { let a0_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(16)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); F::TextBox_SetUnk50(c, a0_, 0i32) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(792); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(788); c.m.w32(a_, (v_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SurvSel_Init", &PROBE_N, &[], true); }
    r_
}

pub fn SurvSel_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SurvSel_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    F::IconWin_Term(c);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(576)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Destroy(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gSurvSel)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gSurvSel); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gSurvSel; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SurvSel_Term", &PROBE_N, &[], true); }
    r_
}

pub fn SurvSel_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SurvSel_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_flash_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_n_3: i32 = 0;
    let mut l_t0_4: i32 = 0;
    let mut l_t1_5: i32 = 0;
    let mut l_t2_6: i32 = 0;
    let mut l_rank_7: i32 = 0;
    let r_ = 'f: {
        { let a0_ = { let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(620); c.m.r32(a_) }; F::Sprite_DrawPicture(c, a0_, 0i32, 0i32, 128i32) };
        l_flash_1 = ((c.m.r32(sym::gSurvSel)).wrapping_add(576)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        F::Flash_FindLabel(c, l_flash_1, sym::menu_s_b__str0, sym::menu_s_b__str1, fp);
        { let a2_ = (c.m.r32(sym::gSurvSel)).wrapping_add(800); F::FlashAnim_Blink(c, l_flash_1, fp, a2_, 0i32) };
        F::Flash_FindLabel(c, l_flash_1, sym::menu_s_b__str2, sym::menu_s_b__str3, fp);
        { let a2_ = (c.m.r32(sym::gSurvSel)).wrapping_add(804); F::FlashAnim_Talk(c, l_flash_1, fp, a2_, 0i32) };
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (3i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_s_b__str4, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_s_b__str5, fp);
                    { let a5_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(16)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, (l_i_2).wrapping_add(316i32), a5_) };
                    l_rank_7 = (({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(1852)).wrapping_add((((l_i_2) as u32)).wrapping_mul(12))).wrapping_add(8); c.m.r8(a_) }) as i32);
                    { let v_ = (l_rank_7).wrapping_mul(32i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_s_b__str6, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_s_b__str7, fp);
                    F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
                    if (((((l_i_2) == ({ let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(756)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
                        l_rank_7 = (({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(1852)).wrapping_add((((l_i_2) as u32)).wrapping_mul(12))).wrapping_add(8); c.m.r8(a_) }) as i32);
                        { let v_ = (l_rank_7).wrapping_mul(64i32); let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                        { let v_ = ((c.m.r32(fp.wrapping_add(8)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_s_b__str8, fp);
                        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        {
            l_i_2 = 3i32;
            'l3: loop {
                if !(((((l_i_2) < (5i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_s_b__str9, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), fp.wrapping_add(28), fp);
                    { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_s_b__str10, fp);
        { let a5_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(16)).wrapping_add((((3i32) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, 336i32, a5_) };
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, sym::menu_s_b__str11, sym::menu_s_b__str12, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        l_n_3 = { let a_ = ((c.m.r32(sym::gSaveData)).wrapping_add(1852)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(756)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(12)); (c.m.r32(a_) as i32) };
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 96i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 128i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_s_b__str13, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        F::Num_Draw(c, l_flash_1, sym::menu_s_b__str14, 0i32, 3i32, l_n_3, 32i32, 32i32, 0i32);
        l_t1_5 = (({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(1852)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(756)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(12))).wrapping_add(10); c.m.r8(a_) }) as i32);
        l_t0_4 = (({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(1852)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(756)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(12))).wrapping_add(9); c.m.r8(a_) }) as i32);
        l_t2_6 = (({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(1852)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(756)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(12))).wrapping_add(11); c.m.r8(a_) }) as i32);
        F::Num_Draw(c, l_flash_1, sym::menu_s_b__str15, 4i32, 2i32, l_t0_4, 32i32, 32i32, 1i32);
        F::Num_Draw(c, l_flash_1, sym::menu_s_b__str16, 2i32, 2i32, l_t1_5, 32i32, 32i32, 1i32);
        F::Num_Draw(c, l_flash_1, sym::menu_s_b__str17, 0i32, 2i32, l_t2_6, 32i32, 32i32, 1i32);
        l_n_3 = ({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(1852)).wrapping_add(((({ let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(756)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(12))).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_mul(100i32);
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_s_b__str18, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        F::Num_Draw(c, l_flash_1, sym::menu_s_b__str19, 0i32, 9i32, l_n_3, 32i32, 32i32, 0i32);
        { let a0_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(576)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
        F::IconWin_Draw(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SurvSel_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn SurvSel_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SurvSel_Update", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(764); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
        { let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(764); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    }
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(576)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Advance(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SurvSel_Update", &PROBE_N, &[], true); }
    r_
}

pub fn SurvSel_UpdateVoice(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SurvSel_UpdateVoice", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(788); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        if ((((({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(780); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            break 'f;
        }
        if ((((((((((((({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(792); (c.m.r32(a_) as i32) }) != ((1i32).wrapping_neg())) as i32)) != 0) && (((((F::Voice_GetStat(c, 0i32)) != (5i32)) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(780); (c.m.r32(a_) as i32) }) == ({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(784); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                { let v_ = 1i32; let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(788); c.m.w32(a_, (v_) as u32); };
            }
            break 'f;
        }
    } else {
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(788); c.m.w32(a_, (v_) as u32); };
    }
    's1: {
        let sv1: i32 = { let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(780); (c.m.r32(a_) as i32) };
        match sv1 {
            1i32 => {
                { let v_ = 52i32; let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(792); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = 53i32; let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(792); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(780); c.m.w32(a_, (v_) as u32); };
    { let a0_ = { let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(12); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(792); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34661i32, a2_) };
    { let v_ = { let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(780); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(784); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SurvSel_UpdateVoice", &PROBE_N, &[], true); }
    r_
}

pub fn SurvSel_Input(c: &mut Ctx, mut p_result_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SurvSel_Input", &PROBE_N, &[(p_result_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_up_2: i32 = 0;
    let mut l_down_3: i32 = 0;
    let mut l_confirm_4: i32 = 0;
    let mut l_cancel_5: i32 = 0;
    let r_ = 'f: {
        l_up_2 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) as i32);
        l_down_3 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) as i32);
        l_confirm_4 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) as i32);
        l_cancel_5 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) as i32);
        if ((((!(((({ let a_ = (((c.m.r32(sym::gSurvSel)).wrapping_add(576)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
            break 'f;
        }
        if ((((!(((({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(752); (c.m.r32(a_) as i32) }) & (4i32))) != 0)) as i32)) != 0) {
            { let v_ = 1i32; let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(780); c.m.w32(a_, (v_) as u32); };
            F::SurvSel_ClipGoto(c, 0i32, 0i32, sym::menu_s_b__str20);
            { let r_ = 4i32; let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(752); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        's1: {
            let sv6: i32 = { let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(776); (c.m.r32(a_) as i32) };
            match sv6 {
                0i32 => {
                    if ((l_up_2) != 0) {
                        F::SurvSel_ClipGoto(c, 0i32, 0i32, sym::menu_s_b__str21);
                        if ((((({ let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(756)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(776); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) < (0i32)) as i32)) != 0) {
                            { let v_ = (3i32).wrapping_sub(1i32); let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(756)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(776); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        F::SurvSel_ClipGoto(c, 0i32, 0i32, sym::menu_s_b__str22);
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(808); c.m.w32(a_, (v_) as u32); };
                    } else {
                        if ((l_down_3) != 0) {
                            F::SurvSel_ClipGoto(c, 0i32, 0i32, sym::menu_s_b__str23);
                            if ((((({ let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(756)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(776); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) >= (3i32)) as i32)) != 0) {
                                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(756)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(776); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            }
                            F::SurvSel_ClipGoto(c, 0i32, 0i32, sym::menu_s_b__str24);
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(808); c.m.w32(a_, (v_) as u32); };
                        } else {
                            if ((l_confirm_4) != 0) {
                                F::SurvSel_ClipGoto(c, 0i32, 0i32, sym::menu_s_b__str25);
                                { let _ = { let a2_ = ({ let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(756)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(776); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp, sym::menu_s_b__str26, &[((a2_) as i64 as u64)]) }; };
                                { let a0_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(576)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, fp, ((((1i32) as i64 as u64)) as u8)) };
                                { let v_ = 1i32; let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(776); c.m.w32(a_, (v_) as u32); };
                                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(808); c.m.w32(a_, (v_) as u32); };
                            } else {
                                if ((l_cancel_5) != 0) {
                                    F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                                    { let v_ = 0i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
                                    { let a0_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(576)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_s_b__str27, ((((1i32) as i64 as u64)) as u8)) };
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(808); c.m.w32(a_, (v_) as u32); };
                                } else {
                                    { let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(808); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                    if ((((({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(808); (c.m.r32(a_) as i32) }) == (1800i32)) as i32)) != 0) {
                                        { let v_ = 2i32; let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(780); c.m.w32(a_, (v_) as u32); };
                                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(808); c.m.w32(a_, (v_) as u32); };
                                    }
                                }
                            }
                        }
                    }
                    break 's1;
                }
                1i32 => {
                    if ((l_confirm_4) != 0) {
                        { let r_ = 1i32; let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(752); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        { let r_ = 2i32; let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(752); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        { let v_ = 15i32; let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(764); c.m.w32(a_, (v_) as u32); };
                        { let v_ = { let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(756)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1600); c.m.w32(a_, (v_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    } else {
                        if ((l_cancel_5) != 0) {
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(776); c.m.w32(a_, (v_) as u32); };
                            F::SurvSel_ClipGoto(c, 0i32, 0i32, sym::menu_s_b__str28);
                            { let _ = { let a2_ = ({ let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(756)).wrapping_add(((({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(776); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp, sym::menu_s_b__str29, &[((a2_) as i64 as u64)]) }; };
                            { let a0_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(576)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, fp, ((((1i32) as i64 as u64)) as u8)) };
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                        }
                    }
                    break 's1;
                }
                _ => {
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SurvSel_Input", &PROBE_N, &[], true); }
    r_
}

pub fn SurvSel_ClipGoto(c: &mut Ctx, mut p_movie_1: i32, mut p_level_2: i32, mut p_label_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SurvSel_ClipGoto", &PROBE_N, &[(p_movie_1) as u32, (p_level_2) as u32, (p_label_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_flash_4: u32 = 0;
    let r_ = 'f: {
        l_flash_4 = ((c.m.r32(sym::gSurvSel)).wrapping_add(576)).wrapping_add((((p_movie_1) as u32)).wrapping_mul(44));
        { let _ = { let a2_ = ({ let a_ = ((c.m.r32(sym::gSurvSel)).wrapping_add(756)).wrapping_add((((p_level_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_s_b__str30, &[((a2_) as i64 as u64)]) }; };
        F::Flash_FindLabel(c, l_flash_4, ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, l_flash_4, fp, p_label_3);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SurvSel_ClipGoto", &PROBE_N, &[], true); }
    r_
}

pub fn SurvSel_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SurvSel_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = 1i32; c.m.w32(fp, (v_) as u32); }
        F::SurvSel_Init(c, p_section_1);
        F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
        'l1: while ((1i32) != 0) {
            F::Gfx_BeginFrame(c);
            F::Pad_Update(c);
            F::Snd_Update(c);
            F::ColorFade_Update(c);
            F::SurvSel_Update(c);
            F::SurvSel_UpdateVoice(c);
            F::SurvSel_Draw(c);
            F::ColorFade_Draw(c);
            F::Gfx_EndFrame(c, 1i32);
            F::Dma_Flush(c);
            F::File_Stub264D90(c);
            if ((F::ColorFade_IsInDone(c)) != 0) {
                if ((((((((!(((({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(752); (c.m.r32(a_) as i32) }) & (8i32))) != 0)) as i32)) != 0) && (((({ let a_ = (((c.m.r32(sym::gSurvSel)).wrapping_add(576)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
                    { let r_ = 8i32; let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(752); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
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
            if (((({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(752); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(764); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                }
            } else {
                if ((((({ let a_ = (c.m.r32(sym::gSurvSel)).wrapping_add(780); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::SurvSel_Input(c, fp);
                }
            }
        }
        if (((c.m.r32(fp) as i32)) != 0) {
            F::SurvSel_SetupBattle(c);
        }
        F::SurvSel_Term(c);
        F::Dma_ResetBuffers(c);
        break 'f (c.m.r32(fp) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "SurvSel_Run", &PROBE_N, &[], true); }
    r_
}
