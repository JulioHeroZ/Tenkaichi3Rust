#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Timer_SetMode(c: &mut Ctx, mut p_ch_1: i32, mut p_value_2: u32, mut p_op_3: u8) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Timer_SetMode", &PROBE_N, &[(p_ch_1) as u32, (p_value_2) as u32, (p_op_3) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv4: i32 = p_ch_1;
        match sv4 {
            0i32 => {
                's2: {
                    let sv5: i32 = ((p_op_3) as i32);
                    match sv5 {
                        61i32 => {
                            { let v_ = p_value_2; let a_ = (((268435472i32).wrapping_add((0i32).wrapping_mul(2048i32))) as u32); c.m.w32(a_, v_); };
                            break 's2;
                        }
                        38i32 => {
                            { let r_ = p_value_2; let a_ = (((268435472i32).wrapping_add((0i32).wrapping_mul(2048i32))) as u32); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
                            break 's2;
                        }
                        124i32 => {
                            { let r_ = p_value_2; let a_ = (((268435472i32).wrapping_add((0i32).wrapping_mul(2048i32))) as u32); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                            break 's2;
                        }
                        _ => {
                        }
                    }
                }
                break 's1;
            }
            1i32 => {
                's3: {
                    let sv6: i32 = ((p_op_3) as i32);
                    match sv6 {
                        61i32 => {
                            { let v_ = p_value_2; let a_ = (((268435472i32).wrapping_add((1i32).wrapping_mul(2048i32))) as u32); c.m.w32(a_, v_); };
                            break 's3;
                        }
                        38i32 => {
                            { let r_ = p_value_2; let a_ = (((268435472i32).wrapping_add((1i32).wrapping_mul(2048i32))) as u32); let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
                            break 's3;
                        }
                        124i32 => {
                            { let r_ = p_value_2; let a_ = (((268435472i32).wrapping_add((1i32).wrapping_mul(2048i32))) as u32); let n_ = ((c.m.r32(a_)) | (r_)); c.m.w32(a_, n_); };
                            break 's3;
                        }
                        _ => {
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
    if rt::probing() { crate::host::harness::probe(c, "Timer_SetMode", &PROBE_N, &[], true); }
    r_
}

pub fn Timer_GetMode(c: &mut Ctx, mut p_ch_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Timer_GetMode", &PROBE_N, &[(p_ch_1) as u32], false); }
    let mut l_mode_2: u32 = 0;
    let r_ = 'f: {
    l_mode_2 = ((0i32) as u32);
    's1: {
        let sv3: i32 = p_ch_1;
        match sv3 {
            0i32 => {
                l_mode_2 = c.m.r32((((268435472i32).wrapping_add((0i32).wrapping_mul(2048i32))) as u32));
                break 's1;
            }
            1i32 => {
                l_mode_2 = c.m.r32((((268435472i32).wrapping_add((1i32).wrapping_mul(2048i32))) as u32));
                break 's1;
            }
            _ => {
            }
        }
    }
    break 'f l_mode_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Timer_GetMode", &PROBE_N, &[], true); }
    r_
}

pub fn Timer_SetCompare(c: &mut Ctx, mut p_ch_1: i32, mut p_value_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Timer_SetCompare", &PROBE_N, &[(p_ch_1) as u32, (p_value_2) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv3: i32 = p_ch_1;
        match sv3 {
            0i32 => {
                { let v_ = p_value_2; let a_ = (((268435488i32).wrapping_add((0i32).wrapping_mul(2048i32))) as u32); c.m.w32(a_, v_); };
                break 's1;
            }
            1i32 => {
                { let v_ = p_value_2; let a_ = (((268435488i32).wrapping_add((1i32).wrapping_mul(2048i32))) as u32); c.m.w32(a_, v_); };
                break 's1;
            }
            _ => {
            }
        }
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Timer_SetCompare", &PROBE_N, &[], true); }
    r_
}

pub fn Timer_SetCount(c: &mut Ctx, mut p_ch_1: i32, mut p_value_2: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Timer_SetCount", &PROBE_N, &[(p_ch_1) as u32, (p_value_2) as u32], false); }
    let r_ = 'f: {
    's1: {
        let sv3: i32 = p_ch_1;
        match sv3 {
            0i32 => {
                { let v_ = p_value_2; let a_ = (((268435456i32).wrapping_add((0i32).wrapping_mul(2048i32))) as u32); c.m.w32(a_, v_); };
                break 's1;
            }
            1i32 => {
                { let v_ = p_value_2; let a_ = (((268435456i32).wrapping_add((1i32).wrapping_mul(2048i32))) as u32); c.m.w32(a_, v_); };
                break 's1;
            }
            _ => {
            }
        }
    }
    { let v_ = ((0i32) as u32); let a_ = (sym::gTimerLines).wrapping_add((((p_ch_1) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Timer_SetCount", &PROBE_N, &[], true); }
    r_
}

pub fn Timer_ReadMicros(c: &mut Ctx, mut p_ch_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Timer_ReadMicros", &PROBE_N, &[(p_ch_1) as u32], false); }
    let mut l_lines_2: u64 = 0;
    let r_ = 'f: {
    l_lines_2 = ((0i32) as u64);
    's1: {
        let sv3: i32 = p_ch_1;
        match sv3 {
            0i32 => {
                l_lines_2 = ((c.m.r32((((268435456i32).wrapping_add((0i32).wrapping_mul(2048i32))) as u32))) as u64);
                break 's1;
            }
            1i32 => {
                l_lines_2 = ((c.m.r32((((268435456i32).wrapping_add((1i32).wrapping_mul(2048i32))) as u32))) as u64);
                break 's1;
            }
            _ => {
            }
        }
    }
    if ((((F::Timer_GetMode(c, p_ch_1)) & (((1024i32) as u32)))) != 0) {
        { let r_ = ((65535i32) as u32); let a_ = (sym::gTimerLines).wrapping_add((((p_ch_1) as u32)).wrapping_mul(4)); let n_ = (c.m.r32(a_)).wrapping_add(r_); c.m.w32(a_, n_); };
        F::Timer_SetMode(c, p_ch_1, ((1024i32) as u32), ((124i32) as u8));
    }
    { let r_ = ((c.m.r32((sym::gTimerLines).wrapping_add((((p_ch_1) as u32)).wrapping_mul(4)))) as u64); let n_ = (l_lines_2).wrapping_add(r_); l_lines_2 = n_; };
    break 'f ((rt::div_u64((l_lines_2).wrapping_mul(((1000000i32) as u64)), ((15734i32) as u64))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Timer_ReadMicros", &PROBE_N, &[], true); }
    r_
}

pub fn Timer_Stub(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Timer_Stub", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Timer_Stub", &PROBE_N, &[], true); }
    r_
}

pub fn Timer_Init(c: &mut Ctx, mut p_ch_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Timer_Init", &PROBE_N, &[(p_ch_1) as u32], false); }
    let r_ = 'f: {
    F::Timer_SetMode(c, p_ch_1, ((((((((3i32) | (64i32))) | (256i32))) | (512i32))) as u32), ((61i32) as u8));
    F::Timer_SetCompare(c, p_ch_1, ((65535i32) as u32));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Timer_Init", &PROBE_N, &[], true); }
    r_
}

pub fn Timer_Start(c: &mut Ctx, mut p_ch_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Timer_Start", &PROBE_N, &[(p_ch_1) as u32], false); }
    let r_ = 'f: {
    F::Timer_SetMode(c, p_ch_1, ((128i32) as u32), ((124i32) as u8));
    F::Timer_Reset(c, p_ch_1);
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Timer_Start", &PROBE_N, &[], true); }
    r_
}

pub fn Timer_Stop(c: &mut Ctx, mut p_ch_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Timer_Stop", &PROBE_N, &[(p_ch_1) as u32], false); }
    let r_ = 'f: {
    F::Timer_SetMode(c, p_ch_1, (((!(128i32))) as u32), ((38i32) as u8));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Timer_Stop", &PROBE_N, &[], true); }
    r_
}

pub fn Timer_Reset(c: &mut Ctx, mut p_ch_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Timer_Reset", &PROBE_N, &[(p_ch_1) as u32], false); }
    let r_ = 'f: {
    F::Timer_SetCount(c, p_ch_1, ((0i32) as u32));
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Timer_Reset", &PROBE_N, &[], true); }
    r_
}

pub fn Timer_GetMicros(c: &mut Ctx, mut p_ch_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Timer_GetMicros", &PROBE_N, &[(p_ch_1) as u32], false); }
    let r_ = 'f: {
    break 'f F::Timer_ReadMicros(c, p_ch_1);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Timer_GetMicros", &PROBE_N, &[], true); }
    r_
}

pub fn Timer_GetFrames(c: &mut Ctx, mut p_ch_1: i32) -> F32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Timer_GetFrames", &PROBE_N, &[(p_ch_1) as u32], false); }
    let r_ = 'f: {
    break 'f rt::fdiv(rt::u2f((((F::Timer_GetMicros(c, p_ch_1)).wrapping_mul(60i32)) as u32)), F32(0x49742400));
       F32(0)
    };
    if rt::probing() { crate::host::harness::probe(c, "Timer_GetFrames", &PROBE_N, &[], true); }
    r_
}
