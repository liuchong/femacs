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
// **Eight slices are ported: C 1-1095, C 1096-2211, C 2212-3550, C 3552-4623, C 4620-6163,
// C 6166-7967, C 7970-8351 and C 13301-13805** (the file is 13,805 lines).  **The unit is
// complete in `extract-abi.py --check`'s sense: it reads `OK`** - the eighth slice landed the
// live tail (the four `DEFUN`s, `initialize_image_type`, the `image_types[]` table,
// `native_image_type`, `lookup_image_type`, `image_prune_animation_caches` and `syms_of_image`
// with its five `DEFVAR_*` invocations, 43 `DEFSYM`s, `add_image_type` calls, `staticpro` and
// eleven `defsubr`s; docs/ports/image.md Evidence 36-39).  What the port does **not** carry is
// the recorded remainder C 8352-13300 - the eight format loaders and the
// `HAVE_NATIVE_IMAGE_API`/`HAVE_NATIVE_TRANSFORMS` helpers whose `#if`s the frozen configuration
// makes false - which the ledger's range table records instead of transcribing (P-08's shape)
// and which is why the unit's line-count row (12,104 Rust against C's 13,805, -12.3%) keeps the
// `total` basis.  The seventh slice is the PBM loader: the banner,
// `enum pbm_keyword_index` (C 7976-7990), the `pbm_format` table (C 7995-8008) and the five
// functions `pbm_image_p`, `pbm_next_char`, `pbm_scan_number`, `pbm_scan_index` and `pbm_load`
// (C 8097-8351, whose `error:` label is one labelled block and whose `ckd_mul`s are the
// `overflowing_mul` pair the family checker reads).  It and the eighth slice are the unit's
// **type-checked** text - the `have_ns` probe compiles both (0 errors, Evidence 32 and 36).  The
// sixth slice ends at the `}` of `image_build_heuristic_mask` (C 7967) and holds: the
// `HAVE_XPM || HAVE_NS || HAVE_HAIKU || HAVE_PGTK || HAVE_ANDROID` predicate pair
// (`xpm_valid_color_symbols_p` C 6213, `xpm_image_p` C 6231), the `WINDOWSNT` XPM DLL block
// (C 6166-6204), `x_create_bitmap_from_xpm_data` (C 6249), the plain-X11 `xpm_load` (C 6307), the
// libxpm-free XPM implementation (C 6615-7122: `xpm_scan`, the two colour-table pairs,
// `xpm_load_image` and its second `xpm_load`), the "Color table" section (C 7126-7396) and the
// "Algorithms" section (C 7398-7967).
// The first slice (C 1-1095) ends at the `}` of `image_create_bitmap_from_file` (C 1095) and holds:
// the include graph and the
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
/* lisp.h 41 is its `byteswap` include (`#include <byteswap.h>`), whose `bswap_32` this file's
   `canvas_apply_data` (C 5592/5651) reads.  The mirror that owns C 41's chain
   (`include/lisp.rs`) does not carry the edge yet, so it stands here at the C position - the
   P-90 ③ shape the cairo edge above uses.  */
#[cfg (feature = "have_window_system")]
include!("../lib/include/byteswap.in.rs");     // lisp.h 41: `byteswap` (`bswap_32`)
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

/* C 64's `TERM_HEADER` chain is what brings cairo (and GTK) into this unit: `xterm.h` 98's
   `<cairo-xlib.h>` under `USE_CAIRO` and 69's `<gtk/gtk.h>` under `USE_GTK`, `pgtkterm.h` 31's
   `<gtk/gtk.h>`, and the NS/Haiku/Android headers' own.  Their declarations are the SDK mirror
   `femacs/src/include/gtk.rs`'s (P-90 ①/②), and the mirrors that own those C edges do not carry
   the edge yet (`include/xterm.rs`, `include/nsterm.rs`; `include/pgtkterm.rs`'s C 31 edge is
   owed for the same reason), so the edge stands here, at the C position that makes the names
   visible - the `src/xwidget.rs` C 30 stand-in shape.  It carries this unit's cairo block's own
   condition, C 92's `#ifdef USE_CAIRO`, and it stands **above** the `TERM_HEADER` block on
   purpose: when `include/pgtkterm.rs`'s C 31 edge lands it is the *second* reach of this same
   mirror in this TU, and the first reach claims the file's one expansion slot (P-89) - measured
   with the edge placed below, xtask expanded `include/gtk.rs` at the pgtkterm reach and this
   unit's X/cairo arm (where that reach is gated off) lost all ten names it needs.  */
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
include!("include/gtk.rs");                    // TERM_HEADER: the cairo surface `include/gtk.rs` carries

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
   cairo is a third-party library header with no file in the reference tree.  Its declarations are
   the SDK mirror `femacs/src/include/gtk.rs`'s (P-90 ①② - cairo is part of that surface), and
   this unit reaches it through the edge below, at C 64's `TERM_HEADER` position; what the unit
   still declares are the `<cairo.h>` names that mirror does not carry.  Every constant and record
   here is a **clang reading** from this machine's `/opt/homebrew/include/cairo/cairo.h`
   (cairo 1.18.4), quoted in the ledger.  */
/// C: C 100-101: `static unsigned long image_alloc_image_color (struct frame *, struct image *, Lisp_Object, unsigned long);`
/// The first slice kept this declaration because C 2318's definition was outside it; the second
/// slice's C 2182-2211 **is** the definition, so the prototype is dropped here (PORTING.md 0.1-12:
/// a prototype and its definition in one translation unit have no Rust counterpart, and Rust
/// items are order-independent), and `docs/ports/image.md` records the move.
/* cairo.h 62-64/197/216's `cairo_surface_t`, `cairo_pattern_t`, `cairo_format_t` and
   `cairo_status_t` are `include/gtk.rs`'s now, reached through the edge below - the names the two
   mirrors shared (P-88 ⑤'s move; `## Evidence` of `docs/ports/image.md`).  */
/// C: cairo.h 246: `typedef enum _cairo_extend { CAIRO_EXTEND_NONE, CAIRO_EXTEND_REPEAT, ... } cairo_extend_t;`
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
pub type cairo_extend_t = c_int;
/// C: cairo.h 259: `typedef enum _cairo_filter { CAIRO_FILTER_FAST, ... } cairo_filter_t;`
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
pub type cairo_filter_t = c_int;
/// C: cairo.h 2691: `typedef void (*cairo_destroy_func_t) (void *);`
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
pub type cairo_destroy_func_t = Option<unsafe extern "C" fn (data: *mut c_void)>;
/// C: cairo.h 217-221 - measured: `CAIRO_FORMAT_RGB24` **1**, `CAIRO_FORMAT_A8` **2**,
/// `CAIRO_FORMAT_A1` **3** (`CAIRO_FORMAT_ARGB32` **0** is `include/gtk.rs`'s now).
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
/// C: cairo.h 259-266: `typedef enum _cairo_filter { CAIRO_FILTER_FAST, CAIRO_FILTER_GOOD,
/// CAIRO_FILTER_BEST, CAIRO_FILTER_NEAREST, CAIRO_FILTER_BILINEAR, CAIRO_FILTER_GAUSSIAN } cairo_filter_t;`
/// - measured on this machine's cairo 1.18.4: `FAST` **0**, `GOOD` **1**, `BEST` **2**,
/// `NEAREST` **3**, `BILINEAR` **4**, `GAUSSIAN` **5**.  `image_set_transform`'s cairo arm is
/// their first reader in this unit.
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
pub const CAIRO_FILTER_FAST: cairo_filter_t = 0;
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
pub const CAIRO_FILTER_GOOD: cairo_filter_t = 1;
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
pub const CAIRO_FILTER_BEST: cairo_filter_t = 2;
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
pub const CAIRO_FILTER_NEAREST: cairo_filter_t = 3;
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
/// C: cairo.h 2934: `cairo_public cairo_pattern_t *cairo_pattern_create_rgb (double, double, double);`
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
  pub fn cairo_surface_set_user_data (surface: *mut cairo_surface_t,
                                      key: *const cairo_user_data_key_t, user_data: *mut c_void,
                                      destroy: cairo_destroy_func_t) -> cairo_status_t;
  pub fn cairo_pattern_create_for_surface (surface: *mut cairo_surface_t) -> *mut cairo_pattern_t;
  pub fn cairo_pattern_create_rgb (red: c_double, green: c_double, blue: c_double)
    -> *mut cairo_pattern_t;
  pub fn cairo_pattern_get_matrix (pattern: *mut cairo_pattern_t, matrix: *mut cairo_matrix_t);
  pub fn cairo_pattern_set_matrix (pattern: *mut cairo_pattern_t, matrix: *const cairo_matrix_t);
  pub fn cairo_pattern_set_filter (pattern: *mut cairo_pattern_t, filter: cairo_filter_t);
  pub fn cairo_pattern_get_filter (pattern: *mut cairo_pattern_t) -> cairo_filter_t;
  pub fn cairo_pattern_destroy (pattern: *mut cairo_pattern_t);
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
/* C 7716-7735's `image_pixmap_draw_cross` reads seven more entry points of the same
   `<cairo.h>`; they are `include/gtk.rs`'s surface by P-90 ① (the GTK/cairo surface has more
   than one consumer), so these copies are owed to that mirror with the rest of this unit's cairo
   section (`docs/ports/image.md`).  */
/* cairo_create (cairo.h 539-540) and cairo_destroy (cairo.h 545-546) are `include/gtk.rs`'s
   already (the P-90 round moved ten of this surface's names there), so this slice adds only the
   five it reads beyond them.  */
/// C: cairo.h 704-705: `cairo_public void cairo_set_source_rgb (cairo_t *cr, double red, double green, double blue);`
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
unsafe extern "C" {
  pub fn cairo_set_source_rgb (cr: *mut cairo_t, red: c_double, green: c_double, blue: c_double);
}
/* C: cairo.h 810-811: `cairo_public void cairo_set_line_width (cairo_t *cr, double width);` -
   the row is `include/gtk.rs`'s now (P-26's one spelling per construct, status 3.732) and this
   unit's call sites are unchanged.  */
/// C: cairo.h 906-907: `cairo_public void cairo_move_to (cairo_t *cr, double x, double y);`
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
unsafe extern "C" {
  pub fn cairo_move_to (cr: *mut cairo_t, x: c_double, y: c_double);
}
/// C: cairo.h 941-942: `cairo_public void cairo_rel_move_to (cairo_t *cr, double dx, double dy);`
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
unsafe extern "C" {
  pub fn cairo_rel_move_to (cr: *mut cairo_t, dx: c_double, dy: c_double);
}
/// C: cairo.h 944-945: `cairo_public void cairo_rel_line_to (cairo_t *cr, double dx, double dy);`
#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
unsafe extern "C" {
  pub fn cairo_rel_line_to (cr: *mut cairo_t, dx: c_double, dy: c_double);
}
/* C: cairo.h 989-990: `cairo_public void cairo_stroke (cairo_t *cr);` - the row is
   `include/gtk.rs`'s now (P-26's one spelling per construct, status 3.732).  */

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

/* C 224-229: `image_disable_image`/`image_edge_detection`/`image_compute_scale` - `static`, all
   three defined outside this slice: dropped (PORTING.md 0.1-12).  The third is C 2727's own
   definition in the file's third slice, and the first two are called by C 2606's
   `postprocess_image`, so their declarations stand in that slice's use-site block.  */
/* C 231-232: `init_color_table`/`lookup_rgb_color` - `static`: dropped.  */
/* C 233-236: `#ifdef COLOR_TABLE_SUPPORT`'s `free_color_table`/`colors_in_color_table` - `static`,
   under C 169-173's derived condition: dropped.  */
/* C 238-242: `#ifdef HAVE_NTGUI`'s two `static` prototypes - `w32_create_pixmap_from_bitmap_data`
   (C 239, defined at C 4940) and `XPutPixel` (C 240, defined at C 7492).  Both are called by this
   slice's Windows arm, so the arm reaches names whose definitions are the *next* slices'; that is
   recorded in `docs/ports/image.md` rather than papered over with a stub (the arm is text-only on
   this machine anyway - no Windows toolchain).  */
/// C: C 244-246: `#if defined (HAVE_WEBP) || defined (HAVE_GIF) / static void anim_prune_animation_cache (Lisp_Object); / #endif` -
/// a `static` whose definition is C 9184's; C 2490's `Fclear_image_cache` is the call site, so
/// the declaration stands in the third slice's use-site block under this same condition.

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

    USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);

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

    SAFE_FREE! (sa_count, sa_mark);

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

/* `struct face` (C dispextern.h 1787-1937) - the use-site copy the display units carry, for the
   reason the mirror's `dispextern_face_records` ruling records (status 3.220): the mirror
   keeps the record behind a `dead-arms` feature that no configuration produces, so on
   Android the name is out of scope (`src/xfaces.rs` 288 is the oldest copy).  The text is
   the mirror's; the assertions hold for both targets, and the Android arm's numbers are
   measured in docs/baseline/face-layout-android.tsv.  */
/// C 1787-1937: `struct face`.  Measured with the frozen clang: `sizeof` 280, align 8,
/// `lface`@0, `id`@160, `foreground`@168, `background`@176, `underline_color`@184,
/// `overline_color`@192, `strike_through_color`@200, `box_color`@208, `font`@216,
/// `fontset`@224, `box_vertical_line_width`@228, `box_horizontal_line_width`@232,
/// `underline_pixels_above_descent_line`@236, the 21 `bool_bf`/`ENUM_BF` bits C declares at
/// 1893-1935 in one 4-byte storage unit at 240, `hash`@248, `next`@256, `prev`@264,
/// `ascii_face`@272.
#[repr(C)]
pub struct face
{
  pub lface: [Lisp_Object; LFACE_VECTOR_SIZE as usize],
  pub id: c_int,

  // dispextern.h 1797-1811: the `gc` (C's three arms) and `stipple` that C adds to the face
  // record under `#ifdef HAVE_WINDOW_SYSTEM`.  Read from `docs/baseline/face-layout.tsv` (P-82: frozen
  // 280/8, both window-system arms 296/8, `gc` at 168 and `stipple` at 176 in both), which is
  // also why the two window-system arms add the same sixteen bytes.
  #[cfg (all (feature = "have_window_system", feature = "have_x_windows"))]
  pub gc: *mut c_void,        /// C: `GC gc;` (1799) - X11's `struct _XGC *`; no mirror, so the
                              /// pointer stays untyped (P-72 rule 3, as in `glyph_string`)
  #[cfg (all (feature = "have_window_system", feature = "have_android"))]
  pub gc: *mut c_void,        /// C: `struct android_gc *gc;` (1801)
  #[cfg (all (feature = "have_window_system", not (feature = "have_x_windows"),
              not (feature = "have_android")))]
  pub gc: *mut Emacs_GC,      // C: `Emacs_GC *gc;` (1803)
  #[cfg (feature = "have_window_system")]
  pub stipple: ptrdiff_t,     // 1810 (measured 176)
  pub foreground: c_ulong,
  pub background: c_ulong,
  pub underline_color: c_ulong,
  pub overline_color: c_ulong,
  pub strike_through_color: c_ulong,
  pub box_color: c_ulong,
  /* P-72 rule ③: `struct font` is font.rs's type and this mirror cannot reach it (C's
     dispextern.h only knows the incomplete tag), so the member is the untyped pointer; a unit
     that includes `include/font.rs` casts it (`src/xfaces.rs` 294 spells it `*mut font`).  */
  pub font: *mut c_void,   /// C: `struct font *font;` (dispextern.h 1845)
  pub fontset: c_int,
  pub box_vertical_line_width: c_int,
  pub box_horizontal_line_width: c_int,
  pub underline_pixels_above_descent_line: c_int,
  /* C 1893-1935: `ENUM_BF (face_box_type) box : 2; ENUM_BF (face_underline_type)
     underline : 3;` and the seventeen 1-bit `bool_bf` members in one storage unit.  The port
     stores C's bit-field bytes and names every member through the masks below (PORTING.md
     1.3/P-80).  */
  pub bits: u32,
  pub hash: uintptr_t,
  pub next: *mut face,
  pub prev: *mut face,
  pub ascii_face: *mut face,
}

/* C 1797-1811's two `HAVE_WINDOW_SYSTEM` members sit between `id` and `foreground`, so every
   member from `foreground` on is sixteen bytes further along in a window-system build
   (`docs/baseline/face-layout.tsv`: `frozen` 280/8 with `foreground`@168, `x11`/`ns_cocoa`/
   `x11_ext` 296/8 with `foreground`@184 - the same two members, measured in both).  The
   assertions below add this shift rather than repeating a second table, so the one set of
   measured numbers covers every arm; status 3.505 added it after `src/fringe.rs`'s flip read
   `13 x E0080` on the frozen values.  */
#[cfg (not (feature = "have_window_system"))]
const FACE_WS_SHIFT: usize = 0;
#[cfg (feature = "have_window_system")]
const FACE_WS_SHIFT: usize = 16;

const _: () = core::assert! (std::mem::size_of::<face> () == 280 + FACE_WS_SHIFT);
const _: () = core::assert! (std::mem::align_of::<face> () == 8);
const _: () = core::assert! (std::mem::offset_of! (face, id) == 160);
#[cfg (all (feature = "have_window_system"))]
const _: () = core::assert! (std::mem::offset_of! (face, gc) == 168);
#[cfg (all (feature = "have_window_system"))]
const _: () = core::assert! (std::mem::offset_of! (face, stipple) == 176);
const _: () = core::assert! (std::mem::offset_of! (face, foreground) == 168 + FACE_WS_SHIFT);
const _: () = core::assert! (std::mem::offset_of! (face, background) == 176 + FACE_WS_SHIFT);
const _: () = core::assert! (std::mem::offset_of! (face, underline_color) == 184 + FACE_WS_SHIFT);
const _: () = core::assert! (std::mem::offset_of! (face, overline_color) == 192 + FACE_WS_SHIFT);
const _: () = core::assert! (std::mem::offset_of! (face, strike_through_color) == 200 + FACE_WS_SHIFT);
const _: () = core::assert! (std::mem::offset_of! (face, box_color) == 208 + FACE_WS_SHIFT);
const _: () = core::assert! (std::mem::offset_of! (face, font) == 216 + FACE_WS_SHIFT);
const _: () = core::assert! (std::mem::offset_of! (face, fontset) == 224 + FACE_WS_SHIFT);
const _: () = core::assert! (std::mem::offset_of! (face, bits) == 240 + FACE_WS_SHIFT);
const _: () = core::assert! (std::mem::offset_of! (face, hash) == 248 + FACE_WS_SHIFT);
const _: () = core::assert! (std::mem::offset_of! (face, next) == 256 + FACE_WS_SHIFT);
const _: () = core::assert! (std::mem::offset_of! (face, ascii_face) == 272 + FACE_WS_SHIFT);

/* `struct face_cache` (C dispextern.h 1991-2009) - the use-site copy the display units carry, for the
   reason the mirror's `dispextern_face_records` ruling records (status 3.220): the mirror
   keeps the record behind a `dead-arms` feature that no configuration produces, so on
   Android the name is out of scope (`src/xfaces.rs` 288 is the oldest copy).  The text is
   the mirror's; the assertions hold for both targets, and the Android arm's numbers are
   measured in docs/baseline/face-layout-android.tsv.  */
/// C 1991-2009: `struct face_cache`.  Measured: `sizeof` 40, `buckets`@0, `f`@8,
/// `faces_by_id`@16, `size`@24, `used`@32, and C's one `bool_bf menu_face_changed_p : 1` in
/// the int unit at 36 (carried as a byte, the P-80 convention - `src/window.rs` 907 spells
/// the same member `bool`, one byte wide either way).
#[repr(C)]
pub struct face_cache
{
  pub buckets: *mut *mut face,
  /* P-72 rule ③: `struct frame` is frame.rs's type and a unit that includes this mirror need
     not include that one (measured: `build-tu src/bidi.c` and `src/lread.c` answered "cannot
     find type frame" while this member was `*mut frame`), so the member is the untyped
     pointer - the same pointer, which is what C's incomplete `struct frame *` is.  */
  pub f: *mut c_void,   /// C: `struct frame *f;` (dispextern.h 1997)
  pub faces_by_id: *mut *mut face,
  pub size: ptrdiff_t,
  pub used: c_int,
  pub menu_face_changed_p: u8,
}
const _: () = core::assert! (std::mem::size_of::<face_cache> () == 40);
const _: () = core::assert! (std::mem::offset_of! (face_cache, buckets) == 0);
const _: () = core::assert! (std::mem::offset_of! (face_cache, f) == 8);
const _: () = core::assert! (std::mem::offset_of! (face_cache, faces_by_id) == 16);
const _: () = core::assert! (std::mem::offset_of! (face_cache, size) == 24);
const _: () = core::assert! (std::mem::offset_of! (face_cache, used) == 32);

#[cfg (all (feature = "have_window_system", all (feature = "have_android", feature = "not_android_stubify")))]
pub type image_fd = android_fd_or_asset;
#[cfg (all (feature = "have_window_system", not (all (feature = "have_android", feature = "not_android_stubify"))))]
pub type image_fd = c_int;

/* C 749-752: the three `static` prototypes of functions this file defines outside the first
   slice - `slurp_file` (C 4529), `image_find_image_fd` (C 4457) and `xbm_read_bitmap_data`
   (C 5080).  The first slice calls them from its platform arms, so the declarations were kept
   here (C's own signature, in a `/// C:` line) until their slices land.  The fourth slice's
   C 4457 and C 4529 **are** the definitions of the first two and the fifth slice's C 5080 is
   `xbm_read_bitmap_data`'s, so all three prototypes are dropped (PORTING.md 0.1-12) and the
   definitions keep C's place.  */

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

/* C 1195-1198: two `static` prototypes whose definitions the fourth slice lands (C 4086-4089 and
   C 4246-4247), so the prototypes are dropped here (PORTING.md 0.1-12: a prototype and its
   definition in one translation unit have no Rust counterpart).  */

/* C 1200-1211: the X-image accessor pair.  Under `HAVE_NTGUI` they are functions (C 1201-1204,
   defined outside this slice); everywhere else C 1206-1210 declares `image_get_x_image`/
   `image_unget_x_image` and defines the two `_or_dc` wrappers as macros, which the port carries as
   `macro_rules!` (the `GET_PIXEL`/`PUT_PIXEL` rule of this file's own C 67-222 section).  */
/* C 1201-1204 and C 1206-1208: the two pairs of prototypes; the fourth slice lands all four
   definitions (C 4367-4369 / C 4380-4382 under `HAVE_NTGUI`, C 4391-4392 / C 4423-4424 under its
   `#else`), so the prototypes are dropped (PORTING.md 0.1-12) and C 1209-1210's two `#define`s
   below stay.  */
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
   prototypes, C's own comment, and the nested `#ifdef HAVE_X_WINDOWS` pair.  C 1218's
   `image_sync_to_pixmaps` prototype is dropped (its definition is the fourth slice's C 4344).  */
/* We are working on X-specific data structures here even with cairo.
   So we use X-specific versions of image construction/destruction
   functions and inline the specific case of four_corners_best.  */

/* C 1225-1226: `static bool x_create_x_image_and_pixmap (…)` / `static void x_destroy_x_image
   (XImage *);` - the two prototypes are dropped (their definitions are the fourth slice's
   C 3935-3937 and C 3995-3996).  */

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
// Three of the four are declared by `image_laplace`/`image_emboss`/`image_build_heuristic_mask`'s
// definitions, which are C 7651/7662/7860's - inside the "Algorithms" slice, so those three rows
// are **gone** (PORTING.md 0.1-12), and now so is `lookup_image_type`'s: its definition is
// C 13483-13503's and the seventh slice landed it (P-91 ④).

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

/* C: dispextern.h 3757: `ptrdiff_t lookup_image (struct frame *, Lisp_Object, int);` - the
   declaration the third slice needed because the definition was outside it.  The fourth slice's
   C 3556 **is** the definition, so the declaration is gone from here (PORTING.md 0.1-12: a
   prototype and its definition in one translation unit have no Rust counterpart) and the unit's
   `lookup_image` carries `#[unsafe(no_mangle)]`, which is what the `--check` row asked for.  */
// C: frame.h 1474-1475: `extern WINDOW_SYSTEM_RETURN struct frame * decode_window_system_frame (Lisp_Object);`
// (`WINDOW_SYSTEM_RETURN` is `_Noreturn ATTRIBUTE_COLD`, which is not part of the type - the
// `DEFUN!` `noreturn` note in PORTING.md 5.1).
#[cfg (feature = "have_window_system")]
unsafe extern "C" {
  pub fn decode_window_system_frame (frame: Lisp_Object) -> *mut frame;
}
// C: frame.h 1476: `extern struct frame *decode_live_frame (Lisp_Object);` - C 13346's
// `image-transforms-p` is this file's first reader (the ledger's use-site list already names it,
// P-55/P-72 rule 3).
#[cfg (feature = "have_window_system")]
unsafe extern "C" {
  pub fn decode_live_frame (frame: Lisp_Object) -> *mut frame;
}
// C: lisp.h 5135: `extern Lisp_Object decode_env_path (const char *, const char *, bool);` -
// C 13796's `x-bitmap-file-path` initialiser is this file's first reader (the `callproc.rs` 110
// shape).
#[cfg (feature = "have_window_system")]
unsafe extern "C" {
  pub fn decode_env_path (envvar: *const c_char, deflt: *const c_char,
			  empty_is_default: bool) -> Lisp_Object;
}
// C: C 8360: `static bool image_can_use_native_api (Lisp_Object type);` - its definition is in
// C 8358-8449, the `HAVE_NATIVE_IMAGE_API` region this unit's ledger records rather than
// transcribes, so the name stands at the four call sites the arms below reach it from and those
// arms are text-only until that region lands (the `clear_current_matrices` shape above; the
// Windows/GNUstep configurations are P-86 ③'s unmeasurable class).
#[cfg (all (feature = "have_window_system",
	    any (feature = "have_native_image_api",
		 all (feature = "have_ns", not (feature = "ns_impl_cocoa")))))]
unsafe extern "C" {
  pub fn image_can_use_native_api (r#type: Lisp_Object) -> bool;
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
// C 126-127: `static void canvas_prepare_for_display (struct frame *f, struct image *img);` /
// `static void canvas_free_unused (void);` - the first slice dropped C's prototype pair because
// their definitions were outside it, and the two declarations `prepare_image_for_display` and
// `clear_image_caches` call stood here until the fifth slice's C 5742 / C 5535 landed them
// (PORTING.md 0.1-12).  The file's canvas-section note carries the same pair.
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
/* cairo.h 2990-3000's `cairo_pattern_type_t`, `CAIRO_PATTERN_TYPE_SURFACE` and
   `cairo_pattern_get_type` - the three names C 1879/545's `cairo_pattern_get_type` call needs -
   are `include/gtk.rs`'s now (measured there: `CAIRO_PATTERN_TYPE_SURFACE` **1**,
   `sizeof (cairo_pattern_type_t)` **4**), reached through the edge below.  */

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
      /* `include/frame.rs` 186 degrades the member to the untyped pointer (P-72 rule 3, its own
	 note), so C's `struct image_cache *c = FRAME_IMAGE_CACHE (f);` takes the cast the
	 `IMAGE_FROM_ID!` above already carries - without it every window-system configuration reads
	 `E0308 expected *mut image_cache, found *mut c_void` at this line.  */
      let c: *mut image_cache = FRAME_IMAGE_CACHE! (f) as *mut image_cache;

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
unsafe extern "C" fn image_clear_image (f: *mut frame, img: *mut image)
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


/* ----------------------------------------------------------------------------------------------
   C 2212-3550: the file's third slice.

   It holds the "Image Cache" banner (C 2214-2218), C 2219's `static void cache_image (…)`
   prototype (dropped: PORTING.md 0.1-12; its definition is C 3694's), `make_image_cache`
   (C 2225-2237), `search_image_cache` (C 2241-2279), `filter_image_spec` (C 2284-2320),
   `uncache_image` (C 2324-2343), `free_image_cache` (C 2347-2368), `clear_image_cache`
   (C 2374-2453), `clear_image_caches` (C 2455-2465), `DEFUN ("clear-image-cache")` (C 2467-2506),
   `image_size_in_bytes` (C 2508-2554), `image_frame_cache_size` (C 2556-2569), `DEFUN
   ("image-flush")` (C 2571-2604), `postprocess_image` (C 2606-2670), the `HAVE_IMAGEMAGICK ||
   HAVE_NATIVE_TRANSFORMS` block's `scale_image_size` (C 2676-2698) and `image_get_dimension`
   (C 2699-2720), `image_compute_scale` (C 2727-2776), the same gate's `compute_image_size`
   (C 2784-2851), the affine-transform banner and `typedef double matrix3x3[3][3]` (C 2853-2931),
   `matrix3x3_mult` (C 2933-2946), `compute_image_rotation` (C 2948-2968), the `HAVE_ANDROID`
   matrix helpers (C 2970-3056) and `image_set_transform` (C 3058-3548).  C 3550 closes the gate;
   **the next live definition is `lookup_image` (C 3556)** - the next slice's first item.

   The per-`#if` verdicts, the C line-range rows and this round's readings are in
   `docs/ports/image.md` (`## C line ranges`, `## Evidence`, `## Deviations`).  Names the mirrors
   this unit includes do not carry stand at the use site with C's own prototype in a `/// C:` line
   (PORTING.md P-55/P-72 rule 3); names the platform SDKs supply (`<X11/Xlib.h>`, `<windows.h>`,
   Xrender, cairo, the NDK) keep C's tokens.
   ---------------------------------------------------------------------------------------------- */

/***********************************************************************
			     Image Cache
 ***********************************************************************/

/* C 2219: `static void cache_image (struct frame *f, struct image *img);` - a `static` prototype
   whose definition is C 3694's, outside this slice.  C's prototype-plus-definition pair in one
   translation unit has no Rust counterpart (PORTING.md 0.1-12), and Rust's items are
   order-independent, so the declaration is dropped and the definition keeps C's place.  */

/* Return a new, initialized image cache that is allocated from the
   heap.  Call free_image_cache to free an image cache.  */

/// C: C 2223-2225: `struct image_cache * make_image_cache (void)` - non-`static`.
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn make_image_cache () -> *mut image_cache
{
  let c: *mut image_cache = xmalloc (core::mem::size_of::<image_cache> ()) as *mut image_cache;

  (*c).size = 50;
  /* C's `c->used = c->refcount = 0;` - the two stores.  */
  (*c).used = 0;
  (*c).refcount = 0;
  (*c).images = xmalloc ((((*c).size) as usize) * core::mem::size_of::<*mut image> ())
		  as *mut *mut image;
  (*c).buckets = xzalloc (((IMAGE_CACHE_BUCKETS_SIZE) as usize)
			  * core::mem::size_of::<*mut image> ()) as *mut *mut image;
  /* This value should never be encountered.  */
  (*c).scaling_col_width = -1;
  return c;
}

/* Find an image matching SPEC in the cache, and return it.  If no
   image is found, return NULL.  */

/// C: C 2239-2244: `static struct image * search_image_cache (struct frame *f, Lisp_Object spec, EMACS_UINT hash, unsigned long foreground, unsigned long background, int font_size, char *font_family, bool ignore_colors)`
#[cfg (feature = "have_window_system")]
unsafe fn search_image_cache (f: *mut frame, spec: Lisp_Object, hash: EMACS_UINT,
			      foreground: c_ulong, background: c_ulong, font_size: c_int,
			      font_family: *mut c_char, ignore_colors: bool) -> *mut image
{
  let mut img: *mut image;
  let c: *mut image_cache = FRAME_IMAGE_CACHE! (f) as *mut image_cache;
  let i: c_int = (hash % (IMAGE_CACHE_BUCKETS_SIZE as EMACS_UINT)) as c_int;

  if c.is_null () { return core::ptr::null_mut (); }

  /* If the image spec does not specify a background color, the cached
     image must have the same background color as the current frame.
     The foreground color must also match, for the sake of monochrome
     images.

     In fact, we could ignore the foreground color matching condition
     for color images, or if the image spec specifies :foreground;
     similarly we could ignore the background color matching condition
     for formats that don't use transparency (such as jpeg), or if the
     image spec specifies :background.  However, the extra memory
     usage is probably negligible in practice, so we don't bother.  */

  let scale: c_double = image_compute_scale (f, spec, core::ptr::null_mut ());

  /* C's `for (img = c->buckets[i]; img; img = img->next) …` - the `while` with the walk last
     (`x_create_bitmap_mask`'s shape).  */
  img = *(*c).buckets.offset (i as isize);
  while !img.is_null ()
    {
      if (*img).hash == hash
	 && !NILP (Fequal ((*img).spec, spec))
	 && scale == (*img).scale
	 && (ignore_colors || ((*img).face_foreground == foreground
			       && (*img).face_background == background
			       && (*img).face_font_size == font_size
			       && (!font_family.is_null ()
				   && strcmp (font_family, (*img).face_font_family) == 0)))
	{ break; }
      img = (*img).next;
    }
  return img;
}


/* Filter out image elements that don't affect display, but will
   disrupt finding the image in the cache.  This should perhaps be
   user-configurable, but for now it's hard-coded (but new elements
   can be added at will).  */

/// C: C 2281-2284: `static Lisp_Object filter_image_spec (Lisp_Object spec)`
#[cfg (feature = "have_window_system")]
unsafe fn filter_image_spec (spec: Lisp_Object) -> Lisp_Object
{
  let mut out: Lisp_Object = Qnil ();

  let is_canvas: bool = EQ (image_spec_value (spec, QCtype (), core::ptr::null_mut ()),
			    Qcanvas ());

  /* Skip past the `image' element.  */
  let mut spec: Lisp_Object = spec;
  if CONSP (spec)
    { spec = XCDR (spec); }

  while CONSP (spec)
    {
      let key: Lisp_Object = XCAR (spec);
      spec = XCDR (spec);
      if CONSP (spec)
	{
	  let value: Lisp_Object = XCAR (spec);
	  spec = XCDR (spec);

	  /* Some animation-related data doesn't affect display, but
	     breaks the image cache.  Furthermore for canvases do not
	     check the data for equality, such that data can be changed
	     via mutation to refresh the canvas.  Filter these keys out.
	  */
	  if !(if is_canvas
	       { EQ (key, QCdata ()) }
	       else
		 {
		   EQ (key, QCanimate_buffer ())
		     || EQ (key, QCanimate_tardiness ())
		     || EQ (key, QCanimate_position ())
		 })
	    {
	      out = Fcons (value, out);
	      out = Fcons (key, out);
	    }
	}
    }
  return out;
}

/* Search frame F for an image with spec SPEC, and free it.  */

/// C: C 2322-2324: `static void uncache_image (struct frame *f, Lisp_Object spec)`
#[cfg (feature = "have_window_system")]
unsafe fn uncache_image (f: *mut frame, spec: Lisp_Object)
{
  let mut img: *mut image;
  let hash: EMACS_UINT = sxhash (filter_image_spec (spec));

  /* Because the background colors are based on the current face, we
     can have multiple copies of an image with the same spec. We want
     to remove them all to ensure the user doesn't see an old version
     of the image when the face changes.  */
  /* C's `while ((img = search_image_cache (…)))` - the block-in-condition form (`alloc.rs` 794's
     shape).  */
  while {
    img = search_image_cache (f, spec, hash, 0, 0, 0, core::ptr::null_mut (), true);
    !img.is_null ()
  }
    {
      free_image (f, img);
      /* As display glyphs may still be referring to the image ID, we
	 must garbage the frame (Bug#6426).  */
      SET_FRAME_GARBAGED! (f);
    }
}


/* Free image cache of frame F.  Be aware that X frames share images
   caches.  */

/// C: C 2345-2347: `void free_image_cache (struct frame *f)` - non-`static`.
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_image_cache (f: *mut frame)
{
  let c: *mut image_cache = FRAME_IMAGE_CACHE! (f) as *mut image_cache;
  let mut i: ptrdiff_t;

  /* This function assumes the caller already verified that the frame's
     image cache is non-NULL.  */
  eassert! (!c.is_null ());
  /* Cache should not be referenced by any frame when freed.  */
  eassert! ((*c).refcount == 0);

  i = 0;
  while i < (*c).used
    {
      free_image (f, *(*c).images.offset (i));
      i += 1;
    }
  xfree ((*c).images as *mut c_void);
  xfree ((*c).buckets as *mut c_void);
  xfree (c as *mut c_void);
}

/* Clear image cache of frame F.  FILTER=t means free all images.
   FILTER=nil means clear only images that haven't been
   displayed for some time.
   Else, only free the images which have FILTER in their `dependencies'.
   Should be called from time to time to reduce the number of loaded images.
   If image-cache-eviction-delay is non-nil, this frees images in the cache
   which weren't displayed for at least that many seconds.  */

/// C: C 2372-2374: `void clear_image_cache (struct frame *f, Lisp_Object filter)` - non-`static`.
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn clear_image_cache (f: *mut frame, filter: Lisp_Object)
{
  let c: *mut image_cache = FRAME_IMAGE_CACHE! (f) as *mut image_cache;

  /* C's `f->inhibit_clear_image_cache` is one of `struct frame`'s bit members, which the mirror
     carries as the `bits2` storage unit plus its shift (frame.rs 718-723; PORTING.md 1.3/P-80).  */
  if !c.is_null () && !((*f).bits2 >> FRAME_INHIBIT_CLEAR_IMAGE_CACHE_SHIFT & 1 != 0)
    {
      let mut i: ptrdiff_t;
      let mut nfreed: ptrdiff_t = 0;

      /* Block input so that we won't be interrupted by a SIGIO
	 while being in an inconsistent state.  */
      block_input ();

      if !NILP (filter)
	{
	  /* Filter image cache.  */
	  i = 0;
	  while i < (*c).used
	    {
	      let img: *mut image = *(*c).images.offset (i);
	      if !img.is_null ()
		 && (EQ (Qt (), filter) || !NILP (Fmember (filter, (*img).dependencies)))
		{
		  free_image (f, img);
		  nfreed += 1;
		}
	      i += 1;
	    }
	}
      else if FIXNUMP (Vimage_cache_eviction_delay)
	{
	  /* Free cache based on timestamp.  */
	  let mut old: timespec;
	  let t: timespec;
	  let mut delay: c_double;
	  let mut nimages: ptrdiff_t = 0;

	  i = 0;
	  while i < (*c).used
	    {
	      if !(*(*c).images.offset (i)).is_null ()
		{ nimages += 1; }
	      i += 1;
	    }

	  /* If the number of cached images has grown unusually large,
	     decrease the cache eviction delay (Bug#6230).  */
	  delay = XFIXNUM (Vimage_cache_eviction_delay) as c_double;
	  if nimages > 40
	    { delay = 1600 as c_double * delay / nimages as c_double / nimages as c_double; }
	  /* C's `max (delay, 1)` is the ternary (`f64` is not `Ord`).  */
	  delay = if delay > 1 as c_double { delay } else { 1 as c_double };

	  t = current_timespec ();
	  old = timespec_sub (t, dtotimespec (delay));

	  i = 0;
	  while i < (*c).used
	    {
	      let img: *mut image = *(*c).images.offset (i);
	      if !img.is_null () && timespec_cmp ((*img).timestamp, old) < 0
		{
		  free_image (f, img);
		  nfreed += 1;
		}
	      i += 1;
	    }
	}

      /* We may be clearing the image cache because, for example,
	 Emacs was iconified for a longer period of time.  In that
	 case, current matrices may still contain references to
	 images freed above.  So, clear these matrices.  */
      if nfreed != 0
	{
	  let mut tail: Lisp_Object;
	  let mut frame: Lisp_Object;

	  FOR_EACH_FRAME! (tail, frame, {
	    let fr: *mut frame = XFRAME (frame);
	    if FRAME_IMAGE_CACHE! (fr) as *mut image_cache == c
	      { clear_current_matrices (fr); }
	  });

	  windows_or_buffers_changed = 19;
	}

      unblock_input ();
    }
}

/// C: C 2455-2456: `void clear_image_caches (Lisp_Object filter)` - non-`static`.
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn clear_image_caches (filter: Lisp_Object)
{
  /* FIXME: We want to do
   * struct terminal *t;
   * for (t = terminal_list; t; t = t->next_terminal)
   *   clear_image_cache (t, filter); */
  let mut tail: Lisp_Object;
  let mut frame: Lisp_Object;
  FOR_EACH_FRAME! (tail, frame, {
    if FRAME_WINDOW_P (XFRAME (frame))
      { clear_image_cache (XFRAME (frame), filter); }
  });
}

#[cfg (feature = "have_window_system")]
DEFUN! ("clear-image-cache", Fclear_image_cache, Sclear_image_cache, 0, 2, 0,
       doc: "Clear the image and animation caches.\nFILTER nil or a frame means clear all images in the selected frame.\nFILTER t means clear the image caches of all frames.\nAnything else means clear only those images that refer to FILTER,\nwhich is then usually a filename.\n\nThis function also clears the image animation cache.\nANIMATION-FILTER nil means clear all animation cache entries.\nOtherwise, clear the image spec `eq' to ANIMATION-FILTER only\nfrom the animation cache, and do not clear any image caches.\nThis can help reduce memory usage after an animation is stopped\nbut the image is still displayed.",
       (filter: Lisp_Object, animation_filter: Lisp_Object),
       {
  if !NILP (animation_filter)
    {
      /* IMAGEP?  */
      CHECK_CONS (animation_filter);
      /* C 2488-2491: `#if defined (HAVE_WEBP) || defined (HAVE_GIF)`.  */
      #[cfg (all (feature = "have_window_system",
		  any (feature = "have_webp", feature = "have_gif")))]
      {
	/* FIXME: Implement the ImageMagick case.  */
	anim_prune_animation_cache (XCDR (animation_filter));
      }
      return Qnil ();
    }

  if !(NILP (filter) || FRAMEP (filter))
    { clear_image_caches (filter); }
  else
    { clear_image_cache (decode_window_system_frame (filter), Qt ()); }

  /* Also clear the animation caches.  */
  image_prune_animation_caches (true);

  canvas_free_unused ();

  return Qnil ();
});

/// C: C 2508-2509: `static intptr_t image_size_in_bytes (struct image *img)`
#[cfg (feature = "have_window_system")]
unsafe fn image_size_in_bytes (img: *mut image) -> intptr_t
{
  let mut size: intptr_t = 0;

  /* C 2513-2552's five arms are one `#if`/`#elif` chain, so each below excludes the ones before
     it (the `image_clear_image_1` spelling).  */
  /* C 2513-2521: `#if defined USE_CAIRO`.  */
  #[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
  {
    /* The mirror's `pixmap`/`mask` are the untyped pointer (dispextern.rs's own P-72 rule 3
       note); this arm's record is `dispextern.rs` 713's `emacs_pix_container`.  */
    let pm: Emacs_Pixmap = (*img).pixmap as Emacs_Pixmap;
    if !pm.is_null ()
      { size += ((*pm).height as ptrdiff_t) * ((*pm).bytes_per_line as ptrdiff_t); }
    let msk: Emacs_Pixmap = (*img).mask as Emacs_Pixmap;
    if !msk.is_null ()
      { size += ((*msk).height as ptrdiff_t) * ((*msk).bytes_per_line as ptrdiff_t); }
  }

  /* C 2523-2531: `#elif defined HAVE_X_WINDOWS || defined HAVE_ANDROID`.  */
  #[cfg (all (feature = "have_window_system", not (feature = "use_cairo"),
	      any (feature = "have_x_windows", feature = "have_android")))]
  {
    /* Use a nominal depth of 24 and a bpp of 32 for pixmap and 1 bpp
       for mask, to avoid having to query the server. */
    if (*img).pixmap as Emacs_Pixmap != NO_PIXMAP! () as Emacs_Pixmap
      { size += ((*img).width as ptrdiff_t) * ((*img).height as ptrdiff_t) * 4; }
    if (*img).mask as Emacs_Pixmap != NO_PIXMAP! () as Emacs_Pixmap
      { size += ((*img).width as ptrdiff_t) * ((*img).height as ptrdiff_t) / 8; }

    /* C 2528-2531's `img->ximg->data` family.  The mirror's `ximg`/`mask_img` are the untyped
       pointer, so the reads name the record C's member type is: the X arm's `XImage` (Xlib's,
       no mirror - the first slice's Xlib-name class) or C 201's own `typedef struct android_image
       XImage` (`have_android`'s alias above), which is why one spelling serves both arms.  */
    if !(*img).ximg.is_null () && !(*((*img).ximg as *mut XImage)).data.is_null ()
      {
	size += ((*((*img).ximg as *mut XImage)).bytes_per_line as ptrdiff_t)
		* ((*((*img).ximg as *mut XImage)).height as ptrdiff_t);
      }
    if !(*img).mask_img.is_null () && !(*((*img).mask_img as *mut XImage)).data.is_null ()
      {
	size += ((*((*img).mask_img as *mut XImage)).bytes_per_line as ptrdiff_t)
		* ((*((*img).mask_img as *mut XImage)).height as ptrdiff_t);
      }
  }

  /* C 2533-2536: `#elif defined HAVE_NS`.  */
  #[cfg (all (feature = "have_window_system", not (feature = "use_cairo"),
	      not (feature = "have_x_windows"), not (feature = "have_android"),
	      feature = "have_ns"))]
  {
    if !(*img).pixmap.is_null ()
      { size += ns_image_size_in_bytes ((*img).pixmap) as intptr_t; }
    if !(*img).mask.is_null ()
      { size += ns_image_size_in_bytes ((*img).mask) as intptr_t; }
  }

  /* C 2538-2541: `#elif defined HAVE_NTGUI`.  */
  #[cfg (all (feature = "have_window_system", not (feature = "use_cairo"),
	      not (feature = "have_x_windows"), not (feature = "have_android"),
	      not (feature = "have_ns"), feature = "have_ntgui"))]
  {
    if !(*img).pixmap.is_null ()
      { size += w32_image_size ((*img).pixmap) as intptr_t; }
    if !(*img).mask.is_null ()
      { size += w32_image_size ((*img).mask) as intptr_t; }
  }

  /* C 2543-2547: `#elif defined HAVE_HAIKU`.  */
  #[cfg (all (feature = "have_window_system", not (feature = "use_cairo"),
	      not (feature = "have_x_windows"), not (feature = "have_android"),
	      not (feature = "have_ns"), not (feature = "have_ntgui"), feature = "have_haiku"))]
  {
    if !(*img).pixmap.is_null ()
      { size += BBitmap_bytes_length ((*img).pixmap) as intptr_t; }
    if !(*img).mask.is_null ()
      { size += BBitmap_bytes_length ((*img).mask) as intptr_t; }
  }

  return size;
}

/// C: C 2556-2557: `static intptr_t image_frame_cache_size (struct frame *f)`
#[cfg (feature = "have_window_system")]
unsafe fn image_frame_cache_size (f: *mut frame) -> intptr_t
{
  let c: *mut image_cache = FRAME_IMAGE_CACHE! (f) as *mut image_cache;
  if c.is_null ()
    { return 0; }

  let mut total: intptr_t = 0;
  let mut i: ptrdiff_t = 0;
  while i < (*c).used
    {
      let img: *mut image = *(*c).images.offset (i);
      total += if !img.is_null () { image_size_in_bytes (img) } else { 0 };
      i += 1;
    }
  return total;
}

#[cfg (feature = "have_window_system")]
DEFUN! ("image-flush", Fimage_flush, Simage_flush, 1, 2, 0,
       doc: "Flush the image with specification SPEC on frame FRAME.\nThis removes the image from the Emacs image cache.  If SPEC specifies\nan image file, the next redisplay of this image will read from the\ncurrent contents of that file.\n\nFRAME nil or omitted means use the selected frame.\nFRAME t means refresh the image on all frames.",
       (spec: Lisp_Object, frame: Lisp_Object),
       {
  if !valid_image_p (spec)
    { error (c"Invalid image specification".as_ptr ()); }

  if EQ (frame, Qt ())
    {
      let mut tail: Lisp_Object;
      /* C reuses the `frame` parameter as the loop's variable; a `DEFUN!` parameter is immutable
	 in Rust, so the port shadows it with C's own name (the `terminal.rs` 589 shape, PORTING.md
	 5.1's note).  */
      let mut frame: Lisp_Object = frame;
      FOR_EACH_FRAME! (tail, frame, {
	let f: *mut frame = XFRAME (frame);
	if FRAME_WINDOW_P (f)
	  { uncache_image (f, spec); }
      });
    }
  else
    { uncache_image (decode_window_system_frame (frame), spec); }

  return Qnil ();
});


/* Compute masks and transform image IMG on frame F, as specified
   by the image's specification,  */

/// C: C 2604-2606: `static void postprocess_image (struct frame *f, struct image *img)`
#[cfg (feature = "have_window_system")]
unsafe fn postprocess_image (f: *mut frame, img: *mut image)
{
  /* Manipulation of the image's mask.  */
  if !(*img).pixmap.is_null ()
    {
      let mut conversion: Lisp_Object;
      let mut spec: Lisp_Object;
      let mut mask: Lisp_Object;

      spec = (*img).spec;

      /* `:heuristic-mask t'
	 `:mask heuristic'
	 means build a mask heuristically.
	 `:heuristic-mask (R G B)'
	 `:mask (heuristic (R G B))'
	 means build a mask from color (R G B) in the
	 image.
	 `:mask nil'
	 means remove a mask, if any.  */

      mask = image_spec_value (spec, QCheuristic_mask (), core::ptr::null_mut ());
      if !NILP (mask)
	{ image_build_heuristic_mask (f, img, mask); }
      else
	{
	  /* C's `bool found_p;` - zeroed (PORTING.md 1.5/0.1-13): `&raw mut` of a binding Rust
	     has not initialised is `E0381`.  */
	  let mut found_p: bool = false;

	  mask = image_spec_value (spec, QCmask (), &raw mut found_p);

	  if EQ (mask, Qheuristic ())
	    { image_build_heuristic_mask (f, img, Qt ()); }
	  else if CONSP (mask) && EQ (XCAR (mask), Qheuristic ())
	    {
	      if CONSP (XCDR (mask))
		{ image_build_heuristic_mask (f, img, XCAR (XCDR (mask))); }
	      else
		{ image_build_heuristic_mask (f, img, XCDR (mask)); }
	    }
	  else if NILP (mask) && found_p && !(*img).mask.is_null ()
	    { image_clear_image_1 (f, img, CLEAR_IMAGE_MASK! ()); }
	}


      /* Should we apply an image transformation algorithm?  */
      conversion = image_spec_value (spec, QCconversion (), core::ptr::null_mut ());
      if EQ (conversion, Qdisabled ())
	{ image_disable_image (f, img); }
      else if EQ (conversion, Qlaplace ())
	{ image_laplace (f, img); }
      else if EQ (conversion, Qemboss ())
	{ image_emboss (f, img); }
      else if CONSP (conversion) && EQ (XCAR (conversion), Qedge_detection ())
	{
	  let mut tem: Lisp_Object;
	  tem = XCDR (conversion);
	  if CONSP (tem)
	    { image_edge_detection (f, img, plist_get (tem, QCmatrix ()),
				    plist_get (tem, QCcolor_adjustment ())); }
	}
    }
}

/* ----------------------------------------------------------------------------------------------
   The names the mirrors this unit includes do not carry, declared at the use site with C's own
   line (PORTING.md P-55/P-72 rule 3).  This slice's `postprocess_image` (C 2606) and
   `Fclear_image_cache` (C 2467) are the call sites of `image_disable_image`/`image_edge_detection`
   (C 224-226's prototypes), whose definitions are C 7765/7680's - inside the "Algorithms" slice -
   so those two rows are **gone** (PORTING.md 0.1-12; the `canvas_prepare_for_display` shape of the
   second slice, retired the way it says).
   ---------------------------------------------------------------------------------------------- */
/* C: C 244-246: `#if defined (HAVE_WEBP) || defined (HAVE_GIF)` / `static void
   anim_prune_animation_cache (Lisp_Object);` / `#endif` - the declaration the third slice kept
   because the definition was outside it.  The fourth slice's C 3812 **is** the definition, so the
   declaration is gone (PORTING.md 0.1-12).  */
// C: lisp.h 4250: `EMACS_UINT sxhash (Lisp_Object);` - `include/lisp.rs` does not carry it (a
// mirror gap; it belongs in that mirror).
#[cfg (feature = "have_window_system")]
unsafe extern "C" {
  pub fn sxhash (obj: Lisp_Object) -> EMACS_UINT;
}
// C: lisp.h 4280: `extern Lisp_Object plist_get (Lisp_Object plist, Lisp_Object prop);` - the same
// mirror gap.
#[cfg (feature = "have_window_system")]
unsafe extern "C" {
  pub fn plist_get (plist: Lisp_Object, prop: Lisp_Object) -> Lisp_Object;
}
// C: dispextern.h 3754: `void image_prune_animation_caches (bool);` - the definition is
// `image.c`'s own (C 13504-13515) and the seventh slice landed it, so this declaration is gone
// (PORTING.md 0.1-12 / P-91 ④).
// C: dispextern.h 3907: `void clear_current_matrices (struct frame *f);` - the definition is
// `xdisp.c`'s.
#[cfg (feature = "have_window_system")]
unsafe extern "C" {
  pub fn clear_current_matrices (f: *mut frame);
}
/// C: math.h 1172-1173: `extern float sinf (float __x) __THROW;` / `extern float cosf (float __x) __THROW;`
/// `lib/include/math.rs` carries the `double` family only; this file's `HAVE_ANDROID` arm is their
/// first reader (`matrix_rotate`), and both belong in that mirror.  C's `float` is the port's
/// `f32` - `c_float` is not in this toolchain's prelude, and `android_transform`'s members
/// (`include/androidgui.rs` 900-908) are the tree's spelling of C's `float` already.
#[cfg (all (feature = "have_window_system", feature = "have_android"))]
unsafe extern "C" {
  pub fn sinf (x: f32) -> f32;
  pub fn cosf (x: f32) -> f32;
}
/// C: float.h 85: `# define DBL_MAX __DBL_MAX__` (the frozen target's clang reports
/// `1.79769313486231570815e+308`).  `lib/include/float.in.rs` mirrors the seven `float.h` names
/// the reference read before this file, and `DBL_MAX` is not one of them; this slice's
/// `image_set_transform` is its first reader.  It belongs in that mirror (the `sinf` row above).
#[cfg (all (feature = "have_window_system",
	    any (feature = "have_imagemagick", feature = "have_native_transforms")))]
pub const DBL_MAX: c_double = 1.7976931348623157e+308;

/* C 2671: `#if defined (HAVE_IMAGEMAGICK) || defined (HAVE_NATIVE_TRANSFORMS)` - the two
   `scale_image_size`/`image_get_dimension` items below and C 2777's second gate's items close it
   at C 2721.  */
/* Scale an image size by returning SIZE / DIVISOR * MULTIPLIER,
   safely rounded and clipped to int range.  */

/// C: C 2674-2676: `static int scale_image_size (int size, double divisor, double multiplier)`
#[cfg (all (feature = "have_window_system",
	    any (feature = "have_imagemagick", feature = "have_native_transforms")))]
unsafe fn scale_image_size (size: c_int, divisor: c_double, multiplier: c_double) -> c_int
{
  if divisor != 0 as c_double
    {
      let scaled: c_double = size as c_double * multiplier / divisor;
      if scaled < INT_MAX as c_double
	{
	  /* Use ceil, as rounding can discard fractional SVG pixels.  */
	  /* C's implicit `double` -> `int` conversion.  */
	  return ceil (scaled) as c_int;
	}
    }
  return INT_MAX;
}

/* Return a size, in pixels, from the value specified by SYMBOL, which
   may be an integer or a pair of the form (VALUE . 'em) where VALUE
   is a float that is multiplied by the font size to get the final
   dimension.

   If the value doesn't exist in the image spec, or is invalid, return
   -1.
*/
/// C: C 2697-2699: `static int image_get_dimension (struct image *img, Lisp_Object symbol)`
#[cfg (all (feature = "have_window_system",
	    any (feature = "have_imagemagick", feature = "have_native_transforms")))]
unsafe fn image_get_dimension (img: *mut image, symbol: Lisp_Object) -> c_int
{
  let value: Lisp_Object = image_spec_value ((*img).spec, symbol, core::ptr::null_mut ());

  if FIXNATP (value)
    { return std::cmp::min (XFIXNAT (value), INT_MAX as EMACS_INT) as c_int; }
  if CONSP (value) && NUMBERP (CAR (value))
    {
      let dim: Lisp_Object = CDR (value);

      if EQ (Qem (), dim)
	{ return scale_image_size ((*img).face_font_size, 1 as c_double, XFLOATINT (CAR (value))); }
      if EQ (Qch (), dim)
	{ return scale_image_size ((*img).face_font_height, 1 as c_double,
				   XFLOATINT (CAR (value))); }
      if EQ (Qcw (), dim)
	{ return scale_image_size ((*img).face_font_width, 1 as c_double,
				   XFLOATINT (CAR (value))); }
    }
  return -1;
}
/* C 2721: `#endif` */

/* Calculate the scale of the image.  IMG may be null as it is only
   required when creating an image, and this function is called from
   image cache related functions that do not have access to the image
   structure.  */

/// C: C 2725-2727: `static double image_compute_scale (struct frame *f, Lisp_Object spec, struct image *img)`
#[cfg (feature = "have_window_system")]
unsafe fn image_compute_scale (f: *mut frame, spec: Lisp_Object, img: *mut image) -> c_double
{
  let mut scale: c_double = 1 as c_double;
  let value: Lisp_Object = image_spec_value (spec, QCscale (), core::ptr::null_mut ());

  if EQ (value, Qdefault ())
    {
      let mut sval: Lisp_Object = Vimage_scaling_factor;

      /* C's `invalid_value:` label.  The label is the head of the block, so C's
	 `goto invalid_value` is this loop's `continue` (the tree's `goto` shape, PORTING.md 2.2;
	 `doprnt`/`push_it` are the precedent).  */
      loop
	{
	  if EQ (sval, Qauto ())
	    {
	      /* This is a tag with which callers of `clear_image_cache' can
		 refer to this image and its likenesses.  */
	      if !img.is_null ()
		{ (*img).dependencies = Fcons (Qauto (), (*img).dependencies); }

	      /* C's `10.0f` is a `float` literal, so the division is C's own `float` one.  */
	      scale = if FRAME_COLUMN_WIDTH (f) > 10
			{ (FRAME_COLUMN_WIDTH (f) as f32 / 10.0f32) as c_double }
		      else
			{ 1 as c_double };
	      break;
	    }
	  else if NUMBERP (sval)
	    {
	      scale = XFLOATINT (sval);
	      break;
	    }
	  else
	    {
	      image_error (c"Invalid `image-scaling-factor': %s".as_ptr (),
			   Vimage_scaling_factor);

	      /* If Vimage_scaling_factor is set to an invalid value, treat
		 it as though it were the default.  */
	      sval = Qauto ();
	      /* C's `goto invalid_value;` */
	      continue;
	    }
	}
    }
  else if NUMBERP (value)
    {
      let dval: c_double = XFLOATINT (value);
      if 0 as c_double <= dval
	{ scale = dval; }
    }

  if !img.is_null ()
    { (*img).scale = scale; }

  return scale;
}

/* C 2777: `#if defined HAVE_IMAGEMAGICK || defined HAVE_NATIVE_TRANSFORMS` - closed by C 3550
   `#endif /* HAVE_IMAGEMAGICK || HAVE_NATIVE_TRANSFORMS */`, after `image_set_transform`.  */
/* Compute the desired size of an image with native size WIDTH x HEIGHT,
   which is to be displayed on F.  Use IMG to deduce the size.  Store
   the desired size into *D_WIDTH x *D_HEIGHT.  Store -1 x -1 if the
   native size is OK.  */

/// C: C 2782-2785: `static void compute_image_size (struct frame *f, double width, double height, struct image *img, int *d_width, int *d_height)`
#[cfg (all (feature = "have_window_system",
	    any (feature = "have_imagemagick", feature = "have_native_transforms")))]
unsafe fn compute_image_size (f: *mut frame, width: c_double, height: c_double, img: *mut image,
			      d_width: *mut c_int, d_height: *mut c_int)
{
  let scale: c_double = image_compute_scale (f, (*img).spec, img);

  /* If width and/or height is set in the display spec assume we want
     to scale to those values.  If either h or w is unspecified, the
     unspecified should be calculated from the specified to preserve
     aspect ratio.  */
  let mut desired_width: c_int = image_get_dimension (img, QCwidth ());
  let mut max_width: c_int;
  if desired_width < 0
    { max_width = image_get_dimension (img, QCmax_width ()); }
  else
    {
      desired_width = scale_image_size (desired_width, 1 as c_double, scale);
      /* :width overrides :max-width. */
      max_width = -1;
    }

  let mut desired_height: c_int = image_get_dimension (img, QCheight ());
  let mut max_height: c_int;
  if desired_height < 0
    { max_height = image_get_dimension (img, QCmax_height ()); }
  else
    {
      desired_height = scale_image_size (desired_height, 1 as c_double, scale);
      /* :height overrides :max-height. */
      max_height = -1;
    }

  /* If we have both width/height set explicitly, we skip past all the
     aspect ratio-preserving computations below. */
  /* C's `goto out` - the label is the function's tail, so the skip is the flag the tree uses
     (`fileio.rs` 4880's shape).  */
  let mut goto_out: bool = false;
  if 0 <= desired_width && 0 <= desired_height
    { goto_out = true; }

  if !goto_out
    {
      if 0 <= desired_width
	/* Width known, calculate height. */
	{ desired_height = scale_image_size (desired_width, width, height); }
      else if 0 <= desired_height
	/* Height known, calculate width. */
	{ desired_width = scale_image_size (desired_height, height, width); }
      else
	{
	  /* C's `width`/`height` are `double` here and `scale_image_size`'s first parameter is
	     `int`, so this is C's implicit conversion written out.  */
	  desired_width = scale_image_size (width as c_int, 1 as c_double, scale);
	  desired_height = scale_image_size (height as c_int, 1 as c_double, scale);
	}

      if 0 <= max_width && max_width < desired_width
	{
	  /* The image is wider than :max-width. */
	  desired_width = max_width;
	  desired_height = scale_image_size (desired_width, width, height);
	}

      if 0 <= max_height && max_height < desired_height
	{
	  /* The image is higher than :max-height. */
	  desired_height = max_height;
	  desired_width = scale_image_size (desired_height, height, width);
	}
    }

  /* out: */
  *d_width = desired_width;
  *d_height = desired_height;
}

/* image_set_rotation and image_set_transform use affine
   transformation matrices to perform various transforms on the image.
   The matrix is a 2D array of doubles.  It is laid out like this:

   m[0][0] = m11 | m[1][0] = m12 | m[2][0] = tx
   --------------+---------------+-------------
   m[0][1] = m21 | m[1][1] = m22 | m[2][1] = ty
   --------------+---------------+-------------
   m[0][2] = 0   | m[1][2] = 0   | m[2][2] = 1

   tx and ty represent translations, m11 and m22 represent scaling
   transforms and m21 and m12 represent shear transforms.  Most
   graphics toolkits don't require the third row, however it is
   necessary for multiplication.

   Transforms are done by creating a matrix for each action we wish to
   take, then multiplying the transformation matrix by each of those
   matrices in order (matrix multiplication is not commutative).
   After we've done that we can use our modified transformation matrix
   to transform points.  We take the x and y coordinates and convert
   them into a 3x1 matrix and multiply that by the transformation
   matrix and it gives us a new, transformed, set of coordinates:

       [m11 m12 tx]   [x]   [m11*x+m12*y+tx*1]   [x']
       [m21 m22 ty] X [y] = [m21*x+m22*y+ty*1] = [y']
       [  0   0  1]   [1]   [     0*x+0*y+1*1]   [ 1]

   We don't have to worry about the last step as the graphics toolkit
   will do it for us.

   The three transforms we are concerned with are translation, scaling
   and rotation.  The translation matrix looks like this:

       [1 0 tx]
       [0 1 ty]
       [0 0  1]

   Where tx and ty are the amount to translate the origin in the x and
   y coordinates, respectively.  Since we are translating the origin
   and not the image data itself, it can appear backwards in use, for
   example to move the image 10 pixels to the right, you would set tx
   to -10.

   To scale we use:

       [x 0 0]
       [0 y 0]
       [0 0 1]

   Where x and y are the amounts to scale in the x and y dimensions.
   Values smaller than 1 make the image larger, values larger than 1
   make it smaller.  Negative values flip the image.  For example to
   double the image size set x and y to 0.5.

   To rotate we use:

       [ cos(r) sin(r) 0]
       [-sin(r) cos(r) 0]
       [      0      0 1]

   Where r is the angle of rotation required.  Rotation occurs around
   the origin, not the center of the image.  Note that this is
   normally considered a counter-clockwise rotation, however because
   our y axis is reversed, (0, 0) at the top left, it works as a
   clockwise rotation.

   The full process of rotating an image is to move the origin to the
   center of the image (width/2, height/2), perform the rotation, and
   finally move the origin back to the top left of the image, which
   may now be a different corner.

   Note that different GUI backends (X, Cairo, w32, NS, Haiku,
   Android) want the transform matrix defined as transform from the
   original image to the transformed image, while others want the
   matrix to describe the transform of the space, which boils down to
   inverting the matrix.

   It's possible to pre-calculate the matrix multiplications and just
   generate one transform matrix that will do everything we need in a
   single step, but the maths for each element is much more complex
   and performing the steps separately makes for more readable code.  */

/// C: C 2931: `typedef double matrix3x3[3][3];`
#[cfg (all (feature = "have_window_system",
	    any (feature = "have_imagemagick", feature = "have_native_transforms")))]
type matrix3x3 = [[c_double; 3]; 3];

/// C: C 2933-2934: `static void matrix3x3_mult (matrix3x3 a, matrix3x3 b, matrix3x3 result)`
/// C's three parameters are `double (*)[3]` (the array typedef decays), so each is the pointer
/// P-49 spells; the callers pass `…as_mut_ptr ()`.
#[cfg (all (feature = "have_window_system",
	    any (feature = "have_imagemagick", feature = "have_native_transforms")))]
unsafe fn matrix3x3_mult (a: *mut [c_double; 3], b: *mut [c_double; 3],
			  result: *mut [c_double; 3])
{
  let mut i: c_int = 0;
  while i < 3
    {
      let mut j: c_int = 0;
      while j < 3
	{
	  let mut sum: c_double = 0 as c_double;
	  let mut k: c_int = 0;
	  while k < 3
	    {
	      sum += (*(a.offset (i as isize)))[k as usize]
		     * (*(b.offset (k as isize)))[j as usize];
	      k += 1;
	    }
	  (*(result.offset (i as isize)))[j as usize] = sum;
	  j += 1;
	}
      i += 1;
    }
}

/// C: C 2948-2949: `static void compute_image_rotation (struct image *img, double *rotation)`
#[cfg (all (feature = "have_window_system",
	    any (feature = "have_imagemagick", feature = "have_native_transforms")))]
unsafe fn compute_image_rotation (img: *mut image, rotation: *mut c_double)
{
  let mut foundp: bool = false;
  let value: Lisp_Object = image_spec_value ((*img).spec, QCrotation (),
					     &raw mut foundp);
  if !foundp
    { return; }
  if ! NUMBERP (value)
    {
      image_error (c"Invalid image `:rotation' parameter".as_ptr ());
      return;
    }

  let reduced_angle: Lisp_Object = Fmod (value, make_fixnum (360));
  if FLOATP (reduced_angle)
    { *rotation = XFLOAT_DATA (reduced_angle); }
  else
    { *rotation = XFIXNUM (reduced_angle) as c_double; }
}

/* C 2970: `#ifdef HAVE_ANDROID` - the four matrix helpers below, closed by C 3056's `#endif`.  */

/// C: C 2972-2973: `static void matrix_identity (matrix3x3 matrix)`
#[cfg (all (feature = "have_window_system",
	    any (feature = "have_imagemagick", feature = "have_native_transforms"),
	    feature = "have_android"))]
unsafe fn matrix_identity (matrix: *mut [c_double; 3])
{
  memset (matrix as *mut c_void, 0, core::mem::size_of::<matrix3x3> ());

  (*(matrix.offset (0)))[0] = 1.0;
  (*(matrix.offset (1)))[1] = 1.0;
  (*(matrix.offset (2)))[2] = 1.0;
}

/* Translate the matrix TRANSFORM to X, Y, and then perform clockwise
   rotation by the given angle THETA in radians and translate back.
   As the transform is being performed in a coordinate system where Y
   grows downwards, the given angle describes a clockwise
   rotation.  */

/// C: C 2986-2987: `static void matrix_rotate (matrix3x3 transform, double theta, double x, double y)`
#[cfg (all (feature = "have_window_system",
	    any (feature = "have_imagemagick", feature = "have_native_transforms"),
	    feature = "have_android"))]
unsafe fn matrix_rotate (transform: *mut [c_double; 3], theta: c_double, x: c_double, y: c_double)
{
  let mut temp: matrix3x3 = [[0.0; 3]; 3];
  let mut copy: matrix3x3 = [[0.0; 3]; 3];

  /* 1. Translate the matrix so X and Y are in the center.  */

  matrix_identity (temp.as_mut_ptr ());
  memcpy (copy.as_mut_ptr () as *mut c_void, transform as *const c_void,
	  core::mem::size_of::<matrix3x3> ());

  temp[0][2] = x;
  temp[1][2] = y;

  matrix3x3_mult (copy.as_mut_ptr (), temp.as_mut_ptr (), transform);
  matrix_identity (temp.as_mut_ptr ());
  memcpy (copy.as_mut_ptr () as *mut c_void, transform as *const c_void,
	  core::mem::size_of::<matrix3x3> ());

  /* 2. Rotate the matrix counter-clockwise, assuming a coordinate
     system where Y grows downwards.  */

  temp[0][0] = cos (theta);
  temp[0][1] = -sin (theta);
  temp[1][0] = sinf (theta as f32) as c_double;
  temp[1][1] = cosf (theta as f32) as c_double;

  matrix3x3_mult (copy.as_mut_ptr (), temp.as_mut_ptr (), transform);
  matrix_identity (temp.as_mut_ptr ());
  memcpy (copy.as_mut_ptr () as *mut c_void, transform as *const c_void,
	  core::mem::size_of::<matrix3x3> ());

  /* 3. Translate back.  */

  temp[0][2] = -x;
  temp[1][2] = -y;

  matrix3x3_mult (copy.as_mut_ptr (), temp.as_mut_ptr (), transform);
}

/* Scale the matrix TRANSFORM by -1, and then apply a TX of width, in
   effect flipping the image horizontally.  */

/// C: C 3026-3027: `static void matrix_mirror_horizontal (matrix3x3 transform, double width)`
#[cfg (all (feature = "have_window_system",
	    any (feature = "have_imagemagick", feature = "have_native_transforms"),
	    feature = "have_android"))]
unsafe fn matrix_mirror_horizontal (transform: *mut [c_double; 3], width: c_double)
{
  let mut temp: matrix3x3 = [[0.0; 3]; 3];
  let mut copy: matrix3x3 = [[0.0; 3]; 3];

  matrix_identity (temp.as_mut_ptr ());
  memcpy (copy.as_mut_ptr () as *mut c_void, transform as *const c_void,
	  core::mem::size_of::<matrix3x3> ());

  temp[0][0] = (-1.0f32) as c_double;
  temp[0][2] = width;

  matrix3x3_mult (copy.as_mut_ptr (), temp.as_mut_ptr (), transform);
}

/// C: C 3040-3041: `static void matrix_translate (matrix3x3 transform, float tx, float ty)`
#[cfg (all (feature = "have_window_system",
	    any (feature = "have_imagemagick", feature = "have_native_transforms"),
	    feature = "have_android"))]
unsafe fn matrix_translate (transform: *mut [c_double; 3], tx: f32, ty: f32)
{
  let mut temp: matrix3x3 = [[0.0; 3]; 3];
  let mut copy: matrix3x3 = [[0.0; 3]; 3];

  matrix_identity (temp.as_mut_ptr ());
  memcpy (copy.as_mut_ptr () as *mut c_void, transform as *const c_void,
	  core::mem::size_of::<matrix3x3> ());

  /* Set the tx and ty.  */
  temp[0][2] = tx as c_double;
  temp[1][2] = ty as c_double;

  /* Multiply it with the transform.  */
  matrix3x3_mult (copy.as_mut_ptr (), temp.as_mut_ptr (), transform);
}

/* C 3056: `#endif` */

/// C: C 3058-3059: `static void image_set_transform (struct frame *f, struct image *img)`
#[cfg (all (feature = "have_window_system",
	    any (feature = "have_imagemagick", feature = "have_native_transforms")))]
unsafe fn image_set_transform (f: *mut frame, img: *mut image)
{
  let mut flip: bool;

  /* C 3063-3072: `#if defined HAVE_HAIKU`.  */
  #[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
  {
    let identity: matrix3x3 = [[1 as c_double, 0 as c_double, 0 as c_double],
			       [0 as c_double, 1 as c_double, 0 as c_double],
			       [0 as c_double, 0 as c_double, 1 as c_double]];

    (*img).use_bilinear_filtering = false;

    memcpy (&raw mut (*img).transform as *mut c_void, &raw const identity as *const c_void,
	    core::mem::size_of::<matrix3x3> ());
  }

  /* C 3074-3081: `#if defined HAVE_ANDROID`.  */
  #[cfg (all (feature = "have_window_system", feature = "have_android"))]
  let identity: matrix3x3 = [[1 as c_double, 0 as c_double, 0 as c_double],
			     [0 as c_double, 1 as c_double, 0 as c_double],
			     [0 as c_double, 0 as c_double, 1 as c_double]];

  /* C 3082-3087: `# if (defined HAVE_IMAGEMAGICK && !defined DONT_CREATE_TRANSFORMED_IMAGEMAGICK_IMAGE)`.
     `DONT_CREATE_TRANSFORMED_IMAGEMAGICK_IMAGE` is the `have_pgtk` && `have_imagemagick` constant
     above (C 104-111), so `!defined` is `not (have_pgtk && have_imagemagick)`.  */
  #[cfg (all (feature = "have_window_system", feature = "have_imagemagick",
	      not (feature = "have_pgtk")))]
  if EQ (image_spec_value ((*img).spec, QCtype (), core::ptr::null_mut ()), Qimagemagick ())
    { return; }

  /* C 3089-3092: `# if !defined USE_CAIRO && defined HAVE_XRENDER`.  */
  #[cfg (all (feature = "have_window_system", not (feature = "use_cairo"),
	      feature = "have_xrender"))]
  if (*img).picture.is_null ()
    { return; }

  /* Determine size.  */
  /* C's `int width, height;` are written by `compute_image_size`/the SVG arm before any read;
     zeroed here because `&raw mut` of a binding Rust has not initialised is `E0381`
     (PORTING.md 1.5/0.1-13).  */
  let mut width: c_int = 0;
  let mut height: c_int = 0;

  /* C 3095-3103: `#ifdef HAVE_RSVG`, whose `if (… SVG …) {…} else` prefix on the
     `compute_image_size` call below cannot be spelled as a Rust `if`/`else` pair when only one
     arm compiles (the `keyboard.rs` 826 rule): the gated arm is carried here, and the call keeps
     C's place below under C's own `#endif` condition.  */
  #[cfg (all (feature = "have_window_system", feature = "have_rsvg"))]
  if EQ (image_spec_value ((*img).spec, QCtype (), core::ptr::null_mut ()), Qsvg ())
    {
      width = (*img).width / FRAME_SCALE_FACTOR (f);
      height = (*img).height / FRAME_SCALE_FACTOR (f);
    }
  else
    {
      compute_image_size (f, (*img).width as c_double, (*img).height as c_double, img,
			  &raw mut width, &raw mut height);
    }

  #[cfg (all (feature = "have_window_system", not (feature = "have_rsvg")))]
  compute_image_size (f, (*img).width as c_double, (*img).height as c_double, img,
		      &raw mut width, &raw mut height);

  /* Determine rotation.  */
  let mut rotation: c_double = 0.0;
  compute_image_rotation (img, &raw mut rotation);

  /* Determine flipping.  */
  flip = !NILP (image_spec_value ((*img).spec, QCflip (), core::ptr::null_mut ()));

  /* C 3105-3115: `# if defined USE_CAIRO || defined HAVE_XRENDER || defined HAVE_NS || defined
     HAVE_HAIKU || defined HAVE_ANDROID || defined HAVE_NTGUI`.  */
  #[cfg (all (feature = "have_window_system",
	      any (feature = "use_cairo", feature = "have_xrender", feature = "have_ns",
		   feature = "have_haiku", feature = "have_android", feature = "have_ntgui")))]
  let mut smoothing: bool;
  #[cfg (all (feature = "have_window_system",
	      any (feature = "use_cairo", feature = "have_xrender", feature = "have_ns",
		   feature = "have_haiku", feature = "have_android", feature = "have_ntgui")))]
  {
    /* We want scale up operations to use a nearest neighbor filter to
       show real pixels instead of munging them, but scale down
       operations to use a blended filter, to avoid aliasing and the like.  */
    let s: Lisp_Object = image_spec_value ((*img).spec, QCtransform_smoothing (),
					   core::ptr::null_mut ());
    if NILP (s)
      { smoothing = (width < (*img).width) || (height < (*img).height); }
    else
      { smoothing = !NILP (s); }
  }

  /* C 3117-3119: `#ifdef HAVE_HAIKU`.  */
  #[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
  { (*img).use_bilinear_filtering = smoothing; }

  /* C 3120-3122: `#ifdef HAVE_NTGUI`.  */
  #[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
  { (*img).smoothing = smoothing; }

  /* Perform scale transformation.  */

  /* C 3126-3141: C's designated initializer's three arms; the elements it does not name are
     its own zero fill.  */
  #[cfg (all (feature = "have_window_system",
	      any (feature = "use_cairo", feature = "have_xrender", feature = "have_android")))]
  let mut matrix: matrix3x3
    = [[if !IEEE_FLOATING_POINT && width == 0
	 { DBL_MAX }
       else
	 { (*img).width as c_double / width as c_double },
	0 as c_double, 0 as c_double],
       [0 as c_double,
	if !IEEE_FLOATING_POINT && height == 0
	  { DBL_MAX }
	else
	  { (*img).height as c_double / height as c_double },
	0 as c_double],
       [0 as c_double, 0 as c_double, 1 as c_double]];
  #[cfg (all (feature = "have_window_system",
	      any (feature = "have_ntgui", feature = "have_ns", feature = "have_haiku"),
	      not (any (feature = "use_cairo", feature = "have_xrender", feature = "have_android"))))]
  let mut matrix: matrix3x3
    = [[if !IEEE_FLOATING_POINT && (*img).width == 0
	 { DBL_MAX }
       else
	 { width as c_double / (*img).width as c_double },
	0 as c_double, 0 as c_double],
       [0 as c_double,
	if !IEEE_FLOATING_POINT && (*img).height == 0
	  { DBL_MAX }
	else
	  { height as c_double / (*img).height as c_double },
	0 as c_double],
       [0 as c_double, 0 as c_double, 1 as c_double]];
  #[cfg (all (feature = "have_window_system",
	      not (any (feature = "use_cairo", feature = "have_xrender", feature = "have_android",
			feature = "have_ntgui", feature = "have_ns", feature = "have_haiku"))))]
  let mut matrix: matrix3x3 = [[1 as c_double, 0 as c_double, 0 as c_double],
			       [0 as c_double, 1 as c_double, 0 as c_double],
			       [0 as c_double, 0 as c_double, 1 as c_double]];
  (*img).width = width;
  (*img).height = height;

  /* Perform rotation transformation.  */

  let mut rotate_flag: c_int = -1;

  /* Haiku needs this, since the transformation is done on the basis
     of the view, and not the image.  */
  /* C 3149-3154: `#if defined HAVE_HAIKU`'s `int extra_tx, extra_ty; extra_tx = 0; extra_ty = 0;`  */
  #[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
  let mut extra_tx: c_int = 0;
  #[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
  let mut extra_ty: c_int = 0;

  if rotation == 0 as c_double && !flip
    { rotate_flag = 0; }
  else
    {
      /* C 3163-3330: `#ifndef HAVE_ANDROID`, whose width-system arm is C 3165's
	 `# if (defined USE_CAIRO || defined HAVE_XRENDER || defined HAVE_NTGUI || defined HAVE_NS
	 || defined HAVE_HAIKU)`.  */
      #[cfg (all (feature = "have_window_system", not (feature = "have_android"),
		  any (feature = "use_cairo", feature = "have_xrender", feature = "have_ntgui",
		       feature = "have_ns", feature = "have_haiku")))]
      {
	/* C's `int cos_r, sin_r;` - zeroed (PORTING.md 1.5/0.1-13); every path that reaches
	   `if (0 < rotate_flag)` has assigned both.  */
	let mut cos_r: c_int = 0;
	let mut sin_r: c_int = 0;
	if rotation == 0 as c_double
	  {
	    /* FLIP is always true here.  As this will rotate by 0
	       degrees, it has no visible effect.  Applying only
	       translation matrix to the image would be sufficient for
	       horizontal flipping, but writing special handling for
	       this case would increase code complexity somewhat.  */
	    cos_r = 1;
	    sin_r = 0;
	    rotate_flag = 1;

	    /* C 3180-3183: `#ifdef HAVE_HAIKU`.  */
	    #[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
	    {
	      extra_tx = width;
	      extra_ty = 0;
	    }
	  }
	else if rotation == 90 as c_double
	  {
	    width = (*img).height;
	    height = (*img).width;
	    cos_r = 0;
	    sin_r = 1;
	    rotate_flag = 1;

	    /* C 3189-3193: `#if defined HAVE_HAIKU`.  */
	    #[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
	    {
	      if !flip
		{ extra_ty = height; }
	      extra_tx = 0;
	    }
	  }
	else if rotation == 180 as c_double
	  {
	    cos_r = -1;
	    sin_r = 0;
	    rotate_flag = 1;

	    /* C 3198-3202: `#ifdef HAVE_HAIKU`.  */
	    #[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
	    {
	      if !flip
		{ extra_tx = width; }
	      extra_ty = height;
	    }
	  }
	else if rotation == 270 as c_double
	  {
	    width = (*img).height;
	    height = (*img).width;
	    cos_r = 0;
	    sin_r = -1;
	    rotate_flag = 1;

	    /* C 3210-3215: `#ifdef HAVE_HAIKU`.  */
	    #[cfg (all (feature = "have_window_system", feature = "have_haiku"))]
	    {
	      extra_tx = width;

	      if flip
		{ extra_ty = height; }
	    }
	  }

	if 0 < rotate_flag
	  {
	    /* C 3227-3266: `#  if defined USE_CAIRO || defined HAVE_XRENDER || defined HAVE_ANDROID`.  */
	    #[cfg (all (feature = "have_window_system",
			any (feature = "use_cairo", feature = "have_xrender",
			     feature = "have_android")))]
	    {
	      /* 1. Translate so (0, 0) is in the center of the image.  */
	      let mut t: matrix3x3
		= [[1 as c_double, 0 as c_double, 0 as c_double],
		   [0 as c_double, 1 as c_double, 0 as c_double],
		   [(*img).width as c_double * 0.5, (*img).height as c_double * 0.5, 1 as c_double]];
	      let mut u: matrix3x3 = [[0.0; 3]; 3];
	      matrix3x3_mult (t.as_mut_ptr (), matrix.as_mut_ptr (), u.as_mut_ptr ());

	      /* 2. Rotate.  */
	      let mut rot: matrix3x3
		= [[cos_r as c_double, -sin_r as c_double, 0 as c_double],
		   [sin_r as c_double, cos_r as c_double, 0 as c_double],
		   [0 as c_double, 0 as c_double, 1 as c_double]];
	      let mut v: matrix3x3 = [[0.0; 3]; 3];
	      matrix3x3_mult (rot.as_mut_ptr (), u.as_mut_ptr (), v.as_mut_ptr ());

	      /* 3. Translate back.  Flip horizontally if requested.  */
	      t[2][0] = width as c_double * -0.5;
	      t[2][1] = height as c_double * -0.5;
	      if flip
		{
		  t[0][0] = -t[0][0];
		  t[2][0] = -t[2][0];
		}
	      matrix3x3_mult (t.as_mut_ptr (), v.as_mut_ptr (), matrix.as_mut_ptr ());
	    }
	    /* C 3267-3287: `#  else`.  */
	    #[cfg (all (feature = "have_window_system",
			not (any (feature = "use_cairo", feature = "have_xrender",
				  feature = "have_android"))))]
	    {
	      /* 1. Translate so (0, 0) is in the center of the image.  */
	      let mut t: matrix3x3
		= [[1 as c_double, 0 as c_double, 0 as c_double],
		   [0 as c_double, 1 as c_double, 0 as c_double],
		   [(*img).width as c_double * -0.5, (*img).height as c_double * -0.5, 1 as c_double]];
	      let mut u: matrix3x3 = [[0.0; 3]; 3];
	      matrix3x3_mult (matrix.as_mut_ptr (), t.as_mut_ptr (), u.as_mut_ptr ());

	      /* 2. Rotate.  */
	      let mut rot: matrix3x3
		= [[cos_r as c_double, sin_r as c_double, 0 as c_double],
		   [-sin_r as c_double, cos_r as c_double, 0 as c_double],
		   [0 as c_double, 0 as c_double, 1 as c_double]];
	      let mut v: matrix3x3 = [[0.0; 3]; 3];
	      matrix3x3_mult (u.as_mut_ptr (), rot.as_mut_ptr (), v.as_mut_ptr ());

	      /* 3. Translate back.  Flip horizontally if requested.  */
	      t[2][0] = width as c_double * 0.5;
	      t[2][1] = height as c_double * 0.5;
	      if flip { t[0][0] = -t[0][0]; }
	      matrix3x3_mult (v.as_mut_ptr (), t.as_mut_ptr (), matrix.as_mut_ptr ());
	    }
	    (*img).width = width;
	    (*img).height = height;
	  }
      }

      /* C 3331-3492: `#else` (HAVE_ANDROID).  */
      #[cfg (all (feature = "have_window_system", feature = "have_android"))]
      {
	/* Calculate the inverse transform from the destination to the
	   source.  The matrix is currently identity with scale
	   applied.

           This code makes more sense to me than what lies above.  But
           I'm not touching what works.  */

	/* C 3344's `goto bail` - the label is the block's tail, so the skip is this flag (the
	   `fileio.rs` 4880 shape).  */
	let mut goto_bail: bool = false;
	if rotation != 0 as c_double && rotation != 90 as c_double
	   && rotation != 180 as c_double && rotation != 270 as c_double
	  {
	    rotate_flag = 0;
	    goto_bail = true;
	  }

	if !goto_bail
	  {
	    rotate_flag = 1;

	    /* C's `switch ((int) rotation + (flip ? 1 : 0))`; C has no `default`, so `_` is the
	       empty one (the `parse_image_spec` `switch`->`match` shape).  */
	    match (rotation as c_int) + (if flip { 1 } else { 0 })
	      {
		0 =>
		  { }

		90 =>
		  {
		    /* Rotate the image 90 degrees clockwise.  IOW, rotate the
		       destination by 90 degrees counterclockwise, which is 270
		       degrees clockwise.  */
		    matrix_rotate (matrix.as_mut_ptr (), M_PI * 1.5, 0 as c_double, 0 as c_double);
		    matrix_translate (matrix.as_mut_ptr (), (-height) as f32, 0 as f32);
		  }

		180 =>
		  {
		    /* Apply clockwise 180 degree rotation around the
		       center.  */
		    matrix_rotate (matrix.as_mut_ptr (), M_PI, width as c_double / 2.0,
				   height as c_double / 2.0);
		  }

		270 =>
		  {
		    /* Apply 270 degree counterclockwise rotation to the
		       destination, which is 90 degrees clockwise.  */
		    matrix_rotate (matrix.as_mut_ptr (), M_PI * 0.5, 0 as c_double, 0 as c_double);
		    matrix_translate (matrix.as_mut_ptr (), 0 as f32, (-width) as f32);
		  }

		1 =>
		  {
		    /* Flipped.  Apply horizontal flip.  */
		    matrix_mirror_horizontal (matrix.as_mut_ptr (), width as c_double);
		  }

		91 =>
		  {
		    /* Apply a flip but otherwise treat this the same as 90.  */
		    matrix_rotate (matrix.as_mut_ptr (), M_PI * 1.5, 0 as c_double, 0 as c_double);
		    matrix_translate (matrix.as_mut_ptr (), (-height) as f32, 0 as f32);
		    matrix_mirror_horizontal (matrix.as_mut_ptr (), height as c_double);
		  }

		181 =>
		  {
		    /* Flipped 180 degrees.  Apply a flip and treat this the
		       same as 180.  */
		    matrix_rotate (matrix.as_mut_ptr (), M_PI, width as c_double / 2.0,
				   height as c_double / 2.0);
		    matrix_mirror_horizontal (matrix.as_mut_ptr (), width as c_double);
		  }

		271 =>
		  {
		    /* Flipped 270 degrees.  Apply a flip and treat this the
		       same as 270.  */
		    matrix_rotate (matrix.as_mut_ptr (), M_PI * 0.5, 0 as c_double, 0 as c_double);
		    matrix_translate (matrix.as_mut_ptr (), 0 as f32, (-width) as f32);
		    matrix_mirror_horizontal (matrix.as_mut_ptr (), height as c_double);
		  }

		_ =>
		  { }
	      }

	    /* Now set img->width and img->height.  Flip them if the
	       rotation being applied requires so.  */

	    if rotation != 270 as c_double && rotation != 90 as c_double
	      {
		(*img).width = width;
		(*img).height = height;
	      }
	    else
	      {
		(*img).height = width;
		(*img).width = height;
	      }
	  }
	/* bail: */
      }
    }

  if rotate_flag < 0
    {
      image_error (c"No native support for rotation by %g degrees".as_ptr (),
		   make_float (rotation));
    }

  /* C 3494-3548: the six arms of C's store-the-transform chain.  */
  /* C 3494-3500: `# if defined (HAVE_NS)`.  */
  #[cfg (all (feature = "have_window_system", feature = "have_ns"))]
  {
    /* Under NS the transform is applied to the drawing surface at
       drawing time, so store it for later.  */
    ns_image_set_transform ((*img).pixmap, &raw mut matrix as *mut [f64; 3]);
    ns_image_set_smoothing ((*img).pixmap, smoothing);
  }
  /* C 3501-3517: `# elif defined USE_CAIRO`.  */
  #[cfg (all (feature = "have_window_system", not (feature = "have_ns"),
	      feature = "use_cairo"))]
  {
    /* C's positional initializer is written out with the record's own member names (`cairo.h`
       460-462's `xx, yx, xy, yy, x0, y0`, the order the clang reading above measured).  */
    let cr_matrix: cairo_matrix_t
      = cairo_matrix_t { xx: matrix[0][0], yx: matrix[0][1], xy: matrix[1][0],
			 yy: matrix[1][1], x0: matrix[2][0], y0: matrix[2][1] };
    let pattern: *mut cairo_pattern_t
      = cairo_pattern_create_rgb (0 as c_double, 0 as c_double, 0 as c_double);
    cairo_pattern_set_matrix (pattern, &raw const cr_matrix);

    /* Performance degrades with CAIRO_FILTER_BEST when using canvas, and possibly
       we get HW acceleration from CAIRO_FILTER_GOOD */
    let cairo_filter: c_int;
    if EQ (image_spec_value ((*img).spec, QCtype (), core::ptr::null_mut ()), Qcanvas ())
      { cairo_filter = if smoothing { CAIRO_FILTER_GOOD } else { CAIRO_FILTER_NEAREST }; }
    else
      { cairo_filter = if smoothing { CAIRO_FILTER_BEST } else { CAIRO_FILTER_NEAREST }; }
    cairo_pattern_set_filter (pattern, cairo_filter);

    /* Dummy solid color pattern just to record pattern matrix.  */
    (*img).cr_data = pattern as *mut c_void;
  }
  /* C 3518-3536: `# elif defined (HAVE_XRENDER)`.  */
  #[cfg (all (feature = "have_window_system", not (feature = "have_ns"),
	      not (feature = "use_cairo"), feature = "have_x_windows", feature = "have_xrender"))]
  if !(*img).picture.is_null ()
    {
      /* C 3520-3528: `XTransform tmat = {{{XDoubleToFixed (…), …}}};` - `XTransform` is
	 Xrender.h's `struct { XFixed matrix[3][3]; }` and `XFixed`/`XDoubleToFixed` the same
	 header's; no mirror carries them (the first slice's Xlib-name rule), so the record keeps
	 C's own member name and its initializer's shape.  */
      let tmat: XTransform
	= XTransform {
	    matrix: [[XDoubleToFixed (matrix[0][0]), XDoubleToFixed (matrix[1][0]),
		      XDoubleToFixed (matrix[2][0])],
		     [XDoubleToFixed (matrix[0][1]), XDoubleToFixed (matrix[1][1]),
		      XDoubleToFixed (matrix[2][1])],
		     [XDoubleToFixed (matrix[0][2]), XDoubleToFixed (matrix[1][2]),
		      XDoubleToFixed (matrix[2][2])]] };

      XRenderSetPictureFilter (FRAME_X_DISPLAY! (f), (*img).picture,
			       if smoothing { FilterBest } else { FilterNearest }, 0, 0);
      XRenderSetPictureTransform (FRAME_X_DISPLAY! (f), (*img).picture, &raw const tmat);

      if !(*img).mask_picture.is_null ()
	{
	  XRenderSetPictureFilter (FRAME_X_DISPLAY! (f), (*img).mask_picture,
				   if smoothing { FilterBest } else { FilterNearest }, 0, 0);
	  XRenderSetPictureTransform (FRAME_X_DISPLAY! (f), (*img).mask_picture,
				      &raw const tmat);
	}
    }
  /* C 3537-3545: `# elif defined HAVE_NTGUI`.  */
  #[cfg (all (feature = "have_window_system", not (feature = "have_ns"),
	      not (feature = "use_cairo"), not (feature = "have_x_windows"),
	      feature = "have_ntgui"))]
  {
    /* Store the transform matrix for application at draw time.  */
    /* The mirror's `xform` member is the untyped pointer its own P-72 rule 3 note records (a
       by-value `XFORM` is not preserved), so C's six stores have no field to reach; C's text is
       kept and the arm is this machine's Windows text-only class.  */
    (*img).xform.eM11 = matrix[0][0];
    (*img).xform.eM12 = matrix[0][1];
    (*img).xform.eM21 = matrix[1][0];
    (*img).xform.eM22 = matrix[1][1];
    (*img).xform.eDx  = matrix[2][0];
    (*img).xform.eDy  = matrix[2][1];
  }
  /* C 3546-3554: `# elif defined HAVE_HAIKU`.  */
  #[cfg (all (feature = "have_window_system", not (feature = "have_ns"),
	      not (feature = "use_cairo"), not (feature = "have_x_windows"),
	      not (feature = "have_ntgui"), feature = "have_haiku"))]
  {
    /* Store the transform in the struct image for later.  */
    memcpy (&raw mut (*img).transform as *mut c_void, &raw const matrix as *const c_void,
	    core::mem::size_of::<matrix3x3> ());

    /* Also add the extra translations.   */
    if rotate_flag != 0
      {
	(*img).transform[0][2] = extra_tx as c_double;
	(*img).transform[1][2] = extra_ty as c_double;
      }
  }
  /* C 3556-3666: `# elif defined HAVE_ANDROID`.  */
  #[cfg (all (feature = "have_window_system", not (feature = "have_ns"),
	      not (feature = "use_cairo"), not (feature = "have_x_windows"),
	      not (feature = "have_ntgui"), not (feature = "have_haiku"),
	      feature = "have_android"))]
  {
    /* Create a new image of the right size, then turn it into a pixmap
       and set that as img->pixmap.  Destroy img->mask for now (this is
       not right.)  */

    let mut transformed_image: *mut android_image;
    let mut image: *mut android_image;
    let mut transform: android_transform;

    /* If there is no transform, simply return.  */
    if memcmp (&raw const matrix as *const c_void, &raw const identity as *const c_void,
	       core::mem::size_of::<matrix3x3> ()) == 0
      { return; }

    /* First, get the source image.  */
    image = image_get_x_image (f, img, false);

    /* Make the transformed image.  */
    transformed_image = android_create_image ((*image).depth as c_uint, ANDROID_Z_PIXMAP,
					      core::ptr::null_mut (), (*img).width as c_uint,
					      (*img).height as c_uint);

    /* Allocate memory for that image.  */
    (*transformed_image).data
      = xmalloc ((((*transformed_image).bytes_per_line as c_uint)
		  * ((*transformed_image).height as c_uint)) as usize) as *mut c_char;

    /* Do the transform.  */
    transform = core::mem::zeroed ();
    transform.m1 = matrix[0][0] as f32;
    transform.m2 = matrix[0][1] as f32;
    transform.m3 = matrix[0][2] as f32;
    transform.m4 = matrix[1][0] as f32;
    transform.m5 = matrix[1][1] as f32;
    transform.m6 = matrix[1][2] as f32;

    if (*image).depth == 24 && smoothing
      {
	android_project_image_bilinear (image, transformed_image, &raw mut transform);
      }
    else
      {
	android_project_image_nearest (image, transformed_image, &raw mut transform);
      }

    image_unget_x_image (img, false, image);

    /* Now replace the image.  */

    if !(*img).ximg.is_null ()
      { image_destroy_x_image ((*img).ximg as Emacs_Pix_Container); }

    (*img).ximg = transformed_image as *mut c_void;

    #[cfg (not (feature = "android_stubify"))]
    {
      /* Then replace the pixmap.  */
      android_free_pixmap ((*img).pixmap as android_pixmap);

      /* In case android_create_pixmap signals.  */
      (*img).pixmap = ANDROID_NONE as *mut c_void;
      (*img).pixmap = android_create_pixmap ((*img).width as c_uint, (*img).height as c_uint,
					     (*transformed_image).depth) as *mut c_void;
      android_put_image ((*img).pixmap as android_pixmap, transformed_image);
    }
    #[cfg (feature = "android_stubify")]
    { emacs_abort (); }

    /* Now, transform the mask.  The mask should be depth 1, and is
       always transformed using a nearest neighbor filter.  */

    if !(*img).mask_img.is_null () || !(*img).mask.is_null ()
      {
	image = image_get_x_image (f, img, true);
	transformed_image = android_create_image (1 as c_uint, ANDROID_Z_PIXMAP,
						  core::ptr::null_mut (),
						  (*img).width as c_uint,
						  (*img).height as c_uint);
	(*transformed_image).data
	  = xmalloc ((((*transformed_image).bytes_per_line as c_uint)
		      * ((*transformed_image).height as c_uint)) as usize) as *mut c_char;
	android_project_image_nearest (image, transformed_image, &raw mut transform);
	image_unget_x_image (img, true, image);

	/* Now replace the image.  */

	if !(*img).mask_img.is_null ()
	  { image_destroy_x_image ((*img).mask_img as Emacs_Pix_Container); }

	(*img).mask_img = transformed_image as *mut c_void;

	#[cfg (not (feature = "android_stubify"))]
	{
	  if !(*img).mask.is_null ()
	    { android_free_pixmap ((*img).mask as android_pixmap); }

	  (*img).mask = ANDROID_NONE as *mut c_void;
	  (*img).mask = android_create_pixmap ((*img).width as c_uint, (*img).height as c_uint,
					       1) as *mut c_void;
	  android_put_image ((*img).mask as android_pixmap, transformed_image);
	}
      }

    /* Done! */
  }
}


/* ----------------------------------------------------------------------------------------------
   C 3552-3890: the file's fourth slice, part one - the rest of the "Image Cache" section and the
   two GC marking functions.

   It holds `lookup_image` (C 3552-3688), `cache_image` (C 3690-3727), the `HAVE_WEBP ||
   HAVE_GIF` animation-cache block's three records and `anim_create_cache` /
   `anim_prune_animation_cache` / `anim_get_animation_cache` (C 3730-3860), `mark_image`
   (C 3862-3871) and `mark_image_cache` (C 3873-3890).

   The per-`#if` verdicts, the C line-range rows and this round's readings are in
   `docs/ports/image.md` (`## C line ranges`, `## Evidence`, `## Deviations`).  Names the mirrors
   this unit includes do not carry stand at the use site with C's own prototype in a `/// C:` line
   (PORTING.md P-55/P-72 rule 3); names the platform SDKs supply keep C's tokens.
   ---------------------------------------------------------------------------------------------- */

/* The names the mirrors this unit includes do not carry, declared at the use site with C's own
   line (PORTING.md P-55/P-72 rule 3); `docs/ports/image.md` names the file each belongs in.  */

// C: frame.h 1379: `#define FRAME_FACE_CACHE(F) (F)->face_cache`
// `include/frame.rs` 184 carries the member as the untyped pointer (P-72 rule 3), so the cast the
// `IMAGE_FROM_ID!` above already needs is repeated here.  C's macro pastes into every unit that
// includes the header, so the port's live form is one copy per including unit and the mirror's own
// copy is a module this unit does not glob-import - `src/term.rs` 364-377 is the same shape.
#[cfg (feature = "have_window_system")]
#[inline(always)]
fn FRAME_FACE_CACHE (f: *mut frame) -> *mut face_cache
{ unsafe { (*f).face_cache as *mut face_cache } }
// C: frame.h 1646-1650: `INLINE struct face * FACE_FROM_ID (struct frame *f, int id)`
#[cfg (feature = "have_window_system")]
#[inline(always)]
fn FACE_FROM_ID (f: *mut frame, id: c_int) -> *mut face
{
  eassert! (0 <= id && id < unsafe { (*FRAME_FACE_CACHE (f)).used });
  unsafe { *(*FRAME_FACE_CACHE (f)).faces_by_id.add (id as usize) }
}
// C: dispextern.h 3821: `void init_frame_faces (struct frame *);` - the definition is `xfaces.c`'s.
#[cfg (feature = "have_window_system")]
unsafe extern "C" {
  pub fn init_frame_faces (f: *mut frame);
}
// C: dispextern.h 3823: `void recompute_basic_faces (struct frame *);` - the definition is
// `dispnew.c`'s.
#[cfg (feature = "have_window_system")]
unsafe extern "C" {
  pub fn recompute_basic_faces (f: *mut frame);
}
// C: dispextern.h 3819: `extern struct image_cache *share_image_cache (struct frame *f);` - the
// definition is `xfaces.c`'s; `src/frame.rs` 4783-4784 declares the same name in its own block.
#[cfg (feature = "have_window_system")]
unsafe extern "C" {
  pub fn share_image_cache (f: *mut frame) -> *mut image_cache;
}
// C: lisp.h 4436: `extern void mark_object (Lisp_Object);` - the definition is `alloc.c`'s.
#[cfg (feature = "have_window_system")]
unsafe extern "C" {
  pub fn mark_object (obj: Lisp_Object);
}
/// C: dispextern.h 3311-3312: `#define DEFAULT_IMAGE_WIDTH 30` / `#define DEFAULT_IMAGE_HEIGHT 30`
/// Mirror gap: `include/dispextern.rs` carries the two corner macros and the two ascent constants
/// around them but not these two.
#[cfg (feature = "have_window_system")]
pub const DEFAULT_IMAGE_WIDTH: c_int = 30;
#[cfg (feature = "have_window_system")]
pub const DEFAULT_IMAGE_HEIGHT: c_int = 30;

/* Return the id of image with Lisp specification SPEC on frame F.
   SPEC must be a valid Lisp image specification (see valid_image_p).  */

/// C: C 3555-3556: `ptrdiff_t lookup_image (struct frame *f, Lisp_Object spec, int face_id)` - non-static.
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lookup_image (f: *mut frame, spec: Lisp_Object, mut face_id: c_int)
  -> ptrdiff_t
{
  let mut img: *mut image;
  let mut hash: EMACS_UINT;

  if FRAME_FACE_CACHE (f).is_null ()
    { init_frame_faces (f); }
  if (*FRAME_FACE_CACHE (f)).used == 0
    { recompute_basic_faces (f); }
  if face_id < 0 || face_id >= (*FRAME_FACE_CACHE (f)).used
    { face_id = DEFAULT_FACE_ID; }

  let face: *mut face = FACE_FROM_ID (f, face_id);
  let mut foreground: c_ulong = (*face).foreground;
  let mut background: c_ulong = (*face).background;
  let font_size: c_int = (*((*face).font as *mut font)).pixel_size;
  let font_family: *mut c_char = SSDATA ((*face).lface[LFACE_FAMILY_INDEX as usize]);

  /* F must be a window-system frame, and SPEC must be a valid image
     specification.  */
  eassert! (FRAME_WINDOW_P (f));
  eassert! (valid_image_p (spec));

  /* Look up SPEC in the hash table of the image cache.  */
  hash = sxhash (filter_image_spec (spec));
  img = search_image_cache (f, spec, hash, foreground, background, font_size, font_family, false);
  if !img.is_null () && (*img).load_failed_p
    {
      free_image (f, img);
      img = core::ptr::null_mut ();
    }

  /* If not found, create a new image and cache it.  */
  if img.is_null ()
    {
      block_input ();
      img = make_image (spec, hash);
      cache_image (f, img);
      (*img).face_foreground = foreground;
      (*img).face_background = background;
      (*img).face_font_size = font_size;
      (*img).face_font_height = (*((*face).font as *mut font)).height;
      (*img).face_font_width = (*((*face).font as *mut font)).average_width;
      let len: ptrdiff_t = (strlen (font_family) + 1) as ptrdiff_t;
      (*img).face_font_family = xmalloc (len as usize) as *mut c_char;
      memcpy ((*img).face_font_family as *mut c_void, font_family as *const c_void, len as usize);
      (*img).load_failed_p
	= ! (*((*img).r#type as *const image_type)).load_img.unwrap () (f, img);

      /* If we can't load the image, and we don't have a width and
	 height, use some arbitrary width and height so that we can
	 draw a rectangle for it.  */
      if (*img).load_failed_p
	{
	  let mut value: Lisp_Object;

	  value = image_spec_value (spec, QCwidth (), core::ptr::null_mut ());
	  (*img).width = if FIXNUMP (value)
			   { XFIXNAT (value) as c_int } else { DEFAULT_IMAGE_WIDTH };
	  value = image_spec_value (spec, QCheight (), core::ptr::null_mut ());
	  (*img).height = if FIXNUMP (value)
			    { XFIXNAT (value) as c_int } else { DEFAULT_IMAGE_HEIGHT };
	}
      else
	{
	  /* Handle image type independent image attributes
	     `:ascent ASCENT', `:margin MARGIN', `:relief RELIEF',
	     `:background COLOR'.  */
	  let mut ascent: Lisp_Object;
	  let mut margin: Lisp_Object;
	  let mut relief: Lisp_Object;
	  let mut bg: Lisp_Object;
	  let mut relief_bound: c_int;

	  ascent = image_spec_value (spec, QCascent (), core::ptr::null_mut ());
	  if FIXNUMP (ascent)
	    { (*img).ascent = XFIXNUM (ascent) as c_int; }
	  else if EQ (ascent, Qcenter ())
	    { (*img).ascent = CENTERED_IMAGE_ASCENT; }

	  margin = image_spec_value (spec, QCmargin (), core::ptr::null_mut ());
	  if FIXNUMP (margin)
	    { (*img).hmargin = XFIXNUM (margin) as c_int; (*img).vmargin = (*img).hmargin; }
	  else if CONSP (margin)
	    {
	      (*img).hmargin = XFIXNUM (XCAR (margin)) as c_int;
	      (*img).vmargin = XFIXNUM (XCDR (margin)) as c_int;
	    }

	  relief = image_spec_value (spec, QCrelief (), core::ptr::null_mut ());
	  relief_bound = INT_MAX - std::cmp::max ((*img).hmargin, (*img).vmargin);
	  if RANGED_FIXNUMP (- (relief_bound as EMACS_INT), relief, relief_bound as EMACS_INT)
	    {
	      (*img).relief = XFIXNUM (relief) as c_int;
	      (*img).hmargin += eabs! ((*img).relief) as c_int;
	      (*img).vmargin += eabs! ((*img).relief) as c_int;
	    }

	  if (*img).bits & IMAGE_BACKGROUND_VALID_MASK == 0
	    {
	      bg = image_spec_value ((*img).spec, QCbackground (), core::ptr::null_mut ());
	      if !NILP (bg)
		{
		  (*img).background = image_alloc_image_color (f, img, bg, background);
		  /* C's `img->background_valid = 1;` - the bit member's storage unit (P-80).  */
		  (*img).bits = (*img).bits & !IMAGE_BACKGROUND_VALID_MASK
				| ((1 as u32) << IMAGE_BACKGROUND_VALID_SHIFT);
		}
	    }

	  /* Store the original width and height before transforming the image.  */
	  (*img).original_width = (*img).width;
	  (*img).original_height = (*img).height;

	  /* Do image transformations and compute masks, unless we
	     don't have the image yet.  */
	  if !EQ (builtin_lisp_symbol ((*((*img).r#type as *const image_type)).r#type as usize),
		  Qpostscript ())
	    { postprocess_image (f, img); }

          /* postprocess_image above may modify the image or the mask,
             relying on the image's real width and height, so
             image_set_transform must be called after it.  */
	  /* C 3645-3647: `#ifdef HAVE_NATIVE_TRANSFORMS`.  */
	  #[cfg (all (feature = "have_window_system", feature = "have_native_transforms"))]
          image_set_transform (f, img);
	}

      unblock_input ();
    }

  /* IMG is now being used, so set its timestamp to the current
     time.  */
  (*img).timestamp = current_timespec ();

  /* Value is the image id.  */
  return (*img).id;
}


/* Cache image IMG in the image cache of frame F.  */

/// C: C 3693-3694: `static void cache_image (struct frame *f, struct image *img)`
#[cfg (feature = "have_window_system")]
unsafe fn cache_image (f: *mut frame, img: *mut image)
{
  let mut c: *mut image_cache = FRAME_IMAGE_CACHE! (f) as *mut image_cache;
  let mut i: ptrdiff_t;

  if c.is_null ()
    {
      /* C's `c = FRAME_IMAGE_CACHE (f) = share_image_cache (f);` - the two stores.  */
      c = share_image_cache (f);
      (*f).image_cache = c as *mut c_void;
      (*c).refcount += 1;
    }

  /* Find a free slot in c->images.  */
  i = 0;
  while i < (*c).used
    {
      if *(*c).images.offset (i as isize) == core::ptr::null_mut ()
	{ break; }
      i += 1;
    }

  /* If no free slot found, maybe enlarge c->images.  */
  if i == (*c).used && (*c).used == (*c).size
    { (*c).images = xpalloc ((*c).images as *mut c_void, &raw mut (*c).size, 1, -1,
			     core::mem::size_of::<*mut image> () as ptrdiff_t)
		      as *mut *mut image; }

  /* Add IMG to c->images, and assign IMG an id.  */
  *(*c).images.offset (i as isize) = img;
  (*img).id = i;
  if i == (*c).used
    { (*c).used += 1; }

  /* Add IMG to the cache's hash table.  */
  i = ((*img).hash % IMAGE_CACHE_BUCKETS_SIZE as EMACS_UINT) as ptrdiff_t;
  (*img).next = *(*c).buckets.offset (i as isize);
  if !(*img).next.is_null ()
    { (*(*img).next).prev = img; }
  (*img).prev = core::ptr::null_mut ();
  *(*c).buckets.offset (i as isize) = img;
}


/* C 3730-3860: `#if defined (HAVE_WEBP) || defined (HAVE_GIF)` - the animation-cache section.
   `<gif_lib.h>` and `<webp/...>` have no file in the reference tree and no mirror, so the two
   records C names only through a pointer are declared opaque here (P-88: the declarations this
   unit needs from an SDK stand in the unit, with the SDK header as the citation), the `GtkWidget`
   shape.  */

/* C 3732-3738: `# ifdef HAVE_GIF`.  */
/// C: gif_lib.h 247: `typedef struct GifFileType { ... } GifFileType;` - the port never reads a
/// member of it (the GIF reader is C 9xxx's).
#[cfg (all (feature = "have_window_system", any (feature = "have_webp", feature = "have_gif"),
	    feature = "have_gif"))]
#[repr(C)]
pub struct gif_anim_handle
{
  /// C: `struct GifFileType *gif;`
  pub gif: *mut c_void,
  pub pixmap: *mut c_ulong,
}
/* C 3740-3751: `# ifdef HAVE_WEBP`.  */
/// C: webp/demux.h: `typedef struct WebPAnimDecoder WebPAnimDecoder;` - the port never reads a
/// member of it (the WebP reader is C 9xxx's).
#[cfg (all (feature = "have_window_system", any (feature = "have_webp", feature = "have_gif"),
	    feature = "have_webp"))]
#[repr(C)]
pub struct webp_anim_handle
{
  /* Decoder iterator+compositor.  */
  /// C: `struct WebPAnimDecoder *dec;`
  pub dec: *mut c_void,
  /* Owned copy of input WebP bitstream data consumed by decoder,
     which it must outlive unchanged.  */
  pub contents: *mut uint8_t,
  /* Timestamp in milliseconds of last decoded frame.  */
  pub timestamp: c_int,
}

/* To speed animations up, we keep a cache (based on EQ-ness of the
   image spec/object) where we put the animator iterator.  */

/// C: C 3756-3788: `struct anim_cache` (which forward-references itself through `next`)
#[cfg (all (feature = "have_window_system", any (feature = "have_webp", feature = "have_gif")))]
#[repr(C)]
pub struct anim_cache
{
  /* 'Key' of this cache entry.
     Typically the cdr (plist) of an image spec.  */
  pub spec: Lisp_Object,                       // C 3760
  /* Image type dependent animation handle (e.g., WebP iterator), freed
     by 'destructor'.  The union allows maintaining multiple fields per
     image type and image frame without further heap allocations.  */
  pub handle: anim_handle,                     // C 3764-3772
  /* A function to call to free the handle.  */
  /// C 3774: `void (*destructor) (union anim_handle *);`
  pub destructor: Option<unsafe extern "C" fn (handle: *mut anim_handle)>,
  /* Current frame index, and total number of frames.  Note that
     different image formats may start at different indices.  */
  pub index: c_int,                            // C 3777: `int index, frames;`
  pub frames: c_int,
  /* Animation frame dimensions.  */
  pub width: c_int,                            // C 3779: `int width, height;`
  pub height: c_int,
  /* This is used to be able to say something about the cache size.
     We don't know how much memory the different libraries actually
     use here (since these cache structures are opaque), so this is
     mostly just the size of the original image file.  */
  pub byte_size: intmax_t,                     // C 3784
  /* Last time this cache entry was updated.  */
  pub update_time: timespec,                   // C 3786
  pub next: *mut anim_cache,                   // C 3787
}

/* C 3764-3772: the nested `union anim_handle` - one `#[repr(C)]` union whose members are the two
   per-format records above (PORTING.md 1.4's untagged-union rule).  */
#[cfg (all (feature = "have_window_system", any (feature = "have_webp", feature = "have_gif")))]
#[repr(C)]
pub union anim_handle
{
  /* C 3766-3768: `# ifdef HAVE_GIF`.  */
  #[cfg (feature = "have_gif")]
  pub gif: gif_anim_handle,
  /* C 3769-3771: `# ifdef HAVE_WEBP`.  */
  #[cfg (feature = "have_webp")]
  pub webp: webp_anim_handle,
}

/// C: C 3790: `static struct anim_cache *anim_cache = NULL;`
#[cfg (all (feature = "have_window_system", any (feature = "have_webp", feature = "have_gif")))]
static mut anim_cache: *mut anim_cache = core::ptr::null_mut ();

/* Return a new animation cache entry for image SPEC (which need not be
   an image specification, and is typically its cdr/plist).
   Freed only by pruning the cache.  */

/// C: C 3795-3796: `static ATTRIBUTE_MALLOC struct anim_cache * anim_create_cache (Lisp_Object spec)`
/// (`ATTRIBUTE_MALLOC` is an allocator attribute, not part of the type - PORTING.md 8).
#[cfg (all (feature = "have_window_system", any (feature = "have_webp", feature = "have_gif")))]
unsafe fn anim_create_cache (spec: Lisp_Object) -> *mut anim_cache
{
  let cache: *mut anim_cache = xzalloc (core::mem::size_of::<anim_cache> ()) as *mut anim_cache;
  (*cache).spec = spec;
  (*cache).index = -1;
  (*cache).frames = -1;
  (*cache).width = -1;
  (*cache).height = -1;
  return cache;
}

/* Discard cached images that haven't been used for a minute.  If
   CLEAR is t, remove all animation cache entries.  If CLEAR is
   anything other than nil or t, only remove the entries that have a
   spec `eq' to CLEAR.  */

/// C: C 3811-3812: `static void anim_prune_animation_cache (Lisp_Object clear)`
#[cfg (all (feature = "have_window_system", any (feature = "have_webp", feature = "have_gif")))]
unsafe fn anim_prune_animation_cache (clear: Lisp_Object)
{
  let mut pcache: *mut *mut anim_cache = &raw mut anim_cache;
  let old: timespec = timespec_sub (current_timespec (), make_timespec (60, 0));

  while !(*pcache).is_null ()
    {
      let cache: *mut anim_cache = *pcache;
      if EQ (clear, Qt ())
	 || (NILP (clear) && timespec_cmp (old, (*cache).update_time) > 0)
	 || EQ (clear, (*cache).spec)
	{
	  if (*cache).destructor.is_some ()
	    { ((*cache).destructor.unwrap ()) (&raw mut (*cache).handle); }
	  *pcache = (*cache).next;
	  xfree (cache as *mut c_void);
	}
      else
	{ pcache = &raw mut (*cache).next; }
    }
}

/// C: C 3835-3836: `static struct anim_cache * anim_get_animation_cache (Lisp_Object spec)`
#[cfg (all (feature = "have_window_system", any (feature = "have_webp", feature = "have_gif")))]
unsafe fn anim_get_animation_cache (spec: Lisp_Object) -> *mut anim_cache
{
  let mut cache: *mut anim_cache;
  let mut pcache: *mut *mut anim_cache = &raw mut anim_cache;

  anim_prune_animation_cache (Qnil ());

  /* C's `while (1)` (`loop` is the tree's spelling of it).  */
  loop
    {
      cache = *pcache;
      if cache.is_null ()
	{
          *pcache = anim_create_cache (spec);
	  cache = *pcache;
          break;
        }
      if EQ (spec, (*cache).spec)
	{ break; }
      pcache = &raw mut (*cache).next;
    }

  (*cache).update_time = current_timespec ();
  return cache;
}

/* Mark Lisp objects in image IMG.  */

/// C: C 3863-3864: `static void mark_image (struct image *img)`
#[cfg (feature = "have_window_system")]
unsafe fn mark_image (img: *mut image)
{
  mark_object ((*img).spec);
  mark_object ((*img).dependencies);

  if !NILP ((*img).lisp_data)
    { mark_object ((*img).lisp_data); }
}

/* Mark every image in image cache C, as well as the global animation
   cache.  */

/// C: C 3876-3877: `void mark_image_cache (struct image_cache *c)` - non-static.
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mark_image_cache (c: *mut image_cache)
{
  if !c.is_null ()
    {
      let mut i: ptrdiff_t;
      i = 0;
      while i < (*c).used
	{
	  if !(*(*c).images.offset (i as isize)).is_null ()
	    { mark_image (*(*c).images.offset (i as isize)); }
	  i += 1;
	}
    }

  /* C 3886-3889: `#if defined HAVE_WEBP || defined HAVE_GIF`.  */
  #[cfg (any (feature = "have_webp", feature = "have_gif"))]
  {
    let mut cache: *mut anim_cache = anim_cache;
    while !cache.is_null ()
      {
	mark_object ((*cache).spec);
	cache = (*cache).next;
      }
  }
}


/***********************************************************************
			  X / NS / W32 support code
             Most of this code is shared with Android to make
             it easier to maintain.
 ***********************************************************************/

/* C 3900: `#if defined HAVE_X_WINDOWS || defined HAVE_ANDROID` - it closes at C 4114 (whose
   `#endif` C mislabels `/* HAVE_X_WINDOWS */`).  C's own banner above is the section's.  */

/* Respect Xlib's limits and C's own for the raster a window-system image may allocate.  */

/// C: C 3902-3903: `static bool x_check_image_size (XImage *ximg, int width, int height)`
#[cfg (all (feature = "have_window_system", any (feature = "have_x_windows", feature = "have_android")))]
unsafe fn x_check_image_size (ximg: *mut XImage, width: c_int, height: c_int) -> bool
{
  /* Respect Xlib's limits: it cannot deal with images that have more
     than INT_MAX (and/or UINT_MAX) bytes.  And respect Emacs's limits
     of PTRDIFF_MAX (and/or SIZE_MAX) bytes for any object.  */
  /* C's local `enum` - the port's `const` items with C's own value types (`min` of a signed and
     an unsigned int is C's usual-arithmetic `unsigned int`; the outer `min` is `unsigned long`,
     which is `PTRDIFF_MAX`).  */
  const XLIB_BYTES_MAX: c_uint = if (INT_MAX as c_uint) < UINT_MAX { INT_MAX as c_uint } else { UINT_MAX };
  const X_IMAGE_BYTES_MAX: c_ulong
    = if (XLIB_BYTES_MAX as c_ulong) < (if (PTRDIFF_MAX as c_ulong) < (SIZE_MAX as c_ulong)
					{ PTRDIFF_MAX as c_ulong } else { SIZE_MAX as c_ulong })
	{ XLIB_BYTES_MAX as c_ulong }
      else
	{ if (PTRDIFF_MAX as c_ulong) < (SIZE_MAX as c_ulong)
	    { PTRDIFF_MAX as c_ulong } else { SIZE_MAX as c_ulong } };

  let (mut bitmap_pad, mut depth, mut bytes_per_line): (c_int, c_int, c_int);
  if !ximg.is_null ()
    {
      /* C 3917-3921: `#ifndef HAVE_ANDROID` / `#else`.  */
      #[cfg (not (feature = "have_android"))]
      { bitmap_pad = (*ximg).bitmap_pad; }
      #[cfg (feature = "have_android")]
      { bitmap_pad = if (*ximg).depth == 1 { 8 } else { 32 }; }
      depth = (*ximg).depth;
      bytes_per_line = (*ximg).bytes_per_line;
    }
  else
    {
      bitmap_pad = 8;
      depth = 1;
      bytes_per_line = (width >> 3) + (((width & 7) != 0) as c_int);
    }
  return (width <= (INT_MAX - (bitmap_pad - 1)) / depth
	  && height as c_ulong <= X_IMAGE_BYTES_MAX / bytes_per_line as c_ulong);
}

/* Create an X image and a pixmap of size WIDTH x HEIGHT for use on
   frame F.  Set *XIMG and *PIXMAP to the XImage and Pixmap created.  */

/// C: C 3935-3937: `static bool x_create_x_image_and_pixmap (struct frame *f, int width, int height, int depth, XImage **ximg, Pixmap *pixmap)`
#[cfg (all (feature = "have_window_system", any (feature = "have_x_windows", feature = "have_android")))]
unsafe fn x_create_x_image_and_pixmap (f: *mut frame, width: c_int, height: c_int,
				       mut depth: c_int, ximg: *mut *mut XImage,
				       pixmap: *mut Pixmap) -> bool
{
  /* C 3939-3942: `#ifndef HAVE_ANDROID`'s two locals.  */
  #[cfg (not (feature = "have_android"))]
  let display: *mut Display = FRAME_X_DISPLAY! (f);
  #[cfg (not (feature = "have_android"))]
  let drawable: Drawable = FRAME_X_DRAWABLE! (f);

  eassert! (input_blocked_p ());

  if depth <= 0
    { depth = (*FRAME_DISPLAY_INFO! (f)).n_planes; }
  /* C 3948-3955: `#ifndef HAVE_ANDROID` (Xlib's `XCreateImage`) / `#else` (the NDK's).  */
  /* A cfg'd *assignment* is not a statement Rust accepts an attribute on (only blocks, `let`s,
     calls and macros are), so both arms are C's expression inside a block.  */
  #[cfg (not (feature = "have_android"))]
  {
    *ximg = XCreateImage (display, FRAME_X_VISUAL! (f), depth as c_uint, ZPixmap, 0,
			  core::ptr::null_mut (), width as c_uint, height as c_uint,
			  if depth > 16 { 32 } else if depth > 8 { 16 } else { 8 }, 0);
  }
  #[cfg (feature = "have_android")]
  {
    *ximg = android_create_image (depth as c_uint, ANDROID_Z_PIXMAP, core::ptr::null_mut (),
				  width as c_uint, height as c_uint);
  }
  if (*ximg).is_null ()
    {
      image_error (c"Unable to allocate X image".as_ptr ());
      return false;
    }

  if !x_check_image_size (*ximg, width, height)
    {
      x_destroy_x_image (*ximg);
      *ximg = core::ptr::null_mut ();
      image_error (c"Image too large (%dx%d)".as_ptr (), make_fixnum (width as EMACS_INT),
		   make_fixnum (height as EMACS_INT));
      return false;
    }

  /* Allocate image raster.  */
  (**ximg).data = xmalloc (((**ximg).bytes_per_line * height) as usize) as *mut c_char;

  /* Allocate a pixmap of the same size.  */
  /* C 3975-3983: `#ifndef HAVE_ANDROID` / `#else` (`#ifndef ANDROID_STUBIFY` inside it).  */
  #[cfg (not (feature = "have_android"))]
  { *pixmap = XCreatePixmap (display, drawable, width as c_uint, height as c_uint, depth as c_uint); }
  #[cfg (all (feature = "have_android", not (feature = "android_stubify")))]
  { *pixmap = android_create_pixmap (width as c_uint, height as c_uint, depth) as Pixmap; }
  #[cfg (all (feature = "have_android", feature = "android_stubify"))]
  { emacs_abort (); }
  if *pixmap == NO_PIXMAP! () as Pixmap
    {
      x_destroy_x_image (*ximg);
      *ximg = core::ptr::null_mut ();
      image_error (c"Unable to create X pixmap".as_ptr ());
      return false;
    }

  return true;
}

/// C: C 3995-3996: `static void x_destroy_x_image (XImage *ximg)`
#[cfg (all (feature = "have_window_system", any (feature = "have_x_windows", feature = "have_android")))]
unsafe fn x_destroy_x_image (ximg: *mut XImage)
{
  if !(*ximg).data.is_null ()
    {
      xfree ((*ximg).data as *mut c_void);
      (*ximg).data = core::ptr::null_mut ();
    }

  /* C 4004-4008: `#ifndef HAVE_ANDROID` / `#else`.  */
  #[cfg (not (feature = "have_android"))]
  { XDestroyImage (ximg); }
  #[cfg (feature = "have_android")]
  { android_destroy_image (ximg as *mut android_image); }
}

/* C 4011-4056: `# if !defined USE_CAIRO && defined HAVE_XRENDER` - XRender's `Picture` and the
   attributes/repeat names below are Xrender.h's; no mirror carries them (`include/xlib.rs` carries
   no Xrender surface), so the arm keeps C's tokens - the `XTransform`/`XDoubleToFixed` rule the
   third slice's `image_set_transform` already follows, and the ledger records the names.  */
/* Create and return an XRender Picture for XRender transforms.  */

/// C: C 4013-4014: `static Picture x_create_xrender_picture (struct frame *f, Emacs_Pixmap pixmap, int depth)`
#[cfg (all (feature = "have_window_system", not (feature = "use_cairo"), feature = "have_x_windows",
	    feature = "have_xrender"))]
unsafe fn x_create_xrender_picture (f: *mut frame, pixmap: Emacs_Pixmap, mut depth: c_int)
  -> Picture
{
  /* C's `None` is X11/X.h 115's (`include/xlib.rs`'s `x11_constants`), the `xterm.rs` shape.  */
  use x11_constants::None;
  let mut p: Picture = None as Picture;
  let display: *mut Display = FRAME_X_DISPLAY! (f);

  if (*FRAME_DISPLAY_INFO! (f)).xrender_supported_p
    {
      if depth <= 0
	{ depth = (*FRAME_DISPLAY_INFO! (f)).n_planes; }
      if depth == 32 || depth == 24 || depth == 8 || depth == 4 || depth == 1
        {
          /* FIXME: Do we need to handle all possible bit depths?
             XRenderFindStandardFormat supports PictStandardARGB32,
             PictStandardRGB24, PictStandardA8, PictStandardA4,
             PictStandardA1, and PictStandardNUM (what is this?!).

             XRenderFindFormat may support more, but I don't
             understand the documentation.  */
          let mut format: *mut XRenderPictFormat;
          format = XRenderFindStandardFormat (display,
                                              if depth == 32 { PictStandardARGB32 }
                                              else if depth == 24 { PictStandardRGB24 }
                                              else if depth == 8 { PictStandardA8 }
                                              else if depth == 4 { PictStandardA4 }
                                              else { PictStandardA1 });

          /* Set the Picture repeat to "pad".  This means when
             operations look at pixels outside the image area they
             will use the value of the nearest real pixel instead of
             using a transparent black pixel.  */
          let mut attr: XRenderPictureAttributes;
          let mut attr_mask: c_ulong = CPRepeat;
          attr.repeat = RepeatPad;

          p = XRenderCreatePicture (display, pixmap, format, attr_mask, &raw mut attr);
        }
      else
	{ image_error (c"Specified image bit depth is not supported by XRender".as_ptr ()); }
    }

  return p;
}

/* Return true if XIMG's size WIDTH x HEIGHT doesn't break the
   windowing system.
   WIDTH and HEIGHT must both be positive.
   If XIMG is null, assume it is a bitmap.  */

/// C: C 4064-4065: `static bool image_check_image_size (Emacs_Pix_Container ximg, int width, int height)`
#[cfg (feature = "have_window_system")]
unsafe fn image_check_image_size (ximg: Emacs_Pix_Container, width: c_int, height: c_int) -> bool
{
  /* C 4067-4068: `#if (defined HAVE_X_WINDOWS || defined HAVE_ANDROID) && !defined USE_CAIRO`.
     The unit's `Emacs_Pix_Container` is the untyped pointer C's X-arm typedef is one word wide as
     (its own declaration above), so C's cast-free call carries one here.  */
  #[cfg (all (feature = "have_window_system", any (feature = "have_x_windows", feature = "have_android"),
	      not (feature = "use_cairo")))]
  { return x_check_image_size (ximg as *mut XImage, width, height); }
  /* C 4069-4072: the `#else` arm - C's own FIXME for the NS and NTGUI cases.  */
  #[cfg (all (feature = "have_window_system",
	      not (all (any (feature = "have_x_windows", feature = "have_android"),
			not (feature = "use_cairo")))))]
  {
    /* FIXME: Implement this check for the HAVE_NS and HAVE_NTGUI cases.
       For now, assume that every image size is allowed on these systems.  */
    return true;
  }
}

/* Create an Emacs_Pix_Container and a pixmap of size WIDTH x
   HEIGHT for use on frame F.  Set *PIMG and *PIXMAP to the
   Emacs_Pix_Container and Emacs_Pixmap created.  Set (*PIMG)->data
   to a raster of WIDTH x HEIGHT pixels allocated via xmalloc.  Print
   error messages via image_error if an error occurs.  Value is true
   if successful.

   On W32, a DEPTH of zero signifies a 24 bit image, otherwise DEPTH
   should indicate the bit depth of the image.  */

/// C: C 4086-4089: `static bool image_create_x_image_and_pixmap_1 (struct frame *f, int width, int height, int depth, Emacs_Pix_Container *pimg, Emacs_Pixmap *pixmap, Picture *picture)`
#[cfg (feature = "have_window_system")]
unsafe fn image_create_x_image_and_pixmap_1 (f: *mut frame, width: c_int, height: c_int,
					    mut depth: c_int, pimg: *mut Emacs_Pix_Container,
					    pixmap: *mut Emacs_Pixmap, picture: *mut Picture)
  -> bool
{
  /* C 4091-4104: `#ifdef USE_CAIRO`.  */
  #[cfg (feature = "use_cairo")]
  {
    eassert! (input_blocked_p ());

    /* Allocate a pixmap of the same size.  */
    *pixmap = image_create_pix_container (width as c_uint, height as c_uint, depth as c_uint);
    if *pixmap == NO_PIXMAP! () as Emacs_Pixmap
      {
	*pimg = core::ptr::null_mut ();
	image_error (c"Unable to create X pixmap".as_ptr ());
	return false;
      }

    *pimg = *pixmap;
    return true;
  }
  /* C 4105-4114: `#elif defined HAVE_X_WINDOWS || defined HAVE_ANDROID`.  */
  #[cfg (all (not (feature = "use_cairo"), any (feature = "have_x_windows", feature = "have_android")))]
  {
    if !x_create_x_image_and_pixmap (f, width, height, depth,
				     pimg as *mut *mut XImage, pixmap as *mut Pixmap)
      { return false; }
    /* C 4108-4111: the nested `#ifdef HAVE_XRENDER`.  */
    #[cfg (feature = "have_xrender")]
    if !picture.is_null ()
      { *picture = x_create_xrender_picture (f, *pixmap as Emacs_Pixmap, depth); }

    return true;
  }

  /* C 4116-4138: `#ifdef HAVE_HAIKU` (a second `#if`, not an `#elif`).  */
  #[cfg (feature = "have_haiku")]
  {
    if depth == 0
      { depth = 24; }

    if depth != 24 && depth != 1
      {
	*pimg = core::ptr::null_mut ();
	image_error (c"Invalid image bit depth specified".as_ptr ());
	return false;
      }

    *pixmap = BBitmap_new (width, height, depth == 1);

    if *pixmap == NO_PIXMAP! () as Emacs_Pixmap
      {
	*pimg = core::ptr::null_mut ();
	image_error (c"Unable to create pixmap".as_ptr ());
	return false;
      }

    *pimg = *pixmap;
    return true;
  }

  /* C 4140-4228: `#ifdef HAVE_NTGUI`.  The Windows arm is text-only on this machine (no
     `x86_64-w64-mingw32-gcc`), so C's tokens stay and the ledger records the names.  */
  #[cfg (feature = "have_ntgui")]
  {
    let mut header: *mut BITMAPINFOHEADER;
    let mut hdc: HDC;
    let mut scanline_width_bits: c_int;
    let mut remainder: c_int;
    let mut palette_colors: c_int = 0;

    if depth == 0
      { depth = 24; }

    if depth != 1 && depth != 4 && depth != 8
	&& depth != 16 && depth != 24 && depth != 32
      {
	image_error (c"Invalid image bit depth specified".as_ptr ());
	return false;
      }

    scanline_width_bits = width * depth;
    remainder = scanline_width_bits % 32;

    if remainder != 0
      { scanline_width_bits += 32 - remainder; }

    /* Bitmaps with a depth less than 16 need a palette.  */
    /* BITMAPINFO structure already contains the first RGBQUAD.  */
    if depth < 16
      { palette_colors = 1 << (depth - 1); }

    *pimg = xmalloc (core::mem::size_of::<XImage> ()
		     + (palette_colors as usize) * core::mem::size_of::<RGBQUAD> ())
	      as Emacs_Pix_Container;

    header = &raw mut (**pimg).info.bmiHeader;
    memset (&raw mut (**pimg).info as *mut c_void, 0, core::mem::size_of::<BITMAPINFO> ());
    (*header).biSize = core::mem::size_of::<BITMAPINFOHEADER> () as u32;
    (*header).biWidth = width;
    (*header).biHeight = -height;  /* negative indicates a top-down bitmap.  */
    (*header).biPlanes = 1;
    (*header).biBitCount = depth;
    (*header).biCompression = BI_RGB;
    (*header).biClrUsed = palette_colors as u32;

    /* TODO: fill in palette.  */
    if depth == 1
      {
	(**pimg).info.bmiColors[0].rgbBlue = 0;
	(**pimg).info.bmiColors[0].rgbGreen = 0;
	(**pimg).info.bmiColors[0].rgbRed = 0;
	(**pimg).info.bmiColors[0].rgbReserved = 0;
	/* bmiColors is a variable-length array declared by w32api
	   headers as bmiColors[1], which triggers a warning under
	   -Warray-bounds; shut that up.  */
	/* C 4191-4194 and C 4199-4201: the `GNUC_PREREQ (4, 4, 0)` pragma pair - C text this
	   port carries as C's own comment (PORTING.md: a pragma is not a statement).  */
	(**pimg).info.bmiColors[1].rgbBlue = 255;
	(**pimg).info.bmiColors[1].rgbGreen = 255;
	(**pimg).info.bmiColors[1].rgbRed = 255;
	(**pimg).info.bmiColors[1].rgbReserved = 0;
      }

    hdc = get_frame_dc (f);

    /* Create a DIBSection and raster array for the bitmap,
       and store its handle in *pixmap.  */
    *pixmap = CreateDIBSection (hdc, &raw mut (**pimg).info,
				if depth < 16 { DIB_PAL_COLORS } else { DIB_RGB_COLORS },
				/* casting avoids a GCC warning */
				&raw mut (**pimg).data as *mut *mut c_void,
				core::ptr::null_mut (), 0) as Emacs_Pixmap;

    /* Realize display palette and garbage all frames. */
    release_frame_dc (f, hdc);

    if *pixmap == core::ptr::null_mut ()
      {
	let err: DWORD = GetLastError ();
	/* All system errors are < 10000, so the following is safe.  */
	image_error (c"Unable to create bitmap, error code %d".as_ptr (),
		     make_fixnum (err as EMACS_INT));
	image_destroy_x_image (*pimg);
	*pimg = core::ptr::null_mut ();
	return false;
      }

    return true;
  }

  /* C 4230-4240: `#ifdef HAVE_NS`.  */
  #[cfg (feature = "have_ns")]
  {
    *pixmap = ns_image_for_XPM (width, height, depth);
    if (*pixmap).is_null ()
      {
	*pimg = core::ptr::null_mut ();
	image_error (c"Unable to allocate NSImage for XPM pixmap".as_ptr ());
	return false;
      }
    *pimg = *pixmap;
    return true;
  }

  /* C 4086-4241 has no statement after its last arm: with none of the five compiled the function
     falls off its end, which C reports as a warning and Rust as a type error (`E0308`).  A
     window-system configuration always selects one of them (configure.ac 7521 fills
     `WINDOW_SYSTEM_OBJ` from the same test that picks the platform), so this arm is unreachable in
     every configuration that exists - the `image_create_bitmap_from_file` shape above.  */
  #[cfg (not (any (feature = "use_cairo",
		  any (feature = "have_x_windows", feature = "have_android"),
		  feature = "have_haiku", feature = "have_ntgui", feature = "have_ns")))]
  unreachable! ()
}


/* Destroy Emacs_Pix_Container PIMG.  Free data associated with PIMG.  */

/// C: C 4246-4247: `static void image_destroy_x_image (Emacs_Pix_Container pimg)`
#[cfg (feature = "have_window_system")]
unsafe fn image_destroy_x_image (pimg: Emacs_Pix_Container)
{
  /* C 4249-4250: `#if (defined HAVE_X_WINDOWS || defined HAVE_ANDROID) && !defined USE_CAIRO`.  */
  #[cfg (all (any (feature = "have_x_windows", feature = "have_android"), not (feature = "use_cairo")))]
  x_destroy_x_image (pimg as *mut XImage);

  /* C 4251-4266: the `#else` arm.  */
  #[cfg (not (all (any (feature = "have_x_windows", feature = "have_android"),
		   not (feature = "use_cairo"))))]
  {
    eassert! (input_blocked_p ());
    if !pimg.is_null ()
      {
	/* C 4255-4259: `#if defined USE_CAIRO || defined HAVE_HAIKU || defined HAVE_NS` - the
	   block holds C's comment and no statement.  */
	#[cfg (feature = "have_ntgui")]
	{
	  /* Data will be freed by DestroyObject.  */
	  (**pimg).data = core::ptr::null_mut ();
	  xfree (pimg);
	}
      }
  }
}


/* Put Emacs_Pix_Container PIMG into pixmap PIXMAP on frame F.
   WIDTH and HEIGHT are width and height of both the image and
   pixmap.  */

/// C: C 4274-4276: `static void gui_put_x_image (struct frame *f, Emacs_Pix_Container pimg, Emacs_Pixmap pixmap, int width, int height)`
#[cfg (feature = "have_window_system")]
unsafe fn gui_put_x_image (f: *mut frame, pimg: Emacs_Pix_Container, pixmap: Emacs_Pixmap,
			   width: c_int, height: c_int)
{
  /* C 4278-4279: `#if defined USE_CAIRO || defined HAVE_HAIKU || defined HAVE_NS`.  */
  #[cfg (any (feature = "use_cairo", feature = "have_haiku", feature = "have_ns"))]
  eassert! (pimg as Emacs_Pixmap == pixmap);
  /* C 4280-4287: `#elif defined HAVE_X_WINDOWS`.  */
  #[cfg (all (not (any (feature = "use_cairo", feature = "have_haiku", feature = "have_ns")),
	      feature = "have_x_windows"))]
  {
    let mut gc: GC;

    eassert! (input_blocked_p ());
    gc = XCreateGC (FRAME_X_DISPLAY! (f), pixmap, 0, core::ptr::null_mut ());
    XPutImage (FRAME_X_DISPLAY! (f), pixmap, gc, pimg as *mut XImage, 0, 0, 0, 0,
	       (*(pimg as *mut XImage)).width, (*(pimg as *mut XImage)).height);
    XFreeGC (FRAME_X_DISPLAY! (f), gc);
  }
  /* C 4288-4289: `#elif defined HAVE_ANDROID`.  */
  #[cfg (all (not (any (feature = "use_cairo", feature = "have_haiku", feature = "have_ns")),
	      not (feature = "have_x_windows"), feature = "have_android"))]
  android_put_image (pixmap as android_pixmap, pimg as *mut android_image);
}

/* Thin wrapper for image_create_x_image_and_pixmap_1, so that it matches
   with image_put_x_image.  */

/// C: C 4296-4299: `static bool image_create_x_image_and_pixmap (struct frame *f, struct image *img, int width, int height, int depth, Emacs_Pix_Container *ximg, bool mask_p)`
#[cfg (feature = "have_window_system")]
unsafe fn image_create_x_image_and_pixmap (f: *mut frame, img: *mut image, width: c_int,
					   height: c_int, depth: c_int,
					   ximg: *mut Emacs_Pix_Container, mask_p: bool) -> bool
{
  /* C's `(!mask_p ? img->pixmap : img->mask) == NO_PIXMAP` - the record's members are the
     untyped pointers the mirror carries (P-72 rule 3), so each read names `Emacs_Pixmap`.  */
  eassert! ((if !mask_p { (*img).pixmap } else { (*img).mask }) as Emacs_Pixmap
	    == NO_PIXMAP! () as Emacs_Pixmap);

  let mut picture: *mut Picture = core::ptr::null_mut ();
  /* C 4304-4306: `#if !defined USE_CAIRO && defined HAVE_XRENDER`.  */
  #[cfg (all (not (feature = "use_cairo"), feature = "have_xrender"))]
  {
    picture = if !mask_p { &raw mut (*img).picture } else { &raw mut (*img).mask_picture };
  }
  return image_create_x_image_and_pixmap_1 (f, width, height, depth, ximg,
					    (if !mask_p { &raw mut (*img).pixmap }
					     else { &raw mut (*img).mask }) as *mut Emacs_Pixmap,
					    picture);
}

/* Put pixel image PIMG into image IMG on frame F, as a mask if and only
   if MASK_P.  On X, this simply records PIMG on a member of IMG, so
   it can be put into the pixmap afterwards via image_sync_to_pixmaps.
   On the other platforms, it puts PIMG into the pixmap, then frees
   the pixel image and its buffer.  */

/// C: C 4318-4320: `static void image_put_x_image (struct frame *f, struct image *img, Emacs_Pix_Container ximg, bool mask_p)`
#[cfg (feature = "have_window_system")]
unsafe fn image_put_x_image (f: *mut frame, img: *mut image, ximg: Emacs_Pix_Container,
			     mask_p: bool)
{
  /* C 4322-4332: `#if (defined HAVE_X_WINDOWS || defined HAVE_ANDROID) && !defined USE_CAIRO`.  */
  #[cfg (all (any (feature = "have_x_windows", feature = "have_android"), not (feature = "use_cairo")))]
  {
    if !mask_p
      {
	eassert! ((*img).ximg.is_null ());
	(*img).ximg = ximg as *mut c_void;
      }
    else
      {
	eassert! ((*img).mask_img.is_null ());
	(*img).mask_img = ximg as *mut c_void;
      }
  }
  /* C 4333-4336: the `#else` arm.  */
  #[cfg (not (all (any (feature = "have_x_windows", feature = "have_android"),
		   not (feature = "use_cairo"))))]
  {
    gui_put_x_image (f, ximg, (if !mask_p { (*img).pixmap } else { (*img).mask }) as Emacs_Pixmap,
		     (*img).width, (*img).height);
    image_destroy_x_image (ximg);
  }
}

/* C 4340-4360: `#if (defined HAVE_X_WINDOWS || defined HAVE_ANDROID) && !defined USE_CAIRO`.  */
/* Put the X images recorded in IMG on frame F into pixmaps, then free
   the X images and their buffers.  */

/// C: C 4344-4345: `static void image_sync_to_pixmaps (struct frame *f, struct image *img)`
#[cfg (all (feature = "have_window_system", any (feature = "have_x_windows", feature = "have_android"),
	    not (feature = "use_cairo")))]
unsafe fn image_sync_to_pixmaps (f: *mut frame, img: *mut image)
{
  if !(*img).ximg.is_null ()
    {
      gui_put_x_image (f, (*img).ximg as Emacs_Pix_Container, (*img).pixmap as Emacs_Pixmap,
		       (*img).width, (*img).height);
      image_destroy_x_image ((*img).ximg as Emacs_Pix_Container);
      (*img).ximg = core::ptr::null_mut ();
    }
  if !(*img).mask_img.is_null ()
    {
      gui_put_x_image (f, (*img).mask_img as Emacs_Pix_Container, (*img).mask as Emacs_Pixmap,
		       (*img).width, (*img).height);
      image_destroy_x_image ((*img).mask_img as Emacs_Pix_Container);
      (*img).mask_img = core::ptr::null_mut ();
    }
}

/* C 4362-4386: `#ifdef HAVE_NTGUI`.  */
/* Create a memory device context for IMG on frame F.  It stores the
   currently selected GDI object into *PREV for future restoration by
   image_unget_x_image_or_dc.  */

/// C: C 4367-4369: `static HDC image_get_x_image_or_dc (struct frame *f, struct image *img, bool mask_p, HGDIOBJ *prev)`
#[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
unsafe fn image_get_x_image_or_dc (f: *mut frame, img: *mut image, mask_p: bool,
				   prev: *mut HGDIOBJ) -> HDC
{
  let frame_dc: HDC = get_frame_dc (f);
  let ximg: HDC = CreateCompatibleDC (frame_dc);

  release_frame_dc (f, frame_dc);
  *prev = SelectObject (ximg, if !mask_p { (*img).pixmap } else { (*img).mask } as HGDIOBJ);

  return ximg;
}

/// C: C 4380-4382: `static void image_unget_x_image_or_dc (struct image *img, bool mask_p, HDC ximg, HGDIOBJ prev)`
#[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
unsafe fn image_unget_x_image_or_dc (img: *mut image, mask_p: bool, ximg: HDC, prev: HGDIOBJ)
{
  SelectObject (ximg, prev);
  DeleteDC (ximg);
}

/* C 4387-4442: `#else  /* !HAVE_NTGUI */`.  */
/* Get the X image for IMG on frame F.  The resulting X image data
   should be treated as read-only at least on X.  */

/// C: C 4391-4392: `static Emacs_Pix_Container image_get_x_image (struct frame *f, struct image *img, bool mask_p)`
#[cfg (all (feature = "have_window_system", not (feature = "have_ntgui")))]
unsafe fn image_get_x_image (f: *mut frame, img: *mut image, mask_p: bool)
  -> Emacs_Pix_Container
{
  /* C 4394-4395: `#if defined USE_CAIRO || defined (HAVE_HAIKU)`.  */
  #[cfg (any (feature = "use_cairo", feature = "have_haiku"))]
  { return (if !mask_p { (*img).pixmap } else { (*img).mask }) as Emacs_Pix_Container; }
  /* C 4396-4414: `#elif defined HAVE_X_WINDOWS || defined HAVE_ANDROID`.  */
  #[cfg (all (not (any (feature = "use_cairo", feature = "have_haiku")),
	      any (feature = "have_x_windows", feature = "have_android")))]
  {
    let ximg_in_img: *mut XImage
      = (if !mask_p { (*img).ximg } else { (*img).mask_img }) as *mut XImage;

    if !ximg_in_img.is_null ()
      { return ximg_in_img as Emacs_Pix_Container; }
    /* C 4401-4405: `#ifdef HAVE_XRENDER`.  Every arm of C's chain returns, so C's `else if` is
       this early-return sequence - the same order, the same tests, the same values.  */
    #[cfg (feature = "have_xrender")]
    if !(*img).picture.is_null ()
      { return XGetImage (FRAME_X_DISPLAY! (f),
			  (if !mask_p { (*img).pixmap } else { (*img).mask }) as Pixmap,
			  0, 0, (*img).original_width as c_uint, (*img).original_height as c_uint,
			  !0i32 as c_ulong, ZPixmap) as Emacs_Pix_Container; }
    /* C 4406-4409: `#ifndef HAVE_ANDROID`.  */
    #[cfg (not (feature = "have_android"))]
    return XGetImage (FRAME_X_DISPLAY! (f),
		      (if !mask_p { (*img).pixmap } else { (*img).mask }) as Pixmap,
		      0, 0, (*img).width as c_uint, (*img).height as c_uint,
		      !0i32 as c_ulong, ZPixmap) as Emacs_Pix_Container;
    /* C 4410-4414: `#else`.  */
    #[cfg (feature = "have_android")]
    return android_get_image ((if !mask_p { (*img).pixmap } else { (*img).mask }) as android_drawable,
			      ANDROID_Z_PIXMAP) as Emacs_Pix_Container;
  }
  /* C 4415-4420: `#elif defined (HAVE_NS)`.  */
  #[cfg (all (not (any (feature = "use_cairo", feature = "have_haiku")),
	      not (any (feature = "have_x_windows", feature = "have_android")),
	      feature = "have_ns"))]
  {
    let pixmap: Emacs_Pix_Container
      = (if !mask_p { (*img).pixmap } else { (*img).mask }) as Emacs_Pix_Container;

    ns_retain_object (pixmap as *mut c_void);
    return pixmap;
  }
}

/// C: C 4423-4424: `static void image_unget_x_image (struct image *img, bool mask_p, Emacs_Pix_Container ximg)`
#[cfg (all (feature = "have_window_system", not (feature = "have_ntgui")))]
unsafe fn image_unget_x_image (img: *mut image, mask_p: bool, ximg: Emacs_Pix_Container)
{
  /* C 4426: `#ifdef USE_CAIRO` - C's arm holds no statement (the empty side of the `#elif`), so
     the port carries the line and nothing else; `docs/ports/image.md`'s Deviations records it.  */

  /* C 4427-4438: `#elif defined HAVE_X_WINDOWS || defined HAVE_ANDROID`.  */
  #[cfg (all (not (feature = "use_cairo"), any (feature = "have_x_windows", feature = "have_android")))]
  {
    let ximg_in_img: *mut XImage
      = (if !mask_p { (*img).ximg } else { (*img).mask_img }) as *mut XImage;

    if !ximg_in_img.is_null ()
      { eassert! (ximg as *mut XImage == ximg_in_img); }
    else
      {
	/* C 4432-4438: `#ifdef HAVE_ANDROID` / `#else`.  */
	#[cfg (feature = "have_android")]
	{ android_destroy_image (ximg as *mut android_image); }
	#[cfg (not (feature = "have_android"))]
	{ XDestroyImage (ximg as *mut XImage); }
      }
  }
  /* C 4439-4440: `#elif defined (HAVE_NS)`.  */
  #[cfg (all (not (feature = "use_cairo"), not (any (feature = "have_x_windows", feature = "have_android")),
	      feature = "have_ns"))]
  ns_release_object (ximg as *mut c_void);
}


/***********************************************************************
			      File Handling
 ***********************************************************************/

/* Find image file FILE.  Look in data-directory/images, then
   x-bitmap-file-path.  Value is the full name of the file
   found, or nil if not found.  If PFD is nonnull store into *PFD a
   readable file descriptor for the file, opened in binary mode.  If
   PFD is null, do not open the file.  */

/* The names the mirrors this unit includes do not carry, declared at the use site with C's own
   line (PORTING.md P-55/P-72 rule 3); `src/charset.rs` 91-94 and `src/fileio.rs` 77-78 declare
   the same three.  */
// C: lisp.h 4977: `extern void fclose_unwind (void *);`
// C: lisp.h 5320: `extern FILE *emacs_fdopen (int, const char *)`
// C: lisp.h 5305: `extern int sys_fstat (int, struct stat *);`
#[cfg (feature = "have_window_system")]
unsafe extern "C" {
  pub fn fclose_unwind (arg: *mut c_void);
  pub fn emacs_fdopen (fd: c_int, mode: *const c_char) -> *mut FILE;
  pub fn sys_fstat (fd: c_int, buf: *mut stat) -> c_int;
}

/// C: C 4456-4457: `static Lisp_Object image_find_image_fd (Lisp_Object file, image_fd *pfd)`
#[cfg (feature = "have_window_system")]
unsafe fn image_find_image_fd (file: Lisp_Object, pfd: *mut image_fd) -> Lisp_Object
{
  /* C's `Lisp_Object file_found, search_path;` - `openp` writes `*storeptr`, and `&raw mut` of a
     binding Rust has not initialised is `E0381`, so both take C's zero (`Qnil`; PORTING.md
     1.5/0.1-13).  */
  let mut file_found: Lisp_Object = Qnil ();
  let mut search_path: Lisp_Object;
  let mut fd: c_int;
  let mut platform: *mut c_void;

  /* TODO I think this should use something like image-load-path
     instead.  Unfortunately, that can contain non-string elements.  */
  search_path = Fcons (Fexpand_file_name (build_string (c"images".as_ptr ()), Vdata_directory),
		       Vx_bitmap_file_path);

  /* Try to find FILE in data-directory/images, then x-bitmap-file-path.  */
  platform = core::ptr::null_mut ();
  fd = openp (search_path, file, Qnil (), &raw mut file_found,
	      if !pfd.is_null () { Qt () } else { make_fixnum (R_OK as EMACS_INT) }, false, false,
	      if !pfd.is_null () { &raw mut platform } else { core::ptr::null_mut () });
  if fd == -2
    {
      /* The file exists locally, but has a file name handler.
	 (This happens, e.g., under Auto Image File Mode.)
	 'openp' didn't open the file, so we should, because the
	 caller expects that.  */
      let encoded_name: Lisp_Object = ENCODE_FILE (file_found);
      fd = emacs_open (SSDATA (encoded_name), O_RDONLY, 0);
    }
  /* FD is -3 if PLATFORM is set to a valid asset file descriptor on
     Android.  */
  else if fd < 0 && fd != -3
    { return Qnil (); }

  /* C 4488-4499: `#if !defined HAVE_ANDROID || defined ANDROID_STUBIFY` / `#else`.  */
  #[cfg (not (all (feature = "have_android", not (feature = "android_stubify"))))]
  if !pfd.is_null ()
    { *pfd = fd; }
  #[cfg (all (feature = "have_android", not (feature = "android_stubify")))]
  if !pfd.is_null ()
    {
      /* Construct an asset file descriptor.  */
      (*pfd).fd = fd;
      (*pfd).asset = platform;
    }
  return file_found;
}

/* Find image file FILE.  Look in data-directory/images, then
   x-bitmap-file-path.  Value is the full name of the file found, or
   nil if not found.  */

/// C: C 4507-4508: `Lisp_Object image_find_image_file (Lisp_Object file)` - non-static.
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn image_find_image_file (file: Lisp_Object) -> Lisp_Object
{
  return image_find_image_fd (file, core::ptr::null_mut ());
}

/* C 4513-4521: `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY`.  */

/// C: C 4515-4516: `static void close_android_fd (void *ptr)`
#[cfg (all (feature = "have_window_system", feature = "have_android",
	    not (feature = "android_stubify")))]
unsafe fn close_android_fd (ptr: *mut c_void)
{
  android_close_asset (*(ptr as *mut android_fd_or_asset));
}

/* Read FILE into memory.  Value is a pointer to a buffer allocated
   with xmalloc holding FILE's contents.  Value is null if an error
   occurred.  FD is a file descriptor open for reading FILE.  Set
   *SIZE to the size of the file.  */

/// C: C 4528-4529: `static char * slurp_file (image_fd fd, ptrdiff_t *size)`
#[cfg (feature = "have_window_system")]
unsafe fn slurp_file (fd: image_fd, size: *mut ptrdiff_t) -> *mut c_char
{
  /* C declares `buf` inside both arms below (`char *buf = NULL;` in the first, the uninitialised
     `char *buf;` in the second) and reads it once, after the `#endif`; Rust's `let` is
     block-scoped, so the declaration is hoisted to the function and each arm keeps its own
     initialisation - the second arm's bare declaration is the port's zero (PORTING.md 1.5/0.1-13,
     and C's `buf = NULL;` in its `else` already covers every path that leaves it set).  */
  let mut buf: *mut c_char = core::ptr::null_mut ();

  /* C 4531-4559: `#if !defined HAVE_ANDROID || defined ANDROID_STUBIFY`.  */
  #[cfg (not (all (feature = "have_android", not (feature = "android_stubify"))))]
  {
    let fp: *mut FILE = emacs_fdopen (fd, c"rb".as_ptr ());
    let mut st: stat = core::mem::zeroed ();

    if !fp.is_null ()
      {
	let count: specpdl_ref = SPECPDL_INDEX ();
	record_unwind_protect_ptr (Some (fclose_unwind), fp as *mut c_void);

	if sys_fstat (fileno (fp), &raw mut st) == 0
	    && 0 <= st.st_size
	    && (st.st_size as u64) < std::cmp::min (PTRDIFF_MAX as u64, SIZE_MAX as u64)
	  {
	    /* Report an error if we read past the purported EOF.
	       This can happen if the file grows as we read it.  */
	    let buflen: ptrdiff_t = st.st_size as ptrdiff_t;
	    buf = xmalloc ((buflen + 1) as usize) as *mut c_char;
	    if fread (buf as *mut c_void, 1, (buflen + 1) as usize, fp) == buflen as usize
	      { *size = buflen; }
	    else
	      {
		xfree (buf as *mut c_void);
		buf = core::ptr::null_mut ();
	      }
	  }

	unbind_to (count, Qnil ());
      }
  }
  /* C 4560-4592: the `#else` arm (Android without stubs).  */
  #[cfg (all (feature = "have_android", not (feature = "android_stubify")))]
  {
    let mut st: stat = core::mem::zeroed ();
    let mut count: specpdl_ref;

    if android_asset_fstat (fd, &raw mut st) == 0
	&& (0 <= st.st_size
	    && (st.st_size as u64) < std::cmp::min (PTRDIFF_MAX as u64, SIZE_MAX as u64))
      {
	count = SPECPDL_INDEX ();
	record_unwind_protect_ptr (Some (close_android_fd), &raw mut fd as *mut c_void);
	buf = xmalloc ((st.st_size + 1) as usize) as *mut c_char;

	/* Read one byte past the end of the file.  That allows
	   detecting if the file grows as it is being read.  */

	if android_asset_read (fd, buf as *mut c_void, (st.st_size + 1) as size_t)
	    == st.st_size as ssize_t
	  { *size = st.st_size as ptrdiff_t; }
	else
	  {
	    xfree (buf as *mut c_void);
	    buf = core::ptr::null_mut ();
	  }

	unbind_to (count, Qnil ());
      }
    else
      {
	buf = core::ptr::null_mut ();
	android_close_asset (fd);
      }
  }

  return buf;
}

/* Like slurp_file above, but with added error handling.  Value is
   null if an error occurred.  Set SIZE to the size of the file.
   IMAGE_TYPE describes the image type (e.g. "PNG").  */

/// C: C 4601-4602: `static char * slurp_image (Lisp_Object filename, ptrdiff_t *size, const char *image_type)`
#[cfg (feature = "have_window_system")]
unsafe fn slurp_image (filename: Lisp_Object, size: *mut ptrdiff_t, image_type: *const c_char)
  -> *mut c_char
{
  let mut fd: image_fd = core::mem::zeroed ();
  let file: Lisp_Object = image_find_image_fd (filename, &raw mut fd);
  if !STRINGP (file)
    {
      image_not_found_error (filename);
      return core::ptr::null_mut ();
    }
  let result: *mut c_char = slurp_file (fd, size);
  if result.is_null ()
    { image_error (c"Error loading %s image `%s'".as_ptr (),
		   build_unibyte_string (image_type), file); }
  return result;
}


/* ----------------------------------------------------------------------------------------------
   C 4620-6161: the file's fifth slice.

   It holds the "XBM images" banner (C 4620-4622), C 4624's `static bool xbm_file_p (Lisp_Object);`
   prototype (dropped: PORTING.md 0.1-12 - its definition is C 5301, inside this slice), the
   `xbm_keyword_index`/`xbm_token` enums and the `xbm_format` table (C 4629-4679), `xbm_image_p`
   (C 4710-4797), `xbm_scan` (C 4805-4934), the `HAVE_NTGUI` pair
   `w32_create_pixmap_from_bitmap_data` (C 4938-4969) and `convert_mono_to_color_image`
   (C 4971-5001), `Create_Pixmap_From_Bitmap_Data` (C 5005-5066), `xbm_read_bitmap_data`
   (C 5076-5216), `xbm_load_image` (C 5238-5295), `xbm_file_p` (C 5300-5308) and `xbm_load`
   (C 5313-5451); the "Canvas" banner (C 5453-5456), the `canvas_keyword_index` enum and
   `canvas_format` table (C 5460-5492), the `canvas_map`/`canvas_list` objects (C 5502-5509),
   `canvas_parse` (C 5520-5520 + C 5522-5530), `canvas_image_p` (C 5525-5529), `canvas_free_unused`
   (C 5533-5558), `canvas_apply_data` (C 5566-5659), `canvas_get` (C 5662-5710), `canvas_load`
   (C 5713-5737), `canvas_prepare_for_display` (C 5740-5849), `canvas_data` (C 5853-5864) and
   `DEFUN ("canvas-refresh")` (C 5866-5906); and the XPM section's head (C 5909-6161): the banner,
   C 5913-5919's prototype pair (dropped: both definitions are C 6635/6704's region, outside this
   slice), C 5921-5942's `<X11/xpm.h>` (the P-88 declaration section), C 5944-5984's
   `xpm_keyword_index`/`xpm_format`, C 5986-5996's `ALLOC_XPM_COLORS` derivation and C 5998-6161's
   colour cache (`xpm_cached_color` and the seven functions).

   The per-`#if` verdicts, the C line-range rows and this round's readings are in
   `docs/ports/image.md` (`## C line ranges`, `## Evidence`, `## Deviations`).  Names the mirrors
   this unit includes do not carry stand at the use site with C's own prototype in a `/// C:` line
   (PORTING.md P-55/P-72 rule 3); names the platform SDKs supply (`<X11/Xlib.h>`, `<X11/xpm.h>`,
   Xrender, cairo, `<windows.h>`, Haiku's `<Bitmap.h>`, the NDK) keep C's tokens or, where the
   header has no file in the reference tree, carry the P-88 section.
   ---------------------------------------------------------------------------------------------- */

/***********************************************************************
			      XBM images
 ***********************************************************************/

/* C 4624: `static bool xbm_file_p (Lisp_Object);` - a `static` prototype whose definition is
   C 5301's, *inside* this slice, so C's prototype-plus-definition pair in one translation unit is
   dropped (PORTING.md 0.1-12) and the definition keeps C's place.  */

/* Indices of image specification fields in xbm_format, below.  */

/* C 4629-4647: `enum xbm_keyword_index` - the port's `c_int` type plus one constant per member
   (`image_value_type`'s C 1461-1477 shape).  */
/// C: C 4629-4647: `enum xbm_keyword_index`
#[cfg (feature = "have_window_system")]
pub const XBM_TYPE: c_int = 0;
#[cfg (feature = "have_window_system")]
pub const XBM_FILE: c_int = 1;
#[cfg (feature = "have_window_system")]
pub const XBM_WIDTH: c_int = 2;
#[cfg (feature = "have_window_system")]
pub const XBM_HEIGHT: c_int = 3;
#[cfg (feature = "have_window_system")]
pub const XBM_STRIDE: c_int = 4;
#[cfg (feature = "have_window_system")]
pub const XBM_DATA: c_int = 5;
#[cfg (feature = "have_window_system")]
pub const XBM_FOREGROUND: c_int = 6;
#[cfg (feature = "have_window_system")]
pub const XBM_BACKGROUND: c_int = 7;
#[cfg (feature = "have_window_system")]
pub const XBM_ASCENT: c_int = 8;
#[cfg (feature = "have_window_system")]
pub const XBM_MARGIN: c_int = 9;
#[cfg (feature = "have_window_system")]
pub const XBM_RELIEF: c_int = 10;
#[cfg (feature = "have_window_system")]
pub const XBM_ALGORITHM: c_int = 11;
#[cfg (feature = "have_window_system")]
pub const XBM_HEURISTIC_MASK: c_int = 12;
#[cfg (feature = "have_window_system")]
pub const XBM_MASK: c_int = 13;
#[cfg (feature = "have_window_system")]
pub const XBM_DATA_WIDTH: c_int = 14;
#[cfg (feature = "have_window_system")]
pub const XBM_DATA_HEIGHT: c_int = 15;
#[cfg (feature = "have_window_system")]
pub const XBM_LAST: c_int = 16;

/* Rust requires an immutable `static`'s type to be `Sync`, and a raw pointer is not; C's
   `struct image_keyword` is immutable data (every `name` points at a string literal in this
   unit), so the port states that with an explicit impl - the same reason the three tables below
   stay ordinary `static`s rather than becoming `static mut` (`src/font.rs` 529's `table_entry`
   impl).  */
#[cfg (feature = "have_window_system")]
unsafe impl Sync for image_keyword {}

/* Vector of image_keyword structures describing the format
   of valid XBM image specifications.  */

/* C 4649-4669: the table.  C's initialisers name three of the record's five members, so the
   trailing `count`/`value` are C's implicit zeros written out (PORTING.md 1.5).  */
/// C: C 4652-4653: `static const struct image_keyword xbm_format [XBM_LAST]`
#[cfg (feature = "have_window_system")]
static xbm_format: [image_keyword; XBM_LAST as usize] =
[
  image_keyword { name: c":type".as_ptr (), r#type: IMAGE_SYMBOL_VALUE,
		  mandatory_p: true, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":file".as_ptr (), r#type: IMAGE_STRING_VALUE,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":width".as_ptr (), r#type: IMAGE_POSITIVE_INTEGER_VALUE,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":height".as_ptr (), r#type: IMAGE_POSITIVE_INTEGER_VALUE,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":stride".as_ptr (), r#type: IMAGE_POSITIVE_INTEGER_VALUE,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":data".as_ptr (), r#type: IMAGE_DONT_CHECK_VALUE_TYPE,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":foreground".as_ptr (), r#type: IMAGE_STRING_OR_NIL_VALUE,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":background".as_ptr (), r#type: IMAGE_STRING_OR_NIL_VALUE,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":ascent".as_ptr (), r#type: IMAGE_ASCENT_VALUE,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":margin".as_ptr (), r#type: IMAGE_NON_NEGATIVE_INTEGER_VALUE_OR_PAIR,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":relief".as_ptr (), r#type: IMAGE_INTEGER_VALUE,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":conversion".as_ptr (), r#type: IMAGE_DONT_CHECK_VALUE_TYPE,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":heuristic-mask".as_ptr (), r#type: IMAGE_DONT_CHECK_VALUE_TYPE,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":mask".as_ptr (), r#type: IMAGE_DONT_CHECK_VALUE_TYPE,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":data-width".as_ptr (), r#type: IMAGE_POSITIVE_INTEGER_VALUE,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":data-height".as_ptr (), r#type: IMAGE_POSITIVE_INTEGER_VALUE,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
];

/* Tokens returned from xbm_scan.  */

/* C 4674-4679: `enum xbm_token` - the port's `c_int` constants.  */
/// C: C 4674-4679: `enum xbm_token`
#[cfg (feature = "have_window_system")]
pub const XBM_TK_IDENT: c_int = 256;
#[cfg (feature = "have_window_system")]
pub const XBM_TK_NUMBER: c_int = 257;
#[cfg (feature = "have_window_system")]
pub const XBM_TK_OVERFLOW: c_int = 258;

/* Return true if OBJECT is a valid XBM-type image specification.
   A valid specification is a list starting with the symbol `image'.
   The rest of the list is a property list which must contain an
   entry `:type xbm'.

   If the specification specifies a file to load, it must contain
   an entry `:file FILENAME' where FILENAME is a string.

   If the specification is for a bitmap loaded from memory it must
   contain `:data-width WIDTH', `:data-height HEIGHT', and `:data DATA',
   where WIDTH and HEIGHT are integers > 0.  DATA may be:

   1. a string large enough to hold the bitmap data, i.e. it must
   have a size >= (WIDTH + 7) / 8 * HEIGHT

   2. a bool-vector of size >= WIDTH * HEIGHT

   3. a vector of strings or bool-vectors, one for each line of the
   bitmap.

   4. a string containing an in-memory XBM file.

   Both the file and data forms may contain the additional entries
   `:background COLOR' and `:foreground COLOR'.  If not present,
   the foreground and background of the frame on which the image is
   displayed are used.  */

/// C: C 4710-4711: `static bool xbm_image_p (Lisp_Object object)`
#[cfg (feature = "have_window_system")]
unsafe extern "C" fn xbm_image_p (object: Lisp_Object) -> bool
{
  /* C's `struct image_keyword kw[XBM_LAST];` filled by the `memcpy` below - zeroed first
     (PORTING.md 1.5/0.1-13).  */
  let mut kw: [image_keyword; XBM_LAST as usize] = core::mem::zeroed ();

  /* C's `memcpy (kw, xbm_format, sizeof kw);`.  */
  memcpy (kw.as_mut_ptr () as *mut c_void, xbm_format.as_ptr () as *const c_void,
	  core::mem::size_of::<[image_keyword; XBM_LAST as usize]> ());
  if !parse_image_spec (object, kw.as_mut_ptr (), XBM_LAST, Qxbm ())
    { return false; }

  eassert! (EQ (kw [XBM_TYPE as usize].value, Qxbm ()));

  if kw [XBM_FILE as usize].count
    {
      if kw [XBM_DATA as usize].count
	{ return false; }
    }
  else if ! (kw [XBM_DATA as usize].count && xbm_file_p (kw [XBM_DATA as usize].value))
    /* Not an in-memory XBM file.  */
    {
      let data: Lisp_Object;
      let mut width: c_int;
      let mut height: c_int;
      let stride: c_int;

      /* Entries for `:data-width', `:data-height', and `:data' must be
	 present.  */
      if !kw [XBM_DATA_WIDTH as usize].count
	  || !kw [XBM_DATA_HEIGHT as usize].count
	  || !kw [XBM_DATA as usize].count
	{ return false; }

      data = kw [XBM_DATA as usize].value;
      width = XFIXNAT (kw [XBM_DATA_WIDTH as usize].value) as c_int;
      height = XFIXNAT (kw [XBM_DATA_HEIGHT as usize].value) as c_int;

      if !kw [XBM_STRIDE as usize].count
	{ stride = width; }
      else
	{ stride = XFIXNAT (kw [XBM_STRIDE as usize].value) as c_int; }

      /* Check type of data, and width and height against contents of
	 data.  */
      if VECTORP (data)
	{
	  let mut i: EMACS_INT;

	  /* Number of elements of the vector must be >= height.  */
	  if ASIZE (data) < height as isize
	    { return false; }

	  /* Each string or bool-vector in data must be large enough
	     for one line of the image.  */
	  i = 0;
	  while i < height as EMACS_INT
	    {
	      let elt: Lisp_Object = AREF (data, i as isize);

	      if STRINGP (elt)
		{
		  if SCHARS (elt) < (stride / CHAR_BIT) as isize
		    { return false; }
		}
	      else if BOOL_VECTOR_P (elt)
		{
		  if bool_vector_size (elt) < width as EMACS_INT
		    { return false; }
		}
	      else
		{ return false; }
	      i += 1;
	    }
	}
      else if STRINGP (data)
	{
	  if SCHARS (data) < (stride / CHAR_BIT * height) as isize
	    { return false; }
	}
      else if BOOL_VECTOR_P (data)
	{
	  /* C's `height > 1 && stride != (width + CHAR_BIT - 1) / CHAR_BIT * CHAR_BIT`.  */
	  if height > 1
	      && stride != (width + CHAR_BIT - 1) / CHAR_BIT * CHAR_BIT
	    { return false; }

	  if bool_vector_size (data) / (height as EMACS_INT) < stride as EMACS_INT
	    { return false; }
	}
      else
	{ return false; }
    }

  return true;
}

/* The names this slice calls that the mirrors this unit includes do not carry, declared at the
   use site with C's own line (PORTING.md P-55/P-72 rule 3); `docs/ports/image.md` names the file
   each belongs in, and the same commit that lands a definition deletes its copy here.  */

/* `<stdio.h>`'s `BUFSIZ`: C 4807's `sval_end` and C 5095's `buffer` are its two readers, and
   `lib/include/stdio.rs` does not carry it.  The value is the one this machine's `<stdio.h>`
   measures (clang: **1024**; the `androidselect.rs` 95 shape).  */
/// C: C 4807: `char *sval_end = sval + BUFSIZ;` - the first reader.
#[cfg (feature = "have_window_system")]
pub const BUFSIZ: c_int = 1024;

/* C 231-232's two `static` prototypes (`init_color_table`, `lookup_rgb_color`) and C 234-235's
   pair (`free_color_table`, `colors_in_color_table`, behind C 169-173's derived
   `COLOR_TABLE_SUPPORT`) are **gone**: the slice that landed C 7150-7395 is their definition
   (PORTING.md 0.1-12 / P-91 ④; `docs/ports/image.md` records the deletion).  */

/* The mirror gaps this slice's XPM half reads, each with the file that owns it.  */
// C: xterm.h 391: `extern Status x_parse_color (struct frame *, const char *, XColor *);` - the
// definition is `xterm.c`'s; `include/xterm.rs` does not carry the declaration.  `Status` is
// `<X11/Xlib.h>`'s `int`.
#[cfg (all (feature = "have_window_system", feature = "have_x_windows", not (feature = "use_cairo"),
	    feature = "alloc_xpm_colors"))]
unsafe extern "C" {
  pub fn x_parse_color (f: *mut frame, color_name: *const c_char, color: *mut XColor) -> c_int;
}
// C: lisp.h 4249: `EMACS_UINT hash_char_array (char const *, ptrdiff_t);` - the definition is
// `fns.c` 5349's; `include/lisp.rs` does not carry the declaration.
#[cfg (all (feature = "have_window_system", feature = "have_x_windows", not (feature = "use_cairo"),
	    feature = "alloc_xpm_colors"))]
unsafe extern "C" {
  pub fn hash_char_array (ptr: *const c_char, len: ptrdiff_t) -> EMACS_UINT;
}
// C: dispextern.h 3635: `extern void redraw_image_glyphs (Lisp_Object);` - the definition is
// `xdisp.c`'s; `include/dispextern.rs` does not carry the declaration.
#[cfg (feature = "have_window_system")]
unsafe extern "C" {
  pub fn redraw_image_glyphs (spec: Lisp_Object);
}

/* The three Xlib entry points this slice (and the fourth slice's `x_create_x_image_and_pixmap`)
   call and that `include/xlib.rs` does not carry: Xlib.h 1437-1447's `XCreateImage`, Xlib.h
   1593-1600's `XCreatePixmap` and Xlib.h 1607-1616's `XCreatePixmapFromBitmapData`.  The
   declarations belong in that mirror; the ledger names them.  */
// C: Xlib.h 1437-1447: `extern XImage *XCreateImage (Display *, Visual *, unsigned int, int, int, char *, unsigned int, unsigned int, int, int);`
#[cfg (all (feature = "have_window_system", feature = "have_x_windows"))]
unsafe extern "C" {
  pub fn XCreateImage (display: *mut Display, visual: *mut Visual, depth: c_uint, format: c_int,
		       offset: c_int, data: *mut c_char, width: c_uint, height: c_uint,
		       bitmap_pad: c_int, bytes_per_line: c_int) -> *mut XImage;
}
// C: Xlib.h 1593-1600: `extern Pixmap XCreatePixmap (Display *, Drawable, unsigned int, unsigned int, unsigned int);`
#[cfg (all (feature = "have_window_system", feature = "have_x_windows"))]
unsafe extern "C" {
  pub fn XCreatePixmap (display: *mut Display, d: Drawable, width: c_uint, height: c_uint,
			depth: c_uint) -> Pixmap;
}
// C: Xlib.h 1607-1616: `extern Pixmap XCreatePixmapFromBitmapData (Display *, Drawable, char *, unsigned int, unsigned int, unsigned long, unsigned long, unsigned int);`
#[cfg (all (feature = "have_window_system", feature = "have_x_windows"))]
unsafe extern "C" {
  pub fn XCreatePixmapFromBitmapData (display: *mut Display, d: Drawable, data: *mut c_char,
				      width: c_uint, height: c_uint, fg: c_ulong, bg: c_ulong,
				      depth: c_uint) -> Pixmap;
}

/* Scan a bitmap file.  FP is the stream to read from.  Value is
   either an enumerator from enum xbm_token, or a character for a
   single-character token, or 0 at end of file.  If scanning an
   identifier, store the lexeme of the identifier in SVAL.  If
   scanning a number, store its value in *IVAL.  */

/// C: C 4805-4807: `static int xbm_scan (char **s, char *end, char *sval, int *ival)`
#[cfg (feature = "have_window_system")]
unsafe fn xbm_scan (s: *mut *mut c_char, end: *mut c_char, mut sval: *mut c_char,
		    ival: *mut c_int) -> c_int
{
  /* C's `unsigned char c UNINIT;` - zeroed (PORTING.md 1.5/0.1-13).  */
  let mut c: c_uchar = 0;
  let sval_end: *mut c_char = sval.offset (BUFSIZ as isize);

  /* C's `loop:` label and its `goto loop` are the labelled loop below: the only other exit is
     C's own `return c;` at the end of the body (PORTING.md 2.2 - the `image_compute_scale`
     loop shape).  */
  'loop_: loop
    {
      /* Skip white space.  C's `while (*s < end && (c = *(*s)++, c_isspace (c)))` is the
	 short-circuit `&&` whose second operand is the comma expression (the `frame.rs` 5789
	 shape).  */
      while *s < end
	    && { let ch: c_char = *(*s); *s = (*s).offset (1); c = ch as c_uchar;
		 c_isspace (c as c_int) }
	{ }

      if *s >= end
	{ c = 0; }
      else if c_isdigit (c as c_int)
	{
	  let mut value: c_int = 0;
	  let mut digit: c_int;
	  let mut overflow: bool = false;

	  if c == b'0' as c_uchar && *s < end
	    {
	      c = *(*s) as c_uchar;
	      *s = (*s).offset (1);
	      if c == b'x' as c_uchar || c == b'X' as c_uchar
		{
		  while *s < end
		    {
		      c = *(*s) as c_uchar;
		      *s = (*s).offset (1);
		      digit = char_hexdigit (c as c_int);
		      if digit < 0
			{ break; }
		      /* C's `overflow |= ckd_mul (&value, value, 16);` (P-15: the wrapped product
			 and the flag).  */
		      let (wrapped, over) = value.overflowing_mul (16);
		      value = wrapped;
		      overflow |= over;
		      value += digit;
		    }
		}
	      else if b'0' as c_uchar <= c && c <= b'7' as c_uchar
		{
		  value = c as c_int - b'0' as c_int;
		  while *s < end
			&& { let ch: c_char = *(*s); *s = (*s).offset (1); c = ch as c_uchar;
			     b'0' as c_uchar <= c && c <= b'7' as c_uchar }
		    {
		      let (wrapped, over) = value.overflowing_mul (8);
		      value = wrapped;
		      overflow |= over;
		      value += c as c_int - b'0' as c_int;
		    }
		}
	    }
	  else
	    {
	      value = c as c_int - b'0' as c_int;
	      while *s < end
		    && { let ch: c_char = *(*s); *s = (*s).offset (1); c = ch as c_uchar;
			 c_isdigit (c as c_int) }
		{
		  let (wrapped, over) = value.overflowing_mul (10);
		  value = wrapped;
		  overflow |= over;
		  /* C's `overflow |= ckd_add (&value, value, c - '0');`.  */
		  let (sum, over2) = value.overflowing_add (c as c_int - b'0' as c_int);
		  value = sum;
		  overflow |= over2;
		}
	    }

	  if *s < end
	    { *s = (*s).offset (-1); }
	  *ival = value;
	  return if overflow { XBM_TK_OVERFLOW } else { XBM_TK_NUMBER };
	}
      /* Character literal.  XBM images typically contain hex escape
	 sequences and not actual characters, so we only try to handle
	 that here.  */
      else if c == b'\'' as c_uchar
	{
	  let mut value: c_int = 0;
	  let mut digit: c_int;
	  let mut overflow: bool = false;

	  if *s == end
	    { return 0; }

	  c = *(*s) as c_uchar;
	  *s = (*s).offset (1);

	  if c != b'\\' as c_uchar || *s == end
	    { return 0; }

	  c = *(*s) as c_uchar;
	  *s = (*s).offset (1);

	  if c == b'x' as c_uchar
	    {
	      while *s < end
		{
		  c = *(*s) as c_uchar;
		  *s = (*s).offset (1);

		  if c == b'\'' as c_uchar
		    {
		      *ival = value;
		      return if overflow { XBM_TK_OVERFLOW } else { XBM_TK_NUMBER };
		    }

		  digit = char_hexdigit (c as c_int);

		  if digit < 0
		    { return 0; }

		  let (wrapped, over) = value.overflowing_mul (16);
		  value = wrapped;
		  overflow |= over;
		  value += digit;
		}
	    }

	  return 0;
	}
      else if c_isalpha (c as c_int) || c == b'_' as c_uchar
	{
	  *sval = c as c_char;
	  sval = sval.offset (1);
	  while *s < end && sval < sval_end
		&& { let ch: c_char = *(*s); *s = (*s).offset (1); c = ch as c_uchar;
		     c_isalnum (c as c_int) || c == b'_' as c_uchar }
	    {
	      *sval = c as c_char;
	      sval = sval.offset (1);
	    }
	  *sval = 0;
	  if *s < end
	    { *s = (*s).offset (-1); }
	  return XBM_TK_IDENT;
	}
      else if c == b'/' as c_uchar && **s == b'*' as c_char
	{
	  /* C-style comment.  */
	  *s = (*s).offset (1);
	  while **s != 0 && ! (**s == b'*' as c_char && *(*s).offset (1) == b'/' as c_char)
	    { *s = (*s).offset (1); }
	  if **s != 0
	    {
	      *s = (*s).offset (2);
	      continue 'loop_;
	    }
	}

      return c as c_int;
    }
}

/* C 4936-5003: `#ifdef HAVE_NTGUI`.  Both definitions are text-only on this machine (no
   `x86_64-w64-mingw32-gcc`), so `<windows.h>`'s names stay C's tokens (the file's own C 226-235
   note and the ledger's SDK-name class).  */

/* Create a Windows bitmap from X bitmap data.  */
/* C's `static HBITMAP w32_create_pixmap_from_bitmap_data (int width, int height, char *data)`
   (C 239's prototype, which the first slice left undeclared for the same reason).  */
/// C: C 4938-4940: `static HBITMAP w32_create_pixmap_from_bitmap_data (int width, int height, char *data)`
#[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
unsafe fn w32_create_pixmap_from_bitmap_data (width: c_int, height: c_int, data: *mut c_char)
  -> HBITMAP
{
  #[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
  static mut swap_nibble: [c_uchar; 16] =
    [0x0, 0x8, 0x4, 0xc,    /* 0000 1000 0100 1100 */
     0x2, 0xa, 0x6, 0xe,    /* 0010 1010 0110 1110 */
     0x1, 0x9, 0x5, 0xd,    /* 0001 1001 0101 1101 */
     0x3, 0xb, 0x7, 0xf];   /* 0011 1011 0111 1111 */
  let mut i: c_int;
  let mut j: c_int;
  let mut w1: c_int;
  let mut w2: c_int;
  let mut bits: *mut c_uchar;
  let mut p: *mut c_uchar;
  let bmp: HBITMAP;

  w1 = (width + 7) / 8;         /* nb of 8bits elt in X bitmap */
  w2 = ((width + 15) / 16) * 2; /* nb of 16bits elt in W32 bitmap */
  /* C's `bits = alloca (height * w2);` - no `alloca` (PORTING.md P-05), so the block comes
     from the stack pool `sa_stack_alloc` serves (the `dosfns.rs` 438 shape).  */
  bits = sa_stack_alloc ((height * w2) as usize) as *mut c_uchar;
  memset (bits as *mut c_void, 0, (height * w2) as usize);
  i = 0;
  while i < height
    {
      p = bits.offset ((i * w2) as isize);
      j = 0;
      while j < w1
	{
	  /* Bitswap XBM bytes to match how Windows does things.  */
	  let c: c_uchar = *data as c_uchar;
	  data = data.offset (1);
	  *p = ((*swap_nibble.as_ptr ().offset ((c & 0xf) as isize) << 4)
		| *swap_nibble.as_ptr ().offset (((c >> 4) & 0xf) as isize)) as c_uchar;
	  p = p.offset (1);
	  j += 1;
	}
      i += 1;
    }
  bmp = CreateBitmap (width, height, 1, 1, bits as *mut c_char);

  return bmp;
}

/// C: C 4971-4973: `static void convert_mono_to_color_image (struct frame *f, struct image *img, COLORREF foreground, COLORREF background)`
#[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
unsafe fn convert_mono_to_color_image (f: *mut frame, img: *mut image, foreground: COLORREF,
				       background: COLORREF)
{
  let mut hdc: HDC;
  let mut old_img_dc: HDC;
  let mut new_img_dc: HDC;
  let mut old_prev: HGDIOBJ;
  let mut new_prev: HGDIOBJ;
  let mut new_pixmap: HBITMAP;

  hdc = get_frame_dc (f);
  old_img_dc = CreateCompatibleDC (hdc);
  new_img_dc = CreateCompatibleDC (hdc);
  new_pixmap = CreateCompatibleBitmap (hdc, (*img).width, (*img).height);
  release_frame_dc (f, hdc);
  old_prev = SelectObject (old_img_dc, (*img).pixmap as HGDIOBJ);
  new_prev = SelectObject (new_img_dc, new_pixmap as HGDIOBJ);
  SetTextColor (new_img_dc, foreground);
  SetBkColor (new_img_dc, background);

  BitBlt (new_img_dc, 0, 0, (*img).width, (*img).height, old_img_dc, 0, 0, SRCCOPY);

  SelectObject (old_img_dc, old_prev);
  SelectObject (new_img_dc, new_prev);
  DeleteDC (old_img_dc);
  DeleteDC (new_img_dc);
  DeleteObject ((*img).pixmap as HGDIOBJ);
  if new_pixmap == 0
    { fputs (c"Failed to convert image to color.\n".as_ptr (), stderr); }
  else
    { (*img).pixmap = new_pixmap as *mut c_void; }
}

/// C: C 5005-5007: `static void Create_Pixmap_From_Bitmap_Data (struct frame *f, struct image *img, char *data, RGB_PIXEL_COLOR fg, RGB_PIXEL_COLOR bg, bool non_default_colors)`
#[cfg (feature = "have_window_system")]
unsafe fn Create_Pixmap_From_Bitmap_Data (f: *mut frame, img: *mut image, mut data: *mut c_char,
					  mut fg: RGB_PIXEL_COLOR, mut bg: RGB_PIXEL_COLOR,
					  non_default_colors: bool)
{
  /* C 5011-5016: `#ifdef USE_CAIRO`.  */
  #[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
  {
    /* C's `Emacs_Color fgbg[] = {{.pixel = fg}, {.pixel = bg}};` - C's designated initialisers
       leave the three channel members zero (PORTING.md 1.5).  */
    let mut fgbg: [Emacs_Color; 2]
      = [Emacs_Color { pixel: fg, red: 0, green: 0, blue: 0 },
	 Emacs_Color { pixel: bg, red: 0, green: 0, blue: 0 }];
    (*FRAME_TERMINAL! (f)).query_colors.unwrap () (f as *mut c_void, fgbg.as_mut_ptr (),
						  countof! (fgbg) as c_int);
    fg = lookup_rgb_color (f, fgbg[0].red as c_int, fgbg[0].green as c_int, fgbg[0].blue as c_int);
    bg = lookup_rgb_color (f, fgbg[1].red as c_int, fgbg[1].green as c_int, fgbg[1].blue as c_int);
    (*img).pixmap
      = image_pix_container_create_from_bitmap_data (data, (*img).width as c_uint,
						     (*img).height as c_uint, fg, bg)
	as *mut c_void;
  }
  /* C 5017-5026: `#elif defined HAVE_X_WINDOWS`.  */
  #[cfg (all (feature = "have_window_system",
	      not (feature = "use_cairo"), feature = "have_x_windows"))]
  {
    (*img).pixmap
      = XCreatePixmapFromBitmapData (FRAME_X_DISPLAY! (f), FRAME_X_DRAWABLE! (f), data,
				     (*img).width as c_uint, (*img).height as c_uint,
				     fg, bg, (*FRAME_DISPLAY_INFO! (f)).n_planes as c_uint)
	as *mut c_void;
    /* C 5027-5030: the nested `# if !defined USE_CAIRO && defined HAVE_XRENDER`.  */
    #[cfg (all (feature = "have_window_system", not (feature = "use_cairo"),
		feature = "have_xrender"))]
    if !(*img).pixmap.is_null ()
      { (*img).picture = x_create_xrender_picture (f, (*img).pixmap as Emacs_Pixmap, 0)
			  as *mut c_void; }
  }
  /* C 5031-5040: `#elif defined HAVE_ANDROID`, with C 5033-5039's `ANDROID_STUBIFY` pair.  */
  #[cfg (all (feature = "have_window_system",
	      not (any (feature = "use_cairo", feature = "have_x_windows")),
	      feature = "have_android"))]
  {
    #[cfg (all (feature = "have_window_system", feature = "not_android_stubify"))]
    {
      (*img).pixmap
	= android_create_pixmap_from_bitmap_data (data, (*img).width as c_uint,
						  (*img).height as c_uint, fg, bg,
						  (*FRAME_DISPLAY_INFO! (f)).n_image_planes as c_uint)
	  as *mut c_void;
    }
    #[cfg (all (feature = "have_window_system", not (feature = "not_android_stubify")))]
    { emacs_abort (); }
  }
  /* C 5041-5047: `#elif defined HAVE_NTGUI` (text-only).  */
  #[cfg (all (feature = "have_window_system",
	      not (any (feature = "use_cairo", feature = "have_x_windows", feature = "have_android")),
	      feature = "have_ntgui"))]
  {
    (*img).pixmap = w32_create_pixmap_from_bitmap_data ((*img).width, (*img).height, data)
		    as *mut c_void;

    /* If colors were specified, transfer the bitmap to a color one.  */
    if non_default_colors
      { convert_mono_to_color_image (f, img, fg, bg); }
  }
  /* C 5048-5049: `#elif defined HAVE_NS`.  */
  #[cfg (all (feature = "have_window_system",
	      not (any (feature = "use_cairo", feature = "have_x_windows", feature = "have_android",
			feature = "have_ntgui")),
	      feature = "have_ns"))]
  { (*img).pixmap = ns_image_from_XBM (data, (*img).width, (*img).height, fg, bg); }
  /* C 5050-5063: `#elif defined HAVE_HAIKU` (text-only).  */
  #[cfg (all (feature = "have_window_system",
	      not (any (feature = "use_cairo", feature = "have_x_windows", feature = "have_android",
			feature = "have_ntgui", feature = "have_ns")),
	      feature = "have_haiku"))]
  {
    (*img).pixmap = BBitmap_new ((*img).width, (*img).height, 0);

    if !(*img).pixmap.is_null ()
      {
	let bytes_per_line: c_int = ((*img).width + 7) / 8;
	let mut y: c_int = 0;
	while y < (*img).height
	  {
	    let mut x: c_int = 0;
	    while x < (*img).width
	      {
		PUT_PIXEL! ((*img).pixmap, x, y,
			    if (*data.offset ((x / 8) as isize) as c_uchar >> (x % 8) & 1) != 0
			      { fg } else { bg });
		x += 1;
	      }
	    data = data.offset (bytes_per_line as isize);
	    y += 1;
	  }
      }
  }
}

/* Replacement for XReadBitmapFileData which isn't available under old
   X versions.  CONTENTS is a pointer to a buffer to parse; END is the
   buffer's end.  Set *WIDTH and *HEIGHT to the width and height of
   the image.  Return in *DATA the bitmap data allocated with xmalloc.
   Value is true if successful.  DATA null means just test if
   CONTENTS looks like an in-memory XBM file.  If INHIBIT_IMAGE_ERROR,
   inhibit the call to image_error when the image size is invalid (the
   bitmap remains unread).  */

/// C: C 5076-5079: `static bool xbm_read_bitmap_data (struct frame *f, char *contents, char *end, int *width, int *height, char **data, bool inhibit_image_error)`
#[cfg (feature = "have_window_system")]
unsafe fn xbm_read_bitmap_data (f: *mut frame, contents: *mut c_char, end: *mut c_char,
				width: *mut c_int, height: *mut c_int, data: *mut *mut c_char,
				inhibit_image_error: bool) -> bool
{
  let mut s: *mut c_char = contents;
  let mut buffer: [c_char; BUFSIZ as usize] = [0; BUFSIZ as usize];
  let mut padding_p: bool = false;
  let mut v10: bool = false;
  let mut bytes_per_line: c_int;
  let mut i: c_int;
  let mut nbytes: c_int;
  let mut p: *mut c_char;
  /* C's `int value;` and `int LA1;` are written before any read (PORTING.md 1.5).  */
  let mut value: c_int = 0;
  let mut LA1: c_int;

  /* C 5093-5111's three macros are local to this function and close over its locals (`s`,
     `end`, `buffer`, `value`, `LA1`), which `macro_rules!` hygiene does not permit (measured:
     `E0426` for the label and `E0425` for the locals), so each use site below carries the
     expansion C's preprocessor produces - `match ()` is `LA1 = xbm_scan (&s, end, buffer,
     &value)`, `expect (TOKEN)` is `do { if (LA1 != (TOKEN)) goto failure; match (); } while
     (0)`, and `expect_ident (IDENT)` is the `if`/`else` pair.  C's `goto failure` is
     `break 'failure` and `goto success` is `break 'success` (PORTING.md 2.2); C 5210-5212's
     three `#undef`s have no Rust counterpart (the block's end is the macro's scope).  */

  *width = -1;
  *height = -1;
  if !data.is_null ()
    { *data = core::ptr::null_mut (); }
  LA1 = xbm_scan (&raw mut s, end, buffer.as_mut_ptr (), &raw mut value);

  'failure: {
    /* Parse defines for width, height and hot-spots.  */
    while LA1 == b'#' as c_int
      {
	LA1 = xbm_scan (&raw mut s, end, buffer.as_mut_ptr (), &raw mut value);
	if LA1 == XBM_TK_IDENT && strcmp (buffer.as_ptr (), c"define".as_ptr ()) == 0
	  { LA1 = xbm_scan (&raw mut s, end, buffer.as_mut_ptr (), &raw mut value); }
	else
	  { break 'failure; }
	if LA1 != XBM_TK_IDENT
	  { break 'failure; }
	LA1 = xbm_scan (&raw mut s, end, buffer.as_mut_ptr (), &raw mut value);

	if LA1 == XBM_TK_NUMBER
	  {
	    /* C's `char *q = strrchr (buffer, '_'); q = q ? q + 1 : buffer;`.  */
	    let q0: *mut c_char = strrchr (buffer.as_ptr (), b'_' as c_int);
	    let q: *mut c_char
	      = if !q0.is_null () { q0.offset (1) } else { buffer.as_mut_ptr () };
	    if strcmp (q, c"width".as_ptr ()) == 0
	      { *width = value; }
	    else if strcmp (q, c"height".as_ptr ()) == 0
	      { *height = value; }
	  }
	if LA1 != XBM_TK_NUMBER
	  { break 'failure; }
	LA1 = xbm_scan (&raw mut s, end, buffer.as_mut_ptr (), &raw mut value);
      }

    'success: {
      if !check_image_size (f, *width, *height)
	{
	  if !inhibit_image_error
	    { image_size_error (); }
	  break 'failure;
	}
      else if data.is_null ()
	{ break 'success; }

      /* Parse bits.  Must start with `static'.  */
      if LA1 == XBM_TK_IDENT && strcmp (buffer.as_ptr (), c"static".as_ptr ()) == 0
	{ LA1 = xbm_scan (&raw mut s, end, buffer.as_mut_ptr (), &raw mut value); }
      else
	{ break 'failure; }
      if LA1 == XBM_TK_IDENT
	{
	  if strcmp (buffer.as_ptr (), c"unsigned".as_ptr ()) == 0
	    {
	      LA1 = xbm_scan (&raw mut s, end, buffer.as_mut_ptr (), &raw mut value);
	      if LA1 == XBM_TK_IDENT && strcmp (buffer.as_ptr (), c"char".as_ptr ()) == 0
		{ LA1 = xbm_scan (&raw mut s, end, buffer.as_mut_ptr (), &raw mut value); }
	      else
		{ break 'failure; }
	    }
	  else if strcmp (buffer.as_ptr (), c"short".as_ptr ()) == 0
	    {
	      LA1 = xbm_scan (&raw mut s, end, buffer.as_mut_ptr (), &raw mut value);
	      v10 = true;
	      if *width % 16 != 0 && *width % 16 < 9
		{ padding_p = true; }
	    }
	  else if strcmp (buffer.as_ptr (), c"char".as_ptr ()) == 0
	    { LA1 = xbm_scan (&raw mut s, end, buffer.as_mut_ptr (), &raw mut value); }
	  else
	    { break 'failure; }
	}
      else
	{ break 'failure; }

      /* C's `expect (XBM_TK_IDENT); expect ('['); expect (']'); expect ('='); expect ('{');`.  */
      if LA1 != XBM_TK_IDENT
	{ break 'failure; }
      LA1 = xbm_scan (&raw mut s, end, buffer.as_mut_ptr (), &raw mut value);
      if LA1 != b'[' as c_int
	{ break 'failure; }
      LA1 = xbm_scan (&raw mut s, end, buffer.as_mut_ptr (), &raw mut value);
      if LA1 != b']' as c_int
	{ break 'failure; }
      LA1 = xbm_scan (&raw mut s, end, buffer.as_mut_ptr (), &raw mut value);
      if LA1 != b'=' as c_int
	{ break 'failure; }
      LA1 = xbm_scan (&raw mut s, end, buffer.as_mut_ptr (), &raw mut value);
      if LA1 != b'{' as c_int
	{ break 'failure; }
      LA1 = xbm_scan (&raw mut s, end, buffer.as_mut_ptr (), &raw mut value);

      if ! image_check_image_size (core::ptr::null_mut (), *width, *height)
	{
	  if !inhibit_image_error
	    { image_error (c"Image too large (%dx%d)".as_ptr (), make_fixnum (*width as EMACS_INT),
			   make_fixnum (*height as EMACS_INT)); }
	  break 'failure;
	}
      bytes_per_line = (*width + 7) / 8 + padding_p as c_int;
      nbytes = bytes_per_line * *height;
      p = xmalloc (nbytes as usize) as *mut c_char;
      *data = p;

      if v10
	{
	  i = 0;
	  while i < nbytes
	    {
	      let val: c_int = value;
	      if LA1 != XBM_TK_NUMBER
		{ break 'failure; }
	      LA1 = xbm_scan (&raw mut s, end, buffer.as_mut_ptr (), &raw mut value);

	      /* C's `*p++ = XBM_BIT_SHUFFLE (val);` narrows the macro's `int` result to `char`
		 on assignment; the port writes the conversion out (the file's "C's implicit
		 conversions are written out" class).  */
	      *p = XBM_BIT_SHUFFLE! (val) as c_char;
	      p = p.offset (1);
	      if !padding_p || ((i + 2) % bytes_per_line) != 0
		{
		  *p = XBM_BIT_SHUFFLE! (value >> 8) as c_char;
		  p = p.offset (1);
		}

	      if LA1 == b',' as c_int || LA1 == b'}' as c_int
		{ LA1 = xbm_scan (&raw mut s, end, buffer.as_mut_ptr (), &raw mut value); }
	      else
		{ break 'failure; }
	      i += 2;
	    }
	}
      else
	{
	  i = 0;
	  while i < nbytes
	    {
	      let val: c_int = value;
	      if LA1 != XBM_TK_NUMBER
		{ break 'failure; }
	      LA1 = xbm_scan (&raw mut s, end, buffer.as_mut_ptr (), &raw mut value);

	      *p = XBM_BIT_SHUFFLE! (val) as c_char;
	      p = p.offset (1);

	      if LA1 == b',' as c_int || LA1 == b'}' as c_int
		{ LA1 = xbm_scan (&raw mut s, end, buffer.as_mut_ptr (), &raw mut value); }
	      else
		{ break 'failure; }
	      i += 1;
	    }
	}
    }

    /* C's `success: return 1;`.  */
    return true;
  }

  /* C's `failure:`.  */
  if !data.is_null () && !(*data).is_null ()
    {
      xfree (*data as *mut c_void);
      *data = core::ptr::null_mut ();
    }
  return false;
}

/* Load XBM image IMG which will be displayed on frame F from buffer
   CONTENTS.  END is the end of the buffer.  Value is true if
   successful.  */

/// C: C 5238-5239: `static bool xbm_load_image (struct frame *f, struct image *img, char *contents, char *end)`
#[cfg (feature = "have_window_system")]
unsafe fn xbm_load_image (f: *mut frame, img: *mut image, contents: *mut c_char,
			  end: *mut c_char) -> bool
{
  let rc: bool;
  let mut data: *mut c_char = core::ptr::null_mut ();
  let mut success_p: bool = false;

  rc = xbm_read_bitmap_data (f, contents, end, &raw mut (*img).width, &raw mut (*img).height,
			     &raw mut data, false);

  if rc
    {
      let mut foreground: c_ulong = (*img).face_foreground;
      let mut background: c_ulong = (*img).face_background;
      let mut non_default_colors: bool = false;
      let mut value: Lisp_Object;

      eassert! ((*img).width > 0 && (*img).height > 0);

      /* Get foreground and background colors, maybe allocate colors.  */
      value = image_spec_value ((*img).spec, QCforeground (), core::ptr::null_mut ());
      if !NILP (value)
	{
	  foreground = image_alloc_image_color (f, img, value, foreground);
	  non_default_colors = true;
	}
      value = image_spec_value ((*img).spec, QCbackground (), core::ptr::null_mut ());
      if !NILP (value)
	{
	  background = image_alloc_image_color (f, img, value, background);
	  (*img).background = background;
	  /* C's `img->background_valid = 1;` - the bit member's storage unit (P-80).  */
	  (*img).bits = (*img).bits & !IMAGE_BACKGROUND_VALID_MASK
			| ((1 as u32) << IMAGE_BACKGROUND_VALID_SHIFT);
	  non_default_colors = true;
	}

      if image_check_image_size (core::ptr::null_mut (), (*img).width, (*img).height)
	{ Create_Pixmap_From_Bitmap_Data (f, img, data, foreground, background,
					  non_default_colors); }
      else
	{ (*img).pixmap = NO_PIXMAP! () as *mut c_void; }
      xfree (data as *mut c_void);

      if (*img).pixmap as Emacs_Pixmap == NO_PIXMAP! () as Emacs_Pixmap
	{
	  image_clear_image (f, img);
	  image_error (c"Unable to create X pixmap for `%s'".as_ptr (), (*img).spec);
	}
      else
	{ success_p = true; }
    }
  else
    { image_error (c"Error loading XBM image `%s'".as_ptr (), (*img).spec); }

  return success_p;
}

/* Value is true if DATA looks like an in-memory XBM file.  */

/// C: C 5300-5301: `static bool xbm_file_p (Lisp_Object data)`
#[cfg (feature = "have_window_system")]
unsafe fn xbm_file_p (data: Lisp_Object) -> bool
{
  let mut w: c_int = 0;
  let mut h: c_int = 0;
  return STRINGP (data)
    && xbm_read_bitmap_data (core::ptr::null_mut (), SSDATA (data),
			     SSDATA (data).offset (SBYTES (data) as isize),
			     &raw mut w, &raw mut h, core::ptr::null_mut (), true);
}

/* Fill image IMG which is used on frame F with pixmap data.  Value is
   true if successful.  */

/// C: C 5313-5314: `static bool xbm_load (struct frame *f, struct image *img)`
#[cfg (feature = "have_window_system")]
unsafe extern "C" fn xbm_load (f: *mut frame, img: *mut image) -> bool
{
  let mut success_p: bool = false;
  let file_name: Lisp_Object;

  eassert! (xbm_image_p ((*img).spec));

  /* If IMG->spec specifies a file name, create a non-file spec from it.  */
  file_name = image_spec_value ((*img).spec, QCfile (), core::ptr::null_mut ());
  if STRINGP (file_name)
    {
      let mut size: ptrdiff_t = 0;
      let contents: *mut c_char = slurp_image (file_name, &raw mut size, c"XBM".as_ptr ());
      if contents.is_null ()
	{ return false; }
      success_p = xbm_load_image (f, img, contents, contents.offset (size as isize));
      xfree (contents as *mut c_void);
    }
  else
    {
      let mut fmt: [image_keyword; XBM_LAST as usize] = core::mem::zeroed ();
      let data: Lisp_Object;
      let mut foreground: c_ulong = (*img).face_foreground;
      let mut background: c_ulong = (*img).face_background;
      let mut non_default_colors: bool = false;
      /* lisp.rs's `SAFE_NALLOCA!` note: the macro passes `$buf` by value, so a buffer C leaves
	 uninitialized takes C's zero here (PORTING.md 1.5/0.1-13).  */
      let mut bits: *mut c_char = core::ptr::null_mut ();
      let parsed_p: bool;
      let mut in_memory_file_p: bool = false;

      /* See if data looks like an in-memory XBM file.  */
      data = image_spec_value ((*img).spec, QCdata (), core::ptr::null_mut ());
      in_memory_file_p = xbm_file_p (data);

      /* Parse the image specification.  */
      memcpy (fmt.as_mut_ptr () as *mut c_void, xbm_format.as_ptr () as *const c_void,
	      core::mem::size_of::<[image_keyword; XBM_LAST as usize]> ());
      parsed_p = parse_image_spec ((*img).spec, fmt.as_mut_ptr (), XBM_LAST, Qxbm ());
      eassert! (parsed_p);

      /* Get specified width, and height.  */
      if !in_memory_file_p
	{
	  (*img).width = XFIXNAT (fmt [XBM_DATA_WIDTH as usize].value) as c_int;
	  (*img).height = XFIXNAT (fmt [XBM_DATA_HEIGHT as usize].value) as c_int;
	  eassert! ((*img).width > 0 && (*img).height > 0);
	  if !check_image_size (f, (*img).width, (*img).height)
	    {
	      image_size_error ();
	      return false;
	    }
	}

      /* Get foreground and background colors, maybe allocate colors.  */
      if fmt [XBM_FOREGROUND as usize].count
	  && STRINGP (fmt [XBM_FOREGROUND as usize].value)
	{
	  foreground = image_alloc_image_color (f, img, fmt [XBM_FOREGROUND as usize].value,
						foreground);
	  non_default_colors = true;
	}

      if fmt [XBM_BACKGROUND as usize].count
	  && STRINGP (fmt [XBM_BACKGROUND as usize].value)
	{
	  background = image_alloc_image_color (f, img, fmt [XBM_BACKGROUND as usize].value,
						background);
	  non_default_colors = true;
	}

      if in_memory_file_p
	{ success_p = xbm_load_image (f, img, SSDATA (data),
				      SSDATA (data).offset (SBYTES (data) as isize)); }
      else
	{
	  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);

	  if VECTORP (data)
	    {
	      let mut i: c_int;
	      let mut p: *mut c_char;
	      let nbytes: c_int = ((*img).width + CHAR_BIT - 1) / CHAR_BIT;

	      SAFE_NALLOCA! (sa_avail, bits, nbytes, (*img).height);
	      p = bits;
	      i = 0;
	      while i < (*img).height
		{
		  let line: Lisp_Object = AREF (data, i as isize);
		  if STRINGP (line)
		    { memcpy (p as *mut c_void, SDATA (line) as *const c_void, nbytes as usize); }
		  else
		    { memcpy (p as *mut c_void, bool_vector_data (line) as *const c_void,
			      nbytes as usize); }
		  i += 1;
		  p = p.offset (nbytes as isize);
		}
	    }
	  else if STRINGP (data)
	    { bits = SSDATA (data); }
	  else
	    { bits = bool_vector_data (data) as *mut c_char; }

	  /* C 5416-5427: `#ifdef HAVE_NTGUI` (text-only).  */
	  #[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
	  {
	    let invertedBits: *mut c_char;
	    let nbytes: c_int;
	    let mut i: c_int;
	    /* Windows mono bitmaps are reversed compared with X.  */
	    invertedBits = bits;
	    nbytes = ((*img).width + CHAR_BIT - 1) / CHAR_BIT * (*img).height;
	    SAFE_NALLOCA! (sa_avail, bits, 1, nbytes);
	    i = 0;
	    while i < nbytes
	      {
		*bits.offset (i as isize) = XBM_BIT_SHUFFLE! (*invertedBits.offset (i as isize));
		i += 1;
	      }
	  }
	  /* Create the pixmap.  */

	  if image_check_image_size (core::ptr::null_mut (), (*img).width, (*img).height)
	    { Create_Pixmap_From_Bitmap_Data (f, img, bits, foreground, background,
					      non_default_colors); }
	  else
	    { (*img).pixmap = NO_PIXMAP! () as *mut c_void; }

	  if (*img).pixmap as Emacs_Pixmap != NO_PIXMAP! () as Emacs_Pixmap
	    { success_p = true; }
	  else
	    {
	      image_error (c"Unable to create pixmap for XBM image `%s'".as_ptr (), (*img).spec);
	      image_clear_image (f, img);
	    }

	  SAFE_FREE! (sa_count, sa_mark);
	}
    }

  return success_p;
}

/***********************************************************************
			      Canvas
 ***********************************************************************/

/* Indices of image specification fields in canvas_format, below.  */

/* C 5460-5471: `enum canvas_keyword_index` - the port's `c_int` constants.  */
/// C: C 5460-5471: `enum canvas_keyword_index`
#[cfg (feature = "have_window_system")]
pub const CANVAS_TYPE: c_int = 0;
#[cfg (feature = "have_window_system")]
pub const CANVAS_ID: c_int = 1;
#[cfg (feature = "have_window_system")]
pub const CANVAS_FILE: c_int = 2;
#[cfg (feature = "have_window_system")]
pub const CANVAS_DATA: c_int = 3;
#[cfg (feature = "have_window_system")]
pub const CANVAS_WIDTH: c_int = 4;
#[cfg (feature = "have_window_system")]
pub const CANVAS_HEIGHT: c_int = 5;
#[cfg (feature = "have_window_system")]
pub const CANVAS_ASCENT: c_int = 6;
#[cfg (feature = "have_window_system")]
pub const CANVAS_MARGIN: c_int = 7;
#[cfg (feature = "have_window_system")]
pub const CANVAS_RELIEF: c_int = 8;
#[cfg (feature = "have_window_system")]
pub const CANVAS_LAST: c_int = 9;

/* Vector of image_keyword structures describing the format
   of valid user-defined image specifications.  */

/* C 5476-5489: the table, with C's implicit trailing zeros written out (the `xbm_format`
   shape).  */
/// C: C 5476-5477: `static const struct image_keyword canvas_format [CANVAS_LAST]`
#[cfg (feature = "have_window_system")]
static canvas_format: [image_keyword; CANVAS_LAST as usize] =
[
  image_keyword { name: c":type".as_ptr (), r#type: IMAGE_SYMBOL_VALUE,
		  mandatory_p: true, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":id".as_ptr (), r#type: IMAGE_SYMBOL_VALUE,
		  mandatory_p: true, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":file".as_ptr (), r#type: IMAGE_STRING_VALUE,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":data".as_ptr (), r#type: IMAGE_DONT_CHECK_VALUE_TYPE,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":data-width".as_ptr (), r#type: IMAGE_POSITIVE_INTEGER_VALUE,
		  mandatory_p: true, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":data-height".as_ptr (), r#type: IMAGE_POSITIVE_INTEGER_VALUE,
		  mandatory_p: true, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":ascent".as_ptr (), r#type: IMAGE_ASCENT_VALUE,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":margin".as_ptr (), r#type: IMAGE_NON_NEGATIVE_INTEGER_VALUE_OR_PAIR,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":relief".as_ptr (), r#type: IMAGE_INTEGER_VALUE,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
];

/* Weak hash map associating canvas image specs with the canvas objects.

   As long as a canvas image spec object is alive, the canvas object
   backing it, will stay alive.  As soon as the GC runs and frees
   unreferenced canvas image specs, the specs are are also removed from
   the weak canvas_map.  Then canvas_free_unused will check the
   canvas_map and free the backing canvas objects.   */

/* C 5502: `static Lisp_Object canvas_map;` - C's tentative definition, i.e. `Qnil` (0).  */
#[cfg (feature = "have_window_system")]
static mut canvas_map: Lisp_Object = core::ptr::null_mut ();

/* Linked list of all canvas objects.  */

/* C 5506: `static struct canvas* canvas_list = 0;`  */
#[cfg (feature = "have_window_system")]
static mut canvas_list: *mut canvas = core::ptr::null_mut ();

/* Parse canvas specification OBJECT and return true if valid.  */

/// C: C 5512-5513: `static bool canvas_parse (Lisp_Object object, struct image_keyword *fmt)`
#[cfg (feature = "have_window_system")]
unsafe fn canvas_parse (object: Lisp_Object, fmt: *mut image_keyword) -> bool
{
  memcpy (fmt as *mut c_void, canvas_format.as_ptr () as *const c_void,
	  core::mem::size_of::<[image_keyword; CANVAS_LAST as usize]> ());

  /* Check that only one of :data or :file is present.  */
  if !parse_image_spec (object, fmt, CANVAS_LAST, Qcanvas ())
      || (*fmt.offset (CANVAS_FILE as isize)).count as c_int
	   + (*fmt.offset (CANVAS_DATA as isize)).count as c_int > 1
    { return false; }

  let w: ptrdiff_t = XFIXNAT ((*fmt.offset (CANVAS_WIDTH as isize)).value);
  let h: ptrdiff_t = XFIXNAT ((*fmt.offset (CANVAS_HEIGHT as isize)).value);

  /* Check that w*h*4 does not overflow */
  return w <= INT_MAX as ptrdiff_t / 4 / h;
}

/* Return true if OBJECT is a valid canvas image specification.  */

/// C: C 5525-5526: `static bool canvas_image_p (Lisp_Object object)`
#[cfg (feature = "have_window_system")]
unsafe extern "C" fn canvas_image_p (object: Lisp_Object) -> bool
{
  let mut fmt: [image_keyword; CANVAS_LAST as usize] = core::mem::zeroed ();
  return canvas_parse (object, fmt.as_mut_ptr ());
}

/* Clear canvas list.  All canvases which are not referenced anymore in
   the weak hash table canvas_map are freed.  */

/// C: C 5533-5534: `static void canvas_free_unused (void)`
#[cfg (feature = "have_window_system")]
unsafe fn canvas_free_unused ()
{
  /* Mark all referenced canvases as used (negative width).  */
  DOHASH! (XHASH_TABLE (canvas_map), k, v, {
    (*(XFIXNUMPTR (v) as *mut canvas)).width *= -1;
  });

  /* Free unreferenced canvases and remove them from the list.  */
  let mut p: *mut *mut canvas = &raw mut canvas_list;
  while !(*p).is_null ()
    {
      let c: *mut canvas = *p;
      if (*c).width < 0
        {
	  p = &raw mut (*c).next;
	  (*c).width *= -1;
	}
      else
	{
	  *p = (*c).next;
	  xfree ((*c).data as *mut c_void);
	  xfree (c as *mut c_void);
        }
    }
}

/* Copy pixel data into canvas C from a parsed image keyword array FMT.

   :data must be an unibyte string of exactly 4*WIDTH*HEIGHT bytes, or a
   vector of size WIDTH*HEIGHT in row-major order, where each element is
   a 32 bit integer.  :file names a binary file with size 4*WIDTH*HEIGHT
   bytes.  */

/// C: C 5566-5567: `static void canvas_apply_data (struct canvas *c, struct image_keyword *fmt)`
#[cfg (feature = "have_window_system")]
unsafe fn canvas_apply_data (c: *mut canvas, fmt: *mut image_keyword)
{
  /* C's compound literal `(ptrdiff_t) {c->width}` is the cast below.  */
  let expected_size: ptrdiff_t = ((*c).width as ptrdiff_t) * ((*c).height as ptrdiff_t);

  let data: Lisp_Object = (*fmt.offset (CANVAS_DATA as isize)).value;
  let file: Lisp_Object = (*fmt.offset (CANVAS_FILE as isize)).value;

  if STRINGP (data) /* Unibyte string data in ARGB32 format.  */
    {
      if STRING_MULTIBYTE (data)
	{
          image_error (c"Canvas :data string must be unibyte".as_ptr ());
	  return;
	}

      if SBYTES (data) != 4 * expected_size
	{
          image_error (c"Canvas :data size mismatch".as_ptr ());
	  return;
	}

      let buf: *const uint32_t = SDATA (data) as *const uint32_t;
      /* C 5590-5596: `#ifdef WORDS_BIGENDIAN` / `#else`.  */
      #[cfg (feature = "words_bigendian")]
      {
	let mut i: ptrdiff_t = 0;
	while i < expected_size
	  {
	    *(*c).data.offset (i as isize) = bswap_32 (*buf.offset (i as isize));
	    i += 1;
	  }
      }
      #[cfg (not (feature = "words_bigendian"))]
      {
	let mut i: ptrdiff_t = 0;
	while i < expected_size
	  {
	    *(*c).data.offset (i as isize) = *buf.offset (i as isize);
	    i += 1;
	  }
      }
    }
  else if VECTORP (data) /* Vector of ARGB32 integers.  */
    {
      if ASIZE (data) != expected_size
	{
	  image_error (c"Canvas :data size mismatch".as_ptr ());
	  return;
	}

      let mut i: ptrdiff_t = 0;
      while i < expected_size
	{
          let pixel: Lisp_Object = AREF (data, i as isize);
	  let mut pix: uint32_t = 0;
	  let mut bigpix: intmax_t = 0;
	  /* C's `FIXNUMP (pixel) ? ckd_add (&pix, XFIXNUM (pixel), 0) : (…)`: the sum is
	     formed in the wider type and range-checked into the `uint32_t` result (P-15's
	     family).  */
	  let bad: bool
	    = if FIXNUMP (pixel)
		{
		  let (sum, over) = XFIXNUM (pixel).overflowing_add (0);
		  let fits: bool = !over && 0 <= sum && sum <= UINT32_MAX as EMACS_INT;
		  pix = sum as uint32_t;
		  !fits
		}
	      else
		{
		  UINT32_MAX as EMACS_INT <= MOST_POSITIVE_FIXNUM
		    || !BIGNUMP (pixel)
		    || { bigpix = bignum_to_intmax (pixel); bigpix == 0 }
		    || { let (sum, over) = bigpix.overflowing_add (0);
			 let fits: bool
			   = !over && 0 <= sum && sum <= UINT32_MAX as intmax_t;
			 pix = sum as uint32_t;
			 !fits }
		};
	  if bad
	    {
	      image_error (c"Expected 0 <= datum < 2**32 in the canvas :data vector".as_ptr ());
	      return;
	    }
	  *(*c).data.offset (i as isize) = pix;
	  i += 1;
	}
    }
  else if STRINGP (file) /* Binary file with ARGB32 data.  */
    {
      let mut fd: image_fd = core::mem::zeroed ();
      let found: Lisp_Object = image_find_image_fd (file, &raw mut fd);
      if !STRINGP (found)
	{
	  image_error (c"Cannot open image :file for canvas %s".as_ptr (), file);
	  return;
	}

      let mut nbytes: ptrdiff_t = 0;
      let buf: *mut uint32_t = slurp_file (fd, &raw mut nbytes) as *mut uint32_t;
      if buf.is_null ()
	{
	  image_error (c"Cannot read image :file for canvas %s".as_ptr (), file);
	  return;
	}

      if nbytes != 4 * expected_size
	{
	  image_error (c"Canvas :file size mismatch for %s".as_ptr (), file);
	  xfree (buf as *mut c_void);
	  return;
	}

      #[cfg (feature = "words_bigendian")]
      {
	let mut i: ptrdiff_t = 0;
	while i < expected_size
	  {
	    *(*c).data.offset (i as isize) = bswap_32 (*buf.offset (i as isize));
	    i += 1;
	  }
      }
      #[cfg (not (feature = "words_bigendian"))]
      {
	let mut i: ptrdiff_t = 0;
	while i < expected_size
	  {
	    *(*c).data.offset (i as isize) = *buf.offset (i as isize);
	    i += 1;
	  }
      }

      xfree (buf as *mut c_void);
    }
}

/* Get canvas object for IMAGE specification.  Returns NULL on error.  */

/// C: C 5662-5663: `static struct canvas* canvas_get (Lisp_Object image, struct image_keyword *fmt)`
#[cfg (feature = "have_window_system")]
unsafe fn canvas_get (image: Lisp_Object, fmt: *mut image_keyword) -> *mut canvas
{
  if !canvas_parse (image, fmt)
    {
      image_error (c"Not a canvas image specification".as_ptr ());
      return core::ptr::null_mut ();
    }

  let canvas_ptr: Lisp_Object = Fgethash (image, canvas_map, Qnil ());
  let mut c: *mut canvas = if NILP (canvas_ptr) { core::ptr::null_mut () }
			   else { XFIXNUMPTR (canvas_ptr) as *mut canvas };
  let width: c_int = XFIXNAT ((*fmt.offset (CANVAS_WIDTH as isize)).value) as c_int;
  let height: c_int = XFIXNAT ((*fmt.offset (CANVAS_HEIGHT as isize)).value) as c_int;

  if c.is_null ()
    {
      /* Free old canvases now, when allocating a new one, to keep
         memory usage low.  */
      canvas_free_unused ();

      c = xzalloc (core::mem::size_of::<canvas> ()) as *mut canvas;
      (*c).refresh = 2; /* 2 in order to enforce first refresh.  */
      (*c).width = width;
      (*c).height = height;
      (*c).data = xzalloc ((4 * width * height) as usize) as *mut uint32_t;

      /* Register the canvas in the list and the map.  */
      (*c).next = canvas_list;
      canvas_list = c;
      Fputhash (image, make_pointer_integer (c as *mut c_void), canvas_map);

      /* Initialize pixel buffer from :data or :file if supplied.  */
      canvas_apply_data (c, fmt);
    }
  else if (*c).width != width || (*c).height != height
    {
      /* Resize canvas.  */
      (*c).width = width;
      (*c).height = height;
      (*c).data = xrealloc ((*c).data as *mut c_void, (4 * width * height) as usize)
		  as *mut uint32_t;
      memset ((*c).data as *mut c_void, 0, (4 * width * height) as usize);

      /* Initialize pixel buffer from :data or :file if supplied.  */
      canvas_apply_data (c, fmt);
    }

  return c;
}

/* Create canvas IMG in frame F.  Value is true if successful.  */

/// C: C 5713-5714: `static bool canvas_load (struct frame *f, struct image *img)`
#[cfg (feature = "have_window_system")]
unsafe extern "C" fn canvas_load (f: *mut frame, img: *mut image) -> bool
{
  let mut fmt: [image_keyword; CANVAS_LAST as usize] = core::mem::zeroed ();
  let c: *mut canvas = canvas_get ((*img).spec, fmt.as_mut_ptr ());

  if c.is_null ()
    { return false; }

  (*img).lisp_data = make_pointer_integer (c as *mut c_void);
  (*img).refresh = 1; /* refresh is always > 0 for canvas images */
  (*img).width = (*c).width;
  (*img).height = (*c).height;
  /* C's `img->background_valid = 1;` and `img->background_transparent_valid = 1;` - the two
     bit members' storage units (P-80).  */
  (*img).bits = (*img).bits & !IMAGE_BACKGROUND_VALID_MASK
		| ((1 as u32) << IMAGE_BACKGROUND_VALID_SHIFT);
  (*img).bits = (*img).bits & !IMAGE_BACKGROUND_TRANSPARENT_VALID_MASK
		| ((1 as u32) << IMAGE_BACKGROUND_TRANSPARENT_VALID_SHIFT);

  let mut ximg: Emacs_Pix_Container = core::ptr::null_mut ();
  if !image_create_x_image_and_pixmap (f, img, (*c).width, (*c).height, 0, &raw mut ximg, false)
    { return false; }

  image_put_x_image (f, img, ximg, false);

  return true;
}

/* Prepare image IMG from canvas for display.  */

/// C: C 5740-5741: `static void canvas_prepare_for_display (struct frame *f, struct image *img)`
#[cfg (feature = "have_window_system")]
unsafe fn canvas_prepare_for_display (f: *mut frame, img: *mut image)
{
  let c: *mut canvas = XFIXNUMPTR ((*img).lisp_data) as *mut canvas;
  (*img).refresh = (*c).refresh;
  let src: *mut uint32_t = (*c).data;
  let width: c_int = (*c).width;
  let height: c_int = (*c).height;

  /* If canvas has been resized in the meantime and the image is stale,
     mark frame as garbaged and wait for redisplay.  See also
     uncache_image which handles stale images.  */
  if width != (*img).original_width || height != (*img).original_height
    {
      SET_FRAME_GARBAGED! (f);
      return;
    }

  block_input ();

  /* C 5760-5826: `#ifdef USE_CAIRO`.  */
  #[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
  {
    /* Cairo: Optimized canvas reloading.  Reuse the existing Cairo surface.  */
    /* C's `cairo_surface_t* surface;` - written by the call below before any read
       (PORTING.md 1.5).  */
    let mut surface: *mut cairo_surface_t = core::ptr::null_mut ();
    if !(*img).cr_data.is_null ()
	/* prepare_image_for_display ensures that cr_data is a surface pattern */
	&& cairo_pattern_get_type ((*img).cr_data as *mut cairo_pattern_t)
	   == CAIRO_PATTERN_TYPE_SURFACE
	&& cairo_pattern_get_surface ((*img).cr_data as *mut cairo_pattern_t,
				      &raw mut surface) == 0
      {
	cairo_surface_flush (surface);
	let stride: c_int = cairo_image_surface_get_stride (surface);
	let dst: *mut c_uchar = cairo_image_surface_get_data (surface);
	/* Alpha channel is preserved here.  Potentially preserve it when
	   drawing the image in x_draw_image_glyph_string?  */
	if stride == 4 * width /* Fast path */
	  {
	    memcpy (dst as *mut c_void, src as *const c_void, (stride * height) as usize);
	  }
	else
	  {
	    let mut y: c_int = 0;
	    while y < height
	      {
		memcpy (dst.offset ((y * stride) as isize) as *mut c_void,
			src.offset ((y * width) as isize) as *const c_void,
			(4 * width) as usize);
		y += 1;
	      }
	  }
	cairo_surface_mark_dirty (surface);
      }
  }
  /* C 5827-5845: `#elif defined HAVE_X_WINDOWS`.  */
  #[cfg (all (feature = "have_window_system",
	      not (feature = "use_cairo"), feature = "have_x_windows"))]
  {
    /* X11: Optimized canvas reloading.  Reuse the existing pixmap.  */
    let depth: c_int = (*FRAME_DISPLAY_INFO! (f)).n_planes;
    let ximg: *mut XImage
      = XCreateImage (FRAME_X_DISPLAY! (f), FRAME_X_VISUAL! (f), depth as c_uint, ZPixmap, 0,
		      core::ptr::null_mut (), width as c_uint, height as c_uint,
		      if depth > 16 { 32 } else if depth > 8 { 16 } else { 8 }, 0);
    if !ximg.is_null ()
      {
	(*ximg).data = xmalloc (((*ximg).bytes_per_line * height) as usize) as *mut c_char;
	let mut y: c_int = 0;
	while y < height
	  {
	    let mut x: c_int = 0;
	    while x < width
	      {
		let c: uint32_t = *src.offset ((y * width + x) as isize);
		let r: uint32_t = (c >> 16) & 255;
		let g: uint32_t = (c >> 8) & 255;
		let b: uint32_t = c & 255;
		PUT_PIXEL! (ximg, x, y,
			    lookup_rgb_color (f, (r << 8) as c_int, (g << 8) as c_int,
					      (b << 8) as c_int));
		x += 1;
	      }
	    y += 1;
	  }
	gui_put_x_image (f, ximg as Emacs_Pix_Container, (*img).pixmap as Emacs_Pixmap,
			 width, height);
	x_destroy_x_image (ximg);
      }
  }
  /* C 5846-5848's `#elif defined HAVE_ANDROID` sits between the two below.  */
  #[cfg (all (feature = "have_window_system",
	      not (any (feature = "use_cairo", feature = "have_x_windows")),
	      feature = "have_android"))]
  {
    /* Android: Optimized canvas reloading.  Reuse the existing pixmap.  */
    let ximg: *mut android_image
      = android_create_image ((*FRAME_DISPLAY_INFO! (f)).n_planes as c_uint, ANDROID_Z_PIXMAP,
			      core::ptr::null_mut (), width as c_uint, height as c_uint);
    if !ximg.is_null ()
      {
	(*ximg).data = xmalloc (((*ximg).bytes_per_line * height) as usize) as *mut c_char;
	let mut y: c_int = 0;
	while y < height
	  {
	    let mut x: c_int = 0;
	    while x < width
	      {
		PUT_PIXEL! (ximg, x, y,
			    (*src.offset ((y * width + x) as isize) & 0x00FFFFFF) as c_ulong);
		x += 1;
	      }
	    y += 1;
	  }
	gui_put_x_image (f, ximg as Emacs_Pix_Container, (*img).pixmap as Emacs_Pixmap,
			 width, height);
	image_destroy_x_image (ximg as Emacs_Pix_Container);
      }
  }
  /* C 5850-5855: `#elif defined HAVE_NS`.  */
  #[cfg (all (feature = "have_window_system",
	      not (any (feature = "use_cairo", feature = "have_x_windows", feature = "have_android")),
	      feature = "have_ns"))]
  {
    /* NS: Recreates and fills the pixmap.  This is a workaround, ideally
       we'd like to recache the NSImage instead.  */
    (*img).pixmap = ns_image_reset ((*img).pixmap, width, height);
    let mut y: c_int = 0;
    while y < height
      {
	let mut x: c_int = 0;
	while x < width
	  {
	    PUT_PIXEL! ((*img).pixmap, x, y, *src.offset ((y * width + x) as isize) as c_ulong);
	    x += 1;
	  }
	y += 1;
      }
  }
  /* C 5856-5875: `#else`.  */
  #[cfg (not (any (feature = "use_cairo", feature = "have_x_windows", feature = "have_android",
		   feature = "have_ns")))]
  {
    /* Platform independent canvas reloading.  Less efficient, since it
       recreates images and pixmaps.  */
    (*FRAME_TERMINAL! (f)).free_pixmap.unwrap () (f as *mut c_void, (*img).pixmap);
    (*img).pixmap = NO_PIXMAP! () as *mut c_void;
    let mut ximg: Emacs_Pix_Container = core::ptr::null_mut ();
    if image_create_x_image_and_pixmap (f, img, width, height, 0, &raw mut ximg, false)
      {
	let mut y: c_int = 0;
	while y < height
	  {
	    let mut x: c_int = 0;
	    while x < width
	      {
		let c: uint32_t = *src.offset ((y * width + x) as isize);
		let r: uint32_t = (c >> 16) & 255;
		let g: uint32_t = (c >> 8) & 255;
		let b: uint32_t = c & 255;
		PUT_PIXEL! (ximg, x, y,
			    lookup_rgb_color (f, (r << 8) as c_int, (g << 8) as c_int,
					      (b << 8) as c_int));
		x += 1;
	      }
	    y += 1;
	  }
	image_put_x_image (f, img, ximg, false);
      }
  }

  unblock_input ();
}

/* Access canvas buffer.  Note that the pixel buffer
 is valid only as long as its dimensions have not been changed.*/

/// C: C 5853-5854: `uint32_t * canvas_data (Lisp_Object image)` - non-static.
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn canvas_data (image: Lisp_Object) -> *mut uint32_t
{
  let mut fmt: [image_keyword; CANVAS_LAST as usize] = core::mem::zeroed ();
  let c: *mut canvas = canvas_get (image, fmt.as_mut_ptr ());

  if c.is_null ()
    { error (c"Not a canvas".as_ptr ()); }

  return (*c).data;
}

#[cfg (feature = "have_window_system")]
DEFUN! ("canvas-refresh", Fcanvas_refresh, Scanvas_refresh, 1, 2, 0,
       doc: "Refresh canvas IMAGE and update it on screen.\n\nIf RELOAD-DATA is non-nil, reload the :data or :file from the image\nspecification before redrawing the canvas.\n\nWhen `canvas-refresh' is called from a timer or a command, `redisplay'\nwill be called implicitly after the timer or command.  `redisplay' must\nbe called explicitly after `canvas-refresh' only when the redraw should\nhappen from a loop.  See Info node `(elisp) Canvas Images' for\nexamples.",
       (image: Lisp_Object, reload_data: Lisp_Object),
       {
  let mut fmt: [image_keyword; CANVAS_LAST as usize] = core::mem::zeroed ();
  let c: *mut canvas = canvas_get (image, fmt.as_mut_ptr ());

  if c.is_null ()
    { error (c"Not a canvas".as_ptr ()); }

  /* Reload :data or :file from the image specification.  */
  if !NILP (reload_data)
    { canvas_apply_data (c, fmt.as_mut_ptr ()); }

  /* Increment refresh counter; reset to one on overflow, since the
     refresh counter must always be greater than zero.  */
  (*c).refresh = (*c).refresh + 1;
  if (*c).refresh == 0
    { (*c).refresh = 1; }

  /* C 5894-5903: `#ifdef HAVE_WINDOW_SYSTEM`.  */
  #[cfg (feature = "have_window_system")]
  {
    /* Redraw all image glyphs.  */
    block_input ();
    redraw_image_glyphs (image);
    /* We do not call `redisplay' or `flush_frame' here.  This means the
       canvas images are not updated immediately on screen, since the
       double buffer won't be flipped immediately.  The next call to
       `redisplay' will flip the double buffer.  */
    unblock_input ();
  }

  return Qnil ();
});

/***********************************************************************
			      XPM images
 ***********************************************************************/

/* C 5913-5919: `#if defined (HAVE_XPM) || defined (HAVE_NS) || defined (HAVE_PGTK) ||
   defined (HAVE_ANDROID)`'s two `static` prototypes - `xpm_image_p` (C 6231) and `xpm_load`
   (C 6307), both outside this slice, so the pair is dropped (PORTING.md 0.1-12; nothing here
   calls them).  */

/* C 5921-5942: `#ifdef HAVE_XPM`'s `#include "X11/xpm.h"` (C 5939/5941) and its `HAVE_NTGUI`
   arm (C 5922-5936), whose `noX/xpm.h` renames Xlib's `XColor`/`XImage`/`Display` (C 5925-5929)
   and which no file in the reference tree carries.  `<X11/xpm.h>` has no file in the reference
   tree and no mirror either, so the declarations this slice reads from it stand here (P-88 ①) -
   the X11 arm's, so they carry `have_x_windows` on top of C's own `have_xpm`; the Windows arm's
   declarations are owed with that arm (its `<windows.h>` names and the `xpm_*` shims are the
   text-only Windows class of `docs/ports/image.md`).  Every number below is measured with this
   machine's clang against the upstream header (`docs/ports/image.md`, Evidence 21): the header
   is `X11 xpm.h` (libXpm, sha256 652b9aa9…), `sizeof (XpmAttributes)` **224** with
   `valuemask` 0, `visual` 8, `colormap` 16, `depth` 24, `width` 28, `npixels` 56, `colorsymbols`
   64, `numsymbols` 72, `nalloc_pixels` 184, `alloc_close_colors` 188, `alloc_color` 200,
   `free_colors` 208, `color_closure` 216, and `sizeof (XpmColorSymbol)` **24** with `name` 0,
   `value` 8, `pixel` 16.  */

/// C: xpm.h 62-65: `#if ! defined(_XtIntrinsic_h) && ! defined(PIXEL_ALREADY_TYPEDEFED)` /
/// `typedef unsigned long Pixel;` (`include/xlib.rs` carries no `Pixel`).
#[cfg (all (feature = "have_window_system", feature = "have_xpm", feature = "have_x_windows"))]
pub type Pixel = c_ulong;

/// C: xpm.h 77-81: `typedef struct { char *name; char *value; Pixel pixel; } XpmColorSymbol;`
#[cfg (all (feature = "have_window_system", feature = "have_xpm", feature = "have_x_windows"))]
#[repr(C)]
pub struct XpmColorSymbol
{
  pub name: *mut c_char,		/* Symbolic color name */
  pub value: *mut c_char,		/* Color value */
  pub pixel: Pixel,			/* Color pixel */
}

/// C: xpm.h 83-87: `typedef struct { char *name; unsigned int nlines; char **lines; } XpmExtension;`
#[cfg (all (feature = "have_window_system", feature = "have_xpm", feature = "have_x_windows"))]
#[repr(C)]
pub struct XpmExtension
{
  pub name: *mut c_char,		/* name of the extension */
  pub nlines: c_uint,			/* number of lines in this extension */
  pub lines: *mut *mut c_char,		/* pointer to the extension array of strings */
}

/// C: xpm.h 89-96: `typedef struct { char *string; char *symbolic; char *m_color; char *g4_color; char *g_color; char *c_color; } XpmColor;`
#[cfg (all (feature = "have_window_system", feature = "have_xpm", feature = "have_x_windows"))]
#[repr(C)]
pub struct XpmColor
{
  pub string: *mut c_char,		/* characters string */
  pub symbolic: *mut c_char,		/* symbolic name */
  pub m_color: *mut c_char,		/* monochrom default */
  pub g4_color: *mut c_char,		/* 4 level grayscale default */
  pub g_color: *mut c_char,		/* other level grayscale default */
  pub c_color: *mut c_char,		/* color default */
}

/// C: xpm.h 112-117: `typedef int (*XpmAllocColorFunc)(Display*, Colormap, char*, XColor*, void*);`
#[cfg (all (feature = "have_window_system", feature = "have_xpm", feature = "have_x_windows"))]
pub type XpmAllocColorFunc = Option<unsafe extern "C" fn (display: *mut Display,
							  colormap: Colormap,
							  colorname: *mut c_char,
							  xcolor: *mut XColor,
							  closure: *mut c_void) -> c_int>;
/// C: xpm.h 119-126: `typedef int (*XpmFreeColorsFunc)(Display*, Colormap, Pixel*, int, void*);`
#[cfg (all (feature = "have_window_system", feature = "have_xpm", feature = "have_x_windows"))]
pub type XpmFreeColorsFunc = Option<unsafe extern "C" fn (display: *mut Display,
							  colormap: Colormap,
							  pixels: *mut Pixel,
							  npixels: c_int,
							  closure: *mut c_void) -> c_int>;

/* C 132-197: `struct XpmAttributes` - C's members and their order; the record's members are
   Xlib's `Visual`/`Colormap`/`Bool`/`XColor`, which `include/xlib.rs` carries under
   `have_x_windows` (the section's own gate above).  */
/// C: xpm.h 132-197: `typedef struct { unsigned long valuemask; Visual *visual; Colormap colormap; … } XpmAttributes;`
#[cfg (all (feature = "have_window_system", feature = "have_xpm", feature = "have_x_windows"))]
#[repr(C)]
pub struct XpmAttributes
{
  /// C 138: `unsigned long valuemask;`
  pub valuemask: c_ulong,
  /// C 140: `Visual *visual;`
  pub visual: *mut Visual,
  /// C 141: `Colormap colormap;`
  pub colormap: Colormap,
  /// C 142: `unsigned int depth;`
  pub depth: c_uint,
  /// C 143: `unsigned int width;`
  pub width: c_uint,
  /// C 144: `unsigned int height;`
  pub height: c_uint,
  /// C 145: `unsigned int x_hotspot;`
  pub x_hotspot: c_uint,
  /// C 147: `unsigned int y_hotspot;`
  pub y_hotspot: c_uint,
  /// C 149: `unsigned int cpp;`
  pub cpp: c_uint,
  /// C 150: `Pixel *pixels;`
  pub pixels: *mut Pixel,
  /// C 151: `unsigned int npixels;`
  pub npixels: c_uint,
  /// C 152: `XpmColorSymbol *colorsymbols;`
  pub colorsymbols: *mut XpmColorSymbol,
  /// C 153: `unsigned int numsymbols;`
  pub numsymbols: c_uint,
  /// C 154: `char *rgb_fname;`
  pub rgb_fname: *mut c_char,
  /// C 155: `unsigned int nextensions;`
  pub nextensions: c_uint,
  /// C 156: `XpmExtension *extensions;`
  pub extensions: *mut XpmExtension,
  /// C 158: `unsigned int ncolors;`
  pub ncolors: c_uint,
  /// C 159: `XpmColor *colorTable;`
  pub colorTable: *mut XpmColor,
  /// C 161: `char *hints_cmt;`
  pub hints_cmt: *mut c_char,
  /// C 162: `char *colors_cmt;`
  pub colors_cmt: *mut c_char,
  /// C 163: `char *pixels_cmt;`
  pub pixels_cmt: *mut c_char,
  /// C 165: `unsigned int mask_pixel;`
  pub mask_pixel: c_uint,
  /// C 169: `Bool exactColors;`
  pub exactColors: Bool,
  /// C 170: `unsigned int closeness;`
  pub closeness: c_uint,
  /// C 171: `unsigned int red_closeness;`
  pub red_closeness: c_uint,
  /// C 172: `unsigned int green_closeness;`
  pub green_closeness: c_uint,
  /// C 173: `unsigned int blue_closeness;`
  pub blue_closeness: c_uint,
  /// C 174: `int color_key;`
  pub color_key: c_int,
  /// C 176: `Pixel *alloc_pixels;`
  pub alloc_pixels: *mut Pixel,
  /// C 177: `int nalloc_pixels;`
  pub nalloc_pixels: c_int,
  /// C 180: `Bool alloc_close_colors;`
  pub alloc_close_colors: Bool,
  /// C 181: `int bitmap_format;`
  pub bitmap_format: c_int,
  /// C 184: `XpmAllocColorFunc alloc_color;`
  pub alloc_color: XpmAllocColorFunc,
  /// C 185: `XpmFreeColorsFunc free_colors;`
  pub free_colors: XpmFreeColorsFunc,
  /// C 186: `void *color_closure;`
  pub color_closure: *mut c_void,
}
#[cfg (all (feature = "have_window_system", feature = "have_xpm", feature = "have_x_windows"))]
const _: () = core::assert! (core::mem::size_of::<XpmAttributes> () == 224);
#[cfg (all (feature = "have_window_system", feature = "have_xpm", feature = "have_x_windows"))]
const _: () = core::assert! (core::mem::align_of::<XpmAttributes> () == 8);
#[cfg (all (feature = "have_window_system", feature = "have_xpm", feature = "have_x_windows"))]
const _: () = core::assert! (core::mem::offset_of! (XpmAttributes, colorsymbols) == 64);
#[cfg (all (feature = "have_window_system", feature = "have_xpm", feature = "have_x_windows"))]
const _: () = core::assert! (core::mem::offset_of! (XpmAttributes, numsymbols) == 72);
#[cfg (all (feature = "have_window_system", feature = "have_xpm", feature = "have_x_windows"))]
const _: () = core::assert! (core::mem::offset_of! (XpmAttributes, nalloc_pixels) == 184);
#[cfg (all (feature = "have_window_system", feature = "have_xpm", feature = "have_x_windows"))]
const _: () = core::assert! (core::mem::offset_of! (XpmAttributes, alloc_color) == 200);
#[cfg (all (feature = "have_window_system", feature = "have_xpm", feature = "have_x_windows"))]
const _: () = core::assert! (core::mem::offset_of! (XpmAttributes, color_closure) == 216);
#[cfg (all (feature = "have_window_system", feature = "have_xpm", feature = "have_x_windows"))]
const _: () = core::assert! (core::mem::size_of::<XpmColorSymbol> () == 24);
#[cfg (all (feature = "have_window_system", feature = "have_xpm", feature = "have_x_windows"))]
const _: () = core::assert! (core::mem::offset_of! (XpmColorSymbol, value) == 8);
#[cfg (all (feature = "have_window_system", feature = "have_xpm", feature = "have_x_windows"))]
const _: () = core::assert! (core::mem::offset_of! (XpmColorSymbol, pixel) == 16);

/* C 199-221: `XpmAttributes value masks bits` - the four this slice reads.  */
/// C: xpm.h 206: `#define XpmColorSymbols	   (1L<<6)`
#[cfg (all (feature = "have_window_system", feature = "have_xpm", feature = "have_x_windows"))]
pub const XpmColorSymbols: c_long = 1 << 6;
/// C: xpm.h 228: `#define XpmAllocColor      (1L<<19)`
#[cfg (all (feature = "have_window_system", feature = "have_xpm", feature = "have_x_windows"))]
pub const XpmAllocColor: c_long = 1 << 19;
/// C: xpm.h 229: `#define XpmFreeColors      (1L<<20)`
#[cfg (all (feature = "have_window_system", feature = "have_xpm", feature = "have_x_windows"))]
pub const XpmFreeColors: c_long = 1 << 20;
/// C: xpm.h 230: `#define XpmColorClosure    (1L<<21)`
#[cfg (all (feature = "have_window_system", feature = "have_xpm", feature = "have_x_windows"))]
pub const XpmColorClosure: c_long = 1 << 21;

/* The rest of C 6166-6613's `<X11/xpm.h>` surface.  C 74-78's five `Xpm*` results were measured
   with this machine's clang against the same upstream header as the numbers above (`docs/ports/image.md`,
   Evidence 21); C 200-202/224-225's four masks are the same table's.  The Windows arm of C
   5921-5942 (`noX/xpm.h`, whose `xpm_XImage`/`xpm_Display` no file in the reference tree
   carries) stays owed with that arm - the block it serves is `text-only` here.  */
/// C: xpm.h 74: `#define XpmSuccess       0`
#[cfg (all (feature = "have_window_system", feature = "have_xpm", feature = "have_x_windows"))]
pub const XpmSuccess: c_int = 0;
/// C: xpm.h 75: `#define XpmOpenFailed   -1`
#[cfg (all (feature = "have_window_system", feature = "have_xpm", feature = "have_x_windows"))]
pub const XpmOpenFailed: c_int = -1;
/// C: xpm.h 76: `#define XpmFileInvalid  -2`
#[cfg (all (feature = "have_window_system", feature = "have_xpm", feature = "have_x_windows"))]
pub const XpmFileInvalid: c_int = -2;
/// C: xpm.h 77: `#define XpmNoMemory     -3`
#[cfg (all (feature = "have_window_system", feature = "have_xpm", feature = "have_x_windows"))]
pub const XpmNoMemory: c_int = -3;
/// C: xpm.h 78: `#define XpmColorFailed  -4`
#[cfg (all (feature = "have_window_system", feature = "have_xpm", feature = "have_x_windows"))]
pub const XpmColorFailed: c_int = -4;
/// C: xpm.h 200: `#define XpmVisual	   (1L<<0)`
#[cfg (all (feature = "have_window_system", feature = "have_xpm", feature = "have_x_windows"))]
pub const XpmVisual: c_long = 1 << 0;
/// C: xpm.h 201: `#define XpmColormap	   (1L<<1)`
#[cfg (all (feature = "have_window_system", feature = "have_xpm", feature = "have_x_windows"))]
pub const XpmColormap: c_long = 1 << 1;
/// C: xpm.h 202: `#define XpmDepth	   (1L<<2)`
#[cfg (all (feature = "have_window_system", feature = "have_xpm", feature = "have_x_windows"))]
pub const XpmDepth: c_long = 1 << 2;
/// C: xpm.h 224: `#define XpmReturnAllocPixels (1L<<16)`
#[cfg (all (feature = "have_window_system", feature = "have_xpm", feature = "have_x_windows"))]
pub const XpmReturnAllocPixels: c_long = 1 << 16;
/// C: xpm.h 225: `#define XpmAllocCloseColors (1L<<17)`
#[cfg (all (feature = "have_window_system", feature = "have_xpm", feature = "have_x_windows"))]
pub const XpmAllocCloseColors: c_long = 1 << 17;

/// C: xpm.h 268-273: `FUNC(XpmCreatePixmapFromData, int, (Display *display, Drawable d, char **data, Pixmap *pixmap_return, Pixmap *shapemask_return, XpmAttributes *attributes));`
#[cfg (all (feature = "have_window_system", feature = "have_xpm", feature = "have_x_windows"))]
unsafe extern "C" {
  pub fn XpmCreatePixmapFromData (display: *mut Display, d: Drawable, data: *mut *mut c_char,
                                  pixmap_return: *mut Pixmap, shapemask_return: *mut Pixmap,
                                  attributes: *mut XpmAttributes) -> c_int;
}
/// C: xpm.h 306-310: `FUNC(XpmReadFileToImage, int, (Display *display, const char *filename, XImage **image_return, XImage **shapeimage_return, XpmAttributes *attributes));`
#[cfg (all (feature = "have_window_system", feature = "have_xpm", feature = "have_x_windows"))]
unsafe extern "C" {
  pub fn XpmReadFileToImage (display: *mut Display, filename: *const c_char,
                             image_return: *mut *mut XImage, shapeimage_return: *mut *mut XImage,
                             attributes: *mut XpmAttributes) -> c_int;
}
/// C: xpm.h 318-322: `FUNC(XpmCreateImageFromBuffer, int, (Display *display, char *buffer, XImage **image_return, XImage **shapemask_return, XpmAttributes *attributes));`
#[cfg (all (feature = "have_window_system", feature = "have_xpm", feature = "have_x_windows"))]
unsafe extern "C" {
  pub fn XpmCreateImageFromBuffer (display: *mut Display, buffer: *mut c_char,
                                   image_return: *mut *mut XImage,
                                   shapemask_return: *mut *mut XImage,
                                   attributes: *mut XpmAttributes) -> c_int;
}
/// C: xpm.h 350: `FUNC(XpmFreeAttributes, void, (XpmAttributes *attributes));`
#[cfg (all (feature = "have_window_system", feature = "have_xpm", feature = "have_x_windows"))]
unsafe extern "C" {
  pub fn XpmFreeAttributes (attributes: *mut XpmAttributes);
}

/* C 5944-5984: `#if defined HAVE_XPM || defined USE_CAIRO || defined HAVE_NS || defined
   HAVE_HAIKU || defined HAVE_ANDROID`'s keyword table, with C 5965's inner condition on the table
   itself.  */

/* Indices of image specification fields in xpm_format, below.  */

/// C: C 5948-5962: `enum xpm_keyword_index`
#[cfg (all (feature = "have_window_system", any (feature = "have_xpm", feature = "use_cairo",
						  feature = "have_ns", feature = "have_haiku",
						  feature = "have_android")))]
pub const XPM_TYPE: c_int = 0;
#[cfg (all (feature = "have_window_system", any (feature = "have_xpm", feature = "use_cairo",
						  feature = "have_ns", feature = "have_haiku",
						  feature = "have_android")))]
pub const XPM_FILE: c_int = 1;
#[cfg (all (feature = "have_window_system", any (feature = "have_xpm", feature = "use_cairo",
						  feature = "have_ns", feature = "have_haiku",
						  feature = "have_android")))]
pub const XPM_DATA: c_int = 2;
#[cfg (all (feature = "have_window_system", any (feature = "have_xpm", feature = "use_cairo",
						  feature = "have_ns", feature = "have_haiku",
						  feature = "have_android")))]
pub const XPM_ASCENT: c_int = 3;
#[cfg (all (feature = "have_window_system", any (feature = "have_xpm", feature = "use_cairo",
						  feature = "have_ns", feature = "have_haiku",
						  feature = "have_android")))]
pub const XPM_MARGIN: c_int = 4;
#[cfg (all (feature = "have_window_system", any (feature = "have_xpm", feature = "use_cairo",
						  feature = "have_ns", feature = "have_haiku",
						  feature = "have_android")))]
pub const XPM_RELIEF: c_int = 5;
#[cfg (all (feature = "have_window_system", any (feature = "have_xpm", feature = "use_cairo",
						  feature = "have_ns", feature = "have_haiku",
						  feature = "have_android")))]
pub const XPM_ALGORITHM: c_int = 6;
#[cfg (all (feature = "have_window_system", any (feature = "have_xpm", feature = "use_cairo",
						  feature = "have_ns", feature = "have_haiku",
						  feature = "have_android")))]
pub const XPM_HEURISTIC_MASK: c_int = 7;
#[cfg (all (feature = "have_window_system", any (feature = "have_xpm", feature = "use_cairo",
						  feature = "have_ns", feature = "have_haiku",
						  feature = "have_android")))]
pub const XPM_MASK: c_int = 8;
#[cfg (all (feature = "have_window_system", any (feature = "have_xpm", feature = "use_cairo",
						  feature = "have_ns", feature = "have_haiku",
						  feature = "have_android")))]
pub const XPM_COLOR_SYMBOLS: c_int = 9;
#[cfg (all (feature = "have_window_system", any (feature = "have_xpm", feature = "use_cairo",
						  feature = "have_ns", feature = "have_haiku",
						  feature = "have_android")))]
pub const XPM_BACKGROUND: c_int = 10;
#[cfg (all (feature = "have_window_system", any (feature = "have_xpm", feature = "use_cairo",
						  feature = "have_ns", feature = "have_haiku",
						  feature = "have_android")))]
pub const XPM_LAST: c_int = 11;

/* Vector of image_keyword structures describing the format
   of valid XPM image specifications.  */

/// C: C 5968-5969: `static const struct image_keyword xpm_format [XPM_LAST]`
#[cfg (all (feature = "have_window_system", any (feature = "have_xpm", feature = "have_ns",
						  feature = "have_haiku", feature = "have_pgtk",
						  feature = "have_android")))]
static xpm_format: [image_keyword; XPM_LAST as usize] =
[
  image_keyword { name: c":type".as_ptr (), r#type: IMAGE_SYMBOL_VALUE,
		  mandatory_p: true, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":file".as_ptr (), r#type: IMAGE_STRING_VALUE,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":data".as_ptr (), r#type: IMAGE_STRING_VALUE,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":ascent".as_ptr (), r#type: IMAGE_ASCENT_VALUE,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":margin".as_ptr (), r#type: IMAGE_NON_NEGATIVE_INTEGER_VALUE_OR_PAIR,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":relief".as_ptr (), r#type: IMAGE_INTEGER_VALUE,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":conversion".as_ptr (), r#type: IMAGE_DONT_CHECK_VALUE_TYPE,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":heuristic-mask".as_ptr (), r#type: IMAGE_DONT_CHECK_VALUE_TYPE,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":mask".as_ptr (), r#type: IMAGE_DONT_CHECK_VALUE_TYPE,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":color-symbols".as_ptr (), r#type: IMAGE_DONT_CHECK_VALUE_TYPE,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":background".as_ptr (), r#type: IMAGE_STRING_OR_NIL_VALUE,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
];

/* C 5986-5996: `#if defined HAVE_X_WINDOWS && !defined USE_CAIRO`'s derivation of
   `ALLOC_XPM_COLORS` (C 5993-5995's `#if defined XpmAllocColor && defined XpmFreeColors &&
   defined XpmColorClosure`).  The three are `<X11/xpm.h>`'s own names, so the port spells the
   derived condition as the feature `alloc_xpm_colors` (P-51's "name the macro"; the manifest does
   not carry it yet - the ledger's owed list).  */

/* C 5998-6161: `#ifdef ALLOC_XPM_COLORS`, the colour cache.  */

/// C: C 6000-6001: `static struct xpm_cached_color *xpm_cache_color (struct frame *, char *, XColor *, int);`
/// - a `static` prototype whose definition is C 6089-6102's, inside this slice: dropped
/// (PORTING.md 0.1-12).

/* An entry in a hash table used to cache color definitions of named
   colors.  This cache is necessary to speed up XPM image loading in
   case we do color allocations ourselves.  Without it, we would need
   a call to XParseColor per pixel in the image.

   FIXME Now that we're using x_parse_color and its cache, reevaluate
   the need for this caching layer.  */

/// C: C 6012-6018: `struct xpm_cached_color`
#[cfg (all (feature = "have_window_system", feature = "have_x_windows", not (feature = "use_cairo"),
	    feature = "alloc_xpm_colors"))]
#[repr(C)]
pub struct xpm_cached_color
{
  /* Next in collision chain.  */
  pub next: *mut xpm_cached_color,

  /* Color definition (RGB and pixel color).  */
  pub color: XColor,

  /* Color name.  */
  pub name: [c_char; 0],               // FLEXIBLE_ARRAY_MEMBER
}

/* The hash table used for the color cache, and its bucket vector
   size (which should be prime).  */

/// C: C 6023: `#define XPM_COLOR_CACHE_BUCKETS 1009`
#[cfg (all (feature = "have_window_system", feature = "have_x_windows", not (feature = "use_cairo"),
	    feature = "alloc_xpm_colors"))]
pub const XPM_COLOR_CACHE_BUCKETS: c_int = 1009;
/// C: C 6024: `static struct xpm_cached_color **xpm_color_cache;` - C's tentative definition.
#[cfg (all (feature = "have_window_system", feature = "have_x_windows", not (feature = "use_cairo"),
	    feature = "alloc_xpm_colors"))]
static mut xpm_color_cache: *mut *mut xpm_cached_color = core::ptr::null_mut ();

/* Initialize the color cache.  */

/// C: C 6028-6029: `static void xpm_init_color_cache (struct frame *f, XpmAttributes *attrs)`
#[cfg (all (feature = "have_window_system", feature = "have_x_windows", not (feature = "use_cairo"),
	    feature = "alloc_xpm_colors"))]
unsafe fn xpm_init_color_cache (f: *mut frame, attrs: *mut XpmAttributes)
{
  let nbytes: ptrdiff_t = XPM_COLOR_CACHE_BUCKETS as ptrdiff_t
			  * core::mem::size_of::<*mut xpm_cached_color> () as ptrdiff_t;
  xpm_color_cache = xzalloc (nbytes as usize) as *mut *mut xpm_cached_color;
  init_color_table ();

  if (*attrs).valuemask & XpmColorSymbols as c_ulong != 0
    {
      let mut i: c_int;
      /* C's `XColor color;` - zeroed (PORTING.md 1.5).  */
      let mut color: XColor = core::mem::zeroed ();

      i = 0;
      while (i as c_uint) < (*attrs).numsymbols
	{
	  if x_parse_color (f, (*(*attrs).colorsymbols.offset (i as isize)).value,
			    &raw mut color) != 0
	    {
	      color.pixel = lookup_rgb_color (f, color.red as c_int, color.green as c_int,
					      color.blue as c_int);
	      xpm_cache_color (f, (*(*attrs).colorsymbols.offset (i as isize)).name,
			       &raw mut color, -1);
	    }
	  i += 1;
	}
    }
}

/* Free the color cache.  */

/// C: C 6053-6054: `static void xpm_free_color_cache (void)`
#[cfg (all (feature = "have_window_system", feature = "have_x_windows", not (feature = "use_cairo"),
	    feature = "alloc_xpm_colors"))]
unsafe fn xpm_free_color_cache ()
{
  let mut p: *mut xpm_cached_color;
  let mut next: *mut xpm_cached_color;
  let mut i: c_int;

  i = 0;
  while i < XPM_COLOR_CACHE_BUCKETS
    {
      p = *xpm_color_cache.offset (i as isize);
      while !p.is_null ()
	{
	  next = (*p).next;
	  xfree (p as *mut c_void);
	  p = next;
	}
      i += 1;
    }

  xfree (xpm_color_cache as *mut c_void);
  xpm_color_cache = core::ptr::null_mut ();
  free_color_table ();
}

/* Return the bucket index for color named COLOR_NAME in the color
   cache.  */

/// C: C 6074-6075: `static int xpm_color_bucket (char *color_name)`
#[cfg (all (feature = "have_window_system", feature = "have_x_windows", not (feature = "use_cairo"),
	    feature = "alloc_xpm_colors"))]
unsafe fn xpm_color_bucket (color_name: *mut c_char) -> c_int
{
  let hash: EMACS_UINT = hash_char_array (color_name, strlen (color_name) as ptrdiff_t);
  return (hash % XPM_COLOR_CACHE_BUCKETS as EMACS_UINT) as c_int;
}

/* On frame F, cache values COLOR for color with name COLOR_NAME.
   BUCKET, if >= 0, is a precomputed bucket index.  Value is the cache
   entry added.  */

/// C: C 6086-6088: `static struct xpm_cached_color * xpm_cache_color (struct frame *f, char *color_name, XColor *color, int bucket)`
#[cfg (all (feature = "have_window_system", feature = "have_x_windows", not (feature = "use_cairo"),
	    feature = "alloc_xpm_colors"))]
unsafe fn xpm_cache_color (f: *mut frame, color_name: *mut c_char, color: *mut XColor,
			   mut bucket: c_int) -> *mut xpm_cached_color
{
  if bucket < 0
    { bucket = xpm_color_bucket (color_name); }

  let len: ptrdiff_t = strlen (color_name) as ptrdiff_t + 1;
  let nbytes: ptrdiff_t = FLEXSIZEOF! (xpm_cached_color, name, len);
  let p: *mut xpm_cached_color = xmalloc (nbytes as usize) as *mut xpm_cached_color;
  memcpy (fam_base (core::ptr::addr_of_mut! ((*p).name)) as *mut c_void,
	  color_name as *const c_void, len as usize);
  (*p).color = *color;
  (*p).next = *xpm_color_cache.offset (bucket as isize);
  *xpm_color_cache.offset (bucket as isize) = p;
  return p;
}

/* Look up color COLOR_NAME for frame F in the color cache.  If found,
   return the cached definition in *COLOR.  Otherwise, make a new
   entry in the cache and allocate the color.  Value is false if color
   allocation failed.  */

/// C: C 6108-6109: `static bool xpm_lookup_color (struct frame *f, char *color_name, XColor *color)`
#[cfg (all (feature = "have_window_system", feature = "have_x_windows", not (feature = "use_cairo"),
	    feature = "alloc_xpm_colors"))]
unsafe fn xpm_lookup_color (f: *mut frame, color_name: *mut c_char, color: *mut XColor) -> bool
{
  let mut p: *mut xpm_cached_color;
  let h: c_int = xpm_color_bucket (color_name);

  p = *xpm_color_cache.offset (h as isize);
  while !p.is_null ()
    {
      if strcmp (fam_base (core::ptr::addr_of_mut! ((*p).name)), color_name) == 0
	{ break; }
      p = (*p).next;
    }

  if !p.is_null ()
    { *color = (*p).color; }
  else if x_parse_color (f, color_name, color) != 0
    {
      (*color).pixel = lookup_rgb_color (f, (*color).red as c_int, (*color).green as c_int,
					 (*color).blue as c_int);
      p = xpm_cache_color (f, color_name, color, h);
    }
  /* You get `opaque' at least from ImageMagick converting pbm to xpm
     with transparency, and it's useful.  */
  else if strcmp (c"opaque".as_ptr (), color_name) == 0
    {
      memset (color as *mut c_void, 0, core::mem::size_of::<XColor> ());  /* Is this necessary/correct?  */
      (*color).pixel = FRAME_FOREGROUND_PIXEL (f);
      p = xpm_cache_color (f, color_name, color, h);
    }

  return !p.is_null ();
}

/* Callback for allocating color COLOR_NAME.  Called from the XPM lib.
   CLOSURE is a pointer to the frame on which we allocate the
   color.  Return in *COLOR the allocated color.  Value is non-zero
   if successful.  */

/// C: C 6143-6145: `static int xpm_alloc_color (Display *dpy, Colormap cmap, char *color_name, XColor *color, void *closure)`
#[cfg (all (feature = "have_window_system", feature = "have_x_windows", not (feature = "use_cairo"),
	    feature = "alloc_xpm_colors"))]
unsafe extern "C" fn xpm_alloc_color (dpy: *mut Display, cmap: Colormap,
				      color_name: *mut c_char, color: *mut XColor,
				      closure: *mut c_void) -> c_int
{
  return xpm_lookup_color (closure as *mut frame, color_name, color) as c_int;
}

/* Callback for freeing NPIXELS colors contained in PIXELS.  CLOSURE
   is a pointer to the frame on which we allocate the color.  Value is
   non-zero if successful.  */

/// C: C 6154-6156: `static int xpm_free_colors (Display *dpy, Colormap cmap, Pixel *pixels, int npixels, void *closure)`
#[cfg (all (feature = "have_window_system", feature = "have_x_windows", not (feature = "use_cairo"),
	    feature = "alloc_xpm_colors"))]
unsafe extern "C" fn xpm_free_colors (dpy: *mut Display, cmap: Colormap, pixels: *mut Pixel,
				      npixels: c_int, closure: *mut c_void) -> c_int
{
  return 1;
}

/* The names this slice calls that the mirrors this unit includes do not carry, declared at the
   use site with the header's own line (PORTING.md P-55/P-72 rule 3 / P-88 ①); `docs/ports/image.md`
   names the file each belongs in, and the same commit that lands a definition or a mirror
   declaration deletes its copy here.  */
/// C: dispextern.h 104-111: `INLINE int xstrcasecmp (char const *a, char const *b) { return c_strcasecmp (a, b); }` - the header's inline, which C 6987/6998 of this slice's `xpm_load_image` read.  The mirror keeps its own copy in `dispextern_externs` (a module the including unit does not glob-import, measured there as needed against `src/font.rs`'s definition), so the unit carries C's text here (the `src/font.rs` 298 shape).
#[cfg (feature = "have_window_system")]
#[inline(always)]
pub unsafe fn xstrcasecmp (a: *const c_char, b: *const c_char) -> c_int
{
  return c_strcasecmp (a, b);
}
// C: xterm.h 1796: `extern bool x_alloc_nearest_color (struct frame *, Colormap, XColor *);`
// - the definition is `xterm.c`'s; `include/xterm.rs` does not carry the declaration.
#[cfg (all (feature = "have_window_system", feature = "have_x_windows",
	    not (any (feature = "have_ntgui", feature = "use_cairo", feature = "have_ns"))))]
unsafe extern "C" {
  pub fn x_alloc_nearest_color (f: *mut frame, cmap: Colormap, color: *mut XColor) -> bool;
}
// C: xterm.h 1797: `extern void x_query_colors (struct frame *f, XColor *, int);` - the
// definition is `xterm.c`'s; `include/xterm.rs` does not carry the declaration.
#[cfg (all (feature = "have_window_system", feature = "have_x_windows",
	    not (any (feature = "have_ntgui", feature = "use_cairo", feature = "have_ns"))))]
unsafe extern "C" {
  pub fn x_query_colors (f: *mut frame, color: *mut XColor, n: c_int);
}
// C: dispextern.h 3838-3839: `#ifdef HAVE_X_WINDOWS` / `void gamma_correct (struct frame *, XColor *);`
// - the definition is `xfaces.c`'s; `include/dispextern.rs` does not carry the declaration.
#[cfg (all (feature = "have_window_system", feature = "have_x_windows"))]
unsafe extern "C" {
  pub fn gamma_correct (f: *mut frame, color: *mut XColor);
}
// C: Xlib.h 1292-1294: `extern void XSetForeground (Display *display, GC gc, unsigned long foreground);`
// - `include/xlib.rs` carries the surface `src/xrdb.c` needed and stops before this entry point.
#[cfg (all (feature = "have_window_system", feature = "have_x_windows",
	    not (feature = "use_cairo")))]
unsafe extern "C" {
  pub fn XSetForeground (display: *mut Display, gc: GC, foreground: c_ulong) -> c_int;
}
// C: Xlib.h 1526-1529: `extern int XDrawLine (Display *display, Drawable d, GC gc, int x1, int y1, int x2, int y2);`
#[cfg (all (feature = "have_window_system", feature = "have_x_windows",
	    not (feature = "use_cairo")))]
unsafe extern "C" {
  pub fn XDrawLine (display: *mut Display, d: Drawable, gc: GC, x1: c_int, y1: c_int, x2: c_int,
                    y2: c_int) -> c_int;
}
// C: string.h: `extern char *strtok (char *__restrict, const char *__restrict);` - this
// machine's `<string.h>` measures the declaration at 209; `lib/include/string.rs` does not carry
// it (that mirror is included at C 902/951/1041's reach).  `xpm_load_image` (C 6946) is the reader.
#[cfg (feature = "have_window_system")]
unsafe extern "C" {
  pub fn strtok (s: *mut c_char, delim: *const c_char) -> *mut c_char;
}
// C: androidgui.h 926-927: `extern struct android_gc *android_create_gc (enum android_gc_value_mask, struct android_gc_values *);`
// - `include/androidgui.rs` carries the other three `android_*_gc` names this slice reads and not
// this one.
#[cfg (all (feature = "have_window_system", feature = "have_android"))]
unsafe extern "C" {
  pub fn android_create_gc (mask: android_gc_value_mask, values: *mut android_gc_values)
    -> *mut android_gc;
}
/* C: dispextern.h 134-138: `#ifdef HAVE_NTGUI` / `typedef HDC Emacs_Pix_Context;` - the arm is
   already declared by the first slice's use-site block in this file (`*mut c_void`, the mirror's
   own spelling of the handle), so this slice adds nothing here.  */

/* ==============================================================================================
   C 6166-6204: `#ifdef WINDOWSNT` - the XPM library's lazy DLL binding (P-86 ①).

   C's `DEF_DLL_FN`/`LOAD_DLL_FN` are `w32common.h` 77-97's macros; that header's mirror leaves
   the three deferred (P-22: Rust has no token pasting and no `__VA_ARGS__`), so this unit
   declares its own `DEF_DLL_FN`/`LOAD_DLL_FN` of the five-argument shape the first consumer
   settled (`src/decompress.rs` 67, `src/lcms.rs` 70, `src/comp.rs` 112-150): C pastes
   `W32_PFN_##func` and `fn_##func` from one identifier, which no `macro_rules!` can do, so both
   generated names are parameters, and C's `__VA_ARGS__` (the parameter list) is the `$args`
   fragment.  The names this block calls (`get_proc_addr`, `VOIDFNPTR`, `w32_delayed_load`)
   reach the TU through C 131-135's nested `#ifdef WINDOWSNT` include pair, which carries
   `have_ntgui` on top of `windowsnt`; the block is `text-only` on this machine (no Windows
   toolchain, P-86 ③-⑵) and its `xpm_XImage` is the `noX/xpm.h` shim's name, owed with that arm
   (`docs/ports/image.md`).  */

/// C: w32common.h 77-79: `#define DEF_DLL_FN(type, func, ...)  typedef type (CDECL *W32_PFN_##func) __VA_ARGS__; static W32_PFN_##func fn_##func` - the five-argument shape above; `CDECL` is the `__cdecl` attribute, empty on this target's ABI.
#[cfg (all (feature = "have_window_system", feature = "windowsnt"))]
macro_rules! DEF_DLL_FN {
  ($type:ty, $func:ident, $pfn:ident, $fnvar:ident, ($($arg:ty),* $(,)?)) => {
    pub type $pfn = Option<unsafe extern "C" fn ($($arg),*) -> $type>;
    static mut $fnvar: $pfn = None;
  };
}
/// C: w32common.h 82-89: `#define LOAD_DLL_FN(lib, func)       do { fn_##func = (W32_PFN_##func) get_proc_addr (lib, #func); if (!fn_##func) return false; } while (false)` - C's `do { … } while (false)` is a block expression (PORTING.md 2.1) and its `#func` is the stringified name, a parameter here for the same reason as above.
#[cfg (all (feature = "have_window_system", feature = "windowsnt"))]
macro_rules! LOAD_DLL_FN {
  ($lib:expr, $func:ident, $pfn:ident, $fnvar:ident, $name:expr) => {
    {
      $fnvar = std::mem::transmute::<VOIDFNPTR, $pfn> (get_proc_addr ($lib, $name.as_ptr ()));
      if $fnvar.is_none () { return false; }
    }
  };
}

/* XPM library details.  */
#[cfg (all (feature = "have_window_system", feature = "windowsnt"))]
DEF_DLL_FN! ((), XpmFreeAttributes, W32_PFN_XpmFreeAttributes, fn_XpmFreeAttributes,
             (*mut XpmAttributes));
#[cfg (all (feature = "have_window_system", feature = "windowsnt"))]
DEF_DLL_FN! (c_int, XpmCreateImageFromBuffer, W32_PFN_XpmCreateImageFromBuffer,
             fn_XpmCreateImageFromBuffer,
             (*mut Display, *mut c_char, *mut *mut xpm_XImage, *mut *mut xpm_XImage,
              *mut XpmAttributes));
#[cfg (all (feature = "have_window_system", feature = "windowsnt"))]
DEF_DLL_FN! (c_int, XpmReadFileToImage, W32_PFN_XpmReadFileToImage, fn_XpmReadFileToImage,
             (*mut Display, *mut c_char, *mut *mut xpm_XImage, *mut *mut xpm_XImage,
              *mut XpmAttributes));
#[cfg (all (feature = "have_window_system", feature = "windowsnt"))]
DEF_DLL_FN! ((), XImageFree, W32_PFN_XImageFree, fn_XImageFree, (*mut xpm_XImage));

/// C: C 6179-6180: `static bool init_xpm_functions (void)`.  C's `HMODULE library;` plus `if (!(library = w32_delayed_load (Qxpm))) return 0;` is the port's declaration-with-initialiser and null test (the `init_gccjit_functions` shape, `src/comp.rs` 279-364).
#[cfg (all (feature = "have_window_system", feature = "windowsnt"))]
unsafe fn init_xpm_functions () -> bool
{
  let library: *mut c_void = w32_delayed_load (Qxpm ());

  if library.is_null ()
    { return false; }

  #[cfg (all (feature = "have_window_system", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, XpmFreeAttributes, W32_PFN_XpmFreeAttributes, fn_XpmFreeAttributes,
                c"XpmFreeAttributes");
  #[cfg (all (feature = "have_window_system", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, XpmCreateImageFromBuffer, W32_PFN_XpmCreateImageFromBuffer,
                fn_XpmCreateImageFromBuffer, c"XpmCreateImageFromBuffer");
  #[cfg (all (feature = "have_window_system", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, XpmReadFileToImage, W32_PFN_XpmReadFileToImage,
                fn_XpmReadFileToImage, c"XpmReadFileToImage");
  #[cfg (all (feature = "have_window_system", feature = "windowsnt"))]
  LOAD_DLL_FN! (library, XImageFree, W32_PFN_XImageFree, fn_XImageFree, c"XImageFree");
  return true;
}

/* C 6194-6197: `# undef XImageFree` and its three companions, which cancel `<X11/xpm.h>`'s own
   declarations so that the four `# define`s below can rename each call to this unit's pointer
   variable.  They have no Rust counterpart (there is nothing to undef - the port never writes
   the header's macros; the same reading `src/comp.rs` 105-114 records).  */
/* C 6199-6202: `# define XImageFree fn_XImageFree` and its three companions (`XpmCreateImageFromBuffer`
   -> `fn_XpmCreateImageFromBuffer`, `XpmFreeAttributes` -> `fn_XpmFreeAttributes`,
   `XpmReadFileToImage` -> `fn_XpmReadFileToImage`).  C's preprocessor leaves every call site in
   `xpm_load` below spelled with the *pointer* name, which is what the port writes there.  */

/* C 6206-6245: `#if defined HAVE_XPM || defined HAVE_NS || defined HAVE_HAIKU || defined
   HAVE_PGTK || defined HAVE_ANDROID` - the two XPM image-type predicates.  */
/// C: C 6206-6207: `#if defined HAVE_XPM || defined HAVE_NS || defined HAVE_HAIKU || defined HAVE_PGTK || defined HAVE_ANDROID`
#[cfg (all (feature = "have_window_system", any (feature = "have_xpm", feature = "have_ns",
						  feature = "have_haiku", feature = "have_pgtk",
						  feature = "have_android")))]
/// C: C 6212-6213: `static bool xpm_valid_color_symbols_p (Lisp_Object color_symbols)`
unsafe fn xpm_valid_color_symbols_p (mut color_symbols: Lisp_Object) -> bool
{
  while CONSP (color_symbols)
    {
      let sym: Lisp_Object = XCAR (color_symbols);
      if !CONSP (sym)
	|| !STRINGP (XCAR (sym))
	|| !STRINGP (XCDR (sym))
	{ break; }
      color_symbols = XCDR (color_symbols);
    }

  return NILP (color_symbols);
}

/// C: C 6230-6231: `static bool xpm_image_p (Lisp_Object object)`
#[cfg (all (feature = "have_window_system", any (feature = "have_xpm", feature = "have_ns",
						  feature = "have_haiku", feature = "have_pgtk",
						  feature = "have_android")))]
unsafe extern "C" fn xpm_image_p (object: Lisp_Object) -> bool
{
  /* C's `struct image_keyword fmt[XPM_LAST];` filled by the `memcpy` below - zeroed first
     (PORTING.md 1.5/0.1-13).  */
  let mut fmt: [image_keyword; XPM_LAST as usize] = core::mem::zeroed ();

  /* C's `memcpy (fmt, xpm_format, sizeof fmt);`.  */
  memcpy (fmt.as_mut_ptr () as *mut c_void, xpm_format.as_ptr () as *const c_void,
	  core::mem::size_of::<[image_keyword; XPM_LAST as usize]> ());
  return (parse_image_spec (object, fmt.as_mut_ptr (), XPM_LAST, Qxpm ())
	  /* Either `:file' or `:data' must be present.  */
	  /* The mirror's `count` is a `bool` (`image_keyword`), so C's int sum is written with
	     the conversion out.  */
	  && (fmt [XPM_FILE as usize].count as c_int)
	     + (fmt [XPM_DATA as usize].count as c_int) == 1
	  /* Either no `:color-symbols' or it's a list of conses
	     whose car and cdr are strings.  */
	  && (! fmt [XPM_COLOR_SYMBOLS as usize].count
	      || xpm_valid_color_symbols_p (fmt [XPM_COLOR_SYMBOLS as usize].value)));
}

/* C 6247-6299: `#if defined HAVE_XPM && defined HAVE_X_WINDOWS && !defined USE_GTK` - the
   X11-only bitmap builder.  Non-`static`, so it keeps C's symbol (PORTING.md 0.1-3); its C
   prototype is `dispextern.h` 3736's, which `include/dispextern.rs` does not carry yet (the
   ledger records the gap - `xterm.c` 26153 is the other caller).  */
/// C: C 6248-6249: `ptrdiff_t x_create_bitmap_from_xpm_data (struct frame *f, const char **bits)`
#[cfg (all (feature = "have_window_system", feature = "have_xpm", feature = "have_x_windows",
	    not (feature = "use_gtk")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_create_bitmap_from_xpm_data (f: *mut frame,
							bits: *mut *const c_char) -> ptrdiff_t
{
  let dpyinfo: *mut Display_Info = FRAME_DISPLAY_INFO! (f);
  let id: ptrdiff_t;
  let mut rc: c_int;
  /* C's `XpmAttributes attrs;` + `memset (&attrs, 0, sizeof attrs);` - the record is zeroed
     first, then C's memset (PORTING.md 1.5/0.1-13).  */
  let mut attrs: XpmAttributes = core::mem::zeroed ();
  let mut bitmap: Pixmap;
  let mut mask: Pixmap;

  memset (&raw mut attrs as *mut c_void, 0, core::mem::size_of::<XpmAttributes> ());

  attrs.visual = FRAME_X_VISUAL! (f);
  attrs.colormap = FRAME_X_COLORMAP! (f);
  attrs.valuemask |= XpmVisual as c_ulong;
  attrs.valuemask |= XpmColormap as c_ulong;

  #[cfg (all (feature = "have_window_system", feature = "have_x_windows",
	      not (feature = "use_cairo"), feature = "alloc_xpm_colors"))]
  {
    attrs.color_closure = f as *mut c_void;
    attrs.alloc_color = Some (xpm_alloc_color);
    attrs.free_colors = Some (xpm_free_colors);
    attrs.valuemask |= (XpmAllocColor | XpmFreeColors | XpmColorClosure) as c_ulong;
    xpm_init_color_cache (f, &raw mut attrs);
  }

  rc = XpmCreatePixmapFromData (FRAME_X_DISPLAY! (f), FRAME_X_DRAWABLE! (f),
				bits as *mut *mut c_char, &raw mut bitmap, &raw mut mask,
				&raw mut attrs);
  if rc != XpmSuccess
    {
      XpmFreeAttributes (&raw mut attrs);
      return -1;
    }

  id = image_allocate_bitmap_record (f);
  (*((*dpyinfo).bitmaps.offset (id - 1))).pixmap = bitmap;
  (*((*dpyinfo).bitmaps.offset (id - 1))).have_mask = true;
  (*((*dpyinfo).bitmaps.offset (id - 1))).mask = mask;
  (*((*dpyinfo).bitmaps.offset (id - 1))).file = core::ptr::null_mut ();
  (*((*dpyinfo).bitmaps.offset (id - 1))).height = attrs.height;
  (*((*dpyinfo).bitmaps.offset (id - 1))).width = attrs.width;
  (*((*dpyinfo).bitmaps.offset (id - 1))).depth = attrs.depth;
  (*((*dpyinfo).bitmaps.offset (id - 1))).refcount = 1;
  #[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
  { (*((*dpyinfo).bitmaps.offset (id - 1))).stipple = core::ptr::null_mut (); }

  #[cfg (all (feature = "have_window_system", feature = "have_x_windows",
	      not (feature = "use_cairo"), feature = "alloc_xpm_colors"))]
  { xpm_free_color_cache (); }
  XpmFreeAttributes (&raw mut attrs);
  return id;
}

/* C 6301-6302's comment and `xpm_load` (C 6304's `#if defined HAVE_XPM && !defined USE_CAIRO`)
   - the plain-X11 XPM loader, with C 6314's nested `HAVE_NTGUI` arm (the `noX/xpm.h` shim's
   names, `text-only` on this machine; P-86 ③-⑵).  */
/* Load image IMG which will be displayed on frame F.  Value is
   true if successful.  */
/// C: C 6306-6307: `static bool xpm_load (struct frame *f, struct image *img)`
#[cfg (all (feature = "have_window_system", feature = "have_xpm", not (feature = "use_cairo")))]
unsafe extern "C" fn xpm_load (f: *mut frame, img: *mut image) -> bool
{
  let mut rc: c_int;
  /* C's `XpmAttributes attrs;` (zeroed, PORTING.md 1.5) - the `memset` below is C's own.  */
  let mut attrs: XpmAttributes = core::mem::zeroed ();
  let mut specified_file: Lisp_Object;
  let mut color_symbols: Lisp_Object;
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);

  #[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
  let mut hdc: HDC = core::ptr::null_mut ();
  #[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
  let mut xpm_image: *mut xpm_XImage = core::ptr::null_mut ();
  #[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
  let mut xpm_mask: *mut xpm_XImage = core::ptr::null_mut ();

  /* Configure the XPM lib.  Use the visual of frame F.  Allocate
     close colors.  Return colors allocated.  */
  memset (&raw mut attrs as *mut c_void, 0, core::mem::size_of::<XpmAttributes> ());

  #[cfg (all (feature = "have_window_system", not (feature = "have_ntgui")))]
  {
    attrs.visual = FRAME_X_VISUAL! (f);
    attrs.colormap = FRAME_X_COLORMAP! (f);
    /* C's `#define n_planes n_image_planes` (C 219-221) applies under HAVE_ANDROID; the
       statement is written the way C's preprocessor leaves it.  */
    #[cfg (all (feature = "have_window_system", not (feature = "have_android")))]
    { attrs.depth = (*FRAME_DISPLAY_INFO! (f)).n_planes as c_uint; }
    #[cfg (all (feature = "have_window_system", feature = "have_android"))]
    { attrs.depth = (*FRAME_DISPLAY_INFO! (f)).n_image_planes as c_uint; }
    attrs.valuemask |= XpmVisual as c_ulong;
    attrs.valuemask |= XpmColormap as c_ulong;
    attrs.valuemask |= XpmDepth as c_ulong;
  }

  #[cfg (all (feature = "have_window_system", feature = "have_x_windows",
	      not (feature = "use_cairo"), feature = "alloc_xpm_colors"))]
  {
    /* Allocate colors with our own functions which handle
       failing color allocation more gracefully.  */
    attrs.color_closure = f as *mut c_void;
    attrs.alloc_color = Some (xpm_alloc_color);
    attrs.free_colors = Some (xpm_free_colors);
    attrs.valuemask |= (XpmAllocColor | XpmFreeColors | XpmColorClosure) as c_ulong;
  }
  #[cfg (not (all (feature = "have_window_system", feature = "have_x_windows",
		   not (feature = "use_cairo"), feature = "alloc_xpm_colors")))]
  {
    /* Let the XPM lib allocate colors.  */
    attrs.valuemask |= XpmReturnAllocPixels as c_ulong;
    /* C 6342-6348: `#ifdef XpmAllocCloseColors` ... `#else` ... `#endif`.  C's condition is
       `<X11/xpm.h>` 225's own `#define XpmAllocCloseColors (1L<<17)`, a header constant the
       frozen macro table cannot carry; C's live arm with that header is written here, and the
       `#else` arm (`attrs.closeness = 600;` + `XpmCloseness`) is recorded in
       `docs/ports/image.md`'s Deviations.  */
    attrs.alloc_close_colors = 1;
    attrs.valuemask |= XpmAllocCloseColors as c_ulong;
  }

  /* If image specification contains symbolic color definitions, add
     these to `attrs'.  */
  color_symbols = image_spec_value ((*img).spec, QCcolor_symbols (), core::ptr::null_mut ());
  if CONSP (color_symbols)
    {
      let mut tail: Lisp_Object;
      let mut xpm_syms: *mut XpmColorSymbol;
      let mut i: ptrdiff_t;
      let size: ptrdiff_t;

      attrs.valuemask |= XpmColorSymbols as c_ulong;

      /* Count number of symbols.  */
      attrs.numsymbols = 0;
      tail = color_symbols;
      while CONSP (tail)
	{
	  attrs.numsymbols += 1;
	  tail = XCDR (tail);
	}

      /* Allocate an XpmColorSymbol array.  */
      xpm_syms = core::ptr::null_mut ();
      SAFE_NALLOCA! (sa_avail, xpm_syms, 1, attrs.numsymbols);
      size = (attrs.numsymbols as ptrdiff_t * core::mem::size_of::<XpmColorSymbol> ()) as ptrdiff_t;
      memset (xpm_syms as *mut c_void, 0, size as usize);
      attrs.colorsymbols = xpm_syms;

      /* Fill the color symbol array.  */
      tail = color_symbols;
      i = 0;
      while CONSP (tail)
	{
	  let name: Lisp_Object;
	  let color: Lisp_Object;
	  let empty_string: *mut c_char = c"".as_ptr () as *mut c_char;

	  if !CONSP (XCAR (tail))
	    {
	      (*xpm_syms.offset (i as isize)).name = empty_string;
	      (*xpm_syms.offset (i as isize)).value = empty_string;
	      i += 1;
	      tail = XCDR (tail);
	      continue;
	    }
	  name = XCAR (XCAR (tail));
	  color = XCDR (XCAR (tail));
	  if STRINGP (name)
	    {
	      /* C's `SAFE_ALLOCA_STRING (xpm_syms[i].name, name);` - the mirror's macro takes an
		 `$ptr:ident` and C passes an array-element lvalue, so its expansion stands here
		 (the `fileio.rs` 1034 call sites are the ident form).  */
	      (*xpm_syms.offset (i as isize)).name
		= SAFE_ALLOCA! (sa_avail, SBYTES (name) + 1) as *mut c_char;
	      std::ptr::copy_nonoverlapping (SDATA (name),
					     (*xpm_syms.offset (i as isize)).name as *mut u8,
					     (SBYTES (name) + 1) as usize);
	    }
	  else
	    { (*xpm_syms.offset (i as isize)).name = empty_string; }
	  if STRINGP (color)
	    {
	      (*xpm_syms.offset (i as isize)).value
		= SAFE_ALLOCA! (sa_avail, SBYTES (color) + 1) as *mut c_char;
	      std::ptr::copy_nonoverlapping (SDATA (color),
					     (*xpm_syms.offset (i as isize)).value as *mut u8,
					     (SBYTES (color) + 1) as usize);
	    }
	  else
	    { (*xpm_syms.offset (i as isize)).value = empty_string; }
	  i += 1;
	  tail = XCDR (tail);
	}
    }

  /* Create a pixmap for the image, either from a file, or from a
     string buffer containing data in the same format as an XPM file.  */
  #[cfg (all (feature = "have_window_system", feature = "have_x_windows",
	      not (feature = "use_cairo"), feature = "alloc_xpm_colors"))]
  { xpm_init_color_cache (f, &raw mut attrs); }

  specified_file = image_spec_value ((*img).spec, QCfile (), core::ptr::null_mut ());

  #[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
  {
    let frame_dc: HDC = get_frame_dc (f);
    hdc = CreateCompatibleDC (frame_dc);
    release_frame_dc (f, frame_dc);
  }

  if STRINGP (specified_file)
    {
      let mut file: Lisp_Object = image_find_image_file (specified_file);
      if !STRINGP (file)
	{
	  image_not_found_error (specified_file);
	  #[cfg (all (feature = "have_window_system", feature = "have_x_windows",
		      not (feature = "use_cairo"), feature = "alloc_xpm_colors"))]
	  { xpm_free_color_cache (); }
	  SAFE_FREE! (sa_count, sa_mark);
	  return false;
	}

      file = ENCODE_FILE (file);
      #[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
      {
	#[cfg (all (feature = "have_window_system", feature = "windowsnt"))]
	{
	  /* FILE is encoded in UTF-8, but image libraries on Windows
	     support neither UTF-8 nor UTF-16 encoded file names.  So we
	     need to re-encode it in ANSI.  */
	  file = ansi_encode_filename (file);
	}
	/* XpmReadFileToPixmap is not available in the Windows port of
	   libxpm.  But XpmReadFileToImage almost does what we want.  */
	rc = fn_XpmReadFileToImage (&raw mut hdc as *mut Display, SSDATA (file) as *mut c_char,
				    &raw mut xpm_image, &raw mut xpm_mask, &raw mut attrs);
      }
      #[cfg (all (feature = "have_window_system", not (feature = "have_ntgui")))]
      {
	rc = XpmReadFileToImage (FRAME_X_DISPLAY! (f), SSDATA (file) as *mut c_char,
				 &raw mut (*img).ximg as *mut *mut XImage,
				 &raw mut (*img).mask_img as *mut *mut XImage,
				 &raw mut attrs);
      }
    }
  else
    {
      let buffer: Lisp_Object = image_spec_value ((*img).spec, QCdata (),
						  core::ptr::null_mut ());
      if !STRINGP (buffer)
	{
	  image_invalid_data_error (buffer);
	  #[cfg (all (feature = "have_window_system", feature = "have_x_windows",
		      not (feature = "use_cairo"), feature = "alloc_xpm_colors"))]
	  { xpm_free_color_cache (); }
	  SAFE_FREE! (sa_count, sa_mark);
	  return false;
	}
      #[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
      {
	/* XpmCreatePixmapFromBuffer is not available in the Windows port
	   of libxpm.  But XpmCreateImageFromBuffer almost does what we want.  */
	rc = fn_XpmCreateImageFromBuffer (&raw mut hdc as *mut Display,
					  SSDATA (buffer) as *mut c_char,
					  &raw mut xpm_image, &raw mut xpm_mask, &raw mut attrs);
      }
      #[cfg (all (feature = "have_window_system", not (feature = "have_ntgui")))]
      {
	rc = XpmCreateImageFromBuffer (FRAME_X_DISPLAY! (f), SSDATA (buffer) as *mut c_char,
				       &raw mut (*img).ximg as *mut *mut XImage,
				       &raw mut (*img).mask_img as *mut *mut XImage,
				       &raw mut attrs);
      }
    }

  #[cfg (all (feature = "have_window_system", feature = "have_x_windows"))]
  if rc == XpmSuccess
    {
      (*img).pixmap = XCreatePixmap (FRAME_X_DISPLAY! (f), FRAME_X_DRAWABLE! (f),
				     (*((*img).ximg as *mut XImage)).width,
				     (*((*img).ximg as *mut XImage)).height,
				     (*((*img).ximg as *mut XImage)).depth) as *mut c_void;
      if (*img).pixmap.is_null ()
	{
	  image_clear_image (f, img);
	  rc = XpmNoMemory;
	}
      else
	{
	  #[cfg (all (feature = "have_window_system", not (feature = "use_cairo"),
		      feature = "have_xrender"))]
	  { (*img).picture = x_create_xrender_picture
	      (f, (*img).pixmap as Emacs_Pixmap,
	       (*((*img).ximg as *mut XImage)).depth) as *mut c_void; }
	  if !(*img).mask_img.is_null ()
	    {
	      (*img).mask = XCreatePixmap (FRAME_X_DISPLAY! (f), FRAME_X_DRAWABLE! (f),
					   (*((*img).mask_img as *mut XImage)).width,
					   (*((*img).mask_img as *mut XImage)).height,
					   (*((*img).mask_img as *mut XImage)).depth) as *mut c_void;
	      if (*img).mask.is_null ()
		{
		  image_clear_image (f, img);
		  rc = XpmNoMemory;
		}
	      else
		{
		  /* C 6502-6506's `# if !defined USE_CAIRO && defined HAVE_XRENDER` gates the
		     whole `else` body (a single statement), which is the block below.  */
		  #[cfg (all (feature = "have_window_system", not (feature = "use_cairo"),
			      feature = "have_xrender"))]
		  { (*img).mask_picture = x_create_xrender_picture
		      (f, (*img).mask as Emacs_Pixmap,
		       (*((*img).mask_img as *mut XImage)).depth) as *mut c_void; }
		}
	    }
	}
    }

  if rc == XpmSuccess
    {
      #[cfg (all (feature = "have_window_system", feature = "have_x_windows",
		  not (any (feature = "have_ntgui", feature = "use_cairo", feature = "have_ns")),
		  feature = "alloc_xpm_colors"))]
      { (*img).colors = colors_in_color_table (&raw mut (*img).ncolors); }
      #[cfg (not (all (feature = "have_window_system", feature = "have_x_windows",
		       not (any (feature = "have_ntgui", feature = "use_cairo", feature = "have_ns")),
		       feature = "alloc_xpm_colors")))]
      {
	let mut i: c_int;

	#[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
	{
	  /* W32 XPM uses XImage to wrap what W32 Emacs calls a Pixmap,
	     plus some duplicate attributes.  */
	  if !xpm_image.is_null () && !(*xpm_image).bitmap.is_null ()
	    {
	      (*img).pixmap = (*xpm_image).bitmap as *mut c_void;
	      /* XImageFree in libXpm frees XImage struct without destroying
		 the bitmap, which is what we want.  */
	      fn_XImageFree (xpm_image);
	    }
	  if !xpm_mask.is_null () && !(*xpm_mask).bitmap.is_null ()
	    {
	      /* The mask appears to be inverted compared with what we expect.
		 TODO: invert our expectations.  See other places where we
		 have to invert bits because our idea of masks is backwards.  */
	      let old_obj: HGDIOBJ;
	      old_obj = SelectObject (hdc, (*xpm_mask).bitmap);

	      PatBlt (hdc, 0, 0, (*xpm_mask).width, (*xpm_mask).height, DSTINVERT);
	      SelectObject (hdc, old_obj);

	      (*img).mask = (*xpm_mask).bitmap as *mut c_void;
	      fn_XImageFree (xpm_mask);
	      DeleteDC (hdc);
	    }

	  DeleteDC (hdc);
	}

	/* Remember allocated colors.  */
	(*img).colors = xnmalloc (attrs.nalloc_pixels as isize,
				  core::mem::size_of::<c_ulong> () as isize) as *mut c_ulong;
	(*img).ncolors = attrs.nalloc_pixels;
	i = 0;
	while i < attrs.nalloc_pixels
	  {
	    *(*img).colors.offset (i as isize) = *attrs.alloc_pixels.offset (i as isize);
	    i += 1;
	  }
      }

      (*img).width = attrs.width as c_int;
      (*img).height = attrs.height as c_int;
      eassert! ((*img).width > 0 && (*img).height > 0);

      /* The call to XpmFreeAttributes below frees attrs.alloc_pixels.  C 6199-6202's
	 `# define XpmFreeAttributes fn_XpmFreeAttributes` renames it on the Windows side, where
	 this same line is compiled for the other arm.  */
      #[cfg (feature = "have_ntgui")]
      { fn_XpmFreeAttributes (&raw mut attrs); }
      #[cfg (not (feature = "have_ntgui"))]
      { XpmFreeAttributes (&raw mut attrs); }

      #[cfg (all (feature = "have_window_system", feature = "have_x_windows"))]
      {
	/* Maybe fill in the background field while we have ximg handy.  */
	IMAGE_BACKGROUND! (img, f, (*img).ximg as *mut XImage);
	/* Fill in the background_transparent field while we have the
	   mask handy.  */
	if !(*img).mask_img.is_null ()
	  { image_background_transparent (img, f, (*img).mask_img as *mut XImage); }
      }
    }
  else
    {
      #[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
      { DeleteDC (hdc); }

      match rc
	{
	  XpmOpenFailed
	    => { image_error (c"Error opening XPM file (%s)".as_ptr (), (*img).spec); }
	  XpmFileInvalid
	    => { image_error (c"Invalid XPM file (%s)".as_ptr (), (*img).spec); }
	  XpmNoMemory
	    => { image_error (c"Out of memory (%s)".as_ptr (), (*img).spec); }
	  XpmColorFailed
	    => { image_error (c"Color allocation error (%s)".as_ptr (), (*img).spec); }
	  _ => { image_error (c"Unknown error (%s)".as_ptr (), (*img).spec); }
	}
    }

  #[cfg (all (feature = "have_window_system", feature = "have_x_windows",
	      not (feature = "use_cairo"), feature = "alloc_xpm_colors"))]
  { xpm_free_color_cache (); }
  SAFE_FREE! (sa_count, sa_mark);
  return rc == XpmSuccess;
}

/* ==============================================================================================
   C 6615-7122: the XPM support that does not use libxpm - "XPM support functions for NS, Haiku
   and Android where libxpm is not available, and for Cairo.  Only XPM version 3 (without any
   extensions) is supported."  The gate is C 6615-6619's five-way condition, carried arm for arm
   (C's own `#endif` comment at C 7122 reads `HAVE_NS && !HAVE_XPM`, which its condition does not
   say; the text is C's).  The four `static` prototypes at C 6625-6630 are dropped (PORTING.md
   0.1-12: their definitions are C 6725-6779, in this slice).  */
/* Tokens returned from xpm_scan.  */

/// C: C 6634-6639: `enum xpm_token { XPM_TK_IDENT = 256, XPM_TK_STRING, XPM_TK_EOF }`
#[cfg (all (feature = "have_window_system",
	    any (all (feature = "use_cairo", feature = "have_xpm"),
		 all (feature = "have_ns", not (feature = "have_xpm")),
		 all (feature = "have_haiku", not (feature = "have_xpm")),
		 all (feature = "have_pgtk", not (feature = "have_xpm")),
		 all (feature = "have_android", not (feature = "have_xpm")))))]
pub const XPM_TK_IDENT: c_int = 256;
#[cfg (all (feature = "have_window_system",
	    any (all (feature = "use_cairo", feature = "have_xpm"),
		 all (feature = "have_ns", not (feature = "have_xpm")),
		 all (feature = "have_haiku", not (feature = "have_xpm")),
		 all (feature = "have_pgtk", not (feature = "have_xpm")),
		 all (feature = "have_android", not (feature = "have_xpm")))))]
pub const XPM_TK_STRING: c_int = 257;
#[cfg (all (feature = "have_window_system",
	    any (all (feature = "use_cairo", feature = "have_xpm"),
		 all (feature = "have_ns", not (feature = "have_xpm")),
		 all (feature = "have_haiku", not (feature = "have_xpm")),
		 all (feature = "have_pgtk", not (feature = "have_xpm")),
		 all (feature = "have_android", not (feature = "have_xpm")))))]
pub const XPM_TK_EOF: c_int = 258;

/* Scan an XPM data and return a character (< 256) or a token defined
   by enum xpm_token above.  *S and END are the start (inclusive) and
   the end (exclusive) addresses of the data, respectively.  Advance
   *S while scanning.  If token is either XPM_TK_IDENT or
   XPM_TK_STRING, *BEG and *LEN are set to the start address and the
   length of the corresponding token, respectively.  */

/// C: C 6648-6649: `static int xpm_scan (const char **s, const char *end, const char **beg, ptrdiff_t *len)`
#[cfg (all (feature = "have_window_system",
	    any (all (feature = "use_cairo", feature = "have_xpm"),
		 all (feature = "have_ns", not (feature = "have_xpm")),
		 all (feature = "have_haiku", not (feature = "have_xpm")),
		 all (feature = "have_pgtk", not (feature = "have_xpm")),
		 all (feature = "have_android", not (feature = "have_xpm")))))]
unsafe fn xpm_scan (s: *mut *const c_char, end: *const c_char, beg: *mut *const c_char,
		    len: *mut ptrdiff_t) -> c_int
{
  /* C's `unsigned char c;` - uninitialised, zeroed (PORTING.md 1.5).  */
  let mut c: c_uchar = 0;

  while *s < end
    {
      /* Skip white-space.  */
      loop
	{
	  c = **s as c_uchar;
	  *s = (*s).offset (1);
	  if !(c_isspace (c as c_int) && *s < end)
	    { break; }
	}

      /* gnus-pointer.xpm uses '-' in its identifier.
	 sb-dir-plus.xpm uses '+' in its identifier.  */
      if c_isalpha (c as c_int) || c == b'_' || c == b'-' || c == b'+'
	{
	  *beg = (*s).offset (-1);
	  loop
	    {
	      if !(*s < end)
		{ break; }
	      c = **s as c_uchar;
	      if !(c_isalnum (c as c_int) || c == b'_' || c == b'-' || c == b'+')
		{ break; }
	      *s = (*s).offset (1);
	    }
	  *len = (*s).offset_from (*beg);
	  return XPM_TK_IDENT;
	}
      else if c == b'"'
	{
	  *beg = *s;
	  while *s < end && **s != b'"' as c_char
	    { *s = (*s).offset (1); }
	  *len = (*s).offset_from (*beg);
	  if *s < end
	    { *s = (*s).offset (1); }
	  return XPM_TK_STRING;
	}
      else if c == b'/'
	{
	  if *s < end && **s == b'*' as c_char
	    {
	      /* C-style comment.  */
	      *s = (*s).offset (1);
	      loop
		{
		  loop
		    {
		      if !(*s < end)
			{ break; }
		      let ch: c_char = **s;
		      *s = (*s).offset (1);
		      if ch != b'*' as c_char
			{ continue; }
		      break;
		    }
		  if !(*s < end && **s != b'/' as c_char)
		    { break; }
		}
	      if *s < end
		{ *s = (*s).offset (1); }
	    }
	  else
	    { return c as c_int; }
	}
      else
	{ return c as c_int; }
    }

  return XPM_TK_EOF;
}

/* Functions for color table lookup in XPM data.  A key is a string
   specifying the color of each pixel in XPM data.  A value is either
   an integer that specifies a pixel color, Qt that specifies
   transparency, or Qnil for the unspecified color.  If the length of
   the key string is one, a vector is used as a table.  Otherwise, a
   hash table is used.  */

/// C: C 6714-6718: `static Lisp_Object xpm_make_color_table_v (void (**put_func) (Lisp_Object, const char *, int, Lisp_Object), Lisp_Object (**get_func) (Lisp_Object, const char *, int))`
#[cfg (all (feature = "have_window_system",
	    any (all (feature = "use_cairo", feature = "have_xpm"),
		 all (feature = "have_ns", not (feature = "have_xpm")),
		 all (feature = "have_haiku", not (feature = "have_xpm")),
		 all (feature = "have_pgtk", not (feature = "have_xpm")),
		 all (feature = "have_android", not (feature = "have_xpm")))))]
unsafe fn xpm_make_color_table_v (put_func: *mut Option<unsafe extern "C" fn (Lisp_Object,
									     *const c_char, c_int,
									     Lisp_Object)>,
				  get_func: *mut Option<unsafe extern "C" fn (Lisp_Object,
									      *const c_char,
									      c_int) -> Lisp_Object>)
  -> Lisp_Object
{
  *put_func = Some (xpm_put_color_table_v);
  *get_func = Some (xpm_get_color_table_v);
  return make_nil_vector (256);
}

/// C: C 6725-6729: `static void xpm_put_color_table_v (Lisp_Object color_table, const char *chars_start, int chars_len, Lisp_Object color)`
#[cfg (all (feature = "have_window_system",
	    any (all (feature = "use_cairo", feature = "have_xpm"),
		 all (feature = "have_ns", not (feature = "have_xpm")),
		 all (feature = "have_haiku", not (feature = "have_xpm")),
		 all (feature = "have_pgtk", not (feature = "have_xpm")),
		 all (feature = "have_android", not (feature = "have_xpm")))))]
unsafe extern "C" fn xpm_put_color_table_v (color_table: Lisp_Object,
					   chars_start: *const c_char, chars_len: c_int,
					   color: Lisp_Object)
{
  let uc: c_uchar = *chars_start as c_uchar;
  ASET (color_table, uc as isize, color);
}

/// C: C 6735-6738: `static Lisp_Object xpm_get_color_table_v (Lisp_Object color_table, const char *chars_start, int chars_len)`
#[cfg (all (feature = "have_window_system",
	    any (all (feature = "use_cairo", feature = "have_xpm"),
		 all (feature = "have_ns", not (feature = "have_xpm")),
		 all (feature = "have_haiku", not (feature = "have_xpm")),
		 all (feature = "have_pgtk", not (feature = "have_xpm")),
		 all (feature = "have_android", not (feature = "have_xpm")))))]
unsafe extern "C" fn xpm_get_color_table_v (color_table: Lisp_Object,
					   chars_start: *const c_char, chars_len: c_int)
  -> Lisp_Object
{
  let uc: c_uchar = *chars_start as c_uchar;
  return AREF (color_table, uc as isize);
}

/// C: C 6744-6748: `static Lisp_Object xpm_make_color_table_h (void (**put_func) (Lisp_Object, const char *, int, Lisp_Object), Lisp_Object (**get_func) (Lisp_Object, const char *, int))`
#[cfg (all (feature = "have_window_system",
	    any (all (feature = "use_cairo", feature = "have_xpm"),
		 all (feature = "have_ns", not (feature = "have_xpm")),
		 all (feature = "have_haiku", not (feature = "have_xpm")),
		 all (feature = "have_pgtk", not (feature = "have_xpm")),
		 all (feature = "have_android", not (feature = "have_xpm")))))]
unsafe fn xpm_make_color_table_h (put_func: *mut Option<unsafe extern "C" fn (Lisp_Object,
									     *const c_char, c_int,
									     Lisp_Object)>,
				  get_func: *mut Option<unsafe extern "C" fn (Lisp_Object,
									      *const c_char,
									      c_int) -> Lisp_Object>)
  -> Lisp_Object
{
  *put_func = Some (xpm_put_color_table_h);
  *get_func = Some (xpm_get_color_table_h);
  return make_hash_table (&raw const hashtest_equal, DEFAULT_HASH_SIZE as EMACS_INT, Weak_None);
}

/// C: C 6755-6759: `static void xpm_put_color_table_h (Lisp_Object color_table, const char *chars_start, int chars_len, Lisp_Object color)`
#[cfg (all (feature = "have_window_system",
	    any (all (feature = "use_cairo", feature = "have_xpm"),
		 all (feature = "have_ns", not (feature = "have_xpm")),
		 all (feature = "have_haiku", not (feature = "have_xpm")),
		 all (feature = "have_pgtk", not (feature = "have_xpm")),
		 all (feature = "have_android", not (feature = "have_xpm")))))]
unsafe extern "C" fn xpm_put_color_table_h (color_table: Lisp_Object,
					   chars_start: *const c_char, chars_len: c_int,
					   color: Lisp_Object)
{
  let table: *mut Lisp_Hash_Table = XHASH_TABLE (color_table);
  let chars: Lisp_Object = make_unibyte_string (chars_start, chars_len as ptrdiff_t);

  /* C's `hash_hash_t hash_code;` - zeroed (PORTING.md 1.5).  */
  let mut hash_code: hash_hash_t = 0;
  hash_find_get_hash (table, chars, &raw mut hash_code);
  hash_put (table, chars, color, hash_code);
}

/// C: C 6769-6772: `static Lisp_Object xpm_get_color_table_h (Lisp_Object color_table, const char *chars_start, int chars_len)`
#[cfg (all (feature = "have_window_system",
	    any (all (feature = "use_cairo", feature = "have_xpm"),
		 all (feature = "have_ns", not (feature = "have_xpm")),
		 all (feature = "have_haiku", not (feature = "have_xpm")),
		 all (feature = "have_pgtk", not (feature = "have_xpm")),
		 all (feature = "have_android", not (feature = "have_xpm")))))]
unsafe extern "C" fn xpm_get_color_table_h (color_table: Lisp_Object,
					   chars_start: *const c_char, chars_len: c_int)
  -> Lisp_Object
{
  let table: *mut Lisp_Hash_Table = XHASH_TABLE (color_table);
  let i: ptrdiff_t
    = hash_find (table, make_unibyte_string (chars_start, chars_len as ptrdiff_t));

  return if i >= 0 { HASH_VALUE (table, i as hash_idx_t) } else { Qnil () };
}

/// C: C 6781-6787: `enum xpm_color_key { XPM_COLOR_KEY_S, XPM_COLOR_KEY_M, XPM_COLOR_KEY_G4, XPM_COLOR_KEY_G, XPM_COLOR_KEY_C }`
#[cfg (all (feature = "have_window_system",
	    any (all (feature = "use_cairo", feature = "have_xpm"),
		 all (feature = "have_ns", not (feature = "have_xpm")),
		 all (feature = "have_haiku", not (feature = "have_xpm")),
		 all (feature = "have_pgtk", not (feature = "have_xpm")),
		 all (feature = "have_android", not (feature = "have_xpm")))))]
pub const XPM_COLOR_KEY_S: c_int = 0;
/// C: C 6781-6787
#[cfg (all (feature = "have_window_system",
	    any (all (feature = "use_cairo", feature = "have_xpm"),
		 all (feature = "have_ns", not (feature = "have_xpm")),
		 all (feature = "have_haiku", not (feature = "have_xpm")),
		 all (feature = "have_pgtk", not (feature = "have_xpm")),
		 all (feature = "have_android", not (feature = "have_xpm")))))]
pub const XPM_COLOR_KEY_M: c_int = 1;
/// C: C 6781-6787
#[cfg (all (feature = "have_window_system",
	    any (all (feature = "use_cairo", feature = "have_xpm"),
		 all (feature = "have_ns", not (feature = "have_xpm")),
		 all (feature = "have_haiku", not (feature = "have_xpm")),
		 all (feature = "have_pgtk", not (feature = "have_xpm")),
		 all (feature = "have_android", not (feature = "have_xpm")))))]
pub const XPM_COLOR_KEY_G4: c_int = 2;
/// C: C 6781-6787
#[cfg (all (feature = "have_window_system",
	    any (all (feature = "use_cairo", feature = "have_xpm"),
		 all (feature = "have_ns", not (feature = "have_xpm")),
		 all (feature = "have_haiku", not (feature = "have_xpm")),
		 all (feature = "have_pgtk", not (feature = "have_xpm")),
		 all (feature = "have_android", not (feature = "have_xpm")))))]
pub const XPM_COLOR_KEY_G: c_int = 3;
/// C: C 6781-6787
#[cfg (all (feature = "have_window_system",
	    any (all (feature = "use_cairo", feature = "have_xpm"),
		 all (feature = "have_ns", not (feature = "have_xpm")),
		 all (feature = "have_haiku", not (feature = "have_xpm")),
		 all (feature = "have_pgtk", not (feature = "have_xpm")),
		 all (feature = "have_android", not (feature = "have_xpm")))))]
pub const XPM_COLOR_KEY_C: c_int = 4;

/// C: C 6789: `static const char xpm_color_key_strings[][4] = {"s", "m", "g4", "g", "c"};`
#[cfg (all (feature = "have_window_system",
	    any (all (feature = "use_cairo", feature = "have_xpm"),
		 all (feature = "have_ns", not (feature = "have_xpm")),
		 all (feature = "have_haiku", not (feature = "have_xpm")),
		 all (feature = "have_pgtk", not (feature = "have_xpm")),
		 all (feature = "have_android", not (feature = "have_xpm")))))]
static xpm_color_key_strings: [[c_char; 4]; 5] =
  [
    [b's' as c_char, 0, 0, 0],
    [b'm' as c_char, 0, 0, 0],
    [b'g' as c_char, b'4' as c_char, 0, 0],
    [b'g' as c_char, 0, 0, 0],
    [b'c' as c_char, 0, 0, 0],
  ];

/// C: C 6791-6792: `static int xpm_str_to_color_key (const char *s)`
#[cfg (all (feature = "have_window_system",
	    any (all (feature = "use_cairo", feature = "have_xpm"),
		 all (feature = "have_ns", not (feature = "have_xpm")),
		 all (feature = "have_haiku", not (feature = "have_xpm")),
		 all (feature = "have_pgtk", not (feature = "have_xpm")),
		 all (feature = "have_android", not (feature = "have_xpm")))))]
unsafe fn xpm_str_to_color_key (s: *const c_char) -> c_int
{
  let mut i: c_int = 0;
  while i < countof! (xpm_color_key_strings) as c_int
    {
      if strcmp (xpm_color_key_strings [i as usize].as_ptr (), s) == 0
	{ return i; }
      i += 1;
    }
  return -1;
}

/// C: C 6800-6801: `static int xpm_str_to_int (char **buf)`
#[cfg (all (feature = "have_window_system",
	    any (all (feature = "use_cairo", feature = "have_xpm"),
		 all (feature = "have_ns", not (feature = "have_xpm")),
		 all (feature = "have_haiku", not (feature = "have_xpm")),
		 all (feature = "have_pgtk", not (feature = "have_xpm")),
		 all (feature = "have_android", not (feature = "have_xpm")))))]
unsafe fn xpm_str_to_int (buf: *mut *mut c_char) -> c_int
{
  /* C's `char *p;` is written by `strtol`'s out parameter, so it takes C's zero
     (PORTING.md 1.5/0.1-13).  */
  let mut p: *mut c_char = core::ptr::null_mut ();

  *errno () = 0;
  let result: c_long = strtol (*buf, &raw mut p as *mut *mut c_char, 10);
  if *errno () != 0 || p == *buf || result < INT_MIN as c_long || result > INT_MAX as c_long
    { return -1; }

  /* Error out if we see something like "12x3xyz".  */
  if !c_isspace (*p as c_int) && *p != b'\0' as c_char
    { return -1; }

  /* Update position to read next integer.  */
  *buf = p;

  return result as c_int;
}

/// C: C 6820-6824: `static bool xpm_load_image (struct frame *f, struct image *img, const char *contents, const char *end)`
#[cfg (all (feature = "have_window_system",
	    any (all (feature = "use_cairo", feature = "have_xpm"),
		 all (feature = "have_ns", not (feature = "have_xpm")),
		 all (feature = "have_haiku", not (feature = "have_xpm")),
		 all (feature = "have_pgtk", not (feature = "have_xpm")),
		 all (feature = "have_android", not (feature = "have_xpm")))))]
unsafe fn xpm_load_image (f: *mut frame, img: *mut image, contents: *const c_char,
			  end: *const c_char) -> bool
{
  let mut s: *const c_char = contents;
  /* C's `const char *s = contents, *beg, *str;` - `beg` is written by every `xpm_scan` call
     before it is read, `str` by the loops below; both are zeroed (PORTING.md 1.5).  */
  let mut beg: *const c_char = core::ptr::null ();
  let mut str: *const c_char = core::ptr::null ();
  /* C's `char buffer[BUFSIZ];` - a stack array, not zeroed by C, zeroed here (1.5).  */
  let mut buffer: [c_char; BUFSIZ as usize] = [0; BUFSIZ as usize];
  let mut width: c_int;
  let mut height: c_int;
  let mut x: c_int;
  let mut y: c_int;
  let mut num_colors: c_int;
  let mut chars_per_pixel: c_int;
  let mut len: ptrdiff_t = 0;
  /* C's `int LA1;` - written by every `match` expansion before it is read (1.5).  */
  let mut LA1: c_int = 0;
  let mut put_color_table: Option<unsafe extern "C" fn (Lisp_Object, *const c_char, c_int,
							Lisp_Object)> = None;
  let mut get_color_table: Option<unsafe extern "C" fn (Lisp_Object, *const c_char, c_int)
						   -> Lisp_Object> = None;
  let mut frame: Lisp_Object;
  let mut color_symbols: Lisp_Object;
  let mut color_table: Lisp_Object;
  let mut best_key: c_int;
  #[cfg (not (feature = "have_ns"))]
  let mut have_mask: bool = false;
  let mut ximg: Emacs_Pix_Container = core::ptr::null_mut ();
  let mut mask_img: Emacs_Pix_Container = core::ptr::null_mut ();

  /* C 6841-6858's three function-local macros close over this function's own locals (`s`, `end`,
     `beg`, `len`, `LA1`) and its `failure:` label, which `macro_rules!` hygiene forbids (measured
     for `xbm_read_bitmap_data`'s pair, `docs/ports/image.md`; `match` is a Rust keyword besides),
     so each use site carries the expansion the preprocessor produces:

       match ()                 -> `LA1 = xpm_scan (&raw mut s, end, &raw mut beg, &raw mut len);`
       expect (TOKEN)           -> `if LA1 != (TOKEN) { break 'failure; }` then the `match` line
       expect_ident (IDENT)     -> the `LA1 == XPM_TK_IDENT && strlen (IDENT) == len &&
				    memcmp (IDENT, beg, len) == 0` test, then `match ()` or
				    `break 'failure`

     The macro-body-notes rows record the four tokens the expansions cannot spell.  */
  'failure: {
    if !(end.offset_from (s) >= 9
	 && memcmp (s as *const c_void, c"/* XPM */".as_ptr () as *const c_void, 9) == 0)
      { break 'failure; }
    s = s.offset (9);
    /* match (); */
    LA1 = xpm_scan (&raw mut s, end, &raw mut beg, &raw mut len);
    /* expect_ident ("static"); */
    if LA1 == XPM_TK_IDENT
      && strlen (c"static".as_ptr ()) as ptrdiff_t == len
      && memcmp (c"static".as_ptr () as *const c_void, beg as *const c_void, len as usize) == 0
      { LA1 = xpm_scan (&raw mut s, end, &raw mut beg, &raw mut len); }
    else
      { break 'failure; }
    /* expect_ident ("char"); */
    if LA1 == XPM_TK_IDENT
      && strlen (c"char".as_ptr ()) as ptrdiff_t == len
      && memcmp (c"char".as_ptr () as *const c_void, beg as *const c_void, len as usize) == 0
      { LA1 = xpm_scan (&raw mut s, end, &raw mut beg, &raw mut len); }
    else
      { break 'failure; }
    /* expect ('*'); */
    if LA1 != '*' as c_int
      { break 'failure; }
    LA1 = xpm_scan (&raw mut s, end, &raw mut beg, &raw mut len);
    /* expect (XPM_TK_IDENT); */
    if LA1 != XPM_TK_IDENT
      { break 'failure; }
    LA1 = xpm_scan (&raw mut s, end, &raw mut beg, &raw mut len);
    /* expect ('['); */
    if LA1 != '[' as c_int
      { break 'failure; }
    LA1 = xpm_scan (&raw mut s, end, &raw mut beg, &raw mut len);
    /* expect (']'); */
    if LA1 != ']' as c_int
      { break 'failure; }
    LA1 = xpm_scan (&raw mut s, end, &raw mut beg, &raw mut len);
    /* expect ('='); */
    if LA1 != '=' as c_int
      { break 'failure; }
    LA1 = xpm_scan (&raw mut s, end, &raw mut beg, &raw mut len);
    /* expect ('{'); */
    if LA1 != '{' as c_int
      { break 'failure; }
    LA1 = xpm_scan (&raw mut s, end, &raw mut beg, &raw mut len);
    /* expect (XPM_TK_STRING); */
    if LA1 != XPM_TK_STRING
      { break 'failure; }
    LA1 = xpm_scan (&raw mut s, end, &raw mut beg, &raw mut len);
    if len >= BUFSIZ as ptrdiff_t
      { break 'failure; }
    memcpy (buffer.as_mut_ptr () as *mut c_void, beg as *const c_void, len as usize);
    buffer [len as usize] = 0;
    /* C's `char *next_int = buffer;` - it is advanced by each `xpm_str_to_int` call.  */
    let mut next_int: *mut c_char = buffer.as_mut_ptr ();
    width = xpm_str_to_int (&raw mut next_int);
    if width <= 0
      { break 'failure; }
    height = xpm_str_to_int (&raw mut next_int);
    if height <= 0
      { break 'failure; }
    num_colors = xpm_str_to_int (&raw mut next_int);
    if num_colors <= 0
      { break 'failure; }
    chars_per_pixel = xpm_str_to_int (&raw mut next_int);
    if chars_per_pixel <= 0
      { break 'failure; }

    if !check_image_size (f, width, height)
      {
	image_size_error ();
	break 'failure;
      }

    let mut created: bool
      = image_create_x_image_and_pixmap (f, img, width, height, 0, &raw mut ximg, false);
    /* C 6893-6898: `!image_create_x_image_and_pixmap (…, 0, &ximg, 0)
	 #ifndef HAVE_NS || !image_create_x_image_and_pixmap (…, 1, &mask_img, 1) #endif` - the
       `#ifndef` operand is written as C's short-circuit leaves it (the second call runs only
       when the first succeeded).  */
    #[cfg (not (feature = "have_ns"))]
    { created = created && image_create_x_image_and_pixmap (f, img, width, height, 1,
							    &raw mut mask_img, true); }
    if !created
      {
	image_error (c"Image too large".as_ptr ());
	break 'failure;
      }

    /* expect (','); */
    if LA1 != ',' as c_int
      { break 'failure; }
    LA1 = xpm_scan (&raw mut s, end, &raw mut beg, &raw mut len);

    XSETFRAME! (frame, f);

    /* C 6908-6923: `#ifndef HAVE_ANDROID` (the display-capability chain) / `#else` (the
       comment plus `best_key = XPM_COLOR_KEY_C`).  */
    #[cfg (not (feature = "have_android"))]
    {
      if !NILP (Fxw_display_color_p (frame))
	{ best_key = XPM_COLOR_KEY_C; }
      else if !NILP (Fx_display_grayscale_p (frame))
	{
	  best_key = if XFIXNAT (Fx_display_planes (frame)) > 2 { XPM_COLOR_KEY_G }
		     else { XPM_COLOR_KEY_G4 };
	}
      else
	{ best_key = XPM_COLOR_KEY_M; }
    }
    #[cfg (feature = "have_android")]
    {
      /* The color-loading loop has not been taught to progressively settle
	 for less optimal color keys if no colors are defined for best_key,
	 and since libXpm is not available on Android, there is no better
	 option than delegating the task of mapping whatever color values
	 are provided to B/W or grayscale to the display driver.  */
      best_key = XPM_COLOR_KEY_C;
    }

    color_symbols = image_spec_value ((*img).spec, QCcolor_symbols (), core::ptr::null_mut ());
    if chars_per_pixel == 1
      { color_table = xpm_make_color_table_v (&raw mut put_color_table, &raw mut get_color_table); }
    else
      { color_table = xpm_make_color_table_h (&raw mut put_color_table, &raw mut get_color_table); }

    while num_colors > 0
      {
	num_colors -= 1;
	let mut color: *mut c_char;
	let mut max_color: *mut c_char = core::ptr::null_mut ();
	let mut key: c_int;
	/* C's `int key, next_key, max_key = 0;` - `next_key` is written by the loop below before
	   it is read (1.5).  */
	let mut next_key: c_int = 0;
	let mut max_key: c_int = 0;
	let mut symbol_color: Lisp_Object = Qnil ();
	let mut color_val: Lisp_Object;
	/* C's `Emacs_Color cdef;` - zeroed (1.5).  */
	let mut cdef: Emacs_Color = core::mem::zeroed ();

	/* expect (XPM_TK_STRING); */
	if LA1 != XPM_TK_STRING
	  { break 'failure; }
	LA1 = xpm_scan (&raw mut s, end, &raw mut beg, &raw mut len);
	if len <= chars_per_pixel as ptrdiff_t || len >= (BUFSIZ + chars_per_pixel) as ptrdiff_t
	  { break 'failure; }
	memcpy (buffer.as_mut_ptr () as *mut c_void,
		beg.offset (chars_per_pixel as isize) as *const c_void,
		(len - chars_per_pixel as ptrdiff_t) as usize);
	buffer [(len - chars_per_pixel as ptrdiff_t) as usize] = 0;

	str = strtok (buffer.as_mut_ptr (), c" \t".as_ptr ());
	if str.is_null ()
	  { break 'failure; }
	key = xpm_str_to_color_key (str);
	if key < 0
	  { break 'failure; }
	loop
	  {
	    color = strtok (core::ptr::null_mut (), c" \t".as_ptr ());
	    if color.is_null ()
	      { break 'failure; }

	    loop
	      {
		str = strtok (core::ptr::null_mut (), c" \t".as_ptr ());
		if str.is_null ()
		  { break; }
		next_key = xpm_str_to_color_key (str);
		if next_key >= 0
		  { break; }
		*color.offset (strlen (color) as isize) = ' ' as c_char;
	      }

	    if key == XPM_COLOR_KEY_S
	      {
		if NILP (symbol_color)
		  { symbol_color = build_string (color); }
	      }
	    else if max_key < key && key <= best_key
	      {
		max_key = key;
		max_color = color;
	      }
	    key = next_key;
	    /* C's `while (str);` - the do-while exits when `strtok` returned NULL.  */
	    if str.is_null ()
	      { break; }
	  }

	color_val = Qnil ();
	if !NILP (color_symbols) && !NILP (symbol_color)
	  {
	    let specified_color: Lisp_Object = Fassoc (symbol_color, color_symbols, Qnil ());

	    if CONSP (specified_color) && STRINGP (XCDR (specified_color))
	      {
		if xstrcasecmp (SSDATA (XCDR (specified_color)), c"None".as_ptr ()) == 0
		  { color_val = Qt (); }
		else if ((*FRAME_TERMINAL! (f)).defined_color_hook.unwrap ())
		    (f as *mut c_void, SSDATA (XCDR (specified_color)), &raw mut cdef, false, false)
		  {
		    color_val = make_fixnum (lookup_rgb_color (f, cdef.red as c_int,
							      cdef.green as c_int,
							      cdef.blue as c_int) as EMACS_INT);
		  }
	      }
	  }
	if NILP (color_val) && !max_color.is_null ()
	  {
	    if xstrcasecmp (max_color, c"None".as_ptr ()) == 0
	      { color_val = Qt (); }
	    else if ((*FRAME_TERMINAL! (f)).defined_color_hook.unwrap ())
		(f as *mut c_void, max_color, &raw mut cdef, false, false)
	      {
		color_val = make_fixnum (lookup_rgb_color (f, cdef.red as c_int,
							  cdef.green as c_int,
							  cdef.blue as c_int) as EMACS_INT);
	      }
	  }
	if !NILP (color_val)
	  { (put_color_table.unwrap ()) (color_table, beg, chars_per_pixel, color_val); }

	/* expect (','); */
	if LA1 != ',' as c_int
	  { break 'failure; }
	LA1 = xpm_scan (&raw mut s, end, &raw mut beg, &raw mut len);
      }

    let mut frame_fg: c_ulong = FRAME_FOREGROUND_PIXEL (f);
    #[cfg (feature = "use_cairo")]
    {
      let mut color: Emacs_Color = Emacs_Color { pixel: frame_fg, red: 0, green: 0, blue: 0 };
      ((*FRAME_TERMINAL! (f)).query_colors.unwrap ()) (f as *mut c_void, &raw mut color, 1);
      frame_fg = lookup_rgb_color (f, color.red as c_int, color.green as c_int,
				  color.blue as c_int);
    }
    y = 0;
    while y < height
      {
	/* expect (XPM_TK_STRING); */
	if LA1 != XPM_TK_STRING
	  { break 'failure; }
	LA1 = xpm_scan (&raw mut s, end, &raw mut beg, &raw mut len);
	str = beg;
	if len < (width * chars_per_pixel) as ptrdiff_t
	  { break 'failure; }
	x = 0;
	while x < width
	  {
	    let color_val: Lisp_Object
	      = (get_color_table.unwrap ()) (color_table, str, chars_per_pixel);

	    PUT_PIXEL! (ximg, x, y, if FIXNUMP (color_val) { XFIXNUM (color_val) as c_ulong }
				     else { frame_fg });
	    #[cfg (not (feature = "have_ns"))]
	    {
	      /* C's `(!EQ (color_val, Qt) ? PIX_MASK_DRAW : (have_mask = true,
		 PIX_MASK_RETAIN))` - the comma expression is the block the `else` arm opens
		 with.  */
	      PUT_PIXEL! (mask_img, x, y,
			  if !EQ (color_val, Qt ()) { PIX_MASK_DRAW! () }
			  else { have_mask = true; PIX_MASK_RETAIN! () });
	    }
	    #[cfg (feature = "have_ns")]
	    {
	      if EQ (color_val, Qt ())
		{ ns_set_alpha (ximg, x, y, 0); }
	    }
	    str = str.offset (chars_per_pixel as isize);
	    x += 1;
	  }
	if y + 1 < height
	  {
	    /* expect (','); */
	    if LA1 != ',' as c_int
	      { break 'failure; }
	    LA1 = xpm_scan (&raw mut s, end, &raw mut beg, &raw mut len);
	  }
	y += 1;
      }

    (*img).width = width;
    (*img).height = height;

    /* Maybe fill in the background field while we have ximg handy. */
    if NILP (image_spec_value ((*img).spec, QCbackground (), core::ptr::null_mut ()))
      { IMAGE_BACKGROUND! (img, f, ximg); }

    image_put_x_image (f, img, ximg, false);
    #[cfg (not (feature = "have_ns"))]
    {
      if have_mask
	{
	  /* Fill in the background_transparent field while we have the
	     mask handy.  */
	  image_background_transparent (img, f, mask_img);

	  image_put_x_image (f, img, mask_img, true);
	}
      else
	{
	  image_destroy_x_image (mask_img);
	  image_clear_image_1 (f, img, CLEAR_IMAGE_MASK! ());
	}
    }
    return true;
  }

  /* failure: */
  image_error (c"Invalid XPM3 file (%s)".as_ptr (), (*img).spec);
  if !ximg.is_null ()
    {
      image_destroy_x_image (ximg);
      if !mask_img.is_null ()
	{ image_destroy_x_image (mask_img); }
      image_clear_image (f, img);
    }
  return false;
}

/// C: C 7086-7088: `static bool xpm_load (struct frame *f, struct image *img)` - the NS/cairo/cairo-with-XPM arm's second definition of the name (C 6307's is the plain-X11 one; C's `#if` chain keeps exactly one live, and the port's two Rust definitions carry natively disjoint gates).
#[cfg (all (feature = "have_window_system",
	    any (all (feature = "use_cairo", feature = "have_xpm"),
		 all (feature = "have_ns", not (feature = "have_xpm")),
		 all (feature = "have_haiku", not (feature = "have_xpm")),
		 all (feature = "have_pgtk", not (feature = "have_xpm")),
		 all (feature = "have_android", not (feature = "have_xpm")))))]
unsafe extern "C" fn xpm_load (f: *mut frame, img: *mut image) -> bool
{
  let mut success_p: bool = false;
  let mut file_name: Lisp_Object;

  /* If IMG->spec specifies a file name, create a non-file spec from it.  */
  file_name = image_spec_value ((*img).spec, QCfile (), core::ptr::null_mut ());
  if STRINGP (file_name)
    {
      /* C's `ptrdiff_t size;` - written by `slurp_image`'s out parameter (PORTING.md 1.5).  */
      let mut size: ptrdiff_t = 0;
      let contents: *mut c_char = slurp_image (file_name, &raw mut size, c"XPM".as_ptr ());
      if contents.is_null ()
	{ return false; }

      success_p = xpm_load_image (f, img, contents, contents.offset (size as isize));
      xfree (contents as *mut c_void);
    }
  else
    {
      let mut data: Lisp_Object;

      data = image_spec_value ((*img).spec, QCdata (), core::ptr::null_mut ());
      if !STRINGP (data)
	{
	  image_invalid_data_error (data);
	  return false;
	}
      success_p = xpm_load_image (f, img, SSDATA (data),
				  SSDATA (data).offset (SBYTES (data) as isize));
    }

  return success_p;
}

/* ==============================================================================================
   C 7126-7396: the "Color table" section.  Gate: C 7130's `#ifdef COLOR_TABLE_SUPPORT`, whose
   condition the first slice spells at its C 169-173 (the port repeats the condition - see
   `COLOR_TABLE_SUPPORT` above); the `#else` arm at C 7376-7396 is its negation and is where the
   frozen configuration's two definitions of the names come from.  */
/* An entry in the color table mapping an RGB color to a pixel color.  */

/// C: C 7134-7141: `struct ct_color { int r, g, b; unsigned long pixel; struct ct_color *next; };`
#[cfg (all (feature = "have_window_system", feature = "have_x_windows",
	    not (any (feature = "have_ntgui", feature = "use_cairo", feature = "have_ns"))))]
#[repr(C)]
pub struct ct_color
{
  pub r: c_int,
  pub g: c_int,
  pub b: c_int,
  pub pixel: c_ulong,

  /* Next in color table collision list.  */
  pub next: *mut ct_color,
}

/* The bucket vector size to use.  Must be prime.  */

/// C: C 7145: `#define CT_SIZE 101`
#[cfg (all (feature = "have_window_system", feature = "have_x_windows",
	    not (any (feature = "have_ntgui", feature = "use_cairo", feature = "have_ns"))))]
pub const CT_SIZE: c_int = 101;

/* Value is a hash of the RGB color given by R, G, and B.  */

/// C: C 7149-7150: `static unsigned ct_hash_rgb (unsigned r, unsigned g, unsigned b)`
#[cfg (all (feature = "have_window_system", feature = "have_x_windows",
	    not (any (feature = "have_ntgui", feature = "use_cairo", feature = "have_ns"))))]
unsafe fn ct_hash_rgb (r: c_uint, g: c_uint, b: c_uint) -> c_uint
{
  return (r << 16) ^ (g << 8) ^ b;
}

/* The color hash table.  */

/// C: C 7157: `static struct ct_color **ct_table;` - C's tentative definition.
#[cfg (all (feature = "have_window_system", feature = "have_x_windows",
	    not (any (feature = "have_ntgui", feature = "use_cairo", feature = "have_ns"))))]
static mut ct_table: *mut *mut ct_color = core::ptr::null_mut ();

/* Number of entries in the color table.  */

/// C: C 7161: `static int ct_colors_allocated;` - C's tentative definition.
#[cfg (all (feature = "have_window_system", feature = "have_x_windows",
	    not (any (feature = "have_ntgui", feature = "use_cairo", feature = "have_ns"))))]
static mut ct_colors_allocated: c_int = 0;
/// C: C 7162-7167: `enum { ct_colors_allocated_max = min (INT_MAX, min (PTRDIFF_MAX, SIZE_MAX) / sizeof (unsigned long)) };` - the value is derived from C's expression (the usual arithmetic conversions make the inner `min` a `usize` comparison, P-59's rule against hand-written numbers).
#[cfg (all (feature = "have_window_system", feature = "have_x_windows",
	    not (any (feature = "have_ntgui", feature = "use_cairo", feature = "have_ns"))))]
pub const ct_colors_allocated_max: c_int
  = core::cmp::min (INT_MAX as usize,
		    core::cmp::min (PTRDIFF_MAX as usize, SIZE_MAX)
		    / core::mem::size_of::<c_ulong> ()) as c_int;

/* Initialize the color table.  */

/// C: C 7171-7172: `static void init_color_table (void)`
#[cfg (all (feature = "have_window_system", feature = "have_x_windows",
	    not (any (feature = "have_ntgui", feature = "use_cairo", feature = "have_ns"))))]
unsafe fn init_color_table ()
{
  let size: c_int = CT_SIZE * core::mem::size_of::<*mut ct_color> () as c_int;
  ct_table = xzalloc (size as usize) as *mut *mut ct_color;
  ct_colors_allocated = 0;
}


/* Free memory associated with the color table.  */

/// C: C 7182-7183: `static void free_color_table (void)`
#[cfg (all (feature = "have_window_system", feature = "have_x_windows",
	    not (any (feature = "have_ntgui", feature = "use_cairo", feature = "have_ns"))))]
unsafe fn free_color_table ()
{
  let mut i: c_int;
  let mut p: *mut ct_color;
  let mut next: *mut ct_color;

  i = 0;
  while i < CT_SIZE
    {
      p = *ct_table.offset (i as isize);
      while !p.is_null ()
	{
	  next = (*p).next;
	  xfree (p as *mut c_void);
	  p = next;
	}
      i += 1;
    }

  xfree (ct_table as *mut c_void);
  ct_table = core::ptr::null_mut ();
}


/* Value is a pixel color for RGB color R, G, B on frame F.  If an
   entry for that color already is in the color table, return the
   pixel color of that entry.  Otherwise, allocate a new color for R,
   G, B, and make an entry in the color table.  */

/// C: C 7205-7206: `static unsigned long lookup_rgb_color (struct frame *f, int r, int g, int b)`
#[cfg (all (feature = "have_window_system", feature = "have_x_windows",
	    not (any (feature = "have_ntgui", feature = "use_cairo", feature = "have_ns"))))]
unsafe fn lookup_rgb_color (f: *mut frame, mut r: c_int, mut g: c_int, mut b: c_int) -> c_ulong
{
  let hash: c_uint = ct_hash_rgb (r as c_uint, g as c_uint, b as c_uint);
  let i: c_int = (hash % CT_SIZE as c_uint) as c_int;
  let mut p: *mut ct_color;
  let dpyinfo: *mut Display_Info;

  /* Handle TrueColor visuals specially, which improves performance by
     two orders of magnitude.  Freeing colors on TrueColor visuals is
     a nop, and pixel colors specify RGB values directly.  See also
     the Xlib spec, chapter 3.1.  */
  dpyinfo = FRAME_DISPLAY_INFO! (f);
  if (*dpyinfo).red_bits > 0
    {
      /* Apply gamma-correction like normal color allocation does.  */
      if (*f).gamma != 0.0
	{
	  let mut color: XColor = core::mem::zeroed ();
	  color.red = r as c_ushort;
	  color.green = g as c_ushort;
	  color.blue = b as c_ushort;
	  gamma_correct (f, &raw mut color);
	  r = color.red as c_int;
	  g = color.green as c_int;
	  b = color.blue as c_int;
	}

      return x_make_truecolor_pixel (dpyinfo, r, g, b);
    }

  p = *ct_table.offset (i as isize);
  while !p.is_null ()
    {
      if (*p).r == r && (*p).g == g && (*p).b == b
	{ break; }
      p = (*p).next;
    }

  if p.is_null ()
    {
      #[cfg (feature = "have_x_windows")]
      {
	let mut color: XColor = core::mem::zeroed ();
	let cmap: Colormap;
	let mut rc: bool;

	if ct_colors_allocated_max <= ct_colors_allocated
	  { return FRAME_FOREGROUND_PIXEL (f); }

	color.red = r as c_ushort;
	color.green = g as c_ushort;
	color.blue = b as c_ushort;

	cmap = FRAME_X_COLORMAP! (f);
	rc = x_alloc_nearest_color (f, cmap, &raw mut color);
	if rc
	  {
	    ct_colors_allocated += 1;
	    p = xmalloc (core::mem::size_of::<ct_color> ()) as *mut ct_color;
	    (*p).r = r;
	    (*p).g = g;
	    (*p).b = b;
	    (*p).pixel = color.pixel;
	    (*p).next = *ct_table.offset (i as isize);
	    *ct_table.offset (i as isize) = p;
	  }
	else
	  { return FRAME_FOREGROUND_PIXEL (f); }
      }
      #[cfg (not (feature = "have_x_windows"))]
      {
	/* C 7243-7245's `#else COLORREF color;` (dead text: this section's own gate,
	   `COLOR_TABLE_SUPPORT`, includes `HAVE_X_WINDOWS` - kept so the arm is carried).  */
	let mut color: COLORREF;

	if ct_colors_allocated_max <= ct_colors_allocated
	  { return FRAME_FOREGROUND_PIXEL (f); }

	#[cfg (feature = "have_ntgui")]
	{ color = PALETTERGB (r, g, b); }
	#[cfg (not (feature = "have_ntgui"))]
	{ color = RGB_TO_ULONG! (r, g, b) as COLORREF; }
	ct_colors_allocated += 1;
	p = xmalloc (core::mem::size_of::<ct_color> ()) as *mut ct_color;
	(*p).r = r;
	(*p).g = g;
	(*p).b = b;
	(*p).pixel = color as c_ulong;
	(*p).next = *ct_table.offset (i as isize);
	*ct_table.offset (i as isize) = p;
      }
    }

  return (*p).pixel;
}


/* Look up pixel color PIXEL which is used on frame F in the color
   table.  If not already present, allocate it.  Value is PIXEL.  */

/// C: C 7296-7297: `static unsigned long lookup_pixel_color (struct frame *f, unsigned long pixel)`
#[cfg (all (feature = "have_window_system", feature = "have_x_windows",
	    not (any (feature = "have_ntgui", feature = "use_cairo", feature = "have_ns"))))]
unsafe fn lookup_pixel_color (f: *mut frame, pixel: c_ulong) -> c_ulong
{
  let i: c_int = (pixel % CT_SIZE as c_ulong) as c_int;
  let mut p: *mut ct_color;

  p = *ct_table.offset (i as isize);
  while !p.is_null ()
    {
      if (*p).pixel == pixel
	{ break; }
      p = (*p).next;
    }

  if p.is_null ()
    {
      let mut color: XColor = core::mem::zeroed ();
      let cmap: Colormap;
      let mut rc: bool;

      if ct_colors_allocated >= ct_colors_allocated_max
	{ return FRAME_FOREGROUND_PIXEL (f); }

      #[cfg (feature = "have_x_windows")]
      {
	cmap = FRAME_X_COLORMAP! (f);
	color.pixel = pixel;
	x_query_colors (f, &raw mut color, 1);
	rc = x_alloc_nearest_color (f, cmap, &raw mut color);
      }
      #[cfg (not (feature = "have_x_windows"))]
      {
	/* C 7320-7327's `#else` arm - dead text for the same reason as `lookup_rgb_color`'s
	   (`COLOR_TABLE_SUPPORT` includes `HAVE_X_WINDOWS`); its `<X11/Xlib.h>` names stay C's
	   tokens.  */
	block_input ();
	cmap = DefaultColormapOfScreen (FRAME_X_SCREEN! (f));
	color.pixel = pixel;
	XQueryColor (core::ptr::null_mut (), cmap, &raw mut color);
	rc = x_alloc_nearest_color (f, cmap, &raw mut color);
	unblock_input ();
      }

      if rc
	{
	  ct_colors_allocated += 1;

	  p = xmalloc (core::mem::size_of::<ct_color> ()) as *mut ct_color;
	  (*p).r = color.red as c_int;
	  (*p).g = color.green as c_int;
	  (*p).b = color.blue as c_int;
	  (*p).pixel = pixel;
	  (*p).next = *ct_table.offset (i as isize);
	  *ct_table.offset (i as isize) = p;
	}
      else
	{ return FRAME_FOREGROUND_PIXEL (f); }
    }
  return (*p).pixel;
}


/* Value is a vector of all pixel colors contained in the color table,
   allocated via xmalloc.  Set *N to the number of colors.  */

/// C: C 7351-7352: `static unsigned long *colors_in_color_table (int *n)`
#[cfg (all (feature = "have_window_system", feature = "have_x_windows",
	    not (any (feature = "have_ntgui", feature = "use_cairo", feature = "have_ns"))))]
unsafe fn colors_in_color_table (n: *mut c_int) -> *mut c_ulong
{
  let mut i: c_int;
  let mut j: c_int;
  let mut p: *mut ct_color;
  let mut colors: *mut c_ulong;

  if ct_colors_allocated == 0
    {
      *n = 0;
      colors = core::ptr::null_mut ();
    }
  else
    {
      colors = xmalloc (ct_colors_allocated as usize * core::mem::size_of::<c_ulong> ())
	as *mut c_ulong;
      *n = ct_colors_allocated;

      i = 0;
      j = 0;
      while i < CT_SIZE
	{
	  p = *ct_table.offset (i as isize);
	  while !p.is_null ()
	    {
	      *colors.offset (j as isize) = (*p).pixel;
	      j += 1;
	      p = (*p).next;
	    }
	  i += 1;
	}
    }

  return colors;
}

/* C 7376-7396: `#else /* COLOR_TABLE_SUPPORT */` - the arm the frozen configuration takes (the
   gate is the negation of the one above).  */
/// C: C 7378-7379: `static unsigned long lookup_rgb_color (struct frame *f, int r, int g, int b)`
#[cfg (all (feature = "have_window_system",
	    not (all (feature = "have_x_windows",
		      not (any (feature = "have_ntgui", feature = "use_cairo",
				feature = "have_ns"))))))]
unsafe fn lookup_rgb_color (f: *mut frame, r: c_int, g: c_int, b: c_int) -> c_ulong
{
  #[cfg (feature = "have_ntgui")]
  { return PALETTERGB (r >> 8, g >> 8, b >> 8); }
  #[cfg (all (not (feature = "have_ntgui"),
	      any (feature = "use_cairo", feature = "have_ns", feature = "have_haiku",
		   feature = "have_android")))]
  { return RGB_TO_ULONG! (r >> 8, g >> 8, b >> 8) as c_ulong; }
  #[cfg (not (any (feature = "have_ntgui", feature = "use_cairo", feature = "have_ns",
		   feature = "have_haiku", feature = "have_android")))]
  {
    let _ = (f, r, g, b);
    xsignal1 (Qfile_error (), build_string (c"This Emacs mishandles this image file type".as_ptr ()));
  }
}

/// C: C 7392-7393: `static void init_color_table (void)`
#[cfg (all (feature = "have_window_system",
	    not (all (feature = "have_x_windows",
		      not (any (feature = "have_ntgui", feature = "use_cairo",
				feature = "have_ns"))))))]
unsafe fn init_color_table ()
{
}

/* ==============================================================================================
   C 7398-7967: the "Algorithms" section - edge detection, colour conversion, cross drawing and
   the heuristic mask.  Ungated in C, so every item carries the file gate alone.  */
/* Edge detection matrices for different edge-detection
   strategies.  */

/// C: C 7406: `static int emboss_matrix[9] = { ... };`
#[cfg (feature = "have_window_system")]
static mut emboss_matrix: [c_int; 9] =
  [
    /* x - 1	x	x + 1  */
    2,     -1,  	  0,		/* y - 1 */
    -1,      0,        1,		/* y     */
    0,      1,       -2		/* y + 1 */
  ];

/// C: C 7413: `static int laplace_matrix[9] = { ... };`
#[cfg (feature = "have_window_system")]
static mut laplace_matrix: [c_int; 9] =
  [
    /* x - 1	x	x + 1  */
    1,      0,  	  0,		/* y - 1 */
    0,      0,        0,		/* y     */
    0,      0,       -1		/* y + 1 */
  ];

/* Value is the intensity of the color whose red/green/blue values
   are R, G, and B.  */

/// C: C 7423: `#define COLOR_INTENSITY(R, G, B) ((2 * (R) + 3 * (G) + (B)) / 6)`
#[cfg (feature = "have_window_system")]
macro_rules! COLOR_INTENSITY {
  ($r:expr, $g:expr, $b:expr) => { (2 * ($r) + 3 * ($g) + ($b)) / 6 };
}

/* gnulib's `<stdckdint.h>` `ckd_mul` as C 7443/7594 uses it: on the frozen target the header
   defines the name as clang's `__builtin_mul_overflow` (P-15), whose port counterpart is the
   `overflowing_mul` pair with a write-back.  The body is the `src/gtkutil.rs` 1055 / `src/coding.rs`
   165 pair - the same two gnulib names, spelled at the unit that reads them.  */
#[cfg (feature = "have_window_system")]
macro_rules! ckd_mul {
  ($dst:ident, $a:expr, $b:expr) => {{
    let (ckd_mul_value, ckd_mul_overflow) = ($a).overflowing_mul ($b);
    $dst = ckd_mul_value;
    ckd_mul_overflow
  }};
}

/* On frame F, return an array of Emacs_Color structures describing image
   IMG->pixmap.  Each Emacs_Color structure has its pixel color set.  RGB_P
   means also fill the red/green/blue members of the Emacs_Color
   structures.  Value is a pointer to the array of Emacs_Color structures,
   allocated with xmalloc; it must be freed by the caller.  */

/// C: C 7432-7433: `static Emacs_Color *image_to_emacs_colors (struct frame *f, struct image *img, bool rgb_p)`
#[cfg (feature = "have_window_system")]
unsafe fn image_to_emacs_colors (f: *mut frame, img: *mut image, rgb_p: bool)
  -> *mut Emacs_Color
{
  let mut x: c_int;
  let mut y: c_int;
  let mut colors: *mut Emacs_Color;
  let mut p: *mut Emacs_Color;
  let mut ximg: Emacs_Pix_Context;
  let mut nbytes: ptrdiff_t;
  #[cfg (feature = "have_ntgui")]
  let mut prev: HGDIOBJ = core::ptr::null_mut ();

  if ckd_mul! (nbytes, core::mem::size_of::<Emacs_Color> () as ptrdiff_t,
	       (*img).width as ptrdiff_t)
      || ckd_mul! (nbytes, nbytes, (*img).height as ptrdiff_t)
      || SIZE_MAX < nbytes as usize
    { memory_full_up (); }
  colors = xmalloc (nbytes as usize) as *mut Emacs_Color;

  /* Get the X image or create a memory device context for IMG. */
  ximg = image_get_x_image_or_dc! (f, img, false, &raw mut prev);

  /* Fill the `pixel' members of the Emacs_Color array.  I wished there
     were an easy and portable way to circumvent XGetPixel.  */
  p = colors;
  y = 0;
  while y < (*img).height
    {
      /* C 7457-7477: `#if !defined USE_CAIRO && !defined HAVE_NS && !defined HAVE_HAIKU &&
	 !defined HAVE_ANDROID` / `#else`.  */
      #[cfg (not (any (feature = "use_cairo", feature = "have_ns", feature = "have_haiku",
		       feature = "have_android")))]
      {
	let row: *mut Emacs_Color = p;
	x = 0;
	while x < (*img).width
	  {
	    (*p).pixel = GET_PIXEL! (ximg, x, y);
	    x += 1;
	    p = p.offset (1);
	  }
	if rgb_p
	  {
	    ((*FRAME_TERMINAL! (f)).query_colors.unwrap ()) (f as *mut c_void, row,
							    (*img).width);
	  }
      }
      #[cfg (any (feature = "use_cairo", feature = "have_ns", feature = "have_haiku",
		  feature = "have_android"))]
      {
	x = 0;
	while x < (*img).width
	  {
	    (*p).pixel = GET_PIXEL! (ximg, x, y);
	    if rgb_p
	      {
		(*p).red = RED16_FROM_ULONG! ((*p).pixel) as c_ushort;
		(*p).green = GREEN16_FROM_ULONG! ((*p).pixel) as c_ushort;
		(*p).blue = BLUE16_FROM_ULONG! ((*p).pixel) as c_ushort;
	      }
	    x += 1;
	    p = p.offset (1);
	  }
      }
      y += 1;
    }

  image_unget_x_image_or_dc! (img, false, ximg, prev);

  return colors;
}

/* C 7485-7529: `#ifdef HAVE_NTGUI` - the Windows pixel writer (`<windows.h>`'s `GetBValue`
   and friends stay C's tokens; the arm is `text-only` here).  */
/* Put a pixel of COLOR at position X, Y in XIMG.  XIMG must have been
   created with CreateDIBSection, with the pointer to the bit values
   stored in ximg->data.  */

/// C: C 7491-7492: `static void XPutPixel (XImage *ximg, int x, int y, COLORREF color)`
#[cfg (all (feature = "have_window_system", feature = "have_ntgui"))]
unsafe fn XPutPixel (ximg: *mut XImage, x: c_int, y: c_int, color: COLORREF)
{
  let width: c_int = (*ximg).info.bmiHeader.biWidth;
  let pixel: *mut c_uchar;

  /* True color images.  */
  if (*ximg).info.bmiHeader.biBitCount == 24
    {
      let mut rowbytes: c_int = width * 3;
      /* Ensure scanlines are aligned on 4 byte boundaries.  */
      if rowbytes % 4 != 0
	{ rowbytes += 4 - (rowbytes % 4); }

      pixel = (*ximg).data.offset ((y * rowbytes + x * 3) as isize);
      /* Windows bitmaps are in BGR order.  */
      *pixel = GetBValue (color) as c_uchar;
      *(pixel.offset (1)) = GetGValue (color) as c_uchar;
      *(pixel.offset (2)) = GetRValue (color) as c_uchar;
    }
  /* Monochrome images.  */
  else if (*ximg).info.bmiHeader.biBitCount == 1
    {
      let mut rowbytes: c_int = width / 8;
      /* Ensure scanlines are aligned on 4 byte boundaries.  */
      if rowbytes % 4 != 0
	{ rowbytes += 4 - (rowbytes % 4); }
      pixel = (*ximg).data.offset ((y * rowbytes + x / 8) as isize);
      /* Filter out palette info.  */
      if color & 0x00ffffff != 0
	{ *pixel = *pixel | (1 << x % 8) as c_uchar; }
      else
	{ *pixel = *pixel & !((1 << x % 8) as c_uchar); }
    }
  else
    { image_error (c"XPutPixel: palette image not supported".as_ptr ()); }
}

/* Create IMG->pixmap from an array COLORS of Emacs_Color structures, whose
   RGB members are set.  F is the frame on which this all happens.
   COLORS will be freed; an existing IMG->pixmap will be freed, too.  */

/// C: C 7535-7536: `static void image_from_emacs_colors (struct frame *f, struct image *img, Emacs_Color *colors)`
#[cfg (feature = "have_window_system")]
unsafe fn image_from_emacs_colors (f: *mut frame, img: *mut image, colors: *mut Emacs_Color)
{
  let mut x: c_int;
  let mut y: c_int;
  let mut ximage: Emacs_Pix_Container;
  let mut p: *mut Emacs_Color;

  /* C 7542-7546: `#ifndef HAVE_ANDROID ximage = NULL; #else ximage = 0; #endif` - both arms
     are C's null pointer for the arm's type.  */
  ximage = core::ptr::null_mut ();

  init_color_table ();

  image_clear_image_1 (f, img, CLEAR_IMAGE_PIXMAP! () | CLEAR_IMAGE_COLORS! ());
  image_create_x_image_and_pixmap (f, img, (*img).width, (*img).height, 0,
				   &raw mut ximage, false);
  p = colors;
  y = 0;
  while y < (*img).height
    {
      x = 0;
      while x < (*img).width
	{
	  let pixel: c_ulong;
	  pixel = lookup_rgb_color (f, (*p).red as c_int, (*p).green as c_int,
				    (*p).blue as c_int);
	  PUT_PIXEL! (ximage, x, y, pixel);
	  x += 1;
	  p = p.offset (1);
	}
      y += 1;
    }

  xfree (colors as *mut c_void);

  image_put_x_image (f, img, ximage, false);
  #[cfg (all (feature = "have_x_windows",
	      not (any (feature = "have_ntgui", feature = "use_cairo", feature = "have_ns"))))]
  {
    (*img).colors = colors_in_color_table (&raw mut (*img).ncolors);
    free_color_table ();
  }
}

/* On frame F, perform edge-detection on image IMG.

   MATRIX is a nine-element array specifying the transformation
   matrix.  See emboss_matrix for an example.

   COLOR_ADJUST is a color adjustment added to each pixel of the
   outgoing image.  */

/// C: C 7580-7581: `static void image_detect_edges (struct frame *f, struct image *img, int *matrix, int color_adjust)`
#[cfg (feature = "have_window_system")]
unsafe fn image_detect_edges (f: *mut frame, img: *mut image, matrix: *mut c_int,
			      color_adjust: c_int)
{
  let colors: *mut Emacs_Color = image_to_emacs_colors (f, img, true);
  let mut new: *mut Emacs_Color;
  let mut p: *mut Emacs_Color;
  let mut x: c_int;
  let mut y: c_int;
  let mut i: c_int;
  let mut sum: c_int;
  let mut nbytes: ptrdiff_t;

  i = 0;
  sum = 0;
  while i < 9
    {
      sum += eabs! (*matrix.offset (i as isize));
      i += 1;
    }

  /// C: C 7592: `#define COLOR(A, X, Y) ((A) + (Y) * img->width + (X))`
  macro_rules! COLOR {
    ($a:expr, $x:expr, $y:expr) => { ($a).offset ((($y) * (*img).width + ($x)) as isize) };
  }

  if ckd_mul! (nbytes, core::mem::size_of::<Emacs_Color> () as ptrdiff_t,
	       (*img).width as ptrdiff_t)
      || ckd_mul! (nbytes, nbytes, (*img).height as ptrdiff_t)
    { memory_full_up (); }
  new = xmalloc (nbytes as usize) as *mut Emacs_Color;

  y = 0;
  while y < (*img).height
    {
      p = COLOR! (new, 0, y);
      (*p).red = (0xffff / 2) as c_ushort;
      (*p).green = (0xffff / 2) as c_ushort;
      (*p).blue = (0xffff / 2) as c_ushort;
      p = COLOR! (new, (*img).width - 1, y);
      (*p).red = (0xffff / 2) as c_ushort;
      (*p).green = (0xffff / 2) as c_ushort;
      (*p).blue = (0xffff / 2) as c_ushort;
      y += 1;
    }

  x = 1;
  while x < (*img).width - 1
    {
      p = COLOR! (new, x, 0);
      (*p).red = (0xffff / 2) as c_ushort;
      (*p).green = (0xffff / 2) as c_ushort;
      (*p).blue = (0xffff / 2) as c_ushort;
      p = COLOR! (new, x, (*img).height - 1);
      (*p).red = (0xffff / 2) as c_ushort;
      (*p).green = (0xffff / 2) as c_ushort;
      (*p).blue = (0xffff / 2) as c_ushort;
      x += 1;
    }

  y = 1;
  while y < (*img).height - 1
    {
      p = COLOR! (new, 1, y);

      x = 1;
      while x < (*img).width - 1
	{
	  let mut r: c_int;
	  let mut g: c_int;
	  let mut b: c_int;
	  let mut yy: c_int;
	  let mut xx: c_int;

	  r = 0;
	  g = 0;
	  b = 0;
	  i = 0;
	  yy = y - 1;
	  while yy < y + 2
	    {
	      xx = x - 1;
	      while xx < x + 2
		{
		  if *matrix.offset (i as isize) != 0
		    {
		      let t: *mut Emacs_Color = COLOR! (colors, xx, yy);
		      r += *matrix.offset (i as isize) * (*t).red as c_int;
		      g += *matrix.offset (i as isize) * (*t).green as c_int;
		      b += *matrix.offset (i as isize) * (*t).blue as c_int;
		    }
		  i += 1;
		  xx += 1;
		}
	      yy += 1;
	    }

	  r = (r / sum + color_adjust) & 0xffff;
	  g = (g / sum + color_adjust) & 0xffff;
	  b = (b / sum + color_adjust) & 0xffff;
	  let v: c_int = COLOR_INTENSITY! (r, g, b);
	  (*p).red = v as c_ushort;
	  (*p).green = v as c_ushort;
	  (*p).blue = v as c_ushort;
	  x += 1;
	  p = p.offset (1);
	}
      y += 1;
    }

  xfree (colors as *mut c_void);
  image_from_emacs_colors (f, img, new);
}

/* Perform the pre-defined `emboss' edge-detection on image IMG
   on frame F.  */

/// C: C 7651-7652: `static void image_emboss (struct frame *f, struct image *img)`
#[cfg (feature = "have_window_system")]
unsafe fn image_emboss (f: *mut frame, img: *mut image)
{
  image_detect_edges (f, img, &raw mut emboss_matrix as *mut c_int, 0xffff / 2);
}

/* Transform image IMG which is used on frame F with a Laplace
   edge-detection algorithm.  The result is an image that can be used
   to draw disabled buttons, for example.  */

/// C: C 7662-7663: `static void image_laplace (struct frame *f, struct image *img)`
#[cfg (feature = "have_window_system")]
unsafe fn image_laplace (f: *mut frame, img: *mut image)
{
  image_detect_edges (f, img, &raw mut laplace_matrix as *mut c_int, 45000);
}

/* Perform edge-detection on image IMG on frame F, with specified
   transformation matrix MATRIX and color-adjustment COLOR_ADJUST.

   MATRIX must be either

   - a list of at least 9 numbers in row-major form
   - a vector of at least 9 numbers

   COLOR_ADJUST nil means use a default; otherwise it must be a
   number.  */

/// C: C 7680-7682: `static void image_edge_detection (struct frame *f, struct image *img, Lisp_Object matrix, Lisp_Object color_adjust)`
#[cfg (feature = "have_window_system")]
unsafe fn image_edge_detection (f: *mut frame, img: *mut image, mut matrix: Lisp_Object,
				mut color_adjust: Lisp_Object)
{
  let mut i: c_int = 0;
  /* C's `int trans[9];` - written only up to `i` (PORTING.md 1.5/0.1-13).  */
  let mut trans: [c_int; 9] = [0; 9];

  if CONSP (matrix)
    {
      i = 0;
      while i < 9 && CONSP (matrix) && NUMBERP (XCAR (matrix))
	{
	  trans [i as usize] = XFLOATINT (XCAR (matrix)) as c_int;
	  i += 1;
	  matrix = XCDR (matrix);
	}
    }
  else if VECTORP (matrix) && ASIZE (matrix) >= 9
    {
      i = 0;
      while i < 9 && NUMBERP (AREF (matrix, i as isize))
	{
	  trans [i as usize] = XFLOATINT (AREF (matrix, i as isize)) as c_int;
	  i += 1;
	}
    }

  if NILP (color_adjust)
    { color_adjust = make_fixnum (0xffff / 2); }

  if i == 9 && NUMBERP (color_adjust)
    { image_detect_edges (f, img, trans.as_mut_ptr (), XFLOATINT (color_adjust) as c_int); }
}

/// C: C 7711-7714: `static void image_pixmap_draw_cross (struct frame *f, Emacs_Pixmap pixmap, int x, int y, unsigned int width, unsigned int height, unsigned long color)`
#[cfg (all (feature = "have_window_system",
	    any (feature = "have_x_windows", feature = "use_cairo", feature = "have_haiku",
		 feature = "have_android")))]
unsafe fn image_pixmap_draw_cross (f: *mut frame, pixmap: Emacs_Pixmap, x: c_int, y: c_int,
				   width: c_uint, height: c_uint, color: c_ulong)
{
  /* C 7716-7758: the four-way `#ifdef USE_CAIRO` / `#elif HAVE_X_WINDOWS` / `#elif HAVE_HAIKU`
     / `#elif HAVE_ANDROID` chain - each arm excludes the earlier conditions.  */
  #[cfg (feature = "use_cairo")]
  {
    let surface: *mut cairo_surface_t
      = cairo_image_surface_create_for_data ((*pixmap).data as *mut c_uchar,
					     (if (*pixmap).bits_per_pixel == 32
					      { CAIRO_FORMAT_RGB24 } else { CAIRO_FORMAT_A8 }),
					     (*pixmap).width, (*pixmap).height,
					     (*pixmap).bytes_per_line);
    let cr: *mut cairo_t = cairo_create (surface);
    cairo_surface_destroy (surface);
    cairo_set_source_rgb (cr, RED_FROM_ULONG! (color) as f64 / 255.0,
			  GREEN_FROM_ULONG! (color) as f64 / 255.0,
			  BLUE_FROM_ULONG! (color) as f64 / 255.0);
    cairo_move_to (cr, x as f64 + 0.5, y as f64 + 0.5);
    cairo_rel_line_to (cr, (width - 1) as f64, (height - 1) as f64);
    cairo_rel_move_to (cr, 0.0, -((height - 1) as f64));
    cairo_rel_line_to (cr, -((width - 1) as f64), (height - 1) as f64);
    cairo_set_line_width (cr, 1.0);
    cairo_stroke (cr);
    cairo_destroy (cr);
  }
  #[cfg (all (not (feature = "use_cairo"), feature = "have_x_windows"))]
  {
    let dpy: *mut Display = FRAME_X_DISPLAY! (f);
    let gc: GC = XCreateGC (dpy, pixmap, 0, core::ptr::null_mut ());

    XSetForeground (dpy, gc, color);
    XDrawLine (dpy, pixmap, gc, x, y, x + width as c_int - 1, y + height as c_int - 1);
    XDrawLine (dpy, pixmap, gc, x, y + height as c_int - 1, x + width as c_int - 1, y);
    XFreeGC (dpy, gc);
  }
  #[cfg (all (not (any (feature = "use_cairo", feature = "have_x_windows")),
	      feature = "have_haiku"))]
  {
    /* `femacs/include/haiku_support.rs` 572's declaration takes `c_int`/`uint32_t` where C's
       `unsigned int`/`unsigned long` are wider.  */
    be_draw_cross_on_pixmap (pixmap, x, y, width as c_int, height as c_int, color as uint32_t);
  }
  #[cfg (all (not (any (feature = "use_cairo", feature = "have_x_windows", feature = "have_haiku")),
	      feature = "have_android"))]
  {
    #[cfg (feature = "not_android_stubify")]
    {
      let gc: *mut android_gc;

      gc = android_create_gc (0, core::ptr::null_mut ());
      android_set_foreground (gc, color);
      android_draw_line (pixmap, gc, x, y, x + width as c_int - 1,
			 y + height as c_int - 1);
      android_draw_line (pixmap, gc, x, y + height as c_int - 1, x + width as c_int - 1, y);
      android_free_gc (gc);
    }
    #[cfg (not (feature = "not_android_stubify"))]
    { emacs_abort (); }
  }
}

/* Transform image IMG on frame F so that it looks disabled.  */

/// C: C 7765-7766: `static void image_disable_image (struct frame *f, struct image *img)`
#[cfg (feature = "have_window_system")]
unsafe fn image_disable_image (f: *mut frame, img: *mut image)
{
  let dpyinfo: *mut Display_Info = FRAME_DISPLAY_INFO! (f);
  /* C 7769-7772: `#ifdef HAVE_NTGUI` `int n_planes = dpyinfo->n_planes * dpyinfo->n_cbits;`
     / `#else` `int n_planes = dpyinfo->n_planes;` - and C 219-221's
     `#define n_planes n_image_planes` applies under HAVE_ANDROID.  */
  #[cfg (feature = "have_ntgui")]
  let n_planes: c_int = (*dpyinfo).n_planes * (*dpyinfo).n_cbits;
  #[cfg (all (not (feature = "have_ntgui"), not (feature = "have_android")))]
  let n_planes: c_int = (*dpyinfo).n_planes;
  #[cfg (all (not (feature = "have_ntgui"), feature = "have_android"))]
  let n_planes: c_int = (*dpyinfo).n_image_planes;

  if n_planes >= 2
    {
      /* Color (or grayscale).  Convert to gray, and equalize.  Just
	 drawing such images with a stipple can look very odd, so
	 we're using this method instead.  */
      let colors: *mut Emacs_Color = image_to_emacs_colors (f, img, true);
      let mut p: *mut Emacs_Color;
      let end: *mut Emacs_Color;
      let h: c_int = 15000;
      let l: c_int = 30000;

      p = colors;
      end = colors.offset (((*img).width * (*img).height) as isize);
      while p < end
	{
	  let i: c_int = COLOR_INTENSITY! ((*p).red as c_int, (*p).green as c_int,
					   (*p).blue as c_int);
	  let i2: c_int = (0xffff - h - l) * i / 0xffff + l;
	  (*p).red = i2 as c_ushort;
	  (*p).green = i2 as c_ushort;
	  (*p).blue = i2 as c_ushort;
	  p = p.offset (1);
	}

      image_from_emacs_colors (f, img, colors);
    }

  /* Draw a cross over the disabled image, if we must or if we
     should.  */
  if n_planes < 2 || cross_disabled_images
    {
      /* C 7801-7848: the `#ifndef HAVE_NTGUI` / `#else` pair.  */
      #[cfg (not (feature = "have_ntgui"))]
      {
	/* C 7802's `#ifndef HAVE_NS` - and the two macros' two arms, which C defines here and
	   uses a few lines below (both are function-local, so the port defines them the same
	   way; `PIX_MASK_DRAW` is the `USE_CAIRO || HAVE_HAIKU || HAVE_ANDROID` arm's name).  */
	#[cfg (not (feature = "have_ns"))]
	{
	  /// C: C 7805: `#define CrossForeground(f) BLACK_PIX_DEFAULT (f)` (the `!USE_CAIRO && !HAVE_HAIKU && !HAVE_ANDROID` arm)
	  #[cfg (not (any (feature = "use_cairo", feature = "have_haiku", feature = "have_android")))]
	  macro_rules! CrossForeground { ($f:expr) => { BLACK_PIX_DEFAULT! ($f) }; }
	  /// C: C 7806: `#define MaskForeground(f)  WHITE_PIX_DEFAULT (f)` (the same arm)
	  #[cfg (not (any (feature = "use_cairo", feature = "have_haiku", feature = "have_android")))]
	  macro_rules! MaskForeground { ($f:expr) => { WHITE_PIX_DEFAULT! ($f) }; }
	  /// C: C 7808: `#define CrossForeground(f) 0` (the `#else /* USE_CAIRO || HAVE_HAIKU */` arm)
	  #[cfg (any (feature = "use_cairo", feature = "have_haiku", feature = "have_android"))]
	  macro_rules! CrossForeground { ($f:expr) => { 0 }; }
	  /// C: C 7809: `#define MaskForeground(f)  PIX_MASK_DRAW` (the same arm)
	  #[cfg (any (feature = "use_cairo", feature = "have_haiku", feature = "have_android"))]
	  macro_rules! MaskForeground { ($f:expr) => { PIX_MASK_DRAW! () }; }

	  #[cfg (not (any (feature = "use_cairo", feature = "have_haiku")))]
	  { image_sync_to_pixmaps (f, img); }
	  image_pixmap_draw_cross (f, (*img).pixmap as Emacs_Pixmap, 0, 0, (*img).width as c_uint,
				   (*img).height as c_uint, CrossForeground! (f));
	  if !(*img).mask.is_null ()
	    { image_pixmap_draw_cross (f, (*img).mask as Emacs_Pixmap, 0, 0,
				       (*img).width as c_uint, (*img).height as c_uint,
				       MaskForeground! (f)); }
	}
      }
      #[cfg (feature = "have_ntgui")]
      {
	let hdc: HDC;
	let bmpdc: HDC;
	let mut prev: HGDIOBJ;

	hdc = get_frame_dc (f);
	bmpdc = CreateCompatibleDC (hdc);
	release_frame_dc (f, hdc);

	prev = SelectObject (bmpdc, (*img).pixmap as HGDIOBJ);

	SetTextColor (bmpdc, BLACK_PIX_DEFAULT! (f) as COLORREF);
	MoveToEx (bmpdc, 0, 0, core::ptr::null_mut ());
	LineTo (bmpdc, (*img).width - 1, (*img).height - 1);
	MoveToEx (bmpdc, 0, (*img).height - 1, core::ptr::null_mut ());
	LineTo (bmpdc, (*img).width - 1, 0);

	if !(*img).mask.is_null ()
	  {
	    SelectObject (bmpdc, (*img).mask as HGDIOBJ);
	    SetTextColor (bmpdc, WHITE_PIX_DEFAULT! (f) as COLORREF);
	    MoveToEx (bmpdc, 0, 0, core::ptr::null_mut ());
	    LineTo (bmpdc, (*img).width - 1, (*img).height - 1);
	    MoveToEx (bmpdc, 0, (*img).height - 1, core::ptr::null_mut ());
	    LineTo (bmpdc, (*img).width - 1, 0);
	  }
	SelectObject (bmpdc, prev);
	DeleteDC (bmpdc);
      }
    }
}


/* Build a mask for image IMG which is used on frame F.  FILE is the
   name of an image file, for error messages.  HOW determines how to
   determine the background color of IMG.  If it is a list '(R G B)',
   with R, G, and B being integers >= 0, take that as the color of the
   background.  Otherwise, determine the background color of IMG
   heuristically.  */

/// C: C 7860-7862: `static void image_build_heuristic_mask (struct frame *f, struct image *img, Lisp_Object how)`
#[cfg (feature = "have_window_system")]
unsafe fn image_build_heuristic_mask (f: *mut frame, img: *mut image, mut how: Lisp_Object)
{
  let mut ximg: Emacs_Pix_Context;
  /* C 7865-7871: `#ifdef HAVE_NTGUI`'s three / `#elif !defined HAVE_NS`'s one.  */
  #[cfg (feature = "have_ntgui")]
  let mut prev: HGDIOBJ = core::ptr::null_mut ();
  #[cfg (feature = "have_ntgui")]
  let mut mask_img: *mut c_char = core::ptr::null_mut ();
  #[cfg (feature = "have_ntgui")]
  let mut row_width: c_int = 0;
  #[cfg (all (not (feature = "have_ntgui"), not (feature = "have_ns")))]
  let mut mask_img: Emacs_Pix_Container = core::ptr::null_mut ();
  let mut x: c_int;
  let mut y: c_int;
  let mut use_img_background: bool;
  let mut bg: c_ulong = 0;

  if !(*img).mask.is_null ()
    { image_clear_image_1 (f, img, CLEAR_IMAGE_MASK! ()); }

  /* C 7879-7890: `#ifndef HAVE_NTGUI` (`#ifndef HAVE_NS`'s create-and-pixmap) / `#else` (the
     Windows bit array).  */
  #[cfg (not (feature = "have_ntgui"))]
  {
    #[cfg (not (feature = "have_ns"))]
    {
      /* Create an image and pixmap serving as mask.  */
      if !image_create_x_image_and_pixmap (f, img, (*img).width, (*img).height, 1,
					   &raw mut mask_img, true)
	{ return; }
    }
  }
  #[cfg (feature = "have_ntgui")]
  {
    /* Create the bit array serving as mask.  */
    row_width = ((*img).width + 7) / 8;
    mask_img = xzalloc ((row_width * (*img).height) as usize) as *mut c_char;
  }

  /* Get the X image or create a memory device context for IMG.  */
  ximg = image_get_x_image_or_dc! (f, img, false, &raw mut prev);

  /* Determine the background color of ximg.  If HOW is `(R G B)'
     take that as color.  Otherwise, use the image's background color. */
  use_img_background = true;

  if CONSP (how)
    {
      /* C's `int rgb[3], i;` - `rgb` is written only up to `i` (PORTING.md 1.5/0.1-13).  */
      let mut rgb: [c_int; 3] = [0; 3];
      let mut i: c_int;

      i = 0;
      while i < 3 && CONSP (how) && FIXNATP (XCAR (how))
	{
	  rgb [i as usize] = (XFIXNAT (XCAR (how)) & 0xffff) as c_int;
	  i += 1;
	  how = XCDR (how);
	}

      if i == 3 && NILP (how)
	{
	  #[cfg (not (feature = "use_cairo"))]
	  {
	    let color_name: Lisp_Object = make_color_name (rgb [0] as c_uint, rgb [1] as c_uint,
							   rgb [2] as c_uint);
	    bg = image_alloc_image_color (f, img, color_name, 0);
	    #[cfg (feature = "have_ntgui")]
	    { bg &= 0x00ffffff; /* Filter out palette info.  */ }
	  }
	  #[cfg (feature = "use_cairo")]
	  { bg = lookup_rgb_color (f, rgb [0], rgb [1], rgb [2]); }
	  use_img_background = false;
	}
    }

  if use_img_background
    { bg = four_corners_best (ximg, (*img).corners.as_mut_ptr (), (*img).width as c_ulong,
			      (*img).height as c_ulong); }

  /* Set all bits in mask_img to 1 whose color in ximg is different
     from the background color bg.  */
  #[cfg (not (feature = "have_ntgui"))]
  {
    y = 0;
    while y < (*img).height
      {
	x = 0;
	while x < (*img).width
	  {
	    #[cfg (not (feature = "have_ns"))]
	    {
	      PUT_PIXEL! (mask_img, x, y, (if GET_PIXEL! (ximg, x, y) != bg
					   { PIX_MASK_DRAW! () } else { PIX_MASK_RETAIN! () }));
	    }
	    #[cfg (feature = "have_ns")]
	    {
	      if XGetPixel (ximg, x, y) == bg
		{ ns_set_alpha (ximg, x, y, 0); }
	    }
	    x += 1;
	  }
	y += 1;
      }
    #[cfg (not (feature = "have_ns"))]
    {
      /* Fill in the background_transparent field while we have the mask handy. */
      image_background_transparent (img, f, mask_img);

      /* Put mask_img into the image.  */
      image_put_x_image (f, img, mask_img, true);
    }
  }
  #[cfg (feature = "have_ntgui")]
  {
    y = 0;
    while y < (*img).height
      {
	x = 0;
	while x < (*img).width
	  {
	    let p: COLORREF = GetPixel (ximg, x, y);
	    if p != bg as COLORREF
	      {
		*mask_img.offset ((y * row_width + x / 8) as isize)
		  |= (1 << (x % 8)) as c_char;
	      }
	    x += 1;
	  }
	y += 1;
      }

    /* Create the mask image.  */
    (*img).mask = w32_create_pixmap_from_bitmap_data ((*img).width, (*img).height, mask_img)
		  as *mut c_void;
    /* Fill in the background_transparent field while we have the mask handy. */
    SelectObject (ximg, (*img).mask as HGDIOBJ);
    image_background_transparent (img, f, ximg);

    /* Was: image_destroy_x_image ((XImagePtr )mask_img); which seems bogus ++kfs */
    xfree (mask_img as *mut c_void);
  }

  image_unget_x_image_or_dc! (img, false, ximg, prev);
}

/* C 7970-7972's banner.  */
/***********************************************************************
		       PBM (mono, gray, color)
 ***********************************************************************/

/* Indices of image specification fields in pbm_format, below.  */

/// C: C 7976-7990: `enum pbm_keyword_index { PBM_TYPE, PBM_FILE, PBM_DATA, PBM_ASCENT, PBM_MARGIN, PBM_RELIEF, PBM_ALGORITHM, PBM_HEURISTIC_MASK, PBM_MASK, PBM_FOREGROUND, PBM_BACKGROUND, PBM_LAST }`
#[cfg (feature = "have_window_system")]
pub const PBM_TYPE: c_int = 0;
#[cfg (feature = "have_window_system")]
pub const PBM_FILE: c_int = 1;
#[cfg (feature = "have_window_system")]
pub const PBM_DATA: c_int = 2;
#[cfg (feature = "have_window_system")]
pub const PBM_ASCENT: c_int = 3;
#[cfg (feature = "have_window_system")]
pub const PBM_MARGIN: c_int = 4;
#[cfg (feature = "have_window_system")]
pub const PBM_RELIEF: c_int = 5;
#[cfg (feature = "have_window_system")]
pub const PBM_ALGORITHM: c_int = 6;
#[cfg (feature = "have_window_system")]
pub const PBM_HEURISTIC_MASK: c_int = 7;
#[cfg (feature = "have_window_system")]
pub const PBM_MASK: c_int = 8;
#[cfg (feature = "have_window_system")]
pub const PBM_FOREGROUND: c_int = 9;
#[cfg (feature = "have_window_system")]
pub const PBM_BACKGROUND: c_int = 10;
#[cfg (feature = "have_window_system")]
pub const PBM_LAST: c_int = 11;

/* Vector of image_keyword structures describing the format
   of valid user-defined image specifications.  */

/// C: C 7995-8008: `static const struct image_keyword pbm_format[PBM_LAST]`
#[cfg (feature = "have_window_system")]
static pbm_format: [image_keyword; PBM_LAST as usize] =
[
  image_keyword { name: c":type".as_ptr (), r#type: IMAGE_SYMBOL_VALUE,
		  mandatory_p: true, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":file".as_ptr (), r#type: IMAGE_STRING_VALUE,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":data".as_ptr (), r#type: IMAGE_STRING_VALUE,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":ascent".as_ptr (), r#type: IMAGE_ASCENT_VALUE,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":margin".as_ptr (), r#type: IMAGE_NON_NEGATIVE_INTEGER_VALUE_OR_PAIR,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":relief".as_ptr (), r#type: IMAGE_INTEGER_VALUE,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":conversion".as_ptr (), r#type: IMAGE_DONT_CHECK_VALUE_TYPE,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":heuristic-mask".as_ptr (), r#type: IMAGE_DONT_CHECK_VALUE_TYPE,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":mask".as_ptr (), r#type: IMAGE_DONT_CHECK_VALUE_TYPE,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":foreground".as_ptr (), r#type: IMAGE_STRING_OR_NIL_VALUE,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () },
  image_keyword { name: c":background".as_ptr (), r#type: IMAGE_STRING_OR_NIL_VALUE,
		  mandatory_p: false, count: false, value: core::ptr::null_mut () }
];

/* Return true if OBJECT is a valid PBM image specification.  */

/// C: C 8012-8028: `static bool pbm_image_p (Lisp_Object object)`
#[cfg (feature = "have_window_system")]
unsafe extern "C" fn pbm_image_p (object: Lisp_Object) -> bool
{
  /* C's `struct image_keyword fmt[PBM_LAST];` filled by the `memcpy` below - zeroed first
     (PORTING.md 1.5/0.1-13).  */
  let mut fmt: [image_keyword; PBM_LAST as usize] = core::mem::zeroed ();

  /* C's `memcpy (fmt, pbm_format, sizeof fmt);`.  */
  memcpy (fmt.as_mut_ptr () as *mut c_void, pbm_format.as_ptr () as *const c_void,
	  core::mem::size_of::<[image_keyword; PBM_LAST as usize]> ());

  if !parse_image_spec (object, fmt.as_mut_ptr (), PBM_LAST, Qpbm ())
    { return false; }

  /* Must specify either :data or :file.  */
  /* The mirror's `count` is a `bool` (`image_keyword`), so C's int sum is written with the
     conversion out (the `xpm_image_p` 8681 note).  */
  return (fmt[PBM_DATA as usize].count as c_int)
    + (fmt[PBM_FILE as usize].count as c_int) == 1;
}


/* Get next char skipping comments in Netpbm header.  Returns -1 at
   end of input.  */

/// C: C 8030-8052: `static int pbm_next_char (char **s, char *end)`
#[cfg (feature = "have_window_system")]
unsafe fn pbm_next_char (s: *mut *mut c_char, end: *mut c_char) -> c_int
{
  /* C's `unsigned char c = *(*s)++;` is the load-then-step pair (Deviation 26): the port
     loads into `c` and steps `*s` as the two statements C's post-increment performs.  */
  let mut c: c_uchar;

  while *s < end
    {
      c = **s as c_uchar;
      *s = (*s).offset (1);
      if c as c_int != b'#' as c_int
	{ return c as c_int; }
      while *s < end
	{
	  c = **s as c_uchar;
	  *s = (*s).offset (1);
	  if c as c_int == b'\n' as c_int || c as c_int == b'\r' as c_int
	    { break; }
	}
    }

  return -1;
}


/* Scan a decimal number from *S and return it.  Advance *S while
   reading the number.  END is the end of the string.  Value is -1 at
   end of input.  */

/// C: C 8054-8076: `static int pbm_scan_number (char **s, char *end)`
#[cfg (feature = "have_window_system")]
unsafe fn pbm_scan_number (s: *mut *mut c_char, end: *mut c_char) -> c_int
{
  let mut c: c_int = 0;
  let mut val: c_int = -1;

  /* Skip white-space.  */
  while { c = pbm_next_char (s, end); c != -1 && c_isspace (c) }
    { }

  if c_isdigit (c)
    {
      /* Read decimal number.  */
      val = c - b'0' as c_int;
      while { c = pbm_next_char (s, end); c != -1 && c_isdigit (c) }
	{ val = 10 * val + c - b'0' as c_int; }
    }

  return val;
}

/* Scan an index from *S and return it.  It is a one-byte unsigned
   index if !TWO_BYTE, and a two-byte big-endian unsigned index if
   TWO_BYTE.  */

/// C: C 8078-8095: `static int pbm_scan_index (char **s, bool two_byte)`
#[cfg (feature = "have_window_system")]
unsafe fn pbm_scan_index (s: *mut *mut c_char, two_byte: bool) -> c_int
{
  let mut p: *mut c_char = *s;
  /* C's `unsigned char c0 = *p++;` - the load-then-step pair (Deviation 26).  */
  let c0: c_uchar = *p as c_uchar;
  p = p.offset (1);
  let mut n: c_int = c0 as c_int;
  if two_byte
    {
      /* C's `unsigned char c1 = *p++;` - the same pair.  */
      let c1: c_uchar = *p as c_uchar;
      p = p.offset (1);
      n = (n << 8) + c1 as c_int;
    }
  *s = p;
  return n;
}


/* Load PBM image IMG for use on frame F.  */

/// C: C 8097-8351: `static bool pbm_load (struct frame *f, struct image *img)`
#[cfg (feature = "have_window_system")]
unsafe extern "C" fn pbm_load (f: *mut frame, img: *mut image) -> bool
{
  let mut raw_p: bool = false;
  let mut x: c_int;
  let mut y: c_int;
  let mut width: c_int;
  let mut height: c_int;
  let mut max_color_idx: c_int = 0;
  let mut specified_file: Lisp_Object;
  /* C's function-scope `enum {PBM_MONO, PBM_GRAY, PBM_COLOR} type;` is the port's
     function-local consts (the `syntax.rs` 3545 shape); `type` is Rust's keyword, so the local
     keeps the `r#type` spelling this file's `image_keyword` member already uses.  */
  const PBM_MONO: c_int = 0;
  const PBM_GRAY: c_int = 1;
  const PBM_COLOR: c_int = 2;
  let mut r#type: c_int;
  /* C's `char *contents = NULL;` - the initial value written out.  */
  let mut contents: *mut c_char = core::ptr::null_mut ();
  let mut end: *mut c_char;
  let mut p: *mut c_char;
  /* C's `Emacs_Pix_Container ximg;` is written by the `image_create_x_image_and_pixmap`
     call below (PORTING.md 1.5).  */
  let mut ximg: Emacs_Pix_Container = core::ptr::null_mut ();

  specified_file = image_spec_value ((*img).spec, QCfile (), core::ptr::null_mut ());

  if STRINGP (specified_file)
    {
      /* C's `ptrdiff_t size;` is written by the call below (PORTING.md 1.5).  */
      let mut size: ptrdiff_t = 0;
      contents = slurp_image (specified_file, &raw mut size, c"PBM".as_ptr ());
      if contents.is_null ()
	{ return false; }

      p = contents;
      end = contents.offset (size as isize);
    }
  else
    {
      let mut data: Lisp_Object;
      data = image_spec_value ((*img).spec, QCdata (), core::ptr::null_mut ());
      if !STRINGP (data)
	{
	  image_invalid_data_error (data);
	  return false;
	}
      p = SSDATA (data);
      end = p.offset (SBYTES (data) as isize);
    }

  /* C's `error:` (C 8113) sits *inside* the first `if` and every later `goto error` jumps
     backward to it, which no Rust `break` can express (PORTING.md 2.2's label shape): the
     labelled block below opens at the first statement that can reach the label and closes after
     the last jump site, and C's label body (the `xfree`/`NO_PIXMAP`/`return 0` triple) stands
     after the block, where every `break 'error` lands.  The success path returns inside the
     block, exactly as C's fall-through reaches its own `xfree (contents); return 1;`.  */
  'error: {
    /* Check magic number.  */
    /* C's `end - p < 2 || *p++ != 'P'`: the post-increment is the load-then-step pair
       (Deviation 26) and `||` evaluates it only when the left arm is false - which is why the
       port builds the pair's result instead of stepping unconditionally.  */
    let mut magic_p: bool = false;
    if 2 <= end.offset_from (p)
      {
	magic_p = *p == b'P' as c_char;
	p = p.offset (1);
      }

    if !magic_p
      {
	image_error (c"Not a PBM image: `%s'".as_ptr (), (*img).spec);
	break 'error;
      }

    /* C's `switch (*p++)` - the same load-then-step pair.  */
    let pbm_magic: c_uchar = *p as c_uchar;
    p = p.offset (1);

    match pbm_magic
      {
	b'1' =>
	  { raw_p = false; r#type = PBM_MONO; }

	b'2' =>
	  { raw_p = false; r#type = PBM_GRAY; }

	b'3' =>
	  { raw_p = false; r#type = PBM_COLOR; }

	b'4' =>
	  { raw_p = true; r#type = PBM_MONO; }

	b'5' =>
	  { raw_p = true; r#type = PBM_GRAY; }

	b'6' =>
	  { raw_p = true; r#type = PBM_COLOR; }

	_ =>
	  {
	    image_error (c"Not a PBM image: `%s'".as_ptr (), (*img).spec);
	    break 'error;
	  }
      }

    /* Read width, height, maximum color-component.  Characters
       starting with `#' up to the end of a line are ignored.  */
    width = pbm_scan_number (&raw mut p, end);
    height = pbm_scan_number (&raw mut p, end);

    if r#type != PBM_MONO
      {
	max_color_idx = pbm_scan_number (&raw mut p, end);
	if max_color_idx > 65535 || max_color_idx < 0
	  {
	    image_error (c"Unsupported maximum PBM color value".as_ptr ());
	    break 'error;
	  }
      }

    if !check_image_size (f, width, height)
      {
	image_size_error ();
	break 'error;
      }

    if !image_create_x_image_and_pixmap (f, img, width, height, 0, &raw mut ximg, false)
      { break 'error; }

    /* Initialize the color hash table.  */
    init_color_table ();

    if r#type == PBM_MONO
      {
	let mut c: c_uchar = 0;
	let mut g: c_int;
	/* C's `struct image_keyword fmt[PBM_LAST];` filled by the `memcpy` below - zeroed first
	   (PORTING.md 1.5/0.1-13).  */
	let mut fmt: [image_keyword; PBM_LAST as usize] = core::mem::zeroed ();
	let mut fg: c_ulong = (*img).face_foreground;
	let mut bg: c_ulong = (*img).face_background;
	/* Parse the image specification.  */
	memcpy (fmt.as_mut_ptr () as *mut c_void, pbm_format.as_ptr () as *const c_void,
		core::mem::size_of::<[image_keyword; PBM_LAST as usize]> ());
	parse_image_spec ((*img).spec, fmt.as_mut_ptr (), PBM_LAST, Qpbm ());

	/* Get foreground and background colors, maybe allocate colors.  */
	if fmt[PBM_FOREGROUND as usize].count
	  && STRINGP (fmt[PBM_FOREGROUND as usize].value)
	  { fg = image_alloc_image_color (f, img, fmt[PBM_FOREGROUND as usize].value, fg); }
	if fmt[PBM_BACKGROUND as usize].count
	  && STRINGP (fmt[PBM_BACKGROUND as usize].value)
	  {
	    bg = image_alloc_image_color (f, img, fmt[PBM_BACKGROUND as usize].value, bg);
	    (*img).background = bg;
	    /* C's `img->background_valid = 1;` - the bit member's storage unit (P-80).  */
	    (*img).bits = (*img).bits & !IMAGE_BACKGROUND_VALID_MASK
			  | ((1 as u32) << IMAGE_BACKGROUND_VALID_SHIFT);
	  }

	/* C 8216-8223: `#ifdef USE_CAIRO`.  */
	#[cfg (all (feature = "have_window_system", feature = "use_cairo"))]
	{
	  /* C's `Emacs_Color fgbg[] = {{.pixel = fg}, {.pixel = bg}};` - C's designated
	     initialisers leave the three channel members zero (PORTING.md 1.5).  */
	  let mut fgbg: [Emacs_Color; 2]
	    = [Emacs_Color { pixel: fg, red: 0, green: 0, blue: 0 },
	       Emacs_Color { pixel: bg, red: 0, green: 0, blue: 0 }];
	  (*FRAME_TERMINAL! (f)).query_colors.unwrap () (f as *mut c_void, fgbg.as_mut_ptr (),
							countof! (fgbg) as c_int);
	  fg = lookup_rgb_color (f, fgbg[0].red as c_int, fgbg[0].green as c_int,
				 fgbg[0].blue as c_int);
	  bg = lookup_rgb_color (f, fgbg[1].red as c_int, fgbg[1].green as c_int,
				 fgbg[1].blue as c_int);
	}

	y = 0;
	while y < height
	  {
	    x = 0;
	    while x < width
	      {
		if raw_p
		  {
		    if x & 7 == 0
		      {
			if p >= end
			  {
			    image_destroy_x_image (ximg);
			    image_clear_image (f, img);
			    image_error (c"Invalid image size in image `%s'".as_ptr (),
					 (*img).spec);
			    break 'error;
			  }
			/* C's `c = *p++;` - the load-then-step pair (Deviation 26).  */
			c = *p as c_uchar;
			p = p.offset (1);
		      }
		    g = (c as c_int) & 0x80;
		    /* C's `c <<= 1;` on the `unsigned char` - the truncating shift.  */
		    c = c << 1;
		  }
		else
		  {
		    /* C's inner `int c = 0;` shadows the outer `unsigned char c` - the
		       shadowing is kept.  */
		    let mut c: c_int = 0;
		    /* Skip white-space and comments.  */
		    while { c = pbm_next_char (&raw mut p, end); c != -1 && c_isspace (c) }
		      { }

		    if c == b'0' as c_int || c == b'1' as c_int
		      { g = c - b'0' as c_int; }
		    else
		      { g = 0; }
		  }

		PUT_PIXEL! (ximg, x, y, if g != 0 { fg } else { bg });
		x += 1;
	      }
	    y += 1;
	  }
      }
    else
      {
	let two_byte: bool = 255 < max_color_idx;

	if raw_p
	  {
	    let mut expected_size: ptrdiff_t = 0;
	    /* C's two `ckd_mul` calls (C 8275-8277) are the port's `overflowing_mul` pair
	       (P-15, the `xnmalloc` 381 shape): the family's argument check reads the
	       numbers the second call passes, which is where C's own multiplicand sits.  */
	    let (ckd_temp, mul_overflow)
	      = (height as ptrdiff_t).overflowing_mul (width as ptrdiff_t);
	    expected_size = ckd_temp;
	    let mut bad: bool = mul_overflow;
	    let (ckd_temp, mul_overflow)
	      = expected_size.overflowing_mul (((if two_byte { 2 } else { 1 })
						* (if r#type == PBM_COLOR { 3 } else { 1 }))
					       as ptrdiff_t);
	    expected_size = ckd_temp;
	    bad |= mul_overflow;
	    bad |= end.offset_from (p) < expected_size;
	    if bad
	      {
		image_destroy_x_image (ximg);
		image_clear_image (f, img);
		image_error (c"Invalid image size in image `%s'".as_ptr (), (*img).spec);
		break 'error;
	      }
	  }

	y = 0;
	while y < height
	  {
	    x = 0;
	    while x < width
	      {
		let mut r: c_int;
		let mut g: c_int;
		let mut b: c_int;

		/* C's `r = g = b = …;` is one statement per assignment in C's right-to-left
		   order (the `dispnew.md` 312 (b) rule), so the read happens once.  */
		if r#type == PBM_GRAY && raw_p
		  {
		    b = pbm_scan_index (&raw mut p, two_byte);
		    g = b;
		    r = g;
		  }
		else if r#type == PBM_GRAY
		  {
		    b = pbm_scan_number (&raw mut p, end);
		    g = b;
		    r = g;
		  }
		else if raw_p
		  {
		    r = pbm_scan_index (&raw mut p, two_byte);
		    g = pbm_scan_index (&raw mut p, two_byte);
		    b = pbm_scan_index (&raw mut p, two_byte);
		  }
		else
		  {
		    r = pbm_scan_number (&raw mut p, end);
		    g = pbm_scan_number (&raw mut p, end);
		    b = pbm_scan_number (&raw mut p, end);
		  }

		if r < 0 || g < 0 || b < 0
		  {
		    image_destroy_x_image (ximg);
		    image_error (c"Invalid pixel value in image `%s'".as_ptr (), (*img).spec);
		    break 'error;
		  }

		/* RGB values are now in the range 0..max_color_idx.
		   Scale this to the range 0..0xffff supported by X.  */
		r = (r as c_double * 65535.0 / max_color_idx as c_double) as c_int;
		g = (g as c_double * 65535.0 / max_color_idx as c_double) as c_int;
		b = (b as c_double * 65535.0 / max_color_idx as c_double) as c_int;
		PUT_PIXEL! (ximg, x, y, lookup_rgb_color (f, r, g, b));
		x += 1;
	      }
	    y += 1;
	  }
      }

    /* C 8326-8331: `#ifdef COLOR_TABLE_SUPPORT` - the arm `COLOR_TABLE_SUPPORT`'s own
       definition selects, the same condition this unit's `init_color_table` carries.  */
    #[cfg (all (feature = "have_window_system", feature = "have_x_windows",
		not (any (feature = "have_ntgui", feature = "use_cairo", feature = "have_ns"))))]
    {
      /* Store in IMG->colors the colors allocated for the image, and
	 free the color table.  */
      (*img).colors = colors_in_color_table (&raw mut (*img).ncolors);
      free_color_table ();
    }

    (*img).width = width;
    (*img).height = height;

    /* Maybe fill in the background field while we have ximg handy.  */

    if NILP (image_spec_value ((*img).spec, QCbackground (), core::ptr::null_mut ()))
      /* Casting avoids a GCC warning.  */
      { IMAGE_BACKGROUND! (img, f, PIX_CONTAINER_TO_CONTEXT! (ximg)); }

    /* Put ximg into the image.  */
    image_put_x_image (f, img, ximg, false);

    /* X and W32 versions did it here, MAC version above.  ++kfs
       img->width = width;
       img->height = height; */

    xfree (contents as *mut c_void);
    return true;
  }

  /* error: - C 8113-8115's label body, hoisted to the labelled block's end (see the note
     above its opening).  */
  xfree (contents as *mut c_void);
  (*img).pixmap = NO_PIXMAP! () as *mut c_void;
  return false;
}


/***********************************************************************
				Tests
 ***********************************************************************/

/// C: C 13306-13311: `DEFUN ("imagep", Fimagep, Simagep, 1, 1, 0, …)`
#[cfg (feature = "have_window_system")]
DEFUN! ("imagep", Fimagep, Simagep, 1, 1, 0,
       doc: "Value is non-nil if SPEC is a valid image specification.",
       (spec: Lisp_Object),
       {
  return if valid_image_p (spec) { Qt () } else { Qnil () };
});

/* C 13313-13331: `#ifdef GLYPH_DEBUG`.  */
#[cfg (all (feature = "have_window_system", feature = "glyph_debug"))]
DEFUN! ("lookup-image", Flookup_image, Slookup_image, 1, 1, 0,
       doc: "",
       (spec: Lisp_Object),
       {
  /* C's `ptrdiff_t id = -1;`.  */
  let mut id: ptrdiff_t = -1;

  if valid_image_p (spec)
    { id = lookup_image (SELECTED_FRAME (), spec, -1); }

  debug_print (spec);
  return make_fixnum (id);
});


/***********************************************************************
			    Initialization
 ***********************************************************************/

/// C: C 13335-13364: `DEFUN ("image-transforms-p", Fimage_transforms_p, Simage_transforms_p, 0, 1, 0, …)`
#[cfg (feature = "have_window_system")]
DEFUN! ("image-transforms-p", Fimage_transforms_p, Simage_transforms_p, 0, 1, 0,
       doc: "Test whether FRAME supports image transformation.\nReturn list of capabilities if FRAME supports native transforms, nil otherwise.\nFRAME defaults to the selected frame.\nThe list of capabilities can include one or more of the following:\n\n - the symbol `scale' if FRAME supports image scaling\n - the symbol `rotate90' if FRAME supports image rotation only by angles\n    that are integral multiples of 90 degrees.",
       (frame: Lisp_Object),
       {
  let f: *mut frame = decode_live_frame (frame);
  if FRAME_WINDOW_P (f)
    {
      /* C 13349-13361: `#ifdef HAVE_NATIVE_TRANSFORMS` and its three-way `#elif` chain, one arm
	 per condition with each excluding the ones before it (the file's gate-mapping rule).  */
      #[cfg (feature = "have_native_transforms")]
      {
	#[cfg (any (feature = "have_imagemagick", feature = "use_cairo", feature = "have_ns",
		    feature = "have_haiku", feature = "have_android"))]
	{ return list2 (Qscale (), Qrotate90 ()); }
	#[cfg (all (feature = "have_x_windows", feature = "have_xrender",
		    not (any (feature = "have_imagemagick", feature = "use_cairo",
			      feature = "have_ns", feature = "have_haiku",
			      feature = "have_android"))))]
	{
	  if (*FRAME_DISPLAY_INFO! (f)).xrender_supported_p
	    { return list2 (Qscale (), Qrotate90 ()); }
	}
	#[cfg (all (feature = "have_ntgui",
		    not (any (feature = "have_imagemagick", feature = "use_cairo",
			      feature = "have_ns", feature = "have_haiku",
			      feature = "have_android")),
		    not (all (feature = "have_x_windows", feature = "have_xrender"))))]
	{
	  return if w32_image_rotations_p () { list2 (Qscale (), Qrotate90 ()) }
	  else { list1 (Qscale ()) };
	}
      }
    }

  return Qnil ();
});

/// C: C 13367-13388: `DEFUN ("image-cache-size", Fimage_cache_size, Simage_cache_size, 0, 0, 0, …)`
#[cfg (feature = "have_window_system")]
DEFUN! ("image-cache-size", Fimage_cache_size, Simage_cache_size, 0, 0, 0,
       doc: "Return the size of the image cache.",
       (),
       {
  let mut tail: Lisp_Object;
  let mut frame: Lisp_Object;
  let mut total: intmax_t = 0;

  FOR_EACH_FRAME! (tail, frame, {
    if FRAME_WINDOW_P (XFRAME (frame))
      { total += image_frame_cache_size (XFRAME (frame)) as intmax_t; }
  });

  /* C 13378-13385: `#if defined (HAVE_WEBP) || defined (HAVE_GIF)`.  */
  #[cfg (any (feature = "have_webp", feature = "have_gif"))]
  {
    let mut pcache: *mut anim_cache = anim_cache;
    while !pcache.is_null ()
      {
	total += (*pcache).byte_size;
	pcache = (*pcache).next;
      }
  }

  return make_int (total);
});

/// C: C 13391-13401: `DEFUN ("init-image-library", Finit_image_library, Sinit_image_library, 1, 1, 0, …)`
#[cfg (feature = "have_window_system")]
DEFUN! ("init-image-library", Finit_image_library, Sinit_image_library, 1, 1, 0,
       doc: "Initialize image library implementing image type TYPE.\nReturn t if TYPE is a supported image type.\n\nIf image libraries are loaded dynamically (currently the case only on\nMS-Windows), load the library for TYPE if it is not yet loaded, using\nthe library file(s) specified by `dynamic-library-alist'.",
       (r#type: Lisp_Object),
       {
  return if !lookup_image_type (r#type).is_null () { Qt () } else { Qnil () };
});

/* C 13403-13432: `initialize_image_type` - the WINDOWSNT arm under `feature = "windowsnt"` and
   C's unconditional `return true;` outside it.  */
#[cfg (feature = "have_window_system")]
unsafe fn initialize_image_type (r#type: *const image_type) -> bool
{
  /* C 13406-13427: `#ifdef WINDOWSNT`.  */
  #[cfg (feature = "windowsnt")]
  {
    let typesym: Lisp_Object = builtin_lisp_symbol ((*r#type).r#type as usize);

    /* C 13409-13412: `# if HAVE_NATIVE_IMAGE_API`.  */
    #[cfg (feature = "have_native_image_api")]
    {
      if image_can_use_native_api (typesym)
	{ return true; }
    }

    let tested: Lisp_Object = Fassq (typesym, Vlibrary_cache);
    /* If we failed to load the library before, don't try again.  */
    if CONSP (tested)
      { return !NILP (XCDR (tested)); }

    /* C's `bool (*init) (void) = type->init;`.  */
    let init: Option<unsafe extern "C" fn () -> bool> = (*r#type).init;
    if init.is_some ()
      {
	let type_valid: bool = init.unwrap () ();
	Vlibrary_cache = Fcons (Fcons (typesym, if type_valid { Qt () } else { Qnil () }),
				Vlibrary_cache);
	return type_valid;
      }
  }
  /* C's `Vlibrary_cache` is a `lread.c` global: the port imports it through the globals
     mirror's extern declaration (the `decompress.rs` 468 / `comp.rs` 527 shape).  */
  return true;
}

/* Array of supported image types.  */

/// C: C 13433-13475: `static struct image_type const image_types[]` - C's array with one entry
/// per configured format.  Its *length* is configuration-dependent, which a Rust array's type
/// cannot spell without enumerating the arm combinations, so the port carries the same entries
/// as a slice and `countof (image_types)` reads `image_types.len ()` (this slice's Deviation).
#[cfg (feature = "have_window_system")]
static image_types: &[image_type] =
&[
  /* C 13435-13437: `#ifdef HAVE_GHOSTSCRIPT`.  */
  #[cfg (feature = "have_ghostscript")]
  image_type { r#type: iQpostscript as c_int, valid_p: Some (gs_image_p), load_img: Some (gs_load),
	       free_img: Some (image_clear_image) },
  /* C 13438-13441: `#ifdef HAVE_IMAGEMAGICK`.  */
  #[cfg (feature = "have_imagemagick")]
  image_type { r#type: iQimagemagick as c_int, valid_p: Some (imagemagick_image_p),
	       load_img: Some (imagemagick_load), free_img: Some (image_clear_image) },
  /* C 13442-13445: `#ifdef HAVE_RSVG`.  */
  #[cfg (all (feature = "have_rsvg", feature = "windowsnt"))]
  image_type { r#type: iQsvg as c_int, valid_p: Some (svg_image_p), load_img: Some (svg_load),
	       free_img: Some (image_clear_image), init: Some (init_svg_functions) },
  #[cfg (all (feature = "have_rsvg", not (feature = "windowsnt")))]
  image_type { r#type: iQsvg as c_int, valid_p: Some (svg_image_p), load_img: Some (svg_load),
	       free_img: Some (image_clear_image) },
  /* C 13446-13449: `#if defined HAVE_PNG`.  */
  #[cfg (all (feature = "have_png", feature = "windowsnt"))]
  image_type { r#type: iQpng as c_int, valid_p: Some (png_image_p), load_img: Some (png_load),
	       free_img: Some (image_clear_image), init: Some (init_png_functions) },
  #[cfg (all (feature = "have_png", not (feature = "windowsnt")))]
  image_type { r#type: iQpng as c_int, valid_p: Some (png_image_p), load_img: Some (png_load),
	       free_img: Some (image_clear_image) },
  /* C 13450-13453: `#if defined HAVE_GIF`.  */
  #[cfg (all (feature = "have_gif", feature = "windowsnt"))]
  image_type { r#type: iQgif as c_int, valid_p: Some (gif_image_p), load_img: Some (gif_load),
	       free_img: Some (image_clear_image), init: Some (init_gif_functions) },
  #[cfg (all (feature = "have_gif", not (feature = "windowsnt")))]
  image_type { r#type: iQgif as c_int, valid_p: Some (gif_image_p), load_img: Some (gif_load),
	       free_img: Some (image_clear_image) },
  /* C 13454-13457: `#if defined HAVE_TIFF`.  */
  #[cfg (all (feature = "have_tiff", feature = "windowsnt"))]
  image_type { r#type: iQtiff as c_int, valid_p: Some (tiff_image_p), load_img: Some (tiff_load),
	       free_img: Some (image_clear_image), init: Some (init_tiff_functions) },
  #[cfg (all (feature = "have_tiff", not (feature = "windowsnt")))]
  image_type { r#type: iQtiff as c_int, valid_p: Some (tiff_image_p), load_img: Some (tiff_load),
	       free_img: Some (image_clear_image) },
  /* C 13458-13461: `#if defined HAVE_JPEG`.  */
  #[cfg (all (feature = "have_jpeg", feature = "windowsnt"))]
  image_type { r#type: iQjpeg as c_int, valid_p: Some (jpeg_image_p), load_img: Some (jpeg_load),
	       free_img: Some (image_clear_image), init: Some (init_jpeg_functions) },
  #[cfg (all (feature = "have_jpeg", not (feature = "windowsnt")))]
  image_type { r#type: iQjpeg as c_int, valid_p: Some (jpeg_image_p), load_img: Some (jpeg_load),
	       free_img: Some (image_clear_image) },
  /* C 13462-13466: `#if defined HAVE_XPM || defined HAVE_NS || defined HAVE_HAIKU ||
     defined HAVE_PGTK || defined HAVE_ANDROID`.  */
  #[cfg (all (any (feature = "have_xpm", feature = "have_ns", feature = "have_haiku",
		   feature = "have_pgtk", feature = "have_android"),
	      feature = "windowsnt"))]
  image_type { r#type: iQxpm as c_int, valid_p: Some (xpm_image_p), load_img: Some (xpm_load),
	       free_img: Some (image_clear_image), init: Some (init_xpm_functions) },
  #[cfg (all (any (feature = "have_xpm", feature = "have_ns", feature = "have_haiku",
		   feature = "have_pgtk", feature = "have_android"),
	      not (feature = "windowsnt")))]
  image_type { r#type: iQxpm as c_int, valid_p: Some (xpm_image_p), load_img: Some (xpm_load),
	       free_img: Some (image_clear_image) },
  /* C 13467-13470: `#if defined HAVE_WEBP`.  */
  #[cfg (all (feature = "have_webp", feature = "windowsnt"))]
  image_type { r#type: iQwebp as c_int, valid_p: Some (webp_image_p), load_img: Some (webp_load),
	       free_img: Some (image_clear_image), init: Some (init_webp_functions) },
  #[cfg (all (feature = "have_webp", not (feature = "windowsnt")))]
  image_type { r#type: iQwebp as c_int, valid_p: Some (webp_image_p), load_img: Some (webp_load),
	       free_img: Some (image_clear_image) },
  /* C 13472-13474's three ungated entries.  */
  image_type { r#type: iQxbm as c_int, valid_p: Some (xbm_image_p), load_img: Some (xbm_load),
	       free_img: Some (image_clear_image) },
  image_type { r#type: iQpbm as c_int, valid_p: Some (pbm_image_p), load_img: Some (pbm_load),
	       free_img: Some (image_clear_image) },
  image_type { r#type: iQcanvas as c_int, valid_p: Some (canvas_image_p),
	       load_img: Some (canvas_load), free_img: Some (image_clear_image) },
];

/* C 13477-13480: `#if HAVE_NATIVE_IMAGE_API`'s `native_image_type`.  */
#[cfg (all (feature = "have_window_system", feature = "have_native_image_api"))]
static native_image_type: image_type =
  image_type { r#type: iQnative_image as c_int, valid_p: Some (native_image_p),
	       load_img: Some (native_image_load), free_img: Some (image_clear_image) };

/* Look up image TYPE, and return a pointer to its image_type structure.
   Return a null pointer if TYPE is not a known image type.  */

/// C: C 13483-13503: `static struct image_type const * lookup_image_type (Lisp_Object type)`
#[cfg (feature = "have_window_system")]
unsafe fn lookup_image_type (r#type: Lisp_Object) -> *const image_type
{
  /* C 13487-13491: `#if HAVE_NATIVE_IMAGE_API`.  */
  #[cfg (feature = "have_native_image_api")]
  {
    if image_can_use_native_api (r#type)
      { return &raw const native_image_type; }
  }

  /* C 13493's `countof (image_types)` is the slice's length here (the table's own Deviation).  */
  let mut i: c_int = 0;
  while i < image_types.len () as c_int
    {
      let r: *const image_type = &raw const image_types[i as usize];
      if EQ (r#type, builtin_lisp_symbol ((*r).r#type as usize))
	{ return if initialize_image_type (r) { r } else { core::ptr::null () }; }
      i += 1;
    }
  return core::ptr::null ();
}

/* Prune old entries from the animation cache.
   If CLEAR, remove all animation cache entries.  */

/// C: C 13504-13515: `void image_prune_animation_caches (bool clear)`
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn image_prune_animation_caches (clear: bool)
{
  /* FIXME: Consolidate these animation cache implementations.  */
  /* C 13508-13510: `#if defined (HAVE_WEBP) || defined (HAVE_GIF)`.  */
  #[cfg (any (feature = "have_webp", feature = "have_gif"))]
  { anim_prune_animation_cache (if clear { Qt () } else { Qnil () }); }
  /* C 13511-13513: `#ifdef HAVE_IMAGEMAGICK`.  */
  #[cfg (feature = "have_imagemagick")]
  { imagemagick_prune_animation_cache (clear); }
}


/* C 13796's `decode_env_path (0, PATH_BITMAPS, 0)` names an `epaths.h` macro, and the frozen
   reference build's value is `#define PATH_BITMAPS ""` (measured: `build/emacs-ref/src/epaths.h`
   67; C's other arm, line 98, is the Android one).  The port keeps it at the use site with the
   measured value, the `callproc.rs` 275 `PATH_DOC` shape.  */
const PATH_BITMAPS: &std::ffi::CStr = c"";

/* C's seven `globals.f_<name>` fields this unit's `syms_of_image` registers (its DEFVAR_* calls
   at C 13521, 13527, 13759, 13765, 13769, 13779 and 13790), defined here as the port's storage -
   C's `struct emacs_globals` is zero-initialised and the DEFVAR macros only take each field's
   address, so the initialisers are `lisp_h_Qnil ()` / `false` / `0` (docs/status.md 3.29, the
   `src/cmds.rs` row and the `src/alloc.rs` 8022-8025 note).  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub static mut Vimage_types: Lisp_Object = lisp_h_Qnil ();              // C 13521
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub static mut Vmax_image_size: Lisp_Object = lisp_h_Qnil ();           // C 13527
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub static mut cross_disabled_images: bool = false;                     // C 13759
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub static mut Vx_bitmap_file_path: Lisp_Object = lisp_h_Qnil ();       // C 13765
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub static mut Vimage_cache_eviction_delay: Lisp_Object = lisp_h_Qnil ();   // C 13769
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub static mut Vimage_scaling_factor: Lisp_Object = lisp_h_Qnil ();     // C 13779
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub static mut imagemagick_render_type: EMACS_INT = 0;                  // C 13790

/// C: C 13516-13805: `void syms_of_image (void)`
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_image ()
{
  /* Must be defined now because we're going to update it below, while
     defining the supported image types.  */
  DEFVAR_LISP! (c"image-types", Vimage_types,
    "List of potentially supported image types.\nEach element of the list is a symbol for an image type, like `jpeg' or `png'.\nTo check whether it is really supported, use `image-type-available-p'.");
  Vimage_types = Qnil ();

  DEFVAR_LISP! (c"max-image-size", Vmax_image_size,
    "Maximum size of images.\nEmacs will not load an image into memory if its pixel width or\npixel height exceeds this limit.\n\nIf the value is an integer, it directly specifies the maximum\nimage height and width, measured in pixels.  If it is a floating\npoint number, it specifies the maximum image height and width\nas a ratio to the frame height and width.  If the value is\nnon-numeric, there is no explicit limit on the size of images.");
  Vmax_image_size = make_float (MAX_IMAGE_SIZE);

  /* Other symbols.  */
  DEFSYM! (Qcount, "count");
  DEFSYM! (Qextension_data, "extension-data");
  DEFSYM! (Qdelay, "delay");
  DEFSYM! (Qauto, "auto");

  /* Keywords.  */
  DEFSYM! (QCascent, ":ascent");
  DEFSYM! (QCmargin, ":margin");
  DEFSYM! (QCrelief, ":relief");
  DEFSYM! (QCconversion, ":conversion");
  DEFSYM! (QCcolor_symbols, ":color-symbols");
  DEFSYM! (QCheuristic_mask, ":heuristic-mask");
  DEFSYM! (QCindex, ":index");
  DEFSYM! (QCcrop, ":crop");
  DEFSYM! (QCrotation, ":rotation");
  DEFSYM! (QCmatrix, ":matrix");
  DEFSYM! (QCscale, ":scale");
  DEFSYM! (QCtransform_smoothing, ":transform-smoothing");
  DEFSYM! (QCcolor_adjustment, ":color-adjustment");
  DEFSYM! (QCmask, ":mask");
  DEFSYM! (QCflip, ":flip");

  /* Other symbols.  */
  DEFSYM! (Qlaplace, "laplace");
  DEFSYM! (Qemboss, "emboss");
  DEFSYM! (Qedge_detection, "edge-detection");
  DEFSYM! (Qheuristic, "heuristic");

  DEFSYM! (Qpostscript, "postscript");
  DEFSYM! (QCmax_width, ":max-width");
  DEFSYM! (QCmax_height, ":max-height");

  DEFSYM! (Qem, "em");
  DEFSYM! (Qch, "ch");
  DEFSYM! (Qcw, "cw");

  /* C 13565-13571: `#ifdef HAVE_NATIVE_TRANSFORMS`.  */
  #[cfg (feature = "have_native_transforms")]
  {
    DEFSYM! (Qscale, "scale");
    DEFSYM! (Qrotate, "rotate");
    DEFSYM! (Qrotate90, "rotate90");
    DEFSYM! (Qcrop, "crop");
  }

  /* C 13573-13581: `#ifdef HAVE_GHOSTSCRIPT`.  */
  #[cfg (feature = "have_ghostscript")]
  {
    add_image_type (Qpostscript ());
    DEFSYM! (QCloader, ":loader");
    DEFSYM! (QCpt_width, ":pt-width");
    DEFSYM! (QCpt_height, ":pt-height");
    DEFSYM! (Qgs_load_image, "gs-load-image");
  }

  /* C 13583-13621: `#ifdef WINDOWSNT`.  */
  #[cfg (feature = "windowsnt")]
  {
    /* Versions of libpng, libgif, and libjpeg that we were compiled with,
       or -1 if no PNG/GIF support was compiled in.  This is tested by
       w32-win.el to correctly set up the alist used to search for the
       respective image libraries.  */
    DEFSYM! (Qlibpng_version, "libpng-version");
    Fset (Qlibpng_version (), if cfg! (feature = "have_png")
				{ make_fixnum (PNG_LIBPNG_VER) }
			      else
				{ make_fixnum (-1) });
    DEFSYM! (Qlibgif_version, "libgif-version");
    Fset (Qlibgif_version (), if cfg! (feature = "have_gif")
				{ make_fixnum (GIFLIB_MAJOR * 10000
					       + GIFLIB_MINOR * 100
					       + GIFLIB_RELEASE) }
			      else
				{ make_fixnum (-1) });
    DEFSYM! (Qlibjpeg_version, "libjpeg-version");
    Fset (Qlibjpeg_version (), if cfg! (feature = "have_jpeg")
				 { make_fixnum (JPEG_LIB_VERSION) }
			       else
				 { make_fixnum (-1) });
  }

  DEFSYM! (Qcanvas, "canvas");
  add_image_type (Qcanvas ());

  canvas_map = make_hash_table (&raw const hashtest_eq, DEFAULT_HASH_SIZE as EMACS_INT, Weak_Key);
  staticpro (&raw const canvas_map);
  defsubr (&raw mut Scanvas_refresh);

  DEFSYM! (Qpbm, "pbm");
  add_image_type (Qpbm ());

  DEFSYM! (Qxbm, "xbm");
  add_image_type (Qxbm ());

  /* C 13641-13646: `#if defined (HAVE_XPM) || … || defined (HAVE_ANDROID)`.  */
  #[cfg (any (feature = "have_xpm", feature = "have_ns", feature = "have_haiku",
	      feature = "have_pgtk", feature = "have_android"))]
  {
    DEFSYM! (Qxpm, "xpm");
    add_image_type (Qxpm ());
  }

  /* C 13648-13651: `#if defined (HAVE_JPEG) || defined (HAVE_NATIVE_IMAGE_API)`.  */
  #[cfg (any (feature = "have_jpeg", feature = "have_native_image_api"))]
  {
    DEFSYM! (Qjpeg, "jpeg");
    add_image_type (Qjpeg ());
  }

  /* C 13653-13656: `#if defined (HAVE_TIFF) || defined (HAVE_NATIVE_IMAGE_API)`.  */
  #[cfg (any (feature = "have_tiff", feature = "have_native_image_api"))]
  {
    DEFSYM! (Qtiff, "tiff");
    add_image_type (Qtiff ());
  }

  /* C 13658-13661: `#if defined (HAVE_GIF) || defined (HAVE_NATIVE_IMAGE_API)`.  */
  #[cfg (any (feature = "have_gif", feature = "have_native_image_api"))]
  {
    DEFSYM! (Qgif, "gif");
    add_image_type (Qgif ());
  }

  /* C 13663-13666: `#if defined (HAVE_PNG) || defined (HAVE_NATIVE_IMAGE_API)`.  */
  #[cfg (any (feature = "have_png", feature = "have_native_image_api"))]
  {
    DEFSYM! (Qpng, "png");
    add_image_type (Qpng ());
  }

  /* C 13668-13682: `#if defined (HAVE_WEBP) || (defined (HAVE_NATIVE_IMAGE_API) &&
     (defined (HAVE_NS) || defined (HAVE_HAIKU)))`.  */
  #[cfg (any (feature = "have_webp",
	      all (feature = "have_native_image_api",
		   any (feature = "have_ns", feature = "have_haiku"))))]
  {
    DEFSYM! (Qwebp, "webp");
    DEFSYM! (Qwebpdemux, "webpdemux");
    /* C 13672-13680: `#if !defined (NS_IMPL_GNUSTEP) || defined (HAVE_WEBP)`.  */
    #[cfg (any (not (feature = "ns_impl_gnustep"), feature = "have_webp"))]
    { add_image_type (Qwebp ()); }
    /* C 13673-13679's `#else` arm.  */
    #[cfg (all (feature = "ns_impl_gnustep", not (feature = "have_webp")))]
    {
      /* On GNUstep, WEBP support is provided via ImageMagick only if
	 gnustep-gui is built with --enable-imagemagick.  */
      if image_can_use_native_api (Qwebp ())
	{ add_image_type (Qwebp ()); }
    }
  }

  /* C 13684-13687: `#if defined (HAVE_IMAGEMAGICK)`.  */
  #[cfg (feature = "have_imagemagick")]
  {
    DEFSYM! (Qimagemagick, "imagemagick");
    add_image_type (Qimagemagick ());
  }

  /* C 13689-13722: `#if defined (HAVE_RSVG)` with its `#elif defined HAVE_NATIVE_IMAGE_API &&
     (defined HAVE_NS || defined HAVE_HAIKU)`.  */
  #[cfg (feature = "have_rsvg")]
  {
    DEFSYM! (Qsvg, "svg");
    DEFSYM! (QCbase_uri, ":base-uri");
    DEFSYM! (QCcss, ":css");
    add_image_type (Qsvg ());
    /* C 13697-13706: `#ifdef HAVE_NTGUI`.  */
    #[cfg (feature = "have_ntgui")]
    {
      /* Other libraries used directly by svg code.  */
      DEFSYM! (Qgdk_pixbuf, "gdk-pixbuf");
      DEFSYM! (Qglib, "glib");
      /* C 13702-13704: `#if LIBRSVG_CHECK_VERSION (2, 32, 0)`.  */
      #[cfg (feature = "librsvg_2_32") ]
      { DEFSYM! (Qgio, "gio"); }
      DEFSYM! (Qgobject, "gobject");
    }
  }
  #[cfg (all (not (feature = "have_rsvg"), feature = "have_native_image_api",
	      any (feature = "have_ns", feature = "have_haiku")))]
  {
    DEFSYM! (Qsvg, "svg");

    /* On Haiku, the SVG translator may not be installed.  On GNUstep, SVG
       support is provided by ImageMagick so not guaranteed.  Furthermore,
       some distros (e.g., Debian) ship ImageMagick's SVG module in a
       separate binary package which may not be installed.  */
    if image_can_use_native_api (Qsvg ())
      { add_image_type (Qsvg ()); }
  }

  /* C 13724-13736: `#ifdef HAVE_NS`.  */
  #[cfg (feature = "have_ns")]
  {
    DEFSYM! (Qheic, "heic");
    /* C 13726-13734: `#ifdef NS_IMPL_COCOA`.  */
    #[cfg (feature = "ns_impl_cocoa")]
    { add_image_type (Qheic ()); }
    #[cfg (not (feature = "ns_impl_cocoa"))]
    {
      /* HEIC support in gnustep-gui is provided by ImageMagick.  */
      if image_can_use_native_api (Qheic ())
	{ add_image_type (Qheic ()); }
    }
  }

  /* C 13738-13753: `#if HAVE_NATIVE_IMAGE_API`.  */
  #[cfg (feature = "have_native_image_api")]
  {
    DEFSYM! (Qnative_image, "native-image");

    /* C 13741-13744: `# if defined HAVE_NTGUI || defined HAVE_HAIKU`.  */
    #[cfg (any (feature = "have_ntgui", feature = "have_haiku"))]
    {
      DEFSYM! (Qbmp, "bmp");
      add_image_type (Qbmp ());
    }

    /* C 13746-13749: `# ifdef HAVE_NTGUI`.  */
    #[cfg (feature = "have_ntgui")]
    {
      DEFSYM! (Qgdiplus, "gdiplus");
      DEFSYM! (Qshlwapi, "shlwapi");
    }
  }

  defsubr (&raw mut Sinit_image_library);
  /* C 13757-13759: `#ifdef HAVE_IMAGEMAGICK`.  */
  #[cfg (feature = "have_imagemagick")]
  { defsubr (&raw mut Simagemagick_types); }
  defsubr (&raw mut Sclear_image_cache);
  defsubr (&raw mut Simage_flush);
  defsubr (&raw mut Simage_size);
  defsubr (&raw mut Simage_mask_p);
  defsubr (&raw mut Simage_metadata);
  defsubr (&raw mut Simage_cache_size);
  defsubr (&raw mut Simagep);

  /* C 13767-13769: `#ifdef GLYPH_DEBUG`.  */
  #[cfg (feature = "glyph_debug")]
  { defsubr (&raw mut Slookup_image); }

  DEFSYM! (QCanimate_buffer, ":animate-buffer");
  DEFSYM! (QCanimate_tardiness, ":animate-tardiness");
  DEFSYM! (QCanimate_position, ":animate-position");

  defsubr (&raw mut Simage_transforms_p);

  DEFVAR_BOOL! (c"cross-disabled-images", cross_disabled_images,
    "Non-nil means always draw a cross over disabled images.\nDisabled images are those having a `:conversion disabled' property.\nA cross is always drawn on black & white displays.");
  cross_disabled_images = false;

  DEFVAR_LISP! (c"x-bitmap-file-path", Vx_bitmap_file_path,
    "List of directories to search for window system bitmap files.");
  Vx_bitmap_file_path = decode_env_path (core::ptr::null (), PATH_BITMAPS.as_ptr (), false);

  DEFVAR_LISP! (c"image-cache-eviction-delay", Vimage_cache_eviction_delay,
    "Maximum time after which images are removed from the cache.\nWhen an image has not been displayed this many seconds, Emacs\nautomatically removes it from the image cache.  If the cache contains\na large number of images, the actual eviction time may be shorter.\nThe value can also be nil, meaning the cache is never cleared.\n\nThe function `clear-image-cache' disregards this variable.");
  Vimage_cache_eviction_delay = make_fixnum (300);

  DEFVAR_LISP! (c"image-scaling-factor", Vimage_scaling_factor,
    "When displaying images, apply this scaling factor before displaying.\nThis is not supported for all image types, and is mostly useful\nwhen you have a high-resolution monitor.\nThe value is either a floating point number (where numbers higher\nthan 1 means to increase the size and lower means to shrink the\nsize), or the symbol `auto', which will compute a scaling factor\nbased on the font pixel size.");
  Vimage_scaling_factor = Qauto ();

  /* C 13788-13804: `#ifdef HAVE_IMAGEMAGICK`.  */
  #[cfg (feature = "have_imagemagick")]
  {
    DEFVAR_INT! (c"imagemagick-render-type", imagemagick_render_type,
      "Integer indicating which ImageMagick rendering method to use.\nThe options are:\n  0 -- the default method (pixel pushing)\n  1 -- a newer method (\"MagickExportImagePixels\") that may perform\n       better (speed etc) in some cases, but has not been as thoroughly\n       tested with Emacs as the default method.  This method requires\n       ImageMagick version 6.4.6 (approximately) or later.");
    /* MagickExportImagePixels is in 6.4.6-9, but not 6.4.4-10.  */
    imagemagick_render_type = 0;

    DEFSYM! (Qimage_format_suffixes, "image-format-suffixes");
    DEFSYM! (QCformat, ":format");
  }
}
