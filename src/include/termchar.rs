// femacs: port of src/termchar.h @ fd7afced403.
//
// WIP mirror (PORTING.md P-66): the declarations of the frozen configuration's live text.
// `termchar.h` includes `dispextern.h` (C 23) and is written for a translation unit that has
// already seen `frame.h`/`termhooks.h` (its `FRAME_TTY` reads `(f)->output_method` and
// `(f)->terminal->display_info.tty`), so the port's `include!` list names all three mirrors:
// item order is irrelevant for Rust items, but the *macros* below are expanded at their use
// site, exactly as C expands them.
//
// Degradations this file carries (PORTING.md P-72 rule ③): `struct emacs_tty` (sysdep.h) has
// no mirror yet, and its field is used only as a pointer, so it keeps the untyped pointer
// with a `/// C: …` line and the use sites cast.  `display_info.tty` is termhooks.rs's
// `*mut c_void` for the same reason (termhooks.h 533-542), so `FRAME_TTY` casts it back.
//
// `struct cm` *is* mirrored now (`include/cm.rs`), and this file is where it gets its home:
// termchar.h names `struct cm *Wcm` (C 86), so P-70's stand-in is replaced by the real type
// and the mirror that defines it is included here, in C's declaration order (termchar.h's own
// `#include` list does not name cm.h; C's units do, and cm.h in turn needs nothing).
//
// `bool_bf` is `bool` here (conf_post.h 59-61 takes that arm), one byte wide, so each run of
// consecutive `bool_bf : 1` members is one byte in the layout; the port keeps a `bits_*`
// byte per run with the `TTY_*` masks, the shape `itree.h`'s `itree_node.bits` already uses.
// `tools/gen-layout-fields.py` keys the non-bitfield members, so the byte's position is
// asserted against the C compiler (see docs/ports/include-termchar.md).

// termchar.h 20: the include guard has no Rust counterpart (PORTING.md 0.1).
// [U1-MODULE-MIRROR]
use super::*;
pub const EMACS_TERMCHAR_H: bool = true;

// [U1] ../../lib/include/stdio.rs imported via super
// [U1] dispextern.rs imported via super
// [U1] termhooks.rs imported via super
// [U1] frame.rs imported via super
// termchar.h 75's `struct cm *Wcm` — the type cm.h defines, so its mirror comes in here and
// every unit that carries termchar.rs carries cm.h too (C's tty units all do).
// [U1] cm.rs imported via super

/* Each termcap frame points to its own struct tty_output object in the
   output_data.tty field.  The tty_output structure contains the
   information that is specific to termcap frames. */

// termchar.h 28-34.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct tty_output
{
  /* The Emacs structure for the tty device this frame is on. */
  pub display_info: *mut tty_display_info,
}

// termchar.h 36.
pub const TERMCAP_BUFFER_SIZE: c_int = 4096;

/* Parameters that are shared between frames on the same tty device. */

// termchar.h 40-222.  Measured by the layout probe: the two `char[4096]` buffers sit where C
// puts them, and every `bool_bf` run is one byte (see the file comment).
#[repr(C)]
pub struct tty_display_info
{
  pub next: *mut tty_display_info, /* Chain of all tty devices. */

  pub name: *mut c_char,           /* The name of the device file or 0 if
                                      stdin/stdout. */
  pub r#type: *mut c_char,         /* The type of the tty. */

  /* Input/output */

  pub input: *mut FILE,            /* The stream to be used for terminal input.
                                      NULL if the terminal is suspended. */
  pub output: *mut FILE,           /* The stream to be used for terminal output.
                                      NULL if the terminal is suspended. */

  /* Size of output buffer.  A value of zero means use the default of
     BUFIZE.  If non-zero, also minimize writes to the tty by avoiding
     calls to flush.  */
  pub output_buffer_size: ptrdiff_t,

  pub termscript: *mut FILE,       /* If nonzero, send all terminal output
                                      characters to this stream also.  */

  pub old_tty: *mut c_void,        /// C: `struct emacs_tty *old_tty` (sysdep.h; P-72 ③)

  // termchar.h 62-64: `bool_bf term_initted : 1;` — a run of one, so one byte.
  pub bits_initted: u8,

  pub reference_count: c_int,      /* Number of frames that are on this display. */

  pub terminal: *mut terminal,     /* Points back to the generic terminal
                                      structure.  This is sometimes handy. */

  /* Info on cursor positioning.  */
  pub Wcm: *mut cm,

  /* Redisplay. */

  pub top_frame: Lisp_Object,      /* The topmost frame on this tty. */

  /* The previous frame we displayed on this tty.  */
  pub previous_frame: *mut frame,
  pub previous_color_mode: c_int,

  /* Information about the range of text currently shown in
     mouse-face.  */
  pub mouse_highlight: Mouse_HLInfo,

  /* Buffer used internally by termcap (see tgetent in the Termcap
     manual).  Only init_tty should use this.  */
  pub termcap_term_buffer: [c_char; TERMCAP_BUFFER_SIZE as usize],

  /* Buffer storing terminal description strings (see tgetstr in the
     Termcap manual).  Only init_tty should use this.  */
  pub termcap_strings_buffer: [c_char; TERMCAP_BUFFER_SIZE as usize],

  /* Strings, numbers and flags taken from the termcap entry.  */

  pub TS_ins_line: *const c_char,             /* "al" */
  pub TS_ins_multi_lines: *const c_char,      /* "AL" (one parameter, # lines to insert) */
  pub TS_bell: *const c_char,                 /* "bl" */
  pub TS_clr_to_bottom: *const c_char,        /* "cd" */
  pub TS_clr_line: *const c_char,             /* "ce", clear to end of line */
  pub TS_clr_frame: *const c_char,            /* "cl" */
  pub TS_set_scroll_region: *const c_char,    /* "cs" (2 params, first line and last line) */
  pub TS_set_scroll_region_1: *const c_char,  /* "cS" (4 params: total lines,
                                                 lines above scroll region, lines below it,
                                                 total lines again) */
  pub TS_del_char: *const c_char,             /* "dc" */
  pub TS_del_multi_chars: *const c_char,      /* "DC" (one parameter, # chars to delete) */
  pub TS_del_line: *const c_char,             /* "dl" */
  pub TS_del_multi_lines: *const c_char,      /* "DL" (one parameter, # lines to delete) */
  pub TS_delete_mode: *const c_char,          /* "dm", enter character-delete mode */
  pub TS_end_delete_mode: *const c_char,      /* "ed", leave character-delete mode */
  pub TS_end_insert_mode: *const c_char,      /* "ei", leave character-insert mode */
  pub TS_ins_char: *const c_char,             /* "ic" */
  pub TS_ins_multi_chars: *const c_char,      /* "IC" (one parameter, # chars to insert) */
  pub TS_insert_mode: *const c_char,          /* "im", enter character-insert mode */
  pub TS_pad_inserted_char: *const c_char,    /* "ip".  Just padding, no commands.  */
  pub TS_end_keypad_mode: *const c_char,      /* "ke" */
  pub TS_keypad_mode: *const c_char,          /* "ks" */
  pub TS_pad_char: *const c_char,             /* "pc", char to use as padding */
  pub TS_repeat: *const c_char,               /* "rp" (2 params, # times to repeat
                                                 and character to be repeated) */
  pub TS_end_standout_mode: *const c_char,    /* "se" */
  pub TS_fwd_scroll: *const c_char,           /* "sf" */
  pub TS_standout_mode: *const c_char,        /* "so" */
  pub TS_rev_scroll: *const c_char,           /* "sr" */
  pub TS_end_termcap_modes: *const c_char,    /* "te" */
  pub TS_termcap_modes: *const c_char,        /* "ti" */
  pub TS_visible_bell: *const c_char,         /* "vb" */
  pub TS_cursor_normal: *const c_char,        /* "ve" */
  pub TS_cursor_visible: *const c_char,       /* "vs" */
  pub TS_cursor_invisible: *const c_char,     /* "vi" */
  pub TS_set_window: *const c_char,           /* "wi" (4 params, start and end of window,
                                                 each as vpos and hpos) */

  pub TS_enter_bold_mode: *const c_char,      /* "md" -- turn on bold (extra bright mode).  */
  pub TS_enter_italic_mode: *const c_char,    /* "ZH" -- turn on italics mode.  */
  pub TS_enter_dim_mode: *const c_char,       /* "mh" -- turn on half-bright mode.  */
  pub TS_enter_reverse_mode: *const c_char,   /* "mr" -- enter reverse video mode.  */
  pub TS_exit_underline_mode: *const c_char,  /* "us" -- start underlining.  */
  pub TS_enter_underline_mode: *const c_char, /* "ue" -- end underlining.  */
  pub TS_enter_strike_through_mode: *const c_char, /* "smxx" -- turn on strike-through
                                                      mode.  */
  pub TS_enter_overline_mode: *const c_char,  /* "Smol" -- turn on overline mode.  */

  /* "as"/"ae" -- start/end alternate character set.  Not really
     supported, yet.  */
  pub TS_enter_alt_charset_mode: *const c_char,
  pub TS_exit_alt_charset_mode: *const c_char,

  pub TS_exit_attribute_mode: *const c_char,  /* "me" -- switch appearances off.  */

  /* Value of the "NC" (no_color_video) capability, or 0 if not present.  */
  pub TN_no_color_video: c_int,

  pub TN_max_colors: c_int,                   /* "Co" -- number of colors.  */

  /* "op" -- SVr4 set default pair to its original value.  */
  pub TS_orig_pair: *const c_char,

  pub TS_set_foreground: *const c_char,       /* "AF"/"Sf"/"setrgbf" -- set foreground color. */
  pub TS_set_background: *const c_char,       /* "AB"/"Sb"/"setrgbb" -- set background color. */
  /* If set, TS_set_foreground and TS_set_background take 3 separate
     params for R, G, and B values ("setrgbf"/"setrgbb" -- non-standard).
     If unset, they take 1 param ("AF"/"AB" or "Sf"/"Sb" -- ANSI or SVr4r). */
  pub TF_rgb_separate: c_int,

  pub TF_hazeltine: c_int,           /* termcap hz flag. */
  pub TF_insmode_motion: c_int,      /* termcap mi flag: can move while in insert mode. */
  pub TF_standout_motion: c_int,     /* termcap mi flag: can move while in standout mode. */
  pub TF_underscore: c_int,          /* termcap ul flag: _ underlines if over-struck on
                                        non-blank position.  Must clear before writing _.  */
  pub TF_teleray: c_int,             /* termcap xt flag: many weird consequences.
                                        For t1061. */
  pub TF_set_underline_style: *const c_char, /* termcap Smulx entry: Switches the underline
                                                style based on the parameter.  Param should
                                                be one of: 0 (none), 1 (straight), 2 (double-line),
                                                3 (wave), 4 (dots), or 5 (dashes).  */
  pub TF_set_underline_color: *const c_char, /* Enabled when TF_set_underline_style is set:
                                                Sets the color of the underline.  Accepts a
                                                single parameter, the color index.  */

  pub RPov: c_int,                   /* # chars to start a TS_repeat */

  // termchar.h 174-183: four consecutive `bool_bf : 1` members, i.e. one byte (bits 0-3).
  pub bits_modes: u8,

  /* 1 if should obey 0200 bit in input chars as "Meta", 2 if should
     keep 0200 bit in input chars.  0 to ignore the 0200 bit.  */

  pub meta_key: c_int,

  /* Size of window specified by higher levels.
   This is the number of lines, from the top of frame downwards,
   which can participate in insert-line/delete-line operations.

   Effectively it excludes the bottom frame_lines - specified_window_size
   lines from those operations.  */

  pub specified_window: c_int,

  // termchar.h 196-213: eight consecutive `bool_bf : 1` members, i.e. one byte (bits 0-7).
  pub bits_caps: u8,

  /* Cost of setting the scroll window, measured in characters.  */
  pub scroll_region_cost: c_int,
}

// The masks the `bits_*` bytes above carry — one per `bool_bf : 1` member, in C's declaration
// order (termchar.h 62, 174-177, 196-213).  Use sites read them the way `itree.rs` reads
// `itree_node.bits` (`(*t).bits_caps & TTY_SCROLL_REGION_OK != 0`), and `docs/ports/
// token-aliases.tsv` keeps the two spellings comparable.
pub const TTY_TERM_INITTED: u8 = 1 << 0;           // termchar.h 62

pub const TTY_DELETE_IN_INSERT_MODE: u8 = 1 << 0;  // termchar.h 174
pub const TTY_COSTS_SET: u8 = 1 << 1;              // termchar.h 176
pub const TTY_INSERT_MODE: u8 = 1 << 2;            // termchar.h 178
pub const TTY_STANDOUT_MODE: u8 = 1 << 3;          // termchar.h 179

pub const TTY_CURSOR_HIDDEN: u8 = 1 << 0;          // termchar.h 196
pub const TTY_FLOW_CONTROL: u8 = 1 << 1;           // termchar.h 200
pub const TTY_SHOWING_MENU: u8 = 1 << 2;           // termchar.h 203
pub const TTY_MUST_WRITE_SPACES: u8 = 1 << 3;      // termchar.h 207
pub const TTY_LINE_INS_DEL_OK: u8 = 1 << 4;        // termchar.h 210
pub const TTY_CHAR_INS_DEL_OK: u8 = 1 << 5;        // termchar.h 213
pub const TTY_SCROLL_REGION_OK: u8 = 1 << 6;       // termchar.h 216
pub const TTY_MEMORY_BELOW_FRAME: u8 = 1 << 7;     // termchar.h 219

/* A chain of structures for all tty devices currently in use. */
unsafe extern "C" { pub static mut tty_list: *mut tty_display_info; }     // termchar.h 227

// termchar.h 230-233: `FRAME_TTY (f)`.  It is a macro in C and stays one here: it names
// `frame`'s `output_method` and `struct terminal`'s `display_info.tty`, which the including
// unit brings in through the mirrors above.  The `emacs_abort ()` arm is the
// `SELECTED_FRAME` shape `frame.rs` already carries (frame.h 1459-1462).
macro_rules! FRAME_TTY {
  ($f:expr) => {{
    let f = $f;
    if output_method (f) == output_termcap || output_method (f) == output_msdos_raw
    { unsafe { (*(*f).terminal).display_info.tty }.cast::<tty_display_info> () }
    else
    { unsafe { emacs_abort () }; 0 as *mut tty_display_info }
  }};
}

// termchar.h 235: `CURTTY ()`.
macro_rules! CURTTY {
  () => {{ FRAME_TTY! (SELECTED_FRAME ()) }};
}

// termchar.h 237: the tag the declaration below names; C sees it as incomplete here.
// `struct input_event` lives in termhooks.h, whose mirror declares it (PORTING.md P-70).
// termchar.h 238-239.
unsafe extern "C" {
  pub fn tty_handle_tab_bar_click (f: *mut frame, x: c_int, y: c_int, down: bool,
                                   event: *mut input_event) -> Lisp_Object;
}

// termchar.h 241.
unsafe extern "C" { pub fn tty_setup_colors (tty: *mut tty_display_info, mode: c_int); }
