# emacs/lib/warn-on-use.h (153 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The header carries no item for the port to mirror, and that is a measurement.** `python3
  tools/extract-abi.py emacs/lib/warn-on-use.h` reports 0 functions, 0 macros, 0 types and 0
  file-scope objects, and `python3 tools/extract-abi.py --check emacs/lib/warn-on-use.h
  femacs/lib/include/warn-on-use.rs` is `OK` with no problem lines.  The file below is the
  disposition plus the evidence that produced it, the shape `include-arg-nonnull.md` uses for
  the sibling module.

- **Why 0 macros although the header writes four `#define`s.** All four names are the frozen
  configuration's own: `docs/baseline/config/darwin-arm64.macros.tsv` 4268 `_GL_WARN_EXTERN_C
  extern`, 4269 `_GL_WARN_ON_USE (function,message) _GL_WARN_EXTERN_C __typeof__ (function)
  function __attribute__ ((__diagnose_if__ (1, message, "warning")))`, 4270
  `_GL_WARN_ON_USE_ATTRIBUTE`, 4271 `_GL_WARN_ON_USE_CXX`.  Each of the file's three guards
  tests one of those names - `#ifndef _GL_WARN_ON_USE` (C 86), `#ifndef _GL_WARN_ON_USE_CXX`
  (C 119), `#ifndef _GL_WARN_EXTERN_C` (C 147) - so the guarded block is dead text and the
  `#define`s inside it are never the file's own definitions.  `lib/warn-on-use.h` is in
  `docs/baseline/config/cond-eval-verified.txt` (line 179), so this is the evaluator's
  measured answer, not the "unknown" fallback of P-66.

- **The live arm, measured.** `python3 tools/check-cond-eval.py lib/warn-on-use.h` ->
  `lib/warn-on-use.h: 12 conditions, 0 unknown, 0 disagreements [VERIFIED]`.  The claim takes
  clang's answer (`__diagnose_if__`, C 96-98) over gcc's `__warning__` arm (C 90-93), the
  `GNULIB_STRICT_CHECKING` arm (C 103-105) and the unsupported `int _gl_warn_on_use` arm
  (C 107-109).

- **Every arm is an attribute or an unused declaration, so nothing becomes a Rust item.**
  Attributes are syntax rather than semantics: the port drops them structurally (PORTING.md
  0.3, and 8's attribute family - the `_GL_ATTRIBUTE_*` names are empty `macro_rules!` in
  `include/attribute.rs`).  The unsupported arm's `extern int _gl_warn_on_use` declares a name
  nothing references, and the port's aggregates name their own declarations rather than
  inheriting a header's (PORTING.md 0.1-4 / P-61).

- **`_GL_WARN_EXTERN_C` (C 145-153) is the same case**: the configuration defines it as
  `extern`, and the port spells `extern "C"` where a declaration needs it.  No Rust
  counterpart is wanted, so no row in `docs/ports/excluded-macros.txt` is needed either: the
  exclusion table is for macros the extractor still demands, and the table is not consulted
  for this file's guards (see above).

- **No includer, and that is the reference's shape.** Measured: `grep -rn 'warn-on-use.h'
  emacs/src emacs/lib` prints only `emacs/lib/gnulib.mk.in`.  The header reaches its users
  through the build machinery's `$(WARN_ON_USE_H)` substitution into other gnulib headers, so
  the port's consumers are those headers' mirrors (`include/string.in.rs`,
  `include/inttypes.in.rs`, `include/sys_stat.in.rs`), which spell the warnings as ordinary
  declarations.  The mirror is deliberately unincluded and carries a single commentary block.

- **This mirror is a compilation unit of its own.**  `build-tu lib/warn-on-use.h` reaches
  `femacs/lib/include/warn-on-use.rs` as the crate root (xtask `port_path`), which a mirror
  with no item clears without an includer's scope.  (`include-sfnt.md` and
  `include-lib-malloc-scratch_buffer.md` record the other case: a mirror that names `c_int`
  cannot be a crate root, because a mirror may not carry a `use` - E0252 against the
  includer's - and takes those scalars from the unit that includes it.)

## Evidence

```
$ python3 tools/extract-abi.py emacs/lib/warn-on-use.h
# emacs/lib/warn-on-use.h (153 lines)
## Subrs (DEFUN) - 0
## Functions defined - 0
## Macros defined - 0
## Types and their fields - 0
## File-scope objects - 0

$ python3 tools/extract-abi.py --check emacs/lib/warn-on-use.h femacs/lib/include/warn-on-use.rs
OK emacs/lib/warn-on-use.h -> femacs/lib/include/warn-on-use.rs

$ python3 tools/check-cond-eval.py lib/warn-on-use.h
lib/warn-on-use.h: 12 conditions, 0 unknown, 0 disagreements [VERIFIED]

$ grep -n '_GL_WARN' docs/baseline/config/darwin-arm64.macros.tsv
4268:_GL_WARN_EXTERN_C	extern
4269:_GL_WARN_ON_USE	(function,message) _GL_WARN_EXTERN_C __typeof__ (function) function __attribute__ ((__diagnose_if__ (1, message, "warning")))
4270:_GL_WARN_ON_USE_ATTRIBUTE	(message) __attribute__ ((__diagnose_if__ (1, message, "warning")))
4271:_GL_WARN_ON_USE_CXX	(function,rettype_gcc,rettype_clang,parameters_and_attributes,msg) _GL_WARN_ON_USE (function, msg)

$ grep -n 'warn-on-use' docs/baseline/config/cond-eval-verified.txt
179:lib/warn-on-use.h

$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-g188 cargo run -q -p xtask -- build-tu lib/warn-on-use.h
built lib/include/warn-on-use.rs -> ../build/femacs/include_warn-on-use.o
```

- The reference header has no consumer in the port, so `layout-check` is the tree-level
  reading that covers the rest of this round's work; the last line of a run on this worktree
  is recorded with the other four ledgers.
- `grep -rn 'warn-on-use.h' emacs/src emacs/lib` -> two lines, both from
  emacs/lib/gnulib.mk.in (3194 `WARN_ON_USE_H=$(srcdir)/warn-on-use.h`, 3196
  `EXTRA_DIST += warn-on-use.h`), confirmed on this checkout.
