"""Export standalone text tables; prove no unknown binary payload is discarded.

Uses extracted JSON/layout evidence only. Existing native manifests are preserved
so rerunning the converter does not overwrite translations.
"""
import json
from pathlib import Path
import struct

import native_source  # skeleton.bin, or what it held, from the typed records
import asset_dirs  # where each file folder is (numbered or organized)


def main():
    root=Path(__file__).resolve().parents[2]
    tables=strings=scripts=0
    batch=[]
    for _, d in asset_dirs.having(asset_dirs.assets_root(), 'texts.json'):
        path=d/'texts.json'
        directory=path.parent
        source=native_source.skeleton(directory)
        count=struct.unpack_from('<I',source)[0]
        entries=json.loads(path.read_text(encoding='utf-8'))['texts']
        if not 1<=count<=4096 or len(entries)!=count or any(e['entry']!=i for i,e in enumerate(entries)):
            continue
        offsets=struct.unpack_from(f'<{count+1}I',source,4)
        if offsets[0]!=((count+2)*4+63)//64*64 or offsets[-1]!=len(source):
            continue
        if any(o&63 for o in offsets) or any(offsets[i]>=offsets[i+1] for i in range(count)):
            continue
        expected=bytearray(len(source))
        struct.pack_into(f'<{count+2}I',expected,0,count,*offsets)
        for e in entries:
            expected[e['at']-2:e['at']]=b'\xff\xfe'
        if expected!=source:
            continue  # mixed graphics/data resource: never replace it with text alone
        document=dict(version=1,slots=[dict(text=e['text'],minimumBytes=offsets[i+1]-offsets[i]) for i,e in enumerate(entries)])
        # Reconstruct from semantic text/slot sizes, not copied header bytes.
        rebuilt=bytearray(offsets[0])
        positions=[]
        for slot in document['slots']:
            positions.append(len(rebuilt))
            payload=b'\xff\xfe'+slot['text'].encode('utf-16-le')+b'\0\0'
            capacity=(max(len(payload),slot['minimumBytes'])+63)//64*64
            rebuilt.extend(payload+bytes(capacity-len(payload)))
        positions.append(len(rebuilt))
        struct.pack_into(f'<{count+2}I',rebuilt,0,count,*positions)
        reference=bytearray(source)
        for e in entries:
            raw=e['text'].encode('utf-16-le')
            reference[e['at']:e['at']+len(raw)]=raw
        if rebuilt!=reference:
            raise ValueError(f'{directory}: native text schema does not reproduce reference')
        target=directory/'text-table.json'
        if not target.exists():
            target.write_text(json.dumps(document,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
        tables+=1;strings+=count
        batch.append(f'{directory.resolve()}\t{(root/"build/audits/native-text/compiled"/directory.name[:5]).with_suffix(".bin")}')
    for _, d in asset_dirs.having(asset_dirs.assets_root(), 'script.txt'):
        path=d/'script.txt'
        source=native_source.skeleton(path.parent)
        if any(source) or (path.parent/'bpe.txt').exists():
            raise ValueError(f'{path}: script is not standalone')
        scripts+=1
        batch.append(f'{path.parent.resolve()}\t{(root/"build/audits/native-text/compiled"/path.parent.name[:5]).with_suffix(".bin")}')
    audit=root/'build/audits/native-text'
    audit.mkdir(parents=True,exist_ok=True)
    (audit/'compile.tsv').write_text('\n'.join(batch)+'\n',encoding='utf-8')
    report=dict(text_tables=tables,strings=strings,standalone_scripts=scripts)
    (audit/'conversion.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
    print(json.dumps(report,indent=2))


if __name__=='__main__':
    main()
