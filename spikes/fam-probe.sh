#!/bin/bash
# Run the flexible-array-member guard (spikes/fam_probe.rs).  Exit status is 0 only when
# the FAM accessors run without a bounds panic.
set -uo pipefail
cd "$(dirname "$0")/.."
mkdir -p build
(cd femacs && rustc --edition 2024 -O ../spikes/fam_probe.rs -o ../build/fam-probe \
        2> ../build/fam-probe.err) || { echo "fam-probe: FAILED to build"; cat build/fam-probe.err; exit 1; }
if build/fam-probe > build/fam-probe.txt 2>&1; then
  echo "fam-probe: $(wc -l < build/fam-probe.txt | tr -d ' ') lines ok"
  cat build/fam-probe.txt
else
  echo "fam-probe: FAILED to run"; cat build/fam-probe.txt; exit 1
fi
