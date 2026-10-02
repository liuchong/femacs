# emacs/lib/gettime.c (51 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 2
- [x] gettime  (line 29)
- [x] current_timespec  (line 46)

## Macros defined — 0

## Types and their fields — 1
- [x] timespec: struct timespec ts

## File-scope objects — 0


## Decisions

- `CLOCK_REALTIME` is defined by this target's <time.h> and HAVE_CLOCK_GETTIME is 1, so C 30-32
  is the live arm (verified list) and the `timespec_get`/`gettimeofday` arms are dead; the port
  declares the two system entries that arm needs in-unit, as getgroups.c declares `malloc`/
  `free`: `CLOCK_REALTIME` (0, measured with clang) and `clock_gettime` (`clockid_t` is a 32-bit
  `int` on this target).
- The timespec mirror's seven declarations moved into a whole-import module, because this unit
  defines `gettime`/`current_timespec` and C's header declares both (PORTING.md 0.1-10, the
  lisp.rs::other_units shape).
- C's `struct timespec ts;` in `current_timespec` is written with `std::mem::zeroed ()`.
- No call-relationship differences (D7): one `clock_gettime` call and the `gettime` call.
