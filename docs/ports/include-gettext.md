# emacs/lib/gettext.h (542 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 18
- [x] gettext  (line 127)
- [x] dgettext  (line 141)
- [x] dcgettext  (line 156)
- [x] ngettext  (line 172)
- [x] dngettext  (line 186)
- [x] dcngettext  (line 201)
- [x] textdomain  (line 217)
- [x] _libgettext_gettext  (line 233, static)
- [x] _libgettext_dgettext  (line 241, static)
- [x] _libgettext_dcgettext  (line 250, static)
- [x] _libgettext_ngettext  (line 260, static)
- [x] _libgettext_dngettext  (line 268, static)
- [x] _libgettext_dcngettext  (line 277, static)
- [x] _libgettext_textdomain  (line 286, static)
- [x] pgettext_aux  (line 397, static)
- [x] npgettext_aux  (line 414, static)
- [x] dcpgettext_expr  (line 463, static)
- [x] dcnpgettext_expr  (line 507, static)

## Macros defined — 27
- [x] GETTEXT_CONTEXT_GLUE  (line 354, body 31178a23aab5)
- [x] LC_MESSAGES  (line 365, body 7dbb80922345)
- [x] _LIBGETTEXT_FUNCAST  (line 66, body dd924e2eb7bf)
- [x] _LIBGETTEXT_H  (line 18, body 356a192b7913)
- [x] _LIBGETTEXT_HAVE_VARIABLE_SIZE_ARRAYS  (line 445, body b6589fc6ab0d)
- [x] _LIBGETTEXT_INLINE  (line 113, body 8de3a3b48b06)
- [x] bind_textdomain_codeset  (line 329, body 4cf068f80459)
- [x] bindtextdomain  (line 325, body 0152abbf1c92)
- [x] dcgettext  (line 301, body aa047e205b51)
- [x] dcngettext  (line 316, body e92eeaf20efa)
- [x] dcnpgettext  (line 388, body 187b43990170)
- [x] dcpgettext  (line 377, body 85f380d1111b)
- [x] dgettext  (line 298, body 08a33928497b)
- [x] dngettext  (line 312, body 19fe3b6a6d03)
- [x] dnpgettext  (line 386, body a093e883787e)
- [x] dnpgettext_expr  (line 498, body 7f7ce6c42ba6)
- [x] dpgettext  (line 375, body e7675d64be7f)
- [x] dpgettext_expr  (line 454, body 545187b3b9fa)
- [x] gettext  (line 296, body f02b6bcd2a7f)
- [x] gettext_noop  (line 350, body 3df63b7acb05)
- [x] ngettext  (line 305, body 877f685219ec)
- [x] npgettext  (line 383, body 8506e766d238)
- [x] npgettext_expr  (line 496, body f47cc5025b2b)
- [x] pgettext  (line 372, body 2db76cc1caff)
- [x] pgettext_expr  (line 452, body 1a4e2664d99a)
- [x] setlocale  (line 339, body b40493865d6d)
- [x] textdomain  (line 320, body ef64a5dd2cef)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/lib/gettext.h` @ `fd7afced403`; port `lib/include/gettext.rs`.

**Which arm is live.**  The header is not a gnulib template (there are no `@PLACEHOLDER@`s) but
it is also not in `docs/baseline/config/cond-eval-verified.txt`, so `tools/extract-abi.py`
reads every `#if` blind (P-66) and the mirror must name every arm's items.  The frozen
configuration takes the third arm:

- C 23's `#if defined ENABLE_NLS && ENABLE_NLS` is **false** - `ENABLE_NLS` is not in
  `build/emacs-ref/src/config.h` (the frozen configure was `--without-all`) - so C 26's
  `<libintl.h>` and C 31-38's DEFAULT_TEXT_DOMAIN arm are dead and the port has no catalog.
- Inside the no-NLS arm, C 109's `#if (defined __GNUC__ || defined __clang__) && !defined
  __cplusplus` is **true**, so C 121-224's seven `extern inline` definitions are the live text.
- C 337's `#ifdef GNULIB_defined_setlocale` is dead (`GNULIB_defined_setlocale` is not in the
  frozen macro table), so `setlocale` is the system's.

**The frozen values** (measured with the frozen includes:
`clang -dM -E -std=gnu23 -I build/emacs-ref/src -I build/emacs-ref/lib -I emacs/src -I emacs/lib`):
`LC_MESSAGES` 6 (C 361's `<locale.h>`), `_LIBGETTEXT_HAVE_VARIABLE_SIZE_ARRAYS` 1
(`__STDC_VERSION__` is >= 199901L and neither `GNULIB_NO_VLA` nor `__STDC_NO_VLA__` is defined)
and `_LIBGETTEXT_INLINE` `__always_inline__, __gnu_inline__` (C 111's clang arm).  The extractor
takes the **last** definition of a name, which for `LC_MESSAGES` (C 364-366: 1729) and
`_LIBGETTEXT_HAVE_VARIABLE_SIZE_ARRAYS` (C 445: 0) is the *dead* arm; both are written first,
gated, with the frozen value second and ungated - the same shape as
`lib/include/fcntl.in.rs`'s `#ifndef` fallbacks, so the extractor compares the arm C's template
spells while rustc compiles the frozen one.

**The live seven are functions.**  They are header-inline in C (`__gnu_inline__` is what keeps
C from emitting an external definition), so they are `#[inline(always)]` Rust functions with no
symbol (PORTING.md 1.2/P-12), and the four helpers C spells `static` (`pgettext_aux`,
`npgettext_aux`, `dcpgettext_expr`, `dcnpgettext_expr`) are plain functions in the mirror for the
same reason.  `dcpgettext_expr`/`dcnpgettext_expr` keep C's `#else` arm (the `buf`/`malloc`
pair, C 473-477) because Rust has no VLA: the VLA arm's tokens are a subset of that arm's.

**The dead arms (P-51), each under the feature naming its own condition:**

| C lines | arm | how it is carried |
| --- | --- | --- |
| 229-291 | the C++ arm's seven `static inline` identities | `#[cfg (feature = "cplusplus")]` functions |
| 293-322 | the plain-macro arm | `#[cfg (feature = "no_gnu_inline")] macro_rules! gettext` (the other six names are carried by the live functions; C defines every one of them twice or three times, so the extractor compares no body) |
| 336-340 | `#ifdef GNULIB_defined_setlocale` | `#[cfg (feature = "gnulib_defined_setlocale")] macro_rules! setlocale { () => { rpl_setlocale } }` |
| 440-446 | the `_LIBGETTEXT_HAVE_VARIABLE_SIZE_ARRAYS` `#else` | gated const, frozen 1 second |

**`GETTEXT_CONTEXT_GLUE`.**  C's is a string macro (`"\004"`), and a string-valued macro keeps
its value (PORTING.md 5.1), so the mirror's is `pub const … : &CStr = c"\x04"` - which is also
why the six `pgettext`-family macros concatenate with `concat!` (C's `Msgctxt
GETTEXT_CONTEXT_GLUE Msgid` is *adjacent literals*, and the header's own comment requires the
arguments to be literals) and pin the const to the byte `concat!` spells with a
`const _: () = assert! (GETTEXT_CONTEXT_GLUE.to_bytes ()[0] == 0x04);` in each body.

**Convergence (PORTING.md 0.1-3, same batch).**  The mirror's `gettext` is now the live arm's
*function*, so the identity `macro_rules! gettext` the mirror carried while the header was WIP
became the dead plain arm's macro (gated) and the two call sites moved to the function:

| unit | copy | now |
| --- | --- | --- |
| `lib/include/gettext.rs` 14 (old) | `macro_rules! gettext { ($msgid:expr) => { $msgid }; }` (an invented identity) | `#[cfg (feature = "no_gnu_inline")] macro_rules! gettext`, the dead arm's text |
| `lib/include/gettext.rs` 19 (old) | `macro_rules! N_` (unused; `N_` is not in this header) | deleted |
| `lib/regcomp.rs` 192, 473 | `gettext! (…)` | `gettext (…) as *const c_char` (C 227/530 call the function) |

No other unit spells `gettext`, `dgettext`, `_libgettext_*`, `pgettext_*`, `GETTEXT_CONTEXT_GLUE`,
`LC_MESSAGES`, `_LIBGETTEXT_*`, `bindtextdomain` or `bind_textdomain_codeset`.

## Evidence

```
$ python3 tools/extract-abi.py --check emacs/lib/gettext.h lib/include/gettext.rs
  (1 C macros have a recorded Rust counterpart: _)
  (12 C macros have several definitions (configuration branches), body not compared:
   _LIBGETTEXT_FUNCAST, _LIBGETTEXT_HAVE_VARIABLE_SIZE_ARRAYS, _LIBGETTEXT_INLINE, dcgettext,
   dcngettext, dgettext ...)
OK emacs/lib/gettext.h -> lib/include/gettext.rs

$ cd /tmp/femacs-probe2 && rustc --edition 2024 --crate-type=lib --emit=obj -C panic=abort \
    gettext_probe.rs -o gettext_probe.o
exit=0        # the probe include!s the mirror and calls all eleven live functions/helpers and
              # `gettext_noop!`, and asserts LC_MESSAGES == 6 and
              # _LIBGETTEXT_HAVE_VARIABLE_SIZE_ARRAYS == 1

$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-mir2 cargo run -q -p xtask -- build-tu <unit.c>
built lib/getopt.rs / lib/regex_internal.rs / lib/regexec.rs / lib/regcomp.rs   # all exit 0
              # (regcomp.rs is the edited consumer: the four units whose include graph reaches
              #  this mirror)
$ python3 tools/extract-abi.py --check emacs/<unit.c> <unit.rs>
OK emacs/lib/getopt.c -> lib/getopt.rs
OK emacs/lib/regcomp.c -> lib/regcomp.rs
OK emacs/lib/regexec.c -> lib/regexec.rs
OK emacs/lib/regex_internal.h -> lib/include/regex_internal.rs

$ python3 tools/check-mirror-dupes.py
1 duplicate declaration(s), 1 ratchet row(s), 0 failure(s)
$ python3 tools/check-declarations.py
13 declaration sites (0 forbidden, 13 allowed literals), 0 failures
$ python3 tools/check-verbatim.py
6 trees, 4607 files, 0 failures
$ python3 tools/check-globals.py
check-globals: 283 units, 0 DEFVAR_* variables without a definition in their own unit
```
