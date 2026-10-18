"""Both programs started from the same memory snapshot of the translated game; calls are run on both and the
memory they leave compared, so any function can be checked at a real game state."""
import os, sys, struct
from dt import Proc, GEN, TABLES, parse_dispatch, ref_syms
import replay

LO, STK, HI = 0x10000, 0x03000000, 0x04000000

class Twin:
    def __init__(self, pre_path, args_path):
        self.pre = open(pre_path, 'rb').read()
        sp, *_ = open(args_path).read().split()
        self.sp = int(sp, 0)
        self.c, self.maps, self.rs = replay.setup(self.pre)
        self.r = Proc([GEN + '/target/release/difftest'], dict(os.environ, BT3_TABLES=TABLES))
        for a in range(LO, HI, 1 << 22):
            self.r.W(a, self.pre[a:min(HI, a + (1 << 22))])
        self.r.tx(b'S' + struct.pack('<II', self.sp, 0))
        self.sig = parse_dispatch()
        self.sy = replay.symbolizer()

    def call(self, name, args):
        a = self.r.C(self.sig[name][0], args, timeout=120)
        b = self.c.C(self.rs[name], args, timeout=120)
        return a, b

    def mem(self):
        x = bytearray(LO) + bytearray(self.r.D(LO, STK - LO))
        y = bytearray(LO) + bytearray(self.c.D(LO, STK - LO))
        for ra, xa, n, kind, nm in self.maps:
            if not kind.startswith('.rodata'):
                y[xa:xa + n] = self.c.D(ra, n)
        for f in replay.setup.fids:
            y[f:f + 5] = x[f:f + 5]
        return x, y

    def compare(self, limit=40, quiet=False):
        x, y = self.mem()
        d = [i for i in range(LO, STK, 4) if x[i:i + 4] != y[i:i + 4]]
        # pointers to the reference's own stack or code are expected
        d = [i for i in d if not (y[i + 3] in (0xff, 0x60) or x[i + 3] == 0x03)]
        if not quiet:
            for i in d[:limit]:
                print('  %08x %-36s rust %s ref %s' % (i, self.sy(i), x[i:i + 4].hex(), y[i:i + 4].hex()))
        return d

    def sync(self):
        """Copy the translated program's memory into the reference (to continue from an agreed state)."""
        x = self.r.D(LO, STK - LO)
        self.c.W(LO, x)  # trampolines live below the data, rewrite them
        for f, b in replay.setup.tramps:
            self.c.W(f, b)
        for ra, xa, n, kind, nm in self.maps:
            if not kind.startswith('.rodata'):
                self.c.W(ra, x[xa - LO:xa - LO + n])
