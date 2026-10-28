# CLN-20261029-123 — Consumed notification source-context copies

Only duplicate source-test contexts from seven completed runs were retired.
The selected roots are each run's `source-context-data` and `source-contexts`:

- `runtime-p421-remote07-calibration-1`
- `runtime-p421-full-notification-reference-1`
- `runtime-p421-full-notification-cocoa-normal-1`
- `runtime-p421-monitor-chunks-diagnostic-1`
- `runtime-p422-base64-diagnostic-3`
- `runtime-p421-local-functional-1`
- `runtime-p421-p422-local-functional-1`

Every run had completed, retained unchanged product identities, released its
artifact journal and confirmed native process cleanup before selection. All
selected trees were checked for open handles and rechecked immediately before
retirement. This does not reclassify failed or skipped cases as passes.

## Actual retirement

| Item | Recorded result |
| --- | --- |
| Exact context roots | 14 |
| Files and links | 2216 |
| Directories | 87 |
| Regular-file logical bytes | 195406890 |
| Manifest records | 2303, all decoded and checked |
| Symlink identities | 102, including 6 external targets retained only by hash |
| Selected roots absent afterward | Confirmed |

The manifest is
[`2026-10-29-123-consumed-notification-context-copies.jsonl.gz`](../manifests/2026-10-29-123-consumed-notification-context-copies.jsonl.gz),
SHA-256 `94f3bdd839c8550d7ac65775a891aea491d6ca257d06b83d4c4bfde22c1d24ee`.
Each entry binds its repository-relative location, type, mode, inode identity
and content hash where applicable. Raw link text is hashed; an internal target
also has a normalized repository-relative locator. External targets are not
embedded as machine-specific paths.

Original scripts, test inputs, all raw pass/failure/skip records, captured GIO
chunks and Base64 payloads, actual source/bytecode/programs/dumps and controller
receipts remain. Result/journal/controller hashes, the current production
repair source, four retained candidate program/dump pairs and the installed
comparison program/dump pair were verified unchanged around retirement.
The active P423 build uses its independent frozen export and the pinned corpus
cache; it does not consume these old context copies. Its source, compiler
outputs, objects and products were not selected.

The byte total is logical file content, not an attributed APFS free-space
increase. The native-input report and original source-test results remain
limited to their recorded scopes; neither whole G3 nor the Goal is completed
by this cleanup.

## Recovery and local evidence

Recreate test-only contexts from the retained pinned corpus/context manifests,
verified source cache, `tools/stable-context.py` and the retained exact product
libraries. A fresh owned output can replay the same original input bytes and
links without changing the old result files. Inode identity is not promised
after regeneration.

The private exact plan, pre-retirement retained-input hashes, durable unlink
intent/completion log and final summary are under
`build/cleanup/2026-10-29-123`. The original context result metadata remain as
historical evidence; the retired physical context copies are no longer present
and must be regenerated before a replay that needs them.
