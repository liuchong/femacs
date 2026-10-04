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
