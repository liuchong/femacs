#!/bin/bash
# Build and run the three spikes once, then leave everything under build/ so the
# caller can copy the report out and delete the products.  Nothing here writes to
# the reference checkout.
set -uo pipefail
cd "$(dirname "$0")/.."
REF=emacs
CFG=build/emacs-ref
OUT=build/spikes-out
mkdir -p "$OUT"

echo "# spike report $(date -u +%Y-%m-%dT%H:%M:%SZ)" > "$OUT/report.txt"
echo "rustc: $(rustc --version)" >> "$OUT/report.txt"
echo "clang: $(clang --version | head -1)" >> "$OUT/report.txt"
echo >> "$OUT/report.txt"

echo "## spike 1: layout identity (C vs Rust)" >> "$OUT/report.txt"
clang -std=gnu23 -O0 -I "$CFG/lib" -I spikes/shim -I "$CFG/src" \
      -I "$REF/src" -I "$REF/lib" \
      spikes/layout_c.c -o "$OUT/layout_c" 2> "$OUT/layout_c.err" \
  && "$OUT/layout_c" > "$OUT/layout_c.txt" \
  || { echo "C build failed:" >> "$OUT/report.txt"; cat "$OUT/layout_c.err" >> "$OUT/report.txt"; }
rustc --edition 2024 -O spikes/layout.rs -o "$OUT/layout_rs" 2> "$OUT/layout_rs.err" \
  && "$OUT/layout_rs" > "$OUT/layout_rs.txt" \
  || { echo "Rust build failed:" >> "$OUT/report.txt"; cat "$OUT/layout_rs.err" >> "$OUT/report.txt"; }
if [ -s "$OUT/layout_c.txt" ] && [ -s "$OUT/layout_rs.txt" ]; then
  python3 spikes/compare_layout.py "$OUT/layout_c.txt" "$OUT/layout_rs.txt" >> "$OUT/report.txt" 2>&1 \
    && echo "=> every shared size and offset matches" >> "$OUT/report.txt" \
    || echo "=> MISMATCHES above" >> "$OUT/report.txt"
fi
echo >> "$OUT/report.txt"

echo "## spike 2: setjmp/longjmp across Rust frames" >> "$OUT/report.txt"
rustc --edition 2024 -O spikes/jump.rs -o "$OUT/jump" 2> "$OUT/jump.err" \
  && "$OUT/jump" >> "$OUT/report.txt" 2>&1 \
  || { echo "build failed:" >> "$OUT/report.txt"; cat "$OUT/jump.err" >> "$OUT/report.txt"; }
echo >> "$OUT/report.txt"

echo "## spike 3: conservative stack scan of Rust frames" >> "$OUT/report.txt"
for opt in 0 2 3; do
  SPIKE_OPT="-$opt" rustc --edition 2024 -C opt-level=$opt -C debuginfo=0 spikes/scan.rs \
      -o "$OUT/scan$opt" 2> "$OUT/scan$opt.err" \
    || { echo "build failed at -O$opt:" >> "$OUT/report.txt"; cat "$OUT/scan$opt.err" >> "$OUT/report.txt"; continue; }
  "$OUT/scan$opt" >> "$OUT/report.txt" 2>&1
done
python3 spikes/to-baseline.py "$OUT/layout_c.txt" \
        docs/baseline/layout/c-layout-darwin-arm64.tsv >> "$OUT/report.txt" 2>&1
echo "## jmp_buf sizes reported by the C side" >> "$OUT/report.txt"
grep -E "^sizeof (jmp_buf|sigjmp_buf)" "$OUT/layout_c.txt" >> "$OUT/report.txt" 2>/dev/null
cat "$OUT/report.txt"
