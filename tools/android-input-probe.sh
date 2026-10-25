#!/bin/sh
# Legacy spelling with canonical Python arguments; see docs/repository-bootstrap.md.
set -eu
exec python3 "$(dirname "$0")/android-device.py" probe "$@"
