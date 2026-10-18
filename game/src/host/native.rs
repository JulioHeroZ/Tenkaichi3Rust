//! Native versions of translated helpers whose C does nothing but move bits through memory (the number model's
//! float <-> bit-pattern conversions: a memcpy of four bytes between two locals). Translated literally they cost
//! a stack frame in emulated memory, its zeroing and a copy on every float operation of the vector library; here
//! they are the bit pattern itself. Same results, same live memory (only dead stack below the stack pointer
//! differs). Bound in gen/funcs.rs by tools/translate/c2rs/link.py (NATIVE).
#![allow(non_snake_case)]
use crate::rt::prelude::*;

/// vu0_b.c f_bits: memcpy(&u, &f, 4).
#[inline(always)]
pub fn vu0_b__f_bits(_c: &mut Ctx, f: F32) -> u32 {
    f.0
}

/// vu0_b.c f_from: memcpy(&f, &u, 4).
#[inline(always)]
pub fn vu0_b__f_from(_c: &mut Ctx, u: u32) -> F32 {
    F32(u)
}
