"""GSC scripts (src/sys/gsc.c) to text and back, exactly.

A script file is chunks: a 16-byte header {tag, 16, data size, arg}, the data, then a 16-byte "EOFC" trailer. GSCF
wraps the file; inside, GSHD (header words), GSCD (the code: GSAC chunks, one action each, arg = action id) and GSDT
(4-byte values the code's operands point at). script.txt writes the chunks as an indented tree; the words of an
action as `end`, `cmd <id> <operand count>`, `start <action>`, `call <action>`, `opt <letter> <operand count>`,
`@<index> <type>` (an operand: GSDT value, type 0 int / 1 float / 2 string; its value follows as a `; comment`)
or `word 0x........`; GSDT values as
`int`, `float` or `hex`. The rebuild writes the chunks back with their sizes from what is there, so actions and
values can be changed, added or removed.
"""
import struct

def f32(w):
    return struct.unpack("<f", struct.pack("<I", w))[0]

def word_text(w):
    k = w & 15
    if k == 0 and w == 0:
        return "end"
    if k == 1 and (w >> 4) & 15 == 0:
        return "cmd %d %d" % (w >> 16, (w >> 8) & 255)
    if k in (2, 3) and (w >> 4) & 0xfff == 0:
        return "%s %d" % ("start" if k == 2 else "call", struct.unpack("<h", struct.pack("<H", w >> 16))[0])
    if k == 8 and (w >> 4) & 15 == 0 and w >> 24 == 0 and 33 <= (w >> 8) & 255 < 127:
        return "opt %s %d" % (chr((w >> 8) & 255), (w >> 16) & 255)
    if k == 10:
        return "@%d %d" % (w >> 8, (w >> 4) & 15)
    return "word 0x%08x" % w

def word_of(t):
    f = t.split()
    if f[0] == "end":
        return 0
    if f[0] == "cmd":
        return 1 | int(f[2]) << 8 | int(f[1]) << 16
    if f[0] in ("start", "call"):
        return (2 if f[0] == "start" else 3) | (int(f[1]) & 0xffff) << 16
    if f[0] == "opt":
        return 8 | ord(f[1]) << 8 | int(f[2]) << 16
    if f[0].startswith("@"):
        return 10 | int(f[1]) << 4 | int(f[0][1:]) << 8
    if f[0] == "word":
        return int(f[1], 16)
    raise ValueError("unknown word: %s" % t)

def parse(d, p, end, out, depth, types):
    while p < end:
        tag, head, size, arg = struct.unpack_from("<4sIIi", d, p)
        if head != 16 or p + 16 + size + 16 > end or d[p + 16 + size:p + 20 + size] != b"EOFC":
            raise ValueError("chunk")
        if d[p + 20 + size:p + 32 + size] != b"\x10\0\0\0" + bytes(8):
            raise ValueError("trailer")
        name = tag.decode("latin1")
        if not name.isalnum():
            raise ValueError("tag")
        ind = " " * depth
        body = d[p + 16:p + 16 + size]
        if name in ("GSCF", "GSCD"):
            out.append("%s%s %d" % (ind, name, arg))
            parse(d, p + 16, p + 16 + size, out, depth + 1, types)
        elif name == "GSAC":
            if size % 4:
                raise ValueError("action size")
            out.append("%s%s %d" % (ind, name, arg))
            for i in range(size // 4):
                w = struct.unpack_from("<I", body, 4 * i)[0]
                t = word_text(w)
                if word_of(t) != w:
                    t = "word 0x%08x" % w
                if (w & 15) == 10:
                    types.setdefault(w >> 8, (w >> 4) & 15)
                out.append("%s %s" % (ind, t))
        elif name == "GSDT":
            if size % 4:
                raise ValueError("data size")
            out.append("%s%s %d" % (ind, name, arg))
            for i in range(size // 4):
                w = struct.unpack_from("<I", body, 4 * i)[0]
                ty = types.get(i)
                v = f32(w)
                if ty == 1 and struct.unpack("<I", struct.pack("<f", float(repr(v))))[0] == w and v == v:
                    out.append("%s %d float %r" % (ind, i, v))
                elif ty == 0:
                    out.append("%s %d int %d" % (ind, i, struct.unpack("<i", struct.pack("<I", w))[0]))
                else:
                    out.append("%s %d hex 0x%08x" % (ind, i, w))
        else:
            out.append("%s%s %d hex %s" % (ind, name, arg, body.hex()))
        p += 32 + size
    return p

def to_text(d):
    if d[:8] != b"GSCF\x10\0\0\0":
        return None
    size = struct.unpack_from("<I", d, 8)[0]
    if 32 + size > len(d):
        return None
    out, types = [], {}
    # GSDT comes after GSCD: the operand types are known when the values are written
    parse(d, 0, 32 + size, out, 0, types)
    # the value of each operand as a comment (`; ...`, ignored by the rebuild)
    vals = {}
    for l in out:
        g = l.split()
        if len(g) == 3 and g[0].isdigit() and g[1] in ("int", "float", "hex"):
            vals[int(g[0])] = g[2]
    out = [l + "  ; " + vals[int(l.split()[0][1:])] if l.strip().startswith("@") and int(l.split()[0][1:]) in vals else l for l in out]
    if d[32 + size:] != bytes(len(d) - 32 - size):
        return None
    return out, len(d)

def assemble(lines, total):
    """the script back from its text lines; padded with zeros to `total` bytes (the file's size)"""
    items = []  # (depth, line)
    for l in lines:
        l = l.split(";")[0].rstrip()
        if not l.strip() or l.startswith("#"):
            continue
        items.append((len(l) - len(l.lstrip(" ")), l.strip()))
    pos = [0]
    def chunk(depth):
        _, l = items[pos[0]]
        f = l.split()
        name, arg = f[0], int(f[1])
        pos[0] += 1
        body = b""
        if name in ("GSCF", "GSCD"):
            while pos[0] < len(items) and items[pos[0]][0] > depth:
                body += chunk(items[pos[0]][0])
        elif name == "GSAC":
            while pos[0] < len(items) and items[pos[0]][0] > depth:
                body += struct.pack("<I", word_of(items[pos[0]][1]))
                pos[0] += 1
        elif name == "GSDT":
            while pos[0] < len(items) and items[pos[0]][0] > depth:
                g = items[pos[0]][1].split()
                if g[1] == "float":
                    body += struct.pack("<f", float(g[2]))
                elif g[1] == "int":
                    body += struct.pack("<i", int(g[2]))
                else:
                    body += struct.pack("<I", int(g[2], 16))
                pos[0] += 1
        else:
            body = bytes.fromhex(f[3]) if len(f) > 3 else b""
        return struct.pack("<4sIIi", name.encode("latin1"), 16, len(body), arg) + body + b"EOFC" + struct.pack("<I", 16) + bytes(8)
    out = chunk(items[0][0])
    if len(out) > total:
        raise ValueError("the script is %d bytes, the file has %d" % (len(out), total))
    return out + bytes(total - len(out))

def convert(d, skel):
    r = to_text(d)
    if r is None:
        return None
    lines, total = r
    txt = ("# GSC script (tools/assets/gscconv.py, src/sys/gsc.c): chunks as an indented tree; words: end, cmd <id> <operands>, "
           "start/call <action>, opt <letter> <operands>, @<value> <type>; values: int / float / hex\n"
           "size %d\n" % total + "\n".join(lines) + "\n")
    if assemble(lines, total) != d:
        return None
    skel[:] = bytes(len(skel))
    return txt

def apply(buf, txt):
    lines = txt.splitlines()
    total = int(next(l for l in lines if l.startswith("size ")).split()[1])
    body = [l for l in lines if not l.startswith("size ") and not l.startswith("#")]
    buf[:total] = assemble(body, total)
