#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn GetThreadId(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "GetThreadId", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "GetThreadId", &PROBE_N, &[], true); }
    r_
}

pub fn ChangeThreadPriority(c: &mut Ctx, mut p_tid_1: i32, mut p_prio_2: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "ChangeThreadPriority", &PROBE_N, &[(p_tid_1) as u32, (p_prio_2) as u32], false); }
    let r_ = 'f: {
    { let _ = p_tid_1; };
    { let _ = p_prio_2; };
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "ChangeThreadPriority", &PROBE_N, &[], true); }
    r_
}

pub fn FlushCache(c: &mut Ctx, mut p_mode_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "FlushCache", &PROBE_N, &[(p_mode_1) as u32], false); }
    let r_ = 'f: {
    { let _ = p_mode_1; };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "FlushCache", &PROBE_N, &[], true); }
    r_
}

pub fn sceSifInitRpc(c: &mut Ctx, mut p_mode_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "sceSifInitRpc", &PROBE_N, &[(p_mode_1) as u32], false); }
    let r_ = 'f: {
    { let _ = p_mode_1; };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "sceSifInitRpc", &PROBE_N, &[], true); }
    r_
}

pub fn sceSifRebootIop(c: &mut Ctx, mut p_img_1: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "sceSifRebootIop", &PROBE_N, &[(p_img_1) as u32], false); }
    let r_ = 'f: {
    { let _ = p_img_1; };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "sceSifRebootIop", &PROBE_N, &[], true); }
    r_
}

pub fn sceSifSyncIop(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "sceSifSyncIop", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "sceSifSyncIop", &PROBE_N, &[], true); }
    r_
}

pub fn sceSifLoadModule(c: &mut Ctx, mut p_name_1: u32, mut p_argLen_2: i32, mut p_args_3: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "sceSifLoadModule", &PROBE_N, &[(p_name_1) as u32, (p_argLen_2) as u32, (p_args_3) as u32], false); }
    let r_ = 'f: {
    { let _ = p_name_1; };
    { let _ = p_argLen_2; };
    { let _ = p_args_3; };
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "sceSifLoadModule", &PROBE_N, &[], true); }
    r_
}

pub fn sceFsReset(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "sceFsReset", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "sceFsReset", &PROBE_N, &[], true); }
    r_
}

pub fn sceCdInit(c: &mut Ctx, mut p_mode_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "sceCdInit", &PROBE_N, &[(p_mode_1) as u32], false); }
    let r_ = 'f: {
    { let _ = p_mode_1; };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "sceCdInit", &PROBE_N, &[], true); }
    r_
}

pub fn sceCdMmode(c: &mut Ctx, mut p_media_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "sceCdMmode", &PROBE_N, &[(p_media_1) as u32], false); }
    let r_ = 'f: {
    { let _ = p_media_1; };
    break 'f 1i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "sceCdMmode", &PROBE_N, &[], true); }
    r_
}

pub fn func_002BB390(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "func_002BB390", &PROBE_N, &[], false); }
    let mut l_s_1: u32 = 0;
    let r_ = 'f: {
    l_s_1 = F::getenv(c, sym::plat_sys__str0);
    break 'f (if (((((l_s_1) != (((0i32) as u32))) as i32)) != 0) { F::strtol(c, l_s_1, ((0i32) as u32), 0i32) } else { 305419896i32 });
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "func_002BB390", &PROBE_N, &[], true); }
    r_
}

pub fn Port_Srand(c: &mut Ctx, mut p_seed_1: u32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Port_Srand", &PROBE_N, &[(p_seed_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = ((p_seed_1) as u64); let a_ = sym::gPortRandNext; c.m.w64(a_, v_); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "Port_Srand", &PROBE_N, &[], true); }
    r_
}

pub fn Port_Rand(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Port_Rand", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = ((c.m.r64(sym::gPortRandNext)).wrapping_mul(6364136223846793005u64)).wrapping_add(((1i32) as u64)); let a_ = sym::gPortRandNext; c.m.w64(a_, v_); };
    break 'f (((((c.m.r64(sym::gPortRandNext)).wrapping_shr((32i32) as u32)) & (((2147483647i32) as u64)))) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Port_Rand", &PROBE_N, &[], true); }
    r_
}
