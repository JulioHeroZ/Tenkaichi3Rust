# Text and script source resources — 2026-10-15

The runtime now reads **100 standalone text tables (30000 slots)** from
`text-table.json`, and **50 standalone GSC scripts** directly from `script.txt`.
These resources no longer open or need `skeleton.bin`. The stock data produced
by the Rust readers matches every byte of the reference resources.

## Editing text

Edit `assets/files/pzs3us1/<resource>/text-table.json`:

```json
{
  "version": 1,
  "slots": [
    {"text": "Translated dialogue", "minimumBytes": 64},
    {"text": "Following dialogue", "minimumBytes": 64}
  ]
}
```

The array index remains the text id. `minimumBytes` preserves the initial slot
capacity; it is optional for new files. Longer text automatically expands the
slot and updates subsequent offsets. The compiler serializes UTF-16 and the
small offset table needed by the translated text readers entirely from JSON.
It does not read a binary layout template. Resource size is capped at 64 MiB;
the game's allocator, glyph support and on-screen text layout still impose
their own practical limits. This does not add font glyphs or redesign windows.

For these 100 tables, `text-table.json` is now the runtime source; their older
`texts.json` files remain reference extraction metadata. Mixed graphic/text
resources still use their existing `texts.json` path until separately migrated.
`convert_native_text.py` checks that each migrated resource contains only a
text directory, text entries and padding. It refuses to discard unknown payloads
and preserves existing native manifests when rerun, protecting translations.

## Editing scripts

The same extracted `script.txt` files remain the source. Rust compiles GSC
actions, commands, operands and values directly. GSC is the game's scripting
language, not PS2 machine instructions. The existing Rust GSC VM still executes
the commands with their original phases and frame scheduling.

The `size` declaration is now a minimum capacity for normal source loading;
larger scripts can grow. Byte-exact offline `BT3_VERIFY_FILES` comparison keeps
the previous capacity restriction. Extra content after the root chunk is now
rejected instead of silently discarded. No stock command, action, value or
gameplay timing was changed. The VM's task limits remain unchanged.

## Offline compiler

`cargo build --release` in `game` also builds `bt3-assetc`, which uses
the same readers as the game without opening SDL, a window or saves:

```
bt3-assetc <resource-directory> <output-file>
bt3-assetc --batch <source-TAB-output-list.tsv>
```

With the normal launcher cache it is located alongside `bt3-original.exe` in
`%LOCALAPPDATA%\bt3-original-target\release`. `cargo run` still defaults to the
game. Offline outputs are inspection/reference memory images, not a complete
PC distribution package. The compiler rejects `BT3_PC_ONLY` in-process transport
mode so ephemeral resource handles cannot accidentally be saved as assets.

Preparation and verification:

```
python tools/assets/convert_native_text.py
python tools/assets/verify_native_text.py --compiler <path-to-bt3-assetc.exe>
```

Verification stages all 150 resources into folders containing only JSON or GSC
source, compiles them with Rust, and compares their outputs against reference
data. It also verifies longer Unicode strings, preservation of following text
entries, growth past the old script capacity and rejection of malformed roots.
All passed; no binary templates are present in the isolated input folders.
The expanded fixtures are never loaded into the game or the user's saves.

The remaining layout inventory is **2103 skeleton resources and 185 banks**,
or **2288 top-level containers**. This is not a global port percentage. The
translated memory/layout model and the remaining graphic/model/animation
structures still need migration.
