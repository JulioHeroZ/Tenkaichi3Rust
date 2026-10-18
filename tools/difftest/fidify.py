"""Rewrites the address-taken uses of every function in a unit's LLVM IR to its function id (the PS2 address the
translated program uses as the function's value), so function pointers agree between the two programs; calls
stay direct and indirect calls reach the code through trampolines placed at the ids."""
import re, sys
sys.path.insert(0, __import__('os').path.dirname(__import__('os').path.abspath(__file__)))
from dt import parse_dispatch
SIG = {n: f for n, (f, _, _) in parse_dispatch().items()}

def fidify(text, tu):
    internal = set(re.findall(r'^define internal [^@]*@([\w.$]+)\(', text, re.M))
    funcs = set(re.findall(r'^(?:define|declare) [^@]*@([\w.$]+)\(', text, re.M))
    def fid(name):
        if name in internal:
            return SIG.get('%s__%s' % (tu, name))
        return SIG.get(name)
    def rep(m):
        if m.group(1) not in funcs:
            return m.group(0)
        f = fid(m.group(1))
        return 'inttoptr (i32 %d to ptr)' % f if f is not None else m.group(0)
    out = []
    for line in text.split('\n'):
        if line.startswith(('define ', 'declare ')):
            out.append(line)
        else:
            out.append(re.sub(r'@([\w.$]+)\b(?![\w.$(])', rep, line))
    return '\n'.join(out)

if __name__ == '__main__':
    src, dst, tu = sys.argv[1:4]
    open(dst, 'w').write(fidify(open(src).read(), tu))
