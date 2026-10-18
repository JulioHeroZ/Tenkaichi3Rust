"""The game tables of a fighter's model file as typed records (layout.py) in model-tables.json. The entries are
those BtlObj (src/battle/bobj_a.c) points the object at; the names follow include/battle/btl_tech_b.h,
src/battle/btl_char_hit.c and include/battle/bobj_b.h ("entry n" there counts from 1: file entry n - 1 here):

* entry 1 (BObjMdl.unk3C, the body of include/battle/bobj_a.h): BodyData and its BodyPart list (node, centre, the
  oriented box: centre, axes, matrix, corners);
* entry 0x11 (obj + 0x91C): FighterParam;
* entry 0x12 (obj + 0x920): AttackRecord, 0x30 bytes each, indexed by the attack id;
* entry 0x13 (obj + 0x924): KiBlast, the 13 ki blast kinds;
* entry 0x14 (obj + 0x928): MoveParam, speeds, turn rates, ki costs;
* entry 0x17 (obj + 0x92C): SuperParam, the techniques of slots 2..4 (two Blast 2, the Ultimate);
* entry 0x18 (obj + 0x930): SkillParam, the skills of slots 0..1 (Blast 1);
* entry 0x16 (obj + 0x940): ChainParam, the secondary motion (hair, tails, capes);
* entry 0x1A (obj + 0x934): AiParam, the CPU's values at the lowest and at the highest level (src/battle/btl_ai_*.c
  read value n of both and scale by the CPU level);
* entry 0x1B (obj + 0x938): CameraCut, the character's camera cuts (include/battle/btl_char_cam.h).

What a table does not fill of its entry stays for file-tables.json.
"""
import struct
import layout


def u32(d, o):
    return struct.unpack_from("<I", d, o)[0]


def types(lay):
    lay.type("FighterParam", 0xCC, [
        ("unk5C", 0x5C, "f32"), ("hitReactScale", 0x60, "f32"), ("gauge99CTime", 0x64, "f32"), ("unk68", 0x68, "f32"),
        ("unk6C", 0x6C, "f32"), ("unk70", 0x70, "s16"), ("unk72", 0x72, "s16"), ("chargeRateA", 0x74, "f32"),
        ("chargeRateB", 0x78, "f32"), ("damageTaken", 0x7C, "f32"), ("unk8F", 0x8F, "u8[56]"), ("unkC7", 0xC7, "u8"),
        ("unkC8", 0xC8, "f32")])
    lay.type("AttackRecord", 0x30, [
        ("flags", 0, "u32"), ("damage", 4, "u16"), ("guardDamage", 6, "u16"), ("guardKiCost", 8, "u16"),
        ("kiGain", 0x0A, "u16"), ("unk0C", 0x0C, "u16[2]"), ("unk10", 0x10, "u16"), ("pushOnHit", 0x12, "u8"),
        ("pushOnGuard", 0x13, "u8"), ("unk14", 0x14, "u8"), ("launchAngleA", 0x15, "s8"), ("launchAngleB", 0x16, "s8"),
        ("shakePower", 0x17, "u8"), ("shakeTime", 0x18, "u8"), ("shakePowerGuard", 0x19, "u8"),
        ("shakeTimeGuard", 0x1A, "u8"), ("reaction", 0x1B, "s8[5]"), ("reactionSub", 0x20, "s8"),
        ("reactionFG", 0x21, "s8[2]"), ("hitFxKind", 0x23, "s8"), ("unk24", 0x24, "u8[2]"), ("priority", 0x26, "s8"),
        ("guardKindB", 0x27, "s8"), ("guardKindC", 0x28, "s8"), ("guardKindA", 0x29, "s8"), ("guardKindE", 0x2A, "s8"),
        ("guardFlagSel", 0x2B, "s8"), ("ignoreArmour", 0x2C, "s8"), ("unk2D", 0x2D, "u8[3]")])
    lay.type("KiBlast", 0x34, [
        ("flags", 0, "u16"), ("blastType", 2, "s8"), ("unk3", 3, "s8"), ("damage", 4, "s32"), ("guardDamage", 8, "s32"),
        ("ki", 0x0C, "s32"), ("speed", 0x10, "f32"), ("turn", 0x14, "f32"), ("time", 0x18, "f32"), ("push", 0x1C, "f32"),
        ("guardPush", 0x20, "f32"), ("react", 0x24, "s8"), ("reactS800", 0x25, "s8"), ("reactS4000", 0x26, "s8"),
        ("reactS200", 0x27, "s8"), ("unk28", 0x28, "f32"), ("unk2C", 0x2C, "f32"), ("hits", 0x30, "s8"),
        ("unk31", 0x31, "s8"), ("unk32", 0x32, "u8[2]")])
    lay.type("MoveParam", 0xA0, [
        ("speed", 0, "f32[20]"), ("accel", 0x50, "f32[2]"), ("angle58", 0x58, "f32"), ("height", 0x5C, "f32[2]"),
        ("kiCost", 0x64, "s32[4]"), ("turnRate", 0x74, "f32[3]"), ("turnAccel", 0x80, "f32[4]"), ("turnMax", 0x90, "f32[4]")])
    lay.type("SuperParam", 0x244, [
        ("flagsA", 0, "u32[3]"), ("flags", 0x0C, "u32[3]"), ("id", 0x18, "s16[3]"), ("unk1E", 0x1E, "s16[3]"),
        ("shotTime", 0x24, "f32[3]"), ("unk30", 0x30, "u8[36]"), ("shotSpeed", 0x54, "f32[3]"), ("shotTurn", 0x60, "f32[3]"),
        ("shotUnk6C", 0x6C, "f32[3]"), ("unk78", 0x78, "u8[24]"), ("unk90", 0x90, "s8[3]"), ("shots", 0x93, "s8[3]"),
        ("hitsB", 0x96, "s8[3]"), ("unk99", 0x99, "s8[3]"), ("hitsC", 0x9C, "s8[3]"), ("unk9F", 0x9F, "u8[156]"),
        ("hitDirKind", 0x13B, "s8[3]"), ("superType", 0x13E, "s8[3]"), ("react", 0x141, "s8[3]"), ("reactAlt", 0x144, "s8[3]"),
        ("reactS800", 0x147, "s8[3]"), ("reactAltS800", 0x14A, "s8[3]"), ("unk14D", 0x14D, "s8[3]"),
        ("guardKind", 0x150, "s8[3]"), ("hitSound", 0x153, "s8[6]"), ("steps", 0x159, "s8[3]"), ("unk15C", 0x15C, "u8[9]"),
        ("unk165", 0x165, "s8[3]"), ("lastStep", 0x168, "s8[3]"), ("unk16B", 0x16B, "u8"), ("push", 0x16C, "f32[3]"),
        ("guardPush", 0x178, "f32[3]"), ("damage", 0x184, "s32[3]"), ("guardDamage", 0x190, "s32[3]"), ("ki", 0x19C, "s32[3]"),
        ("recoil", 0x1A8, "s32[3]"), ("drain", 0x1B4, "s32[3]"), ("cooldown", 0x1C0, "f32[3]"),
        ("chargeLimit", 0x1CC, "f32[3]"), ("chargeScale", 0x1D8, "f32[3]"), ("throwAngleA", 0x1E4, "f32[3]"),
        ("throwAngleB", 0x1F0, "f32[3]"), ("rushSpeed", 0x1FC, "f32[3]"), ("throwChara", 0x208, "s16[3]"),
        ("throwCharaAlt", 0x20E, "s16[3]"), ("unk214", 0x214, "u8[12]"), ("unk220", 0x220, "s8[3]"),
        ("unk223", 0x223, "s8[3]"), ("unk226", 0x226, "s8[3]"), ("promptRow", 0x229, "s8[3]"), ("time22C", 0x22C, "f32[3]"),
        ("time238", 0x238, "f32[3]")])
    lay.type("SkillParam", 0xC8, [
        ("flagsA", 0, "u32[2]"), ("flags", 8, "u32[2]"), ("id", 0x10, "s16[2]"), ("unk14", 0x14, "s16[2]"),
        ("shotTime", 0x18, "f32[2]"), ("shotSpeed", 0x20, "f32[2]"), ("shotTurn", 0x28, "f32[2]"),
        ("shotUnk30", 0x30, "f32[2]"), ("unk38", 0x38, "s8[2]"), ("shots", 0x3A, "s8[2]"), ("hitsB", 0x3C, "s8[2]"),
        ("unk3E", 0x3E, "s8[2]"), ("hitsC", 0x40, "s8[2]"), ("unk42", 0x42, "u8[32]"), ("hitDirKind", 0x62, "s8[2]"),
        ("sequence", 0x64, "s8[2]"), ("statKind", 0x66, "s8[2]"), ("react", 0x68, "s8[2]"), ("reactS800", 0x6A, "s8[2]"),
        ("hitSound", 0x6C, "s8[2]"), ("unk6E", 0x6E, "u8[6]"), ("push", 0x74, "f32[2]"), ("guardPush", 0x7C, "f32[2]"),
        ("damage", 0x84, "s32[2]"), ("guardDamage", 0x8C, "s32[2]"), ("stocks", 0x94, "s8[2]"),
        ("statLevel0", 0x96, "s8[2]"), ("statLevel2", 0x98, "s8[2]"), ("statLevel1", 0x9A, "s8[2]"),
        ("statLevel3", 0x9C, "s8[2]"), ("unk9E", 0x9E, "s8[2]"), ("time", 0xA0, "f32[2]"), ("healthPct", 0xA8, "s32[2]"),
        ("ki", 0xB0, "s32[2]"), ("valF", 0xB8, "s32[2]"), ("valE", 0xC0, "s32[2]")])
    lay.type("ChainParam", 0x350, [
        ("nodeA", 0, "u8[16]"), ("axisX", 0x10, "f32[16]"), ("axisY", 0x50, "f32[16]"), ("axisZ", 0x90, "f32[16]"),
        ("hinge", 0xD0, "u8[16]"), ("hingeOfs", 0xE0, "s16[16]"), ("hingeLimit", 0x100, "u8[16]"),
        ("inertia", 0x110, "f32[16]"), ("damping", 0x150, "f32[16]"), ("speed", 0x190, "s8[16]"), ("phase", 0x1A0, "s16[16]"),
        ("swingMax", 0x1C0, "u8[16]"), ("swing", 0x1D0, "f32[16]"), ("follow", 0x210, "f32[16]"), ("limit", 0x250, "u8[16]"),
        ("rest", 0x260, "f32[16]"), ("gravity", 0x2A0, "f32[16]"), ("stiff", 0x2E0, "u8[16]"), ("nodeB", 0x2F0, "u8[8]"),
        ("yaw", 0x2F8, "s8[8]"), ("gravityB", 0x300, "f32[8]"), ("swingB", 0x320, "f32[8]"), ("yawMax", 0x340, "s8[8]"),
        ("yawMin", 0x348, "s8[8]")])
    lay.type("BodyData", 0x10, [("unk00", 0, "s32"), ("unk04", 4, "f32"), ("unk08", 8, "u32[2]")])
    lay.type("BodyPart", 0x1D0, [
        ("flags", 0, "u32"), ("node", 4, "s32"), ("bonePtr", 8, "u32"), ("partPtr", 0x0C, "u32"),
        ("offset", 0x10, "f32[4]"), ("center", 0x20, "f32[4]"), ("unk30", 0x30, "f32[4]"),
        ("boxCenter", 0x40, "f32[4]"), ("boxUnk10", 0x50, "f32[4]"), ("boxAxis", 0x60, "f32[12]"),
        ("boxMtx", 0x90, "f32[16]"), ("boxLocal", 0xD0, "f32[32]"), ("boxWorld", 0x150, "f32[32]")])
    lay.type("AiParam", 0x1F0, [("levelLow", 0, "s8[248]"), ("levelHigh", 0xF8, "s8[248]")])
    lay.type("CameraCut", 0x24, [
        ("nodeA", 0, "u8"), ("nodeA2", 1, "u8"), ("nodeC", 2, "u8"), ("nodeC2", 3, "u8"), ("yaw", 4, "f32"),
        ("yawDelta", 8, "f32"), ("pitch", 0x0C, "f32"), ("pitchDelta", 0x10, "f32"), ("dist", 0x14, "f32"),
        ("distDelta", 0x18, "f32"), ("seconds", 0x1C, "f32"), ("cutFlags", 0x20, "u16"), ("flags", 0x22, "u16")])


# file entry -> (type, records: None = one, or the record size for as many as fit)
TABLES = {0x11: ("FighterParam", None), 0x12: ("AttackRecord", 0x30), 0x13: ("KiBlast", 13), 0x14: ("MoveParam", None),
          0x17: ("SuperParam", None), 0x18: ("SkillParam", None), 0x16: ("ChainParam", None),
          0x1A: ("AiParam", None), 0x1B: ("CameraCut", 0x24)}


def convert(d, skel):
    """d: a fighter model file (pmdlconv.py converted its model). -> model-tables.json text or None."""
    if len(d) < 8:
        return None
    n = u32(d, 0)
    if n < 0x1C or 4 + 4 * (n + 1) > len(d):
        return None
    offs = [u32(d, 4 + 4 * i) for i in range(n + 1)]
    lay = layout.Layout(0)
    types(lay)
    for e, (t, count) in TABLES.items():
        a, b = offs[e], offs[e + 1]
        size = lay.types[t]["size"]
        if t in ("AttackRecord", "CameraCut"):
            k = (b - a) // size
        elif t == "KiBlast":
            k = 13 if b - a >= 13 * size else 0
        else:
            k = 1 if b - a >= size else 0
        for i in range(k):
            lay.read(d, t, a + i * size)
    # entry 1: the body collision data (BobjBodyData, include/battle/bobj_a.h): a header, then the body parts up to
    # the one with flag 1 (last)
    a, b = offs[1], offs[2]
    if b - a >= 0x10 + 0x1D0:
        lay.read(d, "BodyData", a)
        p = a + 0x10
        while p + 0x1D0 <= b:
            lay.read(d, "BodyPart", p)
            if u32(d, p) & 1:
                break
            p += 0x1D0
    if not lay.records:
        return None
    layout.finish(lay, d, skel)
    return lay.text("the game tables of the fighter (tools/assets/fightertables.py): parameters, attack records, ki "
                    "blasts, movement, techniques (Blast 2, Ultimate), skills (Blast 1), secondary motion; names after "
                    "include/battle/btl_tech_b.h, src/battle/btl_char_hit.c, include/battle/bobj_b.h")
