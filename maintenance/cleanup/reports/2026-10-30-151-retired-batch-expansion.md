# CLN151: retired owner-batch candidate expansion

P436 was not adopted and production is restored to the measured prior. Final
independent source/result/receipt review finished and its signed delivery is
`76b930d0a205c9ea07cb9faa3686b40772da1b0b`. Retire only the completed
`build/acceptance/gc-owner-batch-batch-1/source` expansion: its runtime consumers
have ended. Retain the prior cohort for the next source-bound collector work.

## Source recovery and retained evidence

All 7,298 original source identities were verified before removal. Of these,
7,296 map exactly to retained prior source; the two changed inputs, `PORTING.md`
and `src/runtime_gc_collector.rs`, have independent content/mode/hash-checked
copies. The complete mapping is retained with the original source inventory.
After removal, every one of these 7,298 recovery identities is verified again.
The candidate collector also reconstructs from the committed inert CLN150 patch.

Twenty-four product/config/link metadata files are copied exactly before
removal and retained with the source recovery material. The original build
owner, source inventory, product manifests, complete raw measurements, result
hashes and recovery patch remain. Recovery copies and their directories have
been explicitly synced. This is a small exact-source/recipe handoff, not a new
archive of the complete generated tree.

## Actual removal and guards

Removed 20,697 regular files, 943 directories and 12 symbolic links, totaling
1,500,543,266 logical and 1,546,506,240 allocated regular-file bytes. Allocation
is not physical volume reclaim proof. Links are unlinked as entries without
following targets, after durable identity capture and a final inode/mode/target
check. Their public targets are repository relative; original target bytes and
machine identities remain private. Regular files/directories use the existing
production retirement guard, successful reaped process receipts, exact content
identities and bounded occupancy checks. Protected/unknown entries are refused.

The [21,652-row manifest](../manifests/2026-10-30-151-retired-batch-expansion.jsonl.gz)
has SHA-256 `8fc0a8aff994f7795035f9b88ce3dbc86254bef564023c92b0c9901d190bb3d3`.
All decoded file/directory fields match their original pre-delete plans; all
12 link rows match the separately captured link operations. The selected source
root is absent; prior program/dump/full-resource identities, raw detailed results,
Git HEAD and index remain unchanged. Current trial App and unrelated WIP are
outside the selection. No compiler, product or benchmark was rerun for cleanup.
The controller exits 0 with complete cleanup in 15.784 seconds and 133,038,584
peak accounted bytes.

## Replay boundary

**The historical batch product paths no longer run directly.** Restore exact
source from the retained prior mapping plus the two unique-source inputs, then
rebuild generated programs/objects/resources with the retained owner/config and
metadata. Verify new binaries, signatures and behavior before use; byte-identical
binary reproduction is not promised. Original batch products/measurement input
manifests remain historical identities and are not silently rewritten. The prior
cohort has the source-recovery and next collector-work consumers; its eventual
retirement must first transfer those dependencies.
