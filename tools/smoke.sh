#!/bin/sh
# Compatibility entry only; measurement/audit policy lives in the Python tool.
exec python3 "$(dirname "$0")/differential.py" --smoke "$@"
