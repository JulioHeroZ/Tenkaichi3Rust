"""Index and translate every translation unit."""
import sys, json, glob, pathlib, traceback, concurrent.futures, os
sys.path.insert(0, os.path.dirname(__file__))
import tu as tumod
from trans import Gen, Unsupported, strip
from ctype import Record, Array, Void, Ptr, Func
from funcgen_expr import sig_rs

TU_DIR, AST_DIR, LAY_DIR, OUT = map(pathlib.Path, sys.argv[1:5])
MODE = sys.argv[5] if len(sys.argv) > 5 else "all"

def tuid(path):
    return path.name[:-2].split("__")[-1]

def load(path):
    n = path.name
    return tumod.TU(tuid(path), AST_DIR / (n + ".json"), path, LAY_DIR / (n + ".txt"))

def index_one(path):
    t = load(path)
    g = Gen(t, {"functions": {}})
    fns, gvars = {}, {}
    for n in t.decls:
        if n.get("kind") == "FunctionDecl" and any(c.get("kind") == "CompoundStmt" for c in n.get("inner", [])):
            sym = g.fsym(n["name"])
            ft = t.ty(n["type"])
            params = []
            for p in n.get("inner", []):
                if p.get("kind") == "ParmVarDecl":
                    pt = t.ty(p["type"])
                    params.append(sig_rs(pt, t.native_float))
            ret = "()" if isinstance(ft.ret, Void) else sig_rs(ft.ret, t.native_float)
            fns[sym] = {"params": params, "ret": ret, "variadic": ft.variadic, "tu": t.id,
                        "ret_size": t.sizeof(ft.ret) if isinstance(ft.ret, Record) else 0}
    return fns

def host_sigs(folder):
    import re
    sigs = {}
    for f in pathlib.Path(folder).glob("*.rs"):
        for m in re.finditer(r"pub fn (\w+)\(\s*_?c: &(?:mut )?Ctx\s*((?:,[^)]*)?)\)\s*(?:->\s*([\w()]+))?", f.read_text()):
            name, params, ret = m.group(1), m.group(2), m.group(3) or "()"
            ps, variadic = [], False
            for p in [x.strip() for x in params.split(",") if x.strip()]:
                ty = p.split(":", 1)[1].strip()
                if ty == "&[u64]":
                    variadic = True
                else:
                    ps.append(ty)
            sigs[name] = {"params": ps, "ret": ret, "variadic": variadic}
    return sigs

def translate_one(args):
    path, index = args
    t = load(path)
    g = Gen(t, index)
    out, errors = [], []
    for n in t.decls:
        k = n.get("kind")
        try:
            if k == "FunctionDecl" and any(c.get("kind") == "CompoundStmt" for c in n.get("inner", [])):
                out.append(g.function(n))
            elif k == "VarDecl" and n.get("storageClass") != "extern":
                g.global_var(n, g.vsym(n["name"]), n.get("storageClass") == "static")
        except Exception as e:
            errors.append("%s %s: %s: %s" % (t.id, n.get("name"), type(e).__name__, str(e)[:300]))
            if os.environ.get("TRACE"):
                traceback.print_exc()
    code = "#![allow(unused, non_snake_case, non_upper_case_globals, unreachable_code, unused_parens, unused_braces, clippy::all)]\n" \
           "use crate::rt::prelude::*;\n\n" + "\n\n".join(out) + "\n"
    (OUT / "src" / "tu" / (t.id + ".rs")).write_text(code)
    meta = {"id": t.id, "globals": g.out_globals, "host": sorted([list(map(lambda x: list(x) if isinstance(x, tuple) else x, h)) for h in g.calls_host])}
    (OUT / "meta" / (t.id + ".json")).write_text(json.dumps(meta))
    return t.id, errors

def main():
    (OUT / "src" / "tu").mkdir(parents=True, exist_ok=True)
    (OUT / "meta").mkdir(parents=True, exist_ok=True)
    paths = sorted(TU_DIR.glob("*.i"))
    if MODE != "all":
        sel = [p for p in paths if tuid(p) in MODE.split(",")]
    else:
        sel = paths
    idx_path = OUT / "meta" / "_index.json"
    if MODE == "all" or not idx_path.exists():
        index = {"functions": {}}
        with concurrent.futures.ProcessPoolExecutor(16) as ex:
            for fns in ex.map(index_one, paths):
                for k, v in fns.items():
                    if k in index["functions"] and index["functions"][k] != v:
                        print("duplicate definition", k)
                    index["functions"][k] = v
        idx_path.write_text(json.dumps(index))
    index = json.loads(idx_path.read_text())
    index["host"] = host_sigs(OUT / "src" / "host")
    print(len(index["functions"]), "functions indexed")
    errs = []
    with concurrent.futures.ProcessPoolExecutor(16) as ex:
        for tid, e in ex.map(translate_one, [(p, index) for p in sel]):
            errs += e
    for e in errs[:60]:
        print(e)
    print(len(errs), "errors")
    import collections
    print(collections.Counter(e.split(": ")[1] for e in errs).most_common(10))

if __name__ == "__main__":
    main()
