"""Convert the common roster/item tables to named JSON fields (battle/view_b.h).

Unknown fields are explicitly retained; no source blob or opaque base64 payload.
"""
from pathlib import Path
import json
import struct
import asset_dirs  # where each file folder is (numbered or organized)


def main():
    root = Path(__file__).resolve().parents[2]
    source = asset_dirs.assets_root() / '_legacy/raw/pzs3us1/00003.bin'
    data = source.read_bytes()
    characters = []
    items = []
    for i in range(161):
        at = 64 + i * 60
        ai, unknown, flags, costumes, cost, level = struct.unpack_from('<ii4H', data, at)
        characters.append(dict(aiType=ai, unknown4=unknown, flags=flags, costumes=costumes,
            cost=cost, baseLevel=level, experience=list(struct.unpack_from('<7i', data, at+16)),
            formLinks=list(data[at+44:at+48]), targetY=struct.unpack_from('<f', data, at+48)[0],
            cameraDistance=struct.unpack_from('<f', data, at+52)[0], unknown56=list(data[at+56:at+60])))
    for i in range(350):
        at = 9728 + i * 40
        items.append(dict(type=data[at], unknown1=list(data[at+1:at+3]), slots=data[at+3],
            unknown4=list(data[at+4:at+12]), stats=list(struct.unpack_from('<4h', data, at+12)),
            flags=struct.unpack_from('<i', data, at+20)[0], abilities=list(struct.unpack_from('<4i', data, at+24))))
    doc = dict(version=1, characters=characters, items=items)
    # Independent roundtrip using only the JSON document and the schema constants.
    doc = json.loads(json.dumps(doc, allow_nan=False))
    out = bytearray(23744)
    struct.pack_into('<4I', out, 0, 2, 64, 9728, 23744)
    for i, c in enumerate(doc['characters']):
        at = 64 + i * 60
        struct.pack_into('<ii4H7i4B2f4B', out, at, c['aiType'], c['unknown4'], c['flags'],
            c['costumes'], c['cost'], c['baseLevel'], *c['experience'], *c['formLinks'],
            c['targetY'], c['cameraDistance'], *c['unknown56'])
    for i, c in enumerate(doc['items']):
        struct.pack_into('<4B8B4hi4i', out, 9728+i*40, c['type'], *c['unknown1'], c['slots'],
            *c['unknown4'], *c['stats'], c['flags'], *c['abilities'])
    if out != data:
        raise ValueError('Roster/item schema does not reproduce the source exactly')
    target = asset_dirs.find(asset_dirs.assets_root(), 3) / 'roster-items.json'
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(json.dumps(doc, indent=2, allow_nan=False)+'\n', encoding='utf-8')
    print('161 characters and 350 items converted; all 23744 bytes verified.')


if __name__ == '__main__':
    main()
