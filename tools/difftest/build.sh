#!/bin/sh
# Builds the reference: the same preprocessed C the translator reads, compiled for i386 with the PS2 float
# routines (-mno-x87: every float operation is a softfloat_ps2 call), function values = function ids, data tables
# at the translated program's addresses. Needs clang and ld.lld.
#   DECOMP: decompilation checkout; WORK: holds tu_out/ (tools/translate/build_ast.sh); BT3_GEN: translated crate.
set -e
HERE=$(cd "$(dirname "$0")" && pwd)
DECOMP=${DECOMP:-$HOME/decomp}
WORK=${WORK:-$HOME}
GEN=${BT3_GEN:-$HERE/../../game}
OUT=${BT3_REF:-$HERE/build}
mkdir -p "$OUT/obj" "$OUT/ll"
RES=$(clang -print-resource-dir)/include
SYS="-nostdinc -isystem $HERE/../translate/sysinc -isystem $RES -I$DECOMP/src -I$DECOMP/include -I$DECOMP/port/src"
T="--target=i686-linux-gnu -m32 -ffreestanding -fno-builtin -fno-pic -w"
clang $T -O2 -c "$HERE/harness.c" -o "$OUT/harness.o"
clang $T -O2 -c "$HERE/hostmath.c" -o "$OUT/hostmath.o"
clang $T -O2 $SYS -c "$DECOMP/port/src/softfloat_ps2.c" -o "$OUT/softfloat.o"
clang $T -O2 $SYS -include "$HERE/hm.h" -c "$DECOMP/port/src/plat_libm.c" -o "$OUT/libm_bridge.o"
CF="$T -x cpp-output -std=gnu99 -O1 -fno-strict-aliasing -fwrapv -fno-common -fdata-sections -ffunction-sections -fno-stack-protector -ffp-contract=off -mno-x87 -mno-sse -mno-mmx -malign-double -ftrivial-auto-var-init=zero -Wno-error=return-type -Wno-error=implicit-function-declaration -Wno-error=int-conversion -Wno-error=incompatible-function-pointer-types"
export BT3_GEN="$GEN"
for f in "$WORK"/tu_out/tu/*.i; do echo "$f"; done | xargs -P4 -I{} sh -c '
  f={}; b=$(basename "$f" .i); tu=${b##*__}
  clang '"$CF"' -S -emit-llvm "$f" -o "'"$OUT"'/ll/$b.ll" &&
  python3 "'"$HERE"'/fidify.py" "'"$OUT"'/ll/$b.ll" "'"$OUT"'/ll/$b.fid.ll" "$tu" &&
  clang '"$T"' -O1 -mno-x87 -mno-sse -mno-mmx -fdata-sections -ffunction-sections -c "'"$OUT"'/ll/$b.fid.ll" -o "'"$OUT"'/obj/$b.o" || echo "FAILED $f"'
OBJS="$OUT/harness.o $OUT/softfloat.o $OUT/libm_bridge.o $OUT/hostmath.o $OUT/obj/*.o"
ld.lld -m elf_i386 -static -e _start --error-limit=0 $OBJS -o "$OUT/ref32" 2>&1 | sed -n 's/.*undefined symbol: //p' | sort -u > "$OUT/undef.txt" || true
python3 "$HERE/gen_stubs.py" "$GEN/src/gen/sym.rs" "$OUT/undef.txt" "$OUT/stubs.c" "$OUT/defsyms.txt"
clang $T -O2 -c "$OUT/stubs.c" -o "$OUT/stubs.o"
ld.lld -m elf_i386 -static -e _start --image-base=0x60000000 -Map="$OUT/ref32.map" $(cat "$OUT/defsyms.txt") \
  "$OUT/harness.o" "$OUT/stubs.o" "$OUT/softfloat.o" "$OUT/libm_bridge.o" "$OUT/hostmath.o" "$OUT"/obj/*.o -o "$OUT/ref32"
echo "reference built: $OUT/ref32"
