# CLN164: consumed original JSON state

After 75 original observations pass and independent review finds no confirmed
defect, retire only consumed copies under `build/acceptance/g3-json-1`.
All three file processes and the owner exit 0, complete and fully reap.

The runner already retired three successful environments: zero regular files,
24 directories, zero regular-file bytes. Their original identity rows are
carried as `already_retired_by_runner`, not deleted twice.

Post-review deletion selects three manifest-verified `test` trees, three
`context/test` links and three remaining `environments` trees: 4,608 regular
files, 759 directories, three links, 62,990,856 logical and 75,264,000 allocated
regular-file bytes. Combined accounting is **4,608 files, 783 directories,
three links**, with the same byte totals. Allocated bytes do not prove physical
volume space reclaimed.

The [manifest](../manifests/2026-10-30-164-consumed-json-state.jsonl.gz) has
5,394 decoded rows, SHA-256
`ef0d0313c9fba94eaae729b932fb73240e9cfcdba6cab46742e01a56e2a43c79`.
Post-review identities match private pre-delete plans; earlier automatic rows
match retained original ledgers. Public link targets are repository-relative.

Existing `tools/runtime-retire.py` checks targeted open handles, durably records
identities before deletion and verifies file/directory ownership. Context links
have separate identity/target checks. No product or compiler runs for cleanup.
Raw results/events/timings and automatic receipts, HEAD/index/tracked working
changes, program/dump/full resource identities and the entire trial App are
verified unchanged. All selected consumed roots and links are absent afterward.

Retain the pinned source corpus, actual products and eleven live terminal
libraries. No user configuration, package, unique source, unrelated build root,
acceptance policy or Goal changes. Replay rematerializes test copies and fresh
isolated environments/context links with original products, dumps, assertions
and deadlines; exact random names, inode numbers and timestamps are not promised.
See the [comparison and replay recipe](../../releases/2026-10-30-original-json.md).

Accounting correction: the automatic runner also removed two empty parent
directories per successful file, six here. Thus the original 783-directory
ledger total excludes six receipt-recorded empty parents: 789 total removed.
Those parents lack individual durable full pre-delete identities; this gap is
not repaired by inventing data or changing the original manifest/hash. See
[CLN166](2026-10-30-166-consumed-buffer-marker-state.md) and the planned narrow
runner correction. Regular-file byte totals and all product results are unchanged.
