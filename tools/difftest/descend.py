"""Finds the translated function whose own code diverges from the reference: probes calls in the translated game,
replays them on the reference build, and descends into the first callee call that also diverges."""
import os, re, sys, subprocess, shutil
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import replay
from replay import LO, STK

from config import GEN, WORK, TABLES, GAMEDATA
ENV = dict(os.environ, BT3_DEMO='1', BT3_SEED='0x12345678', BT3_DATA=GAMEDATA, BT3_TABLES=TABLES)
BATCH = 6

def bodies():
    out = {}
    for f in os.listdir(GEN + '/src/tu'):
        src = open(GEN + '/src/tu/' + f).read()
        for m in re.finditer(r'^pub fn (\w+)\(c: &mut Ctx.*?^}\n', src, re.S | re.M):
            out[m.group(1)] = m.group(0)
    return out

BODIES = bodies()

def _writers():
    """Functions that can change memory: a direct store, an indirect call, or a call of such a function."""
    direct, calls = set(), {}
    for n, b in BODIES.items():
        b = re.sub(r'crate::host::harness::probe\([^;]*\);', '', b)
        if re.search(r'c\.m\.w|D::call|rt::mem|bf_set|c\.host', b):
            direct.add(n)
        calls[n] = set(re.findall(r'F::(\w+)\(c', b))
    w = set(direct) | {g for cs in calls.values() for g in cs if g not in BODIES}  # host functions: assume
    changed = True
    while changed:
        changed = False
        for n, cs in calls.items():
            if n not in w and cs & w:
                w.add(n); changed = True
    return w

WRITERS = _writers()

def pure_leaf(name):
    return name in BODIES and name not in WRITERS

def callees(name):
    seen = []
    for g in re.findall(r'F::(\w+)\(c', BODIES.get(name, '')):
        if g not in seen and g != name and not pure_leaf(g):
            seen.append(g)
    return seen

def run_probes(items):
    d = '%s/run_%d' % (WORK, os.getpid())
    shutil.rmtree(d, ignore_errors=True); os.makedirs(d)
    spec = ','.join('%s:%d' % it for it in items)
    subprocess.run([GEN + '/target/release/bt3-original'], cwd=d, env=dict(ENV, BT3_PROBE=spec),
                   stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, timeout=900)
    return d

def cnt(f):
    return {l.split()[0]: int(l.split()[1]) for l in open(f)} if os.path.exists(f) else {}

def check(d, name, n):
    p = '%s/probe_%s_%d' % (d, name, n)
    if not os.path.exists(p + '_post.bin'):
        return 'not reached', None
    pre = open(p + '_pre.bin', 'rb').read(); post = open(p + '_post.bin', 'rb').read()
    sp, *args = open(p + '.args').read().split()
    args = [int(a, 16) for a in args]
    c, maps, rs = replay.setup(pre)
    kids = [(l.split()[0], int(l.split()[1])) for l in open(p + '.children')] if os.path.exists(p + '.children') else None
    counts = (cnt(p + '.counts'), cnt(p + '.counts_post'), kids)
    if name not in rs:
        c.kill(); return 'no reference function', counts
    try:
        c.C(rs[name], args, timeout=120)
    except Exception as e:
        c.kill(); return 'reference crashed (%s)' % type(e).__name__, counts
    got = bytearray(LO) + bytearray(c.D(LO, STK - LO))
    for ra, xa, nn, kind, nm in maps:
        if not kind.startswith('.rodata'):
            got[xa:xa + nn] = c.D(ra, nn)
    c.kill()
    for f in replay.setup.fids:
        got[f:f + 5] = post[f:f + 5]
    diffs = differing_words(got, post)
    return (diffs, got, post), counts

def differing_words(got, post):
    """Addresses of the words that differ below the frame stack, without the reference's own stack and code
    addresses (pointers into those are expected to differ)."""
    import numpy as np
    a = np.frombuffer(bytes(got[LO:STK]), dtype='<u4'); b = np.frombuffer(post[LO:STK], dtype='<u4')
    idx = np.nonzero(a != b)[0]
    keep = [(LO + 4 * int(i)) for i in idx if not ((int(a[i]) >> 24) in (0xff, 0x60) or (int(b[i]) >> 24) == 0x03)]
    return keep

SY = None
def report(ind, name, n, res):
    global SY
    SY = SY or replay.symbolizer()
    if isinstance(res, str):
        print(ind + '%s#%d: %s' % (name, n, res), flush=True); return None
    d, got, post = res
    print(ind + '%s#%d: %d differing words' % (name, n, len(d)), flush=True)
    for i in d[:8]:
        print(ind + '    %08x %-34s rust %s ref %s' % (i, SY(i), post[i:i + 4].hex(), got[i:i + 4].hex()), flush=True)
    return len(d)

def descend(name, n, d=None, depth=0):
    ind = '  ' * depth
    if d is None:
        d = run_probes([(name, n)])
    res, counts = check(d, name, n)
    nd = report(ind, name, n, res)
    if not nd:
        return
    c0, c1, kids = counts
    if kids is not None:
        items = [(g, k) for g, k in kids if not pure_leaf(g)]
    else:
        items = [(g, k) for g in callees(name) for k in range(c0.get(g, 0) + 1, c1.get(g, 0) + 1)]
    for i in range(0, len(items), BATCH):
        batch = items[i:i + BATCH]
        bd = run_probes(batch)
        for g, k in batch:
            r, cc = check(bd, g, k)
            if report(ind + '  ', g, k, r):
                keep = bd + '_keep'
                shutil.rmtree(keep, ignore_errors=True); shutil.move(bd, keep)
                return descend(g, k, keep, depth + 1) or True
        shutil.rmtree(bd, ignore_errors=True)
    print(ind + '=> the divergence is in the body of %s (call #%d)' % (name, n), flush=True)
    return True

if __name__ == '__main__':
    descend(sys.argv[1], int(sys.argv[2]))
