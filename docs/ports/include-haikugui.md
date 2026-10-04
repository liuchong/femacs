# emacs/src/haikugui.h (205 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 35
- [x] AllValues  (line 82, body 652902e8f0f1)
- [x] CONVERT_FROM_EMACS_RECT  (line 53, body 38273a6d66a7)
- [x] CONVERT_TO_EMACS_RECT  (line 47, body 4bba530e72d0)
- [x] CenterGravity  (line 70, body ac3478d69a3c)
- [x] EastGravity  (line 71, body c1dfd96eea8c)
- [x] ForgetGravity  (line 65, body b6589fc6ab0d)
- [x] HeightValue  (line 81, body 92df71781c5c)
- [x] NativeRectangle  (line 46, body 6d2b72d24576)
- [x] NoValue  (line 77, body bd0d3415ccf6)
- [x] NorthEastGravity  (line 68, body 77de68daecd8)
- [x] NorthGravity  (line 67, body da4b9237bacc)
- [x] NorthWestGravity  (line 66, body 356a192b7913)
- [x] PAspect  (line 93, body f5a08b8b3015)
- [x] PBaseSize  (line 94, body 08b008cb3ba4)
- [x] PMaxSize  (line 91, body 2a0a8ece4dfd)
- [x] PMinSize  (line 90, body 7f2dac477a2d)
- [x] PPosition  (line 88, body b56fff76c324)
- [x] PResizeInc  (line 92, body b307cdcc94a2)
- [x] PSize  (line 89, body a33785b93413)
- [x] PWinGravity  (line 95, body 148030b1dc2b)
- [x] STORE_NATIVE_RECT  (line 59, body cc3225e87153)
- [x] SouthEastGravity  (line 74, body 0ade7c2cf97f)
- [x] SouthGravity  (line 73, body fe5dbbcea5ce)
- [x] SouthWestGravity  (line 72, body 902ba3cda188)
- [x] StaticGravity  (line 75, body b1d5781111d8)
- [x] USPosition  (line 86, body 1e2ed944d274)
- [x] USSize  (line 87, body 72fd47b601c4)
- [x] WINDOW_HANDLE_UINTPTR  (line 39, body 8dde79acf9e5)
- [x] WestGravity  (line 69, body 1b6453892473)
- [x] WidthValue  (line 80, body eeed498aee81)
- [x] XNegative  (line 83, body d73afeb43f0b)
- [x] XValue  (line 78, body f2b1649be8f5)
- [x] YNegative  (line 84, body 19fbaf699b05)
- [x] YValue  (line 79, body 5f399b2ba212)
- [x] _HAIKU_GUI_H_  (line 20, body da39a3ee5e6b)

## Types and their fields — 1
- [x] haiku_rect: int x, int y, int width, int height

## File-scope objects — 0

## Decisions

- **The whole file is carried; nothing is dead text.**  `haikugui.h` has no `#if` of its own
  (`tools/check-cond-eval.py src/haikugui.h` -> `0 conditions`); the `#ifndef _HAIKU_GUI_H_` guard
  is the only conditional line, and it has no Rust counterpart (PORTING.md 0.1), so every
  declaration below is C's live text.  The frozen target does not build the header (no unit in
  `docs/baseline/frozen-build-sources.txt` includes it), but that is the *unit* rule, not a licence
  to shorten the mirror.
- **`typedef struct haiku_char_struct { … } XCharStruct;`** (C 23-29) is declared as the tag
  `haiku_char_struct` plus `pub type XCharStruct = haiku_char_struct;`, the shape PORTING.md 1.3
  fixes for a tagged struct that also has a typedef name.  **Blind spot (measured):** the
  extractor's `struct_fields` scanner does not see this declaration form - the typedef name after
  the closing brace - so the ledger's "Types and their fields" section lists only `haiku_rect`,
  while the five `haiku_char_struct` fields (`rbearing`, `lbearing`, `width`, `ascent`, `descent`)
  are carried in the mirror and are **not** compared by `--check`.  Contrast `androidgui.h`'s
  `typedef struct android_char_struct { … };` (no name after the brace), which the scanner does
  see.
- **`#define NativeRectangle struct haiku_rect`** (C 46) is a `macro_rules!` with no
  metavariables, because C uses the name in *type* position (`NativeRectangle *nr`).  A
  `pub type NativeRectangle = haiku_rect;` is not equivalent for the gate: `macro_body_of` reads
  `macro_rules!`/`const`/`fn` only, so a type alias is reported as `missing macro NativeRectangle`.
- **The four `*Gravity`/`*Value` groups are `c_int`, the ten `*Position` macros are `c_long`.**
  C's implicit type for `#define USPosition (1L << 0)` is `long`, and PORTING.md 1.1 maps `long`
  to `c_long`; the plain-decimal macros are `int`.  Rust cannot spell `1L`, so the value is
  `1 << 0` - token-wise identical after normalisation (`--check` compares the expression's tokens).
- **`CONVERT_TO_EMACS_RECT`/`CONVERT_FROM_EMACS_RECT`/`STORE_NATIVE_RECT`** (C 47-63) are comma
  expressions in C.  Rust has no comma operator, so each becomes a block of four assignments, the
  shape `dispextern.rs`'s `SET_GLYPH`/`SET_CHAR_GLYPH` already use.  The block's value is `()`
  where C's comma expression yields the last assigned value; every use site in the reference
  (`haikuterm.c` 1790, `xdisp.c` 3046-3056) is a statement, and that is recorded here rather than
  hidden.
- **The twelve cursor bitmaps** (C 103-203) are file-scope C objects with internal linkage
  (`MAYBE_UNUSED static unsigned char name[] = { … };`), so the mirror keeps the object shape:
  `pub static mut name: [c_uchar; N] = [ … ];` with C's own count (`128, 128, 30 x 10`, counted from
  the initialiser lists) and **no `#[no_mangle]`** - the C name is never a symbol.  `MAYBE_UNUSED`
  is an attribute (`lib/attribute.h` 200) and has no Rust counterpart.  Note the divergence from
  `macuvs.rs`, which reads PORTING.md 0.1-2's "no `pub static` definitions" as banning the object
  shape and uses `pub const` for its `static const` table; here the C object is *mutable*
  (`unsigned char`, not `const`), and without `#[no_mangle]` the mangled static cannot produce the
  `ld: duplicate symbols` failure 0.1-2 exists to prevent (`buildobj.rs` is the other mirror with a
  `pub static` initialiser).  Recorded for the integrator: flipping these to `pub const` is a
  one-word change per line if the stricter reading is preferred.
- **Names shared with other platform headers are declared here anyway.**  The X-compat block and
  `Emacs_Cursor`/`Emacs_Pixmap`/`Emacs_Window`/`Drawable`/`XCharStruct`/`Window`/`Display` repeat
  text that `androidgui.h`, `nsgui.h`, `pgtkgui.h`, `w32gui.h` and `dispextern.h`'s
  `#ifndef HAVE_WINDOW_SYSTEM` arm also declare; C separates them with platform `#if`s.  The
  mirrors meet in one crate when `xtask layout-check` compiles every `src/include` mirror, so the
  pairs listed at the end of `## Evidence` are `E0428` until the integrator decides how a platform
  mirror is scoped (the `spikes/f_inc_probe.rs` precedent for `femacs/nt/inc`, or a `cfg` on
  `dispextern.rs`'s alias as `include-androidterm.md` records).  No workaround was invented here:
  a `pub mod` wrapper would hide the `macro_rules!` names from the including unit (macro_rules
  scoping is textual), which is worse than the collision.

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/haikugui.h femacs/src/include/haikugui.rs`
  -> `OK emacs/src/haikugui.h -> femacs/src/include/haikugui.rs` (no `- ` problem lines).
- `python3 tools/check-cond-eval.py src/haikugui.h` -> decidable (no condition of its own).
- Compile proof: the mirror is included in a `spikes/f_inc_probe.rs`-shaped crate together with
  its dependencies (`c_int`, `c_long`, `c_void`, `c_uchar`) and `rustc --edition 2024
  --crate-type=lib --emit=obj` accepts it; see the lane report for the exact command and output.
- Consumer smoke: `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-mir-haikugui \
  cargo run -q -p xtask -- build-tu src/haikuterm.c` -> `built src/haikuterm.rs -> ...` (the
  platform unit is the empty unit plus its note, per docs/status.md 3.31; the mirror has no
  consumer that compiles yet).
- `python3 tools/check-declarations.py` -> 0 failures; `python3 tools/check-verbatim.py` -> 6 trees
  / 0; `python3 tools/check-mirror-dupes.py` -> 0 failures; `python3 tools/check-globals.py` -> 0.

- **The `layout-check` collision, measured** (`cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-mir-probe cargo run -q -p xtask -- layout-check` -> `error: layout probe failed to compile (exit status: 1)`, 35 x `E0428 the name ... is defined multiple times`).  The pairs are: this file's 60 `Emacs_Cursor` vs `dispextern.rs` 62 (C's `HAVE_HAIKU` vs `#ifndef HAVE_WINDOW_SYSTEM` guards keep them apart); and `androidgui.rs` 38 `XCharStruct`, 45 `Emacs_Pixmap`, 47 `Emacs_Window`, 51 `Drawable`, 53 `Emacs_Cursor` plus the 29 X-compat constants at 262-292 (`ForgetGravity` … `PWinGravity`, `NoValue` … `YNegative`) against this file's same-named items - this mirror is the *first* definition for every one of them.  `NativeRectangle`/`STORE_NATIVE_RECT` do **not** appear: two `macro_rules!` of one name in one scope shadow each other textually instead of erroring.  The fix is a project-level decision (a `cfg` on the platform-alternative items, or compiling the platform mirrors in a probe of their own the way `femacs/nt/inc` is handled by `spikes/f_inc_probe.rs`); this lane did not invent one.
