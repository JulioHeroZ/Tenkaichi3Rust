"""Installs Tenkaichi 3 for the PC from the user's own disc image. Two kinds of installation, both a game folder like any PC game's (by default Program Files (x86)\\Tenkaichi 3) with Tenkaichi3.exe, its libraries
and shortcuts on the desktop and in the Start menu; they differ in assets/:
* jogador (player): the game's data in a few packs (tools/install/pack.py);
* desenvolvedor (developer): the game's files as editable open files (PNG, glTF, WAV, JSON, text), grouped by kind
  and named, which the game reads as they are: edit them and play.
The work files are deleted at the end.

The steps:
 1. the disc image to loose files (extract_iso.py);
 2. the game's data tables (assets/data/tables.txt, tools/translate/c2rs/tables_from_elf.py);
 3. the raw files copied to assets/_legacy/raw and the audio streams (ADX) as WAVs (crate bt3-install);
 4. every file in open formats (files_extract.py: textures, models, stages, collision, animations, texts, sounds...);
 5. the native PC resources (audio banks, text tables, lip sync, motions, roster, point groups, empty directories),
    then the folders grouped by kind and named (organize.py);
 6. the movies' MPEG-2 video, unchanged (pss_video.py; the game decodes it in Rust).
Each step is skipped when its result is already there, so it can be stopped and run again.

    python tools/install/instalar.py <disc.iso> [--modo jogador|desenvolvedor] [--destino <folder>]

The installer's window (Instalar Tenkaichi 3.exe, tools/install/src/bin/instalador.rs) runs this.
"""
import argparse, os, subprocess, sys, shutil, time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PY = sys.executable
TOOLS = ROOT / "tools"
BIN = ROOT / "bin"  # the player package's ready programs (tools/install/empacotar.py)
EXE = ".exe" if os.name == "nt" else ""


def run(what, args, env=None):
    print("\n== %s" % what, flush=True)
    t = time.time()
    r = subprocess.run(args, cwd=ROOT, env=env)
    if r.returncode != 0:
        raise SystemExit("falhou: %s (código %d). Rode de novo depois de corrigir; os passos prontos são pulados." % (what, r.returncode))
    print("   (%.0f s)" % (time.time() - t), flush=True)


def cargo():
    c = shutil.which("cargo") or str(Path.home() / ".cargo" / "bin" / ("cargo" + EXE))
    if not Path(c).exists():
        raise SystemExit("Não achei o Rust (cargo). Instale em https://rustup.rs e rode de novo.")
    return c


def program(name, package, binary):
    """a ready program of the player package (bin/), else built from the project with cargo"""
    ready = BIN / (name + EXE)
    if ready.exists():
        return ready
    run("compilando %s" % binary, [cargo(), "build", "--release", "-p", package, "--bin", binary])
    target = Path(os.environ.get("CARGO_TARGET_DIR") or ROOT / "target")
    return target / "release" / (binary + EXE)


def extract(iso, assets, disc):
    """steps 1 to 6 into an assets folder"""
    raw = assets / "_legacy" / "raw"
    env = dict(os.environ, BT3_ASSETS_DIR=str(assets))
    # 1, 3: the disc and the raw files
    if not (raw / "pzs3us1" / "00001.bin").exists() or not (assets / "audio" / "streams" / "disc").exists():
        if not (disc / "pzs3us1" / "00001.bin").exists():
            if not iso or not Path(iso).is_file():
                raise SystemExit("Não achei a imagem do disco (.iso): a sua cópia de Dragon Ball Z Budokai Tenkaichi 3, EUA.")
            run("1/6 lendo o disco", [PY, str(TOOLS / "install" / "extract_iso.py"), iso, str(disc)])
        run("3/6 copiando os arquivos e convertendo as músicas e vozes em WAV",
            [str(program("bt3-install", "bt3-install", "bt3-install")), "export", str(disc), str(assets)])
    # 2: the tables (the game reads them when it starts; the program carries none of the game's data)
    tables = assets / "data" / "tables.txt"
    if not tables.exists():
        tables.parent.mkdir(parents=True, exist_ok=True)
        run("2/6 tabelas do jogo", [PY, str(TOOLS / "translate" / "c2rs" / "tables_from_elf.py"), str(raw / "disc" / "SLUS_216.78"),
                                    str(ROOT / "game" / "src" / "gen" / "data.rs"), str(tables)])
    # 4: open formats (resumable by itself)
    run("4/6 arquivos em formatos abertos (a primeira vez demora bastante; pode parar e continuar depois)",
        [PY, str(TOOLS / "assets" / "files_extract.py"), str(raw), str(assets), "pzs3us1", "pzs3us0", "pzs3us2", "disc"])
    # 5: native PC resources, then the folders grouped by kind and named
    for script in ["convert_empty_directories.py", "convert_roster_items.py", "convert_point_groups.py",
                   "convert_native_audio_banks.py", "convert_native_text.py", "convert_native_lips.py",
                   "convert_native_motion_sources.py"]:
        run("5/6 %s" % script, [PY, str(TOOLS / "assets" / script)], env)
    run("5/6 organizando as pastas por tipo e nome", [PY, str(TOOLS / "assets" / "organize.py"), str(assets)], env)
    # 6: movies: the MPEG-2 video of the disc, as it is (the game decodes it itself)
    if not (assets / "video" / "zs3usop.m2v").exists() or not (assets / "video" / "zs3used.m2v").exists():
        run("6/6 vídeos", [PY, str(TOOLS / "assets" / "pss_video.py"), str(assets)], env)
    if disc.exists() and (raw / "pzs3us1" / "00001.bin").exists():
        shutil.rmtree(disc, ignore_errors=True)  # the copy in assets/_legacy/raw is the one kept


def shortcuts(target, folder):
    """shortcuts to the game on the desktop and in the Start menu (Windows)"""
    if os.name != "nt":
        return
    for where in ("[Environment]::GetFolderPath('Desktop')", "[Environment]::GetFolderPath('Programs')"):
        ps = ("$s=(New-Object -ComObject WScript.Shell).CreateShortcut(%s+'\\Tenkaichi 3.lnk');"
              "$s.TargetPath='%s';$s.WorkingDirectory='%s';$s.Save()" % (where, target, folder))
        subprocess.run(["powershell", "-NoProfile", "-NonInteractive", "-Command", ps], creationflags=0x08000000)


def register(dest, mode):
    """the game in the list of installed programs (Settings > Apps), with Desinstalar.exe to remove it (Windows);
    for all users when the installer runs as administrator, else for this user"""
    if os.name != "nt":
        return
    size = sum(f.stat().st_size for f in dest.rglob("*") if f.is_file()) // 1024
    values = [("DisplayName", "REG_SZ", "Dragon Ball Z Budokai Tenkaichi 3 (PC%s)" % (", desenvolvedor" if mode == "desenvolvedor" else "")),
              ("DisplayIcon", "REG_SZ", str(dest / "Tenkaichi3.exe") + ",0"),
              ("DisplayVersion", "REG_SZ", "0.1.0"),
              ("Publisher", "REG_SZ", "Tenkaichi3Rust"),
              ("InstallLocation", "REG_SZ", str(dest)),
              ("UninstallString", "REG_SZ", '"%s"' % (dest / "Desinstalar.exe")),
              ("NoModify", "REG_DWORD", "1"),
              ("NoRepair", "REG_DWORD", "1"),
              ("EstimatedSize", "REG_DWORD", str(size))]
    key = r"Software\Microsoft\Windows\CurrentVersion\Uninstall\Tenkaichi3Rust"
    for root in ("HKLM", "HKCU"):
        ok = all(subprocess.run(["reg", "add", "%s\\%s" % (root, key), "/v", name, "/t", kind, "/d", data, "/f"],
                                capture_output=True, creationflags=0x08000000).returncode == 0 for name, kind, data in values)
        if ok:
            print("   registrado na lista de programas (%s)" % root, flush=True)
            return


def install(iso, mode, dest):
    """the game in `dest`: Tenkaichi3.exe and its libraries, and assets/ with the game's data, in packs (player) or as
    editable open files (developer); work files under dest/_instalacao, deleted at the end"""
    dest = Path(dest)
    dest.mkdir(parents=True, exist_ok=True)
    work = dest / "_instalacao"
    extract(iso, work / "assets", work / "disc")
    print("\n== instalando o jogo em %s" % dest, flush=True)
    exe = program("Tenkaichi3", "bt3-original", "tenkaichi3")
    shutil.copy2(exe, dest / ("Tenkaichi3" + EXE))
    for lib in ("SDL3.dll", "libiconv-2.dll"):
        for src in (BIN / lib, ROOT / "vendor" / "windows" / lib):
            if src.exists():
                shutil.copyfile(src, dest / lib)
                break
    assets = dest / "assets"
    for old in list(assets.glob("*.bt3pak")) if assets.exists() else []:
        old.unlink()  # a player install before
    if mode == "jogador":
        for top in ("files", "audio", "video", "data"):
            shutil.rmtree(assets / top, ignore_errors=True)  # a developer install before
        run("empacotando os dados do jogo", [PY, str(TOOLS / "install" / "pack.py"), str(work / "assets"), str(assets)])
    else:
        # the open files as they are: what the game reads, editable (the copy of the disc, _legacy, is left out)
        assets.mkdir(exist_ok=True)
        for top in ("files", "audio", "video", "data"):
            shutil.rmtree(assets / top, ignore_errors=True)
            if (work / "assets" / top).exists():
                os.replace(work / "assets" / top, assets / top)
        for done in (assets / "files").glob("extraidos-*.txt"):
            done.unlink()  # the extraction's lists of what it had looked at
        (dest / "LEIA-ME (desenvolvedor).txt").write_text(
            "Tenkaichi 3, instalação de desenvolvedor\r\n\r\n"
            "Os arquivos do jogo estão em assets\\files, agrupados por tipo e com nome (lutadores, cenários, menus,\r\n"
            "sons...): texturas em PNG, modelos, cenários e animações em glTF, sons em WAV, textos e tabelas em JSON.\r\n"
            "Edite e abra Tenkaichi3.exe: o jogo monta os arquivos a partir deles ao carregar. O número no começo de\r\n"
            "cada pasta é como o jogo acha o arquivo; o resto do nome e o lugar da pasta podem mudar.\r\n"
            "Os formatos estão em docs/open-formats.md do projeto.\r\n", encoding="utf-8")
    uninstaller = program("desinstalar", "bt3-install", "desinstalar")
    shutil.copy2(uninstaller, dest / ("Desinstalar" + EXE))
    # the game's icon, from the title logo at the end of the opening on the disc (bt3-install icone)
    opening = work / "assets" / "video" / "zs3usop.m2v"
    if not opening.exists():
        opening = assets / "video" / "zs3usop.m2v"
    if opening.exists():
        run("ícone do jogo", [str(program("bt3-install", "bt3-install", "bt3-install")), "icone", str(opening),
                              str(dest / "Tenkaichi3.ico"), str(dest / ("Tenkaichi3" + EXE)), str(dest / ("Desinstalar" + EXE))])
    shutil.rmtree(work, ignore_errors=True)
    shortcuts(dest / ("Tenkaichi3" + EXE), dest)
    register(dest, mode)
    print("\nPronto. O jogo está em %s (atalho \"Tenkaichi 3\" na área de trabalho e no menu Iniciar; para remover,"
          " Configurações > Aplicativos ou Desinstalar.exe na pasta do jogo)." % dest, flush=True)


def default_dest():
    base = os.environ.get("ProgramFiles(x86)") or os.environ.get("ProgramFiles") or str(Path.home())
    return str(Path(base) / "Tenkaichi 3")


def main():
    p = argparse.ArgumentParser(description="Instala o Tenkaichi 3 de PC a partir da sua imagem do disco.")
    p.add_argument("iso", nargs="?", help="a imagem do disco (.iso), Dragon Ball Z Budokai Tenkaichi 3, EUA")
    p.add_argument("--modo", choices=["jogador", "desenvolvedor"], default="jogador",
                   help="jogador: os dados em pacotes; desenvolvedor: os arquivos do jogo editáveis")
    p.add_argument("--destino", default=None, help="pasta do jogo (padrão: Program Files (x86)\\Tenkaichi 3)")
    a = p.parse_args()
    install(a.iso, a.modo, a.destino or default_dest())


if __name__ == "__main__":
    main()
