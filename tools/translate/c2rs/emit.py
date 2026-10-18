"""Rust code fragments for C scalar operations over the PS2 memory model."""
from ctype import Int, Float, Ptr, Array, Func, Record, Void

def rty(t, native=False):
    if isinstance(t, Int):
        if t.boolean:
            return "u8"
        return ("i" if t.signed else "u") + str(t.size * 8)
    if isinstance(t, Ptr):
        return "u32"
    if isinstance(t, Float):
        return "f64" if t.size == 8 else ("f32" if native else "F32")
    if isinstance(t, Void):
        return "()"
    if isinstance(t, (Record, Array)):
        return "u32"  # address of the object
    if isinstance(t, Func):
        return "u32"
    raise TypeError(t)

def zero(t, native=False):
    r = rty(t, native)
    return {"F32": "F32(0)", "f32": "0.0f32", "f64": "0.0f64", "()": "()"}.get(r, "0")

def load(t, a, native=False):
    if isinstance(t, Int):
        if t.size == 1:
            return "(c.m.r8(%s) as i8)" % a if t.signed and not t.boolean else "c.m.r8(%s)" % a
        if t.size == 2:
            return "(c.m.r16(%s) as i16)" % a if t.signed else "c.m.r16(%s)" % a
        if t.size == 4:
            return "(c.m.r32(%s) as i32)" % a if t.signed else "c.m.r32(%s)" % a
        if t.size == 8:
            return "(c.m.r64(%s) as i64)" % a if t.signed else "c.m.r64(%s)" % a
        if t.size == 16:
            return "(c.m.r128(%s) as i128)" % a if t.signed else "c.m.r128(%s)" % a
    if isinstance(t, Ptr):
        return "c.m.r32(%s)" % a
    if isinstance(t, Float):
        if t.size == 8:
            return "f64::from_bits(c.m.r64(%s))" % a
        return ("f32::from_bits(c.m.r32(%s))" if native else "F32(c.m.r32(%s))") % a
    if isinstance(t, (Record, Array, Func)):
        return a
    raise TypeError(t)

def store(t, a, v, native=False):
    if isinstance(t, Int):
        fn = {1: "w8", 2: "w16", 4: "w32", 8: "w64", 16: "w128"}[t.size]
        u = {1: "u8", 2: "u16", 4: "u32", 8: "u64", 16: "u128"}[t.size]
        if t.signed:
            return "c.m.%s(%s, (%s) as %s)" % (fn, a, v, u)
        return "c.m.%s(%s, %s)" % (fn, a, v)
    if isinstance(t, Ptr):
        return "c.m.w32(%s, %s)" % (a, v)
    if isinstance(t, Float):
        if t.size == 8:
            return "c.m.w64(%s, (%s).to_bits())" % (a, v)
        return ("c.m.w32(%s, (%s).to_bits())" if native else "c.m.w32(%s, (%s).0)") % (a, v)
    raise TypeError(t)

def to_raw(code, t, native=False):
    """Value -> u64 argument slot (cdecl-like: 32-bit values zero-extended, signed sign-extended)."""
    if isinstance(t, Int):
        if t.signed:
            return "((%s) as i64 as u64)" % code
        return "((%s) as u64)" % code
    if isinstance(t, (Ptr, Record, Array, Func)):
        return "((%s) as u64)" % code
    if isinstance(t, Float):
        if t.size == 8:
            return "(%s).to_bits()" % code
        return "((%s).to_bits() as u64)" % code if native else "((%s).0 as u64)" % code
    if isinstance(t, Void):
        return "{ %s; 0u64 }" % code
    raise TypeError(t)

def from_raw(code, t, native=False):
    if isinstance(t, Int):
        return "((%s) as %s)" % (code, rty(t))
    if isinstance(t, (Ptr, Record, Array, Func)):
        return "((%s) as u32)" % code
    if isinstance(t, Float):
        if t.size == 8:
            return "f64::from_bits(%s)" % code
        return "f32::from_bits((%s) as u32)" % code if native else "F32((%s) as u32)" % code
    if isinstance(t, Void):
        return "{ let _ = %s; }" % code
    raise TypeError(t)

def same(a, b, native=False):
    if isinstance(a, (Record, Array)) or isinstance(b, (Record, Array)):
        return True
    return rty(a, native) == rty(b, native)

def adapt(code, a, b, native=False):
    """Reinterpret through an argument slot (calls whose declaration does not match the definition)."""
    if same(a, b, native):
        return code
    if isinstance(b, Void):
        return "{ let _ = %s; }" % code
    return from_raw(to_raw(code, a, native), b, native)

def is_bool(t):
    return isinstance(t, Int) and t.boolean

def cast(code, a, b, native=False):
    """C conversion of a value of type a to type b."""
    if isinstance(b, Void):
        return "{ let _ = %s; }" % code
    if isinstance(b, (Record, Array)) or isinstance(a, (Record, Array)):
        if isinstance(a, Array) and isinstance(b, Ptr):
            return code
        return code
    ra, rb = rty(a, native), rty(b, native)
    if is_bool(b) and not is_bool(a):
        return "((%s) as u8)" % truth(code, a, native)
    if isinstance(a, Float) or isinstance(b, Float):
        if isinstance(a, Float) and isinstance(b, Float):
            if a.size == b.size:
                return code
            if native:
                return "((%s) as %s)" % (code, rb)
            return "rt::f2d(%s)" % code if b.size == 8 else "rt::d2f(%s)" % code
        if isinstance(a, Float):  # float -> integer / pointer
            if native and a.size == 4:
                return "((%s) as %s)" % (code, rb)
            d = "d" if a.size == 8 else "f"
            if isinstance(b, Int) and b.size == 8:
                return "rt::%s2l(%s)" % (d, code) if b.signed else "rt::%s2ul(%s)" % (d, code)
            if isinstance(b, Ptr) or (isinstance(b, Int) and not b.signed and b.size == 4):
                return "rt::%s2u(%s)" % (d, code)
            base = "rt::%s2i(%s)" % (d, code)
            return base if rb == "i32" else "((%s) as %s)" % (base, rb)
        # integer -> float
        if b.size == 8:
            return "((%s) as f64)" % code
        if native:
            return "((%s) as f32)" % code
        if isinstance(a, Int) and a.size == 8:
            return "rt::l2f(%s)" % code if a.signed else "rt::ul2f(%s)" % code
        if isinstance(a, Int) and not a.signed and a.size == 4 or isinstance(a, Ptr):
            return "rt::u2f(%s)" % code
        return "rt::i2f((%s) as i32)" % code if ra != "i32" else "rt::i2f(%s)" % code
    if ra == rb:
        return code
    return "((%s) as %s)" % (code, rb)

def truth(code, t, native=False):
    """C truth value as a Rust bool."""
    if isinstance(t, Float):
        if t.size == 8:
            return "((%s) != 0.0)" % code
        return "((%s) != 0.0)" % code if native else "rt::ftrue(%s)" % code
    if isinstance(t, (Record, Array)):
        return "true"
    return "((%s) != 0)" % code
