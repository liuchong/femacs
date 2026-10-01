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

`emacs/src/itree.h` @ `fd7afced403`; port `femacs/src/include/itree.rs`; the gate is
`python3 tools/extract-abi.py --check emacs/src/itree.h femacs/src/include/itree.rs` -> `OK`.

- `#if` branches: none in this header.
- Layout: `itree_node` (three bitfields), `itree_tree` and `itree_iterator` are asserted
  against the C compiler's numbers (`docs/baseline/layout/`, regenerated with
  `spikes/run.sh`; `tools/gen-layout-fields.py` emits one offset per field).  The three
  `bool_bf` bitfields are packed explicitly as `bits: u32` (PORTING.md 1.3); the
  neighbouring field offsets and `sizeof itree_node` prove the packing.
- Deviations: `itree_empty_p`'s dereference sits in an `unsafe` block, with `||`
  short-circuiting exactly as C does; the include guard has no Rust counterpart
  (`pub const ITREE_H: bool = true`); `ITREE_FOREACH` / `ITREE_FOREACH_NARROW` are
  **deferred** (P-33: C declares the iterator at the expansion site and the NARROW macro
  reads it, which Rust macro hygiene forbids), recorded in
  `docs/ports/excluded-macros.txt`.  Their five users (`buffer.c`, `itree.c`,
  `editfns.c`, `textprop.c`, `xdisp.c`, 23 sites) wait for that decision.
- Call-relationship differences (D7): none; the parameter names in the `extern` block
  come from the definitions in `itree.c`, because C's prototypes have none.
