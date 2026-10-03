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

## 2026-10-04: the three missing accessors

`<sys/select.h>` defines four accessors, and this mirror carried only `FD_ISSET`; `src/sysselect.h`
needs all four (`FD_CLR`/`FD_SET` in its `INLINE` functions, `FD_ZERO` for C's callers), so

- `FD_SET` and `FD_CLR` are now here, mirroring the SDK's `__darwin_fd_set`/`__darwin_fd_clr`
  (`__header_always_inline`, `sys/_types/_fd_def.h` 84-104): the same
  `__darwin_check_fd_set` guard as `FD_ISSET`, then
  `fds_bits[fd / __DARWIN_NFDBITS] |= / &= ~(1 << (fd % __DARWIN_NFDBITS))`;
- `FD_ZERO` mirrors `#define FD_ZERO(p) __DARWIN_FD_ZERO (p)` → `__builtin_bzero (p, sizeof *(p))`
  as `std::ptr::write_bytes (p, 0, 1)`.

Evidence: the differential probe in `docs/ports/include-sysselect.md` runs one deterministic
sequence through C's macros and through these functions on the same `fd_set` and compares both
the per-fd dumps and the final 128 bytes (`sha256 0c83234699c0c656027f540d32c31a9ca7fe90b40e59e78baa24aabffdbcd656`),
and separately measures the guard: `__darwin_check_fd_set_overflow (1024, &f, 1)` returns 1,
i.e. C's accessors are *not* bounded by `FD_SETSIZE` and read out of bounds for fd >= 1024
(UB; the port panics).  That measurement is what the "guard" comment above describes - the check
guards the *object* size, not the descriptor range.
