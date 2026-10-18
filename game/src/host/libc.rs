//! The C library functions the game calls.
use crate::rt::{Ctx, F32};

pub fn memcpy(c: &mut Ctx, d: u32, s: u32, n: u32) -> u32 { c.m.copy(d, s, n); d }
pub fn memset(c: &mut Ctx, d: u32, v: i32, n: u32) -> u32 { c.m.fill(d, v as u8, n); d }
pub fn memcmp(c: &mut Ctx, a: u32, b: u32, n: u32) -> i32 {
    for i in 0..n {
        let (x, y) = (c.m.r8(a.wrapping_add(i)), c.m.r8(b.wrapping_add(i)));
        if x != y {
            return x as i32 - y as i32;
        }
    }
    0
}
pub fn strlen(c: &mut Ctx, s: u32) -> u32 { c.m.cstr(s).len() as u32 }
pub fn strcpy(c: &mut Ctx, d: u32, s: u32) -> u32 {
    let mut v = c.m.cstr(s);
    v.push(0);
    c.m.write_bytes(d, &v);
    d
}
pub fn strcmp(c: &mut Ctx, a: u32, b: u32) -> i32 {
    let (x, y) = (c.m.cstr(a), c.m.cstr(b));
    for i in 0.. {
        let p = *x.get(i).unwrap_or(&0) as i32;
        let q = *y.get(i).unwrap_or(&0) as i32;
        if p != q || p == 0 {
            return p - q;
        }
    }
    0
}
pub fn abs(_c: &mut Ctx, v: i32) -> i32 { v.wrapping_abs() }
fn parse_long(s: &[u8], base: i32) -> (i64, usize) {
    let mut i = 0;
    while i < s.len() && (s[i] == b' ' || s[i] == b'\t' || s[i] == b'\n') {
        i += 1;
    }
    let mut neg = false;
    if i < s.len() && (s[i] == b'-' || s[i] == b'+') {
        neg = s[i] == b'-';
        i += 1;
    }
    let mut base = base as u32;
    if (base == 0 || base == 16) && i + 1 < s.len() && s[i] == b'0' && (s[i + 1] | 0x20) == b'x' {
        base = 16;
        i += 2;
    } else if base == 0 && i < s.len() && s[i] == b'0' {
        base = 8;
    } else if base == 0 {
        base = 10;
    }
    let start = i;
    let mut v: i64 = 0;
    while i < s.len() {
        let d = match s[i] {
            b'0'..=b'9' => (s[i] - b'0') as u32,
            b'a'..=b'z' => (s[i] - b'a' + 10) as u32,
            b'A'..=b'Z' => (s[i] - b'A' + 10) as u32,
            _ => 99,
        };
        if d >= base {
            break;
        }
        v = v.wrapping_mul(base as i64).wrapping_add(d as i64);
        i += 1;
    }
    if i == start {
        return (0, 0);
    }
    (if neg { -v } else { v }, i)
}
pub fn atoi(c: &mut Ctx, s: u32) -> i32 { parse_long(&c.m.cstr(s), 10).0 as i32 }
pub fn strtol(c: &mut Ctx, s: u32, end: u32, base: i32) -> i32 {
    let bytes = c.m.cstr(s);
    let (v, n) = parse_long(&bytes, base);
    if end != 0 {
        c.m.w32(end, s.wrapping_add(n as u32));
    }
    v.clamp(i32::MIN as i64, i32::MAX as i64) as i32
}
pub fn pow(_c: &mut Ctx, a: f64, b: f64) -> f64 { a.powf(b) }

/// The game's environment variables: none are given to it (the switches of the port are host-side).
pub fn getenv(c: &mut Ctx, name: u32) -> u32 {
    let n = String::from_utf8_lossy(&c.m.cstr(name)).to_string();
    match std::env::var(&n) {
        Ok(v) if n.starts_with("BT3_") => super::low_string(c, v.as_bytes()),
        _ => 0,
    }
}

pub fn fopen(_c: &mut Ctx, _path: u32, _mode: u32) -> u32 { 0 }
pub fn fread(_c: &mut Ctx, _b: u32, _s: u32, _n: u32, _f: u32) -> u32 { 0 }
pub fn fclose(_c: &mut Ctx, _f: u32) -> i32 { 0 }
pub fn fseek(_c: &mut Ctx, _f: u32, _o: i32, _w: i32) -> i32 { -1 }

/// printf-style formatting over the game's memory; `args` are the call's argument slots (doubles as bits).
pub fn format(c: &Ctx, fmt: &[u8], args: &[u64]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut ai = 0usize;
    let mut next = || {
        let v = args.get(ai).copied().unwrap_or(0);
        ai += 1;
        v
    };
    let mut i = 0;
    while i < fmt.len() {
        let ch = fmt[i];
        if ch != b'%' {
            out.push(ch);
            i += 1;
            continue;
        }
        i += 1;
        let (mut left, mut zero, mut plus, mut space, mut alt) = (false, false, false, false, false);
        while i < fmt.len() {
            match fmt[i] {
                b'-' => left = true,
                b'0' => zero = true,
                b'+' => plus = true,
                b' ' => space = true,
                b'#' => alt = true,
                _ => break,
            }
            i += 1;
        }
        let mut width = 0usize;
        if i < fmt.len() && fmt[i] == b'*' {
            width = next() as i32 as usize;
            i += 1;
        }
        while i < fmt.len() && fmt[i].is_ascii_digit() {
            width = width * 10 + (fmt[i] - b'0') as usize;
            i += 1;
        }
        let mut prec: Option<usize> = None;
        if i < fmt.len() && fmt[i] == b'.' {
            i += 1;
            let mut p = 0;
            if i < fmt.len() && fmt[i] == b'*' {
                p = next() as i32 as usize;
                i += 1;
            }
            while i < fmt.len() && fmt[i].is_ascii_digit() {
                p = p * 10 + (fmt[i] - b'0') as usize;
                i += 1;
            }
            prec = Some(p);
        }
        let mut long = 0;
        while i < fmt.len() && matches!(fmt[i], b'l' | b'h' | b'z' | b'j' | b't') {
            if fmt[i] == b'l' {
                long += 1;
            }
            i += 1;
        }
        if i >= fmt.len() {
            break;
        }
        let conv = fmt[i];
        i += 1;
        let mut body: Vec<u8> = match conv {
            b'd' | b'i' => {
                let v = if long >= 2 { next() as i64 } else { next() as u32 as i32 as i64 };
                let mut s = v.abs().to_string();
                if let Some(p) = prec {
                    while s.len() < p {
                        s.insert(0, '0');
                    }
                }
                if v < 0 {
                    s.insert(0, '-');
                } else if plus {
                    s.insert(0, '+');
                } else if space {
                    s.insert(0, ' ');
                }
                s.into_bytes()
            }
            b'u' | b'x' | b'X' | b'o' => {
                let v = if long >= 2 { next() } else { next() as u32 as u64 };
                let mut s = match conv {
                    b'u' => v.to_string(),
                    b'x' => format!("{v:x}"),
                    b'X' => format!("{v:X}"),
                    _ => format!("{v:o}"),
                };
                if let Some(p) = prec {
                    while s.len() < p {
                        s.insert(0, '0');
                    }
                }
                if alt && v != 0 && conv != b'u' {
                    s.insert_str(0, if conv == b'o' { "0" } else if conv == b'x' { "0x" } else { "0X" });
                }
                s.into_bytes()
            }
            b'c' => vec![next() as u8],
            b's' => {
                let a = next() as u32;
                let mut s = if a == 0 { b"(null)".to_vec() } else { c.m.cstr(a) };
                if let Some(p) = prec {
                    s.truncate(p);
                }
                s
            }
            b'p' => format!("0x{:x}", next() as u32).into_bytes(),
            b'f' | b'F' | b'e' | b'g' | b'G' | b'E' => {
                let v = f64::from_bits(next());
                let p = prec.unwrap_or(6);
                let s = match conv {
                    b'e' | b'E' => format!("{v:.p$e}"),
                    b'g' | b'G' => {
                        let s = format!("{v}");
                        s
                    }
                    _ => format!("{v:.p$}"),
                };
                let s = if plus && v >= 0.0 { format!("+{s}") } else { s };
                s.into_bytes()
            }
            b'%' => vec![b'%'],
            b'n' => {
                next();
                vec![]
            }
            other => vec![b'%', other],
        };
        if body.len() < width {
            let pad = width - body.len();
            if left {
                body.extend(std::iter::repeat(b' ').take(pad));
            } else if zero && !matches!(conv, b's' | b'c') {
                let sign = matches!(body.first(), Some(b'-') | Some(b'+') | Some(b' '));
                let at = if sign { 1 } else { 0 };
                for _ in 0..pad {
                    body.insert(at, b'0');
                }
            } else {
                let mut b2 = vec![b' '; pad];
                b2.extend(body);
                body = b2;
            }
        }
        out.extend(body);
    }
    out
}

pub fn sprintf(c: &mut Ctx, buf: u32, fmt: u32, args: &[u64]) -> i32 {
    let f = c.m.cstr(fmt);
    let mut s = format(c, &f, args);
    let n = s.len() as i32;
    s.push(0);
    c.m.write_bytes(buf, &s);
    n
}

pub fn fprintf(c: &mut Ctx, _fp: u32, fmt: u32, args: &[u64]) -> i32 {
    let f = c.m.cstr(fmt);
    let s = format(c, &f, args);
    eprint!("{}", String::from_utf8_lossy(&s));
    s.len() as i32
}

pub fn printf(c: &mut Ctx, fmt: u32, args: &[u64]) -> i32 {
    let f = c.m.cstr(fmt);
    let s = format(c, &f, args);
    print!("{}", String::from_utf8_lossy(&s));
    s.len() as i32
}

pub fn ffloat(v: F32) -> f32 { f32::from_bits(v.0) }
