# CLN161: consumed original text-processing test copies

After the four-file comparison and independent review, retire only the three
`test` trees and three `context/test` links under
`build/acceptance/g3-text-processing-2`. The twelve file processes exit 0,
complete and leave no group to terminate; the owner exits 0 and fully reaps.
All 339 original observations pass, with no skips or generated resources.

## Exact scope and result

Retired: 4,608 regular files, 753 directories and three links. Each test tree
contains 1,536 manifest-verified originals. Regular-file logical bytes total
62,990,856; allocated bytes total 75,264,000. Allocation totals do not establish
physical volume space reclaimed.

The public [manifest](../manifests/2026-10-30-161-consumed-text-processing-test-copies.jsonl.gz)
has 5,364 decoded rows, SHA-256
`d024768e1e733a87087d627fef7cf20850427b8bc01424be4140675a64dcd41f`.
Every decoded file/directory/link field matches the captured private plan;
published link targets are repository-relative. All selected roots and links
are absent after retirement.

## Preservation and recovery

Existing `tools/runtime-retire.py` checks targeted open handles, writes durable
pre-delete identities, and rechecks files and directory ownership before
removal. The three test links have separate identity/target checks. No compiler
or product is run for cleanup. HEAD/index and prior tracked working changes,
raw results/events/timings, inputs, retained source corpus, current programs,
selected dumps/full resources and entire trial App are checked unchanged.

The eleven restored terminal runtime libraries are live dependencies and are
**retained**. The two failed preflight records and restoration identities are
also retained. No user file, unique source, package, actual product or unrelated
build root is selected. No Goal or acceptance policy changes.

To replay, rematerialize separate test trees from the retained pinned manifest
and source cache, recreate their test context links and use the recorded bridge,
products and deadlines. Original inode/timestamps are not promised. See the
[comparison and replay recipe](../../releases/2026-10-30-original-text-processing.md).
