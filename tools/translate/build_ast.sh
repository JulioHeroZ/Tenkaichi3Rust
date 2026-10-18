#!/bin/sh
# Regenerates the preprocessed units, their JSON ASTs and the record layouts the translator reads.
#   DECOMP: the decompilation checkout; WORK: where tu_out/, ast/, layout/ and layout2/ go.
set -e
HERE=$(cd "$(dirname "$0")" && pwd)
DECOMP=${DECOMP:-$HOME/decomp}
WORK=${WORK:-$HOME}
python3 "$HERE/prepare_tus.py" "$DECOMP" "$WORK/tu_out" | tail -1
CF="--target=i686-linux-gnu -m32 -x cpp-output -std=gnu99 -w -ffreestanding -Wno-error=return-type -Wno-error=implicit-function-declaration -Wno-error=int-conversion -Wno-error=incompatible-function-pointer-types"
mkdir -p "$WORK/ast" "$WORK/layout"
cd "$WORK/tu_out/tu"
# layout/: the eager dump, only a fallback for records the real-layout pass does not reach (see make_layouts.py)
ls *.i | xargs -P8 -I{} sh -c "clang $CF -fsyntax-only -Xclang -ast-dump=json {} > '$WORK/ast/{}.json' 2>/dev/null; clang $CF -malign-double -fsyntax-only -Xclang -fdump-record-layouts-complete {} > '$WORK/layout/{}.txt' 2>/dev/null"
python3 "$HERE/make_layouts.py" "$WORK/tu_out/tu" "$WORK/ast" "$WORK/layout2" "$WORK/laytu"
echo ast done
