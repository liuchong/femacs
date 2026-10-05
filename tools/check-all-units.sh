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
#
# The checking leg (status 3.393).  `build-tu` runs a single rustc, so the
# `#[cfg (enable_checking)]` arms - where `eassert!`/`eassume!` live - are never compiled, and
# 3.387's `E0609` hid in one of them for ten rounds (docs/status.md 3.387).  `gate-fast` does
# compile them, but only for units whose ledger is finished, which left `src/process.c` and
# `src/xdisp.c` out - and xdisp is where that defect sat.  This leg reuses the `_root.rs` build-tu
# has just materialised and re-compiles it with `--cfg enable_checking`, emitting metadata to a
# scratch directory so no `.rlib` is left behind.
#
#   * The `cd "$root/femacs"` below is what pins the toolchain (`femacs/rust-toolchain.toml`'s
#     nightly-2026-10-01).  Run from the repository root with a bare `rustc` and stable 1.98.1
#     reports 134 `E0658 c_variadic` errors across 75 of the 83 units, which both looks red and
#     *hides the real errors* behind them (measured, 2026-10-05).
#   * The reading is `checking 83 unit(s), 0 unit failure(s)`; any red is a hard failure, since the
#     class is empty today (all 83 units at 305669b, and frame/xdisp re-measured in the working
#     tree afterwards).  A failure line names whether the error is in the unit text or in a mirror
#     copy, the way P-86 item 7 requires.

# `build-tu` is the same build the other gates read objects from, and its object is what
# `check-link.py` and `extract-abi.py` measure; a unit that fails prints its last output line.
set -euo pipefail
root=$(cd "$(dirname "$0")/.." && pwd)
list=${1:-"$root/docs/baseline/frozen-build-sources.txt"}
target=${CARGO_TARGET_DIR:-/tmp/femacs-sweep}
n=0
fails=0
checking_n=0
checking_fails=0
scratch=$(mktemp -d "${TMPDIR:-/tmp}/femacs-checking.XXXXXX")
trap 'rm -rf "$scratch"' EXIT
cd "$root/femacs"
while read -r line; do
  case "$line" in \#*|"") continue ;; esac
  n=$((n + 1))
  out=$(CARGO_TARGET_DIR="$target" cargo run -q -p xtask -- build-tu "$line" 2>&1 | tail -1) || true
  case "$out" in
    *built*) ;;
    *) echo "FAIL $line: $out"; fails=$((fails + 1)); continue ;;
  esac
  stem=$(basename "$line"); stem=${stem%.*}
  # Rust crate names take no hyphens; `lib-src/make-fingerprint.c` and `src/android-emacs.c` do
  # (measured 2026-10-05: `--crate-name make-fingerprint` is rejected outright).
  crate=$(printf '%s' "$stem" | tr '-' '_')
  root_rs="$root/build/femacs/units/$stem/_root.rs"
  if [ ! -f "$root_rs" ]; then
    echo "FAIL $line: build-tu left no $root_rs to check"; checking_fails=$((checking_fails + 1)); continue
  fi
  checking_n=$((checking_n + 1))
  cout=$(rustc --edition 2024 --crate-type=lib -C panic=abort -C codegen-units=1 -C opt-level=0 \
         --cfg enable_checking --crate-name "$crate" --emit=metadata \
         -o "$scratch/$crate.rmeta" "$root_rs" 2>&1) || {
    where=$(printf '%s\n' "$cout" | grep -c "$root_rs" || true)
    echo "FAIL $line (enable_checking): $where error line(s) in the unit text, $(printf '%s\n' "$cout" | grep -c '^error' || true) error(s) total"
    printf '%s\n' "$cout" | grep '^error' | head -3 | sed 's/^/    /'
    checking_fails=$((checking_fails + 1))
  }
  # The materialised unit is ~1.5 MB and the run keeps every one of them until the end, so a full
  # sweep needs ~130 MB of scratch *at once* - which a machine under external disk pressure does
  # not have (measured twice on 2026-10-05: the run died at unit 42 and again at 45 with `No space
  # left on device`, i.e. failures that are not about the code at all).  Each unit is pruned as
  # soon as its two legs have read it unless `KEEP_UNITS=1`; the frozen 83 stems are disjoint from
  # the lanes' in-flight targets, so this never deletes a directory another lane is using.
  if [ "${KEEP_UNITS:-0}" != "1" ]; then
    rm -rf "$root/build/femacs/units/$stem"
  fi
done < "$list"
echo "checked $n unit(s), $fails failure(s)"
if [ "$checking_n" -eq "$n" ] && [ "$checking_fails" -eq 0 ]; then
  echo "checking $checking_n unit(s), 0 unit failure(s)"
else
  echo "checking $checking_n unit(s), $checking_fails unit failure(s)"
fi
[ "$fails" -eq 0 ] && [ "$checking_fails" -eq 0 ]
