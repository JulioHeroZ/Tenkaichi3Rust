"""Groups the extracted files by kind and names their folders, so it is clear what each one is:

    assets/files/lutadores/Goku (Early)/01423 roupa 1/
    assets/files/cenários/12 Kame House/00380 cenário/
    assets/files/sons/00329 efeitos de batalha/
    assets/files/menus/00448/

The number stays at the start of every name: the game finds each file by it (game/src/host/assets.rs file_dirs,
asset_dirs.py), so the names and the grouping are free. What a file is comes from the decompilation's file ids
(include/battle/battle_work.h, include/battle/view_a.h, include/menu/menu_d.h; folder = file id - 1) and the names
of the characters and stages from the game's own texts (files 00448 and 00454); the rest is grouped by what its
folder holds. The glTF links between folders (the animations' "../01423/model.bin") are rewritten. Can be run
again: folders already in place stay; new numbered folders (files/pzs3us1/<NNNNN>) are moved in.

    python tools/assets/organize.py [assets]
"""
import json, os, re, sys, urllib.parse
from pathlib import Path

import asset_dirs

CHARA_COUNT = 161      # CHRVIEW_CHARA_COUNT
STAGE_COUNT = 35       # SAVE_STAGE_COUNT
CHARA = 0x590 - 1      # BTL_FILE_CHARA: + character * 10 + costume; +4 damaged; +8 animations; +9 effects
CHARA_DATA = 8 - 1     # BTL_FILE_CHARA_DATA: + character * 2 + side
VOICE = 0xBDA - 1      # BTL_FILE_VOICE: + character
VOICE_ALT = 0xC7B - 1  # BTL_FILE_VOICE_ALT: + character (the other voice option)
FACE = 0x2F9 - 1       # N_FACE_FILE: + character
STAGE = 0x171 - 1      # BTL_FILE_STAGE: + stage
STAGE_SPLIT = 0x198 - 1
SND_STAGE = 0x14E - 1  # BTL_FILE_SND_STAGE: + stage
STAGE_PICTURE = 0x39D - 1  # CHARSEL_STAGE_FILE: + stage
FIXED = {0x14A - 1: ("sons", "efeitos de batalha"), 0x14B - 1: ("sons", "efeitos comuns"),
         0x197 - 1: ("cenários", "fundo do visualizador de personagens")}
for d in range(3):
    FIXED[0x194 - 1 + d] = ("cenários", "fundo do dragão %d" % (d + 1))  # SHENSCENE_FILE_STAGE + dragon

# by what the folder holds, in this order
KINDS = [("script.txt", "roteiros"), ("movies.json", "menus"), ("audio-bank.json", "sons"), ("samples.txt", "sons"),
         ("stream.wav", "músicas e vozes"), ("collision.gltf", "cenários"), ("bank.txt", "animações"),
         ("motion-sources.json", "animações"), ("model.gltf", "modelos"), ("effect-tables.json", "efeitos"),
         ("effects.gltf", "efeitos"), ("lip-sync.json", "sincronia labial"), ("lips.json", "sincronia labial"),
         ("text-table.json", "textos"), ("texts.json", "textos"), ("empty-directory.json", "vazios"),
         ("roster-items.json", "tabelas"), ("point-groups.json", "tabelas")]


def clean(name):
    """a folder name from a text of the game: its font's marks out, no characters Windows refuses"""
    name = re.sub(r"[^\x20-\x7EÀ-ÿ]", "", name)
    name = re.sub(r'[<>:"/\\|?*]', "", name)
    return re.sub(r"\s+", " ", name).strip().rstrip(".")


def sections(texts):
    """the texts of a texts.json split where the entry numbers start again (nested tables)"""
    out, prev = [], None
    for t in texts:
        if prev is None or t["entry"] <= prev:
            out.append([])
        out[-1].append(t)
        prev = t["entry"]
    return out


def names(dirs):
    chars, stages = {}, {}
    try:
        s = sections(json.loads((dirs[448] / "entry_002" / "texts.json").read_text(encoding="utf-8"))["texts"])
        for c in range(CHARA_COUNT):
            n, f = clean(s[0][c]["text"]), clean(s[1][c]["text"])
            chars[c] = "%s - %s" % (n, f) if f else n
    except (KeyError, IndexError, OSError, ValueError):
        pass
    try:
        for sec in sections(json.loads((dirs[454] / "texts.json").read_text(encoding="utf-8"))["texts"]):
            by = {t["entry"]: t["text"] for t in sec}
            if "Noon" in by.get(56, "") and len(by) > 90:
                for s_ in range(STAGE_COUNT):
                    stages[s_] = clean(by[56 + s_])
                break
    except (KeyError, OSError, ValueError):
        pass
    return chars, stages


def plan(dirs):
    """{number: path of its folder relative to files/}"""
    chars, stages = names(dirs)
    out = {}
    unique = {}
    for c in range(CHARA_COUNT):
        who = chars.get(c, "personagem %03d" % c)
        # two characters with the same name and form keep apart by their number
        if who in unique.values():
            who = "%s (%03d)" % (who, c)
        unique[c] = who
        group = "lutadores/" + who
        base = CHARA + 10 * c
        for n in range(4):
            out[base + n] = (group, "roupa %d" % (n + 1))
            out[base + 4 + n] = (group, "roupa %d (danificada)" % (n + 1))
        out[base + 8] = (group, "animações")
        out[base + 9] = (group, "efeitos das técnicas")
        out[VOICE + c] = (group, "vozes")
        out[VOICE_ALT + c] = (group, "vozes (outra opção de voz)")
        out[FACE + c] = (group, "retrato")
        out[CHARA_DATA + 2 * c] = (group, "dados (lado 1)")
        out[CHARA_DATA + 2 * c + 1] = (group, "dados (lado 2)")
    for s_ in range(STAGE_COUNT):
        group = "cenários/%02d %s" % (s_, stages.get(s_, "cenário"))
        out[STAGE + s_] = (group, "cenário")
        out[STAGE_SPLIT + s_] = (group, "cenário (tela dividida)")
        out[SND_STAGE + s_] = (group, "sons do cenário")
        out[STAGE_PICTURE + s_] = (group, "imagem de seleção")
    out.update(FIXED)
    paths = {}
    for n, d in dirs.items():
        if n in out:
            group, what = out[n]
            paths[n] = "%s/%05d %s" % (group, n, what)
            continue
        kind = next((k for f, k in KINDS if (d / f).is_file()), None)
        if kind is None:
            kind = "imagens" if any(d.glob("tex_*.png")) else "outros"
        paths[n] = "%s/%05d" % (kind, n)
    return paths


def relink(folder, moved):
    """the glTF links of a folder to other folders made to point at their new places: the animations of a fighter
    take the skeleton of its model ("../01423/model.bin", motion.py)"""
    for g in folder.glob("animations.gltf"):
        text = g.read_text(encoding="utf-8")
        if '"../' not in text:
            continue
        doc = json.loads(text)
        changed = False
        for key in ("buffers", "images"):
            for item in doc.get(key, []):
                uri = item.get("uri", "")
                m = re.match(r"\.\./(?:[^/]*/)*?(\d{5})(?:[^/]*)/(.+)$", urllib.parse.unquote(uri))
                if not m or int(m.group(1)) not in moved:
                    continue
                target = moved[int(m.group(1))] / m.group(2)
                new = urllib.parse.quote(os.path.relpath(target, folder).replace(os.sep, "/"))
                if new != uri:
                    item["uri"] = new
                    changed = True
        if changed:
            tmp = g.with_name(g.name + ".tmp")
            tmp.write_text(json.dumps(doc, separators=(",", ":")), encoding="utf-8")
            tmp.replace(g)


def main():
    root = Path(__file__).resolve().parents[2]
    assets = Path(sys.argv[1]) if len(sys.argv) > 1 else root / "assets"
    files = assets / "files"
    dirs = asset_dirs.index(assets)
    paths = plan(dirs)
    moved = 0
    for n, rel in sorted(paths.items()):
        target = files / rel
        if dirs[n] != target:
            if target.exists():
                raise SystemExit("%s já existe; não movi %s" % (target, dirs[n]))
            target.parent.mkdir(parents=True, exist_ok=True)
            dirs[n].rename(target)
            moved += 1
        dirs[n] = target
    for d in dirs.values():
        relink(d, dirs)
    # the list of files already looked at by files_extract.py goes with it; files/pzs3us1 is left empty
    old = files / "pzs3us1"
    if old.is_dir():
        if (old / "scanned3.txt").is_file():
            (files / "extraidos-pzs3us1.txt").write_bytes((old / "scanned3.txt").read_bytes())
            (old / "scanned3.txt").unlink()
        try:
            old.rmdir()
        except OSError:
            pass
    counts = {}
    for rel in paths.values():
        counts[rel.split("/")[0]] = counts.get(rel.split("/")[0], 0) + 1
    print("%d pastas, %d movidas: %s" % (len(paths), moved, ", ".join("%s %d" % kv for kv in sorted(counts.items()))))


if __name__ == "__main__":
    main()
