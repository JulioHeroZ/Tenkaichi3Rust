"""C types as clang prints them, resolved to canonical form for an i686 (-m32 -malign-double) target."""
import re

class T:
    kind = "?"
    def __repr__(self): return self.show()

class Void(T):
    kind = "void"
    def show(self): return "void"

class Int(T):
    kind = "int"
    def __init__(self, size, signed, boolean=False):
        self.size, self.signed, self.boolean = size, signed, boolean
    def show(self): return ("bool" if self.boolean else ("i" if self.signed else "u") + str(self.size * 8))
    def __eq__(self, o): return isinstance(o, Int) and (o.size, o.signed, o.boolean) == (self.size, self.signed, self.boolean)
    def __hash__(self): return hash((self.size, self.signed))

class Float(T):
    kind = "float"
    def __init__(self, size): self.size = size
    def show(self): return "f%d" % (self.size * 8)
    def __eq__(self, o): return isinstance(o, Float) and o.size == self.size
    def __hash__(self): return hash(self.size)

class Ptr(T):
    kind = "ptr"
    size = 4
    def __init__(self, to): self.to = to
    def show(self): return "*" + self.to.show()

class Array(T):
    kind = "array"
    def __init__(self, elem, n): self.elem, self.n = elem, n
    def show(self): return "[%s;%s]" % (self.elem.show(), self.n)

class Func(T):
    kind = "func"
    def __init__(self, ret, params, variadic, proto):
        self.ret, self.params, self.variadic, self.proto = ret, params, variadic, proto
    def show(self): return "fn(%s%s)->%s" % (",".join(p.show() for p in self.params), ",..." if self.variadic else "", self.ret.show())

class Record(T):
    kind = "record"
    def __init__(self, tag, key):
        self.tag, self.key = tag, key  # tag: struct/union; key: name or '@loc'
    def show(self): return "%s %s" % (self.tag, self.key)
    def __eq__(self, o): return isinstance(o, Record) and o.key == self.key and o.tag == self.tag
    def __hash__(self): return hash(self.key)

BUILTIN = {
    ("void",): Void(), ("_Bool",): Int(1, False, True), ("char",): Int(1, True), ("signed", "char"): Int(1, True),
    ("unsigned", "char"): Int(1, False), ("short",): Int(2, True), ("short", "int"): Int(2, True),
    ("signed", "short"): Int(2, True), ("unsigned", "short"): Int(2, False), ("unsigned", "short", "int"): Int(2, False),
    ("int",): Int(4, True), ("signed",): Int(4, True), ("signed", "int"): Int(4, True), ("unsigned",): Int(4, False),
    ("unsigned", "int"): Int(4, False), ("long",): Int(4, True), ("long", "int"): Int(4, True), ("signed", "long"): Int(4, True),
    ("unsigned", "long"): Int(4, False), ("unsigned", "long", "int"): Int(4, False), ("long", "long"): Int(8, True),
    ("long", "long", "int"): Int(8, True), ("signed", "long", "long"): Int(8, True), ("unsigned", "long", "long"): Int(8, False),
    ("unsigned", "long", "long", "int"): Int(8, False), ("float",): Float(4), ("double",): Float(8), ("long", "double"): Float(8),
    ("__int128",): Int(16, True), ("unsigned", "__int128",): Int(16, False),
}
QUAL = {"const", "volatile", "restrict", "__restrict", "__unaligned"}
TOK = re.compile(r"\s*(?:((?:\w+::)*\((?:unnamed|anonymous)[^)]*\))|(__attribute__\(\((?:[^()]|\([^()]*\))*\)\))|([A-Za-z_]\w*)|(\d+)|(\.\.\.)|([*()\[\],]))")

def loc_key(s):
    m = re.search(r"at ([^)]+)\)", s)
    return "@" + (m.group(1) if m else s)

class Parser:
    def __init__(self, typedefs, enums, varty=None):
        self.typedefs, self.enums = typedefs, enums  # typedef name -> qualType string ; enum key -> Int
        self.varty = varty if varty is not None else {}
        self.cache = {}

    def parse(self, s):
        if s in self.cache:
            return self.cache[s]
        toks = []
        pos = 0
        s2 = s.strip()
        while pos < len(s2):
            m = TOK.match(s2, pos)
            if not m or m.end() == pos:
                raise ValueError("cannot tokenize type %r at %d" % (s, pos))
            pos = m.end()
            if m.group(2):
                continue  # attributes: alignment handled by layouts
            toks.append(next(g for g in m.groups() if g is not None))
        self.toks, self.i = toks, 0
        t = self.type_name()
        if self.i != len(toks):
            raise ValueError("trailing tokens in %r: %r" % (s, toks[self.i:]))
        self.cache[s] = t
        return t

    def peek(self):
        return self.toks[self.i] if self.i < len(self.toks) else None

    def next(self):
        t = self.toks[self.i]; self.i += 1; return t

    def specifiers(self):
        words, base = [], None
        while True:
            t = self.peek()
            if t is None:
                break
            if t in QUAL:
                self.i += 1; continue
            if t in ("typeof", "__typeof__", "__typeof"):
                self.i += 1
                assert self.next() == "("
                name = self.next()
                assert self.next() == ")"
                p = Parser(self.typedefs, self.enums, self.varty)
                base = p.parse(self.varty[name])
                continue
            if t in ("struct", "union", "enum"):
                self.i += 1
                n = self.next()
                if "(" in n:
                    key = loc_key(n)
                else:
                    key = n
                base = self.enums.get(key, Int(4, True)) if t == "enum" else Record(t, key)
                continue
            if "(unnamed" in t or "(anonymous" in t:  # bare (unnamed struct at ...)
                if "union" in t: base = Record("union", loc_key(t)); self.i += 1; continue
                if "enum" in t: base = Int(4, True); self.i += 1; continue
                if "struct" in t or "anonymous" in t: base = Record("struct", loc_key(t)); self.i += 1; continue
                break
            if t in ("void", "_Bool", "char", "short", "int", "long", "signed", "unsigned", "float", "double", "__int128"):
                words.append(t); self.i += 1; continue
            if re.match(r"[A-Za-z_]\w*$", t) and base is None and not words and t in self.typedefs:
                self.i += 1
                base = self.resolve_typedef(t)
                continue
            if re.match(r"[A-Za-z_]\w*$", t) and base is None and not words:
                raise ValueError("unknown type name %r" % t)
            break
        if words:
            key = tuple(words)
            if key not in BUILTIN:
                raise ValueError("builtin %r" % (key,))
            return BUILTIN[key]
        if base is None:
            raise ValueError("no type specifier in %r" % (self.toks,))
        return base

    def resolve_typedef(self, name):
        v = self.typedefs[name]
        if isinstance(v, T):
            return v
        if v == "__builtin_va_list" or name == "__builtin_va_list":
            return Ptr(Int(1, True))
        saved = (self.toks, self.i)
        p = Parser(self.typedefs, self.enums, self.varty)
        p.cache = self.cache
        t = p.parse(v)
        self.typedefs[name] = t
        self.toks, self.i = saved
        return t

    def type_name(self):
        base = self.specifiers()
        return self.abstract(base)

    def abstract(self, base):
        # pointers
        ptrs = 0
        while self.peek() == "*":
            self.i += 1; ptrs += 1
            while self.peek() in QUAL:
                self.i += 1
        # direct abstract declarator: may be '(' declarator ')' or suffixes
        inner = None
        if self.peek() == "(" and self.is_nested_declarator():
            self.i += 1
            mark = self.i
            depth = 1
            while depth:
                t = self.next()
                depth += (t == "(") - (t == ")")
            inner = self.toks[mark:self.i - 1]
        t = base
        for _ in range(ptrs):
            t = Ptr(t)
        t = self.suffixes(t)
        if inner is not None:
            saved = (self.toks, self.i)
            self.toks, self.i = inner, 0
            t = self.abstract(t)
            self.toks, self.i = saved
        return t

    def is_nested_declarator(self):
        n = self.toks[self.i + 1] if self.i + 1 < len(self.toks) else None
        return n in ("*", "(", "[")

    def suffixes(self, t):
        # collect suffixes left to right, apply right to left (int [2][3] = array 2 of array 3)
        sufs = []
        while self.peek() in ("[", "("):
            if self.next() == "[":
                n = None
                if self.peek() != "]":
                    n = int(self.next())
                assert self.next() == "]"
                sufs.append(("a", n))
            else:
                params, variadic = [], False
                if self.peek() == ")":
                    self.i += 1
                    sufs.append(("f", [], False, False))
                    continue
                while True:
                    if self.peek() == "...":
                        self.i += 1; variadic = True
                    else:
                        params.append(self.param())
                    t2 = self.next()
                    if t2 == ")":
                        break
                    assert t2 == ",", t2
                if len(params) == 1 and isinstance(params[0], Void):
                    params = []
                sufs.append(("f", params, variadic, True))
        for s in reversed(sufs):
            if s[0] == "a":
                t = Array(t, s[1])
            else:
                t = Func(t, s[1], s[2], s[3])
        return t

    def param(self):
        base = self.specifiers()
        t = self.abstract(base)
        if isinstance(t, Array):
            t = Ptr(t.elem)
        elif isinstance(t, Func):
            t = Ptr(t)
        return t
