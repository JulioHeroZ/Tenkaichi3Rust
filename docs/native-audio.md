# Native PC audio resources — 2026-10-15

In PC-only mode, sound effects now read **361 audio-bank.json manifests** and
**18924 sample bindings** directly from editable PCM WAVs. No Sony HD/SQ tables,
SPU ADPCM bank reconstruction, IOP copies or SPU transfers are used to load or
play these banks. The 2 MiB SPU sample-memory buffer is not allocated in this
mode. Attempts to play a legacy bank or upload SPU data stop with an error.

`host/assets.rs` represents a bank file request with a small PC-owned resource
token. `host/snd.rs` resolves it to the manifest and WAVs. Generated bindings
and `tools/translate/c2rs/link.py` route Snd_LoadBank, Snd_ReloadBank and
Snd_UnloadBank to the PC implementation. Small compatibility state fields
(loaded flag, max sample id and original queue loop bits) are retained for
game code; the original command queue still supplies volume, pitch, pan,
pause and stopping behavior. Fixed-slot reload rules are retained.

## Editable files

Each standalone effects bank lives in:

```
assets/files/pzs3us1/<resource>/audio-bank.json
assets/files/pzs3us1/<resource>/sound_1_<sample>.wav
```

The manifest contains a sample filename, playback rate, playable frame count,
loop start, original queue loop flag, and an effect-id to sample-id map. Repeated
sample aliases are explicit bindings. Paths are restricted to WAV filenames in
the bank's own directory. PCM loads are lazy and cached, retaining the existing
mixer's interpolation, stereo panning and pitch behavior. A missing/invalid PCM
file fails visibly when first played instead of decoding an opaque fallback.

Music, voices and movie audio use the already converted native WAV tree:

```
assets/audio/streams/pzs3us2/<resource>.bin.wav
assets/audio/streams/disc/<name>.ADX.wav
```

These are ordinary PCM WAVs. The names retain resource keys but the files contain
no ADX data. Native WAVs preserve the exact playable length and RIFF smpl loops.
The reversible `assets/files/.../stream.wav` extraction includes encoder-frame
padding and does not contain smpl loops, so it is no longer used for playback
in PC-only mode. A missing native stream cannot fall back to ADX reconstruction.
For music/voice edits in this mode, edit `assets/audio/streams`.

## Offline preparation and verification

```
python tools/assets/convert_native_audio_banks.py
python tools/assets/verify_native_audio.py
```

The converter needs only existing extracted metadata and PC WAVs. It never
opens the ISO, ELF or raw disc directory. Some final samples omitted by the
reversible extractor are recovered from the existing PC WAV export. Their end
and loop positions are recovered from retained extraction metadata. Unknown
bank data is not packaged into the native manifest.

The verifier compares every played PCM frame, playback rate, playable length,
loop point and bank queue loop flag with the prior independent PC export:
**361 banks, 18924 bindings, zero metadata or played-PCM differences**.
Records: `build/audits/native-audio/pcm-verification.json`.

The first complete GPU attract-demo fight ended normally with native effect
banks and SPU/legacy playback blocked (`fight.log`, exit 0). A second complete fight also passed with canonical native streams and loops
(`native-streams-fight.log`, exit 0); both fights processed 360 effect commands. These are technical
hidden-window runs with isolated saves. They do not replace listening checks
across every character/mode or long-duration real-time A/V validation.

Original skeletons and reconstruction metadata remain on disk for offline
reference; bank playback no longer opens them. Legacy diagnostics outside
PC-only mode retain their prior backend. Overall native rendering/models,
animation and other resource layouts still require conversion; this milestone
does not make the entire port 100% native.
