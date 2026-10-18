"""The HD texture pack (optional): a pack made for the game under PCSX2 (its texture replacement), freely shared on
MediaFire as one RAR file (folders replacements/, Fonts/, Original Buttons/, Xbox Layout/). The game finds its
files at any depth below assets/textures and uses them as the texture quality of Options > Graphics says
(game/src/host/texpack.rs). Nothing of the pack is in the project or in its installer: it is downloaded from the
link where it is shared, or taken from a .rar or a folder the user already has.

    python tools/install/texturas.py <baixar | file.rar | folder> <game folder>

The pack's replacements/ folder goes to assets/textures/pcsx2. A RAR file is extracted with what the computer has:
Windows' own tar (Windows 11), 7-Zip or WinRAR. The packs of earlier versions (the Wii one, assets/textures/2x
and 4x) are removed: the game no longer reads them.
"""
import html, os, re, shutil, subprocess, sys, time, urllib.request
from pathlib import Path

PAGE = "https://www.mediafire.com/file/2jm75b9v163tieg/Dragon_Ball_Z_-Budokai_Tenkaichi_3.rar/file"
SIZE = 2107013535
NAME = "Dragon Ball Z -Budokai Tenkaichi 3.rar"
CREDIT = "texturas HD: pacote para PCSX2 compartilhado no MediaFire (não faz parte deste projeto)"
UA = {"User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) Tenkaichi3Rust installer"}


def say(text):
    print(text, flush=True)


def direct_link():
    """the file's own address, from MediaFire's page (its download button)"""
    with urllib.request.urlopen(urllib.request.Request(PAGE, headers=UA), timeout=60) as r:
        page = r.read(2_000_000).decode("utf-8", "replace")
    m = re.search(r'href="(https?://download\d*\.mediafire\.com/[^"]+)"', page)
    if m:
        return html.unescape(m.group(1))
    m = re.search(r'data-scrambled-url="([^"]+)"', page)
    if m:
        import base64
        return base64.b64decode(m.group(1)).decode("utf-8", "replace")
    raise SystemExit("O MediaFire não mostrou o link do arquivo (a página pode ter mudado). Baixe pelo navegador:\n"
                     + PAGE + "\ne escolha o .rar no instalador (Já tenho...).")


def download(out):
    """the RAR file, with progress"""
    url = direct_link()
    tmp = out.with_suffix(".parte")
    with urllib.request.urlopen(urllib.request.Request(url, headers=UA), timeout=60) as r:
        if "text/html" in (r.headers.get("Content-Type") or ""):
            raise SystemExit("O MediaFire não entregou o arquivo. Baixe pelo navegador:\n" + PAGE +
                             "\ne escolha o .rar no instalador (Já tenho...).")
        total = int(r.headers.get("Content-Length") or SIZE)
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
                    say("texturas HD: baixando %d%% (%.0f de %.0f MB, %.1f MB/s)" % (pct, done / (1 << 20), total / (1 << 20), speed))
    if done < total:
        raise SystemExit("O download das texturas parou no meio (%d de %d bytes). Rode de novo." % (done, total))
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


def replacements_in(folder):
    """the pack's replacements folder inside an extracted pack (or the folder itself when it holds the files)"""
    folder = Path(folder)
    if folder.name.lower() == "replacements":
        return folder
    for p in sorted(folder.rglob("replacements")):
        if p.is_dir():
            return p
    return folder


def extract(rar, dest):
    dest.mkdir(parents=True, exist_ok=True)
    tried = []
    for kind, exe in extractors():
        say("texturas HD: extraindo com %s" % Path(exe).name)
        args = {"7z": [exe, "x", "-y", "-o" + str(dest), str(rar)],
                "unrar": [exe, "x", "-o+", "-y", str(rar), str(dest) + os.sep],
                "tar": [exe, "-xf", str(rar), "-C", str(dest)]}[kind]
        r = subprocess.run(args, capture_output=True, creationflags=0x08000000 if os.name == "nt" else 0)
        if r.returncode == 0 and any(dest.rglob("*.dds")):
            return
        tried.append(Path(exe).name)
    raise SystemExit("Não consegui abrir o arquivo RAR das texturas%s. Instale o 7-Zip (7-zip.org) e rode de novo, "
                     "ou extraia o .rar e escolha a pasta no instalador." % (" (tentei %s)" % ", ".join(tried) if tried else ""))


def install(source, game):
    textures = Path(game) / "assets" / "textures"
    dest = textures / "pcsx2"
    say("\n== texturas HD (%s)" % CREDIT)
    for old in ("2x", "4x"):
        if (textures / old).exists():
            say("texturas HD: removendo o pacote antigo (%s)" % old)
            shutil.rmtree(textures / old, ignore_errors=True)
    if dest.exists() and any(dest.rglob("*.dds")) and source == "baixar":
        say("texturas HD: já instaladas")
        return
    shutil.rmtree(dest, ignore_errors=True)
    src = Path(source) if source != "baixar" else None
    if src is not None and src.is_file() and src.suffix.lower() == ".dds":
        src = src.parent  # a .dds of a folder already extracted: that folder
    if src is not None and src.is_dir():
        # a folder already extracted: its replacements folder, as it is laid out
        rep = replacements_in(src)
        n = 0
        for f in rep.rglob("*"):
            if f.suffix.lower() not in (".dds", ".png"):
                continue
            out = dest / f.relative_to(rep)
            out.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(f, out)
            n += 1
            if n % 500 == 0:
                say("texturas HD: %d arquivos copiados" % n)
        if n == 0:
            raise SystemExit("A pasta %s não tem texturas (.dds)." % src)
    else:
        work = Path(game) / "_instalacao"
        work.mkdir(parents=True, exist_ok=True)
        rar = src
        if rar is None:
            rar = work / "texturas-hd.rar"
            if not (rar.exists() and rar.stat().st_size == SIZE):
                download(rar)
        out = textures / "_extraindo"
        shutil.rmtree(out, ignore_errors=True)
        extract(rar, out)
        os.replace(replacements_in(out), dest)
        shutil.rmtree(out, ignore_errors=True)
        if src is None:
            rar.unlink()  # downloaded here: not needed any more
    say("texturas HD: %d texturas instaladas" % sum(1 for _ in dest.rglob("*.dds")))


def main():
    if len(sys.argv) != 3:
        raise SystemExit(__doc__)
    install(sys.argv[1], sys.argv[2])


if __name__ == "__main__":
    main()
