# CLN190: consumed calendar/custom state

Independent raw review accepted all 657 observations and explicitly released
this batch's consumed copies. Exactly 29 roots are removed: nine test trees,
nine context roots with 18 links, nine empty environment roots with their
labels, and two adopted P450 delivery helpers. No product/build cache selected.

Total: 13,826 files / 2,916 directories / 18 links = 16,760 manifest rows;
189,001,431 logical / 225,824,768 allocated bytes.
Post-review deletion: 13,826 files / 2,286 directories / 18 links = 16,130 rows.
Automatic retirement: 630 directories from 63 schema-3 plans, including 126
case/container parents, counted once and not deleted again. No automatic files.
The test trees contain 9 x 1,536 original inputs and 9 x 251 directories;
The post-review total includes 27 context/environment directories. Two helpers
have separate identities. Their content is not counted as pinned test input.

Canonical paths, ownership, no flags, no consumer overlap and lsof are checked.
The private plan and public gzip are fsynced before deletion. Each unlink/rmdir
rechecks identity and appends intent/completion; 32,260 operation rows survive.
All 29 roots are absent. All 572 retained evidence files, 34 protected inputs,
nine product profiles and 7,481 frozen source inputs reverify. Raw/seals/events,
stdout/stderr/controller logs, product sidecars, private plans, current/prior
products and complete frozen sources remain. Historical manifests are untouched.

An additional read-only public/plan comparison first failed because the local
Python does not support zip(strict=...). Cleanup was started after that error;
this sequencing error is recorded. The cleanup's independent sealed-plan and
per-node guards and before/after checks passed. The corrected zip_longest
comparison matches all 16,760 rows with no repeated deletion. The original
failure and correction remain in the private audit note and public receipt.

Manifest SHA256: 72144e4f3cf9729d6339a9973d01377ddd9b9eb8f0a126fc79a7236ef219168b.
Replaying requires rematerialization into a new owned output. Temporary/native
bytes, original directory inodes and timestamps are not promised recoverable.
Totals sum regular-file pathname bytes/allocation, not unique physical storage
or whole-volume free space. This cleanup changes no product or acceptance gate.

[Manifest](../manifests/2026-11-01-190-consumed-calendar-custom-state.jsonl.gz) ·
[Coverage](../../releases/2026-11-01-current-calendar-custom.md)
