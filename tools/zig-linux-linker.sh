#!/bin/sh
# A C linker for the Linux target, built out of the zig that ships on this machine.
#
# rustc's `aarch64-unknown-linux-gnu` target hands the linker GNU-gold style aarch64 hints that
# zig's linker refuses outright (`unsupported linker arg: --fix-cortex-a53-843419`, measured);
# they are erratum/hardening hints, not correctness flags, so they are dropped and the rest is
# passed through unchanged.  Use it as
#
#   rustc --target aarch64-unknown-linux-gnu -C linker=tools/zig-linux-linker.sh …
#   CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=tools/zig-linux-linker.sh cargo build --target …
#
# Reading (2026-10-17): a hello-world links and `file` reports
# "ELF 64-bit LSB pie executable, ARM aarch64, dynamically linked".
set -u
ZIG=${ZIG:-/opt/homebrew/bin/zig}
TRIPLE=${ZIG_TARGET:-aarch64-linux-gnu}
args=""
for a in "$@"; do
  case "$a" in
    --fix-cortex-a53-843419|-Wl,--fix-cortex-a53-843419) ;;
    *) args="$args \"$a\"" ;;
  esac
done
eval "exec \"$ZIG\" cc -target \"$TRIPLE\" $args"
