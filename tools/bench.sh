#!/bin/sh
# The performance table of docs/acceptance.md, re-measurable on demand.
#
# Every row is min-of-five on both programs, same machine, same -O2 build; the bytecode rows
# run *compiled* functions (source-level loops measure eval_sub instead, which is a different
# layer and was mistaken for the VM once - PORTING.md P-180/P-181).
#
#   tools/bench.sh                     # port against the reference
#   tools/bench.sh --prog build/port/src/emacs --ref build/emacs-ref/src/emacs
#
# Exit status is 0 (this reports, it does not gate: DoD-7's target is a judgement call).
set -u
REPO=$(cd "$(dirname "$0")/.." && pwd)
PROG="$REPO/build/port/src/emacs"
REF="$REPO/build/emacs-ref/src/emacs"
while [ $# -gt 0 ]; do
  case "$1" in
    --prog) PROG=$2; shift 2 ;;
    --ref)  REF=$2;  shift 2 ;;
    *) echo "unknown argument: $1" >&2; exit 2 ;;
  esac
done
[ -x "$PROG" ] || { echo "missing program: $PROG" >&2; exit 2; }
[ -x "$REF" ]  || { echo "missing reference: $REF" >&2; exit 2; }

cat > /tmp/bench-forms.el <<'ELISP'
(setq gc-cons-threshold most-positive-fixnum)
(defun bench-bc-loop (n) (let ((i 0)) (while (< i n) (setq i (1+ i)))))
(defun bench-bc-sum (n) (let ((i 0) (s 0)) (while (< i n) (setq s (+ s i)) (setq i (1+ i))) s))
(defun bench-bc-car (n) (let ((x '(1 2 3))) (dotimes (_ n) (car x))))
(mapc #'byte-compile '(bench-bc-loop bench-bc-sum bench-bc-car))
(defmacro bench-m (label &rest body)
  `(let ((best nil))
     (dotimes (_ 5)
       (let ((t0 (float-time))) ,@body
            (setq best (let ((dt (- (float-time) t0))) (if best (min best dt) dt)))))
     (message "BENCH %-12s %8.4f ms" (quote ,label) (* 1000 best))))
(let ((text (mapconcat (lambda (i) (format "line %d: alpha beta gamma\n" i))
                       (number-sequence 1 8000) "")))
  (bench-m bc-loop (bench-bc-loop 1000000))
  (bench-m bc-sum (bench-bc-sum 1000000))
  (bench-m bc-car (bench-bc-car 1000000))
  (bench-m regexp-repl (replace-regexp-in-string "alpha \\(beta\\)" "X-\\1" text))
  (bench-m buffer-scan (with-temp-buffer (insert text) (goto-char (point-min))
                        (while (re-search-forward "^line [0-9]+:" nil t))))
  (bench-m sort-100k (sort (number-sequence 1 100000) #'<)))
ELISP

run() {	# prog
  python3 "$HOME/.assistant/skills/controlled-run/scripts/run.py" --profile runtime \
    --memory-mib 900 -- sh -c \
    "cd '$REPO' && HOME=/nonexistent PATH=/usr/bin:/bin LC_ALL=C '$1' \
       --no-init-file --no-site-file --no-site-lisp --batch -l /tmp/bench-forms.el" 2>&1 \
    | grep '^BENCH '
}
echo "== port"
run "$PROG" > /tmp/bench-port.txt
cat /tmp/bench-port.txt
echo "== reference"
run "$REF" > /tmp/bench-ref.txt
cat /tmp/bench-ref.txt
echo "== ratio (port / reference)"
paste /tmp/bench-port.txt /tmp/bench-ref.txt | awk '{
  name=$2; a=$3; b=$7;
  if (b > 0) printf "  %-12s %8.4f / %8.4f = %.2fx\n", name, a, b, a/b
}'
