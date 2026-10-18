"""Expressions."""
import struct
import re
from fractions import Fraction
from ctype import Int, Float, Ptr, Array, Func, Record, Void
from emit import rty, zero, load, store, cast, truth, is_bool
from trans_util import *
import trans

INT = Int(4, True)
UINT = Int(4, False)

def impure(code):
    return re.search(r"\bc\b", code) is not None

RS_RAW_TO = {"i8": "((%s) as i8 as i64 as u64)", "i16": "((%s) as i16 as i64 as u64)", "i32": "((%s) as i64 as u64)",
             "i64": "((%s) as u64)", "u8": "((%s) as u64)", "u16": "((%s) as u64)", "u32": "((%s) as u64)", "u64": "(%s)",
             "F32": "((%s).0 as u64)", "f32": "((%s).to_bits() as u64)", "f64": "(%s).to_bits()", "rec": "((%s) as u64)",
             "i128": "((%s) as u64)", "u128": "((%s) as u64)"}
RS_RAW_FROM = {"i8": "((%s) as i8)", "i16": "((%s) as i16)", "i32": "((%s) as i32)", "i64": "((%s) as i64)", "u8": "((%s) as u8)",
               "u16": "((%s) as u16)", "u32": "((%s) as u32)", "u64": "((%s) as u64)", "F32": "F32((%s) as u32)",
               "f32": "f32::from_bits((%s) as u32)", "f64": "f64::from_bits(%s)", "rec": "((%s) as u32)",
               "i128": "((%s) as i128)", "u128": "((%s) as u128)"}

def adapt_rs(code, a, b):
    if a == b or (a in ("rec", "u32") and b in ("rec", "u32")):
        return code
    if b == "()":
        return "{ let _ = %s; }" % code
    if a == "()":
        return "{ %s; %s }" % (code, {"F32": "F32(0)", "f32": "0.0f32", "f64": "0.0"}.get(b, "0"))
    return RS_RAW_FROM[b] % (RS_RAW_TO[a] % code)

def sig_rs(t, native):
    if isinstance(t, (Record,)):
        return "rec"
    if isinstance(t, Array):
        return "u32"
    return rty(t, native)

VARIADIC_HOST = {"printf", "sprintf", "fprintf", "snprintf", "Dbg_Printf", "scePrintf"}

class ExprMixin:
    # ------------------------------------------------------------------ lvalues
    def lv(self, n):
        n = strip(n)
        k = n.get("kind")
        t = self.ty(n)
        if k == "DeclRefExpr":
            r = n["referencedDecl"]
            if r["id"] in self.vars:
                v = self.vars[r["id"]]
                if v[0] == "local":
                    return trans.LV("local", v[2], v[1])
                return trans.LV("mem", v[2], self.fpaddr(v[1]))
            if r["kind"] == "VarDecl":
                return trans.LV("mem", t, "sym::%s" % self.g.static_sym(r))
            if r["kind"] == "FunctionDecl":
                return trans.LV("mem", t, "fid::%s" % self.g.fsym(r["name"]))
            raise Unsupported("lvalue ref %s" % r["kind"])
        if k == "MemberExpr":
            base = n["inner"][0]
            off, b0, bw, ft = self.tu.field(n["referencedMemberDecl"])
            if n.get("isArrow"):
                code, bt = self.rv(base)
            else:
                code = self.addr_of_obj(base)
            a = code if off == 0 else "(%s).wrapping_add(%d)" % (code, off)
            if b0 is not None:
                return trans.LV("bit", ft, a, b0, bw)
            return trans.LV("mem", t, a)
        if k == "ArraySubscriptExpr":
            a, i = n["inner"]
            ta = self.ty(a)
            if not isinstance(ta, Ptr):
                a, i = i, a
                ta = self.ty(a)
            pc, _ = self.rv(a)
            ic, it = self.rv(i)
            sz = self.elem_size(ta)
            return trans.LV("mem", t, self.ptr_add(pc, ic, it, sz))
        if k == "UnaryOperator" and n["opcode"] == "*":
            pc, pt = self.rv(n["inner"][0])
            return trans.LV("mem", t, pc)
        if k == "StringLiteral":
            return trans.LV("mem", t, "sym::%s" % self.g.string(n))
        if k == "CompoundLiteralExpr":
            off = self.alloc(t)
            a = self.fpaddr(off)
            init = "; ".join(self.init_mem(a, t, n["inner"][0]))
            return trans.LV("mem", t, "{ %s; %s }" % (init, a))
        if k in ("ImplicitCastExpr", "CStyleCastExpr") and n.get("castKind") in ("NoOp", "LValueToRValue", "BitCast"):
            inner = self.lv(n["inner"][0])
            return trans.LV(inner.kind, t if inner.kind != "bit" else inner.t, inner.a, inner.first, inner.width)
        if k in ("CallExpr", "StmtExpr", "ConditionalOperator", "BinaryOperator") and isinstance(t, (Record, Array)):
            code, _ = self.rv(n)
            return trans.LV("mem", t, code)
        if k == "ConstantExpr":
            return self.lv(n["inner"][0])
        if k == "PredefinedExpr":
            return self.lv(n["inner"][0])
        raise Unsupported("lvalue %s" % k)

    def addr_of_obj(self, n):
        """Address of a record/array object expression (lvalue or record rvalue)."""
        n = strip(n)
        t = self.ty(n)
        if n.get("valueCategory") == "prvalue" or n.get("kind") in ("CallExpr",):
            code, _ = self.rv(n)
            return code
        l = self.lv(n)
        if l.kind == "local":
            raise Unsupported("address of register local")
        return l.a

    def elem_size(self, pt):
        if isinstance(pt, Ptr):
            if isinstance(pt.to, (Void, Func)):
                return 1
            return self.tu.sizeof(pt.to)
        return 1

    def ptr_add(self, pc, ic, it, sz, neg=False):
        i = ic if rty(it) == "u32" else "((%s) as u32)" % ic
        if sz != 1:
            i = "(%s).wrapping_mul(%d)" % (i, sz)
        return "(%s).%s(%s)" % (pc, "wrapping_sub" if neg else "wrapping_add", i)

    def read(self, l):
        if l.kind == "local":
            return l.a
        if l.kind == "bit":
            return self.bf_get(l)
        if isinstance(l.t, (Record, Array, Func)):
            return l.a
        if impure(l.a):
            return "{ let a_ = %s; %s }" % (l.a, load(l.t, "a_", self.native))
        return load(l.t, l.a, self.native)

    def bf_get(self, l):
        r = self.rt(l.t)
        fn = "bf_get_s" if l.t.signed else "bf_get_u"
        if impure(l.a):
            return "{ let a_ = %s; (rt::%s(c, a_, %d, %d) as %s) }" % (l.a, fn, l.first, l.width, r)
        return "(rt::%s(c, %s, %d, %d) as %s)" % (fn, l.a, l.first, l.width, r)

    def write(self, l, v):
        """Statement storing rust value code v (already of l.t) into l."""
        if l.kind == "local":
            return "%s = %s" % (l.a, v)
        if l.kind == "bit":
            return "rt::bf_set(c, %s, %d, %d, (%s) as u64)" % (l.a, l.first, l.width, v)
        return store(l.t, l.a, v, self.native)

    # ------------------------------------------------------------------ rvalues
    def cond(self, n):
        code, t = self.rv(n)
        return truth(code, t, self.native)

    def expr_stmt(self, n):
        s = strip(n)
        k = s.get("kind")
        if k in ("ImplicitCastExpr", "CStyleCastExpr") and s.get("castKind") == "ToVoid":
            return self.expr_stmt(s["inner"][0])
        if k == "BinaryOperator" and s["opcode"] == "=":
            return self.assign(s, used=False)
        if k == "BinaryOperator" and s["opcode"] == ",":
            return "{ %s; %s; }" % (self.expr_stmt(s["inner"][0]), self.expr_stmt(s["inner"][1]))
        if k == "UnaryOperator" and s["opcode"] in ("++", "--"):
            return self.incdec(s, used=False)
        if k == "CompoundAssignOperator":
            return self.compound(s, used=False)
        if k == "CallExpr":
            code, t = self.rv(s)
            return code if isinstance(t, Void) else "{ let _ = %s; }" % code
        if k == "ConditionalOperator":
            ch = s["inner"]
            return "if %s { %s; } else { %s; }" % (self.cond(ch[0]), self.expr_stmt(ch[1]), self.expr_stmt(ch[2]))
        code, t = self.rv(s)
        return "{ let _ = %s; }" % code

    def fconst(self, n, depth=0):
        """Value of a float expression the C compiler folds at compile time (IEEE, round to nearest): literals,
        const-qualified variables with constant initializers and arithmetic on them. None if not constant."""
        if depth > 20 or not isinstance(n, dict):
            return None
        k = n.get("kind")
        t = self.ty(n) if "type" in n else None
        def rnd(v, ty):
            if v is None or ty is None:
                return None
            if isinstance(ty, Float) and ty.size == 4:
                try:
                    return struct.unpack("<f", struct.pack("<f", v))[0]
                except OverflowError:
                    return None
            return v
        if k in ("ParenExpr", "ConstantExpr") and n.get("inner"):
            return self.fconst(n["inner"][0], depth + 1) if isinstance(t, Float) else None
        if k == "FloatingLiteral":
            return rnd(float(trans.float_literal(self.tu.text(n))), t)
        if k in ("IntegerLiteral",):
            return float(int(n["value"]))
        if k in ("ImplicitCastExpr", "CStyleCastExpr"):
            ck = n.get("castKind"); inner = n["inner"][0]
            if ck in ("FloatingCast", "NoOp"):
                return rnd(self.fconst(inner, depth + 1), t)
            if ck == "IntegralToFloating":
                iv = inner
                while iv.get("kind") in ("ParenExpr", "ConstantExpr", "ImplicitCastExpr") and iv.get("inner") and iv.get("kind") != "IntegerLiteral":
                    iv = iv["inner"][0]
                if iv.get("kind") == "IntegerLiteral":
                    return rnd(float(int(iv["value"])), t)
                return None
            if ck == "LValueToRValue" and isinstance(t, Float):
                d = inner
                while d.get("kind") == "ParenExpr":
                    d = d["inner"][0]
                if d.get("kind") == "DeclRefExpr":
                    cv = self.tu.const_vars.get(d["referencedDecl"].get("id"))
                    if cv:
                        return rnd(self.fconst(cv[1], depth + 1), t)
                if d.get("kind") == "ArraySubscriptExpr":
                    # element of a const array with a constant index (atanhi[3] in the libm)
                    b, ix = d["inner"]
                    while b.get("kind") in ("ImplicitCastExpr", "ParenExpr") and b.get("inner"):
                        b = b["inner"][0]
                    while ix.get("kind") in ("ImplicitCastExpr", "ParenExpr", "ConstantExpr") and ix.get("inner") and ix.get("kind") != "IntegerLiteral":
                        if ix.get("kind") == "ConstantExpr" and "value" in ix:
                            break
                        ix = ix["inner"][0]
                    iv = ix.get("value")
                    if b.get("kind") == "DeclRefExpr" and iv is not None:
                        cv = self.tu.const_vars.get(b["referencedDecl"].get("id"))
                        if cv and strip(cv[1]).get("kind") == "InitListExpr":
                            il = strip(cv[1])
                            items = il.get("inner", []) or (il.get("array_filler") or [None])[1:]
                            i = int(iv)
                            if 0 <= i < len(items):
                                return rnd(self.fconst(items[i], depth + 1), t)
            return None
        if k == "UnaryOperator" and n.get("opcode") in ("-", "+") and isinstance(t, Float):
            v = self.fconst(n["inner"][0], depth + 1)
            return None if v is None else (-v if n["opcode"] == "-" else v)
        if k == "BinaryOperator" and n.get("opcode") in ("+", "-", "*", "/") and isinstance(t, Float):
            a = self.fconst(n["inner"][0], depth + 1)
            b = self.fconst(n["inner"][1], depth + 1)
            if a is None or b is None:
                return None
            op = n["opcode"]
            if op == "/" and b == 0:
                return None
            v = a + b if op == "+" else a - b if op == "-" else a * b if op == "*" else a / b
            return rnd(v, t)
        return None

    def rv(self, n):
        k = n.get("kind")
        t = self.ty(n) if "type" in n else Void()
        if isinstance(t, Float) and k in ("BinaryOperator", "UnaryOperator", "ImplicitCastExpr", "CStyleCastExpr"):
            v = self.fconst(n)
            if v is not None:
                if t.size == 8:
                    return "f64::from_bits(0x%016x)" % trans.f64_bits(v), t
                b = trans.f32_bits_nearest(v)
                return ("f32::from_bits(0x%08x)" if self.native else "F32(0x%08x)") % b, t
        if k == "ParenExpr":
            return self.rv(n["inner"][0])
        if k == "ConstantExpr":
            if "value" in n and isinstance(t, Int):
                return self.lit(trans.wrap(int(n["value"]), t), t), t
            return self.rv(n["inner"][0])
        if k in ("IntegerLiteral", "CharacterLiteral"):
            return self.lit(trans.wrap(int(n["value"]), t), t), t
        if k == "FloatingLiteral":
            v = trans.float_literal(self.tu.text(n))
            if t.size == 8:
                return "f64::from_bits(0x%016x)" % trans.f64_bits(v), t
            b = trans.f32_bits_nearest(v)
            return ("f32::from_bits(0x%08x)" if self.native else "F32(0x%08x)") % b, t
        if k == "UnaryExprOrTypeTraitExpr":
            return self.lit(self.g.trait_value(n), t), t
        if k == "DeclRefExpr":
            r = n["referencedDecl"]
            if r["kind"] == "EnumConstantDecl":
                return self.lit(trans.wrap(self.tu.enum_consts[r["name"]], t), t), t
            if r["kind"] == "FunctionDecl":
                return "fid::%s" % self.g.fsym(r["name"]), t
            return self.read(self.lv(n)), t
        if k in ("ImplicitCastExpr", "CStyleCastExpr"):
            return self.cast_expr(n, t)
        if k in ("MemberExpr", "ArraySubscriptExpr", "StringLiteral", "CompoundLiteralExpr"):
            return self.read(self.lv(n)), t
        if k == "UnaryOperator":
            return self.unary(n, t)
        if k == "BinaryOperator":
            return self.binary(n, t)
        if k == "CompoundAssignOperator":
            return self.compound(n, used=True), t
        if k == "ConditionalOperator":
            ch = n["inner"]
            a, ta = self.rv(ch[1])
            b, tb = self.rv(ch[2])
            if isinstance(t, Void):
                return "if %s { %s; } else { %s; }" % (self.cond(ch[0]), a, b), t
            return "(if %s { %s } else { %s })" % (self.cond(ch[0]), cast(a, ta, t, self.native), cast(b, tb, t, self.native)), t
        if k == "BinaryConditionalOperator":
            ch = n["inner"]  # common, cond(opaque), true(opaque), false
            v, tv = self.rv(ch[0])
            b, tb = self.rv(ch[-1])
            return "{ let v_ = %s; if %s { %s } else { %s } }" % (v, truth("v_", tv, self.native), cast("v_", tv, t, self.native), cast(b, tb, t, self.native)), t
        if k == "CallExpr":
            return self.call(n, t)
        if k == "StmtExpr":
            body = n["inner"][0]
            out = []
            items = body.get("inner", [])
            for s in items[:-1]:
                self.stmt(s, out, 0)
            last = items[-1] if items else None
            if last is not None and not isinstance(t, Void):
                code, lt = self.rv(last)
                return "{ %s %s }" % (" ".join(out), cast(code, lt, t, self.native)), t
            if last is not None:
                self.stmt(last, out, 0)
            return "{ %s }" % " ".join(out), t
        if k == "ImplicitValueInitExpr":
            return zero(t, self.native), t
        if k == "InitListExpr":
            # scalar initialised with braces
            ch = n.get("inner", [])
            if ch and not isinstance(t, (Record, Array)):
                code, ct = self.rv(ch[0])
                return cast(code, ct, t, self.native), t
            raise Unsupported("init list rvalue")
        if k == "PredefinedExpr":
            return self.rv(n["inner"][0])
        if k == "OffsetOfExpr":
            raise Unsupported("offsetof")
        raise Unsupported("expr %s" % k)

    def cast_expr(self, n, t):
        ck = n.get("castKind")
        inner = n["inner"][0]
        if ck == "LValueToRValue":
            return self.read(self.lv(inner)), t
        if ck == "ArrayToPointerDecay":
            s = strip(inner)
            if s.get("kind") == "StringLiteral":
                return "sym::%s" % self.g.string(s), t
            return self.addr_of_obj(inner), t
        if ck == "FunctionToPointerDecay":
            s = strip(inner)
            if s.get("kind") == "DeclRefExpr":
                return "fid::%s" % self.g.fsym(s["referencedDecl"]["name"]), t
            return self.rv(inner)
        if ck == "ToVoid":
            return self.expr_stmt(inner), Void()
        code, it = self.rv(inner)
        if ck in ("NoOp",) and isinstance(t, (Record, Array)):
            return code, t
        return cast(code, it, t, self.native), t

    # ------------------------------------------------------------------ operators
    def unary(self, n, t):
        op = n["opcode"]
        a = n["inner"][0]
        if op == "&":
            s = strip(a)
            if s.get("kind") == "DeclRefExpr" and s["referencedDecl"]["kind"] == "FunctionDecl":
                return "fid::%s" % self.g.fsym(s["referencedDecl"]["name"]), t
            l = self.lv(a)
            if l.kind != "mem":
                raise Unsupported("& of %s" % l.kind)
            return l.a, t
        if op == "*":
            return self.read(self.lv(n)), t
        if op in ("++", "--"):
            return self.incdec(n, used=True), t
        if op == "__extension__":
            return self.rv(a)
        code, at = self.rv(a)
        if op == "+":
            return code, t
        if op == "-":
            if isinstance(t, Float):
                if t.size == 8 or self.native:
                    return "(-(%s))" % code, t
                return "rt::fneg(%s)" % code, t
            return "(%s).wrapping_neg()" % code, t
        if op == "~":
            return "(!(%s))" % code, t
        if op == "!":
            return "((!%s) as i32)" % truth(code, at, self.native), t
        raise Unsupported("unary %s" % op)

    def arith(self, op, a, b, t, ta=None, tb=None):
        """Binary arithmetic of two values of the same (promoted) type t."""
        r = self.rt(t)
        if isinstance(t, Float):
            if t.size == 8 or self.native:
                return "((%s) %s (%s))" % (a, op, b)
            fn = {"+": "fadd", "-": "fsub", "*": "fmul", "/": "fdiv"}[op]
            return "rt::%s(%s, %s)" % (fn, a, b)
        if op == "+":
            return "(%s).wrapping_add(%s)" % (a, b)
        if op == "-":
            return "(%s).wrapping_sub(%s)" % (a, b)
        if op == "*":
            return "(%s).wrapping_mul(%s)" % (a, b)
        if op == "/":
            return "rt::div_%s(%s, %s)" % (r, a, b)
        if op == "%":
            return "rt::rem_%s(%s, %s)" % (r, a, b)
        if op in ("&", "|", "^"):
            return "((%s) %s (%s))" % (a, op, b)
        if op == "<<":
            return "(%s).wrapping_shl((%s) as u32)" % (a, b)
        if op == ">>":
            return "(%s).wrapping_shr((%s) as u32)" % (a, b)
        raise Unsupported("arith %s" % op)

    def compare(self, op, a, b, t):
        if isinstance(t, Float) and t.size == 4 and not self.native:
            fn = {"<": "flt", ">": "fgt", "<=": "fle", ">=": "fge", "==": "feq", "!=": "fne"}[op]
            return "(rt::%s(%s, %s) as i32)" % (fn, a, b)
        return "(((%s) %s (%s)) as i32)" % (a, op, b)

    def binary(self, n, t):
        op = n["opcode"]
        if op == "=":
            return self.assign(n, used=True), t
        if op == ",":
            code, tb = self.rv(n["inner"][1])
            return "{ %s; %s }" % (self.expr_stmt(n["inner"][0]), code), tb
        if op in ("&&", "||"):
            return "((%s %s %s) as i32)" % (self.cond(n["inner"][0]), op, self.cond(n["inner"][1])), t
        an, bn = n["inner"]
        a, ta = self.rv(an)
        b, tb = self.rv(bn)
        if impure(a) and impure(b):
            pass  # Rust evaluates left to right like clang
        if op in ("<", ">", "<=", ">=", "==", "!="):
            return self.compare(op, a, b, ta), t
        if isinstance(ta, Ptr) and isinstance(tb, Ptr) and op == "-":
            sz = self.elem_size(ta)
            d = "((%s).wrapping_sub(%s) as i32)" % (a, b)
            return ("(%s).wrapping_div(%d)" % (d, sz) if sz != 1 else d), t
        if isinstance(ta, Ptr) and op in ("+", "-"):
            return self.ptr_add(a, b, tb, self.elem_size(ta), op == "-"), t
        if isinstance(tb, Ptr) and op == "+":
            return self.ptr_add(b, a, ta, self.elem_size(tb)), t
        if op in ("<<", ">>"):
            return self.arith(op, a, b, ta), t
        return self.arith(op, a, b, t), t

    def assign(self, n, used):
        ln, rn = n["inner"]
        t = self.ty(ln)
        rcode, rt_ = self.rv(rn)
        l = self.lv(ln)
        if isinstance(t, (Record, Array)):
            size = self.tu.sizeof(t)
            if used:
                return "{ let s_ = %s; let d_ = %s; rt::memcpy(c, d_, s_, %d); d_ }" % (rcode, l.a, size)
            return "{ let s_ = %s; let d_ = %s; rt::memcpy(c, d_, s_, %d); }" % (rcode, l.a, size)
        v = cast(rcode, rt_, t, self.native)
        if l.kind == "local":
            if used:
                return "{ %s = %s; %s }" % (l.a, v, l.a)
            return "%s = %s" % (l.a, v)
        if l.kind == "bit":
            s = "{ let v_ = %s; let a_ = %s; rt::bf_set(c, a_, %d, %d, (v_) as u64); %s }" % (
                v, l.a, l.first, l.width, self.bf_get(trans.LV("bit", l.t, "a_", l.first, l.width)) if used else "")
            return s
        st = store(t, "a_", "v_", self.native)
        if used:
            return "{ let v_ = %s; let a_ = %s; %s; v_ }" % (v, l.a, st)
        return "{ let v_ = %s; let a_ = %s; %s; }" % (v, l.a, st)

    def compound(self, n, used):
        op = n["opcode"][:-1]
        ln, rn = n["inner"]
        t = self.ty(ln)
        lt = self.tu.ty(n["computeLHSType"])
        resT = self.tu.ty(n["computeResultType"])
        rcode, rt_ = self.rv(rn)
        l = self.lv(ln)
        bind = ""
        if l.kind == "mem":
            bind = "let a_ = %s; " % l.a
            l = trans.LV("mem", l.t, "a_")
        elif l.kind == "bit":
            bind = "let a_ = %s; " % l.a
            l = trans.LV("bit", l.t, "a_", l.first, l.width)
        cur = self.read(l)
        if isinstance(t, Ptr) and op in ("+", "-"):
            newv = self.ptr_add(cur, "r_", rt_, self.elem_size(t), op == "-")
        else:
            x = cast(cur, t, lt, self.native)
            if op in ("<<", ">>"):
                res = self.arith(op, x, "r_", lt)
            else:
                res = self.arith(op, x, "r_", resT)
            newv = cast(res, resT, t, self.native)
        body = "{ let r_ = %s; %slet n_ = %s; %s; %s}" % (rcode, bind, newv, self.write(l, "n_"), "n_ " if used else "")
        return body

    def incdec(self, n, used):
        op = n["opcode"]
        post = n.get("isPostfix", False)
        a = n["inner"][0]
        t = self.ty(a)
        l = self.lv(a)
        bind = ""
        if l.kind in ("mem", "bit"):
            bind = "let a_ = %s; " % l.a
            l = trans.LV(l.kind, l.t, "a_", l.first, l.width)
        cur = self.read(l)
        if isinstance(t, Ptr):
            sz = self.elem_size(t)
            newv = "(o_).%s(%d)" % ("wrapping_add" if op == "++" else "wrapping_sub", sz)
        elif isinstance(t, Float):
            one = "F32(0x3f800000)" if t.size == 4 and not self.native else ("1.0f32" if t.size == 4 else "1.0f64")
            newv = self.arith("+" if op == "++" else "-", "o_", one, t)
        else:
            if isinstance(t, Int) and t.boolean:
                newv = "1u8" if op == "++" else "((o_ != 0) as u8 ^ 1)"
            else:
                newv = "(o_).%s(1)" % ("wrapping_add" if op == "++" else "wrapping_sub")
        body = "{ %slet o_ = %s; let n_ = %s; %s; %s}" % (bind, cur, newv, self.write(l, "n_"), ("o_ " if post else "n_ ") if used else "")
        return body

    # ------------------------------------------------------------------ calls
    def call(self, n, t):
        ch = n["inner"]
        callee = strip(ch[0])
        args = ch[1:]
        direct = None
        if callee.get("kind") == "ImplicitCastExpr" and callee.get("castKind") in ("FunctionToPointerDecay", "BuiltinFnToFnPtr"):
            d = strip(callee["inner"][0])
            if d.get("kind") == "DeclRefExpr" and d["referencedDecl"]["kind"] == "FunctionDecl":
                direct = d
        vals = [self.rv(a) for a in args]
        pre = []
        codes = []
        for i, (code, at) in enumerate(vals):
            if impure(code):
                v = "a%d_" % i
                pre.append("let %s = %s;" % (v, code))
                codes.append((v, at))
            else:
                codes.append((code, at))
        if direct is not None:
            name = direct["referencedDecl"]["name"]
            if name.startswith("__builtin_"):
                return self.builtin(name, codes, pre, t)
            sym = self.g.fsym(name)
            caller_ft = self.tu.ty(direct["type"])
            info = self.g.index["functions"].get(sym)
            ret_rs = sig_rs(t, self.native) if not isinstance(t, Void) else "()"
            argl = []
            sret_tmp = None
            if info is None:
                # host function: adapted to its Rust signature (src/host)
                hs = self.g.index.get("host", {}).get(sym)
                self.g.calls_host.add((sym, tuple(sig_rs(at, self.native) for _, at in codes), ret_rs,
                                       bool(caller_ft.variadic) if isinstance(caller_ft, Func) else False))
                if hs is None:
                    hs = {"params": [sig_rs(at, self.native) for _, at in codes], "ret": ret_rs, "variadic": False}
                    if isinstance(caller_ft, Func) and caller_ft.variadic:
                        hs = {"params": [sig_rs(p, self.native) for p in caller_ft.params], "ret": ret_rs, "variadic": True}
                hp = hs["params"]
                for i, p in enumerate(hp):
                    if i < len(codes):
                        argl.append(adapt_rs(codes[i][0], sig_rs(codes[i][1], self.native), p))
                    else:
                        argl.append({"F32": "F32(0)", "f32": "0.0f32", "f64": "0.0"}.get(p, "0"))
                if hs["variadic"]:
                    rest = [to_raw_any(c_, at, self.native) for c_, at in codes[len(hp):]]
                    argl.append("&[%s]" % ", ".join(rest))
                else:
                    for c_, at in codes[len(hp):]:
                        pre.append("let _ = %s;" % c_)
                call = "F::%s(c%s)" % (rust_ident(sym), "".join(", " + x for x in argl))
                call = adapt_rs(call, hs["ret"], ret_rs)
                if pre:
                    return "{ %s %s }" % (" ".join(pre), call), t
                return call, t
            params = info["params"]
            for i, p in enumerate(params):
                if i < len(codes):
                    c_, at = codes[i]
                    argl.append(adapt_rs(c_, sig_rs(at, self.native), p))
                else:
                    argl.append({"F32": "F32(0)", "f32": "0.0f32", "f64": "0.0"}.get(p, "0"))
            for c_, at in codes[len(params):]:
                pre.append("let _ = %s;" % c_)
            if info["ret"] == "rec":
                size = self.tu.sizeof(t) if isinstance(t, Record) else info.get("ret_size", 16)
                sret_tmp = self.fpaddr(self.alloc(t if isinstance(t, Record) else Array(Int(1, False), size)))
                argl.insert(0, sret_tmp)
                call = "F::%s(c%s)" % (rust_ident(sym), "".join(", " + x for x in argl))
            else:
                call = "F::%s(c%s)" % (rust_ident(sym), "".join(", " + x for x in argl))
                call = adapt_rs(call, info["ret"], ret_rs)
            if pre:
                return "{ %s %s }" % (" ".join(pre), call), t
            return call, t
        # indirect call
        fcode, ft = self.rv(ch[0])
        if impure(fcode):
            pre.insert(0, "let fp_ = %s;" % fcode)
            # keep left-to-right: callee first
            fcode = "fp_"
        slots = []
        sret_tmp = None
        if isinstance(t, Record):
            sret_tmp = self.fpaddr(self.alloc(t))
            slots.append("(%s) as u64" % sret_tmp)
        slots += [to_raw_any(c, at, self.native) for c, at in codes]
        call = "D::call(c, %s, &[%s])" % (fcode, ", ".join(slots))
        if isinstance(t, Record):
            call = "{ let _ = %s; %s }" % (call, sret_tmp)
        elif isinstance(t, Void):
            call = "{ let _ = %s; }" % call
        else:
            call = from_raw_any(call, t, self.native)
        if pre:
            return "{ %s %s }" % (" ".join(pre), call), t
        return call, t

    def builtin(self, name, codes, pre, t):
        a = [c for c, _ in codes]
        if name == "__builtin_fabsf":
            body = "rt::ffabs(%s)" % a[0] if not self.native else "(%s).abs()" % a[0]
        elif name == "__builtin_fabs":
            body = "(%s).abs()" % a[0]
        elif name == "__builtin_abs":
            body = "(%s).wrapping_abs()" % a[0]
        elif name == "__builtin_memcpy":
            body = "{ rt::memcpy(c, %s, %s, (%s) as u32); %s }" % (a[0], a[1], a[2], a[0])
        elif name == "__builtin_memset":
            body = "{ rt::memset(c, %s, (%s) as u8, (%s) as u32); %s }" % (a[0], a[1], a[2], a[0])
        elif name == "__builtin_expect":
            body = a[0]
        else:
            raise Unsupported(name)
        if pre:
            return "{ %s %s }" % (" ".join(pre), body), t
        return body, t

def to_raw_any(code, t, native):
    from emit import to_raw
    return to_raw(code, t, native)

def from_raw_any(code, t, native):
    from emit import from_raw
    return from_raw(code, t, native)
