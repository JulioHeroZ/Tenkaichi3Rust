#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Dc_Main(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Dc_Main", &PROBE_N, &[], false); }
    let mut l_ret_1: i32 = 0;
    let mut l_done_2: i32 = 0;
    let mut l_next_3: i32 = 0;
    let mut l_start_4: i32 = 0;
    let r_ = 'f: {
    l_ret_1 = 1i32;
    { let v_ = (1i32).wrapping_neg(); let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1684); c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1688); c.m.w32(a_, (v_) as u32); };
    l_done_2 = 0i32;
    if (((((c.m.r32(sym::gMenuArc8)) == (((0i32) as u32))) as i32)) != 0) {
        { let v_ = { let a0_ = ({ let a_ = (c.m.r32(sym::gProgress)).wrapping_add(4); (c.m.r32(a_) as i32) }).wrapping_add(11i32); F::File_LoadSync(c, a0_, ((0i32) as u32), 0i32) }; let a_ = sym::gMenuArc8; c.m.w32(a_, v_); };
    }
    'l1: loop {
        'c2: {
            's3: {
                let sv5: i32 = { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); (c.m.r32(a_) as i32) };
                match sv5 {
                    53i32 => {
                        F::Bgm_Play(c, 68376i32);
                        l_next_3 = F::DcMenu_Run(c, 1i32);
                        if (((((l_next_3) != (0i32)) as i32)) != 0) {
                            F::Adx_StopAll(c);
                            { let v_ = l_next_3; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        } else {
                            l_ret_1 = 0i32;
                            F::Adx_StopAll(c);
                            { let v_ = 4i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                            l_done_2 = 1i32;
                        }
                        break 's3;
                    }
                    54i32 => {
                        F::Bgm_Play(c, 68376i32);
                        { let _ = F::DcPass_Run(c, 3i32); };
                        F::Adx_StopAll(c);
                        { let v_ = 53i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        break 's3;
                    }
                    55i32 => {
                        F::Bgm_Play(c, 68376i32);
                        { let _ = F::DcList_Run(c, 2i32); };
                        F::Adx_StopAll(c);
                        { let v_ = 53i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                        break 's3;
                    }
                    56i32 => {
                        F::Bgm_Play(c, 68376i32);
                        l_next_3 = F::ReplayMenu_Run(c, 4i32);
                        if (((((l_next_3) != (0i32)) as i32)) != 0) {
                            l_start_4 = 1i32;
                            F::Adx_StopAll(c);
                            if (((((l_next_3) == (l_start_4)) as i32)) != 0) {
                                l_ret_1 = 1i32;
                            } else {
                                l_ret_1 = 0i32;
                                { let v_ = l_next_3; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                            }
                            l_done_2 = 1i32;
                        } else {
                            F::Adx_StopAll(c);
                            { let v_ = 53i32; let a_ = (c.m.r32(sym::gProgress)).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
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
    if (((((c.m.r32(sym::gMenuArc8)) != (((0i32) as u32))) as i32)) != 0) {
        { let a0_ = c.m.r32(sym::gMenuArc8); F::Heap_Free(c, a0_) };
        { let v_ = ((0i32) as u32); let a_ = sym::gMenuArc8; c.m.w32(a_, v_); };
    }
    { let a_ = (c.m.r32(sym::gProgress)).wrapping_add(1680); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
    break 'f l_ret_1;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Dc_Main", &PROBE_N, &[], true); }
    r_
}
