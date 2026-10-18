"""Export PC sound-bank manifests from extracted metadata and WAVs, without ISO/raw inputs.

Preserves the current runtime's Sesq -> Setb -> Vagi effect selection. The new
runtime never reconstructs those tables, ADPCM data, or SPU transfers.
"""
import json
from pathlib import Path
import struct

import native_source  # skeleton.bin, or what it held, from the typed records
import wave
import shutil
import asset_dirs  # where each file folder is (numbered or organized)


def word(data, at):
    return struct.unpack_from('<I', data, at)[0]


def main():
    root = Path(__file__).resolve().parents[2]
    manifests = samples_total = 0
    for _, directory in sorted(asset_dirs.index(asset_dirs.assets_root()).items()):
        metadata = directory/'samples.txt'
        if not metadata.is_file():
            continue
        data = native_source.skeleton(directory)
        count, body, hd, sq, end = struct.unpack_from('<5I', data)
        if count != 3 or not body <= hd <= sq <= end == len(data):
            raise ValueError(f'{directory}: not a standalone audio bank')
        vagi = data.index(b'IECSigaV', hd, sq)
        setb = data.find(b'IECSbteS', hd, sq)
        sesq = data.find(b'IECSqseS', sq, end)
        source_samples = {}
        spans = []
        for line in metadata.read_text().splitlines():
            if not line or line.startswith('#'):
                continue
            name, header, offset, blocks, heads, _ = line.split()
            flags = bytes.fromhex(heads)[1::2]
            stop = next((i+1 for i, flag in enumerate(flags) if flag & 1), int(blocks))
            live = flags[:stop]
            starts = [i*28 for i, flag in enumerate(live) if flag & 4]
            spans.append((int(offset), int(offset)+16*int(blocks)))
            source_samples[int(offset)] = (name, starts[-1] if starts else -1,
                stop*28, bool(live[-1] & 1 and live[-1] & 2 and starts))
        maximum = word(data, vagi+12)
        samples = []
        for i in range(maximum+1):
            at = vagi + word(data, vagi+16+i*4)
            offset, rate, loop = struct.unpack_from('<IHB', data, at)
            if body+offset in source_samples:
                name, loop_start, play_frames, repeats = source_samples[body+offset]
            elif (lead := next((k for k in range(1, 64) if body+offset+16*k in source_samples), None)) \
                    and '_tail' in source_samples[body+offset+16*lead][0] \
                    and not any(a < body+offset+16*lead and b > body+offset for a, b in spans):
                # vagconv.py writes sample data no header entry lists from its first block that is not zero
                # (sound_<hd>_tail<n>.wav); this sample starts `lead` zero blocks before it, which decode to
                # 28 silent frames each and leave the decoder as it starts.
                tail, loop_start, play_frames, repeats = source_samples[body+offset+16*lead]
                name = f'vagi_{i:03}.wav'
                with wave.open(str(directory/tail), 'rb') as wav:
                    params, pcm = wav.getparams(), wav.readframes(wav.getnframes())
                with wave.open(str(directory/name), 'wb') as wav:
                    wav.setparams(params._replace(framerate=rate))
                    wav.writeframes(bytes(2*28*lead) + pcm)
                loop_start = loop_start+28*lead if loop_start >= 0 else -1
                play_frames += 28*lead
            else:
                # The older extractor omitted final samples from samples.txt in
                # some banks. Their editable WAV export is already available.
                name = f'sound_1_{i:03}.wav'
                legacy = asset_dirs.assets_root()/'audio/sfx'/directory.name[:5]/f'sample_{i:03}.wav'
                if not (directory/name).is_file():
                    shutil.copyfile(legacy, directory/name)
                flags = []
                for pos in range(body+offset, hd-15, 16):
                    flag = data[pos+1]
                    flags.append(flag)
                    if flag & 1:
                        break
                if not flags or not flags[-1] & 1:
                    raise ValueError(f'{directory}: sample {i} has no end flag')
                starts = [j*28 for j, f in enumerate(flags) if f & 4]
                loop_start = starts[-1] if starts else -1
                play_frames = len(flags)*28
                repeats = bool(flags[-1] & 2 and starts)
            with wave.open(str(directory/name), 'rb') as wav:
                if wav.getnchannels()!=1 or wav.getsampwidth()!=2:
                    raise ValueError(f'{directory/name}: incompatible WAV format')
                if play_frames > wav.getnframes():
                    raise ValueError(f'{directory/name}: truncated WAV')
            samples.append(dict(file=name, rate=rate, frames=play_frames,
                loopStart=loop_start if repeats else -1, queueLoopFlag=loop))
        # Sesq's sequence count is stored at +0x24. The existing player maps
        # note-on timbre through Setb before selecting a Vagi sample.
        effects = []
        effect_count = word(data, sesq+0x24) if sesq >= 0 else maximum
        for effect in range(min(effect_count+1, 256)):
            timbre = effect
            if sesq >= 0:
                seq = sesq+word(data, sesq+0x34+effect*4)
                if seq+0x14 > end:
                    raise ValueError(f'{directory}: sequence out of range')
                if data[seq+0x11] == 0x90:
                    timbre = data[seq+0x13]
            sample = timbre
            if setb >= 0 and timbre <= data[setb+0x22]:
                sample = struct.unpack_from('<H', data, setb+0x24+timbre*data[setb+0x20])[0]
            effects.append(sample if sample <= maximum else -1)
        doc = dict(version=1, resource=int(directory.name[:5]), samples=samples, effects=effects)
        (directory/'audio-bank.json').write_text(json.dumps(doc, indent=2)+'\n', encoding='utf-8')
        manifests += 1
        samples_total += len(samples)
    report = dict(banks=manifests, sample_bindings=samples_total,
        source='extracted metadata and editable WAVs; no raw/ISO/ELF inputs')
    target = root/'build/audits/native-audio/conversion.json'
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(json.dumps(report, indent=2)+'\n', encoding='utf-8')
    print(json.dumps(report, indent=2))


if __name__ == '__main__':
    main()
