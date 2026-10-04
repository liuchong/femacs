#!/usr/bin/env bash
# Build every unit the frozen configuration compiles, and report the failures.
#
# Why this exists (status 3.375): a mirror change's verification set is *every unit that includes
# it*, and nothing was reading that set.  `reconcile-link-ratchets.sh` rebuilds only the includers
# of *in-flight* mirrors, so a committed mirror change was never checked against all of its
# includers, and two defects survived that way in one session - `SET_FRAME_GARBAGED`'s cast (3.344)
# and `window_part` in include/window.rs (3.375), which broke `editfns`/`insdel`/`textprop` for ten
# commits until a ratchet reconciliation tripped over it.
#
#     tools/check-all-units.sh [list]      # default: docs/baseline/frozen-build-sources.txt
#
# The reading this project wants is "83 unit(s), 0 failure(s)" at HEAD.  Run it from a worktree at
# HEAD for HEAD's number: in the main working tree the same run includes the lanes' in-flight
# edits, which is a *different* reading (3.339's rule about the two readings).
#
# `build-tu` is the same build the other gates read objects from, and its object is what
# `check-link.py` and `extract-abi.py` measure; a unit that fails prints its last output line.
set -euo pipefail
root=$(cd "$(dirname "$0")/.." && pwd)
list=${1:-"$root/docs/baseline/frozen-build-sources.txt"}
target=${CARGO_TARGET_DIR:-/tmp/femacs-sweep}
n=0
fails=0
cd "$root/femacs"
while read -r line; do
  case "$line" in \#*|"") continue ;; esac
  n=$((n + 1))
  out=$(CARGO_TARGET_DIR="$target" cargo run -q -p xtask -- build-tu "$line" 2>&1 | tail -1) || true
  case "$out" in
    *built*) ;;
    *) echo "FAIL $line: $out"; fails=$((fails + 1)) ;;
  esac
done < "$list"
echo "checked $n unit(s), $fails failure(s)"
[ "$fails" -eq 0 ]
