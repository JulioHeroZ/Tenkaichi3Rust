"""The HD texture packs (optional): the remaster made by the PLAYANDO channel for the Wii version under Dolphin
(https://www.youtube.com/watch?v=rJW7idQDqVc), freely shared on Google Drive as two RAR files. The game finds them in
assets/textures/2x ("1080p": the textures at twice their size) and assets/textures/4x ("4K": four times), and uses
them as the texture quality of Options > Graphics says (game/src/host/texpack.rs). Nothing of the packs is in the
project or in its installer: they are downloaded from their author's links, or taken from a .rar or a folder the
user already has.

    python tools/install/texturas.py <2x|4x> <baixar | file.rar | folder> <game folder>

A RAR file is extracted with what the computer has: Windows' own tar (Windows 11), 7-Zip or WinRAR.
"""
import os, re, shutil, subprocess, sys, time, urllib.request
from pathlib import Path

PACKS = {
    "2x": ("1080p", "1b3y3W_n7PkhtuiZyxcCiTdLk7FloqnOT", 1049249734),
    "4x": ("4K", "1JCyqoopSkz9J29zqeSwlhjvChENdXrqh", 3429234806),
}
CREDIT = "texturas HD: canal PLAYANDO (youtube.com/watch?v=rJW7idQDqVc)"


def say(text):
    print(text, flush=True)


def download(file_id, out, size, label):
    """the file from Google Drive (the link for big files, past the "cannot scan for viruses" page), with progress"""
    url = "https://drive.usercontent.google.com/download?id=%s&export=download&confirm=t" % file_id
    req = urllib.request.Request(url, headers={"User-Agent": "Mozilla/5.0 (Tenkaichi3Rust installer)"})
    tmp = out.with_suffix(".parte")
    with urllib.request.urlopen(req, timeout=60) as r:
        if "text/html" in (r.headers.get("Content-Type") or ""):
            page = r.read(200000).decode("utf-8", "replace")
            if re.search(r"quota|cota|too many users", page, re.I):
                raise SystemExit("O Google Drive limitou os downloads deste arquivo por hoje (muita gente baixou). Tente "
                                 "amanhã, ou baixe pelo navegador e escolha o .rar no instalador.")
            raise SystemExit("O Google Drive não entregou o arquivo (o link pode ter mudado). Baixe pelo navegador e "
                             "escolha o .rar no instalador.")
        total = int(r.headers.get("Content-Length") or size)
        done, last, t0 = 0, -1, time.time()
        with open(tmp, "wb") as f:
            while True:
                b = r.read(1 << 20)
                if not b:
                    break
                f.write(b)
                done += len(b)
                pct = int(done * 100 / max(total, 1))
                if pct != last:
                    last = pct
                    speed = done / max(time.time() - t0, 0.1) / (1 << 20)
                    say("texturas %s: baixando %d%% (%.0f de %.0f MB, %.1f MB/s)" % (label, pct, done / (1 << 20), total / (1 << 20), speed))
    if done < total:
        raise SystemExit("O download das texturas %s parou no meio (%d de %d bytes). Rode de novo." % (label, done, total))
    os.replace(tmp, out)


def extractors():
    """the programs that can open a RAR 5 file here, in order"""
    found = []
    pf = [os.environ.get(k) for k in ("ProgramFiles", "ProgramFiles(x86)", "ProgramW6432")]
    for base in filter(None, pf):
        for exe, kind in (("7-Zip/7z.exe", "7z"), ("WinRAR/UnRAR.exe", "unrar"), ("WinRAR/Rar.exe", "unrar")):
            p = Path(base) / exe
            if p.exists():
                found.append((kind, str(p)))
    for name, kind in (("7z", "7z"), ("unrar", "unrar")):
        p = shutil.which(name)
        if p:
            found.append((kind, p))
    system_tar = Path(os.environ.get("SystemRoot", "C:\\Windows")) / "System32" / "tar.exe"
    if system_tar.exists():
        found.append(("tar", str(system_tar)))
    elif shutil.which("bsdtar"):
        found.append(("tar", shutil.which("bsdtar")))
    return found


def extract(rar, dest, label):
    dest.mkdir(parents=True, exist_ok=True)
    tried = []
    for kind, exe in extractors():
        say("texturas %s: extraindo com %s" % (label, Path(exe).name))
        args = {"7z": [exe, "x", "-y", "-o" + str(dest), str(rar)],
                "unrar": [exe, "x", "-o+", "-y", str(rar), str(dest) + os.sep],
                "tar": [exe, "-xf", str(rar), "-C", str(dest)]}[kind]
        r = subprocess.run(args, capture_output=True, creationflags=0x08000000 if os.name == "nt" else 0)
        if r.returncode == 0 and any(dest.rglob("*.dds")):
            return
        tried.append(Path(exe).name)
    raise SystemExit("Não consegui abrir o arquivo RAR das texturas %s%s. Instale o 7-Zip (7-zip.org) e rode de novo, "
                     "ou extraia o .rar e escolha a pasta no instalador." % (label, " (tentei %s)" % ", ".join(tried) if tried else ""))


def install(which, source, game):
    label, file_id, size = PACKS[which]
    dest = Path(game) / "assets" / "textures" / which
    say("\n== texturas HD %s (%s)" % (label, CREDIT))
    if dest.exists() and any(dest.rglob("*.dds")) and source == "baixar":
        say("texturas %s: já instaladas" % label)
        return
    shutil.rmtree(dest, ignore_errors=True)
    src = Path(source) if source != "baixar" else None
    if src is not None and src.is_dir():
        # a folder already extracted: its .dds files, as they are laid out
        n = 0
        for f in src.rglob("*.dds"):
            out = dest / f.relative_to(src)
            out.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(f, out)
            n += 1
            if n % 500 == 0:
                say("texturas %s: %d arquivos copiados" % (label, n))
        if n == 0:
            raise SystemExit("A pasta %s não tem texturas (.dds)." % src)
    else:
        work = Path(game) / "_instalacao"
        work.mkdir(parents=True, exist_ok=True)
        rar = src
        if rar is None:
            rar = work / ("texturas-%s.rar" % which)
            if not (rar.exists() and rar.stat().st_size == size):
                download(file_id, rar, size, label)
        extract(rar, dest, label)
        if src is None:
            rar.unlink()  # downloaded here: not needed any more
    say("texturas %s: %d texturas instaladas" % (label, sum(1 for _ in dest.rglob("*.dds"))))


def main():
    if len(sys.argv) != 4 or sys.argv[1] not in PACKS:
        raise SystemExit(__doc__)
    install(sys.argv[1], sys.argv[2], sys.argv[3])


if __name__ == "__main__":
    main()
