# CLN195: ended build state, product extents and exact evidence encoding

Three separate storage operations are recorded; their byte counts are not added as logical deletion.

| Operation | Files / directories | Logical bytes | Allocation or encoding |
| --- | ---: | ---: | ---: |
| Completed Cocoa native generated units | 10,036 / 344 | 335,601,791 retired | 355,405,824 per-path allocated |
| Ended Cargo release and four manual source trees | 2,408 / 512 | 215,409,244 retired | 221,495,296 per-path allocated |
| Three closed DBus test/context copies | 5,514 / 780 | 146,736,666 retired | 160,886,784 per-path allocated |
| Product APFS extent transfer | 13,032 / 0 | 1,336,308,816 preserved | 1,363,734,528 prior per-path allocated |
| Private operation evidence encoding | 2 original streams | 80,999,973 preserved | 4,135,024 gzip bytes |

Canonical retirement totals: 17,958 files and 1,636 directories, 19,594 manifest rows; 697,747,701 logical and 737,787,904 per-path allocated bytes. File path hashes bind each private canonical entry, including generated compiler names that encode machine paths; all selected nodes are absent. Rebuild generated units from retained source/configuration and Cargo caches from the frozen Cargo inputs. Staged build-owner executables and archives remain. Manual source trees can be regenerated; manual info, logs and receipts remain, including 140 live terminal product info files.

The six DBus input trees restore from the complete retained stable-context cache and pinned corpus/context manifests, not from the candidate test tree. Their 92 other nodes remain, including 36 context links whose target strings and identities are unchanged; targets can remain absent until rematerialization. Original failure, skip and retained semantic outcomes stay unchanged. The 21 old parallel-5 through parallel-8 materialization trees remain because the sealed old continuation index binds their leaves.

Product extent transfer changes physical identity explicitly while preserving bytes, mode, ownership, ACLs, extended attributes and modification times. Old 61 products are read-only provenance donors, never replacement targets; all 7,500 frozen source inputs are unselected. Private mapping records resolve old physical nodes to cloned nodes; immutable original journals remain historical and must not be used alone to assert old inode retention. The next build consumer requires this exact transition receipt. The 13,604 unmodified files and two links are witnessed separately. Signatures passed before/after transfer. Four post-integration negative cases reject wrong receipt hash, retired-inode substitution, unchanged-witness substitution and a missing consumer binding. No VM/compiler/ERT ran for this transition or its protection cases.

Both private raw streams are preserved byte for byte in separate gzip archives; complete decompression verifies original sizes and SHA-256 before removal of raw paths. Canonical transition ledgers and its result are unchanged. Logical evidence bytes deleted: zero. The public encoding manifest contains only relative recovery paths, sizes, modes and hashes; private archive contents are not published. Gunzip restores the exact streams, not their old inode or timestamps.

Per-path allocated bytes do not measure exclusive APFS physical blocks. Observed free-space changes have other contributors: extent transfer 721,059,840 to 2,007,392,256 bytes; encoding 2,184,302,592 to 2,261,921,792 bytes. Neither delta is attributed exclusively to one operation. Original results and private recovery ledgers remain. No full G3, full new 61-profile baseline, Cocoa runtime regression, G5 performance or Goal acceptance is claimed. G1/G2/G4 remain three of seven.
