// femacs: port of src/bidi.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// macro, struct, struct field, file-scope object, local), same control flow, same error
// model.  The eleven non-`static` definitions (`bidi_mirror_char`, `bidi_level_start`,
// `bidi_push_it`, `bidi_pop_it`, `bidi_shelve_cache`, `bidi_unshelve_cache`,
// `bidi_init_it`, `bidi_paragraph_init`, `bidi_move_to_visually_next`,
// `bidi_find_first_overridden`, `bidi_dump_cached_states`) carry `#[unsafe(no_mangle)]`
// (PORTING.md 0.2); the 42 C `static` helpers stay plain `unsafe fn`, and all thirteen
// file-scope objects are `static` in C, so they stay unexported `static mut`s.
//
// **The four `#ifdef ENABLE_CHECKING` arms are transcribed under `#[cfg
// (enable_checking)]`**, not recorded as dead text: `ENABLE_CHECKING` is `undef` in the
// frozen `config.h` (macros.tsv 872, `python3 tools/check-cond-eval.py src/bidi.c` →
// `4 conditions, 0 unknown, 0 disagreements [VERIFIED]`), and `xtask`'s
// `--cfg enable_checking` pass compiles exactly these arms (the search.rs/region-cache.rs
// shape, P-51/§3.50).  They are C 1654-1659 (`find_first_strong_char`'s fetch-and-check
// before skipping an isolate), C 2540-2542 (`bpa_stack_entry.open_bracket_pos`),
// C 2587-2592 (`STORE_BRACKET_CHARPOS`) and C 2691-2693 (the pairing-position eassert in
// `bidi_find_bracket_pairs`).  The file has no other `#if`.
//
// **Names no mirror carries are declared at their use site**, each with C's own prototype
// in a `/// C:` line (PORTING.md 0.1-2/P-72, the ccl.rs/composite.rs shape):
// `dispextern.h`'s mirror stops at its batch-II prefix, so `struct bidi_it` (2159-2203),
// `IT_STACK_SIZE` (2384), `update_redisplay_ticks` (3721),
// `compute_display_string_pos` (3621) and `compute_display_string_end` (3624) are
// declared below; `lisp.h`'s `find_newline_no_quit` (5040) and `fast_looking_at` (5031)
// are not in its mirror either; `region-cache.h`'s four prototypes come from its own mirror
// now (`include/region-cache.rs`, which landed with this change - the identical declarations
// that stood below would be an `E0428`); `<intprops.h>`'s `INT_PROMOTE` (P-26) and
// `<stdio.h>`'s `fprintf`/`putc`/`stderr` (the bytecode.rs/cm.rs shape) likewise.
//
// `struct bidi_it` is a *complete* record here rather than a partial one: this unit is
// the bidi iterator's owner, every member is read or written, and the whole layout is
// pinned by the `const _` assertions below against clang's measurements for the frozen
// target (sizeof 2328, align 8, and every named offset).  `struct window *w` is the
// ABI-equal `*mut c_void` with a `/// C:` note (P-72 rule 3): `window.h` is not in this
// unit's include graph — as in C, `struct window` is an incomplete type here — and the
// three entry points that take it (`compute_display_string_pos`,
// `compute_display_string_end`, `update_redisplay_ticks`) declare the same pointer.
//
// `struct bidi_it`'s three `bool_bf : 1` members, `struct bidi_stack`'s three 3-bit
// members and `struct bidi_string_data`'s two `bool_bf : 1` members are packed into the
// mirrors' `bits` fields (PORTING.md 1.3/P-80); every read and write keeps C's member
// name through the `bidi_it_bits!`/`bidi_stack_bits!`/`bidi_string_bits!` macros below,
// whose masks come from the measured byte patterns (C 2320 reads `01`/`02`/`04` for
// `first_elt`/`new_paragraph`/`frame_window_p`; `struct bidi_stack`'s 9-bit unit is
// `f5 01` for the three members set to 5/6/7; `struct bidi_string_data`'s unit is `03`
// for `from_disp_str`+`unibyte`).
//
// C's forward declaration of `bidi_initialize` (C 370) is dropped (PORTING.md 0.1-12);
// C's uninitialised locals are the `std::mem::zeroed ()` declarations named in
// `docs/ports/bidi.md` (PORTING.md 1.5).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_unsafe, unused_assignments, unreachable_code, while_true, unused_parens, unused_mut)]
// [U1-MODULE-UNIT]
use crate::common::*;

const _: () = ();
// [U1] ../lib/include/stdio.rs imported via common
// [U1] ../lib/include/string.rs imported via common
// [U1] include/lisp.rs imported via common
// [U1] include/character.rs imported via common
// [U1] include/itree.rs imported via common
// [U1] include/buffer.rs imported via common
// [U1] include/dispextern.rs imported via common
// [U1] include/region-cache.rs imported via common

/* The names C's headers give this unit that no mirror carries yet (PORTING.md
   0.1-2/P-72, the ccl.rs/composite.rs shape).  Nothing here is this unit's own
   definition; each declaration repeats C's prototype on the line above it.  */

/* C: dispextern.h 2384: `#define IT_STACK_SIZE 5` (the display iterator's stack depth;
   the bidi cache keeps a stack of the same size, C 594).  */
pub const IT_STACK_SIZE: c_int = 5;

/* C: dispextern.h 2159-2203 `struct bidi_it`, the bidi iterator state.  clang against
   the frozen target: sizeof 2328, align 8, and every offset asserted below.  The three
   `bool_bf : 1` members at C 2200-2202 share the byte at 2320 (measured: `01` for
   `first_elt`, `02` for `new_paragraph`, `04` for `frame_window_p`), which the port
   stores as `bits`; `level_stack` is `[bidi_stack; BIDI_MAXDEPTH + 2 + 1]` (128 entries
   of 16 bytes, measured 2048).  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct bidi_it
{
  pub bytepos: ptrdiff_t,
  pub charpos: ptrdiff_t,
  pub ch: c_int,
  pub nchars: ptrdiff_t,
  pub ch_len: ptrdiff_t,
  pub r#type: bidi_type_t,
  pub type_after_wn: bidi_type_t,
  pub orig_type: bidi_type_t,
  pub resolved_level: i8,            // C: `signed char` (the composite.rs spelling)
  pub isolate_level: i8,             // C: `signed char`
  pub invalid_levels: ptrdiff_t,
  pub invalid_isolates: ptrdiff_t,
  pub prev: bidi_saved_info,
  pub last_strong: bidi_saved_info,
  pub next_for_neutral: bidi_saved_info,
  pub prev_for_neutral: bidi_saved_info,
  pub next_for_ws: bidi_saved_info,
  pub bracket_pairing_pos: ptrdiff_t,
  pub bracket_enclosed_type: bidi_type_t,
  pub next_en_pos: ptrdiff_t,
  pub next_en_type: bidi_type_t,
  pub sos: bidi_dir_t,
  pub scan_dir: c_int,
  pub disp_pos: ptrdiff_t,
  pub disp_prop: c_int,
  pub stack_idx: c_int,
  pub level_stack: [bidi_stack; (BIDI_MAXDEPTH + 2 + 1) as usize],
  pub string: bidi_string_data,
  pub w: *mut c_void,                   /// C: `struct window *w;`
  pub paragraph_dir: bidi_dir_t,
  pub separator_limit: ptrdiff_t,
  pub bits: u8,                         // first_elt:1 | new_paragraph:1 | frame_window_p:1
}

const _: () = core::assert! (std::mem::size_of::<bidi_it> () == 2328);
const _: () = core::assert! (std::mem::align_of::<bidi_it> () == 8);
const _: () = core::assert! (std::mem::offset_of! (bidi_it, bytepos) == 0);
const _: () = core::assert! (std::mem::offset_of! (bidi_it, charpos) == 8);
const _: () = core::assert! (std::mem::offset_of! (bidi_it, ch) == 16);
const _: () = core::assert! (std::mem::offset_of! (bidi_it, nchars) == 24);
const _: () = core::assert! (std::mem::offset_of! (bidi_it, ch_len) == 32);
const _: () = core::assert! (std::mem::offset_of! (bidi_it, r#type) == 40);
const _: () = core::assert! (std::mem::offset_of! (bidi_it, type_after_wn) == 44);
const _: () = core::assert! (std::mem::offset_of! (bidi_it, orig_type) == 48);
const _: () = core::assert! (std::mem::offset_of! (bidi_it, resolved_level) == 52);
const _: () = core::assert! (std::mem::offset_of! (bidi_it, isolate_level) == 53);
const _: () = core::assert! (std::mem::offset_of! (bidi_it, invalid_levels) == 56);
const _: () = core::assert! (std::mem::offset_of! (bidi_it, invalid_isolates) == 64);
const _: () = core::assert! (std::mem::offset_of! (bidi_it, prev) == 72);
const _: () = core::assert! (std::mem::offset_of! (bidi_it, last_strong) == 88);
const _: () = core::assert! (std::mem::offset_of! (bidi_it, next_for_neutral) == 104);
const _: () = core::assert! (std::mem::offset_of! (bidi_it, prev_for_neutral) == 120);
const _: () = core::assert! (std::mem::offset_of! (bidi_it, next_for_ws) == 136);
const _: () = core::assert! (std::mem::offset_of! (bidi_it, bracket_pairing_pos) == 152);
const _: () = core::assert! (std::mem::offset_of! (bidi_it, bracket_enclosed_type) == 160);
const _: () = core::assert! (std::mem::offset_of! (bidi_it, next_en_pos) == 168);
const _: () = core::assert! (std::mem::offset_of! (bidi_it, next_en_type) == 176);
const _: () = core::assert! (std::mem::offset_of! (bidi_it, sos) == 180);
const _: () = core::assert! (std::mem::offset_of! (bidi_it, scan_dir) == 184);
const _: () = core::assert! (std::mem::offset_of! (bidi_it, disp_pos) == 192);
const _: () = core::assert! (std::mem::offset_of! (bidi_it, disp_prop) == 200);
const _: () = core::assert! (std::mem::offset_of! (bidi_it, stack_idx) == 204);
const _: () = core::assert! (std::mem::offset_of! (bidi_it, level_stack) == 208);
const _: () = core::assert! (std::mem::offset_of! (bidi_it, string) == 2256);
const _: () = core::assert! (std::mem::offset_of! (bidi_it, w) == 2296);
const _: () = core::assert! (std::mem::offset_of! (bidi_it, paragraph_dir) == 2304);
const _: () = core::assert! (std::mem::offset_of! (bidi_it, separator_limit) == 2312);
const _: () = core::assert! (std::mem::offset_of! (bidi_it, bits) == 2320);

pub const BIDI_IT_FIRST_ELT: u8 = 1 << 0;
pub const BIDI_IT_NEW_PARAGRAPH: u8 = 1 << 1;
pub const BIDI_IT_FRAME_WINDOW_P: u8 = 1 << 2;

/* `struct bidi_it`'s one-byte `bool_bf` run, read and written by C's member name.  */
macro_rules! bidi_it_bits {
  (first_elt, $b:expr) => { ((*$b).bits & BIDI_IT_FIRST_ELT != 0) };
  (first_elt, $b:expr, $value:expr) =>
    { (*$b).bits = if ($value) as c_int != 0 { (*$b).bits | BIDI_IT_FIRST_ELT }
		     else { (*$b).bits & !BIDI_IT_FIRST_ELT }; };
  (new_paragraph, $b:expr) => { ((*$b).bits & BIDI_IT_NEW_PARAGRAPH != 0) };
  (new_paragraph, $b:expr, $value:expr) =>
    { (*$b).bits = if ($value) as c_int != 0 { (*$b).bits | BIDI_IT_NEW_PARAGRAPH }
		     else { (*$b).bits & !BIDI_IT_NEW_PARAGRAPH }; };
  (frame_window_p, $b:expr) => { ((*$b).bits & BIDI_IT_FRAME_WINDOW_P != 0) };
  (frame_window_p, $b:expr, $value:expr) =>
    { (*$b).bits = if ($value) as c_int != 0 { (*$b).bits | BIDI_IT_FRAME_WINDOW_P }
		     else { (*$b).bits & !BIDI_IT_FRAME_WINDOW_P }; };
}

/* C: dispextern.h 2137-2144 `struct bidi_stack`'s three 3-bit members.  The mirror packs
   them into `bits: u16`; the byte pattern with the three set to 5/6/7 is `f5 01`, i.e.
   `next_for_neutral_type` at bits 0-2, `last_strong_type` at 3-5 and
   `prev_for_neutral_type` at 6-8 (measured, little-endian).  */
pub const BIDI_STACK_NEXT_FOR_NEUTRAL_TYPE_MASK: u16 = 0x7;
pub const BIDI_STACK_LAST_STRONG_TYPE_MASK: u16 = 0x7 << 3;
pub const BIDI_STACK_PREV_FOR_NEUTRAL_TYPE_MASK: u16 = 0x7 << 6;

macro_rules! bidi_stack_bits {
  (next_for_neutral_type, $s:expr) => { ((*$s).bits & BIDI_STACK_NEXT_FOR_NEUTRAL_TYPE_MASK) };
  (next_for_neutral_type, $s:expr, $value:expr) =>
    { (*$s).bits = ((*$s).bits & !BIDI_STACK_NEXT_FOR_NEUTRAL_TYPE_MASK)
		   | (($value) as u16 & BIDI_STACK_NEXT_FOR_NEUTRAL_TYPE_MASK); };
  (last_strong_type, $s:expr) => { ((*$s).bits & BIDI_STACK_LAST_STRONG_TYPE_MASK) >> 3 };
  (last_strong_type, $s:expr, $value:expr) =>
    { (*$s).bits = ((*$s).bits & !BIDI_STACK_LAST_STRONG_TYPE_MASK)
		   | (((($value) as u16) << 3) & BIDI_STACK_LAST_STRONG_TYPE_MASK); };
  (prev_for_neutral_type, $s:expr) => { ((*$s).bits & BIDI_STACK_PREV_FOR_NEUTRAL_TYPE_MASK) >> 6 };
  (prev_for_neutral_type, $s:expr, $value:expr) =>
    { (*$s).bits = ((*$s).bits & !BIDI_STACK_PREV_FOR_NEUTRAL_TYPE_MASK)
		   | (((($value) as u16) << 6) & BIDI_STACK_PREV_FOR_NEUTRAL_TYPE_MASK); };
}

/* C: dispextern.h 2147-2156 `struct bidi_string_data`'s two `bool_bf : 1` members; the
   mirror packs them into `bits: u32` (measured: `01` for `from_disp_str`, `02` for
   `unibyte`).  */
pub const BIDI_STRING_FROM_DISP_STR: u32 = 1 << 0;
pub const BIDI_STRING_UNIBYTE: u32 = 1 << 1;

macro_rules! bidi_string_bits {
  (unibyte, $s:expr) => { ((*$s).bits & BIDI_STRING_UNIBYTE != 0) };
  (unibyte, $s:expr, $value:expr) =>
    { (*$s).bits = if ($value) as c_int != 0 { (*$s).bits | BIDI_STRING_UNIBYTE }
		     else { (*$s).bits & !BIDI_STRING_UNIBYTE }; };
}

/* C: dispextern.h 291-292: `#define SET_TEXT_POS(pos, charpos, bytepos)
   ((pos).charpos = (charpos), (pos).bytepos = (bytepos))`.  The dispextern.h mirror
   stops at its batch-II prefix and does not carry the position macros (the indent.rs
   shape), so the one this unit expands is declared here with C's own body.  */
macro_rules! SET_TEXT_POS {
  ($pos:expr, $charpos:expr, $bytepos:expr) => {
    { ($pos).charpos = ($charpos); ($pos).bytepos = ($bytepos); }
  };
}


unsafe extern "C-unwind" {
  /// C: lisp.h 5031-5032: `extern ptrdiff_t fast_looking_at (Lisp_Object, ptrdiff_t,
  /// ptrdiff_t, ptrdiff_t, ptrdiff_t, Lisp_Object);`
  pub fn fast_looking_at (regexp: Lisp_Object, pos: ptrdiff_t, pos_byte: ptrdiff_t,
			  limit: ptrdiff_t, limit_byte: ptrdiff_t, object: Lisp_Object)
			  -> ptrdiff_t;

}

/* `region-cache.h`'s four prototypes are the mirror's now: `include/region-cache.rs`
   carries them with C's prototypes, and this unit names them through that header exactly as
   C does (PORTING.md 0.1-2/0.1-10).  */

/* <stdio.h>: `putc` and `fprintf` for `bidi_dump_cached_states`, and `stderr` under its
   Darwin link name (the cm.rs/bytecode.rs/region-cache.rs shape).  */

/* <intprops.h>'s `INT_PROMOTE (e)` is `(+ (e))` (intprops.h 55): C uses it to promote an
   enumeration to `int` so that `-Wswitch-enum` stays quiet.  The header has no mirror
   (PORTING.md P-26, the ccl.rs `INT_DIVIDE_OVERFLOW` shape), so the macro is declared
   here with C's own body.  */
macro_rules! INT_PROMOTE {
  ($e:expr) => { ($e) as c_int };
}

static bidi_initialized: VmGlobal<bool> = VmGlobal::<bool>::new (false);

static bidi_type_table: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (std::ptr::null_mut ());
static bidi_mirror_table: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (std::ptr::null_mut ());
static bidi_brackets_table: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (std::ptr::null_mut ());

const BIDI_EOB: c_int = -1;

/* Data type for describing the bidirectional character categories.  */
pub type bidi_category_t = c_int;
pub const UNKNOWN_BC: bidi_category_t = 0;
pub const NEUTRAL: bidi_category_t = 1;
pub const WEAK: bidi_category_t = 2;
pub const STRONG: bidi_category_t = 3;
pub const EXPLICIT_FORMATTING: bidi_category_t = 4;

static paragraph_start_re: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (std::ptr::null_mut ());
static paragraph_separate_re: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (std::ptr::null_mut ());

/***********************************************************************
				Utilities
 ***********************************************************************/

/* Return the bidi type of a character CH, subject to the current
   directional OVERRIDE.  */
unsafe fn bidi_get_type (ch: c_int, r#override: bidi_dir_t) -> bidi_type_t
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut default_type: bidi_type_t;

  if ch == BIDI_EOB
    { return NEUTRAL_B; }
  if ch < 0 || ch > MAX_CHAR
    { emacs_abort (); }

  default_type = XFIXNUM (CHAR_TABLE_REF ((*bidi_type_table.as_mut_ptr ()), ch as isize)) as bidi_type_t;
  /* Every valid character code, even those that are unassigned by the
     UCD, have some bidi-class property, according to
     DerivedBidiClass.txt file.  Therefore, if we ever get UNKNOWN_BT
     (= zero) code from CHAR_TABLE_REF, that's a bug.  */
  if default_type == UNKNOWN_BT
    { emacs_abort (); }

  /* Promote default_type to int to allow not enumerating all the values
     without compiler warnings.  */
  match INT_PROMOTE! (default_type)
    {
      WEAK_BN | NEUTRAL_B | LRE | LRO | RLE | RLO | PDF | LRI | RLI | FSI | PDI =>
	{ return default_type; }
      _ => {
	if r#override == L2R
	  { return STRONG_L; }
	else if r#override == R2L
	  { return STRONG_R; }
	else
	  { return default_type; }
      }
    }

  }
}

unsafe fn bidi_check_type (r#type: bidi_type_t)
{
  eassert! (UNKNOWN_BT <= r#type && r#type <= NEUTRAL_ON);
}

/* Given a bidi TYPE of a character, return its category.  */
fn bidi_get_category (r#type: bidi_type_t) -> bidi_category_t
{
  {
  match r#type
    {
      UNKNOWN_BT => return UNKNOWN_BC,
      STRONG_L | STRONG_R | STRONG_AL => return STRONG,
      WEAK_EN | WEAK_ES | WEAK_ET | WEAK_AN | WEAK_CS | WEAK_NSM | WEAK_BN => return WEAK,
      NEUTRAL_B | NEUTRAL_S | NEUTRAL_WS | NEUTRAL_ON => return NEUTRAL,
      LRE | LRO | RLE | RLO | PDF | LRI | RLI | FSI | PDI => return EXPLICIT_FORMATTING,
      _ => { emacs_abort (); }
    }

  }
}

fn bidi_isolate_fmt_char (ch_type: bidi_type_t) -> bool
{
  return ch_type == LRI || ch_type == RLI || ch_type == PDI || ch_type == FSI;
}

/* Return the mirrored character of C, if it has one.  If C has no
   mirrored counterpart, return C.
   Note: The conditions in UAX#9 clause L4 regarding the surrounding
   context must be tested by the caller.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn bidi_mirror_char (c: c_int) -> c_int
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut val: Lisp_Object;

  if c == BIDI_EOB
    { return c; }
  if c < 0 || c > MAX_CHAR
    { emacs_abort (); }

  /* We can be called at the very beginning of init_iterator, via
     produce_special_glyphs, and the first such call in a session might
     happen when the bidi-mirroring table was not yet initialized.  Make
     sure we do this now.  */
  if !CHAR_TABLE_P ((*bidi_mirror_table.as_mut_ptr ()))
    && !(*bidi_initialized.as_mut_ptr ())
    { bidi_initialize (); }

  val = CHAR_TABLE_REF ((*bidi_mirror_table.as_mut_ptr ()), c as isize);
  if FIXNUMP (val)
    {
      let mut v: c_int;

      /* When debugging, check before assigning to V, so that the check
	 isn't broken by undefined behavior due to int overflow.  */
      eassert! (CHAR_VALID_P (XFIXNUM (val) as intmax_t));

      v = XFIXNUM (val) as c_int;

      /* Minimal test we must do in optimized builds, to prevent weird
	 crashes further down the road.  */
      if v < 0 || v > MAX_CHAR
	{ emacs_abort (); }

      return v;
    }

  return c;

  }
}

/* Return the Bidi_Paired_Bracket_Type property of the character C.  */
unsafe fn bidi_paired_bracket_type (c: c_int) -> bidi_bracket_type_t
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  if c == BIDI_EOB || (*bidi_inhibit_bpa.as_mut_ptr ())
    { return BIDI_BRACKET_NONE; }
  if c < 0 || c > MAX_CHAR
    { emacs_abort (); }

  return XFIXNUM (CHAR_TABLE_REF ((*bidi_brackets_table.as_mut_ptr ()), c as isize))
	 as bidi_bracket_type_t;

  }
}

/* Determine the start-of-sequence (sos) directional type given the two
   embedding levels on either side of the run boundary.  Also, update
   the saved info about previously seen characters, since that info is
   generally valid for a single level run.  */
unsafe fn bidi_set_sos_type (bidi_it: *mut bidi_it, level_before: c_int,
			     level_after: c_int)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let higher_level: c_int = max (level_before as ptrdiff_t, level_after as ptrdiff_t)
			    as c_int;

  /* FIXME: should the default sos direction be user selectable?  */
  (*bidi_it).sos = if (higher_level & 1) != 0 { R2L } else { L2R }; /* X10 */

  (*bidi_it).prev.r#type = UNKNOWN_BT;
  (*bidi_it).last_strong.orig_type = UNKNOWN_BT;
  (*bidi_it).last_strong.r#type = UNKNOWN_BT;
  (*bidi_it).prev_for_neutral.r#type = if (*bidi_it).sos == R2L { STRONG_R } else { STRONG_L };
  (*bidi_it).prev_for_neutral.charpos = (*bidi_it).charpos;
  (*bidi_it).next_for_neutral.orig_type = UNKNOWN_BT;
  (*bidi_it).next_for_neutral.r#type = UNKNOWN_BT;

  }
}

macro_rules! ISOLATE_STATUS {
  ($bidi_it:expr, $idx:expr) => { ((*$bidi_it).level_stack[($idx) as usize].flags & 1) };
}
macro_rules! OVERRIDE {
  ($bidi_it:expr, $idx:expr) => { (((*$bidi_it).level_stack[($idx) as usize].flags >> 1) & 3) };
}

/* Push the current embedding level and override status; reset the
   current level to LEVEL and the current override status to OVERRIDE.  */
unsafe fn bidi_push_embedding_level (bidi_it: *mut bidi_it, level: c_int,
				     r#override: bidi_dir_t, isolate_status: bool)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut st: *mut bidi_stack;
  let prev_level: c_int = (*bidi_it).level_stack[(*bidi_it).stack_idx as usize].level
			  as c_int;

  (*bidi_it).stack_idx += 1;
  eassert! ((*bidi_it).stack_idx < BIDI_MAXDEPTH + 2 + 1);
  st = &raw mut (*bidi_it).level_stack[(*bidi_it).stack_idx as usize];
  eassert! (level <= (1 << 7));
  (*st).level = level as u8;
  (*st).flags = ((((r#override & 3) << 1)
		 | ((isolate_status as c_int) != 0) as c_int) as u8);
  if isolate_status
    {
      bidi_stack_bits! (last_strong_type, st, (*bidi_it).last_strong.r#type);
      bidi_stack_bits! (prev_for_neutral_type, st, (*bidi_it).prev_for_neutral.r#type);
      bidi_stack_bits! (next_for_neutral_type, st, (*bidi_it).next_for_neutral.r#type);
      (*st).next_for_neutral_pos = (*bidi_it).next_for_neutral.charpos;
      (*st).flags |= ((if (*bidi_it).sos == L2R { 0 } else { 1 }) << 3) as u8;
    }
  /* We've got a new isolating sequence, compute the directional type
     of sos and initialize per-sequence variables (UAX#9, clause X10).  */
  bidi_set_sos_type (bidi_it, prev_level, level);

  }
}

/* Pop from the stack the embedding level, the directional override
   status, and optionally saved information for the isolating run
   sequence.  Return the new level.  */
unsafe fn bidi_pop_embedding_level (bidi_it: *mut bidi_it) -> c_int
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut level: c_int;

  /* UAX#9 says to ignore invalid PDFs (X7, last bullet)
     and PDIs (X6a, 2nd bullet).  */
  if (*bidi_it).stack_idx > 0
    {
      let isolate_status: bool = ISOLATE_STATUS! (bidi_it, (*bidi_it).stack_idx) != 0;
      let old_level: c_int = (*bidi_it).level_stack[(*bidi_it).stack_idx as usize].level
			     as c_int;

      let mut st: bidi_stack;

      st = (*bidi_it).level_stack[(*bidi_it).stack_idx as usize];
      if isolate_status
	{
	  let sos: bidi_dir_t = ((st.flags >> 3) & 1) as bidi_dir_t;
	  /* PREV is used in W1 for resolving WEAK_NSM.  By the time
	     we get to an NSM, we must have gotten past at least one
	     character: the PDI that ends the isolate from which we
	     are popping here.  So PREV will have been filled up by
	     the time we first use it.  We initialize it here to
	     UNKNOWN_BT to be able to catch any blunders in this
	     logic.  */
	  (*bidi_it).prev.orig_type = UNKNOWN_BT;
	  (*bidi_it).prev.r#type = UNKNOWN_BT;
	  (*bidi_it).last_strong.r#type = bidi_stack_bits! (last_strong_type, &raw const st) as bidi_type_t;
	  (*bidi_it).prev_for_neutral.r#type = bidi_stack_bits! (prev_for_neutral_type, &raw const st) as bidi_type_t;
	  (*bidi_it).next_for_neutral.r#type = bidi_stack_bits! (next_for_neutral_type, &raw const st) as bidi_type_t;
	  (*bidi_it).next_for_neutral.charpos = st.next_for_neutral_pos;
	  (*bidi_it).sos = if sos == 0 { L2R } else { R2L };
	}
      else
	{ bidi_set_sos_type (bidi_it, old_level,
			     (*bidi_it).level_stack[(*bidi_it).stack_idx as usize - 1].level
			     as c_int); }

      (*bidi_it).stack_idx -= 1;
    }
  level = (*bidi_it).level_stack[(*bidi_it).stack_idx as usize].level as c_int;
  eassert! (0 <= level && level <= BIDI_MAXDEPTH + 1);
  return level;

  }
}

/* Record in SAVED_INFO the information about the current character.  */
unsafe fn bidi_remember_char (saved_info: *mut bidi_saved_info, bidi_it: *mut bidi_it,
			      from_type: bool)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  (*saved_info).charpos = (*bidi_it).charpos;
  if from_type
    { (*saved_info).r#type = (*bidi_it).r#type; }
  else
    { (*saved_info).r#type = (*bidi_it).type_after_wn; }
  bidi_check_type ((*saved_info).r#type);
  (*saved_info).orig_type = (*bidi_it).orig_type;
  bidi_check_type ((*saved_info).orig_type);

  }
}

/* Copy the bidi iterator from FROM to TO.  To save cycles, this only
   copies the part of the level stack that is actually in use.  */
unsafe fn bidi_copy_it (to: *mut bidi_it, from: *mut bidi_it)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  /* Copy everything from the start through the active part of
     the level stack.  */
  memcpy (to as *mut c_void, from as *const c_void,
	  std::mem::offset_of! (bidi_it, level_stack)
	  + std::mem::size_of_val (&(*from).level_stack[0])
	  + (*from).stack_idx as usize * std::mem::size_of_val (&(*from).level_stack[0]));

  }
}

/***********************************************************************
			Caching the bidi iterator states
 ***********************************************************************/

/* We allocate and de-allocate the cache in chunks of this size (in
   characters).  200 was chosen as an upper limit for reasonably-long
   lines in a text file/buffer.  */
const BIDI_CACHE_CHUNK: ptrdiff_t = 200;
/* Maximum size we allow the cache to become, per iterator stack slot,
   in units of struct bidi_it size.  ... [C 561-579's comment] */
const BIDI_CACHE_MAX_ELTS_PER_SLOT: ptrdiff_t = 50000;
static_assert! (BIDI_CACHE_CHUNK < BIDI_CACHE_MAX_ELTS_PER_SLOT);
static bidi_cache_max_elts: VmGlobal<ptrdiff_t> = VmGlobal::<ptrdiff_t>::new (BIDI_CACHE_MAX_ELTS_PER_SLOT);
static bidi_cache: VmGlobal<*mut bidi_it> = VmGlobal::<*mut bidi_it>::new (std::ptr::null_mut ());
static bidi_cache_size: VmGlobal<ptrdiff_t> = VmGlobal::<ptrdiff_t>::new (0);
const elsz: usize = std::mem::size_of::<bidi_it> ();
static bidi_cache_idx: VmGlobal<ptrdiff_t> = VmGlobal::<ptrdiff_t>::new (0);	/* next unused cache slot */
static bidi_cache_last_idx: VmGlobal<ptrdiff_t> = VmGlobal::<ptrdiff_t>::new (0);	/* slot of last cache hit */
static bidi_cache_start: VmGlobal<ptrdiff_t> = VmGlobal::<ptrdiff_t>::new (0);	/* start of cache for this
						   "stack" level */

/* 5-slot stack for saving the start of the previous level of the
   cache.  xdisp.c maintains a 5-slot stack for its iterator state,
   and we need the same size of our stack.  */
static bidi_cache_start_stack: VmGlobal<[ptrdiff_t; IT_STACK_SIZE as usize]> = VmGlobal::<[ptrdiff_t; IT_STACK_SIZE as usize]>::new ([0; IT_STACK_SIZE as usize]);
static bidi_cache_sp: VmGlobal<c_int> = VmGlobal::<c_int>::new (0);

/* Size of header used by bidi_shelve_cache.  C's `sizeof OBJECT` is Rust's
   `size_of_val (&OBJECT)` (the `sizeof` token family), so C's six operands are named here
   exactly as C names them; taking a shared reference to a `static mut` is the one thing
   that needs the `unsafe` const block (PORTING.md 0.2's `static_mut_refs` allowance).  */
const bidi_shelve_header_size: usize
  = const { {
      femacs_sizeof_ptr (bidi_cache_idx.as_static_ptr ())
      + femacs_sizeof_ptr (bidi_cache_start_stack.as_static_ptr ())
      + femacs_sizeof_ptr (bidi_cache_sp.as_static_ptr ())
      + femacs_sizeof_ptr (bidi_cache_start.as_static_ptr ())
      + femacs_sizeof_ptr (bidi_cache_last_idx.as_static_ptr ())
      + femacs_sizeof_ptr (bidi_cache_max_elts.as_static_ptr ())
    } };

/* Effectively remove the cached states beyond the Nth state from the
   part of the cache relevant to iteration of the current object
   (buffer or string).  */
unsafe fn bidi_cache_reset_to (n: c_int)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  (*bidi_cache_idx.as_mut_ptr ()) = (*bidi_cache_start.as_mut_ptr ()) + n as ptrdiff_t;
  (*bidi_cache_last_idx.as_mut_ptr ()) = -1;

  }
}

/* Reset the cache state to the empty state.  We only reset the part
   of the cache relevant to iteration of the current object.  Previous
   objects, which are pushed on the display iterator's stack, are left
   intact.  This is called when the cached information is no more
   useful for the current iteration, e.g. when we were reseated to a
   new position on the same object.  */
unsafe fn bidi_cache_reset ()
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  bidi_cache_reset_to (0);

  }
}

/* Shrink the cache to its minimal size.  Called when we init the bidi
   iterator for reordering a buffer or a string that does not come
   from display properties, because that means all the previously
   cached info is of no further use.  */
unsafe fn bidi_cache_shrink ()
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  if (*bidi_cache_size.as_mut_ptr ()) > BIDI_CACHE_CHUNK
    {
      (*bidi_cache.as_mut_ptr ()) = xrealloc ((*bidi_cache.as_mut_ptr ()) as *mut c_void,
			     BIDI_CACHE_CHUNK as usize * elsz) as *mut bidi_it;
      (*bidi_cache_size.as_mut_ptr ()) = BIDI_CACHE_CHUNK;
    }
  bidi_cache_reset ();
  (*bidi_cache_max_elts.as_mut_ptr ()) = BIDI_CACHE_MAX_ELTS_PER_SLOT;

  }
}

unsafe fn bidi_cache_fetch_state (idx: ptrdiff_t, bidi_it: *mut bidi_it)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let current_scan_dir: c_int = (*bidi_it).scan_dir;

  if idx < (*bidi_cache_start.as_mut_ptr ()) || idx >= (*bidi_cache_idx.as_mut_ptr ())
    { emacs_abort (); }

  bidi_copy_it (bidi_it, (*bidi_cache.as_mut_ptr ()).offset (idx));
  (*bidi_it).scan_dir = current_scan_dir;
  (*bidi_cache_last_idx.as_mut_ptr ()) = idx;

  }
}

/* Find a cached state with a given CHARPOS and resolved embedding
   level less or equal to LEVEL.  If LEVEL is -1, disregard the
   resolved levels in cached states.  DIR, if non-zero, means search
   in that direction from the last cache hit.

   Value is the index of the cached state, or -1 if not found.  */
unsafe fn bidi_cache_search (charpos: ptrdiff_t, level: c_int, mut dir: c_int)
			     -> ptrdiff_t
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut i: ptrdiff_t;
  let mut i_start: ptrdiff_t;

  if (*bidi_cache_idx.as_mut_ptr ()) > (*bidi_cache_start.as_mut_ptr ())
    {
      if (*bidi_cache_last_idx.as_mut_ptr ()) == -1
	{ (*bidi_cache_last_idx.as_mut_ptr ()) = (*bidi_cache_idx.as_mut_ptr ()) - 1; }
      if charpos < (*(*bidi_cache.as_mut_ptr ()).offset ((*bidi_cache_last_idx.as_mut_ptr ()))).charpos
	{
	  dir = -1;
	  i_start = (*bidi_cache_last_idx.as_mut_ptr ()) - 1;
	}
      else if charpos > (*(*bidi_cache.as_mut_ptr ()).offset ((*bidi_cache_last_idx.as_mut_ptr ()))).charpos
			  + (*(*bidi_cache.as_mut_ptr ()).offset ((*bidi_cache_last_idx.as_mut_ptr ()))).nchars - 1
	{
	  dir = 1;
	  i_start = (*bidi_cache_last_idx.as_mut_ptr ()) + 1;
	}
      else if dir != 0
	{ i_start = (*bidi_cache_last_idx.as_mut_ptr ()); }
      else
	{
	  dir = -1;
	  i_start = (*bidi_cache_idx.as_mut_ptr ()) - 1;
	}

      if dir < 0
	{
	  /* Linear search for now; FIXME!  */
	  i = i_start;
	  while i >= (*bidi_cache_start.as_mut_ptr ())
	    {
	      if (*(*bidi_cache.as_mut_ptr ()).offset (i)).charpos <= charpos
		  && charpos < (*(*bidi_cache.as_mut_ptr ()).offset (i)).charpos
			       + (*(*bidi_cache.as_mut_ptr ()).offset (i)).nchars
		  && (level == -1
		      || (*(*bidi_cache.as_mut_ptr ()).offset (i)).resolved_level as c_int <= level)
		{ return i; }
	      i -= 1;
	    }
	}
      else
	{
	  i = i_start;
	  while i < (*bidi_cache_idx.as_mut_ptr ())
	    {
	      if (*(*bidi_cache.as_mut_ptr ()).offset (i)).charpos <= charpos
		  && charpos < (*(*bidi_cache.as_mut_ptr ()).offset (i)).charpos
			       + (*(*bidi_cache.as_mut_ptr ()).offset (i)).nchars
		  && (level == -1
		      || (*(*bidi_cache.as_mut_ptr ()).offset (i)).resolved_level as c_int <= level)
		{ return i; }
	      i += 1;
	    }
	}
    }

  return -1;

  }
}

/* Find a cached state where the resolved level changes to a value
   that is lower than LEVEL, and return its cache slot index.  DIR is
   the direction to search, starting with the last used cache slot.
   If DIR is zero, we search backwards from the last occupied cache
   slot.  BEFORE means return the index of the slot that
   is ``before'' the level change in the search direction.  ...  */
unsafe fn bidi_cache_find_level_change (level: c_int, mut dir: c_int, before: bool)
					 -> ptrdiff_t
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  if (*bidi_cache_idx.as_mut_ptr ()) != 0
    {
      let mut i: ptrdiff_t = if dir != 0 { (*bidi_cache_last_idx.as_mut_ptr ()) }
			     else { (*bidi_cache_idx.as_mut_ptr ()) - 1 };
      let incr: c_int = if before { 1 } else { 0 };

      if i < 0  /* cache overflowed? */
	{ i = 0; }

      if dir == 0
	{ dir = -1; }
      else if incr == 0
	{ i += dir as ptrdiff_t; }

      if dir < 0
	{
	  while i >= (*bidi_cache_start.as_mut_ptr ()) + incr as ptrdiff_t
	    {
	      if (*(*bidi_cache.as_mut_ptr ()).offset (i - incr as ptrdiff_t)).resolved_level >= 0
		  && ((*(*bidi_cache.as_mut_ptr ()).offset (i - incr as ptrdiff_t)).resolved_level as c_int)
		     < level
		{ return i; }
	      i -= 1;
	    }
	}
      else
	{
	  while i < (*bidi_cache_idx.as_mut_ptr ()) - incr as ptrdiff_t
	    {
	      if (*(*bidi_cache.as_mut_ptr ()).offset (i + incr as ptrdiff_t)).resolved_level >= 0
		  && ((*(*bidi_cache.as_mut_ptr ()).offset (i + incr as ptrdiff_t)).resolved_level as c_int)
		     < level
		{ return i; }
	      i += 1;
	    }
	}
    }

  return -1;

  }
}

/* Find the previous character position where LEVEL changes to a lower
   one.  Return -1 if not found (which really shouldn't happen if this
   function is called on a backward scan).  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn bidi_level_start (level: c_int) -> ptrdiff_t
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let slot: ptrdiff_t = bidi_cache_find_level_change (level, -1, true);

  if slot >= 0
    { return (*(*bidi_cache.as_mut_ptr ()).offset (slot)).charpos; }
  return -1;

  }
}

unsafe fn bidi_cache_ensure_space (idx: ptrdiff_t)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  /* Enlarge the cache as needed.  */
  if idx >= (*bidi_cache_size.as_mut_ptr ())
    {
      let mut chunk_size: ptrdiff_t = BIDI_CACHE_CHUNK;

      if (*bidi_cache_size.as_mut_ptr ()) > (*bidi_cache_max_elts.as_mut_ptr ()) - chunk_size
	{ chunk_size = (*bidi_cache_max_elts.as_mut_ptr ()) - (*bidi_cache_size.as_mut_ptr ()); }

      if max (idx + 1, (*bidi_cache_size.as_mut_ptr ()) + chunk_size) <= (*bidi_cache_max_elts.as_mut_ptr ())
	{
	  /* The bidi cache cannot be larger than the largest Lisp
	     string or buffer.  */
	  let string_or_buffer_bound: ptrdiff_t = max (BUF_BYTES_MAX, STRING_BYTES_BOUND);

	  /* Also, it cannot be larger than what C can represent.  */
	  let c_bound: ptrdiff_t
	    = ((PTRDIFF_MAX as usize).min (SIZE_MAX) as ptrdiff_t
	       - bidi_shelve_header_size as ptrdiff_t) / elsz as ptrdiff_t;
	  let mut max_elts: ptrdiff_t = (*bidi_cache_max_elts.as_mut_ptr ());

	  max_elts = min (max_elts, min (string_or_buffer_bound, c_bound));

	  /* Force xpalloc not to over-allocate by passing it MAX_ELTS
	     as its 4th argument.  */
	  (*bidi_cache.as_mut_ptr ()) = xpalloc ((*bidi_cache.as_mut_ptr ()) as *mut c_void, bidi_cache_size.as_mut_ptr (),
				max (chunk_size, idx - (*bidi_cache_size.as_mut_ptr ()) + 1),
				max_elts, elsz as ptrdiff_t) as *mut bidi_it;
	  eassert! ((*bidi_cache_size.as_mut_ptr ()) > idx);
	}
    }

  }
}

unsafe fn bidi_cache_iterator_state (bidi_it: *mut bidi_it, resolved: bool,
				     update_only: bool) -> c_int
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
	  let mut idx: ptrdiff_t = 0;

  /* We should never cache on backward scans.  */
  if (*bidi_it).scan_dir == -1
    { emacs_abort (); }
  idx = bidi_cache_search ((*bidi_it).charpos, -1, 1);

  if idx < 0 && update_only
    { return 0; }

  if idx < 0
    {
      idx = (*bidi_cache_idx.as_mut_ptr ());
      bidi_cache_ensure_space (idx);
      /* Character positions should correspond to cache positions 1:1.
	 If we are outside the range of cached positions, the cache is
	 useless and must be reset.  */
      if (*bidi_cache_start.as_mut_ptr ()) < idx && idx < (*bidi_cache_size.as_mut_ptr ())
	  && ((*bidi_it).charpos > (*(*bidi_cache.as_mut_ptr ()).offset (idx - 1)).charpos
				      + (*(*bidi_cache.as_mut_ptr ()).offset (idx - 1)).nchars
	      || (*bidi_it).charpos < (*(*bidi_cache.as_mut_ptr ()).offset ((*bidi_cache_start.as_mut_ptr ()))).charpos)
	{
	  bidi_cache_reset ();
	  idx = (*bidi_cache_start.as_mut_ptr ());
	}
      if (*bidi_it).nchars <= 0
	{ emacs_abort (); }
      /* Don't cache if no available space in the cache.  */
      if (*bidi_cache_size.as_mut_ptr ()) > idx
	{
	  bidi_copy_it ((*bidi_cache.as_mut_ptr ()).offset (idx), bidi_it);
	  if !resolved
	    { (*(*bidi_cache.as_mut_ptr ()).offset (idx)).resolved_level = -1; }
	}
    }
  else
    {
      /* Copy only the members which could have changed, to avoid
	 costly copying of the entire struct.  */
      (*(*bidi_cache.as_mut_ptr ()).offset (idx)).r#type = (*bidi_it).r#type;
      bidi_check_type ((*bidi_it).r#type);
      (*(*bidi_cache.as_mut_ptr ()).offset (idx)).type_after_wn = (*bidi_it).type_after_wn;
      bidi_check_type ((*bidi_it).type_after_wn);
      if resolved
	{ (*(*bidi_cache.as_mut_ptr ()).offset (idx)).resolved_level = (*bidi_it).resolved_level; }
      else
	{ (*(*bidi_cache.as_mut_ptr ()).offset (idx)).resolved_level = -1; }
      (*(*bidi_cache.as_mut_ptr ()).offset (idx)).invalid_levels = (*bidi_it).invalid_levels;
      (*(*bidi_cache.as_mut_ptr ()).offset (idx)).next_for_neutral = (*bidi_it).next_for_neutral;
      (*(*bidi_cache.as_mut_ptr ()).offset (idx)).next_for_ws = (*bidi_it).next_for_ws;
      (*(*bidi_cache.as_mut_ptr ()).offset (idx)).disp_pos = (*bidi_it).disp_pos;
      (*(*bidi_cache.as_mut_ptr ()).offset (idx)).disp_prop = (*bidi_it).disp_prop;
      (*(*bidi_cache.as_mut_ptr ()).offset (idx)).bracket_pairing_pos = (*bidi_it).bracket_pairing_pos;
      (*(*bidi_cache.as_mut_ptr ()).offset (idx)).bracket_enclosed_type = (*bidi_it).bracket_enclosed_type;
    }

  if (*bidi_cache_size.as_mut_ptr ()) > idx
    {
      (*bidi_cache_last_idx.as_mut_ptr ()) = idx;
      if idx >= (*bidi_cache_idx.as_mut_ptr ())
	{ (*bidi_cache_idx.as_mut_ptr ()) = idx + 1; }
      return 1;
    }
  else
    {
      /* The cache overflowed.  */
      (*bidi_cache_last_idx.as_mut_ptr ()) = -1;
      return 0;
    }

  }
}

/* Look for a cached iterator state that corresponds to CHARPOS.  If
   found, copy the cached state into BIDI_IT and return the type of
   the cached entry.  If not found, return UNKNOWN_BT.  RESOLVED_ONLY
   zero means it is OK to return cached states that were not fully
   resolved yet.  This can happen if the state was cached before it
   was resolved in bidi_resolve_neutral.  */
unsafe fn bidi_cache_find (charpos: ptrdiff_t, resolved_only: bool,
			   bidi_it: *mut bidi_it) -> bidi_type_t
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let i: ptrdiff_t = bidi_cache_search (charpos, -1, (*bidi_it).scan_dir);

  if i >= (*bidi_cache_start.as_mut_ptr ())
      && (!resolved_only
	  /* Callers that want only fully resolved states (and set
	     resolved_only = true) need to be sure that there's enough
	     info in the cached state to return the state as final,
	     and if not, they don't want the cached state.  */
	  || (*(*bidi_cache.as_mut_ptr ()).offset (i)).resolved_level >= 0)
    {
      let current_scan_dir: bidi_dir_t = (*bidi_it).scan_dir as bidi_dir_t;

      bidi_copy_it (bidi_it, (*bidi_cache.as_mut_ptr ()).offset (i));
      (*bidi_cache_last_idx.as_mut_ptr ()) = i;
      /* Don't let scan direction from the cached state override
	 the current scan direction.  */
      (*bidi_it).scan_dir = current_scan_dir;
      return (*bidi_it).r#type;
    }

  return UNKNOWN_BT;

  }
}

unsafe fn bidi_peek_at_next_level (bidi_it: *mut bidi_it) -> c_int
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  if (*bidi_cache_idx.as_mut_ptr ()) == (*bidi_cache_start.as_mut_ptr ())
    { emacs_abort (); }
  /* If the cache overflowed, return the level of the last cached
     character.  */
  if (*bidi_cache_last_idx.as_mut_ptr ()) == -1
      || ((*bidi_cache_last_idx.as_mut_ptr ()) >= (*bidi_cache_idx.as_mut_ptr ()) - 1 && (*bidi_it).scan_dir > 0)
    { return (*(*bidi_cache.as_mut_ptr ()).offset ((*bidi_cache_idx.as_mut_ptr ()) - 1)).resolved_level as c_int; }
  return (*(*bidi_cache.as_mut_ptr ()).offset ((*bidi_cache_last_idx.as_mut_ptr ()) + (*bidi_it).scan_dir as ptrdiff_t))
	 .resolved_level as c_int;

  }
}

/***********************************************************************
	     Pushing and popping the bidi iterator state
 ***********************************************************************/

/* Push the bidi iterator state in preparation for reordering a
   different object, e.g. display string found at certain buffer
   position.  Pushing the bidi iterator boils down to saving its
   entire state on the cache and starting a new cache "stacked" on top
   of the current cache.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn bidi_push_it (bidi_it: *mut bidi_it)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  /* Give this stack slot its cache room.  */
  (*bidi_cache_max_elts.as_mut_ptr ()) += BIDI_CACHE_MAX_ELTS_PER_SLOT;
  /* Save the current iterator state in its entirety after the last
     used cache slot.  */
  bidi_cache_ensure_space ((*bidi_cache_idx.as_mut_ptr ()));
  *(*bidi_cache.as_mut_ptr ()).offset ((*bidi_cache_idx.as_mut_ptr ())) = *bidi_it;
  (*bidi_cache_idx.as_mut_ptr ()) += 1;

  /* Push the current cache start onto the stack.  */
  eassert! ((*bidi_cache_sp.as_mut_ptr ()) < IT_STACK_SIZE);
  (*bidi_cache_start_stack.as_mut_ptr ())[(*bidi_cache_sp.as_mut_ptr ()) as usize] = (*bidi_cache_start.as_mut_ptr ());
  (*bidi_cache_sp.as_mut_ptr ()) += 1;

  /* Start a new level of cache, and make it empty.  */
  (*bidi_cache_start.as_mut_ptr ()) = (*bidi_cache_idx.as_mut_ptr ());
  (*bidi_cache_last_idx.as_mut_ptr ()) = -1;

  }
}

/* Restore the iterator state saved by bidi_push_it and return the
   cache to the corresponding state.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn bidi_pop_it (bidi_it: *mut bidi_it)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  if (*bidi_cache_start.as_mut_ptr ()) <= 0
    { emacs_abort (); }

  /* Reset the next free cache slot index to what it was before the
     call to bidi_push_it.  */
  (*bidi_cache_idx.as_mut_ptr ()) = (*bidi_cache_start.as_mut_ptr ()) - 1;

  /* Restore the bidi iterator state saved in the cache.  */
  *bidi_it = *(*bidi_cache.as_mut_ptr ()).offset ((*bidi_cache_idx.as_mut_ptr ()));

  /* Pop the previous cache start from the stack.  */
  if (*bidi_cache_sp.as_mut_ptr ()) <= 0
    { emacs_abort (); }
  (*bidi_cache_sp.as_mut_ptr ()) -= 1;
  (*bidi_cache_start.as_mut_ptr ()) = (*bidi_cache_start_stack.as_mut_ptr ())[(*bidi_cache_sp.as_mut_ptr ()) as usize];

  /* Invalidate the last-used cache slot data.  */
  (*bidi_cache_last_idx.as_mut_ptr ()) = -1;

  (*bidi_cache_max_elts.as_mut_ptr ()) -= BIDI_CACHE_MAX_ELTS_PER_SLOT;
  eassert! ((*bidi_cache_max_elts.as_mut_ptr ()) > 0);

  }
}

static bidi_cache_total_alloc: VmGlobal<ptrdiff_t> = VmGlobal::<ptrdiff_t>::new (0);

/* Stash away a copy of the cache and its control variables.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn bidi_shelve_cache () -> *mut c_void
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut databuf: *mut u8;
  let mut alloc: ptrdiff_t;

  /* Empty cache.  */
  if (*bidi_cache_idx.as_mut_ptr ()) == 0
    { return std::ptr::null_mut (); }

  alloc = (bidi_shelve_header_size
	   + (*bidi_cache_idx.as_mut_ptr ()) as usize * std::mem::size_of::<bidi_it> ()) as ptrdiff_t;
  databuf = xmalloc (alloc as usize) as *mut u8;
  (*bidi_cache_total_alloc.as_mut_ptr ()) += alloc;

  memcpy (databuf as *mut c_void, bidi_cache_idx.as_ptr () as *const c_void,
	  std::mem::size_of::<ptrdiff_t> ());
  memcpy (databuf.offset (std::mem::size_of::<ptrdiff_t> () as ptrdiff_t) as *mut c_void,
	  (*bidi_cache.as_mut_ptr ()) as *const c_void,
	  (*bidi_cache_idx.as_mut_ptr ()) as usize * std::mem::size_of::<bidi_it> ());
  memcpy (databuf.offset (std::mem::size_of::<ptrdiff_t> () as ptrdiff_t
			  + (*bidi_cache_idx.as_mut_ptr ()) * std::mem::size_of::<bidi_it> () as ptrdiff_t)
	  as *mut c_void,
	  bidi_cache_start_stack.as_ptr () as *const c_void,
	  std::mem::size_of::<[ptrdiff_t; IT_STACK_SIZE as usize]> ());
  memcpy (databuf.offset (std::mem::size_of::<ptrdiff_t> () as ptrdiff_t
			  + (*bidi_cache_idx.as_mut_ptr ()) * std::mem::size_of::<bidi_it> () as ptrdiff_t
			  + std::mem::size_of::<[ptrdiff_t; IT_STACK_SIZE as usize]> ()
			    as ptrdiff_t) as *mut c_void,
	  bidi_cache_sp.as_ptr () as *const c_void, std::mem::size_of::<c_int> ());
  memcpy (databuf.offset (std::mem::size_of::<ptrdiff_t> () as ptrdiff_t
			  + (*bidi_cache_idx.as_mut_ptr ()) * std::mem::size_of::<bidi_it> () as ptrdiff_t
			  + std::mem::size_of::<[ptrdiff_t; IT_STACK_SIZE as usize]> ()
			    as ptrdiff_t
			  + std::mem::size_of::<c_int> () as ptrdiff_t) as *mut c_void,
	  bidi_cache_start.as_ptr () as *const c_void, std::mem::size_of::<ptrdiff_t> ());
  memcpy (databuf.offset (std::mem::size_of::<ptrdiff_t> () as ptrdiff_t
			  + (*bidi_cache_idx.as_mut_ptr ()) * std::mem::size_of::<bidi_it> () as ptrdiff_t
			  + std::mem::size_of::<[ptrdiff_t; IT_STACK_SIZE as usize]> ()
			    as ptrdiff_t
			  + std::mem::size_of::<c_int> () as ptrdiff_t
			  + std::mem::size_of::<ptrdiff_t> () as ptrdiff_t) as *mut c_void,
	  bidi_cache_last_idx.as_ptr () as *const c_void,
	  std::mem::size_of::<ptrdiff_t> ());
  memcpy (databuf.offset (std::mem::size_of::<ptrdiff_t> () as ptrdiff_t
			  + (*bidi_cache_idx.as_mut_ptr ()) * std::mem::size_of::<bidi_it> () as ptrdiff_t
			  + std::mem::size_of::<[ptrdiff_t; IT_STACK_SIZE as usize]> ()
			    as ptrdiff_t
			  + std::mem::size_of::<c_int> () as ptrdiff_t
			  + std::mem::size_of::<ptrdiff_t> () as ptrdiff_t
			  + std::mem::size_of::<ptrdiff_t> () as ptrdiff_t) as *mut c_void,
	  bidi_cache_max_elts.as_ptr () as *const c_void,
	  std::mem::size_of::<ptrdiff_t> ());

  return databuf as *mut c_void;

  }
}

/* Restore the cache state from a copy stashed away by
   bidi_shelve_cache, and free the buffer used to stash that copy.
   JUST_FREE means free the buffer, but don't restore the
   cache; used when the corresponding iterator is discarded instead of
   being restored.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn bidi_unshelve_cache (databuf: *mut c_void, just_free: bool)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut p: *mut u8 = databuf as *mut u8;

  if p.is_null ()
    {
      if !just_free
	{
	  /* A NULL pointer means an empty cache.  */
	  (*bidi_cache_start.as_mut_ptr ()) = 0;
	  (*bidi_cache_sp.as_mut_ptr ()) = 0;
	  (*bidi_cache_max_elts.as_mut_ptr ()) = BIDI_CACHE_MAX_ELTS_PER_SLOT;
	  bidi_cache_reset ();
	}
    }
  else
    {
      if just_free
	{
	  let mut idx: ptrdiff_t = 0;

	  memcpy (&raw mut idx as *mut c_void, p as *const c_void,
		  std::mem::size_of::<ptrdiff_t> ());
	  (*bidi_cache_total_alloc.as_mut_ptr ())
	    -= (bidi_shelve_header_size
		+ idx as usize * std::mem::size_of::<bidi_it> ()) as ptrdiff_t;
	}
      else
	{
	  memcpy (bidi_cache_idx.as_mut_ptr () as *mut c_void, p as *const c_void,
		  std::mem::size_of::<ptrdiff_t> ());
	  bidi_cache_ensure_space ((*bidi_cache_idx.as_mut_ptr ()));
	  memcpy ((*bidi_cache.as_mut_ptr ()) as *mut c_void,
		  p.offset (std::mem::size_of::<ptrdiff_t> () as ptrdiff_t) as *const c_void,
		  (*bidi_cache_idx.as_mut_ptr ()) as usize * std::mem::size_of::<bidi_it> ());
	  memcpy (bidi_cache_start_stack.as_mut_ptr () as *mut c_void,
		  p.offset (std::mem::size_of::<ptrdiff_t> () as ptrdiff_t
			    + (*bidi_cache_idx.as_mut_ptr ()) * std::mem::size_of::<bidi_it> () as ptrdiff_t)
		   as *const c_void,
		  std::mem::size_of::<[ptrdiff_t; IT_STACK_SIZE as usize]> ());
	  memcpy (bidi_cache_sp.as_mut_ptr () as *mut c_void,
		  p.offset (std::mem::size_of::<ptrdiff_t> () as ptrdiff_t
			    + (*bidi_cache_idx.as_mut_ptr ()) * std::mem::size_of::<bidi_it> () as ptrdiff_t
			    + std::mem::size_of::<[ptrdiff_t; IT_STACK_SIZE as usize]> ()
			      as ptrdiff_t) as *const c_void,
		  std::mem::size_of::<c_int> ());
	  memcpy (bidi_cache_start.as_mut_ptr () as *mut c_void,
		  p.offset (std::mem::size_of::<ptrdiff_t> () as ptrdiff_t
			    + (*bidi_cache_idx.as_mut_ptr ()) * std::mem::size_of::<bidi_it> () as ptrdiff_t
			    + std::mem::size_of::<[ptrdiff_t; IT_STACK_SIZE as usize]> ()
			      as ptrdiff_t
			    + std::mem::size_of::<c_int> () as ptrdiff_t) as *const c_void,
		  std::mem::size_of::<ptrdiff_t> ());
	  memcpy (bidi_cache_last_idx.as_mut_ptr () as *mut c_void,
		  p.offset (std::mem::size_of::<ptrdiff_t> () as ptrdiff_t
			    + (*bidi_cache_idx.as_mut_ptr ()) * std::mem::size_of::<bidi_it> () as ptrdiff_t
			    + std::mem::size_of::<[ptrdiff_t; IT_STACK_SIZE as usize]> ()
			      as ptrdiff_t
			    + std::mem::size_of::<c_int> () as ptrdiff_t
			    + std::mem::size_of::<ptrdiff_t> () as ptrdiff_t) as *const c_void,
		  std::mem::size_of::<ptrdiff_t> ());
	  memcpy (bidi_cache_max_elts.as_mut_ptr () as *mut c_void,
		  p.offset (std::mem::size_of::<ptrdiff_t> () as ptrdiff_t
			    + (*bidi_cache_idx.as_mut_ptr ()) * std::mem::size_of::<bidi_it> () as ptrdiff_t
			    + std::mem::size_of::<[ptrdiff_t; IT_STACK_SIZE as usize]> ()
			      as ptrdiff_t
			    + std::mem::size_of::<c_int> () as ptrdiff_t
			    + std::mem::size_of::<ptrdiff_t> () as ptrdiff_t
			    + std::mem::size_of::<ptrdiff_t> () as ptrdiff_t) as *const c_void,
		  std::mem::size_of::<ptrdiff_t> ());
	  (*bidi_cache_total_alloc.as_mut_ptr ())
	    -= (bidi_shelve_header_size
		+ (*bidi_cache_idx.as_mut_ptr ()) as usize * std::mem::size_of::<bidi_it> ()) as ptrdiff_t;
	}

      xfree (p as *mut c_void);
    }

  }
}

/***********************************************************************
				Initialization
 ***********************************************************************/
unsafe fn bidi_initialize ()
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  (*bidi_type_table.as_mut_ptr ()) = uniprop_table (intern (c"bidi-class".as_ptr ()));
  if NILP ((*bidi_type_table.as_mut_ptr ()))
    { emacs_abort (); }
  staticpro (bidi_type_table.as_ptr ());

  (*bidi_mirror_table.as_mut_ptr ()) = uniprop_table (intern (c"mirroring".as_ptr ()));
  if NILP ((*bidi_mirror_table.as_mut_ptr ()))
    { emacs_abort (); }
  staticpro (bidi_mirror_table.as_ptr ());

  (*bidi_brackets_table.as_mut_ptr ()) = uniprop_table (intern (c"bracket-type".as_ptr ()));
  if NILP ((*bidi_brackets_table.as_mut_ptr ()))
    { emacs_abort (); }
  staticpro (bidi_brackets_table.as_ptr ());

  (*paragraph_start_re.as_mut_ptr ()) = build_string (c"^\\(\u{c}\\|[ \t]*\\)$".as_ptr ());
  staticpro (paragraph_start_re.as_ptr ());
  (*paragraph_separate_re.as_mut_ptr ()) = build_string (c"^[ \t\u{c}]*$".as_ptr ());
  staticpro (paragraph_separate_re.as_ptr ());

  (*bidi_cache_sp.as_mut_ptr ()) = 0;
  (*bidi_cache_total_alloc.as_mut_ptr ()) = 0;
  (*bidi_cache_max_elts.as_mut_ptr ()) = BIDI_CACHE_MAX_ELTS_PER_SLOT;

  (*bidi_initialized.as_mut_ptr ()) = true;

  }
}

/* Do whatever UAX#9 clause X8 says should be done at paragraph's
   end.  */
unsafe fn bidi_set_paragraph_end (bidi_it: *mut bidi_it)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  (*bidi_it).invalid_levels = 0;
  (*bidi_it).invalid_isolates = 0;
  (*bidi_it).stack_idx = 0;
  (*bidi_it).isolate_level = 0;
  (*bidi_it).resolved_level = (*bidi_it).level_stack[0].level as i8;

  }
}

/* Initialize the bidi iterator from buffer/string position CHARPOS.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn bidi_init_it (charpos: ptrdiff_t, bytepos: ptrdiff_t,
				       frame_window_p: bool, bidi_it: *mut bidi_it)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  if !(*bidi_initialized.as_mut_ptr ())
    { bidi_initialize (); }
  if charpos >= 0
    { (*bidi_it).charpos = charpos; }
  if bytepos >= 0
    { (*bidi_it).bytepos = bytepos; }
  bidi_it_bits! (frame_window_p, bidi_it, frame_window_p);
  (*bidi_it).nchars = -1;	/* to be computed in bidi_resolve_explicit */
  bidi_it_bits! (first_elt, bidi_it, 1);
  bidi_set_paragraph_end (bidi_it);
  bidi_it_bits! (new_paragraph, bidi_it, 1);
  (*bidi_it).separator_limit = -1;
  (*bidi_it).r#type = NEUTRAL_B;
  (*bidi_it).type_after_wn = NEUTRAL_B;
  (*bidi_it).orig_type = NEUTRAL_B;
  /* FIXME: Review this!!! */
  (*bidi_it).prev.orig_type = UNKNOWN_BT;
  (*bidi_it).prev.r#type = UNKNOWN_BT;
  (*bidi_it).last_strong.orig_type = UNKNOWN_BT;
  (*bidi_it).last_strong.r#type = UNKNOWN_BT;
  (*bidi_it).next_for_neutral.charpos = -1;
  (*bidi_it).next_for_neutral.orig_type = UNKNOWN_BT;
  (*bidi_it).next_for_neutral.r#type = UNKNOWN_BT;
  (*bidi_it).prev_for_neutral.charpos = -1;
  (*bidi_it).prev_for_neutral.orig_type = UNKNOWN_BT;
  (*bidi_it).prev_for_neutral.r#type = UNKNOWN_BT;
  (*bidi_it).bracket_pairing_pos = -1;
  (*bidi_it).sos = L2R;	 /* FIXME: should it be user-selectable? */
  (*bidi_it).disp_pos = -1;	/* invalid/unknown */
  (*bidi_it).disp_prop = 0;
  /* We can only shrink the cache if we are at the bottom level of its
     "stack".  */
  if (*bidi_cache_start.as_mut_ptr ()) == 0
    { bidi_cache_shrink (); }
  else
    { bidi_cache_reset (); }

  }
}

/* Perform initializations for reordering a new line of bidi text.  */
unsafe fn bidi_line_init (bidi_it: *mut bidi_it)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  (*bidi_it).scan_dir = 1; /* FIXME: do we need to have control on this? */
  (*bidi_it).stack_idx = 0;
  (*bidi_it).resolved_level = (*bidi_it).level_stack[0].level as i8;
  (*bidi_it).level_stack[0].flags = 0; /* NEUTRAL_DIR, false per X1 */
  (*bidi_it).invalid_levels = 0;
  (*bidi_it).isolate_level = 0;	 /* X1 */
  (*bidi_it).invalid_isolates = 0; /* X1 */
  /* Setting this to zero will force its recomputation the first time
     we need it for W5.  */
  (*bidi_it).next_en_pos = 0;
  (*bidi_it).next_en_type = UNKNOWN_BT;
  (*bidi_it).next_for_ws.charpos = -1;
  (*bidi_it).next_for_ws.r#type = UNKNOWN_BT;
  (*bidi_it).bracket_pairing_pos = -1;
  bidi_set_sos_type (bidi_it,
		     if (*bidi_it).paragraph_dir == R2L { 1 } else { 0 },
		     (*bidi_it).level_stack[0].level as c_int); /* X10 */

  bidi_cache_reset ();

  }
}

/***********************************************************************
			Fetching characters
 ***********************************************************************/

/* Count bytes in string S between BEG/BEGBYTE and END.  BEG and END
   are zero-based character positions in S, BEGBYTE is byte position
   corresponding to BEG.  UNIBYTE means S is a unibyte string.  */
unsafe fn bidi_count_bytes (s: *const u8, beg: ptrdiff_t, begbyte: ptrdiff_t,
			    end: ptrdiff_t, unibyte: bool) -> ptrdiff_t
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut pos: ptrdiff_t = beg;
  let mut p: *const u8 = s.offset (begbyte);
  let start: *const u8 = p;

  if unibyte
    { p = s.offset (end); }
  else
    {
      if !CHAR_HEAD_P (*p as c_int)
	{ emacs_abort (); }

      while pos < end
	{
	  p = p.offset (BYTES_BY_CHAR_HEAD (*p as c_int) as ptrdiff_t);
	  pos += 1;
	}
    }

  return p.offset_from (start);

  }
}

/* Fetch and return the character at byte position BYTEPOS.  If S is
   non-NULL, fetch the character from string S; otherwise fetch the
   character from the current buffer.  UNIBYTE means S is a
   unibyte string.  */
unsafe fn bidi_char_at_pos (bytepos: ptrdiff_t, s: *const u8, unibyte: bool) -> c_int
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut s: *const u8 = s;

  if !s.is_null ()
    {
      s = s.offset (bytepos);
      if unibyte
	{ return *s as c_int; }
    }
  else
    { s = BYTE_POS_ADDR (bytepos); }
  return STRING_CHAR (s);

  }
}

/* Fetch and return the character at CHARPOS/BYTEPOS. ...  */
unsafe fn bidi_fetch_char (charpos: ptrdiff_t, bytepos: ptrdiff_t,
			   disp_pos: *mut ptrdiff_t, disp_prop: *mut c_int,
			   string: *mut bidi_string_data, w: *mut c_void,
			   frame_window_p: bool, ch_len: *mut ptrdiff_t,
			   nchars: *mut ptrdiff_t) -> c_int
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  /* C's `int ch;` is assigned on every path (PORTING.md 1.5 zero-initialises it).  */
  let mut ch: c_int = 0;
  let endpos: ptrdiff_t
    = if !(*string).s.is_null () || STRINGP ((*string).lstring)
      { (*string).schars } else { ZV! () };
  let mut pos: text_pos = std::mem::zeroed ();
  /* C's `normal_char:` label, reached by `goto normal_char` from the display-string
     arm: the port runs that arm's body as an entry stage after the chain, exactly as
     C's `goto` reaches the label at the top of the `else` arm (PORTING.md 2.2).  */
  let mut normal_char: bool = false;

  /* If we got past the last known position of display string, compute
     the position of the next one.  That position could be at CHARPOS.  */
  if charpos < endpos && charpos > *disp_pos
    {
      SET_TEXT_POS! (pos, charpos, bytepos);
      *disp_pos = compute_display_string_pos (&raw mut pos, string, w, frame_window_p,
					      disp_prop);
      /* The factor of 100 below is a heuristic that needs to be
	 tuned.  It means we consider 100 buffer positions examined by
	 the above call roughly equivalent to the display engine
	 iterating over a single buffer position.  */
      if (*max_redisplay_ticks.as_mut_ptr ()) > 0 && *disp_pos > charpos
	{ update_redisplay_ticks (((*disp_pos - charpos) / 100 + 1) as c_int, w); }
    }

  /* Fetch the character at BYTEPOS.  */
  if charpos >= endpos
    {
      ch = BIDI_EOB;
      *ch_len = 1;
      *nchars = 1;
      *disp_pos = endpos;
      *disp_prop = 0;
    }
  else if charpos >= *disp_pos && *disp_prop != 0
    {
      let mut disp_end_pos: ptrdiff_t;

      /* We don't expect to find ourselves in the middle of a display
	 property.  Hopefully, it will never be needed.  */
      if charpos > *disp_pos
	{ emacs_abort (); }
      /* Text covered by `display' properties and overlays with
	 display properties or display strings is handled as a single
	 character that represents the entire run of characters
	 covered by the display property.  */
      if *disp_prop == 2
	{
	  /* `(space ...)' display specs are handled as paragraph
	     separators for the purposes of the reordering; see UAX#9
	     section 3 and clause HL1 in section 4.3 there.  */
	  ch = PARAGRAPH_SEPARATOR;
	}
      else
	{
	  /* All other display specs are handled as the Unicode Object
	     Replacement Character.  */
	  ch = OBJECT_REPLACEMENT_CHARACTER;
	}
      disp_end_pos = compute_display_string_end (*disp_pos, string, w);
      if disp_end_pos < 0
	{
	  /* Somebody removed the display string from the buffer
	     behind our back.  Recover by processing this buffer
	     position as if no display property were present there to
	     begin with.  */
	  *disp_prop = 0;
	  normal_char = true;
	}
      else
	{
	  *nchars = disp_end_pos - *disp_pos;
	  if *nchars <= 0
	    { emacs_abort (); }
	  if !(*string).s.is_null ()
	    { *ch_len = bidi_count_bytes ((*string).s, *disp_pos, bytepos,
					  disp_end_pos,
					  bidi_string_bits! (unibyte, string)); }
	  else if STRINGP ((*string).lstring)
	    { *ch_len = bidi_count_bytes (SDATA ((*string).lstring), *disp_pos,
					  bytepos, disp_end_pos,
					  bidi_string_bits! (unibyte, string)); }
	  else
	    { *ch_len = CHAR_TO_BYTE (disp_end_pos) - bytepos; }
	}
    }
  else
    { normal_char = true; }

  if normal_char
    {
    /* normal_char: */
      if !(*string).s.is_null ()
	{
	  if !bidi_string_bits! (unibyte, string)
	    {
	      /* C's `int len;`, filled through the pointer by the callee (PORTING.md 1.5).  */
	      let mut len: c_int = 0;
	      ch = string_char_and_length ((*string).s.offset (bytepos), &raw mut len);
	      *ch_len = len as ptrdiff_t;
	    }
	  else
	    {
	      ch = UNIBYTE_TO_CHAR (*(*string).s.offset (bytepos) as c_int);
	      *ch_len = 1;
	    }
	}
      else if STRINGP ((*string).lstring)
	{
	  if !bidi_string_bits! (unibyte, string)
	    {
	      /* C's `int len;`, see above.  */
	      let mut len: c_int = 0;
	      ch = string_char_and_length ((SDATA ((*string).lstring)).offset (bytepos),
					   &raw mut len);
	      *ch_len = len as ptrdiff_t;
	    }
	  else
	    {
	      ch = UNIBYTE_TO_CHAR (SREF ((*string).lstring, bytepos as usize) as c_int);
	      *ch_len = 1;
	    }
	}
      else
	{
	  /* C's `int len;`, see above.  */
	  let mut len: c_int = 0;
	  ch = string_char_and_length (BYTE_POS_ADDR (bytepos), &raw mut len);
	  *ch_len = len as ptrdiff_t;
	}

      *nchars = 1;
    }

  /* If we just entered a run of characters covered by a display
     string, compute the position of the next display string.  */
  if charpos + *nchars <= endpos && charpos + *nchars > *disp_pos
      && *disp_prop != 0
    {
      SET_TEXT_POS! (pos, charpos + *nchars, bytepos + *ch_len);
      *disp_pos = compute_display_string_pos (&raw mut pos, string, w, frame_window_p,
					      disp_prop);
      if (*max_redisplay_ticks.as_mut_ptr ()) > 0 && *disp_pos > charpos + *nchars
	{ update_redisplay_ticks (((*disp_pos - charpos - *nchars) / 100 + 1) as c_int, w); }
    }

  return ch;

  }
}

/* Like bidi_fetch_char, but ignore any text between an isolate
   initiator and its matching PDI or, if it has no matching PDI, the
   end of the paragraph.  If isolates were skipped, CH_LEN and NCHARS
   are set to the number of bytes and characters between BYTEPOS/CHARPOS
   and the character that was fetched after skipping the isolates.  */
unsafe fn bidi_fetch_char_skip_isolates (charpos: ptrdiff_t, bytepos: ptrdiff_t,
					 disp_pos: *mut ptrdiff_t,
					 disp_prop: *mut c_int,
					 string: *mut bidi_string_data,
					 w: *mut c_void, frame_window_p: bool,
					 ch_len: *mut ptrdiff_t,
					 nchars: *mut ptrdiff_t) -> c_int
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let orig_charpos: ptrdiff_t = charpos;
  let orig_bytepos: ptrdiff_t = bytepos;
  let mut charpos: ptrdiff_t = charpos;
  let mut bytepos: ptrdiff_t = bytepos;
  let mut ch: c_int = bidi_fetch_char (charpos, bytepos, disp_pos, disp_prop, string, w,
				       frame_window_p, ch_len, nchars);
  let mut ch_type: bidi_type_t = bidi_get_type (ch, NEUTRAL_DIR);
  let mut level: ptrdiff_t = 0;

  if ch_type == LRI || ch_type == RLI || ch_type == FSI
    {
      level += 1;
      while level > 0 && ch_type != NEUTRAL_B
	{
	  charpos += *nchars;
	  bytepos += *ch_len;
	  ch = bidi_fetch_char (charpos, bytepos, disp_pos, disp_prop, string,
				w, frame_window_p, ch_len, nchars);
	  ch_type = bidi_get_type (ch, NEUTRAL_DIR);
	  /* A Note to P2 says to ignore max_depth limit.  */
	  if ch_type == LRI || ch_type == RLI || ch_type == FSI
	    { level += 1; }
	  else if ch_type == PDI
	    { level -= 1; }
	}
    }

  /* Communicate to the caller how much did we skip, so it could get
     past the last character position we examined.  */
  *nchars += charpos - orig_charpos;
  *ch_len += bytepos - orig_bytepos;
  return ch;

  }
}

/***********************************************************************
			Determining paragraph direction
 ***********************************************************************/

/* Check if buffer position CHARPOS/BYTEPOS is the end of a paragraph.
   Value is the non-negative length of the paragraph separator
   following the buffer position, -1 if position is at the beginning
   of a new paragraph, or -2 if position is neither at beginning nor
   at end of a paragraph.  */
unsafe fn bidi_at_paragraph_end (charpos: ptrdiff_t, bytepos: ptrdiff_t) -> ptrdiff_t
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut sep_re: Lisp_Object;
  let mut start_re: Lisp_Object;
  let mut val: ptrdiff_t;

  if STRINGP (BVAR! ((current_buffer! ()), bidi_paragraph_separate_re_))
    { sep_re = BVAR! ((current_buffer! ()), bidi_paragraph_separate_re_); }
  else
    { sep_re = (*paragraph_separate_re.as_mut_ptr ()); }
  if STRINGP (BVAR! ((current_buffer! ()), bidi_paragraph_start_re_))
    { start_re = BVAR! ((current_buffer! ()), bidi_paragraph_start_re_); }
  else
    { start_re = (*paragraph_start_re.as_mut_ptr ()); }

  /* Prevent quitting inside re_match_2, as redisplay_window could
     have temporarily moved point.  */
  let count: specpdl_ref = SPECPDL_INDEX ();
  specbind (Qinhibit_quit (), Qt ());

  val = fast_looking_at (sep_re, charpos, bytepos, ZV! (), ZV_BYTE! (), Qnil ());
  if val < 0
    {
      if fast_looking_at (start_re, charpos, bytepos, ZV! (), ZV_BYTE! (), Qnil ()) >= 0
	{ val = -1; }
      else
	{ val = -2; }
    }

  unbind_to (count, Qnil ());
  return val;

  }
}

/* If the user has requested the long scans caching, make sure that
   BIDI cache is enabled.  Otherwise, make sure it's disabled.  */

unsafe fn bidi_paragraph_cache_on_off () -> *mut c_void
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut cache_buffer: *mut buffer = (current_buffer! ());
  let mut indirect_p: bool = false;

  /* For indirect buffers, make sure to use the cache of their base
     buffer.  */
  if !(*cache_buffer).base_buffer.is_null ()
    {
      cache_buffer = (*cache_buffer).base_buffer;
      indirect_p = true;
    }

  /* Don't turn on or off the cache in the base buffer, if the value
     of cache-long-scans of the base buffer is inconsistent with that.
     This is because doing so will just make the cache pure overhead,
     since if we turn it on via indirect buffer, it will be
     immediately turned off by its base buffer.  */
  if NILP (BVAR! ((current_buffer! ()), cache_long_scans_))
    {
      if !indirect_p
	  || NILP (BVAR! (cache_buffer, cache_long_scans_))
	{
	  if !(*cache_buffer).bidi_paragraph_cache.is_null ()
	    {
	      free_region_cache ((*cache_buffer).bidi_paragraph_cache);
	      (*cache_buffer).bidi_paragraph_cache = std::ptr::null_mut ();
	    }
	}
      return std::ptr::null_mut ();
    }
  else
    {
      if !indirect_p
	  || !NILP (BVAR! (cache_buffer, cache_long_scans_))
	{
	  if (*cache_buffer).bidi_paragraph_cache.is_null ()
	    { (*cache_buffer).bidi_paragraph_cache = new_region_cache (); }
	}
      return (*cache_buffer).bidi_paragraph_cache;
    }

  }
}

/* On my 2005-vintage machine, searching back for paragraph start
   takes ~1 ms per line.  And bidi_paragraph_init is called 4 times
   when user types C-p.  The number below limits each call to
   bidi_paragraph_init to about 10 ms.  */
const MAX_PARAGRAPH_SEARCH: ptrdiff_t = 7500;

/* Find the beginning of this paragraph by looking back in the buffer.
   Value is the byte position of the paragraph's beginning, or
   BEGV_BYTE if paragraph_start_re is still not found after looking
   back MAX_PARAGRAPH_SEARCH lines in the buffer.  */
unsafe fn bidi_find_paragraph_start (pos: ptrdiff_t, pos_byte: ptrdiff_t)
				     -> ptrdiff_t
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut pos: ptrdiff_t = pos;
  let mut pos_byte: ptrdiff_t = pos_byte;
  let re: Lisp_Object
    = if STRINGP (BVAR! ((current_buffer! ()), bidi_paragraph_start_re_))
      { BVAR! ((current_buffer! ()), bidi_paragraph_start_re_) }
      else { (*paragraph_start_re.as_mut_ptr ()) };
  let limit: ptrdiff_t = ZV! ();
  let limit_byte: ptrdiff_t = ZV_BYTE! ();
  let bpc: *mut c_void = bidi_paragraph_cache_on_off ();
  let mut n: ptrdiff_t = 0;
  let oldpos: ptrdiff_t = pos;
  let mut next: ptrdiff_t = 0;
  let mut cache_buffer: *mut buffer = (current_buffer! ());

  if !(*cache_buffer).base_buffer.is_null ()
    { cache_buffer = (*cache_buffer).base_buffer; }

  /* Prevent quitting inside re_match_2, as redisplay_window could
     have temporarily moved point.  */
  let count: specpdl_ref = SPECPDL_INDEX ();
  specbind (Qinhibit_quit (), Qt ());

  while pos_byte > BEGV_BYTE! ()
	 /* C's `n++` compares the value before the increment, and the two `&&`s
	    short-circuit left to right.  */
	 && { let n_old: ptrdiff_t = n; n += 1; n_old } < MAX_PARAGRAPH_SEARCH
	 && fast_looking_at (re, pos, pos_byte, limit, limit_byte, Qnil ()) < 0
    {
      /* FIXME: What if the paragraph beginning is covered by a
	 display string?  And what if a display string covering some
	 of the text over which we scan back includes
	 paragraph_start_re?  */
      dec_both (&raw mut pos, &raw mut pos_byte);
      if !bpc.is_null ()
	  && region_cache_backward (cache_buffer, bpc, pos, &raw mut next) != 0
	{
	  pos = next;
	  pos_byte = CHAR_TO_BYTE (pos);
	  if pos_byte == BEGV_BYTE! ()
	      || fast_looking_at (re, pos, pos_byte, limit, limit_byte, Qnil ()) >= 0
	    { break; }
	}
      else
	{ pos = find_newline_no_quit (pos, pos_byte, -1, &raw mut pos_byte); }
    }
  unbind_to (count, Qnil ());
  if n >= MAX_PARAGRAPH_SEARCH
    {
      pos = BEGV! ();
      pos_byte = BEGV_BYTE! ();
    }
  if !bpc.is_null ()
    { know_region_cache (cache_buffer, bpc, pos, oldpos); }
  /* Positions returned by the region cache are not limited to
     BEGV..ZV range, so we limit them here.  */
  pos_byte = clip_to_bounds (BEGV_BYTE! (), pos_byte, ZV_BYTE! ());
  return pos_byte;

  }
}

/* This tracks how far we needed to search for first strong character.  */
static nsearch_for_strong: VmGlobal<ptrdiff_t> = VmGlobal::<ptrdiff_t>::new (0);

/* On a 3.4 GHz machine, searching forward for a strong directional
   character in a long paragraph full of weaks or neutrals takes about
   1 ms for each 20K characters.  The number below limits each call to
   bidi_paragraph_init to less than 10 ms even on slow machines.  */
const MAX_STRONG_CHAR_SEARCH: ptrdiff_t = 100000;

/* Starting from POS, find the first strong (L, R, or AL) character,
   while skipping over any characters between an isolate initiator and
   its matching PDI.  STOP_AT_PDI non-zero means stop at the PDI that
   matches the isolate initiator at POS.  Return the bidi type of the
   character where the search stopped.  Give up if after examining
   MAX_STRONG_CHAR_SEARCH buffer or string positions no strong
   character was found.  */
unsafe fn find_first_strong_char (pos: ptrdiff_t, bytepos: ptrdiff_t,
				  end: ptrdiff_t, disp_pos: *mut ptrdiff_t,
				  disp_prop: *mut c_int,
				  string: *mut bidi_string_data, w: *mut c_void,
				  string_p: bool, frame_window_p: bool,
				  ch_len: *mut ptrdiff_t, nchars: *mut ptrdiff_t,
				  stop_at_pdi: bool) -> bidi_type_t
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut pos: ptrdiff_t = pos;
  let mut bytepos: ptrdiff_t = bytepos;
  let mut pos1: ptrdiff_t;
  let mut r#type: bidi_type_t;
  let mut ch: c_int;

  if stop_at_pdi
    {
      /* If STOP_AT_PDI is non-zero, we must have been called with FSI
	 at POS.  Get past it.  */
      #[cfg(enable_checking)]
      {
	ch = bidi_fetch_char (pos, bytepos, disp_pos, disp_prop, string, w,
			      frame_window_p, ch_len, nchars);
	r#type = bidi_get_type (ch, NEUTRAL_DIR);
	eassert! (r#type == FSI /* || type == LRI || type == RLI */);
      }
      pos += *nchars;
      bytepos += *ch_len;
    }
  ch = bidi_fetch_char_skip_isolates (pos, bytepos, disp_pos, disp_prop, string,
				      w, frame_window_p, ch_len, nchars);
  r#type = bidi_get_type (ch, NEUTRAL_DIR);

  pos1 = pos;
  /* C's `for (pos += *nchars, bytepos += *ch_len; COND; type = …)`: the init runs
     once here, the step at the end of every iteration that does not `break`, and
     the condition before every iteration (PORTING.md 2.2).  */
  pos += *nchars;
  bytepos += *ch_len;
  loop
    {
      if !(bidi_get_category (r#type) != STRONG
	   /* If requested to stop at first PDI, stop there.  */
	   && !(stop_at_pdi && r#type == PDI)
	   /* Stop when searched too far into an abnormally large
	      paragraph full of weak or neutral characters.  */
	   && pos - pos1 < MAX_STRONG_CHAR_SEARCH)
	{ break; }
      if pos >= end
	{
	  /* Pretend there's a paragraph separator at end of
	     buffer/string.  */
	  r#type = NEUTRAL_B;
	  break;
	}
      if !string_p
	  && r#type == NEUTRAL_B
	  && bidi_at_paragraph_end (pos, bytepos) >= -1
	{ break; }
      /* Fetch next character and advance to get past it.  */
      ch = bidi_fetch_char_skip_isolates (pos, bytepos, disp_pos, disp_prop,
					  string, w, frame_window_p,
					  ch_len, nchars);
      pos += *nchars;
      bytepos += *ch_len;
      r#type = bidi_get_type (ch, NEUTRAL_DIR);
    }

  (*nsearch_for_strong.as_mut_ptr ()) += pos - pos1;
  return r#type;

  }
}

/* Determine the base direction, a.k.a. base embedding level, of the
   paragraph we are about to iterate through. ...  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn bidi_paragraph_init (dir: bidi_dir_t, bidi_it: *mut bidi_it,
					      no_default_p: bool)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut bytepos: ptrdiff_t = (*bidi_it).bytepos;
  let string_p: bool = !(*bidi_it).string.s.is_null ()
		       || STRINGP ((*bidi_it).string.lstring);
  let mut pstartbyte: ptrdiff_t;
  /* Note that begbyte is a byte position, while end is a character
     position.  Yes, this is ugly, but we are trying to avoid costly
     calls to BYTE_TO_CHAR and its ilk.  */
  let begbyte: ptrdiff_t = if string_p { 0 } else { BEGV_BYTE! () };
  let end: ptrdiff_t = if string_p { (*bidi_it).string.schars } else { ZV! () };
  let mut pos: ptrdiff_t = (*bidi_it).charpos;
  let mut dir: bidi_dir_t = dir;

  (*nsearch_for_strong.as_mut_ptr ()) = 0;

  /* Special case for an empty buffer. */
  if bytepos == begbyte && (*bidi_it).charpos == end
    { dir = L2R; }
  /* We should never be called at EOB or before BEGV.  */
  else if (*bidi_it).charpos >= end || bytepos < begbyte
    { emacs_abort (); }

  if dir == L2R
    {
      (*bidi_it).paragraph_dir = L2R;
      bidi_it_bits! (new_paragraph, bidi_it, 0);
    }
  else if dir == R2L
    {
      (*bidi_it).paragraph_dir = R2L;
      bidi_it_bits! (new_paragraph, bidi_it, 0);
    }
  else if dir == NEUTRAL_DIR	/* P2 */
    {
      let mut ch_len: ptrdiff_t = 0;
      let mut nchars: ptrdiff_t = 0;
      let mut disp_pos: ptrdiff_t = -1;
      let mut disp_prop: c_int = 0;
      let mut r#type: bidi_type_t = UNKNOWN_BT;
      let s: *const u8;

      if !(*bidi_initialized.as_mut_ptr ())
	{ bidi_initialize (); }

      /* If we are inside a paragraph separator, we are just waiting
	 for the separator to be exhausted; use the previous paragraph
	 direction.  But don't do that if we have been just reseated,
	 because we need to reinitialize below in that case.  */
      if !bidi_it_bits! (first_elt, bidi_it)
	  && (*bidi_it).charpos < (*bidi_it).separator_limit
	{ return; }

      /* If we are on a newline, get past it to where the next
	 paragraph might start.  But don't do that at BEGV since then
	 we are potentially in a new paragraph that doesn't yet
	 exist.  */
      pos = (*bidi_it).charpos;
      s = if STRINGP ((*bidi_it).string.lstring)
	  { SDATA ((*bidi_it).string.lstring) }
	  else { (*bidi_it).string.s };
      if bytepos > begbyte
	  && bidi_char_at_pos (bytepos, s, bidi_string_bits! (unibyte, &raw mut (*bidi_it).string)) == '\n' as c_int
	{
	  bytepos += 1;
	  pos += 1;
	}

      /* We are either at the beginning of a paragraph or in the
	 middle of it.  Find where this paragraph starts.  */
      if string_p
	{
	  /* We don't support changes of paragraph direction inside a
	     string.  It is treated as a single paragraph.  */
	  pstartbyte = 0;
	}
      else
	{ pstartbyte = bidi_find_paragraph_start (pos, bytepos); }
      (*bidi_it).separator_limit = -1;
      bidi_it_bits! (new_paragraph, bidi_it, 0);

      /* The following loop is run more than once only if NO_DEFAULT_P,
	 and only if we are iterating on a buffer.  */
      loop {
	bytepos = pstartbyte;
	if !string_p
	  { pos = BYTE_TO_CHAR (bytepos); }
	r#type = find_first_strong_char (pos, bytepos, end, &raw mut disp_pos,
					 &raw mut disp_prop,
					 &raw mut (*bidi_it).string, (*bidi_it).w,
					 string_p, bidi_it_bits! (frame_window_p, bidi_it),
					 &raw mut ch_len, &raw mut nchars, false);
	if r#type == STRONG_R || r#type == STRONG_AL /* P3 */
	  { (*bidi_it).paragraph_dir = R2L; }
	else if r#type == STRONG_L
	  { (*bidi_it).paragraph_dir = L2R; }
	if !string_p
	    && no_default_p && (*bidi_it).paragraph_dir == NEUTRAL_DIR
	  {
	    /* If this paragraph is at BEGV, default to L2R.  */
	    if pstartbyte == BEGV_BYTE! ()
	      { (*bidi_it).paragraph_dir = L2R; } /* P3 and HL1 */
	    else
	      {
		let mut prevpbyte: ptrdiff_t = pstartbyte;
		let mut p: ptrdiff_t = BYTE_TO_CHAR (pstartbyte);
		let mut pbyte: ptrdiff_t = pstartbyte;

		/* Find the beginning of the previous paragraph, if any.  */
		while pbyte > BEGV_BYTE! () && prevpbyte >= pstartbyte
		  {
		    /* FXIME: What if p is covered by a display
		       string?  See also a FIXME inside
		       bidi_find_paragraph_start.  */
		    dec_both (&raw mut p, &raw mut pbyte);
		    prevpbyte = bidi_find_paragraph_start (p, pbyte);
		  }
		pstartbyte = prevpbyte;
	      }
	  }
	if !(!string_p
	     && no_default_p && (*bidi_it).paragraph_dir == NEUTRAL_DIR)
	  { break; }
      }
    }
  else
    { emacs_abort (); }

  /* Contrary to UAX#9 clause P3, we only default the paragraph
     direction to L2R if we have no previous usable paragraph
     direction.  This is allowed by the HL1 clause.  */
  if (*bidi_it).paragraph_dir != L2R && (*bidi_it).paragraph_dir != R2L
    { (*bidi_it).paragraph_dir = L2R; } /* P3 and HL1 ``higher-level protocols'' */
  if (*bidi_it).paragraph_dir == R2L
    { (*bidi_it).level_stack[0].level = 1; }
  else
    { (*bidi_it).level_stack[0].level = 0; }

  bidi_line_init (bidi_it);

  /* The factor of 50 below is a heuristic that needs to be tuned.  It
     means we consider 50 buffer positions examined by this function
     roughly equivalent to the display engine iterating over a single
     buffer position.  */
  let nexamined: ptrdiff_t = (*bidi_it).charpos - pos + (*nsearch_for_strong.as_mut_ptr ());
  if (*max_redisplay_ticks.as_mut_ptr ()) > 0 && nexamined > 0
    { update_redisplay_ticks ((nexamined / 50) as c_int, (*bidi_it).w); }

  }
}

/***********************************************************************
		 Resolving explicit and implicit levels.
  The rest of this file constitutes the core of the UBA implementation.
 ***********************************************************************/

unsafe fn bidi_explicit_dir_char (ch: c_int) -> bool
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let ch_type: bidi_type_t;

  if !(*bidi_initialized.as_mut_ptr ())
    { emacs_abort (); }
  if ch < 0
    {
      eassert! (ch == BIDI_EOB);
      return false;
    }
  ch_type = XFIXNUM (CHAR_TABLE_REF ((*bidi_type_table.as_mut_ptr ()), ch as isize)) as bidi_type_t;
  return ch_type == LRE || ch_type == LRO
	 || ch_type == RLE || ch_type == RLO
	 || ch_type == PDF;

  }
}

/* Given an iterator state in BIDI_IT, advance one character position
   in the buffer/string to the next character (in the logical order),
   resolve any explicit embeddings, directional overrides, and isolate
   initiators and terminators, and return the embedding level of the
   character after resolving these explicit directives.  */
unsafe fn bidi_resolve_explicit (bidi_it: *mut bidi_it) -> c_int
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut curchar: c_int;
  let mut r#type: bidi_type_t;
  let mut typ1: bidi_type_t;
  let mut prev_type: bidi_type_t = UNKNOWN_BT;
  let mut current_level: c_int;
  let mut new_level: c_int;
  let mut r#override: bidi_dir_t;
  let mut isolate_status: bool;
  let string_p: bool = !(*bidi_it).string.s.is_null ()
		       || STRINGP ((*bidi_it).string.lstring);
  let mut ch_len: ptrdiff_t = 0;
  let mut nchars: ptrdiff_t = 0;
  let mut disp_pos: ptrdiff_t = 0;
  let mut end: ptrdiff_t = 0;
  let mut disp_prop: c_int = 0;
  let eob: ptrdiff_t
    = if !(*bidi_it).string.s.is_null () || STRINGP ((*bidi_it).string.lstring)
      { (*bidi_it).string.schars } else { ZV! () };

  /* Record the info about the previous character.  */
  if (*bidi_it).type_after_wn != WEAK_BN /* W1/Retaining */
      && (*bidi_it).r#type != WEAK_BN
    {
      /* This special case is needed in support of Unicode 8.0
	 correction to N0, as implemented in bidi_resolve_weak/W1
	 below.  */
      if (*bidi_it).type_after_wn == NEUTRAL_ON
	  && bidi_get_category ((*bidi_it).r#type) == STRONG
	  && bidi_paired_bracket_type ((*bidi_it).ch) == BIDI_BRACKET_CLOSE
	{ bidi_remember_char (&raw mut (*bidi_it).prev, bidi_it, true); }
      else
	{ bidi_remember_char (&raw mut (*bidi_it).prev, bidi_it, false); }
    }
  if (*bidi_it).type_after_wn == STRONG_R
      || (*bidi_it).type_after_wn == STRONG_L
      || (*bidi_it).type_after_wn == STRONG_AL
    { bidi_remember_char (&raw mut (*bidi_it).last_strong, bidi_it, false); }
  if (*bidi_it).r#type == STRONG_R || (*bidi_it).r#type == STRONG_L
      || (*bidi_it).r#type == WEAK_EN || (*bidi_it).r#type == WEAK_AN
    { bidi_remember_char (&raw mut (*bidi_it).prev_for_neutral, bidi_it, true); }

  /* If we overstepped the characters used for resolving neutrals
     and whitespace, invalidate their info in the iterator.  */
  if (*bidi_it).charpos >= (*bidi_it).next_for_neutral.charpos
    {
      (*bidi_it).next_for_neutral.r#type = UNKNOWN_BT;
      /* If needed, reset the "magical" value of pairing bracket
	 position, so that bidi_resolve_brackets will resume
	 resolution of brackets according to BPA.  */
      if (*bidi_it).bracket_pairing_pos == eob
	{ (*bidi_it).bracket_pairing_pos = -1; }
    }
  if (*bidi_it).next_en_pos >= 0
      && (*bidi_it).charpos >= (*bidi_it).next_en_pos
    {
      (*bidi_it).next_en_pos = 0;
      (*bidi_it).next_en_type = UNKNOWN_BT;
    }

  /* Reset the bracket resolution info, unless we previously decided
     (in bidi_find_bracket_pairs) that brackets in this level run
     should be resolved as neutrals.  */
  if (*bidi_it).bracket_pairing_pos != eob
    {
      (*bidi_it).bracket_pairing_pos = -1;
      (*bidi_it).bracket_enclosed_type = UNKNOWN_BT;
    }

  /* If reseat()'ed, don't advance, so as to start iteration from the
     position where we were reseated.  bidi_it->bytepos can be less
     than BEGV_BYTE after reseat to BEGV.  */
  if (*bidi_it).bytepos < (if string_p { 0 } else { BEGV_BYTE! () })
      || bidi_it_bits! (first_elt, bidi_it)
    {
      bidi_it_bits! (first_elt, bidi_it, 0);
      if string_p
	{
	  let p: *const u8
	    = if STRINGP ((*bidi_it).string.lstring)
	      { SDATA ((*bidi_it).string.lstring) }
	      else { (*bidi_it).string.s };

	  if (*bidi_it).charpos < 0
	    {
	      (*bidi_it).charpos = 0;
	      (*bidi_it).bytepos = 0;
	    }
	  eassert! ((*bidi_it).bytepos
		    == bidi_count_bytes (p, 0, 0, (*bidi_it).charpos,
					 bidi_string_bits! (unibyte, &raw mut (*bidi_it).string)));
	}
      else
	{
	  if (*bidi_it).charpos < BEGV! ()
	    {
	      (*bidi_it).charpos = BEGV! ();
	      (*bidi_it).bytepos = BEGV_BYTE! ();
	    }
	  eassert! ((*bidi_it).bytepos == CHAR_TO_BYTE ((*bidi_it).charpos));
	}
      /* Determine the original bidi type of the previous character,
	 which is needed for handling isolate initiators and PDF.  The
	 type of the previous character will be non-trivial only if
	 our caller moved through some previous text in
	 get_visually_first_element, in which case bidi_it->prev holds
	 the information we want.  */
      if bidi_it_bits! (first_elt, bidi_it) && (*bidi_it).prev.r#type != UNKNOWN_BT
	{
	  eassert! ((*bidi_it).prev.charpos == (*bidi_it).charpos - 1);
	  prev_type = (*bidi_it).prev.orig_type;
	}
    }
  /* Don't move at end of buffer/string.  */
  else if (*bidi_it).charpos < (if string_p { (*bidi_it).string.schars } else { ZV! () })
    {
      /* Advance to the next character, skipping characters covered by
	 display strings (nchars > 1).  */
      if (*bidi_it).nchars <= 0
	{ emacs_abort (); }
      (*bidi_it).charpos += (*bidi_it).nchars;
      if (*bidi_it).ch_len == 0
	{ emacs_abort (); }
      (*bidi_it).bytepos += (*bidi_it).ch_len;
      prev_type = (*bidi_it).orig_type;
    }
  else	/* EOB or end of string */
    { prev_type = NEUTRAL_B; }

  current_level = (*bidi_it).level_stack[(*bidi_it).stack_idx as usize].level as c_int; /* X1 */
  isolate_status = ISOLATE_STATUS! (bidi_it, (*bidi_it).stack_idx) != 0;
  r#override = OVERRIDE! (bidi_it, (*bidi_it).stack_idx) as bidi_dir_t;
  new_level = current_level;

  if (*bidi_it).charpos
       >= (if string_p { (*bidi_it).string.schars } else { ZV! () })
    {
      curchar = BIDI_EOB;
      (*bidi_it).ch_len = 1;
      (*bidi_it).nchars = 1;
      (*bidi_it).disp_pos
	= if string_p { (*bidi_it).string.schars } else { ZV! () };
      (*bidi_it).disp_prop = 0;
    }
  else
    {
      /* LRI, RLI, and FSI increment, and PDF decrements, the
	 embedding level of the _following_ characters, so we must
	 first look at the type of the previous character to support
	 that.  */
      match INT_PROMOTE! (prev_type) /* promote to int to avoid warnings */
	{
	  RLI => {	/* X5a */
	    if current_level < BIDI_MAXDEPTH
		&& (*bidi_it).invalid_levels == 0
		&& (*bidi_it).invalid_isolates == 0
	      {
		new_level = ((current_level + 1) & !1) + 1;
		(*bidi_it).isolate_level += 1;
		bidi_push_embedding_level (bidi_it, new_level, NEUTRAL_DIR, true);
	      }
	    else
	      { (*bidi_it).invalid_isolates += 1; }
	  }
	  LRI => {	/* X5b */
	    if current_level < BIDI_MAXDEPTH - 1
		&& (*bidi_it).invalid_levels == 0
		&& (*bidi_it).invalid_isolates == 0
	      {
		new_level = ((current_level + 2) & !1);
		(*bidi_it).isolate_level += 1;
		bidi_push_embedding_level (bidi_it, new_level, NEUTRAL_DIR, true);
	      }
	    else
	      { (*bidi_it).invalid_isolates += 1; }
	  }
	  PDF => {	/* X7 */
	    if (*bidi_it).invalid_isolates == 0
	      {
		if (*bidi_it).invalid_levels != 0
		  { (*bidi_it).invalid_levels -= 1; }
		else if !isolate_status && (*bidi_it).stack_idx >= 1
		  { new_level = bidi_pop_embedding_level (bidi_it); }
	      }
	  }
	  _ => {
	    eassert! (prev_type != FSI);
	    /* Nothing.  */
	  }
	}
      /* Fetch the character at BYTEPOS.  If it is covered by a
	 display string, treat the entire run of covered characters as
	 a single character u+FFFC.  */
      curchar = bidi_fetch_char ((*bidi_it).charpos, (*bidi_it).bytepos,
				 &raw mut (*bidi_it).disp_pos,
				 &raw mut (*bidi_it).disp_prop,
				 &raw mut (*bidi_it).string, (*bidi_it).w,
				 bidi_it_bits! (frame_window_p, bidi_it),
				 &raw mut (*bidi_it).ch_len, &raw mut (*bidi_it).nchars);
    }
  (*bidi_it).ch = curchar;
  (*bidi_it).resolved_level = new_level as i8;

  /* Don't apply directional override here, as all the types we handle
     below will not be affected by the override anyway, and we need
     the original type unaltered.  The override will be applied in
     bidi_resolve_weak.  */
  r#type = bidi_get_type (curchar, NEUTRAL_DIR);
  (*bidi_it).orig_type = r#type;
  bidi_check_type ((*bidi_it).orig_type);

  (*bidi_it).type_after_wn = UNKNOWN_BT;

  /* C's `case FSI:` either falls through into `case RLI:`'s body or jumps into
     `case LRI:`'s with `goto fsi_as_lri`; the two textual copies of that shared body
     are kept (C 2171-2185), and the port dispatches between them by re-entering this
     loop with the arm name the `goto`/`FALLTHROUGH` selects — the entry-stage shape of
     PORTING.md 2.2 (the ccl.rs `ccl_repeat`/`ccl_goto` precedent).  */
  let mut arm: bidi_type_t = r#type;
  'fsi_as_lri: loop
    {
      match INT_PROMOTE! (arm) /* promote to int to avoid warnings */
	{
	  RLE | RLO => {	/* X2, X4 */
	    (*bidi_it).type_after_wn = r#type;
	    bidi_check_type ((*bidi_it).type_after_wn);
	    r#type = WEAK_BN; /* X9/Retaining */
	    if new_level < BIDI_MAXDEPTH
		&& (*bidi_it).invalid_levels == 0
		&& (*bidi_it).invalid_isolates == 0
	      {
		/* Compute the least odd embedding level greater than
		   the current level.  */
		new_level = ((new_level + 1) & !1) + 1;
		if (*bidi_it).type_after_wn == RLE
		  { r#override = NEUTRAL_DIR; }
		else
		  { r#override = R2L; }
		bidi_push_embedding_level (bidi_it, new_level, r#override, false);
		(*bidi_it).resolved_level = new_level as i8;
	      }
	    else
	      {
		if (*bidi_it).invalid_isolates == 0
		  { (*bidi_it).invalid_levels += 1; }
	      }
	    break 'fsi_as_lri;
	  }
	  LRE | LRO => {	/* X3, X5 */
	    (*bidi_it).type_after_wn = r#type;
	    bidi_check_type ((*bidi_it).type_after_wn);
	    r#type = WEAK_BN; /* X9/Retaining */
	    if new_level < BIDI_MAXDEPTH - 1
		&& (*bidi_it).invalid_levels == 0
		&& (*bidi_it).invalid_isolates == 0
	      {
		/* Compute the least even embedding level greater than
		   the current level.  */
		new_level = ((new_level + 2) & !1);
		if (*bidi_it).type_after_wn == LRE
		  { r#override = NEUTRAL_DIR; }
		else
		  { r#override = L2R; }
		bidi_push_embedding_level (bidi_it, new_level, r#override, false);
		(*bidi_it).resolved_level = new_level as i8;
	      }
	    else
	      {
		if (*bidi_it).invalid_isolates == 0
		  { (*bidi_it).invalid_levels += 1; }
	      }
	    break 'fsi_as_lri;
	  }
	  FSI => {		/* X5c */
	    end = if string_p { (*bidi_it).string.schars } else { ZV! () };
	    disp_pos = (*bidi_it).disp_pos;
	    disp_prop = (*bidi_it).disp_prop;
	    nchars = (*bidi_it).nchars;
	    ch_len = (*bidi_it).ch_len;
	    typ1 = find_first_strong_char ((*bidi_it).charpos,
					   (*bidi_it).bytepos, end,
					   &raw mut disp_pos, &raw mut disp_prop,
					   &raw mut (*bidi_it).string, (*bidi_it).w,
					   string_p,
					   bidi_it_bits! (frame_window_p, bidi_it),
					   &raw mut ch_len, &raw mut nchars, true);
	    if typ1 != STRONG_R && typ1 != STRONG_AL
	      {
		arm = LRI;
		r#type = LRI;
		/* Override orig_type, which will be needed when we come to
		   examine the next character, which is the first character
		   inside the isolate.  */
		(*bidi_it).orig_type = r#type;
	      }
	    else
	      {
		arm = RLI;
		r#type = RLI;
		(*bidi_it).orig_type = r#type;
	      }
	    /* FALLTHROUGH into `case RLI:`'s body, or `goto fsi_as_lri` into
	       `case LRI:`'s.  */
	    continue 'fsi_as_lri;
	  }
	  RLI => {		/* X5a */
	    if r#override == NEUTRAL_DIR
	      { (*bidi_it).type_after_wn = r#type; }
	    else	/* Unicode 8.0 correction.  */
	      { (*bidi_it).type_after_wn = if r#override == L2R { STRONG_L } else { STRONG_R }; }
	    bidi_check_type ((*bidi_it).type_after_wn);
	    break 'fsi_as_lri;
	  }
	  LRI => {		/* X5b */
	    /* fsi_as_lri: */
	    if r#override == NEUTRAL_DIR
	      { (*bidi_it).type_after_wn = r#type; }
	    else	/* Unicode 8.0 correction.  */
	      { (*bidi_it).type_after_wn = if r#override == L2R { STRONG_L } else { STRONG_R }; }
	    bidi_check_type ((*bidi_it).type_after_wn);
	    break 'fsi_as_lri;
	  }
	  PDI => {		/* X6a */
	    if (*bidi_it).invalid_isolates != 0
	      { (*bidi_it).invalid_isolates -= 1; }
	    else if (*bidi_it).isolate_level > 0
	      {
		(*bidi_it).invalid_levels = 0;
		while ISOLATE_STATUS! (bidi_it, (*bidi_it).stack_idx) == 0
		  { bidi_pop_embedding_level (bidi_it); }
		eassert! ((*bidi_it).stack_idx > 0);
		new_level = bidi_pop_embedding_level (bidi_it);
		(*bidi_it).isolate_level -= 1;
	      }
	    (*bidi_it).resolved_level = new_level as i8;
	    /* Unicode 8.0 correction.  */
	    {
	      let stack_override: bidi_dir_t = OVERRIDE! (bidi_it, (*bidi_it).stack_idx)
					       as bidi_dir_t;
	      if stack_override == L2R
		{ (*bidi_it).type_after_wn = STRONG_L; }
	      else if stack_override == R2L
		{ (*bidi_it).type_after_wn = STRONG_R; }
	      else
		{ (*bidi_it).type_after_wn = r#type; }
	    }
	    break 'fsi_as_lri;
	  }
	  PDF => {		/* X7 */
	    (*bidi_it).type_after_wn = r#type;
	    bidi_check_type ((*bidi_it).type_after_wn);
	    r#type = WEAK_BN; /* X9/Retaining */
	    break 'fsi_as_lri;
	  }
	  _ => {
	    /* Nothing.  */
	    break 'fsi_as_lri;
	  }
	}
    }

  (*bidi_it).r#type = r#type;
  bidi_check_type ((*bidi_it).r#type);

  if (*bidi_it).r#type == NEUTRAL_B	/* X8 */
    {
      bidi_set_paragraph_end (bidi_it);
      /* This is needed by bidi_resolve_weak below, and in L1.  */
      (*bidi_it).type_after_wn = (*bidi_it).r#type;
    }

  eassert! ((*bidi_it).resolved_level >= 0);
  return (*bidi_it).resolved_level as c_int;

  }
}

/* Advance in the buffer/string, resolve weak types and return the
   type of the next character after weak type resolution.  */
unsafe fn bidi_resolve_weak (bidi_it: *mut bidi_it) -> bidi_type_t
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut r#type: bidi_type_t;
  let mut r#override: bidi_dir_t;
  let prev_level: c_int = (*bidi_it).level_stack[(*bidi_it).stack_idx as usize].level
			  as c_int;
  let new_level: c_int = bidi_resolve_explicit (bidi_it);
  let mut next_char: c_int;
  let mut type_of_next: bidi_type_t;
  let mut saved_it: bidi_it = std::mem::zeroed ();
  let eob: ptrdiff_t
    = if STRINGP ((*bidi_it).string.lstring) || !(*bidi_it).string.s.is_null ()
      { (*bidi_it).string.schars } else { ZV! () };

  r#type = (*bidi_it).r#type;
  r#override = OVERRIDE! (bidi_it, (*bidi_it).stack_idx) as bidi_dir_t;

  eassert! (!(r#type == UNKNOWN_BT
	      || r#type == LRE
	      || r#type == LRO
	      || r#type == RLE
	      || r#type == RLO
	      || r#type == PDF));

  eassert! (prev_level >= 0);
  if (*bidi_it).r#type == NEUTRAL_B
    {
      /* We've got a new isolating sequence, compute the directional
         type of sos and initialize per-run variables (UAX#9, clause
         X10).  */
      bidi_set_sos_type (bidi_it, prev_level, new_level);
    }
  if r#type == NEUTRAL_S || r#type == NEUTRAL_WS
      || r#type == WEAK_BN || r#type == STRONG_AL
    { (*bidi_it).type_after_wn = r#type; }	/* needed in L1 */
  bidi_check_type ((*bidi_it).type_after_wn);

  /* Level and directional override status are already recorded in
     bidi_it, and do not need any change; see X6.  */
  if r#override == R2L		/* X6 */
    { r#type = STRONG_R; }
  else if r#override == L2R
    { r#type = STRONG_L; }
  else
    {
      if r#type == WEAK_NSM	/* W1 */
	{
	  /* Note that we don't need to consider the case where the
	     prev character has its type overridden by an RLO or LRO,
	     because then either the type of this NSM would have been
	     also overridden, or the previous character is outside the
	     current level run, and thus not relevant to this NSM.
	     This is why NSM gets the type_after_wn of the previous
	     character.  */
	  /* bidi_set_sos_type sets type_after_wn to UNKNOWN_BT.  */
	  if (*bidi_it).prev.r#type != UNKNOWN_BT
	      /* If type_after_wn is NEUTRAL_B, this NSM is at sos.  */
	      && (*bidi_it).prev.r#type != NEUTRAL_B
	    {
	      if bidi_isolate_fmt_char ((*bidi_it).prev.r#type)
		{
		  /* From W1: "Note that in an isolating run sequence,
		     an isolate initiator followed by an NSM or any
		     type other than PDI must be an overflow isolate
		     initiator."  */
		  eassert! ((*bidi_it).invalid_isolates > 0);
		  r#type = NEUTRAL_ON;
		}
	      else
		{
		  /* This includes the Unicode 8.0 correction for N0,
		     due to how we set prev.type in bidi_resolve_explicit,
		     which see.  */
		  r#type = (*bidi_it).prev.r#type;
		}
	    }
	  else if (*bidi_it).sos == R2L
	    { r#type = STRONG_R; }
	  else if (*bidi_it).sos == L2R
	    { r#type = STRONG_L; }
	  else /* shouldn't happen! */
	    { emacs_abort (); }
	}
      if r#type == WEAK_EN	/* W2 */
	  && (*bidi_it).last_strong.r#type == STRONG_AL
	{ r#type = WEAK_AN; }
      else if r#type == STRONG_AL /* W3 */
	{ r#type = STRONG_R; }
      else if (r#type == WEAK_ES	/* W4 */
	       && (*bidi_it).prev.r#type == WEAK_EN
	       && (*bidi_it).prev.orig_type == WEAK_EN)
	      || (r#type == WEAK_CS
		  && ((*bidi_it).prev.r#type == WEAK_EN
		      && (*bidi_it).prev.orig_type == WEAK_EN)
		  || (*bidi_it).prev.r#type == WEAK_AN)
	{
	  let s: *const u8
	    = if STRINGP ((*bidi_it).string.lstring)
	      { SDATA ((*bidi_it).string.lstring) }
	      else { (*bidi_it).string.s };

	  next_char = if (*bidi_it).charpos + (*bidi_it).nchars >= eob
		      { BIDI_EOB }
		      else { bidi_char_at_pos ((*bidi_it).bytepos + (*bidi_it).ch_len,
					       s,
					       bidi_string_bits! (unibyte, &raw mut (*bidi_it).string)) };
	  type_of_next = bidi_get_type (next_char, r#override);

	  if type_of_next == WEAK_BN
	      || bidi_explicit_dir_char (next_char)
	    {
	      bidi_copy_it (&raw mut saved_it, bidi_it);
	      while bidi_resolve_explicit (bidi_it) == new_level
		    && (*bidi_it).r#type == WEAK_BN
		{ type_of_next = (*bidi_it).r#type; }
	      bidi_copy_it (bidi_it, &raw mut saved_it);
	    }

	  /* If the next character is EN, but the last strong-type
	     character is AL, that next EN will be changed to AN when
	     we process it in W2 above.  So in that case, this ES
	     should not be changed into EN.  */
	  if r#type == WEAK_ES
	      && type_of_next == WEAK_EN
	      && (*bidi_it).last_strong.r#type != STRONG_AL
	    { r#type = WEAK_EN; }
	  else if r#type == WEAK_CS
	    {
	      if (*bidi_it).prev.r#type == WEAK_AN
		  && (type_of_next == WEAK_AN
		      /* If the next character is EN, but the last
			 strong-type character is AL, EN will be later
			 changed to AN when we process it in W2 above.
			 So in that case, this ES should not be
			 changed into EN.  */
		      || (type_of_next == WEAK_EN
			  && (*bidi_it).last_strong.r#type == STRONG_AL))
		{ r#type = WEAK_AN; }
	      else if (*bidi_it).prev.r#type == WEAK_EN
		       && type_of_next == WEAK_EN
		       && (*bidi_it).last_strong.r#type != STRONG_AL
		{ r#type = WEAK_EN; }
	    }
	}
      else if r#type == WEAK_ET	/* W5: ET with EN before or after it */
	       || r#type == WEAK_BN	/* W5/Retaining */
	{
	  if (*bidi_it).prev.r#type == WEAK_EN /* ET/BN w/EN before it */
	    { r#type = WEAK_EN; }
	  else if (*bidi_it).next_en_pos > (*bidi_it).charpos
		   && (*bidi_it).next_en_type != WEAK_BN
	    {
	      if (*bidi_it).next_en_type == WEAK_EN /* ET/BN with EN after it */
		{ r#type = WEAK_EN; }
	    }
	  else if r#type == WEAK_BN
		   /* This condition is for the following important case:

		      . we are at level zero
		      . either previous strong character was L,
			 or we've seen no strong characters since sos
			 and the base paragraph direction is L2R
		      . this BN is NOT a bidi directional control

		      For such a situation, either this BN will be
		      converted to EN per W5, and then to L by virtue
		      of W7; or it will become ON per W6, and then L
		      because of N1/N2.  So we take a shortcut here
		      and make it L right away, to avoid the
		      potentially costly loop below.  This is
		      important when the buffer has a long series of
		      control characters, like binary nulls, and no
		      R2L characters at all.  */
		   && new_level == 0
		   && !bidi_explicit_dir_char ((*bidi_it).ch)
		   && ((*bidi_it).last_strong.r#type == STRONG_L
		       || ((*bidi_it).last_strong.r#type == UNKNOWN_BT
			   && (*bidi_it).sos == L2R))
	    { r#type = STRONG_L; }
	  else if (*bidi_it).next_en_pos >= 0
	    {
	      /* We overstepped the last known position for ET
		 resolution but there could be other such characters
		 in this paragraph (when we are sure there are no more
		 such positions, we set next_en_pos to a negative
		 value).  Try to find the next position for ET
		 resolution.  */
	      let mut en_pos: ptrdiff_t = (*bidi_it).charpos + (*bidi_it).nchars;
	      let s: *const u8 = if STRINGP ((*bidi_it).string.lstring)
				 { SDATA ((*bidi_it).string.lstring) }
				 else { (*bidi_it).string.s };

	      if (*bidi_it).nchars <= 0
		{ emacs_abort (); }
	      next_char = if (*bidi_it).charpos + (*bidi_it).nchars >= eob
			  { BIDI_EOB }
			  else { bidi_char_at_pos ((*bidi_it).bytepos + (*bidi_it).ch_len,
						   s,
						   bidi_string_bits! (unibyte, &raw mut (*bidi_it).string)) };
	      type_of_next = bidi_get_type (next_char, r#override);

	      if type_of_next == WEAK_ET
		  || type_of_next == WEAK_BN
		  || bidi_explicit_dir_char (next_char)
		{
		  bidi_copy_it (&raw mut saved_it, bidi_it);
		  while bidi_resolve_explicit (bidi_it) == new_level
			&& ((*bidi_it).r#type == WEAK_BN
			    || (*bidi_it).r#type == WEAK_ET)
		    { type_of_next = (*bidi_it).r#type; }
		  if r#type == WEAK_BN
		      && (*bidi_it).charpos == saved_it.charpos + saved_it.nchars
		    {
		      /* If we entered the above loop with a BN that
			 changes the level, the type of next
			 character, which is in a different level, is
			 not relevant to resolving this series of ET
			 and BN.  */
		      en_pos = saved_it.charpos;
		      type_of_next = r#type;
		    }
		  else
		    { en_pos = (*bidi_it).charpos; }
		  bidi_copy_it (bidi_it, &raw mut saved_it);
		}
	      /* Remember this position, to speed up processing of the
		 next ETs.  */
	      (*bidi_it).next_en_pos = en_pos;
	      if type_of_next == WEAK_EN
		{
		  /* If the last strong character is AL, the EN we've
		     found will become AN when we get to it (W2). */
		  if (*bidi_it).last_strong.r#type == STRONG_AL
		    { type_of_next = WEAK_AN; }
		  else if r#type == WEAK_BN
		    { r#type = NEUTRAL_ON; } /* W6/Retaining */
		  else
		    { r#type = WEAK_EN; }
		}
	      else if type_of_next == NEUTRAL_B
		/* Record the fact that there are no more ENs from
		   here to the end of paragraph, to avoid entering the
		   loop above ever again in this paragraph.  */
		{ (*bidi_it).next_en_pos = -1; }
	      /* Record the type of the character where we ended our search.  */
	      (*bidi_it).next_en_type = type_of_next;
	    }
	}
    }

  if r#type == WEAK_ES || r#type == WEAK_ET || r#type == WEAK_CS /* W6 */
      || (r#type == WEAK_BN
	  && ((*bidi_it).prev.r#type == WEAK_CS	    /* W6/Retaining */
	      || (*bidi_it).prev.r#type == WEAK_ES
	      || (*bidi_it).prev.r#type == WEAK_ET))
    { r#type = NEUTRAL_ON; }

  /* Store the type we've got so far, before we clobber it with strong
     types in W7 and while resolving neutral types.  But leave alone
     the original types that were recorded above, because we will need
     them for the L1 clause.  */
  if (*bidi_it).type_after_wn == UNKNOWN_BT
    { (*bidi_it).type_after_wn = r#type; }
  bidi_check_type ((*bidi_it).type_after_wn);

  if r#type == WEAK_EN	/* W7 */
    {
      if (*bidi_it).last_strong.r#type == STRONG_L
	  || ((*bidi_it).last_strong.r#type == UNKNOWN_BT && (*bidi_it).sos == L2R)
	{ r#type = STRONG_L; }
    }

  (*bidi_it).r#type = r#type;
  bidi_check_type ((*bidi_it).r#type);
  return r#type;

  }
}

/* Resolve the type of a neutral character according to the type of
   surrounding strong text and the current embedding level.  */
fn bidi_resolve_neutral_1 (mut prev_type: bidi_type_t, mut next_type: bidi_type_t,
				  lev: c_int) -> bidi_type_t
{
  /* N1: "European and Arabic numbers act as if they were R in terms
     of their influence on NIs."  */
  if next_type == WEAK_EN || next_type == WEAK_AN
    { next_type = STRONG_R; }
  if prev_type == WEAK_EN || prev_type == WEAK_AN
    { prev_type = STRONG_R; }

  if next_type == prev_type	/* N1 */
    { return next_type; }
  else if (lev & 1) == 0	/* N2 */
    { return STRONG_L; }
  else
    { return STRONG_R; }
}

const FLAG_EMBEDDING_INSIDE: c_uint = 1;
const FLAG_OPPOSITE_INSIDE: c_uint = 2;

/* A data type used in the stack maintained by
   bidi_find_bracket_pairs below.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct bpa_stack_entry
{
  pub close_bracket_char: c_int,
  pub open_bracket_idx: c_int,
  #[cfg (enable_checking)]
  pub open_bracket_pos: ptrdiff_t,      /* C 2540-2542 */
  pub flags: c_uint,                    // flags:2
}

/* clang against the frozen target: sizeof 12, align 4, `close_bracket_char` 0,
   `open_bracket_idx` 4 (the 2-bit `flags` shares the 4-byte unit at 8).
   With `ENABLE_CHECKING` defined, the extra `ptrdiff_t` puts `open_bracket_pos` at 8 and
   the unit at 16: sizeof 24, align 8 (measured with `-DENABLE_CHECKING`); that variant is
   the one the `--cfg enable_checking` pass compiles (P-51, §3.50).  */
#[cfg (not (enable_checking))]
const _: () = core::assert! (std::mem::size_of::<bpa_stack_entry> () == 12);
#[cfg (not (enable_checking))]
const _: () = core::assert! (std::mem::align_of::<bpa_stack_entry> () == 4);
#[cfg (enable_checking)]
const _: () = core::assert! (std::mem::size_of::<bpa_stack_entry> () == 24);
#[cfg (enable_checking)]
const _: () = core::assert! (std::mem::align_of::<bpa_stack_entry> () == 8);
const _: () = core::assert! (std::mem::offset_of! (bpa_stack_entry, close_bracket_char) == 0);
const _: () = core::assert! (std::mem::offset_of! (bpa_stack_entry, open_bracket_idx) == 4);
#[cfg (enable_checking)]
const _: () = core::assert! (std::mem::offset_of! (bpa_stack_entry, open_bracket_pos) == 8);

/* Allow for the two struct bidi_it objects too, since they can be big.
   With MAX_ALLOCA of 16 KiB, this should allow at least 900 slots in the
   BPA stack, which should be more than enough for actual bidi text.  */
/* C 2549-2550: `enum { MAX_BPA_STACK = max (1, ((MAX_ALLOCA - 2 * sizeof (struct bidi_it))
   / sizeof (bpa_stack_entry))) };`.  C's `max` is the ternary `((a) > (b) ? (a) : (b))`, and
   Rust's `max` is not a `const fn`, so the fold is written out in the same operand order
   (the `BUF_BYTES_MAX` precedent in docs/ports/macro-body-notes.tsv); `elsz` is C 585's
   `sizeof (struct bidi_it)`.  */
const MAX_BPA_STACK: c_int
  = if 1 > ((MAX_ALLOCA as usize - 2 * std::mem::size_of::<bidi_it> ())
	    / std::mem::size_of::<bpa_stack_entry> ()) as isize
    { 1 }
    else { ((MAX_ALLOCA as usize - 2 * std::mem::size_of::<bidi_it> ())
	    / std::mem::size_of::<bpa_stack_entry> ()) as c_int };

/* UAX#9 says to match opening brackets with the matching closing
   brackets or their canonical equivalents.  As of Unicode 8.0, there
   are only 2 bracket characters that have canonical equivalence
   decompositions: u+2329 and u+232A.  So instead of accessing the
   table in uni-decomposition.el, we just handle these 2 characters
   with this simple macro.  Note that ASCII characters don't have
   canonical equivalents by definition.  */
macro_rules! CANONICAL_EQU {
  ($c:expr) => {
    if ASCII_CHAR_P (($c) as isize) { $c }
    else if ($c) == LEFT_POINTING_ANGLE_BRACKET { LEFT_ANGLE_BRACKET }
    else if ($c) == RIGHT_POINTING_ANGLE_BRACKET { RIGHT_ANGLE_BRACKET }
    else { $c }
  };
}

/* This function implements BPA, the Bidi Parenthesis Algorithm,
   described in BD16 and N0 of UAX#9.  It finds all the bracket pairs
   in the current isolating sequence, and records the enclosed type
   and the position of the matching bracket in the cache.  It returns
   non-zero if called with the iterator on the opening bracket which
   has a matching closing bracket in the current isolating sequence,
   zero otherwise.  */
unsafe fn bidi_find_bracket_pairs (bidi_it: *mut bidi_it) -> bool
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut btype: bidi_bracket_type_t;
  let mut r#type: bidi_type_t = (*bidi_it).r#type;
  let mut retval: bool = false;
  let mut n: ptrdiff_t = 0;

  /* C's `give_up:` label (reached by two `goto give_up`s inside the loop below),
     carried as a labelled block (PORTING.md 2.2, the intervals.rs `'check_done`
     shape); the epilogue after it runs on every path.  */
  'give_up: {
    /* When scanning backwards, we don't expect any unresolved bidi
       bracket characters.  */
    if (*bidi_it).scan_dir != 1
      { emacs_abort (); }

    btype = bidi_paired_bracket_type ((*bidi_it).ch);
    if btype == BIDI_BRACKET_OPEN
      {
	let mut bpa_stack: [bpa_stack_entry; MAX_BPA_STACK as usize]
	  = [std::mem::zeroed (); MAX_BPA_STACK as usize];
	let mut bpa_sp: c_int = -1;
	/* C's `struct bidi_it saved_it;`, `tem_it` and `pairing_pos` (PORTING.md 1.5).  */
	let mut saved_it: bidi_it = std::mem::zeroed ();
	let base_level: c_int = (*bidi_it).level_stack[0].level as c_int;
	let embedding_level: c_int
	  = (*bidi_it).level_stack[(*bidi_it).stack_idx as usize].level as c_int;
	let mut maxlevel: c_int = embedding_level;
	let embedding_type: bidi_type_t
	  = if (embedding_level & 1) != 0 { STRONG_R } else { STRONG_L };
	let mut tem_it: bidi_it = std::mem::zeroed ();
	let mut l2r_seen: bool = false;
	let mut r2l_seen: bool = false;
	let mut pairing_pos: ptrdiff_t = 0;
	let idx_at_entry: ptrdiff_t = (*bidi_cache_idx.as_mut_ptr ());

	/* `bpa_stack` is the `bpa_stack_entry` array, `bpa_sp` its index and `bidi_it`
	   the parameter the two C statement macros capture, so `PUSH_BPA_STACK` and
	   `STORE_BRACKET_CHARPOS` are defined here, after those names exist (the
	   ccl.rs P-57 placement; C has them at file scope, C 2587-2606).  */
	#[cfg (enable_checking)]
	macro_rules! STORE_BRACKET_CHARPOS {
	  () => { bpa_stack[bpa_sp as usize].open_bracket_pos = (*bidi_it).charpos };
	}
	#[cfg (not (enable_checking))]
	macro_rules! STORE_BRACKET_CHARPOS {
	  () => { };
	}

	macro_rules! PUSH_BPA_STACK {
	  () => {
	    {
	      let mut ch: c_int;
	      if bpa_sp < MAX_BPA_STACK - 1 && (*bidi_cache_last_idx.as_mut_ptr ()) <= INT_MAX as ptrdiff_t
		{
		  bpa_sp += 1;
		  ch = CANONICAL_EQU! ((*bidi_it).ch);
		  bpa_stack[bpa_sp as usize].close_bracket_char = bidi_mirror_char (ch);
		  bpa_stack[bpa_sp as usize].open_bracket_idx = (*bidi_cache_last_idx.as_mut_ptr ()) as c_int;
		  bpa_stack[bpa_sp as usize].flags = 0;
		  STORE_BRACKET_CHARPOS! ();
		}
	    }
	  };
	}

	static_assert! (MAX_BPA_STACK >= 100);
	bidi_copy_it (&raw mut saved_it, bidi_it);
	/* bidi_cache_iterator_state refuses to cache on backward scans,
	   and bidi_cache_fetch_state doesn't bring scan_dir from the
	   cache, so we must initialize this explicitly.  */
	tem_it.scan_dir = 1;

	loop
	  {
	    let old_sidx: c_int;
	    let new_sidx: c_int;
	    let current_level: c_int
	      = (*bidi_it).level_stack[(*bidi_it).stack_idx as usize].level as c_int;

	    if maxlevel < current_level
	      { maxlevel = current_level; }
	    /* Mark every opening bracket character we've traversed by
	       putting its own position into bracket_pairing_pos.  This
	       is examined in bidi_resolve_brackets to distinguish
	       brackets that were already resolved to stay NEUTRAL_ON,
	       and those that were not yet processed by this function
	       (because they were skipped when we skip higher embedding
	       levels below).  */
	    if btype == BIDI_BRACKET_OPEN && (*bidi_it).bracket_pairing_pos == -1
	      { (*bidi_it).bracket_pairing_pos = (*bidi_it).charpos; }
	    if bidi_cache_iterator_state (bidi_it, r#type == NEUTRAL_B, false) == 0
	      {
		/* No more space in cache -- give up and let the opening
		   bracket that started this be processed as a
		   NEUTRAL_ON.  */
		bidi_cache_reset_to ((idx_at_entry - (*bidi_cache_start.as_mut_ptr ())) as c_int);
		bidi_copy_it (bidi_it, &raw mut saved_it);
		break 'give_up;
	      }
	    if btype == BIDI_BRACKET_OPEN
	      { PUSH_BPA_STACK! (); }
	    else if btype == BIDI_BRACKET_CLOSE
	      {
		let mut sp: c_int = bpa_sp;
		let curchar: c_int = CANONICAL_EQU! ((*bidi_it).ch);

		eassert! (sp >= 0);
		while sp >= 0 && bpa_stack[sp as usize].close_bracket_char != curchar
		  { sp -= 1; }
		if sp >= 0
		  {
		    /* Update and cache the corresponding opening bracket.  */
		    bidi_cache_fetch_state (bpa_stack[sp as usize].open_bracket_idx as ptrdiff_t,
					    &raw mut tem_it);
		    #[cfg (enable_checking)]
		    {
		      eassert! (bpa_stack[sp as usize].open_bracket_pos == tem_it.charpos);
		    }
		    /* Determine the enclosed type for this bracket
		       pair's type resolution according to N0.  */
		    if (bpa_stack[sp as usize].flags & FLAG_EMBEDDING_INSIDE) != 0
		      { tem_it.bracket_enclosed_type = embedding_type; } /* N0b */
		    else if (bpa_stack[sp as usize].flags & FLAG_OPPOSITE_INSIDE) != 0
		      { tem_it.bracket_enclosed_type		   /* N0c */
			  = if embedding_type == STRONG_L { STRONG_R } else { STRONG_L }; }
		    else						   /* N0d */
		      { tem_it.bracket_enclosed_type = UNKNOWN_BT; }

		    /* Record the position of the matching closing
		       bracket, and update the cache.  */
		    tem_it.bracket_pairing_pos = (*bidi_it).charpos;
		    bidi_cache_iterator_state (&raw mut tem_it, false, true);

		    /* Pop the BPA stack.  */
		    bpa_sp = sp - 1;
		  }
		if bpa_sp < 0
		  {
		    retval = true;
		    break;
		  }
	      }
	    else if bidi_get_category ((*bidi_it).type_after_wn) != NEUTRAL
	      {
		let mut flag: c_uint = 0;
		let mut sp: c_int;

		/* Whenever we see a strong type, update the flags of
		   all the slots on the stack.  */
		match INT_PROMOTE! ((*bidi_it).r#type) /* avoid warnings */
		  {
		    STRONG_L => {
		      flag = if (embedding_level & 1) == 0
			     { FLAG_EMBEDDING_INSIDE }
			     else { FLAG_OPPOSITE_INSIDE };
		      l2r_seen = true;
		    }
		    STRONG_R | WEAK_EN | WEAK_AN => {
		      flag = if (embedding_level & 1) == 1
			     { FLAG_EMBEDDING_INSIDE }
			     else { FLAG_OPPOSITE_INSIDE };
		      r2l_seen = true;
		    }
		    _ => {
		    }
		  }
		if flag != 0
		  {
		    sp = bpa_sp;
		    while sp >= 0
		      {
			bpa_stack[sp as usize].flags |= flag;
			sp -= 1;
		      }
		  }
	      }
	    old_sidx = (*bidi_it).stack_idx;
	    r#type = bidi_resolve_weak (bidi_it);
	    n += 1;
	    /* Skip level runs excluded from this isolating run sequence.  */
	    new_sidx = (*bidi_it).stack_idx;
	    if (*bidi_it).level_stack[new_sidx as usize].level as c_int > current_level
		&& (ISOLATE_STATUS! (bidi_it, new_sidx) != 0
		    || (new_sidx > old_sidx + 1
			&& ISOLATE_STATUS! (bidi_it, new_sidx - 1) != 0))
	      {
		while (*bidi_it).level_stack[(*bidi_it).stack_idx as usize].level as c_int
			> current_level
		  {
		    if maxlevel < (*bidi_it).level_stack[(*bidi_it).stack_idx as usize].level as c_int
		      { maxlevel = (*bidi_it).level_stack[(*bidi_it).stack_idx as usize].level as c_int; }
		    if bidi_cache_iterator_state (bidi_it,
						  r#type == NEUTRAL_B, false) == 0
		      {
			/* No more space in cache -- give up and let the
			   opening bracket that started this be
			   processed as any other NEUTRAL_ON.  */
			bidi_cache_reset_to ((idx_at_entry - (*bidi_cache_start.as_mut_ptr ())) as c_int);
			bidi_copy_it (bidi_it, &raw mut saved_it);
			break 'give_up;
		      }
		    r#type = bidi_resolve_weak (bidi_it);
		    n += 1;
		  }
	      }
	    if r#type == NEUTRAL_B
		|| ((*bidi_it).level_stack[(*bidi_it).stack_idx as usize].level as c_int
		    != current_level)
	      {
		/* We've marched all the way to the end of this
		   isolating run sequence, and didn't find matching
		   closing brackets for some opening brackets.  Leave
		   their type unchanged.  */
		pairing_pos = (*bidi_it).charpos;
		break;
	      }
	    if (*bidi_it).type_after_wn == NEUTRAL_ON /* Unicode 8.0 correction */
	      { btype = bidi_paired_bracket_type ((*bidi_it).ch); }
	    else
	      { btype = BIDI_BRACKET_NONE; }
	  }

	/* Restore bidi_it from the cache, which should have the bracket
	   resolution members set as determined by the above loop.  */
	r#type = bidi_cache_find (saved_it.charpos, false, bidi_it);
	eassert! (r#type == NEUTRAL_ON);

	/* The following is an optimization for bracketed text that has
	   only one level which is equal to the paragraph's base
	   embedding level.  That is, only L2R and weak/neutral
	   characters in a L2R paragraph, or only R2L and weak/neutral
	   characters in a R2L paragraph.  Such brackets can be resolved
	   by bidi_resolve_neutral, which has a further shortcut for
	   this case.  So we pretend we did not resolve the brackets in
	   this case, set up next_for_neutral for the entire bracketed
	   text, and reset the cache to the character before the opening
	   bracket.  The upshot is to allow bidi_move_to_visually_next
	   reset the cache when it returns this opening bracket, thus
	   cutting significantly on the size of the cache, which is
	   important with long lines, especially if word-wrap is non-nil
	   (which requires the display engine to copy the cache back and
	   forth many times).  */
	if maxlevel == base_level
	    && (l2r_seen || r2l_seen) /* N0d */
	    && ((base_level == 0 && !r2l_seen)
		|| (base_level == 1 && !l2r_seen))
	  {
	    let eob: ptrdiff_t
	      = if !(*bidi_it).string.s.is_null () || STRINGP ((*bidi_it).string.lstring)
		{ (*bidi_it).string.schars } else { ZV! () };

	    if retval
	      { pairing_pos = (*bidi_it).bracket_pairing_pos; }

	    /* This special value (which cannot possibly happen when
	       brackets are resolved, since there's no character at ZV)
	       will be noticed by bidi_resolve_explicit, and will be
	       copied to the following iterator states, instead of being
	       reset to -1.  */
	    (*bidi_it).bracket_pairing_pos = eob;
	    /* This type value will be used for resolving the outermost
	       closing bracket in bidi_resolve_brackets.  */
	    (*bidi_it).bracket_enclosed_type = embedding_type;
	    /* bidi_cache_last_idx is set to the index of the current
	       state, because we just called bidi_cache_find above.
	       That state describes the outermost opening bracket, the
	       one with which we entered this function.  Force the cache
	       to "forget" all the cached states starting from that state.  */
	    bidi_cache_reset_to (((*bidi_cache_last_idx.as_mut_ptr ()) - (*bidi_cache_start.as_mut_ptr ())) as c_int);
	    /* Set up the next_for_neutral member, to help
	       bidi_resolve_neutral.  */
	    (*bidi_it).next_for_neutral.r#type = embedding_type;
	    (*bidi_it).next_for_neutral.charpos = pairing_pos;
	    /* Pretend we didn't resolve this bracket.  */
	    retval = false;
	  }
      }
  }

  /* The factor of 20 below is a heuristic that needs to be tuned.  It
     means we consider 20 buffer positions examined by this function
     roughly equivalent to the display engine iterating over a single
     buffer position.  */
  if (*max_redisplay_ticks.as_mut_ptr ()) > 0 && n > 0
    { update_redisplay_ticks ((n / 20 + 1) as c_int, (*bidi_it).w); }
  return retval;

  }
}

unsafe fn bidi_record_type_for_neutral (info: *mut bidi_saved_info, level: c_int,
					nextp: bool)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut idx: c_int;

  idx = (*bidi_cache_last_idx.as_mut_ptr ()) as c_int + 1;
  while idx < (*bidi_cache_idx.as_mut_ptr ()) as c_int
    {
      let lev: c_int
	= (*(*bidi_cache.as_mut_ptr ()).offset (idx as ptrdiff_t)).level_stack[(*(*bidi_cache.as_mut_ptr ()).offset (idx as ptrdiff_t)).stack_idx as usize].level as c_int;

      if lev <= level
	{
	  eassert! (lev == level);
	  if nextp
	    { (*(*bidi_cache.as_mut_ptr ()).offset (idx as ptrdiff_t)).next_for_neutral = *info; }
	  else
	    { (*(*bidi_cache.as_mut_ptr ()).offset (idx as ptrdiff_t)).prev_for_neutral = *info; }
	  break;
	}
      idx += 1;
    }

  }
}

unsafe fn bidi_resolve_brackets (bidi_it: *mut bidi_it) -> bidi_type_t
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let prev_level: c_int = (*bidi_it).level_stack[(*bidi_it).stack_idx as usize].level
			  as c_int;
  let mut resolve_bracket: bool = false;
  let mut r#type: bidi_type_t = UNKNOWN_BT;
  /* C's `int ch;`, assigned in the `if (!first_elt)` block and read only where that
     block ran (PORTING.md 1.5).  */
  let mut ch: c_int = 0;
  let mut prev_for_neutral: bidi_saved_info = std::mem::zeroed ();
  let mut next_for_neutral: bidi_saved_info = std::mem::zeroed ();
  let eob: ptrdiff_t
    = if !(*bidi_it).string.s.is_null () || STRINGP ((*bidi_it).string.lstring)
      { (*bidi_it).string.schars } else { ZV! () };

  /* Record the prev_for_neutral type either from the previous
     character, if it was a strong or AN/EN, or from the
     prev_for_neutral information recorded previously.  */
  if (*bidi_it).r#type == STRONG_L || (*bidi_it).r#type == STRONG_R
      || (*bidi_it).r#type == WEAK_AN || (*bidi_it).r#type == WEAK_EN
    { bidi_remember_char (&raw mut prev_for_neutral, bidi_it, true); }
  else
    { prev_for_neutral = (*bidi_it).prev_for_neutral; }
  /* Record the next_for_neutral type information.  */
  if (*bidi_it).next_for_neutral.charpos > (*bidi_it).charpos
    { next_for_neutral = (*bidi_it).next_for_neutral; }
  else
    { next_for_neutral.charpos = -1; }
  if !bidi_it_bits! (first_elt, bidi_it)
    {
      r#type = bidi_cache_find ((*bidi_it).charpos + (*bidi_it).nchars, false, bidi_it);
      ch = (*bidi_it).ch;
    }
  if r#type == UNKNOWN_BT
    {
      r#type = bidi_resolve_weak (bidi_it);
      if r#type == NEUTRAL_ON
	{
	  /* bracket_pairing_pos == eob means this bracket does not
	     need to be resolved as a bracket, but as a neutral, see
	     the optimization trick we play near the end of
	     bidi_find_bracket_pairs.  */
	  if (*bidi_it).bracket_pairing_pos == eob
	    {
	      /* If this is the outermost closing bracket of a run of
		 characters in which we decided to resolve brackets as
		 neutrals, use the embedding level's type, recorded in
		 bracket_enclosed_type, to resolve the bracket.  */
	      if (*bidi_it).next_for_neutral.charpos == (*bidi_it).charpos
		  && bidi_paired_bracket_type ((*bidi_it).ch) == BIDI_BRACKET_CLOSE
		{ r#type = (*bidi_it).bracket_enclosed_type; }
	    }
	  else if bidi_find_bracket_pairs (bidi_it)
	    { resolve_bracket = true; }
	}
    }
  else if (*bidi_it).bracket_pairing_pos != eob
    {
      /* If the cached state shows an increase of embedding level due
	 to an isolate initiator, we need to update the 1st cached
	 state of the next run of the current isolating sequence with
	 the prev_for_neutral and next_for_neutral information, so
	 that it will be picked up when we advance to that next run.  */
      if (*bidi_it).level_stack[(*bidi_it).stack_idx as usize].level as c_int > prev_level
	  && ISOLATE_STATUS! (bidi_it, (*bidi_it).stack_idx) != 0
	{
	  eassert! ((*bidi_it).resolved_level == -1);
	  bidi_record_type_for_neutral (&raw mut prev_for_neutral, prev_level, false);
	  bidi_record_type_for_neutral (&raw mut next_for_neutral, prev_level, true);
	}
      if r#type == NEUTRAL_ON
	  && bidi_paired_bracket_type (ch) == BIDI_BRACKET_OPEN
	{
	  if (*bidi_it).bracket_pairing_pos > (*bidi_it).charpos
	    {
	      /* A cached opening bracket that wasn't completely
		 resolved yet.  */
	      resolve_bracket = true;
	    }
	  else if (*bidi_it).bracket_pairing_pos == -1
	    {
	      eassert! ((*bidi_it).resolved_level == -1);
	      /* Higher levels were not BPA-resolved yet, even if
		 cached by bidi_find_bracket_pairs.  Force application
		 of BPA to the new level now.  */
	      if bidi_find_bracket_pairs (bidi_it)
		{ resolve_bracket = true; }
	    }
	}
      /* Keep track of the prev_for_neutral and next_for_neutral
	 types, needed for resolving brackets below and for resolving
	 neutrals in bidi_resolve_neutral.  */
      if (*bidi_it).level_stack[(*bidi_it).stack_idx as usize].level as c_int
	   == prev_level
	{
	  (*bidi_it).prev_for_neutral = prev_for_neutral;
	  if next_for_neutral.charpos > 0
	    { (*bidi_it).next_for_neutral = next_for_neutral; }
	}
    }

  /* If needed, resolve the bracket type according to N0.  */
  if resolve_bracket
    {
      let embedding_level: c_int
	= (*bidi_it).level_stack[(*bidi_it).stack_idx as usize].level as c_int;
      let embedding_type: bidi_type_t
	= if (embedding_level & 1) != 0 { STRONG_R } else { STRONG_L };

      eassert! ((*bidi_it).bracket_pairing_pos > (*bidi_it).charpos);
      if (*bidi_it).bracket_enclosed_type == embedding_type /* N0b */
	{ r#type = embedding_type; }
      else if (*bidi_it).bracket_enclosed_type == STRONG_L   /* N0c, N0d */
	       || (*bidi_it).bracket_enclosed_type == STRONG_R
	{
	  let mut prev_type_for_neutral: bidi_type_t
	    = (*bidi_it).prev_for_neutral.r#type;

	  if prev_type_for_neutral == UNKNOWN_BT
	    { prev_type_for_neutral = embedding_type; }
	  match INT_PROMOTE! (prev_type_for_neutral) /* avoid warnings */
	    {
	      STRONG_R | WEAK_EN | WEAK_AN => {
		r#type =
		  if (*bidi_it).bracket_enclosed_type == STRONG_R /* N0c */
		  { STRONG_R }				     /* N0c1 */
		  else { embedding_type };			     /* N0c2 */
	      }
	      STRONG_L => {
		r#type =
		  if (*bidi_it).bracket_enclosed_type == STRONG_L /* N0c */
		  { STRONG_L }				     /* N0c1 */
		  else { embedding_type };			     /* N0c2 */
	      }
	      _ => {
		/* N0d: Do not set the type for that bracket pair.  */
	      }
	    }
	}
      eassert! (r#type == STRONG_L || r#type == STRONG_R || r#type == NEUTRAL_ON);

      /* Update the type of the paired closing bracket to the same
	 type as for the resolved opening bracket.  */
      if r#type != NEUTRAL_ON
	{
	  let idx: ptrdiff_t = bidi_cache_search ((*bidi_it).bracket_pairing_pos,
						  -1, 1);

	  if idx < (*bidi_cache_start.as_mut_ptr ())
	    { emacs_abort (); }
	  (*(*bidi_cache.as_mut_ptr ()).offset (idx)).r#type = r#type;
	}
    }

  return r#type;

  }
}

unsafe fn bidi_resolve_neutral (bidi_it: *mut bidi_it) -> bidi_type_t
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut r#type: bidi_type_t = bidi_resolve_brackets (bidi_it);
  let mut current_level: c_int;
  let mut is_neutral: bool;

  eassert! (r#type == STRONG_R
	    || r#type == STRONG_L
	    || r#type == WEAK_BN
	    || r#type == WEAK_EN
	    || r#type == WEAK_AN
	    || r#type == NEUTRAL_B
	    || r#type == NEUTRAL_S
	    || r#type == NEUTRAL_WS
	    || r#type == NEUTRAL_ON
	    || r#type == LRI
	    || r#type == RLI
	    || r#type == PDI);

  current_level = (*bidi_it).level_stack[(*bidi_it).stack_idx as usize].level as c_int;
  eassert! (current_level >= 0);
  is_neutral = bidi_get_category (r#type) == NEUTRAL;

  if (r#type != NEUTRAL_B /* Don't risk entering the long loop below if
			      we are already at paragraph end.  */
      && (is_neutral || bidi_isolate_fmt_char (r#type)))
      /* N1-N2/Retaining */
      || r#type == WEAK_BN
    {
      if (*bidi_it).next_for_neutral.r#type != UNKNOWN_BT
	  && ((*bidi_it).next_for_neutral.charpos > (*bidi_it).charpos
	      /* PDI defines an eos, so it's OK for it to serve as its
		 own next_for_neutral.  */
	      || ((*bidi_it).next_for_neutral.charpos == (*bidi_it).charpos
		  && (*bidi_it).r#type == PDI))
	{
	  r#type = bidi_resolve_neutral_1 ((*bidi_it).prev_for_neutral.r#type,
					   (*bidi_it).next_for_neutral.r#type,
					   current_level);
	}
      /* The next two "else if" clauses are shortcuts for the
	 important special case when we have a long sequence of
	 neutral or WEAK_BN characters, such as whitespace or nulls or
	 other control characters, on the base embedding level of the
	 paragraph, and that sequence goes all the way to the end of
	 the paragraph and follows a character whose resolved
	 directionality is identical to the base embedding level.
	 (This is what happens in a buffer with plain L2R text that
	 happens to include long sequences of control characters.)  By
	 virtue of N1, the result of examining this long sequence will
	 always be either STRONG_L or STRONG_R, depending on the base
	 embedding level.  So we use this fact directly instead of
	 entering the expensive loop in the "else" clause.  */
      else if current_level == 0
	       && (*bidi_it).prev_for_neutral.r#type == STRONG_L
	       && (ASCII_CHAR_P ((*bidi_it).ch as isize)
		   || (r#type != WEAK_BN
		       && !bidi_explicit_dir_char ((*bidi_it).ch)
		       && !bidi_isolate_fmt_char (r#type)))
	{ r#type = bidi_resolve_neutral_1 ((*bidi_it).prev_for_neutral.r#type,
					   STRONG_L, current_level); }
      else if /* current level is 1 */
	       current_level == 1
	       /* base embedding level is also 1 */
	       && (*bidi_it).level_stack[0].level as c_int == 1
	       /* previous character is one of those considered R for
		  the purposes of W5 */
	       && ((*bidi_it).prev_for_neutral.r#type == STRONG_R
		   || (*bidi_it).prev_for_neutral.r#type == WEAK_EN
		   || (*bidi_it).prev_for_neutral.r#type == WEAK_AN)
	       && r#type != WEAK_BN
	       && !bidi_explicit_dir_char ((*bidi_it).ch)
	       && !bidi_isolate_fmt_char (r#type)
	{ r#type = bidi_resolve_neutral_1 ((*bidi_it).prev_for_neutral.r#type,
					   STRONG_R, current_level); }
      else
	{
	  /* Arrrgh!!  The UAX#9 algorithm is too deeply entrenched in
	     the assumption of batch-style processing; see clauses W4,
	     W5, and especially N1, which require looking far forward
	     (as well as back) in the buffer/string.  May the fleas of
	     a thousand camels infest the armpits of those who design
	     supposedly general-purpose algorithms by looking at their
	     own implementations, and fail to consider other possible
	     implementations!  */
	  let mut saved_it: bidi_it = std::mem::zeroed ();
	  let mut next_type: bidi_type_t = UNKNOWN_BT;
	  let mut adjacent_to_neutrals: bool = is_neutral;

	  bidi_copy_it (&raw mut saved_it, bidi_it);
	  /* Scan the text forward until we find the first non-neutral
	     character, and then use that to resolve the neutral we
	     are dealing with now.  We also cache the scanned iterator
	     states, to salvage some of the effort later.  */
	  loop {
	    let old_sidx: c_int;
	    let new_sidx: c_int;

	    /* Paragraph separators have their levels fully resolved
	       at this point, so cache them as resolved.  */
	    bidi_cache_iterator_state (bidi_it, r#type == NEUTRAL_B, false);
	    old_sidx = (*bidi_it).stack_idx;
	    r#type = bidi_resolve_brackets (bidi_it);
	    /* Skip level runs excluded from this isolating run sequence.  */
	    new_sidx = (*bidi_it).stack_idx;
	    if (*bidi_it).level_stack[new_sidx as usize].level as c_int > current_level
		&& (ISOLATE_STATUS! (bidi_it, new_sidx) != 0
		    /* This is for when we have an isolate initiator
		       immediately followed by an embedding or
		       override initiator, in which case we get the
		       level stack pushed twice by the single call to
		       bidi_resolve_weak above.  */
		    || (new_sidx > old_sidx + 1
			&& ISOLATE_STATUS! (bidi_it, new_sidx - 1) != 0))
	      {
		while (*bidi_it).level_stack[(*bidi_it).stack_idx as usize].level as c_int
			> current_level
		  {
		    bidi_cache_iterator_state (bidi_it, r#type == NEUTRAL_B, false);
		    r#type = bidi_resolve_brackets (bidi_it);
		  }
	      }
	    if !adjacent_to_neutrals
		&& (bidi_get_category (r#type) == NEUTRAL
		    || bidi_isolate_fmt_char (r#type))
	      { adjacent_to_neutrals = true; }
	    if r#type == NEUTRAL_B
		|| (r#type != WEAK_BN
		    && bidi_get_category (r#type) != NEUTRAL
		    && !bidi_isolate_fmt_char (r#type))
		/* This is all per level run, so stop when we
		   reach the end of this level run.  */
		|| ((*bidi_it).level_stack[(*bidi_it).stack_idx as usize].level as c_int
		    != current_level)
	      { break; }
	  }

	  /* Record the character we stopped at.  */
	  bidi_remember_char (&raw mut saved_it.next_for_neutral, bidi_it, true);

	  if ((*bidi_it).level_stack[(*bidi_it).stack_idx as usize].level as c_int
		!= current_level)
	      || r#type == NEUTRAL_B
	    {
	      /* Marched all the way to the end of this level run.  We
		 need to use the eos type, whose information is stored
		 by bidi_set_sos_type in the prev_for_neutral
		 member.  */
	      if adjacent_to_neutrals
		{ next_type = (*bidi_it).prev_for_neutral.r#type; }
	      else
		{
		  /* This is a BN which does not adjoin neutrals.
		     Leave its type alone.  */
		  bidi_copy_it (bidi_it, &raw mut saved_it);
		  return (*bidi_it).r#type;
		}
	    }
	  else
	    {
	      match INT_PROMOTE! (r#type) /* promotion to int avoids warnings */
		{
		  STRONG_L | STRONG_R | STRONG_AL => {
		    /* Actually, STRONG_AL cannot happen here, because
		       bidi_resolve_weak converts it to STRONG_R, per W3.  */
		    eassert! (r#type != STRONG_AL);
		    next_type = r#type;
		  }
		  WEAK_EN | WEAK_AN => {
		    /* N1: "European and Arabic numbers act as if they
		       were R in terms of their influence on NIs."  */
		    next_type = STRONG_R;
		  }
		  _ => {
		    emacs_abort ();
		  }
		}
	    }
	  /* Resolve the type of all the NIs found during the above loop.  */
	  r#type = bidi_resolve_neutral_1 (saved_it.prev_for_neutral.r#type,
					   next_type, current_level);
	  /* Update next_for_neutral with the resolved type, so we
	     could use it for all the other NIs up to the place where
	     we exited the loop.  */
	  saved_it.next_for_neutral.r#type = next_type;
	  bidi_check_type (r#type);
	  /* Update the character which caused us to enter the above loop.  */
	  saved_it.r#type = r#type;
	  bidi_check_type (next_type);
	  bidi_copy_it (bidi_it, &raw mut saved_it);
	}
    }
  return r#type;

  }
}

/* Given an iterator state in BIDI_IT, advance one character position
   in the buffer/string to the next character (in the logical order),
   resolve the bidi type of that next character, and return that
   type.  */
unsafe fn bidi_type_of_next_char (bidi_it: *mut bidi_it) -> bidi_type_t
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let r#type: bidi_type_t;

  /* This should always be called during a forward scan.  */
  if (*bidi_it).scan_dir != 1
    { emacs_abort (); }

  r#type = bidi_resolve_neutral (bidi_it);

  return r#type;

  }
}

/* Given an iterator state BIDI_IT, advance one character position in
   the buffer/string to the next character (in the current scan
   direction), resolve the embedding and implicit levels of that next
   character, and return the resulting level.  */
unsafe fn bidi_level_of_next_char (bidi_it: *mut bidi_it) -> c_int
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut r#type: bidi_type_t = UNKNOWN_BT;
  let mut level: c_int;
  let mut next_char_pos: ptrdiff_t = -2;

  if (*bidi_it).scan_dir == 1
    {
      let eob: ptrdiff_t
	= if !(*bidi_it).string.s.is_null () || STRINGP ((*bidi_it).string.lstring)
	  { (*bidi_it).string.schars } else { ZV! () };

      /* There's no sense in trying to advance if we've already hit
	 the end of text.  */
      if (*bidi_it).charpos >= eob
	{
	  eassert! ((*bidi_it).resolved_level >= 0);
	  return (*bidi_it).resolved_level as c_int;
	}
    }

  /* Perhaps the character we want is already cached as fully resolved.
     If it is, the call to bidi_cache_find below will return a type
     other than UNKNOWN_BT.  */
  if (*bidi_cache_idx.as_mut_ptr ()) > (*bidi_cache_start.as_mut_ptr ()) && !bidi_it_bits! (first_elt, bidi_it)
    {
      let bob: c_int
	= if !(*bidi_it).string.s.is_null () || STRINGP ((*bidi_it).string.lstring)
	  { 0 } else { 1 };

      if (*bidi_it).scan_dir > 0
	{
	  if (*bidi_it).nchars <= 0
	    { emacs_abort (); }
	  next_char_pos = (*bidi_it).charpos + (*bidi_it).nchars;
	}
      else if (*bidi_it).charpos >= bob as ptrdiff_t
	/* Implementation note: we allow next_char_pos to be as low as
	   0 for buffers or -1 for strings, and that is okay because
	   that's the "position" of the sentinel iterator state we
	   cached at the beginning of the iteration.  */
	{ next_char_pos = (*bidi_it).charpos - 1; }
      if next_char_pos >= bob as ptrdiff_t - 1
	{ r#type = bidi_cache_find (next_char_pos, true, bidi_it); }
      if r#type != UNKNOWN_BT
	{
	  /* We asked the cache for fully resolved states.  */
	  eassert! ((*bidi_it).resolved_level >= 0);
	  return (*bidi_it).resolved_level as c_int;
	}
    }

  if (*bidi_it).scan_dir == -1
    /* If we are going backwards, the iterator state is already cached
       from previous scans, and should be fully resolved.  */
    { emacs_abort (); }

  if r#type == UNKNOWN_BT
    { r#type = bidi_type_of_next_char (bidi_it); }

  if r#type == NEUTRAL_B
    {
      eassert! ((*bidi_it).resolved_level >= 0);
      return (*bidi_it).resolved_level as c_int;
    }

  level = (*bidi_it).level_stack[(*bidi_it).stack_idx as usize].level as c_int;

  eassert! (r#type == STRONG_R
	    || r#type == STRONG_L
	    || r#type == WEAK_BN
	    || r#type == WEAK_EN
	    || r#type == WEAK_AN);
  (*bidi_it).r#type = r#type;
  bidi_check_type ((*bidi_it).r#type);

  /* For L1 below, we need to know, for each WS character, whether
     it belongs to a sequence of WS characters preceding a newline
     or a TAB or a paragraph separator.  */
  if ((*bidi_it).orig_type == NEUTRAL_WS
      || ((*bidi_it).orig_type == WEAK_BN
	  /* If this BN character is already at base level, we don't
	     need to consider resetting it, since I1 and I2 below
	     will not change the level, so avoid the potentially
	     costly loop below.  */
	  && level != (*bidi_it).level_stack[0].level as c_int)
      || bidi_isolate_fmt_char ((*bidi_it).orig_type))
      /* This means the informaition about WS resolution is not valid.  */
      && (*bidi_it).next_for_ws.charpos < (*bidi_it).charpos
    {
      let mut ch: c_int;
      let mut clen: ptrdiff_t = (*bidi_it).ch_len;
      let mut bpos: ptrdiff_t = (*bidi_it).bytepos;
      let mut cpos: ptrdiff_t = (*bidi_it).charpos;
      let mut disp_pos: ptrdiff_t = (*bidi_it).disp_pos;
      let mut nc: ptrdiff_t = (*bidi_it).nchars;
      let mut bs: bidi_string_data = (*bidi_it).string;
      let mut chtype: bidi_type_t;
      let fwp: bool = bidi_it_bits! (frame_window_p, bidi_it);
      let mut dpp: c_int = (*bidi_it).disp_prop;

      if (*bidi_it).nchars <= 0
	{ emacs_abort (); }
      loop {
	cpos += nc;
	bpos += clen;
	ch = bidi_fetch_char (cpos, bpos, &raw mut disp_pos, &raw mut dpp, &raw mut bs,
			      (*bidi_it).w, fwp, &raw mut clen, &raw mut nc);
	chtype = bidi_get_type (ch, NEUTRAL_DIR);
	if !(chtype == NEUTRAL_WS || chtype == WEAK_BN
	     || bidi_isolate_fmt_char (chtype)
	     || bidi_explicit_dir_char (ch)) /* L1/Retaining */
	  { break; }
      }
      (*bidi_it).next_for_ws.r#type = chtype;
      bidi_check_type ((*bidi_it).next_for_ws.r#type);
      (*bidi_it).next_for_ws.charpos = cpos;
    }

  /* Update the cache, but only if this state was already cached.  */
  bidi_cache_iterator_state (bidi_it, true, true);

  /* Resolve implicit levels.  */
  if (*bidi_it).orig_type == NEUTRAL_B /* L1 */
      || (*bidi_it).orig_type == NEUTRAL_S
      || (*bidi_it).ch == '\n' as c_int || (*bidi_it).ch == BIDI_EOB
      || (((*bidi_it).orig_type == NEUTRAL_WS
	   || (*bidi_it).orig_type == WEAK_BN /* L1/Retaining */
	   || bidi_isolate_fmt_char ((*bidi_it).orig_type)
	   || bidi_explicit_dir_char ((*bidi_it).ch))
	  && ((*bidi_it).next_for_ws.r#type == NEUTRAL_B
	      || (*bidi_it).next_for_ws.r#type == NEUTRAL_S))
    { level = (*bidi_it).level_stack[0].level as c_int; }
  else if (level & 1) == 0 /* I1 */
    {
      if r#type == STRONG_R
	{ level += 1; }
      else if r#type == WEAK_EN || r#type == WEAK_AN
	{ level += 2; }
    }
  else			/* I2 */
    {
      if r#type == STRONG_L || r#type == WEAK_EN || r#type == WEAK_AN
	{ level += 1; }
    }

  (*bidi_it).resolved_level = level as i8;
  return level;

  }
}

/* Move to the other edge of a level given by LEVEL.  ...  */
unsafe fn bidi_find_other_level_edge (bidi_it: *mut bidi_it, level: c_int,
				      end_flag: bool)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let dir: c_int = if end_flag { -(*bidi_it).scan_dir } else { (*bidi_it).scan_dir };
  let idx: ptrdiff_t;

  /* Try the cache first.  */
  idx = bidi_cache_find_level_change (level, dir, end_flag);
  if idx >= (*bidi_cache_start.as_mut_ptr ())
    { bidi_cache_fetch_state (idx, bidi_it); }
  else
    {
      let mut new_level: c_int;
      let pos0: ptrdiff_t = (*bidi_it).charpos;

      /* If we are at end of level, its edges must be cached.  */
      if end_flag
	{ emacs_abort (); }

      if bidi_cache_iterator_state (bidi_it, true, false) == 0
	{
	  /* Can't happen: if the cache needs to grow, it means we
	     were at base embedding level, so the cache should have
	     been either empty or already large enough to cover this
	     character position.  */
	  emacs_abort ();
	}
      loop {
	new_level = bidi_level_of_next_char (bidi_it);
	/* If the cache is full, perform an emergency return by
	   pretending that the level ended.  */
	if bidi_cache_iterator_state (bidi_it, true, false) == 0
	  {
	    new_level = level - 1;
	    /* Since the cache should only grow when we are scanning
	       forward looking for the edge of the level that is one
	       above the base embedding level, we can only have this
	       contingency when LEVEL - 1 is the base embedding
	       level.  */
	    eassert! (new_level == (*bidi_it).level_stack[0].level as c_int);
	    /* Plan B, for when the cache overflows: Back up to the
	       previous character by fetching the last cached state,
	       and force the resolved level of that character be the
	       base embedding level.  */
	    bidi_cache_fetch_state ((*bidi_cache_idx.as_mut_ptr ()) - 1, bidi_it);
	    (*bidi_it).resolved_level = new_level as i8;
	    bidi_cache_iterator_state (bidi_it, true, true);
	  }
	if !(new_level >= level)
	  { break; }
      }
      /* The factor of 50 below is a heuristic that needs to be
	 tuned.  It means we consider 50 buffer positions examined by
	 the above call roughly equivalent to the display engine
	 iterating over a single buffer position.  */
      if (*max_redisplay_ticks.as_mut_ptr ()) > 0 && (*bidi_it).charpos > pos0
	{ update_redisplay_ticks ((((*bidi_it).charpos - pos0) / 50 + 1) as c_int,
				 (*bidi_it).w); }
    }

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn bidi_move_to_visually_next (bidi_it: *mut bidi_it)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let old_level: c_int;
  let mut new_level: c_int;
  let mut next_level: c_int;
  let mut sentinel: bidi_it = std::mem::zeroed ();

  if (*bidi_it).charpos < 0 || (*bidi_it).bytepos < 0
    { emacs_abort (); }

  if (*bidi_it).scan_dir == 0
    {
      (*bidi_it).scan_dir = 1;	/* default to logical order */
    }

  /* If we just passed a newline, initialize for the next line.  */
  if !bidi_it_bits! (first_elt, bidi_it)
      && ((*bidi_it).ch == '\n' as c_int || (*bidi_it).ch == BIDI_EOB)
    { bidi_line_init (bidi_it); }

  /* Prepare the sentinel iterator state, and cache it.  When we bump
     into it, scanning backwards, we'll know that the last non-base
     level is exhausted.  */
  if (*bidi_cache_idx.as_mut_ptr ()) == (*bidi_cache_start.as_mut_ptr ())
    {
      bidi_copy_it (&raw mut sentinel, bidi_it);
      if bidi_it_bits! (first_elt, bidi_it)
	{
	  sentinel.charpos -= 1;	/* cached charpos needs to be monotonic */
	  sentinel.bytepos -= 1;
	  sentinel.ch = '\n' as c_int;	/* doesn't matter, but why not? */
	  sentinel.ch_len = 1;
	  sentinel.nchars = 1;
	}
      bidi_cache_iterator_state (&raw mut sentinel, true, false);
    }

  old_level = (*bidi_it).resolved_level as c_int;
  new_level = bidi_level_of_next_char (bidi_it);

  /* Reordering of resolved levels (clause L2) is implemented by
     jumping to the other edge of the level and flipping direction of
     scanning the text whenever we find a level change.  */
  if new_level != old_level
    {
      let ascending: bool = new_level > old_level;
      let mut level_to_search: c_int = if ascending { old_level + 1 } else { old_level };
      let incr: c_int = if ascending { 1 } else { -1 };
      let mut expected_next_level: c_int = old_level + incr;

      /* Jump (or walk) to the other edge of this level.  */
      bidi_find_other_level_edge (bidi_it, level_to_search, !ascending);
      /* Switch scan direction and peek at the next character in the
	 new direction.  */
      (*bidi_it).scan_dir = -(*bidi_it).scan_dir;

      /* The following loop handles the case where the resolved level
	 jumps by more than one.  ...  */
      next_level = bidi_peek_at_next_level (bidi_it);
      while next_level != expected_next_level
	{
	  /* If next_level is -1, it means we have an unresolved level
	     in the cache, which at this point should not happen.  If
	     it does, we will infloop.  */
	  eassert! (next_level >= 0);
	  /* If next_level is not consistent with incr, we might
	     infloop.  */
	  eassert! (if incr > 0 { next_level > expected_next_level }
		    else { next_level < expected_next_level });
	  expected_next_level += incr;
	  level_to_search += incr;
	  bidi_find_other_level_edge (bidi_it, level_to_search, !ascending);
	  (*bidi_it).scan_dir = -(*bidi_it).scan_dir;
	  next_level = bidi_peek_at_next_level (bidi_it);
	}

      /* Finally, deliver the next character in the new direction.  */
      next_level = bidi_level_of_next_char (bidi_it);
    }

  /* Take note when we have just processed the newline that precedes
     the end of the paragraph.  ...  */
  if (*bidi_it).scan_dir == 1
      && ((*bidi_it).ch == '\n' as c_int || (*bidi_it).ch == BIDI_EOB)
    {
      /* The paragraph direction of the entire string, once
	 determined, is in effect for the entire string.  Setting the
	 separator limit to the end of the string prevents
	 bidi_paragraph_init from being called automatically on this
	 string.  */
      if !(*bidi_it).string.s.is_null () || STRINGP ((*bidi_it).string.lstring)
	{ (*bidi_it).separator_limit = (*bidi_it).string.schars; }
      else if (*bidi_it).bytepos < ZV_BYTE! ()
	{
	  let sep_len: ptrdiff_t
	    = bidi_at_paragraph_end ((*bidi_it).charpos + (*bidi_it).nchars,
				     (*bidi_it).bytepos + (*bidi_it).ch_len);
	  if (*bidi_it).nchars <= 0
	    { emacs_abort (); }
	  if sep_len >= 0
	    {
	      bidi_it_bits! (new_paragraph, bidi_it, 1);
	      /* Record the buffer position of the last character of
		 the paragraph separator.  If the paragraph separator
		 is an empty string (e.g., the regex is "^"), the
		 newline that precedes the end of the paragraph is
		 that last character.  */
	      if sep_len > 0
		{ (*bidi_it).separator_limit
		    = (*bidi_it).charpos + (*bidi_it).nchars + sep_len; }
	      else
		{ (*bidi_it).separator_limit = (*bidi_it).charpos; }
	    }
	}
    }

  if (*bidi_it).scan_dir == 1 && (*bidi_cache_idx.as_mut_ptr ()) > (*bidi_cache_start.as_mut_ptr ())
    {
      /* If we are at paragraph's base embedding level and beyond the
	 last cached position, the cache's job is done and we can
	 discard it.  */
      if (*bidi_it).resolved_level == (*bidi_it).level_stack[0].level as i8
	  && (*bidi_it).charpos
	       > (*(*bidi_cache.as_mut_ptr ()).offset ((*bidi_cache_idx.as_mut_ptr ()) - 1)).charpos
		 + (*(*bidi_cache.as_mut_ptr ()).offset ((*bidi_cache_idx.as_mut_ptr ()) - 1)).nchars - 1
	{ bidi_cache_reset (); }
      /* Also reset the cache if it overflowed and we have just
	 emergency-exited using Plan B.  */
      else if (*bidi_it).resolved_level == (*bidi_it).level_stack[0].level as i8
	       && (*bidi_cache_idx.as_mut_ptr ()) >= (*bidi_cache_size.as_mut_ptr ())
	       && (*bidi_it).charpos == (*(*bidi_cache.as_mut_ptr ()).offset ((*bidi_cache_idx.as_mut_ptr ()) - 1)).charpos
	{ bidi_cache_reset (); }
	/* But as long as we are caching during forward scan, we must
	   cache each state, or else the cache integrity will be
	   compromised: it assumes cached states correspond to buffer
	   positions 1:1.  */
      else
	{ bidi_cache_iterator_state (bidi_it, true, false); }
    }

  eassert! ((*bidi_it).resolved_level >= 0
	    && (*bidi_it).resolved_level as c_int <= BIDI_MAXDEPTH + 2);

  }
}

/* Utility function for looking for strong directional characters
   whose bidi type was overridden by directional override or embedding
   or isolate control characters.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn bidi_find_first_overridden (bidi_it: *mut bidi_it) -> ptrdiff_t
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let eob: ptrdiff_t
    = if STRINGP ((*bidi_it).string.lstring) { (*bidi_it).string.schars } else { ZV! () };
  let mut found_pos: ptrdiff_t = eob;
  /* Maximum bidi levels we allow for L2R and R2L characters.  Note
     that these are levels after resolving explicit embeddings,
     overrides, and isolates, i.e. before resolving implicit levels.  */
  let max_l2r: c_int = if (*bidi_it).paragraph_dir == L2R { 0 } else { 2 };
  let max_r2l: c_int = 1;
  /* Same for WEAK and NEUTRAL_ON types.  */
  let max_weak: c_int = if (*bidi_it).paragraph_dir == L2R { 1 } else { 2 };

  loop
    {
      /* Need to call bidi_resolve_weak, not bidi_resolve_explicit,
	 because the directional overrides are applied by the
	 former.  */
      let r#type: bidi_type_t = bidi_resolve_weak (bidi_it);
      let level: c_uint
	= (*bidi_it).level_stack[(*bidi_it).stack_idx as usize].level as c_uint;
      let category: bidi_category_t = bidi_get_category ((*bidi_it).orig_type);

      /* Detect strong L or R types that have been overridden by
	 explicit overrides.  */
      if (r#type == STRONG_R && (*bidi_it).orig_type == STRONG_L)
	  || (r#type == STRONG_L
	      && ((*bidi_it).orig_type == STRONG_R
		  || (*bidi_it).orig_type == STRONG_AL))
	  /* Detect strong L or R types or WEAK_EN types that were
	     pushed into higher embedding levels (and will thus
	     reorder) by explicit embeddings and isolates.  */
	  || (((*bidi_it).orig_type == STRONG_L
	       || (*bidi_it).orig_type == WEAK_EN)
	      && level > max_l2r as c_uint)
	  || (((*bidi_it).orig_type == STRONG_R
	       || (*bidi_it).orig_type == STRONG_AL)
	      && level > max_r2l as c_uint)
	  /* Detect other weak or neutral types whose level was
	     tweaked by explicit embeddings and isolates.  */
	  || ((category == WEAK || (*bidi_it).orig_type == NEUTRAL_ON)
	      && level > max_weak as c_uint)
	{ found_pos = (*bidi_it).charpos; }
      if !(found_pos == eob
	   && (*bidi_it).charpos < eob
	   && (*bidi_it).ch != BIDI_EOB
	   && (*bidi_it).ch != '\n' as c_int)
	{ break; }
    }

  return found_pos;

  }
}

/* This is meant to be called from within the debugger, whenever you
   wish to examine the cache contents.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn bidi_dump_cached_states ()
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut i: ptrdiff_t;
  let mut ndigits: c_int = 1;

  if (*bidi_cache_idx.as_mut_ptr ()) == 0
    {
      fputs (c"The cache is empty.\n".as_ptr (), (*stderr.as_mut_ptr ()));
      return;
    }
  fprintf ((*stderr.as_mut_ptr ()), c"Total of  %td state%s in cache:\n".as_ptr (),
	   (*bidi_cache_idx.as_mut_ptr ()), if (*bidi_cache_idx.as_mut_ptr ()) == 1 { c"".as_ptr () } else { c"s".as_ptr () });

  i = (*(*bidi_cache.as_mut_ptr ()).offset ((*bidi_cache_idx.as_mut_ptr ()) - 1)).charpos;
  while i > 0
    {
      ndigits += 1;
      i /= 10;
    }
  fputs (c"ch  ".as_ptr (), (*stderr.as_mut_ptr ()));
  i = 0;
  while i < (*bidi_cache_idx.as_mut_ptr ())
    {
      fprintf ((*stderr.as_mut_ptr ()), c"%*c".as_ptr (), ndigits, (*(*bidi_cache.as_mut_ptr ()).offset (i)).ch);
      i += 1;
    }
  fputs (c"\nlvl ".as_ptr (), (*stderr.as_mut_ptr ()));
  i = 0;
  while i < (*bidi_cache_idx.as_mut_ptr ())
    {
      fprintf ((*stderr.as_mut_ptr ()), c"%*d".as_ptr (), ndigits,
	       (*(*bidi_cache.as_mut_ptr ()).offset (i)).resolved_level as c_int);
      i += 1;
    }
  fputs (c"\npos ".as_ptr (), (*stderr.as_mut_ptr ()));
  i = 0;
  while i < (*bidi_cache_idx.as_mut_ptr ())
    {
      fprintf ((*stderr.as_mut_ptr ()), c"%*td".as_ptr (), ndigits,
	       (*(*bidi_cache.as_mut_ptr ()).offset (i)).charpos);
      i += 1;
    }
  putc ('\n' as c_int, (*stderr.as_mut_ptr ()));

  }
}
