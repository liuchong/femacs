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
# `FEMACS_CWD` sets the working directory (default: the repository root).  The dump path needs it:
# `Snarf-documentation` opens `../etc/DOC`, a path relative to the *build* src directory, and the
# reference build's own DOC is at `build/emacs-ref/etc/DOC` - so a dump run wants
# `FEMACS_CWD=$repo/build/emacs-ref/src`.
#
#     tools/loadup-fixture.sh --batch          # the ladder reading
#     tools/loadup-fixture.sh --batch --temacs=pdump
#     FEMACS_CWD=$PWD/build/emacs-ref/src tools/loadup-fixture.sh --batch --temacs=pdump
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
cd "${FEMACS_CWD:-$repo}"
echo "loadup-fixture: $(printf '%s' "$lp" | tr ':' '\n' | wc -l | tr -d ' ') load-path entries, binary $bin, cwd $(pwd)" >&2
exec env EMACSLOADPATH="$lp" "$bin" "$@"
