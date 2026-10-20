#!/bin/sh
# The performance table of docs/acceptance.md, re-measurable on demand.
#
# Every row is min-of-five inside each program, same machine, same -O2 build; `--rounds N`
# alternates the two programs N times and keeps the per-case minimum, so a machine that is
# busy with something else cannot decide the ratio (status 4.992).  The bytecode rows
# run *compiled* functions (source-level loops measure eval_sub instead, which is a different
# layer and was mistaken for the VM once - PORTING.md P-180/P-181).
#
#   tools/bench.sh                     # port against the reference
#   tools/bench.sh --prog build/port/src/emacs --ref build/emacs-ref/src/emacs
#   tools/bench.sh --save docs/baseline/bench.tsv    # record this machine's numbers
#   tools/bench.sh --save F --force                  # record even when the machine is busy
#   tools/bench.sh --max-load-per-core 0.05         # tighten the quiet-machine guard
#   tools/bench.sh --gate docs/baseline/bench.tsv    # fail on a regression against them
#
# Exit status is 0 when reporting (DoD-7's target is a judgement call) and, in `--gate` mode,
# 1 when a case is slower than its baseline by more than `--tolerance` (default 15%).  The
# reference is rebuilt from time to time for other reasons (P-193 compiled it to the switch
# arm once), so a gate on the *ratio* would fire on changes that have nothing to do with the
# port: the gate therefore compares the **port's own milliseconds** to the recorded ones, and
# prints the reference's beside them for context.  A `--save` is refused when the one-minute
# load average is above `--max-load-per-core` (default 0.10) times the core count: a baseline
# recorded on a busy machine is a slow baseline, and three rounds of readings were lost that way.
set -u
REPO=$(cd "$(dirname "$0")/.." && pwd)
SAVE=""; GATE=""; TOL=0.15; ROUNDS=1; FORCE=0; MAX_LOAD_PER_CORE=0.10
PROG="$REPO/build/port/src/emacs"
REF="$REPO/build/emacs-ref/src/emacs"
while [ $# -gt 0 ]; do
  case "$1" in
    --prog) PROG=$2; shift 2 ;;
    --ref)  REF=$2;  shift 2 ;;
    --save) SAVE=$2; shift 2 ;;
    --gate) GATE=$2; shift 2 ;;
    --tolerance) TOL=$2; shift 2 ;;
    --rounds) ROUNDS=$2; shift 2 ;;
    --force) FORCE=1; shift ;;
    --max-load-per-core) MAX_LOAD_PER_CORE=$2; shift 2 ;;
    *) echo "unknown argument: $1" >&2; exit 2 ;;
  esac
done
[ -x "$PROG" ] || { echo "missing program: $PROG" >&2; exit 2; }
[ -x "$REF" ]  || { echo "missing reference: $REF" >&2; exit 2; }
load1=$(sysctl -n vm.loadavg 2>/dev/null | awk '{print $2}'); [ -z "$load1" ] && load1=$(awk '{print $1}' /proc/loadavg 2>/dev/null)
ncpu=$(sysctl -n hw.ncpu 2>/dev/null || nproc 2>/dev/null || echo 1)
if [ -n "$SAVE" ] && [ "$FORCE" -eq 0 ]; then
  over=$(awk -v l="${load1:-0}" -v c="$ncpu" -v m="$MAX_LOAD_PER_CORE" 'BEGIN { print (l > c * m) ? 1 : 0 }')
  if [ "$over" = 1 ]; then
    echo "bench: refusing to save a baseline at load ${load1} on ${ncpu} core(s) (limit $MAX_LOAD_PER_CORE per core)." >&2
    echo "       A baseline recorded on a busy machine is a slow baseline; rerun when quiet, or pass --force." >&2
    exit 2
  fi
fi
case "$ROUNDS" in ''|*[!0-9]*) echo "bad --rounds: $ROUNDS" >&2; exit 2 ;;
esac
[ "$ROUNDS" -ge 1 ] || { echo "bad --rounds: $ROUNDS" >&2; exit 2; }

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
# One round runs each program once.  `--rounds N` alternates the two programs N times and keeps
# the per-case minimum across rounds: the two sides are then sampled across the same window
# instead of one after the other, so a load average that moves during the run cancels out.
fold() {	# file
  awk '{
    if (!($2 in m)) { n++; order[n] = $2; m[$2] = $3 }
    else if ($3 + 0 < m[$2] + 0) m[$2] = $3
  } END { for (i = 1; i <= n; i++) printf "BENCH %-12s %8.4f ms\n", order[i], m[order[i]] }' "$1"
}
: > /tmp/bench-port-all.txt
: > /tmp/bench-ref-all.txt
r=1
while [ "$r" -le "$ROUNDS" ]; do
  run "$PROG" >> /tmp/bench-port-all.txt
  run "$REF" >> /tmp/bench-ref-all.txt
  r=$((r + 1))
done
echo "== port (min over $ROUNDS round(s); each round is already min-of-five inside the program)"
fold /tmp/bench-port-all.txt > /tmp/bench-port.txt
cat /tmp/bench-port.txt
echo "== reference (min over $ROUNDS round(s))"
fold /tmp/bench-ref-all.txt > /tmp/bench-ref.txt
cat /tmp/bench-ref.txt
echo "== ratio (port / reference)"
paste /tmp/bench-port.txt /tmp/bench-ref.txt | awk '{
  name=$2; a=$3; b=$7;
  if (b > 0) printf "  %-12s %8.4f / %8.4f = %.2fx\n", name, a, b, a/b
}'

if [ -n "$SAVE" ]; then
  {
    printf 'case\tport_ms\tref_ms\tdate\n'
    today=$(date +%Y-%m-%d)
    paste /tmp/bench-port.txt /tmp/bench-ref.txt | awk -v d="$today" '{
      name=$2; a=$3; b=$7;
      if (b > 0) printf "%s\t%.4f\t%.4f\t%s\n", name, a, b, d
    }'
  } > "$SAVE"
  echo "saved $SAVE"
fi

if [ -n "$GATE" ]; then
  status=0
  echo "== gate against $GATE (tolerance ${TOL})"
  while IFS="$(printf '\t')" read -r name base_ms rest; do
    [ "$name" = "case" ] && continue
    [ -z "$name" ] && continue
    now=$(awk -v n="$name" '$2 == n { print $3 }' /tmp/bench-port.txt)
    [ -z "$now" ] && { echo "  MISSING  $name (in the baseline, not in this run)"; status=1; continue; }
    verdict=$(awk -v b="$base_ms" -v n="$now" -v t="$TOL" 'BEGIN {
      if (b <= 0) { print "skip"; exit }
      r = n / b
      printf "%+.1f%% %s", (r - 1) * 100, (r - 1 > t ? "FAIL" : "ok")
    }')
    printf "  %-12s baseline %8.4f ms, now %8.4f ms  %s\n" "$name" "$base_ms" "$now" "$verdict"
    case "$verdict" in *FAIL) status=1 ;; esac
  done < "$GATE"
  [ "$status" = 0 ] && echo "gate: no case is slower than its baseline by more than ${TOL}" \
                    || echo "gate: a case regressed beyond ${TOL} (DoD-7)"
  exit "$status"
fi
