//! The OpenGL 3.3 core entry points the GPU renderer uses, loaded by name through the platform's
//! GetProcAddress (SDL_GL_GetProcAddress on Windows, eglGetProcAddress in the headless tests).
#![allow(non_snake_case, clippy::too_many_arguments)]
use std::ffi::c_void;

pub type GLenum = u32;
pub type GLuint = u32;
pub type GLint = i32;
pub type GLsizei = i32;

macro_rules! gl_funcs {
    ($($name:ident : fn($($arg:ty),*) $(-> $ret:ty)?;)*) => {
        pub struct Gl { $(pub $name: unsafe extern "system" fn($($arg),*) $(-> $ret)?,)* }
        impl Gl {
            pub fn load(get: &dyn Fn(&str) -> *const c_void) -> Result<Gl, String> {
                Ok(Gl { $($name: {
                    let p = get(concat!("gl", stringify!($name)));
                    if p.is_null() { return Err(format!("missing GL entry point gl{}", stringify!($name))); }
                    unsafe { std::mem::transmute::<*const c_void, unsafe extern "system" fn($($arg),*) $(-> $ret)?>(p) }
                },)* })
            }
        }
    };
}

gl_funcs! {
    GetError: fn() -> GLenum;
    Enable: fn(GLenum);
    Disable: fn(GLenum);
    Enablei: fn(GLenum, GLuint);
    Disablei: fn(GLenum, GLuint);
    Viewport: fn(GLint, GLint, GLsizei, GLsizei);
    Scissor: fn(GLint, GLint, GLsizei, GLsizei);
    ClearColor: fn(f32, f32, f32, f32);
    Clear: fn(u32);
    ClearBufferfv: fn(GLenum, GLint, *const f32);
    GetString: fn(GLenum) -> *const u8;
    Finish: fn();
    PixelStorei: fn(GLenum, GLint);
    BlendFuncSeparate: fn(GLenum, GLenum, GLenum, GLenum);
    BlendEquationSeparate: fn(GLenum, GLenum);
    BlendColor: fn(f32, f32, f32, f32);
    ColorMaski: fn(GLuint, u8, u8, u8, u8);
    ColorMask: fn(u8, u8, u8, u8);
    DepthFunc: fn(GLenum);
    DepthMask: fn(u8);
    CreateShader: fn(GLenum) -> GLuint;
    ShaderSource: fn(GLuint, GLsizei, *const *const u8, *const GLint);
    CompileShader: fn(GLuint);
    GetShaderiv: fn(GLuint, GLenum, *mut GLint);
    GetShaderInfoLog: fn(GLuint, GLsizei, *mut GLsizei, *mut u8);
    DeleteShader: fn(GLuint);
    CreateProgram: fn() -> GLuint;
    AttachShader: fn(GLuint, GLuint);
    LinkProgram: fn(GLuint);
    GetProgramiv: fn(GLuint, GLenum, *mut GLint);
    GetProgramInfoLog: fn(GLuint, GLsizei, *mut GLsizei, *mut u8);
    UseProgram: fn(GLuint);
    GetUniformLocation: fn(GLuint, *const u8) -> GLint;
    Uniform1i: fn(GLint, GLint);
    Uniform2f: fn(GLint, f32, f32);
    GetUniformBlockIndex: fn(GLuint, *const u8) -> GLuint;
    UniformBlockBinding: fn(GLuint, GLuint, GLuint);
    BindBufferBase: fn(GLenum, GLuint, GLuint);
    GenBuffers: fn(GLsizei, *mut GLuint);
    BindBuffer: fn(GLenum, GLuint);
    BufferData: fn(GLenum, isize, *const c_void, GLenum);
    BufferSubData: fn(GLenum, isize, isize, *const c_void);
    GenVertexArrays: fn(GLsizei, *mut GLuint);
    BindVertexArray: fn(GLuint);
    EnableVertexAttribArray: fn(GLuint);
    VertexAttribPointer: fn(GLuint, GLint, GLenum, u8, GLsizei, *const c_void);
    DrawArrays: fn(GLenum, GLint, GLsizei);
    DrawElements: fn(GLenum, GLsizei, GLenum, *const c_void);
    GenSamplers: fn(GLsizei, *mut GLuint);
    BindSampler: fn(GLuint, GLuint);
    SamplerParameteri: fn(GLuint, GLenum, GLint);
    GenTextures: fn(GLsizei, *mut GLuint);
    BindTexture: fn(GLenum, GLuint);
    ActiveTexture: fn(GLenum);
    TexImage2D: fn(GLenum, GLint, GLint, GLsizei, GLsizei, GLint, GLenum, GLenum, *const c_void);
    TexSubImage2D: fn(GLenum, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *const c_void);
    TexParameteri: fn(GLenum, GLenum, GLint);
    DeleteTextures: fn(GLsizei, *const GLuint);
    GenFramebuffers: fn(GLsizei, *mut GLuint);
    BindFramebuffer: fn(GLenum, GLuint);
    FramebufferTexture2D: fn(GLenum, GLenum, GLenum, GLuint, GLint);
    CheckFramebufferStatus: fn(GLenum) -> GLenum;
    DeleteFramebuffers: fn(GLsizei, *const GLuint);
    DrawBuffers: fn(GLsizei, *const GLenum);
    ReadBuffer: fn(GLenum);
    BlitFramebuffer: fn(GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLint, u32, GLenum);
    ReadPixels: fn(GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *mut c_void);
}

pub const COLOR_BUFFER_BIT: u32 = 0x4000;
pub const POINTS: GLenum = 0;
pub const LINES: GLenum = 1;
pub const TRIANGLES: GLenum = 4;
pub const UNSIGNED_BYTE: GLenum = 0x1401;
pub const FLOAT: GLenum = 0x1406;
pub const RED: GLenum = 0x1903;
pub const COLOR: GLenum = 0x1800;
pub const DEPTH: GLenum = 0x1801;
pub const DEPTH_COMPONENT: GLenum = 0x1902;
pub const RGBA: GLenum = 0x1908;
pub const RGBA8: GLenum = 0x8058;
pub const R8: GLenum = 0x8229;
pub const DEPTH_COMPONENT32F: GLenum = 0x8CAC;
pub const TEXTURE_2D: GLenum = 0x0DE1;
pub const TEXTURE0: GLenum = 0x84C0;
pub const TEXTURE_MIN_FILTER: GLenum = 0x2801;
pub const TEXTURE_MAG_FILTER: GLenum = 0x2800;
pub const TEXTURE_WRAP_S: GLenum = 0x2802;
pub const TEXTURE_WRAP_T: GLenum = 0x2803;
pub const TEXTURE_MAX_LEVEL: GLenum = 0x813D;
pub const NEAREST: GLint = 0x2600;
pub const LINEAR: GLint = 0x2601;
pub const CLAMP_TO_EDGE: GLint = 0x812F;
pub const REPEAT: GLint = 0x2901;
pub const ARRAY_BUFFER: GLenum = 0x8892;
pub const ELEMENT_ARRAY_BUFFER: GLenum = 0x8893;
pub const UNSIGNED_INT: GLenum = 0x1405;
pub const STREAM_DRAW: GLenum = 0x88E0;
pub const DYNAMIC_DRAW: GLenum = 0x88E8;
pub const UNIFORM_BUFFER: GLenum = 0x8A11;
pub const FRAGMENT_SHADER: GLenum = 0x8B30;
pub const VERTEX_SHADER: GLenum = 0x8B31;
pub const COMPILE_STATUS: GLenum = 0x8B81;
pub const LINK_STATUS: GLenum = 0x8B82;
pub const INFO_LOG_LENGTH: GLenum = 0x8B84;
pub const VERSION: GLenum = 0x1F02;
pub const RENDERER: GLenum = 0x1F01;
pub const UNPACK_ALIGNMENT: GLenum = 0x0CF5;
pub const PACK_ALIGNMENT: GLenum = 0x0D05;
pub const BLEND: GLenum = 0x0BE2;
pub const DEPTH_TEST: GLenum = 0x0B71;
pub const SCISSOR_TEST: GLenum = 0x0C11;
pub const CULL_FACE: GLenum = 0x0B44;
pub const DEPTH_CLAMP: GLenum = 0x864F;
pub const ZERO: GLenum = 0;
pub const ONE: GLenum = 1;
pub const SRC_ALPHA: GLenum = 0x0302;
pub const ONE_MINUS_SRC_ALPHA: GLenum = 0x0303;
pub const DST_ALPHA: GLenum = 0x0304;
pub const ONE_MINUS_DST_ALPHA: GLenum = 0x0305;
pub const CONSTANT_COLOR: GLenum = 0x8001;
pub const ONE_MINUS_CONSTANT_COLOR: GLenum = 0x8002;
pub const FUNC_ADD: GLenum = 0x8006;
pub const FUNC_SUBTRACT: GLenum = 0x800A;
pub const FUNC_REVERSE_SUBTRACT: GLenum = 0x800B;
pub const NEVER: GLenum = 0x0200;
pub const GREATER: GLenum = 0x0204;
pub const GEQUAL: GLenum = 0x0206;
pub const ALWAYS: GLenum = 0x0207;
pub const FRAMEBUFFER: GLenum = 0x8D40;
pub const READ_FRAMEBUFFER: GLenum = 0x8CA8;
pub const DRAW_FRAMEBUFFER: GLenum = 0x8CA9;
pub const COLOR_ATTACHMENT0: GLenum = 0x8CE0;
pub const COLOR_ATTACHMENT1: GLenum = 0x8CE1;
pub const DEPTH_ATTACHMENT: GLenum = 0x8D00;
pub const FRAMEBUFFER_COMPLETE: GLenum = 0x8CD5;
pub const INVALID_INDEX: GLuint = 0xFFFF_FFFF;
pub const BACK: GLenum = 0x0405;
