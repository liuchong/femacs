#!/bin/sh
# Differential acceptance: one Lisp file, one run per binary, stdout/stderr/exit code compared.
#
# Why (status 4.891): DoD-3's last line asks for differential runs ("随机 Lisp 程序差分对拍
# （stdout/stderr/退出码逐字节一致）"), and the frozen reference build runs on this host
# (`build/emacs-ref/src/emacs`), so the comparison needs no device and no Android GUI.  Each form
# in the file is evaluated in *both* programs with the same `--batch -Q` command line; the two
# outputs must be byte-identical.
#
#   tools/differential.sh                                  # docs/baseline/differential-forms.el
#   tools/differential.sh --forms FILE [--prog PATH] [--ref PATH] [--stderr] [--keep]
#
# Prints one line per mismatch, then a summary; exit status is the number of mismatching forms.
set -u
REPO=$(cd "$(dirname "$0")/.." && pwd)
PROG="$REPO/build/port/src/emacs"
REF="$REPO/build/emacs-ref/src/emacs"
FORMS="$REPO/docs/baseline/differential-forms.el"
KEEP=0
STDERR=0
while [ $# -gt 0 ]; do
  case "$1" in
    --forms) FORMS=$2; shift 2 ;;
    --prog) PROG=$2; shift 2 ;;
    --ref) REF=$2; shift 2 ;;
    --keep) KEEP=1; shift ;;
    --stderr) STDERR=1; shift ;;
    *) echo "unknown argument: $1" >&2; exit 2 ;;
  esac
done
[ -x "$PROG" ] || { echo "missing program: $PROG" >&2; exit 2; }
[ -x "$REF" ] || { echo "missing reference: $REF" >&2; exit 2; }
[ -f "$FORMS" ] || { echo "missing forms file: $FORMS" >&2; exit 2; }

WORK=$(mktemp -d "${TMPDIR:-/tmp}/femacs-diff.XXXXXX")
trap '[ "$KEEP" = 1 ] || rm -rf "$WORK"' EXIT

# Each form is one line so the files stay diffable; a form that prints is compared by its output.
n=0
bad=0
while IFS= read -r form; do
  case "$form" in \#*|"") continue ;; esac
  n=$((n + 1))
  printf '%s\n' "$form" > "$WORK/form.el"
  HOME="$WORK" "$REF" --batch -Q -l "$WORK/form.el" > "$WORK/ref.out" 2> "$WORK/ref.err"
  ref_status=$?
  HOME="$WORK" "$PROG" --batch -Q -l "$WORK/form.el" > "$WORK/por.out" 2> "$WORK/por.err"
  por_status=$?
  # stderr is compared only with `--stderr`: a run's stderr carries environment noise (the
  # reference tree's own stale `.elc` warnings, for one), and mixing it into every form would
  # bury the stdout differences that are the point.  Exit status is always compared.
  stderr_diff=0
  if [ "$STDERR" = 1 ] && ! cmp -s "$WORK/ref.err" "$WORK/por.err"; then stderr_diff=1; fi
  if ! cmp -s "$WORK/ref.out" "$WORK/por.out" || [ "$stderr_diff" = 1 ] \
     || [ "$ref_status" != "$por_status" ]; then
    bad=$((bad + 1))
    echo "MISMATCH form $n: $form"
    echo "  status: ref=$ref_status port=$por_status"
    diff -u "$WORK/ref.out" "$WORK/por.out" 2>/dev/null | sed -n '3,8p' | sed 's/^/  out /'
    if [ "$STDERR" = 1 ]; then
      diff -u "$WORK/ref.err" "$WORK/por.err" 2>/dev/null | sed -n '3,8p' | sed 's/^/  err /'
    fi
  fi
done < "$FORMS"
echo "differential: $n form(s), $bad mismatch(es)"
exit "$bad"
