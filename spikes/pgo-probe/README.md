# PGO no_std feasibility probe (2026-10-21)

These are the source files and the two `.profraw` files from the minimal probes.  They are kept
in the project because they establish a build fact needed by the PGO plan; they are not part of
the port and are not linked into the program.

- `femacs-pgo-nostd.rs`: `no_std`/`no_main`, ordinary `main` returning 0; linked with
  `-C panic=abort -C linker=clang -Cprofile-generate=<dir> -C link-arg=-lSystem`.
  Result: binary ran and wrote `nostd-main.profraw` (568 bytes).
- `femacs-pgo-exit.rs`: same link, but calls libc `exit(0)` at the end.
  Result: binary ran and wrote `nostd-exit.profraw` (568 bytes).

The `-lSystem` link argument is required: without it the profiler runtime leaves
`___assert_rtn`, `___chkstk_darwin`, `___error` undefined.  With it, the no_std link closes and
`exit()` flushes the profile through the runtime's normal atexit path.  This does not by itself
prove the whole program link works; it only removes the blanket claim that `panic=abort`/`exit`
makes profile output impossible.
