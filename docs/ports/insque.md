# emacs/oldXMenu/insque.c (57 lines)

> Filled in: the port is `femacs/oldXMenu/insque.rs`.  Until this round it was the empty unit that
> docs/status.md 3.31 prescribed; P-86 ① overturns that reading ("3 行的头注释不是'落完'，是'还没开始'"),
> and `oldXMenu/Internal.rs` is the sibling that already carries this directory's shape.

**File gate**: `feature = "have_x_windows"` (P-86 clause 8).  The reference text carries no macro of
its own for this file: `oldXMenu/Makefile.in`'s `OBJS` block lists the object unconditionally, and it
is `configure.ac` 7287-7295 (`HAVE_X11 = yes`, `HAVE_GTK != yes`, `USE_X_TOOLKIT = none`) that decides
whether the library is built at all.  `oldXMenu/Internal.rs` 8-16 records the same reading, and
`## Deviations` 1 below keeps the reference's own words for it.

**Nothing in the frozen configuration compiles this unit**: `HAVE_X11`, `HAVE_X_WINDOWS` and
`USE_X_TOOLKIT` are `undef` in `docs/baseline/config/darwin-arm64.macros.tsv`, so `xtask build-tu`
leaves the 336-byte empty object (P-08), and this target has no X11 window system to run the arm
against.  The arm reading below is therefore a `rustc` reading of the materialized mirror graph.

## C line ranges

| 1-15 | the FSF licence | dropped (P-86 ①) |
| 17-20 | the `emacs_mumble` renaming note | carried as prose in the port |
| 22 | `#include "XMenuInt.h"` | the file gate, through `mod xmenu_int` (P-21) |
| 24-28 | `struct qelem` - `q_forw`, `q_back`, `char q_data[1]` | the file gate |
| 30-43 | `emacs_insque` | the file gate |
| 45-57 | `emacs_remque` | the file gate |

## Evidence

| reading | value |
| --- | --- |
| `xtask build-tu oldXMenu/insque.c` (frozen) | exit 0, `build/femacs/insque.o` **336 bytes** (P-08's empty object) |
| arm: `probe oldXMenu/insque.rs … --cfg 'feature="have_x_windows"'` | **247 error messages → 4** after the two repairs below (the four are `XFontStruct` x2, reported twice) |

The arm read 121 error messages on the first take (247 in `insque`): the dominant row was
`E0425 cannot find type c_int` because XMenu's headers include no `lisp.h`, so each unit brings the
fixed-width set itself (`oldXMenu/Internal.rs` 63-66 is that shape), and the second class was the X11
names `Display`/`XID`/`Window`/`Pixmap`/`GC`/`XEvent`/`Cursor` that
`oldXMenu/include/X10.rs`, `XMenu.rs` and `XMenuInt.rs` had left "for the unit that eventually uses
this header to supply".  A name declared at a unit's crate root is not in scope inside the nested
module that `include!`s those mirrors, so supplying them from the unit cannot fix them.
`oldXMenu/include/XMenuInt.rs` now carries XMenuInt.h 46's own edge
(`include!("../../src/include/xlib.rs")`, P-90 clause 1) and that class is gone; `Internal.rs`'s own
arm went 54 error messages on the same take.

## Deviations

1. **The file gate is ours, not the reference's**: `oldXMenu/Makefile.in`'s `OBJS` block lists the
   object unconditionally and the configure arm decides whether the library exists (P-86 clause 8;
   `oldXMenu/Internal.rs` records the same reading).
2. **`register` has no Rust counterpart**: C's storage hint is dropped and the declared types are
   kept (PORTING.md 1.2).
3. **C 1-15 is not transcribed**: the MIT/FSF notice and the file's description block -
   PORTING.md 0.1-19, which the P-86 class keeps as the `// femacs: port of …` first line.

## Owed

1. **`XFontStruct` is still missing from `include/xlib.rs`** (integrator): the arm's remaining row is
   exactly `E0425 cannot find type XFontStruct`, raised by `oldXMenu/include/XMenu.rs` 274 and 288 -
   XMenu.h's own public API names it.  `oldXMenu/Internal.rs` declares the record in-unit today
   (clang-measured, its 100-130, with `XCharStruct`); P-90 clause 1 wants it in the shared mirror with
   the in-unit copy removed in the same move.
2. `X11/Xlib.h`'s `True`/`False`/`NULL` have no mirror home either; `SetPane.rs` declares `False` at
   its use site (P-88 clause 1) and C's `NULL` test is `is_null ()`.
3. These arm readings are `rustc` readings of the materialized graph, not a build of the XMenu
   library: nothing here links or runs (this target has no X11 window system).
