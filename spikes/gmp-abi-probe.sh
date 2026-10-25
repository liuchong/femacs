#!/bin/sh
# Compatibility entry; probe policy is owned by xtask.
set -eu
cd "$(dirname "$0")/.."
exec cargo run -q -p xtask -- audit-probe gmp-abi "$@"
