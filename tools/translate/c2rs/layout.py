"""Record layouts from clang -fdump-record-layouts-complete (the i686 -malign-double layouts the port uses)."""
import re

HDR = re.compile(r"^\s*(\d+)\s*\|\s*(struct|union)\s+(.*?)\s*$")
FLD = re.compile(r"^\s*(\d+)(?::(\d+)-(\d+))?\s*\|(\s+)(.*?)\s*$")
END = re.compile(r"\[sizeof=(\d+),.*?align=(\d+)")

def key_of(rest):
    rest = rest.strip()
    if "(unnamed" in rest or "(anonymous" in rest:
        m = re.search(r"at ([^)]+)\)", rest)
        return "@" + m.group(1)
    return rest.split()[0]

def parse(text):
    """-> {(tag, key): {'size', 'align', 'fields': [(offset_bytes, bit_first or None, bit_last, name)]}}"""
    out = {}
    blocks = text.split("*** Dumping AST Record Layout")
    for b in blocks[1:]:
        lines = [l for l in b.splitlines() if l.strip()]
        m = HDR.match(lines[0])
        if not m:
            continue
        tag, key = m.group(2), key_of(m.group(3))
        rows = []
        size = align = None
        for l in lines[1:]:
            e = END.search(l)
            if e:
                size, align = int(e.group(1)), int(e.group(2))
                break
            f = FLD.match(l)
            if f:
                rows.append((len(f.group(4)), int(f.group(1)), f.group(2), f.group(3), f.group(5)))
        fields = collect(rows, 0, len(rows), rows[0][0] if rows else 0, 0, out)
        out[(tag, key)] = {"size": size, "align": align, "fields": fields}
    return out

REC = re.compile(r"^(struct|union)\s+(.*?)\s*(?:([A-Za-z_]\w*))?$")

def collect(rows, a, b, indent, base, out):
    fields = []
    i = a
    while i < b:
        ind, off, b0, b1, body = rows[i]
        j = i + 1
        while j < b and rows[j][0] > ind:
            j += 1
        name = ""
        rm = re.match(r"^(struct|union)\s+((?:\w+::)*\((?:unnamed|anonymous)[^)]*\)|\w+)\s*(\w*)$", body)
        if rm:
            name = rm.group(3)
            if j > i + 1:  # nested record: keep its layout too (anonymous ones are only dumped here)
                sub = collect(rows, i + 1, j, rows[i + 1][0], off, out)
                k = (rm.group(1), key_of(rm.group(2)))
                if k not in out:
                    end = max([s[0] + 1 for s in sub] + [0])
                    out[k] = {"size": None, "align": None, "fields": sub}
        else:
            parts = body.rsplit(" ", 1)
            name = parts[1] if len(parts) == 2 and re.match(r"^[A-Za-z_]\w*$", parts[1]) else ""
            if b0 is not None and len(body.split()) == 1:
                name = ""
        fields.append((off - base, int(b0) if b0 is not None else None, int(b1) if b1 is not None else None, name))
        i = j
    return fields
