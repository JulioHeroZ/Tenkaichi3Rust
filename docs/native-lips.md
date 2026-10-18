# Lip-sync tables without skeleton.bin

The Rust loader now reads `lip-sync.json` for 96 standalone lip-sync tables
(9756 clips). It does not open `skeleton.bin`, `lips.json`, texture metadata or
the original archive for these resources. Old extracted files remain on disk
as offline reference; deleting them is not required for the native reader.

The schema contains `version: 1` and an indexed `slots` array. A null slot is
empty. A clip has `minimumBytes` and `keys`, pairs of unsigned 16-bit
`[frame, mouth]` values in frame order. Mouth values are preserved numerically;
the converter does not invent additional semantic constraints. Minimum capacity
preserves stock layout, while extra keys can grow the allocation. Rust computes
the directory, LIPS headers, padding and following offsets from these fields.

This removes a binary file dependency, not the translated lip-sync consumer's
in-memory layout. Mixed fighter/menu resources still require separate conversion.
No opaque byte arrays, original offsets or copied headers are stored in the JSON.
Malformed native JSON fails visibly instead of falling back to a skeleton.

## Preparation and validation

```
python tools/assets/convert_native_lips.py
python tools/assets/verify_native_lips.py --compiler build/cargo/release/bt3-assetc.exe
python tools/assets/audit_skeletons.py
```

The converter accepts a resource only when every template byte is explained by
the directory, LIPS constants and zero padding. Existing native manifests are
preserved on rerun. It leaves mixed resources pending. The verifier compiles
JSON-only folders through the actual Rust reader and compares all 96 outputs
against the stock archives: zero byte differences. It also checks growing clips,
empty slot indices, following entries and 13 invalid inputs (including a broken
manifest next to a legacy template). Release compiler and game builds passed.
Reports are in `build/audits/native-lips`.

A hidden-window GPU attract-demo fight finished normally (exit 0) with
`BT3_PC_ONLY=1`, `BT3_NO_BPE=1`, a nonexistent raw root and isolated saves.
This is general regression coverage: its resource log did not include any of
the 96 standalone lip tables. Their specific coverage is the isolated Rust
reader comparison above, not an observed story dialogue or visual mouth test.

## Remaining skeleton work

The updated inventory counts 2007 active top-level skeletons (previously 2103):

| Category | Resources |
|---|---:|
| Textures and retained layouts | 1186 |
| Models and mixed data | 816 |
| Mixed timelines | 3 |
| Unclassified | 2 |

These categories describe the remaining dependencies, not full interpretation of
their bytes. Nested bank blobs and actual model bone hierarchies are separate.
185 banks also remain, for 2192 top-level containers dependent on compatibility
layouts. No global port completion percentage is established.
