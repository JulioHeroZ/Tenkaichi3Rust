"""What is left of a file after every other converter: its directory and the runs of words whose structure is not
known yet, as typed records (layout.py) in file-tables.json, so skeleton.bin is all zeros and is not written (the
file's size is in skeleton.txt). The directory of a file of entries is {count, offsets...} at 0: each offset where
an entry starts, the last the end of the data."""
import struct
import layout


def u32(d, o):
    return struct.unpack_from("<I", d, o)[0]


def directory(d):
    """-> the number of offsets of the directory at 0 (n, or n + 1 when the end offset follows), or 0 when d does not
    start with one."""
    if len(d) < 8:
        return 0
    n = u32(d, 0)
    if not 1 <= n <= 4096 or 4 + 4 * n > len(d):
        return 0
    offs = [u32(d, 4 + 4 * i) for i in range(n)]
    if offs[0] < 4 + 4 * n or any(b < a for a, b in zip(offs, offs[1:])) or offs[-1] > len(d):
        return 0
    if 8 + 4 * n <= len(d) and offs[-1] <= u32(d, 4 + 4 * n) <= len(d):
        return n + 1
    return n


def nested(lay, d, skel, base, end, depth):
    """A directory at `base` (an entry ending at `end`) as a Directory record when its words are still in skel."""
    if depth > 3 or end - base < 12:
        return
    n = u32(d, base)
    if not 1 <= n <= 4096 or 4 + 4 * n > end - base:
        return
    offs = [u32(d, base + 4 + 4 * i) for i in range(n)]
    if offs[0] < 4 + 4 * n or any(b < a for a, b in zip(offs, offs[1:])) or base + offs[-1] > end:
        return
    k = n + 1 if 8 + 4 * n <= end - base and u32(d, base + 4 + 4 * n) == end - base else n
    if not any(skel[base:base + 4 + 4 * k]):
        return
    lay.read(d, "Directory", base, k)
    ends = [o & ~3 for o in offs] + [end - base]
    for i in range(n):
        nested(lay, d, skel, base + ends[i], base + ends[i + 1], depth + 1)


def convert(d, skel):
    """-> file-tables.json text (skel all zeros afterwards) or None when skel is already all zeros."""
    if not any(skel):
        return None
    lay = layout.Layout(0)
    lay.type("Directory", 4, [("count", 0, "u32")], tail=("offsets", "u32"))
    n = directory(d)
    if n and any(skel[:4 + 4 * n]):
        lay.read(d, "Directory", 0, n)
    if n:
        # the directories inside the entries (offsets from their own start)
        offs = [u32(d, 4 + 4 * i) & ~3 for i in range(u32(d, 0))] + [len(d)]
        for i in range(len(offs) - 1):
            nested(lay, d, skel, offs[i], offs[i + 1], 1)
    if lay.records:
        layout.finish(lay, d, skel)
    layout.words(lay, d, skel, 0, len(d))
    layout.finish(lay, d, skel)
    assert not any(skel)
    return lay.text("the rest of the file (tools/assets/filetables.py, tools/assets/layout.py): its directory and runs "
                    "of words whose structure is not known yet, at their file offsets")
