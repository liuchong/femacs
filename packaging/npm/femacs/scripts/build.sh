#!/bin/sh
set -eu

ROOT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$ROOT_DIR"

ELISCRIPT_BIN="${ELISCRIPT_BIN:-$ROOT_DIR/node_modules/.bin/eliscript}"
if [ ! -x "$ELISCRIPT_BIN" ]; then
  echo "femacs build: missing eliscript compiler at $ELISCRIPT_BIN (run npm install first)" >&2
  exit 1
fi

if [ -n "${BUN:-}" ]; then
  BUN_BIN="$BUN"
elif command -v bun >/dev/null 2>&1; then
  BUN_BIN=$(command -v bun)
elif [ -x "$HOME/.bun/bin/bun" ]; then
  BUN_BIN="$HOME/.bun/bin/bun"
else
  echo "femacs build: bun is required during prepack to inline eliscript/runtime" >&2
  exit 1
fi

rm -rf "$ROOT_DIR/.tmp" "$ROOT_DIR/dist"
mkdir -p "$ROOT_DIR/.tmp/bin" "$ROOT_DIR/dist/bin"

"$ELISCRIPT_BIN" -o "$ROOT_DIR/.tmp/index.mjs" "$ROOT_DIR/src/index.eli"
"$ELISCRIPT_BIN" -o "$ROOT_DIR/.tmp/postinstall.mjs" "$ROOT_DIR/src/postinstall.eli"
"$ELISCRIPT_BIN" -o "$ROOT_DIR/.tmp/bin/elisp.mjs" "$ROOT_DIR/src/bin/elisp.eli"
"$ELISCRIPT_BIN" -o "$ROOT_DIR/.tmp/bin/femacs.mjs" "$ROOT_DIR/src/bin/femacs.eli"
"$ELISCRIPT_BIN" -o "$ROOT_DIR/.tmp/bin/fe.mjs" "$ROOT_DIR/src/bin/fe.eli"

"$BUN_BIN" build "$ROOT_DIR/.tmp/index.mjs" \
  --target=node --format=esm --minify --outfile="$ROOT_DIR/dist/index.mjs"
"$BUN_BIN" build "$ROOT_DIR/.tmp/postinstall.mjs" \
  --target=node --format=esm --minify --outfile="$ROOT_DIR/dist/postinstall.mjs"

for cmd in elisp femacs fe; do
  "$BUN_BIN" build "$ROOT_DIR/.tmp/bin/${cmd}.mjs" \
    --target=node --format=esm --minify --outfile="$ROOT_DIR/dist/bin/${cmd}.raw.mjs"
  {
    printf '#!/usr/bin/env node\n'
    cat "$ROOT_DIR/dist/bin/${cmd}.raw.mjs"
  } > "$ROOT_DIR/dist/bin/${cmd}.mjs"
  rm -f "$ROOT_DIR/dist/bin/${cmd}.raw.mjs"
  chmod 0755 "$ROOT_DIR/dist/bin/${cmd}.mjs"
done

if grep -R -E 'from[[:space:]]*["'\'']eliscript/|import[[:space:]]*["'\'']eliscript/' "$ROOT_DIR/dist" >/dev/null 2>&1; then
  echo "femacs build: bundled dist/ still imports external eliscript runtime!" >&2
  exit 1
fi

echo "femacs build: OK ($(ls -lh "$ROOT_DIR/dist/index.mjs" "$ROOT_DIR/dist/bin/elisp.mjs" | awk '{print $9 ":" $5}' | tr '\n' ' '))"
