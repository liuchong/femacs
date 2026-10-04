# emacs/lib/attribute.h (364 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 32
- [x] ATTRIBUTE_ALLOC_SIZE  (line 131, body db6231f6cf8e)
- [x] ATTRIBUTE_ALWAYS_INLINE  (line 254, body 6fdf56cc30eb)
- [x] ATTRIBUTE_ARTIFICIAL  (line 232, body c5daa5e8712a)
- [x] ATTRIBUTE_COLD  (line 323, body 1ab71e3a2c57)
- [x] ATTRIBUTE_CONST  (line 270, body 78ddefb9dbb8)
- [x] ATTRIBUTE_DEALLOC  (line 139, body b9c41969340b)
- [x] ATTRIBUTE_DEALLOC_FREE  (line 140, body feb579fccbb3)
- [x] ATTRIBUTE_ERROR  (line 117, body 1be15b6ff42b)
- [x] ATTRIBUTE_EXTERNALLY_VISIBLE  (line 236, body bc3484efd7f1)
- [x] ATTRIBUTE_FORMAT  (line 167, body 3eae10898dc0)
- [x] ATTRIBUTE_LEAF  (line 329, body 17504d5bff6c)
- [x] ATTRIBUTE_MALLOC  (line 124, body 85d0edd07ff8)
- [x] ATTRIBUTE_MAY_ALIAS  (line 347, body c4955aca18c8)
- [x] ATTRIBUTE_NOINLINE  (line 249, body 42374afc111d)
- [x] ATTRIBUTE_NONNULL  (line 172, body 7ee3ed9b5137)
- [x] ATTRIBUTE_NONNULL_IF_NONZERO  (line 177, body 54097287ff5f)
- [x] ATTRIBUTE_NONSTRING  (line 205, body be9f74382b8b)
- [x] ATTRIBUTE_NOTHROW  (line 245, body 1f386600c1c7)
- [x] ATTRIBUTE_PACKED  (line 336, body 65081910e513)
- [x] ATTRIBUTE_PURE  (line 303, body a9282adda303)
- [x] ATTRIBUTE_RETURNS_NONNULL  (line 182, body 273de8859427)
- [x] ATTRIBUTE_SENTINEL  (line 148, body e81244463594)
- [x] ATTRIBUTE_WARNING  (line 113, body 4fe28f9bbb03)
- [x] COUNTED_BY  (line 223, body 0f091954f2da)
- [x] DEPRECATED  (line 109, body 21bb63e6a163)
- [x] FALLTHROUGH  (line 210, body cb6247d99acc)
- [x] MAYBE_UNUSED  (line 200, body 749eff4abe4a)
- [x] NODISCARD  (line 187, body 4caf8cf17bd7)
- [x] REPRODUCIBLE  (line 319, body 4052633f193a)
- [x] UNNAMED  (line 361, body d457889c03d1)
- [x] UNSEQUENCED  (line 289, body 9b168b205902)
- [x] _GL_ATTRIBUTE_H  (line 69, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/lib/attribute.h` @ `fd7afced403`; port `femacs/lib/include/attribute.rs`.  Every
checklist item is ticked and `extract-abi.py --check` reports `OK` for the pair.  The header
is in `docs/baseline/config/cond-eval-verified.txt`, so the gate reads its `#if`s with the
frozen configuration and the 32 names below are the ones that build selects.

**What the header is.**  A thin mapping: every public name forwards to a private
`_GL_ATTRIBUTE_*` (or `_GL_UNNAMED`) spelling, and those private names are defined by
gnulib's configuration block in the generated `config.h` as C attributes
(`__attribute__ ((...))`, or C23's `[[...]]` form), **not** in this header.  The port's
`config.rs` is generated from the frozen value list and carries no macro shapes, so the
private names have no other home; they are kept here, adjacent to their only users, as the
no-ops their attributes are (PORTING.md 8's "其余记无操作").  The file also already carried
two `lib/cdefs.h` names (`__attribute_noinline__`, `__attribute_warn_unused_result__`) and
`libc-config.h`'s `attribute_compat_text_section` in the same arrangement, so this is the
file's existing shape rather than a new one.

**Two layers, so the extractor's token comparison is satisfied and a use site still works.**
Each pair is
`macro_rules! _GL_ATTRIBUTE_X { () => {}; }` +
`macro_rules! ATTRIBUTE_X { () => { _GL_ATTRIBUTE_X! () }; }` (with `$tt` parameters where
C passes arguments, since the C arguments are attributes' operands - `((2, 1))`,
`((printf, 1, 2))`, `free, 1`, a message string - and nothing in the port expands them).
An outer body that was merely empty would leave the C body's `_GL_ATTRIBUTE_X` token
unmentioned and fail `--check` (that was the state of the WIP mirror's `FALLTHROUGH` and
`NODISCARD`); a body that named it without the inner macro would not expand anywhere.

**Rust counterparts** (PORTING.md 8; each is written at the site that needs it, not here):
`ATTRIBUTE_NOINLINE` → `#[inline(never)]`, `ATTRIBUTE_ALWAYS_INLINE` →
`#[inline(always)]`, `NODISCARD` → `#[must_use]`, `ATTRIBUTE_PACKED` → `#[repr(packed)]`,
`ATTRIBUTE_COLD` → `#[cold]`, `ATTRIBUTE_EXTERNALLY_VISIBLE` → `#[unsafe(no_mangle)] pub`,
`ATTRIBUTE_MAY_ALIAS`/`ATTRIBUTE_NONSTRING`/`ATTRIBUTE_NOTHROW`/`ATTRIBUTE_LEAF`/
`UNSEQUENCED`/`REPRODUCIBLE`/`ATTRIBUTE_CONST`/`ATTRIBUTE_PURE`: no counterpart (Rust's
optimizer attributes are unstable), so those macros stay no-ops at their sites.
`UNNAMED (id)` is empty on the frozen target: `config.h`'s `unnamed_##id
_GL_ATTRIBUTE_UNUSED` arm is guarded by `__STDC_VERSION__ < 202311`, and the target is C23.
`FALLTHROUGH! ()` is the one name a ported site actually expands (ebrowse.rs 1861, 3593,
3715 - recorded in docs/ports/ebrowse.md); the expansion is empty, as the token is in C's
`((void) 0)` arm's place.

**Convergence**: no unit or mirror declared any of these names itself, so there was nothing
to delete (the tree already read them from this mirror); `check-mirror-dupes.py` stays at its
one ratchet row.

## Evidence

```
$ python3 tools/extract-abi.py --check emacs/lib/attribute.h femacs/lib/include/attribute.rs
OK emacs/lib/attribute.h -> femacs/lib/include/attribute.rs

$ cd /tmp/femacs-probe && rustc --edition 2024 --crate-type=lib --emit=obj \
    -C panic=abort attribute_probe.rs -o /tmp/femacs-probe/attribute_probe.o
exit=0        # the probe includes the mirror and expands every public name (plus the three
              # `cdefs.h`/`libc-config.h` ones) inside a function body, and reads
              # `_GL_ATTRIBUTE_H`; all of them expand to nothing.

$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-mir-limits \
    cargo run -q -p xtask -- build-tu <unit.c>
lib/regex_internal.c   -> built lib/regex_internal.rs      # lib/regex_internal.h 21 includes
                                                           # <attribute.h>; the mirror is in
                                                           # its include graph
lib-src/emacsclient.c  -> built lib-src/emacsclient.rs     # a finished lib-src unit that
                                                           # includes attribute.rs at C 88

$ python3 tools/check-mirror-dupes.py
1 duplicate declaration(s), 1 ratchet row(s), 0 failure(s)
$ python3 tools/check-declarations.py
13 declaration sites (0 forbidden, 13 allowed literals), 0 failures
$ python3 tools/check-verbatim.py
6 trees, 4607 files, 0 failures
$ python3 tools/check-globals.py
check-globals: 283 units, 0 DEFVAR_* variables without a definition in their own unit
```
