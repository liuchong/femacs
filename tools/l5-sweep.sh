#!/bin/bash
# Local, resumable differential audit. Source/test paths are owned by check-ert.py.
set -euo pipefail
HERE=$(cd "$(dirname "$0")/.." && pwd)
START=0
CHUNK=30
TMO=120
OUT="$HERE/build/acceptance/l5"
EXTRA=()
while [ "$#" -gt 0 ]; do
  case "$1" in
    --start-at|--chunk|--timeout|--out)
      [ "$#" -ge 2 ] || { echo "missing value: $1" >&2; exit 2; }
      case "$1" in
        --start-at) START=$2;; --chunk) CHUNK=$2;; --timeout) TMO=$2;; --out) OUT=$2;;
      esac
      shift 2;;
    --gc-stress) EXTRA+=(--gc-stress); shift;;
    --port-prog|--ref-prog)
      [ "$#" -ge 2 ] || { echo "missing program path" >&2; exit 2; }
      EXTRA+=("$1" "$2"); shift 2;;
    *) echo "unknown argument: $1" >&2; exit 2;;
  esac
done
for n in "$START" "$CHUNK" "$TMO"; do
  case "$n" in ''|*[!0-9]*) echo "invalid nonnegative integer: $n" >&2; exit 2;; esac
done
[ "$CHUNK" -gt 0 ] && [ "$TMO" -gt 0 ] || { echo "chunk/timeout must be positive" >&2; exit 2; }
LIST=$(python3 "$HERE/tools/check-ert.py" --list)
TOTAL=$(printf '%s\n' "$LIST" | tail -n +2 | wc -l | tr -d ' ')
[ "$TOTAL" -gt 0 ] || { echo "no test files" >&2; exit 2; }
mkdir -p "$OUT"
at=$START
failed=0
while [ "$at" -lt "$TOTAL" ]; do
  ts=$(printf '%s/ert-l5-chunk%03d.tsv' "$OUT" "$((at / CHUNK + 1))")
  echo "== chunk from $at -> $ts =="
  if python3 "$HERE/tools/check-ert.py" --timeout "$TMO" --start-at "$at" --max-files "$CHUNK" \
       ${EXTRA[@]+"${EXTRA[@]}"} --tsv "$ts" --results-json "${ts%.tsv}.json" 2>&1 | tail -3; then
    :
  else
    failed=1
  fi
  at=$((at + CHUNK))
done
echo "L5 sweep complete; failed=$failed; evidence=$OUT"
exit "$failed"
