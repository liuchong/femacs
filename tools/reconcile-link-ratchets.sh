#!/usr/bin/env bash
# Reconcile the two link ratchets against the *committed* tree.
#
# Why this exists: both ratchets (`docs/baseline/link-pending.txt` for a partial link of the
# program, `link-exec-pending.txt` for a real executable link) are `nm` readings of
# `build/femacs/*.o`, and this repository always has lane objects in flight - a unit that is being
# ported defines some of its own names and references others before defining them.  Regenerating a
# ratchet in the working tree therefore bakes the lanes' half-finished state into a file every
# later gate compares against, and the reading that counts is the one at HEAD.
#
# The same is true of the two *Swift* ratchets: `swift-units.tsv` is a reading of the tracked Swift
# text against the reference's `.m` files, and `swift-import-pending.txt` reads the Swift units'
# imports against the core's export tables - which come from the same objects as the link
# ratchets.  All four are regenerated here.
#
# So: a throwaway worktree at HEAD, the two trees the build needs symlinked in (the read-only
# reference `emacs/` and the frozen build `build/emacs-ref/` are not tracked), the current objects
# copied over, and *only* the units the working tree has in flight rebuilt from HEAD's own sources.
# Then `--strict` must be green before the ratchets are written back.
#
#     tools/reconcile-link-ratchets.sh
#
# Leaves the tree with reconciled ratchets (a `git diff` shows what changed, if anything) and no
# worktree behind.  Safe to re-run; the objects are copied, never written back to `build/femacs/`.
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
wt=${TMPDIR:-/tmp}/femacs-reconcile
target=${TMPDIR:-/tmp}/femacs-reconcile-target

cd "$root"
rm -rf "$wt"
# A previous run that was killed can leave the *registration* behind (the directory is gone), and
# `worktree add` then refuses; prune first so the script is re-runnable.
git worktree prune
# A leftover *registration whose directory exists* is not pruned (only one whose directory is
# gone is), and `worktree add` then refuses; say so instead of exiting with `--quiet` and nothing
# to read (the 3.319 failure mode).
if ! git worktree add --detach --quiet "$wt" HEAD; then
  echo "reconcile: git worktree add failed at $wt - check 'git worktree list' and remove the" >&2
  echo "reconcile: stale entry (git worktree remove --force <path>), then re-run" >&2
  exit 1
fi

# The reference checkout and the frozen build are git-ignored, so a worktree has neither.
ln -s "$root/emacs" "$wt/emacs"
mkdir -p "$wt/build"
ln -s "$root/build/emacs-ref" "$wt/build/emacs-ref"

# The objects that are already correct at HEAD: every unit the working tree has *not* touched.
mkdir -p "$wt/build/femacs"
cp "$root"/build/femacs/*.o "$wt/build/femacs/"
rm -f "$wt"/build/femacs/*.checking.o

# ...and the ones it has: rebuilt from the worktree's own (HEAD) sources.  A changed *unit* is a
# stem of its own; a changed *mirror* (`src/include/*.rs`) is not a translation unit at all
# (building it directly is 446 errors), so the units that include it are the stems instead - a
# mirror reaches its objects only through them (measured 3.279: an in-flight `src/include/xterm.rs`
# made the old rule try to build the mirror itself).
units=$(git status --porcelain -- femacs/src ':!femacs/src/include' | awk '{print $NF}')
mirrors=$(git status --porcelain -- femacs/src/include | awk '{print $NF}' | sed 's|^femacs/src/include/||')
# `grep -l` exits 1 when a changed mirror has no includer yet - a mirror landed before the unit
# that will include it (measured 3.319: the new `src/include/xlib.rs` killed this script after
# 1.6 s with *no output at all*, because `set -o pipefail` turned that 1 into a silent death).
# The `|| true` is what makes "no unit includes it yet" a reading rather than a failure.
stems=$(for u in $units; do echo "$u" | sed 's|^femacs/||' | sed 's|\.rs$|.c|'; done; \
        for m in $mirrors; do grep -l "include/$m" femacs/src/*.rs 2>/dev/null \
          | sed 's|^femacs/||' | sed 's|\.rs$|.c|' || true; done | sort -u | tr '\n' ' ')
if [ -n "$stems" ]; then
  echo "rebuilding at HEAD: $stems"
  for stem in $stems; do
    (cd "$wt/femacs" && CARGO_TARGET_DIR="$target" cargo run -q -p xtask -- build-tu "$stem" >/dev/null)
  done
fi

# Write first, then check: the post-condition is what has to hold - the ratchet in the worktree
# must be *exact* for HEAD's objects (`--strict`), and that is the reading that is copied back.
cd "$wt"
python3 tools/check-link.py --write-pending
python3 tools/check-link.py --strict
python3 tools/check-link.py --executable --write-pending
python3 tools/check-link.py --executable --strict
python3 tools/check-swift-units.py --write-manifest
python3 tools/check-swift-imports.py --write-pending

cd "$root"
for f in link-pending.txt link-exec-pending.txt swift-units.tsv swift-import-pending.txt; do
  cp "$wt/docs/baseline/$f" "docs/baseline/$f"
done
git worktree remove --force "$wt"
rm -rf "$target"
echo "reconciled: docs/baseline/{link-pending,link-exec-pending}.txt," \
     "docs/baseline/{swift-units.tsv,swift-import-pending.txt}"
