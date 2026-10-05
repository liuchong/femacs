
## 2026-10-05: the `<string.h>` edge (C 30) was tried and reverted - the materializer's dedupe does not fold it (3.523)

`lisp.h` 30 is `#include <string.h>`, and this chain does not carry it: `src/ralloc.rs` and
`src/widget.rs` each had to add `../lib/include/string.rs` at their own crate roots, and
`memcpy`/`memmove`/`memset`/`memcmp` reach a unit *through lisp.h* in C (P-90 (2)'s shape: a
mirror's own include list).  The edge was added here exactly as C writes it - and it broke the
build: `src/keyboard.rs` then had `lib/include/string.rs` **twice in one crate**
(`E0428 memcpy/memmove/memset defined multiple times`, both sites in that file), because the unit
reaches the same file through its own include and the materializer's dedupe compares the
`include!` **paths as written** (this file's `../../lib/include/string.rs` from `src/include/`,
the unit's `../lib/include/string.rs` from `src/`), not the resolved file.

So the edge is **owed to a tool fix first**: the dedupe has to key on the resolved canonical path
(P-89's `// [xtask dedupe]` marker is emitted for the spelling it saw).  Until then the faithful
edge stays out, and the two units' crate-root edges are the workaround.  Reading: with the edge in,
`check-all-units` read `83 unit(s), 3 failure(s)`; reverted, `src/keyboard.c` builds again and the
tree is back at HEAD.
