# CLN-20261027-64: artwork replacement and consumed release outputs

The user rejected the flat letter monogram and then the literal anvil direction.
The final committed master is a transparent iron/oxide epsilon sculpture.
Retire the two uncommitted SVG masters and the consumed caches/intermediate
packages; retain the current source, failure evidence and final trial/release.

## Uncommitted artwork retirement

| Previous source | SHA-256 | Bytes | Replacement / recovery |
| --- | --- | --- | --- |
| `packaging/assets/fe-logo.svg` | `56130a7d9c1b15a1f414a3bb7ffc648d2382da7410f4c5854cf6ac019d9993f0` | 2183 | `packaging/assets/fe-logo.png`; original bytes in exact private backup and prior frozen source |
| `packaging/assets/fe-app.svg` | `9e276d6de1e7411b29f8b6c597f9054d9d2ebc0070a3cc0d9838b127d4ae0df2` | 2223 | `packaging/assets/fe-logo.png`; original bytes in exact private backup and prior frozen source |

These two files were copied to exact private backups and checked by SHA before
unlink; the private ledger records paths, hashes, sizes and backup locations.
The first artwork retirement did not explicitly fsync that ledger before unlink.
That process gap is disclosed rather than represented as the later durable cache
protocol; the intact copies/ledger are now explicitly synced. No source bytes are
lost. The rejected anvil raster was never a repository input and remains in the
image tool's original storage; this operation does not remove it.

## Consumed output retirement

Removed23363 regular generated files,1584540183 logical bytes,
1638494208 allocated bytes, from these exact scopes:

- The final normal Cocoa build's generated units and Cargo compiler cache.
- `build/cargo-packaging` and the superseded private packaging-driver copy.
- The two intermediate monogram release directories,
  `dist/releases/fe-macos-p394-2` and `dist/releases/fe-macos`.
- The three replaced trial-App backups inside `fe-delivery-1`, `fe-delivery-2`
  and `fe-delivery-3`. Exact old-file comparisons pass against their matching
  frozen or intermediate original products before removal.

All processes consuming these outputs had ended. Two exact-root lsof checks
passed. Before deletion the per-file hash/inode/mode/size/time ledger and public
manifest were fsynced, then every selected file identity was checked again. All
selected roots are absent; all16013 retained identities are unchanged.
This preserves the current PNG/source, current trial App, final `fe-iron` App/DMG,
frozen source/products, original input failures, native receipts and build archive.

Manifest: `maintenance/cleanup/manifests/2026-10-27-64-consumed-multiframe-build-caches.jsonl.gz`,
SHA-256 `4e98376fd44ad44bcad60e61455b541b19d0697f366cf0d7cc8b216e3caa1e75`. Hash-only public path IDs map to the exact private
ledger under `build/cleanup/2026-10-27-64`. Retired release/driver metadata and
original artwork copies are kept there; no private conversation data is published.

Rebuild caches/tools from committed or exact frozen source and lockfile. Restore
old trial packages from their recorded frozen products; intermediate monogram
packages use the prior frozen packaging source and icon masters. New disk-image
creation is not promised to reproduce old filesystem timestamps/checksums. The
current supported delivery is `dist/releases/fe-iron` and `dist/macos/Femacs.app`.
This is cleanup evidence, not new compatibility or performance acceptance.
