# CLN139: consumed runtime-support test copies

The thirteen-file normal-product comparison and native-enabled cache follow-up
have completed, and independent review has consumed their fixtures. Only their
seven isolated `test` trees and seven `context/test` links are retired. This
does not change default features, product code or any original test outcome.

## Exact scope

Three trees belong to `build/acceptance/g3-runtime-libraries-1`: reference,
terminal and Cocoa. Four belong to `build/acceptance/g3-native-cache-1`:
native-enabled terminal/Cocoa normal and checking profiles. Each contains
1,536 copied original corpus files and no generated resource files. The
complete pinned source cache is retained as the reconstruction input.

The selected trees contain 10,752 regular files, 1,757 directories and
146,978,664 logical file bytes; seven context links are also removed. File
allocation was 175,616,000 bytes, which does not prove equivalent physical
volume reclamation. The public
[manifest](../manifests/2026-10-30-139-consumed-runtime-support-test-copies.jsonl.gz)
contains 12,516 rows, SHA-256
`62252fcf063ab4746ff5aef7561b2572db76db4271cc171723bb940c245b84d8`.
All decoded fields match the private file/directory/link identity plan; link
targets are repository-relative and contain no local machine paths.

## Preservation and recovery

Both compatibility owners completed process cleanup; the normal group retains
exit 1 and six original native-feature skips, while the native-cache group
exits 0. Every selected file process completed and was reaped. Review also
ended before retirement. Existing `tools/runtime-retire.py` performs targeted
open-handle checks, durable pre-delete per-file ledgers and identity checks
before unlinking. Successful process exits do not turn skips into test passes.

All selected roots and context links are now absent. Git HEAD/index and prior
working changes, programs/dumps, complete selected normal/native resource
groups and build receipts, original false/skip/pass results, events, logs and
owner receipts remain unchanged. No actual product, compiler cache, package,
user configuration, unique input or unrelated tree is selected. No editor or
test is rerun for cleanup.

Recreate independent test trees with the existing stable-corpus materializer
using `compatibility/stable-31.1/sources.json` and the retained verified cache,
restore each profile's context links, then replay the recorded original file
arguments with the existing ERT bridge. Reuse of the old output paths requires
rematerialization first; original inodes and timestamps are not promised.
The original reference cache observations are reusable only while their exact
product/input identities still match. See [result and feature boundaries](../../releases/2026-10-30-original-runtime-support.md).
