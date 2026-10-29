# CLN137: consumed background-repair compiler and App copies

P432's fresh Cocoa build, strict signature, 16 native background/GC cases and
four caller branches passed; an old-receiver mutant was rejected in two cases.
The normal configuration subsequently completed startup with 443 package
entries and no initialization error. The old binary's negative-fixture
consumer also ended. This batch retires consumed copies, not product sources,
installed user packages or an unsupported platform.

## Exact scope and manifest

- The isolated repair checkout's Cargo cache and generated Cocoa units.
- Its App copy, identical in every member to `dist/macos/Femacs.app`.
- The previous broken standard App after its negative diagnostic consumer ended.
- The two compiled caller-protection executables; generated source, logs and
  positive/negative results remain.

Six roots contained 15,077 regular files, two links, 625 directories and
811,763,529 logical bytes. The compressed
[manifest](../manifests/2026-10-29-137-consumed-background-repair-copies.jsonl.gz)
has 15,704 rows, SHA-256
`0cbf155d36e4e519f4b0a6b4b57831d08bcde84d2188031282914c720d195a85`.
All 5,974,743 decoded bytes/fields match the private identity plan after
explicit path normalization; 4,847 derived aliases avoid publishing local
source names. All 31,408 intent/result operations pair exactly. Logical bytes
are not a promise of equivalent physical space on a copy-on-write filesystem.

## Preservation and recovery

Named compiler/test/negative/normal-startup owners had exited with complete
controller cleanup. Selected roots were ignored, untracked, nonsymlink roots
with no open handles. Each member and directory identity was checked again
before removal; all selected roots are now absent. Git HEAD/index, tracked
checkout inputs, retained source/object identities, products and evidence
remained unchanged. No editor or test was rerun for cleanup.

Retained: the repair checkout and its Git metadata, actual compiled object
files, canonical new App, terminal pair, original G1 rollback App, previous
disk image, source/data/manual resources, all initial failures, debugger raw
and derived records, source bindings, signatures, normal-startup receipts and
unrelated work. User package installations are retained and outside this batch.

Regenerate Cargo/units and component executables from the recorded source and
commands; original inodes/timestamps are not promised. Restore the new duplicate
from the identical canonical App. The old broken App is recoverable from the
retained prior disk image for diagnosis, not normal use. No Git object or
unique source was reclaimed. See [the defect record](../../incidents/2026-10-29-cocoa-background-receiver.md).
