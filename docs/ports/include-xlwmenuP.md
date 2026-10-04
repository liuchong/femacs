# emacs/lwlib/xlwmenuP.h (158 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] _XlwMenuP_h  (line 22, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 1
- [x] extern XlwMenuClassRec xlwMenuClassRec  (line 156)
## Decisions

Mirror of `lwlib/xlwmenuP.h`, complete for this target: `window_state`, `XlwMenuPart`,
`XlwMenuRec`, `XlwMenuClassPart`, `XlwMenuClassRec` and the `xlwMenuClassRec` global.
`xlwmenuP.h` 24's `#include "xlwmenu.h"` is `include! ("xlwmenu.rs")`; C 25's
`#include <X11/CoreP.h>` has no mirror.

The frozen configuration builds no window system, no cairo and no i18n (`USE_CAIRO`,
`HAVE_XFT`, `HAVE_X_I18N`, `USE_LUCID`/`USE_X_TOOLKIT` and `HAVE_X11` are all `undef` in
docs/baseline/config/darwin-arm64.macros.tsv), so nothing includes this header and no probe
compiles it; the text gate is the whole acceptance (docs/status.md 3.31/3.102).

### Deviations

1. The X11 names (`Widget`, `Window`, `Pixmap`, `Position`, `Dimension`, `String`,
   `XFontStruct`, `Pixel`, `Cursor`, `XtCallbackList`, `GC`, `CorePart`, `CoreClassPart`,
   `XFontSet`, `XFontSetExtents`) and the cairo/Xft ones (`XftDraw`, `XftFont`, `XftColor`)
   have no mirror in the port; they keep C's spelling and are left for the using unit
   (PORTING.md P-26/P-72 (3)).
2. C 26-32's cairo/Xft `#if` pair and C 59-67's `HAVE_X_I18N`/cairo groups are dead in the
   frozen configuration and are carried as `#[cfg (feature = ...)]` field text (P-51), which
   keeps `xft_draw`, `fontSet`, `font_extents`, `default_face`, `xft_font` and the four
   `xft_*` colours in the mirror.  The feature names follow C's own conditions
   (`use_cairo_or_have_xft`, `have_x_i18n`), the `linux_or_android` shape.
3. `String` is X11's `char *` (C 68's `fontName`), and the name collides with the Rust
   prelude's `std::string::String`: the unit that eventually uses this header must define the
   alias (a module item shadows the prelude), or the field silently widens from a pointer to a
   24-byte `String` - a layout change no text gate can see.
4. Each `typedef struct _tag { ... } NAME;` is the tag struct plus `pub type NAME = _tag;`; the
   anonymous `XlwMenuClassPart` (C 143-146) is the struct named after its typedef.
5. `extern XlwMenuClassRec xlwMenuClassRec;` (C 156) is a mutable static declaration in the
   `extern` block (PORTING.md 0.1-2).

Evidence: `python3 tools/extract-abi.py --check emacs/lwlib/xlwmenuP.h
femacs/lwlib/include/xlwmenuP.rs` -> `OK` (2 ledger item(s)).
