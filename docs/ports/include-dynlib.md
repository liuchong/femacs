# emacs/src/dynlib.h (41 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] DYNLIB_H  (line 21, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/src/dynlib.h` @ `fd7afced403`; port `src/include/dynlib.rs`.  The header is the
include guard, two typedefs (`dynlib_handle_ptr` = `void *`, `dynlib_function_ptr` = a pointer to
`void (void)`, which the port spells `Option<unsafe extern "C" fn ()>` - the mirror's function
pointer convention) and the prototypes, in a whole-import module so the unit's definitions
shadow them.  C 20's `<attribute.h>` include is only there for the two `ATTRIBUTE_MAY_ALIAS`
spellings, which have no counterpart (PORTING.md 0.3).
