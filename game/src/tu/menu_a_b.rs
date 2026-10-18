#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Progress_Main(c: &mut Ctx, mut p_arg_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Progress_Main", &PROBE_N, &[(p_arg_1) as u32], false); }
    let mut l_done_2: i32 = 0;
    let mut l_flags_3: i32 = 0;
    let mut l_reason_4: i32 = 0;
    let mut l_ret_5: i32 = 0;
    let r_ = 'f: {
    l_done_2 = 0i32;
    F::Pad_SetRepeat(c, 40i32, 3i32);
    if (((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); (c.m.r32(a_) as i32) }) & (64i32))) != 0) {
        { let _ = F::FirstRun_Main(c); };
        { let r_ = (!(64i32)); let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
    }
    l_flags_3 = F::BattleResult_GetFlags(c);
    l_reason_4 = F::BattleResult_GetReason(c);
    if ((((l_flags_3) & (8i32))) != 0) {
        if ((((l_reason_4) & (8i32))) != 0) {
            { let r_ = (!(16i32)); let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            { let r_ = (!(32i32)); let a_ = (c.m.r32(sym::gProgress)).wrapping_add(20); let n_ = (((c.m.r32(a_) as i32)) & (r_)); c.m.w32(a_, (n_) as u32); };
            { let v_ = 4i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
        } else {
            if ((((l_reason_4) & (16i32))) != 0) {
                if ((((((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); (c.m.r32(a_) as i32) }) >= (38i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); (c.m.r32(a_) as i32) }) <= (41i32)) as i32)) != 0)) as i32)) != 0) {
                    if ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1572); (c.m.r32(a_) as i32) }) != (0i32)) as i32)) != 0) {
                        { let v_ = 40i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                    } else {
                        { let v_ = 39i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                    }
                } else {
                    if ((((((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); (c.m.r32(a_) as i32) }) >= (44i32)) as i32)) != 0) && ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); (c.m.r32(a_) as i32) }) <= (45i32)) as i32)) != 0)) as i32)) != 0) {
                        { let v_ = 45i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                    }
                }
            } else {
                if ((((l_reason_4) & (128i32))) != 0) {
                    { let v_ = 38i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                } else {
                    if ((((l_reason_4) & (2048i32))) != 0) {
                        { let v_ = 6i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                    } else {
                        if ((((l_reason_4) & (4096i32))) != 0) {
                            { let v_ = 56i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                            { let r_ = 1i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1676); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                        }
                    }
                }
            }
        }
    }
    if ((((((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); (c.m.r32(a_) as i32) }) == (1i32)) as i32)) != 0) || ((((({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); (c.m.r32(a_) as i32) }) == (4i32)) as i32)) != 0)) as i32)) != 0) {
        if (((((c.m.r32(sym::gMenuArc1)) == (((0i32) as u32))) as i32)) != 0) {
            { let v_ = { let a0_ = ({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_add(1i32); F::File_LoadSync(c, a0_, ((0i32) as u32), 0i32) }; let a_ = sym::gMenuArc1; c.m.w32(a_, v_); };
        }
    }
    F::Snd_LoadBankFile(c, ((2i32) as u32), 332i32);
    'l1: loop {
        'c2: {
            's3: {
                let sv6: i32 = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); (c.m.r32(a_) as i32) };
                match sv6 {
                    2i32 => {
                        F::PadWatch_SetEnabled(c, 0i32);
                        F::Movie_PlayOpening(c);
                        F::PadWatch_SetEnabled(c, 1i32);
                        { let v_ = 1i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        break 's3;
                    }
                    1i32 => {
                        F::Bgm_Play(c, 68374i32);
                        l_ret_5 = F::Title_Run(c, 1i32);
                        F::Adx_StopAll(c);
                        's4: {
                            let sv7: i32 = l_ret_5;
                            match sv7 {
                                1i32 => {
                                    l_done_2 = 1i32;
                                    break 's4;
                                }
                                2i32 => {
                                    { let v_ = l_ret_5; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                                    if (((((c.m.r32(sym::gMenuArc1)) != (((0i32) as u32))) as i32)) != 0) {
                                        { let a0_ = c.m.r32(sym::gMenuArc1); F::Heap_Free(c, a0_) };
                                        { let v_ = ((0i32) as u32); let a_ = sym::gMenuArc1; c.m.w32(a_, v_); };
                                    }
                                    break 's4;
                                }
                                _ => {
                                    { let v_ = 4i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                                    break 's4;
                                }
                            }
                        }
                        break 's3;
                    }
                    4i32 => {
                        F::Bgm_Play(c, 68374i32);
                        if ((F::MainMenu_Run(c, 2i32)) != 0) {
                            F::Adx_StopAll(c);
                            if (((((c.m.r32(sym::gMenuArc1)) != (((0i32) as u32))) as i32)) != 0) {
                                { let a0_ = c.m.r32(sym::gMenuArc1); F::Heap_Free(c, a0_) };
                                { let v_ = ((0i32) as u32); let a_ = sym::gMenuArc1; c.m.w32(a_, v_); };
                            }
                        } else {
                            F::Adx_StopAll(c);
                            { let v_ = 1i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        }
                        break 's3;
                    }
                    6i32 | 7i32 | 8i32 | 9i32 | 10i32 => {
                        if ((F::Hist_Main(c)) != 0) {
                            l_done_2 = 1i32;
                        }
                        break 's3;
                    }
                    13i32 | 14i32 | 15i32 | 16i32 | 17i32 | 18i32 | 19i32 | 20i32 | 21i32 | 22i32 | 23i32 | 24i32 | 25i32 | 26i32 | 27i32 | 28i32 | 29i32 | 30i32 => {
                        if ((F::Ub_Main(c)) != 0) {
                            l_done_2 = 1i32;
                        }
                        break 's3;
                    }
                    33i32 | 34i32 | 35i32 => {
                        if ((F::Tour_Main(c)) != 0) {
                            l_done_2 = 1i32;
                        }
                        break 's3;
                    }
                    38i32 | 39i32 | 40i32 | 41i32 => {
                        if ((F::Duel_Main(c)) != 0) {
                            l_done_2 = 1i32;
                        }
                        break 's3;
                    }
                    44i32 | 45i32 => {
                        if ((F::TrainMode_Main(c)) != 0) {
                            l_done_2 = 1i32;
                        }
                        break 's3;
                    }
                    48i32 | 49i32 | 50i32 => {
                        { let _ = F::EvoMode_Main(c); };
                        break 's3;
                    }
                    53i32 | 54i32 | 55i32 | 56i32 => {
                        if ((F::Dc_Main(c)) != 0) {
                            l_done_2 = 1i32;
                        }
                        break 's3;
                    }
                    60i32 => {
                        { let _ = F::CharRefMode_Main(c); };
                        break 's3;
                    }
                    62i32 => {
                        { let _ = F::OptMode_Main(c); };
                        break 's3;
                    }
                    70i32 => {
                        { let _ = F::Shen_Main(c); };
                        break 's3;
                    }
                    _ => {
                    }
                }
            }
        }
        if !((((!((l_done_2) != 0)) as i32)) != 0) { break 'l1; }
    }
    F::Pad_SetRepeat(c, 20i32, 1i32);
    F::Snd_UnloadBank(c, ((2i32) as u32));
    if (((((c.m.r32(sym::gMenuArc0)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gMenuArc0); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gMenuArc0; c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gMenuArc1)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gMenuArc1); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gMenuArc1; c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gMenuArc2)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gMenuArc2); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gMenuArc2; c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gMenuArc3)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gMenuArc3); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gMenuArc3; c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gMenuArc4)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gMenuArc4); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gMenuArc4; c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gMenuArc5)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gMenuArc5); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gMenuArc5; c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gMenuArc6)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gMenuArc6); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gMenuArc6; c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gMenuArc7)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gMenuArc7); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gMenuArc7; c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gMenuArc8)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gMenuArc8); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gMenuArc8; c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gMenuArc9)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gMenuArc9); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gMenuArc9; c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gMenuArc10)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gMenuArc10); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gMenuArc10; c.m.w32(a_, v_); };
    }
    if (((((c.m.r32(sym::gMenuArc11)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gMenuArc11); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gMenuArc11; c.m.w32(a_, v_); };
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Progress_Main", &PROBE_N, &[], true); }
    r_
}
