# CLN133: consumed Tree-sitter compiler copies

Retired eleven ended-run roots after the limited P430 implementation and product
evidence had passed independent review:

- Cargo caches under `build/acceptance/runtime-g3-treesit-current-products-1`,
  `runtime-g3-treesit-current-products-2` and `runtime-p430-protection-1`.
- Generated `objects/units` beneath the first failed Cocoa normal/checking build
  and the second successful Cocoa/terminal normal/checking build.
- Generated `objects/units` in `runtime-p430-tu-integration-1` and `-2`.

These roots have no remaining compiler consumer. The diagnostic runs were bound
to an old frozen driver, not the new source. Both original materialized root
files were retained privately as minimal source-binding evidence before their
rebuildable include copies were removed. The tested programs, actual objects,
SDK libraries and all original failures remain.

## Actual removal

- 30,706 regular files, zero links and 1,690 directories.
- 1,144,859,531 logical bytes (about 1.066 GiB).
- All eleven selected roots are absent; no already-absent root was counted.
- 32,396 public manifest rows, 15,966,221 decoded bytes and
  28,387 irreversible aliases for machine-derived filenames.
- 64,792 durable intent/result operations pair exactly once
  per file or directory. All decoded manifest fields match the private plan
  after path aliasing, including nested link/identity values.

Manifest SHA256: `5a3c547d933c614ced9ff30c825b12807859ee4a973e26de294f64537b69a253`.
The [manifest](../manifests/2026-10-29-133-consumed-treesit-compiler-copies.jsonl.gz)
contains public relative identities. Exact original locators and deletion
operations remain private. Whole-filesystem free-space readings are retained
privately; their entire delta is not attributed to this cleanup.

## Retained and verified

11 root identities were checked as ignored, untracked, regular owned
directories with zero immutable flags and no open handles. Each original node
identity was rechecked immediately before removal. HEAD, default index and
tracked entries remained unchanged during deletion.

1083 selected retained file hashes, both frozen source
exports and five selected product identities with their complete resource groups
matched before and after cleanup. Retained inputs include the real link objects,
four current programs/dumps, Lisp/native backend/TLS/LCMS/Tree-sitter libraries
and licenses, grammar libraries, functional and ABI results, source manifests,
owner archives, raw failed attempts and reconstruction recipes. No runtime
library, source-context or test-state root was selected. No new build or test
was run for cleanup.

## Restore and acceptance boundary

Regenerate the named Cargo and generated-unit copies from the retained frozen
inputs and build recipes when needed. No ignored reference tree or other
conversation is a source dependency. Actual objects and tested products remain
usable. Rebuilding with changed paths, compilers or inputs does not promise
identical cache bytes or original inode identities.

P430 has limited evidence: 200 original case-products, 15 unique lifecycle/GC
case-products, five two-process query image restores, two later configuration
guards and 123 measured ABI observations. See the
[component report](../../releases/2026-10-29-macos-treesit.md).
Coherent prerequisite source delivery remains pending; this cleanup does not
publish incomplete implementation dependencies or claim full G3. The overall
Goal remains active at 1/7.
