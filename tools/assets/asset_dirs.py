"""Where the folder of each file of pzs3us1 is in assets/files (the same rule as game/src/host/assets.rs file_dirs):
files/pzs3us1/<NNNNN>/ as files_extract.py writes them, or grouped by kind with a description after the number
(files/lutadores/Goku (Early)/01423 roupa 1/, organize.py). The other partitions (files/pzs3us0, files/pzs3us2,
files/disc) keep their own places.
"""
import os, re
from pathlib import Path

def assets_root():
    """the assets folder the tools work on: BT3_ASSETS_DIR (the installer sets it), else the project's assets/"""
    return Path(os.environ.get("BT3_ASSETS_DIR") or Path(__file__).resolve().parents[2] / "assets")


NUMBERED = re.compile(r"^(\d{5})(?: .*)?$")
OTHER_PARTITIONS = {"pzs3us0", "pzs3us2", "disc"}


def number_of(name):
    m = NUMBERED.match(name)
    return int(m.group(1)) if m else None


def index(assets):
    """{file number: folder} of every file folder of pzs3us1"""
    out = {}

    def walk(d, depth):
        try:
            items = sorted(d.iterdir())
        except OSError:
            return
        for p in items:
            if not p.is_dir() or (depth == 0 and p.name in OTHER_PARTITIONS):
                continue
            n = number_of(p.name)
            if n is not None:
                out.setdefault(n, p)
            elif depth < 4 and not p.name.startswith(("entry_", "at_")):
                walk(p, depth + 1)

    walk(Path(assets) / "files", 0)
    return out


_cache = {}


def find(assets, number):
    """the folder of pzs3us1/<number>.bin (files/pzs3us1/<NNNNN> when it has none yet); the folders are looked up
    once per run"""
    key = str(Path(assets).resolve())
    if key not in _cache:
        _cache[key] = index(assets)
    return _cache[key].get(number, Path(assets) / "files" / "pzs3us1" / ("%05d" % number))


def having(assets, name):
    """[(number, folder)] of the file folders that have a file `name`, by number"""
    return sorted((n, d) for n, d in index(assets).items() if (d / name).is_file())
