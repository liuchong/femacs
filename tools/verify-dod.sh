#!/bin/bash
# Re-prove the completion definition, one row at a time, with the command that backs it.
#
#   tools/verify-dod.sh            # the cheap rows (about 8 minutes)
#   tools/verify-dod.sh --full     # also gate-fast, the platform gate and the L5 sweep start
#
# `docs/current-state.md` is the ledger; this script is the same eight rows as commands, so a
# session can refresh them without re-reading the document and guessing which command each row
# means.  It prints one `ok`/`FAIL` line per row and exits non-zero if any cheap row fails.
#
# Row 7 (performance) is not run: it is shelved by the user's standing decision and recorded as
# unmet, so running it would produce a number that is not a pass.  The script says so rather than
# quietly leaving a gap.
set -u
HERE=$(cd "$(dirname "$0")/.." && pwd)
cd "$HERE" || exit 1
FULL=0
[ "${1:-}" = "--full" ] && FULL=1
fail=0
row () { printf '%-46s %s\n' "$1" "$2"; }

say () { printf '\n== %s ==\n' "$1"; }

# `gate-fast` and the platform gate both need C's own archive, and the environment wipes build
# products between rounds (AGENTS.md section 3).  Rebuilding it takes seconds and is not a
# judgement about the port, so it happens here instead of turning into a mysterious FAIL.
prepare () {
  if [ ! -f build/emacs-ref/lib/libgnu.a ]; then
    printf 'prepare: build/emacs-ref/lib/libgnu.a was missing, rebuilding it\n'
    make -C build/emacs-ref/lib libgnu.a >/dev/null 2>&1 || {
      printf 'prepare: could not build libgnu.a\n'; return 1; }
  fi
  return 0
}

# A failing row must show why; swallowing the output turned a missing archive into an
# unexplained FAIL the first time this script ran with --full (status 4.980).
run_row () {
  local label=$1; shift
  local log; log=$(mktemp "${TMPDIR:-/tmp}/verify-dod.XXXXXX")
  if "$@" >"$log" 2>&1; then
    rm -f "$log"; row "$label" ok; return 0
  fi
  row "$label" FAIL; fail=1
  tail -4 "$log" | sed 's/^/    /'
  rm -f "$log"; return 1
}

prepare || { echo "verify-dod: the environment could not be prepared"; exit 2; }

# The programs the macOS rows measure must be newer than the sources they claim to prove.  A
# stale binary is not a hypothetical: on 2026-10-20 the binary was seven hours older than the last
# source change, so smoke, differential and bench all reported on code that had already been
# replaced (status 4.990).  Checked here, once, instead of trusting whoever ran the build.
if [ -f build/port/src/emacs ]; then
  newer=$(find femacs/src femacs/lib -name '*.rs' -newer build/port/src/emacs 2>/dev/null | head -1)
  if [ -n "$newer" ]; then
    row "port binary newer than sources" "STALE ($newer is newer)"
    echo "    rebuild: xtask gate-load, then xtask dump"
    fail=1
  else
    row "port binary newer than sources" ok
  fi
else
  row "port binary newer than sources" "MISSING (build it with gate-load + dump)"
  fail=1
fi

say "row 1: 1:1 transcription"
if python3 tools/extract-abi.py --all 2>&1 | tail -1 | grep -q '590 done, 0 in progress, 0 failing'; then
  row "extract-abi --all" ok
else
  row "extract-abi --all" FAIL; fail=1
fi

say "row 2: layout / symbols / subr / declarations"
if python3 tools/check-declarations.py >/dev/null 2>&1; then row "check-declarations" ok; else row "check-declarations" FAIL; fail=1; fi
if [ "$FULL" = 1 ]; then
  run_row "xtask gate-fast" bash -c "cd femacs && CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/xt-cargo} cargo run -q -p xtask -- gate-fast"
else
  row "xtask gate-fast" "skipped (--full)"
fi

say "row 4: generated products, loadup, dump offsets"
if python3 tools/check-generated.py 2>&1 | tail -1 | grep -q '0 DIFFERENT, 0 missing'; then row "check-generated" ok; else row "check-generated" FAIL; fail=1; fi
if python3 tools/check-loadup.py 2>&1 | tail -1 | grep -q 'agree file for file'; then row "check-loadup" ok; else row "check-loadup" FAIL; fail=1; fi
if python3 tools/check-dump-offsets.py 2>&1 | tail -2 | grep -q '0 failure'; then row "check-dump-offsets" ok; else row "check-dump-offsets" FAIL; fail=1; fi
run_row "gen-lisp-mk" python3 tools/gen-lisp-mk.py
run_row "gen-unidata" python3 tools/gen-unidata.py
run_row "gen-charsets" python3 tools/gen-charsets.py

say "row 5: the verbatim trees"
if python3 tools/check-verbatim.py 2>&1 | tail -1 | grep -q '0 failures'; then row "check-verbatim" ok; else row "check-verbatim" FAIL; fail=1; fi

say "row 6: the authorised platforms"
if [ "$FULL" = 1 ]; then
  run_row "check-authorized-platforms" bash tools/check-authorized-platforms.sh
  run_row "tui-smoke 24x80" python3 tools/tui-smoke.py build/port/src/emacs
  run_row "tui-smoke 10x80" python3 tools/tui-smoke.py build/port/src/emacs --rows 10
else
  row "check-authorized-platforms" "skipped (--full)"
  row "tui-smoke (terminal frame)" "skipped (--full)"
fi
# The device reading is evidence rather than a command: the acceptance images are committed.
if [ -f docs/baseline/android-accept/2-typed.png ]; then
  row "android device acceptance (evidence)" "present (tools/android-accept.sh refreshes it)"
else
  row "android device acceptance (evidence)" "MISSING"; fail=1
fi

say "row 8: signed commits and the ledger"
if python3 tools/check-ledger-ticks.py 2>&1 | tail -1 | grep -q '0 failure'; then row "check-ledger-ticks" ok; else row "check-ledger-ticks" FAIL; fail=1; fi
unsigned=0
for c in $(git log --format=%H -5); do git verify-commit "$c" >/dev/null 2>&1 || unsigned=$((unsigned + 1)); done
if [ "$unsigned" = 0 ]; then row "last 5 commits signed" ok; else row "last 5 commits signed" "FAIL ($unsigned unsigned)"; fail=1; fi

say "row 7: performance"
row "DoD-7" "not run: shelved by the user's standing decision, recorded as unmet"

say "documentation gates"
if python3 tools/check-doc-numbers.py >/dev/null 2>&1; then row "check-doc-numbers" ok; else row "check-doc-numbers" FAIL; fail=1; fi
if python3 tools/check-own-voice.py >/dev/null 2>&1; then row "check-own-voice" ok; else row "check-own-voice" FAIL; fail=1; fi

printf '\n'
if [ "$fail" = 0 ]; then echo "verify-dod: every row that was run is ok"; else echo "verify-dod: at least one row failed"; fi
exit "$fail"
