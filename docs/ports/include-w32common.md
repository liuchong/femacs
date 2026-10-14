# emacs/src/w32common.h (103 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] get_proc_addr  (line 67)

## Macros defined — 11
- [x] DEF_DLL_FN  (line 77, body d8e52635513d)
- [x] LOAD_DLL_FN  (line 82, body 71bae1fa8aee)
- [x] LOAD_DLL_FN_OPT  (line 92, body 54cfc5abe4c0)
- [x] ROUND_DOWN  (line 27, body 90b552029495)
- [x] ROUND_UP  (line 26, body 2726c2e9c2f7)
- [x] W32COMMON_H  (line 22, body da39a3ee5e6b)
- [x] get_allocation_unit  (line 30, body d17c786ec1f8)
- [x] get_page_size  (line 29, body b74d3952805a)
- [x] get_processor_type  (line 31, body 0ac10f7446f3)
- [x] get_w32_major_version  (line 32, body 67fe267090c6)
- [x] get_w32_minor_version  (line 33, body 43d86b24b333)

## Types and their fields — 0

## File-scope objects — 7
- [x] extern SYSTEM_INFO sysinfo_cache  (line 35)
- [x] extern OSVERSIONINFO osinfo_cache  (line 36)
- [x] extern DWORD_PTR syspage_mask  (line 37)
- [x] extern int w32_major_version  (line 39)
- [x] extern int w32_minor_version  (line 40)
- [x] extern int w32_build_number  (line 41)
- [x] extern int os_subtype  (line 48)

## Decisions

- **The header is unconditional apart from two dead arms.**  C 21-22 is the include guard, C 24
  is `#include <windows.h>`, and the rest is live text except C 53-58 (`#ifdef WINDOWSNT`) and
  C 99-101 (`#ifdef HAVE_HARFBUZZ`).  `tools/check-cond-eval.py src/w32common.h` reports
  `3 conditions, 0 unknown, 0 disagreements [VERIFIED]` (guard, `WINDOWSNT`, `HAVE_HARFBUZZ`) and
  `docs/baseline/config/darwin-arm64.macros.tsv` records both feature macros as `undef`, so the
  two arms are dead text here: they are carried under `#[cfg (feature = "windowsnt")]` /
  `#[cfg (feature = "harfbuzz")]` (PORTING.md P-51 names the non-frozen side; `feature =
  "windowsnt"` is the name `src/vm-limit.rs` 95 already uses).  Nothing on this target compiles
  the header - `WINDOW_SYSTEM_OBJ` is empty (docs/baseline/frozen-build-sources.txt) - so no unit
  `include!`s the mirror and the declarations are deferred to the Windows target.
- **`--check` is not OK for this file, and the five gaps are the deferrals below.**  The exact
  readings are in `## Evidence`; three are macros whose Rust form is an open design decision and
  two are objects whose C type this target cannot measure.
- **C 77-97 (`DEF_DLL_FN`, `LOAD_DLL_FN`, `LOAD_DLL_FN_OPT`) are deferred, not written.**  C
  composes two names per expansion by token pasting (`W32_PFN_##func`, `fn_##func`) and takes the
  function's parameter list, with any trailing attributes, as `__VA_ARGS__`; Rust has no token
  pasting and no counterpart for the trailing-attribute form, so the Rust shape is a decision for
  the Windows target that first expands them (PORTING.md P-22 - the `kind=deferred` class in
  `docs/ports/excluded-macros.txt`), not something to invent here.  The mirror keeps C's three
  bodies in one comment block where the macros would be, so a reader sees what is not carried,
  and the rows the extractor needs are listed in `## Evidence` for the integrator to add.
- **C 35-36 (`sysinfo_cache`, `osinfo_cache`) are deferred.**  C declares a `<sysinfoapi.h>`
  record and a `<winver.h>` record **by value**; the port has no mirror of either header and no
  compiler on this target could measure them, so no Rust spelling is written rather than an
  unmeasured one (the `struct file_data`-layout rule of `include-w32heap.rs`, from the other
  side).  The five `get_*` macros above still name `sysinfo_cache` because C's own bodies do
  (C 29-31) and a `macro_rules!` body is not resolved until it is expanded.
- **`DWORD_PTR` is declared, and its width is PORTING.md's mapping, not a measurement.**  The two
  rounding macros cast through it (C 26-27), so the mirror declares
  `pub type DWORD_PTR = usize;` - `uintptr_t`'s row in PORTING.md 1.1, which is what the platform
  typedef is (`typedef ULONG_PTR DWORD_PTR;`).  A Windows target that grows a windows.h mirror
  must retire this copy rather than declare the name twice.
- **`GetProcAddress` (C 69's callee) is declared here, and `get_proc_addr` is the port's
  definition of C's one.**  `<windows.h>` declares `FARPROC GetProcAddress (HMODULE, LPCSTR)`,
  which is two pointer arguments and a function-pointer result; the port has no windows.h mirror,
  so the declaration lives in this mirror with the shape the header's own body needs, and
  `get_proc_addr` writes C's cast as a **type ascription** (`let addr: VOIDFNPTR = …`) because a
  Rust `as` cast to an `Option<unsafe extern "C" fn ()>` does not compile.  `VOIDFNPTR` is C's
  nullable function pointer, so it is an `Option` (PORTING.md P-27, the `ms-w32.rs` `malloc_fn`
  spelling), and C 56's `intmax_t` is `i64` and C 101's `HMODULE` a pointer.
- **The two `#ifdef` arms carry their names as P-51 text and nothing else**: the extractor's
  `## Functions defined` count is 1 (`get_proc_addr`), and none of `w32_version_string`,
  `w32_valid_process_id`, `VALID_PROCESS_ID` or `hbfont_init_w32_funcs` is in its item list - C's
  own preprocessor drops them here.
- **`--check` asks for less than fidelity does** for the declarations: C 51 and C 55-56 are
  declarations (or a macro) with no checklist row, and the `## File-scope objects` rows only cover
  the seven `extern`s.

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/w32common.h femacs/src/include/w32common.rs`
  -> **FAIL**, with exactly these five lines and no others:
  ```
  - missing macro DEF_DLL_FN
  - missing macro LOAD_DLL_FN
  - missing macro LOAD_DLL_FN_OPT
  - missing file-scope object sysinfo_cache
  - missing file-scope object osinfo_cache
  ```
  i.e. the three deferred macros (C 77-97) and the two deferred objects (C 35-36); every other
  item - the guard, `ROUND_UP`/`ROUND_DOWN`, the five `get_*` macros, the five remaining objects,
  the two enum constants, `get_proc_addr`'s body and the `WINDOWSNT`/`HAVE_HARFBUZZ` text - is
  accepted, including every macro body's tokens.
- `python3 tools/check-cond-eval.py src/w32common.h` -> `3 conditions, 0 unknown to the evaluator,
  0 disagreements`.
- `python3 tools/extract-abi.py emacs/src/w32common.h` -> this ledger: 0 subrs, **1 function
  defined** (`get_proc_addr`), 11 macros, 0 types, 7 file-scope objects.
- **The rows this file needs in the shared tables** (the lane may not write them; reported for the
  integrator):
  - `docs/ports/excluded-macros.txt`:
    `DEF_DLL_FN	deferred	w32common.h 77-79: C pastes `W32_PFN_##func`/`fn_##func` and takes the parameter list plus trailing attributes as `__VA_ARGS__`; Rust has no token pasting and the Rust shape is a Windows-target decision (P-22)`
    `LOAD_DLL_FN	deferred	w32common.h 82-89: the `DEF_DLL_FN` row's loader (a `do … while (false)` body whose missing-symbol arm returns false)`
    `LOAD_DLL_FN_OPT	deferred	w32common.h 92-97: the same loader without the failure arm`
  - `docs/ports/include-w32common.md` (this file) is the record of the two object deferrals.
- **Compile evidence**: `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-win cargo run -q -p xtask -- layout-check`
  -> `layout-check: 66 of 77 mirror(s) compiled, 11 platform-exclusive skipped (android-asset.rs,
  android.rs, androidgui.rs, androidterm.rs, dispextern.rs, gtkutil.rs, haiku_support.rs,
  haikugui.rs, haikuterm.rs, nsgui.rs, pgtkterm.rs)`, `layout probe ok`,
  `layout-check passed (see build/femacs/layout_asserts.rs)`.  **This mirror is among the 66** (it
  is not in that skip list), so the gate compiles it every round.  An earlier run of the same gate,
  with only `cygw32.rs` of this batch present, printed `layout probe ok`; a later one failed with
  `error[E0428]: the name Emacs_Cursor is defined multiple times … nsgui.rs:57:1 …
  dispextern.rs:62:1` - two mirrors of another lane, both of which had entered the skip list by the
  time of the passing run, and no diagnostic of either run named a mirror of this batch.  The `<windows.h>` names it declares (`DWORD_PTR`, `VOIDFNPTR`, `GetProcAddress`) resolve inside the crate, and the two deferred objects (C 35-36) are absent by design.

- Gates on the same worktree: `check-declarations` 13 sites (0 forbidden, 13 allowed literals) / 0
  failures; `check-mirror-dupes` 1 duplicate declaration(s) / 1 ratchet row(s) / 0 failure(s);
  `check-globals` 283 units / 0; `check-verbatim` 6 trees / 4607 files / 0 failures;
  `check-target-names` 566 .rs files / 593 map targets / 0 findings.

## Evidence

```
$ python3 tools/extract-abi.py --check emacs/src/w32common.h femacs/src/include/w32common.rs
OK emacs/src/w32common.h -> femacs/src/include/w32common.rs            (0 problem rows)
$ cd femacs && cargo run -q -p xtask -- layout-check
layout-check passed (see build/femacs/layout_asserts.rs)
```

The three DLL-loader macros were already recorded (`excluded-macros.txt` 149-151, kind=deferred:
their bodies paste names, use `__VA_ARGS__` and `return` from the enclosing function - none of which
a `macro_rules!` can spell, so the Windows target that first expands them decides the shape,
PORTING.md P-22).  The two caches are now carried under `#[cfg (feature = "windowsnt")]` with C's
type names, the way `w32gdiplus.rs` carries its objects; `layout-check` compiles this mirror (it is
a `src/include` seed) with the feature off, which is the evidence that the cfg'd items cannot break
the shared crate.

# ----------------------------------------------------------------------
## The nine declarations move into `w32common_decls` (2026-10-05, `docs/ports/w32fns.md` owed 5)

**C 35-51.**  This header's file-scope objects and its one function are all defined in
`src/w32fns.c` (`syspage_mask`, `w32_major_version`, `w32_minor_version`, `w32_build_number`,
`os_subtype`, `cache_system_info`, and the two deferred caches), so a crate-root declaration here
collides with that unit's own item - measured, five `error[E0428]` naming
`w32common.rs` 80/81/82/83/95 in `xtask probe src/w32fns.rs … --cfg 'feature="have_ntgui"'
--cfg 'feature="windowsnt"'`.  They now sit in `pub mod w32common_decls` with
`use w32common_decls::*;` (the `window.rs` 302-367 / `androidterm.rs` 513-537 shape,
PORTING.md 0.1-10 / P-21); the two `<windows.h>`-typed caches keep their
`#[cfg (feature = "windowsnt")]` per item, and `GetProcAddress`/`get_proc_addr` stay where they
were (the loader's own body is the one definition in this mirror).

**Evidence.**  `extract-abi --check emacs/src/w32common.h femacs/src/include/w32common.rs` -> `OK`
(the three deferred macros and two deferred objects are its recorded deferrals, unchanged);
`xtask layout-check` passed; `xtask probe src/w32fns.rs` above falls from 12 to 0 `E0428`;
`check-all-units.sh` 83/0 both legs.

## `w32_version_string` moves into `w32common_decls` (2026-10-14, w32fns slice 24)

The tenth declaration of the same class as the nine above.  `src/w32fns.c` 10085 defines
`char * w32_version_string (void)` (C 10083's `#ifdef WINDOWSNT`), and this mirror declared the name
at file scope under `#[cfg (feature = "windowsnt")]`, so the unit's own definition collided with it
(`E0428`, measured: the w32fns arm probe read 43 errors against the standing 42, the extra one
`the name w32_version_string is defined multiple times`).  The declaration now stands inside
`w32common_decls` with the four version objects, and its file-scope copy is removed; every unit that
calls it still resolves the name through `use w32common_decls::*;`.

## 2026-10-14: `w32_valid_process_id` moves into `w32common_decls` (4.297)

The eleventh move of the class this file's C 53 note records (`w32_version_string` was the tenth).
`src/w32proc.c` 3488 **defines** `w32_valid_process_id`, so the top-level `unsafe extern "C"` copy
was one `E0428` in `src/w32proc.rs`; measured: `probe src/w32proc.rs` read **36 errors (26 `E0428` +
9 `E0425` + 1 `compile_error!`)** before the move and **35 (25 + 9 + 1)** after it, with the new
`E0428` anchored here at line 191.  The declaration sits in the glob-imported `w32common_decls` now;
`VALID_PROCESS_ID!` stays at the root.
