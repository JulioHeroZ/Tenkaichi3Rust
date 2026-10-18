"""The game's texts: directory entries that are UTF-16 text (a byte order mark FF FE, the text, then zeros to the
end of the entry). texts.json holds them as ordinary text; the rebuild writes each back in UTF-16 (it has to fit in
the entry: `room` bytes, the zero at the end included)."""
import struct, json

def entries(d, base=0, end=None):
    """the texts of the directory at `base` (offsets from it; the last entry ends at `end`)"""
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
        e = d[a:b]
        if e[:2] != b"\xff\xfe":
            continue
        k = 2
        while k + 1 < len(e) and (e[k] | e[k + 1]):
            k += 2
        if k + 1 >= len(e) or e[k:] != bytes(len(e) - k):
            continue
        try:
            t = e[2:k].decode("utf-16-le")
        except UnicodeDecodeError:
            continue
        if t.encode("utf-16-le") != e[2:k]:
            continue
        out.append((i, a + 2, b - a - 2, t))
    return out

def convert(d, skel):
    """-> texts.json text or None; the texts (and their byte order marks) are zeroed in skel. Texts of the file's
    directory and of the directories inside its entries (the menus' text tables)."""
    es, seen = [], set()

    def walk(base, end, depth):
        for e in entries(d, base, end):
            if e[1] not in seen:
                seen.add(e[1])
                es.append(e)
        if depth >= 3 or end - base < 8:
            return
        n = struct.unpack_from("<I", d, base)[0]
        if not 1 <= n <= 4096 or (n + 1) * 4 > end - base:
            return
        o = [base + (struct.unpack_from("<I", d, base + 4 * i)[0] & ~3) for i in range(1, n + 1)] + [end]
        if any(o[i] > o[i + 1] or o[i + 1] > end for i in range(n)):
            return
        for i in range(n):
            if o[i + 1] - o[i] >= 8 and o[i] > base:
                walk(o[i], o[i + 1], depth + 1)

    walk(0, len(d), 0)
    if not es:
        return None
    for i, at, room, t in es:
        skel[at - 2:at + room] = bytes(room + 2)
    doc = {"about": "texts of the file (tools/assets/textconv.py): edit `text`; `at` is where it goes in the file and "
                    "`room` how many bytes of UTF-16 it can take, the final zero included; bom: the byte order mark before it is written too",
           "texts": [{"entry": i, "at": at, "room": room, "bom": 1, "text": t} for i, at, room, t in es]}
    return json.dumps(doc, ensure_ascii=False, indent=1)

def apply(buf, text):
    for e in json.loads(text)["texts"]:
        b = e["text"].encode("utf-16-le")
        if len(b) + 2 > e["room"]:
            raise ValueError("text of entry %d is too long (%d bytes, room %d)" % (e["entry"], len(b) + 2, e["room"]))
        buf[e["at"]:e["at"] + len(b)] = b
        if e.get("bom"):
            buf[e["at"] - 2:e["at"]] = b"\xff\xfe"
