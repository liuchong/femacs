# emacs/src/androidselect.c (1,220 lines)

> **P-86's second unit: the frozen configuration compiles nothing here.** `configure.ac` 2597 puts
> `androidmenu.o` in `ANDROID_OBJ` under `AC_DEFINE([HAVE_ANDROID])` (2602), and 2647-2648 adds
> `androidselect.o` to that list only in the really-Android branch (`REALLY_ANDROID`, 2654), so
> `docs/baseline/frozen-build-sources.txt` lists no `androidselect.o` for this target and
> `extract-abi.py`'s checklist for the file is empty - its checkbox sections cannot be coverage
> evidence.  What follows is P-86 ②'s substitute: the **C line-range table**, the `## Evidence`
> readings, and the `## Deviations`.  Port: `src/androidselect.rs` (1,312 lines), both arms
> of every `#if` carried.  This **overturns the decision this ledger used to record** ("the file is
> not in the frozen build, so the port is the empty unit plus a note"): P-86 ① rules that a
> three-line header comment is *not started*, not *done*.  The old note's measurements are still
> true and are still where the *file gate* comes from; what changed is the conclusion drawn from
> them - see the last section.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`configure.ac` 2647-2648, the `XCONFIGURE = android` branch) | `all (feature = "have_android", feature = "not_android_stubify")` (on every item and on C 21-34's include graph) | a *real* Android configuration |
| C 432 `#if __ANDROID_API__ <= 11` … C 434 (the `static` pointer) | `feature = "android_api_le_11"` (the derived condition, the xterm.rs `have_xcb_shape_input_rects` shape) | an Android configuration whose NDK min SDK is <= 11 |
| C 447 `#if __ANDROID_API__ <= 11` … C 477 `#endif` (the first arm, over `jniGetFDFromFileDescriptor`) | `feature = "android_api_le_11"` | the same |
| C 478 `#if __ANDROID_API__ <= 30` … C 500 `#endif` (the second arm, over `getParcelFileDescriptor`) | `feature = "android_api_le_30"` | an Android configuration whose NDK min SDK is <= 30 |
| C 501-524 (the third arm, over `AFileDescriptor_getFd`) | `not (feature = "android_api_le_30")` | min SDK > 30 |

`not_android_stubify` is the feature the tree already registered for C's
`HAVE_ANDROID && !defined ANDROID_STUBIFY` (`docs/baseline/feature-coverage.tsv` line 640, first
used by `alloc.rs` 6428); `configure.ac` 2605-2606 defines `ANDROID_STUBIFY` in the branch that
does *not* take 2647-2648, which is why this file's gate is the pair and not `have_android` alone
(`androidmenu.o` is in 2597's list, i.e. in both Android branches, so androidmenu.rs's file gate is
`have_android` with the pair only on its inner arm).

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's opening declaration block | dropped (P-86 ①; the port opens with the `// femacs: port of …` line) |
| 20 | `#include <config.h>` | ungated (every unit includes it) |
| 21-24 | `<assert.h>`, `<minmax.h>`, `<unistd.h>`, `<dlfcn.h>` | file gate (`assert.h` names nothing here; `<dlfcn.h>` has no mirror, so its two entries and two `RTLD_*` are declared in the unit) |
| 26-27 | `<boot-time.h>`, `<sys/types.h>` | file gate |
| 29-34 | `lisp.h`, `blockinput.h`, `coding.h`, `android.h`, `androidterm.h`, `termhooks.h` | file gate |
| 36-45 | the selection-support note | file gate (carried as text) |
| 49-61 | `struct android_emacs_clipboard` | file gate |
| 63-64 | `clipboard_class` | file gate |
| 66-67 | `clipboard` | file gate |
| 71-116 | `android_init_emacs_clipboard` (C 90-95 the `FIND_METHOD` macro, C 97-105 its six uses, C 107-113 `makeClipboard`, C 115-116 the `#undef` and `}`) | file gate |
| 121-147 | `DEFUN ("android-clipboard-owner-p", …)` | file gate |
| 149-175 | `DEFUN ("android-set-clipboard", …)` | file gate |
| 177-219 | `DEFUN ("android-get-clipboard", …)` | file gate |
| 221-239 | `DEFUN ("android-clipboard-exists-p", …)` | file gate |
| 241-271 | `DEFUN ("android-browse-url-internal", …)` | file gate |
| 275-339 | the MIME note and `DEFUN ("android-get-clipboard-targets", …)` (C 302-303 the `goto fail`, C 336-339 the `fail:` label) | file gate |
| 343-352 | `struct android_asset_file_descriptor` | file gate |
| 354-355 | `asset_fd_class` | file gate |
| 357-395 | `android_init_asset_file_descriptor` (C 380-385 the macro, C 387-393 its five uses, C 394-395 the `#undef` and `}`) | file gate |
| 397-403 | `android_xfree_inside` | file gate |
| 405-418 | `close_asset_fd` | file gate |
| 420-528 | `extract_fd_offsets` (C 432-434 the gated `static`, C 447-477 the `android_api_le_11` arm, C 478-500 the `android_api_le_30` arm, C 501-524 the third arm, C 526-528 `failure:`) | file gate, and the two derived features inside the chain |
| 530-638 | `DEFUN ("android-get-clipboard-data", …)` (C 567-568 the `goto fail`, C 635-638 the `fail:` label) | file gate |
| 642-652 | the desktop-notification note and `struct android_emacs_desktop_notification` | file gate |
| 654-655 | `notification_class` | file gate |
| 657-658 | `notification_table` | file gate |
| 660-695 | `android_init_emacs_desktop_notification` (C 682-687 the macro, C 689-692 its two uses, C 694-695 the `#undef` and `}`) | file gate |
| 697-730 | `android_locate_icon` | file gate |
| 732-929 | `android_notifications_notify_1` (C 742-745 the `identifier` array, C 752 the `static` counter) | file gate |
| 931-1082 | `DEFUN ("android-notifications-notify", …)` (0, MANY) | file gate |
| 1084-1108 | `android_notification_deleted` | file gate |
| 1110-1156 | `android_notification_action` | file gate |
| 1158-1187 | `init_androidselect` | file gate |
| 1189-1220 | `syms_of_androidselect` | file gate |

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/androidselect-p86b cargo run -q -p xtask -- build-tu
src/androidselect.c` → `built src/androidselect.rs -> ../build/femacs/androidselect.o`, **exit 0**,
and `build/femacs/androidselect.o` is **336 bytes** - an empty object, the same reading
`docs/ports/androidmenu.md`'s Evidence ① records for its unit.  Read twice: on the first complete
revision and on the final one, with the same result.  That is the expected reading and not a proof
of the new text - every item sits behind the file gate, so under the frozen feature set rustc
compiles only `include/config.rs` and the `#![allow]` attribute.

**② The flip of the arm (P-86 ③⑴; the file gate plus the two sub-gates, P-86 ⑥).**
`CARGO_TARGET_DIR=/tmp/androidselect-p86b cargo run -q -p xtask -- probe src/androidselect.rs
/tmp/androidselect-p86b-flip3.o --crate-type lib --emit obj -C panic=abort
--cfg 'feature="have_android"' --cfg 'feature="not_android_stubify"'`
→ **exit 1, 5 `error` lines (4 errors + the abort line)**.  Read on the revision *before* the last
three text changes (the `isize` arity, the two `ckd_add` sites, one `i` initialiser), so its own
error is listed with the fix that followed it; the final revision's own reading is ②a.

| file | line | error |
| --- | --- | --- |
| an included mirror, `src/include/window.rs` | 346 | `E0425 cannot find function … BUFFERP` (window.h 605) |
| an included mirror, `src/include/window.rs` | 371 | `E0425 … BUFFERP` (window.h 605) |
| an included mirror, `src/include/window.rs` | 372 | `E0425 cannot find function buffer_local_value` (window.h 634-635) |
| an included mirror, `src/include/window.rs` | 441 | `E0425 … BUFFERP` (window.h 1199) |
| this unit's own text (fixed) | `Fandroid_notifications_notify_1` | `E0381 used binding i isn't initialized` - C 756's `for (i = 0; …)` initialiser; the port now declares the counter initialised |

`window.rs` is in this graph the way it is in C's: `frame.h` 23 includes `window.h`, and
`androidterm.h` 24 includes `frame.h`.  Its three sites are *macros* in C, which the preprocessor
expands only at use sites - a translation unit that includes window.h without using them never
needs `BUFFERP` (buffer.h 757 defines it, and window.h includes neither buffer.h nor anything that
does).  The port spells them as functions, whose bodies Rust does type-check, and `window.rs`'s own
include graph does not reach `include/buffer.rs` (767 defines `BUFFERP`, 1337 declares
`buffer_local_value`).  That is the P-72 rule-4 / P-87 class gap and it is owed by the integrator
(see below).  Because the graph breaks in name resolution, this flip **cannot** be read as "the
unit has 0 errors" (P-86 ⑦'s criterion: the include graph breaking first makes "0 in our text" an
empty reading) - the reading that does compile this unit's own body is ②a.

**②a Arm-reach pre-flight: the unit's text against the real mirrors (P-86 ⑦'s different reading,
on the final revision).**
The materialization of the final revision (`build/femacs/units/androidselect/_root.rs`, produced by
①'s `build-tu` under that revision) with **two stub items prepended after the `lisp.rs` include** -
`pub fn BUFFERP (a: Lisp_Object) -> bool` and
`pub unsafe fn buffer_local_value (variable: Lisp_Object, buffer: Lisp_Object) -> Lisp_Object`, the
two names ② is blocked on and nothing else - compiled with the same two `--cfg`s:

    rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type lib --emit obj -C panic=abort \
      --cfg 'feature="have_android"' --cfg 'feature="not_android_stubify"' \
      -o /tmp/androidselect-p86b/harness/arm.o /tmp/androidselect-p86b/harness/_root.rs
    → exit 0, 0 error lines, 122,688-byte object

This is what compiled the unit's Android body: all three arms of C 447-524's chain (with C 432-434's
gated `static`), the seven functions and eight subrs `--check` compares, the three `goto` targets and
the three `dlsym`/`RTLD_*` arms.  It is **not** the same reading as ② and is reported as its own
line: the stubs stand in for two names C's window.h never had expanded, and neither appears anywhere
in this unit's text (measured with `rg` over both files) - so no body of this unit is masked by
them.  The harness root's own lines are this unit's text line for line: compared with the file, the
only differences are the `include!` lines xtask rewrites to absolute paths and its two
`[xtask dedupe]` comments (measured by diffing the two texts after the stubs are removed), so the
reading is the final revision's.

**③ Inventory.**
`python3 tools/extract-abi.py --check emacs/src/androidselect.c src/androidselect.rs` →

    - function extract_fd_offsets does not mention goto
    - subr Fandroid_get_clipboard_targets does not mention goto
    - subr Fandroid_get_clipboard_data does not mention goto
    - line count drifted 7.5%: C 1220 vs Rust 1312
    FAIL emacs/src/androidselect.c -> src/androidselect.rs

(plus one informational line: `FIND_METHOD` has several C definitions, so its body is not
compared).  The file's checklist is empty (P-86 ⑩), so `--check` compares the eight subr
docstrings, the function bodies' tokens and the line counts; the three `goto` tokens and the line
drift are what it reports, and both are owed (below).  The docstring comparison and the
`ckd_*` token-aliases family, `--check`'s other two checks, pass.

**④ Other gates (repo root, final revision).**

    python3 tools/check-dead-file-coverage.py
      → info docs/ports/androidselect.md: 34 range row(s) cover src/androidselect.c (1220 lines;
        45 blank line(s) between blocks need no row)
      → 6 ledger(s) bound (a C range table), 0 failure(s)
    python3 tools/check-port-lines.py
      → stale src/androidselect.c: no longer under the threshold
      → 92 file(s) under ten percent, 243,405 reference line(s); 95 ratchet row(s), 0 new, 3 stale
    python3 tools/check-citations.py
      → 675 file(s) scanned, 0 drift, 0 range, 457 external, 33 allowed, 0 stale
    python3 tools/check-declarations.py
      → 13 declaration sites (0 forbidden, 13 allowed literals), 0 failures

The `check-port-lines.py` row is this file's own ratchet row leaving the table (P-86 ④'s
`--write-pending`: the row is stale and the integrator's re-run drops it).

## Deviations

- **The file gate is the pair, not `have_android` alone** - `configure.ac` 2647-2648 adds
  `androidselect.o` only in the really-Android branch while 2597's list (`androidmenu.o`) is in
  both.  The three configuration cases this creates are covered because the port keeps *all three*
  arms of its only `#if` chain (P-86 ①): a build that switches an arm off still has the other two.
- **The two derived feature names are not in `xtask/Cargo.toml`'s `[features]` table.**
  `android_api_le_11` and `android_api_le_30` stand for C's `#if __ANDROID_API__ <= 11` and `<= 30`;
  the table carries no `__ANDROID_API__`-conditioned name today (measured: no row's comment names
  it, and `rg 'api' xtask/Cargo.toml` finds none).  Owed by the integrator - a port lane does
  not add feature names to that manifest.
- **Rust has no `#[cfg]` on an `else` clause, so each `#if` is a compile-time operand of its arm's
  own runtime test** (`cfg! (feature = "android_api_le_11") && android_get_current_api_level () <=
  11`).  That is a *faithful* reading, not a weakening: `android_get_current_api_level () >=
  __ANDROID_API__` is the header's own invariant (android.h 152-158), so with `__ANDROID_API__ >
  11` the first arm's test is always false and the kept ladder is C's `if (<= 30) … else {…}`, and
  with `> 30` both kept tests are always false and it is C's single third arm.  The three cases are
  therefore exactly the three preprocessor expansions.  Gating the arm *statements* with `#[cfg]`
  could not do this: `#[cfg]` removes the whole `if/else` statement, not one arm of it.
- **C 432-434's `static int (*jniGetFDFromFileDescriptor) (JNIEnv *, jobject)` is declared
  unconditionally** (the arm that reads it is carried, so its declaration must compile); C's
  `static` is zeroed and the port's is the `Option<unsafe extern "C" fn …>` the function-pointer
  field shape uses.  The same for C 435's `AFileDescriptor_getFd`, which has no `#if` in C.
- **`<dlfcn.h>` (C 24) has no mirror in this tree** (`lib/include/dlfcn.rs` does not exist;
  `src/dynlib.rs` 17-30 declares the same entries in-unit for its own `<dlfcn.h>`, the
  "getgroups.c shape").  The unit declares `dlopen`, `dlsym`, `RTLD_LAZY` and `RTLD_GLOBAL` at the
  use site with the **NDK's** values, measured on this machine's
  `~/Library/Android/sdk/ndk/29.0.14206865` (`clang --target=aarch64-linux-android24 -dM -E
  -include dlfcn.h`: `#define RTLD_LAZY 0x00001`, `#define RTLD_GLOBAL 0x00100`).  `dynlib.rs`'s 1
  and 8 are the *frozen* target's values for `src/dynlib.c`, which never compiles in an Android
  configuration and vice versa; both are the platform's `<dlfcn.h>` values, so the divergence is
  the platform's, not the port's.
- **`<stdio.h>` names the mirror does not carry**: `sscanf` (C 1104, C 1128) and `BUFSIZ`
  (C 601-629).  `stdio.rs` has `sprintf` (C 836) and not these two; the unit declares `sscanf`
  in-unit (the `xfaces.rs` 540 / `sysdep.rs` 430 shape) and `BUFSIZ` as the measured bionic value
  (`#define BUFSIZ 1024`, the same clang invocation).
- **termhooks.h 353-355's `NOTIFICATION_EVENT` has no home**: `include/termhooks.rs` carries
  `enum event_kind` up to `TOOLKIT_THEME_CHANGED_EVENT` (36) and neither conditional member, so the
  unit defines it with the value C's chain gives when `HAVE_ANDROID` is defined and `HAVE_HAIKU`
  is not (37; the HAIKU member `NOTIFICATION_CLICKED_EVENT` would take 37 first if that macro were
  defined too).
- **`FUNCTIONP` (lisp.h 4799) is in neither `lisp.rs` nor `globals.rs`**; the unit declares it at
  the use site - the `editfns.rs` 141 shape, which cites the same line.
- **`<intprops.h>` at the use site**: `TYPE_MAXIMUM`/`TYPE_MINIMUM` (`macro_rules!` over
  `<$t>::MAX`/`MIN`, the androidmenu.rs pair) and `INT_STRLEN_BOUND` (C 742-745; the body is
  gnulib's `((bits - signed) * 146 + 484) / 485 + signed` over `size_of::<$t>`, the
  `lib/strftime.rs` 542 instantiation, which also keeps the token in the body).
- **`ckd_add` (C 619-620, C 811) is P-15's spelled-out pair at both sites**, not a local
  `macro_rules!`: gnulib's `<stdckdint.h>` (lisp.h 27) has the frozen mirror's *dead* arm and a
  marker const, the ported units spell the call as `overflowing_add` plus the store
  (`lib/timespec-add.rs` 36-38, `lib/mktime.rs` 36), and `token-aliases.tsv`'s family rule reads
  C's numeric argument out of a call of the family's spellings - a `macro_rules!` invocation
  (`ckd_add! (…)`) is not such a call, so the second site's `1` would have gone uncompared.
- **`ANDROID_DELETE_LOCAL_REF`** (android.h 340-342): an excluded macro whose premise
  ("`JNINativeInterface` has no mirror") `include/jni.rs` superseded; the unit defines the expansion
  under C's own name (P-86 ⑫, the androidmenu.rs shape).
- **`input_event_bits!`**: termhooks.h 373-412's `kind : 16` is the mirror's `input_event::bits`
  unit (P-80), so the two `ie->kind = NOTIFICATION_EVENT` stores are member-named macro calls - the
  keyboard.rs 4080 / xterm.rs 1367 body-identical per-unit macro.
- **The MANY subr's parameters are `(nargs: isize, args: *mut Lisp_Object)`** - the shape
  `DEFUN_ARGS_MANY` declares and `callint.rs`'s `Ffuncall_interactively` writes (C's `ptrdiff_t`
  is the same type, but `--check` compares the spelled name).
- **Type-level conversions C writes implicitly**: `title as jobject` at C 856 and 861 (C's
  `Lisp_Object` argument to a `jobject` parameter - the tree's raw pointers are distinct types),
  `as jlong`/`as isize`/`as usize`/`as ptrdiff_t`/`as jsize` on C 588-629's size and offset
  arithmetic (`jlong` and `ptrdiff_t` are both 64-bit here but different Rust types), pointer casts
  at C 399-403 and C 408-413 (`*(char **) ptr`, `void *` to `jobject *`), `as *mut c_char` where C
  passes a `char *` to a `void *` parameter, and the two `std::mem::transmute (dlsym (…))` at
  C 461 and C 511 (C converts `void *` to a function pointer implicitly; the `dynlib.rs` 82 shape).
- **`goto` and labels (PORTING.md 2.2)**: C 302-303's `goto fail`/C 336-339's `fail:` and
  C 567-568's `goto fail`/C 635-638's `fail:` become labeled blocks; C 462/464/508/512's four
  `goto failure`/C 526-528's `failure:` become one labeled block.  The `goto` token therefore does
  not survive in any of the three bodies - the rows `extract-abi --check` asks for are owed.
- **Locals C leaves uninitialised and the port zeroes** (PORTING.md 1.5): `fd`, `offset` and
  `length` (C 535-536, written through their addresses at C 576), `rc` in
  `Fandroid_get_clipboard_data` (C 536; the loop C writes as `while (true)`), and `id` in
  `android_notifications_notify_1` (C 753) and in the two notification hooks (C 1093, C 1121,
  written through their addresses by `integer_to_intmax`/`sscanf`).  C 845's `boot_time.tv_sec = 0;`
  keeps its own statement while the local gets `std::mem::zeroed ()`, because C leaves the rest of
  the struct to `get_boot_time` and Rust would otherwise reject the borrow of a partly-initialised
  local.
- **C's `specpdl_ref ref` is written `r#ref`** - `ref` is a Rust keyword and the tree's raw
  identifier spelling (`r#type` and friends) covers it.
- **Two `for` loops and one chained assignment**: C 289's and C 756's `for (i = 0; …)` are the
  declared counter plus `while`, with the increment C writes in the header at the body's end (the
  `dispnew.rs` 569-572 shape), C 305's `++i` likewise; C 1014-1015's chained
  `title = body = … = Qnil;` is one statement per target on each line (the androidmenu.rs
  `prefix = Qnil (); entry = Qnil ();` shape), because Rust's assignment yields `()`.
- **Line count.** C 1,220 vs Rust 1,312 (7.5%): the per-item file-gate `#[cfg]`s, the `cfg!`
  operands, and the in-unit declarations the mirrors do not carry yet.  A
  `docs/ports/line-drift-tolerance.tsv` row (stem `androidselect`, basis `total`) is owed.

**Owed by the integrator (nothing here is claimed as done)**

1. `docs/ports/macro-body-notes.tsv`: three rows - `extract_fd_offsets`,
   `android_get_clipboard_targets` and `android_get_clipboard_data` → `goto` (PORTING.md 2.2's
   labeled-block spelling; the `androidmenu.rs` `android_menu_show` row is the precedent).
2. `docs/ports/line-drift-tolerance.tsv`: one row for `androidselect`, with the reason above.
3. `xtask/Cargo.toml`'s `[features]` table: `android_api_le_11` and `android_api_le_30`
   (comments: the derived conditions `__ANDROID_API__ <= 11` / `<= 30`, androidselect.c 432/447 and
   478 - note the second row's `not (…)` spelling in this port).
4. **The mirror gap that blocks the flip reading**: `include/window.rs` names `BUFFERP`
   (buffer.h 757, the port's `include/buffer.rs` 767) and `buffer_local_value` (buffer.h 1264,
   `buffer.rs` 1337) without reaching `buffer.rs`, so *no* unit whose graph includes window.h - this
   one, and any other that includes `frame.h` without `keyboard.h` (whose graph does reach
   `buffer.rs`, which is why androidmenu.rs's flip passes) - compiles under `have_android`.  Either
   window.rs adds the `include!("buffer.rs")` P-72 rule 4 asks for, or the two names get a home
   every includer of lisp.h has.  This is the androidmenu.rs 3.377 situation and it needs a mirror
   round plus `tools/check-all-units.sh` (P-87), which is why it is not touched here.
5. The three in-unit declarations that belong in mirrors once those mirrors carry them:
   `FUNCTIONP` (lisp.h 4799 → `lisp.rs`), `NOTIFICATION_EVENT` (termhooks.h 353-355 →
   `termhooks.rs`, whose `enum event_kind` is missing both conditional members), and the
   `<dlfcn.h>` pair plus the `<stdio.h>` pair (`lib/include/dlfcn.rs` does not exist; `stdio.rs`
   has `sprintf` but not `sscanf`/`BUFSIZ`).
6. `tools/check-port-lines.py --write-pending`: this file's ratchet row is now stale (92 files under
   ten percent against the placeholder's 11 lines) and the re-run drops it.

## The decision this file overturns

The previous revision of this ledger recorded: *"**This file is not compiled by the frozen
configuration** … the port is an empty unit plus the note that names the gate.  Nothing in this file
is required here."*  That conclusion is wrong, and P-86 ① replaced it: the frozen configuration not
compiling a file says which *gate* its text carries, not whether the text is ported.  The
replacement is this ledger's `## C line ranges` table (every non-blank line of C 1-1220, checked
mechanically by `tools/check-dead-file-coverage.py`), the `## Evidence` readings (the frozen build,
the blocked flip and the arm-reach pre-flight that compiles the body), and the `## Deviations` list
above - including the six names that still have no home (items 4 and 5 of the owed list).
