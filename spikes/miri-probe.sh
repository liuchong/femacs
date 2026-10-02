#!/bin/bash
# Run the mirrors' flexible-array-member paths under Miri (the one tool that can see the
# zero-size-retag class of undefined behaviour; see PORTING.md 1.3 and the lisp.rs
# `fam_base` note).  Needs a nightly toolchain with miri: `rustup component add miri
# --toolchain nightly`.  Slow tier: not part of gate-fast.
set -uo pipefail
cd "$(dirname "$0")/.."
if ! cargo +nightly miri --version > /dev/null 2>&1; then
  echo "miri-probe: skipped (no nightly miri)"
  exit 0
fi
if (cd spikes/miri && cargo +nightly miri run) > build/miri-probe.txt 2>&1; then
  echo "miri-probe: clean"
  grep -E "^(read back|addr write|char table)" build/miri-probe.txt
else
  echo "miri-probe: FAILED (undefined behaviour or build error)"
  grep -E "Undefined Behavior|^error" build/miri-probe.txt | head -5
  exit 1
fi
