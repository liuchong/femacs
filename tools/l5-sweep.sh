#!/bin/bash
# L5 (the reference's own ERT suite) over every test file, in chunks.
#
# `check-ert.py` runs one file at a time on both programs and compares the summaries; a full
# sweep is hours of wall clock, so it is chunked and resumable: each chunk writes its own TSV and
# the log line after it says where to start the next run.  Chunk size and the per-side timeout are
# arguments because a slow machine trades them off.
#
#   tools/l5-sweep.sh [--start-at N] [--chunk N] [--timeout N] [--out DIR]
#
# The first chunk already measured 30 files / 399 s (status 4.967: 28 same, 1 both-fail, 1
# symmetric timeout, port and reference both 706 ran / 2 unexpected), so the default chunk is 30.
set -u
HERE=$(cd "$(dirname "$0")/.." && pwd)
START=0
CHUNK=30
TMO=120
OUT=$HERE/docs/baseline
while [ $# -gt 0 ]; do
  case "$1" in
    --start-at) START=$2; shift 2 ;;
    --chunk)    CHUNK=$2; shift 2 ;;
    --timeout)  TMO=$2; shift 2 ;;
    --out)      OUT=$2; shift 2 ;;
    *) echo "unknown argument: $1" >&2; exit 2 ;;
  esac
done
TOTAL=$(python3 "$HERE/tools/check-ert.py" --list | tail -n +2 | wc -l | tr -d ' ')
echo "L5 sweep: $TOTAL file(s), from $START, chunks of $CHUNK, per-side timeout ${TMO}s"
at=$START
while [ "$at" -lt "$TOTAL" ]; do
  ts=$(printf '%s/ert-l5-chunk%03d.tsv' "$OUT" "$((at / CHUNK + 1))")
  echo "== chunk from $at -> $ts =="
  python3 "$HERE/tools/check-ert.py" --timeout "$TMO" --start-at "$at" --max-files "$CHUNK" \
          --tsv "$ts" 2>&1 | tail -3
  at=$((at + CHUNK))
  echo "   next --start-at $at"
done
echo "L5 sweep: done; per-chunk TSVs in $OUT"
