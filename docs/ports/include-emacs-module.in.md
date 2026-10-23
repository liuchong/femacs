# emacs/src/emacs-module.in.h (246 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 6
- [x] EMACS_ATTRIBUTE_NONNULL  (line 64, body da39a3ee5e6b)
- [x] EMACS_LIMB_MAX  (line 139, body 256c9f9a067e)
- [x] EMACS_MAJOR_VERSION  (line 38, body 9f767c809a6e)
- [x] EMACS_MODULE_H  (line 27, body da39a3ee5e6b)
- [x] EMACS_NOEXCEPT  (line 43, body da39a3ee5e6b)
- [x] EMACS_NOEXCEPT_TYPEDEF  (line 49, body da39a3ee5e6b)

## Types and their fields — 1
- [x] emacs_runtime: ptrdiff_t size, struct emacs_runtime_private* private_members, emacs_env * (*) get_environment

## File-scope objects — 0

## Decisions

- **The judge is the generated header.**  `src/emacs-module.in.h` is a template: `configure`
  substitutes `@emacs_major_version@` and the eight `@module_env_snippet_NN@` placeholders.  The
  frozen build's output is `build/emacs-ref/src/emacs-module.h` (49,460 bytes, 1,494 lines), and
  the readings the mirror uses are its own lines: `#define EMACS_MAJOR_VERSION 32` (38),
  `typedef struct emacs_env_32 emacs_env;` (72), the eight records (141, 276, 415, 573, 751, 930,
  1110, 1291) and `emacs_module_init` (1486).  This is the `lib/dirent.in.h`/`lib/errno.in.h`
  reading of a template (status 3.159), and it is why the mirror spells `32` rather than the
  placeholder.
- **`tools/check-cond-eval.py src/emacs-module.in.h` reports `9 conditions, 3 unknown,
  5 disagreements [NOT VERIFIED]`** - the conditions ask about `__STDC_VERSION__`, `__cplusplus`,
  `__GNUC__`, `__GNUC_MINOR__` and `__has_attribute`, none of which the frozen macro table decides
  the way the template's text assumes (C 33-65).  The file stays NOT VERIFIED, so the extractor
  keeps both arms of every condition live and its item list is the union (P-66's conservative
  reading); the port follows the judge, and the attribute arms are disposed of below.  The three
  condition-driven macros are additionally reported by the tool as branch macros: *"3 C macros have
  several definitions (configuration branches), body not compared: EMACS_ATTRIBUTE_NONNULL,
  EMACS_NOEXCEPT, EMACS_NOEXCEPT_TYPEDEF"*.
- **The eight `struct emacs_env_NN` records are here, not in the fragments' mirrors, and this is a
  measured decision.**  C's template builds each record by pasting snippets 25..NN into its body
  (C 141-235) and the judge has them inlined; a Rust mirror can only be pasted where a macro or an
  item may appear, and a struct's member list is not such a place:
  ```
  $ printf 'pub a: u8,\n' > /tmp/frag.rs
  $ rustc --edition 2024 --crate-type lib - <<'X'
  #[repr(C)] pub struct S { include!("/tmp/frag.rs") }
  X
  error: unexpected token: `!`
   = note: macros cannot expand to struct fields
  ```
  So the eight fragment mirrors (`docs/ports/include-module-env-25..32.md`) carry C's text verbatim
  and record the substitution, and the member *spellings* live in the eight `#[repr(C)]` records
  this mirror declares, in the judge's member order.  Consequence for the checklist above: the
  template's `## Types and their fields` names only `emacs_runtime`, because the placeholders leave
  the `emacs_env_*` containers with no fields for the extractor to see - the eight records are
  therefore **not** covered by `--check`, and what keeps them honest is the measured layout
  evidence below (the same position `include/w32gui.rs` takes for its deferred record, from the
  other side).
- **The four macros this mirror cannot carry, and the rows they need** (the lane may not write the
  shared tables; the suggested rows are in `## Evidence`): `EMACS_NOEXCEPT` and
  `EMACS_NOEXCEPT_TYPEDEF` expand to nothing on this target and `EMACS_ATTRIBUTE_NONNULL` to an
  attribute, which a `macro_rules!` cannot produce (the `_Alignas` row of
  docs/ports/excluded-macros.txt states the rule) - the house treatment for that class is an
  `excluded-macros.txt` row with `kind=syntax` (the `_GL_ATTRIBUTE_CONST`, `_GL_ARG_NONNULL` and
  `_GL_ATTRIBUTE_COUNTED_BY` precedents).  `EMACS_MAJOR_VERSION`'s body is the template's own
  `@PLACEHOLDER@` text, which has no Rust spelling; the mirror carries the frozen substitution (32,
  judge 38) and the name needs a row, exactly as `SIG_ATOMIC_MAX`/`WCHAR_MAX` do for their
  `@PLACEHOLDER@` bodies in excluded-macros.txt (`kind=dead-text` there; here the macro is live, so
  `kind=config` is the closer class).  `EMACS_MODULE_H` (the guard) and `EMACS_LIMB_MAX`
  (`SIZE_MAX`) are carried normally.
- **Degradations (PORTING.md P-55/P-72 rule 3).**  `struct emacs_value_tag` is defined by
  `src/emacs-module.c` 125, and `struct emacs_env_private`/`struct emacs_runtime_private` by its
  163/177 - records of the *unit*, whose port is the empty shape `HAVE_MODULES` leaves
  (`src/emacs-module.rs`), so the three pointers keep C's depth with an untyped interior
  (`emacs_value` stays an alias with C's name, so its ~40 uses in the member list spell one type,
  P-26); the C text of each is in a `/// C:` line.  `size_t`/`ptrdiff_t`/`intmax_t` are
  `include/lisp.rs` 50-53's aliases, which every unit and the layout probe carry (PORTING.md
  0.1-9 - C 29's `<stddef.h>` has no port mirror yet), `<stdint.h>` (C 30) and `<time.h>` (C 31)
  are `include!`ed for `SIZE_MAX` and `struct timespec`, and the function-pointer members are
  `Option<unsafe extern "C" fn …>` (PORTING.md P-27, the `include/lisp.rs` `finalizer` shape).
- **The layout claim is measured with the judge's own compiler, and the mirror keeps a durable
  subset of the readings.**  The nine records' sizes/alignments and every member offset were read
  with `clang -std=gnu23 -O0 -include config.h -I build/emacs-ref/src -I build/emacs-ref/lib
  -I emacs/src -I emacs/lib -Xclang -fdump-record-layouts-complete -fsyntax-only`; the mirror
  carries 62 of those readings as `const _: () = assert! (…)` (nine `size_of`, nine `align_of`,
  `emacs_runtime`'s three offsets and `emacs_env_32`'s forty-one - i.e. the current ABI record in
  full), and the ledger records the exhaustive check of all **298** member offsets plus the nine
  sizes with a lane-local probe.  A member whose type changes width, or a member that moves, now
  fails either the mirror's own assertions or that probe rather than quietly keeping the ABI claim.
- **Integrator item (recommended, not done here)**: `tools/gen-layout-fields.py`'s `STRUCTS` can
  cover these records - `live_bodies`/`inventory` resolve `ref / header`, so
  `"emacs_env_32": "../build/emacs-ref/src/emacs-module.h"` reaches the generated header through
  `emacs/../build/...`, and the generator's own `-I` flags are the ones the dump above used.  One
  hazard measured here: cinventory's field scanner also sees the *nested* parameter names inside
  those function-pointer members (`func` of `make_function`, `fin` of `make_user_ptr`), and those
  names must not be emitted as `offsetof` keys - see `## Evidence`.

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/emacs-module.in.h src/include/emacs-module.in.rs`
  -> **OK** (no `- ` problem lines).  The four items the first run reported - the three
  attribute/noexcept macros and `EMACS_MAJOR_VERSION`, whose body is the template's own
  `@emacs_major_version@` text - are recorded in `docs/ports/excluded-macros.txt` (three
  `kind=syntax`, one `kind=config`) with the reason each has no Rust item; the extractor now prints
  `(4 C macros have a recorded Rust counterpart: …)` and the checklist boxes are ticked.
  The four lines the first run printed and no others:
  ```
  - missing macro EMACS_ATTRIBUTE_NONNULL
  - missing macro EMACS_NOEXCEPT
  - missing macro EMACS_NOEXCEPT_TYPEDEF
  - macro EMACS_MAJOR_VERSION does not mention emacs_major_version (C body: @emacs_major_version@)
  ```
- `python3 tools/check-cond-eval.py src/emacs-module.in.h` -> `src/emacs-module.in.h: 9 conditions,
  3 unknown, 5 disagreements [NOT VERIFIED]` (tail line: `9 conditions, 3 unknown to the evaluator,
  5 disagreements`).
- `python3 tools/extract-abi.py emacs/src/emacs-module.in.h` -> this ledger: 0 subrs, 0 functions
  defined, 6 macros, 1 type (`emacs_runtime`, 3 fields), 0 file-scope objects.
- The judge's own readings the mirror quotes: `ls -l build/emacs-ref/src/emacs-module.h` ->
  `49460` bytes; `grep -n '^struct emacs_env_'` -> 141, 276, 415, 573, 751, 930, 1110, 1291;
  `grep -n 'EMACS_MAJOR_VERSION'` -> 38; the `-dM` reading of the attribute family is in the
  file note above.
- **The exhaustive layout check** (lane-local probe, `/tmp/modprobe/verify.rs`, deleted afterwards):
  it `include!`s this mirror with `config.rs`-free stand-ins for lisp.rs's four aliases and asserts
  every size, alignment and member offset the dump reported ->
  `rustc +nightly-2026-10-01 --edition 2024 --crate-type bin -C panic=abort -o /tmp/modprobe/verify /tmp/modprobe/verify.rs`
  then `/tmp/modprobe/verify` -> `mod ABI probe ok: 9 records, 298 member offsets`.  The measured
  sizes are `emacs_runtime` 24, `emacs_env_25` 232, `_26` 240, `_27` 280, `_28/_29/_30/_31` 320 (all
  align 8) and `_32` 328 - which is also what proves the three 0-byte fragments add nothing.
- **Compile evidence**: `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-win cargo run -q -p xtask
  -- layout-check` -> `layout-check: 77 of 88 mirror(s) compiled, 11 platform-exclusive skipped
  (android-asset.rs, android.rs, androidgui.rs, androidterm.rs, dispextern.rs, gtkutil.rs,
  haiku_support.rs, haikugui.rs, haikuterm.rs, nsgui.rs, pgtkterm.rs)` and `layout-check passed
  (see build/femacs/layout_asserts.rs)`.  This mirror is among the 77, so the gate compiles it and
  **evaluates its 62 layout assertions every round** (a false assertion would be a compile error).
- `rg -n 'include/emacs-module' femacs` -> no unit `include!`s this mirror: the C header's readers
  are the module-support units, whose port is the empty shape `HAVE_MODULES` leaves
  (`src/emacs-module.rs`), and `src/emacs-module.c`'s own readers on other platforms reach
  the API through that unit.
- Gates on the same worktree: `check-declarations` 13 sites (0 forbidden, 13 allowed literals) / 0
  failures; `check-mirror-dupes` 1 duplicate declaration(s) / 1 ratchet row(s) / 0 failure(s);
  `check-globals` 283 units / 0; `check-verbatim` 6 trees / 4607 files / 0 failures;
  `check-target-names` 589 .rs files / 592 map targets / 0 findings.
