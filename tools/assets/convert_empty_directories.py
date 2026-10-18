"""Export empty resource directories as semantic JSON, with exact roundtrip checks."""
import json
from pathlib import Path
import struct
import asset_dirs  # where each file folder is (numbered or organized)


def main():
    root = Path(__file__).resolve().parents[2]
    assets = asset_dirs.assets_root()
    count = 0
    for source in sorted((assets / '_legacy/raw/pzs3us1').glob('*.bin')):
        target = asset_dirs.find(assets, int(source.stem))
        # files_extract.py gives such a file as its directory record alone (file-tables.json, skeleton.txt)
        made = {p.name for p in target.iterdir()} if target.is_dir() else set()
        if made - {'file-tables.json', 'skeleton.txt'}:
            continue
        data = source.read_bytes()
        if len(data) < 8:
            continue
        entries = struct.unpack_from('<I', data)[0]
        if entries > 65536:
            continue
        size = ((entries + 2) * 4 + 63) // 64 * 64
        expected = struct.pack('<I', entries) + struct.pack('<I', size) * (entries + 1)
        expected += bytes(size - len(expected))
        if expected != data:
            continue
        target.mkdir(parents=True, exist_ok=True)
        (target / 'empty-directory.json').write_text(
            json.dumps({'type': 'empty-resource-directory', 'entries': entries}, indent=2) + '\n',
            encoding='utf-8')
        for name in made:
            (target / name).unlink()  # the same bytes, now given by empty-directory.json
        count += 1
    print(f'{count} empty directories converted to JSON; every reconstructed byte verified.')


if __name__ == '__main__':
    main()
