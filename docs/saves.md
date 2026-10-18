# Saves (game/src/host/saves.rs)

The PC port does not emulate the memory card. It saves the way PC games do:

* **Three profile slots.** At start-up a list shows the slots with when each was last saved, the time played, and how
  many characters, stages and Zeni it has. Picking a slot with a save loads it. Picking an empty slot starts a new
  game there.
* **Automatic saving.** Wherever the PS2 game asked "save to the memory card?", the port saves at once to the
  profile in use, with no question. While it saves, a "Salvando..." sign with a spinner shows in the bottom-right
  corner.
* **New Game** on the title screen when a profile is already loaded opens the slot list. Choosing a slot that is in
  use asks before erasing it.
* **Load Game** on the title screen opens the slot list. Cancelling returns to the title; loading a profile
  continues to the main menu. At boot, the last-saved profile is loaded automatically when available.
* **Options → Load** opens the slot list, to switch profiles. **Options → Save** saves the profile in use.
* **Replays** are kept per profile (`replay_1.sav` .. `replay_7.sav`). The game's own replay list is unchanged.

Files: `Documents\My Games\Tenkaichi3Rust\saves\slot<N>\`. Each slot holds:

* `system.sav`: the game's 0x4000-byte save, in the card format and with its checksum.
* `system.bak`: the previous save. It is loaded if `system.sav` is damaged.
* `info.txt`: when the slot was saved, and the time played.
* The replays.

Every file is written through a temporary file, so a crash never leaves half a save. `BT3_SAVES=<folder>` puts the
saves somewhere else.

## How it is wired

The game drives every card operation through `McFlow_Start(mode)` / `McFlow_Update` (src/sys/mcflow_a.c): eight
dialog state machines on top of the card layer. These two functions, plus `McFlow_PollCard`, are bound to the
versions in saves.rs (`WRAP` in tools/translate/c2rs/link.py). Those versions do the same work on the game's data
that each flow did on success, and end the same way:

* flags 8 (done) or 0x100 (refused);
* then `McFlow_CallDone` (system modes 0–4) or `McFlow_CallModeDone` (replay modes 5–9), at the next
  `McFlow_Update`, after the caller has registered its callbacks.

| Mode | PS2 | PC |
|---|---|---|
| 0, 4 save | question, card checks, write | write to the profile |
| 1 load | question, read | slot list; read; `SndOpt_Apply`, `Progress_ClearSession` |
| 2 boot load | card checks, read | load last-saved profile automatically; `SndOpt_Apply`; progress flag 8 |
| 3 new save | overwrite question, defaults, write | defaults, write (slot list when a profile is loaded) |
| 5 / 6 replay save / load | card | `replay_<n>.sav` |
| 7 / 9 replay list | reads the 7 headers | the same from the files |
| 8 replay question | card dialog | yes / no window of the port |

The slot list and the saving sign are drawn by the port on a canvas laid over the game's picture (host/ui.rs; the
font is made by tools/assets/make_ui_font.py). While the list is up, the game gets no buttons. `BT3_MEMCARD=1`
brings back the game's own card flows. `BT3_UI_PREVIEW=<folder>` draws the port's screens to PNGs without playing.

## Title integration verified on Windows (2026-10-15)

`host/title.rs` adds the title entry using the existing animated plates. The
five wrappers also live in `tools/translate/c2rs/link.py`, so regeneration keeps
the integration. The pending cloud files were integrated without changing the
game's combat or the real user saves.

A bounded GPU run used copies of the existing test profile in two isolated
slots under `build/audits/load-game-integration/profiles`. Boot loaded slot 2;
Load Game opened, cancellation returned to the title, reopening allowed Up
and Confirm to select slot 1, and the game reached its original main menu.
The process exited with code 0 at vblank 4300. Logs and captures are in
`build/audits/load-game-integration/runtime2`. In particular, `gpu_03300.png`
shows the profile picker and `gpu_04200.png` the main menu.

The reported cloud blocker did not reproduce with the Windows GPU overlay.
This does not establish software-renderer overlay support or full save/replay
coverage across every mode. Automatic story saves and replay flows remain
separate acceptance work.
