"""The user's own disc image (ISO) to the loose files the project works from: assets/_legacy/raw/disc/ (the disc's
files) and assets/_legacy/raw/pzs3us0..2/NNNNN.bin (one file per entry of each AFS archive, named by its index,
the id the game asks for). Reads the ISO 9660 file system itself (no 7-Zip needed). Nothing extracted may be
committed or redistributed.

    python3 extract_iso.py <disc.iso> [raw folder, default assets/_legacy/raw]
"""
import os, struct, sys

SECTOR = 2048


def walk(f, lba, size, path=""):
    """-> [(path, lba, size)] of the files under a directory extent."""
    out = []
    f.seek(lba * SECTOR)
    data = f.read(size)
    o = 0
    while o < len(data):
        n = data[o]
        if n == 0:
            o = (o // SECTOR + 1) * SECTOR  # records do not cross sectors
            continue
        rec = data[o:o + n]
        o += n
        ext, length, flags, nlen = struct.unpack_from("<I", rec, 2)[0], struct.unpack_from("<I", rec, 10)[0], rec[25], rec[32]
        name = rec[33:33 + nlen]
        if name in (b"\0", b"\1"):
            continue
        name = name.decode("ascii", "replace").split(";")[0].rstrip(".")
        full = path + "/" + name if path else name
        if flags & 2:
            out += walk(f, ext, length, full)
        else:
            out.append((full, ext, length))
    return out


def files(iso):
    with open(iso, "rb") as f:
        f.seek(16 * SECTOR)
        pvd = f.read(SECTOR)
        if pvd[1:6] != b"CD001" or pvd[0] != 1:
            raise SystemExit("%s: not an ISO 9660 image (no primary volume descriptor)" % iso)
        root = pvd[156:190]
        return walk(f, struct.unpack_from("<I", root, 2)[0], struct.unpack_from("<I", root, 10)[0])


def copy(f, lba, size, dest):
    os.makedirs(os.path.dirname(dest), exist_ok=True)
    if os.path.exists(dest) and os.path.getsize(dest) == size:
        return
    f.seek(lba * SECTOR)
    tmp = dest + ".tmp"
    with open(tmp, "wb") as out:
        left = size
        while left:
            b = f.read(min(left, 1 << 22))
            if not b:
                raise SystemExit("the image ends inside %s" % dest)
            out.write(b)
            left -= len(b)
    os.replace(tmp, dest)


def split_afs(f, lba, size, out):
    """The entries of an AFS archive inside the image -> out/NNNNN.bin."""
    base = lba * SECTOR
    f.seek(base)
    magic, count = struct.unpack("<4sI", f.read(8))
    if magic != b"AFS\0":
        raise SystemExit("not an AFS archive")
    table = struct.unpack("<%dI" % (count * 2), f.read(count * 8))
    os.makedirs(out, exist_ok=True)
    for i in range(count):
        off, n = table[i * 2], table[i * 2 + 1]
        dest = os.path.join(out, "%05d.bin" % i)
        if os.path.exists(dest) and os.path.getsize(dest) == n:
            continue
        f.seek(base + off)
        b = f.read(n)
        with open(dest + ".tmp", "wb") as o:
            o.write(b)
        os.replace(dest + ".tmp", dest)
    return count


def main():
    iso = sys.argv[1]
    raw = sys.argv[2] if len(sys.argv) > 2 else os.path.join("assets", "_legacy", "raw")
    entries = files(iso)
    if not any(p.upper() == "SLUS_216.78" for p, _, _ in entries):
        raise SystemExit("%s: this is not Budokai Tenkaichi 3 (US, SLUS-21678)" % iso)
    with open(iso, "rb") as f:
        for path, lba, size in entries:
            up = path.upper()
            if up.endswith(".AFS"):
                part = os.path.splitext(os.path.basename(path))[0].lower()
                n = split_afs(f, lba, size, os.path.join(raw, part))
                print("%s: %d files" % (path, n), flush=True)
            elif up.startswith(("DATA/", "BIN/")) or up == "SLUS_216.78":
                copy(f, lba, size, os.path.join(raw, "disc", *path.split("/")))
    print("done: %s" % raw)


if __name__ == "__main__":
    main()
