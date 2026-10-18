"""Record layouts as the compiler really lays them out.

clang's -fdump-record-layouts-complete computes layouts too early: a record containing a typedef that carries
__attribute__((aligned(N))) (the 16-byte vectors) comes out with the plain struct's alignment. The ordinary
dump, done when a layout is actually needed, is right; so every file-scope record is asked for with sizeof in an
appended line and that dump is used (i686, -malign-double: the PS2's natural alignment, as the port builds it)."""
import json, os, subprocess, sys
from concurrent.futures import ThreadPoolExecutor

TU, AST, OUT, TMP = sys.argv[1:5]
CF = ["clang", "--target=i686-linux-gnu", "-m32", "-malign-double", "-x", "cpp-output", "-std=gnu99", "-w",
      "-ffreestanding", "-Wno-error=return-type", "-Wno-error=implicit-function-declaration",
      "-Wno-error=int-conversion", "-Wno-error=incompatible-function-pointer-types", "-fsyntax-only",
      "-Xclang", "-fdump-record-layouts"]

def names(ast):
    out = []
    for n in ast.get("inner", []):
        k = n.get("kind")
        if k == "RecordDecl" and n.get("completeDefinition") and n.get("name"):
            out.append("%s %s" % (n["tagUsed"], n["name"]))
        elif k == "TypedefDecl":
            out.append(n["name"])
    return out

def one(f):
    ast = json.load(open(os.path.join(AST, f + ".json")))
    src = open(os.path.join(TU, f), "rb").read()
    lines = ["", "/* layouts */"]
    for i, n in enumerate(names(ast)):
        lines.append("typedef char __lay_%d[sizeof(%s) + 1];" % (i, n))
    tmp = os.path.join(TMP, f)
    base = src.count(b"\n") + 1
    for _ in range(6):
        open(tmp, "wb").write(src + b"\n" + "\n".join(lines).encode() + b"\n")
        r = subprocess.run(CF + [f], capture_output=True, cwd=TMP)
        if r.returncode == 0:
            break
        bad = set()
        for l in r.stderr.decode(errors="replace").splitlines():
            p = l.split(":")
            if len(p) > 3 and p[0] == f and p[1].isdigit() and "error" in l:
                bad.add(int(p[1]) - base - 1)
        if not bad:
            break
        lines = [x for i, x in enumerate(lines) if i not in bad]
    open(os.path.join(OUT, f + ".txt"), "wb").write(r.stdout)
    return f, r.returncode, r.stderr.decode(errors="replace")[:300]

os.makedirs(OUT, exist_ok=True); os.makedirs(TMP, exist_ok=True)
files = sorted(x for x in os.listdir(TU) if x.endswith(".i"))
with ThreadPoolExecutor(4) as ex:
    for f, rc, err in ex.map(one, files):
        if rc:
            print("errors in", f, err.splitlines()[:2])
print(len(files), "units")
