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
// `struct ns_output` / `struct ns_display_info` - the two records the batch-6 entries below read
// and write - are `nsterm.h`'s, and their mirror sits behind the same `HAVE_NS` C gates the
// whole file does (`include/nsterm.rs` 29, which is why every entry that names them carries
// `have_ns` too).  A unit that names a mirrored type has to reach the mirror (P-72), so the
// include carries the gate of the entries that need it: in the frozen configuration the line is
// stripped before expansion and the frozen object is unchanged (measured, `## Evidence`).
#[cfg (feature = "have_ns")]
include!("include/nsterm.rs");                // nsterm.h: `struct ns_output`, the `FRAME_*` macros

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

/* ---------------------------------------------------------------------------------------------
   Batch 6, group 1: `struct ns_output`'s member accessors - the whole `ns_output_*` /
   `ns_output_set_*` family the ratchet carried (14 reads + 28 writes).  Every entry reads or
   writes a member of `struct ns_output` (nsterm.h 935-1022), whose mirror is
   `include/nsterm.rs` 419-489; C keeps that record behind `#ifdef HAVE_NS`, and the mirror keeps
   the gate, so **all 42 carry it** - the `frame_set_double_buffered` shape of status 3.255 item
   3 - and the frozen object answers none of them (the expected reading; `## Evidence`).
   `HAVE_NS`, `NS_IMPL_COCOA` and `NS_IMPL_GNUSTEP` are all undef in the frozen configuration
   (`docs/baseline/config/darwin-arm64.macros.tsv` 1481/2996/2997), so the arm's text is recorded
   rather than compiled (P-51).

   Why these are A-class entries and not B: ruling 3.255 makes the Swift-side expansion the
   first choice when the importing file has declared the record's layout, and the family's
   importing units have no `ns_output` prefix (`nsfns.swift` 246-248's own comment says the names
   stand for members of a record the core owns); expanding them needs the record's layout, which
   is a core fact.  The bodies spell C's own accessor where C has one (`FRAME_NS_VIEW`,
   `FRAME_NS_WINDOW`, `FRAME_FOREGROUND_COLOR`, `FRAME_BACKGROUND_COLOR`, `FRAME_CURSOR_COLOR`,
   `FRAME_DISPLAY_INFO` - the mirror's `macro_rules!`, nsterm.rs 513-577) and C's own
   `f->output_data.ns->member` chain (through the mirror's `FRAME_OUTPUT_DATA!`) otherwise, so
   each entry stands for the access C's `.m` files make.
   --------------------------------------------------------------------------------------------- */

/// C: `EmacsView *view;` (nsterm.h 938; the frozen `#else` half is `void *view;` 948) and
/// C: `#define FRAME_NS_VIEW(f) ((f)->output_data.ns->view)` (nsterm.h 1045) - C reads it at
/// C: nsfns.m 145, 281.
/// Swift `nsfns.swift` 275 (`(UnsafeMutableRawPointer?) -> NSView?`); the write's entry is
/// `ns_output_set_view` below.
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_view (f: *mut c_void) -> *mut c_void
{
  unsafe { FRAME_NS_VIEW! (f.cast::<frame> ()) }
}

/// C: `id miniimage;` (nsterm.h 939; the frozen `#else` half is `void *miniimage;` 949) - C
/// C: reads it at nsfns.m 858 and nsterm.m 9729.  No accessor macro in C, so the body spells
/// C: `f->output_data.ns->miniimage` itself.
/// Swift `nsfns.swift` 278 (`-> AnyObject?`) and `nsterm.swift` 1850 (`-> NSImage?`) - one
/// C-ABI entry, two declarations; both spellings are one pointer.
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_miniimage (f: *mut c_void) -> *mut c_void
{
  unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).miniimage
  }
}

/// C: `NSColor *cursor_color;` (nsterm.h 940; the frozen `#else` half is `void *cursor_color;`
/// C: 950) and `#define FRAME_CURSOR_COLOR(f) ((f)->output_data.ns->cursor_color)` (nsterm.h
/// C: 1046) - C reads it at nsterm.m 3283.
/// Swift `nsfns.swift` 282 (`-> NSColor?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_cursor_color (f: *mut c_void) -> *mut c_void
{
  unsafe { FRAME_CURSOR_COLOR! (f.cast::<frame> ()) }
}

/// C: `NSColor *foreground_color;` (nsterm.h 941; the frozen `#else` half is
/// C: `void *foreground_color;` 951) and `#define FRAME_FOREGROUND_COLOR(f)
/// C: ((f)->output_data.ns->foreground_color)` (nsterm.h 1037) - C reads it at nsfont.m 1198
/// C: and nsterm.m 4878.
/// Swift `nsfns.swift` 286 (`-> NSColor?`); the write's entry is below.
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_foreground_color (f: *mut c_void) -> *mut c_void
{
  unsafe { FRAME_FOREGROUND_COLOR! (f.cast::<frame> ()) }
}

/// C: `NSColor *background_color;` (nsterm.h 942; the frozen `#else` half is
/// C: `void *background_color;` 952) and `#define FRAME_BACKGROUND_COLOR(f)
/// C: ((f)->output_data.ns->background_color)` (nsterm.h 1038) - C reads it at nsterm.m 2362.
/// Swift `nsfns.swift` 290 (`-> NSColor?`); the write's entry is below.
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_background_color (f: *mut c_void) -> *mut c_void
{
  unsafe { FRAME_BACKGROUND_COLOR! (f.cast::<frame> ()) }
}

/// C: `int icon_top;` (nsterm.h 990) - C writes it at nsfns.m 1036 (`= -1;`) and 1048
/// C: (`= XFIXNUM (icon_y);`), and the reads the Swift unit makes are of the same member.
/// Swift `nsfns.swift` 294 (`-> Int32`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_icon_top (f: *mut c_void) -> c_int
{
  unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).icon_top
  }
}

/// C: `int icon_left;` (nsterm.h 991) - C writes it at nsfns.m 1037 (`= -1;`) and 1049
/// C: (`= XFIXNUM (icon_x);`).
/// Swift `nsfns.swift` 297 (`-> Int32`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_icon_left (f: *mut c_void) -> c_int
{
  unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).icon_left
  }
}

/// C: `Window window_desc, parent_desc;` (nsterm.h 980) and `#define FRAME_NS_WINDOW(f)
/// C: ((f)->output_data.ns->window_desc)` (nsterm.h 1034) - C writes it at nsfns.m 1285
/// C: (`f->output_data.ns->window_desc = desc_ctr++;`) and reads it at nsfns.m 654.  The
/// C: member is C's `Window` (`typedef int Window`, nsgui.h 67) while the lane's declaration is
/// C: `UInt`; the ABI is the declaration (`docs/ports/abi_shim.md`), so the read widens.
/// Swift `nsfns.swift` 300 (`-> UInt`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_window_desc (f: *mut c_void) -> c_ulong
{
  unsafe { FRAME_NS_WINDOW! (f.cast::<frame> ()) as c_ulong }
}

/// C: `Window window_desc, parent_desc;` (nsterm.h 980) - C writes `parent_desc` at nsfns.m
/// C: 1288 (`= XFIXNAT (parent);`), 1293 and 3076 (`= FRAME_DISPLAY_INFO (f)->root_window;`).
/// C: The member is C's `Window` (`int`) and the lane's declaration is `UInt`, so the read
/// C: widens (the `ns_output_window_desc` note).
/// Swift `nsfns.swift` 303 (`-> UInt`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_parent_desc (f: *mut c_void) -> c_ulong
{
  unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).parent_desc as c_ulong
  }
}

/// C: `char explicit_parent;` (nsterm.h 981) - C writes it at nsfns.m 1289 (`= 1;`), 1294
/// C: (`= 0;`) and tests it at 1550 (`if (! f->output_data.ns->explicit_parent)`), i.e. C uses
/// C: the byte as a truth value.  The lane's declaration is `Bool`, so the read is `!= 0`.
/// Swift `nsfns.swift` 306 (`-> Bool`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_explicit_parent (f: *mut c_void) -> bool
{
  unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).explicit_parent != 0
  }
}

/// C: `Emacs_Cursor text_cursor;` (nsterm.h 960; `Emacs_Cursor` is `void *` in the frozen
/// C: half, nsgui.rs 57) - C reads it at nsfns.m 1498
/// C: (`f->output_data.ns->current_pointer = f->output_data.ns->text_cursor;`) and writes it at
/// C: 1478.
/// Swift `nsfns.swift` 310 (`-> NSObject?`); the write's entry is `ns_output_set_text_cursor`.
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_text_cursor (f: *mut c_void) -> *mut c_void
{
  unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).text_cursor
  }
}

/// C: `int in_animation;` (nsterm.h 1010) - C writes it at nsfns.m 1500 (`= NO;`) and
/// C: nsmenu.m 1109/1111 (`= 1;` / `= 0;`); the reads are the Swift side's.
/// Swift `nsfns.swift` 314 and `nsmenu.swift` 326 (`-> Int32`) - one entry, two units.
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_in_animation (f: *mut c_void) -> c_int
{
  unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).in_animation
  }
}

/// C: `struct ns_display_info *display_info;` (nsterm.h 1004) and `#define FRAME_DISPLAY_INFO(f)
/// C: ((f)->output_data.ns->display_info)` (nsterm.h 1032) - C reads it at nsfns.m 109, 129 and
/// C: writes it at 1280 (`FRAME_DISPLAY_INFO (f) = dpyinfo;`), whose entry landed in batch 5
/// C: (`frame_set_display_info`).  The member's own type is the mirror's (`include/nsterm.rs`),
/// C: so the read returns it rather than degrading to `*mut c_void`.
/// Swift `nsfns.swift` 318 (`-> UnsafeMutablePointer<ns_display_info>`, the unit's own Swift
/// prefix declaration); the ABI is one pointer.
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_display_info (f: *mut c_void) -> *mut ns_display_info
{
  unsafe { FRAME_DISPLAY_INFO! (f.cast::<frame> ()) }
}

/// C: `int zooming;` (nsterm.h 1007) - C tests it at nsterm.m 1906 and 1948 (`if
/// C: (f->output_data.ns->zooming)`) and sets it at 1907/1949 (`= 0;`) and 8662 (`= 1;`), i.e.
/// C: the int is a truth value.  The lane's declaration is `Bool`, so the read is `!= 0`.
/// Swift `nsterm.swift` 2011 (`-> Bool`); the write's entry is below.
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_zooming (f: *mut c_void) -> bool
{
  unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).zooming != 0
  }
}

/// C: `EmacsView *view;` (nsterm.h 938; the frozen half is `void *view;` 948) - C's write is
/// C: `FRAME_NS_VIEW (f) = self;` (nsterm.m 8587, in `[EmacsView initFrameFromEmacs:]`).
/// Swift `nsfns.swift` 276 (`_ v: NSView?`); the read's entry is `ns_output_view`.
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_set_view (f: *mut c_void, v: *mut c_void)
{
  unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).view = v;
  }
}

/// C: `id miniimage;` (nsterm.h 939; the frozen half is `void *miniimage;` 949) - C writes it
/// C: at nsfns.m 921 (`f->output_data.ns->miniimage = image;`) and 962.
/// Swift `nsfns.swift` 280 (`_ v: AnyObject?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_set_miniimage (f: *mut c_void, v: *mut c_void)
{
  unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).miniimage = v;
  }
}

/// C: `NSColor *cursor_color;` (nsterm.h 940; the frozen half is `void *cursor_color;` 950) -
/// C: C's write is the macro's (`FRAME_CURSOR_COLOR (f) = [col retain];`, nsfns.m 358, after the
/// C: release at 357).
/// Swift `nsfns.swift` 284 (`_ v: NSColor?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_set_cursor_color (f: *mut c_void, v: *mut c_void)
{
  unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).cursor_color = v;
  }
}

/// C: `NSColor *foreground_color;` (nsterm.h 941; the frozen half is `void *foreground_color;`
/// C: 951) - C's write is `f->output_data.ns->foreground_color = col;` (nsfns.m 277, after the
/// C: release at 276).
/// Swift `nsfns.swift` 288 (`_ v: NSColor?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_set_foreground_color (f: *mut c_void, v: *mut c_void)
{
  unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).foreground_color = v;
  }
}

/// C: `NSColor *background_color;` (nsterm.h 942; the frozen half is `void *background_color;`
/// C: 952) - C's write is `f->output_data.ns->background_color = col;` (nsfns.m 310, after the
/// C: release at 309).
/// Swift `nsfns.swift` 292 (`_ v: NSColor?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_set_background_color (f: *mut c_void, v: *mut c_void)
{
  unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).background_color = v;
  }
}

/// C: `int icon_top;` (nsterm.h 990) - C writes it at nsfns.m 1036 (`= -1;`) and 1048
/// C: (`= XFIXNUM (icon_y);`).
/// Swift `nsfns.swift` 295 (`_ v: Int32`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_set_icon_top (f: *mut c_void, v: c_int)
{
  unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).icon_top = v;
  }
}

/// C: `int icon_left;` (nsterm.h 991) - C writes it at nsfns.m 1037 (`= -1;`) and 1049
/// C: (`= XFIXNUM (icon_x);`).
/// Swift `nsfns.swift` 298 (`_ v: Int32`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_set_icon_left (f: *mut c_void, v: c_int)
{
  unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).icon_left = v;
  }
}

/// C: `Window window_desc, parent_desc;` (nsterm.h 980) - C's write is
/// C: `f->output_data.ns->window_desc = desc_ctr++;` (nsfns.m 1285), where `desc_ctr` is the
/// C: file's `static int`.  The lane's declaration passes `UInt` (`UInt(desc_ctr)` at
/// C: nsfns.swift 2269), so the parameter is `c_ulong` and the store narrows to C's `int`.
/// Swift `nsfns.swift` 301 (`_ v: UInt`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_set_window_desc (f: *mut c_void, v: c_ulong)
{
  unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).window_desc = v as c_int;
  }
}

/// C: `Window window_desc, parent_desc;` (nsterm.h 980) - C writes `parent_desc` at nsfns.m
/// C: 1288 (`= XFIXNAT (parent);`), 1293 and 3076 (`= FRAME_DISPLAY_INFO (f)->root_window;`),
/// C: the lane's declaration passes `UInt`, so the parameter is `c_ulong` and the store narrows
/// C: to C's `int` (the `ns_output_set_window_desc` note).
/// Swift `nsfns.swift` 304 (`_ v: UInt`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_set_parent_desc (f: *mut c_void, v: c_ulong)
{
  unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).parent_desc = v as c_int;
  }
}

/// C: `char explicit_parent;` (nsterm.h 981) - C writes 1 and 0 (`= 1;` nsfns.m 1289, `= 0;`
/// C: 1294) and tests the byte as a truth value (1550).  The lane's declaration is `Bool`, so
/// C: the store writes C's two byte values back (`v as c_char`).
/// Swift `nsfns.swift` 308 (`_ v: Bool`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_set_explicit_parent (f: *mut c_void, v: bool)
{
  unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).explicit_parent = v as c_char;
  }
}

/// C: `Emacs_Cursor current_pointer;` (nsterm.h 977) plus `#define FRAME_POINTER_TYPE(f)
/// C: ((f)->output_data.ns->current_pointer)` (nsterm.h 1047) - C writes it at nsfns.m 1498
/// C: (`= f->output_data.ns->text_cursor;`) and nsterm.m 2634 (`FRAME_POINTER_TYPE (f) =
/// C: cursor;`).
/// Swift `nsfns.swift` 312 (`_ v: NSObject?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_set_current_pointer (f: *mut c_void, v: *mut c_void)
{
  unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).current_pointer = v;
  }
}

/// C: `Emacs_Cursor text_cursor;` (nsterm.h 960; `Emacs_Cursor` is `void *` in the frozen half,
/// C: nsgui.rs 57) - C writes it at nsfns.m 1478, in the 15-cursor block at 1478-1492
/// C: (`[EmacsView initFrameFromEmacs:]`: `f->output_data.ns->text_cursor = [NSCursor …];`).
/// Swift `nsfns.swift` 651 (`_ v: NSObject?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_set_text_cursor (f: *mut c_void, v: *mut c_void)
{
  unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).text_cursor = v;
  }
}

/// C: `Emacs_Cursor nontext_cursor;` (nsterm.h 961; `Emacs_Cursor` is `void *` in the frozen half,
/// C: nsgui.rs 57) - C writes it at nsfns.m 1479, in the 15-cursor block at 1478-1492
/// C: (`[EmacsView initFrameFromEmacs:]`: `f->output_data.ns->nontext_cursor = [NSCursor …];`).
/// Swift `nsfns.swift` 653 (`_ v: NSObject?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_set_nontext_cursor (f: *mut c_void, v: *mut c_void)
{
  unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).nontext_cursor = v;
  }
}

/// C: `Emacs_Cursor modeline_cursor;` (nsterm.h 962; `Emacs_Cursor` is `void *` in the frozen half,
/// C: nsgui.rs 57) - C writes it at nsfns.m 1480, in the 15-cursor block at 1478-1492
/// C: (`[EmacsView initFrameFromEmacs:]`: `f->output_data.ns->modeline_cursor = [NSCursor …];`).
/// Swift `nsfns.swift` 655 (`_ v: NSObject?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_set_modeline_cursor (f: *mut c_void, v: *mut c_void)
{
  unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).modeline_cursor = v;
  }
}

/// C: `Emacs_Cursor hand_cursor;` (nsterm.h 963; `Emacs_Cursor` is `void *` in the frozen half,
/// C: nsgui.rs 57) - C writes it at nsfns.m 1481, in the 15-cursor block at 1478-1492
/// C: (`[EmacsView initFrameFromEmacs:]`: `f->output_data.ns->hand_cursor = [NSCursor …];`).
/// Swift `nsfns.swift` 657 (`_ v: NSObject?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_set_hand_cursor (f: *mut c_void, v: *mut c_void)
{
  unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).hand_cursor = v;
  }
}

/// C: `Emacs_Cursor hourglass_cursor;` (nsterm.h 964; `Emacs_Cursor` is `void *` in the frozen half,
/// C: nsgui.rs 57) - C writes it at nsfns.m 1482, in the 15-cursor block at 1478-1492
/// C: (`[EmacsView initFrameFromEmacs:]`: `f->output_data.ns->hourglass_cursor = [NSCursor …];`).
/// Swift `nsfns.swift` 659 (`_ v: NSObject?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_set_hourglass_cursor (f: *mut c_void, v: *mut c_void)
{
  unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).hourglass_cursor = v;
  }
}

/// C: `Emacs_Cursor horizontal_drag_cursor;` (nsterm.h 965; `Emacs_Cursor` is `void *` in the frozen half,
/// C: nsgui.rs 57) - C writes it at nsfns.m 1483, in the 15-cursor block at 1478-1492
/// C: (`[EmacsView initFrameFromEmacs:]`: `f->output_data.ns->horizontal_drag_cursor = [NSCursor …];`).
/// Swift `nsfns.swift` 661 (`_ v: NSObject?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_set_horizontal_drag_cursor (f: *mut c_void, v: *mut c_void)
{
  unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).horizontal_drag_cursor = v;
  }
}

/// C: `Emacs_Cursor vertical_drag_cursor;` (nsterm.h 966; `Emacs_Cursor` is `void *` in the frozen half,
/// C: nsgui.rs 57) - C writes it at nsfns.m 1484, in the 15-cursor block at 1478-1492
/// C: (`[EmacsView initFrameFromEmacs:]`: `f->output_data.ns->vertical_drag_cursor = [NSCursor …];`).
/// Swift `nsfns.swift` 663 (`_ v: NSObject?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_set_vertical_drag_cursor (f: *mut c_void, v: *mut c_void)
{
  unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).vertical_drag_cursor = v;
  }
}

/// C: `Emacs_Cursor left_edge_cursor;` (nsterm.h 967; `Emacs_Cursor` is `void *` in the frozen half,
/// C: nsgui.rs 57) - C writes it at nsfns.m 1485, in the 15-cursor block at 1478-1492
/// C: (`[EmacsView initFrameFromEmacs:]`: `f->output_data.ns->left_edge_cursor = [NSCursor …];`).
/// Swift `nsfns.swift` 665 (`_ v: NSObject?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_set_left_edge_cursor (f: *mut c_void, v: *mut c_void)
{
  unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).left_edge_cursor = v;
  }
}

/// C: `Emacs_Cursor top_left_corner_cursor;` (nsterm.h 968; `Emacs_Cursor` is `void *` in the frozen half,
/// C: nsgui.rs 57) - C writes it at nsfns.m 1486, in the 15-cursor block at 1478-1492
/// C: (`[EmacsView initFrameFromEmacs:]`: `f->output_data.ns->top_left_corner_cursor = [NSCursor …];`).
/// Swift `nsfns.swift` 667 (`_ v: NSObject?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_set_top_left_corner_cursor (f: *mut c_void, v: *mut c_void)
{
  unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).top_left_corner_cursor = v;
  }
}

/// C: `Emacs_Cursor top_edge_cursor;` (nsterm.h 969; `Emacs_Cursor` is `void *` in the frozen half,
/// C: nsgui.rs 57) - C writes it at nsfns.m 1487, in the 15-cursor block at 1478-1492
/// C: (`[EmacsView initFrameFromEmacs:]`: `f->output_data.ns->top_edge_cursor = [NSCursor …];`).
/// Swift `nsfns.swift` 669 (`_ v: NSObject?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_set_top_edge_cursor (f: *mut c_void, v: *mut c_void)
{
  unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).top_edge_cursor = v;
  }
}

/// C: `Emacs_Cursor top_right_corner_cursor;` (nsterm.h 970; `Emacs_Cursor` is `void *` in the frozen half,
/// C: nsgui.rs 57) - C writes it at nsfns.m 1488, in the 15-cursor block at 1478-1492
/// C: (`[EmacsView initFrameFromEmacs:]`: `f->output_data.ns->top_right_corner_cursor = [NSCursor …];`).
/// Swift `nsfns.swift` 671 (`_ v: NSObject?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_set_top_right_corner_cursor (f: *mut c_void, v: *mut c_void)
{
  unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).top_right_corner_cursor = v;
  }
}

/// C: `Emacs_Cursor right_edge_cursor;` (nsterm.h 971; `Emacs_Cursor` is `void *` in the frozen half,
/// C: nsgui.rs 57) - C writes it at nsfns.m 1489, in the 15-cursor block at 1478-1492
/// C: (`[EmacsView initFrameFromEmacs:]`: `f->output_data.ns->right_edge_cursor = [NSCursor …];`).
/// Swift `nsfns.swift` 673 (`_ v: NSObject?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_set_right_edge_cursor (f: *mut c_void, v: *mut c_void)
{
  unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).right_edge_cursor = v;
  }
}

/// C: `Emacs_Cursor bottom_right_corner_cursor;` (nsterm.h 972; `Emacs_Cursor` is `void *` in the frozen half,
/// C: nsgui.rs 57) - C writes it at nsfns.m 1490, in the 15-cursor block at 1478-1492
/// C: (`[EmacsView initFrameFromEmacs:]`: `f->output_data.ns->bottom_right_corner_cursor = [NSCursor …];`).
/// Swift `nsfns.swift` 675 (`_ v: NSObject?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_set_bottom_right_corner_cursor (f: *mut c_void, v: *mut c_void)
{
  unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).bottom_right_corner_cursor = v;
  }
}

/// C: `Emacs_Cursor bottom_edge_cursor;` (nsterm.h 973; `Emacs_Cursor` is `void *` in the frozen half,
/// C: nsgui.rs 57) - C writes it at nsfns.m 1491, in the 15-cursor block at 1478-1492
/// C: (`[EmacsView initFrameFromEmacs:]`: `f->output_data.ns->bottom_edge_cursor = [NSCursor …];`).
/// Swift `nsfns.swift` 677 (`_ v: NSObject?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_set_bottom_edge_cursor (f: *mut c_void, v: *mut c_void)
{
  unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).bottom_edge_cursor = v;
  }
}

/// C: `Emacs_Cursor bottom_left_corner_cursor;` (nsterm.h 974; `Emacs_Cursor` is `void *` in the frozen half,
/// C: nsgui.rs 57) - C writes it at nsfns.m 1492, in the 15-cursor block at 1478-1492
/// C: (`[EmacsView initFrameFromEmacs:]`: `f->output_data.ns->bottom_left_corner_cursor = [NSCursor …];`).
/// Swift `nsfns.swift` 679 (`_ v: NSObject?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_set_bottom_left_corner_cursor (f: *mut c_void, v: *mut c_void)
{
  unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).bottom_left_corner_cursor = v;
  }
}

/// C: `int in_animation;` (nsterm.h 1010) - C writes it at nsfns.m 1500 (`= NO;`) and nsmenu.m
/// C: 1109/1111 (`= 1;` / `= 0;`).
/// Swift `nsfns.swift` 316 and `nsmenu.swift` 328 (`_ v: Int32`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_set_in_animation (f: *mut c_void, v: c_int)
{
  unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).in_animation = v;
  }
}

/// C: `int zooming;` (nsterm.h 1007) - C resets it at nsterm.m 1907 and 1949 (`= 0;`) and sets
/// C: it at 8662 (`emacsframe->output_data.ns->zooming = 1;`).  The lane's declaration is
/// C: `Bool`, so the store writes C's two int values back (`v as c_int`).
/// Swift `nsterm.swift` 2012 (`_ v: Bool`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ns_output_set_zooming (f: *mut c_void, v: c_int)
{
  unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).zooming = v as c_int;
  }
}

/* ---------------------------------------------------------------------------------------------
   Batch 6, group 2: `struct glyph_row`'s two members the nsterm lane reads directly.  The record
   is the core's (`include/dispextern.rs` 773-..., ungated text: `struct glyph_row` is not behind
   a platform switch in C either), so each member access is a C-ABI entry - the rule status
   3.257 item 5 (3) states for this pair ("每次访问就是一条 C-ABI 条目").  Unlike group 1 these
   two carry no gate: the record is live in the frozen configuration.
   --------------------------------------------------------------------------------------------- */

/// C: `int x, y;` (dispextern.h 950, inside `struct glyph_row` 918-...) - C reads the y member as
/// `row->y` (nsterm.m 818, in `static NSRect ns_row_rect (struct window *w, struct glyph_row
/// *row, enum glyph_row_area area)` 807-824).
/// Swift `nsterm.swift` 834 (`(UnsafeMutableRawPointer?) -> Int32`), used at `nsterm.swift` 980.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glyph_row_y (row: *mut c_void) -> c_int
{
  unsafe { (*row.cast::<glyph_row> ()).y }
}

/// C: `int visible_height;` (dispextern.h 969, inside `struct glyph_row` 918-...) - C reads it as
/// `row->visible_height` (nsterm.m 821, the same `ns_row_rect`).
/// Swift `nsterm.swift` 835-836 (`(UnsafeMutableRawPointer?) -> Int32`), used at
/// `nsterm.swift` 983.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glyph_row_visible_height (row: *mut c_void) -> c_int
{
  unsafe { (*row.cast::<glyph_row> ()).visible_height }
}

/* ---------------------------------------------------------------------------------------------
   Batch 6, group 3: `thread.h`'s `waiting_for_input` flag, written through the mirror's own
   macro.  `struct thread_state` (thread.h 66-...) is the core's record and is reachable here
   through `include/lisp.rs` 1442's `include!("thread.rs")`, so this one carries no gate.
   --------------------------------------------------------------------------------------------- */

/// C: `#define waiting_for_input (current_thread->m_waiting_for_input)` (thread.h 185, over
/// `bool m_waiting_for_input;` 184) - C's write is `waiting_for_input = 0;` (nsmenu.m 202, right
/// after the read into `owfi` at 201).  The body is the mirror's own accessor
/// (`include/thread.rs`'s `macro_rules! waiting_for_input`, thread.h 185), i.e. the same
/// expression the core's readers use, so writer and reader cannot drift apart.
/// Swift `nsmenu.swift` 407 (`func waiting_for_input_set(_ v: Bool)`); the read's entry
/// (`waiting_for_input`, `nsmenu.swift` 406) is still owed and is *not* part of this batch.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_waiting_for_input_flag (v: bool)
{
  unsafe { waiting_for_input! () = v; }
}

/* ---------------------------------------------------------------------------------------------
   Batch 6, group 4: `struct face`'s `background`, behind the mirror's own gate.  C has `struct
   face` unconditionally (dispextern.h 1787-1937), but the port's copy sits behind
   `#[cfg (feature = "dispextern_face_records")]` (`include/dispextern.rs` 1941-1943) because the
   units that keep typed copies of it (`xfaces.rs`, `term.rs`, `alloc.rs`) are not yet closed
   under the flip (`include/frame.rs` 698-718), so the frozen configuration has no `face` name
   for this body to reach and the export can only exist in that arm - the
   `frame_set_double_buffered` shape again (`## Decisions`).
   --------------------------------------------------------------------------------------------- */

/// C: `unsigned long background;` (dispextern.h 1819, inside `struct face` 1787-1937) - C's
/// write is `face->background = [[col colorWithAlphaComponent: alpha] unsignedLong];`
/// (nsfns.m 328-329, whose right-hand side reads `NS_FACE_BACKGROUND (face)` at 327).
/// Swift `nsfns.swift` 475-476 (`func face_set_background(_ face: UnsafeMutableRawPointer?,
/// _ v: UInt)`), used at `nsfns.swift` 1404.
#[cfg (feature = "dispextern_face_records")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn face_set_background (face: *mut c_void, v: c_ulong)
{
  unsafe { (*face.cast::<face> ()).background = v; }
}

/* ---------------------------------------------------------------------------------------------
   Batch 7, group 1: the two entries of the nsterm lane's tenth slice (status 3.259) that the
   frozen mirror can host.  Both names are new to this file and free at the crate root, so they
   are ordinary exports; the rest of that slice's list is held back for the reasons `## Decisions`
   gives (missing hosts) or is not the core's (the `Q*` family, `setAppearance`).
   --------------------------------------------------------------------------------------------- */

/// C: `#define FRAME_NO_ACCEPT_FOCUS(f) ((f)->no_accept_focus)` (frame.h 1249) - the `bool_bf`
/// member at frame.h 411, written at src/nsterm.m 2074 (`FRAME_NO_ACCEPT_FOCUS (f) = !NILP
/// (new_value);`).  The write uses the mirror's own shift constant
/// (`FRAME_NO_ACCEPT_FOCUS_SHIFT` 26, `include/frame.rs` 170) - the same bit `femacs/src/frame.rs`
/// 244-247's reader/writer macro uses - so writer and reader cannot drift apart (the batch-5
/// `frame_set_no_split` shape).
/// Swift `nsterm.swift` 2162-2163 (`_ v: Bool`).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_set_no_accept_focus (f: *mut c_void, v: bool)
{
  unsafe {
    let bits = &mut (*f.cast::<frame> ()).bits1;
    *bits = (*bits & !(1 << FRAME_NO_ACCEPT_FOCUS_SHIFT)) | ((v as u32) << FRAME_NO_ACCEPT_FOCUS_SHIFT);
  }
}

/// C: `int menu_bar_items_used;` (frame.h 307) - C's writes are src/nsmenu.m 303
/// (`f->menu_bar_items_used = menu_items_used;`) and 359 (`= 0;`).  The reader landed in batch 1
/// (`frame_menu_bar_items_used`, above), and the member is the mirror's own (`include/frame.rs`
/// 69), so this closes the ruling's class-1 pair for the name.
/// Swift `nsmenu.swift` 316-317 (`(UnsafeMutableRawPointer?, Int32)`).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_set_menu_bar_items_used (f: *mut c_void, v: c_int)
{
  unsafe { (*f.cast::<frame> ()).menu_bar_items_used = v; }
}

/// C: `double alpha[2];` (frame.h 707-711, "alpha[0]: alpha transparency of the active frame,
/// alpha[1]: alpha transparency of inactive frames") - read at src/nsterm.m 2378/2380
/// (`alpha = f->alpha[0];` / `alpha = f->alpha[1];`) with C's own subscript, which is the
/// entry's second argument (`nsterm.swift` 2235-2236).
/// Swift `nsterm.swift` 2235-2236 (`(UnsafeMutableRawPointer?, Int32) -> Double`).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_alpha (f: *mut c_void, index: c_int) -> c_double
{
  unsafe { (*f.cast::<frame> ()).alpha[index as usize] }
}

/* ---------------------------------------------------------------------------------------------
   Batch 7, group 2: the entries whose *names* the mirrors already spell in Rust.  C's
   enumerators (`enum z_group`, frame.h 61-67) and header accessors (`FLOATP` lisp.h 448,
   `XFLOAT_DATA` lisp.h 3186) have no symbol in C either, and the port already carries each value
   under its own name: `include/frame.rs` 752-756's `pub const z_group_none` ... and
   `include/lisp.rs` 916/1695's `pub fn FLOATP`/`XFLOAT_DATA`.  A crate-root definition of the
   same name beside them is `E0428` (measured, `## Evidence`), so the C-ABI spelling lives in a
   module of its own - the `include/thread.rs` 281-288 `thread_objs` and `include/frame.rs` 720's
   `frame_face_accessors` shape - and each body calls the mirror's own item through `super::`, so
   the value has one source of truth.  `use super::*` is what puts the crate root's types
   (`Lisp_Object`, `c_int`) in scope here; the `super::` prefixes are deliberate, because a glob
   import is shadowed by the module's own item of the same name.
   --------------------------------------------------------------------------------------------- */

#[allow(unused_imports)]
pub mod c_abi_of_mirror_named_items
{
  use super::*;

  /// C: `enum z_group { z_group_none, z_group_above, z_group_below, z_group_above_suspended, };`
  /// (frame.h 61-67, inside `#ifdef HAVE_WINDOW_SYSTEM`; the port carries the four members
  /// because C's live `#else` arm of `FRAME_Z_GROUP` names one - `include/frame.rs` 748-756).
  /// The Swift import is `nsterm.swift` 2169 (`func z_group_none() -> Int32`), the lane's
  /// function-shaped spelling of a C enumerator (status 3.255 lists enumerators as A class).
  #[unsafe(no_mangle)]
  pub unsafe extern "C" fn z_group_none () -> c_int { super::z_group_none as c_int }

  /// C: `z_group_above,` (frame.h 64), value 1 (`include/frame.rs` 754).
  #[unsafe(no_mangle)]
  pub unsafe extern "C" fn z_group_above () -> c_int { super::z_group_above as c_int }

  /// C: `z_group_below,` (frame.h 65), value 2 (`include/frame.rs` 755).
  #[unsafe(no_mangle)]
  pub unsafe extern "C" fn z_group_below () -> c_int { super::z_group_below as c_int }

  /// C: `z_group_above_suspended,` (frame.h 66), value 3 (`include/frame.rs` 756).
  #[unsafe(no_mangle)]
  pub unsafe extern "C" fn z_group_above_suspended () -> c_int
  { super::z_group_above_suspended as c_int }

  /// C: `#define FLOATP(x) lisp_h_FLOATP (x)` (src/lisp.h 448, over the INLINE at 445-447) - the
  /// header-only predicate the lane imports as a function (`nsterm.swift` 2228).  The body is the
  /// mirror's own `FLOATP` (`include/lisp.rs` 916, which is `lisp_h_FLOATP`), so the tag test has
  /// one spelling in this tree.
  #[unsafe(no_mangle)]
  pub unsafe extern "C" fn FLOATP (x: Lisp_Object) -> bool { super::FLOATP (x) }

  /// C: `INLINE double XFLOAT_DATA (Lisp_Object f)` (src/lisp.h 3186-3189,
  /// `return XFLOAT (f)->u.data;`) - the lane's import is `nsterm.swift` 2229.  The body is the
  /// mirror's own `XFLOAT_DATA` (`include/lisp.rs` 1695).
  #[unsafe(no_mangle)]
  pub unsafe extern "C" fn XFLOAT_DATA (f: Lisp_Object) -> f64 { super::XFLOAT_DATA (f) }
}
