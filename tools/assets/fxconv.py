"""The effects' tables as typed records (layout.py) in effect-tables.json: found by walking the directories of a file
(effect packs of the techniques, the stages and the common files) and recognised by their own structure.

* Emitter sets (EftEmit_LoadSet, src/battle/eft_h.c; include/battle/eft_h.h): an EmitterSet header (mask of the
  group kinds, group and part counts, times), its EmitterGroup table (kind, parts, resources) and its EmitterPart
  table (flags, resource, start / end phase...). The entries after the set in the pack are the groups' texture
  sets and, per part, its data a and b; the particle emitters' a (kind 5) is a ParticleDef (include/battle/eft_v.h:
  flags, shape, sheet, key times, the keyed values, angles and spins) and its b ParticleKeys (80 more keyed
  values: colours and sizes, not named yet); kind 2 RaysDef (eft_o.h), kind 9 Part9Def (eft_x.h), kind 16 BillDef (eft_z.h), kind 10 Part10Def (eft_x.h, eft_w.h), kind
  17 RibbonPrm + RibbonAnim (eft_ab.h); the other kinds are not typed yet.
* "V000" sprite animation packs (EftSprPack, include/battle/eft_ae.h): SpritePack header, SpriteLayer definitions
  (texture, ordering layer, speed, keys, loop, length, grid, blink, flags, fade, jitter), SpriteKey keys (position,
  rotation in degrees, scale, colour, frame).
"""
import struct
import layout


def u32(d, o):
    return struct.unpack_from("<I", d, o)[0]


def types(lay):
    lay.type("EmitterSet", 0x40, [
        ("mask", 0, "u32"), ("groupCount", 4, "u8"), ("partCount", 5, "u8"), ("aimKind", 6, "u8"), ("unk7", 7, "u8"),
        ("unk8", 8, "f32"), ("unkC", 0x0C, "f32[2]"), ("unk14", 0x14, "f32"), ("unk18", 0x18, "f32"),
        ("unk1C", 0x1C, "f32"), ("endFrames", 0x20, "u8"), ("unk21", 0x21, "u8"), ("unk22", 0x22, "u8"),
        ("unk23", 0x23, "u8[2]"), ("unk25", 0x25, "u8"), ("unk26", 0x26, "u8"), ("phaseMask", 0x27, "u8"),
        ("unk28", 0x28, "f32"), ("unk2C", 0x2C, "f32"), ("unk30", 0x30, "f32[2]"), ("unk38", 0x38, "f32"),
        ("unk3C", 0x3C, "f32")])
    lay.type("EmitterGroup", 0x20, [("kind", 0, "u8"), ("partCount", 1, "u8"), ("resourceCount", 2, "u8"),
                                    ("unk3", 3, "u8[29]")])
    lay.type("EmitterPart", 0x40, [
        ("flags", 0, "u8"), ("resource", 1, "u8"), ("unk2", 2, "u8"), ("unk3", 3, "u8"), ("light", 4, "u8"),
        ("unk5", 5, "u8"), ("unk6", 6, "u8"), ("unk7", 7, "u8"), ("phase", 8, "u8"), ("endPhase", 9, "u8"),
        ("unkA", 0x0A, "u8"), ("unkB", 0x0B, "u8[5]"), ("unk10", 0x10, "f32"), ("unk14", 0x14, "f32"),
        ("unk18", 0x18, "f32"), ("unk1C", 0x1C, "f32"), ("unk20", 0x20, "u8[20]"), ("unk34", 0x34, "u8"),
        ("unk35", 0x35, "u8"), ("unk36", 0x36, "u8"), ("unk37", 0x37, "u8[9]")])
    lay.type("ParticleDef", 0x2B0, [
        ("flags", 0, "s32"), ("unk4", 4, "u8"), ("shape", 5, "u8"), ("flip", 6, "u8"), ("centred", 7, "u8"),
        ("unk8", 8, "u8[2]"), ("layer", 0x0A, "u8"), ("unkB", 0x0B, "u8[3]"), ("cols", 0x0E, "u8"), ("rows", 0x0F, "u8"),
        ("animTime", 0x10, "f32"), ("keyTime", 0x14, "f32"), ("keySplit", 0x18, "f32"), ("keys", 0x1C, "f32[153]"),
        ("angX", 0x280, "f32[2]"), ("spinX", 0x288, "f32[2]"), ("angY", 0x290, "f32[2]"), ("spinY", 0x298, "f32[2]"),
        ("angZ", 0x2A0, "f32[2]"), ("spinZ", 0x2A8, "f32[2]")])
    lay.type("ParticleKeys", 0x140, [("values", 0, "f32[80]")])
    # kind 2, rays (EftRaysDef, include/battle/eft_o.h): ten rays and their count and flags
    ray = []
    for i in range(10):
        b = 0x78 * i
        for k in range(3):
            ray += [("ray%dKey%dHeight" % (i, k), b + 0x10 * k, "f32"), ("ray%dKey%dWidth" % (i, k), b + 0x10 * k + 4, "f32"),
                    ("ray%dKey%dUnk8" % (i, k), b + 0x10 * k + 8, "f32"), ("ray%dKey%dColor" % (i, k), b + 0x10 * k + 12, "u8[4]")]
        ray += [("ray%dLen" % i, b + 0x30, "f32[4]"), ("ray%dFlickTime" % i, b + 0x40, "f32"), ("ray%dColA" % i, b + 0x44, "f32[3]"),
                ("ray%dColB" % i, b + 0x50, "f32[4]"), ("ray%dAngle" % i, b + 0x60, "f32"), ("ray%dSpin" % i, b + 0x64, "f32"),
                ("ray%dKeyTime" % i, b + 0x68, "f32"), ("ray%dKeySplit" % i, b + 0x6C, "f32"), ("ray%dDelay" % i, b + 0x70, "u16"),
                ("ray%dBlend" % i, b + 0x72, "s8"), ("ray%dUnk73" % i, b + 0x73, "u8[5]")]
    lay.type("RaysDef", 0x4B8, ray + [("count", 0x4B0, "s32"), ("flags", 0x4B4, "s32")])
    # kind 17, ribbon (EftRbnPrm / EftRbnAnim, include/battle/eft_ab.h)
    lay.type("RibbonPrm", 0x60, [("flags", 0, "s32"), ("blend", 4, "u8"), ("kind", 5, "u8"), ("unk6", 6, "u8[2]"),
                                 ("fadeIn", 8, "f32"), ("fadeOut", 0x0C, "f32"), ("animTime", 0x10, "f32"), ("unk14", 0x14, "f32"),
                                 ("width", 0x18, "f32[3]"), ("widthLo", 0x24, "f32[3]"), ("widthHi", 0x30, "f32[3]"),
                                 ("widthTime", 0x3C, "f32[3]"), ("alpha", 0x48, "f32[3]"), ("scroll", 0x54, "f32[3]")])
    lay.type("RibbonAnim", 0xB4, [("color", 0, "f32[12]"), ("unk30", 0x30, "f32[12]"), ("pulseR", 0x60, "f32[6]"),
                                  ("pulseG", 0x78, "f32[6]"), ("pulseB", 0x90, "f32[6]"), ("pulseTime", 0xA8, "f32[3]")])
    # kind 16, sprite (EftBillDef, include/battle/eft_z.h)
    lay.type("BillDef", 0x90, [("flags", 0, "s32"), ("blend", 4, "u8"), ("shape", 5, "u8"), ("cols", 6, "u8"),
                               ("rows", 7, "u8"), ("frameTime", 8, "f32"), ("fadeIn", 0x0C, "f32"), ("fadeOut", 0x10, "f32"),
                               ("keyTime", 0x14, "f32"), ("unk18", 0x18, "f32"), ("keys", 0x1C, "f32[27]"),
                               ("spinRange", 0x88, "f32"), ("angleRange", 0x8C, "f32")])
    # kind 9, quad emitter (EftPart9Def, include/battle/eft_x.h)
    lay.type("Part9Def", 0x243, [("keys", 0, "f32[134]"), ("unk218", 0x218, "f32"), ("unk21C", 0x21C, "f32"),
                                 ("sizeMid", 0x220, "f32"), ("unk224", 0x224, "f32"), ("unk228", 0x228, "f32"),
                                 ("unk22C", 0x22C, "f32"), ("unk230", 0x230, "f32"), ("flags", 0x234, "s32"),
                                 ("unk238", 0x238, "u8[2]"), ("interval", 0x23A, "u8"), ("unk23B", 0x23B, "u8[2]"),
                                 ("count", 0x23D, "u8"), ("layer", 0x23E, "u8"), ("texCols", 0x23F, "u8"),
                                 ("texRows", 0x240, "u8"), ("unk241", 0x241, "u8"), ("unk242", 0x242, "u8")])
    # kind 10, particles (EftPart10Def, include/battle/eft_x.h and eft_w.h)
    lay.type("Part10Def", 0x20E, [
        ("size", 0, "f32[9]"), ("sizeRange", 0x24, "f32[9]"), ("angle", 0x48, "f32[3]"), ("angleRange", 0x54, "f32[3]"),
        ("unkC0", 0x60, "f32[3]"), ("unkC8", 0x6C, "f32[3]"), ("unkD0", 0x78, "f32[3]"), ("unkD8", 0x84, "f32[3]"),
        ("spin", 0x90, "f32[3]"), ("spinRange", 0x9C, "f32[3]"), ("unkF0", 0xA8, "f32[3]"), ("unkF8", 0xB4, "f32[3]"),
        ("unk100", 0xC0, "f32[3]"), ("unk108", 0xCC, "f32[3]"), ("speed", 0xD8, "f32[3]"), ("speedRange", 0xE4, "f32[3]"),
        ("speedBase", 0xF0, "f32[3]"), ("unk128", 0xFC, "f32[3]"), ("unk130", 0x108, "f32[3]"), ("scaleX", 0x114, "f32[3]"),
        ("scaleY", 0x120, "f32[3]"), ("life", 0x12C, "f32[3]"), ("lifeRange", 0x138, "f32[3]"), ("unk158", 0x144, "f32[3]"),
        ("unk160", 0x150, "f32[3]"), ("unk168", 0x15C, "f32[6]"), ("unk178", 0x174, "f32[3]"), ("twistStep", 0x180, "f32"),
        ("spinA", 0x184, "f32[3]"), ("spinARange", 0x190, "f32[3]"), ("spinB", 0x19C, "f32[3]"), ("spinBRange", 0x1A8, "f32[3]"),
        ("scale", 0x1B4, "f32[3]"), ("scaleRange", 0x1C0, "f32[3]"), ("count", 0x1CC, "f32[3]"), ("spinTime", 0x1D8, "f32"),
        ("spinMid", 0x1DC, "f32"), ("keyTime", 0x1E0, "f32"), ("keyMid", 0x1E4, "f32"), ("fadeIn", 0x1E8, "f32"),
        ("fadeOut", 0x1EC, "f32"), ("sizeMid", 0x1F0, "f32"), ("flags", 0x1F4, "s32"), ("texCols", 0x1F8, "u8"),
        ("texRows", 0x1F9, "u8"), ("texStep", 0x1FA, "u8"), ("layer", 0x1FB, "u8"), ("ringFrom", 0x1FC, "f32"),
        ("ringTo", 0x200, "f32"), ("yaw", 0x204, "f32"), ("pitch", 0x208, "f32"), ("drawMode", 0x20C, "u8"),
        ("unk20D", 0x20D, "u8")])
    lay.type("SpritePack", 0x20, [("magic", 0, "u8[4]"), ("layerCount", 4, "u8"), ("loop", 5, "u8"),
                                  ("unk6", 6, "u8[2]"), ("keys", 8, "s32"), ("unkC", 0x0C, "u8[20]")])
    lay.type("SpriteLayer", 0x20, [
        ("tex", 0, "u8"), ("unk1", 1, "u8"), ("otLayer", 2, "u8"), ("speed", 3, "u8"), ("keyCount", 4, "u8"),
        ("loopEndKey", 5, "u8"), ("loopStartKey", 6, "u8"), ("loopCount", 7, "s8"), ("length", 8, "s16"),
        ("jitterPeriod", 0x0A, "s16"), ("grid", 0x0C, "s8"), ("blinkOn", 0x0D, "s8"), ("blinkOff", 0x0E, "s8"),
        ("gridStep", 0x0F, "s8"), ("flags", 0x10, "u32"), ("keysPtr", 0x14, "u32"), ("fade", 0x18, "s32"),
        ("jitterX", 0x1C, "s16"), ("jitterY", 0x1E, "s16")])
    lay.type("SpriteKey", 0x40, [("pos", 0, "f32[4]"), ("rot", 0x10, "f32[4]"), ("scale", 0x20, "f32[4]"),
                                 ("color", 0x30, "u8[4]"), ("frame", 0x34, "s32"), ("unk38", 0x38, "s32[2]")])


# the typed data a (and b) of a part, by group kind
PART_TYPES = {2: ("RaysDef", None), 5: ("ParticleDef", "ParticleKeys"), 9: ("Part9Def", None),
              10: ("Part10Def", None), 16: ("BillDef", None), 17: ("RibbonPrm", "RibbonAnim")}


def emitter_set(d, a, b):
    """-> (groups, parts) when [a, b) is an emitter set, else None"""
    if b - a < 0x40:
        return None
    mask, ng, np_ = u32(d, a), d[a + 4], d[a + 5]
    if not 1 <= ng <= 19 or not np_:
        return None
    size = 0x40 + 0x20 * ng + 0x40 * np_
    if size > b - a or (b - a) - size >= 64:
        return None
    kinds = [d[a + 0x40 + 0x20 * g] for g in range(ng)]
    if any(k > 18 for k in kinds) or len(set(kinds)) != ng:
        return None
    m = 0
    for k in kinds:
        m |= 1 << k
    if m != mask or sum(d[a + 0x41 + 0x20 * g] for g in range(ng)) != np_:
        return None
    return ng, np_


def sprite_pack(d, a, b):
    """-> (layers, keys per layer) when [a, b) is a V000 pack, else None"""
    if b - a < 0x40 or d[a:a + 4] != b"V000":
        return None
    nl, ko = d[a + 4], u32(d, a + 8)
    if not nl or ko != 0x20 + 0x20 * nl:
        return None
    counts = [d[a + 0x20 + 0x20 * i + 4] for i in range(nl)]
    if ko + 0x40 * sum(counts) > b - a:
        return None
    return nl, counts


def walk(d, base, end, depth, out):
    """the entries of every directory: out gets one list of (start, end) per directory"""
    if depth > 4 or end - base < 8:
        return
    n = u32(d, base)
    if not 1 <= n <= 4096 or 4 + 4 * n > end - base:
        return
    offs = [u32(d, base + 4 + 4 * i) & ~3 for i in range(n)]
    if offs[0] < 4 + 4 * n or any(y < x for x, y in zip(offs, offs[1:])) or base + offs[-1] > end:
        return
    ends = offs[1:] + [end - base]
    out.append([(base + o, base + e) for o, e in zip(offs, ends)])
    for o, e in zip(offs, ends):
        if e > o:
            walk(d, base + o, base + e, depth + 1, out)


def convert(d, skel):
    """-> effect-tables.json text or None (what it covers zeroed in skel)."""
    dirs = []
    walk(d, 0, len(d), 0, dirs)
    lay = layout.Layout(0)
    types(lay)
    for ents in dirs:
        for i, (a, b) in enumerate(ents):
            es = emitter_set(d, a, b)
            if es:
                ng, np_ = es
                lay.read(d, "EmitterSet", a)
                for g in range(ng):
                    lay.read(d, "EmitterGroup", a + 0x40 + 0x20 * g)
                for p in range(np_):
                    lay.read(d, "EmitterPart", a + 0x40 + 0x20 * ng + 0x40 * p)
                # the entries after the set: per group its texture sets, then per part its data a (and b)
                k = i + 1
                for g in range(ng):
                    kind, nparts, nres = d[a + 0x40 + 0x20 * g], d[a + 0x41 + 0x20 * g], d[a + 0x42 + 0x20 * g]
                    if kind == 0:
                        continue
                    k += nres
                    for _ in range(nparts):
                        ta, tb = PART_TYPES.get(kind, (None, None))
                        for t, j in ((ta, k), (tb, k + 1)):
                            if t and j < len(ents) and ents[j][1] - ents[j][0] >= lay.types[t]["size"]:
                                lay.read(d, t, ents[j][0])
                        k += 1 if kind in (2, 14) else 2
                continue
            sp = sprite_pack(d, a, b)
            if sp:
                nl, counts = sp
                lay.read(d, "SpritePack", a)
                for j in range(nl):
                    lay.read(d, "SpriteLayer", a + 0x20 + 0x20 * j)
                kk = a + 0x20 + 0x20 * nl
                for c in counts:
                    for _ in range(c):
                        lay.read(d, "SpriteKey", kk)
                        kk += 0x40
    if not lay.records:
        return None
    layout.finish(lay, d, skel)
    return lay.text("the effects' tables (tools/assets/fxconv.py): emitter sets (header, groups, parts), the particle "
                    "emitters' definitions (kind 5) and V000 sprite animations (layers and keys); names after "
                    "include/battle/eft_h.h, eft_v.h and eft_ae.h")
