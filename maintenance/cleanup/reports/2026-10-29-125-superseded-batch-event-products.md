# CLN-20261029-125 — Superseded batch-event products

The P417 terminal/Cocoa normal/checking products have been superseded by the
later P418/P421/P423 cohorts. Their original build and bounded regressions/review
are terminal, and the artifact journal is released. The original build controller
exits1 with complete cleanup in1252.652seconds; original remote failures remain
failures. This retirement does not relabel that batch as accepted.

Retire only two generated runtime family roots containing the four old products,
objects and their local resource copies: 19330 files/links,
716 directories, 1269578620 logical regular-file bytes.
No live consumer or production build reader uses them. Historical drivers still
name them and require a rebuild before replay; their paths are no longer present.
Both roots are absent after exact content/inode/flags/open-file checks and durable
delete intents. No source edits or current-product deletion are included.

Keep unique frozen working-tree source, original test inputs, all original build
logs, failures and raw results, owner receipts and build recipes. Preserve24 old
per-product metadata files privately before retirement. All14767
selected retained regular files, including current P423 programs/dumps and the
production fix, have unchanged hashes. The active P423 journal has no retired
path dependency. Logical bytes are not an APFS free-space measurement.

Manifest: `maintenance/cleanup/manifests/2026-10-29-125-superseded-batch-event-products.jsonl.gz`.
SHA256: `8562db32ec57609a0a5f1372bd004e202c6dd1cf8569e78f109c87dc1379b600`.
All20046 decoded rows, nested fields and link targets are reviewed.
Rebuild from retained exact frozen source, original driver and saved metadata;
new executable/dump/signature bytes and inode identities are not promised
identical. Original observations and deleted product identities remain traceable.
Private intents, retained-file hashes and metadata are under
`build/cleanup/2026-10-29-125`. Current notification diagnosis and Goal remain open.
