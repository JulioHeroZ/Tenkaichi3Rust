#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]
use crate::rt::prelude::*;

pub fn plat_gs__Port_DmaTransfer(c: &mut Ctx, mut p_channel_1: i32) {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "plat_gs__Port_DmaTransfer", &PROBE_N, &[(p_channel_1) as u32], false); }
    let mut l_chain_2: i32 = 0;
    let r_ = 'f: {
    if (((((((((p_channel_1) == (1i32)) as i32)) != 0) && ((((((((c.m.r32(268472320u32)).wrapping_shr((2i32) as u32)) & (((3i32) as u32)))) == (((1i32) as u32))) as i32)) != 0)) as i32)) != 0) {
        { let a0_ = c.m.r32(268472368u32); let a1_ = (((((c.m.r32(268472320u32)).wrapping_shr((6i32) as u32)) & (((1i32) as u32)))) as i32); F::Port_GsVif1Chain(c, a0_, a1_) };
    }
    if (((((p_channel_1) == (2i32)) as i32)) != 0) {
        l_chain_2 = ((((((c.m.r32(268476416u32)).wrapping_shr((2i32) as u32)) & (((3i32) as u32)))) == (((1i32) as u32))) as i32);
        { let a0_ = (if ((l_chain_2) != 0) { c.m.r32(268476464u32) } else { c.m.r32(268476432u32) }); let a1_ = c.m.r32(268476448u32); F::Port_GsGifChannel(c, a0_, a1_, l_chain_2) };
    }
       
    };
    if rt::probing() { crate::host::harness::probe(c, "plat_gs__Port_DmaTransfer", &PROBE_N, &[], true); }
    r_
}

pub fn Port_DmaChcr(c: &mut Ctx, mut p_channel_1: i32) -> u32 {
    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();
    if rt::probing() { crate::host::harness::probe(c, "Port_DmaChcr", &PROBE_N, &[(p_channel_1) as u32], false); }
    let mut l_chcr_2: u32 = 0;
    let r_ = 'f: {
    l_chcr_2 = ((268468224u32).wrapping_add((4096u32).wrapping_mul(((p_channel_1) as u32)))).wrapping_sub((if (((((p_channel_1) == (4i32)) as i32)) != 0) { 3072u32 } else { 0u32 }));
    if ((((c.m.r32(l_chcr_2)) & (256u32))) != 0) {
        F::plat_gs__Port_DmaTransfer(c, p_channel_1);
        { let r_ = (!(256u32)); let a_ = l_chcr_2; let n_ = ((c.m.r32(a_)) & (r_)); c.m.w32(a_, n_); };
    }
    break 'f l_chcr_2;
       0
    };
    if rt::probing() { crate::host::harness::probe(c, "Port_DmaChcr", &PROBE_N, &[], true); }
    r_
}
