# emacs/lib/stdckdint.in.h (58 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 4
- [x] _  (line 31, body d8aad3daa9cc)
- [x] ckd_add  (line 52, body b5d0c40b2f80)
- [x] ckd_mul  (line 54, body e7b1fddadf95)
- [x] ckd_sub  (line 53, body b51903b05b4f)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **This target generates no such header, so the criterion is the system `<stdckdint.h>`.**
  gnulib substitutes the template only when `GL_GENERATE_STDCKDINT_H_CONDITION` is non-empty
  (`build/emacs-ref/lib/gnulib.mk` 3325's `ifneq`), and this configuration's value is the empty
  string (3340's `else` branch is the rule that applies: it regenerates nothing and the file is
  absent).  So the header every unit sees is the system one that C 41's
  `# @INCLUDE_NEXT@ @NEXT_STDCKDINT_H@` reaches - the same situation `errno.in.rs` records - and
  every name the template defines itself is dead text (P-51).  The substitutions the template's
  three conditions would take are in the same file: `HAVE_C_STDCKDINT_H = 1`,
  `HAVE_WORKING_C_STDCKDINT_H = 1`, `HAVE_CXX_STDCKDINT_H =` and `HAVE_WORKING_CXX_STDCKDINT_H =`
  empty (761-762), so C 26's test is `#if 1` (the include_next is live) and C 34's is `#if !1`
  (C 36-54, the three definitions, is dead).  The measurements are in Evidence.
- **The three dead definitions are carried with C's own tokens, under one feature.**  C 52-54's
  bodies are `((bool) _GL_INT_*_WRAPV (a, b, r))`; the mirror writes each as a
  `#[cfg (feature = "no_working_ckdint")] macro_rules!`, naming `_GL_INT_ADD_WRAPV` /
  `_GL_INT_SUBTRACT_WRAPV` / `_GL_INT_MULTIPLY_WRAPV` - `lib/intprops-internal.h`'s names, which
  C 41 includes and whose mirror is still `todo` in `docs/port-inventory.md`.  The feature names
  the condition (`# if !@HAVE_WORKING_C_STDCKDINT_H@`) that would take the arm.
  **`bool` is a token the comparison skips** (`MACRO_TOKEN_SKIP`), so the bearing token is the
  gnulib macro's name - which is what the negative control below removes.
- **The frozen arm is a marker, because the port spells a `ckd_*` call with the `overflowing_*`
  pair at the site.**  C 52-54 are the store-and-report shape (`ckd_add (&x, a, b)` writes the
  wrapped sum and returns the overflow flag); the ported units write
  `let (value, overflowed) = … .overflowing_add (…)` plus the store instead (P-15: measured in
  `femacs/lib/timespec-add.rs` 33-38, `femacs/lib/mktime.rs` 36/315/405, `femacs/src/doprnt.rs`
  83), and no ported unit names `ckd_add`/`ckd_sub`/`ckd_mul` (Evidence).  So each frozen arm is
  `pub const ckd_add: () = ();` - the marker shape `lib/include/gnulib_config.rs` uses for a
  name whose counterpart the port spells elsewhere - with the frozen spelling
  (`__builtin_add_overflow ((A), (B), (R))`) in its doc comment.  **If the integrator prefers a
  usable macro**, the body is the `overflowing_*` pair plus the store into `*$r`; nothing else in
  the tree depends on the choice.
- **The consumers are wired** (P-66's include discipline: the unit's C file includes
  `<stdckdint.h>`, so the ported unit includes the mirror): `femacs/lib/timespec-add.rs`
  (C 26) and `femacs/lib/timespec-sub.rs` (C 27), both `built` in Evidence, with their own
  `--check` unchanged.  The remaining includers are the integrator's sweep, the
  `include-dirent.in.md` pattern: `lib/mktime.c` 49, `lib/strftime.c` 111,
  `lib/group-member.c` 24, `lib/stdio-consolesafe.c` 22, `lib/file-has-acl.c` 52,
  `lib/realloc.c` 29, `lib/malloc.c` 27, `lib/stat-time.h` 30 (the `stat-time.rs` mirror) and
  `lib-src/etags.c` 112, `lib-src/emacsclient.c` 79 - each names the three dead definitions and
  nothing else, so the include is a one-line change per file.
- C 18-21's `__GNUC__ >= 3` test with the two pragma placeholders, C 23/30/57-58's guard pair,
  C 25-31's split-guard comment and C 43-51's doc comment have no Rust counterpart
  (PORTING.md 0.1, P-08); C 36-39's three `# undef`s are what C's own definitions replace.

## Evidence

```
$ grep -n 'STDCKDINT' build/emacs-ref/config.status build/emacs-ref/lib/gnulib.mk | head -3
build/emacs-ref/config.status:820:S["GL_GENERATE_STDCKDINT_H_CONDITION"]=""
build/emacs-ref/lib/gnulib.mk:374:GL_GENERATE_STDCKDINT_H_CONDITION =
build/emacs-ref/lib/gnulib.mk:3325:ifneq (,$(GL_GENERATE_STDCKDINT_H_CONDITION))
$ ls build/emacs-ref/lib/stdckdint.h
ls: build/emacs-ref/lib/stdckdint.h: No such file or directory
$ sed -n '761,762p' build/emacs-ref/lib/gnulib.mk
HAVE_CXX_STDCKDINT_H =
HAVE_C_STDCKDINT_H = 1

# the criterion, measured with the configuration's compiler (the system header after config.h)
$ printf '' > /tmp/empty.c
$ clang -std=gnu23 -I build/emacs-ref/src -I build/emacs-ref/lib -include config.h -E -dM \
    -include stdckdint.h /tmp/empty.c | grep -E 'ckd_|STDC_VERSION_STDCKDINT'
#define __STDC_VERSION_STDCKDINT_H__ 202311L
#define ckd_add(R,A,B) __builtin_add_overflow((A), (B), (R))
#define ckd_mul(R,A,B) __builtin_mul_overflow((A), (B), (R))
#define ckd_sub(R,A,B) __builtin_sub_overflow((A), (B), (R))

$ python3 tools/extract-abi.py --check emacs/lib/stdckdint.in.h femacs/lib/include/stdckdint.in.rs
  (1 C macros have a recorded Rust counterpart: _)
OK emacs/lib/stdckdint.in.h -> femacs/lib/include/stdckdint.in.rs

# the negative control: the bearing token is the gnulib macro's name, not `bool` (a skipped token)
$ sed 's/_GL_INT_ADD_WRAPV/_GL_INT_NOPE_WRAPV/' femacs/lib/include/stdckdint.in.rs > /tmp/gt-neg2/n3b.rs
$ python3 tools/extract-abi.py --check emacs/lib/stdckdint.in.h /tmp/gt-neg2/n3b.rs
- macro ckd_add does not mention _gl_int_add_wrapv (C body: ((bool) _GL_INT_ADD_WRAPV (a, b, r)))
FAIL emacs/lib/stdckdint.in.h -> /tmp/gt-neg2/n3b.rs
$ sed 's/(_GL_INT_ADD_WRAPV! ($a, $b, $r)) as bool/(_GL_INT_ADD_WRAPV! ($a, $b, $r))/' \
    femacs/lib/include/stdckdint.in.rs > /tmp/gt-neg2/n3.rs
$ python3 tools/extract-abi.py --check emacs/lib/stdckdint.in.h /tmp/gt-neg2/n3.rs
OK emacs/lib/stdckdint.in.h -> /tmp/gt-neg2/n3.rs      # measured: `bool` is in MACRO_TOKEN_SKIP

# the two consumers build, and their own checks stay green
$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-gt cargo run -q -p xtask -- build-tu lib/timespec-add.c
built lib/timespec-add.rs -> ../build/femacs/timespec-add.o
$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-gt cargo run -q -p xtask -- build-tu lib/timespec-sub.c
built lib/timespec-sub.rs -> ../build/femacs/timespec-sub.o
$ python3 tools/extract-abi.py --check emacs/lib/timespec-add.c femacs/lib/timespec-add.rs
OK emacs/lib/timespec-add.c -> femacs/lib/timespec-add.rs
$ python3 tools/extract-abi.py --check emacs/lib/timespec-sub.c femacs/lib/timespec-sub.rs
OK emacs/lib/timespec-sub.c -> femacs/lib/timespec-sub.rs

# no ported unit names the three macros
$ rg -n "\bckd_(add|sub|mul)\s*\(" femacs/src femacs/lib femacs/lib-src | grep -v 'include/stdckdint.in.rs' | head -3
femacs/lib-src/etags.rs:9032:  /* C's `ckd_mul (&nbytes, nitems, item_size)`: the store happens only when the
femacs/lib/mktime.rs:315:  /* C 399's `ckd_sub (&negative_offset_guess, 0, off)` stores the wrapped difference and
femacs/lib/mktime.rs:405:              /* C 476's `ckd_add (&ot, t, delta * direction)` stores the wrapped sum in
# the three hits are comments quoting C's calls; there is no Rust call site

# the file is a gnulib template, so it stays off the verified list (P-66) and `--check` reads
# every arm as live - the reason the mirror carries C's three definitions as text
$ python3 tools/check-cond-eval.py lib/stdckdint.in.h
lib/stdckdint.in.h: 5 conditions, 4 unknown to the evaluator, 5 disagreements [NOT VERIFIED]
```
