# emacs/lwlib/lwlib-utils.h (63 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 8
- [x] XftDrawCreate  (line 48, body b7f358250ed0)
- [x] XftDrawDestroy  (line 51, body 0eed0a33f8e2)
- [x] XftDrawRect  (line 52, body 48a013579254)
- [x] XftDrawStringUtf8  (line 55, body 694df11e5791)
- [x] XftFontClose  (line 46, body e9516ff48a05)
- [x] XftFontOpenName  (line 44, body ec528b107afe)
- [x] XftTextExtentsUtf8  (line 58, body c91dc2b3d279)
- [x] _LWLIB_UTILS_H_  (line 2, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0
## Decisions

Mirror of `lwlib/lwlib-utils.h`, complete for this target: the widget-tree helpers and the whole
`USE_CAIRO` region.  The frozen configuration has `USE_CAIRO` and `HAVE_XRENDER` `undef`
(docs/baseline/config/darwin-arm64.macros.tsv), so this header's text is outside the frozen
units' checklists; the text gate was the whole acceptance when it landed (docs/status.md
3.31/3.102).  **Since 3.508 that is no longer the whole story**: `lwlib/lwlib-utils.rs`
includes this mirror under `use_lucid`, and its flip type-checks this file's eleven prototypes
(they now live in a `lwlib_utils_decls` module the unit globs, P-39 - the `lwlib-Xlw.rs` 3.500
shape, which is also what moved the `48-49` line numbers the `src/haikuterm.rs` note cites).

### Deviations

1. `Widget`, `XtPointer`, `Display`, `Drawable` and `Visual` are X11 names, and `cairo_t`,
   `cairo_scaled_font_t` and `FcChar8` are cairo/fontconfig names; the port has no mirror for
   any of those headers, so they keep C's spelling and are left for the using unit (PORTING.md
   P-26/P-72 (3)).
2. C 18-62's `#ifdef USE_CAIRO` region is dead in the frozen configuration and is carried as
   `#[cfg (feature = "use_cairo")]` items (P-51), so the ledger's seven Xft macros keep their
   tokens.  C 35-42's `#ifdef HAVE_XRENDER` pair is inside it; the `#else` arm's own
   `XGlyphInfo` is the one mirrored.
3. The two anonymous `typedef struct { ... }` records (`XftFont`, `XftColor`'s nested `color`
   member) are named after their typedef, and the nested one `<type>_<member>` (PORTING.md
   1.3/P-67).
4. The seven C macros are `macro_rules!` (PORTING.md 2.1) because their C use sites are calls.
   `XftFontClose`'s second C parameter is spelled `pub`, a Rust keyword, so the port's
   metavariable is `pub_` (the token compares as `pub`, PORTING.md 0.2).
5. The `crxft_*` entries C 45-60 declares keep their names as declarations; their definitions
   live in `lwlib-utils.c`'s cairo arm, which the frozen configuration does not build.

Evidence: `python3 tools/extract-abi.py --check emacs/lwlib/lwlib-utils.h
lwlib/include/lwlib-utils.rs` -> `OK` (8 ledger item(s)).
5. **The eleven prototypes moved into `lwlib_utils_decls` (3.508)** - P-39: `lwlib-utils.c` defines
   every one of them, so a top-level copy in the mirror is `E0428` in that unit (measured on
   `src/widget.rs`'s flip for `include/widget.rs`, status 3.496/3.504; the `lwlib-Xlw.rs` shape).
