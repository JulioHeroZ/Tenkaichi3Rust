# The game's files in open formats (assets/files)

Current PC-only launcher disables raw fallback. Additional typed JSON resources,
direct WAV playback and converted Matroska movies are documented in
[PC independence](pc-independence.md). This extraction still retains compatibility
layouts and is not a wholly native resource pipeline.

The port reads the game's files through `game/src/host/files.rs`. When a file has an extraction in
`assets/files/<partition>/<number>/`, the port rebuilds the file from it and the game gets those bytes instead of
the disc's. The rule is **exact memory**: something is taken out into an open format only when the rebuild gives back
every byte of the original. So the game reads what it read from the disc. If you edit the open files, the game reads
your edits.

* Folders: after the extraction, `tools/assets/organize.py` groups the folders by kind and names them
  (`files/lutadores/Goku (Early)/01423 roupa 1/`, `files/cenários/12 Kame House/00380 cenário/`, `files/menus/00448/`).
  The game finds a file of pzs3us1 by the five digits at the start of its folder's name, wherever it is
  (`game/src/host/assets.rs` file_dirs, `tools/assets/asset_dirs.py`); the rest of the name and the grouping are free.
* `tools/assets/files_extract.py <raw folder> <assets folder> [partition]` writes the extraction. It can be resumed:
  the files it has already looked at are listed in `assets/files/<partition>/scanned3.txt`.
* `BT3_VERIFY_FILES=1` (with `BT3_DATA` = the raw folder, `BT3_ASSETS` = the assets folder) rebuilds every file of
  the extraction and compares it with the original. `BT3_VERIFY_DUMP=<folder>` also writes out each file that
  differs. `BT3_ORIGINAL_FILES=1` ignores the extraction.

What a folder can hold:

| File | What it is | Edits |
|---|---|---|
| `skeleton.txt` | The size of the file (`zeros <bytes>`): every byte of it comes from the other files of the folder. Older extractions had a `skeleton.bin` with the bytes nothing else gave; `tools/assets/add_tables.py` turns those into `file-tables.json` in place. For a packed file the size is that of the unpacked data. | — |
| `file-tables.json` | What no other file gives, as typed records ([below](#typed-records-layoutpy)): the file's directory and runs of words whose structure is not known yet (`Words`). | Values |
| `model-tables.json` | The game tables of a fighter's model file as typed records: `FighterParam`, `AttackRecord` (damage, guard damage, ki, push, launch angles, reactions...), `KiBlast` (13 kinds), `MoveParam` (speeds in 10 km/h, turn rates, ki costs), `SuperParam` (Blast 2 and Ultimate), `SkillParam` (Blast 1), `ChainParam` (hair/cape physics). | Values (balance edits) |
| `effects.gltf` + `effects.bin` + `effect-chains.json` | The effects' small meshes (effect packs of the stages and others) and their VIF chains (the `stage-chains.json` schema). | Vertex values (same count per primitive) |
| `effect-tables.json` | The effects' tables as typed records: emitter sets (header, groups, parts), particle emitter definitions (kind 5: shape, sheet, key times, keyed values, angles, spins) and "V000" sprite animations (layers and keys: position, rotation, scale, colour, frame). | Values |
| `surfaces.gltf` + `.bin` + `surfaces.txt` + `surface-tables.json` | A stage's animated surfaces (water, lava, scrolling layers): the triangles in glTF, the reflection parameters, textures and surfaces (scroll, flags, bounds) as typed records. | Vertex values (same count) |
| `raw-textures.txt` + `rawtex_NNN.png` | GS uploads no texture entry describes: the uploaded bytes as PNGs (32-bit pixels as RGBA, else the bytes as a grey palette image); the packet is written from the line. | Pixels (same size) |
| `nested.txt` + `at_<offset>/` | Model files inside this file (the technique models of the effect files), each extracted like a file of its own and copied back at its offset. | Like a file's folder (same size) |
| `stage-layout.json` | The tables of a stage as typed records: header, draw groups, materials, meshes, nodes, parts, objects, octree, timers. | Values; pointers are file offsets |
| `stage-chains.json` (with `model.gltf`) | The drawing chains of a stage's meshes: per batch its primitive and GS header. The chains are written whole from these two files. | Vertex values (same count per primitive) |
| `textures.txt` + `tex_NNN_WxH.png` | One PNG per texture: an indexed PNG with the palette, or RGBA. Alpha is the GS alpha (0..128) scaled to 0..255; textures whose alpha goes above 128 are marked `gs` and keep the raw value. | Pixels and palette (same size) |
| `model.gltf` + `model.bin` + `models.txt` | A fighter model (pmdl) or a stage. | Vertex values (same count per primitive) |
| `model.json` (with `model.gltf`) | The rest of a fighter model: header, node records, the GS settings of each batch. The model is built from these two files; nothing of it is left in `skeleton.bin`. | Vertices and triangles freely (another vertex count too) |
| `model-motions.gltf` + `.bin` + `model-motions.json` | The motions inside a fighter file (entries 0x1C..0x23): keys as glTF animations, structure and events in JSON. | Keys (same count per track), events |
| `collision.gltf` + `collision.bin` | The collision of a stage: zones, meshes, polygons and their box trees. | Polygon flags; positions only together with the planes and boxes |
| `bpe.txt` | The file is packed with the game's BPE. The port packs the rebuilt file again on load. | — |
| `bank.txt` + `entry_NNN.bin` or `entry_NNN/` + `animations.gltf` + `animations.bin` | A bank (a directory of files each packed with the BPE): animation banks, and others such as the 1-entry packed title images. Each entry is unpacked. Motions' keys are in glTF animations. A motion entry's structure (header, tracks, keys no glTF track has) is a typed row of `motion-sources.json`; every other entry gets a folder `entry_NNN/` with the same files as a file's folder (an empty entry stays an empty `entry_NNN.bin`). `tools/assets/add_motion_sources.py` brings an older extraction to this. | Key values (same key count per track) |
| `texts.json` | The UTF-16 texts of the file (menus, move lists, dialogue) as plain text, those of the text tables inside its entries too; `bom`: the byte order mark before the text is written as well. | Text (fits in `room` bytes) |
| `movies.json` | The menus' Flash-like movies (FOD): tags, sprites, images, shapes (quads), timelines with their place / remove / action tags ([below](#menu-movies-fodconvpy)). | Anything: the movie is written whole (it must fit where it was) |
| `texture-packets.txt` + `texture-entries.json` | The GS upload packets around the textures' data (`offset length width height`, written whole) and the texture entries as typed records. | — |
| `lips.json` | The lip sync keys of voice lines: [frame, mouth open]. | Keys (up to `room`) |
| `lip-sync.json` | Standalone lip-sync table; replaces skeleton and lips metadata for 96 resources. See [native lip tables](native-lips.md). | Keys may grow; empty slots preserve indices |
| `cameras.json` | The camera animations (intros, cuts): default pose and keys [flags, time, value] per channel. | Values (same key counts) |
| `sound_<hd>_<n>.wav` + `samples.txt` | The samples of a sound bank (SPU ADPCM) as 16-bit mono WAVs; `sound_<hd>_tail<n>.wav` is sample data the bank's header does not list. | Samples (same length) |
| `sound-tables.json` | The Sony sound headers as typed records: chunk headers, version, header (sizes, chunk offsets), the sample list (offset, rate, loop). | Values |
| `script.txt` | A GSC script (menus, opening) as text. | Anything that fits in the file: actions, commands, values |
| `events.json` (banks) | The events of the motions: hit windows and their volumes, and the other events. | Values (same count) |
| `stream.wav` + `adx.txt` | A CRI ADX audio stream (disc/DATA/*.ADX, in a folder named like the file) as a 16-bit WAV. | Samples (same length) |

## Textures (`tools/assets/texconv.py`)

A texture entry (64 bytes) has these fields: the pixel and CLUT upload packets (offsets in quadwords), their sizes,
the BITBLT words, TBP, CBP − 0x2000, and TEX0. The PNG is what the GS samples after the uploads. This is worked out
through the GS memory layouts (PSMCT32/24/16/16S, PSMT8/4, PSMT8H/4HL/4HH, CLUT CSM1), so swizzled uploads (for
example 8-bit data sent as 32-bit) come out as plain pictures. Rebuilding runs the same layouts in the opposite
direction.

Textures are found by their 64-byte entries anywhere in the file. Their offsets count from the start of the
directory they are in, which can be nested: `dir_bases` follows directories (u32 count, offsets) four levels down.

## BPE (`tools/assets/bpe.py`, `bpe_pack` in assets.rs)

The game's compression is Philip Gage's byte pair encoding (C Users Journal, 1994). A packed file is `u32 size`,
`u32 packed size`, then blocks of 10000 bytes, each compressed on its own. A file whose size is a multiple of 10000
ends with an empty block. To give back the same bytes, the packer must make the same choices as the game's packer:

* Gage's compress.c, with a 16384-entry pair hash table.
* Threshold 3.
* Codes tried from 255 downwards.
* Ties going to the first pair in hash order.

Verified on all 699 packed files and on every entry of the 233 banks. After the packed data, the original files are
padded to 16 bytes with leftovers of the packer's memory; `bpe.txt` keeps that padding (`tail`).

## Fighter models (`tools/assets/modelconv.py`)

The model is entry 3 of a fighter's resource directory. It is a list of node records (the skeleton). Each node's DMA
chain uploads vertex batches into VU1 memory (UNPACK V4-32) and kicks the drawing program. A vertex is 3 quadwords:

* `x y z weight`
* `nx ny nz -`
* `u v - -`

The positions are in model space, blended between the node and its parent by the weight.

In the glTF:

* Each batch is one primitive: the triangle strip as a triangle list.
* The nodes are the skeleton, and the mesh is skinned with JOINTS (node, parent) and WEIGHTS (weight, 1 − weight).
* The game's normals are not of unit length (the lighting uses their length), so they are kept as they are in
  `_NORMAL_GAME`. That is what goes back into the file. `NORMAL` is the unit version, for viewers.
* A `root` node turned half a turn about x shows the model standing (the game's y points down).

`models.txt` gives, per primitive, the file offset of its first vertex and its vertex count. The words that go back
are POSITION, WEIGHTS_0.x, _NORMAL_GAME, TEXCOORD_0 and _Q_GAME (q, word 10, 1.0 as the game has it). All 742 fighter models are converted.

Entries 3..8 of a fighter file are loose VIF chains drawn the same way: faces and other parts the game swaps in.
There are 2722 of them. Each becomes a further mesh, `part_<entry>`, which is not skinned (which bones they follow is
not known here). Its weight words are in `_WEIGHT_GAME`, and its lines in `models.txt` start with the mesh number.

## Fighter models built from glTF (`tools/assets/pmdlconv.py`)

`model.json` holds everything of a fighter model that is not a vertex, so the Rust rebuild (`model_build` in
host/assets.rs) writes the whole model, entry 2 and the loose parts (entries 3..8), from `model.gltf` + `model.json`:

* the pmdl header, then one record per node: next record, pop, last, vertex count, id, a word, then 3 vectors (a node
  without geometry: world position, parent's, position relative to the parent) or 5 (with geometry: two more);
* per node with geometry, a DMA chain: one RET tag whose VIF code is, per batch of at most 30 vertices, UNPACK of 5
  quadwords (GIF tag A+D, TEX0_1, TEX0_2, the strip's GIF tag without and with EOP), UNPACK of the vertices
  (`x y z weight | nx ny nz flag | u v q 0`), MSCNT; NOPs to the quadword; an END tag and zeros.

Every chain of the 742 models has exactly this form, so the builder writes the chains itself from the glTF
primitives: `gs` keeps the 20 words of each batch's header (NLOOP left out), and the vertex words come from POSITION,
WEIGHTS_0.x (or _WEIGHT_GAME), _NORMAL_GAME, _FLAG_GAME (0 or 1, only where a primitive has one), TEXCOORD_0 and
_Q_GAME.

**Editing.** A primitive is drawn as a triangle strip. When its triangles are the ones the extraction wrote, its
vertices are the strip as they are. Otherwise (triangles added, removed or reordered in a 3D program) a strip is made
of its triangles: grown over shared edges, keeping each triangle's winding, joined by repeated vertices, and cut into
batches of 30 vertices. A model of another size moves the file's entries after it (the directory follows), up to
0xCE000 bytes, the game's buffer for a fighter file (BTL_RES_MODEL_SIZE). Checked: all 97 primitives of a fighter
reordered gave a file 1.6 KB larger, and a demo battle drew pixel for pixel the same pictures with the same trace.

Limits: a primitive keeps its node (the bone it moves with, the first of JOINTS_0) and its batch settings; new
primitives or materials, and moving geometry between bones, are not supported yet.

## Stages (`tools/assets/stageconv.py`)

The stage is entry 0 of its file. Offsets are in words from its start (`BtlStage_Relocate`, src/sys/gfxm_e_d.c).
It has 7 draw groups, each a list of materials. A material is a texture and its meshes, and a mesh is a VIF chain.

A chain uploads batches. Each batch has a 3-quadword header whose third quadword is the GIF tag (NLOOP vertices,
triangle strip). The vertices start at quadword 3, 3 quadwords each:

* `x y z 1`
* `r g b a` (floats, 128 = full)
* `s t q 0`

The colours are kept as they are in `_COLOR_GAME`, which goes back into the file. `COLOR_0` is them divided by 128,
for viewers. The materials point at the stage's own textures (texture file at header +0x54). The `root` node also
scales the stage by 1/1000.

The 4th word of each position and q (`x y z w`, `s t q 0`) are kept as they are in `_W_GAME` and `_Q_GAME`:
w is 1.0 on most vertices but 0 or the integer 1 on some, and q is 1.0.

## Stage tables and chains (`stagelayout.py`, `stagechain.py`)

Entry 0 of a stage file is what `BtlStage_Relocate` (src/sys/gfxm_e_d.c) relocates; every offset in it counts in
words from its start. `stage-layout.json` holds it as typed records, following the pointers from the header:

* `StageHeader`: counts and pointers of the tables below, the octree size, the texture file.
* `DrawGroup` (7) → `Material` (texture number, meshes) → `Mesh` (its chain, its `Node`).
* `Node` (0x50 bytes: 16 floats, a pointer to a `NodeExtra` of 8 floats).
* `PartGroup` → `Part` (a `MeshList` of meshes, a node, 5 floats); the octree's leaves and the objects list parts.
* `Object` (destructible things: parts in three pairs and a `PartList`), `OctNode` (8 children, a list of parts),
  `StageTimer`.

`stage-chains.json` gives each mesh's VIF chain: a DMA RET tag; optionally one setting quadword (UNPACK + MSCNT);
per batch UNPACK of its 3 header quadwords (NLOOP = the primitive's vertex count), UNPACK of the vertices
(`x y z w | r g b a | s t q 0` from `model.gltf`), MSCNT; NOPs to the quadword, a DMA END tag and zeros up to the
next chain. A chain may not grow past the next one.

## Typed records (`layout.py`)

`file-tables.json`, `stage-layout.json` (and later typed tables) share one format, applied first by the rebuild
(`layout_apply` in host/assets.rs), so the other files can read the structure they give:

* `types`: per type its size and fields `[name, byte offset, kind]`. Kinds: `u8 s8 u16 s16 u32 s32 f32`, `ptr`
  (an offset in words from `base`, written as the file offset it points at; `null` for 0, `-1` for 0xFFFFFFFF),
  arrays `kind[n]`, and a last field `kind[*]` with as many values as the record gives.
* `base`: where pointers count from. `records`: `{"type", "at": file offset, field: value, ...}`.
* A float JSON cannot give back exactly (NaN, infinities) is `{"bits": n}`.
* Names follow the decompilation's headers. `unkXX` fields and `Words` records (runs of words of no known
  structure: `w` is the list of words) are what is not understood yet.

## Menu movies (`fodconv.py`)

The menus and the HUD play Flash-like movies ("FOD" 0x11 "LIT", src/sys/gfxm_b_c.c and gfxm_c.c). A movie is a
list of tags `{code u8, unk u8, count u16, size u32, data}` ended by code 0; each tag holds records with the same
8-byte header (`i`, `u`, `n`, size):

* tag 2, sprites: `id`, the record of tag 6 with the sprite's frames;
* tag 3, images: `w`, `h` (the texture is the movie's texture `n`);
* tag 4, shapes: `quads` `[image (-1 flat), unk, rgba (r in the top byte), x0, x1, y0, y1]`;
* tags 6 (sprites) and 7 (root): `n` depths and `frames`, each a `name` and `tags`, then the `last` block's name.

Frame tags (`t`): 4 PlaceObject, 26 PlaceObject2 (`flags`: 2 new object, 4 matrix, 8 colour transform, 0x20 name),
5 RemoveObject, 28 RemoveObject2, 12 DoAction (`actions`: `[0x81, frame]` GotoFrame, `[0x83, url, arg]` GetURL —
the game's commands "pad", "trig", "se", "trigger" —, `[0x8B, name]` SetTarget, `[0x8C, label]` GotoLabel,
`[4]` `[5]` `[6]` `[7]` next / previous / play / stop), 1 ShowFrame. A matrix `m` is `{"f": flags, "s": [bits,
scale x, scale y], "r": [bits, skew...], "t": [bits, x, y]}` (scale in 16.16, translation in twentieths of a
pixel); a colour transform `cx` is `{"f", "n": bits, "v": [multipliers (8.8)..., offsets...]}`.

## Stage collision (`tools/assets/colconv.py`)

The collision of a stage is in entry 0 too, after the drawn model. It was most of what was left of the original
bytes of the stages (about 79 MB of 110 MB over the 74 stages).

* Header +0x38 / +0x3C: the zones (`StgColZone`, include/battle/eft_det_b.h, 0x40 bytes). A zone has a mark, the
  collision mesh of its static geometry, the list of neighbouring zones, a list of records (`StgColRec`: broken,
  object, mesh, a word; the collision of a destructible object, whole or broken), two words, the centre and the
  half size.
* A mesh (`ColMesh`, include/battle/col_a.h): node count, polygon count, 0, then the offsets (words from the
  stage's start) of its nodes, polygons and vertices. A vertex is `x y z w`. A polygon is the flags, three vertex
  indices and the plane (normal, distance). A node is a box (min, max), then left and right; right = -1 marks a
  leaf, whose left is its polygon.

`collision.gltf` has one mesh per collision mesh, with POSITION and the triangles, so it opens in any 3D program.
Its extras hold what the game needs as it is: per mesh the header, the polygon flags and accessors for the vertex
w, the planes, the node boxes and links; in the scene the zones (neighbours, records, the centre and half size in
the `zone_boxes` accessor). The rebuild writes everything back where it was (host/assets.rs `collision_apply`).

The flags can be edited freely (for example STGCOL_POLY_NO_CAMERA, NO_SOLID). Moving vertices also needs the
planes and the node boxes to match, which nothing recomputes yet.

`tools/assets/add_collision.py`, `add_stage_wq.py`, `add_model_q.py` and `add_model_layout.py` took the collision, the stage w / q words
and the fighter q words out of files that were already extracted, checking each one byte for byte before writing.

## Animation banks (`tools/assets/motion.py`)

A bank is laid out as:

1. `u32 count`.
2. The entries' offsets.
3. The end offset.
4. Zeros up to 64 bytes.
5. The entries, each packed with the BPE and padded to 64 bytes. `bank.txt` keeps the original padding; an entry
   whose packed size changed gets zeros instead.

A motion (entries 1..414 of a 440-entry bank) is laid out as:

1. `u8`, `u8 events`, `u16 last frame`, `u16`.
2. 71 `u16` track offsets (in words), one per node.
3. The events, from byte 148.
4. The tracks. Each track is `u16 flags` (bit 0: rotation only), `u16 keys`, then either `n × u64 rotation` and
   `n × u16 frame`, or `n × (x y z, u32 frame, u64 rotation)`.

A packed rotation keeps the three smaller components of the quaternion in 20 bits each, and the index of the largest
in the top bits. The game rebuilds the largest as `sqrt(1 − others²)`. The packer also writes the exact packed value
back from the unpacked quaternion. The game's packer sometimes kept out a component that was only almost the largest,
so the packer tries each index and keeps the one that unpacks to the same floats. This is verified on every key.

`animations.gltf` takes the nodes, skin, mesh and textures of the fighter's model, by reference to the model's folder
(the file 8 before the bank). It adds one animation per motion (`motion_NNN`): rotation channels, plus translation
channels for tracks that have positions. Times are frame / 30. Some tracks stay in their entry as they are:

* a track whose frames do not go up, since glTF needs increasing times;
* a track with a frame that a float time cannot carry exactly (a few end on frame 0xFFFFFF88).

`events.json` holds each motion's length and events (`BobjAnimEvent`, include/battle/bobj_a.h):

* frame and attr. Bit 0 opens a hit window and bit 1 closes it.
* arg. For a hit, this is the mask of attack nodes.
* For a hit, its volume: position, w and radii, a sphere or a box by attr bit 32.

## Texts, lip sync, cameras

* **Texts** (`textconv.py`): directory entries that start with FF FE: UTF-16 text, then zeros. That makes 36107
  entries in 801 files.
* **Lip sync** (`lipsconv.py`, `LipSync_Start` in src/battle/view_b_c.c): "LIPS", 2, the key count, 0, then u16
  frame and u16 mouth open. That makes 149556 entries.
* **Cameras** (`camconv.py`, `DemoCamAnim` in include/battle/btl_demo_cam.h): "CMAn" entries. Each has channels that
  drive one word of the pose (x y z, rotation x y z, frame, -), with keys of 32 bytes. The game reads the flags, time
  and value of a key; the other 20 bytes (tangents of the tool that made them) stay in the file. That makes 2280
  entries. Floats are written in their shortest exact decimal form, so they come back bit for bit.

## Sound banks (`vagconv.py`)

A sound bank is Sony's IOP format. It is a sample data entry followed by a header entry ("IECSsreV", "IECSdaeH";
its Vagi chunk lists each sample's offset, rate and loop flag), and often a sequence entry. That makes 361 files.

The samples are SPU ADPCM in blocks of 16 bytes:

* Byte 0: shift | filter << 4.
* Byte 1: flags (1 end, 2 loop, 4 loop start).
* Then 28 four-bit steps.

Each sample becomes a 16-bit mono WAV of what the SPU plays. `samples.txt` keeps the two header bytes of every
block. The rebuild finds the steps again from the WAV: with the block's shift and filter, only one step gives each
output sample. The exception is a clamped output (±32767), where several steps can give it; those few blocks keep
their steps in `samples.txt`. For an edited WAV, the nearest step is taken.

## GSC scripts (`gscconv.py`)

A script (src/sys/gsc.c) is made of chunks. Each chunk is `{tag, 16, size, arg}`, then the data, then an "EOFC"
trailer. GSCF wraps the file. Inside it are:

* GSHD, the header words.
* GSCD, the code: one GSAC chunk per action, arg = the action id.
* GSDT, the 4-byte values that the operands point at.

`script.txt` writes the chunks as an indented tree. The words of an action are written as:

* `end`
* `cmd <id> <operand count>`
* `start <action>` and `call <action>`
* `opt <letter> <operand count>`
* `@<value index> <type>`, with the value as a `;` comment
* `word 0x...` for anything else

The values are written as `int`, `float` or `hex`. The rebuild works out every chunk size again, so commands and
values can be added or removed, as long as the script still fits in the file. All 50 scripts are converted.

## ADX streams (`adxconv.py`)

CRI ADX has a big-endian header: 0x8000, data offset − 4, encoding 3, frame size 18, 4 bits, channels, rate, sample
count, high-pass frequency. After it come frames of 18 bytes per channel, each a scale and then 32 four-bit steps.
A sample is `step × scale + (c1 × previous + c2 × the one before) >> 12`, clamped.

* `stream.wav` holds every frame's 32 samples, with the channels interleaved.
* `adx.txt` keeps the two coefficients, the frame count and every frame's scale (`scales` lines, frames in order,
  channels interleaved; `tools/assets/add_adx_scales.py` moves them there in an older extraction). The rebuild finds
  the steps again from the WAV with those scales.
* Frames whose output was clamped keep their steps in `adx.txt`.
* For an edited WAV, the nearest steps are taken with the same scales, so a much louder edit will clip.

Loose files of the disc (`disc/DATA/...`) get a folder named like the file, for example
`assets/files/disc/DATA/ZS3USOP.ADX/`. Original PSS movies are offline conversion inputs. The runtime now reads verified
lossless Matroska videos from `assets/video`; see `tools/assets/prepare_pc_media.py`.
