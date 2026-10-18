"""Camera animations (DemoCamAnim, include/battle/btl_demo_cam.h): directory entries "CMAn". Offsets from the
entry's start: +8 channels (s32 pose word, keys, key count, -), +0xC channel count, +0x14 length, +0x18 the default
pose (x y z, rotation x y z, s32 frame, f32). A key is 32 bytes: u32 flags (0x10: hold until the next key), f32 time,
f32 value, then words the game does not read (kept in the file). cameras.json holds the default poses and the keys;
the rebuild writes them back (the same number of keys per channel)."""
import struct, json, math

def dir_entries(d, base=0, end=None):
    """the cameras of the directory at `base` (offsets from it; the last entry ends at `end`)"""
    end = len(d) if end is None else end
    if end - base < 8:
        return []
    n = struct.unpack_from("<I", d, base)[0]
    if not 1 <= n <= 4096 or (n + 1) * 4 > end - base:
        return []
    o = [base + (struct.unpack_from("<I", d, base + 4 * i)[0] & ~3) for i in range(1, n + 1)] + [end]
    if any(o[i] > o[i + 1] or o[i + 1] > end for i in range(n)):
        return []
    out = []
    for i in range(n):
        a, b = o[i], o[i + 1]
        if b - a < 0x24 or d[a:a + 4] != b"CMAn":
            continue
        try:
            chs, cc, length, dfl = struct.unpack_from("<IIIf I", d, a + 8)[0], struct.unpack_from("<I", d, a + 12)[0], \
                struct.unpack_from("<f", d, a + 0x14)[0], struct.unpack_from("<I", d, a + 0x18)[0]
            if cc > 64 or a + chs + cc * 16 > b or a + dfl + 32 > b:
                continue
            pose = list(struct.unpack_from("<6fif", d, a + dfl))
            channels = []
            for c in range(cc):
                idx, keys, kc, _ = struct.unpack_from("<iIii", d, a + chs + c * 16)
                if not 0 <= kc <= 4096 or a + keys + kc * 32 > b:
                    raise ValueError
                ks = [list(struct.unpack_from("<Iff", d, a + keys + k * 32)) for k in range(kc)]
                channels.append({"word": idx, "keys": ks, "_at": a + keys})
            vals = [x for x in pose if isinstance(x, float)] + [k[1] for c in channels for k in c["keys"]] + [k[2] for c in channels for k in c["keys"]]
            if not all(math.isfinite(x) for x in vals):
                continue
            out.append((i, a, a + dfl, pose, channels, a + chs))
        except (ValueError, struct.error):
            continue
    return out

def entries(d):
    """the cameras of the file's directory and of the directories inside its entries (down to depth 3)"""
    out, seen = [], set()

    def walk(base, end, depth):
        for e in dir_entries(d, base, end):
            if e[1] not in seen:
                seen.add(e[1])
                out.append(e)
        if depth >= 3 or end - base < 8:
            return
        n = struct.unpack_from("<I", d, base)[0]
        if not 1 <= n <= 4096 or (n + 1) * 4 > end - base:
            return
        o = [base + (struct.unpack_from("<I", d, base + 4 * i)[0] & ~3) for i in range(1, n + 1)] + [end]
        if any(o[i] > o[i + 1] or o[i + 1] > end for i in range(n)):
            return
        for i in range(n):
            if o[i + 1] - o[i] >= 8 and o[i] > base and d[o[i]:o[i] + 4] != b"CMAn":
                walk(o[i], o[i + 1], depth + 1)

    walk(0, len(d), 0)
    return out


def convert(d, skel):
    es = entries(d)
    if not es:
        return None
    doc = {"about": "camera animations (tools/assets/camconv.py): `pose` is the default pose (x y z, rotation x y z in "
                    "radians applied X, Y, Z, frame, -), each channel drives pose word `word` with keys [flags, time, "
                    "value] (flags 0x10: hold the value until the next key, else linear)", "cameras": []}
    for i, a, pa, pose, channels, ca in es:
        skel[pa:pa + 32] = bytes(32)
        for c in channels:
            for k in range(len(c["keys"])):
                skel[c["_at"] + k * 32:c["_at"] + k * 32 + 12] = bytes(12)
        doc["cameras"].append({"entry": i, "at": a, "pose_at": pa, "pose": pose, "channels_at": ca,
                               "channels": [{"word": c["word"], "keys_at": c["_at"], "keys": c["keys"]} for c in channels]})
    return json.dumps(doc, indent=1)

def apply(buf, text):
    for cam in json.loads(text)["cameras"]:
        p = cam["pose"]
        struct.pack_into("<6fif", buf, cam["pose_at"], *p)
        if len(cam["channels"]) != struct.unpack_from("<I", buf, cam["at"] + 12)[0]:
            raise ValueError("camera of entry %d: the number of channels is fixed" % cam["entry"])
        for ci, c in enumerate(cam["channels"]):
            if len(c["keys"]) != struct.unpack_from("<i", buf, cam["channels_at"] + ci * 16 + 8)[0]:
                raise ValueError("camera of entry %d: the number of keys of a channel is fixed" % cam["entry"])
            for k, (fl, t, v) in enumerate(c["keys"]):
                struct.pack_into("<Iff", buf, c["keys_at"] + k * 32, fl, t, v)


def tables(d, skel):
    """The rest of each camera (its header, its channel table, the key words the game does not read) as typed
    records: camera-tables.json text or None (zeroed in skel)."""
    import layout
    es = entries(d)
    if not es:
        return None
    lay = layout.Layout(0)
    lay.type("CameraHeader", 0x20, [("magic", 0, "u8[4]"), ("version", 4, "u32"), ("channels", 8, "u32"),
                                     ("channelCount", 12, "u32"), ("length", 0x14, "f32"), ("pose", 0x18, "u32"),
                                     ("size", 0x1C, "u32")])
    lay.type("CameraChannel", 0x10, [("word", 0, "s32"), ("keys", 4, "u32"), ("keyCount", 8, "s32")])
    lay.type("CameraKeyRest", 0x14, [("unread", 0, "u32[5]")])
    for i, a, pa, pose, channels, ca in es:
        lay.read(d, "CameraHeader", a)
        for c in range(len(channels)):
            lay.read(d, "CameraChannel", ca + 16 * c)
        for c in channels:
            for k in range(len(c["keys"])):
                lay.read(d, "CameraKeyRest", c["_at"] + 32 * k + 12)
    layout.finish(lay, d, skel)
    return lay.text("the rest of the cameras (tools/assets/camconv.py): header (offsets from the entry's start), channel "
                    "table, and per key the 20 bytes after flags, time and value, which the game does not read")
