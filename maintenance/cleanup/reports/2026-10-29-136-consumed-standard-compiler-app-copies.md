# CLN136: consumed standard compiler and App copies

The standard committed-checkout delivery has completed build, functional,
packaging, source-denied relocation and current-path key/file acceptance, with
independent limited G2 review. This batch removes consumed copies, not sources
or an unsupported platform. The Goal remains active; G2 and G4 are delivered.

## Exact scope

- The clean clone's Cargo cache and terminal generated translation units.
- Root Cargo cache and terminal generated translation units after root rebuild.
- The clone's original Cocoa app and the release staging app, each identical in
  every member to the current published `dist/macos/Femacs.app`.
- The requested clone Cocoa generated-unit path was already absent: no deletion
  or recovered bytes are attributed to it.

Actual scope: six roots, 20,977 regular files, two symbolic links, 1,054
directories, and 1,194,535,878 logical bytes. The compressed
[manifest](../manifests/2026-10-29-136-consumed-standard-compiler-app-copies.jsonl.gz)
contains 22,033 rows, SHA-256
`65f4268aeca16f718956b057c963faac2f649d6ea8660adc527780b03d9f6c43`.
All 8,404,167 decoded bytes/fields match the private identity plan after explicit
path normalization; 9,176 derived path aliases do not expose local source names.
The 44,066 intent/result operations pair exactly. Entries retain kind, mode,
size, original node identity and content hash or normalized link identity.

## Preservation and recovery

Before deletion, each selected root had no open handles, no tracked Git entry
and an ignored owned location; all named workload owners had ended and reported
complete cleanup. Member identities were rechecked before unlink; directory identities before
rmdir. Git HEAD/index and tracked source identities remained unchanged.

The clean clone and its Git database, actual object files, clone/root terminal
program/dump pairs, current full App, disk image/checksum/manifest, raw failed and
passing evidence, generated manuals/resources and unrelated WIP remain. The
previous full App at `dist/macos/.Femacs-before-g2-standard.app` has a specific
G1 rollback consumer and is retained until that use ends. Full selected product
and Lisp/data/Info resource identities and retained evidence hashes match the
pre-cleanup values; no build or test was rerun for cleanup.

Regenerate compiler caches/units with the recorded standard recipes and source
commit; those regenerated intermediates are not promised identical in inode or
timestamp. Restore either removed App copy from the identical published App or
the retained release image. Raw original identities and operation receipts remain
in the task's private cleanup evidence. No Git object or source was reclaimed.

See [the delivery evidence](../../releases/2026-10-29-standard-checkout-delivery.md).
