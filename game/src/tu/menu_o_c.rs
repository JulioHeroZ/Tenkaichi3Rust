#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Ub_SetupTeam(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Ub_SetupTeam", &PROBE_N, &[], false); }
    let mut l_i_1: i32 = 0;
    let mut l_count_2: i32 = 0;
    let r_ = 'f: {
    l_count_2 = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1604); (c.m.r32(a_) as i32) };
    F::BattleSetup_SetSide(c, 0i32, 0i32, 0i32, l_count_2, 1i32, 1i32, 0i32, ((0i32) as u32));
    {
        l_i_1 = 0i32;
        'l1: loop {
            if !(((((l_i_1) < (l_count_2)) as i32)) != 0) { break 'l1; }
            'c2: {
                { let a2_ = { let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1088)).wrapping_add((((l_i_1) as u32)).wrapping_mul(48))).wrapping_add(28); (c.m.r32(a_) as i32) }; let a3_ = { let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1088)).wrapping_add((((l_i_1) as u32)).wrapping_mul(48))).wrapping_add(24); (c.m.r32(a_) as i32) }; let a7_ = (((c.m.r32(sym::gProgress)).wrapping_add(1088)).wrapping_add((((l_i_1) as u32)).wrapping_mul(48))).wrapping_add(32); F::BattleSetup_SetMember(c, 0i32, l_i_1, a2_, a3_, 0i32, 0i32, F32(0x42c80000), a7_) };
            }
            { let o_ = l_i_1; let n_ = (o_).wrapping_add(1); l_i_1 = n_; };
        }
    }
    F::BattleSetup_Finish(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Ub_SetupTeam", &PROBE_N, &[], true); }
    r_
}

pub fn Ub_SetupSolo(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Ub_SetupSolo", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a2_ = { let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1088)).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(28); (c.m.r32(a_) as i32) }; let a3_ = { let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1088)).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(24); (c.m.r32(a_) as i32) }; let a7_ = (((c.m.r32(sym::gProgress)).wrapping_add(1088)).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(32); F::BattleSetup_SetMember(c, 0i32, 0i32, a2_, a3_, 0i32, 0i32, F32(0x42c80000), a7_) };
    F::BattleSetup_Finish(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Ub_SetupSolo", &PROBE_N, &[], true); }
    r_
}

pub fn Ub_SetupSolo2(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Ub_SetupSolo2", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let a2_ = { let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1088)).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(28); (c.m.r32(a_) as i32) }; let a3_ = { let a_ = (((c.m.r32(sym::gProgress)).wrapping_add(1088)).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(24); (c.m.r32(a_) as i32) }; let a7_ = (((c.m.r32(sym::gProgress)).wrapping_add(1088)).wrapping_add((((0i32) as u32)).wrapping_mul(48))).wrapping_add(32); F::BattleSetup_SetMember(c, 0i32, 0i32, a2_, a3_, 0i32, 0i32, F32(0x42c80000), a7_) };
    F::BattleSetup_Finish(c);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Ub_SetupSolo2", &PROBE_N, &[], true); }
    r_
}

pub fn Ub_Main(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Ub_Main", &PROBE_N, &[], false); }
    let mut l_done_1: i32 = 0;
    let mut l_result_2: i32 = 0;
    let mut l_r_3: i32 = 0;
    let r_ = 'f: {
    l_done_1 = 0i32;
    l_result_2 = 1i32;
    'l1: loop {
        'c2: {
            if (((((c.m.r32(sym::gMenuArc3)) == (((0i32) as u32))) as i32)) != 0) {
                { let v_ = { let a0_ = ({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_add(3i32); F::File_LoadSync(c, a0_, ((0i32) as u32), 0i32) }; let a_ = sym::gMenuArc3; c.m.w32(a_, v_); };
            }
            's3: {
                let sv4: i32 = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); (c.m.r32(a_) as i32) };
                match sv4 {
                    13i32 => {
                        F::Bgm_Play(c, 68379i32);
                        l_r_3 = F::UbMenu_Run(c, 1i32);
                        if ((l_r_3) != 0) {
                            's4: {
                                let sv5: i32 = l_r_3;
                                match sv5 {
                                    1i32 => {
                                        { let v_ = 20i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                                        break 's4;
                                    }
                                    2i32 => {
                                        { let v_ = 14i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                                        break 's4;
                                    }
                                    3i32 => {
                                        { let v_ = 17i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                                        break 's4;
                                    }
                                    4i32 => {
                                        { let v_ = 24i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                                        break 's4;
                                    }
                                    _ => {
                                    }
                                }
                            }
                        } else {
                            { let v_ = 4i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                            l_result_2 = 0i32;
                            l_done_1 = 1i32;
                        }
                        F::Adx_StopAll(c);
                        break 's3;
                    }
                    14i32 => {
                        F::Bgm_Play(c, 68379i32);
                        if ((F::MisSel_Run(c, 2i32)) != 0) {
                            { let v_ = 15i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        } else {
                            { let v_ = 13i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        }
                        F::Adx_StopAll(c);
                        break 's3;
                    }
                    15i32 => {
                        F::Bgm_Play(c, 68379i32);
                        if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1604); (c.m.r32(a_) as i32) }) >= (2i32)) as i32)) != 0) {
                            l_r_3 = F::UbTeamSel_Run(c, 4i32);
                        } else {
                            l_r_3 = F::SoloSel_Run(c, 4i32);
                        }
                        if ((l_r_3) != 0) {
                            F::Ub_SetupTeam(c);
                            l_done_1 = 1i32;
                            { let v_ = 16i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        } else {
                            { let v_ = 14i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        }
                        F::Adx_StopAll(c);
                        break 's3;
                    }
                    16i32 => {
                        F::Bgm_Play(c, 68379i32);
                        { let _ = F::MisResult_Run(c, 3i32); };
                        { let v_ = 14i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        F::Adx_StopAll(c);
                        break 's3;
                    }
                    17i32 => {
                        F::Bgm_Play(c, 68379i32);
                        if ((F::SurvSel_Run(c, 2i32)) != 0) {
                            { let v_ = 18i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        } else {
                            { let v_ = 13i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        }
                        F::Adx_StopAll(c);
                        break 's3;
                    }
                    18i32 => {
                        F::Bgm_Play(c, 68379i32);
                        if ((F::SoloSel_Run(c, 4i32)) != 0) {
                            F::Ub_SetupSolo(c);
                            l_done_1 = 1i32;
                            { let v_ = 19i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        } else {
                            { let v_ = 17i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        }
                        F::Adx_StopAll(c);
                        break 's3;
                    }
                    19i32 => {
                        F::Bgm_Play(c, 68379i32);
                        { let _ = F::SurvResult_Run(c, 3i32); };
                        { let v_ = 17i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        F::Adx_StopAll(c);
                        break 's3;
                    }
                    20i32 => {
                        F::Bgm_Play(c, 68379i32);
                        if ((F::SimTop_Run(c, 5i32)) != 0) {
                            { let v_ = 21i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        } else {
                            { let v_ = 13i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        }
                        F::Adx_StopAll(c);
                        break 's3;
                    }
                    21i32 => {
                        F::Bgm_Play(c, 68379i32);
                        if ((F::SoloSel_Run(c, 4i32)) != 0) {
                            { let v_ = 22i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        } else {
                            { let v_ = 20i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        }
                        F::Adx_StopAll(c);
                        break 's3;
                    }
                    22i32 => {
                        if ((F::SimDay_Run(c, 0i32)) != 0) {
                            { let v_ = 23i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                            l_done_1 = 1i32;
                        } else {
                            { let v_ = 13i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        }
                        F::Adx_StopAll(c);
                        break 's3;
                    }
                    23i32 => {
                        if ((F::SimResult_Run(c, 3i32)) != 0) {
                            { let v_ = 20i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        } else {
                            { let v_ = 22i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                            { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1652); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                        }
                        F::Adx_StopAll(c);
                        break 's3;
                    }
                    24i32 => {
                        F::Bgm_Play(c, 68379i32);
                        if ((F::DiscFusion_Run(c, 6i32)) != 0) {
                            if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1600); (c.m.r32(a_) as i32) }) == (0i32)) as i32)) != 0) {
                                { let v_ = 1i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1604); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 0i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1608); c.m.w32(a_, (v_) as u32); };
                                { let v_ = 25i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                            } else {
                                { let v_ = 28i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                            }
                        } else {
                            { let v_ = 13i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        }
                        F::Adx_StopAll(c);
                        break 's3;
                    }
                    25i32 => {
                        F::Bgm_Play(c, 68379i32);
                        if ((F::SoloSel_Run(c, 4i32)) != 0) {
                            { let v_ = 26i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        } else {
                            { let v_ = 24i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        }
                        F::Adx_StopAll(c);
                        break 's3;
                    }
                    26i32 => {
                        F::Bgm_Play(c, 68379i32);
                        if (({ let _ = 0i32; F::UbRank_Run(c) }) != 0) {
                            { let v_ = 27i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                            l_done_1 = 1i32;
                        } else {
                            { let v_ = 24i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        }
                        F::Adx_StopAll(c);
                        break 's3;
                    }
                    27i32 => {
                        F::Bgm_Play(c, 68379i32);
                        { let _ = F::UbResult_Run(c, 3i32); };
                        { let v_ = 26i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        F::Adx_StopAll(c);
                        break 's3;
                    }
                    28i32 => {
                        F::Bgm_Play(c, 68379i32);
                        if (({ let _ = 0i32; F::UbzSel_Run(c) }) != 0) {
                            { let v_ = 29i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        } else {
                            { let v_ = 24i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        }
                        F::Adx_StopAll(c);
                        break 's3;
                    }
                    29i32 => {
                        F::Bgm_Play(c, 68379i32);
                        if ((F::SoloSel_Run(c, 4i32)) != 0) {
                            F::Ub_SetupSolo2(c);
                            l_done_1 = 1i32;
                            { let v_ = 30i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        } else {
                            { let v_ = 28i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        }
                        F::Adx_StopAll(c);
                        break 's3;
                    }
                    30i32 => {
                        F::Bgm_Play(c, 68379i32);
                        { let _ = F::UbResult_Run(c, 3i32); };
                        { let v_ = 28i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        F::Adx_StopAll(c);
                        break 's3;
                    }
                    _ => {
                    }
                }
            }
            { let _ = F::sceGsSyncPath(c, 0i32, ((0i32) as u16)); };
        }
        if !((((!((l_done_1) != 0)) as i32)) != 0) { break 'l1; }
    }
    if (((((c.m.r32(sym::gMenuArc3)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gMenuArc3); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gMenuArc3; c.m.w32(a_, v_); };
    }
    break 'f l_result_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Ub_Main", &PROBE_N, &[], true); }
    r_
}
