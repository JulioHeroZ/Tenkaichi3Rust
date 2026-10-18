#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn MisSel_SetRank(c: &mut Ctx, mut p_row_1: i32, mut p_rank_2: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MisSel_SetRank", &PROBE_N, &[(p_row_1) as u32, (p_rank_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_flash_3: u32 = 0;
    let r_ = 'f: {
        l_flash_3 = ((c.m.r32(sym::gMisSel)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        { let v_ = (p_rank_2).wrapping_mul(32i32); let a_ = fp.wrapping_add(72); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp.wrapping_add(72)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = ((c.m.r32(fp.wrapping_add(72)) as i32)).wrapping_add(32i32); let a_ = (fp.wrapping_add(72)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(72)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        { let _ = F::sprintf(c, fp.wrapping_add(8), sym::menu_o_d__str0, &[(((p_row_1).wrapping_add(1i32)) as i64 as u64)]); };
        F::Flash_FindLabel(c, l_flash_3, fp.wrapping_add(8), sym::menu_o_d__str1, fp);
        F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(72));
        if (((((p_row_1) == ({ let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1216)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) })) as i32)) != 0) {
            { let v_ = (p_rank_2).wrapping_mul(64i32); let a_ = fp.wrapping_add(72); c.m.w32(a_, (v_) as u32); };
            { let v_ = 0i32; let a_ = (fp.wrapping_add(72)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            { let v_ = ((c.m.r32(fp.wrapping_add(72)) as i32)).wrapping_add(64i32); let a_ = (fp.wrapping_add(72)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = 64i32; let a_ = (fp.wrapping_add(72)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
            F::Flash_FindLabel(c, l_flash_3, ((0i32) as u32), sym::menu_o_d__str2, fp);
            F::Flash_ClipSetUv(c, l_flash_3, fp, fp.wrapping_add(72));
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "MisSel_SetRank", &PROBE_N, &[], true); }
    r_
}

pub fn MisSel_SetupBattle(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MisSel_SetupBattle", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_unk1FC_1: i32 = 0;
    let mut l_unk10_2: i32 = 0;
    let mut l_def_3: u32 = 0;
    let mut l_announcer_4: i32 = 0;
    let mut l_timeLimit_5: i32 = 0;
    let mut l_stage_6: i32 = 0;
    let mut l_bgm_7: i32 = 0;
    let mut l_teamSize_8: i32 = 0;
    let mut l_count_9: u32 = 0;
    let mut l_i_10: i32 = 0;
    let mut l_j_11: i32 = 0;
    let mut l_opp_12: u32 = 0;
    let mut l_item_13: u32 = 0;
    let mut l_id_14: i32 = 0;
    let r_ = 'f: {
        l_def_3 = ({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1000); c.m.r32(a_) }).wrapping_add(((({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1284); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(52));
        l_unk10_2 = ((((c.m.r32((l_def_3).wrapping_add(4)) as i32)) != (0i32)) as i32);
        l_unk1FC_1 = ((((c.m.r32((l_def_3).wrapping_add(20)) as i32)) != (0i32)) as i32);
        l_announcer_4 = (c.m.r32(l_def_3) as i32);
        l_timeLimit_5 = (c.m.r32((l_def_3).wrapping_add(8)) as i32);
        l_stage_6 = (c.m.r32((l_def_3).wrapping_add(12)) as i32);
        l_bgm_7 = (c.m.r32((l_def_3).wrapping_add(16)) as i32);
        if (((((l_announcer_4) == (998i32)) as i32)) != 0) {
            l_announcer_4 = ((F::Rand_Range(c, ((8i32) as u32))) as i32);
        }
        if (((((l_stage_6) == (998i32)) as i32)) != 0) {
            l_stage_6 = ((F::Rand_Range(c, ((35i32) as u32))) as i32);
        }
        if (((((l_bgm_7) == (998i32)) as i32)) != 0) {
            l_bgm_7 = 24i32;
        }
        l_teamSize_8 = 0i32;
        's1: {
            let sv15: i32 = (c.m.r32((l_def_3).wrapping_add(24)) as i32);
            match sv15 {
                0i32 => {
                    l_teamSize_8 = 1i32;
                    break 's1;
                }
                1i32 => {
                    l_teamSize_8 = 2i32;
                    break 's1;
                }
                2i32 => {
                    l_teamSize_8 = 3i32;
                    break 's1;
                }
                3i32 => {
                    l_teamSize_8 = 4i32;
                    break 's1;
                }
                6i32 | 7i32 | 10i32 => {
                    l_teamSize_8 = 5i32;
                    break 's1;
                }
                _ => {
                }
            }
        }
        l_count_9 = ((0i32) as u32);
        {
            l_i_10 = 0i32;
            'l2: loop {
                if !(((((l_i_10) < (5i32)) as i32)) != 0) { break 'l2; }
                'c3: {
                    if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1004); c.m.r32(a_) }).wrapping_add(((((c.m.r32(((l_def_3).wrapping_add(32)).wrapping_add((((l_i_10) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(44)); (c.m.r32(a_) as i32) }) != (999i32)) as i32)) != 0) {
                        { let o_ = l_count_9; let n_ = (o_).wrapping_add(1); l_count_9 = n_; };
                    } else {
                        break 'l2;
                    }
                }
                { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; };
            }
        }
        F::Battle_ClearWork(c);
        F::BattleSetup_SetRule(c, 0i32, 2i32, l_bgm_7, l_timeLimit_5, l_announcer_4, l_stage_6, l_unk10_2);
        F::BattleSetup_SetSide(c, 1i32, 2i32, 1i32, ((l_count_9) as i32), l_unk1FC_1, 1i32, 0i32, ((0i32) as u32));
        {
            l_i_10 = 0i32;
            'l4: loop {
                if !(((((((l_i_10) as u32)) < (l_count_9)) as i32)) != 0) { break 'l4; }
                'c5: {
                    l_opp_12 = ({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1004); c.m.r32(a_) }).wrapping_add(((((c.m.r32(((l_def_3).wrapping_add(32)).wrapping_add((((l_i_10) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(44));
                    { let _ = F::memset(c, fp, 0i32, 16u32); };
                    l_item_13 = fp;
                    {
                        l_j_11 = 0i32;
                        'l6: loop {
                            if !(((((l_j_11) < (7i32)) as i32)) != 0) { break 'l6; }
                            'c7: {
                                l_id_14 = (c.m.r32(((l_opp_12).wrapping_add(16)).wrapping_add((((l_j_11) as u32)).wrapping_mul(4))) as i32);
                                if (((((l_id_14) != (999i32)) as i32)) != 0) {
                                    { let v_ = (((l_id_14).wrapping_add(1i32)) as u16); let a_ = { let o_ = l_item_13; let n_ = (o_).wrapping_add(2); l_item_13 = n_; o_ }; c.m.w16(a_, v_); };
                                }
                            }
                            { let o_ = l_j_11; let n_ = (o_).wrapping_add(1); l_j_11 = n_; };
                        }
                    }
                    if ((((((c.m.r32((l_opp_12).wrapping_add(12)) as i32)) == (999i32)) as i32)) != 0) {
                        { let v_ = ((0i32) as u16); let a_ = (fp).wrapping_add((((7i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
                    } else {
                        { let v_ = ((((c.m.r32((l_opp_12).wrapping_add(12)) as i32)).wrapping_add(1i32)) as u16); let a_ = (fp).wrapping_add((((7i32) as u32)).wrapping_mul(2)); c.m.w16(a_, v_); };
                    }
                    { let a2_ = (c.m.r32(l_opp_12) as i32); let a3_ = (c.m.r32((l_opp_12).wrapping_add(4)) as i32); let a5_ = (c.m.r32((l_opp_12).wrapping_add(8)) as i32); F::BattleSetup_SetMember(c, 1i32, l_i_10, a2_, a3_, 0i32, a5_, F32(0x42c80000), fp) };
                }
                { let o_ = l_i_10; let n_ = (o_).wrapping_add(1); l_i_10 = n_; };
            }
        }
        { let v_ = l_teamSize_8; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1604); c.m.w32(a_, (v_) as u32); };
        { let v_ = (c.m.r32((l_def_3).wrapping_add(28)) as i32); let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1608); c.m.w32(a_, (v_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "MisSel_SetupBattle", &PROBE_N, &[], true); }
    r_
}

pub fn MisSel_Init(c: &mut Ctx, mut p_section_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MisSel_Init", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_i_2: i32 = 0;
    let r_ = 'f: {
        { let v_ = ((0i32) as u32); c.m.w32(fp, v_); }
        { let v_ = F::Heap_Alloc(c, ((1292u32) as i32), ((32i32) as u32), 0i32, 2i32); let a_ = sym::gMisSel; c.m.w32(a_, v_); };
        { let _ = { let a0_ = c.m.r32(sym::gMisSel); F::memset(c, a0_, 0i32, 1292u32) }; };
        { let v_ = (c.m.r32(sym::gMenuArc3)).wrapping_add((({ let a_ = (c.m.r32(sym::gMenuArc3)).wrapping_add((((p_section_1) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = c.m.r32(sym::gMisSel); c.m.w32(a_, v_); };
        { let v_ = { let a0_ = { let a_ = c.m.r32(sym::gMisSel); c.m.r32(a_) }; F::Sprite_Unpack(c, a0_, ((0i32) as u32), ((0i32) as u32)) }; let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(4); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1000); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1004); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = c.m.r32(fp); let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1052); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((33i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((34i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((35i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((0i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((1i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((2i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((13i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((3i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((20i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((4i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((21i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((5i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((6i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((16i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((7i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((8i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((31i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((9i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((10i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((30i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((11i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((12i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((32i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((13i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((26i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((14i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((3i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((15i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((16i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((14i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((17i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((29i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((18i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((4i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((19i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((5i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((20i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((21i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((12i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((22i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((11i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((23i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((6i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((24i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((28i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((25i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((36i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((26i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((37i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(fp)).wrapping_add(16); c.m.r32(a_) }).wrapping_add((((27i32).wrapping_mul(64i32)) as u32)); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((38i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((10i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((19i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((22i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((23i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((24i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((25i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add((((27i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((15i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(8); c.m.w32(a_, v_); };
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (165i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    { let v_ = ({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(8); c.m.r32(a_) }).wrapping_add(((((l_i_2).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
                    { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let a0_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); let a1_ = ({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a2_ = (c.m.r32(sym::gMisSel)).wrapping_add(1056); F::Flash_Create(c, a0_, a1_, a2_) };
        { let a0_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Play(c, a0_, 1i32) };
        { let v_ = rt::rem_i32(F::Rand_Libc(c), 32i32); let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1268); c.m.w32(a_, (v_) as u32); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((8i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = fp; c.m.w32(a_, v_); };
        { let a1_ = c.m.r32(fp); let a2_ = c.m.r32(fp); F::Res_RelocateOffsets(c, fp, a1_, a2_) };
        { let a0_ = ({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((7i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a1_ = c.m.r32(fp); F::IconWin_Init(c, a0_, a1_) };
        F::IconWin_Open(c);
        { let v_ = ({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((18i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(12); c.m.w32(a_, v_); };
        { let v_ = ({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(4); c.m.r32(a_) }).wrapping_add((((17i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(16); c.m.w32(a_, v_); };
        {
            l_i_2 = 0i32;
            'l3: loop {
                if !(((((l_i_2) < (7i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    { let a0_ = ((c.m.r32(sym::gMisSel)).wrapping_add(20)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); let a1_ = { let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(12); c.m.r32(a_) }; F::TextBox_Init(c, a0_, a1_, ((0i32) as u32)) };
                    { let a0_ = ((c.m.r32(sym::gMisSel)).wrapping_add(20)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); F::TextBox_SetUnk50(c, a0_, 0i32) };
                    if (((((l_i_2) < (5i32)) as i32)) != 0) {
                        { let a0_ = ((c.m.r32(sym::gMisSel)).wrapping_add(20)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); F::TextBox_SetMaxWidth(c, a0_, 235i32) };
                        { let a0_ = ((c.m.r32(sym::gMisSel)).wrapping_add(20)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); F::TextBox_SetLineOffsets(c, a0_, 0i32, (10i32).wrapping_neg(), 0i32, 0i32, 0i32) };
                    }
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1260); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1596); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1280); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1600); (c.m.r32(a_) as i32) }; let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1216)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1256); c.m.w32(a_, (v_) as u32); };
        { let v_ = { let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(524); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1276); c.m.w32(a_, (v_) as u32); };
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "MisSel_Init", &PROBE_N, &[], true); }
    r_
}

pub fn MisSel_Term(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MisSel_Term", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    F::IconWin_Term(c);
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1008)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Destroy(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    if ((((({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(4); c.m.r32(a_) }) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = { let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(4); c.m.r32(a_) }; F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(4); c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gMisSel)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gMisSel); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gMisSel; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "MisSel_Term", &PROBE_N, &[], true); }
    r_
}

pub fn MisSel_Draw(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MisSel_Draw", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(96) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 96);
    let mut l_flash_1: u32 = 0;
    let mut l_i_2: i32 = 0;
    let mut l_cur_3: i32 = 0;
    let mut l_chip_4: u32 = 0;
    let mut l_n_5: i32 = 0;
    let mut l_rank_6: i32 = 0;
    let mut l_m_7: i32 = 0;
    let mut l_h_8: i32 = 0;
    let mut l_sec_9: i32 = 0;
    let mut l_score_10: i32 = 0;
    let r_ = 'f: {
        { let a0_ = { let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1052); c.m.r32(a_) }; F::Sprite_DrawPicture(c, a0_, 0i32, 0i32, 128i32) };
        l_flash_1 = ((c.m.r32(sym::gMisSel)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44));
        F::Flash_FindLabel(c, l_flash_1, sym::menu_o_d__str3, sym::menu_o_d__str4, fp);
        { let a2_ = (c.m.r32(sym::gMisSel)).wrapping_add(1268); F::FlashAnim_Blink(c, l_flash_1, fp, a2_, 0i32) };
        F::Flash_FindLabel(c, l_flash_1, sym::menu_o_d__str5, sym::menu_o_d__str6, fp);
        { let a2_ = (c.m.r32(sym::gMisSel)).wrapping_add(1272); F::FlashAnim_Talk(c, l_flash_1, fp, a2_, 0i32) };
        { let a4_ = { let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1276); (c.m.r32(a_) as i32) }; F::Num_Draw(c, l_flash_1, sym::menu_o_d__str7, 0i32, 2i32, a4_, 32i32, 32i32, 1i32) };
        { let a4_ = ({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1280); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::Num_Draw(c, l_flash_1, sym::menu_o_d__str8, 2i32, 2i32, a4_, 32i32, 32i32, 1i32) };
        { let v_ = 32i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_o_d__str9, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        if ((((({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1280); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_o_d__str10, fp);
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
        }
        if ((((({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1280); (c.m.r32(a_) as i32) }) == (({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1276); (c.m.r32(a_) as i32) }).wrapping_sub(1i32))) as i32)) != 0) {
            F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_o_d__str11, fp);
            { let _ = F::Flash_ClipSetFlags(c, l_flash_1, fp, 2i32, ((((0i32) as i64 as u64)) as u8)); };
        }
        {
            l_i_2 = 0i32;
            'l1: loop {
                if !(((((l_i_2) < (5i32)) as i32)) != 0) { break 'l1; }
                'c2: {
                    l_n_5 = (({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1280); (c.m.r32(a_) as i32) }).wrapping_mul(5i32)).wrapping_add(l_i_2);
                    l_rank_6 = (({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(652)).wrapping_add((((l_n_5) as u32)).wrapping_mul(12))).wrapping_add(1); c.m.r8(a_) }) as i32);
                    { let _ = F::sprintf(c, fp.wrapping_add(28), sym::menu_o_d__str12, &[(((l_i_2).wrapping_add(1i32)) as i64 as u64)]); };
                    F::Num_DrawChild(c, l_flash_1, fp.wrapping_add(28), sym::menu_o_d__str13, 0i32, 3i32, (l_n_5).wrapping_add(1i32), 32i32, 32i32, 1i32, 0i32);
                    F::Flash_FindLabel(c, l_flash_1, fp.wrapping_add(28), sym::menu_o_d__str14, fp);
                    { let a5_ = ((c.m.r32(sym::gMisSel)).wrapping_add(20)).wrapping_add((((l_i_2) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, (l_n_5).wrapping_add(216i32), a5_) };
                    F::MisSel_SetRank(c, l_i_2, l_rank_6);
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        l_cur_3 = { let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1284); (c.m.r32(a_) as i32) };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_o_d__str15, fp);
        { let a4_ = ({ let a_ = (({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1000); c.m.r32(a_) }).wrapping_add((((l_cur_3) as u32)).wrapping_mul(52))).wrapping_add(24); (c.m.r32(a_) as i32) }).wrapping_add(328i32); let a5_ = ((c.m.r32(sym::gMisSel)).wrapping_add(20)).wrapping_add((((5i32) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, a4_, a5_) };
        F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_o_d__str16, fp);
        if ((((({ let a_ = (({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1000); c.m.r32(a_) }).wrapping_add((((l_cur_3) as u32)).wrapping_mul(52))).wrapping_add(28); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
            { let a4_ = ({ let a_ = (({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1000); c.m.r32(a_) }).wrapping_add((((l_cur_3) as u32)).wrapping_mul(52))).wrapping_add(28); (c.m.r32(a_) as i32) }).wrapping_add(342i32); let a5_ = ((c.m.r32(sym::gMisSel)).wrapping_add(20)).wrapping_add((((6i32) as u32)).wrapping_mul(140)); F::TextBox_AttachLine(c, l_flash_1, fp, 0i32, 0i32, a4_, a5_) };
        }
        {
            l_i_2 = 0i32;
            'l3: loop {
                if !(((((l_i_2) < (5i32)) as i32)) != 0) { break 'l3; }
                'c4: {
                    if ((((({ let a_ = ({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1004); c.m.r32(a_) }).wrapping_add(((({ let a_ = ((({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1000); c.m.r32(a_) }).wrapping_add((((l_cur_3) as u32)).wrapping_mul(52))).wrapping_add(32)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(44)); (c.m.r32(a_) as i32) }) == (999i32)) as i32)) != 0) {
                        l_chip_4 = ({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((165i32) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
                    } else {
                        l_chip_4 = ({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((({ let a_ = ({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(8); c.m.r32(a_) }).wrapping_add((((({ let a_ = ({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1004); c.m.r32(a_) }).wrapping_add(((({ let a_ = ((({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1000); c.m.r32(a_) }).wrapping_add((((l_cur_3) as u32)).wrapping_mul(52))).wrapping_add(32)).wrapping_add((((l_i_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(44)); (c.m.r32(a_) as i32) }).wrapping_add(1i32)) as u32)).wrapping_mul(4)); c.m.r32(a_) }).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
                    }
                    { let v_ = c.m.r32((l_chip_4).wrapping_add(16)); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1056)).wrapping_add(((((c.m.r32((sym::gMisSelFaceTex).wrapping_add((((l_i_2) as u32)).wrapping_mul(4))) as i32)) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
                }
                { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
            }
        }
        { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
        { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
        { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
        F::Flash_FindLabel(c, l_flash_1, sym::menu_o_d__str17, sym::menu_o_d__str18, fp);
        F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
        {
            l_m_7 = (({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(652)).wrapping_add((((l_cur_3) as u32)).wrapping_mul(12))).wrapping_add(3); c.m.r8(a_) }) as i32);
            l_h_8 = (({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(652)).wrapping_add((((l_cur_3) as u32)).wrapping_mul(12))).wrapping_add(2); c.m.r8(a_) }) as i32);
            l_sec_9 = (({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(652)).wrapping_add((((l_cur_3) as u32)).wrapping_mul(12))).wrapping_add(4); c.m.r8(a_) }) as i32);
            F::Num_Draw(c, l_flash_1, sym::menu_o_d__str19, 4i32, 2i32, l_h_8, 32i32, 32i32, 1i32);
            F::Num_Draw(c, l_flash_1, sym::menu_o_d__str20, 2i32, 2i32, l_m_7, 32i32, 32i32, 1i32);
            F::Num_Draw(c, l_flash_1, sym::menu_o_d__str21, 0i32, 2i32, l_sec_9, 32i32, 32i32, 1i32);
            l_score_10 = ({ let a_ = (((c.m.r32(sym::gSaveData)).wrapping_add(652)).wrapping_add((((l_cur_3) as u32)).wrapping_mul(12))).wrapping_add(8); (c.m.r32(a_) as i32) }).wrapping_mul(100i32);
            { let v_ = 0i32; let a_ = fp.wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = 32i32; let a_ = (fp.wrapping_add(8)).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
            { let v_ = 256i32; let a_ = (fp.wrapping_add(8)).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
            { let v_ = 64i32; let a_ = (fp.wrapping_add(8)).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
            F::Flash_FindLabel(c, l_flash_1, ((0i32) as u32), sym::menu_o_d__str22, fp);
            F::Flash_ClipSetUv(c, l_flash_1, fp, fp.wrapping_add(8));
            F::Num_Draw(c, l_flash_1, sym::menu_o_d__str23, 0i32, 9i32, l_score_10, 32i32, 32i32, 0i32);
        }
        { let a0_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_Draw(c, a0_) };
        F::IconWin_Draw(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "MisSel_Draw", &PROBE_N, &[], true); }
    r_
}

pub fn MisSel_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MisSel_Update", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1232); (c.m.r32(a_) as i32) }) > (0i32)) as i32)) != 0) {
        { let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1232); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
    }
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (1i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a0_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1008)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44)); F::Flash_Advance(c, a0_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "MisSel_Update", &PROBE_N, &[], true); }
    r_
}

pub fn MisSel_UpdateVoice(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MisSel_UpdateVoice", &PROBE_N, &[], false); }
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
        break 'f;
    }
    if ((((({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1256); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
        if ((((({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1248); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
            break 'f;
        }
        if ((((((((((((({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1260); (c.m.r32(a_) as i32) }) != ((1i32).wrapping_neg())) as i32)) != 0) && (((((F::Voice_GetStat(c, 0i32)) != (5i32)) as i32)) != 0)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1248); (c.m.r32(a_) as i32) }) == ({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1252); (c.m.r32(a_) as i32) })) as i32)) != 0)) as i32)) != 0) {
            if ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) != 0) {
                { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                { let v_ = 1i32; let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1256); c.m.w32(a_, (v_) as u32); };
            }
            break 'f;
        }
    } else {
        { let v_ = 0i32; let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1256); c.m.w32(a_, (v_) as u32); };
    }
    's1: {
        let sv1: i32 = { let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1248); (c.m.r32(a_) as i32) };
        match sv1 {
            1i32 => {
                { let v_ = 51i32; let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1260); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = 53i32; let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1260); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            _ => {
            }
        }
    }
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1248); c.m.w32(a_, (v_) as u32); };
    { let a0_ = { let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(16); c.m.r32(a_) }; let a2_ = { let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1260); (c.m.r32(a_) as i32) }; F::Voice_PlayWithSubtitle(c, a0_, 34661i32, a2_) };
    { let v_ = { let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1248); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1252); c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "MisSel_UpdateVoice", &PROBE_N, &[], true); }
    r_
}

pub fn MisSel_Input(c: &mut Ctx, mut p_result_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MisSel_Input", &PROBE_N, &[(p_result_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(64) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 64);
    let mut l_up_2: i32 = 0;
    let mut l_down_3: i32 = 0;
    let mut l_left_4: i32 = 0;
    let mut l_right_5: i32 = 0;
    let mut l_confirm_6: i32 = 0;
    let mut l_cancel_7: i32 = 0;
    let r_ = 'f: {
        l_up_2 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((8i32) as u32)))) as i32);
        l_down_3 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((4i32) as u32)))) as i32);
        l_left_4 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((1i32) as u32)))) as i32);
        l_right_5 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(400))) & (((2i32) as u32)))) as i32);
        l_confirm_6 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((512i32) as u32)))) as i32);
        l_cancel_7 = ((((c.m.r32(((sym::gPad).wrapping_add((((0i32) as u32)).wrapping_mul(448))).wrapping_add(396))) & (((1024i32) as u32)))) as i32);
        if ((((!(((({ let a_ = (((c.m.r32(sym::gMisSel)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
            break 'f;
        }
        if ((((!(((({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1212); (c.m.r32(a_) as i32) }) & (4i32))) != 0)) as i32)) != 0) {
            { let v_ = 1i32; let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1248); c.m.w32(a_, (v_) as u32); };
            F::MisSel_ClipGoto(c, 0i32, 0i32, sym::menu_o_d__str24);
            { let r_ = 4i32; let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1212); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
        }
        's1: {
            let sv8: i32 = { let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1244); (c.m.r32(a_) as i32) };
            match sv8 {
                0i32 => {
                    if ((l_up_2) != 0) {
                        F::MisSel_ClipGoto(c, 0i32, 0i32, sym::menu_o_d__str25);
                        if ((((({ let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1216)).wrapping_add(((({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1244); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); n_ }) < (0i32)) as i32)) != 0) {
                            { let v_ = (5i32).wrapping_sub(1i32); let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1216)).wrapping_add(((({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1244); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                        }
                        F::MisSel_ClipGoto(c, 0i32, 0i32, sym::menu_o_d__str26);
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1288); c.m.w32(a_, (v_) as u32); };
                    } else {
                        if ((l_down_3) != 0) {
                            F::MisSel_ClipGoto(c, 0i32, 0i32, sym::menu_o_d__str27);
                            if ((((({ let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1216)).wrapping_add(((({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1244); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); n_ }) >= (5i32)) as i32)) != 0) {
                                { let v_ = 0i32; let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1216)).wrapping_add(((({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1244); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                            }
                            F::MisSel_ClipGoto(c, 0i32, 0i32, sym::menu_o_d__str28);
                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1288); c.m.w32(a_, (v_) as u32); };
                        } else {
                            if ((l_left_4) != 0) {
                                if ((((({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1280); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                                    { let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1280); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_sub(1); c.m.w32(a_, (n_) as u32); };
                                    { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                                    { let v_ = 0i32; let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1288); c.m.w32(a_, (v_) as u32); };
                                }
                            } else {
                                if ((l_right_5) != 0) {
                                    if ((((((({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1280); (c.m.r32(a_) as i32) }) as u32)) < (((({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1276); (c.m.r32(a_) as i32) }).wrapping_sub(1i32)) as u32))) as i32)) != 0) {
                                        { let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1280); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 0i32); };
                                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1288); c.m.w32(a_, (v_) as u32); };
                                    }
                                } else {
                                    if ((l_confirm_6) != 0) {
                                        { let v_ = (({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1280); (c.m.r32(a_) as i32) }).wrapping_mul(5i32)).wrapping_add({ let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1216)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }); let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1284); c.m.w32(a_, (v_) as u32); };
                                        F::MisSel_ClipGoto(c, 0i32, 0i32, sym::menu_o_d__str29);
                                        { let _ = { let a2_ = ({ let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1216)).wrapping_add(((({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1244); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp, sym::menu_o_d__str30, &[((a2_) as i64 as u64)]) }; };
                                        { let a0_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, fp, ((((1i32) as i64 as u64)) as u8)) };
                                        { let v_ = 1i32; let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1244); c.m.w32(a_, (v_) as u32); };
                                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                                        { let v_ = 0i32; let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1288); c.m.w32(a_, (v_) as u32); };
                                    } else {
                                        if ((l_cancel_7) != 0) {
                                            F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                                            { let v_ = 0i32; let a_ = p_result_1; c.m.w32(a_, (v_) as u32); };
                                            { let a0_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, sym::menu_o_d__str31, ((((1i32) as i64 as u64)) as u8)) };
                                            { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 2i32); };
                                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1288); c.m.w32(a_, (v_) as u32); };
                                        } else {
                                            { let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1288); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                                            if ((((({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1288); (c.m.r32(a_) as i32) }) == (1800i32)) as i32)) != 0) {
                                                { let v_ = 2i32; let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1248); c.m.w32(a_, (v_) as u32); };
                                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1288); c.m.w32(a_, (v_) as u32); };
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    break 's1;
                }
                1i32 => {
                    if ((l_confirm_6) != 0) {
                        { let r_ = 1i32; let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1212); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        { let r_ = 2i32; let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1212); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        { let v_ = 15i32; let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1232); c.m.w32(a_, (v_) as u32); };
                        { let v_ = { let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1280); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1596); c.m.w32(a_, (v_) as u32); };
                        { let v_ = { let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1216)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1600); c.m.w32(a_, (v_) as u32); };
                        { let _ = F::Snd_PlaySe(c, ((1i32) as u32), 1i32); };
                    } else {
                        if ((l_cancel_7) != 0) {
                            { let v_ = 0i32; let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1244); c.m.w32(a_, (v_) as u32); };
                            F::MisSel_ClipGoto(c, 0i32, 0i32, sym::menu_o_d__str32);
                            { let _ = { let a2_ = ({ let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1216)).wrapping_add(((({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1244); (c.m.r32(a_) as i32) }) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp, sym::menu_o_d__str33, &[((a2_) as i64 as u64)]) }; };
                            { let a0_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44)); F::Flash_GotoLabel(c, a0_, fp, ((((1i32) as i64 as u64)) as u8)) };
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
    if rt::probing() { crate::host::harness::probe(c, "MisSel_Input", &PROBE_N, &[], true); }
    r_
}

pub fn MisSel_ClipGoto(c: &mut Ctx, mut p_movie_1: i32, mut p_level_2: i32, mut p_label_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MisSel_ClipGoto", &PROBE_N, &[(p_movie_1) as u32, (p_level_2) as u32, (p_label_3) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(80) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 80);
    let mut l_flash_4: u32 = 0;
    let r_ = 'f: {
        l_flash_4 = ((c.m.r32(sym::gMisSel)).wrapping_add(1008)).wrapping_add((((p_movie_1) as u32)).wrapping_mul(44));
        { let _ = { let a2_ = ({ let a_ = ((c.m.r32(sym::gMisSel)).wrapping_add(1216)).wrapping_add((((p_level_2) as u32)).wrapping_mul(4)); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::sprintf(c, fp.wrapping_add(8), sym::menu_o_d__str34, &[((a2_) as i64 as u64)]) }; };
        F::Flash_FindLabel(c, l_flash_4, ((0i32) as u32), fp.wrapping_add(8), fp);
        F::Flash_ClipGotoLabel(c, l_flash_4, fp, p_label_3);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "MisSel_ClipGoto", &PROBE_N, &[], true); }
    r_
}

pub fn MisSel_Run(c: &mut Ctx, mut p_section_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MisSel_Run", &PROBE_N, &[(p_section_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let r_ = 'f: {
        { let v_ = 1i32; c.m.w32(fp, (v_) as u32); }
        F::MisSel_Init(c, p_section_1);
        F::ColorFade_StartIn(c, 0i32, 0i32, 0i32, 20i32);
        'l1: while ((1i32) != 0) {
            F::Gfx_BeginFrame(c);
            F::Pad_Update(c);
            F::Snd_Update(c);
            F::ColorFade_Update(c);
            F::MisSel_Update(c);
            F::MisSel_UpdateVoice(c);
            F::MisSel_Draw(c);
            F::ColorFade_Draw(c);
            F::Gfx_EndFrame(c, 1i32);
            F::Dma_Flush(c);
            F::File_Stub264D90(c);
            if ((F::ColorFade_IsInDone(c)) != 0) {
                if ((((((((!(((({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1212); (c.m.r32(a_) as i32) }) & (8i32))) != 0)) as i32)) != 0) && (((({ let a_ = (((c.m.r32(sym::gMisSel)).wrapping_add(1008)).wrapping_add((((0i32) as u32)).wrapping_mul(44))).wrapping_add(8); c.m.r32(a_) }) & (((2i32) as u32)))) != 0)) as i32)) != 0) {
                    { let r_ = 8i32; let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1212); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
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
            if (((({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1212); (c.m.r32(a_) as i32) }) & (2i32))) != 0) {
                if ((((({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1232); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::ColorFade_StartOut(c, 0i32, 0i32, 0i32, 20i32);
                }
            } else {
                if ((((({ let a_ = (c.m.r32(sym::gMisSel)).wrapping_add(1248); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                    F::MisSel_Input(c, fp);
                }
            }
        }
        if (((c.m.r32(fp) as i32)) != 0) {
            F::MisSel_SetupBattle(c);
        }
        F::MisSel_Term(c);
        F::Dma_ResetBuffers(c);
        break 'f (c.m.r32(fp) as i32);
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "MisSel_Run", &PROBE_N, &[], true); }
    r_
}
