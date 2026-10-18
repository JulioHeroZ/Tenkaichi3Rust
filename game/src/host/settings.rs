//! The PC settings: full screen, the internal resolution and the keyboard keys of the controller, kept in
//! Documents\My Games\Tenkaichi3Rust\settings.ini (BT3_SETTINGS=<file> for another one) and changed in the game with
//! F1, on a screen drawn over the paused picture (gs/gpu.rs settings_menu). The file is plain text and can be edited
//! by hand too: `name=value` lines, keys by name (Up, K, RightShift, Keypad8...).
use super::ui::{self, Canvas};
use std::path::PathBuf;
use std::sync::Mutex;

/// The controls of the PS2 controller a key can be given to: the 16 buttons (their bit in the button word) and the
/// left stick's four directions.
pub const ACTIONS: [(&str, &str); 20] = [
    ("select", "Select"),
    ("l3", "L3"),
    ("r3", "R3"),
    ("start", "Start"),
    ("up", "Direcional cima"),
    ("right", "Direcional direita"),
    ("down", "Direcional baixo"),
    ("left", "Direcional esquerda"),
    ("l2", "L2"),
    ("r2", "R2"),
    ("l1", "L1"),
    ("r1", "R1"),
    ("triangle", "Triângulo"),
    ("circle", "Círculo"),
    ("cross", "X"),
    ("square", "Quadrado"),
    ("stick_up", "Analógico cima"),
    ("stick_down", "Analógico baixo"),
    ("stick_left", "Analógico esquerda"),
    ("stick_right", "Analógico direita"),
];

/// The keys of the C port (and of the first versions of this one).
const DEFAULT_KEYS: [u32; 20] = [229, 0, 0, 40, 82, 79, 81, 80, 36, 38, 24, 18, 12, 15, 14, 13, 26, 22, 4, 7];

/// Keys kept for the port itself: Esc (quit question), F1 (this screen), F11 (full screen).
const RESERVED: [u32; 3] = [41, 58, 68];

#[derive(Clone)]
pub struct Settings {
    pub fullscreen: bool,
    /// internal resolution, times the PS2's (1..4); used when the game starts
    pub scale: i32,
    /// SDL scancode per action (0: none)
    pub keys: [u32; 20],
}

impl Default for Settings {
    fn default() -> Self {
        Settings { fullscreen: false, scale: 2, keys: DEFAULT_KEYS }
    }
}

static SETTINGS: Mutex<Option<Settings>> = Mutex::new(None);

fn path() -> PathBuf {
    match std::env::var("BT3_SETTINGS") {
        Ok(p) => PathBuf::from(p),
        Err(_) => super::saves::documents().join("My Games").join("Tenkaichi3Rust").join("settings.ini"),
    }
}

/// The settings (read from the file the first time).
pub fn get() -> Settings {
    let mut s = SETTINGS.lock().unwrap();
    if s.is_none() {
        *s = Some(load());
    }
    s.clone().unwrap()
}

fn load() -> Settings {
    let mut st = Settings::default();
    let Ok(text) = std::fs::read_to_string(path()) else { return st };
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with('#') {
            continue;
        }
        let Some((k, v)) = line.split_once('=') else { continue };
        let (k, v) = (k.trim(), v.trim());
        match k {
            "fullscreen" => st.fullscreen = v == "1" || v.eq_ignore_ascii_case("true"),
            "scale" => st.scale = v.parse().unwrap_or(2).clamp(1, 4),
            _ => {
                if let Some(a) = k.strip_prefix("key.").and_then(|n| ACTIONS.iter().position(|x| x.0 == n)) {
                    st.keys[a] = if v.is_empty() || v == "-" { 0 } else { scancode(v).unwrap_or(st.keys[a]) };
                }
            }
        }
    }
    st
}

/// Changes the settings and writes the file.
pub fn set(st: Settings) {
    let mut text = String::from("# Tenkaichi3Rust: configurações do PC (também pela tecla F1 no jogo).\n");
    text += &format!("# tela cheia (0 ou 1)\nfullscreen={}\n", st.fullscreen as u8);
    text += &format!("# resolução interna: vezes a do PS2, 1 a 4 (vale ao abrir o jogo)\nscale={}\n", st.scale);
    text += "# teclas do controle (nomes como Up, K, RightShift, Keypad8; - = nenhuma)\n";
    for (i, (name, _)) in ACTIONS.iter().enumerate() {
        text += &format!("key.{name}={}\n", if st.keys[i] == 0 { "-".into() } else { key_name(st.keys[i]) });
    }
    let p = path();
    if let Some(dir) = p.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let tmp = p.with_extension("tmp");
    if std::fs::write(&tmp, text).is_ok() {
        let _ = std::fs::rename(&tmp, &p);
    }
    *SETTINGS.lock().unwrap() = Some(st);
}

/// SDL scancode names.
fn key_names() -> Vec<(u32, String)> {
    let mut v: Vec<(u32, String)> = Vec::new();
    for (i, c) in ('A'..='Z').enumerate() {
        v.push((4 + i as u32, c.to_string()));
    }
    for i in 1..=9u32 {
        v.push((29 + i, i.to_string()));
    }
    v.push((39, "0".into()));
    for (sc, n) in [
        (40, "Enter"), (41, "Esc"), (42, "Backspace"), (43, "Tab"), (44, "Space"), (45, "Minus"), (46, "Equals"),
        (47, "LeftBracket"), (48, "RightBracket"), (49, "Backslash"), (51, "Semicolon"), (52, "Apostrophe"),
        (53, "Grave"), (54, "Comma"), (55, "Period"), (56, "Slash"), (57, "CapsLock"), (73, "Insert"), (74, "Home"),
        (75, "PageUp"), (76, "Delete"), (77, "End"), (78, "PageDown"), (79, "Right"), (80, "Left"), (81, "Down"),
        (82, "Up"), (84, "KeypadDivide"), (85, "KeypadMultiply"), (86, "KeypadMinus"), (87, "KeypadPlus"),
        (88, "KeypadEnter"), (99, "KeypadPeriod"), (98, "Keypad0"), (224, "LeftCtrl"), (225, "LeftShift"),
        (226, "LeftAlt"), (228, "RightCtrl"), (229, "RightShift"), (230, "RightAlt"),
    ] {
        v.push((sc, n.into()));
    }
    for i in 1..=12u32 {
        v.push((57 + i, format!("F{i}")));
    }
    for i in 1..=9u32 {
        v.push((88 + i, format!("Keypad{i}")));
    }
    v
}

pub fn key_name(sc: u32) -> String {
    key_names().into_iter().find(|k| k.0 == sc).map(|k| k.1).unwrap_or_else(|| format!("#{sc}"))
}

fn scancode(name: &str) -> Option<u32> {
    if let Some(n) = name.strip_prefix('#') {
        return n.parse().ok();
    }
    key_names().into_iter().find(|k| k.1.eq_ignore_ascii_case(name)).map(|k| k.0)
}

// ------------------------------------------------------------------------------------------------ the F1 screen
/// What the screen did with a frame's input.
pub enum Step {
    Stay,
    Close,
}

pub struct Menu {
    st: Settings,
    page: usize, // 0 main, 1 keys
    cursor: usize,
    /// waiting for the key of this action
    waiting: Option<usize>,
    last: u16,
    note: String,
}

const MAIN: [&str; 4] = ["Tela cheia", "Resolução interna", "Teclas do controle", "Voltar ao jogo"];

impl Menu {
    pub fn new() -> Menu {
        Menu { st: get(), page: 0, cursor: 0, waiting: None, last: 0xFFFF, note: String::new() }
    }

    /// The current settings (full screen is applied by the caller as it changes).
    pub fn fullscreen(&self) -> bool {
        self.st.fullscreen
    }

    /// One frame: the controller buttons now (PS2 bits), the key pressed this frame (scancode, 0 none), whether
    /// Esc was pressed.
    pub fn step(&mut self, buttons: u16, key: u32, escape: bool) -> Step {
        let pressed = buttons & !self.last;
        self.last = buttons;
        if let Some(a) = self.waiting {
            if escape {
                self.waiting = None;
                self.note.clear();
            } else if key != 0 {
                if RESERVED.contains(&key) {
                    self.note = format!("{} é usada pelo jogo no PC; escolha outra.", key_name(key));
                } else {
                    // a key goes to one action only
                    for k in self.st.keys.iter_mut() {
                        if *k == key {
                            *k = 0;
                        }
                    }
                    self.st.keys[a] = key;
                    self.waiting = None;
                    self.note.clear();
                    set(self.st.clone());
                }
            }
            return Step::Stay;
        }
        const UP: u16 = 1 << 4;
        const RIGHT: u16 = 1 << 5;
        const DOWN: u16 = 1 << 6;
        const LEFT: u16 = 1 << 7;
        const START: u16 = 1 << 3;
        const CROSS: u16 = 1 << 14;
        const CIRCLE: u16 = 1 << 13;
        let count = if self.page == 0 { MAIN.len() } else { ACTIONS.len() + 2 };
        if pressed & UP != 0 {
            self.cursor = (self.cursor + count - 1) % count;
        }
        if pressed & DOWN != 0 {
            self.cursor = (self.cursor + 1) % count;
        }
        let ok = pressed & (CROSS | START) != 0 || key == 40;
        let back = escape || pressed & CIRCLE != 0;
        let side = if pressed & LEFT != 0 { -1 } else if pressed & RIGHT != 0 { 1 } else { 0 };
        if self.page == 0 {
            match self.cursor {
                0 if ok || side != 0 => {
                    self.st.fullscreen = !self.st.fullscreen;
                    set(self.st.clone());
                }
                1 if ok || side != 0 => {
                    let d = if side < 0 { -1 } else { 1 };
                    self.st.scale = (self.st.scale - 1 + d).rem_euclid(4) + 1;
                    set(self.st.clone());
                    self.note = "A resolução nova vale na próxima vez que o jogo abrir.".into();
                }
                2 if ok => {
                    self.page = 1;
                    self.cursor = 0;
                    self.note.clear();
                }
                3 if ok => return Step::Close,
                _ => {}
            }
            if back {
                return Step::Close;
            }
        } else {
            if ok {
                if self.cursor < ACTIONS.len() {
                    self.waiting = Some(self.cursor);
                    self.note = "Pressione a tecla nova (Esc cancela).".into();
                } else if self.cursor == ACTIONS.len() {
                    self.st.keys = DEFAULT_KEYS;
                    set(self.st.clone());
                    self.note = "Teclas padrão de volta.".into();
                } else {
                    self.page = 0;
                    self.cursor = 2;
                    self.note.clear();
                }
            } else if back {
                self.page = 0;
                self.cursor = 2;
                self.note.clear();
            }
        }
        Step::Stay
    }

    /// The screen over a darkened copy of what was shown.
    pub fn draw(&self, under: &ui::Snapshot) {
        const WHITE: [u8; 4] = [255, 255, 255, 255];
        const DIM: [u8; 4] = [190, 196, 210, 255];
        const ACCENT: [u8; 4] = [255, 160, 40, 255];
        let mut cv = Canvas::new();
        under.copy_into(&mut cv);
        cv.rect(0, 0, ui::W as i32, ui::H as i32, 0, [8, 10, 18, 200]);
        cv.text_centered(640, 60, "Configurações do PC", 1.2, WHITE);
        if self.page == 0 {
            let values = [
                if self.st.fullscreen { "Sim".to_string() } else { "Não".to_string() },
                format!("{}x ({} x {})", self.st.scale, 640 * self.st.scale, 448 * self.st.scale),
                String::new(),
                String::new(),
            ];
            for (i, label) in MAIN.iter().enumerate() {
                let y = 190 + i as i32 * 110;
                let sel = i == self.cursor;
                cv.rect(240, y, 800, 86, 16, if sel { [60, 46, 26, 235] } else { [28, 32, 46, 220] });
                cv.text(280, y + 24, label, 0.85, if sel { ACCENT } else { WHITE });
                if !values[i].is_empty() {
                    let w = cv.text_width(&values[i], 0.8);
                    cv.text(1000 - w, y + 26, &values[i], 0.8, WHITE);
                }
            }
            cv.text_centered(640, 790, "↑ ↓  escolher     ← →  mudar     Enter / ✕  confirmar     Esc / ○  voltar", 0.6, DIM);
        } else {
            // the key list, a page that scrolls with the cursor
            let rows = ACTIONS.len() + 2;
            let per = 11;
            let first = self.cursor.saturating_sub(per / 2).min(rows - per);
            for r in first..first + per {
                let y = 150 + (r - first) as i32 * 54;
                let sel = r == self.cursor;
                cv.rect(240, y, 800, 46, 12, if sel { [60, 46, 26, 235] } else { [28, 32, 46, 220] });
                let (label, value) = if r < ACTIONS.len() {
                    let v = if self.waiting == Some(r) { "...".to_string() } else if self.st.keys[r] == 0 { "nenhuma".to_string() } else { key_name(self.st.keys[r]) };
                    (ACTIONS[r].1.to_string(), v)
                } else if r == ACTIONS.len() {
                    ("Restaurar as teclas padrão".to_string(), String::new())
                } else {
                    ("Voltar".to_string(), String::new())
                };
                cv.text(270, y + 8, &label, 0.7, if sel { ACCENT } else { WHITE });
                if !value.is_empty() {
                    let w = cv.text_width(&value, 0.7);
                    cv.text(1010 - w, y + 8, &value, 0.7, WHITE);
                }
            }
            cv.text_centered(640, 790, "↑ ↓  escolher     Enter / ✕  trocar a tecla     Esc / ○  voltar", 0.6, DIM);
        }
        if !self.note.is_empty() {
            cv.text_centered(640, 740, &self.note, 0.65, ACCENT);
        }
        ui::show(Some(cv));
    }
}
