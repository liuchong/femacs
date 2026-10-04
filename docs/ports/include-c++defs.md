# emacs/lib/c++defs.h (367 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **Nothing to carry, and that is measured rather than assumed.**  Every definition in the header
  sits in a C++-only arm (`#if defined __cplusplus && defined GNULIB_NAMESPACE` at C 21-27,
  165-183, 200-218, 238-256, 263-281, 289-312, 318-340, 345-362; `#if defined __cplusplus` at
  C 90-105, 134-138) whose `#else` arm is a no-op spelling of the same name, and the frozen
  target compiles C.  `tools/extract-abi.py lib/c++defs.h` reports 0 subrs, 0 functions, 0 macros,
  0 types and 0 file-scope objects; `tools/check-cond-eval.py lib/c++defs.h` confirms the file's
  27 conditions against clang (0 unknown, 0 disagreements), so the dead-arm reading is the
  frozen configuration's own.
- **The alias machinery keeps an owner, and the use sites record the resolution.**  The
  `_GL_CXXALIAS_*` / `_GL_FUNCDECL_*` macros are how the generated gnulib headers name the system
  entry point they resolve to; on this side that resolution is recorded where it is used:
  `fcntl.in.rs` 274-276 (`creat`, `open`, `openat`), `time.in.rs` 156, `unistd.in.rs` 24 and
  `sys_stat.in.rs` 43 and 175.  This mirror is the file-level record those four comments already
  describe one use at a time.
- **Who includes it, measured:** nobody.  A fixed-string search over the reference
  (`grep -rn -F 'c++defs.h' emacs --include='*.h' --include='*.in.h' --include='*.c'`) matches only
  `emacs/lib/gnulib.mk.in` 3180 and 3182 - the build machinery's `CXXDEFS_H` variable and its
  `EXTRA_DIST` - and a search under `build/emacs-ref/lib/` matches only `gnulib.mk`; no source file
  and no generated header names it.  (An earlier draft of this paragraph listed `lib/verify.h`,
  `lib/stdint.in.h`, `lib/getopt.in.h` and `lib/getopt-cdefs.in.h` as includers: that came from a
  grep whose pattern was `c++defs\.h`, which is not a valid regex, so its output was never
  evidence.  The paragraph is rewritten from the fixed-string search.)  The macro machinery still
  has use sites on this side, and those are recorded where they are used: `fcntl.in.rs` 274-276
  (`creat`, `open`, `openat`), `time.in.rs` 156, `unistd.in.rs` 24 and `sys_stat.in.rs` 43 and 175.
  Nothing includes this mirror because the header it mirrors is a snippet the generated headers
  consume under the gnulib build; the omission is named here so it is auditable.

## Evidence

- `python3 tools/extract-abi.py --check emacs/lib/c++defs.h femacs/lib/include/c++defs.rs`
  -> `OK emacs/lib/c++defs.h -> femacs/lib/include/c++defs.rs` (no `- ` problem lines).
- `python3 tools/check-cond-eval.py lib/c++defs.h`
  -> `27 conditions, 0 unknown to the evaluator, 0 disagreements`.
- `python3 tools/extract-abi.py lib/c++defs.h` -> this ledger: 0 items in all five sections.
- Compile proof: the mirror is one `pub const` with no dependencies; a standalone
  `rustc --edition 2024 --crate-type=lib --emit=obj` crate that `include!`s it builds (exit 0).
- Gates on the same worktree: `check-declarations` 13 sites (0 forbidden, 13 allowed literals) /
  0 failures; `check-mirror-dupes` 0 failures; `check-globals` 283 units / 0; `check-verbatim`
  6 trees / 4607 files / 0; `check-target-names` 0 findings.
