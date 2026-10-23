#!/bin/bash
# Differential for the places the buffer.h macros denote: build the same statement
# sequence on both sides and diff.  The C side needs the frozen configure output (see
# docs/spikes/2026-10-02-three-spikes.md); the Rust side needs only the mirrors.  Exit
# status is 0 only when every line matches.
set -uo pipefail
cd "$(dirname "$0")/.."
CFG=${CFG:-build/emacs-ref}
OUT=build/write-probe
mkdir -p "$OUT"
clang -std=gnu23 -O1 -I "$CFG/lib" -idirafter spikes/shim -I "$CFG/src" -I emacs/src -I emacs/lib \
      spikes/write_probe.c -o "$OUT/c" 2> "$OUT/c.err" \
  || { echo "C build failed:"; cat "$OUT/c.err"; exit 1; }
(cd "$HERE" && cargo run -q -p xtask -- probe ../spikes/write_probe.rs ../"$OUT"/rs) 2> "$OUT/rs.err" \
  || { echo "Rust build failed:"; cat "$OUT/rs.err"; exit 1; }
"$OUT/c" > "$OUT/c.txt" || exit 1
"$OUT/rs" > "$OUT/rs.txt" || exit 1
if diff -q "$OUT/c.txt" "$OUT/rs.txt" > /dev/null; then
  echo "write-probe: $(wc -l < "$OUT/c.txt" | tr -d ' ') lines identical"
else
  echo "write-probe: DIFFERENCES"
  diff "$OUT/c.txt" "$OUT/rs.txt" | head -20
  exit 1
fi
