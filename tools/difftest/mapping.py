"""Pairs every data object of the reference build (ref32.map) with its address in the translated program (sym.rs)."""
import os, re
from config import GEN, REFDIR
def load_sym(path=os.path.join(GEN, 'src', 'gen', 'sym.rs')):
    s = {}
    for l in open(path):
        m = re.match(r'pub const (\w+): u32 = (0x[0-9A-Fa-f]+);', l)
        if m: s[m.group(1)] = int(m.group(2), 16)
    return s
def load_map(path=os.path.join(REFDIR, 'ref32.map'), sym=None):
    sym = sym or load_sym()
    out = []; miss = []
    for l in open(path):
        m = re.match(r'([0-9a-f]+) [0-9a-f]+ +([0-9a-f]+) +\d+ +\S*obj\d?/(\S+)\.o:\((\.(?:data|bss|rodata)(?:\.rel\.ro)?)\.(\S+)\)', l)
        if not m: continue
        addr, size, obj, kind, name = int(m.group(1), 16), int(m.group(2), 16), m.group(3), m.group(4), m.group(5)
        if size == 0 or name.startswith(('.L', 'str')): continue
        tu = obj.split('__')[-1]
        if '.' in name:
            f, v = name.split('.', 1); cands = ['%s__%s__%s' % (tu, f, v)]
        else:
            cands = [name, '%s__%s' % (tu, name)]
        for c in cands:
            if c in sym:
                out.append((addr, sym[c], size, kind, c)); break
        else:
            miss.append((obj, kind, name))
    return out, miss
if __name__ == '__main__':
    o, m = load_map()
    print(len(o), 'mapped', len(m), 'missing'); print(m[:30])
