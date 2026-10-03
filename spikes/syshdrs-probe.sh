#!/bin/bash
# Differential for the system-header mirrors (`math.rs`, `limits.in.rs`, `float.in.rs`):
# build the same constants and macro verdicts on both sides and diff.  These mirrors have
# no reference file to pair with, so this probe is their gate (review B's B4).  The Rust
# side is compiled from femacs/ so that rust-toolchain.toml decides the toolchain.
set -uo pipefail
cd "$(dirname "$0")/.."
OUT=build/syshdrs-probe
mkdir -p "$OUT"
clang -std=gnu23 -O1 spikes/syshdrs_probe.c -o "$OUT/c" 2> "$OUT/c.err" \
  || { echo "C build failed:"; cat "$OUT/c.err"; exit 1; }
(cd femacs && cargo run -q -p xtask -- probe ../spikes/syshdrs_probe.rs ../"$OUT"/rs) 2> "$OUT/rs.err" \
  || { echo "Rust build failed:"; cat "$OUT/rs.err"; exit 1; }
"$OUT/c" > "$OUT/c.txt" || exit 1
"$OUT/rs" > "$OUT/rs.txt" || exit 1
if diff -q "$OUT/c.txt" "$OUT/rs.txt" > /dev/null; then
  echo "syshdrs-probe: $(wc -l < "$OUT/c.txt" | tr -d ' ') lines identical"
else
  echo "syshdrs-probe: DIFFERENCES"
  diff "$OUT/c.txt" "$OUT/rs.txt" | head -20
  exit 1
fi
