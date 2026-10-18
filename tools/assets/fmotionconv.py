"""The motions inside a fighter file (entries 0x1C..0x23: uncompressed motions of the model, BObjMdl.unk720 in
include/battle/bobj_b.h, the same format as the motions of an animation bank, motion.py) in open formats, so that
no byte of them stays in skeleton.bin:

* model-motions.gltf + .bin: their keys as glTF animations motion_<entry> (rotations, and translations for tracks
  with positions; times are frames / 30), on nodes named node_<n> like the model's;
* model-motions.json: per motion its structure (convert_native_motion_sources.py: header, track places and sizes,
  event volume places, the keys of tracks glTF cannot carry) and its events (motion.py: length, hit windows...).

The rebuild (host/assets.rs model_motions_apply) writes each entry from these, where the file's directory has it.
"""
import json, struct
import motion
from convert_native_motion_sources import describe

FIRST, LAST = 0x1C, 0x23


def entries(d):
    n = struct.unpack_from("<I", d, 0)[0]
    if not 0x24 <= n <= 4096:
        return []
    o = struct.unpack_from("<%dI" % (n + 1), d, 4)
    out = []
    for k in range(FIRST, LAST + 1):
        a, b = o[k], o[k + 1]
        if b - a >= 148 and motion.motion_ok(d[a:b]):
            out.append((k, a, b))
    return out


def convert(d, skel):
    """-> (gltf text, bin, json text) or None; the motions' bytes zeroed in skel."""
    import numpy as np
    es = entries(d)
    if not es:
        return None
    g = {"asset": {"version": "2.0", "generator": "Tenkaichi3Rust tools/assets/fmotionconv.py"}, "scene": 0,
         "scenes": [{"nodes": [0]}], "nodes": [{"name": "root", "rotation": [1.0, 0.0, 0.0, 0.0], "children": []}],
         "buffers": [], "bufferViews": [], "accessors": [], "animations": []}
    node_of = {}
    binb = bytearray()
    def add(data, ctype, count, typ, mm=None):
        while len(binb) % 4:
            binb.append(0)
        g["bufferViews"].append({"buffer": 0, "byteOffset": len(binb), "byteLength": len(data)})
        binb.extend(data)
        a = {"bufferView": len(g["bufferViews"]) - 1, "componentType": ctype, "count": count, "type": typ}
        if mm:
            a["min"], a["max"] = mm
        g["accessors"].append(a)
        return len(g["accessors"]) - 1
    rows, events = [], []
    for k, a, b in es:
        data = d[a:b]
        m = bytearray(data)
        samplers, channels, bound = [], [], set()
        for tr in motion.tracks(data):
            node, at, fl, n, keys = tr
            frames = [kk[0] for kk in keys]
            if any(y <= x for x, y in zip(frames, frames[1:])):
                continue
            if any(int(round(float(np.float32(f / 30.0)) * 30.0)) != f for f in frames):
                continue
            if node not in node_of:
                g["nodes"].append({"name": "node_%03d" % node})
                node_of[node] = len(g["nodes"]) - 1
                g["nodes"][0]["children"].append(node_of[node])
            times = [float(np.float32(f / 30.0)) for f in frames]
            inp = add(struct.pack("<%df" % n, *times), 5126, n, "SCALAR", ([times[0]], [times[-1]]))
            rots, prev = [], None
            for kk in keys:
                q = motion.unpack(kk[2])
                if prev is not None and sum(float(x) * float(y) for x, y in zip(q, prev)) < 0:
                    q = [np.float32(-x) for x in q]
                prev = q
                rots.append(struct.pack("<4f", *q))
            samplers.append({"input": inp, "output": add(b"".join(rots), 5126, n, "VEC4"), "interpolation": "LINEAR"})
            channels.append({"sampler": len(samplers) - 1, "target": {"node": node_of[node], "path": "rotation"}})
            if not fl & 1:
                samplers.append({"input": inp, "output": add(b"".join(struct.pack("<3f", *kk[1]) for kk in keys), 5126, n, "VEC3"), "interpolation": "LINEAR"})
                channels.append({"sampler": len(samplers) - 1, "target": {"node": node_of[node], "path": "translation"}})
            motion.take(m, tr)
            bound.add(node)
        if channels:
            g["animations"].append({"name": "motion_%03d" % k, "samplers": samplers, "channels": channels})
        length, evs = motion.events_of(data)
        import math
        vals = [x for e in evs if "volume" in e for x in e["volume"]["position"] + [e["volume"]["w"]] + e["volume"]["radii"]]
        if not all(math.isfinite(x) for x in vals):
            raise ValueError("motion %d: an event volume is not a number" % k)
        motion.take_events(m, evs)
        events.append({"entry": k, "length": length, "events": evs})
        row = describe(bytes(m), bound)
        row["entry"] = k
        rows.append(row)
    g["buffers"].append({"uri": "model-motions.bin", "byteLength": len(binb)})
    if not g["animations"]:
        del g["animations"]
    doc = {"version": 1, "about": "motions of the fighter file (tools/assets/fmotionconv.py): structure per entry "
           "(convert_native_motion_sources.py) and events (motion.py); keys in model-motions.gltf",
           "motions": rows, "events": {"motions": events}}
    gtext, jtext = json.dumps(g, separators=(",", ":")), json.dumps(doc, separators=(",", ":"), allow_nan=False)
    check = bytearray(len(d))
    apply(check, gtext, bytes(binb), jtext, d)
    for k, a, b in es:
        if check[a:b] != d[a:b]:
            raise ValueError("motion %d does not rebuild" % k)
    for k, a, b in es:
        skel[a:b] = bytes(b - a)
    return gtext, bytes(binb), jtext


def build_entry(row):
    """The bytes of a motion entry from its structure (as host/assets.rs motion_source_entry)."""
    out = bytearray(row["bytes"])
    vols = row["eventVolumeWords"]
    out[0] = row["headerByte"]
    out[1] = len(vols)
    struct.pack_into("<HH", out, 2, row["templateLength"], row["headerWord"])
    for i, v in enumerate(vols):
        struct.pack_into("<H", out, 148 + i * 16 + 10, v)
    tracks = {}
    for node, at, flags, count in row["tracks"]:
        tracks[node] = (at, flags, count)
        struct.pack_into("<H", out, 6 + node * 2, at // 4)
        struct.pack_into("<HH", out, at, flags, count)
    def keys(at, flags, ks):
        f32 = lambda v: struct.unpack("<I", struct.pack("<f", v))[0]
        motion.write_keys(out, at, [k["frame"] for k in ks], [k["rotation"] for k in ks],
                          [[f32(v) for v in k["position"]] for k in ks] if not flags & 1 else None)
    for fb in row["fallback"]:
        at, flags, count = tracks[fb["node"]]
        keys(at, flags, fb["keys"])
    for u in row.get("unboundTracks", []):
        struct.pack_into("<HH", out, u["byteOffset"], u["flags"], len(u["keys"]))
        keys(u["byteOffset"], u["flags"], u["keys"])
    return out


def apply(buf, gtext, binb, jtext, d=None):
    """The motions into buf at the entries of the directory of buf (or of d)."""
    g, doc = json.loads(gtext), json.loads(jtext)
    src = d if d is not None else buf
    n = struct.unpack_from("<I", src, 0)[0]
    o = struct.unpack_from("<%dI" % (n + 1), src, 4)
    for row in doc["motions"]:
        k = row["entry"]
        m = build_entry(row)
        motion.apply_bank_entry(m, k, g, [binb])
        motion.apply_bank_events(m, k, doc["events"])
        if len(m) != o[k + 1] - o[k]:
            raise ValueError("motion %d: %d bytes, the file has room for %d" % (k, len(m), o[k + 1] - o[k]))
        buf[o[k]:o[k] + len(m)] = m
