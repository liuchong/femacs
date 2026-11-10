# CLN221: consumed runtime semantic state and copied packages

The completed semantic execution and subsequent exact late cleanup retired only
consumed case state, copied relocation packages and empty owned parents.
Original source/product files and raw semantic observations remain. The failed
original phase stays unchanged; [semantic evidence](../../releases/2026-11-10-current-runtime-semantics.md)
separates successful functional observations from late physical completion.

| Scope | Regular files | Links | Directories | Logical payload bytes | Per-path allocated bytes |
| --- | ---: | ---: | ---: | ---: | ---: |
| Consumed case state | 41,216 | 0 | 67,590 | 1,044,962,164 | 1,113,919,488 |
| All60 copied packages | 269,748 | 30 | 6,950 | 16,209,150,009 | 16,826,556,416 |
| Empty owned parents | 0 | 0 | 62 | 0 | 0 |
| Total | 310,964 | 30 | 74,602 | 17,254,112,173 | 17,940,475,904 |

The [complete public projection](../manifests/2026-11-10-221-consumed-runtime-semantic-state.jsonl.gz)
has385,596 eight-field rows,26,652,028 compressed bytes, SHA256
`2e706a8b6c05067c137042897ac30b6eb3a96dce0d2527008b9f751fa3f51204`.
Fields are `scope`, `path_id_sha256`, `kind`, `bytes`, `allocated_bytes`,
`sha256`, `mode`, `reason`. Directory counts are zero bytes; link digests hash
the link locator and expose no target. Logical totals follow the canonical
payload accounting and exclude link-locator text. No machine paths or process
identities appear in the public rows.

The main projection binds11,248 source scopes, validates canonical totals and each
copied package's unlink/rmdir operations, then independently decodes the full
public stream to EOF. That projection adds zero deletions. Independent review
reconstructed every scope from394 content-addressed metadata mirrors and matched
all stream digests/counts, while keeping the original source paths read-only.

After review, a real verifier subprocess rechecked all11,316 source pins and
the complete mirror set, then exited0 with complete process cleanup. Canonical
retirement removed exactly394 consumed mirror files and their empty directory:
158,178,794 logical/159,260,672 per-path allocated bytes. These auxiliary counts
are separate from the table above. All original metadata and the main public
manifest remain unchanged. The [395-row auxiliary manifest](../manifests/2026-11-10-221-reviewed-metadata-mirrors.jsonl.gz)
uses the same eight fields, SHA256
`657dff75872d0c5a689eb650aa30cfb187054f88a3532e91fc7df0d7081cea72`.

Fourteen packages had been retained for actual read-only Spotlight holders.
Once unoccupied, late retirement removed62,947 leaves and1,597 package directories,
then62 empty parents, preserving all85 bound inputs. It did not alter the
original failed receipt or rerun semantic cases. No causal volume-reclamation
claim is made from the per-path allocated totals.

Restore disposable state from the retained frozen inputs/profiles, and copied
packages from the still-retained original products. This is content-level
reconstruction, not survival of old inodes or restoration of random names.
Current source, products and SDK remain required by full stable-corpus,
performance and component consumers. Raw observations and staged format-control
inputs remain for final native artifact review; their physical read requirement
must close before later reclamation. No compatibility, performance or whole-Goal
completion follows from this cleanup.
