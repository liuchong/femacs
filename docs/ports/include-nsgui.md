# emacs/src/nsgui.h (137 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 34
- [x] AllValues  (line 121, body 652902e8f0f1)
- [x] CONVERT_FROM_EMACS_RECT  (line 88, body 77e0856ac4c8)
- [x] CONVERT_TO_EMACS_RECT  (line 82, body 180b09a43555)
- [x] CenterGravity  (line 109, body ac3478d69a3c)
- [x] EastGravity  (line 110, body c1dfd96eea8c)
- [x] ForgetGravity  (line 104, body b6589fc6ab0d)
- [x] HeightValue  (line 120, body 92df71781c5c)
- [x] NativeRectangle  (line 80, body 8f8d043e3858)
- [x] NoValue  (line 116, body bd0d3415ccf6)
- [x] NorthEastGravity  (line 107, body 77de68daecd8)
- [x] NorthGravity  (line 106, body da4b9237bacc)
- [x] NorthWestGravity  (line 105, body 356a192b7913)
- [x] PAspect  (line 133, body f5a08b8b3015)
- [x] PBaseSize  (line 134, body 08b008cb3ba4)
- [x] PMaxSize  (line 131, body 2a0a8ece4dfd)
- [x] PMinSize  (line 130, body 7f2dac477a2d)
- [x] PPosition  (line 128, body b56fff76c324)
- [x] PResizeInc  (line 132, body b307cdcc94a2)
- [x] PSize  (line 129, body a33785b93413)
- [x] PWinGravity  (line 135, body 148030b1dc2b)
- [x] STORE_NATIVE_RECT  (line 94, body 4b4b98d63c8d)
- [x] SouthEastGravity  (line 113, body 0ade7c2cf97f)
- [x] SouthGravity  (line 112, body fe5dbbcea5ce)
- [x] SouthWestGravity  (line 111, body 902ba3cda188)
- [x] StaticGravity  (line 114, body b1d5781111d8)
- [x] USPosition  (line 125, body 1e2ed944d274)
- [x] USSize  (line 126, body 72fd47b601c4)
- [x] WestGravity  (line 108, body 1b6453892473)
- [x] WidthValue  (line 119, body eeed498aee81)
- [x] XNegative  (line 122, body d73afeb43f0b)
- [x] XValue  (line 117, body f2b1649be8f5)
- [x] YNegative  (line 123, body 19fbaf699b05)
- [x] YValue  (line 118, body 5f399b2ba212)
- [x] __NSGUI_H__  (line 20, body da39a3ee5e6b)

## Types and their fields — 1
- [x] _XCharStruct: int rbearing, int lbearing, int width, int ascent, int descent

## File-scope objects — 0

## Decisions

- **What is live and what is dead** (PORTING.md P-08/P-66; `tools/check-cond-eval.py src/nsgui.h`
  -> `10 conditions, 0 unknown to the evaluator, 0 disagreements`).  Dead on the frozen
  configuration: C 23-43, the whole `#ifdef __OBJC__` head - the `Cursor` -> `FooFoo` rename
  (C 29) with its `#undef` (C 35) and the three framework imports (`<AppKit/AppKit.h>` C 32,
  `<Foundation/NSDistantObject.h>` C 37, `<AvailabilityMacros.h>` C 40); C 56 (`typedef id
  Emacs_Pixmap;`) and C 62 (`typedef NSCursor *Emacs_Cursor;`), the Objective-C halves of the
  C 55-59 and C 61-65 pairs (the live halves are C 58 and C 64); and C 72-73 (`typedef float
  CGFloat;`), the `#else` of C 70's `#if defined (__LP64__) && __LP64__` (the live arm is C 71's
  `double`).  Live: C 46-53, C 58, C 64, C 67, C 70-71, C 75-78 and C 80-135.  C 19-20 is the
  include guard and C 137 its `#endif`; the reference has no trailing local-variables block.
  `HAVE_NS`/`NS_IMPL_COCOA` are undefined for this target, so no translation unit compiles the
  header at all - but its text is (almost) unconditional, so the mirror carries it (the
  `src/w32select.h` reading, status 3.158).  C's includers are `dispextern.h` 142-148 (behind
  `HAVE_NS`) and `macfont.m` 34, which is also why `Emacs_Cursor` and `NativeRectangle` have two
  definitions apiece in C (dispextern.h 175-177 and 178-180 are the `#ifndef` fallbacks the
  platform headers define over) - the two texts the port's single crate sees at once.
- **`typedef struct _XCharStruct { … } XCharStruct;` (C 46-53) is the C tag plus the typedef
  alias** (PORTING.md 1.3).  Unlike `haikugui.h`'s `haiku_char_struct` - whose typedef name sits
  after the closing brace on one line and which `cinventory` therefore does *not* see - this
  declaration is in the extractor's item list (`## Types and their fields — 1`), so its five fields
  are compared by `--check` rather than recorded by hand.
- **`NSPoint`/`NSSize`/`NSRect` (C 75-77) are C's own re-declaration for non-Objective-C
  translation units**, so the mirror declares the tags (`_NSPoint`, `_NSSize`, `_NSRect`) and
  aliases the typedef names instead of taking them from a framework header, exactly as C does;
  `_NSRect` embeds `NSPoint`/`NSSize` by value, which is C's layout.  **Blind spot (measured):**
  these three are written with the brace on the declaration's first line, which the extractor's
  struct scanner does not see, so no gate compares their fields - the same blind spot
  `include-haikugui.md` records.  `CGFloat` is `f64` (the `__LP64__` arm).
- **`NativeRectangle` (C 80) is a `macro_rules!` with no metavariables**, because C uses the name
  in *type* position (`NativeRectangle *nr`): a `pub type NativeRectangle = NSRect;` would be
  reported as `missing macro NativeRectangle` by `--check` (the `haikugui.rs` note).
- **`CONVERT_TO_EMACS_RECT`/`CONVERT_FROM_EMACS_RECT`/`STORE_NATIVE_RECT` (C 82-98)** are comma
  expressions in C; Rust has no comma operator, so each becomes a block of four assignments (the
  `haikugui.rs`/`dispextern.rs` shape).  The block's value is `()` where C's comma expression
  yields the last assigned value; the macros are used as statements and that is recorded here
  rather than hidden.
- **The constants keep C's types**: the eleven `*Gravity` and eight value-mask macros are `int`
  (`c_int`, the hex literals as written); the ten `*Position` macros are `c_long`, because C writes
  `(1L << n)` and PORTING.md 1.1 maps `long` to `c_long`.
- **This mirror cannot join `xtask layout-check`'s single crate.**  C keeps the X-compatibility
  block apart with its platform `#if`s and compiles one backend at a time; the probe compiles every
  `src/include` mirror at once, where `nsgui.rs`'s live `Emacs_Cursor` (C 64) collides with
  `dispextern.rs` 62 (dispextern.h 175, outside its X branches).  **Measured**: `layout-check` ->
  `exit 1`, one `error[E0428]: the name 'Emacs_Cursor' is defined multiple times`; with that single
  `include!` line removed from the generated materialised root, the same crate compiles and runs
  (`layout probe ok`, exit 0).  The `XCharStruct`/`Emacs_Pixmap`/`Window`/`NativeRectangle`/
  gravity/value/position names would collide with `haikugui.rs`/`androidgui.rs` too, but those two
  are already excluded by data.  **The integrator's action** (this lane may not edit the baseline or
  `xtask`): add `nsgui.rs` to `docs/baseline/layout-excluded-mirrors.txt` - the row's reason is
  "HAVE_NS/NS_IMPL_COCOA are undefined: nsgui.h is dead on this target, and its live X-compat
  names collide with dispextern.rs (Emacs_Cursor, measured) and with haikugui.rs/androidgui.rs".
  Compile evidence in the meantime is the NS probe below; nothing was papered over with a `cfg` or a
  `pub mod` wrapper (a wrapper would hide the `macro_rules!` names, which are textually scoped).

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/nsgui.h femacs/src/include/nsgui.rs`
  -> `OK emacs/src/nsgui.h -> femacs/src/include/nsgui.rs` (no `- ` problem lines).  The compared
  items are the header's 34 macros, the 19 constants they also define (the eleven `*Gravity` and
  eight value-mask families; the ten `*Position` macros have expression bodies and stay macros) and
  `_XCharStruct`'s five fields.
- `python3 tools/check-cond-eval.py src/nsgui.h` -> `10 conditions, 0 unknown to the evaluator,
  0 disagreements`.
- `python3 tools/extract-abi.py emacs/src/nsgui.h` -> this ledger: 0 subrs, 0 functions defined,
  34 macros, 1 type with fields, 0 file-scope objects.
- Compile proof: `spikes/ns_inc_probe.rs` (new this round, the minimal `spikes/f_inc_probe.rs`
  shape: the `c_*` aliases and a one-word `Lisp_Object` stub, then `include!`s of this mirror and
  `nsxwidget.rs`, which are the two halves of what a Cocoa translation unit sees).
  `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-ns cargo run -q -p xtask -- probe
  ../spikes/ns_inc_probe.rs /tmp/probes-ns/ns.rmeta` -> exit 0.  The probe is not in `xtask`'s
  `PROBES` list (this lane may not edit `xtask`); registering it is the integrator's call, together
  with the `layout-excluded-mirrors.txt` row above.
- `rg -n "Emacs_Cursor|NativeRectangle|XCharStruct" femacs/src/include/*.rs` -> six files:
  `androidgui.rs` (9 hits), `androidterm.rs` (19), `dispextern.rs` (2), `haikugui.rs` (7),
  `haikuterm.rs` (35) and this mirror (11).  Of those, only `dispextern.rs` is compiled by
  `layout-check` (the other four are already in `layout-excluded-mirrors.txt`), which is why the
  measured failure is exactly one `Emacs_Cursor`.
- No unit includes this mirror: `rg -n 'include/nsgui' femacs` -> no hits, so there is no
  consumer copy to retire (unlike 3.143/3.153) and no compiler other than the probe.  The name
  `NativeRectangle` does appear in `femacs/src/menu.rs` 87, `frame.rs` 467-468 and `keyboard.rs`
  633-634, but only inside comments quoting `dispextern.h` 178-179's `#ifndef NativeRectangle
  #define NativeRectangle int` fallback - no declaration, no use.
