"""Differential tester: runs the same calls on the translated Rust (difftest) and the clang i386 build of the C (ref32)."""
import subprocess, struct, re, random, sys, os, select

from config import GEN, REF, REFDIR, TABLES
T = 0x50000000; TLEN = 0x10000

def parse_dispatch():
    sig = {}
    for l in open(GEN + '/src/gen/dispatch.rs'):
        m = re.match(r'\s+0x([0-9A-F]{8}) => (.*)', l)
        if not m: continue
        fid = int(m.group(1), 16); body = m.group(2)
        n = re.search(r'F::(\w+)\(c', body)
        if not n: continue
        args = re.findall(r'(F32\(\(g\(\d+\)\) as u32\)|\(g\(\d+\)\) as (\w+))', body)
        kinds = ['f' if a[0].startswith('F32') else a[1] for a in args]
        ret = 'void' if body.strip().endswith('0 },') else ('f' if ').0 as u64' in body else 'i')
        sig[n.group(1)] = (fid, kinds, ret)
    return sig

def ref_syms():
    """Reference code address of every translated function name (statics as <unit>__<name>), from the link map."""
    out = {}
    for l in open(os.path.join(REFDIR, 'ref32.map')):
        m = re.match(r'([0-9a-f]+) [0-9a-f]+ +[0-9a-f]+ +\d+ +\S*obj\d?/(\S+)\.o:\(\.text\.(\S+)\)', l)
        if m:
            a, obj, name = int(m.group(1), 16), m.group(2), m.group(3)
            out.setdefault(name, a)
            out['%s__%s' % (obj.split('__')[-1], name)] = a
    nm = subprocess.run(['nm', REF], capture_output=True, text=True).stdout
    for p in (l.split() for l in nm.splitlines()):
        if len(p) == 3 and p[1] == 'T':
            out.setdefault(p[2], int(p[0], 16))
    return out

class Proc:
    def __init__(self, argv, env=None):
        self.argv, self.env = argv, env; self.start()
    def start(self):
        self.p = subprocess.Popen(self.argv, stdin=subprocess.PIPE, stdout=subprocess.PIPE, env=self.env)
    def rx(self, n, timeout=5):
        b = b''
        while len(b) < n:
            r, _, _ = select.select([self.p.stdout], [], [], timeout)
            if not r: raise TimeoutError
            d = os.read(self.p.stdout.fileno(), n - len(b))
            if not d: raise EOFError
            b += d
        return b
    def tx(self, b): self.p.stdin.write(b); self.p.stdin.flush()
    def M(self, a, n): self.tx(b'M' + struct.pack('<II', a, n)); return struct.unpack('<i', self.rx(4))[0]
    def W(self, a, d): self.tx(b'W' + struct.pack('<II', a, len(d)) + d)
    def D(self, a, n): self.tx(b'D' + struct.pack('<II', a, n)); return self.rx(n)
    def C(self, f, args, timeout=5):
        a = list(args) + [0] * (12 - len(args))
        self.tx(b'C' + struct.pack('<14I', f, len(args), *a)); return struct.unpack('<Q', self.rx(8, timeout))[0]
    def kill(self): self.p.kill(); self.p.wait()

DATA = [(0x2BF600, 0x101000), (0x2000000, 0x100000)]

class Pair:
    """The two programs with the same memory: the reference maps the translated program's address space, gets
    its data and the reference's own globals are synchronised; function ids get trampolines to the reference code."""
    _maps = None
    def __init__(self):
        from mapping import load_map
        env = dict(os.environ, BT3_TABLES=TABLES)
        self.r = Proc([GEN + '/target/release/difftest'], env)
        self.c = Proc([REF])
        assert self.c.M(0x10000, 0x04000000 - 0x10000) == 0x10000
        assert self.c.M(0x0F000000, 0x100000) == 0x0F000000
        assert self.c.M(T, TLEN) == T
        self.r.M(T, TLEN)
        for a, n in DATA:
            self.c.W(a, self.r.D(a, n))
        if Pair._maps is None:
            Pair._maps = load_map()[0]
            sig = parse_dispatch(); rs = ref_syms()
            img = {0x100000: bytearray(0x1BF600), 0x0F000000: bytearray(0x100000)}
            for n, (f, _, _) in sig.items():
                if n in rs:
                    base = 0x100000 if f < 0x2BF600 else 0x0F000000
                    img[base][f - base:f - base + 5] = b'\xe9' + struct.pack('<i', rs[n] - (f + 5))
            Pair._tramp = img
        for ra, xa, n, kind, name in Pair._maps:
            if not kind.startswith('.rodata'):
                self.c.W(ra, self.r.D(xa, n))
        for base, img in Pair._tramp.items():
            self.c.W(base, bytes(img))
    def kill(self): self.r.kill(); self.c.kill()

def rand_region(rng):
    out = bytearray()
    for _ in range(TLEN // 4):
        k = rng.random()
        if k < 0.75: out += struct.pack('<f', rng.uniform(-100, 100))
        elif k < 0.85: out += struct.pack('<f', rng.choice([0.0, 1.0, -1.0, 0.5]))
        else: out += struct.pack('<I', rng.randrange(0, 8))
    return bytes(out)

def mkargs(kinds, rng):
    a = []
    for i, k in enumerate(kinds):
        if k == 'f': a.append(struct.unpack('<I', struct.pack('<f', rng.uniform(-10, 10)))[0])
        elif k == 'u32': a.append(T + 0x800 * i + 0x10 * rng.randrange(0, 4))
        else: a.append(rng.randrange(0, 4))
    return a

def run(names, iters=50, seed=1, verbose=False):
    sig = parse_dispatch(); rs = ref_syms()
    pr = Pair(); res = {}
    for n in names:
        if n not in sig or n not in rs: res[n] = 'missing'; continue
        fid, kinds, ret = sig[n]
        pr.kill(); pr = Pair()
        rng = random.Random(seed); bad = 0; first = None
        for it in range(iters):
            reg = rand_region(rng); args = mkargs(kinds, rng)
            try:
                pr.r.W(T, reg); pr.c.W(T, reg)
                a = pr.r.C(fid, args) & 0xffffffff
                b = pr.c.C(rs[n], args) & 0xffffffff
                ma = pr.r.D(T, TLEN); mb = pr.c.D(T, TLEN)
            except (TimeoutError, EOFError, BrokenPipeError) as e:
                res[n] = 'crash/timeout ' + type(e).__name__; pr.kill(); pr = Pair(); break
            if ret == 'void': a = b = 0
            if a != b or ma != mb:
                bad += 1
                if first is None:
                    diffs = [i for i in range(0, TLEN, 4) if ma[i:i+4] != mb[i:i+4]]
                    first = (it, [hex(x) for x in args], hex(a), hex(b), [(hex(T+i), ma[i:i+4].hex(), mb[i:i+4].hex()) for i in diffs[:6]], len(diffs))
        else:
            res[n] = 'ok' if not bad else f'DIFF {bad}/{iters} {first}'
        if verbose: print(n, res[n], flush=True)
    pr.kill()
    return res

if __name__ == '__main__':
    pat = sys.argv[1]
    sig = parse_dispatch()
    names = sorted(n for n in sig if re.match(pat, n))
    for n, r in run(names, verbose=False).items():
        print(f'{n:40s} {r}', flush=True)

def rand_region(rng):
    out = bytearray()
    for _ in range(TLEN // 4):
        k = rng.random()
        if k < 0.75: out += struct.pack('<f', rng.uniform(-100, 100))
        elif k < 0.85: out += struct.pack('<f', rng.choice([0.0, 1.0, -1.0, 0.5]))
        else: out += struct.pack('<I', rng.randrange(0, 8))
    return bytes(out)

def mkargs(kinds, rng):
    a = []
    for i, k in enumerate(kinds):
        if k == 'f': a.append(struct.unpack('<I', struct.pack('<f', rng.uniform(-10, 10)))[0])
        elif k == 'u32': a.append(T + 0x800 * i + 0x10 * rng.randrange(0, 4))
        else: a.append(rng.randrange(0, 4))
    return a

def run(names, iters=50, seed=1, verbose=False):
    sig = parse_dispatch(); rs = ref_syms()
    pr = Pair(); res = {}
    for n in names:
        if n not in sig or n not in rs: res[n] = 'missing'; continue
        fid, kinds, ret = sig[n]
        pr.kill(); pr = Pair()
        rng = random.Random(seed); bad = 0; first = None
        for it in range(iters):
            reg = rand_region(rng); args = mkargs(kinds, rng)
            try:
                pr.r.W(T, reg); pr.c.W(T, reg)
                a = pr.r.C(fid, args) & 0xffffffff
                b = pr.c.C(rs[n], args) & 0xffffffff
                ma = pr.r.D(T, TLEN); mb = pr.c.D(T, TLEN)
            except (TimeoutError, EOFError, BrokenPipeError) as e:
                res[n] = 'crash/timeout ' + type(e).__name__; pr.kill(); pr = Pair(); break
            if ret == 'void': a = b = 0
            if a != b or ma != mb:
                bad += 1
                if first is None:
                    diffs = [i for i in range(0, TLEN, 4) if ma[i:i+4] != mb[i:i+4]]
                    first = (it, [hex(x) for x in args], hex(a), hex(b), [(hex(T+i), ma[i:i+4].hex(), mb[i:i+4].hex()) for i in diffs[:6]], len(diffs))
        else:
            res[n] = 'ok' if not bad else f'DIFF {bad}/{iters} {first}'
        if verbose: print(n, res[n], flush=True)
    pr.kill()
    return res

if __name__ == '__main__':
    pat = sys.argv[1]
    sig = parse_dispatch()
    names = sorted(n for n in sig if re.match(pat, n))
    for n, r in run(names, iters=int(os.environ.get('IT', 50))).items():
        print(f'{n:40s} {r}', flush=True)
