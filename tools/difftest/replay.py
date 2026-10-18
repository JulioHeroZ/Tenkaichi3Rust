"""Replays one probed call of the translated game on the reference build and compares the memory it leaves."""
import sys, struct, bisect
from dt import Proc, REF, parse_dispatch, ref_syms
from mapping import load_map, load_sym

LO, HI, STK = 0x10000, 0x04000000, 0x03000000
EXTRA_FN = (0x0F000000, 0x00100000)

def symbolizer():
    s = load_sym(); items = sorted((a, n) for n, a in s.items())
    addrs = [a for a, _ in items]
    def f(a):
        i = bisect.bisect_right(addrs, a) - 1
        return '%s+%#x' % (items[i][1], a - items[i][0]) if i >= 0 and a - items[i][0] < 0x100000 else 'heap?'
    return f


def setup(pre):
    c = Proc([REF])
    assert c.M(LO, HI - LO) == LO
    assert c.M(*EXTRA_FN) == EXTRA_FN[0]
    for a in range(LO, HI, 1 << 22):
        c.W(a, pre[a:min(HI, a + (1 << 22))])
    maps, _ = load_map()
    for ra, xa, n, kind, name in maps:
        if kind.startswith('.rodata'):
            got = c.D(ra, n)
            if got != pre[xa:xa + n]:
                print('rodata differs:', name)
        else:
            c.W(ra, pre[xa:xa + n])
    if not hasattr(setup, 'img'):
        sig = parse_dispatch(); rs = ref_syms()
        setup.rs = rs
        setup.tramps = [(fid, b'\xe9' + struct.pack('<i', rs[name] - (fid + 5))) for name, (fid, _, _) in sig.items() if name in rs]
        setup.fids = [f for f, _ in setup.tramps if f < HI]
        img = {}
        for f, b in setup.tramps:
            base = f & ~0xFFFFF
            img.setdefault(base, bytearray(0x100000))[f - base:f - base + 5] = b
        setup.img = img
    rs = setup.rs
    for base, im in setup.img.items():
        # the code region of the translated program's memory is empty: the trampolines are all there is
        blk = bytearray(pre[base:base + 0x100000]) if base < HI else bytearray(0x100000)
        nz = [i for i in range(0, 0x100000, 4096) if any(im[i:i + 4096])]
        for i in nz:
            seg = bytes(a | b for a, b in zip(blk[i:i + 4096], im[i:i + 4096]))
            c.W(base + i, seg)
    return c, maps, rs

def main(name, pre_p, post_p, args_p):
    pre = open(pre_p, 'rb').read(); post = open(post_p, 'rb').read()
    sp, *args = open(args_p).read().split()
    args = [int(a, 16) for a in args]
    c, maps, rs = setup(pre)
    r = c.C(rs[name], args, timeout=60)
    got = bytearray(c.D(LO, STK - LO)); got = bytearray(LO) + got
    for ra, xa, n, kind, nm in maps:
        if not kind.startswith('.rodata'):
            got[xa:xa + n] = c.D(ra, n)
    for f in setup.fids: got[f:f + 5] = post[f:f + 5]
    sy = symbolizer(); diffs = []
    i = LO
    while i < STK:
        if got[i:i + 4] != post[i:i + 4]:
            diffs.append(i)
        i += 4
    print('return', hex(r), 'diff words:', len(diffs))
    for d in diffs[:80]:
        print('%08x %-40s rust %s ref %s pre %s' % (d, sy(d), post[d:d + 4].hex(), got[d:d + 4].hex(), pre[d:d + 4].hex()))

if __name__ == '__main__':
    main(*sys.argv[1:5])
