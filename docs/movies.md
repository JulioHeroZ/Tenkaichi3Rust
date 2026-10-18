# Movies

The opening and the ending (DATA/ZS3USOP.PSS, ZS3USED.PSS on the disc) are MPEG program streams: MPEG-2 video and,
beside them, the sound as ADX (DATA/ZS3USOP.ADX, ZS3USED.ADX).

* `tools/assets/pss_video.py` takes the video stream out of each PSS, unchanged, as `assets/video/zs3usop.m2v` and
  `zs3used.m2v`: a standard MPEG-2 elementary stream that video players open.
* `game/src/host/mpeg2.rs` decodes it in Rust, without libraries. The two movies are intra-coded only (every picture
  an I picture; 2810 and 4200 pictures, 512x448, 30 per second), and that is what the decoder handles: sequence
  header and extensions, picture coding extension, slices, intra macroblocks, inverse quantisation with
  saturation and mismatch control, and a precise inverse DCT. A stream with P or B pictures is refused.
  Against FFmpeg's decoding of the same streams every sample is within 1 (1.1% differ by 1: inverse DCT rounding).
* The pictures become RGB as the PS2's IPU does (ITU-R BT.601, each chroma sample for its 2x2 pixels).
  `game/src/host/movie.rs` hands them to the game in its 16x16 blocks; the game's own audio player (the ADX stream as
  a WAV, assets/audio/streams), GS upload chains, START to skip and fades stay in use.
* `BT3_MOVIE_PREVIEW=opening` (or `ending`) plays one movie after the start-up and exits.
