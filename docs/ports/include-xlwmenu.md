# emacs/lwlib/xlwmenu.h (87 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 44
- [x] XmCBottomShadowColor  (line 72, body 9470414ae825)
- [x] XmCBottomShadowPixmap  (line 76, body 2187df89d58f)
- [x] XmCShadowThickness  (line 68, body b95398281105)
- [x] XmCTopShadowColor  (line 70, body fcc08811a782)
- [x] XmCTopShadowPixmap  (line 74, body c757996225ac)
- [x] XmNbottomShadowColor  (line 71, body d408965186a6)
- [x] XmNbottomShadowPixmap  (line 75, body dc914c9a5689)
- [x] XmNshadowThickness  (line 67, body b38ba0e0e583)
- [x] XmNtopShadowColor  (line 69, body e46464725b0d)
- [x] XmNtopShadowPixmap  (line 73, body fc6db32ab753)
- [x] XmRHorizontalDimension  (line 77, body e0a1c27c8f1e)
- [x] XtCAllowResize  (line 58, body 501294f7274f)
- [x] XtCBorderThickness  (line 60, body 11d89833f03e)
- [x] XtCButtonForeground  (line 36, body 016b6d06fede)
- [x] XtCDisabledForeground  (line 34, body a20dd1a455b2)
- [x] XtCHighlightBackground  (line 64, body d9fe7ce8dc6e)
- [x] XtCHighlightForeground  (line 62, body 907b268cb5c9)
- [x] XtCHorizontal  (line 50, body 297a940d25f3)
- [x] XtCMenu  (line 42, body c784aaa3477c)
- [x] XtCResizeToPreferred  (line 56, body 3c9595a00c00)
- [x] XtCShowGrip  (line 54, body 169dcd7c9740)
- [x] XtNCursor  (line 52, body 093ad7e659e7)
- [x] XtNallowResize  (line 57, body 0557fef1099a)
- [x] XtNarrowSpacing  (line 40, body 8a25713521a5)
- [x] XtNborderThickness  (line 59, body 90b0f78f5e7c)
- [x] XtNbuttonForeground  (line 35, body a1a43c7ea05c)
- [x] XtNcursor  (line 51, body 5320813c056f)
- [x] XtNdisabledForeground  (line 33, body c75e82b96dde)
- [x] XtNenterCallback  (line 46, body 305e44588438)
- [x] XtNhighlightBackground  (line 63, body c78ee4c34f17)
- [x] XtNhighlightCallback  (line 45, body 3e68ac80ab56)
- [x] XtNhighlightForeground  (line 61, body 233d74f4df5b)
- [x] XtNhorizontal  (line 49, body b3cc2863b795)
- [x] XtNhorizontalSpacing  (line 38, body 2427fee3835a)
- [x] XtNleaveCallback  (line 47, body 1664f456bfa3)
- [x] XtNmargin  (line 37, body 217825869ffb)
- [x] XtNmenu  (line 41, body 78ff6b89cd42)
- [x] XtNmenuBorderWidth  (line 48, body 84e571ab2850)
- [x] XtNopen  (line 43, body f5e5c4e542dd)
- [x] XtNresizeToPreferred  (line 55, body 68d557e08f21)
- [x] XtNselect  (line 44, body 96b45dfa08a8)
- [x] XtNshowGrip  (line 53, body 1b01e0922aa4)
- [x] XtNverticalSpacing  (line 39, body b28e54fcf01f)
- [x] _XlwMenu_h  (line 22, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 1
- [x] extern WidgetClass xlwMenuWidgetClass  (line 82)
## Decisions

Mirror of `lwlib/xlwmenu.h`, complete for this target: the 43 resource-name string constants,
the two widget typedefs, the class pointer and the two entry points.  `xlwmenu.h` 30's
`#include "lwlib.h"` is `include! ("lwlib.rs")`.

The frozen configuration builds no window system (`USE_LUCID`/`USE_X_TOOLKIT` and `HAVE_X11`
are `undef` in docs/baseline/config/darwin-arm64.macros.tsv), so nothing includes this header
and no probe compiles it; the text gate is the whole acceptance (docs/status.md 3.31/3.102).

### Deviations

1. `Widget`, `WidgetClass` and `Window` are X11 names with no mirror in the port; they keep C's
   spelling and are left for the using unit (PORTING.md P-26/P-72 (3)).
2. The 43 string-valued macros are `pub const NAME: &core::ffi::CStr = c"..."` (the string-macro
   rule of PORTING.md 2.1 / status 3.91), fully qualified because a mirror may not `use`
   `std::ffi::CStr` itself (it would clash with the including unit's prelude).
3. `typedef struct _XlwMenuRec *XlwMenuWidget;` (C 79) and `_XlwMenuClassRec` (C 80) name tags
   that `xlwmenuP.h` defines; the mirror names the tag and lets `xlwmenuP.rs` define it, so no
   stand-in is needed (P-70's closing rule: the definition arrives in the same batch).
4. `extern WidgetClass xlwMenuWidgetClass;` (C 82) is a mutable static in the `extern` block;
   the two prototypes are declarations only (PORTING.md 0.1-2).

Evidence: `python3 tools/extract-abi.py --check emacs/lwlib/xlwmenu.h
femacs/lwlib/include/xlwmenu.rs` -> `OK` (45 ledger item(s)).
