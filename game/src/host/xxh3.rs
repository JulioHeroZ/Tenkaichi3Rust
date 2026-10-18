//! XXH3, 64 bits, seed 0 and the default secret (xxHash 0.8), the hash PCSX2 names its texture replacements by
//! (host/texpack.rs). Written from the reference implementation (xxhash.h, the scalar code); checked against it.

const SECRET: [u8; 192] = [
    0xb8, 0xfe, 0x6c, 0x39, 0x23, 0xa4, 0x4b, 0xbe, 0x7c, 0x01, 0x81, 0x2c, 0xf7, 0x21, 0xad, 0x1c,
    0xde, 0xd4, 0x6d, 0xe9, 0x83, 0x90, 0x97, 0xdb, 0x72, 0x40, 0xa4, 0xa4, 0xb7, 0xb3, 0x67, 0x1f,
    0xcb, 0x79, 0xe6, 0x4e, 0xcc, 0xc0, 0xe5, 0x78, 0x82, 0x5a, 0xd0, 0x7d, 0xcc, 0xff, 0x72, 0x21,
    0xb8, 0x08, 0x46, 0x74, 0xf7, 0x43, 0x24, 0x8e, 0xe0, 0x35, 0x90, 0xe6, 0x81, 0x3a, 0x26, 0x4c,
    0x3c, 0x28, 0x52, 0xbb, 0x91, 0xc3, 0x00, 0xcb, 0x88, 0xd0, 0x65, 0x8b, 0x1b, 0x53, 0x2e, 0xa3,
    0x71, 0x64, 0x48, 0x97, 0xa2, 0x0d, 0xf9, 0x4e, 0x38, 0x19, 0xef, 0x46, 0xa9, 0xde, 0xac, 0xd8,
    0xa8, 0xfa, 0x76, 0x3f, 0xe3, 0x9c, 0x34, 0x3f, 0xf9, 0xdc, 0xbb, 0xc7, 0xc7, 0x0b, 0x4f, 0x1d,
    0x8a, 0x51, 0xe0, 0x4b, 0xcd, 0xb4, 0x59, 0x31, 0xc8, 0x9f, 0x7e, 0xc9, 0xd9, 0x78, 0x73, 0x64,
    0xea, 0xc5, 0xac, 0x83, 0x34, 0xd3, 0xeb, 0xc3, 0xc5, 0x81, 0xa0, 0xff, 0xfa, 0x13, 0x63, 0xeb,
    0x17, 0x0d, 0xdd, 0x51, 0xb7, 0xf0, 0xda, 0x49, 0xd3, 0x16, 0x55, 0x26, 0x29, 0xd4, 0x68, 0x9e,
    0x2b, 0x16, 0xbe, 0x58, 0x7d, 0x47, 0xa1, 0xfc, 0x8f, 0xf8, 0xb8, 0xd1, 0x7a, 0xd0, 0x31, 0xce,
    0x45, 0xcb, 0x3a, 0x8f, 0x95, 0x16, 0x04, 0x28, 0xaf, 0xd7, 0xfb, 0xca, 0xbb, 0x4b, 0x40, 0x7e,
];

const P32_1: u64 = 0x9E3779B1;
const P32_2: u64 = 0x85EBCA77;
const P32_3: u64 = 0xC2B2AE3D;
const P64_1: u64 = 0x9E3779B185EBCA87;
const P64_2: u64 = 0xC2B2AE3D27D4EB4F;
const P64_3: u64 = 0x165667B19E3779F9;
const P64_4: u64 = 0x85EBCA77C2B2AE63;
const P64_5: u64 = 0x27D4EB2F165667C5;
const MX1: u64 = 0x165667919E3779F9;
const MX2: u64 = 0x9FB21C651E98DF25;

fn r32(b: &[u8], o: usize) -> u64 {
    u32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]]) as u64
}

fn r64(b: &[u8], o: usize) -> u64 {
    let mut a = [0u8; 8];
    a.copy_from_slice(&b[o..o + 8]);
    u64::from_le_bytes(a)
}

fn fold(a: u64, b: u64) -> u64 {
    let p = (a as u128).wrapping_mul(b as u128);
    (p as u64) ^ ((p >> 64) as u64)
}

fn avalanche(mut h: u64) -> u64 {
    h ^= h >> 37;
    h = h.wrapping_mul(MX1);
    h ^ (h >> 32)
}

fn avalanche64(mut h: u64) -> u64 {
    h ^= h >> 33;
    h = h.wrapping_mul(P64_2);
    h ^= h >> 29;
    h = h.wrapping_mul(P64_3);
    h ^ (h >> 32)
}

fn rrmxmx(mut h: u64, len: u64) -> u64 {
    h ^= h.rotate_left(49) ^ h.rotate_left(24);
    h = h.wrapping_mul(MX2);
    h ^= (h >> 35).wrapping_add(len);
    h = h.wrapping_mul(MX2);
    h ^ (h >> 28)
}

fn mix16(d: &[u8], o: usize, s: usize) -> u64 {
    fold(r64(d, o) ^ r64(&SECRET, s), r64(d, o + 8) ^ r64(&SECRET, s + 8))
}

/// XXH3_64bits(data, len): seed 0, default secret.
pub fn hash64(d: &[u8]) -> u64 {
    let len = d.len();
    let s = &SECRET;
    if len == 0 {
        return avalanche64(r64(s, 56) ^ r64(s, 64));
    }
    if len <= 3 {
        let combined = ((d[0] as u64) << 16) | ((d[len >> 1] as u64) << 24) | (d[len - 1] as u64) | ((len as u64) << 8);
        return avalanche64(combined ^ (r32(s, 0) ^ r32(s, 4)));
    }
    if len <= 8 {
        let input64 = r32(d, len - 4).wrapping_add(r32(d, 0) << 32);
        return rrmxmx(input64 ^ (r64(s, 8) ^ r64(s, 16)), len as u64);
    }
    if len <= 16 {
        let lo = r64(d, 0) ^ (r64(s, 24) ^ r64(s, 32));
        let hi = r64(d, len - 8) ^ (r64(s, 40) ^ r64(s, 48));
        let acc = (len as u64).wrapping_add(lo.swap_bytes()).wrapping_add(hi).wrapping_add(fold(lo, hi));
        return avalanche(acc);
    }
    if len <= 128 {
        let mut acc = (len as u64).wrapping_mul(P64_1);
        if len > 32 {
            if len > 64 {
                if len > 96 {
                    acc = acc.wrapping_add(mix16(d, 48, 96)).wrapping_add(mix16(d, len - 64, 112));
                }
                acc = acc.wrapping_add(mix16(d, 32, 64)).wrapping_add(mix16(d, len - 48, 80));
            }
            acc = acc.wrapping_add(mix16(d, 16, 32)).wrapping_add(mix16(d, len - 32, 48));
        }
        acc = acc.wrapping_add(mix16(d, 0, 0)).wrapping_add(mix16(d, len - 16, 16));
        return avalanche(acc);
    }
    if len <= 240 {
        let mut acc = (len as u64).wrapping_mul(P64_1);
        for i in 0..8 {
            acc = acc.wrapping_add(mix16(d, 16 * i, 16 * i));
        }
        let mut acc_end = mix16(d, len - 16, 136 - 17);
        acc = avalanche(acc);
        for i in 8..len / 16 {
            acc_end = acc_end.wrapping_add(mix16(d, 16 * i, 16 * (i - 8) + 3));
        }
        return avalanche(acc.wrapping_add(acc_end));
    }
    // long: stripes of 64 bytes into 8 accumulators, scrambled after every block
    let mut acc = [P32_3, P64_1, P64_2, P64_3, P64_4, P32_2, P64_5, P32_1];
    let accumulate = |acc: &mut [u64; 8], at: usize, so: usize| {
        for lane in 0..8 {
            let v = r64(d, at + lane * 8);
            let k = v ^ r64(s, so + lane * 8);
            acc[lane ^ 1] = acc[lane ^ 1].wrapping_add(v);
            acc[lane] = (k & 0xFFFF_FFFF).wrapping_mul(k >> 32).wrapping_add(acc[lane]);
        }
    };
    let stripes_per_block = (192 - 64) / 8;
    let block_len = 64 * stripes_per_block;
    let blocks = (len - 1) / block_len;
    for n in 0..blocks {
        for st in 0..stripes_per_block {
            accumulate(&mut acc, n * block_len + st * 64, st * 8);
        }
        for lane in 0..8 {
            let mut a = acc[lane];
            a ^= a >> 47;
            a ^= r64(s, 192 - 64 + lane * 8);
            acc[lane] = a.wrapping_mul(P32_1);
        }
    }
    let stripes = ((len - 1) - block_len * blocks) / 64;
    for st in 0..stripes {
        accumulate(&mut acc, blocks * block_len + st * 64, st * 8);
    }
    accumulate(&mut acc, len - 64, 192 - 64 - 7);
    let mut r = (len as u64).wrapping_mul(P64_1);
    for i in 0..4 {
        r = r.wrapping_add(fold(acc[2 * i] ^ r64(s, 11 + 16 * i), acc[2 * i + 1] ^ r64(s, 11 + 16 * i + 8)));
    }
    avalanche(r)
}
