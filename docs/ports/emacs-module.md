# emacs/src/emacs-module.c (1798 lines) — whole file, C 1-1798

> **One of P-86's class: the frozen configuration compiles nothing here.**  `emacs-module.o` is in
> the module object list `emacs/src/Makefile.in` fills under `HAVE_MODULES`, and
> `build/emacs-ref/src/Makefile`'s platform object variables (`WINDOW_SYSTEM_OBJ=`, `HAIKU_OBJ = `,
> `MSDOS_OBJ =`, `CYGWIN_OBJ=`) are all empty for this target — so
> `docs/baseline/frozen-build-sources.txt` lists no `emacs-module.o`, `extract-abi.py`'s checklist
> for the file is empty (P-86 ②), and `build-tu` compiles a **336-byte** empty object.  This ledger
> is that clause's substitute: the **C line-range table**, the `## Evidence` readings and the
> `## Deviations`.
>
> Port: `femacs/src/emacs-module.rs` (**2010 lines**, md5 `1825d8a7e1d5d34f610e145e980c00db`),
> carrying **C 1-1798 with both arms of every `#if`**.  The reference `emacs/src/emacs-module.c` is
> md5 `19364d4adcd108826472d06a1cb937d6` (unchanged; `emacs/` is read-only).
>
> **The gate set is `have_modules` + `windowsnt` + `have_window_system` +
> `gcc_lint_not_clang`**, the file gate plus the three conditions the body carries.  P-86 ①'s
> reading applies: transcribe the text, both arms, no stub.
>
> **Acquisition: the `windowsnt` arm is `text-only` (P-86 ③⑵); the rest was arm-flipped.**  The
> machine has no Windows SDK and no mingw toolchain, so `#include <windows.h>` (C 113) has no
> header to compile against and the arm's only mirror (`include/w32term.rs`) fails on its own
> Windows-SDK declarations (Evidence ④: 98 errors, all inside that mirror).  The other three gates
> were compiled: Evidence ② (frozen), ③ (the `have_modules` + `have_window_system` +
> `gcc_lint_not_clang` arm).
>
> Every `#if` this file carries, and both arms of each:
>
> | C condition | port gate | C lines |
> | --- | --- | --- |
> | the file itself (`HAVE_MODULES`; `undef` in `docs/baseline/config/darwin-arm64.macros.tsv`) | `feature = "have_modules"`, on every item *and* on C 78-96's include graph (P-86 ⑧) | the whole file |
> | `#ifdef WINDOWSNT` | `feature = "windowsnt"` | C 112-115 |
> | `#ifdef HAVE_WINDOW_SYSTEM` | `feature = "have_window_system"` (the `#else` arm is C 973) | C 970-974 |
> | `#if GCC_LINT && __GNUC__ && !__clang__` | `feature = "gcc_lint_not_clang"` (the triple is one feature: `GCC_LINT` is 1 but `__clang__` is defined in the frozen table, so the condition is false there — the `femacs/src/eval.rs` 1604-1606 spelling) | C 1076-1083 |

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-18 | the licence/version head (`/* emacs-module.c … */`) | **dropped** (PORTING.md 0.1-19) |
| 20-74 | the module-API rules comment (`emacs-module.h`'s contract) | file gate (carried verbatim) |
| 76 | `#include <config.h>` | ungated |
| 78 | `#include "emacs-module.h"` (the generated header) | file gate |
| 80-84 | `<stdarg.h>`, `<stddef.h>`, `<stdint.h>`, `<stdlib.h>`, `<time.h>` | file gate; C 80 is the port's `use core::ffi::VaList` |
| 86-94 | `lisp.h`, `bignum.h` (25 `<gmp.h>`), `dynlib.h`, `coding.h`, `keyboard.h`, `process.h`, `syssignal.h`, `sysstdio.h`, `thread.h` | file gate; the mirrors C's chain reaches (termhooks/frame/window/font, string, mini-gmp) are listed with their own lines |
| 96 | `#include <intprops.h>` | file gate |
| 98-107 | the two user-object allocation strategies comment | file gate |
| 110-115 | `/* Feature tests.  */`, `#ifdef WINDOWSNT`, `<windows.h>`, `w32term.h`, `#endif` | file gate; C 113-114 the `windowsnt` arm |
| 117-118 | `typedef int (*emacs_init_function) (struct emacs_runtime *);` | file gate |
| 120-155 | `struct emacs_value_tag`, `value_frame_size`, `struct emacs_value_frame`, `struct emacs_value_storage` | file gate |
| 157-180 | `struct emacs_env_private`, `struct emacs_runtime_private` | file gate |
| 183-206 | C's forward declarations | file gate; **dropped** (PORTING.md 0.1-12) |
| 207 | `static bool module_assertions = false;` | file gate |
| 210-233 | `module_decode_utf_8`, `module_memory_buffer_too_small` | file gate |
| 236-264 | the non-local-exit policy comment | file gate |
| 266-289 | `MODULE_HANDLE_NONLOCAL_EXIT` | file gate |
| 291-292 | `MODULE_INTERNAL_CLEANUP` | file gate |
| 294-329 | the six rules comment | file gate |
| 331-350 | `MODULE_FUNCTION_BEGIN_NO_CATCH`, `MODULE_FUNCTION_BEGIN` | file gate |
| 352-362 | `CHECK_MODULE_FUNCTION`, `CHECK_USER_PTR` | file gate |
| 364-374 | `module_get_environment` | file gate |
| 376-404 | `Vmodule_refs_hash`, `struct module_global_reference`, `XMODULE_GLOBAL_REFERENCE` | file gate |
| 406-423 | `module_global_reference_p` | file gate |
| 425-460 | `module_make_global_ref` | file gate |
| 462-491 | `module_free_global_ref` | file gate |
| 493-507 | `module_non_local_exit_check`, `module_non_local_exit_clear` | file gate |
| 509-536 | the two dedicated OOM slots, `module_non_local_exit_get` | file gate |
| 538-558 | `module_non_local_exit_signal`, `module_non_local_exit_throw` | file gate |
| 560-589 | `struct Lisp_Module_Function`, `allocate_module_function`, `XSET_MODULE_FUNCTION` | file gate |
| 591-627 | `module_make_function` | file gate |
| 629-668 | `module_get_function_finalizer`, `module_set_function_finalizer`, `module_finalize_function`, `module_make_interactive` | file gate |
| 670-680 | `module_function_interactive_form`, `module_function_command_modes` | file gate |
| 682-703 | `module_funcall` | file gate |
| 705-725 | `module_intern`, `module_type_of` | file gate |
| 727-787 | `module_is_not_nil` … `module_make_float` | file gate |
| 789-834 | `module_copy_string_contents` | file gate |
| 836-877 | `module_make_string`, `module_make_unibyte_string`, `module_make_user_ptr` | file gate |
| 879-919 | `module_get_user_ptr` … `module_set_user_finalizer` | file gate |
| 921-938 | `check_vec_index`, `module_vec_set` | file gate |
| 940-963 | `module_vec_get`, `module_vec_size` | file gate |
| 965-977 | `module_canvas_data` (C 970-974 the `HAVE_WINDOW_SYSTEM` pair) | file gate; both arms of `have_window_system` |
| 979-998 | `module_should_quit`, `module_process_input` | file gate |
| 1000-1022 | `module_extract_time`, `module_make_time` | file gate |
| 1024-1059 | the GMP-strategy comment | file gate |
| 1061-1070 | `module_bignum_count_max`, the two `static_assert`s | file gate |
| 1072-1168 | `module_extract_big_integer` (C 1076-1083 the `GCC_LINT` triple) | file gate; both arms of `gcc_lint_not_clang` |
| 1170-1190 | `module_make_big_integer` | file gate |
| 1192-1202 | `module_open_channel` | file gate |
| 1204-1223 | `/* Subroutines.  */`, `module_signal_or_throw` | file gate |
| 1225-1284 | `module-load` (the file's only `DEFUN`) | file gate |
| 1286-1333 | `funcall_module` | file gate |
| 1335-1360 | `module_function_arity`, `_documentation`, `_address`, `_data` | file gate |
| 1362-1409 | `module_assert_thread`, `module_assert_runtime`, `module_assert_env` | file gate |
| 1411-1445 | `module_non_local_exit_signal_1`, `_throw_1`, `module_out_of_memory` | file gate |
| 1447-1493 | `value_to_lisp`, `lisp_to_value` | file gate |
| 1495-1550 | `initialize_frame`, `initialize_storage`, `finalize_storage`, `allocate_emacs_value` | file gate |
| 1552-1565 | `mark_module_environment` | file gate |
| 1567-1631 | `initialize_environment` | file gate |
| 1633-1652 | `finalize_environment`, `finalize_environment_unwind`, `finalize_runtime_unwind` | file gate |
| 1654-1686 | `module_reset_handlerlist`, `module_handle_nonlocal_exit` | file gate |
| 1688-1714 | `init_module_assertions`, `value_storage_contains_p` | file gate |
| 1716-1727 | `module_abort` | file gate |
| 1729-1798 | `syms_of_module` (the nine `DEFSYM`s, their `Fput`s, the two `staticpro`s, `defsubr`) | file gate |

## Evidence

**① `build-tu` (the frozen configuration).**

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/emod-lane cargo run -q -p xtask -- build-tu src/emacs-module.c
built src/emacs-module.rs -> ../build/femacs/emacs-module.o
exit 0;  build/femacs/emacs-module.o = 336 bytes
```

336 bytes is **the empty object the file gate leaves**: the frozen arm compiles nothing of this
file, exactly as the reference's own frozen configuration compiles nothing of it.

**② The arm probe (`have_modules` + `have_window_system` + `gcc_lint_not_clang`).**

```
$ cd femacs && RUSTUP_TOOLCHAIN=nightly-2026-10-01 CARGO_TARGET_DIR=/tmp/emod-lane \
    cargo run -q -p xtask -- probe src/emacs-module.rs /tmp/emod-lane/emod-arm.rlib \
    --crate-type=lib -C panic=abort \
    --cfg 'feature="have_modules"' --cfg 'feature="have_window_system"' \
    --cfg 'feature="gcc_lint_not_clang"'
exit 1
```

- **errors: 1**, and it is a **mirror copy**, not the unit:
  `build/probes/emacs-module/_Users_…_femacs_src_include_dispextern_rs.rs:796` =
  `femacs/src/include/dispextern.rs` 796's `pub type Emacs_Pix_Context = Emacs_Pix_Container;`
  (`E0425`; rustc's own note points at dispextern.rs 739, whose `Emacs_Pix_Container` arm is
  `all (have_window_system, have_x_windows, not use_cairo)` — with `have_window_system` and no GUI
  backend a real configuration does not exist, so the mirror's arm set has a hole there;
  `## Owed` 6).
- **errors in the unit itself (`_root.rs`): 0.**
- warnings: 15 — 11 in the unit (`unused doc comment`: the `/// C: <owner header> <line>` lines this
  lane had put in front of `unsafe extern "C"` blocks, a nuisance the mirrors carry the same way:
  `include/emacs-module.in.rs` 619, `lib/include/stdbit.in.rs` 645, `include/lisp.rs` 3940; the
  reading was taken before those 11 were converted to `// C:` — a comment-only change, so the
  compiled text is unchanged), 4 in mirror copies (`include/frame.rs` 1361 `unused attribute` and the
  three above).  The unit's statements themselves carry no warning.
- toolchain: `rustc 1.101.0-nightly` — nightly is required because `module_abort`'s `...`
  parameter is `c_variadic` (PORTING.md P-04; the frozen `build-tu` run does not compile it).
- the file's four gates were given their frozen values: `have_modules` on, `windowsnt` off,
  `have_window_system` on, `gcc_lint_not_clang` on.
- two **comment-only** edits were made after this reading (the `/// C:` lines in front of the unit's
  `unsafe extern "C"` blocks became `// C:`, which drops the 11 `unused doc comment` warnings, and
  the integrator's drift report moved one citation to `dispextern.h 1392`): no statement changed, so
  this reading still describes the file's compiled text.

**③ The two earlier probe runs (what they measured, and why they were repeated).**

- run 1 (`+ windowsnt`, `+ have_window_system`): 150 errors — **39 in the unit** (21 `NULL`, 6
  `list`, 6 `mpz_*`/`__mpz_struct`, 3 implicit conversions, 1 uninitialized `args`), **98 inside
  `include/w32term.rs`** (the Windows SDK mirror: `HWND`/`HDC`/`RECT`/… are the SDK's, and this
  machine has no `<windows.h>` — P-86 ③⑵'s `text-only` arm), 25 inside `include/bignum.rs` (the
  unit had not included `lib/include/mini-gmp.rs`, which bignum.h 25's `<gmp.h>` reaches), 3 in
  `include/dispextern.rs`, 1 in `include/lisp.rs` (a `SAFE_NALLOCA!` use whose `args` the unit left
  uninitialized).
- run 2 (`+ have_window_system + have_x_windows`, no `windowsnt`): 7 errors — 2 in the unit
  (`MODULE_FUNCTION_BEGIN! (…)`'s `NULL` in the two `emacs_finalizer`-returning functions, where C's
  macro argument is `NULL` and the port needs `None`), 2 in `include/dispextern.rs` (`frame`/
  `window`), 1 in `include/systime.rs` 27 (the mirror's own
  `const _: () = core::assert! (!cfg! (feature = "have_x_windows") || HAVE_X_WINDOWS)` — the
  feature/constant pair `xtask dead-arms` flips together, P-51, not a lane-level cfg).
- Every unit-side error those runs found is fixed; run ② above is the reading after the fixes.

**④ The `windowsnt` arm (`text-only`, P-86 ③⑵).**

```
$ cd femacs && RUSTUP_TOOLCHAIN=nightly-2026-10-01 CARGO_TARGET_DIR=/tmp/emod-lane \
    cargo run -q -p xtask -- probe src/emacs-module.rs … --cfg 'feature="have_modules"' \
    --cfg 'feature="windowsnt"'
98 errors, every one inside build/probes/emacs-module/_…_w32term_rs.rs (E0425: `HWND`, `HDC`,
`RECT`, `HINSTANCE`, … — the Windows SDK's own names)
```

The unit's own two lines of that arm (C 113-114) are two `#include`s of a system header and one
mirror; the arm cannot be arm-flipped here.  `which -a x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc
mingw32-gcc` is empty and no `windows.h`/`winnt.h` exists on this machine.

**⑤ `extract-abi.py --check`.**

```
$ python3 tools/extract-abi.py --check emacs/src/emacs-module.c femacs/src/emacs-module.rs
- missing field Lisp_Module_Function.header
… the nine C fields …
- struct Lisp_Module_Function: missing fields ['header', …, 'finalizer']
- stub or empty body in fn finalize_runtime_unwind
- line count drifted 11.8%: C 1798 vs Rust 2010
FAIL emacs/src/emacs-module.c -> femacs/src/emacs-module.rs
```

**Twelve rows.**  Nine token rows were cleared by this lane's nine appended rows in
`docs/ports/macro-body-notes.tsv` (the `do { … } while (false)` tails of the two non-local-exit
macros, `min`, `0`, `mpz_t`/`static_assert`, three `switch`→`match` sites, one `goto`, and
`va_list`/`va_start`/`va_end`).  The twelve that remain are:
the ten `Lisp_Module_Function` rows (`## Owed` 1 — `rust_inventory` merges the unit's own text
*and* its `include!` targets, and `include/lisp.rs` 2350-2355's crate-root placeholder
`pub struct Lisp_Module_Function { _private: [u8; 0] }` is matched **after** the unit's definition,
so its one-field list overwrites the unit's nine: measured with `rust_inventory` directly —
`structs['Lisp_Module_Function'] == ['_private']`), the `finalize_runtime_unwind` row (`## Owed` 4:
C's own body is one comment, and the check's `empty_in_c` does not treat a comment-only C body as
empty), and the line-count row (11.8%, the integrator's tolerance row).

**⑥ The five text gates (whole-tree last lines; whether this lane's files appear).**

| gate | whole-tree last line | this lane's files |
| --- | --- | --- |
| `check-citations.py` | `676 file(s) scanned, 1 drift, 0 range, 1603 external, 36 allowed, 0 stale` (the one drift is `femacs/src/xmenu.rs:872`, another lane's file, and it landed while this lane ran) | no finding: the integrator's `termhooks.h 27` drift report was fixed in this lane (the citation now reads `dispextern.h 1392 `struct frame *f``), and `grep -c emacs-module` on the findings is 0 |
| `check-declarations.py` | `19 declaration sites (2 forbidden, 17 allowed literals), 2 failures` — both findings name `femacs/src/xmenu.rs` (another lane's file: a `recital` and a `title`) | no finding |
| `check-mirror-dupes.py` | `17 duplicate declaration(s), 17 ratchet row(s), 0 failure(s)` | no finding |
| `check-dead-file-coverage.py` | `122 ledger(s) bound (a C range table), 0 failure(s)` | this ledger is bound to `src/emacs-module.c`: `64 range row(s) cover src/emacs-module.c (1798 lines; 66 blank line(s) between blocks need no row)`, no failure |
| `check-ledger-ticks.py` | `234 ledger(s) bound …, 0 failure(s), …, 356 row(s) with no function tick, 0 row(s) without a ledger` | no finding (this ledger has no `- [x]` ticks) |

This lane's command budget: `build-tu` **3** (two readings and one failed run before the `DEFUN!`
invocation's missing semicolon was found), `probe` **3** (Evidence ③'s two runs and ②'s reading),
`cargo test`/`gate-fast`/`xtask --all`/`check-all-units.sh` not run, git untouched.  `df -h /` over
the lane's runs: `1.8Ti` size, from `4.2Gi` available (`12Gi` used, `74%`) at the first reading to
`1.7Gi` (`12Gi` used, `88%`) at the last — the fall is the other lanes' builds running while this
lane measured, not this lane's: its own artifacts are the 336-byte object and the build cache under
`/tmp/emod-lane`, which is deleted at the end (`ls -d /tmp/emod-lane` is empty).

## Deviations

1. **`emacs_value` is `*mut c_void` here** (the header mirror's reading: `include/emacs-module.in.rs`
   97-102 spells C 74-76's `typedef struct emacs_value_tag *emacs_value` that way because C 125's tag
   is *this* unit's record, P-26/P-72 ③).  Every `v->v` is
   `(*v.cast::<emacs_value_tag> ()).v` and every `&…->value` an `… as emacs_value` cast
   (`module_global_reference_p` C 417, `module_make_global_ref` C 445/C 458, `lisp_to_value`,
   `allocate_emacs_value`, `value_to_lisp`).
2. **The non-local-exit macros' call sites pass the names C's macro declares.**  A Rust
   `macro_rules!` body cannot name a caller's local, so `MODULE_HANDLE_NONLOCAL_EXIT`,
   `MODULE_FUNCTION_BEGIN` and `MODULE_INTERNAL_CLEANUP` take `env`/`internal_cleanup` as
   parameters — the `USE_SAFE_ALLOCA! (sa_avail, sa_count)` shape PORTING.md P-05 already records —
   and the no-value form (`MODULE_FUNCTION_BEGIN ()`, C 468/C 643/C 660/C 893/C 914/C 933) is the
   macros' second arm (`return ;` for a void function).  C's `do { } while (false)` tail is dropped
   (a semicolon-forcing wrapper with no Rust counterpart; recorded in `macro-body-notes.tsv`).
3. **C 183-206's forward declarations are dropped** (PORTING.md 0.1-12: Rust has no prototypes and
   its items are order-independent), so `module_assert_thread`/`module_assert_runtime`/
   `module_assert_env`/`module_abort`/`module_out_of_memory` call their definitions textually later
   in the file.
4. **`struct Lisp_Module_Function` is defined in the `lisp_module_function_def` module** (the
   `make_string_from_utf8_def` device, `femacs/src/coding.rs` 7372-7383): `include/lisp.rs`
   2350-2355 already spells the type as a crate-root incomplete-type record (lisp.h only uses it
   through pointers), so a second crate-root item of that name would be `E0428`.  The unit's own
   references spell the path; `XMODULE_FUNCTION`'s crate-root pointer is cast at the use site.
   `## Owed` 1 is the move-back that removes both the path and the ten `extract-abi` rows.
5. **C's two function-pointer casts are `std::mem::transmute`**: C 1242's
   `(emacs_init_function) dynlib_func (…)` and C 1353's `(module_funcptr) function->subr` — Rust has
   no `as` cast between `Option<fn>` types, and the tree already spells this cast that way
   (`femacs/src/comp.rs` 137).
6. **The C conditions the port writes out** (each one a token row in `macro-body-notes.tsv`):
   C 968's `= 0` is `core::ptr::null_mut ()`; C 1062-1063's `min (SIZE_MAX, PTRDIFF_MAX)` converts
   both operands to `size_t` (the port's `min` is the `isize` one) so the comparison is written out;
   `NULL` is `std::ptr::null_mut ()`, or `None` where C's `NULL` is a null `emacs_finalizer`;
   `MODULE_FUNCTION_BEGIN (0)` in the `double`-returning `module_extract_float` is `0.0`
   (PORTING.md 1.1); `(EMACS_UINT) -x` is `(x as EMACS_UINT).wrapping_neg ()` (1.1); `static_assert`
   is `const _: () = core::assert! (…)` (status 3.432); C 1093-1101/1116/1183's `enum { … }`
   constant blocks are `const`s; `sizeof *x` is `std::mem::size_of::<…> ()`; `%"pD"d` is `%td`
   (lisp.h 150: `pD` is `"t"`); `va_list args; va_start; va_end;` are the `...` parameter and
   `let ap: VaList<'_> = args;` (P-04).
7. **C 1481's `ok:` label and C 1470/C 1474's `goto ok` are PORTING.md 2.2's forward-jump shape**,
   the `'ok: loop { … break 'ok; }` block around C 1464-1479.
8. **`switch` → `match`**: `module_signal_or_throw`'s two `case`s become arms (C's fall-through from
   `case emacs_funcall_exit_signal:` to `case …_throw:` has no counterpart because `xsignal` never
   returns) with the `_` arm C's `default: eassume (false)` needs, and `module_handle_nonlocal_exit`'s
   two `case`s get the `_ => {}` arm an `int`-typed enum requires.
9. **C locals declared without an initializer get C's zero value** (PORTING.md 0.1-13):
   `Lisp_Object result;` (C 620), `Lisp_Object *newargs;` (C 690), `emacs_value *args;` (C 1302),
   `hash_hash_t hashcode;` (C 431), `intmax_t i;` (C 747), `struct timespec value;`,
   `emacs_value value;`, `struct module_global_reference *ref;`-style locals.  C's four
   `struct` locals in `module-load`/`funcall_module` (C 1246-1249/C 1295-1296) are
   `unsafe { std::mem::zeroed () }` (the `femacs/src/android.rs` 458 reading), since Rust has no
   partial initialization.
10. **The helpers the vtable stores are `unsafe extern "C" fn`** (C's own functions have C linkage,
    and `initialize_environment`/`module_get_environment` store their addresses); the helpers whose
    address is never taken stay plain `unsafe fn`.
11. **`specbind_tag` and its two `HAVE_MODULES` members are declared in the unit** (C 3562-3563;
    `## Owed` 1): `include/lisp.rs` 1801-1822 carries the enum's no-modules arm only, so its
    `SPECPDL_LET`/`SPECPDL_LET_LOCAL`/`SPECPDL_LET_DEFAULT` are 9/10/11 while C's are 11/12/13 with
    modules; the unit's two members keep C's with-modules values (9/10).
12. **C 1708's `&frame->objects[i] == value`** is the same address computed with
    `objects.as_ptr ().offset (i)` (no intermediate reference, which `*const` frames cannot take).
13. **C 1650-1652's `finalize_runtime_unwind` body is one comment** in C and stays one comment here
    (`## Owed` 4).

## Owed

1. **The mirror moves the placeholder out of `include/lisp.rs`.**  Ten `extract-abi.py --check` rows
   (`missing field Lisp_Module_Function.<each of the nine>` and the `struct … missing fields` row)
   come from `include/lisp.rs` 2350-2355's crate-root `pub struct Lisp_Module_Function { _private:
   [u8; 0] }`: `rust_inventory` scans the unit's own text first and its `include!` targets after, and
   a later non-empty field list wins, so the placeholder's `['_private']` replaces the unit's nine
   fields (measured: `rust_inventory(...)['structs']['Lisp_Module_Function'] == ['_private']`).
   With the placeholder removed the unit's definition can move to the crate root and Deviations 4's
   path spelling disappears.  The same header owes `enum specbind_tag`'s `HAVE_MODULES` arm
   (`SPECPDL_MODULE_RUNTIME`/`SPECPDL_MODULE_ENVIRONMENT`, lisp.h 3560-3561, and the shifted
   `SPECPDL_LET` family: without the arm the mirror's values are 9/10/11 against C's 11/12/13), which
   the unit now declares for itself (Deviations 11).
2. **Nine declarations and one typedef the mirrors do not carry yet**, each declared at its use site with the
   owner header's own prototype (P-88 ①'s single-consumer rule; the per-unit `mark_object` shape
   `femacs/src/xselect.rs` 3211 already uses).  Each row is `owner header line → name`:
   `lisp.h 4816 → push_handler_nosignal`, `lisp.h 4259 → hash_remove_from_table`,
   `lisp.h 4432 → memory_full`, `lisp.h 4436 → mark_object`, `lisp.h 4892 → make_user_ptr`,
   `lisp.h 4828 → record_unwind_protect_module`, `coding.h 693-694 → encode_string_utf_8`,
   `coding.h 695-697 → decode_string_utf_8`, `process.h 316 → open_channel_for_module`, and the type
   `lisp.h 4889 → module_funcptr` (`typedef void (*module_funcptr) (void);`).  When the
   mirror that owns one adds it, the unit's copy must go (a second identical `extern` declaration in
   the same crate root is `E0428`).  `vfprintf` is the same shape but has a precedent
   (`femacs/src/pdumper.rs` 123-126 declares it in-unit too, `lib/include/stdio.rs` keeps only
   `fprintf`).
3. **The line-count tolerance row** (`line count drifted 11.8%: C 1798 vs Rust 2010`) belongs in
   `docs/ports/line-drift-tolerance.tsv`, which this lane may not write.  The measured drift is the
   P-86 class's normal one (a per-item `#[cfg]` on every item, one Rust statement per C statement,
   and the `unsafe`/`pub`/type spellings).
4. **`stub or empty body in fn finalize_runtime_unwind`**: C's own body (C 1650-1652) is a single
   comment — "No further cleanup is required, as the initial environment is unwound separately." —
   and the check's `empty_in_c` predicate does not classify a comment-only C body as empty, so the
   port's identically comment-only body reads as a stub.  Filling it would invent code C does not
   have.
5. **The `windowsnt` arm's arm-flip needs the Windows SDK** (Evidence ④): `text-only` this round,
   with the 98-error reading taken for the record.  The unit's own text of the arm is C 113-114.
6. **The `have_window_system` arm's clean flip needs a GUI backend**, and the two readings that
   would give it are not this lane's to take: with `have_x_windows` the mirror
   `include/systime.rs` 27 asserts `HAVE_X_WINDOWS` (the feature/constant pair `xtask dead-arms`
   flips together, P-51); without it `include/dispextern.rs` 739's `Emacs_Pix_Container` arm
   (`all (have_window_system, have_x_windows, not use_cairo)`) leaves dispextern.rs 796 dangling
   (Evidence ②'s single error).  The `have_window_system`-only combination is not a configuration
   C's own `#ifdef HAVE_WINDOW_SYSTEM` ever meets.
7. **No ABI/host measurement of the `initialize_environment` field assignments**: the judge of those
   forty assignments is the generated `emacs-module.h` the header mirror is built from
   (`docs/ports/include-emacs-module.in.md`), not this machine; no module host was loaded here.

## Not claimed

- No `windowsnt` arm flip (no Windows SDK/mingw on this machine), so that arm's two lines are
  text-only.
- No `have_window_system`-with-backend flip (Owed 6).
- No layout assertion or field-offset measurement of `struct emacs_env_private`,
  `struct emacs_value_frame`/`storage`, `struct emacs_value_tag`, `struct module_global_reference` or
  `struct Lisp_Module_Function` was taken here: the first four are checked by the header mirror's own
  layout asserts where they are visible, and the last one is the mirror placeholder Owed 1 names.
- No link, no functional test, no `cargo test`, no `gate-fast`; the object this unit builds in the
  frozen configuration is the 336-byte empty one (Evidence ①).
