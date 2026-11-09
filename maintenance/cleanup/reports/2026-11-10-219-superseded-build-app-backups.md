# CLN219: superseded build trees and App backups

After P453 rebuilt both products, passed closure/runtime protection and local acceptance, and produced the new verified local package, retire exactly the following owned superseded roots. Every selected node was inventoried and hashed before deletion; root occupancy queries were empty. Installed Fe was still running and was not selected. Current sources, products, new package, original observations and reference installation are preserved.

| Scope | Files | Links | Directories | Logical bytes | Prior allocated bytes |
| --- | ---: | ---: | ---: | ---: | ---: |
| `build/.terminal-before-p453` | 5856 | 6 | 216 | 216803420 | 230117376 |
| `build/.macos-before-p453` | 93 | 6 | 45 | 64480658 | 64684032 |
| `build/.terminal-p453-sdk-refused` | 58 | 0 | 32 | 16379552 | 16490496 |
| `dist/macos/.Femacs-before-g2-standard.app` | 2977 | 0 | 94 | 158153106 | 165310464 |
| `dist/macos/.Femacs.before-current-core-1.app` | 4476 | 1 | 113 | 228737185 | 238993408 |

Total: **13,460 files / 13 links / 500 directories**, 684,553,921 logical and 715,595,776 prior per-path allocated bytes. All 13,973 public rows match the private identities and operation ordinals; every selected node is absent. Link payload lengths are included in the logical per-path totals. Shared/COW extents make these counts different from physical volume reclamation.

The two App backups are bound by executable hashes `7727b278a23797e904ef121212dbf8efc446c607d571efb470934e7c90dbfa73` and `b2fe7420ad1c3a041ea33ea9d0ccf80b852b818ea3315ffa65dd1aff84b2bcb0`, as recorded in the pending-composition and current-G1 delivery reports. Their historical UI observations remain valid under those original identities; retirement does not relabel them as current-product tests. The old standard build roots were retained by inode before the fresh P453 builds; the third tree is the refused SDK preparation, not a successful product.

Recovery: reconstruct consumed build state from the retained Git source, independent target recipes and recorded dependency identities. Historical inodes, random names, signing bytes and exact old program hashes are not promised by a rebuild. Keep the current standard products and the independently captured 0.27.1 SDK for the next authorized build; they are not garbage.

Volume free bytes were observed at 142,434,181,120 before and 142,979,215,360 after. No exclusive causal reclamation claim is made.

[Complete public identity manifest](../manifests/2026-11-10-219-superseded-build-app-backups.jsonl.gz) — SHA256 `d23ee53bf57d9ba14c3b5aa68e9d20b2fa3d91ca58ced815dd74aa73f7ec5865`. Private pre-delete identities and operation log remain with the cleanup receipt; their raw machine paths are not published.

No editor, compiler, user data, active installed App or old compatibility evidence was deleted. This cleanup is not whole-Goal acceptance.
