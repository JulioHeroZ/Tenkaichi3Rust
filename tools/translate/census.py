import json, sys, collections, glob, concurrent.futures, os
def scan(path):
    d = json.load(open(path))
    kinds = collections.Counter(); builtins = collections.Counter(); casts = collections.Counter()
    defined = set(); declared_used = set(); gvars_def=set(); extern_vars=set()
    file_nodes = []
    for top in d.get("inner", []):
        # only nodes from this TU's main file are interesting for definitions; with -P there are no file locs, so use all
        k = top.get("kind")
        if k == "FunctionDecl":
            if any(c.get("kind") == "CompoundStmt" for c in top.get("inner", [])):
                defined.add(top.get("name"))
        if k == "VarDecl":
            if top.get("storageClass") == "extern" and "init" not in top: extern_vars.add(top["name"])
            else: gvars_def.add(top["name"])
    stack = [d]
    while stack:
        n = stack.pop()
        k = n.get("kind")
        if k:
            kinds[k] += 1
        if k in ("ImplicitCastExpr", "CStyleCastExpr"):
            casts[n.get("castKind")] += 1
        if k == "DeclRefExpr":
            r = n.get("referencedDecl", {})
            if r.get("kind") == "FunctionDecl":
                declared_used.add(r.get("name"))
                if r.get("name","").startswith("__builtin"): builtins[r["name"]] += 1
        stack.extend(n.get("inner", []))
    return kinds, casts, builtins, defined, declared_used, gvars_def, extern_vars
files = sorted(glob.glob("/home/claude/ast/*.json"))
K=collections.Counter(); C=collections.Counter(); B=collections.Counter(); D=set(); U=set(); GV=set(); EV=set()
with concurrent.futures.ProcessPoolExecutor(8) as ex:
    for k,c,b,dd,u,gv,ev in ex.map(scan, files):
        K+=k; C+=c; B+=b; D|=dd; U|=u; GV|=gv; EV|=ev
print("KINDS", sorted(K.items(), key=lambda x:-x[1]))
print("CASTS", sorted(C.items(), key=lambda x:-x[1]))
print("BUILTINS", B)
ext = sorted(U - D)
print(len(D), "defined functions;", len(ext), "used but not defined")
open("/home/claude/tr/external_funcs.txt","w").write("\n".join(ext)+"\n")
print("extern vars not defined:", sorted(EV-GV)[:80], len(EV-GV))
