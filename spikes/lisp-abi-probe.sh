#!/bin/bash
# Type-check lisp.rs's `extern "C"` declarations against the reference lisp.h.
#
# `extract-abi.py` compares names, so a mirror declaration can carry the wrong type
# indefinitely: round 6's reviewer B found six (`make_string`, `make_unibyte_string`,
# `intern_1`, `intern_c_string_1`, `fast_c_string_match_internal`, `egetenv_internal` all
# said `usize` where lisp.h says `ptrdiff_t`).  This probe turns the mirror's own
# declarations back into C and compiles them with -Werror=incompatible-pointer-types, so a
# wrong width, const-ness or return type is an error.  Needs the frozen configure output
# (build/emacs-ref), like the other C differentials.
set -uo pipefail
cd "$(dirname "$0")/.."
OUT=build/lisp-abi-probe
mkdir -p "$OUT"
if [ ! -f build/emacs-ref/src/config.h ]; then
  echo "lisp-abi-probe: FAILED (build/emacs-ref/src/config.h is missing; see docs/spikes/2026-10-02-three-spikes.md)"
  exit 1
fi
python3 tools/gen-lisp-abi-probe.py > "$OUT/harness.c" 2> "$OUT/gen.err" \
  || { echo "lisp-abi-probe: generation failed:"; cat "$OUT/gen.err"; exit 1; }
n=$(grep -c '^static' "$OUT/harness.c")
clang -std=gnu23 -fsyntax-only -Werror=incompatible-pointer-types \
      -I build/emacs-ref/src -I emacs/src -I emacs/lib -I build/emacs-ref/lib \
      "$OUT/harness.c" 2> "$OUT/c.err" \
  || { echo "lisp-abi-probe: declarations do not type-check:"; head -20 "$OUT/c.err"; exit 1; }
echo "lisp-abi-probe: $n declarations type-check against lisp.h" && grep -o "[0-9]* unmapped, [0-9]* baselined" "$OUT/gen.err" | sed "s/^/  (/" | sed "s/$/)/"
