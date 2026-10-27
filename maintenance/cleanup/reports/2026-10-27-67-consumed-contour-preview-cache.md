# CLN-20261027-67: consumed artwork build outputs

Removed 4058 generated files, 289754448 logical bytes and
299536384 allocated bytes from these exact scopes:

- `build/cargo-fe-horn-shoulder`
- `build/acceptance/fe-horn-shoulder-artwork-1/unit-output`
- `build/acceptance/fe-horn-shoulder-artwork-1/previous-Femacs.app`

All consuming runs had ended with their own cleanup receipts. Two exact-root
lsof checks passed. Before unlink, the hash/inode/mode/size/time ledger and
public manifest were fsynced; each selected identity was checked again. All
selected roots were absent and all 9002 retained identities unchanged
at that operation boundary. These historical identities are not a promise
that later authorized artwork edits will have the same bytes.

Both initial selected-horn and intermediate contour App/releases remain available. The exact lightweight preview driver was copied to its own retained acceptance directory and reproduced all four existing preview outputs byte-for-byte before compiler-cache retirement.

The source, failures, original logs, manifests and native exit receipts remain.
Compiler caches/test fixture copies can be rebuilt from the recorded source
and lockfile. The duplicate old trial App was matched against its retained
release copy. No product behavior, support platform or unique source was removed.

Manifest: `maintenance/cleanup/manifests/2026-10-27-67-consumed-contour-preview-cache.jsonl.gz`

SHA-256: `a409a0830bcb17853e25a80036499dc14622f049105f1f3dac533c4f455963a1`. Hash-only path IDs map to the exact private
ledger under `build/cleanup/2026-10-27-67`. This is cleanup evidence, not
additional editor compatibility or performance acceptance.
