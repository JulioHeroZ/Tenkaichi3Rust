"""Fighter models (pmdl, entry 3 of a resource directory) to glTF, rebuildable to the exact bytes.

A pmdl is a list of node records (the skeleton); each enabled node carries a DMA chain of VIF code that uploads
vertex batches into VU1 memory (UNPACK V4-32) and starts the drawing program (MSCNT). A vertex is 3 quadwords:
x y z weight | nx ny nz - | u v - -. The normals are not of unit length (the lighting uses their length), so the
glTF carries them as the game has them in _NORMAL_GAME (what goes back into the file) and of unit length in NORMAL
(for viewers). The vertices are already in model space, blended between the node and its
parent by the weight. The extraction writes them as glTF (one primitive per batch, as a triangle strip turned into
triangles) and zeroes the words it took from the skeleton file; models.txt says where each vertex goes back.
"""
import struct, json

def u32(d, o): return struct.unpack_from("<I", d, o)[0]
def u16(d, o): return struct.unpack_from("<H", d, o)[0]
def f3(d, o): return list(struct.unpack_from("<3f", d, o))

def model_entry(d):
    """-> offset of the pmdl inside the file, its end; or None"""
    if len(d) < 24:
        return None
    n = u32(d, 0)
    if not 4 <= n <= 4096 or (n + 1) * 4 > len(d):
        return None
    s, e = u32(d, 12) & ~3, u32(d, 16) & ~3
    if not s < e <= len(d) or d[s:s + 4] != b"pmdl":
        return None
    return s, e

def skeleton(d, base, end):
    at = base + u32(d, base + 0x6c)
    bones, stack = [], []
    while True:
        nxt, pop, last, en, bid = u32(d, at), u16(d, at + 4), u16(d, at + 6) != 0, u16(d, at + 8), u16(d, at + 10)
        parent = stack[-1] if stack else None
        rest = f3(d, at + 48)
        world = [rest[i] + (bones[parent]["world"][i] if parent is not None else 0) for i in range(3)]
        size = end - at if last else nxt
        if size < 64 or at + size > end or len(bones) >= 512:
            raise ValueError("bad node")
        bones.append(dict(id=bid, parent=parent, rest=rest, world=world, at=at, size=size, enabled=en != 0))
        stack.append(len(bones) - 1)
        if pop > len(stack):
            raise ValueError("stack")
        del stack[len(stack) - pop:]
        if last:
            return bones
        at += nxt

def batches(d, bones):
    """-> list of dict(bone, tex0, verts=[[12 floats raw u32]], src=[(qw0, qw1, qw2) file offsets])"""
    out = []
    for bi, b in enumerate(bones):
        if not b["enabled"]:
            continue
        at, end = b["at"] + 0x60, b["at"] + b["size"]
        mem = bytearray(16384)
        src = [None] * 1024
        cycle = 0x101
        while True:
            tag = u32(d, at)
            kind, qwc = (tag >> 28) & 7, tag & 0xffff
            if kind not in (1, 6, 7) or at + 16 + qwc * 16 > end:
                raise ValueError("dma")
            # the VIF code: the two words after the tag, then the data
            pos = [at + 8, at + 12] + [at + 16 + 4 * i for i in range(qwc * 4)]
            i = 0
            while i < len(pos):
                w = u32(d, pos[i]); i += 1
                cmd, n, imm = (w >> 24) & 0x7f, ((w >> 16) & 255) or 256, w & 0xffff
                if cmd in (0, 0x10, 0x11, 0x13):
                    continue
                if cmd == 1:
                    cycle = imm
                    continue
                if cmd == 0x17:
                    emit(d, mem, src, bi, out)
                    continue
                if cmd == 0x6c:
                    if cycle & 255 != (cycle >> 8) & 255:
                        raise ValueError("cycle")
                    dest = imm & 1023
                    for q in range(n):
                        o = pos[i + q * 4]
                        if pos[i + q * 4 + 3] != o + 12:
                            raise ValueError("split qword")
                        mem[(dest + q) * 16:(dest + q) * 16 + 16] = d[o:o + 16]
                        src[dest + q] = o
                    i += n * 4
                    continue
                raise ValueError("vif %02x" % cmd)
            at += 16 + qwc * 16
            if kind in (6, 7):
                break
    return out

def part_batches(d, at, end):
    """batches of a loose VIF chain of the fighter file (entries 3..8: faces and other swappable parts), drawn the
    same way as the model's batches"""
    mem = bytearray(16384)
    src = [None] * 1024
    out = []
    tag = u32(d, at)
    kind, qwc = (tag >> 28) & 7, tag & 0xffff
    if kind != 6 or at + 16 + qwc * 16 > end:
        raise ValueError("part tag")
    pos = [at + 8, at + 12] + [at + 16 + 4 * i for i in range(qwc * 4)]
    i = 0
    while i < len(pos):
        w = u32(d, pos[i]); i += 1
        cmd, n, imm = (w >> 24) & 0x7f, ((w >> 16) & 255) or 256, w & 0xffff
        if cmd in (0, 0x10, 0x11, 0x13, 1):
            continue
        if cmd == 0x17:
            emit(d, mem, src, 0, out)
            continue
        if cmd == 0x6c:
            dest = imm & 1023
            for q in range(n):
                o = pos[i + q * 4]
                if pos[i + q * 4 + 3] != o + 12:
                    raise ValueError("split qword")
                mem[(dest + q) * 16:(dest + q) * 16 + 16] = d[o:o + 16]
                src[dest + q] = o
            i += n * 4
            continue
        raise ValueError("vif %02x" % cmd)
    return out

def parts(d):
    """-> [(entry, batches)] of the loose chains of a fighter file"""
    n = u32(d, 0)
    o = [u32(d, 4 * i) & ~3 for i in range(1, n + 1)] + [len(d)]
    out = []
    for i in range(n):
        a, b = o[i], o[i + 1]
        if i == 2 or b - a < 32 or u32(d, a) >> 28 != 6 or u32(d, a + 8) != 0x6c058000:
            continue
        out.append((i, part_batches(d, a, b)))
    return out

def emit(d, mem, src, bone, out):
    tag = u32(mem, 48)
    n = tag & 0x7fff
    if n == 0:
        return
    if n < 3 or 5 + n * 3 > 1024 or (u32(mem, 52) >> 15) & 7 != 4:
        raise ValueError("batch")
    verts, srcs = [], []
    for i in range(n):
        q = 5 + i * 3
        verts.append(list(struct.unpack_from("<12I", mem, q * 16)))
        s = (src[q], src[q + 1], src[q + 2])
        if None in s:
            raise ValueError("vertex not from the file")
        srcs.append(s)
    out.append(dict(bone=bone, tex0=struct.unpack_from("<Q", mem, 16)[0], gif=bytes(mem[48:64]), verts=verts, src=srcs))

TAKEN = (0, 1, 2, 3, 4, 5, 6, 8, 9)  # the words of a vertex that go to the glTF (x y z weight, normal, u v)
TAKEN_Q = 10                         # q (_Q_GAME)

def convert(d, skel, textures=()):
    """d: the file; skel: its skeleton (bytearray, words taken are zeroed here); textures: [(tbp, cbp, png)].
    -> (gltf json text, bin bytes, models.txt text) or None when the file has no model"""
    me = model_entry(d)
    if not me:
        return None
    bones = skeleton(d, *me)
    bs = batches(d, bones)
    if not bs:
        return None
    binb = bytearray()
    views, accs = [], []
    def add(data, ctype, count, typ, target=None, mm=None):
        while len(binb) % 4:
            binb.append(0)
        v = {"buffer": 0, "byteOffset": len(binb), "byteLength": len(data)}
        if target:
            v["target"] = target
        views.append(v)
        binb.extend(data)
        a = {"bufferView": len(views) - 1, "componentType": ctype, "count": count, "type": typ}
        if mm:
            a["min"], a["max"] = mm
        accs.append(a)
        return len(accs) - 1
    mats, matkey, images, texs, samplers = [], {}, [], [], [{"magFilter": 9728, "minFilter": 9728}]
    prims, lines = [], []
    f32 = lambda w: struct.unpack("<f", struct.pack("<I", w))[0]
    def primitive(b, skinned):
        n = len(b["verts"])
        off = b["src"][0][0]
        flat = [q for s in b["src"] for q in s]
        if any(c != flat[0] + 16 * i for i, c in enumerate(flat)):
            raise ValueError("vertices not contiguous")
        pos = b"".join(struct.pack("<3I", *v[0:3]) for v in b["verts"])
        fl = [[f32(v[k]) for k in range(3)] for v in b["verts"]]
        mm = ([min(p[k] for p in fl) for k in range(3)], [max(p[k] for p in fl) for k in range(3)])
        nrm = b"".join(struct.pack("<3I", *v[4:7]) for v in b["verts"])
        unit = bytearray()
        for v in b["verts"]:
            x, y, z = (f32(w) for w in v[4:7])
            l = (x * x + y * y + z * z) ** 0.5
            unit += struct.pack("<3f", *((x / l, y / l, z / l) if l > 0 else (0.0, 0.0, 1.0)))
        uv = b"".join(struct.pack("<2I", *v[8:10]) for v in b["verts"])
        idx = b"".join(struct.pack("<3H", *((i, i + 1, i + 2) if i % 2 == 0 else (i + 1, i, i + 2))) for i in range(n - 2))
        attrs = {"POSITION": add(pos, 5126, n, "VEC3", 34962, mm), "NORMAL": add(bytes(unit), 5126, n, "VEC3", 34962), "_NORMAL_GAME": add(nrm, 5126, n, "VEC3", 34962),
                 "TEXCOORD_0": add(uv, 5126, n, "VEC2", 34962),
                 # q of the texture coordinates (1.0), as the game has it
                 "_Q_GAME": add(b"".join(struct.pack("<I", v[10]) for v in b["verts"]), 5126, n, "SCALAR", 34962)}
        if any(v[7] for v in b["verts"]):
            # the 4th word of the normal: 0, or the integer 1 on a few vertices (a flag of the drawing program)
            attrs["_FLAG_GAME"] = add(b"".join(struct.pack("<I", v[7]) for v in b["verts"]), 5126, n, "SCALAR", 34962)
        if skinned:
            par = bones[b["bone"]]["parent"]
            jt = struct.pack("<4H", b["bone"], b["bone"] if par is None else par, 0, 0) * n
            wt = b"".join(struct.pack("<If2f", v[3], 1.0 - f32(v[3]), 0, 0) for v in b["verts"])
            attrs["JOINTS_0"] = add(jt, 5123, n, "VEC4", 34962)
            attrs["WEIGHTS_0"] = add(wt, 5126, n, "VEC4", 34962)
        else:
            # the weight word without a skin to use it (the part's bones are not known here)
            attrs["_WEIGHT_GAME"] = add(b"".join(struct.pack("<I", v[3]) for v in b["verts"]), 5126, n, "SCALAR", 34962)
        ind = add(idx, 5123, 3 * (n - 2), "SCALAR", 34963)
        key = b["tex0"]
        if key not in matkey:
            tbp, cbp = key & 0x3fff, (key >> 37) & 0x3fff
            m = {"name": "tex0_%016x" % key, "doubleSided": True,
                 "pbrMetallicRoughness": {"metallicFactor": 0, "roughnessFactor": 1}}
            png = next((p for t, c, p in textures if t == tbp and (c is None or c == cbp)), None)
            if png:
                if png not in images:
                    images.append(png)
                    texs.append({"source": len(images) - 1, "sampler": 0})
                m["pbrMetallicRoughness"]["baseColorTexture"] = {"index": images.index(png)}
                m["alphaMode"] = "MASK"
            matkey[key] = len(mats)
            mats.append(m)
        for i in range(n):
            for k in TAKEN + (TAKEN_Q, 7):
                o = off + i * 48 + k * 4
                skel[o:o + 4] = b"\0\0\0\0"
        return {"attributes": attrs, "indices": ind, "material": matkey[key], "mode": 4}, off, n
    for b in bs:
        p, off, n = primitive(b, True)
        prims.append(p)
        lines.append("%d %d %d" % (len(prims) - 1, off, n))
    extra = []
    for entry, pbs in parts(d):
        pp = []
        for b in pbs:
            p, off, n = primitive(b, False)
            pp.append(p)
            lines.append("%d %d %d %d" % (len(extra) + 1, len(pp) - 1, off, n))
        if pp:
            extra.append(({"name": "part_%d" % entry, "primitives": pp}, entry))
    nb = len(bones)
    inv = b"".join(struct.pack("<16f", 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, -b["world"][0], -b["world"][1], -b["world"][2], 1) for b in bones)
    ibm = add(inv, 5126, nb, "MAT4")
    nodes = []
    for i, b in enumerate(bones):
        nd = {"name": "node_%03d" % b["id"], "translation": [float(x) for x in struct.unpack("<3f", struct.pack("<3f", *b["rest"]))]}
        ch = [j for j, c in enumerate(bones) if c["parent"] == i]
        if ch:
            nd["children"] = ch
        nodes.append(nd)
    roots = [i for i, b in enumerate(bones) if b["parent"] is None]
    nodes.append({"name": "model", "mesh": 0, "skin": 0})
    # the game's y points down: a root turned half a turn about x shows the model standing (the data is untouched)
    nodes.append({"name": "root", "rotation": [1.0, 0.0, 0.0, 0.0], "children": roots})
    for k, (mesh, entry) in enumerate(extra):
        nodes.append({"name": "part_%d" % entry, "mesh": k + 1})
        nodes[nb + 1]["children"].append(len(nodes) - 1)
    g = {"asset": {"version": "2.0", "generator": "Tenkaichi3Rust tools/assets/modelconv.py"},
         "scene": 0, "scenes": [{"nodes": [nb + 1, nb]}], "nodes": nodes,
         "skins": [{"joints": list(range(nb)), "inverseBindMatrices": ibm}],
         "meshes": [{"name": "model", "primitives": prims}] + [m for m, e in extra], "materials": mats,
         "buffers": [{"uri": "model.bin", "byteLength": len(binb)}], "bufferViews": views, "accessors": accs}
    if images:
        g["images"] = [{"uri": p} for p in images]
        g["textures"] = texs
        g["samplers"] = samplers
    txt = ("# where the vertices of model.gltf go in the file (tools/assets/modelconv.py): primitive of mesh 0, "
           "file offset of its first vertex (48 bytes each: x y z weight | nx ny nz - | u v - -), vertices; or mesh, "
           "primitive, offset, vertices for the loose parts (meshes 1..)\n"
           "pmdl %d\n" % me[0] + "\n".join(lines) + "\n")
    return json.dumps(g, separators=(",", ":")), bytes(binb), txt

def _accessor(g, binb, i):
    a = g["accessors"][i]
    v = g["bufferViews"][a["bufferView"]]
    comps = {"SCALAR": 1, "VEC2": 2, "VEC3": 3, "VEC4": 4}[a["type"]]
    if a["componentType"] != 5126:
        raise ValueError("accessor %d is not float" % i)
    stride = v.get("byteStride", 4 * comps)
    base = v.get("byteOffset", 0) + a.get("byteOffset", 0)
    return [list(struct.unpack_from("<%dI" % comps, binb, base + k * stride)) for k in range(a["count"])]

def apply(buf, gltf_text, binb, txt):
    """writes the vertices of the glTF back into the file buf (bytearray)"""
    g = json.loads(gltf_text)
    for line in txt.splitlines():
        f = line.split()
        if not f or line.startswith("#") or f[0] == "pmdl":
            continue
        m, p, off, n = map(int, f) if len(f) == 4 else [0] + list(map(int, f))
        at = g["meshes"][m]["primitives"][p]["attributes"]
        cols = [_accessor(g, binb, at[k]) for k in ("POSITION", "_NORMAL_GAME", "TEXCOORD_0")]
        wt = [w[0] for w in _accessor(g, binb, at["WEIGHTS_0"])] if "WEIGHTS_0" in at else [w[0] for w in _accessor(g, binb, at["_WEIGHT_GAME"])]
        if any(len(c) != n for c in cols) or len(wt) != n:
            raise ValueError("mesh %d primitive %d: %d vertices, the file has %d" % (m, p, len(cols[0]), n))
        for i in range(n):
            ws = cols[0][i] + [wt[i]] + cols[1][i] + cols[2][i]
            for k, w in zip(TAKEN, ws):
                struct.pack_into("<I", buf, off + i * 48 + k * 4, w)
        if "_FLAG_GAME" in at:
            fl = _accessor(g, binb, at["_FLAG_GAME"])
            for i in range(n):
                struct.pack_into("<I", buf, off + i * 48 + 28, fl[i][0])
        if "_Q_GAME" in at:
            q = _accessor(g, binb, at["_Q_GAME"])
            if len(q) != n:
                raise ValueError("mesh %d primitive %d: _Q_GAME count" % (m, p))
            for i in range(n):
                struct.pack_into("<I", buf, off + i * 48 + TAKEN_Q * 4, q[i][0])
