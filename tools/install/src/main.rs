//! Passo 3 do instalador (tools/install/instalar.py): os arquivos do disco extraído (build/disc: pzs3us0, pzs3us1,
//! pzs3us2, disc) copiados para assets/_legacy/raw, e os streams de música e voz (CRI ADX) convertidos em WAV em
//! assets/audio/streams, com o catálogo assets/catalog.csv.
//!
//!     bt3-install export <disco extraído> <assets>
use std::{
    env, fs,
    io::{self, BufWriter, Write},
    path::{Path, PathBuf},
};

/// Previous decoded samples, newest first. Each channel owns its history.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct History(pub [i32; 2]);

/// Decode one CRI ADX type-3 frame. Coefficients use 12 fractional bits.
pub fn decode_adx(frame: &[u8; 18], coefficients: [i32; 2], history: &mut History) -> [i16; 32] {
    let scale = i32::from(u16::from_be_bytes([frame[0], frame[1]])) + 1;
    let mut output = [0; 32];
    for (i, sample) in output.iter_mut().enumerate() {
        let raw = (frame[2 + i / 2] >> if i & 1 == 0 { 4 } else { 0 }) & 15;
        let nibble = if raw & 8 != 0 {
            i32::from(raw) - 16
        } else {
            i32::from(raw)
        };
        let prediction = coefficients[0]
            .wrapping_mul(history.0[0])
            .wrapping_add(coefficients[1].wrapping_mul(history.0[1]))
            >> 12;
        let value = (nibble * scale + prediction).clamp(-32768, 32767);
        history.0 = [value, history.0[0]];
        *sample = value as i16;
    }
    output
}


fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}
fn be32(data: &[u8], offset: usize) -> u32 {
    u32::from_be_bytes(data[offset..offset + 4].try_into().unwrap())
}
fn wav(path: &Path, data: &[u8]) -> io::Result<(u32, u32, u8, u32, u32)> {
    if data.len() < 24
        || data[..2] != [0x80, 0]
        || data[4..7] != [3, 18, 4]
        || data[18] != 4
        || data[19] != 0
    {
        return Err(invalid("unsupported ADX header"));
    }
    let channels = data[7];
    let rate = be32(data, 8);
    let total = be32(data, 12);
    let start = usize::from(u16::from_be_bytes([data[2], data[3]])) + 4;
    if !(1..=2).contains(&channels) || rate == 0 || start < 24 || start > data.len() {
        return Err(invalid("invalid ADX dimensions"));
    }
    let frames = (total as usize).div_ceil(32);
    let encoded = frames
        .checked_mul(18 * channels as usize)
        .ok_or_else(|| invalid("ADX length overflow"))?;
    if encoded > data.len() - start {
        return Err(invalid("truncated ADX frames"));
    }
    let pcm_bytes = total
        .checked_mul(u32::from(channels) * 2)
        .ok_or_else(|| invalid("WAV too large"))?;
    let looping = start >= 0x38 && data.len() >= 0x38 && be32(data, 0x24) != 0;
    let loop_start = if looping { be32(data, 0x28) } else { 0 };
    let loop_end = if looping { be32(data, 0x30) } else { 0 };
    if looping && (loop_start >= loop_end || loop_end > total) {
        return Err(invalid("invalid loop range"));
    }
    let mut out = BufWriter::new(fs::File::create(path)?);
    out.write_all(b"RIFF")?;
    out.write_all(&(36 + pcm_bytes + if looping { 68 } else { 0 }).to_le_bytes())?;
    out.write_all(b"WAVEfmt ")?;
    out.write_all(&16u32.to_le_bytes())?;
    out.write_all(&1u16.to_le_bytes())?;
    out.write_all(&u16::from(channels).to_le_bytes())?;
    out.write_all(&rate.to_le_bytes())?;
    out.write_all(&(rate * u32::from(channels) * 2).to_le_bytes())?;
    out.write_all(&(u16::from(channels) * 2).to_le_bytes())?;
    out.write_all(&16u16.to_le_bytes())?;
    out.write_all(b"data")?;
    out.write_all(&pcm_bytes.to_le_bytes())?;
    let highpass = u16::from_be_bytes([data[16], data[17]]);
    let z = (2.0 * std::f64::consts::PI * f64::from(highpass) / f64::from(rate)).cos();
    let a = std::f64::consts::SQRT_2 - z;
    let b = std::f64::consts::SQRT_2 - 1.0;
    let c = (a - ((a + b) * (a - b)).sqrt()) / b;
    let coefficients = [(c * 8192.0) as i32, (c * c * -4096.0) as i32];
    let mut history = [History::default(); 2];
    for f in 0..frames {
        let mut pcm = [[0i16; 32]; 2];
        for ch in 0..channels as usize {
            let offset = start + (f * channels as usize + ch) * 18;
            pcm[ch] = decode_adx(
                data[offset..offset + 18].try_into().unwrap(),
                coefficients,
                &mut history[ch],
            );
        }
        for i in 0..32.min(total as usize - f * 32) {
            for channel in pcm.iter().take(channels as usize) {
                out.write_all(&channel[i].to_le_bytes())?;
            }
        }
    }
    if looping {
        out.write_all(b"smpl")?;
        out.write_all(&60u32.to_le_bytes())?;
        for v in [
            0,
            0,
            1_000_000_000 / rate,
            60,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
            loop_start,
            loop_end - 1,
            0,
            0,
        ] {
            out.write_all(&v.to_le_bytes())?;
        }
    }
    out.flush()?;
    Ok((rate, total, channels, loop_start, loop_end))
}
fn files(root: &Path) -> io::Result<Vec<PathBuf>> {
    fn walk(directory: &Path, result: &mut Vec<PathBuf>) -> io::Result<()> {
        for entry in fs::read_dir(directory)? {
            let path = entry?.path();
            if path.is_dir() {
                walk(&path, result)?;
            } else if path.is_file() {
                result.push(path);
            }
        }
        Ok(())
    }
    let mut result = vec![];
    for partition in ["pzs3us0", "pzs3us1", "pzs3us2", "disc"] {
        let directory = root.join(partition);
        if !directory.is_dir() {
            return Err(invalid("expected extracted gamedata partitions"));
        }
        walk(&directory, &mut result)?;
    }
    result.sort();
    Ok(result)
}
fn csv(s: &str) -> String {
    format!("\"{}\"", s.replace('"', "\"\""))
}
fn main() -> io::Result<()> {
    let args: Vec<_> = env::args_os().collect();
    if args.len() >= 4 && args[1] == "icone" {
        return icone(&args[2..]);
    }
    if args.len() >= 3 && args[1] == "icone-instalador" {
        return icone_instalador(&args[2..]);
    }
    if args.len() != 4 || args[1] != "export" {
        eprintln!("uso: bt3-install export <disco extraído> <assets>");
        eprintln!("     bt3-install icone <zs3usop.m2v> <saída .ico> [programa.exe...]");
        eprintln!("     bt3-install icone-instalador <programa.exe>");
        return Err(invalid("unknown command"));
    }
    let root = Path::new(&args[2]).canonicalize()?;
    let output = Path::new(&args[3]);
    fs::create_dir_all(output)?;
    let output = output.canonicalize()?;
    if output.starts_with(&root) || root.starts_with(&output) {
        return Err(invalid("source and export folders must be separate"));
    }
    let list = files(&root)?;
    let mut catalog = BufWriter::new(fs::File::create(output.join("catalog.csv"))?);
    writeln!(
        catalog,
        "resource_id,source,bytes,signature,wav,rate,samples,channels,loop_start,loop_end,status"
    )?;
    let mut audio = 0;
    let mut failures = 0;
    for (index, path) in list.iter().enumerate() {
        let relative = path.strip_prefix(&root).unwrap();
        let partition = relative
            .components()
            .next()
            .unwrap()
            .as_os_str()
            .to_string_lossy();
        let name = path.file_name().unwrap().to_string_lossy();
        let id = path
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .parse::<u32>()
            .ok()
            .map(|n| match partition.as_ref() {
                "pzs3us0" => n,
                "pzs3us1" => n + 1,
                "pzs3us2" => n + 3400,
                _ => n,
            });
        let data = fs::read(path)?;
        let raw = output.join("_legacy/raw").join(relative);
        fs::create_dir_all(raw.parent().unwrap())?;
        if !raw.exists() || raw.metadata()?.len() != data.len() as u64 {
            fs::write(raw, &data)?;
        }
        let signature: String = data.iter().take(4).map(|b| format!("{b:02x}")).collect();
        let mut values = String::from(",,,,,,raw");
        if data.starts_with(&[0x80, 0]) {
            let target = PathBuf::from("audio/streams")
                .join(partition.as_ref())
                .join(format!("{name}.wav"));
            let destination = output.join(&target);
            fs::create_dir_all(destination.parent().unwrap())?;
            match wav(&destination, &data) {
                Ok((rate, total, ch, ls, le)) => {
                    audio += 1;
                    values = format!(
                        "{},{rate},{total},{ch},{ls},{le},decoded",
                        csv(&target.to_string_lossy())
                    );
                }
                Err(e) => {
                    failures += 1;
                    values = format!(",,,,,,{}", csv(&e.to_string()));
                }
            }
        }
        writeln!(
            catalog,
            "{},{},{},{signature},{values}",
            id.map(|n| n.to_string()).unwrap_or_default(),
            csv(&relative.to_string_lossy()),
            data.len()
        )?;
        if index % 2000 == 0 {
            println!(
                "{index}/{} files; {audio} WAV; {failures} errors",
                list.len()
            );
            catalog.flush()?;
        }
    }
    catalog.flush()?;
    let summary = format!(
        "{{\"files\":{},\"adx_wav\":{audio},\"decode_errors\":{failures}}}\n",
        list.len()
    );
    fs::write(output.join("summary.json"), &summary)?;
    println!("{summary}");
    if failures != 0 {
        return Err(invalid("some audio files failed; see catalog.csv"));
    }
    Ok(())
}

/// The game's icon from the opening of the user's disc (bt3_install::icon): an .ico, and the icon of the programs.
fn icone(args: &[std::ffi::OsString]) -> io::Result<()> {
    let (rgba, side) = bt3_install::icon::logo(fs::read(&args[0])?).map_err(|e| invalid(&e))?;
    let images = bt3_install::icon::images(&rgba, side);
    let ico = Path::new(&args[1]);
    if let Some(dir) = ico.parent() {
        fs::create_dir_all(dir)?;
    }
    fs::write(ico, bt3_install::icon::ico(&images))?;
    for exe in &args[2..] {
        bt3_install::icon::set_exe_icon(Path::new(exe), &images).map_err(|e| invalid(&e))?;
        println!("ícone do jogo em {}", Path::new(exe).display());
    }
    Ok(())
}

/// The installer's own icon: an original drawing (the installer is handed out, so it carries nothing of the game):
/// a navy rounded square with an orange disc and a white arrow pointing down into a tray.
fn icone_instalador(args: &[std::ffi::OsString]) -> io::Result<()> {
    let n = 256usize;
    let mut px = vec![0u8; n * n * 4];
    let f = n as f32;
    for y in 0..n {
        for x in 0..n {
            let (fx, fy) = (x as f32 + 0.5, y as f32 + 0.5);
            // rounded square
            let r = f * 0.18;
            let (cx, cy) = (fx.clamp(r, f - r), fy.clamp(r, f - r));
            let inside = ((fx - cx).powi(2) + (fy - cy).powi(2)).sqrt() <= r;
            if !inside {
                continue;
            }
            let mut c = [0x1D, 0x2A, 0x4A, 255];
            // orange disc
            let d = ((fx - f * 0.5).powi(2) + (fy - f * 0.46).powi(2)).sqrt();
            if d < f * 0.34 {
                c = [0xF5, 0x8C, 0x21, 255];
            }
            // white arrow: shaft and head
            let ax = (fx - f * 0.5).abs();
            let shaft = ax < f * 0.055 && fy > f * 0.22 && fy < f * 0.52;
            let head = fy >= f * 0.46 && fy < f * 0.66 && ax < (f * 0.66 - fy) * 0.9;
            // tray
            let tray = fy > f * 0.74 && fy < f * 0.80 && ax < f * 0.30;
            if shaft || head || tray {
                c = [255, 255, 255, 255];
            }
            px[(y * n + x) * 4..(y * n + x) * 4 + 4].copy_from_slice(&c);
        }
    }
    let images = bt3_install::icon::images(&px, n);
    bt3_install::icon::set_exe_icon(Path::new(&args[0]), &images).map_err(|e| invalid(&e))?;
    println!("ícone do instalador em {}", Path::new(&args[0]).display());
    Ok(())
}
