// femacs: generated mirror of the frozen build's C-ABI exports - the C-ABI entry points the
// macOS Swift units import by name (`@_silgen_name`, tools/check-swift-imports.py).
//
// This is **not** a port of a reference file: C has no functions with these names.  Each one is
// the C-ABI spelling of a C construct (a `struct frame` member, a `FRAME_*` accessor macro or a
// bitfield of `bits1`) that the Swift lanes import by name, so the core has to provide it as a
// real symbol.  The per-name table - the Swift import, the Rust item that provides it, and the C
// line it stands for - is `docs/ports/abi_shim.md`; the rulings behind it are
// `docs/reviews/2026-10-04-swift-import-rulings.md`.
//
// Why a unit of its own rather than the unit that owns the record: a mirror is `include!`d into
// every consumer, so a definition there is one definition per unit (`E0428`), and a name that C
// never defines as a function is rejected by `extract-abi.py`'s "invented exported symbol" rule
// when it stands beside a C file (measured).  A unit with no C counterpart is outside every
// `--check` pairing, and `xtask` is the precedent for a product with no C source (status 3.241).
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_unsafe)]

include!("include/config.rs");                 // C 20: `#include <config.h>`
include!("../lib/include/stdio.rs");           // C 22: `#include <stdio.h>`
include!("../lib/include/stdlib.rs");          // C 23: `#include <stdlib.h>`
include!("../lib/include/errno.rs");           // C 24: `#include <errno.h>`
include!("../lib/include/limits.in.rs");       // C 25: `#include <limits.h>`
include!("../lib/include/stdint.in.rs");       // lisp.h's width family
include!("../lib/include/string.rs");          // lisp.h 30: `#include <string.h>`
include!("../lib/include/c-ctype.rs");         // C 27: `#include <c-ctype.h>`
include!("../lib/include/stdcountof.rs");      // lisp.h 28: `<stdcountof.h>` (`countof`)
include!("include/lisp.rs");                   // C 29: `#include "lisp.h"`
include!("include/character.rs");              // lisp.h's `CHARACTERP`/`MAX_CHAR` users
include!("include/intervals.rs");              // buffer.h 27: `#include "intervals.h"`
include!("include/systime.rs");                // lisp.h: `Time`/`struct timespec`
include!("include/itree.rs");                  // intervals.h: `itree_node`/`itree_tree`
include!("include/composite.rs");              // coding.h 284: `#include "composite.h"`
include!("include/coding.rs");                 // dispextern.h 29: `#include "coding.h"`
include!("include/font.rs");                   // dispextern.h's font names
include!("include/termhooks.rs");              // frame.h 24: `#include "termhooks.h"`
include!("include/frame.rs");                  // C 38: `#include "frame.h"`
include!("include/keyboard.rs");               // C 37: `#include "keyboard.h"`
include!("include/buffer.rs");                 // C 35: `#include "buffer.h"`
include!("include/blockinput.rs");             // C 39: `#include "blockinput.h"`
include!("include/termchar.rs");               // C 40: `#include "termchar.h"`
include!("include/dispextern.rs");             // C 42: `#include "dispextern.h"`
include!("include/window.rs");                 // C 43: `#include "window.h"`
include!("include/cm.rs");                     // C 47: `#include "cm.h"`
include!("include/pdumper.rs");                // C 55: `#include "pdumper.h"`

/* ---------------------------------------------------------------------------------------------
   `struct frame` readers.  The record itself is `include/frame.rs`'s (its members were measured
   there); every function below reads the member C's macro or member access reads, and the
   `/// C:` line carries C's own text with the reference's line number.
   --------------------------------------------------------------------------------------------- */

/// `frame_left_pos` is the ABI of C's `f->left_pos` (`struct frame` member, frame.h 604); the
/// Swift unit `nsmenu.swift` imports it.
/// C: int left_pos, top_pos;   (frame.h 604)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_left_pos (f: *mut c_void) -> c_int
{
  unsafe { (*f.cast::<frame> ()).left_pos }
}

/// C: int left_pos, top_pos;   (frame.h 604)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_top_pos (f: *mut c_void) -> c_int
{
  unsafe { (*f.cast::<frame> ()).top_pos }
}

/// C: Lisp_Object title;   (frame.h 162)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_title (f: *mut c_void) -> Lisp_Object
{
  unsafe { (*f.cast::<frame> ()).title }
}

/// C: Lisp_Object icon_name;   (frame.h 158)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_icon_name (f: *mut c_void) -> Lisp_Object
{
  unsafe { (*f.cast::<frame> ()).icon_name }
}

/// C: Lisp_Object param_alist;   (frame.h 209)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_param_alist (f: *mut c_void) -> Lisp_Object
{
  unsafe { (*f.cast::<frame> ()).param_alist }
}

/// C: #define FRAME_SELECTED_WINDOW(f) f->selected_window   (frame.h 1204)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_selected_window (f: *mut c_void) -> Lisp_Object
{
  unsafe { (*f.cast::<frame> ()).selected_window }
}

/// C: Lisp_Object menu_bar_vector;   (frame.h 234)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_menu_bar_vector (f: *mut c_void) -> Lisp_Object
{
  unsafe { (*f.cast::<frame> ()).menu_bar_vector }
}

/// C: Lisp_Object tool_bar_items;   (frame.h 283)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_tool_bar_items (f: *mut c_void) -> Lisp_Object
{
  unsafe { (*f.cast::<frame> ()).tool_bar_items }
}

/// C: int menu_bar_items_used;   (frame.h 307)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_menu_bar_items_used (f: *mut c_void) -> c_int
{
  unsafe { (*f.cast::<frame> ()).menu_bar_items_used }
}

/// C: int n_tool_bar_items;   (frame.h 573)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_n_tool_bar_items (f: *mut c_void) -> c_int
{
  unsafe { (*f.cast::<frame> ()).n_tool_bar_items }
}

/* ---------------------------------------------------------------------------------------------
   `struct frame` writers.  Same record, same table (`docs/ports/abi_shim.md`); C writes these
   through plain member assignment (`f->border_width = v`), which is what each body does.
   --------------------------------------------------------------------------------------------- */

/// C: int border_width;   (frame.h 613)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_set_border_width (f: *mut c_void, v: c_int)
{
  unsafe { (*f.cast::<frame> ()).border_width = v; }
}

/// C: int child_frame_border_width;   (frame.h 617)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_set_child_frame_border_width (f: *mut c_void, v: c_int)
{
  unsafe { (*f.cast::<frame> ()).child_frame_border_width = v; }
}

/// C: int internal_border_width;   (frame.h 622)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_set_internal_border_width (f: *mut c_void, v: c_int)
{
  unsafe { (*f.cast::<frame> ()).internal_border_width = v; }
}

/// C: int menu_bar_lines;   (frame.h 635; the reader is `FRAME_MENU_BAR_LINES`, frame.h 1113)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_set_menu_bar_lines (f: *mut c_void, v: c_int)
{
  unsafe { (*f.cast::<frame> ()).menu_bar_lines = v; }
}

/// C: int tab_bar_lines;   (frame.h 558; the reader is `FRAME_TAB_BAR_LINES`, frame.h 1126)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_set_tab_bar_lines (f: *mut c_void, v: c_int)
{
  unsafe { (*f.cast::<frame> ()).tab_bar_lines = v; }
}

/// C: int tab_bar_height;   (frame.h 561; the reader is `FRAME_TAB_BAR_HEIGHT`, frame.h 1127)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_set_tab_bar_height (f: *mut c_void, v: c_int)
{
  unsafe { (*f.cast::<frame> ()).tab_bar_height = v; }
}

/// C: unsigned long background_pixel;   (frame.h 732; the reader is `FRAME_BACKGROUND_PIXEL`)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_set_background_pixel (f: *mut c_void, v: c_ulong)
{
  unsafe { (*f.cast::<frame> ()).background_pixel = v; }
}

/// C: unsigned long foreground_pixel;   (frame.h 733; the reader is `FRAME_FOREGROUND_PIXEL`)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_set_foreground_pixel (f: *mut c_void, v: c_ulong)
{
  unsafe { (*f.cast::<frame> ()).foreground_pixel = v; }
}

/* ---------------------------------------------------------------------------------------------
   `frames.bits1` accessors, and the three names the rulings of status 3.242 fixed.  The bit
   shifts are `include/frame.rs`'s (`FRAME_WANTS_MODELINE_SHIFT` 11, `FRAME_EXPLICIT_NAME_SHIFT`
   15, `FRAME_OUTPUT_METHOD_SHIFT` 21 with `FRAME_OUTPUT_METHOD_MASK` 0x0F, the two
   `FRAME_TAB_BAR_*_SHIFT` 30/31); C's own text is on each `/// C:` line.
   --------------------------------------------------------------------------------------------- */

/// C: bool_bf explicit_name : 1;   (frame.h 417)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_explicit_name (f: *mut c_void) -> bool
{
  unsafe { ((*f.cast::<frame> ()).bits1 >> FRAME_EXPLICIT_NAME_SHIFT) & 1 != 0 }
}

/// C: bool_bf explicit_name : 1;   (frame.h 417)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_set_explicit_name (f: *mut c_void, v: bool)
{
  unsafe {
    let bits = &mut (*f.cast::<frame> ()).bits1;
    *bits = (*bits & !(1 << FRAME_EXPLICIT_NAME_SHIFT)) | ((v as u32) << FRAME_EXPLICIT_NAME_SHIFT);
  }
}

/// C: bool_bf wants_modeline : 1;   (frame.h 402; the reader is `FRAME_WANTS_MODELINE_P`, 1183)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_set_wants_modeline (f: *mut c_void, v: bool)
{
  unsafe {
    let bits = &mut (*f.cast::<frame> ()).bits1;
    *bits = (*bits & !(1 << FRAME_WANTS_MODELINE_SHIFT)) | ((v as u32) << FRAME_WANTS_MODELINE_SHIFT);
  }
}

/// C: ENUM_BF (output_method) output_method : 4;   (frame.h 446; `ENUM_BF` is three bits of
/// padding plus the four-bit field, which is why the port's mask is 0x0F at shift 21)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_set_output_method (f: *mut c_void, v: c_int)
{
  unsafe {
    let bits = &mut (*f.cast::<frame> ()).bits1;
    *bits = (*bits & !(FRAME_OUTPUT_METHOD_MASK << FRAME_OUTPUT_METHOD_SHIFT))
            | (((v as u32) & FRAME_OUTPUT_METHOD_MASK) << FRAME_OUTPUT_METHOD_SHIFT);
  }
}

/// C: bool_bf tab_bar_redisplayed : 1;   (frame.h 503)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_tab_bar_redisplayed (f: *mut c_void) -> bool
{
  unsafe { ((*f.cast::<frame> ()).bits1 >> FRAME_TAB_BAR_REDISPLAYED_SHIFT) & 1 != 0 }
}

/// C: bool_bf tab_bar_resized : 1;   (frame.h 504)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_tab_bar_resized (f: *mut c_void) -> bool
{
  unsafe { ((*f.cast::<frame> ()).bits1 >> FRAME_TAB_BAR_RESIZED_SHIFT) & 1 != 0 }
}

/// C: bool_bf tab_bar_resized : 1;   (frame.h 504)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_set_tab_bar_resized (f: *mut c_void, v: bool)
{
  unsafe {
    let bits = &mut (*f.cast::<frame> ()).bits1;
    *bits = (*bits & !(1 << FRAME_TAB_BAR_RESIZED_SHIFT)) | ((v as u32) << FRAME_TAB_BAR_RESIZED_SHIFT);
  }
}

/// C: Lisp_Object tab_bar_window;   (frame.h 254, **inside `#ifdef HAVE_WINDOW_SYSTEM`**)
///
/// `HAVE_WINDOW_SYSTEM` is undef in this configuration, so C never compiles that member and this
/// mirror has no such field (frame.h 254 sits in the arm the frozen configuration does not
/// compile; the port's own use of the member is behind) `#[cfg (feature = "have_window_system")]`).  The frozen answer is
/// `Qnil`, the shape `Fnative_comp_available_p` uses for a value this configuration does not have
/// (status 3.242's ruling 1) - **not** a field invented to host it.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_tab_bar_window (f: *mut c_void) -> Lisp_Object
{
  let _ = f;
  Qnil ()
}

/// C: #ifdef HAVE_WINDOW_SYSTEM / #define FRAME_EXTERNAL_MENU_BAR(f) (f)->external_menu_bar /
/// C: #else / #define FRAME_EXTERNAL_MENU_BAR(f) false / #endif   (frame.h 1080-1082)
///
/// The frozen configuration takes the `#else` arm, so there is no member to write: the export is
/// a no-op that keeps the C-ABI entry the Swift unit imports (status 3.242's ruling 2).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_set_external_menu_bar (f: *mut c_void, v: c_int)
{
  let _ = (f, v);
}

/// C: #ifdef HAVE_WINDOW_SYSTEM / #define FRAME_EXTERNAL_TOOL_BAR(f) (f)->external_tool_bar /
/// C: #else / #define FRAME_EXTERNAL_TOOL_BAR(f) false / #endif   (frame.h 1095-1097)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_set_external_tool_bar (f: *mut c_void, v: c_int)
{
  let _ = (f, v);
}

/* ---------------------------------------------------------------------------------------------
   Batch 5, group 1: the `struct frame` members the Swift lanes' `nsfns.m` port reads and writes.
   Every member here is in the frozen record (`include/frame.rs`), so the export carries no gate;
   the flag positions are that mirror's `FRAME_*_SHIFT` constants, and each `/// C:` line is C's
   own text with the reference's line number.
   --------------------------------------------------------------------------------------------- */

/// `frame_name` is the ABI of C's `f->name` (`struct frame` member, frame.h 154); the Swift unit
/// `nsfns.swift` imports it (`nsfns.swift` 252) and its `nsfns.m` port reads it in six places.
/// C: Lisp_Object name;   (frame.h 154)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_name (f: *mut c_void) -> Lisp_Object
{
  unsafe { (*f.cast::<frame> ()).name }
}

/// C: struct terminal *terminal;   (frame.h 648; C's writes are src/nsfns.m 1267 and 2988,
/// `f->terminal = dpyinfo->terminal;`)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_set_terminal (f: *mut c_void, t: *mut c_void)
{
  unsafe { (*f.cast::<frame> ()).terminal = t.cast::<terminal> (); }
}

/// C: bool_bf no_split : 1;   (frame.h 411; C's writes are src/nsfns.m 1474 `f->no_split = …`
/// and 3139 `f->no_split = true;`)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_set_no_split (f: *mut c_void, v: bool)
{
  unsafe {
    let bits = &mut (*f.cast::<frame> ()).bits1;
    *bits = (*bits & !(1 << FRAME_NO_SPLIT_SHIFT)) | ((v as u32) << FRAME_NO_SPLIT_SHIFT);
  }
}

/// C: bool_bf undecorated : 1;   (frame.h 449; C's write is src/nsfns.m 1404
/// `FRAME_UNDECORATED (f) = !NILP (tem) && !EQ (tem, Qunbound);`)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_set_undecorated (f: *mut c_void, v: bool)
{
  unsafe {
    let bits = &mut (*f.cast::<frame> ()).bits1;
    *bits = (*bits & !(1 << FRAME_UNDECORATED_SHIFT)) | ((v as u32) << FRAME_UNDECORATED_SHIFT);
  }
}

/// C: bool_bf can_set_window_size : 1;   (frame.h 491; C's writes are src/nsfns.m 1545 and 3149,
/// `f->can_set_window_size = true;`)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_set_can_set_window_size (f: *mut c_void, v: bool)
{
  unsafe {
    let bits = &mut (*f.cast::<frame> ()).bits1;
    *bits = (*bits & !(1 << FRAME_CAN_SET_WINDOW_SIZE_SHIFT))
            | ((v as u32) << FRAME_CAN_SET_WINDOW_SIZE_SHIFT);
  }
}

/// C: bool_bf was_invisible : 1;   (frame.h 530; the second flag word, so this writer and the
/// `was_invisible!` macro of `frame.rs` 276-278 address the same bit; C's write is src/nsfns.m
/// 1569 `f->was_invisible = true;`)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_set_was_invisible (f: *mut c_void, v: bool)
{
  unsafe {
    let bits = &mut (*f.cast::<frame> ()).bits2;
    *bits = (*bits & !(1 << FRAME_WAS_INVISIBLE_SHIFT)) | ((v as u32) << FRAME_WAS_INVISIBLE_SHIFT);
  }
}

/* ---------------------------------------------------------------------------------------------
   Batch 5, group 2: the `struct window`, `struct buffer` and `struct terminal` members the same
   Swift lane reads.  Those three records belong to the core and no Swift file declares their
   layout, so the read is a C-ABI entry named after the member - the rule the lanes' own headers
   state (`nsfns.swift` 503-505, 370-375, `nsterm.swift` 818-825).
   --------------------------------------------------------------------------------------------- */

/// C: Lisp_Object contents;   (window.h 143)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn window_contents (w: *mut c_void) -> Lisp_Object
{
  unsafe { (*w.cast::<window> ()).contents }
}

/// C: struct glyph_matrix *current_matrix;   (window.h 224)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn window_current_matrix (w: *mut c_void) -> *mut c_void
{
  unsafe { (*w.cast::<window> ()).current_matrix }
}

/// C: BVAR (XBUFFER (buf), filename)   (buffer.h 308; the member `BVAR` concatenates is
/// `Lisp_Object filename_;`, buffer.h 330)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn buffer_filename (b: *mut c_void) -> Lisp_Object
{
  unsafe { (*b.cast::<buffer> ()).filename_ }
}

/// C: int id;   (termhooks.h 516)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn terminal_id (t: *mut c_void) -> c_int
{
  unsafe { (*t.cast::<terminal> ()).id }
}

/// C: union display_info { … struct ns_display_info *ns; … };   (termhooks.h 533, the member at
/// 538 - "nsterm.h").  The port keeps the untyped pointer the union holds (P-72 rule 3), so the
/// read is a plain pointer even though the Swift declaration names `ns_display_info`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn terminal_display_info_ns (t: *mut c_void) -> *mut c_void
{
  unsafe { (*t.cast::<terminal> ()).display_info.ns }
}

/* ---------------------------------------------------------------------------------------------
   Batch 5, group 3: the arm-only writers.  `struct ns_output` (nsterm.rs 419-489) and
   `struct ns_display_info` (nsterm.rs 317-392) are `#[cfg (feature = "have_ns")]`, and `HAVE_NS`
   is undef in the frozen configuration (`docs/baseline/config/darwin-arm64.macros.tsv` 1481), so
   each export carries the gate of the record it writes - the `frame_set_double_buffered` ruling
   (`docs/status.md` 3.255 item 3): the ratchet row stays with "only a HAVE_NS build can clear
   it".  Exporting any of them ungated would invent a host for a member the frozen record does not
   have (P-08).
   --------------------------------------------------------------------------------------------- */

/// C's prototype is lisp.h's (`extern void *xzalloc (size_t)`); the port's definition is
/// `femacs/src/alloc.rs` 775-783 (`#[unsafe(no_mangle)] pub unsafe extern "C" fn xzalloc`), which
/// is the symbol the frozen build's `_xzalloc` resolves to.
#[cfg (feature = "have_ns")]
unsafe extern "C" { fn xzalloc (size: size_t) -> *mut c_void; }

/// C: f->output_data.ns = xzalloc (sizeof *f->output_data.ns);   (src/nsfns.m 1270 and 2991)
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_alloc_ns_output (f: *mut c_void)
{
  unsafe {
    (*f.cast::<frame> ()).output_data.ns = xzalloc (std::mem::size_of::<ns_output> ());
  }
}

/// C: FRAME_FONTSET (f) = -1;   (src/nsfns.m 1272 and 2994; the macro is nsterm.h 1096,
/// `#define FRAME_FONTSET(f) ((f)->output_data.ns->fontset)`)
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_set_fontset (f: *mut c_void, v: c_int)
{
  unsafe {
    let ns = (*f.cast::<frame> ()).output_data.ns as *mut ns_output;
    (*ns).fontset = v;
  }
}

/// C: FRAME_DISPLAY_INFO (f) = dpyinfo;   (src/nsfns.m 1280 and 2995; the macro is nsterm.h 1032,
/// `#define FRAME_DISPLAY_INFO(f) ((f)->output_data.ns->display_info)`)
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_set_display_info (f: *mut c_void, dpyinfo: *mut c_void)
{
  unsafe {
    let ns = (*f.cast::<frame> ()).output_data.ns as *mut ns_output;
    (*ns).display_info = dpyinfo as *mut ns_display_info;
  }
}

/// C: #if defined (NS_IMPL_COCOA) && MAC_OS_X_VERSION_MIN_REQUIRED >= 101400 /
/// C: #define FRAME_DOUBLE_BUFFERED(f) ((f)->output_data.ns->double_buffered) / #endif
/// (nsterm.h 1051-1053; C's writes are src/nsfns.m 813 `FRAME_DOUBLE_BUFFERED (f) = NILP
/// (new_value);` and 1464 `FRAME_DOUBLE_BUFFERED (f) = NILP (tem) || EQ (tem, Qunbound);`, and
/// the Swift import is `nsfns.swift` 495-496).
///
/// The gate is the mirror's own pair, `have_ns` + `ns_impl_cocoa` (nsterm.rs 582-584 on the
/// macro, 487-488 on the member): the member lives on `ns_output`, itself `have_ns`-gated, so a
/// lone `ns_impl_cocoa` gate would name a type the mirror does not have in that configuration.
/// Status 3.255 item 3 rules the export out of the ungated set; this is that export.
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_cocoa")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_set_double_buffered (f: *mut c_void, v: bool)
{
  unsafe {
    let ns = (*f.cast::<frame> ()).output_data.ns as *mut ns_output;
    (*ns).double_buffered = v;
  }
}
