#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn SimEv36(c: &mut Ctx, mut p_day_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "SimEv36", &PROBE_N, &[(p_day_1) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv2: i32 = (c.m.r32((p_day_1).wrapping_add(2920)) as i32);
        match sv2 {
            0i32 => {
                { let v_ = 11i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            1i32 => {
                { let v_ = 98i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let _ = F::Snd_PlaySe(c, ((2i32) as u32), 23i32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            2i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 25i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            3i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let v_ = 3i32; let a_ = (p_day_1).wrapping_add(2940); c.m.w32(a_, (v_) as u32); };
                { let v_ = 7i32; let a_ = (p_day_1).wrapping_add(2936); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 5i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            4i32 => {
                F::SimDay_Cmd(c, 6i32);
                { let v_ = 175i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            5i32 => {
                { let v_ = 211i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            6i32 => {
                { let v_ = 212i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            7i32 => {
                { let v_ = 213i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_AddChange(c, 3i32, 5000i32);
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            8i32 => {
                { let v_ = 214i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            9i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                F::SimDay_Cmd(c, 7i32);
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                break 's1;
            }
            10i32 => {
                { let v_ = 99i32; let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                { let a1_ = (((((F::Rand_Range(c, ((((3i32) as i64 as u64)) as u32))) as u64)) as i32)).wrapping_neg(); F::SimDay_AddChange(c, 0i32, a1_) };
                { let a1_ = (((((F::Rand_Range(c, ((((3i32) as i64 as u64)) as u32))) as u64)) as i32)).wrapping_neg(); F::SimDay_AddChange(c, 1i32, a1_) };
                F::SimDay_Cmd(c, 26i32);
                { let r_ = 128i32; let a_ = (p_day_1).wrapping_add(1360); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                { let v_ = 0i32; let a_ = (p_day_1).wrapping_add(2924); c.m.w32(a_, (v_) as u32); };
                { let a_ = (p_day_1).wrapping_add(2920); let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 's1;
            }
            11i32 => {
                { let v_ = (1i32).wrapping_neg(); let a_ = (p_day_1).wrapping_add(1440); c.m.w32(a_, (v_) as u32); };
                break 'f 1i32;
            }
            _ => {
            }
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "SimEv36", &PROBE_N, &[], true); }
    r_
}
