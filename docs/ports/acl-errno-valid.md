# emacs/lib/acl-errno-valid.c (52 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] acl_errno_valid  (line 30)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- Blind file; the frozen configuration defines `__APPLE__`/`__MACH__` and gives `ENOTSUP` (45),
  `EOPNOTSUPP` (102) and `ENOSYS` (78) three distinct values, so **every** `case` in C's
  `switch` is live on this target — nothing is dead text here.
- C's `switch (errnum)` / `case: return false;` / `default: return true;` is the port's
  `match` with the codes as pattern arms and `_ =>` (PORTING.md 2.2).  The `switch`/`case`/
  `default` tokens are the docs/ports/macro-body-notes.tsv row for `acl_errno_valid`.
- `acl_errno_valid` is a non-`static` definition → `#[unsafe(no_mangle)]` (P-52).  The
  reference object confirms the name (`nm -g build/emacs-ref/lib/acl-errno-valid.o` prints
  `T _acl_errno_valid`), and its declaration lives in the acl.h mirror's `pub mod acl_decls`,
  which this unit's definition shadows (PORTING.md 0.1-10).
- **Mirror edit**: `femacs/lib/include/errno.rs` gained `EBUSY` (= 16, measured; the switch's
  first label had no entry there yet).
- No line-count tolerance is needed (C 52 vs the port's 53, 1.9%).
- Call-relationship differences (D7): none.
