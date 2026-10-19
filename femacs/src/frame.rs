// femacs: port of src/frame.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// struct, struct field, macro, file-scope object, local), same control flow, same error
// model.  The 66 `DEFUN`s use `DEFUN!` (PORTING.md 5.1); the non-`static` definitions C
// exports carry `#[unsafe(no_mangle)]` (PORTING.md 0.2) and the C `static` helpers stay
// plain `unsafe fn`.
//
// `src/frame.c` *is* in `docs/baseline/config/cond-eval-verified.txt`
// (`python3 tools/check-cond-eval.py src/frame.c` = `105 conditions, 0 unknown,
// 0 disagreements [VERIFIED]`), so per P-08 the arms the frozen configuration does not
// compile are *recorded in `docs/ports/frame.md`, not transcribed*.  The dead arms:
//
//   * C 31-33, 44-46, 81-83, 115-121, 128-130, 136-139, 1187-1213, 1452-1455, 1593-1596,
//     3764-3767, 4156-4159, 4189-4207, 4261-4269, 4352-4357, 4371-4376, 4393-4396,
//     4422-4425, 4438-4442, 4680-4682, 4880-4885, 7055-7112, 7120-7187, 7251-7252,
//     7285-7286, 7316-7321, 7396-7425, 7455-7462, 7551-7552 — the `HAVE_WINDOW_SYSTEM`
//     (`#include TERM_HEADER`/`fontset.h`, `gui_report_frame_params`, the display-available
//     probe, `frame_redisplay_p`'s `HAVE_X_WINDOWS` arm and `make_frame`'s window-system
//     initialization).  The one large arm of this family that is *carried* rather than
//     recorded is the GUI frame-geometry implementation at C 4889-6970 (P-86 ①: both arms,
//     one gate per item; see `docs/ports/frame.md`'s C-line-range table).
//   * C 48-51, 52-54, 1030-1043, 1568-1582, 1762-1789, 3436-3442, 3481-3487 — the `MSDOS`
//     and `USE_X_TOOLKIT` arms (both undef in the frozen configuration).
//   * C 482-491, 206-208, 943-945, 1055-1062, 1064-1071, 1218-1226, 1477/1722/1755-1758
//     (the `HAVE_ANDROID` arms; `HAVE_ANDROID` is undef), 1584-1592's `HAVE_X_WINDOWS`
//     neighbours, 1953-1979, 2167-2177, 2535-2546, 2558-2565, 2607-2629, 2716-2719,
//     2812-2823, 2852-2877, 2885-2888, 2910-2912, 2928-2933, 2950-2957, 2963-2966,
//     2971-2983.
//   * C 3371-3405, 3431-3434, 3446-3451, 3477-3479, 3491-3496 — the `HAVE_GPM` arms and the
//     window-system halves of `Fmouse_position`/`Fset_mouse_position` (`HAVE_GPM` undef).
//
// **Use-site declarations.**  Several headers this file includes have no mirror; each entry
// below carries C's own prototype in a `/// C:` line (PORTING.md 0.1-2/P-72 rule 3, the
// editfns.rs/insdel.rs shape).  The mirror gaps this unit reports and does not touch are
// listed in `docs/ports/frame.md`.
//
// **L-value macros.**  `frame.h`'s accessors are macros in C; `include/frame.rs` mirrors the
// read-only ones as functions and packs the `bool_bf` flag run into `bits1`/`bits2`
// (PORTING.md 1.3).  The macros this file's C *writes* to, and the ones the mirror does not
// carry, are defined here at the use site under C's own name (`SET_FRAME_GARBAGED`,
// `FRAME_COLS`, `wants_modeline`, …), so the port's body keeps C's token and the write goes
// through the mirror's mask; `NAME! (…)` is the macro call in Rust, whose `!` the token
// check ignores.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");                 // C 20: `#include <config.h>`
include!("../lib/include/stdio.rs");           // C 22: `#include <stdio.h>`
include!("../lib/include/stdlib.rs");          // C 23: `#include <stdlib.h>`
include!("../lib/include/errno.rs");           // C 24: `#include <errno.h>`
include!("../lib/include/limits.in.rs");       // C 25: `#include <limits.h>`
include!("../lib/include/stdint.in.rs");       // lisp.h's width family
include!("../lib/include/string.rs");          // lisp.h 30: `#include <string.h>` (`strcmp`)
include!("../lib/include/c-ctype.rs");         // C 27: `#include <c-ctype.h>`
include!("../lib/include/stdcountof.rs");      // lisp.h 28: `#include <stdcountof.h>` (countof)
include!("include/lisp.rs");                   // C 29: `#include "lisp.h"`
/* The mirrors the headers above pull in (the text of C's own include chain, in C's order
   where one applies; Rust item order is irrelevant but macros must precede their uses).  */
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

/* C 31-33 / 44-46 / 52-54: `TERM_HEADER`, `fontset.h` and `widget.h`.  C picks `TERM_HEADER` per
   configuration (configure.ac 7180; the frozen table spells it `undef`, so this arm is text
   here) and it is one of the six platform terminal headers.  The port has a mirror for each, so
   the arm keeps C's include shape: one gated `include!` per platform, behind the same feature
   the mirror's own items use (P-86 ⑧: the gate goes on the include graph too).  */
#[cfg (feature = "have_x_windows")]
include!("include/xterm.rs");                  // TERM_HEADER: `xterm.h`
#[cfg (feature = "have_ns")]
include!("include/nsterm.rs");                 // TERM_HEADER: `nsterm.h`
#[cfg (feature = "have_pgtk")]
include!("include/pgtkterm.rs");               // TERM_HEADER: `pgtkterm.h`
#[cfg (feature = "have_ntgui")]
include!("include/w32term.rs");                // TERM_HEADER: `w32term.h`
#[cfg (feature = "have_haiku")]
include!("include/haikuterm.rs");              // TERM_HEADER: `haikuterm.h`
#[cfg (feature = "have_android")]
include!("include/androidterm.rs");            // TERM_HEADER: `androidterm.h`
#[cfg (feature = "have_window_system")]
include!("include/fontset.rs");                // C 45: `#include "fontset.h"`
#[cfg (feature = "use_x_toolkit")]
include!("include/widget.rs");                 // C 53: `#include "widget.h"`

/* frame.h 1387-1391: `CHECK_FRAME`/`CHECK_LIVE_FRAME` are the mirror's (`include/frame.rs`
   979/983, which this unit includes at C 42); the unit's own copies were body-identical
   redundancy (P-26).  */

/* frame.h 1536-1552: `SET_FRAME_VISIBLE`.  The mirror's copy (`include/frame.rs` 1020) carries
   C's full writer, but its body cannot expand in this unit: it calls `redisplay_other_windows`
   (no such name here) and writes `do_switch_frame (…, Qnil)` where the port's declaration takes
   `Qnil ()`.  The unit therefore keeps C's effect in its own spelling (P-26's "unit copy wins"
   case, reported to the mirror's owner); the flag write is unit 1 (frame.rs 154).  */
/* frame.h 1173-1174: `SET_FRAME_GARBAGED` is a three-term comma expression in C; the port keeps
   the same three effects in the same order (`frame_garbaged = true`, `fset_redisplay (f)`, the
   flag write).  The mirror's copy (`include/frame.rs` 947) cannot expand here: it passes
   `*mut frame` where this tree declares `fset_redisplay (*mut c_void)` (E0308; the cast below
   is the working spelling) - reported to the mirror's owner, not changed there.  */
/* frame.h 1155-1226: the geometry macros.  `FRAME_MENU_BAR_LINES`/`FRAME_TAB_BAR_LINES`'s
   readers are in the mirror, but this file both reads and writes them, so the whole family
   is defined here in C's terms and called as `NAME! (…)` throughout (the token check
   ignores the `!`).  */
macro_rules! FRAME_MENU_BAR_LINES { ($f:expr) => { (*$f).menu_bar_lines }; }
macro_rules! FRAME_MENU_BAR_HEIGHT { ($f:expr) => { (*$f).menu_bar_height }; }
macro_rules! FRAME_TAB_BAR_LINES { ($f:expr) => { (*$f).tab_bar_lines }; }
macro_rules! FRAME_TAB_BAR_HEIGHT { ($f:expr) => { (*$f).tab_bar_height }; }
macro_rules! FRAME_TEXT_WIDTH { ($f:expr) => { (*$f).text_width }; }
macro_rules! FRAME_TEXT_HEIGHT { ($f:expr) => { (*$f).text_height }; }
macro_rules! FRAME_COLS { ($f:expr) => { (*$f).text_cols }; }
macro_rules! FRAME_LINES { ($f:expr) => { (*$f).text_lines }; }
macro_rules! FRAME_TOTAL_COLS { ($f:expr) => { (*$f).total_cols }; }
macro_rules! FRAME_TOTAL_LINES { ($f:expr) => { (*$f).total_lines }; }
macro_rules! FRAME_PIXEL_WIDTH { ($f:expr) => { (*$f).pixel_width }; }
macro_rules! FRAME_PIXEL_HEIGHT { ($f:expr) => { (*$f).pixel_height }; }
macro_rules! FRAME_FOREGROUND_PIXEL { ($f:expr) => { (*$f).foreground_pixel }; }
macro_rules! FRAME_BACKGROUND_PIXEL { ($f:expr) => { (*$f).background_pixel }; }

/* frame.h 1790-1800: the `FRAME_TEXT_TO_PIXEL_WIDTH`/`FRAME_TEXT_TO_PIXEL_HEIGHT`
   conversions `adjust_frame_size` (frame.h 333) is built from.  */
macro_rules! FRAME_TEXT_TO_PIXEL_WIDTH {
    ($f:expr, $width:expr) => {
        ($width) + FRAME_SCROLL_BAR_AREA_WIDTH ($f) + FRAME_TOTAL_FRINGE_WIDTH ($f)
        + 2 * FRAME_INTERNAL_BORDER_WIDTH ($f)
    };
}
macro_rules! FRAME_TEXT_TO_PIXEL_HEIGHT {
    ($f:expr, $height:expr) => {
        ($height) + FRAME_MARGIN_HEIGHT ($f) + FRAME_SCROLL_BAR_AREA_HEIGHT ($f)
        + 2 * FRAME_INTERNAL_BORDER_WIDTH ($f)
    };
}
macro_rules! FRAME_PIXEL_TO_TEXT_WIDTH {
    ($f:expr, $width:expr) => {
        ($width) - FRAME_SCROLL_BAR_AREA_WIDTH ($f) - FRAME_TOTAL_FRINGE_WIDTH ($f)
        - 2 * FRAME_INTERNAL_BORDER_WIDTH ($f)
    };
}
macro_rules! FRAME_PIXEL_TO_TEXT_HEIGHT {
    ($f:expr, $height:expr) => {
        ($height) - FRAME_MARGIN_HEIGHT ($f) - FRAME_SCROLL_BAR_AREA_HEIGHT ($f)
        - 2 * FRAME_INTERNAL_BORDER_WIDTH ($f)
    };
}
macro_rules! FRAME_INNER_WIDTH {
    ($f:expr) => { FRAME_PIXEL_WIDTH! ($f) - 2 * FRAME_INTERNAL_BORDER_WIDTH ($f) };
}
macro_rules! FRAME_INNER_HEIGHT {
    ($f:expr) => {
        FRAME_PIXEL_HEIGHT! ($f) - FRAME_MARGIN_HEIGHT ($f)
        - 2 * FRAME_INTERNAL_BORDER_WIDTH ($f)
    };
}
macro_rules! FRAME_TEXT_COLS_TO_PIXEL_WIDTH {
    ($f:expr, $cols:expr) => {
        ($cols) * FRAME_COLUMN_WIDTH ($f) + FRAME_SCROLL_BAR_AREA_WIDTH ($f)
        + FRAME_TOTAL_FRINGE_WIDTH ($f) + 2 * FRAME_INTERNAL_BORDER_WIDTH ($f)
    };
}
macro_rules! FRAME_TEXT_LINES_TO_PIXEL_HEIGHT {
    ($f:expr, $lines:expr) => {
        ($lines) * FRAME_LINE_HEIGHT ($f) + FRAME_TOP_MARGIN_HEIGHT ($f)
        + FRAME_SCROLL_BAR_AREA_HEIGHT ($f) + 2 * FRAME_INTERNAL_BORDER_WIDTH ($f)
    };
}
macro_rules! FRAME_PIXEL_WIDTH_TO_TEXT_COLS {
    ($f:expr, $width:expr) => {
        (($width) - FRAME_TOTAL_FRINGE_WIDTH ($f) - FRAME_SCROLL_BAR_AREA_WIDTH ($f)
         - 2 * FRAME_INTERNAL_BORDER_WIDTH ($f)) / FRAME_COLUMN_WIDTH ($f)
    };
}
macro_rules! FRAME_PIXEL_HEIGHT_TO_TEXT_LINES {
    ($f:expr, $height:expr) => {
        (($height) - FRAME_MARGIN_HEIGHT ($f) - FRAME_SCROLL_BAR_AREA_HEIGHT ($f)
         - 2 * FRAME_INTERNAL_BORDER_WIDTH ($f)) / FRAME_LINE_HEIGHT ($f)
    };
}
macro_rules! FRAME_PIXEL_X_TO_COL {
    ($f:expr, $x:expr) => {
        (($x) - FRAME_INTERNAL_BORDER_WIDTH ($f)) / FRAME_COLUMN_WIDTH ($f)
    };
}

/* frame.h 300-341's `bool_bf` flag run, read and written through the mirror's
   `bits1`/`bits2` masks.  Each macro keeps C's own field name and both C shapes — a read
   and an assignment — so a body that touches the field keeps the token (PORTING.md 1.3;
   `docs/ports/frame.md` records why C's field names cannot appear as Rust fields).  */
macro_rules! glyphs_initialized_p {
    ($f:expr) => { (*$f).bits1 >> FRAME_GLYPHS_INITIALIZED_P_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_GLYPHS_INITIALIZED_P_SHIFT, $v) };
}
macro_rules! resized_p {
    ($f:expr) => { (*$f).bits1 >> FRAME_RESIZED_P_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_RESIZED_P_SHIFT, $v) };
}
macro_rules! already_hscrolled_p {
    ($f:expr) => { (*$f).bits1 >> FRAME_ALREADY_HSCROLLED_P_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_ALREADY_HSCROLLED_P_SHIFT, $v) };
}
macro_rules! redisplay {
    ($f:expr) => { (*$f).bits1 >> FRAME_REDISPLAY_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_REDISPLAY_SHIFT, $v) };
}
macro_rules! wants_modeline {
    ($f:expr) => { (*$f).bits1 >> FRAME_WANTS_MODELINE_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_WANTS_MODELINE_SHIFT, $v) };
}
macro_rules! auto_raise {
    ($f:expr) => { (*$f).bits1 >> FRAME_AUTO_RAISE_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_AUTO_RAISE_SHIFT, $v) };
}
macro_rules! auto_lower {
    ($f:expr) => { (*$f).bits1 >> FRAME_AUTO_LOWER_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_AUTO_LOWER_SHIFT, $v) };
}
macro_rules! no_split {
    ($f:expr) => { (*$f).bits1 >> FRAME_NO_SPLIT_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_NO_SPLIT_SHIFT, $v) };
}
macro_rules! explicit_name {
    ($f:expr) => { (*$f).bits1 >> FRAME_EXPLICIT_NAME_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_EXPLICIT_NAME_SHIFT, $v) };
}
macro_rules! window_change {
    ($f:expr) => { (*$f).bits1 >> FRAME_WINDOW_CHANGE_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_WINDOW_CHANGE_SHIFT, $v) };
}
macro_rules! window_state_change {
    ($f:expr) => { (*$f).bits1 >> FRAME_WINDOW_STATE_CHANGE_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_WINDOW_STATE_CHANGE_SHIFT, $v) };
}
macro_rules! frozen_window_starts {
    ($f:expr) => { (*$f).bits1 >> FRAME_FROZEN_WINDOW_STARTS_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_FROZEN_WINDOW_STARTS_SHIFT, $v) };
}
macro_rules! undecorated {
    ($f:expr) => { (*$f).bits1 >> FRAME_UNDECORATED_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_UNDECORATED_SHIFT, $v) };
}
macro_rules! no_accept_focus {
    ($f:expr) => { (*$f).bits1 >> FRAME_NO_ACCEPT_FOCUS_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_NO_ACCEPT_FOCUS_SHIFT, $v) };
}
macro_rules! can_set_window_size {
    ($f:expr) => { (*$f).bits1 >> FRAME_CAN_SET_WINDOW_SIZE_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_CAN_SET_WINDOW_SIZE_SHIFT, $v) };
}
macro_rules! after_make_frame {
    ($f:expr) => { (*$f).bits1 >> FRAME_AFTER_MAKE_FRAME_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_AFTER_MAKE_FRAME_SHIFT, $v) };
}
macro_rules! tab_bar_redisplayed {
    ($f:expr) => { (*$f).bits1 >> FRAME_TAB_BAR_REDISPLAYED_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_TAB_BAR_REDISPLAYED_SHIFT, $v) };
}
macro_rules! tab_bar_resized {
    ($f:expr) => { (*$f).bits1 >> FRAME_TAB_BAR_RESIZED_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_TAB_BAR_RESIZED_SHIFT, $v) };
}
macro_rules! tool_bar_redisplayed {
    ($f:expr) => { (*$f).bits2 >> FRAME_TOOL_BAR_REDISPLAYED_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag2! ($f, FRAME_TOOL_BAR_REDISPLAYED_SHIFT, $v) };
}
macro_rules! tool_bar_resized {
    ($f:expr) => { (*$f).bits2 >> FRAME_TOOL_BAR_RESIZED_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag2! ($f, FRAME_TOOL_BAR_RESIZED_SHIFT, $v) };
}
macro_rules! new_size_p {
    ($f:expr) => { (*$f).bits2 >> FRAME_NEW_SIZE_P_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag2! ($f, FRAME_NEW_SIZE_P_SHIFT, $v) };
}
macro_rules! was_invisible {
    ($f:expr) => { (*$f).bits2 >> FRAME_WAS_INVISIBLE_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag2! ($f, FRAME_WAS_INVISIBLE_SHIFT, $v) };
}
macro_rules! select_mini_window_flag {
    ($f:expr) => { (*$f).bits2 >> FRAME_SELECT_MINI_WINDOW_FLAG_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag2! ($f, FRAME_SELECT_MINI_WINDOW_FLAG_SHIFT, $v) };
}
macro_rules! borders_respect_alpha_background {
    ($f:expr) => { (*$f).bits2 >> FRAME_BORDERS_RESPECT_ALPHA_BACKGROUND_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag2! ($f, FRAME_BORDERS_RESPECT_ALPHA_BACKGROUND_SHIFT, $v) };
}
macro_rules! visible {
    ($f:expr) => { (*$f).bits1 >> FRAME_VISIBLE_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_VISIBLE_SHIFT, $v) };
}
macro_rules! iconified {
    ($f:expr) => { (*$f).bits1 >> FRAME_ICONIFIED_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_ICONIFIED_SHIFT, $v) };
}
macro_rules! garbaged {
    ($f:expr) => { (*$f).bits1 >> FRAME_GARBAGED_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_GARBAGED_SHIFT, $v) };
}
macro_rules! updated_p {
    ($f:expr) => { (*$f).bits1 >> FRAME_UPDATED_P_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_UPDATED_P_SHIFT, $v) };
}
macro_rules! fonts_changed {
    ($f:expr) => { (*$f).bits1 >> FRAME_FONTS_CHANGED_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_FONTS_CHANGED_SHIFT, $v) };
}
macro_rules! cursor_type_changed {
    ($f:expr) => { (*$f).bits1 >> FRAME_CURSOR_TYPE_CHANGED_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_CURSOR_TYPE_CHANGED_SHIFT, $v) };
}
macro_rules! mouse_moved {
    ($f:expr) => { (*$f).bits1 >> FRAME_MOUSE_MOVED_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_MOUSE_MOVED_SHIFT, $v) };
}
macro_rules! no_special_glyphs {
    ($f:expr) => { (*$f).bits1 >> FRAME_NO_SPECIAL_GLYPHS_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_NO_SPECIAL_GLYPHS_SHIFT, $v) };
}
macro_rules! pointer_invisible {
    ($f:expr) => { (*$f).bits1 >> FRAME_POINTER_INVISIBLE_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_POINTER_INVISIBLE_SHIFT, $v) };
}

/* C 1145's `ENUM_BF (output_method)` is the four-bit field in the middle of unit 1; the
   mirror's reader is `output_method (f)` and this macro is the writer C writes
   (`f->output_method = …`).  */
macro_rules! set_output_method {
    ($f:expr, $v:expr) => {
        (*$f).bits1 = ((*$f).bits1 & ! (FRAME_OUTPUT_METHOD_MASK << FRAME_OUTPUT_METHOD_SHIFT))
                      | ((($v as u32) & FRAME_OUTPUT_METHOD_MASK) << FRAME_OUTPUT_METHOD_SHIFT)
    };
}

/* window.h's `mini` bit (one of `struct window`'s nineteen packed flags, window.rs 141's
   `WINDOW_MINI_SHIFT`); the mirror's reader is `MINI_WINDOW_P` and this macro is the write
   C spells `w->mini = …` (PORTING.md 1.3).  */
macro_rules! mini {
    ($w:expr) => { MINI_WINDOW_P ($w) };
    ($w:expr, $v:expr) => { (*$w).bits = ((*$w).bits & ! (1u32 << WINDOW_MINI_SHIFT))
					 | ((($v as u32) & 1) << WINDOW_MINI_SHIFT) };
}

/* lisp.h 3966: `extern struct hash_table_test hashtest_eq;` — the mirror carries
   `hashtest_eql`/`hashtest_equal` but not this one (P-72 rule 3).  */
/* C 6666-6677's `atoi`/`atof` come from `<stdlib.h>`.  The mirror's `stdlib.rs` does not carry
   the pair, and two other units declare their own copies at their use sites (P-55), so the
   declaration lives here too rather than in the shared header.  */
unsafe extern "C" {
  pub fn atoi (s: *const c_char) -> c_int;
  pub fn atof (s: *const c_char) -> c_double;
}

/* `struct redisplay_interface` (C dispextern.h 3069-3196) - the use-site copy status
   3.220 prescribes; `FRAME_RIF` hands back the degraded `*mut c_void`, and this unit
   reads two of the record's members (measured: `frame_parm_handlers`,
   `default_font_parameter`).  The text is the mirror's, gate spent.  */
#[cfg (feature = "have_window_system")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct redisplay_interface
{
  pub frame_parm_handlers: *mut c_void,                  // offset 0
  pub produce_glyphs: Option<unsafe extern "C" fn (*mut c_void)>   /* C's `struct it *` untyped (P-72 3) */,                  // offset 8
  pub write_glyphs: Option<unsafe extern "C" fn (w: *mut c_void, row: *mut glyph_row, string: *mut glyph, area: c_int, len: c_int)>,                  // offset 16
  pub insert_glyphs: Option<unsafe extern "C" fn (w: *mut c_void, row: *mut glyph_row, start: *mut glyph, area: c_int, len: c_int)>,                  // offset 24
  pub clear_end_of_line: Option<unsafe extern "C" fn (w: *mut c_void, row: *mut glyph_row, area: c_int, x: c_int)>,                  // offset 32
  pub scroll_run_hook: Option<unsafe extern "C" fn (w: *mut c_void, run: *mut run)>,                  // offset 40
  pub after_update_window_line_hook: Option<unsafe extern "C" fn (w: *mut c_void, desired_row: *mut glyph_row)>,                  // offset 48
  pub update_window_begin_hook: Option<unsafe extern "C" fn (w: *mut c_void)>,                  // offset 56
  pub update_window_end_hook: Option<unsafe extern "C" fn (w: *mut c_void, cursor_on_p: bool, mouse_face_overwritten_p: bool)>,                  // offset 64
  pub flush_display: Option<unsafe extern "C" fn (w: *mut c_void)>,                  // offset 72
  pub clear_window_mouse_face: Option<unsafe extern "C" fn (w: *mut c_void)>,                  // offset 80
  pub get_glyph_overhangs: Option<unsafe extern "C" fn (glyph: *mut glyph, f: *mut c_void, left: *mut c_int, right: *mut c_int)>,                  // offset 88
  pub fix_overlapping_area: Option<unsafe extern "C" fn (w: *mut c_void, row: *mut glyph_row, area: c_int, overlaps: c_int)>,                  // offset 96

  /* C 3131-3195: the `#ifdef HAVE_WINDOW_SYSTEM` arm of the interface - the fifteen display
     hooks a window system installs.  Dead text on the frozen target (`HAVE_WINDOW_SYSTEM` is
     undef there), carried under that feature so that a window-system build gets C's full
     record; each is one pointer wide and none of them exists in the frozen arm, so the
     assertions above are unchanged.  They are what `spikes/mirror-struct-coverage.py --arm
     ns_cocoa` names when it reports this struct's window-system gap (status 3.283).  */
  #[cfg (feature = "have_window_system")]
  pub draw_fringe_bitmap: Option<unsafe extern "C" fn (*mut c_void, *mut glyph_row, *mut c_void   /* C's `struct draw_fringe_bitmap_params *` untyped (P-72 3) */)>,   // C 3134
  #[cfg (feature = "have_window_system")]
  pub define_fringe_bitmap: Option<unsafe extern "C" fn (which: c_int, bits: *mut c_ushort, h: c_int, wd: c_int)>,      // C 3138
  #[cfg (feature = "have_window_system")]
  pub destroy_fringe_bitmap: Option<unsafe extern "C" fn (which: c_int)>,                                               // C 3140
  #[cfg (feature = "have_window_system")]
  pub compute_glyph_string_overhangs: Option<unsafe extern "C" fn (s: *mut glyph_string)>,                              // C 3144
  #[cfg (feature = "have_window_system")]
  pub draw_glyph_string: Option<unsafe extern "C" fn (s: *mut glyph_string)>,                                           // C 3147
  #[cfg (feature = "have_window_system")]
  pub define_frame_cursor: Option<unsafe extern "C" fn (f: *mut c_void, cursor: Emacs_Cursor)>,                        // C 3150
  #[cfg (feature = "have_window_system")]
  pub clear_frame_area: Option<unsafe extern "C" fn (f: *mut c_void, x: c_int, y: c_int, width: c_int, height: c_int)>, // C 3153
  #[cfg (feature = "have_window_system")]
  pub clear_under_internal_border: Option<unsafe extern "C" fn (f: *mut c_void)>,                                       // C 3159
  #[cfg (feature = "have_window_system")]
  pub draw_window_cursor: Option<unsafe extern "C" fn (w: *mut c_void, glyph_row: *mut glyph_row, x: c_int, y: c_int,
                                                       cursor_type: text_cursor_kinds, cursor_width: c_int,
                                                       on_p: bool, active_p: bool)>,                                  // C 3165
  #[cfg (feature = "have_window_system")]
  pub draw_vertical_window_border: Option<unsafe extern "C" fn (w: *mut c_void, x: c_int, y_0: c_int, y_1: c_int)>,   // C 3172
  #[cfg (feature = "have_window_system")]
  pub draw_window_divider: Option<unsafe extern "C" fn (w: *mut c_void, x_0: c_int, x_1: c_int, y_0: c_int, y_1: c_int)>, // C 3176
  #[cfg (feature = "have_window_system")]
  pub shift_glyphs_for_insert: Option<unsafe extern "C" fn (f: *mut c_void, x: c_int, y: c_int, width: c_int,
                                                            height: c_int, shift_by: c_int)>,                         // C 3182
  #[cfg (feature = "have_window_system")]
  pub show_hourglass: Option<unsafe extern "C" fn (f: *mut c_void)>,                                                    // C 3187
  #[cfg (feature = "have_window_system")]
  pub hide_hourglass: Option<unsafe extern "C" fn (f: *mut c_void)>,                                                    // C 3190
  #[cfg (feature = "have_window_system")]
  pub default_font_parameter: Option<unsafe extern "C" fn (f: *mut c_void, parms: Lisp_Object)>,                        // C 3194
}
/* C 3131-3195's `HAVE_WINDOW_SYSTEM` members - the fifteen display hooks this record already
   carries behind that feature - are appended after `fix_overlapping_area`, so they add 120 bytes
   and shift nothing above them (`docs/baseline/redisplay_interface-layout.tsv`: `frozen` 104/8
   with the thirteen base members, `x11`/`ns_cocoa`/`x11_ext` 224/8; the fifteen measure 104 …
   216).  The same device status 3.505/3.506 used for `struct face`, `struct font` and
   `struct font_driver`.  */
#[cfg (not (feature = "have_window_system"))]
const REDISPLAY_IFACE_WS_SHIFT: usize = 0;
#[cfg (feature = "have_window_system")]
const REDISPLAY_IFACE_WS_SHIFT: usize = 120;
#[cfg (all (feature = "have_window_system"))]
const _: () = core::assert! (core::mem::size_of::<redisplay_interface> () == 104 + REDISPLAY_IFACE_WS_SHIFT);

unsafe extern "C" {
  /// C: lisp.h 4260: `extern struct hash_table_test const hashtest_eq, hashtest_eql, hashtest_equal;` (fns.c defines it).
  pub static mut hashtest_eq: hash_table_test;
}

/* frame.h 34-45: `enum internal_border_part` is the mirror's item now - the frame.h mirror
   included at line 70 brings the type and its nine enumerators (status 3.211, the ownership move
   the mirror's own note asked for).  This unit's copy is retired here so that the two cannot be
   `E0428`.  (The wording avoids the include-directive spelling on purpose: xtask's dedupe pass
   reads that spelling as a directive and replaces the whole line - measured, and the reason the
   first attempt at this comment broke `build-tu src/frame.c`.)  */

/* C's `f->output_method = …` (frame.h 446's `ENUM_BF`); the reader is the mirror's
   `output_method (f)`, and this macro is the writer (PORTING.md 1.3).  */
macro_rules! output_method {
    ($f:expr) => { frame_output_method ($f) };
    ($f:expr, $v:expr) => { set_output_method! ($f, $v) };
}

/* frame.h's own spellings of two of those fields (C writes `FRAME_NO_SPECIAL_GLYPHS (f) = …`
   and reads and writes `FRAME_WINDOW_STATE_CHANGE (f)`); the mirror has the two readers as
   functions, and this macro adds C's assignment shape under C's own name.  */
macro_rules! FRAME_NO_SPECIAL_GLYPHS {
    ($f:expr) => { FRAME_NO_SPECIAL_GLYPHS ($f) };
    ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_NO_SPECIAL_GLYPHS_SHIFT, $v) };
}
macro_rules! FRAME_WINDOW_STATE_CHANGE {
    ($f:expr) => { FRAME_WINDOW_STATE_CHANGE ($f) };
    ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_WINDOW_STATE_CHANGE_SHIFT, $v) };
}

/* The writers of those flags.  C writes them as `f->flag = value`; the port's macro takes
   the value as its second argument (the same statement, the mirror's mask).  */
macro_rules! set_frame_flag1 {
    ($f:expr, $shift:expr, $v:expr) => {
        (*$f).bits1 = ((*$f).bits1 & ! (1u32 << $shift)) | (($v as u32) << $shift)
    };
}
macro_rules! set_frame_flag2 {
    ($f:expr, $shift:expr, $v:expr) => {
        (*$f).bits2 = ((*$f).bits2 & ! (1u32 << $shift)) | (($v as u32) << $shift)
    };
}

/* `src/dispnew.c` and `src/dispextern.h`: the tty predicates and `change_frame_size`; the
   mirror carries neither (PORTING.md P-72 rule 3).  */
unsafe extern "C" {
  /// C: dispextern.h 4012: `bool is_tty_frame (struct frame *f);`
  pub fn is_tty_frame (f: *mut frame) -> bool;
  /// C: dispextern.h 4013: `bool is_tty_child_frame (struct frame *f);`
  pub fn is_tty_child_frame (f: *mut frame) -> bool;
  /// C: dispextern.h 4014: `bool is_tty_root_frame (struct frame *f);`
  pub fn is_tty_root_frame (f: *mut frame) -> bool;
  /// C: dispextern.h 3924: `void change_frame_size (struct frame *, int, int, bool, bool, bool);`
  pub fn change_frame_size (f: *mut frame, new_width: c_int, new_height: c_int,
                            pretend: bool, delay: bool, safe: bool);
}

/* `src/window.c`, `src/dispnew.c`, `src/term.c`, `src/alloc.c` and `src/buffer.c` define
   these; the headers declare them and their mirrors do not carry them (PORTING.md P-72
   rule 3).  */
unsafe extern "C" {
  /// C: window.h 1139-1140: `void set_window_buffer (Lisp_Object window, Lisp_Object buffer,
  /// bool run_hooks, bool keep_margins);`
  pub fn set_window_buffer (window: Lisp_Object, buffer: Lisp_Object,
                            run_hooks: bool, keep_margins: bool);
  // window.h 1123's `make_window` is the window.rs mirror's now (`window_fn_decls`).
  /// C: lisp.h 4945: `extern Lisp_Object other_buffer_safely (Lisp_Object);`
  pub fn other_buffer_safely (buffer: Lisp_Object) -> Lisp_Object;
  /// C: lisp.h 5058: `extern Lisp_Object get_minibuffer (EMACS_INT);`
  pub fn get_minibuffer (depth: EMACS_INT) -> Lisp_Object;
  /// C: dispextern.h 3901: `extern void adjust_frame_glyphs (struct frame *);`
  pub fn adjust_frame_glyphs (f: *mut frame);
  /// C: dispextern.h 3955: `extern void calculate_costs (struct frame *);`
  pub fn calculate_costs (f: *mut frame);
  // window.h 1126's `resize_frame_windows` is the mirror's now; the mirror spells
  // `struct frame *` as `*mut c_void` (P-72 rule 3), so the two callers below cast (P-26).
  /// C: dispextern.h 3959: `extern void create_tty_output (struct frame *);`
  pub fn create_tty_output (f: *mut frame);
  /// C: dispextern.h 3960: `extern struct terminal *init_tty (const char *, const char *,
  /// bool);`
  pub fn init_tty (name: *const c_char, r#type: *const c_char, use_initial_terminal: bool)
    -> *mut terminal;
  /// C: dispextern.h 3780: `void get_tty_size (int, int *, int *);`
  pub fn get_tty_size (fileno: c_int, width: *mut c_int, height: *mut c_int);
  /// C: dispextern.h 4019: `int max_child_z_order (struct frame *parent);`
  pub fn max_child_z_order (parent: *mut frame) -> c_int;
  /// C: dispextern.h 3821: `void init_frame_faces (struct frame *);`
  pub fn init_frame_faces (f: *mut frame);
  /// C: dispextern.h 3947: `extern struct terminal *init_initial_terminal (void);`
  pub fn init_initial_terminal () -> *mut terminal;
  /// C: termhooks.h 961: `extern struct terminal *decode_live_terminal (Lisp_Object);`
  pub fn decode_live_terminal (terminal: Lisp_Object) -> *mut terminal;
  /// C: dispextern.h 3570: `bool resize_mini_window (struct window *, bool);`
  pub fn resize_mini_window (w: *mut window, pixelwise: bool) -> bool;
  /// C: lisp.h 5054: `extern void move_minibuffers_onto_frame (struct frame *, Lisp_Object,
  /// bool);`
  pub fn move_minibuffers_onto_frame (f: *mut frame, frame_: Lisp_Object, for_deletion: bool);
  /// C: dispextern.h 3574-3575: `void window_box (struct window *, enum glyph_row_area,
  /// int *, int *, int *, int *);`
  pub fn window_box (w: *mut window, area: glyph_row_area, x: *mut c_int, y: *mut c_int,
                     width: *mut c_int, height: *mut c_int);
  /// C: dispextern.h 4018: `void tty_raise_lower_frame (struct frame *f, bool raise);`
  pub fn tty_raise_lower_frame (f: *mut frame, raise: bool);
  /// C: dispextern.h 3794-3795: `void update_face_from_frame_parameter (struct frame *,
  /// Lisp_Object, Lisp_Object);`
  pub fn update_face_from_frame_parameter (f: *mut frame, prop: Lisp_Object, newval: Lisp_Object);
  /// C: dispextern.h 3801: `Lisp_Object tty_color_name (struct frame *, int);`
  pub fn tty_color_name (f: *mut frame, color: c_int) -> Lisp_Object;
  /// C: dispextern.h 3834: `extern char unspecified_fg[], unspecified_bg[];` (xfaces.c
  /// defines both; the mirror does not carry them).
  pub static mut unspecified_fg: [c_char; 0];
  pub static mut unspecified_bg: [c_char; 0];
  /// C: dispextern.h 3584-3585: `void pixel_to_glyph_coords (struct frame *, int, int,
  /// int *, int *, NativeRectangle *, bool);` — `NativeRectangle` is `int` on this target
  /// (dispextern.h 178-179's `#ifndef NativeRectangle #define NativeRectangle int`).
  pub fn pixel_to_glyph_coords (f: *mut frame, x: c_int, y: c_int, px: *mut c_int,
                                py: *mut c_int, bounds: *mut c_int, noclip: bool);
  // window.h 1235's `mru_rooted_frame` is the mirror's now (same `*mut c_void` spelling;
  // the two callers below cast).
  /// C: dispextern.h 3902: `void free_glyphs (struct frame *);`
  pub fn free_glyphs (f: *mut frame);
  // window.h 1129's `delete_all_child_windows` is the window.rs mirror's now (P-26).
  /// C: lisp.h 4275: `extern Lisp_Object delq_no_quit (Lisp_Object, Lisp_Object);`
  pub fn delq_no_quit (elt: Lisp_Object, list: Lisp_Object) -> Lisp_Object;
  /// C: font.h 897: `extern void free_font_driver_list (struct frame *f);`
  pub fn free_font_driver_list (f: *mut frame);
  /// C: keyboard.h 496: `extern void swallow_events (bool);`
  pub fn swallow_events (do_timers: bool);
  /// C: keyboard.h 479: `extern void not_single_kboard_state (KBOARD *);`
  pub fn not_single_kboard_state (kb: *mut KBOARD);
}

/* The currently selected frame.  */
/* frame.h 1428-1521 declares the seven objects below at the crate root (its mirror's
   `unsafe extern "C"` blocks), so the definitions live in a module that is glob-imported: an
   item shadows a glob import, while a second crate-root item of the same name would be
   `E0428` (PORTING.md P-21's mechanism, from the other side; the terminal.rs/term.rs
   `*_globals` shape).  */
pub mod frame_globals {
  use super::*;

  /* The currently selected frame.  */
  #[unsafe(no_mangle)]
  pub static mut selected_frame: Lisp_Object = lisp_h_Qnil ();

  /* The selected frame the last time window change functions were run.  */
  #[unsafe(no_mangle)]
  pub static mut old_selected_frame: Lisp_Object = lisp_h_Qnil ();

  /* False means there are no visible garbaged frames.  */
  #[unsafe(no_mangle)]
  pub static mut frame_garbaged: bool = false;

  /* The default tab bar height for future frames.  */
  #[unsafe(no_mangle)]
  pub static mut frame_default_tab_bar_height: c_int = 0;

  /* C 75-79: `#ifdef HAVE_EXT_TOOL_BAR` (dead, macros.tsv is undef), so C's live text is the
     `#else` arm's `int frame_default_tool_bar_height;`.  */
  #[unsafe(no_mangle)]
  pub static mut frame_default_tool_bar_height: c_int = 0;

  /* C 343: `EMACS_UINT frame_next_id = 1;` (0 indicates no id yet set).  */
  #[unsafe(no_mangle)]
  pub static mut frame_next_id: EMACS_UINT = 1;

  /* C 284: `Lisp_Object Vframe_list;`  */
  #[unsafe(no_mangle)]
  pub static mut Vframe_list: Lisp_Object = lisp_h_Qnil ();
}
use frame_globals::*;

/* A frame which is not just a mini-buffer, or NULL if there are no such
   frames.  This is usually the most recent such frame that was selected.  */

static mut last_nonminibuf_frame: *mut frame = std::ptr::null_mut ();

/* These setters are used only in this file, so they can be private.  */
unsafe fn fset_buffer_predicate (f: *mut frame, val: Lisp_Object)
{
  (*f).buffer_predicate = val;
}
unsafe fn fset_minibuffer_window (f: *mut frame, val: Lisp_Object)
{
  (*f).minibuffer_window = val;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn decode_live_frame (frame: Lisp_Object) -> *mut frame
{
  let mut frame = frame;
  if NILP (frame)
    { frame = selected_frame; }
  CHECK_LIVE_FRAME! (frame);
  return XFRAME (frame);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn decode_any_frame (frame: Lisp_Object) -> *mut frame
{
  let mut frame = frame;
  if NILP (frame)
    { frame = selected_frame; }
  CHECK_FRAME! (frame);
  return XFRAME (frame);
}

/* C 134-142: with a window system the check returns when one is available; without one the
   only live tail is `error (…)`.  Android's configuration compiles the first arm (its
   HAVE_WINDOW_SYSTEM is 1), and the port had only the frozen arm - so creating the first GUI
   frame errored with "Window system frame should be used" (device run, status 4.856).  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_window_system (f: *mut frame)
{
  if window_system_available (f)
    { return; }
  error (if !f.is_null ()
         { c"Window system frame should be used".as_ptr () }
         else { c"Window system is not in use or not initialized".as_ptr () });
}

#[cfg (not (feature = "have_window_system"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_window_system (f: *mut frame) -> !
{
  error (if !f.is_null ()
         { c"Window system frame should be used".as_ptr () }
         else { c"Window system is not in use or not initialized".as_ptr () });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn decode_window_system_frame (frame: Lisp_Object) -> *mut frame
{
  let f = decode_live_frame (frame);
  check_window_system (f);
  #[cfg (feature = "have_window_system")]
  { return f; }
  #[cfg (not (feature = "have_window_system"))]
  { unreachable! () }
}

unsafe fn check_tty (f: *mut frame)
{
  /* FIXME: the noninteractive case is here because some tests running
     in batch mode, like xt-mouse-tests, test with the initial frame
     which is no tty frame.  It would be nicer if the test harness
     would allow testing with real tty frames.  */
  if !f.is_null () && noninteractive
    { return; }

  if f.is_null () || !FRAME_TERMCAP_P (f)
    { error (c"tty frame should be used".as_ptr ()); }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn decode_tty_frame (frame: Lisp_Object) -> *mut frame
{
  let f = decode_live_frame (frame);
  check_tty (f);
  return f;
}

/* Return a frame with the given NAME (a string) or nil.  Note that if
   there are several frames with this NAME, the first found is returned.
   This function was inspired by Fget_buffer.  */
unsafe fn frame_get (name: Lisp_Object) -> Lisp_Object
{
  let mut _list_var: Lisp_Object = Qnil ();
  let mut frame: Lisp_Object = Qnil ();

  FOR_EACH_FRAME! (_list_var, frame, {
    if !NILP (Fstring_equal ((*XFRAME (frame)).name, name))
      { return frame; }
  });
  return Qnil ();
}

/* Return the value of frame parameter PROP in frame FRAME.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_frame_param (frame: *mut frame, prop: Lisp_Object)
  -> Lisp_Object
{
  return Fcdr (Fassq (prop, (*frame).param_alist));
}

/* Return true if 'frame-inhibit-implied-resize' is non-nil or
   fullscreen state of frame F would be affected by a vertical
   (horizontal if HORIZONTAL is true) resize.  PARAMETER is the symbol
   of the frame parameter about to be changed.

   If 'frame-inhibit-implied-resize' equals 'force', unconditionally
   return true (Bug#76275).  Otherwise, return nil if F has not been
   made yet and (on GTK) its tool bar has not been resized at least
   once.  Together these should ensure that F always gets its requested
   initial size.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_inhibit_resize (f: *mut frame, horizontal: bool,
                                               parameter: Lisp_Object) -> bool
{
  let fullscreen: Lisp_Object = get_frame_param (f, Qfullscreen ());

  return (EQ (frame_inhibit_implied_resize, Qforce ())
	  || (after_make_frame! (f)
	      && (EQ (frame_inhibit_implied_resize, Qt ())
		  || (CONSP (frame_inhibit_implied_resize)
		      && !NILP (Fmemq (parameter, frame_inhibit_implied_resize)))
		  || (horizontal
		      && !NILP (fullscreen) && !EQ (fullscreen, Qfullheight ()))
		  || (!horizontal
		      && !NILP (fullscreen) && !EQ (fullscreen, Qfullwidth ()))
		  || is_tty_frame (f))));
}

/** Set menu bar lines for a TTY frame.  */
unsafe fn set_menu_bar_lines (f: *mut frame, value: Lisp_Object, oldval: Lisp_Object)
{
  let olines: c_int = FRAME_MENU_BAR_LINES! (f);
  let nlines: c_int = if TYPE_RANGED_FIXNUMP (INT_MIN as EMACS_INT, value,
                                              INT_MAX as EMACS_INT)
    { XFIXNUM (value) as c_int } else { 0 };

  if is_tty_frame (f)
    {
      /* Menu bars on child frames don't work on all platforms, which is
	 the reason why prepare_menu_bar does not update_menu_bar for
	 child frames (info from Martin Rudalics).  This could be
	 implemented in ttys, but it's probably not worth it.  */
      if !FRAME_PARENT_FRAME (f).is_null ()
	/* C's `FRAME_MENU_BAR_LINES (f) = FRAME_MENU_BAR_HEIGHT (f) = 0;` is one chained
	   assignment; Rust's assignment is not an expression, so the port keeps C's two
	   effects in C's order as two statements(`docs/ports/frame.md` records it).  */
	{ FRAME_MENU_BAR_HEIGHT! (f) = 0; FRAME_MENU_BAR_LINES! (f) = 0; }
      else
	{
	  /* Make only 0 or 1 menu bar line (Bug#77015).  */
	  FRAME_MENU_BAR_HEIGHT! (f) = if nlines > 0 { 1 } else { 0 };
	  FRAME_MENU_BAR_LINES! (f) = FRAME_MENU_BAR_HEIGHT! (f);

	  if FRAME_MENU_BAR_LINES! (f) != olines
	    {
	      windows_or_buffers_changed = 14;
	      change_frame_size
		(f, FRAME_PIXEL_WIDTH! (f), FRAME_PIXEL_HEIGHT! (f),
		 false, true, false);
	    }
	}
    }
  /* Right now, menu bars don't work properly in minibuf-only frames;
     most of the commands try to apply themselves to the minibuffer
     frame itself, and get an error because you can't switch buffers
     in or split the minibuffer window.  */
  else if !FRAME_MINIBUF_ONLY_P (f) && nlines != olines
    {
      windows_or_buffers_changed = 14;
      FRAME_MENU_BAR_HEIGHT! (f) = nlines;
      FRAME_MENU_BAR_LINES! (f) = FRAME_MENU_BAR_HEIGHT! (f);
      change_frame_size (f, FRAME_PIXEL_WIDTH! (f), FRAME_PIXEL_HEIGHT! (f),
			 false, true, false);
    }
}

/** Set tab bar lines for a TTY frame.  */
unsafe fn set_tab_bar_lines (f: *mut frame, value: Lisp_Object, oldval: Lisp_Object)
{
  let olines: c_int = FRAME_TAB_BAR_LINES! (f);
  let nlines: c_int = if TYPE_RANGED_FIXNUMP (INT_MIN as EMACS_INT, value,
                                              INT_MAX as EMACS_INT)
    { XFIXNUM (value) as c_int } else { 0 };

  /* Right now, tab bars don't work properly in minibuf-only frames;
     most of the commands try to apply themselves to the minibuffer
     frame itself, and get an error because you can't switch buffers
     in or split the minibuffer window.  */
  if !FRAME_MINIBUF_ONLY_P (f) && nlines != olines
    {
      windows_or_buffers_changed = 14;
      FRAME_TAB_BAR_HEIGHT! (f) = nlines;
      FRAME_TAB_BAR_LINES! (f) = FRAME_TAB_BAR_HEIGHT! (f);
      change_frame_size (f, FRAME_PIXEL_WIDTH! (f), FRAME_PIXEL_HEIGHT! (f),
			 false, true, false);
    }
}

DEFUN! ("framep", Fframep, Sframep, 1, 1, 0,
       doc: "Return non-nil if OBJECT is a frame.\nValue is:\n  t for a termcap frame (a character-only terminal),\n `x' for an Emacs frame that is really an X window,\n `w32' for an Emacs frame that is a window on MS-Windows display,\n `ns' for an Emacs frame on a GNUstep or Macintosh Cocoa display,\n `pc' for a direct-write MS-DOS frame,\n `pgtk' for an Emacs frame running on pure GTK.\n `haiku' for an Emacs frame running in Haiku.\n `android' for an Emacs frame running in Android.\nSee also `frame-live-p'.  ",
       (object: Lisp_Object),
       {
  if !FRAMEP (object)
    { return Qnil (); }
  match output_method (XFRAME (object))
    {
    output_initial => { return Qt (); }   /* The initial frame is like a termcap frame. */
    output_termcap => { return Qt (); }
    output_x_window => { return Qx (); }
    output_w32 => { return Qw32 (); }
    output_msdos_raw => { return Qpc (); }
    output_ns => { return Qns (); }
    output_pgtk => { return Qpgtk (); }
    output_haiku => { return Qhaiku (); }
    output_android => { return Qandroid (); }
    _ => { emacs_abort (); }
    }
});

DEFUN! ("frame-live-p", Fframe_live_p, Sframe_live_p, 1, 1, 0,
       doc: "Return non-nil if OBJECT is a frame which has not been deleted.\nValue is nil if OBJECT is not a live frame.  If object is a live\nframe, the return value indicates what sort of terminal device it is\ndisplayed on.  See the documentation of `framep' for possible\nreturn values.  ",
       (object: Lisp_Object),
       {
  return if FRAMEP (object)
	   && FRAME_LIVE_P (XFRAME (object))
	 { Fframep (object) }
	 else { Qnil () };
});

/*                         Frame id.                             */

DEFUN! ("frame-id", Fframe_id, Sframe_id, 0, 1, 0,
       doc: "Return FRAME's id.\nIf FRAME is nil, use the selected frame.\nReturn nil if the id has not been set.  ",
       (frame: Lisp_Object),
       {
  let f = decode_live_frame (frame);
  if (*f).id == 0
    { return Qnil (); }
  else
    { return make_fixnum ((*f).id as EMACS_INT); }
});

/** frame_set_id: Set frame F's id to ID.

    If ID is 0 and F's ID is 0, use frame_next_id and increment it,
    otherwise, use ID.

    Signal an error if ID >= frame_next_id.
    Signal an error if ID is in use on another live frame.

    Return ID if it was used, 0 otherwise.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_set_id (f: *mut frame, id: EMACS_UINT) -> EMACS_UINT
{
  let mut id = id;
  if id >= frame_next_id
    { error (c"Specified frame ID unassigned".as_ptr ()); }

  if id > 0
    {
      eassume! (CONSP (Vframe_list));
      let mut frame: Lisp_Object = Qnil ();
      let mut tail: Lisp_Object = Qnil ();
      FOR_EACH_FRAME! (tail, frame, {
	if id == (*XFRAME (frame)).id
	  { error (c"Specified frame ID already in use".as_ptr ()); }
      });
    }

  if id == 0
    {
      if (*f).id != 0
	{ return 0; }
      else
	{ (*f).id = frame_next_id; frame_next_id += 1; }
    }
  else
    { (*f).id = id; }
  return (*f).id;
}

/** frame_set_id_from_params: Set frame F's id from the frame parameters
    in PARAMS (an alist).  If there's a valid `id' parameter take it
    and a valid positive integer greater than 0, otherwise use 0.

    Return frame_set_id's return value.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_set_id_from_params (f: *mut frame, params: Lisp_Object)
  -> EMACS_UINT
{
  let mut id: EMACS_UINT = 0;
  let param_id: Lisp_Object = Fcdr (Fassq (Qframe_id (), params));
  if TYPE_RANGED_FIXNUMP (INT_MIN as EMACS_INT, param_id, INT_MAX as EMACS_INT)
    {
      let id_1: EMACS_INT = XFIXNUM (param_id);
      if id_1 > 0
	{ id = id_1 as EMACS_UINT; }
    }
  return frame_set_id (f, id);
}

DEFUN! ("window-system", Fwindow_system, Swindow_system, 0, 1, 0,
       doc: "The name of the window system that FRAME is displaying through.\nThe value is a symbol:\n nil for a termcap frame (a character-only terminal),\n `x' for an Emacs frame that is really an X window,\n `w32' for an Emacs frame that is a window on MS-Windows display,\n `ns' for an Emacs frame on a GNUstep or Macintosh Cocoa display,\n `pc' for a direct-write MS-DOS frame.\n `pgtk' for an Emacs frame using pure GTK facilities.\n `haiku' for an Emacs frame running in Haiku.\n `android' for an Emacs frame running in Android.\n\nFRAME defaults to the currently selected frame.\n\nUse of this function as a predicate is deprecated.  Instead,\nuse `display-graphic-p' or any of the other `display-*-p'\npredicates which report frame's specific UI-related capabilities.  ",
       (frame: Lisp_Object),
       {
  let mut frame = frame;
  let r#type: Lisp_Object;
  if NILP (frame)
    { frame = selected_frame; }

  r#type = Fframep (frame);

  if NILP (r#type)
    { wrong_type_argument (Qframep (), frame); }

  if EQ (r#type, Qt ())
    { return Qnil (); }
  else
    { return r#type; }
});

/** Return true if F can be redisplayed, that is if F is visible and, if
    F is a tty frame, all its ancestors are visible too.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_redisplay_p (f: *mut frame) -> bool
{
  if is_tty_frame (f)
    {
      let mut p: *mut frame = f;
      let mut q: *mut frame = f;

      while !p.is_null ()
	{
	  if !visible! (p)
	    /* A tty child frame cannot be redisplayed if one of its
	       ancestors is invisible.  */
	    { return false; }
	  else
	    {
	      q = p;
	      p = FRAME_PARENT_FRAME (p);
	    }
	}

      let tty: *mut tty_display_info = FRAME_TTY! (f);
      let r: *mut frame = XFRAME ((*tty).top_frame);

      /* A tty child frame can be redisplayed iff its root is the top
	 frame of its terminal.  Any other tty frame can be redisplayed
	 iff it is the top frame of its terminal itself which must be
	 always visible.  */
      return q == r;
    }
  else
    { return FRAME_VISIBLE_P (f); }
}

/* Placeholder used by temacs -nw before window.el is loaded.  */
DEFUN! ("frame-windows-min-size", Fframe_windows_min_size,
       Sframe_windows_min_size, 4, 4, 0,
       doc: "SKIP: real doc in window.el.  ",
       (frame: Lisp_Object, horizontal: Lisp_Object,
	ignore: Lisp_Object, pixelwise: Lisp_Object),
       {
  return make_fixnum (0);
});

/**
 * frame_windows_min_size:
 *
 * Return the minimum number of lines (columns if HORIZONTAL is non-nil)
 * of FRAME.  If PIXELWISE is non-nil, return the minimum inner height
 * (width) of FRAME in pixels.
 *
 * This value is calculated by the function `frame-windows-min-size' in
 * window.el unless the `min-height' (`min-width' if HORIZONTAL is
 * non-nil) parameter of FRAME is non-nil thus explicitly specifying the
 * value to be returned.  In that latter case IGNORE is ignored.
 *
 * If `frame-windows-min-size' is called, it will make sure that the
 * return value accommodates all windows of FRAME respecting the values
 * of `window-min-height' (`window-min-width' if HORIZONTAL is
 * non-nil) and `window-safe-min-height' (`window-safe-min-width')
 * according to IGNORE (see `window-min-size').
 *
 * In either case, never return a value less than 1.  For TTY frames,
 * additionally limit the minimum frame height to a value large enough
 * to support menu bar, tab bar, mode line and echo area.
 */
unsafe fn frame_windows_min_size (frame: Lisp_Object, horizontal: Lisp_Object,
				ignore: Lisp_Object, pixelwise: Lisp_Object) -> c_int
{
  let f: *mut frame = XFRAME (frame);
  let mut par_size: Lisp_Object = Qnil ();
  let mut retval: c_int;

  if (!NILP (horizontal)
       && RANGED_FIXNUMP (INT_MIN as EMACS_INT,
			  { par_size = get_frame_param (f, Qmin_width ()); par_size },
			  INT_MAX as EMACS_INT))
      || (NILP (horizontal)
	  && RANGED_FIXNUMP (INT_MIN as EMACS_INT,
			     { par_size = get_frame_param (f, Qmin_height ()); par_size },
			     INT_MAX as EMACS_INT))
    {
      let mut min_size: c_int = XFIXNUM (par_size) as c_int;

      /* Don't allow phantom frames.  */
      if min_size < 1
	{ min_size = 1; }

      retval = if NILP (pixelwise)
	{ min_size }
	else { min_size * (if NILP (horizontal)
			      { FRAME_LINE_HEIGHT (f) }
			      else { FRAME_COLUMN_WIDTH (f) }) };
    }
  else
    { retval = XFIXNUM (calln! (Qframe_windows_min_size (), frame, horizontal,
			       ignore, pixelwise)) as c_int; }

  /* Don't allow too small height of text-mode frames, or else cm.c
     might abort in cmcheckmagic.  */
  if is_tty_frame (f) && NILP (horizontal)
    {
      let mut min_height: c_int = (FRAME_MENU_BAR_LINES! (f) + FRAME_TAB_BAR_LINES! (f)
				   + (FRAME_WANTS_MODELINE_P (f) as c_int)
				   + (FRAME_HAS_MINIBUF_P (f) as c_int));
      if min_height == 0
	{ min_height = 1; }
      if retval < min_height
	{ retval = min_height; }
    }

  return retval;
}

/**
 * keep_ratio:
 *
 * Preserve ratios of frame F which usually happens after its parent
 * frame P got resized.  OLD_WIDTH, OLD_HEIGHT specifies the old native
 * size of F's parent, NEW_WIDTH and NEW_HEIGHT its new size.
 *
 * Adjust F's width if F's 'keep_ratio' parameter is non-nil and, if
 * it is a cons, its car is not 'height-only'.  Adjust F's height if F's
 * 'keep_ratio' parameter is non-nil and, if it is a cons, its car
 * is not 'width-only'.
 *
 * Adjust F's left position if F's 'keep_ratio' parameter is non-nil
 * and, if its is a cons, its cdr is non-nil and not 'top-only'.  Adjust
 * F's top position if F's 'keep_ratio' parameter is non-nil and, if
 * its is a cons, its cdr is non-nil and not 'left-only'.
 *
 * Note that when positional adjustment is requested but the size of F
 * should remain unaltered in the corresponding direction, this routine
 * tries to constrain F to its parent frame - something which usually
 * happens when the parent frame shrinks.  This means, however, that
 * when the parent frame is re-enlarged later, the child's original
 * position will not get restored to its pre-shrinking value.
 *
 * This routine is currently useful for child frames only.  It might be
 * eventually useful when moving non-child frames between monitors with
 * different resolutions.
 */
unsafe fn keep_ratio (f: *mut frame, p: *mut frame, old_width: c_int, old_height: c_int,
	    new_width: c_int, new_height: c_int)
{
  let keep_ratio: Lisp_Object = get_frame_param (f, Qkeep_ratio ());

  if !NILP (keep_ratio)
    {
      let width_factor: c_double = new_width as c_double / old_width as c_double;
      let height_factor: c_double = new_height as c_double / old_height as c_double;
      let mut pixel_width: c_int = 0;
      let mut pixel_height: c_int = 0;
      let mut pos_x: c_int = 0;
      let mut pos_y: c_int = 0;

      if !CONSP (keep_ratio) || !NILP (Fcdr (keep_ratio))
	{
	  if CONSP (keep_ratio) && EQ (Fcdr (keep_ratio), Qtop_only ())
	    { pos_x = (*f).left_pos; }
	  else
	    {
	      pos_x = ((*f).left_pos as c_double * width_factor + 0.5) as c_int;

	      if CONSP (keep_ratio)
		  && (NILP (Fcar (keep_ratio))
		      || EQ (Fcar (keep_ratio), Qheight_only ()))
		  && FRAME_PIXEL_WIDTH! (p) - FRAME_PIXEL_WIDTH! (f) < pos_x
		{
		  let p_f_width: c_int
		    = FRAME_PIXEL_WIDTH! (p) - FRAME_PIXEL_WIDTH! (f);

		  if p_f_width <= 0
		    { pos_x = 0; }
		  else
		    { pos_x = (p_f_width as c_double * width_factor * 0.5 + 0.5) as c_int; }
		}

	      (*f).left_pos = pos_x;
	    }

	  if CONSP (keep_ratio) && EQ (Fcdr (keep_ratio), Qleft_only ())
	    { pos_y = (*f).top_pos; }
	  else
	    {
	      pos_y = ((*f).top_pos as c_double * height_factor + 0.5) as c_int;

	      if CONSP (keep_ratio)
		  && (NILP (Fcar (keep_ratio))
		      || EQ (Fcar (keep_ratio), Qwidth_only ()))
		  && FRAME_PIXEL_HEIGHT! (p) - FRAME_PIXEL_HEIGHT! (f) < pos_y
		/* When positional adjustment was requested and the
		   width of F should remain unaltered, try to constrain
		   F to its parent.  This means that when the parent
		   frame is enlarged later the child's original position
		   won't get restored.  */
		{
		  let p_f_height: c_int
		    = FRAME_PIXEL_HEIGHT! (p) - FRAME_PIXEL_HEIGHT! (f);

		  if p_f_height <= 0
		    { pos_y = 0; }
		  else
		    { pos_y = (p_f_height as c_double * height_factor * 0.5 + 0.5) as c_int; }
		}

	      (*f).top_pos = pos_y;
	    }

          if let Some (set_frame_offset_hook) = (*FRAME_TERMINAL! (f)).set_frame_offset_hook
            { set_frame_offset_hook (f as *mut c_void, pos_x, pos_y, -1); }
	}

      if !CONSP (keep_ratio) || !NILP (Fcar (keep_ratio))
	{
	  if CONSP (keep_ratio) && EQ (Fcar (keep_ratio), Qheight_only ())
	    { pixel_width = -1; }
	  else
	    { pixel_width
		= (FRAME_PIXEL_WIDTH! (f) as c_double * width_factor + 0.5) as c_int; }

	  if CONSP (keep_ratio) && EQ (Fcar (keep_ratio), Qwidth_only ())
	    { pixel_height = -1; }
	  else
	    { pixel_height
		= (FRAME_PIXEL_HEIGHT! (f) as c_double * height_factor + 0.5) as c_int; }

	  adjust_frame_size (f, FRAME_PIXEL_TO_TEXT_WIDTH! (f, pixel_width),
			     FRAME_PIXEL_TO_TEXT_HEIGHT! (f, pixel_height), 1,
			     false, Qkeep_ratio ());
	}
    }
}

unsafe fn frame_size_history_adjust (f: *mut frame, inhibit: c_int, parameter: Lisp_Object,
			   old_text_width: c_int, old_text_height: c_int,
			   new_text_width: c_int, new_text_height: c_int,
			   old_text_cols: c_int, old_text_lines: c_int,
			   new_text_cols: c_int, new_text_lines: c_int,
			   old_native_width: c_int, old_native_height: c_int,
			   new_native_width: c_int, new_native_height: c_int,
			   old_inner_width: c_int, old_inner_height: c_int,
			   new_inner_width: c_int, new_inner_height: c_int,
			   min_inner_width: c_int, min_inner_height: c_int,
			   inhibit_horizontal: bool, inhibit_vertical: bool)
{
  let mut frame: Lisp_Object = Qnil ();

  XSETFRAME! (frame, f);
  if CONSP (frame_size_history)
      && FIXNUMP (XCAR (frame_size_history))
      && 0 < XFIXNUM (XCAR (frame_size_history))
    {
    frame_size_history =
      Fcons (make_fixnum (XFIXNUM (XCAR (frame_size_history)) - 1),
	     Fcons (Fcons (list4 (frame, make_fixnum (5),
				  make_fixnum (inhibit as EMACS_INT), parameter),
			   list5 (list4i (old_text_width as EMACS_INT, old_text_height as EMACS_INT,
					  new_text_width as EMACS_INT, new_text_height as EMACS_INT),
				  list4i (old_text_cols as EMACS_INT, old_text_lines as EMACS_INT,
					  new_text_cols as EMACS_INT, new_text_lines as EMACS_INT),
				  list4i (old_native_width as EMACS_INT, old_native_height as EMACS_INT,
					  new_native_width as EMACS_INT, new_native_height as EMACS_INT),
				  list4i (old_inner_width as EMACS_INT, old_inner_height as EMACS_INT,
					  new_inner_width as EMACS_INT,  new_inner_height as EMACS_INT),
				  list4 (make_fixnum (min_inner_width as EMACS_INT),
					 make_fixnum (min_inner_height as EMACS_INT),
					 if inhibit_horizontal { Qt () } else { Qnil () },
					 if inhibit_vertical { Qt () } else { Qnil () }))),
		    XCDR (frame_size_history)));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_size_history_plain (f: *mut frame, parameter: Lisp_Object)
{
  let mut frame: Lisp_Object = Qnil ();

  XSETFRAME! (frame, f);
  if CONSP (frame_size_history)
      && FIXNUMP (XCAR (frame_size_history))
      && 0 < XFIXNUM (XCAR (frame_size_history))
    {
    frame_size_history =
      Fcons (make_fixnum (XFIXNUM (XCAR (frame_size_history)) - 1),
	     Fcons (Fcons (list3 (frame, make_fixnum (1), parameter), Qt ()),
		    XCDR (frame_size_history)));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_size_history_extra (f: *mut frame, parameter: Lisp_Object,
			  pixel_width: c_int, pixel_height: c_int,
			  extra_width: c_int, extra_height: c_int,
			  delayed_width: c_int, delayed_height: c_int)
{
  let mut frame: Lisp_Object = Qnil ();

  XSETFRAME! (frame, f);
  if CONSP (frame_size_history)
      && FIXNUMP (XCAR (frame_size_history))
      && 0 < XFIXNUM (XCAR (frame_size_history))
    {
    frame_size_history =
      Fcons (make_fixnum (XFIXNUM (XCAR (frame_size_history)) - 1),
	     Fcons (Fcons (list3 (frame, make_fixnum (2), parameter),
			   list2 (list4i (pixel_width as EMACS_INT, pixel_height as EMACS_INT,
					  extra_width as EMACS_INT, extra_height as EMACS_INT),
				  list2i (delayed_width as EMACS_INT, delayed_height as EMACS_INT))),
		    XCDR (frame_size_history)));
    }
}

/**
 * adjust_frame_size:
 *
 * Adjust size of frame F.  NEW_TEXT_WIDTH and NEW_TEXT_HEIGHT specify
 * the new text size of F in pixels.  When INHIBIT equals 2, 3 or 4, a
 * value of -1 means to leave the text size of F unchanged and adjust,
 * if necessary and possible, F's native size accordingly.  When INHIBIT
 * equals 0, 1 or 5, a negative value means that the frame has been (or
 * should be) made pathologically small which usually means that parts
 * of the frame's windows may not be entirely visible.
 *
 * The effect of calling this function can be to either issue a request
 * to resize the frame externally (via set_window_size_hook), to resize
 * the frame internally (via resize_frame_windows) or to do nothing.
 *
 * The argument INHIBIT controls whether set_window_size_hook may be
 * called and can assume the following values:
 *
 * 0 means to unconditionally call set_window_size_hook even if sizes
 *   apparently do not change.  Fx_create_frame uses this to pass the
 *   initial size to the window manager.
 *
 * 1 means to call set_window_size_hook if the native frame size should
 *   change.  Fset_frame_size and friends and width and height parameter
 *   changes use this.
 *
 * 2 means to call set_window_size_hook provided frame_inhibit_resize
 *   allows it.  The code updating external menu and tool bars uses this
 *   to keep the height of the native frame unaltered when one of these
 *   bars is added or removed.  This means that Emacs has to work
 *   against the window manager which usually tries to keep the combined
 *   height (native frame plus bar) unaltered.
 *
 * 3 means to call set_window_size_hook if window minimum sizes must be
 *   preserved or frame_inhibit_resize allows it.  This is the default
 *   for parameters accounted for in a frame's text size like fringes,
 *   scroll bars, internal border, tab bar, internal tool and menu bars.
 *   It's also used when the frame's default font changes.
 *
 * 4 means to call set_window_size_hook only if window minimum sizes
 *   must be preserved.  The code for setting up window dividers and
 *   that responsible for wrapping the (internal) tool bar use this.
 *
 * 5 means to never call set_window_size_hook.  Usually this means to
 *   call resize_frame_windows.  change_frame_size uses this.
 *
 * PRETEND is as for change_frame_size.  PARAMETER, if non-nil, is the
 * symbol of the parameter changed (like `menu-bar-lines', `font', ...).
 * This is passed on to frame_inhibit_resize to let the latter decide on
 * a case-by-case basis whether set_window_size_hook should be called.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn adjust_frame_size (f: *mut frame, new_text_width: c_int,
					   new_text_height: c_int,
		   inhibit: c_int, pretend: bool, parameter: Lisp_Object)
{
  let unit_width: c_int = FRAME_COLUMN_WIDTH (f);
  let unit_height: c_int = FRAME_LINE_HEIGHT (f);
  let old_native_width: c_int = FRAME_PIXEL_WIDTH! (f);
  let old_native_height: c_int = FRAME_PIXEL_HEIGHT! (f);
  let mut new_native_width: c_int;
  let mut new_native_height: c_int;
  /* The desired minimum inner width and height of the frame calculated
     via 'frame-windows-min-size'.  */
  let min_inner_width: c_int;
  let min_inner_height: c_int;
  /* Get the "old" inner width, height and position of F via its root
     window and the minibuffer window.  We cannot use FRAME_INNER_WIDTH
     and FRAME_INNER_HEIGHT here since the internal border and the top
     margin may have been already set to new values.  */
  let r: *mut window = XWINDOW (FRAME_ROOT_WINDOW (f));
  let old_inner_width: c_int = WINDOW_PIXEL_WIDTH (r);
  let old_inner_height: c_int
    = (WINDOW_PIXEL_HEIGHT (r)
       + (if FRAME_HAS_MINIBUF_P (f) && !FRAME_MINIBUF_ONLY_P (f)
	  { WINDOW_PIXEL_HEIGHT (XWINDOW (FRAME_MINIBUF_WINDOW (f))) }
	  else { 0 }));
  let mut new_inner_width: c_int;
  let mut new_inner_height: c_int;
  let old_text_cols: c_int = FRAME_COLS! (f);
  let old_text_lines: c_int = FRAME_LINES! (f);
  let mut new_text_cols: c_int;
  let mut new_text_lines: c_int;
  let mut old_text_width: c_int = FRAME_TEXT_WIDTH! (f);
  let mut old_text_height: c_int = FRAME_TEXT_HEIGHT! (f);
  let mut new_text_width = new_text_width;
  let mut new_text_height = new_text_height;
  let inhibit_horizontal: bool;
  let inhibit_vertical: bool;
  let size_and_position: bool = EQ (parameter, Qsize_and_position ());
  let mut frame: Lisp_Object = Qnil ();

  XSETFRAME! (frame, f);

  min_inner_width
    = frame_windows_min_size (frame, Qt (), if inhibit == 5 { Qsafe () } else { Qnil () },
			      Qt ());
  min_inner_height
    = frame_windows_min_size (frame, Qnil (), if inhibit == 5 { Qsafe () } else { Qnil () },
			      Qt ());

  if inhibit == 1 && !NILP (alter_fullscreen_frames)
    {
      let fullscreen: Lisp_Object = get_frame_param (f, Qfullscreen ());

      if (new_text_width != old_text_width
	   && !NILP (fullscreen) && !EQ (fullscreen, Qfullheight ()))
	  || (new_text_height != old_text_height
	      && !NILP (fullscreen) && !EQ (fullscreen, Qfullwidth ()))

	{
	  if EQ (alter_fullscreen_frames, Qt ())
	    /* Reset fullscreen status and proceed.  */
	    { Fmodify_frame_parameters
		(frame, Fcons (Fcons (Qfullscreen (), Qnil ()), Qnil ())); }
	  else if EQ (alter_fullscreen_frames, Qinhibit ())
	    /* Do nothing and return.  */
	    { return; }
	}
    }

  if inhibit >= 2 && inhibit <= 4
    /* When INHIBIT is in [2..4] inhibit if the "old" window sizes stay
       within the limits and either resizing is inhibited or INHIBIT
       equals 4.  */
    {
      if new_text_width == -1
	{ new_text_width = FRAME_TEXT_WIDTH! (f); }
      if new_text_height == -1
	{ new_text_height = FRAME_TEXT_HEIGHT! (f); }

      inhibit_horizontal = (FRAME_INNER_WIDTH! (f) >= min_inner_width
                            && (inhibit == 4
                                || frame_inhibit_resize (f, true, parameter)));
      inhibit_vertical = (FRAME_INNER_HEIGHT! (f) >= min_inner_height
                          && (inhibit == 4
                              || frame_inhibit_resize (f, false, parameter)));
    }
  else
    /* Otherwise inhibit if INHIBIT equals 5.  If we wanted to overrule
       the WM do that here (could lead to some sort of eternal fight
       with the WM).  */
    { inhibit_horizontal = inhibit == 5; inhibit_vertical = inhibit == 5; }

  new_native_width = if inhibit_horizontal && inhibit < 5
		      { old_native_width }
		      else { max (FRAME_TEXT_TO_PIXEL_WIDTH! (f, new_text_width) as isize,
				  (min_inner_width
				   + 2 * FRAME_INTERNAL_BORDER_WIDTH (f)) as isize) as c_int };
  new_inner_width = new_native_width - 2 * FRAME_INTERNAL_BORDER_WIDTH (f);
  new_text_width = FRAME_PIXEL_TO_TEXT_WIDTH! (f, new_native_width);
  new_text_cols = new_text_width / unit_width;

  new_native_height = if inhibit_vertical && inhibit < 5
		       { old_native_height }
		       else { max (FRAME_TEXT_TO_PIXEL_HEIGHT! (f, new_text_height) as isize,
				   (min_inner_height
				    + FRAME_MARGIN_HEIGHT (f)
				    + 2 * FRAME_INTERNAL_BORDER_WIDTH (f)) as isize) as c_int };
  new_inner_height = (new_native_height
		      - FRAME_MARGIN_HEIGHT (f)
		      - 2 * FRAME_INTERNAL_BORDER_WIDTH (f));
  new_text_height = FRAME_PIXEL_TO_TEXT_HEIGHT! (f, new_native_height);
  new_text_lines = new_text_height / unit_height;

  if FRAME_WINDOW_P (f)
      && can_set_window_size! (f)
      /* For inhibit == 1 call the window_size_hook only if a native
	 size changes.  For inhibit == 0 or inhibit == 2 always call
	 it.  */
      && (size_and_position
	  || (!inhibit_horizontal
	   && (new_native_width != old_native_width
	       || inhibit == 0 || inhibit == 2))
	  || (!inhibit_vertical
	      && (new_native_height != old_native_height
		  || inhibit == 0 || inhibit == 2)))
    {
      if inhibit == 2
	  && ((*f).new_width >= 0 || (*f).new_height >= 0)
	/* For implied resizes with inhibit 2 (external menu and tool
	   bar) pick up any new sizes the display engine has not
	   processed yet.  Otherwise, we would request the old sizes
	   which will make this request appear as a request to set new
	   sizes and have the WM react accordingly which is not TRT.

	   We don't that for the external menu bar on Motif.
	   Otherwise, switching off the menu bar will shrink the frame
	   and switching it on will not enlarge it.  */
	{
	  if (*f).new_width >= 0
	    { new_native_width = (*f).new_width; }
	  if (*f).new_height >= 0
	    { new_native_height = (*f).new_height; }
	}

      if CONSP (frame_size_history)
	{ frame_size_history_adjust (f, inhibit, parameter,
				   old_text_width, old_text_height,
				   new_text_width, new_text_height,
				   old_text_cols, old_text_lines,
				   new_text_cols, new_text_lines,
				   old_native_width, old_native_height,
				   new_native_width, new_native_height,
				   old_inner_width, old_inner_height,
				   new_inner_width, new_inner_height,
				   min_inner_width, min_inner_height,
				   inhibit_horizontal, inhibit_vertical); }

      if inhibit == 0 || inhibit == 1
	{
	  (*f).new_width = new_native_width;
	  (*f).new_height = new_native_height;
	  /* Resetting f->new_size_p is controversial: It might cause
	     do_pending_window_change drop a previous request and we are
	     in troubles when the window manager does not honor the
	     request we issue here.  */
	  new_size_p! (f, false);
	}

      if size_and_position
	/* The caller must have set the new position and gravity and
	   made sure that set_window_size_and_position_hook has been
	   defined.  */
	{ if let Some (set_window_size_and_position_hook)
	    = (*FRAME_TERMINAL! (f)).set_window_size_and_position_hook
	  { set_window_size_and_position_hook (f as *mut c_void, new_native_width,
					       new_native_height); } }
      else if (*FRAME_TERMINAL! (f)).set_window_size_hook.is_some ()
        { if let Some (set_window_size_hook) = (*FRAME_TERMINAL! (f)).set_window_size_hook
	  { set_window_size_hook (f as *mut c_void, false, new_native_width,
				  new_native_height); } }
      resized_p! (f, true);

      return;
    }

  if CONSP (frame_size_history)
    { frame_size_history_adjust (f, inhibit, parameter,
			       old_text_width, old_text_height,
			       new_text_width, new_text_height,
			       old_text_cols, old_text_lines,
			       new_text_cols, new_text_lines,
			       old_native_width, old_native_height,
			       new_native_width, new_native_height,
			       old_inner_width, old_inner_height,
			       new_inner_width, new_inner_height,
			       min_inner_width, min_inner_height,
			       inhibit_horizontal, inhibit_vertical); }

  if (*(XWINDOW (FRAME_ROOT_WINDOW (f)))).pixel_top
       == FRAME_TOP_MARGIN_HEIGHT (f)
      && new_text_width == old_text_width
      && new_text_height == old_text_height
      && new_inner_width == old_inner_width
      && new_inner_height == old_inner_height
      /* We might be able to drop these but some doubts remain.  */
      && new_native_width == old_native_width
      && new_native_height == old_native_height
      && new_text_cols == old_text_cols
      && new_text_lines == old_text_lines
    /* No change.  */
    { return; }

  block_input ();

  if new_inner_width != old_inner_width
    {
      resize_frame_windows (f as *mut c_void, new_inner_width, true);

      /* MSDOS frames cannot PRETEND, as they change frame size by
	 manipulating video hardware.  */
      if is_tty_root_frame (f)
	{
	  if (FRAME_TERMCAP_P (f) && !pretend) || FRAME_MSDOS_P (f)
	    { FrameCols! (FRAME_TTY! (f)) = new_text_cols; }
	}

    }
  else if new_text_cols != old_text_cols
    { calln! (Qwindow__pixel_to_total (), frame, Qt ()); }

  if new_inner_height != old_inner_height
      /* When the top margin has changed we have to recalculate the top
	 edges of all windows.  No such calculation is necessary for the
	 left edges.  */
      || WINDOW_TOP_PIXEL_EDGE (r) != FRAME_TOP_MARGIN_HEIGHT (f)
    {
      resize_frame_windows (f as *mut c_void, new_inner_height, false);

      /* MSDOS frames cannot PRETEND, as they change frame size by
	 manipulating video hardware. */
      if is_tty_root_frame (f)
	{
	  if (FRAME_TERMCAP_P (f) && !pretend) || FRAME_MSDOS_P (f)
	    { FrameRows! (FRAME_TTY! (f)) = new_text_lines + FRAME_TOP_MARGIN (f); }
	}
    }
  else if new_text_lines != old_text_lines
    { calln! (Qwindow__pixel_to_total (), frame, Qnil ()); }

  /* Assign new sizes.  */
  FRAME_COLS! (f) = new_text_cols;
  FRAME_LINES! (f) = new_text_lines;
  FRAME_TEXT_WIDTH! (f) = new_text_width;
  FRAME_TEXT_HEIGHT! (f) = new_text_height;
  FRAME_PIXEL_WIDTH! (f) = new_native_width;
  FRAME_PIXEL_HEIGHT! (f) = new_native_height;
  FRAME_TOTAL_COLS! (f) = FRAME_PIXEL_WIDTH! (f) / FRAME_COLUMN_WIDTH (f);
  FRAME_TOTAL_LINES! (f) = FRAME_PIXEL_HEIGHT! (f) / FRAME_LINE_HEIGHT (f);

  {
    let w: *mut window = XWINDOW (FRAME_SELECTED_WINDOW (f));
    let mut text_area_x: c_int = 0;
    let mut text_area_y: c_int = 0;
    let mut text_area_width: c_int = 0;
    let mut text_area_height: c_int = 0;

    window_box (w, TEXT_AREA, &mut text_area_x, &mut text_area_y, &mut text_area_width,
		&mut text_area_height);
    if (*w).cursor.x >= text_area_x + text_area_width
      { (*w).cursor.hpos = 0; (*w).cursor.x = 0; }
    if (*w).cursor.y >= text_area_y + text_area_height
      { (*w).cursor.vpos = 0; (*w).cursor.y = 0; }
  }

  adjust_frame_glyphs (f);
  calculate_costs (f);
  SET_FRAME_GARBAGED! (f);
  if is_tty_child_frame (f)
    { SET_FRAME_GARBAGED! (root_frame (f)); }

  /* We now say here that F was resized instead of using the old
     condition below.  Some resizing must have taken place and if it was
     only shifting the root window's position (paranoia?).  */
  resized_p! (f, true);

//    /\* A frame was "resized" if its native size changed, even if its X
//       window wasn't resized at all.  *\/
//    f->resized_p = (new_native_width != old_native_width
// 		  || new_native_height != old_native_height);

  unblock_input ();

  {
    /* Adjust size of F's child frames.  */
    let mut frames: Lisp_Object = Qnil ();
    let mut frame1: Lisp_Object = Qnil ();

    FOR_EACH_FRAME! (frames, frame1, {
	if FRAME_PARENT_FRAME (XFRAME (frame1)) == f
	  { keep_ratio (XFRAME (frame1), f, old_native_width, old_native_height,
		      new_native_width, new_native_height); }
    });
  }
}

/* Allocate basically initialized frame.  */

unsafe fn allocate_frame () -> *mut frame
{
  return ALLOCATE_ZEROED_PSEUDOVECTOR! (frame, tool_bar_items,
				       pvec_type::PVEC_FRAME);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn make_frame (mini_p: bool) -> *mut frame
{
  let mut frame: Lisp_Object = Qnil ();
  let f: *mut frame;
  let mut rw: *mut window = std::ptr::null_mut ();
  /* C declares `struct window *rw, *mw UNINIT;`; the port initializes the UNINIT binding
     (PORTING.md 1.5).  */
  let mut mw: *mut window = std::ptr::null_mut ();
  let mut root_window: Lisp_Object = Qnil ();
  let mut mini_window: Lisp_Object = Qnil ();

  f = allocate_frame ();
  XSETFRAME! (frame, f);

  /* Initialize Lisp data.  Note that allocate_frame initializes all
     Lisp data to nil, so do it only for slots which should not be nil.  */
  fset_tool_bar_position (f, Qtop ());

  /* Initialize non-Lisp data.  Note that allocate_frame zeroes out all
     non-Lisp data, so do it only for slots which should not be zero.
     To avoid subtle bugs and for the sake of readability, it's better to
     initialize enum members explicitly even if their values are zero.  */
  wants_modeline! (f, true);
  redisplay! (f, true);
  garbaged! (f, true);
  can_set_window_size! (f, false);
  after_make_frame! (f, false);
  tab_bar_redisplayed! (f, false);
  tab_bar_resized! (f, false);
  tool_bar_redisplayed! (f, false);
  tool_bar_resized! (f, false);
  (*f).column_width = 1;  /* !FRAME_WINDOW_P value.  */
  (*f).line_height = 1;  /* !FRAME_WINDOW_P value.  */
  (*f).new_width = -1;
  (*f).new_height = -1;
  no_special_glyphs! (f, false);
  /* C 1187-1213's `#ifdef HAVE_WINDOW_SYSTEM` arm is dead (P-08).  */
  select_mini_window_flag! (f, false);
  /* This one should never be zero.  */
  (*f).change_stamp = 1;

  /* C 1218-1226's `#ifdef HAVE_TEXT_CONVERSION` arm is dead (P-08).  */

  root_window = make_window ();
  rw = XWINDOW (root_window);
  if mini_p
    {
      mini_window = make_window ();
      mw = XWINDOW (mini_window);
      wset_next (rw, mini_window);
      wset_prev (mw, root_window);
      mini! (mw, 1);
      wset_frame (mw, frame);
      fset_minibuffer_window (f, mini_window);
      store_frame_param (f, Qminibuffer (), Qt ());
    }
  else
    {
      mini_window = Qnil ();
      wset_next (rw, Qnil ());
      fset_minibuffer_window (f, Qnil ());
    }

  wset_frame (rw, frame);

  /* 80/25 is arbitrary, just so that there is "something there."
     Correct size will be set up later with adjust_frame_size.  */
  /* C's four chained assignments here (`FRAME_COLS (f) = FRAME_TOTAL_COLS (f) =
     rw->total_cols = 80;` and the three below) are split into C's own effects in C's own
     order: Rust's assignment is not an expression (`docs/ports/frame.md`).  */
  (*rw).total_cols = 80;
  FRAME_TOTAL_COLS! (f) = 80;
  FRAME_COLS! (f) = 80;
  (*rw).pixel_width = 80 * FRAME_COLUMN_WIDTH (f);
  FRAME_PIXEL_WIDTH! (f) = (*rw).pixel_width;
  FRAME_TEXT_WIDTH! (f) = (*rw).pixel_width;
  FRAME_LINES! (f) = 25;
  FRAME_TOTAL_LINES! (f) = 25;
  FRAME_TEXT_HEIGHT! (f) = 25 * FRAME_LINE_HEIGHT (f);
  FRAME_PIXEL_HEIGHT! (f) = 25 * FRAME_LINE_HEIGHT (f);

  (*rw).total_lines = FRAME_LINES! (f) - if mini_p { 1 } else { 0 };
  (*rw).pixel_height = (*rw).total_lines * FRAME_LINE_HEIGHT (f);

  fset_face_hash_table
    (f, make_hash_table (&raw const hashtest_eq, DEFAULT_HASH_SIZE as EMACS_INT, Weak_None));

  if mini_p
    {
      (*mw).top_line = (*rw).total_lines;
      (*mw).pixel_top = (*rw).pixel_height;
      (*mw).total_cols = (*rw).total_cols;
      (*mw).pixel_width = (*rw).pixel_width;
      (*mw).total_lines = 1;
      (*mw).pixel_height = FRAME_LINE_HEIGHT (f);
    }

  /* Choose a buffer for the frame's root window.  */
  {
    let mut buf: Lisp_Object = Fcurrent_buffer ();

    /* If the current buffer is hidden and shall not be exposed, try to find
       another one.  */
    if BUFFER_HIDDEN_P (XBUFFER (buf)) && NILP (expose_hidden_buffer)
      { buf = other_buffer_safely (buf); }

    /* Use set_window_buffer, not Fset_window_buffer, and don't let
       hooks be run by it.  The reason is that the whole frame/window
       arrangement is not yet fully initialized at this point.  Windows
       don't have the right size, glyph matrices aren't initialized
       etc.  Running Lisp functions at this point surely ends in a
       SEGV.  */
    set_window_buffer (root_window, buf, false, false);
    fset_buffer_list (f, list1 (buf));
  }

  if mini_p
    { set_window_buffer (mini_window,
		       if NILP (Vminibuffer_list) { get_minibuffer (0) }
		       else { Fcar (Vminibuffer_list) },
		       false, false); }

  fset_root_window (f, root_window);
  fset_selected_window (f, root_window);
  /* Make sure this window seems more recently used than
     a newly-created, never-selected window.  */
  (*XWINDOW ((*f).selected_window)).use_time = { window_select_count += 1; window_select_count };

  return f;
}

/* Make a frame using a separate minibuffer window on another frame.
   MINI_WINDOW is the minibuffer window to use.  nil means use the
   default (the global minibuffer).  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn make_frame_without_minibuffer (mini_window: Lisp_Object, kb: *mut KBOARD,
			       display: Lisp_Object) -> *mut frame
{
  let f: *mut frame;
  let mut mini_window = mini_window;

  if !NILP (mini_window)
    { CHECK_LIVE_WINDOW (mini_window); }

  if !NILP (mini_window)
      && FRAME_KBOARD (XFRAME ((*XWINDOW (mini_window)).frame)) != kb as *mut c_void
    { error (c"Frame and minibuffer must be on the same terminal".as_ptr ()); }

  /* Make a frame containing just a root window.  */
  f = make_frame (false);

  if NILP (mini_window)
    {
      /* Use default-minibuffer-frame if possible.  */
      if !FRAMEP ((*kb).Vdefault_minibuffer_frame_)
	  || !FRAME_LIVE_P (XFRAME ((*kb).Vdefault_minibuffer_frame_))
	{
	  let initial_frame: Lisp_Object;

	  /* If there's no minibuffer frame to use, create one.  */
	  initial_frame = calln! (Qmake_initial_minibuffer_frame (),
				 display);
	  kset_default_minibuffer_frame (kb, initial_frame);
	}

      mini_window
	= (*XFRAME ((*kb).Vdefault_minibuffer_frame_)).minibuffer_window;
    }

  fset_minibuffer_window (f, mini_window);
  store_frame_param (f, Qminibuffer (), mini_window);

  /* Make the chosen minibuffer window display the proper minibuffer,
     unless it is already showing a minibuffer.  */
  if NILP (Fmemq ((*XWINDOW (mini_window)).contents, Vminibuffer_list))
    /* Use set_window_buffer instead of Fset_window_buffer (see
       discussion of bug#11984, bug#12025, bug#12026).  */
    { set_window_buffer (mini_window,
		       if NILP (Vminibuffer_list) { get_minibuffer (0) }
		       else { Fcar (Vminibuffer_list) }, false, false); }
  return f;
}

/* Make a frame containing only a minibuffer window.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn make_minibuffer_frame () -> *mut frame
{
  /* First make a frame containing just a root window, no minibuffer.  */

  let f: *mut frame = make_frame (false);
  let mut mini_window: Lisp_Object;
  let mut frame: Lisp_Object = Qnil ();

  XSETFRAME! (frame, f);

  auto_raise! (f, 0);
  auto_lower! (f, 0);
  no_split! (f, 1);
  wants_modeline! (f, 0);

  /* Now label the root window as also being the minibuffer.
     Avoid infinite looping on the window chain by marking next pointer
     as nil. */

  mini_window = (*f).root_window;
  fset_minibuffer_window (f, mini_window);
  store_frame_param (f, Qminibuffer (), Qonly ());
  mini! (XWINDOW (mini_window), 1);
  wset_next (XWINDOW (mini_window), Qnil ());
  wset_prev (XWINDOW (mini_window), Qnil ());
  wset_frame (XWINDOW (mini_window), frame);

  /* Put the proper buffer in that window.  */

  /* Use set_window_buffer instead of Fset_window_buffer (see
     discussion of bug#11984, bug#12025, bug#12026).  */
  set_window_buffer (mini_window,
		     if NILP (Vminibuffer_list) { get_minibuffer (0) }
		     else { Fcar (Vminibuffer_list) }, false, false);
  return f;
}

/* Construct a frame that refers to a terminal.  */

static mut tty_frame_count: c_int = 0;

unsafe fn frame_next_F_name () -> Lisp_Object
{
  let mut string_name: [c_char; 24] = [0; 24];
  let mut list: Lisp_Object = Qnil ();
  let mut frame: Lisp_Object = Qnil ();

 'next_name: loop {
  sprintf (&raw mut string_name as *mut c_char, c"F%jd".as_ptr (),
	   { tty_frame_count += 1; tty_frame_count as intmax_t });
  FOR_EACH_FRAME! (list, frame, {
    if !NILP ((*XFRAME (frame)).name)
	&& strcmp (&raw const string_name as *const c_char,
		   SSDATA ((*XFRAME (frame)).name)) == 0
      { continue 'next_name; }
  });
  break;
 }
  return build_string (&raw const string_name as *const c_char);
}

/* dispextern.h 1932-1940: the three terminal "default colour" sentinels (no mirror carries
   them; the term.rs/terminal.rs shape).  */
pub const FACE_TTY_DEFAULT_COLOR: c_ulong = -1i64 as c_ulong;
pub const FACE_TTY_DEFAULT_FG_COLOR: c_ulong = -2i64 as c_ulong;
pub const FACE_TTY_DEFAULT_BG_COLOR: c_ulong = -3i64 as c_ulong;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn make_initial_frame () -> *mut frame
{
  let f: *mut frame;
  let terminal: *mut terminal;
  let mut frame: Lisp_Object = Qnil ();

  eassert! (!initial_kboard.is_null ());
  eassert! (NILP (Vframe_list) || CONSP (Vframe_list));

  terminal = init_initial_terminal ();

  f = make_frame (true);
  XSETFRAME! (frame, f);
  frame_set_id (f, 0);

  Vframe_list = Fcons (frame, Vframe_list);

  tty_frame_count = 1;
  fset_name (f, build_string (c"F1".as_ptr ()));

  SET_FRAME_VISIBLE! (f, true);

  output_method! (f, (*terminal).r#type);
  (*f).terminal = terminal;
  (*(*f).terminal).reference_count += 1;

  FRAME_FOREGROUND_PIXEL! (f) = FACE_TTY_DEFAULT_FG_COLOR;
  FRAME_BACKGROUND_PIXEL! (f) = FACE_TTY_DEFAULT_BG_COLOR;

  /* C 1452-1455's `#ifdef HAVE_WINDOW_SYSTEM` arm is dead (P-08).  */

  /* The default value of menu-bar-mode is t.  */
  set_menu_bar_lines (f, make_fixnum (1), Qnil ());

  /* The default value of tab-bar-mode is nil.  */
  set_tab_bar_lines (f, make_fixnum (0), Qnil ());

  /* Allocate glyph matrices.  */
  adjust_frame_glyphs (f);

  if !noninteractive
    { init_frame_faces (f); }

  last_nonminibuf_frame = f;

  can_set_window_size! (f, true);
  after_make_frame! (f, true);

  return f;
}

/* C 1477-1479's `#ifndef HAVE_ANDROID` is live (`HAVE_ANDROID` is undef).  */

unsafe fn make_terminal_frame (terminal: *mut terminal, parent: Lisp_Object,
		     params: Lisp_Object) -> *mut frame
{
  if (*terminal).name.is_null ()
    { error (c"Terminal is not live, can't create new frames on it".as_ptr ()); }

  let mut f: *mut frame;

  if NILP (parent)
    { f = make_frame (true); }
  else
    {
      CHECK_LIVE_FRAME! (parent);

      f = std::ptr::null_mut ();
      let mut mini: Lisp_Object = Fassq (Qminibuffer (), params);

      /* Handling the minibuffer parameter on a tty is different from
	 its handling on a GUI.  On a GUI any "client frame" can have,
	 in principle, its minibuffer window on any "minibuffer frame" -
	 a frame that has a minibuffer window.  If necessary, Emacs
	 tells the window manager to make the minibuffer frame visible,
	 raise it and give it input focus.

	 On a tty there's no window manager; so Emacs itself has to make
	 such a minibuffer frame visible, raise and focus it.  Since a
	 tty can show only one root frame (a frame that doesn't have a
	 parent frame) at any time, any client frame shown on a tty must
	 have a minibuffer frame whose root frame is the root frame of
	 that client frame.  If that minibuffer frame is a child frame,
	 Emacs will automatically make it visible, raise it and give it
	 input focus, if necessary.

	 Two trivial consequences of these observations for ttys are:

	 - A root frame cannot be the minibuffer frame of another root
	   frame.

	 - Since a child frame cannot be created before its parent
           frame, each root frame must have its own minibuffer window.

         The situation may change as soon as we can delete and create
         minibuffer windows on the fly.  */
      if CONSP (mini)
	{
	  mini = Fcdr (mini);

	  if EQ (mini, Qnone ()) || NILP (mini)
	    {
	      mini = (*root_frame (XFRAME (parent))).minibuffer_window;
	      f = make_frame (false);
	      fset_minibuffer_window (f, mini);
	      store_frame_param (f, Qminibuffer (), mini);
	    }
	  else if EQ (mini, Qonly ())
	    { f = make_minibuffer_frame (); }
	  else if WINDOWP (mini)
	    {
	      if !WINDOW_LIVE_P (mini)
		  || !MINI_WINDOW_P (XWINDOW (mini))
		  || (root_frame (WINDOW_XFRAME! (XWINDOW (mini)))
		      != root_frame (XFRAME (parent)))
		{ error (c"The `minibuffer' parameter does not specify a valid minibuffer window".as_ptr ()); }
	      else
		{
		  f = make_frame (false);
		  fset_minibuffer_window (f, mini);
		  store_frame_param (f, Qminibuffer (), mini);
		}
	    }
	}

      if f.is_null ()
	{ f = make_frame (true); }
      (*f).parent_frame = parent;
      (*f).z_order = 1 + max_child_z_order (XFRAME (parent));
    }

  let mut frame: Lisp_Object = Qnil ();
  XSETFRAME! (frame, f);
  Vframe_list = Fcons (frame, Vframe_list);

  fset_name (f, frame_next_F_name ());

  SET_FRAME_VISIBLE! (f, true);

  (*f).terminal = terminal;
  (*(*f).terminal).reference_count += 1;
  /* C 1568-1582's `#ifdef MSDOS`/`#else` is the live `#else` arm (MSDOS is undef).  */
  output_method! (f, output_termcap);
  create_tty_output (f);
  FRAME_FOREGROUND_PIXEL! (f) = FACE_TTY_DEFAULT_FG_COLOR;
  FRAME_BACKGROUND_PIXEL! (f) = FACE_TTY_DEFAULT_BG_COLOR;

  let tty: *mut tty_display_info = (*terminal).display_info.tty.cast::<tty_display_info> ();

  if NILP ((*tty).top_frame)
    /* If this frame's terminal's top frame has not been set up yet,
       make the new frame its top frame so the top frame has been set up
       before the first do_switch_frame on this terminal happens.  See
       Bug#78966.  */
    { (*tty).top_frame = frame; }

  /* C 1593-1596's `#ifdef HAVE_WINDOW_SYSTEM` arm is dead (P-08).  */

  /* Menu bars on child frames don't work on all platforms, which is
     the reason why prepare_menu_bar does not update_menu_bar for
     child frames (info from Martin Rudalics).  This could be
     implemented in ttys, but it's unclear if it is worth it.  */
  if NILP (parent)
    { FRAME_MENU_BAR_LINES! (f) = if NILP (Vmenu_bar_mode) { 0 } else { 1 }; }
  else
    { FRAME_MENU_BAR_LINES! (f) = 0; }

  FRAME_TAB_BAR_LINES! (f) = if NILP (Vtab_bar_mode) { 0 } else { 1 };
  FRAME_LINES! (f) = FRAME_LINES! (f) - FRAME_MENU_BAR_LINES! (f)
    - FRAME_TAB_BAR_LINES! (f);
  FRAME_MENU_BAR_HEIGHT! (f) = FRAME_MENU_BAR_LINES! (f) * FRAME_LINE_HEIGHT (f);
  FRAME_TAB_BAR_HEIGHT! (f) = FRAME_TAB_BAR_LINES! (f) * FRAME_LINE_HEIGHT (f);
  FRAME_TEXT_HEIGHT! (f) = FRAME_TEXT_HEIGHT! (f) - FRAME_MENU_BAR_HEIGHT! (f)
    - FRAME_TAB_BAR_HEIGHT! (f);

  return f;
}

/* Get a suitable value for frame parameter PARAMETER for a newly
   created frame, based on (1) the user-supplied frame parameter
   alist SUPPLIED_PARMS, and (2) CURRENT_VALUE.  */

unsafe fn get_future_frame_param (parameter: Lisp_Object,
                        supplied_parms: Lisp_Object,
                        current_value: *mut c_char) -> Lisp_Object
{
  let mut result: Lisp_Object;

  result = Fassq (parameter, supplied_parms);
  if NILP (result)
    { result = Fassq (parameter, (*XFRAME (selected_frame)).param_alist); }
  if NILP (result) && !current_value.is_null ()
    { result = build_string (current_value); }
  if !NILP (result) && !STRINGP (result)
    { result = XCDR (result); }
  if NILP (result) || !STRINGP (result)
    { result = Qnil (); }

  return result;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tty_child_pos_param (f: *mut frame, key: Lisp_Object,
		     params: Lisp_Object, pos: c_int, size: c_int) -> c_int
{
  let p: *mut frame = XFRAME ((*f).parent_frame);
  let mut val: Lisp_Object = Fassq (key, params);
  let mut pos = pos;

  if CONSP (val)
    {
      val = XCDR (val);

      if EQ (val, Qminus ())
	{ pos = if EQ (key, Qtop ())
	       { (*p).pixel_height - size }
	       else { (*p).pixel_width - size }; }
      else if TYPE_RANGED_FIXNUMP (INT_MIN as EMACS_INT, val, INT_MAX as EMACS_INT)
	{
	  pos = XFIXNUM (val) as c_int;

	  if pos < 0
	    /* Handle negative value. */
	    { pos = if EQ (key, Qtop ())
		   { (*p).pixel_height - size + pos }
		   else { (*p).pixel_width - size + pos }; }
	}
      else if CONSP (val) && EQ (XCAR (val), Qplus ())
	       && CONSP (XCDR (val))
	       && TYPE_RANGED_FIXNUMP (INT_MIN as EMACS_INT, XCAR (XCDR (val)),
				       INT_MAX as EMACS_INT)
	{ pos = XFIXNUM (XCAR (XCDR (val))) as c_int; }
      else if CONSP (val) && EQ (XCAR (val), Qminus ())
	       && CONSP (XCDR (val))
	       && RANGED_FIXNUMP (-(INT_MAX as EMACS_INT), XCAR (XCDR (val)),
				  INT_MAX as EMACS_INT)
	{ pos = if EQ (key, Qtop ())
	       { (*p).pixel_height - size - XFIXNUM (XCAR (XCDR (val))) as c_int }
	       else { (*p).pixel_width - size - XFIXNUM (XCAR (XCDR (val))) as c_int }; }
    }

  return pos;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tty_child_size_param (child: *mut frame, key: Lisp_Object,
		      params: Lisp_Object, dflt: c_int) -> c_int
{
  let mut val: Lisp_Object = Fassq (key, params);
  if CONSP (val)
    {
      val = XCDR (val);
      if CONSP (val)
	{
	  /* Width and height may look like (width text-pixels . PIXELS)
	     on window systems.  Mimic that.  */
	  val = XCDR (val);
	  if EQ (val, Qtext_pixels ())
	    { val = XCDR (val); }
	}
      else if FLOATP (val)
	{
	  /* Width and height may be a float, in which case
	     it's a multiple of the parent's value.  */
	  let parent: *mut frame = FRAME_PARENT_FRAME (child);
	  eassert! (!parent.is_null ());	/* the caller ensures this, but... */
	  if !parent.is_null ()
	    {
	      let sz: c_int = if EQ (key, Qwidth ()) { FRAME_TOTAL_COLS! (parent) }
			      else { FRAME_TOTAL_LINES! (parent) };
	      val = make_fixnum ((XFLOAT_DATA (val) * sz as c_double) as EMACS_INT);
	    }
	  else
	    { val = Qnil (); }
	}

      if FIXNATP (val)
	{ return XFIXNUM (val) as c_int; }
    }
  return dflt;
}

/* C 1722-1734's `#ifndef HAVE_ANDROID` is live.  */

unsafe fn tty_child_frame_rect (f: *mut frame, params: Lisp_Object,
		      x: *mut c_int, y: *mut c_int, w: *mut c_int, h: *mut c_int)
{
  *w = tty_child_size_param (f, Qwidth (), params, FRAME_TOTAL_COLS! (f));
  *h = tty_child_size_param (f, Qheight (), params, FRAME_TOTAL_LINES! (f));
  *x = tty_child_pos_param (f, Qleft (), params, 0, *w);
  *y = tty_child_pos_param (f, Qtop (), params, 0, *h);
}

DEFUN! ("make-terminal-frame", Fmake_terminal_frame, Smake_terminal_frame,
       1, 1, 0,
       doc: "Create an additional terminal frame, possibly on another terminal.\nThis function takes one argument, an alist specifying frame parameters.\n\nYou can create multiple frames on a single text terminal, but only one\nof them (the selected terminal frame) is actually displayed.\n\nIn practice, generally you don't need to specify any parameters,\nexcept when you want to create a new frame on another terminal.\nIn that case, the `tty' parameter specifies the device file to open,\nand the `tty-type' parameter specifies the terminal type.  Example:\n\n   (make-terminal-frame \\='((tty . \"/dev/pts/5\") (tty-type . \"xterm\")))\n\nNote that changing the size of one terminal frame automatically\naffects all frames on the same terminal device.  ",
       (parms: Lisp_Object),
       {
  /* C 1755-1758: the `#ifdef HAVE_ANDROID` arm (the platform has no text terminals); everything
     below is C's `#else`, which Android does not compile (status 4.714).  */
  #[cfg (feature = "have_android")]
  { error (c"Text terminals are not supported on this platform".as_ptr ()); }
  #[cfg (not (feature = "have_android"))]
  {
  let mut parms = parms;
  let mut t: *mut terminal = std::ptr::null_mut ();
  let sf: *mut frame = SELECTED_FRAME ();

  /* C 1762-1771's `#ifdef MSDOS`/`WINDOWSNT` arms are dead.  */

  {
    let mut terminal: Lisp_Object;

    terminal = Fassq (Qterminal (), parms);
    if CONSP (terminal)
      {
        terminal = XCDR (terminal);
        t = decode_live_terminal (terminal);
      }
    /* C 1782-1788's `#ifdef MSDOS` arm is dead.  */
  }

  if t.is_null ()
    {
      let mut name: *mut c_char = std::ptr::null_mut ();
      let mut r#type: *mut c_char = std::ptr::null_mut ();
      let tty: Lisp_Object;
      let tty_type: Lisp_Object;
      USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);

      tty = get_future_frame_param
        (Qtty (), parms, if FRAME_TERMCAP_P (XFRAME (selected_frame))
                       { (*FRAME_TTY! (XFRAME (selected_frame))).name }
                       else { std::ptr::null_mut () });
      if !NILP (tty)
	{ SAFE_ALLOCA_STRING! (sa_avail, name, tty); }

      tty_type = get_future_frame_param
        (Qtty_type (), parms, if FRAME_TERMCAP_P (XFRAME (selected_frame))
                            { (*FRAME_TTY! (XFRAME (selected_frame))).r#type }
                            else { std::ptr::null_mut () });
      if !NILP (tty_type)
	{ SAFE_ALLOCA_STRING! (sa_avail, r#type, tty_type); }

      t = init_tty (name, r#type, false); /* Errors are not fatal.  */
      SAFE_FREE! (sa_count, sa_mark);
    }

  /* Make a new frame.  We need to know up front if a parent frame is
     specified because we behave differently in this case, e.g., child
     frames don't obscure other frames.  */
  let parent: Lisp_Object = Fcdr (Fassq (Qparent_frame (), parms));
  let f: *mut frame = make_terminal_frame (t, parent, parms);
  frame_set_id_from_params (f, parms);

  if !noninteractive
    { init_frame_faces (f); }

  /* Visibility of root frames cannot be set with a frame parameter.
     Their visibility solely depends on whether or not they are the
     top_frame on the terminal.  */
  if !FRAME_PARENT_FRAME (f).is_null ()
    {
      let visible: Lisp_Object = Fassq (Qvisibility (), parms);
      if CONSP (visible)
	{ SET_FRAME_VISIBLE! (f, !NILP (visible)); }

      /* FIXME/tty: The only way, for now, to get borders on a tty is
	 to allow decorations.  */
      let undecorated: Lisp_Object = Fassq (Qundecorated (), parms);
      if CONSP (undecorated) && !NILP (XCDR (undecorated))
	{ undecorated! (f, true); }

      /* Unused at present.  */
      let no_focus: Lisp_Object = Fassq (Qno_accept_focus (), parms);
      if CONSP (no_focus) && !NILP (XCDR (no_focus))
	{ no_accept_focus! (f, true); }

      let no_split: Lisp_Object = Fassq (Qunsplittable (), parms);
      if CONSP (no_split) && !NILP (XCDR (no_split))
	{ no_split! (f, true); }
    }

  /* Determine width and height of the frame.  For root frames use the
     width/height of the terminal.  For child frames, take it from frame
     parameters.  Note that a default (80x25) has been set in
     make_frame.  We handle root frames in this way because otherwise we
     would end up needing glyph matrices for the terminal, which is both
     more work and has its downsides (think of clipping frames to the
     terminal size).  */
  let mut x: c_int = 0;
  let mut y: c_int = 0;
  let mut width: c_int = 0;
  let mut height: c_int = 0;
  if !FRAME_PARENT_FRAME (f).is_null ()
    { tty_child_frame_rect (f, parms, &mut x, &mut y, &mut width, &mut height); }
  else
    { get_tty_size (fileno ((*FRAME_TTY! (f)).input), &mut width, &mut height); }
  adjust_frame_size (f, width, height - FRAME_TOP_MARGIN (f), 5, false,
		     Qterminal_frame ());
  adjust_frame_glyphs (f);

  calculate_costs (f);

  (*f).left_pos = x;
  (*f).top_pos = y;

  store_in_alist (&mut parms, Qtty_type (),
		  build_string ((*((*t).display_info.tty).cast::<tty_display_info> ()).r#type));
  store_in_alist (&mut parms, Qtty (),
		  if !(*((*t).display_info.tty).cast::<tty_display_info> ()).name.is_null ()
		  { build_string ((*((*t).display_info.tty).cast::<tty_display_info> ()).name) }
		  else { Qnil () });

  /* Make the frame face hash be frame-specific, so that each
     frame could change its face definitions independently.  */
  fset_face_hash_table (f, Fcopy_hash_table ((*sf).face_hash_table));
  /* Simple copy_hash_table isn't enough, because we need the contents of
     the vectors which are the values in face_hash_table to
     be copied as well.  */
  let mut idx: ptrdiff_t = 0;
  let table: *mut Lisp_Hash_Table = XHASH_TABLE ((*f).face_hash_table);
  while idx < (*table).count as ptrdiff_t
    {
      set_hash_value_slot (table, idx as hash_idx_t, Fcopy_sequence (HASH_VALUE (table, idx as hash_idx_t)));
      idx += 1;
    }

  /* On terminal frames the `minibuffer' frame parameter is always
     virtually t.  Avoid that a different value in parms causes
     complaints, see Bug#24758.  */
  if FRAME_PARENT_FRAME (f).is_null ()
    { store_in_alist (&mut parms, Qminibuffer (), Qt ()); }

  let mut frame: Lisp_Object = Qnil ();
  XSETFRAME! (frame, f);
  Fmodify_frame_parameters (frame, parms);

  can_set_window_size! (f, true);
  after_make_frame! (f, true);

  return frame;
  }
});

/* Perform the switch to frame FRAME.

   If FRAME is a switch-frame event `(switch-frame FRAME1)', use
   FRAME1 as frame.

   If TRACK is non-zero and the frame that currently has the focus
   redirects its focus to the selected frame, redirect that focused
   frame's focus to FRAME instead.

   FOR_DELETION non-zero means that the selected frame is being
   deleted, which includes the possibility that the frame's terminal
   is dead.

   The value of NORECORD is passed as argument to Fselect_window.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_switch_frame (frame: Lisp_Object, track: c_int,
                                          for_deletion: c_int,
                                          norecord: Lisp_Object) -> Lisp_Object
{
  let mut frame = frame;
  /* If FRAME is a switch-frame event, extract the frame we should
     switch to.  */
  if CONSP (frame)
      && EQ (XCAR (frame), Qswitch_frame ())
      && CONSP (XCDR (frame))
    { frame = XCAR (XCDR (frame)); }

  /* This used to say CHECK_LIVE_FRAME, but apparently it's possible for
     a switch-frame event to arrive after a frame is no longer live,
     especially when deleting the initial frame during startup.  */
  CHECK_FRAME! (frame);
  let f: *mut frame = XFRAME (frame);
  let sf: *mut frame = SELECTED_FRAME ();

  /* Silently ignore dead and tooltip frames (Bug#47207).  */
  if !FRAME_LIVE_P (f) || FRAME_TOOLTIP_P (f)
    { return Qnil (); }
  else if f == sf
    { return frame; }

  /* If the frame with GUI focus has had it's Emacs focus redirected
     toward the currently selected frame, we should change the
     redirection to point to the newly selected frame.  This means
     that if the focus is redirected from a minibufferless frame to a
     surrogate minibuffer frame, we can use `other-window' to switch
     between all the frames using that minibuffer frame, and the focus
     redirection will follow us around.  This code is necessary when
     we have a minibufferless frame using the MB in another (normal)
     frame (bug#64152) (ACM, 2023-06-20).  */
  /* C 1953-1980's `#ifdef HAVE_WINDOW_SYSTEM` arm is dead (P-08).  */

  if !(for_deletion != 0) && FRAME_HAS_MINIBUF_P (sf)
    { resize_mini_window (XWINDOW (FRAME_MINIBUF_WINDOW (sf)), true); }

  if is_tty_frame (f)
    {
      let tty: *mut tty_display_info = FRAME_TTY! (f);
      let mut top_frame: Lisp_Object = (*tty).top_frame;

      /* When FRAME's root frame is not its terminal's top frame, make
	 that root frame the new top frame of FRAME's terminal.  */
      if NILP (top_frame) || root_frame (f) != XFRAME (top_frame)
	{
	  let mut p: *mut frame = FRAME_PARENT_FRAME (f);

	  XSETFRAME! (top_frame, root_frame (f));
	  (*tty).top_frame = top_frame;
	  SET_FRAME_VISIBLE! (root_frame (f), true);

	  while !p.is_null ()
	    {
	      /* If FRAME is a child frame, make it redraw.  */
	      SET_FRAME_GARBAGED! (p);
	      p = FRAME_PARENT_FRAME (p);
	    }

	  /* ... and FRAME itself too.  */
	  SET_FRAME_GARBAGED! (f);

	  /* FIXME: Why is it correct to set FrameCols/Rows here?  */
	  if FRAME_PARENT_FRAME (f).is_null ()
	    {
	      /* If the new TTY frame changed dimensions, we need to
		 resync term.c's idea of the frame size with the new
		 frame's data.  */
	      if FRAME_COLS! (f) != FrameCols! (tty)
		{ FrameCols! (tty) = FRAME_COLS! (f); }
	      if FRAME_TOTAL_LINES! (f) != FrameRows! (tty)
		{ FrameRows! (tty) = FRAME_TOTAL_LINES! (f); }
	    }
	}
    }

  select_mini_window_flag! (sf, MINI_WINDOW_P (XWINDOW ((*sf).selected_window)));

  move_minibuffers_onto_frame (sf, frame, for_deletion != 0);

  /* If the selected window in the target frame is its mini-window, we move
     to a different window, the most recently used one, unless there is a
     valid active minibuffer in the mini-window.  */
  if EQ ((*f).selected_window, (*f).minibuffer_window)
      /* The following test might fail if the mini-window contains a
	 non-active minibuffer.  */
      && NILP (Fminibufferp ((*XWINDOW ((*f).minibuffer_window)).contents, Qt ()))
    {
      let w: Lisp_Object = calln! (Qget_mru_window (), frame);
      if WINDOW_LIVE_P (w) /* W can be nil in minibuffer-only frames.  */
        { Fset_frame_selected_window (frame, w, Qnil ()); }
    }

  /* After setting `selected_frame`, we're temporarily in an inconsistent
     state where (selected-window) != (frame-selected-window).  Until this
     invariant is restored we should be very careful not to run any Lisp.
     (bug#58343)  */
  selected_frame = frame;

  if select_mini_window_flag! (f)
      && !NILP (Fminibufferp ((*XWINDOW ((*f).minibuffer_window)).contents, Qt ()))
    { fset_selected_window (f, (*f).minibuffer_window); }
  select_mini_window_flag! (f, false);

  if !FRAME_MINIBUF_ONLY_P (XFRAME (selected_frame))
    { last_nonminibuf_frame = XFRAME (selected_frame); }

  Fselect_window ((*f).selected_window, norecord);

  /* We want to make sure that the next event generates a frame-switch
     event to the appropriate frame.  This seems kludgy to me, but
     before you take it out, make sure that evaluating something like
     (select-window (frame-root-window (make-frame))) doesn't end up
     with your typing being interpreted in the new frame instead of
     the one you're actually typing in.  */

  /* FIXME/tty: I don't understand this.  (The comment above is from
     Jim Blandy 1993 BTW, and the frame_ancestor_p from 2017.)

     Setting the last event frame to nil leads to switch-frame events
     being generated even if they normally wouldn't be because the frame
     in question equals selected-frame.  See the places in keyboard.c
     where make_lispy_switch_frame is called.

     This leads to problems at least on ttys.

     Imagine that we have functions in post-command-hook that use
     select-frame in some way (e.g., with-selected-window).  Let these
     functions select different frames during the execution of
     post-command-hook in command_loop_1.  Setting
     internal_last_event_frame to nil here makes these select-frame
     calls (potentially and in reality) generate switch-frame events.
     (But only in one direction (frame_ancestor_p), which I also don't
     understand).

     These switch-frame events form an endless loop in
     command_loop_1.  It runs post-command-hook, which generates
     switch-frame events, which command_loop_1 finds (bound to '#ignore)
     and executes, which again runs post-command-hook etc., ad
     infinitum.

     Let's not do that for now on ttys.  */
  if !is_tty_frame (f)
    { if !frame_ancestor_p (f, sf)
      { internal_last_event_frame = Qnil (); } }

  return frame;
}

DEFUN! ("select-frame", Fselect_frame, Sselect_frame, 1, 2, "e",
       doc: "Select FRAME.\nSubsequent editing commands apply to its selected window.\nOptional argument NORECORD means to neither change the order of\nrecently selected windows nor the buffer list.\n\nThe selection of FRAME lasts until the next time the user does\nsomething to select a different frame, or until the next time\nthis function is called.  If you are using a window system, the\npreviously selected frame may be restored as the selected frame\nwhen returning to the command loop, because it still may have\nthe window system's input focus.  On a text terminal, the next\nredisplay will display FRAME.\n\nThis function returns FRAME, or nil if FRAME has been deleted.  ",
       (frame: Lisp_Object, norecord: Lisp_Object),
       {
  let f: *mut frame;

  CHECK_LIVE_FRAME! (frame);
  f = XFRAME (frame);

  if FRAME_TOOLTIP_P (f)
    /* Do not select a tooltip frame (Bug#47207).  */
    { error (c"Cannot select a tooltip frame".as_ptr ()); }
  else
    { return do_switch_frame (frame, 1, 0, norecord); }
});

DEFUN! ("handle-switch-frame", Fhandle_switch_frame,
       Shandle_switch_frame, 1, 1, "^e",
       doc: "Handle a switch-frame event EVENT.\nSwitch-frame events are usually bound to this function.\nA switch-frame event is an event Emacs sends itself to\nindicate that input is arriving in a new frame. It does not\nnecessarily represent user-visible input focus.  ",
       (event: Lisp_Object),
       {
  /* Preserve prefix arg that the command loop just cleared.  */
  kset_prefix_arg (current_kboard, Vcurrent_prefix_arg);
  run_hook (Qmouse_leave_buffer_hook ());

  return do_switch_frame (event, 0, 0, Qnil ());
});

DEFUN! ("selected-frame", Fselected_frame, Sselected_frame, 0, 0, 0,
       doc: "Return the frame that is now selected.  ",
       (),
       {
  return selected_frame;
});

DEFUN! ("old-selected-frame", Fold_selected_frame,
       Sold_selected_frame, 0, 0, 0,
       doc: "Return the old selected FRAME.\nFRAME must be a live frame and defaults to the selected one.\n\nThe return value is the frame selected the last time window change\nfunctions were run.  ",
       (),
       {
  return old_selected_frame;
});

DEFUN! ("frame-list", Fframe_list, Sframe_list,
       0, 0, 0,
       doc: "Return a list of all live frames.\nThe return value does not include any tooltip frame.  ",
       (),
       {
  /* C 2167-2175's `#ifdef HAVE_WINDOW_SYSTEM` arm is dead (P-08).  */
  return Fcopy_sequence (Vframe_list);
});

DEFUN! ("frame-parent", Fframe_parent, Sframe_parent,
       0, 1, 0,
       doc: "Return the parent frame of FRAME.\nThe parent frame of FRAME is the Emacs frame whose window-system window\nis the parent window of FRAME's window-system window.  When such a frame\nexists, FRAME is considered a child frame of that frame.\n\nReturn nil if FRAME has no parent frame.  This means that FRAME's\nwindow-system window is either a \"top-level\" window (a window whose\nparent window is the window-system's root window) or an embedded window\n(a window whose parent window is owned by some other application).  ",
     (frame: Lisp_Object),
     {
  let f: *mut frame = decode_live_frame (frame);
  let p: *mut frame = FRAME_PARENT_FRAME (f);
  let mut parent: Lisp_Object = Qnil ();

  /* Can't return f->parent_frame directly since it might not be defined
     for this platform.  */
  if !p.is_null ()
    {
      XSETFRAME! (parent, p);

      return parent;
    }
  else
    { return Qnil (); }
});

/* Return true if frame AF is an ancestor of frame DF.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_ancestor_p (af: *mut frame, df: *mut frame) -> bool
{
  let mut pf: *mut frame = FRAME_PARENT_FRAME (df);

  while !pf.is_null ()
    {
      if pf == af
	{ return true; }
      else
	{ pf = FRAME_PARENT_FRAME (pf); }
    }

  return false;
}

/* A frame AF subsumes a frame DF if AF and DF are the same or AF is an
   ancestor of DF.  */
unsafe fn frame_subsumes_p (af: *mut frame, df: *mut frame) -> bool
{
  let mut df = df;
  while !df.is_null ()
    {
      if df == af
	{ return true; }
      else
	{ df = FRAME_PARENT_FRAME (df); }
    }

  return false;
}

DEFUN! ("frame-ancestor-p", Fframe_ancestor_p, Sframe_ancestor_p,
       2, 2, 0,
       doc: "Return non-nil if ANCESTOR is an ancestor of DESCENDANT.\nANCESTOR is an ancestor of DESCENDANT when it is either DESCENDANT's\nparent frame or it is an ancestor of DESCENDANT's parent frame.  Both,\nANCESTOR and DESCENDANT must be live frames and default to the selected\nframe.  ",
     (ancestor: Lisp_Object, descendant: Lisp_Object),
     {
  let af: *mut frame = decode_live_frame (ancestor);
  let df: *mut frame = decode_live_frame (descendant);
  return if frame_ancestor_p (af, df) { Qt () } else { Qnil () };
});

/* Return the root frame of frame F.  Follow the parent_frame chain
   until we reach a frame that has no parent.  That is the root frame.
   Note that the root of a root frame is itself. */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_frame (f: *mut frame) -> *mut frame
{
  let mut f = f;
  while !FRAME_PARENT_FRAME (f).is_null ()
    { f = FRAME_PARENT_FRAME (f); }
  return f;
}

DEFUN! ("frame-root-frame", Fframe_root_frame, Sframe_root_frame,
       0, 1, 0,
       doc: "Return root frame of specified FRAME.\nFRAME must be a live frame and defaults to the selected one.  The root\nframe of FRAME is the frame obtained by following the chain of parent\nframes starting with FRAME until a frame is reached that has no parent.\nIf FRAME has no parent, its root frame is FRAME.  ",
     (frame: Lisp_Object),
     {
  let f: *mut frame = decode_live_frame (frame);
  let r: *mut frame = root_frame (f);
  let mut root: Lisp_Object = Qnil ();

  XSETFRAME! (root, r);

  return root;
});

/* Return CANDIDATE if it can be used as 'other-than-FRAME' frame on the
   same tty (for tty frames) or among frames which uses FRAME's keyboard.
   If MINIBUF is nil, do not consider minibuffer-only candidate.
   If MINIBUF is `visible', do not consider an invisible candidate.
   If MINIBUF is a window, consider only its own frame and candidate now
   using that window as the minibuffer.
   If MINIBUF is 0, consider candidate if it is visible or iconified.
   Otherwise consider any candidate and return nil if CANDIDATE is not
   acceptable.  */

unsafe fn candidate_frame (candidate: Lisp_Object, frame: Lisp_Object, minibuf: Lisp_Object)
  -> Lisp_Object
{
  let c: *mut frame = XFRAME (candidate);
  let f: *mut frame = XFRAME (frame);

  if (!FRAME_TERMCAP_P (c) && !FRAME_TERMCAP_P (f)
       && FRAME_KBOARD (c) == FRAME_KBOARD (f))
      || (FRAME_TERMCAP_P (c) && FRAME_TERMCAP_P (f)
	  && FRAME_TTY! (c) == FRAME_TTY! (f))
    {
      if !NILP (get_frame_param (c, Qno_other_frame ()))
	{ return Qnil (); }
      else if NILP (minibuf)
	{
	  if !FRAME_MINIBUF_ONLY_P (c)
	    { return candidate; }
	}
      else if EQ (minibuf, Qvisible ())
	{
	  if FRAME_VISIBLE_P (c)
	    { return candidate; }
	}
      else if WINDOWP (minibuf)
	{
	  if EQ (FRAME_MINIBUF_WINDOW (c), minibuf)
	      || EQ (WINDOW_FRAME! (XWINDOW (minibuf)), candidate)
	      || EQ (WINDOW_FRAME! (XWINDOW (minibuf)),
		     FRAME_FOCUS_FRAME (c))
	    { return candidate; }
	}
      else if FIXNUMP (minibuf) && XFIXNUM (minibuf) == 0
	{
	  if FRAME_VISIBLE_P (c) || FRAME_ICONIFIED_P (c)
	    { return candidate; }
	}
      else
	{ return candidate; }
    }
  return Qnil ();
}

/* Return the next frame in the frame list after FRAME.  */

unsafe fn next_frame (frame: Lisp_Object, minibuf: Lisp_Object) -> Lisp_Object
{
  let mut f: Lisp_Object = Qnil ();
  let mut tail: Lisp_Object = Qnil ();
  let mut next: Lisp_Object = Qnil ();
  let mut passed: bool = false;

  eassume! (CONSP (Vframe_list));

  FOR_EACH_FRAME! (tail, f, {
      if EQ (f, frame)
	/* If we encounter FRAME, set PASSED to true.  */
	{ passed = true; }
      else
	{
	  f = candidate_frame (f, frame, minibuf);

	  if !NILP (f)
	    {
	      if passed
		/* If we passed FRAME already, return first suitable
		   candidate following it.  */
		{ return f; }
	      else if NILP (next)
		/* If we didn't pass FRAME and have no suitable
		   candidate yet, set NEXT to the first suitable
		   candidate preceding FRAME.  */
		{ next = f; }
	    }
	}
  });

  /* We have scanned all frames.  Return first candidate preceding FRAME
     if we have found one.  Otherwise return FRAME regardless of whether
     it is a suitable candidate or not.  */
  return if NILP (next) { frame } else { next };
}

/* Return the previous frame in the frame list before FRAME.  */

unsafe fn prev_frame (frame: Lisp_Object, minibuf: Lisp_Object) -> Lisp_Object
{
  let mut f: Lisp_Object = Qnil ();
  let mut tail: Lisp_Object = Qnil ();
  let mut prev: Lisp_Object = Qnil ();

  eassume! (CONSP (Vframe_list));

  FOR_EACH_FRAME! (tail, f, {
      if EQ (frame, f) && !NILP (prev)
	/* If we encounter FRAME and already have found a suitable
	   candidate preceding it, return that candidate.  */
	{ return prev; }

      f = candidate_frame (f, frame, minibuf);

      if !NILP (f)
	/* PREV is always the last suitable candidate we found.  */
	{ prev = f; }
  });

  /* We've scanned the entire list.  */
  if NILP (prev)
    /* We went through the whole frame list without finding a single
       acceptable frame.  Return FRAME.  */
    { return frame; }
  else
    /* There were no acceptable frames in the list before FRAME;
       otherwise, we would have returned directly from the loop.  Since
       PREV is the last suitable frame in the list, return it.  */
    { return prev; }
}

DEFUN! ("next-frame", Fnext_frame, Snext_frame, 0, 2, 0,
       doc: "Return the next frame in the frame list after FRAME.\nOnly frames on the same terminal as FRAME are included in the list\nof candidate frames.  FRAME defaults to the selected frame.\n\nIf MINIFRAME is nil (the default), include all frames except\nminibuffer-only frames.\n\nIf MINIFRAME is a window, include only its own frame and any frame now\nusing that window as the minibuffer.\n\nIf MINIFRAME is `visible', include only visible frames.\n\nIf MINIFRAME is 0, include only visible and iconified frames.\n\nIf MINIFRAME is any other value, include all frames.\n\nReturn FRAME if no suitable next frame is found.  ",
       (frame: Lisp_Object, miniframe: Lisp_Object),
       {
  let mut frame = frame;
  if NILP (frame)
    { frame = selected_frame; }
  CHECK_LIVE_FRAME! (frame);
  return next_frame (frame, miniframe);
});

DEFUN! ("previous-frame", Fprevious_frame, Sprevious_frame, 0, 2, 0,
       doc: "Return the previous frame in the frame list before FRAME.\nOnly frames on the same terminal as FRAME are included in the list\nof candidate frames.  FRAME defaults to the selected frame.\n\nIf MINIFRAME is nil (the default), include all frames except\nminibuffer-only frames.\n\nIf MINIFRAME is a window, include only its own frame and any frame now\nusing that window as the minibuffer.\n\nIf MINIFRAME is `visible', include only visible frames.\n\nIf MINIFRAME is 0, include only visible and iconified frames.\n\nIf MINIFRAME is any other value, include all frames.\n\nReturn FRAME if no suitable previous frame is found.  ",
       (frame: Lisp_Object, miniframe: Lisp_Object),
       {
  let mut frame = frame;
  if NILP (frame)
    { frame = selected_frame; }
  CHECK_LIVE_FRAME! (frame);
  return prev_frame (frame, miniframe);
});

DEFUN! ("last-nonminibuffer-frame", Flast_nonminibuf_frame,
       Slast_nonminibuf_frame, 0, 0, 0,
       doc: "Return last non-minibuffer frame selected. ",
       (),
       {
  let mut frame: Lisp_Object = Qnil ();

  if !last_nonminibuf_frame.is_null ()
    { XSETFRAME! (frame, last_nonminibuf_frame); }

  return frame;
});

/**
 * other_frames:
 *
 * Return true if there exists at least one visible or iconified frame
 * but F.  Tooltip and child frames do not qualify as candidates.
 * Return false if no such frame exists.
 *
 * INVISIBLE true means we are called from make_frame_invisible where
 * such a frame must be visible or iconified.  INVISIBLE nil means we
 * are called from delete_frame.  In that case FORCE true means that the
 * visibility status of such a frame can be ignored.
 *
 * If F is the terminal frame and we are using X, return true if at
 * least one X frame exists.
 */
unsafe fn other_frames (f: *mut frame, invisible: bool, force: bool) -> bool
{
  let mut frames: Lisp_Object = Qnil ();
  let mut frame: Lisp_Object = Qnil ();
  let mut frame1: Lisp_Object = Qnil ();
  let mut minibuffer_window: Lisp_Object = FRAME_MINIBUF_WINDOW (f);

  XSETFRAME! (frame, f);
  if WINDOWP (minibuffer_window)
      && !EQ (frame, WINDOW_FRAME! (XWINDOW (minibuffer_window)))
    { minibuffer_window = Qnil (); }

  FOR_EACH_FRAME! (frames, frame1, {
      let f1: *mut frame = XFRAME (frame1);

      if f != f1
	{
	  /* The following code is defined out because it is
	     responsible for a performance drop under X connections
	     over a network, and its purpose is unclear.  XSync does
	     not handle events (or call any callbacks defined by
	     Emacs), and as such it should not note any "recent change
	     in visibility".

	     When writing new code, please try as hard as possible to
	     avoid calls that require a roundtrip to the X server.
	     When such calls are inevitable, use the XCB library to
	     handle multiple consecutive requests with a data reply in
	     a more asynchronous fashion.  The following code
	     demonstrates why:

	       rc = XGetWindowProperty (dpyinfo->display, window, ...
	       status = XGrabKeyboard (dpyinfo->display, ...

	     here, `XGetWindowProperty' will wait for a reply from the
	     X server before returning, and thus allowing Emacs to
	     make the XGrabKeyboard request, which in itself also
	     requires waiting a reply.  When XCB is available, this
	     code could be written:

	     C 2535-2547's `#ifdef HAVE_XCB` block is a comment inside this comment.

	     In this code, the GetProperty and GrabKeyboard requests
	     are made simultaneously, and replies are then obtained
	     from the server at once, avoiding the extraneous
	     roundtrip to the X server after the call to
	     `XGetWindowProperty'.

	     However, please keep an alternative implementation
	     available for use when Emacs is built without XCB.  */

	  /* C 2558-2565's `#if 0` (a literal-false condition) arm is dead.  */

	  if !FRAME_TOOLTIP_P (f1)
	      /* Tooltips and child frames count neither for
		 invisibility nor for deletions.  */
	      && FRAME_PARENT_FRAME (f1).is_null ()
	      /* Frames with a non-nil `delete-before' parameter don't
		 count for deletions.  */
	      && (invisible || NILP (get_frame_param (f1, Qdelete_before ())))
	      /* For invisibility and normal deletions, at least one
		 visible or iconified frame must remain (Bug#26682).  */
	      && (FRAME_VISIBLE_P (f1)
		  || FRAME_ICONIFIED_P (f1)
		  || (!invisible
		      && (force
			  /* Allow deleting the terminal frame when at
			     least one X frame exists.  */
			  || (FRAME_WINDOW_P (f1) && !FRAME_WINDOW_P (f)))))
	    { return true; }
	}
  });

  return false;
}

/**
 * delete_frame:
 *
 * Delete FRAME.  When FORCE equals Qnoelisp, delete FRAME
 * unconditionally.  x_connection_closed and delete_terminal use this.
 * Any other value of FORCE implements the semantics described for
 * Fdelete_frame.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn delete_frame (frame: Lisp_Object, force: Lisp_Object) -> Lisp_Object
{
  let mut frame = frame;
  let f: *mut frame = decode_any_frame (frame);
  let mut sf: *mut frame;
  let mut kb: *mut KBOARD = std::ptr::null_mut ();
  let mut frames: Lisp_Object = Qnil ();
  let mut frame1: Lisp_Object = Qnil ();
  let is_tooltip_frame: c_int;
  let nochild: bool = FRAME_PARENT_FRAME (f).is_null ();
  let mut minibuffer_child_frame: Lisp_Object = Qnil ();
  /* C 2607-2609's `#ifdef HAVE_X_WINDOWS` arm is dead (P-08).  */

  if !FRAME_LIVE_P (f)
    { return Qnil (); }
  else if !EQ (force, Qnoelisp ()) && !other_frames (f, false, !NILP (force))
    {
      if NILP (force)
	{ error (c"Attempt to delete the sole visible or iconified frame".as_ptr ()); }
      else
	{ error (c"Attempt to delete the only frame".as_ptr ()); }
    }
  else if IS_DAEMON () && FRAME_INITIAL_P (f) && NILP (force)
    { error (c"Attempt to delete daemon's initial frame".as_ptr ()); }
  /* C 2622-2630's `#ifdef HAVE_X_WINDOWS`/`HAVE_HAIKU` arms are dead (P-08).  */

  XSETFRAME! (frame, f);

  if is_tty_frame (f) && NILP (force)
    /* If F is a tty frame, check for surrogate minibuffer frames F
       subsumes used by a frame that is not subsumed by F. */
    { FOR_EACH_FRAME! (frames, frame1, {
	let f1: *mut frame = XFRAME (frame1);

	if frame_subsumes_p (f, WINDOW_XFRAME! (XWINDOW ((*f1).minibuffer_window)))
	    && !frame_subsumes_p (f, f1)
	  { error (c"Cannot delete surrogate minibuffer frame".as_ptr ()); }
      }); }

  /* Softly delete all frames with this frame as their parent frame or
     as their `delete-before' frame parameter value.  */
  FOR_EACH_FRAME! (frames, frame1, {
    /* C's `continue` jumps to the `for` increment; the mirror's FOR_EACH_FRAME puts the
       increment at the body's end, so the port wraps the body in a labelled block and
       `break 'continue_;` from it — the increment still runs (`docs/ports/frame.md`).  */
    'continue_: {
      let f1: *mut frame = XFRAME (frame1);

      if EQ (frame1, frame) || FRAME_TOOLTIP_P (f1)
	{ break 'continue_; }
      else if FRAME_PARENT_FRAME (f1) == f
	{
	  if FRAME_HAS_MINIBUF_P (f1) && !FRAME_HAS_MINIBUF_P (f)
	      && EQ (FRAME_MINIBUF_WINDOW (f), FRAME_MINIBUF_WINDOW (f1))
	    /* frame1 owns frame's minibuffer window so we must not
	       delete it here to avoid a surrogate minibuffer error.
	       Unparent frame1 and make it a top-level frame.  */
	    {
	      Fmodify_frame_parameters
		(frame1, Fcons (Fcons (Qparent_frame (), Qnil ()), Qnil ()));
	      minibuffer_child_frame = frame1;
	    }
	  else
	    { delete_frame (frame1, Qnil ()); }
	}
      else if nochild
	       && EQ (get_frame_param (XFRAME (frame1), Qdelete_before ()), frame)
	/* Process `delete-before' parameter iff FRAME is not a child
	   frame.  This avoids that we enter an infinite chain of mixed
	   dependencies.  */
	{ delete_frame (frame1, Qnil ()); }
    }
  });

  /* Does this frame have a minibuffer, and is it the surrogate
     minibuffer for any other frame?  */
  if FRAME_HAS_MINIBUF_P (f)
    {
      FOR_EACH_FRAME! (frames, frame1, {
	 'continue_: {
	  let fminiw: Lisp_Object;

	  if EQ (frame1, frame)
	    { break 'continue_; }

	  fminiw = FRAME_MINIBUF_WINDOW (XFRAME (frame1));

	  if WINDOWP (fminiw) && EQ (frame, WINDOW_FRAME! (XWINDOW (fminiw)))
	    {
	      /* If we MUST delete this frame, delete the other first.
		 But do this only if FORCE equals `noelisp'.  */
	      if EQ (force, Qnoelisp ())
		{ delete_frame (frame1, Qnoelisp ()); }
	      else
		{ error (c"Attempt to delete a surrogate minibuffer frame".as_ptr ()); }
	    }
	 }
	});
    }

  is_tooltip_frame = if FRAME_TOOLTIP_P (f) { 1 } else { 0 };

  /* Run `delete-frame-functions' unless FORCE is `noelisp' or
     frame is a tooltip.  FORCE is set to `noelisp' when handling
     a disconnect from the terminal, so we don't dare call Lisp
     code.  */
  if NILP (Vrun_hooks) || is_tooltip_frame != 0
    { /* nothing: C's empty statement  */ }
  else if EQ (force, Qnoelisp ())
    { pending_funcalls
      = Fcons (list3 (Qrun_hook_with_args (), Qdelete_frame_functions (), frame),
	       pending_funcalls); }
  else
    {
      /* C 2716-2719's `#ifdef HAVE_X_WINDOWS` arm is dead (P-08).  */

      safe_calln! (Qrun_hook_with_args (), Qdelete_frame_functions (), frame);
    }

  /* delete_frame_functions may have deleted any frame, including this
     one.  */
  if !FRAME_LIVE_P (f)
    { return Qnil (); }
  else if !EQ (force, Qnoelisp ()) && !other_frames (f, false, !NILP (force))
    {
      if NILP (force)
	{ error (c"Attempt to delete the sole visible or iconified frame".as_ptr ()); }
      else
	{ error (c"Attempt to delete the only frame".as_ptr ()); }
    }

  sf = SELECTED_FRAME ();
  /* Don't let the frame remain selected.  */
  if f == sf
    {
      if is_tty_child_frame (f)
	/* If F is a child frame on a tty and is the selected frame, try
	   to re-select the frame that was selected before F.  */
	{ do_switch_frame (mru_rooted_frame (f as *mut c_void), 0, 1, Qnil ()); }
      else
	{
	  frame1 = Qnil ();

	  if EQ (delete_frame_choose_selected, Qmru ())
	      && !EQ (force, Qnoelisp ())
	    {
	      /* Find the most recently used visible frame among all
		 frames on the same terminal as FRAME, excluding FRAME
		 which we are about to delete.  */
	      frame1 = safe_calln! (Qget_mru_frame (), Qvisible (), Qnil (), frame);
	      if FRAMEP (frame1)
		{
		  let f1: *mut frame = XFRAME (frame1);

		  if FRAME_TOOLTIP_P (f1)
		      || FRAME_TERMINAL! (f) != FRAME_TERMINAL! (f1)
		      || !FRAME_VISIBLE_P (f1)
		      || EQ (frame1, frame)
		    { frame1 = Qnil (); }
		}
	      else
		{ frame1 = Qnil (); }
	    }

	  if NILP (frame1)
	    {
	      let mut tail: Lisp_Object = Qnil ();
	      eassume! (CONSP (Vframe_list));

	      /* Look for another visible frame on the same terminal.
		 Do not call next_frame here because it may loop forever.
		 See https://debbugs.gnu.org/cgi/bugreport.cgi?bug=15025.  */
	      FOR_EACH_FRAME! (tail, frame1, {
		  let f1: *mut frame = XFRAME (frame1);

		  if !EQ (frame, frame1)
		      && !FRAME_TOOLTIP_P (f1)
		      && FRAME_TERMINAL! (f) == FRAME_TERMINAL! (f1)
		      && FRAME_VISIBLE_P (f1)
		    { break; }
		  frame1 = Qnil ();
	      });

	      /* If there is none, find *some* other frame.  */
	      if NILP (frame1) || EQ (frame1, frame)
		{
		  FOR_EACH_FRAME! (tail, frame1, {
		      let f1: *mut frame = XFRAME (frame1);

		      if !EQ (frame, frame1)
			  && FRAME_LIVE_P (f1)
			  && !FRAME_TOOLTIP_P (f1)
			{
			  if is_tty_frame (f1)
			    {
			      let top_frame: Lisp_Object = (*FRAME_TTY! (f1)).top_frame;

			      if !EQ (top_frame, frame)
				{ frame1 = top_frame; }
			    }
			  break;
			}
		      frame1 = Qnil ();
		  });
		}
	      /* C 2812-2823's `#ifdef NS_IMPL_COCOA` arm is dead (P-08).  */
	    }

	  if FRAMEP (frame1) && XFRAME (frame1) != f
	    { do_switch_frame (frame1, 0, 1, Qnil ()); }
	  else if EQ (force, Qnoelisp ())
	    {
	      /* This is the last frame, and it's being forcibly
		 deleted.  There's no way to recover from this.  */
	      Fkill_emacs (make_fixnum (70), Qnil ());
	    }
	  else
	    { emacs_abort (); }
	  sf = SELECTED_FRAME ();
	  eassert! (sf != f);
	}
    }
  else
    /* Ensure any minibuffers on FRAME are moved onto the selected
       frame.  */
    { move_minibuffers_onto_frame (f, selected_frame, true); }

  /* At this point, we are committed to deleting the frame.
     There is no more chance for errors to prevent it.  */
  /* Don't let echo_area_window to remain on a deleted frame.  */
  if EQ ((*f).minibuffer_window, echo_area_window)
    { echo_area_window = (*sf).minibuffer_window; }

  /* Clear any X selections for this frame.  */
  /* C 2852-2877's `#ifdef HAVE_X_WINDOWS`/`HAVE_PGTK` arms are dead (P-08).  */

  /* Free glyphs.
     This function must be called before the window tree of the
     frame is deleted because windows contain dynamically allocated
     memory. */
  free_glyphs (f);

  /* C 2885-2888's `#ifdef HAVE_WINDOW_SYSTEM` arm is dead (P-08).  */

  /* Mark all the windows that used to be on FRAME as deleted, and then
     remove the reference to them.  */
  delete_all_child_windows ((*f).root_window);
  fset_root_window (f, Qnil ());

  Vframe_list = delq_no_quit (frame, Vframe_list);
  SET_FRAME_VISIBLE! (f, false);

  /* Allow the vector of menu bar contents to be freed in the next
     garbage collection.  The frame object itself may not be garbage
     collected until much later, because recent_keys and other data
     structures can still refer to it.  */
  fset_menu_bar_vector (f, Qnil ());

  /* If FRAME's buffer lists contains killed
     buffers, this helps GC to reclaim them.  */
  fset_buffer_list (f, Qnil ());
  fset_buried_buffer_list (f, Qnil ());

  free_font_driver_list (f);
  /* C 2910-2912's `#if defined (USE_X_TOOLKIT) || defined (HAVE_NTGUI)` arm is dead.  */
  xfree ((*f).decode_mode_spec_buffer as *mut c_void);
  xfree (FRAME_INSERT_COST (f) as *mut c_void);
  xfree (FRAME_DELETEN_COST (f) as *mut c_void);
  xfree (FRAME_INSERTN_COST (f) as *mut c_void);
  xfree (FRAME_DELETE_COST (f) as *mut c_void);

  /* Since some events are handled at the interrupt level, we may get
     an event for f at any time; if we zero out the frame's terminal
     now, then we may trip up the event-handling code.  Instead, we'll
     promise that the terminal of the frame must be valid until we
     have called the window-system-dependent frame destruction
     routine.  */
  /* Remember if this was a GUI child frame, so we can
     process pending window system events after destruction.  */
  let was_gui_child_frame: bool = FRAME_WINDOW_P (f) && !FRAME_PARENT_FRAME (f).is_null ();
  /* C 2928-2933's `#ifdef HAVE_X_WINDOWS` arm is dead (P-08).  */
  {
    let terminal: *mut terminal;
    block_input ();
    if let Some (delete_frame_hook) = (*FRAME_TERMINAL! (f)).delete_frame_hook
      { delete_frame_hook (f as *mut c_void); }
    terminal = FRAME_TERMINAL! (f);
    (*f).terminal = std::ptr::null_mut ();             /* Now the frame is dead.  */
    unblock_input ();

    /* When a GUI child frame is deleted, the window system may
       generate events that affect the parent frame (e.g.
       ConfigureNotify, Expose, etc.).  We need to sync with the
       X server to ensure all events from the frame destruction
       have been received, then process them to ensure subsequent
       operations like `recenter' see up-to-date window state.
       (Bug#76186)  */
    /* C 2950-2957's `#ifdef HAVE_X_WINDOWS` arm is dead (P-08).  */
    if was_gui_child_frame
      { swallow_events (false); }

    /* Clear markers and overlays set by F on behalf of an input
       method.  */
    /* C 2963-2966's `#ifdef HAVE_TEXT_CONVERSION` arm is dead (P-08).  */

    /* If needed, delete the terminal that this frame was on.
       (This must be done after the frame is killed.)  */
    (*terminal).reference_count -= 1;
    /* C 2971-2983's `#if defined (USE_X_TOOLKIT) || defined (USE_GTK)` arm is dead.  */

    if (*terminal).reference_count == 0
      {
	let mut tmp: Lisp_Object = Qnil ();
	XSETTERMINAL! (tmp, terminal);

        kb = std::ptr::null_mut ();

	/* If force is noelisp, the terminal is going away inside
	   x_delete_terminal, and a recursive call to Fdelete_terminal
	   is unsafe!  */
	if !EQ (force, Qnoelisp ())
	  { Fdelete_terminal (tmp, if NILP (force) { Qt () } else { force }); }
      }
    else
      { kb = (*terminal).kboard as *mut KBOARD; }
  }

  /* If we've deleted the last_nonminibuf_frame, then try to find
     another one.  */
  if f == last_nonminibuf_frame
    {
      last_nonminibuf_frame = std::ptr::null_mut ();

      FOR_EACH_FRAME! (frames, frame1, {
	  let f1: *mut frame = XFRAME (frame1);

	  if !FRAME_MINIBUF_ONLY_P (f1)
	    {
	      last_nonminibuf_frame = f1;
	      break;
	    }
      });
    }

  /* If there's no other frame on the same kboard, get out of
     single-kboard state if we're in it for this kboard.  */
  if !kb.is_null ()
    {
      /* Some frame we found on the same kboard, or nil if there are none.  */
      let mut frame_on_same_kboard: Lisp_Object = Qnil ();

      FOR_EACH_FRAME! (frames, frame1, {
	if kb as *mut c_void == FRAME_KBOARD (XFRAME (frame1))
	  { frame_on_same_kboard = frame1; }
      });

      if NILP (frame_on_same_kboard)
	{ not_single_kboard_state (kb); }
    }

  /* If we've deleted this keyboard's default_minibuffer_frame, try to
     find another one.  Prefer minibuffer-only frames, but also notice
     frames with other windows.  */
  if !kb.is_null () && EQ (frame, (*kb).Vdefault_minibuffer_frame_)
    {
      /* The last frame we saw with a minibuffer, minibuffer-only or not.  */
      let mut frame_with_minibuf: Lisp_Object = Qnil ();
      /* Some frame we found on the same kboard, or nil if there are none.  */
      let mut frame_on_same_kboard: Lisp_Object = Qnil ();

      FOR_EACH_FRAME! (frames, frame1, {
	  let f1: *mut frame = XFRAME (frame1);

	  /* Set frame_on_same_kboard to frame1 if it is on the same
	     keyboard and is not a tooltip frame.  Set
	     frame_with_minibuf to frame1 if it also has a minibuffer.
	     Leave the loop immediately if frame1 is also
	     minibuffer-only.

	     Emacs 26 did _not_ set frame_on_same_kboard here when it
	     found a minibuffer-only frame, and subsequently failed to
	     set default_minibuffer_frame below.  Not a great deal and
	     never noticed since make_frame_without_minibuffer created a
	     new minibuffer frame in that case (which can be a minor
	     annoyance though).  */
	  if !FRAME_TOOLTIP_P (f1)
	      && kb as *mut c_void == FRAME_KBOARD (f1)
	    {
	      frame_on_same_kboard = frame1;
	      if FRAME_HAS_MINIBUF_P (f1)
		{
		  frame_with_minibuf = frame1;
		  if FRAME_MINIBUF_ONLY_P (f1)
		    { break; }
		}
	    }
      });

      if !NILP (frame_on_same_kboard)
	{
	  /* We know that there must be some frame with a minibuffer out
	     there.  If this were not true, all of the frames present
	     would have to be minibufferless, which implies that at some
	     point their minibuffer frames must have been deleted, but
	     that is prohibited at the top; you can't delete surrogate
	     minibuffer frames.  */
	  if NILP (frame_with_minibuf)
	    { emacs_abort (); }

	  kset_default_minibuffer_frame (kb, frame_with_minibuf);
	}
      else
	/* No frames left on this kboard--say no minibuffer either.  */
	{ kset_default_minibuffer_frame (kb, Qnil ()); }
    }

  /* Cause frame titles to update--necessary if we now have just one
     frame.  */
  if is_tooltip_frame == 0
    { update_mode_lines = 15; }

  /* Now run the post-deletion hooks.  */
  if NILP (Vrun_hooks) || is_tooltip_frame != 0
    { /* nothing: C's empty statement  */ }
  else if EQ (force, Qnoelisp ())
    { pending_funcalls
      = Fcons (list3 (Qrun_hook_with_args (), Qafter_delete_frame_functions (), frame),
	       pending_funcalls); }
  else
    { safe_calln! (Qrun_hook_with_args (), Qafter_delete_frame_functions (), frame); }

  if !NILP (minibuffer_child_frame)
    /* If minibuffer_child_frame is non-nil, it was FRAME's minibuffer
       child frame.  Delete it unless it's also the minibuffer frame
       of another frame in which case we make sure it's visible.  */
    {
      let f1: *mut frame = XFRAME (minibuffer_child_frame);

      if FRAME_LIVE_P (f1)
	{
	  let window1: Lisp_Object = FRAME_ROOT_WINDOW (f1);
	  let mut frame2: Lisp_Object = Qnil ();

	  FOR_EACH_FRAME! (frames, frame2, {
	     'continue_: {
	      let f2: *mut frame = XFRAME (frame2);

	      if EQ (frame2, minibuffer_child_frame) || FRAME_TOOLTIP_P (f2)
		{ break 'continue_; }
	      else if EQ (FRAME_MINIBUF_WINDOW (f2), window1)
		{
		  /* minibuffer_child_frame serves as minibuffer frame
		     for at least one other frame - so make it visible
		     and quit.  */
		  if !FRAME_VISIBLE_P (f1) && !FRAME_ICONIFIED_P (f1)
		    { Fmake_frame_visible (minibuffer_child_frame); }

		  return Qnil ();
		}
	     }
	  });

	  /* No other frame found that uses minibuffer_child_frame as
	     minibuffer frame.  If FORCE is Qnoelisp or there are
	     other visible frames left, delete minibuffer_child_frame
	     since it presumably was used by FRAME only.  */
	  if EQ (force, Qnoelisp ()) || other_frames (f1, false, !NILP (force))
	    { delete_frame (minibuffer_child_frame, Qnoelisp ()); }
	}
    }

  return Qnil ();
}

DEFUN! ("delete-frame", Fdelete_frame, Sdelete_frame, 0, 2, "",
       doc: "Delete FRAME, eliminating it from use.\nFRAME must be a live frame and defaults to the selected one.\n\nWhen `undelete-frame-mode' is enabled, the 16 most recently deleted\nframes can be undeleted with `undelete-frame', which see.\n\nDo not delete a frame whose minibuffer serves as surrogate minibuffer\nfor another frame.  Do not delete a frame if all other frames are\ninvisible unless the second optional argument FORCE is non-nil.  Do not\ndelete the initial terminal frame of an Emacs process running as daemon\nunless FORCE is non-nil.\n\nThis function runs `delete-frame-functions' before actually\ndeleting the frame, unless the frame is a tooltip.\nThe functions are run with one argument, the frame to be deleted.  ",
       (frame: Lisp_Object, force: Lisp_Object),
       {
  return delete_frame (frame, if !NILP (force) { Qt () } else { Qnil () });
});

/**
 * frame_internal_border_part:
 *
 * Return part of internal border the coordinates X and Y relative to
 * frame F are on.  Return nil if the coordinates are not on the
 * internal border of F.
 *
 * Return one of INTERNAL_BORDER_LEFT_EDGE, INTERNAL_BORDER_TOP_EDGE,
 * INTERNAL_BORDER_RIGHT_EDGE or INTERNAL_BORDER_BOTTOM_EDGE when the
 * mouse cursor is on the corresponding border with an offset of at
 * least one canonical character height from that border's edges.
 *
 * If no border part could be found this way, return one of
 * INTERNAL_BORDER_TOP_LEFT_CORNER, INTERNAL_BORDER_TOP_RIGHT_CORNER,
 * INTERNAL_BORDER_BOTTOM_LEFT_CORNER or
 * INTERNAL_BORDER_BOTTOM_RIGHT_CORNER to indicate that the mouse is in
 * one of the corresponding corners.  This means that for very small
 * frames an `edge' return value is preferred.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_internal_border_part (f: *mut frame, x: c_int, y: c_int)
  -> internal_border_part
{
  let border: c_int = if FRAME_INTERNAL_BORDER_WIDTH (f) != 0
		{ FRAME_INTERNAL_BORDER_WIDTH (f) }
		else { if is_tty_child_frame (f) && FRAME_UNDECORATED (f) == 0
		{ 1 }
		else { 0 } };
  let mut offset: c_int = FRAME_LINE_HEIGHT (f);
  let width: c_int = FRAME_PIXEL_WIDTH! (f);
  let height: c_int = FRAME_PIXEL_HEIGHT! (f);
  let mut part: internal_border_part = INTERNAL_BORDER_NONE;

  if offset < border
    /* For very wide borders make offset at least as large as
       border.  */
    { offset = border; }

  if offset < x && x < width - offset
    /* Top or bottom border.  */
    {
      if 0 <= y && y <= border
	{ part = INTERNAL_BORDER_TOP_EDGE; }
      else if height - border <= y && y <= height
	{ part = INTERNAL_BORDER_BOTTOM_EDGE; }
    }
  else if offset < y && y < height - offset
    /* Left or right border.  */
    {
      if 0 <= x && x <= border
	{ part = INTERNAL_BORDER_LEFT_EDGE; }
      else if width - border <= x && x <= width
	{ part = INTERNAL_BORDER_RIGHT_EDGE; }
    }
  else
    {
      /* An edge.  */
      let half_width: c_int = width / 2;
      let half_height: c_int = height / 2;

      if 0 <= x && x <= border
	{
	  /* A left edge.  */
	  if 0 <= y && y <= half_height
	     { part = INTERNAL_BORDER_TOP_LEFT_CORNER; }
	  else if half_height < y && y <= height
	     { part = INTERNAL_BORDER_BOTTOM_LEFT_CORNER; }
	}
      else if width - border <= x && x <= width
	{
	  /* A right edge.  */
	  if 0 <= y && y <= half_height
	     { part = INTERNAL_BORDER_TOP_RIGHT_CORNER; }
	  else if half_height < y && y <= height
	     { part = INTERNAL_BORDER_BOTTOM_RIGHT_CORNER; }
	}
      else if 0 <= y && y <= border
	{
	  /* A top edge.  */
	  if 0 <= x && x <= half_width
	     { part = INTERNAL_BORDER_TOP_LEFT_CORNER; }
	  else if half_width < x && x <= width
	    { part = INTERNAL_BORDER_TOP_RIGHT_CORNER; }
	}
      else if height - border <= y && y <= height
	{
	  /* A bottom edge.  */
	  if 0 <= x && x <= half_width
	     { part = INTERNAL_BORDER_BOTTOM_LEFT_CORNER; }
	  else if half_width < x && x <= width
	    { part = INTERNAL_BORDER_BOTTOM_RIGHT_CORNER; }
	}
    }

  return part;
}

/* Return mouse position in character cell units.  */

DEFUN! ("mouse-position", Fmouse_position, Smouse_position, 0, 0, 0,
       doc: "Return a list (FRAME X . Y) giving the current mouse frame and position.\nThe position is given in canonical character cells, where (0, 0) is the\nupper-left corner of the frame, X is the horizontal offset, and Y is the\nvertical offset, measured in units of the frame's default character size.\nIf Emacs is running on a mouseless terminal or hasn't been programmed\nto read the mouse position, it returns the selected frame for FRAME\nand nil for X and Y.\n\nFRAME might be nil if `track-mouse' is set to `drag-source'.  This\nmeans there is no frame under the mouse.  If `mouse-position-function'\nis non-nil, `mouse-position' calls it, passing the normal return value\nto that function as an argument, and returns whatever that function\nreturns.  ",
       (),
       {
  return mouse_position (true);
});

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mouse_position (call_mouse_position_function: bool) -> Lisp_Object
{
  let mut f: *mut frame;
  let mut lispy_dummy: Lisp_Object = Qnil ();
  let mut x: Lisp_Object;
  let mut y: Lisp_Object;
  let mut retval: Lisp_Object;

  f = SELECTED_FRAME ();
  x = Qnil ();
  y = Qnil ();

  /* It's okay for the hook to refrain from storing anything.  */
  if (*FRAME_TERMINAL! (f)).mouse_position_hook.is_some ()
    {
      let mut party_dummy: scroll_bar_part = 0;
      let mut time_dummy: Time = unsafe { std::mem::zeroed () };
      if let Some (mouse_position_hook) = (*FRAME_TERMINAL! (f)).mouse_position_hook
	{ mouse_position_hook (&mut f as *mut *mut frame as *mut *mut c_void, -1,
			       &mut lispy_dummy, &mut party_dummy,
			       &mut x, &mut y,
			       &mut time_dummy); }
    }

  if !NILP (x) && !f.is_null ()
    {
      let mut col: c_int = XFIXNUM (x) as c_int;
      let mut row: c_int = XFIXNUM (y) as c_int;
      pixel_to_glyph_coords (f, col, row, &mut col, &mut row, std::ptr::null_mut (), true);
      XSETINT! (x, col as EMACS_INT);
      XSETINT! (y, row as EMACS_INT);
    }
  if !f.is_null ()
    { XSETFRAME! (lispy_dummy, f); }
  else
    { lispy_dummy = Qnil (); }
  retval = Fcons (lispy_dummy, Fcons (x, y));
  if call_mouse_position_function && !NILP (Vmouse_position_function)
    { retval = calln! (Vmouse_position_function, retval); }
  return retval;
}

DEFUN! ("mouse-pixel-position", Fmouse_pixel_position,
       Smouse_pixel_position, 0, 0, 0,
       doc: "Return a list (FRAME X . Y) giving the current mouse frame and position.\nThe position is given in pixel units, where (0, 0) is the\nupper-left corner of the frame, X is the horizontal offset, and Y is\nthe vertical offset.\nFRAME might be nil if `track-mouse' is set to `drag-source'.  This\nmeans there is no frame under the mouse.  If Emacs is running on a\nmouseless terminal or hasn't been programmed to read the mouse\nposition, it returns the selected frame for FRAME and nil for X and\nY.  ",
       (),
       {
  let mut f: *mut frame;
  let mut lispy_dummy: Lisp_Object = Qnil ();
  let mut x: Lisp_Object;
  let mut y: Lisp_Object;
  let mut retval: Lisp_Object;

  f = SELECTED_FRAME ();
  x = Qnil ();
  y = Qnil ();

  /* It's okay for the hook to refrain from storing anything.  */
  if (*FRAME_TERMINAL! (f)).mouse_position_hook.is_some ()
    {
      let mut party_dummy: scroll_bar_part = 0;
      let mut time_dummy: Time = unsafe { std::mem::zeroed () };
      if let Some (mouse_position_hook) = (*FRAME_TERMINAL! (f)).mouse_position_hook
	{ mouse_position_hook (&mut f as *mut *mut frame as *mut *mut c_void, -1,
			       &mut lispy_dummy, &mut party_dummy,
			       &mut x, &mut y,
			       &mut time_dummy); }
    }

  if !f.is_null ()
    { XSETFRAME! (lispy_dummy, f); }
  else
    { lispy_dummy = Qnil (); }

  retval = Fcons (lispy_dummy, Fcons (x, y));
  if !NILP (Vmouse_position_function)
    { retval = calln! (Vmouse_position_function, retval); }
  return retval;
});

/* C 3371-3405's `#ifdef HAVE_WINDOW_SYSTEM` arm is dead (P-08).  */

DEFUN! ("set-mouse-position", Fset_mouse_position, Sset_mouse_position, 3, 3, 0,
       doc: "Move the mouse pointer to the center of character cell (X,Y) in FRAME.\nCoordinates are relative to the frame, not a window,\nso the coordinates of the top left character in the frame\nmay be nonzero due to left-hand scroll bars or the menu bar.\n\nThe position is given in canonical character cells, where (0, 0) is\nthe upper-left corner of the frame, X is the horizontal offset, and\nY is the vertical offset, measured in units of the frame's default\ncharacter size.\n\nThis function is a no-op for an X frame that is not visible.\nIf you have just created a frame, you must wait for it to become visible\nbefore calling this function on it, like this.\n  (while (not (frame-visible-p frame)) (sleep-for .5))  ",
       (frame: Lisp_Object, x: Lisp_Object, y: Lisp_Object),
       {
  CHECK_LIVE_FRAME! (frame);
  let xval: c_int = check_integer_range (x, INT_MIN as intmax_t, INT_MAX as intmax_t) as c_int;
  let yval: c_int = check_integer_range (y, INT_MIN as intmax_t, INT_MAX as intmax_t) as c_int;

  /* I think this should be done with a hook.  */
  if FRAME_WINDOW_P (XFRAME (frame))
    {
      /* C 3431-3434's `#ifdef HAVE_WINDOW_SYSTEM` arm is dead (P-08).  */
    }
  /* C 3436-3442's `#ifdef MSDOS` arm is dead (MSDOS undef).  */
  else
    {
      Fselect_frame (frame, Qnil ());
      /* C 3446-3451's `#ifdef HAVE_GPM`/`#else` takes the `#else` arm.  */
      /* C's `(void) xval;`/`(void) yval;` discard the two values; reading them is the
	 port's counterpart of the casts to void (the dired.rs shape).  */
      let _ = xval;
      let _ = yval;
    }

  return Qnil ();
});

DEFUN! ("set-mouse-pixel-position", Fset_mouse_pixel_position,
       Sset_mouse_pixel_position, 3, 3, 0,
       doc: "Move the mouse pointer to pixel position (X,Y) in FRAME.\nThe position is given in pixels, where (0, 0) is the upper-left corner\nof the frame, X is the horizontal offset, and Y is the vertical offset.\n\nNote, this is a no-op for an X frame that is not visible.\nIf you have just created a frame, you must wait for it to become visible\nbefore calling this function on it, like this.\n  (while (not (frame-visible-p frame)) (sleep-for .5))  ",
       (frame: Lisp_Object, x: Lisp_Object, y: Lisp_Object),
       {
  CHECK_LIVE_FRAME! (frame);
  let xval: c_int = check_integer_range (x, INT_MIN as intmax_t, INT_MAX as intmax_t) as c_int;
  let yval: c_int = check_integer_range (y, INT_MIN as intmax_t, INT_MAX as intmax_t) as c_int;

  /* I think this should be done with a hook.  */
  if FRAME_WINDOW_P (XFRAME (frame))
    {
      /* Warping the mouse will cause enternotify and focus events.  */
      /* C 3477-3479's `#ifdef HAVE_WINDOW_SYSTEM` arm is dead (P-08).  */
    }
  /* C 3481-3487's `#ifdef MSDOS` arm is dead (MSDOS undef).  */
  else
    {
      Fselect_frame (frame, Qnil ());
      /* C 3491-3496's `#ifdef HAVE_GPM`/`#else` takes the `#else` arm.  */
      let _ = xval;
      let _ = yval;

    }

  return Qnil ();
});

/* C 3503's `static void make_frame_visible_1 (Lisp_Object);` is a forward declaration whose
   definition follows at C 3534, so the port drops the declaration line (PORTING.md 0.1-12;
   `make_frame_visible_1` itself is defined at its C position below).  */

DEFUN! ("make-frame-visible", Fmake_frame_visible, Smake_frame_visible,
       0, 1, "",
       doc: "Make the frame FRAME visible (assuming it is an X window).\nIf omitted, FRAME defaults to the currently selected frame.  ",
       (frame: Lisp_Object),
       {
  let mut frame = frame;
  let f: *mut frame = decode_live_frame (frame);

  if FRAME_WINDOW_P (f) && (*FRAME_TERMINAL! (f)).frame_visible_invisible_hook.is_some ()
    {
      if let Some (frame_visible_invisible_hook)
	= (*FRAME_TERMINAL! (f)).frame_visible_invisible_hook
	{ frame_visible_invisible_hook (f as *mut c_void, true); }
    }

  if is_tty_child_frame (f)
    {
      SET_FRAME_VISIBLE! (f, true);
      tty_raise_lower_frame (f, true);
    }

  make_frame_visible_1 ((*f).root_window);

  /* Make menu bar update for the Buffers and Frames menus.  */
  /* windows_or_buffers_changed = 15; FIXME: Why?  */

  XSETFRAME! (frame, f);
  return frame;
});

/* Update the display_time slot of the buffers shown in WINDOW
   and all its descendants.  */

unsafe fn make_frame_visible_1 (window: Lisp_Object)
{
  let mut window = window;
  let mut w: *mut window;

  while !NILP (window)
    {
      w = XWINDOW (window);
      if WINDOWP ((*w).contents)
	{ make_frame_visible_1 ((*w).contents); }
      else
	{ bset_display_time (XBUFFER ((*w).contents), Fcurrent_time ()); }
      window = (*w).next;
    }
}

DEFUN! ("make-frame-invisible", Fmake_frame_invisible, Smake_frame_invisible,
       0, 2, "",
       doc: "Make the frame FRAME invisible.\nIf omitted, FRAME defaults to the currently selected frame.\nOn graphical displays, invisible frames are not updated and are\nusually not displayed at all, even in a window system's \"taskbar\".\n\nNormally you may not make FRAME invisible if all other frames are\ninvisible, but if the second optional argument FORCE is non-nil, you may\ndo so.\n\nOn a text terminal make FRAME invisible if and only if FRAME is a child\nframe.  If, in that case, FRAME is the selected frame, select the first\nvisible ancestor of FRAME instead.  ",
       (frame: Lisp_Object, force: Lisp_Object),
       {
  let mut frame = frame;
  let f: *mut frame = decode_live_frame (frame);

  XSETFRAME! (frame, f);

  if NILP (force) && !other_frames (f, true, false)
    { error (c"Attempt to make invisible the sole visible or iconified frame".as_ptr ()); }

  if FRAME_WINDOW_P (f) && (*FRAME_TERMINAL! (f)).frame_visible_invisible_hook.is_some ()
    {
      if let Some (frame_visible_invisible_hook)
	= (*FRAME_TERMINAL! (f)).frame_visible_invisible_hook
	{ frame_visible_invisible_hook (f as *mut c_void, false); }
    }

  if is_tty_child_frame (f)
    {
      SET_FRAME_VISIBLE! (f, false);
      if EQ (frame, selected_frame)
	/* If FRAME is a tty child frame and the selected frame, we have
	   to select another frame instead.  Use the first visible
	   ancestor as returned by 'mru_rooted_frame'.  */
	{ Fselect_frame (mru_rooted_frame (f as *mut c_void), Qnil ()); }
    }

  /* Make menu bar update for the Buffers and Frames menus.  */
  windows_or_buffers_changed = 16;

  return Qnil ();
});

DEFUN! ("iconify-frame", Ficonify_frame, Siconify_frame,
       0, 1, "",
       doc: "Make the frame FRAME into an icon.\nIf omitted, FRAME defaults to the currently selected frame.\n\nIf FRAME is a child frame, consult the variable `iconify-child-frame'\nfor how to proceed.  ",
       (frame: Lisp_Object),
       {
  let f: *mut frame = decode_live_frame (frame);

  if !FRAME_PARENT_FRAME (f).is_null ()
    {
      if NILP (iconify_child_frame)
	/* Do nothing.  */
	{ return Qnil (); }
      else if FRAME_WINDOW_P (f)
	       && EQ (iconify_child_frame, Qiconify_top_level ())
	{
	  /* Iconify root frame (the default).  */
	  let mut root: Lisp_Object = Qnil ();

	  XSETFRAME! (root, root_frame (f));
	  Ficonify_frame (root);

	  return Qnil ();
	}
      else if EQ (iconify_child_frame, Qmake_invisible ())
	{
	  /* Make frame invisible.  */
	  Fmake_frame_invisible (frame, Qnil ());

	  return Qnil ();
	}
    }

  if FRAME_WINDOW_P (f) && (*FRAME_TERMINAL! (f)).iconify_frame_hook.is_some ()
    {
      if let Some (iconify_frame_hook) = (*FRAME_TERMINAL! (f)).iconify_frame_hook
	{ iconify_frame_hook (f as *mut c_void); }
    }

  return Qnil ();
});

DEFUN! ("frame-visible-p", Fframe_visible_p, Sframe_visible_p,
       1, 1, 0,
       doc: "Return t if FRAME is \"visible\" (actually in use for display).\nReturn the symbol `icon' if FRAME is iconified or \"minimized\".\nReturn nil if FRAME was made invisible, via `make-frame-invisible'.\nOn graphical displays, invisible frames are not updated and are\nusually not displayed at all, even in a window system's \"taskbar\".  ",
       (frame: Lisp_Object),
       {
  CHECK_LIVE_FRAME! (frame);
  let f: *mut frame = XFRAME (frame);

  if FRAME_VISIBLE_P (f)
    { return Qt (); }
  if FRAME_ICONIFIED_P (f)
    { return Qicon (); }
  return Qnil ();
});

DEFUN! ("visible-frame-list", Fvisible_frame_list, Svisible_frame_list,
       0, 0, 0,
       doc: "Return a list of all frames now \"visible\" (being updated).  ",
       (),
       {
  let mut tail: Lisp_Object = Qnil ();
  let mut frame: Lisp_Object = Qnil ();
  let mut value: Lisp_Object = Qnil ();

  FOR_EACH_FRAME! (tail, frame, {
    if FRAME_VISIBLE_P (XFRAME (frame))
      { value = Fcons (frame, value); }
  });

  return value;
});

DEFUN! ("raise-frame", Fraise_frame, Sraise_frame, 0, 1, "",
       doc: "Bring FRAME to the front, so it occludes any frames it overlaps.\nIf FRAME is invisible or iconified, make it visible.\nIf you don't specify a frame, the selected frame is used.  ",
       (frame: Lisp_Object),
       {
  let mut frame = frame;
  let f: *mut frame = decode_live_frame (frame);

  XSETFRAME! (frame, f);

  Fmake_frame_visible (frame);

  if (*FRAME_TERMINAL! (f)).frame_raise_lower_hook.is_some ()
    {
      if let Some (frame_raise_lower_hook) = (*FRAME_TERMINAL! (f)).frame_raise_lower_hook
	{ frame_raise_lower_hook (f as *mut c_void, true); }
    }

  return Qnil ();
});

/* Should we have a corresponding function called Flower_Power?  */
DEFUN! ("lower-frame", Flower_frame, Slower_frame, 0, 1, "",
       doc: "Send FRAME to the back, so it is occluded by any frames that overlap it.\nIf you don't specify a frame, the selected frame is used.\nIf Emacs is displaying on an ordinary terminal or some other device which\ndoesn't support multiple overlapping frames, this function does nothing.  ",
       (frame: Lisp_Object),
       {
  let f: *mut frame = decode_live_frame (frame);

  if (*FRAME_TERMINAL! (f)).frame_raise_lower_hook.is_some ()
    {
      if let Some (frame_raise_lower_hook) = (*FRAME_TERMINAL! (f)).frame_raise_lower_hook
	{ frame_raise_lower_hook (f as *mut c_void, false); }
    }

  return Qnil ();
});

DEFUN! ("redirect-frame-focus", Fredirect_frame_focus, Sredirect_frame_focus,
       1, 2, 0,
       doc: "Arrange for keystrokes typed at FRAME to be sent to FOCUS-FRAME.\nIn other words, switch-frame events caused by events in FRAME will\nrequest a switch to FOCUS-FRAME, and `last-event-frame' will be\nFOCUS-FRAME after reading an event typed at FRAME.\n\nIf FOCUS-FRAME is nil, any existing redirection is canceled, and the\nframe again receives its own keystrokes.\n\nFocus redirection is useful for temporarily redirecting keystrokes to\na surrogate minibuffer frame when a frame doesn't have its own\nminibuffer window.\n\nA frame's focus redirection can be changed by `select-frame'.  If frame\nFOO is selected, and then a different frame BAR is selected, any\nframes redirecting their focus to FOO are shifted to redirect their\nfocus to BAR.  This allows focus redirection to work properly when the\nuser switches from one frame to another using `select-window'.\n\nThis means that a frame whose focus is redirected to itself is treated\ndifferently from a frame whose focus is redirected to nil; the former\nis affected by `select-frame', while the latter is not.\n\nThe redirection lasts until `redirect-frame-focus' is called to change it.  ",
       (frame: Lisp_Object, focus_frame: Lisp_Object),
       {
  /* Note that we don't check for a live frame here.  It's reasonable
     to redirect the focus of a frame you're about to delete, if you
     know what other frame should receive those keystrokes.  */
  let f: *mut frame = decode_any_frame (frame);

  if !NILP (focus_frame)
    { CHECK_LIVE_FRAME! (focus_frame); }

  fset_focus_frame (f, focus_frame);

  if (*FRAME_TERMINAL! (f)).frame_rehighlight_hook.is_some ()
    {
      if let Some (frame_rehighlight_hook) = (*FRAME_TERMINAL! (f)).frame_rehighlight_hook
	{ frame_rehighlight_hook (f as *mut c_void); }
    }

  return Qnil ();
});

DEFUN! ("frame-focus", Fframe_focus, Sframe_focus, 0, 1, 0,
       doc: "Return the frame to which FRAME's keystrokes are currently being sent.\nIf FRAME is omitted or nil, the selected frame is used.\nReturn nil if FRAME's focus is not redirected.\nSee `redirect-frame-focus'.  ",
       (frame: Lisp_Object),
       {
  return FRAME_FOCUS_FRAME (decode_live_frame (frame));
});

DEFUN! ("x-focus-frame", Fx_focus_frame, Sx_focus_frame, 1, 2, 0,
       doc: "Set the input focus to FRAME.\nFRAME nil means use the selected frame.  Optional argument NOACTIVATE\nmeans do not activate FRAME.\n\nIf there is no window system support, this function does nothing.  ",
     (frame: Lisp_Object, noactivate: Lisp_Object),
     {
  /* C 3764-3768's `#ifdef HAVE_WINDOW_SYSTEM` arm is dead (P-08).  */
  return Qnil ();
});

DEFUN! ("frame-after-make-frame",
       Fframe_after_make_frame,
       Sframe_after_make_frame, 2, 2, 0,
       doc: "Mark FRAME as made.\nFRAME nil means use the selected frame.  Second argument MADE non-nil\nmeans functions on `window-configuration-change-hook' are called\nwhenever the window configuration of FRAME changes.  MADE nil means\nthese functions are not called.\n\nThis function is currently called by `make-frame' only and should be\notherwise used with utter care to avoid that running functions on\n`window-configuration-change-hook' is impeded forever.  ",
       (frame: Lisp_Object, made: Lisp_Object),
       {
  let f: *mut frame = decode_live_frame (frame);
  after_make_frame! (f, !NILP (made));
  return made;
});

/* Discard BUFFER from the buffer-list and buried-buffer-list of each frame.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn frames_discard_buffer (buffer: Lisp_Object)
{
  let mut frame: Lisp_Object = Qnil ();
  let mut tail: Lisp_Object = Qnil ();

  FOR_EACH_FRAME! (tail, frame, {
      fset_buffer_list
	(XFRAME (frame), Fdelq (buffer, (*XFRAME (frame)).buffer_list));
      fset_buried_buffer_list
	(XFRAME (frame), Fdelq (buffer, (*XFRAME (frame)).buried_buffer_list));
  });
}

/* Modify the alist in *ALISTPTR to associate PROP with VAL.
   If the alist already has an element for PROP, we change it.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn store_in_alist (alistptr: *mut Lisp_Object, prop: Lisp_Object,
                                         val: Lisp_Object)
{
  let tem: Lisp_Object = Fassq (prop, *alistptr);
  if NILP (tem)
    { *alistptr = Fcons (Fcons (prop, val), *alistptr); }
  else
    { Fsetcdr (tem, val); }
}

unsafe fn frame_name_fnn_p (str_: *mut c_char, len: ptrdiff_t) -> c_int
{
  if len > 1 && *str_ == b'F' as c_char
      && b'0' as c_char <= *str_.offset (1) && *str_.offset (1) <= b'9' as c_char
    {
      let mut p: *mut c_char = str_.offset (2);
      while b'0' as c_char <= *p && *p <= b'9' as c_char
	{ p = p.offset (1); }
      if p == str_.offset (len)
	{ return 1; }
    }
  return 0;
}

/* Set the name of the terminal frame.  Also used by MSDOS frames.
   Modeled after *_set_name which is used for WINDOW frames.  */

unsafe fn set_term_frame_name (f: *mut frame, name: Lisp_Object)
{
  let mut name = name;
  explicit_name! (f, !NILP (name));

  /* If NAME is nil, set the name to F<num>.  */
  if NILP (name)
    {
      /* Check for no change needed in this very common case
	 before we do any consing.  */
      if frame_name_fnn_p (SSDATA ((*f).name), SBYTES ((*f).name)) != 0
	{ return; }
      name = frame_next_F_name ();
    }
  else
    {
      CHECK_STRING (name);

      /* Don't change the name if it's already NAME.  */
      if !NILP (Fstring_equal (name, (*f).name))
	{ return; }

      /* Stop the user setting the name to F<num> if it is already in use.  */
      if frame_name_fnn_p (SSDATA (name), SBYTES (name)) != 0
	  && !NILP (frame_get (name))
	{ error (c"Frame of the form F<num> named `%s' already exists".as_ptr (),
		 SSDATA (name)); }
    }

  fset_name (f, name);
  update_mode_lines = 16;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn store_frame_param (f: *mut frame, prop: Lisp_Object, val: Lisp_Object)
{
  let old_alist_elt: Lisp_Object;
  let mut val = val;

  if EQ (prop, Qminibuffer ())
    {
      if WINDOWP (val)
	{
	  if !WINDOW_LIVE_P (val) || !MINI_WINDOW_P (XWINDOW (val))
	    { error (c"The `minibuffer' parameter does not specify a valid minibuffer window".as_ptr ()); }
	  else if FRAME_LIVE_P (f)
		   && FRAME_TERMINAL! (WINDOW_XFRAME! (XWINDOW (val))) != FRAME_TERMINAL! (f)
	    { error (c"Minibuffer window must be on same terminal as frame that uses it".as_ptr ()); }
	  else if FRAME_MINIBUF_ONLY_P (f)
	    {
	      if EQ (val, FRAME_MINIBUF_WINDOW (f))
		{ val = Qonly (); }
	      else
		{ error (c"Can't change the minibuffer window of a minibuffer-only frame".as_ptr ()); }
	    }
	  else if FRAME_HAS_MINIBUF_P (f)
	    {
	      if EQ (val, FRAME_MINIBUF_WINDOW (f))
		{ val = Qt (); }
	      else
		{ error (c"Can't change the minibuffer window of a frame with its own minibuffer".as_ptr ()); }
	    }
	  else if is_tty_frame (f)
		   && (root_frame (WINDOW_XFRAME! (XWINDOW (val)))
		       != root_frame (f))
	    { error (c"A frame and its surrogate minibuffer frame must have the same roots".as_ptr ()); }
	  else
	    /* Store the chosen minibuffer window.  */
	    { fset_minibuffer_window (f, val); }
	}
      else
	{
	  let old_val: Lisp_Object = Fcdr (Fassq (Qminibuffer (), (*f).param_alist));

	  if !NILP (old_val)
	    {
	      if WINDOWP (old_val) && NILP (val)
		/* Don't change the value for a minibuffer-less frame if
		   only nil was specified as new value.  */
		{ val = old_val; }
	      else if !EQ (old_val, val)
		{ error (c"Can't change the `minibuffer' parameter of this frame".as_ptr ()); }
	    }
	}
    }

  /* Check each parent-frame and delete-before parameter for a
     circular dependency.  Do not check between parameters, so you can
     still create circular dependencies with different properties, for
     example a chain of frames F1->F2->...Fn such that F1 is an ancestor
     frame of Fn and thus cannot be deleted before Fn and a second chain
     Fn->Fn-1->...F1 such that Fn cannot be deleted before F1.  */
  else if EQ (prop, Qparent_frame ()) || EQ (prop, Qdelete_before ())
    {
      let oldval: Lisp_Object = Fcdr (Fassq (prop, (*f).param_alist));

      if !EQ (oldval, val) && !NILP (val)
	{
	  let mut frame: Lisp_Object = Qnil ();
	  let mut frame1: Lisp_Object = val;

	  if !FRAMEP (frame1) || !FRAME_LIVE_P (XFRAME (frame1))
	      || (FRAME_LIVE_P (f)
		  && FRAME_TERMINAL! (f) != FRAME_TERMINAL! (XFRAME (frame1)))
	    { error (c"Invalid `%s' frame parameter".as_ptr (),
		     SSDATA (SYMBOL_NAME (prop))); }

	  XSETFRAME! (frame, f);

	  while FRAMEP (frame1) && FRAME_LIVE_P (XFRAME (frame1))
	    {
	    if EQ (frame1, frame)
	      { error (c"Circular specification of `%s' frame parameter".as_ptr (),
		     SSDATA (SYMBOL_NAME (prop))); }
	    else
	      { frame1 = get_frame_param (XFRAME (frame1), prop); }
	    }
	}
    }

  /* The buffer-list parameters are stored in a special place and not
     in the alist.  All buffers must be live.  */
  else if EQ (prop, Qbuffer_list ())
    {
      let mut list: Lisp_Object = Qnil ();
      while CONSP (val)
	{
	if !NILP (Fbuffer_live_p (XCAR (val)))
	  { list = Fcons (XCAR (val), list); }
	val = XCDR (val);
	}
      fset_buffer_list (f, Fnreverse (list));
      return;
    }
  else if EQ (prop, Qburied_buffer_list ())
    {
      let mut list: Lisp_Object = Qnil ();
      while CONSP (val)
	{
	if !NILP (Fbuffer_live_p (XCAR (val)))
	  { list = Fcons (XCAR (val), list); }
	val = XCDR (val);
	}
      fset_buried_buffer_list (f, Fnreverse (list));
      return;
    }
  else if (EQ (prop, Qscroll_bar_width ()) || EQ (prop, Qscroll_bar_height ()))
	   && !NILP (val) && !RANGED_FIXNUMP (1, val, INT_MAX as EMACS_INT)
    {
      let old_val: Lisp_Object = Fcdr (Fassq (prop, (*f).param_alist));

      val = old_val;
    }

  /* The parent frame parameter for ttys must be handled specially.  */
  if is_tty_frame (f) && EQ (prop, Qparent_frame ())
    {
      /* Invariant: When a frame F1 uses a surrogate minibuffer frame M1
	 on a tty, both F1 and M1 must have the same root frame.  */
      let mut frames: Lisp_Object = Qnil ();
      let mut frame1: Lisp_Object = Qnil ();
      let old_val: Lisp_Object = (*f).parent_frame;

      FOR_EACH_FRAME! (frames, frame1, {
	  let f1: *mut frame = XFRAME (frame1);

	  /* Spare GUI frames (Bug#79947).  */
	  if is_tty_frame (f1)
	    {
	      let m1: *mut frame = WINDOW_XFRAME! (XWINDOW ((*f1).minibuffer_window));
	      let mismatch: bool;

	      /* Temporarily install VAL and check whether our invariant
		 above gets violated.  */
	      (*f).parent_frame = val;
	      mismatch = root_frame (f1) != root_frame (m1);
	      (*f).parent_frame = old_val;

	      if mismatch
		{ error (c"Cannot re-root surrogate minibuffer frame".as_ptr ()); }
	    }
      });

      if f == XFRAME ((*(*FRAME_TERMINAL! (f)).display_info.tty
			 .cast::<tty_display_info> ()).top_frame)
	  && !NILP (val)
	{ error (c"Cannot make tty top frame a child frame".as_ptr ()); }
      else if NILP (val)
	{
	  if !FRAME_HAS_MINIBUF_P (f)
	      && !frame_ancestor_p
		  (f, WINDOW_XFRAME! (XWINDOW ((*f).minibuffer_window)))
	    { error (c"Cannot make tty root frame without valid minibuffer window".as_ptr ()); }
	  else
	    {
	      /* When making a frame a root frame, expand it to full size,
		 if necessary, and position it at top left corner.  */
	      let mut width: c_int = 0;
	      let mut height: c_int = 0;

	      get_tty_size (fileno ((*FRAME_TTY! (f)).input), &mut width, &mut height);
	      adjust_frame_size (f, width, height - FRAME_TOP_MARGIN (f), 5, false,
				 Qterminal_frame ());
	      (*f).left_pos = 0;
	      (*f).top_pos = 0;
	    }
	}

      SET_FRAME_GARBAGED! (root_frame (f));
      (*f).parent_frame = val;
      SET_FRAME_GARBAGED! (root_frame (f));
    }

  /* The tty color needed to be set before the frame's parameter
     alist was updated with the new value.  This is not true any more,
     but we still do this test early on.  */
  if FRAME_TERMCAP_P (f) && EQ (prop, Qtty_color_mode ())
      && f == (*FRAME_TTY! (f)).previous_frame
    /* Force redisplay of this tty.  */
    { (*FRAME_TTY! (f)).previous_frame = std::ptr::null_mut (); }

  /* Update the frame parameter alist.  */
  old_alist_elt = Fassq (prop, (*f).param_alist);
  if NILP (old_alist_elt)
    { fset_param_alist (f, Fcons (Fcons (prop, val), (*f).param_alist)); }
  else
    { Fsetcdr (old_alist_elt, val); }

  /* Update some other special parameters in their special places
     in addition to the alist.  */

  if EQ (prop, Qbuffer_predicate ())
    { fset_buffer_predicate (f, val); }

  if !FRAME_WINDOW_P (f)
    {
      if EQ (prop, Qmenu_bar_lines ())
	{ set_menu_bar_lines (f, val, make_fixnum (FRAME_MENU_BAR_LINES! (f) as EMACS_INT)); }
      else if EQ (prop, Qtab_bar_lines ())
	{ set_tab_bar_lines (f, val, make_fixnum (FRAME_TAB_BAR_LINES! (f) as EMACS_INT)); }
      else if EQ (prop, Qname ())
	{ set_term_frame_name (f, val); }
    }
}

/* Return color matches UNSPEC on frame F or nil if UNSPEC
   is not an unspecified foreground or background color.  */

unsafe fn frame_unspecified_color (f: *mut frame, unspec: Lisp_Object) -> Lisp_Object
{
  return if strncmp (SSDATA (unspec),
		     std::ptr::addr_of! (unspecified_bg) as *const c_char,
		     SBYTES (unspec) as usize) == 0
	  { tty_color_name (f, FRAME_BACKGROUND_PIXEL! (f) as c_int) }
	  else { if strncmp (SSDATA (unspec),
			     std::ptr::addr_of! (unspecified_fg) as *const c_char,
			     SBYTES (unspec) as usize) == 0
	     { tty_color_name (f, FRAME_FOREGROUND_PIXEL! (f) as c_int) }
	     else { Qnil () } };
}

DEFUN! ("frame-parameters", Fframe_parameters, Sframe_parameters, 0, 1, 0,
       doc: "Return the parameters-alist of frame FRAME.\nIt is a list of elements of the form (PARM . VALUE), where PARM is a symbol.\nThe meaningful PARMs depend on the kind of frame.\nIf FRAME is omitted or nil, return information on the currently selected frame.  ",
       (frame: Lisp_Object),
       {
  let mut alist: Lisp_Object;
  let f: *mut frame = decode_any_frame (frame);
  let mut height: c_int;
  let mut width: c_int;

  if !FRAME_LIVE_P (f)
    { return Qnil (); }

  alist = Fcopy_alist ((*f).param_alist);

  if !FRAME_WINDOW_P (f)
    {
      let mut elt: Lisp_Object;

      /* If the frame's parameter alist says the colors are
	 unspecified and reversed, take the frame's background pixel
	 for foreground and vice versa.  */
      elt = Fassq (Qforeground_color (), alist);
      if CONSP (elt) && STRINGP (XCDR (elt))
	{
	  elt = frame_unspecified_color (f, XCDR (elt));
	  if !NILP (elt)
	    { store_in_alist (&mut alist, Qforeground_color (), elt); }
	}
      else
	{ store_in_alist (&mut alist, Qforeground_color (),
			tty_color_name (f, FRAME_FOREGROUND_PIXEL! (f) as c_int)); }
      elt = Fassq (Qbackground_color (), alist);
      if CONSP (elt) && STRINGP (XCDR (elt))
	{
	  elt = frame_unspecified_color (f, XCDR (elt));
	  if !NILP (elt)
	    { store_in_alist (&mut alist, Qbackground_color (), elt); }
	}
      else
	{ store_in_alist (&mut alist, Qbackground_color (),
			tty_color_name (f, FRAME_BACKGROUND_PIXEL! (f) as c_int)); }
      store_in_alist (&mut alist, Qfont (),
		      build_string (if FRAME_MSDOS_P (f)
				    { c"ms-dos".as_ptr () }
				    else if FRAME_W32_P (f) { c"w32term".as_ptr () }
				    else { c"tty".as_ptr () }));
    }

  store_in_alist (&mut alist, Qname (), (*f).name);
  /* It's questionable whether here we should report the value of
     f->new_height (and f->new_width below) but we've done that in the
     past, so let's keep it.  Note that a value of -1 for either of
     these means that no new size was requested.

     But check f->new_size before to make sure that f->new_height and
     f->new_width are not ones requested by adjust_frame_size.  */
  height = if new_size_p! (f) && (*f).new_height >= 0
	    { (*f).new_height / FRAME_LINE_HEIGHT (f) }
	    else { FRAME_LINES! (f) };
  store_in_alist (&mut alist, Qheight (), make_fixnum (height as EMACS_INT));
  width = if new_size_p! (f) && (*f).new_width >= 0
	   { (*f).new_width / FRAME_COLUMN_WIDTH (f) }
	   else { FRAME_COLS! (f) };
  store_in_alist (&mut alist, Qwidth (), make_fixnum (width as EMACS_INT));

  store_in_alist (&mut alist, Qmodeline (),
		  if FRAME_WANTS_MODELINE_P (f) { Qt () } else { Qnil () });
  store_in_alist (&mut alist, Qunsplittable (),
		  if FRAME_NO_SPLIT_P (f) { Qt () } else { Qnil () });
  store_in_alist (&mut alist, Qbuffer_list (), (*f).buffer_list);
  store_in_alist (&mut alist, Qburied_buffer_list (), (*f).buried_buffer_list);

  /* I think this should be done with a hook.  */
  /* C 4156-4160's `#ifdef HAVE_WINDOW_SYSTEM` arm is dead (P-08).  */
    {
      store_in_alist (&mut alist, Qmenu_bar_lines (),
		      make_fixnum (FRAME_MENU_BAR_LINES! (f) as EMACS_INT));
      store_in_alist (&mut alist, Qtab_bar_lines (),
		      make_fixnum (FRAME_TAB_BAR_LINES! (f) as EMACS_INT));
      store_in_alist (&mut alist, Qvisibility (),
		      if FRAME_VISIBLE_P (f) { Qt () } else { Qnil () });
      store_in_alist (&mut alist, Qno_accept_focus (),
		      if FRAME_NO_ACCEPT_FOCUS (f) != 0 { Qt () } else { Qnil () });
    }

  return alist;
});

DEFUN! ("frame-parameter", Fframe_parameter, Sframe_parameter, 2, 2, 0,
       doc: "Return FRAME's value for parameter PARAMETER.\nIf FRAME is nil, describe the currently selected frame.  ",
       (frame: Lisp_Object, parameter: Lisp_Object),
       {
  let mut frame = frame;
  let f: *mut frame = decode_any_frame (frame);
  let mut value: Lisp_Object = Qnil ();

  CHECK_SYMBOL (parameter);

  XSETFRAME! (frame, f);

  if FRAME_LIVE_P (f)
    {
      /* Avoid consing in frequent cases.  */
      if EQ (parameter, Qname ())
	{ value = (*f).name; }
      /* C 4189-4208's `#ifdef HAVE_WINDOW_SYSTEM`/`HAVE_X_WINDOWS` arms are dead (P-08).  */
      else if EQ (parameter, Qbackground_color ())
	       || EQ (parameter, Qforeground_color ())
	{
	  value = Fassq (parameter, (*f).param_alist);
	  if CONSP (value)
	    {
	      value = XCDR (value);
	      /* Fframe_parameters puts the actual fg/bg color names,
		 even if f->param_alist says otherwise.  This is
		 important when param_alist's notion of colors is
		 "unspecified".  We need to do the same here.  */
	      if STRINGP (value) && !FRAME_WINDOW_P (f)
		{
		  let tem: Lisp_Object = frame_unspecified_color (f, value);

		  if !NILP (tem)
		    { value = tem; }
		}
	    }
	  else
	    { value = Fcdr (Fassq (parameter, Fframe_parameters (frame))); }
	}
      else if EQ (parameter, Qdisplay_type ())
	       || EQ (parameter, Qbackground_mode ())
	{ value = Fcdr (Fassq (parameter, (*f).param_alist)); }
      else
	/* FIXME: Avoid this code path at all (as well as code duplication)
	   by sharing more code with Fframe_parameters.  */
	{ value = Fcdr (Fassq (parameter, Fframe_parameters (frame))); }
    }

  return value;
});

DEFUN! ("modify-frame-parameters", Fmodify_frame_parameters,
       Smodify_frame_parameters, 2, 2, 0,
       doc: "Modify FRAME according to new values of its parameters in ALIST.\nIf FRAME is nil, it defaults to the selected frame.\nALIST is an alist of parameters to change and their new values.\nEach element of ALIST has the form (PARM . VALUE), where PARM is a symbol.\nWhich PARMs are meaningful depends on the kind of frame.\nThe meaningful parameters are acted upon, i.e. the frame is changed\naccording to their new values, and are also stored in the frame's\nparameter list so that `frame-parameters' will return them.\nPARMs that are not meaningful are still stored in the frame's parameter\nlist, but are otherwise ignored.  ",
       (frame: Lisp_Object, alist: Lisp_Object),
       {
  let f: *mut frame = decode_live_frame (frame);
  let mut prop: Lisp_Object = Qnil ();
  let mut val: Lisp_Object = Qnil ();

  /* I think this should be done with a hook.  */
  /* C 4261-4270's `#ifdef HAVE_WINDOW_SYSTEM`/`MSDOS` arms are dead (P-08).  */

    {
      let length: EMACS_INT = list_length (alist);
      let mut i: ptrdiff_t;
      let mut parms: *mut Lisp_Object = std::ptr::null_mut ();
      let values: *mut Lisp_Object;
      USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
      SAFE_ALLOCA_LISP! (sa_avail, parms, 2 * length);
      values = parms.offset (length as isize);
      let mut params: Lisp_Object = alist;
      let mut alist = alist;

      /* Extract parm names and values into those vectors.  */

      i = 0;
      while CONSP (alist)
	{
	  let elt: Lisp_Object;

	  elt = XCAR (alist);
	  *parms.offset (i) = Fcar (elt);
	  *values.offset (i) = Fcdr (elt);
	  i += 1;
	  alist = XCDR (alist);
	}

      /* Now process them in reverse of specified order.  */
      while { i -= 1; i >= 0 }
	{
	  prop = *parms.offset (i);
	  val = *values.offset (i);
	  store_frame_param (f, prop, val);

	  if EQ (prop, Qforeground_color ())
	      || EQ (prop, Qbackground_color ())
	    { update_face_from_frame_parameter (f, prop, val); }
	}

      if is_tty_child_frame (f)
	{
	  let w: c_int = tty_child_size_param (f, Qwidth (), params, (*f).total_cols);
	  let h: c_int = tty_child_size_param (f, Qheight (), params, (*f).total_lines);
	  let x: c_int = tty_child_pos_param (f, Qleft (), params, (*f).left_pos, w);
	  let y: c_int = tty_child_pos_param (f, Qtop (), params, (*f).top_pos, h);

	  if x != (*f).left_pos || y != (*f).top_pos
	    {
	      (*f).left_pos = x;
	      (*f).top_pos = y;
	      SET_FRAME_GARBAGED! (root_frame (f));
	    }

	  if w != (*f).total_cols || h != (*f).total_lines
	    { change_frame_size (f, w, h, false, false, false); }

	  let visible: Lisp_Object = Fassq (Qvisibility (), params);

	  if CONSP (visible)
	    {
	      if EQ (Fcdr (visible), Qicon ())
		  && EQ (iconify_child_frame, Qmake_invisible ())
		{ SET_FRAME_VISIBLE! (f, false); }
	      else
		{ SET_FRAME_VISIBLE! (f, !NILP (Fcdr (visible))); }
	    }

	  let no_special: Lisp_Object = Fassq (Qno_special_glyphs (), params);

	  if CONSP (no_special)
	    { FRAME_NO_SPECIAL_GLYPHS! (f, !NILP (Fcdr (no_special))); }
	}

      SAFE_FREE! (sa_count, sa_mark);
    }
  return Qnil ();
});

DEFUN! ("frame-char-height", Fframe_char_height, Sframe_char_height,
       0, 1, 0,
       doc: "Height in pixels of a line in the font in frame FRAME.\nIf FRAME is omitted or nil, the selected frame is used.\nFor a terminal frame, the value is always 1.  ",
       (frame: Lisp_Object),
       {
  /* C 4352-4358's `#ifdef HAVE_WINDOW_SYSTEM` arm is dead (P-08).  */
    return make_fixnum (1);
});

DEFUN! ("frame-char-width", Fframe_char_width, Sframe_char_width,
       0, 1, 0,
       doc: "Width in pixels of characters in the font in frame FRAME.\nIf FRAME is omitted or nil, the selected frame is used.\nOn a graphical screen, the width is the standard width of the default font.\nFor a terminal screen, the value is always 1.  ",
       (frame: Lisp_Object),
       {
  /* C 4371-4377's `#ifdef HAVE_WINDOW_SYSTEM` arm is dead (P-08).  */
    return make_fixnum (1);
});

DEFUN! ("frame-native-width", Fframe_native_width,
       Sframe_native_width, 0, 1, 0,
       doc: "Return FRAME's native width in pixels.\nFor a terminal frame, the result really gives the width in characters.\nIf FRAME is omitted or nil, the selected frame is used.\n\nIf you're interested only in the width of the text portion of the\nframe, see `frame-text-width' instead.  ",
       (frame: Lisp_Object),
       {
  let f: *mut frame = decode_any_frame (frame);

  /* C 4393-4397's `#ifdef HAVE_WINDOW_SYSTEM` arm is dead (P-08).  */
    return make_fixnum (FRAME_TOTAL_COLS! (f) as EMACS_INT);
});

DEFUN! ("frame-native-height", Fframe_native_height,
       Sframe_native_height, 0, 1, 0,
       doc: "Return FRAME's native height in pixels.\nIf FRAME is omitted or nil, the selected frame is used.  The exact value\nof the result depends on the window-system and toolkit in use:\n\nIn the Gtk+ and NS versions, it includes only any window (including the\nminibuffer or echo area), mode line, and header line.  It does not\ninclude the tool bar or menu bar.  With other graphical versions, it may\nalso include the tool bar and the menu bar.\n\nIf you're interested only in the height of the text portion of the\nframe, see `frame-text-height' instead.\n\nFor a text terminal, it includes the menu bar.  In this case, the\nresult is really in characters rather than pixels (i.e., is identical\nto `frame-height'). ",
       (frame: Lisp_Object),
       {
  let f: *mut frame = decode_any_frame (frame);

  /* C 4422-4426's `#ifdef HAVE_WINDOW_SYSTEM` arm is dead (P-08).  */
    return make_fixnum (FRAME_TOTAL_LINES! (f) as EMACS_INT);
});

DEFUN! ("tool-bar-pixel-width", Ftool_bar_pixel_width,
       Stool_bar_pixel_width, 0, 1, 0,
       doc: "Return width in pixels of FRAME's tool bar.\nThe result is greater than zero only when the tool bar is on the left\nor right side of FRAME.  If FRAME is omitted or nil, the selected frame\nis used.  ",
       (frame: Lisp_Object),
       {
  /* C 4438-4443's `#ifdef FRAME_TOOLBAR_WIDTH` arm is dead (P-08).  */
  return make_fixnum (0);
});

DEFUN! ("frame-text-cols", Fframe_text_cols, Sframe_text_cols, 0, 1, 0,
       doc: "Return width in columns of FRAME's text area.  ",
       (frame: Lisp_Object),
       {
  return make_fixnum (FRAME_COLS! (decode_any_frame (frame)) as EMACS_INT);
});

DEFUN! ("frame-text-lines", Fframe_text_lines, Sframe_text_lines, 0, 1, 0,
       doc: "Return height in lines of FRAME's text area.  ",
       (frame: Lisp_Object),
       {
  return make_fixnum (FRAME_LINES! (decode_any_frame (frame)) as EMACS_INT);
});

DEFUN! ("frame-total-cols", Fframe_total_cols, Sframe_total_cols, 0, 1, 0,
       doc: "Return number of total columns of FRAME.  ",
       (frame: Lisp_Object),
       {
  return make_fixnum (FRAME_TOTAL_COLS! (decode_any_frame (frame)) as EMACS_INT);
});

DEFUN! ("frame-total-lines", Fframe_total_lines, Sframe_total_lines, 0, 1, 0,
       doc: "Return number of total lines of FRAME.  ",
       (frame: Lisp_Object),
       {
  return make_fixnum (FRAME_TOTAL_LINES! (decode_any_frame (frame)) as EMACS_INT);
});

DEFUN! ("frame-text-width", Fframe_text_width, Sframe_text_width, 0, 1, 0,
       doc: "Return text area width of FRAME in pixels.  ",
       (frame: Lisp_Object),
       {
  return make_fixnum (FRAME_TEXT_WIDTH! (decode_any_frame (frame)) as EMACS_INT);
});

DEFUN! ("frame-text-height", Fframe_text_height, Sframe_text_height, 0, 1, 0,
       doc: "Return text area height of FRAME in pixels.  ",
       (frame: Lisp_Object),
       {
  return make_fixnum (FRAME_TEXT_HEIGHT! (decode_any_frame (frame)) as EMACS_INT);
});

DEFUN! ("frame-scroll-bar-width", Fscroll_bar_width, Sscroll_bar_width, 0, 1, 0,
       doc: "Return scroll bar width of FRAME in pixels.  ",
       (frame: Lisp_Object),
       {
  return make_fixnum (FRAME_SCROLL_BAR_AREA_WIDTH (decode_any_frame (frame)) as EMACS_INT);
});

DEFUN! ("frame-scroll-bar-height", Fscroll_bar_height, Sscroll_bar_height, 0, 1, 0,
       doc: "Return scroll bar height of FRAME in pixels.  ",
       (frame: Lisp_Object),
       {
  return make_fixnum (FRAME_SCROLL_BAR_AREA_HEIGHT (decode_any_frame (frame)) as EMACS_INT);
});

DEFUN! ("frame-fringe-width", Ffringe_width, Sfringe_width, 0, 1, 0,
       doc: "Return fringe width of FRAME in pixels.  ",
       (frame: Lisp_Object),
       {
  return make_fixnum (FRAME_TOTAL_FRINGE_WIDTH (decode_any_frame (frame)) as EMACS_INT);
});

DEFUN! ("frame-child-frame-border-width", Fframe_child_frame_border_width, Sframe_child_frame_border_width, 0, 1, 0,
       doc: "Return width of FRAME's child-frame border in pixels.\n If FRAME's `child-frame-border-width' parameter is nil, return FRAME's\n internal border width instead.  ",
       (frame: Lisp_Object),
       {
  let width: c_int = FRAME_CHILD_FRAME_BORDER_WIDTH (decode_any_frame (frame));

  if width < 0
    { return make_fixnum (FRAME_INTERNAL_BORDER_WIDTH (decode_any_frame (frame)) as EMACS_INT); }
  else
    { return make_fixnum (FRAME_CHILD_FRAME_BORDER_WIDTH (decode_any_frame (frame)) as EMACS_INT); }
});

DEFUN! ("frame-internal-border-width", Fframe_internal_border_width, Sframe_internal_border_width, 0, 1, 0,
       doc: "Return width of FRAME's internal border in pixels.  ",
       (frame: Lisp_Object),
       {
  return make_fixnum (FRAME_INTERNAL_BORDER_WIDTH (decode_any_frame (frame)) as EMACS_INT);
});

DEFUN! ("frame-right-divider-width", Fright_divider_width, Sright_divider_width, 0, 1, 0,
       doc: "Return width (in pixels) of vertical window dividers on FRAME.  ",
       (frame: Lisp_Object),
       {
  return make_fixnum (FRAME_RIGHT_DIVIDER_WIDTH (decode_any_frame (frame)) as EMACS_INT);
});

DEFUN! ("frame-bottom-divider-width", Fbottom_divider_width, Sbottom_divider_width, 0, 1, 0,
       doc: "Return width (in pixels) of horizontal window dividers on FRAME.  ",
       (frame: Lisp_Object),
       {
  return make_fixnum (FRAME_BOTTOM_DIVIDER_WIDTH (decode_any_frame (frame)) as EMACS_INT);
});

unsafe fn check_frame_pixels (size: Lisp_Object, pixelwise: Lisp_Object, item_size: c_int)
  -> c_int
{
  let mut sz: intmax_t = 0;
  let mut pixel_size: c_int = 0;
  let mut item_size = item_size;

  CHECK_INTEGER (size);
  if !NILP (pixelwise)
    { item_size = 1; }

  /* C's `ckd_mul (&pixel_size, sz, item_size)` multiplies in the destination's type: the
     product must fit in `int`.  The port keeps the wrapped value in `pixel_size` and
     reports the overflow (PORTING.md P-15: ckd_mul is the port's `overflowing_mul` plus
     the fit check, docs/ports/token-aliases.tsv's ckd_mul family).  */
  if !integer_to_intmax (size, &mut sz)
      || { let (product, mul_overflow) = sz.overflowing_mul (item_size as intmax_t);
	   pixel_size = product as c_int;
	   mul_overflow || product < c_int::MIN as intmax_t
		       || (c_int::MAX as intmax_t) < product }
    { args_out_of_range_3 (size, make_int ((INT_MIN / item_size) as intmax_t),
			   make_int ((INT_MAX / item_size) as intmax_t)); }

  return pixel_size;
}

DEFUN! ("set-frame-height", Fset_frame_height, Sset_frame_height, 2, 4,
       "(set-frame-property--interactive \"Frame height: \" (frame-height))",
       doc: "Set text height of frame FRAME to HEIGHT lines.\nOptional third arg PRETEND non-nil means that redisplay should use\nHEIGHT lines but that the idea of the actual height of the frame should\nnot be changed.\n\nOptional fourth argument PIXELWISE non-nil means that FRAME should be\nHEIGHT pixels high.  Note: When `frame-resize-pixelwise' is nil, some\nwindow managers may refuse to honor a HEIGHT that is not an integer\nmultiple of the default frame font height.\n\nWhen called interactively, HEIGHT is the numeric prefix and the\ncurrently selected frame will be set to this height.\n\nIf FRAME is nil, it defaults to the selected frame.  ",
       (frame: Lisp_Object, height: Lisp_Object, pretend: Lisp_Object,
	pixelwise: Lisp_Object),
       {
  let f: *mut frame = decode_live_frame (frame);
  let text_height: c_int
    = check_frame_pixels (height, pixelwise, FRAME_LINE_HEIGHT (f));

  /* With INHIBIT 1 pass correct text width to adjust_frame_size.  */
  adjust_frame_size
    (f, FRAME_TEXT_WIDTH! (f), text_height, 1, !NILP (pretend), Qheight ());

  return Qnil ();
});

DEFUN! ("set-frame-width", Fset_frame_width, Sset_frame_width, 2, 4,
       "(set-frame-property--interactive \"Frame width: \" (frame-width))",
       doc: "Set text width of frame FRAME to WIDTH columns.\nOptional third arg PRETEND non-nil means that redisplay should use WIDTH\ncolumns but that the idea of the actual width of the frame should not\nbe changed.\n\nOptional fourth argument PIXELWISE non-nil means that FRAME should be\nWIDTH pixels wide.  Note: When `frame-resize-pixelwise' is nil, some\nwindow managers may refuse to honor a WIDTH that is not an integer\nmultiple of the default frame font width.\n\nWhen called interactively, WIDTH is the numeric prefix and the\ncurrently selected frame will be set to this width.\n\nIf FRAME is nil, it defaults to the selected frame.  ",
       (frame: Lisp_Object, width: Lisp_Object, pretend: Lisp_Object,
	pixelwise: Lisp_Object),
       {
  let f: *mut frame = decode_live_frame (frame);
  let text_width: c_int
    = check_frame_pixels (width, pixelwise, FRAME_COLUMN_WIDTH (f));

  /* With INHIBIT 1 pass correct text height to adjust_frame_size.  */
  adjust_frame_size
    (f, text_width, FRAME_TEXT_HEIGHT! (f), 1, !NILP (pretend), Qwidth ());

  return Qnil ();
});

DEFUN! ("set-frame-size", Fset_frame_size, Sset_frame_size, 3, 4, 0,
       doc: "Set text size of FRAME to WIDTH by HEIGHT, measured in characters.\nOptional argument PIXELWISE non-nil means to measure in pixels.  Note:\nWhen `frame-resize-pixelwise' is nil, some window managers may refuse to\nhonor a WIDTH that is not an integer multiple of the default frame font\nwidth or a HEIGHT that is not an integer multiple of the default frame\nfont height.\n\nIf FRAME is nil, it defaults to the selected frame.  ",
       (frame: Lisp_Object, width: Lisp_Object, height: Lisp_Object,
	pixelwise: Lisp_Object),
       {
  let f: *mut frame = decode_live_frame (frame);
  let text_width: c_int
    = check_frame_pixels (width, pixelwise, FRAME_COLUMN_WIDTH (f));
  let text_height: c_int
    = check_frame_pixels (height, pixelwise, FRAME_LINE_HEIGHT (f));

  /* PRETEND is always false here.  */
  adjust_frame_size (f, text_width, text_height, 1, false, Qsize ());

  return Qnil ();
});

DEFUN! ("frame-position", Fframe_position,
       Sframe_position, 0, 1, 0,
       doc: "Return top left corner of FRAME in pixels.\nFRAME must be a live frame and defaults to the selected one.  The return\nvalue is a cons (x, y) of the coordinates of the top left corner of\nFRAME's outer frame, in pixels relative to an origin (0, 0) of FRAME's\ndisplay.\n\nNote that the values returned are not guaranteed to be accurate: The\nvalues depend on the underlying window system, and some systems add a\nconstant offset to the values.  ",
     (frame: Lisp_Object),
     {
  let f: *mut frame = decode_live_frame (frame);

  return Fcons (make_fixnum ((*f).left_pos as EMACS_INT),
		make_fixnum ((*f).top_pos as EMACS_INT));
});

DEFUN! ("set-frame-position", Fset_frame_position,
       Sset_frame_position, 3, 3, 0,
       doc: "Set position of FRAME to (X, Y).\nFRAME must be a live frame and defaults to the selected one.  X and Y,\nif positive, specify the coordinate of the left and top edge of FRAME's\nouter frame in pixels relative to an origin (0, 0) of FRAME's display\nor, if FRAME is a child frame, its parent frame.  If any of X or Y is\nnegative, it specifies the coordinates of the right or bottom edge of\nthe outer frame of FRAME relative to the right or bottom edge of FRAME's\ndisplay or parent frame.  ",
       (frame: Lisp_Object, x: Lisp_Object, y: Lisp_Object),
       {
  let f: *mut frame = decode_live_frame (frame);
  let xval: c_int = check_integer_range (x, INT_MIN as intmax_t, INT_MAX as intmax_t) as c_int;
  let yval: c_int = check_integer_range (y, INT_MIN as intmax_t, INT_MAX as intmax_t) as c_int;

  if FRAME_WINDOW_P (f)
    {
      /* C 4680-4687's `#ifdef HAVE_WINDOW_SYSTEM`/`#else` takes the `#else` arm.  */
      /* C's `(void) xval;`/`(void) yval;` discard the two values (the dired.rs shape).  */
      let _ = xval;
      let _ = yval;
    }
  else if is_tty_child_frame (f)
    {
      (*f).left_pos = xval;
      (*f).top_pos = yval;
    }

  return Qt ();
});

DEFUN! ("set-frame-size-and-position-pixelwise", Fset_frame_size_and_position_pixelwise,
       Sset_frame_size_and_position_pixelwise, 5, 6, 0,
       doc: "Set FRAME's size to WIDTH and HEIGHT and its position to (X, Y).\nFRAME must be a live frame and defaults to the selected one.\n\nWIDTH and HEIGHT must be positive integers and specify the new pixel\nwidth and height of FRAME's text area in pixels.  If WIDTH or HEIGHT do\nnot specify a value that is a multiple of FRAME's character sizes, you\nmay have to set `frame-resize-pixelwise' to a non-nil value in order to\nget the exact size in pixels.\n\nX and Y specify the coordinates of the left and top edge of FRAME's\nouter frame in pixels relative to an origin (0, 0) of FRAME's display or\nparent frame.  Negative values mean the top or left edge may be outside\nthe display or parent frame.\n\nGRAVITY specifies the new gravity of FRAME and must be a value in the\nrange 0..10.  It defaults to 1.\n\nThis function uses any existing backend of the toolkit to resize and\nmove FRAME in one compound step.  If the backend does not provide such a\nfunction, it calls `set-frame-size' followed by `set-frame-position'\ninstead.  See 'set-frame-size-and-position'.  ",
       (frame: Lisp_Object, width: Lisp_Object, height: Lisp_Object,
	x: Lisp_Object, y: Lisp_Object, gravity: Lisp_Object),
       {
  let f: *mut frame = decode_live_frame (frame);

  if NILP (gravity)
    { (*f).win_gravity = 1; }
  else
    { (*f).win_gravity = check_integer_range (gravity, 0, 10) as c_int; }

  if FRAME_WINDOW_P (f)
      && (*FRAME_TERMINAL! (f)).set_window_size_and_position_hook.is_some ()
    {
      let text_width: c_int = check_integer_range (width, 1, INT_MAX as intmax_t) as c_int;
      let text_height: c_int = check_integer_range (height, 1, INT_MAX as intmax_t) as c_int;

      (*f).left_pos = check_integer_range (x, INT_MIN as intmax_t, INT_MAX as intmax_t) as c_int;
      (*f).top_pos = check_integer_range (y, INT_MIN as intmax_t, INT_MAX as intmax_t) as c_int;

      adjust_frame_size (f, text_width, text_height, 1, false,
			 Qsize_and_position ());
    }
  else
    {
      Fset_frame_size (frame, width, height, Qt ());

      let left_pos: c_int = check_integer_range (x, INT_MIN as intmax_t, INT_MAX as intmax_t) as c_int;
      let top_pos: c_int = check_integer_range (y, INT_MIN as intmax_t, INT_MAX as intmax_t) as c_int;

      let left: Lisp_Object
	= Fcons (Qleft (), if left_pos < 0 { list2 (Qplus (), x) } else { x });
      let top: Lisp_Object
	= Fcons (Qtop (), if top_pos < 0 { list2 (Qplus (), y) } else { y });

      Fmodify_frame_parameters (frame, list2 (left, top));
    }

  return Qnil ();
});

DEFUN! ("frame-window-state-change", Fframe_window_state_change,
       Sframe_window_state_change, 0, 1, 0,
       doc: "Return t if FRAME's window state change flag is set, nil otherwise.\nFRAME must be a live frame and defaults to the selected one.\n\nIf FRAME's window state change flag is set, the default values of\n`window-state-change-functions' and `window-state-change-hook' will be\nrun during next redisplay, regardless of whether a window state change\nactually occurred on FRAME or not.  After that, the value of this flag\nis reset.  ",
     (frame: Lisp_Object),
     {
  return if FRAME_WINDOW_STATE_CHANGE! (decode_live_frame (frame)) { Qt () } else { Qnil () };
});

DEFUN! ("set-frame-window-state-change", Fset_frame_window_state_change,
       Sset_frame_window_state_change, 0, 2, 0,
       doc: "Set FRAME's window state change flag according to ARG.\nSet FRAME's window state change flag if ARG is non-nil, reset it\notherwise.\n\nIf FRAME's window state change flag is set, the default values of\n`window-state-change-functions' and `window-state-change-hook' will be\nrun during next redisplay, regardless of whether a window state change\nactually occurred on FRAME or not.  After that, the value of FRAME's\nwindow state change flag is reset.  ",
     (frame: Lisp_Object, arg: Lisp_Object),
     {
  let f: *mut frame = decode_live_frame (frame);

  /* C's `(FRAME_WINDOW_STATE_CHANGE (f) = !NILP (arg)) ? Qt : Qnil` assigns and then
     tests in one expression; Rust's assignment is not an expression, so the port keeps
     C's two effects and the test in a block expression (docs/ports/frame.md).  */
  return if { let window_state_change = !NILP (arg);
	      FRAME_WINDOW_STATE_CHANGE! (f, window_state_change);
	      window_state_change } { Qt () } else { Qnil () };
});

DEFUN! ("frame-scale-factor", Fframe_scale_factor, Sframe_scale_factor,
       0, 1, 0,
       doc: "Return FRAMEs scale factor.\nIf FRAME is omitted or nil, the selected frame is used.\nThe scale factor is the amount by which a logical pixel size must be\nmultiplied to find the real number of pixels.  ",
     (frame: Lisp_Object),
     {
  let f: *mut frame = decode_live_frame (frame);

  return make_float (if !f.is_null () { FRAME_SCALE_FACTOR (f) as f64 } else { 1f64 });
});

/***********************************************************************
				Frame Parameters
 ***********************************************************************/

/* Connect the frame-parameter names for frames to the ways of passing
   the parameter values to the window system.

   The name of a parameter, a Lisp symbol, has an `x-frame-parameter'
   property which is its index in this table.  This is initialized in
   syms_of_frame.  */

#[repr(C)]
pub struct frame_parm_table {
  pub name: *const c_char,
  pub sym: c_int,
}

/* If you're adding a new frame parameter here, consider if it makes sense
   for the user to customize it via `initial-frame-alist' and the like.
   If it does, add it to `frame--special-parameters' in frame.el, in order
   to provide completion in the Customize UI for the new parameter.  */
/* C's `SYMBOL_INDEX (Qauto_raise)` is the token-pasting macro `iQauto_raise`, which the
   generated globals provide as a constant (PORTING.md P-17; the KVAR precedent).  */
static mut frame_parms: [frame_parm_table; 51] =
[
  frame_parm_table { name: c"auto-raise".as_ptr (), sym: iQauto_raise as c_int },
  frame_parm_table { name: c"auto-lower".as_ptr (), sym: iQauto_lower as c_int },
  frame_parm_table { name: c"background-color".as_ptr (), sym: iQbackground_color as c_int },
  frame_parm_table { name: c"border-color".as_ptr (), sym: iQborder_color as c_int },
  frame_parm_table { name: c"border-width".as_ptr (), sym: iQborder_width as c_int },
  frame_parm_table { name: c"cursor-color".as_ptr (), sym: iQcursor_color as c_int },
  frame_parm_table { name: c"cursor-type".as_ptr (), sym: iQcursor_type as c_int },
  frame_parm_table { name: c"font".as_ptr (), sym: iQfont as c_int },
  frame_parm_table { name: c"foreground-color".as_ptr (), sym: iQforeground_color as c_int },
  frame_parm_table { name: c"icon-name".as_ptr (), sym: iQicon_name as c_int },
  frame_parm_table { name: c"icon-type".as_ptr (), sym: iQicon_type as c_int },
  frame_parm_table { name: c"child-frame-border-width".as_ptr (), sym: iQchild_frame_border_width as c_int },
  frame_parm_table { name: c"internal-border-width".as_ptr (), sym: iQinternal_border_width as c_int },
  frame_parm_table { name: c"right-divider-width".as_ptr (), sym: iQright_divider_width as c_int },
  frame_parm_table { name: c"bottom-divider-width".as_ptr (), sym: iQbottom_divider_width as c_int },
  frame_parm_table { name: c"menu-bar-lines".as_ptr (), sym: iQmenu_bar_lines as c_int },
  frame_parm_table { name: c"mouse-color".as_ptr (), sym: iQmouse_color as c_int },
  frame_parm_table { name: c"name".as_ptr (), sym: iQname as c_int },
  frame_parm_table { name: c"scroll-bar-width".as_ptr (), sym: iQscroll_bar_width as c_int },
  frame_parm_table { name: c"scroll-bar-height".as_ptr (), sym: iQscroll_bar_height as c_int },
  frame_parm_table { name: c"title".as_ptr (), sym: iQtitle as c_int },
  frame_parm_table { name: c"unsplittable".as_ptr (), sym: iQunsplittable as c_int },
  frame_parm_table { name: c"vertical-scroll-bars".as_ptr (), sym: iQvertical_scroll_bars as c_int },
  frame_parm_table { name: c"horizontal-scroll-bars".as_ptr (), sym: iQhorizontal_scroll_bars as c_int },
  frame_parm_table { name: c"visibility".as_ptr (), sym: iQvisibility as c_int },
  frame_parm_table { name: c"tab-bar-lines".as_ptr (), sym: iQtab_bar_lines as c_int },
  frame_parm_table { name: c"tool-bar-lines".as_ptr (), sym: iQtool_bar_lines as c_int },
  frame_parm_table { name: c"scroll-bar-foreground".as_ptr (), sym: iQscroll_bar_foreground as c_int },
  frame_parm_table { name: c"scroll-bar-background".as_ptr (), sym: iQscroll_bar_background as c_int },
  frame_parm_table { name: c"screen-gamma".as_ptr (), sym: iQscreen_gamma as c_int },
  frame_parm_table { name: c"line-spacing".as_ptr (), sym: iQline_spacing as c_int },
  frame_parm_table { name: c"left-fringe".as_ptr (), sym: iQleft_fringe as c_int },
  frame_parm_table { name: c"right-fringe".as_ptr (), sym: iQright_fringe as c_int },
  frame_parm_table { name: c"wait-for-wm".as_ptr (), sym: iQwait_for_wm as c_int },
  frame_parm_table { name: c"fullscreen".as_ptr (), sym: iQfullscreen as c_int },
  frame_parm_table { name: c"font-backend".as_ptr (), sym: iQfont_backend as c_int },
  frame_parm_table { name: c"alpha".as_ptr (), sym: iQalpha as c_int },
  frame_parm_table { name: c"sticky".as_ptr (), sym: iQsticky as c_int },
  frame_parm_table { name: c"tool-bar-position".as_ptr (), sym: iQtool_bar_position as c_int },
  frame_parm_table { name: c"inhibit-double-buffering".as_ptr (), sym: iQinhibit_double_buffering as c_int },
  frame_parm_table { name: c"undecorated".as_ptr (), sym: iQundecorated as c_int },
  frame_parm_table { name: c"parent-frame".as_ptr (), sym: iQparent_frame as c_int },
  frame_parm_table { name: c"skip-taskbar".as_ptr (), sym: iQskip_taskbar as c_int },
  frame_parm_table { name: c"no-focus-on-map".as_ptr (), sym: iQno_focus_on_map as c_int },
  frame_parm_table { name: c"no-accept-focus".as_ptr (), sym: iQno_accept_focus as c_int },
  frame_parm_table { name: c"z-group".as_ptr (), sym: iQz_group as c_int },
  frame_parm_table { name: c"override-redirect".as_ptr (), sym: iQoverride_redirect as c_int },
  frame_parm_table { name: c"no-special-glyphs".as_ptr (), sym: iQno_special_glyphs as c_int },
  frame_parm_table { name: c"alpha-background".as_ptr (), sym: iQalpha_background as c_int },
  frame_parm_table { name: c"borders-respect-alpha-background".as_ptr (),
		     sym: iQborders_respect_alpha_background as c_int },
  frame_parm_table { name: c"use-frame-synchronization".as_ptr (), sym: iQuse_frame_synchronization as c_int },
  /* C 4880-4886's `#ifdef HAVE_X_WINDOWS`/`NS_IMPL_COCOA` entries are dead (P-08).  */
];


/* ---------------------------------------------------------------------------------------------
   C 4889-6970: `#ifdef HAVE_WINDOW_SYSTEM`, the GUI frame-geometry implementation.  The frozen
   configuration leaves the condition false (macros.tsv 1677; `python3 tools/check-cond-eval.py
   src/frame.c` counts it among the 105 conditions it settles), so none of this text is compiled
   by anything this repository builds.  P-86 ① applies: the region is carried **in full, both
   arms of every `#if`**, each item behind C's own condition spelled as a lower-case feature
   (`have_window_system`, and the nested `have_x_windows`, `have_ns`, `use_x_toolkit`, …), and
   `docs/ports/frame.md` records it with a C line-range table and the readings rather than
   checkboxes.  The run is carried in two halves and is now complete: C 4889-5919 (through
   `gui_set_unsplittable`, the round before this one) and C 5921-6970 (`gui_set_vertical_scroll_bars`
   through `gui_figure_window_size`, this round); C 6971 is where the `#ifdef` closes.
   --------------------------------------------------------------------------------------------- */

/* Names this arm takes from headers the unit's include graph does not bring in yet: `*gui.h`
   (per platform) and the toolkit headers.  Each is defined here with C's own text named, and
   `docs/ports/frame.md`'s Deviations lists them as mirror gaps (PORTING.md P-72 rule 3, the
   `androidmenu.rs` shape).  */

/* dispextern.h 189 / haikugui.h 39 / pgtkgui.h 52 / w32gui.h 36 / nsterm.h: `#define
   WINDOW_HANDLE_UINTPTR(h) ((uintptr_t) (h))` - the arms agree on the cast.  */
#[cfg (feature = "have_window_system")]
macro_rules! WINDOW_HANDLE_UINTPTR { ($h:expr) => { ($h) as usize }; }

/* The `"%"PRIuMAX` literal of C 5463/5471: the frozen target's `PRIuMAX` is `"ju"` (the
   `dispnew.rs` 69-72 reading, measured with clang).  */
#[cfg (feature = "have_window_system")]
macro_rules! PRIuMAX { () => { "ju" } }




/* C 5157-5161's geometry constants (`XNegative`, `YNegative`, the four gravities and the four
   position/size hints) are the GUI headers' (nsgui.h, pgtkgui.h, w32gui.h, androidgui.h,
   haikugui.h), and this unit reaches them through `TERM_HEADER`'s mirror now
   (`include/androidgui.rs` 245-277 carries all ten) - the unit's own substitutes were `E0428`
   once that edge existed (measured, 8 names).  */

/* The mirror-gap functions this half calls; each `/// C:` is the header's own prototype
   (PORTING.md P-72 rule 3, the `frame.rs` use-site shape above).  */
#[cfg (feature = "have_window_system")]
unsafe extern "C" {
  /// C: lisp.h 4542: extern Lisp_Object make_formatted_string (const char *, ...);
  pub fn make_formatted_string (format: *const c_char, ...) -> Lisp_Object;
  /// C: lisp.h 607: extern int check_int_nonnegative (Lisp_Object);
  pub fn check_int_nonnegative (x: Lisp_Object) -> c_int;
  /// C: dispextern.h 3896: extern void redraw_frame (struct frame *);
  pub fn redraw_frame (f: *mut frame);
  /// C: dispextern.h 3802: void clear_face_cache (bool);
  pub fn clear_face_cache (thorough: bool);
  /// C: dispextern.h 3907: void clear_current_matrices (struct frame *f);
  pub fn clear_current_matrices (f: *mut frame);
  /// C: dispextern.h 3823: void recompute_basic_faces (struct frame *);
  pub fn recompute_basic_faces (f: *mut frame);
  /// C: dispextern.h 3729: extern void clear_image_cache (struct frame *, Lisp_Object);
  pub fn clear_image_cache (f: *mut frame, filter: Lisp_Object);
  /// C: dispextern.h 3819: extern struct image_cache *share_image_cache (struct frame *f);
  pub fn share_image_cache (f: *mut frame) -> *mut image_cache;
  /// C: dispextern.h 3833: void free_all_realized_faces (Lisp_Object);
  pub fn free_all_realized_faces (frame: Lisp_Object);
  /// C: font.h 883: extern Lisp_Object font_open_by_name (struct frame *f, Lisp_Object name);
  pub fn font_open_by_name (f: *mut frame, name: Lisp_Object) -> Lisp_Object;
  /// C: font.h 865: extern Lisp_Object font_spec_from_name (Lisp_Object font_name);
  pub fn font_spec_from_name (font_name: Lisp_Object) -> Lisp_Object;
  /// C: font.h 860: extern bool font_match_p (Lisp_Object spec, Lisp_Object font);
  pub fn font_match_p (spec: Lisp_Object, font: Lisp_Object) -> bool;
  /// C: font.h 907: extern Lisp_Object font_update_drivers (struct frame *f, Lisp_Object list);
  pub fn font_update_drivers (f: *mut frame, list: Lisp_Object) -> Lisp_Object;
  /// C: frame.h 1886: extern void x_wm_set_icon_position (struct frame *, int, int);
  pub fn x_wm_set_icon_position (f: *mut frame, icon_x: c_int, icon_y: c_int);
}

/* Enumeration type for switch in frame_float.  */
/* C 4891-4898: `enum frame_float_type`.  C's enum ABI is `c_int` (PORTING.md 1.3; the mirror
   spells `internal_border_part` the same way).  */
#[cfg (feature = "have_window_system")]
pub type frame_float_type = c_int;
#[cfg (feature = "have_window_system")]
const FRAME_FLOAT_WIDTH: frame_float_type = 0;              // C 4894
#[cfg (feature = "have_window_system")]
const FRAME_FLOAT_HEIGHT: frame_float_type = 1;             // C 4895
#[cfg (feature = "have_window_system")]
const FRAME_FLOAT_LEFT: frame_float_type = 2;               // C 4896
#[cfg (feature = "have_window_system")]
const FRAME_FLOAT_TOP: frame_float_type = 3;                // C 4897

/* C 4935-4937: `frame_float`'s function-scope statics (`static unsigned parent_width,
   parent_height;` / `static int parent_left, parent_top;` / `static unsigned
   outer_minus_text_width, outer_minus_text_height;`).  Rust has no function-scope `static`, so
   the declarations move to file scope inside the same gate; C's storage duration is static and
   zero-initialised, which is what the initialisers spell.  */
#[cfg (feature = "have_window_system")]
static mut parent_width: c_uint = 0;
#[cfg (feature = "have_window_system")]
static mut parent_height: c_uint = 0;
#[cfg (feature = "have_window_system")]
static mut parent_left: c_int = 0;
#[cfg (feature = "have_window_system")]
static mut parent_top: c_int = 0;
#[cfg (feature = "have_window_system")]
static mut outer_minus_text_width: c_uint = 0;
#[cfg (feature = "have_window_system")]
static mut outer_minus_text_height: c_uint = 0;

/**
 * frame_float:
 *
 * Process the value VAL of the float type frame parameter 'width',
 * 'height', 'left', or 'top' specified via a frame_float_type
 * enumeration type WHAT for frame F.  Such parameters relate the outer
 * size or position of F to the size of the F's display or parent frame
 * which have to be both available in some way.
 *
 * The return value is a size or position value in pixels.  VAL must be
 * in the range 0.0 to 1.0 where a width/height of 0.0 means to return 0
 * and 1.0 means to return the full width/height of the display/parent.
 * For positions, 0.0 means position in the left/top corner of the
 * display/parent while 1.0 means to position at the right/bottom corner
 * of the display/parent frame.
 *
 * Set PARENT_DONE and OUTER_DONE to avoid recalculation of the outer
 * size or parent or display attributes when more float parameters are
 * calculated in a row: -1 means not processed yet, 0 means processing
 * failed, 1 means processing succeeded.
 *
 * Return DEFAULT_VALUE when processing fails for whatever reason with
 * one exception: When calculating F's outer edges fails (probably
 * because F has not been created yet) return the difference between F's
 * native and text size.
 */
/* C 4926-5071: `frame_float`.  `static` in C, so no `#[unsafe(no_mangle)]` (PORTING.md 0.2).  */
#[cfg (feature = "have_window_system")]
unsafe fn frame_float (f: *mut frame, val: Lisp_Object, what: frame_float_type,
                       parent_done: *mut c_int, outer_done: *mut c_int,
                       default_value: c_int)
  -> c_int
{
  let d_val: f64 = XFLOAT_DATA (val);

  if d_val < 0.0 || d_val > 1.0
    /* Invalid VAL.  */
    { return default_value; }
  else
    {
      let p: *mut frame = FRAME_PARENT_FRAME (f);

      if *parent_done == 1
        { }                                    /* C 4939: the empty statement.  */
      else if !p.is_null ()
        {
          parent_width = FRAME_PIXEL_WIDTH! (p) as c_uint;   // C 4937: `unsigned`
          parent_height = FRAME_PIXEL_HEIGHT! (p) as c_uint;
          *parent_done = 1;
        }
      else
        {
          if *parent_done == 0
            /* No workarea available.  */
            { return default_value; }
          else if *parent_done == -1
            {
              let monitor_attributes: Lisp_Object;
              let workarea: Lisp_Object;
              let mut frame: Lisp_Object = Qnil ();

              XSETFRAME! (frame, f);
              monitor_attributes = calln! (Qframe_monitor_attributes (), frame);
              if NILP (monitor_attributes)
                {
                  /* No monitor attributes available.  */
                  *parent_done = 0;

                  return default_value;
                }

              workarea = Fcdr (Fassq (Qworkarea (), monitor_attributes));
              if NILP (workarea)
                {
                  /* No workarea available.  */
                  *parent_done = 0;

                  return default_value;
                }

              /* Workarea available.  */
              parent_left = XFIXNUM (Fnth (make_fixnum (0), workarea)) as c_int;
              parent_top = XFIXNUM (Fnth (make_fixnum (1), workarea)) as c_int;
              parent_width = XFIXNUM (Fnth (make_fixnum (2), workarea)) as c_uint;
              parent_height = XFIXNUM (Fnth (make_fixnum (3), workarea)) as c_uint;
              *parent_done = 1;
            }
        }

      if *outer_done == 1
        { }                                    /* C 4965: the empty statement.  */
      else if FRAME_UNDECORATED (f) != 0
        {
          outer_minus_text_width
            = (FRAME_PIXEL_WIDTH! (f) as isize - FRAME_TEXT_WIDTH! (f) as isize) as c_uint;
          outer_minus_text_height
            = (FRAME_PIXEL_HEIGHT! (f) as isize - FRAME_TEXT_HEIGHT! (f) as isize) as c_uint;
          *outer_done = 1;
        }
      else if *outer_done == 0
        /* No outer size available.  */
        { return default_value; }
      else if *outer_done == -1
        {
          let mut frame: Lisp_Object = Qnil ();
          let outer_edges: Lisp_Object;

          XSETFRAME! (frame, f);
          outer_edges = calln! (Qframe_edges (), frame, Qouter_edges ());

          if !NILP (outer_edges)
            {
              outer_minus_text_width
                = (XFIXNUM (Fnth (make_fixnum (2), outer_edges))
                   - XFIXNUM (Fnth (make_fixnum (0), outer_edges))
                   - FRAME_TEXT_WIDTH! (f) as isize) as c_uint;
              outer_minus_text_height
                = (XFIXNUM (Fnth (make_fixnum (3), outer_edges))
                   - XFIXNUM (Fnth (make_fixnum (1), outer_edges))
                   - FRAME_TEXT_HEIGHT! (f) as isize) as c_uint;
            }
          else
            {
              /* If we can't get any outer edges, proceed as if the frame
                 were undecorated.  */
              outer_minus_text_width
                = (FRAME_PIXEL_WIDTH! (f) as isize - FRAME_TEXT_WIDTH! (f) as isize) as c_uint;
              outer_minus_text_height
                = (FRAME_PIXEL_HEIGHT! (f) as isize - FRAME_TEXT_HEIGHT! (f) as isize) as c_uint;
            }

          *outer_done = 1;
        }

      match what
        {
          FRAME_FLOAT_WIDTH =>
            { return (parent_width as f64 * d_val - outer_minus_text_width as f64) as c_int; }

          FRAME_FLOAT_HEIGHT =>
            { return (parent_height as f64 * d_val - outer_minus_text_height as f64) as c_int; }

          FRAME_FLOAT_LEFT =>
            {
              let rest_width: c_int = (parent_width as c_int
                                       - FRAME_TEXT_WIDTH! (f)
                                       - outer_minus_text_width as c_int);

              if !p.is_null ()
                { return if rest_width <= 0 { 0 }
                         else { (d_val * rest_width as f64) as c_int }; }
              else
                { return if rest_width <= 0 { parent_left }
                         else { parent_left + (d_val * rest_width as f64) as c_int }; }
            }
          FRAME_FLOAT_TOP =>
            {
              let rest_height: c_int = (parent_height as c_int
                                        - FRAME_TEXT_HEIGHT! (f)
                                        - outer_minus_text_height as c_int);

              if !p.is_null ()
                { return if rest_height <= 0 { 0 }
                         else { (d_val * rest_height as f64) as c_int }; }
              else
                { return if rest_height <= 0 { parent_top }
                         else { parent_top + (d_val * rest_height as f64) as c_int }; }
            }
          _ =>
            emacs_abort ()
        }
    }
}

/* Handle frame parameter change with frame parameter handler.
   F is the frame whose frame parameter was changed.
   PROP is the name of the frame parameter.
   VAL and OLD_VALUE are the current and the old value of the
   frame parameter.  */
/* C 5079-5090: `handle_frame_param`.  C's `frame_parm_handler` is dispextern.h 3063's typedef and
   `FRAME_RIF (f)->frame_parm_handlers` is the `redisplay_interface` member the mirror keeps as
   `*mut c_void` (P-80's untyped-pointer rule; `dispextern.rs` 3511), so the port reads the slot
   as `*mut c_void` and casts it to the typedef spelled here.  */
#[cfg (feature = "have_window_system")]
type frame_parm_handler = Option<unsafe extern "C" fn (f: *mut frame, val: Lisp_Object,
                                                        oldval: Lisp_Object)>;

#[cfg (feature = "have_window_system")]
unsafe fn handle_frame_param (f: *mut frame, prop: Lisp_Object, val: Lisp_Object,
                              old_value: Lisp_Object)
{
  let param_index: Lisp_Object = Fget (prop, Qx_frame_parameter ());
  if FIXNATP (param_index) && XFIXNAT (param_index) < countof! (frame_parms) as EMACS_INT
    {
      if !FRAME_RIF! (f).is_null ()
        {
          let handlers: *mut frame_parm_handler
            = (*((FRAME_RIF! (f)) as *mut redisplay_interface)).frame_parm_handlers as *mut frame_parm_handler;
          let handler: frame_parm_handler
            = *handlers.offset (XFIXNAT (param_index) as isize);

          if let Some (handler) = handler
            { handler (f, val, old_value); }
        }
    }
}

/* Change the parameters of frame F as specified by ALIST.
   If a parameter is not specially recognized, do nothing special;
   otherwise call the `gui_set_...' function for that parameter.
   Except for certain geometry properties, always call store_frame_param
   to store the new value in the parameter alist.

   DEFAULT_PARAMETER should be set if the alist was not specified by
   the user, or by the face code to set the `font' parameter.  In that
   case, the `font-parameter' frame parameter should not be changed,
   so dynamic-setting.el can restore the user's selected font
   correctly.  */
/* C 5108-5394: `gui_set_frame_parameters_1`.  Non-`static` in C, so `#[unsafe(no_mangle)]`
   (PORTING.md 0.2).  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gui_set_frame_parameters_1 (f: *mut frame, mut alist: Lisp_Object,
                                                      default_parameter: bool)
{
  let mut tail: Lisp_Object;
  let mut frame: Lisp_Object = Qnil ();

  /* Neither of these values should be used.  */
  let mut width: c_int = -1;
  let mut height: c_int = -1;
  let mut width_change: bool = false;
  let mut height_change: bool = false;

  /* Same here.  */
  let mut left: Lisp_Object = Qnil ();
  let mut top: Lisp_Object = Qnil ();

  /* Same with these.  */
  let mut icon_left: Lisp_Object = Qnil ();
  let mut icon_top: Lisp_Object = Qnil ();

  /* And with this.  */
  let mut fullscreen: Lisp_Object = Qnil ();   /* C: `Lisp_Object fullscreen UNINIT;` */
  let mut fullscreen_change: bool = false;

  /* Record in these vectors all the parms specified.  */
  let mut parms: *mut Lisp_Object = std::ptr::null_mut ();
  let values: *mut Lisp_Object;
  let mut i: ptrdiff_t;
  let mut j: ptrdiff_t;
  let mut size: ptrdiff_t;
  let mut left_no_change: bool = false;
  let mut top_no_change: bool = false;
  /* C 5133-5135: the two `#ifdef HAVE_X_WINDOWS` declarations.  */
  #[cfg (feature = "have_x_windows")]
  let mut icon_left_no_change: bool = false;
  #[cfg (feature = "have_x_windows")]
  let mut icon_top_no_change: bool = false;
  let mut parent_done: c_int = -1;
  let mut outer_done: c_int = -1;

  XSETFRAME! (frame, f);
  size = 0;
  tail = alist;
  while CONSP (tail)
    {
      size += 1;
      tail = XCDR (tail);
    }
  CHECK_LIST_END (tail, alist);

  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
  SAFE_ALLOCA_LISP! (sa_avail, parms, 2 * size);
  values = parms.offset (size);

  /* Extract parm names and values into those vectors.  */

  i = 0;
  j = size - 1;
  tail = alist;
  while CONSP (tail)
    {
      let elt: Lisp_Object = XCAR (tail);
      let prop: Lisp_Object = Fcar (elt);
      let val: Lisp_Object = Fcdr (elt);

      /* Some properties are independent of other properties, but other
	 properties are dependent upon them.  These special properties
	 are foreground_color, background_color (affects cursor_color)
	 and font (affects fringe widths); they're recorded starting
	 from the end of PARMS and VALUES to process them first by using
	 reverse iteration.  */

      if EQ (prop, Qforeground_color ())
	  || EQ (prop, Qbackground_color ())
	  || EQ (prop, Qfont ())
	{
	  *parms.offset (j) = prop;
	  *values.offset (j) = val;
	  j -= 1;
	}
      else
	{
	  *parms.offset (i) = prop;
	  *values.offset (i) = val;
	  i += 1;
	}

      tail = XCDR (tail);
    }

  /* TAIL and ALIST are not used again below here.  */
  alist = Qnil ();
  tail = Qnil ();

  top = Qunbound ();
  left = Qunbound ();
  icon_left = Qunbound ();
  icon_top = Qunbound ();

  /* Reverse order is used to make sure that special
     properties noticed above are processed first.  */
  i = size - 1;
  while i >= 0
    {
      let prop: Lisp_Object;
      let val: Lisp_Object;

      prop = *parms.offset (i);
      val = *values.offset (i);

      if EQ (prop, Qwidth ())
        {
	  width_change = true;

	  if RANGED_FIXNUMP (0, val, INT_MAX as EMACS_INT)
	    { width = XFIXNAT (val) as c_int * FRAME_COLUMN_WIDTH (f); }
	  else if CONSP (val) && EQ (XCAR (val), Qtext_pixels ())
		   && RANGED_FIXNUMP (0, XCDR (val), INT_MAX as EMACS_INT)
	    { width = XFIXNAT (XCDR (val)) as c_int; }
	  else if FLOATP (val)
	    { width = frame_float (f, val, FRAME_FLOAT_WIDTH, &mut parent_done,
				   &mut outer_done, -1); }
	  else
	    { width_change = false; }
        }
      else if EQ (prop, Qheight ())
        {
	  height_change = true;

	  if RANGED_FIXNUMP (0, val, INT_MAX as EMACS_INT)
	    { height = XFIXNAT (val) as c_int * FRAME_LINE_HEIGHT (f); }
	  else if CONSP (val) && EQ (XCAR (val), Qtext_pixels ())
		   && RANGED_FIXNUMP (0, XCDR (val), INT_MAX as EMACS_INT)
	    { height = XFIXNAT (XCDR (val)) as c_int; }
	  else if FLOATP (val)
	    { height = frame_float (f, val, FRAME_FLOAT_HEIGHT, &mut parent_done,
				    &mut outer_done, -1); }
	  else
	    { height_change = false; }
        }
      else if EQ (prop, Qtop ())
	{ top = val; }
      else if EQ (prop, Qleft ())
	{ left = val; }
      else if EQ (prop, Qicon_top ())
	{ icon_top = val; }
      else if EQ (prop, Qicon_left ())
	{ icon_left = val; }
      else if EQ (prop, Qfullscreen ())
	{
	  fullscreen = val;
	  fullscreen_change = true;
	}
      else
	{
	  let old_value: Lisp_Object = get_frame_param (f, prop);

	  store_frame_param (f, prop, val);
	  handle_frame_param (f, prop, val, old_value);

	  if !default_parameter && EQ (prop, Qfont ())
	    /* The user manually specified the `font' frame parameter.
	       Save that parameter for future use by the
	       dynamic-setting code.  */
	    { store_frame_param (f, Qfont_parameter (), val); }
	}

      i -= 1;
    }

  /* Don't die if just one of these was set.  */
  if BASE_EQ (left, Qunbound ())
    {
      left_no_change = true;
      if (*f).left_pos < 0
	{ left = list2 (Qplus (), make_fixnum ((*f).left_pos as EMACS_INT)); }
      else
	{ XSETINT! (left, (*f).left_pos as EMACS_INT); }
    }
  if BASE_EQ (top, Qunbound ())
    {
      top_no_change = true;
      if (*f).top_pos < 0
	{ top = list2 (Qplus (), make_fixnum ((*f).top_pos as EMACS_INT)); }
      else
	{ XSETINT! (top, (*f).top_pos as EMACS_INT); }
    }

  /* If one of the icon positions was not set, preserve or default it.  */
  if ! TYPE_RANGED_FIXNUMP (INT_MIN as EMACS_INT, icon_left, INT_MAX as EMACS_INT)
    {
      /* C 5270-5272: `#ifdef HAVE_X_WINDOWS`.  */
      #[cfg (feature = "have_x_windows")]
      { icon_left_no_change = true; }
      icon_left = Fcdr (Fassq (Qicon_left (), (*f).param_alist));
      if NILP (icon_left)
	{ XSETINT! (icon_left, 0); }
    }
  if ! TYPE_RANGED_FIXNUMP (INT_MIN as EMACS_INT, icon_top, INT_MAX as EMACS_INT)
    {
      /* C 5279-5281: `#ifdef HAVE_X_WINDOWS`.  */
      #[cfg (feature = "have_x_windows")]
      { icon_top_no_change = true; }
      icon_top = Fcdr (Fassq (Qicon_top (), (*f).param_alist));
      if NILP (icon_top)
	{ XSETINT! (icon_top, 0); }
    }

  if width_change || height_change
    {
      let parameter: Lisp_Object;

      if width_change
	{
	  if height_change
	    { parameter = Qsize (); }
	  else
	    {
	      height = FRAME_TEXT_HEIGHT! (f);
	      parameter = Qwidth ();
	    }
	}
      else
	{
	  width = FRAME_TEXT_WIDTH! (f);
	  parameter = Qheight ();
	}

      adjust_frame_size (f, width, height, 1, false, parameter);
    }

  if (!NILP (left) || !NILP (top))
      && ! (left_no_change && top_no_change)
      && ! (FIXNUMP (left) && XFIXNUM (left) == (*f).left_pos as EMACS_INT
	    && FIXNUMP (top) && XFIXNUM (top) == (*f).top_pos as EMACS_INT)
    {
      let mut leftpos: c_int = 0;
      let mut toppos: c_int = 0;

      /* Record the signs.  */
      (*f).size_hint_flags &= ! (XNegative | YNegative);
      if EQ (left, Qminus ())
	{ (*f).size_hint_flags |= XNegative; }
      else if TYPE_RANGED_FIXNUMP (INT_MIN as EMACS_INT, left, INT_MAX as EMACS_INT)
	{
	  leftpos = XFIXNUM (left) as c_int;
	  if leftpos < 0
	    { (*f).size_hint_flags |= XNegative; }
	}
      else if CONSP (left) && EQ (XCAR (left), Qminus ())
	       && CONSP (XCDR (left))
	       && RANGED_FIXNUMP (-(INT_MAX as EMACS_INT), XCAR (XCDR (left)),
				  INT_MAX as EMACS_INT)
	{
	  leftpos = - XFIXNUM (XCAR (XCDR (left))) as c_int;
	  (*f).size_hint_flags |= XNegative;
	}
      else if CONSP (left) && EQ (XCAR (left), Qplus ())
	       && CONSP (XCDR (left))
	       && TYPE_RANGED_FIXNUMP (INT_MIN as EMACS_INT, XCAR (XCDR (left)),
				       INT_MAX as EMACS_INT)
	{ leftpos = XFIXNUM (XCAR (XCDR (left))) as c_int; }
      else if FLOATP (left)
	{ leftpos = frame_float (f, left, FRAME_FLOAT_LEFT, &mut parent_done,
				 &mut outer_done, 0); }

      if EQ (top, Qminus ())
	{ (*f).size_hint_flags |= YNegative; }
      else if TYPE_RANGED_FIXNUMP (INT_MIN as EMACS_INT, top, INT_MAX as EMACS_INT)
	{
	  toppos = XFIXNUM (top) as c_int;
	  if toppos < 0
	    { (*f).size_hint_flags |= YNegative; }
	}
      else if CONSP (top) && EQ (XCAR (top), Qminus ())
	       && CONSP (XCDR (top))
	       && RANGED_FIXNUMP (-(INT_MAX as EMACS_INT), XCAR (XCDR (top)),
				  INT_MAX as EMACS_INT)
	{
	  toppos = - XFIXNUM (XCAR (XCDR (top))) as c_int;
	  (*f).size_hint_flags |= YNegative;
	}
      else if CONSP (top) && EQ (XCAR (top), Qplus ())
	       && CONSP (XCDR (top))
	       && TYPE_RANGED_FIXNUMP (INT_MIN as EMACS_INT, XCAR (XCDR (top)),
				       INT_MAX as EMACS_INT)
	{ toppos = XFIXNUM (XCAR (XCDR (top))) as c_int; }
      else if FLOATP (top)
	{ toppos = frame_float (f, top, FRAME_FLOAT_TOP, &mut parent_done,
				&mut outer_done, 0); }

      /* Store the numeric value of the position.  */
      (*f).top_pos = toppos;
      (*f).left_pos = leftpos;

      (*f).win_gravity = NorthWestGravity;

      /* Actually set that position, and convert to absolute.  */
      if let Some (set_frame_offset_hook) = (*FRAME_TERMINAL! (f)).set_frame_offset_hook
        { set_frame_offset_hook (f as *mut c_void, leftpos, toppos, -1); }
    }

  if fullscreen_change
    {
      let old_value: Lisp_Object = get_frame_param (f, Qfullscreen ());

      store_frame_param (f, Qfullscreen (), fullscreen);
      if !EQ (fullscreen, old_value)
	{ gui_set_fullscreen (f, fullscreen, old_value); }
    }

  /* C 5387-5391: `#ifdef HAVE_X_WINDOWS`.  */
  #[cfg (feature = "have_x_windows")]
  {
    if (!NILP (icon_left) || !NILP (icon_top))
	&& ! (icon_left_no_change && icon_top_no_change)
      { x_wm_set_icon_position (f, XFIXNUM (icon_left), XFIXNUM (icon_top)); }
  }

  SAFE_FREE! (sa_count, sa_mark);
}

/* C 5396-5400.  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gui_set_frame_parameters (f: *mut frame, alist: Lisp_Object)
{
  gui_set_frame_parameters_1 (f, alist, false);
}

/* Insert a description of internally-recorded parameters of frame F
   into the parameter alist *ALISTPTR that is to be given to the user.
   Only parameters that are specific to the X window system
   and whose values are not correctly recorded in the frame's
   param_alist need to be considered here.  */
/* C 5408-5488: `gui_report_frame_params`.  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gui_report_frame_params (f: *mut frame, alistptr: *mut Lisp_Object)
{
  let mut tem: Lisp_Object = Qnil ();
  let mut w: uintmax_t = 0;

  /* Represent negative positions (off the top or left screen edge)
     in a way that Fmodify_frame_parameters will understand correctly.  */
  XSETINT! (tem, (*f).left_pos as EMACS_INT);
  if (*f).left_pos >= 0
    { store_in_alist (alistptr, Qleft (), tem); }
  else
    { store_in_alist (alistptr, Qleft (), list2 (Qplus (), tem)); }

  XSETINT! (tem, (*f).top_pos as EMACS_INT);
  if (*f).top_pos >= 0
    { store_in_alist (alistptr, Qtop (), tem); }
  else
    { store_in_alist (alistptr, Qtop (), list2 (Qplus (), tem)); }

  store_in_alist (alistptr, Qborder_width (),
		  make_fixnum ((*f).border_width as EMACS_INT));
  store_in_alist (alistptr, Qchild_frame_border_width (),
		  if FRAME_CHILD_FRAME_BORDER_WIDTH (f) >= 0
		  { make_fixnum (FRAME_CHILD_FRAME_BORDER_WIDTH (f) as EMACS_INT) }
		  else { Qnil () });
  store_in_alist (alistptr, Qinternal_border_width (),
		  make_fixnum (FRAME_INTERNAL_BORDER_WIDTH (f) as EMACS_INT));
  store_in_alist (alistptr, Qright_divider_width (),
		  make_fixnum (FRAME_RIGHT_DIVIDER_WIDTH (f) as EMACS_INT));
  store_in_alist (alistptr, Qbottom_divider_width (),
		  make_fixnum (FRAME_BOTTOM_DIVIDER_WIDTH (f) as EMACS_INT));
  store_in_alist (alistptr, Qleft_fringe (),
		  make_fixnum (FRAME_LEFT_FRINGE_WIDTH (f) as EMACS_INT));
  store_in_alist (alistptr, Qright_fringe (),
		  make_fixnum (FRAME_RIGHT_FRINGE_WIDTH (f) as EMACS_INT));
  store_in_alist (alistptr, Qscroll_bar_width (),
		  if FRAME_CONFIG_SCROLL_BAR_WIDTH (f) > 0
		  { make_fixnum (FRAME_CONFIG_SCROLL_BAR_WIDTH (f) as EMACS_INT) }
		  /* nil means "use default width"
		     for non-toolkit scroll bar.
		     ruler-mode.el depends on this.  */
		  else { Qnil () });
  store_in_alist (alistptr, Qscroll_bar_height (),
		  if FRAME_CONFIG_SCROLL_BAR_HEIGHT (f) > 0
		  { make_fixnum (FRAME_CONFIG_SCROLL_BAR_HEIGHT (f) as EMACS_INT) }
		  /* nil means "use default height"
		     for non-toolkit scroll bar.  */
		  else { Qnil () });
  /* FRAME_NATIVE_WINDOW is not guaranteed to return an integer.
     E.g., on MS-Windows it returns a value whose type is HANDLE,
     which is actually a pointer.  Explicit casting avoids compiler
     warnings.  */
  w = WINDOW_HANDLE_UINTPTR! (FRAME_NATIVE_WINDOW! (f)) as uintmax_t;
  store_in_alist (alistptr, Qwindow_id (),
		  make_formatted_string (concat! ("%", PRIuMAX! (), "\0").as_ptr ()
					 as *const c_char, w));
  /* C 5464-5472: `#ifdef HAVE_X_WINDOWS`, with C 5465-5468's nested `#ifdef USE_X_TOOLKIT`
     (`if (FRAME_X_OUTPUT (f)->widget)` guards the assignment; the port keeps the guard).  */
  #[cfg (feature = "have_x_windows")]
  {
    /* Tooltip frame may not have this widget.  */
    #[cfg (feature = "use_x_toolkit")]
    if !(*FRAME_X_OUTPUT! (f)).widget.is_null ()
      { w = WINDOW_HANDLE_UINTPTR! (FRAME_OUTER_WINDOW! (f)) as uintmax_t; }
    #[cfg (not (feature = "use_x_toolkit"))]
    { w = WINDOW_HANDLE_UINTPTR! (FRAME_OUTER_WINDOW! (f)) as uintmax_t; }
    store_in_alist (alistptr, Qouter_window_id (),
		    make_formatted_string (concat! ("%", PRIuMAX! (), "\0").as_ptr ()
					   as *const c_char, w));
  }
  store_in_alist (alistptr, Qicon_name (), (*f).icon_name);
  store_in_alist (alistptr, Qvisibility (),
		  if FRAME_VISIBLE_P (f) { Qt () }
		  else if FRAME_ICONIFIED_P (f) { Qicon () } else { Qnil () });
  store_in_alist (alistptr, Qdisplay (),
		  XCAR ((*FRAME_DISPLAY_INFO! (f)).name_list_element));

  /* C 5480-5484's `FRAME_OUTPUT_DATA (f)->parent_desc`.  No platform macro names the member
     (it is a field of each GUI's `output_data` record), so the arm reaches it through one gated
     binding per `TERM_HEADER`, exactly as `FRAME_OUTPUT_DATA` is gated above; the record type is
     in scope only when its mirror is.  */
  #[cfg (feature = "have_x_windows")]
  let parent_desc = (*((*f).output_data.x as *mut x_output)).parent_desc;
  #[cfg (feature = "have_ns")]
  let parent_desc = (*((*f).output_data.ns as *mut ns_output)).parent_desc;
  #[cfg (feature = "have_pgtk")]
  let parent_desc = (*((*f).output_data.pgtk as *mut pgtk_output)).parent_desc;
  #[cfg (feature = "have_ntgui")]
  let parent_desc = (*((*f).output_data.w32 as *mut w32_output)).parent_desc;
  #[cfg (feature = "have_haiku")]
  let parent_desc = (*((*f).output_data.haiku as *mut haiku_output)).parent_desc;
  #[cfg (feature = "have_android")]
  let parent_desc = (*((*f).output_data.android as *mut android_output)).parent_desc;

  if parent_desc == (*FRAME_DISPLAY_INFO! (f)).root_window
    { tem = Qnil (); }
  else
    { tem = make_fixed_natnum (WINDOW_HANDLE_UINTPTR! (parent_desc) as EMACS_INT); }
  store_in_alist (alistptr, Qexplicit_name (),
		  if explicit_name! (f) { Qt () } else { Qnil () });
  store_in_alist (alistptr, Qparent_id (), tem);
  store_in_alist (alistptr, Qtool_bar_position (), FRAME_TOOL_BAR_POSITION (f));
}

/* Change the `fullscreen' frame parameter of frame F.  OLD_VALUE is
   the previous value of that parameter, NEW_VALUE is the new value. */
/* C 5494-5510: `gui_set_fullscreen`.  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gui_set_fullscreen (f: *mut frame, new_value: Lisp_Object,
					     old_value: Lisp_Object)
{
  if NILP (new_value)
    { frame_set_field (f, FRAME_WANT_FULLSCREEN_UNIT, FRAME_WANT_FULLSCREEN_SHIFT, FRAME_WANT_FULLSCREEN_MASK, FULLSCREEN_NONE as u32); }
  else if EQ (new_value, Qfullboth ()) || EQ (new_value, Qfullscreen ())
    { frame_set_field (f, FRAME_WANT_FULLSCREEN_UNIT, FRAME_WANT_FULLSCREEN_SHIFT, FRAME_WANT_FULLSCREEN_MASK, FULLSCREEN_BOTH as u32); }
  else if EQ (new_value, Qfullwidth ())
    { frame_set_field (f, FRAME_WANT_FULLSCREEN_UNIT, FRAME_WANT_FULLSCREEN_SHIFT, FRAME_WANT_FULLSCREEN_MASK, FULLSCREEN_WIDTH as u32); }
  else if EQ (new_value, Qfullheight ())
    { frame_set_field (f, FRAME_WANT_FULLSCREEN_UNIT, FRAME_WANT_FULLSCREEN_SHIFT, FRAME_WANT_FULLSCREEN_MASK, FULLSCREEN_HEIGHT as u32); }
  else if EQ (new_value, Qmaximized ())
    { frame_set_field (f, FRAME_WANT_FULLSCREEN_UNIT, FRAME_WANT_FULLSCREEN_SHIFT, FRAME_WANT_FULLSCREEN_MASK, FULLSCREEN_MAXIMIZED as u32); }

  if let Some (fullscreen_hook) = (*FRAME_TERMINAL! (f)).fullscreen_hook
    { fullscreen_hook (f as *mut c_void); }
}

/* Change the `line-spacing' frame parameter of frame F.  OLD_VALUE is
   the previous value of that parameter, NEW_VALUE is the new value.  */
/* C 5516-5578: `gui_set_line_spacing`.  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gui_set_line_spacing (f: *mut frame, new_value: Lisp_Object,
					       old_value: Lisp_Object)
{
  if NILP (new_value)
    {
      (*f).extra_line_spacing = 0;
      (*f).extra_line_spacing_above = 0;
    }
  else if RANGED_FIXNUMP (0, new_value, INT_MAX as EMACS_INT)
    {
      (*f).extra_line_spacing = XFIXNAT (new_value) as c_int;
      (*f).extra_line_spacing_above = 0;
    }
  else if FLOATP (new_value)
    {
      let new_spacing: c_int = (XFLOAT_DATA (new_value) * FRAME_LINE_HEIGHT (f) as f64) as c_int;

      if new_spacing >= 0
	{
	  (*f).extra_line_spacing = new_spacing;
	  (*f).extra_line_spacing_above = 0;
	}
      else
	{ signal_error (c"Invalid line-spacing".as_ptr (), new_value); }
    }
  else if CONSP (new_value)
    {
      let above: Lisp_Object = XCAR (new_value);
      let below: Lisp_Object = XCDR (new_value);

      /* Integer pair case.  */
      if RANGED_FIXNUMP (0, above, INT_MAX as EMACS_INT)
	  && RANGED_FIXNUMP (0, below, INT_MAX as EMACS_INT)
	{
	  (*f).extra_line_spacing = (XFIXNAT (above) + XFIXNAT (below)) as c_int;
	  (*f).extra_line_spacing_above = XFIXNAT (above) as c_int;
	}

      /* Float pair case.  */
      else if FLOATP (XCAR (new_value))
	       && FLOATP (XCDR (new_value))
	{
	  let new_spacing: c_int
	    = ((XFLOAT_DATA (above) + XFLOAT_DATA (below)) * FRAME_LINE_HEIGHT (f) as f64)
	      as c_int;
	  let spacing_above: c_int
	    = (XFLOAT_DATA (above) * FRAME_LINE_HEIGHT (f) as f64) as c_int;

	  if new_spacing >= 0 && spacing_above >= 0
	    {
	      (*f).extra_line_spacing = new_spacing;
	      (*f).extra_line_spacing_above = spacing_above;
	    }
	  else
	    { signal_error (c"Invalid line-spacing".as_ptr (), new_value); }
	}

      /* Unmatched pair case.  */
      else
	{
	  signal_error (c"Invalid line-spacing".as_ptr (), new_value);
	}
    }
  else
    { signal_error (c"Invalid line-spacing".as_ptr (), new_value); }

  if FRAME_VISIBLE_P (f)
    { redraw_frame (f); }
}

/* Change the `screen-gamma' frame parameter of frame F.  OLD_VALUE is
   the previous value of that parameter, NEW_VALUE is the new value.  */
/* C 5584-5604: `gui_set_screen_gamma`.  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gui_set_screen_gamma (f: *mut frame, new_value: Lisp_Object,
					       old_value: Lisp_Object)
{
  let mut bgcolor: Lisp_Object;

  if NILP (new_value)
    { (*f).gamma = 0.0; }
  else if NUMBERP (new_value) && XFLOATINT (new_value) > 0.0
    /* The value 0.4545 is the normal viewing gamma.  */
    { (*f).gamma = 1.0 / (0.4545 * XFLOATINT (new_value)); }
  else
    { signal_error (c"Invalid screen-gamma".as_ptr (), new_value); }

  /* Apply the new gamma value to the frame background.  */
  bgcolor = Fassq (Qbackground_color (), (*f).param_alist);
  if CONSP (bgcolor) && { bgcolor = XCDR (bgcolor); STRINGP (bgcolor) }
    { handle_frame_param (f, Qbackground_color (), bgcolor, Qnil ()); }

  clear_face_cache (true);	/* FIXME: Why of all frames?  */
  fset_redisplay (f as *mut c_void);
}

/* C 5607-5740: `gui_set_font`.  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gui_set_font (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  let mut arg = arg;
  let mut font_object: Lisp_Object = Qnil ();
  let mut fontset: c_int = -1;
  let mut iwidth: c_int = 0;

  /* Set the frame parameter back to the old value because we may
     fail to use ARG as the new parameter value.  */
  store_frame_param (f, Qfont (), oldval);

  /* ARG is a fontset name, a font name, a cons of fontset name and a
     font object, or a font object.  In the last case, this function
     never fail.  */
  if STRINGP (arg)
    {
      fontset = fs_query_fontset (arg, 0);
      if fontset < 0
	{
	  font_object = font_open_by_name (f, arg);
	  if NILP (font_object)
	    { error (c"Font `%s' is not defined".as_ptr (), SSDATA (arg)); }
	  arg = AREF (font_object, FONT_NAME_INDEX as isize);
	}
      else if fontset > 0
	{
	  font_object = font_open_by_name (f, fontset_ascii (fontset));
	  if NILP (font_object)
	    { error (c"Font `%s' is not defined".as_ptr (), SDATA (arg)); }
	  arg = AREF (font_object, FONT_NAME_INDEX as isize);
	}
      else
	{ error (c"The default fontset can't be used for a frame font".as_ptr ()); }
    }
  else if CONSP (arg) && STRINGP (XCAR (arg)) && FONT_OBJECT_P (XCDR (arg))
    {
      /* This is the case that the ASCII font of F's fontset XCAR
	 (arg) is changed to the font XCDR (arg) by
	 `set-fontset-font'.  */
      fontset = fs_query_fontset (XCAR (arg), 0);
      if fontset < 0
	{ error (c"Unknown fontset: %s".as_ptr (), SDATA (XCAR (arg))); }
      font_object = XCDR (arg);
      arg = AREF (font_object, FONT_NAME_INDEX as isize);
    }
  else if FONT_OBJECT_P (arg)
    {
      font_object = arg;
      /* This is to store the XLFD font name in the frame parameter for
	 backward compatibility.  We should store the font-object
	 itself in the future.  */
      arg = AREF (font_object, FONT_NAME_INDEX as isize);
      fontset = FRAME_FONTSET! (f);
      /* Check if we can use the current fontset.  If not, set FONTSET
	 to -1 to generate a new fontset from FONT-OBJECT.  */
      if fontset >= 0
	{
	  let ascii_font: Lisp_Object = fontset_ascii (fontset);
	  let spec: Lisp_Object = font_spec_from_name (ascii_font);

	  /* SPEC might be nil because ASCII_FONT's name doesn't parse
	     according to stupid XLFD rules, which, for example,
	     disallow font names that include a dash followed by a
	     number.  So in those cases we simply call
	     set_new_font_hook below to generate a new fontset.  */
	  if NILP (spec) || ! font_match_p (spec, font_object)
	    { fontset = -1; }
	}
    }
  else
    { signal_error (c"Invalid font".as_ptr (), arg); }

  if ! NILP (Fequal (font_object, oldval))
    { return; }

  if let Some (set_new_font_hook) = (*FRAME_TERMINAL! (f)).set_new_font_hook
    { set_new_font_hook (f as *mut c_void, font_object, fontset); }
  store_frame_param (f, Qfont (), arg);

  /* Recalculate tabbar height.  */
  (*f).n_tab_bar_rows = 0;
  /* Recalculate toolbar height.  */
  (*f).n_tool_bar_rows = 0;

  /* Re-initialize F's image cache.  Since `set_new_font_hook' might
     have changed the frame's column width, by which images are scaled,
     it might likewise need to be assigned a different image cache, or
     have its existing cache adjusted, if by coincidence it is its sole
     user.  */

  iwidth = max (10, FRAME_COLUMN_WIDTH (f));
  if ! FRAME_IMAGE_CACHE! (f).is_null ()
      && iwidth != (*(FRAME_IMAGE_CACHE! (f) as *mut image_cache)).scaling_col_width
    {
      eassert! ((*(FRAME_IMAGE_CACHE! (f) as *mut image_cache)).refcount >= 1);
      if (*(FRAME_IMAGE_CACHE! (f) as *mut image_cache)).refcount == 1
	{
	  /* This frame is the only user of this image cache.  */
	  (*(FRAME_IMAGE_CACHE! (f) as *mut image_cache)).scaling_col_width = iwidth;
	  /* Clean F's image cache of images whose values are derived
	     from the font width.  */
	  clear_image_cache (f, Qauto ());
	}
      else
	{
	  /* Release the current image cache, and reuse or allocate a
	     new image cache with IWIDTH.  */
	  (*(FRAME_IMAGE_CACHE! (f) as *mut image_cache)).refcount -= 1;
	  (*f).image_cache = share_image_cache (f) as *mut c_void;
	  (*(FRAME_IMAGE_CACHE! (f) as *mut image_cache)).refcount += 1;
	}
    }

  /* Ensure we redraw it.  */
  clear_current_matrices (f);

  /* Attempt to hunt down bug#16028.  */
  SET_FRAME_GARBAGED! (f);

  /* This is important if we are called by some Lisp as part of
     redisplaying the frame, see redisplay_internal.  */
  fonts_changed! (f, true);

  recompute_basic_faces (f);

  do_pending_window_change (false);

  /* We used to call face-set-after-frame-default here, but it leads to
     recursive calls (since that function can set the `default' face's
     font which in turns changes the frame's `font' parameter).
     Also I don't know what this call is meant to do, but it seems the
     wrong way to do it anyway (it does a lot more work than what seems
     reasonable in response to a change to `font').  */
}

/* C 5742-5797: `gui_set_font_backend`.  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gui_set_font_backend (f: *mut frame, new_value: Lisp_Object,
					       old_value: Lisp_Object)
{
  let mut new_value = new_value;

  if ! NILP (new_value)
      && !CONSP (new_value)
    {
      let mut p0: *mut c_char;
      let mut p1: *mut c_char;

      CHECK_STRING (new_value);
      p0 = SSDATA (new_value);
      p1 = p0;
      new_value = Qnil ();
      while *p0 != 0
	{
	  while *p1 != 0 && ! c_isspace (*p1 as c_int) && *p1 != b',' as c_char
	    { p1 = p1.offset (1); }
	  if p0 < p1
	    {
	      new_value = Fcons (Fintern (make_string (p0, p1.offset_from (p0)), Qnil ()),
				 new_value);
	    }
	  if *p1 != 0
	    {
	      let mut c: c_int;

	      while { p1 = p1.offset (1); c = *p1 as c_int; c != 0 && c_isspace (c) } { }
	    }
	  p0 = p1;
	}
      new_value = Fnreverse (new_value);
    }

  if ! NILP (old_value) && ! NILP (Fequal (old_value, new_value))
    { return; }

  if ! FRAME_FONT! (f).is_null ()
    {
      let mut frame: Lisp_Object = Qnil ();

      XSETFRAME! (frame, f);
      free_all_realized_faces (frame);
    }

  new_value = font_update_drivers (f, if NILP (new_value) { Qt () } else { new_value });
  if NILP (new_value)
    {
      if NILP (old_value)
	{ error (c"No font backend available".as_ptr ()); }
      font_update_drivers (f, old_value);
      error (c"None of specified font backends are available".as_ptr ());
    }
  store_frame_param (f, Qfont_backend (), new_value);

  if ! FRAME_FONT! (f).is_null ()
    {
      /* Reconsider default font after backend(s) change (Bug#23386).  */
      if let Some (default_font_parameter) = (*((FRAME_RIF! (f)) as *mut redisplay_interface)).default_font_parameter
	{ default_font_parameter (f as *mut c_void, Qnil ()); }
      face_change = true;
      windows_or_buffers_changed = 18;
    }
}

/* C 5799-5821: `gui_set_left_fringe`.  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gui_set_left_fringe (f: *mut frame, new_value: Lisp_Object,
					      old_value: Lisp_Object)
{
  let unit: c_int = FRAME_COLUMN_WIDTH (f);
  let old_width: c_int = FRAME_LEFT_FRINGE_WIDTH (f);
  let new_width: c_int;

  new_width = if RANGED_FIXNUMP (-(INT_MAX as EMACS_INT), new_value, INT_MAX as EMACS_INT)
	      { eabs! (XFIXNUM (new_value)) as c_int } else { 8 };

  if new_width != old_width
    {
      (*f).left_fringe_width = new_width;
      (*f).fringe_cols /* Round up.  */
	= (new_width + FRAME_RIGHT_FRINGE_WIDTH (f) + unit - 1) / unit;

      if FRAME_NATIVE_WINDOW! (f) != 0
	{ adjust_frame_size (f, -1, -1, 3, false, Qleft_fringe ()); }

      SET_FRAME_GARBAGED! (f);
    }
}

/* C 5823-5845: `gui_set_right_fringe`.  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gui_set_right_fringe (f: *mut frame, new_value: Lisp_Object,
					       old_value: Lisp_Object)
{
  let unit: c_int = FRAME_COLUMN_WIDTH (f);
  let old_width: c_int = FRAME_RIGHT_FRINGE_WIDTH (f);
  let new_width: c_int;

  new_width = if RANGED_FIXNUMP (-(INT_MAX as EMACS_INT), new_value, INT_MAX as EMACS_INT)
	      { eabs! (XFIXNUM (new_value)) as c_int } else { 8 };

  if new_width != old_width
    {
      (*f).right_fringe_width = new_width;
      (*f).fringe_cols /* Round up.  */
	= (new_width + FRAME_LEFT_FRINGE_WIDTH (f) + unit - 1) / unit;

      if FRAME_NATIVE_WINDOW! (f) != 0
	{ adjust_frame_size (f, -1, -1, 3, false, Qright_fringe ()); }

      SET_FRAME_GARBAGED! (f);
    }
}

/* C 5847-5859: `gui_set_border_width`.  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gui_set_border_width (f: *mut frame, arg: Lisp_Object,
					       oldval: Lisp_Object)
{
  let border_width: c_int = check_integer_range (arg, INT_MIN as intmax_t,
						 INT_MAX as intmax_t) as c_int;

  if border_width == (*f).border_width
    { return; }

  if FRAME_NATIVE_WINDOW! (f) != 0
    { error (c"Cannot change the border width of a frame".as_ptr ()); }

  (*f).border_width = border_width;
}

/* C 5861-5873: `gui_set_right_divider_width`.  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gui_set_right_divider_width (f: *mut frame, arg: Lisp_Object,
						      oldval: Lisp_Object)
{
  let old: c_int = FRAME_RIGHT_DIVIDER_WIDTH (f);
  let new: c_int = check_int_nonnegative (arg);
  if new != old
    {
      (*f).right_divider_width = new;
      adjust_frame_size (f, -1, -1, 4, false, Qright_divider_width ());
      adjust_frame_glyphs (f);
      SET_FRAME_GARBAGED! (f);
    }
}

/* C 5875-5887: `gui_set_bottom_divider_width`.  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gui_set_bottom_divider_width (f: *mut frame, arg: Lisp_Object,
						       oldval: Lisp_Object)
{
  let old: c_int = FRAME_BOTTOM_DIVIDER_WIDTH (f);
  let new: c_int = check_int_nonnegative (arg);
  if new != old
    {
      (*f).bottom_divider_width = new;
      adjust_frame_size (f, -1, -1, 4, false, Qbottom_divider_width ());
      adjust_frame_glyphs (f);
      SET_FRAME_GARBAGED! (f);
    }
}

/* C 5889-5901: `gui_set_visibility`.  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gui_set_visibility (f: *mut frame, value: Lisp_Object,
					     oldval: Lisp_Object)
{
  let mut frame: Lisp_Object = Qnil ();
  XSETFRAME! (frame, f);

  if NILP (value)
    { Fmake_frame_invisible (frame, Qt ()); }
  else if EQ (value, Qicon ())
    { Ficonify_frame (frame); }
  else
    { Fmake_frame_visible (frame); }
}

/* C 5903-5907: `gui_set_autoraise`.  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gui_set_autoraise (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  auto_raise! (f, !NILP (arg));
}

/* C 5909-5913: `gui_set_autolower`.  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gui_set_autolower (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  auto_lower! (f, !NILP (arg));
}

/* C 5915-5919: `gui_set_unsplittable`.  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gui_set_unsplittable (f: *mut frame, arg: Lisp_Object, oldval: Lisp_Object)
{
  no_split! (f, !NILP (arg));
}

/* C 5921-6970: the rest of the run, carried this round - `gui_set_vertical_scroll_bars`
   (C 5921) through `gui_figure_window_size` (C 6708-6967), with C 6971's `#endif` as the run's
   end.  Same treatment as the first half: both arms of every `#if`, one gate per item, the
   C-line-range table and readings in `docs/ports/frame.md`.  */

/* The names this half takes from headers the unit's include graph does not bring in yet, and
   the ones the mirrors gate away for a window-system configuration.  Each is declared here
   under C's own name with C's own body/prototype in a `/// C:` line (PORTING.md P-72 rule 3,
   the P-86 ⑩ shape); `docs/ports/frame.md`'s Deviations lists them as mirror gaps.  */

/* frame.h 1219-1229: the `HAVE_WINDOW_SYSTEM` arm of the scroll-bar accessors.  `include/frame.rs`
   carries the `#else` arm's readers and gates them with `not (have_window_system)` (its own note
   records why: the window-system body needs the flag numbering in `docs/baseline/frame-bits.tsv`),
   so the arm's versions are defined here, in C's terms.  */

/* frame.h 1207-1223's four vertical-scroll-bar readers are `include/frame.rs` 1203-1230's
   functions now (each is C's own `#else`/`#ifdef HAVE_WINDOW_SYSTEM` pair, and the window-system
   arm reads the packed `vertical_scroll_bar_type` field the mirror numbers per target).  This
   unit's four `macro_rules!` read a named member the record does not have and were `E0609` once
   the window-system arm went live (measured).  */

/* frame.h 1276-1280: `FRAME_HAS_HORIZONTAL_SCROLL_BARS`.  The mirror's reader is the `#else`
   arm (`not (use_horizontal_scroll_bars)`, a constant `0`); the `USE_HORIZONTAL_SCROLL_BARS`
   arm reads the flag, which the mirror's run does not number for a window-system
   configuration - C's text is kept, as above.  */
/// C: frame.h 1277-1278: `#define FRAME_HAS_HORIZONTAL_SCROLL_BARS(f) ((f)->horizontal_scroll_bars)`
#[cfg (feature = "use_horizontal_scroll_bars")]
macro_rules! FRAME_HAS_HORIZONTAL_SCROLL_BARS {
  ($f:expr) => { (*$f).horizontal_scroll_bars };
}

/* frame.h 1283-1299: the four `FRAME_CONFIG_SCROLL_BAR_*` slots.  `include/frame.rs` mirrors
   them as readers, and C's text is an *l-value* macro (C 5985-5986, 6020-6021 write them), so
   the writable form is here at the use site under the same gate as the C text that uses it.  */
/// C: frame.h 1286: `#define FRAME_CONFIG_SCROLL_BAR_WIDTH(f) ((f)->config_scroll_bar_width)`
#[cfg (feature = "have_window_system")]
macro_rules! FRAME_CONFIG_SCROLL_BAR_WIDTH {
  ($f:expr) => { (*$f).config_scroll_bar_width };
}
/// C: frame.h 1291: `#define FRAME_CONFIG_SCROLL_BAR_HEIGHT(f) ((f)->config_scroll_bar_height)`
#[cfg (feature = "have_window_system")]
macro_rules! FRAME_CONFIG_SCROLL_BAR_HEIGHT {
  ($f:expr) => { (*$f).config_scroll_bar_height };
}
/// C: frame.h 1296: `#define FRAME_CONFIG_SCROLL_BAR_COLS(f) ((f)->config_scroll_bar_cols)`
#[cfg (feature = "have_window_system")]
macro_rules! FRAME_CONFIG_SCROLL_BAR_COLS {
  ($f:expr) => { (*$f).config_scroll_bar_cols };
}
/// C: frame.h 1301: `#define FRAME_CONFIG_SCROLL_BAR_LINES(f) ((f)->config_scroll_bar_lines)`
#[cfg (feature = "have_window_system")]
macro_rules! FRAME_CONFIG_SCROLL_BAR_LINES {
  ($f:expr) => { (*$f).config_scroll_bar_lines };
}

/* frame.h 1107-1108: `FRAME_TOOL_BAR_LINES`/`FRAME_TOOL_BAR_HEIGHT`.  `include/frame.rs`
   mirrors both as readers (the C text is an l-value macro), and C 6734/6766/6783-6784 write
   the height, so the writable pair is here at the use site (same shape as the tab-bar pair the
   first half of the run defines).  */
/// C: frame.h 1107: `#define FRAME_TOOL_BAR_LINES(f) (f)->tool_bar_lines`
#[cfg (feature = "have_window_system")]
macro_rules! FRAME_TOOL_BAR_LINES {
  ($f:expr) => { (*$f).tool_bar_lines };
}
/// C: frame.h 1108: `#define FRAME_TOOL_BAR_HEIGHT(f) (f)->tool_bar_height`
#[cfg (feature = "have_window_system")]
macro_rules! FRAME_TOOL_BAR_HEIGHT {
  ($f:expr) => { (*$f).tool_bar_height };
}

/* dispextern.h 115-165's six `typedef struct <platform>_display_info Display_Info;` arms
   are `include/dispextern.rs` 716-737's now (one per platform, each gated by its own
   feature); this unit's own six copies were `E0428` on Android once the window-system
   arm went live (measured) - C has the typedef once, in dispextern.h.  */

/* dispextern.h 3983-3992: `enum resource_types`, itself inside C's `#ifdef HAVE_WINDOW_SYSTEM`;
   C's enum ABI is `c_int` (PORTING.md 1.3).  */
/// C: dispextern.h 3983: `enum resource_types { RES_TYPE_NUMBER, RES_TYPE_FLOAT, RES_TYPE_BOOLEAN, RES_TYPE_STRING, RES_TYPE_SYMBOL, RES_TYPE_BOOLEAN_NUMBER };`
#[cfg (feature = "have_window_system")]
pub type resource_types = c_int;
#[cfg (feature = "have_window_system")]
const RES_TYPE_NUMBER: resource_types = 0;          // C 3985
#[cfg (feature = "have_window_system")]
const RES_TYPE_FLOAT: resource_types = 1;           // C 3986
#[cfg (feature = "have_window_system")]
const RES_TYPE_BOOLEAN: resource_types = 2;         // C 3987
#[cfg (feature = "have_window_system")]
const RES_TYPE_STRING: resource_types = 3;          // C 3988
#[cfg (feature = "have_window_system")]
const RES_TYPE_SYMBOL: resource_types = 4;          // C 3989
#[cfg (feature = "have_window_system")]
const RES_TYPE_BOOLEAN_NUMBER: resource_types = 5;  // C 3990

/* frame.c 6191's `EMACS_CLASS`: frame.h 1840's `#define EMACS_CLASS "Emacs"` (no mirror in the
   include graph; `xterm.rs` 801-802 records the same reading).  C uses it as a string literal,
   `sizeof (EMACS_CLASS) - 1` (the character count without the NUL) and a `%s` argument.  */
/// C: frame.h 1840: `#define EMACS_CLASS "Emacs"`
#[cfg (feature = "have_window_system")]
const EMACS_CLASS: &str = "Emacs";


/* C 6538's `#if !defined (HAVE_X_WINDOWS) && defined (NoValue)`: the `NoValue` family, defined
   by the same six `*gui.h` headers (all `0x0000`/`0x0001`/`0x0002`/`0x0004`/`0x0008`; nsgui.h
   116-120, pgtkgui.h 79-83, w32gui.h 73-77, androidgui.h 236-240, haikugui.h 77-81).  */
/// C: nsgui.h 116: `#define NoValue 0x0000` / `XValue 0x0001` / `YValue 0x0002` / `WidthValue 0x0004` / `HeightValue 0x0008`
#[cfg (all (not (feature = "have_x_windows"), feature = "no_value"))]
const NoValue: c_int = 0x0000;
#[cfg (all (not (feature = "have_x_windows"), feature = "no_value"))]
const XValue: c_int = 0x0001;
#[cfg (all (not (feature = "have_x_windows"), feature = "no_value"))]
const YValue: c_int = 0x0002;
#[cfg (all (not (feature = "have_x_windows"), feature = "no_value"))]
const WidthValue: c_int = 0x0004;
#[cfg (all (not (feature = "have_x_windows"), feature = "no_value"))]
const HeightValue: c_int = 0x0008;

/* The mirror-gap functions this half calls; each `/// C:` is the header's own prototype
   (PORTING.md P-72 rule 3).  */
#[cfg (feature = "have_window_system")]
unsafe extern "C" {
  /// C: dispextern.h 3993: extern Display_Info *check_x_display_info (Lisp_Object);
  pub fn check_x_display_info (frame: Lisp_Object) -> *mut Display_Info;
  /// C: dispextern.h 3688: extern void note_mouse_highlight (struct frame *, int, int);
  pub fn note_mouse_highlight (f: *mut frame, x: c_int, y: c_int);
  /// C: xterm.h 1744: extern const char *x_get_string_resource (void *, const char *, const char *);
  pub fn x_get_string_resource (rdb: *mut c_void, name: *const c_char,
                                class: *const c_char) -> *const c_char;
  /// C: lisp.h 4717: extern ptrdiff_t esprintf (char *, char const *, ...)
  pub fn esprintf (buf: *mut c_char, format: *const c_char, ...) -> ptrdiff_t;
}

/* C 5921-5951: `gui_set_vertical_scroll_bars`.  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gui_set_vertical_scroll_bars (f: *mut frame, arg: Lisp_Object,
						       oldval: Lisp_Object)
{
  if (EQ (arg, Qleft ()) && FRAME_HAS_VERTICAL_SCROLL_BARS_ON_RIGHT (f) != 0)
    || (EQ (arg, Qright ()) && FRAME_HAS_VERTICAL_SCROLL_BARS_ON_LEFT (f) != 0)
    || (NILP (arg) && FRAME_HAS_VERTICAL_SCROLL_BARS (f) != 0)
    || (!NILP (arg) && FRAME_HAS_VERTICAL_SCROLL_BARS (f) == 0)
    {
      frame_set_field (f, FRAME_VERTICAL_SCROLL_BAR_TYPE_FLAG_UNIT,
                       FRAME_VERTICAL_SCROLL_BAR_TYPE_FLAG_SHIFT, FRAME_VERTICAL_SCROLL_BAR_MASK,
                       (if NILP (arg) { vertical_scroll_bar_none }
	  else if EQ (Qleft (), arg) { vertical_scroll_bar_left }
	  else if EQ (Qright (), arg) { vertical_scroll_bar_right }
	  else if EQ (Qleft (), Vdefault_frame_scroll_bars) { vertical_scroll_bar_left }
	  else if EQ (Qright (), Vdefault_frame_scroll_bars) { vertical_scroll_bar_right }
	  else { vertical_scroll_bar_none }) as u32);

      /* We set this parameter before creating the native window for
	 the frame, so we can get the geometry right from the start.
	 However, if the window hasn't been created yet, we shouldn't
	 call set_window_size_hook.  */
      if FRAME_NATIVE_WINDOW! (f) != 0
	{ adjust_frame_size (f, -1, -1, 3, false, Qvertical_scroll_bars ()); }

      SET_FRAME_GARBAGED! (f);
    }
}

/* C 5953-5972: `gui_set_horizontal_scroll_bars`.  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gui_set_horizontal_scroll_bars (f: *mut frame, arg: Lisp_Object,
							  oldval: Lisp_Object)
{
  /* C 5956-5971's `#if USE_HORIZONTAL_SCROLL_BARS` (window.h 898-904; the mirror's
     `USE_HORIZONTAL_SCROLL_BARS` is `false`).  */
  #[cfg (feature = "use_horizontal_scroll_bars")]
  {
    if (NILP (arg) && FRAME_HAS_HORIZONTAL_SCROLL_BARS! (f))
      || (!NILP (arg) && !FRAME_HAS_HORIZONTAL_SCROLL_BARS! (f))
      {
	(*f).horizontal_scroll_bars = if NILP (arg) { false } else { true };

	/* We set this parameter before creating the native window for
	   the frame, so we can get the geometry right from the start.
	   However, if the window hasn't been created yet, we shouldn't
	   call set_window_size_hook.  */
	if FRAME_NATIVE_WINDOW! (f) != 0
	  { adjust_frame_size (f, -1, -1, 3, false, Qhorizontal_scroll_bars ()); }

	SET_FRAME_GARBAGED! (f);
      }
  }
}

/* C 5974-6006: `gui_set_scroll_bar_width`.  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gui_set_scroll_bar_width (f: *mut frame, arg: Lisp_Object,
						   oldval: Lisp_Object)
{
  let unit: c_int = FRAME_COLUMN_WIDTH (f);

  if RANGED_FIXNUMP (1, arg, INT_MAX as EMACS_INT)
    {
      if XFIXNAT (arg) == FRAME_CONFIG_SCROLL_BAR_WIDTH! (f) as EMACS_INT
	{ return; }
      else
	{
	  FRAME_CONFIG_SCROLL_BAR_WIDTH! (f) = XFIXNAT (arg) as c_int;
	  FRAME_CONFIG_SCROLL_BAR_COLS! (f)
	    = ((XFIXNAT (arg) + unit as EMACS_INT - 1) / unit as EMACS_INT) as c_int;
	  if FRAME_NATIVE_WINDOW! (f) != 0
	    { adjust_frame_size (f, -1, -1, 3, false, Qscroll_bar_width ()); }

	  SET_FRAME_GARBAGED! (f);
	}
    }
  else
    {
      if (*FRAME_TERMINAL! (f)).set_scroll_bar_default_width_hook.is_some ()
	{ if let Some (set_scroll_bar_default_width_hook)
	      = (*FRAME_TERMINAL! (f)).set_scroll_bar_default_width_hook
	  { set_scroll_bar_default_width_hook (f as *mut c_void); } }

      if FRAME_NATIVE_WINDOW! (f) != 0
	{ adjust_frame_size (f, -1, -1, 3, false, Qscroll_bar_width ()); }

      SET_FRAME_GARBAGED! (f);
    }

  (*XWINDOW (FRAME_SELECTED_WINDOW (f))).cursor.hpos = 0;
  (*XWINDOW (FRAME_SELECTED_WINDOW (f))).cursor.x = 0;
}

/* C 6008-6042: `gui_set_scroll_bar_height`.  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gui_set_scroll_bar_height (f: *mut frame, arg: Lisp_Object,
						    oldval: Lisp_Object)
{
  /* C 6011-6041's `#if USE_HORIZONTAL_SCROLL_BARS`.  */
  #[cfg (feature = "use_horizontal_scroll_bars")]
  {
    let unit: c_int = FRAME_LINE_HEIGHT (f);

    if RANGED_FIXNUMP (1, arg, INT_MAX as EMACS_INT)
      {
	if XFIXNAT (arg) == FRAME_CONFIG_SCROLL_BAR_HEIGHT! (f) as EMACS_INT
	  { return; }
	else
	  {
	    FRAME_CONFIG_SCROLL_BAR_HEIGHT! (f) = XFIXNAT (arg) as c_int;
	    FRAME_CONFIG_SCROLL_BAR_LINES! (f)
	      = ((XFIXNAT (arg) + unit as EMACS_INT - 1) / unit as EMACS_INT) as c_int;
	    if FRAME_NATIVE_WINDOW! (f) != 0
	      { adjust_frame_size (f, -1, -1, 3, false, Qscroll_bar_height ()); }

	    SET_FRAME_GARBAGED! (f);
	  }
      }
    else
      {
	if (*FRAME_TERMINAL! (f)).set_scroll_bar_default_height_hook.is_some ()
	  { if let Some (set_scroll_bar_default_height_hook)
		= (*FRAME_TERMINAL! (f)).set_scroll_bar_default_height_hook
	    { set_scroll_bar_default_height_hook (f as *mut c_void); } }

	if FRAME_NATIVE_WINDOW! (f) != 0
	  { adjust_frame_size (f, -1, -1, 3, false, Qscroll_bar_height ()); }

	SET_FRAME_GARBAGED! (f);
      }

    (*XWINDOW (FRAME_SELECTED_WINDOW (f))).cursor.vpos = 0;
    (*XWINDOW (FRAME_SELECTED_WINDOW (f))).cursor.y = 0;
  }
}

/* C 6044-6095: `gui_set_alpha`.  C's guard is
   `#if (defined HAVE_PGTK || defined HAVE_NTGUI || defined HAVE_HAIKU || defined HAVE_NS)`.  */
#[cfg (feature = "have_window_system")]
#[cfg (any (feature = "have_pgtk", feature = "have_ntgui", feature = "have_haiku",
	    feature = "have_ns"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gui_set_alpha (f: *mut frame, mut arg: Lisp_Object,
					 oldval: Lisp_Object)
{
  let mut alpha: c_double = 1.0;
  let mut newval: [c_double; 2] = [0.0; 2];
  let mut i: c_int;
  let mut item: Lisp_Object = Qnil ();

  i = 0;
  while i < 2
    {
      newval[i as usize] = 1.0;
      if CONSP (arg)
	{
	  item = XCAR (arg);
	  arg  = XCDR (arg);
	}
      else
	{ item = arg; }

      if NILP (item)
	{ alpha = -1.0; }
      else if FLOATP (item)
	{
	  alpha = XFLOAT_DATA (item);
	  if ! (0.0 <= alpha && alpha <= 1.0)
	    { args_out_of_range (make_float (0.0), make_float (1.0)); }
	}
      else if FIXNUMP (item)
	{
	  let ialpha: EMACS_INT = XFIXNUM (item);
	  if ! (0 <= ialpha && ialpha <= 100)
	    { args_out_of_range (make_fixnum (0), make_fixnum (100)); }
	  alpha = ialpha as c_double / 100.0;
	}
      else
	{ wrong_type_argument (Qnumberp (), item); }
      newval[i as usize] = alpha;
      i += 1;
    }

  i = 0;
  while i < 2
    {
      (*f).alpha[i as usize] = newval[i as usize];
      i += 1;
    }

  if (*FRAME_TERMINAL! (f)).set_frame_alpha_hook.is_some ()
    {
      block_input ();
      if let Some (set_frame_alpha_hook) = (*FRAME_TERMINAL! (f)).set_frame_alpha_hook
	{ set_frame_alpha_hook (f as *mut c_void); }
      unblock_input ();
    }
}

/* C 6097-6124: `gui_set_alpha_background`.  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gui_set_alpha_background (f: *mut frame, arg: Lisp_Object,
						   oldval: Lisp_Object)
{
  let mut alpha: c_double = 1.0;

  if NILP (arg)
    { alpha = 1.0; }
  else if FLOATP (arg)
    {
      alpha = XFLOAT_DATA (arg);
      if ! (0.0 <= alpha && alpha <= 1.0)
	{ args_out_of_range (make_float (0.0), make_float (1.0)); }
    }
  else if FIXNUMP (arg)
    {
      let ialpha: EMACS_INT = XFIXNUM (arg);
      if ! (0 <= ialpha && ialpha <= 100)
	{ args_out_of_range (make_fixnum (0), make_fixnum (100)); }
      alpha = ialpha as c_double / 100.0;
    }
  else
    { wrong_type_argument (Qnumberp (), arg); }

  (*f).alpha_background = alpha;

  recompute_basic_faces (f);
  SET_FRAME_GARBAGED! (f);
}

/* C 6126-6131: `gui_set_borders_respect_alpha_background`.  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gui_set_borders_respect_alpha_background (f: *mut frame,
								   arg: Lisp_Object,
								   oldval: Lisp_Object)
{
  borders_respect_alpha_background! (f, !NILP (arg));
}

/* C 6133-6145: `gui_set_no_special_glyphs`.  */
/**
 * gui_set_no_special_glyphs:
 *
 * Set frame F's `no-special-glyphs' parameter which, if non-nil,
 * suppresses the display of truncation and continuation glyphs
 * outside fringes.
 */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gui_set_no_special_glyphs (f: *mut frame, new_value: Lisp_Object,
						    old_value: Lisp_Object)
{
  if !EQ (new_value, old_value)
    { FRAME_NO_SPECIAL_GLYPHS! (f, !NILP (new_value)); }
}

/* Non-zero if mouse is grabbed on DPYINFO
   and we know the frame where it is.  */
/* C 6151-6159: `gui_mouse_grabbed`.  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gui_mouse_grabbed (dpyinfo: *mut Display_Info) -> bool
{
  return (*dpyinfo).grabbed != 0
    || ((*(*dpyinfo).terminal).any_grab_hook.is_some ()
	&& (if let Some (any_grab_hook) = (*(*dpyinfo).terminal).any_grab_hook
	      { any_grab_hook (dpyinfo as *mut c_void) } else { false }))
    && !(*dpyinfo).last_mouse_frame.is_null ()
    && FRAME_LIVE_P ((*dpyinfo).last_mouse_frame);
}

/* Re-highlight something with mouse-face properties
   on DPYINFO using saved frame and mouse position.  */
/* C 6164-6172: `gui_redo_mouse_highlight`.  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gui_redo_mouse_highlight (dpyinfo: *mut Display_Info)
{
  if !(*dpyinfo).last_mouse_motion_frame.is_null ()
    && FRAME_LIVE_P ((*dpyinfo).last_mouse_motion_frame)
    { note_mouse_highlight ((*dpyinfo).last_mouse_motion_frame,
			    (*dpyinfo).last_mouse_motion_x,
			    (*dpyinfo).last_mouse_motion_y); }
}

/* C's `globals` object defines `Vx_resource_class` for every target, and the port carries it in
   the unit that reads it: C's own `DEFVAR_LISP`s for `x-resource-class` live in the X/GTK/W32/NS
   frontends, none of which is in this target's link set, so the Android unit imported the name and
   the shared library could not be loaded (status 4.751).  `Vx_resource_name` (read by the same
   function) is carried here for the same reason.  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)] pub static mut Vx_resource_class: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)] pub static mut Vx_resource_name: Lisp_Object = lisp_h_Qnil ();

/* Subroutines of creating an X frame.  */

/* Make sure that Vx_resource_name is set to a reasonable value.
   Fix it up, or set it to `emacs' if it is too hopeless.  */
/* C 6179-6243: `validate_x_resource_name`.  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn validate_x_resource_name ()
{
  let mut len: ptrdiff_t = 0;
  /* Number of valid characters in the resource name.  */
  let mut good_count: ptrdiff_t = 0;
  /* Number of invalid characters in the resource name.  */
  let mut bad_count: ptrdiff_t = 0;
  let new: Lisp_Object;
  let mut i: ptrdiff_t;

  if !STRINGP (Vx_resource_class)
    { Vx_resource_class = build_string (EMACS_CLASS.as_ptr () as *const c_char); }

  if STRINGP (Vx_resource_name)
    {
      let p: *mut u8 = SDATA (Vx_resource_name);

      len = SBYTES (Vx_resource_name);

      /* Only letters, digits, - and _ are valid in resource names.
	 Count the valid characters and count the invalid ones.  */
      i = 0;
      while i < len
	{
	  let c: c_int = *p.offset (i as isize) as c_int;
	  if ! ((c >= 'a' as c_int && c <= 'z' as c_int)
		 || (c >= 'A' as c_int && c <= 'Z' as c_int)
		 || (c >= '0' as c_int && c <= '9' as c_int)
		 || c == '-' as c_int || c == '_' as c_int)
	    { bad_count += 1; }
	  else
	    { good_count += 1; }
	  i += 1;
	}
    }
  else
    /* Not a string => completely invalid.  */
    { bad_count = 5; good_count = 0; }

  /* If name is valid already, return.  */
  if bad_count == 0
    { return; }

  /* If name is entirely invalid, or nearly so, or is so implausibly
     large that alloca might not work, use `emacs'.  */
  if good_count < 2 || MAX_ALLOCA - 16 < len /* C's `sizeof ".customization"` (15 bytes + NUL) */
    {
      Vx_resource_name = build_string (c"emacs".as_ptr ());
      return;
    }

  /* Name is partly valid.  Copy it and replace the invalid characters
     with underscores.  */

  Vx_resource_name = Fcopy_sequence (Vx_resource_name);
  new = Vx_resource_name;

  i = 0;
  while i < len
    {
      let c: c_int = SREF (new, i as usize) as c_int;
      if ! ((c >= 'a' as c_int && c <= 'z' as c_int)
	     || (c >= 'A' as c_int && c <= 'Z' as c_int)
	     || (c >= '0' as c_int && c <= '9' as c_int)
	     || c == '-' as c_int || c == '_' as c_int)
	{ SSET (new, i as usize, b'_'); }
      i += 1;
    }
}

/* Get a GUI resource, like Fx_get_resource, but for display DPYINFO.
   See Fx_get_resource below for other parameters.  */
/* C 6248-6319: `gui_display_get_resource`.  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gui_display_get_resource (dpyinfo: *mut Display_Info,
						   attribute: Lisp_Object,
						   class: Lisp_Object,
						   component: Lisp_Object,
						   subclass: Lisp_Object)
  -> Lisp_Object
{
  CHECK_STRING (attribute);
  CHECK_STRING (class);

  if !NILP (component)
    { CHECK_STRING (component); }
  if !NILP (subclass)
    { CHECK_STRING (subclass); }
  if NILP (component) != NILP (subclass)
    { error (c"x-get-resource: must specify both COMPONENT and SUBCLASS or neither".as_ptr ()); }

  validate_x_resource_name ();

  /* Allocate space for the components, the dots which separate them,
     and the final '\0'.  Make them big enough for the worst case.  */
  let name_keysize: ptrdiff_t = SBYTES (Vx_resource_name)
			    + (if STRINGP (component) { SBYTES (component) } else { 0 })
			    + SBYTES (attribute)
			    + 3;

  let class_keysize: ptrdiff_t = SBYTES (Vx_resource_class)
			     + SBYTES (class)
			     + (if STRINGP (subclass) { SBYTES (subclass) } else { 0 })
			     + 3;

  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
  let name_key: *mut c_char = SAFE_ALLOCA! (sa_avail, name_keysize + class_keysize) as *mut c_char;
  let class_key: *mut c_char = name_key.offset (name_keysize as isize);

  /* Start with emacs.FRAMENAME for the name (the specific one)
     and with `Emacs' for the class key (the general one).  */
  let mut nz: *mut c_char = lispstpcpy (name_key, Vx_resource_name);
  let mut cz: *mut c_char = lispstpcpy (class_key, Vx_resource_class);

  *cz = b'.' as c_char;
  cz = cz.offset (1);
  cz = lispstpcpy (cz, class);

  if !NILP (component)
    {
      *cz = b'.' as c_char;
      cz = cz.offset (1);
      lispstpcpy (cz, subclass);

      *nz = b'.' as c_char;
      nz = nz.offset (1);
      nz = lispstpcpy (nz, component);
    }

  *nz = b'.' as c_char;
  nz = nz.offset (1);
  lispstpcpy (nz, attribute);

  /* C 6302-6318: `#ifndef HAVE_ANDROID` (the live arm on this configuration's GUIs; the
     `#else` arm returns `Qnil' after the same `SAFE_FREE`).  */
  #[cfg (not (feature = "have_android"))]
  {
    let value: *const c_char
      = if let Some (get_string_resource_hook)
	  = (*(*dpyinfo).terminal).get_string_resource_hook
	{ get_string_resource_hook (&raw mut (*dpyinfo).rdb as *mut c_void,
				    name_key, class_key) }
	else { std::ptr::null () };

    SAFE_FREE! (sa_count, sa_mark);

    if !value.is_null () && *value != 0
      { return build_string (value); }
    else
      { return Qnil (); }
  }

  #[cfg (feature = "have_android")]
  {
    SAFE_FREE! (sa_count, sa_mark);
    return Qnil ();
  }
}

/* C 6322-6339: `Fx_get_resource`.  */
#[cfg (feature = "have_window_system")]
DEFUN! ("x-get-resource", Fx_get_resource, Sx_get_resource, 2, 4, 0,
       doc: "Return the value of ATTRIBUTE, of class CLASS, from the X defaults database.\nThis uses `INSTANCE.ATTRIBUTE' as the key and `Emacs.CLASS' as the\nclass, where INSTANCE is the name under which Emacs was invoked, or\nthe name specified by the `-name' or `-rn' command-line arguments.\n\nThe optional arguments COMPONENT and SUBCLASS add to the key and the\nclass, respectively.  You must specify both of them or neither.\nIf you specify them, the key is `INSTANCE.COMPONENT.ATTRIBUTE'\nand the class is `Emacs.CLASS.SUBCLASS'.  ",
       (attribute: Lisp_Object, class: Lisp_Object, component: Lisp_Object,
	subclass: Lisp_Object),
       {
  check_window_system (std::ptr::null_mut ());

  return gui_display_get_resource (check_x_display_info (Qnil ()),
				   attribute, class, component, subclass);
});

/* C 6341-6366: `x_get_resource_string`, under
   `#if defined HAVE_X_WINDOWS && !defined USE_X_TOOLKIT && !defined USE_GTK`; used when C code
   wants a resource value (called from oldXMenu/Create.c).  */
#[cfg (feature = "have_window_system")]
#[cfg (all (feature = "have_x_windows", not (feature = "use_x_toolkit"),
	    not (feature = "use_gtk")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x_get_resource_string (attribute: *const c_char,
						class: *const c_char) -> *const c_char
{
  let result: *const c_char;
  let sf: *mut frame = SELECTED_FRAME ();
  let invocation_namelen: ptrdiff_t = SBYTES (Vinvocation_name);
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);

  /* Allocate space for the components, the dots which separate them,
     and the final '\0'.  */
  let name_keysize: ptrdiff_t = invocation_namelen + strlen (attribute) as isize + 2;
  let class_keysize: ptrdiff_t = EMACS_CLASS.len () as isize + strlen (class) as isize + 2;
  let name_key: *mut c_char = SAFE_ALLOCA! (sa_avail, name_keysize + class_keysize) as *mut c_char;
  let class_key: *mut c_char = name_key.offset (name_keysize as isize);
  esprintf (name_key, c"%s.%s".as_ptr (), SSDATA (Vinvocation_name), attribute);
  sprintf (class_key, c"%s.%s".as_ptr (), EMACS_CLASS.as_ptr () as *const c_char, class);

  result = x_get_string_resource (&raw mut (*FRAME_DISPLAY_INFO! (sf)).rdb as *mut c_void,
				  name_key, class_key);
  SAFE_FREE! (sa_count, sa_mark);
  return result;
}

/* Return the value of parameter PARAM.

   First search ALIST, then Vdefault_frame_alist, then the GUI
   resource database, using ATTRIBUTE as the attribute name and CLASS
   as its class.

   Convert the resource to the type specified by desired_type.

   If no default is specified, return Qunbound.  If you call
   gui_display_get_arg, make sure you deal with Qunbound in a
   reasonable way, and don't let it get stored in any Lisp-visible
   variables!  */
/* C 6381-6478: `gui_display_get_arg`.  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gui_display_get_arg (dpyinfo: *mut Display_Info, alist: Lisp_Object,
					      param: Lisp_Object, attribute: *const c_char,
					      class: *const c_char, r#type: resource_types)
  -> Lisp_Object
{
  let mut tem: Lisp_Object;

  tem = Fassq (param, alist);

  if !NILP (tem)
    {
      /* If we find this parm in ALIST, clear it out
	 so that it won't be "left over" at the end.  */
      let mut tail: Lisp_Object;
      XSETCAR (tem, Qnil ());
      /* In case the parameter appears more than once in the alist,
	 clear it out.  */
      tail = alist;
      while CONSP (tail)
	{
	  if CONSP (XCAR (tail))
	    && EQ (XCAR (XCAR (tail)), param)
	    { XSETCAR (XCAR (tail), Qnil ()); }
	  tail = XCDR (tail);
	}
    }
  else
    { tem = Fassq (param, Vdefault_frame_alist); }

  /* If it wasn't specified in ALIST or the Lisp-level defaults,
     look in the X resources.  */
  if NILP (tem)
    {
      if !attribute.is_null () && !dpyinfo.is_null ()
	{
	  AUTO_STRING! (at, attribute);
	  AUTO_STRING! (cl, class);
	  tem = gui_display_get_resource (dpyinfo, at, cl, Qnil (), Qnil ());

	  if NILP (tem)
	    { return Qunbound (); }

	  match r#type
	    {
	      RES_TYPE_NUMBER =>
		{ return make_fixnum (atoi (SSDATA (tem)) as EMACS_INT); }

	      RES_TYPE_BOOLEAN_NUMBER =>
		{
		  if strcmp (SSDATA (tem), c"on".as_ptr ()) == 0
		      || strcmp (SSDATA (tem), c"true".as_ptr ()) == 0
		    { return make_fixnum (1); }
		  return make_fixnum (atoi (SSDATA (tem)) as EMACS_INT);
		}

	      RES_TYPE_FLOAT =>
		{ return make_float (atof (SSDATA (tem))); }

	      RES_TYPE_BOOLEAN =>
		{
		  tem = Fdowncase (tem);
		  if strcmp (SSDATA (tem), c"on".as_ptr ()) == 0
		    /* C 6437-6439's `#ifdef HAVE_NS` alternative (`|| !strcmp (SSDATA (tem), "yes")`).  */
		    || { #[cfg (feature = "have_ns")]
			 { strcmp (SSDATA (tem), c"yes".as_ptr ()) == 0 }
			 #[cfg (not (feature = "have_ns"))]
			 { false } }
		    || strcmp (SSDATA (tem), c"true".as_ptr ()) == 0
		    { return Qt (); }
		  else
		    { return Qnil (); }
		}

	      RES_TYPE_STRING =>
		{ return tem; }

	      RES_TYPE_SYMBOL =>
		{
		  /* As a special case, we map the values `true' and `on'
		     to Qt, and `false' and `off' to Qnil.  */
		  let lower: Lisp_Object;
		  lower = Fdowncase (tem);
		  if strcmp (SSDATA (lower), c"on".as_ptr ()) == 0
		    /* C 6455-6457's `#ifdef HAVE_NS` alternative.  */
		    || { #[cfg (feature = "have_ns")]
			 { strcmp (SSDATA (lower), c"yes".as_ptr ()) == 0 }
			 #[cfg (not (feature = "have_ns"))]
			 { false } }
		    || strcmp (SSDATA (lower), c"true".as_ptr ()) == 0
		    { return Qt (); }
		  else if strcmp (SSDATA (lower), c"off".as_ptr ()) == 0
		    /* C 6461-6463's `#ifdef HAVE_NS` alternative.  */
		    || { #[cfg (feature = "have_ns")]
			 { strcmp (SSDATA (lower), c"no".as_ptr ()) == 0 }
			 #[cfg (not (feature = "have_ns"))]
			 { false } }
		    || strcmp (SSDATA (lower), c"false".as_ptr ()) == 0
		    { return Qnil (); }
		  else
		    { return Fintern (tem, Qnil ()); }
		}

	      _ =>
		{ emacs_abort (); }
	    }
	}
      else
	{ return Qunbound (); }
    }
  return Fcdr (tem);
}

/* C 6480-6487: `gui_frame_get_arg`.  `static` in C, so no `#[unsafe(no_mangle)]`.  */
#[cfg (feature = "have_window_system")]
unsafe fn gui_frame_get_arg (f: *mut frame, alist: Lisp_Object, param: Lisp_Object,
			     attribute: *const c_char, class: *const c_char,
			     r#type: resource_types) -> Lisp_Object
{
  return gui_display_get_arg (FRAME_DISPLAY_INFO! (f), alist, param, attribute, class,
			      r#type);
}

/* Like gui_frame_get_arg, but also record the value in f->param_alist.  */
/* C 6491-6505: `gui_frame_get_and_record_arg`.  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gui_frame_get_and_record_arg (f: *mut frame, alist: Lisp_Object,
						       param: Lisp_Object,
						       attribute: *const c_char,
						       class: *const c_char,
						       r#type: resource_types)
  -> Lisp_Object
{
  let value: Lisp_Object;

  value = gui_display_get_arg (FRAME_DISPLAY_INFO! (f), alist, param, attribute, class,
			       r#type);
  if ! NILP (value) && ! BASE_EQ (value, Qunbound ())
    { store_frame_param (f, param, value); }

  return value;
}

/* Record in frame F the specified or default value according to ALIST
   of the parameter named PROP (a Lisp symbol).
   If no value is specified for PROP, look for an X default for XPROP
   on the frame named NAME.
   If that is not found either, use the value DEFLT.  */
/* C 6514-6535: `gui_default_parameter`.  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gui_default_parameter (f: *mut frame, alist: Lisp_Object,
						prop: Lisp_Object, deflt: Lisp_Object,
						xprop: *const c_char, xclass: *const c_char,
						r#type: resource_types) -> Lisp_Object
{
  let mut tem: Lisp_Object;
  let was_unbound: bool;

  tem = gui_frame_get_arg (f, alist, prop, xprop, xclass, r#type);

  if BASE_EQ (tem, Qunbound ())
    {
      tem = deflt;
      was_unbound = true;
    }
  else
    { was_unbound = false; }

  AUTO_FRAME_ARG! (arg, prop, tem);
  gui_set_frame_parameters_1 (f, arg, was_unbound);
  return tem;
}


/* C 6538-6626: `#if !defined (HAVE_X_WINDOWS) && defined (NoValue)`

     XParseGeometry parses strings of the form
    "=<width>x<height>{+-}<xoffset>{+-}<yoffset>", where
    width, height, xoffset, and yoffset are unsigned integers.
    Example:  "=80x24+300-49"
    The equal sign is optional.
    It returns a bitmask that indicates which of the four values
    were actually found in the string.  For each value found,
    the corresponding argument is updated;  for each value
    not found, the corresponding argument is left unchanged.  */
#[cfg (feature = "have_window_system")]
#[cfg (any (all (not (feature = "have_x_windows"), feature = "no_value"), feature = "have_android"))]
unsafe fn XParseGeometry (mut string: *mut c_char, x: *mut c_int, y: *mut c_int,
			  width: *mut c_uint, height: *mut c_uint) -> c_int
{
  let mut mask: c_int = NoValue;
  let mut strind: *mut c_char;
  let mut tempWidth: c_ulong = 0;      /* C: `unsigned long tempWidth UNINIT` */
  let mut tempHeight: c_ulong = 0;     /* C: `unsigned long tempHeight UNINIT` */
  let mut tempX: c_long = 0;           /* C: `long int tempX UNINIT` */
  let mut tempY: c_long = 0;           /* C: `long int tempY UNINIT` */
  let mut nextCharacter: *mut c_char = std::ptr::null_mut ();

  if string.is_null () || *string == 0
    { return mask; }
  if *string == b'=' as c_char
    { string = string.offset (1); }  /* ignore possible '=' at beg of geometry spec */

  strind = string;
  if *strind != b'+' as c_char && *strind != b'-' as c_char && *strind != b'x' as c_char
    {
      tempWidth = strtoul (strind, &raw mut nextCharacter, 10);
      if strind == nextCharacter
	{ return 0; }
      strind = nextCharacter;
      mask |= WidthValue;
    }

  if *strind == b'x' as c_char || *strind == b'X' as c_char
    {
      strind = strind.offset (1);
      tempHeight = strtoul (strind, &raw mut nextCharacter, 10);
      if strind == nextCharacter
	{ return 0; }
      strind = nextCharacter;
      mask |= HeightValue;
    }

  if *strind == b'+' as c_char || *strind == b'-' as c_char
    {
      if *strind == b'-' as c_char
	{ mask |= XNegative; }
      tempX = strtol (strind, &raw mut nextCharacter, 10);
      if strind == nextCharacter
	{ return 0; }
      strind = nextCharacter;
      mask |= XValue;
      if *strind == b'+' as c_char || *strind == b'-' as c_char
	{
	  if *strind == b'-' as c_char
	    { mask |= YNegative; }
	  tempY = strtol (strind, &raw mut nextCharacter, 10);
	  if strind == nextCharacter
	    { return 0; }
	  strind = nextCharacter;
	  mask |= YValue;
	}
    }

  /* If strind isn't at the end of the string then it's an invalid
     geometry specification. */
  if *strind != 0
    { return 0; }

  if mask & XValue != 0
    { *x = clip_to_bounds (INT_MIN as isize, tempX as isize, INT_MAX as isize) as c_int; }
  if mask & YValue != 0
    { *y = clip_to_bounds (INT_MIN as isize, tempY as isize, INT_MAX as isize) as c_int; }
  if mask & WidthValue != 0
    { *width = min (tempWidth as isize, UINT_MAX as isize) as c_uint; }
  if mask & HeightValue != 0
    { *height = min (tempHeight as isize, UINT_MAX as isize) as c_uint; }
  return mask;
}


/* NS used to define x-parse-geometry in ns-win.el, but that confused
   make-docfile: the documentation string in ns-win.el was used for
   x-parse-geometry even in non-NS builds.

   With two definitions of x-parse-geometry in this file, various
   things still get confused (eg M-x apropos documentation), so that
   it is best if the two definitions just share the same doc-string.  */
/* C 6637-6696: `Fx_parse_geometry`.  */
#[cfg (feature = "have_window_system")]
DEFUN! ("x-parse-geometry", Fx_parse_geometry, Sx_parse_geometry, 1, 1, 0,
       doc: "Parse a display geometry string STRING.\nReturns an alist of the form ((top . TOP), (left . LEFT) ... ).\nThe properties returned may include `top', `left', `height', and `width'.\nFor X, the value of `left' or `top' may be an integer,\nor a list (+ N) meaning N pixels relative to top/left corner,\nor a list (- N) meaning -N pixels relative to bottom/right corner.\nOn Nextstep, this just calls `ns-parse-geometry'.  ",
       (string: Lisp_Object),
       {
  /* x and y don't need initialization, as they are not accessed
     unless XParseGeometry sets them, in which case it always returns
     a non-zero value.  */
  let mut x: c_int = 0;              /* C: `int x UNINIT` */
  let mut y: c_int = 0;              /* C: `int y UNINIT` */
  let mut width: c_uint;
  let mut height: c_uint;

  width = 0;
  height = 0;

  CHECK_STRING (string);

  /* C 6657-6660's `#ifdef HAVE_NS`.  */
  #[cfg (feature = "have_ns")]
  {
    if !strchr (SSDATA (string), b' ' as c_int).is_null ()
      { return calln! (Qns_parse_geometry (), string); }
  }
  #[cfg (any (all (not (feature = "have_x_windows"), feature = "no_value"), feature = "have_android"))]
  let geometry: c_int = XParseGeometry (SSDATA (string) as *mut c_char, &raw mut x, &raw mut y,
					&raw mut width, &raw mut height);
  let mut result: Lisp_Object = Qnil ();
  if geometry & XValue != 0
    {
      let element: Lisp_Object;

      if x >= 0 && (geometry & XNegative) != 0
	{ element = list3 (Qleft (), Qminus (), make_fixnum (-x as EMACS_INT)); }
      else if x < 0 && (geometry & XNegative) == 0
	{ element = list3 (Qleft (), Qplus (), make_fixnum (x as EMACS_INT)); }
      else
	{ element = Fcons (Qleft (), make_fixnum (x as EMACS_INT)); }
      result = Fcons (element, result);
    }

  if geometry & YValue != 0
    {
      let element: Lisp_Object;

      if y >= 0 && (geometry & YNegative) != 0
	{ element = list3 (Qtop (), Qminus (), make_fixnum (-y as EMACS_INT)); }
      else if y < 0 && (geometry & YNegative) == 0
	{ element = list3 (Qtop (), Qplus (), make_fixnum (y as EMACS_INT)); }
      else
	{ element = Fcons (Qtop (), make_fixnum (y as EMACS_INT)); }
      result = Fcons (element, result);
    }

  if geometry & WidthValue != 0
    { result = Fcons (Fcons (Qwidth (), make_fixnum (width as EMACS_INT)), result); }
  if geometry & HeightValue != 0
    { result = Fcons (Fcons (Qheight (), make_fixnum (height as EMACS_INT)), result); }

  return result;
});


/* Calculate the desired size and position of frame F.
   Return the flags saying which aspects were specified.

   Also set the win_gravity and size_hint_flags of F.

   Adjust height for toolbar if TOOLBAR_P is 1.

   This function does not make the coordinates positive.  */
/* C 6708-6967: `gui_figure_window_size`.  */
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gui_figure_window_size (f: *mut frame, parms: Lisp_Object,
						 tabbar_p: bool, toolbar_p: bool) -> c_long
{
  let mut height: Lisp_Object;
  let mut width: Lisp_Object;
  let mut user_size: Lisp_Object;
  let mut top: Lisp_Object;
  let mut left: Lisp_Object;
  let mut user_position: Lisp_Object;
  let mut window_prompting: c_long = 0;
  let dpyinfo: *mut Display_Info = FRAME_DISPLAY_INFO! (f);
  let mut parent_done: c_int = -1;
  let mut outer_done: c_int = -1;
  let mut text_width: c_int = 80 * FRAME_COLUMN_WIDTH (f);
  let mut text_height: c_int = 36 * FRAME_LINE_HEIGHT (f);

  /* Window managers expect that if program-specified
     positions are not (0,0), they're intentional, not defaults.  */
  (*f).top_pos = 0;
  (*f).left_pos = 0;

  /* Calculate a tab bar height so that the user gets a text display
     area of the size he specified with -g or via .Xdefaults.  Later
     changes of the tab bar height don't change the frame size.  This
     is done so that users can create tall Emacs frames without having
     to guess how tall the tab bar will get.  */
  if tabbar_p && FRAME_TAB_BAR_LINES! (f) != 0
    {
      if frame_default_tab_bar_height != 0
	/* A default tab bar height was already set by the display code
	   for some other frame, use that.  */
	{ FRAME_TAB_BAR_HEIGHT! (f) = frame_default_tab_bar_height; }
      else
	/* Calculate the height from various other settings.  For some
	   reason, these are usually off by 2 hence of no use.  */
	{
	  let margin: c_int;
	  let relief: c_int;

	  relief = if tab_bar_button_relief < 0
		   { DEFAULT_TAB_BAR_BUTTON_RELIEF }
		   else { min (tab_bar_button_relief, 1000000) as c_int };

	  if RANGED_FIXNUMP (1, Vtab_bar_button_margin, INT_MAX as EMACS_INT)
	    { margin = XFIXNAT (Vtab_bar_button_margin) as c_int; }
	  else if CONSP (Vtab_bar_button_margin)
		   && RANGED_FIXNUMP (1, XCDR (Vtab_bar_button_margin), INT_MAX as EMACS_INT)
	    { margin = XFIXNAT (XCDR (Vtab_bar_button_margin)) as c_int; }
	  else
	    { margin = 0; }

	  FRAME_TAB_BAR_HEIGHT! (f)
	    = DEFAULT_TAB_BAR_IMAGE_HEIGHT + 2 * margin + 2 * relief;
	}
    }

  /* Calculate a tool bar height so that the user gets a text display
     area of the size he specified with -g or via .Xdefaults.  Later
     changes of the tool bar height don't change the frame size.  This
     is done so that users can create tall Emacs frames without having
     to guess how tall the tool bar will get.  */
  if toolbar_p && FRAME_TOOL_BAR_LINES (f) != 0
    {
      if frame_default_tool_bar_height != 0
	{ FRAME_TOOL_BAR_HEIGHT! (f) = frame_default_tool_bar_height; }
      else
	{
	  let margin: c_int;
	  let relief: c_int;

	  relief = if tool_bar_button_relief < 0
		   { DEFAULT_TOOL_BAR_BUTTON_RELIEF }
		   else { min (tool_bar_button_relief, 1000000) as c_int };

	  if RANGED_FIXNUMP (1, Vtool_bar_button_margin, INT_MAX as EMACS_INT)
	    { margin = XFIXNAT (Vtool_bar_button_margin) as c_int; }
	  else if CONSP (Vtool_bar_button_margin)
		   && RANGED_FIXNUMP (1, XCDR (Vtool_bar_button_margin), INT_MAX as EMACS_INT)
	    { margin = XFIXNAT (XCDR (Vtool_bar_button_margin)) as c_int; }
	  else
	    { margin = 0; }

	  FRAME_TOOL_BAR_HEIGHT! (f)
	    = DEFAULT_TOOL_BAR_IMAGE_HEIGHT + 2 * margin + 2 * relief;
	}
    }

  /* Ensure that earlier new_width and new_height settings won't
     override what we specify below.  */
  (*f).new_width = -1;
  (*f).new_height = -1;

  height = gui_display_get_arg (dpyinfo, parms, Qheight (), std::ptr::null (),
				std::ptr::null (), RES_TYPE_NUMBER);
  width = gui_display_get_arg (dpyinfo, parms, Qwidth (), std::ptr::null (),
			       std::ptr::null (), RES_TYPE_NUMBER);
  if !BASE_EQ (width, Qunbound ()) || !BASE_EQ (height, Qunbound ())
    {
      if !BASE_EQ (width, Qunbound ())
	{
	  if CONSP (width) && EQ (XCAR (width), Qtext_pixels ())
	    {
	      CHECK_FIXNUM (XCDR (width));
	      if XFIXNUM (XCDR (width)) < 0 || XFIXNUM (XCDR (width)) > INT_MAX as EMACS_INT
		{ xsignal1 (Qargs_out_of_range (), XCDR (width)); }

	      text_width = XFIXNUM (XCDR (width)) as c_int;
	    }
	  else if FLOATP (width)
	    {
	      let d_width: c_double = XFLOAT_DATA (width);

	      if d_width < 0.0 || d_width > 1.0
		{ xsignal1 (Qargs_out_of_range (), width); }
	      else
		{
		  let new_width: c_int = frame_float (f, width, FRAME_FLOAT_WIDTH,
						      &raw mut parent_done, &raw mut outer_done, -1);

		  if new_width > -1
		    { text_width = new_width; }
		}
	    }
	  else
	    {
	      CHECK_FIXNUM (width);
	      if XFIXNUM (width) < 0 || XFIXNUM (width) > INT_MAX as EMACS_INT
		{ xsignal1 (Qargs_out_of_range (), width); }

	      text_width = (XFIXNUM (width) * FRAME_COLUMN_WIDTH (f) as EMACS_INT) as c_int;
	    }
	}

      if !BASE_EQ (height, Qunbound ())
	{
	  if CONSP (height) && EQ (XCAR (height), Qtext_pixels ())
	    {
	      CHECK_FIXNUM (XCDR (height));
	      if XFIXNUM (XCDR (height)) < 0 || XFIXNUM (XCDR (height)) > INT_MAX as EMACS_INT
		{ xsignal1 (Qargs_out_of_range (), XCDR (height)); }

	      text_height = XFIXNUM (XCDR (height)) as c_int;
	    }
	  else if FLOATP (height)
	    {
	      let d_height: c_double = XFLOAT_DATA (height);

	      if d_height < 0.0 || d_height > 1.0
		{ xsignal1 (Qargs_out_of_range (), height); }
	      else
		{
		  let new_height: c_int = frame_float (f, height, FRAME_FLOAT_HEIGHT,
						       &raw mut parent_done, &raw mut outer_done, -1);

		  if new_height > -1
		    { text_height = new_height; }
		}
	    }
	  else
	    {
	      CHECK_FIXNUM (height);
	      if XFIXNUM (height) < 0 || XFIXNUM (height) > INT_MAX as EMACS_INT
		{ xsignal1 (Qargs_out_of_range (), height); }

	      text_height = (XFIXNUM (height) * FRAME_LINE_HEIGHT (f) as EMACS_INT) as c_int;
	    }
	}

      user_size = gui_display_get_arg (dpyinfo, parms, Quser_size (), std::ptr::null (),
				       std::ptr::null (), RES_TYPE_NUMBER);
      if !NILP (user_size) && !BASE_EQ (user_size, Qunbound ())
	{ window_prompting |= USSize; }
      else
	{ window_prompting |= PSize; }
    }

  adjust_frame_size (f, text_width, text_height, 5, false, Qgui_figure_window_size ());

  top = gui_display_get_arg (dpyinfo, parms, Qtop (), std::ptr::null (),
			     std::ptr::null (), RES_TYPE_NUMBER);
  left = gui_display_get_arg (dpyinfo, parms, Qleft (), std::ptr::null (),
			      std::ptr::null (), RES_TYPE_NUMBER);
  user_position = gui_display_get_arg (dpyinfo, parms, Quser_position (),
				       std::ptr::null (), std::ptr::null (),
				       RES_TYPE_NUMBER);
  if ! BASE_EQ (top, Qunbound ()) || ! BASE_EQ (left, Qunbound ())
    {
      if EQ (top, Qminus ())
	{
	  (*f).top_pos = 0;
	  window_prompting |= YNegative as c_long;
	}
      else if CONSP (top) && EQ (XCAR (top), Qminus ())
	       && CONSP (XCDR (top))
	       && RANGED_FIXNUMP (-(INT_MAX as EMACS_INT), XCAR (XCDR (top)),
				  INT_MAX as EMACS_INT)
	{
	  (*f).top_pos = -XFIXNUM (XCAR (XCDR (top))) as c_int;
	  window_prompting |= YNegative as c_long;
	}
      else if CONSP (top) && EQ (XCAR (top), Qplus ())
	       && CONSP (XCDR (top))
	       && TYPE_RANGED_FIXNUMP (INT_MIN as EMACS_INT, XCAR (XCDR (top)),
				       INT_MAX as EMACS_INT)
	{
	  (*f).top_pos = XFIXNUM (XCAR (XCDR (top))) as c_int;
	}
      else if FLOATP (top)
	{ (*f).top_pos = frame_float (f, top, FRAME_FLOAT_TOP, &raw mut parent_done,
				     &raw mut outer_done, 0); }
      else if BASE_EQ (top, Qunbound ())
	{ (*f).top_pos = 0; }
      else
	{
	  (*f).top_pos = check_integer_range (top, INT_MIN as intmax_t,
					      INT_MAX as intmax_t) as c_int;
	  if (*f).top_pos < 0
	    { window_prompting |= YNegative as c_long; }
	}

      if EQ (left, Qminus ())
	{
	  (*f).left_pos = 0;
	  window_prompting |= XNegative as c_long;
	}
      else if CONSP (left) && EQ (XCAR (left), Qminus ())
	       && CONSP (XCDR (left))
	       && RANGED_FIXNUMP (-(INT_MAX as EMACS_INT), XCAR (XCDR (left)),
				  INT_MAX as EMACS_INT)
	{
	  (*f).left_pos = -XFIXNUM (XCAR (XCDR (left))) as c_int;
	  window_prompting |= XNegative as c_long;
	}
      else if CONSP (left) && EQ (XCAR (left), Qplus ())
	       && CONSP (XCDR (left))
	       && TYPE_RANGED_FIXNUMP (INT_MIN as EMACS_INT, XCAR (XCDR (left)),
				       INT_MAX as EMACS_INT)
	{
	  (*f).left_pos = XFIXNUM (XCAR (XCDR (left))) as c_int;
	}
      else if FLOATP (left)
	{ (*f).left_pos = frame_float (f, left, FRAME_FLOAT_LEFT, &raw mut parent_done,
				      &raw mut outer_done, 0); }
      else if BASE_EQ (left, Qunbound ())
	{ (*f).left_pos = 0; }
      else
	{
	  (*f).left_pos = check_integer_range (left, INT_MIN as intmax_t,
					      INT_MAX as intmax_t) as c_int;
	  if (*f).left_pos < 0
	    { window_prompting |= XNegative as c_long; }
	}

      if !NILP (user_position) && ! BASE_EQ (user_position, Qunbound ())
	{ window_prompting |= USPosition; }
      else
	{ window_prompting |= PPosition; }
    }

  if window_prompting & (XNegative as c_long) != 0
    {
      if window_prompting & (YNegative as c_long) != 0
	{ (*f).win_gravity = SouthEastGravity; }
      else
	{ (*f).win_gravity = NorthEastGravity; }
    }
  else
    {
      if window_prompting & (YNegative as c_long) != 0
	{ (*f).win_gravity = SouthWestGravity; }
      else
	{ (*f).win_gravity = NorthWestGravity; }
    }

  (*f).size_hint_flags = window_prompting as c_int;

  return window_prompting;
}

/* C 6971: `#endif /* HAVE_WINDOW_SYSTEM */` - the run's end, text only.  */


/* C 6973-6999's two pointer-visibility helpers.  `frame_make_pointer_invisible` is declared
   at the crate root by `include/frame.rs` (frame.h 1506), so the definition lives in a
   glob-imported module: a second crate-root item of the same name would be `E0428` (the
   `frame_globals`/terminal.rs device; `docs/ports/frame.md` reports it).  */
pub mod frame_pointer_fns {
  use super::*;

  #[unsafe(no_mangle)]
  pub unsafe extern "C" fn frame_make_pointer_invisible (f: *mut frame)
  {
    if !NILP (Vmake_pointer_invisible)
      {
        if !f.is_null () && FRAME_LIVE_P (f) && !pointer_invisible! (f)
            && (*FRAME_TERMINAL! (f)).toggle_invisible_pointer_hook.is_some ()
          {
            mouse_moved! (f, 0);
            if let Some (toggle_invisible_pointer_hook)
	      = (*FRAME_TERMINAL! (f)).toggle_invisible_pointer_hook
	      { toggle_invisible_pointer_hook (f as *mut c_void, true); }
            pointer_invisible! (f, 1);
          }
      }
  }

  #[unsafe(no_mangle)]
  pub unsafe extern "C" fn frame_make_pointer_visible (f: *mut frame)
  {
    /* We don't check Vmake_pointer_invisible here in case the
       pointer was invisible when Vmake_pointer_invisible was set to nil.  */
    if !f.is_null () && FRAME_LIVE_P (f) && pointer_invisible! (f) && mouse_moved! (f)
        && (*FRAME_TERMINAL! (f)).toggle_invisible_pointer_hook.is_some ()
      {
        if let Some (toggle_invisible_pointer_hook)
	  = (*FRAME_TERMINAL! (f)).toggle_invisible_pointer_hook
	  { toggle_invisible_pointer_hook (f as *mut c_void, false); }
        pointer_invisible! (f, 0);
      }
  }
}
use frame_pointer_fns::*;

DEFUN! ("frame-pointer-visible-p", Fframe_pointer_visible_p,
       Sframe_pointer_visible_p, 0, 1, 0,
       doc: "Return t if the mouse pointer displayed on FRAME is visible.\nOtherwise it returns nil.  FRAME omitted or nil means the\nselected frame.  This is useful when `make-pointer-invisible' is set.  ",
       (frame: Lisp_Object),
       {
  return if pointer_invisible! (decode_any_frame (frame)) { Qnil () } else { Qt () };
});

DEFUN! ("mouse-position-in-root-frame", Fmouse_position_in_root_frame,
       Smouse_position_in_root_frame, 0, 0, 0,
       doc: "Return mouse position in selected frame's root frame.\nReturn the position of `mouse-position' in coordinates of the root frame\nof the frame returned by 'mouse-position'.  ",
       (),
       {
  let pos: Lisp_Object = mouse_position (true);
  let frame: Lisp_Object = XCAR (pos);

  if !FRAMEP (frame)
    { return Qnil (); }
  else
    {
      let mut f: *mut frame = XFRAME (frame);
      let mut x: c_int = XFIXNUM (XCAR (XCDR (pos))) as c_int + (*f).left_pos;
      let mut y: c_int = XFIXNUM (XCDR (XCDR (pos))) as c_int + (*f).top_pos;

      f = FRAME_PARENT_FRAME (f);

      while !f.is_null ()
	{
	  x = x + (*f).left_pos;
	  y = y + (*f).top_pos;
	  f = FRAME_PARENT_FRAME (f);
	}

      return Fcons (make_fixnum (x as EMACS_INT), make_fixnum (y as EMACS_INT));
    }
});

DEFUN! ("frame--set-was-invisible", Fframe__set_was_invisible,
       Sframe__set_was_invisible, 2, 2, 0,
       doc: "Set FRAME's was-invisible flag if WAS-INVISIBLE is non-nil.\nThis function is for internal use only.  ",
       (frame: Lisp_Object, was_invisible: Lisp_Object),
       {
  let f: *mut frame = decode_live_frame (frame);

  was_invisible! (f, !NILP (was_invisible));

  return if was_invisible! (f) { Qt () } else { Qnil () };
});

/* C 7055-7112 and 7120-7187's `#ifdef HAVE_WINDOW_SYSTEM` blocks (the X pointer and
   `Display_Info` machinery) are dead on the frozen target and live on Android; the part this
   round writes is `make_monitor_attribute_list` (C 7144-7187) below - `free_monitors` (C 7124-7135)
   is the next row of `docs/baseline/window-arms-plan.tsv`.  */

/// C 7144-7187: `make_monitor_attribute_list` - it turns C's `struct MonitorInfo` array into the
/// Lisp attribute list `frame-monitor-attributes` returns.  Inside C 7120's `#ifdef HAVE_WINDOW_SYSTEM`:
/// dead on the frozen target, live on Android, whose unit imported the name with only the header's
/// declaration (status 4.755).  The `#ifdef HAVE_PGTK` member (`scale_factor`) is C's own arm and
/// this configuration does not take it.
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn make_monitor_attribute_list (monitors: *mut MonitorInfo,
                                                      n_monitors: c_int,
                                                      primary_monitor: c_int,
                                                      monitor_frames: Lisp_Object,
                                                      source: *const c_char) -> Lisp_Object
{
  let mut attributes_list: Lisp_Object = Qnil ();        // C 7146
  let mut primary_monitor_attributes: Lisp_Object = Qnil ();   // C 7147
  let mut i: c_int;                                      // C 7148

  i = 0;
  while i < n_monitors
    {
      let mut geometry: Lisp_Object;
      let mut workarea: Lisp_Object;
      let mut attributes: Lisp_Object = Qnil ();         // C 7151
      let mi: *mut MonitorInfo = &mut *monitors.offset (i as isize);   // C 7152

      if (*mi).geom.width == 0 { i += 1; continue; }

      /* C's `list4i`/`list2i` take `EMACS_INT` (lisp.h 2133-2150), so the `int` fields promote -
	 the mirror's signature says so, and the casts are that promotion, not a change of value.  */
      workarea = list4i ((*mi).work.x as EMACS_INT, (*mi).work.y as EMACS_INT,
			  (*mi).work.width as EMACS_INT, (*mi).work.height as EMACS_INT);
      geometry = list4i ((*mi).geom.x as EMACS_INT, (*mi).geom.y as EMACS_INT,
			  (*mi).geom.width as EMACS_INT, (*mi).geom.height as EMACS_INT);

      if !source.is_null ()
	{
	  attributes = Fcons (Fcons (Qsource (), build_string (source)), attributes);
	}

      attributes = Fcons (Fcons (Qframes (), AREF (monitor_frames, i as isize)),
			  attributes);
      /* C 7165-7167's `#ifdef HAVE_PGTK` arm (`scale_factor`) is dead here.  */
      attributes = Fcons (Fcons (Qmm_size (),
				 list2i ((*mi).mm_width as EMACS_INT, (*mi).mm_height as EMACS_INT)),
			  attributes);
      attributes = Fcons (Fcons (Qworkarea (), workarea), attributes);
      attributes = Fcons (Fcons (Qgeometry (), geometry), attributes);
      if !(*mi).name.is_null ()
	{
	  attributes = Fcons (Fcons (Qname (),
				     make_string ((*mi).name, strlen ((*mi).name) as isize)),
			      attributes);
	}

      if i == primary_monitor
	{ primary_monitor_attributes = attributes; }
      else
	{ attributes_list = Fcons (attributes, attributes_list); }
      i += 1;
    }

  if !NILP (primary_monitor_attributes)
    { attributes_list = Fcons (primary_monitor_attributes, attributes_list); }
  return attributes_list;
}

/***********************************************************************
				Initialization
 ***********************************************************************/

/* C 7195's `static void init_frame_once_for_pdumper (void);` is a forward declaration whose
   definition follows, so the port drops the declaration line (PORTING.md 0.1-12).  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_frame_once ()
{
  staticpro (&raw mut Vframe_list);
  staticpro (&raw mut selected_frame);
  PDUMPER_IGNORE! (last_nonminibuf_frame);
  Vframe_list = Qnil ();
  selected_frame = Qnil ();
  pdumper_do_now_and_after_load (Some (init_frame_once_for_pdumper));
}

unsafe extern "C" fn init_frame_once_for_pdumper ()
{
  PDUMPER_RESET_LV! (Vframe_list, Qnil ());
  PDUMPER_RESET_LV! (selected_frame, Qnil ());
}

/* The `DEFVAR_*` variables this C file owns: C's storage is `globals.f_<vname>`
   (lisp.h 3495-3530), and in the port the mirror (`include/globals.rs`) declares the names in
   its `variables` module while the owning unit defines them here (status §3.29).  */
#[unsafe(no_mangle)]
pub static mut Vdefault_frame_alist: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vdefault_frame_scroll_bars: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut scroll_bar_adjust_thumb_portion_p: bool = false;
#[unsafe(no_mangle)]
pub static mut Vterminal_frame: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vmouse_position_function: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vmouse_highlight: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vmake_pointer_invisible: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vmove_frame_functions: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vdelete_frame_functions: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vafter_delete_frame_functions: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vmenu_bar_mode: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vtab_bar_mode: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vtool_bar_mode: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut resize_mini_frames: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut focus_follows_mouse: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut frame_resize_pixelwise: bool = false;
#[unsafe(no_mangle)]
pub static mut frame_inhibit_implied_resize: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut frame_size_history: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut tooltip_reuse_hidden_frame: bool = false;
#[unsafe(no_mangle)]
pub static mut use_system_tooltips: bool = false;
#[unsafe(no_mangle)]
pub static mut iconify_child_frame: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut expose_hidden_buffer: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut frame_internal_parameters: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut alter_fullscreen_frames: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut delete_frame_choose_selected: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vtoolkit_theme: Lisp_Object = lisp_h_Qnil ();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_frame ()
{
  DEFSYM! (Qframep, c"framep");
  DEFSYM! (Qframe_live_p, c"frame-live-p");
  DEFSYM! (Qframe_windows_min_size, c"frame-windows-min-size");
  DEFSYM! (Qframe_monitor_attributes, c"frame-monitor-attributes");
  DEFSYM! (Qwindow__pixel_to_total, c"window--pixel-to-total");
  DEFSYM! (Qmake_initial_minibuffer_frame, c"make-initial-minibuffer-frame");
  DEFSYM! (Qmru, c"mru");
  DEFSYM! (Qget_mru_frame, c"get-mru-frame");
  DEFSYM! (Qexplicit_name, c"explicit-name");
  DEFSYM! (Qheight, c"height");
  DEFSYM! (Qicon, c"icon");
  DEFSYM! (Qminibuffer, c"minibuffer");
  DEFSYM! (Qundecorated, c"undecorated");
  DEFSYM! (Qno_special_glyphs, c"no-special-glyphs");
  DEFSYM! (Qparent_frame, c"parent-frame");
  DEFSYM! (Qskip_taskbar, c"skip-taskbar");
  DEFSYM! (Qno_focus_on_map, c"no-focus-on-map");
  DEFSYM! (Qno_accept_focus, c"no-accept-focus");
  DEFSYM! (Qz_group, c"z-group");
  DEFSYM! (Qoverride_redirect, c"override-redirect");
  DEFSYM! (Qdelete_before, c"delete-before");
  DEFSYM! (Qmodeline, c"modeline");
  DEFSYM! (Qonly, c"only");
  DEFSYM! (Qnone, c"none");
  DEFSYM! (Qwidth, c"width");
  DEFSYM! (Qtext_pixels, c"text-pixels");
  DEFSYM! (Qgeometry, c"geometry");
  DEFSYM! (Qicon_left, c"icon-left");
  DEFSYM! (Qicon_top, c"icon-top");
  DEFSYM! (Qtooltip, c"tooltip");
  DEFSYM! (Quser_position, c"user-position");
  DEFSYM! (Quser_size, c"user-size");
  DEFSYM! (Qwindow_id, c"window-id");
  /* C 7251-7253's `#ifdef HAVE_X_WINDOWS` arm is dead (P-08).  */
  DEFSYM! (Qparent_id, c"parent-id");
  DEFSYM! (Qx, c"x");
  DEFSYM! (Qw32, c"w32");
  DEFSYM! (Qpc, c"pc");
  DEFSYM! (Qns, c"ns");
  DEFSYM! (Qpgtk, c"pgtk");
  DEFSYM! (Qhaiku, c"haiku");
  DEFSYM! (Qandroid, c"android");
  DEFSYM! (Qvisible, c"visible");
  DEFSYM! (Qbuffer_predicate, c"buffer-predicate");
  DEFSYM! (Qbuffer_list, c"buffer-list");
  DEFSYM! (Qburied_buffer_list, c"buried-buffer-list");
  DEFSYM! (Qdisplay_type, c"display-type");
  DEFSYM! (Qbackground_mode, c"background-mode");
  DEFSYM! (Qnoelisp, c"noelisp");
  DEFSYM! (Qtty_color_mode, c"tty-color-mode");
  DEFSYM! (Qtty, c"tty");
  DEFSYM! (Qtty_type, c"tty-type");

  DEFSYM! (Qface_set_after_frame_default, c"face-set-after-frame-default");

  DEFSYM! (Qfullwidth, c"fullwidth");
  DEFSYM! (Qfullheight, c"fullheight");
  DEFSYM! (Qfullboth, c"fullboth");
  DEFSYM! (Qmaximized, c"maximized");
  DEFSYM! (Qshaded, c"shaded");
  DEFSYM! (Qx_resource_name, c"x-resource-name");
  DEFSYM! (Qx_frame_parameter, c"x-frame-parameter");

  DEFSYM! (Qworkarea, c"workarea");
  DEFSYM! (Qmm_size, c"mm-size");
  /* C 7285-7287's `#ifdef HAVE_PGTK` arm is dead (P-08).  */
  DEFSYM! (Qframes, c"frames");
  DEFSYM! (Qsource, c"source");

  DEFSYM! (Qframe_edges, c"frame-edges");
  DEFSYM! (Qouter_edges, c"outer-edges");
  DEFSYM! (Qouter_position, c"outer-position");
  DEFSYM! (Qouter_size, c"outer-size");
  DEFSYM! (Qnative_edges, c"native-edges");
  DEFSYM! (Qinner_edges, c"inner-edges");
  DEFSYM! (Qexternal_border_size, c"external-border-size");
  DEFSYM! (Qtitle_bar_size, c"title-bar-size");
  DEFSYM! (Qmenu_bar_external, c"menu-bar-external");
  DEFSYM! (Qmenu_bar_size, c"menu-bar-size");
  DEFSYM! (Qtab_bar_size, c"tab-bar-size");
  DEFSYM! (Qtool_bar_external, c"tool-bar-external");
  DEFSYM! (Qtool_bar_size, c"tool-bar-size");
  /* The following are passed to adjust_frame_size.  */
  DEFSYM! (Qx_set_menu_bar_lines, c"x_set_menu_bar_lines");
  DEFSYM! (Qchange_frame_size, c"change_frame_size");
  DEFSYM! (Qxg_frame_set_char_size, c"xg_frame_set_char_size");
  DEFSYM! (Qx_set_window_size_1, c"x_set_window_size_1");
  DEFSYM! (Qset_window_configuration, c"set_window_configuration");
  DEFSYM! (Qx_create_frame_1, c"x_create_frame_1");
  DEFSYM! (Qx_create_frame_2, c"x_create_frame_2");
  DEFSYM! (Qgui_figure_window_size, c"gui_figure_window_size");
  DEFSYM! (Qtip_frame, c"tip_frame");
  DEFSYM! (Qterminal_frame, c"terminal_frame");

  /* C 7316-7322's `#ifdef HAVE_NS`/`NS_IMPL_COCOA` arms are dead (P-08).  */

  DEFSYM! (Qalpha, c"alpha");
  DEFSYM! (Qalpha_background, c"alpha-background");
  DEFSYM! (Qborders_respect_alpha_background, c"borders-respect-alpha-background");
  DEFSYM! (Qauto_lower, c"auto-lower");
  DEFSYM! (Qauto_raise, c"auto-raise");
  DEFSYM! (Qborder_color, c"border-color");
  DEFSYM! (Qborder_width, c"border-width");
  DEFSYM! (Qouter_border_width, c"outer-border-width");
  DEFSYM! (Qbottom_divider_width, c"bottom-divider-width");
  DEFSYM! (Qcursor_color, c"cursor-color");
  DEFSYM! (Qcursor_type, c"cursor-type");
  DEFSYM! (Qfont_backend, c"font-backend");
  DEFSYM! (Qfullscreen, c"fullscreen");
  DEFSYM! (Qhorizontal_scroll_bars, c"horizontal-scroll-bars");
  DEFSYM! (Qicon_name, c"icon-name");
  DEFSYM! (Qicon_type, c"icon-type");
  DEFSYM! (Qchild_frame_border_width, c"child-frame-border-width");
  DEFSYM! (Qinternal_border_width, c"internal-border-width");
  DEFSYM! (Qleft_fringe, c"left-fringe");
  DEFSYM! (Qleft_fringe_help, c"left-fringe-help");
  DEFSYM! (Qline_spacing, c"line-spacing");
  DEFSYM! (Qmenu_bar_lines, c"menu-bar-lines");
  DEFSYM! (Qtab_bar_lines, c"tab-bar-lines");
  DEFSYM! (Qmouse_color, c"mouse-color");
  DEFSYM! (Qname, c"name");
  DEFSYM! (Qright_divider_width, c"right-divider-width");
  DEFSYM! (Qright_fringe, c"right-fringe");
  DEFSYM! (Qright_fringe_help, c"right-fringe-help");
  DEFSYM! (Qscreen_gamma, c"screen-gamma");
  DEFSYM! (Qscroll_bar_background, c"scroll-bar-background");
  DEFSYM! (Qscroll_bar_foreground, c"scroll-bar-foreground");
  DEFSYM! (Qscroll_bar_height, c"scroll-bar-height");
  DEFSYM! (Qscroll_bar_width, c"scroll-bar-width");
  DEFSYM! (Qsticky, c"sticky");
  DEFSYM! (Qtitle, c"title");
  DEFSYM! (Qtool_bar_lines, c"tool-bar-lines");
  DEFSYM! (Qtool_bar_position, c"tool-bar-position");
  DEFSYM! (Qunsplittable, c"unsplittable");
  DEFSYM! (Qvertical_scroll_bars, c"vertical-scroll-bars");
  DEFSYM! (Qvisibility, c"visibility");
  DEFSYM! (Qwait_for_wm, c"wait-for-wm");
  DEFSYM! (Qinhibit_double_buffering, c"inhibit-double-buffering");
  DEFSYM! (Qno_other_frame, c"no-other-frame");
  DEFSYM! (Qbelow, c"below");
  DEFSYM! (Qabove_suspended, c"above-suspended");
  DEFSYM! (Qmin_width, c"min-width");
  DEFSYM! (Qmin_height, c"min-height");
  DEFSYM! (Qmouse_wheel_frame, c"mouse-wheel-frame");
  DEFSYM! (Qkeep_ratio, c"keep-ratio");
  DEFSYM! (Qsize_and_position, c"size-and-position");
  DEFSYM! (Qwidth_only, c"width-only");
  DEFSYM! (Qheight_only, c"height-only");
  DEFSYM! (Qleft_only, c"left-only");
  DEFSYM! (Qtop_only, c"top-only");
  DEFSYM! (Qiconify_top_level, c"iconify-top-level");
  DEFSYM! (Qmake_invisible, c"make-invisible");
  DEFSYM! (Quse_frame_synchronization, c"use-frame-synchronization");
  DEFSYM! (Qfont_parameter, c"font-parameter");
  DEFSYM! (Qforce, c"force");
  DEFSYM! (Qinhibit, c"inhibit");
  DEFSYM! (Qframe_id, c"frame-id");
  DEFSYM! (Qcloned_from, c"cloned-from");
  DEFSYM! (Qundeleted, c"undeleted");

  {
    let mut i: c_int = 0;
    while i < countof! (frame_parms) as c_int
    {
      let sym: c_int = frame_parms[i as usize].sym;
      eassert! (0 <= sym && sym < countof! (lispsym) as c_int);
      let v: Lisp_Object = builtin_lisp_symbol (sym as usize);
      Fput (v, Qx_frame_parameter (), make_fixnum (i as EMACS_INT));
      i += 1;
    }
  }

  /* C 7396-7426's `#ifdef HAVE_WINDOW_SYSTEM` block is dead (P-08).  */

  DEFVAR_LISP! (c"default-frame-alist", Vdefault_frame_alist,
    "Alist of default values of frame parameters for frame creation.\nThese may be set in your init file, like this:\n  (setq default-frame-alist \\='((width . 80) (height . 55) (menu-bar-lines . 1)))\n\nThese override values given in window system configuration data,\nincluding X Windows' defaults database.\n\nNote that many display-related modes (like `scroll-bar-mode' or\n`menu-bar-mode') alter `default-frame-alist', so if you set this\nvariable directly, you may be overriding other settings\nunintentionally.  Instead it's often better to use\n`modify-all-frames-parameters' or push new elements to the front of\nthis alist.\n\nFor values specific to the first Emacs frame, see `initial-frame-alist'.\n\nFor window-system specific values, see `window-system-default-frame-alist'.\n\nFor values specific to the separate minibuffer frame, see\n`minibuffer-frame-alist'.\n\nSetting this variable does not affect existing frames, only new ones.  ");
  Vdefault_frame_alist = Qnil ();

  DEFVAR_LISP! (c"default-frame-scroll-bars", Vdefault_frame_scroll_bars,
	       "Default position of vertical scroll bars on this window-system.  ");
  /* C 7455-7465's `#if defined HAVE_WINDOW_SYSTEM && !defined HAVE_ANDROID` is dead, so
     C's live text is the `#else` arm's `Vdefault_frame_scroll_bars = Qnil;`.  */
  Vdefault_frame_scroll_bars = Qnil ();

  DEFVAR_BOOL! (c"scroll-bar-adjust-thumb-portion",
               scroll_bar_adjust_thumb_portion_p,
               "Adjust scroll bars for overscrolling for Gtk+, Motif and Haiku.\nNon-nil means adjust the thumb in the scroll bar so it can be dragged downwards\neven if the end of the buffer is shown (i.e. overscrolling).\nSet to nil if you want the thumb to be at the bottom when the end of the buffer\nis shown.  Also, the thumb fills the whole scroll bar when the entire buffer\nis visible.  In this case you can not overscroll.  ");
  scroll_bar_adjust_thumb_portion_p = true;

  DEFVAR_LISP! (c"terminal-frame", Vterminal_frame,
               "The initial frame-object, which represents Emacs's stdout.  ");

  DEFVAR_LISP! (c"mouse-position-function", Vmouse_position_function,
	       "If non-nil, function to transform normal value of `mouse-position'.\n`mouse-position' and `mouse-pixel-position' call this function, passing their\nusual return value as argument, and return whatever this function returns.\nThis abnormal hook exists for the benefit of packages like `xt-mouse.el'\nwhich need to do mouse handling at the Lisp level.  ");
  Vmouse_position_function = Qnil ();

  DEFVAR_LISP! (c"mouse-highlight", Vmouse_highlight,
	       "If non-nil, clickable text is highlighted when mouse is over it.\nIf the value is an integer, highlighting is shown only after moving the\nmouse, while keyboard input turns off the highlight even when the mouse\nis over the clickable text.  However, the mouse shape still indicates\nwhen the mouse is over clickable text.  ");
  Vmouse_highlight = Qt ();

  DEFVAR_LISP! (c"make-pointer-invisible", Vmake_pointer_invisible,
               "If non-nil, make mouse pointer invisible while typing.\nThe pointer becomes visible again when the mouse is moved.\n\nWhen using this, you might also want to disable highlighting of\nclickable text.  See `mouse-highlight'.  ");
  Vmake_pointer_invisible = Qt ();

  DEFVAR_LISP! (c"move-frame-functions", Vmove_frame_functions,
               "Functions run after a frame was moved.\nThe functions are run with one arg, the frame that moved.  ");
  Vmove_frame_functions = Qnil ();

  DEFVAR_LISP! (c"delete-frame-functions", Vdelete_frame_functions,
	       "Functions run before deleting a frame.\nThe functions are run with one arg, the frame to be deleted.\nSee `delete-frame'.\n\nNote that functions in this list may be called just before the frame is\nactually deleted, or some time later (or even both when an earlier function\nin `delete-frame-functions' (indirectly) calls `delete-frame'\nrecursively).  ");
  Vdelete_frame_functions = Qnil ();
  DEFSYM! (Qdelete_frame_functions, c"delete-frame-functions");

  DEFVAR_LISP! (c"after-delete-frame-functions",
               Vafter_delete_frame_functions,
               "Functions run after deleting a frame.\nThe functions are run with one arg, the frame that was deleted and\nwhich is now dead.  ");
  Vafter_delete_frame_functions = Qnil ();
  DEFSYM! (Qafter_delete_frame_functions, c"after-delete-frame-functions");

  DEFVAR_LISP! (c"menu-bar-mode", Vmenu_bar_mode,
               "Non-nil if Menu-Bar mode is enabled.\nSee the command `menu-bar-mode' for a description of this minor mode.\nSetting this variable directly does not take effect;\neither customize it (see the info node `Easy Customization')\nor call the function `menu-bar-mode'.  ");
  Vmenu_bar_mode = Qt ();

  DEFVAR_LISP! (c"tab-bar-mode", Vtab_bar_mode,
               "Non-nil if Tab-Bar mode is enabled.\nSee the command `tab-bar-mode' for a description of this minor mode.\nSetting this variable directly does not take effect;\neither customize it (see the info node `Easy Customization')\nor call the function `tab-bar-mode'.  ");
  Vtab_bar_mode = Qnil ();

  DEFVAR_LISP! (c"tool-bar-mode", Vtool_bar_mode,
               "Non-nil if Tool-Bar mode is enabled.\nSee the command `tool-bar-mode' for a description of this minor mode.\nSetting this variable directly does not take effect;\neither customize it (see the info node `Easy Customization')\nor call the function `tool-bar-mode'.  ");
  /* C 7551-7555's `#ifdef HAVE_WINDOW_SYSTEM`/`#else` takes the `#else` arm (P-08).  */
  Vtool_bar_mode = Qnil ();

  DEFVAR_KBOARD! (c"default-minibuffer-frame",
		  std::mem::offset_of! (kboard, Vdefault_minibuffer_frame_),
		 "Minibuffer-less frames by default use this frame's minibuffer.\nEmacs consults this variable only when creating a minibuffer-less frame\nand no explicit minibuffer window has been specified for that frame via\nthe `minibuffer' frame parameter.  Once such a frame has been created,\nsetting this variable does not change that frame's previous association.\n\nThis variable is local to the current terminal and cannot be buffer-local.  ");

  DEFVAR_LISP! (c"resize-mini-frames", resize_mini_frames,
    "Non-nil means resize minibuffer-only frames automatically.\nIf this is nil, do not resize minibuffer-only frames automatically.\n\nIf this is a function, call that function with the minibuffer-only\nframe that shall be resized as sole argument.  The buffer of the root\nwindow of that frame is the buffer whose text will be eventually shown\nin the minibuffer window.\n\nAny other non-nil value means to resize minibuffer-only frames by\ncalling `fit-mini-frame-to-buffer'.  ");
  resize_mini_frames = Qnil ();

  DEFVAR_LISP! (c"focus-follows-mouse", focus_follows_mouse,
	       "Non-nil if window system changes focus when you move the mouse.\nYou should set this variable to tell Emacs how your window manager\nhandles focus, since there is no way in general for Emacs to find out\nautomatically.\n\nThere are three meaningful values:\n\n- The default nil should be used when your window manager follows a\n  \"click-to-focus\" policy where you have to click the mouse inside of a\n  frame in order for that frame to get focus.\n\n- The value t should be used when your window manager has the focus\n  automatically follow the position of the mouse pointer but a window\n  that gains focus is not raised automatically.\n\n- The value `auto-raise' should be used when your window manager has the\n  focus automatically follow the position of the mouse pointer and a\n  window that gains focus is raised automatically.\n\nIf this option is non-nil, Emacs moves the mouse pointer to the frame\nselected by `select-frame-set-input-focus'.  This function is used by a\nnumber of commands like, for example, `other-frame' and `pop-to-buffer'.\nIf this option is nil and your focus follows mouse window manager does\nnot autonomously move the mouse pointer to the newly selected frame, the\npreviously selected window manager window might get reselected instead\nimmediately.\n\nThe distinction between the values t and `auto-raise' is not needed for\n\"normal\" frames because the window manager takes care of raising them.\nSetting this to `auto-raise' will, however, override the standard\nbehavior of a window manager that does not automatically raise the frame\nthat gets focus.  Setting this to `auto-raise' is also necessary to\nautomatically raise child frames which are usually left alone by the\nwindow manager.\n\nNote that this option does not distinguish \"sloppy\" focus (where the\nframe that previously had focus retains focus as long as the mouse\npointer does not move into another window manager window) from \"strict\"\nfocus (where a frame immediately loses focus when it's left by the mouse\npointer).\n\nIn order to extend a \"focus follows mouse\" policy to individual Emacs\nwindows, customize the variable `mouse-autoselect-window'.  ");
  focus_follows_mouse = Qnil ();

  DEFVAR_BOOL! (c"frame-resize-pixelwise", frame_resize_pixelwise,
	       "Non-nil means resize frames pixelwise.\nIf this option is nil, resizing a frame rounds its sizes to the frame's\ncurrent values of `frame-char-height' and `frame-char-width'.  If this\nis non-nil, no rounding occurs, hence frame sizes can increase/decrease\nby one pixel.\n\nWith some window managers you may have to set this to non-nil in order\nto set the size of a frame in pixels, to maximize frames or to make them\nfullscreen.  To resize your initial frame pixelwise, set this option to\na non-nil value in your init file.  ");
  frame_resize_pixelwise = false;

  DEFVAR_LISP! (c"frame-inhibit-implied-resize", frame_inhibit_implied_resize,
	       "Whether frames should be resized implicitly.\nIf this option is nil, setting font, menu bar, tool bar, tab bar,\ninternal borders, fringes or scroll bars of a specific frame may resize\nthe frame in order to preserve the number of columns or lines it\ndisplays.\n\nIf this option is t, no such resizing happens once Emacs has agreed with\nthe window manager on the final initial size of a frame.  That size will\nhave taken into account the size of the text area requested by the user\nand the size of all decorations initially present on the frame.\n\nIf this is the symbol `force', no implicit resizing happens even before\na frame has obtained its final initial size.  As a consequence, the\ninitial frame size may not necessarily be the one requested by the user.\nThis value can be useful with tiling window managers where the initial\nsize of a frame is determined by external means.\n\nThe value of this option can be also a list of frame parameters.  In\nthis case, resizing is inhibited once a frame has obtained its final\ninitial size when changing a parameter that appears in that list.  The\nparameters currently handled by this option include `font',\n`font-backend', `internal-border-width', `menu-bar-lines',\n`tool-bar-lines' and `tab-bar-lines'.\n\nChanging any of the parameters `scroll-bar-width', `scroll-bar-height',\n`vertical-scroll-bars', `horizontal-scroll-bars', `left-fringe' and\n`right-fringe' is handled as if the frame contained just one live\nwindow.  This means, for example, that removing vertical scroll bars on\na frame containing several side by side windows will shrink the frame\nwidth by the width of one scroll bar provided this option is nil and\nkeep it unchanged if this option is either t or a list containing\n`vertical-scroll-bars'.\n\nIn GTK+ and NS that use the external tool bar, the default value is\n\\='(tab-bar-lines) which means that adding/removing a tab bar does\nnot change the frame height.  On all other types of GUI frames, the\ndefault value is \\='(tab-bar-lines tool-bar-lines) which means that\nadding/removing a tool bar or tab bar does not change the frame\nheight.  Otherwise it's t which means the frame size never changes\nimplicitly when there's no window system support.\n\nNote that the size of fullscreen and maximized frames, the height of\nfullheight frames and the width of fullwidth frames never change\nimplicitly.  Note also that when a frame is not large enough to\naccommodate a change of any of the parameters listed above, Emacs may\ntry to enlarge the frame even if this option is non-nil.  ");
  /* C 7685-7693's `#if defined (HAVE_WINDOW_SYSTEM) && !defined (HAVE_ANDROID)` is dead,
     so C's live text is the outer `#else` arm's `frame_inhibit_implied_resize = Qt;`.  */
  frame_inhibit_implied_resize = Qt ();

  DEFVAR_LISP! (c"frame-size-history", frame_size_history,
               "History of frame size adjustments.\nIf non-nil, list recording frame size adjustment.  Adjustments are\nrecorded only if the first element of this list is a positive number.\nAdding an adjustment decrements that number by one.\n\nThe remaining elements are the adjustments.  Each adjustment is a list\nof four elements `frame', `function', `sizes' and `more'.  `frame' is\nthe affected frame and `function' the invoking function.  `sizes' is\nusually a list of four elements `old-width', `old-height', `new-width'\nand `new-height' representing the old and new sizes recorded/requested\nby `function'.  `more' is a list with additional information.\n\nThe function `frame--size-history' displays the value of this variable\nin a more readable form.  ");
    frame_size_history = Qnil ();

  DEFVAR_BOOL! (c"tooltip-reuse-hidden-frame", tooltip_reuse_hidden_frame,
	       "Non-nil means reuse hidden tooltip frames.\nWhen this is nil, delete a tooltip frame when hiding the associated\ntooltip.  When this is non-nil, make the tooltip frame invisible only,\nso it can be reused when the next tooltip is shown.\n\nSetting this to non-nil may drastically reduce the consing overhead\nincurred by creating new tooltip frames.  However, a value of non-nil\nmeans also that intermittent changes of faces or `default-frame-alist'\nare not applied when showing a tooltip in a reused frame.\n\nThis variable is effective only with the X toolkit (and there only when\nGtk+ tooltips are not used) and on Windows.  ");
  tooltip_reuse_hidden_frame = false;

  DEFVAR_BOOL! (c"use-system-tooltips", use_system_tooltips,
	       "Whether to use the toolkit to display tooltips.\nThis option is only meaningful when Emacs is built with GTK+, NS or Haiku\nwindowing support, and, if it's non-nil (the default), it results in\ntooltips that look like those displayed by other GTK+/NS/Haiku programs,\nbut will not be able to display text properties inside tooltip text.  ");
  use_system_tooltips = true;

  DEFVAR_LISP! (c"iconify-child-frame", iconify_child_frame,
	       "How to handle iconification of child frames.\nThis variable tells Emacs how to proceed when it is asked to iconify a\nchild frame.  If it is nil, `iconify-frame' will do nothing when invoked\non a child frame.  If it is `iconify-top-level' and the child frame is\non a graphical terminal, Emacs will try to iconify the root frame of\nthis child frame.  If it is `make-invisible', Emacs will try to make\nthis child frame invisible instead.\n\nAny other value means to try iconifying the child frame on a graphical\nterminal.  Since such an attempt is not honored by all window managers\nand may even lead to making the child frame unresponsive to user\nactions, the default is to iconify the root frame instead.  ");
  iconify_child_frame = Qiconify_top_level ();

  DEFVAR_LISP! (c"expose-hidden-buffer", expose_hidden_buffer,
	       "Non-nil means to make a hidden buffer more visible.\nA buffer is considered \"hidden\" if its name starts with a space.  By\ndefault, many functions disregard hidden buffers.  In particular,\n`make-frame' does not show the current buffer in the new frame's\nselected window if that buffer is hidden.  Rather, `make-frame' will\nshow a buffer that is not hidden instead.\n\nIf this variable is non-nil, it will override the default behavior and\nallow `make-frame' to show the current buffer even if its hidden.  ");
  expose_hidden_buffer = Qnil ();
  DEFSYM! (Qexpose_hidden_buffer, c"expose-hidden-buffer");
  Fmake_variable_buffer_local (Qexpose_hidden_buffer ());

  DEFVAR_LISP! (c"frame-internal-parameters", frame_internal_parameters,
	       "Frame parameters specific to every frame.  ");
  /* C 7766-7770's `#ifdef HAVE_X_WINDOWS`/`#else` takes the `#else` arm (P-08).  */
  frame_internal_parameters = list3 (Qname (), Qparent_id (), Qwindow_id ());
  frame_internal_parameters = Fcons (Qframe_id (), frame_internal_parameters);
  frame_internal_parameters = Fcons (Qcloned_from (), frame_internal_parameters);
  frame_internal_parameters = Fcons (Qundeleted (), frame_internal_parameters);

  DEFVAR_LISP! (c"alter-fullscreen-frames", alter_fullscreen_frames,
	       "How to handle requests to resize fullscreen frames.\nEmacs consults this option when asked to resize a fullscreen frame via\nfunctions like `set-frame-size' or when setting the \\+`width' or \\+`height'\nparameter of a frame.  The following values are provided:\n\n- nil means to forward the resize request to the window manager and\n  leave it to the latter how to proceed.\n\n- t means to first reset the fullscreen status and then forward the\n  request to the window manager.\n\n- \\+`inhibit' means to reject the resize request and leave the fullscreen\n  status unchanged.\n\nThe default is \\+`inhibit' in NS builds and nil everywhere else.  ");

  /* C 7792-7796's `#if defined (NS_IMPL_COCOA)`/`#else` takes the `#else` arm (P-08).  */
  alter_fullscreen_frames = Qnil ();

  DEFVAR_LISP! (c"delete-frame-choose-selected",
	       delete_frame_choose_selected,
	       "What frame to select after frame deletion.\nThe value `mru' means `delete-frame' selects most recently used frame.\nIf this is nil, `delete-frame' will select the oldest visible frame on\nthe same terminal.  ");
  delete_frame_choose_selected = Qmru ();

  defsubr (&raw mut Sframe_id);
  defsubr (&raw mut Sframep);
  defsubr (&raw mut Sframe_live_p);
  defsubr (&raw mut Swindow_system);
  defsubr (&raw mut Sframe_windows_min_size);
  defsubr (&raw mut Smake_terminal_frame);
  defsubr (&raw mut Sselect_frame);
  defsubr (&raw mut Shandle_switch_frame);
  defsubr (&raw mut Sselected_frame);
  defsubr (&raw mut Sold_selected_frame);
  defsubr (&raw mut Sframe_list);
  defsubr (&raw mut Sframe_parent);
  defsubr (&raw mut Sframe_ancestor_p);
  defsubr (&raw mut Sframe_root_frame);
  defsubr (&raw mut Snext_frame);
  defsubr (&raw mut Sprevious_frame);
  defsubr (&raw mut Slast_nonminibuf_frame);
  defsubr (&raw mut Sdelete_frame);
  defsubr (&raw mut Smouse_position);
  defsubr (&raw mut Smouse_pixel_position);
  defsubr (&raw mut Sset_mouse_position);
  defsubr (&raw mut Sset_mouse_pixel_position);
  /* C 7828-7831's `#if 0` (a literal-false condition) arm is dead.  */
  defsubr (&raw mut Smake_frame_visible);
  defsubr (&raw mut Smake_frame_invisible);
  defsubr (&raw mut Siconify_frame);
  defsubr (&raw mut Sframe_visible_p);
  defsubr (&raw mut Svisible_frame_list);
  defsubr (&raw mut Sraise_frame);
  defsubr (&raw mut Slower_frame);
  defsubr (&raw mut Sx_focus_frame);
  defsubr (&raw mut Sframe_after_make_frame);
  defsubr (&raw mut Sredirect_frame_focus);
  defsubr (&raw mut Sframe_focus);
  defsubr (&raw mut Sframe_parameters);
  defsubr (&raw mut Sframe_parameter);
  defsubr (&raw mut Smodify_frame_parameters);
  defsubr (&raw mut Sframe_char_height);
  defsubr (&raw mut Sframe_char_width);
  defsubr (&raw mut Sframe_native_height);
  defsubr (&raw mut Sframe_native_width);
  defsubr (&raw mut Sframe_text_cols);
  defsubr (&raw mut Sframe_text_lines);
  defsubr (&raw mut Sframe_total_cols);
  defsubr (&raw mut Sframe_total_lines);
  defsubr (&raw mut Sframe_text_width);
  defsubr (&raw mut Sframe_text_height);
  defsubr (&raw mut Sscroll_bar_width);
  defsubr (&raw mut Sscroll_bar_height);
  defsubr (&raw mut Sfringe_width);
  defsubr (&raw mut Sframe_child_frame_border_width);
  defsubr (&raw mut Sframe_internal_border_width);
  defsubr (&raw mut Sright_divider_width);
  defsubr (&raw mut Sbottom_divider_width);
  defsubr (&raw mut Stool_bar_pixel_width);
  defsubr (&raw mut Sset_frame_height);
  defsubr (&raw mut Sset_frame_width);
  defsubr (&raw mut Sset_frame_size);
  defsubr (&raw mut Sframe_position);
  defsubr (&raw mut Sset_frame_position);
  defsubr (&raw mut Sset_frame_size_and_position_pixelwise);
  defsubr (&raw mut Sframe_pointer_visible_p);
  defsubr (&raw mut Smouse_position_in_root_frame);
  defsubr (&raw mut Sframe__set_was_invisible);
  defsubr (&raw mut Sframe_window_state_change);
  defsubr (&raw mut Sset_frame_window_state_change);
  defsubr (&raw mut Sframe_scale_factor);

  /* C 7877-7881's `#ifdef HAVE_WINDOW_SYSTEM` block is dead (P-08).  */

  /* C 7883-7894's `#ifdef HAVE_WINDOW_SYSTEM` block is dead (P-08).  */

  DEFVAR_LISP! (c"toolkit-theme", Vtoolkit_theme,
               "The current toolkit theme.\nEither the symbol `light' or the symbol `dark', reflecting the system's\ncurrent theme preference.  This variable is updated automatically when\nthe system theme changes.\n\nThis variable is only set on PGTK, Android, and MS-Windows builds.  ");
  Vtoolkit_theme = Qnil ();
  DEFSYM! (Qlight, c"light");
  DEFSYM! (Qdark, c"dark");
  DEFSYM! (Qtoolkit_theme_set_functions, c"toolkit-theme-set-functions");
}
