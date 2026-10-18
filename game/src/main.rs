fn main() {
    #[cfg(target_os = "linux")]
    prof::start();
    let code = bt3_original::host::run_main();
    std::process::exit(code);
}

/// BT3_PROF=<file>: a sampling profiler for the tests (Linux only, no tools needed): SIGPROF every millisecond of
/// processor time records the interrupted instruction address; the addresses are written to <file> every second
/// (and symbolised afterwards with addr2line / nm).
#[cfg(target_os = "linux")]
mod prof {
    use std::sync::atomic::{AtomicUsize, AtomicU64, Ordering};
    const N: usize = 1 << 22;
    static SAMPLES: [AtomicU64; N] = [const { AtomicU64::new(0) }; N];
    static COUNT: AtomicUsize = AtomicUsize::new(0);
    static MAIN: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
    static OTHER: AtomicUsize = AtomicUsize::new(0);

    #[repr(C)]
    struct SigAction {
        handler: usize,
        mask: [u64; 16],
        flags: i32,
        pad: i32,
        restorer: usize,
    }
    #[repr(C)]
    struct ItimerVal {
        interval: [i64; 2],
        value: [i64; 2],
    }
    extern "C" {
        fn sigaction(sig: i32, act: *const SigAction, old: *mut SigAction) -> i32;
        fn setitimer(which: i32, new: *const ItimerVal, old: *mut ItimerVal) -> i32;
        fn gettid() -> i32;
        fn pthread_sigmask(how: i32, set: *const [u64; 16], old: *mut [u64; 16]) -> i32;
    }

    extern "C" fn on_prof(_sig: i32, _info: *mut u8, uc: *mut u8) {
        // ucontext_t: uc_mcontext at 40, gregs[REG_RIP = 16]
        if unsafe { gettid() } != MAIN.load(Ordering::Relaxed) {
            OTHER.fetch_add(1, Ordering::Relaxed);
            return;
        }
        // ucontext_t: uc_mcontext at 40; gregs[REG_RBP = 10], [REG_RSP = 15], [REG_RIP = 16]. With frame pointers
        // (RUSTFLAGS=-Cforce-frame-pointers=yes) the callers are found along the RBP chain: 8 slots a sample.
        let g = |r: usize| unsafe { *(uc.add(40 + r * 8) as *const u64) };
        let (mut bp, sp) = (g(10), g(15));
        let i = COUNT.fetch_add(8, Ordering::Relaxed);
        if i + 8 > N {
            return;
        }
        SAMPLES[i].store(g(16), Ordering::Relaxed);
        for k in 1..8 {
            if bp < sp || bp > sp + (64 << 20) || bp & 7 != 0 {
                break;
            }
            let ret = unsafe { *((bp + 8) as *const u64) };
            SAMPLES[i + k].store(ret, Ordering::Relaxed);
            let next = unsafe { *(bp as *const u64) };
            if next <= bp {
                break;
            }
            bp = next;
        }
    }

    pub fn start() {
        let Ok(path) = std::env::var("BT3_PROF") else { return };
        MAIN.store(unsafe { gettid() }, Ordering::Relaxed);
        let _ = std::fs::copy("/proc/self/maps", format!("{path}.maps"));
        // the writer thread must not take the samples: block SIGPROF in it
        std::thread::spawn(move || {
            let mut set = [0u64; 16];
            set[0] = 1 << (27 - 1);
            unsafe { pthread_sigmask(0, &set, std::ptr::null_mut()) };
            let mut written = 0usize;
            loop {
                std::thread::sleep(std::time::Duration::from_secs(1));
                let n = COUNT.load(Ordering::Relaxed).min(N);
                if n > written {
                    use std::io::Write;
                    let mut f = std::fs::OpenOptions::new().create(true).append(true).open(&path).unwrap();
                    let mut s = String::new();
                    for i in (written..n).step_by(8) {
                        let v: Vec<String> = (i..i + 8).map(|k| format!("{:x}", SAMPLES[k].swap(0, Ordering::Relaxed))).collect();
                        s += &v.join(" ");
                        s += "\n";
                    }
                    let _ = f.write_all(s.as_bytes());
                    written = n;
                }
            }
        });
        std::thread::sleep(std::time::Duration::from_millis(10));
        let act = SigAction { handler: on_prof as usize, mask: [0; 16], flags: 0x4 | 0x1000_0000 /* SA_SIGINFO | SA_RESTART */, pad: 0, restorer: 0 };
        unsafe {
            sigaction(27, &act, std::ptr::null_mut());
            let t = ItimerVal { interval: [0, 1000], value: [0, 1000] };
            setitimer(2 /* ITIMER_PROF */, &t, std::ptr::null_mut());
        }
    }
}
