# femacs/lib/include/sys_select.rs (no reference file)

The frozen target's `<sys/select.h>` is a system header (gnulib has no `sys_select.in.h`
wrapper in this build), so there is no reference file to pair with and no checklist to tick.
What the mirror carries, and the measurements behind it, is in the file's own header comment:
`fd_set` (32 × `__int32_t` = 128 bytes, `fds_bits` at 0), `FD_SETSIZE` = 1024,
`__DARWIN_NFDBITS` = 32, the `FD_ISSET` inline, and the `select`/`pselect`/
`__darwin_check_fd_set_overflow` declarations.

## Decisions

- `FD_ISSET` is the header's `__darwin_fd_isset` inline: the bounds check through
  `__darwin_check_fd_set` and then `fds_bits[fd / 32] & (1 << (fd % 32))`.  The port keeps the
  check; the one deviation is that C's null-test on the address of
  `__darwin_check_fd_set_overflow` (a weak-link guard whose `#pragma clang diagnostic` block
  exists for systems that lack the symbol) is not carried, because this target's libSystem
  exports it — a missing symbol here would be a link error, not a silent fallback.
- `_DARWIN_C_SOURCE` is 1 in this build (config.h 2447-2448, macros.tsv 4041), which is why
  the check calls the overflow entry with the third argument 1.
