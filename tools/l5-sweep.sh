#!/bin/sh
# Compatibility entry; scheduling and result policy live in the Python owner.
exec python3 "$(dirname "$0")/l5_sweep.py" "$@"
