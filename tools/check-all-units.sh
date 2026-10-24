#!/bin/sh
# Compatibility entry: the Rust driver owns normal/checking compilation.
root=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd) || exit 2
cd "$root" || exit 2
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$root/build/cargo}"
exec cargo run -q -p xtask -- check-units "$@"
