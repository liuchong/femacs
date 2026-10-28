# CLN-20261029-117 — First failed notification-build compiler copies

The first enabled-backend terminal normal/checking build fails name resolution
for buffer/pid_t after a gated include claims the required mirror closure.
The source and raw compiler failures remain. Its controller summary was
recovered from this work's bounded conversation output and checked against the
captured root process and own session: exit1, cleanup complete, 166.227s.
Artifact protection is released; the current third build has independent trees.

Retired only that first run's Cargo cache and two `objects/units` roots:
9794 files, 555 directories, 368026198 logical regular-file bytes. All selected
roots are absent. The exact file set, content hashes, node identities and
open-file absence were checked again before removal. Source, actual objects,
compiler receipts and raw failures remain; no successful product is claimed.
Logical bytes are not an APFS free-space measurement.

The manifest includes every file/directory identity and hash, with link targets
normalized to relative identities or hashes. Restore these compiler copies from
the retained frozen source/recipe; old inode identities are not promised.
Manifest: `maintenance/cleanup/manifests/2026-10-29-117-failed-notify-compiler-copies.jsonl.gz`.
SHA256: `72895b5b8f5156375f06d18679ee4f814ac70a05284d387c20cbd6ed72aeebc3`.
Private plan/unlink receipts: `build/cleanup/2026-10-29-117`.
This resolves CLN116's pending recovery for the earlier failed run.
