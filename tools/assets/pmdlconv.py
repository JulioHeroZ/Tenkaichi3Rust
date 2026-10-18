"""The rest of a fighter model: everything of the pmdl (entry 2 of a fighter file) and of the loose parts (entries
3..8) that is not a vertex, as model.json, so that the whole model is built from model.gltf + model.json and no byte
of it stays in skeleton.bin. modelconv.py reads the model; this file describes how the game lays it out.

A pmdl (include/battle/bobj_a.h, read by modelconv.skeleton) is a header, then one record per node. A record is
64 bytes for a node without geometry, 96 bytes for one with geometry, followed by the node's DMA chain:

* record: next record (bytes), u16 pop, u16 last, u16 vertex count, u16 id, a word, then 3 or 5 vectors (x y z w):
  the node's world position, its parent's, its position relative to the parent, and for a node with geometry two
  more (a point and a bounding sphere).
* chain: one DMA tag RET (qwc) whose VIF code is, per batch of at most 30 vertices: UNPACK V4-32 of 5 quadwords to
  VU1 0 (a GIF tag A+D, TEX0_1, TEX0_2, then the GIF tag of the strip twice, without and with EOP; NLOOP = the vertex
  count), UNPACK V4-32 of 3 quadwords per vertex to VU1 5 (x y z weight | nx ny nz flag | u v q 0), MSCNT. NOPs fill
  the last quadword, then a DMA END tag and zeros up to the next record.

A loose part is the same chain at the start of its entry. Every chain of the game's 742 fighter models has this
form; the builder writes it from the glTF primitives, so a primitive's vertices can be edited freely as long as the
model keeps its size (a different size is refused for now).
"""
import struct, json
import modelconv

def u32(d, o): return struct.unpack_from("<I", d, o)[0]
def u16(d, o): return struct.unpack_from("<H", d, o)[0]

HDR_UNPACK = 0x6C058000      # UNPACK V4-32, 5 quadwords, to VU1 0 (+TOPS)
MSCNT = 0x17000000
DMA_RET, DMA_END = 0x60000000, 0x70000000
MAX_VERTS = 30

def fl(w):
    return struct.unpack("<f", struct.pack("<I", w))[0]

def floats(ws):
    out = []
    for w in ws:
        x = fl(w)
        if x != x or struct.unpack("<I", struct.pack("<f", x))[0] != w:
            raise ValueError("a vector word is not a plain float")
        out.append(x)
    return out

def chain(d, p, end):
    """The canonical chain at p: -> (batches [(header words 20, vertex count, vertex offset)], tail length)."""
    tag = u32(d, p)
    if tag >> 28 != 6 or tag & 0x0FFF0000 or u32(d, p + 4) != 0:
        raise ValueError("chain tag")
    qwc = tag & 0xFFFF
    ce = p + 16 + qwc * 16
    if ce > end:
        raise ValueError("chain past its entry")
    w = [u32(d, p + 8), u32(d, p + 12)] + [u32(d, p + 16 + 4 * i) for i in range(qwc * 4)]
    out, i = [], 0
    while i < len(w):
        if w[i] == 0:
            if any(w[i:]):
                raise ValueError("NOP inside the chain")
            break
        if w[i] != HDR_UNPACK:
            raise ValueError("header unpack %08x" % w[i])
        hdr = w[i + 1:i + 21]
        n = hdr[12] & 0x7FFF
        if hdr[16] & 0x7FFF != n or not 3 <= n <= MAX_VERTS:
            raise ValueError("strip tags")
        if w[i + 21] != 0x6C000000 | (3 * n) << 16 | 0x8005:
            raise ValueError("vertex unpack %08x" % w[i + 21])
        vo = p + 8 + 4 * (i + 22)
        if w[i + 22 + 12 * n] != MSCNT:
            raise ValueError("MSCNT")
        for k in range(n):
            if u32(d, vo + 48 * k + 44):
                raise ValueError("vertex word 11 not zero")
        hdr = list(hdr)
        hdr[12] &= ~0x7FFF
        hdr[16] &= ~0x7FFF
        out.append((hdr, n, vo))
        i += 23 + 12 * n
    if len(w) - i != (-(i + 2)) % 4:
        raise ValueError("NOPs at the end of the chain")
    tail = end - ce
    if tail < 16 or d[ce:ce + 16] != struct.pack("<4I", DMA_END, 0, 0, 0) or any(d[ce + 16:end]):
        raise ValueError("chain tail")
    return out, tail, (len(w) - i)

def build_chain(batches, tail):
    """batches: [(header words 20 without NLOOP, [vertex words 12])] -> bytes of the chain and its tail."""
    w = []
    for hdr, verts in batches:
        n = len(verts)
        h = list(hdr)
        h[12] |= n
        h[16] |= n
        w += [HDR_UNPACK] + h + [0x6C000000 | (3 * n) << 16 | 0x8005]
        for v in verts:
            w += v
        w.append(MSCNT)
    w += [0] * ((-(len(w) + 2)) % 4)
    qwc = (len(w) - 2) // 4
    out = struct.pack("<2I", DMA_RET | qwc, 0) + struct.pack("<%dI" % len(w), *w)
    out += struct.pack("<4I", DMA_END, 0, 0, 0) + bytes(tail - 16)
    return out

def convert(d, skel):
    """-> model.json text, the pmdl and part entries zeroed in skel; or None (no model)."""
    me = modelconv.model_entry(d)
    if not me:
        return None
    s, e = me
    bones = modelconv.skeleton(d, s, e)
    first = u32(d, s + 0x6C)
    j = {"pmdl": {"at": s, "size": e - s, "header": [u32(d, s + 4 * k) for k in range(first // 4)], "nodes": []},
         "parts": []}
    prim = 0
    for b in bones:
        at, size = b["at"], b["size"]
        w = [u32(d, at + 4 * k) for k in range(4)]
        node = {"pop": u16(d, at + 4), "last": u16(d, at + 6), "id": u16(d, at + 10), "word3": w[3]}
        if node["last"]:
            node["next"] = w[0]
        if b["enabled"]:
            node["vectors"] = [floats([u32(d, at + 16 + 16 * v + 4 * k) for k in range(4)]) for v in range(5)]
            bs, tail, _ = chain(d, at + 0x60, at + size)
            node["batches"] = []
            for hdr, n, vo in bs:
                node["batches"].append({"primitive": prim, "gs": hdr})
                prim += 1
            node["tail"] = tail
            if u16(d, at + 8) != sum(n for _, n, _ in bs):
                raise ValueError("node vertex count")
        else:
            if u16(d, at + 8):
                raise ValueError("vertex count of a node without geometry")
            node["vectors"] = [floats([u32(d, at + 16 + 16 * v + 4 * k) for k in range(4)]) for v in range(3)]
            if size > 64:
                node["extra"] = [u32(d, at + 64 + 4 * k) for k in range((size - 64) // 4)]
        if not node["last"] and w[0] != size:
            raise ValueError("next record")
        j["pmdl"]["nodes"].append(node)
    if prim == 0:
        return None
    n = u32(d, 0)
    offs = [u32(d, 4 * i) & ~3 for i in range(1, n + 1)] + [len(d)]
    mesh = 1
    for entry, pbs in modelconv.parts(d):
        a, b2 = offs[entry], offs[entry + 1]
        if not pbs:
            continue
        bs, tail, _ = chain(d, a, b2)
        if len(bs) != len(pbs):
            raise ValueError("part batches")
        j["parts"].append({"entry": entry, "at": a, "size": b2 - a, "mesh": mesh,
                           "batches": [{"primitive": k, "gs": hdr} for k, (hdr, nn, vo) in enumerate(bs)], "tail": tail})
        mesh += 1
    text = json.dumps(j, separators=(",", ":"))
    # the check: the build gives the bytes back (from the file's own vertices)
    def verts_of(vo, nn):
        return [list(struct.unpack_from("<12I", d, vo + 48 * k)) for k in range(nn)]
    vmap = {}
    for b in bones:
        if b["enabled"]:
            for hdr, nn, vo in chain(d, b["at"] + 0x60, b["at"] + b["size"])[0]:
                vmap[(0, len(vmap))] = verts_of(vo, nn)
    for part in j["parts"]:
        for k, (hdr, nn, vo) in enumerate(chain(d, part["at"], part["at"] + part["size"])[0]):
            vmap[(part["mesh"], k)] = verts_of(vo, nn)
    built = build(j, lambda m, p: vmap[(m, p)])
    for at, data in built:
        if d[at:at + len(data)] != data:
            raise ValueError("does not rebuild at %d" % at)
    for at, data in built:
        skel[at:at + len(data)] = bytes(len(data))
    return text

def build(j, verts):
    """model.json + the vertices (verts(mesh, primitive) -> [[12 words]] in strip order) -> [(offset, bytes)]:
    the pmdl and the loose parts."""
    pm = j["pmdl"]
    out = bytearray(struct.pack("<%dI" % len(pm["header"]), *pm["header"]))
    for node in pm["nodes"]:
        geo = "batches" in node
        vecs = b"".join(struct.pack("<4f", *v) for v in node["vectors"])
        if geo:
            batches = []
            for bt in node["batches"]:
                vs = verts(0, bt["primitive"])
                for k in range(0, max(len(vs) - 2, 1), MAX_VERTS - 2):
                    batches.append((bt["gs"], vs[k:k + MAX_VERTS]))
            body = build_chain(batches, node["tail"])
            nv = sum(len(v) for _, v in batches)
            rec = struct.pack("<IHHHHI", 0, node["pop"], node["last"], nv, node["id"], node["word3"]) + vecs
        else:
            body = struct.pack("<%dI" % len(node.get("extra", [])), *node.get("extra", []))
            rec = struct.pack("<IHHHHI", 0, node["pop"], node["last"], 0, node["id"], node["word3"]) + vecs
        size = len(rec) + len(body)
        nxt = node["next"] if node["last"] else size
        out += struct.pack("<I", nxt) + rec[4:] + body
    if len(out) != pm["size"]:
        raise ValueError("the model is %d bytes, the file has room for %d" % (len(out), pm["size"]))
    res = [(pm["at"], bytes(out))]
    for part in j["parts"]:
        batches = []
        for bt in part["batches"]:
            vs = verts(part["mesh"], bt["primitive"])
            for k in range(0, max(len(vs) - 2, 1), MAX_VERTS - 2):
                batches.append((bt["gs"], vs[k:k + MAX_VERTS]))
        body = build_chain(batches, part["tail"])
        if len(body) != part["size"]:
            raise ValueError("part %d is %d bytes, the file has room for %d" % (part["entry"], len(body), part["size"]))
        res.append((part["at"], body))
    return res

def gltf_verts(g, binb):
    """verts(mesh, primitive) from model.gltf: the strip of the primitive (its vertices in order, when its triangles
    are the strip's), as the 12 words of the game's vertex."""
    acc = lambda i: modelconv._accessor(g, binb, i)
    def verts(m, p):
        at = g["meshes"][m]["primitives"][p]["attributes"]
        pos, nrm, uv = acc(at["POSITION"]), acc(at["_NORMAL_GAME"]), acc(at["TEXCOORD_0"])
        wt = [w[0] for w in acc(at["WEIGHTS_0"])] if "WEIGHTS_0" in at else [w[0] for w in acc(at["_WEIGHT_GAME"])]
        q = [w[0] for w in acc(at["_Q_GAME"])] if "_Q_GAME" in at else [0x3F800000] * len(pos)
        f = [w[0] for w in acc(at["_FLAG_GAME"])] if "_FLAG_GAME" in at else [0] * len(pos)
        return [pos[i] + [wt[i]] + nrm[i] + [f[i]] + uv[i] + [q[i], 0] for i in range(len(pos))]
    return verts

def apply(buf, text, gltf_text, binb):
    """The model of model.gltf + model.json into buf (the rebuild of host/assets.rs)."""
    for at, data in build(json.loads(text), gltf_verts(json.loads(gltf_text), binb)):
        buf[at:at + len(data)] = data
