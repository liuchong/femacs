# emacs/lib/sys_time.in.h (235 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 22
- [x] GNULIB_defined_struct_timeval  (line 92, body 356a192b7913)
- [x] _  (line 49, body a63bd210d25f)
- [x] _GL_INCLUDING_WINSOCK2_H  (line 65, body da39a3ee5e6b)
- [x] accept  (line 168, body 23c177e695b8)
- [x] bind  (line 170, body b0f7368fca0d)
- [x] close  (line 148, body f8eeef5e6ed3)
- [x] connect  (line 166, body 793ce4533509)
- [x] gethostname  (line 155, body 6e8ece6ac4fa)
- [x] getpeername  (line 172, body 8d67ebf56174)
- [x] getsockname  (line 174, body 73cbc23b0678)
- [x] getsockopt  (line 176, body 70c568bd4a93)
- [x] gettimeofday  (line 105, body 608d14415203)
- [x] listen  (line 178, body 6c83886edf9e)
- [x] recv  (line 180, body 322654f6db7f)
- [x] recvfrom  (line 184, body 33ed9871c4fb)
- [x] select  (line 225, body e78cf58530e6)
- [x] send  (line 182, body de503e7a67a6)
- [x] sendto  (line 186, body 3931182241d0)
- [x] setsockopt  (line 188, body b316418cd81c)
- [x] shutdown  (line 190, body 7b25afff2732)
- [x] socket  (line 164, body e3118d6fa129)
- [x] timeval  (line 130, body d358d4e54f02)

## Types and their fields — 1
- [x] timeval: time_t tv_sec, long int tv_usec

## File-scope objects — 0

# ----------------------------------------------------------------------
## Decisions

- **The criterion is the generated header, and it exists.**  `build/emacs-ref/lib/sys/time.h`
  (33,335 bytes) is what `gnulib.mk`'s rule writes from this template; every `@NAME@` has become a
  literal number there, and that is what decides each arm:

      $ diff <(sed 's/@GUARD_PREFIX@/GL/g' emacs/lib/sys_time.in.h) build/emacs-ref/lib/sys/time.h

  The measured table (generated-header line numbers):

  | C | `@NAME@` | generated value | arm selected |
  | --- | --- | --- | --- |
  | 20 | `@GUARD_PREFIX@` | `GL` (21) | guard is `_GL_SYS_TIME_H` |
  | 23 | `@PRAGMA_SYSTEM_HEADER@` | `#pragma GCC system_header` (24) | C 22 `__GNUC__ >= 3` live |
  | 25 | `@PRAGMA_COLUMNS@` | empty (26) | nothing |
  | 40/45 | `@INCLUDE_NEXT@ @NEXT_SYS_TIME_H@` | `# include_next <sys/time.h>` (41, 46) | system half |
  | 44 | `@HAVE_SYS_TIME_H@` | `1` (45) | include_next live, C 56-58 `<time.h>` dead |
  | 64 | `@REPLACE_STRUCT_TIMEVAL@` | `0` (65) | dead with `_MSC_VER` |
  | 64 | `@HAVE_WINSOCK2_H@` | `0` (65) | winsock2 block dead |
  | 80 | `@HAVE_STRUCT_TIMEVAL@` | `1` (627) | C 87-92 own struct **dead** |
  | 101 | `@GNULIB_GETTIMEOFDAY@` | `1` (648) | module in the build |
  | 102 | `@REPLACE_GETTIMEOFDAY@` | `0` (649) | **SYS** arm (C 113-122) live |
  | 113 | `@HAVE_GETTIMEOFDAY@` | `1` (660) | `_GL_FUNCDECL_SYS` not needed |
  | 129 | `@REPLACE_STRUCT_TIMEVAL@` | `0` (676) | C++ namespace typedef dead |

- **Blind read (P-51/P-66)**, measured: `python3 tools/check-cond-eval.py lib/sys_time.in.h` ->
  `31 conditions, 1 unknown, 2 disagreements [NOT VERIFIED]`.  Both arms are carried, the dead one
  behind the feature that names the non-frozen side; the two disagreements are the table's own
  blind spots, recorded in the mirror's header (C 30 `_GL_CONFIG_H_INCLUDED` is config.h's marker;
  C 39's operands are the *system* `<sys/time.h>`'s guards).
- **The system half is `sys_time.rs`** (P-26/P-50: the frozen target's `<sys/time.h>`, no
  reference file).  C 87-92's own `struct timeval` is dead here (measured `#if !1 || 0`), so the
  port's live `struct timeval` with its measured layout is `sys_time.rs`'s, and the copy in the
  mirror stays behind `no_have_struct_timeval`/`replace_struct_timeval`.
- **`build-tu` has no consumer for this mirror, and that is measured, not assumed.**  The `.in.h`
  template mirrors take their scalars from the including unit (PORTING.md 0.1-9); on this target
  the consumers include the *system* half instead (`src/include/systime.rs` 40, `lib/boot-time.rs`
  51, `lib/include/boot-time-aux.rs` 34 - all `include!("sys_time.rs")`).
- The demanded macro `_` (line 49) is the extractor reading `_@GUARD_PREFIX@_SYS_TIME_H`: it cuts
  the template token at `@` (P-66's documented blindness).  The guard text is what satisfies it.

## Evidence

    $ python3 tools/extract-abi.py --check emacs/lib/sys_time.in.h lib/include/sys_time.in.rs
    OK emacs/lib/sys_time.in.h -> lib/include/sys_time.in.rs

    $ python3 tools/check-cond-eval.py lib/sys_time.in.h
    lib/sys_time.in.h: 31 conditions, 1 unknown, 2 disagreements [NOT VERIFIED]

    $ diff <(sed 's/@GUARD_PREFIX@/GL/g' emacs/lib/sys_time.in.h) build/emacs-ref/lib/sys/time.h | wc -l
    (the diff is the inlined c++defs.h / arg-nonnull.h / warn-on-use.h text plus the @NAME@ lines)

    $ rg -n 'define timeval rpl_timeval|define GNULIB_defined_struct_timeval|Hide some function declarations' build/emacs-ref/lib/sys/time.h
    630:#  define timeval rpl_timeval          <- inside `# if 0` (629): dead
    639:#  define GNULIB_defined_struct_timeval 1
    689:/* Hide some function declarations from <winsock2.h>.  */   <- `#if defined _MSC_VER && 0` (691)

    $ awk 'NR>=627 && NR<=660 {print NR": "$0}' build/emacs-ref/lib/sys/time.h
    627:#if !1 || 0                        <- C 80-95 dead
    648:#if 1                              <- @GNULIB_GETTIMEOFDAY@
    649:# if 0                             <- @REPLACE_GETTIMEOFDAY@
    660:#  if !1                            <- @HAVE_GETTIMEOFDAY@

    $ cd femacs && cargo run -q -p xtask -- build-tu lib/sys_time.in.h
    error[E0425]: cannot find type `timeval` / `c_void` / `c_int` in this scope   (3 errors)
    -> no consumer compiles this mirror standalone; the sibling template mirror has the same
       reading: `build-tu lib/stdlib.in.h` -> E0425 `c_int` / `c_char`

    $ python3 tools/extract-abi.py --check emacs/lib/stdlib.in.h lib/include/stdlib.in.rs
    OK (the sibling template mirror's own criterion, for comparison)
