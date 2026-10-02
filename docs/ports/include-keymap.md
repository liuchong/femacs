# emacs/src/keymap.h (51 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 3
- [x] KEYMAPP  (line 34, body 91cf21cfc9dd)
- [x] KEYMAP_H  (line 20, body da39a3ee5e6b)
- [x] KEY_DESCRIPTION_SIZE  (line 32, body c47a46fd61e8)

## Types and their fields — 0

## File-scope objects — 1
- [x] extern Lisp_Object current_global_map  (line 35)

# ----------------------------------------------------------------------
## Notes

### Deviations (recorded, not missing work)

- The include guard has no Rust counterpart (PORTING.md 0.1): `pub const KEYMAP_H: bool = true`.
- `KEY_DESCRIPTION_SIZE` is a `const` rather than a macro, so `CHARACTERBITS` (lisp.h 3233, in
  `lisp.rs`) is read where C reads it — C's own keymap.h gets it from the same `lisp.h`.  The
  value is 22 either way: `(2 * 6) + 1 + (22 / 3) + 1 + 1` with C's integer division.
- `KEYMAPP` keeps the macro shape (P-37): C's `(!NILP (get_keymap (m, false, false)))` is not a
  place, but it is a macro whose argument must be evaluated exactly once and only when the
  caller uses the result, which a function call cannot promise for a Lisp_Object side effect
  (`get_keymap` can signal).  The port's `KEYMAPP! (m)` expands to the same call.
- The `map_keymap_function_t` typedef: C's function pointer is NULL-able (both `map_keymap`
  callers pass either a function or `0`), so the port spells it `Option<unsafe extern "C" fn …>`
  (PORTING.md 5.2); `sizeof` is unchanged, which is what an ABI needs.
- The parameter names in the `extern` block come from the definitions in `keymap.c` (C's
  prototypes have none), as `docs/ports/include-itree.md` records for itree.h.

### Call-relationship differences (D7)

- None.  The mirror declares exactly the eight functions, one global object and one typedef
  keymap.h declares, in keymap.h's order; `syms_of_keymap` keeps its position.
