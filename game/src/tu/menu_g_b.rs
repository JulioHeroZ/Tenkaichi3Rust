#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Duel_Main(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Duel_Main", &PROBE_N, &[], false); }
    let mut l_bgm_1: i32 = 0;
    let mut l_done_2: i32 = 0;
    let mut l_result_3: i32 = 0;
    let r_ = 'f: {
    l_bgm_1 = 0i32;
    l_done_2 = 0i32;
    l_result_3 = 1i32;
    if (((((c.m.r32(sym::gMenuArc5)) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a0_ = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(4); (c.m.r32(a_) as i32) }; F::File_LoadSync(c, a0_, ((0i32) as u32), 0i32) }; let a_ = sym::gMenuArc5; c.m.w32(a_, v_); };
    }
    'l1: loop {
        'c2: {
            's3: {
                let sv4: i32 = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); (c.m.r32(a_) as i32) };
                match sv4 {
                    38i32 => {
                        if ((((!((l_bgm_1) != 0)) as i32)) != 0) {
                            l_bgm_1 = 1i32;
                            F::Bgm_Play(c, 68379i32);
                        }
                        if ((F::DuelMenu_Run(c, 1i32)) != 0) {
                            F::Adx_StopAll(c);
                            { let v_ = (if (({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1572); (c.m.r32(a_) as i32) }) != 0) { 40i32 } else { 39i32 }); let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        } else {
                            F::Adx_StopAll(c);
                            l_bgm_1 = 0i32;
                            { let v_ = 4i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                            l_result_3 = 0i32;
                            l_done_2 = 1i32;
                        }
                        break 's3;
                    }
                    39i32 => {
                        if ((F::CharSel_Run(c, 2i32)) != 0) {
                            F::Adx_StopAll(c);
                            l_bgm_1 = 0i32;
                            l_done_2 = 1i32;
                        } else {
                            F::Adx_StopAll(c);
                            l_bgm_1 = 0i32;
                            { let v_ = 38i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        }
                        break 's3;
                    }
                    40i32 => {
                        if ((F::TeamSel_Run(c, 2i32)) != 0) {
                            F::Adx_StopAll(c);
                            l_bgm_1 = 0i32;
                            l_done_2 = 1i32;
                        } else {
                            F::Adx_StopAll(c);
                            l_bgm_1 = 0i32;
                            { let v_ = 38i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        }
                        break 's3;
                    }
                    41i32 => {
                        if ((F::Duel_Mode41(c, 0i32)) != 0) {
                            F::Adx_StopAll(c);
                            l_bgm_1 = 0i32;
                            l_done_2 = 1i32;
                            { let v_ = 39i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        } else {
                            F::Adx_StopAll(c);
                            l_bgm_1 = 0i32;
                            { let v_ = 39i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        }
                        break 's3;
                    }
                    _ => {
                    }
                }
            }
            { let _ = F::sceGsSyncPath(c, 0i32, ((0i32) as u16)); };
        }
        if !((((!((l_done_2) != 0)) as i32)) != 0) { break 'l1; }
    }
    if (((((c.m.r32(sym::gMenuArc5)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gMenuArc5); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gMenuArc5; c.m.w32(a_, v_); };
    }
    break 'f l_result_3;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Duel_Main", &PROBE_N, &[], true); }
    r_
}

pub fn Duel_Mode41(c: &mut Ctx, mut p_arg_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Duel_Mode41", &PROBE_N, &[(p_arg_1) as u32], false); }
    let r_ = 'f: {
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Duel_Mode41", &PROBE_N, &[], true); }
    r_
}
