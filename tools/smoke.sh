#!/bin/sh
# A platform smoke: one command, one line per check, differential against the reference.
#
# The point of DoD-6 is a reading, not a checklist: every check is a Lisp form whose printed
# result must match the reference's byte for byte, so a platform is "smoked" when this script
# says so on that platform's build.  The forms avoid anything that legitimately names the
# product (emacs-version's string, subr descriptions) and anything display-dependent, because
# those differ between binaries by design or need a real terminal.
#
#   tools/smoke.sh                          # the port's dumped program against the reference's
#   tools/smoke.sh --prog build/port/src/temacs   # e.g. the undumped program, or an Android build
#
# Exit status is the number of failing checks (0 = smoke passed).

set -u
REPO=$(cd "$(dirname "$0")/.." && pwd)
PROG="$REPO/build/port/src/emacs"
REF="$REPO/build/emacs-ref/src/emacs"
MEM=700

while [ $# -gt 0 ]; do
  case "$1" in
    --prog) PROG=$2; shift 2 ;;
    --ref)  REF=$2;  shift 2 ;;
    --memory-mib) MEM=$2; shift 2 ;;
    *) echo "unknown argument: $1" >&2; exit 2 ;;
  esac
done

[ -x "$PROG" ] || { echo "missing program: $PROG" >&2; exit 2; }
[ -x "$REF" ]  || { echo "missing reference: $REF" >&2; exit 2; }

# The checks, in order.  Each is a single form that prints one line.
CHECKS='
(list 1 2 3)
(with-temp-buffer (insert "abc") (buffer-string))
(mapcar (function 1+) (list 1 2 3))
(sort (list 3 1 2) (function <))
(string-match "a*b" "aaab")
(replace-regexp-in-string "x+" "-" "axxbxx")
(let ((f (make-temp-file "smoke"))) (with-temp-file f (insert "x")) (with-temp-buffer (insert-file-contents f) (delete-file f) (buffer-string)))
(subrp (symbol-function (quote car)))
(length (garbage-collect))
(coding-system-p (quote utf-8-unix))
(char-width ?\u00e5)
'

run_one() {	# prog form
  # shellcheck disable=SC2086
  python3 "$HOME/.assistant/skills/controlled-run/scripts/run.py" --profile runtime \
    --memory-mib "$MEM" -- sh -c \
    "HOME=/nonexistent PATH=/usr/bin:/bin LC_ALL=C '$1' --no-init-file --no-site-file --no-site-lisp --batch --eval '(prin1 $2)' 2>/dev/null" \
    | grep -v '^controlled-run: '
}

fail=0
n=0
old_ifs=$IFS
IFS='
'
for form in $CHECKS; do
  [ -n "$form" ] || continue
  n=$((n + 1))
  port=$(run_one "$PROG" "$form")
  ref=$(run_one "$REF" "$form")
  if [ "$port" = "$ref" ]; then
    printf 'ok   %2d  %s\n' "$n" "$form"
  else
    printf 'DIFF %2d  %s\n   port: %s\n   ref : %s\n' "$n" "$form" "$port" "$ref"
    fail=$((fail + 1))
  fi
done
IFS=$old_ifs

printf '%d check(s), %d failure(s)\n' "$n" "$fail"
exit "$fail"
