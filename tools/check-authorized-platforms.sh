#!/bin/sh
# The acceptance surface the user authorised: macOS (real machine) and Android (structure).
#
# Two claims, one command, fail-fast:
#   * macOS       tools/smoke.sh                     - 11 checks against the dumped program
#   * Android     xtask gate-android                 - builds and signs the APK, then asserts
#                                                      package/activities/members/signature
#
# Run it through the controlled executor (the Android half peaks near a gigabyte):
#   python3 ~/.assistant/skills/controlled-run/scripts/run.py --profile runtime \
#       --memory-mib 1400 -- bash tools/check-authorized-platforms.sh
#
# The remaining Android steps (native libraries, emulator install) need the two external
# prerequisites named in docs/current-state.md; this script reports them as not-run rather
# than pretending they passed.
set -u
REPO=$(cd "$(dirname "$0")/.." && pwd)
status=0

echo "== macOS: smoke =="
if bash "$REPO/tools/smoke.sh" 2>&1 | tail -1; then :; else status=1; fi

echo "== Android: gate-android =="
if (cd "$REPO/femacs" && CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/xt-cargo} cargo run -q -p xtask -- gate-android 2>&1 | tail -1); then :; else status=1; fi

echo "== not run (external prerequisites, docs/current-state.md) =="
printf '  %-34s %s\n' "Android native libraries" "needs makeinfo (A) or the info/emacs stub (B)"
printf '  %-34s %s\n' "Android emulator install" "needs an arm64-v8a system image (C)"
exit "$status"
