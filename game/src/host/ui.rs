//! The port's own screens drawn over the game's picture: the save slot list and the "saving" indicator
//! (host/saves.rs). A software canvas (1280 x 896, twice the TV picture, RGBA with alpha) that the GPU renderer
//! lays over the game's picture when it is shown (gs/gpu.rs), and blends into its screenshots. Text uses the font
//! made by tools/assets/make_ui_font.py (ui_font.png / ui_font.txt).
use std::sync::Mutex;

pub const W: usize = 1280;
pub const H: usize = 896;

/// What the renderer shows over the game: the canvas and a counter that changes when it does.
pub struct Overlay {
    pub visible: bool,
    pub serial: u64,
    pub rgba: Vec<u8>,
}

pub static OVERLAY: Mutex<Overlay> = Mutex::new(Overlay { visible: false, serial: 0, rgba: Vec::new() });

struct Font {
    cw: usize,
    ch: usize,
    base: usize,
    xoff: usize,
    yoff: usize,
    alpha: Vec<u8>, // one byte per pixel of the atlas
    aw: usize,
    glyphs: Vec<(char, usize)>, // char, advance; the index is the cell
}

static FONT: Mutex<Option<Font>> = Mutex::new(None);

fn font() -> std::sync::MutexGuard<'static, Option<Font>> {
    let mut f = FONT.lock().unwrap();
    if f.is_none() {
        let png = super::assets::png_read(include_bytes!("ui_font.png")).expect("ui_font.png");
        let mut lines = include_str!("ui_font.txt").lines();
        let head: Vec<usize> = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let glyphs = lines
            .filter_map(|l| {
                let mut it = l.split_whitespace();
                let c = char::from_u32(it.next()?.parse().ok()?)?;
                Some((c, it.next()?.parse().ok()?))
            })
            .collect();
        let alpha = match &png.palette {
            Some(pal) => png.data.iter().map(|&i| pal.get(i as usize).map_or(0, |c| c[3])).collect(),
            None => png.data.chunks_exact(4).map(|c| c[3]).collect(),
        };
        *f = Some(Font { cw: head[0], ch: head[1], base: head[2], xoff: head[3], yoff: head[4], alpha, aw: png.w as usize, glyphs });
    }
    f
}

/// A canvas to draw a screen on.
pub struct Canvas {
    pub px: Vec<u8>,
}

impl Canvas {
    pub fn new() -> Canvas {
        Canvas { px: vec![0; W * H * 4] }
    }

    fn blend(&mut self, x: i32, y: i32, c: [u8; 4], a: u32) {
        if x < 0 || y < 0 || x as usize >= W || y as usize >= H || a == 0 {
            return;
        }
        let o = (y as usize * W + x as usize) * 4;
        let a = a * c[3] as u32 / 255;
        let d = &mut self.px[o..o + 4];
        let da = d[3] as u32;
        let oa = a + da * (255 - a) / 255;
        if oa == 0 {
            return;
        }
        for k in 0..3 {
            d[k] = ((c[k] as u32 * a + d[k] as u32 * da * (255 - a) / 255) / oa) as u8;
        }
        d[3] = oa as u8;
    }

    /// A filled rectangle with rounded corners (radius r).
    pub fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, r: i32, c: [u8; 4]) {
        for yy in y..y + h {
            for xx in x..x + w {
                // distance into the corner circles, for a soft edge
                let cx = if xx < x + r { x + r } else if xx >= x + w - r { x + w - r - 1 } else { xx };
                let cy = if yy < y + r { y + r } else if yy >= y + h - r { y + h - r - 1 } else { yy };
                let (dx, dy) = ((xx - cx) as f32, (yy - cy) as f32);
                let d = (dx * dx + dy * dy).sqrt() - r as f32 + 0.5;
                let a = if r == 0 || d <= 0.0 { 255 } else if d >= 1.0 { 0 } else { ((1.0 - d) * 255.0) as u32 };
                self.blend(xx, yy, c, a);
            }
        }
    }

    /// A ring of `n` dots turning with `phase` (0..1): the "working" sign.
    pub fn spinner(&mut self, cx: i32, cy: i32, radius: f32, phase: f32, c: [u8; 4]) {
        let n = 10;
        for i in 0..n {
            let t = i as f32 / n as f32;
            let ang = (t + phase) * std::f32::consts::TAU;
            let (x, y) = (cx as f32 + radius * ang.cos(), cy as f32 + radius * ang.sin());
            let a = (40.0 + 215.0 * t) as u8;
            let rr = 2.5 + 2.0 * t;
            for yy in (y - rr - 1.0) as i32..=(y + rr + 1.0) as i32 {
                for xx in (x - rr - 1.0) as i32..=(x + rr + 1.0) as i32 {
                    let d = (((xx as f32 - x).powi(2) + (yy as f32 - y).powi(2)).sqrt() - rr).clamp(0.0, 1.0);
                    self.blend(xx, yy, [c[0], c[1], c[2], a], ((1.0 - d) * 255.0) as u32);
                }
            }
        }
    }

    /// The width of `s` in pixels at `scale` (1.0 = the font's size, 36 pixels).
    pub fn text_width(&self, s: &str, scale: f32) -> i32 {
        let f = font();
        let f = f.as_ref().unwrap();
        s.chars().map(|ch| f.glyphs.iter().find(|g| g.0 == ch).map_or(f.cw / 2, |g| g.1)).sum::<usize>() as i32 * (scale * 256.0) as i32 / 256
    }

    /// Text with its top-left at (x, y); returns the x after it.
    pub fn text(&mut self, x: i32, y: i32, s: &str, scale: f32, c: [u8; 4]) -> i32 {
        let guard = font();
        let f = guard.as_ref().unwrap();
        let mut pen = x as f32;
        for ch in s.chars() {
            let Some(gi) = f.glyphs.iter().position(|g| g.0 == ch) else {
                pen += f.cw as f32 * 0.5 * scale;
                continue;
            };
            let (gx, gy) = ((gi % 16) * f.cw, (gi / 16) * f.ch);
            let (ow, oh) = ((f.cw as f32 * scale).ceil() as i32, (f.ch as f32 * scale).ceil() as i32);
            for yy in 0..oh {
                for xx in 0..ow {
                    // bilinear sample of the atlas cell
                    let (sx, sy) = (xx as f32 / scale, yy as f32 / scale);
                    let (x0, y0) = (sx.floor() as usize, sy.floor() as usize);
                    let (fx, fy) = (sx - x0 as f32, sy - y0 as f32);
                    let at = |px: usize, py: usize| -> f32 {
                        if px >= f.cw || py >= f.ch {
                            0.0
                        } else {
                            f.alpha[(gy + py) * f.aw + gx + px] as f32
                        }
                    };
                    let a = at(x0, y0) * (1.0 - fx) * (1.0 - fy) + at(x0 + 1, y0) * fx * (1.0 - fy) + at(x0, y0 + 1) * (1.0 - fx) * fy + at(x0 + 1, y0 + 1) * fx * fy;
                    if a > 0.5 {
                        let ox = (f.xoff as f32 * scale) as i32;
                        let oy = (f.yoff as f32 * scale) as i32;
                        self.blend(pen as i32 + xx - ox, y + yy - oy, c, a as u32);
                    }
                }
            }
            pen += f.glyphs[gi].1 as f32 * scale;
        }
        let _ = f.base;
        pen as i32
    }

    /// Text centred on x.
    pub fn text_centered(&mut self, cx: i32, y: i32, s: &str, scale: f32, c: [u8; 4]) {
        let w = self.text_width(s, scale);
        self.text(cx - w / 2, y, s, scale, c);
    }
}

/// Shows the canvas over the game (None: nothing).
pub fn show(canvas: Option<Canvas>) {
    let mut o = OVERLAY.lock().unwrap();
    match canvas {
        Some(c) => {
            o.visible = true;
            o.rgba = c.px;
        }
        None => {
            if !o.visible {
                return;
            }
            o.visible = false;
        }
    }
    o.serial += 1;
}

/// The canvas blended into an RGBA picture of `w` x `h` (a screenshot).
pub fn blend_into(out: &mut [u8], w: usize, h: usize) {
    let o = OVERLAY.lock().unwrap();
    if !o.visible || o.rgba.len() != W * H * 4 {
        return;
    }
    for y in 0..h {
        for x in 0..w {
            let s = ((y * H / h) * W + x * W / w) * 4;
            let a = o.rgba[s + 3] as u32;
            if a == 0 {
                continue;
            }
            let d = (y * w + x) * 4;
            for k in 0..3 {
                out[d + k] = ((o.rgba[s + k] as u32 * a + out[d + k] as u32 * (255 - a)) / 255) as u8;
            }
        }
    }
}

/// What is shown now (to put back after a question drawn over it).
pub struct Snapshot {
    visible: bool,
    rgba: Vec<u8>,
}

impl Snapshot {
    /// The shown canvas copied into `cv` (nothing when nothing was shown).
    pub fn copy_into(&self, cv: &mut Canvas) {
        if self.visible && self.rgba.len() == W * H * 4 {
            cv.px.copy_from_slice(&self.rgba);
        }
    }
}

pub fn snapshot() -> Snapshot {
    let o = OVERLAY.lock().unwrap();
    Snapshot { visible: o.visible, rgba: o.rgba.clone() }
}

pub fn restore(s: Snapshot) {
    let mut o = OVERLAY.lock().unwrap();
    o.visible = s.visible;
    o.rgba = s.rgba;
    o.serial += 1;
}

/// A yes / no question over a darkened copy of `under` (host/saves.rs draws it).
pub fn show_question(under: &Snapshot, q: &str, sub: &str, yes: bool) {
    let mut cv = Canvas::new();
    if under.visible && under.rgba.len() == W * H * 4 {
        cv.px.copy_from_slice(&under.rgba);
    }
    cv.rect(0, 0, W as i32, H as i32, 0, [0, 0, 0, 150]);
    super::saves::question(&mut cv, q, sub, yes);
    show(Some(cv));
}
