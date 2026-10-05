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
`XModifierKeymap`, `XWMHints`, `XVisualInfo`, `XRectangle`, `XrmValue`, the display procedures
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
