# CLN-20261029-116 — Failed notification-build compiler copies

The second enabled-backend terminal normal/checking build fails because its
buffer include precedes the Lisp macros. Both failures and the frozen source
are preserved. The controller exits1 after156.536s with complete cleanup;
artifact protection has been released. The corrected third build uses separate
source, Cargo and generated-unit trees.

Retired only the second run's Cargo cache and its two `objects/units` roots:
9794 files, 555 directories, 368026934 logical regular-file bytes. All selected
roots are absent. Exact content, identity, open-file absence and selected file
sets were checked again before deletion. Actual objects, frozen source and all
raw failures remain. No successful product or functional acceptance is claimed.
Logical bytes are not an APFS free-space measurement.

The gzip manifest records each file/directory and SHA256, including normalized
link targets. Restore by regenerating only these compiler copies from the
retained exact source/recipe; old inode identities are not promised.
Manifest: `maintenance/cleanup/manifests/2026-10-29-116-failed-notify-compiler-copies.jsonl.gz`.
SHA256: `34d505023f2939250bf2b04bfe1c00b0394f57dc53ed49015e3f515991809854`.
Exact private plan/operations: `build/cleanup/2026-10-29-116`.
The earlier first failed run is retained pending its controller-summary recovery.
