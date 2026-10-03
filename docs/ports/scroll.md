# emacs/src/scroll.c (993 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 9
- [x] calculate_scrolling  (line 85, static)
- [x] do_scrolling  (line 236, static)
- [x] calculate_direct_scrolling  (line 422, static)
- [x] do_direct_scrolling  (line 641, static)
- [x] scrolling_1  (line 794)
- [x] scrolling_max_lines_saved  (line 833)
- [x] line_ins_del  (line 888, static)
- [x] ins_del_costs  (line 906, static)
- [x] do_line_insertion_deletion_costs  (line 962)

## Macros defined — 0

## Types and their fields — 1
- [x] matrix_elt: int writecost, int insertcost, int deletecost, int insertcount, int deletecount, int writecount

## File-scope objects — 0

## Decisions

1. **C 52-57's two `static` prototypes** (`do_direct_scrolling`, `do_scrolling`) have no Rust
   counterpart: Rust has no forward declarations and the definitions below are the same items
   (PORTING.md 0.1-12, the `region-cache.rs` note).
2. **`#ifndef HAVE_ANDROID` (C 26) wraps the whole file** and C 993's `#endif` closes it; C has
   no text between them.  HAVE_ANDROID is undefined in the frozen configuration, so the whole
   file is live and the dead arm is an empty translation unit - nothing to gate (the contrast
   with lib/fsusage.c, whose dead arms are other platforms' bodies).
3. **`#ifdef GLYPH_DEBUG`** (C 262-320's `CHECK_BOUNDS` definition and its use at C 372-373) is
   dead text: GLYPH_DEBUG is `#undef` in config.h, so C never defines the macro.  The port
   keeps both arms behind `feature = "glyph_debug"` (P-51), the feature dispextern.rs already
   uses for `CHECK_MATRIX`'s dead arm; `xtask dead-arms` does not flip this one (its FLIPS list
   is the conditional-macro table's), so the gated arm is compiled only by hand.
   `CHECK_MATRIX` itself is used *live* at C 366 and C 675: its live arm is C's `#else`,
   `((void) 0)` (dispextern.rs keeps it), so `CHECK_MATRIX! (current_matrix);` neither checks
   nor evaluates the argument on this target.
4. **Five dispextern.h declarations have no mirror yet** (C 3905 `mirrored_line_dance`, C 3936
   `set_terminal_window`, C 3945 `ins_del_lines`, C 3953-3954 `string_cost`/`per_line_cost`),
   and `do_scrolling`/`do_direct_scrolling`/`ins_del_costs` call them.  Each is declared in a
   function-local `unsafe extern "C"` block at its use site - the shape doc.rs and casefiddle.rs
   established for a declaration no mirror carries - rather than at file scope, so that the
   dispextern mirror can add them later without an `E0428` on this unit.  The port owes nothing
   here: the dispextern ledger is still missing those five items, so nothing is duplicated.
5. **`frame.current_matrix` is `*mut c_void`** in the frame mirror (P-72 rule 3: frame.h cannot
   name `struct glyph_matrix`), so `scrolling_1`'s two uses cast once with `as *mut glyph_matrix`
   and the C `frame->current_matrix` tokens stay in the text.
6. **C's object-like macros `FRAME_INSERT_COST (frame) = ...`** (C 971-982, four assignments):
   frame.rs mirrors those macros as functions returning the field, and a function call is not a
   place expression, so the assignment target is spelled `(*frame).<field>` while the right-hand
   side keeps `FRAME_INSERT_COST (frame)` (P-37 keeps C's l-value semantics: the field *is* the
   place C writes).  All four keep C's unusual order - insert, delete**n**, insert**n**, delete.
7. **Loop shapes follow the P-75/region-cache conventions**: C's `for` loops become
   `while` + a step at the body's end; C's `do { } while (cond);` (C 342-357 and C 760-771)
   becomes `loop { ...; if !(cond) { break; } }`; C's empty-body `while (retained_p[++next]) ;`
   becomes `while { next += 1; *retained_p.offset (next as isize) != 0 } { }` (the `tparam.rs`
   shape, which keeps the whole condition in the loop header).
8. **`unsigned` parameters are `*mut c_uint`** (`old_hash`, `new_hash`, `oldhash`, `newhash`),
   `int *` are `*mut c_int`, and C's `char *retained_p` is `*mut c_char`; all indices go
   through `.offset (i as isize)` because these vectors are indexed from the middle (C's
   `first_insert_cost[-delta]` is `*first_insert_cost.offset (-(delta as isize))`).
9. **`sizeof`**: `sizeof (int)` is `std::mem::size_of::<c_int> ()` (three `xnrealloc` calls in
   `do_line_insertion_deletion_costs`), `sizeof (char)` is `std::mem::size_of::<c_char> ()`, and
   `sizeof lines` is `std::mem::size_of_val (&lines)` (C's array-of-anonymous-struct case).
10. **`scrolling_max_lines_saved`'s anonymous element type** (C 840-841: `struct { unsigned hash;
    int count; } lines[NLINES];`) is named `lines_elt` (the `<variable>_elt` shape P-67 gives an
    anonymous member) and the array is initialized with C's zeroed contents
    (`memset (lines.as_mut_ptr () as *mut c_void, 0, ...)`), because Rust needs a value for the
    binding.  C's two `enum { LOG2_NLINES = 9 };`/`enum { NLINES = 1 << LOG2_NLINES };` are
    `const` items inside the function, as C's enums are function-local.
11. **`clip_to_bounds` takes `isize`** in the mirror, so the two `extra_cost` initializers pass
    `baud_rate / (10 * 4) / (frame_total_lines as isize)` (C's `intmax_t` arithmetic - `baud_rate`
    is globals.h's `intmax_t f_baud_rate`) and `(INT_MAX / 2) as isize`, with the result cast
    back to `c_int` the way C narrows the initializer.  C's `min (cost, cost1)` is
    `min (cost as isize, cost1 as isize) as c_int`.
12. **Chained C statements** are split so each C token keeps its own statement: `i = j =
    window_size;` is `i = window_size; j = window_size;` (twice, C 277 and C 690), and the two
    comma expressions `--i, --j;` / `--i, --j, --n_to_write;` are one statement per decrement.
13. **Line count**: C 993 vs the port's 1175 (18.3%), registered in
    `docs/ports/line-drift-tolerance.tsv` with tolerance 0.20; the drift is the port's fixed
    costs (the `#![allow]` block, the twelve-entry include block with its chain notes, the
    prologue note, the gated `CHECK_BOUNDS` macro) plus the wrapped 8-to-11 parameter signatures
    and the `(*p).<field>` spellings.
