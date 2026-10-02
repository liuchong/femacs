# emacs/src/itree.c (1435 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 45
- [x] itree_stack_create  (line 145, static)
- [x] itree_stack_destroy  (line 155, static)
- [x] itree_stack_ensure_space  (line 165, static)
- [x] itree_stack_push  (line 178, static)
- [x] itree_stack_pop  (line 188, static)
- [x] itree_max_height  (line 199, static)
- [x] check_subtree  (line 217, static)
- [x] check_tree  (line 286, static)
- [x] null_safe_is_red  (line 313, static)
- [x] null_safe_is_black  (line 319, static)
- [x] itree_newlimit  (line 325, static)
- [x] itree_update_limit  (line 340, static)
- [x] itree_inherit_offset  (line 355, static)
- [x] itree_propagate_limit  (line 392, static)
- [x] itree_validate  (line 413, static)
- [x] itree_node_init  (line 432)
- [x] itree_node_begin  (line 449)
- [x] itree_node_end  (line 459)
- [x] itree_create  (line 469)
- [x] itree_clear  (line 479)
- [x] itree_init  (line 490, static)
- [x] itree_destroy  (line 498)
- [x] itree_size  (line 507)
- [x] itree_rotate_left  (line 515, static)
- [x] itree_rotate_right  (line 558, static)
- [x] itree_insert_fix  (line 597, static)
- [x] itree_insert_node  (line 676, static)
- [x] itree_insert  (line 732)
- [x] itree_node_set_region  (line 744)
- [x] itree_contains  (line 767, static)
- [x] itree_limit_is_stable  (line 783, static)
- [x] itree_subtree_min  (line 792, static)
- [x] itree_remove_fix  (line 807, static)
- [x] itree_total_offset  (line 903, static)
- [x] itree_replace_child  (line 923, static)
- [x] itree_transplant  (line 949, static)
- [x] itree_remove  (line 966)
- [x] itree_insert_gap  (line 1055)
- [x] itree_delete_gap  (line 1148)
- [x] itree_node_intersects  (line 1209, static)
- [x] itree_iter_next_in_subtree  (line 1229, static)
- [x] itree_iterator_first_node  (line 1345, static)
- [x] itree_iterator_start  (line 1390)
- [x] itree_iterator_next  (line 1410)
- [x] itree_iterator_narrow  (line 1427)

## Macros defined — 0

## Types and their fields — 2
- [x] check_subtree_result: int size, ptrdiff_t limit, int black_height
- [x] itree_stack: struct itree_node** nodes, size_t size, size_t length

## File-scope objects — 0

## Decisions

- **`#ifdef ITREE_TESTING` (C 486-494) is dead text.**  ITREE_TESTING is not one of the
  4956 configuration macros and only the itree test program defines it, so C compiles
  `itree_init` out of this translation unit; the port keeps the body as written, with the
  reason in a comment at the definition (the rawmemchr.c CHERI-arm class), and the static
  function it defines is not a symbol either way.
- **`ITREE_FOREACH` is expanded at its use site (P-33, decided here).**  itree.h 166-176
  declares `struct itree_iterator itree_local_iter_` inside the macro and the body reads the
  loop variable `n`, which Rust macro hygiene cannot hand out; itree_insert_gap (C 1073) is
  the one use in this file, and the port writes the expansion C stands for: the `if (!t) { }
  else` guard, the iterator, `itree_iterator_start`, and C's
  `for (…; ((n = itree_iterator_next (itree_iter_)));)` loop.  `ITREE_FOREACH_NARROW` is the
  direct `itree_iterator_narrow` call (no user in this file); both rows in
  docs/ports/excluded-macros.txt now say so, and four further users (buffer.c, editfns.c,
  textprop.c, xdisp.c) inherit the shape.
- **The three `bool_bf` bits are the mirror's `bits` byte.**  itree.h 51-53 declares `red`,
  `rear_advance` and `front_advance` as one-bit fields; the mirror packs them into
  `bits: u8` with ITREE_RED / ITREE_REAR_ADVANCE / ITREE_FRONT_ADVANCE (itree.rs 96-104), so
  every read is `bits & ITREE_RED` and every write is `bits |= ITREE_RED` /
  `bits &= !ITREE_RED`.  The three token-aliases families (`red`, `front_advance`,
  `rear_advance`) keep the C and Rust spellings comparable and their occurrence check keeps
  every access alive; the bool literals of those writes are in
  docs/ports/macro-body-notes.tsv, per function.
- **The iterator's two `switch (iter->order)` blocks are `match`es** with C's arms in C's
  order, `default:` becoming `_ =>` and C's `break;` statements having no counterpart
  (notes rows for itree_iter_next_in_subtree and itree_iterator_first_node).
- **Conditions with an assignment or a comma are written as loops with an explicit test.**
  C's `while ((next = node->parent) && …)` (C 1244, 1270) and `while ((itree_inherit_offset
  (otick, node), node->left != NULL))` (C 796) cannot be Rust conditions; the port keeps the
  same evaluation order (assign/inherit first, then test, then step) with `loop { }` and a
  `break`, and `while (1)` (C 399) is a `loop` (notes row).
- **`itree_max_height` computes its logarithm in double**, as C does: `2 * log (size + 1) /
  log (2) + 0.5`, truncated by the `int` return, which the port spells with `c_double`
  conversions and `as c_int`.
- **The header mirror moved its `extern` declarations into `itree_externs`** (P-21/P-39):
  this unit defines every one of them, so they must be shadowable imports rather than
  colliding items.  Recorded in docs/ports/include-itree.md; the mirror's declarations are
  unchanged in name, order and signature.
- **The line-count drift (C 1435 vs the port's 1534, 6.9%) is declared** in
  docs/ports/line-drift-tolerance.tsv with this file's reason (bitfield masks, `match` arms,
  loop-with-test rewrites); the per-function token check above is unaffected.
- **Call-relationship differences (D7)**: none.  The unit calls only its own functions and
  `xmalloc`/`xrealloc`/`xfree`/`log`/`emacs_abort`.
