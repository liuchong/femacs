# CLN-20261027-65: consumed F/epsilon artwork outputs

Removed 6812 generated files, 508068131 logical bytes and
524550144 allocated bytes from the exact consumed scopes:

- `build/cargo-fe-f-1`: finished packaging build/test cache.
- `build/acceptance/fe-f-epsilon-artwork-1/unit-output`: passed test fixtures.
- That acceptance directory's `previous-Femacs.app`: replaced trial copy.
- `dist/releases/fe-iron`: superseded epsilon-only package.

Both exact-root occupancy checks passed; no selected disk image was mounted.
The per-file hash/inode/mode/size/time ledger and public manifest were fsynced
before deletion, then each identity was checked again. All selected roots are
absent and all 5980 retained identities remain unchanged.

The F/epsilon master, current trial App and `dist/releases/fe-iron-f` remain.
Raw failure evidence and older build inputs were outside this cleanup. Rebuild
the tool from committed source/lockfile; the previous artwork is in Git history.
Previous package metadata remains in `build/cleanup/2026-10-27-65`. Recreated
DMG filesystem timestamps and checksums are not promised to match.

Manifest: `maintenance/cleanup/manifests/2026-10-27-65-consumed-f-artwork-outputs.jsonl.gz`

SHA-256: `05ea3a65d1f208255b739ff0350fa32a3a7fe7647a5cdb6a826109b1c47bae97`. Hash-only public path IDs map to the exact
private ledger under `build/cleanup/2026-10-27-65`. This is cache retirement,
not additional editor compatibility or performance acceptance.
