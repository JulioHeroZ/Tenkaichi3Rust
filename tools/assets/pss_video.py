"""The video of the game's movies (DATA/ZS3USOP.PSS, ZS3USED.PSS: MPEG program streams) as MPEG-2 elementary video
streams in assets/video (zs3usop.m2v, zs3used.m2v): the very pictures of the disc, in a standard format that video
players open, decoded by the game's own Rust decoder (game/src/host/mpeg2.rs). The sound of the movies is the ADX
stream next to them, already a WAV (assets/audio/streams). No re-encoding: the pictures are not changed.

    python tools/assets/pss_video.py [assets]
"""
import struct, sys
from pathlib import Path


def demux(d, stream_id=0xE0):
    """the payload of the PES packets of one stream of an MPEG-1/2 program stream"""
    out = bytearray()
    i, n = 0, len(d)
    while i + 4 <= n:
        if d[i:i + 3] != b"\0\0\1":
            j = d.find(b"\0\0\1", i + 1)
            if j < 0:
                break
            i = j
            continue
        code = d[i + 3]
        if code == 0xBA:  # pack header
            if d[i + 4] >> 6 == 1:  # MPEG-2
                i += 14 + (d[i + 13] & 7)
            else:
                i += 12
        elif code == 0xB9:  # end of the program stream
            break
        elif code >= 0xBB:
            length = struct.unpack_from(">H", d, i + 4)[0]
            start, end = i + 6, i + 6 + length
            if code == stream_id:
                p = start
                if d[p] >> 6 == 2:  # MPEG-2 PES header
                    p += 3 + d[p + 2]
                else:  # MPEG-1: stuffing, STD buffer, time stamps
                    while d[p] == 0xFF:
                        p += 1
                    if d[p] >> 6 == 1:
                        p += 2
                    p += {2: 5, 3: 10}.get(d[p] >> 4, 1)
                out += d[p:end]
            i = end
        else:
            i += 4
    return bytes(out)


def main():
    root = Path(__file__).resolve().parents[2]
    assets = Path(sys.argv[1]) if len(sys.argv) > 1 else root / "assets"
    out = assets / "video"
    out.mkdir(parents=True, exist_ok=True)
    for name in ("ZS3USOP", "ZS3USED"):
        src = assets / "_legacy" / "raw" / "disc" / "DATA" / ("%s.PSS" % name)
        video = demux(src.read_bytes())
        if not video.startswith(b"\0\0\1\xb3"):
            raise SystemExit("%s: no MPEG video sequence" % src)
        target = out / ("%s.m2v" % name.lower())
        tmp = target.with_name(target.name + ".tmp")
        tmp.write_bytes(video)
        tmp.replace(target)
        print("%s: %.1f MB de vídeo MPEG-2" % (target.name, len(video) / 1e6))


if __name__ == "__main__":
    main()
