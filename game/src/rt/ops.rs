//! Integer division, bit fields and block memory operations of the translated C.
use super::Ctx;

macro_rules! divs {
    ($t:ty, $d:ident, $r:ident) => {
        /// C division (truncation). Division by zero gives the MIPS result (quotient -1 or 1, remainder a).
        #[inline(always)]
        pub fn $d(a: $t, b: $t) -> $t {
            if b == 0 {
                return if a >= 0 as $t { (0 as $t).wrapping_sub(1) } else { 1 };
            }
            a.wrapping_div(b)
        }
        #[inline(always)]
        pub fn $r(a: $t, b: $t) -> $t {
            if b == 0 {
                return a;
            }
            a.wrapping_rem(b)
        }
    };
}
divs!(i32, div_i32, rem_i32);
divs!(u32, div_u32, rem_u32);
divs!(i64, div_i64, rem_i64);
divs!(u64, div_u64, rem_u64);
divs!(i128, div_i128, rem_i128);
divs!(u128, div_u128, rem_u128);
divs!(i16, div_i16, rem_i16);
divs!(u16, div_u16, rem_u16);
divs!(i8, div_i8, rem_i8);
divs!(u8, div_u8, rem_u8);

/// Bits [first, first + width) counted from the byte at `a` (little endian, GCC layout).
#[inline]
pub fn bf_get_u(c: &Ctx, a: u32, first: u32, width: u32) -> u64 {
    let bytes = (first + width + 7) / 8;
    let mut v: u128 = 0;
    for i in 0..bytes {
        v |= (c.m.r8(a.wrapping_add(i)) as u128) << (8 * i);
    }
    ((v >> first) & ((1u128 << width) - 1)) as u64
}

#[inline]
pub fn bf_get_s(c: &Ctx, a: u32, first: u32, width: u32) -> i64 {
    let v = bf_get_u(c, a, first, width);
    let shift = 64 - width;
    ((v << shift) as i64) >> shift
}

#[inline]
pub fn bf_set(c: &mut Ctx, a: u32, first: u32, width: u32, val: u64) {
    let bytes = (first + width + 7) / 8;
    let mut v: u128 = 0;
    for i in 0..bytes {
        v |= (c.m.r8(a.wrapping_add(i)) as u128) << (8 * i);
    }
    let mask = ((1u128 << width) - 1) << first;
    v = (v & !mask) | (((val as u128) << first) & mask);
    for i in 0..bytes {
        c.m.w8(a.wrapping_add(i), (v >> (8 * i)) as u8);
    }
}

#[inline]
pub fn memcpy(c: &mut Ctx, dst: u32, src: u32, n: u32) {
    c.m.copy(dst, src, n);
}

#[inline]
pub fn memset(c: &mut Ctx, dst: u32, v: u8, n: u32) {
    c.m.fill(dst, v, n);
}
