"""Stage models to glTF, rebuildable to the exact bytes (like modelconv.py for fighters).

The stage is entry 0 of its file (BtlStage_Relocate, src/sys/gfxm_e_d.c: offsets in words from the stage's start).
Header +0x18/+0x1C: the 7 draw groups (StgMModel, include/battle/stgm_a.h): u16, u16 materials, materials ->
{u16 texture, u16 meshes, meshes, animation} -> {u16 visibility, u8, u8 program, ..., VIF chain, matrix}. A chain
uploads batches with UNPACK V4-32: a header of 3 quadwords at 0 whose third is the GIF tag (NLOOP vertices, triangle
strip), then the vertices from quadword 3, 3 quadwords each: x y z 1 | r g b a (floats, 128 = full) | s t q 0, and
MSCNT draws them. +0x54: the stage's texture file (entry i at +32 + 64 i). The w and q words go to _W_GAME and
_Q_GAME as the game has them (w is not always 1.0).
"""
import struct, json

def u32(d, o): return struct.unpack_from("<I", d, o)[0]
def u16(d, o): return struct.unpack_from("<H", d, o)[0]

TAKEN = (0, 1, 2, 4, 5, 6, 7, 8, 9)  # x y z | r g b a | s t
TAKEN_WQ = (3, 10)                    # w, q (_W_GAME, _Q_GAME)

def stage_base(d):
    if len(d) < 16:
        return None
    n = u32(d, 0)
    if not 1 <= n <= 4096 or (n + 1) * 4 > len(d):
        return None
    b = u32(d, 4) & ~3
    if b + 0x60 > len(d) or u32(d, b + 0x18) != 7:
        return None
    return b

def meshes(d, B):
    """-> [(group, material, texture, chain offset)]"""
    out = []
    a = B + u32(d, B + 0x1C) * 4
    for g in range(7):
        mc, mats = u16(d, a + g * 8 + 2), B + u32(d, a + g * 8 + 4) * 4
        for m in range(mc):
            ma = mats + m * 16
            tex, kc, ks = u16(d, ma), u16(d, ma + 2), B + u32(d, ma + 4) * 4
            for k in range(kc):
                out.append((g, m, tex, B + u32(d, ks + k * 16 + 8) * 4))
    return out

def batches(d, chain, end):
    """-> [(vertex offset, count, prim)] of a chain"""
    out = []
    mem = {}
    at = chain
    for _ in range(4096):
        tag = u32(d, at)
        kind, qwc = (tag >> 28) & 7, tag & 0xffff
        if at + 16 + qwc * 16 > end:
            raise ValueError("dma")
        pos = [at + 8, at + 12] + [at + 16 + 4 * i for i in range(qwc * 4)]
        i = 0
        while i < len(pos):
            w = u32(d, pos[i]); i += 1
            cmd, n, imm = (w >> 24) & 0x7f, ((w >> 16) & 255) or 256, w & 0xffff
            if cmd in (0, 0x10, 0x11, 0x13, 0x14, 0x01, 0x02, 0x03, 0x05):
                continue
            if cmd == 0x17 or cmd == 0x15:
                tagq = mem.get(2)
                if tagq is not None:
                    nloop = u32(d, tagq) & 0x7fff
                    prim = (u32(d, tagq + 4) >> 15) & 0x7ff
                    v0 = mem.get(3)
                    if nloop and v0 is not None:
                        if any(mem.get(3 + j) != v0 + 16 * j for j in range(3 * nloop)):
                            raise ValueError("vertices not contiguous")
                        out.append((v0, nloop, prim))
                mem = {}
                continue
            if cmd == 0x6c:
                dest = imm & 1023
                for q in range(n):
                    o = pos[i + q * 4]
                    if pos[i + q * 4 + 3] != o + 12:
                        raise ValueError("split qword")
                    mem[dest + q] = o
                i += n * 4
                continue
            raise ValueError("vif %02x" % cmd)
        at += 16 + qwc * 16
        if kind in (0, 6, 7):
            return out
    raise ValueError("chain too long")

def convert(d, skel, textures=()):
    """d: the file; skel: bytearray (words taken are zeroed); textures: [(entry offset, png)].
    -> (gltf text, bin, models.txt text) or None"""
    B = stage_base(d)
    if B is None:
        return None
    end = B + (u32(d, 8) & ~3) if u32(d, 0) >= 2 else len(d)
    T = B + u32(d, B + 0x54) * 4
    f32 = lambda w: struct.unpack("<f", struct.pack("<I", w))[0]
    binb = bytearray(); views = []; accs = []
    def add(data, ctype, count, typ, target=None, mm=None):
        while len(binb) % 4:
            binb.append(0)
        v = {"buffer": 0, "byteOffset": len(binb), "byteLength": len(data)}
        if target:
            v["target"] = target
        views.append(v); binb.extend(data)
        a = {"bufferView": len(views) - 1, "componentType": ctype, "count": count, "type": typ}
        if mm:
            a["min"], a["max"] = mm
        accs.append(a)
        return len(accs) - 1
    tex_png = dict(textures)
    mats, matkey, images, texs = [], {}, [], []
    prims, lines, seen = [], [], set()
    for g, m, tex, chain in meshes(d, B):
        if chain in seen:
            continue
        seen.add(chain)
        for off, n, prim in batches(d, chain, end):
            if prim & 7 != 4 or n < 3:
                continue
            words = [struct.unpack_from("<12I", d, off + i * 48) for i in range(n)]
            P = [[f32(v[k]) for k in range(3)] for v in words]
            mm = ([min(p[k] for p in P) for k in range(3)], [max(p[k] for p in P) for k in range(3)])
            pos = b"".join(struct.pack("<3I", *v[0:3]) for v in words)
            colg = b"".join(struct.pack("<4I", *v[4:8]) for v in words)
            col = b"".join(struct.pack("<4f", *(min(max(f32(c) / 128.0, 0.0), 1.0) for c in v[4:8])) for v in words)
            uv = b"".join(struct.pack("<2I", *v[8:10]) for v in words)
            idx = b"".join(struct.pack("<3H", *((i, i + 1, i + 2) if i % 2 == 0 else (i + 1, i, i + 2))) for i in range(n - 2))
            attrs = {"POSITION": add(pos, 5126, n, "VEC3", 34962, mm), "COLOR_0": add(col, 5126, n, "VEC4", 34962),
                     "_COLOR_GAME": add(colg, 5126, n, "VEC4", 34962), "TEXCOORD_0": add(uv, 5126, n, "VEC2", 34962),
                     # the 4th word of the position (1.0; 0 or the integer 1 on some vertices) and q, as the game has them
                     "_W_GAME": add(b"".join(struct.pack("<I", v[3]) for v in words), 5126, n, "SCALAR", 34962),
                     "_Q_GAME": add(b"".join(struct.pack("<I", v[10]) for v in words), 5126, n, "SCALAR", 34962)}
            ind = add(idx, 5123, 3 * (n - 2), "SCALAR", 34963)
            key = (g, tex)
            if key not in matkey:
                mt = {"name": "group%d_tex%03d" % key, "doubleSided": True,
                      "pbrMetallicRoughness": {"metallicFactor": 0, "roughnessFactor": 1}}
                png = tex_png.get(T + 32 + tex * 64)
                if png:
                    if png not in images:
                        images.append(png); texs.append({"source": len(images) - 1, "sampler": 0})
                    mt["pbrMetallicRoughness"]["baseColorTexture"] = {"index": images.index(png)}
                    if g >= 3:
                        mt["alphaMode"] = "BLEND"
                matkey[key] = len(mats); mats.append(mt)
            prims.append({"attributes": attrs, "indices": ind, "material": matkey[key], "mode": 4})
            lines.append("%d %d %d" % (len(prims) - 1, off, n))
    if not prims:
        return None
    for line in lines:
        p, off, n = map(int, line.split())
        for i in range(n):
            for k in TAKEN + TAKEN_WQ:
                o = off + i * 48 + k * 4
                skel[o:o + 4] = b"\0\0\0\0"
    g = {"asset": {"version": "2.0", "generator": "Tenkaichi3Rust tools/assets/stageconv.py"},
         "scene": 0, "scenes": [{"nodes": [1]}],
         "nodes": [{"name": "stage", "mesh": 0},
                   # the game's y points down and a stage is tens of thousands of units across
                   {"name": "root", "rotation": [1.0, 0.0, 0.0, 0.0], "scale": [0.001, 0.001, 0.001], "children": [0]}],
         "meshes": [{"name": "stage", "primitives": prims}], "materials": mats,
         "buffers": [{"uri": "model.bin", "byteLength": len(binb)}], "bufferViews": views, "accessors": accs}
    if images:
        g["images"] = [{"uri": p} for p in images]
        g["textures"] = texs
        g["samplers"] = [{"magFilter": 9729, "minFilter": 9729}]
    txt = ("# where the vertices of model.gltf go in the file (tools/assets/stageconv.py): primitive of mesh 0, file "
           "offset of its first vertex (48 bytes each: x y z 1 | r g b a | s t q 0), vertices\nlayout stage\n" + "\n".join(lines) + "\n")
    return json.dumps(g, separators=(",", ":")), bytes(binb), txt

def apply(buf, gltf_text, binb, txt):
    import modelconv
    g = json.loads(gltf_text)
    prims = g["meshes"][0]["primitives"]
    for line in txt.splitlines():
        f = line.split()
        if not f or line.startswith("#") or f[0] == "layout":
            continue
        p, off, n = map(int, f)
        at = prims[p]["attributes"]
        cols = [modelconv._accessor(g, binb, at[k]) for k in ("POSITION", "_COLOR_GAME", "TEXCOORD_0")]
        if any(len(c) != n for c in cols):
            raise ValueError("primitive %d: vertex count" % p)
        for i in range(n):
            for k, w in zip(TAKEN, cols[0][i] + cols[1][i] + cols[2][i]):
                struct.pack_into("<I", buf, off + i * 48 + k * 4, w)
        for k, name in zip(TAKEN_WQ, ("_W_GAME", "_Q_GAME")):
            if name in at:
                vals = modelconv._accessor(g, binb, at[name])
                if len(vals) != n:
                    raise ValueError("primitive %d: vertex count" % p)
                for i in range(n):
                    struct.pack_into("<I", buf, off + i * 48 + k * 4, vals[i][0])
