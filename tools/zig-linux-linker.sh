#!/bin/sh
# Keep argv literal; only the two unsupported AArch64 linker hints are filtered.
set -u
if [ -n "${ZIG:-}" ]; then
  zig_program=$ZIG
else
  zig_program=$(command -v zig) || { echo "Zig is required; set ZIG or put zig on PATH" >&2; exit 2; }
fi
triple=${ZIG_TARGET:-aarch64-linux-gnu}
# Rotate exactly the original positional arguments, appending each retained word.
# No argument is parsed as shell source, including an empty or quoted argument.
remaining=$#
while [ "$remaining" -gt 0 ]; do
  argument=$1
  shift
  remaining=$((remaining - 1))
  case "$argument" in
    --fix-cortex-a53-843419|-Wl,--fix-cortex-a53-843419) ;;
    *) set -- "$@" "$argument" ;;
  esac
done
exec "$zig_program" cc -target "$triple" "$@"
