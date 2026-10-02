#!/bin/bash
# Differential for the inline functions a header mirror carries: build the same
# corpus on both sides and diff.  The C side needs the frozen configure output
# (see docs/spikes/2026-10-02-three-spikes.md); the Rust side needs only the
# mirrors.  The Rust side is compiled from femacs/ so that
# femacs/rust-toolchain.toml decides the toolchain (review D's F1: a bare rustc
# at the repository root resolved the stable toolchain and broke three probes).
# Exit status is 0 only when every line matches.
set -uo pipefail
cd "$(dirname "$0")/.."
CFG=${CFG:-build/emacs-ref}
OUT=build/inline-probe
mkdir -p "$OUT"
clang -std=gnu23 -O1 -I "$CFG/lib" -idirafter spikes/shim -I "$CFG/src" -I emacs/src -I emacs/lib \
      spikes/inline_probe.c -o "$OUT/c" 2> "$OUT/c.err" \
  || { echo "C build failed:"; cat "$OUT/c.err"; exit 1; }
(cd femacs && rustc --edition 2024 -O ../spikes/inline_probe.rs -o ../"$OUT"/rs) 2> "$OUT/rs.err" \
  || { echo "Rust build failed:"; cat "$OUT/rs.err"; exit 1; }
"$OUT/c" > "$OUT/c.txt" || exit 1
"$OUT/rs" > "$OUT/rs.txt" || exit 1
if diff -q "$OUT/c.txt" "$OUT/rs.txt" > /dev/null; then
  echo "inline-probe: $(wc -l < "$OUT/c.txt" | tr -d ' ') lines identical"
else
  echo "inline-probe: DIFFERENCES"
  diff "$OUT/c.txt" "$OUT/rs.txt" | head -20
  exit 1
fi
