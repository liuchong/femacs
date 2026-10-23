#!/bin/bash
# Run the mirrors' flexible-array-member paths under Miri (the one tool that can see the
# zero-size-retag class of undefined behaviour; see PORTING.md 1.3 and the lisp.rs
# `fam_base` note).  Miri has to come from the *pinned* toolchain: the earlier version
# used rolling `+nightly`, which is a different (and older) compiler than the pin and
# silently turned the probe's absence into "skipped, exit 0" (review D's F6).
set -uo pipefail
cd "$(dirname "$0")/.."
PIN=$(sed -n 's/^channel[[:space:]]*=[[:space:]]*"\(.*\)"/\1/p' rust-toolchain.toml)
if [ -z "$PIN" ]; then
  echo "miri-probe: FAILED (rust-toolchain.toml has no channel = \"…\" line)"
  exit 1
fi
if ! rustup run "$PIN" cargo miri --version > /dev/null 2>&1; then
  echo "miri-probe: FAILED (miri is not installed for the pinned toolchain $PIN)"
  echo "  install it with: rustup component add miri rust-src --toolchain $PIN"
  exit 1
fi
mkdir -p build
if (cd spikes/miri && rustup run "$PIN" cargo miri run) > build/miri-probe.txt 2>&1; then
  echo "miri-probe: clean ($PIN)"
  grep -E "^(read back|addr write|char table)" build/miri-probe.txt
else
  echo "miri-probe: FAILED (undefined behaviour or build error)"
  grep -E "Undefined Behavior|^error" build/miri-probe.txt | head -5
  exit 1
fi
