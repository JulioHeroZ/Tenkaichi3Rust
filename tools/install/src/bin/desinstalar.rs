//! Desinstalar.exe, in the game's folder (also what "Apps & features" runs): removes the game's folder, its
//! shortcuts and its entry in the list of installed programs (written by tools/install/instalar.py), and, if asked,
//! the saves and settings in Documents\My Games\Tenkaichi3Rust. A program cannot delete itself while it runs, so it
//! copies itself to %TEMP% and that copy does the work. Plain Win32, no libraries.
#![cfg_attr(windows, windows_subsystem = "windows")]
#![allow(non_snake_case)]

use std::path::{Path, PathBuf};
use std::process::Command;

type HWND = isize;

#[cfg_attr(windows, link(name = "user32"))]
extern "system" {
    fn MessageBoxW(h: HWND, text: *const u16, caption: *const u16, kind: u32) -> i32;
}
#[cfg_attr(windows, link(name = "shell32"))]
extern "system" {
    fn ShellExecuteW(h: HWND, op: *const u16, file: *const u16, params: *const u16, dir: *const u16, show: i32) -> isize;
    fn IsUserAnAdmin() -> i32;
    fn SHGetFolderPathW(h: HWND, csidl: i32, token: isize, flags: u32, path: *mut u16) -> i32;
}
#[cfg_attr(windows, link(name = "kernel32"))]
extern "system" {
    fn MoveFileExW(from: *const u16, to: *const u16, flags: u32) -> i32;
}

const MB_OK: u32 = 0;
const MB_YESNO: u32 = 4;
const MB_ICONQUESTION: u32 = 0x20;
const MB_ICONWARNING: u32 = 0x30;
const MB_ICONINFO: u32 = 0x40;
const MB_DEFBUTTON2: u32 = 0x100;
const IDYES: i32 = 6;
const TITLE: &str = "Desinstalar Tenkaichi 3";
const KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Uninstall\Tenkaichi3Rust";

fn w(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}

fn ask(text: &str, kind: u32) -> i32 {
    unsafe { MessageBoxW(0, w(text).as_ptr(), w(TITLE).as_ptr(), kind) }
}

/// A shell folder (CSIDL): desktop 0x10, Start menu programs 0x02, documents 0x05.
fn folder(csidl: i32) -> Option<PathBuf> {
    let mut b = vec![0u16; 1024];
    let r = unsafe { SHGetFolderPathW(0, csidl, 0, 0, b.as_mut_ptr()) };
    let n = b.iter().position(|&c| c == 0).unwrap_or(0);
    (r == 0 && n > 0).then(|| PathBuf::from(String::from_utf16_lossy(&b[..n])))
}

fn hidden(c: &mut Command) -> &mut Command {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        c.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }
    c
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let flag = |name: &str| args.iter().position(|a| a == name);
    let exe = std::env::current_exe().unwrap_or_default();
    // the administrator, for Program Files and the list of programs of all users
    if unsafe { IsUserAnAdmin() } == 0 && flag("--elevado").is_none() {
        let mut params = args[1..].iter().map(|a| format!("\"{a}\"")).collect::<Vec<_>>();
        params.push("--elevado".into());
        let r = unsafe {
            ShellExecuteW(0, w("runas").as_ptr(), w(&exe.display().to_string()).as_ptr(), w(&params.join(" ")).as_ptr(),
                          std::ptr::null(), 1)
        };
        if r > 32 {
            return;
        }
    }
    match flag("--pasta").and_then(|i| args.get(i + 1)) {
        None => start(&exe, flag("--elevado").is_some()),
        Some(dir) => remove(Path::new(dir), flag("--saves").is_some(), &exe),
    }
}

/// Run from the game's folder: confirm, then hand over to a copy outside it.
fn start(exe: &Path, elevated: bool) {
    let Some(dir) = exe.parent().map(Path::to_path_buf) else { return };
    if !dir.join("Tenkaichi3.exe").is_file() {
        ask("Este programa fica na pasta do jogo: não achei o Tenkaichi3.exe ao lado dele.", MB_OK | MB_ICONWARNING);
        return;
    }
    if ask(&format!("Desinstalar o Tenkaichi 3 de\n{}?\n\nA pasta do jogo, os atalhos e a entrada na lista de programas serão apagados.",
                    dir.display()), MB_YESNO | MB_ICONQUESTION) != IDYES
    {
        return;
    }
    let saves = ask("Apagar também os saves e as configurações (Documentos\\My Games\\Tenkaichi3Rust)?\n\n\
                     Escolha Não para guardá-los para uma próxima instalação.", MB_YESNO | MB_ICONQUESTION | MB_DEFBUTTON2) == IDYES;
    let copy = std::env::temp_dir().join("Tenkaichi3-Desinstalar.exe");
    if std::fs::copy(exe, &copy).is_err() {
        ask("Não consegui preparar a desinstalação (pasta temporária).", MB_OK | MB_ICONWARNING);
        return;
    }
    let mut c = Command::new(&copy);
    c.arg("--pasta").arg(&dir);
    if saves {
        c.arg("--saves");
    }
    if elevated {
        c.arg("--elevado");
    }
    let _ = c.current_dir(std::env::temp_dir()).spawn();
}

/// Run from %TEMP%: remove everything, then this copy at the next restart.
fn remove(dir: &Path, saves: bool, exe: &Path) {
    // the game's folder: only one that looks like it (never a drive or a folder of other things)
    if !dir.join("Tenkaichi3.exe").is_file() || dir.parent().is_none() {
        ask(&format!("{} não parece a pasta do Tenkaichi 3; nada foi apagado.", dir.display()), MB_OK | MB_ICONWARNING);
        return;
    }
    let mut left = None;
    for attempt in 0..20 {
        // the copy that started this may still be closing; the game may be open
        std::thread::sleep(std::time::Duration::from_millis(if attempt == 0 { 500 } else { 1000 }));
        match std::fs::remove_dir_all(dir) {
            Ok(()) => {
                left = None;
                break;
            }
            Err(e) => left = Some(e),
        }
        if attempt == 5 {
            ask("Feche o jogo, se estiver aberto, e clique em OK.", MB_OK | MB_ICONWARNING);
        }
    }
    for csidl in [0x10, 0x02, 0x19, 0x17] {
        // the user's and all users' desktop and Start menu programs
        if let Some(f) = folder(csidl) {
            let _ = std::fs::remove_file(f.join("Tenkaichi 3.lnk"));
        }
    }
    for root in ["HKLM", "HKCU"] {
        let _ = hidden(&mut Command::new("reg")).args(["delete", &format!("{root}\\{KEY}"), "/f"]).status();
    }
    if saves {
        if let Some(docs) = folder(0x05) {
            let _ = std::fs::remove_dir_all(docs.join("My Games").join("Tenkaichi3Rust"));
        }
    }
    unsafe { MoveFileExW(w(&exe.display().to_string()).as_ptr(), std::ptr::null(), 4) }; // MOVEFILE_DELAY_UNTIL_REBOOT
    match left {
        None => ask("O Tenkaichi 3 foi desinstalado.", MB_OK | MB_ICONINFO),
        Some(e) => ask(&format!("Quase tudo foi desinstalado, mas parte da pasta do jogo ficou:\n{}\n({e})\n\nApague-a pelo Explorador.",
                                dir.display()), MB_OK | MB_ICONWARNING),
    };
}
