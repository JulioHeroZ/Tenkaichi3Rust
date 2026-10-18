"""The drawing chains of a stage model (stageconv.py) as stage-chains.json, so the whole chain of every mesh is written
from model.gltf + this file and none of it stays in skeleton.bin.

A mesh's chain (StgMMesh.chain, include/battle/stgm_a.h) is one DMA tag RET whose VIF code is: optionally UNPACK V4-32
of 1 quadword to VU1 0 and MSCNT (a setting for the drawing program), then per batch UNPACK of 3 quadwords to VU1 0
(two settings and the GIF tag of the strip, NLOOP = the vertex count), UNPACK of the vertices to VU1 3
(x y z w | r g b a | s t q 0), MSCNT; NOPs fill the last quadword; then a DMA END tag and zeros up to the next chain.
Every chain of the game's stages has this form.
"""
import json, struct
import stageconv

MSCNT = 0x17000000
PRE_UNPACK = 0x6C018000
HDR_UNPACK = 0x6C038000


def u32(d, o):
    return struct.unpack_from("<I", d, o)[0]


def parse(d, ch, limit):
    """-> (prefix words or None, [(header words 12 without NLOOP, n, vertex offset)], end of the VIF code)"""
    tag = u32(d, ch)
    if tag >> 28 != 6 or tag & 0x0FFF0000 or u32(d, ch + 4):
        raise ValueError("chain tag")
    qwc = tag & 0xFFFF
    if ch + 16 + qwc * 16 > limit:
        raise ValueError("chain past the stage")
    w = [u32(d, ch + 8), u32(d, ch + 12)] + [u32(d, ch + 16 + 4 * i) for i in range(qwc * 4)]
    i, prefix, out = 0, None, []
    if w and w[0] == PRE_UNPACK:
        if w[5] != MSCNT:
            raise ValueError("prefix")
        prefix = w[1:5]
        i = 6
    while i < len(w) and w[i] != 0:
        if w[i] != HDR_UNPACK:
            raise ValueError("header unpack %08x" % w[i])
        hdr = list(w[i + 1:i + 13])
        n = hdr[8] & 0x7FFF
        if n < 3 or w[i + 13] != 0x6C000000 | (3 * n) << 16 | 0x8003:
            raise ValueError("vertex unpack")
        vo = ch + 8 + 4 * (i + 14)
        if w[i + 14 + 12 * n] != MSCNT:
            raise ValueError("MSCNT")
        for k in range(n):
            if u32(d, vo + 48 * k + 44):
                raise ValueError("vertex word 11 not zero")
        hdr[8] &= ~0x7FFF
        out.append((hdr, n, vo))
        i += 15 + 12 * n
    if any(w[i:]) or len(w) - i != (-(i + 2)) % 4:
        raise ValueError("NOPs at the end")
    return prefix, out, ch + 16 + qwc * 16


def build(prefix, batches, tail):
    """batches: [(header words 12 without NLOOP, [[12 words]])] -> the chain's bytes and its tail."""
    w = []
    if prefix is not None:
        w += [PRE_UNPACK] + list(prefix) + [MSCNT]
    for hdr, verts in batches:
        n = len(verts)
        h = list(hdr)
        h[8] |= n
        w += [HDR_UNPACK] + h + [0x6C000000 | (3 * n) << 16 | 0x8003]
        for v in verts:
            w += v
        w.append(MSCNT)
    w += [0] * ((-(len(w) + 2)) % 4)
    qwc = (len(w) - 2) // 4
    return struct.pack("<2I", 0x60000000 | qwc, 0) + struct.pack("<%dI" % len(w), *w) + struct.pack("<4I", 0x70000000, 0, 0, 0) + bytes(tail - 16)


def convert(d, skel):
    """-> stage-chains.json text (the chains zeroed in skel) or None. The primitives are numbered as stageconv.py
    numbers them (unique chains in the order of the meshes, their batches in order)."""
    B = stageconv.stage_base(d)
    if B is None:
        return None
    limit = B + (u32(d, 8) & ~3) if u32(d, 0) >= 2 else len(d)
    seen, chains = set(), []
    for g, m, tex, ch in stageconv.meshes(d, B):
        if ch not in seen:
            seen.add(ch)
            chains.append(ch)
    starts = sorted(chains)
    nxt = {a: b for a, b in zip(starts, starts[1:])}
    prim, out, vmap = 0, [], {}
    for ch in chains:
        prefix, bs, ce = parse(d, ch, limit)
        end = nxt.get(ch)
        if end is None:
            # the last chain: the END tag and the zeros up to the next 64 bytes
            end = (ce + 16 + 63) // 64 * 64
        tail = end - ce
        if tail < 16 or d[ce:ce + 16] != struct.pack("<4I", 0x70000000, 0, 0, 0) or any(d[ce + 16:end]):
            raise ValueError("chain tail")
        rec = {"at": ch, "tail": tail, "batches": []}
        if prefix is not None:
            rec["prefix"] = prefix
        for hdr, n, vo in bs:
            rec["batches"].append({"primitive": prim, "gs": hdr})
            vmap[prim] = [list(struct.unpack_from("<12I", d, vo + 48 * k)) for k in range(n)]
            prim += 1
        out.append(rec)
    text = json.dumps({"about": "the drawing chains of the stage's meshes (tools/assets/stagechain.py): where each is, "
                       "its optional setting quadword, per batch the primitive of model.gltf and the 12 words of its "
                       "header (NLOOP left out), the bytes from the END tag to the next chain", "chains": out},
                      separators=(",", ":"))
    for at, data in apply_chains(json.loads(text), lambda p: vmap[p]):
        if d[at:at + len(data)] != data:
            raise ValueError("chain at %d does not rebuild" % at)
        skel[at:at + len(data)] = bytes(len(data))
    return text


def apply_chains(doc, verts):
    return [(c["at"], build(c.get("prefix"), [(b["gs"], verts(b["primitive"])) for b in c["batches"]], c["tail"]))
            for c in doc["chains"]]


def gltf_verts(g, binb):
    import modelconv
    acc = lambda i: modelconv._accessor(g, binb, i)
    prims = g["meshes"][0]["primitives"]
    def verts(p):
        at = prims[p]["attributes"]
        pos, col, uv = acc(at["POSITION"]), acc(at["_COLOR_GAME"]), acc(at["TEXCOORD_0"])
        w = [x[0] for x in acc(at["_W_GAME"])] if "_W_GAME" in at else [0x3F800000] * len(pos)
        q = [x[0] for x in acc(at["_Q_GAME"])] if "_Q_GAME" in at else [0x3F800000] * len(pos)
        return [pos[i] + [w[i]] + col[i] + uv[i] + [q[i], 0] for i in range(len(pos))]
    return verts


def apply(buf, text, gltf_text, binb):
    for at, data in apply_chains(json.loads(text), gltf_verts(json.loads(gltf_text), binb)):
        buf[at:at + len(data)] = data
