"""Loading one translation unit: AST, source text, record layouts, typedef/enum/record tables."""
import json, re, pathlib
from ctype import Parser, Int, Float, Ptr, Array, Func, Record, Void, T
import layout

FLOAT_MODEL_NATIVE = set()  # TU ids compiled with hardware float (none translated so far)

class TU:
    def __init__(self, tuid, ast_path, src_path, layout_path):
        self.id = tuid
        self.ast = json.load(open(ast_path))
        self.src = open(src_path, "rb").read().decode("latin-1")
        self.id_file = pathlib.Path(src_path).name
        self._nl = [i for i, ch in enumerate(self.src) if ch == "\n"]
        self.layouts = layout.parse(open(layout_path).read())
        # records the real-layout dump does not reach (function-local, unused anonymous ones): the eager dump
        lp = pathlib.Path(layout_path).resolve()
        fallback = lp.parent.parent / "layout" / lp.name if lp.parent.name == "layout2" else lp
        if fallback != lp and fallback.exists():
            for k, v in layout.parse(fallback.read_text()).items():
                self.layouts.setdefault(k, v)
        self.native_float = tuid in FLOAT_MODEL_NATIVE
        self.typedefs, self.enums, self.enum_consts = {}, {}, {}
        self.records = {}      # (tag,key) -> list of FieldDecl nodes (complete definition)
        self.field_of = {}     # FieldDecl id -> ((tag,key), index)
        self.asm_label = {}    # declared name -> symbol
        self.decls = []        # top-level nodes in order
        self._alias, self._rid, self.varty = [], {}, {}
        self.const_vars = {}   # VarDecl id -> (type, initializer) of const-qualified variables
        self.scan(self.ast)
        for alias, rid in self._alias:
            if rid in self._rid:
                k = self._rid[rid]
                ak = (k[0], alias)
                if k in self.layouts:
                    self.layouts.setdefault(ak, self.layouts[k])
                if k in self.records:
                    self.records.setdefault(ak, self.records[k])
        self.parser = Parser(self.typedefs, self.enums, self.varty)
        self.parser.typedefs.setdefault("__builtin_va_list", Ptr(Int(1, True)))
        self.parser.typedefs.setdefault("va_list", Ptr(Int(1, True)))

    def text(self, node):
        r = node["range"]
        b = r["begin"]["offset"]
        e = r["end"]["offset"] + r["end"].get("tokLen", 0)
        return self.src[b:e]

    def scan(self, root):
        for n in root.get("inner", []):
            self.decls.append(n)
        stack = list(root.get("inner", []))
        while stack:
            n = stack.pop()
            k = n.get("kind")
            if k == "TypedefDecl":
                self.typedefs[n["name"]] = n["type"]["qualType"]
                for c in n.get("inner", []):
                    o = c.get("ownedTagDecl")
                    if o and o.get("kind") == "RecordDecl" and not o.get("name"):
                        self._alias.append((n["name"], o["id"]))
            elif k in ("VarDecl", "ParmVarDecl") and n.get("name"):
                self.varty.setdefault(n["name"], n["type"]["qualType"])
                if k == "VarDecl" and n.get("mangledName") and n["mangledName"] != n["name"]:
                    self.asm_label[n["name"]] = n["mangledName"]
                qt = n["type"]["qualType"]
                if k == "VarDecl" and qt.startswith("const ") and n.get("init") and n.get("inner"):
                    self.const_vars[n["id"]] = (qt, n["inner"][-1])
            elif k == "EnumDecl":
                self.scan_enum(n)
            elif k == "RecordDecl" and n.get("completeDefinition"):
                key = self.record_key(n)
                self._rid[n["id"]] = key
                fields = [c for c in n.get("inner", []) if c.get("kind") == "FieldDecl"]
                self.records[key] = fields
                for i, f in enumerate(fields):
                    self.field_of[f["id"]] = (key, i)
            elif k == "FunctionDecl":
                if n.get("mangledName") and n["mangledName"] != n["name"]:
                    self.asm_label[n["name"]] = n["mangledName"]
            if k in ("RecordDecl", "FunctionDecl", "CompoundStmt", "DeclStmt", "TranslationUnitDecl", "VarDecl") or True:
                stack.extend(n.get("inner", []))

    def record_key(self, n):
        tag = n.get("tagUsed", "struct")
        if n.get("name"):
            return (tag, n["name"])
        off = n["loc"]["offset"]
        import bisect
        line = bisect.bisect_left(self._nl, off)  # newlines before offset
        col = off - (self._nl[line - 1] + 1 if line > 0 else 0) + 1
        return (tag, "@%s:%d:%d" % (self.id_file, line + 1, col))

    def scan_enum(self, n):
        val = -1
        key = n.get("name") or "@enum%s" % n["id"]
        vals = []
        for c in n.get("inner", []):
            if c.get("kind") != "EnumConstantDecl":
                continue
            ce = [x for x in c.get("inner", []) if x.get("kind") == "ConstantExpr"]
            if ce and "value" in ce[0]:
                val = int(ce[0]["value"])
            else:
                val = val + 1
            self.enum_consts[c["name"]] = val
            vals.append(val)
        unsigned = vals and max(vals) > 0x7FFFFFFF
        self.enums[key] = Int(4, not unsigned)

    # ---- types
    def ty(self, tnode):
        """Canonical type of a JSON 'type' object."""
        q = tnode.get("desugaredQualType") or tnode["qualType"]
        try:
            return self.parser.parse(q)
        except Exception:
            return self.parser.parse(tnode["qualType"])

    def layout(self, rec):
        key = (rec.tag, rec.key)
        if key not in self.layouts:
            raise KeyError("no layout for %s %s in %s" % (rec.tag, rec.key, self.id))
        return self.layouts[key]

    def sizeof(self, t):
        if isinstance(t, (Int, Float)):
            return t.size
        if isinstance(t, Ptr):
            return 4
        if isinstance(t, Array):
            return self.sizeof(t.elem) * (t.n or 0)
        if isinstance(t, Record):
            return self.layout(t)["size"]
        if isinstance(t, Void):
            return 1
        if isinstance(t, Func):
            return 1
        raise TypeError(t)

    def alignof(self, t):
        if isinstance(t, (Int, Float)):
            return min(t.size, 8)
        if isinstance(t, Ptr):
            return 4
        if isinstance(t, Array):
            return self.alignof(t.elem)
        if isinstance(t, Record):
            return self.layout(t)["align"]
        return 1

    def field(self, field_id):
        """-> (offset, bitfirst, bitwidth, type) of a FieldDecl."""
        key, i = self.field_of[field_id]
        fnode = self.records[key][i]
        lay = self.layouts[key]
        off, b0, b1, name = lay["fields"][i]
        if fnode.get("name") and name and fnode["name"] != name:
            raise ValueError("field order mismatch %s: %s vs %s" % (key, fnode["name"], name))
        t = self.ty(fnode["type"])
        if b0 is not None and fnode.get("isBitfield"):
            return off, b0, b1 - b0 + 1, t
        return off, None, None, t
