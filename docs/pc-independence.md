# PC resource independence — 2026-10-15

## Latest update: native text/scripts and prepared-resource transport

100 standalone text tables and 50 GSC scripts no longer read binary skeletons.
All 150 matched the reference bytes when compiled from isolated text-only
folders. Expanded Unicode text and script-size growth passed verification.
See [text/scripts](native-text-scripts.md).

The current active layout inventory is **2103 skeleton resources + 185 banks =
2288 containers**. Historical counts below belong to earlier milestones.
Prepared PC images/animation entries also bypass redundant BPE encoding/decoding;
full demo fight and opening/title runs passed with legacy BPE decoding blocked.
That transport does not remove the remaining graphic/animation layouts.
See [prepared-resource transport](pc-resource-transport.md).


## Update: native audio resources

The normal PC-only runtime now loads all 361 effect banks from audio-bank.json
and WAVs, and all 65201 archive streams from assets/audio/streams. No Sony bank
reconstruction, SPU sample RAM or SPU transfers are used; legacy bank playback
and ADX stream fallback are prohibited in PC-only mode. Every one of the 18924
sample bindings was compared with the prior PCM export: zero differences.
Two complete attract-demo fights passed, with 360 effect commands each.
See [native audio](native-audio.md) for edits, tools and validation limits.

At the native-audio milestone, the active inventory reported **2253 top-level skeleton representations and
185 banks**, or **2438 containers still needing native layout conversion**.
Physical offline reference files remain preserved. Earlier counts below describe
the inventory before the native-audio migration and must not be used as the
current runtime dependency count. No global port percentage is established.


The normal launcher, `game/jogar.cmd`, now sets `BT3_PC_ONLY=1`,
`BT3_ASSETS=<project>/assets` and a data root outside `_legacy/raw`.
The file backend refuses all original-file paths in this mode. A missing or
unrebuildable resource fails immediately with its name instead of falling back
to an original BIN or spinning in the translated disc reader.

Diagnostic overrides `BT3_ORIGINAL_FILES`, `BT3_ORIGINAL_AUDIO` and `BT3_TABLES`
are rejected in this mode. Source-development diagnostic launches without
PC-only mode can still use the legacy backend; the normal launcher cannot.
Original disc files remain preserved for offline conversion and comparison.

## Completed runtime changes

- (Superseded 18/10/2026: the movies are now `assets/video/*.m2v`, decoded in Rust, see movies.md.) Opening/ending used `assets/video/zs3usop.mkv` and `zs3used.mkv`, FFV1 lossless
  video in Matroska. Every decoded RGBA frame matches the PSS input: 2810 and
  4200 frames respectively. `tools/assets/prepare_pc_media.py` performs the
  offline conversion and verification. Runtime does not open PSS or initialize
  PS2 MPEG/IPU/read-ahead objects. It allocates only the display buffers.
- Streaming audio reads native PCM WAVs from `assets/audio/streams`, preserving
  playable lengths and RIFF smpl loop points. ADX reconstruction is prohibited
  during PC-only playback.
- 596 previously unconverted empty directory files now use
  `empty-directory.json` with an entry count, not binary placeholders.
- The common roster/item file is now `roster-items.json`: 161 character records
  and 350 equippable item records, with named fields from `battle/view_b.h`.
  Unknown source fields remain explicitly named unknowns. All 23744 bytes of
  the compatibility memory serialization were checked against the input.
- Four remaining EVPp resources are editable `point-groups.json`: group
  attributes, positions, a second vector and point attributes. The behavioral
  meaning of the second vector/attributes is not established. Every serialized
  byte, including signed float zero, was checked; no opaque source blob remains
  in these four exports.

Offline tools:

```
python tools/assets/convert_empty_directories.py
python tools/assets/convert_roster_items.py
python tools/assets/convert_point_groups.py
python tools/assets/prepare_pc_media.py
python tools/assets/audit_pc_independence.py
```

## Measured coverage and remaining native work

The resource audit covers **68601 / 68601 archive resources**, with no missing
extracted representation. This is **100% extraction coverage**, not 100% native
asset formats or proof of every gameplay path. `build/audits/independence/coverage.json`
records the counts and explicitly marks `fully_native_asset_formats=false`.

Before native audio, the inventory had **67815 top-level skeleton.bin representations and 185 banks**.
They reconstruct compatibility data from PNG/glTF/WAV/JSON/scripts and retained
layout metadata. Some unpacked animation/bank entries still contain original
format data. These are not a complete native asset pipeline, and they must not
be described as one. Removing them requires typed converters plus native readers
for the remaining GS/VIF/SPU/BPE-dependent resource structures, without changing
the original animations, rendering or gameplay behavior.

The Rust code also retains the translated game's memory/layout model. Removing
disc-file access does not turn this architecture into an idiomatic native engine.
No reliable global port completion percentage is established.

## Validation

Windows release build passed. Existing generated-code warnings remain.
`build/audits/independence` holds logs and conversion/inventory evidence.
With `BT3_PC_ONLY=1`, explicit PC assets and a nonexistent raw data path:

- An attract-demo fight reached its normal `battle finished` exit, status 0.
- Ending preview played all 4200 converted frames and exited normally, status 0.
- Rust rebuilt all 601 new JSON exports with zero byte differences against the
  offline reference (`BT3_VERIFY_PC_TABLES=1`, explicit assets and raw roots).
- Normal boot played the complete 2810-frame opening and continued through the
  original title flow to the configured 9000-vblank limit, status 0.

Runs used GPU rendering, isolated saves and hidden windows. Video duration runs
were uncapped technical checks, not long-duration real-time A/V drift tests.
All modes still require regression coverage in strict mode. Neither the original
directory nor the user's real save directory was moved or deleted.
