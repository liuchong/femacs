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

/* dispextern.h 115-196's per-platform `Display_Info`, `Emacs_Pix_Container` and `Emacs_Pix_Context`
   typedefs now come from `include/dispextern.rs` (3.408's mirror round); this unit keeps only the two
   arms that mirror deliberately does not carry - the pgtk `Display_Info` below and the w32 pixel pair
   in the next block - each for the reason written beside it.  */

/// C: dispextern.h 152: `typedef struct pgtk_display_info Display_Info;`
#[cfg (all (feature = "have_window_system", feature = "have_pgtk"))]
pub type Display_Info = pgtk_display_info;


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
macro_rules! RED16_FROM_ULONG { ($color:expr) => { (RED_FROM_ULONG! ($color) * 0x101) }; }
/// C: C 88: `#define GREEN16_FROM_ULONG(color)	(GREEN_FROM_ULONG (color) * 0x101)`
#[cfg (all (feature = "have_window_system", any (feature = "use_cairo", feature = "have_ns")))]
macro_rules! GREEN16_FROM_ULONG { ($color:expr) => { (GREEN_FROM_ULONG! ($color) * 0x101) }; }
/// C: C 89: `#define BLUE16_FROM_ULONG(color)		(BLUE_FROM_ULONG (color) * 0x101)`
#[cfg (all (feature = "have_window_system", any (feature = "use_cairo", feature = "have_ns")))]
macro_rules! BLUE16_FROM_ULONG { ($color:expr) => { (BLUE_FROM_ULONG! ($color) * 0x101) }; }

/* --- C 92-102: `#ifdef USE_CAIRO` ------------------------------------------------------------
   cairo is a third-party library header with no file in the reference tree and no mirror, which is
   exactly P-88's case, so the declarations this slice takes from it stand here rather than in a new
   file under `femacs/src/include/`, and every constant and record is a **clang reading** from this
   machine's `/opt/homebrew/include/cairo/cairo.h` (cairo 1.18.4), quoted in the ledger.  */
/// C: C 100-101: `static unsigned long image_alloc_image_color (struct frame *, struct image *, Lisp_Object, unsigned long);`
/// The first slice kept this declaration because C 2318's definition was outside it; the second
/// slice's C 2182-2211 **is** the definition, so the prototype is dropped here (PORTING.md 0.1-12:
/// a prototype and its definition in one translation unit have no Rust counterpart, and Rust
/// items are order-independent), and `docs/ports/image.md` records the move.
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
macro_rules! RED16_FROM_ULONG { ($color:expr) => { (RED_FROM_ULONG! ($color) * 0x101) }; }
/// C: C 191: `#define GREEN16_FROM_ULONG(color)	(GREEN_FROM_ULONG (color) * 0x101)`
#[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
macro_rules! GREEN16_FROM_ULONG { ($color:expr) => { (GREEN_FROM_ULONG! ($color) * 0x101) }; }
/// C: C 192: `#define BLUE16_FROM_ULONG(color)		(BLUE_FROM_ULONG (color) * 0x101)`
#[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
macro_rules! BLUE16_FROM_ULONG { ($color:expr) => { (BLUE_FROM_ULONG! ($color) * 0x101) }; }

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
macro_rules! RED16_FROM_ULONG { ($color:expr) => { (RED_FROM_ULONG! ($color) * 0x101) }; }
/// C: C 216: `#define GREEN16_FROM_ULONG(color)	(GREEN_FROM_ULONG (color) * 0x101)`
#[cfg (all (feature = "have_window_system", feature = "have_android"))]
macro_rules! GREEN16_FROM_ULONG { ($color:expr) => { (GREEN_FROM_ULONG! ($color) * 0x101) }; }
/// C: C 217: `#define BLUE16_FROM_ULONG(color)		(BLUE_FROM_ULONG (color) * 0x101)`
#[cfg (all (feature = "have_window_system", feature = "have_android"))]
macro_rules! BLUE16_FROM_ULONG { ($color:expr) => { (BLUE_FROM_ULONG! ($color) * 0x101) }; }
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

/* ----------------------------------------------------------------------------------------------
   C 1096-2211: the file's second slice.

   It holds `free_bitmap_record` (C 1099-1153), `image_destroy_bitmap` (C 1157-1173),
   `image_destroy_all_bitmaps` (C 1177-1188), the cross-platform forward declarations (C 1190-1231),
   the X11-only `x_create_bitmap_mask` (C 1234-1336), `struct image_type` (C 1338-1372),
   `add_image_type`/`valid_image_p` (C 1380-1427), the four error helpers (C 1429-1458),
   `enum image_value_type`/`struct image_keyword` (C 1461-1502), `parse_image_spec` (C 1505-1644),
   `image_spec_value` (C 1646-1671), three `DEFUN`s (`image-size` C 1673-1708, `image-mask-p`
   C 1710-1731, `image-metadata` C 1733-1757), `MAX_IMAGE_SIZE` and `make_image`/`free_image`
   (C 1758-1827), `check_image_size` (C 1829-1857), `prepare_image_for_display` (C 1859-1893),
   `image_ascent` (C 1911-1949), `four_corners_best` (C 1957-2004), `make_color_name`
   (C 2006-2010), `image_background`/`image_background_transparent` (C 2017-2100), the
   `CLEAR_IMAGE_*` masks (C 2098-2100), `image_clear_image_1`/`image_clear_image` (C 2102-2174) and
   `image_alloc_image_color` (C 2182-2211).  **The next live definition is `make_image_cache`
   (C 2224)**, after C 2219's `static void cache_image (struct frame *, struct image *);` - the
   next slice's first item.

   The per-`#if` verdicts, the C line-range rows and this round's readings are in
   `docs/ports/image.md` (`## C line ranges`, `## Evidence`, `## Deviations`).  Names the mirrors
   this unit includes do not carry are declared at the use site with C's own prototype in a
   `/// C:` line (PORTING.md P-55/P-72 rule 3); names the platform SDKs supply (`<X11/Xlib.h>`,
   `<windows.h>`, Xrender, cairo) keep C's tokens.
   ---------------------------------------------------------------------------------------------- */

/* Free bitmap B.  */

/// C: C 1099-1100: `static void free_bitmap_record (Display_Info *dpyinfo, Bitmap_Record *bm)`
#[cfg (feature = "have_window_system")]
unsafe fn free_bitmap_record (dpyinfo: *mut Display_Info, bm: *mut Bitmap_Record)
{
  /* C 1102-1118: `#ifdef HAVE_X_WINDOWS`.  */
  #[cfg (all (feature = "have_window_system", feature = "have_x_windows"))]
  {
    /* Free the pixmap and mask.  Only do this if DPYINFO->display is
       still set, which may not be the case if the connection has
       already been closed in response to an IO error.  */

    if !(*dpyinfo).display.is_null ()
      {
	XFreePixmap ((*dpyinfo).display, (*bm).pixmap);
	if (*bm).have_mask
	  { XFreePixmap ((*dpyinfo).display, (*bm).mask); }
      }

    /* C 1114-1117: the nested `#ifdef USE_CAIRO`.  */
    #[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
    if !(*bm).stipple.is_null ()
      { cairo_pattern_destroy ((*bm).stipple); }
  }

  /* C 1120-1125: `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY`.  C's second call passes
     `pixmap` rather than `mask`; kept as written.  */
  #[cfg (all (feature = "have_window_system", feature = "have_android", feature = "not_android_stubify"))]
  {
    android_free_pixmap ((*bm).pixmap);

    if (*bm).have_mask
      { android_free_pixmap ((*bm).pixmap); }
  }

  /* C 1127-1130: `#ifdef HAVE_NTGUI`.  */
  #[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
  {
    DeleteObject ((*bm).pixmap);
    DeleteObject ((*bm).stipple);
  }

  /* C 1132-1134: `#ifdef HAVE_NS`.  */
  #[cfg (all (feature = "have_window_system", feature = "have_ns"))]
  ns_release_object ((*bm).img);

  /* C 1136-1139: `#ifdef HAVE_PGTK`.  */
  #[cfg (all (feature = "have_window_system", feature = "have_pgtk"))]
  if !(*bm).pattern.is_null ()
    { cairo_pattern_destroy ((*bm).pattern); }

  /* C 1141-1146: `#ifdef HAVE_HAIKU`.  */
  #[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
  {
    BBitmap_free ((*bm).img);

    if !(*bm).stipple_bits.is_null ()
      { xfree ((*bm).stipple_bits); }
  }

  if !(*bm).file.is_null ()
    {
      xfree ((*bm).file as *mut c_void);
      (*bm).file = core::ptr::null_mut ();
    }
}

/* Remove reference to bitmap with id number ID.  */

/// C: C 1157-1158: `void image_destroy_bitmap (struct frame *f, ptrdiff_t id)` - non-static.
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn image_destroy_bitmap (f: *mut frame, id: ptrdiff_t)
{
  let dpyinfo: *mut Display_Info = FRAME_DISPLAY_INFO! (f);

  if id > 0
    {
      let bm: *mut Bitmap_Record = (*dpyinfo).bitmaps.offset (id - 1);

      /* C's `if (--bm->refcount == 0)`.  */
      if { (*bm).refcount -= 1; (*bm).refcount == 0 }
	{
	  block_input ();
	  free_bitmap_record (dpyinfo, bm);
	  unblock_input ();
	}
    }
}

/* Free all the bitmaps for the display specified by DPYINFO.  */

/// C: C 1177-1178: `void image_destroy_all_bitmaps (Display_Info *dpyinfo)` - non-static.
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn image_destroy_all_bitmaps (dpyinfo: *mut Display_Info)
{
  let mut i: ptrdiff_t;
  let mut bm: *mut Bitmap_Record = (*dpyinfo).bitmaps;

  /* C's `for (i = 0; i < dpyinfo->bitmaps_last; i++, bm++)`.  */
  i = 0;
  while i < (*dpyinfo).bitmaps_last
    {
      if (*bm).refcount > 0
	{ free_bitmap_record (dpyinfo, bm); }
      i += 1;
      bm = bm.offset (1);
    }

  (*dpyinfo).bitmaps_last = 0;
}

/* C 1190-1193: `#ifndef HAVE_XRENDER`'s `typedef void Picture;` - the stand-in the
   `image_create_x_image_and_pixmap_1` prototype below needs - and the `HAVE_XRENDER` arm's
   Xrender.h `Picture`.  `include/xlib.rs` mirrors no `Picture`/Xrender surface, so the
   declaration stands here (the ledger's mirror-gap list).  */
/// C: C 1191-1192: `typedef void Picture;` (inside C 1190's `#ifndef HAVE_XRENDER`)
#[cfg (all (feature = "have_window_system", not (feature = "have_xrender")))]
pub type Picture = c_void;
/// C: Xrender.h 176: `typedef XID Picture;` - i.e. an `unsigned long` (clang-measured: the
/// header's `XID` is `unsigned long`).
#[cfg (all (feature = "have_window_system", feature = "have_xrender"))]
pub type Picture = c_ulong;

/* C 1195-1198: two `static` prototypes whose definitions are outside this slice; the file's
   platforms call `image_destroy_x_image` from `image_clear_image_1` and keep C's declaration for
   the pair (the C 749-752 shape of the first slice; their slices drop these copies).  */
// C: C 1195-1197: `static bool image_create_x_image_and_pixmap_1 (struct frame *, int, int, int, Emacs_Pix_Container *, Emacs_Pixmap *, Picture *);`
// C: C 1198: `static void image_destroy_x_image (Emacs_Pix_Container);`
#[cfg (feature = "have_window_system")]
unsafe extern "C" {
  pub fn image_create_x_image_and_pixmap_1 (f: *mut frame, width: c_int, height: c_int,
					    depth: c_int, ximg: *mut Emacs_Pix_Container,
					    pixmap: *mut Emacs_Pixmap, picture: *mut Picture) -> bool;
  pub fn image_destroy_x_image (ximg: Emacs_Pix_Container);
}

/* C 1200-1211: the X-image accessor pair.  Under `HAVE_NTGUI` they are functions (C 1201-1204,
   defined outside this slice); everywhere else C 1206-1210 declares `image_get_x_image`/
   `image_unget_x_image` and defines the two `_or_dc` wrappers as macros, which the port carries as
   `macro_rules!` (the `GET_PIXEL`/`PUT_PIXEL` rule of this file's own C 67-222 section).  */
// C: C 1201-1204: `static HDC image_get_x_image_or_dc (struct frame *, struct image *, bool, HGDIOBJ *);` / `static void image_unget_x_image_or_dc (struct image *, bool, HDC, HGDIOBJ);`
#[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
unsafe extern "C" {
  pub fn image_get_x_image_or_dc (f: *mut frame, img: *mut image, mask_p: bool,
				  dummy: *mut HGDIOBJ) -> HDC;
  pub fn image_unget_x_image_or_dc (img: *mut image, mask_p: bool, ximg: HDC, dummy: HGDIOBJ);
}
// C: C 1206-1207: `static Emacs_Pix_Container image_get_x_image (struct frame *, struct image *, bool);`
// C: C 1208: `static void image_unget_x_image (struct image *, bool, Emacs_Pix_Container);`
#[cfg (all (feature = "have_window_system", not (feature = "have_ntgui")))]
unsafe extern "C" {
  pub fn image_get_x_image (f: *mut frame, img: *mut image, mask_p: bool) -> Emacs_Pix_Container;
  pub fn image_unget_x_image (img: *mut image, mask_p: bool, ximg: Emacs_Pix_Container);
}
/// C: C 1209-1210: `#define image_get_x_image_or_dc(f, img, mask_p, dummy) image_get_x_image (f, img, mask_p)` / `#define image_unget_x_image_or_dc(img, mask_p, ximg, dummy) image_unget_x_image (img, mask_p, ximg)`
#[cfg (all (feature = "have_window_system", not (feature = "have_ntgui")))]
macro_rules! image_get_x_image_or_dc {
  ($f:expr, $img:expr, $mask_p:expr, $dummy:expr) => { image_get_x_image ($f, $img, $mask_p) };
}
#[cfg (all (feature = "have_window_system", not (feature = "have_ntgui")))]
macro_rules! image_unget_x_image_or_dc {
  ($img:expr, $mask_p:expr, $ximg:expr, $dummy:expr) =>
    { image_unget_x_image ($img, $mask_p, $ximg) };
}
/* C 1201-1204 spells the same two names as *functions* under `HAVE_NTGUI`, and the call sites
   (`image_background`/`image_background_transparent`) keep C's text in both arms; the port's
   uniform spelling is therefore the macro form, which forwards to those functions here.  */
#[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
macro_rules! image_get_x_image_or_dc {
  ($f:expr, $img:expr, $mask_p:expr, $dummy:expr) =>
    { image_get_x_image_or_dc ($f, $img, $mask_p, $dummy) };
}
#[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
macro_rules! image_unget_x_image_or_dc {
  ($img:expr, $mask_p:expr, $ximg:expr, $dummy:expr) =>
    { image_unget_x_image_or_dc ($img, $mask_p, $ximg, $dummy) };
}

/* C 1213-1231: `#if defined HAVE_X_WINDOWS || defined HAVE_ANDROID`'s two more `static`
   prototypes, C's own comment, and the nested `#ifdef HAVE_X_WINDOWS` pair.  */
// C: C 1218: `static void image_sync_to_pixmaps (struct frame *, struct image *);` (inside C 1215's `#ifndef USE_CAIRO`)
#[cfg (all (feature = "have_window_system", any (feature = "have_x_windows", feature = "have_android"),
            not (feature = "use_cairo")))]
unsafe extern "C" {
  pub fn image_sync_to_pixmaps (f: *mut frame, img: *mut image);
}
/* We are working on X-specific data structures here even with cairo.
   So we use X-specific versions of image construction/destruction
   functions and inline the specific case of four_corners_best.  */

// C: C 1225-1226: `static bool x_create_x_image_and_pixmap (struct frame *, int, int, int, XImage **, Pixmap *);` / `static void x_destroy_x_image (XImage *);`
#[cfg (all (feature = "have_window_system", any (feature = "have_x_windows", feature = "have_android")))]
unsafe extern "C" {
  pub fn x_create_x_image_and_pixmap (f: *mut frame, width: c_int, height: c_int, depth: c_int,
				      ximg: *mut *mut XImage, pixmap: *mut Pixmap) -> bool;
  pub fn x_destroy_x_image (ximg: *mut XImage);
}

/* Create a mask of a bitmap. Note is this not a perfect mask.
   It's nicer with some borders in this context */

/// C: C 1234-1235: `void x_create_bitmap_mask (struct frame *f, ptrdiff_t id)` - non-static.
#[cfg (all (feature = "have_window_system", feature = "have_x_windows"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_create_bitmap_mask (f: *mut frame, id: ptrdiff_t)
{
  let (mut pixmap, mut mask): (Pixmap, Pixmap);
  let (mut ximg, mut mask_img): (*mut XImage, *mut XImage);
  let (mut width, mut height): (c_ulong, c_ulong);
  let mut result: bool;
  /* C's `unsigned long bg UNINIT;` - zeroed (PORTING.md 1.5/0.1-13).  */
  let mut bg: c_ulong = 0;
  let (mut x, mut y, mut xp, mut xm, mut yp, mut ym): (c_ulong, c_ulong, c_ulong, c_ulong, c_ulong, c_ulong);
  let mut gc: GC;

  let dpyinfo: *mut Display_Info = FRAME_DISPLAY_INFO! (f);

  if !(id > 0)
    { return; }

  pixmap = image_bitmap_pixmap (f, id) as Pixmap;
  width = x_bitmap_width (f, id) as c_ulong;
  height = x_bitmap_height (f, id) as c_ulong;

  block_input ();
  /* C's `~0` is the `int` -1, which the `unsigned long` parameter sign-extends; kept as the same
     value.  */
  ximg = XGetImage (FRAME_X_DISPLAY! (f), pixmap, 0, 0, width, height, !0i32 as c_ulong, ZPixmap);

  if ximg.is_null ()
    {
      unblock_input ();
      return;
    }

  result = x_create_x_image_and_pixmap (f, width as c_int, height as c_int, 1, &raw mut mask_img,
					&raw mut mask);

  unblock_input ();
  if !result
    {
      XDestroyImage (ximg);
      return;
    }

  let mut corner_pixels: [c_ulong; 4] = [0; 4];
  corner_pixels[0] = XGetPixel (ximg, 0, 0);
  corner_pixels[1] = XGetPixel (ximg, width - 1, 0);
  corner_pixels[2] = XGetPixel (ximg, width - 1, height - 1);
  corner_pixels[3] = XGetPixel (ximg, 0, height - 1);
  let (mut i, mut best_count): (c_int, c_int);
  i = 0; best_count = 0;
  while i < 4
    {
      let (mut j, mut n): (c_int, c_int);

      j = 0; n = 0;
      while j < 4
	{
	  if corner_pixels[i as usize] == corner_pixels[j as usize]
	    { n += 1; }
	  j += 1;
	}

      if n > best_count
	{ bg = corner_pixels[i as usize]; best_count = n; }
      i += 1;
    }

  y = 0;
  while y < (*ximg).height as c_ulong
    {
      x = 0;
      while x < (*ximg).width as c_ulong
	{
	  xp = if x != (*ximg).width as c_ulong - 1 { x + 1 } else { 0 };
	  xm = if x != 0 { x - 1 } else { (*ximg).width as c_ulong - 1 };
	  yp = if y != (*ximg).height as c_ulong - 1 { y + 1 } else { 0 };
	  ym = if y != 0 { y - 1 } else { (*ximg).height as c_ulong - 1 };
	  if XGetPixel (ximg, x, y) == bg
	      && XGetPixel (ximg, x, yp) == bg
	      && XGetPixel (ximg, x, ym) == bg
	      && XGetPixel (ximg, xp, y) == bg
	      && XGetPixel (ximg, xp, yp) == bg
	      && XGetPixel (ximg, xp, ym) == bg
	      && XGetPixel (ximg, xm, y) == bg
	      && XGetPixel (ximg, xm, yp) == bg
	      && XGetPixel (ximg, xm, ym) == bg
	    { XPutPixel (mask_img, x, y, 0); }
	  else
	    { XPutPixel (mask_img, x, y, 1); }
	  x += 1;
	}
      y += 1;
    }

  eassert! (input_blocked_p ());
  gc = XCreateGC (FRAME_X_DISPLAY! (f), mask, 0, core::ptr::null_mut ());
  XPutImage (FRAME_X_DISPLAY! (f), mask, gc, mask_img, 0, 0, 0, 0,
	     width as c_int, height as c_int);
  XFreeGC (FRAME_X_DISPLAY! (f), gc);

  (*(*dpyinfo).bitmaps.offset (id - 1)).have_mask = true;
  (*(*dpyinfo).bitmaps.offset (id - 1)).mask = mask;

  XDestroyImage (ximg);
  x_destroy_x_image (mask_img);
}

/***********************************************************************
			    Image types
 ***********************************************************************/

/* Each image format (JPEG, TIFF, ...) supported is described by
   a structure of the type below.  */

/// C: C 1338-1343: `struct image_type` (the definition; dispextern.h 3345 only forward-declares it)
#[cfg (feature = "have_window_system")]
#[repr(C)]
pub struct image_type
{
  /* Index of a symbol uniquely identifying the image type, e.g., 'jpeg'.  */
  pub r#type: c_int,

  /* Check that SPEC is a valid image specification for the given
     image type.  Value is true if SPEC is valid.  */
  pub valid_p: Option<unsafe extern "C" fn (spec: Lisp_Object) -> bool>,

  /* Load IMG which is to be used on frame F from information contained
     in IMG->spec.  Value is true if successful.  */
  pub load_img: Option<unsafe extern "C" fn (f: *mut frame, img: *mut image) -> bool>,

  /* Free such resources of image IMG as are used on frame F.  */
  pub free_img: Option<unsafe extern "C" fn (f: *mut frame, img: *mut image)>,

  /* C 1345-1347: the nested `#ifdef WINDOWSNT` member.  */
  #[cfg (feature = "windowsnt")]
  pub init: Option<unsafe extern "C" fn () -> bool>,
}

/* C 1352-1361: `#if defined HAVE_RSVG || … || defined HAVE_ANDROID`'s `IMAGE_TYPE_INIT` pair - the
   identity under `WINDOWSNT` and empty everywhere else.  The condition's eleven macros are spelled
   as features; the four the manifest does not carry yet are named in the ledger's owed list.  */
#[cfg (all (feature = "have_window_system", any (feature = "have_rsvg", feature = "have_png",
              feature = "have_gif", feature = "have_tiff", feature = "have_jpeg", feature = "have_xpm",
              feature = "have_ns", feature = "have_haiku", feature = "have_pgtk", feature = "have_webp",
              feature = "have_android"), feature = "windowsnt"))]
macro_rules! IMAGE_TYPE_INIT { ($f:expr) => { $f }; }
#[cfg (all (feature = "have_window_system", any (feature = "have_rsvg", feature = "have_png",
              feature = "have_gif", feature = "have_tiff", feature = "have_jpeg", feature = "have_xpm",
              feature = "have_ns", feature = "have_haiku", feature = "have_pgtk", feature = "have_webp",
              feature = "have_android"), not (feature = "windowsnt")))]
macro_rules! IMAGE_TYPE_INIT { ($f:expr) => { }; }

/* Forward function prototypes.  */

// C: C 1374-1378: `static struct image_type const *lookup_image_type (Lisp_Object);` / `static void image_laplace (struct frame *, struct image *);` / `static void image_emboss (struct frame *, struct image *);` / `static void image_build_heuristic_mask (struct frame *, struct image *, Lisp_Object);`
#[cfg (feature = "have_window_system")]
unsafe extern "C" {
  pub fn lookup_image_type (spec: Lisp_Object) -> *const image_type;
  pub fn image_laplace (f: *mut frame, img: *mut image);
  pub fn image_emboss (f: *mut frame, img: *mut image);
  pub fn image_build_heuristic_mask (f: *mut frame, img: *mut image, how: Lisp_Object);
}

/// C: C 1380-1381: `static void add_image_type (Lisp_Object type)`
#[cfg (feature = "have_window_system")]
unsafe fn add_image_type (r#type: Lisp_Object)
{
  Vimage_types = Fcons (r#type, Vimage_types);
}


/* Value is true if OBJECT is a valid Lisp image specification.  A
   valid image specification is a list whose car is the symbol
   `image', and whose rest is a property list.  The property list must
   contain a value for key `:type'.  That value must be the name of a
   supported image type.  The rest of the property list depends on the
   image type.  */

/// C: C 1394-1395: `bool valid_image_p (Lisp_Object object)` - non-static.
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn valid_image_p (object: Lisp_Object) -> bool
{
  if IMAGEP (object)
    {
      let mut tail: Lisp_Object = XCDR (object);
      FOR_EACH_TAIL_SAFE! (tail, {
	if EQ (XCAR (tail), QCtype ())
	  {
	    tail = XCDR (tail);
	    if CONSP (tail)
	      {
		let r#type: *const image_type = lookup_image_type (XCAR (tail));
		if !r#type.is_null ()
		  { return (*r#type).valid_p.unwrap () (object); }
	      }
	    break;
	  }
	tail = XCDR (tail);
	if ! CONSP (tail)
	  { return false; }
      });
    }

  return false;
}

/* ----------------------------------------------------------------------------------------------
   The names this slice calls that the mirrors it includes do not carry, each with C's own
   prototype in a `/// C:` line (PORTING.md P-55/P-72 rule 3).  They belong in the mirror that
   owns them and move there in the commit that adds them; `docs/ports/image.md`'s `## Owed by the
   integrator` names the file for each.
   ---------------------------------------------------------------------------------------------- */

// C: dispextern.h 3757: `ptrdiff_t lookup_image (struct frame *, Lisp_Object, int);` - the
// definition is this file's own (C 12440's region), outside this slice.
#[cfg (feature = "have_window_system")]
unsafe extern "C" {
  pub fn lookup_image (f: *mut frame, spec: Lisp_Object, face_id: c_int) -> ptrdiff_t;
}
// C: frame.h 1474-1475: `extern WINDOW_SYSTEM_RETURN struct frame * decode_window_system_frame (Lisp_Object);`
// (`WINDOW_SYSTEM_RETURN` is `_Noreturn ATTRIBUTE_COLD`, which is not part of the type - the
// `DEFUN!` `noreturn` note in PORTING.md 5.1).
#[cfg (feature = "have_window_system")]
unsafe extern "C" {
  pub fn decode_window_system_frame (frame: Lisp_Object) -> *mut frame;
}
/// C: frame.h 1667-1672: `INLINE struct image * IMAGE_FROM_ID (struct frame *f, int id) { eassert (0 <= id && id < FRAME_IMAGE_CACHE (f)->used); return FRAME_IMAGE_CACHE (f)->images[id]; }`
#[cfg (feature = "have_window_system")]
macro_rules! IMAGE_FROM_ID {
  ($f:expr, $id:expr) =>
    {
      ({
	eassert! (0 <= $id && ($id as isize) < (*(FRAME_IMAGE_CACHE! ($f) as *mut image_cache)).used);
	(*(*(FRAME_IMAGE_CACHE! ($f) as *mut image_cache)).images.offset ($id as isize))
      })
    };
}
// C: lisp.h 4379: `extern void vadd_to_log (char const *, va_list);` - the definition is xdisp.c's.
#[cfg (feature = "have_window_system")]
unsafe extern "C" {
  pub fn vadd_to_log (format: *const c_char, ap: core::ffi::VaList<'_>);
}
// C: lisp.h 3290: `INLINE bool FUNCTIONP (Lisp_Object a) { return SUBRP (a) || COMPILEDP (a) || CONSP (a); }`
// (the `lisp.h` name `parse_image_spec` calls; `include/lisp.rs` does not carry it - the
// `memory_full`/`openp` shape of the first slice, and the ledger's mirror-gap list).
#[cfg (feature = "have_window_system")]
unsafe extern "C" {
  pub fn FUNCTIONP (a: Lisp_Object) -> bool;
}
// C: lisp.h 4542: `extern Lisp_Object make_formatted_string (const char *, ...);`
#[cfg (feature = "have_window_system")]
unsafe extern "C" {
  pub fn make_formatted_string (format: *const c_char, ...) -> Lisp_Object;
}
// C: dispextern.h 3791: `void x_free_colors (struct frame *, unsigned long *, int);` - the
// definition is xfaces.c's.
#[cfg (all (feature = "have_window_system", feature = "have_x_windows", not (feature = "use_cairo")))]
unsafe extern "C" {
  pub fn x_free_colors (f: *mut frame, pixels: *mut c_ulong, npixels: c_int);
}
// C: C 126-127: `static void canvas_prepare_for_display (struct frame *, struct image *);` - the
// first slice dropped C's prototype pair because its definition was outside that slice; this
// slice's `prepare_image_for_display` is the call, so the declaration stands here until C 2438's
// definition lands (the same commit deletes it).
#[cfg (feature = "have_window_system")]
unsafe extern "C" {
  pub fn canvas_prepare_for_display (f: *mut frame, img: *mut image);
}
// C: dispextern.h 3760-3766: `# define RGB_PIXEL_COLOR unsigned long` (C 3762) and the
// `HAVE_NTGUI` arm `# define RGB_PIXEL_COLOR COLORREF` (C 3766).  Mirror gap; the `haikuterm.rs`
// 202-207 shape.  `COLORREF` is `<windows.h>`'s and no mirror declares it (the ledger's
// text-only Windows arm).
#[cfg (all (feature = "have_window_system", not (feature = "have_ntgui")))]
pub type RGB_PIXEL_COLOR = c_ulong;
#[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
pub type RGB_PIXEL_COLOR = COLORREF;
/* C 3318-3321: the four corner indices (`struct image`'s `corners` member) and C 3327-3328's two
   ascent constants - dispextern.h macros the mirror does not carry.  The indices are `usize` so
   they index `(*img).corners` directly; `CENTERED_IMAGE_ASCENT` is C's `-1`.  */
/// C: dispextern.h 3318-3321: `#define TOP_CORNER 0` / `#define LEFT_CORNER 1` / `#define BOT_CORNER 2` / `#define RIGHT_CORNER 3`
#[cfg (feature = "have_window_system")]
pub const TOP_CORNER: usize = 0;
#[cfg (feature = "have_window_system")]
pub const LEFT_CORNER: usize = 1;
#[cfg (feature = "have_window_system")]
pub const BOT_CORNER: usize = 2;
#[cfg (feature = "have_window_system")]
pub const RIGHT_CORNER: usize = 3;
/// C: dispextern.h 3327: `#define DEFAULT_IMAGE_ASCENT 50`
#[cfg (feature = "have_window_system")]
pub const DEFAULT_IMAGE_ASCENT: c_int = 50;
/// C: dispextern.h 3328: `#define CENTERED_IMAGE_ASCENT -1`
#[cfg (feature = "have_window_system")]
pub const CENTERED_IMAGE_ASCENT: c_int = -1;
/// C: dispextern.h 3860-3864: `#define IMAGE_BACKGROUND(img, f, ximg) ((img)->background_valid ? (img)->background : image_background (img, f, ximg))`
/// C: dispextern.h 3869-3873: `#define IMAGE_BACKGROUND_TRANSPARENT(img, f, mask) ((img)->background_transparent_valid ? (img)->background_transparent : image_background_transparent (img, f, mask))`
/// The record's bit members are one storage unit (`bits`, P-80), so the predicate is the mirror's
/// `IMAGE_*_MASK`/`_SHIFT` pair - the `haikuterm.rs` 215-233 shape.
#[cfg (feature = "have_window_system")]
macro_rules! IMAGE_BACKGROUND {
  ($img:expr, $f:expr, $ximg:expr) =>
    {
      (if (*$img).bits & IMAGE_BACKGROUND_VALID_MASK != 0
	 { (*$img).background }
       else
	 { image_background ($img, $f, $ximg) })
    };
}
#[cfg (feature = "have_window_system")]
macro_rules! IMAGE_BACKGROUND_TRANSPARENT {
  ($img:expr, $f:expr, $mask:expr) =>
    {
      (if (*$img).bits & IMAGE_BACKGROUND_TRANSPARENT_VALID_MASK != 0
	 { ((*$img).bits >> IMAGE_BACKGROUND_TRANSPARENT_SHIFT & 1) as c_int }
       else
	 { image_background_transparent ($img, $f, $mask) })
    };
}
/// C: C 2098-2100: `#define CLEAR_IMAGE_PIXMAP (1 << 0)` / `#define CLEAR_IMAGE_MASK (1 << 1)` / `#define CLEAR_IMAGE_COLORS (1 << 2)`
#[cfg (feature = "have_window_system")]
macro_rules! CLEAR_IMAGE_PIXMAP { () => { (1 << 0) }; }
#[cfg (feature = "have_window_system")]
macro_rules! CLEAR_IMAGE_MASK { () => { (1 << 1) }; }
#[cfg (feature = "have_window_system")]
macro_rules! CLEAR_IMAGE_COLORS { () => { (1 << 2) }; }
/* P-88: two cairo.h names `prepare_image_for_display` (C 1879) takes that the unit's cairo
   section (C 92-102) does not declare yet; they join it (the section is the `#include` site's
   declarations, not a new `include/` file).  Clang readings: `CAIRO_PATTERN_TYPE_SURFACE` **1**
   and `sizeof (cairo_pattern_type_t)` **4**.  */
/// C: cairo.h 2990-2996: `typedef enum _cairo_pattern_type { CAIRO_PATTERN_TYPE_SOLID, CAIRO_PATTERN_TYPE_SURFACE, ... } cairo_pattern_type_t;`
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
pub type cairo_pattern_type_t = c_int;
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
pub const CAIRO_PATTERN_TYPE_SURFACE: cairo_pattern_type_t = 1;
/// C: cairo.h 2999-3000: `cairo_public cairo_pattern_type_t cairo_pattern_get_type (cairo_pattern_t *pattern);`
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
unsafe extern "C" {
  pub fn cairo_pattern_get_type (pattern: *mut cairo_pattern_t) -> cairo_pattern_type_t;
}

/* Log error message with format string FORMAT and trailing arguments.
   Signaling an error, e.g. when an image cannot be loaded, is not a
   good idea because this would interrupt redisplay, and the error
   message display would lead to another redisplay.  This function
   therefore simply displays a message.  */

/// C: C 1429-1430: `static void image_error (const char *format, ...)`
#[cfg (feature = "have_window_system")]
unsafe extern "C" fn image_error (format: *const c_char, mut ap: ...)
{
  /* C's `va_list ap; va_start (ap, format);` and `va_end (ap);` have no port counterpart
     (PORTING.md P-04): the variadic argument is the port's `VaList`.  */
  vadd_to_log (format, ap);
}

/// C: C 1438-1439: `static void image_invalid_data_error (Lisp_Object data)`
#[cfg (feature = "have_window_system")]
unsafe fn image_invalid_data_error (data: Lisp_Object)
{
  image_error (c"Invalid image data `%s'".as_ptr (), data);
}

/// C: C 1444-1445: `static void image_not_found_error (Lisp_Object filename)`
#[cfg (feature = "have_window_system")]
unsafe fn image_not_found_error (filename: Lisp_Object)
{
  image_error (c"Cannot find image file `%s'".as_ptr (), filename);
}

/// C: C 1450-1451: `static void image_size_error (void)`
#[cfg (feature = "have_window_system")]
unsafe fn image_size_error ()
{
  image_error (c"Invalid image size (see `max-image-size')".as_ptr ());
}


/***********************************************************************
			 Image specifications
 ***********************************************************************/

/* C 1461-1477: `enum image_value_type` - the port's c_int type plus one constant per member
   (`lread.rs` 811-815's shape).  */
/// C: C 1461-1477: `enum image_value_type { IMAGE_DONT_CHECK_VALUE_TYPE, IMAGE_STRING_VALUE, IMAGE_STRING_OR_NIL_VALUE, IMAGE_SYMBOL_VALUE, IMAGE_POSITIVE_INTEGER_VALUE, IMAGE_NON_NEGATIVE_INTEGER_VALUE_OR_PAIR, IMAGE_NON_NEGATIVE_INTEGER_VALUE, IMAGE_ASCENT_VALUE, IMAGE_INTEGER_VALUE, IMAGE_FUNCTION_VALUE, IMAGE_NUMBER_VALUE, IMAGE_BOOL_VALUE }`
#[cfg (feature = "have_window_system")]
pub const IMAGE_DONT_CHECK_VALUE_TYPE: c_int = 0;
#[cfg (feature = "have_window_system")]
pub const IMAGE_STRING_VALUE: c_int = 1;
#[cfg (feature = "have_window_system")]
pub const IMAGE_STRING_OR_NIL_VALUE: c_int = 2;
#[cfg (feature = "have_window_system")]
pub const IMAGE_SYMBOL_VALUE: c_int = 3;
#[cfg (feature = "have_window_system")]
pub const IMAGE_POSITIVE_INTEGER_VALUE: c_int = 4;
#[cfg (feature = "have_window_system")]
pub const IMAGE_NON_NEGATIVE_INTEGER_VALUE_OR_PAIR: c_int = 5;
#[cfg (feature = "have_window_system")]
pub const IMAGE_NON_NEGATIVE_INTEGER_VALUE: c_int = 6;
#[cfg (feature = "have_window_system")]
pub const IMAGE_ASCENT_VALUE: c_int = 7;
#[cfg (feature = "have_window_system")]
pub const IMAGE_INTEGER_VALUE: c_int = 8;
#[cfg (feature = "have_window_system")]
pub const IMAGE_FUNCTION_VALUE: c_int = 9;
#[cfg (feature = "have_window_system")]
pub const IMAGE_NUMBER_VALUE: c_int = 10;
#[cfg (feature = "have_window_system")]
pub const IMAGE_BOOL_VALUE: c_int = 11;
#[cfg (feature = "have_window_system")]
pub type image_value_type = c_int;

/* Structure used when parsing image specifications.  */

/// C: C 1479-1502: `struct image_keyword`
#[cfg (feature = "have_window_system")]
#[repr(C)]
pub struct image_keyword
{
  /* Name of keyword.  */
  pub name: *const c_char,

  /* The type of value allowed.  */
  pub r#type: image_value_type,

  /* True means key must be present.  */
  pub mandatory_p: bool,

  /* True means key is present.
     Also used to recognize duplicate keywords in a property list.  */
  pub count: bool,

  /* The value that was found.  */
  pub value: Lisp_Object,
}


/* Parse image spec SPEC according to KEYWORDS.  A valid image spec
   has the format (image KEYWORD VALUE ...).  One of the keyword/
   value pairs must be `:type TYPE'.  KEYWORDS is a vector of
   image_keywords structures of size NKEYWORDS describing other
   allowed keyword/value pairs.  Value is true if SPEC is valid.  */

/// C: C 1505-1507: `static bool parse_image_spec (Lisp_Object spec, struct image_keyword *keywords, int nkeywords, Lisp_Object type)`
#[cfg (feature = "have_window_system")]
unsafe fn parse_image_spec (spec: Lisp_Object, keywords: *mut image_keyword, nkeywords: c_int,
			   r#type: Lisp_Object) -> bool
{
  let mut i: c_int;
  let mut plist: Lisp_Object;

  if !IMAGEP (spec)
    { return false; }

  plist = XCDR (spec);
  FOR_EACH_TAIL_SAFE! (plist, {
    let (mut key, mut value): (Lisp_Object, Lisp_Object);

    /* First element of a pair must be a symbol.  */
    key = XCAR (plist);
    plist = XCDR (plist);
    if !SYMBOLP (key)
      { return false; }

    /* There must follow a value.  */
    if !CONSP (plist)
      { return false; }
    value = XCAR (plist);

    /* C 1539's `goto maybe_done` skips the recording and the keyword's own type check; the
       label's code runs in both paths, so it is the flag the tree uses (`fileio.rs` 4880).  */
    let mut goto_maybe_done: bool = false;

    /* Find key in KEYWORDS.  Error if not found.  */
    i = 0;
    while i < nkeywords
      {
	if strcmp ((*keywords.offset (i as isize)).name, SSDATA (SYMBOL_NAME (key))) == 0
	  { break; }
	i += 1;
      }

    if i == nkeywords
      { goto_maybe_done = true; }
    else
      {
	/* Record that we recognized the keyword.  If a keyword
	   was found more than once, it's an error.  */
	(*keywords.offset (i as isize)).value = value;
	if (*keywords.offset (i as isize)).count
	  { return false; }
	(*keywords.offset (i as isize)).count = true;

	/* Check type of value against allowed type.  */
	match (*keywords.offset (i as isize)).r#type
	  {
	    IMAGE_STRING_VALUE =>
	      {
		if !STRINGP (value)
		  { return false; }
	      }

	    IMAGE_STRING_OR_NIL_VALUE =>
	      {
		if !STRINGP (value) && !NILP (value)
		  { return false; }
	      }

	    IMAGE_SYMBOL_VALUE =>
	      {
		if !SYMBOLP (value)
		  { return false; }
	      }

	    IMAGE_POSITIVE_INTEGER_VALUE =>
	      {
		if ! RANGED_FIXNUMP (1, value, INT_MAX as EMACS_INT)
		  { return false; }
	      }

	    IMAGE_NON_NEGATIVE_INTEGER_VALUE_OR_PAIR =>
	      {
		if RANGED_FIXNUMP (0, value, INT_MAX as EMACS_INT)
		  { }
		else if CONSP (value)
		     && RANGED_FIXNUMP (0, XCAR (value), INT_MAX as EMACS_INT)
		     && RANGED_FIXNUMP (0, XCDR (value), INT_MAX as EMACS_INT)
		  { }
		else
		  { return false; }
	      }

	    IMAGE_ASCENT_VALUE =>
	      {
		if SYMBOLP (value) && EQ (value, Qcenter ())
		  { }
		else if RANGED_FIXNUMP (0, value, 100)
		  { }
		else
		  { return false; }
	      }

	    IMAGE_NON_NEGATIVE_INTEGER_VALUE =>
	      {
		/* Unlike the other integer-related cases, this one does not
		   verify that VALUE fits in 'int'.  This is because callers
		   want EMACS_INT.  */
		if !FIXNATP (value)
		  { return false; }
	      }

	    IMAGE_DONT_CHECK_VALUE_TYPE =>
	      {
	      }

	    IMAGE_FUNCTION_VALUE =>
	      {
		value = indirect_function (value);
		if FUNCTIONP (value)
		  { }
		else
		  { return false; }
	      }

	    IMAGE_NUMBER_VALUE =>
	      {
		if ! NUMBERP (value)
		  { return false; }
	      }

	    IMAGE_INTEGER_VALUE =>
	      {
		if ! TYPE_RANGED_FIXNUMP (INT_MIN as EMACS_INT, value, INT_MAX as EMACS_INT)
		  { return false; }
	      }

	    IMAGE_BOOL_VALUE =>
	      {
		if !NILP (value) && !EQ (value, Qt ())
		  { return false; }
	      }

	    _ =>
	      {
		emacs_abort ();
	      }
	  }

	if !goto_maybe_done
	  {
	    if EQ (key, QCtype ())
		&& !(EQ (r#type, value) || EQ (r#type, Qnative_image ()))
	      { return false; }
	  }
      }

    /* maybe_done: */
    if NILP (XCDR (plist))
      {
	/* Check that all mandatory fields are present.  */
	i = 0;
	while i < nkeywords
	  {
	    if (*keywords.offset (i as isize)).mandatory_p
		&& !(*keywords.offset (i as isize)).count
	      { return false; }
	    i += 1;
	  }

	return true;
      }
  });

  return false;
}


/* Return the value of KEY in image specification SPEC.  Value is nil
   if KEY is not present in SPEC.  Set *FOUND depending on whether KEY
   was found in SPEC.  */

/// C: C 1646-1647: `Lisp_Object image_spec_value (Lisp_Object spec, Lisp_Object key, bool *found)` - non-static.
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn image_spec_value (spec: Lisp_Object, key: Lisp_Object,
					   found: *mut bool) -> Lisp_Object
{
  let mut tail: Lisp_Object;

  eassert! (valid_image_p (spec));

  tail = XCDR (spec);
  FOR_EACH_TAIL_SAFE! (tail, {
    if EQ (XCAR (tail), key)
      {
	if !found.is_null ()
	  { *found = true; }
	return XCAR (XCDR (tail));
      }
    tail = XCDR (tail);
    if ! CONSP (tail)
      { break; }
  });

  if !found.is_null ()
    { *found = false; }
  return Qnil ();
}

/* C 40-42 / C 114-116 / C 131: the X arm's `Emacs_Pixmap`/`Emacs_Pix_Container`, which
   `include/dispextern.rs` declares for the other platforms only (the mirror-gap list; `XImage` is
   Xlib's and stays C's token, so the X arm's container is the untyped pointer C's type is one
   word wide as - P-72 rule 3).  */
/// C: dispextern.h 41-42: `#ifndef USE_CAIRO` / `typedef Pixmap Emacs_Pixmap;`
#[cfg (all (feature = "have_window_system", feature = "have_x_windows", not (feature = "use_cairo")))]
pub type Emacs_Pixmap = Pixmap;
// C 131's `typedef Emacs_Pix_Container Emacs_Pixmap;` is carried by `include/dispextern.rs` 713
// (the mirror lane's this-batch block, same gate), so the unit's copy is dropped here - the
// P-26/P-70 same-commit move; it collided in every cairo configuration.
// The X arm's `Emacs_Pixmap` (C 41-42) is *not* in the mirror (it defers the X arm with
// `include/xlib.rs`'s `XImage`), so that declaration stays below.
/// C: dispextern.h 114-116: `#ifndef USE_CAIRO` / `typedef XImage *Emacs_Pix_Container;` (the X arm)
#[cfg (all (feature = "have_window_system", feature = "have_x_windows", not (feature = "use_cairo")))]
pub type Emacs_Pix_Container = *mut c_void;

#[cfg (feature = "have_window_system")]
DEFUN! ("image-size", Fimage_size, Simage_size, 1, 3, 0,
       doc: "Return the size of image SPEC as pair (WIDTH . HEIGHT).\nPIXELS non-nil means return the size in pixels, otherwise return the\nsize in canonical character units.\n\nFRAME is the frame on which the image will be displayed.  FRAME nil\nor omitted means use the selected frame.\n\nCalling this function will result in the image being stored in the\nimage cache.  If this is not desirable, call `image-flush' after\ncalling this function.",
       (spec: Lisp_Object, pixels: Lisp_Object, frame: Lisp_Object),
       {
  let mut size: Lisp_Object;

  size = Qnil ();
  if valid_image_p (spec)
    {
      let f: *mut frame = decode_window_system_frame (frame);
      let id: ptrdiff_t = lookup_image (f, spec, -1);
      let img: *mut image = IMAGE_FROM_ID! (f, id as c_int);
      let width: c_int = (*img).width + 2 * (*img).hmargin;
      let height: c_int = (*img).height + 2 * (*img).vmargin;

      if NILP (pixels)
	{ size = Fcons (make_float (width as c_double / FRAME_COLUMN_WIDTH (f) as c_double),
			make_float (height as c_double / FRAME_LINE_HEIGHT (f) as c_double)); }
      else
	{ size = Fcons (make_fixnum (width as EMACS_INT), make_fixnum (height as EMACS_INT)); }
    }
  else
    { error (c"Invalid image specification".as_ptr ()); }

  return size;
});


#[cfg (feature = "have_window_system")]
DEFUN! ("image-mask-p", Fimage_mask_p, Simage_mask_p, 1, 2, 0,
       doc: "Return t if image SPEC has a mask bitmap.\nFRAME is the frame on which the image will be displayed.  FRAME nil\nor omitted means use the selected frame.",
       (spec: Lisp_Object, frame: Lisp_Object),
       {
  let mut mask: Lisp_Object;

  mask = Qnil ();
  if valid_image_p (spec)
    {
      let f: *mut frame = decode_window_system_frame (frame);
      let id: ptrdiff_t = lookup_image (f, spec, -1);
      let img: *mut image = IMAGE_FROM_ID! (f, id as c_int);
      if !(*img).mask.is_null ()
	{ mask = Qt (); }
    }
  else
    { error (c"Invalid image specification".as_ptr ()); }

  return mask;
});

#[cfg (feature = "have_window_system")]
DEFUN! ("image-metadata", Fimage_metadata, Simage_metadata, 1, 2, 0,
       doc: "Return metadata for image SPEC.\nFRAME is the frame on which the image will be displayed.  FRAME nil\nor omitted means use the selected frame.",
       (spec: Lisp_Object, frame: Lisp_Object),
       {
  let mut ext: Lisp_Object;

  ext = Qnil ();
  if valid_image_p (spec)
    {
      let f: *mut frame = decode_window_system_frame (frame);
      let id: ptrdiff_t = lookup_image (f, spec, -1);
      let img: *mut image = IMAGE_FROM_ID! (f, id as c_int);
      ext = (*img).lisp_data;
    }

  return ext;
});


/***********************************************************************
		 Image type independent image structures
 ***********************************************************************/

/// C: C 1758: `#define MAX_IMAGE_SIZE 10.0`
#[cfg (feature = "have_window_system")]
pub const MAX_IMAGE_SIZE: c_double = 10.0;
/* Allocate and return a new image structure for image specification
   SPEC.  SPEC has a hash value of HASH.  */

/// C: C 1762-1763: `static struct image * make_image (Lisp_Object spec, EMACS_UINT hash)`
#[cfg (feature = "have_window_system")]
unsafe fn make_image (spec: Lisp_Object, hash: EMACS_UINT) -> *mut image
{
  let img: *mut image = xzalloc (core::mem::size_of::<image> ()) as *mut image;
  let file: Lisp_Object = image_spec_value (spec, QCfile (), core::ptr::null_mut ());

  eassert! (valid_image_p (spec));
  (*img).dependencies = if NILP (file) { Qnil () } else { list1 (file) };
  (*img).r#type = lookup_image_type (image_spec_value (spec, QCtype (), core::ptr::null_mut ()))
    as *const c_void;
  eassert! (!(*img).r#type.is_null ());
  (*img).spec = spec;
  (*img).lisp_data = Qnil ();
  (*img).ascent = DEFAULT_IMAGE_ASCENT;
  (*img).hash = hash;
  (*img).corners[BOT_CORNER] = -1;  /* Full image */
  return img;
}


/* Free image IMG which was used on frame F, including its resources.  */

/// C: C 1783-1784: `static void free_image (struct frame *f, struct image *img)`
#[cfg (feature = "have_window_system")]
unsafe fn free_image (f: *mut frame, img: *mut image)
{
  if !img.is_null ()
    {
      let c: *mut image_cache = FRAME_IMAGE_CACHE! (f);

      /* Remove IMG from the hash table of its cache.  */
      if !(*img).prev.is_null ()
	{ (*(*img).prev).next = (*img).next; }
      else
	{ *(*c).buckets.offset (((*img).hash % IMAGE_CACHE_BUCKETS_SIZE as EMACS_UINT) as isize) = (*img).next; }

      if !(*img).next.is_null ()
	{ (*(*img).next).prev = (*img).prev; }

      *(*c).images.offset ((*img).id) = core::ptr::null_mut ();

      /* C 1808-1817: `#if !defined USE_CAIRO && defined HAVE_XRENDER`.  */
      #[cfg (all (feature = "have_window_system", not (feature = "use_cairo"), feature = "have_xrender"))]
      {
	/* FRAME_X_DISPLAY (f) could be NULL if this is being called from
	   the display IO error handler.*/

	if !FRAME_X_DISPLAY! (f).is_null ()
	  {
	    if !(*img).picture.is_null ()
	      { XRenderFreePicture (FRAME_X_DISPLAY! (f), (*img).picture); }
	    if !(*img).mask_picture.is_null ()
	      { XRenderFreePicture (FRAME_X_DISPLAY! (f), (*img).mask_picture); }
	  }
      }

      /* C 1818-1820: `#ifdef HAVE_X_WINDOWS` guards the `if (FRAME_X_DISPLAY (f))` that guards
	 the *next statement*; with the X arm off the statement is unguarded.  Rust has no cfg on
	 a trailing guard, so the port keeps both arms in the condition the guarded statement
	 reads (`C's implicit conversions written out`'s rule, `docs/ports/image.md`).  */
      #[cfg (all (feature = "have_window_system", feature = "have_x_windows"))]
      let guard_free_img: bool = !FRAME_X_DISPLAY! (f).is_null ();
      #[cfg (all (feature = "have_window_system", not (feature = "have_x_windows")))]
      let guard_free_img: bool = true;
      if guard_free_img
	{
	  /* Free resources, then free IMG.  */
	  (*((*img).r#type as *const image_type)).free_img.unwrap () (f, img);
	}

      xfree ((*img).face_font_family as *mut c_void);
      xfree (img as *mut c_void);
    }
}

/* Return true if the given widths and heights are valid for display.  */

/// C: C 1829-1830: `static bool check_image_size (struct frame *f, int width, int height)`
#[cfg (feature = "have_window_system")]
unsafe fn check_image_size (f: *mut frame, width: c_int, height: c_int) -> bool
{
  let (mut w, mut h): (c_int, c_int);

  if width <= 0 || height <= 0
    { return false; }

  if FIXNUMP (Vmax_image_size)
    { return (width as EMACS_INT <= XFIXNUM (Vmax_image_size)
	      && height as EMACS_INT <= XFIXNUM (Vmax_image_size)); }
  else if FLOATP (Vmax_image_size)
    {
      if !f.is_null ()
	{
	  w = FRAME_PIXEL_WIDTH (f);
	  h = FRAME_PIXEL_HEIGHT (f);
	}
      else
	{ w = 1024; h = 1024; }  /* Arbitrary size for unknown frame. */
      return (width as c_double <= XFLOAT_DATA (Vmax_image_size) * w as c_double
	      && height as c_double <= XFLOAT_DATA (Vmax_image_size) * h as c_double);
    }
  else
    { return true; }
}

/* Prepare image IMG for display on frame F.  Must be called before
   drawing an image.  */

/// C: C 1859-1860: `void prepare_image_for_display (struct frame *f, struct image *img)` - non-static.
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn prepare_image_for_display (f: *mut frame, img: *mut image)
{
  /* We're about to display IMG, so set its timestamp to `now'.  */
  (*img).timestamp = current_timespec ();

  /* If IMG doesn't have a pixmap yet, load it now, using the image
     type dependent loader function.  */
  if (*img).pixmap == NO_PIXMAP! () as *mut c_void && !(*img).load_failed_p
    { (*img).load_failed_p = ! (*((*img).r#type as *const image_type)).load_img.unwrap () (f, img); }

  /* C 1877-1889: `#ifdef USE_CAIRO` / `#elif defined HAVE_X_WINDOWS || defined HAVE_ANDROID`.  */
  #[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
  if !(*img).load_failed_p
    {
      block_input ();
      if (*img).cr_data.is_null ()
	  || (cairo_pattern_get_type ((*img).cr_data as *mut cairo_pattern_t)
	      != CAIRO_PATTERN_TYPE_SURFACE)
	{
	  /* Fill in the background/background_transparent field while
	     we have img->pixmap->data/img->mask->data.  */
	  IMAGE_BACKGROUND! (img, f, (*img).pixmap);
	  IMAGE_BACKGROUND_TRANSPARENT! (img, f, (*img).mask);
	  cr_put_image_to_cr_data (img);
	  if (*img).cr_data.is_null ()
	    {
	      (*img).load_failed_p = true;
	      (*((*img).r#type as *const image_type)).free_img.unwrap () (f, img);
	    }
	}
      unblock_input ();
    }
  #[cfg (all (feature = "have_window_system", not (feature = "use_cairo"),
	      any (feature = "have_x_windows", feature = "have_android")))]
  if !(*img).load_failed_p
    {
      block_input ();
      image_sync_to_pixmaps (f, img);
      unblock_input ();
    }

  /* Update image pixmap from canvas pixel buffer if refresh counter has
     been updated.  For canvases the refresh counter is always >= 1.  */
  if (*img).refresh != 0
      && EQ (image_spec_value ((*img).spec, QCtype (), core::ptr::null_mut ()), Qcanvas ())
      && (*img).refresh != (*(XFIXNUMPTR ((*img).lisp_data) as *mut canvas)).refresh
    { canvas_prepare_for_display (f, img); }
}


/* Value is the number of pixels for the ascent of image IMG when
   drawn in face FACE.  */

/// C: C 1911-1912: `int image_ascent (struct image *img, struct face *face, struct glyph_slice *slice)` - non-static.
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn image_ascent (img: *mut image, face: *mut face, slice: *mut glyph_slice)
  -> c_int
{
  let mut height: c_int;
  let mut ascent: c_int;

  /* `struct glyph_slice`'s `x`/`y`/`width`/`height` are C's bit members (dispextern.h 455-460),
     read through the mirror's own accessor (P-80).  */
  if glyph_slice_bits! (height, (*slice)) as c_int == (*img).height
    { height = (*img).height + (*img).vmargin; }
  else if glyph_slice_bits! (y, (*slice)) as c_int == 0
    { height = glyph_slice_bits! (height, (*slice)) as c_int + (*img).vmargin; }
  else
    { height = glyph_slice_bits! (height, (*slice)) as c_int; }

  if (*img).ascent == CENTERED_IMAGE_ASCENT
    {
      if !(*face).font.is_null ()
	{
	  /* C 1925-1934: `#ifdef HAVE_NTGUI`'s W32-specific expression and its `#else`.  */
	  #[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
	  /* W32 specific version.  Why?. ++kfs  */
	  { ascent = height / 2 - (FONT_DESCENT! ((*face).font as *mut font)
	       - FONT_BASE! ((*face).font as *mut font)) / 2; }
	  #[cfg (all (feature = "have_window_system", not (feature = "have_ntgui")))]
	  {
	    /* This expression is arranged so that if the image can't be
	       exactly centered, it will be moved slightly up.  This is
	       because a typical font is `top-heavy' (due to the presence
	       uppercase letters), so the image placement should err towards
	       being top-heavy too.  It also just generally looks better.  */
	    ascent = (height + FONT_BASE! ((*face).font as *mut font)
		     - FONT_DESCENT! ((*face).font as *mut font) + 1) / 2;
	  }
	}
      else
	{ ascent = height / 2; }
    }
  else
    { ascent = (height as c_double * ((*img).ascent as c_double / 100.0)) as c_int; }

  return ascent;
}


/* Image background colors.  */

/* Find the "best" corner color of a bitmap.
   On W32, PIMG is assumed to a device context with the bitmap selected.  */

/// C: C 1957-1959: `static RGB_PIXEL_COLOR four_corners_best (Emacs_Pix_Context pimg, int *corners, unsigned long width, unsigned long height)`
#[cfg (feature = "have_window_system")]
unsafe fn four_corners_best (mut pimg: Emacs_Pix_Context, corners: *mut c_int,
			     mut width: c_ulong, mut height: c_ulong) -> RGB_PIXEL_COLOR
{
  let mut corner_pixels: [RGB_PIXEL_COLOR; 4] = [0; 4];
  /* C's `RGB_PIXEL_COLOR best UNINIT;` - zeroed (PORTING.md 1.5/0.1-13).  */
  let mut best: RGB_PIXEL_COLOR = 0;
  let (mut i, mut best_count): (c_int, c_int);

  /* C 1965-1970: `#ifdef USE_CAIRO`.  */
  #[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
  {
    /* Sometimes the Cairo codepath calls this function *after* the image
       sizes have been modified by native-transforms, so the pimg
       dimensions don't match WIDTH and HEIGHT.  */
    width = (*pimg).width as c_ulong;
    height = (*pimg).height as c_ulong;
  }

  if !corners.is_null () && *corners.offset (BOT_CORNER as isize) >= 0
    {
      /* Get the colors at the corner_pixels of pimg.  */
      corner_pixels[0] = GET_PIXEL! (pimg, *corners.offset (LEFT_CORNER as isize),
				     *corners.offset (TOP_CORNER as isize));
      corner_pixels[1] = GET_PIXEL! (pimg, *corners.offset (RIGHT_CORNER as isize) - 1,
				     *corners.offset (TOP_CORNER as isize));
      corner_pixels[2] = GET_PIXEL! (pimg, *corners.offset (RIGHT_CORNER as isize) - 1,
				     *corners.offset (BOT_CORNER as isize) - 1);
      corner_pixels[3] = GET_PIXEL! (pimg, *corners.offset (LEFT_CORNER as isize),
				     *corners.offset (BOT_CORNER as isize) - 1);
    }
  else

    {
      /* Get the colors at the corner_pixels of pimg.  */
      corner_pixels[0] = GET_PIXEL! (pimg, 0, 0);
      corner_pixels[1] = GET_PIXEL! (pimg, width as c_int - 1, 0);
      corner_pixels[2] = GET_PIXEL! (pimg, width as c_int - 1, height as c_int - 1);
      corner_pixels[3] = GET_PIXEL! (pimg, 0, height as c_int - 1);
    }
  /* Choose the most frequently found color as background.  */
  i = 0; best_count = 0;
  while i < 4
    {
      let (mut j, mut n): (c_int, c_int);

      j = 0; n = 0;
      while j < 4
	{
	  if corner_pixels[i as usize] == corner_pixels[j as usize]
	    { n += 1; }
	  j += 1;
	}

      if n > best_count
	{ best = corner_pixels[i as usize]; best_count = n; }
      i += 1;
    }

  return best;
}

/// C: C 2006-2007: `static Lisp_Object make_color_name (unsigned int red, unsigned int green, unsigned int blue)`
#[cfg (feature = "have_window_system")]
unsafe fn make_color_name (red: c_uint, green: c_uint, blue: c_uint) -> Lisp_Object
{
  return make_formatted_string (c"#%04x%04x%04x".as_ptr (), red, green, blue);
}

/* Return the `background' field of IMG.  If IMG doesn't have one yet,
   it is guessed heuristically.  If non-zero, XIMG is an existing
   Emacs_Pix_Context object (device context with the image selected on W32) to use for the
   heuristic.  */

/// C: C 2017-2018: `RGB_PIXEL_COLOR image_background (struct image *img, struct frame *f, Emacs_Pix_Context pimg)` - non-static.
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn image_background (img: *mut image, f: *mut frame,
					   mut pimg: Emacs_Pix_Context) -> RGB_PIXEL_COLOR
{
  if (*img).bits & IMAGE_BACKGROUND_VALID_MASK == 0
    /* IMG doesn't have a background yet, try to guess a reasonable value.  */
    {
      let free_pimg: bool = pimg.is_null ();
      /* C 2021-2023: `#ifdef HAVE_NTGUI`'s `HGDIOBJ prev;`.  The non-NTGUI arm's `prev` never
	 reaches the expansion (`image_unget_x_image_or_dc`'s macro drops its `$dummy`), so the
	 declaration is C's own - and the argument below stays C's text in both arms.  */
      #[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
      let mut prev: HGDIOBJ = core::ptr::null_mut ();

      if free_pimg
	{ pimg = image_get_x_image_or_dc! (f, img, false, &raw mut prev); }

      let bg: RGB_PIXEL_COLOR
	= four_corners_best (pimg, &raw mut (*img).corners as *mut c_int,
			     (*img).width as c_ulong, (*img).height as c_ulong);
      /* C 2029-2033: `#ifdef USE_CAIRO`.  */
      #[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
      let mut bg: RGB_PIXEL_COLOR = bg;
      #[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
      {
	let color_name: Lisp_Object = make_color_name (RED16_FROM_ULONG! (bg), GREEN16_FROM_ULONG! (bg),
						       BLUE16_FROM_ULONG! (bg));
	bg = image_alloc_image_color (f, img, color_name, 0);
      }
      (*img).background = bg;

      if free_pimg
	{ image_unget_x_image_or_dc! (img, false, pimg, prev); }

      /* C's `img->background_valid = 1;` - the bit member's storage unit (P-80).  */
      (*img).bits = (*img).bits & !IMAGE_BACKGROUND_VALID_MASK
		    | ((1 as u32) << IMAGE_BACKGROUND_VALID_SHIFT);
    }

  return (*img).background;
}

/* Return the `background_transparent' field of IMG.  If IMG doesn't
   have one yet, it is guessed heuristically.  If non-zero, MASK is an
   existing Emacs_Pix_Context (XImage* on X) object to use for the
   heuristic.  */

/// C: C 2055-2057: `int image_background_transparent (struct image *img, struct frame *f, Emacs_Pix_Context mask)` - non-static.
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn image_background_transparent (img: *mut image, f: *mut frame,
						       mut mask: Emacs_Pix_Context) -> c_int
{
  if (*img).bits & IMAGE_BACKGROUND_TRANSPARENT_VALID_MASK == 0
    /* IMG doesn't have a background yet, try to guess a reasonable value.  */
    {
      if !(*img).mask.is_null ()
	{
	  let free_mask: bool = mask.is_null ();
	  /* C 2064-2066: `#ifdef HAVE_NTGUI`'s `HGDIOBJ prev;` (see `image_background`).  */
	  #[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
	  let mut prev: HGDIOBJ = core::ptr::null_mut ();

	  if free_mask
	    { mask = image_get_x_image_or_dc! (f, img, true, &raw mut prev); }

	  /* C's `img->background_transparent = (… == PIX_MASK_RETAIN);`.  */
	  let transparent: bool
	    = four_corners_best (mask, &raw mut (*img).corners as *mut c_int,
				 (*img).width as c_ulong, (*img).height as c_ulong)
	      == PIX_MASK_RETAIN! () as RGB_PIXEL_COLOR;
	  (*img).bits = (*img).bits & !IMAGE_BACKGROUND_TRANSPARENT_MASK
			| ((transparent as u32) << IMAGE_BACKGROUND_TRANSPARENT_SHIFT);

	  if free_mask
	    { image_unget_x_image_or_dc! (img, true, mask, prev); }
	}
      else
	{ (*img).bits = (*img).bits & !IMAGE_BACKGROUND_TRANSPARENT_MASK
			| ((0 as u32) << IMAGE_BACKGROUND_TRANSPARENT_SHIFT); }

      (*img).bits = (*img).bits & !IMAGE_BACKGROUND_TRANSPARENT_VALID_MASK
		    | ((1 as u32) << IMAGE_BACKGROUND_TRANSPARENT_VALID_SHIFT);
    }

  return ((*img).bits >> IMAGE_BACKGROUND_TRANSPARENT_SHIFT & 1) as c_int;
}

/***********************************************************************
		  Helper functions for X image types
 ***********************************************************************/

/* Clear X resources of image IMG on frame F according to FLAGS.
   FLAGS is bitwise-or of the following masks:
   CLEAR_IMAGE_PIXMAP free the pixmap if any.
   CLEAR_IMAGE_MASK means clear the mask pixmap if any.
   CLEAR_IMAGE_COLORS means free colors allocated for the image, if
     any.  */

/// C: C 2102-2103: `static void image_clear_image_1 (struct frame *f, struct image *img, int flags)`
#[cfg (feature = "have_window_system")]
unsafe fn image_clear_image_1 (f: *mut frame, img: *mut image, flags: c_int)
{
  if flags & CLEAR_IMAGE_PIXMAP! () != 0
    {
      if !(*img).pixmap.is_null ()
	{
	  (*FRAME_TERMINAL! (f)).free_pixmap.unwrap () (f as *mut c_void, (*img).pixmap);
	  (*img).pixmap = NO_PIXMAP! () as *mut c_void;
	  /* NOTE (HAVE_NS): background color is NOT an indexed color! */
	  (*img).bits = (*img).bits & !IMAGE_BACKGROUND_VALID_MASK
			| ((0 as u32) << IMAGE_BACKGROUND_VALID_SHIFT);
	}
      /* C 2114-2121: `#if (defined HAVE_X_WINDOWS || defined HAVE_ANDROID) && !defined USE_CAIRO`.  */
      #[cfg (all (feature = "have_window_system", any (feature = "have_x_windows", feature = "have_android"),
		  not (feature = "use_cairo")))]
      if !(*img).ximg.is_null ()
	{
	  image_destroy_x_image ((*img).ximg as Emacs_Pix_Container);
	  (*img).ximg = core::ptr::null_mut ();
	  (*img).bits = (*img).bits & !IMAGE_BACKGROUND_VALID_MASK
			| ((0 as u32) << IMAGE_BACKGROUND_VALID_SHIFT);
	}
    }

  if flags & CLEAR_IMAGE_MASK! () != 0
    {
      if !(*img).mask.is_null ()
	{
	  (*FRAME_TERMINAL! (f)).free_pixmap.unwrap () (f as *mut c_void, (*img).mask);
	  (*img).mask = NO_PIXMAP! () as *mut c_void;
	  (*img).bits = (*img).bits & !IMAGE_BACKGROUND_TRANSPARENT_VALID_MASK
			| ((0 as u32) << IMAGE_BACKGROUND_TRANSPARENT_VALID_SHIFT);
	}
      /* C 2132-2139: `#if (defined HAVE_X_WINDOWS || defined HAVE_ANDROID) && !defined USE_CAIRO`.  */
      #[cfg (all (feature = "have_window_system", any (feature = "have_x_windows", feature = "have_android"),
		  not (feature = "use_cairo")))]
      if !(*img).mask_img.is_null ()
	{
	  image_destroy_x_image ((*img).mask_img as Emacs_Pix_Container);
	  (*img).mask_img = core::ptr::null_mut ();
	  (*img).bits = (*img).bits & !IMAGE_BACKGROUND_TRANSPARENT_VALID_MASK
			| ((0 as u32) << IMAGE_BACKGROUND_TRANSPARENT_VALID_SHIFT);
	}
    }

  if (flags & CLEAR_IMAGE_COLORS! ()) != 0 && (*img).ncolors != 0
    {
      /* W32_TODO: color table support.  */
      /* C 2145-2147: `#if defined HAVE_X_WINDOWS && !defined USE_CAIRO`.  */
      #[cfg (all (feature = "have_window_system", feature = "have_x_windows", not (feature = "use_cairo")))]
      x_free_colors (f, (*img).colors, (*img).ncolors);
      xfree ((*img).colors as *mut c_void);
      (*img).colors = core::ptr::null_mut ();
      (*img).ncolors = 0;
    }

  /* C 2153-2159: `#ifdef USE_CAIRO`.  */
  #[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
  if !(*img).cr_data.is_null ()
    {
      cairo_pattern_destroy ((*img).cr_data as *mut cairo_pattern_t);
      (*img).cr_data = core::ptr::null_mut ();
    }
}

/* Free X resources of image IMG which is used on frame F.  */

/// C: C 2164-2165: `static void image_clear_image (struct frame *f, struct image *img)`
#[cfg (feature = "have_window_system")]
unsafe fn image_clear_image (f: *mut frame, img: *mut image)
{
  (*img).lisp_data = Qnil ();
  block_input ();
  image_clear_image_1 (f, img,
		       (CLEAR_IMAGE_PIXMAP! ()
			| CLEAR_IMAGE_MASK! ()
			| CLEAR_IMAGE_COLORS! ()));
  unblock_input ();
}


/* Allocate color COLOR_NAME for image IMG on frame F.  If color
   cannot be allocated, use DFLT.  Add a newly allocated color to
   IMG->colors, so that it can be freed again.  Value is the pixel
   color.  */

/// C: C 2182-2184: `static unsigned long image_alloc_image_color (struct frame *f, struct image *img, Lisp_Object color_name, unsigned long dflt)`
#[cfg (feature = "have_window_system")]
unsafe fn image_alloc_image_color (f: *mut frame, img: *mut image, color_name: Lisp_Object,
				   dflt: c_ulong) -> c_ulong
{
  /* C's `Emacs_Color color;` - zeroed (PORTING.md 1.5/0.1-13).  */
  let mut color: Emacs_Color = core::mem::zeroed ();
  let result: c_ulong;

  eassert! (STRINGP (color_name));

  if (*FRAME_TERMINAL! (f)).defined_color_hook.unwrap () (f as *mut c_void, SSDATA (color_name),
							  &raw mut color, true, false)
      && ((*img).ncolors as usize)
	   < std::cmp::min (std::cmp::min (PTRDIFF_MAX as usize, SIZE_MAX)
			    / core::mem::size_of::<c_ulong> (), INT_MAX as usize)
    {
      /* This isn't called frequently so we get away with simply
	 reallocating the color vector to the needed size, here.  */
      let ncolors: ptrdiff_t = ((*img).ncolors + 1) as ptrdiff_t;
      (*img).colors = xrealloc ((*img).colors as *mut c_void,
				(ncolors as usize) * core::mem::size_of::<c_ulong> ()) as *mut c_ulong;
      *(*img).colors.offset (ncolors - 1) = color.pixel;
      (*img).ncolors = ncolors as c_int;
      result = color.pixel;
    }
  else
    { result = dflt; }

  return result;
}
