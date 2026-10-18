"""The tables of a stage (entry 0 of a stage file, what BtlStage_Relocate in src/sys/gfxm_e_d.c relocates) as typed
records (layout.py) in stage-layout.json: the header, the draw groups -> materials -> meshes (whose chains are in
stage-chains.json, stagechain.py), the nodes, the part groups -> parts -> mesh lists, the objects, the octree and
the timers. Pointers are file offsets. Names follow include/sys/gfxm_e.h; what is not known keeps unkXX names.
The collision zones are in collision.gltf (colconv.py).
"""
import struct
import layout
import stageconv


def u32(d, o):
    return struct.unpack_from("<I", d, o)[0]


def u16(d, o):
    return struct.unpack_from("<H", d, o)[0]


def types(lay):
    lay.type("StageHeader", 0x60, [
        ("timerCount", 0x10, "u32"), ("timers", 0x14, "ptr"), ("drawGroupCount", 0x18, "u32"), ("drawGroups", 0x1C, "ptr"),
        ("partGroupCount", 0x20, "u32"), ("partGroups", 0x24, "ptr"), ("unk28", 0x28, "u32"), ("unk2C", 0x2C, "ptr"),
        ("nodeCount", 0x30, "u32"), ("nodes", 0x34, "ptr"), ("zoneCount", 0x38, "u32"), ("zones", 0x3C, "ptr"),
        ("unk40", 0x40, "ptr"), ("objectCount", 0x44, "u32"), ("objects", 0x48, "ptr"), ("treeSize", 0x4C, "f32"),
        ("tree", 0x50, "ptr"), ("textures", 0x54, "ptr")])
    lay.type("StageTimer", 0x10, [("flags", 0, "u16"), ("unk02", 2, "u16"), ("target", 4, "ptr"), ("time", 8, "f32"),
                                  ("period", 12, "f32")])
    lay.type("DrawGroup", 8, [("unk00", 0, "u16"), ("materialCount", 2, "u16"), ("materials", 4, "ptr")])
    lay.type("Material", 0x10, [("texture", 0, "u16"), ("meshCount", 2, "u16"), ("meshes", 4, "ptr"), ("opt", 8, "ptr")])
    lay.type("Mesh", 0x10, [("chain", 8, "ptr"), ("node", 12, "ptr")])
    lay.type("Node", 0x50, [("unk00", 0, "f32[16]"), ("keyCount", 0x40, "u32"), ("keys", 0x44, "ptr")])
    lay.type("NodeKey", 0x20, [("unk00", 0, "f32[8]")])
    lay.type("PartGroup", 8, [("count", 0, "u32"), ("parts", 4, "ptr")])
    lay.type("Part", 0x20, [("meshCount", 0, "u16"), ("unk02", 2, "u16"), ("meshes", 4, "ptr"), ("node", 8, "ptr"),
                            ("unk0C", 0x0C, "f32[5]")])
    lay.type("MeshList", 0, [], tail=("meshes", "ptr"))
    lay.type("PartList", 0, [], tail=("parts", "ptr"))
    lay.type("Object", 0x50, [("partsA", 0x0C, "ptr[2]"), ("partsB", 0x14, "ptr[2]"), ("partsC", 0x1C, "ptr[2]"),
                              ("partCount", 0x24, "u32"), ("parts", 0x28, "ptr")])
    lay.type("OctNode", 0x40, [("unk00", 0, "ptr"), ("child", 4, "ptr[8]"), ("unk24", 0x24, "f32[5]"),
                               ("count", 0x38, "u32"), ("parts", 0x3C, "ptr")])


def walk(lay, d, B):
    P = lambda o: B + u32(d, o) * 4
    nz = lambda o: u32(d, o) not in (0, 0xFFFFFFFF)
    h = lay.read(d, "StageHeader", B)
    for i in range(u32(d, B + 0x10)):
        lay.read(d, "StageTimer", P(B + 0x14) + 16 * i)

    def node(at):
        if lay.read(d, "Node", at) and nz(at + 0x44):
            n = u32(d, at + 0x40)
            for k in range(max(1, n) if n <= 4096 else 1):
                lay.read(d, "NodeKey", P(at + 0x44) + 0x20 * k)

    def mesh(at):
        lay.read(d, "Mesh", at)
        if nz(at + 12):
            node(P(at + 12))

    def part(at):
        if not lay.read(d, "Part", at):
            return
        n = u16(d, at)
        if nz(at + 4):
            lay.read(d, "MeshList", P(at + 4), n)
            for k in range(n):
                if nz(P(at + 4) + 4 * k):
                    mesh(P(P(at + 4) + 4 * k))
        if nz(at + 8):
            node(P(at + 8))

    a = P(B + 0x1C)
    for g in range(u32(d, B + 0x18)):
        lay.read(d, "DrawGroup", a + 8 * g)
        mats = P(a + 8 * g + 4)
        for m in range(u16(d, a + 8 * g + 2)):
            ma = mats + 16 * m
            lay.read(d, "Material", ma)
            for k in range(u16(d, ma + 2)):
                mesh(P(ma + 4) + 16 * k)
    b = P(B + 0x24)
    for i in range(u32(d, B + 0x20)):
        lay.read(d, "PartGroup", b + 8 * i)
        for j in range(u32(d, b + 8 * i)):
            part(P(b + 8 * i + 4) + 32 * j)
    c = P(B + 0x34)
    for i in range(u32(d, B + 0x30)):
        node(c + 0x50 * i)
    e = P(B + 0x48)
    for i in range(u32(d, B + 0x44)):
        at = e + 0x50 * i
        lay.read(d, "Object", at)
        n = u32(d, at + 0x24)
        if nz(at + 0x28):
            lay.read(d, "PartList", P(at + 0x28), n)
            for k in range(n):
                if nz(P(at + 0x28) + 4 * k):
                    part(P(P(at + 0x28) + 4 * k))
        for j in range(6):
            if nz(at + 0x0C + 4 * j):
                part(P(at + 0x0C + 4 * j))

    def octree(at, depth):
        if depth > 32 or not lay.read(d, "OctNode", at):
            return
        n = u32(d, at + 0x38)
        if nz(at + 0x3C) and n:
            lay.read(d, "PartList", P(at + 0x3C), n)
            for k in range(n):
                if nz(P(at + 0x3C) + 4 * k):
                    part(P(P(at + 0x3C) + 4 * k))
        for k in range(8):
            if nz(at + 4 + 4 * k):
                octree(P(at + 4 + 4 * k), depth + 1)

    if nz(B + 0x50):
        octree(P(B + 0x50), 0)


def convert(d, skel):
    """-> stage-layout.json text (what its records cover zeroed in skel) or None."""
    B = stageconv.stage_base(d)
    if B is None:
        return None
    lay = layout.Layout(B)
    types(lay)
    walk(lay, d, B)
    end = B + (u32(d, 8) & ~3) if u32(d, 0) >= 2 else len(d)
    for at, n in lay.cover:
        if at < B or at + n > end:
            raise ValueError("a table outside entry 0")
    layout.finish(lay, d, skel)
    return lay.text("the tables of the stage (tools/assets/stagelayout.py, tools/assets/layout.py): typed records at "
                    "their file offsets, pointers as file offsets; names after include/sys/gfxm_e.h, unkXX not known yet")
