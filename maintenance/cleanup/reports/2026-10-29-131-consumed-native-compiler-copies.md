# CLN131: consumed native compiler copies

Retired only five ended-run roots from the current native-product build:

- `build/acceptance/runtime-g3-native-current-products-1/cargo`.
- `objects/units` under each current native Cocoa/terminal normal/checking root
  in that build's frozen `source/build/runtime` tree.

The four products and both strict Cocoa signatures were completed. The full
original compiler file finished on all five native profiles: 925 passes and
five common bootstrap binary-comparison failures, with no skips/timeouts/not-run.
Those original failures remain raw false/exit 1. Independent review accepted
only the evidence and metadata diagnosis; no whole-compatibility pass is claimed.
See [compiler evidence](../../releases/2026-10-29-macos-original-compiler.md).

## Actual removal

- 19,910 regular files, zero symlinks and 903 directories.
- 705,203,373 logical bytes (about 672.5 MiB).
- All five selected roots are absent; no already-absent root was counted.
- 20,813 public manifest rows, 10,339,960 decoded bytes and
  18,798 irreversible aliases for machine-derived filenames.
- 41,626 durable intent/result records pair exactly once per file/directory.
- Every decoded manifest field matched the private plan after path aliasing.
  The complete decoded structure was checked, not just filenames.

Manifest SHA256: `eea5308e9ce4b9f952ecfb811cb00e376f4f3ba291517dc90575061238680d29`.
The [manifest](../manifests/2026-10-29-131-consumed-native-compiler-copies.jsonl.gz)
contains public relative identities; exact original locators and operations
remain private. Whole-filesystem available-space readings changed during the
operation and are retained privately; their entire delta is not attributed here.

## Retained and verified

Frozen source and source-input manifest, actual link objects, four executables
and dumps, Lisp, native backend, TLS, LCMS, licenses, all original ELN pairs,
failures, controller receipts, recipes and independent review remain. No
source-context, test-state or runtime-library tree was selected for removal.

726 individually retained file hashes and all five selected product
identities, including their complete recorded resource groups, matched before
and after removal. Frozen source was reverified. HEAD, default index and tracked
entries were unchanged during physical removal. The exact selected roots were
ignored/untracked, regular owned directories with zero immutable flags and no
open handles; each node's original identity was rechecked immediately before
unlink/rmdir. No build or test was rerun for cleanup.

## Restore boundary

Only rebuildable Cargo/generated-source duplicates were removed. Regenerate
them from the retained frozen inputs and build recipe when needed; do not
recover them from the ignored reference tree or another conversation. Actual
link objects and tested products remain immediately usable. Original inode
identity is not recoverable, and rebuilding with changed paths, toolchains or
inputs is not a promise of identical old cache bytes. No unique source or
uncommitted work was deleted. G3 and the overall Goal remain open (active 1/7).
