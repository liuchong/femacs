#!/bin/sh
# Compatibility entry; device ownership and evidence live in Python.
set -eu
exec python3 "$(dirname "$0")/android-device.py" batch "$@"
