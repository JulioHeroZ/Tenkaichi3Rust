//! The game as installed for players (Tenkaichi3.exe): a window and no console. It finds the `assets` folder next
//! to itself (host/harness.rs run_main) and plays with the PC resources only. What the game reports goes to
//! Documents\My Games\Tenkaichi3Rust\log.txt; when it stops on an error, a message says where.
#![cfg_attr(windows, windows_subsystem = "windows")]

fn main() {
    if std::env::var_os("BT3_GS").is_none() {
        std::env::set_var("BT3_GS", "gpu");
    }
    let log = bt3_original::host::saves::documents().join("My Games").join("Tenkaichi3Rust").join("log.txt");
    #[cfg(windows)]
    win::redirect_stderr(&log);
    let code = std::panic::catch_unwind(bt3_original::host::run_main).unwrap_or(101);
    if code != 0 {
        #[cfg(windows)]
        win::message(&format!(
            "O jogo parou por causa de um erro.\n\nO que aconteceu está em:\n{}\n\nSe faltarem arquivos, instale de novo.",
            log.display()
        ));
        #[cfg(not(windows))]
        eprintln!("bt3: stopped with code {code}; see {}", log.display());
    }
    std::process::exit(code);
}

#[cfg(windows)]
mod win {
    use std::os::windows::io::AsRawHandle;

    #[link(name = "kernel32")]
    extern "system" {
        fn SetStdHandle(which: u32, handle: *mut core::ffi::c_void) -> i32;
    }
    #[link(name = "user32")]
    extern "system" {
        fn MessageBoxW(hwnd: *mut core::ffi::c_void, text: *const u16, caption: *const u16, kind: u32) -> i32;
    }

    /// stderr (eprintln!) into the log file, which stays open until the program ends.
    pub fn redirect_stderr(path: &std::path::Path) {
        if let Some(dir) = path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        if let Ok(f) = std::fs::File::create(path) {
            unsafe { SetStdHandle(-12i32 as u32, f.as_raw_handle() as *mut _) };
            std::mem::forget(f);
        }
    }

    pub fn message(text: &str) {
        let w = |s: &str| s.encode_utf16().chain(Some(0)).collect::<Vec<u16>>();
        unsafe { MessageBoxW(std::ptr::null_mut(), w(text).as_ptr(), w("Tenkaichi 3").as_ptr(), 0x10) };
    }
}
