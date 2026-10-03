#!/bin/bash
# Differential for the C-variadic mechanism (PORTING.md P-04): build the same three
# va_list shapes on both sides and diff.  C is plain C; the Rust side needs the pinned
# nightly toolchain from femacs/rust-toolchain.toml.  Exit status is 0 only when every
# line matches.
set -uo pipefail
cd "$(dirname "$0")/.."
OUT=build/va-probe
mkdir -p "$OUT"
clang -std=gnu23 -O1 spikes/va_probe.c -o "$OUT/c" 2> "$OUT/c.err" \
  || { echo "C build failed:"; cat "$OUT/c.err"; exit 1; }
(cd femacs && cargo run -q -p xtask -- probe ../spikes/va_probe.rs ../build/va-probe/rs) 2> "$OUT/rs.err" \
  || { echo "Rust build failed:"; cat "$OUT/rs.err"; exit 1; }
"$OUT/c" > "$OUT/c.txt" || exit 1
"$OUT/rs" > "$OUT/rs.txt" || exit 1
if diff -q "$OUT/c.txt" "$OUT/rs.txt" > /dev/null; then
  echo "va-probe: $(wc -l < "$OUT/c.txt" | tr -d ' ') lines identical"
  cat "$OUT/c.txt"
else
  echo "va-probe: DIFFERENCES"
  diff "$OUT/c.txt" "$OUT/rs.txt" | head -20
  exit 1
fi
