#!/bin/sh
# Compatibility entry; build policy belongs to xtask.
set -eu
cd "$(dirname "$0")/.."
exec cargo run -q -p xtask -- android kotlin "$@"
