# emacs/src/itree.h (190 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] itree_empty_p  (line 120)

## Macros defined — 3
- [x] ITREE_FOREACH  (line 169, body da39a3ee5e6b)
- [x] ITREE_FOREACH_NARROW  (line 185, body 90e4c8d7632d)
- [x] ITREE_H  (line 21, body da39a3ee5e6b)

## Types and their fields — 3
- [x] itree_iterator: struct itree_node* node, ptrdiff_t begin, ptrdiff_t end, uintmax_t otick, enum itree_order order
- [x] itree_node: struct itree_node* parent, struct itree_node* left, struct itree_node* right, ptrdiff_t begin, ptrdiff_t end, ptrdiff_t limit, ptrdiff_t offset, uintmax_t otick, Lisp_Object data, bool_bf red : 1, bool_bf rear_advance : 1, bool_bf front_advance : 1
- [x] itree_tree: struct itree_node* root, uintmax_t otick, intmax_t size

## File-scope objects — 0

## Decisions

`emacs/src/itree.h` @ `fd7afced403`; port `src/include/itree.rs`; the gate is
`python3 tools/extract-abi.py --check emacs/src/itree.h src/include/itree.rs` -> `OK`.

- `#if` branches: none in this header.
- Layout: `itree_node`, `itree_tree` and `itree_iterator` are asserted field by field
  against the C compiler's numbers (`docs/baseline/layout/`, regenerated with
  `spikes/run.sh` + `tools/gen-layout-fields.py`).  The three `bool_bf` bitfields are
  packed explicitly (PORTING.md 1.3) as `bits: u8`: `bool_bf` is one byte on this target
  (`conf_post.h`) and clang's record layout puts the members at 72:0, 72:1, 72:2.  What
  the gate cannot see: `offsetof` may not name a bitfield and `u8`/`u32`/`u64` all keep
  the neighbours and `sizeof` at 80, so the *unit width* is recorded here, not proven
  (P-35).
- The three packed bits are named `ITREE_RED` / `ITREE_REAR_ADVANCE` /
  `ITREE_FRONT_ADVANCE` next to the field, so a reader that C spells as
  `node->rear_advance` reads a mask (`buffer.rs` `OVERLAY_*_ADVANCE_P`); the names are
  the port's, C has only the bitfields (PORTING.md 1.3).  `spikes/write-probe.sh` sets
  the three through the masks on the Rust side and through C's bitfields on the C side
  and compares the raw byte at offset 72: both print 7.
- Deviations: `itree_empty_p`'s dereference sits in an `unsafe` block, with `||`
  short-circuiting exactly as C does; the include guard has no Rust counterpart
  (`pub const ITREE_H: bool = true`); `ITREE_FOREACH` / `ITREE_FOREACH_NARROW` are
  **deferred** (P-33: C declares the iterator at the expansion site and the NARROW macro
  reads it, which Rust macro hygiene forbids), recorded in
  `docs/ports/excluded-macros.txt`.  Their five users (`buffer.c`, `itree.c`,
  `editfns.c`, `textprop.c`, `xdisp.c`, 23 sites) wait for that decision.
- Call-relationship differences (D7): none; the parameter names in the `extern` block
  come from the definitions in `itree.c` (`iter`, `g`, …), because C's prototypes have
  none.

## Decisions (2026-10-03, when itree.c was ported)

- **The `extern` block moved into `itree_externs`.**  itree.c's port defines every function
  itree.h declares (`itree_node_init` … `itree_iterator_next`), and a Rust definition cannot
  share a name with a foreign declaration in the same scope (E0428); the documented shape for
  this is a module plus a glob import (P-21/P-39, the category.h precedent), so the
  declarations are unchanged in name, order and signature but now live in
  `pub mod itree_externs { use super::*; unsafe extern "C" { … } }` with `use
  itree_externs::*;` after it.  The mirror's header comment records the same.

