# emacs/lwlib/lwlib-int.h (75 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] LWLIB_INTERNAL_H  (line 22, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0
## Decisions

Mirror of `lwlib/lwlib-int.h`, complete for this target: the `widget_instance`, `widget_info`,
`widget_creation_function` and `widget_creation_entry` definitions, the `widget_xft_data`
stand-in, the three prototypes and `lwlib-int.h` 24's `#include "lwlib.h"`
(`include! ("lwlib.rs")`).

The frozen configuration builds no window system (`USE_CAIRO`, `HAVE_XFT` and `USE_X_TOOLKIT`
are `undef` in docs/baseline/config/darwin-arm64.macros.tsv), so nothing includes this header
and no probe compiles it; the text gate is the whole acceptance (docs/status.md 3.31/3.102).

### Deviations

1. `Widget`, `Boolean` and `XtPointer` are X11 names with no mirror in the port; they keep C's
   spelling and are left for the using unit (PORTING.md P-26/P-72 (3)).
2. `struct widget_xft_data;` (C 26) is an incomplete type C only points at, so the mirror gives
   it the zero-size `pub struct widget_xft_data;` stand-in (P-70).  The one field that names it
   (C 34) sits in the dead `USE_CAIRO`/`HAVE_XFT` arm.
3. C 33-35 (`#if defined USE_CAIRO || defined HAVE_XFT`) is dead in the frozen configuration and
   is carried as `#[cfg (feature = "use_cairo")]` text (P-51), which keeps `xft_data`'s token.
4. `typedef struct _widget_instance { ... } widget_instance;` (and the other three) is mirrored
   as the tag struct plus `pub type NAME = _tag;`, the `stdio-impl.rs` `FILE = __sFILE` shape,
   so the tag each field writes (`*mut _widget_instance`) keeps C's spelling.
5. `widget_creation_function` (C 55) is a nullable C function pointer -> `Option<unsafe extern
   "C" fn (...)>` (P-27).

Evidence: `python3 tools/extract-abi.py --check emacs/lwlib/lwlib-int.h
femacs/lwlib/include/lwlib-int.rs` -> `OK` (1 ledger item(s), the include guard).

## 2026-10-05: the tag's stand-in and the field's gate (3.517)

`struct widget_xft_data;` (C 26) is an incomplete type whose *owner* is `lwlib/lwlib-Xaw.c` 62-70 -
and that unit includes this mirror, so the ungated zero-size stand-in was `E0428` in it and its
cairo/Xft arms could not be built at all (the `lwlib-Xaw` lane measured it and had to prove the
unit's text with a `/tmp` preflight instead).  The stand-in and the `xft_data` field now carry the
same condition (`use_cairo_or_have_xft`, C 33-35's `#if defined USE_CAIRO || defined HAVE_XFT`),
so the stand-in exists exactly when no unit defines the tag; the field's own gate had said
`use_cairo` alone, which a `have_xft`-without-cairo build reads as `cannot find field xft_data`.
With both changes `lwlib/lwlib-Xaw.rs`'s cairo+xaw3d arm reads **exit 0, 0 error**.

## 2026-10-05: the four prototypes move into a module (P-39)

`lwlib/lwlib.c`'s port *defines* `lw_get_widget_info`, `lw_get_widget_instance`,
`lw_get_widget_value_for_widget` and `lw_internal_update_other_instances`, which this mirror declares
at its top level: declaration + `#[unsafe(no_mangle)]` definition in one scope is `E0428` (measured
on a 5-line probe).  The four now sit in `lwlib_int_decls` with `pub use lwlib_int_decls::*;` - the
`include/lwlib.rs` / `lwlib-Xlw.rs` shape - and the `pub use` is the half `lwlib/xlwmenu.c`'s flip
required, because that unit `include!`s this mirror **inside** its own `xlwmenu_decls` module and a
private glob import is not re-exported to a downstream glob (`docs/ports/lwlib.md` `## Evidence` ④
and `## Deviations` 13).

Evidence: `python3 tools/extract-abi.py --check emacs/lwlib/lwlib-int.h
femacs/lwlib/include/lwlib-int.rs` -> `OK`; `lwlib/lwlib-Xlw.rs` (5,240 bytes), `lwlib/lwlib-utils.rs`
(3,008 bytes) and `lwlib/xlwmenu.rs` (36,424 bytes) each re-read under their own gates with **exit 0
and 0 errors**, and `src/widget.rs`'s `use_x_toolkit` flip reports no error from either mirror
(docs/ports/lwlib.md `## Evidence` ④).
