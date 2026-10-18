#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Voice_PlayWithSubtitle(c: &mut Ctx, mut p_pack_1: u32, mut p_base_2: i32, mut p_line_3: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Voice_PlayWithSubtitle", &PROBE_N, &[(p_pack_1) as u32, (p_base_2) as u32, (p_line_3) as u32], false); }
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5640); (c.m.r32(a_) as i32) }) & (1i32))) != 0) {
        F::Voice_Play(c, 0i32, ((p_base_2).wrapping_add(p_line_3)).wrapping_add(1372i32), 128i32, 0i32);
        if (((((p_pack_1) != (((0i32) as u32))) as i32)) != 0) {
            { let a0_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((((p_line_3).wrapping_mul(2i32)).wrapping_add(2i32)) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::LipSync_Start(c, a0_) };
        }
    } else {
        F::Voice_Play(c, 0i32, (p_base_2).wrapping_add(p_line_3), 128i32, 0i32);
        if (((((p_pack_1) != (((0i32) as u32))) as i32)) != 0) {
            { let a0_ = (p_pack_1).wrapping_add(((c.m.r32((p_pack_1).wrapping_add((((((p_line_3).wrapping_mul(2i32)).wrapping_add(1i32)) as u32)).wrapping_mul(4)))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32)); F::LipSync_Start(c, a0_) };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Voice_PlayWithSubtitle", &PROBE_N, &[], true); }
    r_
}

pub fn Voice_StopWithLip(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Voice_StopWithLip", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Voice_Stop(c, 0i32);
    F::LipSync_Clear(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Voice_StopWithLip", &PROBE_N, &[], true); }
    r_
}

pub fn SndOpt_Apply(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SndOpt_Apply", &PROBE_N, &[], false); }
    let r_ = 'f: {
    F::Bgm_SetVolume(c, 64i32);
    { let a0_ = (({ let a_ = (c.m.r32(sym::gSaveData)).wrapping_add(5796); (c.m.r32(a_) as i32) }) ^ (1i32)); F::Snd_SetStereo(c, a0_) };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "SndOpt_Apply", &PROBE_N, &[], true); }
    r_
}

pub fn Progress_ClearTeams(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Progress_ClearTeams", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_j_2: i32 = 0;
    let mut l_k_3: i32 = 0;
    let r_ = 'f: {
    { let _ = { let a0_ = (c.m.r32(sym::gProgress)).wrapping_add(1692); F::memset(c, a0_, 0i32, 308u32) }; };
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (7i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                {
                    l_j_2 = 0i32;
                    'l3: loop {
                        if !(((((l_j_2) < (2i32)) as i32)) != 0) { break 'l3; }
                        'c4: {
                            {
                                l_k_3 = 0i32;
                                'l5: loop {
                                    if !(((((l_k_3) < (5i32)) as i32)) != 0) { break 'l5; }
                                    'c6: {
                                        { let v_ = 164i32; let a_ = (((((c.m.r32(sym::gProgress)).wrapping_add(1692)).wrapping_add((((l_i_1) as u32)).wrapping_mul(44))).wrapping_add(4)).wrapping_add((((l_j_2) as u32)).wrapping_mul(20))).wrapping_add((((l_k_3) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                                    }
                                    { let o_ = l_k_3; let n_ = (o_).wrapping_add(1); l_k_3 = n_; };
                                }
                            }
                        }
                        { let o_ = l_j_2; let n_ = (o_).wrapping_add(1); l_j_2 = n_; };
                    }
                }
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Progress_ClearTeams", &PROBE_N, &[], true); }
    r_
}

pub fn ChrTbl_IsRelated(c: &mut Ctx, mut p_a_1: i32, mut p_b_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChrTbl_IsRelated", &PROBE_N, &[(p_a_1) as u32, (p_b_2) as u32], false); }
    let mut l_file_3: u32 = 0;
    let mut l_table_4: u32 = 0;
    let mut l_i_5: i32 = 0;
    let r_ = 'f: {
    if (((((((((((((((((((((((((p_a_1) == (79i32)) as i32)) != 0) || (((((p_a_1) == (109i32)) as i32)) != 0)) as i32)) != 0) || (((((p_a_1) == (128i32)) as i32)) != 0)) as i32)) != 0) || (((((p_b_2) == (79i32)) as i32)) != 0)) as i32)) != 0) || (((((p_b_2) == (109i32)) as i32)) != 0)) as i32)) != 0) || (((((p_b_2) == (128i32)) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    l_file_3 = { let a_ = ((c.m.r32(sym::gCommonRes)).wrapping_add(4)).wrapping_add((((2i32) as u32)).wrapping_mul(4)); c.m.r32(a_) };
    l_table_4 = (l_file_3).wrapping_add(((c.m.r32((l_file_3).wrapping_add(4))).wrapping_shr((2i32) as u32)).wrapping_shl((2i32) as u32));
    if (((((p_a_1) == (p_b_2)) as i32)) != 0) {
        break 'f 1i32;
    }
    {
        l_i_5 = 0i32;
        'l1: loop {
            if !(((((l_i_5) < (4i32)) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((((((((((c.m.r8((((l_table_4).wrapping_add((((p_a_1) as u32)).wrapping_mul(60))).wrapping_add(44)).wrapping_add(((l_i_5) as u32)))) as i32)) != (255i32)) as i32)) != 0) && (((((((c.m.r8((((l_table_4).wrapping_add((((p_a_1) as u32)).wrapping_mul(60))).wrapping_add(44)).wrapping_add(((l_i_5) as u32)))) as i32)) == (p_b_2)) as i32)) != 0)) as i32)) != 0) {
                    break 'f 1i32;
                }
                if (((((((((((c.m.r8((((l_table_4).wrapping_add((((p_b_2) as u32)).wrapping_mul(60))).wrapping_add(44)).wrapping_add(((l_i_5) as u32)))) as i32)) != (255i32)) as i32)) != 0) && (((((((c.m.r8((((l_table_4).wrapping_add((((p_b_2) as u32)).wrapping_mul(60))).wrapping_add(44)).wrapping_add(((l_i_5) as u32)))) as i32)) == (p_a_1)) as i32)) != 0)) as i32)) != 0) {
                    break 'f 1i32;
                }
            }
            { let o_ = l_i_5; let n_ = (o_).wrapping_add(1); l_i_5 = n_; };
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ChrTbl_IsRelated", &PROBE_N, &[], true); }
    r_
}

pub fn CpuLevel_FromSetting(c: &mut Ctx, mut p_setting_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CpuLevel_FromSetting", &PROBE_N, &[(p_setting_1) as u32], false); }
    let mut l_level_2: i32 = 0;
    let r_ = 'f: {
    l_level_2 = 0i32;
    's1: {
        let sv3: u32 = p_setting_1;
        match sv3 {
            0u32 => {
                l_level_2 = 0i32;
                break 's1;
            }
            1u32 => {
                l_level_2 = 6i32;
                break 's1;
            }
            2u32 => {
                l_level_2 = 13i32;
                break 's1;
            }
            3u32 => {
                l_level_2 = 21i32;
                break 's1;
            }
            4u32 => {
                l_level_2 = 29i32;
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f l_level_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "CpuLevel_FromSetting", &PROBE_N, &[], true); }
    r_
}

pub fn CpuLevel_ToSetting(c: &mut Ctx, mut p_level_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "CpuLevel_ToSetting", &PROBE_N, &[(p_level_1) as u32], false); }
    let mut l_setting_2: i32 = 0;
    let r_ = 'f: {
    l_setting_2 = 0i32;
    's1: {
        let sv3: i32 = p_level_1;
        match sv3 {
            (-1i32) => {
                l_setting_2 = (1i32).wrapping_neg();
                break 's1;
            }
            0i32 => {
                l_setting_2 = 0i32;
                break 's1;
            }
            6i32 => {
                l_setting_2 = 1i32;
                break 's1;
            }
            13i32 => {
                l_setting_2 = 2i32;
                break 's1;
            }
            21i32 => {
                l_setting_2 = 3i32;
                break 's1;
            }
            29i32 => {
                l_setting_2 = 4i32;
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f l_setting_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "CpuLevel_ToSetting", &PROBE_N, &[], true); }
    r_
}

pub fn Demo_SetupBattle(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Demo_SetupBattle", &PROBE_N, &[], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(16) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 16);
    let mut l_bgm_1: i32 = 0;
    let mut l_stage_2: i32 = 0;
    let mut l_pick_3: i32 = 0;
    let mut l_i_4: i32 = 0;
    let mut l_size_5: i32 = 0;
    let r_ = 'f: {
        l_bgm_1 = 0i32;
        l_stage_2 = 0i32;
        l_size_5 = ((8u32) as i32);
        { let _ = F::memset(c, fp, 0i32, ((l_size_5) as u32)); };
        { let _ = F::memset(c, fp.wrapping_add(8), 0i32, ((l_size_5) as u32)); };
        'l1: loop {
            'c2: {
                l_pick_3 = ((F::Rand_Range(c, ((9i32) as u32))) as i32);
            }
            if !((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(36); (c.m.r32(a_) as i32) }) == (l_pick_3)) as i32)) != 0) { break 'l1; }
        }
        { let v_ = l_pick_3; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(36); c.m.w32(a_, (v_) as u32); };
        's3: {
            let sv6: i32 = l_pick_3;
            match sv6 {
                0i32 => {
                    { let v_ = 10i32; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 147i32; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_stage_2 = 29i32;
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_bgm_1 = (((F::Rand_Range(c, ((9i32) as u32))).wrapping_add(((8i32) as u32))) as i32);
                    break 's3;
                }
                1i32 => {
                    { let v_ = 42i32; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 108i32; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_stage_2 = 6i32;
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_bgm_1 = (((F::Rand_Range(c, ((9i32) as u32))).wrapping_add(((8i32) as u32))) as i32);
                    break 's3;
                }
                2i32 => {
                    { let v_ = 25i32; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 92i32; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_stage_2 = 2i32;
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_bgm_1 = (((F::Rand_Range(c, ((9i32) as u32))).wrapping_add(((8i32) as u32))) as i32);
                    break 's3;
                }
                3i32 => {
                    { let v_ = 39i32; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 96i32; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_stage_2 = 1i32;
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_bgm_1 = (((F::Rand_Range(c, ((9i32) as u32))).wrapping_add(((8i32) as u32))) as i32);
                    break 's3;
                }
                4i32 => {
                    { let v_ = 23i32; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 111i32; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_stage_2 = 1i32;
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_bgm_1 = (((F::Rand_Range(c, ((9i32) as u32))).wrapping_add(((8i32) as u32))) as i32);
                    break 's3;
                }
                5i32 => {
                    { let v_ = 73i32; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 91i32; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_stage_2 = 16i32;
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_bgm_1 = (((F::Rand_Range(c, ((9i32) as u32))).wrapping_add(((8i32) as u32))) as i32);
                    break 's3;
                }
                6i32 => {
                    { let v_ = 0i32; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 30i32; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_stage_2 = 25i32;
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_bgm_1 = (((F::Rand_Range(c, ((9i32) as u32))).wrapping_add(((8i32) as u32))) as i32);
                    break 's3;
                }
                7i32 => {
                    { let v_ = 50i32; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 114i32; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_stage_2 = 5i32;
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_bgm_1 = (((F::Rand_Range(c, ((9i32) as u32))).wrapping_add(((8i32) as u32))) as i32);
                    break 's3;
                }
                8i32 => {
                    { let v_ = 5i32; let a_ = (fp).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 37i32; let a_ = (fp).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_stage_2 = 1i32;
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = (fp.wrapping_add(8)).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, (v_) as u32); };
                    l_bgm_1 = (((F::Rand_Range(c, ((9i32) as u32))).wrapping_add(((8i32) as u32))) as i32);
                    break 's3;
                }
                _ => {
                }
            }
        }
        F::Battle_ClearWork(c);
        F::BattleSetup_SetRule(c, 0i32, 7i32, l_bgm_1, 5i32, 4i32, l_stage_2, 0i32);
        F::BattleSetup_SetSide(c, 0i32, 2i32, 0i32, 1i32, 1i32, 1i32, 0i32, ((0i32) as u32));
        F::BattleSetup_SetSide(c, 1i32, 2i32, 1i32, 1i32, 1i32, 1i32, 0i32, ((0i32) as u32));
        {
            l_i_4 = 0i32;
            'l4: loop {
                if !(((((l_i_4) < (2i32)) as i32)) != 0) { break 'l4; }
                'c5: {
                    { let a2_ = (c.m.r32((fp).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))) as i32); let a3_ = (c.m.r32((fp.wrapping_add(8)).wrapping_add((((l_i_4) as u32)).wrapping_mul(4))) as i32); let a5_ = F::CpuLevel_FromSetting(c, ((2i32) as u32)); F::BattleSetup_SetMember(c, l_i_4, 0i32, a2_, a3_, 0i32, a5_, F32(0x42c80000), ((0i32) as u32)) };
                }
                { let o_ = l_i_4; let n_ = (o_).wrapping_add(1); l_i_4 = n_; };
            }
        }
        F::BattleSetup_Finish(c);
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Demo_SetupBattle", &PROBE_N, &[], true); }
    r_
}

pub fn MenuUtil_ClassifyId(c: &mut Ctx, mut p_id_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "MenuUtil_ClassifyId", &PROBE_N, &[(p_id_1) as u32], false); }
    let r_ = 'f: {
    if ((((((((((((p_id_1).wrapping_sub(20102i32)) as u32)) < (((8i32) as u32))) as i32)) != 0) || ((((((((p_id_1).wrapping_sub(20111i32)) as u32)) < (((889i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        break 'f 0i32;
    }
    if (((((((((p_id_1) == (20101i32)) as i32)) != 0) || ((((((((p_id_1).wrapping_sub(23000i32)) as u32)) < (((1000i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    if (((((p_id_1) == (20110i32)) as i32)) != 0) {
        break 'f 2i32;
    }
    if (((((p_id_1) == (29000i32)) as i32)) != 0) {
        break 'f 3i32;
    }
    if (((((p_id_1) == (29001i32)) as i32)) != 0) {
        break 'f 4i32;
    }
    if (((((((((p_id_1) == (20100i32)) as i32)) != 0) || ((((((((p_id_1).wrapping_sub(50000i32)) as u32)) < (((10000i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        break 'f 5i32;
    }
    break 'f (if (((((p_id_1) == (80430i32)) as i32)) != 0) { 6i32 } else { 7i32 });
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "MenuUtil_ClassifyId", &PROBE_N, &[], true); }
    r_
}

pub fn Text_FindBullet(c: &mut Ctx, mut p_text_1: u32, mut p_line_2: i32, mut p_out_3: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Text_FindBullet", &PROBE_N, &[(p_text_1) as u32, (p_line_2) as u32, (p_out_3) as u32], false); }
    let mut l_p_4: u32 = 0;
    let r_ = 'f: {
    if (((((((((((((p_line_2) < (0i32)) as i32)) != 0) || (((((p_text_1) == (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) || (((((p_out_3) == (((0i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        break 'f;
    }
    l_p_4 = (p_text_1).wrapping_add(c.m.r32((p_text_1).wrapping_add(((((p_line_2).wrapping_add(1i32)) as u32)).wrapping_mul(4))));
    'l1: while (((((((c.m.r16(l_p_4)) as i32)) != (0i32)) as i32)) != 0) {
        if (((((((c.m.r16(l_p_4)) as i32)) == (53029i32)) as i32)) != 0) {
            break 'l1;
        }
        { let o_ = l_p_4; let n_ = (o_).wrapping_add(2); l_p_4 = n_; };
    }
    if (((((((c.m.r16(l_p_4)) as i32)) == (53029i32)) as i32)) != 0) {
        { let v_ = l_p_4; let a_ = p_out_3; c.m.w32(a_, v_); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Text_FindBullet", &PROBE_N, &[], true); }
    r_
}

pub fn Text_PutNumber(c: &mut Ctx, mut p_dst_1: u32, mut p_value_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Text_PutNumber", &PROBE_N, &[(p_dst_1) as u32, (p_value_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let mut l_i_3: i32 = 0;
    let mut l_p_4: u32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 20);
        { let v_ = ((4351i32) as u16); c.m.w16(fp.wrapping_add(0), v_); }
        { let v_ = ((4607i32) as u16); c.m.w16(fp.wrapping_add(2), v_); }
        { let v_ = ((4863i32) as u16); c.m.w16(fp.wrapping_add(4), v_); }
        { let v_ = ((5119i32) as u16); c.m.w16(fp.wrapping_add(6), v_); }
        { let v_ = ((5375i32) as u16); c.m.w16(fp.wrapping_add(8), v_); }
        { let v_ = ((5631i32) as u16); c.m.w16(fp.wrapping_add(10), v_); }
        { let v_ = ((5887i32) as u16); c.m.w16(fp.wrapping_add(12), v_); }
        { let v_ = ((6143i32) as u16); c.m.w16(fp.wrapping_add(14), v_); }
        { let v_ = ((6399i32) as u16); c.m.w16(fp.wrapping_add(16), v_); }
        { let v_ = ((6655i32) as u16); c.m.w16(fp.wrapping_add(18), v_); }
        rt::memset(c, fp.wrapping_add(20), 0, 20);
        { let v_ = 10000i32; c.m.w32(fp.wrapping_add(20).wrapping_add(0), (v_) as u32); }
        { let v_ = 1000i32; c.m.w32(fp.wrapping_add(20).wrapping_add(4), (v_) as u32); }
        { let v_ = 100i32; c.m.w32(fp.wrapping_add(20).wrapping_add(8), (v_) as u32); }
        { let v_ = 10i32; c.m.w32(fp.wrapping_add(20).wrapping_add(12), (v_) as u32); }
        { let v_ = 1i32; c.m.w32(fp.wrapping_add(20).wrapping_add(16), (v_) as u32); }
        if (((((p_dst_1) != (((0i32) as u32))) as i32)) != 0) {
            l_p_4 = p_dst_1;
            if (((((p_value_2) > (((99999i32) as u32))) as i32)) != 0) {
                p_value_2 = ((99999i32) as u32);
            }
            {
                l_i_3 = 0i32;
                'l1: loop {
                    if !(((((l_i_3) < (5i32)) as i32)) != 0) { break 'l1; }
                    'c2: {
                        { let v_ = { let a_ = (fp).wrapping_add((((rt::rem_i32(((rt::div_u32(p_value_2, (((c.m.r32((fp.wrapping_add(20)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4))) as i32)) as u32))) as i32), 10i32)) as u32)).wrapping_mul(2)); c.m.r16(a_) }; let a_ = l_p_4; c.m.w16(a_, v_); };
                        { let r_ = (((c.m.r32((fp.wrapping_add(20)).wrapping_add((((l_i_3) as u32)).wrapping_mul(4))) as i32)) as u32); let n_ = rt::rem_u32(p_value_2, r_); p_value_2 = n_; };
                        { let o_ = l_p_4; let n_ = (o_).wrapping_add(2); l_p_4 = n_; };
                    }
                    { let o_ = l_i_3; let n_ = (o_).wrapping_add(1); l_i_3 = n_; };
                }
            }
        }
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Text_PutNumber", &PROBE_N, &[], true); }
    r_
}

pub fn LipSync_Start(c: &mut Ctx, mut p_data_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "LipSync_Start", &PROBE_N, &[(p_data_1) as u32], false); }
    let mut l_lip_2: u32 = 0;
    let mut l_on_3: i32 = 0;
    let r_ = 'f: {
    l_lip_2 = sym::gLipSync;
    l_on_3 = 1i32;
    { let _ = F::memset(c, l_lip_2, 0i32, 20u32); };
    { let v_ = ((0i32) as u16); let a_ = (l_lip_2).wrapping_add(8); c.m.w16(a_, v_); };
    { let v_ = l_on_3; let a_ = l_lip_2; c.m.w32(a_, (v_) as u32); };
    { let v_ = ((F::Mem_ReadS32(c, (p_data_1).wrapping_add(((8i32) as u32)))) as u16); let a_ = (l_lip_2).wrapping_add(10); c.m.w16(a_, v_); };
    { let r_ = 16i32; let n_ = (p_data_1).wrapping_add(((r_) as u32)); p_data_1 = n_; };
    { let v_ = p_data_1; let a_ = (l_lip_2).wrapping_add(4); c.m.w32(a_, v_); };
    { let v_ = F32(0x00000000); let a_ = (l_lip_2).wrapping_add(12); c.m.w32(a_, (v_).0); };
    { let r_ = (((c.m.r16((l_lip_2).wrapping_add(10))) as i32)).wrapping_mul(4i32); let n_ = (p_data_1).wrapping_add(((r_) as u32)); p_data_1 = n_; };
    { let v_ = rt::i2f(F::Mem_ReadU16(c, (p_data_1).wrapping_sub(((4i32) as u32)))); let a_ = (l_lip_2).wrapping_add(16); c.m.w32(a_, (v_).0); };
    { let v_ = 0i32; let a_ = sym::gLipSyncOpen; c.m.w32(a_, (v_) as u32); };
    { let v_ = l_on_3; let a_ = sym::gLipSyncWaiting; c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "LipSync_Start", &PROBE_N, &[], true); }
    r_
}

pub fn LipSync_Update(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "LipSync_Update", &PROBE_N, &[], false); }
    let mut l_lip_1: u32 = 0;
    let mut l_i_2: u16 = 0;
    let mut l_found_3: i32 = 0;
    let r_ = 'f: {
    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (256i32))) != 0) {
        break 'f;
    }
    { let v_ = 0i32; let a_ = sym::gLipSyncOpen; c.m.w32(a_, (v_) as u32); };
    if ((((((c.m.r32(sym::gLipSyncWaiting) as i32)) != (0i32)) as i32)) != 0) {
        if (((((F::Voice_GetStat(c, 0i32)) != (3i32)) as i32)) != 0) {
            break 'f;
        }
        { let v_ = 0i32; let a_ = sym::gLipSyncWaiting; c.m.w32(a_, (v_) as u32); };
    }
    if ((((((c.m.r32(sym::gLipSync) as i32)) == (0i32)) as i32)) != 0) {
        break 'f;
    }
    l_found_3 = (1i32).wrapping_neg();
    {
        l_i_2 = c.m.r16((sym::gLipSync).wrapping_add(8));
        'l1: loop {
            if !(((((((l_i_2) as i32)) < ((((c.m.r16((sym::gLipSync).wrapping_add(10))) as i32)).wrapping_sub(1i32))) as i32)) != 0) { break 'l1; }
            'c2: {
                if (((rt::fle(rt::i2f({ let a0_ = (c.m.r32((sym::gLipSync).wrapping_add(4))).wrapping_add((((((l_i_2) as i32)).wrapping_mul(4i32)) as u32)); F::Mem_ReadU16(c, a0_) }), F32(c.m.r32((sym::gLipSync).wrapping_add(12)))) as i32)) != 0) {
                    if (((rt::flt(F32(c.m.r32((sym::gLipSync).wrapping_add(12))), rt::i2f({ let a0_ = ((c.m.r32((sym::gLipSync).wrapping_add(4))).wrapping_add((((((l_i_2) as i32)).wrapping_mul(4i32)) as u32))).wrapping_add(((4i32) as u32)); F::Mem_ReadU16(c, a0_) })) as i32)) != 0) {
                        l_found_3 = ((l_i_2) as i32);
                        { let v_ = ((l_found_3) as u16); let a_ = (sym::gLipSync).wrapping_add(8); c.m.w16(a_, v_); };
                        break 'l1;
                    }
                }
            }
            { let o_ = l_i_2; let n_ = (o_).wrapping_add(1); l_i_2 = n_; };
        }
    }
    l_lip_1 = sym::gLipSync;
    if (((((((rt::fgt(F32(c.m.r32((l_lip_1).wrapping_add(12))), F32(c.m.r32((l_lip_1).wrapping_add(16)))) as i32)) != 0) || (((((l_found_3) == ((1i32).wrapping_neg())) as i32)) != 0)) as i32)) != 0) {
        { let _ = F::memset(c, l_lip_1, 0i32, 20u32); };
        break 'f;
    }
    if ((((({ let a0_ = ((c.m.r32((l_lip_1).wrapping_add(4))).wrapping_add((((l_found_3).wrapping_mul(4i32)) as u32))).wrapping_add(((2i32) as u32)); F::Mem_ReadU16(c, a0_) }) != (0i32)) as i32)) != 0) {
        { let v_ = 1i32; let a_ = sym::gLipSyncOpen; c.m.w32(a_, (v_) as u32); };
    }
    { let r_ = F32(0x3f800000); let a_ = (l_lip_1).wrapping_add(12); let n_ = rt::fadd(F32(c.m.r32(a_)), r_); c.m.w32(a_, (n_).0); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "LipSync_Update", &PROBE_N, &[], true); }
    r_
}

pub fn LipSync_IsOpen(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "LipSync_IsOpen", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (c.m.r32(sym::gLipSyncOpen) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "LipSync_IsOpen", &PROBE_N, &[], true); }
    r_
}

pub fn LipSync_Clear(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "LipSync_Clear", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let _ = F::memset(c, sym::gLipSync, 0i32, 20u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "LipSync_Clear", &PROBE_N, &[], true); }
    r_
}
