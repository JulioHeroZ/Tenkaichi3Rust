"""Sound banks (Sony's IOP format: a sample data entry and an "IECSsreV" header entry whose Vagi chunk lists the
samples: bd offset, rate, loop) to WAV files, rebuildable to the exact bytes.

The samples are SPU ADPCM: blocks of 16 bytes, byte 0 = shift | filter << 4, byte 1 = flags (1 end, 2 loop, 4 loop
start), then 28 four-bit steps. Each sample becomes a 16-bit mono WAV of what the SPU plays. samples.txt keeps, per
block, the two header bytes: the rebuild finds the steps again from the WAV (for each output sample there is one
step that gives it, from the same shift and filter). Blocks where two steps would give the same output (the result
was clamped) keep their steps in samples.txt too.
"""
import struct, json

FILT = [(0, 0), (60, 0), (115, -52), (98, -55), (122, -60)]

def decode_block(blk, s1, s2):
    shift, filt = blk[0] & 15, blk[0] >> 4
    f0, f1 = FILT[filt] if filt < 5 else (0, 0)
    out = []
    for i in range(28):
        b = blk[2 + i // 2]
        n = (b >> (4 * (i & 1))) & 15
        if n >= 8:
            n -= 16
        s = ((n << 12) >> shift) if shift <= 12 else ((n << 12) >> 9)
        s += (s1 * f0 + s2 * f1 + 32) >> 6
        s = max(-32768, min(32767, s))
        out.append(s)
        s2, s1 = s1, s
    return out, s1, s2

def steps_for(head, pcm, s1, s2):
    """the 14 step bytes that decode to pcm (28 samples) after head, or None when one is not unique / missing"""
    shift, filt = head & 15, head >> 4
    f0, f1 = FILT[filt] if filt < 5 else (0, 0)
    nib = []
    for t in pcm:
        pred = (s1 * f0 + s2 * f1 + 32) >> 6
        sh = shift if shift <= 12 else 9
        if -32768 < t < 32767:
            # not clamped: the step is (t - pred) scaled back, and only one can give t
            g = ((t - pred) << sh) >> 12
            found = [n for n in (g - 1, g, g + 1) if -8 <= n < 8 and ((n << 12) >> sh) + pred == t]
        else:
            found = [n for n in range(-8, 8) if max(-32768, min(32767, ((n << 12) >> sh) + pred)) == t]
        if len(found) != 1:
            return None
        nib.append(found[0] & 15)
        s2, s1 = s1, t
    return bytes(nib[2 * i] | nib[2 * i + 1] << 4 for i in range(14))

def encode_steps(head, pcm, s1, s2):
    """the nearest steps for pcm (an edited sound): -> (bytes, s1, s2)"""
    shift, filt = head & 15, head >> 4
    f0, f1 = FILT[filt] if filt < 5 else (0, 0)
    nib = []
    for t in pcm:
        pred = (s1 * f0 + s2 * f1 + 32) >> 6
        best = min(range(-8, 8), key=lambda n: abs(max(-32768, min(32767, (((n << 12) >> shift) if shift <= 12 else ((n << 12) >> 9)) + pred)) - t))
        s = max(-32768, min(32767, (((best << 12) >> shift) if shift <= 12 else ((best << 12) >> 9)) + pred))
        nib.append(best & 15)
        s2, s1 = s1, s
    return bytes(nib[2 * i] | nib[2 * i + 1] << 4 for i in range(14)), s1, s2

def wav(rate, pcm):
    data = struct.pack("<%dh" % len(pcm), *pcm)
    return (b"RIFF" + struct.pack("<I", 36 + len(data)) + b"WAVEfmt " + struct.pack("<IHHIIHH", 16, 1, 1, rate, rate * 2, 2, 16)
            + b"data" + struct.pack("<I", len(data)) + data)

def wav_read(b):
    if b[:4] != b"RIFF" or b[8:12] != b"WAVE":
        raise ValueError("not a WAV")
    p = 12
    fmt = None
    while p + 8 <= len(b):
        cid, n = b[p:p + 4], struct.unpack_from("<I", b, p + 4)[0]
        if cid == b"fmt ":
            fmt = struct.unpack_from("<HHIIHH", b, p + 8)
        elif cid == b"data":
            if not fmt or fmt[0] != 1 or fmt[1] != 1 or fmt[5] != 16:
                raise ValueError("WAV must be 16-bit mono PCM")
            return fmt[2], list(struct.unpack_from("<%dh" % (n // 2), b, p + 8))
        p += 8 + n + (n & 1)
    raise ValueError("WAV without data")

def banks(d):
    """-> [(bd offset in file, bd end, [(sample offset in bd, rate, loop)], hd entry index)]"""
    if len(d) < 8:
        return []
    n = struct.unpack_from("<I", d, 0)[0]
    if not 2 <= n <= 4096 or (n + 1) * 4 > len(d):
        return []
    o = [struct.unpack_from("<I", d, 4 * i)[0] & ~3 for i in range(1, n + 1)] + [len(d)]
    if any(o[i] > o[i + 1] for i in range(n)):
        return []
    out = []
    for i in range(1, n):
        h = o[i]
        if d[h:h + 8] != b"IECSsreV" or d[h + 0x10:h + 0x18] != b"IECSdaeH":
            continue
        bd_size = struct.unpack_from("<I", d, h + 0x1C)[0]
        vagi = struct.unpack_from("<I", d, h + 0x10 + 0x1C)[0] if False else None
        offs = struct.unpack_from("<8I", d, h + 0x20)
        # Head: +0x18 hd size, +0x1C bd size, then the chunk offsets; the Vagi chunk is the one named so
        vg = None
        for c in offs:
            if c != 0xFFFFFFFF and h + c + 16 <= o[i + 1] and d[h + c:h + c + 8] == b"IECSigaV":
                vg = h + c
        if vg is None:
            continue
        a, b = o[i - 1], o[i]
        if bd_size > b - a:
            continue
        cnt = struct.unpack_from("<I", d, vg + 12)[0] + 1
        smp = []
        for k in range(cnt):
            so = struct.unpack_from("<I", d, vg + 16 + 4 * k)[0]
            off, rate, loop = struct.unpack_from("<IHB", d, vg + so)
            smp.append((off, rate, loop))
        out.append((a, a + bd_size, smp, i))
    return out

def convert(d, skel):
    """-> ({wav name: bytes}, samples.txt text) or None; the sample data is zeroed in skel"""
    bs = banks(d)
    if not bs:
        return None
    wavs, lines = {}, []
    for a, end, smp, hi in bs:
        starts = sorted(set(s[0] for s in smp))
        for k, (off, rate, loop) in enumerate(smp):
            nxt = [x for x in starts if x > off]
            stop = a + (nxt[0] if nxt else end - a)
            p = a + off
            if p >= stop or (stop - p) % 16:
                continue
            nb = (stop - p) // 16
            if any(l.split()[2] == str(p) for l in lines):
                continue  # a sample listed twice
            s1 = s2 = 0
            pcm, heads, raws = [], [], []
            for j in range(nb):
                blk = d[p + 16 * j:p + 16 * j + 16]
                out, n1, n2 = decode_block(blk, s1, s2)
                if steps_for(blk[0], out, s1, s2) != blk[2:]:
                    raws.append("%d:%s" % (j, blk[2:].hex()))
                s1, s2 = n1, n2
                pcm += out
                heads.append(blk[:2].hex())
            name = "sound_%d_%03d.wav" % (hi, k)
            wavs[name] = wav(rate, pcm)
            lines.append("%s %d %d %d %s %s" % (name, hi, p, nb, "".join(heads), ",".join(raws) or "-"))
            skel[p:p + nb * 16] = bytes(nb * 16)
        # the sample data of the bd entry no sample of the header lists (left over from the tool that made the bank):
        # blocks decoded the same way, as sound_<hd>_tail<n>.wav
        bd_end = struct.unpack_from("<I", d, 4 + 4 * hi)[0] & ~3
        listed = bytearray(bd_end - a)  # the blocks the samples above take (from the file alone, not skel)
        for l in lines:
            f = l.split()
            if f[1] == str(hi):
                q, n = int(f[2]) - a, int(f[3])
                listed[q:q + 16 * n] = b"\1" * (16 * n)
        free = lambda o: not listed[o - a] and any(d[o:o + 16])
        o, t = a, 0
        while o + 16 <= bd_end:
            if free(o):
                p = o
                while o + 16 <= bd_end and free(o):
                    o += 16
                nb = (o - p) // 16
                s1 = s2 = 0
                pcm, heads, raws = [], [], []
                ok = True
                for j in range(nb):
                    blk = d[p + 16 * j:p + 16 * j + 16]
                    if blk[0] >> 4 > 4:
                        ok = False
                        break
                    out, n1, n2 = decode_block(blk, s1, s2)
                    if steps_for(blk[0], out, s1, s2) != blk[2:]:
                        raws.append("%d:%s" % (j, blk[2:].hex()))
                    s1, s2 = n1, n2
                    pcm += out
                    heads.append(blk[:2].hex())
                if ok and nb:
                    name = "sound_%d_tail%d.wav" % (hi, t)
                    t += 1
                    wavs[name] = wav(44100, pcm)
                    lines.append("%s %d %d %d %s %s" % (name, hi, p, nb, "".join(heads), ",".join(raws) or "-"))
                    skel[p:p + nb * 16] = bytes(nb * 16)
            else:
                o += 16
    if not lines:
        return None
    txt = ("# sounds of the file (tools/assets/vagconv.py): wav, header entry, file offset of the first block, blocks, "
           "the two header bytes of every block (shift | filter << 4, flags), blocks whose steps are kept (block:hex)\n"
           + "\n".join(lines) + "\n")
    return wavs, txt

def apply(buf, files, txt):
    """files: {wav name: bytes}"""
    for line in txt.splitlines():
        if not line or line.startswith("#"):
            continue
        name, hi, p, nb, heads, raws = line.split()
        p, nb = int(p), int(nb)
        rate, pcm = wav_read(files[name])
        if len(pcm) != nb * 28:
            raise ValueError("%s: %d samples, the file has room for %d" % (name, len(pcm), nb * 28))
        raw = {int(r.split(":")[0]): bytes.fromhex(r.split(":")[1]) for r in raws.split(",")} if raws != "-" else {}
        s1 = s2 = 0
        for j in range(nb):
            head = bytes.fromhex(heads[4 * j:4 * j + 4])
            blk = pcm[28 * j:28 * j + 28]
            st = raw.get(j) or steps_for(head[0], blk, s1, s2)
            if st is None:
                st, _, _ = encode_steps(head[0], blk, s1, s2)
            buf[p + 16 * j:p + 16 * j + 16] = head + st
            _, s1, s2 = decode_block(head + st, s1, s2)


def tables(d, skel):
    """The Sony sound headers of the file's banks as typed records (sound-tables.json; layout.py): each "SCEI" chunk's
    header; Vers (version), Head (the header and sample data sizes, the chunk offsets), Vagi (the samples: offset in
    the sample data, rate, loop). The chunks of other kinds (Setb, Sequ, Seqs, Prog, Sset, Smpl...) only by their
    header for now. -> text or None (zeroed in skel)."""
    import layout
    lay = layout.Layout(0)
    lay.type("SceiVers", 0x10, [("magic", 0, "u8[8]"), ("size", 8, "u32"), ("version", 12, "u32")])
    lay.type("SceiHead", 0x40, [("magic", 0, "u8[8]"), ("size", 8, "u32"), ("headerSize", 12, "u32"),
                                ("dataSize", 0x10, "u32"), ("chunks", 0x14, "s32[11]")])
    lay.type("SceiVagi", 0x10, [("magic", 0, "u8[8]"), ("size", 8, "u32"), ("maxIndex", 12, "u32")],
             tail=("offsets", "u32"))
    lay.type("SceiSample", 8, [("offset", 0, "u32"), ("rate", 4, "u16"), ("loop", 6, "u8"), ("unk7", 7, "u8")])
    lay.type("SceiChunk", 0x0C, [("magic", 0, "u8[8]"), ("size", 8, "u32")])
    pos = 0
    while True:
        h = d.find(b"IECSsreV", pos)
        if h < 0 or h + 0x10 > len(d):
            break
        pos = h + 8
        if h % 4 or struct.unpack_from("<I", d, h + 8)[0] != 0x10:
            continue
        lay.read(d, "SceiVers", h)
        c = h + 0x10
        if d[c:c + 8] == b"IECSdaeH" and struct.unpack_from("<I", d, c + 8)[0] == 0x40:
            lay.read(d, "SceiHead", c)
            for off in struct.unpack_from("<11i", d, c + 0x14):
                if off <= 0 or h + off + 0x10 > len(d) or d[h + off:h + off + 4] != b"IECS":
                    continue
                q = h + off
                if d[q:q + 8] == b"IECSigaV":
                    n = struct.unpack_from("<I", d, q + 12)[0] + 1
                    if n > 4096 or q + 0x10 + 4 * n > len(d):
                        continue
                    lay.read(d, "SceiVagi", q, n)
                    for k in range(n):
                        so = struct.unpack_from("<I", d, q + 0x10 + 4 * k)[0]
                        if 0 < so and q + so + 8 <= len(d):
                            lay.read(d, "SceiSample", q + so)
                else:
                    lay.read(d, "SceiChunk", q)
        elif d[c:c + 4] == b"IECS":
            lay.read(d, "SceiChunk", c)
            # a sequence file: its chunk offsets follow like a Head's
    if not lay.records:
        return None
    layout.finish(lay, d, skel)
    return lay.text("the Sony sound headers of the file (tools/assets/vagconv.py): chunk headers, version, the header "
                    "(sizes, chunk offsets from the Vers chunk) and the sample list (offset in the sample data, rate, "
                    "loop); the other chunks' contents are not typed yet")
