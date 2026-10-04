# emacs/src/cygw32.h (40 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] CYGW32_H  (line 20, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The header is unconditional, and nothing on this target compiles it.**  C 19-20 is the
  include guard, C 21-34 are `#include`s, and C 36-38 are live text — there is no `#if` anywhere
  in the file (`tools/check-cond-eval.py src/cygw32.h` -> `1 conditions, 0 unknown,
  0 disagreements [VERIFIED]`, that one condition being the guard).  The frozen configuration's
  platform object variables are all empty (`CYGWIN_OBJ=`, `MSDOS_OBJ=`, `WINDOW_SYSTEM_OBJ=`,
  `HAIKU_OBJ=` in `docs/baseline/frozen-build-sources.txt`; docs/status.md 3.31), so
  `src/cygw32.c` has no translation unit here: `femacs/src/cygw32.rs` is the empty shape that
  names the exclusion, and the three entry points the mirror carries are deferred to the Cygwin
  target together with their definitions (the `src/w32select.h` precedent, status 3.158).
- **P-72 decides the names: the mirror may only name what its own text reaches.**  The three
  declarations reach only `void`, `int`, `char *` and two typedefs from the platform's
  <windef.h> (C 22: `LPBYTE` and `LPDWORD`), for which the port has no mirror — so the pointee
  is spelled with the type C's typedef names: `LPBYTE` is `unsigned char *` -> `*mut u8`, and
  `LPDWORD` is `DWORD *` -> `*mut c_ulong` (PORTING.md 1.1 maps C's `unsigned long` to `c_ulong`,
  which is DWORD's own type on that platform).  Each declaration's C text is in a `/// C:` line
  (PORTING.md P-55), which is what makes the substitution auditable.  Arity, pointer depth,
  `const`-ness and the `int`/`void`/`char *` returns are all C's; the parameter named `type` in
  C 38 is `type_` (a Rust keyword, PORTING.md 0.2 — the `atimer.rs` `type_` precedent).
- **The header's own includes contribute nothing to the declarations, so the mirror carries no
  `include!`.**  C 21-34 pull in `<config.h>`, `<windef.h>`, `<sys/cygwin.h>`, `<wchar.h>`,
  `<signal.h>`, `<stdio.h>`, `<limits.h>`, `<errno.h>`, `<math.h>`, `<setjmp.h>`, `"lisp.h"` and
  `"coding.h"` — none of the three declarations uses a name from any of them; a unit on the
  target that compiles this header takes the platform's names from its own includes
  (PORTING.md 0.1-9, the `include/w32inevt.rs` reading).
- **`--check` asks for less than fidelity does, and the mirror keeps the difference visible.**  The
  extractor's item list for this header is the guard alone (`## Functions defined — 0` counts
  definitions, not declarations, and a declaration of a scalar type is not a "file-scope
  object" either), so a mirror holding only `CYGW32_H` would already check OK; C 36-38 are here
  because they are the file's live text, and this paragraph keeps that from looking like a
  required item later.

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/cygw32.h femacs/src/include/cygw32.rs`
  -> `OK emacs/src/cygw32.h -> femacs/src/include/cygw32.rs` (no `- ` problem lines).
- `python3 tools/check-cond-eval.py src/cygw32.h` -> `1 conditions, 0 unknown to the evaluator,
  0 disagreements` (that condition is the guard).
- `python3 tools/extract-abi.py emacs/src/cygw32.h` -> this ledger: 0 subrs, 0 functions defined,
  1 macro (the guard), 0 types, 0 file-scope objects.
- `rg -n "syms_of_cygw32|w32_strerror|w32_get_resource" femacs` -> this mirror, plus one copied
  C *comment* (`lib-src/emacsclient.rs` 17 names `w32_get_resource` while describing the Windows
  arm it does not port).  There is no use-site declaration to retire (unlike 3.143/3.153), and
  `rg -n 'include/cygw32' femacs` -> no unit `include!`s the mirror.
- **Compile evidence**: `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-win cargo run -q -p xtask -- layout-check`
  -> `layout-check: 66 of 77 mirror(s) compiled, 11 platform-exclusive skipped (android-asset.rs,
  android.rs, androidgui.rs, androidterm.rs, dispextern.rs, gtkutil.rs, haiku_support.rs,
  haikugui.rs, haikuterm.rs, nsgui.rs, pgtkterm.rs)`, `layout probe ok`,
  `layout-check passed (see build/femacs/layout_asserts.rs)`.  **This mirror is among the 66** (it
  is not in that skip list), so the gate compiles it every round.  An earlier run of the same gate,
  with only `cygw32.rs` of this batch present, printed `layout probe ok`; a later one failed with
  `error[E0428]: the name Emacs_Cursor is defined multiple times … nsgui.rs:57:1 …
  dispextern.rs:62:1` - two mirrors of another lane, both of which had entered the skip list by the
  time of the passing run, and no diagnostic of either run named a mirror of this batch.  Nothing in this mirror needed a `cfg`.

- Gates on the same worktree: `check-declarations` 13 sites (0 forbidden, 13 allowed literals) / 0
  failures; `check-mirror-dupes` 1 duplicate declaration(s) / 1 ratchet row(s) / 0 failure(s);
  `check-globals` 283 units / 0; `check-verbatim` 6 trees / 4607 files / 0 failures;
  `check-target-names` 576 .rs files / 592 map targets / 0 findings (the file count includes mirrors other lanes landed in the same session).
