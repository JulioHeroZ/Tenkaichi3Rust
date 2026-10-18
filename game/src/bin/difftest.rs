//! Differential-test server: same stdin protocol as the i386 reference harness (ref32).
use bt3_original::rt::prelude::*;
use std::io::{Read, Write};

fn rd<const N: usize>(i: &mut impl Read) -> Option<[u8; N]> { let mut b = [0u8; N]; i.read_exact(&mut b).ok()?; Some(b) }
fn u(b: &[u8], k: usize) -> u32 { u32::from_le_bytes(b[k * 4..k * 4 + 4].try_into().unwrap()) }

fn main() {
    let mut c = Ctx::new();
    if let Ok(p) = std::env::var("BT3_TABLES") {
        let bytes = std::fs::read(p).expect("tables");
        bt3_original::host::files::load_original_tables(&mut c, &bytes).expect("load tables");
    }
    let stdin = std::io::stdin();
    let mut i = stdin.lock();
    let stdout = std::io::stdout();
    let mut o = stdout.lock();
    loop {
        let Some([op]) = rd::<1>(&mut i) else { return };
        match op {
            b'M' => { let h = rd::<8>(&mut i).unwrap(); o.write_all(&u(&h, 0).to_le_bytes()).unwrap(); }
            b'W' => {
                let h = rd::<8>(&mut i).unwrap();
                let mut v = vec![0u8; u(&h, 1) as usize];
                i.read_exact(&mut v).unwrap();
                c.m.write_bytes(u(&h, 0), &v);
            }
            b'D' => {
                let h = rd::<8>(&mut i).unwrap();
                let mut v = vec![0u8; u(&h, 1) as usize];
                c.m.read_bytes(u(&h, 0), &mut v);
                o.write_all(&v).unwrap();
            }
            b'C' => {
                let h = rd::<56>(&mut i).unwrap();
                let args: Vec<u64> = (0..12).map(|k| u(&h, 2 + k) as u64).collect();
                let r = bt3_original::gen::dispatch::call(&mut c, u(&h, 0), &args[..u(&h, 1) as usize]);
                o.write_all(&r.to_le_bytes()).unwrap();
            }
            b'S' => { let h = rd::<8>(&mut i).unwrap(); c.sp = u(&h, 0); }
            _ => return,
        }
        o.flush().unwrap();
    }
}
