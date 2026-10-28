# CLN-20261029-118 — Consumed third notification-build compiler copies

The third P418/P419 integration completes terminal normal/checking builds,
then fails Cocoa normal/checking because three local open flags duplicate the
shared fcntl mirror. Controller exit1 after831.254s and complete cleanup are
retained. Artifact protection is released. The fourth integration has separate
source/Cargo/unit trees and starts with Cocoa; backend acceptance remains open.

Retired only the third run's Cargo cache and four generated `objects/units`
roots: 19662 files, 897 directories, 697483002 logical regular-file bytes.
Each selected file set, identity and content was rechecked, with no open files,
before unlinking. All five roots are absent. Frozen source verifies afterward;
both terminal programs/dumps and completed build receipts remain, along with
actual objects, raw Cocoa failures and compiler receipts. No functional pass is
inferred from the completed terminal builds. Logical bytes are not an APFS
free-space delta.

The compressed manifest records every selected file/directory and its hash.
Absolute link targets are represented by relative identities or hashes. Restore
only the retired compiler copies from the retained frozen source and exact
recipe; old inode identities are not promised.
Manifest: `maintenance/cleanup/manifests/2026-10-29-118-failed-cocoa-notify-compiler-copies.jsonl.gz`.
SHA256: `e1b548ba56a85e9c9888a8c88f0b158911c91325053c5725a302c809fe7dae18`.
Private plan/operations: `build/cleanup/2026-10-29-118`.
