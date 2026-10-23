# emacs/oldXMenu/XMenu.h (292 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 27
- [x] FAILURE  (line 41, body 7984b0a0e139)
- [x] NO_SELECTION  (line 44, body 7984b0a0e139)
- [x] POST_ERROR  (line 43, body 7984b0a0e139)
- [x] SUCCESS  (line 42, body 356a192b7913)
- [x] XME_ARG_BOUNDS  (line 55, body da4b9237bacc)
- [x] XME_CALLOC  (line 61, body fe5dbbcea5ce)
- [x] XME_CODE_COUNT  (line 51, body 0716d9708d32)
- [x] XME_CREATE_ASSOC  (line 62, body 0ade7c2cf97f)
- [x] XME_CREATE_CURSOR  (line 66, body bd307a3ec329)
- [x] XME_CREATE_TRANSP  (line 69, body 1574bddb75c7)
- [x] XME_CREATE_WINDOW  (line 68, body f1abd670358e)
- [x] XME_GRAB_MOUSE  (line 59, body c1dfd96eea8c)
- [x] XME_INTERP_LOC  (line 60, body 902ba3cda188)
- [x] XME_MAKE_PIXMAP  (line 65, body 7b52009b64fd)
- [x] XME_MAKE_TILES  (line 64, body 17ba0791499d)
- [x] XME_NOT_INIT  (line 54, body 356a192b7913)
- [x] XME_NO_ERROR  (line 53, body b6589fc6ab0d)
- [x] XME_OPEN_FONT  (line 67, body fa35e192121e)
- [x] XME_P_NOT_FOUND  (line 56, body 77de68daecd8)
- [x] XME_STORE_BITMAP  (line 63, body b1d5781111d8)
- [x] XME_STYLE_PARAM  (line 58, body ac3478d69a3c)
- [x] XME_S_NOT_FOUND  (line 57, body 1b6453892473)
- [x] XM_FAILURE  (line 46, body 7984b0a0e139)
- [x] XM_IA_SELECT  (line 49, body 77de68daecd8)
- [x] XM_NO_SELECT  (line 48, body da4b9237bacc)
- [x] XM_SUCCESS  (line 47, body 356a192b7913)
- [x] _XMenu_h_  (line 35, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 1
- [x] extern int _XMErrorCode  (line 74)
## Decisions

Mirror of `oldXMenu/XMenu.h`, complete for this target: the four result codes, the four XM_*
codes, the seventeen XME_* codes, the three window records, the two presentation enums, the
`XMenu` record, the two callback typedefs and the 23 prototypes.  `XMenu.h` 39's
`#include "X10.h"` is `include! ("X10.rs")`; C 37's `<stdlib.h>` and C 38's `<X11/Xutil.h>` are
noted where they stand (the header uses nothing from `<stdlib.h>`, and the port has no Xlib
mirror).

The frozen configuration builds no window system (`USE_X_TOOLKIT`, `HAVE_X11` and
`HAVE_X_WINDOWS` are `undef` in docs/baseline/config/darwin-arm64.macros.tsv), so nothing
includes this header and no probe compiles it; the text gate is the whole acceptance
(docs/status.md 3.31/3.102).

### Deviations

1. `Display`, `Window`, `Cursor`, `GC`, `XFontStruct`, `Pixmap` and `XEvent` are X11 names
   (`<X11/Xlib.h>`, `<X11/Xutil.h>`) with no mirror in the port; they keep C's spelling and are
   left for the using unit (PORTING.md P-26/P-72 (3)).
2. The ledger's "Macros defined" section prints `tools/cinventory.py`'s `macros` map alone, so
   the twelve enum members of `XMWType`, `XMStyle` and `XMMode` (C 89, 172-175, 186-188) are
   not checklist rows even though the `--check` constants rule compares them; all twelve are
   mirrored (PORTING.md 1.3).
3. Each `typedef struct _tag { ... } NAME;` is the tag struct plus `pub type NAME = _tag;`, so
   the self- and cross-references (`*mut _xmwindow`, `*mut _xmpane`, `*mut _xmselect`) keep
   C's spelling.
4. The nullable C function-pointer parameters (`Wait_func`, `Translate_func`, `XMenuEventHandler`'s
   handler and `XMenuActivate`'s `help_callback`) are `Option<unsafe extern "C" fn (...)>`
   (P-27).
5. `extern int _XMErrorCode;` and `extern char const *const _XMErrorList[];` (C 74-75) are an
   `extern` static and an incomplete array declared `[*const c_char; 0]`; the arrays are the
   `my_edata` shape.
6. C 1-19's opening declaration block is not part of the mirror (PORTING.md 0.1-19).

Evidence: `python3 tools/extract-abi.py --check emacs/oldXMenu/XMenu.h
oldXMenu/include/XMenu.rs` -> `OK` (28 ledger item(s)).
