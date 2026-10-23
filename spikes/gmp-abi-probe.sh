#!/bin/bash
# The C ABI of lib/include/mini-gmp.rs, checked against the reference header.
#
# The mirror is a hand-mapped transcription of a 314-line header with 159 prototypes, and
# the pair's checklist has no function *definitions*, so --check cannot see a wrong
# parameter width, const-ness or return type.  This probe turns the mirror's own
# declarations back into C (`static RET (*p_NAME)(ARGS) = NAME;`) and compiles them against
# emacs/lib/mini-gmp.h: any mismatch is an incompatible-pointer-type error.  It needs no
# configure output, only the reference header.
set -uo pipefail
cd "$(dirname "$0")/.."
OUT=build/gmp-abi-probe
mkdir -p "$OUT"
python3 tools/gen-gmp-abi-probe.py > "$OUT/harness.c" || exit 1
n=$(grep -c '^static .*(\*p_' "$OUT/harness.c")
# A *type* check, not a link: the harness takes the address of every declaration, so an
# incompatible-pointer-type error means the mirror's type differs from the reference's.
clang -std=gnu23 -fsyntax-only -Werror=incompatible-pointer-types -include stdio.h \
      -I emacs/lib "$OUT/harness.c" 2> "$OUT/c.err" \
  || { echo "gmp-abi-probe: declarations do not type-check:"; head -20 "$OUT/c.err"; exit 1; }
echo "gmp-abi-probe: $n declarations type-check against mini-gmp.h"
