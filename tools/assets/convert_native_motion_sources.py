"""Replace understood motion entry templates with typed structural metadata.

Actual exported pose keys remain in glTF and attack events in events.json.
Non-glTF fallback tracks use explicit frames, positions and float quaternions;
no opaque blobs, packed quaternion words or arbitrary leftover bytes are copied.
Unknown residue causes that motion to stay pending instead of being discarded.
"""
import argparse
import json
from pathlib import Path
import struct
import sys

sys.path.insert(0,str(Path(__file__).resolve().parent))
import motion
import asset_dirs  # where each file folder is (numbered or organized)


def describe(data, gl_nodes):
    reconstructed=bytearray(len(data))
    reconstructed[:6]=data[:6]
    header_byte,event_count,length,header_word=struct.unpack_from('<BBHH',data)
    offsets=list(struct.unpack_from('<71H',data,6))
    struct.pack_into('<71H',reconstructed,6,*offsets)
    event_volumes=[]
    for i in range(event_count):
        shape=struct.unpack_from('<H',data,148+i*16+10)[0]
        struct.pack_into('<H',reconstructed,148+i*16+10,shape)
        event_volumes.append(shape)
    tracks=[];fallback=[];covered=bytearray(len(data))
    covered[:148+event_count*16]=bytes([1])*(148+event_count*16)
    for node,at,flags,count,keys in motion.tracks(data):
        struct.pack_into('<HH',reconstructed,at,flags,count)
        tracks.append([node,at,flags,count])
        end=at+4+count*(10 if flags&1 else 24)
        covered[at:end]=bytes([1])*(end-at)
        if node not in gl_nodes:
            rows=[]
            for frame,pos,packed in keys:
                quaternion=motion.unpack(packed)
                if quaternion is None:
                    raise ValueError('invalid rotation')
                quaternion=[float(v) for v in quaternion]
                if motion.pack(quaternion)!=packed:
                    raise ValueError('fallback rotation cannot be represented exactly')
                rows.append(dict(frame=frame,rotation=quaternion,
                    **({'position':list(pos)} if pos is not None else {})))
            fallback.append(dict(node=node,keys=rows))
            motion.write_keys(reconstructed,at,[r['frame'] for r in rows],
                [r['rotation'] for r in rows],
                [[struct.unpack('<I',struct.pack('<f',v))[0] for v in r['position']] for r in rows] if not flags&1 else None)
    # A stock motion contains old, unbound tracks no node references. Preserve
    # them as typed keys too, rather than dropping them or storing opaque bytes.
    unbound=[];at=148+event_count*16
    while at+4<=len(data):
        if any(covered[at:at+4]) or data[at:at+4]==bytes(4):
            at+=4;continue
        flags,count=struct.unpack_from('<HH',data,at)
        end=at+4+count*(10 if flags&1 else 24)
        if flags not in (0,1) or not count or end>len(data) or any(covered[at:end]):
            raise ValueError('unclassified bytes remain in the motion')
        view=bytearray(data)
        struct.pack_into('<71H',view,6,*([at//4]+[0]*70))
        parsed=motion.tracks(view)[0]
        keys=[]
        for frame,pos,packed in parsed[4]:
            quaternion=[float(v) for v in motion.unpack(packed)]
            if motion.pack(quaternion)!=packed:
                raise ValueError('unbound rotation cannot be represented exactly')
            keys.append(dict(frame=frame,rotation=quaternion,**({'position':list(pos)} if pos is not None else {})))
        struct.pack_into('<HH',reconstructed,at,flags,count)
        motion.write_keys(reconstructed,at,[r['frame'] for r in keys],[r['rotation'] for r in keys],
            [[struct.unpack('<I',struct.pack('<f',v))[0] for v in r['position']] for r in keys] if not flags&1 else None)
        unbound.append(dict(byteOffset=at,flags=flags,keys=keys))
        at=(end+3)//4*4
    if reconstructed!=data:
        raise ValueError('unclassified bytes remain in the motion')
    return dict(bytes=len(data),headerByte=header_byte,headerWord=header_word,
        templateLength=length,eventVolumeWords=event_volumes,tracks=tracks,fallback=fallback,
        **({'unboundTracks':unbound} if unbound else {}))


def write_whole(target,text):
    # whole or not at all: an install stopped here leaves the previous file
    tmp=target.with_name(target.name+'.tmp')
    tmp.write_text(text,encoding='utf-8')
    tmp.replace(target)


def main():
    root=Path(__file__).resolve().parents[2]
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--bank',help='one bank resource number, otherwise all banks')
    args=parser.parse_args()
    banks=motions=pending=empty_total=0;reasons={};pending_entries=[]
    paths=[asset_dirs.find(asset_dirs.assets_root(),int(args.bank))/'bank.txt'] if args.bank else [d/'bank.txt' for _,d in asset_dirs.having(asset_dirs.assets_root(),'bank.txt')]
    for path in paths:
        kinds=[line.split() for line in path.read_text().splitlines() if line and not line.startswith(('#','entries'))]
        flagged=[int(row[0]) for row in kinds if row[1]=='motion']
        if not flagged:continue
        gl=json.loads((path.parent/'animations.gltf').read_text())
        per={}
        for animation in gl.get('animations',[]):
            per[int(animation['name'].removeprefix('motion_'))]={int(gl['nodes'][c['target']['node']]['name'].removeprefix('node_')) for c in animation['channels']}
        empty=[int(row[0]) for row in kinds if row[1]!='motion' and (path.parent/f'entry_{int(row[0]):03}.bin').is_file() and (path.parent/f'entry_{int(row[0]):03}.bin').stat().st_size==0]
        document=dict(version=1,entryCount=len(kinds),motions=[],emptyEntries=empty)
        for entry in flagged:
            if not (path.parent/f'entry_{entry:03}.bin').is_file():
                continue  # files_extract.py already described it in motion-sources.json, or kept it in entry_NNN/
            try:
                data=(path.parent/f'entry_{entry:03}.bin').read_bytes()
                row=describe(data,per.get(entry,set()))
                row['entry']=entry
                # Reject NaN/Infinity instead of exporting non-standard JSON.
                json.dumps(row,allow_nan=False)
                document['motions'].append(row)
            except (ValueError,struct.error,IndexError,OverflowError) as error:
                pending+=1;key=str(error);reasons[key]=reasons.get(key,0)+1
                pending_entries.append(dict(bank=path.parent.name[:5],entry=entry,reason=key))
        target=path.parent/'motion-sources.json'
        if not target.exists():
            write_whole(target,json.dumps(document,separators=(',',':'),allow_nan=False)+'\n')
        else:
            previous=json.loads(target.read_text())
            existing={row['entry'] for row in previous['motions']}
            previous['motions'].extend(row for row in document['motions'] if row['entry'] not in existing)
            previous.setdefault('emptyEntries',empty)
            previous.setdefault('entryCount',len(kinds))
            write_whole(target,json.dumps(previous,separators=(',',':'),allow_nan=False)+'\n')
        motions+=len(document['motions']);banks+=1;empty_total+=len(empty)
        if banks%20==0:print(f'{banks} banks, {motions} motion sources...',flush=True)
    result=dict(scope=args.bank or 'all',banks=banks,motion_sources=motions,empty_entries=empty_total,
        pending_flagged_motions=pending,pending_reasons=reasons,pending_entries=pending_entries)
    audit=root/'build/audits/native-motions';audit.mkdir(parents=True,exist_ok=True)
    (audit/'conversion.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8')
    print(json.dumps(result,indent=2))


if __name__=='__main__':
    main()
