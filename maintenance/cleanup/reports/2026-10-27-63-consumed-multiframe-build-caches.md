# CLN-20261027-63: consumed receiver build caches

After the P-394 real Cocoa build and user-confirmed multi-frame input passed,
retire the consumed compiler/Cargo caches and first signed branding copy. The
first copy failed a too-strict whole-Mach-O hash assertion because signing
changes signature bytes; the corrected package verifies the non-signature
payload. Every file in that retired app copy matches the retained successful app.

- Exact roots: the `g1-frame-receiver-cocoa-build-1` output's generated unit cache
  and Cargo cache; `dist/releases/fe-macos-p394` failed packaging copy.
- Removed: 8153 regular files, 357814435 logical bytes,
  375431168 allocated bytes. These are attributed file totals.
- Preserved: 16015 source/product/evidence identities unchanged,
  including the frozen source, hardware-watch failure, successful input and
  protection receipts, current trial/release apps and original build archive.
- Two exact-root open-file checks passed. Revalidate every selected inode, mode,
  size, modification time and hash before unlink. No symlink traversal, broad
  sweep, source cleanup, process termination or new acceptance run.

The public per-file manifest is
`maintenance/cleanup/manifests/2026-10-27-63-consumed-multiframe-build-caches.jsonl.gz`,
SHA-256 `06c3e9cfa3b8b36ab371d6bf1384e4295e2445f4dd86edcf78806b5932b68003`. Its path IDs map to the exact private ledger at
`build/cleanup/2026-10-27-63/private-ledger.json`; raw private paths are not
published. Manifest and ledger were fsynced before deletion. All selected roots
are absent; every preserved identity was checked again after deletion.

Rebuild caches from the preserved frozen inputs and retained build-driver
receipt. Regenerate the retired branding copy from that exact native product and
the packaging source; the successful current release is retained. This report
records retirement, not whole-G1 or whole-Goal acceptance.
