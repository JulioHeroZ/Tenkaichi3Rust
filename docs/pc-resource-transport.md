# Prepared PC resources instead of BPE roundtrips

In PC-only mode, the resource loader no longer BPE-compresses a prepared image
or animation entry just to have the translated game decompress it immediately.
Resources marked by `bpe.txt` and entries of reconstructed banks now retain
their prepared bytes in host memory. Game memory receives a small transient
handle; `host/resource_transport.rs` copies the prepared bytes to the requested
destination when the game requests decoding.

The first handle word preserves the expected raw size. A requested destination
is honored, a missing destination uses the same game allocator/alignment, the
optional size output is preserved, and handles can be reused. No compression
parameters or original packed padding are read for these prepared resources.
The binding survives translation regeneration through `c2rs/link.py`.

This is an internal transport bridge, **not a new distributable file format**,
not a native renderer, and not removal of all graphic/animation layout data.
The remaining binary-layout count does not decrease because of this bridge.
Only the redundant compress/decompress operation is eliminated here.

Handles belong to their running process. `bt3-assetc` rejects PC-only transport
mode, preventing accidental export of handles that would be invalid in another
process. Native file/task saves continue using their existing source behavior;
the transport is not an emulator savestate format.

Prepared payloads remain in the resource cache for handle reuse. Identical
payloads now share one retained allocation and the same handle, including when
loaded again through another resource path. Hash lookup always verifies the
complete bytes; a hash collision cannot replace another resource. Existing
handles remain valid for the lifetime of the process. Keeping them
uncompressed trades host RAM for reduced encoding/decoding work; cache eviction
and maximum-roster memory usage still need separate measurement. No overall
performance percentage or minimum hardware requirement is claimed.

`BT3_NO_BPE=1` rejects any attempted legacy BPE decode, which is useful for
coverage checks. It is not forced by the normal launcher: unconverted embedded
resources outside the recognized preparation paths may still need analysis.
`BT3_TRANSPORT_VERBOSE=1` logs prepared-resource copies.

The decoder memory-contract integration test covers a payload spanning several
old compression blocks, output length, pointer return, untouched adjacent bytes,
unchanged/reusable handles, and an omitted size pointer. It passed in release.
Technical game-run evidence is stored under `build/audits/native-text`.

On 16/10/2026 the release tests also passed with deduplicated storage:
100 repeated retains reuse one entry, forced hash collisions preserve separate
payloads and old handles, and the decoder integration test confirms identical
loads return the same token while preserving the memory-copy contract. No new
full-fight or session-memory measurement was performed for this change.

Original binary resource layouts inside skeletons and unpacked animation
entries remain explicitly pending. This bridge does not make them native data.
