# emacs/src/dosfns.h (35 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] DOS_COUNTRY_INFO  (line 23, body f1f836cb4ea6)

## Types and their fields — 0

## File-scope objects — 1
- [x] extern unsigned char dos_country_info  (line 24)

## Decisions

- **The header has no include guard, and the extractor's item list says so.**  C 23 is the file's
  first line after the opening comment: `#define DOS_COUNTRY_INFO 34`, with no `#ifndef` above it.
  So there is no guard name for the mirror to carry — the one macro in this ledger is C 23, not a
  guard (compare `include/cygw32.rs`, whose single macro is `CYGW32_H`).
- **`DOS_COUNTRY_INFO` is a constant, not a `macro_rules!`.**  C's macro has no type of its own;
  every use of the name is an array length (C 24, and `src/dosfns.c`'s own tables), so the port
  spells it `pub const DOS_COUNTRY_INFO: usize = 34;` — the reading `lib/root-uid.h`'s ledger
  records for `ROOT_UID` (status 3.156).  The extractor accepts a constant for a C macro name and
  compares the *value* (34), which is what the checklist box above is ticked against.
- **The one conditional (`#ifndef HAVE_X_WINDOWS`, C 31) is *satisfied* on this target, so C 32-34
  are live text.**  `docs/baseline/config/darwin-arm64.macros.tsv` leaves `HAVE_X_WINDOWS`
  undefined (the character-terminal build), so the `#ifndef` holds and the three declarations
  inside it are compiled by C here.  The mirror therefore carries them unconditionally, and there
  is no dead arm to put behind `#[cfg (feature = …)]` (PORTING.md P-51 would apply only to the
  branch the frozen configuration drops).  `tools/check-cond-eval.py src/dosfns.h` ->
  `1 conditions, 0 unknown, 0 disagreements [VERIFIED]`.
- **P-72: `struct frame` degrades, `Lisp_Object` does not.**  The header includes nothing, so the
  two `struct frame *` parameters (C 34 in the guarded arm) are pointers to a record this file
  never declares and cannot reach: they are `*mut c_void` with the C text in a `/// C:` line
  (PORTING.md P-55/P-72 rule 3, the `include/w32inevt.rs` spelling; `xfaces.rs` 982 and
  `alloc.rs` 7728 show the port already reads that record only as a pointer at its MS-DOS arms).
  `Lisp_Object` stays named, because `lisp.h` is the one header every translation unit reaches
  through its own includes (PORTING.md 0.1-9).
- **The four `unsigned long *` out-parameters are `*mut c_ulong`.**  `src/dosfns.c` 633-634 gives
  the definition the parameter names (`totalram`, `freeram`, `totalswap`, `freeswap`) the mirror
  uses; the header itself leaves them unnamed, so C's *names* come from the definition the mirror's
  declaration points at, exactly as PORTING.md 0.2's "same names" rule wants.
- **Nothing on this target compiles the header**, so the mirror has no unit `include!`ing it: the
  frozen configuration's `MSDOS_OBJ` is empty (docs/baseline/frozen-build-sources.txt), and
  `femacs/src/dosfns.rs` is the empty shape that records the exclusion.  The definitions are
  deferred to the MS-DOS target with the declarations.
- **Use-site copies that do *not* need retiring**: `src/vm-limit.rs` 110-113 declares
  `dos_memory_info` inside its `#[cfg (feature = "msdos")]` arm (dead text on this target) and
  says so in the comment above it.  That is the P-51 dead-arm shape, not a second live
  declaration, so it stays where it is; the arm's comment ("dosfns.h declares this") remains true
  now that the header has a mirror.

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/dosfns.h femacs/src/include/dosfns.rs`
  -> `OK emacs/src/dosfns.h -> femacs/src/include/dosfns.rs` (no `- ` problem lines).
- `python3 tools/check-cond-eval.py src/dosfns.h` -> `1 conditions, 0 unknown to the evaluator,
  0 disagreements` (the `#ifndef HAVE_X_WINDOWS` of C 31).
- `python3 tools/extract-abi.py emacs/src/dosfns.h` -> this ledger: 0 subrs, 0 functions defined,
  1 macro (C 23), 0 types, 1 file-scope object (`dos_country_info`, C 24).
- `rg -n "dos_country_info|dos_memory_info|dos_cleanup|syms_of_dosfns|init_dosfns|msdos_stdcolor_idx|msdos_stdcolor_name|x_set_title" femacs`
  -> this mirror, `src/vm-limit.rs` 110/122 and `src/alloc.rs` 7728 (both inside their own
  `msdos`/`MSDOS` arms), and one copied C *comment* (`src/xfaces.rs` 982).  `rg -n 'include/dosfns' femacs`
  -> no unit `include!`s the mirror.
- **Compile evidence**: `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-win cargo run -q -p xtask -- layout-check`
  -> `layout-check: 66 of 77 mirror(s) compiled, 11 platform-exclusive skipped (android-asset.rs,
  android.rs, androidgui.rs, androidterm.rs, dispextern.rs, gtkutil.rs, haiku_support.rs,
  haikugui.rs, haikuterm.rs, nsgui.rs, pgtkterm.rs)`, `layout probe ok`,
  `layout-check passed (see build/femacs/layout_asserts.rs)`.  **This mirror is among the 66** (it
  is not in that skip list), so the gate compiles it every round.  An earlier run of the same gate,
  with only `cygw32.rs` of this batch present, printed `layout probe ok`; a later one failed with
  `error[E0428]: the name Emacs_Cursor is defined multiple times … nsgui.rs:57:1 …
  dispextern.rs:62:1` - two mirrors of another lane, both of which had entered the skip list by the
  time of the passing run, and no diagnostic of either run named a mirror of this batch.  Its `Lisp_Object` comes from `include/lisp.rs`, which the gate includes as a foundation mirror.

- Gates on the same worktree: `check-declarations` 13 sites (0 forbidden, 13 allowed literals) / 0
  failures; `check-mirror-dupes` 1 duplicate declaration(s) / 1 ratchet row(s) / 0 failure(s);
  `check-globals` 283 units / 0; `check-verbatim` 6 trees / 4607 files / 0 failures;
  `check-target-names` 576 .rs files / 592 map targets / 0 findings (the file count includes mirrors other lanes landed in the same session).
