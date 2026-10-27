# CLN-20261027-66: consumed artwork build outputs

Removed 6885 generated files, 486132014 logical bytes and
502730752 allocated bytes from these exact scopes:

- `build/cargo-fe-final-horns`
- `build/acceptance/fe-final-horn-artwork-1/previous-Femacs.app`
- `build/acceptance/fe-final-horn-artwork-1/unit-output`
- `dist/releases/fe-iron-f`

All consuming runs had ended with their own cleanup receipts. Two exact-root
lsof checks passed. Before unlink, the hash/inode/mode/size/time ledger and
public manifest were fsynced; each selected identity was checked again. All
selected roots were absent and all 5982 retained identities unchanged
at that operation boundary. These historical identities are not a promise
that later authorized artwork edits will have the same bytes.

The original selected-horn trial/release and current artwork evidence were retained at the time of cleanup. Subsequent contour drafts are separate revisions.

The source, failures, original logs, manifests and native exit receipts remain.
Compiler caches/test fixture copies can be rebuilt from the recorded source
and lockfile. The duplicate old trial App was matched against its retained
release copy. No product behavior, support platform or unique source was removed.
The superseded `fe-iron-f` package metadata remains in the private cleanup directory; its master is recoverable from Git history. Recreating a DMG does not promise identical filesystem timestamps or checksums.

Manifest: `maintenance/cleanup/manifests/2026-10-27-66-selected-horn-artwork-outputs.jsonl.gz`

SHA-256: `52eadda012733c8f6fa06cb203fc266e0491fe8b5fcfc70fb0b1afe075122bd9`. Hash-only path IDs map to the exact private
ledger under `build/cleanup/2026-10-27-66`. This is cleanup evidence, not
additional editor compatibility or performance acceptance.
