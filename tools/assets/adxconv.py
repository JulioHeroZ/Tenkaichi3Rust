"""CRI ADX audio streams (disc/DATA/*.ADX) to WAV, rebuildable to the exact bytes.

ADX (big-endian header: 0x8000, data offset - 4, encoding 3, frame size 18, 4 bits, channels, rate, samples,
high-pass frequency, ...) is ADPCM in frames of 18 bytes per channel: a 16-bit scale, then 32 four-bit steps;
sample = step * scale + (c1 * previous + c2 * the one before) >> 12, clamped. The WAV holds what that gives (every
frame, 32 samples each, channels interleaved). adx.txt keeps the two coefficients and every frame's scale (the
encoder's choice: `scales` lines, frames in order, channels interleaved), and the rebuild finds the steps again
from the WAV with that scale (one step gives each sample; frames where the output was clamped
keep their steps in adx.txt). An edited WAV gets the nearest steps for the same scales.
"""
import struct, math

def be16(b, o): return struct.unpack_from(">H", b, o)[0]
def be32(b, o): return struct.unpack_from(">I", b, o)[0]

def coefs(highpass, rate):
    z = math.cos(2.0 * math.pi * highpass / rate)
    a = math.sqrt(2.0) - z
    b = math.sqrt(2.0) - 1.0
    c = (a - math.sqrt((a + b) * (a - b))) / b
    return int(c * 8192.0), int(c * c * -4096.0)

def header(d):
    if len(d) < 0x40 or be16(d, 0) != 0x8000 or d[4] != 3 or d[5] != 18 or d[6] != 4 or d[7] not in (1, 2):
        return None
    start = be16(d, 2) + 4
    if start > len(d) or d[start - 6:start] != b"(c)CRI":
        return None
    return dict(start=start, ch=d[7], rate=be32(d, 8), total=be32(d, 12), hp=be16(d, 16))

def frames_of(d, h):
    """the frames that hold samples: (count)"""
    return min((h["total"] + 31) // 32, (len(d) - h["start"]) // (18 * h["ch"]))

def wav(rate, ch, pcm):
    data = struct.pack("<%dh" % len(pcm), *pcm)
    return (b"RIFF" + struct.pack("<I", 36 + len(data)) + b"WAVEfmt " + struct.pack("<IHHIIHH", 16, 1, ch, rate, rate * 2 * ch, 2 * ch, 16)
            + b"data" + struct.pack("<I", len(data)) + data)

def wav_read(b):
    p = 12
    fmt = None
    while p + 8 <= len(b):
        cid, n = b[p:p + 4], struct.unpack_from("<I", b, p + 4)[0]
        if cid == b"fmt ":
            fmt = struct.unpack_from("<HHIIHH", b, p + 8)
        elif cid == b"data":
            return fmt[1], list(struct.unpack_from("<%dh" % (n // 2), b, p + 8))
        p += 8 + n + (n & 1)
    raise ValueError("WAV without data")

def convert(d, skel):
    """-> (wav bytes, adx.txt text) or None; the steps are zeroed in skel"""
    h = header(d)
    if not h:
        return None
    c1, c2 = coefs(h["hp"], h["rate"])
    ch, start, nf = h["ch"], h["start"], frames_of(d, h)
    hist = [[0, 0] for _ in range(ch)]
    out = [[] for _ in range(ch)]
    kept = []
    scales = []
    for fr in range(nf):
        for c in range(ch):
            o = start + (fr * ch + c) * 18
            scale = be16(d, o)
            h1, h2 = hist[c]
            ambiguous = False
            for i in range(32):
                n = (d[o + 2 + i // 2] >> (0 if i & 1 else 4)) & 15
                if n >= 8:
                    n -= 16
                pred = (c1 * h1 + c2 * h2) >> 12
                v = max(-32768, min(32767, n * scale + pred))
                if scale == 0 or v in (-32768, 32767):
                    ambiguous = True
                out[c].append(v)
                h2, h1 = h1, v
            hist[c] = [h1, h2]
            if ambiguous:
                kept.append("%d:%s" % (fr * ch + c, d[o + 2:o + 18].hex()))
            skel[o:o + 18] = bytes(18)
            scales.append(scale)
    pcm = [out[c][i] for i in range(nf * 32) for c in range(ch)]
    txt = ("# ADX stream (tools/assets/adxconv.py): coefficients, frames, frames whose steps are kept (index:hex)\n"
           "coefs %d %d\nframes %d\n" % (c1, c2, nf) + "".join("keep %s\n" % k for k in kept)
           + "".join("scales %s\n" % " ".join(map(str, scales[i:i + 32])) for i in range(0, len(scales), 32)))
    return wav(h["rate"], ch, pcm), txt

def apply(buf, wavb, txt):
    h = header(buf)
    f = {}
    kept = {}
    scales = []
    for line in txt.splitlines():
        g = line.split()
        if not g or line.startswith("#"):
            continue
        if g[0] == "scales":
            scales += [int(x) for x in g[1:]]
        elif g[0] == "keep":
            k, hx = g[1].split(":")
            kept[int(k)] = bytes.fromhex(hx)
        else:
            f[g[0]] = [int(x) for x in g[1:]]
    c1, c2 = f["coefs"]
    nf = f["frames"][0]
    ch, start = h["ch"], h["start"]
    nch, pcm = wav_read(wavb)
    if nch != ch or len(pcm) != nf * 32 * ch:
        raise ValueError("the WAV must have %d channels and %d samples" % (ch, nf * 32))
    if scales:
        if len(scales) != nf * ch:
            raise ValueError("adx.txt has %d scales for %d frames" % (len(scales), nf * ch))
        for k, v in enumerate(scales):
            struct.pack_into(">H", buf, start + k * 18, v)
    hist = [[0, 0] for _ in range(ch)]
    for fr in range(nf):
        for c in range(ch):
            k = fr * ch + c
            o = start + k * 18
            scale = be16(buf, o)
            h1, h2 = hist[c]
            steps = []
            for i in range(32):
                pred = (c1 * h1 + c2 * h2) >> 12
                if k in kept:
                    n = (kept[k][i // 2] >> (0 if i & 1 else 4)) & 15
                    n = n - 16 if n >= 8 else n
                else:
                    t = pcm[(fr * 32 + i) * ch + c]
                    if scale:
                        g = (t - pred) // scale
                        n = min((x for x in (g - 1, g, g + 1, g + 2) if -8 <= x < 8), key=lambda x: abs(max(-32768, min(32767, x * scale + pred)) - t), default=0)
                    else:
                        n = 0
                v = max(-32768, min(32767, n * scale + pred))
                steps.append(n & 15)
                h2, h1 = h1, v
            hist[c] = [h1, h2]
            buf[o + 2:o + 18] = bytes(steps[2 * j] << 4 | steps[2 * j + 1] for j in range(16))
