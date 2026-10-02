# emacs/lib/pipe2.c (166 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] pipe2  (line 39)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- HAVE_PIPE2 and GNULIB_defined_O_NONBLOCK are both undefined here, so C 44-61 (the system-call
  cache), C 66-103 (the `_WIN32` arm) and the `nonblocking.h` include are dead by measurement;
  the live text is the flag check, the Unix `pipe`+`fcntl` path and C's `fail:` cleanup.
- C's three `goto fail` jumps become `break 'fail` out of the labelled block wrapping the live
  body; the code after the block is C's `fail:` label, which is live because this is not a
  native Windows target (the notes row records the missing `goto` token).
- `O_BINARY`/`O_TEXT` come from the fcntl mirror, where gnulib's Unix fallback defines both as
  0; C 146-156's `# if O_BINARY` arm is dead for that reason and its `set_binary_mode` calls are
  not carried (the binary-io mirror still carries the inline functions for the units that ask).
- No call-relationship differences (D7): `pipe`, four `fcntl` calls, two `close`s.
