"""Writes the open-format extraction of the game's files: assets/files/<partition>/<number>/ (docs/open-formats.md).
game/src/host/assets.rs rebuilds each file from it when the game reads the file; something is taken out
only when the rebuild gives back the exact original bytes (checked here for every file).

* skeleton.bin: the file with everything taken out zeroed (unpacked, with bpe.txt, for a file packed with the
  game's BPE: bpe.py).
* textures.txt + PNGs (texconv.py), model.gltf (modelconv.py for fighters, stageconv.py for stages; a stage's
  collision in collision.gltf, colconv.py), texts.json
  (textconv.py), lips.json (lipsconv.py), cameras.json (camconv.py), WAVs of sound banks + samples.txt (vagconv.py), GSC scripts as script.txt (gscconv.py).
* An animation bank / a directory of packed files: bank.txt, its motions' keys in animations.gltf (motion.py) and
  each other entry unpacked, as entry_<n>.bin or, when something in it could be taken out, a folder entry_<n>/ of
  its own (the same files as above).

    python3 files_extract.py <raw data folder: assets/_legacy/raw> <assets folder> [partition ...]
"""
import sys, os, time, struct, urllib.parse
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import texconv as T
import bpe
import asset_dirs
import modelconv
import motion
import stageconv
import textconv
import lipsconv
import camconv
import vagconv
import gscconv
import adxconv
import colconv
import pmdlconv
import fmotionconv
import stagechain
import stagelayout
import filetables
import layout
import texpackets
import fodconv
import effchain
import fightertables
import fxconv
import rawtex
import surfconv


def line(name, r):
    p = r["pix"]
    s = "%s %d %d %d %d %d %d %d %d %d %d" % (name, r["tw"], r["th"], r["psm"], r["bw"], r["tbp"], p[0], p[2], p[3], p[4], p[5])
    if "cpsm" in r:
        c = r["clut"]
        s += " %d %d %d %d %d %d %d" % (r["cpsm"], r["cbp"], c[0], c[2], c[3], c[4], c[5])
    return s + " " + r["alpha"]


def nested_models(data):
    """-> [(start, end)] of the entries of data that are fighter-style model files of their own (a directory whose
    entry 2 is a pmdl: the technique models inside the effect files), found in the directories down to depth 3"""
    out = []

    def u(o):
        return struct.unpack_from("<I", data, o)[0]

    def entries(base, end):
        if end - base < 8:
            return []
        n = u(base)
        if not 1 <= n <= 4096 or 4 + 4 * n > end - base:
            return []
        offs = [u(base + 4 + 4 * i) & ~3 for i in range(n)]
        if offs[0] < 4 + 4 * n or any(y < x for x, y in zip(offs, offs[1:])) or base + offs[-1] > end:
            return []
        return [(base + o, base + e) for o, e in zip(offs, offs[1:] + [end - base]) if e > o]

    def is_model(a, b):
        es = entries(a, b)
        return len(es) >= 1 and u(a) >= 0x1C and data[a + u(a + 12):a + u(a + 12) + 4] == b"pmdl"

    def walk(base, end, depth):
        for a, b in entries(base, end):
            if depth >= 1 and is_model(a, b):
                out.append((a, b))
            elif depth < 3:
                walk(a, b, depth + 1)

    walk(0, len(data), 0)
    return out


def convert_data(data, name, nest=True):
    """Everything that can be taken out of data (unpacked bytes). -> ({file name: bytes}, textures) or (None, 0)
    when nothing was taken out. Asserts that the files rebuild data exactly. A model file inside the data (the
    technique models of the effect files) is extracted like a file of its own into at_<offset>/ (nested.txt)."""
    nested = {}
    if nest:
        masked = None
        for a, b in nested_models(data):
            try:
                sub, _ = convert_data(data[a:b], "%s @%d" % (name, a), nest=False)
            except AssertionError:
                continue
            if sub:
                nested[(a, b)] = sub
                masked = masked or bytearray(data)
                masked[a:b] = bytes(b - a)
        if nested:
            files, nt = convert_data(bytes(masked), name, nest=False)
            lines = []
            for (a, b), sub in nested.items():
                for k, v in sub.items():
                    files["at_%08x/%s" % (a, k)] = v
                lines.append("at_%08x %d %d" % (a, a, b - a))
            files["nested.txt"] = ("# model files inside this file, each extracted like a file of its own (tools/assets/"
                                   "files_extract.py): folder, offset, length\n" + "\n".join(lines) + "\n").encode()
            return files, nt
    skel, recs, blobs = T.convert(data)
    names = ["tex_%03d_%dx%d.png" % (i, r["tw"], r["th"]) for i, r in enumerate(recs)]
    skel = bytearray(skel)
    # the upload packets around the textures' data and the texture entries
    packets = texpackets.convert(data, skel, recs) if recs else None
    tents = texpackets.entries(data, skel, recs) if recs else None
    model = None
    try:
        taken = bytearray(skel)
        model = modelconv.convert(data, taken, [(r["tbp"], r.get("cbp"), n) for r, n in zip(recs, names)])
        skel = taken
    except (ValueError, IndexError, struct.error) as e:
        print("%s: model left in the file (%s)" % (name, e))
    if model is None:
        try:
            taken = bytearray(skel)
            model = stageconv.convert(data, taken, [(r["entry"], n) for r, n in zip(recs, names)])
            if model:
                check = bytearray(T.assemble(bytes(taken), recs, blobs) if recs else taken)
                if packets:
                    texpackets.apply(check, packets)
                if tents:
                    layout.apply(check, tents)
                stageconv.apply(check, *model)
                if check != data:
                    raise ValueError("does not rebuild")
                skel = taken
        except (ValueError, IndexError, KeyError, struct.error):
            model = None
    layout_ = None
    if model and "layout stage" not in model[2]:
        # the rest of a fighter model (node records, chains): model.json, nothing of it left in the skeleton
        try:
            taken = bytearray(skel)
            layout_ = pmdlconv.convert(data, taken)
            if layout_:
                skel = taken
        except (ValueError, IndexError, struct.error) as e:
            print("%s: model layout left in the file (%s)" % (name, e))
            layout_ = None
    fmot = None
    if layout_:
        # the motions of the fighter file (entries 0x1C..0x23): model-motions.gltf / .json
        try:
            taken = bytearray(skel)
            fmot = fmotionconv.convert(data, taken)
            if fmot:
                skel = taken
        except (ValueError, IndexError, KeyError, struct.error) as e:
            print("%s: motions left in the file (%s)" % (name, e))
            fmot = None
    chains = None
    if model and "layout stage" in model[2]:
        # the meshes' drawing chains (stage-chains.json), written from model.gltf
        try:
            taken = bytearray(skel)
            chains = stagechain.convert(data, taken)
            if chains:
                skel = taken
        except (ValueError, IndexError, KeyError, struct.error) as e:
            print("%s: stage chains left in the file (%s)" % (name, e))
            chains = None
    eff = None
    try:
        taken = bytearray(skel)
        eff = effchain.convert(data, taken)
        if eff:
            skel = taken
    except (ValueError, IndexError, KeyError, struct.error) as e:
        print("%s: effect meshes left in the file (%s)" % (name, e))
        eff = None
    col = None
    if model and "layout stage" in model[2]:
        try:
            taken = bytearray(skel)
            col = colconv.convert(data, taken)
            if col:
                skel = taken
        except (ValueError, IndexError, struct.error) as e:
            print("%s: collision left in the file (%s)" % (name, e))
            col = None
    surf = None
    if model and "layout stage" in model[2]:
        # the stage's animated surfaces (surfaces.gltf / .txt, surface-tables.json)
        try:
            taken = bytearray(skel)
            surf = surfconv.convert(data, taken)
            if surf:
                skel = taken
        except (ValueError, IndexError, KeyError, struct.error) as e:
            print("%s: surfaces left in the file (%s)" % (name, e))
            surf = None
    mtab = None
    if layout_:
        # the game tables of the fighter (model-tables.json)
        try:
            taken = bytearray(skel)
            mtab = fightertables.convert(data, taken)
            if mtab:
                skel = taken
        except (ValueError, IndexError, KeyError, struct.error) as e:
            print("%s: fighter tables left in the file (%s)" % (name, e))
            mtab = None
    slay = None
    if model and "layout stage" in model[2]:
        # the stage's tables (stage-layout.json)
        try:
            taken = bytearray(skel)
            slay = stagelayout.convert(data, taken)
            if slay:
                skel = taken
        except (ValueError, IndexError, KeyError, struct.error) as e:
            print("%s: stage tables left in the file (%s)" % (name, e))
            slay = None
    movies = fodconv.convert(data, skel)
    texts = textconv.convert(data, skel)
    lips = lipsconv.convert(data, skel)
    cams = camconv.convert(data, skel)
    camtab = camconv.tables(data, skel) if cams else None
    sounds = vagconv.convert(data, skel)
    sndtab = vagconv.tables(data, skel)
    script = gscconv.convert(data, skel)
    stream = adxconv.convert(data, skel)
    fxt = fxconv.convert(data, skel)
    raws = rawtex.convert(data, skel)
    # what is left: the directory and the words not understood yet (file-tables.json); skeleton.bin all zeros then
    rest = filetables.convert(data, skel)
    check = bytearray(T.assemble(bytes(skel), recs, blobs) if recs else skel)
    # the typed records first: the other files may read the structure they give
    if packets:
        texpackets.apply(check, packets)
    if movies:
        fodconv.apply(check, movies)
    if surf:
        surfconv.apply(check, surf[0], surf[1], surf[2])
    if raws:
        rawtex.apply(check, *raws)
    for t in (tents, camtab, mtab, slay, surf[3] if surf else None, fxt, sndtab, rest):
        if t:
            layout.apply(check, t)
    if texts:
        textconv.apply(check, texts)
    if lips:
        lipsconv.apply(check, lips)
    if cams:
        camconv.apply(check, cams)
    if sounds:
        vagconv.apply(check, *sounds)
    if script:
        gscconv.apply(check, script)
    if stream:
        adxconv.apply(check, *stream)
    if model:
        (stageconv if "layout stage" in model[2] else modelconv).apply(check, *model)
    if col:
        colconv.apply(check, *col)
    if chains:
        stagechain.apply(check, chains, model[0], model[1])
    if eff:
        effchain.apply(check, *eff)
    if fmot:
        fmotionconv.apply(check, fmot[0], fmot[1], fmot[2], data)
    if layout_:
        pmdlconv.apply(check, layout_, model[0], model[1])
    assert check == data, name
    files = {}
    lines = ["# textures of %s (tools/assets/files_extract.py): png tw th psm bw tbp | data offset w h psm bw | [cpsm cbp | offset w h psm bw] alpha" % name]
    for n, r, b in zip(names, recs, blobs):
        files[n] = b
        lines.append(line(n, r))
    if recs:
        files["textures.txt"] = ("\n".join(lines) + "\n").encode()
    if any(skel):
        files["skeleton.bin"] = bytes(skel)
    else:
        files["skeleton.txt"] = ("# the file is all zeros but for what the other files of this folder give\nzeros %d\n" % len(skel)).encode()
    if movies:
        files["movies.json"] = movies.encode("utf-8")
    if packets:
        files["texture-packets.txt"] = packets.encode()
    if tents:
        files["texture-entries.json"] = tents.encode()
    if mtab:
        files["model-tables.json"] = mtab.encode()
    if fxt:
        files["effect-tables.json"] = fxt.encode()
    if sndtab:
        files["sound-tables.json"] = sndtab.encode()
    if surf:
        files["surfaces.gltf"] = surf[0].encode()
        files["surfaces.bin"] = surf[1]
        files["surfaces.txt"] = surf[2].encode()
        files["surface-tables.json"] = surf[3].encode()
    if raws:
        files.update(raws[0])
        files["raw-textures.txt"] = raws[1].encode()
    if slay:
        files["stage-layout.json"] = slay.encode()
    if rest:
        files["file-tables.json"] = rest.encode()
    if texts:
        files["texts.json"] = texts.encode("utf-8")
    if lips:
        files["lips.json"] = lips.encode()
    if cams:
        files["cameras.json"] = cams.encode()
    if camtab:
        files["camera-tables.json"] = camtab.encode()
    if script:
        files["script.txt"] = script.encode()
    if stream:
        files["stream.wav"] = stream[0]
        files["adx.txt"] = stream[1].encode()
    if sounds:
        files.update(sounds[0])
        files["samples.txt"] = sounds[1].encode()
    if model:
        files["model.gltf"] = model[0].encode()
        files["model.bin"] = model[1]
        files["models.txt"] = model[2].encode()
    if layout_:
        files["model.json"] = layout_.encode()
    if chains:
        files["stage-chains.json"] = chains.encode()
    if eff:
        files["effects.gltf"] = eff[0].encode()
        files["effects.bin"] = eff[1]
        files["effect-chains.json"] = eff[2].encode()
    if fmot:
        files["model-motions.gltf"] = fmot[0].encode()
        files["model-motions.bin"] = fmot[1]
        files["model-motions.json"] = fmot[2].encode()
    if col:
        files["collision.gltf"] = col[0].encode()
        files["collision.bin"] = col[1]
    return files, len(recs)


def write(out, files):
    for name, v in files.items():
        path = os.path.join(out, name)
        os.makedirs(os.path.dirname(path), exist_ok=True)
        tmp = path + ".tmp"
        open(tmp, "wb").write(v)
        os.replace(tmp, path)  # whole or not at all


SKIP_SUFFIXES = (".ALG", ".PSS", ".DIR", "SLUS_216.78")  # the partition archives, the movies, the index


def extract_one(raw, assets, part, f, banks):
    """one file of the disc to its open files. Returns (written, textures, deferred): an animation bank is left for
    the second round (`banks` False), when the model of its fighter (8 files before it) is written."""
    src = os.path.join(raw, part)
    d = open(os.path.join(src, f), "rb").read()
    stem = f[:-4] if f.endswith(".bin") else f
    out = os.path.join(assets, "files", part, stem)
    if part == "pzs3us1" and stem.isdigit():
        out = str(asset_dirs.find(assets, int(stem)))  # its folder when it was grouped already (organize.py)
    packed = bpe.is_bpe(d)
    files = None
    ntex = 0
    if not packed and motion.bank_entries(d):
        if not banks:
            return 0, 0, True
        # an animation bank: the model of its fighter is 8 files before it (first costume)
        model = None
        if stem.isdigit() and part == "pzs3us1":
            mdir = asset_dirs.find(assets, int(stem) - 8)
            if (mdir / "model.gltf").exists():
                ref = urllib.parse.quote(os.path.relpath(mdir, out).replace(os.sep, "/"))
                model = ((mdir / "model.gltf").read_text(), ref)
        files = motion.convert_bank(d, model, lambda data, k: convert_data(data, "%s/%s entry %d" % (part, f, k))[0])
        if files:
            ntex += sum(1 for n in files if n.endswith(".png"))
    if files is None:
        data, tail = bpe.expand(d) if packed else (d, b"")
        files, nt = convert_data(data, "%s/%s" % (part, f))
        if files is not None and packed:
            files["bpe.txt"] = ("# %s/%s is packed with the game's BPE (tools/assets/bpe.py); skeleton.bin is unpacked\n"
                                "block 10000 hash 16384 tail %s\n" % (part, f, tail.hex())).encode()
        ntex += nt
    if files:
        write(out, files)
    return (1 if files else 0), ntex, False


def _work(job):
    raw, assets, part, f, banks = job
    try:
        return (part, f) + extract_one(raw, assets, part, f, banks) + (None,)
    except Exception as e:  # reported by the main process, which stops
        import traceback
        return (part, f, 0, 0, False, "%s: %s\n%s" % (f, e, traceback.format_exc()))


def workers():
    """processes: BT3_EXTRACT_JOBS, else one per processor (the files are independent of each other)"""
    n = os.environ.get("BT3_EXTRACT_JOBS")
    return max(1, int(n) if n else (os.cpu_count() or 1))


def main():
    raw, assets = sys.argv[1], sys.argv[2]
    parts = sys.argv[3:] or ["pzs3us1"]
    t0 = time.time()
    nfiles = ntex = 0
    os.makedirs(os.path.join(assets, "files"), exist_ok=True)
    deadline = time.time() + float(os.environ.get("SECONDS_MAX", "1e9"))
    # resumable: the files already looked at are listed in assets/files/extraidos-<part>.txt
    todo, done_files = [], {}
    for part in parts:
        src = os.path.join(raw, part)
        done_path = os.path.join(assets, "files", "extraidos-%s.txt" % part)
        old_path = os.path.join(assets, "files", part, "scanned3.txt")  # where earlier versions kept it
        if os.path.exists(old_path) and not os.path.exists(done_path):
            os.replace(old_path, done_path)
        done = set(open(done_path).read().split()) if os.path.exists(done_path) else set()
        done_files[part] = open(done_path, "a")
        # loose files in folders too (disc/DATA/...): names relative to the partition, with /
        names = sorted(os.path.relpath(os.path.join(r, n), src).replace(os.sep, "/") for r, _, ns in os.walk(src) for n in ns)
        todo += [(part, f) for f in names if f not in done and not f.upper().endswith(SKIP_SUFFIXES)]
    total = len(todo)
    jobs = workers()
    print("%d files to convert, %d processes" % (total, jobs), flush=True)

    def run_round(items, banks):
        nonlocal nfiles, ntex
        deferred = []
        args = [(raw, assets, part, f, banks) for part, f in items]
        if jobs > 1 and len(args) > 1:
            import multiprocessing
            pool = multiprocessing.Pool(jobs)
            results = pool.imap_unordered(_work, args, chunksize=2)
        else:
            pool, results = None, map(_work, args)
        try:
            for part, f, wrote, nt, later, err in results:
                if err:
                    raise SystemExit("failed on %s/%s" % (part, err))
                if later:
                    deferred.append((part, f))
                    continue
                nfiles += wrote
                ntex += nt
                done_files[part].write(f + "\n")
                done_files[part].flush()
                if wrote and nfiles % 100 == 0:
                    print("%d files, %d textures (%.0fs) of %d" % (nfiles, ntex, time.time() - t0, total), flush=True)
                if time.time() > deadline:
                    print("stopped for now (time limit); run again to continue")
                    raise SystemExit(0)
        finally:
            if pool is not None:
                pool.terminate()
        return deferred

    # first round: every file but the animation banks; second: the banks, now that the models are written
    banks = run_round(todo, False)
    run_round(banks, True)
    print("done: %d files, %d textures (%.0fs)" % (nfiles, ntex, time.time() - t0))


if __name__ == "__main__":
    main()
