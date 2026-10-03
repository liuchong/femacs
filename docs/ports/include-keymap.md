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
- `KEYMAPP` keeps C's name and macro shape (PORTING.md 0.2 / P-37): it is not a place
  expression, and a Rust `fn` would evaluate its argument exactly once just as C's macro does,
  so the reason is that C's `KEYMAPP` *is* a macro and the port keeps the name; the body is
  C's `(!NILP (get_keymap (m, false, false)))` (review B's F9 corrected the earlier, wrong
  "a function call cannot promise single evaluation").
- The `map_keymap_function_t` typedef is `Option<unsafe extern "C" fn …>`: the null-pointer
  optimisation makes it `void (*)(…)` at the ABI level (`sizeof` and calls are identical),
  which is what the type needs.  Review B's F4 showed the old note's justification ("both
  callers pass either a function or `0`") was unsupported — no caller passes NULL as `fun`,
  the NULL argument is `data` — so the reason recorded now is the ABI one.  It sits where C
  declares it (keymap.h 42-44, before the two declarations that use it; review B's F8).
- The parameter names in the `extern` block come from the definitions in `keymap.c`, as
  `docs/ports/include-itree.md` records for itree.h; `keymap.h` itself names only
  `map_keymap_canonical`'s four (`map`, `fun`, `args`, `data`), which the port keeps.

### Call-relationship differences (D7)

- None.  The mirror declares exactly the eight functions, one global object and one typedef
  keymap.h declares, in keymap.h's order; `syms_of_keymap` keeps its position.

## Integrator note (2026-10-03, with `port(src/keymap.c)`)

The nine functions and one object this header declares are **defined by `femacs/src/keymap.rs`
itself** (they are `keymap.c`'s non-`static` definitions), so a declaration at the same scope
was a hard `E0428` against the definition.  They moved into `pub mod keymap_decls` + `use
keymap_decls::*;` (PORTING.md 0.1-10/P-21, the `intervals_decls` shape): no name was added,
renamed or dropped, and all eleven consumers of this mirror were rebuilt
(`casefiddle`, `category`, `cmds`, `data`, `doc`, `emacs`, `intervals`, `macros`, `menu`,
`minibuf`, `undo` - each printed `built`).

