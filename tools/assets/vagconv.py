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

# ((n << 12) >> shift) for the 16 nibbles (index n & 15), by the shift the SPU uses (above 12: 9)
STEP = [[(((n - 16 if n >= 8 else n) << 12) >> sh) for n in range(16)] for sh in range(13)]


def decode_checked(d, p, nb):
    """the nb blocks at d[p:]: (pcm, the two header bytes of each block as hex, the blocks whose steps cannot be
    found again from the pcm as "block:hex"); the same as decode_block and steps_for block by block, in one pass
    (the steps are unique wherever the output was not clamped, so only clamped samples are looked at)"""
    pcm, heads, clamped = [], [], []
    append = pcm.append
    s1 = s2 = 0
    for j in range(nb):
        o = p + 16 * j
        h = d[o]
        heads.append(d[o:o + 2].hex())
        filt = h >> 4
        f0, f1 = FILT[filt] if filt < 5 else (0, 0)
        shift = h & 15
        step = STEP[shift if shift <= 12 else 9]
        for b in d[o + 2:o + 16]:
            for n in (b & 15, b >> 4):
                s = step[n] + ((s1 * f0 + s2 * f1 + 32) >> 6)
                if s >= 32767:
                    s = 32767
                    clamped.append(len(pcm))
                elif s <= -32768:
                    s = -32768
                    clamped.append(len(pcm))
                append(s)
                s2 = s1
                s1 = s
    raws = []
    for i in clamped:
        j = i // 28
        if raws and raws[-1][0] == j:
            continue
        h = d[p + 16 * j]
        filt = h >> 4
        f0, f1 = FILT[filt] if filt < 5 else (0, 0)
        shift = h & 15
        step = STEP[shift if shift <= 12 else 9]
        s1 = pcm[i - 1] if i >= 1 else 0
        s2 = pcm[i - 2] if i >= 2 else 0
        pred = (s1 * f0 + s2 * f1 + 32) >> 6
        t = pcm[i]
        if sum(1 for v in step if max(-32768, min(32767, v + pred)) == t) != 1:
            raws.append((j, d[p + 16 * j + 2:p + 16 * j + 16].hex()))
    return pcm, heads, ["%d:%s" % r for r in raws]


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
            pcm, heads, raws = decode_checked(d, p, nb)
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
                ok = all(d[p + 16 * j] >> 4 <= 4 for j in range(nb))
                pcm, heads, raws = decode_checked(d, p, nb) if ok else ([], [], [])
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

def steps_all(heads, pcm, raw):
    """the blocks (header + steps) of a whole sound at once, when every block's steps give back its pcm exactly
    (the sound as converted): numpy over all samples, since then each sample's predictor comes from the pcm itself.
    None when some block needs the nearest steps instead (an edited sound): apply goes block by block then."""
    import numpy as np
    nb = len(heads) // 2
    t = np.asarray(pcm, np.int64)
    hb = np.frombuffer(heads, np.uint8)[0::2].astype(np.int64)
    filt = np.repeat(hb >> 4, 28)
    shift = np.repeat(hb & 15, 28)
    sh = np.where(shift <= 12, shift, 9)
    ft = np.array(FILT + [(0, 0)] * 11, np.int64)
    f0, f1 = ft[filt, 0], ft[filt, 1]
    s1 = np.concatenate(([0], t[:-1]))
    s2 = np.concatenate(([0, 0], t[:-2]))[:len(t)]
    pred = (s1 * f0 + s2 * f1 + 32) >> 6
    nib = np.zeros(len(t), np.int64)
    count = np.zeros(len(t), np.int64)
    free = (t > -32768) & (t < 32767)
    g = ((t - pred) << sh) >> 12
    for d in (-1, 0, 1):
        n = g + d
        hit = free & (n >= -8) & (n < 8) & ((((n << 12) >> sh) + pred) == t)
        nib = np.where(hit, n, nib)
        count += hit
    for n in range(-8, 8):
        v = np.clip(((n << 12) >> sh) + pred, -32768, 32767)
        hit = ~free & (v == t)
        nib = np.where(hit & (count == 0), n, nib)
        count += hit
    good = (count == 1).reshape(nb, 28).all(axis=1)
    nib = (nib & 15).reshape(nb, 14, 2)
    st = (nib[:, :, 0] | (nib[:, :, 1] << 4)).astype(np.uint8)
    out = bytearray(16 * nb)
    for j in range(nb):
        if j in raw:
            # kept steps: they must give back the pcm too (from the pcm's own last two samples)
            i = 28 * j
            got, _, _ = decode_block(heads[2 * j:2 * j + 2] + raw[j], int(s1[i]), int(s2[i]))
            if got != pcm[i:i + 28]:
                return None
            out[16 * j:16 * j + 16] = heads[2 * j:2 * j + 2] + raw[j]
        elif not good[j]:
            return None
        else:
            out[16 * j:16 * j + 16] = heads[2 * j:2 * j + 2] + st[j].tobytes()
    return bytes(out)


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
        steps = steps_all(bytes.fromhex(heads), pcm, raw)
        if steps is not None:
            buf[p:p + 16 * nb] = steps
            continue
        # an edited sound (some output no step gives exactly): block by block, the nearest steps
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
