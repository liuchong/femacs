# emacs/lib/open.c (240 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 3
- [x] orig_open  (line 31, static)
- [x] lstatif  (line 65, static)
- [x] open  (line 71)

## Macros defined — 4
- [x] HAVE_WORKING_O_DIRECTORY  (line 53, body 7cb6efb98ba5)
- [x] OPEN_TRAILING_SLASH_BUG  (line 57, body 7cb6efb98ba5)
- [x] REPLACE_OPEN_DIRECTORY  (line 61, body 7cb6efb98ba5)
- [x] __need_system_fcntl_h  (line 22, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0


## Decisions

- `lib/open.c` is on the verified list, so only the live text is required: HAVE_WORKING_O_DIRECTORY
  is undefined and OPEN_TRAILING_SLASH_BUG/REPLACE_OPEN_DIRECTORY are false, which makes C 52-63's
  three `#ifndef` fallbacks live (the port keeps them as constants) and drops the Windows arms, the
  `#if GNULIB_defined_O_CLOEXEC` arm and both `#if REPLACE_FCHDIR` blocks (the port is 30% shorter
  for that reason; the tolerance row records it).
- C 22's `#define __need_system_fcntl_h` before <config.h> makes the first <fcntl.h> the system
  one, so `orig_open` calls the *system* `open`; the port keeps that entry in its own `system`
  module.  rustc checks a declaration of the runtime `open` symbol against its own expectation
  (`unsafe extern "C" fn(*const i8, i32, ...) -> i32`), so the declaration spells C's variadic
  prototype exactly and the call promotes `mode_t` to `c_uint` the way C promotes it to `int`.
- The definition is C's variadic `open (const char *, int, ...)`: Rust's `mut ap: ...` is the
  list, C's `va_list arg`/`va_start`/`va_end` disappear (P-04; the notes row records the tokens)
  and `va_arg (arg, PROMOTED_MODE_T)` is `next_arg::<c_int> ()` (PROMOTED_MODE_T is `int` on
  this target; the va-arg-types row records the read).
- `lstatif` keeps C's `flags & O_NOFOLLOW ? lstat : stat` shape; `close` comes from the unistd
  mirror and `set_cloexec_flag` from the cloexec mirror.
- No call-relationship differences (D7).
