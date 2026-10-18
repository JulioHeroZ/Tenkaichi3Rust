import re
from trans import strip, Unsupported, parse_c_string, wrap, KEYWORDS

def rust_ident(name):
    return "r#" + name if name in KEYWORDS else name

def contains_case(n):
    k = n.get("kind")
    if k in ("CaseStmt", "DefaultStmt"):
        return True
    if k == "SwitchStmt":
        return False
    return any(contains_case(c) for c in n.get("inner", []))

def ends_jump(n):
    k = n.get("kind")
    if k in ("BreakStmt", "ReturnStmt", "ContinueStmt", "GotoStmt"):
        return True
    if k == "CompoundStmt" and n.get("inner"):
        return ends_jump(n["inner"][-1])
    return False
