# CLN220: consumed runtime compiler copies

The successful current-source matrix build retired consumed compiler-copy state through the existing canonical retirement path after each of its 40 normal/checking legs. All 40 receipts report `status=complete` and `all_selected_absent=true`. Their exact compressed ledger digests are bound by build results SHA256 `e3a68738aa2c8bfea97ce0900e8822a10dd55923e390b88f5d9b820e8d1091da`.

## Scope and complete projection

- 195,460 regular files and 6,800 directories: 202,260 identity rows.
- 6,503,836,334 logical bytes and 6,888,628,224 prior per-path allocated bytes.
- [Complete public manifest](../manifests/2026-11-10-220-consumed-runtime-compiler-copies.jsonl.gz): 13,196,292 bytes, SHA256 `30c3bf49eb6f9422751fd0abe2abb2c4c1c36b425a0c22d148e06273ff6a8a09`.
- Every row has exactly `scope`, `path_id_sha256`, `kind`, `bytes`, `allocated_bytes`, `sha256`, `mode`, and `reason`. Configuration/leg scope disambiguates identities; directory byte counts are zero and content digests null.

The compiler-copy projection verified each original ledger digest and its bound summary, decoded all rows, and matched per-leg and aggregate counts/bytes. That projection performs **zero new deletions**. Source receipts and private identities remain in the current build evidence. The public manifest omits machine paths and random temporary names.

## Consumed review mirrors

After independent review, the 40 temporary ledger mirrors and their empty owned directory were retired: 12,350,535 logical bytes and 12,451,840 per-path allocated bytes. These auxiliary counts are separate from the compiler-copy totals above. An intent preceded deletion; each mirror was checked by open file descriptor and exact identity, with no open handles, before unlink and parent-directory synchronization. All 40 original source ledgers were verified unchanged afterward.

The [supplemental public manifest](../manifests/2026-11-10-220-reviewed-ledger-mirrors.jsonl.gz) contains 41 rows with the same eight-field contract: 3,672 compressed bytes, SHA256 `ed9896a7e77e4cb215a83f4fac0bf4619dc309f3b86d920a8783341bef507ed5`. The private operation stream records exactly 40 verified unlinks and one empty-directory removal. The negative-preparation receipt view remains in use for the pending runtime-input audit and was not selected.

## Preservation and restoration

Frozen source inputs, finished programs/dumps/resources and runtime observations were not selected by this compiler-copy retirement. The completed build verified retention of 198,268 files and 20 links, then released its journal without failures. The build record is [here](../../releases/2026-11-10-current-runtime-matrix-build.md).

Regenerate consumed compiler state from the retained source inventory, configuration and pinned dependency inputs. Restoration means a repeatable build route, not survival of old inodes, random names, signatures or historical binary hashes. Current inputs/products/SDK remain retained for preparation and runtime acceptance. No exclusive physical volume-reclamation, compatibility, performance or whole-Goal completion claim is made.
