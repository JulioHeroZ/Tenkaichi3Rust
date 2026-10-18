//! The game's files rebuilt from the open-format extraction (assets/files): a file of the archives is served from
//! `assets/files/<partition>/<number>/`: `skeleton.bin` (the file with the image data of its textures zeroed),
//! `textures.txt` (one line per texture: where its data goes and how the GS reads it) and one PNG per texture.
//! The extractor (tools/assets/files_extract.py) takes a texture out only when this rebuild gives back the exact
//! bytes of the original file, so the game reads what it read from the disc, and the PNGs can be edited.
//! A fighter model (pmdl) or a stage is taken out the same way to `model.gltf` (its vertices), a file packed with the game's BPE
//! is kept unpacked (`bpe.txt`), and an animation bank to `bank.txt`, one unpacked `entry_<n>.bin` per packed entry
//! and `animations.gltf` (the keys of its motions).
//! PNG decoding (zlib inflate included), JSON, the BPE packer and the GS memory layouts are here, without crates.
use super::vfs::VPath;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

static CACHE: Mutex<Option<HashMap<String, Option<Arc<Vec<u8>>>>>> = Mutex::new(None);

/// The bytes of a file of the game rebuilt from the extraction, if it has one (cached).
pub fn rebuilt(root: &std::path::Path, rel: &str) -> Option<Arc<Vec<u8>>> {
    if std::env::var("BT3_ORIGINAL_FILES").is_ok() {
        return None;
    }
    let mut cache = CACHE.lock().unwrap();
    let map = cache.get_or_insert_with(HashMap::new);
    if let Some(v) = map.get(rel) {
        return v.clone();
    }
    let dir = file_dir(root, rel);
    let v = if is_extracted(&dir) {
        match assemble(&dir) {
            Ok(b) => {
                if std::env::var("BT3_FILES_VERBOSE").is_ok() {
                    eprintln!("bt3: {rel} rebuilt from {}", dir.display());
                }
                Some(Arc::new(b))
            }
            Err(e) => {
                eprintln!("bt3: cannot rebuild {rel} from {}: {e}", dir.display());
                None
            }
        }
    } else {
        None
    };
    map.insert(rel.to_string(), v.clone());
    v
}

/// The number of a file folder of pzs3us1: a name that starts with its five digits, alone or followed by a space and a
/// description ("01423 roupa 1"; tools/assets/organize.py).
fn number_of(name: &str) -> Option<u32> {
    let b = name.as_bytes();
    if b.len() >= 5 && b[..5].iter().all(u8::is_ascii_digit) && (b.len() == 5 || b[5] == b' ') {
        name[..5].parse().ok()
    } else {
        None
    }
}

static DIRS: Mutex<Option<(std::path::PathBuf, HashMap<u32, std::path::PathBuf>)>> = Mutex::new(None);

/// Every file folder of pzs3us1 under assets/files, wherever it is: files/pzs3us1/<NNNNN>/ as the extraction writes
/// them, or grouped by kind and named (files/lutadores/Goku (Early)/01423 roupa 1/, tools/assets/organize.py). The
/// folders of the other partitions (files/pzs3us0, files/pzs3us2, files/disc) keep their own places.
pub fn file_dirs(root: &std::path::Path) -> HashMap<u32, std::path::PathBuf> {
    let mut cache = DIRS.lock().unwrap();
    if let Some((r, m)) = cache.as_ref() {
        if r == root {
            return m.clone();
        }
    }
    fn walk(dir: &std::path::Path, depth: usize, top: bool, out: &mut HashMap<u32, std::path::PathBuf>) {
        let Ok(items) = super::vfs::read_dir(dir) else { return };
        for (path, name, is_dir) in items {
            if !is_dir {
                continue;
            }
            if top && ["pzs3us0", "pzs3us2", "disc"].contains(&name.as_str()) {
                continue;
            }
            match number_of(&name) {
                Some(n) => {
                    out.entry(n).or_insert(path);
                }
                None if depth < 4 && !name.starts_with("entry_") && !name.starts_with("at_") => walk(&path, depth + 1, false, out),
                None => {}
            }
        }
    }
    let mut m = HashMap::new();
    walk(&root.join("files"), 0, true, &mut m);
    *cache = Some((root.to_path_buf(), m.clone()));
    m
}

/// The folder of a file of the game (`pzs3us1/01423.bin`, `disc/DATA/X.ADX`) in the extraction.
pub fn file_dir(root: &std::path::Path, rel: &str) -> std::path::PathBuf {
    let stem = rel.trim_end_matches(".bin");
    if let Some(n) = stem.strip_prefix("pzs3us1/").and_then(|s| s.parse::<u32>().ok()) {
        if let Some(p) = file_dirs(root).get(&n) {
            return p.clone();
        }
    }
    root.join("files").join(stem)
}

/// A glTF uri as a path: percent escapes decoded (a folder name with spaces is written %20).
fn uri_path(uri: &str) -> String {
    let b = uri.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%' && i + 2 < b.len() {
            if let Some(v) = std::str::from_utf8(&b[i + 1..i + 3]).ok().and_then(|h| u8::from_str_radix(h, 16).ok()) {
                out.push(v);
                i += 3;
                continue;
            }
        }
        out.push(b[i]);
        i += 1;
    }
    String::from_utf8(out).unwrap_or_else(|_| uri.to_string())
}

/// BT3_VERIFY_FILES=1: every file of the extraction rebuilt and compared with the original (raw data root);
/// the number that differ.
pub fn verify(assets: &std::path::Path, raw: &std::path::Path) -> usize {
    let (mut n, mut bad) = (0, 0);
    // every extracted folder under files/, at any depth (loose files such as disc/DATA/X.ADX are folders named so);
    // a folder with a numeric name is file <name>.bin of its partition
    fn walk(dir: &std::path::Path, rel: &str, raw: &std::path::Path, n: &mut usize, bad: &mut usize) {
        let Ok(items) = super::vfs::read_dir(dir) else { return };
        for (fpath, name, is_dir) in items {
            let sub = if rel.is_empty() { name.clone() } else { format!("{rel}/{name}") };
            if !is_dir {
                continue;
            }
            if !is_extracted(&fpath) {
                walk(&fpath, &sub, raw, n, bad);
                continue;
            }
            // pzs3us0/<n>, pzs3us2/<n>: <n>.bin of that partition; a numbered folder anywhere else: pzs3us1/<n>.bin;
            // otherwise a loose file of the disc
            let top = sub.split('/').next().unwrap_or("");
            let orig_rel = if name.chars().all(|c| c.is_ascii_digit()) && (top == "pzs3us0" || top == "pzs3us2") {
                format!("{sub}.bin")
            } else if let Some(k) = number_of(&name) {
                format!("pzs3us1/{k:05}.bin")
            } else {
                sub.clone()
            };
            *n += 1;
            match (assemble(&fpath), super::vfs::read(raw.join(&orig_rel))) {
                (Ok(a), Ok(o)) if a == o => {}
                (Ok(a), Ok(_)) => {
                    *bad += 1;
                    eprintln!("bt3: {} rebuilds differently from the original", fpath.display());
                    // BT3_VERIFY_DUMP=<folder>: the rebuilt bytes of the files that differ, to look at
                    if let Ok(d) = std::env::var("BT3_VERIFY_DUMP") {
                        let _ = std::fs::write(std::path::Path::new(&d).join(format!("{name}.bin")), &a);
                    }
                }
                (Err(e), _) => {
                    *bad += 1;
                    eprintln!("bt3: {}: {e}", fpath.display());
                }
                (_, Err(_)) => eprintln!("bt3: no original for {}", fpath.display()),
            }
        }
    }
    walk(&assets.join("files"), "", raw, &mut n, &mut bad);
    eprintln!("bt3: {n} files rebuilt from the extraction, {bad} differ from the originals");
    bad
}

/// Offline check of the new typed exports against source evidence. Not a game load path.
pub fn verify_pc_tables(assets:&std::path::Path,raw:&std::path::Path)->usize {
    let mut checked=0; let mut bad=0;
    let mut entries:Vec<_>=file_dirs(assets).into_iter().collect();
    entries.sort();
    for (number,dir) in entries {
        if !["empty-directory.json","roster-items.json","point-groups.json"].iter().any(|f|dir.join(f).vis_file()) {continue;}
        checked+=1;
        let original=raw.join("pzs3us1").join(format!("{number:05}.bin"));
        match (assemble(&dir),super::vfs::read(&original)) {
            (Ok(a),Ok(b)) if a==b=>{},
            (a,b)=>{bad+=1;eprintln!("bt3: PC table mismatch {}: rebuilt={}, source={}",dir.display(),a.as_ref().map_or(0,|v|v.len()),b.as_ref().map_or(0,|v|v.len()));}
        }
    }
    if checked!=601 {bad+=1;eprintln!("bt3: expected 601 PC table exports, found {checked}");}
    eprintln!("bt3: {checked} PC table exports rebuilt by Rust, {bad} failures");
    bad
}

fn is_extracted(dir: &std::path::Path) -> bool {
    dir.join("skeleton.bin").vis_file() || dir.join("skeleton.txt").vis_file() || dir.join("bank.txt").vis_file()
        || dir.join("lip-sync.json").vis_file()
        || dir.join("empty-directory.json").vis_file()
        || dir.join("roster-items.json").vis_file()
        || dir.join("point-groups.json").vis_file()
        || dir.join("audio-bank.json").vis_file()
        || dir.join("script.txt").vis_file()
        || dir.join("text-table.json").vis_file()
        || dir.join("motion-sources.json").vis_file()
}

/// skeleton + textures + model -> the file (packed again with the game's BPE when the directory has a bpe.txt);
/// or the bank of bank.txt.
pub fn assemble(dir: &std::path::Path) -> Result<Vec<u8>, String> {
    if dir.join("lip-sync.json").vis_file() {
        let text=super::vfs::read_to_string(dir.join("lip-sync.json")).map_err(|e|e.to_string())?;
        return lip_sync_table(&text);
    }
    if let Ok(text)=super::vfs::read_to_string(dir.join("text-table.json")) {
        return native_text_table(&text);
    }
    if let Ok(text)=super::vfs::read_to_string(dir.join("script.txt")) {
        // GSC source is compiled by Rust. A zero-filled binary template is
        // neither opened nor required; native scripts may grow past old size.
        return gsc_assemble(&text,std::env::var_os("BT3_VERIFY_FILES").is_none());
    }
    // Offline reference verification and original-audio diagnostics explicitly
    // use the retained compatibility serializer. Normal playback never does.
    if dir.join("audio-bank.json").vis_file()
        && std::env::var_os("BT3_ORIGINAL_AUDIO").is_none()
        && std::env::var_os("BT3_VERIFY_FILES").is_none() {
        let bank=native_sound_bank(dir)?;
        let mut token=b"BT3SFX1\0".to_vec();
        token.extend_from_slice(&bank.resource.to_le_bytes());
        return Ok(token);
    }
    if let Ok(text)=super::vfs::read_to_string(dir.join("point-groups.json")) {
        return point_groups(&text);
    }
    if let Ok(text)=super::vfs::read_to_string(dir.join("roster-items.json")) {
        return roster_items(&text);
    }
    if let Ok(text)=super::vfs::read_to_string(dir.join("empty-directory.json")) {
        let doc=Json::parse(&text)?;
        let count=doc.get("entries").num().ok_or("empty directory: entries missing")?;
        if !count.is_finite() || count<0.0 || count>65536.0 || count.fract()!=0.0 {
            return Err("empty directory: entries must be an integer in 0..65536".into());
        }
        let count=count as usize;
        let size=((count+2)*4+63)/64*64;
        let mut data=vec![0;size];
        data[..4].copy_from_slice(&(count as u32).to_le_bytes());
        for i in 0..=count {data[4+i*4..8+i*4].copy_from_slice(&(size as u32).to_le_bytes());}
        return Ok(data);
    }
    if super::resource_transport::enabled() && dir.join("motion-sources.json").vis_file() {
        let sources=MotionSources::open(dir)?;
        if sources.as_ref().is_some_and(|s|s.complete()) {
            return bank_assemble_sources(dir,"",sources);
        }
    }
    if let Ok(list) = super::vfs::read_to_string(dir.join("bank.txt")) {
        return bank_assemble(dir, &list);
    }
    let t0 = std::time::Instant::now();
    let buf = assemble_plain(dir)?;
    let t1 = t0.elapsed();
    if super::resource_transport::enabled() && dir.join("bpe.txt").vis_file() {
        // Do not read original compression parameters/padding, or recompress.
        return super::resource_transport::retain(buf);
    }
    let out = match super::vfs::read_to_string(dir.join("bpe.txt")) {
        Ok(s) => bpe_pack(&buf, &s),
        Err(_) => Ok(buf),
    };
    if std::env::var("BT3_FILES_TIME").is_ok() {
        eprintln!("bt3: {}: rebuilt in {:?}, packed in {:?}", dir.display(), t1, t0.elapsed() - t1);
    }
    out
}

fn assemble_plain(dir: &std::path::Path) -> Result<Vec<u8>, String> {
    if std::env::var_os("BT3_PC_ONLY").is_some() && dir.join("samples.txt").vis_file() {
        return Err("native audio-bank.json is required; Sony bank reconstruction is disabled in PC-only mode".into());
    }
    // skeleton.bin: what no open file gives (zeros elsewhere); none when that is nothing, skeleton.txt then gives
    // the size (`zeros <bytes>`)
    let mut buf = match super::vfs::read(dir.join("skeleton.bin")) {
        Ok(b) => b,
        Err(e) => {
            let t = super::vfs::read_to_string(dir.join("skeleton.txt")).map_err(|_| format!("skeleton.bin: {e}"))?;
            let n = t.lines().find_map(|l| l.strip_prefix("zeros ")).and_then(|x| x.trim().parse::<usize>().ok())
                .ok_or("skeleton.txt: no `zeros <size>` line")?;
            vec![0u8; n]
        }
    };
    // the typed records first: the other files may read the structure they give
    for name in LAYOUT_FILES {
        if let Ok(text) = super::vfs::read_to_string(dir.join(name)) {
            layout_apply(&mut buf, &text).map_err(|e| format!("{name}: {e}"))?;
        }
    }
    if let Ok(text) = super::vfs::read_to_string(dir.join("texts.json")) {
        texts_apply(&mut buf, &text).map_err(|e| format!("texts.json: {e}"))?;
    }
    if let Ok(text) = super::vfs::read_to_string(dir.join("lips.json")) {
        lips_apply(&mut buf, &text).map_err(|e| format!("lips.json: {e}"))?;
    }
    if let Ok(text) = super::vfs::read_to_string(dir.join("cameras.json")) {
        cameras_apply(&mut buf, &text).map_err(|e| format!("cameras.json: {e}"))?;
    }
    if let Ok(list) = super::vfs::read_to_string(dir.join("samples.txt")) {
        sounds_apply(&mut buf, dir, &list).map_err(|e| format!("samples.txt: {e}"))?;
    }
    if let Ok(text) = super::vfs::read_to_string(dir.join("script.txt")) {
        let script = gsc_assemble(&text,false).map_err(|e| format!("script.txt: {e}"))?;
        buf.get_mut(..script.len()).ok_or("script.txt: the script is longer than the file")?.copy_from_slice(&script);
    }
    if let Ok(text) = super::vfs::read_to_string(dir.join("adx.txt")) {
        adx_apply(&mut buf, dir, &text).map_err(|e| format!("adx.txt: {e}"))?;
    }
    let layout = super::vfs::read_to_string(dir.join("model.json")).ok();
    if layout.is_none() {
        if let Ok(list) = super::vfs::read_to_string(dir.join("models.txt")) {
            model_apply(&mut buf, dir, &list).map_err(|e| format!("model.gltf: {e}"))?;
        }
    }
    if dir.join("collision.gltf").vis_file() {
        collision_apply(&mut buf, dir).map_err(|e| format!("collision.gltf: {e}"))?;
    }
    if let Ok(text) = super::vfs::read_to_string(dir.join("stage-chains.json")) {
        stage_chains_apply(&mut buf, dir, &text, "model.gltf").map_err(|e| format!("stage-chains.json: {e}"))?;
    }
    // the effects' meshes (effects.gltf + effect-chains.json, tools/assets/effchain.py): chains of the same form
    if let Ok(text) = super::vfs::read_to_string(dir.join("effect-chains.json")) {
        stage_chains_apply(&mut buf, dir, &text, "effects.gltf").map_err(|e| format!("effect-chains.json: {e}"))?;
    }
    let list = super::vfs::read_to_string(dir.join("textures.txt")).unwrap_or_default();
    for line in list.lines() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let rec = Record::parse(line).ok_or_else(|| format!("bad line: {line}"))?;
        let png = super::vfs::read(dir.join(&rec.png)).map_err(|e| format!("{}: {e}", rec.png))?;
        let img = png_read(&png).map_err(|e| format!("{}: {e}", rec.png))?;
        apply(&mut buf, &rec, &img).map_err(|e| format!("{}: {e}", rec.png))?;
    }
    if let Ok(text) = super::vfs::read_to_string(dir.join("movies.json")) {
        movies_apply(&mut buf, &text).map_err(|e| format!("movies.json: {e}"))?;
    }
    if let Ok(text) = super::vfs::read_to_string(dir.join("raw-textures.txt")) {
        raw_textures_apply(&mut buf, dir, &text).map_err(|e| format!("raw-textures.txt: {e}"))?;
    }
    if let Ok(text) = super::vfs::read_to_string(dir.join("surfaces.txt")) {
        surfaces_apply(&mut buf, dir, &text).map_err(|e| format!("surfaces.txt: {e}"))?;
    }
    // the GS upload packets around the textures' data (tools/assets/texpackets.py): `offset length width height`
    if let Ok(text) = super::vfs::read_to_string(dir.join("texture-packets.txt")) {
        texture_packets_apply(&mut buf, &text).map_err(|e| format!("texture-packets.txt: {e}"))?;
    }
    if dir.join("model-motions.json").vis_file() {
        model_motions_apply(&mut buf, dir).map_err(|e| format!("model-motions: {e}"))?;
    }
    // model files inside this one (the technique models of the effect files), each a folder extracted like a file
    // of its own: `folder offset length` (tools/assets/files_extract.py)
    if let Ok(text) = super::vfs::read_to_string(dir.join("nested.txt")) {
        for line in text.lines() {
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let f: Vec<&str> = line.split_whitespace().collect();
            let [folder, o, n] = f[..] else { return Err(format!("nested.txt: bad line: {line}")) };
            let o: usize = o.parse().map_err(|_| format!("nested.txt: bad line: {line}"))?;
            let n: usize = n.parse().map_err(|_| format!("nested.txt: bad line: {line}"))?;
            let sub = assemble(&dir.join(folder)).map_err(|e| format!("{folder}: {e}"))?;
            if sub.len() != n {
                return Err(format!("{folder}: {} bytes, the file has room for {n}", sub.len()));
            }
            buf.get_mut(o..o + n).ok_or_else(|| format!("{folder} past the file"))?.copy_from_slice(&sub);
        }
    }
    // a fighter model built whole from model.gltf + model.json (tools/assets/pmdlconv.py); last, as a model of
    // another size moves the entries after it
    if let Some(text) = layout {
        model_build(&mut buf, dir, &text).map_err(|e| format!("model.json: {e}"))?;
    }
    Ok(buf)
}

// Standalone lip tables have no binary template. Empty slots retain their index;
// edited clips may grow and all following offsets are recalculated.
fn lip_sync_table(text: &str) -> Result<Vec<u8>, String> {
    let doc=Json::parse(text)?;
    let integer=|v:&Json,max:usize|->Result<usize,String> {
        let n=v.num().ok_or("lip-sync: expected integer")?;
        if !n.is_finite() || n<0.0 || n.fract()!=0.0 || n>max as f64 {
            return Err("lip-sync: integer out of range".into());
        }
        Ok(n as usize)
    };
    if integer(doc.get("version"),1)?!=1 {return Err("lip-sync: unsupported version".into());}
    let Json::Arr(slots)=doc.get("slots") else {return Err("lip-sync: slots must be an array".into());};
    if slots.is_empty() || slots.len()>4096 {return Err("lip-sync: invalid slot count".into());}
    const LIMIT:usize=64*1024*1024;
    let mut out=vec![0;((slots.len()+2)*4+63)/64*64];
    let mut offsets=Vec::with_capacity(slots.len()+1);
    for slot in slots {
        offsets.push(out.len() as u32);
        if matches!(slot,Json::Null) {continue;}
        let Json::Arr(keys)=slot.get("keys") else {return Err("lip-sync: keys must be an array".into());};
        if keys.len()>65536 {return Err("lip-sync: too many keys".into());}
        let minimum=integer(slot.get("minimumBytes"),LIMIT)?;
        let size=(minimum.max(16+keys.len()*4)+63)/64*64;
        if size>LIMIT-out.len() {return Err("lip-sync: table too large".into());}
        let at=out.len();out.resize(at+size,0);
        out[at..at+4].copy_from_slice(b"LIPS");
        out[at+4..at+8].copy_from_slice(&2u32.to_le_bytes());
        out[at+8..at+12].copy_from_slice(&(keys.len() as u32).to_le_bytes());
        let mut previous=0;
        for (i,key) in keys.iter().enumerate() {
            let Json::Arr(pair)=key else {return Err("lip-sync: key must be [frame, mouth]".into());};
            if pair.len()!=2 {return Err("lip-sync: key must contain two values".into());}
            let frame=integer(&pair[0],65535)?;
            let mouth=integer(&pair[1],65535)?;
            if i>0 && frame<previous {return Err("lip-sync: frames must be ordered".into());}
            previous=frame;
            let pos=at+16+i*4;
            out[pos..pos+2].copy_from_slice(&(frame as u16).to_le_bytes());
            out[pos+2..pos+4].copy_from_slice(&(mouth as u16).to_le_bytes());
        }
    }
    offsets.push(out.len() as u32);
    out[..4].copy_from_slice(&(slots.len() as u32).to_le_bytes());
    for (i,offset) in offsets.iter().enumerate() {out[4+i*4..8+i*4].copy_from_slice(&offset.to_le_bytes());}
    Ok(out)
}

// ---------------------------------------------------------------------------------------------- texts
/// The texts of texts.json (tools/assets/textconv.py) back into the file, in UTF-16: `at` is where the text goes (after
/// the byte order mark), `room` the bytes it can take with its final zero.
fn texts_apply(buf: &mut [u8], text: &str) -> Result<(), String> {
    let g = Json::parse(text)?;
    let mut i = 0;
    while let e @ Json::Obj(_) = g.get("texts").idx(i) {
        i += 1;
        let at = e.get("at").num().ok_or("text without at")? as usize;
        let room = e.get("room").num().ok_or("text without room")? as usize;
        let t = e.get("text").str().ok_or("text without text")?;
        let words: Vec<u16> = t.encode_utf16().collect();
        if words.len() * 2 + 2 > room {
            return Err(format!("the text at {at} is too long ({} bytes, room {room})", words.len() * 2 + 2));
        }
        if at + room > buf.len() {
            return Err(format!("the text at {at} is past the file"));
        }
        for (k, w) in words.iter().enumerate() {
            buf[at + k * 2..at + k * 2 + 2].copy_from_slice(&w.to_le_bytes());
        }
        // the byte order mark before the text, when texts.json says so (tools/assets/textconv.py)
        if e.get("bom").num() == Some(1.0) {
            if at < 2 {
                return Err(format!("the text at {at} has no room for its byte order mark"));
            }
            buf[at - 2..at].copy_from_slice(&[0xFF, 0xFE]);
        }
    }
    Ok(())
}

/// The lip sync keys of lips.json (tools/assets/lipsconv.py): at `at` the entry "LIPS", 2, count, 0, then the keys
/// (u16 frame, u16 mouth open); `room` keys fit.
fn lips_apply(buf: &mut [u8], text: &str) -> Result<(), String> {
    let g = Json::parse(text)?;
    let mut i = 0;
    while let e @ Json::Obj(_) = g.get("lips").idx(i) {
        i += 1;
        let at = e.get("at").num().ok_or("lips without at")? as usize;
        let room = e.get("room").num().ok_or("lips without room")? as usize;
        let keys = e.get("keys");
        let mut n = 0;
        while let k @ Json::Arr(_) = keys.idx(n) {
            if n >= room {
                return Err(format!("the lips at {at} have more than {room} keys"));
            }
            let fr = k.idx(0).num().ok_or("key without frame")? as u16;
            let op = k.idx(1).num().ok_or("key without state")? as u16;
            let o = at + 16 + n * 4;
            buf.get_mut(o..o + 4).ok_or("lips past the file")?.copy_from_slice(&[fr.to_le_bytes(), op.to_le_bytes()].concat());
            n += 1;
        }
        // the entry's header: "LIPS", version 2, the key count, 0
        let head = [*b"LIPS", 2u32.to_le_bytes(), (n as u32).to_le_bytes(), [0; 4]].concat();
        buf.get_mut(at..at + 16).ok_or("lips past the file")?.copy_from_slice(&head);
    }
    Ok(())
}

/// The camera animations of cameras.json (tools/assets/camconv.py): the default pose (6 floats, s32, float) at
/// `pose_at`, and per channel its keys (u32 flags, f32 time, f32 value, 32 bytes apart) at `keys_at`. The numbers of
/// channels and keys are the ones the file has.
fn cameras_apply(buf: &mut [u8], text: &str) -> Result<(), String> {
    let g = Json::parse(text)?;
    let rd = |buf: &[u8], o: usize| -> Result<u32, String> { buf.get(o..o + 4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).ok_or_else(|| "camera past the file".to_string()) };
    let put = |buf: &mut [u8], o: usize, w: u32| -> Result<(), String> {
        buf.get_mut(o..o + 4).ok_or("camera past the file")?.copy_from_slice(&w.to_le_bytes());
        Ok(())
    };
    let mut i = 0;
    while let cam @ Json::Obj(_) = g.get("cameras").idx(i) {
        i += 1;
        let n = |v: &Json, what: &str| -> Result<f64, String> { v.num().ok_or_else(|| format!("camera: no {what}")) };
        let at = n(cam.get("at"), "at")? as usize;
        let pose_at = n(cam.get("pose_at"), "pose_at")? as usize;
        let ch_at = n(cam.get("channels_at"), "channels_at")? as usize;
        let pose = cam.get("pose");
        for k in 0..8 {
            let v = n(pose.idx(k), "pose value")?;
            let w = if k == 6 { v as i32 as u32 } else { (v as f32).to_bits() };
            put(buf, pose_at + k * 4, w)?;
        }
        let chans = cam.get("channels");
        let count = rd(buf, at + 12)? as usize;
        let mut c = 0;
        while let ch @ Json::Obj(_) = chans.idx(c) {
            if c >= count {
                return Err(format!("the camera at {at} has {count} channels"));
            }
            let keys_at = n(ch.get("keys_at"), "keys_at")? as usize;
            let kc = rd(buf, ch_at + c * 16 + 8)? as usize;
            let keys = ch.get("keys");
            let mut k = 0;
            while let key @ Json::Arr(_) = keys.idx(k) {
                if k >= kc {
                    return Err(format!("a channel of the camera at {at} has {kc} keys"));
                }
                put(buf, keys_at + k * 32, n(key.idx(0), "flags")? as u32)?;
                put(buf, keys_at + k * 32 + 4, (n(key.idx(1), "time")? as f32).to_bits())?;
                put(buf, keys_at + k * 32 + 8, (n(key.idx(2), "value")? as f32).to_bits())?;
                k += 1;
            }
            if k != kc {
                return Err(format!("a channel of the camera at {at} has {kc} keys"));
            }
            c += 1;
        }
        if c != count {
            return Err(format!("the camera at {at} has {count} channels"));
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------- sounds
/// SPU ADPCM filters.
const ADPCM_FILTERS: [(i32, i32); 5] = [(0, 0), (60, 0), (115, -52), (98, -55), (122, -60)];

fn adpcm_sample(n: i32, shift: u32, pred: i32) -> i32 {
    let sh = if shift <= 12 { shift } else { 9 };
    (((n << 12) >> sh) + pred).clamp(-32768, 32767)
}

/// The sounds of samples.txt (tools/assets/vagconv.py) back into the file as SPU ADPCM: per line, the WAV, the
/// header entry, the file offset of the first block, the block count, the two header bytes of every block (shift |
/// filter << 4, flags) and the blocks whose steps are kept as they are (`block:hex`, or `-`). The steps of a block are
/// the ones that decode to the WAV's samples (one for each, from the block's shift and filter); for an edited sound
/// that no step gives exactly, the nearest.
fn sounds_apply(buf: &mut [u8], dir: &std::path::Path, list: &str) -> Result<(), String> {
    for line in list.lines() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let f: Vec<&str> = line.split_whitespace().collect();
        if f.len() != 6 {
            return Err(format!("bad line: {line}"));
        }
        let p: usize = f[2].parse().map_err(|_| format!("bad line: {line}"))?;
        let nb: usize = f[3].parse().map_err(|_| format!("bad line: {line}"))?;
        let heads = unhex(f[4]).ok_or_else(|| format!("bad line: {line}"))?;
        let mut raw: Vec<(usize, Vec<u8>)> = Vec::new();
        if f[5] != "-" {
            for r in f[5].split(',') {
                let (j, h) = r.split_once(':').ok_or_else(|| format!("bad line: {line}"))?;
                raw.push((j.parse().map_err(|_| format!("bad line: {line}"))?, unhex(h).ok_or_else(|| format!("bad line: {line}"))?));
            }
        }
        let pcm = wav_pcm(&super::vfs::read(dir.join(f[0])).map_err(|e| format!("{}: {e}", f[0]))?).map_err(|e| format!("{}: {e}", f[0]))?;
        if pcm.len() != nb * 28 || heads.len() != nb * 2 {
            return Err(format!("{}: {} samples, the file has room for {}", f[0], pcm.len(), nb * 28));
        }
        if p + nb * 16 > buf.len() {
            return Err(format!("{} past the file", f[0]));
        }
        let (mut s1, mut s2) = (0i32, 0i32);
        for j in 0..nb {
            let head = heads[j * 2];
            let (shift, filt) = ((head & 15) as u32, (head >> 4) as usize);
            let (f0, f1) = if filt < 5 { ADPCM_FILTERS[filt] } else { (0, 0) };
            let o = p + 16 * j;
            buf[o] = head;
            buf[o + 1] = heads[j * 2 + 1];
            let kept = raw.iter().find(|r| r.0 == j).map(|r| r.1.clone());
            for i in 0..28 {
                let pred = (s1 * f0 + s2 * f1 + 32) >> 6;
                let n = match &kept {
                    Some(k) => {
                        let v = ((k[i / 2] >> (4 * (i & 1))) & 15) as i32;
                        if v >= 8 { v - 16 } else { v }
                    }
                    None => {
                        let t = pcm[j * 28 + i] as i32;
                        (-8..8).min_by_key(|&n| (adpcm_sample(n, shift, pred) - t).abs()).unwrap()
                    }
                };
                let s = adpcm_sample(n, shift, pred);
                let byte = &mut buf[o + 2 + i / 2];
                if i & 1 == 0 {
                    *byte = (n & 15) as u8;
                } else {
                    *byte |= ((n & 15) as u8) << 4;
                }
                s2 = s1;
                s1 = s;
            }
        }
    }
    Ok(())
}

/// A CRI ADX stream from stream.wav (tools/assets/adxconv.py): frames of 18 bytes per channel after the header (data
/// offset at 2, channels at 7), a big-endian scale (kept in the file) then 32 steps; sample = step * scale +
/// (c1 * previous + c2 * the one before) >> 12, clamped. adx.txt: `coefs c1 c2`, `frames n`, `keep <frame>:<hex>`.
fn adx_apply(buf: &mut [u8], dir: &std::path::Path, text: &str) -> Result<(), String> {
    let (mut c1, mut c2, mut nf) = (0i32, 0i32, 0usize);
    let mut kept: Vec<(usize, Vec<u8>)> = Vec::new();
    // every frame's scale (`scales` lines, frames in order), when adx.txt has them; else the skeleton has them
    let mut scales: Vec<u16> = Vec::new();
    for line in text.lines() {
        let g: Vec<&str> = line.split_whitespace().collect();
        if g.first() == Some(&"scales") {
            for x in &g[1..] {
                scales.push(x.parse().map_err(|_| "bad scale")?);
            }
            continue;
        }
        match g.as_slice() {
            ["coefs", a, b] => {
                c1 = a.parse().map_err(|_| "bad coefs")?;
                c2 = b.parse().map_err(|_| "bad coefs")?;
            }
            ["frames", n] => nf = n.parse().map_err(|_| "bad frames")?,
            ["keep", k] => {
                let (i, h) = k.split_once(':').ok_or("bad keep")?;
                kept.push((i.parse().map_err(|_| "bad keep")?, unhex(h).ok_or("bad keep")?));
            }
            _ => {}
        }
    }
    if buf.len() < 0x40 {
        return Err("not an ADX".into());
    }
    let start = u16::from_be_bytes([buf[2], buf[3]]) as usize + 4;
    let ch = buf[7] as usize;
    let (nch, pcm) = wav_samples(&super::vfs::read(dir.join("stream.wav")).map_err(|e| format!("stream.wav: {e}"))?)?;
    if nch != ch || pcm.len() != nf * 32 * ch {
        return Err(format!("stream.wav must have {ch} channels and {} samples", nf * 32));
    }
    if start + nf * ch * 18 > buf.len() {
        return Err("frames past the file".into());
    }
    if !scales.is_empty() {
        if scales.len() != nf * ch {
            return Err(format!("adx.txt has {} scales for {} frames", scales.len(), nf * ch));
        }
        for (k, v) in scales.iter().enumerate() {
            buf[start + k * 18..start + k * 18 + 2].copy_from_slice(&v.to_be_bytes());
        }
    }
    let mut hist = vec![(0i32, 0i32); ch];
    for fr in 0..nf {
        for c in 0..ch {
            let k = fr * ch + c;
            let o = start + k * 18;
            let scale = u16::from_be_bytes([buf[o], buf[o + 1]]) as i32;
            let (mut h1, mut h2) = hist[c];
            let keep = kept.iter().find(|x| x.0 == k).map(|x| x.1.clone());
            let mut steps = [0u8; 32];
            for i in 0..32 {
                let pred = (c1 * h1 + c2 * h2) >> 12;
                let n = match &keep {
                    Some(b) => {
                        let v = ((b[i / 2] >> if i & 1 != 0 { 0 } else { 4 }) & 15) as i32;
                        if v >= 8 { v - 16 } else { v }
                    }
                    None if scale != 0 => {
                        let t = pcm[(fr * 32 + i) * ch + c] as i32;
                        let g = (t - pred).div_euclid(scale);
                        [g - 1, g, g + 1, g + 2].into_iter().filter(|x| (-8..8).contains(x)).min_by_key(|x| ((x * scale + pred).clamp(-32768, 32767) - t).abs()).unwrap_or(if g < -8 { -8 } else { 7 })
                    }
                    None => 0,
                };
                let v = (n * scale + pred).clamp(-32768, 32767);
                steps[i] = (n & 15) as u8;
                h2 = h1;
                h1 = v;
            }
            hist[c] = (h1, h2);
            for j in 0..16 {
                buf[o + 2 + j] = steps[2 * j] << 4 | steps[2 * j + 1];
            }
        }
    }
    Ok(())
}

/// The channels and the interleaved samples of a 16-bit PCM WAV.
fn wav_samples(b: &[u8]) -> Result<(usize, Vec<i16>), String> {
    if b.len() < 12 || &b[0..4] != b"RIFF" || &b[8..12] != b"WAVE" {
        return Err("not a WAV".into());
    }
    let mut p = 12;
    let mut ch = 0usize;
    while p + 8 <= b.len() {
        let n = u32::from_le_bytes(b[p + 4..p + 8].try_into().unwrap()) as usize;
        let body = b.get(p + 8..p + 8 + n).ok_or("WAV cut short")?;
        match &b[p..p + 4] {
            b"fmt " if n >= 16 => {
                if u16::from_le_bytes([body[0], body[1]]) != 1 || u16::from_le_bytes([body[14], body[15]]) != 16 {
                    return Err("the WAV must be 16-bit PCM".into());
                }
                ch = u16::from_le_bytes([body[2], body[3]]) as usize;
            }
            b"data" => return Ok((ch, body.chunks_exact(2).map(|c| i16::from_le_bytes([c[0], c[1]])).collect())),
            _ => {}
        }
        p += 8 + n + (n & 1);
    }
    Err("WAV without data".into())
}

/// The samples of a 16-bit mono PCM WAV.
fn wav_pcm(b: &[u8]) -> Result<Vec<i16>, String> {
    if b.len() < 12 || &b[0..4] != b"RIFF" || &b[8..12] != b"WAVE" {
        return Err("not a WAV".into());
    }
    let mut p = 12;
    let mut fmt_ok = false;
    while p + 8 <= b.len() {
        let id = &b[p..p + 4];
        let n = u32::from_le_bytes(b[p + 4..p + 8].try_into().unwrap()) as usize;
        let body = b.get(p + 8..p + 8 + n).ok_or("WAV cut short")?;
        if id == b"fmt " && n >= 16 {
            let tag = u16::from_le_bytes([body[0], body[1]]);
            let ch = u16::from_le_bytes([body[2], body[3]]);
            let bits = u16::from_le_bytes([body[14], body[15]]);
            fmt_ok = tag == 1 && ch == 1 && bits == 16;
        } else if id == b"data" {
            if !fmt_ok {
                return Err("the WAV must be 16-bit mono PCM".into());
            }
            return Ok(body.chunks_exact(2).map(|c| i16::from_le_bytes([c[0], c[1]])).collect());
        }
        p += 8 + n + (n & 1);
    }
    Err("WAV without data".into())
}

// ---------------------------------------------------------------------------------------------- GSC scripts
/// A GSC script from script.txt (tools/assets/gscconv.py): `size <bytes>`, then the chunks as an indented tree. A
/// chunk is `{tag, 16, data size, arg}`, the data, then the "EOFC" trailer. GSCF / GSCD hold chunks; GSAC holds the
/// action's words; GSDT holds 4-byte values; any other chunk is `<tag> <arg> hex <data>`. Comments start with `;`. The
/// result is padded with zeros to `size`.
fn gsc_assemble(text: &str,allow_growth:bool) -> Result<Vec<u8>, String> {
    let mut total = 0usize;
    let mut items: Vec<(usize, String)> = Vec::new();
    for l in text.lines() {
        let l = l.split(';').next().unwrap().trim_end();
        if l.trim().is_empty() || l.starts_with('#') {
            continue;
        }
        if let Some(v) = l.strip_prefix("size ") {
            total = v.trim().parse().map_err(|_| "bad size")?;
            continue;
        }
        items.push((l.len() - l.trim_start_matches(' ').len(), l.trim().to_string()));
    }
    fn word(t: &str) -> Result<u32, String> {
        let f: Vec<&str> = t.split_whitespace().collect();
        let n = |i: usize| -> Result<i64, String> { f.get(i).and_then(|x| x.parse().ok()).ok_or_else(|| format!("bad word: {t}")) };
        Ok(match f[0] {
            "end" => 0,
            "cmd" => 1 | (n(2)? as u32) << 8 | (n(1)? as u32) << 16,
            "start" => 2 | ((n(1)? as u32) & 0xffff) << 16,
            "call" => 3 | ((n(1)? as u32) & 0xffff) << 16,
            "opt" => 8 | (*f.get(1).and_then(|x| x.as_bytes().first()).ok_or_else(|| format!("bad word: {t}"))? as u32) << 8 | (n(2)? as u32) << 16,
            "word" => u32::from_str_radix(f.get(1).ok_or("bad word")?.trim_start_matches("0x"), 16).map_err(|_| format!("bad word: {t}"))?,
            x if x.starts_with('@') => 10 | (n(1)? as u32) << 4 | x[1..].parse::<u32>().map_err(|_| format!("bad word: {t}"))? << 8,
            _ => return Err(format!("unknown word: {t}")),
        })
    }
    fn chunk(items: &[(usize, String)], pos: &mut usize) -> Result<Vec<u8>, String> {
        let (depth, l) = items.get(*pos).ok_or("script cut short")?.clone();
        let f: Vec<&str> = l.split_whitespace().collect();
        let name = f[0];
        let arg: i32 = f.get(1).and_then(|x| x.parse().ok()).ok_or_else(|| format!("bad chunk: {l}"))?;
        *pos += 1;
        let mut body = Vec::new();
        let inside = |pos: &usize| *pos < items.len() && items[*pos].0 > depth;
        match name {
            "GSCF" | "GSCD" => {
                while inside(pos) {
                    body.extend(chunk(items, pos)?);
                }
            }
            "GSAC" => {
                while inside(pos) {
                    body.extend(word(&items[*pos].1)?.to_le_bytes());
                    *pos += 1;
                }
            }
            "GSDT" => {
                while inside(pos) {
                    let g: Vec<&str> = items[*pos].1.split_whitespace().collect();
                    let v = g.get(2).ok_or_else(|| format!("bad value: {}", items[*pos].1))?;
                    let w = match g[1] {
                        "float" => v.parse::<f32>().map_err(|_| format!("bad value: {v}"))?.to_bits(),
                        "int" => v.parse::<i32>().map_err(|_| format!("bad value: {v}"))? as u32,
                        _ => u32::from_str_radix(v.trim_start_matches("0x"), 16).map_err(|_| format!("bad value: {v}"))?,
                    };
                    body.extend(w.to_le_bytes());
                    *pos += 1;
                }
            }
            _ => {
                if f.len() > 3 {
                    body = unhex(f[3]).ok_or_else(|| format!("bad chunk data: {l}"))?;
                }
            }
        }
        let mut out = Vec::with_capacity(body.len() + 32);
        let mut tag = [0u8; 4];
        for (i, b) in name.bytes().take(4).enumerate() {
            tag[i] = b;
        }
        out.extend(tag);
        out.extend(16u32.to_le_bytes());
        out.extend((body.len() as u32).to_le_bytes());
        out.extend(arg.to_le_bytes());
        out.extend(body);
        out.extend(b"EOFC");
        out.extend(16u32.to_le_bytes());
        out.extend([0u8; 8]);
        Ok(out)
    }
    let mut pos = 0;
    let mut out = chunk(&items, &mut pos)?;
    if pos!=items.len() {return Err("GSC: unexpected content after root chunk".into());}
    if total>64*1024*1024 || out.len()>64*1024*1024 {return Err("GSC: resource exceeds 64 MiB".into());}
    if out.len() > total && !allow_growth {
        return Err(format!("the script is {} bytes, the file has {total}", out.len()));
    }
    out.resize(total.max(out.len()), 0);
    Ok(out)
}

fn native_text_table(text:&str)->Result<Vec<u8>,String> {
    let doc=Json::parse(text)?;
    if doc.get("version").num()!=Some(1.0) {return Err("text table: unsupported version".into());}
    let Json::Arr(slots)=doc.get("slots") else {return Err("text table: slots missing".into());};
    if slots.is_empty() || slots.len()>4096 {return Err("text table: slot count outside 1..4096".into());}
    let mut out=vec![0;((slots.len()+2)*4+63)/64*64];
    let mut offsets=Vec::with_capacity(slots.len()+1);
    for slot in slots {
        offsets.push(out.len() as u32);
        let value=slot.get("text").str().ok_or("text table: text missing")?;
        if value.contains('\0') {return Err("text table: embedded NUL is not supported".into());}
        let min=match slot.get("minimumBytes") {
            Json::Null=>0.0,
            value=>value.num().ok_or("text table: minimumBytes must be numeric")?,
        };
        if !min.is_finite() || min<0.0 || min.fract()!=0.0 || min>64.0*1024.0*1024.0 {
            return Err("text table: invalid minimumBytes".into());
        }
        let mut bytes=vec![0xff,0xfe];
        for unit in value.encode_utf16() {bytes.extend_from_slice(&unit.to_le_bytes());}
        bytes.extend_from_slice(&[0,0]);
        let size=(bytes.len().max(min as usize)+63)/64*64;
        if out.len()+size>64*1024*1024 {return Err("text table: resource exceeds 64 MiB".into());}
        out.extend_from_slice(&bytes);
        out.resize(*offsets.last().unwrap() as usize+size,0);
    }
    offsets.push(out.len() as u32);
    out[..4].copy_from_slice(&(slots.len() as u32).to_le_bytes());
    for (i,offset) in offsets.iter().enumerate() {out[4+i*4..8+i*4].copy_from_slice(&offset.to_le_bytes());}
    Ok(out)
}

pub struct NativeSoundSample {
    pub path: std::path::PathBuf,
    pub rate:u32,
    pub frames:u32,
    pub loop_start:Option<u32>,
    pub queue_loop:u8,
}

pub struct NativeSoundBank {
    pub resource:u32,
    pub samples:Vec<NativeSoundSample>,
    pub effects:Vec<Option<usize>>,
}

pub fn native_sound_bank(dir:&std::path::Path)->Result<NativeSoundBank,String> {
    let doc=Json::parse(&super::vfs::read_to_string(dir.join("audio-bank.json")).map_err(|e|e.to_string())?)?;
    let integer=|v:&Json,min:f64,max:f64|->Result<i64,String> {
        let n=v.num().ok_or("audio bank: missing integer")?;
        if !n.is_finite() || n.fract()!=0.0 || n<min || n>max {return Err("audio bank: integer out of range".into());}
        Ok(n as i64)
    };
    if integer(doc.get("version"),1.0,1.0)?!=1 {unreachable!();}
    let resource=integer(doc.get("resource"),0.0,99999.0)? as u32;
    let Json::Arr(rows)=doc.get("samples") else {return Err("audio bank: samples missing".into());};
    if rows.is_empty() || rows.len()>152 {return Err("audio bank: sample count outside 1..152".into());}
    let mut samples=Vec::new();
    for row in rows {
        let file=row.get("file").str().ok_or("audio bank: sample filename missing")?;
        // WAVs belong to this bank; absolute/parent paths are not valid bindings.
        if std::path::Path::new(file).components().count()!=1 || file.contains(['/', '\\', ':']) || !file.ends_with(".wav") {
            return Err("audio bank: sample filename must be a local WAV".into());
        }
        let rate=integer(row.get("rate"),1.0,192000.0)? as u32;
        let frames=integer(row.get("frames"),1.0,u32::MAX as f64)? as u32;
        let lp=integer(row.get("loopStart"),-1.0,(frames-1) as f64)?;
        samples.push(NativeSoundSample {path:dir.join(file),rate,frames,
            loop_start:if lp<0 {None}else{Some(lp as u32)},
            queue_loop:integer(row.get("queueLoopFlag"),0.0,255.0)? as u8});
    }
    let Json::Arr(rows)=doc.get("effects") else {return Err("audio bank: effects missing".into());};
    if rows.len()>256 {return Err("audio bank: too many effects".into());}
    let mut effects=Vec::new();
    for row in rows {
        let id=integer(row,-1.0,(samples.len()-1) as f64)?;
        effects.push(if id<0 {None}else{Some(id as usize)});
    }
    Ok(NativeSoundBank {resource,samples,effects})
}

/// Named PC data, serialized to the translated readers' in-memory layout.
/// No original table, skeleton blob or original executable is consulted.
fn point_groups(text:&str)->Result<Vec<u8>,String> {
    let doc=Json::parse(text)?;
    fn word(v:&Json)->Result<u32,String> {
        let n=v.num().ok_or("point groups: expected integer")?;
        if !n.is_finite() || n<0.0 || n>u32::MAX as f64 || n.fract()!=0.0 {
            return Err("point groups: invalid integer".into());
        }
        Ok(n as u32)
    }
    fn list(v:&Json,max:usize)->Result<&Vec<Json>,String> {
        if let Json::Arr(a)=v {if a.len()<=max {return Ok(a);}}
        Err("point groups: missing or oversized array".into())
    }
    let groups=list(doc.get("groups"),4096)?;
    let mut body=vec![0;32+groups.len()*16];
    for (i,v) in [u32::from_le_bytes(*b"EVPp"),word(doc.get("version"))?,groups.len() as u32,32].iter().enumerate() {
        body[i*4..i*4+4].copy_from_slice(&v.to_le_bytes());
    }
    for (i,g) in groups.iter().enumerate() {
        let points=list(g.get("points"),65536)?;
        let offset=body.len() as u32;
        for (j,v) in [points.len() as u32,offset,word(g.get("attribute"))?,0].iter().enumerate() {
            let at=32+i*16+j*4;
            body[at..at+4].copy_from_slice(&v.to_le_bytes());
        }
        for p in points {
            for key in ["position","vector1","attributes"] {
                let a=list(p.get(key),4)?;
                if a.len()!=4 {return Err("point groups: expected four components".into());}
                for v in a {
                    let bits=if key=="attributes" {word(v)?} else {
                        let f=v.num().ok_or("point groups: expected float")? as f32;
                        if !f.is_finite() {return Err("point groups: non-finite float".into());}
                        f.to_bits()
                    };
                    body.extend_from_slice(&bits.to_le_bytes());
                }
            }
        }
    }
    let end=(128+body.len()) as u32;
    let mut out=vec![0;128];
    out[..4].copy_from_slice(&25u32.to_le_bytes());
    for i in 0..26 {out[4+i*4..8+i*4].copy_from_slice(&if i<24 {128u32}else{end}.to_le_bytes());}
    out.extend_from_slice(&body);
    Ok(out)
}

fn roster_items(text:&str)->Result<Vec<u8>,String> {
    let doc=Json::parse(text)?;
    fn number(v:&Json,min:f64,max:f64)->Result<f64,String> {
        let n=v.num().ok_or("roster/items: missing numeric field")?;
        if !n.is_finite() || n<min || n>max || n.fract()!=0.0 {
            return Err(format!("roster/items: integer {n} outside {min}..{max}"));
        }
        Ok(n)
    }
    fn array(v:&Json,n:usize)->Result<(),String> {
        if let Json::Arr(a)=v {if a.len()==n {return Ok(());}}
        Err(format!("roster/items: expected {n} elements"))
    }
    fn put(out:&mut [u8],at:usize,v:&Json,size:usize,signed:bool)->Result<(),String> {
        let bits=size*8;
        let min=if signed {-(2f64.powi(bits as i32-1))}else{0.0};
        let max=2f64.powi(bits as i32-if signed {1}else{0})-1.0;
        let value=number(v,min,max)? as i64 as u32;
        out[at..at+size].copy_from_slice(&value.to_le_bytes()[..size]);
        Ok(())
    }
    array(doc.get("characters"),161)?;
    array(doc.get("items"),350)?;
    let mut out=vec![0;23744];
    for (i,v) in [2u32,64,9728,23744].iter().enumerate() {out[i*4..i*4+4].copy_from_slice(&v.to_le_bytes());}
    for i in 0..161 {
        let c=doc.get("characters").idx(i); let at=64+i*60;
        for (key,off,size,signed) in [("aiType",0,4,true),("unknown4",4,4,true),
            ("flags",8,2,false),("costumes",10,2,false),("cost",12,2,false),("baseLevel",14,2,false)] {
            put(&mut out,at+off,c.get(key),size,signed)?;
        }
        for (key,off,n,size,signed) in [("experience",16,7,4,true),("formLinks",44,4,1,false),("unknown56",56,4,1,false)] {
            array(c.get(key),n)?;
            for j in 0..n {put(&mut out,at+off+j*size,c.get(key).idx(j),size,signed)?;}
        }
        for (key,off) in [("targetY",48),("cameraDistance",52)] {
            let value=c.get(key).num().ok_or("roster: missing camera value")? as f32;
            if !value.is_finite() {return Err("roster: non-finite camera value".into());}
            out[at+off..at+off+4].copy_from_slice(&value.to_le_bytes());
        }
    }
    for i in 0..350 {
        let c=doc.get("items").idx(i); let at=9728+i*40;
        for (key,off,size,signed) in [("type",0,1,false),("slots",3,1,false),("flags",20,4,true)] {
            put(&mut out,at+off,c.get(key),size,signed)?;
        }
        for (key,off,n,size,signed) in [("unknown1",1,2,1,false),("unknown4",4,8,1,false),
            ("stats",12,4,2,true),("abilities",24,4,4,true)] {
            array(c.get(key),n)?;
            for j in 0..n {put(&mut out,at+off+j*size,c.get(key).idx(j),size,signed)?;}
        }
    }
    Ok(out)
}

// ---------------------------------------------------------------------------------------------- models
/// The vertices of a model (model.gltf) back into the file: models.txt gives, per primitive of mesh 0, the file
/// offset of its first vertex and the vertex count; a vertex is 48 bytes, `x y z weight | nx ny nz - | u v - -`, and
/// the words from the glTF are POSITION, WEIGHTS_0.x, _NORMAL_GAME (the normals as the game has them, not of unit
/// length; NORMAL is for viewers) and TEXCOORD_0 (tools/assets/modelconv.py).
/// A glTF with its buffers (uris relative to its folder).
struct Gltf {
    g: Json,
    bins: Vec<Vec<u8>>,
}

impl Gltf {
    fn open(dir: &std::path::Path, name: &str) -> Result<Gltf, String> {
        let text = super::vfs::read_to_string(dir.join(name)).map_err(|e| format!("{name}: {e}"))?;
        let g = Json::parse(&text)?;
        let mut bins = Vec::new();
        let mut i = 0;
        while let Json::Obj(_) = g.get("buffers").idx(i) {
            let uri = g.get("buffers").idx(i).get("uri").str().ok_or("buffer without uri")?;
            bins.push(super::vfs::read(dir.join(uri_path(uri))).map_err(|e| format!("{uri}: {e}"))?);
            i += 1;
        }
        Ok(Gltf { g, bins })
    }

    /// The 4-byte words of accessor i, whatever their type (float or 32-bit integer components).
    fn words(&self, i: usize) -> Result<Vec<u32>, String> {
        let a = self.g.get("accessors").idx(i);
        let comps = match a.get("type").str() {
            Some("SCALAR") => 1,
            Some("VEC2") => 2,
            Some("VEC3") => 3,
            Some("VEC4") => 4,
            _ => return Err(format!("accessor {i}: no type")),
        };
        if !matches!(a.get("componentType").num(), Some(5126.0) | Some(5125.0)) {
            return Err(format!("accessor {i} is not of 4-byte components"));
        }
        let v = self.g.get("bufferViews").idx(a.get("bufferView").num().ok_or("no bufferView")? as usize);
        let bin = self.bins.get(v.get("buffer").num().unwrap_or(0.0) as usize).ok_or("no such buffer")?;
        let count = a.get("count").num().ok_or("no count")? as usize;
        let base = v.get("byteOffset").num().unwrap_or(0.0) as usize + a.get("byteOffset").num().unwrap_or(0.0) as usize;
        let b = bin.get(base..base + count * comps * 4).ok_or("accessor past the buffer")?;
        Ok(b.chunks_exact(4).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect())
    }

    /// The values of an index accessor (unsigned 8, 16 or 32 bits, SCALAR).
    fn indices(&self, i: usize) -> Result<Vec<u32>, String> {
        let a = self.g.get("accessors").idx(i);
        let size = match a.get("componentType").num() {
            Some(5121.0) => 1,
            Some(5123.0) => 2,
            Some(5125.0) => 4,
            _ => return Err(format!("accessor {i} is not of indices")),
        };
        let v = self.g.get("bufferViews").idx(a.get("bufferView").num().ok_or("no bufferView")? as usize);
        let bin = self.bins.get(v.get("buffer").num().unwrap_or(0.0) as usize).ok_or("no such buffer")?;
        let count = a.get("count").num().ok_or("no count")? as usize;
        let base = v.get("byteOffset").num().unwrap_or(0.0) as usize + a.get("byteOffset").num().unwrap_or(0.0) as usize;
        let b = bin.get(base..base + count * size).ok_or("accessor past the buffer")?;
        Ok(b.chunks_exact(size).map(|c| c.iter().rev().fold(0u32, |x, &y| x << 8 | y as u32)).collect())
    }

    /// The words (f32 bits) of float accessor i of `comps` components.
    fn accessor(&self, i: usize, comps: usize) -> Result<Vec<u32>, String> {
        let g = &self.g;
        let a = g.get("accessors").idx(i);
        if a.get("componentType").num() != Some(5126.0) {
            return Err(format!("accessor {i} is not float"));
        }
        let want = ["", "SCALAR", "VEC2", "VEC3", "VEC4"][comps];
        if a.get("type").str() != Some(want) {
            return Err(format!("accessor {i} is not {want}"));
        }
        let v = g.get("bufferViews").idx(a.get("bufferView").num().ok_or("no bufferView")? as usize);
        let bin = self.bins.get(v.get("buffer").num().unwrap_or(0.0) as usize).ok_or("no such buffer")?;
        let count = a.get("count").num().ok_or("no count")? as usize;
        let stride = v.get("byteStride").num().map(|x| x as usize).unwrap_or(comps * 4);
        let base = v.get("byteOffset").num().unwrap_or(0.0) as usize + a.get("byteOffset").num().unwrap_or(0.0) as usize;
        let mut out = Vec::with_capacity(count * comps);
        for k in 0..count {
            for c in 0..comps {
                let o = base + k * stride + c * 4;
                let b = bin.get(o..o + 4).ok_or("accessor past the buffer")?;
                out.push(u32::from_le_bytes(b.try_into().unwrap()));
            }
        }
        Ok(out)
    }
}

/// The collision of a stage (collision.gltf, tools/assets/colconv.py) back into the file: per mesh its header, the
/// vertices (POSITION and the w accessor), the polygons (flags, the triangle indices, the planes) and the box tree
/// (boxes, links), at the offsets of the header; then the zones (StgColZone: mark, mesh, neighbours, records,
/// centre and half size) and their lists. Offsets count words from the stage's start (`base`).
fn collision_apply(buf: &mut [u8], dir: &std::path::Path) -> Result<(), String> {
    let gl = Gltf::open(dir, "collision.gltf")?;
    let g = &gl.g;
    let sx = g.get("scenes").idx(0).get("extras");
    let int = |v: &Json, what: &str| -> Result<i64, String> {
        let x = v.num().ok_or_else(|| format!("no {what}"))?;
        if x.fract() != 0.0 || !(-2147483648.0..=4294967295.0).contains(&x) {
            return Err(format!("{what} is not a 32-bit integer"));
        }
        Ok(x as i64)
    };
    let base = int(sx.get("base"), "base")? as usize;
    let zones_at = int(sx.get("zones_at"), "zones_at")? as usize;
    let put = |buf: &mut [u8], at: usize, ws: &[u32]| -> Result<(), String> {
        let b = buf.get_mut(at..at + ws.len() * 4).ok_or("past the file")?;
        for (k, w) in ws.iter().enumerate() {
            b[k * 4..k * 4 + 4].copy_from_slice(&w.to_le_bytes());
        }
        Ok(())
    };
    let acc = |v: &Json, what: &str| -> Result<Vec<u32>, String> { gl.words(int(v, what)? as usize) };
    // the meshes with triangles, then the empty ones (in the scene's extras)
    let mut extras: Vec<&Json> = Vec::new();
    let mut i = 0;
    while let m @ Json::Obj(_) = g.get("meshes").idx(i) {
        extras.push(m.get("extras"));
        i += 1;
    }
    let drawn = extras.len();
    let mut i = 0;
    while let e @ Json::Obj(_) = sx.get("empty_meshes").idx(i) {
        extras.push(e);
        i += 1;
    }
    let mut mesh_at = Vec::with_capacity(extras.len());
    for (mi, ex) in extras.iter().enumerate() {
        let at = int(ex.get("at"), "mesh at")? as usize;
        mesh_at.push(at);
        let mut h = [0i64; 6];
        for (k, v) in h.iter_mut().enumerate() {
            *v = int(ex.get("header").idx(k), "mesh header")?;
        }
        put(buf, at, &h.map(|x| x as u32))?;
        if mi >= drawn {
            continue;
        }
        let (nodes, polys) = (h[0].max(0) as usize, h[1].max(0) as usize);
        let at_of = |w: i64| base + w as usize * 4;
        let prim = g.get("meshes").idx(mi).get("primitives").idx(0);
        let pos = acc(prim.get("attributes").get("POSITION"), "POSITION")?;
        let w = acc(ex.get("w"), "w")?;
        if pos.len() != w.len() * 3 {
            return Err(format!("mesh {mi}: POSITION and w differ in count"));
        }
        for v in 0..w.len() {
            put(buf, at_of(h[5]) + v * 16, &[pos[v * 3], pos[v * 3 + 1], pos[v * 3 + 2], w[v]])?;
        }
        let idx = acc(prim.get("indices"), "indices")?;
        let planes = acc(ex.get("planes"), "planes")?;
        let flags = ex.get("flags");
        if idx.len() != polys * 3 || planes.len() != polys * 4 {
            return Err(format!("mesh {mi}: the file has {polys} polygons"));
        }
        for p in 0..polys {
            if idx[p * 3..p * 3 + 3].iter().any(|&v| v as usize >= w.len()) {
                return Err(format!("mesh {mi}: polygon {p} names a vertex it does not have"));
            }
            let f = int(flags.idx(p), "polygon flags")? as u32;
            put(buf, at_of(h[4]) + p * 32, &[f, idx[p * 3], idx[p * 3 + 1], idx[p * 3 + 2], planes[p * 4], planes[p * 4 + 1], planes[p * 4 + 2], planes[p * 4 + 3]])?;
        }
        let boxes = acc(ex.get("boxes"), "boxes")?;
        let links = acc(ex.get("links"), "links")?;
        if boxes.len() != nodes * 6 || links.len() != nodes * 2 {
            return Err(format!("mesh {mi}: the file has {nodes} nodes"));
        }
        for n in 0..nodes {
            let mut ws = boxes[n * 6..n * 6 + 6].to_vec();
            ws.extend_from_slice(&links[n * 2..n * 2 + 2]);
            put(buf, at_of(h[3]) + n * 32, &ws)?;
        }
    }
    let word_of = |v: &Json| -> Result<u32, String> {
        match v {
            Json::Null => Ok(0),
            _ => {
                let k = int(v, "mesh number")? as usize;
                let at = *mesh_at.get(k).ok_or("no such mesh")?;
                Ok(((at - base) / 4) as u32)
            }
        }
    };
    let boxes = acc(sx.get("zone_boxes"), "zone_boxes")?;
    let mut z = 0;
    while let zone @ Json::Obj(_) = sx.get("zones").idx(z) {
        let list_at = |key: &str| -> Result<Option<usize>, String> {
            match zone.get(key) {
                Json::Null => Ok(None),
                v => Ok(Some(int(v, key)? as usize)),
            }
        };
        let (near_at, recs_at) = (list_at("near_at")?, list_at("recs_at")?);
        let count = |key: &str| -> usize {
            let mut n = 0;
            while !matches!(zone.get(key).idx(n), Json::Null) {
                n += 1;
            }
            n
        };
        let (nn, nr) = (count("near"), count("recs"));
        let rel = |at: Option<usize>| at.map_or(0, |a| ((a - base) / 4) as u32);
        let mut ws = vec![
            int(zone.get("mark"), "mark")? as u32,
            word_of(zone.get("mesh"))?,
            nn as u32,
            rel(near_at),
            int(zone.get("unk10"), "unk10")? as u32,
            nr as u32,
            rel(recs_at),
            int(zone.get("unk1C"), "unk1C")? as u32,
        ];
        ws.extend_from_slice(boxes.get(z * 8..z * 8 + 8).ok_or("zone_boxes too short")?);
        put(buf, zones_at + z * 0x40, &ws)?;
        if let Some(a) = near_at {
            for k in 0..nn {
                put(buf, a + k * 4, &[int(zone.get("near").idx(k), "neighbour")? as u32])?;
            }
        }
        if let Some(a) = recs_at {
            for k in 0..nr {
                let r = zone.get("recs").idx(k);
                put(buf, a + k * 16, &[int(r.idx(0), "broken")? as u32, int(r.idx(1), "object")? as u32, word_of(r.idx(2))?, int(r.idx(3), "record word")? as u32])?;
            }
        }
        z += 1;
    }
    Ok(())
}

/// The drawing chains of a stage's meshes (stage-chains.json, tools/assets/stagechain.py): per chain a DMA RET tag,
/// optionally UNPACK of one setting quadword + MSCNT, per batch UNPACK of its 3 header quadwords (NLOOP = the vertex
/// count of the primitive), UNPACK of the vertices (x y z w | r g b a | s t q 0 from model.gltf), MSCNT; NOPs to the
/// quadword, a DMA END tag and `tail - 16` zeros.
fn stage_chains_apply(buf: &mut [u8], dir: &std::path::Path, text: &str, gltf: &str) -> Result<(), String> {
    const MSCNT: u32 = 0x1700_0000;
    let doc = Json::parse(text)?;
    let gl = Gltf::open(dir, gltf)?;
    let prims = gl.g.get("meshes").idx(0).get("primitives");
    let word = |v: &Json| -> Result<u32, String> {
        v.num().filter(|x| *x >= 0.0 && *x <= u32::MAX as f64 && x.fract() == 0.0).map(|x| x as u32).ok_or_else(|| "not a word".to_string())
    };
    let verts = |p: usize| -> Result<Vec<u32>, String> {
        let at = prims.idx(p).get("attributes");
        let attr = |name: &str| at.get(name).num().map(|x| x as usize);
        let need = |name: &str| attr(name).ok_or_else(|| format!("primitive {p}: no {name}"));
        let pos = gl.accessor(need("POSITION")?, 3)?;
        let col = gl.accessor(need("_COLOR_GAME")?, 4)?;
        let uv = gl.accessor(need("TEXCOORD_0")?, 2)?;
        let n = pos.len() / 3;
        let one = |name: &str| -> Result<Vec<u32>, String> {
            match attr(name) {
                Some(a) => gl.accessor(a, 1),
                None => Ok(vec![0x3F80_0000; n]),
            }
        };
        let (w, q) = (one("_W_GAME")?, one("_Q_GAME")?);
        if col.len() != n * 4 || uv.len() != n * 2 || w.len() != n || q.len() != n {
            return Err(format!("primitive {p}: attributes of different counts"));
        }
        let mut out = Vec::with_capacity(n * 12);
        for i in 0..n {
            out.extend_from_slice(&pos[i * 3..i * 3 + 3]);
            out.push(w[i]);
            out.extend_from_slice(&col[i * 4..i * 4 + 4]);
            out.extend_from_slice(&uv[i * 2..i * 2 + 2]);
            out.push(q[i]);
            out.push(0);
        }
        Ok(out)
    };
    let mut spans = Vec::new();
    let mut c = 0;
    while let ch @ Json::Obj(_) = doc.get("chains").idx(c) {
        c += 1;
        let at = word(ch.get("at"))? as usize;
        let tail = word(ch.get("tail"))? as usize;
        let mut w: Vec<u32> = Vec::new();
        if let Json::Arr(pre) = ch.get("prefix") {
            if pre.len() != 4 {
                return Err(format!("chain at {at}: the prefix is not 4 words"));
            }
            w.push(0x6C01_8000);
            for v in pre {
                w.push(word(v)?);
            }
            w.push(MSCNT);
        }
        let mut b = 0;
        while let batch @ Json::Obj(_) = ch.get("batches").idx(b) {
            b += 1;
            let v = verts(word(batch.get("primitive"))? as usize)?;
            let n = v.len() / 12;
            if !(3..=0x7FFF).contains(&n) || 3 * n > 0xFF {
                return Err(format!("chain at {at}: a batch of {n} vertices"));
            }
            let mut hdr = Vec::with_capacity(12);
            let mut k = 0;
            while let x @ Json::Num(_) = batch.get("gs").idx(k) {
                hdr.push(word(x)?);
                k += 1;
            }
            if hdr.len() != 12 {
                return Err(format!("chain at {at}: a header of {} words", hdr.len()));
            }
            hdr[8] |= n as u32;
            w.push(0x6C03_8000);
            w.extend_from_slice(&hdr);
            w.push(0x6C00_8003 | (3 * n as u32) << 16);
            w.extend_from_slice(&v);
            w.push(MSCNT);
        }
        while (w.len() + 2) % 4 != 0 {
            w.push(0);
        }
        let qwc = (w.len() - 2) / 4;
        let len = 8 + w.len() * 4 + 16;
        if tail < 16 || qwc > 0xFFFF {
            return Err(format!("chain at {at}: bad size"));
        }
        let end = at + len + tail - 16;
        let out = buf.get_mut(at..end).ok_or_else(|| format!("chain at {at} past the file"))?;
        out.fill(0);
        out[0..4].copy_from_slice(&(0x6000_0000u32 | qwc as u32).to_le_bytes());
        for (i, x) in w.iter().enumerate() {
            out[8 + i * 4..12 + i * 4].copy_from_slice(&x.to_le_bytes());
        }
        out[len - 16..len - 12].copy_from_slice(&0x7000_0000u32.to_le_bytes());
        spans.push((at, end));
    }
    spans.sort();
    if spans.windows(2).any(|s| s[0].1 > s[1].0) {
        return Err("a chain is longer than the room before the next (the primitives have more vertices)".into());
    }
    Ok(())
}

/// The upload packet around a texture's data: three zero words, VIF DIRECT of length / 16 + 7 quadwords, GIF tag
/// (A+D) BITBLTBUF 0, TRXPOS 0, TRXREG w x h, TRXDIR 0, GIF tag IMAGE of length / 16, [data], GIF tag TEXFLUSH.
fn texture_packets_apply(buf: &mut [u8], text: &str) -> Result<(), String> {
    for line in text.lines() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let v: Vec<usize> = line.split_whitespace().map(|x| x.parse().map_err(|_| format!("bad line: {line}"))).collect::<Result<_, _>>()?;
        let [o, n, w, h] = v[..] else { return Err(format!("bad line: {line}")) };
        let q = (n / 16) as u32;
        if o < 0x60 || o + n + 0x20 > buf.len() || n % 16 != 0 {
            return Err(format!("packet at {o} past the file"));
        }
        let head: [u32; 24] = [0, 0, 0, 0x5000_0000 | (q + 7), 0x8003, 0x1000_0000, 0xE, 0, 0, 0, 0x51, 0, w as u32, h as u32, 0x52, 0, 0, 0, 0x53, 0, 0x8000 | q, 0x0800_0000, 0, 0];
        let tail: [u32; 8] = [0x8001, 0x1000_0000, 0xE, 0, 0, 0, 0x3F, 0];
        for (i, x) in head.iter().enumerate() {
            buf[o - 0x60 + 4 * i..o - 0x5C + 4 * i].copy_from_slice(&x.to_le_bytes());
        }
        for (i, x) in tail.iter().enumerate() {
            buf[o + n + 4 * i..o + n + 4 + 4 * i].copy_from_slice(&x.to_le_bytes());
        }
    }
    Ok(())
}

/// The Flash-like movies of a menu file (movies.json, tools/assets/fodconv.py) back into the file: each written
/// whole at its offset from its tags, records, frames and frame tags.
fn movies_apply(buf: &mut [u8], text: &str) -> Result<(), String> {
    let doc = Json::parse(text)?;
    let mut i = 0;
    while let m @ Json::Obj(_) = doc.get("movies").idx(i) {
        i += 1;
        let at = m.get("at").num().ok_or("movie without at")? as usize;
        let b = fod_movie(m)?;
        buf.get_mut(at..at + b.len()).ok_or_else(|| format!("movie at {at} past the file"))?.copy_from_slice(&b);
    }
    Ok(())
}

fn jn(v: &Json, what: &str) -> Result<i64, String> {
    v.num().filter(|x| x.fract() == 0.0).map(|x| x as i64).ok_or_else(|| format!("movie: bad {what}"))
}

fn jlist(v: &Json) -> &[Json] {
    match v {
        Json::Arr(a) => a,
        _ => &[],
    }
}

/// Bits most significant first, padded with zeros to the byte.
struct FodBits<'a> { out: &'a mut Vec<u8>, acc: u32, n: u32 }

impl FodBits<'_> {
    fn put(&mut self, v: i64, bits: u32) {
        for k in (0..bits).rev() {
            self.acc = self.acc << 1 | ((v >> k) & 1) as u32;
            self.n += 1;
            if self.n == 8 {
                self.out.push(self.acc as u8);
                self.acc = 0;
                self.n = 0;
            }
        }
    }
    fn flush(&mut self) {
        if self.n > 0 {
            self.out.push((self.acc << (8 - self.n)) as u8);
            self.acc = 0;
            self.n = 0;
        }
    }
}

fn fod_str(out: &mut Vec<u8>, v: &Json) -> Result<(), String> {
    let s = v.str().ok_or("movie: bad string")?;
    // Latin-1
    for c in s.chars() {
        let c = c as u32;
        if c > 255 {
            return Err("movie: a string with a character beyond Latin-1".into());
        }
        out.push(c as u8);
    }
    out.push(0);
    Ok(())
}

fn fod_mtx(out: &mut Vec<u8>, m: &Json) -> Result<(), String> {
    let f = jn(m.get("f"), "matrix")? as u8;
    out.push(f);
    let mut b = FodBits { out, acc: 0, n: 0 };
    for (key, mask) in [("s", 0x11u8), ("r", 0x0A), ("t", 0x24)] {
        if f & mask != 0 {
            let v = jlist(m.get(key));
            let n = jn(v.first().unwrap_or(&NULL), "matrix width")? as u32;
            b.put(n as i64, 5);
            for x in &v[1..] {
                b.put(jn(x, "matrix value")?, n);
            }
        }
    }
    b.flush();
    Ok(())
}

fn fod_cx(out: &mut Vec<u8>, cx: &Json) -> Result<(), String> {
    let f = jn(cx.get("f"), "colour transform")? as u8;
    out.push(f);
    if f != 0 {
        let n = jn(cx.get("n"), "colour transform width")? as u32;
        let mut b = FodBits { out, acc: 0, n: 0 };
        b.put(n as i64, 4);
        for x in jlist(cx.get("v")) {
            b.put(jn(x, "colour transform value")?, n);
        }
        b.flush();
    }
    Ok(())
}

fn fod_tags(out: &mut Vec<u8>, tags: &Json) -> Result<(), String> {
    for t in jlist(tags) {
        let code = jn(t.get("t"), "frame tag")? as u16;
        out.extend_from_slice(&code.to_le_bytes());
        let u16_ = |out: &mut Vec<u8>, k: &str| -> Result<(), String> { out.extend_from_slice(&(jn(t.get(k), k)? as u16).to_le_bytes()); Ok(()) };
        match code {
            4 | 5 => {
                u16_(out, "depth")?;
                let kind = jn(t.get("kind"), "kind")?;
                out.push(kind as u8);
                u16_(out, "id")?;
                if code == 4 {
                    fod_mtx(out, t.get("m"))?;
                    if kind == 6 {
                        fod_cx(out, t.get("cx"))?;
                    }
                }
            }
            26 => {
                let f = jn(t.get("flags"), "flags")? as u8;
                out.push(f);
                u16_(out, "depth")?;
                if f & 2 != 0 {
                    out.push(jn(t.get("kind"), "kind")? as u8);
                    u16_(out, "id")?;
                }
                if f & 4 != 0 {
                    fod_mtx(out, t.get("m"))?;
                }
                if f & 8 != 0 {
                    fod_cx(out, t.get("cx"))?;
                }
                if f & 0x22 == 0x22 {
                    fod_str(out, t.get("name"))?;
                }
            }
            28 => u16_(out, "depth")?,
            12 => {
                for a in jlist(t.get("actions")) {
                    let c = jn(a.idx(0), "action")? as u8;
                    out.push(c);
                    match c {
                        0x81 => out.extend_from_slice(&(jn(a.idx(1), "frame")? as u16).to_le_bytes()),
                        0x83 => {
                            fod_str(out, a.idx(1))?;
                            fod_str(out, a.idx(2))?;
                        }
                        0x8A => {
                            for x in jlist(a.idx(1)) {
                                out.push(jn(x, "byte")? as u8);
                            }
                        }
                        0x8B | 0x8C => fod_str(out, a.idx(1))?,
                        _ => {}
                    }
                }
                out.push(0);
            }
            1 => {}
            _ => return Err(format!("movie: frame tag {code}")),
        }
    }
    Ok(())
}

fn fod_movie(m: &Json) -> Result<Vec<u8>, String> {
    let mut out = b"FOD\x11LIT\0".to_vec();
    for t in jlist(m.get("tags")) {
        let code = jn(t.get("code"), "tag")? as u8;
        out.push(code);
        out.push(jn(t.get("u"), "tag")? as u8);
        out.extend_from_slice(&(jn(t.get("count"), "tag")? as u16).to_le_bytes());
        let size_at = out.len();
        out.extend_from_slice(&[0; 4]);
        for r in jlist(t.get("records")) {
            out.push(jn(r.get("i"), "record")? as u8);
            out.push(jn(r.get("u"), "record")? as u8);
            out.extend_from_slice(&(jn(r.get("n"), "record")? as u16).to_le_bytes());
            let rec_at = out.len();
            out.extend_from_slice(&[0; 4]);
            match code {
                2 => out.extend_from_slice(&(jn(r.get("id"), "id")? as u16).to_le_bytes()),
                3 => {
                    out.extend_from_slice(&(jn(r.get("w"), "w")? as u16).to_le_bytes());
                    out.extend_from_slice(&(jn(r.get("h"), "h")? as u16).to_le_bytes());
                }
                4 => {
                    let quads = jlist(r.get("quads"));
                    out.extend_from_slice(&(quads.len() as u16).to_le_bytes());
                    for q in quads {
                        out.extend_from_slice(&(jn(q.idx(0), "quad")? as i16).to_le_bytes());
                        out.extend_from_slice(&(jn(q.idx(1), "quad")? as u16).to_le_bytes());
                        out.extend_from_slice(&(jn(q.idx(2), "quad")? as u32).to_le_bytes());
                        for k in 3..7 {
                            out.extend_from_slice(&(jn(q.idx(k), "quad")? as i32).to_le_bytes());
                        }
                    }
                }
                6 | 7 => {
                    for fr in jlist(r.get("frames")) {
                        fod_str(&mut out, fr.get("name"))?;
                        let at = out.len();
                        out.extend_from_slice(&[0; 2]);
                        fod_tags(&mut out, fr.get("tags"))?;
                        let size = match fr.get("size") { Json::Null => out.len() - at - 2, v => jn(v, "size")? as usize };
                        out[at..at + 2].copy_from_slice(&(size as u16).to_le_bytes());
                    }
                    fod_str(&mut out, r.get("last"))?;
                    out.extend_from_slice(&[0; 2]);
                }
                _ => return Err(format!("movie: tag {code}")),
            }
            let n = (out.len() - rec_at - 4) as u32;
            out[rec_at..rec_at + 4].copy_from_slice(&n.to_le_bytes());
        }
        let n = (out.len() - size_at - 4) as u32;
        out[size_at..size_at + 4].copy_from_slice(&n.to_le_bytes());
    }
    Ok(out)
}

/// GS uploads no texture entry describes (raw-textures.txt + PNGs, tools/assets/rawtex.py): per line the PNG, the
/// data offset, length, width, height and `rgba` (the data is the PNG's RGBA bytes) or `bytes` (the data is the
/// palette PNG's indices); the packet around the data as texture_packets_apply writes it.
fn raw_textures_apply(buf: &mut [u8], dir: &std::path::Path, text: &str) -> Result<(), String> {
    let mut packets = String::new();
    for line in text.lines() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let f: Vec<&str> = line.split_whitespace().collect();
        let [name, o, n, w, h, _kind] = f[..] else { return Err(format!("bad line: {line}")) };
        let num = |x: &str| x.parse::<usize>().map_err(|_| format!("bad line: {line}"));
        let (o, n) = (num(o)?, num(n)?);
        let png = png_read(&super::vfs::read(dir.join(name)).map_err(|e| format!("{name}: {e}"))?)?;
        let data = png.data.get(..n).ok_or_else(|| format!("{name}: fewer than {n} bytes"))?;
        buf.get_mut(o..o + n).ok_or_else(|| format!("{name}: past the file"))?.copy_from_slice(data);
        packets.push_str(&format!("{o} {n} {w} {h}\n"));
    }
    texture_packets_apply(buf, &packets)
}

/// The animated surfaces of a stage (surfaces.gltf + surfaces.txt, tools/assets/surfconv.py): per line a primitive
/// of mesh 0, the file offset of its vertices and their count; a vertex is 12 words, POSITION, _W_GAME, TEXCOORD_0,
/// _STQ_GAME, _COLOR_GAME.
fn surfaces_apply(buf: &mut [u8], dir: &std::path::Path, text: &str) -> Result<(), String> {
    let gl = Gltf::open(dir, "surfaces.gltf")?;
    let prims = gl.g.get("meshes").idx(0).get("primitives");
    for line in text.lines() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let v: Vec<usize> = line.split_whitespace().map(|x| x.parse().map_err(|_| format!("bad line: {line}"))).collect::<Result<_, _>>()?;
        let [p, o, n] = v[..] else { return Err(format!("bad line: {line}")) };
        let at = prims.idx(p).get("attributes");
        let get = |name: &str, comps: usize| -> Result<Vec<u32>, String> {
            let i = at.get(name).num().ok_or_else(|| format!("primitive {p}: no {name}"))? as usize;
            let w = gl.accessor(i, comps)?;
            if w.len() != n * comps {
                return Err(format!("primitive {p}: {name} has {} values for {n} vertices", w.len() / comps));
            }
            Ok(w)
        };
        let (pos, w, st, stq, col) = (get("POSITION", 3)?, get("_W_GAME", 1)?, get("TEXCOORD_0", 2)?, get("_STQ_GAME", 2)?, get("_COLOR_GAME", 4)?);
        let out = buf.get_mut(o..o + 48 * n).ok_or_else(|| format!("primitive {p} past the file"))?;
        for k in 0..n {
            let words = [pos[3 * k], pos[3 * k + 1], pos[3 * k + 2], w[k], st[2 * k], st[2 * k + 1], stq[2 * k], stq[2 * k + 1],
                         col[4 * k], col[4 * k + 1], col[4 * k + 2], col[4 * k + 3]];
            for (j, x) in words.iter().enumerate() {
                out[48 * k + 4 * j..48 * k + 4 * j + 4].copy_from_slice(&x.to_le_bytes());
            }
        }
    }
    Ok(())
}

/// The files of typed records (tools/assets/layout.py) a file folder may have, applied in this order.
const LAYOUT_FILES: [&str; 8] = ["texture-entries.json", "camera-tables.json", "stage-layout.json", "surface-tables.json", "model-tables.json", "effect-tables.json", "sound-tables.json", "file-tables.json"];

/// Typed records (tools/assets/layout.py) back into the file: per record type its size and fields
/// [name, offset, kind]; kinds u8 s8 u16 s16 u32 s32 f32 and ptr (a file offset, written as words from `base`, null
/// for 0, -1 for 0xFFFFFFFF), arrays as "kind[n]" or "kind[*]" (as many as the record has); a float as {"bits": n}
/// when it is not written as a number.
fn layout_apply(buf: &mut [u8], text: &str) -> Result<(), String> {
    let doc = Json::parse(text)?;
    let base = doc.get("base").num().unwrap_or(0.0) as i64;
    let Json::Obj(types) = doc.get("types") else { return Err("no types".into()) };
    // n: None = one value, Some(0) = as many as the record has
    struct Field<'a> { name: &'a str, at: usize, kind: &'a str, n: Option<usize> }
    let mut tys: Vec<(&str, Vec<Field>)> = Vec::new();
    for (name, t) in types {
        let mut fields = Vec::new();
        let mut i = 0;
        while let f @ Json::Arr(_) = t.get("fields").idx(i) {
            i += 1;
            let fname = f.idx(0).str().ok_or("field without name")?;
            let at = f.idx(1).num().ok_or("field without offset")? as usize;
            let k = f.idx(2).str().ok_or("field without kind")?;
            let (kind, n) = match k.find('[') {
                Some(p) if &k[p..] == "[*]" => (&k[..p], Some(0)),
                Some(p) => (&k[..p], Some(k[p + 1..k.len() - 1].parse::<usize>().map_err(|_| format!("bad kind {k}"))?.max(1))),
                None => (k, None),
            };
            if !matches!(kind, "u8" | "s8" | "u16" | "s16" | "u32" | "s32" | "f32" | "ptr") {
                return Err(format!("bad kind {k}"));
            }
            fields.push(Field { name: fname, at, kind, n });
        }
        tys.push((name.as_str(), fields));
    }
    let mut r = 0;
    while let rec @ Json::Obj(_) = doc.get("records").idx(r) {
        r += 1;
        let tname = rec.get("type").str().ok_or("record without type")?;
        let (_, fields) = tys.iter().find(|t| t.0 == tname).ok_or_else(|| format!("no type {tname}"))?;
        let at = rec.get("at").num().ok_or("record without at")? as usize;
        for f in fields {
            let width = match f.kind { "u8" | "s8" => 1, "u16" | "s16" => 2, _ => 4 };
            let v = rec.get(f.name);
            let count = match (f.n, v) {
                (None, _) => 1,
                (Some(0), Json::Arr(a)) => a.len(),
                (Some(n), _) => n,
            };
            for i in 0..count {
                let x = if f.n.is_some() { v.idx(i) } else { v };
                let o = at + f.at + i * width;
                let bits: u32 = match (f.kind, x) {
                    ("ptr", Json::Null) => 0,
                    ("ptr", Json::Num(p)) if *p == -1.0 => u32::MAX,
                    ("ptr", Json::Num(p)) => ((*p as i64 - base) / 4) as u32,
                    ("f32", Json::Num(p)) => (*p as f32).to_bits(),
                    ("f32", Json::Obj(_)) => x.get("bits").num().ok_or("bad float")? as u32,
                    (_, Json::Num(p)) => *p as i64 as u32,
                    _ => return Err(format!("record at {at}: bad value of {}", f.name)),
                };
                let b = buf.get_mut(o..o + width).ok_or_else(|| format!("record at {at} past the file"))?;
                b.copy_from_slice(&bits.to_le_bytes()[..width]);
            }
        }
    }
    Ok(())
}

/// A fighter model (the pmdl, entry 2, and the loose parts) built from model.gltf + model.json
/// (tools/assets/pmdlconv.py): the header, the node records, and per node a DMA chain whose batches (at most 30
/// vertices) are UNPACK of the 5 header quadwords, UNPACK of the vertices, MSCNT. A primitive is drawn as a triangle
/// strip: its vertices in order when its triangles are those of the strip, else a strip is made of its triangles.
/// A model or part of another size than the original takes that room in the file: the entries after it move (the
/// file's directory follows), up to the game's buffer for a fighter file.
fn model_build(buf: &mut Vec<u8>, dir: &std::path::Path, text: &str) -> Result<(), String> {
    /// The game's buffer for a fighter file (BTL_RES_MODEL_SIZE, include/battle/btl_obj.h).
    const FILE_ROOM: usize = 0xCE000;
    const MAX_VERTS: usize = 30;
    let j = Json::parse(text)?;
    let gl = Gltf::open(dir, "model.gltf")?;
    let int = |v: &Json, what: &str| -> Result<i64, String> {
        let x = v.num().ok_or_else(|| format!("no {what}"))?;
        if x.fract() != 0.0 || !(-2147483648.0..=4294967295.0).contains(&x) {
            return Err(format!("{what} is not a 32-bit integer"));
        }
        Ok(x as i64)
    };
    fn list(v: &Json) -> Vec<&Json> {
        let mut out = Vec::new();
        while let x @ (Json::Num(_) | Json::Obj(_) | Json::Arr(_)) = v.idx(out.len()) {
            out.push(x);
        }
        out
    }
    // the strip of primitive p of mesh m, as the game's vertex words
    let strip = |m: usize, p: usize| -> Result<Vec<[u32; 12]>, String> {
        let prim = gl.g.get("meshes").idx(m).get("primitives").idx(p);
        let at = prim.get("attributes");
        let attr = |name: &str| at.get(name).num().map(|x| x as usize);
        let need = |name: &str| attr(name).ok_or_else(|| format!("mesh {m} primitive {p}: no {name}"));
        let pos = gl.accessor(need("POSITION")?, 3)?;
        let nrm = gl.accessor(need("_NORMAL_GAME")?, 3)?;
        let uv = gl.accessor(need("TEXCOORD_0")?, 2)?;
        let n = pos.len() / 3;
        let wt: Vec<u32> = match attr("WEIGHTS_0") {
            Some(a) => gl.accessor(a, 4)?.chunks(4).map(|c| c[0]).collect(),
            None => gl.accessor(need("_WEIGHT_GAME")?, 1)?,
        };
        let q = match attr("_Q_GAME") { Some(a) => gl.accessor(a, 1)?, None => vec![0x3F80_0000; n] };
        let fl = match attr("_FLAG_GAME") { Some(a) => gl.accessor(a, 1)?, None => vec![0; n] };
        if nrm.len() != n * 3 || uv.len() != n * 2 || wt.len() != n || q.len() != n || fl.len() != n {
            return Err(format!("mesh {m} primitive {p}: the attributes differ in count"));
        }
        let vert = |i: usize| [pos[i * 3], pos[i * 3 + 1], pos[i * 3 + 2], wt[i], nrm[i * 3], nrm[i * 3 + 1], nrm[i * 3 + 2], fl[i], uv[i * 2], uv[i * 2 + 1], q[i], 0];
        let idx = match prim.get("indices").num() {
            Some(a) => gl.indices(a as usize)?,
            None => (0..n as u32).collect(),
        };
        if idx.len() % 3 != 0 || idx.iter().any(|&i| i as usize >= n) {
            return Err(format!("mesh {m} primitive {p}: bad triangles"));
        }
        // the triangles of the strip of n vertices (as the extraction writes them)?
        let tris = idx.len() / 3;
        let canonical = tris + 2 == n
            && (0..tris).all(|t| {
                let k = t as u32;
                let want = if t % 2 == 0 { [k, k + 1, k + 2] } else { [k + 1, k, k + 2] };
                idx[t * 3..t * 3 + 3] == want
            });
        if canonical {
            return Ok((0..n).map(vert).collect());
        }
        Ok(stripify(&idx).into_iter().map(vert).collect())
    };
    // a chain: DMA RET, the batches, NOPs to the quadword, DMA END, zeros (`tail` bytes from the END tag)
    let chain = |batches: &[(Vec<u32>, &[[u32; 12]])], tail: usize| -> Vec<u8> {
        let mut w: Vec<u32> = Vec::new();
        for (hdr, verts) in batches {
            let n = verts.len() as u32;
            w.push(0x6C05_8000);
            for (k, &x) in hdr.iter().enumerate() {
                w.push(if k == 12 || k == 16 { x | n } else { x });
            }
            w.push(0x6C00_0000 | (3 * n) << 16 | 0x8005);
            for v in verts.iter() {
                w.extend_from_slice(v);
            }
            w.push(0x1700_0000);
        }
        while (w.len() + 2) % 4 != 0 {
            w.push(0);
        }
        let qwc = (w.len() as u32 - 2) / 4;
        let mut out = Vec::with_capacity(8 + w.len() * 4 + tail);
        out.extend_from_slice(&(0x6000_0000 | qwc).to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
        for x in &w {
            out.extend_from_slice(&x.to_le_bytes());
        }
        out.extend_from_slice(&0x7000_0000u32.to_le_bytes());
        out.resize(out.len() + tail.max(16) - 4, 0);
        out
    };
    // the batches of a list of {primitive, gs}: a primitive longer than a batch goes on in the next one (two vertices
    // again, from an even place)
    let batches_of = |mesh: usize, list: &[&Json], strips: &mut Vec<Vec<[u32; 12]>>| -> Result<Vec<(Vec<u32>, usize, usize, usize)>, String> {
        let mut out = Vec::new();
        for bt in list {
            let p = int(bt.get("primitive"), "primitive")? as usize;
            let mut hdr = Vec::with_capacity(20);
            for k in 0..20 {
                hdr.push(int(bt.get("gs").idx(k), "gs word")? as u32);
            }
            let vs = strip(mesh, p)?;
            if vs.len() < 3 {
                return Err(format!("mesh {mesh} primitive {p}: fewer than 3 vertices"));
            }
            let si = strips.len();
            let n = vs.len();
            strips.push(vs);
            let mut k = 0;
            loop {
                let end = (k + MAX_VERTS).min(n);
                out.push((hdr.clone(), si, k, end));
                if end == n {
                    break;
                }
                k += MAX_VERTS - 2;
            }
        }
        Ok(out)
    };
    let put = |buf: &mut [u8], at: usize, data: &[u8]| -> Result<(), String> {
        buf.get_mut(at..at + data.len()).ok_or("past the file")?.copy_from_slice(data);
        Ok(())
    };
    let pm = j.get("pmdl");
    let mut out: Vec<u8> = Vec::new();
    for w in list(pm.get("header")) {
        out.extend_from_slice(&(int(w, "header word")? as u32).to_le_bytes());
    }
    for node in list(pm.get("nodes")) {
        let vecs = list(node.get("vectors"));
        let mut rec: Vec<u8> = Vec::new();
        for v in &vecs {
            for k in 0..4 {
                rec.extend_from_slice(&(v.idx(k).num().ok_or("vector value")? as f32).to_bits().to_le_bytes());
            }
        }
        let last = int(node.get("last"), "last")? as u16;
        let (body, nv) = match node.get("batches") {
            Json::Arr(_) => {
                let mut strips = Vec::new();
                let bs = batches_of(0, &list(node.get("batches")), &mut strips)?;
                let nv: usize = bs.iter().map(|b| b.3 - b.2).sum();
                let refs: Vec<(Vec<u32>, &[[u32; 12]])> = bs.iter().map(|b| (b.0.clone(), &strips[b.1][b.2..b.3])).collect();
                (chain(&refs, int(node.get("tail"), "tail")? as usize), nv)
            }
            _ => {
                let mut b = Vec::new();
                for w in list(node.get("extra")) {
                    b.extend_from_slice(&(int(w, "extra word")? as u32).to_le_bytes());
                }
                (b, 0)
            }
        };
        if nv > 0xFFFF {
            return Err("a node has more than 65535 vertices".into());
        }
        let size = 16 + rec.len() + body.len();
        let next = if last != 0 { int(node.get("next"), "next")? as u32 } else { size as u32 };
        out.extend_from_slice(&next.to_le_bytes());
        out.extend_from_slice(&(int(node.get("pop"), "pop")? as u16).to_le_bytes());
        out.extend_from_slice(&last.to_le_bytes());
        out.extend_from_slice(&(nv as u16).to_le_bytes());
        out.extend_from_slice(&(int(node.get("id"), "id")? as u16).to_le_bytes());
        out.extend_from_slice(&(int(node.get("word3"), "word3")? as u32).to_le_bytes());
        out.extend_from_slice(&rec);
        out.extend_from_slice(&body);
    }
    // (at, room in the file, bytes): the model, then the parts
    let mut pieces = vec![(int(pm.get("at"), "at")? as usize, int(pm.get("size"), "size")? as usize, out)];
    for part in list(j.get("parts")) {
        let mesh = int(part.get("mesh"), "mesh")? as usize;
        let mut strips = Vec::new();
        let bs = batches_of(mesh, &list(part.get("batches")), &mut strips)?;
        let refs: Vec<(Vec<u32>, &[[u32; 12]])> = bs.iter().map(|b| (b.0.clone(), &strips[b.1][b.2..b.3])).collect();
        let body = chain(&refs, int(part.get("tail"), "tail")? as usize);
        pieces.push((int(part.get("at"), "at")? as usize, int(part.get("size"), "size")? as usize, body));
    }
    // from the last to the first, so that the earlier offsets stay right
    pieces.sort_by_key(|p| std::cmp::Reverse(p.0));
    let mut moved = false;
    for (at, room, mut data) in pieces {
        if data.len() == room {
            put(buf, at, &data)?;
            continue;
        }
        // another size: padded to 64 bytes like every entry, the entries after it moved
        data.resize((data.len() + 63) & !63, 0);
        if at + room > buf.len() || buf.len() < 8 {
            return Err("the model is past the file".into());
        }
        let delta = data.len() as i64 - room as i64;
        buf.splice(at..at + room, data);
        let n = u32::from_le_bytes(buf[0..4].try_into().unwrap()) as usize;
        for k in 0..=n {
            let o = 4 + k * 4;
            let off = u32::from_le_bytes(buf.get(o..o + 4).ok_or("the directory is past the file")?.try_into().unwrap()) as i64;
            if off >= (at + room) as i64 {
                buf[o..o + 4].copy_from_slice(&((off + delta) as u32).to_le_bytes());
            }
        }
        moved = true;
    }
    if moved && buf.len() > FILE_ROOM {
        return Err(format!("the file would be {} bytes; the game loads a fighter file into {FILE_ROOM}", buf.len()));
    }
    Ok(())
}

/// Triangles (index triples) to one triangle strip with the same triangles and windings. A strip is grown
/// greedily over shared edges from a triangle with the fewest free neighbours (the end of a strip), trying its three
/// corners and both places (even, odd: an odd place draws the triangle the other way round); strips are joined by
/// repeated vertices (degenerate triangles draw nothing) so that each starts at the place it needs.
fn stripify(idx: &[u32]) -> Vec<usize> {
    // (triangles with a repeated corner draw nothing and are left out)
    let tris: Vec<[usize; 3]> = idx
        .chunks_exact(3)
        .map(|t| [t[0] as usize, t[1] as usize, t[2] as usize])
        .filter(|t| t[0] != t[1] && t[1] != t[2] && t[0] != t[2])
        .collect();
    let mut by_edge: std::collections::HashMap<(usize, usize), Vec<usize>> = std::collections::HashMap::new();
    for (i, t) in tris.iter().enumerate() {
        for k in 0..3 {
            by_edge.entry((t[k], t[(k + 1) % 3])).or_default().push(i);
        }
    }
    let same_winding = |t: [usize; 3], u: [usize; 3]| (0..3).any(|r| [t[r], t[(r + 1) % 3], t[(r + 2) % 3]] == u);
    let mut used = vec![false; tris.len()];
    // grows a strip from triangle `start`, corner r first, at place parity p
    let grow = |used: &Vec<bool>, start: usize, r: usize, p: usize| -> (Vec<usize>, Vec<usize>) {
        let t = tris[start];
        let (a, b, c) = (t[r], t[(r + 1) % 3], t[(r + 2) % 3]);
        let mut s = if p == 0 { vec![a, b, c] } else { vec![b, a, c] };
        let mut taken = vec![start];
        loop {
            let n = s.len();
            let (u, v) = (s[n - 2], s[n - 1]);
            // the next triangle is drawn (u, v, w) at an even place, (v, u, w) at an odd one
            let even = (p + n - 2) % 2 == 0;
            let edge = if even { (u, v) } else { (v, u) };
            let next = by_edge.get(&edge).and_then(|l| {
                l.iter().copied().find_map(|i| {
                    if used[i] || taken.contains(&i) {
                        return None;
                    }
                    let w = tris[i].iter().copied().find(|&x| x != u && x != v)?;
                    let tri = if even { [u, v, w] } else { [v, u, w] };
                    same_winding(tris[i], tri).then_some((i, w))
                })
            });
            match next {
                Some((i, w)) => {
                    s.push(w);
                    taken.push(i);
                }
                None => return (s, taken),
            }
        }
    };
    let free = |used: &Vec<bool>, i: usize| -> usize {
        let t = tris[i];
        (0..3).map(|k| by_edge.get(&(t[(k + 1) % 3], t[k])).map_or(0, |l| l.iter().filter(|&&j| !used[j]).count())).sum()
    };
    let mut out: Vec<usize> = Vec::new();
    loop {
        let Some(start) = (0..tris.len()).filter(|&i| !used[i]).min_by_key(|&i| free(&used, i)) else { break };
        let mut best: Option<(Vec<usize>, Vec<usize>, usize)> = None;
        for r in 0..3 {
            for p in 0..2 {
                let (s, taken) = grow(&used, start, r, p);
                let better = match &best {
                    None => true,
                    Some((_, bt, bp)) => taken.len() > bt.len() || (taken.len() == bt.len() && *bp != out.len() % 2 && p == out.len() % 2),
                };
                if better {
                    best = Some((s, taken, p));
                }
            }
        }
        let (s, taken, p) = best.unwrap();
        for i in taken {
            used[i] = true;
        }
        if let Some(&last) = out.last() {
            out.push(last);
            out.push(s[0]);
        }
        if out.len() % 2 != p {
            out.push(s[0]);
        }
        out.extend(s);
    }
    out
}

/// The motions inside a fighter file (entries 0x1C..0x23, tools/assets/fmotionconv.py): each entry built from its
/// structure in model-motions.json (motion_source_entry), its keys from model-motions.gltf and its events, then put
/// where the file's directory has it.
fn model_motions_apply(buf: &mut [u8], dir: &std::path::Path) -> Result<(), String> {
    let text = super::vfs::read_to_string(dir.join("model-motions.json")).map_err(|e| e.to_string())?;
    let doc = Json::parse(&text)?;
    let gl = Gltf::open(dir, "model-motions.gltf")?;
    let n = buf.get(0..4).map(|b| u32::from_le_bytes(b.try_into().unwrap()) as usize).ok_or("no directory")?;
    let dir_words: Vec<usize> = (0..=n)
        .map(|k| buf.get(4 + k * 4..8 + k * 4).map(|b| u32::from_le_bytes(b.try_into().unwrap()) as usize))
        .collect::<Option<_>>()
        .ok_or("directory past the file")?;
    let off = |k: usize| -> Result<usize, String> { dir_words.get(k).copied().ok_or_else(|| "entry past the directory".to_string()) };
    let mut i = 0;
    while let row @ Json::Obj(_) = doc.get("motions").idx(i) {
        i += 1;
        let k = motion_integer(row.get("entry"), 4096)?;
        let (a, b) = (off(k)?, off(k + 1)?);
        let mut m = motion_source_entry(row)?;
        motion_apply(&mut m, k, &gl)?;
        events_apply(&mut m, k, doc.get("events"))?;
        if a + m.len() != b || b > buf.len() {
            return Err(format!("motion {k}: {} bytes, the file has room for {}", m.len(), b.saturating_sub(a)));
        }
        buf[a..b].copy_from_slice(&m);
    }
    Ok(())
}

fn model_apply(buf: &mut [u8], dir: &std::path::Path, list: &str) -> Result<(), String> {
    let gl = Gltf::open(dir, "model.gltf")?;
    let prims = gl.g.get("meshes").idx(0).get("primitives");
    let accessor = |i: usize, comps: usize| gl.accessor(i, comps);
    // the layout of a vertex: fighter (tools/assets/modelconv.py) or stage (stageconv.py: x y z 1 | r g b a | s t q 0,
    // the words from POSITION, _COLOR_GAME and TEXCOORD_0)
    let stage = list.lines().any(|l| l.trim() == "layout stage");
    for line in list.lines() {
        let f: Vec<&str> = line.split_whitespace().collect();
        if f.is_empty() || line.starts_with('#') || f[0] == "pmdl" || f[0] == "layout" {
            continue;
        }
        if stage {
            let n = |i: usize| -> Result<usize, String> { f.get(i).and_then(|x| x.parse().ok()).ok_or_else(|| format!("bad line: {line}")) };
            let (p, off, count) = (n(0)?, n(1)?, n(2)?);
            let at = prims.idx(p).get("attributes");
            let attr = |name: &str| -> Result<usize, String> { at.get(name).num().map(|x| x as usize).ok_or_else(|| format!("primitive {p}: no {name}")) };
            let pos = accessor(attr("POSITION")?, 3)?;
            let col = accessor(attr("_COLOR_GAME")?, 4)?;
            let uv = accessor(attr("TEXCOORD_0")?, 2)?;
            if pos.len() != count * 3 || col.len() != count * 4 || uv.len() != count * 2 {
                return Err(format!("primitive {p}: {} vertices, the file has {count}", pos.len() / 3));
            }
            if off + count * 48 > buf.len() {
                return Err(format!("primitive {p} past the file"));
            }
            for i in 0..count {
                let words = [pos[i * 3], pos[i * 3 + 1], pos[i * 3 + 2], col[i * 4], col[i * 4 + 1], col[i * 4 + 2], col[i * 4 + 3], uv[i * 2], uv[i * 2 + 1]];
                for (k, w) in [0usize, 1, 2, 4, 5, 6, 7, 8, 9].into_iter().zip(words) {
                    let o = off + i * 48 + k * 4;
                    buf[o..o + 4].copy_from_slice(&w.to_le_bytes());
                }
            }
            // the w and q words as the game has them (stageconv.py), when the extraction has them
            for (k, name) in [(3usize, "_W_GAME"), (10, "_Q_GAME")] {
                if let Ok(a) = attr(name) {
                    let v = accessor(a, 1)?;
                    if v.len() != count {
                        return Err(format!("primitive {p}: {name} has {} values, the file has {count} vertices", v.len()));
                    }
                    for (i, w) in v.into_iter().enumerate() {
                        let o = off + i * 48 + k * 4;
                        buf[o..o + 4].copy_from_slice(&w.to_le_bytes());
                    }
                }
            }
            continue;
        }
        let n = |i: usize| -> Result<usize, String> { f.get(i).and_then(|x| x.parse().ok()).ok_or_else(|| format!("bad line: {line}")) };
        // `primitive offset count` (mesh 0, the skinned model) or `mesh primitive offset count` (a loose part, whose
        // weight words are in _WEIGHT_GAME)
        let (m, p, off, count) = if f.len() == 4 { (n(0)?, n(1)?, n(2)?, n(3)?) } else { (0, n(0)?, n(1)?, n(2)?) };
        let at = gl.g.get("meshes").idx(m).get("primitives").idx(p).get("attributes");
        let attr = |name: &str| -> Result<usize, String> { at.get(name).num().map(|x| x as usize).ok_or_else(|| format!("mesh {m} primitive {p}: no {name}")) };
        let pos = accessor(attr("POSITION")?, 3)?;
        let nrm = accessor(attr("_NORMAL_GAME")?, 3)?;
        let uv = accessor(attr("TEXCOORD_0")?, 2)?;
        let wt = match attr("WEIGHTS_0") {
            Ok(a) => accessor(a, 4)?.chunks(4).map(|c| c[0]).collect::<Vec<u32>>(),
            Err(_) => accessor(attr("_WEIGHT_GAME")?, 1)?,
        };
        if pos.len() != count * 3 || nrm.len() != count * 3 || uv.len() != count * 2 || wt.len() != count {
            return Err(format!("mesh {m} primitive {p}: {} vertices, the file has {count}", pos.len() / 3));
        }
        if off + count * 48 > buf.len() {
            return Err(format!("primitive {p} past the file"));
        }
        for i in 0..count {
            let words = [pos[i * 3], pos[i * 3 + 1], pos[i * 3 + 2], wt[i], nrm[i * 3], nrm[i * 3 + 1], nrm[i * 3 + 2], uv[i * 2], uv[i * 2 + 1]];
            for (k, w) in [0usize, 1, 2, 3, 4, 5, 6, 8, 9].into_iter().zip(words) {
                let o = off + i * 48 + k * 4;
                buf[o..o + 4].copy_from_slice(&w.to_le_bytes());
            }
        }
        // q of the texture coordinates (word 10), when the extraction has it (modelconv.py _Q_GAME)
        if let Ok(a) = attr("_Q_GAME") {
            let q = accessor(a, 1)?;
            if q.len() != count {
                return Err(format!("mesh {m} primitive {p}: _Q_GAME has {} values, the file has {count} vertices", q.len()));
            }
            for (i, w) in q.into_iter().enumerate() {
                let o = off + i * 48 + 40;
                buf[o..o + 4].copy_from_slice(&w.to_le_bytes());
            }
        }
    }
    Ok(())
}

/// Just enough JSON for glTF.
#[allow(dead_code)]
enum Json {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Arr(Vec<Json>),
    Obj(Vec<(String, Json)>),
}

static NULL: Json = Json::Null;

impl Json {
    fn get(&self, key: &str) -> &Json {
        match self {
            Json::Obj(v) => v.iter().find(|(k, _)| k == key).map(|(_, x)| x).unwrap_or(&NULL),
            _ => &NULL,
        }
    }
    fn idx(&self, i: usize) -> &Json {
        match self {
            Json::Arr(v) => v.get(i).unwrap_or(&NULL),
            _ => &NULL,
        }
    }
    fn num(&self) -> Option<f64> {
        match self {
            Json::Num(x) => Some(*x),
            _ => None,
        }
    }
    fn str(&self) -> Option<&str> {
        match self {
            Json::Str(s) => Some(s),
            _ => None,
        }
    }
    fn parse(text: &str) -> Result<Json, String> {
        let b = text.as_bytes();
        let mut p = 0;
        let v = Self::value(b, &mut p)?;
        Self::ws(b, &mut p);
        if p != b.len() {
            return Err(format!("JSON: text after the value at {p}"));
        }
        Ok(v)
    }
    fn ws(b: &[u8], p: &mut usize) {
        while *p < b.len() && b[*p].is_ascii_whitespace() {
            *p += 1;
        }
    }
    fn value(b: &[u8], p: &mut usize) -> Result<Json, String> {
        Self::ws(b, p);
        let err = |p: usize| format!("JSON: unexpected text at {p}");
        match b.get(*p).copied() {
            Some(b'{') => {
                *p += 1;
                let mut v = Vec::new();
                loop {
                    Self::ws(b, p);
                    if b.get(*p) == Some(&b'}') {
                        *p += 1;
                        return Ok(Json::Obj(v));
                    }
                    if !v.is_empty() {
                        if b.get(*p) != Some(&b',') {
                            return Err(err(*p));
                        }
                        *p += 1;
                        Self::ws(b, p);
                    }
                    let Json::Str(k) = Self::value(b, p)? else { return Err(err(*p)) };
                    Self::ws(b, p);
                    if b.get(*p) != Some(&b':') {
                        return Err(err(*p));
                    }
                    *p += 1;
                    v.push((k, Self::value(b, p)?));
                }
            }
            Some(b'[') => {
                *p += 1;
                let mut v = Vec::new();
                loop {
                    Self::ws(b, p);
                    if b.get(*p) == Some(&b']') {
                        *p += 1;
                        return Ok(Json::Arr(v));
                    }
                    if !v.is_empty() {
                        if b.get(*p) != Some(&b',') {
                            return Err(err(*p));
                        }
                        *p += 1;
                    }
                    v.push(Self::value(b, p)?);
                }
            }
            Some(b'"') => {
                *p += 1;
                let mut s = Vec::new();
                loop {
                    match b.get(*p).copied() {
                        None => return Err(err(*p)),
                        Some(b'"') => {
                            *p += 1;
                            break;
                        }
                        Some(b'\\') => {
                            let c = b.get(*p + 1).copied().ok_or_else(|| err(*p))?;
                            *p += 2;
                            match c {
                                b'n' => s.push(b'\n'),
                                b't' => s.push(b'\t'),
                                b'r' => s.push(b'\r'),
                                b'b' => s.push(8),
                                b'f' => s.push(12),
                                b'u' => {
                                    let hex4 = |p: usize| -> Result<u32, String> {
                                        let h = std::str::from_utf8(b.get(p..p + 4).ok_or_else(|| err(p))?).map_err(|_| err(p))?;
                                        u32::from_str_radix(h, 16).map_err(|_| err(p))
                                    };
                                    let mut c = hex4(*p)?;
                                    *p += 4;
                                    // a surrogate pair: \ud83d\ude00
                                    if (0xD800..0xDC00).contains(&c) && b.get(*p) == Some(&b'\\') && b.get(*p + 1) == Some(&b'u') {
                                        let lo = hex4(*p + 2)?;
                                        if (0xDC00..0xE000).contains(&lo) {
                                            c = 0x10000 + ((c - 0xD800) << 10) + (lo - 0xDC00);
                                            *p += 6;
                                        }
                                    }
                                    let ch = char::from_u32(c).unwrap_or('?');
                                    let mut t = [0; 4];
                                    s.extend_from_slice(ch.encode_utf8(&mut t).as_bytes());
                                }
                                c => s.push(c),
                            }
                        }
                        Some(c) => {
                            s.push(c);
                            *p += 1;
                        }
                    }
                }
                Ok(Json::Str(String::from_utf8_lossy(&s).into_owned()))
            }
            Some(b't') if b[*p..].starts_with(b"true") => {
                *p += 4;
                Ok(Json::Bool(true))
            }
            Some(b'f') if b[*p..].starts_with(b"false") => {
                *p += 5;
                Ok(Json::Bool(false))
            }
            Some(b'n') if b[*p..].starts_with(b"null") => {
                *p += 4;
                Ok(Json::Null)
            }
            Some(c) if c == b'-' || c.is_ascii_digit() => {
                let s = *p;
                while *p < b.len() && matches!(b[*p], b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9') {
                    *p += 1;
                }
                let t = std::str::from_utf8(&b[s..*p]).unwrap();
                t.parse().map(Json::Num).map_err(|_| err(s))
            }
            _ => Err(err(*p)),
        }
    }
}

// ---------------------------------------------------------------------------------------------- banks
/// An animation bank (tools/assets/motion.py): `u32 count`, the entries' offsets, the end offset, zeros up to 64
/// bytes, then the entries, each a file packed with the game's BPE and padded to 64 bytes (bank.txt keeps the
/// padding the original had). A motion entry gets the keys of animation `motion_<entry>` of animations.gltf.
fn bank_assemble(dir: &std::path::Path, list: &str) -> Result<Vec<u8>, String> {
    bank_assemble_sources(dir,list,MotionSources::open(dir)?)
}

fn bank_assemble_sources(dir:&std::path::Path,list:&str,sources:Option<MotionSources>)->Result<Vec<u8>,String> {
    let native=super::resource_transport::enabled();
    let mut entries: Vec<(usize, bool, Vec<u8>)> = Vec::new();
    if native && sources.as_ref().is_some_and(|s|s.complete()) {
        let source=sources.as_ref().unwrap();
        for k in 1..=source.count {entries.push((k,source.rows.contains_key(&k),Vec::new()));}
    }else {
    for line in list.lines() {
        let f: Vec<&str> = line.split_whitespace().collect();
        if f.is_empty() || line.starts_with('#') || f[0] == "entries" {
            continue;
        }
        let k: usize = f[0].parse().map_err(|_| format!("bank.txt: {line}"))?;
        let tail = if !native && f.len() >= 4 && f[2] == "tail" { unhex(f[3]).ok_or_else(|| format!("bank.txt: {line}"))? } else { Vec::new() };
        entries.push((k, f.get(1) == Some(&"motion"), tail));
    }
    }
    let gl = if entries.iter().any(|e| e.1) && dir.join("animations.gltf").vis_file() { Some(Gltf::open(dir, "animations.gltf")?) } else { None };
    let events = match super::vfs::read_to_string(dir.join("events.json")) {
        Ok(t) => Some(Json::parse(&t).map_err(|e| format!("events.json: {e}"))?),
        Err(_) => None,
    };
    let n = entries.len();
    let head = ((n + 2) * 4 + 63) / 64 * 64;
    let mut out = vec![0u8; head];
    out[0..4].copy_from_slice(&(n as u32).to_le_bytes());
    for (i, (k, motion, tail)) in entries.iter().enumerate() {
        if *k != i + 1 {
            return Err(format!("bank.txt: entry {k} out of order"));
        }
        // the unpacked entry: entry_<n>.bin, or a folder entry_<n>/ extracted like a file
        let sub = dir.join(format!("entry_{k:03}"));
        let prepared=if *motion {sources.as_ref().and_then(|s|s.rows.get(k))}else{None};
        let mut m = if let Some(row)=prepared {
            motion_source_entry(row).map_err(|e|format!("motion {k}: {e}"))?
        } else if sources.as_ref().is_some_and(|s|s.empty.contains(k)) {
            Vec::new()
        } else if is_extracted(&sub) {
            assemble(&sub).map_err(|e| format!("entry_{k:03}: {e}"))?
        } else {
            let name = format!("entry_{k:03}.bin");
            super::vfs::read(dir.join(&name)).map_err(|e| format!("{name}: {e}"))?
        };
        if *motion {
            if let Some(gl) = &gl {
                motion_apply(&mut m, *k, gl).map_err(|e| format!("animations.gltf: {e}"))?;
            }
            if let Some(ev) = &events {
                events_apply(&mut m, *k, ev).map_err(|e| format!("events.json: {e}"))?;
            }
        }
        let packed = if native {super::resource_transport::retain(m)?}else{bpe_pack_raw(&m, 10000, 16384)?};
        let at = out.len() as u32;
        out[4 + i * 4..8 + i * 4].copy_from_slice(&at.to_le_bytes());
        out.extend_from_slice(&packed);
        if (packed.len() + tail.len()) % 64 == 0 {
            out.extend_from_slice(tail);
        } else {
            out.resize((out.len() + 63) / 64 * 64, 0);
        }
    }
    let end = out.len() as u32;
    out[4 + n * 4..8 + n * 4].copy_from_slice(&end.to_le_bytes());
    Ok(out)
}

struct MotionSources {
    rows:HashMap<usize,Json>,
    empty:std::collections::HashSet<usize>,
    count:usize,
}

impl MotionSources {
    fn open(dir:&std::path::Path)->Result<Option<Self>,String> {
        let text=match super::vfs::read_to_string(dir.join("motion-sources.json")) {
            Ok(v)=>v,
            Err(e) if e.kind()==std::io::ErrorKind::NotFound=>return Ok(None),
            Err(e)=>return Err(e.to_string()),
        };
        let doc=Json::parse(&text)?;
        if doc.get("version").num()!=Some(1.0) {return Err("motion sources: unsupported version".into());}
        let count=match doc.get("entryCount") {Json::Null=>0,v=>motion_integer(v,4096)?};
        let mut empty=std::collections::HashSet::new();
        if !matches!(doc.get("emptyEntries"),Json::Null) {
            for value in motion_array(doc.get("emptyEntries"))? {
                let id=motion_integer(value,4096)?;
                if id==0 || !empty.insert(id) {return Err("motion sources: invalid or duplicate empty entry".into());}
            }
        }
        let Json::Obj(fields)=doc else {return Err("motion sources: expected object".into());};
        let value=fields.into_iter().find(|(name,_)|name=="motions").ok_or("motion sources: motions missing")?.1;
        let Json::Arr(values)=value else {return Err("motion sources: motions must be an array".into());};
        let mut rows=HashMap::new();
        for row in values {
            let id=motion_integer(row.get("entry"),4096)?;
            if id==0 || empty.contains(&id) || rows.insert(id,row).is_some() {return Err("motion sources: invalid or duplicate entry".into());}
        }
        if count!=0 && (rows.keys().any(|&id|id>count) || empty.iter().any(|&id|id>count)) {
            return Err("motion sources: entry outside bank count".into());
        }
        Ok(Some(Self {rows,empty,count}))
    }

    fn complete(&self)->bool {
        self.count!=0 && self.rows.len()+self.empty.len()==self.count
    }
}

fn motion_integer(value:&Json,max:usize)->Result<usize,String> {
    let n=value.num().ok_or("motion source: numeric field missing")?;
    if !n.is_finite() || n<0.0 || n.fract()!=0.0 || n>max as f64 {return Err("motion source: integer out of range".into());}
    Ok(n as usize)
}

fn motion_array(value:&Json)->Result<&Vec<Json>,String> {
    if let Json::Arr(values)=value {Ok(values)}else{Err("motion source: array missing".into())}
}

fn motion_vector<const N:usize>(value:&Json)->Result<[f32;N],String> {
    let values=motion_array(value)?;
    if values.len()!=N {return Err("motion source: vector component count".into());}
    let mut result=[0.0;N];
    for i in 0..N {
        let f=values[i].num().ok_or("motion source: float missing")? as f32;
        if !f.is_finite() {return Err("motion source: non-finite float".into());}
        result[i]=f;
    }
    Ok(result)
}

/// Typed structure + fallback keys only. Exported pose keys and attack events
/// are still supplied by glTF/events.json using the unchanged serializers.
fn motion_source_entry(row:&Json)->Result<Vec<u8>,String> {
    let size=motion_integer(row.get("bytes"),16*1024*1024)?;
    let volumes=motion_array(row.get("eventVolumeWords"))?;
    if volumes.len()>255 || size<148+volumes.len()*16 {return Err("motion source: invalid header size".into());}
    let mut out=vec![0;size];
    out[0]=motion_integer(row.get("headerByte"),255)? as u8;
    out[1]=volumes.len() as u8;
    out[2..4].copy_from_slice(&(motion_integer(row.get("templateLength"),65535)? as u16).to_le_bytes());
    out[4..6].copy_from_slice(&(motion_integer(row.get("headerWord"),65535)? as u16).to_le_bytes());
    for (i,volume) in volumes.iter().enumerate() {
        out[148+i*16+10..148+i*16+12].copy_from_slice(&(motion_integer(volume,65535)? as u16).to_le_bytes());
    }
    let mut tracks:HashMap<usize,(usize,usize,usize)>=HashMap::new();
    for record in motion_array(row.get("tracks"))? {
        let r=motion_array(record)?;
        if r.len()!=4 {return Err("motion source: track needs node, offset, flags, count".into());}
        let node=motion_integer(&r[0],70)?;
        let at=motion_integer(&r[1],65535*4)?;
        let flags=motion_integer(&r[2],1)?;
        let count=motion_integer(&r[3],65535)?;
        let end=at+4+count*if flags&1!=0 {10}else{24};
        if at<148 || at%4!=0 || count==0 || end>size || tracks.insert(node,(at,flags,count)).is_some() {
            return Err("motion source: invalid or duplicate track".into());
        }
        out[6+node*2..8+node*2].copy_from_slice(&((at/4) as u16).to_le_bytes());
        out[at..at+2].copy_from_slice(&(flags as u16).to_le_bytes());
        out[at+2..at+4].copy_from_slice(&(count as u16).to_le_bytes());
    }
    let mut used=std::collections::HashSet::new();
    for record in motion_array(row.get("fallback"))? {
        let node=motion_integer(record.get("node"),70)?;
        let &(at,flags,count)=tracks.get(&node).ok_or("motion source: fallback refers to missing track")?;
        if !used.insert(node) {return Err("motion source: duplicate fallback track".into());}
        let keys=motion_array(record.get("keys"))?;
        if keys.len()!=count {return Err("motion source: fallback key count mismatch".into());}
        motion_source_keys(&mut out,at,flags,keys)?;
    }
    let mut occupied:Vec<(usize,usize)>=tracks.values().map(|&(at,flags,count)|(at,at+4+count*if flags&1!=0 {10}else{24})).collect();
    if !matches!(row.get("unboundTracks"),Json::Null) {
        for record in motion_array(row.get("unboundTracks"))? {
            let at=motion_integer(record.get("byteOffset"),65535*4)?;
            let flags=motion_integer(record.get("flags"),1)?;
            let keys=motion_array(record.get("keys"))?;
            let end=at+4+keys.len()*if flags&1!=0 {10}else{24};
            if at<148+volumes.len()*16 || at%4!=0 || keys.is_empty() || keys.len()>65535 || end>size || occupied.iter().any(|&(a,b)|a<end && at<b) {
                return Err("motion source: invalid unbound track".into());
            }
            occupied.push((at,end));
            out[at..at+2].copy_from_slice(&(flags as u16).to_le_bytes());
            out[at+2..at+4].copy_from_slice(&(keys.len() as u16).to_le_bytes());
            motion_source_keys(&mut out,at,flags,keys)?;
        }
    }
    Ok(out)
}

fn motion_source_keys(out:&mut [u8],at:usize,flags:usize,keys:&[Json])->Result<(),String> {
    let count=keys.len();
    for (i,key) in keys.iter().enumerate() {
            let rotation=quat_pack(motion_vector::<4>(key.get("rotation"))?);
            let frame=motion_integer(key.get("frame"),if flags&1!=0 {65535}else{u32::MAX as usize})? as u32;
            if flags&1!=0 {
                out[at+4+i*8..at+12+i*8].copy_from_slice(&rotation.to_le_bytes());
                out[at+4+count*8+i*2..at+6+count*8+i*2].copy_from_slice(&(frame as u16).to_le_bytes());
            }else {
                let pos=motion_vector::<3>(key.get("position"))?;
                let p=at+4+i*24;
                for axis in 0..3 {out[p+axis*4..p+axis*4+4].copy_from_slice(&pos[axis].to_le_bytes());}
                out[p+12..p+16].copy_from_slice(&frame.to_le_bytes());
                out[p+16..p+24].copy_from_slice(&rotation.to_le_bytes());
            }
    }
    Ok(())
}

/// Offline reference check. Game loading above never reads entry_NNN.bin for
/// entries whose typed motion source exists.
pub fn verify_motion_sources(root:&std::path::Path)->Result<(usize,usize,usize),String> {
    let mut banks=0;let mut motions=0;let mut empty=0;
    for entry in std::fs::read_dir(root).map_err(|e|e.to_string())? {
        let dir=entry.map_err(|e|e.to_string())?.path();
        if !dir.vis_dir() {continue;}
        let Some(source)=MotionSources::open(&dir)? else {continue;};
        for (id,row) in &source.rows {
            // the old extractions kept the entry's structure in entry_NNN.bin too; the new ones only here
            let Ok(original)=super::vfs::read(dir.join(format!("entry_{id:03}.bin"))) else {motions+=1;continue;};
            let native=motion_source_entry(row)?;
            if native!=original {return Err(format!("{} motion {id}: native structure differs",dir.display()));}
            motions+=1;
        }
        for id in &source.empty {
            if std::fs::metadata(dir.join(format!("entry_{id:03}.bin"))).map_err(|e|e.to_string())?.len()!=0 {
                return Err(format!("{} entry {id}: expected empty",dir.display()));
            }
            empty+=1;
        }
        banks+=1;
    }
    Ok((banks,motions,empty))
}

/// The events of motion <k> from events.json (tools/assets/motion.py): the length (u16 at 2) and per event (16 bytes
/// from 148: u64 attr, u16 frame, u16 volume offset in words, u32 arg) attr, frame, arg and the volume (7 floats at
/// `at`). The number of events is the one the motion has (u8 at 1).
fn events_apply(m: &mut [u8], k: usize, doc: &Json) -> Result<(), String> {
    let mut i = 0;
    let mo = loop {
        match doc.get("motions").idx(i) {
            Json::Null => return Ok(()),
            x if x.get("entry").num() == Some(k as f64) => break x,
            _ => i += 1,
        }
    };
    let put = |m: &mut [u8], o: usize, b: &[u8]| -> Result<(), String> {
        m.get_mut(o..o + b.len()).ok_or_else(|| format!("motion {k}: past the entry"))?.copy_from_slice(b);
        Ok(())
    };
    let count = *m.get(1).ok_or("empty motion")? as usize;
    let length = mo.get("length").num().ok_or("no length")? as u16;
    put(m, 2, &length.to_le_bytes())?;
    let evs = mo.get("events");
    let mut n = 0;
    while let e @ Json::Obj(_) = evs.idx(n) {
        if n >= count {
            return Err(format!("motion {k} has {count} events"));
        }
        let at = 148 + n * 16;
        let attr = u64::from_str_radix(e.get("attr").str().ok_or("event without attr")?, 16).map_err(|_| "bad attr")?;
        let frame = e.get("frame").num().ok_or("event without frame")? as u16;
        let arg = e.get("arg").num().ok_or("event without arg")? as u32;
        put(m, at, &attr.to_le_bytes())?;
        put(m, at + 8, &frame.to_le_bytes())?;
        put(m, at + 12, &arg.to_le_bytes())?;
        let v = e.get("volume");
        if let Json::Obj(_) = v {
            let vat = v.get("at").num().ok_or("volume without at")? as usize;
            let mut f = Vec::with_capacity(7);
            for c in 0..3 {
                f.push(v.get("position").idx(c).num().ok_or("volume position")?);
            }
            f.push(v.get("w").num().ok_or("volume w")?);
            for c in 0..3 {
                f.push(v.get("radii").idx(c).num().ok_or("volume radii")?);
            }
            for (c, x) in f.iter().enumerate() {
                put(m, vat + c * 4, &(*x as f32).to_le_bytes())?;
            }
        }
        n += 1;
    }
    if n != count {
        return Err(format!("motion {k} has {count} events"));
    }
    Ok(())
}

/// A packed rotation: the three smaller components of the quaternion in 20 bits each, `(v / sqrt 2 + 1/2)` scaled
/// to 0..1048575, the index of the largest one in the top bits (the game rebuilds it as `sqrt(1 - others²)`).
fn quat_small(bits: u64) -> f32 {
    ((bits as f32) * ((1.0f64 / 1048575.0) as f32) - 0.5) * (1.41421356f64 as f32)
}

fn quat_unpack(p: u64) -> [f32; 4] {
    let largest = ((p >> 60) as usize).min(3);
    let v = [quat_small(p & 0xfffff), quat_small((p >> 20) & 0xfffff), quat_small((p >> 40) & 0xfffff)];
    let mut s = 0f32;
    for x in v {
        s += x * x;
    }
    let w = (1.0f32 - s).sqrt();
    let mut q = [0f32; 4];
    let mut k = 0;
    for (i, slot) in q.iter_mut().enumerate() {
        if i == largest {
            *slot = w;
        } else {
            *slot = v[k];
            k += 1;
        }
    }
    q
}

fn quat_pack_at(q: [f32; 4], largest: usize) -> u64 {
    let q = if q[largest] < 0.0 { q.map(|x| -x) } else { q };
    let mut out = (largest as u64) << 60;
    let mut k = 0;
    for (i, &v) in q.iter().enumerate() {
        if i == largest {
            continue;
        }
        let mut b = (((v as f64) / 1.41421356 + 0.5) * 1048575.0).round().clamp(0.0, 1048575.0) as i64;
        for c in [b, b - 1, b + 1, b - 2, b + 2] {
            if (0..=0xfffff).contains(&c) && quat_small(c as u64) == v {
                b = c;
                break;
            }
        }
        out |= (b as u64) << (k * 20);
        k += 1;
    }
    out
}

/// The packed rotation that unpacks to q exactly when there is one, else the nearest.
fn quat_pack(q: [f32; 4]) -> u64 {
    let mut order = [0usize, 1, 2, 3];
    order.sort_by(|&a, &b| q[b].abs().partial_cmp(&q[a].abs()).unwrap_or(std::cmp::Ordering::Equal));
    for &l in &order {
        let p = quat_pack_at(q, l);
        let u = quat_unpack(p);
        if (0..4).all(|i| u[i] == q[i]) || (0..4).all(|i| u[i] == -q[i]) {
            return p;
        }
    }
    quat_pack_at(q, order[0])
}

/// The keys of animation `motion_<k>` back into the unpacked motion m: per node of the animation, the frames (time
/// x 30), rotations (packed) and translations (for tracks that have them) of its track, whose place and key count
/// are in the motion's own header (u16 offsets in words at 6, one per node; a track is u16 flags, u16 keys, then
/// n x u64 rotation and n x u16 frame, or n x (x y z, u32 frame, u64 rotation)).
fn motion_apply(m: &mut [u8], k: usize, gl: &Gltf) -> Result<(), String> {
    let g = &gl.g;
    let name = format!("motion_{k:03}");
    let mut i = 0;
    let anim = loop {
        match g.get("animations").idx(i) {
            Json::Null => return Ok(()),
            a if a.get("name").str() == Some(name.as_str()) => break a,
            _ => i += 1,
        }
    };
    let rd16 = |m: &[u8], o: usize| -> Result<usize, String> { m.get(o..o + 2).map(|b| u16::from_le_bytes([b[0], b[1]]) as usize).ok_or_else(|| format!("{name}: past the entry")) };
    // per node: (rotation sampler, translation sampler)
    let mut per: Vec<(usize, Option<usize>, Option<usize>)> = Vec::new();
    let mut c = 0;
    while let ch @ Json::Obj(_) = anim.get("channels").idx(c) {
        c += 1;
        let target = ch.get("target");
        let node = target.get("node").num().ok_or("channel without node")? as usize;
        let nname = g.get("nodes").idx(node).get("name").str().unwrap_or("");
        let id: usize = nname.strip_prefix("node_").and_then(|x| x.parse().ok()).ok_or_else(|| format!("{name}: node {nname} is not a node of the game"))?;
        let s = ch.get("sampler").num().ok_or("channel without sampler")? as usize;
        let e = match per.iter().position(|p| p.0 == id) {
            Some(p) => p,
            None => {
                per.push((id, None, None));
                per.len() - 1
            }
        };
        match target.get("path").str() {
            Some("rotation") => per[e].1 = Some(s),
            Some("translation") => per[e].2 = Some(s),
            _ => return Err(format!("{name}: only rotations and translations")),
        }
    }
    let samp = |s: usize, field: &str, comps: usize| -> Result<Vec<u32>, String> {
        let a = anim.get("samplers").idx(s).get(field).num().ok_or("sampler without accessor")? as usize;
        gl.accessor(a, comps)
    };
    for (id, rs, ts) in per {
        if id >= 71 {
            return Err(format!("{name}: node {id}"));
        }
        let at = rd16(m, 6 + id * 2)? * 4;
        if at == 0 {
            return Err(format!("{name}: node {id} has no track in the entry"));
        }
        let rs = rs.ok_or_else(|| format!("{name}: node {id} without rotations"))?;
        let flags = rd16(m, at)?;
        let n = rd16(m, at + 2)?;
        let times = samp(rs, "input", 1)?;
        let rots = samp(rs, "output", 4)?;
        if times.len() != n || rots.len() != n * 4 {
            return Err(format!("{name}: node {id}: {} keys, the entry has {n}", times.len()));
        }
        let rot_only = flags & 1 != 0;
        let poss = match (rot_only, ts) {
            (true, _) => None,
            (false, Some(t)) => Some(samp(t, "output", 3)?),
            (false, None) => return Err(format!("{name}: node {id} needs translations")),
        };
        if let Some(p) = &poss {
            if p.len() != n * 3 {
                return Err(format!("{name}: node {id}: translations count"));
            }
        }
        let end = if rot_only { at + 4 + n * 10 } else { at + 4 + n * 24 };
        if end > m.len() {
            return Err(format!("{name}: node {id}: track past the entry"));
        }
        for j in 0..n {
            let frame = ((f32::from_bits(times[j]) as f64) * 30.0).round() as u32;
            let q = [0, 1, 2, 3].map(|c| f32::from_bits(rots[j * 4 + c]));
            let p = quat_pack(q);
            if rot_only {
                m[at + 4 + j * 8..at + 12 + j * 8].copy_from_slice(&p.to_le_bytes());
                m[at + 4 + n * 8 + j * 2..at + 6 + n * 8 + j * 2].copy_from_slice(&(frame as u16).to_le_bytes());
            } else {
                let o = at + 4 + j * 24;
                let pv = poss.as_ref().unwrap();
                for c in 0..3 {
                    m[o + c * 4..o + c * 4 + 4].copy_from_slice(&pv[j * 3 + c].to_le_bytes());
                }
                m[o + 12..o + 16].copy_from_slice(&frame.to_le_bytes());
                m[o + 16..o + 24].copy_from_slice(&p.to_le_bytes());
            }
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------- BPE
/// The game's compression: Philip Gage's byte pair encoding (C Users Journal, 1994), in blocks compressed on their
/// own, as the game's packer wrote it (the same choices of pairs, so the same bytes): `u32 size, u32 packed`, the
/// blocks, and the padding to 16 bytes the packer left (bytes of its memory, kept in bpe.txt).
/// bpe.txt: `block <bytes> hash <entries> tail <hex>`.
fn bpe_pack(data: &[u8], spec: &str) -> Result<Vec<u8>, String> {
    let (mut block, mut hash, mut tail) = (10000usize, 16384usize, Vec::new());
    for line in spec.lines() {
        if line.starts_with('#') {
            continue;
        }
        let f: Vec<&str> = line.split_whitespace().collect();
        for kv in f.chunks(2) {
            match kv {
                ["block", v] => block = v.parse().map_err(|_| "bpe.txt: block")?,
                ["hash", v] => hash = v.parse().map_err(|_| "bpe.txt: hash")?,
                ["tail", v] => tail = unhex(v).ok_or("bpe.txt: tail")?,
                ["tail"] => {}
                _ => return Err(format!("bpe.txt: {line}")),
            }
        }
    }
    let mut out = bpe_pack_raw(data, block, hash)?;
    out.extend_from_slice(&tail);
    Ok(out)
}

fn unhex(v: &str) -> Option<Vec<u8>> {
    if v.len() % 2 != 0 {
        return None;
    }
    (0..v.len() / 2).map(|i| u8::from_str_radix(v.get(2 * i..2 * i + 2)?, 16).ok()).collect()
}

/// `u32 size, u32 packed`, then the blocks.
fn bpe_pack_raw(data: &[u8], block: usize, hash: usize) -> Result<Vec<u8>, String> {
    if !hash.is_power_of_two() || block == 0 || block > 65535 {
        return Err("BPE: bad block/hash".into());
    }
    // the blocks are packed on their own: in parallel, then put in order
    let mut blocks: Vec<&[u8]> = Vec::new();
    let mut s = 0;
    loop {
        let n = (data.len() - s).min(block);
        blocks.push(&data[s..s + n]);
        s += block;
        if s > data.len() {
            break;
        }
    }
    let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1).clamp(1, 16).min(blocks.len());
    let per = blocks.len().div_ceil(threads);
    let parts: Vec<Result<Vec<u8>, String>> = std::thread::scope(|sc| {
        let hs: Vec<_> = blocks
            .chunks(per)
            .map(|group| {
                sc.spawn(move || {
                    let mut c = Bpe::new(hash);
                    let mut out = Vec::new();
                    for b in group {
                        c.block(b, &mut out)?;
                    }
                    Ok(out)
                })
            })
            .collect();
        hs.into_iter().map(|h| h.join().unwrap_or_else(|_| Err("BPE thread".into()))).collect()
    });
    let mut out = vec![0u8; 8];
    for p in parts {
        out.extend_from_slice(&p?);
    }
    let packed = out.len() - 8;
    out[0..4].copy_from_slice(&(data.len() as u32).to_le_bytes());
    out[4..8].copy_from_slice(&(packed as u32).to_le_bytes());
    Ok(out)
}

struct Bpe {
    left: Vec<u8>,
    right: Vec<u8>,
    count: Vec<u8>,
    mask: usize,
}

impl Bpe {
    fn new(hash: usize) -> Bpe {
        Bpe { left: vec![0; hash], right: vec![0; hash], count: vec![0; hash], mask: hash - 1 }
    }

    fn lookup(&mut self, a: u8, b: u8) -> Result<usize, String> {
        let mut i = (a as usize ^ ((b as usize) << 5)) & self.mask;
        let mut n = 0;
        while (self.left[i] != a || self.right[i] != b) && self.count[i] != 0 {
            i = (i + 1) & self.mask;
            n += 1;
            if n > self.mask {
                return Err("BPE: pair table full".into());
            }
        }
        self.left[i] = a;
        self.right[i] = b;
        Ok(i)
    }

    fn block(&mut self, input: &[u8], out: &mut Vec<u8>) -> Result<(), String> {
        self.count.fill(0);
        self.left.fill(0);
        self.right.fill(0);
        let mut leftcode: [u8; 256] = std::array::from_fn(|i| i as u8);
        let mut rightcode = [0u8; 256];
        let mut buf: Vec<u8> = Vec::with_capacity(input.len() + 1);
        for &c in input {
            if let Some(&last) = buf.last() {
                let i = self.lookup(last, c)?;
                if self.count[i] < 255 {
                    self.count[i] += 1;
                }
            }
            buf.push(c);
            rightcode[c as usize] = 1;
        }
        let mut size = buf.len();
        buf.push(0);
        let mut code: i32 = 255;
        loop {
            while code >= 0 && !(leftcode[code as usize] as i32 == code && rightcode[code as usize] == 0) {
                code -= 1;
            }
            if code < 0 {
                break;
            }
            // the pair with the largest count, the first in hash order on a tie (two passes the compiler vectorises)
            let best = self.count.iter().copied().max().unwrap_or(0);
            if best < 3 {
                break;
            }
            let at = self.count.iter().position(|&c| c == best).unwrap();
            let (lc, rc) = (self.left[at], self.right[at]);
            let k = code as u8;
            let oldsize = size as isize - 1;
            let (mut w, mut r) = (0usize, 0isize);
            while r < oldsize {
                let ru = r as usize;
                if buf[ru] == lc && buf[ru + 1] == rc {
                    if r > 0 {
                        let i = self.lookup(buf[w - 1], lc)?;
                        if self.count[i] > 1 {
                            self.count[i] -= 1;
                        }
                        let i = self.lookup(buf[w - 1], k)?;
                        if self.count[i] < 255 {
                            self.count[i] += 1;
                        }
                    }
                    if r < oldsize - 1 {
                        let i = self.lookup(rc, buf[ru + 2])?;
                        if self.count[i] > 1 {
                            self.count[i] -= 1;
                        }
                        let i = self.lookup(k, buf[ru + 2])?;
                        if self.count[i] < 255 {
                            self.count[i] += 1;
                        }
                    }
                    buf[w] = k;
                    w += 1;
                    r += 1;
                    size -= 1;
                } else {
                    buf[w] = buf[ru];
                    w += 1;
                }
                r += 1;
            }
            if r >= 0 {
                buf[w] = buf[r as usize];
            }
            leftcode[code as usize] = lc;
            rightcode[code as usize] = rc;
            let i = self.lookup(lc, rc)?;
            self.count[i] = 1;
            code -= 1;
        }
        // the pair table: runs of unused codes (128 + n - 1) or of used ones (n - 1, then left [right] each)
        let mut c = 0usize;
        while c < 256 {
            let len;
            if c == leftcode[c] as usize {
                let mut l = 1;
                c += 1;
                while l < 127 && c < 256 && c == leftcode[c] as usize {
                    l += 1;
                    c += 1;
                }
                out.push((l + 127) as u8);
                len = 0;
                if c == 256 {
                    break;
                }
            } else {
                let mut l = 0;
                c += 1;
                while (l < 127 && c < 256 && c != leftcode[c] as usize) || (l < 125 && c < 254 && c + 1 != leftcode[c + 1] as usize) {
                    l += 1;
                    c += 1;
                }
                out.push(l as u8);
                c -= l + 1;
                len = l;
            }
            for _ in 0..=len {
                out.push(leftcode[c]);
                if c != leftcode[c] as usize {
                    out.push(rightcode[c]);
                }
                c += 1;
            }
        }
        out.push((size >> 8) as u8);
        out.push(size as u8);
        out.extend_from_slice(&buf[..size]);
        Ok(())
    }
}

// ---------------------------------------------------------------------------------------------- records
/// An upload of image data: where its data is in the file and how it is written into GS memory.
#[derive(Clone, Copy)]
struct Upload {
    off: usize,
    w: u32,
    h: u32,
    psm: u32,
    bw: u32,
}

struct Record {
    png: String,
    tw: u32,
    th: u32,
    psm: u32,
    bw: u32,
    tbp: u32,
    pix: Upload,
    clut: Option<(u32, u32, Upload)>, // cpsm, cbp, upload
    gs_alpha: bool,
}

impl Record {
    /// `png tw th psm bw tbp | off w h psm bw | [cpsm cbp | off w h psm bw | ] png|gs`
    fn parse(line: &str) -> Option<Record> {
        let f: Vec<&str> = line.split_whitespace().collect();
        let n = |i: usize| -> Option<u32> { f.get(i)?.parse().ok() };
        let up = |i: usize| -> Option<Upload> { Some(Upload { off: n(i)? as usize, w: n(i + 1)?, h: n(i + 2)?, psm: n(i + 3)?, bw: n(i + 4)? }) };
        let pix = up(6)?;
        let (clut, alpha) = if f.len() >= 19 { (Some((n(11)?, n(12)?, up(13)?)), f[18]) } else { (None, *f.get(11)?) };
        Some(Record { png: f[0].to_string(), tw: n(1)?, th: n(2)?, psm: n(3)?, bw: n(4)?, tbp: n(5)?, pix, clut, gs_alpha: alpha == "gs" })
    }
}

// ---------------------------------------------------------------------------------------------- GS memory
const BLOCK32: [[u32; 8]; 4] = [[0, 1, 4, 5, 16, 17, 20, 21], [2, 3, 6, 7, 18, 19, 22, 23], [8, 9, 12, 13, 24, 25, 28, 29], [10, 11, 14, 15, 26, 27, 30, 31]];
const BLOCK16: [[u32; 4]; 8] = [[0, 2, 8, 10], [1, 3, 9, 11], [4, 6, 12, 14], [5, 7, 13, 15], [16, 18, 24, 26], [17, 19, 25, 27], [20, 22, 28, 30], [21, 23, 29, 31]];
const BLOCK16S: [[u32; 4]; 8] = [[0, 2, 16, 18], [1, 3, 17, 19], [8, 10, 24, 26], [9, 11, 25, 27], [4, 6, 20, 22], [5, 7, 21, 23], [12, 14, 28, 30], [13, 15, 29, 31]];
const COL32: [[u32; 8]; 8] = [
    [0, 1, 4, 5, 8, 9, 12, 13],
    [2, 3, 6, 7, 10, 11, 14, 15],
    [16, 17, 20, 21, 24, 25, 28, 29],
    [18, 19, 22, 23, 26, 27, 30, 31],
    [32, 33, 36, 37, 40, 41, 44, 45],
    [34, 35, 38, 39, 42, 43, 46, 47],
    [48, 49, 52, 53, 56, 57, 60, 61],
    [50, 51, 54, 55, 58, 59, 62, 63],
];
const COL16: [[u32; 16]; 8] = [
    [0, 2, 8, 10, 16, 18, 24, 26, 1, 3, 9, 11, 17, 19, 25, 27],
    [4, 6, 12, 14, 20, 22, 28, 30, 5, 7, 13, 15, 21, 23, 29, 31],
    [32, 34, 40, 42, 48, 50, 56, 58, 33, 35, 41, 43, 49, 51, 57, 59],
    [36, 38, 44, 46, 52, 54, 60, 62, 37, 39, 45, 47, 53, 55, 61, 63],
    [64, 66, 72, 74, 80, 82, 88, 90, 65, 67, 73, 75, 81, 83, 89, 91],
    [68, 70, 76, 78, 84, 86, 92, 94, 69, 71, 77, 79, 85, 87, 93, 95],
    [96, 98, 104, 106, 112, 114, 120, 122, 97, 99, 105, 107, 113, 115, 121, 123],
    [100, 102, 108, 110, 116, 118, 124, 126, 101, 103, 109, 111, 117, 119, 125, 127],
];
const COL8_FIRST: [[u32; 16]; 4] = [
    [0, 4, 16, 20, 32, 36, 48, 52, 2, 6, 18, 22, 34, 38, 50, 54],
    [8, 12, 24, 28, 40, 44, 56, 60, 10, 14, 26, 30, 42, 46, 58, 62],
    [33, 37, 49, 53, 1, 5, 17, 21, 35, 39, 51, 55, 3, 7, 19, 23],
    [41, 45, 57, 61, 9, 13, 25, 29, 43, 47, 59, 63, 11, 15, 27, 31],
];
const COL4_FIRST: [[u32; 32]; 4] = [
    [0, 8, 32, 40, 64, 72, 96, 104, 2, 10, 34, 42, 66, 74, 98, 106, 4, 12, 36, 44, 68, 76, 100, 108, 6, 14, 38, 46, 70, 78, 102, 110],
    [16, 24, 48, 56, 80, 88, 112, 120, 18, 26, 50, 58, 82, 90, 114, 122, 20, 28, 52, 60, 84, 92, 116, 124, 22, 30, 54, 62, 86, 94, 118, 126],
    [65, 73, 97, 105, 1, 9, 33, 41, 67, 75, 99, 107, 3, 11, 35, 43, 69, 77, 101, 109, 5, 13, 37, 45, 71, 79, 103, 111, 7, 15, 39, 47],
    [81, 89, 113, 121, 17, 25, 49, 57, 83, 91, 115, 123, 19, 27, 51, 59, 85, 93, 117, 125, 21, 29, 53, 61, 87, 95, 119, 127, 23, 31, 55, 63],
];

fn bits_of(psm: u32) -> u32 {
    match psm {
        2 | 10 => 16,
        19 | 27 => 8,
        20 | 36 | 44 => 4,
        1 => 24,
        _ => 32,
    }
}

/// Bit address in GS memory of pixel (x, y) and its width in bits (tools/assets/texconv.py `addr`).
fn addr(bp: u32, bw: u32, psm: u32, x: u32, y: u32) -> (usize, u32) {
    let bw = bw.max(1);
    let (x, y) = (x as usize, y as usize);
    let (bp, bw) = (bp as usize, bw as usize);
    match psm {
        2 | 10 => {
            let blk = if psm == 10 { &BLOCK16S } else { &BLOCK16 };
            let a = ((bp + ((y >> 1) & !31) * bw + ((x >> 1) & !31) + blk[(y >> 3) & 7][(x >> 4) & 3] as usize) << 7) + COL16[y & 7][x & 15] as usize;
            (((a * 2) & 0x3F_FFFF) * 8, 16)
        }
        19 => {
            let col = (y & 15) >> 2;
            let sx = if col & 1 != 0 { (x & 15) ^ 4 } else { x & 15 };
            let c8 = COL8_FIRST[y & 3][sx] as usize + col * 64;
            let a = ((bp + ((y >> 1) & !31) * (bw >> 1) + ((x >> 2) & !31) + BLOCK32[(y >> 4) & 3][(x >> 4) & 7] as usize) << 8) + c8;
            ((a & 0x3F_FFFF) * 8, 8)
        }
        20 => {
            let col = (y & 15) >> 2;
            let sx = if col & 1 != 0 { (x & 31) ^ 4 } else { x & 31 };
            let c4 = COL4_FIRST[y & 3][sx] as usize + col * 128;
            let a = ((bp + ((y >> 2) & !31) * (bw >> 1) + ((x >> 2) & !31) + BLOCK16[(y >> 4) & 7][(x >> 5) & 3] as usize) << 9) + c4;
            (((a >> 1) & 0x3F_FFFF) * 8 + (a & 1) * 4, 4)
        }
        _ => {
            let a = (((bp + (y & !31) * bw + ((x >> 1) & !31) + BLOCK32[(y >> 3) & 3][(x >> 3) & 7] as usize) << 6) + COL32[y & 7][x & 7] as usize) * 4;
            match psm {
                27 => (((a + 3) & 0x3F_FFFF) * 8, 8),
                36 => (((a + 3) & 0x3F_FFFF) * 8, 4),
                44 => (((a + 3) & 0x3F_FFFF) * 8 + 4, 4),
                1 => ((a & 0x3F_FFFF) * 8, 24),
                _ => ((a & 0x3F_FFFF) * 8, 32),
            }
        }
    }
}

/// The part of GS memory a texture covers, bit by bit: the bits written and their values.
struct Vram {
    base: usize,
    val: Vec<u64>,
    set: Vec<u64>,
}

impl Vram {
    fn new(lo: usize, hi: usize) -> Vram {
        let n = (hi - lo) / 64 + 1;
        Vram { base: lo, val: vec![0; n], set: vec![0; n] }
    }
    fn put(&mut self, at: usize, v: u64, n: u32) {
        for i in 0..n as usize {
            let b = at + i - self.base;
            let m = 1u64 << (b & 63);
            self.set[b >> 6] |= m;
            if (v >> i) & 1 != 0 {
                self.val[b >> 6] |= m;
            } else {
                self.val[b >> 6] &= !m;
            }
        }
    }
    /// The bit at `at`, if the texture wrote it.
    fn get(&self, at: usize) -> Option<bool> {
        let b = at.checked_sub(self.base)?;
        let w = *self.set.get(b >> 6)?;
        let m = 1u64 << (b & 63);
        if w & m == 0 {
            return None;
        }
        Some(self.val[b >> 6] & m != 0)
    }
}

/// The values the GS should read (rpsm, rw x rh at bp, width rbw) written back into the upload's data in buf.
fn put_plane(buf: &mut [u8], up: &Upload, bp: u32, rbw: u32, rpsm: u32, rw: u32, rh: u32, vals: &[u64]) -> Result<(), String> {
    let mut addrs = Vec::with_capacity((rw * rh) as usize);
    let (mut lo, mut hi) = (usize::MAX, 0usize);
    for y in 0..rh {
        for x in 0..rw {
            let (a, n) = addr(bp, rbw, rpsm, x, y);
            lo = lo.min(a);
            hi = hi.max(a + n as usize);
            addrs.push((a, n));
        }
    }
    if addrs.is_empty() {
        return Ok(());
    }
    let mut vram = Vram::new(lo, hi);
    for (k, &(a, n)) in addrs.iter().enumerate() {
        vram.put(a, vals[k], n);
    }
    let nb = bits_of(up.psm);
    let total = (up.w * up.h * nb) as usize;
    if up.off + total.div_ceil(8) > buf.len() {
        return Err("upload past the end of the file".into());
    }
    for y in 0..up.h {
        for x in 0..up.w {
            let (a, n) = addr(bp, up.bw, up.psm, x, y);
            let bit = ((y * up.w + x) * nb) as usize;
            for i in 0..n as usize {
                let Some(v) = vram.get(a + i) else { continue }; // (not decided by the texture: kept)
                let at = up.off + (bit + i) / 8;
                let m = 1u8 << ((bit + i) & 7);
                buf[at] = (buf[at] & !m) | if v { m } else { 0 };
            }
        }
    }
    Ok(())
}

fn clut_index(i: usize, psm: u32) -> usize {
    if psm == 19 || psm == 27 { (i & 0xE7) | ((i & 8) << 1) | ((i & 16) >> 1) } else { i }
}

/// An RGBA value (GS alpha) as stored in format psm.
fn raw_of(c: [u32; 4], psm: u32) -> u64 {
    let [r, g, b, a] = c;
    (match psm {
        2 | 10 => (r >> 3) | ((g >> 3) << 5) | ((b >> 3) << 10) | if a >= 0x40 { 0x8000 } else { 0 },
        1 => r | (g << 8) | (b << 16),
        _ => r | (g << 8) | (b << 16) | (a << 24),
    }) as u64
}

fn gs_alpha(a: u32, gs: bool) -> u32 {
    if gs { a } else { (a * 128 + 127) / 255 }
}

fn apply(buf: &mut [u8], rec: &Record, img: &Png) -> Result<(), String> {
    if (img.w, img.h) != (rec.tw, rec.th) {
        return Err(format!("{}x{} instead of {}x{}", img.w, img.h, rec.tw, rec.th));
    }
    match (&rec.clut, &img.palette) {
        (Some((cpsm, cbp, cup)), Some(pal)) => {
            let ncol = if rec.psm == 19 || rec.psm == 27 { 256 } else { 16 };
            let (cw, ch) = if ncol == 256 { (16, 16) } else { (8, 2) };
            let mut stored = vec![0u64; ncol];
            for k in 0..ncol {
                let c = pal.get(k).copied().unwrap_or([0, 0, 0, 0]);
                let c = [c[0] as u32, c[1] as u32, c[2] as u32, gs_alpha(c[3] as u32, rec.gs_alpha)];
                stored[clut_index(k, rec.psm)] = raw_of(c, *cpsm);
            }
            put_plane(buf, cup, *cbp, 1, *cpsm, cw, ch, &stored)?;
            let px: Vec<u64> = img.data.iter().map(|&v| v as u64).collect();
            put_plane(buf, &rec.pix, rec.tbp, rec.bw, rec.psm, rec.tw, rec.th, &px)
        }
        (None, None) => {
            let px: Vec<u64> = img
                .data
                .chunks_exact(4)
                .map(|c| raw_of([c[0] as u32, c[1] as u32, c[2] as u32, gs_alpha(c[3] as u32, rec.gs_alpha)], rec.psm))
                .collect();
            put_plane(buf, &rec.pix, rec.tbp, rec.bw, rec.psm, rec.tw, rec.th, &px)
        }
        _ => Err("palette PNG expected for an indexed texture (and RGBA for a direct one)".into()),
    }
}

// ---------------------------------------------------------------------------------------------- PNG
pub struct Png {
    pub w: u32,
    pub h: u32,
    /// indices (palette) or RGBA bytes
    pub data: Vec<u8>,
    pub palette: Option<Vec<[u8; 4]>>,
}

fn be32(b: &[u8]) -> u32 {
    u32::from_be_bytes([b[0], b[1], b[2], b[3]])
}

pub fn png_read(blob: &[u8]) -> Result<Png, String> {
    if blob.len() < 8 || &blob[..8] != b"\x89PNG\r\n\x1a\n" {
        return Err("not a PNG".into());
    }
    let (mut w, mut h, mut depth, mut ctype, mut interlace) = (0, 0, 0, 0, 0);
    let mut idat = Vec::new();
    let mut plte: Vec<[u8; 4]> = Vec::new();
    let mut trns: &[u8] = &[];
    let mut o = 8;
    while o + 12 <= blob.len() {
        let n = be32(&blob[o..]) as usize;
        let kind = &blob[o + 4..o + 8];
        let body = blob.get(o + 8..o + 8 + n).ok_or("truncated chunk")?;
        match kind {
            b"IHDR" => {
                w = be32(body);
                h = be32(&body[4..]);
                depth = body[8];
                ctype = body[9];
                interlace = body[12];
            }
            b"PLTE" => plte = body.chunks_exact(3).map(|c| [c[0], c[1], c[2], 255]).collect(),
            b"tRNS" => trns = body,
            b"IDAT" => idat.extend_from_slice(body),
            _ => {}
        }
        o += 12 + n;
    }
    if depth != 8 || interlace != 0 {
        return Err(format!("8-bit, non-interlaced PNG expected (depth {depth}, interlace {interlace})"));
    }
    let bpp = match ctype {
        3 => 1,
        6 => 4,
        2 => 3,
        _ => return Err(format!("colour type {ctype} not supported (palette or RGBA)")),
    };
    let raw = inflate_zlib(&idat)?;
    let stride = (w * bpp) as usize;
    if raw.len() < (stride + 1) * h as usize {
        return Err("image data too short".into());
    }
    let mut out = vec![0u8; stride * h as usize];
    for y in 0..h as usize {
        let f = raw[y * (stride + 1)];
        let line = &raw[y * (stride + 1) + 1..(y + 1) * (stride + 1)];
        for x in 0..stride {
            let a = if x >= bpp as usize { out[y * stride + x - bpp as usize] as i32 } else { 0 };
            let b = if y > 0 { out[(y - 1) * stride + x] as i32 } else { 0 };
            let c = if y > 0 && x >= bpp as usize { out[(y - 1) * stride + x - bpp as usize] as i32 } else { 0 };
            let p = match f {
                0 => 0,
                1 => a,
                2 => b,
                3 => (a + b) / 2,
                4 => {
                    let p = a + b - c;
                    let (pa, pb, pc) = ((p - a).abs(), (p - b).abs(), (p - c).abs());
                    if pa <= pb && pa <= pc { a } else if pb <= pc { b } else { c }
                }
                _ => return Err(format!("bad filter {f}")),
            };
            out[y * stride + x] = (line[x] as i32 + p) as u8;
        }
    }
    if ctype == 3 {
        for (i, &t) in trns.iter().enumerate() {
            if let Some(p) = plte.get_mut(i) {
                p[3] = t;
            }
        }
        return Ok(Png { w, h, data: out, palette: Some(plte) });
    }
    if ctype == 2 {
        let rgba = out.chunks_exact(3).flat_map(|c| [c[0], c[1], c[2], 255]).collect();
        return Ok(Png { w, h, data: rgba, palette: None });
    }
    Ok(Png { w, h, data: out, palette: None })
}

// ---------------------------------------------------------------------------------------------- inflate
struct Bits<'a> {
    d: &'a [u8],
    pos: usize,
    bit: u32,
    nbits: u32,
}

impl<'a> Bits<'a> {
    fn need(&mut self, n: u32) -> Result<(), String> {
        while self.nbits < n {
            let b = *self.d.get(self.pos).ok_or("deflate data ends early")? as u32;
            self.pos += 1;
            self.bit |= b << self.nbits;
            self.nbits += 8;
        }
        Ok(())
    }
    fn take(&mut self, n: u32) -> Result<u32, String> {
        if n == 0 {
            return Ok(0);
        }
        self.need(n)?;
        let v = self.bit & ((1u32 << n) - 1);
        self.bit >>= n;
        self.nbits -= n;
        Ok(v)
    }
}

/// Canonical Huffman table: (counts per length, symbols in order).
struct Huff {
    count: [u16; 16],
    sym: Vec<u16>,
}

impl Huff {
    fn new(lengths: &[u8]) -> Huff {
        let mut count = [0u16; 16];
        for &l in lengths {
            count[l as usize] += 1;
        }
        count[0] = 0;
        let mut offs = [0u16; 16];
        for i in 1..16 {
            offs[i] = offs[i - 1] + count[i - 1];
        }
        let mut sym = vec![0u16; lengths.len()];
        for (s, &l) in lengths.iter().enumerate() {
            if l != 0 {
                sym[offs[l as usize] as usize] = s as u16;
                offs[l as usize] += 1;
            }
        }
        Huff { count, sym }
    }
    fn decode(&self, b: &mut Bits) -> Result<u16, String> {
        let (mut code, mut first, mut index) = (0i32, 0i32, 0i32);
        for len in 1..16 {
            code |= b.take(1)? as i32;
            let count = self.count[len] as i32;
            if code - count < first {
                return Ok(self.sym[(index + (code - first)) as usize]);
            }
            index += count;
            first += count;
            first <<= 1;
            code <<= 1;
        }
        Err("bad Huffman code".into())
    }
}

const LBASE: [u16; 29] = [3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131, 163, 195, 227, 258];
const LEXT: [u8; 29] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0];
const DBASE: [u16; 30] = [1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537, 2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577];
const DEXT: [u8; 30] = [0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13];

/// zlib (RFC 1950) around deflate (RFC 1951).
pub fn inflate_zlib(d: &[u8]) -> Result<Vec<u8>, String> {
    if d.len() < 2 || (d[0] & 15) != 8 {
        return Err("not zlib data".into());
    }
    let mut b = Bits { d: &d[2..], pos: 0, bit: 0, nbits: 0 };
    let mut out = Vec::new();
    loop {
        let last = b.take(1)?;
        let kind = b.take(2)?;
        match kind {
            0 => {
                b.bit = 0;
                b.nbits = 0;
                let p = b.pos;
                let len = u16::from_le_bytes([*b.d.get(p).ok_or("eof")?, *b.d.get(p + 1).ok_or("eof")?]) as usize;
                let start = p + 4;
                out.extend_from_slice(b.d.get(start..start + len).ok_or("stored block past the end")?);
                b.pos = start + len;
            }
            1 | 2 => {
                let (lit, dist) = if kind == 1 {
                    let mut l = [0u8; 288];
                    for (i, x) in l.iter_mut().enumerate() {
                        *x = if i < 144 { 8 } else if i < 256 { 9 } else if i < 280 { 7 } else { 8 };
                    }
                    (Huff::new(&l), Huff::new(&[5u8; 30]))
                } else {
                    let hlit = b.take(5)? as usize + 257;
                    let hdist = b.take(5)? as usize + 1;
                    let hclen = b.take(4)? as usize + 4;
                    const ORDER: [usize; 19] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];
                    let mut cl = [0u8; 19];
                    for &i in ORDER.iter().take(hclen) {
                        cl[i] = b.take(3)? as u8;
                    }
                    let ch = Huff::new(&cl);
                    let mut lens = vec![0u8; hlit + hdist];
                    let mut i = 0;
                    while i < hlit + hdist {
                        let s = ch.decode(&mut b)?;
                        match s {
                            0..=15 => {
                                lens[i] = s as u8;
                                i += 1;
                            }
                            16 => {
                                let prev = *lens.get(i.wrapping_sub(1)).ok_or("repeat with nothing before")?;
                                for _ in 0..3 + b.take(2)? {
                                    *lens.get_mut(i).ok_or("too many lengths")? = prev;
                                    i += 1;
                                }
                            }
                            17 => i += 3 + b.take(3)? as usize,
                            _ => i += 11 + b.take(7)? as usize,
                        }
                    }
                    if i > hlit + hdist {
                        return Err("too many lengths".into());
                    }
                    (Huff::new(&lens[..hlit]), Huff::new(&lens[hlit..]))
                };
                loop {
                    let s = lit.decode(&mut b)?;
                    if s < 256 {
                        out.push(s as u8);
                    } else if s == 256 {
                        break;
                    } else {
                        let k = (s - 257) as usize;
                        let len = *LBASE.get(k).ok_or("bad length")? as usize + b.take(*LEXT.get(k).unwrap_or(&0) as u32)? as usize;
                        let ds = dist.decode(&mut b)? as usize;
                        let dd = *DBASE.get(ds).ok_or("bad distance")? as usize + b.take(*DEXT.get(ds).unwrap_or(&0) as u32)? as usize;
                        if dd > out.len() {
                            return Err("distance too far back".into());
                        }
                        let start = out.len() - dd;
                        for j in 0..len {
                            let v = out[start + j];
                            out.push(v);
                        }
                    }
                }
            }
            _ => return Err("bad block type".into()),
        }
        if last == 1 {
            break;
        }
    }
    Ok(out)
}
