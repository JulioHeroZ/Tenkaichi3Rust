#!/bin/sh
# Translates every unit into the crate (game by default) and lays out its data.
set -e
HERE=$(cd "$(dirname "$0")" && pwd)
DECOMP=${DECOMP:-$HOME/decomp}
WORK=${WORK:-$HOME}
OUT=${OUT:-$HERE/../../game}
python3 "$HERE/c2rs/driver.py" "$WORK/tu_out/tu" "$WORK/ast" "$WORK/layout2" "$OUT"
python3 "$HERE/c2rs/link.py" "$OUT" "$DECOMP"
