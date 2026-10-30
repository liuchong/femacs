# CLN162: consumed undo and file-workflow state

The original comparison completes with 123 passes and 18 common Windows/DOS
skips, no failed assertion or timeout. Every file process exits 0 and fully
reaps; the outer controller exits 1 because skips remain unpassed. Independent
review accepts this classification. Cleanup does not alter that false result.

## Exact scope and timing

The runner already retired six completed undo/file-lock environments before
review: five regular files and 51 directories, 170,464 logical and 184,320
allocated regular-file bytes. Their original hashed identity ledgers and
retirement receipts remain; the public manifest carries those 56 rows explicitly
as `already_retired_by_runner`, not as a second deletion.

After review, retire only three manifest-verified `test` trees, three
`context/test` links and the three remaining `environments` trees under
`build/acceptance/g3-file-workflows-1`. The latter include the six retained
file-I/O/process environments; original skip evidence is outside them and is
preserved. This operation removes 4,608 regular files, 819 directories and
three links: 62,990,856 logical and 75,264,000 allocated regular-file bytes.

Combined accounting is **4,613 files, 870 directories, three links**,
63,161,320 logical and 75,448,320 allocated regular-file bytes. Allocation
counts do not establish physical volume space reclaimed. The
[manifest](../manifests/2026-10-30-162-consumed-file-workflow-state.jsonl.gz)
contains 5,486 decoded rows, SHA-256
`20e5e252e6d211f079051fea910dd082f84684849b75dd35e25b4c13d56b3f7c`.
Post-review identities match captured private plans; earlier automatic rows
match their retained original ledgers. Public link targets are repository-relative.

## Preservation and recovery

Existing `tools/runtime-retire.py` checks targeted open handles, writes durable
identities before deletion and rechecks selected file/directory ownership.
Context links have separate identity/target checks. No compiler or product runs
for cleanup. Original result/events/timings and prior retirement receipts,
HEAD/index/tracked working changes, program/dump/full resource identities and
the entire trial App are verified unchanged. All selected consumed trees and
links are absent afterward.

Retain the complete pinned corpus, actual products, eleven live terminal
libraries and their restoration/preflight evidence. No user configuration,
package, unique source or unrelated build root is selected. No acceptance
policy or Goal changes.

Replay requires rematerializing complete pinned test trees, fresh isolated
environments and their context links, with the original products, dumps,
guards/assertions and time limits. Exact random names, inode numbers and
timestamps are not promised. See the
[comparison and replay recipe](../../releases/2026-10-30-original-file-workflows.md).
