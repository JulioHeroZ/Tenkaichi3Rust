//! The PS2's single-precision arithmetic, bit for bit what port/src/softfloat_ps2.c computes with its default
//! model (PORT_FLOAT_PCSX2, no BT3_*_NEAREST switches): no NaN, no infinity, no denormals, add / subtract /
//! multiply truncated toward zero, FPU division rounded to nearest, overflow and x/0 giving +-FLT_MAX.
#[derive(Clone, Copy, Debug, Default)]
#[repr(transparent)]
pub struct F32(pub u32);

const SIGN: u32 = 0x8000_0000;
const FMAX: u32 = 0x7F7F_FFFF;

#[inline(always)]
fn exp(x: u32) -> u32 { (x >> 23) & 0xFF }
#[inline(always)]
fn mant(x: u32) -> u32 { (x & 0x7F_FFFF) | 0x80_0000 }
#[inline(always)]
fn inp(a: u32) -> u32 { if exp(a) == 255 { (a & SIGN) | FMAX } else { a } }

#[inline(always)]
fn pack(sign: u32, e: i32, m: u32) -> u32 {
    if e >= 255 {
        return sign | FMAX;
    }
    if e <= 0 {
        return sign;
    }
    sign | ((e as u32) << 23) | (m & 0x7F_FFFF)
}

#[inline(always)]
fn round_pack(sign: u32, mut e: i32, m: u64, sticky: bool, nearest: bool) -> u32 {
    let mut hi = (m >> 32) as u32;
    let lo = m as u32;
    if nearest && (lo & 0x8000_0000) != 0 && ((lo & 0x7FFF_FFFF) != 0 || sticky || (hi & 1) != 0) {
        hi += 1;
        if hi == 0x100_0000 {
            hi >>= 1;
            e += 1;
        }
    }
    pack(sign, e, hi)
}

/// Exact sum truncated toward zero (nearest = false) or rounded to nearest even.
pub fn add_core(a: u32, b: u32, nearest: bool) -> u32 {
    let (mut a, mut b) = (inp(a), inp(b));
    if exp(a) == 0 && exp(b) == 0 {
        return a & b & SIGN;
    }
    if exp(b) == 0 {
        return a;
    }
    if exp(a) == 0 {
        return b;
    }
    if (a & 0x7FFF_FFFF) < (b & 0x7FFF_FFFF) {
        std::mem::swap(&mut a, &mut b);
    }
    let sign = a & SIGN;
    let mut e = exp(a) as i32;
    let d = e - exp(b) as i32;
    let mut ma = (mant(a) as u64) << 32;
    let mut mb = (mant(b) as u64) << 32;
    let mut sticky;
    if (a ^ b) & SIGN != 0 {
        let lost = (d > 32) as u64;
        sticky = lost != 0;
        mb = if d >= 64 { 0 } else { mb >> d };
        ma = ma.wrapping_sub(mb + lost);
        if ma == 0 {
            return 0;
        }
        while ma & (1u64 << 55) == 0 {
            ma <<= 1;
            e -= 1;
        }
    } else {
        sticky = d > 32;
        ma += if d >= 64 { 0 } else { mb >> d };
        if ma & (1u64 << 56) != 0 {
            sticky |= ma & 1 != 0;
            ma >>= 1;
            e += 1;
        }
    }
    round_pack(sign, e, ma, sticky, nearest)
}

pub fn mul_core(a: u32, b: u32, nearest: bool) -> u32 {
    let sign = (a ^ b) & SIGN;
    let (a, b) = (inp(a), inp(b));
    if exp(a) == 0 || exp(b) == 0 {
        return sign;
    }
    let mut m = mant(a) as u64 * mant(b) as u64;
    let mut e = exp(a) as i32 + exp(b) as i32 - 127;
    if m & (1u64 << 47) != 0 {
        e += 1;
        m <<= 8;
    } else {
        m <<= 9;
    }
    round_pack(sign, e, m, false, nearest)
}

/// The FPU's add.s: one bit of the smaller operand below the larger one's last place, then a truncating add.
pub fn fpu_add(mut a: u32, mut b: u32) -> u32 {
    let d = exp(a) as i32 - exp(b) as i32;
    if d >= 25 {
        b &= SIGN;
    } else if d > 0 {
        b &= 0xFFFF_FFFFu32 << (d - 1);
    } else if d <= -25 {
        a &= SIGN;
    } else if d < 0 {
        a &= 0xFFFF_FFFFu32 << (-d - 1);
    }
    add_core(a, b, false)
}

pub fn divide(a: u32, b: u32, nearest: bool) -> u32 {
    let sign = (a ^ b) & SIGN;
    let (a, b) = (inp(a), inp(b));
    if exp(b) == 0 {
        return sign | FMAX;
    }
    if exp(a) == 0 {
        return sign;
    }
    let n = (mant(a) as u64) << 26;
    let mut q = n / mant(b) as u64;
    let mut rem = n % mant(b) as u64;
    let mut e = exp(a) as i32 - exp(b) as i32 + 127;
    if q >= 1u64 << 26 {
        rem |= q & 1;
        q >>= 1;
    } else {
        e -= 1;
    }
    rem |= q & 1;
    q >>= 1;
    let mut m = (q >> 1) as u32;
    if nearest && (q & 1) != 0 && (rem != 0 || (m & 1) != 0) {
        m += 1;
        if m == 0x100_0000 {
            m >>= 1;
            e += 1;
        }
    }
    pack(sign, e, m)
}

/// The vector unit's add (RefVu0_AddBits under REF_VU0_EXTERN_ARITH).
#[inline(always)]
pub fn vu_add(a: u32, b: u32) -> u32 { add_core(a, b, false) }
#[inline(always)]
pub fn vu_sub(a: u32, b: u32) -> u32 { add_core(a, b ^ SIGN, false) }
#[inline(always)]
pub fn vu_mul(a: u32, b: u32) -> u32 { mul_core(a, b, false) }
#[inline(always)]
pub fn vu_div(a: u32, b: u32) -> u32 { divide(a, b, false) }

/// a < b on sign-and-magnitude patterns (+0 == -0).
#[inline(always)]
pub fn lt_bits(a: u32, b: u32) -> bool {
    let x = if a & SIGN != 0 { -((a & 0x7FFF_FFFF) as i32) } else { (a & 0x7FFF_FFFF) as i32 };
    let y = if b & SIGN != 0 { -((b & 0x7FFF_FFFF) as i32) } else { (b & 0x7FFF_FFFF) as i32 };
    x < y
}

/// (float)v / 2^frac, truncated (vitof / cvt.s.w).
pub fn itof_bits(v: i32, frac: i32) -> u32 {
    let sign = if v < 0 { SIGN } else { 0 };
    let mut u = if v < 0 { 0u32.wrapping_sub(v as u32) } else { v as u32 };
    let mut e = 31i32;
    if u == 0 {
        return 0;
    }
    while u & 0x8000_0000 == 0 {
        u <<= 1;
        e -= 1;
    }
    sign | (((e + 127 - frac) as u32) << 23) | ((u >> 8) & 0x7F_FFFF)
}

// ---- the C operators of the software-float game code (the __*sf* helpers of softfloat_ps2.c)
#[inline(always)] pub fn fadd(a: F32, b: F32) -> F32 { F32(fpu_add(a.0, b.0)) }
#[inline(always)] pub fn fsub(a: F32, b: F32) -> F32 { F32(fpu_add(a.0, b.0 ^ SIGN)) }
#[inline(always)] pub fn fmul(a: F32, b: F32) -> F32 { F32(mul_core(a.0, b.0, false)) }
#[inline(always)] pub fn fdiv(a: F32, b: F32) -> F32 { F32(divide(a.0, b.0, true)) }
#[inline(always)] pub fn fneg(a: F32) -> F32 { F32(a.0 ^ SIGN) }
#[inline(always)] pub fn ffabs(a: F32) -> F32 { F32(a.0 & !SIGN) }
#[inline(always)] pub fn flt(a: F32, b: F32) -> bool { lt_bits(a.0, b.0) }
#[inline(always)] pub fn fgt(a: F32, b: F32) -> bool { lt_bits(b.0, a.0) }
#[inline(always)] pub fn fle(a: F32, b: F32) -> bool { !lt_bits(b.0, a.0) }
#[inline(always)] pub fn fge(a: F32, b: F32) -> bool { !lt_bits(a.0, b.0) }
#[inline(always)] pub fn feq(a: F32, b: F32) -> bool { !lt_bits(a.0, b.0) && !lt_bits(b.0, a.0) }
#[inline(always)] pub fn fne(a: F32, b: F32) -> bool { !feq(a, b) }
/// x != 0.0 as the compiler tests it (a compare with +0).
#[inline(always)] pub fn ftrue(a: F32) -> bool { fne(a, F32(0)) }

/// cvt.w.s: truncation; out of range gives the largest / smallest integer.
pub fn f2i(a: F32) -> i32 {
    let a = a.0;
    let e = ((a >> 23) & 0xFF) as i32 - 127;
    let m = (a & 0x7F_FFFF) | 0x80_0000;
    if e < 0 {
        return 0;
    }
    if e >= 31 {
        return if a & SIGN != 0 { i32::MIN } else { i32::MAX };
    }
    let v = if e >= 23 { m << (e - 23) } else { m >> (23 - e) };
    if a & SIGN != 0 { (v as i32).wrapping_neg() } else { v as i32 }
}

pub fn f2u(a: F32) -> u32 {
    if !lt_bits(a.0, 0x4F00_0000) {
        return (f2i(F32(vu_sub(a.0, 0x4F00_0000))) as u32) | SIGN;
    }
    f2i(a) as u32
}

#[inline(always)] pub fn i2f(v: i32) -> F32 { F32(itof_bits(v, 0)) }
pub fn u2f(v: u32) -> F32 {
    let f = itof_bits(v as i32, 0);
    F32(if v & SIGN != 0 { vu_add(f, 0x4F80_0000) } else { f })
}
// 64-bit conversions and doubles: IEEE (compiler-rt / the host), as in the port.
#[inline(always)] pub fn f2d(a: F32) -> f64 { f32::from_bits(a.0) as f64 }
#[inline(always)] pub fn d2f(a: f64) -> F32 { F32((a as f32).to_bits()) }
#[inline(always)] pub fn f2l(a: F32) -> i64 { f32::from_bits(a.0) as i64 }
#[inline(always)] pub fn f2ul(a: F32) -> u64 { f32::from_bits(a.0) as u64 }
#[inline(always)] pub fn l2f(v: i64) -> F32 { F32((v as f32).to_bits()) }
#[inline(always)] pub fn ul2f(v: u64) -> F32 { F32((v as f32).to_bits()) }
/// (int)double as x86 cvttsd2si: out of range and NaN give 0x80000000.
#[inline(always)] pub fn d2i(a: f64) -> i32 { if a.is_nan() || a >= 2147483648.0 || a <= -2147483649.0 { i32::MIN } else { a as i32 } }
#[inline(always)] pub fn d2u(a: f64) -> u32 { if a.is_nan() || a >= 18446744073709551616.0 || a <= -9223372036854775809.0 { 0 } else { (a as i64) as u32 } }
#[inline(always)] pub fn d2l(a: f64) -> i64 { a as i64 }
#[inline(always)] pub fn d2ul(a: f64) -> u64 { a as u64 }

#[cfg(test)]
mod tests {
    use super::*;
    fn f(x: f32) -> F32 { F32(x.to_bits()) }
    #[test]
    fn known_patterns() {
        // 1/30 rounded to nearest by the FPU division (the save-state evidence of softfloat_ps2.c)
        assert_eq!(fdiv(f(1.0), f(30.0)).0, (1.0f32 / 30.0).to_bits());
        assert_eq!(fadd(f(1.0), f(2.0)).0, 3.0f32.to_bits());
        assert_eq!(fmul(f(1.5), f(2.0)).0, 3.0f32.to_bits());
        assert_eq!(f2i(f(-3.7)), -3);
        assert_eq!(f2i(f(3e10)), i32::MAX);
        assert_eq!(i2f(16777217).0, 16777216.0f32.to_bits()); // truncation
        assert_eq!(fdiv(f(1.0), f(0.0)).0, FMAX);
        assert!(feq(F32(0), F32(SIGN)));
        assert_eq!(f2u(f(3e9)), 3_000_000_000);
    }
}
