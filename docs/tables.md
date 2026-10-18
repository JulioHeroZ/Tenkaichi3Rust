# The game's data tables

The decompilation leaves some data to the original executable: 2388 table runs (`game/src/gen/data.rs`
`ORIGINAL_RUNS`: PS2 address, source, offset, length): tables of moves, characters, menus, effects...

* The installer writes them from the user's own executable into `assets/data/tables.txt`
  (`tools/translate/c2rs/tables_from_elf.py`): one line per run, its address and its bytes in hex.
* `game/src/host/tables.rs` reads that file when the game starts and writes each run into the PS2 memory image.
  The program carries none of the game's data: it is built without the disc, and the same executable works for
  everyone.
* `BT3_TABLES=<catalog>` (assets/data/runtime/source-tables.bt3, made by older tools) loads a PC catalog instead and
  reports how many runs of tables.txt differ from it.
