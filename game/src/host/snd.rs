//! Sound on PC (with the window only): the C port's gs/snd_adx.c and gs/snd_se.c in Rust.
//!
//! PC-only playback reads canonical PCM WAV streams and typed audio-bank.json
//! effect bindings. It does not reconstruct ADX/Sony banks, allocate SPU sample
//! memory, or perform IOP/SPU uploads. The legacy decoders below are retained
//! only for explicit development/reference diagnostics outside PC-only mode.
//!
//! Streams (music, voices, the movies' sound): CRI's ADXT players. Every ADX file of the game is the plain kind
//! (type 3, 4-bit ADPCM in 18-byte frames, version 4, not encrypted, mono or stereo); a player reads the whole file
//! when it starts and decodes it on SDL's audio thread into its own SDL stream (SDL converts the rate and mixes).
//!
//! Effects: the game's sound driver for the second processor (SOUNDS.IRX), replaced by a sampler. The game loads
//! banks (sample data into the sound chip's memory through sceSdRemote, two tables into the second processor's
//! memory) and sends, once per frame, a queue of PLAY / STOP commands. Every effect of the game is one note of
//! the sample with its own number (measured over all 361 banks), so an effect is "play sample N of the bank": the
//! PS2 sound chip's ADPCM decoded here, pitched, mixed into one 48 kHz stream.
//!
//! What the game sees of all this is kept apart (`AdxView`): whether a stream is still playing is counted in
//! vertical blanks from the file's length, so the game's state does not depend on the sound device. Headless
//! (no window) nothing here exists and the players report "stopped", as the C port's headless build.
#![allow(non_snake_case)]
use super::vfs::VPath;
use crate::host::gs::platform::sys;
use crate::rt::Ctx;
use std::ffi::{c_char, c_void};
use std::sync::{Arc, Mutex};

const ADXT_STAT_STOP: i32 = 0;
const ADXT_STAT_PLAYING: i32 = 3;
const ADXT_STAT_PLAYEND: i32 = 5;
const MAX_PLAYERS: usize = 16;
const FRAME: usize = 18;
const PAN_AUTO: i32 = -128;
const SPU_SIZE: usize = 0x20_0000;
const VOICES: usize = 48;
const OUT_RATE: u32 = 48000;
const SDL_AUDIO_S16: u32 = 0x8010;

#[repr(C)]
struct AudioSpec {
    format: u32,
    channels: i32,
    freq: i32,
}

type GetCallback = unsafe extern "C" fn(*mut c_void, *mut c_void, i32, i32);

struct Sdl {
    create_stream: unsafe extern "C" fn(*const AudioSpec, *const AudioSpec) -> *mut c_void,
    set_get_callback: unsafe extern "C" fn(*mut c_void, Option<GetCallback>, *mut c_void) -> bool,
    bind: unsafe extern "C" fn(u32, *mut c_void) -> bool,
    put: unsafe extern "C" fn(*mut c_void, *const c_void, i32) -> bool,
    gain: unsafe extern "C" fn(*mut c_void, f32) -> bool,
    clear: unsafe extern "C" fn(*mut c_void) -> bool,
    destroy: unsafe extern "C" fn(*mut c_void),
}

// the audio thread calls back with only a pointer: the put function it needs is kept here
static PUT: std::sync::OnceLock<unsafe extern "C" fn(*mut c_void, *const c_void, i32) -> bool> = std::sync::OnceLock::new();

/// What the game can see of a player (plat_sndstate.c's PortAdxView).
#[derive(Clone, Default)]
struct AdxView {
    stat: i32,
    paused: bool,
    left: i32, // vertical blanks to the end; -1 loops
}

#[derive(Default)]
struct Player {
    /// the stream decoded: interleaved 16-bit samples, `channels` per frame
    pcm: Vec<i16>,
    channels: usize,
    rate: u32,
    total: u32,
    looped: bool,
    loop_start: u32,
    loop_end: u32,
    pos: u32,
    stat: i32,
    paused: bool,
    pan: [i32; 2],
    mono: bool,
}

struct PlayerSlot {
    state: Arc<Mutex<Player>>,
    stream: *mut c_void,
    vol: i32,
}

fn be32(p: &[u8]) -> u32 {
    u32::from_be_bytes([p[0], p[1], p[2], p[3]])
}
fn be16(p: &[u8]) -> u32 {
    (p[0] as u32) << 8 | p[1] as u32
}
fn le32(p: &[u8]) -> u32 {
    u32::from_le_bytes([p[0], p[1], p[2], p[3]])
}
fn le16(p: &[u8]) -> u32 {
    p[0] as u32 | (p[1] as u32) << 8
}

/// The high-pass coefficients of an ADX stream, 12 fractional bits, truncated (as vgmstream).
fn adx_coefs(highpass: u32, rate: u32) -> (i32, i32) {
    let z = (2.0 * std::f64::consts::PI * highpass as f64 / rate as f64).cos();
    let a = std::f64::consts::SQRT_2 - z;
    let b = std::f64::consts::SQRT_2 - 1.0;
    let c = (a - ((a + b) * (a - b)).sqrt()) / b;
    ((c * 8192.0) as i32, (c * c * -4096.0) as i32)
}

/// A stream ready to play: decoded samples and where it loops.
pub struct Pcm {
    channels: usize,
    rate: u32,
    samples: Vec<i16>,
    looped: bool,
    loop_start: u32,
    loop_end: u32,
}

impl Pcm {
    fn frames(&self) -> u32 {
        (self.samples.len() / self.channels) as u32
    }

    /// Its length in vertical blanks (59.94 a second), -1 if it loops: what the game is told.
    fn blanks(&self) -> i32 {
        if self.looped {
            return -1;
        }
        let (t, r) = (self.frames() as u64, self.rate as u64);
        if r == 0 { 0 } else { ((t * 60000 + r * 1001 - 1) / (r * 1001)) as i32 + 1 }
    }

    /// An ADX file of the kind the game uses, decoded whole.
    fn from_adx(file: &[u8]) -> Option<Pcm> {
        if file.len() < 0x40 || be16(file) != 0x8000 || file[4] != 3 || file[5] as usize != FRAME || file[6] != 4 || !(1..=2).contains(&file[7]) || file[0x13] & 8 != 0 {
            return None;
        }
        let header = be16(&file[2..]) as usize + 4;
        let channels = file[7] as usize;
        let rate = be32(&file[8..]);
        let mut total = be32(&file[12..]) as usize;
        let (c1, c2) = adx_coefs(be16(&file[16..]), rate);
        let (mut looped, mut loop_start, mut loop_end) = (false, 0, 0);
        if file[0x12] == 4 && header >= 0x38 && be32(&file[0x24..]) != 0 {
            loop_start = be32(&file[0x28..]);
            loop_end = be32(&file[0x30..]);
            looped = loop_end > loop_start && loop_end as usize <= total;
        }
        let frames = (file.len() - header) / (FRAME * channels);
        total = total.min(frames * 32);
        let mut samples = vec![0i16; total * channels];
        let mut hist = [[0i32; 2]; 2];
        for fr in 0..(total + 31) / 32 {
            let mut src = header + fr * FRAME * channels;
            for ch in 0..channels {
                let scale = be16(&file[src..]) as i32 + 1;
                let [mut h1, mut h2] = hist[ch];
                for i in 0..32 {
                    let mut nib = ((file[src + 2 + i / 2] >> if i & 1 != 0 { 0 } else { 4 }) & 15) as i32;
                    if nib & 8 != 0 {
                        nib -= 16;
                    }
                    let v = (nib * scale + ((c1 * h1 + c2 * h2) >> 12)).clamp(-32768, 32767);
                    let at = fr * 32 + i;
                    if at < total {
                        samples[at * channels + ch] = v as i16;
                    }
                    h2 = h1;
                    h1 = v;
                }
                hist[ch] = [h1, h2];
                src += FRAME;
            }
        }
        Some(Pcm { channels, rate, samples, looped, loop_start, loop_end })
    }

    /// A WAV of the extraction (PCM16, mono or stereo; the loop in its `smpl` chunk, end sample inclusive).
    fn from_wav(file: &[u8]) -> Option<Pcm> {
        if file.len() < 12 || &file[0..4] != b"RIFF" || &file[8..12] != b"WAVE" {
            return None;
        }
        let (mut channels, mut rate, mut bits) = (0usize, 0u32, 0u32);
        let mut data: &[u8] = &[];
        let (mut looped, mut loop_start, mut loop_end) = (false, 0, 0);
        let mut o = 12;
        while o + 8 <= file.len() {
            let id = &file[o..o + 4];
            let n = le32(&file[o + 4..]) as usize;
            let body = &file[o + 8..(o + 8 + n).min(file.len())];
            match id {
                b"fmt " if body.len() >= 16 => {
                    if le16(body) != 1 {
                        return None;
                    }
                    channels = le16(&body[2..]) as usize;
                    rate = le32(&body[4..]);
                    bits = le16(&body[14..]);
                }
                b"data" => data = body,
                b"smpl" if body.len() >= 36 + 24 && le32(&body[28..]) > 0 => {
                    looped = true;
                    loop_start = le32(&body[36 + 8..]);
                    loop_end = le32(&body[36 + 12..]) + 1;
                }
                _ => {}
            }
            o += 8 + n + (n & 1);
        }
        if bits != 16 || !(1..=2).contains(&channels) || rate == 0 {
            return None;
        }
        let samples: Vec<i16> = data.chunks_exact(2).map(|b| i16::from_le_bytes([b[0], b[1]])).collect();
        let frames = (samples.len() / channels) as u32;
        looped = looped && loop_end > loop_start && loop_end <= frames;
        Some(Pcm { channels, rate, samples, looped, loop_start, loop_end })
    }
}

impl Player {
    /// Up to `frames` 32-sample frames from `pos` into interleaved stereo; the number of samples.
    fn decode(&mut self, out: &mut [i16], frames: usize) -> usize {
        let mut n = 0;
        let (mut gl, mut gr) = ([0f32; 2], [0f32; 2]);
        for ch in 0..2 {
            let mut pan = if self.pan[ch] != PAN_AUTO {
                self.pan[ch]
            } else if self.channels == 1 {
                0
            } else if ch == 0 {
                -15
            } else {
                15
            };
            if self.mono {
                pan = 0;
            }
            gl[ch] = if pan <= 0 { 1.0 } else { (15 - pan) as f32 / 15.0 };
            gr[ch] = if pan >= 0 { 1.0 } else { (15 + pan) as f32 / 15.0 };
        }
        let mut f = 0;
        while f < frames {
            let end = (if self.looped { self.loop_end } else { self.total }).min((self.pcm.len() / self.channels.max(1)) as u32);
            if self.pos >= end {
                if !self.looped {
                    break;
                }
                self.pos = self.loop_start & !31; // (loop starts are frame-aligned by the encoder)
                continue;
            }
            let count = (end - (self.pos & !31)).min(32);
            for i in (self.pos & 31)..count {
                let at = ((self.pos & !31) + i) as usize * self.channels;
                let (l, r) = if self.channels == 1 {
                    (self.pcm[at] as f32 * gl[0], self.pcm[at] as f32 * gr[0])
                } else {
                    let (a, b) = (self.pcm[at] as f32, self.pcm[at + 1] as f32);
                    (a * gl[0] + b * gl[1], a * gr[0] + b * gr[1])
                };
                out[n * 2] = l.clamp(-32768.0, 32767.0) as i16;
                out[n * 2 + 1] = r.clamp(-32768.0, 32767.0) as i16;
                n += 1;
            }
            self.pos = (self.pos & !31) + 32;
            f += 1;
        }
        n
    }
}

/// SDL's audio thread wants `additional` more bytes from a player.
unsafe extern "C" fn adx_feed(user: *mut c_void, stream: *mut c_void, mut additional: i32, _total: i32) {
    let state = &*(user as *const Mutex<Player>);
    let Ok(mut p) = state.lock() else { return };
    let put = PUT.get().unwrap();
    let mut buf = [0i16; 32 * 2 * 16];
    while additional > 0 && p.stat == ADXT_STAT_PLAYING && !p.paused {
        let n = p.decode(&mut buf, 16);
        if n == 0 {
            p.stat = ADXT_STAT_PLAYEND;
            break;
        }
        put(stream, buf.as_ptr() as *const c_void, (n * 4) as i32);
        additional -= (n * 4) as i32;
    }
}

// ---------------------------------------------------------------------------------------------- effects
#[derive(Clone, Copy, Default)]
struct Bank {
    native: u32, // PC resource id + 1; no Sony HD/SQ/SPU pointers
    set: bool,
    spu: u32,
    hd: u32,
    sq: u32,
    paused: bool,
    volume: f32,
    /// the extraction's folder of this bank's samples (index + 1 in Sound::sfx_folders; 0: none, the
    /// samples come from the sound chip's memory)
    folder: u32,
}

#[derive(Clone, Copy, Default)]
struct Voice {
    on: bool,
    bank: usize,
    id: i32,
    handle: i32,
    pos: u32,
    loop_pos: u32,
    h1: i32,
    h2: i32,
    buf: [i16; 28],
    have: usize,
    frac: f64,
    step: f64,
    last: bool,
    ended: bool,
    prev: i16,
    gl: f32,
    gr: f32,
    fade: f32,
    stopping: bool,
    /// a sample from the extraction's WAVs (index + 1 in Mixer::pcms; 0: decoded from the chip's memory)
    src: u32,
    cursor: u32,
}

struct Mixer {
    /// effect samples read from WAVs: samples, loop start (in samples)
    pcms: Vec<(Vec<i16>, Option<u32>)>,
    spu: Vec<u8>,
    banks: [Bank; 8],
    voices: [Voice; VOICES],
}

impl Mixer {
    fn decode_block(&mut self, k: usize) {
        const F0: [i32; 5] = [0, 60, 115, 98, 122];
        const F1: [i32; 5] = [0, 0, -52, -55, -60];
        if self.voices[k].src != 0 {
            // a WAV sample, in the blocks of 28 the chip would decode, looping where its loop starts
            let v = &mut self.voices[k];
            let (pcm, lp) = &self.pcms[v.src as usize - 1];
            if v.last {
                v.ended = true;
                v.have = 0;
                return;
            }
            v.prev = if v.have != 0 { v.buf[27] } else { 0 };
            for i in 0..28 {
                v.buf[i] = pcm.get(v.cursor as usize + i).copied().unwrap_or(0);
            }
            v.have = 28;
            v.cursor += 28;
            if v.cursor as usize >= pcm.len() {
                match lp {
                    Some(l) => v.cursor = *l,
                    None => v.last = true,
                }
            }
            return;
        }
        let v = &mut self.voices[k];
        if v.last || v.pos as usize + 16 > SPU_SIZE {
            v.ended = true;
            v.have = 0;
            return;
        }
        let p = &self.spu[v.pos as usize..v.pos as usize + 16];
        let mut shift = (p[0] & 15) as i32;
        let mut filter = ((p[0] >> 4) & 7) as usize;
        let flags = p[1];
        if filter > 4 {
            filter = 0;
        }
        if shift > 12 {
            shift = 9;
        }
        if flags & 4 != 0 {
            v.loop_pos = v.pos;
        }
        v.prev = if v.have != 0 { v.buf[27] } else { 0 };
        for i in 0..28 {
            let mut nib = ((p[2 + i / 2] >> ((i & 1) * 4)) & 15) as i32;
            if nib & 8 != 0 {
                nib -= 16;
            }
            let s = (((nib << 12) >> shift) + ((v.h1 * F0[filter] + v.h2 * F1[filter] + 32) >> 6)).clamp(-32768, 32767);
            v.buf[i] = s as i16;
            v.h2 = v.h1;
            v.h1 = s;
        }
        v.have = 28;
        v.pos += 16;
        if flags & 1 != 0 {
            if flags & 2 != 0 && v.loop_pos != 0 {
                v.pos = v.loop_pos;
            } else {
                v.last = true;
            }
        }
    }

    fn mix(&mut self, acc: &mut [f32], n: usize) {
        acc[..n * 2].fill(0.0);
        for k in 0..VOICES {
            if !self.voices[k].on || self.banks[self.voices[k].bank].paused {
                continue;
            }
            for i in 0..n {
                while self.voices[k].frac >= self.voices[k].have as f64 {
                    let have = self.voices[k].have as f64;
                    self.voices[k].frac -= have;
                    self.decode_block(k);
                    if self.voices[k].ended {
                        break;
                    }
                }
                let v = &mut self.voices[k];
                if v.ended {
                    v.on = false;
                    break;
                }
                let ip = v.frac as usize;
                let a = if ip == 0 { v.prev } else { v.buf[ip - 1] } as f32;
                let b = v.buf[ip] as f32;
                let s = (a + (b - a) * (v.frac - ip as f64) as f32) * v.fade;
                acc[i * 2] += s * v.gl;
                acc[i * 2 + 1] += s * v.gr;
                v.frac += v.step;
                if v.stopping {
                    v.fade -= 1.0 / (0.008 * OUT_RATE as f32); // 8 ms
                    if v.fade <= 0.0 {
                        v.on = false;
                        break;
                    }
                }
            }
        }
    }
}

unsafe extern "C" fn se_mix(user: *mut c_void, stream: *mut c_void, additional: i32, _total: i32) {
    let state = &*(user as *const Mutex<Mixer>);
    let Ok(mut m) = state.lock() else { return };
    let put = PUT.get().unwrap();
    let mut frames = (additional / 4) as usize;
    let mut acc = [0f32; 512 * 2];
    let mut out = [0i16; 512 * 2];
    while frames > 0 {
        let n = frames.min(512);
        m.mix(&mut acc, n);
        for i in 0..n * 2 {
            out[i] = acc[i].clamp(-32768.0, 32767.0) as i16;
        }
        put(stream, out.as_ptr() as *const c_void, (n * 4) as i32);
        frames -= n;
    }
}

// ---------------------------------------------------------------------------------------------- the device
pub struct Sound {
    sdl: Sdl,
    device: u32,
    players: Vec<PlayerSlot>,
    views: Vec<AdxView>,
    mono: bool,
    mixer: Arc<Mutex<Mixer>>,
    se_stream: *mut c_void,
    se_mono: bool,
    se_gain: f32,
    verbose: bool,
    /// the extraction's effect samples (assets/audio/sfx): folder names, and a bank's layout (sample offsets
    /// relative to the first, rates) -> folder index
    sfx_folders: Vec<String>,
    sfx_index: std::collections::HashMap<Vec<(u32, u32)>, u32>,
    sfx_loops: std::collections::HashMap<(u32, u32), u32>,
    sfx_loaded: std::collections::HashMap<(u32, u32), u32>,
    sfx_root: Option<std::path::PathBuf>,
    native_banks: std::collections::HashMap<u32,super::assets::NativeSoundBank>,
}

impl Sound {
    /// Opens the default playback device through the window's SDL library.
    pub fn open(lib: *mut c_void) -> Result<Sound, String> {
        macro_rules! f {
            ($n:literal, $t:ty) => {{
                let p = unsafe { sys::sym(lib, $n) };
                if p.is_null() {
                    return Err(format!("{} not found", $n));
                }
                unsafe { std::mem::transmute::<*mut c_void, $t>(p) }
            }};
        }
        let init = f!("SDL_InitSubSystem", unsafe extern "C" fn(u32) -> bool);
        let open = f!("SDL_OpenAudioDevice", unsafe extern "C" fn(u32, *const AudioSpec) -> u32);
        let resume = f!("SDL_ResumeAudioDevice", unsafe extern "C" fn(u32) -> bool);
        let driver = f!("SDL_GetCurrentAudioDriver", unsafe extern "C" fn() -> *const c_char);
        let error = f!("SDL_GetError", unsafe extern "C" fn() -> *const c_char);
        let err = || unsafe { std::ffi::CStr::from_ptr(error()).to_string_lossy().into_owned() };
        if !unsafe { init(0x10) } {
            return Err(format!("SDL audio: {}", err()));
        }
        let device = unsafe { open(0xFFFF_FFFF, std::ptr::null()) };
        if device == 0 {
            return Err(format!("no sound device: {}", err()));
        }
        unsafe { resume(device) };
        let name = unsafe { std::ffi::CStr::from_ptr(driver()).to_string_lossy().into_owned() };
        eprintln!("bt3: sound: {name}");
        let sdl = Sdl {
            create_stream: f!("SDL_CreateAudioStream", unsafe extern "C" fn(*const AudioSpec, *const AudioSpec) -> *mut c_void),
            set_get_callback: f!("SDL_SetAudioStreamGetCallback", unsafe extern "C" fn(*mut c_void, Option<GetCallback>, *mut c_void) -> bool),
            bind: f!("SDL_BindAudioStream", unsafe extern "C" fn(u32, *mut c_void) -> bool),
            put: f!("SDL_PutAudioStreamData", unsafe extern "C" fn(*mut c_void, *const c_void, i32) -> bool),
            gain: f!("SDL_SetAudioStreamGain", unsafe extern "C" fn(*mut c_void, f32) -> bool),
            clear: f!("SDL_ClearAudioStream", unsafe extern "C" fn(*mut c_void) -> bool),
            destroy: f!("SDL_DestroyAudioStream", unsafe extern "C" fn(*mut c_void)),
        };
        let _ = PUT.set(sdl.put);
        let mixer = Arc::new(Mutex::new(Mixer {
            pcms: Vec::new(), spu: if std::env::var_os("BT3_PC_ONLY").is_some() {Vec::new()}else{vec![0; SPU_SIZE]}, banks: [Bank::default(); 8], voices: [Voice::default(); VOICES] }));
        let spec = AudioSpec { format: SDL_AUDIO_S16, channels: 2, freq: OUT_RATE as i32 };
        let se_stream = unsafe { (sdl.create_stream)(&spec, std::ptr::null()) };
        if !se_stream.is_null() {
            unsafe {
                (sdl.set_get_callback)(se_stream, Some(se_mix), Arc::as_ptr(&mixer) as *mut c_void);
                (sdl.bind)(device, se_stream);
            }
        }
        // the effects against the music: the console's chain (sequence volume, velocity, timbre volume 100 of 127
        // each, the voice volume at half scale), BT3_SE_GAIN=<percent> to tune
        let se_gain = std::env::var("BT3_SE_GAIN").ok().and_then(|v| v.parse::<f32>().ok()).map_or(0.25, |p| p / 100.0);
        Ok(Sound {
            sdl,
            device,
            players: Vec::new(),
            views: Vec::new(),
            mono: false,
            mixer,
            se_stream,
            se_mono: false,
            se_gain,
            verbose: std::env::var("BT3_SND_VERBOSE").is_ok(),
            sfx_folders: Vec::new(),
            sfx_index: Default::default(),
            sfx_loops: Default::default(),
            sfx_loaded: Default::default(),
            sfx_root: None,
            native_banks: Default::default(),
        })
    }

    /// Reads assets/audio/sfx/samples.csv (folder, sample, offset in the bank file, rate, samples, loop start):
    /// which folder of WAVs belongs to which bank layout.
    fn sfx_open(&mut self, root: std::path::PathBuf) {
        if self.sfx_root.is_some() || std::env::var("BT3_ORIGINAL_AUDIO").is_ok() {
            return;
        }
        let Ok(text) = super::vfs::read_to_string(root.join("samples.csv")) else { return };
        let mut layouts: std::collections::BTreeMap<String, Vec<(u32, u32, u32)>> = Default::default();
        let mut loops = Vec::new();
        for l in text.lines().skip(1) {
            let f: Vec<&str> = l.split(',').collect();
            if f.len() < 6 {
                continue;
            }
            let (Ok(id), Ok(off), Ok(rate)) = (f[1].parse::<u32>(), f[2].parse::<u32>(), f[3].parse::<u32>()) else { continue };
            layouts.entry(f[0].to_string()).or_default().push((id, off, rate));
            if let Ok(lp) = f[5].parse::<u32>() {
                loops.push((f[0].to_string(), id, lp));
            }
        }
        for (name, mut v) in layouts {
            v.sort();
            let first = v.first().map_or(0, |x| x.1);
            let key: Vec<(u32, u32)> = v.iter().map(|x| (x.1 - first, x.2)).collect();
            let idx = self.sfx_folders.len() as u32;
            self.sfx_folders.push(name);
            self.sfx_index.entry(key).or_insert(idx);
        }
        for (name, id, lp) in loops {
            if let Some(i) = self.sfx_folders.iter().position(|x| *x == name) {
                self.sfx_loops.insert((i as u32, id), lp);
            }
        }
        if self.verbose {
            eprintln!("se: {} banks of WAV samples in {}", self.sfx_folders.len(), root.display());
        }
        self.sfx_root = Some(root);
    }

    /// The folder of a bank, from its sample table in the game's memory (Vagi chunk of its header part).
    fn sfx_folder_of(&self, c: &Ctx, hd: u32) -> u32 {
        if self.sfx_index.is_empty() || hd == 0 {
            return 0;
        }
        let tag = *b"IECSigaV";
        let mut vagi = None;
        let mut o = 0;
        while o + 16 <= 0x8000 {
            let mut t = [0u8; 8];
            c.m.read_bytes(hd + o, &mut t);
            if t == tag {
                vagi = Some(hd + o);
                break;
            }
            o += 4;
        }
        let Some(vagi) = vagi else { return 0 };
        let max = c.m.r32(vagi + 0x0C).min(4096);
        let mut key = Vec::new();
        let mut first = None;
        for k in 0..=max {
            let r = vagi + c.m.r32(vagi + 0x10 + 4 * k);
            let off = c.m.r32(r);
            let rate = c.m.r16(r + 4) as u32;
            let f0 = *first.get_or_insert(off);
            key.push((off.wrapping_sub(f0), rate));
        }
        self.sfx_index.get(&key).map_or(0, |&i| i + 1)
    }

    /// The mixer's index (+ 1) of sample `vag` of folder `f`, read from its WAV the first time; 0 if missing.
    fn sfx_sample(&mut self, f: u32, vag: u32) -> u32 {
        if let Some(&i) = self.sfx_loaded.get(&(f, vag)) {
            return i;
        }
        let path = self.sfx_root.as_ref().unwrap().join(&self.sfx_folders[f as usize]).join(format!("sample_{vag:03}.wav"));
        let i = match super::vfs::read(&path).ok().and_then(|b| Pcm::from_wav(&b)) {
            Some(p) if p.channels == 1 => {
                let lp = self.sfx_loops.get(&(f, vag)).copied().or(if p.looped { Some(p.loop_start) } else { None });
                let mut m = self.mixer.lock().unwrap();
                m.pcms.push((p.samples, lp));
                m.pcms.len() as u32
            }
            _ => 0,
        };
        self.sfx_loaded.insert((f, vag), i);
        i
    }

    /// Player i, made when first needed.
    fn ensure(&mut self, i: usize) {
        while self.players.len() <= i {
            self.players.push(PlayerSlot { state: Arc::new(Mutex::new(Player { pan: [PAN_AUTO; 2], mono: self.mono, ..Default::default() })), stream: std::ptr::null_mut(), vol: 0 });
            self.views.push(AdxView::default());
        }
    }

    fn apply_volume(&self, i: usize) {
        let p = &self.players[i];
        if !p.stream.is_null() {
            let g = if p.vol <= -960 { 0.0 } else { 10f32.powf(p.vol as f32 / 200.0) };
            unsafe { (self.sdl.gain)(p.stream, g) };
        }
    }

    fn stop(&mut self, i: usize) {
        let s = std::mem::replace(&mut self.players[i].stream, std::ptr::null_mut());
        if !s.is_null() {
            unsafe { (self.sdl.destroy)(s) }; // (waits for a running callback; our lock is not held here)
        }
        let mut p = self.players[i].state.lock().unwrap();
        p.pcm = Vec::new();
        p.stat = ADXT_STAT_STOP;
    }

    fn start(&mut self, i: usize, pcm: Pcm, name: &str) {
        self.stop(i);
        let rate = pcm.rate;
        let stream = {
            let mut p = self.players[i].state.lock().unwrap();
            p.channels = pcm.channels;
            p.rate = rate;
            p.total = pcm.frames();
            p.looped = pcm.looped;
            p.loop_start = pcm.loop_start;
            p.loop_end = pcm.loop_end;
            p.pos = 0;
            p.mono = self.mono;
            if self.verbose {
                eprintln!("adx: start {name}: {} ch, {} Hz, {:.1} s, loop {}, volume {}", p.channels, rate, p.total as f64 / rate as f64, p.looped, self.players[i].vol);
            }
            p.pcm = pcm.samples;
            p.stat = ADXT_STAT_PLAYING;
            let spec = AudioSpec { format: SDL_AUDIO_S16, channels: 2, freq: rate as i32 };
            unsafe { (self.sdl.create_stream)(&spec, std::ptr::null()) }
        };
        if stream.is_null() {
            return;
        }
        self.players[i].stream = stream;
        self.apply_volume(i);
        unsafe {
            (self.sdl.set_get_callback)(stream, Some(adx_feed), Arc::as_ptr(&self.players[i].state) as *mut c_void);
            (self.sdl.bind)(self.device, stream);
        }
    }

    /// One vertical blank has passed (Port_AdxTick): the game's view of the streams.
    pub fn tick(&mut self) {
        for v in self.views.iter_mut() {
            if v.stat == ADXT_STAT_PLAYING && !v.paused && v.left > 0 {
                v.left -= 1;
                if v.left == 0 {
                    v.stat = ADXT_STAT_PLAYEND;
                }
            }
        }
    }

    // ------------------------------------------------------------------------------------------ effects
    fn lookup(&self, c: &Ctx, b: &Bank, id: i32) -> Option<(u32, u32, u32)> {
        // the chunk "SCEI<name>" inside a table (searched over at most `limit` bytes)
        let chunk = |p: u32, name: &[u8; 4], limit: u32| -> Option<u32> {
            if p == 0 {
                return None;
            }
            let tag = [b'I', b'E', b'C', b'S', name[3], name[2], name[1], name[0]];
            let mut o = 0;
            while o + 16 <= limit {
                let mut t = [0u8; 8];
                c.m.read_bytes(p + o, &mut t);
                if t == tag {
                    return Some(p + o);
                }
                o += 4;
            }
            None
        };
        let r = |a: u32, n: usize| -> Vec<u8> {
            let mut v = vec![0u8; n];
            c.m.read_bytes(a, &mut v);
            v
        };
        let vagi = chunk(b.hd, b"Vagi", 0x8000)?;
        let sesq = chunk(b.sq, b"Sesq", 0x4000);
        let setb = chunk(b.hd, b"Setb", 0x8000);
        let mut timbre = id as u32;
        if let Some(sesq) = sesq {
            let seq = sesq + le32(&r(sesq + 0x34 + 4 * id as u32, 4));
            let s = r(seq + 0x11, 3);
            if s[0] == 0x90 {
                timbre = s[2] as u32;
            }
        }
        let mut vag = timbre;
        if let Some(setb) = setb {
            let h = r(setb + 0x20, 4);
            if timbre <= h[2] as u32 {
                vag = le16(&r(setb + 0x24 + timbre * h[0] as u32, 2));
            }
        }
        if vag > le32(&r(vagi + 0x0C, 4)) {
            return None;
        }
        let o = le32(&r(vagi + 0x10 + 4 * vag, 4));
        let rec = r(vagi + o, 6);
        let start = b.spu + le32(&rec);
        let rate = le16(&rec[4..]);
        if start as usize + 16 <= SPU_SIZE && rate != 0 { Some((start, rate, vag)) } else { None }
    }

    fn play(&mut self, c: &Ctx, mask: u32, id: i32, volume: i32, pan: i32, pitch: i32, handle: i32) {
        let Some(slot) = (0..8).find(|i| mask & (1 << i) != 0) else { return };
        let bank = self.mixer.lock().unwrap().banks[slot];
        if !bank.set {
            return;
        }
        let (start,rate,src)=if bank.native!=0 {
            let key=bank.native-1;
            let Some(data)=self.native_banks.get(&key) else {return};
            let Some(Some(sample))=data.effects.get(id as usize) else {return};
            let sample=*sample;
            let record=&data.samples[sample];
            let rate=record.rate;
            let cache_key=(0x8000_0000|key,sample as u32);
            let src=if let Some(&src)=self.sfx_loaded.get(&cache_key) {src}else {
                let path=record.path.clone();
                let frames=record.frames as usize;
                let lp=record.loop_start;
                let mut pcm=super::vfs::read(&path).ok().and_then(|v|Pcm::from_wav(&v))
                    .filter(|p|p.channels==1 && p.samples.len()>=frames)
                    .unwrap_or_else(||panic!("bt3: missing or invalid native sound sample {}",path.display()));
                pcm.samples.truncate(frames);
                let mut m=self.mixer.lock().unwrap();
                m.pcms.push((pcm.samples,lp));
                let src=m.pcms.len() as u32;
                self.sfx_loaded.insert(cache_key,src);
                src
            };
            (0,rate,src)
        }else {
            if std::env::var_os("BT3_PC_ONLY").is_some() {
                panic!("bt3: legacy sound bank reached in PC-only playback");
            }
            let Some((start,rate,vag))=self.lookup(c,&bank,id) else {return};
            let src=if bank.folder!=0 {self.sfx_sample(bank.folder-1,vag)}else{0};
            (start,rate,src)
        };
        let theta = if self.se_mono { 0.5 } else { pan as f64 / 127.0 } * std::f64::consts::FRAC_PI_2;
        let k = (volume as f32 / 127.0) * bank.volume * self.se_gain;
        let mut m = self.mixer.lock().unwrap();
        let pick = match m.voices.iter().position(|v| !v.on) {
            Some(k) => k,
            None => m.voices.iter().position(|v| v.stopping).unwrap_or(0),
        };
        m.voices[pick] = Voice {
            on: true,
            bank: slot,
            id,
            handle,
            pos: start,
            src,
            step: rate as f64 * 2f64.powf(pitch as f64 / 1200.0) / OUT_RATE as f64,
            fade: 1.0,
            gl: (theta.cos() * std::f64::consts::SQRT_2) as f32 * k,
            gr: (theta.sin() * std::f64::consts::SQRT_2) as f32 * k,
            ..Default::default()
        };
        if self.verbose {
            eprintln!("se: play bank {slot} effect {id}: {rate} Hz, volume {volume}, pan {pan}, pitch {pitch}, handle {handle}");
        }
    }

    /// Stops voices: by bank mask (id < 0, handle < 0), by bank and number, or by handle.
    fn stop_voices(&mut self, mask: u32, id: i32, handle: i32) {
        let mut m = self.mixer.lock().unwrap();
        for v in m.voices.iter_mut() {
            if v.on && ((handle >= 0 && v.handle == handle) || (handle < 0 && mask & (1 << v.bank) != 0 && (id < 0 || v.id == id))) {
                v.stopping = true;
            }
        }
    }
}

impl Drop for Sound {
    fn drop(&mut self) {
        for i in 0..self.players.len() {
            self.stop(i);
        }
        if !self.se_stream.is_null() {
            unsafe { (self.sdl.destroy)(self.se_stream) };
        }
    }
}

// ---------------------------------------------------------------------------------------------- the host side
/// The sound device, opened with the window's first sound call (None headless or without a device).
fn sound(c: &mut Ctx) -> Option<&mut Sound> {
    if !c.host.snd_checked {
        let lib = c.host.gs.as_ref().and_then(|g| g.gpu.as_ref()).and_then(|g| g.surface.sdl_lib());
        if lib.is_some() || c.host.gs_checked {
            c.host.snd_checked = true;
        }
        if let Some(lib) = lib {
            if std::env::var("BT3_NOSOUND").is_err() {
                match Sound::open(lib) {
                    Ok(s) => c.host.snd = Some(Box::new(s)),
                    Err(e) => eprintln!("bt3: {e}"),
                }
            }
        }
    }
    c.host.snd.as_deref_mut()
}

/// ADXT_Create: a new player (the handle the game keeps is host memory, as before; the players are counted
/// here whether or not there is a sound device, as the C port's gPortAdxCount).
pub fn adxt_create(c: &mut Ctx, handle: u32) {
    let i = c.host.adx_handles.len().min(MAX_PLAYERS - 1);
    if i == c.host.adx_handles.len() {
        c.host.adx_handles.push(handle);
    } else {
        c.host.adx_handles[i] = handle;
    }
    if let Some(s) = sound(c) {
        s.ensure(i);
        s.stop(i);
        s.views[i] = AdxView::default();
        s.players[i].vol = 0;
        s.players[i].state.lock().unwrap().pan = [PAN_AUTO; 2];
    }
}

/// The sound device and the player of `handle`.
fn player(c: &mut Ctx, handle: u32) -> Option<(&mut Sound, usize)> {
    let i = c.host.adx_handles.iter().position(|&h| h == handle)?;
    let s = sound(c)?;
    s.ensure(i);
    Some((s, i))
}

/// A stream the game starts: the file rebuilt from the open-format extraction (assets/files/<file>/stream.wav, so an
/// edited WAV is heard) when there is one, else the older WAV export (assets/audio/streams/<file>.wav), else the
/// game's ADX file decoded (BT3_ORIGINAL_AUDIO=1: always the ADX file).
fn load_stream(c: &Ctx, ptid: i32, flid: i32, fname: Option<&[u8]>) -> Option<(Pcm, String)> {
    let rel = super::files::file_rel(c, ptid, flid, fname);
    if std::env::var_os("BT3_PC_ONLY").is_some() {
        // The native WAV export retains exact playable length and RIFF smpl
        // loop points. The reversible files/ export includes ADX frame padding
        // and omits smpl; it is not the native stream playback source.
        for r in [rel.clone(),rel.replacen("disc/DATA/","disc/",1)] {
            let path=super::files::assets_root(c).join("audio/streams").join(format!("{r}.wav"));
            if let Some(p)=super::vfs::read(&path).ok().and_then(|b|Pcm::from_wav(&b)) {
                return Some((p,path.display().to_string()));
            }
        }
        panic!("bt3: native WAV stream missing or invalid: {rel}; ADX reconstruction disabled");
    }
    if std::env::var("BT3_ORIGINAL_AUDIO").is_err() {
        // Prefer the editable PCM itself: no reconstruction or ADX re-encoding.
        let wav=super::assets::file_dir(&super::files::assets_root(c),&rel).join("stream.wav");
        if let Some(p)=super::vfs::read(&wav).ok().and_then(|f|Pcm::from_wav(&f)) {
            return Some((p,wav.display().to_string()));
        }
        if let Some(p) = super::files::rebuilt(c, &rel).and_then(|b| Pcm::from_adx(&b)) {
            return Some((p, format!("{rel} (rebuilt from assets/files)")));
        }
        // (the movies' sound: audio/streams/disc/<NAME>.ADX.wav)
        for r in [rel.clone(), rel.replacen("disc/DATA/", "disc/", 1)] {
            let wav = super::files::assets_root(c).join("audio/streams").join(format!("{r}.wav"));
            if let Some(p) = super::vfs::read(&wav).ok().and_then(|f| Pcm::from_wav(&f)) {
                return Some((p, wav.display().to_string()));
            }
        }
    }
    let path = super::files::file_path(c, ptid, flid, fname)?;
    let p = Pcm::from_adx(&super::vfs::read(&path).ok()?)?;
    Some((p, path.display().to_string()))
}

pub fn adxt_start(c: &mut Ctx, handle: u32, ptid: i32, flid: i32, fname: Option<Vec<u8>>) {
    let loaded = if c.host.snd.is_some() || sound(c).is_some() { load_stream(c, ptid, flid, fname.as_deref()) } else { None };
    let Some((s, i)) = player(c, handle) else { return };
    let Some((pcm, name)) = loaded else {
        eprintln!("bt3: sound file not found (partition {ptid}, file {flid})");
        s.views[i].stat = ADXT_STAT_STOP;
        return;
    };
    let blanks = pcm.blanks();
    s.views[i].stat = if blanks != 0 { ADXT_STAT_PLAYING } else { ADXT_STAT_STOP };
    s.views[i].left = blanks;
    s.start(i, pcm, &name);
}

pub fn adxt_stop(c: &mut Ctx, handle: u32) {
    if let Some((s, i)) = player(c, handle) {
        s.stop(i);
        s.views[i].stat = ADXT_STAT_STOP;
    }
}

pub fn adxt_pause(c: &mut Ctx, handle: u32, on: bool) {
    if let Some((s, i)) = player(c, handle) {
        s.views[i].paused = on;
        let stream = s.players[i].stream;
        s.players[i].state.lock().unwrap().paused = on;
        if on && !stream.is_null() {
            unsafe { (s.sdl.clear)(stream) }; // what is queued would still be heard
        }
    }
}

pub fn adxt_volume(c: &mut Ctx, handle: u32, vol: i32) {
    if let Some((s, i)) = player(c, handle) {
        s.players[i].vol = vol;
        s.apply_volume(i);
    }
}

pub fn adxt_pan(c: &mut Ctx, handle: u32, ch: i32, pan: i32) {
    if let Some((s, i)) = player(c, handle) {
        if (0..2).contains(&ch) {
            s.players[i].state.lock().unwrap().pan[ch as usize] = pan;
        }
    }
}

pub fn adxt_mono(c: &mut Ctx, on: bool) {
    if let Some(s) = sound(c) {
        s.mono = on;
        for p in &s.players {
            p.state.lock().unwrap().mono = on;
        }
    }
}

/// What ADXT_GetStat answers: with sound, the stream's state counted in vertical blanks; headless, stopped.
pub fn adxt_stat(c: &mut Ctx, handle: u32) -> i32 {
    match player(c, handle) {
        Some((s, i)) => s.views[i].stat,
        None => ADXT_STAT_STOP,
    }
}

pub fn tick(c: &mut Ctx) {
    if let Some(s) = c.host.snd.as_deref_mut() {
        s.tick();
    }
}

fn pc_bank_id(c:&Ctx,file:u32)->Option<u32> {
    if file==0 {return None;}
    let mut magic=[0;8]; c.m.read_bytes(file,&mut magic);
    if &magic==b"BT3SFX1\0" {Some(c.m.r32(file+8))}else{None}
}

fn pc_bank_slot(c:&Ctx,mask:u32)->u32 {
    let index=(0..8).find(|i|mask&(1<<i)!=0).unwrap_or(0);
    c.m.r32(crate::gen::sym::gSndMgr)+index*0x4c
}

fn install_pc_bank(c:&mut Ctx,mask:u32,resource:u32) {
    use crate::gen::{funcs as F,sym};
    let dir=super::assets::file_dir(&super::files::assets_root(c),&format!("pzs3us1/{resource:05}.bin"));
    let data=super::assets::native_sound_bank(&dir)
        .unwrap_or_else(|e|panic!("bt3: native sound bank {resource}: {e}"));
    if data.resource!=resource {panic!("bt3: native bank resource mismatch");}
    let bank=pc_bank_slot(c,mask);
    // Preserve only the small state read by game code (loaded, max id, loop
    // flags). Native samples require no SPU/IOP allocations or deferred DMA.
    F::memset(c,bank,0,0x4c);
    c.m.w32(bank,u32::MAX);
    c.m.w8(bank+0x38,(data.samples.len()-1) as u8);
    for (i,sample) in data.samples.iter().enumerate() {
        if sample.queue_loop!=0 && i+1<data.samples.len() {
            let at=bank+0x39+(i/8) as u32;
            c.m.w8(at,c.m.r8(at)|(1<<(i%8)));
        }
    }
    let slot=((bank-c.m.r32(sym::gSndMgr))/0x4c) as usize;
    if let Some(s)=sound(c) {
        s.native_banks.insert(resource,data);
        let mut m=s.mixer.lock().unwrap();
        let volume=if m.banks[slot].volume==0.0 {1.0}else{m.banks[slot].volume};
        m.banks[slot]=Bank {set:true,native:resource+1,volume,..Default::default()};
        if s.verbose {eprintln!("se: native bank {slot}: resource {resource}, direct WAV; no SPU/IOP upload");}
    }
}

pub fn Snd_LoadBank(c:&mut Ctx,mask:u32,file:u32,sync:i32) {
    if let Some(resource)=pc_bank_id(c,file) {
        if crate::gen::funcs::Snd_IsBankLoaded(c,mask)==0 {install_pc_bank(c,mask,resource);}
    }else {crate::gen::funcs::__real_Snd_LoadBank(c,mask,file,sync);}
}

pub fn Snd_ReloadBank(c:&mut Ctx,mask:u32,file:u32,sync:i32) {
    if let Some(resource)=pc_bank_id(c,file) {
        if matches!(mask,8|16|32) && crate::gen::funcs::Snd_IsBankLoaded(c,mask)!=0 {
            crate::gen::funcs::Snd_StopBank(c,mask as i32);
            Snd_UnloadBank(c,mask);
            install_pc_bank(c,mask,resource);
        }
    }else if matches!(mask,8|16|32) && c.m.r32(pc_bank_slot(c,mask))==u32::MAX {
        // Developer overrides can replace a PC bank with a legacy bank. It
        // needs fresh allocations rather than reusing nonexistent IOP buffers.
        Snd_UnloadBank(c,mask);
        crate::gen::funcs::__real_Snd_LoadBank(c,mask,file,sync);
    }else {crate::gen::funcs::__real_Snd_ReloadBank(c,mask,file,sync);}
}

pub fn Snd_UnloadBank(c:&mut Ctx,mask:u32) {
    if c.m.r32(pc_bank_slot(c,mask))==u32::MAX {
        crate::gen::funcs::Snd_RpcFreeBank(c,mask as i32);
        let bank=pc_bank_slot(c,mask);
        crate::gen::funcs::memset(c,bank,0,0x4c);
    }else {crate::gen::funcs::__real_Snd_UnloadBank(c,mask);}
}

/// sceSdRemote 0x80D0: `size` bytes at `iop` into the sound chip's memory at `spu`.
pub fn spu_upload(c: &mut Ctx, iop: u32, spu: u32, size: u32) {
    if std::env::var_os("BT3_PC_ONLY").is_some() {
        panic!("bt3: SPU upload attempted in PC-only mode");
    }
    let mut data = vec![0u8; size as usize];
    if iop == 0 || spu as usize >= SPU_SIZE || size as usize > SPU_SIZE - spu as usize {
        return;
    }
    c.m.read_bytes(iop, &mut data);
    if let Some(s) = sound(c) {
        s.mixer.lock().unwrap().spu[spu as usize..spu as usize + size as usize].copy_from_slice(&data);
    }
}

/// sceSifCallRpc to SOUNDS.IRX: one command of the effects driver.
pub fn rpc(c: &mut Ctx, fno: i32, send: u32) {
    let w = |c: &Ctx, i: u32| c.m.r32(send + 4 * i);
    let args = [w(c, 0), w(c, 1), w(c, 2), w(c, 3)];
    let mut queue = Vec::new();
    if fno == 0xD {
        let count = args[0].min(32);
        queue = vec![0u8; count as usize * 12];
        c.m.read_bytes(send + 4, &mut queue);
    }
    if sound(c).map_or(true, |s| s.se_stream.is_null()) {
        return;
    }
    let root = super::files::assets_root(c).join("audio/sfx");
    let mut boxed = c.host.snd.take().unwrap();
    let s = &mut *boxed;
    if std::env::var_os("BT3_PC_ONLY").is_none() {s.sfx_open(root);}
    match fno {
        0x0 | 0x2 => s.mixer.lock().unwrap().voices.iter_mut().for_each(|v| v.on = false), // INIT, RESET
        0x3 => {
            // SET_BANK: mask, where the samples are, the header table, the sequence table
            if let Some(slot) = (0..8).find(|i| args[0] & (1 << i) != 0) {
                let mut m = s.mixer.lock().unwrap();
                let b = &mut m.banks[slot];
                b.set = true;
                b.spu = args[1];
                b.hd = args[2];
                b.sq = args[3];
                b.paused = false;
                if b.volume == 0.0 {
                    b.volume = 1.0;
                }
                drop(m);
                let folder = s.sfx_folder_of(c, args[2]);
                s.mixer.lock().unwrap().banks[slot].folder = folder;
                if s.verbose {
                    eprintln!("se: bank {slot}: {}", if folder == 0 { "samples from the sound chip's memory".to_string() } else { format!("WAVs of {}", s.sfx_folders[folder as usize - 1]) });
                }
            }
        }
        0x4 => {
            // FREE_BANK
            let mut m = s.mixer.lock().unwrap();
            for i in 0..8 {
                if args[0] & (1 << i) != 0 {
                    m.banks[i].set = false;
                }
            }
            for v in m.voices.iter_mut() {
                if args[0] & (1 << v.bank) != 0 {
                    v.on = false;
                }
            }
        }
        0x5 => s.se_mono = args[0] != 0,
        0x6 => {
            let mut m = s.mixer.lock().unwrap();
            for i in 0..8 {
                if args[0] & (1 << i) != 0 {
                    m.banks[i].paused = args[1] != 0;
                }
            }
        }
        0x7 => {
            let mut m = s.mixer.lock().unwrap();
            for i in 0..8 {
                if args[0] & (1 << i) != 0 {
                    m.banks[i].volume = args[1] as i32 as f32 / 127.0;
                }
            }
        }
        0xA => s.stop_voices(args[0], -1, -1),
        0xD => {
            // QUEUE: 12-byte commands { type, loop, u16 handle, bank, id, volume, pan, s32 pitch }
            for q in queue.chunks_exact(12) {
                match q[0] {
                    0 => s.play(c, q[4] as u32, q[5] as i32, q[6] as i32, q[7] as i32, le32(&q[8..]) as i32, le16(&q[2..]) as i32),
                    1 => s.stop_voices(q[4] as u32, q[5] as i32, -1),
                    2 => s.stop_voices(0, -1, le16(&q[2..]) as i32),
                    _ => {}
                }
            }
        }
        _ => {}
    }
    c.host.snd = Some(boxed);
}
