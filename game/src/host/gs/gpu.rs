//! The GS on the graphics card (BT3_GS=gpu: a window; BT3_GS=gpu-headless: an offscreen context for tests).
//!
//! The front end (mod.rs) still decodes the display list and keeps GS memory; this module takes over at "a
//! primitive with the current GS registers": every primitive of the frame is recorded as a Draw (target, texture,
//! pipeline state), GS memory is decoded into ordinary RGBA textures (cached by TEX0 / TEXA and the content of
//! the pages they occupy), and at the end of the frame the list is replayed with OpenGL 3.3 into one render target
//! per GS frame-buffer address, at SCALE times the PS2's resolution. One buffer drawn into another (the game's
//! screen effects) samples the target directly.
//! Rust version of the C port's GPU back end (port/src/gs/gs_draw.c capture, gs_gl.c replay, shaders/*), with its
//! widescreen layout (see `draw`); without the texture packs and the settings window. Drawing, not simulation.
#![allow(clippy::too_many_arguments)]
use super::gl::{self, Gl, GLenum, GLuint};
use super::platform::{Input, Surface};
use super::Gs;
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, Ordering};

/// The menus are running (host/harness.rs sets it around Progress_Main): their pages are composed for 4:3 (2D and
/// 3D placed against each other, backdrops drawn to the 4:3 edges), so in widescreen they keep 4:3: the projection
/// is not widened (Port_WideFactor), 2D is not narrowed, and the page is shown in the middle over a blurred copy
/// of itself (`present`). (Laid out wide, the pages showed black bands and cut-off art at the sides.)
pub static MENU_MODE: AtomicBool = AtomicBool::new(false);

pub const GS_W: i32 = 1024;
pub const GS_H: i32 = 1024;
const MAX_VERTS: usize = 1 << 20;
const MAX_DRAWS: usize = 1 << 16;
const MAX_TARGETS: usize = 24;
const MAX_VU_VERTS: usize = 1 << 19;
const MAX_VU_IDX: usize = 3 << 19;
const MAX_VU_UNIFORMS: usize = 1 << 14;

/// The vertex programs' constants as the shaders take them (vu0.vert's uniform block U; std140, all vec4 / mat4).
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
struct VuUniform {
    bone_a: [f32; 16],
    bone_b: [f32; 16],
    pivot_a: [f32; 4],
    pivot_b: [f32; 4],
    screen: [f32; 16],
    light: [f32; 4],
    color0: [f32; 4],
    color1: [f32; 4],
    misc: [f32; 4],
    light2: [f32; 4],
}

impl Default for VuUniform {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

const CC_R: u32 = 1;
const CC_G: u32 = 2;
const CC_B: u32 = 4;
const CC_A: u32 = 8;

#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct Vtx {
    x: f32,
    y: f32,
    z: f32,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
    s: f32,
    t: f32,
    q: f32,
}

#[derive(Clone, Copy, Default, PartialEq)]
struct Draw {
    /// 0 primitives; 1 outline; 3 table pass (dclut); 4 copy of the alpha bytes (aux); 5 DATE copy;
    /// 100 / 101 a picture uploaded into display buffer 0 / 1
    native: i32,
    /// 0 a GS primitive; 1 / 2 / 3 a batch of vertex program 0 (and its relatives) / 4 / 6, drawn by its shader
    vu: u32,
    uniform: u32,
    src: i32,
    first: u32,
    count: u32,
    target: usize,
    tex: GLuint,
    tex_is_target: bool,
    sampler: i32,
    pipeline: usize,
    mode: [i32; 4],
    misc: [f32; 4],
    rect: [f32; 4],
    /// a texture pack's replacement: the original's size, its largest alpha (1.0 = 0x80), the pack's alpha scale
    orig: [f32; 4],
    blendc: f32,
    scissor: [i32; 4],
}

struct Target {
    fbp: u32,
    cleared: bool,
    draws: u32,
    gen: u32,
    stale: bool,
    last: u32,
    col: GLuint,
    aux: GLuint,
    dep: GLuint,
    fbo: GLuint,
}

#[derive(Clone, Copy)]
struct Pipe {
    key: u32,
    bkey: u32,
    wmask: u32,
    ztst: u32,
    zwrite: bool,
    topo: u32,
    vu: u32,
    fx: bool,
}

struct TexEntry {
    tex: GLuint,
    last: u32,
    /// a texture pack's replacement (owned by `repl`): original size, largest alpha, the pack's alpha scale
    repl: Option<(u32, u32, u32, f32)>,
    /// the replacement asked for (its job key), and the original's size and largest alpha
    want: Option<(u64, u32, u32, u32)>,
}

/// A texture pack's replacement on the card.
struct Repl {
    tex: GLuint,
    alpha_scale: f32,
}

pub struct Gpu {
    gl: Gl,
    pub surface: Surface,
    scale: i32,
    /// the HUD's side for the 2D that follows (markers 0x10..0x14 from the HUD code): 0 none, 1 left, 2 right,
    /// 3 centre, 4 by position
    anchor: i32,
    /// the settings' change counter last applied, and the resolution / shape the window was sized for
    settings_serial: Option<u32>,
    window_for: (i32, i32),
    draws: Vec<Draw>,
    verts: Vec<Vtx>,
    targets: Vec<Target>,
    textures: HashMap<(u64, u64, u32), TexEntry>,
    last_tex: Option<((u64, u64, u32), GLuint)>,
    /// texture packs (host/texpack.rs): the replacements on the card by job key, the jobs asked for, the
    /// quality they are for, and what the last texture_get returned (original size, largest alpha)
    repl: HashMap<u64, Repl>,
    pending: std::collections::HashSet<u64>,
    tex_quality: i32,
    cur_repl: Option<(u32, u32, u32, f32)>,
    cluts: Vec<(u32, u32, GLuint)>,
    clut_memo: HashMap<u32, (u32, u32, u32)>,
    pipes: Vec<Pipe>,
    prog_main: [GLuint; 4],
    prog_dclut: GLuint,
    prog_outline: GLuint,
    prog_present: GLuint,
    prog_backdrop: GLuint,
    prog_overlay: GLuint,
    overlay_tex: std::cell::Cell<GLuint>,
    overlay_serial: std::cell::Cell<u64>,
    vao: GLuint,
    vbo: GLuint,
    ubo: GLuint,
    samplers: [GLuint; 16],
    white: GLuint,
    date_copy: GLuint,
    aux_copy: GLuint,
    scratch_read: GLuint,
    scratch_draw: GLuint,
    fb_tex: [GLuint; 2],
    fb_fbo: [GLuint; 2],
    pub fb_uploads: u32,
    depth_byte_is_fog: bool,
    frame: u32,
    clip_control: bool,
    shot_every: u32,
    pub input: Input,
    vu_verts: Vec<[f32; 12]>,
    vu_idx: Vec<u32>,
    vu_uni: Vec<VuUniform>,
    vao_vu: GLuint,
    vbo_vu: GLuint,
    ibo_vu: GLuint,
    ubo_u: GLuint,
    verbose: bool,
    skipped: u32,
    /// The window keeps the PS2's pace: one frame every 1/60 s at most.
    next: std::time::Instant,
    no_replay: bool,
}

// The C port's shaders, unchanged (Vulkan GLSL 450), turned into GL 3.3 core by `translate`.
const GS_VERT: &str = include_str!("shaders/gs.vert");
const GS_FRAG: &str = include_str!("shaders/gs.frag");
const FX_VERT: &str = include_str!("shaders/fx.vert");
const DCLUT_FRAG: &str = include_str!("shaders/dclut.frag");
const OUTLINE_FRAG: &str = include_str!("shaders/outline.frag");
const VU0_VERT: &str = include_str!("shaders/vu0.vert");
const VU4_VERT: &str = include_str!("shaders/vu4.vert");
const VU6_VERT: &str = include_str!("shaders/vu6.vert");

/// Vulkan GLSL 450 to GL 3.3 core (the rules of the C port's gs_gl.c): the version line, layout(set ...) dropped
/// from uniforms and samplers, gl_VertexIndex -> gl_VertexID, layout(location) kept on vertex inputs and fragment
/// outputs only; in a vertex shader gl_Position.y is flipped (SDL GPU's +Y is the top of the target, GL's the
/// bottom) and, without glClipControl, z moved from 0..w to -w..w.
fn translate(src: &str, vertex: bool, clip_control: bool) -> String {
    let mut out = String::with_capacity(src.len() + 128);
    let mut p = src;
    while !p.is_empty() {
        if let Some(r) = p.strip_prefix("#version 450") {
            out += "#version 330 core";
            p = r;
            continue;
        }
        if let Some(r) = p.strip_prefix("gl_VertexIndex") {
            out += "gl_VertexID";
            p = r;
            continue;
        }
        if p.starts_with("layout(set") {
            if let Some(e) = p.find(')') {
                p = &p[e + 1..];
                p = p.strip_prefix(' ').unwrap_or(p);
                continue;
            }
        }
        if p.starts_with("layout(location") {
            if let Some(e) = p.find(')') {
                let q = p[e + 1..].trim_start_matches([' ', '\t']);
                let is_in = q.starts_with("in ");
                let is_out = q.starts_with("out ");
                let bad = (is_in || is_out) && if vertex { is_out } else { is_in };
                if bad {
                    p = q;
                    continue;
                }
                out += &p[..e + 1];
                p = &p[e + 1..];
                continue;
            }
        }
        let c = p.chars().next().unwrap();
        out.push(c);
        p = &p[c.len_utf8()..];
    }
    if vertex {
        if let Some(m) = out.find("void main") {
            out.insert_str(m + 5, "gs_");
        }
        out += if clip_control {
            "\nvoid main() { gs_main(); gl_Position.y = -gl_Position.y; }\n"
        } else {
            "\nvoid main() { gs_main(); gl_Position.y = -gl_Position.y; gl_Position.z = 2.0 * gl_Position.z - gl_Position.w; }\n"
        };
    }
    out
}

const PRESENT_VERT: &str = r#"#version 330 core
out vec2 vUv;
void main() {
    vec2 p = vec2(gl_VertexID == 1 ? 3.0 : -1.0, gl_VertexID == 2 ? 3.0 : -1.0);
    vUv = vec2((p.x + 1.0) * 0.5, (1.0 - p.y) * 0.5);
    gl_Position = vec4(p, 0.0, 1.0);
}
"#;

const PRESENT_FRAG: &str = r#"#version 330 core
in vec2 vUv;
out vec4 o;
uniform sampler2D tex;
uniform vec2 uvScale;
void main() { o = vec4(texture(tex, vUv * uvScale).rgb, 1.0); }
"#;

/// Behind a 4:3 menu page in a wider window: the page itself stretched to the window, blurred and darkened.
const BACKDROP_FRAG: &str = r#"#version 330 core
in vec2 vUv;
out vec4 o;
uniform sampler2D tex;
uniform vec2 uvScale;
void main() {
    vec3 c = vec3(0.0);
    for (int y = -3; y <= 3; y++) {
        for (int x = -3; x <= 3; x++) {
            c += texture(tex, (vUv + vec2(float(x), float(y)) * 0.018) * uvScale).rgb;
        }
    }
    o = vec4(c / 49.0 * 0.42, 1.0);
}
"#;

/// The port's own screens over the picture (host/ui.rs): the canvas as it is, with its alpha.
const OVERLAY_FRAG: &str = r#"#version 330 core
in vec2 vUv;
out vec4 o;
uniform sampler2D tex;
void main() { o = texture(tex, vUv); }
"#;

impl Gpu {
    pub fn new(headless: bool) -> Result<Box<Gpu>, String> {
        let surface = Surface::create(headless)?;
        let gl = Gl::load(&|n| surface.proc_address(n))?;
        let scale = crate::host::settings::get().scale();
        let clip = surface.proc_address("glClipControl");
        let clip_control = !clip.is_null();
        let mut g = Box::new(Gpu {
            gl,
            surface,
            scale,
            anchor: 0,
            settings_serial: None,
            window_for: (0, 0),
            draws: Vec::with_capacity(4096),
            verts: Vec::with_capacity(65536),
            targets: Vec::new(),
            textures: HashMap::new(),
            last_tex: None,
            repl: HashMap::new(),
            pending: std::collections::HashSet::new(),
            tex_quality: crate::host::settings::get().textures,
            cur_repl: None,
            cluts: Vec::new(),
            clut_memo: HashMap::new(),
            pipes: Vec::new(),
            prog_main: [0; 4],
            prog_dclut: 0,
            prog_outline: 0,
            prog_present: 0,
            prog_backdrop: 0,
            prog_overlay: 0,
            overlay_tex: std::cell::Cell::new(0),
            overlay_serial: std::cell::Cell::new(u64::MAX),
            vao: 0,
            vbo: 0,
            ubo: 0,
            samplers: [0; 16],
            white: 0,
            date_copy: 0,
            aux_copy: 0,
            scratch_read: 0,
            scratch_draw: 0,
            fb_tex: [0; 2],
            fb_fbo: [0; 2],
            fb_uploads: 0,
            depth_byte_is_fog: false,
            frame: 0,
            clip_control,
            shot_every: std::env::var("BT3_SHOT").ok().and_then(|v| v.parse().ok()).unwrap_or(0),
            input: Input { lx: 128, ly: 128, rx: 128, ry: 128, ..Default::default() },
            vu_verts: Vec::with_capacity(65536),
            vu_idx: Vec::with_capacity(65536 * 3),
            vu_uni: Vec::with_capacity(4096),
            vao_vu: 0,
            vbo_vu: 0,
            ibo_vu: 0,
            ubo_u: 0,
            verbose: std::env::var("BT3_GS_VERBOSE").is_ok(),
            skipped: 0,
            next: std::time::Instant::now(),
            no_replay: std::env::var("BT3_GPU_NOREPLAY").is_ok(),
        });
        if clip_control {
            let f: unsafe extern "system" fn(GLenum, GLenum) = unsafe { std::mem::transmute(clip) };
            unsafe { f(0x8CA1, 0x935F) }; // GL_LOWER_LEFT, GL_ZERO_TO_ONE
        }
        g.init_gl()?;
        Ok(g)
    }

    fn compile(&self, stage: GLenum, src: &str, tag: &str) -> Result<GLuint, String> {
        let gl = &self.gl;
        unsafe {
            let sh = (gl.CreateShader)(stage);
            let p = src.as_ptr();
            let len = src.len() as i32;
            (gl.ShaderSource)(sh, 1, &p, &len);
            (gl.CompileShader)(sh);
            let mut ok = 0;
            (gl.GetShaderiv)(sh, gl::COMPILE_STATUS, &mut ok);
            if ok == 0 {
                let mut n = 0;
                (gl.GetShaderiv)(sh, gl::INFO_LOG_LENGTH, &mut n);
                let mut log = vec![0u8; n.max(1) as usize];
                (gl.GetShaderInfoLog)(sh, n, std::ptr::null_mut(), log.as_mut_ptr());
                return Err(format!("{tag}: shader compile failed: {}", String::from_utf8_lossy(&log)));
            }
            Ok(sh)
        }
    }

    fn program(&self, vs: &str, fs: &str, tag: &str, samplers: &[&str]) -> Result<GLuint, String> {
        let gl = &self.gl;
        let v = self.compile(gl::VERTEX_SHADER, vs, tag)?;
        let f = self.compile(gl::FRAGMENT_SHADER, fs, tag)?;

        unsafe {
            let p = (gl.CreateProgram)();
            (gl.AttachShader)(p, v);
            (gl.AttachShader)(p, f);
            (gl.LinkProgram)(p);
            let mut ok = 0;
            (gl.GetProgramiv)(p, gl::LINK_STATUS, &mut ok);
            if ok == 0 {
                let mut n = 0;
                (gl.GetProgramiv)(p, gl::INFO_LOG_LENGTH, &mut n);
                let mut log = vec![0u8; n.max(1) as usize];
                (gl.GetProgramInfoLog)(p, n, std::ptr::null_mut(), log.as_mut_ptr());
                return Err(format!("{tag}: link failed: {}", String::from_utf8_lossy(&log)));
            }
            (gl.DeleteShader)(v);
            (gl.DeleteShader)(f);
            (gl.UseProgram)(p);
            for (i, s) in samplers.iter().enumerate() {
                let n = format!("{s}\0");
                (gl.Uniform1i)((gl.GetUniformLocation)(p, n.as_ptr()), i as i32);
            }
            let idx = (gl.GetUniformBlockIndex)(p, b"U\0".as_ptr());
            if idx != gl::INVALID_INDEX {
                (gl.UniformBlockBinding)(p, idx, 0);
            }
            let idx = (gl.GetUniformBlockIndex)(p, b"Params\0".as_ptr());
            if idx != gl::INVALID_INDEX {
                (gl.UniformBlockBinding)(p, idx, 1);
            }
            Ok(p)
        }
    }

    fn init_gl(&mut self) -> Result<(), String> {
        let cc = self.clip_control;
        let fs = translate(GS_FRAG, false, cc);
        self.prog_main[0] = self.program(&translate(GS_VERT, true, cc), &fs, "gs", &["tex", "dateTex"])?;
        self.prog_main[1] = self.program(&translate(VU0_VERT, true, cc), &fs, "vu0", &["tex", "dateTex"])?;
        self.prog_main[2] = self.program(&translate(VU4_VERT, true, cc), &fs, "vu4", &["tex", "dateTex"])?;
        self.prog_main[3] = self.program(&translate(VU6_VERT, true, cc), &fs, "vu6", &["tex", "dateTex"])?;
        let fx = translate(FX_VERT, true, cc);
        self.prog_dclut = self.program(&fx, &translate(DCLUT_FRAG, false, cc), "dclut", &["depthTex", "auxTex", "clutTex"])?;
        self.prog_outline = self.program(&fx, &translate(OUTLINE_FRAG, false, cc), "outline", &["aux"])?;
        // (the overlay's program first: the uniform below is set on the program made last, which is in use)
        self.prog_overlay = self.program(PRESENT_VERT, OVERLAY_FRAG, "overlay", &["tex"])?;
        self.prog_present = self.program(PRESENT_VERT, PRESENT_FRAG, "present", &["tex"])?;
        self.prog_backdrop = self.program(PRESENT_VERT, BACKDROP_FRAG, "backdrop", &["tex"])?;
        let gl = &self.gl;
        unsafe {
            let name = std::ffi::CStr::from_ptr((gl.GetString)(gl::RENDERER) as *const i8).to_string_lossy().into_owned();
            let ver = std::ffi::CStr::from_ptr((gl.GetString)(gl::VERSION) as *const i8).to_string_lossy().into_owned();
            eprintln!("bt3: GL renderer: {name} ({ver})");
            // (a uniform is set on the program in use: program() leaves the last one made in use)
            (gl.UseProgram)(self.prog_present);
            (gl.Uniform2f)((gl.GetUniformLocation)(self.prog_present, b"uvScale\0".as_ptr()), 512.0 / GS_W as f32, 448.0 / GS_H as f32);
            (gl.UseProgram)(self.prog_backdrop);
            (gl.Uniform2f)((gl.GetUniformLocation)(self.prog_backdrop, b"uvScale\0".as_ptr()), 512.0 / GS_W as f32, 448.0 / GS_H as f32);
            (gl.GenBuffers)(1, &mut self.vbo);
            (gl.BindBuffer)(gl::ARRAY_BUFFER, self.vbo);
            (gl.BufferData)(gl::ARRAY_BUFFER, (MAX_VERTS * std::mem::size_of::<Vtx>()) as isize, std::ptr::null(), gl::STREAM_DRAW);
            (gl.GenVertexArrays)(1, &mut self.vao);
            (gl.BindVertexArray)(self.vao);
            let st = std::mem::size_of::<Vtx>() as i32;
            (gl.EnableVertexAttribArray)(0);
            (gl.VertexAttribPointer)(0, 3, gl::FLOAT, 0, st, std::ptr::null());
            (gl.EnableVertexAttribArray)(1);
            (gl.VertexAttribPointer)(1, 4, gl::UNSIGNED_BYTE, 1, st, 12 as *const c_void);
            (gl.EnableVertexAttribArray)(2);
            (gl.VertexAttribPointer)(2, 3, gl::FLOAT, 0, st, 16 as *const c_void);
            (gl.GenBuffers)(1, &mut self.vbo_vu);
            (gl.BindBuffer)(gl::ARRAY_BUFFER, self.vbo_vu);
            (gl.BufferData)(gl::ARRAY_BUFFER, (MAX_VU_VERTS * 48) as isize, std::ptr::null(), gl::STREAM_DRAW);
            (gl.GenVertexArrays)(1, &mut self.vao_vu);
            (gl.BindVertexArray)(self.vao_vu);
            for i in 0..3u32 {
                (gl.EnableVertexAttribArray)(i);
                (gl.VertexAttribPointer)(i, 4, gl::FLOAT, 0, 48, (i as usize * 16) as *const c_void);
            }
            (gl.GenBuffers)(1, &mut self.ibo_vu);
            (gl.BindBuffer)(gl::ELEMENT_ARRAY_BUFFER, self.ibo_vu);
            (gl.BufferData)(gl::ELEMENT_ARRAY_BUFFER, (MAX_VU_IDX * 4) as isize, std::ptr::null(), gl::STREAM_DRAW);
            (gl.BindVertexArray)(0);
            (gl.GenBuffers)(1, &mut self.ubo_u);
            (gl.BindBuffer)(gl::UNIFORM_BUFFER, self.ubo_u);
            (gl.BufferData)(gl::UNIFORM_BUFFER, std::mem::size_of::<VuUniform>() as isize, std::ptr::null(), gl::DYNAMIC_DRAW);
            (gl.BindBufferBase)(gl::UNIFORM_BUFFER, 0, self.ubo_u);
            (gl.GenBuffers)(1, &mut self.ubo);
            (gl.BindBuffer)(gl::UNIFORM_BUFFER, self.ubo);
            (gl.BufferData)(gl::UNIFORM_BUFFER, 64, std::ptr::null(), gl::DYNAMIC_DRAW);
            (gl.BindBufferBase)(gl::UNIFORM_BUFFER, 1, self.ubo);
            for i in 0..16 {
                let lin = i & 1 != 0;
                (gl.GenSamplers)(1, &mut self.samplers[i]);
                let s = self.samplers[i];
                (gl.SamplerParameteri)(s, gl::TEXTURE_MAG_FILTER, if lin { gl::LINEAR } else { gl::NEAREST });
                // 8..15: the replacements of a texture pack on 3D, with their smaller copies
                let min = if i & 8 != 0 { gl::LINEAR_MIPMAP_LINEAR } else if lin { gl::LINEAR } else { gl::NEAREST };
                (gl.SamplerParameteri)(s, gl::TEXTURE_MIN_FILTER, min);
                (gl.SamplerParameteri)(s, gl::TEXTURE_WRAP_S, if i & 2 != 0 { gl::CLAMP_TO_EDGE } else { gl::REPEAT });
                (gl.SamplerParameteri)(s, gl::TEXTURE_WRAP_T, if i & 4 != 0 { gl::CLAMP_TO_EDGE } else { gl::REPEAT });
            }
            let white = 0xFFFF_FFFFu32;
            (gl.GenTextures)(1, &mut self.white);
            (gl.BindTexture)(gl::TEXTURE_2D, self.white);
            (gl.PixelStorei)(gl::UNPACK_ALIGNMENT, 1);
            (gl.TexImage2D)(gl::TEXTURE_2D, 0, gl::RGBA8 as i32, 1, 1, 0, gl::RGBA, gl::UNSIGNED_BYTE, &white as *const u32 as *const c_void);
            (gl.GenFramebuffers)(1, &mut self.scratch_read);
            (gl.GenFramebuffers)(1, &mut self.scratch_draw);
            (gl.Disable)(gl::CULL_FACE);
            (gl.Enable)(gl::DEPTH_CLAMP);
            (gl.Enable)(gl::SCISSOR_TEST);
            (gl.PixelStorei)(gl::PACK_ALIGNMENT, 1);
        }
        self.copies_create();
        Ok(())
    }

    fn r8_texture(&self, w: i32, h: i32) -> GLuint {
        let gl = &self.gl;
        let mut t = 0;
        unsafe {
            (gl.GenTextures)(1, &mut t);
            (gl.BindTexture)(gl::TEXTURE_2D, t);
            (gl.TexImage2D)(gl::TEXTURE_2D, 0, gl::R8 as i32, w, h, 0, gl::RED, gl::UNSIGNED_BYTE, std::ptr::null());
        }
        t
    }

    fn copies_create(&mut self) {
        self.date_copy = self.r8_texture(GS_W * self.scale, GS_H * self.scale);
        self.aux_copy = self.r8_texture(GS_W * self.scale, GS_H * self.scale);
    }

    // ------------------------------------------------------------------------------------------ targets
    fn target_ensure(&mut self, i: usize) {
        let (w, h) = (GS_W * self.scale, GS_H * self.scale);
        let gl = &self.gl;
        let t = &mut self.targets[i];
        unsafe {
            (gl.GenTextures)(1, &mut t.col);
            (gl.BindTexture)(gl::TEXTURE_2D, t.col);
            (gl.TexImage2D)(gl::TEXTURE_2D, 0, gl::RGBA8 as i32, w, h, 0, gl::RGBA, gl::UNSIGNED_BYTE, std::ptr::null());
            (gl.GenTextures)(1, &mut t.aux);
            (gl.BindTexture)(gl::TEXTURE_2D, t.aux);
            (gl.TexImage2D)(gl::TEXTURE_2D, 0, gl::R8 as i32, w, h, 0, gl::RED, gl::UNSIGNED_BYTE, std::ptr::null());
            (gl.GenTextures)(1, &mut t.dep);
            (gl.BindTexture)(gl::TEXTURE_2D, t.dep);
            (gl.TexImage2D)(gl::TEXTURE_2D, 0, gl::DEPTH_COMPONENT32F as i32, w, h, 0, gl::DEPTH_COMPONENT, gl::FLOAT, std::ptr::null());
            (gl.TexParameteri)(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST);
            (gl.TexParameteri)(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST);
            (gl.GenFramebuffers)(1, &mut t.fbo);
            (gl.BindFramebuffer)(gl::FRAMEBUFFER, t.fbo);
            (gl.FramebufferTexture2D)(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, t.col, 0);
            (gl.FramebufferTexture2D)(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT1, gl::TEXTURE_2D, t.aux, 0);
            (gl.FramebufferTexture2D)(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::TEXTURE_2D, t.dep, 0);
            let bufs = [gl::COLOR_ATTACHMENT0, gl::COLOR_ATTACHMENT1];
            (gl.DrawBuffers)(2, bufs.as_ptr());
            if (gl.CheckFramebufferStatus)(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                eprintln!("bt3: gl: target {i} framebuffer incomplete");
            }
        }
    }

    fn target_drop(&mut self, i: usize) {
        let gl = &self.gl;
        let t = &mut self.targets[i];
        unsafe {
            (gl.DeleteTextures)(1, &t.col);
            (gl.DeleteTextures)(1, &t.aux);
            (gl.DeleteTextures)(1, &t.dep);
            (gl.DeleteFramebuffers)(1, &t.fbo);
        }
    }

    fn target_get(&mut self, fbp: u32, create: bool) -> Option<usize> {
        let frame = self.frame;
        if let Some(i) = self.targets.iter().position(|t| t.fbp == fbp) {
            self.targets[i].last = frame;
            return Some(i);
        }
        if !create {
            return None;
        }
        let slot = if self.targets.len() == MAX_TARGETS {
            let mut slot = None;
            for (i, t) in self.targets.iter().enumerate() {
                if t.last + 120 < frame && slot.map_or(true, |s: usize| t.last < self.targets[s].last) {
                    slot = Some(i);
                }
            }
            let s = slot?;
            self.target_drop(s);
            s
        } else {
            self.targets.push(Target { fbp, cleared: false, draws: 0, gen: 0, stale: false, last: frame, col: 0, aux: 0, dep: 0, fbo: 0 });
            self.targets.len() - 1
        };
        let t = &mut self.targets[slot];
        *t = Target { fbp, cleared: false, draws: 0, gen: 0, stale: false, last: frame, col: 0, aux: 0, dep: 0, fbo: 0 };
        self.target_ensure(slot);
        Some(slot)
    }

    // ------------------------------------------------------------------------------------------ textures
    fn clut_hash(&mut self, gs: &mut Gs, cbp: u32, cpsm: u32, count: u32) -> u32 {
        let key = cbp << 5 | cpsm << 1 | (count == 256) as u32;
        let g0 = gs.page_gen[((cbp / 32) & 511) as usize];
        let g1 = gs.page_gen[((cbp / 32 + 1) & 511) as usize];
        if let Some(&(a, b, h)) = self.clut_memo.get(&key) {
            if a == g0 && b == g1 {
                return h;
            }
        }
        let mut h = 2166136261u32;
        for i in 0..count {
            h = (h ^ gs.rd(cbp, 1, cpsm, i & 15, i >> 4)).wrapping_mul(16777619);
        }
        self.clut_memo.insert(key, (g0, g1, h));
        h
    }

    fn texture_get(&mut self, gs: &mut Gs, ctx: usize) -> GLuint {
        let t0 = gs.s.tex0[ctx] & 0x1FFF_FFFF_FFFF_FFFF;
        let texa = gs.s.texa;
        let tbp = (t0 & 0x3FFF) as u32;
        let tbw = ((t0 >> 14) & 0x3F) as u32;
        let psm = ((t0 >> 20) & 0x3F) as u32;
        let tw = (1u32 << ((t0 >> 26) & 15)).min(1024);
        let th = (1u32 << ((t0 >> 30) & 15)).min(1024);
        let cbp = ((t0 >> 37) & 0x3FFF) as u32;
        let cpsm = ((t0 >> 51) & 15) as u32;
        let bits = Gs::psm_bits(psm);
        let pages = ((if tbw != 0 { tbw } else { 1 }) * 64 * th * (if bits == 24 { 32 } else { bits }) / 8 + 8191) / 8192;
        let mut gen = 0u32;
        for i in 0..=pages {
            gen = (gen ^ gs.page_hash(tbp / 32 + i)).wrapping_mul(16777619).wrapping_add(i);
        }
        if bits <= 8 {
            gen = (gen ^ self.clut_hash(gs, cbp, cpsm, if bits == 8 { 256 } else { 16 })).wrapping_mul(16777619);
        }
        let key = (t0, texa, gen);
        if let Some((k, t)) = self.last_tex {
            if k == key {
                self.cur_repl = self.textures.get(&key).and_then(|e| e.repl);
                return t;
            }
        }
        let frame = self.frame;
        if let Some(e) = self.textures.get_mut(&key) {
            e.last = frame;
            self.cur_repl = e.repl;
            self.last_tex = Some((key, e.tex));
            return e.tex;
        }
        self.cur_repl = None;
        if self.textures.len() >= 8192 {
            let keep = frame.saturating_sub(120);
            let gl = &self.gl;
            self.textures.retain(|_, e| {
                if e.last >= keep {
                    true
                } else {
                    if e.repl.is_none() {
                        unsafe { (gl.DeleteTextures)(1, &e.tex) };
                    }
                    false
                }
            });
        }
        let mut px = vec![0u32; (tw * th) as usize];
        for y in 0..th {
            for x in 0..tw {
                let c = gs.rd(tbp, tbw, psm, x, y);
                px[(y * tw + x) as usize] = if bits > 8 {
                    gs.expand(c, psm)
                } else if bits == 8 {
                    let c = (c & 0xE7) | ((c & 8) << 1) | ((c & 0x10) >> 1);
                    let p = gs.rd(cbp, 1, cpsm, c & 15, c >> 4);
                    gs.expand(p, cpsm)
                } else {
                    let p = gs.rd(cbp, 1, cpsm, c & 7, c >> 3);
                    gs.expand(p, cpsm)
                };
            }
        }
        // a texture pack's replacement (host/texpack.rs): found by the texture's raw blocks and its palette
        let mut want = None;
        let mut ready = None;
        let pack = self.tex_quality > 0 && crate::host::texpack::available();
        if let (true, Some((bw, bh))) = (pack, crate::host::texpack::block_size(psm)) {
            if tw >= bw && th >= bh {
                let mut blocks = Vec::with_capacity(((tw / bw) * (th / bh) * 256) as usize);
                for by in (0..th).step_by(bh as usize) {
                    for bx in (0..tw).step_by(bw as usize) {
                        gs.block_into(tbp, tbw, psm, bx, by, &mut blocks);
                    }
                }
                let clut: Vec<u32> = match psm {
                    0x13 => (0..256u32)
                        .map(|i| {
                            let k = (i & 0xE7) | ((i & 8) << 1) | ((i & 0x10) >> 1);
                            gs.rd(cbp, 1, cpsm, k & 15, k >> 4)
                        })
                        .collect(),
                    0x14 => (0..16u32).map(|i| gs.rd(cbp, 1, cpsm, i & 7, i >> 3)).collect(),
                    _ => Vec::new(),
                };
                let q = crate::host::texpack::Query { psm, tw, th, tcc: ((t0 >> 34) & 1) as u32, cpsm, blocks: &blocks, clut: &clut };
                if let Some(found) = crate::host::texpack::lookup(self.tex_quality, &q) {
                    let amax = px.iter().map(|p| p >> 24).max().unwrap_or(0x80);
                    let job = found.key;
                    if let Some(r) = self.repl.get(&job) {
                        ready = Some((r.tex, (tw, th, amax, r.alpha_scale)));
                    } else {
                        if self.pending.insert(job) {
                            crate::host::texpack::request(crate::host::texpack::Job { found });
                        }
                        want = Some((job, tw, th, amax));
                    }
                }
            }
        }
        if let Some((t, info)) = ready {
            self.textures.insert(key, TexEntry { tex: t, last: frame, repl: Some(info), want: None });
            self.cur_repl = Some(info);
            self.last_tex = Some((key, t));
            return t;
        }
        let gl = &self.gl;
        let mut t = 0;
        unsafe {
            (gl.GenTextures)(1, &mut t);
            (gl.BindTexture)(gl::TEXTURE_2D, t);
            (gl.TexParameteri)(gl::TEXTURE_2D, gl::TEXTURE_MAX_LEVEL, 0);
            (gl.PixelStorei)(gl::UNPACK_ALIGNMENT, 1);
            (gl.TexImage2D)(gl::TEXTURE_2D, 0, gl::RGBA8 as i32, tw as i32, th as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, px.as_ptr() as *const c_void);
        }
        self.textures.insert(key, TexEntry { tex: t, last: frame, repl: None, want });
        self.last_tex = Some((key, t));
        t
    }

    /// The replacements decoded since the last frame put on the card, and the textures waiting for them switched
    /// over (between frames: no recorded draw refers to the textures now).
    fn texpack_collect(&mut self) {
        let done = crate::host::texpack::finished();
        if done.is_empty() {
            return;
        }
        for d in done {
            self.pending.remove(&d.key);
            let Some(pic) = d.picture else { continue };
            let gl = &self.gl;
            let mut t = 0;
            unsafe {
                (gl.GenTextures)(1, &mut t);
                (gl.BindTexture)(gl::TEXTURE_2D, t);
                (gl.PixelStorei)(gl::UNPACK_ALIGNMENT, 1);
                (gl.TexImage2D)(gl::TEXTURE_2D, 0, gl::RGBA8 as i32, pic.w as i32, pic.h as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, pic.rgba.as_ptr() as *const c_void);
                (gl.GenerateMipmap)(gl::TEXTURE_2D);
            }
            self.repl.insert(d.key, Repl { tex: t, alpha_scale: pic.alpha_scale });
            for e in self.textures.values_mut() {
                if let Some((job, ow, oh, amax)) = e.want {
                    if job == d.key {
                        unsafe { (gl.DeleteTextures)(1, &e.tex) };
                        e.tex = t;
                        e.repl = Some((ow, oh, amax, pic.alpha_scale));
                        e.want = None;
                    }
                }
            }
        }
        self.last_tex = None;
    }

    /// Every texture made again (the texture quality changed).
    fn textures_flush(&mut self) {
        let gl = &self.gl;
        for e in self.textures.values() {
            if e.repl.is_none() {
                unsafe { (gl.DeleteTextures)(1, &e.tex) };
            }
        }
        self.textures.clear();
        for r in self.repl.values() {
            unsafe { (gl.DeleteTextures)(1, &r.tex) };
        }
        self.repl.clear();
        self.pending.clear();
        self.last_tex = None;
    }

    /// A 256-colour table at cbp (32-bit, the GS's index order undone) as a 256 x 1 texture.
    fn clut_texture(&mut self, gs: &mut Gs, cbp: u32) -> GLuint {
        let mut px = [0u32; 256];
        let mut hash = 2166136261u32;
        for i in 0..256u32 {
            let n = (i & 0xE7) | ((i & 8) << 1) | ((i & 0x10) >> 1);
            px[i as usize] = gs.rd(cbp, 1, 0, n & 15, n >> 4);
            hash = (hash ^ px[i as usize]).wrapping_mul(16777619);
        }
        let frame = self.frame;
        if let Some(c) = self.cluts.iter_mut().find(|c| c.0 == hash) {
            c.1 = frame;
            return c.2;
        }
        let gl = &self.gl;
        let mut t = 0;
        unsafe {
            if self.cluts.len() >= 64 {
                let (i, _) = self.cluts.iter().enumerate().min_by_key(|(_, c)| c.1).unwrap();
                let old = self.cluts.remove(i);
                (gl.DeleteTextures)(1, &old.2);
            }
            (gl.GenTextures)(1, &mut t);
            (gl.BindTexture)(gl::TEXTURE_2D, t);
            (gl.TexParameteri)(gl::TEXTURE_2D, gl::TEXTURE_MAX_LEVEL, 0);
            (gl.PixelStorei)(gl::UNPACK_ALIGNMENT, 1);
            (gl.TexImage2D)(gl::TEXTURE_2D, 0, gl::RGBA8 as i32, 256, 1, 0, gl::RGBA, gl::UNSIGNED_BYTE, px.as_ptr() as *const c_void);
        }
        self.cluts.push((hash, frame, t));
        t
    }

    // ------------------------------------------------------------------------------------------ pipelines
    fn blend_key(alpha: u64, abe: bool) -> u32 {
        if !abe {
            return 0;
        }
        let (a, b, c, d) = ((alpha & 3) as i32, ((alpha >> 2) & 3) as i32, ((alpha >> 4) & 3) as u32, ((alpha >> 6) & 3) as i32);
        let mut k = [0u32; 2];
        for i in 0..2 {
            let cc = (a == i) as i32 - (b == i) as i32;
            let one = d == i;
            k[i as usize] = if cc == 0 {
                one as u32
            } else if cc > 0 {
                if one { 5 } else { 2 }
            } else if one {
                4
            } else {
                3
            };
        }
        1 | k[0] << 1 | k[1] << 4 | c << 7
    }

    fn pipeline_get(&mut self, gs: &Gs, ctx: usize, topo: u32, vu: u32) -> usize {
        let test = gs.s.test[ctx];
        let zb = gs.s.zbuf[ctx];
        let zte = (test >> 16) & 1 != 0;
        let mut ztst = ((test >> 17) & 3) as u32;
        let zwrite = (zb >> 32) & 1 == 0;
        let m = (gs.s.frame[ctx] >> 32) as u32;
        let bkey = Self::blend_key(gs.s.alpha[ctx], (gs.s.prim >> 6) & 1 != 0);
        if !zte {
            ztst = 1;
        }
        let wmask = if m & 0xFF != 0xFF { CC_R } else { 0 }
            | if m & 0xFF00 != 0xFF00 { CC_G } else { 0 }
            | if m & 0xFF_0000 != 0xFF_0000 { CC_B } else { 0 }
            | if m & 0xFF00_0000 != 0xFF00_0000 { CC_A } else { 0 };
        let key = if vu == 4 {
            bkey | wmask << 16 | 1 << 22 // the table pass (depth_clut): only blending and the write mask matter
        } else {
            bkey | ztst << 10 | (zwrite as u32) << 12 | topo << 13 | wmask << 16 | vu << 20
        };
        if let Some(i) = self.pipes.iter().position(|p| p.key == key) {
            return i;
        }
        self.pipes.push(Pipe {
            key,
            bkey: key & 0x1FF,
            ztst: (key >> 10) & 3,
            zwrite: (key >> 12) & 1 != 0,
            topo: (key >> 13) & 3,
            vu: (key >> 20) & 3,
            fx: (key >> 22) & 1 != 0,
            wmask: (key >> 16) & 15,
        });
        self.pipes.len() - 1
    }

    fn scissor_of(&self, sc: u64) -> [i32; 4] {
        let s = self.scale;
        let x0 = (sc & 0x7FF) as i32 * s;
        let y0 = ((sc >> 32) & 0x7FF) as i32 * s;
        [x0, y0, (((sc >> 16) & 0x7FF) as i32 + 1) * s - x0, (((sc >> 48) & 0x7FF) as i32 + 1) * s - y0]
    }

    // ------------------------------------------------------------------------------------------ capture
    /// GfxPost_DrawDepthClut as one full-screen pass (the game sends 16 strips; they become one draw).
    fn depth_clut(&mut self, gs: &mut Gs, ctx: usize) {
        if self.draws.len() >= MAX_DRAWS {
            return;
        }
        let Some(target) = self.target_get((gs.s.frame[ctx] & 0x1FF) as u32, false) else { return };
        let cbp = ((gs.s.tex0[ctx] >> 37) & 0x3FFF) as u32;
        if cbp == 0x3E8C {
            return; // the outline's own passes: the outline is drawn natively at its marker
        }
        let tex = self.clut_texture(gs, cbp);
        let mut d = Draw { native: 3, src: if self.depth_byte_is_fog { 0 } else { 1 }, target, tex, ..Default::default() };
        d.pipeline = self.pipeline_get(gs, ctx, 0, 4);
        d.blendc = ((gs.s.alpha[ctx] >> 32) & 0xFF) as f32 / 128.0;
        d.misc[0] = match (gs.s.zbuf[ctx] >> 24) & 15 {
            0 => 4294967295.0,
            1 => 16777215.0,
            _ => 65535.0,
        };
        d.scissor = self.scissor_of(gs.s.scissor[ctx]);
        if let Some(last) = self.draws.last() {
            if last.native == 3 && last.tex == d.tex && last.target == d.target && last.pipeline == d.pipeline && last.src == d.src {
                return;
            }
        }
        self.draws.push(d);
    }

    /// The state a draw records, or None for a pass that only makes sense on the PS2's memory layout.
    fn draw_state(&mut self, gs: &mut Gs, ctx: usize, topo: u32, sprite: bool, vu: u32) -> Option<(Draw, f32, f32)> {
        let prim = gs.s.prim;
        let t0 = gs.s.tex0[ctx];
        let test = gs.s.test[ctx];
        let cl = gs.s.clamp[ctx];
        let tme = (prim >> 4) & 1 != 0;
        let tw = (1u32 << ((t0 >> 26) & 15)) as f32;
        let th = (1u32 << ((t0 >> 30) & 15)) as f32;
        let (mut us, mut vs) = (1.0f32, 1.0f32);
        let fpsm = ((gs.s.frame[ctx] >> 24) & 0x3F) as u32;
        let fbp = (gs.s.frame[ctx] & 0x1FF) as u32;
        let zbp = (gs.s.zbuf[ctx] & 0x1FF) as u32;
        let tpsm = ((t0 >> 20) & 0x3F) as u32;
        let tbp = (t0 & 0x3FFF) as u32;
        let main_fbp = gs.main_fbp;
        if Gs::psm_bits(fpsm) != 16 && fbp == zbp && tme {
            self.depth_byte_is_fog = tpsm == 0x1B;
            if tpsm != 0x1B {
                if let Some(t) = self.target_get(tbp / 32, false) {
                    if self.draws.len() < MAX_DRAWS && !self.draws.last().map_or(false, |l| l.native == 4) {
                        self.draws.push(Draw { native: 4, target: t, ..Default::default() });
                    }
                }
            }
        }
        if Gs::psm_bits(fpsm) == 16 || fbp == zbp {
            if fbp != zbp && fbp as i32 != main_fbp {
                let gen = gs.page_gen[(fbp & 511) as usize];
                if let Some(t) = self.target_get(fbp, true) {
                    self.targets[t].stale = true;
                    self.targets[t].cleared = true;
                    self.targets[t].gen = gen;
                }
            }
            self.skipped += 1;
            return None;
        }
        if tme && sprite && tpsm == 0x1B && tbp / 32 == zbp {
            self.depth_clut(gs, ctx);
            self.skipped += 1;
            return None;
        }
        if tme && ((tpsm & 0x30) == 0x30 || tbp / 32 == zbp || tpsm == 0x1B || tpsm == 0x24 || tpsm == 0x2C) {
            if let Some(t) = self.target_get(fbp, false) {
                if fbp as i32 != main_fbp {
                    self.targets[t].stale = true;
                }
            }
            self.skipped += 1;
            return None;
        }
        let target = self.target_get(fbp, true)?;
        let mut d = Draw { target, tex: self.white, ..Default::default() };
        let mut replaced = false;
        if !tme && sprite {
            self.targets[target].stale = false;
        }
        if tme {
            let src = self.target_get(tbp / 32, false);
            let src_ok = src.map_or(false, |s| {
                (t0 & 0x1F) == 0 && self.targets[s].cleared && self.targets[s].gen == gs.page_gen[(self.targets[s].fbp & 511) as usize]
            });
            if let (Some(src), true) = (src, src_ok) {
                if self.targets[src].stale {
                    self.skipped += 1;
                    return None;
                }
                if src == target || tpsm > 1 {
                    if target != src && fbp as i32 != main_fbp {
                        self.targets[target].stale = true;
                    }
                    self.skipped += 1;
                    return None;
                }
                d.tex = self.targets[src].col;
                d.tex_is_target = true;
                let wms = cl & 3;
                let wmt = (cl >> 2) & 3;
                let (mut u0, mut u1, mut v0, mut v1) = (0.0f32, tw - 1.0, 0.0f32, th - 1.0);
                if wms >= 2 {
                    u0 = ((cl >> 4) & 0x3FF) as f32;
                    u1 = ((cl >> 14) & 0x3FF) as f32;
                }
                if wmt >= 2 {
                    v0 = ((cl >> 24) & 0x3FF) as f32;
                    v1 = ((cl >> 34) & 0x3FF) as f32;
                }
                d.rect = [(u0 + 0.5) / GS_W as f32, (v0 + 0.5) / GS_H as f32, (u1 + 0.5) / GS_W as f32, (v1 + 0.5) / GS_H as f32];
                us = tw / GS_W as f32;
                vs = th / GS_H as f32;
            } else {
                d.tex = self.texture_get(gs, ctx);
                if let Some((ow, oh, amax, ascale)) = self.cur_repl {
                    // a texture pack's replacement: always filtered (several texels per original texel), with
                    // its smaller copies on 3D (draw() takes them off again for 2D art)
                    replaced = true;
                    d.orig = [ow as f32, oh as f32, amax as f32 / 128.0, ascale];
                }
            }
        }
        d.sampler = ((gs.s.tex1[ctx] >> 5) & 1) as i32 | if cl & 3 != 0 { 2 } else { 0 } | if (cl >> 2) & 3 != 0 { 4 } else { 0 };
        if replaced {
            d.sampler |= 1 | 8;
        }
        d.pipeline = self.pipeline_get(gs, ctx, topo, vu);
        d.mode[0] = if !tme {
            0
        } else if d.tex_is_target {
            2
        } else if replaced {
            3
        } else {
            1
        };
        d.mode[1] = ((t0 >> 35) & 3) as i32;
        d.mode[2] = ((t0 >> 34) & 1) as i32;
        d.mode[3] = if test & 1 != 0 && (test >> 12) & 3 == 0 { ((test >> 1) & 7) as i32 + 1 } else { 0 };
        d.misc[0] = ((test >> 4) & 0xFF) as f32;
        d.misc[1] = if (test >> 14) & 1 != 0 { (1 + ((test >> 15) & 1)) as f32 } else { 0.0 };
        d.misc[2] = (gs.s.fba[ctx] & 1) as f32;
        d.blendc = ((gs.s.alpha[ctx] >> 32) & 0xFF) as f32 / 128.0;
        d.scissor = self.scissor_of(gs.s.scissor[ctx]);
        Some((d, us, vs))
    }

    fn date_snapshot(&mut self, d: &Draw) {
        if d.misc[1] == 0.0 {
            return;
        }
        if let Some(p) = self.draws.last() {
            if p.native == 0 && p.misc[1] != 0.0 && p.target == d.target {
                return;
            }
        }
        if self.draws.len() + 1 < MAX_DRAWS {
            self.draws.push(Draw { native: 5, target: d.target, ..Default::default() });
        }
    }

    fn same_state(a: &Draw, b: &Draw) -> bool {
        a.native == 0
            && a.vu == b.vu
            && a.target == b.target
            && a.tex == b.tex
            && a.sampler == b.sampler
            && a.pipeline == b.pipeline
            && a.mode == b.mode
            && a.misc == b.misc
            && a.blendc == b.blendc
            && a.rect == b.rect
            && a.orig == b.orig
            && a.scissor == b.scissor
    }

    /// A primitive of type 0 (point), 1 (line), 3 (triangle) or 6 (sprite) with its vertices.
    pub fn draw(&mut self, gs: &mut Gs, ty: u32, ctx: usize, v: &[super::Vertex]) {
        let prim = gs.s.prim;
        let t0 = gs.s.tex0[ctx];
        let fst = (prim >> 8) & 1 != 0;
        let gouraud = (prim >> 3) & 1 != 0;
        let n = match ty {
            6 => 2,
            3 => 3,
            1 => 2,
            _ => 1,
        };
        let tw = (1u32 << ((t0 >> 26) & 15)) as f32;
        let th = (1u32 << ((t0 >> 30) & 15)) as f32;
        let zmax = match (gs.s.zbuf[ctx] >> 24) & 15 {
            0 => 4294967295.0f64,
            1 => 16777215.0,
            _ => 65535.0,
        };
        if self.verts.len() + 6 > MAX_VERTS || self.draws.len() >= MAX_DRAWS {
            return;
        }
        let topo = match ty {
            1 => 1,
            0 => 2,
            _ => 0,
        };
        let Some((mut d, us, vs)) = self.draw_state(gs, ctx, topo, ty == 6, 0) else { return };
        let packed2d = (ty == 6 || fst) && d.mode[0] == 3;
        if ty == 6 || fst {
            d.sampler &= 7;
            if !d.tex_is_target && !packed2d {
                d.misc[3] = self.scale as f32;
            }
        }
        self.date_snapshot(&d);
        let mut s = [0f32; 3];
        let mut t = [0f32; 3];
        let mut q = [0f32; 3];
        for i in 0..n {
            if fst {
                s[i] = v[i].u as f32 / 16.0 / tw * us;
                t[i] = v[i].v as f32 / 16.0 / th * vs;
                q[i] = 1.0;
            } else {
                s[i] = v[i].s * us;
                t[i] = v[i].t * vs;
                q[i] = if v[i].q != 0.0 { v[i].q } else { 1.0 };
            }
        }
        if packed2d {
            // A texture pack's replacement as 2D art (the C port's GsGpu_Draw): sampled per output pixel (it has
            // several texels per original texel), but only inside the rectangle of the sheet this piece shows on
            // the console: the original texels from the one its first GS pixel takes to the one its last takes.
            // Without the limit the filtering reaches the neighbouring picture of the sheet.
            let (mut lo, mut hi) = ([0i32; 2], [0i32; 2]);
            for axis in 0..2 {
                let size = if axis == 1 { th } else { tw };
                let (mut pmin, mut pmax, mut cmin, mut cmax, mut c_at_min, mut c_at_max) = (0f32, 0f32, 0f32, 0f32, 0f32, 0f32);
                for k in 0..n {
                    let pos = if axis == 1 { v[k].y } else { v[k].x };
                    let c = (if axis == 1 { t[k] } else { s[k] }) / (if ty == 6 { q[1] } else { q[k] }) * size;
                    if k == 0 || pos < pmin {
                        pmin = pos;
                        c_at_min = c;
                    }
                    if k == 0 || pos > pmax {
                        pmax = pos;
                        c_at_max = c;
                    }
                    if k == 0 || c < cmin {
                        cmin = c;
                    }
                    if k == 0 || c > cmax {
                        cmax = c;
                    }
                }
                let step = if pmax > pmin { (cmax - cmin) / (pmax - pmin) } else { 0.0 };
                let rising = c_at_max >= c_at_min;
                let first = if rising { cmin } else { cmin + step };
                let lastc = if rising { cmax - step } else { cmax };
                lo[axis] = (first + 1.0 / 64.0).floor() as i32;
                hi[axis] = (lastc + if rising { 1.0 / 64.0 } else { -1.0 / 64.0 }).floor() as i32 + 1;
                if hi[axis] <= lo[axis] {
                    hi[axis] = lo[axis] + 1;
                }
            }
            d.misc[3] = -(self.scale as f32);
            d.rect = [lo[0] as f32 / tw, lo[1] as f32 / th, hi[0] as f32 / tw, hi[1] as f32 / th];
        }
        d.first = self.verts.len() as u32;
        let put = |o: &mut Vec<Vtx>, z: u32, x: f32, y: f32, s: f32, t: f32, q: f32, c: &super::Vertex| {
            o.push(Vtx { x, y, z: (z as f64 / zmax) as f32, r: c.r, g: c.g, b: c.b, a: c.a, s, t, q });
        };
        if ty == 6 {
            let (a, b) = (&v[0], &v[1]);
            put(&mut self.verts, b.z, a.x, a.y, s[0], t[0], 1.0, b);
            put(&mut self.verts, b.z, b.x, a.y, s[1], t[0], 1.0, b);
            put(&mut self.verts, b.z, a.x, b.y, s[0], t[1], 1.0, b);
            put(&mut self.verts, b.z, b.x, a.y, s[1], t[0], 1.0, b);
            put(&mut self.verts, b.z, b.x, b.y, s[1], t[1], 1.0, b);
            put(&mut self.verts, b.z, a.x, b.y, s[0], t[1], 1.0, b);
            if !fst {
                for k in d.first as usize..self.verts.len() {
                    self.verts[k].s /= q[1];
                    self.verts[k].t /= q[1];
                }
            }
        } else {
            let flat = &v[n - 1];
            for i in 0..n {
                let c = if gouraud { &v[i] } else { flat };
                put(&mut self.verts, v[i].z, v[i].x, v[i].y, s[i], t[i], q[i], c);
            }
        }
        if (ty == 6 || fst) && !d.tex_is_target && self.targets[d.target].fbp as i32 == gs.main_fbp && Self::wide_picture() {
            self.narrow_2d(d.first as usize, ty == 6 && d.mode[0] == 0);
        }
        if d.tex_is_target && ty == 6 && v[1].x != v[0].x && v[1].y != v[0].y {
            let f = d.first as usize;
            let ds = 0.5 * (self.verts[f + 1].s - self.verts[f].s) / (v[1].x - v[0].x);
            let dt = 0.5 * (self.verts[f + 2].t - self.verts[f].t) / (v[1].y - v[0].y);
            for k in f..self.verts.len() {
                self.verts[k].s -= ds;
                self.verts[k].t -= dt;
            }
        }
        d.count = self.verts.len() as u32 - d.first;
        let tgen = gs.page_gen[(self.targets[d.target].fbp & 511) as usize];
        self.targets[d.target].draws += 1;
        self.targets[d.target].gen = tgen;
        if let Some(last) = self.draws.last_mut() {
            if Self::same_state(last, &d) {
                last.count += d.count;
                return;
            }
        }
        self.draws.push(d);
    }

    /// Widescreen outside the menus: the game projects its 3D for the wider picture itself (Port_WideFactor).
    fn wide_picture() -> bool {
        crate::host::settings::wide() && !MENU_MODE.load(Ordering::Relaxed)
    }

    /// Widescreen (the C port's GsGpu_Draw). 2D art is laid out for 4:3 and would come out too wide in a wider
    /// picture: each 2D piece drawn into the picture is narrowed (to 3/4 at 16:9) about a fixed point: in a fight
    /// the left edge, the right edge or the middle for the HUD's left panel, right panel and centre parts
    /// (markers from the HUD code), so the panels sit at the screen's edges; captions on the side they are on;
    /// any other 2D piece about the middle of the screen. Left alone: full-screen fills and fades (untextured,
    /// full height) and sprites as wide as the screen (the bands of READY / FIGHT, flashes, speed lines).
    fn narrow_2d(&mut self, first: usize, untextured_sprite: bool) {
        let vs = &mut self.verts[first..];
        let (mut x0, mut x1, mut y0, mut y1) = (vs[0].x, vs[0].x, vs[0].y, vs[0].y);
        for v in vs.iter() {
            x0 = x0.min(v.x);
            x1 = x1.max(v.x);
            y0 = y0.min(v.y);
            y1 = y1.max(v.y);
        }
        let pivot = if (untextured_sprite && y1 - y0 >= 400.0) || x1 - x0 >= 480.0 {
            return;
        } else if self.anchor != 0 {
            match self.anchor {
                1 => 0.0,
                2 => 512.0,
                4 => {
                    let cx = (x0 + x1) * 0.5;
                    if x1 <= 300.0 || cx < 180.0 {
                        0.0
                    } else if x0 >= 212.0 || cx > 332.0 {
                        512.0
                    } else {
                        256.0
                    }
                }
                _ => 256.0,
            }
        } else {
            256.0
        };
        let narrow = 1333.333 / crate::host::settings::aspect_milli() as f32;
        for v in vs.iter_mut() {
            v.x = pivot + (v.x - pivot) * narrow;
        }
    }

    fn depth_max(gs: &Gs, ctx: usize) -> f32 {
        match (gs.s.zbuf[ctx] >> 24) & 15 {
            0 => 4294967295.0,
            1 => 16777215.0,
            _ => 65535.0,
        }
    }

    /// A batch of a vertex program served by its shader: one strip (`count` vertices of `stride` floats) as
    /// triangles, without those `skip` names (by the index of their last vertex).
    fn draw_vu(&mut self, gs: &mut Gs, ctx: usize, vu: u32, mut u: VuUniform, verts: &[f32], count: usize, stride: usize, skip: &dyn Fn(usize) -> bool) {
        if count < 3
            || self.vu_verts.len() + count > MAX_VU_VERTS
            || self.vu_idx.len() + (count - 2) * 3 > MAX_VU_IDX
            || self.draws.len() >= MAX_DRAWS
            || self.vu_uni.len() >= MAX_VU_UNIFORMS
        {
            return;
        }
        let Some((mut d, us, vs)) = self.draw_state(gs, ctx, 0, false, vu) else { return };
        if vu == 3 {
            u.pivot_b[0] = us; // program 6: the texture's size over the size of the buffer it lives in
            u.pivot_b[1] = vs;
        }
        self.date_snapshot(&d);
        d.vu = vu;
        d.first = self.vu_idx.len() as u32;
        let base = self.vu_verts.len() as u32;
        for i in 0..count - 2 {
            if skip(i + 2) {
                continue;
            }
            for k in 0..3 {
                self.vu_idx.push(base + (i + k) as u32);
            }
        }
        d.count = self.vu_idx.len() as u32 - d.first;
        if d.count == 0 {
            return;
        }
        for i in 0..count {
            let v = &verts[i * stride..];
            let mut o = [0f32; 12];
            if vu == 3 {
                o[..4].copy_from_slice(&v[..4]); // position
                o[4..8].copy_from_slice(&v[8..12]); // colour
            } else {
                o.copy_from_slice(&v[..12]);
            }
            self.vu_verts.push(o);
        }
        let tgen = gs.page_gen[(self.targets[d.target].fbp & 511) as usize];
        self.targets[d.target].draws += 1;
        self.targets[d.target].gen = tgen;
        if let Some(last) = self.draws.last_mut() {
            if Self::same_state(last, &d) && self.vu_uni[last.uniform as usize] == u {
                last.count += d.count;
                return;
            }
        }
        d.uniform = self.vu_uni.len() as u32;
        self.vu_uni.push(u);
        self.draws.push(d);
    }

    fn offsets(gs: &Gs, ctx: usize, u: &mut VuUniform) {
        u.misc[0] = (gs.s.xyoffset[ctx] & 0xFFFF) as f32 / 16.0;
        u.misc[1] = ((gs.s.xyoffset[ctx] >> 32) & 0xFFFF) as f32 / 16.0;
        u.misc[2] = Self::depth_max(gs, ctx);
    }

    /// Vertex program 0 (the fighters' models) and its relatives (layers 2..5), see shaders/vu0.vert. Vertices:
    /// 12 floats each; `c`: the constants as program 0 has them (96 floats).
    pub fn draw_vu0(&mut self, gs: &mut Gs, layer: u32, ctx: usize, verts: &[f32], count: usize, c: &[f32]) {
        let mut u = VuUniform::default();
        u.bone_a.copy_from_slice(&c[0..16]);
        u.bone_b.copy_from_slice(&c[16..32]);
        u.pivot_a.copy_from_slice(&c[32..36]);
        u.pivot_b.copy_from_slice(&c[36..40]);
        u.screen.copy_from_slice(&c[56..72]);
        u.light = [c[40], c[44], c[48], c[52]];
        u.color0.copy_from_slice(&c[88..92]);
        u.color1.copy_from_slice(&c[92..96]);
        Self::offsets(gs, ctx, &mut u);
        u.misc[3] = layer as f32;
        u.light2 = [c[41], c[45], c[49], c[53]];
        let skip = |i: usize| layer == 4 && verts[i * 12 + 3].to_bits() & 0xFFFF != 0;
        self.draw_vu(gs, ctx, 1, u, verts, count, 12, &skip);
    }

    /// Vertex program 4 (the stage): position, colour (0..255 floats), texture coordinates; the screen matrix is
    /// VU memory 0..3.
    pub fn draw_vu4(&mut self, gs: &mut Gs, ctx: usize, verts: &[f32], count: usize, c: &[f32]) {
        let mut u = VuUniform::default();
        u.screen.copy_from_slice(&c[0..16]);
        Self::offsets(gs, ctx, &mut u);
        self.draw_vu(gs, ctx, 2, u, verts, count, 12, &|_| false);
    }

    /// Vertex program 6 (a fighter's ground shadow): four quadwords a vertex (position with a "no draw" flag in w,
    /// normal, colour, unused).
    pub fn draw_vu6(&mut self, gs: &mut Gs, ctx: usize, verts: &[f32], count: usize, c: &[f32]) {
        let mut u = VuUniform::default();
        u.bone_a.copy_from_slice(&c[0x0C * 4..0x10 * 4]);
        u.pivot_a[0] = c[0x10 * 4];
        u.screen.copy_from_slice(&c[0..16]);
        Self::offsets(gs, ctx, &mut u);
        let skip = |i: usize| verts[i * 16 + 3].to_bits() & 0xFFFF != 0;
        self.draw_vu(gs, ctx, 3, u, verts, count, 16, &skip);
    }

    /// The port's marker register (0x7F): 1 = the outline goes here.
    pub fn native(&mut self, gs: &Gs, effect: i32) {
        if (0x10..=0x14).contains(&effect) {
            self.anchor = match effect {
                0x13 => 0,
                0x14 => 4,
                e => e - 0x0F,
            };
            return;
        }
        if effect != 1 || self.draws.len() >= MAX_DRAWS {
            return;
        }
        let Some(target) = self.target_get((gs.s.frame[0] & 0x1FF) as u32, false) else { return };
        let scissor = self.scissor_of(gs.s.scissor[0]);
        self.draws.push(Draw { native: 1, target, scissor, ..Default::default() });
    }

    /// A picture was uploaded straight into a display buffer (the movies).
    pub fn fb_upload(&mut self, second: bool) {
        if self.draws.len() >= MAX_DRAWS {
            return;
        }
        if let Some(t) = self.target_get(if second { 0x70 } else { 0 }, true) {
            self.draws.push(Draw { native: 100 + second as i32, target: t, ..Default::default() });
        }
    }

    // ------------------------------------------------------------------------------------------ replay
    unsafe fn apply_blend(&self, bkey: u32) {
        let gl = &self.gl;
        let k = [(bkey >> 1) & 7, (bkey >> 4) & 7];
        let c = (bkey >> 7) & 3;
        let fc = match c {
            0 => gl::SRC_ALPHA,
            1 => gl::DST_ALPHA,
            _ => gl::CONSTANT_COLOR,
        };
        let fi = match c {
            0 => gl::ONE_MINUS_SRC_ALPHA,
            1 => gl::ONE_MINUS_DST_ALPHA,
            _ => gl::ONE_MINUS_CONSTANT_COLOR,
        };
        if bkey & 1 != 0 {
            (gl.Enablei)(gl::BLEND, 0);
            (gl.Disablei)(gl::BLEND, 1);
        } else {
            (gl.Disablei)(gl::BLEND, 0);
            (gl.Disablei)(gl::BLEND, 1);
            return;
        }
        let f = |k: u32| match k {
            0 => gl::ZERO,
            1 | 5 => gl::ONE,
            4 => fi,
            _ => fc,
        };
        (gl.BlendFuncSeparate)(f(k[0]), f(k[1]), gl::ONE, gl::ZERO);
        let eq = if k[0] == 3 {
            gl::FUNC_REVERSE_SUBTRACT
        } else if k[1] == 3 {
            gl::FUNC_SUBTRACT
        } else {
            gl::FUNC_ADD
        };
        (gl.BlendEquationSeparate)(eq, gl::FUNC_ADD);
    }

    unsafe fn apply_masks(&self, w: u32) {
        let gl = &self.gl;
        (gl.ColorMaski)(0, (w & CC_R != 0) as u8, (w & CC_G != 0) as u8, (w & CC_B != 0) as u8, (w & CC_A != 0) as u8);
        (gl.ColorMaski)(1, (w & CC_A != 0) as u8, 0, 0, 0);
    }

    unsafe fn set_pipe(&self, p: &Pipe, cur_prog: &mut GLuint) {
        let gl = &self.gl;
        let prog = if p.fx { self.prog_dclut } else { self.prog_main[p.vu as usize] };
        if *cur_prog != prog {
            *cur_prog = prog;
            (gl.UseProgram)(prog);
        }
        self.apply_blend(p.bkey);
        self.apply_masks(p.wmask);
        if p.fx {
            (gl.Disable)(gl::DEPTH_TEST);
            (gl.DepthMask)(0);
        } else {
            (gl.Enable)(gl::DEPTH_TEST);
            (gl.DepthMask)(p.zwrite as u8);
            (gl.DepthFunc)(match p.ztst {
                0 => gl::NEVER,
                1 => gl::ALWAYS,
                2 => gl::GEQUAL,
                _ => gl::GREATER,
            });
        }
    }

    unsafe fn bind_target(&self, i: usize, clear: bool) {
        let gl = &self.gl;
        let bufs = [gl::COLOR_ATTACHMENT0, gl::COLOR_ATTACHMENT1];
        (gl.BindFramebuffer)(gl::FRAMEBUFFER, self.targets[i].fbo);
        (gl.DrawBuffers)(2, bufs.as_ptr());
        (gl.Viewport)(0, 0, GS_W * self.scale, GS_H * self.scale);
        if clear {
            let z = [0f32; 4];
            (gl.Disable)(gl::SCISSOR_TEST);
            (gl.ColorMaski)(0, 1, 1, 1, 1);
            (gl.ColorMaski)(1, 1, 1, 1, 1);
            (gl.DepthMask)(1);
            (gl.ClearBufferfv)(gl::COLOR, 0, z.as_ptr());
            (gl.ClearBufferfv)(gl::COLOR, 1, z.as_ptr());
            (gl.ClearBufferfv)(gl::DEPTH, 0, z.as_ptr());
            (gl.Enable)(gl::SCISSOR_TEST);
        }
    }

    /// The port's own screen (host/ui.rs) over the picture just presented, when one is up.
    unsafe fn draw_overlay(&self) {
        let o = crate::host::ui::OVERLAY.lock().unwrap();
        if !o.visible || o.rgba.len() != crate::host::ui::W * crate::host::ui::H * 4 {
            return;
        }
        let gl = &self.gl;
        let (w, h) = (crate::host::ui::W as i32, crate::host::ui::H as i32);
        if self.overlay_tex.get() == 0 {
            let mut t = 0;
            (gl.GenTextures)(1, &mut t);
            self.overlay_tex.set(t);
            (gl.BindTexture)(gl::TEXTURE_2D, t);
            (gl.TexImage2D)(gl::TEXTURE_2D, 0, gl::RGBA8 as i32, w, h, 0, gl::RGBA, gl::UNSIGNED_BYTE, std::ptr::null());
        }
        if self.overlay_serial.get() != o.serial {
            (gl.BindTexture)(gl::TEXTURE_2D, self.overlay_tex.get());
            (gl.TexSubImage2D)(gl::TEXTURE_2D, 0, 0, 0, w, h, gl::RGBA, gl::UNSIGNED_BYTE, o.rgba.as_ptr() as *const c_void);
            self.overlay_serial.set(o.serial);
        }
        drop(o);
        let gl = &self.gl;
        (gl.Enablei)(gl::BLEND, 0);
        (gl.BlendEquationSeparate)(gl::FUNC_ADD, gl::FUNC_ADD);
        (gl.BlendFuncSeparate)(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA, gl::ONE, gl::ONE_MINUS_SRC_ALPHA);
        (gl.UseProgram)(self.prog_overlay);
        (gl.BindVertexArray)(self.vao);
        self.bind_tex(0, self.overlay_tex.get(), 7);
        let gl = &self.gl;
        (gl.DrawArrays)(gl::TRIANGLES, 0, 3);
        (gl.BindVertexArray)(0);
        (gl.Disablei)(gl::BLEND, 0);
    }

    unsafe fn bind_tex(&self, unit: u32, tex: GLuint, sampler: usize) {
        let gl = &self.gl;
        (gl.ActiveTexture)(gl::TEXTURE0 + unit);
        (gl.BindTexture)(gl::TEXTURE_2D, tex);
        (gl.BindSampler)(unit, self.samplers[sampler]);
    }

    unsafe fn set_params(&self, data: &[u8]) {
        let gl = &self.gl;
        (gl.BindBuffer)(gl::UNIFORM_BUFFER, self.ubo);
        (gl.BufferSubData)(gl::UNIFORM_BUFFER, 0, data.len() as isize, data.as_ptr() as *const c_void);
    }

    unsafe fn blit_aux(&self, target: usize, dst: GLuint) {
        let gl = &self.gl;
        let (w, h) = (GS_W * self.scale, GS_H * self.scale);
        (gl.Disable)(gl::SCISSOR_TEST);
        (gl.BindFramebuffer)(gl::READ_FRAMEBUFFER, self.scratch_read);
        (gl.FramebufferTexture2D)(gl::READ_FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, self.targets[target].aux, 0);
        (gl.ReadBuffer)(gl::COLOR_ATTACHMENT0);
        (gl.BindFramebuffer)(gl::DRAW_FRAMEBUFFER, self.scratch_draw);
        (gl.FramebufferTexture2D)(gl::DRAW_FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, dst, 0);
        let one = [gl::COLOR_ATTACHMENT0];
        (gl.DrawBuffers)(1, one.as_ptr());
        (gl.BlitFramebuffer)(0, 0, w, h, 0, 0, w, h, gl::COLOR_BUFFER_BIT, gl::NEAREST as u32);
        (gl.BindFramebuffer)(gl::READ_FRAMEBUFFER, 0);
        (gl.BindFramebuffer)(gl::DRAW_FRAMEBUFFER, 0);
        (gl.Enable)(gl::SCISSOR_TEST);
    }

    /// The target shown for display buffer `main_fbp` (sceGsSwapDBuff's), else the first.
    fn shown_target(&self, main_fbp: i32) -> Option<usize> {
        let mut best = if main_fbp >= 0 { self.targets.iter().position(|t| t.fbp == main_fbp as u32) } else { None };
        if best.is_none() && !self.targets.is_empty() {
            best = Some(0);
        }
        best
    }

    /// The target `best` to the window (kept to 4:3), the port's screens over it, then the swap.
    unsafe fn present(&mut self, best: Option<usize>) {
        let gl = &self.gl;
        (gl.BindVertexArray)(0);
        (gl.BindFramebuffer)(gl::FRAMEBUFFER, 0);
        if self.surface.has_window() {
            let (sw, sh) = self.surface.size();
            self.draw_window(best, sw, sh);
            self.surface.swap(); // (pacing: at the vertical blank, host::vblank_wait)
        }
    }

    /// The picture as the window shows it, into the framebuffer bound (`sw` x `sh` pixels).
    unsafe fn draw_window(&mut self, best: Option<usize>, sw: i32, sh: i32) {
        let gl = &self.gl;
        (gl.Viewport)(0, 0, sw, sh);
        (gl.Disable)(gl::SCISSOR_TEST);
        (gl.ClearColor)(0.0, 0.0, 0.0, 1.0);
        (gl.Clear)(gl::COLOR_BUFFER_BIT);
        if let Some(b) = best.filter(|&b| self.targets[b].cleared) {
            // the picture's shape: the chosen one (16:9 by default). Fights and the attract demo are drawn wide;
            // the menus' pages are composed for 4:3 and are stretched to it (the player's choice: better than
            // bars or a blurred copy at the sides)
            let want = crate::host::settings::aspect_milli() as f32 / 1000.0;
            let fit = |want: f32| {
                if sw as f32 > sh as f32 * want {
                    ((sh as f32 * want + 0.5) as i32, sh)
                } else {
                    (sw, (sw as f32 / want + 0.5) as i32)
                }
            };
            let (w, h) = fit(want);
            (gl.Viewport)((sw - w) / 2, (sh - h) / 2, w, h);
            (gl.Disable)(gl::DEPTH_TEST);
            (gl.Disablei)(gl::BLEND, 0);
            (gl.ColorMask)(1, 1, 1, 1);
            (gl.UseProgram)(self.prog_present);
            (gl.BindVertexArray)(self.vao);
            self.bind_tex(0, self.targets[b].col, 7);
            (gl.DrawArrays)(gl::TRIANGLES, 0, 3);
            (gl.BindVertexArray)(0);
            // the port's screens are drawn for 4:3: in the middle, at the picture's height
            let (w, h) = fit(4.0 / 3.0);
            (gl.Viewport)((sw - w) / 2, (sh - h) / 2, w, h);
            self.draw_overlay();
        }
    }

    /// BT3_SHOT_WINDOW=WxH (with BT3_SHOT): the picture as a window of that size shows it, to
    /// shots/window_NNNNN.png (tests without a window: present's own drawing, letterbox, menus' backdrop, overlay).
    unsafe fn window_shot(&mut self, best: Option<usize>) {
        let Some((w, h)) = std::env::var("BT3_SHOT_WINDOW").ok().and_then(|v| {
            let (a, b) = v.split_once('x')?;
            Some((a.parse::<i32>().ok()?, b.parse::<i32>().ok()?))
        }) else {
            return;
        };
        let gl = &self.gl;
        let (mut tex, mut fbo) = (0, 0);
        (gl.GenTextures)(1, &mut tex);
        (gl.BindTexture)(gl::TEXTURE_2D, tex);
        (gl.TexImage2D)(gl::TEXTURE_2D, 0, gl::RGBA8 as i32, w, h, 0, gl::RGBA, gl::UNSIGNED_BYTE, std::ptr::null());
        (gl.GenFramebuffers)(1, &mut fbo);
        (gl.BindFramebuffer)(gl::FRAMEBUFFER, fbo);
        (gl.FramebufferTexture2D)(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, tex, 0);
        let bufs = [gl::COLOR_ATTACHMENT0];
        (gl.DrawBuffers)(1, bufs.as_ptr());
        self.draw_window(best, w, h);
        let gl = &self.gl;
        let mut px = vec![0u8; (w * h * 4) as usize];
        (gl.BindFramebuffer)(gl::READ_FRAMEBUFFER, fbo);
        (gl.ReadBuffer)(gl::COLOR_ATTACHMENT0);
        (gl.ReadPixels)(0, 0, w, h, gl::RGBA, gl::UNSIGNED_BYTE, px.as_mut_ptr() as *mut c_void);
        (gl.BindFramebuffer)(gl::FRAMEBUFFER, 0);
        (gl.DeleteFramebuffers)(1, &fbo);
        (gl.DeleteTextures)(1, &tex);
        // the window's rows are bottom-up
        let row = (w * 4) as usize;
        let mut out = vec![0u8; px.len()];
        for y in 0..h as usize {
            out[y * row..(y + 1) * row].copy_from_slice(&px[(h as usize - 1 - y) * row..(h as usize - y) * row]);
        }
        let _ = std::fs::create_dir_all("shots");
        let _ = super::write_png(&format!("shots/window_{:05}.png", self.frame), w as u32, h as u32, &out);
    }

    /// Esc: "quit the game?" over the last picture, the game paused until it is answered.
    fn quit_question(&mut self, main_fbp: i32) {
        let best = self.shown_target(main_fbp);
        let saved = crate::host::ui::snapshot();
        let mut yes = false;
        let mut last = 0xFFFFu16;
        loop {
            crate::host::ui::show_question(&saved, "Sair do jogo?", "O progresso desde o último salvamento será perdido.", yes);
            unsafe { self.present(best) };
            std::thread::sleep(std::time::Duration::from_millis(16));
            let inp = self.surface.poll();
            if inp.quit {
                std::process::exit(0);
            }
            let pressed = inp.buttons & !last;
            last = inp.buttons;
            if pressed & (0x10 | 0x20 | 0x40 | 0x80) != 0 {
                yes = !yes;
            }
            if pressed & (0x4000 | 0x8) != 0 {
                if yes {
                    std::process::exit(0);
                }
                break;
            }
            if inp.escape || pressed & 0x2000 != 0 {
                break;
            }
        }
        crate::host::ui::restore(saved);
    }

    /// F1: the PC settings (host/settings.rs) over the last picture, the game paused until they are closed.
    fn settings_menu(&mut self, main_fbp: i32) {
        let best = self.shown_target(main_fbp);
        let saved = crate::host::ui::snapshot();
        let mut menu = crate::host::settings::Menu::new();
        loop {
            menu.draw(&saved);
            unsafe { self.present(best) };
            std::thread::sleep(std::time::Duration::from_millis(16));
            let inp = self.surface.poll();
            if inp.quit {
                std::process::exit(0);
            }
            let close = matches!(menu.step(inp.buttons, inp.key_down, inp.escape), crate::host::settings::Step::Close);
            if close || inp.settings {
                break;
            }
        }
        crate::host::ui::restore(saved);
    }

    /// The settings of the picture (host/settings.rs, changed on Options > Graphics) applied when they change:
    /// full screen, the window's size for the resolution, the internal resolution (render targets made again).
    fn apply_settings(&mut self) {
        let serial = crate::host::settings::SERIAL.load(Ordering::Relaxed);
        if self.settings_serial == Some(serial) {
            return;
        }
        let first = self.settings_serial.is_none();
        self.settings_serial = Some(serial);
        let st = crate::host::settings::get();
        self.surface.set_fullscreen(st.fullscreen);
        let size = (st.resolution, crate::host::settings::aspect_milli());
        if !st.fullscreen && (first || self.window_for != size) {
            let w = (st.resolution as f32 * size.1 as f32 / 1000.0 + 0.5) as i32;
            self.surface.set_window_size(w, st.resolution);
        }
        self.window_for = size;
        if st.textures != self.tex_quality {
            self.tex_quality = st.textures;
            self.textures_flush();
        }
        let scale = st.scale();
        if scale != self.scale {
            // the frame buffers again at the new size (their contents are lost: one frame drawn afresh)
            for i in 0..self.targets.len() {
                self.target_drop(i);
            }
            self.targets.clear();
            unsafe {
                (self.gl.DeleteTextures)(1, &self.date_copy);
                (self.gl.DeleteTextures)(1, &self.aux_copy);
            }
            self.scale = scale;
            self.copies_create();
            self.last_tex = None;
            eprintln!("bt3: internal resolution {scale}x ({} x {})", 512 * scale, 448 * scale);
        }
    }

    /// The end of the frame: the list replayed, the shown buffer presented (and a screenshot when asked).
    pub fn frame_end(&mut self, gs: &mut Gs) {
        self.input = self.surface.poll();
        if self.input.quit {
            std::process::exit(0);
        }
        if self.input.escape {
            self.quit_question(gs.main_fbp);
            self.input = Input { lx: 128, ly: 128, rx: 128, ry: 128, ..Default::default() };
        } else if self.input.settings {
            self.settings_menu(gs.main_fbp);
            self.input = Input { lx: 128, ly: 128, rx: 128, ry: 128, ..Default::default() };
        }
        // pictures uploaded into the display buffers (the movies)
        for i in 0..2 {
            if self.fb_uploads & (1 << i) == 0 {
                continue;
            }
            let fbp = if i == 1 { 0x70 } else { 0 };
            let mut px = vec![0u32; (GS_W * GS_H) as usize];
            for y in 0..GS_H as u32 {
                for x in 0..GS_W as u32 {
                    px[(y * GS_W as u32 + x) as usize] = gs.rd(fbp * 32, 8, 0, x, y) | 0xFF00_0000;
                }
            }
            let gl = &self.gl;
            unsafe {
                if self.fb_tex[i] == 0 {
                    (gl.GenTextures)(1, &mut self.fb_tex[i]);
                    (gl.BindTexture)(gl::TEXTURE_2D, self.fb_tex[i]);
                    (gl.TexImage2D)(gl::TEXTURE_2D, 0, gl::RGBA8 as i32, GS_W, GS_H, 0, gl::RGBA, gl::UNSIGNED_BYTE, std::ptr::null());
                    (gl.GenFramebuffers)(1, &mut self.fb_fbo[i]);
                    (gl.BindFramebuffer)(gl::FRAMEBUFFER, self.fb_fbo[i]);
                    (gl.FramebufferTexture2D)(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, self.fb_tex[i], 0);
                }
                (gl.BindTexture)(gl::TEXTURE_2D, self.fb_tex[i]);
                (gl.TexSubImage2D)(gl::TEXTURE_2D, 0, 0, 0, GS_W, GS_H, gl::RGBA, gl::UNSIGNED_BYTE, px.as_ptr() as *const c_void);
            }
        }
        self.fb_uploads = 0;
        let mut draws = std::mem::take(&mut self.draws);
        if self.no_replay {
            draws.clear(); // (profiling the capture side: nothing is drawn)
        }
        unsafe {
            let gl = &self.gl;
            if !self.verts.is_empty() {
                (gl.BindBuffer)(gl::ARRAY_BUFFER, self.vbo);
                (gl.BufferSubData)(gl::ARRAY_BUFFER, 0, (self.verts.len() * std::mem::size_of::<Vtx>()) as isize, self.verts.as_ptr() as *const c_void);
            }
            if !self.vu_verts.is_empty() {
                (gl.BindBuffer)(gl::ARRAY_BUFFER, self.vbo_vu);
                (gl.BufferSubData)(gl::ARRAY_BUFFER, 0, (self.vu_verts.len() * 48) as isize, self.vu_verts.as_ptr() as *const c_void);
            }
            if !self.vu_idx.is_empty() {
                (gl.BindVertexArray)(self.vao_vu);
                (gl.BufferSubData)(gl::ELEMENT_ARRAY_BUFFER, 0, (self.vu_idx.len() * 4) as isize, self.vu_idx.as_ptr() as *const c_void);
            }
            (gl.BindVertexArray)(self.vao);
            let mut cur_vao = self.vao;
            let mut last_uniform: Option<u32> = None;
            (gl.Enable)(gl::SCISSOR_TEST);
            let mut cur: Option<usize> = None;
            let mut cur_prog = 0;
            let mut last_params: Option<[u8; 64]> = None;
            let mut last_blendc = -1.0f32;
            for d in &draws {
                let s = self.scale;
                if d.native >= 100 {
                    let i = (d.native - 100) as usize;
                    if self.fb_tex[i] == 0 {
                        continue;
                    }
                    (gl.Disable)(gl::SCISSOR_TEST);
                    (gl.BindFramebuffer)(gl::READ_FRAMEBUFFER, self.fb_fbo[i]);
                    (gl.BindFramebuffer)(gl::DRAW_FRAMEBUFFER, self.targets[d.target].fbo);
                    let one = [gl::COLOR_ATTACHMENT0];
                    (gl.DrawBuffers)(1, one.as_ptr());
                    (gl.BlitFramebuffer)(0, 0, GS_W, GS_H, 0, 0, GS_W * s, GS_H * s, gl::COLOR_BUFFER_BIT, gl::LINEAR as u32);
                    (gl.Enable)(gl::SCISSOR_TEST);
                    self.targets[d.target].cleared = true;
                    cur = None;
                    continue;
                }
                if d.native == 5 || d.native == 4 {
                    if !self.targets[d.target].cleared {
                        continue;
                    }
                    self.blit_aux(d.target, if d.native == 5 { self.date_copy } else { self.aux_copy });
                    cur = None;
                    continue;
                }
                if d.native == 3 {
                    if !self.targets[d.target].cleared {
                        continue;
                    }
                    if cur != Some(d.target) {
                        self.bind_target(d.target, false);
                        cur = Some(d.target);
                    }
                    let p = self.pipes[d.pipeline];
                    self.set_pipe(&p, &mut cur_prog);
                    self.bind_tex(0, self.targets[d.target].dep, 6);
                    self.bind_tex(1, self.aux_copy, 6);
                    self.bind_tex(2, d.tex, 6);
                    let mut params = [0u8; 64];
                    params[0..4].copy_from_slice(&(d.src as f32).to_le_bytes());
                    params[4..8].copy_from_slice(&d.misc[0].to_le_bytes());
                    self.set_params(&params);
                    last_params = None;
                    if d.blendc != last_blendc {
                        (gl.BlendColor)(d.blendc, d.blendc, d.blendc, d.blendc);
                        last_blendc = d.blendc;
                    }
                    (gl.Scissor)(d.scissor[0], d.scissor[1], d.scissor[2], d.scissor[3]);
                    (gl.DrawArrays)(gl::TRIANGLES, 0, 3);
                    continue;
                }
                if d.native == 1 {
                    if !self.targets[d.target].cleared {
                        continue;
                    }
                    if cur != Some(d.target) {
                        self.bind_target(d.target, false);
                        cur = Some(d.target);
                    }
                    (gl.Disable)(gl::DEPTH_TEST);
                    (gl.DepthMask)(0);
                    (gl.Enablei)(gl::BLEND, 0);
                    (gl.Disablei)(gl::BLEND, 1);
                    (gl.BlendFuncSeparate)(gl::ONE, gl::ONE, gl::ONE, gl::ZERO);
                    (gl.BlendEquationSeparate)(gl::FUNC_REVERSE_SUBTRACT, gl::FUNC_ADD);
                    (gl.ColorMaski)(0, 1, 1, 1, 0);
                    (gl.ColorMaski)(1, 0, 0, 0, 0);
                    cur_prog = self.prog_outline;
                    (gl.UseProgram)(cur_prog);
                    self.bind_tex(0, self.aux_copy, 6);
                    let mut params = [0u8; 64];
                    params[0..4].copy_from_slice(&(1.0f32 / GS_W as f32).to_le_bytes());
                    params[4..8].copy_from_slice(&(1.0f32 / GS_H as f32).to_le_bytes());
                    params[8..12].copy_from_slice(&(100.0f32 / 255.0).to_le_bytes());
                    self.set_params(&params);
                    last_params = None;
                    (gl.Scissor)(d.scissor[0], d.scissor[1], d.scissor[2], d.scissor[3]);
                    (gl.DrawArrays)(gl::TRIANGLES, 0, 3);
                    // the next primitive sets its pipeline again
                    cur_prog = 0;
                    continue;
                }
                // an ordinary primitive, or a vertex program's batch
                if cur != Some(d.target) {
                    let clear = !self.targets[d.target].cleared;
                    self.bind_target(d.target, clear);
                    self.targets[d.target].cleared = true;
                    cur = Some(d.target);
                } else {
                    self.targets[d.target].cleared = true;
                }
                let p = self.pipes[d.pipeline];
                self.set_pipe(&p, &mut cur_prog);
                let mut params = [0u8; 64];
                for i in 0..4 {
                    params[i * 4..i * 4 + 4].copy_from_slice(&d.mode[i].to_le_bytes());
                    params[16 + i * 4..20 + i * 4].copy_from_slice(&d.misc[i].to_le_bytes());
                    params[32 + i * 4..36 + i * 4].copy_from_slice(&d.rect[i].to_le_bytes());
                    params[48 + i * 4..52 + i * 4].copy_from_slice(&d.orig[i].to_le_bytes());
                }
                if last_params != Some(params) {
                    self.set_params(&params);
                    last_params = Some(params);
                }
                self.bind_tex(0, d.tex, d.sampler as usize);
                self.bind_tex(1, self.date_copy, 6);
                if d.blendc != last_blendc {
                    (gl.BlendColor)(d.blendc, d.blendc, d.blendc, d.blendc);
                    last_blendc = d.blendc;
                }
                (gl.Scissor)(d.scissor[0], d.scissor[1], d.scissor[2], d.scissor[3]);
                if d.vu != 0 {
                    if cur_vao != self.vao_vu {
                        cur_vao = self.vao_vu;
                        (gl.BindVertexArray)(cur_vao);
                    }
                    if last_uniform != Some(d.uniform) {
                        let u = &self.vu_uni[d.uniform as usize];
                        (gl.BindBuffer)(gl::UNIFORM_BUFFER, self.ubo_u);
                        (gl.BufferSubData)(gl::UNIFORM_BUFFER, 0, std::mem::size_of::<VuUniform>() as isize, u as *const VuUniform as *const c_void);
                        last_uniform = Some(d.uniform);
                    }
                    (gl.DrawElements)(gl::TRIANGLES, d.count as i32, gl::UNSIGNED_INT, (d.first as usize * 4) as *const c_void);
                } else {
                    if cur_vao != self.vao {
                        cur_vao = self.vao;
                        (gl.BindVertexArray)(cur_vao);
                    }
                    let mode = match p.topo {
                        0 => gl::TRIANGLES,
                        1 => gl::LINES,
                        _ => gl::POINTS,
                    };
                    (gl.DrawArrays)(mode, d.first as i32, d.count as i32);
                }
            }
        }
        // the shown buffer: the one sceGsSwapDBuff set up for this frame, else the first target
        let best = self.shown_target(gs.main_fbp);
        unsafe {
            self.present(best);
            if self.shot_every > 0 && self.frame % self.shot_every == 0 {
                self.window_shot(best);
            }
            let gl = &self.gl;
            if self.verbose && self.frame % 30 == 0 {
                eprintln!(
                    "gpu: frame {}: {} draws, {} vertices, {} targets, {} textures, {} pipelines, {} PS2-only primitives dropped, natives {:?}, GL error {:#x}",
                    self.frame,
                    draws.len(),
                    self.verts.len(),
                    self.targets.len(),
                    self.textures.len(),
                    self.pipes.len(),
                    self.skipped,
                    [1, 3, 4, 5].map(|n| draws.iter().filter(|d| d.native == n).count()),
                    (gl.GetError)()
                );
            }
            if self.shot_every > 0 && self.frame % self.shot_every == 0 {
                if let Some(b) = best.filter(|&b| self.targets[b].cleared) {
                    let s = self.scale;
                    let (w, h) = (512 * s, 448 * s);
                    let mut px = vec![0u8; (w * h * 4) as usize];
                    (gl.BindFramebuffer)(gl::READ_FRAMEBUFFER, self.targets[b].fbo);
                    (gl.ReadBuffer)(gl::COLOR_ATTACHMENT0);
                    (gl.ReadPixels)(0, 0, w, h, gl::RGBA, gl::UNSIGNED_BYTE, px.as_mut_ptr() as *mut c_void);
                    (gl.BindFramebuffer)(gl::READ_FRAMEBUFFER, 0);
                    if std::env::var("BT3_SHOT_FULL").is_ok() {
                        // BT3_SHOT_FULL=1: the render target as it is (internal resolution)
                        let mut flip = px.clone();
                        for a in flip.chunks_mut(4) {
                            a[3] = 255;
                        }
                        let _ = std::fs::create_dir_all("shots");
                        let _ = super::write_png(&format!("shots/full_{:05}.png", self.frame), w as u32, h as u32, &flip);
                    }
                    // to 640 x 448 (the TV picture), nearest; wider pictures keep the height (853 x 448 at 16:9),
                    // the port's screens in the middle 640
                    let ow = if Self::wide_picture() { (640 * crate::host::settings::aspect_milli() + 666) / 1333 } else { 640 } as usize;
                    let mut out = vec![0u8; ow * 448 * 4];
                    for y in 0..448usize {
                        for x in 0..ow {
                            let sx = x * w as usize / ow;
                            let sy = y * s as usize;
                            let o = (sy * w as usize + sx) * 4;
                            let d = (y * ow + x) * 4;
                            out[d..d + 3].copy_from_slice(&px[o..o + 3]);
                            out[d + 3] = 255;
                        }
                    }
                    let x0 = (ow - 640) / 2;
                    let mut mid = vec![0u8; 640 * 448 * 4];
                    for y in 0..448 {
                        mid[y * 2560..(y + 1) * 2560].copy_from_slice(&out[(y * ow + x0) * 4..(y * ow + x0 + 640) * 4]);
                    }
                    crate::host::ui::blend_into(&mut mid, 640, 448);
                    for y in 0..448 {
                        out[(y * ow + x0) * 4..(y * ow + x0 + 640) * 4].copy_from_slice(&mid[y * 2560..(y + 1) * 2560]);
                    }
                    let _ = std::fs::create_dir_all("shots");
                    let _ = super::write_png(&format!("shots/gpu_{:05}.png", self.frame), ow as u32, 448, &out);
                }
            }
        }
        for t in self.targets.iter_mut() {
            t.draws = 0;
        }
        self.verts.clear();
        self.vu_verts.clear();
        self.vu_idx.clear();
        self.vu_uni.clear();
        self.draws = draws;
        self.draws.clear();
        self.skipped = 0;
        self.frame += 1;
        // between frames: no recorded draw refers to the render targets or the textures now
        self.apply_settings();
        self.texpack_collect();
    }
}
