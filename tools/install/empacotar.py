"""Makes the installer to hand out: one file, dist/Instalar Tenkaichi 3.exe. It holds no data of the game, only the
programs; whoever runs it chooses their disc image (.iso), the kind of installation (play, or developer with the
editable files) and the folder (Program Files (x86)\\Tenkaichi 3 by default), and needs nothing else installed.

The file is the installer's window (tools/install/src/bin/instalador.rs) with its payload after it
(bt3_install::payload, unpacked to %TEMP% when it runs):

    python/              an embeddable Python with numpy, for the extraction tools
    tools/               tools/install, tools/assets and tables_from_elf.py
    bin/                 Tenkaichi3.exe (the game), desinstalar.exe, bt3-install.exe, SDL3.dll, libiconv-2.dll
    game/src/gen/data.rs where the game's tables go (read by tables_from_elf.py)
    dados.versao         the version of the data the tools make (instalar.data_version): an installation with the
                         same one is updated, not made again; the window knows it too (BT3_DADOS_VERSAO when built)

Run on Windows from the project, with Rust and Python installed (it downloads the embeddable Python and numpy):

    python tools/install/empacotar.py
"""
import io, os, shutil, struct, subprocess, sys, urllib.request, zipfile, zlib
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import instalar

ROOT = Path(__file__).resolve().parents[2]
PYTHON = "3.12.7"
DIST = ROOT / "dist"
STAGE = DIST / "instalador"


def run(args, **kw):
    print("> " + " ".join(map(str, args)), flush=True)
    subprocess.run(args, check=True, cwd=ROOT, **kw)


def payload(folder):
    """the files of a folder as the installer's payload (bytes to put after the executable, without the trailer)"""
    data, index = bytearray(), bytearray()
    files = sorted(p for p in folder.rglob("*") if p.is_file())
    for p in files:
        raw = p.read_bytes()
        z = zlib.compress(raw, 9)
        method, stored = (1, z) if len(z) < len(raw) else (0, raw)
        rel = p.relative_to(folder).as_posix().encode("utf-8")
        index += struct.pack("<H", len(rel)) + rel + struct.pack("<QIIB", len(data), len(stored), len(raw), method)
        data += stored
    return bytes(data), bytes(index), len(files)


def main():
    if os.name != "nt":
        raise SystemExit("O instalador é para Windows: rode no Windows.")
    # the programs, without debug information (smaller)
    target = ROOT / "target" / "pacote"
    version = instalar.data_version(ROOT)
    print("versão dos dados: %s" % version, flush=True)
    env = dict(os.environ, CARGO_TARGET_DIR=str(target), CARGO_PROFILE_RELEASE_DEBUG="false",
               CARGO_PROFILE_RELEASE_STRIP="true", BT3_DADOS_VERSAO=version)
    run(["cargo", "build", "--release", "-p", "bt3-original", "--bin", "tenkaichi3"], env=env)
    run(["cargo", "build", "--release", "-p", "bt3-install", "--bins"], env=env)
    if STAGE.exists():
        shutil.rmtree(STAGE)
    (STAGE / "bin").mkdir(parents=True)
    shutil.copyfile(target / "release" / "tenkaichi3.exe", STAGE / "bin" / "Tenkaichi3.exe")
    shutil.copyfile(target / "release" / "bt3-install.exe", STAGE / "bin" / "bt3-install.exe")
    shutil.copyfile(target / "release" / "desinstalar.exe", STAGE / "bin" / "desinstalar.exe")
    for lib in ("SDL3.dll", "libiconv-2.dll", "README.md"):
        shutil.copyfile(ROOT / "vendor" / "windows" / lib, STAGE / "bin" / lib)
    for rel in ["tools/install/instalar.py", "tools/install/extract_iso.py", "tools/install/pack.py", "tools/install/texturas.py",
                "tools/translate/c2rs/tables_from_elf.py", "game/src/gen/data.rs", "LICENSE"]:
        (STAGE / rel).parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(ROOT / rel, STAGE / rel)
    (STAGE / "dados.versao").write_text(version + "\n")
    (STAGE / "tools" / "assets").mkdir(parents=True, exist_ok=True)
    for f in (ROOT / "tools" / "assets").glob("*.py"):
        shutil.copyfile(f, STAGE / "tools" / "assets" / f.name)
    # an embeddable Python with numpy; its path file lets the tools import their neighbours
    py = STAGE / "python"
    url = "https://www.python.org/ftp/python/%s/python-%s-embed-amd64.zip" % (PYTHON, PYTHON)
    print("baixando " + url, flush=True)
    zipfile.ZipFile(io.BytesIO(urllib.request.urlopen(url).read())).extractall(py)
    tag = "".join(PYTHON.split(".")[:2])
    site = py / "Lib" / "site-packages"
    site.mkdir(parents=True)
    run([sys.executable, "-m", "pip", "download", "numpy", "--only-binary=:all:", "--platform", "win_amd64",
         "--python-version", tag, "--no-deps", "-d", str(py / "_wheels")])
    for wheel in (py / "_wheels").glob("*.whl"):
        zipfile.ZipFile(wheel).extractall(site)
    shutil.rmtree(py / "_wheels")
    (py / ("python%s._pth" % tag)).write_text("python%s.zip\n.\nLib\\site-packages\n..\\tools\\assets\n..\\tools\\install\n"
                                             "..\\tools\\translate\\c2rs\nimport site\n" % tag)
    # one file: the window, then the payload and its trailer
    # the installer's own icon: an original drawing (it is handed out, so it carries nothing of the game; the
    # installed programs get the game's icon from the user's disc)
    window = STAGE.parent / "instalador.exe"
    shutil.copyfile(target / "release" / "instalador.exe", window)
    run([str(target / "release" / "bt3-install.exe"), "icone-instalador", str(window)])
    exe = window.read_bytes()
    window.unlink()
    data, index, count = payload(STAGE)
    out = DIST / "Instalar Tenkaichi 3.exe"
    with open(out, "wb") as f:
        f.write(exe)
        f.write(data)
        f.write(index)
        f.write(struct.pack("<QQI", len(exe), len(data), count) + b"BT3SFX1\0")
    shutil.rmtree(STAGE)
    print("pronto: %s (%.1f MB, %d arquivos dentro)" % (out, out.stat().st_size / 1e6, count))


if __name__ == "__main__":
    main()
