# CLN222: superseded P453 build and App backups

After P458 clean rebuild, real three-product functional acceptance, subsequent
VM protections, local/TTY acceptance and packaging passed, retire only these
three owned backups. Independent review accepted the complete source/actual
result unit and the exact cleanup plan before apply.

| Scope | Files | Links | Directories | Logical bytes | Prior allocated bytes |
| --- | ---: | ---: | ---: | ---: | ---: |
| `build/.terminal-before-p458` | 5,179 | 6 | 215 | 232,365,042 | 242,999,296 |
| `build/.macos-before-p458` | 95 | 6 | 45 | 87,372,625 | 87,576,576 |
| `dist/macos/.Femacs-before-p458.app` | 4,486 | 1 | 113 | 229,128,499 | 239,407,104 |
| Total | 9,760 | 13 | 373 | 548,866,166 | 569,982,976 |

The original backup root inodes bind the admission snapshot. Each scope has
zero open holders before planning and apply. The unchanged prior per-node
cleanup implementation verifies type, ownership, identity, mode, file hash
and timestamp or link target, then rechecks the entry through its parent FD
before unlink/rmdir. The private stream records all10,146 operations.

Actual apply exited0 in3.59s, peak65,732,872 bytes, complete process cleanup.
All selected roots are absent. Eight protected current source/program/dump
and P453/P458 fallback/release paths are unchanged; the running installed App
and the frozen runtime matrix were outside selection.

The [10,146-row public manifest](../manifests/2026-11-10-222-superseded-p453-build-app-backups.jsonl.gz)
is SHA256 `7efff6243458d206d9d3fbf5c2ae931c5aed9f02df47f797f6ce9aef61389bec`.
Every eight-field row was streamed against the private plan and actual
operation ordinal/path/hash, decoded through EOF, and checked for private
paths. Symlinks publish only the hash of target text. Publication adds zero
deletions.

The observed volume free-space change was41,674,825,728 to41,964,433,408 bytes.
This is a whole-volume observation, not causal proof of physical reclamation;
the table records prior per-path allocation and logical payload. Recovery
uses retained Git sources and dependency recipes; exact historical binary
hashes and inode identities are not promised. The reviewed P453 release
image remains available. [Current replacement](../../releases/2026-11-10-bytecode-indirect-call-frames.md).
