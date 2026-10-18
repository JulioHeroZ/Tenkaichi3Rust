"""Tests the game's build before the installer is made (tools/install/testar_build.cmd; gerar_instalador.cmd runs it
first): compiles Tenkaichi3.exe as the installer carries it (target/pacote, as empacotar.py does, so it is not compiled
twice), plays it in a hidden window without sound, and looks at the pictures as the window shows them:

 1. the title screen (start pressed until the menu): the menu in 4:3 over its blurred copy;
 2. an attract-mode battle: the fight in 16:9.

A picture fails when its centre is all one colour (the 0.2.0 bug), or when the game stops early or crashes. The
pictures are left in dist/teste for a look. Uses the project's assets folder (a developer install of your disc,
assets/files), and its own saves and settings (yours are not touched).

    python tools/install/testar_build.py [--sem-compilar]

Exit code 0 when everything passed.
"""
import os, struct, subprocess, sys, tempfile, time, zlib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
EXE = ".exe" if os.name == "nt" else ""
OUT = ROOT / "dist" / "teste"


def say(t):
    print(t, flush=True)


def png(path):
    """(w, h, rgba rows) of an 8-bit RGBA PNG (the game's own screenshots)"""
    d = Path(path).read_bytes()
    at, idat, w = 8, b"", 0
    while at < len(d):
        n, kind = struct.unpack(">I4s", d[at:at + 8])
        body = d[at + 8:at + 8 + n]
        if kind == b"IHDR":
            w, h, depth, color = struct.unpack(">IIBB", body[:10])
            if (depth, color) != (8, 6):
                raise ValueError("PNG que não é RGBA 8 bits")
        elif kind == b"IDAT":
            idat += body
        at += 12 + n
    raw = zlib.decompress(idat)
    row, rows, prev = w * 4, [], bytearray(w * 4)
    for y in range(h):
        f, line = raw[y * (row + 1)], bytearray(raw[y * (row + 1) + 1:(y + 1) * (row + 1)])
        for i in range(row if f else 0):
            a = line[i - 4] if i >= 4 else 0
            b = prev[i]
            c = prev[i - 4] if i >= 4 else 0
            if f == 1:
                line[i] = (line[i] + a) & 255
            elif f == 2:
                line[i] = (line[i] + b) & 255
            elif f == 3:
                line[i] = (line[i] + (a + b) // 2) & 255
            elif f == 4:
                p = a + b - c
                pa, pb, pc = abs(p - a), abs(p - b), abs(p - c)
                line[i] = (line[i] + (a if pa <= pb and pa <= pc else b if pb <= pc else c)) & 255
        rows.append(bytes(line))
        prev = line
    return w, h, rows


def spread(img, x0, x1, y0, y1):
    """the brightness's standard deviation over a box (fractions of the picture), sampled"""
    w, h, rows = img
    v = []
    for y in range(int(y0 * h), int(y1 * h), 4):
        r = rows[y]
        for x in range(int(x0 * w), int(x1 * w), 4):
            v.append((r[4 * x] * 3 + r[4 * x + 1] * 6 + r[4 * x + 2]) / 10)
    m = sum(v) / len(v)
    return (sum((a - m) ** 2 for a in v) / len(v)) ** 0.5


def build():
    target = ROOT / "target" / "pacote"
    env = dict(os.environ, CARGO_TARGET_DIR=str(target), CARGO_PROFILE_RELEASE_DEBUG="false",
               CARGO_PROFILE_RELEASE_STRIP="true")
    say("== compilando o jogo (como o instalador leva)")
    t = time.time()
    r = subprocess.run(["cargo", "build", "--release", "-p", "bt3-original", "--bin", "tenkaichi3"], cwd=ROOT, env=env)
    if r.returncode != 0:
        raise SystemExit("FALHOU: o jogo não compila.")
    say("   (%.0f s)" % (time.time() - t))
    return target / "release" / ("tenkaichi3" + EXE)


def play(exe, name, frames, shot, extra):
    """the game for `frames` frames in a hidden window; its window pictures"""
    work = OUT / name
    work.mkdir(parents=True, exist_ok=True)
    for old in (work / "shots").glob("*.png") if (work / "shots").exists() else []:
        old.unlink()
    tmp = Path(tempfile.mkdtemp(prefix="bt3teste"))
    env = dict(os.environ, BT3_GS=os.environ.get("BT3_TESTE_GS", "gpu"), BT3_HIDDEN="1", BT3_UNCAPPED="1", BT3_NOSOUND="1",
               BT3_EXIT_AT=str(frames), BT3_SHOT=str(shot), BT3_SHOT_WINDOW="1280x720",
               BT3_ASSETS=str(ROOT / "assets"), BT3_PC_ONLY="1", BT3_SAVES=str(tmp / "saves"), BT3_SETTINGS=str(tmp / "settings.ini"), **extra)
    for k in ("BT3_ASPECT", "BT3_TEXTURES", "BT3_SCALE"):
        env.pop(k, None)
    say("== %s (%d quadros)" % (name, frames))
    t = time.time()
    with open(work / "saida.txt", "wb") as log:
        try:
            r = subprocess.run([str(exe)], cwd=work, env=env, stdout=log, stderr=subprocess.STDOUT, timeout=900)
            code = r.returncode
        except subprocess.TimeoutExpired:
            code = "tempo esgotado (15 min)"
    say("   (%.0f s, saída: %s)" % (time.time() - t, code))
    return code, sorted((work / "shots").glob("window_*.png")) if (work / "shots").exists() else []


def main():
    if not (ROOT / "assets" / "files").is_dir():
        raise SystemExit("Não achei assets\\files no projeto: o teste precisa de uma instalação de desenvolvedor do "
                         "seu disco na pasta assets do projeto.")
    exe = ROOT / "target" / "pacote" / "release" / ("tenkaichi3" + EXE)
    if "--sem-compilar" not in sys.argv or not exe.exists():
        exe = build()
    fails = []
    starts = ",".join("%d:start" % f for f in range(300, 2401, 60))
    for name, frames, shot, extra, first, edges in [
        ("tela-inicial", 2700, 2600, {"BT3_PAD": starts}, 2600, False),
        ("luta", 1500, 300, {"BT3_DEMO": "1"}, 300, True),
    ]:
        code, shots = play(exe, name, frames, shot, extra)
        if code != 0:
            fails.append("%s: o jogo terminou com erro (%s); veja dist\\teste\\%s\\saida.txt" % (name, code, name))
        shots = [s for s in shots if int(s.stem.split("_")[1]) >= first]
        if not shots:
            fails.append("%s: nenhuma imagem da janela foi salva" % name)
        for s in shots:
            img = png(s)
            centre = spread(img, 0.3, 0.7, 0.2, 0.8)
            ok = centre > 6
            note = "centro %.0f" % centre
            if edges:
                side = max(spread(img, 0.0, 0.1, 0.1, 0.9), spread(img, 0.9, 1.0, 0.1, 0.9))
                note += ", laterais %.0f" % side
            say("   %s: %s %s" % (s.name, note, "ok" if ok else "<- de uma cor só"))
            if not ok:
                fails.append("%s: %s com o centro de uma cor só" % (name, s.name))
    say("")
    if fails:
        say("TESTE FALHOU:")
        for f in fails:
            say("  - " + f)
        say("As imagens estão em %s" % OUT)
        sys.exit(1)
    say("TESTE OK: o jogo compila, abre, chega ao menu e luta, e a imagem da janela está certa (%s)." % OUT)


if __name__ == "__main__":
    main()
