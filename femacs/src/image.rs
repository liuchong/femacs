// femacs: port of src/image.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.
//
// **`src/image.c` is a P-86 unit.**  Its object is in `WINDOW_SYSTEM_OBJ`, which `configure.ac`
// 7521 fills only under `HAVE_WINDOW_SYSTEM`, `HAVE_WINDOW_SYSTEM` is `undef` in
// `docs/baseline/config/darwin-arm64.macros.tsv` (1678) and `docs/baseline/frozen-build-sources.txt`
// lists no `image.o`, so the frozen configuration compiles nothing from this file.  (`src/image.c`
// *is* in `docs/baseline/config/cond-eval-verified.txt` and its text is mostly ungated - the file
// itself is the gate - so `tools/extract-abi.py`'s checklist is *not* empty: it is the whole
// 13,805-line text, and `--check`'s reading counts out the remainder; `docs/ports/image.md`'s
// Evidence 6 is that reading.)  P-86 ① therefore applies: the file is
// *transcribed*, both arms of every `#if` are carried, and the file gate (`have_window_system`)
// goes on every item **and on C 22-65's include graph** (P-86 ⑧).  The nested gates are C's own
// macros lower-cased: `have_x_windows`, `use_cairo`, `have_ntgui`, `windowsnt`, `have_ns`,
// `have_pgtk`, `have_imagemagick`, `have_haiku`, `have_android`, `not_android_stubify`,
// `have_webp`, `have_gif`, `words_bigendian`.  `docs/ports/image.md` carries the C line-range
// table (P-86 ②), the readings of this round (`## Evidence`) and the per-`#if` verdicts
// (`## Deviations`).
//
// **This round's slice is C 1-1095** (the file is 13,805 lines), which ends at the `}` of
// `image_create_bitmap_from_file` (C 1095).  **The next live definition is `free_bitmap_record`
// (C 1099-1119)** - the next slice's first item.  The slice holds: the include graph and the
// per-platform macro/type sections (C 20-222), the forward-declaration block (C 224-246), the six
// `USE_CAIRO` pixel-container helpers (C 250-368), the two `HAVE_NS` pixel shims (C 374-386), the
// six bitmap accessors (C 403-496), the `HAVE_PGTK` Cairo-pattern builder (C 505-591),
// `image_create_bitmap_from_data` (C 597-735) and `image_create_bitmap_from_file` (C 757-1095).
//
// Names this slice calls that the mirrors it includes do not carry are declared at the use site
// with C's own line in a `/// C:` comment (PORTING.md P-55/P-72 rule 3, the `frame.rs` 4760-4800 /
// `androidmenu.rs` shape); `docs/ports/image.md` lists them as mirror gaps and names the file each
// belongs in.  Names the platform SDKs supply (`<X11/Xlib.h>`, cairo, `<windows.h>`, Haiku's
// `<Bitmap.h>`, the NDK) are declared the same way, with the SDK header as the citation - the
// `include/xlib.rs` / `include/jni.rs` / `lib/include/haiku_support.rs` precedent for a system
// header that has no file in the reference tree.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");                 // C 20: `#include <config.h>` (every unit includes it)

/* C 22-26: the five system headers, and C 36-39's four more.  Each has a mirror in
   `femacs/lib/include/`, and each is inside the file gate: the frozen configuration must not
   compile a single line of this unit (P-86 ⑧; the `androidmenu.rs` reading is what this
   catches - an include outside the gate fails the frozen build at once).  */
#[cfg (feature = "have_window_system")]
include!("../lib/include/errno.rs");           // C 22: `#include <errno.h>`
#[cfg (feature = "have_window_system")]
include!("../lib/include/fcntl.in.rs");        // C 23: `#include <fcntl.h>`
#[cfg (feature = "have_window_system")]
include!("../lib/include/math.rs");            // C 24 / C 36: `#include <math.h>`
#[cfg (feature = "have_window_system")]
include!("../lib/include/unistd.in.rs");       // C 25: `#include <unistd.h>`
#[cfg (feature = "have_window_system")]
include!("../lib/include/stdlib.rs");          // C 26: `#include <stdlib.h>`
#[cfg (feature = "have_window_system")]
include!("../lib/include/stdint.in.rs");       // C 37: `#include <stdint.h>` (uint32_t)
#[cfg (feature = "have_window_system")]
include!("../lib/include/c-ctype.rs");         // C 38: `#include <c-ctype.h>`
#[cfg (feature = "have_window_system")]
include!("../lib/include/flexmember.rs");     // C 39: `#include <flexmember.h>`

/* C 30-32: `#if defined HAVE_PNG / # include <png.h> / #endif`, ahead of `<setjmp.h>` "to work
   around bugs with older libpng" - C's own comment.  `<png.h>` has no file in the reference tree
   and no mirror, so the declarations this unit needs from it stand in the unit (P-88), and
   **this slice uses none of them**: the PNG loader is C 7xxx's, and its slice brings the
   declarations and the clang readings with it.  The gate is C's own condition.  */
/* C 31: `#include <png.h>`, inside C 30-32's `#if defined HAVE_PNG`.  No declaration of this
   slice's is taken from it, so the P-88 section this arm needs (`<png.h>` has no file in the
   reference tree and no mirror) is owed with the PNG slice and its clang readings.  */
/* C 34: `#include <setjmp.h>`, the register barrier's header.  Its names are `lisp.h`'s mirror's
   (`include/lisp.rs` 1060/2147: `jmp_buf` and `_setjmp`), which C 41 brings in below; this slice
   reads none of them (`image.c`'s one `setjmp` use is the PNG reader's).  The C line stays named
   here so the range table has its statement.  */

/* C 41-53: the reference headers.  `include/lisp.rs` reaches `<stdio.h>`, `<string.h>`,
   `<stdcountof.h>`, `<limits.h>` and `thread.h` the way C's `lisp.h` does, and
   `include/dispextern.rs`/`coding.rs`/`buffer.h` pull the rest; the transitive mirrors are listed
   after them in C's own order where one applies (Rust item order is irrelevant, but
   `macro_rules!` must precede its expansions).  */
#[cfg (feature = "have_window_system")]
include!("../lib/include/stdio.rs");           // sysstdio.h 24: `#include <stdio.h>`
#[cfg (feature = "have_window_system")]
include!("../lib/include/string.rs");          // C 902/951/1041: `strcmp`, `strrchr`
#[cfg (feature = "have_window_system")]
include!("../lib/include/limits.in.rs");       // C 295/636/790: `CHAR_BIT`
#[cfg (feature = "have_window_system")]
include!("../lib/include/stdcountof.rs");      // lisp.h 28: `#include <stdcountof.h>` (`countof`)
#[cfg (feature = "have_window_system")]
include!("include/lisp.rs");                   // C 41: `#include "lisp.h"`
#[cfg (feature = "have_window_system")]
include!("../lib/include/mini-gmp.rs");        // lisp.h's `bignum.h` chain (`bignum.rs`'s members)
#[cfg (feature = "have_window_system")]
include!("include/bignum.rs");
#[cfg (feature = "have_window_system")]
include!("include/character.rs");              // lisp.h's `CHARACTERP`/`MAX_CHAR` users
#[cfg (feature = "have_window_system")]
include!("include/frame.rs");                  // C 42: `#include "frame.h"`
#[cfg (feature = "have_window_system")]
include!("include/process.rs");                // C 43: `#include "process.h"`
#[cfg (feature = "have_window_system")]
include!("include/window.rs");                 // C 44: `#include "window.h"`
#[cfg (feature = "have_window_system")]
include!("include/itree.rs");                  // buffer.h 27: `#include "itree.h"`
#[cfg (feature = "have_window_system")]
include!("include/buffer.rs");                 // C 45: `#include "buffer.h"`
#[cfg (feature = "have_window_system")]
include!("include/systime.rs");                // C 49: `#include "systime.h"` (`Time`)
#[cfg (feature = "have_window_system")]
include!("include/coding.rs");                 // C 51: `#include "coding.h"`
#[cfg (feature = "have_window_system")]
include!("include/composite.rs");              // coding.h 284: `#include "composite.h"`
#[cfg (feature = "have_window_system")]
include!("include/dispextern.rs");             // C 46: `#include "dispextern.h"`
#[cfg (feature = "have_window_system")]
include!("include/blockinput.rs");             // C 47: `#include "blockinput.h"`
#[cfg (feature = "have_window_system")]
include!("include/sysstdio.rs");               // C 48: `#include "sysstdio.h"`
#[cfg (feature = "have_window_system")]
include!("include/termhooks.rs");              // C 52: `#include "termhooks.h"`
#[cfg (feature = "have_window_system")]
include!("include/font.rs");                   // C 53: `#include "font.h"`
#[cfg (feature = "have_window_system")]
include!("../lib/include/sys_stat.in.rs");     // C 55-57: `#include <sys/stat.h>`
#[cfg (feature = "have_window_system")]
include!("../lib/include/sys_types.in.rs");    // C 59-61: `#include <sys/types.h>`

/* C 50: `#include <epaths.h>` - a generated header; this slice reads none of its `PATH_*` values
   (the image *libraries*' load path is C 11xxx's), so the C line is named here and no value is
   carried (`src/xrdb.rs` 45-47 / `src/callproc.rs` keep the measured value their files use).  */

/* C 63-65: `#ifdef HAVE_WINDOW_SYSTEM / #include TERM_HEADER / #endif`.  C picks `TERM_HEADER`
   per configuration (configure.ac 7180) and it is one of the six platform terminal headers; the
   port has a mirror for each, so the arm keeps C's shape - one gated `include!` per platform,
   behind the same feature the mirror's own items use (P-86 ⑧).  */
#[cfg (all (feature = "have_window_system", feature = "have_x_windows"))]
include!("include/xterm.rs");                  // TERM_HEADER: `xterm.h`
#[cfg (all (feature = "have_window_system", feature = "have_ns"))]
include!("include/nsterm.rs");                 // TERM_HEADER: `nsterm.h`
#[cfg (all (feature = "have_window_system", feature = "have_pgtk"))]
include!("include/pgtkterm.rs");               // TERM_HEADER: `pgtkterm.h`
#[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
include!("include/w32term.rs");                // TERM_HEADER: `w32term.h`
#[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
include!("include/haikuterm.rs");              // TERM_HEADER: `haikuterm.h`
#[cfg (all (feature = "have_window_system", feature = "have_android"))]
include!("include/androidterm.rs");            // TERM_HEADER: `androidterm.h`

/* ----------------------------------------------------------------------------------------------
   dispextern.h 115-196: the per-platform `Display_Info`, `Emacs_Pix_Container` and
   `Emacs_Pix_Context` typedefs.  C gets them from `dispextern.h`; `include/dispextern.rs` does not
   carry them yet (the same gap `src/frame.rs` 6064-6085 records for `Display_Info`), so the six
   `Display_Info` arms are written here with `frame.rs`'s own text, and the pixel-container pair -
   which no unit has claimed - is added after them.  `docs/ports/image.md` reports both to their
   owner.
   ---------------------------------------------------------------------------------------------- */

/// C: dispextern.h 115: `typedef struct x_display_info Display_Info;`
#[cfg (all (feature = "have_window_system", feature = "have_x_windows"))]
pub type Display_Info = x_display_info;
/// C: dispextern.h 136: `typedef struct w32_display_info Display_Info;`
#[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
pub type Display_Info = w32_display_info;
/// C: dispextern.h 145: `typedef struct ns_display_info Display_Info;`
#[cfg (all (feature = "have_window_system", feature = "have_ns"))]
pub type Display_Info = ns_display_info;
/// C: dispextern.h 152: `typedef struct pgtk_display_info Display_Info;`
#[cfg (all (feature = "have_window_system", feature = "have_pgtk"))]
pub type Display_Info = pgtk_display_info;
/// C: dispextern.h 159: `typedef struct haiku_display_info Display_Info;`
#[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
pub type Display_Info = haiku_display_info;
/// C: dispextern.h 165: `typedef struct android_display_info Display_Info;`
#[cfg (all (feature = "have_window_system", feature = "have_android"))]
pub type Display_Info = android_display_info;

/* C 121-131: `#ifdef USE_CAIRO`'s "Minimal version of XImage" - an anonymous record, so the port
   names it after the object C's typedef names (PORTING.md 1.3, the `w32fns.rs` `kbdhook_s`
   reading).  The layout is C's own four members; `Clang`'s reading of the record is in
   `docs/ports/image.md`.  */
/// C: dispextern.h 122-131: `typedef struct { int width, height; char *data; int bytes_per_line; int bits_per_pixel; } *Emacs_Pix_Container;`
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
#[repr(C)]
pub struct emacs_pix_container
{
  pub width: c_int,                                   // C 124
  pub height: c_int,
  pub data: *mut c_char,                              // C 125
  pub bytes_per_line: c_int,                          // C 126
  pub bits_per_pixel: c_int,                          // C 127
}
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
pub type Emacs_Pix_Container = *mut emacs_pix_container;
/* The record's layout, measured with this machine's clang against `dispextern.h` 122-131's own
   text (`docs/ports/image.md`'s Evidence ④): sizeof **24**, align 8, `data` at 8,
   `bytes_per_line` at 16, `bits_per_pixel` at 20.  */
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
const _: () = assert! (core::mem::size_of::<emacs_pix_container> () == 24);
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
const _: () = assert! (core::mem::offset_of! (emacs_pix_container, data) == 8);
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
const _: () = assert! (core::mem::offset_of! (emacs_pix_container, bytes_per_line) == 16);
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
const _: () = assert! (core::mem::offset_of! (emacs_pix_container, bits_per_pixel) == 20);

/* C 136-139 and C 166: the other platforms' `Emacs_Pix_Container`, and C 192-196's
   `Emacs_Pix_Context`, which is the container itself unless a platform overrides it (w32 does, with
   `HDC`).  Each names the platform mirror's own record (P-72 rule 1: the definition stays in the
   mirror that owns it).  */
/// C: dispextern.h 137: `typedef XImage *Emacs_Pix_Container;` - `XImage` here is w32gui.h's
/// `struct _XImage`, and `include/w32gui.rs` 202 gates its alias on `windows` rather than on the
/// arm's own feature, so the member is the untyped pointer C's type is one word wide as (P-72
/// rule 3; the ledger reports the mirror's gate).
#[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
pub type Emacs_Pix_Container = *mut c_void;
/// C: dispextern.h 138: `typedef HDC Emacs_Pix_Context;` - `HDC` is `<windef.h>`'s, and
/// `include/w32gui.rs` 56 spells the same handle `*mut c_void`.
#[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
pub type Emacs_Pix_Context = *mut c_void;
/// C: nsgui.h 56: `typedef id Emacs_Pixmap;` - `id` is an Objective-C pointer, the degrade
/// `include/nsgui.rs` 54 spells `*mut c_void`.  `include/nsterm.rs`'s include list does not reach
/// `nsgui.h`'s mirror (C's `nsterm.h` includes it), so the declaration stands here and the ledger
/// asks for it in the mirror.
#[cfg (all (feature = "have_window_system", feature = "have_ns"))]
pub type Emacs_Pixmap = *mut c_void;
/// C: dispextern.h 146: `typedef Emacs_Pixmap Emacs_Pix_Container;`
#[cfg (all (feature = "have_window_system", feature = "have_ns"))]
pub type Emacs_Pix_Container = Emacs_Pixmap;
/// C: dispextern.h 160: `typedef Emacs_Pixmap Emacs_Pix_Container;`
#[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
pub type Emacs_Pix_Container = Emacs_Pixmap;
/// C: dispextern.h 166: `typedef struct android_image *Emacs_Pix_Container;`
#[cfg (all (feature = "have_window_system", feature = "have_android"))]
pub type Emacs_Pix_Container = *mut android_image;
/// C: dispextern.h 192-196: "Ordinarily an Emacs_Pix_Context is just an Emacs_Pix_Container;
/// C:   platforms can override this by defining the latter type and defining the conversion macro
/// C:   PIX_CONTAINER_TO_CONTEXT."
/// C: dispextern.h 196: `typedef Emacs_Pix_Container Emacs_Pix_Context;`
#[cfg (all (feature = "have_window_system", not (feature = "have_ntgui")))]
pub type Emacs_Pix_Context = Emacs_Pix_Container;

/* lisp.h's three declarations this slice calls that `include/lisp.rs` does not carry (the mirror
   gaps `src/fns.rs` 156 and `src/lread.rs` 1888 also record).  Each keeps C's own prototype in a
   `/// C:` line and belongs in that mirror (P-72 rule 4); `docs/ports/image.md` asks for the move
   and this unit drops its copies in the commit that makes it.  */
// C: lisp.h 4432: `extern AVOID memory_full (size_t);`
#[cfg (feature = "have_window_system")]
unsafe extern "C" {
  pub fn memory_full (size: size_t) -> !;
}
// C: lisp.h 5553: `extern char *xlispstrdup (Lisp_Object);`
#[cfg (feature = "have_window_system")]
unsafe extern "C" {
  pub fn xlispstrdup (string: Lisp_Object) -> *mut c_char;
}
// C: lisp.h 4743: `extern int openp (Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object *, Lisp_Object, bool, bool, void **);`
#[cfg (feature = "have_window_system")]
unsafe extern "C" {
  pub fn openp (path: Lisp_Object, str_: Lisp_Object, suffixes: Lisp_Object,
                storeptr: *mut Lisp_Object, predicate: Lisp_Object, newer: bool,
                no_native: bool, platform: *mut *mut c_void) -> c_int;
}

/* ----------------------------------------------------------------------------------------------
   C 67-222: the per-platform macro and type sections.  Each is C's own `#ifdef`/`#if`, spelled as
   the feature its macro lower-cases to, and both arms of every condition are carried.  Two are not
   a plain platform pair: C 147-151's `#else` (the `XBM_BIT_SHUFFLE` pair) and C 169-173's derived
   `COLOR_TABLE_SUPPORT`.

   Which names each arm reaches, and where they come from:

     * the X arm's `XCreateBitmapFromData`/`XReadBitmapFile`/`XFreePixmap` and `<X11/Xutil.h>`'s
       `BitmapSuccess` are `<X11/Xlib.h>`'s/Xutil's, and `Display`, `Screen`, `Pixmap` and
       `XGetPixel` are the same SDK's.  `include/xlib.rs` mirrors the surface `src/xrdb.c` needed
       and carries none of these (it says so: "`Display` and `Screen` are records this mirror does
       **not** define"), so the arm keeps C's tokens and is inert on this target - the
       `include/xterm.rs` 18-23 rule, and the ledger records the names.
     * the Windows arm's `GetPixel`, `CreateBitmap`, `COLORREF` and `HBITMAP` are `<windows.h>`'s,
       and this machine has no Windows toolchain at all (the ledger's Evidence), so the same rule
       applies.
     * the Haiku and Android arms' native calls *are* mirrored (`haiku_get_pixel`/`haiku_put_pixel`
       in `include/haikuterm.rs`, `android_put_pixel`/`android_get_pixel` and
       `android_create_bitmap_from_data` in `include/androidgui.rs`), and so are the bitmap records;
       the PGTK arm's `pgtk_bitmap_record` is **not** (`include/pgtkterm.rs` is a placeholder with
       no `include!` chain), which the ledger records as a mirror gap.
   ---------------------------------------------------------------------------------------------- */

/* --- C 67-77: `#ifdef HAVE_X_WINDOWS` --------------------------------------------------------- */
/// C: Xlib.h 79: `typedef unsigned long XID;` / Xlib.h 82: `typedef XID Pixmap;` - the X target's
/// own type, which `include/xlib.rs` does not carry yet (measured: `sizeof (Pixmap)` == **8**).
/// `include/xterm.rs` 155/192 already name it, so the declaration serves both.
#[cfg (all (feature = "have_window_system", feature = "have_x_windows"))]
pub type Pixmap = c_ulong;
/// C: xterm.h 186: `struct x_bitmap_record` - the arm's `Bitmap_Record`
#[cfg (all (feature = "have_window_system", feature = "have_x_windows"))]
pub type Bitmap_Record = x_bitmap_record;
/// C: C 70: `#define GET_PIXEL(ximg, x, y) XGetPixel (ximg, x, y)` - `XGetPixel` is Xlib's own
/// macro (Xlib.h 1123); this slice never expands this arm's copy (its one `GET_PIXEL` call is
/// C 324, the cairo arm).
#[cfg (all (feature = "have_window_system", all (feature = "have_x_windows", not (feature = "use_cairo"))))]
macro_rules! GET_PIXEL { ($ximg:expr, $x:expr, $y:expr) => { XGetPixel ($ximg, $x, $y) }; }
/// C: C 71: `#define PUT_PIXEL XPutPixel`.  An object-like `#define` of an identifier has no Rust
/// counterpart, so the port writes C's *use sites* as one `macro_rules!` per arm - which also keeps
/// C's token `PUT_PIXEL` in the ported bodies (see the ledger's Deviations).
#[cfg (all (feature = "have_window_system", all (feature = "have_x_windows", not (feature = "use_cairo"))))]
macro_rules! PUT_PIXEL { ($ximg:expr, $x:expr, $y:expr, $pixel:expr) => { XPutPixel ($ximg, $x, $y, $pixel) }; }
/// C: C 72: `#define NO_PIXMAP None` - Xlib's `None` (X11/X.h 115), which `include/xlib.rs`'s
/// `x11_constants` module carries (`xlib.rs` 48-52).
#[cfg (all (feature = "have_window_system", all (feature = "have_x_windows", not (feature = "use_cairo"))))]
macro_rules! NO_PIXMAP { () => { x11_constants::None } }
/// C: C 74: `#define PIX_MASK_RETAIN	0`
#[cfg (all (feature = "have_window_system", all (feature = "have_x_windows", not (feature = "use_cairo"))))]
macro_rules! PIX_MASK_RETAIN { () => { 0 } }
/// C: C 75: `#define PIX_MASK_DRAW	1`
#[cfg (all (feature = "have_window_system", all (feature = "have_x_windows", not (feature = "use_cairo"))))]
macro_rules! PIX_MASK_DRAW { () => { 1 } }

/* --- C 79-90: `#if defined (USE_CAIRO) || defined (HAVE_NS)` ---------------------------------- */
/// C: C 80: `#define RGB_TO_ULONG(r, g, b) (((r) << 16) | ((g) << 8) | (b))`
#[cfg (all (feature = "have_window_system", any (feature = "use_cairo", feature = "have_ns")))]
macro_rules! RGB_TO_ULONG {
  ($r:expr, $g:expr, $b:expr) => { ((($r) << 16) | (($g) << 8) | ($b)) };
}
/// C: C 82: `#define ARGB_TO_ULONG(a, r, g, b) (((a) << 24) | ((r) << 16) | ((g) << 8) | (b))`
#[cfg (all (feature = "have_window_system", all (any (feature = "use_cairo", feature = "have_ns"), not (feature = "have_ns"))))]
macro_rules! ARGB_TO_ULONG {
  ($a:expr, $r:expr, $g:expr, $b:expr) => { ((($a) << 24) | (($r) << 16) | (($g) << 8) | ($b)) };
}
/// C: C 84: `#define RED_FROM_ULONG(color)	(((color) >> 16) & 0xff)`
#[cfg (all (feature = "have_window_system", any (feature = "use_cairo", feature = "have_ns")))]
macro_rules! RED_FROM_ULONG { ($color:expr) => { ((($color) >> 16) & 0xff) }; }
/// C: C 85: `#define GREEN_FROM_ULONG(color)	(((color) >> 8) & 0xff)`
#[cfg (all (feature = "have_window_system", any (feature = "use_cairo", feature = "have_ns")))]
macro_rules! GREEN_FROM_ULONG { ($color:expr) => { ((($color) >> 8) & 0xff) }; }
/// C: C 86: `#define BLUE_FROM_ULONG(color)	((color) & 0xff)`
#[cfg (all (feature = "have_window_system", any (feature = "use_cairo", feature = "have_ns")))]
macro_rules! BLUE_FROM_ULONG { ($color:expr) => { (($color) & 0xff) }; }
/// C: C 87: `#define RED16_FROM_ULONG(color)		(RED_FROM_ULONG (color) * 0x101)`
#[cfg (all (feature = "have_window_system", any (feature = "use_cairo", feature = "have_ns")))]
macro_rules! RED16_FROM_ULONG { ($color:expr) => { (RED_FROM_ULONG ($color) * 0x101) }; }
/// C: C 88: `#define GREEN16_FROM_ULONG(color)	(GREEN_FROM_ULONG (color) * 0x101)`
#[cfg (all (feature = "have_window_system", any (feature = "use_cairo", feature = "have_ns")))]
macro_rules! GREEN16_FROM_ULONG { ($color:expr) => { (GREEN_FROM_ULONG ($color) * 0x101) }; }
/// C: C 89: `#define BLUE16_FROM_ULONG(color)		(BLUE_FROM_ULONG (color) * 0x101)`
#[cfg (all (feature = "have_window_system", any (feature = "use_cairo", feature = "have_ns")))]
macro_rules! BLUE16_FROM_ULONG { ($color:expr) => { (BLUE_FROM_ULONG ($color) * 0x101) }; }

/* --- C 92-102: `#ifdef USE_CAIRO` ------------------------------------------------------------
   cairo is a third-party library header with no file in the reference tree and no mirror, which is
   exactly P-88's case, so the declarations this slice takes from it stand here rather than in a new
   file under `femacs/src/include/`, and every constant and record is a **clang reading** from this
   machine's `/opt/homebrew/include/cairo/cairo.h` (cairo 1.18.4), quoted in the ledger.  */
/// C: C 100-101: `static unsigned long image_alloc_image_color (struct frame *, struct image *, Lisp_Object, unsigned long);`
/// C: C 2318: the definition, outside this slice (its declaration is kept here until then).
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
unsafe extern "C" {
  pub fn image_alloc_image_color (f: *mut frame, img: *mut image, color_name: Lisp_Object,
                                  color: c_ulong) -> c_ulong;
}
/// C: cairo.h 62-64: `typedef struct _cairo cairo_t;` / `typedef struct _cairo_surface cairo_surface_t;` / `typedef struct _cairo_pattern cairo_pattern_t;`
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
pub enum cairo_surface_t {}
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
pub enum cairo_pattern_t {}
/// C: cairo.h 216: `typedef enum _cairo_format { CAIRO_FORMAT_ARGB32, CAIRO_FORMAT_RGB24, CAIRO_FORMAT_A8, CAIRO_FORMAT_A1, ... } cairo_format_t;`
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
pub type cairo_format_t = c_int;
/// C: cairo.h 197: `typedef enum _cairo_status { CAIRO_STATUS_SUCCESS = 0, ... } cairo_status_t;`
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
pub type cairo_status_t = c_int;
/// C: cairo.h 246: `typedef enum _cairo_extend { CAIRO_EXTEND_NONE, CAIRO_EXTEND_REPEAT, ... } cairo_extend_t;`
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
pub type cairo_extend_t = c_int;
/// C: cairo.h 259: `typedef enum _cairo_filter { CAIRO_FILTER_FAST, ... } cairo_filter_t;`
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
pub type cairo_filter_t = c_int;
/// C: cairo.h 2691: `typedef void (*cairo_destroy_func_t) (void *);`
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
pub type cairo_destroy_func_t = Option<unsafe extern "C" fn (data: *mut c_void)>;
/// C: cairo.h 217-221 - measured: `CAIRO_FORMAT_ARGB32` **0**, `CAIRO_FORMAT_RGB24` **1**,
/// `CAIRO_FORMAT_A8` **2**, `CAIRO_FORMAT_A1` **3**
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
pub const CAIRO_FORMAT_ARGB32: cairo_format_t = 0;
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
pub const CAIRO_FORMAT_RGB24: cairo_format_t = 1;
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
pub const CAIRO_FORMAT_A8: cairo_format_t = 2;
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
pub const CAIRO_FORMAT_A1: cairo_format_t = 3;
/// C: cairo.h 197 - measured: `CAIRO_STATUS_SUCCESS` **0**
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
pub const CAIRO_STATUS_SUCCESS: cairo_status_t = 0;
/// C: cairo.h 247 - measured: `CAIRO_EXTEND_REPEAT` **1**
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
pub const CAIRO_EXTEND_REPEAT: cairo_extend_t = 1;
/// C: cairo.h 460-462: `typedef struct _cairo_matrix { double xx, yx, xy, yy, x0, y0; } cairo_matrix_t;`
/// - measured: sizeof **48**, align 8; offsets `xx` 0, `yx` 8, `xy` 16, `yy` 24, `x0` 32, `y0` 40
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct cairo_matrix_t
{
  pub xx: c_double,
  pub yx: c_double,
  pub xy: c_double,
  pub yy: c_double,
  pub x0: c_double,
  pub y0: c_double,
}
/// C: cairo.h 479-482: `typedef struct _cairo_user_data_key { int unused; } cairo_user_data_key_t;`
/// - measured: sizeof **4**
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct cairo_user_data_key_t
{
  pub unused: c_int,
}
/// C: cairo.h 2715: `cairo_public int cairo_format_stride_for_width (cairo_format_t, int);`
/// C: cairo.h 2723: `cairo_public cairo_surface_t *cairo_image_surface_create_for_data (unsigned char *, cairo_format_t, int, int, int);`
/// C: cairo.h 2620: `cairo_public cairo_status_t cairo_surface_set_user_data (cairo_surface_t *, const cairo_user_data_key_t *, void *, cairo_destroy_func_t);`
/// C: cairo.h 2930: `cairo_public cairo_pattern_t *cairo_pattern_create_for_surface (cairo_surface_t *);`
/// C: cairo.h 3060: `cairo_public void cairo_pattern_get_matrix (cairo_pattern_t *, cairo_matrix_t *);`
/// C: cairo.h 3057: `cairo_public void cairo_pattern_set_matrix (cairo_pattern_t *, const cairo_matrix_t *);`
/// C: cairo.h 3132: `cairo_public void cairo_pattern_set_filter (cairo_pattern_t *, cairo_filter_t);`
/// C: cairo.h 3135: `cairo_public cairo_filter_t cairo_pattern_get_filter (cairo_pattern_t *);`
/// C: cairo.h 2947: `cairo_public void cairo_pattern_destroy (cairo_pattern_t *);`
/// C: cairo.h 2496: `cairo_public void cairo_surface_destroy (cairo_surface_t *);`
/// C: cairo.h 2505: `cairo_public cairo_status_t cairo_surface_status (cairo_surface_t *);`
/// C: cairo.h 2666: `cairo_public void cairo_surface_mark_dirty (cairo_surface_t *);`
/// C: cairo.h 2663: `cairo_public void cairo_surface_flush (cairo_surface_t *);`
/// C: cairo.h 2727: `cairo_public unsigned char *cairo_image_surface_get_data (cairo_surface_t *);`
/// C: cairo.h 2744: `cairo_public int cairo_image_surface_get_stride (cairo_surface_t *);`
/// C: cairo.h 2710: `cairo_public cairo_surface_t *cairo_image_surface_create (cairo_format_t, int, int);`
/// C: cairo.h 3097: `cairo_public void cairo_pattern_set_extend (cairo_pattern_t *, cairo_extend_t);`
/// C: cairo.h 2953: `cairo_public cairo_status_t cairo_pattern_status (cairo_pattern_t *);`
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
unsafe extern "C" {
  pub fn cairo_format_stride_for_width (format: cairo_format_t, width: c_int) -> c_int;
  pub fn cairo_image_surface_create_for_data (data: *mut c_uchar, format: cairo_format_t,
                                              width: c_int, height: c_int, stride: c_int)
    -> *mut cairo_surface_t;
  pub fn cairo_surface_set_user_data (surface: *mut cairo_surface_t,
                                      key: *const cairo_user_data_key_t, user_data: *mut c_void,
                                      destroy: cairo_destroy_func_t) -> cairo_status_t;
  pub fn cairo_pattern_create_for_surface (surface: *mut cairo_surface_t) -> *mut cairo_pattern_t;
  pub fn cairo_pattern_get_matrix (pattern: *mut cairo_pattern_t, matrix: *mut cairo_matrix_t);
  pub fn cairo_pattern_set_matrix (pattern: *mut cairo_pattern_t, matrix: *const cairo_matrix_t);
  pub fn cairo_pattern_set_filter (pattern: *mut cairo_pattern_t, filter: cairo_filter_t);
  pub fn cairo_pattern_get_filter (pattern: *mut cairo_pattern_t) -> cairo_filter_t;
  pub fn cairo_pattern_destroy (pattern: *mut cairo_pattern_t);
  pub fn cairo_surface_destroy (surface: *mut cairo_surface_t);
  pub fn cairo_surface_status (surface: *mut cairo_surface_t) -> cairo_status_t;
  pub fn cairo_surface_mark_dirty (surface: *mut cairo_surface_t);
  pub fn cairo_surface_flush (surface: *mut cairo_surface_t);
  pub fn cairo_image_surface_get_data (surface: *mut cairo_surface_t) -> *mut c_uchar;
  pub fn cairo_image_surface_get_stride (surface: *mut cairo_surface_t) -> c_int;
  pub fn cairo_image_surface_create (format: cairo_format_t, width: c_int, height: c_int)
    -> *mut cairo_surface_t;
  pub fn cairo_pattern_set_extend (pattern: *mut cairo_pattern_t, extend: cairo_extend_t);
  pub fn cairo_pattern_status (pattern: *mut cairo_pattern_t) -> cairo_status_t;
}
/// C: cairo-xlib.h 56-60: `cairo_public cairo_surface_t *cairo_xlib_surface_create_for_bitmap (Display *, Pixmap, Screen *, int, int);`
/// `Display` and `Screen` are records `include/xlib.rs` does not define (its own note), so the two
/// parameters are the untyped pointer C's types are one word wide as (P-72 rule 3).
#[cfg (all (feature = "have_window_system", all (feature = "use_cairo", feature = "have_x_windows")))]
unsafe extern "C" {
  pub fn cairo_xlib_surface_create_for_bitmap (dpy: *mut c_void, bitmap: Pixmap,
                                               screen: *mut c_void, width: c_int,
                                               height: c_int) -> *mut cairo_surface_t;
}
/// C: C 93: `#define GET_PIXEL image_pix_context_get_pixel`
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
macro_rules! GET_PIXEL { ($ximg:expr, $x:expr, $y:expr) => { image_pix_context_get_pixel ($ximg, $x, $y) }; }
/// C: C 94: `#define PUT_PIXEL image_pix_container_put_pixel`
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
macro_rules! PUT_PIXEL { ($ximg:expr, $x:expr, $y:expr, $pixel:expr) => { image_pix_container_put_pixel ($ximg, $x, $y, $pixel) }; }
/// C: C 95: `#define NO_PIXMAP 0`
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
macro_rules! NO_PIXMAP { () => { 0 } }
/// C: C 97: `#define PIX_MASK_RETAIN	0`
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
macro_rules! PIX_MASK_RETAIN { () => { 0 } }
/// C: C 98: `#define PIX_MASK_DRAW	255`
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
macro_rules! PIX_MASK_DRAW { () => { 255 } }

/* --- C 104-111: `#if defined HAVE_PGTK && defined HAVE_IMAGEMAGICK` --------------------------- */
/// C: C 110: `# define DONT_CREATE_TRANSFORMED_IMAGEMAGICK_IMAGE` - a flag with no body, so the port
/// spells its *definedness* as a constant (the `XTERM_H` shape) for the `#ifdef` tests later in the
/// file.
#[cfg (all (feature = "have_window_system", all (feature = "have_pgtk", feature = "have_imagemagick")))]
pub const DONT_CREATE_TRANSFORMED_IMAGEMAGICK_IMAGE: bool = true;

/* --- C 113-124: `struct canvas` --------------------------------------------------------------- */
/// C: C 113-124: `struct canvas { struct canvas *next; uint32_t *data; uint32_t refresh; int width, height; };`
#[cfg (feature = "have_window_system")]
#[repr(C)]
pub struct canvas
{
  /* Linked list of canvases, see `canvas_list'.  */
  pub next: *mut canvas,
  /* Memory buffer in ARGB32 format.  The same format on all platforms!  */
  pub data: *mut uint32_t,
  /* Incremented if the canvas should be redrawn.  The value is always
     greater than 0.  */
  pub refresh: uint32_t,
  /* Dimension of the canvas.  */
  pub width: c_int,
  pub height: c_int,
}

/* C 126-127: `static void canvas_prepare_for_display (struct frame *f, struct image *img);` and
   `static void canvas_free_unused (void);` - `static` definitions outside this slice (C 2438 /
   C 2472).  C's prototype-plus-definition pair in one translation unit has no Rust counterpart, so
   the prototypes are dropped and the definitions will keep C's place (PORTING.md 0.1-12; the
   `w32fns.rs` 380-383 reading).  Nothing in this slice calls them.  */

/* --- C 129-151: `#ifdef HAVE_NTGUI` / `#else` ------------------------------------------------ */
/* C 131-135: "We need (or want) w32.h only when we're _not_ compiling for Cygwin." - the nested
   `#ifdef WINDOWSNT` include pair, gated exactly as C gates it (the `lcms.rs` C 36-38 shape).  */
#[cfg (all (feature = "have_window_system", feature = "have_ntgui", feature = "windowsnt"))]
include!("include/w32common.rs");              // C 133: `#include "w32common.h"`
#[cfg (all (feature = "have_window_system", feature = "have_ntgui", feature = "windowsnt"))]
include!("include/w32.rs");                    // C 134: `#include "w32.h"`

/// C: w32term.h 57: `struct w32_bitmap_record` - the arm's `Bitmap_Record`
#[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
pub type Bitmap_Record = w32_bitmap_record;
/// C: C 138: `#define GET_PIXEL(ximg, x, y) GetPixel (ximg, x, y)` - `GetPixel` is GDI's
/// (`<wingdi.h>`), which the Windows target supplies.
#[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
macro_rules! GET_PIXEL { ($ximg:expr, $x:expr, $y:expr) => { GetPixel ($ximg, $x, $y) }; }
/// C: C 139: `#define PUT_PIXEL XPutPixel` - this file's own C 7492.
#[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
macro_rules! PUT_PIXEL { ($ximg:expr, $x:expr, $y:expr, $pixel:expr) => { XPutPixel ($ximg, $x, $y, $pixel) }; }
/// C: C 140: `#define NO_PIXMAP 0`
#[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
macro_rules! NO_PIXMAP { () => { 0 } }
/// C: C 142: `#define PIX_MASK_RETAIN	0`
#[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
macro_rules! PIX_MASK_RETAIN { () => { 0 } }
/// C: C 143: `#define PIX_MASK_DRAW	1`
#[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
macro_rules! PIX_MASK_DRAW { () => { 1 } }
/// C: C 145: `#define XBM_BIT_SHUFFLE(b) (~(b))` - the `#ifdef HAVE_NTGUI` arm; C's `~` on the
/// `char` C 642-643 shuffles is Rust's `!` on the same type.
#[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
macro_rules! XBM_BIT_SHUFFLE { ($b:expr) => { !($b) }; }
/// C: C 149: `#define XBM_BIT_SHUFFLE(b) (b)` - C 147's `#else` arm, carried (P-51's "name the
/// non-frozen side": the feature is the *other* arm's, so this one is its negation).
#[cfg (all (feature = "have_window_system", not (feature = "have_ntgui")))]
macro_rules! XBM_BIT_SHUFFLE { ($b:expr) => { ($b) }; }

/* --- C 153-163: `#ifdef HAVE_NS` -------------------------------------------------------------- */
/// C: nsterm.h 810: `struct ns_bitmap_record` - the arm's `Bitmap_Record`
#[cfg (all (feature = "have_window_system", feature = "have_ns"))]
pub type Bitmap_Record = ns_bitmap_record;
/// C: C 156: `#define GET_PIXEL(ximg, x, y) XGetPixel (ximg, x, y)` - this file's own C 374.
#[cfg (all (feature = "have_window_system", feature = "have_ns"))]
macro_rules! GET_PIXEL { ($ximg:expr, $x:expr, $y:expr) => { XGetPixel ($ximg, $x, $y) }; }
/// C: C 157: `#define PUT_PIXEL XPutPixel` - this file's own C 382.
#[cfg (all (feature = "have_window_system", feature = "have_ns"))]
macro_rules! PUT_PIXEL { ($ximg:expr, $x:expr, $y:expr, $pixel:expr) => { XPutPixel ($ximg, $x, $y, $pixel) }; }
/// C: C 158: `#define NO_PIXMAP 0`
#[cfg (all (feature = "have_window_system", feature = "have_ns"))]
macro_rules! NO_PIXMAP { () => { 0 } }
/// C: C 160: `#define PIX_MASK_RETAIN	0`
#[cfg (all (feature = "have_window_system", feature = "have_ns"))]
macro_rules! PIX_MASK_RETAIN { () => { 0 } }
/// C: C 161: `#define PIX_MASK_DRAW	1`
#[cfg (all (feature = "have_window_system", feature = "have_ns"))]
macro_rules! PIX_MASK_DRAW { () => { 1 } }

/* --- C 165-167: `#ifdef HAVE_PGTK` ------------------------------------------------------------ */
/// C: pgtkterm.h 43: `struct pgtk_bitmap_record` - the arm's `Bitmap_Record`; the record is a mirror
/// gap (`include/pgtkterm.rs` is a placeholder with no `include!` chain), recorded in the ledger.
#[cfg (all (feature = "have_window_system", feature = "have_pgtk"))]
pub type Bitmap_Record = pgtk_bitmap_record;

/* --- C 169-173: the derived `COLOR_TABLE_SUPPORT` ---------------------------------------------
   C defines it only when `HAVE_X_WINDOWS` holds and `HAVE_NTGUI`, `USE_CAIRO` and `HAVE_NS` all do
   not, so the port spells that *condition* wherever C spells `#ifdef COLOR_TABLE_SUPPORT`, and the
   constant below is what the tests read (`HAVE_EXT_MENU_BAR`'s shape, `include/lisp.rs` 2913: a
   conditional define is a bool constant plus its own feature).  */
/// C: C 172: `# define COLOR_TABLE_SUPPORT 1`
#[cfg (all (feature = "have_window_system", all (feature = "have_x_windows",
            not (any (feature = "have_ntgui", feature = "use_cairo", feature = "have_ns")))))]
pub const COLOR_TABLE_SUPPORT: bool = true;

/* --- C 175-194: `#ifdef HAVE_HAIKU` ----------------------------------------------------------- */
/// C: haikuterm.h 52: `struct haiku_bitmap_record` - the arm's `Bitmap_Record`
#[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
pub type Bitmap_Record = haiku_bitmap_record;
/// C: C 179: `#define GET_PIXEL(ximg, x, y) haiku_get_pixel (ximg, x, y)` - the mirror's
/// `include/haikuterm.rs` 478.
#[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
macro_rules! GET_PIXEL { ($ximg:expr, $x:expr, $y:expr) => { haiku_get_pixel ($ximg, $x, $y) }; }
/// C: C 180: `#define PUT_PIXEL haiku_put_pixel` - the mirror's `include/haikuterm.rs` 479.
#[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
macro_rules! PUT_PIXEL { ($ximg:expr, $x:expr, $y:expr, $pixel:expr) => { haiku_put_pixel ($ximg, $x, $y, $pixel) }; }
/// C: C 181: `#define NO_PIXMAP 0`
#[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
macro_rules! NO_PIXMAP { () => { 0 } }
/// C: C 183: `#define PIX_MASK_RETAIN	0`
#[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
macro_rules! PIX_MASK_RETAIN { () => { 0 } }
/// C: C 184: `#define PIX_MASK_DRAW	1`
#[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
macro_rules! PIX_MASK_DRAW { () => { 1 } }
/// C: C 186: `#define RGB_TO_ULONG(r, g, b) (((r) << 16) | ((g) << 8) | (b))`
#[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
macro_rules! RGB_TO_ULONG {
  ($r:expr, $g:expr, $b:expr) => { ((($r) << 16) | (($g) << 8) | ($b)) };
}
/// C: C 187: `#define RED_FROM_ULONG(color)	(((color) >> 16) & 0xff)`
#[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
macro_rules! RED_FROM_ULONG { ($color:expr) => { ((($color) >> 16) & 0xff) }; }
/// C: C 188: `#define GREEN_FROM_ULONG(color)	(((color) >> 8) & 0xff)`
#[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
macro_rules! GREEN_FROM_ULONG { ($color:expr) => { ((($color) >> 8) & 0xff) }; }
/// C: C 189: `#define BLUE_FROM_ULONG(color)	((color) & 0xff)`
#[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
macro_rules! BLUE_FROM_ULONG { ($color:expr) => { (($color) & 0xff) }; }
/// C: C 190: `#define RED16_FROM_ULONG(color)		(RED_FROM_ULONG (color) * 0x101)`
#[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
macro_rules! RED16_FROM_ULONG { ($color:expr) => { (RED_FROM_ULONG ($color) * 0x101) }; }
/// C: C 191: `#define GREEN16_FROM_ULONG(color)	(GREEN_FROM_ULONG (color) * 0x101)`
#[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
macro_rules! GREEN16_FROM_ULONG { ($color:expr) => { (GREEN_FROM_ULONG ($color) * 0x101) }; }
/// C: C 192: `#define BLUE16_FROM_ULONG(color)		(BLUE_FROM_ULONG (color) * 0x101)`
#[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
macro_rules! BLUE16_FROM_ULONG { ($color:expr) => { (BLUE_FROM_ULONG ($color) * 0x101) }; }

/* --- C 196-222: `#ifdef HAVE_ANDROID` ---------------------------------------------------------
   C 197's `#include "androidterm.h"` is the file C 63-65's `TERM_HEADER` already brought in under
   `have_android`, so it is C's include-guard no-op here (the mirror is in the graph once).  */
/// C: androidterm.h 30: `struct android_bitmap_record` - the arm's `Bitmap_Record`
#[cfg (all (feature = "have_window_system", feature = "have_android"))]
pub type Bitmap_Record = android_bitmap_record;
/// C: C 201: `typedef struct android_image XImage;` - the arm's own `XImage`, not X11's.
#[cfg (all (feature = "have_window_system", feature = "have_android"))]
pub type XImage = android_image;
/// C: C 202: `typedef android_pixmap Pixmap;`
#[cfg (all (feature = "have_window_system", feature = "have_android"))]
pub type Pixmap = android_pixmap;
/// C: C 204: `#define GET_PIXEL(ximg, x, y) 		android_get_pixel (ximg, x, y)`
#[cfg (all (feature = "have_window_system", feature = "have_android"))]
macro_rules! GET_PIXEL { ($ximg:expr, $x:expr, $y:expr) => { android_get_pixel ($ximg, $x, $y) }; }
/// C: C 205: `#define PUT_PIXEL(ximg, x, y, pixel)    android_put_pixel (ximg, x, y, pixel)`
#[cfg (all (feature = "have_window_system", feature = "have_android"))]
macro_rules! PUT_PIXEL { ($ximg:expr, $x:expr, $y:expr, $pixel:expr) => { android_put_pixel ($ximg, $x, $y, $pixel) }; }
/// C: C 206: `#define NO_PIXMAP			0`
#[cfg (all (feature = "have_window_system", feature = "have_android"))]
macro_rules! NO_PIXMAP { () => { 0 } }
/// C: C 208: `#define PIX_MASK_RETAIN	0`
#[cfg (all (feature = "have_window_system", feature = "have_android"))]
macro_rules! PIX_MASK_RETAIN { () => { 0 } }
/// C: C 209: `#define PIX_MASK_DRAW	1`
#[cfg (all (feature = "have_window_system", feature = "have_android"))]
macro_rules! PIX_MASK_DRAW { () => { 1 } }
/// C: C 211: `#define RGB_TO_ULONG(r, g, b) (((r) << 16) | ((g) << 8) | (b))`
#[cfg (all (feature = "have_window_system", feature = "have_android"))]
macro_rules! RGB_TO_ULONG {
  ($r:expr, $g:expr, $b:expr) => { ((($r) << 16) | (($g) << 8) | ($b)) };
}
/// C: C 212: `#define RED_FROM_ULONG(color)	(((color) >> 16) & 0xff)`
#[cfg (all (feature = "have_window_system", feature = "have_android"))]
macro_rules! RED_FROM_ULONG { ($color:expr) => { ((($color) >> 16) & 0xff) }; }
/// C: C 213: `#define GREEN_FROM_ULONG(color)	(((color) >> 8) & 0xff)`
#[cfg (all (feature = "have_window_system", feature = "have_android"))]
macro_rules! GREEN_FROM_ULONG { ($color:expr) => { ((($color) >> 8) & 0xff) }; }
/// C: C 214: `#define BLUE_FROM_ULONG(color)	((color) & 0xff)`
#[cfg (all (feature = "have_window_system", feature = "have_android"))]
macro_rules! BLUE_FROM_ULONG { ($color:expr) => { (($color) & 0xff) }; }
/// C: C 215: `#define RED16_FROM_ULONG(color)		(RED_FROM_ULONG (color) * 0x101)`
#[cfg (all (feature = "have_window_system", feature = "have_android"))]
macro_rules! RED16_FROM_ULONG { ($color:expr) => { (RED_FROM_ULONG ($color) * 0x101) }; }
/// C: C 216: `#define GREEN16_FROM_ULONG(color)	(GREEN_FROM_ULONG (color) * 0x101)`
#[cfg (all (feature = "have_window_system", feature = "have_android"))]
macro_rules! GREEN16_FROM_ULONG { ($color:expr) => { (GREEN_FROM_ULONG ($color) * 0x101) }; }
/// C: C 217: `#define BLUE16_FROM_ULONG(color)		(BLUE_FROM_ULONG (color) * 0x101)`
#[cfg (all (feature = "have_window_system", feature = "have_android"))]
macro_rules! BLUE16_FROM_ULONG { ($color:expr) => { (BLUE_FROM_ULONG ($color) * 0x101) }; }
/* C 219-221: "DPYINFO->n_planes is unsuitable for this file, because it accepts values that may
   not be supported for pixmap creation."  `#define n_planes n_image_planes` renames a *field*,
   which Rust cannot spell, so the arm's text is written the way C's preprocessor leaves it (the
   `lcms.rs` 82-88 rule).  This slice writes no `n_planes` at all - the read is C 11219's.  */

/* ----------------------------------------------------------------------------------------------
   C 224-246: the forward-declaration block.  C's `static` prototype + definition pairs in one
   translation unit have no Rust counterpart (PORTING.md 0.1-12), so every one of them is dropped
   here and its definition keeps C's place in the slice that reaches it.
   ---------------------------------------------------------------------------------------------- */

/* C 224-229: `image_disable_image`/`image_edge_detection`/`image_compute_scale` - `static`,
   defined outside this slice, called by nothing here: dropped.  */
/* C 231-232: `init_color_table`/`lookup_rgb_color` - `static`: dropped.  */
/* C 233-236: `#ifdef COLOR_TABLE_SUPPORT`'s `free_color_table`/`colors_in_color_table` - `static`,
   under C 169-173's derived condition: dropped.  */
/* C 238-242: `#ifdef HAVE_NTGUI`'s two `static` prototypes - `w32_create_pixmap_from_bitmap_data`
   (C 239, defined at C 4940) and `XPutPixel` (C 240, defined at C 7492).  Both are called by this
   slice's Windows arm, so the arm reaches names whose definitions are the *next* slices'; that is
   recorded in `docs/ports/image.md` rather than papered over with a stub (the arm is text-only on
   this machine anyway - no Windows toolchain).  */
/// C: C 244-246: `#if defined (HAVE_WEBP) || defined (HAVE_GIF) / static void anim_prune_animation_cache (Lisp_Object); / #endif` -
/// a `static` whose definition is C 9184's; nothing here calls it, so C's prototype is dropped.

/* ----------------------------------------------------------------------------------------------
   C 248-370: `#ifdef USE_CAIRO`'s pixel-container helpers.
   ---------------------------------------------------------------------------------------------- */

/// C: C 250-252: `static Emacs_Pix_Container image_create_pix_container (unsigned int width, unsigned int height, unsigned int depth)`
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
unsafe fn image_create_pix_container (width: c_uint, height: c_uint, depth: c_uint)
  -> Emacs_Pix_Container
{
  let mut pimg: Emacs_Pix_Container;

  pimg = xmalloc (core::mem::size_of::<emacs_pix_container> ()) as Emacs_Pix_Container;
  (*pimg).width = width as c_int;
  (*pimg).height = height as c_int;
  /* C's `depth == 1 ? 8 : 32` (Rust has no `?:`, PORTING.md 2.1).  */
  (*pimg).bits_per_pixel = if depth == 1 { 8 } else { 32 };
  (*pimg).bytes_per_line = cairo_format_stride_for_width ((if depth == 1
                                                             { CAIRO_FORMAT_A8 }
                                                           else { CAIRO_FORMAT_RGB24 }),
                                                          width as c_int);
  (*pimg).data = xmalloc (((*pimg).bytes_per_line as c_uint * height) as usize) as *mut c_char;

  return pimg;
}

/// C: C 269-271: `static void image_pix_container_put_pixel (Emacs_Pix_Container image, int x, int y, unsigned long pixel)`
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
unsafe fn image_pix_container_put_pixel (image: Emacs_Pix_Container, x: c_int, y: c_int,
                                         pixel: c_ulong)
{
  if (*image).bits_per_pixel == 32
    {
      *(((*image).data.offset ((y * (*image).bytes_per_line) as isize)) as *mut uint32_t)
        .offset (x as isize) = pixel as uint32_t;
    }
  else
    {
      *(((*image).data.offset ((y * (*image).bytes_per_line) as isize)) as *mut uint8_t)
        .offset (x as isize) = pixel as uint8_t;
    }
}

/// C: C 279-280: `static unsigned long image_pix_context_get_pixel (Emacs_Pix_Context image, int x, int y)`
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
unsafe fn image_pix_context_get_pixel (image: Emacs_Pix_Context, x: c_int, y: c_int) -> c_ulong
{
  if (*image).bits_per_pixel == 32
    {
      return *(((*image).data.offset ((y * (*image).bytes_per_line) as isize)) as *mut uint32_t)
        .offset (x as isize) as c_ulong;
    }
  else
    {
      return *(((*image).data.offset ((y * (*image).bytes_per_line) as isize)) as *mut uint8_t)
        .offset (x as isize) as c_ulong;
    }
}

/// C: C 288-292: `static Emacs_Pix_Container image_pix_container_create_from_bitmap_data (char *data, unsigned int width, unsigned int height, unsigned long fg, unsigned long bg)`
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
unsafe fn image_pix_container_create_from_bitmap_data (data: *mut c_char, width: c_uint,
                                                      height: c_uint, fg: c_ulong, bg: c_ulong)
  -> Emacs_Pix_Container
{
  let mut pimg: Emacs_Pix_Container = image_create_pix_container (width, height, 0);
  let mut data = data;
  let bytes_per_line: c_int = ((width + (CHAR_BIT as c_uint - 1)) / CHAR_BIT as c_uint) as c_int;

  let mut y: c_uint = 0;
  while y < height
    {
      let mut x: c_uint = 0;
      while x < width
        {
          PUT_PIXEL! (pimg, x as c_int, y as c_int,
                      if (*data.offset ((x / CHAR_BIT as c_uint) as isize)
                          >> (x % CHAR_BIT as c_uint)) & 1 != 0
                        { fg } else { bg });
          x += 1;
        }
      data = data.offset (bytes_per_line as isize);
      y += 1;
    }

  return pimg;
}

/// C: C 308-310: `static cairo_surface_t *cr_create_surface_from_pix_containers (Emacs_Pix_Container pimg, Emacs_Pix_Container mask)`
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
unsafe fn cr_create_surface_from_pix_containers (pimg: Emacs_Pix_Container,
                                                mask: Emacs_Pix_Container)
  -> *mut cairo_surface_t
{
  let surface: *mut cairo_surface_t;

  if !mask.is_null ()
    {
      let (mut x, mut y): (c_int, c_int);

      y = 0;
      while y < (*pimg).height
        {
          x = 0;
          while x < (*pimg).width
            {
              let (mut color, mut alpha): (c_ulong, c_ulong);
              let (mut r, mut g, mut b): (c_int, c_int, c_int);

              color = GET_PIXEL! (pimg, x, y);
              alpha = GET_PIXEL! (mask, x, y);
              r = ((RED_FROM_ULONG! (color) * alpha + 0x7f) / 0xff) as c_int;
              g = ((GREEN_FROM_ULONG! (color) * alpha + 0x7f) / 0xff) as c_int;
              b = ((BLUE_FROM_ULONG! (color) * alpha + 0x7f) / 0xff) as c_int;
              PUT_PIXEL! (pimg, x, y, ARGB_TO_ULONG! (alpha, r as c_ulong, g as c_ulong,
                                                      b as c_ulong));
              x += 1;
            }
          y += 1;
        }
      xfree ((*mask).data as *mut c_void);
      (*mask).data = core::ptr::null_mut ();
    }
  surface = cairo_image_surface_create_for_data ((*pimg).data as *mut c_uchar,
                                                 (if !mask.is_null () { CAIRO_FORMAT_ARGB32 }
                                                  else { CAIRO_FORMAT_RGB24 }),
                                                 (*pimg).width, (*pimg).height,
                                                 (*pimg).bytes_per_line);
  /* C 339: `static const cairo_user_data_key_t key;` - a function-local `static` with static
     storage duration (`&key` is what C 340 takes); Rust keeps it a `static`.  */
  static key: cairo_user_data_key_t = cairo_user_data_key_t { unused: 0 };
  cairo_surface_set_user_data (surface, &raw const key, (*pimg).data as *mut c_void,
                              Some (xfree));
  (*pimg).data = core::ptr::null_mut ();

  return surface;
}

/// C: C 346-347: `static void cr_put_image_to_cr_data (struct image *img)`
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
unsafe fn cr_put_image_to_cr_data (img: *mut image)
{
  let mut pattern: *mut cairo_pattern_t = core::ptr::null_mut ();
  let surface: *mut cairo_surface_t
    = cr_create_surface_from_pix_containers ((*img).pixmap as Emacs_Pix_Container,
                                             (*img).mask as Emacs_Pix_Container);
  if !surface.is_null ()
    {
      pattern = cairo_pattern_create_for_surface (surface);
      if !(*img).cr_data.is_null ()
        {
          let mut matrix: cairo_matrix_t = core::mem::zeroed ();
          cairo_pattern_get_matrix ((*img).cr_data as *mut cairo_pattern_t, &raw mut matrix);
          cairo_pattern_set_matrix (pattern, &raw const matrix);
          cairo_pattern_set_filter
            (pattern, cairo_pattern_get_filter ((*img).cr_data as *mut cairo_pattern_t));
          cairo_pattern_destroy ((*img).cr_data as *mut cairo_pattern_t);
        }
      cairo_surface_destroy (surface);
    }

  (*img).cr_data = pattern as *mut c_void;
}

/* ----------------------------------------------------------------------------------------------
   C 372-387: `#ifdef HAVE_NS`'s two pixel shims.  C's `XGetPixel`/`XPutPixel` here are this
   file's own `static` functions (C 383's `XPutPixel` has the X name but the `unsigned long`
   pixel type); the definitions are inside this slice, so they carry C's place.
   ---------------------------------------------------------------------------------------------- */

/// C: C 374-375: `static unsigned long XGetPixel (Emacs_Pix_Container image, int x, int y)`
#[cfg (all (feature = "have_window_system", feature = "have_ns"))]
unsafe fn XGetPixel (image: Emacs_Pix_Container, x: c_int, y: c_int) -> c_ulong
{
  return ns_get_pixel (image as *mut c_void, x, y);
}

/// C: C 382-383: `static void XPutPixel (Emacs_Pix_Container image, int x, int y, unsigned long pixel)`
#[cfg (all (feature = "have_window_system", feature = "have_ns"))]
unsafe fn XPutPixel (image: Emacs_Pix_Container, x: c_int, y: c_int, pixel: c_ulong)
{
  ns_put_pixel (image as *mut c_void, x, y, pixel);
}

/* Code to deal with bitmaps.  Bitmaps are referenced by their bitmap
   id, which is just an int that this section returns.  Bitmaps are
   reference counted so they can be shared among frames.

   Bitmap indices are guaranteed to be > 0, so a negative number can
   be used to indicate no bitmap.

   If you use image_create_bitmap_from_data, then you must keep track
   of the bitmaps yourself.  That is, creating a bitmap from the same
   data more than once will not be caught.  */

/* Functions to access the contents of a bitmap, given an id.  */

/// C: C 403-404: `static int x_bitmap_height (struct frame *f, ptrdiff_t id)`
#[cfg (all (feature = "have_window_system", feature = "have_x_windows"))]
unsafe fn x_bitmap_height (f: *mut frame, id: ptrdiff_t) -> c_int
{
  return (*((*FRAME_DISPLAY_INFO! (f)).bitmaps.offset (id - 1))).height;
}

/// C: C 409-410: `static int x_bitmap_width (struct frame *f, ptrdiff_t id)`
#[cfg (all (feature = "have_window_system", feature = "have_x_windows"))]
unsafe fn x_bitmap_width (f: *mut frame, id: ptrdiff_t) -> c_int
{
  return (*((*FRAME_DISPLAY_INFO! (f)).bitmaps.offset (id - 1))).width;
}

/// C: C 416-417: `cairo_pattern_t *x_bitmap_stipple (struct frame *f, Pixmap pixmap)` - non-static
/// in C, so `#[unsafe(no_mangle)]` (PORTING.md 0.2).  `include/xterm.rs` 190-192 declares the same
/// name, which is the collision `docs/ports/image.md` owes the mirror (`w32fns.md`'s item 5 shape).
#[cfg (all (feature = "have_window_system", all (feature = "have_x_windows", feature = "use_cairo")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_bitmap_stipple (f: *mut frame, pixmap: Pixmap) -> *mut cairo_pattern_t
{
  let dpyinfo: *mut Display_Info = FRAME_DISPLAY_INFO! (f);

  let mut i: ptrdiff_t = 0;
  while i < (*dpyinfo).bitmaps_last
    {
      let bm: *mut x_bitmap_record = (*dpyinfo).bitmaps.offset (i);

      if (*bm).refcount != 0 && (*bm).pixmap == pixmap && (*bm).depth == 1
        {
          if (*bm).stipple.is_null ()
            {
              let surface: *mut cairo_surface_t
                = cairo_xlib_surface_create_for_bitmap (FRAME_X_DISPLAY! (f) as *mut c_void,
                                                        pixmap,
                                                        FRAME_X_SCREEN! (f) as *mut c_void,
                                                        (*bm).width, (*bm).height);
              let pattern: *mut cairo_pattern_t
                = cairo_pattern_create_for_surface (surface);
              cairo_surface_destroy (surface);
              cairo_pattern_set_extend (pattern, CAIRO_EXTEND_REPEAT);
              (*bm).stipple = pattern;
            }

          return (*bm).stipple;
        }
      i += 1;
    }

  return core::ptr::null_mut ();
}

/// C: C 452-453: `ptrdiff_t image_bitmap_pixmap (struct frame *f, ptrdiff_t id)` - non-static.
#[cfg (all (feature = "have_window_system", any (feature = "have_x_windows", feature = "have_ntgui", feature = "have_android")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn image_bitmap_pixmap (f: *mut frame, id: ptrdiff_t) -> ptrdiff_t
{
  /* HAVE_NTGUI needs the explicit cast here because .pixmap is a handle,
     i.e., a pointer to a struct (see w32gui.h), not an integer.  */
  return (*((*FRAME_DISPLAY_INFO! (f)).bitmaps.offset (id - 1))).pixmap as ptrdiff_t;
}

/// C: C 462-463: `int x_bitmap_mask (struct frame *f, ptrdiff_t id)` - non-static.
#[cfg (all (feature = "have_window_system", feature = "have_x_windows"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_bitmap_mask (f: *mut frame, id: ptrdiff_t) -> c_int
{
  return (*((*FRAME_DISPLAY_INFO! (f)).bitmaps.offset (id - 1))).mask as c_int;
}

/* Allocate a new bitmap record.  Returns index of new record.  */

/// C: C 471-472: `static ptrdiff_t image_allocate_bitmap_record (struct frame *f)`
#[cfg (feature = "have_window_system")]
unsafe fn image_allocate_bitmap_record (f: *mut frame) -> ptrdiff_t
{
  let dpyinfo: *mut Display_Info = FRAME_DISPLAY_INFO! (f);
  let mut i: ptrdiff_t;

  if (*dpyinfo).bitmaps_last < (*dpyinfo).bitmaps_size
    {
      (*dpyinfo).bitmaps_last += 1;
      return (*dpyinfo).bitmaps_last;
    }

  i = 0;
  while i < (*dpyinfo).bitmaps_size
    {
      if (*((*dpyinfo).bitmaps.offset (i))).refcount == 0
        { return i + 1; }
      i += 1;
    }

  (*dpyinfo).bitmaps
    = xpalloc ((*dpyinfo).bitmaps as *mut c_void, &raw mut (*dpyinfo).bitmaps_size,
               10, -1, core::mem::size_of::<Bitmap_Record> () as ptrdiff_t)
      as *mut Bitmap_Record;
  (*dpyinfo).bitmaps_last += 1;
  return (*dpyinfo).bitmaps_last;
}

/* Add one reference to the reference count of the bitmap with id ID.  */

/// C: C 492-493: `void image_reference_bitmap (struct frame *f, ptrdiff_t id)` - non-static.
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn image_reference_bitmap (f: *mut frame, id: ptrdiff_t)
{
  (*((*FRAME_DISPLAY_INFO! (f)).bitmaps.offset (id - 1))).refcount += 1;
}

/* ----------------------------------------------------------------------------------------------
   C 498-593: `#ifdef HAVE_PGTK`'s Cairo-pattern builder.
   ---------------------------------------------------------------------------------------------- */

/* Create a Cairo pattern from the bitmap BITS, which should be WIDTH
   and HEIGHT in size.  BITS's fill order is LSB first, meaning that
   the value of the left most pixel within a byte is its least
   significant bit.  */

/// C: C 505-506: `static cairo_pattern_t *image_bitmap_to_cr_pattern (char *bits, int width, int height)`
#[cfg (all (feature = "have_window_system", feature = "have_pgtk"))]
unsafe fn image_bitmap_to_cr_pattern (bits: *mut c_char, width: c_int, height: c_int)
  -> *mut cairo_pattern_t
{
  let surface: *mut cairo_surface_t;
  let mut data: *mut c_uchar;
  let stride: c_int;
  let pattern: *mut cairo_pattern_t;
  #[cfg (all (feature = "have_window_system", feature = "words_bigendian"))]
  let mut x: c_int;
  /// C: C 514-546: the 128-entry byte-reversal table the big-endian arm indexes.
  #[cfg (all (feature = "have_window_system", feature = "words_bigendian"))]
  static table: [c_uchar; 128] = [
    0x00, 0x80, 0x40, 0xc0, 0x20, 0xa0, 0x60, 0xe0,
    0x10, 0x90, 0x50, 0xd0, 0x30, 0xb0, 0x70, 0xf0,
    0x08, 0x88, 0x48, 0xc8, 0x28, 0xa8, 0x68, 0xe8,
    0x18, 0x98, 0x58, 0xd8, 0x38, 0xb8, 0x78, 0xf8,
    0x04, 0x84, 0x44, 0xc4, 0x24, 0xa4, 0x64, 0xe4,
    0x14, 0x94, 0x54, 0xd4, 0x34, 0xb4, 0x74, 0xf4,
    0x0c, 0x8c, 0x4c, 0xcc, 0x2c, 0xac, 0x6c, 0xec,
    0x1c, 0x9c, 0x5c, 0xdc, 0x3c, 0xbc, 0x7c, 0xfc,
    0x02, 0x82, 0x42, 0xc2, 0x22, 0xa2, 0x62, 0xe2,
    0x12, 0x92, 0x52, 0xd2, 0x32, 0xb2, 0x72, 0xf2,
    0x0a, 0x8a, 0x4a, 0xca, 0x2a, 0xaa, 0x6a, 0xea,
    0x1a, 0x9a, 0x5a, 0xda, 0x3a, 0xba, 0x7a, 0xfa,
    0x06, 0x86, 0x46, 0xc6, 0x26, 0xa6, 0x66, 0xe6,
    0x16, 0x96, 0x56, 0xd6, 0x36, 0xb6, 0x76, 0xf6,
    0x0e, 0x8e, 0x4e, 0xce, 0x2e, 0xae, 0x6e, 0xee,
    0x1e, 0x9e, 0x5e, 0xde, 0x3e, 0xbe, 0x7e, 0xfe,
    0x01, 0x81, 0x41, 0xc1, 0x21, 0xa1, 0x61, 0xe1,
    0x11, 0x91, 0x51, 0xd1, 0x31, 0xb1, 0x71, 0xf1,
    0x09, 0x89, 0x49, 0xc9, 0x29, 0xa9, 0x69, 0xe9,
    0x19, 0x99, 0x59, 0xd9, 0x39, 0xb9, 0x79, 0xf9,
    0x05, 0x85, 0x45, 0xc5, 0x25, 0xa5, 0x65, 0xe5,
    0x15, 0x95, 0x55, 0xd5, 0x35, 0xb5, 0x75, 0xf5,
    0x0d, 0x8d, 0x4d, 0xcd, 0x2d, 0xad, 0x6d, 0xed,
    0x1d, 0x9d, 0x5d, 0xdd, 0x3d, 0xbd, 0x7d, 0xfd,
    0x03, 0x83, 0x43, 0xc3, 0x23, 0xa3, 0x63, 0xe3,
    0x13, 0x93, 0x53, 0xd3, 0x33, 0xb3, 0x73, 0xf3,
    0x0b, 0x8b, 0x4b, 0xcb, 0x2b, 0xab, 0x6b, 0xeb,
    0x1b, 0x9b, 0x5b, 0xdb, 0x3b, 0xbb, 0x7b, 0xfb,
    0x07, 0x87, 0x47, 0xc7, 0x27, 0xa7, 0x67, 0xe7,
    0x17, 0x97, 0x57, 0xd7, 0x37, 0xb7, 0x77, 0xf7,
    0x0f, 0x8f, 0x4f, 0xcf, 0x2f, 0xaf, 0x6f, 0xef,
  ];
  let mut bits = bits;
  let mut height = height;

  surface = cairo_image_surface_create (CAIRO_FORMAT_A1, width, height);

  if cairo_surface_status (surface) != CAIRO_STATUS_SUCCESS
    { memory_full (0); }

  cairo_surface_flush (surface);
  data = cairo_image_surface_get_data (surface);
  stride = cairo_image_surface_get_stride (surface);

  /* Big endian systems require that individual bytes be inverted to
     compensate for the different fill order used by Cairo.  */
  #[cfg (all (feature = "have_window_system", feature = "words_bigendian"))]
  while { height -= 1; height >= 0 }
    {
      std::ptr::copy_nonoverlapping (bits as *const c_void, data as *mut c_void,
                                    ((width + 7) / 8) as usize);
      x = 0;
      while x < (width + 7) / 8
        {
          *data.offset (x as isize) = table[*data.offset (x as isize) as usize];
          x += 1;
        }
      data = data.offset (stride as isize);
      bits = bits.offset (((width + 7) / 8) as isize);
    }
  /* Cairo uses LSB first fill order for bitmaps on little-endian
     systems, so copy each row over.  */
  #[cfg (all (feature = "have_window_system", not (feature = "words_bigendian")))]
  while { height -= 1; height >= 0 }
    {
      std::ptr::copy_nonoverlapping (bits as *const c_void, data as *mut c_void,
                                    ((width + 7) / 8) as usize);
      data = data.offset (stride as isize);
      bits = bits.offset (((width + 7) / 8) as isize);
    }

  cairo_surface_mark_dirty (surface);
  pattern = cairo_pattern_create_for_surface (surface);
  if cairo_pattern_status (pattern) != CAIRO_STATUS_SUCCESS
    { memory_full (0); }

  /* The pattern now holds a reference to the surface.  */
  cairo_surface_destroy (surface);
  cairo_pattern_set_extend (pattern, CAIRO_EXTEND_REPEAT);
  return pattern;
}

/* Create a bitmap for frame F from a HEIGHT x WIDTH array of bits at BITS.  */

/// C: C 597-599: `ptrdiff_t image_create_bitmap_from_data (struct frame *f, char *bits, unsigned int width, unsigned int height)` - non-static.
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn image_create_bitmap_from_data (f: *mut frame, bits: *mut c_char,
                                                        width: c_uint, height: c_uint)
  -> ptrdiff_t
{
  let dpyinfo: *mut Display_Info = FRAME_DISPLAY_INFO! (f);
  let mut id: ptrdiff_t;

  #[cfg (all (feature = "have_window_system", feature = "have_x_windows"))]
  let bitmap: Pixmap = XCreateBitmapFromData (FRAME_X_DISPLAY! (f), (*dpyinfo).root_window,
                                             bits, width, height);
  #[cfg (all (feature = "have_window_system", feature = "have_x_windows"))]
  if bitmap == 0
    { return -1; }

  #[cfg (all (feature = "have_window_system", all (feature = "have_android", feature = "not_android_stubify")))]
  let bitmap: android_pixmap = android_create_bitmap_from_data (bits, width as c_uint,
                                                                 height as c_uint);
  #[cfg (all (feature = "have_window_system", all (feature = "have_android", feature = "not_android_stubify")))]
  if bitmap == 0
    { return -1; }
  #[cfg (all (feature = "have_window_system", all (feature = "have_android", not (feature = "not_android_stubify"))))]
  {
    let _ = dpyinfo;
    emacs_abort ();
  }

  #[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
  let mut stipple: Emacs_Pixmap;
  #[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
  let bitmap: Emacs_Pixmap = CreateBitmap (width as c_int, height as c_int,
                                           (*dpyinfo).n_planes, (*dpyinfo).n_cbits, bits);

  /* Convert X bitmap to W32 bitmap.  */
  /* Windows mono bitmaps are reversed compared with X.  */
  #[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
  {
    let mut bits = bits;

    USE_SAFE_ALLOCA! (sa_avail, sa_count);

    {
      let invertedBits: *mut c_char;
      let mut i: c_int;
      let nbytes: c_int = ((width + CHAR_BIT - 1) / CHAR_BIT * height) as c_int;

      invertedBits = bits;

      SAFE_NALLOCA! (sa_avail, bits, 1, nbytes);

      i = 0;
      while i < nbytes
        {
          *bits.offset (i as isize) = XBM_BIT_SHUFFLE! (*invertedBits.offset (i as isize));
          i += 1;
        }
    }

    stipple = w32_create_pixmap_from_bitmap_data (width as c_int, height as c_int, bits);

    SAFE_FREE! (sa_count);

    if bitmap.is_null () || stipple.is_null ()
      { return -1; }
  }

  #[cfg (all (feature = "have_window_system", feature = "have_ns"))]
  let bitmap: *mut c_void = ns_image_from_XBM (bits, width as c_int, height as c_int, 0, 0);
  #[cfg (all (feature = "have_window_system", feature = "have_ns"))]
  if bitmap.is_null ()
    { return -1; }

  #[cfg (all (feature = "have_window_system", feature = "have_pgtk"))]
  let pattern: *mut cairo_pattern_t = image_bitmap_to_cr_pattern (bits, width as c_int,
                                                                 height as c_int);

  #[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
  let bitmap: *mut c_void = BBitmap_new (width as c_int, height as c_int, false as c_int);
  #[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
  let mut stipple: *mut c_void;
  #[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
  let (mut bytes_per_line, mut x, mut y): (c_int, c_int, c_int);

  #[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
  if bitmap.is_null ()
    { return -1; }

  #[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
  {
    let mut bits = bits;

    bytes_per_line = ((width + 7) / 8) as c_int;
    stipple = xmalloc ((height * bytes_per_line as c_uint) as usize);
    std::ptr::copy_nonoverlapping (bits as *const c_void, stipple,
                                  (height * bytes_per_line as c_uint) as usize);

    y = 0;
    while y < height as c_int
      {
        x = 0;
        while x < width as c_int
          {
            /* C 682's `bits[8]` is C's own index (a fixed 8, not `x / 8`), kept as written.  */
            PUT_PIXEL! (bitmap, x, y, (if (*bits.offset (8) >> (x % 8)) & 1 != 0
                                         { (*f).foreground_pixel }
                                       else { (*f).background_pixel }));
            x += 1;
          }
        bits = bits.offset (bytes_per_line as isize);
        y += 1;
      }
  }

  id = image_allocate_bitmap_record (f);

  #[cfg (all (feature = "have_window_system", feature = "have_ns"))]
  {
    (*((*dpyinfo).bitmaps.offset (id - 1))).img = bitmap;
    (*((*dpyinfo).bitmaps.offset (id - 1))).depth = 1;
  }

  #[cfg (all (feature = "have_window_system", feature = "have_pgtk"))]
  {
    (*((*dpyinfo).bitmaps.offset (id - 1))).depth = 1;
    (*((*dpyinfo).bitmaps.offset (id - 1))).pattern = pattern;
  }

  #[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
  {
    let bm: *mut haiku_bitmap_record = (*dpyinfo).bitmaps.offset (id - 1);
    (*bm).img = bitmap;
    (*bm).depth = 1;
    (*bm).stipple_bits = stipple;
    (*bm).stipple_foreground = ((*f).foreground_pixel & 0xffffffff) as uint32_t;
    (*bm).stipple_background = ((*f).background_pixel & 0xffffffff) as uint32_t;
  }

  (*((*dpyinfo).bitmaps.offset (id - 1))).file = core::ptr::null_mut ();
  (*((*dpyinfo).bitmaps.offset (id - 1))).height = height as c_int;
  (*((*dpyinfo).bitmaps.offset (id - 1))).width = width as c_int;
  (*((*dpyinfo).bitmaps.offset (id - 1))).refcount = 1;

  #[cfg (all (feature = "have_window_system", any (feature = "have_x_windows", feature = "have_android")))]
  {
    #[cfg (all (feature = "have_window_system", feature = "not_android_stubify"))]
    { (*((*dpyinfo).bitmaps.offset (id - 1))).pixmap = bitmap; }
    (*((*dpyinfo).bitmaps.offset (id - 1))).have_mask = false;
    (*((*dpyinfo).bitmaps.offset (id - 1))).depth = 1;
    #[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
    { (*((*dpyinfo).bitmaps.offset (id - 1))).stipple = core::ptr::null_mut (); }
  }

  #[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
  {
    let bm: *mut w32_bitmap_record = (*dpyinfo).bitmaps.offset (id - 1);
    (*bm).pixmap = bitmap;
    (*bm).stipple = stipple;
    (*bm).hinst = core::ptr::null_mut ();
    (*bm).depth = 1;
  }

  return id;
}

/// C: C 737-742: `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY / #include "android.h" / typedef struct android_fd_or_asset image_fd; / #else typedef int image_fd; / #endif`
/// C 738's `"android.h"` is a mirror this unit's TERM_HEADER chain already brought in
/// (`include/androidterm.rs` 41 includes `include/androidgui.rs`, and `android.h`'s record is
/// `include/android.rs` 69) - the C line is named and the port keeps C's typedef pair.
#[cfg (all (feature = "have_window_system", feature = "have_android", feature = "not_android_stubify"))]
include!("include/android.rs");                // C 738: `#include "android.h"`
#[cfg (all (feature = "have_window_system", all (feature = "have_android", feature = "not_android_stubify")))]
pub type image_fd = android_fd_or_asset;
#[cfg (all (feature = "have_window_system", not (all (feature = "have_android", feature = "not_android_stubify"))))]
pub type image_fd = c_int;

/* C 749-752: the three `static` prototypes of functions this file defines outside this slice -
   `slurp_file` (C 4529), `image_find_image_fd` (C 4457) and `xbm_read_bitmap_data` (C 5080).  This
   slice calls all three from its platform arms, so the declarations are kept here (C's own
   signature, in a `/// C:` line) until their slices land - `docs/ports/image.md`'s owed list, and
   the reason the arms below resolve at all.  */
// C: C 749: `static char *slurp_file (image_fd, ptrdiff_t *);`
#[cfg (all (feature = "have_window_system", any (feature = "have_haiku", feature = "have_ns", feature = "have_pgtk",
            feature = "have_android", feature = "have_ntgui")))]
unsafe extern "C" {
  pub fn slurp_file (fd: image_fd, size: *mut ptrdiff_t) -> *mut c_char;
  // C: C 750: `static Lisp_Object image_find_image_fd (Lisp_Object, image_fd *);`
  pub fn image_find_image_fd (file: Lisp_Object, pfd: *mut image_fd) -> Lisp_Object;
  // C: C 751-752: `static bool xbm_read_bitmap_data (struct frame *, char *, char *, int *, int *, char **, bool);`
  pub fn xbm_read_bitmap_data (f: *mut frame, contents: *mut c_char, end: *mut c_char,
                               width: *mut c_int, height: *mut c_int, data: *mut *mut c_char,
                               inhibit_image_error: bool) -> bool;
}

/* Create bitmap from file FILE for frame F.  */

/// C: C 757-758: `ptrdiff_t image_create_bitmap_from_file (struct frame *f, Lisp_Object file)` - non-static.
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn image_create_bitmap_from_file (f: *mut frame, file: Lisp_Object)
  -> ptrdiff_t
{
  let dpyinfo: *mut Display_Info = FRAME_DISPLAY_INFO! (f);
  let mut id: ptrdiff_t = 0;
  let mut size: ptrdiff_t = 0;
  let mut width: c_int = 0;
  let mut height: c_int = 0;

  #[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
  {
    let mut rc: c_int = 0;
    let mut fd: image_fd = unsafe { core::mem::zeroed () };
    let mut contents: *mut c_char = core::ptr::null_mut ();
    let mut data: *mut c_char = core::ptr::null_mut ();
    let mut bitmap: Emacs_Pixmap = core::ptr::null_mut ();

    if !STRINGP (image_find_image_fd (file, &raw mut fd))
      { return -1; }

    contents = slurp_file (fd, &raw mut size);

    if contents.is_null ()
      { return -1; }

    rc = if xbm_read_bitmap_data (f, contents, contents.offset (size), &raw mut width,
                                  &raw mut height, &raw mut data, false) { 1 } else { 0 };

    if rc == 0
      {
        xfree (contents as *mut c_void);
        return -1;
      }

    {
      /* Windows mono bitmaps are reversed compared with X.  */

      let mut nbytes: c_int;
      let mut i: c_int;
      nbytes = ((width + CHAR_BIT - 1) / CHAR_BIT * height) as c_int;

      i = 0;
      while i < nbytes
        {
          *data.offset (i as isize) = XBM_BIT_SHUFFLE! (*data.offset (i as isize));
          i += 1;
        }
    }

    id = image_allocate_bitmap_record (f);
    bitmap = w32_create_pixmap_from_bitmap_data (width, height, data);

    (*((*dpyinfo).bitmaps.offset (id - 1))).height = width;
    (*((*dpyinfo).bitmaps.offset (id - 1))).width = height;
    (*((*dpyinfo).bitmaps.offset (id - 1))).stipple = bitmap;
    (*((*dpyinfo).bitmaps.offset (id - 1))).file = xlispstrdup (file);

    xfree (contents as *mut c_void);
    xfree (data as *mut c_void);
    return id;
  }

  #[cfg (all (feature = "have_window_system", feature = "have_ns"))]
  {
    let mut fd: c_int = 0;
    let mut rc: c_int = 0;
    let mut contents: *mut c_char = core::ptr::null_mut ();
    let mut data: *mut c_char = core::ptr::null_mut ();
    let mut bitmap: *mut c_void = core::ptr::null_mut ();

    if !STRINGP (image_find_image_fd (file, &raw mut fd))
      { return -1; }

    contents = slurp_file (fd, &raw mut size);

    if contents.is_null ()
      { return -1; }

    rc = if xbm_read_bitmap_data (f, contents, contents.offset (size), &raw mut width,
                                  &raw mut height, &raw mut data, false) { 1 } else { 0 };

    if rc == 0
      {
        xfree (contents as *mut c_void);
        return -1;
      }

    bitmap = ns_image_from_XBM (data, width, height, 0, 0);

    if bitmap.is_null ()
      {
        xfree (contents as *mut c_void);
        xfree (data as *mut c_void);
        return -1;
      }

    id = image_allocate_bitmap_record (f);
    (*((*dpyinfo).bitmaps.offset (id - 1))).img = bitmap;
    (*((*dpyinfo).bitmaps.offset (id - 1))).refcount = 1;
    (*((*dpyinfo).bitmaps.offset (id - 1))).file = xlispstrdup (file);
    (*((*dpyinfo).bitmaps.offset (id - 1))).depth = 1;
    (*((*dpyinfo).bitmaps.offset (id - 1))).height = ns_image_width (bitmap);
    (*((*dpyinfo).bitmaps.offset (id - 1))).width = ns_image_height (bitmap);

    xfree (contents as *mut c_void);
    xfree (data as *mut c_void);
    return id;
  }

  #[cfg (all (feature = "have_window_system", feature = "have_pgtk"))]
  {
    let mut fd: c_int = 0;
    let mut rc: c_int = 0;
    let mut contents: *mut c_char = core::ptr::null_mut ();
    let mut data: *mut c_char = core::ptr::null_mut ();

    if !STRINGP (image_find_image_fd (file, &raw mut fd))
      { return -1; }

    contents = slurp_file (fd, &raw mut size);

    if contents.is_null ()
      { return -1; }

    rc = if xbm_read_bitmap_data (f, contents, contents.offset (size), &raw mut width,
                                  &raw mut height, &raw mut data, false) { 1 } else { 0 };

    if rc == 0
      {
        xfree (contents as *mut c_void);
        return -1;
      }

    id = image_allocate_bitmap_record (f);

    (*((*dpyinfo).bitmaps.offset (id - 1))).refcount = 1;
    (*((*dpyinfo).bitmaps.offset (id - 1))).file = xlispstrdup (file);
    (*((*dpyinfo).bitmaps.offset (id - 1))).height = width;
    (*((*dpyinfo).bitmaps.offset (id - 1))).width = height;
    (*((*dpyinfo).bitmaps.offset (id - 1))).pattern
      = image_bitmap_to_cr_pattern (data, width, height);
    xfree (contents as *mut c_void);
    xfree (data as *mut c_void);
    return id;
  }

  #[cfg (all (feature = "have_window_system", feature = "have_x_windows"))]
  {
    let mut bitmap: Pixmap = 0;
    let mut xhot: c_int = 0;
    let mut yhot: c_int = 0;
    let mut result: c_int = 0;
    let mut found: Lisp_Object = Qnil ();
    let mut filename: *mut c_char = core::ptr::null_mut ();
    let mut width: c_uint = 0;
    let mut height: c_uint = 0;
    let mut id: ptrdiff_t = 0;

    /* Look for an existing bitmap with the same name.  */
    id = 0;
    while id < (*dpyinfo).bitmaps_last
      {
        let bm: *mut x_bitmap_record = (*dpyinfo).bitmaps.offset (id);
        if (*bm).refcount != 0
            && !(*bm).file.is_null ()
            && strcmp ((*bm).file, SSDATA (file)) == 0
          {
            (*bm).refcount += 1;
            return id + 1;
          }
        id += 1;
      }

    /* Search bitmap-file-path for the file, if appropriate.  */
    if openp (Vx_bitmap_file_path, file, Qnil (), &raw mut found,
              make_fixnum (R_OK as EMACS_INT), false, false, core::ptr::null_mut ()) < 0
      { return -1; }

    filename = SSDATA (found);

    result = XReadBitmapFile (FRAME_X_DISPLAY! (f),
                              (*dpyinfo).root_window,
                              filename, &raw mut width, &raw mut height, &raw mut bitmap,
                              &raw mut xhot, &raw mut yhot);
    if result != BitmapSuccess
      { return -1; }

    id = image_allocate_bitmap_record (f);
    (*((*dpyinfo).bitmaps.offset (id - 1))).pixmap = bitmap;
    (*((*dpyinfo).bitmaps.offset (id - 1))).have_mask = false;
    (*((*dpyinfo).bitmaps.offset (id - 1))).refcount = 1;
    (*((*dpyinfo).bitmaps.offset (id - 1))).file = xlispstrdup (file);
    (*((*dpyinfo).bitmaps.offset (id - 1))).depth = 1;
    (*((*dpyinfo).bitmaps.offset (id - 1))).height = height as c_int;
    (*((*dpyinfo).bitmaps.offset (id - 1))).width = width as c_int;
    #[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
    { (*((*dpyinfo).bitmaps.offset (id - 1))).stipple = core::ptr::null_mut (); }

    return id;
  }

  #[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
  {
    let mut fd: c_int = 0;
    let mut rc: c_int = 0;
    let mut bytes_per_line: c_int = 0;
    let mut x: c_int = 0;
    let mut y: c_int = 0;
    let mut contents: *mut c_char = core::ptr::null_mut ();
    let mut data: *mut c_char = core::ptr::null_mut ();
    let mut tmp: *mut c_char = core::ptr::null_mut ();
    let mut bitmap: *mut c_void = core::ptr::null_mut ();
    let mut found: Lisp_Object = Qnil ();

    /* Look for an existing bitmap with the same name.  */
    id = 0;
    while id < (*dpyinfo).bitmaps_last
      {
        let bm: *mut haiku_bitmap_record = (*dpyinfo).bitmaps.offset (id);
        if (*bm).refcount != 0
            && !(*bm).file.is_null ()
            && strcmp ((*bm).file, SSDATA (file)) == 0
          {
            (*bm).refcount += 1;
            return id + 1;
          }
        id += 1;
      }

    /* Search bitmap-file-path for the file, if appropriate.  */
    if openp (Vx_bitmap_file_path, file, Qnil (), &raw mut found,
              make_fixnum (R_OK as EMACS_INT), false, false, core::ptr::null_mut ()) < 0
      { return -1; }

    if !STRINGP (image_find_image_fd (file, &raw mut fd))
        && !STRINGP (image_find_image_fd (found, &raw mut fd))
      { return -1; }

    contents = slurp_file (fd, &raw mut size);

    if contents.is_null ()
      { return -1; }

    rc = if xbm_read_bitmap_data (f, contents, contents.offset (size), &raw mut width,
                                  &raw mut height, &raw mut data, false) { 1 } else { 0 };

    if rc == 0
      {
        xfree (contents as *mut c_void);
        return -1;
      }

    bitmap = BBitmap_new (width, height, false as c_int);

    if bitmap.is_null ()
      {
        xfree (contents as *mut c_void);
        xfree (data as *mut c_void);
        return -1;
      }

    id = image_allocate_bitmap_record (f);

    let bm: *mut haiku_bitmap_record = (*dpyinfo).bitmaps.offset (id - 1);
    (*bm).img = bitmap;
    (*bm).depth = 1;
    (*bm).file = xlispstrdup (file);
    (*bm).height = height;
    (*bm).width = width;
    (*bm).refcount = 1;
    (*bm).stipple_foreground = ((*f).foreground_pixel & 0xffffffff) as uint32_t;
    (*bm).stipple_background = ((*f).background_pixel & 0xffffffff) as uint32_t;
    (*bm).stipple_bits = data as *mut c_void;

    bytes_per_line = (width + 7) / 8;
    tmp = data;

    y = 0;
    while y < height
      {
        x = 0;
        while x < width
          {
            PUT_PIXEL! (bitmap, x, y, (if (*tmp.offset ((x / 8) as isize) >> (x % 8)) & 1 != 0
                                         { (*f).foreground_pixel }
                                       else { (*f).background_pixel }));
            x += 1;
          }

        tmp = tmp.offset (bytes_per_line as isize);
        y += 1;
      }

    xfree (contents as *mut c_void);
    return id;
  }

  #[cfg (all (feature = "have_window_system", all (feature = "have_android", not (feature = "not_android_stubify"))))]
  {
    let _ = dpyinfo;

    /* This function should never be called when building stubs.  */
    emacs_abort ();
  }

  #[cfg (all (feature = "have_window_system", all (feature = "have_android", feature = "not_android_stubify")))]
  {
    let mut rc: c_int = 0;
    let mut fd: image_fd = unsafe { core::mem::zeroed () };
    let mut contents: *mut c_char = core::ptr::null_mut ();
    let mut data: *mut c_char = core::ptr::null_mut ();
    let mut found: Lisp_Object = Qnil ();
    let mut bitmap: android_pixmap = 0;

    /* Look for an existing bitmap with the same name.  */
    id = 0;
    while id < (*dpyinfo).bitmaps_last
      {
        let bm: *mut android_bitmap_record = (*dpyinfo).bitmaps.offset (id);
        if (*bm).refcount != 0
            && !(*bm).file.is_null ()
            && strcmp ((*bm).file, SSDATA (file)) == 0
          {
            (*bm).refcount += 1;
            return id + 1;
          }
        id += 1;
      }

    /* Search bitmap-file-path for the file, if appropriate.  If no file
       extension or directory is specified and no file by this name
       exists, append the extension ".xbm" and retry.  */
    if (openp (Vx_bitmap_file_path, file, Qnil (), &raw mut found,
               make_fixnum (R_OK as EMACS_INT), false, false, core::ptr::null_mut ()) < 0)
        && (!NILP (Fequal (Ffile_name_nondirectory (file), file))
            || !strrchr (SSDATA (file), b'.' as c_int).is_null ()
            || (openp (Vx_bitmap_file_path,
                       CALLN! (Fconcat, file, build_string (c".xbm".as_ptr ())),
                       Qnil (), &raw mut found, make_fixnum (R_OK as EMACS_INT), false, false,
                       core::ptr::null_mut ()) < 0))
      { return -1; }

    if !STRINGP (image_find_image_fd (file, &raw mut fd))
        && !STRINGP (image_find_image_fd (found, &raw mut fd))
      { return -1; }

    contents = slurp_file (fd, &raw mut size);

    if contents.is_null ()
      { return -1; }

    rc = if xbm_read_bitmap_data (f, contents, contents.offset (size), &raw mut width,
                                  &raw mut height, &raw mut data, false) { 1 } else { 0 };

    if rc == 0
      {
        xfree (contents as *mut c_void);
        return -1;
      }

    xfree (contents as *mut c_void);
    bitmap = android_create_bitmap_from_data (data, width as c_uint, height as c_uint);
    xfree (data as *mut c_void);

    id = image_allocate_bitmap_record (f);
    (*((*dpyinfo).bitmaps.offset (id - 1))).pixmap = bitmap;
    (*((*dpyinfo).bitmaps.offset (id - 1))).have_mask = false;
    (*((*dpyinfo).bitmaps.offset (id - 1))).refcount = 1;
    (*((*dpyinfo).bitmaps.offset (id - 1))).file = xlispstrdup (file);
    (*((*dpyinfo).bitmaps.offset (id - 1))).depth = 1;
    (*((*dpyinfo).bitmaps.offset (id - 1))).height = height;
    (*((*dpyinfo).bitmaps.offset (id - 1))).width = width;

    return id;
  }

  /* C 755-1095 has no statement after its last arm: with none of the six platforms compiled the
     function falls off its end, which C reports as a warning and Rust as a type error.  A
     window-system configuration always selects one of the six (configure.ac 7521 fills
     `WINDOW_SYSTEM_OBJ` from the same test that picks the platform, and `dispextern.h` defines
     `Display_Info` only per platform), so this arm is unreachable in every configuration that
     exists - it is the port's spelling of the empty body, not a value C returns (`lread.rs` 719's
     documented `unreachable! ()` shape).  */
  #[cfg (all (feature = "have_window_system", not (any (feature = "have_ntgui", feature = "have_ns", feature = "have_pgtk",
                   feature = "have_x_windows", feature = "have_haiku", feature = "have_android"))))]
  unreachable! ()
}
