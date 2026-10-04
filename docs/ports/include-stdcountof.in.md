# emacs/lib/stdcountof.in.h (124 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 4
- [x] GNULIB_defined_struct__gl_verify_type  (line 67, body 356a192b7913)
- [x] _  (line 31, body 463071bfc2c3)
- [x] _gl_verify_is_array  (line 118, body b6589fc6ab0d)
- [x] countof  (line 55, body 74a24674c32c)

## Types and their fields — 1
- [x] _gl_verify_type: unsigned int _gl_verify_error_if_negative : w

## File-scope objects — 0

# ----------------------------------------------------------------------
## Decisions

- **This template is not turned into a header on this target, and the criterion is the frozen
  configuration's own substitution table.**  There is no `build/emacs-ref/lib/stdcountof.h`
  (`ls` -> no such file) because gnulib generates it only when its condition is non-empty:

      $ grep -n 'STDCOUNTOF' build/emacs-ref/config.status
      814:S["GL_GENERATE_STDCOUNTOF_H_CONDITION"]=""
      815:S["STDCOUNTOF_H"]=""
      816:S["CXX_HAVE_STDCOUNTOF_H"]="1"
      817:S["HAVE_STDCOUNTOF_H"]="1"
      818:S["NEXT_AS_FIRST_DIRECTIVE_STDCOUNTOF_H"]="<stdcountof.h>"
      819:S["NEXT_STDCOUNTOF_H"]="<stdcountof.h>"

  With both `HAVE_STDCOUNTOF_H` and `CXX_HAVE_STDCOUNTOF_H` equal to `1`, C 26's
  `#if (defined __cplusplus ? @CXX_HAVE_STDCOUNTOF_H@ : @HAVE_STDCOUNTOF_H@)` is true on either
  side of the language test, so **the live text is C 27's `# @INCLUDE_NEXT@ @NEXT_STDCOUNTOF_H@`**
  and the whole body (C 30-122) is dead.  Every item the gate asks for therefore comes from a dead
  arm, which is why they are carried behind `no_have_stdcountof_h` / `cplusplus` /
  `gnu_c_without_clang` (the third dead-arm-carrying shape in this batch, after 3.207's
  `struct timeval` and 3.209's `struct sigaction`).
- **This closes status 3.10's deferral, and the reason it can be closed is that the deferral's
  premise was too strong.**  3.10 ruled "the template's conditions cannot be judged (P-66)" on the
  reading `10 conditions, 4 unknown, 5 disagreements [NOT VERIFIED]`; re-measured at this HEAD it
  is `10 conditions, 4 unknown, 4 disagreements`, and all four unknowns are the *placeholders* -
  a `@NAME@` is not a condition the evaluator can know, and the generated header it would
  otherwise have read does not exist.  The placeholders' values, however, are in the frozen
  configuration's substitution table (above), so the unit is decidable: measured substitution for
  the template, blind read for the two arms the table still cannot judge (C 112's
  `_GL_GNUC_PREREQ (3, 1) && ! defined __clang__` and its siblings - measured, the frozen clang
  takes the `#else`).
- **What the port could not transcribe**: C 63-66's `template <int w>` and its negative-bitfield
  error trick.  The type exists in the C++ arm only (dead here) and the port carries the member C
  declares, recording the template parameter as the reason it is not spelled - the P-08 reading of
  a dead arm's fact.
- One body-note row was needed: `countof`'s `__va_args` (C's variadic token, written as the Rust
  metavariable), in `docs/ports/macro-body-notes.tsv`.

## Evidence

    $ python3 tools/extract-abi.py --check emacs/lib/stdcountof.in.h femacs/lib/include/stdcountof.in.rs
    OK emacs/lib/stdcountof.in.h -> femacs/lib/include/stdcountof.in.rs

    $ python3 tools/check-cond-eval.py lib/stdcountof.in.h
    lib/stdcountof.in.h: 10 conditions, 4 unknown, 4 disagreements [NOT VERIFIED]

    $ ls build/emacs-ref/lib/stdcountof.h
    ls: build/emacs-ref/lib/stdcountof.h: No such file or directory

    $ cd femacs && cargo run -q -p xtask -- build-tu lib/stdcountof.in.h
    built lib/include/stdcountof.in.rs -> ../build/femacs/include_stdcountof_in.o
