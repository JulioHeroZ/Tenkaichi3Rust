"""The collision of a stage (where fighters stand, what stops them and the camera) to glTF, rebuildable to the exact
bytes. The collision is in entry 0 of a stage file, after the drawn model (stageconv.py):

* Header +0x38 / +0x3C: the zones (StgColZone, include/battle/eft_det_b.h, 0x40 bytes): mark, mesh of the zone's
  static geometry, neighbour count and list, a word, record count and list (StgColRec: broken, object, mesh, a
  word: the collision of a destructible object, whole or broken), a word, then the centre and the half size.
* A mesh (ColMesh, include/battle/col_a.h): node count, polygon count, 0, then the offsets of its nodes, polygons
  and vertices. Offsets count words from the stage's start (BtlStage_Relocate / ColMesh_SetBase).
  Vertex: x y z w. Polygon: flags, three vertex indices, the plane (normal, distance). Node: a box (min, max),
  then left and right (right = -1: a leaf, left = its polygon).

collision.gltf: one mesh per collision mesh (POSITION and the triangles, to look at or edit in any 3D program),
and in its extras what the game needs as it is: per mesh the header, the polygon flags and accessors for the w
of the vertices, the planes, the node boxes and links; in the scene's extras the zones. A mesh header's offsets and
the zones' lists are written back where they were.
"""
import struct, json

def u32(d, o): return struct.unpack_from("<I", d, o)[0]
def s32(d, o): return struct.unpack_from("<i", d, o)[0]

def stage_base(d):
    if len(d) < 16:
        return None
    n = u32(d, 0)
    if not 1 <= n <= 4096 or (n + 1) * 4 > len(d):
        return None
    b = u32(d, 4) & ~3
    if b + 0x60 > len(d) or u32(d, b + 0x18) != 7:
        return None
    return b

def read(d):
    """-> (base, zones_at, zones, meshes) or None. zones: dicts; meshes: {header offset: dict}."""
    B = stage_base(d)
    if B is None:
        return None
    zc, zw = u32(d, B + 0x38), u32(d, B + 0x3C)
    if zc == 0 or zc > 4096 or zw == 0:
        return None
    Z = B + zw * 4
    if Z + zc * 0x40 > len(d):
        return None
    ptr = lambda w: B + w * 4
    meshes = {}
    def mesh(w):
        if w == 0:
            return None
        M = ptr(w)
        if M + 0x18 > len(d):
            raise ValueError("mesh past the file")
        if M in meshes:
            return M
        h = [s32(d, M + 4 * k) for k in range(6)]
        m = {"at": M, "header": h}
        nodes, polys = h[0], h[1]
        if nodes > 0:
            if h[2] != 0 or polys <= 0 or min(h[3:]) < 0:
                raise ValueError("mesh header %r" % h)
            no, po, vo = ptr(h[3]), ptr(h[4]), ptr(h[5])
            if no + nodes * 32 > len(d) or po + polys * 32 > len(d):
                raise ValueError("mesh past the file")
            vmax = -1
            for p in range(polys):
                for k in range(3):
                    v = s32(d, po + p * 32 + 4 + 4 * k)
                    if v < 0:
                        raise ValueError("vertex index")
                    vmax = max(vmax, v)
            for i in range(nodes):
                l, r = s32(d, no + i * 32 + 24), s32(d, no + i * 32 + 28)
                if r == -1 and l >= polys or r != -1 and not (0 <= l < nodes and 0 <= r < nodes):
                    raise ValueError("node links")
            nv = vmax + 1
            if vo + nv * 16 > len(d):
                raise ValueError("vertices past the file")
            m.update(nodes_at=no, polys_at=po, verts_at=vo, nodes=nodes, polys=polys, verts=nv)
        meshes[M] = m
        return M
    zones = []
    for z in range(zc):
        zo = Z + z * 0x40
        w = [u32(d, zo + 4 * k) for k in range(8)]
        zone = {"mark": w[0], "mesh": mesh(w[1]), "unk10": w[4], "unk1C": w[7]}
        if w[3]:
            na = ptr(w[3])
            zone["near_at"] = na
            zone["near"] = [s32(d, na + 4 * i) for i in range(w[2])]
        elif w[2]:
            raise ValueError("neighbours without a list")
        if w[6]:
            ra = ptr(w[6])
            zone["recs_at"] = ra
            recs = []
            for r in range(w[5]):
                b, o, mw, x = struct.unpack_from("<4I", d, ra + r * 16)
                recs.append([b, o, mesh(mw), x])
            zone["recs"] = recs
        elif w[5]:
            raise ValueError("records without a list")
        zones.append(zone)
    return B, Z, zones, meshes

def ranges(B, Z, zones, meshes):
    """The bytes the collision takes: [(offset, length)]."""
    out = [(Z, len(zones) * 0x40)]
    for z in zones:
        if "near_at" in z:
            out.append((z["near_at"], 4 * len(z["near"])))
        if "recs_at" in z:
            out.append((z["recs_at"], 16 * len(z["recs"])))
    for m in meshes.values():
        out.append((m["at"], 0x18))
        if "nodes" in m:
            out += [(m["nodes_at"], m["nodes"] * 32), (m["polys_at"], m["polys"] * 32), (m["verts_at"], m["verts"] * 16)]
    return out

def convert(d, skel):
    """-> (collision.gltf text, collision.bin) or None; the collision's bytes are zeroed in skel."""
    r = read(d)
    if r is None:
        return None
    B, Z, zones, meshes = r
    binb = bytearray(); views = []; accs = []
    def add(data, ctype, count, typ, target=None, mm=None):
        while len(binb) % 4:
            binb.append(0)
        v = {"buffer": 0, "byteOffset": len(binb), "byteLength": len(data)}
        if target:
            v["target"] = target
        views.append(v); binb.extend(data)
        a = {"bufferView": len(views) - 1, "componentType": ctype, "count": count, "type": typ}
        if mm:
            a["min"], a["max"] = mm
        accs.append(a)
        return len(accs) - 1
    order = sorted(meshes)
    index = {M: i for i, M in enumerate(order)}
    gmeshes, nodes = [], []
    f32 = lambda w: struct.unpack("<f", struct.pack("<I", w))[0]
    for M in order:
        m = meshes[M]
        ex = {"at": M, "header": m["header"]}
        prim = {"attributes": {}, "mode": 4}
        if "nodes" in m:
            vo, po, no = m["verts_at"], m["polys_at"], m["nodes_at"]
            vw = [struct.unpack_from("<4I", d, vo + i * 16) for i in range(m["verts"])]
            P = [[f32(v[k]) for k in range(3)] for v in vw]
            mm = ([min(p[k] for p in P) for k in range(3)], [max(p[k] for p in P) for k in range(3)])
            prim["attributes"]["POSITION"] = add(b"".join(struct.pack("<3I", *v[:3]) for v in vw), 5126, len(vw), "VEC3", 34962, mm)
            pw = [struct.unpack_from("<8I", d, po + p * 32) for p in range(m["polys"])]
            prim["indices"] = add(b"".join(struct.pack("<3I", *p[1:4]) for p in pw), 5125, 3 * len(pw), "SCALAR", 34963)
            ex["flags"] = [p[0] for p in pw]
            ex["w"] = add(b"".join(struct.pack("<I", v[3]) for v in vw), 5126, len(vw), "SCALAR")
            ex["planes"] = add(b"".join(struct.pack("<4I", *p[4:8]) for p in pw), 5126, len(pw), "VEC4")
            nw = [struct.unpack_from("<8I", d, no + i * 32) for i in range(m["nodes"])]
            ex["boxes"] = add(b"".join(struct.pack("<6I", *n[:6]) for n in nw), 5126, 2 * len(nw), "VEC3")
            ex["links"] = add(b"".join(struct.pack("<2I", *n[6:8]) for n in nw), 5125, 2 * len(nw), "SCALAR")
        gm = {"name": "collision_%03d" % index[M], "extras": ex}
        if prim["attributes"]:
            gm["primitives"] = [prim]
            nodes.append({"name": gm["name"], "mesh": len(gmeshes)})
        gmeshes.append(gm)
    # meshes without polygons have no primitive (a glTF mesh must have one): kept in the scene extras
    empty = [g for g in gmeshes if "primitives" not in g]
    drawn = [g for g in gmeshes if "primitives" in g]
    zj = []
    centres = []
    for z in zones:
        e = {"mark": z["mark"], "mesh": index[z["mesh"]] if z["mesh"] is not None else None, "unk10": z["unk10"], "unk1C": z["unk1C"]}
        if "near_at" in z:
            e["near_at"], e["near"] = z["near_at"], z["near"]
        if "recs_at" in z:
            e["recs_at"] = z["recs_at"]
            e["recs"] = [[b, o, index[mi] if mi is not None else None, x] for b, o, mi, x in z["recs"]]
        zj.append(e)
    zo = Z
    cw = b"".join(d[zo + i * 0x40 + 0x20: zo + i * 0x40 + 0x40] for i in range(len(zones)))
    zone_box = add(cw, 5126, 2 * len(zones), "VEC4")
    # mesh numbers: the glTF meshes with primitives come first, in the order of `order`; `mesh_order` maps
    gl_index = {}
    for g in drawn + empty:
        gl_index[int(g["name"].split("_")[1])] = len(gl_index)
    for e in zj:
        if e["mesh"] is not None:
            e["mesh"] = gl_index[e["mesh"]]
        for rec in e.get("recs", []):
            if rec[2] is not None:
                rec[2] = gl_index[rec[2]]
    nodes = [{"name": g["name"], "mesh": i} for i, g in enumerate(drawn)]
    g = {"asset": {"version": "2.0", "generator": "Tenkaichi3Rust tools/assets/colconv.py"},
         "scene": 0,
         "scenes": [{"nodes": [len(nodes)], "extras": {"base": B, "zones_at": Z, "zone_boxes": zone_box, "zones": zj,
                                                       "empty_meshes": [g["extras"] for g in empty]}}],
         "nodes": nodes + [{"name": "root", "rotation": [1, 0, 0, 0], "scale": [0.001, 0.001, 0.001], "children": list(range(len(nodes)))}],
         "meshes": drawn,
         "accessors": accs, "bufferViews": views,
         "buffers": [{"uri": "collision.bin", "byteLength": len(binb)}]}
    for o, l in ranges(B, Z, zones, meshes):
        skel[o:o + l] = bytes(l)
    return json.dumps(g, separators=(",", ":")), bytes(binb)

def apply(buf, text, binb):
    """Writes the collision of collision.gltf / collision.bin back into buf (the rebuild of host/assets.rs)."""
    g = json.loads(text)
    def words(i):
        a = g["accessors"][i]; v = g["bufferViews"][a["bufferView"]]
        comps = {"SCALAR": 1, "VEC2": 2, "VEC3": 3, "VEC4": 4}[a["type"]]
        o = v.get("byteOffset", 0) + a.get("byteOffset", 0)
        return list(struct.unpack_from("<%dI" % (a["count"] * comps), binb, o))
    sx = g["scenes"][0]["extras"]
    B, Z = sx["base"], sx["zones_at"]
    allm = [m["extras"] for m in g["meshes"]] + sx["empty_meshes"]
    put = lambda o, ws: struct.pack_into("<%dI" % len(ws), buf, o, *ws)
    for mi, ex in enumerate(allm):
        h = ex["header"]
        put(ex["at"], [x & 0xFFFFFFFF for x in h])
        if mi >= len(g["meshes"]):
            continue
        prim = g["meshes"][mi]["primitives"][0]
        pos, w = words(prim["attributes"]["POSITION"]), words(ex["w"])
        vo = B + h[5] * 4
        for i in range(len(w)):
            put(vo + i * 16, pos[i * 3:i * 3 + 3] + [w[i]])
        idx, pl, fl = words(prim["indices"]), words(ex["planes"]), ex["flags"]
        po = B + h[4] * 4
        if len(fl) != h[1]:
            raise ValueError("polygon count")
        for p in range(h[1]):
            put(po + p * 32, [fl[p]] + idx[p * 3:p * 3 + 3] + pl[p * 4:p * 4 + 4])
        bx, ln = words(ex["boxes"]), words(ex["links"])
        no = B + h[3] * 4
        for i in range(h[0]):
            put(no + i * 32, bx[i * 6:i * 6 + 6] + ln[i * 2:i * 2 + 2])
    boxes = words(sx["zone_boxes"])
    mesh_at = lambda k: 0 if k is None else (allm[k]["at"] - B) // 4
    for i, z in enumerate(sx["zones"]):
        near, recs = z.get("near", []), z.get("recs", [])
        put(Z + i * 0x40, [z["mark"], mesh_at(z["mesh"]), len(near), (z["near_at"] - B) // 4 if "near_at" in z else 0,
                           z["unk10"], len(recs), (z["recs_at"] - B) // 4 if "recs_at" in z else 0, z["unk1C"]] + boxes[i * 8:i * 8 + 8])
        if near:
            put(z["near_at"], [x & 0xFFFFFFFF for x in near])
        for r, (b, o, mk, x) in enumerate(recs):
            put(z["recs_at"] + r * 16, [b, o, mesh_at(mk), x])
