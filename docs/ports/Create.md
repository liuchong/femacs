# emacs/oldXMenu/Create.c (752 lines)

> Filled in: the port is `oldXMenu/Create.rs` (**1291 lines**).  Until this round it was the
> empty unit that docs/status.md 3.31 prescribed; P-86 ① overturns that reading ("3 行的头注释不是
> '落完'，是'还没开始'"), and `oldXMenu/Activate.rs` + `oldXMenu/Internal.rs` are the siblings that
> already carry this directory's shape.

**File gate**: `feature = "have_x_windows"` (P-86 clause 8, the build's condition rather than one the
C text spells).  The reference text carries no macro of its own for this file: `oldXMenu/Makefile.in`
69-92's `OBJS` block lists every object of the directory unconditionally (this file's object is
`Create.o`, line 74), and it is `configure.ac` 7288-7296 that decides whether the library is built at
all (`7291`'s `elif test "$USE_X_TOOLKIT" = none` arm sets `HAVE_OLDXMENU=yes` at 7292 and
`LIBXMENU='$(oldXMenudir)/libXMenu11.a'` at 7293, on the way in from `HAVE_X11 = yes` + `HAVE_GTK !=
yes`).  `oldXMenu/Internal.rs` 8-16 records the same reading.

**The one sub-gate the C text spells is `EMACS_BITMAP_FILES` at C 54, and the reference's own build
decides it**: `oldXMenu/Makefile.in` 109-113's `ALL_CFLAGS` carries `-DEMACS_BITMAP_FILES` on line
112, and line 116's `.c.o:` rule is what compiles every object of the directory with it - so the live
arm is C 55-67 (the twelve in-tree `../src/bitmaps/*.xbm` includes) and C 69-85's `<X11/bitmaps/*>`
arm is dead for this library.  A *constant* condition, so it is not a `cfg` and the port carries the
live arm's twelve objects; the dead arm is recorded by range in the range table below, and the
`Deviations` section's third entry names it.  The two arms name the same twelve X11R4 bitmaps anyway
(`emacs/src/bitmaps/README` 9-20's correspondence table).

**Nothing in the frozen configuration compiles this unit**: `HAVE_X11`, `HAVE_X_WINDOWS` and
`USE_X_TOOLKIT` are `undef` in `docs/baseline/config/darwin-arm64.macros.tsv`, so `xtask build-tu`
leaves the 336-byte empty object (P-08), and this target has no X11 window system to run the arm
against - and no X11 *program* either.  The arm reading below is therefore a `rustc` reading of the
materialized mirror graph, not a build of the XMenu library: nothing here links or runs.

## C line ranges

The rows are contiguous and ascending; C's blank lines between the blocks need no row (the gate
`tools/check-dead-file-coverage.py` reads it that way).

| C | block | arm |
| --- | --- | --- |
| 1-48 | the file's opening block: the MIT notice (1-19), the FSF notice (23-37) and the `XMenuCreate` description comment (40-48) | dropped (PORTING.md 0.1-19, P-86 clause 1) |
| 50-52 | `#include "XMenuInt.h"` (50), `#include <stdlib.h>` (51), `#include <string.h>` (52) | the file gate; each mirror is `include!`d at its own C line, the `XMenuInt.h` one inside `mod xmenu_int` and glob-imported (`## Deviations` 8), and `<stdlib.h>`'s two missing names are declared at C 51's line (`## Owed` 2) |
| 54-85 | `#ifdef EMACS_BITMAP_FILES` (54) with the twelve in-tree `../src/bitmaps/*.xbm` includes (55-67), `#else` (69) with the twelve `<X11/bitmaps/*>` includes (71-83) and `#endif` (85) | the file gate; the condition is constant (`oldXMenu/Makefile.in` 112), so the live arm's twelve objects are declared in the unit (P-88 clause 1) and the dead arm is recorded here (`## Deviations` 3) |
| 87-106 | the sixteen `#define`s: the five menu defaults `DEF_FREEZE`…`DEF_INACT_PNUM` (87-91), `DEF_P_STYLE` (93), the four `DEF_P_*` (95-98), the five `DEF_S_*` (100-104) and `XASSOC_TABLE_SIZE` (106) | the file gate; each is a `pub const` of C's name |
| 108 | `const char *x_get_resource_string (char const *, char const *);` | the file gate; declared in-unit in an `unsafe extern "C"` block - `frame.h` 1888's own prototype is not reached from here (this file includes no `frame.h`) |
| 112-119 | `static Status XAllocDisplayColor (…)` - the `&&` chain over `XParseColor`/`XAllocColor` | the file gate; C's 1/0 result is `as c_int` (`## Deviations` 10) |
| 122-185 | `XMenuCreate`'s head: the locals (128-183) and the `Window root = RootWindow (display, DefaultScreen (display));` initialiser (185) | the file gate; C's uninitialised locals are zero-initialised (`## Deviations` 4), and the five Xlib macros are their entry points (`## Deviations` 2) |
| 187-199 | the two `calloc`s (`XMenu`, `XMPane`) and their `XME_CALLOC` returns | the file gate |
| 201-208 | `XCreateAssocTable (XASSOC_TABLE_SIZE)` and the `XME_CREATE_ASSOC` return | the file gate |
| 210-213 | the default environment name - `def_env == NULL \|\| *def_env == '\0'` then `"XMenu"` | the file gate; C assigns to its parameter, so the port's binding is `mut` (`## Deviations` 9) |
| 215-232 | the internal fail-safe defaults: `freeze`/`reverse`/`menu_style`/`menu_mode`/`inact_pnum` and the four `p_*`/`s_*` groups | the file gate |
| 234-260 | the `menuFreeze`, `menuReverseVideo`, `menuStyle` and `menuMode` resource lookups | the file gate |
| 262-284 | the `menuMouse` colour chain - four branches, C's three empty-statement bodies and its bare `else {}` | the file gate; each `XAllocDisplayColor` call used as a condition is `!= 0` |
| 286-306 | the `menuBackground` colour chain (the same four-branch shape, `black` before `white`) | the file gate |
| 308-315 | the `menuInactivePattern` lookup - the five pattern names to the five numbers | the file gate |
| 317-325 | the `paneStyle` and `paneFont` lookups | the file gate |
| 327-345 | the `paneForeground` colour chain - it has **no** trailing `else {}`; its third branch is a bare `else` | the file gate |
| 347-365 | the `paneBorder` colour chain | the file gate |
| 367-371 | `paneBorderWidth` (`atoi`) and `paneSpread` (`atof`) | the file gate; the two conversions are declared in-unit (`## Owed` 2) |
| 373-381 | the `selectionStyle` and `selectionFont` lookups | the file gate |
| 383-403 | the `selectionForeground` colour chain | the file gate |
| 406-426 | the `selectionBorder` colour chain | the file gate |
| 428-432 | `selectionBorderWidth` and `selectionSpread` | the file gate |
| 434-485 | the inactive-pattern pixmap block: its two locals (438-439), `switch (inact_pnum)` (441-472) picking one of the five `.xbm` objects, the `XME_STORE_BITMAP` test and `XCreatePixmapFromBitmapData` | the file gate; the `switch` is a `match` (`## Deviations` 6) |
| 487-563 | the mouse cursor: `switch (menu_style)` (491-559) with `XCreateBitmapFromData`/`XCreatePixmapCursor`/`XFreePixmap` per style, C 555-558's `default:` error arm, then the `mouse_cursor == _X_FAILURE` test | the file gate; the `switch` is a `match` and the test is `_X_FAILURE as Cursor` (`## Deviations` 6, 10) |
| 565-580 | the two `XLoadQueryFont` calls and their `XME_OPEN_FONT` returns | the file gate |
| 581-600 | the padding/height/offset arithmetic: the two `max_bounds` sums, the two `s_spread` products, `flag_height`, `p_height`/`p_y_off`/`p_x_off`, `s_height`/`s_y_off`/`s_x_off` | the file gate; C 586-587 read `s_spread` for *both* fonts (the reference's own text, `## Deviations` 7) and the `double`→`int` conversions are spelled |
| 602-613 | the pane list header (`next`/`prev`/`type`/`serial`) and `_XMWinQueInit ()` | the file gate |
| 615-684 | the four GCs: the `malloc` of the `XGCValues`, the pane GC, the normal and inverse selection GCs, the stipple bitmap, the inactive GC and `graphics_exposures = False` | the file gate; `XGCValues` is declared in the unit and the mirror's `XCreateGC` takes `*mut c_void` (`## Deviations` 11) |
| 687-745 | the `XMenu` object's field assignments - menu, pane, selection, colour and pixmap data | the file gate; the five `None`s are `x11_constants::None as Pixmap` (`## Deviations` 10) |
| 747-752 | `_XMErrorCode = XME_NO_ERROR; return (menu);` and the closing brace | the file gate |

## Evidence

1. **frozen reading** (the file gate off): `cd femacs && CARGO_TARGET_DIR=/tmp/create-lane/target
   cargo run -q -p xtask -- build-tu oldXMenu/Create.c` -> **exit 0**, `build/femacs/Create.o`
   **336 bytes**, `sha256 d1ea88b2fe3d960975d134e566e6593217e9ad67abdc2db5a451ea9620edd0ed` - the
   empty object the gate leaves (P-08; the same reading `docs/ports/oldXMenu-Internal.md` records
   for its file, byte for byte).
2. **arm flip, take 1** (`sha256 68e58aada08155d0986ac32b0e27ed2f2698f0cd06c005c2efe8ff3b4a6a9a16` is the
   final revision; take 1 is the first working text): `cargo run -q -p xtask -- probe
   oldXMenu/Create.rs /tmp/create-lane/create.rlib --crate-type=lib -C panic=abort --cfg
   'feature="have_x_windows"'` -> **exit 1, 25 errors, all of them in the unit's own text** (counted
   by file: every one names `build/probes/Create/_root.rs`, which *is* this unit's expansion): 17
   `E0425` for the three entry points that live in `include/xlib.rs`'s own module
   (`XCreateBitmapFromData` x7, `XFreePixmap` x6, `XCreateGC` x4 - the mirror's private
   `use xlib_surface::*;` is not re-exported at a unit's crate root, `## Deviations` 12), 4 `E0425`
   for the Xlib types the unit's own records spell (`Atom`, `Font` x2, `XExtData`), 2 `E0308` for the
   two `XCharStruct` sums (C's `short + short` promotes to `int`; the port spells it, `## Deviations`
   10) and 2 `free function without a body` for the two `<stdlib.h>` declarations that were written
   outside an `extern "C"` block.  All four classes are the unit's own text; no error was raised
   inside a mirror.
3. **arm flip, take 3** (the final revision): the same command with `-o
   /tmp/create-lane/create3.rlib` -> **exit 0, 0 errors, 78 warnings, a 320,192-byte rlib**
   (`sha256 ffaea07de2ecfd6da7f86af41ef5e2ae21c5edb4d71458d675c3359939a97b32`).  The three probes are
   take 1, take 2 (after the repairs above - the import list, the `extern` block and the two
   conversions: 0 errors) and take 3 on the revision that ships -
   the `&CStr` change of `DEF_P_FNT_NAME`/`DEF_S_FNT_NAME` (`## Deviations` 13) landed between take 2
   and take 3, so the shipped text has its own reading.  The 78 warnings are one class,
   `improper_ctypes` ("block uses type …, which is not FFI-safe"): the opaque `_XDisplay`/`_XGC`/…
   records and the unused Xlib entry points the mirror and this unit carry.  The file has no sub-gate
   (no `#if` other than the constant C 54), so the file gate is the whole set P-86 clause 8(6) asks
   for.
4. **the X11 surface is measured, not remembered** - `clang -I/opt/homebrew/include` on this machine
   (Apple clang 21.0.0.0, target arm64-apple-darwin25.6.0, xorgproto 2025.1, libX11 1.8.13):
   - `XGCValues` `[sizeof=128, align=8]`, `foreground` 16, `background` 24, `line_width` 32,
     `line_style` 36, `fill_style` 48, `stipple` 72, `font` 88, `graphics_exposures` 100,
     `dash_offset` 120 - every field the port carries is asserted.
   - `XFontStruct` `[sizeof=96, align=8]`, `fid` 8, `direction` 16, `all_chars_exist` 36,
     `properties` 48, `min_bounds` 56, `max_bounds` **68**, `per_char` 80, `ascent` 88, `descent`
     92; `XCharStruct` `[sizeof=12, align=2]`, `ascent` 6, `descent` 8; `XFontProp` `[sizeof=16]`,
     `card32` 8; `XColor` `[sizeof=16]`, `green` 10, `blue` 12, `flags` 14, `pad` 15; `Status` 4.
   - the masks and bits: `EnterWindowMask` 16, `LeaveWindowMask` 32, `ExposureMask` 32768,
     `GCForeground` 4, `GCBackground` 8, `GCFont` 16384, `GCLineWidth` 16, `GCFillStyle` 256,
     `GCStipple` 2048, `GCGraphicsExposures` 65536, `FillStippled` 2, `None` 0, `False` 0, `True` 1
     (`None` is the mirror's `x11_constants` row; the six others are this unit's, `## Owed` 1).
   - the ten entry points this unit declares are all exported:
     `nm -gU /opt/homebrew/lib/libX11.6.dylib` shows `_XRootWindow`, `_XDefaultScreen`,
     `_XDisplayCells`, `_XDefaultColormap`, `_XDisplayPlanes`, `_XParseColor`, `_XAllocColor`,
     `_XLoadQueryFont`, `_XCreatePixmapCursor`, `_XCreatePixmapFromBitmapData` (the three the mirror
     already carries - `_XCreateBitmapFromData`, `_XCreateGC`, `_XFreePixmap` - are included in the
     same reading).
5. **the twelve XBM objects are byte for byte the reference's**:
   `emacs/src/bitmaps/{dimple1,dimple3,gray1,gray3,crosswv,leftptr,leftpmsk,rtptr,rtpmsk,cntrptr,
   cntrpmsk,stipple}.xbm` were read back and compared with the port's twelve arrays - **0
   mismatches** (the twelve arrays hold 32/32/2/4/32/32/32/32/32/32/32/8 bytes), and the thirty
   `#define`s that head the same files - twelve widths, twelve heights, three `_x_hot` and three
   `_y_hot` - were compared the same way (30 values, 0 mismatches).  The bytes are *data*, not a
   behaviour: no bitmap was ever drawn.
6. **text gates**: `python3 tools/extract-abi.py --check emacs/oldXMenu/Create.c
   oldXMenu/Create.rs` -> the two constant findings and the `XMenuCreate` token finding are
   closed (the token one by the appended `docs/ports/macro-body-notes.tsv` row) and the one remaining
   row is the line-count drift, which is `docs/ports/line-drift-tolerance.tsv`'s and is not this
   lane's to write (`## Owed` 3); `python3 tools/check-declarations.py` -> **exit 0**;
   `python3 tools/check-mirror-dupes.py` -> **17 duplicate(s), 17 ratchet row(s), 0 failure(s)**;
   `python3 tools/check-citations.py` -> **1 drift, 0 range, 1812 external, 0 stale**, and that one
   drift is `src/w32image.rs` 76's `lisp.h 30` (another lane's file) - **this unit
   contributes 0 drift**; `python3 tools/check-dead-file-coverage.py` -> **139 ledger(s) bound, 0
   failure(s)**; `python3 tools/check-ledger-ticks.py` -> **0 failure(s)**.

## Hashes

| file | lines | md5 | sha256 |
| --- | ---: | --- | --- |
| `oldXMenu/Create.rs` (this revision) | 1291 | `4cc475f85a2868607b7bf0eeda10d979` | `68e58aada08155d0986ac32b0e27ed2f2698f0cd06c005c2efe8ff3b4a6a9a16` |
| `build/femacs/Create.o` (frozen reading) | - | - | `d1ea88b2fe3d960975d134e566e6593217e9ad67abdc2db5a451ea9620edd0ed` (336 bytes) |
| `/tmp/create-lane/create3.rlib` (arm reading, take 3) | - | - | `ffaea07de2ecfd6da7f86af41ef5e2ae21c5edb4d71458d675c3359939a97b32` (320,192 bytes) |

## Functions defined — 2

- [x] XAllocDisplayColor  (line 112, `static`)
- [x] XMenuCreate  (line 122)

## Macros defined — 16

- [x] DEF_FREEZE  (C 87) — [x] DEF_REVERSE  (C 88) — [x] DEF_MENU_STYLE  (C 89) —
  [x] DEF_MENU_MODE  (C 90) — [x] DEF_INACT_PNUM  (C 91)
- [x] DEF_P_STYLE  (C 93)
- [x] DEF_P_EVENTS  (C 95, `(EnterWindowMask | ExposureMask)`, carried narrowed to C's `int` use)
  — [x] DEF_P_FNT_NAME  (C 96, `"fixed"`) — [x] DEF_P_SPREAD  (C 97) — [x] DEF_P_BDR_WIDTH  (C 98)
- [x] DEF_S_STYLE  (C 100) — [x] DEF_S_EVENTS  (C 101) — [x] DEF_S_FNT_NAME  (C 102) —
  [x] DEF_S_SPREAD  (C 103) — [x] DEF_S_BDR_WIDTH  (C 104)
- [x] XASSOC_TABLE_SIZE  (C 106)

## Types and their fields — 0

The file declares no type of its own (the `XGCValues`/`XFontStruct`/… records of `## Owed` 1 are
`<X11/Xlib.h>`'s, and the twelve `.xbm` objects are their own files').

## File-scope objects — 12

The twelve `static char NAME_bits[]` objects of C 55-67's twelve includes, each carried as an
unexported `static mut [c_char; N]` and each compared byte for byte with its reference file
(Evidence 5):

- [x] dimple1_bits  (C 55) — [x] dimple3_bits  (C 56) — [x] gray1_bits  (C 57) —
  [x] gray3_bits  (C 58) — [x] cross_weave_bits  (C 59)
- [x] left_ptr_bits  (C 61) — [x] left_ptrmsk_bits  (C 62) — [x] right_ptr_bits  (C 63) —
  [x] right_ptrmsk_bits  (C 64) — [x] cntr_ptr_bits  (C 65) — [x] cntr_ptrmsk_bits  (C 66) —
  [x] stipple_bits  (C 67)

## Deviations

1. **The file gate is ours, not the reference's**: `oldXMenu/Makefile.in` 69-92's `OBJS` block lists
   the object unconditionally and `configure.ac` 7288-7296's arm decides whether the library exists
   at all (P-86 clause 8; `oldXMenu/Internal.rs` records the same reading).
2. **The five Xlib macros are their entry points**: `RootWindow`, `DefaultScreen`, `DisplayCells`,
   `DefaultColormap` and `DisplayPlanes` read Xlib's private display record (`_XPrivDisplay`,
   Xlib.h 85-119), which no mirror here carries, so each is an `#[inline (always)]` function over the
   entry point that reads the same field (`XRootWindow`, `XDefaultScreen`, `XDisplayCells`,
   `XDefaultColormap`, `XDisplayPlanes` - same parameter lists, all five measured present in
   libX11).  `include/xlib.rs`'s `DefaultScreenOfDisplay` and `oldXMenu/Internal.rs` 298-303 take the
   same reading.
3. **C 54's `#ifdef EMACS_BITMAP_FILES` is a constant condition, so only its live arm is carried**:
   `oldXMenu/Makefile.in` 109-113's `ALL_CFLAGS` defines it on line 112 for the line-116 rule that
   compiles the whole directory, so C 55-67 is the arm this library always builds and C 69-85 is dead
   text.  The port carries the live arm's twelve objects; the dead arm is recorded by range in
   `## C line ranges`.  (The two arms name the same twelve X11R4 bitmaps: `emacs/src/bitmaps/README`
   9-20.)
4. **C's uninitialised locals are zero-initialised** (PORTING.md 1.5): the four scalars, the twelve
   pointers, the GCs and the seven `XColor`s (`core::mem::zeroed ()` for the records).
5. **The twelve XBM byte literals are spelled `0xNNu8 as c_char`**: `c_char` is signed on this
   target and C's `'\xNN'` char literals with the high bit set are out of range for `i8` (measured:
   `error: literal out of range for i8`, with `0xf8` the worked value).  The bytes are C's; only the
   literal's spelling is the port's.
6. **C's two `switch`es are `match`es**: `switch (inact_pnum)` (C 441) and `switch (menu_style)`
   (C 491) become `match`es whose arms are the `case` labels (the first has **no** `default:`, so its
   `_ => { }` arm is the empty statement C's label-less switch falls out to; the second's `default:`
   at C 555 is the `_` arm).  The row is in `docs/ports/macro-body-notes.tsv` (C's `switch`/`case`/
   `default`/`break` tokens have no Rust site), and the residual - a *deleted* arm would be invisible
   to the token check - is named there.
7. **C 586-587 read `s_spread` for both fonts** (`p_fnt_pad = s_spread * p_fnt_height;`
   `s_fnt_pad = s_spread * s_fnt_height;`), which is the reference's own text and is carried as it
   stands; the pane's padding is therefore computed from the *selection* spread, not from
   `p_spread`.  Recorded as a reading, not a repair.
8. **The `XMenuInt.h` mirror is included inside `mod xmenu_int` and glob-imported** (the E0428 rule
   for a name that is both declared by a mirror and defined here: `XMenuCreate` is `XMenu.h` 266's
   own prototype) - the `oldXMenu/Internal.rs` 44-53 and `oldXMenu/Activate.rs` 46-59 shape.
9. **`register` has no Rust counterpart** (PORTING.md 1.2), and C 213 assigns to `def_env`, so the
   parameter's binding is `mut` - a Rust requirement with no C counterpart.
10. **C's integer conversions are spelled where C performs them**: `as c_int` on
    `XAllocDisplayColor`'s `&&` chain, on the two `XCharStruct` sums (C's `short + short` → `int`), on
    the two `double`→`int` products of C 586-587 and on `(p_y_off as f64 * p_spread)`; `as c_ulong`
    on the three `valuemask` ORs and the two `DEF_*_EVENTS` constants (C's `long` masks assigned to
    C's `int` fields); `as c_uint` on every width/height/depth argument; `_X_FAILURE as Cursor` (C
    compares C's `-1` against an `unsigned long` `Cursor`); `None as Pixmap` (C's `0L` into a
    `Pixmap`).
11. **`XGCValues` is declared in the unit, and the mirror's `XCreateGC` takes `*mut c_void`**: the
    mirror's note says "a record no ported arm reads" - true until this unit, which fills the record
    - so the four call sites cast (`values as *mut c_void`) and the record's measured layout is
    carried here (`## Owed` 1).  A `*mut XGCValues` parameter in the mirror would remove the four
    casts.
12. **The unit takes its Xlib names from the mirror's own module**: `include/xlib.rs` ends with a
    *private* `use xlib_surface::*;`, so a unit that includes the mirror does not get
    `Display`/`Window`/`XColor`/…/`XCreateBitmapFromData`/`XCreateGC`/`XFreePixmap` at its crate root;
    the port imports the eight types and the three entry points it needs from `xlib_surface::{…}`
    and the six constants from `xlib_surface`'s `x11_constants` module (PORTING.md 0.1-9's rule, the
    `oldXMenu/Activate.rs` 60-81 shape).  - `docs/ports/Activate.md`'s finding about
    `oldXMenu/Internal.rs`'s arm reading is the same root; this lane did not edit that file.
13. **`DEF_P_FNT_NAME`/`DEF_S_FNT_NAME` are `&core::ffi::CStr`**: `extract-abi.py`'s constant check
    compares a C string macro's *bytes* against a Rust string literal, so the constant carries the
    literal itself (`c"fixed"`) and the two use sites call `.as_ptr ()` - one spelling more than C's
    bare macro name.  This is the second take's change, and the final probe reading (Evidence 3)
    postdates it.
14. **C 1-48 is not transcribed**: the MIT/FSF notices and the file's description block - PORTING.md
    0.1-19, which the P-86 class keeps as the `// femacs: port of …` first line.

## Owed

1. **`include/xlib.rs` should carry the surface this unit declares** (integrator): the records
   (`XCharStruct`, `XFontProp`, `XFontStruct`, `XGCValues`), `Status`, `True`/`False`, the six GC
   bits (`GCFont`, `GCLineWidth`, `GCFillStyle`, `GCStipple`, `GCGraphicsExposures`, `FillStippled`)
   and the ten entry points, each with the measured layout of Evidence 4.  P-90 clause 1 wants one
   mirror for one header; the unit's copies are today's only home.  Two of them are *already* in the
   tree in another unit's text (`oldXMenu/Internal.rs` 105-183 carries the same `XCharStruct`/
   `XFontProp`/`XFontStruct` readings; `lwlib/xlwmenu.rs` 745-799 and `src/xfont.rs` 236-239 carry
   `XAllocColor`/`XLoadQueryFont`), so the move is a *dedupe* as much as a relocation.
2. **`lib/include/stdlib.rs` should carry `atoi` and `atof`** (integrator): C 51's `<stdlib.h>`
   mirror has `malloc`/`calloc`/`strtod`/`strtol`/… but not the two ASCII conversions this file is
   the first consumer of, so they are declared at C 51's line in the unit.  `lib/include/stdlib.rs`
   is not in this lane's write scope (`lib/include/**`).
3. **`docs/ports/line-drift-tolerance.tsv`**: the `Create` row for the line-count drift, which
   P-86 clause 10's second reading prints and which is not the porter's row to write.  Measured value
   **71.7%** (C 752 vs the port's 1291), so the row is `0.73` (`max (0.05, tolerance)`, the peers'
   margin convention); the reason text the row needs:

   ```
   Create	0.73	C 752 vs the port's 1291 (71.7%, the measured value + 0.01): the whole file is transcribed (the two definitions, both `switch`es as `match`es, the sixty resource-lookup steps) and the difference is the port's fixed costs - the `have_x_windows` gate on every item and on the whole include graph (P-86 (8)), the twelve XBM objects of C 55-67 with a measured width/height (and three hot spots) each, the in-unit `<X11/*>` surface this unit is the directory's first consumer of (four records with a measured offset per field, two scalars, six GC bits and ten entry points, each with a citation line; P-88 (1)), the `xlib_surface`/`x11_constants` imports the include dedupe forces, the fifty zero-initialised locals (PORTING.md 1.5), the `atoi`/`atof` declarations `lib/include/stdlib.rs` lacks, and C's comment blocks carried as prose	total
   ```
4. **`docs/port-map.md` and `docs/baseline/port-lines-pending.tsv`**: this file's map row still reads
   3 port lines and the ratchet table still carries its row; `tools/gen-port-map.py` and
   `tools/check-port-lines.py --write-pending` are the generators and both are the integrator's step
   (P-86 clause 4 - neither file is in this lane's write scope).  The map's *reference* count (752) is
   what `check-dead-file-coverage.py` reads and is unaffected.
5. **`xtask/Cargo.toml`**: nothing - `have_x_windows` is already registered in the generated
   `cfg` namespace, and this file's only sub-gate is a constant, so no second feature is needed.
   Recorded so the absence is a reading, not an omission.
6. **`XCreateGC`'s fourth parameter**: typing it `*mut XGCValues` in `include/xlib.rs` (with the
   record above) would remove this unit's four `as *mut c_void` casts; today the mirror's untyped
   pointer is what the older arms pass (`core::ptr::null_mut ()`, `src/image.rs` 1995).

## Not claimed

- **Nothing here runs**: the frozen configuration compiles none of the file, this machine has no X11
  program, and the arm flip is a *type check* of the materialized mirror graph (Evidence 2, 3) - not
  a menu that was created, a pixmap that was stored, a cursor that was loaded or a resource that was
  read.  The two functions have no behavioural evidence at all.
- **The dead `<X11/bitmaps/*>` arm has no reading**: this machine has no
  `/opt/homebrew/include/X11/bitmaps/` directory, so C 69-85's text was compared with nothing; the
  port's data came from the reference's own `src/bitmaps/*.xbm` (the arm the reference's Makefile
  selects) and the two arms' equivalence is the reference's README table, not a measurement here.
- **The X11 layout numbers are this machine's X11, not the frozen target's**: measured against
  `/opt/homebrew/include` (xorgproto 2025.1, libX11 1.8.13) rather than a target build, and the four
  records are checked only against that reading.
- **The XBM objects are data, not behaviour**: they are compared byte for byte with the reference
  files (Evidence 5), but no `XCreateBitmapFromData`/`XCreatePixmapFromBitmapData` call in this file
  ever executed, so nothing is claimed about how they would render.
- **The C 1-48 block is not carried** and this ledger records it by range rather than by text
  (PORTING.md 0.1-19); what the block *says* is therefore not part of the port's acceptance.
- **C 262-284, 286-306, 327-345, 347-365, 383-403 and 406-426's branch chains have no separate
  reading**: each is one `if`/`else if`/`else` chain here and the four-branch ones' empty bodies are
  C's empty statements, so "which colour won" is a text-level claim (the conditions are in the arms).
- **`docs/ports/Create.md` was the generated P-86 checklist** (0 items, "the port is an empty unit
  plus the note that names the gate") until this round; the sections above replace it, the
  `oldXMenu-SetAEQ`-class convention.
