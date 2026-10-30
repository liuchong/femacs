# CLN167: consumed parent-ledger integration state

After P443 implementation, real integration, 22 component protections and
independent source/result review, retire only the two consumed input copies
and owned context/environments from `build/acceptance/empty-parent-original-file-1`.
The original marker file's six passes and all original results, event streams,
sealed file results, owner receipts, public/private retirement plans and timings
remain unchanged. Product/resource identities, full trial App, pinned source
corpus, HEAD/index and unrelated tracked changes are verified unchanged.

Post-review removal: 3,072 files, 508 directories and twelve context symlinks;
41,993,904 logical and 50,176,000 allocated regular-file bytes. This includes
the two empty `environments/<product>` label trees left outside per-file
retirement. Exact source-file hashes are checked against the pinned manifest,
selected roots are ignored/untracked, and targeted occupancy queries precede
removal. Existing retirement plans record child identities durably; separate
context-link and parent plans are synchronized before unlink/rmdir.
Context targets are represented publicly by hashes, without machine paths.

Twenty directories were already automatically retired by the actual file
runner. Under schema 3, these include both case roots and both containers:
all twenty have complete private pre-delete identities and public ledger rows.
They are counted once, not deleted twice. Combined: **3,072 files, 528 directories,
twelve symlinks**, **3,612 rows**. The
[manifest](../manifests/2026-10-30-167-consumed-parent-ledger-state.jsonl.gz)
SHA-256 is `8d5ff642333a2a6b5df0da25d7b578bafb00eb91d3e0e231cbbcd3805ab68877`.
Allocated-byte totals do not establish physical-volume reclamation.

All selected test/context/environment roots are absent. No editor/compiler
runs during cleanup. Component failure-injection receipts and tiny diagnostic
fixtures remain available; unrelated build/cache trees are not selected.
The source change removes the former unledgered parent-removal helper, moving
its ownership checks and two rmdir operations into existing ledger-bound cleanup.
No original source fixture, original assertion, product or old manifest is removed.
Historical missing parent metadata is not fabricated or retroactively replaced.

Replay rematerializes the complete pinned test tree and new isolated contexts
from retained inputs; random names, inode and timestamps need not match. See
the [implementation and bounded acceptance](../../releases/2026-10-30-owned-parent-ledger.md).
