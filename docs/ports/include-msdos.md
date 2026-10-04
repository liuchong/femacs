# emacs/src/msdos.h (156 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 21
- [x] ButtonReleaseMask  (line 131, body b6589fc6ab0d)
- [x] DisplayHeight  (line 124, body 73d705742c4f)
- [x] DisplayWidth  (line 123, body 8ff5eb559d6f)
- [x] EINPROGRESS  (line 80, body 601ca99d55f0)
- [x] EMACS_MSDOS_H  (line 20, body da39a3ee5e6b)
- [x] FRAME_DISPLAY_INFO  (line 108, body 8e9c4c5d2957)
- [x] FRAME_FONT  (line 107, body 750e9222a4b6)
- [x] FRAME_X_DISPLAY  (line 98, body eef19c54306d)
- [x] O_CLOEXEC  (line 87, body b6589fc6ab0d)
- [x] PIX_TYPE  (line 99, body 48fd659982bc)
- [x] XDISPLAY  (line 100, body da39a3ee5e6b)
- [x] XFreeGC  (line 120, body cbf9e406f246)
- [x] XGetGeometry  (line 122, body da39a3ee5e6b)
- [x] XM_FAILURE  (line 127, body 7984b0a0e139)
- [x] XM_IA_SELECT  (line 130, body 77de68daecd8)
- [x] XM_NO_SELECT  (line 129, body da4b9237bacc)
- [x] XM_SUCCESS  (line 128, body 356a192b7913)
- [x] XMenuRecompute  (line 126, body cbf9e406f246)
- [x] XMenuSetFreeze  (line 125, body cbf9e406f246)
- [x] tcdrain  (line 28, body 145bc17fd3ec)
- [x] x_destroy_bitmap  (line 121, body da39a3ee5e6b)

## Types and their fields — 1
- [x] x_menu_struct: int count, char** text, struct x_menu_struct** submenu, int* panenumber, int allocated, int panecount, int width, const char** help_text

## File-scope objects — 3
- [x] extern int have_mouse  (line 44)
- [x] extern struct tty_display_info the_only_display_info  (line 104)
- [x] extern struct tty_output the_only_tty_output  (line 105)

## Decisions

- **The whole header is MS-DOS-only, and the frozen target compiles none of it.**  Every C reader
  includes it inside `#ifdef MSDOS` (`src/dired.c` 43-45, `src/sysdep.c` 96-98, `src/keyboard.c`
  61-63, `src/atimer.c` 36-38, `src/fileio.c` 78-80, `src/frame.c` 47-49, `src/lread.c` 48-50,
  `src/callproc.c` 82-84, `src/dosfns.c` 44, `src/msdos.c` 68), and `MSDOS` is `undef` in the
  frozen macro table, so `femacs/src/msdos.rs` (the empty unit) records the exclusion and the
  mirror's declarations are deferred to that target with their definitions.  The `#ifndef
  HAVE_X_WINDOWS` arm of C 90-154 is nevertheless *live text for this header* (C's guard is about
  X, not about MS-DOS), which is why the dummy types and the XMenu façade are carried rather than
  treated as a dead arm.
- **The `O_CLOEXEC` cond-eval disagreement is recorded, not resolved.**  `python3
  tools/check-cond-eval.py src/msdos.h` reports `4 conditions, 0 unknown, 1 disagreement [NOT
  VERIFIED]` - C 84's `#ifdef O_CLOEXEC` is False to clang and True to the evaluator.  C 84-86 only
  `#undef`s a macro the system may have defined first, and C 87 then defines the name to 0, so
  **both arms end with the same text** and the mirror is unaffected; the file stays NOT VERIFIED
  (the evaluator's conservative "both arms live" reading is the one the extractor uses, exactly as
  `docs/ports/include-assert.in.md` records for its `#pragma` condition, status 3.157).
- **Three classes of names carry `#[cfg (feature = "msdos")]`, because the port's other mirrors
  already declare them.**  Rust holds one declaration per name in a crate, so the MS-DOS side is
  marked with the feature that names the only configuration that includes this header (the
  `lib/include/errno.in.rs` 95-99 `feature = "windows"` shape, and the reason `src/vm-limit.rs` 95
  already spells `feature = "msdos"`):
  1. **value overrides** - C 80's `EINPROGRESS 112` (the port's `<errno.h>` mirror already owns the
     name: `lib/include/errno.rs` 92, value 36) and C 87's `O_CLOEXEC 0` (`lib/include/fcntl.in.rs`
     118, the frozen target's `0x1000000`; C's C 81-83 comment says why the MS-DOS build overrides
     it - gnulib's `O_NOINHERIT` value "gets in the way when we need to redirect standard handles
     for subprocesses using temporary files created by mkostemp").
  2. **prototypes the MS-DOS build defines itself** - C 57-67 and C 71's `readlink`,
     `readlinkat`, `fstatat`, `unsetenv`, `faccessat`, `openat`, `fchmodat`, `futimens`,
     `utimensat`, `pthread_sigmask`.  The port already declares every one of them in its system
     mirrors (`lib/include/unistd.in.rs` 39-41, `lib/include/sys_stat.in.rs` 181-196,
     `lib/include/fcntl.in.rs` 287, `lib/include/signal.rs` 157), so a *second*, unmasked
     declaration would make any crate that includes both unbuildable.
  3. **the five dummy X typedefs** of C 92-96 - `Display` and `Window` are declared by the port's
     other platform mirrors (`include/w32gui.rs` 57-58, `include/nsgui.rs` 60), and C's own
     `#ifndef HAVE_X_WINDOWS` guard is what keeps them apart from an X build.  C 98 and C 100
     (`FRAME_X_DISPLAY`, `PIX_TYPE`, `XDISPLAY`) are *not* masked: nothing else declares them.
- **C 57-59 is the file's one dead arm.**  `__DJGPP__ == 2 && __DJGPP_MINOR__ < 4` is false in the
  frozen macro table (`__DJGPP__` undefined), so C 58-59's `readlink` prototype is carried under
  `#[cfg (feature = "djgpp_2_3")]` - the feature names the non-frozen side (PORTING.md P-51); its
  parameter names are `lib/include/unistd.in.rs` 39's, because C's own prototype leaves them out.
- **P-72/P-55: the records this header does not define.**  `struct tty_display_info`/
  `struct tty_output` are needed *by value* (C 104-105), and C's units reach termchar.h through
  their own includes (`src/msdos.c` 72, `src/dosfns.c` 38), so the mirror includes
  `termchar.rs`; `<sys/types.h>` (C 52) is included for `mode_t`.  `struct terminal` (C 25, C 42),
  `struct frame` (C 50, C 113, C 118) and `struct window` (C 114) are forward declarations of
  records this header does not include, so they get no items of their own and every pointer to
  them is `*mut c_void` with the C text in a `/// C:` line (the `haikuterm.rs`/`include/w32inevt.rs`
  readings); the same degradation covers `struct stat *`, `const struct timespec[2]` and
  `sigset_t *` in the C 60-67 block.  `Lisp_Object` stays named (PORTING.md 0.1-9).  C's `ssize_t`
  (C 60) is written `c_long`: `lib/include/unistd.in.rs` owns the alias and this header does not
  include `<unistd.h>`.
- **The macros keep C's shapes.**  `tcdrain` (C 28) is a nullary-argument `macro_rules!` over
  `_dos_commit`; C 120-126's no-op X entries become `macro_rules!` bodies that are the unit
  expression (C's `(void)` and empty bodies, C 120/121/122/125/126) - an *empty* Rust body would
  be reported ("macro X has an empty body"), and the two size accessors keep C's `SELECTED_FRAME ()`
  name (`lisp.h`'s macro) and read `text_cols`/`text_lines` with Rust field syntax; `PIX_TYPE`
  (C 99) is a type macro and so a nullary `macro_rules!` (the `haikugui.rs` 63 spelling);
  `XDISPLAY` (C 100) has no body at all, so it is the constant that says "defined" (the
  `nt/inc/include/ms-w32.rs` 147 `_WINSOCK_H` spelling).  `FRAME_FONT` (C 107) reads
  `output_data.tty.font` because Rust has no `->`.
- **Parameter names the header omits come from the definitions**: `src/msdos.c` 165-4312 (the
  terminal, file-name, mouse, keyboard, child and POSIX-call definitions, e.g. C 4095-4096's
  `fstatat (fd, name, st, flags)`), `src/xfns.c` 1683 (`x_set_menu_bar_lines`),
  `src/w32proc.c` 293 (`pthread_sigmask`), `src/w16select.c` 675 (`syms_of_win16select`).
- **`--check` asks for less than fidelity does**: the 25 entry points of C 30-50, C 69-76 and
  C 145-152 are declarations with no checklist row (only the three objects, the one record and the
  21 macros are compared), and they are carried because they are the header's live text.

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/msdos.h femacs/src/include/msdos.rs`
  -> `OK emacs/src/msdos.h -> femacs/src/include/msdos.rs` (no `- ` problem lines) - accepted on
  the first attempt, including the no-op macros of C 120-126 whose C bodies are empty or `(void)`.
- `python3 tools/check-cond-eval.py src/msdos.h` -> `src/msdos.h: 4 conditions, 0 unknown,
  1 disagreements [NOT VERIFIED]` and `DISAGREE src/msdos.h:84 `ifdef O_CLOEXEC` — clang False,
  evaluator True`; the evaluator's own tail line reads
  `4 conditions, 0 unknown to the evaluator, 1 disagreements`.
- `python3 tools/extract-abi.py emacs/src/msdos.h` -> this ledger: 0 subrs, 0 functions defined,
  21 macros, 1 type (`x_menu_struct`, 8 fields), 3 file-scope objects.
- **Compile evidence**: `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-win cargo run -q -p xtask
  -- layout-check` -> `layout-check: 66 of 77 mirror(s) compiled, 11 platform-exclusive skipped
  (android-asset.rs, android.rs, androidgui.rs, androidterm.rs, dispextern.rs, gtkutil.rs,
  haiku_support.rs, haikugui.rs, haikuterm.rs, nsgui.rs, pgtkterm.rs)`, `layout probe ok`,
  `layout-check passed (see build/femacs/layout_asserts.rs)`.  **This mirror is among the 66** (it
  is not in the skip list), so the gate compiles it - with its `msdos`/`djgpp_2_3` arms masked, and
  together with `include/w32gui.rs`'s `Display`/`Window` - every round; the `msdos`-masked items
  are compiled only by a future MS-DOS target.
- `rg -n 'include/msdos' femacs` -> no unit `include!`s the mirror: the C units that include the
  header are all inside `#ifdef MSDOS`, and their ports are the empty shapes of
  `femacs/src/msdos.rs`/`femacs/src/dosfns.rs`.
- Gates on the same worktree: `check-declarations` 13 sites (0 forbidden, 13 allowed literals) / 0
  failures; `check-mirror-dupes` 1 duplicate declaration(s) / 1 ratchet row(s) / 0 failure(s);
  `check-globals` 283 units / 0; `check-verbatim` 6 trees / 4607 files / 0 failures;
  `check-target-names` 566 .rs files / 593 map targets / 0 findings.
