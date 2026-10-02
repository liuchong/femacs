# emacs/src/systime.h (94 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 2
- [x] invalid_timespec  (line 59)
- [x] timespec_valid_p  (line 68)

## Macros defined — 2
- [x] EMACS_SYSTIME_H  (line 20, body da39a3ee5e6b)
- [x] __TIMEVAL__  (line 41, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

# ----------------------------------------------------------------------
## Notes

### Deviations (recorded, not missing work)

- The include guard has no Rust counterpart (PORTING.md 0.1): `pub const EMACS_SYSTIME_H: bool = true`.
- `Time` (systime.h 34-41) keeps all three branches: the frozen configuration takes `#else`
  (`typedef unsigned long Time`, HAVE_X_WINDOWS and HAIKU are both 0, P-08), and the two dead ones
  keep C's text under `feature = "x_windows"` / `feature = "haiku"` (P-51), each bound to its frozen
  constant by a `const _` assertion.  `<X11/X.h>` (which *defines* `Time` as `unsigned long` on every
  supported X target) and `<support/SupportDefs.h>` have no mirror, so the dead branches state the
  type C ends up with instead of including a header the port cannot mirror.
- The HPUX block (systime.h 43-51) is dead text for this target: `__TIMEVAL__` is declared under
  `feature = "hpux"`, and the inner `#ifdef _STRUCT_TIMEVAL` is a *system* macro the port does not
  model (recorded here rather than as a second feature, because the frozen target's `<sys/time.h>`
  does not define it).  `#undef hz` (AIX's `<sys/param.h>` defines it) has no counterpart: the port
  never included that header, so there is nothing to undefine.
- `INLINE_HEADER_BEGIN`/`INLINE_HEADER_END` have no Rust counterpart (PORTING.md 0.1); they come from
  the generated config header in C, not from this file.
- The mirror needs `time_t`/`struct timespec`, which in C arrive through `<timespec.h>` →
  `<time.h>`; the port gets them because this mirror includes `timespec.rs`, which includes
  `time.in.rs`.  `buffer.rs` reaches `time.in.rs` directly as well, so a unit that includes both
  (every `data.c`-shaped unit) hits the same file twice — `xtask`'s include-graph expansion
  (PORTING.md 0.1-4/P-61) makes that one copy, verified by a negative test: with the expansion
  short-circuited, `layout-check` fails with `E0428: the name time_t is defined multiple times`.
- `<timespec.h>` is `femacs/lib/include/timespec.rs` (a *reference* mirror: gnulib's module) and
  `<sys/time.h>` is `femacs/lib/include/sys_time.rs` (a system mirror, P-26/P-50, carrying
  `struct timeval` with the frozen target's layout pinned by `const _` assertions).
- Parameter names in the `extern` block come from the definitions (`keyboard.c`'s
  `set_waiting_for_input (struct timespec *time_to_clear)` and `timefns.c`'s seven); C's prototypes
  have none.

### Call-relationship differences (D7)

- None.  Two functions, one constant, ten declarations, same order.
