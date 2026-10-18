"""assets/data/tables.txt from the user's own executable (SLUS_216.78): the bytes of every table run the
decompilation leaves to the original executable (game/src/gen/data.rs ORIGINAL_RUNS: address and length), which the
game reads when it starts (game/src/host/tables.rs). The program carries none of the game's data, so it is built
without the disc and the same executable works for everyone.

A run outside the executable's file data is zeros, as are the two runs of overlay data the decompilation places
over text (ZERO_RUNS). Checked against the typed tables of the decompilation on the same executable: the 2388 runs
are byte for byte the same.

    python3 tables_from_elf.py <SLUS_216.78> <game/src/gen/data.rs> <out tables.txt>
"""
import re, struct, sys

# runs that are zeros in the game's tables although the executable has bytes at their addresses
ZERO_RUNS = {0x0017C201, 0x0017C204}


def segments(elf):
    if elf[:4] != b"\x7fELF":
        raise SystemExit("not an ELF file")
    phoff = struct.unpack_from("<I", elf, 0x1C)[0]
    size, count = struct.unpack_from("<HH", elf, 0x2A)
    out = []
    for i in range(count):
        o = phoff + i * size
        kind, offset, vaddr, _, filesz = struct.unpack_from("<5I", elf, o)
        if kind == 1:
            out.append((vaddr, offset, filesz))
    return out


def run_bytes(elf, segs, addr, n):
    if addr in ZERO_RUNS:
        return bytes(n)
    for vaddr, offset, filesz in segs:
        if vaddr <= addr and addr + n <= vaddr + filesz:
            o = offset + addr - vaddr
            return elf[o:o + n]
    return bytes(n)


def main():
    elf = open(sys.argv[1], "rb").read()
    data_rs = open(sys.argv[2]).read()
    body = data_rs[data_rs.index("ORIGINAL_RUNS"):]
    body = body[:body.index("];")]
    runs = [(int(a, 16), int(n)) for a, n in re.findall(r"\(0x([0-9A-Fa-f]+), \d+, 0x[0-9A-Fa-f]+, (\d+)\)", body)]
    if not runs:
        raise SystemExit("no ORIGINAL_RUNS in data.rs")
    segs = segments(elf)
    lines = ["# the game's data tables, from your executable (tools/translate/c2rs/tables_from_elf.py): per line the PS2",
             "# address of a table run and its bytes in hex; the game writes them into its memory when it starts"]
    for addr, n in runs:
        lines.append("%08X %s" % (addr, run_bytes(elf, segs, addr, n).hex().upper()))
    open(sys.argv[3], "w").write("\n".join(lines) + "\n")
    print("%d table runs written to %s" % (len(runs), sys.argv[3]))


if __name__ == "__main__":
    main()
