# CLN166: consumed original buffer and marker state

After complete execution and independent review, retire only consumed state
from `build/acceptance/g3-buffer-markers-1`. Preserve 1,323 passes, three
shared graphical-mode skips, original aggregate false and owner exit 1.
All nine file processes exit 0 complete, and the controller records full cleanup.

The runner already retired six successful marker/editing environments: zero
files, **48 ledgered child directories**, plus **12 empty parent directories**
(case root and container per environment). The parent deletion receipt exists,
but those twelve parents lack individual durable full pre-delete identities.
The manifest explicitly marks that gap; preparation device/inode is included
only for containers where actually recorded. No identity is reconstructed or
presented as original. This is an accounting limitation, not a product failure.

Post-review deletion selects three manifest-verified test copies, three
`context/test` links and remaining owned environments: **4,612 files,
792 directories, three links**, 63,024,972 logical and
75,313,152 allocated regular-file bytes. This includes three
four-byte fixture auto-save files and one 34,104-byte reference trampoline.
The fixture input and generation mechanisms remain available for replay.

Combined: **4,612 files, 852 directories, three links**. Of these directories,
840 have full listed identities and twelve have only the retirement evidence
specified above. The [manifest](../manifests/2026-10-30-166-consumed-buffer-marker-state.jsonl.gz)
has **5,467 rows**, SHA-256
`eaf4af92f0b9628ce4189b9af980ccb972d27b1a3ce94cc1d0c0bb6894f407ac`. Already retired rows are not deleted twice.
Allocated bytes do not prove physical-volume reclamation.

Post-review deletion uses targeted open-handle checks and the existing helper's
durable identities, rechecking hashes/ownership before removal. Original full
test hashes are reverified. Separate link targets/identities are retained.
Raw results/events/timings and prior automatic receipts, HEAD/index/tracked
working changes, programs/dumps/full resources and trial App remain unchanged.
All selected consumed roots and links are absent; source corpus, live terminal
libraries and unrelated work remain. No product/compiler runs for cleanup.

Replay recreates complete test trees, isolated empty environments/context links
and the small fixture-generated files as needed; original random names and
inode/timestamps are not promised. Next narrow tooling work must record parent
identities durably before `rmdir`, without inventing missing historical data.
The same accounting omission affected earlier auto-retirement batches: their
child-ledger totals do not include the two empty parents per successful file.
For CLN164 specifically, 783 ledgered directories plus six receipt-recorded
parents means 789 total; its original manifest/hash remain unchanged.
See the [functional comparison](../../releases/2026-10-30-original-buffer-markers.md).
