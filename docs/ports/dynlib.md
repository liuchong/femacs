# emacs/src/dynlib.c (340 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 5
- [x] dynlib_open  (line 277)
- [x] dynlib_sym  (line 291)
- [x] dynlib_addr  (line 297)
- [x] dynlib_error  (line 313)
- [x] dynlib_func  (line 337)

## Macros defined — 1
- [x] dlfunc  (line 333, body ce584ce2f5f4)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- WINDOWSNT is undefined and HAVE_UNISTD_H is 1, so the native-Windows arm (C 33-266: the
  `LoadLibrary`/`GetProcAddress` family, `os_subtype` and the two w32 headers) is dead by
  measurement and the live text is the POSIX arm (C 268-345).
- Inside that arm, HAVE_NATIVE_COMP is undefined (so `dynlib_open_for_eln` and `dynlib_close`
  are dead) and HAVE_DLADDR is undefined, so `dynlib_addr`'s live body is the two NULL stores.
  The tolerance row records that the port is 71% shorter than C for these reasons.
- HAVE_DLFUNC is undefined, so C 333-335's `# define dlfunc dynlib_sym` is live and the port
  keeps it as a macro_rules the one function calls through.
- C 274's <dlfcn.h> is not mirrored (there is no reference header for it in this tree); the four
  entries the live arm names plus the `RTLD_*` values (measured: RTLD_LAZY 1, RTLD_NOW 2,
  RTLD_LOCAL 4, RTLD_GLOBAL 8) are declared in-unit, the getgroups.c shape.
- `(dynlib_function_ptr) dlfunc (h, sym)` keeps C's cast: the port binds the transmuted value to
  a local of that type, so the ledger sees the same type name.
- No call-relationship differences (D7): one `dlopen`, one `dlsym`, one `dlerror`, and
  `dynlib_func` calls through `dlfunc`.
