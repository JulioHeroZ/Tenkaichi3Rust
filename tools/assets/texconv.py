"""Textures of the game's files as PNG, exactly reversible.

A texture inside a file is a GS upload (a GIF packet: BITBLTBUF / TRXPOS / TRXREG / TRXDIR, then the IMAGE data)
plus an entry that says how the game samples it (TEX0: format, size, CLUT). The PNG shows the texture as the GS
samples it: indexed formats as palette PNGs (pixel = the index the GS reads, palette entry k = the CLUT colour the
GS picks for index k), direct formats as RGBA. Alpha: GS alpha 0..0x80 is stored as round(a * 255 / 128) (so the
PNG looks right); a texture with alpha above 0x80 keeps the GS value as is ("alpha": "gs" in its record).

`convert(data)` -> (skeleton, records, images): the skeleton is the file with every converted texture's IMAGE data
zeroed; `assemble(skeleton, records, images)` rebuilds the file. A texture is converted only when the rebuild gives
back its exact bytes and every byte of its IMAGE data is decided by the texture (else it stays in the skeleton).
"""
import struct, zlib
import numpy as np
import bisect
from gs_tables import BLOCK32, BLOCK16, BLOCK16S, COL32, COL16, COL8_FIRST, COL4_FIRST

B32, B16, B16S = np.array(BLOCK32), np.array(BLOCK16), np.array(BLOCK16S)
C32, C16 = np.array(COL32), np.array(COL16)
C8, C4 = np.zeros((16, 16), int), np.zeros((16, 32), int)
for y in range(16):
    col = y >> 2
    for x in range(16):
        C8[y, x] = COL8_FIRST[y & 3][x ^ 4 if col & 1 else x] + col * 64
    for x in range(32):
        C4[y, x] = COL4_FIRST[y & 3][x ^ 4 if col & 1 else x] + col * 128

PSMS = (0, 1, 2, 10, 19, 20, 27, 36, 44)

def bits_of(psm):
    return {2: 16, 10: 16, 19: 8, 27: 8, 20: 4, 36: 4, 44: 4, 1: 24}.get(psm, 32)

def addr(bp, bw, psm, x, y):
    """Bit address in GS memory of pixel (x, y) (numpy arrays), and its width in bits."""
    bw = max(bw, 1)
    if psm in (2, 10):
        blk = B16S if psm == 10 else B16
        a = ((bp + ((y >> 1) & ~31) * bw + ((x >> 1) & ~31) + blk[(y >> 3) & 7, (x >> 4) & 3]) << 7) + C16[y & 7, x & 15]
        return ((a * 2) & 0x3FFFFF) * 8, 16
    if psm == 19:
        a = ((bp + ((y >> 1) & ~31) * (bw >> 1) + ((x >> 2) & ~31) + B32[(y >> 4) & 3, (x >> 4) & 7]) << 8) + C8[y & 15, x & 15]
        return (a & 0x3FFFFF) * 8, 8
    if psm == 20:
        a = ((bp + ((y >> 2) & ~31) * (bw >> 1) + ((x >> 2) & ~31) + B16[(y >> 4) & 7, (x >> 5) & 3]) << 9) + C4[y & 15, x & 31]
        return ((a >> 1) & 0x3FFFFF) * 8 + (a & 1) * 4, 4
    a = (((bp + (y & ~31) * bw + ((x >> 1) & ~31) + B32[(y >> 3) & 3, (x >> 3) & 7]) << 6) + C32[y & 7, x & 7]) * 4
    if psm == 27:
        return ((a + 3) & 0x3FFFFF) * 8, 8
    if psm == 36:
        return ((a + 3) & 0x3FFFFF) * 8, 4
    if psm == 44:
        return ((a + 3) & 0x3FFFFF) * 8 + 4, 4
    if psm == 1:
        return (a & 0x3FFFFF) * 8, 24
    return (a & 0x3FFFFF) * 8, 32

def u32(d, o):
    return struct.unpack_from("<I", d, o)[0]

def u64(d, o):
    return struct.unpack_from("<Q", d, o)[0]

def packet_info(d, packet, size, blt):
    """The IMAGE data of an upload packet: (data offset, data length, w, h, psm, bw), or None."""
    if size < 0x60 or packet + size > len(d):
        return None
    if u32(d, packet + 0x38) != 0x52 or ((u32(d, packet + 0x54) >> 26) & 3) != 2:
        return None
    w, h = u32(d, packet + 0x30), u32(d, packet + 0x34)
    psm, bw = (blt >> 8) & 63, blt & 63
    if psm not in PSMS or not (0 < w <= 2048 and 0 < h <= 2048):
        return None
    length = (u32(d, packet + 0x50) & 0x7FFF) * 16
    if length < (w * h * bits_of(psm) + 7) // 8 or 0x60 + length > size:
        return None
    return packet + 0x60, length, w, h, psm, bw

def bitmap(bp, bw, psm, w, h):
    """GS-memory bit addresses of every bit of a w x h rectangle, in row-major pixel order, LSB first."""
    ys, xs = np.mgrid[0:h, 0:w]
    a, nb = addr(bp, bw, psm, xs.ravel(), ys.ravel())
    return (a[:, None] + np.arange(nb)[None, :]).ravel(), nb

def dir_bases(data, base=0, depth=0, out=None):
    """starts of the directories of a file, nested ones too (u32 count, then offsets from the directory's start)"""
    if out is None:
        out = {0}
    if depth > 3 or base + 8 > len(data):
        return out
    if data[base:base + 4] == b"EPCK" and base + 16 <= len(data):
        # an effect pack: "EPCK", 1, count, 0, then the entries' offsets from its start (0 = none)
        n = u32(data, base + 8)
        if 1 <= n <= 256 and base + 16 + 4 * n <= len(data):
            for i in range(n):
                o = u32(data, base + 16 + 4 * i)
                if o and base + o < len(data) and base + o not in out:
                    out.add(base + o)
                    dir_bases(data, base + o, depth + 1, out)
        return out
    n = u32(data, base)
    if not 1 <= n <= 4096 or base + 4 + n * 4 > len(data):
        return out
    offs = [u32(data, base + 4 + 4 * i) & ~3 for i in range(n)]
    if any(o == 0 or base + o > len(data) for o in offs) or any(offs[i] > offs[i + 1] for i in range(n - 1)):
        return out
    for o in offs:
        if base + o not in out:
            out.add(base + o)
            dir_bases(data, base + o, depth + 1, out)
    return out

def scan(data):
    """Texture entries of a file (as the asset exporter finds them): dicts with the entry's fields."""
    bases = sorted(dir_bases(data))
    out, seen = [], set()
    for e in range(0, max(0, len(data) - 64), 16):
        tex0 = u64(data, e + 48)
        tw, th = 1 << ((tex0 >> 26) & 15), 1 << ((tex0 >> 30) & 15)
        psm = (tex0 >> 20) & 63
        if tw > 1024 or th > 1024 or psm not in PSMS:
            continue
        pix, clut = u32(data, e) * 4, u32(data, e + 4) * 4
        size, cs = u32(data, e + 8), u32(data, e + 12)
        if pix == 0 and clut and size == 0 and 96 <= cs <= len(data):
            # a palette of its own (another colour set for a texture): its CLUT upload as a direct texture
            k = bisect.bisect_right(bases, e)
            for base in [0] + bases[max(0, k - 4):k][::-1]:
                p = base + clut
                if p + cs > len(data) or u32(data, p + 0x38) != 0x52 or (p, tex0) in seen:
                    continue
                ci = packet_info(data, p, cs, u32(data, e + 28))
                if ci is None:
                    continue
                out.append(dict(entry=e, base=base, tex0=tex0, tw=ci[2], th=ci[3], psm=ci[4], bw=max(ci[5], 1),
                                cpsm=0, tbp=0x2000 + u32(data, e + 36), cbp=0, pix=ci, clut=None, palette=True))
                seen.add((p, tex0))
                break
            continue
        if pix == 0 or size < 96 or size > len(data):
            continue
        # the directory the entry is in (offsets count from its start), the ones around it, the file's start
        k = bisect.bisect_right(bases, e)
        for base in [0] + bases[max(0, k - 4):k][::-1]:
            p = base + pix
            if p + size > len(data) or u32(data, p + 0x38) != 0x52:
                continue
            if (p, tex0) in seen:
                continue
            pi = packet_info(data, p, size, u32(data, e + 24))
            if pi is None:
                continue
            indexed = psm in (19, 20, 27, 36, 44)
            ci = None
            if indexed:
                if clut == 0:
                    continue
                ci = packet_info(data, base + clut, cs, u32(data, e + 28))
                if ci is None:
                    continue
            out.append(dict(entry=e, base=base, tex0=tex0, tw=tw, th=th, psm=psm, bw=(tex0 >> 14) & 63,
                            cpsm=(tex0 >> 51) & 15, tbp=u32(data, e + 32), cbp=0x2000 + u32(data, e + 36),
                            pix=pi, clut=ci))
            seen.add((p, tex0))
            break
    return out

def gather(src_bits, src_addr, dst_addr):
    """Values at dst bit addresses after writing the bits src_bits to src_addr (unwritten bits: -1)."""
    order = np.argsort(src_addr, kind="stable")
    sa = src_addr[order]
    idx = np.searchsorted(sa, dst_addr)
    idx[idx >= len(sa)] = 0
    hit = sa[idx] == dst_addr
    vals = np.where(hit, src_bits[order][idx], -1)
    return vals

def payload_bits(data, off, n):
    b = np.frombuffer(bytes(data[off:off + (n + 7) // 8]), dtype=np.uint8)
    return np.unpackbits(b, bitorder="little")[:n].astype(np.int64)

def pack_bits(bits):
    return np.packbits(bits.astype(np.uint8), bitorder="little").tobytes()

def values(bits, nb):
    v = bits.reshape(-1, nb).astype(np.int64)
    return (v << np.arange(nb)).sum(axis=1)

def to_bits(vals, nb):
    return ((vals[:, None] >> np.arange(nb)[None, :]) & 1).ravel()

def plane(data, info, bp, rbw, rpsm, rw, rh):
    """What the GS reads as (rpsm, rw x rh at bp, width rbw) after the upload `info` at bp: values, mapping."""
    off, length, w, h, psm, bw = info
    up_addr, nb_up = bitmap(bp, bw, psm, w, h)
    n = w * h * nb_up
    pbits = payload_bits(data, off, n)
    rd_addr, nb_rd = bitmap(bp, rbw, rpsm, rw, rh)
    got = gather(pbits, up_addr, rd_addr)
    return got, up_addr, rd_addr, nb_rd, n

def clut_index(i, psm):
    if psm in (19, 27):
        return (i & 0xE7) | ((i & 8) << 1) | ((i & 16) >> 1)
    return i

def color_of(raw, cpsm):
    if cpsm in (2, 10):
        return np.stack([(raw & 31) << 3, ((raw >> 5) & 31) << 3, ((raw >> 10) & 31) << 3, np.where(raw & 0x8000, 0x80, 0)], 1)
    rgba = np.stack([raw & 255, (raw >> 8) & 255, (raw >> 16) & 255, (raw >> 24) & 255], 1)
    if cpsm == 1:
        rgba[:, 3] = 0x80
    return rgba

def raw_of(rgba, cpsm):
    r, g, b, a = (rgba[:, i].astype(np.int64) for i in range(4))
    if cpsm in (2, 10):
        return (r >> 3) | ((g >> 3) << 5) | ((b >> 3) << 10) | np.where(a >= 0x40, 0x8000, 0)
    if cpsm == 1:
        return r | (g << 8) | (b << 16)
    return r | (g << 8) | (b << 16) | (a << 24)

def png_alpha(a, mode):
    return a if mode == "gs" else (a * 255 + 64) // 128

def gs_alpha(a, mode):
    return a if mode == "gs" else (a * 128 + 127) // 255

def expand5(c):
    return c | (c >> 5)  # c is already << 3: 8-bit from 5-bit with the top bits repeated

# ------------------------------------------------------------------------------------------------ PNG
def png_chunk(kind, body):
    return struct.pack(">I", len(body)) + kind + body + struct.pack(">I", zlib.crc32(kind + body) & 0xFFFFFFFF)

def png_write(w, h, pixels, palette=None):
    """pixels: (h, w) indices with a palette (n x 4 RGBA), or (h, w, 4) RGBA."""
    if palette is not None:
        ihdr = struct.pack(">IIBBBBB", w, h, 8, 3, 0, 0, 0)
        raw = b"".join(b"\0" + bytes(pixels[y].astype(np.uint8)) for y in range(h))
        pal = bytes(palette[:, :3].astype(np.uint8).ravel())
        trns = bytes(palette[:, 3].astype(np.uint8))
        body = png_chunk(b"IHDR", ihdr) + png_chunk(b"PLTE", pal) + png_chunk(b"tRNS", trns)
    else:
        ihdr = struct.pack(">IIBBBBB", w, h, 8, 6, 0, 0, 0)
        raw = b"".join(b"\0" + bytes(pixels[y].astype(np.uint8).ravel()) for y in range(h))
        body = png_chunk(b"IHDR", ihdr)
    return b"\x89PNG\r\n\x1a\n" + body + png_chunk(b"IDAT", zlib.compress(raw, 9)) + png_chunk(b"IEND", b"")

def png_read(blob):
    """-> (w, h, pixels, palette or None) for 8-bit palette or RGBA PNGs (any filter)."""
    assert blob[:8] == b"\x89PNG\r\n\x1a\n"
    o, idat, pal, trns = 8, b"", None, None
    while o < len(blob):
        n = struct.unpack(">I", blob[o:o + 4])[0]
        kind, body = blob[o + 4:o + 8], blob[o + 8:o + 8 + n]
        if kind == b"IHDR":
            w, h, depth, ctype = struct.unpack(">IIBB", body[:10])
        elif kind == b"PLTE":
            pal = np.frombuffer(body, np.uint8).reshape(-1, 3)
        elif kind == b"tRNS":
            trns = np.frombuffer(body, np.uint8)
        elif kind == b"IDAT":
            idat += body
        o += 12 + n
    bpp = {3: 1, 6: 4, 2: 3}[ctype]
    raw = zlib.decompress(idat)
    stride = w * bpp
    rows = np.frombuffer(raw[:h * (stride + 1)], np.uint8).reshape(h, stride + 1)
    if not rows[:, 0].any():  # no filters (what png_write makes): straight copy
        out = rows[:, 1:].astype(np.int64)
        y0 = h
    else:
        out = np.zeros((h, stride), np.int64)
        y0 = 0
    prev = np.zeros(stride, np.int64)
    for y in range(y0, h):
        f = raw[y * (stride + 1)]
        line = np.frombuffer(raw[y * (stride + 1) + 1:(y + 1) * (stride + 1)], np.uint8).astype(np.int64)
        cur = np.zeros(stride, np.int64)
        for x in range(stride):
            a = cur[x - bpp] if x >= bpp else 0
            b = prev[x]
            c = prev[x - bpp] if x >= bpp else 0
            if f == 0: v = line[x]
            elif f == 1: v = line[x] + a
            elif f == 2: v = line[x] + b
            elif f == 3: v = line[x] + (a + b) // 2
            else:
                p = a + b - c; pa, pb, pc = abs(p - a), abs(p - b), abs(p - c)
                v = line[x] + (a if pa <= pb and pa <= pc else b if pb <= pc else c)
            cur[x] = v & 255
        out[y] = cur
        prev = cur
    if ctype == 3:
        palette = np.zeros((len(pal), 4), np.int64)
        palette[:, :3] = pal
        palette[:, 3] = 255
        if trns is not None:
            palette[:len(trns), 3] = trns
        return w, h, out, palette
    if ctype == 2:
        rgba = np.concatenate([out.reshape(h, w, 3), np.full((h, w, 1), 255)], 2)
        return w, h, rgba, None
    return w, h, out.reshape(h, w, 4), None

# ------------------------------------------------------------------------------------------------ convert
def _all_in_once(a, b):
    """every value of a is in b, and a has no value twice (sorting: faster than np.unique / np.isin on large arrays)"""
    sa = np.sort(a, kind="stable")
    if len(sa) > 1 and not (sa[1:] != sa[:-1]).all():
        return False
    sb = np.sort(b, kind="stable")
    i = np.searchsorted(sb, sa)
    i[i >= len(sb)] = 0
    return bool((sb[i] == sa).all()) if len(sb) else len(sa) == 0

def texture_image(data, t):
    """The texture as the GS samples it: (png bytes, record) or None when it cannot be made exactly reversible."""
    tw, th, psm, bw, tbp = t["tw"], t["th"], t["psm"], t["bw"], t["tbp"]
    indexed = psm in (19, 20, 27, 36, 44)
    got, up_addr, rd_addr, nb, n = plane(data, t["pix"], tbp, bw, psm, tw, th)
    if (got < 0).any():
        return None  # the texture reads memory the upload does not write
    # every bit the upload writes must be read back by the texture (no bit decided by anything else)
    if not _all_in_once(up_addr, rd_addr):
        return None
    tex = values(got, nb).reshape(th, tw)
    rec = dict(tw=tw, th=th, psm=psm, bw=bw, tbp=tbp, pix=list(t["pix"]))
    if indexed:
        cpsm, cbp = t["cpsm"], t["cbp"]
        ncol = 256 if psm in (19, 27) else 16
        cw, ch = (16, 16) if ncol == 256 else (8, 2)
        cgot, cup, crd, cnb, cn = plane(data, t["clut"], cbp, 1, cpsm, cw, ch)
        if (cgot < 0).any() or not _all_in_once(cup, crd):
            return None
        craw = values(cgot, cnb)                     # CLUT entries in storage order (x + y * cw)
        cols = color_of(craw, cpsm)
        order = np.array([clut_index(k, psm) for k in range(ncol)])
        logical = cols[order]                       # the colour the GS picks for index k
        mode = "gs" if (logical[:, 3] > 0x80).any() else "png"
        pal = logical.copy()
        pal[:, 3] = png_alpha(pal[:, 3], mode)
        if cpsm in (2, 10):
            pal[:, :3] = expand5(pal[:, :3])
        rec.update(cpsm=cpsm, cbp=cbp, clut=list(t["clut"]), alpha=mode)
        blob = png_write(tw, th, tex, pal)
    else:
        cols = color_of(tex.ravel(), psm)
        mode = "gs" if (cols[:, 3] > 0x80).any() else "png"
        cols[:, 3] = png_alpha(cols[:, 3], mode)
        if psm in (2, 10):
            cols[:, :3] = expand5(cols[:, :3])
        rec.update(alpha=mode)
        blob = png_write(tw, th, cols.reshape(th, tw, 4))
    return blob, rec

def put_plane(buf, info, bp, rbw, rpsm, rw, rh, vals):
    """Writes the values the GS should read (rpsm rectangle) back into the upload's IMAGE data in buf."""
    off, length, w, h, psm, bw = info
    up_addr, nb_up = bitmap(bp, bw, psm, w, h)
    rd_addr, nb_rd = bitmap(bp, rbw, rpsm, rw, rh)
    bits = to_bits(np.asarray(vals, np.int64).ravel(), nb_rd)
    got = gather(bits, rd_addr, up_addr)
    n = w * h * nb_up
    old = payload_bits(buf, off, n)
    new = np.where(got >= 0, got, old)
    blob = bytearray(pack_bits(new))
    nbytes = (n + 7) // 8
    if n % 8:  # keep the bits after the last pixel in the last byte
        keep = buf[off + nbytes - 1] & (0xFF << (n % 8)) & 0xFF
        blob[-1] = (blob[-1] & ((1 << (n % 8)) - 1)) | keep
    buf[off:off + nbytes] = blob

def texture_apply(buf, rec, blob):
    """Writes a texture PNG back into the file bytes `buf` (the inverse of texture_image)."""
    w, h, px, pal = png_read(blob)
    assert (w, h) == (rec["tw"], rec["th"])
    psm, mode = rec["psm"], rec["alpha"]
    if pal is not None and "cpsm" in rec:
        cpsm = rec["cpsm"]
        ncol = 256 if psm in (19, 27) else 16
        cw, ch = (16, 16) if ncol == 256 else (8, 2)
        logical = np.zeros((ncol, 4), np.int64)
        logical[:len(pal)] = pal[:ncol]
        logical[:, 3] = gs_alpha(logical[:, 3], mode)
        stored = np.zeros_like(logical)
        for k in range(ncol):
            stored[clut_index(k, psm)] = logical[k]
        put_plane(buf, tuple(rec["clut"]), rec["cbp"], 1, cpsm, cw, ch, raw_of(stored, cpsm))
        put_plane(buf, tuple(rec["pix"]), rec["tbp"], rec["bw"], psm, w, h, px)
    else:
        cols = px.reshape(-1, 4).copy()
        cols[:, 3] = gs_alpha(cols[:, 3], mode)
        put_plane(buf, tuple(rec["pix"]), rec["tbp"], rec["bw"], psm, w, h, raw_of(cols, psm))

def convert(data):
    """-> (skeleton bytes, records, png blobs). Only textures that rebuild exactly are taken out."""
    recs, blobs = [], []
    skel = bytearray(data)
    for t in scan(data):
        r = texture_image(data, t)
        if r is None:
            continue
        blob, rec = r
        rec["entry"] = t["entry"]
        # exactness: zero this texture's data (and CLUT) in a copy, write it back from the PNG, compare
        test = bytearray(data)
        for key in ("pix", "clut"):
            if key in rec:
                off, length = rec[key][0], rec[key][1]
                test[off:off + length] = bytes(length)
        try:
            texture_apply(test, rec, blob)
        except Exception:
            continue
        if bytes(test) != bytes(data):
            continue
        recs.append(rec)
        blobs.append(blob)
    # zero the data of every texture taken out (shared CLUTs too)
    for rec in recs:
        for key in ("pix", "clut"):
            if key in rec:
                off, length = rec[key][0], rec[key][1]
                skel[off:off + length] = bytes(length)
    return bytes(skel), recs, blobs

def assemble(skel, recs, blobs):
    buf = bytearray(skel)
    for rec, blob in zip(recs, blobs):
        texture_apply(buf, rec, blob)
    return bytes(buf)
