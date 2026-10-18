"""GS upload packets (of the form texpackets.py writes) that no texture entry describes, so texconv.py cannot say
how the GS reads them: their data is kept as it is uploaded, as PNGs, in raw-textures.txt + rawtex_<n>.png.

A packet whose data is width x height 32-bit pixels becomes an RGBA PNG of those bytes (alpha as stored, 0..128);
any other is a PNG of its bytes (a palette image whose palette is grey: index = byte value), `length / height`
bytes wide. The packet itself is written from the line (texpackets.header and TRAILER).
"""
import struct, zlib
import numpy as np
import texpackets
from texconv import png_write, png_read


def u32(d, o):
    return struct.unpack_from("<I", d, o)[0]


GREY = np.array([[i, i, i, 255] for i in range(256)])


def convert(d, skel):
    """-> ({png name: bytes}, raw-textures.txt text) or None; the packets zeroed in skel."""
    files, lines = {}, []
    o = 0
    end = len(d) - 0x80
    while o < end:
        if u32(d, o + 12) >> 24 == 0x50 and u32(d, o + 16) == 0x8003 and u32(d, o + 0x28) == 0x51 and any(skel[o + 12:o + 16]):
            q = u32(d, o + 0x50) & 0x7FFF
            n, w, h = q * 16, u32(d, o + 0x30), u32(d, o + 0x34)
            if n and 0 < w <= 2048 and 0 < h <= 2048 and o + 0x60 + n + 0x20 <= len(d) \
                    and d[o:o + 0x60] == texpackets.header(n, w, h) and d[o + 0x60 + n:o + 0x80 + n] == texpackets.TRAILER \
                    and skel[o + 0x60:o + 0x60 + n] == d[o + 0x60:o + 0x60 + n]:
                data = np.frombuffer(d[o + 0x60:o + 0x60 + n], np.uint8)
                name = "rawtex_%03d.png" % len(lines)
                if n == w * h * 4:
                    files[name] = png_write(w, h, data.reshape(h, w, 4))
                    kind = "rgba"
                else:
                    rw = n // h if n % h == 0 else 16
                    files[name] = png_write(rw, n // rw, data.reshape(n // rw, rw), GREY)
                    kind = "bytes"
                lines.append("%s %d %d %d %d %s" % (name, o + 0x60, n, w, h, kind))
                skel[o:o + 0x80 + n] = bytes(0x80 + n)
                o += 0x80 + n
                continue
        o += 16
    if not lines:
        return None
    return files, ("# GS uploads no texture entry describes (tools/assets/rawtex.py): png, data offset, length, width, "
                   "height, rgba (32-bit pixels) or bytes (the data's bytes as a grey palette image)\n" + "\n".join(lines) + "\n")


def apply(buf, files, text):
    for line in text.splitlines():
        if not line or line.startswith("#"):
            continue
        name, o, n, w, h, kind = line.split()
        o, n, w, h = int(o), int(n), int(w), int(h)
        _, _, pixels, _ = png_read(files[name])
        px = np.asarray(pixels, np.uint8).ravel()
        buf[o - 0x60:o] = texpackets.header(n, w, h)
        buf[o:o + n] = bytes(px[:n])
        buf[o + n:o + n + 0x20] = texpackets.TRAILER
