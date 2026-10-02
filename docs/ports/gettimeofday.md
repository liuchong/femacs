# emacs/lib/gettimeofday.c (154 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 2
- [x] initialize  (line 50, static)
- [x] gettimeofday  (line 81)

## Macros defined — 4
- [x] GetProcAddress  (line 41, body f09ee83f58e4)
- [x] GetSystemTimePreciseAsFileTimeFunc  (line 63, body 267f1f6dfdf5)
- [x] LoadLibrary  (line 36, body 2ff89c407367)
- [x] WINDOWS_NATIVE  (line 28, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 2
- [x] static GetSystemTimePreciseAsFileTimeFuncType GetSystemTimePreciseAsFileTimeFunc  (line 46, static)
- [x] static BOOL initialized  (line 47, static)

## Decisions

- **The arm that is live here is five lines.**  The frozen configuration defines
  HAVE_GETTIMEOFDAY (1) and does not define `timeval`, so C's `# if HAVE_GETTIMEOFDAY` arm and,
  inside it, the `# else` of `# if defined timeval` are the live text: call the system entry and
  return its result.  Everything else in the file belongs to arms this target does not take.
- **The native-Windows arm (C 28-71) is kept as gated text** under `#[cfg (feature =
  "windows_native")]`: the marker macro, the two statics, `initialize` and the three rename
  macros (`LoadLibrary`, `GetProcAddress`, `GetSystemTimePreciseAsFileTimeFunc`, which are rows
  in docs/ports/excluded-macros.txt).  Its body needs <windows.h>'s FILETIME/HMODULE/BOOL and its
  own function-pointer typedef, which this target has not got, so the items cannot compile even
  if the feature were enabled; the tokens of the FILETIME arithmetic inside `gettimeofday` are in
  docs/ports/macro-body-notes.tsv.
- **C 118's `# error` arm** (the 1-second fallback, taken only when the system has no
  gettimeofday) is dead here for the same reason and needs no text.
- **The identifier is renamed by gnulib's header.**  sys_time.in.h 104-105 turns `gettimeofday`
  into `rpl_gettimeofday` when gnulib replaces the function (REPLACE_GETTIMEOFDAY is a configure
  variable the table cannot decide), so the port keeps C's name for the ledger and emits the
  replacement symbol with `#[unsafe(export_name = "rpl_gettimeofday")]` (PORTING.md 10.13, the
  mempcpy precedent).  C 84's `#undef gettimeofday` restores the *system* declaration for the
  call inside the wrapper, which the port spells with that entry in its own `system` module (the
  category.rs shape for a name this unit defines).
- **The line-count drift (C 154 vs the port's ~89, 42%) is declared** in
  docs/ports/line-drift-tolerance.tsv with this file's reason.
- **Call-relationship differences (D7)**: none beyond the system entry above.
