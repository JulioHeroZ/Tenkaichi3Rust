"""Lip sync keys of the voice lines (LipSync_Start, src/battle/view_b_c.c): directory entries "LIPS", u32 2, u32 key
count, u32 0, then the keys (u16 frame, u16 mouth open) and zeros to the end of the entry. lips.json holds the keys;
the rebuild writes the header ("LIPS", 2, count, 0) and the keys back (they have to fit in the entry: `room` keys)."""
import struct, json

def entries(d, base=0, end=None):
    """the lip sync entries of the directory at `base` (offsets from it; the last entry ends at `end`)"""
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
        if b - a < 16 or d[a:a + 4] != b"LIPS":
            continue
        ver, cnt, z = struct.unpack_from("<3I", d, a + 4)
        if ver != 2 or z != 0 or 16 + cnt * 4 > b - a or d[a + 16 + cnt * 4:b] != bytes(b - a - 16 - cnt * 4):
            continue
        keys = [list(struct.unpack_from("<2H", d, a + 16 + k * 4)) for k in range(cnt)]
        out.append((i, a, (b - a - 16) // 4, keys))
    return out


def all_entries(d):
    """the lip sync entries of the file's directory and of the directories inside its entries"""
    out, seen = [], set()

    def walk(base, end, depth):
        for e in entries(d, base, end):
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
            if o[i + 1] - o[i] >= 8 and o[i] > base and d[o[i]:o[i] + 4] != b"LIPS":
                walk(o[i], o[i + 1], depth + 1)

    walk(0, len(d), 0)
    return out


def convert(d, skel):
    es = all_entries(d)
    if not es:
        return None
    for i, a, room, keys in es:
        skel[a:a + 16] = bytes(16)
        skel[a + 16:a + 16 + room * 4] = bytes(room * 4)
    doc = {"about": "lip sync of voice lines (tools/assets/lipsconv.py): keys are [frame, mouth open]; the mouth "
                    "follows the key whose frame range the time is in; the last key's frame is the length. `at` is the "
                    "entry in the file, `room` how many keys fit",
           "lips": [{"entry": i, "at": a, "room": room, "keys": keys} for i, a, room, keys in es]}
    return json.dumps(doc, indent=1)

def apply(buf, text):
    for e in json.loads(text)["lips"]:
        keys = e["keys"]
        if len(keys) > e["room"]:
            raise ValueError("lips of entry %d: %d keys, room %d" % (e["entry"], len(keys), e["room"]))
        a = e["at"]
        buf[a:a + 4] = b"LIPS"
        struct.pack_into("<3I", buf, a + 4, 2, len(keys), 0)
        for k, (fr, op) in enumerate(keys):
            struct.pack_into("<2H", buf, a + 16 + k * 4, fr, op)
