# emacs/lib/getopt.c (811 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 5
- [x] exchange  (line 131, static)
- [x] process_long_option  (line 193, static)
- [x] _getopt_initialize  (line 385, static)
- [x] _getopt_internal_r  (line 475)
- [x] _getopt_internal  (line 706)

## Macros defined — 5
- [x] GETOPT_ENTRY  (line 728, body 48fa758d88a5)
- [x] NONOPTION_P  (line 495, body 3c66fcd8d9c6)
- [x] _  (line 45, body bcead234c68a)
- [x] __libc_use_alloca  (line 54, body b6589fc6ab0d)
- [x] alloca  (line 56, body f247bc660ed5)

## Types and their fields — 0

## File-scope objects — 5
- [x] char optarg  (line 89)
- [x] int optind  (line 104)
- [x] int opterr  (line 109)
- [x] int optopt  (line 115)
- [x] static struct _getopt_data getopt_data  (line 119, static)

## Decisions

`emacs/lib/getopt.c` @ `fd7afced403`; port `lib/getopt.rs` (870 lines).  All checklist
items ticked; `build-tu lib/getopt.c` -> `built lib/getopt.rs -> ../build/femacs/getopt.o`
(0 errors, 4 warnings), `--check` -> `OK`.

- **Which arm is live** (the file is VERIFIED by `tools/check-cond-eval.py`): `_LIBC` is
  undefined, so C 22-24's `# include <config.h>` is live and C 40-60's `#ifdef _LIBC` block
  (the `fprintf` -> `__fxprintf_nocancel` and `_IO_*` lock renames) is dead;
  `_POSIX_THREAD_SAFE_FUNCTIONS` is `200112L`, so C 48-50's empty `flockfile`/`funlockfile`
  arm is dead and the two locks are the real stdio functions; C 61's `# else` arm is the live
  text (gettext.h + `_` + the alloca repurposing); C 731-733's `#ifdef _LIBC` `GETOPT_ENTRY`
  twin and C 740-811's `#ifdef TEST` program are dead and are named at their sites (P-51).
- **The `__GETOPT_PREFIX` layer.**  `__GETOPT_PREFIX` is `rpl_` in the frozen configuration,
  so the names this unit defines are the prefixed ones (PORTING.md 10.13): the four globals
  and the `GETOPT_ENTRY` expansion carry `#[unsafe(export_name = "rpl_optarg")]` etc., and
  `_getopt_internal` is `rpl_getopt_internal`.  `nm build/femacs/getopt.o` now defines
  `_rpl_getopt`, `_rpl_getopt_internal`, `_rpl_optarg`, `_rpl_opterr`, `_rpl_optind`,
  `_rpl_optopt` and the unrenamed `__getopt_internal_r` - the same set the reference object
  defines.  One recorded difference: C's tentative definition `char *optarg;` is a *common*
  symbol (`C`), the port's `static mut` is `S`/`D` in the object; both are zero-initialised
  and every use goes through the same name.
- **`_` cannot be a Rust identifier** (PORTING.md 0.2), so the macro is recorded in
  `docs/ports/excluded-macros.txt` and its body is written out at the five message sites.
  With gettext.h's live arm (`GNULIB_TEXT_DOMAIN` is `"gnulib"`, and the header's
  no-libintl arm defines an inlined identity `dgettext`), the body reduces to `msgid` - the
  reference object has no `dgettext` symbol at all (`nm build/emacs-ref/lib/getopt.o`), which
  is the same evidence the port's call sites rely on.
- **`NONOPTION_P` (C 495) names the enclosing function's `argv` and `d`**, which a Rust
  `macro_rules!` cannot (hygiene), so the port passes both in - the strftime.rs/md5.rs shape -
  and the call sites read `NONOPTION_P! (argv, d)`.
- **`alloca` (C 56)** is C's comma expression `(abort (), (void *) 0)`; Rust spells a comma
  expression as a block whose last expression is the value (the sha1.rs `M` shape), so the
  macro body is `{ abort (); 0 as *mut c_void }`.  `__libc_use_alloca` (C 54) is the constant
  `0`, as in C.
- **`GETOPT_ENTRY` (C 728)** defines a function from two macro parameters, which Rust's
  `macro_rules!` does with a metavariable name; C 738's `GETOPT_ENTRY (getopt, 1)` is the one
  invocation (the `#ifdef _LIBC` twin is dead).
- **Initialisation.**  `static struct _getopt_data getopt_data;` (C 119) is C's zero value
  written out, and `int option_index;` (C 206) starts at C's zero for the type because Rust
  needs a value before the binding can be read (PORTING.md 0.1-13).
- **Mirrors.**  `include/getopt.in.rs` (the generated `<getopt.h>`),
  `include/getopt-cdefs.in.rs`, `include/getopt-pfx-core.rs`, `include/getopt-pfx-ext.rs`,
  `include/getopt-core.rs`, `include/getopt-ext.rs` and `include/getopt_int.rs` are new; the
  first three carry the record only, because a `#include_next` of the system header, the
  `__BEGIN_DECLS`/`__THROW` attributes and the `rpl_` rename layer have no Rust item (the
  renames are expressed with `link_name`/`export_name` where the names are used).  The
  not-yet-mirrored libc entries the live text needs (`fprintf`/`stderr`, `flockfile`,
  `funlockfile`, `abort`, `getenv`, `strncmp`) are declared in the unit, the fstatat.rs shape.
- Line drift: `line-drift-tolerance.tsv` records `getopt 0.10` (7.3%).
- Not verified: gate-fast/gate-probes, ERT, and any behavioural test of the parser.
