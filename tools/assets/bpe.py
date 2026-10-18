"""The game's BPE compression (Philip Gage's byte pair encoding, C Users Journal 1994): the files start with
u32 size, u32 packed size, then blocks of 10000 bytes each compressed on their own; a file whose size is a
multiple of 10000 (or empty) ends with an empty block. The packer's hash table has 16384 entries. After the packed
data the file is padded to 16 bytes with leftover bytes of the packer's memory (kept in bpe.txt as they are)."""
import struct

def is_bpe(d, length=None):
    """d: the file (or its first 8 bytes and length: its size)"""
    length = len(d) if length is None else length
    if length < 16 or len(d) < 8:
        return False
    size, packed = struct.unpack_from("<II", d, 0)
    return size > 0 and 8 + packed <= length < 8 + packed + 16 and length % 16 == 0 and packed < size * 2

def expand(d):
    """-> (data, tail bytes after the packed data)"""
    size, packed = struct.unpack_from("<II", d, 0)
    p, end = 8, 8 + packed
    out = []
    while p < end:
        left = list(range(256)); right = [0] * 256
        sym = 0
        c = d[p]; p += 1
        while True:
            if c > 127:
                sym += c - 127; c = 0
            if sym == 256:
                break
            for _ in range(c + 1):
                left[sym] = d[p]; p += 1
                if left[sym] != sym:
                    right[sym] = d[p]; p += 1
                sym += 1
            if sym == 256:
                break
            c = d[p]; p += 1
        n = (d[p] << 8) | d[p + 1]; p += 2
        memo = {}
        def exp(s):
            v = memo.get(s)
            if v is None:
                v = bytes([s]) if left[s] == s else exp(left[s]) + exp(right[s])
                memo[s] = v
            return v
        tab = [exp(s) for s in range(256)]  # every symbol's bytes, then the block by table lookups (in C)
        out.append(b"".join(map(tab.__getitem__, d[p:p + n])))
        p += n
    data = b"".join(out)
    assert p == end and len(data) == size, (p, end, len(data), size)
    return data, d[end:]
