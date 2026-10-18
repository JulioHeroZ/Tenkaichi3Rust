"""The open files of an installation in a few packs for the player install (game/src/host/vfs.rs reads them): one
pack per top folder of the assets (files.bt3pak, audio.bt3pak, video.bt3pak), plus data/tables.txt in data.bt3pak. The
copy of the disc (assets/_legacy) is left out: the game does not read it.

Text, JSON, glTF buffers and WAVs are stored zlib-compressed when that saves a tenth; PNGs (already compressed) as
they are. A pack: "BT3PAK1\\0", u32 entry count, u64 offset of the index; the entries; the index: per entry u16 path
length, the path (UTF-8, '/', relative to the assets folder), u64 offset, u32 stored size, u32 size, u8 method
(0 stored, 1 zlib).

    python tools/install/pack.py <assets> <out folder>
"""
import os, struct, sys, zlib
from pathlib import Path

SKIP_TOP = {"_legacy", "catalog.csv", "summary.json"}
PACKS = {"files": "files", "audio": "audio", "data": "data", "video": "video"}
STORED = {".png", ".m2v"}


def pack(assets, rels, out):
    tmp = out.with_name(out.name + ".tmp")
    index = []
    with open(tmp, "wb") as f:
        f.write(b"BT3PAK1\0" + bytes(12))
        for i, rel in enumerate(rels):
            data = (assets / rel).read_bytes()
            method, stored = 0, data
            if Path(rel).suffix.lower() not in STORED and len(data) > 64:
                z = zlib.compress(data, 6)
                if len(z) < len(data) * 0.9:
                    method, stored = 1, z
            index.append((rel, f.tell(), len(stored), len(data), method))
            f.write(stored)
            if i % 5000 == 0:
                print("  %s: %d/%d" % (out.name, i, len(rels)), flush=True)
        at = f.tell()
        for rel, off, ln, size, method in index:
            p = rel.encode("utf-8")
            f.write(struct.pack("<H", len(p)) + p + struct.pack("<QIIB", off, ln, size, method))
        f.seek(8)
        f.write(struct.pack("<IQ", len(index), at))
    os.replace(tmp, out)
    return len(index), out.stat().st_size


def main():
    assets, out = Path(sys.argv[1]), Path(sys.argv[2])
    out.mkdir(parents=True, exist_ok=True)
    groups = {}
    for root, dirs, files in os.walk(assets, followlinks=True):
        rel_root = Path(root).relative_to(assets)
        top = rel_root.parts[0] if rel_root.parts else None
        if top in SKIP_TOP:
            dirs[:] = []
            continue
        for n in files:
            if n.endswith(".tmp") or (top is None and n in SKIP_TOP):
                continue
            rel = (rel_root / n).as_posix()
            name = PACKS.get(top or "", "data")
            if top is None:
                continue  # loose files at the top (catalogs of the export): not needed
            if top == "video" and not n.endswith(".m2v"):
                continue  # older lossless copies of the movies (.mkv): the game reads the .m2v
            if top == "data" and rel != "data/tables.txt":
                continue  # older catalogs: the game reads only tables.txt
            groups.setdefault(name, []).append(rel)
    for name, rels in sorted(groups.items()):
        n, size = pack(assets, sorted(rels), out / ("%s.bt3pak" % name))
        print("%s.bt3pak: %d arquivos, %.1f MB" % (name, n, size / 1e6), flush=True)


if __name__ == "__main__":
    main()
