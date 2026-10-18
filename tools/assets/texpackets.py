"""The GS upload packets around the textures' data (texconv.py takes out only the pixels) as texture-packets.txt:
one line per packet, `data_offset length width height`; host/assets.rs writes each packet whole from it.

An upload packet of the game's texture files is: three zero words, VIF DIRECT of (length / 16 + 7) quadwords,
a GIF tag (NLOOP 3, EOP, A+D) with BITBLTBUF 0, TRXPOS 0, TRXREG width x height, TRXDIR 0 (host to local), a GIF
tag of IMAGE mode of length / 16 quadwords, the data, a GIF tag (NLOOP 1, EOP, A+D) with TEXFLUSH and a zero word.
(The real BITBLTBUF is set by the game from the texture entry.) Only packets of exactly this form are taken out.
"""
import struct


def header(length, w, h):
    q = length // 16
    return struct.pack("<24I", 0, 0, 0, 0x50000000 | (q + 7), 0x8003, 0x10000000, 0xE, 0, 0, 0, 0x51, 0, w, h, 0x52, 0,
                       0, 0, 0x53, 0, 0x8000 | q, 0x08000000, 0, 0)


TRAILER = struct.pack("<8I", 0x8001, 0x10000000, 0xE, 0, 0, 0, 0x3F, 0)


def convert(d, skel, recs):
    """recs: texconv records (their pix / clut: data offset, length, w, h, ...). -> texture-packets.txt text or
    None; the packets' bytes zeroed in skel."""
    lines, seen = [], set()
    for r in recs:
        for pi in (r["pix"], r.get("clut")):
            if not pi or pi[0] in seen:
                continue
            o, n, w, h = pi[0], pi[1], pi[2], pi[3]
            if n % 16 or (n // 16 + 7) > 0xFFFF or o < 0x60 or o + n + 0x20 > len(d):
                continue
            if d[o - 0x60:o] != header(n, w, h) or d[o + n:o + n + 0x20] != TRAILER:
                continue
            seen.add(o)
            skel[o - 0x60:o] = bytes(0x60)
            skel[o + n:o + n + 0x20] = bytes(0x20)
            lines.append("%d %d %d %d" % (o, n, w, h))
    if not lines:
        return None
    return ("# GS upload packets of the textures (tools/assets/texpackets.py): data offset, length, width, height\n"
            + "\n".join(lines) + "\n")


def apply(buf, text):
    for line in text.splitlines():
        if not line or line.startswith("#"):
            continue
        o, n, w, h = map(int, line.split())
        buf[o - 0x60:o] = header(n, w, h)
        buf[o + n:o + n + 0x20] = TRAILER


def entries(d, skel, recs):
    """The texture entries (64 bytes each, as texconv.scan reads them) as typed records: texture-entries.json text or
    None; their bytes zeroed in skel."""
    import layout
    lay = layout.Layout(0)
    lay.type("TexEntry", 0x40, [("pix", 0, "u32"), ("clut", 4, "u32"), ("pixSize", 8, "u32"), ("clutSize", 12, "u32"),
                                ("pixBlt", 24, "u32"), ("clutBlt", 28, "u32"), ("tbp", 32, "u32"), ("cbp", 36, "u32"),
                                ("tex0", 48, "u32[2]")])
    for r in recs:
        if "entry" in r:
            lay.read(d, "TexEntry", r["entry"])
    if not lay.records:
        return None
    layout.finish(lay, d, skel)
    return lay.text("the texture entries (tools/assets/texpackets.py): where a texture's upload packets are (pix / clut "
                    "in words from the entry's directory), their sizes, the GS BITBLTBUF words, the texture and CLUT "
                    "base blocks, TEX0")
