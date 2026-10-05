
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

## 2026-10-05: two mirror defects the first runs exposed (status 3.612, 3.614)

1. **`FOR_EACH_TAIL`/`FOR_EACH_TAIL_SAFE` shadowed their caller's variable** (status 3.612).
   C's `struct for_each_tail_internal li = { tail, 0 };` *reads* the caller's `tail` and the
   body's own `tail = XCDR (tail)` advances it, so C's post-loop `NILP (tail)` sees what the
   stepper left.  The port added `let mut $state: Lisp_Object = $state;`, which shadowed the
   caller's variable: the loop left the caller's copy stale, the call sites grew a fabricated
   `tail_end` (44 places) and one of them (`plist_put`) checked the wrong side of the step.  The
   shadow line is gone; the loop variable is the caller's, and the `mut` the macro needs is on
   the caller's binding.
2. **`obarray_iter_at_end` used `NILP` where C compares with the fixnum zero** (status 3.614).
   An empty obarray bucket holds `make_fixnum (0)` - the fixnum, not `Qnil` - so the port walked
   into every empty bucket as if it held a symbol and skipped the `nil` symbol itself; the first
   `map_obarray` (from `harmonize_variable_watchers`) died in `XSYMBOL`.  The test is now C's
   `!BASE_EQ (obj, make_fixnum (0))` and the conversion is C's `XBARE_SYMBOL` (not `XSYMBOL`).
