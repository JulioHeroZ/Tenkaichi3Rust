"""Export the remaining EVPp point groups to editable numeric JSON.

The second vector and attribute meanings remain unconfirmed. They are preserved,
not interpreted as gameplay properties. No opaque byte payload is retained.
"""
import json
from pathlib import Path
import struct
import asset_dirs  # where each file folder is (numbered or organized)


def main():
    root = Path(__file__).resolve().parents[2]
    for index in range(442, 446):
        source = asset_dirs.assets_root() / f'_legacy/raw/pzs3us1/{index:05}.bin'
        data = source.read_bytes()
        groups = []
        assert struct.unpack_from('<I', data)[0] == 25
        payload = data[128:]
        magic, version, count, header = struct.unpack_from('<4I', payload)
        assert magic == int.from_bytes(b'EVPp', 'little') and header == 32
        for i in range(count):
            n, at, attribute, reserved = struct.unpack_from('<4I', payload, 32+i*16)
            assert reserved == 0
            points = []
            for j in range(n):
                row = at+j*48
                points.append(dict(position=list(struct.unpack_from('<4f', payload, row)),
                    vector1=list(struct.unpack_from('<4f', payload, row+16)),
                    attributes=list(struct.unpack_from('<4I', payload, row+32))))
            groups.append(dict(attribute=attribute, points=points))
        doc = json.loads(json.dumps(dict(version=version, groups=groups), allow_nan=False))
        # Rebuild the container from the schema, verifying even the padding and signed zero.
        body = bytearray(32+16*len(groups))
        struct.pack_into('<4I', body, 0, magic, version, len(groups), 32)
        for i, group in enumerate(doc['groups']):
            struct.pack_into('<4I', body, 32+i*16, len(group['points']), len(body), group['attribute'], 0)
            for p in group['points']:
                body.extend(struct.pack('<8f4I', *p['position'], *p['vector1'], *p['attributes']))
        out = bytearray(128)
        struct.pack_into('<I', out, 0, 25)
        struct.pack_into('<26I', out, 4, *([128]*24+[128+len(body)]*2))
        out.extend(body)
        assert out == data, f'{index}: point schema differs from source'
        target = asset_dirs.find(asset_dirs.assets_root(), index) / 'point-groups.json'
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(json.dumps(doc, indent=2, allow_nan=False)+'\n', encoding='utf-8')
        print(f'{index}: {len(groups)} point groups exported and verified.')


if __name__ == '__main__':
    main()
