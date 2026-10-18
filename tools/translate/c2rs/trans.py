"""C (clang JSON AST of the port's translation units) -> Rust over the PS2 memory model."""
import json, re, sys, pathlib, struct
from fractions import Fraction
from ctype import Int, Float, Ptr, Array, Func, Record, Void
import emit
from emit import rty, zero, load, store, cast, truth, adapt, to_raw, from_raw

INT = Int(4, True)
UINT = Int(4, False)
KEYWORDS = set("as break const continue crate else enum extern false fn for if impl in let loop match mod move mut pub ref "
               "return self Self static struct super trait true type unsafe use where while async await dyn abstract become "
               "box do final macro override priv typeof unsized virtual yield try gen union".split())

def strip(n):
    while n.get("kind") in ("ParenExpr",):
        n = n["inner"][0]
    return n

def kids(n):
    return n.get("inner", [])

class Unsupported(Exception):
    pass

class LV:
    """An lvalue: ('local', name) | ('mem', addr) | ('bit', addr, first, width)."""
    def __init__(self, kind, t, a, first=None, width=None):
        self.kind, self.t, self.a, self.first, self.width = kind, t, a, first, width

def parse_c_string(text):
    """Bytes of one or more adjacent C string literals (source text, latin-1)."""
    out = bytearray()
    i = 0
    while i < len(text):
        ch = text[i]
        if ch == '"':
            i += 1
            while text[i] != '"':
                c = text[i]
                if c == "\\":
                    i += 1
                    e = text[i]
                    simple = {"n": 10, "t": 9, "r": 13, "0": None, "a": 7, "b": 8, "f": 12, "v": 11, "\\": 92, "'": 39, '"': 34, "?": 63, "e": 27}
                    if e == "x":
                        j = i + 1
                        while j < len(text) and text[j] in "0123456789abcdefABCDEF":
                            j += 1
                        out.append(int(text[i + 1:j], 16) & 0xFF)
                        i = j
                        continue
                    if e in "01234567":
                        j = i
                        while j < len(text) and j < i + 3 and text[j] in "01234567":
                            j += 1
                        out.append(int(text[i:j], 8) & 0xFF)
                        i = j
                        continue
                    out.append(simple[e])
                    i += 1
                    continue
                out.append(ord(c) & 0xFF)
                i += 1
            i += 1
        else:
            i += 1
    return bytes(out)

def float_literal(text):
    t = text.rstrip("fFlL")
    if t.lower().startswith("0x"):
        m = re.match(r"0[xX]([0-9a-fA-F]*)(?:\.([0-9a-fA-F]*))?[pP]([+-]?\d+)", t)
        whole, frac, exp = m.group(1) or "0", m.group(2) or "", int(m.group(3))
        v = Fraction(int(whole + frac, 16), 16 ** len(frac)) * Fraction(2) ** exp
        return v
    return Fraction(t)

def f32_bits_nearest(v):
    return struct.unpack("<I", struct.pack("<f", float(v)))[0]

def f64_bits(v):
    return struct.unpack("<Q", struct.pack("<d", float(v)))[0]

def exact_f32_bits(v):
    """Bits of a value that eeconst made exactly representable as a float (else nearest)."""
    return f32_bits_nearest(v)

class Gen:
    def __init__(self, tu, index):
        self.tu, self.index = tu, index
        self.native = tu.native_float
        self.out_globals = []   # data definitions for the linker
        self.strings = []       # (sym, bytes)
        self.statics = {}       # decl id -> symbol for function-local statics
        self.fn_static = {}     # name -> is static (functions in this TU)
        self.var_static = {}
        for n in tu.decls:
            if n.get("kind") == "FunctionDecl" and n.get("storageClass") == "static":
                self.fn_static[n["name"]] = True
            if n.get("kind") == "VarDecl" and n.get("storageClass") == "static":
                self.var_static[n["name"]] = True
        self.calls_host = set()
        self.str_count = 0

    # ------------------------------------------------------------------ names
    def fsym(self, name):
        name = self.tu.asm_label.get(name, name)
        if self.fn_static.get(name):
            return "%s__%s" % (self.tu.id, name)
        return name

    def vsym(self, name):
        name = self.tu.asm_label.get(name, name)
        if self.var_static.get(name):
            return "%s__%s" % (self.tu.id, name)
        return name

    def ty(self, n):
        return self.tu.ty(n["type"])

    # ------------------------------------------------------------------ strings
    def string(self, node):
        data = parse_c_string(self.tu.text(node)) + b"\0"
        t = self.ty(node)
        if isinstance(t, Array) and t.n:
            data = (data + b"\0" * t.n)[:t.n]
        sym = "%s__str%d" % (self.tu.id, self.str_count)
        self.str_count += 1
        self.out_globals.append({"name": sym, "size": len(data), "align": 1, "bytes": data.hex(), "relocs": [], "static": True})
        return sym

    # ------------------------------------------------------------------ functions
    def function(self, fn):
        f = FuncGen(self, fn)
        return f.run()

    # ------------------------------------------------------------------ data
    def global_var(self, v, sym, static):
        t = self.ty(v)
        if isinstance(t, Array) and t.n is None:
            # int x[] = {...}: size from initializer
            init = [c for c in kids(v) if c.get("kind") not in ("AlignedAttr", "UsedAttr", "SectionAttr", "VisibilityAttr")]
            if init:
                t = self.ty(init[0])
        size = self.tu.sizeof(t)
        align = self.tu.alignof(t)
        for c in kids(v):
            if c.get("kind") == "AlignedAttr":
                al = [x for x in kids(c) if "value" in x]
                if al:
                    align = max(align, int(al[0]["value"]))
                else:
                    align = max(align, 16)
        buf = bytearray(size)
        relocs = []
        init = [c for c in kids(v) if not c.get("kind", "").endswith("Attr")]
        if init:
            self.init_data(init[0], t, buf, 0, relocs)
        self.out_globals.append({"name": sym, "size": size, "align": align, "bytes": buf.hex() if init else None,
                                 "relocs": relocs, "static": static, "type": t.show()})

    def init_data(self, n, t, buf, off, relocs):
        n = strip(n)
        k = n.get("kind")
        if k == "ImplicitValueInitExpr":
            return
        if k == "InitListExpr":
            items = kids(n) or (n.get("array_filler") or [None])[1:]
            if isinstance(t, Array):
                es = self.tu.sizeof(t.elem)
                filler = n.get("array_filler")
                for i, it in enumerate(items):
                    self.init_data(it, t.elem, buf, off + i * es, relocs)
                if filler and t.n:
                    for fl in filler:
                        if fl.get("kind") != "ImplicitValueInitExpr":
                            for i in range(len(items), t.n):
                                self.init_data(fl, t.elem, buf, off + i * es, relocs)
                return
            if isinstance(t, Record):
                fields = self.tu.records[(t.tag, t.key)]
                if t.tag == "union":
                    fid = n.get("field", {}).get("id")
                    idx = [i for i, f in enumerate(fields) if f["id"] == fid] if fid else [0]
                    i = idx[0] if idx else 0
                    if items:
                        self.init_field(fields[i], items[0], buf, off, relocs)
                    return
                fi = 0
                named = [f for f in fields]
                for it in items:
                    if fi >= len(named):
                        break
                    self.init_field(named[fi], it, buf, off, relocs)
                    fi += 1
                return
            # scalar in braces
            if items:
                self.init_data(items[0], t, buf, off, relocs)
            return
        if k == "StringLiteral" and isinstance(t, Array):
            data = parse_c_string(self.tu.text(n))
            nmax = t.n if t.n is not None else len(data) + 1
            data = (data + b"\0" * nmax)[:nmax]
            buf[off:off + len(data)] = data
            return
        val = self.const(n)
        self.put_scalar(val, t, buf, off, relocs)

    def init_field(self, fnode, item, buf, off, relocs):
        foff, b0, bw, ft = self.tu.field(fnode["id"])
        if b0 is not None:
            v = self.const(strip(item))
            if v[0] != "int":
                raise Unsupported("bitfield init")
            bits = int.from_bytes(buf[off + foff:off + foff + 8].ljust(8, b"\0"), "little")
            mask = ((1 << bw) - 1) << b0
            bits = (bits & ~mask) | ((v[1] << b0) & mask)
            nb = (b0 + bw + 7) // 8
            buf[off + foff:off + foff + nb] = bits.to_bytes(8, "little")[:nb]
            return
        self.init_data(item, ft, buf, off + foff, relocs)

    def put_scalar(self, val, t, buf, off, relocs):
        size = self.tu.sizeof(t)
        if val[0] == "addr":
            relocs.append([off, val[1], val[2]])
            return
        if isinstance(t, Float):
            if val[0] == "int":
                v = Fraction(val[1])
            else:
                v = val[1]
            if t.size == 4:
                bits = val[2] if val[0] == "float" and val[2] is not None and val[3] == 4 else f32_bits_nearest(v)
                buf[off:off + 4] = bits.to_bytes(4, "little")
            else:
                buf[off:off + 8] = f64_bits(v).to_bytes(8, "little")
            return
        if val[0] == "float":
            v = int(val[1])  # truncation toward zero
        else:
            v = val[1]
        if isinstance(t, Int) and t.boolean:
            v = 1 if v else 0
        buf[off:off + size] = (v & ((1 << (8 * size)) - 1)).to_bytes(size, "little")

    def const(self, n):
        """Constant expression -> ('int', v) | ('float', Fraction, bits|None, size) | ('addr', [kind, name], addend)."""
        n = strip(n)
        k = n.get("kind")
        if k == "ConstantExpr" and "value" in n and isinstance(self.ty(n), Int):
            return ("int", int(n["value"]))
        if k == "ConstantExpr":
            return self.const(kids(n)[0])
        if k == "IntegerLiteral":
            return ("int", int(n["value"]))
        if k == "CharacterLiteral":
            return ("int", int(n["value"]))
        if k == "FloatingLiteral":
            v = float_literal(self.tu.text(n))
            t = self.ty(n)
            return ("float", v, f32_bits_nearest(v) if t.size == 4 else None, t.size)
        if k == "StringLiteral":
            return ("addr", ["sym", self.string(n)], 0)
        if k == "DeclRefExpr":
            r = n["referencedDecl"]
            if r["kind"] == "EnumConstantDecl":
                return ("int", self.tu.enum_consts[r["name"]])
            if r["kind"] == "FunctionDecl":
                return ("addr", ["fn", self.fsym(r["name"])], 0)
            if r["kind"] == "VarDecl":
                return ("addr", ["sym", self.static_sym(r)], 0)
            raise Unsupported("const ref %s" % r["kind"])
        if k in ("ImplicitCastExpr", "CStyleCastExpr"):
            ck = n.get("castKind")
            inner = kids(n)[0]
            t = self.ty(n)
            if ck in ("ArrayToPointerDecay", "FunctionToPointerDecay"):
                return self.addr_const(inner)
            v = self.const(inner)
            if ck in ("BitCast", "NoOp", "NullToPointer", "IntegralToPointer", "PointerToIntegral", "LValueToRValue"):
                if v[0] == "int" and isinstance(t, (Int, Ptr)):
                    return ("int", wrap(v[1], t))
                return v
            if ck in ("IntegralCast", "IntegralToBoolean"):
                if v[0] == "addr":
                    return v
                if isinstance(t, Int) and t.boolean:
                    return ("int", 1 if v[1] else 0)
                return ("int", wrap(v[1], t))
            if ck == "IntegralToFloating":
                return ("float", Fraction(v[1]), None, t.size)
            if ck == "FloatingCast":
                if t.size == 4:
                    b = f32_bits_nearest(v[1])
                    return ("float", Fraction(struct.unpack("<f", struct.pack("<I", b))[0]), b, 4)
                return ("float", v[1], None, 8)
            if ck == "FloatingToIntegral":
                return ("int", wrap(int(v[1]), t))
            raise Unsupported("const cast %s" % ck)
        if k == "UnaryOperator":
            op = n["opcode"]
            if op == "&":
                return self.addr_const(kids(n)[0])
            v = self.const(kids(n)[0])
            t = self.ty(n)
            if op == "-":
                if v[0] == "float":
                    return ("float", -v[1], (v[2] ^ 0x80000000) if v[2] is not None else None, v[3])
                return ("int", wrap(-v[1], t))
            if op == "+":
                return v
            if op == "~":
                return ("int", wrap(~v[1], t))
            if op == "!":
                return ("int", 0 if v[1] else 1)
            raise Unsupported("const unary %s" % op)
        if k == "BinaryOperator":
            op = n["opcode"]
            a, b = self.const(kids(n)[0]), self.const(kids(n)[1])
            t = self.ty(n)
            if a[0] == "addr" or b[0] == "addr":
                if op in ("+", "-") and a[0] == "addr" and b[0] == "int":
                    pt = self.ty(kids(n)[0])
                    es = self.tu.sizeof(pt.to) if isinstance(pt, Ptr) else 1
                    return ("addr", a[1], a[2] + (b[1] if op == "+" else -b[1]) * es)
                if op == "+" and b[0] == "addr" and a[0] == "int":
                    pt = self.ty(kids(n)[1])
                    return ("addr", b[1], b[2] + a[1] * self.tu.sizeof(pt.to))
                raise Unsupported("const addr arith")
            if a[0] == "float" or b[0] == "float":
                x, y = Fraction(a[1]), Fraction(b[1])
                r = {"+": lambda: x + y, "-": lambda: x - y, "*": lambda: x * y, "/": lambda: x / y}.get(op)
                if r is None:
                    cmp = {"<": x < y, ">": x > y, "<=": x <= y, ">=": x >= y, "==": x == y, "!=": x != y}[op]
                    return ("int", int(cmp))
                v = r()
                if t.size == 4:
                    bb = f32_bits_nearest(v)
                    return ("float", Fraction(struct.unpack("<f", struct.pack("<I", bb))[0]), bb, 4)
                return ("float", Fraction(float(v)), None, 8)
            x, y = a[1], b[1]
            ops = {"+": lambda: x + y, "-": lambda: x - y, "*": lambda: x * y, "&": lambda: x & y, "|": lambda: x | y,
                   "^": lambda: x ^ y, "<<": lambda: x << y, ">>": lambda: x >> y, "<": lambda: int(x < y), ">": lambda: int(x > y),
                   "<=": lambda: int(x <= y), ">=": lambda: int(x >= y), "==": lambda: int(x == y), "!=": lambda: int(x != y),
                   "&&": lambda: int(bool(x) and bool(y)), "||": lambda: int(bool(x) or bool(y)),
                   "/": lambda: int(x / y) if y else 0, "%": lambda: x - int(x / y) * y if y else 0}
            return ("int", wrap(ops[op](), t))
        if k == "ConditionalOperator":
            c = self.const(kids(n)[0])
            return self.const(kids(n)[1] if c[1] else kids(n)[2])
        if k == "UnaryExprOrTypeTraitExpr":
            return ("int", self.trait_value(n))
        if k == "CompoundLiteralExpr":
            raise Unsupported("compound literal in constant")
        raise Unsupported("const %s" % k)

    def trait_value(self, n):
        t = self.tu.ty(n["argType"]) if "argType" in n else self.ty(kids(n)[0])
        if n["name"] == "sizeof":
            return self.tu.sizeof(t)
        if n["name"] in ("alignof", "__alignof", "_Alignof", "__alignof__"):
            return self.tu.alignof(t)
        raise Unsupported(n["name"])

    def addr_const(self, n):
        n = strip(n)
        k = n.get("kind")
        if k == "DeclRefExpr":
            return self.const(n)
        if k == "StringLiteral":
            return ("addr", ["sym", self.string(n)], 0)
        if k == "MemberExpr":
            base = kids(n)[0]
            off = self.tu.field(n["referencedMemberDecl"])[0]
            if n.get("isArrow"):
                b = self.const(base)
            else:
                b = self.addr_const(base)
            return ("addr", b[1], b[2] + off)
        if k == "ArraySubscriptExpr":
            a, i = kids(n)
            ta = self.ty(a)
            if not isinstance(ta, Ptr):
                a, i = i, a
                ta = self.ty(a)
            b = self.const(a)
            idx = self.const(i)
            return ("addr", b[1], b[2] + idx[1] * self.tu.sizeof(ta.to))
        if k in ("ImplicitCastExpr", "CStyleCastExpr"):
            return self.addr_const(kids(n)[0]) if n.get("castKind") in ("NoOp", "BitCast", "LValueToRValue") else self.const(n)
        if k == "UnaryOperator" and n["opcode"] == "*":
            return self.const(kids(n)[0])
        if k == "CompoundLiteralExpr":
            # a file-scope compound literal: an anonymous global
            t = self.ty(n)
            sym = "%s__lit%d" % (self.tu.id, self.str_count)
            self.str_count += 1
            buf = bytearray(self.tu.sizeof(t))
            relocs = []
            self.init_data(kids(n)[0], t, buf, 0, relocs)
            self.out_globals.append({"name": sym, "size": len(buf), "align": self.tu.alignof(t), "bytes": buf.hex(), "relocs": relocs, "static": True})
            return ("addr", ["sym", sym], 0)
        raise Unsupported("addr const %s" % k)

    def static_sym(self, r):
        if r["id"] in self.statics:
            return self.statics[r["id"]]
        return self.vsym(r["name"])


def wrap(v, t):
    if isinstance(t, Ptr):
        size, signed = 4, False
    elif isinstance(t, Int):
        size, signed = t.size, t.signed
    else:
        return v
    v &= (1 << (8 * size)) - 1
    if signed and v >= 1 << (8 * size - 1):
        v -= 1 << (8 * size)
    return v

from funcgen import FuncGen
