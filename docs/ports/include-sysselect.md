# emacs/src/sysselect.h (98 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 3
- [x] fd_CLR  (line 67)
- [x] fd_ISSET  (line 74)
- [x] fd_SET  (line 81)

## Macros defined — 4
- [x] FD_CLR  (line 90, body 0d9b50096b22)
- [x] FD_ISSET  (line 91, body 470a04083cbb)
- [x] FD_SET  (line 92, body 034ea60d3e01)
- [x] SYSSELECT_H  (line 20, body 356a192b7913)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

Mirror of src/sysselect.h, complete for the frozen target (`HAVE_SELECT` is 1; `DOS_NT`,
`WINDOWSNT`, `MSDOS` are all undefined, macros.tsv).

1. **Live text.**  C 24's `#include <sys/select.h>` becomes
   `include!("../../lib/include/sys_select.rs")` (the port's system mirror).  C 33-44's
   `# define FD_SETSIZE 32` / `typedef int fd_set` fallback is dead because the system header
   supplies `FD_SET` - the extractor's live text takes the `#ifdef FD_SET` arm, which is why
   only the guard, the three `INLINE` functions and the four macros are live.
2. **The three functions and their macros are the header's real content.**  C 62-93 (the
   `#ifndef WINDOWSNT` block) defines `fd_CLR`/`fd_ISSET`/`fd_SET` with an `eassume` bounds check
   and then `#undef`s the three system macros and re-points them at those functions.  The mirror
   keeps both halves: `#[inline(always)] pub unsafe fn fd_*` calling the system mirror's
   accessors, and `macro_rules! FD_CLR`/`FD_ISSET`/`FD_SET` expanding to them - so a caller that
   writes `FD_ISSET (n, p)` after this header gets the checked version, exactly as in C (macro
   expansion precedes name resolution, so the macro shadows the mirror's function).
3. **Dead arms.**  `#if !defined (HAVE_SELECT)` (C 46-48, `#define select sys_select`) and
   `#ifdef MSDOS`'s `#define pselect sys_select` (C 50-58) are recorded here, not carried.
4. **The guard has a value.**  C writes `#define SYSSELECT_H 1`, so the constant carries C's `1`
   (`pub const SYSSELECT_H: c_int = 1`), unlike the empty `#define X_H` guards that are
   `bool = true`.

**Differential probe** (this is the evidence for items 1-2 and for the system mirror's three new
accessors; the recipe is reproducible):

    C side:      clang -o /tmp/fdprobe/c /tmp/fdprobe/probe.c && /tmp/fdprobe/c > c.out
    Rust side:   /tmp/fdprobe/mk.py materializes config.rs + lisp.rs + sysselect.rs (dedup by
                 file, the xtask materialize rule) and rustc compiles it; run > rs.out
    sequence:    FD_ZERO; FD_SET {0,1,31,32,63,64,127,1023}; dump; FD_CLR 32/0; dump;
                 FD_ZERO; FD_SET 1023/7; dump; then the same three dumps through the
                 bounds-checked fd_* wrappers; finally the 128 raw bytes of the fd_set.

Result: the three dumps from the system-mirror accessors are byte-identical to C's, the three
dumps from the checked wrappers are identical to C's too, and the final `fd_set` is identical
(`sha256 0c83234699c0c656027f540d32c31a9ca7fe90b40e59e78baa24aabffdbcd656`).

**Deviations found by that probe (recorded, not papered over).**

- Darwin's `__darwin_check_fd_set_overflow` does **not** bound the fd by `FD_SETSIZE`: measured
  `check(1023)=1 check(1024)=1 check(1100)=1`, and C's `FD_ISSET (1024, &f)` therefore reads
  `fds_bits[32]` - one element past the 32-element array, i.e. **undefined behaviour** that
  happened to read 0 on the stack.  The Rust mirror bounds-checks the slice and panics instead.
  This is a language-level deviation in UB territory: every caller in the tree stays in range
  (`sysselect.h`'s three wrappers state the contract with `eassume`), and docs/status.md 3.92
  records it.
- `FD_ISSET` in the system mirror returns `bool` (C's returns `int`); the checked `fd_ISSET`
  wraps it as `FD_ISSET (fd, set) as c_int != 0` to keep C's `!= 0` token and semantics.
