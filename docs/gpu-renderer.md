# GPU renderer and frame time (game)

`BT3_GS=gpu` opens a window (SDL3 + OpenGL 3.3, `SDL3.dll` next to the program); `BT3_GS=gpu-headless` draws
into an offscreen EGL context (Mesa) for tests; `BT3_GS=1` keeps the software reference.

* `src/host/gs/gpu.rs` — capture and replay, the Rust version of the C port's `gs_draw.c` + `gs_gl.c`: one render
  target per frame-buffer address at `BT3_SCALE` (default 2) times the PS2 resolution, textures decoded from GS
  memory and cached by TEX0 / TEXA / page hashes, the native depth-table, outline, DATE and alpha-copy passes,
  frame-buffer uploads (movies), present with 4:3 letterbox, screenshots (`BT3_SHOT=n` -> `shots/gpu_NNNNN.png`).
* `src/host/gs/shaders/` — the C port's shaders unchanged (Vulkan GLSL 450); `translate()` turns them into GL 3.3
  core with the C port's rules (flip of gl_Position.y, z range without glClipControl).
* `src/host/gs/hle.rs` — vertex programs 0, 1, 2a/2b, 4, 6, 7, 8 (recognised by size) handed to their shaders
  instead of the VU1 interpreter, as `gs_vu1.c` does. `BT3_VU_INTERP=1` turns this off.
* `src/host/gs/platform.rs` — SDL3 / EGL loaded at run time, keyboard and gamepad (the C port's default keys).
* `src/host/native.rs` — `vu0_b.c`'s `f_bits` / `f_from` (a 4-byte memcpy between two locals) as the bit
  pattern itself; bound by `tools/translate/c2rs/link.py` (`NATIVE`).

Checks: the software renderer is pixel-identical to the C port's software renderer (frames 100 / 300 / 500 of the
demo, seed 0x12345678). After the native helpers: demo trace identical over 4242 lines; lockstep seed 0x1,
Battle_Update #300 and #1000: 0 differing words.

Frame time of the demo battle (CPU side, cloud machine, 2 cores, `BT3_GPU_NOREPLAY=1 BT3_GS_VERBOSE=1`):

| step | ms / frame |
| --- | --- |
| GPU renderer, VU1 interpreted | ~75 (VU1 ~31, game ~22) |
| + vertex programs as shaders | ~40 (GS ~6) |
| + native f_bits / f_from | ~17-22 (game ~12-15) |

`BT3_PROF=<file>` (Linux) samples the main thread every millisecond of processor time; with
`RUSTFLAGS=-Cforce-frame-pointers=yes` it records the callers too.
