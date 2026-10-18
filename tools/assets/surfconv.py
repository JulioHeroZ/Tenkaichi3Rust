"""The animated surfaces of a stage (water, lava, scrolling layers: stage pack entry 0xD, file entry 12; EftSurf in
src/battle/eft_c.c, include/battle/eft_c.h) as surfaces.gltf + surfaces.bin + surfaces.txt, and their tables as
typed records in surface-tables.json.

The entry: the reflecting surface's parameters (0x40 bytes), then the surface file ("mdl3", version 4): a header,
texture records, surface records (texture, triangle count and offset, scroll speed, flags, bounds) and the triangles,
three vertices each of 12 words: x y z w | s t q - | r g b a. Each surface is a primitive of mesh 0 of
surfaces.gltf (triangles) with POSITION, _W_GAME, TEXCOORD_0, _STQ_GAME (q and the 4th word), _COLOR_GAME (COLOR_0
for viewers); surfaces.txt says where its vertices go.
"""
import json, struct
import layout


def u32(d, o):
    return struct.unpack_from("<I", d, o)[0]


def find(d):
    """-> (entry start, file start) of the surface file of a stage file, or None"""
    if len(d) < 64:
        return None
    n = u32(d, 0)
    if n < 13 or 4 + 4 * n > len(d):
        return None
    a, b = u32(d, 4 + 4 * 12), u32(d, 4 + 4 * 13) if n > 13 else len(d)
    if b - a < 0x80 or d[a + 0x40:a + 0x44] != b"mdl3" or u32(d, a + 0x44) != 4:
        return None
    return a, a + 0x40


def convert(d, skel):
    """-> (surfaces.gltf text, surfaces.bin, surfaces.txt text, surface-tables.json text) or None."""
    f = find(d)
    if f is None:
        return None
    a, F = f
    end = u32(d, 4 + 4 * 13)
    nt, to, nm, mo = u32(d, F + 0x10), u32(d, F + 0x14), u32(d, F + 0x18), u32(d, F + 0x1C)
    if nt > 256 or nm > 4096 or F + to + 0x40 * nt > end or F + mo + 0x60 * nm > end:
        return None
    lay = layout.Layout(0)
    lay.type("SurfaceParam", 0x40, [("colorA", 0, "f32[4]"), ("colorB", 0x10, "f32[4]"), ("lightDir", 0x20, "f32[4]"),
                                    ("unk30", 0x30, "f32"), ("animSpeed", 0x34, "s32"), ("fadeDist", 0x38, "f32"),
                                    ("unk3C", 0x3C, "f32")])
    lay.type("SurfaceFile", 0x20, [("magic", 0, "u8[4]"), ("version", 4, "s32"), ("unk8", 8, "s32"), ("unkC", 0x0C, "s32"),
                                   ("textureCount", 0x10, "s32"), ("textures", 0x14, "s32"), ("surfaceCount", 0x18, "s32"),
                                   ("surfaces", 0x1C, "s32")])
    lay.type("SurfaceTexture", 0x40, [("ofs0", 0, "s32"), ("ofs4", 4, "s32"), ("unk8", 8, "u32[8]"), ("tex0", 0x28, "u32[2]"),
                                      ("unk30", 0x30, "u32[2]"), ("ptr0", 0x38, "s32"), ("ptr4", 0x3C, "s32")])
    lay.type("Surface", 0x60, [("texture", 0, "s32"), ("triangleCount", 4, "s32"), ("triangles", 8, "s32"),
                               ("du", 0x0C, "f32"), ("dv", 0x10, "f32"), ("u", 0x14, "f32"), ("v", 0x18, "f32"),
                               ("zBias", 0x1C, "s32"), ("flags", 0x20, "s32"), ("unk24", 0x24, "s32[3]"),
                               ("boxMin", 0x30, "f32[4]"), ("boxMax", 0x40, "f32[4]"), ("center", 0x50, "f32[4]")])
    lay.read(d, "SurfaceParam", a)
    lay.read(d, "SurfaceFile", F)
    for i in range(nt):
        lay.read(d, "SurfaceTexture", F + to + 0x40 * i)
    surfs = []
    for i in range(nm):
        r = F + mo + 0x60 * i
        lay.read(d, "Surface", r)
        cnt, tri = u32(d, r + 4), u32(d, r + 8)
        if cnt > 100000 or F + tri + 0x90 * cnt > end:
            return None
        surfs.append((F + tri, 3 * cnt))
    f32 = lambda w: struct.unpack("<f", struct.pack("<I", w))[0]
    binb, views, accs, prims, lines = bytearray(), [], [], [], []

    def add(data, ctype, count, typ, target=None, mm=None):
        while len(binb) % 4:
            binb.append(0)
        v = {"buffer": 0, "byteOffset": len(binb), "byteLength": len(data)}
        if target:
            v["target"] = target
        views.append(v)
        binb.extend(data)
        acc = {"bufferView": len(views) - 1, "componentType": ctype, "count": count, "type": typ}
        if mm:
            acc["min"], acc["max"] = mm
        accs.append(acc)
        return len(accs) - 1

    for o, n in surfs:
        if n == 0:
            continue
        words = [struct.unpack_from("<12I", d, o + 48 * k) for k in range(n)]
        P = [[f32(v[j]) for j in range(3)] for v in words]
        fin = all(x == x and abs(x) != float("inf") for p in P for x in p)
        mm = ([min(p[j] for p in P) for j in range(3)], [max(p[j] for p in P) for j in range(3)]) if fin else None
        col = b"".join(struct.pack("<4f", *(min(max(f32(c) / 255.0, 0.0), 1.0) if f32(c) == f32(c) else 0.0 for c in v[8:12])) for v in words)
        prims.append({"attributes": {
            "POSITION": add(b"".join(struct.pack("<3I", *v[0:3]) for v in words), 5126, n, "VEC3", 34962, mm),
            "_W_GAME": add(b"".join(struct.pack("<I", v[3]) for v in words), 5126, n, "SCALAR", 34962),
            "TEXCOORD_0": add(b"".join(struct.pack("<2I", *v[4:6]) for v in words), 5126, n, "VEC2", 34962),
            "_STQ_GAME": add(b"".join(struct.pack("<2I", *v[6:8]) for v in words), 5126, n, "VEC2", 34962),
            "_COLOR_GAME": add(b"".join(struct.pack("<4I", *v[8:12]) for v in words), 5126, n, "VEC4", 34962),
            "COLOR_0": add(col, 5126, n, "VEC4", 34962)}, "mode": 4})
        lines.append("%d %d %d" % (len(prims) - 1, o, n))
    for line in lines:
        p, o, n = map(int, line.split())
        skel[o:o + 48 * n] = bytes(48 * n)
    layout.finish(lay, d, skel)
    g = {"asset": {"version": "2.0", "generator": "Tenkaichi3Rust tools/assets/surfconv.py"},
         "scene": 0, "scenes": [{"nodes": [1]}],
         "nodes": [{"name": "surfaces", "mesh": 0},
                   {"name": "root", "rotation": [1.0, 0.0, 0.0, 0.0], "scale": [0.001, 0.001, 0.001], "children": [0]}],
         "meshes": [{"name": "surfaces", "primitives": prims}],
         "buffers": [{"uri": "surfaces.bin", "byteLength": len(binb)}], "bufferViews": views, "accessors": accs}
    txt = ("# where the vertices of surfaces.gltf go (tools/assets/surfconv.py): primitive of mesh 0, file offset, vertices "
           "(48 bytes each: x y z w | s t q - | r g b a)\n" + "\n".join(lines) + "\n")
    tab = lay.text("the stage's animated surfaces (tools/assets/surfconv.py): reflection parameters, the surface file's "
                   "header, textures and surfaces (texture, triangles, scroll, flags, bounds); names after include/battle/eft_c.h")
    return json.dumps(g, separators=(",", ":")), bytes(binb), txt, tab


def apply(buf, gltf_text, binb, txt):
    import modelconv
    g = json.loads(gltf_text)
    prims = g["meshes"][0]["primitives"]
    acc = lambda i: modelconv._accessor(g, binb, i)
    for line in txt.splitlines():
        if not line or line.startswith("#"):
            continue
        p, o, n = map(int, line.split())
        at = prims[p]["attributes"]
        pos, w, st, stq, col = (acc(at[k]) for k in ("POSITION", "_W_GAME", "TEXCOORD_0", "_STQ_GAME", "_COLOR_GAME"))
        for k in range(n):
            struct.pack_into("<12I", buf, o + 48 * k, *(list(pos[k]) + [w[k][0]] + list(st[k]) + list(stq[k]) + list(col[k])))
