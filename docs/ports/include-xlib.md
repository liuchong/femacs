# `femacs/src/include/xlib.rs` — the X11 SDK surface

**First line.** `// femacs: mirror of the X11 SDK's <X11/Xlib.h> (system header of another
target's toolchain, PORTING.md P-26/P-49).`  There is no C file to pair it with: `<X11/Xlib.h>` is
not in the reference tree and the frozen target has no such header.  `tools/check-target-names.py`
accepts the first line as the declaration; this page is the ledger the file's own head comment
refers to for the measurements.

**What it carries.** The Xlib surface the *gated* arms of `src/*.c`/`src/*.h` name: the scalar
typedefs (`XID`, `Mask`, `Atom`, `VisualID`, `Window`, `Drawable`, `Font`, `Pixmap`, `Cursor`,
`Colormap`, `GContext`, `KeySym`, `KeyCode`), the opaque records (`Display`, `GC`), the records
`XExtData`, `Visual`, `Depth`, `Screen`, `XImage` (with its nested `_XImage_funcs`), `XColor`,
`XModifierKeymap`, `XSizeHints` (with its named `XSizeHintsAspect`), `XWMHints`, `XVisualInfo`,
`XRectangle`, `XrmValue`, the display procedures
(`XCreateBitmapFromData`, `XReadBitmapFile`, `XFreePixmap`, `XGetImage`, `XPutImage`, `XCreateGC`,
`XFreeGC`) and the values `ZPixmap`, the six visual classes, the four `Bitmap*` results and the
eleven event masks.  `src/xrdb.c` 37-41 lists five headers for this one mirror
(`<X11/Xlib.h>`, `<X11/X.h>`, `<X11/Xatom.h>`, `<X11/Xutil.h>`, `<X11/Xresource.h>`): the macros
of the other three have no mirror of their own, so they live in the `x11_constants` module, each
row citing its own header.

**Three names that are macros in C, not entry points.** `XGetPixel`, `XPutPixel` and
`XDestroyImage` are defined in `Xutil.h`/`Xlib.h` and libX11 exports no symbol for them, so the
mirror spells them as `#[inline]` functions (a `macro_rules!` was tried first and rejected by
`E0423: a macro named XGetPixel exists in another namespace` — the units call them without `!`).

**Measured layouts** (re-run with this page's command; every number below is from it):

| record | sizeof | align | |
| --- | ---: | ---: | --- |
| `XRectangle` | 8 | 2 | `include/dispextern.rs` aliases `Emacs_Rectangle` to it under `have_x_windows` |
| `XExtData` | 32 | 8 | |
| `Visual` | 56 | 8 | |
| `Depth` | 16 | 8 | |
| `Screen` | 128 | 8 | |
| `XImage` | 136 | 8 | `data` @16, `f` @88 |
| `XColor` | 16 | 8 | `pixel` @0 |
| `XModifierKeymap` | 16 | 8 | |
| `XSizeHints` | 80 | 8 | `min_aspect` @48, `max_aspect` @56, `win_gravity` @72 |
| `XWMHints` | 56 | 8 | |
| `XVisualInfo` | 64 | 8 | `visualid` @8 |
| `XEvent` | 192 | 8 | **members owed** (see below) |
| `XrmValue` | 16 | 8 | |

Values: `ZPixmap` 2, `BitmapSuccess` 0, `StaticGray` 0, `TrueColor` 4.

```
$ printf '#include <stddef.h>\n#include <stdio.h>\n#include <X11/Xlib.h>\n#include <X11/Xutil.h>\n#include <X11/Xresource.h>\n...' > /tmp/p.c
$ clang -I/opt/homebrew/include -o /tmp/p /tmp/p.c && /tmp/p
```
Headers on this machine: `xorgproto` 2025.1 and `libX11` 1.8.13 under `/opt/homebrew/include`.

**Module shape.** The constants sit in `pub mod x11_constants` rather than at the top level: a
unit that includes the mirror wants C's `None`, and a top-level `None` would shadow Rust's
prelude `Option::None` for the whole unit, so a consumer imports the few names it needs where it
needs them (`include/xterm.rs` 144-155 does, with a targeted `use`; a glob there would drag
`None` into the unit's crate root).  The records and procedures sit in the glob-imported module
`xlib_surface`, because `src/image.rs` 232 has its own `pub type Pixmap = c_ulong;` — a crate-root
item shadows a glob import, so the unit's own item stays the live one (the `window.rs` 302-367
shape).

**Consumers.** `src/xrdb.c` (the first: its own five headers), `src/xterm.c`, `src/xfns.c`,
`src/image.rs` (the `have_x_windows` + `use_cairo` / `have_xrender` arms),
`include/dispextern.rs` (201: `Emacs_Rectangle = XRectangle`), `include/xterm.rs`.

**Owed (named, not claimed).**
1. `XEvent`'s 33 members: the record is declared with the measured 192/8 shape and its
   assertions, but the members are not written out; the only use site in the tree today is
   `include/xterm.rs` 789's `*mut XEvent`.  Landing them needs one reading per member (P-86's
   rule for a record nothing has touched yet).
2. The **XRender** surface (`XTransform`, `XFixed`, `XDoubleToFixed`, `XRenderSetPictureFilter`,
   `XRenderSetPictureTransform`, `FilterBest`, `FilterNearest`) — the
   `have_x_windows + have_xrender` configuration's text-only class
   (`docs/ports/image.md`'s owed list).

**Not claimed.** Nothing here was compiled against a real X server or a Windows/X11 target: the
numbers are this machine's clang readings of this machine's headers (the frozen target has no
`<X11/*.h>` at all, so the whole file is behind `have_x_windows`/`use_xcb` and contributes nothing
to the frozen legs).  No runtime, layout or symbol evidence beyond those measurements.

### status 3.496 (the mirror round this file took part in)

- The edge/declaration changes are recorded in `docs/status.md` 3.496: `include/dispextern.rs`'s
  `include!("xlib.rs")` edge lost its `#[cfg]` (the `xtask` dedupe gives the first edge the slot),
  `include/xlib.rs` widened `XRectangle`/`Window`/`XrmDatabase`/`XID` to
  `any (have_x_windows, have_pgtk)`, `include/pgtkterm.rs` fixed a dangling `///` (a syntax error
  that made every `have_pgtk` TU unparsable), its `pgtk_im_set_cursor_location` prototype and its
  two bare `atimer` tags, `include/font.rs` gained the `harfbuzz` arm of the `font_driver` size
  assertion (clang-measured 208/224), `include/xterm.rs`'s three INLINE bodies became `unsafe fn`,
  `include/gtk.rs` gained `GtkWidgetClass.get_preferred_height`, and `include/emacsgtkfixed.rs`
  moved `emacs_fixed_new` into a `_decls` module.  The two arm flips that were blocked now read
  **exit 0**: `src/pgtkim.rs` (`have_pgtk` + `have_gtk3`, 14,083,208-byte rlib) and
  `src/hbfont.rs` (`harfbuzz`, 14,580,952-byte rlib); `layout-check` passes and
  `tools/check-all-units.sh` is **83/0 + 83/0**.

### status 3.850 (slice 59's eight X11 event constants)

- `x_detect_focus_change`'s `switch (event->type)` and `x_focus_changed`'s two tests needed eight
  constants this module did not carry: `EnterNotify` (7), `LeaveNotify` (8), `FocusIn` (9),
  `FocusOut` (10), `NotifyGrab` (1), `NotifyUngrab` (2), `NotifyInferior` (2) and `NotifyPointer`
  (5).  Each is carried with `<X11/X.h>`'s own line (186, 187, 188, 189, 268, 269, 278, 281) and
  the value a clang probe of this target's header printed (the measurement is this slice's
  evidence in `docs/ports/xterm.md`); `ClientMessage` (33) was already here.
- The change is additive (eight `pub const`s inside the existing `x11_constants` module), so
  P-87 ①'s legs read **83/0 + 83/0** and `check-mirror-dupes` stays 16/16/0; the unit side of the
  reading is `docs/ports/xterm.md`'s slice-59 section.

### status 3.851 (slice 60's five masks and nine keysyms)

- `x_find_modifier_meanings`'s `switch (sym)` matches on nine keysyms and its tail compares two
  modifier masks, so this module gained `ShiftMask` (1), `LockMask` (2), `ControlMask` (4),
  `Mod1Mask` (8), `Mod2Mask` (16) - `<X11/X.h>` 221-225 - and `XK_Shift_Lock` (0xffe6),
  `XK_Meta_L` (0xffe7), `XK_Meta_R` (0xffe8), `XK_Alt_L` (0xffe9), `XK_Alt_R` (0xffea),
  `XK_Super_L` (0xffeb), `XK_Super_R` (0xffec), `XK_Hyper_L` (0xffed), `XK_Hyper_R` (0xffee) -
  `<X11/keysymdef.h>` 396-405 - each measured with clang against this target's header.  The masks
  keep this module's `c_long`; the keysyms are `c_int` because C's macros are plain int literals and
  the `match` they pattern on has `int sym` for a scrutinee.
- Additive, so P-87 ①'s legs read **83/0 + 83/0** and `check-mirror-dupes` stays 16/16/0; the unit
  side of the reading is `docs/ports/xterm.md`'s slice-60 section.
