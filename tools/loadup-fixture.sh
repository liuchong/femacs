#!/bin/sh
# Run a ported binary against the loadup fixture: the port's own `lisp` tree first, then every
# *subdirectory* of both trees, the reference tree last.
#
# Why the subdirectories: `lisp/` carries two kinds of file.  The *source* files are the port's
# copies, which must win, so the port's tree comes first.  The rest are the reference build's
# *generated* files - `charscript.el`, `loaddefs.el`, the ~1645 `.elc`, the four `admin/`
# products - which live in the reference tree only; a `require` of one of them (3.628/3.633:
# `international/charscript`) is found only if the reference's own subdirectories are on the
# path as well, because the subdirectory list in `load-path` comes from `subdirs.el`, which is
# itself a generated file.
#
# The fixture is a *harness*: a file loaded from the reference tree proves nothing about the
# port, and a missing generated file is not a defect in the port (3.628).  What it does give is
# a reproducible reading of how far the loadup gets.
#
#     tools/loadup-fixture.sh --batch          # the ladder reading
#     tools/loadup-fixture.sh -Q --batch -l FILE
set -eu
here=$(cd "$(dirname "$0")" && pwd)
repo=$(cd "$here/.." && pwd)
bin=${FEMACS_BIN:-$repo/build/femacs/emacs}
if [ ! -x "$bin" ]; then
  echo "loadup-fixture: $bin is not executable (link the port first)" >&2
  exit 2
fi
lp="$repo/femacs/lisp"
for d in "$repo"/femacs/lisp/*/; do lp="$lp:${d%/}"; done
lp="$lp:$repo/emacs/lisp"
for d in "$repo"/emacs/lisp/*/; do lp="$lp:${d%/}"; done
echo "loadup-fixture: $(printf '%s' "$lp" | tr ':' '\n' | wc -l | tr -d ' ') load-path entries, binary $bin" >&2
exec env EMACSLOADPATH="$lp" "$bin" "$@"
