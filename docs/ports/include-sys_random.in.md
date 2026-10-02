# emacs/lib/sys_random.in.h (105 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 4
- [x] GRND_NONBLOCK  (line 58, body 356a192b7913)
- [x] GRND_RANDOM  (line 59, body da4b9237bacc)
- [x] _  (line 46, body 7de35e40fd9b)
- [x] getrandom  (line 77, body cd79c20ea46b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/lib/sys_random.in.h` @ `fd7afced403`; port `femacs/lib/include/sys_random.in.rs`.  The
reference is a gnulib template (`@PRAGMA_SYSTEM_HEADER@`, `@GUARD_PREFIX@`, `@HAVE_SYS_RANDOM_H@`,
`@INCLUDE_NEXT@`), so it cannot join the verified list; the mirror carries the `GRND_NONBLOCK`(1)
/`GRND_RANDOM`(2) fallback (live here: there is no system <sys/random.h>) and the `getrandom`
declaration the units name, and the unchecked boxes in this ledger are the rest of the header.
