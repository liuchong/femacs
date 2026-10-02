# emacs/lib/filevercmp.c (188 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 5
- [x] file_prefixlen  (line 36, static)
- [x] order  (line 62, static)
- [x] verrevcmp  (line 92, static)
- [x] filevercmp  (line 134)
- [x] filenvercmp  (line 142)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0


## Decisions

`emacs/lib/filevercmp.c` @ `fd7afced403`; port `femacs/lib/filevercmp.rs` (220 lines).  All
checklist items ticked; `build-tu` → `built filevercmp.rs -> build/femacs/filevercmp.o` and
`--check` → `OK`.

- The file has **no `#if` arms**, so nothing is dead: all four helpers are `static`
  (`file_prefixlen`, `order`, `verrevcmp`; no `#[unsafe(no_mangle)]`) and the two exported
  ones (`filevercmp`, `filenvercmp`) carry it.  Half of the C file is the header's long
  specification comment, which the port keeps as the header mirror's text.
- **`c-ctype.h`**: the three predicates are header-inline in C (no symbol in the object), so
  the port has a new WIP mirror `include/c-ctype.rs` with `#[inline(always)]` functions
  (P-12); its ledger keeps the unimplemented entries unchecked.  The `_C_CTYPE_*` case lists
  become byte-range patterns, and `docs/ports/macro-body-notes.tsv` records the
  `switch`/`case`/`default` of the two predicates the token check names.
- **`static_assert`** (C 76) is the port's `static_assert!`, a unit-local `macro_rules!`
  expanding to `const _: () = assert! (…)` (PORTING.md 2.5) - the token, not just the check,
  is what the gate sees.
- **`file_prefixlen`'s `continue`** (C 45) is recorded in `macro-body-notes.tsv`: C's
  incrementing inner `for` body has no statement to continue to in the port's `while`.
- Line drift: `line-drift-tolerance.tsv` records `filevercmp 0.20` (17.0%).
- Not verified: gate-fast/gate-probes, ERT, and any behavioral differential of the version
  sort itself.
