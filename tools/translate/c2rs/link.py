"""Lay out the program's memory at the PS2's addresses and generate the glue modules."""
import sys, json, re, pathlib, struct, collections
sys.path.insert(0, str(pathlib.Path(__file__).parent))
import trans
from funcgen_expr import RS_RAW_FROM, RS_RAW_TO

OUT = pathlib.Path(sys.argv[1])
DECOMP = pathlib.Path(sys.argv[2])
EXTRA_BASE = 0x0200_0000      # data the PS2 image has no address for (port-only, string literals)
FN_SYNTH = 0x0F00_0000        # function identifiers without an original address
TEXT_END = 0x002B_F6B0

def rust_ident(n):
    kw = set("as break const continue crate else enum extern false fn for if impl in let loop match mod move mut pub ref return self Self static struct super trait true type unsafe use where while async await dyn abstract become box do final macro override priv typeof unsized virtual yield try gen union".split())
    return "r#" + n if n in kw else n

def symbol_files():
    addr, count = {}, collections.Counter()
    files = [DECOMP / "config/symbol_addrs.txt", DECOMP / "config/symbol_addrs_dbzp.txt"] + sorted((DECOMP / "config/symbols").glob("*.txt"))
    for p in files:
        for l in p.read_text(errors="replace").splitlines():
            m = re.match(r"\s*(\w+)\s*=\s*0x([0-9A-Fa-f]+)\s*;(.*)", l)
            if m:
                name, a = m.group(1), int(m.group(2), 16)
                size = None
                s = re.search(r"size:0x([0-9A-Fa-f]+)", m.group(3))
                if s:
                    size = int(s.group(1), 16)
                if name in addr and addr[name][0] != a:
                    count[name] += 1
                addr[name] = (a, size)
    return addr, count

def parse_tables():
    idx = {}
    for l in (DECOMP / "port/data/index.txt").read_text().splitlines():
        p = l.split()
        if len(p) == 3 and not l.startswith("#"):
            idx[p[0]] = (int(p[1]), int(p[2], 16))
    tables = []
    for f in sorted((DECOMP / "port/data").glob("*.s")):
        name = f.name[:-2]
        if name in idx:
            src, off = idx[name]
            base = 0x100000 + off
        else:
            m = re.match(r"cod_([0-9A-F]{8})\.", name)
            if not m:
                raise SystemExit("table without index entry: " + name)
            base, src, off = int(m.group(1), 16), 0, int(m.group(1), 16) - 0x100000
        pos = 0
        labels, longs, spaces = {}, [], []
        for l in f.read_text().splitlines():
            l = l.split("#")[0].strip()
            if not l or l.startswith((".section", ".globl", ".p2align")):
                continue
            if l.endswith(":"):
                labels[l[:-1]] = base + pos
            elif l.startswith(".long"):
                ref = l.split(None, 1)[1].strip()
                m = re.match(r"(\w+)\s*([+-]\s*\d+)?$", ref)
                longs.append((base + pos, m.group(1), int(m.group(2).replace(" ", "")) if m.group(2) else 0))
                pos += 4
            elif l.startswith(".space"):
                n = int(l.split()[1], 0)
                spaces.append((base + pos, src, off + pos, n))
                pos += n
            elif l.startswith(".zero"):
                pos += int(l.split()[1], 0)
            else:
                raise SystemExit("unknown directive in %s: %s" % (f.name, l))
        tables.append({"name": name, "base": base, "size": pos, "labels": labels, "longs": longs, "spaces": spaces})
    return tables

def main():
    index = json.loads((OUT / "meta/_index.json").read_text())
    metas = [json.loads(p.read_text()) for p in sorted((OUT / "meta").glob("*.json")) if not p.name.startswith("_")]
    known, dup = symbol_files()
    tables = parse_tables()
    sym = {}
    regions = []   # (start, end, name)
    for t in tables:
        for lbl, a in t["labels"].items():
            sym[lbl] = a
        regions.append((t["base"], t["base"] + t["size"], t["name"]))
    # C globals: merge tentative definitions (-fcommon)
    gdefs = {}
    for m in metas:
        for g in m["globals"]:
            n = g["name"]
            if n in gdefs:
                old = gdefs[n]
                if g.get("bytes") and not old.get("bytes"):
                    gdefs[n] = g
                elif g.get("bytes") and old.get("bytes") and g["bytes"] != old["bytes"]:
                    print("conflicting definitions of", n)
                gdefs[n]["size"] = max(old["size"], g["size"])
            else:
                gdefs[n] = g
    extra = EXTRA_BASE
    placed_known, moved = 0, []
    order = sorted(gdefs.values(), key=lambda g: (known.get(g["name"], (1 << 40,))[0], g["name"]))
    regions.sort()
    import bisect
    occupied = [(a, b) for a, b, _ in regions]
    def free(a, size):
        for (s, e) in occupied:
            if a < e and s < a + size:
                return False
        return True
    for g in order:
        n = g["name"]
        if n in sym:
            print("global also defined by a data table:", n)
            continue
        k = known.get(n)
        if k and not g.get("static") or (k and g.get("static") and dup[n] == 0 and "__" not in n):
            a = k[0]
            if a % max(1, min(g["align"], 16)) == 0 and free(a, max(1, g["size"])):
                sym[n] = a
                occupied.append((a, a + max(1, g["size"])))
                placed_known += 1
                continue
            moved.append(n)
        al = max(4, min(g["align"], 64))
        extra = (extra + al - 1) // al * al
        sym[n] = extra
        extra += max(1, g["size"])
    print("data: %d tables, %d globals at their PS2 address, %d elsewhere (%d moved for overlap/alignment)" % (
        len(tables), placed_known, len(gdefs) - placed_known, len(moved)))
    if moved:
        print("  moved:", moved[:20])
    # functions
    fid = {}
    nxt = FN_SYNTH
    for name in sorted(index["functions"]):
        k = known.get(name)
        m = re.match(r"func_([0-9A-Fa-f]{8})$", name)
        if k and k[0] < TEXT_END:
            fid[name] = k[0]
        elif m:
            fid[name] = int(m.group(1), 16)
        else:
            fid[name] = nxt
            nxt += 4
    used_ids = collections.Counter(fid.values())
    for name, v in list(fid.items()):
        if used_ids[v] > 1:
            fid[name] = nxt
            nxt += 4
    # host functions: every name called but not defined
    host = {}
    for m in metas:
        for name, params, ret, variadic in m["host"]:
            host.setdefault(name, (params, ret, variadic))
    code_refs = set()
    for p in (OUT / "src/tu").glob("*.rs"):
        code_refs |= set(re.findall(r"fid::(\w+)", p.read_text()))
    for n in sorted(code_refs):
        if n not in fid:
            fid[n] = nxt
            nxt += 4
    # unresolved data references
    data_refs = set()
    for p in (OUT / "src/tu").glob("*.rs"):
        data_refs |= set(re.findall(r"sym::(\w+)", p.read_text()))
    missing = sorted(r for r in data_refs if r not in sym)
    if missing:
        print("data symbols without definition:", len(missing), missing[:40])
        for n in missing:
            k = known.get(n)
            dm = re.match(r"D_([0-9A-Fa-f]{6,8})$", n)
            if dm:
                sym[n] = int(dm.group(1), 16)   # the linker's --defsym of the C port
            elif k:
                sym[n] = k[0]   # an address the PS2 image has; nothing defines its contents
            else:
                extra = (extra + 15) // 16 * 16
                sym[n] = extra
                extra += 64
    gen = OUT / "src/gen"
    gen.mkdir(parents=True, exist_ok=True)
    (gen / "sym.rs").write_text("//! Generated by tools/translate/link.py: data addresses.\n#![allow(non_upper_case_globals)]\n" +
                                "".join("pub const %s: u32 = 0x%08X;\n" % (rust_ident(n), a) for n, a in sorted(sym.items())))
    (gen / "fid.rs").write_text("//! Generated: function identifiers (the original addresses where known).\n#![allow(non_upper_case_globals)]\n" +
                                "".join("pub const %s: u32 = 0x%08X;\n" % (rust_ident(n), a) for n, a in sorted(fid.items())))
    lines = ["//! Generated: every function of the program, by its C name.", "#![allow(unused_imports)]"]
    WRAP = {"Progress_Main": "crate::host::wrap_Progress_Main"}
    WRAP["Bpe_Decode"] = "crate::host::resource_transport::Bpe_Decode"
    for n in ("Snd_LoadBank", "Snd_ReloadBank", "Snd_UnloadBank"):
        WRAP[n] = "crate::host::snd::" + n
    # PC video decoder with the original movie audio, GS packets and skip fade.
    for n in ("Movie_Open", "Movie_Close", "Movie_Run"):
        WRAP[n] = "crate::host::movie::" + n
    # the memory card flows, replaced by PC saves (src/host/saves.rs; BT3_MEMCARD=1 runs the game's own)
    for n in ("McFlow_Start", "McFlow_Update", "McFlow_PollCard"):
        WRAP[n] = "crate::host::saves::" + n
    # the title screen's third item, Load Game (src/host/title.rs)
    for n in ("Title_Init", "Title_Term", "Title_Draw", "Title_Input", "TextBox_DrawClip"):
        WRAP[n] = "crate::host::title::" + n
    # helpers whose C only moves bits through memory, bound to native versions (src/host/native.rs)
    NATIVE = {"vu0_b__f_bits", "vu0_b__f_from"}
    for n in NATIVE:
        WRAP[n] = "crate::host::native::" + n
    for n, info in sorted(index["functions"].items()):
        if n in WRAP:
            lines.append("pub use %s as %s;" % (WRAP[n], rust_ident(n)))
            lines.append("pub use crate::tu::%s::%s as __real_%s;" % (info["tu"], rust_ident(n), n))
            continue
        lines.append("pub use crate::tu::%s::%s;" % (info["tu"], rust_ident(n)))
    for n in sorted(host):
        if n not in index["functions"]:
            lines.append("pub use crate::host::%s;" % rust_ident(n))
    (gen / "funcs.rs").write_text("\n".join(lines) + "\n")
    # dispatch
    d = ["//! Generated: calls through function pointers.", "use crate::rt::prelude::*;", "",
         "#[allow(unused_variables, clippy::all)]", "pub fn call(c: &mut Ctx, id: u32, a: &[u64]) -> u64 {",
         "    let g = |i: usize| a.get(i).copied().unwrap_or(0);", "    match id {"]
    def thunk(name, params, ret):
        args = []
        off = 0
        if ret == "rec":
            args.append("g(0) as u32")
            off = 1
        for i, p in enumerate(params):
            args.append(RS_RAW_FROM.get(p, "((%s) as u32)") % ("g(%d)" % (i + off)))
        call = "F::%s(c%s)" % (rust_ident(name), "".join(", " + x for x in args))
        if ret == "()":
            return "{ %s; 0 }" % call
        if ret == "rec":
            return "(%s) as u64" % call
        return RS_RAW_TO[ret] % call
    for n, info in sorted(index["functions"].items()):
        d.append("        0x%08X => %s," % (fid[n], thunk(n, info["params"], info["ret"])))
    for n in sorted(host):
        if n in index["functions"] or n not in fid:
            continue
        params, ret, variadic = host[n]
        if variadic:
            continue
        d.append("        0x%08X => %s," % (fid[n], thunk(n, params, ret)))
    d += ["        0 => crate::rt::null_call(c),", "        _ => crate::rt::bad_call(c, id),", "    }", "}"]
    (gen / "dispatch.rs").write_text("\n".join(d) + "\n")
    # data image
    blob = bytearray()
    chunks = 0
    def addr_of(ref, addend):
        kind, name = ref
        if name not in sym and name not in fid:
            m = re.match(r"func_([0-9A-Fa-f]{8})$", name)
            k = known.get(name)
            fid[name] = int(m.group(1), 16) if m else (k[0] if k else None)
            if fid[name] is None:
                raise SystemExit("unresolved relocation to " + name)
        if kind == "fn":
            return (fid[name] + addend) & 0xFFFFFFFF
        if name in sym:
            return (sym[name] + addend) & 0xFFFFFFFF
        if name in fid:
            return (fid[name] + addend) & 0xFFFFFFFF
        raise SystemExit("unresolved relocation to " + name)
    for n, g in sorted(gdefs.items(), key=lambda x: sym.get(x[0], 0)):
        if not g.get("bytes") or n not in sym:
            continue
        b = bytearray(bytes.fromhex(g["bytes"]))
        for off, ref, addend in g["relocs"]:
            b[off:off + 4] = struct.pack("<I", addr_of(ref, addend))
        if not any(b):
            continue
        blob += struct.pack("<II", sym[n], len(b)) + b
        chunks += 1
    spaces = []
    for t in tables:
        for a, name, addend in t["longs"]:
            v = addr_of(("sym", name), addend) if name in sym else addr_of(("fn", name), addend)
            blob += struct.pack("<II", a, 4) + struct.pack("<I", v)
            chunks += 1
        spaces += t["spaces"]
    (gen / "data.bin").write_bytes(bytes(blob))
    sp = ["//! Generated: the program's initial data.", "/// (address, length, bytes)* of the C initialisers and table pointers",
          "pub static IMAGE: &[u8] = include_bytes!(\"data.bin\");",
          "/// Table runs whose bytes are the game's original data: (address, source, offset in source, length).",
          "pub static ORIGINAL_RUNS: &[(u32, u32, u32, u32)] = &["]
    sp += ["    (0x%08X, %d, 0x%X, %d)," % s for s in spaces]
    sp += ["];", "pub const EXTRA_END: u32 = 0x%08X;" % extra]
    (gen / "data.rs").write_text("\n".join(sp) + "\n")
    tus = sorted(p.stem for p in (OUT / "src/tu").glob("*.rs") if p.stem != "mod")
    (OUT / "src/tu/mod.rs").write_text("".join("pub mod %s;\n" % rust_ident(t) for t in tus))
    (OUT / "meta/_host.json").write_text(json.dumps({n: host[n] for n in sorted(host) if n not in index["functions"]}, indent=0))
    print("%d chunks, %d original runs, %d functions, %d host functions" % (chunks, len(spaces), len(index["functions"]), len([h for h in host if h not in index["functions"]])))

main()
