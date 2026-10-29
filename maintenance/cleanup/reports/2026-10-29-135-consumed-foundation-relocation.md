# CLN135: consumed foundation relocation copies

Four temporary copied packages were retired after source-denied relocation
checks. These were copies of the normal/checking Cocoa and terminal foundation
products, not the original packages or user-installed application.

| Product | Files | Directories | Logical bytes |
| --- | ---: | ---: | ---: |
| cocoa-normal | 4477 | 113 | 228738365 |
| cocoa-checking | 4487 | 113 | 229855476 |
| terminal-normal | 4486 | 108 | 235672912 |
| terminal-checking | 4486 | 108 | 237106064 |

Total: 17,936 members (17,934 regular files and two symbolic links), 442 directories and
931,372,817 logical bytes. All selected package roots are absent.
The source products and their complete selected resources remained unchanged.
The original programs, dumps, SDK/runtime libraries, inputs, receipts and failures
were retained; no source or original product was deleted.

The ordinary terminal copy was initially held by Spotlight. Its original raw
failure is retained. After zero occupancy and full equality with the unchanged
original package were established, only that copy was retired. No test was rerun.
Other three copies were retired by the original bounded runner.

Each member had a saved byte/mode/link identity and node identity, checked again
before deletion, with durable intent/result operations. The 36,756
operations pair exactly once for each file and directory. All 17,936 public
manifest rows and every decoded field match their original private manifests;
link targets were inspected. No private machine locator is published.

[Manifest](../manifests/2026-10-29-135-consumed-foundation-relocation.jsonl.gz)
SHA256: `76c25e3e732a5246c8d775ec6b34f3483712b43a4ec57b8be4b2375438bc24b7`.
Raw occupancy, lifecycle, node identities and operations remain in
`build/acceptance/runtime-foundation-relocation-1`.

Restore a temporary package by copying its bound original product again. This
preserves product bytes, modes and internal links, not original inode identities
or arbitrary filesystem attributes. Whole-filesystem free-space changes are not
attributed to this cleanup.

The [relocation report](../../releases/2026-10-29-foundation-relocation.md)
limits the result to source-denied batch execution. Physical GUI, complete
capability/compatibility coverage and native/collector performance remain open;
this cleanup does not close G1/G2/G3 or the whole Goal.
