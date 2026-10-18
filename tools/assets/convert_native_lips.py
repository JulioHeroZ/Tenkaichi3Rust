"""Export standalone lip-sync tables only when every skeleton byte is explained."""
import json
from pathlib import Path
import struct

import native_source  # skeleton.bin, or what it held, from the typed records
import asset_dirs  # where each file folder is (numbered or organized)


def assemble(doc):
    slots=doc['slots']; count=len(slots)
    out=bytearray(((count+2)*4+63)//64*64); offsets=[]
    for slot in slots:
        offsets.append(len(out))
        if slot is None: continue
        keys=slot['keys']
        size=(max(slot['minimumBytes'],16+4*len(keys))+63)//64*64
        body=bytearray(size)
        struct.pack_into('<4I',body,0,int.from_bytes(b'LIPS','little'),2,len(keys),0)
        for i,key in enumerate(keys):struct.pack_into('<2H',body,16+4*i,*key)
        out.extend(body)
    offsets.append(len(out))
    struct.pack_into(f'<{count+2}I',out,0,count,*offsets)
    return bytes(out)


def main():
    root=Path(__file__).resolve().parents[2]
    audit=root/'build/audits/native-lips'; audit.mkdir(parents=True,exist_ok=True)
    batch=[]; converted=[]; skipped=[]
    for _, d in asset_dirs.having(asset_dirs.assets_root(), 'lips.json'):
        path=d/'lips.json'
        directory=path.parent; source=native_source.skeleton(directory)
        count=struct.unpack_from('<I',source)[0]
        if not 1<=count<=4096 or 4*(count+2)>len(source):continue
        offsets=struct.unpack_from(f'<{count+1}I',source,4)
        if offsets[0]!=((count+2)*4+63)//64*64 or offsets[-1]!=len(source):continue
        if any(o%64 for o in offsets) or any(a>b for a,b in zip(offsets,offsets[1:])):continue
        lips=json.loads(path.read_text(encoding='utf-8'))['lips']
        by_id={e['entry']:e for e in lips}; slots=[]; expected=bytearray(len(source)); reference=bytearray(source)
        struct.pack_into(f'<{count+2}I',expected,0,count,*offsets)
        valid=len(by_id)==len(lips)
        for i,(a,b) in enumerate(zip(offsets,offsets[1:])):
            e=by_id.get(i)
            if a==b:
                slots.append(None); valid &= e is None; continue
            if e is None or e['at']!=a or 16+e['room']*4!=b-a:
                valid=False;break
            struct.pack_into('<4I',expected,a,int.from_bytes(b'LIPS','little'),2,0,0)
            keys=e['keys']
            struct.pack_into('<I',reference,a+8,len(keys))
            for k,key in enumerate(keys):struct.pack_into('<2H',reference,a+16+k*4,*key)
            slots.append(dict(minimumBytes=b-a,keys=keys))
        if not valid or expected!=source:
            skipped.append(directory.name[:5]);continue
        doc=dict(version=1,slots=slots)
        if assemble(doc)!=reference:raise ValueError(f'{directory}: native schema mismatch')
        target=directory/'lip-sync.json'
        if not target.exists():target.write_text(json.dumps(doc,indent=2)+'\n',encoding='utf-8')
        # Preserve edits on rerun, but require a valid document.
        assemble(json.loads(target.read_text(encoding='utf-8')))
        isolated=audit/'isolated'/directory.name[:5]; isolated.mkdir(parents=True,exist_ok=True)
        (isolated/'lip-sync.json').write_bytes(target.read_bytes())
        expected_path=audit/'reference'/f'{directory.name[:5]}.bin';expected_path.parent.mkdir(exist_ok=True)
        expected_path.write_bytes(assemble(json.loads(target.read_text(encoding='utf-8'))))
        batch.append(f'{isolated.resolve()}\t{audit.resolve()}/compiled/{directory.name[:5]}.bin')
        converted.append(dict(resource=directory.name[:5],slots=count,clips=len(lips)))
    (audit/'compile.tsv').write_text('\n'.join(batch)+'\n',encoding='utf-8')
    report=dict(converted=len(converted),clips=sum(x['clips'] for x in converted),resources=converted,skipped_mixed=skipped)
    (audit/'conversion.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
    print(json.dumps(dict(converted=report['converted'],clips=report['clips'],skipped_mixed=len(skipped)),indent=2))


if __name__=='__main__':main()
