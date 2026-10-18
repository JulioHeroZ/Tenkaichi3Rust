"""Produce preprocessed translation units exactly as the port compiles them (32-bit, PS2 float constants)."""
import pathlib, subprocess, sys, concurrent.futures, json
ROOT = pathlib.Path(sys.argv[1]).resolve()
OUT = pathlib.Path(sys.argv[2]).resolve()
sys.path.insert(0, str(ROOT / "port/tools"))
import portsrc, eeconst
portsrc.ROOT = ROOT; portsrc.GEN = OUT / "gen"
TARGET = ["clang", "--target=i686-linux-gnu", "-m32"]
GAME = ["-std=gnu89", "-fno-builtin", "-msoft-float", "-mno-sse", "-mno-mmx", "-malign-double", "-include", "port_libm.h",
        "-Iinclude", "-Iport/include", "-include", "port_compat.h", "-nostdlibinc", "-ffreestanding", "-isystem", str(pathlib.Path(__file__).parent / "sysinc")]
def pre(flags, src, out, ee):
    r = subprocess.run(TARGET + flags + ["-E", "-P", str(src)], cwd=ROOT, capture_output=True, text=True)
    if r.returncode:
        return f"{src}: {r.stderr[:2000]}"
    text = eeconst.transform(r.stdout) if ee else r.stdout
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(text, encoding="latin-1")
    return None
def job(item):
    kind, f = item
    rel = f.relative_to(ROOT).as_posix()
    out = OUT / "tu" / (rel.replace("/", "__")[:-2] + ".i")
    if kind == "game":
        src, done, missing = portsrc.prepare(f)
        return pre(GAME + [f"-I{f.parent}"], src, out, True)
    if kind == "libm":
        return pre(["-std=gnu89", "-fno-builtin", "-msoft-float", "-malign-double", "-Iport/include", f"-I{f.parent}", "-include", "newlib_shim.h", "-nostdlibinc", "-ffreestanding", "-isystem", str(pathlib.Path(__file__).parent / "sysinc")], f, out, True)
    if kind == "vu0":
        return pre(["-std=gnu99", "-fno-builtin", "-msoft-float", "-malign-double", "-Iinclude", "-Iport/include", "-Iport/src",
                    "-include", "math.h", "-include", "stdlib.h", "-include", "port_libm.h", "-include", "vu0_names.h", "-DREF_VU0_EXTERN_ARITH",
                    "-nostdlibinc", "-ffreestanding", "-isystem", str(pathlib.Path(__file__).parent / "sysinc")], f, out, False)
    if kind == "port":
        return pre(["-std=gnu99", "-fno-builtin", "-msoft-float", "-malign-double", "-Iinclude", "-Iport/include", "-Iport/src",
                    "-include", "math.h", "-include", "stdlib.h", "-include", "port_libm.h", "-DSF_FAST_CALLS",
                    "-nostdlibinc", "-ffreestanding", "-isystem", str(pathlib.Path(__file__).parent / "sysinc")], f, out, True)
    raise SystemExit(kind)
PORT = ["mathf_pc.c", "plat_sys.c", "plat_gs.c", "gs_marker.c", "plat_sndstate.c"]
items = [("vu0", ROOT / "src/port/vu0_a.c"), ("vu0", ROOT / "src/port/vu0_b.c")] + [("port", ROOT / "port/src" / n) for n in PORT] + [("game", f) for f in portsrc.sources()] + [("libm", f) for f in sorted((ROOT / "port/third_party/newlib_libm").glob("*.c"))]
with concurrent.futures.ThreadPoolExecutor(16) as ex:
    errs = [e for e in ex.map(job, items) if e]
for e in errs[:10]: print(e)
print(len(items), "units,", len(errs), "failed")
