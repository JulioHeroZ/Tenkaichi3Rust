#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn Port_GsMarker(c: &mut Ctx, mut p_effect_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Port_GsMarker", &PROBE_N, &[(p_effect_1) as u32], false); }
    let sp_ = c.sp;
    let fp: u32 = c.sp.wrapping_sub(48) & !15;
    c.sp = fp;
    c.frame_check(fp);
    rt::memset(c, fp, 0, 48);
    let r_ = 'f: {
        rt::memset(c, fp, 0, 48);
        { let v_ = ((268435456u32) | (((2i32) as u32))); c.m.w32(fp.wrapping_add(0), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(4), v_); }
        { let v_ = 268435456u32; c.m.w32(fp.wrapping_add(8), v_); }
        { let v_ = ((1342177280u32) | (((2i32) as u32))); c.m.w32(fp.wrapping_add(12), v_); }
        { let v_ = ((32768u32) | (((1i32) as u32))); c.m.w32(fp.wrapping_add(16), v_); }
        { let v_ = 268435456u32; c.m.w32(fp.wrapping_add(20), v_); }
        { let v_ = ((14i32) as u32); c.m.w32(fp.wrapping_add(24), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(28), v_); }
        { let v_ = ((p_effect_1) as u32); c.m.w32(fp.wrapping_add(32), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(36), v_); }
        { let v_ = ((127i32) as u32); c.m.w32(fp.wrapping_add(40), v_); }
        { let v_ = ((0i32) as u32); c.m.w32(fp.wrapping_add(44), v_); }
        F::Dma_AddData(c, fp, ((48u32) as i32));
       
    };
    c.sp = sp_;
    if rt::probing() { crate::host::harness::probe(c, "Port_GsMarker", &PROBE_N, &[], true); }
    r_
}
