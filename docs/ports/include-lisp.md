
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

## 2026-10-05: the `DEFVAR_*` macros type-check their name argument (status 3.617)

Each of the five `DEFVAR_*` macros now starts its body with `let _: &core::ffi::CStr = $lname;`
before the `unsafe` block: C's `defvar_lisp` reads the name with `strlen`, so it must be a
NUL-terminated C string, and a `c"..."` literal is the port's spelling of one.  A plain `"..."` -
which 141 call sites used, registering their variable under a name that ran into the next literal -
is now a compile error rather than a silent defect.

The five `DEFVAR_*` macros now build their forwarding object in the **static initializer**
(`u: Lisp_Fwd_u { objvar: std::ptr::addr_of_mut! ($vname) }`, and `kbdoffset: $vname as c_int`),
the way C's macros do with `.u.objvar = &globals.f_##vname`.  A run-time assignment is invisible to
a process that has just *loaded a dump* - `syms_of_*` never runs again - and left every one of these
objects NULL (status 3.644); `tools/check-fwd-static-init.py` holds the shape.
