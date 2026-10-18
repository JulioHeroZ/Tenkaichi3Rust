"""Fighter motions (the entries of an animation bank, each packed with the game's BPE on its own).

A motion: u8 ?, u8 events, u16 last frame, u16 ?, then 71 u16 offsets (in words) of the node tracks (0 = none),
the events at 148 (16 bytes each), then the tracks: u16 flags (bit 0: rotation only), u16 keys, then the keys:
rotation only: n x u64 packed rotation, then n x u16 frame; else n x (x y z f32, u32 frame, u64 packed rotation).
A packed rotation keeps the three smaller components of the quaternion in 20 bits each, (v / sqrt 2 + 1/2) scaled
to 0..1048575, and in the top bits the index of the largest one, which is rebuilt as sqrt(1 - the others' squares).
"""
import struct
import numpy as np

F = np.float32
K1 = F(1.0 / 1048575.0)
SQ2 = F(1.41421356)

def small_of(bits):
    return (F(bits) * K1 - F(0.5)) * SQ2

def unpack(packed):
    """-> the quaternion (x y z w, f32), as the game rebuilds it (with an IEEE square root)"""
    largest = packed >> 60
    if largest > 3:
        return None
    v = [small_of((packed >> (i * 20)) & 0xfffff) for i in range(3)]
    s = F(0)
    for x in v:
        s = F(s + F(x * x))
    w = np.sqrt(F(F(1) - s))
    v.insert(int(largest), F(w))
    return v

def _pack_at(q, largest):
    if q[largest] < 0:
        q = [F(-x) for x in q]
    out = largest << 60
    k = 0
    for i in range(4):
        if i == largest:
            continue
        v = q[i]
        b = int(round((float(v) / 1.41421356 + 0.5) * 1048575.0))
        b = min(max(b, 0), 0xfffff)
        # the nearest step whose value is exactly v, when there is one
        for c in (b, b - 1, b + 1, b - 2, b + 2):
            if 0 <= c <= 0xfffff and small_of(c) == v:
                b = c
                break
        out |= b << (k * 20)
        k += 1
    return out

def pack(q):
    """quaternion (4 floats) -> the packed rotation that unpacks to it exactly when there is one (the game's packer
    did not always keep the largest component out: two of them can be almost equal), else the nearest"""
    q = [F(x) for x in q]
    order = sorted(range(4), key=lambda i: -abs(float(q[i])))
    for largest in order:
        p = _pack_at(q, largest)
        u = unpack(p)
        if all(a == b for a, b in zip(u, q)) or all(a == -b for a, b in zip(u, q)):
            return p
    return _pack_at(q, order[0])

def tracks(d):
    """-> list of (node, at, flags, n, keys=[(frame, pos or None, packed)])"""
    out = []
    for node in range(71):
        at = struct.unpack_from("<H", d, 6 + node * 2)[0] * 4
        if at == 0:
            continue
        flags, n = struct.unpack_from("<HH", d, at)
        rot_only = flags & 1
        keys = []
        for i in range(n):
            if rot_only:
                packed = struct.unpack_from("<Q", d, at + 4 + i * 8)[0]
                frame = struct.unpack_from("<H", d, at + 4 + n * 8 + i * 2)[0]
                keys.append((frame, None, packed))
            else:
                k = at + 4 + i * 24
                x, y, z, frame, packed = struct.unpack_from("<3fIQ", d, k)
                keys.append((frame, (x, y, z), packed))
        out.append((node, at, flags, n, keys))
    return out

# ------------------------------------------------------------------------------------------------ banks
def bank_entries(d):
    """A bank: u32 count, the entries' offsets, the end offset, zeros to 64 bytes; each entry at a multiple of 64 is a
    file packed with the game's BPE, padded to 64. -> [(offset, end)] or None"""
    if len(d) < 64:
        return None
    n = struct.unpack_from("<I", d, 0)[0]
    if not 1 <= n <= 4096 or (n + 2) * 4 > len(d):
        return None
    o = list(struct.unpack_from("<%dI" % (n + 2), d))
    head = ((n + 2) * 4 + 63) // 64 * 64
    if o[n + 1] != len(d) or o[1] != head or d[(n + 2) * 4:head] != bytes(head - (n + 2) * 4):
        return None
    ents = []
    for i in range(1, n + 1):
        a, b = o[i], o[i + 1]
        if a % 64 or b < a or b > len(d) or b - a < 16:
            return None
        size, packed = struct.unpack_from("<II", d, a)
        if not 8 + packed <= b - a < 8 + packed + 64:
            return None
        ents.append((a, b))
    return ents

def motion_ok(m):
    if len(m) < 148:
        return False
    try:
        for node, at, fl, n, keys in tracks(m):
            if n == 0 or at < 148 or fl & ~1:
                return False
            for frame, pos, packed in keys:
                if packed >> 60 > 3:
                    return False
        return True
    except struct.error:
        return False

def take(m, track):
    """zeroes the words of the track's keys that go to the glTF (frames, positions, rotations)"""
    node, at, fl, n, keys = track
    if fl & 1:
        m[at + 4:at + 4 + n * 10] = bytes(n * 10)
    else:
        m[at + 4:at + 4 + n * 24] = bytes(n * 24)

def write_keys(m, track_at, frames, rots, poss):
    fl, n = struct.unpack_from("<HH", m, track_at)
    if len(frames) != n:
        raise ValueError("track at %d: %d keys, the file has %d" % (track_at, len(frames), n))
    for i in range(n):
        p = pack(rots[i])
        if fl & 1:
            struct.pack_into("<Q", m, track_at + 4 + i * 8, p)
            struct.pack_into("<H", m, track_at + 4 + n * 8 + i * 2, frames[i])
        else:
            struct.pack_into("<3IIQ", m, track_at + 4 + i * 24, *poss[i], frames[i], p)

def events_of(m):
    """-> (length, [event dicts]) of a motion: u16 length at 2, u8 count at 1, events at 148 (u64 attr, u16 frame,
    u16 volume offset in words, u32 arg; attr bit 0: a hit window opens, with a volume x y z w r0 r1 r2)"""
    count, length = m[1], struct.unpack_from("<H", m, 2)[0]
    evs = []
    for i in range(count):
        at = 148 + i * 16
        attr, frame, shape, arg = struct.unpack_from("<QHHI", m, at)
        e = {"frame": frame, "attr": "%016x" % attr, "arg": arg}
        if attr & 1 and shape * 4 + 28 <= len(m):
            v = struct.unpack_from("<7f", m, shape * 4)
            e["volume"] = {"kind": "sphere" if attr >> 32 & 1 else "box", "at": shape * 4,
                           "position": list(v[0:3]), "w": v[3], "radii": list(v[4:7])}
        evs.append(e)
    return length, evs

def take_events(m, evs):
    """zeroes the length, the events' attr / frame / arg (their volume offsets stay) and their volumes"""
    m[2:4] = bytes(2)
    for i in range(len(evs)):
        at = 148 + i * 16
        m[at:at + 10] = bytes(10)
        m[at + 12:at + 16] = bytes(4)
    for e in evs:
        if "volume" in e:
            a = e["volume"]["at"]
            m[a:a + 28] = bytes(28)

def put_events(m, length, evs):
    if len(evs) != m[1]:
        raise ValueError("the number of events of a motion is fixed (%d)" % m[1])
    struct.pack_into("<H", m, 2, length)
    for i, e in enumerate(evs):
        at = 148 + i * 16
        struct.pack_into("<QH", m, at, int(e["attr"], 16), e["frame"])
        struct.pack_into("<I", m, at + 12, e["arg"])
        if "volume" in e:
            v = e["volume"]
            struct.pack_into("<7f", m, v["at"], *(v["position"] + [v["w"]] + v["radii"]))

def convert_bank(d, model=None, sub=None):
    """d: the bank file; model: (model.gltf text, folder of the model relative to the bank's folder) for the nodes and
    the mesh; sub(data, entry): the extraction of an entry that is not a motion ({file name: bytes} or None).
    -> {file name: bytes} for the bank's folder, or None"""
    import json
    import bpe as B
    ents = bank_entries(d)
    if not ents:
        return None
    unpacked = []
    for a, b in ents:
        try:
            unpacked.append(B.expand(d[a:b]))
        except (AssertionError, IndexError, struct.error):
            return None  # not a bank after all
    files = {}
    if model:
        g = json.loads(model[0])
        for b in g["buffers"]:
            b["uri"] = model[1] + "/" + b["uri"]
        for im in g.get("images", []):
            im["uri"] = model[1] + "/" + im["uri"]
    else:
        g = {"asset": {"version": "2.0"}, "scene": 0, "scenes": [{"nodes": [0]}],
             "nodes": [{"name": "root", "rotation": [1.0, 0.0, 0.0, 0.0]}], "buffers": [], "bufferViews": [], "accessors": []}
    g["asset"]["generator"] = "Tenkaichi3Rust tools/assets/motion.py"
    root = next(i for i, n in enumerate(g["nodes"]) if n.get("name") == "root")
    node_of = {int(n["name"][5:]): i for i, n in enumerate(g["nodes"]) if n.get("name", "").startswith("node_")}
    binb = bytearray()
    buf = len(g["buffers"])
    def add(data, ctype, count, typ, mm=None):
        while len(binb) % 4:
            binb.append(0)
        g["bufferViews"].append({"buffer": buf, "byteOffset": len(binb), "byteLength": len(data)})
        binb.extend(data)
        a = {"bufferView": len(g["bufferViews"]) - 1, "componentType": ctype, "count": count, "type": typ}
        if mm:
            a["min"], a["max"] = mm
        g["accessors"].append(a)
        return len(g["accessors"]) - 1
    anims = []
    events = []
    lines = []
    sources = []
    is_motion_bank = len(ents) == 440
    for k, (a, b) in enumerate(ents, 1):
        data, tail = unpacked[k - 1]
        m = bytearray(data)
        kind = "data"
        if is_motion_bank and k <= 414 and len(data) and motion_ok(data):
            kind = "motion"
            samplers, channels = [], []
            exported = set()
            for tr in tracks(data):
                node, at, fl, n, keys = tr
                frames = [kk[0] for kk in keys]
                if any(y <= x for x, y in zip(frames, frames[1:])):
                    continue  # glTF needs increasing times: the track stays in the entry as it is
                if any(int(round(float(np.float32(f / 30.0)) * 30.0)) != f for f in frames):
                    continue  # a frame a float time cannot carry exactly (some tracks end on 0xFFFFFF88)
                if node not in node_of:
                    g["nodes"].append({"name": "node_%03d" % node})
                    node_of[node] = len(g["nodes"]) - 1
                    g["nodes"][root].setdefault("children", []).append(node_of[node])
                times = [float(np.float32(f / 30.0)) for f in frames]
                inp = add(struct.pack("<%df" % n, *times), 5126, n, "SCALAR", ([times[0]], [times[-1]]))
                rots = []
                prev = None
                for kk in keys:
                    q = unpack(kk[2])
                    if prev is not None and sum(float(x) * float(y) for x, y in zip(q, prev)) < 0:
                        q = [F(-x) for x in q]  # the same rotation, without a turn back between keys
                    prev = q
                    rots.append(struct.pack("<4f", *q))
                samplers.append({"input": inp, "output": add(b"".join(rots), 5126, n, "VEC4"), "interpolation": "LINEAR"})
                channels.append({"sampler": len(samplers) - 1, "target": {"node": node_of[node], "path": "rotation"}})
                exported.add(node)
                if not fl & 1:
                    pos = b"".join(struct.pack("<3f", *kk[1]) for kk in keys)
                    samplers.append({"input": inp, "output": add(pos, 5126, n, "VEC3"), "interpolation": "LINEAR"})
                    channels.append({"sampler": len(samplers) - 1, "target": {"node": node_of[node], "path": "translation"}})
                take(m, tr)
            if channels:
                anims.append({"name": "motion_%03d" % k, "samplers": samplers, "channels": channels})
            length, evs = events_of(data)
            import math
            vals = [x for e in evs if "volume" in e for x in e["volume"]["position"] + [e["volume"]["w"]] + e["volume"]["radii"]]
            if all(math.isfinite(x) for x in vals):
                take_events(m, evs)
                events.append({"entry": k, "length": length, "events": evs})
        inner = sub(data, k) if sub is not None and kind == "data" and len(data) else None
        if kind == "motion":
            # the structure of a motion entry (header, tracks, keys no glTF track has) as typed rows of
            # motion-sources.json (convert_native_motion_sources.py)
            import convert_native_motion_sources as native
            try:
                row = native.describe(bytes(m), exported)
                row["entry"] = k
                json.dumps(row, allow_nan=False)
                sources.append(row)
                lines.append("%d %s tail %s" % (k, kind, tail.hex()))
                continue
            except (ValueError, struct.error, IndexError, OverflowError):
                pass
        if not inner and len(m):
            # a motion entry: what its keys and events do not give as typed records (filetables.py)
            import filetables
            rest = filetables.convert(data, m)
            inner = {"skeleton.txt": ("# the entry is all zeros but for what the other files give\nzeros %d\n" % len(m)).encode()}
            if rest:
                inner["file-tables.json"] = rest.encode()
        if inner:
            for name, v in inner.items():
                files["entry_%03d/%s" % (k, name)] = v
        else:
            files["entry_%03d.bin" % k] = bytes(m)
        lines.append("%d %s tail %s" % (k, kind, tail.hex()))
    if sources:
        empty = [k for k, (a, b) in enumerate(ents, 1) if not len(unpacked[k - 1][0])]
        files["motion-sources.json"] = (json.dumps({"version": 1, "entryCount": len(ents), "motions": sources,
                                                    "emptyEntries": empty}, separators=(",", ":"), allow_nan=False) + "\n").encode()
    if events:
        files["events.json"] = json.dumps({"about": "events of the motions (tools/assets/motion.py): per motion entry its "
                                           "length (last frame) and events: frame, attr (bit 0: a hit window opens, with "
                                           "its volume; bit 1: it closes), arg (for a hit: the attack nodes), the volume "
                                           "(position, w, radii); the number of events is fixed",
                                           "motions": events}, indent=1).encode()
    if anims:
        g["animations"] = anims
        g["buffers"].append({"uri": "animations.bin", "byteLength": len(binb)})
        files["animations.gltf"] = json.dumps(g, separators=(",", ":")).encode()
        files["animations.bin"] = bytes(binb)
    files["bank.txt"] = ("# a bank of files each packed with the game's BPE (tools/assets/motion.py): entry, kind "
                         "(motion: its keys are in animations.gltf, animation motion_<entry>; the unpacked entry is entry_<n>.bin, "
                         "or a folder entry_<n>/ like a file's), the bytes after the packed data up to 64\nentries %d\n" % len(ents) + "\n".join(lines) + "\n").encode()
    return files

def apply_bank_events(m, k, doc):
    for mo in doc["motions"]:
        if mo["entry"] == k:
            put_events(m, mo["length"], mo["events"])

def apply_bank_entry(m, k, g, bins):
    """the keys of animation motion_<k> of the glTF g back into the unpacked entry m (bytearray)"""
    anim = next((a for a in g.get("animations", []) if a.get("name") == "motion_%03d" % k), None)
    if not anim:
        return
    def acc(i):
        a = g["accessors"][i]
        v = g["bufferViews"][a["bufferView"]]
        c = {"SCALAR": 1, "VEC2": 2, "VEC3": 3, "VEC4": 4}[a["type"]]
        if a["componentType"] != 5126:
            raise ValueError("accessor %d is not float" % i)
        st = v.get("byteStride", 4 * c)
        base = v.get("byteOffset", 0) + a.get("byteOffset", 0)
        bb = bins[v["buffer"]]
        return [struct.unpack_from("<%df" % c, bb, base + j * st) for j in range(a["count"])]
    def acc_raw(i):
        a = g["accessors"][i]
        v = g["bufferViews"][a["bufferView"]]
        st = v.get("byteStride", 12)
        base = v.get("byteOffset", 0) + a.get("byteOffset", 0)
        return [struct.unpack_from("<3I", bins[v["buffer"]], base + j * st) for j in range(a["count"])]
    at_of = {}
    for node in range(71):
        at = struct.unpack_from("<H", m, 6 + node * 2)[0] * 4
        if at:
            at_of[node] = at
    per = {}
    for ch in anim["channels"]:
        s = anim["samplers"][ch["sampler"]]
        node = int(g["nodes"][ch["target"]["node"]]["name"][5:])
        per.setdefault(node, {})[ch["target"]["path"]] = s
    for node, paths in per.items():
        if node not in at_of:
            raise ValueError("motion_%03d: node %d has no track in the entry" % (k, node))
        rs = paths["rotation"]
        frames = [int(round(t[0] * 30.0)) for t in acc(rs["input"])]
        rots = acc(rs["output"])
        poss = acc_raw(paths["translation"]["output"]) if "translation" in paths else None
        fl = struct.unpack_from("<H", m, at_of[node])[0]
        if not fl & 1 and poss is None:
            raise ValueError("motion_%03d: node %d needs translations" % (k, node))
        write_keys(m, at_of[node], frames, rots, poss)
