#!/bin/sh
set -eu

ROOT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$ROOT_DIR"

sh ./scripts/build.sh

ELISCRIPT_BIN="${ELISCRIPT_BIN:-$ROOT_DIR/node_modules/.bin/eliscript}"
if [ -n "${BUN:-}" ]; then
  BUN_BIN="$BUN"
elif command -v bun >/dev/null 2>&1; then
  BUN_BIN=$(command -v bun)
elif [ -x "$HOME/.bun/bin/bun" ]; then
  BUN_BIN="$HOME/.bun/bin/bun"
else
  echo "femacs test: bun is required to bundle Eliscript tests" >&2
  exit 1
fi

"$ELISCRIPT_BIN" -o "$ROOT_DIR/.tmp/installer.test.mjs" "$ROOT_DIR/test/installer.test.eli"
"$BUN_BIN" build "$ROOT_DIR/.tmp/installer.test.mjs" \
  --target=node --format=esm --outfile="$ROOT_DIR/.tmp/installer.test.bundle.mjs"

node "$ROOT_DIR/.tmp/installer.test.bundle.mjs"
