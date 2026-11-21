// femacs: port of src/indent.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// macro, struct/union field, file-scope object, local), same control flow, same error
// model.  C's 11 non-`static` definitions (`buffer_display_table`,
// `disptab_matches_widthtab`, `recompute_width_table`, `skip_invisible`,
// `invalidate_current_column`, `current_column`, `indented_beyond_p`, `compute_motion`,
// `vmotion`, `line_number_display_width`, `syms_of_indent`) carry `#[unsafe(no_mangle)]`
// (PORTING.md 0.2); the C `static` helpers (`character_width`, `width_run_cache_on_off`,
// `check_display_width`, `scan_for_column`, `current_column_1`, `position_indentation`,
// `window_column_x`, `restore_window_buffer`) stay plain `unsafe fn`, and the four
// `static` objects (`last_known_column`, `last_known_column_modified`,
// `val_compute_motion`, `val_vmotion`) are unexported `static mut`s.
//
// **`indent.h`'s mirror lands completed with this unit** (`src/include/indent.rs`,
// the charset.h/intervals.h precedent): `struct position`, its five prototypes and the one
// `extern` object.  This unit defines all five, so they moved into an `indent_decls` module
// + glob import (PORTING.md 0.1-10) and the object into `indent_objs` (P-21).
//
// **`src/indent.c` is in `docs/baseline/config/cond-eval-verified.txt`**
// (`python3 tools/check-cond-eval.py src/indent.c` reports
// `5 conditions, 0 unknown, 0 disagreements [VERIFIED]`), so the frozen configuration's
// branch selection is clang's and the dead arms are **recorded in docs/ports/indent.md,
// not transcribed** (P-08): the two literal `#if 0` blocks (C 252-260 inside
// `skip_invisible`, C 867-944 the unused `string_display_width`) and the three
// `#ifdef HAVE_WINDOW_SYSTEM` arms (C 1334-1336, 1341-1344 in `compute_motion` and
// 1947-1949 in `Fcompute_motion`; `HAVE_WINDOW_SYSTEM` is `undef` in the frozen
// `config.h`, macros.tsv 1647).  That is why `docs/ports/line-drift-tolerance.tsv`
// declares the `live` basis for this file.
//
// The names this unit takes from headers whose mirrors do not carry them yet are declared
// at their use site below, each with C's own prototype in a `/// C:` line (the
// terminal.rs/insdel.rs shape); `indent.h` is the one header that is mirrored here.
// `dispextern.h` is mirrored only to its "batch II" prefix, so the display iterator
// (`struct it`, `struct bidi_it`, `struct iterator_stack_entry`, the `it_method` /
// `line_wrap_method` / `move_operation_enum` enumerations, the `IT_*`/`MOVE_TO_*` macros
// and the ten `xdisp.c` entry points `indent.c` calls) is declared here as well, with
// every reached offset measured against the frozen target by clang and pinned by the
// `const _` assertions next to each record.  Those records and the enumerations are
// mirror gaps, listed in `docs/ports/indent.md` for the display batch.
//
// C's uninitialised locals are zero-initialised (PORTING.md 1.5) and C 53-54's and 469's
// forward declarations are dropped (PORTING.md 0.1-12); both are named in
// `docs/ports/indent.md`.  C 867-944's `string_display_width` is dead text, so no Rust
// counterpart is written for it.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unreachable_code, while_true,
         unused_parens, unused_mut, unused_labels, unused_comparisons,
         overflowing_literals)]
// [U1-MODULE-UNIT]
use crate::common::*;

const _: () = ();
// [U1] include/lisp.rs imported via common
// [U1] include/character.rs imported via common
// [U1] include/itree.rs imported via common
// [U1] include/buffer.rs imported via common
// [U1] include/category.rs imported via common
// [U1] include/dispextern.rs imported via common
// [U1] include/composite.rs imported via common
// [U1] include/indent.rs imported via common
// [U1] include/coding.rs imported via common
// [U1] include/systime.rs imported via common
// [U1] include/termhooks.rs imported via common
// [U1] include/window.rs imported via common
// [U1] include/frame.rs imported via common
// [U1] include/disptab.rs imported via common
// [U1] include/intervals.rs imported via common
// [U1] include/region-cache.rs imported via common
// [U1] ../lib/include/limits.in.rs imported via common
// [U1] ../lib/include/string.rs imported via common

/* C 33's `"region-cache.h"` is the mirror's now (`include/region-cache.rs`, status 3.74): its
   four prototypes come from there with C's own text, `struct region_cache *` spelled `*mut
   c_void` (0.1-2/P-72 rule 3), and the declarations that stood below went away with it.  */

/* The names C gets from headers whose mirrors do not carry them yet (the terminal.rs
   shape), each with C's own prototype in a `/// C:` line.  */
unsafe extern "C-unwind" {






  /// C: lisp.h 4825: `extern void record_unwind_protect_void (void (*) (void));`
  fn record_unwind_protect_void (function: Option<unsafe extern "C-unwind" fn ()>);
  // window.h 1210/1216 (`decode_live_window`, `window_internal_height`) are the window.rs
  // mirror's now; the use-site copies are gone (P-26).
  /// C: window.h 1223: `extern int window_body_width (struct window *w, enum
  /// window_body_unit);`
  fn window_body_width (w: *mut window, unit: window_body_unit) -> c_int;


  /// C: dispextern.h 3583: `int move_it_to (struct it *, ptrdiff_t, int, int, int, int);`
  fn move_it_to (it: *mut it, to_charpos: ptrdiff_t, to_x: c_int, to_y: c_int,
                 to_vpos: c_int, op: c_int) -> c_int;
  /// C: dispextern.h 3597: `void start_display (struct it *, struct window *, struct
  /// text_pos);`
  fn start_display (it: *mut it, w: *mut window, pos: text_pos);
  /// C: dispextern.h 3600: `void move_it_by_lines (struct it *, ptrdiff_t);`
  fn move_it_by_lines (it: *mut it, dvpos: ptrdiff_t);
  /// C: dispextern.h 3602-3604: `void move_it_in_display_line (struct it *it, ptrdiff_t
  /// to_charpos, int to_x, enum move_operation_enum op);`
  fn move_it_in_display_line (it: *mut it, to_charpos: ptrdiff_t, to_x: c_int,
                              op: move_operation_enum);
  /// C: dispextern.h 3610: `extern bool display_working_on_window_p;`
  static mut display_working_on_window_p: bool;

  /// C: dispextern.h 3619: `extern void reseat_at_previous_visible_line_start (struct
  /// it *);`
  fn reseat_at_previous_visible_line_start (it: *mut it);
}

/* C: lib/limits.in.h 115's `# define SHRT_WIDTH _GL_INTEGER_WIDTH (SHRT_MIN, SHRT_MAX)` is
   16 on the frozen target and now comes from the limits.in.rs mirror this unit includes
   (P-26's mirror gap closed; PORTING.md 0.1-3).  */

/* C: dispextern.h 2590: `#define OVERLAY_STRING_CHUNK_SIZE 16`.  dispextern.h is mirrored
   only to its batch-II prefix (P-66), so the constant is declared here.  */
pub const OVERLAY_STRING_CHUNK_SIZE: c_int = 16;

/* C: dispextern.h 2384: `#define IT_STACK_SIZE 5` (the header's comment: "FIXME: What is
   this?  Why 5?").  Same mirror gap as OVERLAY_STRING_CHUNK_SIZE.  */
pub const IT_STACK_SIZE: c_int = 5;

/* C: dispextern.h 2371-2384 `enum it_method` and 2338-2344 `enum line_wrap_method`:
   platform-free enumerations the mirror does not carry yet (P-66).  C's enum ABI is `int`
   (PORTING.md 1.3), so they are `c_int` with C's constants.  */
pub type it_method = c_int;
pub const GET_FROM_BUFFER: it_method = 0;
pub const GET_FROM_DISPLAY_VECTOR: it_method = 1;
pub const GET_FROM_STRING: it_method = 2;
pub const GET_FROM_C_STRING: it_method = 3;
pub const GET_FROM_IMAGE: it_method = 4;
pub const GET_FROM_STRETCH: it_method = 5;
pub const GET_FROM_XWIDGET: it_method = 6;
pub const NUM_IT_METHODS: it_method = 7;

pub type line_wrap_method = c_int;
pub const TRUNCATE: line_wrap_method = 0;
pub const WORD_WRAP: line_wrap_method = 1;
pub const WINDOW_WRAP: line_wrap_method = 2;

/* C: dispextern.h 2977-2988 `enum move_operation_enum`.  Same mirror gap; this unit
   passes only `MOVE_TO_X` and `MOVE_TO_POS`, and the other two members are carried so the
   enumeration is C's (docs/ports/indent.md records it).  */
pub type move_operation_enum = c_int;
pub const MOVE_TO_X: move_operation_enum = 0x01;
pub const MOVE_TO_Y: move_operation_enum = 0x02;
pub const MOVE_TO_VPOS: move_operation_enum = 0x04;
pub const MOVE_TO_POS: move_operation_enum = 0x08;

/* C: dispextern.h 2386-2424 `struct composition_it`, in C's field order.  dispextern.h is
   not mirrored this far (P-66); the record is declared here as composite.rs declares it,
   with clang's offsets for the frozen target pinned below (sizeof 88, align 8).  */
#[repr(C)]
pub struct composition_it
{
  /* Next position at which to check the composition.  */
  pub stop_pos: ptrdiff_t,              // 2388, offset 0
  /* ID number of the composition or glyph-string.  If negative, we
     are not iterating over a composition now.  */
  pub id: ptrdiff_t,                    // 2391, offset 8
  /* If non-negative, character that triggers the automatic
     composition at `stop_pos', and this is an automatic composition.
     If negative, this is a static composition.  This is set to -2
     temporarily if searching of composition reach a limit or a
     newline.  */
  pub ch: c_int,                        // 2397, offset 16
  /* If this is an automatic composition, index of a rule for making
     the automatic composition.  Provided that ELT is an element of
     Vcomposition_function_table for CH, (nth ELT RULE_IDX) is the
     rule for the composition.  */
  pub rule_idx: EMACS_INT,              // 2402, offset 24
  /* If this is an automatic composition, how many characters to look
     back from the position where a character triggering the
     composition exists.  */
  pub lookback: ptrdiff_t,              // 2406, offset 32
  /* If non-negative, number of glyphs of the glyph-string.  */
  pub nglyphs: c_int,                   // 2408, offset 40
  /* True iff the composition is created while buffer is scanned in
     reverse order, and thus the grapheme clusters must be rendered
     from the last to the first.  */
  pub reversed_p: bool,                 // 2412, offset 44
  /* Parent iterator. */
  pub parent_it: *mut it,               // 2414, offset 48
  /* Position of the first character of the current grapheme cluster.  */
  pub charpos: ptrdiff_t,               // 2418, offset 56
  /* Number of characters and bytes of the current grapheme cluster.  */
  pub nchars: c_int,                    // 2420, offset 64
  pub nbytes: c_int,                    // 2420, offset 68
  /* Indices of the glyphs for the current grapheme cluster.  */
  pub from: c_int,                      // 2422, offset 72
  pub to: c_int,                        // 2422, offset 76
  /* Width of the current grapheme cluster in units of columns it will
     occupy on display; see CHARACTER_WIDTH.  */
  pub width: c_int,                     // 2425, offset 80
}

const _: () = core::assert! (std::mem::size_of::<composition_it> () == 88);
const _: () = core::assert! (std::mem::align_of::<composition_it> () == 8);
const _: () = core::assert! (std::mem::offset_of! (composition_it, stop_pos) == 0);
const _: () = core::assert! (std::mem::offset_of! (composition_it, id) == 8);
const _: () = core::assert! (std::mem::offset_of! (composition_it, ch) == 16);
const _: () = core::assert! (std::mem::offset_of! (composition_it, rule_idx) == 24);
const _: () = core::assert! (std::mem::offset_of! (composition_it, lookback) == 32);
const _: () = core::assert! (std::mem::offset_of! (composition_it, nglyphs) == 40);
const _: () = core::assert! (std::mem::offset_of! (composition_it, reversed_p) == 44);
const _: () = core::assert! (std::mem::offset_of! (composition_it, parent_it) == 48);
const _: () = core::assert! (std::mem::offset_of! (composition_it, charpos) == 56);
const _: () = core::assert! (std::mem::offset_of! (composition_it, nchars) == 64);
const _: () = core::assert! (std::mem::offset_of! (composition_it, nbytes) == 68);
const _: () = core::assert! (std::mem::offset_of! (composition_it, from) == 72);
const _: () = core::assert! (std::mem::offset_of! (composition_it, to) == 76);
const _: () = core::assert! (std::mem::offset_of! (composition_it, width) == 80);

/* C: dispextern.h 2159-2222 `struct bidi_it`: this unit reaches exactly one member of the
   bidi iterator, `scan_dir` (C's `it.bidi_it.scan_dir` in `Fvertical_motion`).  clang
   against the frozen target gives sizeof 2328 with `scan_dir` at 184 and the three
   `bool_bf` bits in the last byte (2327); the pad covers the members C reaches before and
   after it, and the two offsets + size leave no room for a misplaced field.  */
#[repr(C)]
pub struct bidi_it
{
  /// C 2160-2185: `bytepos` … `sos`, the members before `scan_dir` (offset 0 … 184).
  pub _pad: [u8; 184],
  pub scan_dir: c_int,                  // 2186, offset 184
  /// C 2188-2221: `disp_pos` … the three `bool_bf : 1` members (offset 188 … 2328).
  pub _pad_tail: [u8; 2140],
}

const _: () = core::assert! (std::mem::size_of::<bidi_it> () == 2328);
const _: () = core::assert! (std::mem::offset_of! (bidi_it, scan_dir) == 184);

/* C: dispextern.h 2594-2665 `struct iterator_stack_entry`: this unit reaches exactly one
   member, `method` (C's `it.stack[it.sp - 1].method` in `Fvertical_motion`).  clang gives
   sizeof 304 with `method` at 268 and the seven `bool_bf : 1` bits in the byte at 276.  */
#[repr(C)]
pub struct iterator_stack_entry
{
  /// C 2596-2650: `string` … the `u` union (the byte at 276 is where its bits are).
  pub _pad: [u8; 268],
  pub method: it_method,                // 2652, offset 268
  /// C 2653-2664: `paragraph_embedding` … `font_height` (offset 272 … 304).
  pub _pad_tail: [u8; 32],
}

const _: () = core::assert! (std::mem::size_of::<iterator_stack_entry> () == 304);
const _: () = core::assert! (std::mem::offset_of! (iterator_stack_entry, method) == 268);

/* C: dispextern.h 2430-2934 `struct it`, the display iterator.  dispextern.h is mirrored
   only to its batch-II prefix (P-66), so this record is declared the way composite.rs
   declares its three-member prefix: the members this unit reads or writes are named, each
   gap is the measured pad, and the whole layout is pinned by the `const _` assertions
   below.  clang against the frozen target: sizeof 4968, align 8, and every named offset
   below.  The two `bool_bf` runs are one byte at 112 and two bytes at 2216; this unit
   reaches only `string_from_display_prop_p` (bit 3 of the byte at 112) and reads it through
   `it_bits!`, whose mask and member name are in `docs/ports/token-aliases.tsv`'s family.
   `stack` is `[iterator_stack_entry; IT_STACK_SIZE]` (the entry type above pins `method`'s
   offset) and `cmp_it` is the `composition_it` above, so the pads between them are exact.
   Every member C does not reach is inside a pad; the `///` line above each pad lists what
   it covers, so the display batch can extend this record from the same numbers.  */
#[repr(C)]
pub struct it
{
  /// The window in which we iterate over current_buffer (or a string).  (C 2434)
  pub window: Lisp_Object,              // offset 0
  pub w: *mut window,                   // 2435, offset 8
  /// The window's frame.  (C 2438)
  pub f: *mut frame,                    // offset 16
  /// Method to use to load this structure with the next display element.  (C 2441)
  pub method: it_method,                // offset 24
  /// C 2444-2457: `stop_charpos`, `prev_stop`, `base_level_stop`, `end_charpos`,
  /// `medium_narrowing_begv`, `medium_narrowing_zv`, `large_narrowing_begv`,
  /// `large_narrowing_zv`, `s`, `string_nchars` (offset 28 … 112).
  pub _pad_a: [u8; 84],
  /// C 2459-2471: the eight `bool_bf : 1` members `multibyte_p`, `tab_line_p`,
  /// `header_line_p`, `string_from_display_prop_p`, `string_from_prefix_prop_p`,
  /// `from_disp_prop_p`, `ellipsis_p`, `avoid_cursor_p` — one byte at 112.
  pub bits: u8,
  /// C 2473-2506: `dp`, `dpvec`, `dpend`, `dpvec_char_len`, `dpvec_face_id`,
  /// `saved_face_id`, `ctl_chars[16]`, `start` (offset 113 … 336).
  pub _pad_b: [u8; 223],
  /// Current buffer or string position of the iterator, including
  /// position in overlay strings etc.  (C 2512)
  pub current: display_pos,             // offset 336
  /// C 2516-2522: `n_overlay_strings`, `overlay_strings_charpos`, `overlay_strings[16]`
  /// (offset 384 … 528).
  pub _pad_c: [u8; 144],
  /// For each overlay string, the overlay it came from.  (C 2526)
  pub string_overlays: [Lisp_Object; OVERLAY_STRING_CHUNK_SIZE as usize],  // offset 528
  /// If non-nil, a Lisp string being processed.  (C 2532)
  pub string: Lisp_Object,              // offset 656
  /// If non-nil, we are processing a string that came from a `display' property given by
  /// an overlay.  (C 2536)
  pub from_overlay: Lisp_Object,        // offset 664
  /// Stack of saved values.  (C 2541)
  pub stack: [iterator_stack_entry; IT_STACK_SIZE as usize],              // offset 672
  /// Stack pointer.  (C 2667)
  pub sp: c_int,                        // offset 2192
  /// C 2670-2690: `selective`, `what`, `face_id` and the thirteen `bool_bf : 1` members
  /// `selective_display_ellipsis_p` … `align_visually_p` (offset 2196 … 2220).
  pub _pad_d: [u8; 24],
  pub line_wrap: line_wrap_method,      // 2692, offset 2220
  /// C 2696: `base_face_id` (offset 2224 … 2228).
  pub _pad_e: [u8; 4],
  /// If `what' == IT_CHARACTER, the character and the length in bytes of its multibyte
  /// sequence.  (C 2711)
  pub c: c_int,                         // offset 2228
  /// C 2711: `len` (offset 2232 … 2240).
  pub _pad_f: [u8; 8],
  /// If what == IT_COMPOSITION, iterator substructure for the composition.  (C 2715)
  pub cmp_it: composition_it,           // offset 2240
  /// C 2720: `char_to_display` (offset 2328 … 2336).
  pub _pad_g: [u8; 8],
  /// If `what' == IT_IMAGE, the id of the image to display.  (C 2725)
  pub image_id: ptrdiff_t,              // offset 2336
  /// C 2344-2436: `xwidget`, `slice`, `space_width`, `voffset`, `tab_width`,
  /// `font_height`, `object`, `position`, `truncation_pixel_width`,
  /// `continuation_pixel_width` (offset 2344 … 2436).
  pub _pad_h: [u8; 92],
  /// First and last visible x-position in the display area.  (C 2806)
  pub first_visible_x: c_int,           // offset 2436
  pub last_visible_x: c_int,            // 2440
  /// C 2810-2830: `last_visible_y`, `extra_line_spacing`, `extra_line_spacing_above`,
  /// `max_extra_line_spacing`, `override_ascent`, `override_descent`, `override_boff`,
  /// `glyph_row` (offset 2444 … 2480).
  pub _pad_i: [u8; 36],
  /// The area of glyph_row to which glyphs are added.  (C 2833)
  pub area: c_int,                      // offset 2480
  /// C 2837-2864: `nglyphs`, `pixel_width`, `ascent`, `descent`, `max_ascent`,
  /// `max_descent`, `phys_ascent`, `phys_descent`, `max_phys_ascent`, `max_phys_descent`
  /// (offset 2484 … 2524).
  pub _pad_j: [u8; 40],
  /// Current x pixel position within the display line.  (C 2866)
  pub current_x: c_int,                 // offset 2524
  /// C 2877: `wrap_prefix_width` (offset 2528 … 2532).
  pub _pad_k: [u8; 4],
  /// Accumulated width of continuation lines.  (C 2882)
  pub continuation_lines_width: c_int,  // offset 2532
  /// C 2891: `eol_pos` (offset 2536 … 2552).
  pub _pad_l: [u8; 16],
  /// Current y-position.  (C 2895)
  pub current_y: c_int,                 // offset 2552
  /// C 2898: `first_vpos` (offset 2556 … 2560).
  pub _pad_m: [u8; 4],
  /// Current vertical matrix position, or line number.  (C 2902)
  pub vpos: c_int,                      // offset 2560
  /// Horizontal matrix position reached in move_it_in_display_line.  (C 2907)
  pub hpos: c_int,                      // offset 2564
  /// C 2570-2577: `lnum`, `lnum_bytepos` (offset 2568 … 2584).
  pub _pad_n: [u8; 16],
  /// The width, in columns and in pixels, needed for display of the line numbers.  (C 2915)
  pub lnum_width: c_int,                // offset 2584
  pub lnum_pixel_width: c_int,          // 2588
  /// C 2920-2930: `pt_lnum`, `stretch_adjust` and the four `unsigned` fringe bitfields
  /// plus `bidi_p : 1` (the measured 12 bytes at 2604 … 2616).
  pub _pad_o: [u8; 24],
  /// For iterating over bidirectional text.  (C 2933)
  pub bidi_it: bidi_it,                 // offset 2616
  /// C 2934-2936: `paragraph_embedding`, `min_width_property`, `min_width_start`
  /// (offset 4944 … 4968).
  pub _pad_p: [u8; 24],
}

const _: () = core::assert! (std::mem::size_of::<it> () == 4968);
const _: () = core::assert! (std::mem::align_of::<it> () == 8);
const _: () = core::assert! (std::mem::offset_of! (it, window) == 0);
const _: () = core::assert! (std::mem::offset_of! (it, w) == 8);
const _: () = core::assert! (std::mem::offset_of! (it, f) == 16);
const _: () = core::assert! (std::mem::offset_of! (it, method) == 24);
const _: () = core::assert! (std::mem::offset_of! (it, bits) == 112);
const _: () = core::assert! (std::mem::offset_of! (it, current) == 336);
const _: () = core::assert! (std::mem::offset_of! (it, string_overlays) == 528);
const _: () = core::assert! (std::mem::offset_of! (it, string) == 656);
const _: () = core::assert! (std::mem::offset_of! (it, from_overlay) == 664);
const _: () = core::assert! (std::mem::offset_of! (it, stack) == 672);
const _: () = core::assert! (std::mem::offset_of! (it, sp) == 2192);
const _: () = core::assert! (std::mem::offset_of! (it, line_wrap) == 2220);
const _: () = core::assert! (std::mem::offset_of! (it, c) == 2228);
const _: () = core::assert! (std::mem::offset_of! (it, cmp_it) == 2240);
const _: () = core::assert! (std::mem::offset_of! (it, image_id) == 2336);
const _: () = core::assert! (std::mem::offset_of! (it, first_visible_x) == 2436);
const _: () = core::assert! (std::mem::offset_of! (it, last_visible_x) == 2440);
const _: () = core::assert! (std::mem::offset_of! (it, area) == 2480);
const _: () = core::assert! (std::mem::offset_of! (it, current_x) == 2524);
const _: () = core::assert! (std::mem::offset_of! (it, continuation_lines_width) == 2532);
const _: () = core::assert! (std::mem::offset_of! (it, current_y) == 2552);
const _: () = core::assert! (std::mem::offset_of! (it, vpos) == 2560);
const _: () = core::assert! (std::mem::offset_of! (it, hpos) == 2564);
const _: () = core::assert! (std::mem::offset_of! (it, lnum_width) == 2584);
const _: () = core::assert! (std::mem::offset_of! (it, lnum_pixel_width) == 2588);
const _: () = core::assert! (std::mem::offset_of! (it, bidi_it) == 2616);

/* `struct it`'s first `bool_bf` run (dispextern.h 2459-2471, one byte at 112): C reads
   `it.string_from_display_prop_p`, which the port spells as the member name plus the mask
   (PORTING.md 1.3/P-80, the ccl.rs `ccl_program_bits!` shape).  */
pub const IT_MULTIBYTE_P: u8 = 1 << 0;
pub const IT_TAB_LINE_P: u8 = 1 << 1;
pub const IT_HEADER_LINE_P: u8 = 1 << 2;
pub const IT_STRING_FROM_DISPLAY_PROP_P: u8 = 1 << 3;
pub const IT_STRING_FROM_PREFIX_PROP_P: u8 = 1 << 4;
pub const IT_FROM_DISP_PROP_P: u8 = 1 << 5;
pub const IT_ELLIPSIS_P: u8 = 1 << 6;
pub const IT_AVOID_CURSOR_P: u8 = 1 << 7;

macro_rules! it_bits {
  (string_from_display_prop_p, $it:expr) => { (($it).bits & IT_STRING_FROM_DISPLAY_PROP_P != 0) };
}

/* C: dispextern.h 286-287, 291-292 and 2940-2943: the position macros every display unit
   expands (`tools/extract-abi.py`'s dispextern.rs WIP note says they "arrive with the
   functions that use them").  They are `macro_rules!` here, with C's own bodies.  */
macro_rules! BYTEPOS {
  ($pos:expr) => { ($pos).bytepos };
}
macro_rules! CHARPOS {
  ($pos:expr) => { ($pos).charpos };
}
macro_rules! SET_TEXT_POS {
  ($pos:expr, $charpos:expr, $bytepos:expr) => {
    { ($pos).charpos = ($charpos); ($pos).bytepos = ($bytepos); }
  };
}
macro_rules! SET_TEXT_POS_FROM_MARKER {
  ($pos:expr, $marker:expr) => {
    { CHARPOS! ($pos) = marker_position ($marker); BYTEPOS! ($pos) = marker_byte_position ($marker); }
  };
}
macro_rules! IT_CHARPOS {
  ($it:expr) => { CHARPOS! (($it).current.pos) };
}
macro_rules! IT_BYTEPOS {
  ($it:expr) => { BYTEPOS! (($it).current.pos) };
}
macro_rules! IT_STRING_BYTEPOS {
  ($it:expr) => { BYTEPOS! (($it).current.string_pos) };
}

/* C 35: `#define CR 015`.  C's octal literal has no Rust spelling the constants rule
   reads (`scan_const_value`/`norm_const` take `0400` but not `0o400`), so the port writes
   the decimal value; docs/ports/indent.md records the spelling.  */
pub const CR: c_int = 13;

/* These three values memorize the current column to avoid recalculation.  */

/* Last value returned by current_column.
   Some things in set last_known_column_point to -1
   to mark the memorized value as invalid.  */

static mut last_known_column: ptrdiff_t = 0;

/* Value of point when current_column was called.  */

#[unsafe(no_mangle)]
pub static mut last_known_column_point: ptrdiff_t = 0;

/* Value of MODIFF when current_column was called.  */

static mut last_known_column_modified: modiff_count = 0;

/* The `DEFVAR_BOOL` variable this unit owns (`syms_of_indent`): C's storage is
   `globals.f_indent_tabs_mode` (lisp.h 3512-3515) and the unit that owns it defines it
   (status §3.29, the casetab.rs/insdel.rs shape); `include/globals.rs`'s `variables`
   module declares the name, so this definition shadows the glob import.  */
#[unsafe(no_mangle)]
pub static mut indent_tabs_mode: bool = false;

/* C 53-54's forward declarations of `current_column_1` and `position_indentation` are
   dropped: Rust's items are order-independent and a declaration plus the definition is
   E0428 (PORTING.md 0.1-12).  */

/* Get the display table to use for the current buffer.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn buffer_display_table () -> *mut Lisp_Char_Table
{
  let mut thisbuf: Lisp_Object = Qnil ();

  thisbuf = BVAR! ((current_buffer! ()), display_table_);
  if DISP_TABLE_P (thisbuf)
    { return XCHAR_TABLE (thisbuf); }
  if DISP_TABLE_P (Vstandard_display_table)
    { return XCHAR_TABLE (Vstandard_display_table); }
  return 0 as *mut Lisp_Char_Table;
}

/* Width run cache considerations.  */

/* Return the width of character C under display table DP.  */

unsafe fn character_width (c: c_int, dp: *mut Lisp_Char_Table) -> c_int
{
  let mut elt: Lisp_Object = Qnil ();

  /* These width computations were determined by examining the cases
     in display_text_line.  */

  /* Everything can be handled by the display table, if it's
     present and the element is right.  */
  if !dp.is_null () && { elt = DISP_CHAR_VECTOR (dp, c); VECTORP (elt) }
    { return ASIZE (elt) as c_int; }

  /* Some characters are special.  */
  if c == b'\n' as c_int || c == b'\t' as c_int || c == 0o15 as c_int
    { return 0; }

  /* Printing characters have width 1.  */
  else if c >= 0o40 && c < 0o177
    { return 1; }

  /* Everybody else (control characters, metacharacters) has other
     widths.  We could return their actual widths here, but they
     depend on things like ctl_arrow and crud like that, and they're
     not very common at all.  So we'll just claim we don't know their
     widths.  */
  else
    { return 0; }
}

/* Return true if the display table DISPTAB specifies the same widths
   for characters as WIDTHTAB.  We use this to decide when to
   invalidate the buffer's width_run_cache.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn disptab_matches_widthtab (disptab: *mut Lisp_Char_Table,
						   widthtab: *mut Lisp_Vector) -> bool
{
  eassert! ((*widthtab).header.size == 256);

  for i in 0..256
    {
      if character_width (i, disptab)
	   != XFIXNAT (FAM_ELEM! ((*widthtab).contents, i)) as c_int
	{ return false; }
    }

  return true;
}

/* Recompute BUF's width table, using the display table DISPTAB.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn recompute_width_table (buf: *mut buffer,
						disptab: *mut Lisp_Char_Table)
{
  let mut widthtab: *mut Lisp_Vector = std::ptr::null_mut ();

  if !VECTORP (BVAR! (buf, width_table_))
    { bset_width_table (buf, make_uninit_vector (256)); }
  widthtab = XVECTOR (BVAR! (buf, width_table_));
  eassert! ((*widthtab).header.size == 256);

  for i in 0..256
    {
      XSETFASTINT! (FAM_ELEM! ((*widthtab).contents, i),
		    character_width (i, disptab) as EMACS_INT);
    }
}

/* Allocate or free the width run cache, as requested by the
   current state of current_buffer's cache_long_scans variable.  */

/// C 144-145: C returns `struct region_cache *`, and `region-cache.h` has no mirror
/// (P-26/P-66), so the return type is the ABI-equal `*mut c_void` (0.1-2/P-72 rule 3, the
/// insdel.rs `invalidate_region_cache` shape).
unsafe fn width_run_cache_on_off () -> *mut c_void
{
  let mut cache_buffer: *mut buffer = (current_buffer! ());
  let mut indirect_p: bool = false;

  if !(*cache_buffer).base_buffer.is_null ()
    {
      cache_buffer = (*cache_buffer).base_buffer;
      indirect_p = true;
    }

  if NILP (BVAR! ((current_buffer! ()), cache_long_scans_))
      /* And, for the moment, this feature doesn't work on multibyte
         characters.  */
      || !NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
    {
      if !indirect_p
	  || NILP (BVAR! (cache_buffer, cache_long_scans_))
	  || !NILP (BVAR! (cache_buffer, enable_multibyte_characters_))
	{
	  /* It should be off.  */
	  if !(*cache_buffer).width_run_cache.is_null ()
	    {
	      free_region_cache ((*cache_buffer).width_run_cache);
	      (*cache_buffer).width_run_cache = 0 as *mut c_void;
	      bset_width_table ((current_buffer! ()), Qnil ());
	    }
        }
      return std::ptr::null_mut ();
    }
  else
    {
      if !indirect_p
	  || (!NILP (BVAR! (cache_buffer, cache_long_scans_))
	      && NILP (BVAR! (cache_buffer, enable_multibyte_characters_)))
	{
	  /* It should be on.  */
	  if (*cache_buffer).width_run_cache.is_null ()
	    {
	      (*cache_buffer).width_run_cache = new_region_cache ();
	      recompute_width_table ((current_buffer! ()), buffer_display_table ());
	    }
	}
      return (*cache_buffer).width_run_cache;
    }
}

/* Skip some invisible characters starting from POS.
   This includes characters invisible because of text properties
   and characters invisible because of overlays.

   If position POS is followed by invisible characters,
   skip some of them and return the position after them.
   Otherwise return POS itself.

   Set *NEXT_BOUNDARY_P to the next position at which
   it will be necessary to call this function again.

   Don't scan past TO, and don't set *NEXT_BOUNDARY_P
   to a value greater than TO.

   If WINDOW is non-nil, and this buffer is displayed in WINDOW,
   take account of overlays that apply only in WINDOW.

   We don't necessarily skip all the invisible characters after POS
   because that could take a long time.  We skip a reasonable number
   which can be skipped quickly.  If there might be more invisible
   characters immediately following, then *NEXT_BOUNDARY_P
   will equal the return value.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn skip_invisible (pos: ptrdiff_t, next_boundary_p: *mut ptrdiff_t,
					 to: ptrdiff_t, window: Lisp_Object) -> ptrdiff_t
{
  let mut prop: Lisp_Object = Qnil ();
  let mut position: Lisp_Object = Qnil ();
  let mut overlay_limit: Lisp_Object = Qnil ();
  let mut proplimit: Lisp_Object = Qnil ();
  let mut buffer: Lisp_Object = Qnil ();
  let mut tmp: Lisp_Object = Qnil ();
  let mut end: ptrdiff_t = 0;
  let mut inv_p: c_int = 0;

  XSETFASTINT! (position, pos);
  XSETBUFFER! (buffer, (current_buffer! ()));

  /* We must not advance farther than the next overlay change.
     The overlay change might change the invisible property;
     or there might be overlay strings to be displayed there.  */
  overlay_limit = Fnext_overlay_change (position);
  /* As for text properties, this gives a lower bound
     for where the invisible text property could change.  */
  proplimit = Fnext_property_change (position, buffer, Qt ());
  if XFIXNAT (overlay_limit) < XFIXNAT (proplimit)
    { proplimit = overlay_limit; }
  /* PROPLIMIT is now a lower bound for the next change
     in invisible status.  If that is plenty far away,
     use that lower bound.  */
  if XFIXNAT (proplimit) > pos + 100 || XFIXNAT (proplimit) >= to
    { *next_boundary_p = XFIXNAT (proplimit); }
  /* Otherwise, scan for the next `invisible' property change.  */
  else
    {
      /* Don't scan terribly far.  */
      XSETFASTINT! (proplimit, min (pos + 100, to));
      /* No matter what, don't go past next overlay change.  */
      if XFIXNAT (overlay_limit) < XFIXNAT (proplimit)
	{ proplimit = overlay_limit; }
      tmp = Fnext_single_property_change (position, Qinvisible (),
					  buffer, proplimit);
      end = XFIXNAT (tmp);
      /* C 252-260's `#if 0` block (the multibyte-form boundary walk) is dead text; it is
	 recorded in docs/ports/indent.md rather than transcribed (P-08).  */
      *next_boundary_p = end;
    }
  /* if the `invisible' property is set, we can skip to
     the next property change */
  prop = Fget_char_property (position, Qinvisible (),
			     if !NILP (window)
			      && EQ ((*XWINDOW (window)).contents, buffer)
			     { window } else { buffer });
  inv_p = TEXT_PROP_MEANS_INVISIBLE! (prop);
  /* When counting columns (window == nil), don't skip over ellipsis text.  */
  if if NILP (window) { inv_p == 1 } else { inv_p != 0 }
    { return *next_boundary_p; }
  return pos;
}

/* Set variables WIDTH and BYTES for a multibyte sequence starting at P.

   DP is a display table or NULL.

   This macro is used in scan_for_column and in
   compute_motion.  */

macro_rules! MULTIBYTE_BYTES_WIDTH {
  ($p:expr, $dp:expr, $bytes:ident, $width:ident) => {
    {
      let mut ch: c_int = string_char_and_length ($p, &raw mut $bytes);
      if BYTES_BY_CHAR_HEAD (*$p as c_int) != $bytes
	{ $width = $bytes * 4; }
      else
	{
	  if $dp != 0 as *mut Lisp_Char_Table && VECTORP (DISP_CHAR_VECTOR ($dp, ch))
	    { $width = sanitize_char_width (ASIZE (DISP_CHAR_VECTOR ($dp, ch))); }
	  else
	    { $width = CHARACTER_WIDTH (ch); }
	}
    }
  };
}

DEFUN! ("current-column", Fcurrent_column, Scurrent_column, 0, 0, 0,
       doc: "Return the horizontal position of point.  Beginning of line is column 0.\nThis is calculated by adding together the widths of all the displayed\nrepresentations of the character between the start of the previous line\nand point (e.g., control characters will have a width of 2 or 4, tabs\nwill have a variable width).\nIgnores finite width of frame, which means that this function may return\nvalues greater than (frame-width).\nIn a buffer with very long lines, the value will be an approximation,\nbecause calculating the exact number is very expensive.\nWhether the line is visible (if `selective-display' is t) has no effect;\nhowever, ^M is treated as end of line when `selective-display' is t.\nText that has an invisible property is considered as having width 0, unless\n`buffer-invisibility-spec' specifies that it is replaced by an ellipsis.  ",
       (),
       {
  let mut temp: Lisp_Object = Qnil ();

  XSETFASTINT! (temp, current_column ());
  return temp;
});

/* Cancel any recorded value of the horizontal position.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn invalidate_current_column ()
{
  last_known_column_point = 0;
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn current_column () -> ptrdiff_t
{
  let mut col: ptrdiff_t = 0;
  let mut ptr: *mut u8 = std::ptr::null_mut ();
  let mut stop: *mut u8 = std::ptr::null_mut ();
  let mut tab_seen: bool = false;
  let mut post_tab: ptrdiff_t = 0;
  let mut c: c_int = 0;
  let tab_width: c_int = SANE_TAB_WIDTH ((current_buffer! ()));
  let ctl_arrow: bool = !NILP (BVAR! ((current_buffer! ()), ctl_arrow_));
  let dp: *mut Lisp_Char_Table = buffer_display_table ();

  if PT! () == last_known_column_point
      && MODIFF! () == last_known_column_modified
    { return last_known_column; }

  let mut line_beg: ptrdiff_t = find_newline (PT! (), PT_BYTE! (), BEGV! (), BEGV_BYTE! (),
					      -1, std::ptr::null_mut (),
					      std::ptr::null_mut (), true);

  /* Avoid becoming abysmally slow for very long lines.  */
  if (*(current_buffer! ())).bits & BUFFER_LONG_LINE_OPTIMIZATIONS_P != 0
      && !NILP (Vlong_line_threshold)
      && PT! () - line_beg > XFIXNUM (Vlong_line_threshold)
    { return PT! () - line_beg; }	/* this is an approximation! */
  /* If the buffer has overlays, text properties,
     or multibyte characters, use a more general algorithm.  */
  if !buffer_intervals ((current_buffer! ())).is_null ()
      || buffer_has_overlays ()
      || Z! () != Z_BYTE! ()
    { return current_column_1 (); }

  /* Scan backwards from point to the previous newline,
     counting width.  Tab characters are the only complicated case.  */

  /* Make a pointer for decrementing through the chars before point.  */
  ptr = BYTE_POS_ADDR (PT_BYTE! () - 1).offset (1);
  /* Make a pointer to where consecutive chars leave off,
     going backwards from point.  */
  if PT! () == BEGV! ()
    { stop = ptr; }
  else if PT! () <= GPT! () || BEGV! () > GPT! ()
    { stop = BEGV_ADDR! (); }
  else
    { stop = GAP_END_ADDR! (); }

  col = 0;
  tab_seen = false;
  post_tab = 0;

  'start_of_line_found: while true
    {
      let mut i: ptrdiff_t = 0;
      let mut n: ptrdiff_t = 0;
      let mut charvec: Lisp_Object = Qnil ();

      if ptr == stop
	{
	  /* We stopped either for the beginning of the buffer
	     or for the gap.  */
	  if ptr == BEGV_ADDR! ()
	    { break; }

	  /* It was the gap.  Jump back over it.  */
	  stop = BEGV_ADDR! ();
	  ptr = GPT_ADDR! ();

	  /* Check whether that brings us to beginning of buffer.  */
	  if BEGV! () >= GPT! ()
	    { break; }
	}

      ptr = ptr.offset (-1);
      c = *ptr as c_int;

      if !dp.is_null () && VECTORP (DISP_CHAR_VECTOR (dp, c))
	{
	  charvec = DISP_CHAR_VECTOR (dp, c);
	  n = ASIZE (charvec);
	}
      else
	{
	  charvec = Qnil ();
	  n = 1;
	}

      i = n - 1;
      while i >= 0
	{
	  if VECTORP (charvec)
	    {
	      /* This should be handled the same as
		 next_element_from_display_vector does it.  */
	      let entry: Lisp_Object = AREF (charvec, i);

	      if GLYPH_CODE_P (entry)
		{ c = GLYPH_CODE_CHAR (entry); }
	      else
		{ c = b' ' as c_int; }
	    }

	  if c >= 0o40 && c < 0o177
	    { col += 1; }
	  else if c == b'\n' as c_int
		   || (c == b'\r' as c_int
		       && EQ (BVAR! ((current_buffer! ()), selective_display_), Qt ()))
	    {
	      ptr = ptr.offset (1);
	      break 'start_of_line_found;
	    }
	  else if c == b'\t' as c_int
	    {
	      if tab_seen
		{ col = ((col + tab_width as ptrdiff_t) / tab_width as ptrdiff_t)
			  * tab_width as ptrdiff_t; }

	      post_tab += col;
	      col = 0;
	      tab_seen = true;
	    }
	  else if VECTORP (charvec)
	    /* With a display table entry, C is displayed as is, and
	       not displayed as \NNN or as ^N.  If C is a single-byte
	       character, it takes one column.  If C is multi-byte in
	       a unibyte buffer, it's translated to unibyte, so it
	       also takes one column.  */
	    { col += 1; }
	  else
	    { col += if ctl_arrow && c < 0o200 { 2 } else { 4 }; }

	  i -= 1;
	}
    }

  if tab_seen
    {
      col = ((col + tab_width as ptrdiff_t) / tab_width as ptrdiff_t)
	      * tab_width as ptrdiff_t;
      col += post_tab;
    }

  last_known_column = col;
  last_known_column_point = PT! ();
  last_known_column_modified = MODIFF! ();

  return col;
}

/* C 469's forward declaration of `restore_window_buffer` is dropped (PORTING.md 0.1-12).  */

/* Check the presence of a display property and compute its width.
   POS and POS_BYTE are the character and byte positions of the
   display property and COL is the column number at POS.
   If a property was found and its width was found as well, return
   its width (>= 0) and set the position of the end of the property
   in ENDPOS.
   Otherwise just return -1.
   WINDOW is the window to use when it is important; nil stands for
   the selected window.  */
unsafe fn check_display_width (window: Lisp_Object,
			       pos: ptrdiff_t, pos_byte: ptrdiff_t, col: ptrdiff_t,
			       endpos: *mut ptrdiff_t) -> c_int
{
  let mut val: Lisp_Object = Qnil ();
  let mut overlay: Lisp_Object = Qnil ();

  if !NILP ({ val = get_char_property_and_overlay (make_fixnum (pos), Qdisplay (),
						   Qnil (), &raw mut overlay); val })
    {
      let mut width: c_int = -1;
      let mut plist: Lisp_Object = Qnil ();

      if CONSP (val)
	{
	  let xcar: Lisp_Object = XCAR (val);

	  /* Handle '(space ...)' display specs.  */
	  if EQ (Qspace (), xcar)
	    { /* FIXME: Use calc_pixel_width_or_height.  */
	      let mut prop: Lisp_Object = Qnil ();
	      let align_to_max: EMACS_INT =
		if col < MOST_POSITIVE_FIXNUM - INT_MAX as isize
		 { INT_MAX as EMACS_INT + col }
		 else
		 { MOST_POSITIVE_FIXNUM };

	      plist = XCDR (val);
	      if { prop = plist_get (plist, QCwidth ());
		   RANGED_FIXNUMP (0, prop, INT_MAX as EMACS_INT) }
		  || { prop = plist_get (plist, QCrelative_width ());
		       RANGED_FIXNUMP (0, prop, INT_MAX as EMACS_INT) }
		{ width = XFIXNUM (prop) as c_int; }
	      else if FLOATP (prop) && 0.0 <= XFLOAT_DATA (prop)
		       && XFLOAT_DATA (prop) <= INT_MAX as c_double
		{ width = (XFLOAT_DATA (prop) + 0.5) as c_int; }
	      else if { prop = plist_get (plist, QCalign_to ());
			RANGED_FIXNUMP (col, prop, align_to_max) }
		{ width = XFIXNUM (prop) as c_int - col as c_int; }
	      else if FLOATP (prop) && col as c_double <= XFLOAT_DATA (prop)
		       && (XFLOAT_DATA (prop) <= align_to_max as c_double)
		{ width = (XFLOAT_DATA (prop) + 0.5) as c_int - col as c_int; }
	    }
	    /* Handle images.  */
	  else if EQ (Qimage (), xcar)
		   || EQ (Qslice (), xcar)
		   /* 'insert-sliced-image' creates property of the form
                      ((slice ...) image ...) */
		   || (CONSP (xcar) && EQ (Qslice (), XCAR (xcar)))
	    {
	      let count: specpdl_ref = SPECPDL_INDEX ();
	      let w: *mut window = decode_live_window (window);

	      /* If needed, set the window's buffer temporarily to the
                 current buffer and its window-point to POS.  */
	      if XBUFFER ((*w).contents) != (current_buffer! ())
		{
		  let oldbuf: Lisp_Object
		    = list4 (window, (*w).contents,
			     make_fixnum (marker_position ((*w).pointm)),
			     make_fixnum (marker_byte_position ((*w).pointm)));
		  record_unwind_protect (Some (restore_window_buffer), oldbuf);
		  wset_buffer (w, Fcurrent_buffer ());
		  set_marker_both ((*w).pointm, (*w).contents, pos, pos_byte);
		}

	      let mut startpos: text_pos = std::mem::zeroed ();
	      let mut it: it = std::mem::zeroed ();
	      SET_TEXT_POS! (startpos, pos, pos_byte);
	      let itdata: *mut c_void = bidi_shelve_cache ();
	      record_unwind_protect_void (Some (unwind_display_working_on_window));
	      display_working_on_window_p = true;
	      start_display (&raw mut it, w, startpos);
	      it.last_visible_x = 1000000; /* prevent image clipping */
	      /* Forcing HPOS non-zero prevents the display code from
                 generating line/wrap-prefix and line numbers, which
                 would otherwise skew the X coordinate we obtain below.  */
	      it.hpos = 1;
	      /* The POS+1 value is a trick: move_it_in_display_line
                 will not return until it finished processing the entire
                 image, even if it covers more than one buffer position.  */
	      move_it_in_display_line (&raw mut it, pos + 1, -1, MOVE_TO_POS);
	      /* The caller wants the width in units of the frame's
                 canonical character width.  */
	      width = (((it.current_x as c_double) / FRAME_COLUMN_WIDTH (it.f) as c_double)
		       + 0.5) as c_int;
	      bidi_unshelve_cache (itdata, false);
	      unbind_to (count, Qnil ());
	    }
	}
      /* Handle 'display' strings.   */
      else if STRINGP (val)
	{ width = XFIXNUM (Fstring_width (val, Qnil (), Qnil ())) as c_int; }

      if width >= 0
	{
	  let mut start: ptrdiff_t = 0;
	  if OVERLAYP (overlay)
	    { *endpos = OVERLAY_END (overlay); }
	  else
	    { get_property_and_range (pos, Qdisplay (), &raw mut val, &raw mut start,
				      endpos, Qnil ()); }

	  /* For :relative-width, we need to multiply by the column
	     width of the character at POS, if it is greater than 1.  */
	  if !NILP (plist)
	      && !NILP (plist_get (plist, QCrelative_width ()))
	      && !NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
	    {
	      let mut b: c_int = 0;
	      let mut wd: c_int = 0;
	      let p: *mut u8 = BYTE_POS_ADDR (CHAR_TO_BYTE (pos));

	      MULTIBYTE_BYTES_WIDTH! (p, buffer_display_table (), b, wd);
	      width *= wd;
	    }
	  return width;
	}
    }

  return -1;
}

/* Scanning from the beginning of the current line, stop at the buffer
   position ENDPOS or at the column GOALCOL or at the end of line, whichever
   comes first.
   Return the resulting buffer position and column in ENDPOS and GOALCOL.
   PREVCOL gets set to the column of the previous position (it's always
   strictly smaller than the goal column), and PREVPOS and PREVPBOS get set
   to the corresponding buffer character and byte positions.  */
unsafe fn scan_for_column (endpos: *mut ptrdiff_t, goalcol: *mut EMACS_INT,
			   prevpos: *mut ptrdiff_t, prevbpos: *mut ptrdiff_t,
			   prevcol: *mut ptrdiff_t)
{
  let tab_width: c_int = SANE_TAB_WIDTH ((current_buffer! ()));
  let ctl_arrow: bool = !NILP (BVAR! ((current_buffer! ()), ctl_arrow_));
  let dp: *mut Lisp_Char_Table = buffer_display_table ();
  let multibyte: bool = !NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_));
  let mut cmp_it: composition_it = std::mem::zeroed ();
  let mut dummy_it: it = std::mem::zeroed ();
  let mut window: Lisp_Object = Qnil ();
  let mut w: *mut window = std::ptr::null_mut ();

  /* Start the scan at the beginning of this line with column number 0.  */
  let mut col: ptrdiff_t = 0;
  let mut prev_col: ptrdiff_t = 0;
  let mut goal: EMACS_INT = if !goalcol.is_null () { *goalcol } else { MOST_POSITIVE_FIXNUM };
  let mut end: ptrdiff_t = if !endpos.is_null () { *endpos } else { PT! () };
  let mut scan: ptrdiff_t = 0;
  let mut scan_byte: ptrdiff_t = 0;
  let mut next_boundary: ptrdiff_t = 0;
  let mut prev_pos: ptrdiff_t = 0;
  let mut prev_bpos: ptrdiff_t = 0;

  scan = find_newline (PT! (), PT_BYTE! (), BEGV! (), BEGV_BYTE! (), -1,
		       std::ptr::null_mut (), &raw mut scan_byte, true);

  window = Fget_buffer_window (Fcurrent_buffer (), Qnil ());
  w = if ! NILP (window) { XWINDOW (window) } else { std::ptr::null_mut () };

  /* C's `endloop:` label is the epilogue after the scan loop and the three `goto endloop`
     sites (the long-line block below and the two display-table walks) jump to it; the port
     keeps them as `break 'endloop` out of the labelled block that carries the same text
     (PORTING.md 2.2, the intervals.rs `'check_done` shape).  */
  'endloop: {
  if (*(current_buffer! ())).bits & BUFFER_LONG_LINE_OPTIMIZATIONS_P != 0
    {
      let mut lines_truncated: bool = false;

      if !NILP (BVAR! ((current_buffer! ()), truncate_lines_))
	{ lines_truncated = true; }
      else if !NILP (Vtruncate_partial_width_windows) && !w.is_null ()
	       && (*w).total_cols < FRAME_COLS (XFRAME (WINDOW_FRAME! (w)))
	{
	  if FIXNUMP (Vtruncate_partial_width_windows)
	    { lines_truncated =
		(*w).total_cols < XFIXNAT (Vtruncate_partial_width_windows) as c_int; }
	  else
	    { lines_truncated = true; }
	}
      /* Special optimization for buffers with long and truncated
	 lines: assumes that each character is a single column.  */
      if lines_truncated
	{
	  let bolpos: ptrdiff_t = scan;
	  /* The newline which ends this line or ZV.  */
	  let eolpos: ptrdiff_t =
	    find_newline (PT! (), PT_BYTE! (), ZV! (), ZV_BYTE! (), 1,
			  std::ptr::null_mut (), std::ptr::null_mut (), true);

	  scan = bolpos + goal;
	  if scan > end
	    { scan = end; }
	  if scan > eolpos
	    { scan = if eolpos == ZV! () { ZV! () } else { eolpos - 1 }; }
	  col = scan - bolpos;
	  if col > large_hscroll_threshold
	    {
	      prev_col = col - 1;
	      prev_pos = scan - 1;
	      prev_bpos = CHAR_TO_BYTE (scan);
	      break 'endloop;
	    }
	  /* Restore the values we've overwritten above.  */
	  scan = bolpos;
	  col = 0;
	}
    }
  next_boundary = scan;
  prev_pos = scan;
  prev_bpos = scan_byte;

  memset (&raw mut cmp_it as *mut c_void, 0, std::mem::size_of::<composition_it> ());
  dummy_it.w = w;
  dummy_it.f = if !w.is_null () { XFRAME ((*w).frame) } else { std::ptr::null_mut () };
  cmp_it.parent_it = &raw mut dummy_it;
  cmp_it.id = -1;
  composition_compute_stop_pos (&raw mut cmp_it as *mut c_void, scan, scan_byte, end,
				Qnil (), true);

  /* Scan forward to the target position.  */
  while scan < end
    {
      let mut c: c_int = 0;

      /* Occasionally we may need to skip invisible text.  */
      while scan == next_boundary
	{
	  let old_scan: ptrdiff_t = scan;
	  /* This updates NEXT_BOUNDARY to the next place
	     where we might need to skip more invisible text.  */
	  scan = skip_invisible (scan, &raw mut next_boundary, end, Qnil ());
	  if scan != old_scan
	    { scan_byte = CHAR_TO_BYTE (scan); }
	  if scan >= end
	    { break 'endloop; }
	  /* We may have over-stepped cmp_it.stop_pos while skipping
	     the invisible text.  If so, update cmp_it.stop_pos.  */
	  if scan > cmp_it.stop_pos && cmp_it.id < 0
	    { composition_reseat_it (&raw mut cmp_it as *mut c_void, scan, scan_byte, end,
				     w as *mut c_void, -1 as i8,
				     std::ptr::null_mut (), Qnil ()); }
	}

      /* Test reaching the goal column.  We do this after skipping
	 invisible characters, so that we put point before the
	 character on which the cursor will appear.  */
      if col >= goal as ptrdiff_t
	{ break; }
      prev_col = col;
      prev_pos = scan;
      prev_bpos = scan_byte;

      { /* Check display property.  */
	let mut endp: ptrdiff_t = 0;
	let width: c_int = check_display_width (window, scan, scan_byte, col,
						&raw mut endp);
	if width >= 0
	  {
	    col += width as ptrdiff_t;
	    if endp > scan /* Avoid infinite loops with 0-width overlays.  */
	      {
		scan = endp;
		scan_byte = CHAR_TO_BYTE (scan);
		continue;
	      }
	  }
      }

      /* Check composition sequence.  */
      if cmp_it.id >= 0
	  || (scan == cmp_it.stop_pos
	      && composition_reseat_it (&raw mut cmp_it as *mut c_void, scan, scan_byte,
					end, w as *mut c_void, -1 as i8,
					std::ptr::null_mut (), Qnil ()))
	{ composition_update_it (&raw mut cmp_it as *mut c_void, scan, scan_byte,
				  Qnil ()); }
      if cmp_it.id >= 0
	{
	  scan += cmp_it.nchars as ptrdiff_t;
	  scan_byte += cmp_it.nbytes as ptrdiff_t;
	  if scan <= end
	    { col += cmp_it.width as ptrdiff_t; }
	  if cmp_it.to == cmp_it.nglyphs
	    {
	      cmp_it.id = -1;
	      composition_compute_stop_pos (&raw mut cmp_it as *mut c_void, scan, scan_byte,
					    end, Qnil (), true);
	    }
	  else
	    { cmp_it.from = cmp_it.to; }
	  continue;
	}

      c = FETCH_BYTE! (scan_byte) as c_int;

      /* See if there is a display table and it relates
	 to this character.  */

      if !dp.is_null ()
	  && ! (multibyte && LEADING_CODE_P (c))
	  && VECTORP (DISP_CHAR_VECTOR (dp, c))
	{
	  let mut charvec: Lisp_Object = Qnil ();
	  let mut i: ptrdiff_t = 0;
	  let mut n: ptrdiff_t = 0;

	  /* This character is displayed using a vector of glyphs.
	     Update the column/position based on those glyphs.  */

	  charvec = DISP_CHAR_VECTOR (dp, c);
	  n = ASIZE (charvec);

	  i = 0;
	  while i < n
	    {
	      /* This should be handled the same as
		 next_element_from_display_vector does it.  */
	      let entry: Lisp_Object = AREF (charvec, i);

	      if GLYPH_CODE_P (entry)
		{ c = GLYPH_CODE_CHAR (entry); }
	      else
		{ c = b' ' as c_int; }

	      if c == b'\n' as c_int
		{ break 'endloop; }
	      if c == b'\r' as c_int && EQ (BVAR! ((current_buffer! ()), selective_display_), Qt ())
		{ break 'endloop; }
	      if c == b'\t' as c_int
		{
		  col += tab_width as ptrdiff_t;
		  col = col / tab_width as ptrdiff_t * tab_width as ptrdiff_t;
		}
	      else
		{ col += 1; }
	      i += 1;
	    }
	}
      else
	{
	  /* The display table doesn't affect this character;
	     it displays as itself.  */

	  if c == b'\n' as c_int
	    { break 'endloop; }
	  if c == b'\r' as c_int && EQ (BVAR! ((current_buffer! ()), selective_display_), Qt ())
	    { break 'endloop; }
	  if c == b'\t' as c_int
	    {
	      col += tab_width as ptrdiff_t;
	      col = col / tab_width as ptrdiff_t * tab_width as ptrdiff_t;
	    }
	  else if multibyte && LEADING_CODE_P (c)
	    {
	      /* Start of multi-byte form.  */
	      let mut ptr: *mut u8 = std::ptr::null_mut ();
	      let mut bytes: c_int = 0;
	      let mut width: c_int = 0;

	      ptr = BYTE_POS_ADDR (scan_byte);
	      MULTIBYTE_BYTES_WIDTH! (ptr, dp, bytes, width);
	      /* Subtract one to compensate for the increment
		 that is going to happen below.  */
	      scan_byte += bytes as ptrdiff_t - 1;
	      col += width as ptrdiff_t;
	    }
	  else if ctl_arrow && (c < 0o40 || c == 0o177)
	    { col += 2; }
	  else if c < 0o40 || c >= 0o177
	    { col += 4; }
	  else
	    { col += 1; }
	}
      scan += 1;
      scan_byte += 1;

    }
  }

  last_known_column = col;
  last_known_column_point = PT! ();
  last_known_column_modified = MODIFF! ();

  if !goalcol.is_null ()
    { *goalcol = col as EMACS_INT; }
  if !endpos.is_null ()
    { *endpos = scan; }
  if !prevpos.is_null ()
    { *prevpos = prev_pos; }
  if !prevbpos.is_null ()
    { *prevbpos = prev_bpos; }
  if !prevcol.is_null ()
    { *prevcol = prev_col; }
}

/* Return the column number of point
   by scanning forward from the beginning of the line.
   This function handles characters that are invisible
   due to text properties or overlays.  */

unsafe fn current_column_1 () -> ptrdiff_t
{
  let mut col: EMACS_INT = MOST_POSITIVE_FIXNUM;
  let mut opoint: ptrdiff_t = PT! ();

  scan_for_column (&raw mut opoint, &raw mut col, std::ptr::null_mut (),
		   std::ptr::null_mut (), std::ptr::null_mut ());
  return col as ptrdiff_t;
}

// C 867-944's `#if 0` block (the unused `string_display_width`, whose opening comment
// reads "Not used.") is dead text; it is recorded in docs/ports/indent.md rather than
// transcribed (P-08).


DEFUN! ("indent-to", Findent_to, Sindent_to, 1, 2, "NIndent to column: ",
       doc: "Indent from point with tabs and spaces until COLUMN is reached.\nOptional second argument MINIMUM says always do at least MINIMUM spaces\neven if that goes past COLUMN; by default, MINIMUM is zero.\n\nWhether this uses tabs or spaces depends on `indent-tabs-mode'.\n\nThe return value is the column where the insertion ends.  ",
       (column: Lisp_Object, minimum: Lisp_Object),
       {
  /* C's `DEFVAR_BOOL ("indent-tabs-mode", indent_tabs_mode, ...)` storage: C keeps it in
     `globals.f_indent_tabs_mode`, and the unit that owns it defines it (status §3.29).  */
  let mut minimum: Lisp_Object = minimum;
  let mut column: Lisp_Object = column;
  let mut mincol: EMACS_INT = 0;
  let mut fromcol: ptrdiff_t = 0;
  let tab_width: c_int = SANE_TAB_WIDTH ((current_buffer! ()));

  CHECK_FIXNUM (column);
  if NILP (minimum)
    { XSETFASTINT! (minimum, 0); }
  CHECK_FIXNUM (minimum);

  fromcol = current_column ();
  mincol = fromcol + XFIXNUM (minimum);
  if mincol < XFIXNUM (column) { mincol = XFIXNUM (column); }

  if fromcol == mincol
    { return make_fixnum (mincol); }

  if indent_tabs_mode
    {
      let mut n: Lisp_Object = Qnil ();
      XSETFASTINT! (n, mincol / tab_width as EMACS_INT - fromcol / tab_width as ptrdiff_t);
      if XFIXNAT (n) != 0
	{
	  Finsert_char (make_fixnum (b'\t' as EMACS_INT), n, Qt ());

	  fromcol = (mincol / tab_width as EMACS_INT) * tab_width as EMACS_INT;
	}
    }

  XSETFASTINT! (column, mincol - fromcol);
  Finsert_char (make_fixnum (b' ' as EMACS_INT), column, Qt ());

  last_known_column = mincol;
  last_known_column_point = PT! ();
  last_known_column_modified = MODIFF! ();

  XSETINT! (column, mincol);
  return column;
});

DEFUN! ("current-indentation", Fcurrent_indentation, Scurrent_indentation,
       0, 0, 0,
       doc: "Return the indentation of the current line.\nThis is the horizontal position of the character following any initial\nwhitespace.\nText that has an invisible property is considered as having width 0, unless\n`buffer-invisibility-spec' specifies that it is replaced by an ellipsis.  ",
       (),
       {
  let mut posbyte: ptrdiff_t = 0;

  find_newline (PT! (), PT_BYTE! (), BEGV! (), BEGV_BYTE! (), -1,
		std::ptr::null_mut (), &raw mut posbyte, true);
  return make_fixnum (position_indentation (posbyte));
});

unsafe fn position_indentation (pos_byte: ptrdiff_t) -> ptrdiff_t
{
  let mut pos_byte: ptrdiff_t = pos_byte;
  let mut column: ptrdiff_t = 0;
  let tab_width: c_int = SANE_TAB_WIDTH ((current_buffer! ()));
  let mut p: *mut u8 = std::ptr::null_mut ();
  let mut stop: *mut u8 = std::ptr::null_mut ();
  let mut start: *mut u8 = std::ptr::null_mut ();
  let mut next_boundary_byte: ptrdiff_t = pos_byte;
  let mut ceiling: ptrdiff_t = next_boundary_byte;

  p = BYTE_POS_ADDR (pos_byte);
  /* STOP records the value of P at which we will need
     to think about the gap, or about invisible text,
     or about the end of the buffer.  */
  stop = p;
  /* START records the starting value of P.  */
  start = p;
  /* C's `while (1)` leaves only by `return`, so the port's loop is `loop` (PORTING.md
     2.2, the intervals.rs `find_interval` spelling).  */
  loop
    {
      while p == stop
	{
	  let mut stop_pos_byte: ptrdiff_t = 0;

	  /* If we have updated P, set POS_BYTE to match.
	     The first time we enter the loop, POS_BYTE is already right.  */
	  if p != start
	    { pos_byte = PTR_BYTE_POS (p); }
	  /* Consider the various reasons STOP might have been set here.  */
	  if pos_byte == ZV_BYTE! ()
	    { return column; }
	  if pos_byte == next_boundary_byte
	    {
	      let mut next_boundary: ptrdiff_t = 0;
	      let mut pos: ptrdiff_t = BYTE_TO_CHAR (pos_byte);
	      pos = skip_invisible (pos, &raw mut next_boundary, ZV! (), Qnil ());
	      pos_byte = CHAR_TO_BYTE (pos);
	      next_boundary_byte = CHAR_TO_BYTE (next_boundary);
	    }
	  if pos_byte >= ceiling
	    { ceiling = BUFFER_CEILING_OF (pos_byte) + 1; }
	  /* Compute the next place we need to stop and think,
	     and set STOP accordingly.  */
	  stop_pos_byte = min (ceiling, next_boundary_byte);
	  /* The -1 and +1 arrange to point at the first byte of gap
	     (if STOP_POS_BYTE is the position of the gap)
	     rather than at the data after the gap.  */

	  stop = BYTE_POS_ADDR (stop_pos_byte - 1).offset (1);
	  p = BYTE_POS_ADDR (pos_byte);
	}
      /* C's `switch (*p++)` becomes a `match` on the byte it advances past, and the
	 `FALLTHROUGH` from the 0240 arm's false case into the ' ' arm is that shared arm.  */
      let c: c_int = { let c = *p as c_int; p = p.offset (1); c };
      match c
	{
	  0o240 | 0x20 =>
	    {
	      if c == 0o240
		 && ! NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
		{ return column; }
	      column += 1;
	    }
	  0x09 =>
	    { column += tab_width as ptrdiff_t - column % tab_width as ptrdiff_t; }
	  _ =>
	    {
	      if ASCII_CHAR_P (*p.offset (-1) as isize)
		  || NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
		{ return column; }
	      {
		let mut c: c_int = 0;
		pos_byte = PTR_BYTE_POS (p.offset (-1));
		c = FETCH_MULTIBYTE_CHAR (pos_byte);
		if CHAR_HAS_CATEGORY (c, b' ' as c_int)
		  {
		    column += 1;
		    pos_byte += next_char_len (pos_byte) as ptrdiff_t;
		    p = BYTE_POS_ADDR (pos_byte);
		  }
		else
		  { return column; }
	      }
	    }
	}
    }
}

/* Test whether the line beginning at POS is indented beyond COLUMN.
   Blank lines are treated as if they had the same indentation as the
   preceding line.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn indented_beyond_p (pos: ptrdiff_t, pos_byte: ptrdiff_t,
					     column: EMACS_INT) -> bool
{
  let mut pos: ptrdiff_t = pos;
  let mut pos_byte: ptrdiff_t = pos_byte;

  while pos > BEGV! () && FETCH_BYTE! (pos_byte) == b'\n'
    {
      dec_both (&raw mut pos, &raw mut pos_byte);
      pos = find_newline (pos, pos_byte, BEGV! (), BEGV_BYTE! (),
			  -1, std::ptr::null_mut (), &raw mut pos_byte, false);
    }
  return position_indentation (pos_byte) >= column;
}

DEFUN! ("move-to-column", Fmove_to_column, Smove_to_column, 1, 2,
       "NMove to column: ",
       doc: "Move point to column COLUMN in the current line.\nInteractively, COLUMN is the value of prefix numeric argument.\nThe column of a character is calculated by adding together the widths\nas displayed of the previous characters in the line.\nThis function ignores line-continuation;\nthere is no upper limit on the column number a character can have\nand horizontal scrolling has no effect.\nText that has an invisible property is considered as having width 0,\nunless `buffer-invisibility-spec' specifies that it is replaced by\nan ellipsis.\n\nIf specified column is within a character, point goes after that character.\nIf it's past end of line, point goes to end of line.\n\nOptional second argument FORCE non-nil means if COLUMN is in the\nmiddle of a tab character, either change it to spaces (when\n`indent-tabs-mode' is nil), or insert enough spaces before it to reach\nCOLUMN (otherwise).  In addition, if FORCE is t, and the line is too short\nto reach COLUMN, add spaces/tabs to get there.\n\nThe return value is the current column.  ",
       (column: Lisp_Object, force: Lisp_Object),
       {
  let mut pos: ptrdiff_t = 0;
  let mut prev_pos: ptrdiff_t = 0;
  let mut prev_bpos: ptrdiff_t = 0;
  let mut prev_col: ptrdiff_t = 0;
  let mut col: EMACS_INT = 0;
  let mut goal: EMACS_INT = 0;

  CHECK_FIXNAT (column);
  goal = XFIXNUM (column);

  col = goal;
  pos = ZV! ();
  scan_for_column (&raw mut pos, &raw mut col, &raw mut prev_pos, &raw mut prev_bpos,
		   &raw mut prev_col);

  SET_PT (pos);

  /* If a tab char made us overshoot, change it to spaces
     and scan through it again.  */
  if !NILP (force) && col > goal
    {
      let mut c: c_int = 0;

      c = FETCH_CHAR (prev_bpos);
      if c == b'\t' as c_int && prev_col < goal && prev_bpos < PT_BYTE! ()
	{
	  let mut goal_pt: ptrdiff_t = 0;
	  let mut goal_pt_byte: ptrdiff_t = 0;

	  /* Insert spaces in front of the tab to reach GOAL.  Do this
	     first so that a marker at the end of the tab gets
	     adjusted.  */
	  SET_PT_BOTH (prev_pos, prev_bpos);
	  Finsert_char (make_fixnum (b' ' as EMACS_INT),
			make_fixnum (goal - prev_col), Qt ());

	  /* Now delete the tab, and indent to COL.  */
	  del_range (PT! (), PT! () + 1);
	  goal_pt = PT! ();
	  goal_pt_byte = PT_BYTE! ();
	  Findent_to (make_fixnum (col), Qnil ());
	  SET_PT_BOTH (goal_pt, goal_pt_byte);

	  /* Set the last_known... vars consistently.  */
	  col = goal;
	}
    }

  /* If line ends prematurely, add space to the end.  */
  if col < goal && EQ (force, Qt ())
    { Findent_to (make_fixnum ({ col = goal; col }), Qnil ()); }

  last_known_column = col;
  last_known_column_point = PT! ();
  last_known_column_modified = MODIFF! ();

  return make_fixnum (col);
});

/* compute_motion: compute buffer posn given screen posn and vice versa */

static mut val_compute_motion: position = position {
  bufpos: 0, bytepos: 0, hpos: 0, vpos: 0, prevhpos: 0, contin: 0,
};

/* Scan the current buffer forward from offset FROM, pretending that
   this is at line FROMVPOS, column FROMHPOS, until reaching buffer
   offset TO or line TOVPOS, column TOHPOS (whichever comes first),
   and return the ending buffer position and screen location.  If we
   can't hit the requested column exactly (because of a tab or other
   multi-column character), overshoot.

   DID_MOTION is true if FROMHPOS has already accounted for overlay strings
   at FROM.  This is the case if FROMVPOS and FROMVPOS came from an
   earlier call to compute_motion.  The other common case is that FROMHPOS
   is zero and FROM is a position that "belongs" at column zero, but might
   be shifted by overlay strings; in this case DID_MOTION should be false.

   WIDTH is the number of columns available to display text;
   compute_motion uses this to handle continuation lines and such.
   If WIDTH is -1, use width of window's text area adjusted for
   continuation glyph when needed.

   HSCROLL is the number of columns not being displayed at the left
   margin; this is usually taken from a window's hscroll member.
   TAB_OFFSET is the number of columns of the first tab that aren't
   being displayed, perhaps because of a continuation line or
   something.

   compute_motion returns a pointer to a struct position.  The bufpos
   member gives the buffer position at the end of the scan, and hpos
   and vpos give its cartesian location.  prevhpos is the column at
   which the character before bufpos started, and contin is non-zero
   if we reached the current line by continuing the previous.

   Note that FROMHPOS and TOHPOS should be expressed in real screen
   columns, taking HSCROLL and the truncation glyph at the left margin
   into account.  That is, beginning-of-line moves you to the hpos
   -HSCROLL + (HSCROLL > 0).

   For example, to find the buffer position of column COL of line LINE
   of a certain window, pass the window's starting location as FROM
   and the window's upper-left coordinates as FROMVPOS and FROMHPOS.
   Pass the buffer's ZV as TO, to limit the scan to the end of the
   visible section of the buffer, and pass LINE and COL as TOVPOS and
   TOHPOS.

   When displaying in window w, a typical formula for WIDTH is:

	window_width - 1
	 - (has_vertical_scroll_bars
	    ? WINDOW_CONFIG_SCROLL_BAR_COLS (window)
	    : (window_width + window_left != frame_cols))

	where
	  window_width is w->total_cols,
	  window_left is w->left_col,
	  has_vertical_scroll_bars is
	    WINDOW_HAS_VERTICAL_SCROLL_BAR (window)
	  and frame_cols = FRAME_COLS (XFRAME (window->frame))

   Or you can let window_body_cols do this all for you, and write:
	window_body_cols (w) - 1

   The `-1' accounts for the continuation-line backslashes; the rest
   accounts for window borders if the window is split horizontally, and
   the scroll bars if they are turned on.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn compute_motion (from: ptrdiff_t, frombyte: ptrdiff_t,
					 fromvpos: EMACS_INT, fromhpos: EMACS_INT,
					 did_motion: bool, to: ptrdiff_t,
					 tovpos: EMACS_INT, tohpos: EMACS_INT,
					 width: EMACS_INT, hscroll: ptrdiff_t,
					 tab_offset: c_int, win: *mut window)
					 -> *mut position
{
  let mut did_motion: bool = did_motion;
  let mut width: EMACS_INT = width;
  let mut tab_offset: c_int = tab_offset;
  let mut hpos: EMACS_INT = fromhpos;
  let mut vpos: EMACS_INT = fromvpos;

  let mut pos: ptrdiff_t = 0;
  let mut pos_byte: ptrdiff_t = 0;
  let mut c: c_int = 0;
  let tab_width: c_int = SANE_TAB_WIDTH ((current_buffer! ()));
  let ctl_arrow: bool = !NILP (BVAR! ((current_buffer! ()), ctl_arrow_));
  let dp: *mut Lisp_Char_Table = window_display_table (win);
  let mut selective: EMACS_INT
    = if FIXNUMP (BVAR! ((current_buffer! ()), selective_display_))
      { XFIXNUM (BVAR! ((current_buffer! ()), selective_display_)) }
      else if !NILP (BVAR! ((current_buffer! ()), selective_display_)) { -1 } else { 0 };
  let mut selective_rlen: ptrdiff_t
    = if selective != 0 && !dp.is_null () && VECTORP (DISP_INVIS_VECTOR (dp))
      { ASIZE (DISP_INVIS_VECTOR (dp)) } else { 0 };
  /* The next location where the `invisible' property changes, or an
     overlay starts or ends.  */
  let mut next_boundary: ptrdiff_t = from;

  /* For computing runs of characters with similar widths.
     Invariant: width_run_width is zero, or all the characters
     from width_run_start to width_run_end have a fixed width of
     width_run_width.  */
  let mut width_run_start: ptrdiff_t = from;
  let mut width_run_end: ptrdiff_t = from;
  let mut width_run_width: ptrdiff_t = 0;
  let mut width_table: *mut Lisp_Object = std::ptr::null_mut ();

  /* The next buffer pos where we should consult the width run cache. */
  let mut next_width_run: ptrdiff_t = from;
  let mut window: Lisp_Object = Qnil ();

  let multibyte: bool = !NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_));
  /* If previous char scanned was a wide character,
     this is the column where it ended.  Otherwise, this is 0.  */
  let mut wide_column_end_hpos: EMACS_INT = 0;
  let mut prev_pos: ptrdiff_t = 0;	/* Previous buffer position.  */
  let mut prev_pos_byte: ptrdiff_t = 0;	/* Previous buffer position.  */
  let mut prev_hpos: EMACS_INT = 0;
  let mut prev_vpos: EMACS_INT = 0;
  let mut contin_hpos: EMACS_INT = 0;	/* HPOS of last column of continued line.  */
  let mut prev_tab_offset: c_int = 0;	/* Previous tab offset.  */
  let mut continuation_glyph_width: c_int = 0;
  let mut cache_buffer: *mut buffer = (current_buffer! ());
  let mut width_cache: *mut c_void = std::ptr::null_mut ();

  let mut cmp_it: composition_it = std::mem::zeroed ();

  XSETWINDOW! (window, win);

  if !(*cache_buffer).base_buffer.is_null ()
    { cache_buffer = (*cache_buffer).base_buffer; }
  if dp == buffer_display_table ()
    {
      width_table = if VECTORP (BVAR! ((current_buffer! ()), width_table_))
	{ (*XVECTOR (BVAR! ((current_buffer! ()), width_table_))).contents.as_mut_ptr () }
	else
	{ 0 as *mut Lisp_Object };
      if !width_table.is_null ()
	{ width_cache = width_run_cache_on_off (); }
    }
  else
    /* If the window has its own display table, we can't use the width
       run cache, because that's based on the buffer's display table.  */
    { width_table = 0 as *mut Lisp_Object; }

  /* Negative width means use all available text columns.  */
  if width < 0
    {
      width = window_body_width (win, window_body_unit::WINDOW_BODY_IN_CANONICAL_CHARS)
	      as EMACS_INT;
      /* We must make room for continuation marks if we don't have fringes.  */
      /* C 1334-1336's `#ifdef HAVE_WINDOW_SYSTEM` (`if (!FRAME_WINDOW_P (XFRAME
	 (win->frame)))`) is dead in the frozen configuration, so C's live text is the
	 unconditional `width -= 1;` below; docs/ports/indent.md records the dead arm (P-08).
	 */
      width -= 1;
    }

  continuation_glyph_width = 1;
  /* C 1341-1344's `#ifdef HAVE_WINDOW_SYSTEM` (`if (FRAME_WINDOW_P (XFRAME (win->frame)))
     continuation_glyph_width = 0;`) is dead in the frozen configuration; docs/ports/indent.md
     records it (P-08).  */

  /* It's just impossible to be too paranoid here.  */
  eassert! (from == BYTE_TO_CHAR (frombyte) && frombyte == CHAR_TO_BYTE (from));

  pos = from;
  prev_pos = from;
  pos_byte = frombyte;
  prev_pos_byte = frombyte;
  contin_hpos = 0;
  prev_tab_offset = tab_offset;
  memset (&raw mut cmp_it as *mut c_void, 0, std::mem::size_of::<composition_it> ());
  cmp_it.id = -1;
  composition_compute_stop_pos (&raw mut cmp_it as *mut c_void, pos, pos_byte, to,
				Qnil (), true);

  let mut quit_count: c_ushort = 0;

  /* C's `after_loop:` label is the epilogue below and the `goto after_loop` inside the
     invisible-text walk jumps to it (PORTING.md 2.2, the intervals.rs `'check_done`
     shape).  */
  'after_loop: {
  while true
    {
      quit_count = quit_count.wrapping_add (1);
      rarely_quit (quit_count as u64);

      while pos == next_boundary
	{
	  let pos_here: ptrdiff_t = pos;
	  let mut newpos: ptrdiff_t = 0;

	  /* Don't skip invisible if we are already at the margin.  */
	  if vpos > tovpos || (vpos == tovpos && hpos >= tohpos)
	    {
	      if contin_hpos != 0 && prev_hpos == 0
		  && hpos > tohpos
		  && (contin_hpos == width || wide_column_end_hpos > width)
		{ /* Line breaks because we can't put the character at the
		     previous line any more.  It is not the multi-column
		     character continued in middle.  Go back to previous
		     buffer position, screen position, and set tab offset
		     to previous value.  It's the beginning of the
		     line.  */
		  pos = prev_pos;
		  pos_byte = prev_pos_byte;
		  hpos = prev_hpos;
		  vpos = prev_vpos;
		  tab_offset = prev_tab_offset;
		}
	      break;
	    }

	  /* If the caller says that the screen position came from an earlier
	     call to compute_motion, then we've already accounted for the
	     overlay strings at point.  This is only true the first time
	     through, so clear the flag after testing it.  */
	  if !did_motion
	    /* We need to skip past the overlay strings.  Currently those
	       strings must not contain TAB;
	       if we want to relax that restriction, something will have
	       to be changed here.  */
	    {
	      let mut ovstr: *mut u8 = std::ptr::null_mut ();
	      let ovlen: ptrdiff_t = overlay_strings (pos, win as *mut c_void,
						      &raw mut ovstr);
	      hpos += if multibyte && ovlen > 0
		      { strwidth (ovstr as *const c_char, ovlen) } else { ovlen };
	    }
	  did_motion = false;

	  if pos >= to
	    { break; }

	  /* Advance POS past invisible characters
	     (but not necessarily all that there are here),
	     and store in next_boundary the next position where
	     we need to call skip_invisible.  */
	  newpos = skip_invisible (pos, &raw mut next_boundary, to, window);

	  if newpos >= to
	    {
	      pos = min (to, newpos);
	      pos_byte = CHAR_TO_BYTE (pos);
	      break 'after_loop;
	    }

	  if newpos != pos_here
	    {
	      pos = newpos;
	      pos_byte = CHAR_TO_BYTE (pos);
	    }
	  if newpos > cmp_it.stop_pos && cmp_it.id < 0
	    { composition_reseat_it (&raw mut cmp_it as *mut c_void, pos, pos_byte, to,
				     win as *mut c_void, -1 as i8,
				     std::ptr::null_mut (), Qnil ()); }

	  quit_count = quit_count.wrapping_add (1);
	  rarely_quit (quit_count as u64);
	}

      /* Handle right margin.  */
      /* Note on a wide-column character.

	 Characters are classified into the following three categories
	 according to the width (columns occupied on screen).

	 (1) single-column character: ex. `a'
	 (2) multi-column character: ex. `^A', TAB, `\033'
	 (3) wide-column character: ex. Japanese character, Chinese character
	     (In the following example, `W_' stands for them.)

	 Multi-column characters can be divided around the right margin,
	 but wide-column characters cannot.

	 NOTE:

	 (*) The cursor is placed on the next character after the point.

	     ----------
	     abcdefghi\
	     j        ^---- next after the point
	     ^---  next char. after the point.
	     ----------
	              In case of single-column character

	     ----------
	     abcdefgh\\
	     033     ^----  next after the point, next char. after the point.
	     ----------
	              In case of multi-column character

	     ----------
	     abcdefgh\\
	     W_      ^---- next after the point
	     ^----  next char. after the point.
	     ----------
	              In case of wide-column character

	 The problem here is continuation at a wide-column character.
	 In this case, the line may shorter less than WIDTH.
	 And we find the continuation AFTER it occurs.

       */

      if hpos > width
	{
	  let total_width: EMACS_INT = width + continuation_glyph_width as EMACS_INT;
	  let mut truncate: bool = false;

	  if !NILP (Vtruncate_partial_width_windows)
	      && (total_width < FRAME_COLS (XFRAME (WINDOW_FRAME! (win))) as EMACS_INT)
	    {
	      if FIXNUMP (Vtruncate_partial_width_windows)
		{ truncate
		    = total_width < XFIXNAT (Vtruncate_partial_width_windows); }
	      else
		{ truncate = true; }
	    }

	  if hscroll != 0 || truncate
	      || !NILP (BVAR! ((current_buffer! ()), truncate_lines_))
	    {
	      /* Truncating: skip to newline, unless we are already past
                 TO (we need to go back below).  */
	      if pos <= to
		{
		  pos = find_before_next_newline (pos, to, 1, &raw mut pos_byte);
		  hpos = width;
		  /* If we just skipped next_boundary,
		     loop around in the main while
		     and handle it.  */
		  if pos >= next_boundary
		    { next_boundary = pos + 1; }
		  prev_hpos = width;
		  prev_vpos = vpos;
		  prev_tab_offset = tab_offset;
		}
	    }
	  else
	    {
	      /* Continuing.  */
	      /* Remember the previous value.  */
	      prev_tab_offset = tab_offset;

	      if wide_column_end_hpos > width
		{
		  hpos -= prev_hpos;
		  tab_offset += prev_hpos as c_int;
		}
	      else
		{
		  tab_offset += width as c_int;
		  hpos -= width;
		}
	      vpos += 1;
	      contin_hpos = prev_hpos;
	      prev_hpos = 0;
	      prev_vpos = vpos;
	    }
	}

      /* Stop if past the target buffer position or screen position.  */
      if pos > to
	{
	  /* Go back to the previous position.  */
	  pos = prev_pos;
	  pos_byte = prev_pos_byte;
	  hpos = prev_hpos;
	  vpos = prev_vpos;
	  tab_offset = prev_tab_offset;

	  /* NOTE on contin_hpos, hpos, and prev_hpos.

	     ----------
	     abcdefgh\\
	     W_      ^----  contin_hpos
	     | ^-----  hpos
	     \---- prev_hpos
	     ----------
	   */

	  if contin_hpos != 0 && prev_hpos == 0
	      && contin_hpos < width && wide_column_end_hpos == 0
	    {
	      /* Line breaking occurs in the middle of multi-column
		 character.  Go back to previous line.  */
	      hpos = contin_hpos;
	      vpos = vpos - 1;
	    }
	  break;
	}

      if vpos > tovpos || (vpos == tovpos && hpos >= tohpos)
	{
	  if contin_hpos != 0 && prev_hpos == 0
	      && hpos > tohpos
	      && (contin_hpos == width || wide_column_end_hpos > width)
	    { /* Line breaks because we can't put the character at the
		 previous line any more.  It is not the multi-column
		 character continued in middle.  Go back to previous
		 buffer position, screen position, and set tab offset
		 to previous value.  It's the beginning of the
		 line.  */
	      pos = prev_pos;
	      pos_byte = prev_pos_byte;
	      hpos = prev_hpos;
	      vpos = prev_vpos;
	      tab_offset = prev_tab_offset;
	    }
	  break;
	}
      if pos == ZV! () /* We cannot go beyond ZV.  Stop here. */
	{ break; }

      prev_hpos = hpos;
      prev_vpos = vpos;
      prev_pos = pos;
      prev_pos_byte = pos_byte;
      wide_column_end_hpos = 0;

      /* Consult the width run cache to see if we can avoid inspecting
         the text character-by-character.  */
      if !width_cache.is_null () && pos >= next_width_run
        {
          let mut run_end: ptrdiff_t = 0;
          let common_width: c_int
            = region_cache_forward (cache_buffer, width_cache, pos, &raw mut run_end);

          /* A width of zero means the character's width varies (like
             a tab), is meaningless (like a newline), or we just don't
             want to skip over it for some other reason.  */
          if common_width != 0
            {
              let mut run_end_hpos: ptrdiff_t = 0;

              /* Don't go past the final buffer posn the user
                 requested.  */
              if run_end > to
                { run_end = to; }

              run_end_hpos = hpos + (run_end - pos) * common_width as ptrdiff_t;

              /* Don't go past the final horizontal position the user
                 requested.  */
              if vpos == tovpos && run_end_hpos > tohpos
                {
                  run_end      = pos + (tohpos - hpos) / common_width as ptrdiff_t;
                  run_end_hpos = hpos + (run_end - pos) * common_width as ptrdiff_t;
                }

              /* Don't go past the margin.  */
              if run_end_hpos >= width
                {
                  run_end      = pos + (width  - hpos) / common_width as ptrdiff_t;
                  run_end_hpos = hpos + (run_end - pos) * common_width as ptrdiff_t;
                }

              hpos = run_end_hpos;
              if run_end > pos
                { prev_hpos = hpos - common_width as EMACS_INT; }
	      if pos != run_end
		{
		  pos = run_end;
		  pos_byte = CHAR_TO_BYTE (pos);
		}
            }

          next_width_run = run_end + 1;
        }

      /* We have to scan the text character-by-character.  */
      else
	{
	  let mut i: ptrdiff_t = 0;
	  let mut n: ptrdiff_t = 0;
	  let mut charvec: Lisp_Object = Qnil ();

	  /* Check composition sequence.  */
	  if cmp_it.id >= 0
	      || (pos == cmp_it.stop_pos
		  && composition_reseat_it (&raw mut cmp_it as *mut c_void, pos, pos_byte,
					    to, win as *mut c_void, -1 as i8,
					    std::ptr::null_mut (), Qnil ()))
	    { composition_update_it (&raw mut cmp_it as *mut c_void, pos, pos_byte,
				      Qnil ()); }
	  if cmp_it.id >= 0
	    {
	      pos += cmp_it.nchars as ptrdiff_t;
	      pos_byte += cmp_it.nbytes as ptrdiff_t;
	      hpos += cmp_it.width as EMACS_INT;
	      if cmp_it.to == cmp_it.nglyphs
		{
		  cmp_it.id = -1;
		  composition_compute_stop_pos (&raw mut cmp_it as *mut c_void, pos,
						pos_byte, to, Qnil (), true);
		}
	      else
		{ cmp_it.from = cmp_it.to; }
	      continue;
	    }

	  c = FETCH_BYTE! (pos_byte) as c_int;
	  pos += 1;
	  pos_byte += 1;

	  /* Perhaps add some info to the width_run_cache.  */
	  if !width_cache.is_null ()
	    {
	      /* Is this character part of the current run?  If so, extend
		 the run.  */
	      if pos - 1 == width_run_end
		  && XFIXNAT (*width_table.offset (c as isize)) == width_run_width
		{ width_run_end = pos; }

	      /* The previous run is over, since this is a character at a
		 different position, or a different width.  */
	      else
		{
		  /* Have we accumulated a run to put in the cache?
		     (Currently, we only cache runs of width == 1).  */
		  if width_run_start < width_run_end
		      && width_run_width == 1
		    { know_region_cache (cache_buffer, width_cache,
					 width_run_start, width_run_end); }

		  /* Start recording a new width run.  */
		  width_run_width = XFIXNAT (*width_table.offset (c as isize));
		  width_run_start = pos - 1;
		  width_run_end = pos;
		}
	    }

	  if !dp.is_null ()
	      && ! (multibyte && LEADING_CODE_P (c))
	      && VECTORP (DISP_CHAR_VECTOR (dp, c))
	    {
	      charvec = DISP_CHAR_VECTOR (dp, c);
	      n = ASIZE (charvec);
	    }
	  else
	    {
	      charvec = Qnil ();
	      n = 1;
	    }

	  i = 0;
	  while i < n
	    {
	      if VECTORP (charvec)
		{
		  /* This should be handled the same as
		     next_element_from_display_vector does it.  */
		  let entry: Lisp_Object = AREF (charvec, i);

		  if GLYPH_CODE_P (entry)
		    { c = GLYPH_CODE_CHAR (entry); }
		  else
		    { c = b' ' as c_int; }
		}

	      if c >= 0o40 && c < 0o177
		{ hpos += 1; }
	      else if c == b'\t' as c_int
		{
		  let mut tem: c_int = ((hpos + tab_offset as EMACS_INT + hscroll
					 - (hscroll > 0) as EMACS_INT)
					% tab_width as EMACS_INT) as c_int;
		  if tem < 0
		    { tem += tab_width; }
		  hpos += tab_width as EMACS_INT - tem as EMACS_INT;
		}
	      else if c == b'\n' as c_int
		{
		  if selective > 0
		      && indented_beyond_p (pos, pos_byte, selective)
		    {
		      /* If (pos == to), we don't have to take care of
			 selective display.  */
		      if pos < to
			{
			  /* Skip any number of invisible lines all at once */
			  loop
			    {
			      pos = find_before_next_newline (pos, to, 1,
							      &raw mut pos_byte);
			      if pos < to
				{ inc_both (&raw mut pos, &raw mut pos_byte); }
			      quit_count = quit_count.wrapping_add (1);
			      rarely_quit (quit_count as u64);
			      if !(pos < to
				   && indented_beyond_p (pos, pos_byte, selective))
				{ break; }
			    }
			  /* Allow for the " ..." that is displayed for them. */
			  if selective_rlen != 0
			    {
			      hpos += selective_rlen;
			      if hpos >= width
				{ hpos = width; }
			    }
			  dec_both (&raw mut pos, &raw mut pos_byte);
			  /* We have skipped the invis text, but not the
			     newline after.  */
			}
		    }
		  else
		    {
		      /* A visible line.  */
		      vpos += 1;
		      hpos = 0;
		      hpos -= hscroll as EMACS_INT;
		      /* Count the truncation glyph on column 0 */
		      if hscroll > 0
			{ hpos += continuation_glyph_width as EMACS_INT; }
		      tab_offset = 0;
		    }
		  contin_hpos = 0;
		}
	      else if c == CR && selective < 0
		{
		  /* In selective display mode,
		     everything from a ^M to the end of the line is invisible.
		     Stop *before* the real newline.  */
		  if pos < to
		    { pos = find_before_next_newline (pos, to, 1, &raw mut pos_byte); }
		  /* If we just skipped next_boundary,
		     loop around in the main while
		     and handle it.  */
		  if pos > next_boundary
		    { next_boundary = pos; }
		  /* Allow for the " ..." that is displayed for them. */
		  if selective_rlen != 0
		    {
		      hpos += selective_rlen;
		      if hpos >= width
			{ hpos = width; }
		    }
		}
	      else if multibyte && LEADING_CODE_P (c)
		{
		  /* Start of multi-byte form.  */
		  let mut ptr: *mut u8 = std::ptr::null_mut ();
		  let mut mb_bytes: c_int = 0;
		  let mut mb_width: c_int = 0;

		  pos_byte -= 1;	/* rewind POS_BYTE */
		  ptr = BYTE_POS_ADDR (pos_byte);
		  MULTIBYTE_BYTES_WIDTH! (ptr, dp, mb_bytes, mb_width);
		  pos_byte += mb_bytes as ptrdiff_t;
		  if mb_width > 1 && BYTES_BY_CHAR_HEAD (*ptr as c_int) == mb_bytes
		    { wide_column_end_hpos = hpos + mb_width as EMACS_INT; }
		  hpos += mb_width as EMACS_INT;
		}
	      else if VECTORP (charvec)
		{ hpos += 1; }
	      else
		{ hpos += if ctl_arrow && c < 0o200 { 2 } else { 4 }; }
	      i += 1;
	    }
	}
    }
  }

  /* Remember any final width run in the cache.  */
  if !width_cache.is_null ()
      && width_run_width == 1
      && width_run_start < width_run_end
    { know_region_cache (cache_buffer, width_cache, width_run_start, width_run_end); }

  val_compute_motion.bufpos = pos;
  val_compute_motion.bytepos = pos_byte;
  val_compute_motion.hpos = hpos;
  val_compute_motion.vpos = vpos;
  if contin_hpos != 0 && prev_hpos == 0
    { val_compute_motion.prevhpos = contin_hpos; }
  else
    { val_compute_motion.prevhpos = prev_hpos; }

  /* Nonzero if have just continued a line */
  val_compute_motion.contin = (contin_hpos != 0 && prev_hpos == 0) as c_int;

  return &raw mut val_compute_motion;
}

DEFUN! ("compute-motion", Fcompute_motion, Scompute_motion, 7, 7, 0,
       doc: "Scan through the current buffer, calculating screen position.\nScan the current buffer forward from offset FROM,\nassuming it is at position FROMPOS--a cons of the form (HPOS . VPOS)--\nto position TO or position TOPOS--another cons of the form (HPOS . VPOS)--\nand return the ending buffer position and screen location.\n\nIf TOPOS is nil, the actual width and height of the window's\ntext area are used.\n\nThere are three additional arguments:\n\nWIDTH is the number of columns available to display text;\nthis affects handling of continuation lines.  A value of nil\ncorresponds to the actual number of available text columns.\n\nOFFSETS is either nil or a cons cell (HSCROLL . TAB-OFFSET).\nHSCROLL is the number of columns not being displayed at the left\nmargin; this is usually taken from a window's hscroll member.\nTAB-OFFSET is the number of columns of the first tab that aren't\nbeing displayed, perhaps because the line was continued within it.\nIf OFFSETS is nil, HSCROLL and TAB-OFFSET are assumed to be zero.\n\nWINDOW is the window to operate on.  It is used to choose the display table;\nif it is showing the current buffer, it is used also for\ndeciding which overlay properties apply.\nNote that `compute-motion' always operates on the current buffer.\n\nThe value is a list of five elements:\n  (POS HPOS VPOS PREVHPOS CONTIN)\nPOS is the buffer position where the scan stopped.\nVPOS is the vertical position where the scan stopped.\nHPOS is the horizontal position where the scan stopped.\n\nPREVHPOS is the horizontal position one character back from POS.\nCONTIN is t if a line was continued after (or within) the previous character.\n\nFor example, to find the buffer position of column COL of line LINE\nof a certain window, pass the window's starting location as FROM\nand the window's upper-left coordinates as FROMPOS.\nPass the buffer's (point-max) as TO, to limit the scan to the end of the\nvisible section of the buffer, and pass LINE and COL as TOPOS.  ",
       (from: Lisp_Object, frompos: Lisp_Object, to: Lisp_Object, topos: Lisp_Object,
        width: Lisp_Object, offsets: Lisp_Object, window: Lisp_Object),
       {
  /* C's `CHECK_FIXNUM_COERCE_MARKER` assigns to its argument, so the two parameters are
     shadowed mutably (PORTING.md 1.5, the marker.rs `Fmarkers_in` shape).  */
  let mut from: Lisp_Object = from;
  let mut to: Lisp_Object = to;
  let mut w: *mut window = std::ptr::null_mut ();
  let mut bufpos: Lisp_Object = Qnil ();
  let mut hpos: Lisp_Object = Qnil ();
  let mut vpos: Lisp_Object = Qnil ();
  let mut prevhpos: Lisp_Object = Qnil ();
  let mut pos: *mut position = std::ptr::null_mut ();
  let mut hscroll: ptrdiff_t = 0;
  let mut tab_offset: c_int = 0;

  CHECK_FIXNUM_COERCE_MARKER! (from);
  CHECK_CONS (frompos);
  CHECK_FIXNUM (XCAR (frompos));
  CHECK_FIXNUM (XCDR (frompos));
  CHECK_FIXNUM_COERCE_MARKER! (to);
  if !NILP (topos)
    {
      CHECK_CONS (topos);
      CHECK_FIXNUM (XCAR (topos));
      CHECK_FIXNUM (XCDR (topos));
    }
  if !NILP (width)
    { CHECK_FIXNUM (width); }

  if !NILP (offsets)
    {
      CHECK_CONS (offsets);
      CHECK_FIXNUM (XCAR (offsets));
      CHECK_FIXNUM (XCDR (offsets));
      if ! (0 <= XFIXNUM (XCAR (offsets)) && XFIXNUM (XCAR (offsets)) <= PTRDIFF_MAX
	     && 0 <= XFIXNUM (XCDR (offsets)) && XFIXNUM (XCDR (offsets)) <= INT_MAX as isize)
	{ args_out_of_range (XCAR (offsets), XCDR (offsets)); }
      hscroll = XFIXNUM (XCAR (offsets));
      tab_offset = XFIXNUM (XCDR (offsets)) as c_int;
    }
  else
    { tab_offset = 0; hscroll = 0; }

  w = decode_live_window (window);

  if XFIXNUM (from) < BEGV! () || XFIXNUM (from) > ZV! ()
    { args_out_of_range_3 (from, make_fixnum (BEGV! ()), make_fixnum (ZV! ())); }
  if XFIXNUM (to) < BEGV! () || XFIXNUM (to) > ZV! ()
    { args_out_of_range_3 (to, make_fixnum (BEGV! ()), make_fixnum (ZV! ())); }

  pos = compute_motion (XFIXNUM (from), CHAR_TO_BYTE (XFIXNUM (from)),
			XFIXNUM (XCDR (frompos)),
			XFIXNUM (XCAR (frompos)), false,
			XFIXNUM (to),
			if NILP (topos)
			 { window_internal_height (w) as EMACS_INT }
			 else
			 { XFIXNUM (XCDR (topos)) },
			if NILP (topos)
			 { window_body_width (w,
					      window_body_unit::WINDOW_BODY_IN_CANONICAL_CHARS)
			   as EMACS_INT - (1) }
			 else
			 { XFIXNUM (XCAR (topos)) },
			if NILP (width) { -1 } else { XFIXNUM (width) },
			hscroll, tab_offset, w);

  XSETFASTINT! (bufpos, (*pos).bufpos);
  XSETINT! (hpos, (*pos).hpos);
  XSETINT! (vpos, (*pos).vpos);
  XSETINT! (prevhpos, (*pos).prevhpos);

  return list5 (bufpos, hpos, vpos, prevhpos,
		if (*pos).contin != 0 { Qt () } else { Qnil () });
});

/* Fvertical_motion and vmotion.  */

static mut val_vmotion: position = position {
  bufpos: 0, bytepos: 0, hpos: 0, vpos: 0, prevhpos: 0, contin: 0,
};

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn vmotion (from: ptrdiff_t, from_byte: ptrdiff_t,
				  vtarget: EMACS_INT, w: *mut window) -> *mut position
{
  let mut from: ptrdiff_t = from;
  let mut from_byte: ptrdiff_t = from_byte;
  let hscroll: ptrdiff_t = (*w).hscroll;
  let mut pos: position = std::mem::zeroed ();
  /* VPOS is cumulative vertical position, changed as from is changed.  */
  let mut vpos: EMACS_INT = 0;
  let mut prevline: ptrdiff_t = 0;
  let mut first: ptrdiff_t = 0;
  let lmargin: ptrdiff_t = if hscroll > 0 { 1 - hscroll } else { 0 };
  let selective: ptrdiff_t
    = if FIXNUMP (BVAR! ((current_buffer! ()), selective_display_))
      { clip_to_bounds (-1, XFIXNUM (BVAR! ((current_buffer! ()), selective_display_)),
			PTRDIFF_MAX) }
      else if !NILP (BVAR! ((current_buffer! ()), selective_display_)) { -1 } else { 0 };
  let mut window: Lisp_Object = Qnil ();
  let mut did_motion: bool = false;
  /* This is the object we use for fetching character properties.  */
  let mut text_prop_object: Lisp_Object = Qnil ();

  XSETWINDOW! (window, w);

  /* If the window contains this buffer, use it for getting text properties.
     Otherwise use the current buffer as arg for doing that.  */
  if BASE_EQ ((*w).contents, Fcurrent_buffer ())
    { text_prop_object = window; }
  else
    { text_prop_object = Fcurrent_buffer (); }

  if vpos >= vtarget
    {
      /* To move upward, go a line at a time until
	 we have gone at least far enough.  */

      first = 1;

      while (vpos > vtarget || first != 0) && from > BEGV! ()
	{
	  let mut bytepos: ptrdiff_t = from_byte;
	  let mut propval: Lisp_Object = Qnil ();

	  prevline = from;
	  dec_both (&raw mut prevline, &raw mut bytepos);
	  prevline = find_newline_no_quit (prevline, bytepos, -1, &raw mut bytepos);

	  while prevline > BEGV! ()
		 && ((selective > 0
		      && indented_beyond_p (prevline, bytepos, selective))
		     /* Watch out for newlines with `invisible' property.
			When moving upward, check the newline before.  */
		     || { propval = Fget_char_property (make_fixnum (prevline - 1),
							Qinvisible (),
							text_prop_object);
			  TEXT_PROP_MEANS_INVISIBLE! (propval) != 0 })
	    {
	      dec_both (&raw mut prevline, &raw mut bytepos);
	      prevline = find_newline_no_quit (prevline, bytepos, -1, &raw mut bytepos);
	    }
	  pos = *compute_motion (prevline, bytepos, 0, lmargin, false, from,
				 /* Don't care for VPOS...  */
				 (1 << (SHRT_WIDTH - 1)) as EMACS_INT,
				 /* ... nor HPOS.  */
				 (1 << (SHRT_WIDTH - 1)) as EMACS_INT,
				 -1, hscroll, 0, w);
	  vpos -= pos.vpos;
	  first = 0;
	  from = prevline;
	  from_byte = bytepos;
	}

      /* If we made exactly the desired vertical distance, or
	 if we hit beginning of buffer, return point found.  */
      if vpos >= vtarget
	{
	  val_vmotion.bufpos = from;
	  val_vmotion.bytepos = from_byte;
	  val_vmotion.vpos = vpos;
	  val_vmotion.hpos = lmargin;
	  val_vmotion.contin = 0;
	  val_vmotion.prevhpos = 0;
	  return &raw mut val_vmotion;
	}

      /* Otherwise find the correct spot by moving down.  */
    }

  /* Moving downward is simple, but must calculate from
     beg of line to determine hpos of starting point.  */

  if from > BEGV! () && FETCH_BYTE! (from_byte - 1) != b'\n'
    {
      let mut bytepos: ptrdiff_t = 0;
      let mut propval: Lisp_Object = Qnil ();

      prevline = find_newline_no_quit (from, from_byte, -1, &raw mut bytepos);
      while prevline > BEGV! ()
	     && ((selective > 0
		  && indented_beyond_p (prevline, bytepos, selective))
		 /* Watch out for newlines with `invisible' property.
		    When moving downward, check the newline after.  */
		 || { propval = Fget_char_property (make_fixnum (prevline),
						   Qinvisible (),
						   text_prop_object);
		      TEXT_PROP_MEANS_INVISIBLE! (propval) != 0 })
	{
	  dec_both (&raw mut prevline, &raw mut bytepos);
	  prevline = find_newline_no_quit (prevline, bytepos, -1, &raw mut bytepos);
	}
      pos = *compute_motion (prevline, bytepos, 0, lmargin, false, from,
			     /* Don't care for VPOS...  */
			     (1 << (SHRT_WIDTH - 1)) as EMACS_INT,
			     /* ... nor HPOS.  */
			     (1 << (SHRT_WIDTH - 1)) as EMACS_INT,
			     -1, hscroll, 0, w);
      did_motion = true;
    }
  else
    {
      pos.hpos = lmargin;
      pos.vpos = 0;
      did_motion = false;
    }
  return compute_motion (from, from_byte, vpos, pos.hpos, did_motion,
			 ZV! (), vtarget, - ((1 << (SHRT_WIDTH - 1)) as EMACS_INT),
			 -1, hscroll, 0, w);
}

/* Return the width taken by line-number display in window W.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn line_number_display_width (w: *mut window, width: *mut c_int,
						    pixel_width: *mut c_int)
{
  if NILP (Vdisplay_line_numbers)
    {
      *width = 0;
      *pixel_width = 0;
    }
  else
    {
      let mut it: it = std::mem::zeroed ();
      let mut startpos: text_pos = std::mem::zeroed ();
      let mut saved_restriction: bool = false;
      let old_buf: *mut buffer = (current_buffer! ());
      let count: specpdl_ref = SPECPDL_INDEX ();
      SET_TEXT_POS_FROM_MARKER! (startpos, (*w).start);
      let itdata: *mut c_void = bidi_shelve_cache ();

      /* Make sure W's buffer is the current one.  */
      set_buffer_internal_1 (XBUFFER ((*w).contents));
      /* We want to start from window's start point, but it could be
	 outside the accessible region, in which case we widen the
	 buffer temporarily.  It could even be beyond the buffer's end
	 (Org mode's display of source code snippets is known to cause
	 that) or belong to the wrong buffer, in which cases we just
	 punt and start from point instead.  */
      if startpos.charpos > Z! ()
	  || !(BUFFERP ((*w).contents)
	       && XBUFFER ((*w).contents) == (*XMARKER ((*w).start)).buffer as *mut buffer)
	{ SET_TEXT_POS! (startpos, PT! (), PT_BYTE! ()); }
      if startpos.charpos < BEGV! () || startpos.charpos > ZV! ()
	{
	  record_unwind_protect (Some (save_restriction_restore),
				 save_restriction_save ());
	  labeled_restrictions_remove_in_current_buffer ();
	  Fwiden ();
	  saved_restriction = true;
	}
      start_display (&raw mut it, w, startpos);
      /* The call to move_it_by_lines below will not generate a line
	 number if the first line shown in the window is hscrolled
	 such that all of its display elements are out of view.  So we
	 pretend the hscroll doesn't exist.  */
      it.first_visible_x = 0;
      move_it_by_lines (&raw mut it, 1);
      *width = it.lnum_width;
      *pixel_width = it.lnum_pixel_width;
      if saved_restriction
	{ unbind_to (count, Qnil ()); }
      set_buffer_internal_1 (old_buf);
      bidi_unshelve_cache (itdata, false);
    }
}

DEFUN! ("line-number-display-width", Fline_number_display_width,
       Sline_number_display_width, 0, 1, 0,
       doc: "Return the width used for displaying line numbers in the selected window.\nIf optional argument PIXELWISE is the symbol `columns', return the width\nin units of the frame's canonical character width.  In this case, the\nvalue is a float.\nIf optional argument PIXELWISE is t or any other non-nil value, return\nthe width as an integer number of pixels.\nOtherwise return the value as an integer number of columns of the face\nused to display line numbers, `line-number'.  Note that in the latter\ncase, the value doesn't include the 2 columns used for padding the\nnumbers on display.  ",
       (pixelwise: Lisp_Object),
       {
  let mut width: c_int = 0;
  let mut pixel_width: c_int = 0;
  let w: *mut window = XWINDOW (selected_window);
  line_number_display_width (w, &raw mut width, &raw mut pixel_width);
  if EQ (pixelwise, Qcolumns ())
    {
      let f: *mut frame = XFRAME ((*w).frame);
      return make_float ((pixel_width as c_double) / FRAME_COLUMN_WIDTH (f) as c_double);
    }
  else if !NILP (pixelwise)
    { return make_fixnum (pixel_width as EMACS_INT); }
  return make_fixnum (width as EMACS_INT);
});

/* In window W (derived from WINDOW), return x coordinate for column
   COL (derived from COLUMN).  */
unsafe fn window_column_x (w: *mut window, window: Lisp_Object,
			   col: c_double, column: Lisp_Object) -> c_int
{
  let x: c_double = col * FRAME_COLUMN_WIDTH (XFRAME ((*w).frame)) as c_double + 0.5;

  /* FIXME: Should this be limited to W's dimensions?  */
  if ! (INT_MIN as c_double <= x && x <= INT_MAX as c_double)
    { args_out_of_range (window, column); }

  return x as c_int;
}

/* Restore window's buffer and point.  */

/* FIXME: Merge with `with_echo_area_buffer_unwind_data`?  */
/// C's prototype is `static void restore_window_buffer (Lisp_Object);` and the address is
/// taken at both `record_unwind_protect` call sites, so the port's helper is
/// `unsafe extern "C" fn` (PORTING.md 1.4: one ABI, the `floatfns.c` precedent).
unsafe extern "C-unwind" fn restore_window_buffer (list: Lisp_Object)
{
  let mut list: Lisp_Object = list;
  let w: *mut window = decode_live_window (XCAR (list));
  list = XCDR (list);
  wset_buffer (w, XCAR (list));
  list = XCDR (list);
  set_marker_both ((*w).pointm, (*w).contents,
		   XFIXNAT (XCAR (list)),
		   XFIXNAT (XCAR (XCDR (list))));
}

DEFUN! ("vertical-motion", Fvertical_motion, Svertical_motion, 1, 3, 0,
       doc: "Move point to start of the screen line LINES lines down.\nIf LINES is negative, this means moving up.\n\nThis function is an ordinary cursor motion function\nwhich calculates the new position based on how text would be displayed.\nThe new position may be the start of a line,\nor the start of a continuation line,\nor the start of the visible portion of a horizontally-scrolled line.\n\nThe function returns number of screen lines moved over;\nthat usually equals LINES, but may be closer to zero if\nbeginning or end of buffer was reached.\n\nThe optional second argument WINDOW specifies the window to use for\nparameters such as width, horizontal scrolling, and so on.\nThe default is to use the selected window's parameters.\n\nIf LINES is zero, point will move to the first visible character on\nthe current screen line.\n\nLINES can optionally take the form (COLS . LINES), in which case the\nmotion will stop at the COLSth column from the visual start of the\nline (if such column exists on that line, that is).  If the line is\nscrolled horizontally, COLS is interpreted visually, i.e., as addition\nto the columns of text beyond the left edge of the window.\nIf LINES is a cons cell, its car COLS can be a float, which allows\nspecifying an accurate position of point on a screen line that mixes\nfonts or uses variable-pitch font: COLS is interpreted in units of the\ncanonical character width, and is internally converted to pixel units;\npoint will then stop at the position closest to that pixel coordinate.\nThe cdr of the cons, LINES, must be an integer; if it is zero, this\nfunction moves point horizontally in the current screen line, to the\nposition specified by COLS.\n\nThe optional third argument CUR-COL specifies the horizontal\nwindow-relative coordinate of point, in units of frame's canonical\ncharacter width, where the function is invoked.  If this argument is\nomitted or nil, the function will determine the point coordinate by\ngoing back to the beginning of the line.\n\n`vertical-motion' always uses the current buffer, regardless of which\nbuffer is displayed in WINDOW.  This is consistent with other cursor\nmotion functions and makes it possible to use `vertical-motion' in any\nbuffer, whether or not it is currently displayed in some window.  ",
       (lines: Lisp_Object, window: Lisp_Object, cur_col: Lisp_Object),
       {
  let mut lines: Lisp_Object = lines;
  let mut it: it = std::mem::zeroed ();
  let mut pt: text_pos = std::mem::zeroed ();
  let mut w: *mut window = std::ptr::null_mut ();
  let mut lcols: Lisp_Object = Qnil ();
  let mut itdata: *mut c_void = std::ptr::null_mut ();
  let count: specpdl_ref = SPECPDL_INDEX ();

  /* Allow LINES to be of the form (HPOS . VPOS) aka (COLUMNS . LINES).  */
  if CONSP (lines)
    {
      lcols = XCAR (lines);
      CHECK_NUMBER (lcols);
      lines = XCDR (lines);
    }

  CHECK_FIXNUM (lines);
  w = decode_live_window (window);

  if XBUFFER ((*w).contents) != (current_buffer! ())
    {
      /* Set the window's buffer temporarily to the current buffer.  */
      let old: Lisp_Object = list4 (window, (*w).contents,
				    make_fixnum (marker_position ((*w).pointm)),
				    make_fixnum (marker_byte_position ((*w).pointm)));
      record_unwind_protect (Some (restore_window_buffer), old);
      wset_buffer (w, Fcurrent_buffer ());
      set_marker_both ((*w).pointm, (*w).contents,
		       BUF_PT ((current_buffer! ())), BUF_PT_BYTE ((current_buffer! ())));
    }

  if noninteractive
    {
      let mut pos: position = std::mem::zeroed ();
      pos = *vmotion (PT! (), PT_BYTE! (), XFIXNUM (lines), w);
      SET_PT_BOTH (pos.bufpos, pos.bytepos);
      it.vpos = pos.vpos as c_int;
    }
  else
    {
      let mut it_start: ptrdiff_t = 0;
      let mut it_overshoot_count: ptrdiff_t = 0;
      let mut first_x: c_int = 0;
      let mut overshoot_handled: bool = false;
      let mut disp_string_at_start_p: bool = false;
      let mut nlines: ptrdiff_t = XFIXNUM (lines);
      let mut vpos_init: c_int = 0;
      let mut start_col: c_double = 0.0;
      let mut start_x: c_int = 0;
      let mut to_x: c_int = -1;
      let mut ovl_start: ptrdiff_t = -1;

      let start_x_given: bool = !NILP (cur_col);
      if start_x_given
	{
	  start_col = extract_float (cur_col);
	  start_x = window_column_x (w, window, start_col, cur_col);
	}

      /* When displaying line numbers, we need to prime IT's
	 lnum_width with the value calculated at window's start, since
	 that's what normal window redisplay does.  Otherwise C-n/C-p
	 will sometimes err by one column.  */
      let mut lnum_width: c_int = 0;
      let mut lnum_pixel_width: c_int = 0;
      if !NILP (Vdisplay_line_numbers)
	{ line_number_display_width (w, &raw mut lnum_width, &raw mut lnum_pixel_width); }
      SET_TEXT_POS! (pt, PT! (), PT_BYTE! ());
      itdata = bidi_shelve_cache ();
      record_unwind_protect_void (Some (unwind_display_working_on_window));
      display_working_on_window_p = true;
      start_display (&raw mut it, w, pt);
      it.lnum_width = lnum_width;
      first_x = it.first_visible_x;
      it_start = IT_CHARPOS! (it);

      /* See comments below for why we calculate this.  */
      if it.cmp_it.id >= 0
	{ it_overshoot_count = 0; }
      else if it.method == GET_FROM_STRING
	{
	  let mut s: *const c_char = SSDATA (it.string);
	  let e: *const c_char = s.offset (SBYTES (it.string));
	  let mut prop: Lisp_Object = Qnil ();
	  let ovl_idx: ptrdiff_t =
	    if it.current.overlay_string_index >= 0
	     { it.current.overlay_string_index % OVERLAY_STRING_CHUNK_SIZE as ptrdiff_t }
	     else
	     { -1 };

	  /* If this is a string from an overlay, compute where that
             overlay starts.  */
	  if !it_bits! (string_from_display_prop_p, it) && ovl_idx >= 0
	    { ovl_start = OVERLAY_START (it.string_overlays[ovl_idx as usize]); }

	  disp_string_at_start_p =
	  /* If it.area is anything but TEXT_AREA, we need not bother
	     about the display string, as it doesn't affect cursor
	     positioning.  */
	    it.area == TEXT_AREA
	    && (it_bits! (string_from_display_prop_p, it)
		/* Overlay string on invisible text has the same effect
                   on display and cursor movement as a display string.  */
		|| (ovl_start >= BEGV! ()
		    && { prop = Fget_char_property (make_fixnum (ovl_start),
						    Qinvisible (), window);
			 TEXT_PROP_MEANS_INVISIBLE! (prop) != 0 }))
	    /* A display string on anything but buffer text (e.g., on
	       an overlay string) doesn't affect cursor positioning.  */
	    && (it.sp > 0
		&& it.stack[(it.sp - 1) as usize].method == GET_FROM_BUFFER);
	  while s < e
	    {
	      let c: c_char = { let c = *s; s = s.offset (1); c };
	      if c == b'\n' as c_char
		{ it_overshoot_count += 1; }
	    }
	  if it_overshoot_count == 0
	    { it_overshoot_count = -1; }
	}
      else
	{ it_overshoot_count =
	  /* If image_id is negative, it's a fringe bitmap, which by
	     definition doesn't affect display in the text area.  */
	  !((it.method == GET_FROM_IMAGE && it.image_id >= 0)
	    || it.method == GET_FROM_STRETCH) as ptrdiff_t; }

      if start_x_given
	{
	  it.hpos = start_col as c_int;
	  it.current_x = start_x;
	}
      else
	{
	  /* Scan from the start of the line containing PT.  If we don't
	     do this, we start moving with IT->current_x == 0, while PT is
	     really at some x > 0.  */
	  reseat_at_previous_visible_line_start (&raw mut it);
	  it.hpos = 0;
	  it.current_x = 0;
	}
      if IT_CHARPOS! (it) != PT! ()
	/* We used to temporarily disable selective display here; the
	   comment said this is "so we don't move too far" (2005-01-19
	   checkin by kfs).  But this does nothing useful that I can
	   tell, and it causes Bug#2694 .  -- cyd */
	/* When the position we started from is covered by a display
	   string, move_it_to will overshoot it, while vertical-motion
	   wants to put the cursor _before_ the display string.  So in
	   that case, we move to buffer position before the display
	   string, and avoid overshooting.  But if the position before
	   the display string is a newline, we don't do this, because
	   otherwise we will end up in a screen line that is one too
	   far back.  */
	{ move_it_to (&raw mut it,
		      if !disp_string_at_start_p
			 || FETCH_BYTE! (IT_BYTEPOS! (it)) == b'\n'
		      { PT! () }
		      else
		      { PT! () - 1 },
		      -1, -1, -1, MOVE_TO_POS); }

      /* IT may move too far if truncate-lines is on and PT lies
	 beyond the right margin.  IT may also move too far if the
	 starting point is on a Lisp string that has embedded
	 newlines, or spans several screen lines.  In these cases,
	 backtrack.  */
      if IT_CHARPOS! (it) > it_start
	{
	  /* We need to backtrack also if the Lisp string contains no
	     newlines, but there is a newline right after it.  In this
	     case, IT overshoots if there is an after-string just
	     before the newline.  */
	  if it_overshoot_count < 0
	      && it.method == GET_FROM_BUFFER
	      && it.c == b'\n' as c_int
	    { it_overshoot_count = 1; }
	  else if it_overshoot_count == 1 && it.vpos == 0
		   && it.current_x < it.last_visible_x
	    {
	      /* If we came to the same screen line as the one where
		 we started, we didn't overshoot the line, and won't
		 need to backtrack after all.  This happens, for
		 example, when PT is in the middle of a composition.  */
	      it_overshoot_count = 0;
	    }
	  else if disp_string_at_start_p && it.vpos > 0
	    {
	      /* This is the case of a display string that spans
		 several screen lines.  In that case, we end up at the
		 end of the string, and it.vpos tells us how many
		 screen lines we need to backtrack.  */
	      it_overshoot_count = it.vpos as ptrdiff_t;
	    }
	  /* We might overshoot if lines are truncated and point lies
	     beyond the right margin of the window.  */
	  if it.line_wrap == TRUNCATE && it.current_x >= it.last_visible_x
	      && it_overshoot_count == 0 && it.vpos > 0
	    { it_overshoot_count = 1; }
	  if it_overshoot_count > 0
	    { move_it_by_lines (&raw mut it, -it_overshoot_count); }

	  overshoot_handled = true;
	}
      else if IT_CHARPOS! (it) == PT! () - 1
	       && FETCH_BYTE! (PT_BYTE! () - 1) == b'\n'
	       && nlines <= 0
	{
	  /* The position we started from was covered by a display
	     property, so we moved to position before the string, and
	     backed up one line, because the character at PT - 1 is
	     a newline.  So we need one less line to go up (or exactly
	     one line to go down if nlines == 0).  */
	  nlines += 1;
	  /* But we still need to record that one line, in order to
	     return the correct value to the caller.  */
	  vpos_init = -1;

	  overshoot_handled = true;
	}
      if !NILP (lcols)
	{ to_x =
	  window_column_x (w, window, XFLOATINT (lcols), lcols)
	  + lnum_pixel_width; }
      if nlines <= 0
	{
	  it.vpos = vpos_init;
	  it.current_y = 0;
	  /* Do this even if LINES is 0, so that we move back to the
	     beginning of the current line as we ought.  */
	  if (nlines < 0 && IT_CHARPOS! (it) > BEGV! ())
	      || (nlines == 0 && !(start_x_given && start_x <= to_x))
	    { move_it_by_lines (&raw mut it, max (PTRDIFF_MIN, nlines)); }
	  /* If we haven't moved due to an overlay string on invisible
             text, back up past that overlay string.  */
	  if IT_CHARPOS! (it) == it_start
	      && disp_string_at_start_p
	      && ovl_start >= BEGV! ()
	      && it_overshoot_count > 0
	    { move_it_by_lines (&raw mut it, -it_overshoot_count); }
	}
      else if overshoot_handled
	{
	  it.vpos = vpos_init;
	  it.current_y = 0;
	  move_it_by_lines (&raw mut it, min (PTRDIFF_MAX, nlines));
	}
      else
	{
	  /* Otherwise, we are at the first row occupied by PT, which
	     might span multiple screen lines (e.g., if it's on a
	     multi-line display string).  We want to start from the
	     last line that it occupies.  */
	  if it_start < ZV! ()
	    {
	      if if it.bidi_it.scan_dir >= 0 || it.vpos == vpos_init
		 { IT_CHARPOS! (it) < it_start }
		 else
		 { IT_CHARPOS! (it) > it_start }
		{
		  it.vpos = 0;
		  it.current_y = 0;
		  move_it_by_lines (&raw mut it, 1);
		}
	      while IT_CHARPOS! (it) == it_start
		{
		  it.vpos = 0;
		  it.current_y = 0;
		  move_it_by_lines (&raw mut it, 1);
		}
	      if nlines > 1
		{ move_it_by_lines (&raw mut it, min (PTRDIFF_MAX, nlines - 1)); }
	    }
	  else	/* it_start = ZV */
	    {
	      it.vpos = 0;
	      it.current_y = 0;
	      move_it_by_lines (&raw mut it, min (PTRDIFF_MAX, nlines));
	      /* We could have some display or overlay string at ZV,
		 in which case it.vpos will be nonzero now, while
		 actually we didn't move vertically at all.  */
	      if IT_CHARPOS! (it) == CHARPOS! (pt) && CHARPOS! (pt) == it_start
		{ it.vpos = 0; }
	    }
	}

      /* Move to the goal column, if one was specified.  If the window
	 was originally hscrolled, the goal column is interpreted as
	 an addition to the hscroll amount.  */
      if !NILP (lcols)
	{
	  /* Start at beginning of line if inside an overlay string, to
             avoid becoming stuck at the beginning of the overlay string.  */
	  if it.continuation_lines_width <= 0 /* not in continuation line */
	      && it.hpos > 0		       /* and not at BOL */
	      && it.method == GET_FROM_STRING && !NILP (it.from_overlay)
	    { reseat_at_previous_visible_line_start(&raw mut it); }

	  move_it_in_display_line (&raw mut it, ZV! (), first_x + to_x, MOVE_TO_X);
	  /* If we find ourselves in the middle of an overlay string
	     which includes a newline after current string position,
	     we need to move by lines until we get out of the string,
	     and then reposition point at the requested X coordinate;
	     if we don't, the cursor will be placed just after the
	     string, which might not be the requested column.  */
	  if nlines >= 0 && it.area == TEXT_AREA
	    {
	      while it.method == GET_FROM_STRING
		     && !it_bits! (string_from_display_prop_p, it)
		     && ! memchr (SSDATA (it.string).offset (IT_STRING_BYTEPOS! (it))
				  as *const c_void,
				  b'\n' as c_int,
				  (SBYTES (it.string) - IT_STRING_BYTEPOS! (it))
				  as usize).is_null ()
		{
		  move_it_by_lines (&raw mut it, 1);
		  move_it_in_display_line (&raw mut it, ZV! (), first_x + to_x,
					   MOVE_TO_X);
		}
	    }
	}

      SET_PT_BOTH (IT_CHARPOS! (it), IT_BYTEPOS! (it));
      bidi_unshelve_cache (itdata, false);
    }

  return unbind_to (count, make_fixnum (it.vpos as EMACS_INT));
});

/* File's initialization.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn syms_of_indent ()
{
  DEFVAR_BOOL! (c"indent-tabs-mode", indent_tabs_mode,
	       "Indentation can insert tabs if this is non-nil.  ");
  indent_tabs_mode = true;

  DEFSYM! (Qcolumns, c"columns");

  defsubr (&raw mut Scurrent_indentation);
  defsubr (&raw mut Sindent_to);
  defsubr (&raw mut Scurrent_column);
  defsubr (&raw mut Smove_to_column);
  defsubr (&raw mut Sline_number_display_width);
  defsubr (&raw mut Svertical_motion);
  defsubr (&raw mut Scompute_motion);
}
