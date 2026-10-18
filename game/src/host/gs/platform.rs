//! Where the GPU renderer draws: an SDL3 window with an OpenGL 3.3 core context (SDL3.dll / libSDL3.so loaded at
//! run time), or, for tests without a display, an EGL context without any surface (Mesa). No crates: the
//! libraries are opened with the system loader (LoadLibrary / dlopen).
#![allow(non_snake_case)]
use std::ffi::{c_char, c_void, CString};

// ---------------------------------------------------------------------------------------------- loader
#[cfg(windows)]
pub(crate) mod sys {
    use std::ffi::{c_char, c_void};
    #[link(name = "kernel32")]
    extern "system" {
        fn LoadLibraryExA(name: *const c_char, file: *mut c_void, flags: u32) -> *mut c_void;
        fn GetProcAddress(lib: *mut c_void, name: *const c_char) -> *mut c_void;
        fn GetLastError() -> u32;
    }
    pub fn last_error() -> u32 {
        unsafe { GetLastError() }
    }
    pub unsafe fn open(name: &str) -> *mut c_void {
        let n = std::ffi::CString::new(name).unwrap();
        // LOAD_WITH_ALTERED_SEARCH_PATH: a library given by its path finds the libraries it needs in its own
        // folder (SDL3.dll needs libiconv-2.dll beside it)
        let flags = if name.contains('\\') || name.contains('/') { 0x8 } else { 0 };
        LoadLibraryExA(n.as_ptr(), std::ptr::null_mut(), flags)
    }
    pub unsafe fn sym(lib: *mut c_void, name: &str) -> *mut c_void {
        let n = std::ffi::CString::new(name).unwrap();
        GetProcAddress(lib, n.as_ptr())
    }
}

#[cfg(not(windows))]
pub(crate) mod sys {
    use std::ffi::{c_char, c_int, c_void};
    #[link(name = "dl")]
    extern "C" {
        fn dlopen(name: *const c_char, flags: c_int) -> *mut c_void;
        fn dlsym(lib: *mut c_void, name: *const c_char) -> *mut c_void;
    }
    pub unsafe fn open(name: &str) -> *mut c_void {
        let n = std::ffi::CString::new(name).unwrap();
        dlopen(n.as_ptr(), 2 /* RTLD_NOW */)
    }
    pub unsafe fn sym(lib: *mut c_void, name: &str) -> *mut c_void {
        let n = std::ffi::CString::new(name).unwrap();
        dlsym(lib, n.as_ptr())
    }
    pub fn last_error() -> u32 {
        0
    }
}

macro_rules! load {
    ($lib:expr, $name:literal, $t:ty) => {{
        let p = unsafe { sys::sym($lib, $name) };
        if p.is_null() {
            return Err(format!("{} not found", $name));
        }
        unsafe { std::mem::transmute::<*mut c_void, $t>(p) }
    }};
}

/// The vibration the game asked for on controller 1 (sceVibSetActParam, host/mod.rs): the DualShock's actuator
/// byte, large motor power (0..127) << 1 | small motor on.
pub static RUMBLE: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

/// What the game reads from the keyboard / pad this frame (PS2 button bits, active high, and the sticks).
#[derive(Clone, Copy, Default)]
pub struct Input {
    pub buttons: u16,
    pub lx: u8,
    pub ly: u8,
    pub rx: u8,
    pub ry: u8,
    pub quit: bool,
    /// Esc pressed (the quit question, gs/gpu.rs)
    pub escape: bool,
    /// F1 pressed (the PC settings, host/settings.rs)
    pub settings: bool,
    /// the first key pressed this frame (SDL scancode, 0: none), for choosing keys
    pub key_down: u32,
}

pub enum Backend {
    Sdl(Sdl),
    Egl(Egl),
}

pub struct Surface {
    pub backend: Backend,
}

impl Surface {
    /// A window (BT3_GS=gpu) or, with `headless`, an offscreen context.
    pub fn create(headless: bool) -> Result<Surface, String> {
        if headless {
            return Ok(Surface { backend: Backend::Egl(Egl::create()?) });
        }
        Ok(Surface { backend: Backend::Sdl(Sdl::create()?) })
    }

    pub fn proc_address(&self, name: &str) -> *const c_void {
        let n = CString::new(name).unwrap();
        match &self.backend {
            Backend::Sdl(s) => unsafe { (s.get_proc)(n.as_ptr()) as *const c_void },
            Backend::Egl(e) => unsafe { (e.get_proc)(n.as_ptr()) as *const c_void },
        }
    }

    /// The SDL3 library of the window (None headless).
    pub fn sdl_lib(&self) -> Option<*mut c_void> {
        match &self.backend {
            Backend::Sdl(s) => Some(s.lib),
            Backend::Egl(_) => None,
        }
    }

    /// Full screen on or off (the window's).
    pub fn set_fullscreen(&mut self, on: bool) {
        if let Backend::Sdl(s) = &mut self.backend {
            if s.full != on {
                s.full = on;
                unsafe { (s.set_full)(s.window, on) };
            }
        }
    }

    /// The window's size (not in full screen): `w` x `h`, made smaller (same shape) to fit the display.
    pub fn set_window_size(&mut self, w: i32, h: i32) {
        if let Backend::Sdl(s) = &mut self.backend {
            let mut r = [0i32; 4];
            let (mut w, mut h) = (w, h);
            if unsafe { (s.usable)((s.display_of)(s.window), &mut r) } && r[2] > 0 && r[3] > 0 {
                let k = (r[2] as f32 * 0.95 / w as f32).min(r[3] as f32 * 0.92 / h as f32).min(1.0);
                w = (w as f32 * k) as i32;
                h = (h as f32 * k) as i32;
            }
            unsafe { (s.set_size)(s.window, w, h) };
        }
    }

    pub fn has_window(&self) -> bool {
        matches!(self.backend, Backend::Sdl(_))
    }

    /// The drawable's size in pixels.
    pub fn size(&self) -> (i32, i32) {
        match &self.backend {
            Backend::Sdl(s) => {
                let (mut w, mut h) = (0, 0);
                unsafe { (s.size)(s.window, &mut w, &mut h) };
                (w, h)
            }
            Backend::Egl(_) => (0, 0),
        }
    }

    pub fn swap(&mut self) {
        if let Backend::Sdl(s) = &self.backend {
            unsafe { (s.swap)(s.window) };
        }
    }

    /// Events and the keyboard / pad state.
    pub fn poll(&mut self) -> Input {
        match &mut self.backend {
            Backend::Sdl(s) => s.poll(),
            Backend::Egl(_) => Input { lx: 128, ly: 128, rx: 128, ry: 128, ..Default::default() },
        }
    }
}

// ---------------------------------------------------------------------------------------------- SDL3
pub struct Sdl {
    /// The loaded SDL3 library (the sound code takes its functions from it too).
    pub lib: *mut c_void,
    window: *mut c_void,
    get_proc: unsafe extern "C" fn(*const c_char) -> *mut c_void,
    swap: unsafe extern "C" fn(*mut c_void) -> bool,
    size: unsafe extern "C" fn(*mut c_void, *mut i32, *mut i32) -> bool,
    poll_event: unsafe extern "C" fn(*mut [u8; 128]) -> bool,
    keys: unsafe extern "C" fn(*mut i32) -> *const bool,
    gamepads: unsafe extern "C" fn(*mut i32) -> *mut u32,
    open_pad: unsafe extern "C" fn(u32) -> *mut c_void,
    pad_axis: unsafe extern "C" fn(*mut c_void, i32) -> i16,
    pad_button: unsafe extern "C" fn(*mut c_void, i32) -> bool,
    rumble: unsafe extern "C" fn(*mut c_void, u16, u16, u32) -> bool,
    /// the actuator byte last sent, and the frames since
    rumble_sent: (u32, u32),
    free: unsafe extern "C" fn(*mut c_void),
    pad: *mut c_void,
    full: bool,
    set_full: unsafe extern "C" fn(*mut c_void, bool) -> bool,
    set_size: unsafe extern "C" fn(*mut c_void, i32, i32) -> bool,
    display_of: unsafe extern "C" fn(*mut c_void) -> u32,
    usable: unsafe extern "C" fn(u32, *mut [i32; 4]) -> bool,
}

impl Sdl {
    fn create() -> Result<Sdl, String> {
        // BT3_SDL3=<path of the library>, else next to the program, else the system's search
        let mut names: Vec<String> = Vec::new();
        if let Ok(p) = std::env::var("BT3_SDL3") {
            names.push(p);
        }
        let file = if cfg!(windows) { "SDL3.dll" } else { "libSDL3.so.0" };
        if let Some(dir) = std::env::current_exe().ok().and_then(|e| e.parent().map(|d| d.to_path_buf())) {
            names.push(dir.join(file).to_string_lossy().into_owned());
        }
        names.push(file.to_string());
        if !cfg!(windows) {
            names.push("libSDL3.so".to_string());
        }
        let mut lib = std::ptr::null_mut();
        let mut tried = Vec::new();
        for n in &names {
            lib = unsafe { sys::open(n) };
            if !lib.is_null() {
                break;
            }
            tried.push(format!("{n} (error {})", sys::last_error()));
        }
        if lib.is_null() {
            return Err(format!("SDL3 library not found; tried: {}", tried.join(", ")));
        }
        let init = load!(lib, "SDL_Init", unsafe extern "C" fn(u32) -> bool);
        let attr = load!(lib, "SDL_GL_SetAttribute", unsafe extern "C" fn(i32, i32) -> bool);
        let create_window = load!(lib, "SDL_CreateWindow", unsafe extern "C" fn(*const c_char, i32, i32, u64) -> *mut c_void);
        let create_context = load!(lib, "SDL_GL_CreateContext", unsafe extern "C" fn(*mut c_void) -> *mut c_void);
        let make_current = load!(lib, "SDL_GL_MakeCurrent", unsafe extern "C" fn(*mut c_void, *mut c_void) -> bool);
        let interval = load!(lib, "SDL_GL_SetSwapInterval", unsafe extern "C" fn(i32) -> bool);
        let error = load!(lib, "SDL_GetError", unsafe extern "C" fn() -> *const c_char);
        let err = || unsafe { std::ffi::CStr::from_ptr(error()).to_string_lossy().into_owned() };
        if !unsafe { init(0x20 | 0x2000) } {
            return Err(format!("SDL_Init: {}", err()));
        }
        for (k, v) in [(17, 3), (18, 3), (20, 1), (6, 24), (5, 1)] {
            unsafe { attr(k, v) };
        }
        let title = CString::new("Dragon Ball Z Budokai Tenkaichi 3 (Rust)").unwrap();
        // SDL_WINDOW_OPENGL | SDL_WINDOW_RESIZABLE
        // BT3_HIDDEN=1: the window is never shown (automatic tests on someone's desktop; the picture still goes to
        // the screenshots, which are taken from the render targets)
        let hidden: u64 = if std::env::var("BT3_HIDDEN").is_ok() { 0x8 } else { 0 };
        // the window has the picture's shape (host/settings.rs aspect) at a modest size; the chosen resolution is
        // given to it right after (Gpu::apply_settings, which keeps it inside the display)
        let ww = (720.0 * crate::host::settings::aspect_milli() as f32 / 1000.0 + 0.5) as i32;
        let window = unsafe { create_window(title.as_ptr(), ww, 720, 0x2 | 0x20 | hidden) };
        if window.is_null() {
            return Err(format!("SDL_CreateWindow: {}", err()));
        }
        let ctx = unsafe { create_context(window) };
        if ctx.is_null() || !unsafe { make_current(window, ctx) } {
            return Err(format!("no OpenGL 3.3 core context: {}", err()));
        }
        unsafe { interval(0) };
        Ok(Sdl {
            lib,
            window,
            get_proc: load!(lib, "SDL_GL_GetProcAddress", unsafe extern "C" fn(*const c_char) -> *mut c_void),
            swap: load!(lib, "SDL_GL_SwapWindow", unsafe extern "C" fn(*mut c_void) -> bool),
            size: load!(lib, "SDL_GetWindowSizeInPixels", unsafe extern "C" fn(*mut c_void, *mut i32, *mut i32) -> bool),
            poll_event: load!(lib, "SDL_PollEvent", unsafe extern "C" fn(*mut [u8; 128]) -> bool),
            keys: load!(lib, "SDL_GetKeyboardState", unsafe extern "C" fn(*mut i32) -> *const bool),
            gamepads: load!(lib, "SDL_GetGamepads", unsafe extern "C" fn(*mut i32) -> *mut u32),
            open_pad: load!(lib, "SDL_OpenGamepad", unsafe extern "C" fn(u32) -> *mut c_void),
            pad_axis: load!(lib, "SDL_GetGamepadAxis", unsafe extern "C" fn(*mut c_void, i32) -> i16),
            pad_button: load!(lib, "SDL_GetGamepadButton", unsafe extern "C" fn(*mut c_void, i32) -> bool),
            rumble: load!(lib, "SDL_RumbleGamepad", unsafe extern "C" fn(*mut c_void, u16, u16, u32) -> bool),
            rumble_sent: (0, 0),
            free: load!(lib, "SDL_free", unsafe extern "C" fn(*mut c_void)),
            set_full: load!(lib, "SDL_SetWindowFullscreen", unsafe extern "C" fn(*mut c_void, bool) -> bool),
            set_size: load!(lib, "SDL_SetWindowSize", unsafe extern "C" fn(*mut c_void, i32, i32) -> bool),
            display_of: load!(lib, "SDL_GetDisplayForWindow", unsafe extern "C" fn(*mut c_void) -> u32),
            usable: load!(lib, "SDL_GetDisplayUsableBounds", unsafe extern "C" fn(u32, *mut [i32; 4]) -> bool),
            pad: std::ptr::null_mut(),
            full: false,
        })
        .map(|mut s| {
            if crate::host::settings::get().fullscreen {
                s.full = true;
                unsafe { (s.set_full)(s.window, true) };
            }
            s
        })
    }

    fn poll(&mut self) -> Input {
        let mut inp = Input { lx: 128, ly: 128, rx: 128, ry: 128, ..Default::default() };
        let mut ev = [0u8; 128];
        while unsafe { (self.poll_event)(&mut ev) } {
            let t = u32::from_le_bytes(ev[0..4].try_into().unwrap());
            // SDL_EVENT_QUIT, SDL_EVENT_WINDOW_CLOSE_REQUESTED
            if t == 0x100 || t == 0x210 {
                inp.quit = true;
            }
            // SDL_EVENT_KEY_DOWN: F11 toggles full screen (scancode at offset 24)
            if t == 0x300 {
                let scan = u32::from_le_bytes(ev[24..28].try_into().unwrap());
                let repeat = ev[37] != 0;
                if scan == 68 && !repeat {
                    self.full = !self.full;
                    unsafe { (self.set_full)(self.window, self.full) };
                    let mut st = crate::host::settings::get();
                    st.fullscreen = self.full;
                    crate::host::settings::set(st);
                }
                if scan == 41 && !repeat {
                    inp.escape = true;
                }
                if scan == 58 && !repeat {
                    inp.settings = true;
                }
                if !repeat && inp.key_down == 0 {
                    inp.key_down = scan;
                }
            }
        }
        if self.pad.is_null() {
            let mut n = 0;
            let ids = unsafe { (self.gamepads)(&mut n) };
            if !ids.is_null() {
                if n > 0 {
                    self.pad = unsafe { (self.open_pad)(*ids) };
                }
                unsafe { (self.free)(ids as *mut c_void) };
            }
        }
        // the vibration: sent when it changes, and again before SDL's duration runs out
        if !self.pad.is_null() {
            let want = RUMBLE.load(std::sync::atomic::Ordering::Relaxed);
            self.rumble_sent.1 += 1;
            if want != self.rumble_sent.0 || (want != 0 && self.rumble_sent.1 >= 20) {
                let large = ((want >> 1) & 0x7F) * 0xFFFF / 0x7F;
                let small = if want & 1 != 0 { 0xC000 } else { 0 };
                unsafe { (self.rumble)(self.pad, large as u16, small, 500) };
                self.rumble_sent = (want, 0);
            }
        }
        let mut nk = 0;
        let keys = unsafe { (self.keys)(&mut nk) };
        let key = |s: usize| -> bool { !keys.is_null() && (s as i32) < nk && unsafe { *keys.add(s) } };
        let pad = self.pad;
        let button = |b: i32| -> bool { !pad.is_null() && unsafe { (self.pad_button)(pad, b) } };
        let axis = |a: i32| -> i16 { if pad.is_null() { 0 } else { unsafe { (self.pad_axis)(pad, a) } } };
        // PS2 digital buttons (bit = pressed): select 0, L3 1, R3 2, start 3, up 4, right 5, down 6, left 7,
        // L2 8, R2 9, L1 10, R1 11, triangle 12, circle 13, cross 14, square 15.
        // Keyboard (the C port's defaults): arrows, K cross, L circle, J square, I triangle, U L1, O R1, 7 L2, 9 R2,
        // Enter start, right Shift select; W A S D the left stick.
        // SDL gamepad buttons: south 0, east 1, west 2, north 3, back 4, guide 5, start 6, lstick 7, rstick 8,
        // lshoulder 9, rshoulder 10, up 11, down 12, left 13, right 14. Triggers are axes 4 and 5.
        // the keys from the settings (host/settings.rs, F1), the gamepad buttons fixed
        let keys = crate::host::settings::get().keys;
        let pad_buttons: [i32; 16] = [4, 7, 8, 6, 11, 14, 12, 13, -1, -2, 9, 10, 3, 1, 0, 2];
        let map: Vec<(usize, i32, u16)> = (0..16).map(|k| (keys[k] as usize, pad_buttons[k], 1u16 << k)).collect();
        for (sc, b, bit) in map {
            let pressed = (sc != 0 && key(sc))
                || match b {
                    -1 => axis(4) > 12000,
                    -2 => axis(5) > 12000,
                    b => button(b),
                };
            if pressed {
                inp.buttons |= bit;
            }
        }
        let st = |v: i16| -> u8 { ((v as i32 + 32768) >> 8) as u8 };
        inp.lx = st(axis(0));
        inp.ly = st(axis(1));
        inp.rx = st(axis(2));
        inp.ry = st(axis(3));
        let held = |k: u32| k != 0 && key(k as usize);
        if held(keys[16]) {
            inp.ly = 0x00;
        }
        if held(keys[17]) {
            inp.ly = 0xFF;
        }
        if held(keys[18]) {
            inp.lx = 0x00;
        }
        if held(keys[19]) {
            inp.lx = 0xFF;
        }
        inp
    }
}

// ---------------------------------------------------------------------------------------------- EGL
pub struct Egl {
    get_proc: unsafe extern "C" fn(*const c_char) -> *mut c_void,
}

impl Egl {
    fn create() -> Result<Egl, String> {
        let lib = unsafe { sys::open("libEGL.so.1") };
        if lib.is_null() {
            return Err("libEGL.so.1 not found".into());
        }
        let get_platform_display = load!(lib, "eglGetPlatformDisplay", unsafe extern "C" fn(u32, *mut c_void, *const isize) -> *mut c_void);
        let initialize = load!(lib, "eglInitialize", unsafe extern "C" fn(*mut c_void, *mut i32, *mut i32) -> u32);
        let bind_api = load!(lib, "eglBindAPI", unsafe extern "C" fn(u32) -> u32);
        let choose = load!(lib, "eglChooseConfig", unsafe extern "C" fn(*mut c_void, *const i32, *mut *mut c_void, i32, *mut i32) -> u32);
        let create_context = load!(lib, "eglCreateContext", unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void, *const i32) -> *mut c_void);
        let make_current = load!(lib, "eglMakeCurrent", unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void, *mut c_void) -> u32);
        let get_proc = load!(lib, "eglGetProcAddress", unsafe extern "C" fn(*const c_char) -> *mut c_void);
        // EGL_PLATFORM_SURFACELESS_MESA
        let dpy = unsafe { get_platform_display(0x31DD, std::ptr::null_mut(), std::ptr::null()) };
        if dpy.is_null() {
            return Err("no surfaceless EGL display".into());
        }
        let (mut ma, mut mi) = (0, 0);
        if unsafe { initialize(dpy, &mut ma, &mut mi) } == 0 {
            return Err("eglInitialize failed".into());
        }
        unsafe { bind_api(0x30A2) }; // EGL_OPENGL_API
        // EGL_SURFACE_TYPE 0x3033 = EGL_PBUFFER_BIT 1, EGL_RENDERABLE_TYPE 0x3040 = EGL_OPENGL_BIT 8
        let attrs = [0x3033, 1, 0x3040, 8, 0x3024, 8, 0x3023, 8, 0x3022, 8, 0x3038];
        let mut cfg = std::ptr::null_mut();
        let mut n = 0;
        if unsafe { choose(dpy, attrs.as_ptr(), &mut cfg, 1, &mut n) } == 0 || n == 0 {
            return Err("no EGL config".into());
        }
        // EGL_CONTEXT_MAJOR_VERSION 0x3098, MINOR 0x30FB, EGL_CONTEXT_OPENGL_PROFILE_MASK 0x30FD = core 1
        let cattrs = [0x3098, 3, 0x30FB, 3, 0x30FD, 1, 0x3038];
        let ctx = unsafe { create_context(dpy, cfg, std::ptr::null_mut(), cattrs.as_ptr()) };
        if ctx.is_null() {
            return Err("eglCreateContext failed".into());
        }
        if unsafe { make_current(dpy, std::ptr::null_mut(), std::ptr::null_mut(), ctx) } == 0 {
            return Err("eglMakeCurrent failed".into());
        }
        Ok(Egl { get_proc })
    }
}
