#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn McCard_GetStep(c: &mut Ctx) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McCard_GetStep", &PROBE_N, &[], false); }
    let r_ = 'f: {
    break 'f (c.m.r32(sym::gMcCardStep) as i32);
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "McCard_GetStep", &PROBE_N, &[], true); }
    r_
}

pub fn McCard_ResetStep(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McCard_ResetStep", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = sym::gMcCardStep; c.m.w32(a_, (v_) as u32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "McCard_ResetStep", &PROBE_N, &[], true); }
    r_
}

pub fn McCard_Init(c: &mut Ctx) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McCard_Init", &PROBE_N, &[], false); }
    let r_ = 'f: {
    { let _ = F::memset(c, sym::gMcCardPort, 0i32, 72u32); };
    { let _ = F::func_002A1A48(c); };
    { let _ = F::McCard_Probe(c, 0i32); };
       
    };
    if rt::probing() { crate::host::harness::probe(c, "McCard_Init", &PROBE_N, &[], true); }
    r_
}

pub fn McCard_GetInfo(c: &mut Ctx, mut p_port_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McCard_GetInfo", &PROBE_N, &[(p_port_1) as u32], false); }
    let mut l_ret_2: i32 = 0;
    let r_ = 'f: {
    if ((((((c.m.r32(sym::gMcCardCmd) as i32)) != (0i32)) as i32)) != 0) {
        { let v_ = (if ((((((c.m.r32(sym::gMcCardCmd) as i32)) == (10i32)) as i32)) != 0) { (c.m.r32(sym::gMcCardStep) as i32) } else { 0i32 }); let a_ = sym::gMcCardStep; c.m.w32(a_, (v_) as u32); };
    }
    's1: {
        let sv3: i32 = (c.m.r32(sym::gMcCardStep) as i32);
        match sv3 {
            0i32 => {
                { let v_ = 0i32; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 10i32; let a_ = sym::gMcCardCmd; c.m.w32(a_, (v_) as u32); };
                l_ret_2 = F::func_002A25B8(c, p_port_1, 0i32, ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(12), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(16), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(20));
                { let v_ = l_ret_2; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if (((((l_ret_2) < (0i32)) as i32)) != 0) {
                    if (((((l_ret_2) == ((200i32).wrapping_neg())) as i32)) != 0) {
                        { let _ = F::func_002A2498(c, 1i32, ((0i32) as u32), ((0i32) as u32)); };
                    }
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            1i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (1i32)) as i32)) != 0) {
                    { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    break 'f 1i32;
                }
                break 'f 0i32;
            }
            _ => {
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "McCard_GetInfo", &PROBE_N, &[], true); }
    r_
}

pub fn McCard_GetInfoSync(c: &mut Ctx, mut p_port_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McCard_GetInfoSync", &PROBE_N, &[(p_port_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = 0i32; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = 24i32; let a_ = sym::gMcCardCmd; c.m.w32(a_, (v_) as u32); };
    { let v_ = F::func_002A25B8(c, p_port_1, 0i32, ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(12), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(16), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(20)); let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    break 'f F::func_002A2498(c, 0i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "McCard_GetInfoSync", &PROBE_N, &[], true); }
    r_
}

pub fn McCard_Probe(c: &mut Ctx, mut p_port_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McCard_Probe", &PROBE_N, &[(p_port_1) as u32], false); }
    let r_ = 'f: {
    { let v_ = 25i32; let a_ = sym::gMcCardCmd; c.m.w32(a_, (v_) as u32); };
    { let v_ = 0i32; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
    { let v_ = F::func_002A25B8(c, p_port_1, 0i32, ((0i32) as u32), ((0i32) as u32), ((0i32) as u32)); let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
    break 'f F::func_002A2498(c, 0i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8));
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "McCard_Probe", &PROBE_N, &[], true); }
    r_
}

pub fn McCard_CheckChange(c: &mut Ctx, mut p_port_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McCard_CheckChange", &PROBE_N, &[(p_port_1) as u32], false); }
    let mut l_ret_2: i32 = 0;
    let r_ = 'f: {
    if ((((((c.m.r32(sym::gMcCardCmd) as i32)) != (0i32)) as i32)) != 0) {
        { let v_ = (if ((((((c.m.r32(sym::gMcCardCmd) as i32)) == (11i32)) as i32)) != 0) { (c.m.r32(sym::gMcCardStep) as i32) } else { 0i32 }); let a_ = sym::gMcCardStep; c.m.w32(a_, (v_) as u32); };
    }
    's1: {
        let sv3: i32 = (c.m.r32(sym::gMcCardStep) as i32);
        match sv3 {
            0i32 => {
                { let v_ = 0i32; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 11i32; let a_ = sym::gMcCardCmd; c.m.w32(a_, (v_) as u32); };
                l_ret_2 = F::func_002A25B8(c, p_port_1, 0i32, ((0i32) as u32), ((0i32) as u32), ((0i32) as u32));
                { let v_ = l_ret_2; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if (((((l_ret_2) < (0i32)) as i32)) != 0) {
                    if (((((l_ret_2) == ((200i32).wrapping_neg())) as i32)) != 0) {
                        { let _ = F::func_002A2498(c, 1i32, ((0i32) as u32), ((0i32) as u32)); };
                    }
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            1i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 1i32;
            }
            _ => {
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "McCard_CheckChange", &PROBE_N, &[], true); }
    r_
}

pub fn McCard_Format(c: &mut Ctx, mut p_port_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McCard_Format", &PROBE_N, &[(p_port_1) as u32], false); }
    let mut l_ret_2: i32 = 0;
    let r_ = 'f: {
    if ((((((c.m.r32(sym::gMcCardCmd) as i32)) != (0i32)) as i32)) != 0) {
        { let v_ = (if ((((((c.m.r32(sym::gMcCardCmd) as i32)) == (12i32)) as i32)) != 0) { (c.m.r32(sym::gMcCardStep) as i32) } else { 0i32 }); let a_ = sym::gMcCardStep; c.m.w32(a_, (v_) as u32); };
    }
    's1: {
        let sv3: i32 = (c.m.r32(sym::gMcCardStep) as i32);
        match sv3 {
            0i32 => {
                { let v_ = 0i32; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = sym::gMcCardCmd; c.m.w32(a_, (v_) as u32); };
                l_ret_2 = F::func_002A2AC8(c, p_port_1, 0i32);
                { let v_ = l_ret_2; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if (((((l_ret_2) < (0i32)) as i32)) != 0) {
                    if (((((l_ret_2) == ((200i32).wrapping_neg())) as i32)) != 0) {
                        { let _ = F::func_002A2498(c, 1i32, ((0i32) as u32), ((0i32) as u32)); };
                    }
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            1i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let v_ = 0i32; let a_ = sym::gMcCardStep; c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = sym::gMcCardCmd; c.m.w32(a_, (v_) as u32); };
                break 'f 1i32;
            }
            _ => {
                { let v_ = 0i32; let a_ = sym::gMcCardStep; c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = sym::gMcCardCmd; c.m.w32(a_, (v_) as u32); };
                break 'f 1i32;
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "McCard_Format", &PROBE_N, &[], true); }
    r_
}

pub fn McCard_CreateSave(c: &mut Ctx, mut p_port_1: i32, mut p_file_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McCard_CreateSave", &PROBE_N, &[(p_port_1) as u32, (p_file_2) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(176) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 176);
    let mut l_ret_3: i32 = 0;
    let r_ = 'f: {
        rt::memset(c, fp, 0, 64);
        rt::memset(c, fp.wrapping_add(0), 0, 16);
        { let v_ = 255i32; c.m.w32(fp.wrapping_add(0).wrapping_add(0), (v_) as u32); }
        { let v_ = 128i32; c.m.w32(fp.wrapping_add(0).wrapping_add(4), (v_) as u32); }
        { let v_ = 0i32; c.m.w32(fp.wrapping_add(0).wrapping_add(8), (v_) as u32); }
        { let v_ = 0i32; c.m.w32(fp.wrapping_add(0).wrapping_add(12), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(16), 0, 16);
        { let v_ = 255i32; c.m.w32(fp.wrapping_add(16).wrapping_add(0), (v_) as u32); }
        { let v_ = 128i32; c.m.w32(fp.wrapping_add(16).wrapping_add(4), (v_) as u32); }
        { let v_ = 0i32; c.m.w32(fp.wrapping_add(16).wrapping_add(8), (v_) as u32); }
        { let v_ = 0i32; c.m.w32(fp.wrapping_add(16).wrapping_add(12), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(32), 0, 16);
        { let v_ = 128i32; c.m.w32(fp.wrapping_add(32).wrapping_add(0), (v_) as u32); }
        { let v_ = 128i32; c.m.w32(fp.wrapping_add(32).wrapping_add(4), (v_) as u32); }
        { let v_ = 128i32; c.m.w32(fp.wrapping_add(32).wrapping_add(8), (v_) as u32); }
        { let v_ = 0i32; c.m.w32(fp.wrapping_add(32).wrapping_add(12), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(48), 0, 16);
        { let v_ = 128i32; c.m.w32(fp.wrapping_add(48).wrapping_add(0), (v_) as u32); }
        { let v_ = 128i32; c.m.w32(fp.wrapping_add(48).wrapping_add(4), (v_) as u32); }
        { let v_ = 128i32; c.m.w32(fp.wrapping_add(48).wrapping_add(8), (v_) as u32); }
        { let v_ = 0i32; c.m.w32(fp.wrapping_add(48).wrapping_add(12), (v_) as u32); }
        rt::memset(c, fp.wrapping_add(64), 0, 48);
        rt::memset(c, fp.wrapping_add(64).wrapping_add(0), 0, 16);
        { let v_ = F32(0x3f000000); c.m.w32(fp.wrapping_add(64).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f000000); c.m.w32(fp.wrapping_add(64).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f000000); c.m.w32(fp.wrapping_add(64).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(64).wrapping_add(16), 0, 16);
        { let v_ = F32(0x3f000000); c.m.w32(fp.wrapping_add(64).wrapping_add(16).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f000000); c.m.w32(fp.wrapping_add(64).wrapping_add(16).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f000000); c.m.w32(fp.wrapping_add(64).wrapping_add(16).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(16).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(64).wrapping_add(32), 0, 16);
        { let v_ = F32(0x3f000000); c.m.w32(fp.wrapping_add(64).wrapping_add(32).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3f000000); c.m.w32(fp.wrapping_add(64).wrapping_add(32).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3f000000); c.m.w32(fp.wrapping_add(64).wrapping_add(32).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(64).wrapping_add(32).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(112), 0, 48);
        rt::memset(c, fp.wrapping_add(112).wrapping_add(0), 0, 16);
        { let v_ = F32(0x3e999999); c.m.w32(fp.wrapping_add(112).wrapping_add(0).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3e999999); c.m.w32(fp.wrapping_add(112).wrapping_add(0).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3cf5c28f); c.m.w32(fp.wrapping_add(112).wrapping_add(0).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(112).wrapping_add(0).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(112).wrapping_add(16), 0, 16);
        { let v_ = F32(0x3e999999); c.m.w32(fp.wrapping_add(112).wrapping_add(16).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3e4ccccc); c.m.w32(fp.wrapping_add(112).wrapping_add(16).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3e4ccccc); c.m.w32(fp.wrapping_add(112).wrapping_add(16).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(112).wrapping_add(16).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(112).wrapping_add(32), 0, 16);
        { let v_ = F32(0x3e0f5c28); c.m.w32(fp.wrapping_add(112).wrapping_add(32).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3e0f5c28); c.m.w32(fp.wrapping_add(112).wrapping_add(32).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3e4ccccc); c.m.w32(fp.wrapping_add(112).wrapping_add(32).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(112).wrapping_add(32).wrapping_add(12), (v_).0); }
        rt::memset(c, fp.wrapping_add(160), 0, 16);
        { let v_ = F32(0x3e800000); c.m.w32(fp.wrapping_add(160).wrapping_add(0), (v_).0); }
        { let v_ = F32(0x3e800000); c.m.w32(fp.wrapping_add(160).wrapping_add(4), (v_).0); }
        { let v_ = F32(0x3e800000); c.m.w32(fp.wrapping_add(160).wrapping_add(8), (v_).0); }
        { let v_ = F32(0x00000000); c.m.w32(fp.wrapping_add(160).wrapping_add(12), (v_).0); }
        if ((((((c.m.r32(sym::gMcCardCmd) as i32)) != (0i32)) as i32)) != 0) {
            { let v_ = (if ((((((c.m.r32(sym::gMcCardCmd) as i32)) == (14i32)) as i32)) != 0) { (c.m.r32(sym::gMcCardStep) as i32) } else { 0i32 }); let a_ = sym::gMcCardStep; c.m.w32(a_, (v_) as u32); };
        }
        's1: {
            let sv4: i32 = (c.m.r32(sym::gMcCardStep) as i32);
            match sv4 {
                0i32 => {
                    { let v_ = 0i32; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 14i32; let a_ = sym::gMcCardCmd; c.m.w32(a_, (v_) as u32); };
                    l_ret_3 = F::func_002A1F80(c, p_port_1, 0i32, (p_file_2).wrapping_add(24));
                    { let v_ = l_ret_3; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    if (((((l_ret_3) < (0i32)) as i32)) != 0) {
                        if (((((l_ret_3) == ((200i32).wrapping_neg())) as i32)) != 0) {
                            { let _ = F::func_002A2498(c, 1i32, ((0i32) as u32), ((0i32) as u32)); };
                        }
                        break 'f 0i32;
                    }
                    { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    break 'f 0i32;
                }
                1i32 => {
                    if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                        if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) != ((4i32).wrapping_neg())) as i32)) != 0) {
                            break 'f 0i32;
                        }
                        { let v_ = 0i32; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    }
                    { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    break 'f 0i32;
                }
                2i32 => {
                    { let v_ = F::func_002A1E58(c, p_port_1, 0i32, (p_file_2).wrapping_add(152), 514i32); let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    break 'f 0i32;
                }
                3i32 => {
                    if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    { let v_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                    { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    break 'f 0i32;
                }
                4i32 => {
                    { let _ = F::memset(c, sym::gMcCardIconSys, 0i32, 964u32); };
                    { let _ = F::strcpy(c, sym::gMcCardIconSys, sym::stgm_a_b__str0); };
                    { let _ = F::strcpy(c, (sym::gMcCardIconSys).wrapping_add(192), (p_file_2).wrapping_add(792)); };
                    { let v_ = c.m.r16((p_file_2).wrapping_add(1052)); let a_ = (sym::gMcCardIconSys).wrapping_add(6); c.m.w16(a_, v_); };
                    { let v_ = 96i32; let a_ = (sym::gMcCardIconSys).wrapping_add(12); c.m.w32(a_, (v_) as u32); };
                    { let _ = F::memcpy(c, (sym::gMcCardIconSys).wrapping_add(16), fp, 64u32); };
                    { let _ = F::memcpy(c, (sym::gMcCardIconSys).wrapping_add(80), fp.wrapping_add(64), 48u32); };
                    { let _ = F::memcpy(c, (sym::gMcCardIconSys).wrapping_add(128), fp.wrapping_add(112), 48u32); };
                    { let _ = F::memcpy(c, (sym::gMcCardIconSys).wrapping_add(176), fp.wrapping_add(160), 16u32); };
                    { let _ = F::strcpy(c, (sym::gMcCardIconSys).wrapping_add(260), ((p_file_2).wrapping_add(860)).wrapping_add((((0i32) as u32)).wrapping_mul(64))); };
                    { let _ = F::strcpy(c, (sym::gMcCardIconSys).wrapping_add(324), ((p_file_2).wrapping_add(860)).wrapping_add((((1i32) as u32)).wrapping_mul(64))); };
                    { let _ = F::strcpy(c, (sym::gMcCardIconSys).wrapping_add(388), ((p_file_2).wrapping_add(860)).wrapping_add((((2i32) as u32)).wrapping_mul(64))); };
                    { let v_ = { let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24)) as i32); F::func_002A2320(c, a0_, sym::gMcCardIconSys, ((964u32) as i32)) }; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    break 'f 0i32;
                }
                5i32 => {
                    if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    break 'f 0i32;
                }
                6i32 => {
                    { let v_ = { let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24)) as i32); F::func_002A1FB8(c, a0_) }; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    break 'f 0i32;
                }
                7i32 => {
                    if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    break 'f 0i32;
                }
                8i32 => {
                    { let v_ = F::func_002A1E58(c, p_port_1, 0i32, ((p_file_2).wrapping_add(280)).wrapping_add((((0i32) as u32)).wrapping_mul(128)), 514i32); let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    break 'f 0i32;
                }
                9i32 => {
                    if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    { let v_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                    { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    break 'f 0i32;
                }
                10i32 => {
                    { let v_ = { let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24)) as i32); let a1_ = c.m.r32(p_file_2); let a2_ = (c.m.r32((p_file_2).wrapping_add(12)) as i32); F::func_002A2320(c, a0_, a1_, a2_) }; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    break 'f 0i32;
                }
                11i32 => {
                    if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    break 'f 0i32;
                }
                12i32 => {
                    { let v_ = { let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24)) as i32); F::func_002A1FB8(c, a0_) }; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                    if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    break 'f 0i32;
                }
                13i32 => {
                    if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                        break 'f 0i32;
                    }
                    { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    break 'f 1i32;
                }
                _ => {
                    { let v_ = 0i32; let a_ = sym::gMcCardStep; c.m.w32(a_, (v_) as u32); };
                    { let v_ = 0i32; let a_ = sym::gMcCardCmd; c.m.w32(a_, (v_) as u32); };
                    break 'f 1i32;
                }
            }
        }
       0
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "McCard_CreateSave", &PROBE_N, &[], true); }
    r_
}

pub fn McCard_WriteSave(c: &mut Ctx, mut p_port_1: i32, mut p_data_2: u32, mut p_size_3: i32, mut p_file_4: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McCard_WriteSave", &PROBE_N, &[(p_port_1) as u32, (p_data_2) as u32, (p_size_3) as u32, (p_file_4) as u32], false); }
    let mut l_ret_5: i32 = 0;
    let r_ = 'f: {
    if ((((((c.m.r32(sym::gMcCardCmd) as i32)) != (0i32)) as i32)) != 0) {
        { let v_ = (if ((((((c.m.r32(sym::gMcCardCmd) as i32)) == (15i32)) as i32)) != 0) { (c.m.r32(sym::gMcCardStep) as i32) } else { 0i32 }); let a_ = sym::gMcCardStep; c.m.w32(a_, (v_) as u32); };
    }
    's1: {
        let sv6: i32 = (c.m.r32(sym::gMcCardStep) as i32);
        match sv6 {
            0i32 => {
                { let v_ = 0i32; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 15i32; let a_ = sym::gMcCardCmd; c.m.w32(a_, (v_) as u32); };
                l_ret_5 = F::func_002A1E58(c, p_port_1, 0i32, (p_file_4).wrapping_add(664), 514i32);
                { let v_ = l_ret_5; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if (((((l_ret_5) < (0i32)) as i32)) != 0) {
                    if (((((l_ret_5) == ((200i32).wrapping_neg())) as i32)) != 0) {
                        { let _ = F::func_002A2498(c, 1i32, ((0i32) as u32), ((0i32) as u32)); };
                    }
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            1i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    { let v_ = 0i32; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    { let v_ = 2i32; let a_ = sym::gMcCardStep; c.m.w32(a_, (v_) as u32); };
                    break 'f 0i32;
                }
                { let v_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let v_ = 4i32; let a_ = sym::gMcCardStep; c.m.w32(a_, (v_) as u32); };
                break 'f 0i32;
            }
            2i32 => {
                { let v_ = F::func_002A1E58(c, p_port_1, 0i32, (p_file_4).wrapping_add(664), 2i32); let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                    if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4)) as i32)) == ((200i32).wrapping_neg())) as i32)) != 0) {
                        { let _ = F::func_002A2498(c, 1i32, ((0i32) as u32), ((0i32) as u32)); };
                    }
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            3i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let v_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            4i32 => {
                { let v_ = { let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24)) as i32); F::func_002A2320(c, a0_, p_data_2, p_size_3) }; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            5i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) != (p_size_3)) as i32)) != 0) {
                    { let v_ = (8i32).wrapping_neg(); let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            6i32 => {
                { let v_ = { let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24)) as i32); F::func_002A1FB8(c, a0_) }; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            7i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            8i32 => {
                { let v_ = F::func_002A1E58(c, p_port_1, 0i32, ((p_file_4).wrapping_add(280)).wrapping_add((((0i32) as u32)).wrapping_mul(128)), 3i32); let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            9i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) == ((4i32).wrapping_neg())) as i32)) != 0) {
                        { let v_ = 0i32; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                        { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    }
                    break 'f 0i32;
                }
                { let v_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let v_ = 12i32; let a_ = sym::gMcCardStep; c.m.w32(a_, (v_) as u32); };
                break 'f 0i32;
            }
            10i32 => {
                { let v_ = F::func_002A1E58(c, p_port_1, 0i32, ((p_file_4).wrapping_add(280)).wrapping_add((((0i32) as u32)).wrapping_mul(128)), 514i32); let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            11i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let v_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let v_ = 16i32; let a_ = sym::gMcCardStep; c.m.w32(a_, (v_) as u32); };
                break 'f 0i32;
            }
            12i32 => {
                { let v_ = { let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24)) as i32); F::func_002A2078(c, a0_, 0i32, 2i32) }; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            13i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32((p_file_4).wrapping_add(12)) as i32)) != ((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32))) as i32)) != 0) {
                    { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    break 'f 0i32;
                }
                { let v_ = 18i32; let a_ = sym::gMcCardStep; c.m.w32(a_, (v_) as u32); };
                break 'f 0i32;
            }
            14i32 => {
                { let v_ = { let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24)) as i32); F::func_002A2078(c, a0_, 0i32, 0i32) }; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            15i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            16i32 => {
                { let v_ = { let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24)) as i32); let a1_ = c.m.r32(p_file_4); let a2_ = (c.m.r32((p_file_4).wrapping_add(12)) as i32); F::func_002A2320(c, a0_, a1_, a2_) }; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            17i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            18i32 => {
                { let v_ = { let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24)) as i32); F::func_002A1FB8(c, a0_) }; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            19i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 1i32;
            }
            _ => {
                { let v_ = 0i32; let a_ = sym::gMcCardStep; c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = sym::gMcCardCmd; c.m.w32(a_, (v_) as u32); };
                break 'f 1i32;
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "McCard_WriteSave", &PROBE_N, &[], true); }
    r_
}

pub fn McCard_ReadFile(c: &mut Ctx, mut p_port_1: i32, mut p_buf_2: u32, mut p_size_3: i32, mut p_file_4: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McCard_ReadFile", &PROBE_N, &[(p_port_1) as u32, (p_buf_2) as u32, (p_size_3) as u32, (p_file_4) as u32], false); }
    let mut l_ret_5: i32 = 0;
    let r_ = 'f: {
    if ((((((c.m.r32(sym::gMcCardCmd) as i32)) != (0i32)) as i32)) != 0) {
        { let v_ = (if ((((((c.m.r32(sym::gMcCardCmd) as i32)) == (16i32)) as i32)) != 0) { (c.m.r32(sym::gMcCardStep) as i32) } else { 0i32 }); let a_ = sym::gMcCardStep; c.m.w32(a_, (v_) as u32); };
    }
    's1: {
        let sv6: i32 = (c.m.r32(sym::gMcCardStep) as i32);
        match sv6 {
            0i32 => {
                { let v_ = 0i32; let a_ = (sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 16i32; let a_ = sym::gMcCardCmd; c.m.w32(a_, (v_) as u32); };
                l_ret_5 = F::func_002A1E58(c, p_port_1, 0i32, (p_file_4).wrapping_add(664), 1i32);
                { let v_ = l_ret_5; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if (((((l_ret_5) < (0i32)) as i32)) != 0) {
                    if (((((l_ret_5) == ((200i32).wrapping_neg())) as i32)) != 0) {
                        { let _ = F::func_002A2498(c, 1i32, ((0i32) as u32), ((0i32) as u32)); };
                    }
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            1i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    { let v_ = 0i32; let a_ = sym::gMcCardStep; c.m.w32(a_, (v_) as u32); };
                    break 'f 0i32;
                }
                { let v_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            2i32 => {
                { let v_ = { let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24)) as i32); F::func_002A2208(c, a0_, p_buf_2, p_size_3) }; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            3i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    { let v_ = 0i32; let a_ = sym::gMcCardStep; c.m.w32(a_, (v_) as u32); };
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) != (p_size_3)) as i32)) != 0) {
                    { let r_ = 1i32; let a_ = (sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                } else {
                    { let r_ = 2i32; let a_ = (sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            4i32 => {
                { let v_ = { let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24)) as i32); F::func_002A1FB8(c, a0_) }; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            5i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    { let v_ = 0i32; let a_ = sym::gMcCardStep; c.m.w32(a_, (v_) as u32); };
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 1i32;
            }
            _ => {
                break 'f 1i32;
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "McCard_ReadFile", &PROBE_N, &[], true); }
    r_
}

pub fn McCard_LoadFile(c: &mut Ctx, mut p_port_1: i32, mut p_file_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McCard_LoadFile", &PROBE_N, &[(p_port_1) as u32, (p_file_2) as u32], false); }
    let mut l_ret_3: i32 = 0;
    let r_ = 'f: {
    if ((((((c.m.r32(sym::gMcCardCmd) as i32)) != (0i32)) as i32)) != 0) {
        { let v_ = (if ((((((c.m.r32(sym::gMcCardCmd) as i32)) == (17i32)) as i32)) != 0) { (c.m.r32(sym::gMcCardStep) as i32) } else { 0i32 }); let a_ = sym::gMcCardStep; c.m.w32(a_, (v_) as u32); };
    }
    's1: {
        let sv4: i32 = (c.m.r32(sym::gMcCardStep) as i32);
        match sv4 {
            0i32 => {
                { let v_ = ((0i32) as u32); let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(32); c.m.w32(a_, v_); };
                { let v_ = 0i32; let a_ = (sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36)); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 17i32; let a_ = sym::gMcCardCmd; c.m.w32(a_, (v_) as u32); };
                l_ret_3 = F::func_002A1E58(c, p_port_1, 0i32, (p_file_2).wrapping_add(664), 1i32);
                { let v_ = l_ret_3; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if (((((l_ret_3) < (0i32)) as i32)) != 0) {
                    if (((((l_ret_3) == ((200i32).wrapping_neg())) as i32)) != 0) {
                        { let _ = F::func_002A2498(c, 1i32, ((0i32) as u32), ((0i32) as u32)); };
                    }
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            1i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let v_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            2i32 => {
                { let v_ = { let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24)) as i32); F::func_002A2078(c, a0_, 0i32, 2i32) }; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            3i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let v_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            4i32 => {
                { let v_ = { let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24)) as i32); F::func_002A2078(c, a0_, 0i32, 0i32) }; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            5i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            6i32 => {
                { let v_ = { let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(28)) as i32); F::Heap_Alloc(c, a0_, ((32i32) as u32), 0i32, 2i32) }; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(32); c.m.w32(a_, v_); };
                { let v_ = { let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24)) as i32); let a1_ = c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(32)); let a2_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(28)) as i32); F::func_002A2208(c, a0_, a1_, a2_) }; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            7i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) != ((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(28)) as i32))) as i32)) != 0) {
                    { let r_ = 1i32; let a_ = (sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36)); let n_ = (((c.m.r32(a_) as i32)) | (r_)); c.m.w32(a_, (n_) as u32); };
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            8i32 => {
                { let v_ = { let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24)) as i32); F::func_002A1FB8(c, a0_) }; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            9i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 1i32;
            }
            _ => {
                break 'f 1i32;
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "McCard_LoadFile", &PROBE_N, &[], true); }
    r_
}

pub fn McCard_CheckFiles(c: &mut Ctx, mut p_port_1: i32, mut p_file_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McCard_CheckFiles", &PROBE_N, &[(p_port_1) as u32, (p_file_2) as u32], false); }
    let mut l_ret_3: i32 = 0;
    let mut l_r_4: i32 = 0;
    let r_ = 'f: {
    if ((((((c.m.r32(sym::gMcCardCmd) as i32)) != (0i32)) as i32)) != 0) {
        { let v_ = (if ((((((c.m.r32(sym::gMcCardCmd) as i32)) == (18i32)) as i32)) != 0) { (c.m.r32(sym::gMcCardStep) as i32) } else { 0i32 }); let a_ = sym::gMcCardStep; c.m.w32(a_, (v_) as u32); };
    }
    's1: {
        let sv5: i32 = (c.m.r32(sym::gMcCardStep) as i32);
        match sv5 {
            0i32 => {
                { let v_ = 0i32; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 18i32; let a_ = sym::gMcCardCmd; c.m.w32(a_, (v_) as u32); };
                l_ret_3 = F::func_002A1E58(c, p_port_1, 0i32, (p_file_2).wrapping_add(664), 1i32);
                { let v_ = l_ret_3; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if (((((l_ret_3) < (0i32)) as i32)) != 0) {
                    if (((((l_ret_3) == ((200i32).wrapping_neg())) as i32)) != 0) {
                        { let _ = F::func_002A2498(c, 1i32, ((0i32) as u32), ((0i32) as u32)); };
                        break 'f 0i32;
                    }
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            1i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) != (1i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let v_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            2i32 => {
                { let v_ = { let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24)) as i32); F::func_002A1FB8(c, a0_) }; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            3i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            4i32 => {
                { let v_ = { l_r_4 = F::func_002A1E58(c, p_port_1, 0i32, (p_file_2).wrapping_add(152), 1i32); l_r_4 }; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if (((((l_r_4) < (0i32)) as i32)) != 0) {
                    if (((((l_r_4) == ((200i32).wrapping_neg())) as i32)) != 0) {
                        { let _ = F::func_002A2498(c, 1i32, ((0i32) as u32), ((0i32) as u32)); };
                        break 'f 0i32;
                    }
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            5i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) != (1i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let v_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            6i32 => {
                { let v_ = { let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24)) as i32); F::func_002A1FB8(c, a0_) }; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            7i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            8i32 => {
                l_r_4 = F::func_002A1E58(c, p_port_1, 0i32, ((p_file_2).wrapping_add(280)).wrapping_add((((0i32) as u32)).wrapping_mul(128)), 1i32);
                { let v_ = l_r_4; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if (((((l_r_4) < (0i32)) as i32)) != 0) {
                    if (((((l_r_4) == ((200i32).wrapping_neg())) as i32)) != 0) {
                        { let _ = F::func_002A2498(c, 1i32, ((0i32) as u32), ((0i32) as u32)); };
                        break 'f 0i32;
                    }
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            9i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) != (1i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let v_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            10i32 => {
                { let v_ = { let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24)) as i32); F::func_002A1FB8(c, a0_) }; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            11i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 1i32;
            }
            _ => {
                break 'f 1i32;
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "McCard_CheckFiles", &PROBE_N, &[], true); }
    r_
}

pub fn McCardFile_SetNames(c: &mut Ctx, mut p_file_1: u32, mut p_name_2: u32, mut p_num_3: i32, mut p_unused_4: i32, mut p_replay_5: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McCardFile_SetNames", &PROBE_N, &[(p_file_1) as u32, (p_name_2) as u32, (p_num_3) as u32, (p_unused_4) as u32, (p_replay_5) as u32], false); }
    let r_ = 'f: {
    if (((((p_num_3) >= (0i32)) as i32)) != 0) {
        { let _ = F::memset(c, (p_file_1).wrapping_add(24), 0i32, ((128i32) as u32)); };
        { let _ = F::sprintf(c, (p_file_1).wrapping_add(24), sym::stgm_a_b__str1, &[((sym::stgm_a_b__str2) as u64), ((p_name_2) as u64), ((p_num_3) as i64 as u64)]); };
    } else {
        { let _ = F::memset(c, (p_file_1).wrapping_add(24), 0i32, ((128i32) as u32)); };
        { let _ = F::sprintf(c, (p_file_1).wrapping_add(24), sym::stgm_a_b__str3, &[((sym::stgm_a_b__str4) as u64), ((p_name_2) as u64)]); };
    }
    { let _ = F::memset(c, (p_file_1).wrapping_add(152), 0i32, ((128i32) as u32)); };
    { let _ = F::sprintf(c, (p_file_1).wrapping_add(152), sym::stgm_a_b__str5, &[(((p_file_1).wrapping_add(24)) as u64)]); };
    { let _ = F::memset(c, ((p_file_1).wrapping_add(280)).wrapping_add((((0i32) as u32)).wrapping_mul(128)), 0i32, ((128i32) as u32)); };
    if (((((p_replay_5) == (0i32)) as i32)) != 0) {
        { let _ = F::sprintf(c, ((p_file_1).wrapping_add(280)).wrapping_add((((0i32) as u32)).wrapping_mul(128)), sym::stgm_a_b__str6, &[(((p_file_1).wrapping_add(24)) as u64)]); };
    } else {
        { let _ = F::sprintf(c, ((p_file_1).wrapping_add(280)).wrapping_add((((0i32) as u32)).wrapping_mul(128)), sym::stgm_a_b__str7, &[(((p_file_1).wrapping_add(24)) as u64)]); };
    }
    { let _ = F::memset(c, ((p_file_1).wrapping_add(280)).wrapping_add((((1i32) as u32)).wrapping_mul(128)), 0i32, ((128i32) as u32)); };
    if (((((p_replay_5) == (0i32)) as i32)) != 0) {
        { let _ = F::sprintf(c, ((p_file_1).wrapping_add(280)).wrapping_add((((1i32) as u32)).wrapping_mul(128)), sym::stgm_a_b__str8, &[(((p_file_1).wrapping_add(24)) as u64)]); };
    } else {
        { let _ = F::sprintf(c, ((p_file_1).wrapping_add(280)).wrapping_add((((1i32) as u32)).wrapping_mul(128)), sym::stgm_a_b__str9, &[(((p_file_1).wrapping_add(24)) as u64)]); };
    }
    { let _ = F::memset(c, ((p_file_1).wrapping_add(280)).wrapping_add((((2i32) as u32)).wrapping_mul(128)), 0i32, ((128i32) as u32)); };
    if (((((p_replay_5) == (0i32)) as i32)) != 0) {
        { let _ = F::sprintf(c, ((p_file_1).wrapping_add(280)).wrapping_add((((2i32) as u32)).wrapping_mul(128)), sym::stgm_a_b__str10, &[(((p_file_1).wrapping_add(24)) as u64)]); };
    } else {
        { let _ = F::sprintf(c, ((p_file_1).wrapping_add(280)).wrapping_add((((2i32) as u32)).wrapping_mul(128)), sym::stgm_a_b__str11, &[(((p_file_1).wrapping_add(24)) as u64)]); };
    }
    { let _ = F::memset(c, (p_file_1).wrapping_add(664), 0i32, ((128i32) as u32)); };
    { let _ = F::sprintf(c, (p_file_1).wrapping_add(664), sym::stgm_a_b__str12, &[(((p_file_1).wrapping_add(24)) as u64), (((p_file_1).wrapping_add(24)) as u64)]); };
    { let _ = F::memset(c, ((p_file_1).wrapping_add(860)).wrapping_add((((0i32) as u32)).wrapping_mul(64)), 0i32, ((64i32) as u32)); };
    if (((((p_replay_5) == (0i32)) as i32)) != 0) {
        { let _ = F::strcpy(c, ((p_file_1).wrapping_add(860)).wrapping_add((((0i32) as u32)).wrapping_mul(64)), sym::stgm_a_b__str13); };
    } else {
        { let _ = F::strcpy(c, ((p_file_1).wrapping_add(860)).wrapping_add((((0i32) as u32)).wrapping_mul(64)), sym::stgm_a_b__str14); };
    }
    { let _ = F::memset(c, ((p_file_1).wrapping_add(860)).wrapping_add((((1i32) as u32)).wrapping_mul(64)), 0i32, ((64i32) as u32)); };
    if (((((p_replay_5) == (0i32)) as i32)) != 0) {
        { let _ = F::strcpy(c, ((p_file_1).wrapping_add(860)).wrapping_add((((1i32) as u32)).wrapping_mul(64)), sym::stgm_a_b__str15); };
    } else {
        { let _ = F::strcpy(c, ((p_file_1).wrapping_add(860)).wrapping_add((((1i32) as u32)).wrapping_mul(64)), sym::stgm_a_b__str16); };
    }
    { let _ = F::memset(c, ((p_file_1).wrapping_add(860)).wrapping_add((((2i32) as u32)).wrapping_mul(64)), 0i32, ((64i32) as u32)); };
    if (((((p_replay_5) == (0i32)) as i32)) != 0) {
        { let _ = F::strcpy(c, ((p_file_1).wrapping_add(860)).wrapping_add((((2i32) as u32)).wrapping_mul(64)), sym::stgm_a_b__str17); };
    } else {
        { let _ = F::strcpy(c, ((p_file_1).wrapping_add(860)).wrapping_add((((2i32) as u32)).wrapping_mul(64)), sym::stgm_a_b__str18); };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "McCardFile_SetNames", &PROBE_N, &[], true); }
    r_
}

pub fn McCard_IsFatalError(c: &mut Ctx, mut p_code_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McCard_IsFatalError", &PROBE_N, &[(p_code_1) as u32], false); }
    let r_ = 'f: {
    if (((((((((p_code_1) >= ((19i32).wrapping_neg())) as i32)) != 0) && (((((p_code_1) <= ((11i32).wrapping_neg())) as i32)) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    if (((((((((p_code_1) >= ((49i32).wrapping_neg())) as i32)) != 0) && (((((p_code_1) <= ((40i32).wrapping_neg())) as i32)) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    if (((((((((p_code_1) >= ((59i32).wrapping_neg())) as i32)) != 0) && (((((p_code_1) <= ((50i32).wrapping_neg())) as i32)) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    if (((((((((p_code_1) >= ((79i32).wrapping_neg())) as i32)) != 0) && (((((p_code_1) <= ((70i32).wrapping_neg())) as i32)) != 0)) as i32)) != 0) {
        break 'f 1i32;
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "McCard_IsFatalError", &PROBE_N, &[], true); }
    r_
}

pub fn McCard_ReadDir(c: &mut Ctx, mut p_port_1: i32, mut p_maxEntries_2: i32, mut p_table_3: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McCard_ReadDir", &PROBE_N, &[(p_port_1) as u32, (p_maxEntries_2) as u32, (p_table_3) as u32], false); }
    let mut l_ret_4: i32 = 0;
    let r_ = 'f: {
    if ((((((c.m.r32(sym::gMcCardCmd) as i32)) != (0i32)) as i32)) != 0) {
        { let v_ = (if ((((((c.m.r32(sym::gMcCardCmd) as i32)) == (22i32)) as i32)) != 0) { (c.m.r32(sym::gMcCardStep) as i32) } else { 0i32 }); let a_ = sym::gMcCardStep; c.m.w32(a_, (v_) as u32); };
    }
    's1: {
        let sv5: i32 = (c.m.r32(sym::gMcCardStep) as i32);
        match sv5 {
            0i32 => {
                { let v_ = 0i32; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 22i32; let a_ = sym::gMcCardCmd; c.m.w32(a_, (v_) as u32); };
                l_ret_4 = F::func_002A27A8(c, p_port_1, 0i32, sym::stgm_a_b__str19, 0i32, p_maxEntries_2, p_table_3);
                { let v_ = l_ret_4; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if (((((l_ret_4) < (0i32)) as i32)) != 0) {
                    if (((((l_ret_4) == ((200i32).wrapping_neg())) as i32)) != 0) {
                        { let _ = F::func_002A2498(c, 1i32, ((0i32) as u32), ((0i32) as u32)); };
                        break 'f 0i32;
                    }
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            1i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let v_ = 0i32; let a_ = sym::gMcCardStep; c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = sym::gMcCardCmd; c.m.w32(a_, (v_) as u32); };
                break 'f 1i32;
            }
            _ => {
                { let v_ = 0i32; let a_ = sym::gMcCardStep; c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = sym::gMcCardCmd; c.m.w32(a_, (v_) as u32); };
                break 'f 1i32;
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "McCard_ReadDir", &PROBE_N, &[], true); }
    r_
}

pub fn McCard_FindSystemSave(c: &mut Ctx, mut p_port_1: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McCard_FindSystemSave", &PROBE_N, &[(p_port_1) as u32], false); }
    let mut l_ret_2: i32 = 0;
    let r_ = 'f: {
    if ((((((c.m.r32(sym::gMcCardCmd) as i32)) != (0i32)) as i32)) != 0) {
        { let v_ = (if ((((((c.m.r32(sym::gMcCardCmd) as i32)) == (23i32)) as i32)) != 0) { (c.m.r32(sym::gMcCardStep) as i32) } else { 0i32 }); let a_ = sym::gMcCardStep; c.m.w32(a_, (v_) as u32); };
    }
    's1: {
        let sv3: i32 = (c.m.r32(sym::gMcCardStep) as i32);
        match sv3 {
            0i32 => {
                { let v_ = 0i32; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 23i32; let a_ = sym::gMcCardCmd; c.m.w32(a_, (v_) as u32); };
                l_ret_2 = F::func_002A27A8(c, p_port_1, 0i32, sym::stgm_a_b__str20, 0i32, (1i32).wrapping_neg(), ((0i32) as u32));
                { let v_ = l_ret_2; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if (((((l_ret_2) < (0i32)) as i32)) != 0) {
                    if (((((l_ret_2) == ((200i32).wrapping_neg())) as i32)) != 0) {
                        { let _ = F::func_002A2498(c, 1i32, ((0i32) as u32), ((0i32) as u32)); };
                        break 'f 0i32;
                    }
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            1i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (1i32)) as i32)) != 0) {
                    { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    break 'f 1i32;
                }
                break 'f 0i32;
            }
            _ => {
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "McCard_FindSystemSave", &PROBE_N, &[], true); }
    r_
}

pub fn McCard_FindSave(c: &mut Ctx, mut p_port_1: i32, mut p_file_2: u32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McCard_FindSave", &PROBE_N, &[(p_port_1) as u32, (p_file_2) as u32], false); }
    let mut l_ret_3: i32 = 0;
    let r_ = 'f: {
    if ((((((c.m.r32(sym::gMcCardCmd) as i32)) != (0i32)) as i32)) != 0) {
        { let v_ = (if ((((((c.m.r32(sym::gMcCardCmd) as i32)) == (23i32)) as i32)) != 0) { (c.m.r32(sym::gMcCardStep) as i32) } else { 0i32 }); let a_ = sym::gMcCardStep; c.m.w32(a_, (v_) as u32); };
    }
    's1: {
        let sv4: i32 = (c.m.r32(sym::gMcCardStep) as i32);
        match sv4 {
            0i32 => {
                { let v_ = 0i32; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 23i32; let a_ = sym::gMcCardCmd; c.m.w32(a_, (v_) as u32); };
                l_ret_3 = F::func_002A27A8(c, p_port_1, 0i32, (p_file_2).wrapping_add(24), 0i32, (1i32).wrapping_neg(), ((0i32) as u32));
                { let v_ = l_ret_3; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if (((((l_ret_3) < (0i32)) as i32)) != 0) {
                    if (((((l_ret_3) == ((200i32).wrapping_neg())) as i32)) != 0) {
                        { let _ = F::func_002A2498(c, 1i32, ((0i32) as u32), ((0i32) as u32)); };
                        break 'f 0i32;
                    }
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            1i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (1i32)) as i32)) != 0) {
                    { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                    break 'f 1i32;
                }
                break 'f 0i32;
            }
            _ => {
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "McCard_FindSave", &PROBE_N, &[], true); }
    r_
}

pub fn McCard_Checksum(c: &mut Ctx, mut p_buf_1: u32, mut p_size_2: i32, mut p_write_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McCard_Checksum", &PROBE_N, &[(p_buf_1) as u32, (p_size_2) as u32, (p_write_3) as u32], false); }
    let mut l_sum_4: u32 = 0;
    let mut l_p_5: u32 = 0;
    let mut l_i_6: i32 = 0;
    let r_ = 'f: {
    l_sum_4 = ((0i32) as u32);
    if (((((p_size_2) <= (0i32)) as i32)) != 0) {
        break 'f 1i32;
    }
    l_p_5 = (p_buf_1).wrapping_add(((8i32) as u32));
    { let r_ = 8i32; let n_ = (p_size_2).wrapping_sub(r_); p_size_2 = n_; };
    if (((((p_write_3) == (1i32)) as i32)) != 0) {
        { let v_ = ((0i32) as u32); let a_ = (p_buf_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((0i32) as u32); let a_ = (p_buf_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    }
    {
        l_i_6 = 0i32;
        'l1: loop {
            if !(((((l_i_6) < (p_size_2)) as i32)) != 0) { break 'l1; }
            'c2: {
                if ((((l_i_6) & (1i32))) != 0) {
                    { let r_ = (((((c.m.r8(l_p_5)) as i32)).wrapping_shl((8i32) as u32)) as u32); let n_ = (l_sum_4).wrapping_add(r_); l_sum_4 = n_; };
                } else {
                    { let r_ = ((c.m.r8(l_p_5)) as u32); let n_ = (l_sum_4).wrapping_add(r_); l_sum_4 = n_; };
                }
                { let o_ = l_p_5; let n_ = (o_).wrapping_add(1); l_p_5 = n_; };
                { let r_ = ((((((l_sum_4) > (((65535i32) as u32))) as i32)).wrapping_mul(65536i32)) as u32); let n_ = (l_sum_4).wrapping_sub(r_); l_sum_4 = n_; };
            }
            { let o_ = l_i_6; let n_ = (o_).wrapping_add(1); l_i_6 = n_; };
        }
    }
    if (((((p_write_3) == (1i32)) as i32)) != 0) {
        { let v_ = (l_sum_4).wrapping_shr((8i32) as u32); let a_ = (p_buf_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
        { let v_ = ((l_sum_4) & (((255i32) as u32))); let a_ = (p_buf_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)); c.m.w32(a_, v_); };
    } else {
        if (((((((((c.m.r32((p_buf_1).wrapping_add((((0i32) as u32)).wrapping_mul(4)))) != ((l_sum_4).wrapping_shr((8i32) as u32))) as i32)) != 0) && (((((c.m.r32((p_buf_1).wrapping_add((((1i32) as u32)).wrapping_mul(4)))) != (((l_sum_4) & (((255i32) as u32))))) as i32)) != 0)) as i32)) != 0) {
            break 'f 1i32;
        }
    }
    break 'f 0i32;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "McCard_Checksum", &PROBE_N, &[], true); }
    r_
}

pub fn McCard_CalcNeed(c: &mut Ctx, mut p_port_1: i32, mut p_file_2: u32, mut p_replay_3: i32) -> i32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "McCard_CalcNeed", &PROBE_N, &[(p_port_1) as u32, (p_file_2) as u32, (p_replay_3) as u32], false); }
    let mut l_ret_4: i32 = 0;
    let mut l_needData_5: i32 = 0;
    let mut l_needSys_6: i32 = 0;
    let mut l_needIcon_7: i32 = 0;
    let r_ = 'f: {
    if (((((p_replay_3) == (0i32)) as i32)) != 0) {
        l_needData_5 = 16i32;
        l_needSys_6 = 1i32;
        l_needIcon_7 = 40i32;
    } else {
        l_needData_5 = 107i32;
        l_needSys_6 = 1i32;
        l_needIcon_7 = 48i32;
    }
    if ((((((c.m.r32(sym::gMcCardCmd) as i32)) != (0i32)) as i32)) != 0) {
        { let v_ = (if ((((((c.m.r32(sym::gMcCardCmd) as i32)) == (26i32)) as i32)) != 0) { (c.m.r32(sym::gMcCardStep) as i32) } else { 0i32 }); let a_ = sym::gMcCardStep; c.m.w32(a_, (v_) as u32); };
    }
    's1: {
        let sv8: i32 = (c.m.r32(sym::gMcCardStep) as i32);
        match sv8 {
            0i32 => {
                { let v_ = 0i32; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(28); c.m.w32(a_, (v_) as u32); };
                { let v_ = 0i32; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8); c.m.w32(a_, (v_) as u32); };
                { let v_ = 26i32; let a_ = sym::gMcCardCmd; c.m.w32(a_, (v_) as u32); };
                l_ret_4 = F::func_002A1E58(c, p_port_1, 0i32, (p_file_2).wrapping_add(664), 1i32);
                { let v_ = l_ret_4; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if (((((l_ret_4) < (0i32)) as i32)) != 0) {
                    if (((((l_ret_4) == ((200i32).wrapping_neg())) as i32)) != 0) {
                        { let _ = F::func_002A2498(c, 1i32, ((0i32) as u32), ((0i32) as u32)); };
                        break 'f 0i32;
                    }
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            1i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) != (1i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let v_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            2i32 => {
                { let v_ = { let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24)) as i32); F::func_002A2078(c, a0_, 0i32, 2i32) }; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            3i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let r_ = (l_needData_5).wrapping_sub(rt::div_i32(((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)).wrapping_add(1023i32), 1024i32)); let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(28); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            4i32 => {
                { let v_ = { let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24)) as i32); F::func_002A2078(c, a0_, 0i32, 0i32) }; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            5i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            6i32 => {
                { let v_ = { let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24)) as i32); F::func_002A1FB8(c, a0_) }; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            7i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            8i32 => {
                l_ret_4 = F::func_002A1E58(c, p_port_1, 0i32, (p_file_2).wrapping_add(152), 1i32);
                { let v_ = l_ret_4; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if (((((l_ret_4) < (0i32)) as i32)) != 0) {
                    if (((((l_ret_4) == ((200i32).wrapping_neg())) as i32)) != 0) {
                        { let _ = F::func_002A2498(c, 1i32, ((0i32) as u32), ((0i32) as u32)); };
                        break 'f 0i32;
                    }
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            9i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) != (1i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let v_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            10i32 => {
                { let v_ = { let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24)) as i32); F::func_002A2078(c, a0_, 0i32, 2i32) }; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            11i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let r_ = (l_needSys_6).wrapping_sub(rt::div_i32(((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)).wrapping_add(1023i32), 1024i32)); let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(28); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            12i32 => {
                { let v_ = { let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24)) as i32); F::func_002A2078(c, a0_, 0i32, 0i32) }; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            13i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            14i32 => {
                { let v_ = { let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24)) as i32); F::func_002A1FB8(c, a0_) }; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            15i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            16i32 => {
                l_ret_4 = F::func_002A1E58(c, p_port_1, 0i32, ((p_file_2).wrapping_add(280)).wrapping_add((((0i32) as u32)).wrapping_mul(128)), 1i32);
                { let v_ = l_ret_4; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if (((((l_ret_4) < (0i32)) as i32)) != 0) {
                    if (((((l_ret_4) == ((200i32).wrapping_neg())) as i32)) != 0) {
                        { let _ = F::func_002A2498(c, 1i32, ((0i32) as u32), ((0i32) as u32)); };
                        break 'f 0i32;
                    }
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            17i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) != (1i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let v_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32); let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24); c.m.w32(a_, (v_) as u32); };
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            18i32 => {
                { let v_ = { let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24)) as i32); F::func_002A2078(c, a0_, 0i32, 2i32) }; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            19i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let r_ = (l_needIcon_7).wrapping_sub(rt::div_i32(((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)).wrapping_add(1023i32), 1024i32)); let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(28); let n_ = ((c.m.r32(a_) as i32)).wrapping_add(r_); c.m.w32(a_, (n_) as u32); };
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            20i32 => {
                { let v_ = { let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24)) as i32); F::func_002A2078(c, a0_, 0i32, 0i32) }; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            21i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            22i32 => {
                { let v_ = { let a0_ = (c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(24)) as i32); F::func_002A1FB8(c, a0_) }; let a_ = ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4); c.m.w32(a_, (v_) as u32); };
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(4)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 0i32;
            }
            23i32 => {
                if (((((F::func_002A2498(c, 1i32, ((0i32) as u32), ((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8))) == (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                if ((((((c.m.r32(((sym::gMcCardPort).wrapping_add((((p_port_1) as u32)).wrapping_mul(36))).wrapping_add(8)) as i32)) < (0i32)) as i32)) != 0) {
                    break 'f 0i32;
                }
                { let a_ = sym::gMcCardStep; let o_ = (c.m.r32(a_) as i32); let n_ = (o_).wrapping_add(1); c.m.w32(a_, (n_) as u32); };
                break 'f 1i32;
            }
            _ => {
                break 'f 1i32;
            }
        }
    }
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "McCard_CalcNeed", &PROBE_N, &[], true); }
    r_
}
