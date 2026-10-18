"""The menus' Flash-like movies (FOD files: "FOD" 0x11 "LIT", read by src/sys/gfxm_b_c.c and gfxm_c.c) as
movies.json, so none of their bytes stay in the skeleton.

A movie is a list of tags {code u8, unk u8, count u16, size u32, data}, ended by code 0. The tags of the game's
movies hold records with the same 8-byte header (the record's index, unk, a number n, size):

* tag 2, the sprite list: per record u16 `id` (the record of tag 6 with its frames);
* tag 3, the images: u16 `w`, u16 `h` (the image's texture is the movie's texture n);
* tag 4, the shapes: u16 count, then quads {s16 image (-1: flat), u16 unk02, u32 rgba (r in the top byte),
  s32 x0 x1 y0 y1};
* tag 6, the sprites' timelines, and tag 7, the root's (one record): n = the number of depths, then frame blocks
  {name, u16 size, tags {u16 code, data} up to code 1 (ShowFrame)}, the last block a name and size 0.

Frame tags: 4 PlaceObject (u16 depth, u8 kind: 4 shape / 6 sprite, u16 id, matrix, for a sprite a colour
transform), 26 PlaceObject2 (u8 flags: 2 new, 4 matrix, 8 colour transform, 0x20 name with a new object, other bits unread; u16 depth; new: u8 kind,
u16 id), 5 RemoveObject (u16 depth, u8 kind, u16 id), 28 RemoveObject2 (u16 depth), 12 DoAction (actions up to 0:
0x81 GotoFrame u16, 0x83 GetURL two strings, 0x8A WaitForFrame 3 bytes, 0x8B SetTarget string, 0x8C GotoLabel string,
4 5 6 7 none). A matrix is a flag byte and bit fields (a 5-bit width, then the values: scale x y for flags 1 / 0x10,
skew for 2 / 8, translation for 4 / 0x20) kept as the numbers they are; a colour transform likewise (4-bit width,
multipliers for flags 1..8, offsets for 0x10..0x80). Strings are Latin-1.
"""
import json, struct


class Bits:
    def __init__(self, d, o):
        self.d, self.o, self.p = d, o, 0

    def u(self, n):
        v = 0
        for _ in range(n):
            v = v << 1 | (self.d[self.o + (self.p >> 3)] >> (7 - (self.p & 7))) & 1
            self.p += 1
        return v

    def s(self, n):
        v = self.u(n)
        return v - (1 << n) if n and v >> (n - 1) & 1 else v

    def end(self):
        return self.o + (self.p + 7) // 8


class Writer:
    def __init__(self):
        self.b = bytearray()
        self.bits = []

    def put(self, v, n):
        for i in range(n - 1, -1, -1):
            self.bits.append((v >> i) & 1)

    def flush(self):
        while len(self.bits) % 8:
            self.bits.append(0)
        for i in range(0, len(self.bits), 8):
            x = 0
            for b in self.bits[i:i + 8]:
                x = x << 1 | b
            self.b.append(x)
        self.bits = []


def cstr(d, o):
    e = d.index(b"\0", o)
    return d[o:e].decode("latin-1"), e + 1


def read_mtx(d, o):
    f = d[o]
    b = Bits(d, o + 1)
    m = {"f": f}
    for key, a, c in (("s", 1, 0x10), ("r", 2, 8), ("t", 4, 0x20)):
        if f & (a | c):
            n = b.u(5)
            m[key] = [n] + [b.s(n) for bit in (a, c) if f & bit]
    return m, b.end()


def write_mtx(w, m):
    f = m["f"]
    w.b.append(f)
    for key, a, c in (("s", 1, 0x10), ("r", 2, 8), ("t", 4, 0x20)):
        if f & (a | c):
            n = m[key][0]
            w.put(n, 5)
            for v in m[key][1:]:
                w.put(v & ((1 << n) - 1), n)
    w.flush()


def read_cx(d, o):
    f = d[o]
    b = Bits(d, o + 1)
    cx = {"f": f}
    if f:
        n = b.u(4)
        cx["n"] = n
        cx["v"] = [b.s(n) for i in range(8) if f >> i & 1]
    return cx, b.end()


def write_cx(w, cx):
    w.b.append(cx["f"])
    if cx["f"]:
        n = cx["n"]
        w.put(n, 4)
        for v in cx["v"]:
            w.put(v & ((1 << n) - 1), n)
    w.flush()


def read_actions(d, o):
    out = []
    while True:
        c = d[o]
        o += 1
        if c == 0:
            return out, o
        if c == 0x81:
            out.append([c, struct.unpack_from("<H", d, o)[0]])
            o += 2
        elif c == 0x83:
            a, o = cstr(d, o)
            b, o = cstr(d, o)
            out.append([c, a, b])
        elif c == 0x8A:
            out.append([c, list(d[o:o + 3])])
            o += 3
        elif c in (0x8B, 0x8C):
            a, o = cstr(d, o)
            out.append([c, a])
        else:
            out.append([c])  # 4 5 6 7, and codes the game does not know (read as one byte, as it does)


def write_actions(w, acts):
    for a in acts:
        w.b.append(a[0])
        if a[0] == 0x81:
            w.b += struct.pack("<H", a[1])
        elif a[0] == 0x83:
            w.b += a[1].encode("latin-1") + b"\0" + a[2].encode("latin-1") + b"\0"
        elif a[0] == 0x8A:
            w.b += bytes(a[1])
        elif a[0] in (0x8B, 0x8C):
            w.b += a[1].encode("latin-1") + b"\0"
    w.b.append(0)


def read_frame_tags(d, o):
    tags = []
    while True:
        code = struct.unpack_from("<H", d, o)[0]
        o += 2
        t = {"t": code}
        if code == 1:
            tags.append(t)
            return tags, o
        if code == 4:
            t["depth"], t["kind"], t["id"] = struct.unpack_from("<HBH", d, o)
            o += 5
            if t["kind"] not in (4, 6):
                raise ValueError("place kind")
            t["m"], o = read_mtx(d, o)
            if t["kind"] == 6:
                t["cx"], o = read_cx(d, o)
        elif code == 26:
            f, t["depth"] = struct.unpack_from("<BH", d, o)
            o += 3
            t["flags"] = f
            if f & 2:
                t["kind"], t["id"] = struct.unpack_from("<BH", d, o)
                o += 3
            if f & 4:
                t["m"], o = read_mtx(d, o)
            if f & 8:
                t["cx"], o = read_cx(d, o)
            if f & 0x20 and f & 2:
                t["name"], o = cstr(d, o)  # (the game reads a name only with a new object)
        elif code == 5:
            t["depth"], t["kind"], t["id"] = struct.unpack_from("<HBH", d, o)
            o += 5
        elif code == 28:
            t["depth"] = struct.unpack_from("<H", d, o)[0]
            o += 2
        elif code == 12:
            t["actions"], o = read_actions(d, o)
        else:
            raise ValueError("frame tag %d" % code)
        tags.append(t)


def write_frame_tags(w, tags):
    for t in tags:
        w.b += struct.pack("<H", t["t"])
        c = t["t"]
        if c == 4:
            w.b += struct.pack("<HBH", t["depth"], t["kind"], t["id"])
            write_mtx(w, t["m"])
            if t["kind"] == 6:
                write_cx(w, t["cx"])
        elif c == 26:
            f = t["flags"]
            w.b += struct.pack("<BH", f, t["depth"])
            if f & 2:
                w.b += struct.pack("<BH", t["kind"], t["id"])
            if f & 4:
                write_mtx(w, t["m"])
            if f & 8:
                write_cx(w, t["cx"])
            if f & 0x20 and f & 2:
                w.b += t["name"].encode("latin-1") + b"\0"
        elif c == 5:
            w.b += struct.pack("<HBH", t["depth"], t["kind"], t["id"])
        elif c == 28:
            w.b += struct.pack("<H", t["depth"])
        elif c == 12:
            write_actions(w, t["actions"])


def read_frames(d, o, end):
    frames = []
    while True:
        name, o = cstr(d, o)
        size = struct.unpack_from("<H", d, o)[0]
        o += 2
        if size == 0:
            return frames, name, o
        tags, e = read_frame_tags(d, o)
        fr = {"name": name, "tags": tags}
        if size != e - o:
            fr["size"] = size  # (only when it is not the length of the tags)
        frames.append(fr)
        o = e
        if o > end:
            raise ValueError("frames past the record")


def write_frames(w, frames, last):
    for fr in frames:
        w.b += fr["name"].encode("latin-1") + b"\0"
        at = len(w.b)
        w.b += b"\0\0"
        write_frame_tags(w, fr["tags"])
        struct.pack_into("<H", w.b, at, fr.get("size", len(w.b) - at - 2))
    w.b += last.encode("latin-1") + b"\0\0\0"


def read_record(d, o, code):
    idx, unk, n, size = struct.unpack_from("<BBHI", d, o)
    r = {"i": idx, "u": unk, "n": n}
    p, e = o + 8, o + 8 + size
    if code == 2:
        if size != 2:
            raise ValueError("sprite record")
        r["id"] = struct.unpack_from("<H", d, p)[0]
    elif code == 3:
        if size != 4:
            raise ValueError("image record")
        r["w"], r["h"] = struct.unpack_from("<HH", d, p)
    elif code == 4:
        cnt = struct.unpack_from("<H", d, p)[0]
        if size != 2 + 24 * cnt:
            raise ValueError("shape record")
        r["quads"] = [list(struct.unpack_from("<hHIiiii", d, p + 2 + 24 * k)) for k in range(cnt)]
    elif code in (6, 7):
        r["frames"], r["last"], q = read_frames(d, p, e)
        if q != e:
            raise ValueError("frames do not end the record")
    else:
        raise ValueError("tag %d" % code)
    return r, e


def write_record(w, r, code):
    w.b += struct.pack("<BBH", r["i"], r["u"], r["n"])
    at = len(w.b)
    w.b += b"\0\0\0\0"
    if code == 2:
        w.b += struct.pack("<H", r["id"])
    elif code == 3:
        w.b += struct.pack("<HH", r["w"], r["h"])
    elif code == 4:
        w.b += struct.pack("<H", len(r["quads"]))
        for q in r["quads"]:
            w.b += struct.pack("<hHIiiii", *q)
    else:
        write_frames(w, r["frames"], r["last"])
    struct.pack_into("<I", w.b, at, len(w.b) - at - 4)


def read_movie(d, o):
    if d[o:o + 8] != b"FOD\x11LIT\0":
        return None
    p = o + 8
    tags = []
    while True:
        code, unk, cnt, size = struct.unpack_from("<BBHI", d, p)
        e = p + 8 + size
        if e > len(d):
            raise ValueError("tag past the file")
        t = {"code": code, "u": unk, "count": cnt}
        if code == 0:
            if size:
                raise ValueError("end tag with data")
            tags.append(t)
            return {"at": o, "tags": tags}, e
        q = p + 8
        nrec = 1 if code == 7 else cnt
        t["records"] = []
        for _ in range(nrec):
            r, q = read_record(d, q, code)
            t["records"].append(r)
        if q != e:
            raise ValueError("records do not fill tag %d" % code)
        tags.append(t)
        p = e


def write_movie(m):
    w = Writer()
    w.b += b"FOD\x11LIT\0"
    for t in m["tags"]:
        w.b += struct.pack("<BBH", t["code"], t["u"], t["count"])
        at = len(w.b)
        w.b += b"\0\0\0\0"
        for r in t.get("records", []):
            write_record(w, r, t["code"])
        struct.pack_into("<I", w.b, at, len(w.b) - at - 4)
    return bytes(w.b)


def convert(d, skel):
    """-> movies.json text (the movies zeroed in skel) or None."""
    import texconv
    movies = []
    for base in sorted(texconv.dir_bases(d)):
        if d[base:base + 8] != b"FOD\x11LIT\0":
            continue
        try:
            m, e = read_movie(d, base)
            if write_movie(m) != d[base:e]:
                raise ValueError("does not rebuild")
        except (ValueError, IndexError, struct.error, UnicodeDecodeError):
            continue
        movies.append(m)
        skel[base:e] = bytes(e - base)
    if not movies:
        return None
    return json.dumps({"about": "the Flash-like movies of the file (tools/assets/fodconv.py): tags, records, frames "
                       "and their place / remove / action tags; matrices and colour transforms as their bit fields",
                       "movies": movies}, separators=(",", ":"), ensure_ascii=False)


def apply(buf, text):
    for m in json.loads(text)["movies"]:
        b = write_movie(m)
        buf[m["at"]:m["at"] + len(b)] = b
