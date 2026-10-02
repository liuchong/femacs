#!/bin/bash
# Differential for the read accessors of buffer.h (spikes/accessor_probe.{c,rs}).  The C
# side needs the frozen configure output (see docs/spikes/2026-10-02-three-spikes.md);
# the Rust side needs only the mirrors.  Exit status is 0 only when every line matches.
set -uo pipefail
cd "$(dirname "$0")/.."
CFG=${CFG:-build/emacs-ref}
OUT=build/accessor-probe
mkdir -p "$OUT"
clang -std=gnu23 -O1 -I "$CFG/lib" -I spikes/shim -I "$CFG/src" -I emacs/src -I emacs/lib \
      spikes/accessor_probe.c -o "$OUT/c" 2> "$OUT/c.err" \
  || { echo "C build failed:"; cat "$OUT/c.err"; exit 1; }
rustc --edition 2024 -O spikes/accessor_probe.rs -o "$OUT/rs" 2> "$OUT/rs.err" \
  || { echo "Rust build failed:"; cat "$OUT/rs.err"; exit 1; }
"$OUT/c" > "$OUT/c.txt" || exit 1
"$OUT/rs" > "$OUT/rs.txt" || exit 1
if diff -q "$OUT/c.txt" "$OUT/rs.txt" > /dev/null; then
  echo "accessor-probe: $(wc -l < "$OUT/c.txt" | tr -d ' ') lines identical"
else
  echo "accessor-probe: DIFFERENCES"
  diff "$OUT/c.txt" "$OUT/rs.txt" | head -20
  exit 1
fi
