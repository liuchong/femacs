#!/bin/bash
# L1's worklist: byte-compile a Lisp directory with the port's own program, one
# `Compiling <file>...` line per file plus the compiler's own error text, then summarize.
#
#   tools/l1-worklist.sh lisp/emacs-lisp              # every *.el in the directory
#   tools/l1-worklist.sh lisp/emacs-lisp bindat.el    # named files only
#
# Why a worklist rather than a gate: the port's byte-compiler is the gate for L1
# (`lisp/emacs-lisp/*.elc` all load) and for the completion definition's generated-artefact
# parity (the reference generates its Lisp with byte-compiled generators), and the failures are
# per file, not systemic - the same set contains files that produce byte-identical `.elc` and
# files the compiler rejects.  Measured 2026-10-16 (docs/status.md 4.373): 110 files -> 38
# `.elc`, 47 files with errors whose dominant class is `Invalid function: #[...]`, plus a SIGBUS
# around the 66th file.
#
# Run it from the port's tree (the default directory argument is inside it), because the `.elc`
# files land next to their sources, and the relative name a real build writes is what the
# `.elc` byte-comparison against the reference needs.  A worklist is not a gate: the exit status
# is 0 and the readings are the line it prints.
set -uo pipefail
REPO=$(cd "$(dirname "$0")/.." && pwd)
EMACS=${EMACS:-$REPO/build/port/src/temacs}
DIR=${1:?usage: l1-worklist.sh <lisp-directory> [file.el ...]}
shift
cd "$DIR" || exit 2
LOG=${LOG:-$REPO/build/l1-worklist.log}
mkdir -p "$(dirname "$LOG")"
files=("$@")
if [ ${#files[@]} -eq 0 ]; then files=(./*.el); fi
"$EMACS" -batch --no-site-file --no-site-lisp \
    --eval '(setq byte-compile-verbose t)' -f batch-byte-compile "${files[@]}" > "$LOG" 2>&1
status=$?
elc=$(ls ./*.elc 2>/dev/null | wc -l | tr -d ' ')
# The log carries raw byte-code strings (NUL bytes), so a plain `grep -c` counts segments, not
# lines: count the *distinct files* the compiler announced instead.
reported=$(grep -a -o '^Compiling [^ ]*' "$LOG" | sed 's/\.\.\.done$//; s/\.\.\.$//; s/.*\///' \
    | sort -u | wc -l | tr -d ' ')
errors=$(grep -ac 'Error:' "$LOG")
echo "l1-worklist: exit $status, $reported file(s) reported, $elc .elc present, $errors error line(s)"
echo "log: ${LOG#"$REPO"/}"
exit 0
