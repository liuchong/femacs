
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

`union specbinding` (lisp.h 3572-3635) is **measured, not assumed** (3.645; the layout baseline has
no keys for it because `let.kind` is a bitfield and `offsetof` cannot name one): a clang probe over
the frozen headers with `-std=gnu23` prints `sizeof (union specbinding) = 32`, `let.symbol = 8`,
`let.old_value = 16`, `let.where = 24`, `where.buf`/`where.kbd` both at 24, `bt.function = 8`,
`unwind.func = 8`, `SPECPDL_LET = 9`, `SPECPDL_LET_LOCAL = 10`.  The port's `specbinding_let` and the
`specbinding_unwind*`/`specbinding_bt` variants match field for field.

## 2026-10-13: `c_longlong` joins the shared `core::ffi` import (w32.c's Owed 22)

`src/lisp.h` is included by nearly every unit of the tree, so its `core::ffi` import list is the
place every unit's `c_*` spellings come from - and the `nt/inc` mirrors spell MSVC's `__int64` as
`c_longlong` (`sys/stat.rs`'s `st_size`, `resource.rs`'s `rlim_t`), a name that list did not
carry.  `femacs/src/w32.rs`'s `windowsnt` probe read **three** `cannot find type c_longlong` rows
for it (one per mirror site, carried since slice 1); adding the name to this mirror's
`use std::ffi::{…}` retired all three in one edit and needed no other change - the frozen probe is
unchanged (**exit 0**, **336-byte** empty object), and the same probe's located-site count fell
**135 -> 98** in the round that made the edit (that round also spelled the socket types
`femacs/nt/inc/sys/include/socket.rs` assigns to its includers; `docs/ports/w32.md` records both).
