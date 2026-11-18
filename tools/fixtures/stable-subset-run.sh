#!/bin/sh
# Thin forwarder for a declared-subset re-verification of the stable corpus.
#
#   stable-subset-run.sh <subset.json> <out-dir> <candidate-program> \
#                        <candidate-dump> <reference-selection.json> [collector]
#
# The repository root is derived from this script's own location, never from a
# machine-specific absolute path. The reference program, dump and Lisp root come
# from an existing reference-selection receipt (`products.json` beside the given
# selection file), so this entry adds no second way of discovering a product.
set -u
SELF_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd) || exit 2
REPO=$(CDPATH= cd -- "$SELF_DIR/../.." && pwd) || exit 2
if [ $# -lt 5 ]; then
  echo "usage: $0 <subset.json> <out-dir> <candidate-program> <candidate-dump> <reference-selection.json> [collector]" >&2
  exit 2
fi
SUBSET=$1
OUT=$2
CAND=$3
CAND_DUMP=$4
SELECTION=$5
COLLECTOR=${6:-young}
APP=$(dirname "$(dirname "$CAND")")
CAND_LISP=$APP/Resources/lisp
for required in "$SUBSET" "$CAND" "$CAND_DUMP" "$SELECTION"; do
  if [ ! -f "$required" ]; then
    echo "missing input: $required" >&2
    exit 2
  fi
done
if [ ! -d "$CAND_LISP" ]; then
  echo "candidate Lisp root is missing: $CAND_LISP" >&2
  exit 2
fi
REFERENCE=$(python3 - "$SELECTION" <<'PY'
import json, sys
from pathlib import Path
selection = Path(sys.argv[1]).resolve()
products_path = selection.parent / 'products.json'
products = json.loads(products_path.read_text(encoding='utf-8'))
rows = [row for row in products.get('products', []) if row.get('id') == 'installed-reference']
if len(rows) != 1:
    raise SystemExit('reference selection must declare exactly one installed-reference product')
row = rows[0]
lisp = (row.get('resource_roots') or {}).get('lisp')
if not lisp:
    raise SystemExit('installed-reference product has no Lisp resource root')
print(row['program'])
print(row['dump'])
print(lisp)
PY
) || exit 2
REFERENCE_PROGRAM=$(printf '%s\n' "$REFERENCE" | sed -n '1p')
REFERENCE_DUMP=$(printf '%s\n' "$REFERENCE" | sed -n '2p')
REFERENCE_LISP=$(printf '%s\n' "$REFERENCE" | sed -n '3p')
python3 "$REPO/tools/stable-suite.py" \
  --repo "$REPO" \
  --manifest "$REPO/compatibility/stable-31.1/sources.json" \
  --out "$OUT" \
  --reference "$REFERENCE_PROGRAM" \
  --candidate "$CAND" \
  --reference-dump-args "[\"--dump-file=$REFERENCE_DUMP\"]" \
  --candidate-dump-args "[\"--dump-file=$CAND_DUMP\"]" \
  --reference-env "{\"FEMACS_GC_OBSERVATION\": \"off\", \"FEMACS_STABLE_PRODUCT_LISP\": \"$REFERENCE_LISP\"}" \
  --candidate-env "{\"FEMACS_GC_OBSERVATION\": \"off\", \"FEMACS_GC_COLLECTOR\": \"$COLLECTOR\", \"FEMACS_STABLE_PRODUCT_LISP\": \"$CAND_LISP\"}" \
  --timeout 300 \
  --runner "$REPO/tools/fixtures/stable-suite-runner.el" \
  --corpus-cache "$REPO/.femacs-cache/stable-corpus" \
  --subset-file "$SUBSET"
rc=$?
echo "SUBSET_RC=$rc"
exit $rc
