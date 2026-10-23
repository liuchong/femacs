#!/bin/bash
# Compile the P-39 shadowing probe (see spikes/shadow_probe.rs).  Exit status is 0 only
# when a unit can define what lisp.h declares.
set -uo pipefail
cd "$(dirname "$0")/.."
(cd "$HERE" && cargo run -q -p xtask -- probe ../spikes/shadow_probe.rs ../build/shadow-probe \
        2> ../build/shadow-probe.err) || { echo "shadow-probe: FAILED"; cat build/shadow-probe.err; exit 1; }
build/shadow-probe > build/shadow-probe.txt
echo "shadow-probe: $(cat build/shadow-probe.txt)"
