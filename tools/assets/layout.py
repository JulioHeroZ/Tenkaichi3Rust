"""Typed records: the open format for the structured data of the game's files that is neither picture, model nor
sound (tables, trees, parameters). A layout file (JSON) has

* "types": per record type its size and fields [name, byte offset, kind], kinds u8 u16 u32 s8 s16 s32 f32,
  arrays like "f32[3]" (a last field "u32[*]" has as many values as the record gives; the type's size is then the
  size of the fixed part), and "ptr": an offset counted in words from `base` (written as the file offset it points
  at, null for 0, -1 for 0xFFFFFFFF), as the game's relocation turns it into a pointer;
* "base": the file offset pointers count from;
* "records": each {"type", "at": file offset, field: value...}.

Names come from the decompilation's headers; a field whose meaning is not known is named unkXX after its offset (a
record of type "Words" is a run of words whose structure is not known yet). A float whose bits a JSON number does
not give back (NaN, infinities) is written {"bits": n}. host/assets.rs layout_apply writes every record at its
place; the converters check that the records give back every byte they take out of skeleton.bin.
"""
import json, math, struct

SIZES = {"u8": 1, "s8": 1, "u16": 2, "s16": 2, "u32": 4, "s32": 4, "f32": 4, "ptr": 4}
FMT = {"u8": "B", "s8": "b", "u16": "H", "s16": "h", "u32": "I", "s32": "i", "f32": "f", "ptr": "I"}


def short(v, bits):
    """the shortest decimal that gives these float bits back (as the rebuild reads it: a double made a float)"""
    if v == 0:
        return v
    for p in range(1, 10):
        x = float("%.*g" % (p, v))
        if struct.unpack("<I", struct.pack("<f", x))[0] == bits:
            return int(x) if x == int(x) and abs(x) < 1e15 else x
    return v


def kind_of(k):
    """'f32[3]' -> ('f32', 3); 'u32[*]' -> ('u32', '*'); 'u16' -> ('u16', None)"""
    if "[" in k:
        b, n = k[:-1].split("[")
        return b, n if n == "*" else int(n)
    return k, None


class Layout:
    def __init__(self, base=0):
        self.base = base
        self.types = {}
        self.records = []
        self.cover = []
        self.seen = set()

    def type(self, name, size, fields, tail=None):
        """fields: [(name, offset, kind)]; unnamed bytes in [0, size) get unkXX fields (u32 where aligned, else u8).
        tail: (name, kind) of a last variable field at `size`."""
        assert not {"type", "at"} & {f for f, _, _ in fields}, "type and at are the record's own keys"
        taken = bytearray(size)
        for f, o, k in fields:
            b, n = kind_of(k)
            taken[o:o + SIZES[b] * (n or 1)] = b"\1" * (SIZES[b] * (n or 1))
        full = list(fields)
        o = 0
        while o < size:
            if taken[o]:
                o += 1
                continue
            e = o
            while e < size and not taken[e]:
                e += 1
            while o < e:
                if o % 4 == 0 and e - o >= 4:
                    n = (e - o) // 4
                    full.append(("unk%02X" % o, o, "u32" if n == 1 else "u32[%d]" % n))
                    o += 4 * n
                else:
                    full.append(("unk%02X" % o, o, "u8"))
                    o += 1
        full.sort(key=lambda f: f[1])
        if tail:
            full.append((tail[0], size, tail[1] + "[*]"))
        self.types[name] = {"size": size, "fields": [list(f) for f in full]}

    def read(self, d, tname, at, count=None):
        """-> the record of type tname at `at` of d (added once), covering its bytes; count: of the variable field."""
        t = self.types[tname]
        if (tname, at) in self.seen:
            return None
        self.seen.add((tname, at))
        rec = {"type": tname, "at": at}
        size = t["size"]
        for f, o, k in t["fields"]:
            b, n = kind_of(k)
            if n == "*":
                size += SIZES[b] * count
                rec[f] = [self._value(d, b, at + o + SIZES[b] * i) for i in range(count)]
                continue
            vals = [self._value(d, b, at + o + SIZES[b] * i) for i in range(n or 1)]
            rec[f] = vals if n else vals[0]
        self.records.append(rec)
        self.cover.append((at, size))
        return rec

    def _value(self, d, b, o):
        v = struct.unpack_from("<" + FMT[b], d, o)[0]
        if b == "f32":
            bits = struct.unpack_from("<I", d, o)[0]
            if not math.isfinite(v) or struct.unpack("<I", struct.pack("<f", v))[0] != bits:
                return {"bits": bits}
            return short(v, bits)
        if b == "ptr":
            return None if v == 0 else -1 if v == 0xFFFFFFFF else self.base + v * 4
        return v

    def text(self, about):
        return json.dumps({"about": about, "base": self.base, "types": self.types, "records": self.records},
                          separators=(",", ":"), allow_nan=False)


def apply(buf, text):
    doc = json.loads(text) if isinstance(text, str) else text
    base, types = doc["base"], doc["types"]
    for rec in doc["records"]:
        t = types[rec["type"]]
        at = rec["at"]
        for f, o, k in t["fields"]:
            b, n = kind_of(k)
            vals = rec[f] if n else [rec[f]]
            for i, v in enumerate(vals):
                p = at + o + SIZES[b] * i
                if b == "ptr":
                    struct.pack_into("<I", buf, p, 0 if v is None else 0xFFFFFFFF if v == -1 else (v - base) // 4)
                elif b == "f32" and isinstance(v, dict):
                    struct.pack_into("<I", buf, p, v["bits"])
                else:
                    struct.pack_into("<" + FMT[b], buf, p, v)


def words(lay, d, skel, lo, hi, gap=16):
    """Runs of skel[lo:hi] that are not zero as "Words" records (runs closer than `gap` bytes joined; the bytes of
    a last word cut by the end of the file as a "Bytes" record)."""
    if "Words" not in lay.types:
        lay.type("Words", 0, [], tail=("w", "u32"))
        lay.type("Bytes", 0, [], tail=("b", "u8"))
    hi = min(hi, len(d))
    o = lo - lo % 4
    while o < hi:
        if not any(skel[o:o + 4]):
            o += 4
            continue
        s = z = o
        while o < hi and o - z < gap:
            if any(skel[o:o + 4]):
                z = o + 4
            o += 4
        z = min(z, hi)
        n = (z - s) // 4
        if n:
            lay.read(d, "Words", s, n)
        if (z - s) % 4:
            lay.read(d, "Bytes", s + 4 * n, (z - s) % 4)
        o = z


def finish(lay, d, skel):
    """Checks that the records give d's bytes back and zeroes what they cover in skel."""
    buf = bytearray(len(d))
    apply(buf, json.loads(lay.text("")))
    for at, n in lay.cover:
        if buf[at:at + n] != d[at:at + n]:
            raise ValueError("record at %d does not rebuild" % at)
        skel[at:at + n] = bytes(n)
