//! The installer's window (Instalar Tenkaichi 3.exe): the disc image, the kind of installation (play only, or the
//! developer's editable files), the folder, the optional HD texture packs (downloaded from their author's links, or
//! a .rar / folder the user has: tools/install/texturas.py), then a progress bar and what the installation reports. It runs
//! tools/install/instalar.py (with the player package's own Python, or the project's .venv) and follows its output.
//! Plain Win32 (user32, gdi32, comdlg32, shell32, comctl32), no libraries.
#![cfg_attr(windows, windows_subsystem = "windows")]
#![allow(non_snake_case, clippy::upper_case_acronyms)]

use std::collections::VecDeque;
use std::ffi::c_void;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

type HWND = isize;
type HANDLE = isize;

#[repr(C)]
struct WNDCLASSEXW {
    cbSize: u32,
    style: u32,
    lpfnWndProc: extern "system" fn(HWND, u32, usize, isize) -> isize,
    cbClsExtra: i32,
    cbWndExtra: i32,
    hInstance: HANDLE,
    hIcon: HANDLE,
    hCursor: HANDLE,
    hbrBackground: HANDLE,
    lpszMenuName: *const u16,
    lpszClassName: *const u16,
    hIconSm: HANDLE,
}
#[repr(C)]
#[derive(Default)]
struct POINT {
    x: i32,
    y: i32,
}
#[repr(C)]
#[derive(Default)]
struct RECT {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}
#[repr(C)]
#[derive(Default)]
struct MSG {
    hwnd: HWND,
    message: u32,
    wParam: usize,
    lParam: isize,
    time: u32,
    pt: POINT,
    lPrivate: u32,
}
#[repr(C)]
struct PAINTSTRUCT {
    hdc: HANDLE,
    fErase: i32,
    rcPaint: RECT,
    fRestore: i32,
    fIncUpdate: i32,
    rgbReserved: [u8; 32],
}
#[repr(C)]
struct OPENFILENAMEW {
    lStructSize: u32,
    hwndOwner: HWND,
    hInstance: HANDLE,
    lpstrFilter: *const u16,
    lpstrCustomFilter: *mut u16,
    nMaxCustFilter: u32,
    nFilterIndex: u32,
    lpstrFile: *mut u16,
    nMaxFile: u32,
    lpstrFileTitle: *mut u16,
    nMaxFileTitle: u32,
    lpstrInitialDir: *const u16,
    lpstrTitle: *const u16,
    Flags: u32,
    nFileOffset: u16,
    nFileExtension: u16,
    lpstrDefExt: *const u16,
    lCustData: isize,
    lpfnHook: *const c_void,
    lpTemplateName: *const u16,
    pvReserved: *mut c_void,
    dwReserved: u32,
    FlagsEx: u32,
}
#[repr(C)]
struct BROWSEINFOW {
    hwndOwner: HWND,
    pidlRoot: *const c_void,
    pszDisplayName: *mut u16,
    lpszTitle: *const u16,
    ulFlags: u32,
    lpfn: *const c_void,
    lParam: isize,
    iImage: i32,
}
#[repr(C)]
struct INITCOMMONCONTROLSEX {
    dwSize: u32,
    dwICC: u32,
}

#[cfg_attr(windows, link(name = "user32"))]
extern "system" {
    fn RegisterClassExW(c: *const WNDCLASSEXW) -> u16;
    fn CreateWindowExW(ex: u32, class: *const u16, title: *const u16, style: u32, x: i32, y: i32, w: i32, h: i32,
                       parent: HWND, menu: isize, inst: HANDLE, param: *const c_void) -> HWND;
    fn DefWindowProcW(h: HWND, m: u32, w: usize, l: isize) -> isize;
    fn GetMessageW(m: *mut MSG, h: HWND, min: u32, max: u32) -> i32;
    fn TranslateMessage(m: *const MSG) -> i32;
    fn DispatchMessageW(m: *const MSG) -> isize;
    fn IsDialogMessageW(h: HWND, m: *const MSG) -> i32;
    fn PostQuitMessage(code: i32);
    fn PostMessageW(h: HWND, m: u32, w: usize, l: isize) -> i32;
    fn SendMessageW(h: HWND, m: u32, w: usize, l: isize) -> isize;
    fn ShowWindow(h: HWND, cmd: i32) -> i32;
    fn UpdateWindow(h: HWND) -> i32;
    fn EnableWindow(h: HWND, on: i32) -> i32;
    fn SetWindowTextW(h: HWND, t: *const u16) -> i32;
    fn GetWindowTextW(h: HWND, t: *mut u16, n: i32) -> i32;
    fn GetWindowTextLengthW(h: HWND) -> i32;
    fn LoadCursorW(inst: HANDLE, name: usize) -> HANDLE;
    fn LoadIconW(inst: HANDLE, name: usize) -> HANDLE;
    fn MessageBoxW(h: HWND, text: *const u16, caption: *const u16, kind: u32) -> i32;
    fn BeginPaint(h: HWND, ps: *mut PAINTSTRUCT) -> HANDLE;
    fn EndPaint(h: HWND, ps: *const PAINTSTRUCT) -> i32;
    fn FillRect(dc: HANDLE, r: *const RECT, brush: HANDLE) -> i32;
    fn DrawTextW(dc: HANDLE, t: *const u16, n: i32, r: *mut RECT, fmt: u32) -> i32;
    fn GetClientRect(h: HWND, r: *mut RECT) -> i32;
    fn GetSystemMetrics(i: i32) -> i32;
    fn SetProcessDPIAware() -> i32;
    fn GetDC(h: HWND) -> HANDLE;
    fn ReleaseDC(h: HWND, dc: HANDLE) -> i32;
    fn AdjustWindowRect(r: *mut RECT, style: u32, menu: i32) -> i32;
}
#[cfg_attr(windows, link(name = "gdi32"))]
extern "system" {
    fn CreateFontW(h: i32, w: i32, esc: i32, orient: i32, weight: i32, italic: u32, underline: u32, strike: u32,
                   charset: u32, out: u32, clip: u32, quality: u32, pitch: u32, face: *const u16) -> HANDLE;
    fn CreateSolidBrush(c: u32) -> HANDLE;
    fn SetTextColor(dc: HANDLE, c: u32) -> u32;
    fn SetBkMode(dc: HANDLE, mode: i32) -> i32;
    fn SetBkColor(dc: HANDLE, c: u32) -> u32;
    fn SelectObject(dc: HANDLE, o: HANDLE) -> HANDLE;
    fn GetDeviceCaps(dc: HANDLE, i: i32) -> i32;
}
#[cfg_attr(windows, link(name = "comdlg32"))]
extern "system" {
    fn GetOpenFileNameW(o: *mut OPENFILENAMEW) -> i32;
}
#[cfg_attr(windows, link(name = "shell32"))]
extern "system" {
    fn SHBrowseForFolderW(b: *const BROWSEINFOW) -> *mut c_void;
    fn SHGetPathFromIDListW(pidl: *const c_void, path: *mut u16) -> i32;
    fn ShellExecuteW(h: HWND, op: *const u16, file: *const u16, params: *const u16, dir: *const u16, show: i32) -> HANDLE;
    fn IsUserAnAdmin() -> i32;
}
#[cfg_attr(windows, link(name = "ole32"))]
extern "system" {
    fn CoInitializeEx(r: *const c_void, flags: u32) -> i32;
    fn CoTaskMemFree(p: *mut c_void);
}
#[cfg_attr(windows, link(name = "comctl32"))]
extern "system" {
    fn InitCommonControlsEx(i: *const INITCOMMONCONTROLSEX) -> i32;
}
#[cfg_attr(windows, link(name = "kernel32"))]
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> HANDLE;
}

const WS_OVERLAPPED: u32 = 0;
const WS_CAPTION: u32 = 0x00C00000;
const WS_SYSMENU: u32 = 0x00080000;
const WS_MINIMIZEBOX: u32 = 0x00020000;
const WS_CHILD: u32 = 0x40000000;
const WS_VISIBLE: u32 = 0x10000000;
const WS_TABSTOP: u32 = 0x00010000;
const WS_GROUP: u32 = 0x00020000;
const WS_VSCROLL: u32 = 0x00200000;
const WS_EX_CLIENTEDGE: u32 = 0x200;
const ES_AUTOHSCROLL: u32 = 0x80;
const ES_MULTILINE: u32 = 0x4;
const ES_READONLY: u32 = 0x800;
const ES_AUTOVSCROLL: u32 = 0x40;
const BS_PUSHBUTTON: u32 = 0;
const BS_DEFPUSHBUTTON: u32 = 1;
const BS_AUTORADIOBUTTON: u32 = 9;
const BS_AUTOCHECKBOX: u32 = 3;
const WM_CREATE: u32 = 0x1;
const WM_DESTROY: u32 = 0x2;
const WM_PAINT: u32 = 0xF;
const WM_CLOSE: u32 = 0x10;
const WM_SETFONT: u32 = 0x30;
const WM_COMMAND: u32 = 0x111;
const WM_CTLCOLOREDIT: u32 = 0x133;
const WM_CTLCOLORBTN: u32 = 0x135;
const WM_CTLCOLORSTATIC: u32 = 0x138;
const WM_APP: u32 = 0x8000;
const BM_GETCHECK: u32 = 0xF0;
const BM_SETCHECK: u32 = 0xF1;
const EM_SETSEL: u32 = 0xB1;
const EM_REPLACESEL: u32 = 0xC2;
const EM_SETLIMITTEXT: u32 = 0xC5;
const PBM_SETRANGE32: u32 = 0x406;
const PBM_SETPOS: u32 = 0x402;
const PBM_SETBARCOLOR: u32 = 0x409;
const MB_OK: u32 = 0;
const MB_YESNO: u32 = 4;
const MB_ICONWARNING: u32 = 0x30;
const MB_ICONINFO: u32 = 0x40;
const IDYES: i32 = 6;
const DT_LEFT: u32 = 0;
const DT_SINGLELINE: u32 = 0x20;
const DT_VCENTER: u32 = 0x4;
const DT_WORDBREAK: u32 = 0x10;

// controls
const ID_ISO: isize = 101;
const ID_ISO_BROWSE: isize = 102;
const ID_PLAY: isize = 103;
const ID_DEV: isize = 104;
const ID_DIR: isize = 105;
const ID_DIR_BROWSE: isize = 106;
const ID_INSTALL: isize = 107;
const ID_CLOSE: isize = 108;
const ID_START: isize = 109;
const ID_TEX: [isize; 2] = [110, 111];
const ID_TEX_PICK: [isize; 2] = [112, 113];
const ID_FRESH: isize = 114;
const EN_CHANGE: usize = 0x300;
/// the version of the data this installer makes (empacotar.py: instalar.data_version); an installation that has it
/// (assets\instalacao.txt) is updated: only the programs are replaced, no disc image needed
const DATA_VERSION: Option<&str> = option_env!("BT3_DADOS_VERSAO");
const WM_OUTPUT: u32 = WM_APP + 1;
const WM_FINISHED: u32 = WM_APP + 2;

// colours (0x00BBGGRR)
const NAVY: u32 = 0x004A2A1D;
const ORANGE: u32 = 0x00218CF5;
const WHITE: u32 = 0x00FFFFFF;
const PAPER: u32 = 0x00F7F5F3;

fn w(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}

/// The window's controls and drawing things, set once when it is created (read without locks: Win32 calls made
/// while a lock is held could re-enter the window procedure).
#[derive(Clone, Copy, Default)]
struct H {
    iso: HWND,
    iso_browse: HWND,
    play: HWND,
    dev: HWND,
    dir: HWND,
    dir_browse: HWND,
    status: HWND,
    progress: HWND,
    log: HWND,
    install: HWND,
    close: HWND,
    start: HWND,
    /// "Refazer tudo do zero": shown when the folder can be updated
    fresh: HWND,
    /// the texture packs (2x, 4x): check box, "Já tenho..." button, where it comes from
    tex: [HWND; 2],
    tex_pick: [HWND; 2],
    tex_from: [HWND; 2],
    paper: HANDLE,
    navy: HANDLE,
    orange: HANDLE,
    white: HANDLE,
    font: HANDLE,
    title: HANDLE,
    dpi: i32,
}

/// What changes while the installation runs.
#[derive(Default)]
struct State {
    running: bool,
    stage: f64,
    done: Option<PathBuf>,
    child: Option<u32>,
    /// a texture pack the user already has (.rar or folder), instead of the download
    tex_file: [Option<String>; 2],
    /// the folder holds this installer's data: install updates it (unless "Refazer tudo do zero")
    update: bool,
}

static HANDLES: std::sync::OnceLock<H> = std::sync::OnceLock::new();
static DPI: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(96);
static MAIN: std::sync::atomic::AtomicIsize = std::sync::atomic::AtomicIsize::new(0);
static STATE: Mutex<State> = Mutex::new(State { running: false, stage: 0.0, done: None, child: None, tex_file: [None, None], update: false });
static OUTPUT: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());
static PAPER_BRUSH: std::sync::atomic::AtomicIsize = std::sync::atomic::AtomicIsize::new(0);

fn h() -> H {
    HANDLES.get().copied().unwrap_or_default()
}

fn state<R>(f: impl FnOnce(&mut State) -> R) -> R {
    f(&mut STATE.lock().unwrap())
}

static ADMIN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// The folder with tools/install/instalar.py: where this installer unpacks what it carries (Python, the tools, the
/// game's program; bt3_install::payload), or, run from the project, the first folder above the program with it.
fn root() -> PathBuf {
    if let Some(dir) = unpack_dir() {
        return dir;
    }
    let exe = std::env::current_exe().unwrap_or_default();
    let mut d = exe.parent().map(Path::to_path_buf);
    while let Some(p) = d {
        if p.join("tools").join("install").join("instalar.py").is_file() {
            return p;
        }
        d = p.parent().map(Path::to_path_buf);
    }
    std::env::current_dir().unwrap_or_default()
}

/// %TEMP%\Tenkaichi3-Instalador when this program carries its payload.
fn unpack_dir() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let mut f = std::fs::File::open(&exe).ok()?;
    use std::io::{Read, Seek, SeekFrom};
    let mut tail = [0u8; 8];
    f.seek(SeekFrom::End(-8)).ok()?;
    f.read_exact(&mut tail).ok()?;
    (&tail == b"BT3SFX1\0").then(|| std::env::temp_dir().join("Tenkaichi3-Instalador"))
}

/// Unpacks the payload (once per version of the installer); the folder.
fn unpack(log: &dyn Fn(&str)) -> Result<Option<PathBuf>, String> {
    let Some(dir) = unpack_dir() else { return Ok(None) };
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let stamp = std::fs::metadata(&exe).map(|m| format!("{} {:?}", m.len(), m.modified().ok())).unwrap_or_default();
    let mark = dir.join(".pronto");
    if std::fs::read_to_string(&mark).ok().as_deref() == Some(stamp.as_str()) {
        return Ok(Some(dir));
    }
    log("Preparando os arquivos do instalador...");
    let _ = std::fs::remove_dir_all(&dir);
    let bytes = std::fs::read(&exe).map_err(|e| e.to_string())?;
    let n = bt3_install::extract(&bytes, &dir, &mut |_, _| {})?;
    std::fs::write(&mark, stamp).map_err(|e| e.to_string())?;
    log(&format!("{n} arquivos do instalador prontos."));
    Ok(Some(dir))
}

/// Program Files (x86)\Tenkaichi 3, as a PC game; the user's own programs folder when not run as administrator.
fn default_dir() -> String {
    let admin = ADMIN.load(std::sync::atomic::Ordering::Relaxed);
    let base = if admin {
        std::env::var("ProgramFiles(x86)").or_else(|_| std::env::var("ProgramFiles")).map(PathBuf::from).ok()
    } else {
        std::env::var("LOCALAPPDATA").map(|l| PathBuf::from(l).join("Programs")).ok()
    };
    base.unwrap_or_else(|| PathBuf::from("C:\\")).join("Tenkaichi 3").display().to_string()
}

fn text(h: HWND) -> String {
    unsafe {
        let n = GetWindowTextLengthW(h);
        let mut b = vec![0u16; n as usize + 1];
        let got = GetWindowTextW(h, b.as_mut_ptr(), n + 1);
        String::from_utf16_lossy(&b[..got.max(0) as usize])
    }
}

fn set_text(h: HWND, s: &str) {
    unsafe { SetWindowTextW(h, w(s).as_ptr()) };
}

fn append_log(h: HWND, s: &str) {
    let mut line = s.replace('\n', "\r\n");
    line.push_str("\r\n");
    unsafe {
        SendMessageW(h, EM_SETSEL, usize::MAX >> 1, (usize::MAX >> 1) as isize);
        SendMessageW(h, EM_REPLACESEL, 0, w(&line).as_ptr() as isize);
    }
}

fn main() {
    unsafe {
        // an installer into Program Files asks for the administrator once, as games' installers do
        if IsUserAnAdmin() == 0 && !std::env::args().any(|a| a == "--elevado") {
            let exe = std::env::current_exe().unwrap_or_default();
            let r = ShellExecuteW(0, w("runas").as_ptr(), w(&exe.display().to_string()).as_ptr(), w("--elevado").as_ptr(),
                                  std::ptr::null(), 1);
            if r > 32 {
                return; // the administrator's copy goes on
            }
        }
        ADMIN.store(IsUserAnAdmin() != 0, std::sync::atomic::Ordering::Relaxed);
        SetProcessDPIAware();
        CoInitializeEx(std::ptr::null(), 0x2); // apartment threaded, for the folder dialog
        let icc = INITCOMMONCONTROLSEX { dwSize: 8, dwICC: 0x20 }; // ICC_PROGRESS_CLASS
        InitCommonControlsEx(&icc);
        let inst = GetModuleHandleW(std::ptr::null());
        let class = w("Tenkaichi3Instalador");
        let paper = CreateSolidBrush(PAPER);
        let wc = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: 0,
            lpfnWndProc: wndproc,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: inst,
            hIcon: LoadIconW(0, 32512),
            hCursor: LoadCursorW(0, 32512),
            hbrBackground: paper,
            lpszMenuName: std::ptr::null(),
            lpszClassName: class.as_ptr(),
            hIconSm: 0,
        };
        RegisterClassExW(&wc);
        let dc = GetDC(0);
        let dpi = GetDeviceCaps(dc, 88).max(96); // LOGPIXELSX
        ReleaseDC(0, dc);
        DPI.store(dpi, std::sync::atomic::Ordering::Relaxed);
        PAPER_BRUSH.store(paper, std::sync::atomic::Ordering::Relaxed);
        let style = WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX;
        let mut r = RECT { left: 0, top: 0, right: s(dpi, 680), bottom: s(dpi, 732) };
        AdjustWindowRect(&mut r, style, 0);
        let (ww, wh) = (r.right - r.left, r.bottom - r.top);
        let x = (GetSystemMetrics(0) - ww) / 2;
        let y = (GetSystemMetrics(1) - wh) / 2;
        let hwnd = CreateWindowExW(0, class.as_ptr(), w("Instalar Tenkaichi 3").as_ptr(), style, x, y, ww, wh, 0, 0, inst,
                                   std::ptr::null());
        ShowWindow(hwnd, 1);
        UpdateWindow(hwnd);
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, 0, 0, 0) > 0 {
            if IsDialogMessageW(hwnd, &msg) == 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    }
}

fn s(dpi: i32, v: i32) -> i32 {
    v * dpi / 96
}

unsafe fn control(parent: HWND, class: &str, label: &str, style: u32, ex: u32, x: i32, y: i32, cw: i32, ch: i32,
                  id: isize, font: HANDLE) -> HWND {
    let dpi = DPI.load(std::sync::atomic::Ordering::Relaxed);
    let h = CreateWindowExW(ex, w(class).as_ptr(), w(label).as_ptr(), WS_CHILD | WS_VISIBLE | style, s(dpi, x), s(dpi, y),
                            s(dpi, cw), s(dpi, ch), parent, id, GetModuleHandleW(std::ptr::null()), std::ptr::null());
    SendMessageW(h, WM_SETFONT, font as usize, 1);
    h
}

unsafe fn create(hwnd: HWND) {
    let dpi = DPI.load(std::sync::atomic::Ordering::Relaxed);
    let face = w("Segoe UI");
    let font = CreateFontW(-s(dpi, 15), 0, 0, 0, 400, 0, 0, 0, 1, 0, 0, 5, 0, face.as_ptr());
    let bold = CreateFontW(-s(dpi, 15), 0, 0, 0, 600, 0, 0, 0, 1, 0, 0, 5, 0, face.as_ptr());
    let title = CreateFontW(-s(dpi, 26), 0, 0, 0, 700, 0, 0, 0, 1, 0, 0, 5, 0, face.as_ptr());
    let mono = CreateFontW(-s(dpi, 13), 0, 0, 0, 400, 0, 0, 0, 1, 0, 0, 5, 0, w("Consolas").as_ptr());
    let st = 0; // SS_LEFT
    let lbl = |t: &str, y: i32, f: HANDLE| control(hwnd, "STATIC", t, st, 0, 24, y, 632, 20, 0, f);
    lbl("1.  Imagem do disco", 104, bold);
    let iso = control(hwnd, "EDIT", "", WS_TABSTOP | ES_AUTOHSCROLL, WS_EX_CLIENTEDGE, 24, 128, 520, 26, ID_ISO, font);
    let iso_browse = control(hwnd, "BUTTON", "Procurar...", WS_TABSTOP | BS_PUSHBUTTON, 0, 552, 127, 104, 28, ID_ISO_BROWSE, font);
    control(hwnd, "STATIC", "A sua cópia de Dragon Ball Z Budokai Tenkaichi 3 (EUA, SLUS-21678), em .iso.", st, 0, 24, 158, 632, 20, 0, font);
    lbl("2.  Tipo de instalação", 194, bold);
    let play = control(hwnd, "BUTTON", "Jogar", WS_TABSTOP | WS_GROUP | BS_AUTORADIOBUTTON, 0, 24, 218, 632, 22, ID_PLAY, bold);
    control(hwnd, "STATIC", "O jogo pronto para jogar: Tenkaichi3.exe com os dados em pacotes, como um jogo de PC.",
            st, 0, 44, 240, 612, 20, 0, font);
    let dev = control(hwnd, "BUTTON", "Desenvolvedor", WS_TABSTOP | BS_AUTORADIOBUTTON, 0, 24, 266, 632, 22, ID_DEV, bold);
    control(hwnd, "STATIC",
            "O mesmo jogo, com todos os arquivos editáveis (PNG, glTF, WAV, JSON) por tipo e nome, para mods.",
            st, 0, 44, 288, 612, 20, 0, font);
    lbl("3.  Pasta de instalação", 324, bold);
    let dir = control(hwnd, "EDIT", &default_dir(), WS_TABSTOP | ES_AUTOHSCROLL, WS_EX_CLIENTEDGE, 24, 348, 520, 26, ID_DIR, font);
    let dir_browse = control(hwnd, "BUTTON", "Procurar...", WS_TABSTOP | BS_PUSHBUTTON, 0, 552, 347, 104, 28, ID_DIR_BROWSE, font);
    lbl("4.  Texturas HD (opcional)", 388, bold);
    let mut tex = [0; 2];
    let mut tex_pick = [0; 2];
    let mut tex_from = [0; 2];
    for (i, label) in ["1080p (Alta): 1 GB", "4K (Ultra): 3,4 GB"].iter().enumerate() {
        let y = 412 + 28 * i as i32;
        tex[i] = control(hwnd, "BUTTON", label, WS_TABSTOP | BS_AUTOCHECKBOX, 0, 24, y, 220, 22, ID_TEX[i], font);
        tex_from[i] = control(hwnd, "STATIC", "baixado do Google Drive do autor", st, 0, 250, y + 2, 296, 20, 0, font);
        tex_pick[i] = control(hwnd, "BUTTON", "Já tenho...", WS_TABSTOP | BS_PUSHBUTTON, 0, 552, y - 2, 104, 26, ID_TEX_PICK[i], font);
    }
    control(hwnd, "STATIC", "Remaster das texturas feito pelo canal PLAYANDO (YouTube). Não faz parte deste projeto.",
            st, 0, 24, 470, 632, 20, 0, font);
    let status = control(hwnd, "STATIC", "Escolha a imagem do disco e clique em Instalar.", st, 0, 24, 504, 632, 20, 0, font);
    let progress = control(hwnd, "msctls_progress32", "", 0, 0, 24, 528, 632, 18, 0, font);
    SendMessageW(progress, PBM_SETRANGE32, 0, 1000);
    SendMessageW(progress, PBM_SETBARCOLOR, 0, ORANGE as isize);
    let log = control(hwnd, "EDIT", "", WS_VSCROLL | ES_MULTILINE | ES_READONLY | ES_AUTOVSCROLL, WS_EX_CLIENTEDGE,
                      24, 556, 632, 112, 0, mono);
    SendMessageW(log, EM_SETLIMITTEXT, 0, 0);
    let start = control(hwnd, "BUTTON", "Jogar agora", WS_TABSTOP | BS_PUSHBUTTON, 0, 324, 684, 112, 32, ID_START, font);
    let install = control(hwnd, "BUTTON", "Instalar", WS_TABSTOP | BS_DEFPUSHBUTTON, 0, 444, 684, 104, 32, ID_INSTALL, bold);
    let fresh = control(hwnd, "BUTTON", "Refazer tudo do zero", WS_TABSTOP | BS_AUTOCHECKBOX, 0, 24, 689, 280, 22, ID_FRESH, font);
    ShowWindow(fresh, 0);
    let close = control(hwnd, "BUTTON", "Fechar", WS_TABSTOP | BS_PUSHBUTTON, 0, 556, 684, 100, 32, ID_CLOSE, font);
    ShowWindow(start, 0);
    SendMessageW(play, BM_SETCHECK, 1, 0);
    let _ = HANDLES.set(H {
        iso,
        iso_browse,
        play,
        dev,
        dir,
        dir_browse,
        status,
        progress,
        log,
        install,
        close,
        start,
        fresh,
        tex,
        tex_pick,
        tex_from,
        paper: PAPER_BRUSH.load(std::sync::atomic::Ordering::Relaxed),
        navy: CreateSolidBrush(NAVY),
        orange: CreateSolidBrush(ORANGE),
        white: CreateSolidBrush(WHITE),
        font,
        title,
        dpi,
    });
    let _ = (bold, mono);
    if let Some(found) = installed_dir() {
        set_text(dir, &found);
    }
    check_existing();
}

/// The folder of a game installed before (InstallLocation in Settings > Apps, instalar.py register).
fn installed_dir() -> Option<String> {
    let key = r"Software\Microsoft\Windows\CurrentVersion\Uninstall\Tenkaichi3Rust";
    for root in ["HKLM", "HKCU"] {
        let out = hide(&mut Command::new("reg")).args(["query", &format!("{root}\\{key}"), "/v", "InstallLocation"]).output().ok()?;
        let text = String::from_utf8_lossy(&out.stdout).to_string();
        if let Some(line) = text.lines().find(|l| l.contains("InstallLocation")) {
            if let Some((_, path)) = line.split_once("REG_SZ") {
                let path = path.trim();
                if Path::new(path).join("assets").is_dir() {
                    return Some(path.to_string());
                }
            }
        }
    }
    None
}

/// Whether the chosen folder already has this installer's data for the chosen kind (as instalar.py up_to_date):
/// then Instalar becomes Atualizar, the disc image is not needed, and "Refazer tudo do zero" shows.
fn check_existing() {
    let u = h();
    if u.dir == 0 || state(|s| s.running) {
        return;
    }
    let dir = PathBuf::from(text(u.dir).trim().trim_matches('"'));
    let assets = dir.join("assets");
    let stamp: std::collections::HashMap<String, String> = std::fs::read_to_string(assets.join("instalacao.txt"))
        .unwrap_or_default()
        .lines()
        .filter_map(|l| l.split_once('=').map(|(k, v)| (k.trim().to_string(), v.trim().to_string())))
        .collect();
    let dev = is_dev();
    let mode = if dev { "desenvolvedor" } else { "jogador" };
    let have = if dev {
        assets.join("files").is_dir()
    } else {
        std::fs::read_dir(&assets).map(|r| r.flatten().any(|e| e.path().extension().map_or(false, |x| x == "bt3pak"))).unwrap_or(false)
    };
    let same = have && DATA_VERSION.is_some() && stamp.get("dados").map(String::as_str) == DATA_VERSION
        && stamp.get("modo").map(String::as_str) == Some(mode);
    let fresh = unsafe { SendMessageW(u.fresh, BM_GETCHECK, 0, 0) == 1 };
    state(|s| s.update = same && !fresh);
    unsafe { ShowWindow(u.fresh, same as i32) };
    let installed = |i: usize| assets.join("textures").join(["2x", "4x"][i]).is_dir();
    for i in 0..2 {
        if state(|s| s.tex_file[i].is_none()) {
            set_text(u.tex_from[i], if installed(i) { "já instaladas nesta pasta" } else { "baixado do Google Drive do autor" });
        }
    }
    if same && !fresh {
        set_text(u.install, "Atualizar");
        set_text(u.status, &format!("Já instalado aqui{}: só os programas serão trocados, sem a imagem do disco (segundos).",
                                     stamp.get("versao").map(|v| format!(" (versão {v})")).unwrap_or_default()));
    } else {
        set_text(u.install, "Instalar");
        set_text(u.status, if same { "Vai refazer tudo a partir da imagem do disco." } else if have {
            "Há um jogo nesta pasta de outra versão dos dados: vai ser refeito a partir da imagem do disco."
        } else {
            "Escolha a imagem do disco e clique em Instalar."
        });
    }
}

unsafe fn paint(hwnd: HWND) {
    let mut ps = PAINTSTRUCT { hdc: 0, fErase: 0, rcPaint: RECT::default(), fRestore: 0, fIncUpdate: 0, rgbReserved: [0; 32] };
    let dc = BeginPaint(hwnd, &mut ps);
    let H { dpi, navy, orange, title, font, .. } = h();
    let mut r = RECT::default();
    GetClientRect(hwnd, &mut r);
    let band = RECT { left: 0, top: 0, right: r.right, bottom: s(dpi, 84) };
    FillRect(dc, &band, navy);
    let stripe = RECT { left: 0, top: s(dpi, 84), right: r.right, bottom: s(dpi, 89) };
    FillRect(dc, &stripe, orange);
    SetBkMode(dc, 1); // transparent
    SetTextColor(dc, WHITE);
    SelectObject(dc, title);
    let t = w("Dragon Ball Z Budokai Tenkaichi 3");
    let mut tr = RECT { left: s(dpi, 24), top: s(dpi, 14), right: r.right - s(dpi, 24), bottom: s(dpi, 50) };
    DrawTextW(dc, t.as_ptr(), -1, &mut tr, DT_LEFT | DT_SINGLELINE | DT_VCENTER);
    SetTextColor(dc, 0x00D8C8B8);
    SelectObject(dc, font);
    let sub = w("Instalação para PC, a partir da sua imagem do disco");
    let mut sr = RECT { left: s(dpi, 24), top: s(dpi, 50), right: r.right - s(dpi, 24), bottom: s(dpi, 76) };
    DrawTextW(dc, sub.as_ptr(), -1, &mut sr, DT_LEFT | DT_SINGLELINE | DT_VCENTER | DT_WORDBREAK);
    EndPaint(hwnd, &ps);
}

fn is_dev() -> bool {
    unsafe { SendMessageW(h().dev, BM_GETCHECK, 0, 0) == 1 }
}

unsafe fn pick_iso(hwnd: HWND) {
    let mut file = vec![0u16; 1024];
    let filter: Vec<u16> = "Imagem de disco (*.iso)\0*.iso\0Todos os arquivos\0*.*\0\0".encode_utf16().collect();
    let title = w("Escolha a imagem do disco do Tenkaichi 3");
    let mut o = OPENFILENAMEW {
        lStructSize: std::mem::size_of::<OPENFILENAMEW>() as u32,
        hwndOwner: hwnd,
        hInstance: 0,
        lpstrFilter: filter.as_ptr(),
        lpstrCustomFilter: std::ptr::null_mut(),
        nMaxCustFilter: 0,
        nFilterIndex: 1,
        lpstrFile: file.as_mut_ptr(),
        nMaxFile: file.len() as u32,
        lpstrFileTitle: std::ptr::null_mut(),
        nMaxFileTitle: 0,
        lpstrInitialDir: std::ptr::null(),
        lpstrTitle: title.as_ptr(),
        Flags: 0x1000 | 0x800 | 0x80000, // OFN_FILEMUSTEXIST | OFN_PATHMUSTEXIST | OFN_EXPLORER
        nFileOffset: 0,
        nFileExtension: 0,
        lpstrDefExt: std::ptr::null(),
        lCustData: 0,
        lpfnHook: std::ptr::null(),
        lpTemplateName: std::ptr::null(),
        pvReserved: std::ptr::null_mut(),
        dwReserved: 0,
        FlagsEx: 0,
    };
    if GetOpenFileNameW(&mut o) != 0 {
        let n = file.iter().position(|&c| c == 0).unwrap_or(0);
        set_text(h().iso, &String::from_utf16_lossy(&file[..n]));
    }
}

/// "Já tenho...": a texture pack's .rar, or any .dds of a pack already extracted (then its folder).
unsafe fn pick_tex(hwnd: HWND, i: usize) {
    let mut file = vec![0u16; 1024];
    let filter: Vec<u16> = "Pacote de texturas (*.rar; *.dds)\0*.rar;*.dds\0Todos os arquivos\0*.*\0\0".encode_utf16().collect();
    let title = w(if i == 0 {
        "Texturas 1080p: escolha o TEXTURAS 1080p.rar (ou um .dds da pasta já extraída)"
    } else {
        "Texturas 4K: escolha o TEXTURAS 4K.rar (ou um .dds da pasta já extraída)"
    });
    let mut o = OPENFILENAMEW {
        lStructSize: std::mem::size_of::<OPENFILENAMEW>() as u32,
        hwndOwner: hwnd,
        hInstance: 0,
        lpstrFilter: filter.as_ptr(),
        lpstrCustomFilter: std::ptr::null_mut(),
        nMaxCustFilter: 0,
        nFilterIndex: 1,
        lpstrFile: file.as_mut_ptr(),
        nMaxFile: file.len() as u32,
        lpstrFileTitle: std::ptr::null_mut(),
        nMaxFileTitle: 0,
        lpstrInitialDir: std::ptr::null(),
        lpstrTitle: title.as_ptr(),
        Flags: 0x1000 | 0x800 | 0x80000,
        nFileOffset: 0,
        nFileExtension: 0,
        lpstrDefExt: std::ptr::null(),
        lCustData: 0,
        lpfnHook: std::ptr::null(),
        lpTemplateName: std::ptr::null(),
        pvReserved: std::ptr::null_mut(),
        dwReserved: 0,
        FlagsEx: 0,
    };
    if GetOpenFileNameW(&mut o) != 0 {
        let n = file.iter().position(|&c| c == 0).unwrap_or(0);
        let mut p = PathBuf::from(String::from_utf16_lossy(&file[..n]));
        if p.extension().map_or(false, |e| e.eq_ignore_ascii_case("dds")) {
            p = p.parent().map(Path::to_path_buf).unwrap_or(p);
        }
        let u = h();
        set_text(u.tex_from[i], &format!("de {}", p.file_name().map(|f| f.to_string_lossy().into_owned()).unwrap_or_default()));
        SendMessageW(u.tex[i], BM_SETCHECK, 1, 0);
        state(|s| s.tex_file[i] = Some(p.display().to_string()));
    }
}

unsafe fn pick_dir(hwnd: HWND) {
    let mut name = vec![0u16; 260];
    let title = w("Escolha a pasta onde o jogo vai ficar");
    let b = BROWSEINFOW {
        hwndOwner: hwnd,
        pidlRoot: std::ptr::null(),
        pszDisplayName: name.as_mut_ptr(),
        lpszTitle: title.as_ptr(),
        ulFlags: 0x1 | 0x40, // BIF_RETURNONLYFSDIRS | BIF_NEWDIALOGSTYLE
        lpfn: std::ptr::null(),
        lParam: 0,
        iImage: 0,
    };
    let pidl = SHBrowseForFolderW(&b);
    if !pidl.is_null() {
        let mut path = vec![0u16; 1024];
        if SHGetPathFromIDListW(pidl, path.as_mut_ptr()) != 0 {
            let n = path.iter().position(|&c| c == 0).unwrap_or(0);
            let mut p = PathBuf::from(String::from_utf16_lossy(&path[..n]));
            if p.file_name().map(|f| f != "Tenkaichi3").unwrap_or(true) {
                p = p.join("Tenkaichi3");
            }
            set_text(h().dir, &p.display().to_string());
        }
        CoTaskMemFree(pidl);
    }
}

fn message(hwnd: HWND, text: &str, kind: u32) -> i32 {
    unsafe { MessageBoxW(hwnd, w(text).as_ptr(), w("Instalar Tenkaichi 3").as_ptr(), kind) }
}

/// The Python that runs the installation: the player package's own, or the project's .venv (made when missing).
fn python(root: &Path, log: &dyn Fn(&str)) -> Result<PathBuf, String> {
    let packaged = root.join("python").join("python.exe");
    if packaged.is_file() {
        return Ok(packaged);
    }
    let venv = root.join(".venv").join("Scripts").join("python.exe");
    if !venv.is_file() {
        log("Criando o ambiente Python do projeto (.venv)...");
        let made = run_logged(Command::new("py").args(["-3", "-m", "venv"]).arg(root.join(".venv")), log)
            .or_else(|_| run_logged(Command::new("python").args(["-m", "venv"]).arg(root.join(".venv")), log));
        if made.is_err() || !venv.is_file() {
            return Err("Não achei o Python 3. Instale em https://www.python.org e tente de novo.".into());
        }
    }
    run_logged(Command::new(&venv).args(["-m", "pip", "install", "--quiet", "numpy"]), log)?;
    Ok(venv)
}

use std::process::{Command, Stdio};

fn hide(c: &mut Command) -> &mut Command {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        c.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }
    c
}

/// Runs a command, each line of its output to `log`; Ok when it ends well.
fn run_logged(c: &mut Command, log: &dyn Fn(&str)) -> Result<(), String> {
    use std::io::{BufRead, BufReader};
    let mut child = hide(c)
        .env("PYTHONIOENCODING", "utf-8")
        .env("PYTHONUNBUFFERED", "1")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    state(|s| s.child = Some(child.id()));
    let err = child.stderr.take().unwrap();
    let errors = std::thread::spawn(move || {
        let mut v = Vec::new();
        for line in BufReader::new(err).split(b'\n').map_while(Result::ok) {
            let l = String::from_utf8_lossy(&line).trim_end().to_string();
            push(&l);
            v.push(l);
        }
        v
    });
    for line in BufReader::new(child.stdout.take().unwrap()).split(b'\n').map_while(Result::ok) {
        log(String::from_utf8_lossy(&line).trim_end());
    }
    let status = child.wait().map_err(|e| e.to_string())?;
    let errors = errors.join().unwrap_or_default();
    state(|s| s.child = None);
    if status.success() {
        Ok(())
    } else {
        Err(errors.into_iter().rev().find(|l| !l.is_empty()).unwrap_or_else(|| format!("terminou com {status}")))
    }
}

fn push(line: &str) {
    OUTPUT.lock().unwrap().push_back(line.to_string());
    unsafe { PostMessageW(MAIN.load(std::sync::atomic::Ordering::Relaxed), WM_OUTPUT, 0, 0) };
}

fn start_install(hwnd: HWND) {
    let u = h();
    let iso = text(u.iso).trim().trim_matches('"').to_string();
    let dir = text(u.dir).trim().trim_matches('"').to_string();
    let dev = is_dev();
    let files = state(|s| s.tex_file.clone());
    let textures: Vec<(&str, String)> = (0..2)
        .filter(|&i| unsafe { SendMessageW(u.tex[i], BM_GETCHECK, 0, 0) == 1 })
        .map(|i| (["--texturas-2x", "--texturas-4x"][i], files[i].clone().unwrap_or_else(|| "baixar".into())))
        .collect();
    let update = state(|s| s.update);
    let fresh = unsafe { SendMessageW(u.fresh, BM_GETCHECK, 0, 0) == 1 };
    if !update && !Path::new(&iso).is_file() {
        message(hwnd, "Escolha a imagem do disco (.iso) do Tenkaichi 3.", MB_OK | MB_ICONWARNING);
        return;
    }
    if dir.is_empty() {
        message(hwnd, "Escolha a pasta de instalação.", MB_OK | MB_ICONWARNING);
        return;
    }
    state(|s| {
        s.running = true;
        s.stage = 0.0;
    });
    unsafe {
        for c in [u.iso, u.iso_browse, u.play, u.dev, u.dir, u.dir_browse, u.install, u.tex[0], u.tex[1], u.tex_pick[0], u.tex_pick[1], u.fresh] {
            EnableWindow(c, 0);
        }
        SendMessageW(u.progress, PBM_SETPOS, 0, 0);
    }
    set_text(u.close, "Cancelar");
    set_text(u.status, "Preparando...");
    std::thread::spawn(move || {
        let result = (|| -> Result<PathBuf, String> {
            let root = match unpack(&push)? {
                Some(d) => d,
                None => root(),
            };
            std::fs::create_dir_all(&dir).map_err(|e| format!("{dir}: {e}"))?;
            if dev && ADMIN.load(std::sync::atomic::Ordering::Relaxed) {
                // the editable files stay editable without the administrator: the Users group may change the folder
                let _ = hide(&mut Command::new("icacls")).arg(&dir).args(["/grant", "*S-1-5-32-545:(OI)(CI)M"]).status();
            }
            let py = python(&root, &push)?;
            let mut c = Command::new(py);
            c.arg("-u").arg(root.join("tools").join("install").join("instalar.py")).current_dir(&root);
            if !iso.is_empty() {
                c.arg(&iso);
            }
            if fresh {
                c.arg("--do-zero");
            }
            c.args(["--modo", if dev { "desenvolvedor" } else { "jogador" }, "--destino", &dir]);
            for (flag, from) in &textures {
                c.arg(flag).arg(from);
            }
            run_logged(&mut c, &push)?;
            Ok(PathBuf::from(&dir).join("Tenkaichi3.exe"))
        })();
        let ok = result.is_ok();
        match result {
            Ok(exe) => state(|s| s.done = Some(exe)),
            Err(e) => push(&format!("ERRO: {e}")),
        }
        unsafe { PostMessageW(MAIN.load(std::sync::atomic::Ordering::Relaxed), WM_FINISHED, ok as usize, 0) };
    });
}

/// Where the installation is, from a line of its output: (progress 0..1000, status text).
fn progress_of(line: &str, dev: bool) -> Option<(f64, String)> {
    let stage = |a: f64, t: &str| Some((a, t.to_string()));
    if let Some(rest) = line.strip_prefix("== ") {
        return match rest.get(..3) {
            Some("1/6") => stage(10.0, "Lendo o disco (1 de 6)"),
            Some("3/6") => stage(60.0, "Copiando os arquivos e convertendo músicas e vozes (2 de 6)"),
            Some("2/6") => stage(150.0, "Tabelas do jogo (3 de 6)"),
            Some("4/6") => stage(160.0, "Convertendo os arquivos do jogo (4 de 6); a primeira vez demora bastante"),
            Some("5/6") => stage(780.0, "Recursos de PC e pastas (5 de 6)"),
            Some("6/6") => stage(if dev { 950.0 } else { 870.0 }, "Vídeos (6 de 6)"),
            _ if rest.starts_with("empacot") => stage(900.0, "Empacotando os dados do jogo"),
            _ if rest.starts_with("atualizando") => stage(300.0, "Atualizando: os dados do jogo já estão em dia"),
            _ if rest.starts_with("instalando o jogo") => stage(if state(|s| s.update) { 600.0 } else { 890.0 }, "Instalando o programa"),
            _ if rest.starts_with("texturas HD 1080p") => stage(940.0, "Texturas HD 1080p"),
            _ if rest.starts_with("texturas HD 4K") => stage(965.0, "Texturas HD 4K"),
            _ if rest.starts_with("compilando") => None,
            _ => None,
        };
    }
    // texturas.py: "texturas 1080p: baixando 45% (...)", "texturas 4K: extraindo com 7z.exe"
    if let Some(rest) = line.strip_prefix("texturas ") {
        let (label, what) = rest.split_once(": ")?;
        let (base, span) = if label == "4K" { (965.0, 30.0) } else { (940.0, 24.0) };
        if let Some(pct) = what.strip_prefix("baixando ").and_then(|w| w.split('%').next()).and_then(|p| p.trim().parse::<f64>().ok()) {
            return Some((base + span * 0.8 * pct / 100.0, format!("Baixando as texturas HD {label}: {}", what.trim_start_matches("baixando "))));
        }
        if what.starts_with("extraindo") {
            return Some((base + span * 0.8, format!("Extraindo as texturas HD {label}")));
        }
        return None;
    }
    // files_extract.py: "<n> files, <m> textures (<s>s) of <total>" every 100 files (older: without the total)
    if let Some((n, rest)) = line.split_once(" files, ") {
        if let Ok(n) = n.trim().parse::<f64>() {
            let total = rest.rsplit_once(" of ").and_then(|(_, t)| t.trim().parse::<f64>().ok()).unwrap_or(3400.0).max(1.0);
            return Some((160.0 + 600.0 * (n / total).min(1.0), format!("Convertendo os arquivos do jogo (4 de 6): {n} de {total}")));
        }
    }
    None
}

extern "system" fn wndproc(hwnd: HWND, msg: u32, wp: usize, lp: isize) -> isize {
    unsafe {
        match msg {
            WM_CREATE => {
                MAIN.store(hwnd, std::sync::atomic::Ordering::Relaxed);
                create(hwnd);
                0
            }
            WM_PAINT => {
                paint(hwnd);
                0
            }
            WM_CTLCOLORSTATIC | WM_CTLCOLORBTN => {
                // labels and radio buttons on the window's colour; the read-only log white
                let Some(u) = HANDLES.get().copied() else { return DefWindowProcW(hwnd, msg, wp, lp) };
                let (paper, white, log) = (u.paper, u.white, u.log);
                SetBkColor(wp as HANDLE, PAPER);
                if lp == log {
                    SetBkColor(wp as HANDLE, WHITE);
                    return white;
                }
                paper
            }
            WM_CTLCOLOREDIT => DefWindowProcW(hwnd, msg, wp, lp),
            WM_COMMAND => {
                if (wp >> 16) & 0xFFFF == EN_CHANGE && (wp & 0xFFFF) as isize == ID_DIR {
                    check_existing();
                    return 0;
                }
                match (wp & 0xFFFF) as isize {
                    ID_FRESH => check_existing(),
                    ID_ISO_BROWSE => pick_iso(hwnd),
                    ID_DIR_BROWSE => pick_dir(hwnd),
                    id if id == ID_TEX_PICK[0] => pick_tex(hwnd, 0),
                    id if id == ID_TEX_PICK[1] => pick_tex(hwnd, 1),
                    ID_PLAY | ID_DEV => check_existing(),
                    ID_INSTALL => start_install(hwnd),
                    ID_START => {
                        if let Some(p) = state(|s| s.done.clone()) {
                            let dir = p.parent().map(|d| d.display().to_string()).unwrap_or_default();
                            ShellExecuteW(hwnd, w("open").as_ptr(), w(&p.display().to_string()).as_ptr(), std::ptr::null(),
                                          w(&dir).as_ptr(), 1);
                            PostMessageW(hwnd, WM_CLOSE, 0, 0);
                        }
                    }
                    ID_CLOSE => {
                        PostMessageW(hwnd, WM_CLOSE, 0, 0);
                    }
                    _ => {}
                }
                0
            }
            WM_OUTPUT => {
                let lines: Vec<String> = OUTPUT.lock().unwrap().drain(..).collect();
                let dev = is_dev();
                let u = h();
                for l in &lines {
                    append_log(u.log, l);
                    if let Some((p, st)) = progress_of(l, dev) {
                        if state(|s| if p >= s.stage { s.stage = p; true } else { false }) {
                            SendMessageW(u.progress, PBM_SETPOS, p as usize, 0);
                            set_text(u.status, &st);
                        }
                    }
                }
                0
            }
            WM_FINISHED => {
                let ok = wp != 0;
                state(|s| s.running = false);
                let u = h();
                set_text(u.close, "Fechar");
                if ok {
                    SendMessageW(u.progress, PBM_SETPOS, 1000, 0);
                    set_text(u.status, "Pronto! O jogo está instalado.");
                    ShowWindow(u.start, 1);
                } else {
                    set_text(u.status, "A instalação parou. Veja o motivo abaixo; ao instalar de novo, os passos prontos são pulados.");
                    for c in [u.iso, u.iso_browse, u.play, u.dev, u.dir, u.dir_browse, u.install, u.tex[0], u.tex[1], u.tex_pick[0], u.tex_pick[1], u.fresh] {
                        EnableWindow(c, 1);
                    }
                }
                if ok {
                    message(hwnd, "Instalação concluída. Clique em \"Jogar agora\" ou use o atalho \"Tenkaichi 3\" da área de trabalho ou do menu Iniciar.",
                            MB_OK | MB_ICONINFO);
                }
                0
            }
            WM_CLOSE => {
                let (running, child) = state(|s| (s.running, s.child));
                if running {
                    if message(hwnd, "Parar a instalação? Ao instalar de novo, os passos prontos são pulados.", MB_YESNO | MB_ICONWARNING)
                        != IDYES
                    {
                        return 0;
                    }
                    if let Some(pid) = child {
                        // the installer and the programs it started
                        let _ = hide(&mut Command::new("taskkill")).args(["/T", "/F", "/PID", &pid.to_string()]).status();
                    }
                }
                DefWindowProcW(hwnd, msg, wp, lp)
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                0
            }
            _ => DefWindowProcW(hwnd, msg, wp, lp),
        }
    }
}
