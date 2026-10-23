# emacs/lib/sys_select.in.h (342 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] rpl_fd_isset  (line 153, static)

## Macros defined — 25
- [x] FD_ISSET  (line 169, body b0ee5ef4c8ec)
- [x] GNULIB_defined_rpl_fd_isset  (line 165, body 356a192b7913)
- [x] GNULIB_defined_suseconds_t  (line 145, body 356a192b7913)
- [x] _  (line 108, body c006b296ec00)
- [x] _GL_INCLUDING_WINSOCK2_H  (line 125, body da39a3ee5e6b)
- [x] _GL_SYS_SELECT_H_REDIRECT_FROM_SYS_TIME_H  (line 50, body da39a3ee5e6b)
- [x] _GL_SYS_SELECT_H_REDIRECT_FROM_SYS_TYPES_H  (line 38, body da39a3ee5e6b)
- [x] accept  (line 199, body 23c177e695b8)
- [x] bind  (line 201, body b0f7368fca0d)
- [x] close  (line 179, body f8eeef5e6ed3)
- [x] connect  (line 197, body 793ce4533509)
- [x] gethostname  (line 186, body 6e8ece6ac4fa)
- [x] getpeername  (line 203, body 8d67ebf56174)
- [x] getsockname  (line 205, body 73cbc23b0678)
- [x] getsockopt  (line 207, body 70c568bd4a93)
- [x] listen  (line 209, body 6c83886edf9e)
- [x] pselect  (line 260, body 819c710558d6)
- [x] recv  (line 211, body 322654f6db7f)
- [x] recvfrom  (line 215, body 33ed9871c4fb)
- [x] select  (line 315, body b7a604c673ce)
- [x] send  (line 213, body de503e7a67a6)
- [x] sendto  (line 217, body 3931182241d0)
- [x] setsockopt  (line 219, body b316418cd81c)
- [x] shutdown  (line 221, body 7b25afff2732)
- [x] socket  (line 195, body e3118d6fa129)

## Types and their fields — 0

## File-scope objects — 0

# ----------------------------------------------------------------------
## Decisions

- **The criterion is the generated header, and it exists.**  `build/emacs-ref/lib/sys/select.h`
  (36,552 bytes) is what `gnulib.mk`'s rule writes from this template; the diff that names every
  substitution is

      $ diff <(sed 's/@GUARD_PREFIX@/GL/g' emacs/lib/sys_select.in.h) build/emacs-ref/lib/sys/select.h

  and the arms it decides are in the mirror's header table.  The three that shape the file:
  `@HAVE_SYS_SELECT_H@` 1 (so C 112-144's `!` arm is dead and the real body is live),
  `@HAVE_WINSOCK2_H@` 0 (so the `suseconds_t` typedef, the `rpl_fd_isset` body and the `FD_ISSET`
  override at C 154-174 are dead, as are the eighteen `*_used_without_including_*` renames at
  C 176-254), and `@GNULIB_PSELECT@` 1 with `@REPLACE_PSELECT@` 0 (so `pselect`'s **SYS** arm is
  live and nothing is renamed) - while `@GNULIB_SELECT@` is **0** (generated 813 reads `#if 0`),
  so `select`'s whole block is dead and its name is carried from the dead arms only.
- **Blind read (P-51/P-66)**, measured: `python3 tools/check-cond-eval.py lib/sys_select.in.h` ->
  `40 conditions, 18 unknown, 29 disagreements [NOT VERIFIED]`.  Every `#if` carries both arms.
- **The system half is `sys_select.rs`** (P-26/P-50, no reference file): C 39/51/60/88's
  `#include_next`, plus `sys_types.in.rs` (C 75) and `signal.in.rs`/`signal.rs` (C 103-105, live:
  measured `!((__GLIBC__ || __CYGWIN__ || __KLIBC__) && !__UCLIBC__)` is True).
- **`rpl_fd_isset` is a function, not a macro**, so the mirror carries C's body token for token
  behind `windowsnt` (dead on this target: the generated 661 reads `#if 0`).
- **No `build-tu` consumer**, the same reading the sibling template mirrors give
  (`include-sys_time.in.md`, `include-stdlib.in.md` 258): `.in.h` mirrors take their scalars
  (`c_int`, `SOCKET`, `fd_set`, `timespec`, `sigset_t`) from the including unit, and this target's
  consumers include the system half.

## Evidence

    $ python3 tools/extract-abi.py --check emacs/lib/sys_select.in.h lib/include/sys_select.in.rs
    OK emacs/lib/sys_select.in.h -> lib/include/sys_select.in.rs

    $ python3 tools/check-cond-eval.py lib/sys_select.in.h
    lib/sys_select.in.h: 40 conditions, 18 unknown, 29 disagreements [NOT VERIFIED]

    $ rg -n 'if !1|if 1$|if 0$|elif 1|elif 0' build/emacs-ref/lib/sys/select.h
    33:#if (1 ...            <- @HAVE_SYS_SELECT_H@, first redirect arm
    42:#elif (1 ...           <- second redirect arm
    59:#elif 1 && defined __OpenBSD__ ...
    77:#if 1                  <- the body's include_next is live
    111:#if !1                <- C 110-144 dead
    661:#if 0                 <- C 154-174 (suseconds_t, rpl_fd_isset, FD_ISSET) dead
    696:#if 0                 <- C 176-254 (the 18 renames) dead
    777:#if 1                 <- @GNULIB_PSELECT@
    778:# if 0                <- @REPLACE_PSELECT@
    790:#  if !1               <- @HAVE_PSELECT@
    813:#if 0                 <- @GNULIB_SELECT@ : the select block is dead
    833:#elif 0               <- @HAVE_WINSOCK2_H@ of the select chain
