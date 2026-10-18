"""The bytes of an extracted file that its pictures, sounds, texts and lip sync do not give (what skeleton.bin held
before the extraction wrote those bytes as typed records): for the converters of the native PC resources
(convert_native_*.py), which check that a file holds nothing but what they convert.

skeleton.bin when the folder still has one; otherwise zeros of the size skeleton.txt gives, with the typed records
(layout.py: file-tables.json, sound-tables.json...) written, the byte order marks of texts.json and the headers
"LIPS", 2, 0, 0 of lips.json (the key count left zero, as skeleton.bin had it). None when the folder has neither.
"""
import json
from pathlib import Path

import layout

LAYOUT_FILES = ["texture-entries.json", "camera-tables.json", "stage-layout.json", "surface-tables.json",
                "model-tables.json", "effect-tables.json", "sound-tables.json", "file-tables.json"]


def skeleton(directory):
    directory = Path(directory)
    if (directory / "skeleton.bin").is_file():
        return (directory / "skeleton.bin").read_bytes()
    txt = directory / "skeleton.txt"
    if not txt.is_file():
        return None
    size = next(int(l.split()[1]) for l in txt.read_text().splitlines() if l.startswith("zeros "))
    buf = bytearray(size)
    for name in LAYOUT_FILES:
        if (directory / name).is_file():
            layout.apply(buf, (directory / name).read_text(encoding="utf-8"))
    if (directory / "texts.json").is_file():
        for e in json.loads((directory / "texts.json").read_text(encoding="utf-8"))["texts"]:
            if e.get("bom") == 1:
                buf[e["at"] - 2:e["at"]] = b"\xff\xfe"
    if (directory / "lips.json").is_file():
        for e in json.loads((directory / "lips.json").read_text(encoding="utf-8"))["lips"]:
            buf[e["at"]:e["at"] + 8] = b"LIPS\x02\0\0\0"
    return bytes(buf)
