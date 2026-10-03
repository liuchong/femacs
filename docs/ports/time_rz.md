# emacs/lib/time_rz.c (322 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 9
- [x] extend_abbrs  (line 88, static)
- [x] tzalloc  (line 97)
- [x] save_abbr  (line 117, static)
- [x] tzfree  (line 168)
- [x] change_env  (line 201, static)
- [x] set_tz  (line 214)
- [x] revert_tz  (line 239)
- [x] localtime_rz  (line 257)
- [x] mktime_z  (line 291)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

# ----------------------------------------------------------------------
## Decisions

### Summary

`femacs/lib/time_rz.rs` is the live text of `emacs/lib/time_rz.c` for the frozen
configuration (`tools/check-cond-eval.py lib/time_rz.c`: 6 conditions, 0 unknown,
0 disagreements).  The 9 functions all have a counterpart, `extract-abi.py --check` is OK,
and the unit's object reproduces the reference's symbols exactly:

    $ nm -g build/emacs-ref/lib/time_rz.o     $ nm -g build/femacs/time_rz.o
    T _localtime_rz  T _mktime_z              the same six definitions,
    T _revert_tz     T _set_tz                and the same undefined set:
    T _tzalloc       T _tzfree                ___error _emacs_getenv_TZ _emacs_setenv_TZ
    U _emacs_getenv_TZ  U _emacs_setenv_TZ    _gmtime_r _localtime_r _malloc _memcpy
    U _gmtime_r  U _localtime_r  U _malloc    _rpl_free _rpl_mktime _rpl_timegm
    U _memcpy  U _rpl_free  U _rpl_mktime     _strcmp _strlen _tzset
    U _rpl_timegm  U _strcmp  U _strlen  U _tzset

### Arms the frozen configuration deletes

`HAVE_TZALLOC` is undefined, so C 32-63's `#if HAVE_TZALLOC` arm — the Android
`NEED_TIMEZONE_NULL_SUPPORT` wrappers of `localtime_rz`/`mktime_z` and its `tzfree` — is dead
text (a note stands where the `#if` was).  `HAVE_LOCALTIME_INFLOOP_BUG` is undefined, so
C 259-272's range guard inside `localtime_rz` is dead too.  C 182-196's two `static`
definitions (`getenv_TZ`, `setenv_TZ`) are dead because `src/conf_post.h` 178-184 defines
both names as macros.

### The header renames this unit spells (measured, not guessed)

| C text | the frozen build's generated header | evidence |
|---|---|---|
| `getenv_TZ ()` / `setenv_TZ (…)` | `src/conf_post.h` 180-181: `emacs_getenv_TZ` / `emacs_setenv_TZ` | `nm` shows `U _emacs_getenv_TZ`, `U _emacs_setenv_TZ` |
| `free (tz)` | `<stdlib.h>` 884: `# define free rpl_free` | `nm` shows `U _rpl_free` |
| `mktime (&tm_1)` | `<time.h>` 841-842: `# define mktime rpl_mktime` | `nm` shows `U _rpl_mktime` |
| `timegm (tm)` | `<time.h>` 1144-1145: `# define timegm rpl_timegm` | `nm` shows `U _rpl_timegm` |

Each is kept as C's spelling plus a `macro_rules!` of the same name (the strftime.rs
`LOCALE_PARAM` shape), so the bodies read like C'; `rpl_free`'s declaration is unit-local
(the lchmod.rs local-declaration shape) and the three `rpl_*` calls resolve through the
declarations appended to the time mirror.

### Deviations

1. **`mod tz_impl`** holds every definition.  C's own header (`time-internal.h`) declares
   `set_tz` and `revert_tz`, which the time-internal mirror carries, and a definition plus an
   `unsafe extern` declaration of one name in one module are E0428 (PORTING.md 0.1-10).  The
   module's `use super::*` gives it the mirror's items; its own names shadow the
   glob-imported declarations (§0.1-10's shadowing, the `pub mod subrs` precedent), and each
   `#[unsafe(no_mangle)]` definition still emits its C symbol (measured above).
2. **Inside that module `timezone_t` is the complete pointer.**  The time.h mirror spells
   `timezone_t` as `*mut c_void`, because `time.in.h` only forward-declares `struct tm_zone`
   (PORTING.md 0.1-2, and the mirror must compile in units that do not include
   time-internal.h).  This unit *does* include it, so the module defines
   `type timezone_t = *mut tm_zone;` — an item shadows the glob-imported alias, and
   `local_tz!` (a textual macro) resolves `timezone_t` at its expansion site, so C's spelling
   holds everywhere with no casts.  A probe verified both resolutions before the port was
   written.
3. **The flexible array member** `struct tm_zone.abbrs` is the mirror's `[c_char; 0]`; a Rust
   index of a zero-length array is rejected outright, so the port reaches it through
   `abbrs (tz)` — the base address with `addr_of_mut!`, the lisp.rs `fam_base`/`FAM_ELEM!`
   idiom (PORTING.md 1.3).  The helper is named after the member so the ledger's token check
   still sees C's field name.
4. `struct tm tm_1;` (C 300) is zero-initialized (PORTING.md 1.5) and C's field assignments
   stay as separate statements.
5. C's `static char const mt[] = ""`, a block-scope static, is a function-local
   `static mt: [c_char; 1]` (Rust allows items in a body), so its lifetime is C's.  The C
   text advances `zone_copy` past that one-byte array (`strnul (zone_copy) + 1`), which the
   port reproduces with raw-pointer arithmetic rather than a Rust index.
6. `streq` and `strnul` are gnulib `<string.h>` extensions, so the port takes them from the
   string mirror (`streq!`; `strnul!` over the header's `_gl_strnul`), which was appended
   rather than re-shaped.
7. The `DEFAULT_MXFAST`/`ABBR_SIZE_MIN` enumerators are `usize` constants (they are only
   compared with `size_t` values), and `ABBR_SIZE_MIN` uses `offset_of!` for C's
   `offsetof (struct tm_zone, abbrs)`.

### Shared files touched (all additive)

* `femacs/lib/include/time.in.rs`: appended a second `unsafe extern "C"` block declaring
  `rpl_mktime` and `rpl_timegm` (with the reference-object evidence).  Nothing existing was
  reordered; `--check emacs/lib/time.in.h …` reports the same two pre-existing WIP items as
  before the append.
* `femacs/lib/include/string.rs`: appended `_gl_strnul` + `strnul!` (string.in.h 1250-1305;
  the header's `_GL_STRNUL_INLINE` function is the mirror's, per PORTING.md 10.12).  The
  `streq!` macro this unit also calls came from another change already in the tree.
* `femacs/lib/include/flexmember.rs` (new mirror) + `docs/ports/include-flexmember.md`:
  `FLEXALIGNOF`, `FLEXSIZEOF`, `FLEXNSIZEOF`.  `FLEXNSIZEOF` is expanded by no ported unit;
  it keeps C's place expression (`sizeof (((type *) 0)->member[0])`) with `size_of_val`.
* `docs/ports/token-aliases.tsv`: the `sizeof` family gained `size_of_val`, and a `free`
  family was added (`free`,`rpl_free`) for the rename above; both are monotone (they can only
  turn a failing file green, and every other `free`/`sizeof` user still checks OK).
* `docs/ports/macro-body-notes.tsv`: `FLEXALIGNOF`'s `_alignof`.
* `docs/ports/line-drift-tolerance.tsv`: `time_rz 0.15`.

### Verification

* `python3 tools/check-cond-eval.py lib/time_rz.c` → `6 conditions, 0 unknown,
  0 disagreements [VERIFIED]`.
* `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target cargo run -q -p xtask -- build-tu lib/time_rz.c`
  → `built lib/time_rz.rs -> ../build/femacs/time_rz.o`, 0 errors.
* `python3 tools/extract-abi.py --check emacs/lib/time_rz.c femacs/lib/time_rz.rs` → OK.
* Symbol comparison against the frozen object (above).  No *behavioural* differential probe
  was run (the round's workflow stops at `build-tu`/`--check`).

