# emacs/lib/getrandom.c (193 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] getrandom  (line 115)

## Macros defined — 2
- [x] NAME_OF_NONCE_DEVICE  (line 106, body 36c73e2c3772)
- [x] NAME_OF_RANDOM_DEVICE  (line 99, body a67d1b0f0104)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- This is not a native Windows target and HAVE_GETRANDOM is undefined, so the `_WIN32` setup,
  the Windows arm and the `#elif HAVE_GETRANDOM` arm are dead; the live text is C 99-108's two
  device names and C 181-192's /dev/urandom reader.
- The two names are byte-string constants with C's spelling (the checker compares a string
  constant by its bytes), and the reader's table is a local pointer binding of their addresses:
  the C's `char const randdevice[][MAX (sizeof …, sizeof …)]` holds the same read-only data, and
  the row length the C needs is recorded in the notes row instead (the port's table needs none).
- `GRND_NONBLOCK`(1)/`GRND_RANDOM`(2) come from the new sys_random mirror (gnulib's fallback:
  this target has no <sys/random.h>).
- C 116's `#undef getrandom` is the port's `system` module (the getgroups.c shape), so the
  replacement keeps C's name and the dead arm's call has a declaration.
- No call-relationship differences (D7): `open` once, `read` once.
