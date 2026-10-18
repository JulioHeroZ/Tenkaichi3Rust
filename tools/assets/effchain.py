"""The small meshes of the effects (effect packs of the stages and of the fighters' technique files) as
effects.gltf + effect-chains.json, so their VIF chains are written whole from open files.

The chains have the form of the stage meshes' (stagechain.py): a DMA RET tag; optionally one setting quadword; per
batch UNPACK of a 3-quadword header (two settings and the GIF tag of the strip, NLOOP = the vertex count), UNPACK of
the vertices (x y z w | r g b a | s t q 0), MSCNT; NOPs to the quadword; a DMA END tag. They are found by scanning
the file for such chains (16-byte aligned) where the bytes are still in the skeleton. Each batch is a primitive of
mesh 0 of effects.gltf (a triangle strip given as triangles), with the attributes of stageconv.py: POSITION,
_W_GAME, _COLOR_GAME (COLOR_0 for viewers), TEXCOORD_0, _Q_GAME. effect-chains.json has the stage-chains.json
schema; its chains end with their END tag (tail 16).
"""
import json, struct
import stagechain


def u32(d, o):
    return struct.unpack_from("<I", d, o)[0]


def find(d, skel):
    """-> [(chain offset, prefix, batches, end)] of the chains whose bytes are all still in skel"""
    out = []
    o = 0
    n = len(d) - 32
    while o < n:
        tag = u32(d, o)
        if tag >> 16 == 0x6000 and tag & 0xFFFF and not u32(d, o + 4) and u32(d, o + 8) in (stagechain.HDR_UNPACK, stagechain.PRE_UNPACK):
            try:
                prefix, bs, ce = stagechain.parse(d, o, len(d))
            except (ValueError, struct.error, IndexError):
                o += 16
                continue
            if bs and ce + 16 <= len(d) and d[ce:ce + 16] == struct.pack("<4I", 0x70000000, 0, 0, 0) \
                    and skel[o:ce + 16] == d[o:ce + 16]:
                out.append((o, prefix, bs, ce))
                o = ce + 16
                continue
        o += 16
    return out


def convert(d, skel):
    """-> (effects.gltf text, effects.bin, effect-chains.json text) or None; the chains zeroed in skel."""
    found = find(d, skel)
    if not found:
        return None
    f32 = lambda w: struct.unpack("<f", struct.pack("<I", w))[0]
    binb, views, accs, prims, chains = bytearray(), [], [], [], []

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

    vmap = {}
    for at, prefix, bs, ce in found:
        rec = {"at": at, "tail": 16, "batches": []}
        if prefix is not None:
            rec["prefix"] = prefix
        for hdr, n, vo in bs:
            words = [struct.unpack_from("<12I", d, vo + 48 * i) for i in range(n)]
            P = [[f32(v[k]) for k in range(3)] for v in words]
            if not all(x == x and abs(x) != float("inf") for p in P for x in p):
                mm = None
            else:
                mm = ([min(p[k] for p in P) for k in range(3)], [max(p[k] for p in P) for k in range(3)])
            col = b"".join(struct.pack("<4f", *(min(max(f32(c) / 128.0, 0.0), 1.0) if f32(c) == f32(c) else 0.0 for c in v[4:8])) for v in words)
            attrs = {"POSITION": add(b"".join(struct.pack("<3I", *v[0:3]) for v in words), 5126, n, "VEC3", 34962, mm),
                     "COLOR_0": add(col, 5126, n, "VEC4", 34962),
                     "_COLOR_GAME": add(b"".join(struct.pack("<4I", *v[4:8]) for v in words), 5126, n, "VEC4", 34962),
                     "TEXCOORD_0": add(b"".join(struct.pack("<2I", *v[8:10]) for v in words), 5126, n, "VEC2", 34962),
                     "_W_GAME": add(b"".join(struct.pack("<I", v[3]) for v in words), 5126, n, "SCALAR", 34962),
                     "_Q_GAME": add(b"".join(struct.pack("<I", v[10]) for v in words), 5126, n, "SCALAR", 34962)}
            idx = b"".join(struct.pack("<3H", *((i, i + 1, i + 2) if i % 2 == 0 else (i + 1, i, i + 2))) for i in range(n - 2))
            prims.append({"attributes": attrs, "indices": add(idx, 5123, 3 * (n - 2), "SCALAR", 34963), "mode": 4})
            vmap[len(prims) - 1] = [list(w) for w in words]
            rec["batches"].append({"primitive": len(prims) - 1, "gs": hdr})
        chains.append(rec)
    text = json.dumps({"about": "the VIF chains of the effects' meshes (tools/assets/effchain.py, the schema of "
                       "stage-chains.json): where each is, its optional setting quadword, per batch its primitive of "
                       "effects.gltf and the 12 words of its header (NLOOP left out); each ends with its END tag",
                       "chains": chains}, separators=(",", ":"))
    for at, data in stagechain.apply_chains(json.loads(text), lambda p: vmap[p]):
        if d[at:at + len(data)] != data:
            raise ValueError("effect chain at %d does not rebuild" % at)
        skel[at:at + len(data)] = bytes(len(data))
    g = {"asset": {"version": "2.0", "generator": "Tenkaichi3Rust tools/assets/effchain.py"},
         "scene": 0, "scenes": [{"nodes": [0]}], "nodes": [{"name": "effects", "mesh": 0}],
         "meshes": [{"name": "effects", "primitives": prims}],
         "buffers": [{"uri": "effects.bin", "byteLength": len(binb)}], "bufferViews": views, "accessors": accs}
    return json.dumps(g, separators=(",", ":")), bytes(binb), text


def apply(buf, gltf_text, binb, text):
    for at, data in stagechain.apply_chains(json.loads(text), stagechain.gltf_verts(json.loads(gltf_text), binb)):
        buf[at:at + len(data)] = data
