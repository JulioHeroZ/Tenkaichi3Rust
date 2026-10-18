"""One C function -> one Rust function."""
import re
from ctype import Int, Float, Ptr, Array, Func, Record, Void
from emit import rty, zero, load, store, cast, truth, adapt, to_raw, from_raw, is_bool
from trans_util import *

INT = Int(4, True)

def impure(code):
    return re.search(r"\bc\b", code) is not None

class FuncGen:
    def __init__(self, g, fn):
        self.g, self.tu, self.fn = g, g.tu, fn
        self.native = g.native
        self.ftype = g.ty(fn)
        self.name = g.fsym(fn["name"])
        self.vars = {}
        self.frame = 0
        self.ntmp = 0
        self.nlabel = 0
        self.locals = []      # (rustname, rusttype) declared at top
        self.loops = []       # stack of (break_target, continue_target) as Rust labels or flatten blocks
        self.flat = None
        self.uses_frame = False

    # ------------------------------------------------------------------ helpers
    def tmp(self, prefix="t"):
        self.ntmp += 1
        return "%s%d" % (prefix, self.ntmp)

    def label(self, prefix="l"):
        self.nlabel += 1
        return "'%s%d" % (prefix, self.nlabel)

    def alloc(self, t):
        size = self.tu.sizeof(t)
        al = max(self.tu.alignof(t), 4)
        self.frame = (self.frame + al - 1) // al * al
        off = self.frame
        self.frame += max(size, 1)
        self.uses_frame = True
        return off

    def fpaddr(self, off):
        return "fp.wrapping_add(%d)" % off if off else "fp"

    def ty(self, n):
        return self.tu.ty(n["type"])

    def rt(self, t):
        return rty(t, self.native)

    def lit(self, v, t):
        r = self.rt(t)
        if r in ("F32", "f32", "f64"):
            raise ValueError("float literal via lit")
        if r == "u8" and isinstance(t, Int) and t.boolean:
            return "%du8" % (1 if v else 0)
        return "%d%s" % (v, r) if v >= 0 else "(%d%s)" % (v, r)

    # ------------------------------------------------------------------ entry
    def run(self):
        fn = self.fn
        params = [c for c in fn.get("inner", []) if c.get("kind") == "ParmVarDecl"]
        body = [c for c in fn.get("inner", []) if c.get("kind") == "CompoundStmt"][0]
        ret = self.ftype.ret
        self.ret_t = ret
        self.sret = isinstance(ret, Record)
        addr_taken = set()
        decls = []
        has_goto = [False]
        def scan(n):
            k = n.get("kind")
            if k == "UnaryOperator" and n.get("opcode") == "&":
                s = strip(n["inner"][0])
                if s.get("kind") == "DeclRefExpr":
                    addr_taken.add(s["referencedDecl"]["id"])
            if k == "VarDecl":
                decls.append(n)
            if k in ("GotoStmt", "LabelStmt", "IndirectGotoStmt"):
                has_goto[0] = True
            for c in n.get("inner", []):
                scan(c)
        scan(body)
        sig = []
        prologue = []
        for i, p in enumerate(params):
            pt = self.tu.ty(p["type"])
            if isinstance(pt, Array):
                pt = Ptr(pt.elem)
            name = "p_%s" % (p.get("name") or "arg%d" % i) if p.get("name") else "p_arg%d" % i
            name = self.uniq(name)
            if isinstance(pt, Record):
                sig.append("%s: u32" % name)
                off = self.alloc(pt)
                self.vars[p["id"]] = ("frame", off, pt)
                prologue.append("rt::memcpy(c, %s, %s, %d);" % (self.fpaddr(off), name, self.tu.sizeof(pt)))
            elif p["id"] in addr_taken:
                sig.append("%s: %s" % (name, self.rt(pt)))
                off = self.alloc(pt)
                self.vars[p["id"]] = ("frame", off, pt)
                prologue.append(store(pt, self.fpaddr(off), name, self.native) + ";")
            else:
                sig.append("mut %s: %s" % (name, self.rt(pt)))
                self.vars[p["id"]] = ("local", name, pt)
        for d in decls:
            if d.get("storageClass") == "static":
                sym = "%s__%s__%s" % (self.tu.id, self.fn["name"], d["name"])
                used = getattr(self.g, "_static_names", set())
                self.g._static_names = used
                k = 2
                base = sym
                while sym in used:
                    sym = "%s_%d" % (base, k)
                    k += 1
                used.add(sym)
                if d["id"] not in self.g.statics:
                    self.g.statics[d["id"]] = sym
                    self.g.global_var(d, sym, True)
                continue
            if d.get("storageClass") == "extern":
                continue
            t = self.tu.ty(d["type"])
            if isinstance(t, Array) and t.n is None:
                init = [x for x in d.get("inner", []) if not x.get("kind", "").endswith("Attr")]
                if init:
                    t = self.ty(init[0])
            if isinstance(t, (Array, Record)) or d["id"] in addr_taken:
                self.vars[d["id"]] = ("frame", self.alloc(t), t)
            else:
                name = self.uniq("l_%s" % d["name"])
                self.vars[d["id"]] = ("local", name, t)
                self.locals.append((name, self.rt(t), zero(t, self.native)))
        lines = []
        if has_goto[0] or self.needs_flatten(body):
            self.flatten(body, lines)
        else:
            self.block_items(body, lines, 1)
        # assemble
        rett = self.rt(ret) if not self.sret else "u32"
        args = (["sret: u32"] if self.sret else []) + sig
        out = []
        out.append("pub fn %s(c: &mut Ctx%s)%s {" % (rust_ident(self.name), "".join(", " + a for a in args),
                                                       "" if isinstance(ret, Void) else " -> " + rett))
        pargs = []
        for a in args:
            an, at = a.replace("mut ", "").split(": ")
            pargs.append("(%s).0" % an if at == "F32" else "(%s).to_bits() as u32" % an if at in ("f32", "f64") else "(%s) as u32" % an)
        probe_args = "&[%s]" % ", ".join(pargs)
        out.append("    static PROBE_N: rt::ProbeCount = rt::ProbeCount::new();")
        out.append("    if rt::probing() { crate::host::harness::probe(c, \"%s\", &PROBE_N, %s, false); }" % (self.name, probe_args))
        probe_post = "    if rt::probing() { crate::host::harness::probe(c, \"%s\", &PROBE_N, &[], true); }" % self.name
        if self.uses_frame:
            out.append("    let sp_ = c.sp;")
            out.append("    let fp: u32 = c.sp.wrapping_sub(%d) & !15;" % ((self.frame + 15) // 16 * 16))
            out.append("    c.sp = fp;")
            out.append("    c.frame_check(fp);")
            # locals without an initializer read as 0 (the reference build uses -ftrivial-auto-var-init=zero)
            out.append("    rt::memset(c, fp, 0, %d);" % ((self.frame + 15) // 16 * 16))
        for n, t, z in self.locals:
            out.append("    let mut %s: %s = %s;" % (n, t, z))
        for l in prologue:
            out.append("    " + l)
        endv = "" if isinstance(ret, Void) else (" sret" if self.sret else " " + zero(ret, self.native))
        if self.uses_frame:
            out.append("    let r_ = 'f: {")
            out.extend("    " + l for l in lines)
            out.append("       %s" % endv.strip())
            out.append("    };")
            out.append("    c.sp = sp_;")
            out.append(probe_post)
            out.append("    r_")
        else:
            out.append("    let r_ = 'f: {")
            out.extend(lines)
            out.append("       %s" % endv.strip())
            out.append("    };")
            out.append(probe_post)
            out.append("    r_")
        out.append("}")
        return "\n".join(out)

    def uniq(self, name):
        base = re.sub(r"\W", "_", name)
        self.ntmp += 1
        return "%s_%d" % (base, self.ntmp)

    # ------------------------------------------------------------------ statements (structured)
    def block_items(self, n, out, ind):
        for c in n.get("inner", []):
            self.stmt(c, out, ind)

    def emit(self, out, ind, s):
        out.append("    " * ind + s)

    def stmt(self, n, out, ind):
        k = n.get("kind")
        if k == "CompoundStmt":
            self.emit(out, ind, "{")
            self.block_items(n, out, ind + 1)
            self.emit(out, ind, "}")
        elif k == "DeclStmt":
            for d in n.get("inner", []):
                if d.get("kind") == "VarDecl":
                    self.decl_init(d, out, ind)
        elif k == "NullStmt":
            pass
        elif k == "ReturnStmt":
            self.emit(out, ind, self.ret_code(n) + ";")
        elif k == "IfStmt":
            ch = n["inner"]
            cond = self.cond(ch[0])
            self.emit(out, ind, "if %s {" % cond)
            self.sub(ch[1], out, ind + 1)
            if len(ch) > 2:
                self.emit(out, ind, "} else {")
                self.sub(ch[2], out, ind + 1)
            self.emit(out, ind, "}")
        elif k == "WhileStmt":
            ch = n["inner"]
            L = self.label()
            self.emit(out, ind, "%s: while %s {" % (L, self.cond(ch[0])))
            self.loops.append(("break %s" % L, "continue %s" % L))
            self.sub(ch[1], out, ind + 1)
            self.loops.pop()
            self.emit(out, ind, "}")
        elif k == "DoStmt":
            body, cond = n["inner"]
            L, C = self.label(), self.label("c")
            self.emit(out, ind, "%s: loop {" % L)
            self.emit(out, ind + 1, "%s: {" % C)
            self.loops.append(("break %s" % L, "break %s" % C))
            self.sub(body, out, ind + 2)
            self.loops.pop()
            self.emit(out, ind + 1, "}")
            self.emit(out, ind + 1, "if !%s { break %s; }" % (self.cond(cond), L))
            self.emit(out, ind, "}")
        elif k == "ForStmt":
            ch = n["inner"]  # init, condvar, cond, inc, body (missing parts are {})
            init, _cv, cond, inc, body = ch
            self.emit(out, ind, "{")
            if init:
                self.stmt(init, out, ind + 1)
            L, C = self.label(), self.label("c")
            self.emit(out, ind + 1, "%s: loop {" % L)
            if cond:
                self.emit(out, ind + 2, "if !%s { break %s; }" % (self.cond(cond), L))
            self.emit(out, ind + 2, "%s: {" % C)
            self.loops.append(("break %s" % L, "break %s" % C))
            self.sub(body, out, ind + 3)
            self.loops.pop()
            self.emit(out, ind + 2, "}")
            if inc:
                self.emit(out, ind + 2, self.expr_stmt(inc) + ";")
            self.emit(out, ind + 1, "}")
            self.emit(out, ind, "}")
        elif k == "SwitchStmt":
            self.switch(n, out, ind)
        elif k == "BreakStmt":
            self.emit(out, ind, self.loops[-1][0] + ";")
        elif k == "ContinueStmt":
            cont = [l for l in reversed(self.loops) if l[1] is not None][0]
            self.emit(out, ind, cont[1] + ";")
        elif k == "GCCAsmStmt":
            self.emit(out, ind, "/* asm: %s */" % self.tu.text(n).replace("*/", "* /")[:200])
        elif k in ("LabelStmt", "GotoStmt", "CaseStmt", "DefaultStmt"):
            raise Unsupported("%s in structured code" % k)
        else:
            self.emit(out, ind, self.expr_stmt(n) + ";")

    def sub(self, n, out, ind):
        if n.get("kind") == "CompoundStmt":
            self.block_items(n, out, ind)
        else:
            self.stmt(n, out, ind)

    def decl_init(self, d, out, ind):
        if d.get("storageClass") in ("static", "extern"):
            return
        init = [x for x in d.get("inner", []) if not x.get("kind", "").endswith("Attr")]
        v = self.vars[d["id"]]
        if not init:
            return
        e = init[0]
        if v[0] == "local":
            code, t = self.rv(e)
            self.emit(out, ind, "%s = %s;" % (v[1], cast(code, t, v[2], self.native)))
        else:
            for l in self.init_mem(self.fpaddr(v[1]), v[2], e):
                self.emit(out, ind, l)

    def init_mem(self, a, t, e):
        """Statements that initialise the object at address a (constant expression a) from initializer e."""
        e = strip(e)
        k = e.get("kind")
        lines = []
        if k == "InitListExpr":
            lines.append("rt::memset(c, %s, 0, %d);" % (a, self.tu.sizeof(t)))
            items = e.get("inner", []) or (e.get("array_filler") or [None])[1:]
            if isinstance(t, Array):
                es = self.tu.sizeof(t.elem)
                for i, it in enumerate(items):
                    if strip(it).get("kind") == "ImplicitValueInitExpr":
                        continue
                    lines += self.init_mem_nz("%s.wrapping_add(%d)" % (a, i * es), t.elem, it)
                filler = e.get("array_filler")
                if filler and t.n and strip(filler[0]).get("kind") != "ImplicitValueInitExpr":
                    for i in range(len(items), t.n):
                        lines += self.init_mem_nz("%s.wrapping_add(%d)" % (a, i * es), t.elem, filler[0])
                return lines
            if isinstance(t, Record):
                fields = self.tu.records[(t.tag, t.key)]
                if t.tag == "union":
                    fid = e.get("field", {}).get("id")
                    idx = [i for i, f in enumerate(fields) if f["id"] == fid] if fid else [0]
                    pairs = [(fields[idx[0] if idx else 0], items[0])] if items else []
                else:
                    pairs = list(zip(fields, items))
                for f, it in pairs:
                    if strip(it).get("kind") == "ImplicitValueInitExpr":
                        continue
                    off, b0, bw, ft = self.tu.field(f["id"])
                    if b0 is not None:
                        code, vt = self.rv(it)
                        lines.append("rt::bf_set(c, %s.wrapping_add(%d), %d, %d, (%s) as u64);" % (a, off, b0, bw, self.hoist_val(code)))
                    else:
                        lines += self.init_mem_nz("%s.wrapping_add(%d)" % (a, off), ft, it)
                return lines
            if items:
                return lines + self.init_mem_nz(a, t, items[0])
            return lines
        if k == "StringLiteral" and isinstance(t, Array):
            sym = self.g.string(e)
            data_len = len(parse_c_string(self.tu.text(e))) + 1
            n = min(data_len, t.n) if t.n else data_len
            lines.append("rt::memset(c, %s, 0, %d);" % (a, self.tu.sizeof(t)))
            lines.append("rt::memcpy(c, %s, sym::%s, %d);" % (a, sym, n))
            return lines
        return self.init_mem_nz(a, t, e)

    def init_mem_nz(self, a, t, e):
        if strip(e).get("kind") in ("InitListExpr",) or (strip(e).get("kind") == "StringLiteral" and isinstance(t, Array)):
            return self.init_mem(a, t, e)
        code, et = self.rv(e)
        if isinstance(t, (Record, Array)):
            return ["{ let s_ = %s; rt::memcpy(c, %s, s_, %d); }" % (code, a, self.tu.sizeof(t))]
        code = cast(code, et, t, self.native)
        return ["{ let v_ = %s; %s; }" % (code, store(t, a, "v_", self.native))]

    def hoist_val(self, code):
        return code

    def ret_code(self, n):
        ch = n.get("inner", [])
        if not ch:
            return "break 'f" if isinstance(self.ret_t, Void) else "break 'f %s" % (zero(self.ret_t, self.native) if not self.sret else "sret")
        code, t = self.rv(ch[0])
        if self.sret:
            return "break 'f { let s_ = %s; rt::memcpy(c, sret, s_, %d); sret }" % (code, self.tu.sizeof(self.ret_t))
        if isinstance(self.ret_t, Void):
            return "{ %s; break 'f; }" % code
        return "break 'f %s" % cast(code, t, self.ret_t, self.native)

    # ------------------------------------------------------------------ switch
    def segments(self, body):
        """[(labels, stmts)] for a switch whose case labels are all directly in its compound body; None otherwise."""
        items = body.get("inner", []) if body.get("kind") == "CompoundStmt" else [body]
        segs = []
        for it in items:
            labels = []
            cur = it
            while cur.get("kind") in ("CaseStmt", "DefaultStmt"):
                if cur["kind"] == "DefaultStmt":
                    labels.append(None)
                    cur = cur["inner"][0]
                else:
                    ch = cur["inner"]
                    vals = [x for x in ch[:-1]]
                    lo = int(self.g.const(vals[0])[1])
                    hi = int(self.g.const(vals[1])[1]) if len(vals) > 1 else None
                    labels.append((lo, hi))
                    cur = ch[-1]
            if labels:
                segs.append([labels, [cur]])
            elif segs:
                segs[-1][1].append(it)
            else:
                if it.get("kind") not in ("DeclStmt", "NullStmt"):
                    pass  # unreachable statements before the first label
        for _, st in segs:
            for s in st:
                if contains_case(s):
                    return None
        return segs

    def needs_flatten(self, body):
        found = [False]
        def walk(n):
            if n.get("kind") == "SwitchStmt":
                if self.segments(n["inner"][-1]) is None:
                    found[0] = True
            for c in n.get("inner", []):
                walk(c)
        walk(body)
        return found[0]

    def switch(self, n, out, ind):
        ch = n["inner"]
        cond, body = ch[0], ch[-1]
        code, t = self.rv(cond)
        segs = self.segments(body)
        S = self.label("s")
        sv = self.tmp("sv")
        rtyp = self.rt(t)
        def pat(lbl):
            lo, hi = lbl
            if hi is not None:
                return "%s..=%s" % (self.lit(wrap(lo, t), t), self.lit(wrap(hi, t), t))
            return self.lit(wrap(lo, t), t)
        fall = False
        for i, (labels, st) in enumerate(segs):
            nonempty = any(s.get("kind") != "NullStmt" for s in st)
            if i < len(segs) - 1 and nonempty and not ends_jump(st[-1]):
                fall = True
        self.emit(out, ind, "%s: {" % S)
        self.emit(out, ind + 1, "let %s: %s = %s;" % (sv, rtyp, code))
        self.loops.append(("break %s" % S, None))
        if not fall:
            self.emit(out, ind + 1, "match %s {" % sv)
            default = None
            pending = []
            for labels, st in segs:
                pending += labels
                if all(s.get("kind") == "NullStmt" for s in st) and (labels, st) is not segs[-1]:
                    if not any(s.get("kind") != "NullStmt" for s in st):
                        continue
                pats = [pat(l) for l in pending if l is not None]
                if None in pending:
                    arm = "_"
                else:
                    arm = " | ".join(pats) if pats else None
                pending = []
                if arm is None:
                    continue
                if arm == "_":
                    default = st
                    continue
                self.emit(out, ind + 2, "%s => {" % arm)
                for s in st:
                    self.stmt(s, out, ind + 3)
                self.emit(out, ind + 2, "}")
            self.emit(out, ind + 2, "_ => {")
            if default:
                for s in default:
                    self.stmt(s, out, ind + 3)
            self.emit(out, ind + 2, "}")
            self.emit(out, ind + 1, "}")
        else:
            names = [self.label("k") for _ in segs]
            for nm in reversed(names):
                self.emit(out, ind + 1, "%s: {" % nm)
            arms = []
            dflt = None
            for nm, (labels, st) in zip(names, segs):
                for l in labels:
                    if l is None:
                        dflt = nm
                    else:
                        arms.append("%s => break %s," % (pat(l), nm))
            self.emit(out, ind + 2, "match %s { %s _ => break %s, }" % (sv, " ".join(arms), dflt or S))
            for nm, (labels, st) in zip(names, segs):
                self.emit(out, ind + 1, "}")
                for s in st:
                    self.stmt(s, out, ind + 1)
        self.loops.pop()
        self.emit(out, ind, "}")

    # ------------------------------------------------------------------ flattened control flow (goto)
    def flatten(self, body, out):
        self.blocks = [[]]
        self.term = [None]
        self.cur = 0
        self.label_blocks = {}
        self.FL = self.label("fl")
        self.lower(body)
        self.jump_to(None)
        bb = "bb_"
        self.locals.append((bb, "u32", "0"))
        out.append("    %s: loop {" % self.FL)
        out.append("        match %s {" % bb)
        for i, (lines, t) in enumerate(zip(self.blocks, self.term)):
            out.append("            %d => {" % i)
            for l in lines:
                out.append("                " + l)
            if t is None:
                out.append("                break %s;" % self.FL)
            else:
                out.append("                " + t)
            out.append("            }")
        out.append("            _ => unreachable!(),")
        out.append("        }")
        out.append("    }")

    def new_block(self):
        self.blocks.append([])
        self.term.append(None)
        return len(self.blocks) - 1

    def jump_to(self, b):
        if self.term[self.cur] is None:
            self.term[self.cur] = ("bb_ = %d; continue %s;" % (b, self.FL)) if b is not None else None
            if b is None:
                self.term[self.cur] = "break %s;" % self.FL

    def jmp(self, b):
        return "bb_ = %d; continue %s;" % (b, self.FL)

    def start(self, b):
        self.cur = b

    def lab(self, name):
        if name not in self.label_blocks:
            self.label_blocks[name] = self.new_block()
        return self.label_blocks[name]

    def line(self, s):
        if self.term[self.cur] is None:
            self.blocks[self.cur].append(s)

    def lower(self, n):
        k = n.get("kind")
        if k == "CompoundStmt":
            for c in n.get("inner", []):
                self.lower(c)
        elif k == "LabelStmt":
            b = self.lab(n["declId"])
            self.jump_to(b)
            self.start(b)
            for c in n.get("inner", []):
                self.lower(c)
        elif k == "GotoStmt":
            b = self.lab(n["targetLabelDeclId"])
            # GotoStmt names the label by declaration id; map ids to names via LabelStmt declId
            self.jump_to(b)
            self.start(self.new_block())
        elif k == "IfStmt":
            ch = n["inner"]
            t, j = self.new_block(), self.new_block()
            e = self.new_block() if len(ch) > 2 else j
            self.term[self.cur] = "if %s { %s } else { %s }" % (self.cond(ch[0]), self.jmp(t), self.jmp(e))
            self.start(t); self.lower(ch[1]); self.jump_to(j)
            if len(ch) > 2:
                self.start(e); self.lower(ch[2]); self.jump_to(j)
            self.start(j)
        elif k == "WhileStmt":
            ch = n["inner"]
            h, b, x = self.new_block(), self.new_block(), self.new_block()
            self.jump_to(h)
            self.start(h)
            self.term[h] = "if %s { %s } else { %s }" % (self.cond(ch[0]), self.jmp(b), self.jmp(x))
            self.loops.append((self.jmp(x), self.jmp(h)))
            self.start(b); self.lower(ch[1]); self.jump_to(h)
            self.loops.pop()
            self.start(x)
        elif k == "DoStmt":
            body, cond = n["inner"]
            b, cnd, x = self.new_block(), self.new_block(), self.new_block()
            self.jump_to(b)
            self.loops.append((self.jmp(x), self.jmp(cnd)))
            self.start(b); self.lower(body); self.jump_to(cnd)
            self.loops.pop()
            self.start(cnd)
            self.term[cnd] = "if %s { %s } else { %s }" % (self.cond(cond), self.jmp(b), self.jmp(x))
            self.start(x)
        elif k == "ForStmt":
            init, _cv, cond, inc, body = n["inner"]
            if init:
                self.lower(init)
            h, b, i, x = self.new_block(), self.new_block(), self.new_block(), self.new_block()
            self.jump_to(h)
            self.start(h)
            self.term[h] = ("if %s { %s } else { %s }" % (self.cond(cond), self.jmp(b), self.jmp(x))) if cond else self.jmp(b)
            self.loops.append((self.jmp(x), self.jmp(i)))
            self.start(b); self.lower(body); self.jump_to(i)
            self.loops.pop()
            self.start(i)
            if inc:
                self.line(self.expr_stmt(inc) + ";")
            self.jump_to(h)
            self.start(x)
        elif k == "SwitchStmt":
            ch = n["inner"]
            code, t = self.rv(ch[0])
            sv = self.uniq("sv")
            self.locals.append((sv, self.rt(t), "0"))
            self.line("%s = %s;" % (sv, code))
            disp = self.cur
            x = self.new_block()
            ctx = {"cases": [], "default": None, "t": t}
            self.switch_ctx = getattr(self, "switch_ctx", [])
            self.switch_ctx.append(ctx)
            self.loops.append((self.jmp(x), None))
            self.start(self.new_block())  # code before the first label is unreachable
            self.lower(ch[-1])
            self.jump_to(x)
            self.loops.pop()
            self.switch_ctx.pop()
            arms = []
            for (lo, hi), b in ctx["cases"]:
                p = self.lit(wrap(lo, t), t) if hi is None else "%s..=%s" % (self.lit(wrap(lo, t), t), self.lit(wrap(hi, t), t))
                arms.append("%s => %d," % (p, b))
            self.term[disp] = "bb_ = match %s { %s _ => %d, }; continue %s;" % (sv, " ".join(arms), ctx["default"] if ctx["default"] is not None else x, self.FL)
            self.start(x)
        elif k in ("CaseStmt", "DefaultStmt"):
            b = self.new_block()
            self.jump_to(b)
            self.start(b)
            ctx = self.switch_ctx[-1]
            if k == "DefaultStmt":
                ctx["default"] = b
                self.lower(n["inner"][0])
            else:
                ch = n["inner"]
                lo = int(self.g.const(ch[0])[1])
                hi = int(self.g.const(ch[1])[1]) if len(ch) > 2 else None
                ctx["cases"].append(((lo, hi), b))
                self.lower(ch[-1])
        elif k == "BreakStmt":
            self.term[self.cur] = self.term[self.cur] or self.loops[-1][0]
            self.start(self.new_block())
        elif k == "ContinueStmt":
            cont = [l for l in reversed(self.loops) if l[1] is not None][0]
            self.term[self.cur] = self.term[self.cur] or cont[1]
            self.start(self.new_block())
        elif k == "ReturnStmt":
            self.term[self.cur] = self.term[self.cur] or (self.ret_code(n) + ";")
            self.start(self.new_block())
        elif k == "DeclStmt":
            for d in n.get("inner", []):
                if d.get("kind") == "VarDecl":
                    tmp = []
                    self.decl_init(d, tmp, 0)
                    for l in tmp:
                        self.line(l.strip())
        elif k == "NullStmt":
            pass
        elif k == "GCCAsmStmt":
            self.line("/* asm */")
        else:
            self.line(self.expr_stmt(n) + ";")

from funcgen_expr import ExprMixin
for _k, _v in ExprMixin.__dict__.items():
    if not _k.startswith("__"):
        setattr(FuncGen, _k, _v)
