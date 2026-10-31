# CLN184: consumed combined-runtime state

The primary ledger records113,190files,19,923directories,49links;
133,162manifest rows,6,859,645,810logical and7,102,935,040
allocated file bytes. These are per-path ledger totals, not volume free space.

Post-review retirement:41,105files,
16,116directories,38links;
2,065,327,309/2,143,739,904bytes.
Automatic retirement:72,085files,
3,807directories,11links,
4,794,318,501/4,959,195,136bytes,
counted once from original private plans and completed operations.

The eight old relocation case-parent removals are proven only by completed
original rows: inode/mode/mtime are unavailable and explicitly null. Sixteen
opportunistic ancestor rmdir attempts have no recorded success; they are not
counted as proven removed directories. Old package-copy mtime is unavailable.
No historical identity or manifest was reconstructed. The new P447 ten parent
nodes were checked against their captured lifetimes and then recorded freshly
before deletion. Directory mtime can legitimately change as children leave.

Exact owned case-state subtrees and five build legs' consumed intermediates
were selected. Port links were unlinked without traversing their source targets.
Test negative-control payloads and four adopted prior helpers were consumed.
Targets were checked for open handles at finite covering roots; every selected
node was rechecked before removal. Public gzip and private plan were fsynced
before the first unlink, with per-operation durable intent/outcome records.
Two earlier preflights were deliberately stopped before creating a plan or
removing anything: repeated leaf occupancy checks, then quadratic prefix scans.
Their failed receipts remain. The final plan uses ancestor sets and fixed roots.

Retained and verified:7,459frozen inputs and
30,379evidence files and actual current products; prior product paths are unselected,
original failed build/action/continuation/preflight records, signed native
controls, P447 harness/receipts and private plans. Shared content-addressed
build owners, manuals and installed products were not selected. Restore
consumed intermediates by replaying the retained inputs and recipes; original
inode/random cache/ELN bytes are not promised. Controller metrics are primary-only.
The original cleanup postcheck failed:20 port files still required by product
receipts had been selected incorrectly. Delivery packages stayed intact.
The20 paths now exactly match original receipt SHA/bytes/mode after separate
file-only recovery;12 directories were recreated,245,682,448 logical and
245,719,040 allocated bytes restored. Original inode identity is not preserved.
Gross retirement totals include deleted nodes whose paths were later recreated;
no claim that every selected path remains absent. Fourteen paths were directly
copied; six Cocoa paths required original signature-byte recovery, checked
against pre-delete SHA commitments. Original cleanup exit1 and failed recovery
attempts are retained. All full product receipts,raw/source/App/WIP/Git checks
now pass. Fourteen reconstructible transform payloads and two empty directories were
retired separately after independent restoration review and durable public
records; their logs/results/hashes/recipes remain. The original restoration
log has20final file rows, without creation intent pairs;12created directories
are recorded in the final restoration results.

Nine pre-existing WIP contents remain; after this delivery's prefixes, the three
documents are compared by retained old bodies, not current whole-file hashes.

Manifest SHA256:d12b3fba1c6bbc0f9ac30129a97898a2722dd7a3a43667d4b61af026ff0d9784.
[Manifest](../manifests/2026-10-31-184-consumed-combined-runtime-state.jsonl.gz) ·
[Functional scope](../../releases/2026-10-31-current-native-collector.md).

Supplemental recovery-payload ledger:14files/2directories,16rows,153,343,584/153,391,104bytes. Combined gross deletion events:113,204files/19,925directories/49links,133,178rows,7,012,989,394/7,256,326,144bytes. These include old nodes whose20file/12directory paths were recreated; no net free-space claim.
[Recovery payload ledger](../manifests/2026-10-31-184-consumed-recovery-payloads.jsonl.gz).

Whole-tree own-voice checking reports warnings in unchanged treesit/runtime-resume
files and then a checker AttributeError; it did not pass. New text and complete
public ledger fields were checked separately, without claiming a global pass.
