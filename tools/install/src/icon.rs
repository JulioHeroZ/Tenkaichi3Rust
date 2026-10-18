//! The game's icon, made at installation from the user's own disc: the title logo at the end of the opening movie
//! (assets/video/zs3usop.m2v, decoded by the game's own MPEG-2 decoder), cut square and scaled to the sizes Windows
//! shows, then written into the installed programs (Tenkaichi3.exe, Desinstalar.exe) as their icon resource and to
//! assets/data/icone.ico. The project itself carries no picture of the game.

/// The picture of the opening with the whole logo, and the square around the logo (x, y, side) in its 512x448.
const LOGO_FRAME: usize = 2750;
const LOGO_SQUARE: (usize, usize, usize) = (44, 14, 424);
pub const SIZES: [usize; 5] = [256, 64, 48, 32, 16];

/// RGBA (side x side) of the logo from the opening's video.
pub fn logo(m2v: Vec<u8>) -> Result<(Vec<u8>, usize), String> {
    let mut d = crate::mpeg2::Decoder::new(m2v)?;
    for _ in 0..=LOGO_FRAME {
        if !d.next_picture()? {
            return Err("o vídeo de abertura terminou antes do logotipo".into());
        }
    }
    let (w, h) = (d.width, d.height);
    let mut rgba = vec![0u8; w * h * 4];
    d.rgba(&mut rgba);
    let (x0, y0, side) = LOGO_SQUARE;
    if x0 + side > w || y0 + side > h {
        return Err("vídeo de abertura de tamanho inesperado".into());
    }
    let mut out = vec![0u8; side * side * 4];
    for y in 0..side {
        let s = ((y0 + y) * w + x0) * 4;
        out[y * side * 4..(y + 1) * side * 4].copy_from_slice(&rgba[s..s + side * 4]);
    }
    Ok((out, side))
}

/// A square RGBA picture scaled down to `n` x `n` (the average of the pixels each one covers).
pub fn scale(src: &[u8], side: usize, n: usize) -> Vec<u8> {
    let mut out = vec![0u8; n * n * 4];
    for y in 0..n {
        let (sy0, sy1) = (y * side / n, ((y + 1) * side / n).max(y * side / n + 1));
        for x in 0..n {
            let (sx0, sx1) = (x * side / n, ((x + 1) * side / n).max(x * side / n + 1));
            let mut acc = [0u32; 4];
            for sy in sy0..sy1 {
                for sx in sx0..sx1 {
                    for c in 0..4 {
                        acc[c] += src[(sy * side + sx) * 4 + c] as u32;
                    }
                }
            }
            let cnt = ((sy1 - sy0) * (sx1 - sx0)) as u32;
            for c in 0..4 {
                out[(y * n + x) * 4 + c] = (acc[c] / cnt) as u8;
            }
        }
    }
    out
}

/// One icon image as Windows keeps it: BITMAPINFOHEADER (height doubled), BGRA rows bottom-up, then the AND mask.
fn dib(rgba: &[u8], n: usize) -> Vec<u8> {
    let mask_row = n.div_ceil(32) * 4;
    let mut b = Vec::with_capacity(40 + n * n * 4 + mask_row * n);
    for v in [40u32, n as u32, (2 * n) as u32] {
        b.extend_from_slice(&v.to_le_bytes());
    }
    b.extend_from_slice(&1u16.to_le_bytes()); // planes
    b.extend_from_slice(&32u16.to_le_bytes()); // bits per pixel
    for _ in 0..6 {
        b.extend_from_slice(&0u32.to_le_bytes()); // BI_RGB, size, resolution, colours
    }
    for y in (0..n).rev() {
        for x in 0..n {
            let p = &rgba[(y * n + x) * 4..(y * n + x) * 4 + 4];
            b.extend_from_slice(&[p[2], p[1], p[0], p[3]]);
        }
    }
    b.extend(std::iter::repeat(0u8).take(mask_row * n));
    b
}

/// The icon images (one per size) of a square RGBA picture.
pub fn images(src: &[u8], side: usize) -> Vec<(usize, Vec<u8>)> {
    SIZES.iter().map(|&n| (n, dib(&scale(src, side, n), n))).collect()
}

/// An .ico file of the images.
pub fn ico(images: &[(usize, Vec<u8>)]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&[0, 0, 1, 0]);
    out.extend_from_slice(&(images.len() as u16).to_le_bytes());
    let mut at = 6 + 16 * images.len();
    for (n, data) in images {
        let s = if *n >= 256 { 0 } else { *n as u8 };
        out.extend_from_slice(&[s, s, 0, 0]);
        out.extend_from_slice(&1u16.to_le_bytes());
        out.extend_from_slice(&32u16.to_le_bytes());
        out.extend_from_slice(&(data.len() as u32).to_le_bytes());
        out.extend_from_slice(&(at as u32).to_le_bytes());
        at += data.len();
    }
    for (_, data) in images {
        out.extend_from_slice(data);
    }
    out
}

/// The images of an .ico written by `ico` (or any .ico of 32-bit images): (side, image data).
pub fn read_ico(d: &[u8]) -> Result<Vec<(usize, Vec<u8>)>, String> {
    let u16_at = |o: usize| d.get(o..o + 2).map(|b| u16::from_le_bytes([b[0], b[1]]) as usize);
    let u32_at = |o: usize| d.get(o..o + 4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]) as usize);
    if u16_at(2) != Some(1) {
        return Err("não é um arquivo .ico".into());
    }
    let n = u16_at(4).ok_or("ícone vazio")?;
    let mut out = Vec::new();
    for i in 0..n {
        let e = 6 + 16 * i;
        let side = match d.get(e) { Some(0) => 256, Some(&s) => s as usize, None => return Err("ícone cortado".into()) };
        let (len, at) = (u32_at(e + 8).ok_or("ícone cortado")?, u32_at(e + 12).ok_or("ícone cortado")?);
        out.push((side, d.get(at..at + len).ok_or("ícone cortado")?.to_vec()));
    }
    Ok(out)
}

/// The icon group resource (GRPICONDIR) of images stored as RT_ICON 1..n.
pub fn group(images: &[(usize, Vec<u8>)]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&[0, 0, 1, 0]);
    out.extend_from_slice(&(images.len() as u16).to_le_bytes());
    for (i, (n, data)) in images.iter().enumerate() {
        let s = if *n >= 256 { 0 } else { *n as u8 };
        out.extend_from_slice(&[s, s, 0, 0]);
        out.extend_from_slice(&1u16.to_le_bytes());
        out.extend_from_slice(&32u16.to_le_bytes());
        out.extend_from_slice(&(data.len() as u32).to_le_bytes());
        out.extend_from_slice(&((i + 1) as u16).to_le_bytes());
    }
    out
}

/// Writes the icon into a program (Windows: its RT_ICON and RT_GROUP_ICON resources, group 1).
#[cfg(windows)]
pub fn set_exe_icon(exe: &std::path::Path, images: &[(usize, Vec<u8>)]) -> Result<(), String> {
    #[link(name = "kernel32")]
    extern "system" {
        fn BeginUpdateResourceW(file: *const u16, delete_existing: i32) -> isize;
        fn UpdateResourceW(h: isize, kind: usize, name: usize, lang: u16, data: *const u8, size: u32) -> i32;
        fn EndUpdateResourceW(h: isize, discard: i32) -> i32;
    }
    let path: Vec<u16> = exe.display().to_string().encode_utf16().chain(Some(0)).collect();
    unsafe {
        let h = BeginUpdateResourceW(path.as_ptr(), 0);
        if h == 0 {
            return Err(format!("{}: não consegui abrir os recursos", exe.display()));
        }
        let mut ok = true;
        for (i, (_, data)) in images.iter().enumerate() {
            ok &= UpdateResourceW(h, 3, i + 1, 0, data.as_ptr(), data.len() as u32) != 0; // RT_ICON
        }
        let g = group(images);
        ok &= UpdateResourceW(h, 14, 1, 0, g.as_ptr(), g.len() as u32) != 0; // RT_GROUP_ICON
        if EndUpdateResourceW(h, (!ok) as i32) == 0 || !ok {
            return Err(format!("{}: não consegui gravar o ícone", exe.display()));
        }
    }
    Ok(())
}

#[cfg(not(windows))]
pub fn set_exe_icon(_exe: &std::path::Path, _images: &[(usize, Vec<u8>)]) -> Result<(), String> {
    Ok(())
}
