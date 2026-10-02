#!/bin/bash
# Expand one representative call per macro of the mirrors (spikes/macro_probe.rs) and
# compile metadata-only.  Exit status is 0 only when every expansion type-checks.
set -uo pipefail
cd "$(dirname "$0")/.."
mkdir -p build
if rustc --edition 2024 --crate-type lib --emit metadata spikes/macro_probe.rs \
        -o build/macro-probe.rmeta 2> build/macro-probe.err; then
  echo "macro-probe: every expansion compiles"
else
  echo "macro-probe: FAILED"
  grep -E "^(error|error\[)" -A 4 build/macro-probe.err | head -30
  exit 1
fi
