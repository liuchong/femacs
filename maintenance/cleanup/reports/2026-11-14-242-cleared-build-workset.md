# CLN242 - Complete build workset removal

The user explicitly requested deleting the entire build directory, including previously retained products. Both `build/` and `.femacs-cache/` are now physically absent. Neither scope contained Git-tracked files. Before deletion there were no active owned controlled runs, open handles, or command references to either scope.

| Item | Actual count |
| --- | ---: |
| Files | 1,161,012 |
| Directories | 123,044 |
| Symlinks | 865 |
| FIFO | 1 |
| Nodes | 1,284,922 |
| Logical bytes | 73,148,183,830 |
| Prior per-path allocated bytes | 75,691,778,048 |
| Protected source/WIP/index contents unchanged | 84/84 |

The bounded streaming operation recorded content hashes and node identities before deletion, used parent directory descriptors without following links, and reconciled identity, operation and public streams to EOF. The controller exited0 with cleanup complete: 298.149 seconds, peak25,968,640 bytes. Installed applications, source, uncommitted work and formal external toolchains were preserved.

After reconciliation, seven consumed audit files and their owned directory were also retired through the maintained canonical retirement API: 280,768,959 logical bytes. The million-row raw audit streams were consumed and removed instead of becoming permanent repository garbage. The compact [scope summary](../manifests/2026-11-13-242-cleared-build-workset.json) contains both tree digests, counts and closure evidence in1,775 bytes.

The old matrix, frozen export and all preparation/runtime/product paths below build are retired. Existing outcomes remain historical; they are not live inputs and must not be restored from old caches. Continue the current batch of source fixes, build only minimum necessary targets when required, then perform final clean concentrated acceptance. Do not immediately rebuild the whole matrix.

Before removal, P473 concentrated tool/component integration passed8/8 groups (8.170 seconds, four actual editor processes), and the Rust build wiring passed cargo check. The original sidebar file, rebuilt complete products, critical protection, full555/48 corpus, performance and independent acceptance remain pending. These historical results do not complete the Goal.

Byte totals are logical and prior per-path allocation observations. No causal whole-volume reclamation claim is made.

Subsequent P473 key protection passed15/15 without rebuilding products. Its consumed test output, logs and explicit evidence copy were retired (14 files,7 directories,15,177 logical bytes;21 ledger rows verified). The original manifest containment guard was preserved by declaring the synthetic fixture repository. This closes critical tool protection only; actual product and original-file acceptance remains pending.
