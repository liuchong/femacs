# CLN158: consumed final remote diagnostic state

Scope: only `environments` and `test` beneath
`build/acceptance/remote09-diagnostic-final-single-1`. The one actual pinned remote
case passed and its process exited zero, complete, without remaining-group
termination. An occupancy query found no open handles before deletion. Neither
selected tree contains a symlink or tracked input.

The durable pre-deletion ledger binds all 2 regular files and 15 directories by
content hash or path-identity hash, mode, size and allocated bytes. Private inode,
device, modification-time and original path identities were checked again before
each removal. Both selected trees are absent. Retired regular-file totals are
72,046 logical and 77,824 allocated bytes; no physical-volume reclaim is claimed.

The original `results.json` remains SHA256
`63376eb9415028eeaf46d4e24c52f8a5b353c1dd5fc3d9a0d5776478430a145d`.
All original result/process files and diagnostic bindings are retained; program,
dump, runtime resources and production sources are unchanged. No failed run,
user configuration, capture or installed product is selected.

Recovery: materialize the pinned 31.1 test from the tracked source manifest;
its retained source hash is
`8d812305c5c31652164b9da8495beb4d2572df8e6d1c504e7d47408d0aadd031`.
The consumed mock-shell environment is reproducible through the normal isolated
runner; original random names are not promised. The diagnostic remains accepted
only as a diagnostic: this pass captured no historical bad input and does not
establish that the intermittent failure is fixed.

[17-row public manifest](../manifests/2026-10-30-158-consumed-final-remote-diagnostic-state.jsonl.gz),
SHA256 `c3d5bf8f83b668c3351adaebd83c6fe0f3e981859a577580d0d94beab438054b`.
All decoded fields were checked: hashes, numeric metadata and directory kind only;
no machine paths or raw private payloads are published.
