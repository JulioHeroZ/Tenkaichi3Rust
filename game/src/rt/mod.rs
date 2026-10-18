//! Runtime of the translated program: memory, PS2 float arithmetic, helpers for the generated code.
pub mod mem;
pub mod float;
pub mod ops;

pub use float::*;
pub use mem::Mem;
pub use ops::*;

/// The machine the translated C runs on.
pub struct Ctx {
    pub m: Mem,
    /// Stack pointer of the frames of address-taken locals (grows down from STACK_TOP).
    pub sp: u32,
    pub host: crate::host::HostState,
}

pub const STACK_TOP: u32 = 0x0400_0000;
pub const STACK_SIZE: u32 = 0x0100_0000;

impl Ctx {
    pub fn new() -> Box<Ctx> {
        let mut c = Box::new(Ctx { m: Mem::new(), sp: STACK_TOP, host: Default::default() });
        c.load_image();
        c
    }

    fn load_image(&mut self) {
        let img = crate::gen::data::IMAGE;
        let mut at = 0usize;
        while at + 8 <= img.len() {
            let a = u32::from_le_bytes(img[at..at + 4].try_into().unwrap());
            let n = u32::from_le_bytes(img[at + 4..at + 8].try_into().unwrap()) as usize;
            self.m.write_bytes(a, &img[at + 8..at + 8 + n]);
            at += 8 + n;
        }
    }

    #[inline(always)]
    pub fn frame_check(&self, fp: u32) {
        if fp < STACK_TOP - STACK_SIZE {
            panic!("translated program stack overflow");
        }
    }
}

pub fn null_call(_c: &mut Ctx) -> u64 {
    panic!("call through a null function pointer");
}

pub fn bad_call(_c: &mut Ctx, id: u32) -> u64 {
    panic!("call through an unknown function pointer 0x{id:08X}");
}

pub mod prelude {
    pub use super::float::F32;
    pub use super::Ctx;
    pub use crate::gen::dispatch as D;
    pub use crate::gen::fid;
    pub use crate::gen::funcs as F;
    pub use crate::gen::sym;
    pub use crate::rt;
}

/// Set when BT3_PROBE names a function whose calls are to be dumped (see host::harness::probe).
pub static PROBING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
#[inline(always)]
pub fn probing() -> bool { PROBING.load(std::sync::atomic::Ordering::Relaxed) }

/// Per-function call counter of the probes (see host::harness::probe).
pub struct ProbeCount { pub n: std::sync::atomic::AtomicU32, pub depth: std::sync::atomic::AtomicU32 }
impl ProbeCount {
    pub const fn new() -> Self { ProbeCount { n: std::sync::atomic::AtomicU32::new(0), depth: std::sync::atomic::AtomicU32::new(0) } }
}
