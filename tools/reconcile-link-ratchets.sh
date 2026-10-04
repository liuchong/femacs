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
git worktree add --detach --quiet "$wt" HEAD

# The reference checkout and the frozen build are git-ignored, so a worktree has neither.
ln -s "$root/emacs" "$wt/emacs"
mkdir -p "$wt/build"
ln -s "$root/build/emacs-ref" "$wt/build/emacs-ref"

# The objects that are already correct at HEAD: every unit the working tree has *not* touched.
mkdir -p "$wt/build/femacs"
cp "$root"/build/femacs/*.o "$wt/build/femacs/"
rm -f "$wt"/build/femacs/*.checking.o

# ...and the ones it has: rebuilt from the worktree's own (HEAD) sources.
stems=$(git status --porcelain -- femacs/src | awk '{print $NF}' | sed 's|^femacs/||' | sed 's|\.rs$|.c|')
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

cd "$root"
cp "$wt/docs/baseline/link-pending.txt" docs/baseline/link-pending.txt
cp "$wt/docs/baseline/link-exec-pending.txt" docs/baseline/link-exec-pending.txt
git worktree remove --force "$wt"
rm -rf "$target"
echo "reconciled: docs/baseline/link-pending.txt, docs/baseline/link-exec-pending.txt"
