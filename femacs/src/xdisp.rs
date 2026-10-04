// femacs: port of src/xdisp.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// struct, struct field, macro, file-scope object, local), same control flow, same error
// model.  C's non-`static` definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2
// `unsafe(no_mangle)`/P-52); the `static` helpers stay plain `unsafe fn` (and
// `unsafe extern "C" fn` where C's is variadic or its address is handed out).  The 24
// `DEFUN`s use `DEFUN!` (PORTING.md 5.1).
//
// **This lane is incremental.**  `src/xdisp.c` is 39,805 lines, the largest unit in the
// tree; this file is being written in C order and is complete through the C line range named
// in `docs/ports/xdisp.md`'s Decisions section ("Progress").  Everything past that line is
// still to come; the names this file's transcribed text calls but does not define yet are in
// the `TEMPORARY SCAFFOLD` block at the end of this file, one `/// C:` prototype each, and
// each is deleted as its definition lands (the ledger in `docs/ports/xdisp.md` tracks them).
//
// **`src/xdisp.c` is in `docs/baseline/config/cond-eval-verified.txt`**
// (`python3 tools/check-cond-eval.py src/xdisp.c` reports
// `196 conditions, 0 unknown, 0 disagreements [VERIFIED]`), so per P-08 the arms the frozen
// configuration does not compile are *recorded in `docs/ports/xdisp.md`, not transcribed* —
// the list of them is in that ledger's Decisions section.  In particular the whole
// `HAVE_WINDOW_SYSTEM` half of this unit (the GUI redisplay: fringe/scroll-bar drawing,
// tool/tab bars, mouse-face highlighting, hourglass) is one gated arm and does not appear
// here as text.
//
// **Use-site declarations.**  `window.h`'s mirror is WIP (P-66): it carries the two
// records, the nineteen packed flag shifts, the read-only accessors and the redisplay entry
// points, but not the accessor macros that dereference `struct frame` (`WINDOW_XFRAME`,
// `WINDOW_FRAME_COLUMN_WIDTH`, the fringe/scroll-bar geometry, `WINDOW_*_DIVIDER_WIDTH`,
// `WINDOW_MODE_LINE_HEIGHT` and the `CURRENT_*_LINE_HEIGHT` family).  As in `window.rs`,
// those are defined here under C's own name with the C header's line in a `/// C:` note
// (PORTING.md 0.1-2/P-72 rule 3), and the mirror gaps this unit reports and does not touch
// are listed in `docs/ports/xdisp.md` for the integrator.
//
// **`struct it`.**  `dispextern.h` is mirrored only to its batch-II prefix, so the display
// iterator is declared here as this unit's own measured copy (the `window.rs`/`indent.rs`
// precedent): the members this unit reaches are named, every other member is inside a
// measured pad, and the whole layout is pinned by the `const _` assertions below.  Every
// offset below was measured by clang on the frozen target with the `spikes/run.sh` flags.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_assignments,
         unused_unsafe, static_mut_refs, unsafe_op_in_unsafe_fn, unused_parens,
         unused_labels, unreachable_code, unused_comparisons,
         unused_must_use, overflowing_literals, clashing_extern_declarations,
         unused_must_use, clippy::all)]

/* C gets `VaList` from <stdarg.h>; the port's equivalent is `core::ffi::VaList` (PORTING.md
   P-04, the doprnt.rs/pdumper.rs shape).  */
use core::ffi::VaList;

/* C 20-466, the reference's "New redisplay" essay, is kept verbatim below; the port's own
   `#![allow (…)]` prelude has to come first because that is an inner attribute.  */


/* New redisplay written by Gerd Moellmann <gerd@gnu.org>.

   Redisplay.

   Emacs separates the task of updating the display -- which we call
   "redisplay" -- from the code modifying global state, e.g. buffer
   text.  This way functions operating on buffers don't also have to
   be concerned with updating the display as result of their
   operations.

   Redisplay is triggered by the Lisp interpreter when it decides it's
   time to do it.  This is done either automatically for you as part
   of the interpreter's command loop, or as the result of calling Lisp
   functions like `sit-for'.  The C function `redisplay_internal' in
   xdisp.c is the only entry into the inner redisplay code.

   The following diagram shows how redisplay code is invoked.  As you
   can see, Lisp calls redisplay and vice versa.

   Under window systems like X, some portions of the redisplay code
   are also called asynchronously, due to mouse movement or expose
   events.  "Asynchronously" in this context means that any C function
   which calls maybe_quit or process_pending_signals could enter
   redisplay via expose_frame and/or note_mouse_highlight, if X events
   were recently reported to Emacs about mouse movements or frame(s)
   that were exposed.  And such redisplay could invoke the Lisp
   interpreter, e.g. via the :eval forms in mode-line-format, and as
   result the global state could change.  It is therefore very
   important that C functions which might cause such "asynchronous"
   redisplay, but cannot tolerate the results, use
   block_input/unblock_input around code fragments which assume that
   global Lisp state doesn't change.  If you don't follow this rule,
   you will encounter bugs which are very hard to explain.  One place
   that needs to take such precautions is timer_check, some of whose
   code cannot tolerate changes in timer alists while it processes
   timers.

   +--------------+   redisplay     +----------------+
   | Lisp machine |---------------->| Redisplay code |<--+
   +--------------+   (xdisp.c)     +----------------+   |
	  ^				     |		 |
	  +----------------------------------+           |
	    Block input to prevent this when             |
	    called asynchronously!			 |
							 |
		    note_mouse_highlight (asynchronous)	 |
							 |
				    X mouse events  -----+
							 |
			    expose_frame (asynchronous)	 |
							 |
				   X expose events  -----+

   What does redisplay do?  Obviously, it has to figure out somehow what
   has been changed since the last time the display has been updated,
   and to make these changes visible.  Preferably it would do that in
   a moderately intelligent way, i.e. fast.

   At its highest level, redisplay can be divided into 3 distinct
   steps, all of which are visible in `redisplay_internal':

    . decide which frames need their windows to be considered for redisplay
    . for each window whose display might need to be updated, compute
      a structure, called "glyph matrix", which describes how it
      should look on display
    . actually update the display of windows on the glass where the
      newly obtained glyph matrix differs from the one produced by the
      previous redisplay cycle

   The first of these steps is done by `redisplay_internal' itself, by
   looping through all the frames and testing their various flags,
   such as their visibility.  The result of this could be that only
   the selected window on the selected frame must be redisplayed, or
   it could conclude that other windows need to be considered as well.

   The second step considers each window that might need to be
   redisplayed.  This could be only the selected window, or the window
   trees of one or more frames.  The function which considers a window
   and decides whether it actually needs redisplay is
   `redisplay_window'.  It does so by looking at the changes in
   position of point, in buffer text, in text properties, overlays,
   etc.  These changes can be deduced from window and buffer
   structures, and from some global variables like `beg_unchanged' and
   `end_unchanged'.  The current contents of the display are recorded
   in a `glyph matrix', a two-dimensional matrix of glyph structures.
   Each row in such a matrix corresponds to a line on the display, and
   each glyph in a row corresponds to a column displaying a character,
   an image, or what else.  This matrix is called the `current glyph
   matrix', or `current matrix', in redisplay terminology.

   For buffer parts that have been changed since the last redisplay,
   `redisplay_window' constructs a second glyph matrix, the so called
   `desired glyph matrix' or short `desired matrix'.  It does so in
   the most optimal way possible, avoiding the examination of text
   that didn't change, reusing portions of the current matrix if
   possible, etc.  It could, in particular, decide that a window
   doesn't need to be redisplayed at all.

   This second step of redisplay also updates the parts of the desired
   matrix that correspond to the mode lines, header lines, and
   tab-lines of the windows which need that; see `display_mode_lines'.

   In the third and last step, the current and desired matrix are then
   compared to find a cheap way to update the display, e.g. by reusing
   part of the display by scrolling lines.  The actual update of the
   display of each window, by comparing the desired and the current
   matrix, is done by `update_window', which calls functions which
   draw to the glass (those functions are specific to the type of the
   window's frame: X, w32, NS, etc.).

   Once the display of a window on the glass has been updated, its
   desired matrix is used to update the corresponding rows of the
   current matrix, and then the desired matrix is discarded.

   You will find a lot of redisplay optimizations when you start
   looking at the innards of `redisplay_window'.  The overall goal of
   all these optimizations is to make redisplay fast because it is
   done frequently.  Some of these optimizations are implemented by
   the following functions:

    . try_cursor_movement

      This optimization is applicable if the text in the window did
      not change and did not scroll, only point moved, and it did not
      move off the displayed portion of the text.  In that case, the
      window's glyph matrix is still valid, and only the position of
      the cursor might need to be updated.

    . try_window_reusing_current_matrix

      This function reuses the current glyph matrix of a window when
      text has not changed, but the window start changed (e.g., due to
      scrolling).

    . try_window_id

      This function attempts to update a window's glyph matrix by
      reusing parts of its current glyph matrix.  It finds and reuses
      the part that was not changed, and regenerates the rest.  (The
      "id" part in the function's name stands for "insert/delete", not
      for "identification" or somesuch.)

    . try_window

      This function performs the full, unoptimized, generation of a
      single window's glyph matrix, assuming that its fonts were not
      changed and that the cursor will not end up in the scroll
      margins.  (Loading fonts requires re-adjustment of dimensions of
      glyph matrices, which makes this method impossible to use.)

   The optimizations are tried in sequence (some can be skipped if
   it is known that they are not applicable).  If none of the
   optimizations were successful, redisplay calls redisplay_windows,
   which performs a full redisplay of all windows.

   Note that there's one more important optimization up Emacs's
   sleeve, but it is related to actually redrawing the potentially
   changed portions of the window/frame as part of the third step, not
   to generating the desired matrices of those potentially changed
   portions.  Namely, the function `update_frame' and its subroutines,
   which you will find in dispnew.c, compare the desired matrices with
   the current matrices, and only redraw the portions that changed.
   So it could happen that the functions in this file for some reason
   decide that the entire desired matrix needs to be regenerated from
   scratch, and still only parts of the Emacs display, or even nothing
   at all, will be actually delivered to the glass, because
   `update_frame' has found that the new and the old screen contents
   are similar or identical.

   Desired matrices.

   Desired matrices are always built per Emacs window.  It is
   important to know that a desired matrix is in general "sparse": it
   only has some of the glyph rows "enabled".  This is because
   redisplay tries to optimize its work, and thus only generates
   glyphs for rows that need to be updated on the screen.  Rows that
   don't need to be updated are left "disabled", and their contents
   in the desired matrix should be ignored.

   The function `display_line' is the central function to look at if
   you are interested in how the rows of the desired matrix are
   produced.  It constructs one row in a desired matrix given an
   iterator structure containing both a buffer position and a
   description of the environment in which the text is to be
   displayed.  But this is too early, read on.

   Glyph rows.

   A glyph row is an array of `struct glyph', where each glyph element
   describes a "display element" to be shown on the screen.  More
   accurately, a glyph row can have up to 3 different arrays of
   glyphs: one each for every display margins, and one for the "text
   area", where buffer text is displayed.  The text-area glyph array
   is always present, whereas the arrays for the marginal areas are
   present (non-empty) only if the corresponding display margin is
   shown in the window.  If the glyph array for a marginal area is not
   present its beginning and end coincide, i.e. such arrays are
   actually empty (they contain no glyphs).  Frame glyph matrices, used
   on text-mode terminals (see below) never have marginal areas; they
   treat the entire frame-wide row of glyphs as a single large "text
   area".

   Iteration over buffer and strings.

   Characters and pixmaps displayed for a range of buffer text depend
   on various settings of buffers and windows, on overlays and text
   properties, on display tables, on selective display.  The good news
   is that all this hairy stuff is hidden behind a small set of
   interface functions taking an iterator structure (`struct it')
   argument.

   Iteration over things to be displayed is then simple.  It is
   started by initializing an iterator with a call to `init_iterator',
   passing it the buffer position where to start iteration.  For
   iteration over strings, pass -1 as the position to `init_iterator',
   and call `reseat_to_string' when the string is ready, to initialize
   the iterator for that string.  Thereafter, calls to
   `get_next_display_element' fill the iterator structure with
   relevant information about the next thing to display.  Calls to
   `set_iterator_to_next' move the iterator to the next thing.

   Besides this, an iterator also contains information about the
   display environment in which glyphs for display elements are to be
   produced.  It has fields for the width and height of the display,
   the information whether long lines are truncated or continued, a
   current X and Y position, the face currently in effect, and lots of
   other stuff you can better see in dispextern.h.

   The "stop position".

   Some of the fields maintained by the iterator change relatively
   infrequently.  These include the face of the characters, whether
   text is invisible, the object (buffer or display or overlay string)
   being iterated, character composition info, etc.  For any given
   buffer or string position, the sources of information that affects
   the display can be determined by calling the appropriate
   primitives, such as `Fnext_single_property_change', but both these
   calls and the processing of their return values is relatively
   expensive.  To optimize redisplay, the display engine checks these
   sources of display information only when needed, not for every
   character.  To that end, it always maintains the position of the
   next place where it must stop and re-examine all those potential
   sources.  This is called "the stop position" and is stored in the
   `stop_charpos' field of the iterator.  The stop position is updated
   by `compute_stop_pos', which is called whenever the iteration
   reaches the current stop position and processes it.  Processing a
   stop position is done by `handle_stop', which invokes a series of
   handlers, one each for every potential source of display-related
   information; see the `it_props' array for those handlers.  For
   example, one handler is `handle_face_prop', which detects changes
   in face properties, and supplies the face ID that the iterator will
   use for all the glyphs it generates up to the next stop position;
   this face ID is the result of "realizing" the face specified by the
   relevant text properties at this position (see xfaces.c).  Each
   handler called by `handle_stop' processes the sources of display
   information for which it is "responsible", and returns a value
   which tells `handle_stop' what to do next.

   Once `handle_stop' returns, the information it stores in the
   iterator fields will not be refreshed until the iteration reaches
   the next stop position, which is computed by `compute_stop_pos'
   called at the end of `handle_stop'.  `compute_stop_pos' examines
   the buffer's or string's interval tree to determine where the text
   properties change, finds the next position where overlays and
   character composition can change, and stores in `stop_charpos' the
   closest position where any of these factors should be reconsidered.

   Handling of the stop position is done as part of the code in
   `get_next_display_element'.

   Producing glyphs.

   Glyphs in a desired matrix are normally constructed in a loop
   calling `get_next_display_element' and then `PRODUCE_GLYPHS'.  The
   call to `PRODUCE_GLYPHS' will fill the iterator structure with
   pixel information about the element being displayed and at the same
   time will produce glyphs for it.  If the display element fits on
   the line being displayed, `set_iterator_to_next' is called next,
   otherwise the glyphs produced are discarded, and `display_line'
   marks this glyph row as a "continued line".  The function
   `display_line' is the workhorse of filling glyph rows in the
   desired matrix with glyphs.  In addition to producing glyphs, it
   also handles line truncation and continuation, word wrap, and
   cursor positioning (for the latter, see `set_cursor_from_row').

   Frame matrices.

   That just couldn't be all, could it?  What about terminal types not
   supporting operations on sub-windows of the screen (a.k.a. "TTY" or
   "text-mode terminals")?  To update the display on such a terminal,
   window-based glyph matrices are not well suited.  To be able to
   reuse part of the display (scrolling lines up and down), we must
   instead have a view of the whole screen.  This is what `frame
   matrices' are for.  They are a trick.

   Frames on text terminals have a glyph pool.  Windows on such a
   frame sub-allocate their glyph memory from their frame's glyph
   pool.  The frame itself is given its own glyph matrices.  By
   coincidence---or maybe something else---rows in window glyph
   matrices are slices of corresponding rows in frame matrices.  Thus
   writing to window matrices implicitly updates a frame matrix which
   provides us with the view of the whole screen that we originally
   wanted to have without having to move many bytes around.  Then
   updating all the visible windows on text-terminal frames is done by
   using the frame matrices, which allows frame-global optimization of
   what is actually written to the glass.

   Frame matrices don't have marginal areas, only a text area.  That
   is, the entire row of glyphs that spans the width of a text-mode
   frame is treated as a single large "text area" for the purposes of
   manipulating and updating a frame glyph matrix.

   To be honest, there is a little bit more done for frame matrices,
   but not much more.  If you plan to extend that code, take a look at
   dispnew.c.  The function build_frame_matrix is a good starting
   point.

   Simulating display.

   Some of Emacs commands and functions need to take display layout
   into consideration.  For example, C-n moves to the next screen
   line, but to implement that, Emacs needs to find the buffer
   position which is directly below the cursor position on display.
   This is not trivial when buffer display includes variable-size
   elements such as different fonts, tall images, etc.

   To solve this problem, the display engine implements several
   functions that can move through buffer text in the same manner as
   `display_line' and `display_string' do, but without producing any
   glyphs for the glyph matrices.  The workhorse of this is
   `move_it_in_display_line_to'.  Its code and logic are very similar
   to `display_line', but it differs in two important aspects: it
   doesn't produce glyphs for any glyph matrix, and it returns a
   status telling the caller how it ended the iteration: whether it
   reached the required position, hit the end of line, arrived at the
   window edge without exhausting the buffer's line, etc.  Since the
   glyphs are not produced, the layout information available to the
   callers of this function is what is recorded in `struct it' by the
   iteration process.

   Several higher-level functions call `move_it_in_display_line_to' to
   perform more complex tasks: `move_it_by_lines' can move N lines up
   or down from a given buffer position and `move_it_to' can move to a
   given buffer position or to a given X or Y pixel coordinate.

   These functions are called by the display engine itself as well,
   when it needs to make layout decisions before producing the glyphs.
   For example, one of the first things to decide when redisplaying a
   window is where to put the `window-start' position; if the window
   is to be recentered (the default), Emacs makes that decision by
   starting from the position of point, then moving up the number of
   lines corresponding to half the window height using
   `move_it_by_lines'.

   Bidirectional display.

   Bidirectional display adds quite some hair to this already complex
   design.  The good news are that a large portion of that hairy stuff
   is hidden in bidi.c behind only 3 interfaces.  bidi.c implements a
   reordering engine which is called by `set_iterator_to_next' and
   returns the next character to display in the visual order.  See
   commentary on bidi.c for more details.  As far as redisplay is
   concerned, the effect of calling `bidi_move_to_visually_next', the
   main interface of the reordering engine, is that the iterator gets
   magically placed on the buffer or string position that is to be
   displayed next in the visual order.  In other words, a linear
   iteration through the buffer/string is replaced with a non-linear
   one.  All the rest of the redisplay is oblivious to the bidi
   reordering.

   Well, almost oblivious---there are still complications, most of
   them due to the fact that buffer and string positions no longer
   change monotonously with glyph indices in a glyph row.  Moreover,
   for continued lines, the buffer positions may not even be
   monotonously changing with vertical positions.  Also, accounting
   for face changes, overlays, etc. becomes more complex because
   non-linear iteration could potentially skip many positions with
   such changes, and then cross them again on the way back (see
   `handle_stop_backwards')...

   One other prominent effect of bidirectional display is that some
   paragraphs of text need to be displayed starting at the right
   margin of the window---the so-called right-to-left, or R2L
   paragraphs.  R2L paragraphs are displayed with R2L glyph rows,
   which have their `reversed_p' flag set.  The bidi reordering engine
   produces characters in such rows starting from the character which
   should be the rightmost on display.  `PRODUCE_GLYPHS' then reverses
   the order, when it fills up the glyph row whose `reversed_p' flag
   is set, by prepending each new glyph to what is already there,
   instead of appending it.  When the glyph row is complete, the
   function `extend_face_to_end_of_line' fills the empty space to the
   left of the leftmost character with special glyphs, which will
   display as, well, empty.  On text terminals, these special glyphs
   are simply blank characters.  On graphics terminals, there's a
   single stretch glyph of a suitably computed width.  Both the blanks
   and the stretch glyph are given the face of the background of the
   line.  This way, the terminal-specific back-end can still draw the
   glyphs left to right, even for R2L lines.

   Bidirectional display and character compositions.

   Some scripts cannot be displayed by drawing each character
   individually, because adjacent characters change each other's shape
   on display.  For example, Arabic and Indic scripts belong to this
   category.

   Emacs display supports this by providing "character compositions",
   most of which is implemented in composite.c.  During the buffer
   scan that delivers characters to `PRODUCE_GLYPHS', if the next
   character to be delivered is a composed character, the iteration
   calls `composition_reseat_it' and `next_element_from_composition'.
   If they succeed to compose the character with one or more of the
   following characters, the whole sequence of characters that were
   composed is recorded in the `struct composition_it' object that is
   part of the buffer iterator.  The composed sequence could produce
   one or more font glyphs (called "grapheme clusters") on the screen.
   Each of these grapheme clusters is then delivered to
   `PRODUCE_GLYPHS' in the direction corresponding to the current bidi
   scan direction (recorded in the `scan_dir' member of the `struct
   bidi_it' object that is part of the iterator).  In particular, if
   the bidi iterator currently scans the buffer backwards, the
   grapheme clusters are delivered back to front.  This reorders the
   grapheme clusters as appropriate for the current bidi context.
   Note that this means that the grapheme clusters are always stored
   in the `LGSTRING' object (see composite.c) in the logical order.

   Moving an iterator in bidirectional text
   without producing glyphs.

   Note one important detail mentioned above: that the bidi reordering
   engine, driven by the iterator, produces characters in R2L rows
   starting at the character that will be the rightmost on display.
   As far as the iterator is concerned, the geometry of such rows is
   still left to right, i.e. the iterator "thinks" the first character
   is at the leftmost pixel position.  The iterator does not know that
   `PRODUCE_GLYPHS' reverses the order of the glyphs that the iterator
   delivers.  This is important when functions from the `move_it_*'
   family are used to get to certain screen position or to match
   screen coordinates with buffer coordinates: these functions use the
   iterator geometry, which is left to right even in R2L paragraphs.
   This works well with most callers of `move_it_*', because they need
   to get to a specific column, and columns are still numbered in the
   reading order, i.e. the rightmost character in a R2L paragraph is
   still column zero.  But some callers do not get well with this; a
   notable example is mouse clicks that need to find the character
   that corresponds to certain pixel coordinates.  See
   `buffer_posn_from_coords' in dispnew.c for how this is handled.  */

/* C 468-502: the `#include`s, each with C's own header named.  `termopts.h` (C 492),
   `fontset.h` (C 497) and `xwidget.h` (C 499) have no mirror yet; nothing this lane
   transcribes names anything from them, and the ones that do land with their use site
   (PORTING.md P-72 rule 3).  `TERM_HEADER` (C 501) is the platform terminal header and is
   only reached under `HAVE_WINDOW_SYSTEM`, which is dead on this target (P-08).  */
include!("include/config.rs");                 // C 468: `#include <config.h>`
include!("../lib/include/stdlib.rs");          // C 469: `#include <stdlib.h>`
include!("../lib/include/limits.in.rs");       // C 470: `#include <limits.h>` (`INT_MAX`)
include!("../lib/include/math.rs");            // C 471: `#include <math.h>`
include!("../lib/include/stdio.rs");           // C 477: `#include "sysstdio.h"`
include!("../lib/include/stdint.in.rs");       // lisp.h: the width family
include!("../lib/include/string.rs");          // lisp.h 30: `#include <string.h>`
include!("../lib/include/stdcountof.rs");      // lisp.h 28: `#include <stdcountof.h>`
include!("include/lisp.rs");                   // C 473: `#include "lisp.h"`
include!("include/character.rs");              // C 484: `#include "character.h"`
include!("include/category.rs");               // C 485: `#include "category.h"`
include!("include/atimer.rs");                 // C 474: `#include "atimer.h"`
include!("include/composite.rs");              // C 475: `#include "composite.h"`
include!("include/intervals.rs");              // C 493: `#include "intervals.h"`
include!("include/itree.rs");                  // intervals.h: `itree_node`/`itree_tree`
include!("include/systime.rs");                // C 478: `#include "systime.h"`
include!("include/coding.rs");                 // C 494: `#include "coding.h"`
include!("include/charset.rs");                // C 486: `#include "charset.h"`
include!("include/font.rs");                   // C 496: `#include "font.h"`
include!("include/termhooks.rs");              // C 491: `#include "termhooks.h"`
include!("include/frame.rs");                  // C 479: `#include "frame.h"`
include!("include/window.rs");                 // C 480: `#include "window.h"`
include!("include/keyboard.rs");               // C 476: `#include "keyboard.h"`
include!("include/keymap.rs");                 // C 489: `#include "keymap.h"`
include!("include/buffer.rs");                 // C 485: `#include "buffer.h"`
include!("include/blockinput.rs");             // C 498: `#include "blockinput.h"`
include!("include/termchar.rs");               // C 481: `#include "termchar.h"`
include!("include/disptab.rs");                // C 490: `#include "disptab.h"`
include!("include/dispextern.rs");             // C 482: `#include "dispextern.h"`
include!("include/commands.rs");               // C 488: `#include "commands.h"`
include!("include/indent.rs");                 // C 487: `#include "indent.h"`
include!("include/region-cache.rs");           // C 495: `#include "region-cache.h"`
include!("include/pdumper.rs");                // lisp.h: `pdumper` declarations

/* C spells `enum itree_order`'s members bare (`DESCENDING` inside `ITREE_FOREACH`), and
   Rust 2024 keeps enum variants out of scope, so the port imports them (the
   buffer.rs/textprop.rs shape).  */
use itree_order::*;

/* C 504-506: `#ifndef FRAME_OUTPUT_DATA` / `#define FRAME_OUTPUT_DATA(f) (NULL)`.  The
   frozen configuration has no window system, so `FRAME_OUTPUT_DATA` is C's NULL fallback
   (frame.h only defines it again under `HAVE_WINDOW_SYSTEM`).  */
macro_rules! FRAME_OUTPUT_DATA {
  ($f:expr) => { { let _ = $f; core::ptr::null_mut () } };
}

/* The `window.h`/`dispextern.h` macro families this unit used to declare here — the
   `WINDOW_*` accessors, the divider widths and frame edges, the scroll-bar geometry, the
   `WINDOW_{MODE,HEADER,TAB}_LINE_HEIGHT` triple, and the `CURRENT_{MODE,HEADER,TAB}_LINE_HEIGHT`
   family with its `MATRIX_*_LINE_HEIGHT` helpers — are carried by `include/window.rs` and
   `include/dispextern.rs`, and the unit names the mirror's copies (the integrator's
   mirror-dupes sweep, 2026-10-05).  `window_bits`/`window_bits_shift` below stay: the mirror
   carries `window_bit` (the reader) and the shift constants, not these two names.  */
/* window.h 392-410: the nineteen `bool_bf : 1` members share the unit `bits`; the reader is
   in the mirror (`window_bit`), the writer is here (PORTING.md 1.3/P-80).  */
macro_rules! window_bits {
    ($w:expr, $name:ident) => { window_bit ($w, window_bits_shift! ($name)) };
    ($w:expr, $name:ident, $v:expr) => {{
        (*$w).bits = ((*$w).bits & ! (1u32 << window_bits_shift! ($name)))
                     | (($v as u32) << window_bits_shift! ($name));
    }};
}
/* window.h 502-560's packed-flag shifts, the members this unit names.  The mirror carries
   the shifts it needs; the `window.rs` shape names the rest here.  */
macro_rules! window_bits_shift {
    (mini) => { WINDOW_MINI_SHIFT };
    (pseudo_window_p) => { WINDOW_PSEUDO_WINDOW_P_SHIFT };
    (update_mode_line) => { WINDOW_UPDATE_MODE_LINE_SHIFT };
    (redisplay) => { WINDOW_REDISPLAY_SHIFT };
    (horizontal) => { WINDOW_HORIZONTAL_SHIFT };
    (last_had_star) => { WINDOW_LAST_HAD_STAR_SHIFT };
    (start_at_line_beg) => { WINDOW_START_AT_LINE_BEG_SHIFT };
    (force_start) => { WINDOW_FORCE_START_SHIFT };
    (optional_new_start) => { WINDOW_OPTIONAL_NEW_START_SHIFT };
    (phys_cursor_on_p) => { WINDOW_PHYS_CURSOR_ON_P_SHIFT };
    (cursor_off_p) => { WINDOW_CURSOR_OFF_P_SHIFT };
    (last_cursor_off_p) => { WINDOW_LAST_CURSOR_OFF_P_SHIFT };
    (must_be_updated_p) => { WINDOW_MUST_BE_UPDATED_P_SHIFT };
    (fringes_outside_margins) => { WINDOW_FRINGES_OUTSIDE_MARGINS_SHIFT };
    (scroll_bars_persistent) => { WINDOW_SCROLL_BARS_PERSISTENT_SHIFT };
    (window_end_valid) => { WINDOW_WINDOW_END_VALID_SHIFT };
    (suspend_auto_hscroll) => { WINDOW_SUSPEND_AUTO_HSCROLL_SHIFT };
    (preserve_vscroll_p) => { WINDOW_PRESERVE_VSCROLL_P_SHIFT };
}

/* Definitions in other units whose headers' mirrors do not carry the prototype
   (PORTING.md 0.1-2/P-72 rule 3); each keeps C's own prototype in a `/// C:` line.  These
   are *not* part of this unit's work (the `TEMPORARY SCAFFOLD` block at the end of the file
   is), so they stay.  */
unsafe extern "C" {
/* `buffer_window_count` is declared *and defined* by `include/buffer.rs` (buffer.h 1104-
   1108), which C's xdisp.c reaches through `#include "buffer.h"` (C 485); the mirror-dupes
   gate flags a second local declaration (the integrator's 2026-10-04 07:3x finding), so this
   unit uses the mirror's copy and declares nothing.  */
  /// window.h 1209's `window_parameter` is the window.rs mirror's now (P-72 rule 3: the mirror spells the pointee `*mut c_void`).
  /// C: dispextern.h 3556-3557: `extern void *bidi_shelve_cache (void);`
  /// `extern void bidi_unshelve_cache (void *, bool);`
  pub fn bidi_shelve_cache () -> *mut c_void;
  pub fn bidi_unshelve_cache (databuf: *mut c_void, really_free_p: bool);
  /// C: dispextern.h 3550: `extern void bidi_init_it (ptrdiff_t, ptrdiff_t, bool,
  /// struct bidi_it *);` (bidi.c defines it).
  pub fn bidi_init_it (charpos: ptrdiff_t, bytepos: ptrdiff_t, frame_window_p: bool,
		       bidi_it: *mut bidi_it);
  /// C: dispextern.h 3552: `extern void bidi_paragraph_init (bidi_dir_t, struct bidi_it *,
  /// bool);` (bidi.c defines it).
  pub fn bidi_paragraph_init (dir: bidi_dir_t, bidi_it: *mut bidi_it, no_default_p: bool);
  /// C: dispextern.h 3551: `extern void bidi_move_to_visually_next (struct bidi_it *);`
  /// (bidi.c defines it).
  pub fn bidi_move_to_visually_next (bidi_it: *mut bidi_it);
  /// C: dispextern.h 3559: `extern ptrdiff_t bidi_level_start (int);` (bidi.c defines
  /// it; `compute_stop_pos` needs it for the backward-scan stop position).
  pub fn bidi_level_start (level: c_int) -> ptrdiff_t;
  /* The face cache's entry points: `dispextern.h` declares them (3806-3833) and xfaces.c
     defines them; the mirror chain this unit includes does not carry them, so they are
     declared here with C's prototypes (PORTING.md 0.1-2/P-72 rule 3).  */
  /// C: dispextern.h 3813: `int lookup_basic_face (struct window *, struct frame *, int);`
  pub fn lookup_basic_face (w: *mut window, f: *mut frame, face_id: c_int) -> c_int;
  /// C: dispextern.h 3821: `void init_frame_faces (struct frame *);`
  pub fn init_frame_faces (f: *mut frame);
  /// C: dispextern.h 3823: `void recompute_basic_faces (struct frame *);`
  pub fn recompute_basic_faces (f: *mut frame);
  /// C: dispextern.h 3833: `void free_all_realized_faces (Lisp_Object);`
  pub fn free_all_realized_faces (frame: Lisp_Object);
  /// C: dispextern.h 2068: `extern bool face_change;` (xfaces.c defines it; the mirror
  /// chain does not carry the declaration).
  pub static mut face_change: bool;
  /// C: window.h 1223-1224: `extern int window_body_width (struct window *w, enum
  /// window_body_unit);` `extern int window_body_height (struct window *w, enum
  /// window_body_unit);` (window.c defines both).
  pub fn window_body_width (w: *mut window, unit: window_body_unit) -> c_int;
  pub fn window_body_height (w: *mut window, unit: window_body_unit) -> c_int;
  /// C: lisp.h 5033-5034: `extern ptrdiff_t find_newline1 (ptrdiff_t, ptrdiff_t,
  /// ptrdiff_t, ptrdiff_t, ptrdiff_t, ptrdiff_t *, ptrdiff_t *, bool);` (search.c defines
  /// it; the mirror chain carries only `find_newline`).
  pub fn find_newline1 (start: ptrdiff_t, start_byte: ptrdiff_t, end: ptrdiff_t,
			end_byte: ptrdiff_t, count: ptrdiff_t, shortage: *mut ptrdiff_t,
			shortage_byte: *mut ptrdiff_t, allow_quit: bool) -> ptrdiff_t;
  /// C: term.c 1636-1640: `void produce_glyphs (struct it *it);` (the `PRODUCE_GLYPHS`
  /// macro's fallback when `FRAME_RIF` is null; `init_tty` leaves `terminal->rif = 0`).
  pub fn produce_glyphs (it: *mut it);
  /* lisp.h 4808/4814-4815: eval.c's control-flow entry points.  C's xdisp.c reaches all
     three through lisp.h; the mirror chain does not carry them, so this unit declares
     them with C's prototypes (PORTING.md 0.1-2/P-72 rule 3; the `keyboard.rs`/`eval.rs`
     spellings — a handler is never NULL at these call sites, so the parameters are the
     bare function pointers C's prototype names).  */
  /// C: lisp.h 4808: `extern Lisp_Object internal_condition_case_n
  ///   (Lisp_Object (*) (ptrdiff_t, Lisp_Object *), ptrdiff_t, Lisp_Object *,
  ///    Lisp_Object, Lisp_Object (*) (Lisp_Object, ptrdiff_t, Lisp_Object *));`
  pub fn internal_condition_case_n (bfun: unsafe extern "C" fn (ptrdiff_t, *mut Lisp_Object)
                                                                   -> Lisp_Object,
                                    nargs: ptrdiff_t, args: *mut Lisp_Object,
                                    handlers: Lisp_Object,
                                    hfun: unsafe extern "C" fn (Lisp_Object, ptrdiff_t,
                                                                *mut Lisp_Object)
                                                                  -> Lisp_Object) -> Lisp_Object;
  /// C: lisp.h 4814: `extern void pop_handler (void);`
  pub fn pop_handler ();
  /// C: lisp.h 4815: `extern void push_handler_bind (Lisp_Object, Lisp_Object, int);`
  pub fn push_handler_bind (conditions: Lisp_Object, handler: Lisp_Object, var: c_int);
  /// C: <stdio.h>: `int vfprintf (FILE *, const char *, va_list);` (`stdio.rs` keeps only
  /// the non-variadic half, the pdumper.rs shape).
  pub fn vfprintf (stream: *mut FILE, format: *const c_char, args: VaList<'_>) -> c_int;
  /// C: lisp.h 4929: `extern void labeled_narrow_to_region (Lisp_Object, Lisp_Object,
  /// Lisp_Object);` (editfns.c defines it; the lisp.h mirror does not carry it).
  pub fn labeled_narrow_to_region (begv: Lisp_Object, zv: Lisp_Object, label: Lisp_Object);
  /// C: dispextern.h 3824-3825: `int face_at_buffer_position (struct window *, ptrdiff_t,
  /// ptrdiff_t *, ptrdiff_t, bool, int, enum lface_attribute_index);` (xfaces.c defines it).
  pub fn face_at_buffer_position (w: *mut window, pos: ptrdiff_t, endptr: *mut ptrdiff_t,
                                  limit: ptrdiff_t, mouse: bool, base_face_id: c_int,
                                  attr_filter: lface_attribute_index) -> c_int;
  /// C: dispextern.h 3826-3827: `int face_for_overlay_string (struct window *, ptrdiff_t,
  /// ptrdiff_t *, ptrdiff_t, bool, Lisp_Object, enum lface_attribute_index);`
  pub fn face_for_overlay_string (w: *mut window, pos: ptrdiff_t, endptr: *mut ptrdiff_t,
                                  limit: ptrdiff_t, mouse: bool, overlay: Lisp_Object,
                                  attr_filter: lface_attribute_index) -> c_int;
  /// C: dispextern.h 3828-3830: `int face_at_string_position (struct window *,
  /// Lisp_Object, ptrdiff_t, ptrdiff_t, ptrdiff_t *, enum face_id, bool,
  /// enum lface_attribute_index);`
  pub fn face_at_string_position (w: *mut window, string: Lisp_Object, bufpos: ptrdiff_t,
                                  bufpos_byte: ptrdiff_t, endptr: *mut ptrdiff_t,
                                  base_face_id: c_int, mouse_p: bool,
                                  attr_filter: lface_attribute_index) -> c_int;
  /// C: dispextern.h 3814: `int smaller_face (struct frame *, int, int);` (xfaces.c
  /// defines it; the mirror chain does not carry it).
  pub fn smaller_face (f: *mut frame, face_id: c_int, steps: c_int) -> c_int;
  /// C: dispextern.h 3815: `int face_with_height (struct frame *, int, int);` (xfaces.c
  /// defines it).
  pub fn face_with_height (f: *mut frame, face_id: c_int, height: c_int) -> c_int;
  /// C: lisp.h 4799: `extern bool FUNCTIONP (Lisp_Object);` (bytecode.c defines it; the
  /// mirror chain does not carry it — the `keyboard.rs`/`xfaces.rs` spelling).
  pub fn FUNCTIONP (o: Lisp_Object) -> bool;
  /// C: dispextern.h 3553: `extern int bidi_mirror_char (int);` (bidi.c defines it; the
  /// mirror chain does not carry it).
  pub fn bidi_mirror_char (c: c_int) -> c_int;
  /// C: dispextern.h 3554: `extern void bidi_push_it (struct bidi_it *);` (bidi.c defines
  /// it; the mirror chain does not carry it).
  pub fn bidi_push_it (bidi_it: *mut bidi_it);
  /// C: dispextern.h 3555: `extern void bidi_pop_it (struct bidi_it *);` (bidi.c defines it).
  pub fn bidi_pop_it (bidi_it: *mut bidi_it);
  /// C: lisp.h 5040-5041: `extern ptrdiff_t find_newline_no_quit (ptrdiff_t, ptrdiff_t,
  /// ptrdiff_t, ptrdiff_t *);` (search.c defines it; the mirror chain does not carry it —
  /// the `bidi.rs`/`indent.rs` spelling).
  pub fn find_newline_no_quit (from: ptrdiff_t, from_byte: ptrdiff_t, cnt: ptrdiff_t,
			       bytepos: *mut ptrdiff_t) -> ptrdiff_t;
  /// C: lisp.h 5113: `extern bool indented_beyond_p (ptrdiff_t, ptrdiff_t, EMACS_INT);`
  /// (indent.c defines it; `indent.rs` is not in this unit's include list).
  pub fn indented_beyond_p (pos: ptrdiff_t, pos_byte: ptrdiff_t, column: EMACS_INT) -> bool;
  /// C: dispextern.h 3831: `int merge_faces (struct window *, Lisp_Object, int, int);`
  /// (xfaces.c defines it; the `dispnew.rs` spelling).
  pub fn merge_faces (w: *mut window, face_name: Lisp_Object, face_id: c_int,
		      base_face_id: c_int) -> c_int;
  /// window.h 1233's `window_point` is the window.rs mirror's now (P-72 rule 3: the mirror spells the pointee `*mut c_void`).
  /// window.h 1210's `decode_live_window` is the window.rs mirror's now (P-72 rule 3: the mirror spells the pointee `*mut c_void`).
  /// C: <stdlib.h> 489: `extern void qsort (void *__base, size_t __nmemb, size_t __size,
  /// __compar_fn_t __compar) __nonnull ((1, 4));` (the port's stdlib.rs mirror does not
  /// carry it; declared here, the `buffer.rs`/`font.rs` shape).
  pub fn qsort (base: *mut c_void, nmemb: usize, size: usize,
		compar: Option<unsafe extern "C" fn (*const c_void, *const c_void) -> c_int>);
  /// window.h 1231's `wset_buffer` is the window.rs mirror's now (P-72 rule 3: the mirror spells the pointee `*mut c_void`).
  /// C: lisp.h 4331-4332: `extern void insert_1_both (const char *, ptrdiff_t, ptrdiff_t,
  /// bool, bool, bool);` (insdel.c defines it).
  pub fn insert_1_both (s: *const c_char, nchars: ptrdiff_t, nbytes: ptrdiff_t,
			prepare: bool, inherit: bool, before_markers: bool);
  /// C: lisp.h 4348: `extern void del_range_both (ptrdiff_t, ptrdiff_t, ptrdiff_t,
  /// ptrdiff_t, bool);` (insdel.c defines it).
  pub fn del_range_both (from: ptrdiff_t, from_byte: ptrdiff_t, to: ptrdiff_t,
			 to_byte: ptrdiff_t, prepare: bool);
  /// C: lisp.h 4226: `extern ptrdiff_t chars_in_text (const unsigned char *, ptrdiff_t);`
  /// (character.c defines it; `include/character.rs`'s `chars_in_text` takes `*const u8`).
  pub fn chars_in_text (ptr: *const u8, nbytes: ptrdiff_t) -> ptrdiff_t;
  /// C: lisp.h 5082: `extern void cancel_echoing (void);` (keyboard.c defines it).
  pub fn cancel_echoing ();
  /// C: dispextern.h 3923: `void do_pending_window_change (bool);` (window.c defines it).
  pub fn do_pending_window_change (safe: bool);
  /// C: dispextern.h 3906: `void clear_glyph_matrix (struct glyph_matrix *);` (dispnew.c
  /// defines it).
  pub fn clear_glyph_matrix (matrix: *mut glyph_matrix);
  /// C: sysstdio.h 32: `extern void errputc (int);` (`include/sysstdio.rs` mirrors the header,
  /// but this unit includes `lib/include/stdio.rs` for C 477, so the two are declared here).
  pub fn errputc (c: c_int);
  /// C: sysstdio.h 33: `extern void errwrite (void const *, ptrdiff_t);`
  pub fn errwrite (buf: *const c_void, nbuf: ptrdiff_t);
  /// C: <inttypes.h>: `extern intmax_t strtoimax (const char *, char **, int);` (libc; gnulib's
  /// `inttypes.in.h` only renames it, the `qsort`/`strchr` shape).
  pub fn strtoimax (s: *const c_char, endptr: *mut *mut c_char, base: c_int) -> intmax_t;
  /// C: lisp.h 4715-4716: `extern ptrdiff_t doprnt (char *, ptrdiff_t, const char *,
  /// const char *, va_list);` (doprnt.c defines it; the mirror chain does not carry it).
  pub fn doprnt (buffer: *mut c_char, size: ptrdiff_t, format: *const c_char,
		 format_end: *const c_char, ap: VaList<'_>) -> ptrdiff_t;
  /// C: frame.h 1480: `extern bool frame_redisplay_p (struct frame *);` (frame.c defines it).
  pub fn frame_redisplay_p (f: *mut frame) -> bool;
  /// C: dispextern.h 4012: `bool is_tty_frame (struct frame *f);` (dispnew.c defines it).
  pub fn is_tty_frame (f: *mut frame) -> bool;
  /// window.h 1232's `window_outdated` is the window.rs mirror's now (P-72 rule 3: the mirror spells the pointee `*mut c_void`).
  /// C: lisp.h 4843: `extern Lisp_Object call_debugger (Lisp_Object arg);` (eval.c defines it).
  pub fn call_debugger (arg: Lisp_Object) -> Lisp_Object;
  /// C: dispextern.h 3907: `void clear_current_matrices (struct frame *f);` (dispnew.c).
  pub fn clear_current_matrices (f: *mut frame);
  /// C: dispextern.h 3896: `extern void redraw_frame (struct frame *);` (dispnew.c).
  pub fn redraw_frame (f: *mut frame);
  /// C: dispextern.h 3918: `void update_single_window (struct window *);` (dispnew.c).
  pub fn update_single_window (w: *mut window);
  /// C: dispextern.h 4017: `void combine_updates_for_frame (struct frame *f, bool
  /// inhibit_id_p);` (dispnew.c).
  /// window.h 1130's `grow_mini_window` is the window.rs mirror's now (P-72 rule 3: the mirror spells the pointee `*mut c_void`).
  pub fn combine_updates_for_frame (f: *mut frame, inhibit_id_p: bool);
  /// window.h 1131's `shrink_mini_window` is the window.rs mirror's now (P-72 rule 3: the mirror spells the pointee `*mut c_void`).
  /// C: dispextern.h 3897: `void update_frame (struct frame *, bool);` (dispnew.c).
  pub fn update_frame (f: *mut frame, force_p: bool);
  /// C: lisp.h 4335-4336: `extern void insert_from_string (Lisp_Object, ptrdiff_t, ptrdiff_t,
  /// ptrdiff_t, ptrdiff_t, bool);` (insdel.c defines it).
  pub fn insert_from_string (string: Lisp_Object, from: ptrdiff_t, from_byte: ptrdiff_t,
			     nchars: ptrdiff_t, nbytes: ptrdiff_t, inherit: bool);
  /// C: dispextern.h 3914: `void clear_glyph_row (struct glyph_row *);` (dispnew.c).
  pub fn clear_glyph_row (row: *mut glyph_row);
  /// C: dispextern.h 3908: `void clear_desired_matrices (struct frame *);` (dispnew.c).
  pub fn clear_desired_matrices (f: *mut frame);
  /// C: search.c's `void record_unwind_save_match_data (void);` (lisp.h 4852).
  pub fn record_unwind_save_match_data ();
  /// C: frame.h 1385: `struct frame *root_frame (struct frame *);` (frame.c defines it).
  pub fn root_frame (f: *mut frame) -> *mut frame;
  /// C: lisp.h 4326: `extern void safe_run_hooks (Lisp_Object hook);` (keyboard.c defines it).
  pub fn safe_run_hooks (hook: Lisp_Object);
  /// C: lisp.h 5098: `extern void safe_run_hooks_2 (Lisp_Object, Lisp_Object, Lisp_Object);`
  /// (keyboard.c defines it).
  pub fn safe_run_hooks_2 (hook: Lisp_Object, arg1: Lisp_Object, arg2: Lisp_Object);
  /// C: window.h 1226: `extern int window_scroll_margin (struct window *, enum margin_unit);`
  /// (window.c defines it; the mirror chain does not carry it).
  pub fn window_scroll_margin (window: *mut window, unit: margin_unit) -> c_int;
  /// window.h 1208's `window_list` is the window.rs mirror's now (P-72 rule 3: the mirror spells the pointee `*mut c_void`).
  /// C: lisp.h 4850: `extern Lisp_Object menu_bar_items (Lisp_Object);` (keyboard.c defines it).
  pub fn menu_bar_items (menu_bar_items: Lisp_Object) -> Lisp_Object;
  /// C: lisp.h 4851: `extern Lisp_Object tab_bar_items (Lisp_Object, int *);` (keyboard.c).
  pub fn tab_bar_items (tab_bar_items: Lisp_Object, ntab_bar_items: *mut c_int) -> Lisp_Object;
  /// C: lisp.h 4944: `extern bool overlay_touches_p (ptrdiff_t);` (buffer.c defines it).
  pub fn overlay_touches_p (pos: ptrdiff_t) -> bool;
  /// C: keyboard.h 485-486: `extern void start_polling (void);` / `stop_polling (void);`
  /// (keyboard.c defines both).
  pub fn start_polling ();
  pub fn stop_polling ();
  /// C: lisp.h 4825: `extern void record_unwind_protect_void (void (*) (void));` (alloc.c).
  pub fn record_unwind_protect_void (function: Option<unsafe extern "C" fn ()>);
  /// C: lisp.h 4930: `extern void reset_outermost_restrictions (void);` (editfns.c).
  pub fn reset_outermost_restrictions ();
  /// C: window.c's `void run_window_change_functions (void);` (window.h 484).
  pub fn run_window_change_functions ();
  /// C: dispextern.h 4015: `bool is_tty_root_frame_with_visible_child (struct frame *f);`
  /// (dispnew.c).
  pub fn is_tty_root_frame_with_visible_child (f: *mut frame) -> bool;
  /// C: dispextern.h 3912: `void increment_matrix_positions (struct glyph_matrix *, int, int,
  /// ptrdiff_t, ptrdiff_t);` (dispnew.c).
  pub fn increment_matrix_positions (matrix: *mut glyph_matrix, start: c_int, end: c_int,
                                     delta: ptrdiff_t, delta_bytes: ptrdiff_t);
  /// C: lisp.h 4806: `extern Lisp_Object internal_condition_case_1 (Lisp_Object (*)
  /// (Lisp_Object), Lisp_Object, Lisp_Object, Lisp_Object (*) (Lisp_Object));` (eval.c).
  pub fn internal_condition_case_1 (bfun: Option<unsafe extern "C" fn (Lisp_Object) -> Lisp_Object>,
                                    arg: Lisp_Object, handlers: Lisp_Object,
                                    hfun: Option<unsafe extern "C" fn (Lisp_Object) -> Lisp_Object>)
                                    -> Lisp_Object;
  /// C: dispextern.h 4016: `void combine_updates (Lisp_Object root_frames);` (dispnew.c).
  pub fn combine_updates (root_frames: Lisp_Object);
  /// C: lisp.h 4286: `Lisp_Object memq_no_quit (Lisp_Object, Lisp_Object);` (fns.c).
  pub fn memq_no_quit (elt: Lisp_Object, list: Lisp_Object) -> Lisp_Object;
  /// C: dispextern.h 3958: `extern void set_tty_color_mode (struct tty_display_info *,
  /// struct frame *);` (term.c).
  pub fn set_tty_color_mode (tty: *mut tty_display_info, f: *mut frame);
  /// C: dispextern.h 3802: `void clear_face_cache (bool);` (xfaces.c defines it).
  pub fn clear_face_cache (force_p: bool);
  /// C: dispextern.h 3901: `extern void adjust_frame_glyphs (struct frame *);` (dispnew.c).
  pub fn adjust_frame_glyphs (f: *mut frame);
  /// C: dispextern.h 3781-3782: `void request_sigio (void);` / `unrequest_sigio (void);`
  /// (sysdep.c).
  pub fn request_sigio ();
  pub fn unrequest_sigio ();
}

/* C 348-351: `termopts.h`'s two objects (that header has a mirror, but no unit `include!`s
   it yet, so every unit that reads them declares them at its own use site — the
   keyboard.rs/sysdep.rs shape).  */
unsafe extern "C" {
  pub static mut interrupt_input: bool;
  pub static mut interrupts_deferred: bool;
}
/* `set_buffer_internal_1` is C's prototype at buffer.h 1261, which C's xdisp.c
   reaches through its own `#include "buffer.h"` (C 485); `include/buffer.rs` already
   declares it, so this unit must not declare it a second time (P-26/the mirror-dupes gate
   — the integrator's 2026-10-04 07:3x finding).  The call site keeps C's spelling.  */

/* C: dispextern.h 1770-1937 `struct face`, `frame.h 1379`'s `FRAME_FACE_CACHE` and
   dispextern.h's `FACE_FROM_ID_OR_NULL`: the record and the two accessors `init_iterator`
   reaches (`face->box`, `face->box_vertical_line_width`).  `dispextern.h` is mirrored only
   to its batch-II prefix (P-66), so the record is this unit's own measured copy with only
   the reached members named (the `window.rs`/`font.rs` shape).  clang against the frozen
   target: sizeof 280, align 8, `lface`@0, `box_vertical_line_width`@228, the packed face
   bits (C 1893-1935's `ENUM_BF (face_box_type) box : 2` at bit 0) in the 4-byte unit at
   240.  `HAVE_WINDOW_SYSTEM`'s `gc`/`stipple`, `HAVE_NS`'s and `HAVE_XFT`'s members are
   undef on this target, so those are absent from the layout.  */
#[repr(C)]
pub struct face
{
  pub lface: [Lisp_Object; LFACE_VECTOR_SIZE as usize],
  /// The id of this face.  (C 1786-1788)
  pub id: c_int,                        // offset 160
  /// C 1796-1826: the alignment hole after `id`, then `foreground`, `background`,
  /// `underline_color`, `overline_color`, `strike_through_color` and `box_color`
  /// (offset 164 … 216).  clang measures `id`@160 and `font`@216.
  _pad0: [u8; 52],
  /// The font used for this face.  (C 1828; `include/font.rs`'s `struct font`, clang: 216)
  pub font: *mut font,                  // offset 216
  /// C 1830-1832: `fontset` (offset 224 … 228).
  _pad0b: [u8; 4],
  pub box_vertical_line_width: c_int,   // offset 228
  /// C 1891-1892: `box_horizontal_line_width`, `underline_pixels_above_descent_line`
  /// (offset 232 … 240).
  _pad1: [u8; 8],
  pub bits: u32,                        // offset 240
  /// C 1936-1937: `hash`, `next`, `prev`, `ascii_face` (offset 244 … 280).
  _pad2: [u8; 36],
}
const _: () = assert! (core::mem::size_of::<face> () == 280);
const _: () = assert! (core::mem::align_of::<face> () == 8);
const _: () = assert! (core::mem::offset_of! (face, id) == 160);
const _: () = assert! (core::mem::offset_of! (face, font) == 216);
const _: () = assert! (core::mem::offset_of! (face, box_vertical_line_width) == 228);
const _: () = assert! (core::mem::offset_of! (face, bits) == 240);

pub const FACE_BOX_SHIFT: u32 = 0;
pub const FACE_BOX_MASK: u32 = 0x3;
/// C: dispextern.h 1893-1935's `ENUM_BF (face_box_type) box : 2` — the P-80 accessor
/// (`font.rs`/`xfaces.rs` carry the same pair for their own copies of the record).
macro_rules! face_bits {
  (box, $f:expr) => { ((*$f).bits & FACE_BOX_MASK) >> FACE_BOX_SHIFT };
}

/// C: frame.h 1379: `#define FRAME_FACE_CACHE(F) (F)->face_cache`.  The `frame` mirror
/// carries the field as `*mut c_void` (the record has no mirror), so the cast is the
/// use-site conversion P-72 rule 3.
macro_rules! FRAME_FACE_CACHE {
  ($f:expr) => { (*$f).face_cache as *mut face_cache };
}

/* C: dispextern.h 1991-2005 `struct face_cache`; only `used` and `faces_by_id` are read
   here.  clang: sizeof 40, `faces_by_id`@16, `used`@32.  */
#[repr(C)]
pub struct face_cache
{
  pub buckets: *mut *mut face,
  pub f: *mut frame,
  pub faces_by_id: *mut *mut face,
  pub size: ptrdiff_t,
  pub used: c_int,
  pub menu_face_changed_p: bool,
}
const _: () = assert! (core::mem::size_of::<face_cache> () == 40);
const _: () = assert! (core::mem::offset_of! (face_cache, faces_by_id) == 16);
const _: () = assert! (core::mem::offset_of! (face_cache, used) == 32);

/// C: frame.h 1643-1649: `INLINE struct face *FACE_FROM_ID (struct frame *f, int id);`
/// (the `eassert` form; `font.rs`/`term.rs` carry the same copy of the record's accessor).
#[inline(always)]
unsafe fn FACE_FROM_ID (f: *mut frame, id: c_int) -> *mut face
{
  eassert! (0 <= id && id < (*FRAME_FACE_CACHE! (f)).used);
  *((*FRAME_FACE_CACHE! (f)).faces_by_id).offset (id as isize)
}

/// C: dispextern.h 2030-2038: `INLINE int FACE_FOR_CHAR (struct frame *f, struct face *face,
/// int character, ptrdiff_t pos, Lisp_Object object);`.  `HAVE_WINDOW_SYSTEM` is undef on
/// this target, so C's body is `return face->id;` (the `xfaces.rs` copy of the macro).
#[inline(always)]
unsafe fn FACE_FOR_CHAR (f: *mut frame, face: *mut face, character: c_int, pos: ptrdiff_t,
			 object: Lisp_Object) -> face_id
{
  let _ = (f, character, pos, object);
  (*face).id
}

/// C: frame.h 1656: `INLINE struct face *FACE_FROM_ID_OR_NULL (struct frame *f, int id);`
/// `include/dispextern.rs` carries `FACE_FROM_ID`'s `eassert` form, not the NULL-returning
/// sibling, so the port spells it here (the `window.rs`/`term.rs` copy).
unsafe fn FACE_FROM_ID_OR_NULL (f: *mut frame, id: c_int) -> *mut face
{
  let used: c_int = (*FRAME_FACE_CACHE! (f)).used;
  eassume! (0 <= used);
  return if 0 <= id && id < used
    { *((*FRAME_FACE_CACHE! (f)).faces_by_id).offset (id as isize) }
  else { core::ptr::null_mut () };
}

/* `xwidget.h` has no mirror and C's xdisp.c reaches it through `#include "xwidget.h"`
   (C 499); `HAVE_XWIDGETS` is undef on this target, so C compiles the header's `#else`
   arm, whose two entry points are `INLINE`s with constant bodies.  This unit names them at
   their use sites (PORTING.md P-72 rule 3, the `FACE_FOR_CHAR` shape).  */

/// C: xwidget.h 234: `INLINE bool valid_xwidget_spec_p (Lisp_Object obj) { return false; }`.
#[inline(always)]
unsafe fn valid_xwidget_spec_p (obj: Lisp_Object) -> bool
{
  let _ = obj;
  false
}

/// C: xwidget.h 237: `INLINE struct xwidget *lookup_xwidget (Lisp_Object obj) { eassume (0); }`.
/// C's `struct xwidget` is incomplete on this target, so the port's pointer is `*mut c_void`.
#[inline(always)]
unsafe fn lookup_xwidget (obj: Lisp_Object) -> *mut c_void
{
  let _ = obj;
  eassume! (false);
  /* C's function has no `return` statement: `eassume (0)` says the tail is unreachable, so
     this aborts instead of producing a value C never produces (the `include/frame.rs` 240
     shape; a bare `unreachable` panic would also read as a stub to `tools/extract-abi.py`).  */
  emacs_abort ()
}

/* dispextern.h 2285-2315 `enum display_element_type`, 2384's `OVERLAY_STRING_CHUNK_SIZE`
   and 2590's `IT_STACK_SIZE`.  `dispextern.h` is mirrored only to its batch-II prefix
   (P-66), so these are declared here the way `indent.rs`/`window.rs` declare their copies.
   PORTING.md 1.3: C's enum ABI is `c_int`, so the port keeps C's numbers in constants.  */
pub type display_element_type = c_int;
pub const IT_CHARACTER: display_element_type = 0;
pub const IT_COMPOSITION: display_element_type = 1;
pub const IT_GLYPHLESS: display_element_type = 2;
pub const IT_IMAGE: display_element_type = 3;
pub const IT_STRETCH: display_element_type = 4;
pub const IT_EOB: display_element_type = 5;
pub const IT_TRUNCATION: display_element_type = 6;
pub const IT_CONTINUATION: display_element_type = 7;
pub const IT_XWIDGET: display_element_type = 8;

pub const OVERLAY_STRING_CHUNK_SIZE: c_int = 16;
pub const IT_STACK_SIZE: c_int = 5;

/* C: dispextern.h 2371-2384 `enum it_method` and 2338-2344 `enum line_wrap_method`.  */
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

/* C: dispextern.h 2977-2990 `enum move_operation_enum`.  */
pub type move_operation_enum = c_int;
pub const MOVE_TO_X: move_operation_enum = 0x01;
pub const MOVE_TO_Y: move_operation_enum = 0x02;
pub const MOVE_TO_VPOS: move_operation_enum = 0x04;
pub const MOVE_TO_POS: move_operation_enum = 0x08;

/* C: dispextern.h 2347-2358 `enum glyphless_display_method`: the mirror chain does not
   carry it, so the unit spells it at its use site (the `term.rs` copy).  */
pub type glyphless_display_method = c_int;
pub const GLYPHLESS_DISPLAY_THIN_SPACE: glyphless_display_method = 0;
pub const GLYPHLESS_DISPLAY_EMPTY_BOX: glyphless_display_method = 1;
pub const GLYPHLESS_DISPLAY_ACRONYM: glyphless_display_method = 2;
pub const GLYPHLESS_DISPLAY_HEX_CODE: glyphless_display_method = 3;

/* C: dispextern.h 2159-2222 `struct bidi_it`: this unit reaches `scan_dir` (via the
   iterator's own field), `disp_pos`/`disp_prop` (the display-string search short-cut in
   `forward_to_next_line_start` and `reseat_to_string`) and `paragraph_dir` (C's
   `it->bidi_it.paragraph_dir` in `PRODUCE_GLYPHS` and in the bidi code).  clang against
   the frozen target: sizeof 2328, `scan_dir` 184, `disp_pos` 192, `disp_prop` 200,
   `paragraph_dir` 2304.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct bidi_it
{
  /// Iterator's position in buffer/string.  (C 2161-2162)
  pub bytepos: ptrdiff_t,               // offset 0
  pub charpos: ptrdiff_t,               // offset 8
  /// Character at that position, or U+FFFC.  (C 2163-2164)
  pub ch: c_int,                        // offset 16
  /// C 2165: the alignment hole before `nchars` (offset 20 … 24).
  pub _pad_a0: [u8; 4],
  /// Its "length", usually 1.  (C 2166-2167)
  pub nchars: ptrdiff_t,                // offset 24
  /// Its length in bytes.  (C 2168-2169)
  pub ch_len: ptrdiff_t,                // offset 32
  /// Final bidi type of this character, after resolving weak and neutral types.  (C 2168-2169)
  /// C's member name is a Rust keyword, so the port uses the raw identifier (the `alloc.rs`
  /// `mem_node.r#type` shape).
  pub r#type: bidi_type_t,              // offset 40
  /// C 2170-2171: `type_after_wn` and `orig_type` (offset 44 … 52).
  pub _pad_a: [u8; 8],
  /// Final resolved level of this character.  (C 2176)
  pub resolved_level: c_char,          // offset 52
  /// C 2177-2185: `isolate_level` … `next_en_type` (offset 53 … 184).
  pub _pad_a2: [u8; 131],
  /// Direction of text scan, 1: forw, -1: back.  (C 2186)
  pub scan_dir: c_int,                  // offset 184
  /// C 2188: the alignment hole before `disp_pos` (offset 188 … 192).
  pub _pad_b0: [u8; 4],
  /// Position of display string after ch.  (C 2187)
  pub disp_pos: ptrdiff_t,              // offset 192
  /// If non-zero, there really is a `display' property/string at `disp_pos`; if 2, the
  /// property is a `space' spec.  (C 2188-2190)
  pub disp_prop: c_int,                 // offset 200
  /// C 2191-2206: `stack_idx` and the start of `level_stack` (offset 204 … 218).
  pub _pad_b0b: [u8; 14],
  /// The `level` member of `level_stack[0]`, which `BIDI_AT_BASE_LEVEL` reads (clang puts
  /// it at 218; the whole stack is 128 entries of 16 bytes, `bidi.rs`'s copy).
  pub level_stack0_level: u8,           // offset 218
  /// C 2211-2294: the remaining 2037 bytes of `level_stack` (offset 219 … 2256).
  pub _pad_b1: [u8; 2037],
  /// String to reorder.  (C 2196-2197)
  pub string: bidi_string_data,         // offset 2256
  /// The window whose buffer we are reordering.  (C 2296-2298)
  pub w: *mut window,                   // offset 2296
  pub paragraph_dir: bidi_dir_t,        // offset 2304
  /// C 2306-2319: `separator_limit` (offset 2312; clang aligns it past the four-byte hole
  /// that follows `paragraph_dir`).
  pub _pad_tail_a: [u8; 12],            // 2308 … 2320
  /// C 2320-2322: the three `bool_bf : 1` members `first_elt`, `new_paragraph`,
  /// `frame_window_p` (offset 2320; the measured byte is `01`/`02`/`04`, as `bidi.rs` has).
  pub bits: u8,                         // offset 2320
  /// C 2323-2327: the tail (offset 2321 … 2328).
  pub _pad_tail_b: [u8; 7],
}

const _: () = assert! (core::mem::size_of::<bidi_it> () == 2328);
const _: () = assert! (core::mem::offset_of! (bidi_it, bytepos) == 0);
const _: () = assert! (core::mem::offset_of! (bidi_it, charpos) == 8);
const _: () = assert! (core::mem::offset_of! (bidi_it, ch) == 16);
const _: () = assert! (core::mem::offset_of! (bidi_it, nchars) == 24);
const _: () = assert! (core::mem::offset_of! (bidi_it, ch_len) == 32);
const _: () = assert! (core::mem::offset_of! (bidi_it, r#type) == 40);
const _: () = assert! (core::mem::offset_of! (bidi_it, resolved_level) == 52);
const _: () = assert! (core::mem::offset_of! (bidi_it, scan_dir) == 184);
const _: () = assert! (core::mem::offset_of! (bidi_it, disp_pos) == 192);
const _: () = assert! (core::mem::offset_of! (bidi_it, disp_prop) == 200);
const _: () = assert! (core::mem::offset_of! (bidi_it, level_stack0_level) == 218);
const _: () = assert! (core::mem::offset_of! (bidi_it, string) == 2256);
const _: () = assert! (core::mem::offset_of! (bidi_it, w) == 2296);
const _: () = assert! (core::mem::offset_of! (bidi_it, paragraph_dir) == 2304);
const _: () = assert! (core::mem::offset_of! (bidi_it, bits) == 2320);

/* C: dispextern.h 2159 `struct bidi_it`; its three `bool_bf : 1` members (2200-2202) share
   the byte at 2320 (measured `01` for `first_elt`), and C 2207-2208's `BIDI_AT_BASE_LEVEL`
   compares
   `resolved_level` with `level_stack[0].level` (the measured byte at 218).  `bidi.rs` names
   the whole record; this unit reaches only `first_elt` and that one `level`.  */
pub const BIDI_IT_FIRST_ELT: u8 = 1 << 0;
pub const BIDI_IT_NEW_PARAGRAPH: u8 = 1 << 1;
pub const BIDI_IT_FRAME_WINDOW_P: u8 = 1 << 2;

macro_rules! bidi_it_bits {
  (first_elt, $b:expr) => { ((*$b).bits & BIDI_IT_FIRST_ELT != 0) };
  (new_paragraph, $b:expr) => { ((*$b).bits & BIDI_IT_NEW_PARAGRAPH != 0) };
  (first_elt, $b:expr, $value:expr) => {
    (*$b).bits = ((*$b).bits & ! BIDI_IT_FIRST_ELT)
		 | (if $value { BIDI_IT_FIRST_ELT } else { 0 }) };
}

/* C: dispextern.h 2207-2208: `#define BIDI_AT_BASE_LEVEL(BIDI_IT)
   ((BIDI_IT).resolved_level == (BIDI_IT).level_stack[0].level)`.  */
macro_rules! BIDI_AT_BASE_LEVEL {
  ($bidi_it:expr) => { ((*$bidi_it).resolved_level as c_int
			== (*$bidi_it).level_stack0_level as c_int) };
}

/* C: dispextern.h 2147-2155 `struct bidi_string_data`'s two `bool_bf : 1` members; the
   mirror packs them into `bits: u32` (measured: `01` for `from_disp_str`, `02` for
   `unibyte`).  */
pub const BIDI_STRING_FROM_DISP_STR: u32 = 1 << 0;
pub const BIDI_STRING_UNIBYTE: u32 = 1 << 1;

macro_rules! bidi_string_bits {
  (from_disp_str, $s:expr) => { ((*$s).bits & BIDI_STRING_FROM_DISP_STR != 0) };
  (from_disp_str, $s:expr, $value:expr) =>
    { (*$s).bits = if ($value) as c_int != 0 { (*$s).bits | BIDI_STRING_FROM_DISP_STR }
		     else { (*$s).bits & !BIDI_STRING_FROM_DISP_STR }; };
  (unibyte, $s:expr) => { ((*$s).bits & BIDI_STRING_UNIBYTE != 0) };
  (unibyte, $s:expr, $value:expr) =>
    { (*$s).bits = if ($value) as c_int != 0 { (*$s).bits | BIDI_STRING_UNIBYTE }
		     else { (*$s).bits & !BIDI_STRING_UNIBYTE }; };
}

/* C: dispextern.h 2592-2643 `struct iterator_stack_entry`: the `stack` member of `struct it`
   is an array of these, so its size (304) is what fixes every offset past it.  The members
   this unit's `push_it`/`pop_it` reach are named and every other member is inside a measured
   pad (the `indent.rs` shape).  clang against the frozen target: sizeof 304, `string` 0,
   `string_nchars` 8, `end_charpos` 16, `stop_charpos` 24, `prev_stop` 32, `base_level_stop`
   40, `cmp_it` 48, `face_id` 136, `u` 144, `position` 192, `current` 208, `from_overlay` 256,
   `area` 264, `method` 268, `paragraph_embedding` 272, `bits` 276, `line_wrap` 280,
   `voffset` 284, `space_width` 288, `font_height` 296.  */

/* C: dispextern.h 2604-2619's anonymous `u` union (P-67: the port names an anonymous
   record after its member path).  clang: `u` 144 (sizeof 48, align 8), `u.image.object` 144,
   `u.image.slice` 152, `u.image.image_id` 184, `u.stretch.object` 144, `u.xwidget.object` 144.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct iterator_stack_entry_u_image
{
  pub object: Lisp_Object,              // 2607
  /// Values from `slice' property.
  pub slice: it_slice,                  // 2608
  pub image_id: ptrdiff_t,              // 2609
}
const _: () = assert! (core::mem::size_of::<iterator_stack_entry_u_image> () == 48);
const _: () = assert! (core::mem::offset_of! (iterator_stack_entry_u_image, object) == 0);
const _: () = assert! (core::mem::offset_of! (iterator_stack_entry_u_image, slice) == 8);
const _: () = assert! (core::mem::offset_of! (iterator_stack_entry_u_image, image_id) == 40);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct iterator_stack_entry_u_stretch
{
  pub object: Lisp_Object,              // 2613
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct iterator_stack_entry_u_xwidget
{
  pub object: Lisp_Object,              // 2617
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union iterator_stack_entry_u
{
  pub image: iterator_stack_entry_u_image,      // 2610
  pub stretch: iterator_stack_entry_u_stretch,  // 2614
  pub xwidget: iterator_stack_entry_u_xwidget,  // 2618
}
const _: () = assert! (core::mem::size_of::<iterator_stack_entry_u> () == 48);
const _: () = assert! (core::mem::align_of::<iterator_stack_entry_u> () == 8);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct iterator_stack_entry
{
  /// The string being processed, if any.  (C 2594)
  pub string: Lisp_Object,              // offset 0
  /// How many characters in the string.  (C 2595)
  pub string_nchars: c_int,             // offset 8
  /// C 2596: the alignment hole before `end_charpos` (offset 12 … 16).
  pub _pad_a0: [u8; 4],
  /// End of the buffer or string in which the iteration is taking place.  (C 2596)
  pub end_charpos: ptrdiff_t,           // offset 16
  /// The next position at which to check for invisible text etc.  (C 2597)
  pub stop_charpos: ptrdiff_t,          // offset 24
  /// Previous stop position, i.e. the last one before the current iterator position in
  /// `current'.  (C 2598)
  pub prev_stop: ptrdiff_t,             // offset 32
  /// Last stop position iterated across whose bidi embedding level is equal to the
  /// current paragraph's base embedding level.  (C 2599)
  pub base_level_stop: ptrdiff_t,       // offset 40
  /// If `what' == IT_COMPOSITION, iterator substructure for the composition.  (C 2600)
  pub cmp_it: composition_it,           // offset 48
  /// The face to use.  (C 2601)
  pub face_id: c_int,                   // offset 136
  /// C 2602: the alignment hole before `u` (offset 140 … 144).
  pub _pad_a1: [u8; 4],
  /// Save values specific to a given method.  (C 2603-2604)
  pub u: iterator_stack_entry_u,        // offset 144
  /// Current text and display positions.  (C 2622)
  pub position: text_pos,               // offset 192
  pub current: display_pos,             // 2623, offset 208
  pub from_overlay: Lisp_Object,        // 2624, offset 256
  pub area: glyph_row_area,             // 2625, offset 264
  pub method: it_method,                // 2626, offset 268
  pub paragraph_embedding: bidi_dir_t,  // 2627, offset 272
  /// C 2628-2634: the seven `bool_bf : 1` members `multibyte_p`,
  /// `string_from_display_prop_p`, `string_from_prefix_prop_p`, `display_ellipsis_p`,
  /// `avoid_cursor_p`, `bidi_p`, `from_disp_prop_p` (offset 276; measured `01` for
  /// `multibyte_p`, `08` for `display_ellipsis_p`, `20` for `bidi_p`).
  pub bits: u8,                         // offset 276
  /// C 2635: the padding before `line_wrap` (offset 277 … 280).
  pub _pad_c0: [u8; 3],
  /// How to wrap or truncate long lines.  (C 2635)
  pub line_wrap: line_wrap_method,      // offset 280
  /// Computed from the value of the `raise' property.  (C 2639)
  pub voffset: c_short,                 // offset 284
  /// C 2640: the alignment hole before `space_width` (offset 286 … 288).
  pub _pad_c1: [u8; 2],
  /// Value of the `space-width' property, if any; nil if none.  (C 2640)
  pub space_width: Lisp_Object,         // offset 288
  /// Value of the `height' property, if any; nil if none.  (C 2641)
  pub font_height: Lisp_Object,         // offset 296
}

const _: () = assert! (core::mem::size_of::<iterator_stack_entry> () == 304);
const _: () = assert! (core::mem::offset_of! (iterator_stack_entry, string) == 0);
const _: () = assert! (core::mem::offset_of! (iterator_stack_entry, string_nchars) == 8);
const _: () = assert! (core::mem::offset_of! (iterator_stack_entry, end_charpos) == 16);
const _: () = assert! (core::mem::offset_of! (iterator_stack_entry, stop_charpos) == 24);
const _: () = assert! (core::mem::offset_of! (iterator_stack_entry, prev_stop) == 32);
const _: () = assert! (core::mem::offset_of! (iterator_stack_entry, base_level_stop) == 40);
const _: () = assert! (core::mem::offset_of! (iterator_stack_entry, cmp_it) == 48);
const _: () = assert! (core::mem::offset_of! (iterator_stack_entry, face_id) == 136);
const _: () = assert! (core::mem::offset_of! (iterator_stack_entry, u) == 144);
const _: () = assert! (core::mem::offset_of! (iterator_stack_entry, position) == 192);
const _: () = assert! (core::mem::offset_of! (iterator_stack_entry, current) == 208);
const _: () = assert! (core::mem::offset_of! (iterator_stack_entry, from_overlay) == 256);
const _: () = assert! (core::mem::offset_of! (iterator_stack_entry, area) == 264);
const _: () = assert! (core::mem::offset_of! (iterator_stack_entry, method) == 268);
const _: () = assert! (core::mem::offset_of! (iterator_stack_entry, paragraph_embedding) == 272);
const _: () = assert! (core::mem::offset_of! (iterator_stack_entry, bits) == 276);
const _: () = assert! (core::mem::offset_of! (iterator_stack_entry, line_wrap) == 280);
const _: () = assert! (core::mem::offset_of! (iterator_stack_entry, voffset) == 284);
const _: () = assert! (core::mem::offset_of! (iterator_stack_entry, space_width) == 288);
const _: () = assert! (core::mem::offset_of! (iterator_stack_entry, font_height) == 296);

/* C: dispextern.h 2592 `struct iterator_stack_entry`'s one-byte `bool_bf` run (2628-2634), read
   and written by C's member name (measured byte patterns above).  */
pub const IT_STACK_MULTIBYTE_P: u8 = 1 << 0;
pub const IT_STACK_STRING_FROM_DISPLAY_PROP_P: u8 = 1 << 1;
pub const IT_STACK_STRING_FROM_PREFIX_PROP_P: u8 = 1 << 2;
pub const IT_STACK_DISPLAY_ELLIPSIS_P: u8 = 1 << 3;
pub const IT_STACK_AVOID_CURSOR_P: u8 = 1 << 4;
pub const IT_STACK_BIDI_P: u8 = 1 << 5;
pub const IT_STACK_FROM_DISP_PROP_P: u8 = 1 << 6;

macro_rules! it_stack_bits {
  (display_ellipsis_p, $e:expr) => { ((*$e).bits & IT_STACK_DISPLAY_ELLIPSIS_P != 0) };
  (display_ellipsis_p, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & ! IT_STACK_DISPLAY_ELLIPSIS_P)
		 | (if $v { IT_STACK_DISPLAY_ELLIPSIS_P } else { 0 }) };
  (multibyte_p, $e:expr) => { ((*$e).bits & IT_STACK_MULTIBYTE_P != 0) };
  (multibyte_p, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & ! IT_STACK_MULTIBYTE_P)
		 | (if $v { IT_STACK_MULTIBYTE_P } else { 0 }) };
  (string_from_display_prop_p, $e:expr) =>
    { ((*$e).bits & IT_STACK_STRING_FROM_DISPLAY_PROP_P != 0) };
  (string_from_display_prop_p, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & ! IT_STACK_STRING_FROM_DISPLAY_PROP_P)
		 | (if $v { IT_STACK_STRING_FROM_DISPLAY_PROP_P } else { 0 }) };
  (string_from_prefix_prop_p, $e:expr) =>
    { ((*$e).bits & IT_STACK_STRING_FROM_PREFIX_PROP_P != 0) };
  (string_from_prefix_prop_p, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & ! IT_STACK_STRING_FROM_PREFIX_PROP_P)
		 | (if $v { IT_STACK_STRING_FROM_PREFIX_PROP_P } else { 0 }) };
  (avoid_cursor_p, $e:expr) => { ((*$e).bits & IT_STACK_AVOID_CURSOR_P != 0) };
  (avoid_cursor_p, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & ! IT_STACK_AVOID_CURSOR_P)
		 | (if $v { IT_STACK_AVOID_CURSOR_P } else { 0 }) };
  (bidi_p, $e:expr) => { ((*$e).bits & IT_STACK_BIDI_P != 0) };
  (bidi_p, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & ! IT_STACK_BIDI_P)
		 | (if $v { IT_STACK_BIDI_P } else { 0 }) };
  (from_disp_prop_p, $e:expr) => { ((*$e).bits & IT_STACK_FROM_DISP_PROP_P != 0) };
  (from_disp_prop_p, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & ! IT_STACK_FROM_DISP_PROP_P)
		 | (if $v { IT_STACK_FROM_DISP_PROP_P } else { 0 }) };
}

/* C: dispextern.h 2387-2428 `struct composition_it`: the `cmp_it` member of `struct it`.
   Only the members this unit reaches are named (`stop_pos`, `id`, `ch`, `nglyphs`,
   `parent_it`, `nchars`, `nbytes`); every other member is inside a measured pad.  clang
   against the frozen target: sizeof 88, `ch` 16, `nglyphs` 40, `parent_it` 48.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct composition_it
{
  /// Next position at which to check the composition.  (C 2388-2389)
  pub stop_pos: ptrdiff_t,              // offset 0
  /// ID number of the composition or glyph-string.  If negative, we are not iterating over
  /// a composition now.  (C 2392-2394)
  pub id: ptrdiff_t,                    // offset 8
  /// If non-negative, character that triggers the automatic composition at `stop_pos`.  (C 2395-2401)
  pub ch: c_int,                        // offset 16
  /// C 2402-2410: `rule_idx` and `lookback` (offset 20 … 40).
  pub _pad_b: [u8; 20],                 // 20 … 40
  /// If non-negative, number of glyphs of the glyph-string.  (C 2405-2406)
  pub nglyphs: c_int,                   // offset 40
  /// True iff the composition is created while buffer is scanned in reverse order.
  /// (C 2410-2413)
  pub reversed_p: bool,                 // offset 44
  /// C 2414: the alignment hole before `parent_it` (offset 45 … 48).
  pub _pad_b2: [u8; 3],                 // 45 … 48
  /// Parent iterator.  (C 2414-2415)
  pub parent_it: *mut it,               // offset 48
  /// Position of the first character of the current grapheme cluster.  (C 2419-2420)
  pub charpos: ptrdiff_t,               // offset 56
  /// Number of characters and bytes of the current grapheme cluster.  (C 2421-2422)
  pub nchars: c_int,                    // offset 64
  pub nbytes: c_int,                    // 2422, offset 68
  /// Indices of the glyphs for the current grapheme cluster.  (C 2423-2424)
  pub from: c_int,                      // offset 72
  pub to: c_int,                        // 2424, offset 76
  /// C 2425-2428: `width` (offset 80 … 88).
  pub _pad_c: [u8; 8],
}

const _: () = assert! (core::mem::size_of::<composition_it> () == 88);
const _: () = assert! (core::mem::offset_of! (composition_it, stop_pos) == 0);
const _: () = assert! (core::mem::offset_of! (composition_it, id) == 8);
const _: () = assert! (core::mem::offset_of! (composition_it, ch) == 16);
const _: () = assert! (core::mem::offset_of! (composition_it, nglyphs) == 40);
const _: () = assert! (core::mem::offset_of! (composition_it, reversed_p) == 44);
const _: () = assert! (core::mem::offset_of! (composition_it, from) == 72);
const _: () = assert! (core::mem::offset_of! (composition_it, to) == 76);
const _: () = assert! (core::mem::offset_of! (composition_it, parent_it) == 48);
const _: () = assert! (core::mem::offset_of! (composition_it, charpos) == 56);
const _: () = assert! (core::mem::offset_of! (composition_it, nchars) == 64);
const _: () = assert! (core::mem::offset_of! (composition_it, nbytes) == 68);

/* C: dispextern.h 2430-2936 `struct it`, the display iterator.  `dispextern.h` is mirrored
   only to its batch-II prefix (P-66), so this record is declared here as this unit's own
   measured copy — the members this unit reaches are named, every other member is inside a
   measured pad whose `///` line names the C range it covers, and the layout is pinned by the
   `const _` assertions below.  clang against the frozen target: sizeof 4968, align 8, and
   every named offset (the `spikes/run.sh` flags).  Fields C declares as `bool_bf : 1` live
   in the byte runs at 112 and 2216 and in the byte at 2614; they are read and written
   through `it_bits!`, which names the member (PORTING.md 1.3/P-80).  `C's struct it it1 =
   it` copies (window.rs's `window_scroll_pixel_based`) are the port's `Clone`/`Copy`.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct it
{
  /// The window in which we iterate over current_buffer (or a string).  (C 2434)
  pub window: Lisp_Object,              // offset 0
  pub w: *mut window,                   // 2435, offset 8
  /// The window's frame.  (C 2438)
  pub f: *mut frame,                    // offset 16
  /// Method to use to load this structure with the next display element.  (C 2441)
  pub method: it_method,                // offset 24
  /// C 2444: the alignment hole before `stop_charpos` (offset 28 … 32).
  pub _pad_a: [u8; 4],
  /// The next position at which to check for face changes, invisible text, overlay
  /// strings, end of text etc., which see.  (C 2446-2447)
  pub stop_charpos: ptrdiff_t,          // offset 32
  /// Previous stop position, i.e. the last one before the current iterator position in
  /// `current'.  (C 2449-2450)
  pub prev_stop: ptrdiff_t,             // offset 40
  /// Last stop position iterated across whose bidi embedding level is equal to the
  /// current paragraph's base embedding level.  (C 2452-2454)
  pub base_level_stop: ptrdiff_t,       // offset 48
  /// Maximum string or buffer position + 1.  ZV when iterating over current_buffer.
  /// (C 2456-2462)
  pub end_charpos: ptrdiff_t,           // offset 56
  /// C 2464-2478: `medium_narrowing_begv`, `medium_narrowing_zv`, `large_narrowing_begv`,
  /// `large_narrowing_zv` (offset 64 … 96).
  pub medium_narrowing_begv: ptrdiff_t, // offset 64
  pub medium_narrowing_zv: ptrdiff_t,   // 72
  pub large_narrowing_begv: ptrdiff_t,  // 80
  pub large_narrowing_zv: ptrdiff_t,    // 88
  /// If we are iterating a string, the string's characters.  (C 2557)
  pub s: *const u8,                     // offset 96
  pub string_nchars: ptrdiff_t,         // 2558, offset 104
  /// C 2459-2471: the eight `bool_bf : 1` members `multibyte_p`, `tab_line_p`,
  /// `header_line_p`, `string_from_display_prop_p`, `string_from_prefix_prop_p`,
  /// `from_disp_prop_p`, `ellipsis_p`, `avoid_cursor_p` — one byte at 112.
  pub bits: u8,                         // offset 112
  /// Display table in effect or null for none.  (C 2519)
  pub dp: *mut Lisp_Char_Table,         // offset 120
  /// Current display table vector to return characters from.  (C 2525)
  pub dpvec: *mut Lisp_Object,          // offset 128
  /// The end of the current display table vector.  (C 2525)
  pub dpend: *mut Lisp_Object,          // offset 136
  /// Length in bytes of the char that filled dpvec.  (C 2529-2531)
  pub dpvec_char_len: c_int,            // offset 144
  /// Face id to use for all characters in display vector, -1 if unused.  (C 2533-2534)
  pub dpvec_face_id: c_int,             // offset 148
  /// Face id of the iterator saved in case a glyph from dpvec contains a face.  (C 2539)
  pub saved_face_id: c_int,             // offset 152
  /// C 2541-2545: the alignment hole before `ctl_chars` (offset 156 … 160; clang puts the
  /// array at 160, where the earlier comment said 156).
  pub _pad_b1: [u8; 4],
  /// Vector of glyphs for control character translation.  The pointer `dpvec` is set to
  /// `ctl_chars` when a control character is translated.  (C 2541-2546)
  pub ctl_chars: [Lisp_Object; 16],     // offset 160
  /// Where to continue from when we get to the end of what we are currently iterating
  /// over, e.g. when we have exhausted an overlay string.  (C 2548-2551)
  pub start: display_pos,               // offset 288
  /// Current buffer or string position of the iterator, including position in overlay
  /// strings etc.  (C 2512)
  pub current: display_pos,             // offset 336
  /// Total number of overlay strings to process.  (C 2562)
  pub n_overlay_strings: ptrdiff_t,     // offset 384
  /// The charpos where `n_overlay_strings` was calculated.  (C 2568)
  pub overlay_strings_charpos: ptrdiff_t,   // offset 392
  /// The strings of the current overlay, if any.  (C 2520)
  pub overlay_strings: [Lisp_Object; OVERLAY_STRING_CHUNK_SIZE as usize],   // offset 400
  /// For each overlay string, the overlay it came from.  (C 2526)
  pub string_overlays: [Lisp_Object; OVERLAY_STRING_CHUNK_SIZE as usize],   // offset 528
  /// If non-nil, a Lisp string being processed.  (C 2532)
  pub string: Lisp_Object,              // offset 656
  /// If non-nil, we are processing a string that came from a `display' property given by
  /// an overlay.  (C 2536)
  pub from_overlay: Lisp_Object,        // offset 664
  /// Stack of saved values.  (C 2541)
  pub stack: [iterator_stack_entry; IT_STACK_SIZE as usize],               // offset 672
  /// Stack pointer.  (C 2667)
  pub sp: c_int,                        // offset 2192
  /// -1 means selective display hides everything between a \r and the next newline; an
  /// integer means hide lines indented more than that column.  (C 2650)
  pub selective: ptrdiff_t,             // offset 2200
  /// What kind of element `current' describes.  (C 2697)
  pub what: display_element_type,       // offset 2208
  /// Real face id of the element.  (C 2700)
  pub face_id: c_int,                   // offset 2212
  /// C 2702-2719: the thirteen `bool_bf : 1` members `selective_display_ellipsis_p` …
  /// `align_visually_p` (offset 2216 … 2220).
  pub bits2: [u8; 4],                   // offset 2216
  /// How to wrap or truncate long lines.  (C 2692)
  pub line_wrap: line_wrap_method,      // offset 2220
  /// The ID of the default face to use.  (C 2696)
  pub base_face_id: c_int,              // offset 2224
  /// If `what' == IT_CHARACTER, the character and the length in bytes of its multibyte
  /// sequence.  (C 2711-2712: `int c, len;` — clang measures `c` 2228 and `len` 2232, both
  /// four bytes; the copy's earlier `ptrdiff_t` was wrong, and it also kept `&it->len` from
  /// meeting `check_char_and_length`'s `int *`.)
  pub c: c_int,                         // offset 2228
  pub len: c_int,                       // 2712, offset 2232
  /// C 2713-2714: the alignment hole before `cmp_it` (offset 2236 … 2240).
  pub _pad_d0: [u8; 4],
  /// If what == IT_COMPOSITION, iterator substructure for the composition.  (C 2715-2716)
  pub cmp_it: composition_it,           // offset 2240
  /// The character to display for this display element.  (C 2720)
  pub char_to_display: c_int,           // offset 2328
  /// If what == IT_GLYPHLESS, the method to display such a character.  (C 2746-2748)
  pub glyphless_method: glyphless_display_method,  // offset 2332
  /// If `what' == IT_IMAGE, the id of the image to display.  (C 2725)
  pub image_id: ptrdiff_t,              // offset 2336
  /// C 2752-2753: `struct xwidget *xwidget;`.  `HAVE_XWIDGETS` is undef on this target, so
  /// C's `struct xwidget` is an incomplete type; the port keeps the pointer as
  /// `*mut c_void` (P-72 rule 3, the `termhooks.rs` `rif` shape).
  pub xwidget: *mut c_void,             // offset 2344
  /// Values from `slice' property.  (C 2755-2756)
  pub slice: it_slice,                  // offset 2352
  /// Value of the `space-width' property, if any; nil if none.  (C 2758-2759)
  pub space_width: Lisp_Object,         // offset 2384
  /// Computed from the value of the `raise' property.  (C 2761-2762)
  pub voffset: c_short,                 // offset 2392
  /// Number of columns per \t.  (C 2795-2796)
  pub tab_width: c_short,               // offset 2394
  /// C 2797-2798: the alignment hole before `font_height` (offset 2396 … 2400).
  pub _pad_e0: [u8; 4],
  /// Value of the `height' property, if any; nil if none.  (C 2800-2801)
  pub font_height: Lisp_Object,         // offset 2400
  /// The object, if any, from which the current text came.  (C 2793)
  pub object: Lisp_Object,              // offset 2408
  /// The position of the current text in the buffer.  (C 2794)
  pub position: text_pos,               // offset 2416
  /// Width, in pixels, of the truncation and continuation glyphs.  (C 2797-2799)
  pub truncation_pixel_width: c_short,  // offset 2432
  pub continuation_pixel_width: c_short,// 2434
  /// First and last visible x-position in the display area.  (C 2806)
  pub first_visible_x: c_int,           // offset 2436
  pub last_visible_x: c_int,            // 2440
  /// Last visible y-position in the display area.  (C 2810)
  pub last_visible_y: c_int,            // offset 2444
  pub extra_line_spacing: c_int,        // 2813, offset 2448
  pub extra_line_spacing_above: c_int,  // 2816, offset 2452
  pub max_extra_line_spacing: c_int,    // 2821, offset 2456
  pub override_ascent: c_int,           // 2827, offset 2460
  pub override_descent: c_int,          // 2828, offset 2464
  pub override_boff: c_int,             // 2829, offset 2468
  /// The glyph row in which to produce glyphs.  (C 2833)
  pub glyph_row: *mut glyph_row,        // offset 2472
  /// The area of glyph_row to which glyphs are added.  (C 2836)
  pub area: glyph_row_area,             // offset 2480
  /// Number of glyphs produced so far in the current row.  (C 2839)
  pub nglyphs: c_int,                   // offset 2484
  /// Width in pixels of the glyph produced last.  (C 2842)
  pub pixel_width: c_int,               // offset 2488
  /// Current, maximum logical, and maximum physical line height information.  (C 2843)
  pub ascent: c_int,                    // offset 2492
  pub descent: c_int,                   // 2492
  pub max_ascent: c_int,                // offset 2500
  pub max_descent: c_int,               // 2504
  pub phys_ascent: c_int,               // offset 2508
  pub phys_descent: c_int,              // 2512
  pub max_phys_ascent: c_int,           // offset 2516
  pub max_phys_descent: c_int,          // 2520
  /// Current x pixel position within the display line.  (C 2853)
  pub current_x: c_int,                 // offset 2524
  /// Pixel position within a display line with a wrap prefix.  (C 2862)
  pub wrap_prefix_width: c_int,         // offset 2528
  /// Accumulated width of continuation lines.  (C 2866)
  pub continuation_lines_width: c_int,  // offset 2532
  /// C 2868-2870: `eol_pos` (a `struct text_pos`, 16 bytes at 2536).
  pub _pad_f: [u8; 16],
  /// Current y-position.  (C 2876)
  pub current_y: c_int,                 // offset 2552
  /// Vertical matrix position of first text line in window.  (C 2879)
  pub first_vpos: c_int,                // offset 2556
  /// Current vertical matrix position, or line number.  (C 2883)
  pub vpos: c_int,                      // offset 2560
  /// Horizontal matrix position reached in move_it_in_display_line.  (C 2887)
  pub hpos: c_int,                      // offset 2564
  /// Current line number, zero-based.  (C 2890)
  pub lnum: ptrdiff_t,                  // offset 2568
  /// The byte position corresponding to lnum.  (C 2893)
  pub lnum_bytepos: ptrdiff_t,          // offset 2576
  /// The width, in columns and in pixels, needed for display of the line numbers.  (C 2897)
  pub lnum_width: c_int,                // offset 2584
  pub lnum_pixel_width: c_int,          // 2588
  /// The line number of point's line, or zero if not computed yet.  (C 2899)
  pub pt_lnum: ptrdiff_t,               // offset 2592
  /// Number of pixels to adjust tab stops and stretch glyphs.  (C 2905)
  pub stretch_adjust: c_int,            // offset 2600
  /// C 2908-2929: the four `unsigned` fringe bitfields and the byte holding `bidi_p : 1`
  /// (the measured 12 bytes at 2604 … 2616).  `bidi_p` is bit 4 of `bits3[10]` (2614).
  pub bits3: [u8; 12],                  // offset 2604
  /// For iterating over bidirectional text.  (C 2933)
  pub bidi_it: bidi_it,                 // offset 2616
  /// The paragraph direction this buffer wants to use.  (C 2938-2941)
  pub paragraph_embedding: bidi_dir_t,  // offset 4944
  /// C 2943: the four-byte alignment hole before `min_width_property` (offset 4948 … 4952).
  pub _pad_tail_a: [u8; 4],
  /// For handling the :min-width property: the text property object we're testing the `eq'
  /// of (nil if none).  (C 2944)
  pub min_width_property: Lisp_Object,  // offset 4952
  /// The x position of the start of the run of glyphs.  (C 2945-2946)
  pub min_width_start: c_int,           // offset 4960
  /// C 2947: the tail (offset 4964 … 4968).
  pub _pad_tail: [u8; 4],
}

const _: () = assert! (core::mem::size_of::<it> () == 4968);
const _: () = assert! (core::mem::align_of::<it> () == 8);
const _: () = assert! (core::mem::offset_of! (it, window) == 0);
const _: () = assert! (core::mem::offset_of! (it, w) == 8);
const _: () = assert! (core::mem::offset_of! (it, f) == 16);
const _: () = assert! (core::mem::offset_of! (it, method) == 24);
const _: () = assert! (core::mem::offset_of! (it, stop_charpos) == 32);
const _: () = assert! (core::mem::offset_of! (it, end_charpos) == 56);
const _: () = assert! (core::mem::offset_of! (it, medium_narrowing_begv) == 64);
const _: () = assert! (core::mem::offset_of! (it, s) == 96);
const _: () = assert! (core::mem::offset_of! (it, bits) == 112);
const _: () = assert! (core::mem::offset_of! (it, dp) == 120);
const _: () = assert! (core::mem::offset_of! (it, dpvec) == 128);
const _: () = assert! (core::mem::offset_of! (it, dpend) == 136);
const _: () = assert! (core::mem::offset_of! (it, dpvec_char_len) == 144);
const _: () = assert! (core::mem::offset_of! (it, dpvec_face_id) == 148);
const _: () = assert! (core::mem::offset_of! (it, saved_face_id) == 152);
const _: () = assert! (core::mem::offset_of! (it, ctl_chars) == 160);
const _: () = assert! (core::mem::offset_of! (it, start) == 288);
const _: () = assert! (core::mem::offset_of! (it, current) == 336);
const _: () = assert! (core::mem::offset_of! (it, n_overlay_strings) == 384);
const _: () = assert! (core::mem::offset_of! (it, overlay_strings_charpos) == 392);
const _: () = assert! (core::mem::offset_of! (it, overlay_strings) == 400);
const _: () = assert! (core::mem::offset_of! (it, string_overlays) == 528);
const _: () = assert! (core::mem::offset_of! (it, string) == 656);
const _: () = assert! (core::mem::offset_of! (it, from_overlay) == 664);
const _: () = assert! (core::mem::offset_of! (it, stack) == 672);
const _: () = assert! (core::mem::offset_of! (it, sp) == 2192);
const _: () = assert! (core::mem::offset_of! (it, selective) == 2200);
const _: () = assert! (core::mem::offset_of! (it, what) == 2208);
const _: () = assert! (core::mem::offset_of! (it, face_id) == 2212);
const _: () = assert! (core::mem::offset_of! (it, bits2) == 2216);
const _: () = assert! (core::mem::offset_of! (it, line_wrap) == 2220);
const _: () = assert! (core::mem::offset_of! (it, base_face_id) == 2224);
const _: () = assert! (core::mem::offset_of! (it, c) == 2228);
const _: () = assert! (core::mem::offset_of! (it, len) == 2232);
const _: () = assert! (core::mem::offset_of! (it, char_to_display) == 2328);
const _: () = assert! (core::mem::offset_of! (it, char_to_display) == 2328);
const _: () = assert! (core::mem::offset_of! (it, glyphless_method) == 2332);
const _: () = assert! (core::mem::offset_of! (it, image_id) == 2336);
const _: () = assert! (core::mem::offset_of! (it, xwidget) == 2344);
const _: () = assert! (core::mem::offset_of! (it, slice) == 2352);
const _: () = assert! (core::mem::offset_of! (it, space_width) == 2384);
const _: () = assert! (core::mem::offset_of! (it, voffset) == 2392);
const _: () = assert! (core::mem::offset_of! (it, font_height) == 2400);
const _: () = assert! (core::mem::offset_of! (it, object) == 2408);
const _: () = assert! (core::mem::offset_of! (it, position) == 2416);
const _: () = assert! (core::mem::offset_of! (it, first_visible_x) == 2436);
const _: () = assert! (core::mem::offset_of! (it, last_visible_x) == 2440);
const _: () = assert! (core::mem::offset_of! (it, last_visible_y) == 2444);
const _: () = assert! (core::mem::offset_of! (it, extra_line_spacing) == 2448);
const _: () = assert! (core::mem::offset_of! (it, max_extra_line_spacing) == 2456);
const _: () = assert! (core::mem::offset_of! (it, glyph_row) == 2472);
const _: () = assert! (core::mem::offset_of! (it, area) == 2480);
const _: () = assert! (core::mem::offset_of! (it, nglyphs) == 2484);
const _: () = assert! (core::mem::offset_of! (it, pixel_width) == 2488);
const _: () = assert! (core::mem::offset_of! (it, ascent) == 2492);
const _: () = assert! (core::mem::offset_of! (it, descent) == 2496);
const _: () = assert! (core::mem::offset_of! (it, max_ascent) == 2500);
const _: () = assert! (core::mem::offset_of! (it, max_descent) == 2504);
const _: () = assert! (core::mem::offset_of! (it, phys_ascent) == 2508);
const _: () = assert! (core::mem::offset_of! (it, max_phys_descent) == 2520);
const _: () = assert! (core::mem::offset_of! (it, current_x) == 2524);
const _: () = assert! (core::mem::offset_of! (it, wrap_prefix_width) == 2528);
const _: () = assert! (core::mem::offset_of! (it, continuation_lines_width) == 2532);
const _: () = assert! (core::mem::offset_of! (it, current_y) == 2552);
const _: () = assert! (core::mem::offset_of! (it, first_vpos) == 2556);
const _: () = assert! (core::mem::offset_of! (it, vpos) == 2560);
const _: () = assert! (core::mem::offset_of! (it, hpos) == 2564);
const _: () = assert! (core::mem::offset_of! (it, lnum) == 2568);
const _: () = assert! (core::mem::offset_of! (it, lnum_bytepos) == 2576);
const _: () = assert! (core::mem::offset_of! (it, lnum_width) == 2584);
const _: () = assert! (core::mem::offset_of! (it, lnum_pixel_width) == 2588);
const _: () = assert! (core::mem::offset_of! (it, pt_lnum) == 2592);
const _: () = assert! (core::mem::offset_of! (it, cmp_it) == 2240);
const _: () = assert! (core::mem::offset_of! (it, tab_width) == 2394);
const _: () = assert! (core::mem::offset_of! (it, truncation_pixel_width) == 2432);
const _: () = assert! (core::mem::offset_of! (it, continuation_pixel_width) == 2434);
const _: () = assert! (core::mem::offset_of! (it, paragraph_embedding) == 4944);
const _: () = assert! (core::mem::offset_of! (it, min_width_property) == 4952);
const _: () = assert! (core::mem::offset_of! (it, min_width_start) == 4960);
const _: () = assert! (core::mem::offset_of! (it, stretch_adjust) == 2600);
const _: () = assert! (core::mem::offset_of! (it, bidi_it) == 2616);

/* `struct it`'s three `bool_bf : 1` runs, named member by member (PORTING.md 1.3/P-80;
   the `indent.rs`/`window.rs` `it_bits!` shape).  The byte index is the measured offset
   that carries the run, the value the mask of that member inside it.  */
pub const IT_MULTIBYTE_P: u8 = 1 << 0;              // byte 112
pub const IT_TAB_LINE_P: u8 = 1 << 1;
pub const IT_HEADER_LINE_P: u8 = 1 << 2;
pub const IT_STRING_FROM_DISPLAY_PROP_P: u8 = 1 << 3;
pub const IT_STRING_FROM_PREFIX_PROP_P: u8 = 1 << 4;
pub const IT_FROM_DISP_PROP_P: u8 = 1 << 5;
pub const IT_ELLIPSIS_P: u8 = 1 << 6;
pub const IT_AVOID_CURSOR_P: u8 = 1 << 7;
pub const IT_SELECTIVE_DISPLAY_ELLIPSIS_P: u8 = 1 << 0;   // byte 2216
pub const IT_CTL_ARROW_P: u8 = 1 << 1;
pub const IT_FACE_BOX_P: u8 = 1 << 2;
pub const IT_START_OF_BOX_RUN_P: u8 = 1 << 3;
pub const IT_END_OF_BOX_RUN_P: u8 = 1 << 4;
pub const IT_FACE_BEFORE_SELECTIVE_P: u8 = 1 << 1;        // byte 2217
pub const IT_OVERLAY_STRINGS_AT_END_PROCESSED_P: u8 = 1 << 5;
pub const IT_IGNORE_OVERLAY_STRINGS_AT_POS_P: u8 = 1 << 6;
pub const IT_GLYPH_NOT_AVAILABLE_P: u8 = 1 << 7;
pub const IT_CONSTRAIN_ROW_ASCENT_DESCENT_P: u8 = 1 << 2; // byte 2217
pub const IT_LINE_NUMBER_PRODUCED_P: u8 = 1 << 3;
pub const IT_ALIGN_VISUALLY_P: u8 = 1 << 4;
pub const IT_BIDI_P: u8 = 1 << 4;                         // byte 2614

macro_rules! it_bits {
  (multibyte_p, $it:expr) => { ((*$it).bits & IT_MULTIBYTE_P != 0) };
  (multibyte_p, $it:expr, $v:expr) => {
    (*$it).bits = ((*$it).bits & ! IT_MULTIBYTE_P)
                  | (if $v { IT_MULTIBYTE_P } else { 0 }) };
  (tab_line_p, $it:expr, $v:expr) => {
    (*$it).bits = ((*$it).bits & ! IT_TAB_LINE_P)
                  | (if $v { IT_TAB_LINE_P } else { 0 }) };
  (header_line_p, $it:expr, $v:expr) => {
    (*$it).bits = ((*$it).bits & ! IT_HEADER_LINE_P)
                  | (if $v { IT_HEADER_LINE_P } else { 0 }) };
  (string_from_display_prop_p, $it:expr) =>
    { ((*$it).bits & IT_STRING_FROM_DISPLAY_PROP_P != 0) };
  (string_from_prefix_prop_p, $it:expr) =>
    { ((*$it).bits & IT_STRING_FROM_PREFIX_PROP_P != 0) };
  (string_from_prefix_prop_p, $it:expr, $v:expr) => {
    (*$it).bits = ((*$it).bits & ! IT_STRING_FROM_PREFIX_PROP_P)
                  | (if $v { IT_STRING_FROM_PREFIX_PROP_P } else { 0 }) };
  (string_from_display_prop_p, $it:expr, $v:expr) => {
    (*$it).bits = ((*$it).bits & ! IT_STRING_FROM_DISPLAY_PROP_P)
                  | (if $v { IT_STRING_FROM_DISPLAY_PROP_P } else { 0 }) };
  (from_disp_prop_p, $it:expr) => { ((*$it).bits & IT_FROM_DISP_PROP_P != 0) };
  (from_disp_prop_p, $it:expr, $v:expr) => {
    (*$it).bits = ((*$it).bits & ! IT_FROM_DISP_PROP_P)
                  | (if $v { IT_FROM_DISP_PROP_P } else { 0 }) };
  (ellipsis_p, $it:expr) => { ((*$it).bits & IT_ELLIPSIS_P != 0) };
  (ellipsis_p, $it:expr, $v:expr) => {
    (*$it).bits = ((*$it).bits & ! IT_ELLIPSIS_P)
                  | (if $v { IT_ELLIPSIS_P } else { 0 }) };
  (ignore_overlay_strings_at_pos_p, $it:expr) =>
    { ((*$it).bits2 [0] & IT_IGNORE_OVERLAY_STRINGS_AT_POS_P != 0) };
  (ignore_overlay_strings_at_pos_p, $it:expr, $v:expr) => {
    (*$it).bits2 [0] = ((*$it).bits2 [0] & ! IT_IGNORE_OVERLAY_STRINGS_AT_POS_P)
                       | (if $v { IT_IGNORE_OVERLAY_STRINGS_AT_POS_P } else { 0 }) };
  (face_box_p, $it:expr) => { ((*$it).bits2 [0] & IT_FACE_BOX_P != 0) };
  (face_box_p, $it:expr, $v:expr) => {
    (*$it).bits2 [0] = ((*$it).bits2 [0] & ! IT_FACE_BOX_P)
                       | (if $v { IT_FACE_BOX_P } else { 0 }) };
  (selective_display_ellipsis_p, $it:expr) =>
    { ((*$it).bits2 [0] & IT_SELECTIVE_DISPLAY_ELLIPSIS_P != 0) };
  (selective_display_ellipsis_p, $it:expr, $v:expr) => {
    (*$it).bits2 [0] = ((*$it).bits2 [0] & ! IT_SELECTIVE_DISPLAY_ELLIPSIS_P)
                       | (if $v { IT_SELECTIVE_DISPLAY_ELLIPSIS_P } else { 0 }) };
  (ctl_arrow_p, $it:expr) => { ((*$it).bits2 [0] & IT_CTL_ARROW_P != 0) };
  (ctl_arrow_p, $it:expr, $v:expr) => {
    (*$it).bits2 [0] = ((*$it).bits2 [0] & ! IT_CTL_ARROW_P)
                       | (if $v { IT_CTL_ARROW_P } else { 0 }) };
  (start_of_box_run_p, $it:expr) => { ((*$it).bits2 [0] & IT_START_OF_BOX_RUN_P != 0) };
  (start_of_box_run_p, $it:expr, $v:expr) => {
    (*$it).bits2 [0] = ((*$it).bits2 [0] & ! IT_START_OF_BOX_RUN_P)
                       | (if $v { IT_START_OF_BOX_RUN_P } else { 0 }) };
  (end_of_box_run_p, $it:expr) => { ((*$it).bits2 [0] & IT_END_OF_BOX_RUN_P != 0) };
  (end_of_box_run_p, $it:expr, $v:expr) => {
    (*$it).bits2 [0] = ((*$it).bits2 [0] & ! IT_END_OF_BOX_RUN_P)
                       | (if $v { IT_END_OF_BOX_RUN_P } else { 0 }) };
  (bidi_p, $it:expr) => { ((*$it).bits3 [10] & IT_BIDI_P != 0) };
  (bidi_p, $it:expr, $v:expr) => {
    (*$it).bits3 [10] = ((*$it).bits3 [10] & ! IT_BIDI_P)
                        | (if $v { IT_BIDI_P } else { 0 }) };
  (line_number_produced_p, $it:expr) =>
    { ((*$it).bits2 [1] & IT_LINE_NUMBER_PRODUCED_P != 0) };
  /* C 11220 writes the member (`it->line_number_produced_p = false;`), the first body in
     this unit to do so.  */
  (line_number_produced_p, $it:expr, $v:expr) => {
    (*$it).bits2 [1] = ((*$it).bits2 [1] & ! IT_LINE_NUMBER_PRODUCED_P)
                       | (if $v { IT_LINE_NUMBER_PRODUCED_P } else { 0 }) };
  (face_before_selective_p, $it:expr) => { ((*$it).bits2 [1] & IT_FACE_BEFORE_SELECTIVE_P != 0) };
  (face_before_selective_p, $it:expr, $v:expr) => {
    (*$it).bits2 [1] = ((*$it).bits2 [1] & ! IT_FACE_BEFORE_SELECTIVE_P)
                       | (if $v { IT_FACE_BEFORE_SELECTIVE_P } else { 0 }) };
  /* C 2697-2699: `bool_bf constrain_row_ascent_descent_p : 1;` — byte 2217 bit 2, between
     `face_before_selective_p` (C 2695) and `line_number_produced_p` (C 2703).  The mask
     constant has been in the block since round 12's layout work; this round is the first
     body to touch the member (C 10724).  */
  (constrain_row_ascent_descent_p, $it:expr) =>
    { ((*$it).bits2 [1] & IT_CONSTRAIN_ROW_ASCENT_DESCENT_P != 0) };
  (constrain_row_ascent_descent_p, $it:expr, $v:expr) => {
    (*$it).bits2 [1] = ((*$it).bits2 [1] & ! IT_CONSTRAIN_ROW_ASCENT_DESCENT_P)
                       | (if $v { IT_CONSTRAIN_ROW_ASCENT_DESCENT_P } else { 0 }) };
  (avoid_cursor_p, $it:expr) => { ((*$it).bits & IT_AVOID_CURSOR_P != 0) };
  (avoid_cursor_p, $it:expr, $v:expr) => {
    (*$it).bits = ((*$it).bits & ! IT_AVOID_CURSOR_P)
                  | (if $v { IT_AVOID_CURSOR_P } else { 0 }) };
  (overlay_strings_at_end_processed_p, $it:expr) =>
    { ((*$it).bits2 [0] & IT_OVERLAY_STRINGS_AT_END_PROCESSED_P != 0) };
  (overlay_strings_at_end_processed_p, $it:expr, $v:expr) => {
    (*$it).bits2 [0] = ((*$it).bits2 [0] & ! IT_OVERLAY_STRINGS_AT_END_PROCESSED_P)
                       | (if $v { IT_OVERLAY_STRINGS_AT_END_PROCESSED_P } else { 0 }) };
  (align_visually_p, $it:expr, $v:expr) => {
    (*$it).bits2 [1] = ((*$it).bits2 [1] & ! IT_ALIGN_VISUALLY_P)
                       | (if $v { IT_ALIGN_VISUALLY_P } else { 0 }) };
}

/* C: dispextern.h 286-287, 291-294 and 2940-2943: the text-position accessors every
   display unit expands (the `indent.rs`/`dispnew.rs` `BYTEPOS`/`CHARPOS` shape; C's bodies
   arrive with the functions that use them — the `include/dispextern.rs` WIP note).  */
macro_rules! BYTEPOS { ($pos:expr) => { ($pos).bytepos }; }
macro_rules! CHARPOS { ($pos:expr) => { ($pos).charpos }; }
macro_rules! SET_TEXT_POS {
  ($pos:expr, $charpos:expr, $bytepos:expr) => {
    { ($pos).charpos = ($charpos); ($pos).bytepos = ($bytepos); }
  };
}
/* C: dispextern.h 296-320 `INC_TEXT_POS`/`DEC_TEXT_POS`.  The mirror's note says they arrive
   with the functions that use them, so this unit defines them next to `SET_TEXT_POS`
   (PORTING.md P-37: C's `do { … } while (false)` statement macro becomes a `macro_rules!`
   block, called as `NAME! (…)`).  */
macro_rules! INC_TEXT_POS {
  ($pos:expr, $multibyte_p:expr) => {
    { ($pos).charpos += 1;
      if $multibyte_p { ($pos).bytepos += next_char_len (($pos).bytepos) as ptrdiff_t; }
      else { ($pos).bytepos += 1; } }
  };
}
macro_rules! DEC_TEXT_POS {
  ($pos:expr, $multibyte_p:expr) => {
    { ($pos).charpos -= 1;
      if $multibyte_p { ($pos).bytepos -= prev_char_len (($pos).bytepos) as ptrdiff_t; }
      else { ($pos).bytepos -= 1; } }
  };
}
/* C: dispextern.h 336-337: `#define SET_MARKER_FROM_TEXT_POS(MARKER, POS)
   set_marker_both (MARKER, Qnil, CHARPOS (POS), BYTEPOS (POS))` -- the inverse of the macro
   below, and a mirror gap like it (PORTING.md 0.1-2/P-72 rule 3).  */
macro_rules! SET_MARKER_FROM_TEXT_POS {
  ($marker:expr, $pos:expr) => {
    { set_marker_both ($marker, Qnil (),
		       CHARPOS! ($pos) as ptrdiff_t, BYTEPOS! ($pos) as ptrdiff_t); }
  };
}
macro_rules! SET_TEXT_POS_FROM_MARKER { ($pos:expr, $marker:expr) => {
  { CHARPOS! ($pos) = marker_position ($marker); BYTEPOS! ($pos) = marker_byte_position ($marker); } }; }
macro_rules! IT_CHARPOS { ($it:expr) => { CHARPOS! (($it).current.pos) }; }
macro_rules! IT_BYTEPOS { ($it:expr) => { BYTEPOS! (($it).current.pos) }; }
macro_rules! IT_STRING_BYTEPOS { ($it:expr) => { BYTEPOS! (($it).current.string_pos) }; }
macro_rules! IT_STRING_CHARPOS { ($it:expr) => { CHARPOS! (($it).current.string_pos) }; }

/* C: dispextern.h 3067-3200 `struct redisplay_interface`, declared the way `dispnew.rs`
   declares its own copy (termhooks.rs degrades `terminal.rif` to `*mut c_void`, P-72 rule
   3).  Only the two members before and including `produce_glyphs` matter to this unit's
   `PRODUCE_GLYPHS`; clang against the frozen target puts `produce_glyphs` at 8.  The
   `#ifdef HAVE_WINDOW_SYSTEM` tail of the record is dead on this target (P-08).  */
#[repr(C)]
pub struct redisplay_interface
{
  _prefix_frame_parm_handlers: [*mut c_void; 1],
  pub produce_glyphs: Option<unsafe extern "C" fn (it: *mut it)>,
  /// C 3080-3114: `write_glyphs` … `update_window_end_hook` (offset 16 … 72).
  _pad0: [u8; 56],
  /// C 3115: `void (*flush_display) (struct frame *f);` — clang measures 72, sizeof 104.
  pub flush_display: Option<unsafe extern "C" fn (f: *mut frame)>,
  /// C 3116-3200: the rest of the record (offset 80 … 104).
  _pad1: [u8; 24],
}
const _: () = assert! (core::mem::offset_of! (redisplay_interface, produce_glyphs) == 8);
const _: () = assert! (core::mem::offset_of! (redisplay_interface, flush_display) == 72);
const _: () = assert! (core::mem::size_of::<redisplay_interface> () == 104);

/* C: frame.h 1913-1920: `INLINE void flush_frame (struct frame *f) { struct
   redisplay_interface *rif = FRAME_RIF (f); if (rif && rif->flush_display)
   rif->flush_display (f); }` -- `INLINE` gives no external symbol, so each unit defines its
   own copy (`minibuf.rs` is the precedent).  */
unsafe fn flush_frame (f: *mut frame)
{
  let rif: *mut redisplay_interface = FRAME_RIF! (f) as *mut redisplay_interface;

  if !rif.is_null () && !(*rif).flush_display.is_none ()
    { (*rif).flush_display.unwrap () (f); }
}

/* C: dispextern.h 2954-2957: `ITERATOR_AT_END_OF_LINE_P`.  The mirror does not carry it
   (the dispextern.h WIP note).  C's `(IT)->selective` is a `ptrdiff_t` tested for
   truthiness, the port's `!= 0`.  */
macro_rules! ITERATOR_AT_END_OF_LINE_P {
  ($it:expr) => {
    ((*$it).what == IT_CHARACTER
     && ((*$it).c == '\n' as c_int
	 || ((*$it).c == '\r' as c_int && (*$it).selective != 0)))
  };
}

/* C: dispextern.h 2962-2971: `PRODUCE_GLYPHS`.  The `FRAME_RIF (IT->f)->produce_glyphs`
   call is the port's a call through the minimal record above; `produce_glyphs` (term.c) is
   the fallback `FRAME_RIF`-null branch C also has.  `inhibit_free_realized_faces` is a
   `syms_of_xdisp` `DEFVAR_BOOL` target this unit owns.  */
macro_rules! PRODUCE_GLYPHS {
  ($it:expr) => { {
    let it = $it;
    if !(*it).glyph_row.is_null () && it_bits! (bidi_p, it)
      { glyph_row_bits! (reversed_p, (*it).glyph_row,
                         (*it).bidi_it.paragraph_dir == R2L); }
    let rif: *mut redisplay_interface = FRAME_RIF! ((*it).f) as *mut redisplay_interface;
    if !rif.is_null () { ((*rif).produce_glyphs.unwrap ()) (it); }
    else { produce_glyphs (it); }
    if !(*it).glyph_row.is_null () { inhibit_free_realized_faces = true; }
    reset_box_start_end_flags (it);
  } };
}

/* C 508: `#define DISP_INFINITY 10000000`.  */
pub const DISP_INFINITY: c_int = 10000000;

/* Holds the list (error).  (C 510-511)  */
static mut list_of_error: Lisp_Object = lisp_h_Qnil ();

/* C 514-515's forward declarations (`restore_selected_window`,
   `restore_frame_selected_window`) are dropped: Rust's items are order-independent and a
   declaration plus the definition is E0428 (PORTING.md 0.1-12).  */

/* C 517-532: the `#ifdef HAVE_WINDOW_SYSTEM` arm of
   `IT_OVERFLOW_NEWLINE_INTO_FRINGE` (`HAVE_WINDOW_SYSTEM` is undefined in the frozen
   configuration, macros.tsv) is recorded in `docs/ports/xdisp.md`, not transcribed; the
   live `#else` arm is the macro below.  */
macro_rules! IT_OVERFLOW_NEWLINE_INTO_FRINGE { ($it:expr) => { { let _ = $it; false } }; }

/* C 534-546: `IT_DISPLAYING_WHITESPACE`.  */
macro_rules! IT_DISPLAYING_WHITESPACE {
  ($it:expr) => {
    ((*$it).what == IT_CHARACTER && ((*$it).c == ' ' as c_int || (*$it).c == '\t' as c_int))
    || ((STRINGP ((*$it).string)
         && (SREF ((*$it).string, IT_STRING_BYTEPOS! (*$it) as usize) == b' '
             || SREF ((*$it).string, IT_STRING_BYTEPOS! (*$it) as usize) == b'\t'))
        || (!(*$it).s.is_null ()
            && (*(*$it).s.offset (IT_BYTEPOS! (*$it)) == b' '
                || *(*$it).s.offset (IT_BYTEPOS! (*$it)) == b'\t'))
        || (IT_BYTEPOS! (*$it) < ZV_BYTE! ()
            && (*BYTE_POS_ADDR (IT_BYTEPOS! (*$it)) == b' '
                || *BYTE_POS_ADDR (IT_BYTEPOS! (*$it)) == b'\t')))
  };
}

/* These are the category sets we use.  They are defined by kinsoku.el and characters.el.
   (C 548-554)  */
/* C declares these three as `char` constants, which is the type `coding.rs` gives
   `CODING_INHIBIT_CHARACTER_SUBSTITUTION`; the two call sites below take the `int` parameter
   `it_char_has_category` wants from the same value C passes (PORTING.md 1.1).  */
pub const NOT_AT_EOL: char = '<';
pub const NOT_AT_BOL: char = '>';
pub const LINE_BREAKABLE: char = '|';

/* Return true if the current character in IT has category CAT.  (C 556-574)  */
unsafe fn it_char_has_category (it: *mut it, cat: c_int) -> bool
{
  let mut ch: c_int = 0;
  if (*it).what == IT_CHARACTER
    { ch = (*it).c; }
  else if STRINGP ((*it).string)
    { ch = SREF ((*it).string, IT_STRING_BYTEPOS! (*it) as usize) as c_int; }
  else if !(*it).s.is_null ()
    { ch = *(*it).s.offset (IT_BYTEPOS! (*it)) as c_int; }
  else if IT_BYTEPOS! (*it) < ZV_BYTE! ()
    { ch = *BYTE_POS_ADDR (IT_BYTEPOS! (*it)) as c_int; }

  if ch == 0
    { return false; }
  else
    { return CHAR_HAS_CATEGORY (ch, cat); }
}

/* Return true if the current character allows wrapping before it.  (C 576-597)  */
unsafe fn char_can_wrap_before (it: *mut it) -> bool
{
  if !word_wrap_by_category
    { return ! IT_DISPLAYING_WHITESPACE! (it); }

  /* For CJK (LTR) text in RTL paragraph, EOL and BOL are flipped.
     Because in RTL paragraph, each glyph is prepended to the last
     one, effectively drawing right to left.  */
  let not_at_bol: c_int;
  if !(*it).glyph_row.is_null () && glyph_row_bits! (reversed_p, (*it).glyph_row)
    { not_at_bol = NOT_AT_EOL as c_int; }
  else
    { not_at_bol = NOT_AT_BOL as c_int; }
  /* You cannot wrap before a space or tab because that way you'll
     have space and tab at the beginning of next line.  */
  return ! IT_DISPLAYING_WHITESPACE! (it)
	 /* Can be at BOL.  */
	 && ! it_char_has_category (it, not_at_bol);
}

/* Return true if the current character allows wrapping after it.  (C 599-616)  */
unsafe fn char_can_wrap_after (it: *mut it) -> bool
{
  if !word_wrap_by_category
    { return IT_DISPLAYING_WHITESPACE! (it); }

  /* For CJK (LTR) text in RTL paragraph, EOL and BOL are flipped.
     Because in RTL paragraph, each glyph is prepended to the last
     one, effectively drawing right to left.  */
  let not_at_eol: c_int;
  if !(*it).glyph_row.is_null () && glyph_row_bits! (reversed_p, (*it).glyph_row)
    { not_at_eol = NOT_AT_BOL as c_int; }
  else
    { not_at_eol = NOT_AT_EOL as c_int; }

  return IT_DISPLAYING_WHITESPACE! (it)
	 /* Can break after && can be at EOL.  */
	 || (it_char_has_category (it, LINE_BREAKABLE as c_int)
	     && ! it_char_has_category (it, not_at_eol));
}

/* C 619-622: `#undef IT_DISPLAYING_WHITESPACE` / `#undef NOT_AT_EOL` / `#undef NOT_AT_BOL`
   / `#undef LINE_BREAKABLE` — the four names are C's (and the port's) only in the three
   functions above; Rust has no `#undef`, and leaving the definitions in place changes
   nothing (recorded in docs/ports/xdisp.md).  */

/* If all the conditions needed to print the fill column indicator are
   met, return the (nonnegative) column number, else return a negative
   value.  (C 624-649)  */
unsafe fn fill_column_indicator_column (it: *mut it, char_width: c_int) -> c_int
{
  if display_fill_column_indicator
      && !window_bits! ((*it).w, pseudo_window_p)
      && (*it).continuation_lines_width == 0
      && CHARACTERP (Vdisplay_fill_column_indicator_character)
    {
      let col: Lisp_Object
	= if EQ (Vdisplay_fill_column_indicator_column, Qt ())
	    { BVAR! (current_buffer, fill_column_) }
	  else { Vdisplay_fill_column_indicator_column };

      /* The stretch width needs to consider the latter
	 added glyph in append_space_for_newline.  */
      if RANGED_FIXNUMP (0, col, INT_MAX as EMACS_INT)
	{
          let mut icol: c_int = XFIXNUM (col) as c_int;
          /* C's `!ckd_mul (&icol, icol, char_width) && !ckd_add (&icol, icol,
             it->lnum_pixel_width)`: each store happens whether or not the operation
             overflowed, and the guard tests the overflow (P-15, the ckd family of
             docs/ports/token-aliases.tsv; the casefiddle.rs shape).  */
          let (mul, mul_overflowed) = icol.overflowing_mul (char_width);
          icol = mul;
          if !mul_overflowed
	    {
	      let (sum, add_overflowed) = icol.overflowing_add ((*it).lnum_pixel_width);
	      icol = sum;
	      if !add_overflowed
		{ return icol; }
	    }
	}
    }
  return -1;
}

/* True means print newline to stdout before next mini-buffer message.  (C 652-654)  */

#[unsafe(no_mangle)]
pub static mut noninteractive_need_newline: bool = false;

/* True means print newline to message log before next message.  (C 656-658)  */

static mut message_log_need_newline: bool = false;

/* Three markers that message_dolog uses.
   It could allocate them itself, but that causes trouble
   in handling memory-full errors.  (C 660-665)  */
static mut message_dolog_marker1: Lisp_Object = lisp_h_Qnil ();
static mut message_dolog_marker2: Lisp_Object = lisp_h_Qnil ();
static mut message_dolog_marker3: Lisp_Object = lisp_h_Qnil ();

/* The buffer position of the first character appearing entirely or
   partially on the line of the selected window which contains the
   cursor; <= 0 if not known.  Set by set_cursor_from_row, used for
   redisplay optimization in redisplay_internal.  (C 667-672)  */

static mut this_line_start_pos: text_pos = unsafe { core::mem::zeroed () };

/* Number of characters past the end of the line above, including the
   terminating newline.  (C 674-677)  */

static mut this_line_end_pos: text_pos = unsafe { core::mem::zeroed () };

/* The vertical positions and the height of this line.  (C 679-683)  */

static mut this_line_vpos: c_int = 0;
static mut this_line_y: c_int = 0;
static mut this_line_pixel_height: c_int = 0;

/* X position at which this display line starts.  Usually zero;
   negative if first character is partially visible.  (C 685-688)  */

static mut this_line_start_x: c_int = 0;

/* The smallest character position seen by move_it_* functions as they
   move across display lines.  Used to set MATRIX_ROW_START_CHARPOS of
   hscrolled lines, see display_line.  (C 690-694)  */

static mut this_line_min_pos: text_pos = unsafe { core::mem::zeroed () };

/* Buffer that this_line_.* variables are referring to.  (C 696-698)  */

static mut this_line_buffer: *mut buffer = core::ptr::null_mut ();

/* True if an overlay arrow has been displayed in this window.  (C 700-702)  */

static mut overlay_arrow_seen: bool = false;

/* Vector containing glyphs for an ellipsis `...'.  (C 704-706)  */

static mut default_invis_vector: [Lisp_Object; 3] = [lisp_h_Qnil (); 3];

/* `include/window.rs` declares `echo_area_window`, `update_mode_lines` and
   `windows_or_buffers_changed` at the mirror's *own top level* (window.h 1146/1154/1161,
   "the .c files define them"), so a second top-level definition here is E0428 — those three
   are therefore defined inside a module this unit glob-imports (PORTING.md 0.1-10/P-21, the
   `window.rs` `window_defined_decls`/`window_display_table` shape).  The mirror gap (the
   three declarations should sit in a module of their own) is reported in
   `docs/ports/xdisp.md` for the integrator; the symbol here is the one C defines.  */
pub mod xdisp_defined_globals {
  use super::*;

/* This is the window where the echo area message was displayed.  It
   is always a mini-buffer window, but it may not be the same window
   currently active as a mini-buffer.  (C 708-712)  */

#[unsafe(no_mangle)]
pub static mut echo_area_window: Lisp_Object = lisp_h_Qnil ();

/* At each redisplay cycle, we should refresh everything there is to refresh. ... (C 724-754) */

#[unsafe(no_mangle)]
pub static mut windows_or_buffers_changed: c_int = 0;

/* Nonzero if we should redraw the mode lines on the next redisplay. ... (C 756-775) */

#[unsafe(no_mangle)]
pub static mut update_mode_lines: c_int = 0;

}
use xdisp_defined_globals::*;

/* Stack of messages, which are pushed by push_message and popped and
   displayed by restore_message.  (C 714-717)  */

static mut Vmessage_stack: Lisp_Object = lisp_h_Qnil ();

/* True means multibyte characters were enabled when the echo area
   message was specified.  (C 719-722)  */

static mut message_enable_multibyte: bool = false;

/* True after display_mode_line if %l was used and it displayed a
   line number.  (C 777-780)  */

static mut line_number_displayed: bool = false;

/* Current, index 0, and last displayed echo area message.  Either
   buffers from echo_buffers, or nil to indicate no message.  (C 782-785)  */

#[unsafe(no_mangle)]
pub static mut echo_area_buffer: [Lisp_Object; 2] = [lisp_h_Qnil (); 2];

/* The buffers referenced from echo_area_buffer.  (C 787-789)  */

static mut echo_buffer: [Lisp_Object; 2] = [lisp_h_Qnil (); 2];

/* A vector saved used in with_area_buffer to reduce consing.  (C 791-793)  */

static mut Vwith_echo_area_save_vector: Lisp_Object = lisp_h_Qnil ();

/* True means display_echo_area should display the last echo area
   message again.  Set by redisplay_preserve_echo_area.  (C 795-798)  */

static mut display_last_displayed_message_p: bool = false;

/* True if echo area is being used by print; false if being used by
   message.  (C 800-803)  */

static mut message_buf_print: bool = false;

/* Set to true in clear_message to make redisplay_internal aware
   of an emptied echo area.  (C 805-808)  */

static mut message_cleared_p: bool = false;

/* A scratch glyph row with contents used for generating truncation
   glyphs and overlay-arrow glyphs.  (C 810-815)  */

pub const MAX_SCRATCH_GLYPHS: c_int = 100;
static mut scratch_glyph_row: glyph_row = unsafe { core::mem::zeroed () };
static mut scratch_glyphs: [glyph; MAX_SCRATCH_GLYPHS as usize]
  = unsafe { core::mem::zeroed () };

/* Ascent and height of the last line processed by move_it_to.  (C 817-819)  */

static mut last_height: c_int = 0;

/* True if there's a help-echo in the echo area.  (C 821-823)  */

#[unsafe(no_mangle)]
pub static mut help_echo_showing_p: bool = false;

/* The maximum distance to look ahead for text properties. ... (C 825-831)  */

pub const TEXT_PROP_DISTANCE_LIMIT: c_int = 100;

/* SAVE_IT and RESTORE_IT are called when we save a snapshot of the
   iterator state and later restore it. ... (C 833-859)  */
macro_rules! SAVE_IT {
  ($ITCOPY:expr, $ITORIG:expr, $CACHE:expr) => {
    {
      if !$CACHE.is_null () { bidi_unshelve_cache ($CACHE, true); }
      $ITCOPY = $ITORIG;
      $CACHE = bidi_shelve_cache ();
    }
  };
}
macro_rules! RESTORE_IT {
  ($pITORIG:expr, $pITCOPY:expr, $CACHE:expr) => {
    {
      if $pITORIG != $pITCOPY { *($pITORIG) = *($pITCOPY); }
      bidi_unshelve_cache ($CACHE, false);
      $CACHE = core::ptr::null_mut ();
    }
  };
}

/* buffer.h 702-716: the four `bool_bf` members of `struct buffer` share `bits`; the writer is
   here, with C's member name as the macro's first argument (PORTING.md 1.3/P-80, the
   keyboard.rs `buffer_bits!` precedent — the read-only half is in `include/buffer.rs`).  */
macro_rules! buffer_bits {
  (prevent_redisplay_optimizations_p, $b:expr, $v:expr) => {{
    if $v { (*$b).bits |= BUFFER_PREVENT_REDISPLAY_OPTIMIZATIONS_P; }
    else { (*$b).bits &= ! BUFFER_PREVENT_REDISPLAY_OPTIMIZATIONS_P; } }};
  (prevent_redisplay_optimizations_p, $b:expr) =>
    { (*$b).bits & BUFFER_PREVENT_REDISPLAY_OPTIMIZATIONS_P != 0 };
  (clip_changed, $b:expr) => { (*$b).bits & BUFFER_CLIP_CHANGED != 0 };
  (clip_changed, $b:expr, $v:expr) => {{
    if $v { (*$b).bits |= BUFFER_CLIP_CHANGED; }
    else { (*$b).bits &= ! BUFFER_CLIP_CHANGED; } }};
  (long_line_optimizations_p, $b:expr) =>
    { (*$b).bits & BUFFER_LONG_LINE_OPTIMIZATIONS_P != 0 };
}
/* buffer.h 240's `struct buffer_text` carries the same kind of 1-bit member (`redisplay`,
   buffer.h 303) in its own `bits`
   (`redisplay`, `BUFFER_TEXT_REDISPLAY` = 1 << 1, `include/buffer.rs`); the arm keeps C's own
   member path as its first argument so the statement's tokens stay C's.  */
macro_rules! buffer_text_bits {
  (text.redisplay, $b:expr) => { (*(*$b).text).bits & BUFFER_TEXT_REDISPLAY != 0 };
  (text.redisplay, $b:expr, $v:expr) => {{
    if $v { (*(*$b).text).bits |= BUFFER_TEXT_REDISPLAY; }
    else { (*(*$b).text).bits &= ! BUFFER_TEXT_REDISPLAY; } }};
}

/* frame.h 300-341's `bool_bf` flags live in two 4-byte units; `f->redisplay` is bit 7 of
   `bits1` (the `include/frame.rs` `FRAME_REDISPLAY_SHIFT`; the xfaces.rs/keyboard.rs
   `frame_bits!` shape, one arm per member this unit names).  */
macro_rules! frame_bits {
  /* frame.h 331's `bool_bf glyphs_initialized_p : 1` (bit 0 of `bits1`); the reader is here
     because the mirror carries the shift but no accessor for this member.  */
  (glyphs_initialized_p, $f:expr) =>
    { ((*$f).bits1 >> FRAME_GLYPHS_INITIALIZED_P_SHIFT & 1) != 0 };
  /* frame.h 336's `bool_bf resized_p : 1` (frame.h's `FRAME_RESIZED_P_SHIFT`, bit 1 of
     `bits1`) — `FRAME_GARBAGED_P`-style reader the mirror does not carry.  */
  (resized_p, $f:expr) => { ((*$f).bits1 >> FRAME_RESIZED_P_SHIFT & 1) != 0 };
  (resized_p, $f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_RESIZED_P_SHIFT, $v); };
  /* frame.h 336's `bool_bf garbaged : 1` (bit 10 of `bits1`) — the mirror has the
     `FRAME_GARBAGED_P` reader and the shift; the writer is here (C 13732's `f->garbaged =
     false;`).  */
  (garbaged, $f:expr) => { ((*$f).bits1 >> FRAME_GARBAGED_SHIFT & 1) != 0 };
  (garbaged, $f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_GARBAGED_SHIFT, $v); };
  (redisplay, $f:expr) => { ((*$f).bits1 >> FRAME_REDISPLAY_SHIFT) & 1 != 0 };
  (redisplay, $f:expr, $v:expr) => {
    (*$f).bits1 = ((*$f).bits1 & ! (1u32 << FRAME_REDISPLAY_SHIFT))
                  | (((($v) as u32) & 1) << FRAME_REDISPLAY_SHIFT); };
  (no_special_glyphs, $f:expr) => { (*$f).bits1 >> FRAME_NO_SPECIAL_GLYPHS_SHIFT & 1 != 0 };
  (no_special_glyphs, $f:expr, $v:expr) => {
    set_frame_flag1! ($f, FRAME_NO_SPECIAL_GLYPHS_SHIFT, $v); };
  (face_change, $f:expr) => { (*$f).bits2 >> FRAME_FACE_CHANGE_SHIFT & 1 != 0 };
  (face_change, $f:expr, $v:expr) => {
    set_frame_flag2! ($f, FRAME_FACE_CHANGE_SHIFT, $v); };
  (inhibit_clear_image_cache, $f:expr) =>
    { (*$f).bits2 >> FRAME_INHIBIT_CLEAR_IMAGE_CACHE_SHIFT & 1 != 0 };
  (inhibit_clear_image_cache, $f:expr, $v:expr) => {
    set_frame_flag2! ($f, FRAME_INHIBIT_CLEAR_IMAGE_CACHE_SHIFT, $v); };
  /* The five members `redisplay_internal` names (frame.h 344-494).  */
  (already_hscrolled_p, $f:expr) =>
    { ((*$f).bits1 >> FRAME_ALREADY_HSCROLLED_P_SHIFT & 1) != 0 };
  (already_hscrolled_p, $f:expr, $v:expr) =>
    { set_frame_flag1! ($f, FRAME_ALREADY_HSCROLLED_P_SHIFT, $v); };
  (updated_p, $f:expr) => { ((*$f).bits1 >> FRAME_UPDATED_P_SHIFT & 1) != 0 };
  (updated_p, $f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_UPDATED_P_SHIFT, $v); };
  (fonts_changed, $f:expr) => { ((*$f).bits1 >> FRAME_FONTS_CHANGED_SHIFT & 1) != 0 };
  (fonts_changed, $f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_FONTS_CHANGED_SHIFT, $v); };
  (cursor_type_changed, $f:expr) =>
    { ((*$f).bits1 >> FRAME_CURSOR_TYPE_CHANGED_SHIFT & 1) != 0 };
  (cursor_type_changed, $f:expr, $v:expr) =>
    { set_frame_flag1! ($f, FRAME_CURSOR_TYPE_CHANGED_SHIFT, $v); };
  (after_make_frame, $f:expr) => { ((*$f).bits1 >> FRAME_AFTER_MAKE_FRAME_SHIFT & 1) != 0 };
}

/* frame.h 300-341's two packed-flag units: the writer behind the `frame_bits!` arms above
   and the `FRAME_WINDOW_CHANGE` arm below (PORTING.md 1.3/P-80, the frame.rs/window.rs
   shape).  */
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

/* frame.h 1191: `#define FRAME_WINDOW_CHANGE(f) (f)->window_change`.  C uses it as an
   l-value; the mirror carries the read-only accessor of the same name, so the port's
   two-argument arm is the writer (the `window.rs` copy).  */
macro_rules! FRAME_WINDOW_CHANGE {
    ($f:expr) => { (*$f).bits1 >> FRAME_WINDOW_CHANGE_SHIFT & 1 != 0 };
    ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_WINDOW_CHANGE_SHIFT, $v) };
}

/* frame.h 1387: `#define FRAME_SELECTED_WINDOW(f) ((f)->selected_window)`.  C stores through
   it in `restore_selected_window` (C 14529) and `restore_frame_selected_window` (C 14572);
   the mirror carries the read-only accessor of the same name, so the store goes through this
   macro (the `FRAME_WINDOW_CHANGE` shape above, PORTING.md P-72 rule 3).  */
macro_rules! FRAME_SELECTED_WINDOW {
  ($f:expr) => { (*$f).selected_window };
}

/* Functions to mark elements as needing redisplay.  (C 861)  */
pub const REDISPLAY_SOME: c_int = 2;	/* Arbitrary choice.  */

/* C 863-864's forward declaration of `calc_pixel_width_or_height` is dropped
   (PORTING.md 0.1-12).  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn redisplay_other_windows ()
{
  if windows_or_buffers_changed == 0
    { windows_or_buffers_changed = REDISPLAY_SOME; }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wset_redisplay (w: *mut window)
{
  /* Beware: selected_window can be nil during early stages.  */
  if !BASE_EQ (make_lisp_ptr (w as *mut c_void, Lisp_Type::Lisp_Vectorlike), selected_window)
    { redisplay_other_windows (); }
  window_bits! (w, redisplay, true);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fset_redisplay (f: *mut frame)
{
  redisplay_other_windows ();
  frame_bits! (redisplay, f, true);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bset_redisplay (b: *mut buffer)
{
  let count: c_int = buffer_window_count (b);
  if count > 0
    {
      /* ... it's visible in other window than selected,  */
      if count > 1 || b != XBUFFER ((*XWINDOW (selected_window)).contents)
	{ redisplay_other_windows (); }
      /* Even if we don't set windows_or_buffers_changed, do set `redisplay'
	 so that if we later set windows_or_buffers_changed, this buffer will
	 not be omitted.  */
      buffer_text_bits! (text.redisplay, b, true);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bset_update_mode_line (b: *mut buffer)
{
  if update_mode_lines == 0
    { update_mode_lines = REDISPLAY_SOME; }
  buffer_text_bits! (text.redisplay, b, true);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wset_update_mode_line (w: *mut window)
{
  window_bits! (w, update_mode_line, true);
  /* When a window's mode line needs to be updated, the window's frame's
     title may also need to be updated, but we don't need to worry about it
     here.  Instead, `gui_consider_frame_title' is automatically called
     whenever w->update_mode_line is set for that frame's selected window.
     But for this to work reliably, we have to make sure the window
     is considered, so we have to mark it for redisplay.  */
  wset_redisplay (w);
}

DEFUN! ("set-buffer-redisplay", Fset_buffer_redisplay,
       Sset_buffer_redisplay, 4, 4, 0,
       doc: "Mark the current buffer for redisplay.\nThis function may be passed to `add-variable-watcher'.  ",
       (symbol: Lisp_Object, newval: Lisp_Object, op: Lisp_Object, where_: Lisp_Object),
       {
  let _ = (symbol, newval, op, where_);
  bset_update_mode_line (current_buffer);
  buffer_bits! (prevent_redisplay_optimizations_p, current_buffer, true);
  return Qnil ();
});

/* redisplay_trace is for displaying traces of redisplay.
   If Emacs was compiled with GLYPH_DEBUG defined, the variable
   trace_redisplay_p can be set to a non-zero value in debugging
   sessions to activate traces.  (C 933-942)  */
/* C 937-942's `#ifdef GLYPH_DEBUG` arm declares and defines
   `bool trace_redisplay_p EXTERNALLY_VISIBLE`; `GLYPH_DEBUG` is undefined in the frozen
   configuration (macros.tsv), so C's live text is the `#else` arm,
   `enum { trace_redisplay_p = false };` — the constant below.  The dead arm is recorded in
   docs/ports/xdisp.md (P-08).  */
const trace_redisplay_p: bool = false;

unsafe extern "C" fn redisplay_trace (fmt: *const c_char, mut args: ...)
{
  if trace_redisplay_p
    {
      let va: VaList<'_> = args;
      vfprintf (stderr, fmt, va);
    }
}

/* C 953-960: `#ifdef DEBUG_TRACE_MOVE`'s `bool trace_move` and the live `#else`'s
   `enum { trace_move = false };` — same shape as `trace_redisplay_p` above.  */
const trace_move: bool = false;

unsafe extern "C" fn move_trace (fmt: *const c_char, mut args: ...)
{
  if trace_move
    {
      let va: VaList<'_> = args;
      vfprintf (stderr, fmt, va);
    }
}

/* Buffer being redisplayed -- for redisplay_window_error.  (C 977-979)  */

static mut displayed_buffer: *mut buffer = core::ptr::null_mut ();

/* Value returned from text property handlers (see below).  (C 981-989)  */

pub type prop_handled = c_int;
pub const HANDLED_NORMALLY: prop_handled = 0;
pub const HANDLED_RECOMPUTE_PROPS: prop_handled = 1;
pub const HANDLED_OVERLAY_STRING_CONSUMED: prop_handled = 2;
pub const HANDLED_RETURN: prop_handled = 3;

/* C: dispextern.h 2318-2334 `enum prop_idx`.  */
pub type prop_idx = c_int;
pub const FONTIFIED_PROP_IDX: prop_idx = 0;
pub const FACE_PROP_IDX: prop_idx = 1;
pub const INVISIBLE_PROP_IDX: prop_idx = 2;
pub const DISPLAY_PROP_IDX: prop_idx = 3;
pub const COMPOSITION_PROP_IDX: prop_idx = 4;
pub const OVERLAY_PROP_IDX: prop_idx = 5;
pub const LAST_PROP_IDX: prop_idx = 6;

/* A description of text properties that redisplay is interested
   in.  (C 991-1001)  */

#[repr(C)]
pub struct props
{
  /* The symbol index of the name of the property.  */
  pub name: c_short,

  /* A unique index for the property.  */
  pub idx: prop_idx,

  /* A handler function called to set up iterator IT from the property
     at IT's current position.  Value is used to steer handle_stop.  */
  pub handler: Option<unsafe extern "C" fn (it: *mut it) -> prop_handled>,
}

/* C 1003-1008's six forward declarations (`handle_face_prop` … `handle_fontified_prop`) are
   dropped (PORTING.md 0.1-12): Rust's items are order-independent, and the definitions below
   in the file's C order are the declarations.  */

/* Properties handled by iterators.  (C 1010-1017)  */

static mut it_props: [props; 6] =
[
  props { name: iQfontified as c_short, idx: FONTIFIED_PROP_IDX,
          handler: Some (handle_fontified_prop) },
  /* Handle `face' before `display' because some sub-properties of
     `display' need to know the face.  */
  props { name: iQface as c_short, idx: FACE_PROP_IDX, handler: Some (handle_face_prop) },
  props { name: iQdisplay as c_short, idx: DISPLAY_PROP_IDX, handler: Some (handle_display_prop) },
  props { name: iQinvisible as c_short, idx: INVISIBLE_PROP_IDX,
          handler: Some (handle_invisible_prop) },
  props { name: iQcomposition as c_short, idx: COMPOSITION_PROP_IDX,
          handler: Some (handle_composition_prop) },
  props { name: 0, idx: 0, handler: None }
];

/* Enumeration returned by some move_it_.* functions internally.  (C 1019-1058)  */

pub type move_it_result = c_int;
/* Not used.  Undefined value.  */
pub const MOVE_UNDEFINED: move_it_result = 0;
/* Move ended at the requested buffer position or ZV.  */
pub const MOVE_POS_MATCH_OR_ZV: move_it_result = 1;
/* Move ended at the requested X pixel position.  */
pub const MOVE_X_REACHED: move_it_result = 2;
/* Move within a line ended at the end of a line that must
   be continued.  */
pub const MOVE_LINE_CONTINUED: move_it_result = 3;
/* Move within a line ended at the end of a line that would
   be displayed truncated.  */
pub const MOVE_LINE_TRUNCATED: move_it_result = 4;
/* Move within a line ended at a line end.  */
pub const MOVE_NEWLINE_OR_CR: move_it_result = 5;

/* This counter is used to clear the face cache every once in a while
   in redisplay_internal.  It is incremented for each redisplay.
   Every CLEAR_FACE_CACHE_COUNT full redisplays, the face cache is
   cleared.  (C 1060-1062)  */

pub const CLEAR_FACE_CACHE_COUNT: c_int = 500;
static mut clear_face_cache_count: c_int = 0;

/* C 1064-1071: `#ifdef HAVE_WINDOW_SYSTEM`'s `CLEAR_IMAGE_CACHE_COUNT`,
   `clear_image_cache_count` and `null_glyph_slice` are dead on this target (P-08) and are
   recorded in docs/ports/xdisp.md.  */

/* True while redisplay_internal is in progress.  (C 1073-1075)  */

#[unsafe(no_mangle)]
pub static mut redisplaying_p: bool = false;

/* True while some display-engine code is working on layout of some
   window.  (C 1077-1084)  */
#[unsafe(no_mangle)]
pub static mut display_working_on_window_p: bool = false;

/* Non-zero when we do not allow resizing frames. ... (C 1086-1094)  */

#[unsafe(no_mangle)]
pub static mut dont_resize_frames: c_int = 0;

/* If a string, XTread_socket generates an event to display that string.
   (The display is done in read_char.)  (C 1096-1101)  */

#[unsafe(no_mangle)]
pub static mut help_echo_string: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut help_echo_window: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut help_echo_object: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut help_echo_pos: ptrdiff_t = 0;

/* Temporary variable for XTread_socket.  (C 1103-1106)  */

#[unsafe(no_mangle)]
pub static mut previous_help_echo_string: Lisp_Object = lisp_h_Qnil ();

/* Platform-independent portion of hourglass implementation.  (C 1108-1122)  */
/* C 1110-1122's `#ifdef HAVE_WINDOW_SYSTEM` arms (`hourglass_shown_p`,
   `hourglass_atimer`, `THIN_SPACE_WIDTH`) are dead on this target (P-08); the live text is
   C 1113-1115's `DEFAULT_HOURGLASS_DELAY`.  */

/* Default number of seconds to wait before displaying an hourglass
   cursor.  (C 1113-1115)  */
pub const DEFAULT_HOURGLASS_DELAY: c_int = 1;

/* C 1124-1181's function prototypes are dropped (PORTING.md 0.1-12).  */

/* dispextern.h 1212-1213: two of the prototypes C 1183-1209 does not restate.  */
macro_rules! face_before_it_pos { ($IT:expr) => { face_before_or_after_it_pos (($IT), true) }; }
macro_rules! face_after_it_pos { ($IT:expr) => { face_before_or_after_it_pos (($IT), false) }; }

/* C 1211's `#ifdef HAVE_WINDOW_SYSTEM` block of prototypes (1215-1238) is dead on this
   target (P-08) and recorded in docs/ports/xdisp.md.  */

/***********************************************************************
		      Window display dimensions
 ***********************************************************************/

/* Return the bottom boundary y-position for text lines in window W.
   This is the first y position at which a line cannot start.
   It is relative to the top of the window.

   This is the height of W minus the height of a mode line, if any.  (C 1246-1272)  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn window_text_bottom_y (w: *mut window) -> c_int
{
  let mut height: c_int = WINDOW_PIXEL_HEIGHT (w);

  height -= WINDOW_BOTTOM_DIVIDER_WIDTH! (w);

  if window_wants_mode_line (w)
    { height -= CURRENT_MODE_LINE_HEIGHT! (w); }

  height -= WINDOW_SCROLL_BAR_AREA_HEIGHT! (w);

  return height;
}

/* Return the pixel width of display area AREA of window W.
   ANY_AREA means return the total width of W, not including
   fringes to the left and right of the window.  (C 1274-1299)  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn window_box_width (w: *mut window, area: glyph_row_area) -> c_int
{
  let mut width: c_int = (*w).pixel_width;

  if !window_bits! (w, pseudo_window_p)
    {
      width -= WINDOW_SCROLL_BAR_AREA_WIDTH! (w);
      width -= WINDOW_RIGHT_DIVIDER_WIDTH! (w);

      if area == TEXT_AREA
	{ width -= WINDOW_MARGINS_WIDTH! (w) + WINDOW_FRINGES_WIDTH! (w); }
      else if area == LEFT_MARGIN_AREA
	{ width = WINDOW_LEFT_MARGIN_WIDTH! (w); }
      else if area == RIGHT_MARGIN_AREA
	{ width = WINDOW_RIGHT_MARGIN_WIDTH! (w); }
    }

  /* With wide margins, fringes, etc. we might end up with a negative
     width, correct that here.  */
  return max (0, width as isize) as c_int;
}

/* Return the pixel height of the display area of window W, not
   including mode lines of W, if any.  (C 1301-1375)  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn window_box_height (w: *mut window) -> c_int
{
  let f: *mut frame = XFRAME ((*w).frame);
  let mut height: c_int = WINDOW_PIXEL_HEIGHT (w);

  eassert! (height >= 0);

  height -= WINDOW_BOTTOM_DIVIDER_WIDTH! (w);
  height -= WINDOW_SCROLL_BAR_AREA_HEIGHT! (w);

  /* Note: the code below that determines the mode-line/header-line/tab-line
     height is essentially the same as that contained in the macro
     CURRENT_{MODE,HEADER,TAB}_LINE_HEIGHT, except that it checks whether
     the appropriate glyph row has its `mode_line_p' flag set, and if
     it doesn't, uses estimate_mode_line_height instead.  */

  if window_wants_mode_line (w)
    {
      if (*w).mode_line_height >= 0
	{ height -= (*w).mode_line_height; }
      else
	{
	  let ml_row: *mut glyph_row
	    = (if !(*w).current_matrix.is_null () && !(*((*w).current_matrix as *mut glyph_matrix)).rows.is_null ()
	       { MATRIX_MODE_LINE_ROW! ((*w).current_matrix as *mut glyph_matrix) }
	       else { core::ptr::null_mut () });
	  if !ml_row.is_null () && glyph_row_bits! (mode_line_p, ml_row)
	    { height -= (*ml_row).height; }
	  else
	    { height -= estimate_mode_line_height (f, CURRENT_MODE_LINE_ACTIVE_FACE_ID! (w)); }
	}
    }

  if window_wants_tab_line (w)
    {
      if (*w).tab_line_height >= 0
	{ height -= (*w).tab_line_height; }
      else
	{
	  let tl_row: *mut glyph_row
	    = (if !(*w).current_matrix.is_null () && !(*((*w).current_matrix as *mut glyph_matrix)).rows.is_null ()
	       { MATRIX_TAB_LINE_ROW! ((*w).current_matrix as *mut glyph_matrix) }
	       else { core::ptr::null_mut () });
	  if !tl_row.is_null () && glyph_row_bits! (mode_line_p, tl_row)
	    { height -= (*tl_row).height; }
	  else
	    { height -= estimate_mode_line_height (f, CURRENT_TAB_LINE_ACTIVE_FACE_ID! (w)); }
	}
    }

  if window_wants_header_line (w)
    {
      if (*w).header_line_height >= 0
	{ height -= (*w).header_line_height; }
      else
	{
	  let hl_row: *mut glyph_row
	    = (if !(*w).current_matrix.is_null () && !(*((*w).current_matrix as *mut glyph_matrix)).rows.is_null ()
	       { MATRIX_HEADER_LINE_ROW! ((*w).current_matrix as *mut glyph_matrix) }
	       else { core::ptr::null_mut () });
	  if !hl_row.is_null () && glyph_row_bits! (mode_line_p, hl_row)
	    { height -= (*hl_row).height; }
	  else
	    { height -= estimate_mode_line_height (f, CURRENT_HEADER_LINE_ACTIVE_FACE_ID! (w)); }
	}
    }

  /* With a very small font and a mode-line that's taller than
     default, we might end up with a negative height.  */
  return max (0, height as isize) as c_int;
}

/* Return the window-relative coordinate of the left edge of display
   area AREA of window W.  ANY_AREA means return the left edge of the
   whole window, to the right of the left fringe of W.  (C 1377-1412)  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn window_box_left_offset (w: *mut window, area: glyph_row_area) -> c_int
{
  let mut x: c_int;

  if window_bits! (w, pseudo_window_p)
    { return 0; }

  x = WINDOW_LEFT_SCROLL_BAR_AREA_WIDTH! (w);

  if area == TEXT_AREA
    { x += WINDOW_LEFT_FRINGE_WIDTH! (w)
	   + window_box_width (w, LEFT_MARGIN_AREA); }
  else if area == RIGHT_MARGIN_AREA
    { x += WINDOW_LEFT_FRINGE_WIDTH! (w)
	   + window_box_width (w, LEFT_MARGIN_AREA)
	   + window_box_width (w, TEXT_AREA)
	   + (if WINDOW_HAS_FRINGES_OUTSIDE_MARGINS (w)
	      { 0 }
	      else { WINDOW_RIGHT_FRINGE_WIDTH! (w) }); }
  else if area == LEFT_MARGIN_AREA
	   && WINDOW_HAS_FRINGES_OUTSIDE_MARGINS (w)
    { x += WINDOW_LEFT_FRINGE_WIDTH! (w); }

  /* Don't return more than the window's pixel width.  */
  return min (x as isize, (*w).pixel_width as isize) as c_int;
}

/* Return the window-relative coordinate of the right edge of display
   area AREA of window W.  ANY_AREA means return the right edge of the
   whole window, to the left of the right fringe of W.  (C 1414-1426)  */

unsafe fn window_box_right_offset (w: *mut window, area: glyph_row_area) -> c_int
{
  /* Don't return more than the window's pixel width.  */
  return min ((window_box_left_offset (w, area) + window_box_width (w, area)) as isize,
	      (*w).pixel_width as isize) as c_int;
}

/* Return the frame-relative coordinate of the left edge of display
   area AREA of window W.  ANY_AREA means return the left edge of the
   whole window, to the right of the left fringe of W.  (C 1428-1442)  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn window_box_left (w: *mut window, area: glyph_row_area) -> c_int
{
  let f: *mut frame = XFRAME ((*w).frame);
  let x: c_int;

  if window_bits! (w, pseudo_window_p)
    { return FRAME_INTERNAL_BORDER_WIDTH (f); }

  x = WINDOW_LEFT_EDGE_X! (w)
      + window_box_left_offset (w, area);

  return x;
}

/* Return the frame-relative coordinate of the right edge of display
   area AREA of window W.  ANY_AREA means return the right edge of the
   whole window, to the left of the right fringe of W.  (C 1444-1453)  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn window_box_right (w: *mut window, area: glyph_row_area) -> c_int
{
  return window_box_left (w, area) + window_box_width (w, area);
}

/* Get the bounding box of the display area AREA of window W, without
   mode lines, in frame-relative coordinates.  ANY_AREA means the
   whole window, not including the left and right fringes of
   the window.  Return in *BOX_X and *BOX_Y the frame-relative pixel
   coordinates of the upper-left corner of the box.  Return in
   *BOX_WIDTH, and *BOX_HEIGHT the pixel width and height of the box.  (C 1455-1478)  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn window_box (w: *mut window, area: glyph_row_area, box_x: *mut c_int,
				     box_y: *mut c_int, box_width: *mut c_int,
				     box_height: *mut c_int)
{
  if !box_width.is_null ()
    { *box_width = window_box_width (w, area); }
  if !box_height.is_null ()
    { *box_height = window_box_height (w); }
  if !box_x.is_null ()
    { *box_x = window_box_left (w, area); }
  if !box_y.is_null ()
    {
      *box_y = WINDOW_TOP_EDGE_Y! (w);
      if window_wants_tab_line (w)
	{ *box_y += CURRENT_TAB_LINE_HEIGHT! (w); }
      if window_wants_header_line (w)
	{ *box_y += CURRENT_HEADER_LINE_HEIGHT! (w); }
    }
}

/* C 1480-1502's `window_box_edges` is inside `#ifdef HAVE_WINDOW_SYSTEM`, dead on this
   target (P-08); recorded in docs/ports/xdisp.md.  */

/***********************************************************************
			      Utilities
 ***********************************************************************/

/* Return the bottom y-position of the line the iterator IT is in.
   This can modify IT's settings.  (C 1504-1541)  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn line_bottom_y (it: *mut it) -> c_int
{
  let mut line_height: c_int = (*it).max_ascent + (*it).max_descent;
  let line_top_y: c_int = (*it).current_y;

  if line_height == 0
    {
      if last_height != 0
	{ line_height = last_height; }
      else if IT_CHARPOS! (*it) < ZV! ()
	{
	  move_it_by_lines (it, 1);
	  line_height = (if (*it).max_ascent != 0 || (*it).max_descent != 0
			 { (*it).max_ascent + (*it).max_descent }
			 else { last_height });
	}
      else
	{
	  let row: *mut glyph_row = (*it).glyph_row;

	  /* Use the default character height.  */
	  (*it).glyph_row = core::ptr::null_mut ();
	  (*it).what = IT_CHARACTER;
	  (*it).c = ' ' as c_int;
	  (*it).len = 1;
	  PRODUCE_GLYPHS! (it);
	  line_height = (*it).ascent + (*it).descent;
	  (*it).glyph_row = row;
	}
    }

  return line_top_y + line_height;
}

DEFUN! ("line-pixel-height", Fline_pixel_height,
       Sline_pixel_height, 0, 0, 0,
       doc: "Return height in pixels of text line in the selected window.\n\nValue is the height in pixels of the line at point.  ",
       (),
       {
  /* C's `struct it it;` is uninitialised and `start_display` fills it in; Rust needs a
     value to take its address (PORTING.md 1.5).  */
  let mut it: it = unsafe { core::mem::zeroed () };
  let mut pt: text_pos = unsafe { core::mem::zeroed () };
  let w: *mut window = XWINDOW (selected_window);
  let mut old_buffer: *mut buffer = core::ptr::null_mut ();
  let result: Lisp_Object;

  if XBUFFER ((*w).contents) != current_buffer
    {
      old_buffer = current_buffer;
      set_buffer_internal_1 (XBUFFER ((*w).contents));
    }
  SET_TEXT_POS! (pt, PT! (), PT_BYTE! ());
  let mut itdata: *mut c_void = bidi_shelve_cache ();
  start_display (&raw mut it, w, pt);
  /* Start from the beginning of the screen line, to make sure we
     traverse all of its display elements, and thus capture the
     correct metrics.  */
  move_it_by_lines (&raw mut it, 0);
  it.current_y = 0;
  it.vpos = 0;
  last_height = 0;
  result = make_fixnum (line_bottom_y (&raw mut it) as EMACS_INT);
  if !old_buffer.is_null ()
    { set_buffer_internal_1 (old_buffer); }

  bidi_unshelve_cache (itdata, false);
  return result;
});

/* Return the default pixel height of text lines in window W.  The
   value is the canonical height of the W frame's default font, plus
   any extra space required by the line-spacing variable or frame
   parameter.

   Implementation note: this ignores any line-spacing text properties
   put on the newline characters.  This is because those properties
   only affect the _screen_ line ending in the newline (i.e., in a
   continued line, only the last screen line will be affected), which
   means only a small number of lines in a buffer can ever use this
   feature.  Since this function is used to compute the default pixel
   equivalent of text lines in a window, we can safely ignore those
   few lines.  For the same reasons, we ignore the line-height
   properties.  (C 1560-1592)  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn default_line_pixel_height (w: *mut window) -> c_int
{
  let f: *mut frame = WINDOW_XFRAME! (w);
  let mut height: c_int = FRAME_LINE_HEIGHT (f);

  if !FRAME_INITIAL_P (f) && BUFFERP ((*w).contents)
    {
      let b: *mut buffer = XBUFFER ((*w).contents);
      let mut val: Lisp_Object = BVAR! (b, extra_line_spacing_);

      if NILP (val)
	{ val = BVAR! (&raw mut buffer_defaults, extra_line_spacing_); }
      if !NILP (val)
	{
	  if RANGED_FIXNUMP (0, val, INT_MAX as EMACS_INT)
	    { height += XFIXNAT (val) as c_int; }
	  else if FLOATP (val)
	    {
	      let addon: c_int = (XFLOAT_DATA (val) * height as f64 + 0.5) as c_int;

	      if addon >= 0
		{ height += addon; }
	    }
	}
      else
	{ height += (*f).extra_line_spacing; }
    }

  return height;
}

/* Subroutine of pos_visible_p below.  Extracts a display string, if
   any, from the display spec given as its argument.  (C 1594-1613)  */

unsafe fn string_from_display_spec (spec: Lisp_Object) -> Lisp_Object
{
  if VECTORP (spec)
    {
      let mut i: ptrdiff_t = 0;
      while i < ASIZE (spec)
	{
	  if STRINGP (AREF (spec, i))
	    { return AREF (spec, i); }
	  i += 1;
	}
    }
  else
    {
      let mut spec = spec;
      while CONSP (spec)
	{
	  if STRINGP (XCAR (spec))
	    { return XCAR (spec); }
	  spec = XCDR (spec);
	}
    }
  return spec;
}

/* Limit insanely large values of W->hscroll on frame F to the largest
   value that will still prevent first_visible_x and last_visible_x of
   'struct it' from overflowing an int.  (C 1615-1630)  */

unsafe fn window_hscroll_limited (w: *mut window, f: *mut frame) -> c_int
{
  let mut window_hscroll: ptrdiff_t = (*w).hscroll;
  let window_text_width: c_int = window_box_width (w, TEXT_AREA);
  let colwidth: c_int = FRAME_COLUMN_WIDTH (f);

  if window_hscroll > ((INT_MAX - window_text_width) / colwidth - 1) as ptrdiff_t
    { window_hscroll = ((INT_MAX - window_text_width) / colwidth - 1) as ptrdiff_t; }

  /* C returns a `ptrdiff_t` through an `int` return type; the narrowing cast is the one
     C's implicit conversion performs (docs/ports/xdisp.md).  */
  return window_hscroll as c_int;
}

/* Reset the box-face start and end flags in the iterator.  This is
   called after producing glyphs, such that we reset these flags only
   after producing a glyph with the flag set.  (C 1632-1652)  */

unsafe fn reset_box_start_end_flags (it: *mut it)
{
  /* Don't reset if we've drawn the glyph in the display margins --
     those don't count as "produced glyphs".  */
  if (*it).area == TEXT_AREA
      /* Don't reset if we displayed a fringe bitmap.  */
      && !((*it).what == IT_IMAGE && (*it).image_id < 0)
    {
      /* Don't reset if the face is not a box face: that might mean we
	 are iterating some overlay or display string, and the first
	 character to have the box face is yet to be seen, when we pop
	 the iterator stack. */
      if it_bits! (face_box_p, it)
	{ it_bits! (start_of_box_run_p, it, false); }
      it_bits! (end_of_box_run_p, it, false);
    }
}

/* Return true if position CHARPOS is visible in window W.
   CHARPOS < 0 means return info about WINDOW_END position.
   If visible, set *X and *Y to pixel coordinates of top left corner.
   Set *RTOP and *RBOT to pixel height of an invisible area of glyph at POS.
   Set *ROWH and *VPOS to row's visible height and VPOS (row number).  (C 1686-2150)  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pos_visible_p (w: *mut window, charpos: ptrdiff_t, x: *mut c_int,
	       y: *mut c_int, rtop: *mut c_int, rbot: *mut c_int,
	       rowh: *mut c_int, vpos: *mut c_int) -> bool
{
  /* C's `struct it it;` is uninitialised (C leaves it to `start_display`); Rust needs a
     value to take its address (PORTING.md 1.5).  */
  let mut it: it = std::mem::zeroed ();
  let mut itdata: *mut c_void = bidi_shelve_cache ();
  let mut top: text_pos = std::mem::zeroed ();
  let mut visible_p: bool = false;
  let mut old_buffer: *mut buffer = core::ptr::null_mut ();
  let mut r2l: bool = false;

  if FRAME_INITIAL_P (XFRAME (WINDOW_FRAME! (w)))
    { return visible_p; }

  if XBUFFER ((*w).contents) != current_buffer
    {
      old_buffer = current_buffer;
      set_buffer_internal_1 (XBUFFER ((*w).contents));
    }

  SET_TEXT_POS_FROM_MARKER! (top, (*w).start);
  /* Scrolling a minibuffer window via scroll bar when the echo area
     shows long text sometimes resets the minibuffer contents behind
     our backs.  Also, someone might narrow-to-region and immediately
     call a scroll function.  */
  if CHARPOS! (top) > ZV! () || CHARPOS! (top) < BEGV! ()
    { SET_TEXT_POS! (top, BEGV! (), BEGV_BYTE! ()); }

  /* If the top of the window is after CHARPOS, the latter is surely
     not visible.  */
  if charpos >= 0 && CHARPOS! (top) > charpos
    { return visible_p; }

  /* Some Lisp hook could call us in the middle of redisplaying this
     very window.  If, by some bad luck, we are retrying redisplay
     because we found that the mode-line height and/or tab/header-line
     height needs to be updated, the assignment of mode_line_height
     and header_line_height below could disrupt that, due to the
     selected/nonselected window dance during mode-line display, and
     we could infloop.  Avoid that.  */
  let mut prev_mode_line_height: c_int = (*w).mode_line_height;
  let mut prev_header_line_height: c_int = (*w).header_line_height;
  let mut prev_tab_line_height: c_int = (*w).tab_line_height;
  /* Compute exact mode line heights.  */
  if window_wants_mode_line (w)
    {
      let window_mode_line_format: Lisp_Object
	= window_parameter (w, Qmode_line_format ());

      (*w).mode_line_height
	= display_mode_line (w, CURRENT_MODE_LINE_ACTIVE_FACE_ID! (w),
			     if NILP (window_mode_line_format)
			     { BVAR! (current_buffer, mode_line_format_) }
			     else { window_mode_line_format });
    }

  if window_wants_tab_line (w)
    {
      let window_tab_line_format: Lisp_Object
	= window_parameter (w, Qtab_line_format ());

      (*w).tab_line_height
	= display_mode_line (w, CURRENT_TAB_LINE_ACTIVE_FACE_ID! (w),
			     if NILP (window_tab_line_format)
			     { BVAR! (current_buffer, tab_line_format_) }
			     else { window_tab_line_format });
    }

  if window_wants_header_line (w)
    {
      let window_header_line_format: Lisp_Object
	= window_parameter (w, Qheader_line_format ());

      (*w).header_line_height
	= display_mode_line (w, CURRENT_HEADER_LINE_ACTIVE_FACE_ID! (w),
			     if NILP (window_header_line_format)
			     { BVAR! (current_buffer, header_line_format_) }
			     else { window_header_line_format });
    }

  start_display (&raw mut it, w, top);
  move_it_to (&raw mut it, charpos, -1, it.last_visible_y - 1, -1,
	      (if charpos >= 0 { MOVE_TO_POS } else { 0 }) | MOVE_TO_Y);

  /* Adjust for line numbers, if CHARPOS is at or beyond first_visible_x,
     but we didn't yet produce the line-number glyphs.  */
  if !NILP (Vdisplay_line_numbers)
      && it.current_x >= it.first_visible_x
      && IT_CHARPOS! (it) == charpos
      && !it_bits! (line_number_produced_p, &raw mut it)
    {
      /* If the pixel width of line numbers was not yet known, compute
	 it now.  This usually happens in the first display line of a
	 window.  */
      if it.lnum_pixel_width == 0
	{
	  let mut it2: it = std::mem::zeroed ();
	  let mut it2data: *mut c_void = core::ptr::null_mut ();

	  SAVE_IT! (it2, it, it2data);
	  move_it_by_lines (&raw mut it, 1);
	  it2.lnum_pixel_width = it.lnum_pixel_width;
	  RESTORE_IT! (&raw mut it, &raw mut it2, it2data);
	}
      it.current_x += it.lnum_pixel_width;
    }

  if charpos >= 0
      && (((!it_bits! (bidi_p, &raw mut it) || it.bidi_it.scan_dir != -1)
	   && IT_CHARPOS! (it) >= charpos)
	  /* When scanning backwards under bidi iteration, move_it_to
	     stops at or _before_ CHARPOS, because it stops at or to
	     the _right_ of the character at CHARPOS.  */
	  || (it_bits! (bidi_p, &raw mut it) && it.bidi_it.scan_dir == -1
	      && IT_CHARPOS! (it) <= charpos))
    {
      /* We have reached CHARPOS, or passed it.  How the call to
	 move_it_to can overshoot: (i) If CHARPOS is on invisible text
	 or covered by a display property, move_it_to stops at the end
	 of the invisible text, to the right of CHARPOS.  (ii) If
	 CHARPOS is in a display vector, move_it_to stops on its last
	 glyph.  */
      let mut top_x: c_int = it.current_x;
      let mut top_y: c_int = it.current_y;
      let window_top_y: c_int = WINDOW_TAB_LINE_HEIGHT! (w) + WINDOW_HEADER_LINE_HEIGHT! (w);
      let mut bottom_y: c_int = 0;
      let mut save_it: it = std::mem::zeroed ();
      let mut save_it_data: *mut c_void = core::ptr::null_mut ();

      /* Calling line_bottom_y may change it.method, it.position, etc.  */
      SAVE_IT! (save_it, it, save_it_data);
      last_height = 0;
      bottom_y = line_bottom_y (&raw mut it);
      if top_y < window_top_y
	{ visible_p = bottom_y > window_top_y; }
      else if top_y < it.last_visible_y
	{ visible_p = true; }
      if bottom_y >= it.last_visible_y
	  && it_bits! (bidi_p, &raw mut it) && it.bidi_it.scan_dir == -1
	  && IT_CHARPOS! (it) < charpos
	{
	  /* When the last line of the window is scanned backwards
	     under bidi iteration, we could be duped into thinking
	     that we have passed CHARPOS, when in fact move_it_to
	     simply stopped short of CHARPOS because it reached
	     last_visible_y.  To see if that's what happened, we call
	     move_it_to again with a slightly larger vertical limit,
	     and see if it actually moved vertically; if it did, we
	     didn't really reach CHARPOS, which is beyond window end.  */
	  /* Why 10? because we don't know how many canonical lines
	     will the height of the next line(s) be.  So we guess.  */
	  let ten_more_lines: c_int = 10 * default_line_pixel_height (w);

	  move_it_to (&raw mut it, charpos, -1, bottom_y + ten_more_lines, -1,
		      MOVE_TO_POS | MOVE_TO_Y);
	  if it.current_y > top_y
	    { visible_p = false; }

	}
      RESTORE_IT! (&raw mut it, &raw mut save_it, save_it_data);
      if visible_p
	{
	  if it.method == GET_FROM_DISPLAY_VECTOR
	    {
	      /* We stopped on the last glyph of a display vector.
		 Try and recompute.  Hack alert!  */
	      if charpos < 2 || CHARPOS! (top) >= charpos
		{ top_x = (*it.glyph_row).x; }
	      else
		{
		  let mut it2: it = std::mem::zeroed ();
		  let mut it2_prev: it = std::mem::zeroed ();
		  /* The idea is to get to the previous buffer
		     position, consume the character there, and use
		     the pixel coordinates we get after that.  But if
		     the previous buffer position is also displayed
		     from a display vector, we need to consume all of
		     the glyphs from that display vector.  */
		  start_display (&raw mut it2, w, top);
		  it2.glyph_row = core::ptr::null_mut ();
		  move_it_to (&raw mut it2, charpos - 1, -1, -1, -1, MOVE_TO_POS);
		  /* If we didn't get to CHARPOS - 1, there's some
		     replacing display property at that position, and
		     we stopped after it.  That is exactly the place
		     whose coordinates we want.  */
		  if IT_CHARPOS! (it2) != charpos - 1
		    { it2_prev = it2; }
		  else
		    {
		      /* Iterate until we get out of the display
			 vector that displays the character at
			 CHARPOS - 1.  */
		      loop {
			get_next_display_element (&raw mut it2);
			PRODUCE_GLYPHS! (&raw mut it2);
			it2_prev = it2;
			set_iterator_to_next (&raw mut it2, true);
			if !(it2.method == GET_FROM_DISPLAY_VECTOR
			       && IT_CHARPOS! (it2) < charpos) { break; }
		      }
		    }
		  if ITERATOR_AT_END_OF_LINE_P! (&raw mut it2_prev)
		      || it2_prev.current_x > it2_prev.last_visible_x
		    { top_x = (*it.glyph_row).x; }
		  else
		    {
		      top_x = it2_prev.current_x;
		      top_y = it2_prev.current_y;
		    }
		}
	    }
	  else if IT_CHARPOS! (it) != charpos
	    {
	      let cpos: Lisp_Object = make_fixnum (charpos);
	      let spec: Lisp_Object = Fget_char_property (cpos, Qdisplay (), Qnil ());
	      let string: Lisp_Object = string_from_display_spec (spec);
	      let mut tpos: text_pos = std::mem::zeroed ();
	      let newline_in_string: bool
		= (STRINGP (string)
		   && !memchr (SDATA (string) as *const c_void, b'\n' as c_int,
			       SBYTES (string) as usize).is_null ());

	      SET_TEXT_POS! (tpos, charpos, CHAR_TO_BYTE (charpos));
	      let replacing_spec_p: bool
		= (!NILP (spec)
		   && handle_display_spec (core::ptr::null_mut (), spec, Qnil (), Qnil (),
					   &raw mut tpos, charpos,
					   FRAME_WINDOW_P (it.f)) != 0);
	      /* The tricky code below is needed because there's a
		 discrepancy between move_it_to and how we set cursor
		 when PT is at the beginning of a portion of text
		 covered by a display property or an overlay with a
		 display property, or the display line ends in a
		 newline from a display string.  move_it_to will stop
		 _after_ such display strings, whereas
		 set_cursor_from_row conspires with cursor_row_p to
		 place the cursor on the first glyph produced from the
		 display string.  */

	      /* We have overshoot PT because it is covered by a
		 display property that replaces the text it covers.
		 If the string includes embedded newlines, we are also
		 in the wrong display line.  Backtrack to the correct
		 line, where the display property begins.  */
	      if replacing_spec_p
		{
		  let mut startpos: Lisp_Object;
		  let mut endpos: Lisp_Object;
		  let mut start: EMACS_INT;
		  let mut end: EMACS_INT;
		  let mut it3: it = std::mem::zeroed ();

		  /* Find the first and the last buffer positions
		     covered by the display string.  */
		  endpos
		    = Fnext_single_char_property_change (cpos, Qdisplay (), Qnil (),
							 Qnil ());
		  startpos
		    = Fprevious_single_char_property_change (endpos, Qdisplay (),
							     Qnil (), Qnil ());
		  start = XFIXNAT (startpos);
		  end = XFIXNAT (endpos);
		  /* Move to the last buffer position before the
		     display property.  */
		  start_display (&raw mut it3, w, top);
		  if start > CHARPOS! (top)
		    { move_it_to (&raw mut it3, start - 1, -1, -1, -1, MOVE_TO_POS); }
		  /* Move forward one more line if the position before
		     the display string is a newline or if it is the
		     rightmost character on a line that is
		     continued or word-wrapped.  */
		  if it3.method == GET_FROM_BUFFER
		      && (it3.c == '\n' as c_int
			  || FETCH_BYTE! (IT_BYTEPOS! (it3)) == b'\n')
		    { move_it_by_lines (&raw mut it3, 1); }
		  else if move_it_in_display_line_to (&raw mut it3, -1,
						      it3.current_x
						      + it3.pixel_width,
						      MOVE_TO_X)
			   == MOVE_LINE_CONTINUED
		    {
		      move_it_by_lines (&raw mut it3, 1);
		      /* When we are under word-wrap, the #$@%!
			 move_it_by_lines moves 2 lines, so we need to
			 fix that up.  */
		      if it3.line_wrap == WORD_WRAP
			{ move_it_by_lines (&raw mut it3, -1); }
		    }

		  /* Record the vertical coordinate of the display
		     line where we wound up.  */
		  top_y = it3.current_y;
		  if it_bits! (bidi_p, &raw mut it3)
		    {
		      /* When characters are reordered for display,
			 the character displayed to the left of the
			 display string could be _after_ the display
			 property in the logical order.  Use the
			 smallest vertical position of these two.  */
		      start_display (&raw mut it3, w, top);
		      move_it_to (&raw mut it3, end + 1, -1, -1, -1, MOVE_TO_POS);
		      if it3.current_y < top_y
			{ top_y = it3.current_y; }
		    }
		  /* Move from the top of the window to the beginning
		     of the display line where the display string
		     begins.  */
		  start_display (&raw mut it3, w, top);
		  it3.glyph_row = core::ptr::null_mut ();
		  move_it_to (&raw mut it3, -1, 0, top_y, -1, MOVE_TO_X | MOVE_TO_Y);
		  /* If it3_moved stays false after the 'while' loop
		     below, that means we already were at a newline
		     before the loop (e.g., the display string begins
		     with a newline), so we don't need to return to
		     the last position before the display string,
		     because PRODUCE_GLYPHS will not produce anything
		     for a newline.  */
		  let mut it3_moved: bool = false;
		  let mut top_x_before_string: c_int = it3.current_x;
		  /* Finally, advance the iterator until we hit the
		     first display element whose character position is
		     at or beyond CHARPOS, or until the first newline
		     from the display string, which signals the end of
		     the display line.  */
		  while get_next_display_element (&raw mut it3)
		    {
		      if !EQ (it3.object, string)
			{ top_x_before_string = it3.current_x; }
		      PRODUCE_GLYPHS! (&raw mut it3);
		      if ((it3.bidi_it.scan_dir == 1
			   && IT_CHARPOS! (it3) >= charpos)
			  || (it3.bidi_it.scan_dir == -1
			      && IT_CHARPOS! (it3) <= charpos)
			  || ITERATOR_AT_END_OF_LINE_P! (&raw mut it3))
			{ break; }
		      it3_moved = true;
		      set_iterator_to_next (&raw mut it3, false);
		    }
		  top_x = it3.current_x - it3.pixel_width;
		  /* Account for line-number display, if IT3 still
		     didn't.  This can happen if START - 1 is the
		     first or the last character on its display line.  */
		  if !it_bits! (line_number_produced_p, &raw mut it3)
		    {
		      if it3.lnum_pixel_width > 0
			{
			  top_x += it3.lnum_pixel_width;
			  top_x_before_string += it3.lnum_pixel_width;
			}
		      else if it_bits! (line_number_produced_p, &raw mut it)
			{
			  top_x += it.lnum_pixel_width;
			  top_x_before_string += it3.lnum_pixel_width;
			}
		    }
		  /* Normally, we would exit the above loop because we
		     found the display element whose character
		     position is CHARPOS.  For the contingency that we
		     didn't, and stopped at the first newline from the
		     display string, reset top_x to the coordinate of
		     the rightmost glyph not from the string.  */
		  if it3_moved
		      && newline_in_string
		      && IT_CHARPOS! (it3) != charpos && EQ (it3.object, string)
		    { top_x = top_x_before_string; }
		}
	    }

	  *x = top_x;
	  /* The condition below is a heuristic fix for the situation
	     where move_it_to stops just after finishing the display
	     of a fringe bitmap, which resets it.ascent to zero, and
	     thus causes Y to be offset by it.max_ascent.  */
	  if it.ascent == 0 && it.what == IT_IMAGE
	      && it.method != GET_FROM_IMAGE
	      && it.image_id < 0
	      && it.max_ascent > 0
	    { *y = max (top_y as isize, window_top_y as isize) as c_int; }
	  else
	    { *y = max ((top_y + max (0, (it.max_ascent - it.ascent) as isize) as c_int) as isize,
			window_top_y as isize) as c_int; }
	  *rtop = max (0, (window_top_y - top_y) as isize) as c_int;
	  *rbot = max (0, (bottom_y - it.last_visible_y) as isize) as c_int;
	  *rowh = max (0, (min (bottom_y as isize, it.last_visible_y as isize) as c_int
			   - max (top_y as isize, window_top_y as isize) as c_int) as isize)
		  as c_int;
	  *vpos = it.vpos;
	  if it.bidi_it.paragraph_dir == R2L
	    { r2l = true; }
	}
    }
  else
    {
      /* Either we were asked to provide info about WINDOW_END, or
	 CHARPOS is in the partially visible glyph row at end of
	 window.  */
      let mut it2: it = std::mem::zeroed ();
      let mut it2data: *mut c_void = core::ptr::null_mut ();

      SAVE_IT! (it2, it, it2data);
      if IT_CHARPOS! (it) < ZV! () && FETCH_BYTE! (IT_BYTEPOS! (it)) != b'\n'
	{ move_it_by_lines (&raw mut it, 1); }
      if charpos < IT_CHARPOS! (it)
	  || (it.what == IT_EOB && charpos == IT_CHARPOS! (it))
	{
	  visible_p = true;
	  RESTORE_IT! (&raw mut it2, &raw mut it2, it2data);
	  move_it_to (&raw mut it2, charpos, -1, -1, -1, MOVE_TO_POS);
	  *x = it2.current_x;
	  if it2.ascent == 0 && it2.what == IT_IMAGE
	      && it2.method != GET_FROM_IMAGE
	      && it2.image_id < 0
	      && it2.max_ascent > 0
	    { *y = it2.current_y; }
	  else
	    { *y = it2.current_y + it2.max_ascent - it2.ascent; }
	  *rtop = max (0, -it2.current_y as isize) as c_int;
	  *rbot = max (0, ((it2.current_y + it2.max_ascent + it2.max_descent)
			   - it.last_visible_y) as isize) as c_int;
	  *rowh = max (0, (min ((it2.current_y + it2.max_ascent
				 + it2.max_descent) as isize,
				it.last_visible_y as isize) as c_int
			   - max (max (it2.current_y as isize,
				       WINDOW_TAB_LINE_HEIGHT! (w) as isize),
				  WINDOW_HEADER_LINE_HEIGHT! (w) as isize) as c_int)
		  as isize) as c_int;
	  *vpos = it2.vpos;
	  if it2.bidi_it.paragraph_dir == R2L
	    { r2l = true; }
	}
      else
	{ bidi_unshelve_cache (it2data, true); }
    }
  bidi_unshelve_cache (itdata, false);

  if !old_buffer.is_null ()
    { set_buffer_internal_1 (old_buffer); }

  if visible_p
    {
      if (*w).hscroll > 0
	{ *x -= window_hscroll_limited (w, WINDOW_XFRAME! (w))
	  * WINDOW_FRAME_COLUMN_WIDTH! (w); }
      /* For lines in an R2L paragraph, we need to mirror the X pixel
         coordinate wrt the text area.  For the reasons, see the
         commentary in buffer_posn_from_coords and the explanation of
         the geometry used by the move_it_* functions at the end of
         the large commentary near the beginning of this file.  */
      if r2l
	{ *x = window_box_width (w, TEXT_AREA) - *x - 1; }
    }

  /* C 2135-2142: `#if false` guards a debugging `fprintf` block here (P-08: recorded in
     docs/ports/xdisp.md, not transcribed).  */

  /* Restore potentially overwritten values.  */
  (*w).mode_line_height = prev_mode_line_height;
  (*w).header_line_height = prev_header_line_height;
  (*w).tab_line_height = prev_tab_line_height;

  return visible_p;
}


/* Return the next character from STR.  Return in *LEN the length of
   the character.  This is like string_char_and_length but never
   returns an invalid character.  If we find one, we return a `?', but
   with the length of the invalid character.  (C 2153-2172)  */

unsafe fn check_char_and_length (str: *const u8, len: *mut c_int) -> c_int
{
  let mut c: c_int = string_char_and_length (str, len);
  if !CHAR_VALID_P (c as intmax_t)
    /* We may not change the length here because other places in Emacs
       don't use this function, i.e. they silently accept invalid
       characters.  */
    { c = '?' as c_int; }

  return c;
}



/* Given a position POS containing a valid character and byte position
   in STRING, return the position NCHARS ahead (NCHARS >= 0).  (C 2173-2197)  */

unsafe fn string_pos_nchars_ahead (mut pos: text_pos, string: Lisp_Object,
				   mut nchars: ptrdiff_t) -> text_pos
{
  eassert! (STRINGP (string) && nchars >= 0);

  if STRING_MULTIBYTE (string)
    {
      let mut p: *const u8 = (SDATA (string) as *const u8).offset (BYTEPOS! (pos));

      while nchars != 0
	{
	  nchars -= 1;
	  let len: c_int = BYTES_BY_CHAR_HEAD (*p as c_int);
	  p = p.offset (len as isize);
	  CHARPOS! (pos) += 1;
	  BYTEPOS! (pos) += len as ptrdiff_t;
	}
    }
  else
    { SET_TEXT_POS! (pos, CHARPOS! (pos) + nchars, BYTEPOS! (pos) + nchars); }

  return pos;
}


/* Value is the text position, i.e. character and byte position,
   for character position CHARPOS in STRING.  (C 2200-2211)  */

unsafe fn string_pos (charpos: ptrdiff_t, string: Lisp_Object) -> text_pos
{
  let mut pos: text_pos = std::mem::zeroed ();
  eassert! (STRINGP (string));
  eassert! (charpos >= 0);
  SET_TEXT_POS! (pos, charpos, string_char_to_byte (string, charpos));
  return pos;
}


/* Value is a text position, i.e. character and byte position, for
   character position CHARPOS in C string S.  MULTIBYTE_P
   means recognize multibyte characters.  (C 2214-2241)  */

unsafe fn c_string_pos (mut charpos: ptrdiff_t, mut s: *const c_char,
			multibyte_p: bool) -> text_pos
{
  let mut pos: text_pos = std::mem::zeroed ();

  eassert! (!s.is_null ());
  eassert! (charpos >= 0);

  if multibyte_p
    {
      SET_TEXT_POS! (pos, 0, 0);
      while charpos != 0
	{
	  charpos -= 1;
	  let len: c_int = BYTES_BY_CHAR_HEAD (*s as c_int);
	  s = s.offset (len as isize);
	  CHARPOS! (pos) += 1;
	  BYTEPOS! (pos) += len as ptrdiff_t;
	}
    }
  else
    { SET_TEXT_POS! (pos, charpos, charpos); }

  return pos;
}


/* Value is the number of characters in C string S.  MULTIBYTE_P
   means recognize multibyte characters.  (C 2244-2267)  */

unsafe fn number_of_chars (mut s: *const c_char, multibyte_p: bool) -> ptrdiff_t
{
  let mut nchars: ptrdiff_t = 0;

  if multibyte_p
    {
      let mut rest: ptrdiff_t = strlen (s) as ptrdiff_t;
      let mut p: *const u8 = s as *const u8;

      nchars = 0;
      while rest > 0
	{
	  let len: c_int = BYTES_BY_CHAR_HEAD (*p as c_int);
	  rest -= len as ptrdiff_t;
	  p = p.offset (len as isize);
	  nchars += 1;
	}
    }
  else
    { nchars = strlen (s) as ptrdiff_t; }

  return nchars;
}


/* Compute byte position NEWPOS->bytepos corresponding to
   NEWPOS->charpos.  POS is a known position in string STRING.
   NEWPOS->charpos must be >= POS.charpos.  (C 2270-2285)  */

unsafe fn compute_string_pos (newpos: *mut text_pos, pos: text_pos, string: Lisp_Object)
{
  eassert! (STRINGP (string));
  eassert! (CHARPOS! (*newpos) >= CHARPOS! (pos));

  if STRING_MULTIBYTE (string)
    {
      *newpos = string_pos_nchars_ahead (pos, string,
					 CHARPOS! (*newpos) - CHARPOS! (pos));
    }
  else
    { BYTEPOS! (*newpos) = CHARPOS! (*newpos); }
}


/* EXPORT:
   Return an estimation of the pixel height of mode or header lines on
   frame F.  FACE_ID specifies what line's height to estimate.  (C 2287-2318)  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn estimate_mode_line_height (f: *mut frame, face_id: face_id) -> c_int
{
  /* C 2294-2315's `HAVE_WINDOW_SYSTEM` arm (the `FRAME_WINDOW_P (f)` height computation) is
     dead on this target (P-08); recorded in docs/ports/xdisp.md.  */

  return 1;
}


/* Given a pixel position (PIX_X, PIX_Y) on frame F, return glyph
   coordinates in (*X, *Y).  Set *BOUNDS to the rectangle that the
   glyph at X, Y occupies, if BOUNDS != 0.  If NOCLIP, do
   not force the value into range.  (C 2320-2370)  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pixel_to_glyph_coords (f: *mut frame, mut pix_x: c_int, mut pix_y: c_int,
					       x: *mut c_int, y: *mut c_int,
					       bounds: *mut c_int, noclip: bool)
{

  /* C 2330-2366's `HAVE_WINDOW_SYSTEM` arm (the `FRAME_WINDOW_P (f)` column/line conversion
     and the `bounds` rectangle) is dead on this target (P-08); recorded in
     docs/ports/xdisp.md.  */

  *x = pix_x;
  *y = pix_y;
}


/* Find the glyph under window-relative coordinates X/Y in window W.
   Consider only glyphs from buffer text, i.e. no glyphs from overlay
   strings.  Return in *HPOS and *VPOS the row and column number of
   the glyph found.  Return in *AREA the glyph area containing X.
   Value is a pointer to the glyph found or null if X/Y is not on
   text, or we can't tell because W's current matrix is not up to
   date.  (C 2372-2450)  */

unsafe fn x_y_to_hpos_vpos (w: *mut window, mut x: c_int, y: c_int, hpos: *mut c_int,
			    vpos: *mut c_int, dx: *mut c_int, dy: *mut c_int,
			    area: *mut c_int) -> *mut glyph
{
  let mut glyph: *mut glyph = core::ptr::null_mut ();
  let mut end: *mut glyph = core::ptr::null_mut ();
  let mut row: *mut glyph_row = core::ptr::null_mut ();
  let mut x0: c_int = 0;
  let mut i: c_int = 0;

  /* Find row containing Y.  Give up if some row is not enabled.  */
  i = 0;
  while i < (*( (*w).current_matrix as *mut glyph_matrix )).nrows
    {
      row = MATRIX_ROW! ((*w).current_matrix as *mut glyph_matrix, i);
      if !glyph_row_bits! (enabled_p, row)
	{ return core::ptr::null_mut (); }
      if y >= (*row).y && y < MATRIX_ROW_BOTTOM_Y! (row)
	{ break; }
      i += 1;
    }

  *vpos = i;
  *hpos = 0;

  /* Give up if Y is not in the window.  */
  if i == (*( (*w).current_matrix as *mut glyph_matrix )).nrows
    { return core::ptr::null_mut (); }

  /* Get the glyph area containing X.  */
  if window_bits! (w, pseudo_window_p)
    {
      *area = TEXT_AREA;
      x0 = 0;
    }
  else
    {
      if x < window_box_left_offset (w, TEXT_AREA)
	{
	  *area = LEFT_MARGIN_AREA;
	  x0 = window_box_left_offset (w, LEFT_MARGIN_AREA);
	}
      else if x < window_box_right_offset (w, TEXT_AREA)
	{
	  *area = TEXT_AREA;
	  x0 = window_box_left_offset (w, TEXT_AREA) + min ((*row).x as isize, 0) as c_int;
	}
      else
	{
	  *area = RIGHT_MARGIN_AREA;
	  x0 = window_box_left_offset (w, RIGHT_MARGIN_AREA);
	}
    }

  /* Find glyph containing X.  */
  glyph = (*row).glyphs [*area as usize];
  end = glyph.offset ((*row).used [*area as usize] as isize);
  x -= x0;
  while glyph < end && x >= (*glyph).pixel_width as c_int
    {
      x -= (*glyph).pixel_width as c_int;
      glyph = glyph.offset (1);
    }

  if glyph == end
    { return core::ptr::null_mut (); }

  if !dx.is_null ()
    {
      *dx = x;
      *dy = y - ((*row).y + (*row).ascent - (*glyph).ascent as c_int);
    }

  *hpos = glyph.offset_from ((*row).glyphs [*area as usize]) as c_int;
  return glyph;
}

/* C 2452-2465: `x_y_to_column_row` sits under
   `#if defined HAVE_WINDOW_SYSTEM && !defined HAVE_EXT_MENU_BAR`, dead on this target (P-08);
   recorded in docs/ports/xdisp.md.  */

/* Convert frame-relative x/y to coordinates relative to window W.
   Takes pseudo-windows into account.  (C 2467-2486)  */

unsafe fn frame_to_window_pixel_xy (w: *mut window, x: *mut c_int, y: *mut c_int)
{
  if window_bits! (w, pseudo_window_p)
    {
      /* A pseudo-window is always full-width, and starts at the
	 left edge of the frame, plus a frame border.  */
      let f: *mut frame = XFRAME ((*w).frame);
      *x -= FRAME_INTERNAL_BORDER_WIDTH (f);
      *y = FRAME_TO_WINDOW_PIXEL_Y! (w, *y);
    }
  else
    {
      *x -= WINDOW_LEFT_EDGE_X! (w);
      *y = FRAME_TO_WINDOW_PIXEL_Y! (w, *y);
    }
}

unsafe fn adjust_window_ends (w: *mut window, row: *mut glyph_row, current: bool)
{
  eassert! (!w.is_null ());
  (*w).window_end_pos = Z! () - MATRIX_ROW_END_CHARPOS! (row);
  (*w).window_end_bytepos = Z_BYTE! () - MATRIX_ROW_END_BYTEPOS! (row);
  (*w).window_end_vpos
    = MATRIX_ROW_VPOS! (row, if current
			     { (*w).current_matrix as *mut glyph_matrix }
			     else { (*w).desired_matrix as *mut glyph_matrix }) as c_int;
}

unsafe fn hscrolling_current_line_p (w: *mut window) -> bool
{
  return (!window_bits! (w, suspend_auto_hscroll)
	  && EQ (Fbuffer_local_value (Qauto_hscroll_mode (), (*w).contents),
		 Qcurrent_line ()));
}

/***********************************************************************
			Lisp form evaluation
 ***********************************************************************/

/* The redisplay is not re-entrant.  But we can run Elisp code during
   redisplay, which in turn can trigger redisplay.
   We try to make this inner redisplay work correctly, but it messes up
   the state of the outer redisplay, so when we return to this outer
   redisplay, we need to abort it.
   To detect this case, we keep a counter that identifies every call to
   redisplay, so we can detect when a nested redisplay happened by the
   fact that the counter has changed.  (C 3085-3094)  */

#[unsafe(no_mangle)]
pub static mut redisplay_counter: c_uint = 0;

/* Error handler for dsafe_eval and dsafe_call.  (C 3096-3104)  */

/* C's `dsafe_eval_handler` is one of `internal_condition_case_n`'s two function arguments,
   so its address is taken and the port declares it `unsafe extern "C" fn` rather than the
   plain `unsafe fn` a `static` helper otherwise gets (the `redisplay_trace`/`move_trace`
   rule; the previous window declared it before its caller, `dsafe__call`, existed).  */
unsafe extern "C" fn dsafe_eval_handler (arg: Lisp_Object, nargs: ptrdiff_t,
					 args: *mut Lisp_Object) -> Lisp_Object
{
  add_to_log (c"Error during redisplay: %S signaled %S".as_ptr (),
	      Flist (nargs, args), arg);
  return Qnil ();
}

/* Call function FUNC with the rest of NARGS - 1 arguments
   following.  Return the result, or nil if something went
   wrong.  Prevent redisplay during the evaluation.  (C 3106-3108)  */

/* FIXME: What's the guiding principle behind the choice
   of which calls should set 'inhibit_quit' and which don't.  (C 3110-3111)  */
unsafe fn dsafe__call (inhibit_quit: bool,
		       f: unsafe extern "C" fn (ptrdiff_t, *mut Lisp_Object) -> Lisp_Object,
		       nargs: ptrdiff_t, args: *mut Lisp_Object) -> Lisp_Object
{
  let mut val: Lisp_Object;

  if inhibit_eval_during_redisplay
    { val = Qnil (); }
  else
    {
      let count: specpdl_ref = SPECPDL_INDEX ();

      specbind (Qinhibit_redisplay (), Qt ());
      if inhibit_quit
	{ specbind (Qinhibit_quit (), Qt ()); }
      let redisplay_counter_before: c_int = redisplay_counter as c_int;
      /* Use Qt to ensure debugger does not run,
	 to reduce the risk of wanting to redisplay.  */
      val = internal_condition_case_n (f, nargs, args, Qt (),
				       dsafe_eval_handler);
      if redisplay_counter_before != redisplay_counter as c_int
        /* A nested redisplay happened, abort this one!  */
        /* FIXME: Rather than jump all the way to `top-level`
           we should exit only the current redisplay.  */
        { Ftop_level (); }

      val = unbind_to (count, val);
    }

  return val;
}

/* C's `funcall_with_backtraces` is the other half of `dsafe_calln`'s ternary and is passed
   as a function pointer there, so it is `unsafe extern "C" fn` too.  */
unsafe extern "C" fn funcall_with_backtraces (nargs: ptrdiff_t, args: *mut Lisp_Object)
  -> Lisp_Object
{
  /* If an error is signaled during a Lisp hook in redisplay, write a
     backtrace into the buffer *Redisplay-trace*.  (C 3147-3148)  */
  push_handler_bind (list_of_error, Qdebug_early__muted (), 0);
  let res: Lisp_Object = Ffuncall (nargs, args);
  pop_handler ();
  return res;
}

/* C 3155-3156: `#define SAFE_CALLMANY(inhibit_quit, f, array)
   dsafe__call (inhibit_quit, f, countof (array), array)`.  */
macro_rules! SAFE_CALLMANY {
  ($inhibit_quit:expr, $f:expr, $array:expr) => {
    dsafe__call ($inhibit_quit, $f, countof! ($array) as ptrdiff_t,
		$array.as_ptr () as *mut Lisp_Object)
  };
}

/* C 3157-3162: `#define dsafe_calln(inhibit_quit, ...) SAFE_CALLMANY (inhibit_quit,
   backtrace_on_redisplay_error ? funcall_with_backtraces : Ffuncall, ((Lisp_Object [])
   {__VA_ARGS__}))`.  C's compound literal is the port's local array (the lisp.rs `CALLN!`
   shape) and C's ternary is the `if`/`else` below, whose two arms both coerce to C's
   function-pointer type.  */
macro_rules! dsafe_calln {
  ($inhibit_quit:expr, $($args:expr),* $(,)?) => {{
    let arr: [Lisp_Object; 0 $(+ { let _ = stringify!($args); 1 })*] = [$($args),*];
    SAFE_CALLMANY! ($inhibit_quit,
		    if backtrace_on_redisplay_error
		      { funcall_with_backtraces
			  as unsafe extern "C" fn (ptrdiff_t, *mut Lisp_Object) -> Lisp_Object }
		      else
		      { Ffuncall
			  as unsafe extern "C" fn (ptrdiff_t, *mut Lisp_Object) -> Lisp_Object },
		    arr)
  }};
}

unsafe fn dsafe_call1 (f: Lisp_Object, arg: Lisp_Object) -> Lisp_Object
{
  return dsafe_calln! (false, f, arg);
}

unsafe fn dsafe_eval (sexpr: Lisp_Object) -> Lisp_Object
{
  return dsafe_calln! (true, Qeval (), sexpr, Qt ());
}

/***********************************************************************
			      Debugging
 ***********************************************************************/

/* Define CHECK_IT to perform sanity checks on iterators.
   This is for debugging.  It is too slow to do unconditionally.  (C 3180-3183)  */

unsafe fn CHECK_IT (it: *mut it)
{
  /* C 3185-3211 is a single `#if false` block, dead in every configuration (the ledger
     records it), so the frozen reference's body is empty.  The port keeps the parameter
     live with C's own no-op spelling for this situation (`xfaces.rs`'s `clear_face_cache`,
     `dynlib.rs`'s `dynlib_addr`) rather than writing `{}`, which `extract-abi.py` reads
     as a stub.  */
  let _ = it;
}

/* Check that the window end of window W is what we expect it
   to be---the last row in the current matrix displaying text.  (C 3217-3218)  */

unsafe fn CHECK_WINDOW_END (w: *mut window)
{
  /* C 3222-3229 is `#if defined GLYPH_DEBUG && defined ENABLE_CHECKING`; neither macro is
     defined on this target (the ledger records it), so the reference's body is empty; the
     no-op parameter keeps the name from reading as a stub.  */
  let _ = w;
}

/***********************************************************************
		       Iterator initialization
 ***********************************************************************/

/* Initialize IT for displaying current_buffer in window W, starting
   at character position CHARPOS.  CHARPOS < 0 means that no buffer
   position is specified which is useful when the iterator is assigned
   a position later.  BYTEPOS is the byte position corresponding to
   CHARPOS.

   If ROW is not null, calls to produce_glyphs with IT as parameter
   will produce glyphs in that row.

   BASE_FACE_ID is the id of a base face to use.  It must be one of
   DEFAULT_FACE_ID for normal text, MODE_LINE_ACTIVE_FACE_ID,
   MODE_LINE_INACTIVE_FACE_ID, HEADER_LINE_ACTIVE_FACE_ID, or
   HEADER_LINE_INACTIVE_FACE_ID for displaying mode lines, or
   TOOL_BAR_FACE_ID for displaying the tool-bar.

   If ROW is null and BASE_FACE_ID is equal to MODE_LINE_ACTIVE_FACE_ID,
   MODE_LINE_INACTIVE_FACE_ID, HEADER_LINE_ACTIVE_FACE_ID, or
   HEADER_LINE_INACTIVE_FACE_ID the iterator will be initialized to use
   the corresponding mode line glyph row of the desired matrix of W.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_iterator (it: *mut it, w: *mut window,
					charpos: ptrdiff_t, bytepos: ptrdiff_t,
					mut row: *mut glyph_row, base_face_id: face_id)
{
  let mut remapped_base_face_id: face_id = base_face_id;
  let mut body_width: c_int = 0;
  let mut body_height: c_int = 0;

  /* Some precondition checks.  */
  eassert! (w != core::ptr::null_mut () && it != core::ptr::null_mut ());
  eassert! (charpos < 0
	    || (charpos >= BUF_BEG (current_buffer) && charpos <= ZV! ()));

  /* If face attributes have been changed since the last redisplay,
     free realized faces now because they depend on face definitions
     that might have changed.  Don't free faces while there might be
     desired matrices pending which reference these faces.  */
  if !inhibit_free_realized_faces
    {
      if face_change
	{
	  face_change = false;
	  frame_bits! (face_change, XFRAME ((*w).frame), 0);
	  free_all_realized_faces (Qnil ());
	}
      else if frame_bits! (face_change, XFRAME ((*w).frame))
	{
	  frame_bits! (face_change, XFRAME ((*w).frame), 0);
	  free_all_realized_faces ((*w).frame);
	}
    }

  /* Perhaps remap BASE_FACE_ID to a user-specified alternative.  */
  if !NILP (Vface_remapping_alist)
    {
      remapped_base_face_id
	= lookup_basic_face (w, XFRAME ((*w).frame), base_face_id);
    }

  /* Use one of the mode line rows of W's desired matrix if
     appropriate.  */
  if row == core::ptr::null_mut ()
    {
      if base_face_id == MODE_LINE_ACTIVE_FACE_ID
	  || base_face_id == MODE_LINE_INACTIVE_FACE_ID
	{ row = MATRIX_MODE_LINE_ROW! ((*w).desired_matrix as *mut glyph_matrix); }
      else if base_face_id == TAB_LINE_ACTIVE_FACE_ID
	       || base_face_id == TAB_LINE_INACTIVE_FACE_ID
	{ row = MATRIX_TAB_LINE_ROW! ((*w).desired_matrix as *mut glyph_matrix); }
      else if base_face_id == HEADER_LINE_ACTIVE_FACE_ID
	       || base_face_id == HEADER_LINE_INACTIVE_FACE_ID
	{
	  /* Header line row depends on whether tab line is enabled.  */
	  glyph_matrix_bits! (tab_line_p, (*w).desired_matrix as *mut glyph_matrix,
			      window_wants_tab_line (w));
	  row = MATRIX_HEADER_LINE_ROW! ((*w).desired_matrix as *mut glyph_matrix);
	}
    }

  /* Clear IT, and set it->object and other IT's Lisp objects to Qnil.
     Other parts of redisplay rely on that.  */
  memclear (it as *mut c_void, core::mem::size_of::<it> () as isize);
  (*it).current.overlay_string_index = -1;
  (*it).current.dpvec_index = -1;
  (*it).base_face_id = remapped_base_face_id;
  IT_STRING_CHARPOS! (*it) = -1;
  IT_STRING_BYTEPOS! (*it) = -1;
  (*it).paragraph_embedding = L2R;
  (*it).bidi_it.w = w;

  /* The window in which we iterate over current_buffer:  */
  XSETWINDOW! ((*it).window, w);
  (*it).w = w;
  (*it).f = XFRAME ((*w).frame);

  (*it).cmp_it.id = -1;
  (*it).cmp_it.parent_it = it;

  if max_redisplay_ticks > 0
    { update_redisplay_ticks (0, w); }

  /* Extra space between lines (on window systems only).  */
  if base_face_id == DEFAULT_FACE_ID
    && FRAME_WINDOW_P ((*it).f)
    {
      let mut line_space_above: Lisp_Object;
      let mut line_space_below: Lisp_Object;

      if FIXNATP (BVAR! (current_buffer, extra_line_spacing_))
	{
	  (*it).extra_line_spacing =
	    XFIXNAT (BVAR! (current_buffer, extra_line_spacing_)) as c_int;
	  (*it).extra_line_spacing_above = 0;
	}
      else if FLOATP (BVAR! (current_buffer, extra_line_spacing_))
	{
	  (*it).extra_line_spacing
	    = (XFLOAT_DATA (BVAR! (current_buffer, extra_line_spacing_))
	       * FRAME_LINE_HEIGHT ((*it).f) as c_double) as c_int;
	  (*it).extra_line_spacing_above = 0;
	}
      else if CONSP (BVAR! (current_buffer, extra_line_spacing_))
	{
	  line_space_above = XCAR (BVAR! (current_buffer, extra_line_spacing_));
	  line_space_below = XCDR (BVAR! (current_buffer, extra_line_spacing_));
	  /* Integer pair case.	 */
	  if FIXNATP (line_space_above) && FIXNATP (line_space_below)
	    {
	      let line_space_total: c_int
		= (XFIXNAT (line_space_below) + XFIXNAT (line_space_above)) as c_int;
	      (*it).extra_line_spacing = line_space_total;
	      (*it).extra_line_spacing_above = XFIXNAT (line_space_above) as c_int;
	    }
	  /* Float pair case.  */
	  else if FLOATP (line_space_above) && FLOATP (line_space_below)
	    {
	      let line_space_total: c_double
		= XFLOAT_DATA (line_space_above) + XFLOAT_DATA (line_space_below);
	      (*it).extra_line_spacing
		= (line_space_total * FRAME_LINE_HEIGHT ((*it).f) as c_double) as c_int;
	      (*it).extra_line_spacing_above
		= (XFLOAT_DATA (line_space_above)
		   * FRAME_LINE_HEIGHT ((*it).f) as c_double) as c_int;
	    }
	  /* Invalid cons. */
	  else
	    {
	      (*it).extra_line_spacing = 0;
	      (*it).extra_line_spacing_above = 0;
	    }
	}
      else if (*(*it).f).extra_line_spacing > 0
	{
	  (*it).extra_line_spacing	= (*(*it).f).extra_line_spacing;
	  (*it).extra_line_spacing_above = (*(*it).f).extra_line_spacing_above;
	}
    }

  /* If realized faces have been removed, e.g. because of face
     attribute changes of named faces, recompute them.  When running
     in batch mode, the face cache of the initial frame is null.  If
     we happen to get called, make a dummy face cache.  */
  if FRAME_FACE_CACHE! ((*it).f).is_null ()
    { init_frame_faces ((*it).f); }
  if (*FRAME_FACE_CACHE! ((*it).f)).used == 0
    { recompute_basic_faces ((*it).f); }

  (*it).override_ascent = -1;

  /* Are control characters displayed as `^C'?  */
  it_bits! (ctl_arrow_p, it, !NILP (BVAR! (current_buffer, ctl_arrow_)));

  /* -1 means everything between a CR and the following line end
     is invisible.  >0 means lines indented more than this value are
     invisible.  */
  (*it).selective
    = (if FIXNUMP (BVAR! (current_buffer, selective_display_))
       { clip_to_bounds (-1, XFIXNUM (BVAR! (current_buffer, selective_display_)) as isize,
			 PTRDIFF_MAX as isize) as ptrdiff_t }
       else if !NILP (BVAR! (current_buffer, selective_display_))
       { -1 }
       else { 0 });
  it_bits! (selective_display_ellipsis_p, it,
	    !NILP (BVAR! (current_buffer, selective_display_ellipses_)));

  /* Display table to use.  */
  (*it).dp = window_display_table (w);

  /* Are multibyte characters enabled in current_buffer?  */
  it_bits! (multibyte_p, it, !NILP (BVAR! (current_buffer, enable_multibyte_characters_)));

  (*it).tab_width = SANE_TAB_WIDTH (current_buffer) as c_short;

  /* Are lines in the display truncated?  */
  if TRUNCATE != 0
    { (*it).line_wrap = TRUNCATE; }
  if base_face_id == DEFAULT_FACE_ID
    && (*(*it).w).hscroll == 0
    && (WINDOW_FULL_WIDTH_P! ((*it).w)
	|| NILP (Vtruncate_partial_width_windows)
	|| (FIXNUMP (Vtruncate_partial_width_windows)
	    /* PXW: Shall we do something about this?  */
	    && (XFIXNUM (Vtruncate_partial_width_windows)
		<= WINDOW_TOTAL_COLS ((*it).w) as EMACS_INT)))
    && NILP (BVAR! (current_buffer, truncate_lines_))
    {
      (*it).line_wrap = if NILP (BVAR! (current_buffer, word_wrap_))
	{ WINDOW_WRAP } else { WORD_WRAP };
    }

  /* Get dimensions of truncation and continuation glyphs.  These are
     displayed as fringe bitmaps under X, but we need them for such
     frames when the fringes are turned off.  The no_special_glyphs slot
     of the iterator's frame, when set, suppresses their display - by
     default for tooltip frames and when set via the 'no-special-glyphs'
     frame parameter.  */
  if !frame_bits! (no_special_glyphs, (*it).f)
    {
      let mut width: c_int;

      /* For each special glyph, we get the dimensions in the L2R and in
	 the R2L case and use the maximum produced width.  This is
	 because the glyph can come from a different font in each
	 case.  */
      if (*it).line_wrap == TRUNCATE
	{
	  /* We will need the truncation glyph.  */
	  eassert! ((*it).glyph_row == core::ptr::null_mut ());
	  produce_special_glyphs (it, IT_TRUNCATION, L2R, false);
	  width = (*it).pixel_width;
	  produce_special_glyphs (it, IT_TRUNCATION, R2L, false);
	  (*it).truncation_pixel_width
	    = max (width as isize, (*it).pixel_width as isize) as c_short;
	}
      else
	{
	  /* We will need the continuation glyph.  */
	  eassert! ((*it).glyph_row == core::ptr::null_mut ());
	  produce_special_glyphs (it, IT_CONTINUATION, L2R, false);
	  width = (*it).pixel_width;
	  produce_special_glyphs (it, IT_CONTINUATION, R2L, false);
	  (*it).continuation_pixel_width
	    = max (width as isize, (*it).pixel_width as isize) as c_short;
	}
    }

  /* Reset these values to zero because the produce_special_glyphs
     above has changed them.  */
  (*it).pixel_width = 0;
  (*it).ascent = 0;
  (*it).descent = 0;
  (*it).phys_ascent = 0;
  (*it).phys_descent = 0;

  /* Set this after getting the dimensions of truncation and
     continuation glyphs, so that we don't produce glyphs when calling
     produce_special_glyphs, above.  */
  (*it).glyph_row = row;
  (*it).area = TEXT_AREA;

  /* Get the dimensions of the display area.  The display area
     consists of the visible window area plus a horizontally scrolled
     part to the left of the window.  All x-values are relative to the
     start of this total display area.  */
  if base_face_id != DEFAULT_FACE_ID
    {
      /* Mode lines, menu bar in terminal frames.  */
      (*it).first_visible_x = 0;
      (*it).last_visible_x
	= WINDOW_PIXEL_WIDTH (w) - WINDOW_RIGHT_DIVIDER_WIDTH! (w);
    }
  else
    {
      /* When hscrolling only the current line, don't apply the
	 hscroll here, it will be applied by display_line when it gets
	 to laying out the line showing point.  However, if the
	 window's min_hscroll is positive, the user specified a lower
	 bound for automatic hscrolling, so they expect the
	 non-current lines to obey that hscroll amount.  */
      if hscrolling_current_line_p (w)
	{
	  if (*w).min_hscroll > 0
	    {
	      (*it).first_visible_x
		= ((*w).min_hscroll * FRAME_COLUMN_WIDTH ((*it).f) as ptrdiff_t) as c_int;
	    }
	  else
	    { (*it).first_visible_x = 0; }
	}
      else
	{
	  (*it).first_visible_x
	    = (window_hscroll_limited (w, (*it).f) * FRAME_COLUMN_WIDTH ((*it).f)) as c_int;
	}

      body_width = window_box_width (w, TEXT_AREA);
      if !window_bits! (w, pseudo_window_p) && !MINI_WINDOW_P (w)
	  && body_width != (*w).old_body_pixel_width
	{ FRAME_WINDOW_CHANGE! ((*it).f, true); }
      (*it).last_visible_x = (*it).first_visible_x + body_width;

      /* If we truncate lines, leave room for the truncation glyph(s) at
	 the right margin.  Otherwise, leave room for the continuation
	 glyph(s).  Done only if the window has no right fringe.  */
      if WINDOW_RIGHT_FRINGE_WIDTH! ((*it).w) == 0
	{
	  if (*it).line_wrap == TRUNCATE
	    { (*it).last_visible_x -= (*it).truncation_pixel_width as c_int; }
	  else
	    { (*it).last_visible_x -= (*it).continuation_pixel_width as c_int; }
	}

      it_bits! (tab_line_p, it, window_wants_tab_line (w));
      it_bits! (header_line_p, it, window_wants_header_line (w));
      body_height = WINDOW_TAB_LINE_HEIGHT! (w) + WINDOW_HEADER_LINE_HEIGHT! (w);
      (*it).current_y = body_height + (*w).vscroll;
    }

  /* Leave room for a border glyph.  */
  if !FRAME_WINDOW_P ((*it).f)
    && !WINDOW_RIGHTMOST_P! ((*it).w)
    { (*it).last_visible_x -= 1; }

  (*it).last_visible_y = window_text_bottom_y (w);
  body_height += (*it).last_visible_y;
  if !window_bits! (w, pseudo_window_p) && !MINI_WINDOW_P (w)
    && body_height != (*w).old_body_pixel_height
    { FRAME_WINDOW_CHANGE! ((*it).f, true); }

  /* For mode lines and alike, arrange for the first glyph having a
     left box line if the face specifies a box.  */
  if base_face_id != DEFAULT_FACE_ID
    {
      let mut face: *mut face;

      (*it).face_id = remapped_base_face_id;

      /* If we have a boxed mode line, make the first character appear
	 with a left box line.  */
      face = FACE_FROM_ID_OR_NULL ((*it).f, remapped_base_face_id);
      if !face.is_null () && face_bits! (box, face) as face_box_type != FACE_NO_BOX
	{
	  let box_thickness: c_int = (*face).box_vertical_line_width;
	  it_bits! (face_box_p, it, true);
	  it_bits! (start_of_box_run_p, it, true);
	  /* Make sure we will have enough horizontal space to add the
	     right box line at the end.  */
	  if box_thickness > 0
	    { (*it).last_visible_x -= box_thickness; }
	}
    }

  /* If a buffer position was specified, set the iterator there,
     getting overlays and face properties from that position.  */
  if charpos >= BUF_BEG (current_buffer)
    {
      (*it).stop_charpos = charpos;
      (*it).end_charpos = ZV! ();
      eassert! (charpos == BYTE_TO_CHAR (bytepos));
      IT_CHARPOS! (*it) = charpos;
      IT_BYTEPOS! (*it) = bytepos;

      /* We will rely on `reseat' to set this up properly, via
	 handle_face_prop.  */
      (*it).face_id = (*it).base_face_id;

      (*it).start = (*it).current;
      /* Do we need to reorder bidirectional text?  Not if this is a
	 unibyte buffer: by definition, none of the single-byte
	 characters are strong R2L, so no reordering is needed.  And
	 bidi.c doesn't support unibyte buffers anyway.  Also, don't
	 reorder while we are loading loadup.el, since the tables of
	 character properties needed for reordering are not yet
	 available.  */
      it_bits! (bidi_p, it,
		!redisplay__inhibit_bidi
		&& !NILP (BVAR! (current_buffer, bidi_display_reordering_))
		&& it_bits! (multibyte_p, it));

      /* If we are to reorder bidirectional text, init the bidi
	 iterator.  */
      if it_bits! (bidi_p, it)
	{
	  /*  Since we don't know at this point whether there will be
	      any R2L lines in the window, we reserve space for
	      truncation/continuation glyphs even if only the left
	      fringe is absent.  */
	  if base_face_id == DEFAULT_FACE_ID
	      && WINDOW_LEFT_FRINGE_WIDTH! ((*it).w) == 0
	      && WINDOW_RIGHT_FRINGE_WIDTH! ((*it).w) != 0
	    {
	      if (*it).line_wrap == TRUNCATE
		{ (*it).last_visible_x -= (*it).truncation_pixel_width as c_int; }
	      else
		{ (*it).last_visible_x -= (*it).continuation_pixel_width as c_int; }
	    }
	  /* Note the paragraph direction that this buffer wants to
	     use.  */
	  if EQ (BVAR! (current_buffer, bidi_paragraph_direction_),
		 Qleft_to_right ())
	    { (*it).paragraph_embedding = L2R; }
	  else if EQ (BVAR! (current_buffer, bidi_paragraph_direction_),
		      Qright_to_left ())
	    { (*it).paragraph_embedding = R2L; }
	  else
	    { (*it).paragraph_embedding = NEUTRAL_DIR; }
	  bidi_unshelve_cache (core::ptr::null_mut (), false);
	  bidi_init_it (charpos, IT_BYTEPOS! (*it), FRAME_WINDOW_P ((*it).f),
			&raw mut (*it).bidi_it);
	}

      /* This is set only when long_line_optimizations_p is non-zero
	 for the current buffer.  */
      (*it).medium_narrowing_begv = 0;

      /* Compute faces etc.  */
      reseat (it, (*it).current.pos, true);
    }

  CHECK_IT (it);
}


/* How Emacs deals with long lines.

   (1) When a buffer is about to be (re)displayed, 'redisplay_window'
   detects, with a heuristic, whether it contains long lines.

   This happens in 'redisplay_window' because it is only displaying
   buffers with long lines that is problematic.  In other words, none
   of the optimizations described below is ever used in buffers that
   are never displayed.

   This happens with a heuristic, which checks whether a buffer
   contains long lines, each time its contents have changed "enough"
   between two redisplay cycles, because a buffer without long lines
   can become a buffer with long lines at any time, for example after
   a yank command, or after a replace command, or while the output of
   an external process is inserted in a buffer.

   When Emacs has detected that a buffer contains long lines, the
   buffer-local variable 'long_line_optimizations_p' (in 'struct
   buffer') is set, and Emacs does not try to detect whether the
   buffer does or does not contain long lines anymore.

   What a long line is depends on the variable 'long-line-threshold',
   whose default value is 50000 (characters).

   (2) When a buffer with long lines is (re)displayed, the amount of
   data that the display routines consider is, in a few well-chosen
   places, limited with a temporary restriction, whose bounds are
   calculated with the functions below.

   (2.1) 'get_small_narrowing_begv' is used to create a restriction
   which starts a few hundred characters before point.  The exact
   number of characters depends on the width of the window in which
   the buffer is displayed.

   There is no corresponding 'get_small_narrowing_zv' function,
   because it is not necessary to set the end limit of that
   restriction.

   This restriction is used in four places, namely:
   'back_to_previous_line_start' and 'move_it_vertically_backward'
   (with the 'SET_WITH_NARROWED_BEGV' macro), and in
   'composition_compute_stop_pos' and 'find_automatic_composition' (in
   a conditional statement depending on 'long_line_optimizations_p').

   (2.2) 'get_medium_narrowing_begv' is used to create a restriction
   which starts a few thousand characters before point.  The exact
   number of characters depends on the size (width and height) of the
   window in which the buffer is displayed.  For performance reasons,
   the return value of that function is cached in 'struct it', in the
   'medium_narrowing_begv' field.

   The corresponding function 'get_medium_narrowing_zv' (and
   'medium_narrowing_zv' field in 'struct it') is not used to set the
   end limit of the restriction, which is again unnecessary, but to
   determine, in 'reseat', whether the iterator has moved far enough
   from its original position, and whether the start position of the
   restriction must be computed anew.

   This restriction is used in a single place:
   'get_visually_first_element', with the 'SET_WITH_NARROWED_BEGV'
   macro.

   (2.3) 'get_large_narrowing_begv' and 'get_large_narrowing_zv' are
   used to create a restriction which starts a few hundred thousand
   characters before point and ends a few hundred thousand characters
   after point.  The size of that restriction depends on the variable
   'long-line-optimizations-region-size', whose default value is
   500000 (characters); it can be adjusted by a few hundred characters
   depending on 'long-line-optimizations-bol-search-limit', whose
   default value is 128 (characters).

   For performance reasons again, the return values of these functions
   are stored in the 'large_narrowing_begv' and 'large_narrowing_zv'
   fields in 'struct it'.

   The restriction defined by these values is used around three
   low-level hooks: around 'fontification-functions', in
   'handle_fontified_prop', and around 'pre-command-hook' and
   'post-command-hook', in 'safe_run_hooks_maybe_narrowed', which is
   called in 'command_loop_1'.  These restrictions are set around
   these hooks with 'labeled_narrow_to_region'; the restrictions are
   labeled, and cannot be removed with a call to 'widen', but can be
   removed with 'without-restriction' with a :label argument.
*/

unsafe fn get_narrowed_width (w: *mut window) -> c_int
{
  /* In a character-only terminal, only one font size is used, so we
     can use a smaller factor.  */
  let fact: c_int = if FRAME_WINDOW_P (XFRAME ((*w).frame)) { 3 } else { 2 };
  /* If the window has no fringes (in a character-only terminal or in
     a GUI frame without fringes), subtract 1 from the width for the
     '\' line wrapping character.  */
  let width: c_int
    = window_body_width (w, window_body_unit::WINDOW_BODY_IN_CANONICAL_CHARS)
      - (if WINDOW_RIGHT_FRINGE_WIDTH! (w) == 0 || WINDOW_LEFT_FRINGE_WIDTH! (w) == 0
	 { 1 } else { 0 });
  return fact * max (1, width as isize) as c_int;
}

unsafe fn get_narrowed_len (w: *mut window) -> c_int
{
  let height: c_int
    = window_body_height (w, window_body_unit::WINDOW_BODY_IN_CANONICAL_CHARS);
  return get_narrowed_width (w) * max (1, height as isize) as c_int;
}

unsafe fn get_medium_narrowing_begv (w: *mut window, pos: ptrdiff_t) -> ptrdiff_t
{
  let len: ptrdiff_t = get_narrowed_len (w) as ptrdiff_t;
  return max ((pos / len - 1) * len, BEGV! ());
}

unsafe fn get_medium_narrowing_zv (w: *mut window, pos: ptrdiff_t) -> ptrdiff_t
{
  let len: ptrdiff_t = get_narrowed_len (w) as ptrdiff_t;
  return min ((pos / len + 1) * len, ZV! ());
}

/* Find the position of the last BOL before POS, unless it is too far
   away.  The buffer portion in which the search occurs is gradually
   enlarged: [POS-500..POS], [POS-5500..POS-500],
   [POS-55500..POS-5500], and finally [POS-555500..POS-55500].  Return
   BEGV-1 if no BOL was found in [POS-555500..POS].  (C 3765-3768)  */
unsafe fn get_nearby_bol_pos (mut pos: ptrdiff_t) -> ptrdiff_t
{
  let mut start: ptrdiff_t;
  let mut pos_bytepos: ptrdiff_t;
  let mut cur: ptrdiff_t;
  let mut next: ptrdiff_t;
  let mut found: ptrdiff_t = 0;
  let mut bol: ptrdiff_t = BEGV! () - 1;
  let init_pos: ptrdiff_t = pos;
  let mut dist: c_int;
  dist = 500;
  while dist <= 500000
    {
      pos_bytepos = if pos == BEGV! () { BEGV_BYTE! () } else { CHAR_TO_BYTE (pos) };
      start = max (pos - dist as ptrdiff_t, BEGV! ());
      cur = start;
      while cur < pos
	{
	  next = find_newline1 (cur, CHAR_TO_BYTE (cur),
				pos, pos_bytepos,
				1, &raw mut found, core::ptr::null_mut (), false);
	  if found != 0
	    { bol = next; }
	  else
	    { break; }
	  cur = next;
	}
      if bol >= BEGV! () || start == BEGV! ()
	{ break; }
      else
	{ pos = max (pos - dist as ptrdiff_t, BEGV! ()); }
      dist *= 10;
    }
  eassert! (bol <= init_pos);
  return bol;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_small_narrowing_begv (w: *mut window, pos: ptrdiff_t)
						   -> ptrdiff_t
{
  let len: c_int = get_narrowed_width (w);
  let bol_pos: ptrdiff_t = max (get_nearby_bol_pos (pos), BEGV! ());
  return max (bol_pos + ((pos - bol_pos) / len as ptrdiff_t - 1) * len as ptrdiff_t,
	      BEGV! ());
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_large_narrowing_begv (pos: ptrdiff_t) -> ptrdiff_t
{
  if long_line_optimizations_region_size <= 0
    { return BEGV! (); }
  let len: ptrdiff_t = (long_line_optimizations_region_size / 2) as ptrdiff_t;
  let mut begv: ptrdiff_t = max (pos - len, BEGV! ());
  let mut limit: ptrdiff_t = long_line_optimizations_bol_search_limit as ptrdiff_t;
  while limit > 0
    {
      if begv == BEGV! () || FETCH_BYTE! (CHAR_TO_BYTE (begv) - 1) == b'\n'
	{ return begv; }
      begv -= 1;
      limit -= 1;
    }
  return begv;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_large_narrowing_zv (pos: ptrdiff_t) -> ptrdiff_t
{
  if long_line_optimizations_region_size <= 0
    { return ZV! (); }
  let len: ptrdiff_t = (long_line_optimizations_region_size / 2) as ptrdiff_t;
  return min (pos + len, ZV! ());
}

unsafe extern "C" fn unwind_narrowed_begv (point_min: Lisp_Object)
{
  SET_BUF_BEGV (current_buffer, XFIXNUM (point_min));
}

/* Set DST to EXPR.  When IT indicates that BEGV should temporarily be
   updated to optimize display, evaluate EXPR with BEGV set to BV.  (C 3827-3828)  */

macro_rules! SET_WITH_NARROWED_BEGV {
  ($IT:expr, $DST:expr, $EXPR:expr, $BV:expr) => {
    {
      if (*$IT).medium_narrowing_begv != 0
	{
	  let count: specpdl_ref = SPECPDL_INDEX ();
	  record_unwind_protect (Some (unwind_narrowed_begv), Fpoint_min ());
	  SET_BUF_BEGV (current_buffer, $BV);
	  $DST = $EXPR;
	  unbind_to (count, Qnil ());
	}
      else
	{ $DST = $EXPR; }
    }
  };
}

/* Initialize IT for the display of window W with window start POS.  (C 3844-3845)  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn start_display (it: *mut it, w: *mut window, pos: text_pos)
{
  let mut row: *mut glyph_row;
  let first_vpos: c_int = window_wants_tab_line (w) as c_int
			  + window_wants_header_line (w) as c_int;

  row = (*((*w).desired_matrix as *mut glyph_matrix)).rows.add (first_vpos as usize);
  init_iterator (it, w, CHARPOS! (pos), BYTEPOS! (pos), row, DEFAULT_FACE_ID);
  (*it).first_vpos = first_vpos;

  /* Don't reseat to previous visible line start if current start
     position is in a string or image.  */
  if (*it).line_wrap != TRUNCATE
    {
      let method: it_method = (*it).method;

      /* If window start is not at a line start, skip forward to POS
	 from the beginning of physical line to get the correct
	 continuation lines width.  */
      let start_at_line_beg_p: bool
	= (CHARPOS! (pos) == BEGV! ()
	   || FETCH_BYTE! (BYTEPOS! (pos) - 1) == b'\n');
      if !start_at_line_beg_p
	{
	  let first_y: c_int = (*it).current_y;
	  let mut continuation_width: c_int;
	  let mut itdata: *mut c_void = core::ptr::null_mut ();
	  let mut it2: it = core::mem::zeroed ();
	  let mut new_x: c_int;

	  if method != GET_FROM_BUFFER
	    { SAVE_IT! (it2, *it, itdata); }
	  reseat_at_previous_visible_line_start (it);
	  move_it_to (it, CHARPOS! (pos), -1, -1, -1, MOVE_TO_POS);

	  new_x = (*it).current_x + (*it).pixel_width;

	  /* If lines are continued, this line may end in the middle
	     of a multi-glyph character (e.g. a control character
	     displayed as \003, or in the middle of an overlay
	     string).  In this case move_it_to above will not have
	     taken us to the start of the continuation line but to the
	     end of the continued line.  */
	  if (*it).current_x > 0
	      && (*it).line_wrap != TRUNCATE /* Lines are continued.  */
	      && (/* And glyph doesn't fit on the line.  */
		  new_x > (*it).last_visible_x
		  /* Or it fits exactly and we're on a window
		     system frame.  */
		  || (new_x == (*it).last_visible_x
		      && FRAME_WINDOW_P ((*it).f)
		      && (if it_bits! (bidi_p, it)
			     && (*it).bidi_it.paragraph_dir == R2L
			  { WINDOW_LEFT_FRINGE_WIDTH! ((*it).w) }
			  else
			  { WINDOW_RIGHT_FRINGE_WIDTH! ((*it).w) }) != 0))
	    {
	      if (((*it).current.dpvec_index >= 0
		   || (*it).current.overlay_string_index >= 0)
		  /* If we are on a newline from a display vector or
		     overlay string, then we are already at the end of
		     a screen line; no need to go to the next line in
		     that case, as this line is not really continued.
		     (If we do go to the next line, C-e will not DTRT.)  */
		  && (*it).c != '\n' as c_int)
		{
		  set_iterator_to_next (it, true);
		  move_it_in_display_line_to (it, -1, -1, 0);
		}

	      (*it).continuation_lines_width += (*it).current_x;
	    }
	  /* If the character at POS is displayed via a display
	     vector, move_it_to above stops at the final glyph of
	     IT->dpvec.  To make the caller redisplay that character
	     again (a.k.a. start at POS), we need to reset the
	     dpvec_index to the beginning of IT->dpvec.  */
	  else if (*it).current.dpvec_index >= 0
	    { (*it).current.dpvec_index = 0; }

	  continuation_width = (*it).continuation_lines_width;
	  /* If we started from a position in something other than a
	     buffer, restore the original iterator state, keeping only
	     the continuation_lines_width, since we could now be very
	     far from the original position.  */
	  if method != GET_FROM_BUFFER
	    {
	      RESTORE_IT! (it, &raw mut it2, itdata);
	      (*it).continuation_lines_width = continuation_width;
	    }

	  /* We're starting a new display line, not affected by the
	     height of the continued line, so clear the appropriate
	     fields in the iterator structure.  */
	  (*it).max_ascent = 0;
	  (*it).max_descent = 0;
	  (*it).max_phys_ascent = 0;
	  (*it).max_phys_descent = 0;

	  (*it).current_y = first_y;
	  (*it).vpos = 0;
	  (*it).current_x = 0;
	  (*it).hpos = 0;
	  (*it).wrap_prefix_width = 0;
	}
    }
}


/* Return true if POS is a position in ellipses displayed for invisible
   text.  W is the window we display, for text property lookup.  (C 3948-3949)  */

unsafe fn in_ellipses_for_invisible_text_p (pos: *mut display_pos, w: *mut window) -> bool
{
  let mut prop: Lisp_Object;
  let mut window: Lisp_Object;
  let mut ellipses_p: bool = false;
  let charpos: ptrdiff_t = CHARPOS! ((*pos).pos);

  /* If POS specifies a position in a display vector, this might
     be for an ellipsis displayed for invisible text.  We won't
     get the iterator set up for delivering that ellipsis unless
     we make sure that it gets aware of the invisible text.  */
  if (*pos).dpvec_index >= 0
      && (*pos).overlay_string_index < 0
      && CHARPOS! ((*pos).string_pos) < 0
      && charpos > BEGV! ()
      && {
	XSETWINDOW! (window, w);
	prop = Fget_char_property (make_fixnum (charpos),
				   Qinvisible (), window);
	TEXT_PROP_MEANS_INVISIBLE! (prop) == 0
      }
    {
      prop = Fget_char_property (make_fixnum (charpos - 1), Qinvisible (),
				 window);
      ellipses_p = 2 == TEXT_PROP_MEANS_INVISIBLE! (prop);
    }

  return ellipses_p;
}


/* Initialize IT for stepping through current_buffer in window W,
   starting at position POS that includes overlay string and display
   vector/ control character translation position information.  Value
   is false if there are overlay strings with newlines at POS.  (C 3978-4125)  */

unsafe fn init_from_display_pos (it: *mut it, w: *mut window, pos: *mut display_pos) -> bool
{
  let mut charpos: ptrdiff_t = CHARPOS! ((*pos).pos);
  let mut bytepos: ptrdiff_t = BYTEPOS! ((*pos).pos);
  let mut i: c_int;
  let mut overlay_strings_with_newlines: bool = false;

  /* If POS specifies a position in a display vector, this might
     be for an ellipsis displayed for invisible text.  We won't
     get the iterator set up for delivering that ellipsis unless
     we make sure that it gets aware of the invisible text.  */
  if in_ellipses_for_invisible_text_p (pos, w)
    {
      charpos -= 1;
      bytepos = BYTE_TO_CHAR (charpos);
    }

  /* Keep in mind: the call to reseat in init_iterator skips invisible
     text, so we might end up at a position different from POS.  This
     is only a problem when POS is a row start after a newline and an
     overlay starts there with an after-string, and the overlay has an
     invisible property.  Since we don't skip invisible text in
     display_line and elsewhere immediately after consuming the
     newline before the row start, such a POS will not be in a string,
     but the call to init_iterator below will move us to the
     after-string.  */
  init_iterator (it, w, charpos, bytepos, core::ptr::null_mut (), DEFAULT_FACE_ID);

  /* This only scans the current chunk -- it should scan all chunks.
     However, OVERLAY_STRING_CHUNK_SIZE has been increased from 3 in 21.1
     to 16 in 22.1 to make this a lesser problem.  */
  i = 0;
  while (i as ptrdiff_t) < (*it).n_overlay_strings && i < OVERLAY_STRING_CHUNK_SIZE
    {
      let mut s: *const c_char
	= SSDATA ((*it).overlay_strings [i as usize]) as *const c_char;
      let e: *const c_char
	= s.offset (SBYTES ((*it).overlay_strings [i as usize]) as isize);

      while s < e && *s != b'\n' as c_char
	{ s = s.offset (1); }

      if s < e
	{
	  overlay_strings_with_newlines = true;
	  break;
	}
      i += 1;
    }

  /* If position is within an overlay string, set up IT to the right
     overlay string.  */
  if (*pos).overlay_string_index >= 0
    {
      let mut relative_index: c_int;

      /* If the first overlay string happens to have a `display'
	 property for an image, the iterator will be set up for that
	 image, and we have to undo that setup first before we can
	 correct the overlay string index.  */
      if (*it).method == GET_FROM_IMAGE
	{ pop_it (it); }

      /* We already have the first chunk of overlay strings in
	 IT->overlay_strings.  Load more until the one for
	 pos->overlay_string_index is in IT->overlay_strings.  */
      if (*pos).overlay_string_index >= OVERLAY_STRING_CHUNK_SIZE as ptrdiff_t
	{
	  let mut n: ptrdiff_t
	    = (*pos).overlay_string_index / OVERLAY_STRING_CHUNK_SIZE as ptrdiff_t;
	  (*it).current.overlay_string_index = 0;
	  while n != 0
	    {
	      n -= 1;
	      load_overlay_strings (it, 0);
	      (*it).current.overlay_string_index += OVERLAY_STRING_CHUNK_SIZE as ptrdiff_t;
	    }
	}

      (*it).current.overlay_string_index = (*pos).overlay_string_index;
      relative_index = ((*it).current.overlay_string_index
			% OVERLAY_STRING_CHUNK_SIZE as ptrdiff_t) as c_int;
      (*it).string = (*it).overlay_strings [relative_index as usize];
      eassert! (STRINGP ((*it).string));
      (*it).current.string_pos = (*pos).string_pos;
      (*it).method = GET_FROM_STRING;
      (*it).end_charpos = SCHARS ((*it).string);
      /* Set up the bidi iterator for this overlay string.  */
      if it_bits! (bidi_p, it)
	{
	  (*it).bidi_it.string.lstring = (*it).string;
	  (*it).bidi_it.string.s = core::ptr::null ();
	  (*it).bidi_it.string.schars = SCHARS ((*it).string);
	  (*it).bidi_it.string.bufpos = (*it).overlay_strings_charpos;
	  bidi_string_bits! (from_disp_str, &raw mut (*it).bidi_it.string,
			     it_bits! (string_from_display_prop_p, it));
	  bidi_string_bits! (unibyte, &raw mut (*it).bidi_it.string,
			     ! it_bits! (multibyte_p, it));
	  (*it).bidi_it.w = (*it).w;
	  bidi_init_it (IT_STRING_CHARPOS! (*it), IT_STRING_BYTEPOS! (*it),
			FRAME_WINDOW_P ((*it).f), &raw mut (*it).bidi_it);

	  /* Synchronize the state of the bidi iterator with
	     pos->string_pos.  For any string position other than
	     zero, this will be done automagically when we resume
	     iteration over the string and get_visually_first_element
	     is called.  But if string_pos is zero, and the string is
	     to be reordered for display, we need to resync manually,
	     since it could be that the iteration state recorded in
	     pos ended at string_pos of 0 moving backwards in string.  */
	  if CHARPOS! ((*pos).string_pos) == 0
	    {
	      get_visually_first_element (it);
	      if IT_STRING_CHARPOS! (*it) != 0
		{
		  loop {
		    /* Paranoia.  */
		    eassert! ((*it).bidi_it.charpos < (*it).bidi_it.string.schars);
		    bidi_move_to_visually_next (&raw mut (*it).bidi_it);
		    if !((*it).bidi_it.charpos != 0) { break; }
		  }
		}
	    }
	  eassert! (IT_STRING_CHARPOS! (*it) == (*it).bidi_it.charpos
		    && IT_STRING_BYTEPOS! (*it) == (*it).bidi_it.bytepos);
	}
    }

  if CHARPOS! ((*pos).string_pos) >= 0
    {
      /* Recorded position is not in an overlay string, but in another
	 string.  This can only be a string from a `display' property.
	 IT should already be filled with that string.  */
      (*it).current.string_pos = (*pos).string_pos;
      eassert! (STRINGP ((*it).string));
      if it_bits! (bidi_p, it)
	{ bidi_init_it (IT_STRING_CHARPOS! (*it), IT_STRING_BYTEPOS! (*it),
			FRAME_WINDOW_P ((*it).f), &raw mut (*it).bidi_it); }
    }

  /* Restore position in display vector translations, control
     character translations or ellipses.  */
  if (*pos).dpvec_index >= 0
    {
      if (*it).dpvec.is_null ()
	{ get_next_display_element (it); }
      eassert! (! (*it).dpvec.is_null () && (*it).current.dpvec_index == 0);
      (*it).current.dpvec_index = (*pos).dpvec_index;
    }

  CHECK_IT (it);
  return !overlay_strings_with_newlines;
}

/* Initialize IT for stepping through current_buffer in window W
   starting at ROW->start.  (C 4128-4138)  */

unsafe fn init_to_row_start (it: *mut it, w: *mut window, row: *mut glyph_row)
{
  init_from_display_pos (it, w, &raw mut (*row).start);
  (*it).start = (*row).start;
  (*it).continuation_lines_width = (*row).continuation_lines_width;
  CHECK_IT (it);
}

/* Initialize IT for stepping through current_buffer in window W
   starting in the line following ROW, i.e. starting at ROW->end.
   Value is false if there are overlay strings with newlines at ROW's
   end position, or if the following row begins with bidi-reordered
   characters that could be composed.  (C 4141-4175)  */

unsafe fn init_to_row_end (it: *mut it, w: *mut window, row: *mut glyph_row) -> bool
{
  let mut success: bool = false;

  if init_from_display_pos (it, w, &raw mut (*row).end)
    {
      if glyph_row_bits! (continued_p, row)
	{ (*it).continuation_lines_width =
	    (*row).continuation_lines_width + (*row).pixel_width; }
      CHECK_IT (it);
      /* Initializing IT in the presence of compositions in reordered
	 rows is tricky: row->end above will generally cause us to
	 start at position that is not the first one in the logical
	 order, and we might therefore miss the composition earlier in
	 the buffer that affects how glypsh are laid out in this row.
	 So we punt instead.  Note: the test below works because
	 get_next_display_element calls get_visually_first_element,
	 which calls composition_compute_stop_pos, which populates
	 it->cmp_it.  */
      if get_next_display_element (it)
	  && ((*it).bidi_it.scan_dir == -1 && (*it).cmp_it.id >= 0)
	{ success = false; }
      else
	{ success = true; }
    }

  return success;
}

/***********************************************************************
			   Text properties
 ***********************************************************************/

/* Called when IT reaches IT->stop_charpos.  Handle text property and
   overlay changes.  Set IT->stop_charpos to the next position where
   to stop.  (C 4184-4323)  */

unsafe fn handle_stop (it: *mut it)
{
  let mut handled: prop_handled;
  let mut handle_overlay_change_p: bool;
  let mut p: *mut props;

  (*it).dpvec = core::ptr::null_mut ();
  (*it).current.dpvec_index = -1;
  handle_overlay_change_p = ! it_bits! (ignore_overlay_strings_at_pos_p, it);
  it_bits! (ellipsis_p, it, false);

  /* Use face of preceding text for ellipsis (if invisible) */
  if it_bits! (selective_display_ellipsis_p, it)
    { (*it).saved_face_id = (*it).face_id; }

  /* Here's the description of the semantics of, and the logic behind,
     the various HANDLED_* statuses:

     HANDLED_NORMALLY means the handler did its job, and the loop
     should proceed to calling the next handler in order.

     HANDLED_RECOMPUTE_PROPS means the handler caused a significant
     change in the properties and overlays at current position, so the
     loop should be restarted, to re-invoke the handlers that were
     already called.  This happens when fontification-functions were
     called by handle_fontified_prop, and actually fontified
     something.  Another case where HANDLED_RECOMPUTE_PROPS is
     returned is when we discover overlay strings that need to be
     displayed right away.  The loop below will continue for as long
     as the status is HANDLED_RECOMPUTE_PROPS.

     HANDLED_RETURN means return immediately to the caller, to
     continue iteration without calling any further handlers.  This is
     used when we need to act on some property right away, for example
     when we need to display the ellipsis or a replacing display
     property, such as display string or image.

     HANDLED_OVERLAY_STRING_CONSUMED means an overlay string was just
     consumed, and the handler switched to the next overlay string.
     This signals the loop below to refrain from looking for more
     overlays before all the overlay strings of the current overlay
     are processed.

     Some of the handlers called by the loop push the iterator state
     onto the stack (see 'push_it'), and arrange for the iteration to
     continue with another object, such as an image, a display string,
     or an overlay string.  In most such cases, it->stop_charpos is
     set to the first character of the string, so that when the
     iteration resumes, this function will immediately be called
     again, to examine the properties at the beginning of the string.

     When a display or overlay string is exhausted, the iterator state
     is popped (see 'pop_it'), and iteration continues with the
     previous object.  Again, in many such cases this function is
     called again to find the next position where properties might
     change.  */
  loop
    {
      handled = HANDLED_NORMALLY;

      /* Call text property handlers.  */
      p = it_props.as_mut_ptr ();
      while (*p).handler.is_some ()
	{
	  handled = ((*p).handler.unwrap ()) (it);

	  if handled == HANDLED_RECOMPUTE_PROPS
	    { break; }
	  else if handled == HANDLED_RETURN
	    {
	      /* We still want to show before and after strings from
		 overlays even if the actual buffer text is replaced.  */
	      if !handle_overlay_change_p
		  || (*it).sp > 1
		  /* Don't call get_overlay_strings_1 if we already
		     have overlay strings loaded, because doing so
		     will load them again and push the iterator state
		     onto the stack one more time, which is not
		     expected by the rest of the code that processes
		     overlay strings.  */
		  || ((*it).current.overlay_string_index < 0
		      && ! get_overlay_strings_1 (it, 0, false))
		{
		  if it_bits! (ellipsis_p, it)
		    { setup_for_ellipsis (it, 0); }
		  /* When handling a display spec, we might load an
		     empty string.  In that case, discard it here.  We
		     used to discard it in handle_single_display_spec,
		     but that causes get_overlay_strings_1, above, to
		     ignore overlay strings that we must check.  */
		  if STRINGP ((*it).string) && SCHARS ((*it).string) == 0
		    { pop_it (it); }
		  return;
		}
	      else if STRINGP ((*it).string) && SCHARS ((*it).string) == 0
		{ pop_it (it); }
	      else
		{
		  it_bits! (string_from_display_prop_p, it, false);
		  it_bits! (from_disp_prop_p, it, false);
		  handle_overlay_change_p = false;
		}
	      handled = HANDLED_RECOMPUTE_PROPS;
	      break;
	    }
	  else if handled == HANDLED_OVERLAY_STRING_CONSUMED
	    { handle_overlay_change_p = false; }

	  p = p.offset (1);
	}

      if handled != HANDLED_RECOMPUTE_PROPS
	{
	  /* Don't check for overlay strings below when set to deliver
	     characters from a display vector.  */
	  if (*it).method == GET_FROM_DISPLAY_VECTOR
	    { handle_overlay_change_p = false; }

	  /* Handle overlay changes.
	     This sets HANDLED to HANDLED_RECOMPUTE_PROPS
	     if it finds overlays.  */
	  if handle_overlay_change_p
	    { handled = handle_overlay_change (it); }
	}

      if it_bits! (ellipsis_p, it)
	{
	  setup_for_ellipsis (it, 0);
	  break;
	}
      if !(handled == HANDLED_RECOMPUTE_PROPS) { break; }
    }

  /* Determine where to stop next.  */
  if handled == HANDLED_NORMALLY
    { compute_stop_pos (it); }
}


/* Compute IT->stop_charpos from text property and overlay change
   information for IT's current position.  (C 4326-4484)  */

unsafe fn compute_stop_pos (it: *mut it)
{
  let mut iv: INTERVAL;
  let mut next_iv: INTERVAL;
  let mut object: Lisp_Object;
  let mut limit: Lisp_Object;
  let mut position: Lisp_Object;
  let mut charpos: ptrdiff_t;
  let mut bytepos: ptrdiff_t;
  let mut cmp_limit_pos: ptrdiff_t = -1;

  if STRINGP ((*it).string)
    {
      /* Strings are usually short, so don't limit the search for
	 properties.  */
      (*it).stop_charpos = (*it).end_charpos;
      object = (*it).string;
      limit = Qnil ();
      charpos = IT_STRING_CHARPOS! (*it);
      bytepos = IT_STRING_BYTEPOS! (*it);
    }
  else
    {
      let mut pos: ptrdiff_t;

      /* If end_charpos is out of range for some reason, such as a
	 misbehaving display function, rationalize it (Bug#5984).  */
      if (*it).end_charpos > ZV! ()
	{ (*it).end_charpos = ZV! (); }
      (*it).stop_charpos = (*it).end_charpos;

      /* If next overlay change is in front of the current stop pos
	 (which is IT->end_charpos), stop there.  Note: value of
	 next_overlay_change is point-max if no overlay change
	 follows.  */
      charpos = IT_CHARPOS! (*it);
      bytepos = IT_BYTEPOS! (*it);
      pos = next_overlay_change (charpos);
      if pos < (*it).stop_charpos
	{ (*it).stop_charpos = pos; }
      /* If we are breaking compositions at point, stop at point.  */
      if !NILP (BVAR! (current_buffer, enable_multibyte_characters_))
	  && !NILP (Vauto_composition_mode)
	  && composition_break_at_point
	  && charpos < PT! () && PT! () < (*it).stop_charpos
	{ (*it).stop_charpos = PT! (); }

      /* Set up variables for computing the stop position from text
         property changes.  */
      XSETBUFFER! (object, current_buffer);
      pos = charpos + TEXT_PROP_DISTANCE_LIMIT as ptrdiff_t;
      /* Make sure the above arbitrary limit position is not in the
	 middle of composable text, so we don't break compositions by
	 submitting the composable text to the shaper in separate
	 chunks.  We play safe here by assuming that only SPC, TAB,
	 FF, and NL cannot be in some composition; in particular, most
	 ASCII punctuation characters could be composed into ligatures.  */
      if !composition_break_at_point
	  && !NILP (BVAR! (current_buffer, enable_multibyte_characters_))
	  && !NILP (Vauto_composition_mode)
	{
	  let mut endpos: ptrdiff_t
	    = charpos + 10 * TEXT_PROP_DISTANCE_LIMIT as ptrdiff_t;
	  let mut found: bool = false;

	  if pos > ZV! ()
	    { pos = ZV! (); }
	  if endpos > ZV! ()
	    { endpos = ZV! (); }
	  let mut bpos: ptrdiff_t = CHAR_TO_BYTE (pos);
	  while pos < endpos
	    {
	      let ch: c_int = fetch_char_advance_no_check (&raw mut pos, &raw mut bpos);
	      if ch == b' ' as c_int || ch == b'\t' as c_int
		  || ch == b'\n' as c_int || ch == b'\x0c' as c_int
		{
		  found = true;
		  break;
		}
	    }
	  if found
	    {
	      pos -= 1;
	      cmp_limit_pos = pos;
	    }
	  else if (*it).stop_charpos < endpos
	    { pos = (*it).stop_charpos; }
	  else
	    {
	      /* Give up and use the original arbitrary limit.  */
	      pos = charpos + TEXT_PROP_DISTANCE_LIMIT as ptrdiff_t;
	    }
	}
      limit = make_fixnum (pos);
    }

  /* Get the interval containing IT's position.  Value is a null
     interval if there isn't such an interval.  */
  position = make_fixnum (charpos);
  iv = validate_interval_range (object, &raw mut position, &raw mut position, false);
  if !iv.is_null ()
    {
      let mut values_here: [Lisp_Object; LAST_PROP_IDX as usize]
	= [lisp_h_Qnil (); LAST_PROP_IDX as usize];
      let mut p: *mut props;

      /* Get properties here.  */
      p = it_props.as_mut_ptr ();
      while (*p).handler.is_some ()
	{
	  values_here[(*p).idx as usize]
	    = textget ((*((iv) as *mut interval)).plist,
		       builtin_lisp_symbol ((*p).name as usize));
	  p = p.offset (1);
	}

      /* Look for an interval following iv that has different
	 properties.  */
      next_iv = next_interval (iv);
      while !next_iv.is_null ()
	    && (NILP (limit)
		|| XFIXNAT (limit) > (*((next_iv) as *mut interval)).position)
	{
	  p = it_props.as_mut_ptr ();
	  while (*p).handler.is_some ()
	    {
	      let new_value: Lisp_Object
		= textget ((*((next_iv) as *mut interval)).plist,
			   builtin_lisp_symbol ((*p).name as usize));
	      if !EQ (values_here[(*p).idx as usize], new_value)
		{ break; }
	      p = p.offset (1);
	    }

	  if (*p).handler.is_some ()
	    { break; }
	  next_iv = next_interval (next_iv);
	}

      if !next_iv.is_null ()
	{
	  if FIXNUMP (limit)
	      && (*((next_iv) as *mut interval)).position >= XFIXNAT (limit)
	    /* No text property change up to limit.  */
	    { (*it).stop_charpos = min (XFIXNAT (limit), (*it).stop_charpos); }
	  else
	    /* Text properties change in next_iv.  */
	    { (*it).stop_charpos
		= min ((*it).stop_charpos,
		       (*((next_iv) as *mut interval)).position); }
	}
    }

  if (*it).cmp_it.id < 0
    {
      let mut stoppos: ptrdiff_t = (*it).end_charpos;

      /* If we found, above, a buffer position that cannot be part of
         an automatic composition, limit the search of composable
         characters to that position.  */
      if it_bits! (bidi_p, it) && (*it).bidi_it.scan_dir < 0
	{ stoppos = bidi_level_start ((*it).bidi_it.resolved_level as c_int) - 1; }
      else if !STRINGP ((*it).string)
	       && (*it).cmp_it.stop_pos <= IT_CHARPOS! (*it)
	       && cmp_limit_pos > 0
	{ stoppos = cmp_limit_pos; }
      /* Force composition_compute_stop_pos avoid the costly search
         for static compositions, since those were already found by
         looking at text properties, above.  */
      composition_compute_stop_pos (&raw mut (*it).cmp_it as *mut c_void,
				    charpos, bytepos, stoppos, (*it).string,
				    false);
    }

  eassert! (STRINGP ((*it).string)
	    || ((*it).stop_charpos >= BEGV! ()
		&& (*it).stop_charpos >= IT_CHARPOS! (*it)));
}

/* How many characters forward to search for a display property or
   display string.  Searching too far forward makes the bidi display
   sluggish, especially in small windows.  (C 4490-4493)  */
pub const MAX_DISP_SCAN: c_int = 250;

/* Return the character position of a display string at or after
   position specified by POSITION.  If no display string exists at or
   after POSITION, return ZV.  A display string is either an overlay
   with `display' property whose value is a string, or a `display'
   text property whose value is a string.  STRING is data about the
   string to iterate; if STRING->lstring is nil, we are iterating a
   buffer.  FRAME_WINDOW_P is true when we are displaying a window
   on a GUI frame.  DISP_PROP is set to zero if we searched
   MAX_DISP_SCAN characters forward without finding any display
   strings, non-zero otherwise.  It is set to 2 if the display string
   uses any kind of `(space ...)' spec that will produce a stretch of
   white space in the text area.  (C 4495-4582)  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn compute_display_string_pos (position: *mut text_pos,
						     string: *mut bidi_string_data,
						     w: *mut window,
						     frame_window_p: bool,
						     disp_prop: *mut c_int)
     -> ptrdiff_t
{
  /* OBJECT = nil means current buffer.  */
  let mut object: Lisp_Object;
  let mut object1: Lisp_Object;
  let mut pos: Lisp_Object;
  let mut spec: Lisp_Object;
  let mut limpos: Lisp_Object;
  let string_p: bool = !string.is_null ()
    && (STRINGP ((*string).lstring) || !(*string).s.is_null ());
  let eob: ptrdiff_t = if string_p { (*string).schars } else { ZV! () };
  let begb: ptrdiff_t = if string_p { 0 } else { BEGV! () };
  let mut bufpos: ptrdiff_t;
  let charpos: ptrdiff_t = CHARPOS! (*position);
  let lim: ptrdiff_t
    = if charpos < eob - MAX_DISP_SCAN as ptrdiff_t
	{ charpos + MAX_DISP_SCAN as ptrdiff_t }
      else
	{ eob };
  let mut tpos: text_pos;
  let mut rv: c_int = 0;
  let mut overlay: Lisp_Object = Qnil ();
  let mut ov1: Lisp_Object = Qnil ();

  if !string.is_null () && STRINGP ((*string).lstring)
    { object = (*string).lstring; object1 = object; }
  else if !w.is_null () && !string_p
    {
      XSETWINDOW! (object, w);
      object1 = Qnil ();
    }
  else
    { object = Qnil (); object1 = Qnil (); }

  *disp_prop = 1;

  if charpos >= eob
      /* We don't support display properties whose values are strings
	 that have display string properties.  */
      || bidi_string_bits! (from_disp_str, string)
      /* C strings cannot have display properties.  */
      || (!(*string).s.is_null () && !STRINGP (object))
    {
      *disp_prop = 0;
      return eob;
    }

  /* If the character at CHARPOS is where the display string begins,
     return CHARPOS.  */
  pos = make_fixnum (charpos);
  if STRINGP (object)
    { bufpos = (*string).bufpos; }
  else
    { bufpos = charpos; }
  tpos = *position;
  if !NILP ({ spec = get_char_property_and_overlay (pos, Qdisplay (), object,
						    &raw mut overlay); spec })
      && (charpos <= begb
	  || !EQ ({ get_char_property_and_overlay (make_fixnum (charpos - 1),
						   Qdisplay (), object,
						   &raw mut ov1) },
		  spec)
	  || !BASE_EQ (overlay, ov1))
      && { rv = handle_display_spec (core::ptr::null_mut (), spec, object1,
				     overlay, &raw mut tpos, bufpos,
				     frame_window_p);
	   rv != 0 }
    {
      if rv == 2
	{ *disp_prop = 2; }
      return charpos;
    }

  /* Look forward for the first character with a `display' property
     that will replace the underlying text when displayed.  */
  limpos = make_fixnum (lim);
  loop {
    pos = Fnext_single_char_property_change (pos, Qdisplay (), object1, limpos);
    CHARPOS! (tpos) = XFIXNAT (pos);
    if CHARPOS! (tpos) >= lim
      {
	*disp_prop = 0;
	break;
      }
    if STRINGP (object)
      { BYTEPOS! (tpos) = string_char_to_byte (object, CHARPOS! (tpos)); }
    else
      { BYTEPOS! (tpos) = CHAR_TO_BYTE (CHARPOS! (tpos)); }
    spec = get_char_property_and_overlay (pos, Qdisplay (), object, &raw mut overlay);
    if !STRINGP (object)
      { bufpos = CHARPOS! (tpos); }
    if !(NILP (spec)
	 || { rv = handle_display_spec (core::ptr::null_mut (), spec, object1,
					overlay, &raw mut tpos, bufpos,
					frame_window_p);
	      rv == 0 })
      { break; }
  }
  if rv == 2
    { *disp_prop = 2; }

  return CHARPOS! (tpos);
}

/* Return the character position of the end of the display string that
   started at CHARPOS.  If there's no display string at CHARPOS,
   return -1.  A display string is either an overlay with `display'
   property whose value is a string or a `display' text property whose
   value is a string.  (C 4594-4641)  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn compute_display_string_end (charpos: ptrdiff_t,
						     string: *mut bidi_string_data,
						     w: *mut window)
     -> ptrdiff_t
{
  let mut window: Lisp_Object = Qnil ();
  if !w.is_null ()
    { XSETWINDOW! (window, w); }

  /* OBJECT = nil means current buffer.  */
  let mut object: Lisp_Object
    = if !string.is_null () && STRINGP ((*string).lstring)
	{ (*string).lstring }
      else if WINDOWP (window)
	{ window }
      else
	{ Qnil () };
  let mut pos: Lisp_Object = make_fixnum (charpos);
  let eob: ptrdiff_t
    = if STRINGP (object) || (!string.is_null () && !(*string).s.is_null ())
	{ (*string).schars }
      else
	{ ZV! () };

  if charpos >= eob || (!(*string).s.is_null () && !STRINGP (object))
    { return eob; }

  /* It could happen that the display property or overlay was removed
     since we found it in compute_display_string_pos above.  One way
     this can happen is if JIT font-lock was called (through
     handle_fontified_prop), and jit-lock-functions remove text
     properties or overlays from the portion of buffer that includes
     CHARPOS.  Muse mode is known to do that, for example.  In this
     case, we return -1 to the caller, to signal that no display
     string is actually present at CHARPOS.  See bidi_fetch_char for
     how this is handled.

     An alternative would be to never look for display properties past
     it->stop_charpos.  But neither compute_display_string_pos nor
     bidi_fetch_char that calls it know or care where the next
     stop_charpos is.  */
  let mut overlay: Lisp_Object = Qnil ();
  if NILP (get_char_property_and_overlay (pos, Qdisplay (), object,
					  &raw mut overlay))
    { return -1; }

  /* Look forward for the first character where the `display' property
     changes.  */
  if OVERLAYP (overlay)
    { return OVERLAY_END (overlay); }

  /* next-char-property-change doesn't support window OBJECTs.  */
  if WINDOWP (object)
    { object = (*w).contents; }
  pos = Fnext_single_char_property_change (pos, Qdisplay (), object, Qnil ());

  return XFIXNAT (pos);
}


/***********************************************************************
			    Fontification
 ***********************************************************************/

/* Handle changes in the `fontified' property of the current buffer by
   calling hook functions from Qfontification_functions to fontify
   regions of text.  */

unsafe extern "C" fn handle_fontified_prop (it: *mut it) -> prop_handled
{
  let mut prop: Lisp_Object = Qnil ();
  let mut pos: Lisp_Object = Qnil ();
  let mut handled: prop_handled = HANDLED_NORMALLY;

  if !NILP (Vmemory_full)
    { return handled; }

  /* Get the value of the `fontified' property at IT's current buffer
     position.  (The `fontified' property doesn't have a special
     meaning in strings.)  If the value is nil, call functions from
     Qfontification_functions.  */
  if !STRINGP ((*it).string)
      && (*it).s.is_null ()
      && !NILP (Vfontification_functions)
      && !(input_was_pending && redisplay_skip_fontification_on_input)
      && !NILP (Vrun_hooks)
      && { pos = make_fixnum (IT_CHARPOS! (*it));
	   prop = Fget_char_property (pos, Qfontified (), Qnil ());
	   /* Ignore the special cased nil value always present at EOB since
	      no amount of fontifying will be able to change it.  */
	   NILP (prop) && IT_CHARPOS! (*it) < Z! () }
    {
      let count: specpdl_ref = SPECPDL_INDEX ();
      let mut val: Lisp_Object;
      let obuf: *mut buffer = current_buffer;
      let begv: ptrdiff_t = BEGV! ();
      let zv: ptrdiff_t = ZV! ();
      let old_clip_changed: bool = buffer_bits! (clip_changed, current_buffer);
      let saved_inhibit_flag: bool
	= frame_bits! (inhibit_clear_image_cache, (*it).f);

      val = Vfontification_functions;
      specbind (Qfontification_functions (), Qnil ());

      eassert! ((*it).end_charpos == ZV! ());

      if buffer_bits! (long_line_optimizations_p, current_buffer)
	  && long_line_optimizations_region_size > 0
	{
	  let mut begv: ptrdiff_t = (*it).large_narrowing_begv;
	  let mut zv: ptrdiff_t = (*it).large_narrowing_zv;
	  let charpos: ptrdiff_t = IT_CHARPOS! (*it);
	  if charpos < begv || charpos > zv
	    {
	      begv = get_large_narrowing_begv (charpos);
	      zv = get_large_narrowing_zv (charpos);
	    }
	  if begv != BEG as ptrdiff_t || zv != Z! ()
	    {
	      labeled_narrow_to_region (make_fixnum (begv), make_fixnum (zv),
					Qlong_line_optimizations_in_fontification_functions ());
	    }
	}

      /* Don't allow Lisp that runs from 'fontification-functions'
	 clear our face and image caches behind our back.  */
      frame_bits! (inhibit_clear_image_cache, (*it).f, true);

      if !CONSP (val) || EQ (XCAR (val), Qlambda ())
	{ dsafe_call1 (val, pos); }
      else
	{
	  let mut fns: Lisp_Object;
	  let mut fn_: Lisp_Object;

	  fns = Qnil ();

	  while CONSP (val)
	    {
	      fn_ = XCAR (val);

	      if EQ (fn_, Qt ())
		{
		  /* A value of t indicates this hook has a local
		     binding; it means to run the global binding too.
		     In a global value, t should not occur.  If it
		     does, we must ignore it to avoid an endless
		     loop.  */
		  fns = Fdefault_value (Qfontification_functions ());
		  while CONSP (fns)
		    {
		      fn_ = XCAR (fns);
		      if !EQ (fn_, Qt ())
			{ dsafe_call1 (fn_, pos); }
		      fns = XCDR (fns);
		    }
		}
	      else
		{ dsafe_call1 (fn_, pos); }

	      val = XCDR (val);
	    }
	}

      frame_bits! (inhibit_clear_image_cache, (*it).f, saved_inhibit_flag);
      unbind_to (count, Qnil ());

      /* Fontification functions routinely call `save-restriction'.
	 Normally, this tags clip_changed, which can confuse redisplay
	 (see discussion in Bug#6671).  Since we don't perform any
	 special handling of fontification changes in the case where
	 `save-restriction' isn't called, there's no point doing so in
	 this case either.  So, if the buffer's restrictions are
	 actually left unchanged, reset clip_changed.  */
      if obuf == current_buffer
	{
	  if begv == BEGV! () && zv == ZV! ()
	    { buffer_bits! (clip_changed, current_buffer, old_clip_changed); }
	}
      /* There isn't much we can reasonably do to protect against
	 misbehaving fontification, but here's a fig leaf.  */
      else if BUFFER_LIVE_P (obuf)
	{ set_buffer_internal_1 (obuf); }

      /* The fontification code may have added/removed text.
	 It could do even a lot worse, but let's at least protect against
	 the most obvious case where only the text past `pos' gets changed',
	 as is/was done in grep.el where some escapes sequences are turned
	 into face properties (bug#7876).  */
      (*it).end_charpos = ZV! ();

      /* Return HANDLED_RECOMPUTE_PROPS only if function fontified
	 something.  This avoids an endless loop if they failed to
	 fontify the text for which reason ever.  */
      if !NILP (Fget_char_property (pos, Qfontified (), Qnil ()))
	{ handled = HANDLED_RECOMPUTE_PROPS; }
    }

  return handled;
}


/***********************************************************************
				Faces
 ***********************************************************************/

unsafe fn face_at_pos (it: *const it, attr_filter: lface_attribute_index) -> c_int
{
  let mut next_stop: ptrdiff_t = 0;

  if !STRINGP ((*it).string)
    {
      return face_at_buffer_position ((*it).w,
				      IT_CHARPOS! (*it),
				      &raw mut next_stop,
				      IT_CHARPOS! (*it)
				      + TEXT_PROP_DISTANCE_LIMIT as ptrdiff_t,
				      false, (*it).base_face_id,
				      attr_filter);
    }
  else
    {
      let mut base_face_id: c_int;
      let mut bufpos: ptrdiff_t;
      let mut i: c_int;
      let mut from_overlay: Lisp_Object
	= if (*it).current.overlay_string_index >= 0
	  { (*it).string_overlays [((*it).current.overlay_string_index
				    % OVERLAY_STRING_CHUNK_SIZE as ptrdiff_t) as usize] }
	  else { Qnil () };

      /* See if we got to this string directly or indirectly from
	 an overlay property.  That includes the before-string or
	 after-string of an overlay, strings in display properties
	 provided by an overlay, their text properties, etc.

	 FROM_OVERLAY is the overlay that brought us here, or nil if none.  */
      if ! NILP (from_overlay)
	{
	  i = (*it).sp - 1;
	  while i >= 0
	    {
	      if (*it).stack [i as usize].current.overlay_string_index >= 0
		{ from_overlay
		    = (*it).string_overlays [((*it).stack [i as usize].current.overlay_string_index
					      % OVERLAY_STRING_CHUNK_SIZE as ptrdiff_t) as usize]; }
	      else if ! NILP ((*it).stack [i as usize].from_overlay)
		{ from_overlay = (*it).stack [i as usize].from_overlay; }

	      if !NILP (from_overlay)
		{ break; }

	      i -= 1;
	    }
	}

      if ! NILP (from_overlay)
	{
	  bufpos = IT_CHARPOS! (*it);
	  /* For a string from an overlay, the base face depends
	     only on text properties and ignores overlays.  */
	  base_face_id
	    = face_for_overlay_string ((*it).w,
				       IT_CHARPOS! (*it),
				       &raw mut next_stop,
				       IT_CHARPOS! (*it)
				       + TEXT_PROP_DISTANCE_LIMIT as ptrdiff_t,
				       false,
				       from_overlay, attr_filter);
	}
      else
	{
	  bufpos = 0;

	  /* For strings from a `display' property, use the face at
	     IT's current buffer position as the base face to merge
	     with, so that overlay strings appear in the same face as
	     surrounding text, unless they specify their own faces.
	     For strings from wrap-prefix and line-prefix properties,
	     use the default face, possibly remapped via
	     Vface_remapping_alist.  */
	  /* Note that the fact that we use the face at _buffer_
	     position means that a 'display' property on an overlay
	     string will not inherit the face of that overlay string,
	     but will instead revert to the face of buffer text
	     covered by the overlay.  This is visible, e.g., when the
	     overlay specifies a box face, but neither the buffer nor
	     the display string do.  This sounds like a design bug,
	     but Emacs always did that since v21.1, so changing that
	     might be a big deal.  */
	  base_face_id = if it_bits! (string_from_prefix_prop_p, it)
	    { if !NILP (Vface_remapping_alist)
		{ lookup_basic_face ((*it).w, (*it).f, DEFAULT_FACE_ID) }
	      else { DEFAULT_FACE_ID } }
	    else { underlying_face_id (it) };
	}

      /* For strings displayed in a margin area via a 'display' property,
         use the realized 'margin' face as the base so that unspecified
         attributes (notably background) are inherited from 'margin'
         rather than from 'default' or the buffer face at point.  This
         allows packages to specify only a foreground color for a margin
         annotation and have the margin background fill in automatically.  */
      if it_bits! (string_from_display_prop_p, it)
	  && (*it).area != TEXT_AREA
	  && ! (*it).w.is_null ()
	{ base_face_id = lookup_basic_face ((*it).w, (*it).f, MARGIN_FACE_ID); }

      return face_at_string_position ((*it).w,
				      (*it).string,
				      IT_STRING_CHARPOS! (*it),
				      bufpos,
				      &raw mut next_stop,
				      base_face_id, false,
				      attr_filter);
    } /* !STRINGP (it->string) */
}


/* Set up iterator IT from face properties at its current position.
   Called from handle_stop.  */
unsafe extern "C" fn handle_face_prop (it: *mut it) -> prop_handled
{
  let count: specpdl_ref = SPECPDL_INDEX ();
  /* Don't allow the user to quit out of face-merging code, in case
     this is called when redisplaying a non-selected window, with
     point temporarily moved to window-point.  */
  specbind (Qinhibit_quit (), Qt ());
  let new_face_id: c_int = face_at_pos (it, 0);
  unbind_to (count, Qnil ());


  /* Is this a start of a run of characters with box face?
     Caveat: this can be called for a freshly initialized
     iterator; face_id is -1 in this case.  We know that the new
     face will not change until limit, i.e. if the new face has a
     box, all characters up to limit will have one.  But, as
     usual, we don't know whether limit is really the end.  */
  if new_face_id != (*it).face_id
    {
      let new_face: *mut face = FACE_FROM_ID ((*it).f, new_face_id);
      /* If it->face_id is -1, old_face below will be NULL, see
	 the definition of FACE_FROM_ID_OR_NULL.  This will happen
	 if this is the initial call that gets the face.  */
      let mut old_face: *mut face = FACE_FROM_ID_OR_NULL ((*it).f, (*it).face_id);

      /* If the value of face_id of the iterator is -1, we have to
	 look in front of IT's position and see whether there is a
	 face there that's different from new_face_id.  */
      if !STRINGP ((*it).string)
          && old_face.is_null ()
          && IT_CHARPOS! (*it) > BEG as ptrdiff_t
	{
	  let prev_face_id: c_int = face_before_it_pos! (it);

	  old_face = FACE_FROM_ID_OR_NULL ((*it).f, prev_face_id);
	}

      /* If the new face has a box, but the old face does not,
	 this is the start of a run of characters with box face,
	 i.e. this character has a shadow on the left side.  */
      (*it).face_id = new_face_id;
      /* Don't reset the start_of_box_run_p flag, only set it if
	 needed.  */
      if !(it_bits! (start_of_box_run_p, it) && !old_face.is_null ()
	   && face_bits! (box, old_face) as face_box_type != FACE_NO_BOX)
	{
	  it_bits! (start_of_box_run_p, it,
		    face_bits! (box, new_face) as face_box_type != FACE_NO_BOX
		    && (old_face.is_null ()
			|| face_bits! (box, old_face) as face_box_type == FACE_NO_BOX));
	}
      it_bits! (face_box_p, it,
		face_bits! (box, new_face) as face_box_type != FACE_NO_BOX);
    }

  return HANDLED_NORMALLY;
}


/* Return the ID of the face ``underlying'' IT's current position,
   which is in a string.  If the iterator is associated with a
   buffer, return the face at IT's current buffer position.
   Otherwise, use the iterator's base_face_id.  */

unsafe fn underlying_face_id (it: *const it) -> c_int
{
  let mut face_id: c_int = (*it).base_face_id;
  let mut i: c_int;

  eassert! (STRINGP ((*it).string));

  i = (*it).sp - 1;
  while i >= 0
    {
      if NILP ((*it).stack [i as usize].string)
	{ face_id = (*it).stack [i as usize].face_id; }
      i -= 1;
    }

  return face_id;
}

/* Compute the face one character before or after the current position
   of IT, in the visual order.  BEFORE_P means get the face
   in front (to the left in L2R paragraphs, to the right in R2L
   paragraphs) of IT's screen position.  Value is the ID of the face.  */

unsafe extern "C" fn face_before_or_after_it_pos (it: *mut it, before_p: bool) -> c_int
{
  let mut face_id: c_int;
  let mut limit: c_int;
  let mut next_check_charpos: ptrdiff_t = 0;
  let mut it_copy: it;
  let mut it_copy_data: *mut c_void = core::ptr::null_mut ();

  eassert! ((*it).s.is_null ());

  if STRINGP ((*it).string)
    {
      let mut bufpos: ptrdiff_t;
      let mut charpos: ptrdiff_t;
      let mut base_face_id: c_int;

      /* No face change past the end of the string (for the case we
	 are padding with spaces).  No face change before the string
	 start.  Ignore face changes before the first visible
	 character on this display line.  */
      if IT_STRING_CHARPOS! (*it) >= SCHARS ((*it).string)
	  || (IT_STRING_CHARPOS! (*it) == 0 && before_p)
	  || (*it).current_x <= (*it).first_visible_x
	{ return (*it).face_id; }

      if !it_bits! (bidi_p, it)
	{
	  /* Set charpos to the position before or after IT's current
	     position, in the logical order, which in the non-bidi
	     case is the same as the visual order.  */
	  if before_p
	    { charpos = IT_STRING_CHARPOS! (*it) - 1; }
	  else if (*it).what == IT_COMPOSITION
	    /* For composition, we must check the character after the
	       composition.  */
	    { charpos = IT_STRING_CHARPOS! (*it) + (*it).cmp_it.nchars as ptrdiff_t; }
	  else
	    { charpos = IT_STRING_CHARPOS! (*it) + 1; }
	}
      else
	{
	  /* With bidi iteration, the character before the current in
	     the visual order cannot be found by simple iteration,
	     because "reverse" reordering is not supported.  Instead,
	     we need to start from the string beginning and go all the
	     way to the current string position, remembering the
	     visually-previous position.  We need to start from the
	     string beginning for the character after the current as
	     well, since the iterator state in IT may have been
	     pushed, and the bidi cache is no longer coherent with the
	     string's text.  */
	  SAVE_IT! (it_copy, *it, it_copy_data);
	  IT_STRING_CHARPOS! (it_copy) = 0;
	  bidi_init_it (0, 0, FRAME_WINDOW_P (it_copy.f), &raw mut it_copy.bidi_it);
	  it_copy.bidi_it.scan_dir = 0;

	  loop
	    {
	      charpos = it_copy.bidi_it.charpos;
	      if charpos >= SCHARS ((*it).string)
		{ break; }
	      bidi_move_to_visually_next (&raw mut it_copy.bidi_it);
	      if !(it_copy.bidi_it.charpos != IT_STRING_CHARPOS! (*it)) { break; }
	    }

	  if !before_p
	    {
	      /* Set charpos to the string position of the character
		 that comes after IT's current position in the visual
		 order.  */
	      let mut n: c_int
		= if (*it).what == IT_COMPOSITION { (*it).cmp_it.nchars } else { 1 };
	      /* If this is the first string character,
		 bidi_move_to_visually_next will deliver character at
		 current position without moving, so we need to enlarge N.  */
	      if bidi_it_bits! (first_elt, &raw mut it_copy.bidi_it)
		{ n += 1; }
	      while n != 0
		{
		  n -= 1;
		  bidi_move_to_visually_next (&raw mut it_copy.bidi_it);
		}

	      charpos = it_copy.bidi_it.charpos;
	    }

	  RESTORE_IT! (it, it, it_copy_data);
	}
      eassert! (0 <= charpos && charpos <= SCHARS ((*it).string));

      if (*it).current.overlay_string_index >= 0
	{ bufpos = IT_CHARPOS! (*it); }
      else
	{ bufpos = 0; }

      base_face_id = underlying_face_id (it);

      /* Get the face for ASCII, or unibyte.  */
      face_id = face_at_string_position ((*it).w, (*it).string, charpos,
                                         bufpos, &raw mut next_check_charpos,
                                         base_face_id, false, 0);

      /* Correct the face for charsets different from ASCII.  Do it
	 for the multibyte case only.  The face returned above is
	 suitable for unibyte text if IT->string is unibyte.  */
      if STRING_MULTIBYTE ((*it).string)
	{
	  let pos1: text_pos = string_pos (charpos, (*it).string);
	  let p: *const u8 = SDATA ((*it).string).offset (BYTEPOS! (pos1) as isize);
	  let face: *mut face = FACE_FROM_ID ((*it).f, face_id);
	  let mut len: c_int = 0;
	  let c: c_int = check_char_and_length (p, &raw mut len);
	  face_id = FACE_FOR_CHAR ((*it).f, face, c, charpos, (*it).string);
	}
    }
  else
    {
      let mut pos: text_pos;

      if (IT_CHARPOS! (*it) >= ZV! () && !before_p)
	  || (IT_CHARPOS! (*it) <= BEGV! () && before_p)
	{ return (*it).face_id; }

      limit = (IT_CHARPOS! (*it) + TEXT_PROP_DISTANCE_LIMIT as ptrdiff_t) as c_int;
      pos = (*it).current.pos;

      if !it_bits! (bidi_p, it)
	{
	  if before_p
	    { DEC_TEXT_POS! (pos, it_bits! (multibyte_p, it)); }
	  else
	    {
	      if (*it).what == IT_COMPOSITION
		{
		  /* For composition, we must check the position after
		     the composition.  */
		  pos.charpos += (*it).cmp_it.nchars as ptrdiff_t;
		  pos.bytepos += (*it).cmp_it.nbytes as ptrdiff_t;
		}
	      else
		{ INC_TEXT_POS! (pos, it_bits! (multibyte_p, it)); }
	    }
	}
      else
	{
	  if before_p
	    {
	      let mut current_x: c_int;

	      /* With bidi iteration, the character before the current
		 in the visual order cannot be found by simple
		 iteration, because "reverse" reordering is not
		 supported.  Instead, we need to use the move_it_*
		 family of functions, and move to the previous
		 character starting from the beginning of the visual
		 line.  */
	      /* Ignore face changes before the first visible
		 character on this display line.  */
	      if (*it).current_x <= (*it).first_visible_x
		{ return (*it).face_id; }
	      SAVE_IT! (it_copy, *it, it_copy_data);
	      /* Implementation note: Since move_it_in_display_line
		 works in the iterator geometry, and thinks the first
		 character is always the leftmost, even in R2L lines,
		 we don't need to distinguish between the R2L and L2R
		 cases here.  */
	      current_x = it_copy.current_x;
	      move_it_vertically_backward (&raw mut it_copy, 0);
	      move_it_in_display_line (&raw mut it_copy, ZV! (), current_x - 1, MOVE_TO_X);
	      pos = it_copy.current.pos;
	      RESTORE_IT! (it, it, it_copy_data);
	    }
	  else
	    {
	      /* Set charpos to the buffer position of the character
		 that comes after IT's current position in the visual
		 order.  */
	      let mut n: c_int
		= if (*it).what == IT_COMPOSITION { (*it).cmp_it.nchars } else { 1 };

	      it_copy = *it;
	      /* If this is the first display element,
		 bidi_move_to_visually_next will deliver character at
		 current position without moving, so we need to enlarge N.  */
	      if bidi_it_bits! (first_elt, &raw mut (*it).bidi_it)
		{ n += 1; }
	      while n != 0
		{
		  n -= 1;
		  bidi_move_to_visually_next (&raw mut it_copy.bidi_it);
		}

	      SET_TEXT_POS! (pos,
			     it_copy.bidi_it.charpos, it_copy.bidi_it.bytepos);
	    }
	}
      eassert! (BEGV! () <= CHARPOS! (pos) && CHARPOS! (pos) <= ZV! ());

      /* Determine face for CHARSET_ASCII, or unibyte.  */
      face_id = face_at_buffer_position ((*it).w,
					 CHARPOS! (pos),
					 &raw mut next_check_charpos,
                                         limit as ptrdiff_t, false, -1, 0);

      /* Correct the face for charsets different from ASCII.  Do it
	 for the multibyte case only.  The face returned above is
	 suitable for unibyte text if current_buffer is unibyte.  */
      if it_bits! (multibyte_p, it)
	{
	  let c: c_int = FETCH_MULTIBYTE_CHAR (BYTEPOS! (pos));
	  let face: *mut face = FACE_FROM_ID ((*it).f, face_id);
	  face_id = FACE_FOR_CHAR ((*it).f, face, c, CHARPOS! (pos), Qnil ());
	}
    }

  return face_id;
}


/***********************************************************************
			    Invisible text
 ***********************************************************************/

/* Set up iterator IT from invisible properties at its current
   position.  Called from handle_stop.  */

unsafe extern "C" fn handle_invisible_prop (it: *mut it) -> prop_handled
{
  let mut handled: prop_handled = HANDLED_NORMALLY;
  let mut invis: c_int;
  let mut curpos: ptrdiff_t;
  let mut endpos: ptrdiff_t;
  let mut prop: Lisp_Object;
  let mut pos: Lisp_Object;
  let mut overlay: Lisp_Object = Qnil ();

  /* Get the value of the invisible text property at the current
     position.  Value will be nil if there is no such property.  */
  if STRINGP ((*it).string)
    {
      curpos = IT_STRING_CHARPOS! (*it);
      endpos = SCHARS ((*it).string);
      pos = make_fixnum (curpos);
      prop = Fget_text_property (pos, Qinvisible (), (*it).string);
    }
  else	/* buffer */
    {
      curpos = IT_CHARPOS! (*it);
      endpos = ZV! ();
      pos = make_fixnum (curpos);
      prop = get_char_property_and_overlay (pos, Qinvisible (), (*it).window,
					    &raw mut overlay);
    }

  /* Do we have anything to do here?  */
  invis = TEXT_PROP_MEANS_INVISIBLE! (prop);
  if invis == 0 || curpos >= (*it).end_charpos
    { return handled; }

  /* If not bidi, or the bidi iteration is at base paragraph level, we
     can use a faster method; otherwise we need to check invisibility
     of every character while bidi-iterating out of invisible text.  */
  let slow: bool
    = it_bits! (bidi_p, it) && ! BIDI_AT_BASE_LEVEL! (&raw mut (*it).bidi_it);
  /* Record whether we have to display an ellipsis for the
     invisible text.  */
  let mut display_ellipsis_p: bool = (invis == 2);

  handled = HANDLED_RECOMPUTE_PROPS;

  if slow
    {
      if bidi_it_bits! (first_elt, &raw mut (*it).bidi_it)
	  && (*it).bidi_it.charpos < endpos
	{ bidi_paragraph_init ((*it).paragraph_embedding, &raw mut (*it).bidi_it, true); }

      if STRINGP ((*it).string)
	{
	  let mut done: bool = false;
	  /* Bidi-iterate out of the invisible part of the string.  */
	  loop
	    {
	      bidi_move_to_visually_next (&raw mut (*it).bidi_it);
	      if (*it).bidi_it.charpos < 0 || (*it).bidi_it.charpos >= endpos
		{ done = true; }
	      else
		{
		  pos = make_fixnum ((*it).bidi_it.charpos);
		  prop = Fget_text_property (pos, Qinvisible (), (*it).string);
		  invis = TEXT_PROP_MEANS_INVISIBLE! (prop);
		  /* If there are adjacent invisible texts, don't lose
                     the second one's ellipsis.  */
		  if invis == 2
		    { display_ellipsis_p = true; }
		}
	      if !(!done && invis != 0) { break; }
	    }

	  if display_ellipsis_p
	    { it_bits! (ellipsis_p, it, true); }
	  IT_STRING_CHARPOS! (*it) = (*it).bidi_it.charpos;
	  IT_STRING_BYTEPOS! (*it) = (*it).bidi_it.bytepos;
	  if IT_STRING_BYTEPOS! (*it) >= endpos
	    {
	      /* The rest of the string is invisible.  If this is an
		 overlay string, proceed with the next overlay string
		 or whatever comes and return a character from there.  */
	      if (*it).current.overlay_string_index >= 0
		  && !display_ellipsis_p
		{
		  next_overlay_string (it);
		  /* Don't check for overlay strings when we just
		     finished processing them.  */
		  handled = HANDLED_OVERLAY_STRING_CONSUMED;
		}
	    }
	}
      else
	{
	  let mut done: bool = false;
	  /* Bidi-iterate out of the invisible text.  */
	  loop
	    {
	      bidi_move_to_visually_next (&raw mut (*it).bidi_it);
	      if (*it).bidi_it.charpos < BEGV! () || (*it).bidi_it.charpos >= endpos
		{ done = true; }
	      else
		{
		  pos = make_fixnum ((*it).bidi_it.charpos);
		  prop = Fget_char_property (pos, Qinvisible (), (*it).window);
		  invis = TEXT_PROP_MEANS_INVISIBLE! (prop);
		  /* If there are adjacent invisible texts, don't lose
                     the second one's ellipsis.  */
		  if invis == 2
		    { display_ellipsis_p = true; }
		}
	      if !(!done && invis != 0) { break; }
	    }

	  IT_CHARPOS! (*it) = (*it).bidi_it.charpos;
	  IT_BYTEPOS! (*it) = (*it).bidi_it.bytepos;
	  if display_ellipsis_p
            {
              /* Make sure that the glyphs of the ellipsis will get
                 correct `charpos' values.  See below for detailed
                 explanation why this is needed.  */
	      (*it).position.charpos = IT_CHARPOS! (*it) - 1;
	      (*it).position.bytepos = CHAR_TO_BYTE ((*it).position.charpos);
	    }
	  /* If there are before-strings at the start of invisible
	     text, and the text is invisible because of a text
	     property, arrange to show before-strings because 20.x did
	     it that way.  (If the text is invisible because of an
	     overlay property instead of a text property, this is
	     already handled in the overlay code.)  */
	  if NILP (overlay)
	      && get_overlay_strings (it, (*it).stop_charpos)
	    {
	      handled = HANDLED_RECOMPUTE_PROPS;
	      if (*it).sp > 0
		{
		  it_stack_bits! (display_ellipsis_p,
				  &raw mut (*it).stack [((*it).sp - 1) as usize],
				  display_ellipsis_p);
		  /* The call to get_overlay_strings above recomputes
		     it->stop_charpos, but it only considers changes
		     in properties and overlays beyond iterator's
		     current position.  This causes us to miss changes
		     that happen exactly where the invisible property
		     ended.  So we play it safe here and force the
		     iterator to check for potential stop positions
		     immediately after the invisible text.  Note that
		     if get_overlay_strings returns true, it
		     normally also pushed the iterator stack, so we
		     need to update the stop position in the slot
		     below the current one.  */
		  (*it).stack [((*it).sp - 1) as usize].stop_charpos
		    = CHARPOS! ((*it).stack [((*it).sp - 1) as usize].current.pos);
		}
	    }
	  else if display_ellipsis_p
            {
	      it_bits! (ellipsis_p, it, true);
	      /* Let the ellipsis display before
		 considering any properties of the following char.
		 Fixes jasonr@gnu.org 01 Oct 07 bug.  */
	      handled = HANDLED_RETURN;
            }
	}
    }
  else if STRINGP ((*it).string)
    {
      let mut end_charpos: Lisp_Object = pos;
      let mut limit: Lisp_Object;

      if invis != 0 && IT_STRING_CHARPOS! (*it) < (*it).end_charpos
	{
	  let len: ptrdiff_t = endpos;

	  /* Get the position at which the next visible text can be
	     found in IT->string, if any.  */
	  XSETINT! (limit, len);
	  loop
	    {
	      end_charpos
		= Fnext_single_property_change (end_charpos, Qinvisible (),
						(*it).string, limit);
	      /* Since LIMIT is always an integer, so should be the
		 value returned by Fnext_single_property_change.  */
	      eassert! (FIXNUMP (end_charpos));
	      if FIXNUMP (end_charpos)
		{
		  endpos = XFIXNAT (end_charpos);
		  prop = Fget_text_property (end_charpos, Qinvisible (), (*it).string);
		  invis = TEXT_PROP_MEANS_INVISIBLE! (prop);
		  if invis == 2
		    { display_ellipsis_p = true; }
		}
	      else /* Should never happen; but if it does, exit the loop.  */
		{ endpos = len; }
	      if !(invis != 0 && endpos < len) { break; }
	    }

	  if display_ellipsis_p
	    { it_bits! (ellipsis_p, it, true); }

	  if endpos < len
	    {
	      /* Text at END_CHARPOS is visible.  Move IT there.  */
	      let old: text_pos;
	      let oldpos: ptrdiff_t;

	      old = (*it).current.string_pos;
	      oldpos = CHARPOS! (old);
	      if it_bits! (bidi_p, it)
		{
		  if bidi_it_bits! (first_elt, &raw mut (*it).bidi_it)
		      && (*it).bidi_it.charpos < SCHARS ((*it).string)
		    {
		      bidi_paragraph_init ((*it).paragraph_embedding,
					   &raw mut (*it).bidi_it, true);
		    }
		  /* Bidi-iterate out of the invisible text.  */
		  loop
		    {
		      bidi_move_to_visually_next (&raw mut (*it).bidi_it);
		      if !(oldpos <= (*it).bidi_it.charpos
			   && (*it).bidi_it.charpos < endpos
			   && (*it).bidi_it.charpos < (*it).bidi_it.string.schars)
			{ break; }
		    }

		  IT_STRING_CHARPOS! (*it) = (*it).bidi_it.charpos;
		  IT_STRING_BYTEPOS! (*it) = (*it).bidi_it.bytepos;
		  if IT_STRING_CHARPOS! (*it) >= endpos
		    { (*it).prev_stop = endpos; }
		}
	      else
		{
		  IT_STRING_CHARPOS! (*it) = endpos;
		  compute_string_pos (&raw mut (*it).current.string_pos, old, (*it).string);
		}
	    }
	  else
	    {
	      /* The rest of the string is invisible.  If this is an
		 overlay string, proceed with the next overlay string
		 or whatever comes and return a character from there.  */
	      if (*it).current.overlay_string_index >= 0
		  && !display_ellipsis_p
		{
		  next_overlay_string (it);
		  /* Don't check for overlay strings when we just
		     finished processing them.  */
		  handled = HANDLED_OVERLAY_STRING_CONSUMED;
		}
	      else
		{
		  IT_STRING_CHARPOS! (*it) = SCHARS ((*it).string);
		  IT_STRING_BYTEPOS! (*it) = SBYTES ((*it).string);
		}
	    }
	}
    }
  else	/* we are iterating over buffer text at base paragraph level */
    {
      let mut newpos: ptrdiff_t;
      let mut next_stop: ptrdiff_t = 0;
      let mut tem: ptrdiff_t = curpos;
      let mut pos: Lisp_Object;

      /* If we are on invisible text, skip over it.  */
      if invis != 0 && curpos < (*it).end_charpos
	{
	  /* Loop skipping over invisible text.  The loop is left at
	     ZV or with IT on the first char being visible again.  */
	  loop
	    {
	      /* Try to skip some invisible text.  Return value is the
		 position reached which can be equal to where we start
		 if there is nothing invisible there.  This skips both
		 over invisible text properties and overlays with
		 invisible property.  */
	      newpos = skip_invisible (tem, &raw mut next_stop, ZV! (), (*it).window);

	      /* If we skipped nothing at all we weren't at invisible
		 text in the first place.  If everything to the end of
		 the buffer was skipped, end the loop.  */
	      if newpos == tem || newpos >= ZV! ()
		{ invis = 0; }
	      else
		{
		  /* We skipped some characters but not necessarily
		     all there are.  Check if we ended up on visible
		     text.  Fget_char_property returns the property of
		     the char before the given position, i.e. if we
		     get invis = 0, this means that the char at
		     newpos is visible.  */
		  pos = make_fixnum (newpos);
		  prop = Fget_char_property (pos, Qinvisible (), (*it).window);
		  invis = TEXT_PROP_MEANS_INVISIBLE! (prop);
		}

	      /* If we ended up on invisible text, proceed to
		 skip starting with next_stop.  */
	      if invis != 0
		{ tem = next_stop; }

              /* If there are adjacent invisible texts, don't lose the
                 second one's ellipsis.  */
              if invis == 2
                { display_ellipsis_p = true; }
	      if !(invis != 0) { break; }
	    }

	  /* The position newpos is now either ZV or on visible text.  */
	  if it_bits! (bidi_p, it)
	    {
	      let bpos: ptrdiff_t = CHAR_TO_BYTE (newpos);
	      let on_newline: bool
		= bpos == ZV_BYTE! ()
		  || FETCH_BYTE! (bpos) as c_int == b'\n' as c_int;
	      let after_newline: bool
		= newpos <= BEGV! ()
		  || FETCH_BYTE! (bpos - 1) as c_int == b'\n' as c_int;

	      /* If the invisible text ends on a newline or on a
		 character after a newline, we can avoid the costly,
		 character by character, bidi iteration to NEWPOS, and
		 instead simply reseat the iterator there.  That's
		 because all bidi reordering information is tossed at
		 the newline.  This is a big win for modes that hide
		 complete lines, like Outline, Org, etc.  */
	      if on_newline || after_newline
		{
		  let mut tpos: text_pos = std::mem::zeroed ();
		  let pdir: bidi_dir_t = (*it).bidi_it.paragraph_dir;

		  SET_TEXT_POS! (tpos, newpos, bpos);
		  reseat_1 (it, tpos, false);
		  /* If we reseat on a newline/ZV, we need to prep the
		     bidi iterator for advancing to the next character
		     after the newline/EOB, keeping the current paragraph
		     direction (so that PRODUCE_GLYPHS does TRT wrt
		     prepending/appending glyphs to a glyph row).  */
		  if on_newline
		    {
		      bidi_it_bits! (first_elt, &raw mut (*it).bidi_it, false);
		      (*it).bidi_it.paragraph_dir = pdir;
		      (*it).bidi_it.ch = if bpos == ZV_BYTE! () { -1 } else { b'\n' as c_int };
		      (*it).bidi_it.nchars = 1;
		      (*it).bidi_it.ch_len = 1;
		    }
		}
	      else	/* Must use the slow method.  */
		{
		  /* With bidi iteration, the region of invisible text
		     could start and/or end in the middle of a
		     non-base embedding level.  Therefore, we need to
		     skip invisible text using the bidi iterator,
		     starting at IT's current position, until we find
		     ourselves outside of the invisible text.
		     Skipping invisible text _after_ bidi iteration
		     avoids affecting the visual order of the
		     displayed text when invisible properties are
		     added or removed.  */
		  if bidi_it_bits! (first_elt, &raw mut (*it).bidi_it)
		      && (*it).bidi_it.charpos < ZV! ()
		    {
		      /* If we were `reseat'ed to a new paragraph,
			 determine the paragraph base direction.  We
			 need to do it now because
			 next_element_from_buffer may not have a
			 chance to do it, if we are going to skip any
			 text at the beginning, which resets the
			 FIRST_ELT flag.  */
		      bidi_paragraph_init ((*it).paragraph_embedding,
					   &raw mut (*it).bidi_it, true);
		    }
		  loop
		    {
		      bidi_move_to_visually_next (&raw mut (*it).bidi_it);
		      if !((*it).stop_charpos <= (*it).bidi_it.charpos
			   && (*it).bidi_it.charpos < newpos)
			{ break; }
		    }
		  IT_CHARPOS! (*it) = (*it).bidi_it.charpos;
		  IT_BYTEPOS! (*it) = (*it).bidi_it.bytepos;
		  /* If we overstepped NEWPOS, record its position in
		     the iterator, so that we skip invisible text if
		     later the bidi iteration lands us in the
		     invisible region again. */
		  if IT_CHARPOS! (*it) >= newpos
		    { (*it).prev_stop = newpos; }
		}
	    }
	  else
	    {
	      IT_CHARPOS! (*it) = newpos;
	      IT_BYTEPOS! (*it) = CHAR_TO_BYTE (newpos);
	    }

	  if display_ellipsis_p
            {
              /* Make sure that the glyphs of the ellipsis will get
                 correct `charpos' values.  If we would not update
                 it->position here, the glyphs would belong to the
                 last visible character _before_ the invisible
                 text, which confuses `set_cursor_from_row'.

                 We use the last invisible position instead of the
                 first because this way the cursor is always drawn on
                 the first "." of the ellipsis, whenever PT is inside
                 the invisible text.  Otherwise the cursor would be
                 placed _after_ the ellipsis when the point is after the
                 first invisible character.  */
	      if !STRINGP ((*it).object)
		{
		  (*it).position.charpos = newpos - 1;
		  (*it).position.bytepos = CHAR_TO_BYTE ((*it).position.charpos);
		}
	    }

	  /* If there are before-strings at the start of invisible
	     text, and the text is invisible because of a text
	     property, arrange to show before-strings because 20.x did
	     it that way.  (If the text is invisible because of an
	     overlay property instead of a text property, this is
	     already handled in the overlay code.)  */
	  if NILP (overlay)
	      && get_overlay_strings (it, (*it).stop_charpos)
	    {
	      handled = HANDLED_RECOMPUTE_PROPS;
	      if (*it).sp > 0
		{
		  it_stack_bits! (display_ellipsis_p,
				  &raw mut (*it).stack [((*it).sp - 1) as usize],
				  display_ellipsis_p);
		  /* The call to get_overlay_strings above recomputes
		     it->stop_charpos, but it only considers changes
		     in properties and overlays beyond iterator's
		     current position.  This causes us to miss changes
		     that happen exactly where the invisible property
		     ended.  So we play it safe here and force the
		     iterator to check for potential stop positions
		     immediately after the invisible text.  Note that
		     if get_overlay_strings returns true, it
		     normally also pushed the iterator stack, so we
		     need to update the stop position in the slot
		     below the current one.  */
		  (*it).stack [((*it).sp - 1) as usize].stop_charpos
		    = CHARPOS! ((*it).stack [((*it).sp - 1) as usize].current.pos);
		}
	    }
	  else if display_ellipsis_p
            {
	      it_bits! (ellipsis_p, it, true);
	      /* Let the ellipsis display before
		 considering any properties of the following char.
		 Fixes jasonr@gnu.org 01 Oct 07 bug.  */
	      handled = HANDLED_RETURN;
            }
	}
    }

  return handled;
}


/* Make iterator IT return `...' next.
   Replaces LEN characters from buffer.  */

unsafe extern "C" fn setup_for_ellipsis (it: *mut it, len: c_int)
{
  /* Use the display table definition for `...'.  Invalid glyphs
     will be handled by the method returning elements from dpvec.  */
  if ! (*it).dp.is_null () && VECTORP (DISP_INVIS_VECTOR ((*it).dp))
    {
      let v: *mut Lisp_Vector = XVECTOR (DISP_INVIS_VECTOR ((*it).dp));
      (*it).dpvec = (*v).contents.as_mut_ptr ();
      (*it).dpend = (*v).contents.as_mut_ptr ().offset ((*v).header.size);
    }
  else
    {
      /* Default `...'.  */
      (*it).dpvec = default_invis_vector.as_mut_ptr ();
      (*it).dpend = default_invis_vector.as_mut_ptr ().offset (3);
    }

  (*it).dpvec_char_len = len;
  (*it).current.dpvec_index = 0;
  (*it).dpvec_face_id = -1;

  /* Use IT->saved_face_id for the ellipsis, so that it has the same
     face as the preceding text.  IT->saved_face_id was set in
     handle_stop to the face of the preceding character, and will be
     different from IT->face_id only if the invisible text skipped in
     handle_invisible_prop has some non-default face on its first
     character.  We thus ignore the face of the invisible text when we
     display the ellipsis.  IT's face is restored in set_iterator_to_next.  */
  if (*it).saved_face_id >= 0
    { (*it).face_id = (*it).saved_face_id; }

  /* If the ellipsis represents buffer text, it means we advanced in
     the buffer, so we should no longer ignore overlay strings.  */
  if (*it).method == GET_FROM_BUFFER
    { it_bits! (ignore_overlay_strings_at_pos_p, it, false); }

  (*it).method = GET_FROM_DISPLAY_VECTOR;
  it_bits! (ellipsis_p, it, true);
}


unsafe fn find_display_property (disp: Lisp_Object, spec: Lisp_Object) -> Lisp_Object
{
  let mut elem: Lisp_Object = Qnil ();
  // C rebinds its `disp` parameter (`disp = XCDR (disp)`) and leaves `elem` unwritten on
  // the paths that return nil; the port keeps the same names (the `textprop.rs` shape) and
  // gives `elem` the nil initializer those paths imply.
  let mut disp = disp;
  /* C's three `return Qnil`/`goto found` exits: the port keeps the same statement order
     inside a labelled block whose tail is C's `found:` label (the `casefiddle.rs` shape).  */
  'found: {
  if NILP (disp)
    { return Qnil (); }
  /* We have a vector of display specs.  */
  if VECTORP (disp)
    {
      let mut i: ptrdiff_t = 0;
      while i < ASIZE (disp)
	{
	  elem = AREF (disp, i);
	  if CONSP (elem)
	      && CONSP (XCDR (elem))
	      && EQ (XCAR (elem), spec)
	    { break 'found; }
	  i += 1;
	}
      return Qnil ();
    }
  /* We have a list of display specs.  */
  else if CONSP (disp)
	   && CONSP (XCAR (disp))
    {
      while !NILP (disp)
	{
	  elem = XCAR (disp);
	  if CONSP (elem)
	      && CONSP (XCDR (elem))
	      && EQ (XCAR (elem), spec)
	    { break 'found; }

	  /* Check that we have a proper list before going to the next
	     element.  */
	  if CONSP (XCDR (disp))
	    { disp = XCDR (disp); }
	  else
	    { disp = Qnil (); }
	}
      return Qnil ();
    }
  /* A simple display spec.  */
  else if CONSP (disp)
	   && CONSP (XCDR (disp))
	   && EQ (XCAR (disp), spec)
    {
      elem = disp;
      break 'found;
    }

  return Qnil ();
  }

  /* found: */
  /* If the property value is a list of one element, just return the
     CAR. */
  if NILP (XCDR (XCDR (elem)))
    { return XCAR (XCDR (elem)); }
  else
    { return XCDR (elem); }
}

/* Return the value of 'display' text or overlay property PROP of
   character at CHARPOS in OBJECT.  Return nil if character at CHARPOS
   has no 'display' property or if the 'display' property of that
   character does not include PROP.  OBJECT can be a buffer or a window
   or a string.  */
unsafe fn get_display_property (charpos: ptrdiff_t, prop: Lisp_Object,
				object: Lisp_Object) -> Lisp_Object
{
  return find_display_property (Fget_char_property (make_fixnum (charpos),
						    Qdisplay (), object),
				prop);
}

/* Handle 'display' property '(min-width (WIDTH))' at CHARPOS in OBJECT.
   OBJECT can be a buffer (or nil, which means the current buffer) or a
   string.  WIDTH_SPEC is the value of min-width spec that we expect to
   process.  */
unsafe fn display_min_width (it: *mut it, charpos: ptrdiff_t,
			     object: Lisp_Object, width_spec: Lisp_Object)
{
  /* We're being called at the end of the `min-width' sequence,
     probably. */
  if !NILP ((*it).min_width_property)
      && !EQ (width_spec, (*it).min_width_property)
    {
      /* When called from display_string (i.e., the mode line),
	 we're being called with a string as the object, and we
	 may be called with many sub-strings belonging to the same
	 :propertize run.  */
      if (STRINGP (object)
	   && ((charpos == 0
		&& !EQ ((*it).min_width_property,
			get_display_property (0, Qmin_width (), object)))
	       || (charpos > 0
		   && EQ ((*it).min_width_property,
			  get_display_property (charpos - 1, Qmin_width (),
						object))))
	  /* In a buffer -- check that we're really right after the
	     sequence of characters covered by this `min-width'.  */
	  || (!STRINGP (object)
	      && charpos > BEGV! ()
	      && EQ ((*it).min_width_property,
		     get_display_property (charpos - 1, Qmin_width (), object))))
	{
	  let mut w: Lisp_Object = Qnil ();
	  let mut width: c_double = 0.0;
	  /* C 5802-5822's `#ifdef HAVE_WINDOW_SYSTEM` arm (the `FRAME_WINDOW_P (it->f)`
	     `FRAME_FONT`/`FACE_FROM_ID` pixel path that builds `list1 (make_int (width))`) is
	     dead on this target (P-08); recorded in docs/ports/xdisp.md.  */
	  {
	    calc_pixel_width_or_height (&raw mut width, it,
					XCAR ((*it).min_width_property),
					core::ptr::null_mut (), true,
					core::ptr::null_mut (), core::ptr::null_mut ());
	    width -= (((*it).current_x - (*it).min_width_start)
		      / FRAME_COLUMN_WIDTH ((*it).f)) as c_double;
	    if width > 0.0
		&& !((*it).line_wrap == TRUNCATE
		     && ((*it).current_x as c_double + width
			 > (*it).last_visible_x as c_double))
	      { w = make_int (width as intmax_t); }
	  }

	  /* Insert the stretch glyph.  */
	  if !NILP (w)
	    {
	      (*it).object = list3 (Qspace (), QCwidth (), w);
	      produce_stretch_glyph (it);
	      if (*it).area == TEXT_AREA
		{
		  (*it).current_x += (*it).pixel_width;

		  if (*it).continuation_lines_width != 0
		      && it_bits! (string_from_prefix_prop_p, it)
		    { (*it).wrap_prefix_width = (*it).current_x; }
		}
	    }
	  (*it).min_width_property = Qnil ();
	}
    }

  /* We're at the start of a `min-width' sequence -- record the
     position and the property, so that we can later see if we're at
     the end.  */
  if CONSP (width_spec)
    {
      if (!STRINGP (object)
	   && charpos == BEGV! ())
	  /* Mode line (see above).  */
	  || (STRINGP (object)
	      && charpos == 0
	      && !EQ ((*it).min_width_property,
		      get_display_property (0, Qmin_width (), object)))
	  /* Buffer.  */
	  || (!STRINGP (object)
	      && charpos > BEGV! ()
	      && !EQ (width_spec,
		      get_display_property (charpos - 1, Qmin_width (), object)))
	{
	  (*it).min_width_property = width_spec;
	  (*it).min_width_start = (*it).current_x;
	}
    }
}

DEFUN! ("get-display-property", Fget_display_property,
       Sget_display_property, 2, 4, 0,
       doc: "Get the value of the display specification SPEC at POSITION.\nSPEC is the car of the display specification to fetch, e.g. `height'.\n\nOBJECT is either a string or a buffer to fetch the specification from.\nIf omitted, OBJECT defaults to the current buffer.\n\nIf PROPERTIES is non-nil, look for value of SPEC in PROPERTIES instead\nof the properties at POSITION.  ",
       (position: Lisp_Object, spec: Lisp_Object, object: Lisp_Object,
	properties: Lisp_Object),
       {
  // C rebinds its `properties` parameter (`properties = Fget_text_property (…)`); the port
  // keeps the same name in a mutable binding (the `textprop.rs` shape).
  let mut properties = properties;

  if NILP (properties)
    { properties = Fget_text_property (position, Qdisplay (), object); }
  else
    { CHECK_LIST (properties); }

  return find_display_property (properties, spec);
});


/***********************************************************************
			    'display' property
 ***********************************************************************/

/* Set up iterator IT from `display' property at its current position.
   Called from handle_stop.
   We return HANDLED_RETURN if some part of the display property
   overrides the display of the buffer text itself.
   Otherwise we return HANDLED_NORMALLY.  */

unsafe extern "C" fn handle_display_prop (it: *mut it) -> prop_handled
{
  let mut propval: Lisp_Object;
  let mut object: Lisp_Object;
  /* C declares `overlay` and hands its address to `get_char_property_and_overlay`, which
     writes it; the port gives the out-parameter the nil initializer that path implies (the
     `compute_display_string_pos` shape).  */
  let mut overlay: Lisp_Object = Qnil ();
  let mut position: *mut text_pos;
  let mut bufpos: ptrdiff_t;
  /* Nonzero if some property replaces the display of the text itself.  */
  let mut display_replaced: c_int = 0;

  if STRINGP ((*it).string)
    {
      object = (*it).string;
      position = &raw mut (*it).current.string_pos;
      bufpos = CHARPOS! ((*it).current.pos);
    }
  else
    {
      XSETWINDOW! (object, (*it).w);
      position = &raw mut (*it).current.pos;
      bufpos = CHARPOS! (*position);
    }

  /* Reset those iterator values set from display property values.  */
  /* C's chain `it->slice.x = it->slice.y = it->slice.width = it->slice.height = Qnil`
     becomes one statement per member (the `init_iterator` precedent, C 3464/3936/3941).  */
  (*it).slice.x = Qnil ();
  (*it).slice.y = Qnil ();
  (*it).slice.width = Qnil ();
  (*it).slice.height = Qnil ();
  (*it).space_width = Qnil ();
  (*it).font_height = Qnil ();
  (*it).voffset = 0;

  /* We don't support recursive `display' properties, i.e. string
     values that have a string `display' property, that have a string
     `display' property etc.  */
  if !it_bits! (string_from_display_prop_p, it)
    { (*it).area = TEXT_AREA; }

  propval = get_char_property_and_overlay (make_fixnum ((*position).charpos),
					   Qdisplay (), object, &raw mut overlay);

  /* Rest of the code must have OBJECT be either a string or a buffer.  */
  let objwin: Lisp_Object = object;
  if !STRINGP ((*it).string)
    { object = (*(*it).w).contents; }

  /* Handle min-width ends. */
  if !NILP ((*it).min_width_property)
      && NILP (find_display_property (propval, Qmin_width ()))
    {
      let mut pos: ptrdiff_t = bufpos;
      let mut start: ptrdiff_t = BEGV! ();

      if STRINGP (object)
	{
	  pos = IT_STRING_CHARPOS! (*it);
	  start = 0;
	}
      if pos > start
	  /* If we are iterating over a string and display-stack level
	     is zero, this is a mode line or similar.  The case of
	     it->sp > 0 is handled in set_iterator_to_next.  */
	  || (STRINGP (object) && (*it).sp == 0)
	{ display_min_width (it, pos, objwin, Qnil ()); }
    }

  if NILP (propval)
    { return HANDLED_NORMALLY; }
  /* Now OVERLAY is the overlay that gave us this property, or nil
     if it was a text property.  */

  display_replaced = handle_display_spec (it, propval, object, overlay,
					  position, bufpos,
					  FRAME_WINDOW_P ((*it).f));
  return if display_replaced != 0 { HANDLED_RETURN } else { HANDLED_NORMALLY };
}

/* Subroutine of handle_display_prop.  Returns non-zero if the display
   specification in SPEC is a replacing specification, i.e. it would
   replace the text covered by `display' property with something else,
   such as an image or a display string.  If SPEC includes any kind or
   `(space ...) specification, the value is 2; this is used by
   compute_display_string_pos, which see.

   See handle_single_display_spec for documentation of arguments.
   FRAME_WINDOW_P is true if the window being redisplayed is on a
   GUI frame; this argument is used only if IT is NULL, see below.

   IT can be NULL, if this is called by the bidi reordering code
   through compute_display_string_pos, which see.  In that case, this
   function only examines SPEC, but does not otherwise "handle" it, in
   the sense that it doesn't set up members of IT from the display
   spec.  */

unsafe fn handle_display_spec (it: *mut it, spec: Lisp_Object, object: Lisp_Object,
			       overlay: Lisp_Object, position: *mut text_pos,
			       bufpos: ptrdiff_t, frame_window_p: bool) -> c_int
{
  let mut replacing: c_int = 0;
  let mut enable_eval: bool = true;
  // C rebinds its `spec` parameter (`spec = XCDR (spec)`); the port keeps the same name in
  // a mutable binding (the `textprop.rs` shape).
  let mut spec = spec;

  /* Support (disable-eval PROP) which is used by enriched.el.  */
  if CONSP (spec) && EQ (XCAR (spec), Qdisable_eval ())
    {
      enable_eval = false;
      spec = if CONSP (XCDR (spec)) { XCAR (XCDR (spec)) } else { Qnil () };
    }

  if CONSP (spec)
      /* Simple specifications.  */
      && !EQ (XCAR (spec), Qimage ())
      /* C 6016-6018's `#ifdef HAVE_XWIDGETS` arm (`&& !EQ (XCAR (spec), Qxwidget)`) is dead
	 on this target (P-08); recorded in docs/ports/xdisp.md.  */
      && !EQ (XCAR (spec), Qspace ())
      && !EQ (XCAR (spec), Qwhen ())
      && !EQ (XCAR (spec), Qslice ())
      && !EQ (XCAR (spec), Qspace_width ())
      && !EQ (XCAR (spec), Qheight ())
      && !EQ (XCAR (spec), Qraise ())
      /* Marginal area specifications.  */
      && !(CONSP (XCAR (spec)) && EQ (XCAR (XCAR (spec)), Qmargin ()))
      && !EQ (XCAR (spec), Qleft_fringe ())
      && !EQ (XCAR (spec), Qright_fringe ())
      && !EQ (XCAR (spec), Qmin_width ())
      && !NILP (XCAR (spec))
    {
      while CONSP (spec)
	{
	  let rv: c_int = handle_single_display_spec (it, XCAR (spec), object,
						      overlay, position, bufpos,
						      replacing, frame_window_p,
						      enable_eval);
	  if rv != 0
	    {
	      replacing = rv;
	      /* If some text in a string is replaced, `position' no
		 longer points to the position of `object'.  */
	      if it.is_null () || STRINGP (object)
		{ break; }
	    }
	  spec = XCDR (spec);
	}
    }
  else if VECTORP (spec)
    {
      let mut i: ptrdiff_t = 0;
      while i < ASIZE (spec)
	{
	  let rv: c_int = handle_single_display_spec (it, AREF (spec, i), object,
						      overlay, position, bufpos,
						      replacing, frame_window_p,
						      enable_eval);
	  if rv != 0
	    {
	      replacing = rv;
	      /* If some text in a string is replaced, `position' no
		 longer points to the position of `object'.  */
	      if it.is_null () || STRINGP (object)
		{ break; }
	    }
	  i += 1;
	}
    }
  else
    { replacing = handle_single_display_spec (it, spec, object, overlay, position,
					      bufpos, 0, frame_window_p,
					      enable_eval); }
  return replacing;
}

/* Value is the position of the end of the `display' property starting
   at START_POS in OBJECT.  */

unsafe fn display_prop_end (it: *mut it, object: Lisp_Object,
			    start_pos: text_pos) -> text_pos
{
  let mut end: Lisp_Object;
  /* C leaves `end_pos` to its two writers below; Rust has no partial initialisation, so the
     port zero-initialises it first (PORTING.md 1.5, the `bidi.rs`/`window.rs` shape).  */
  let mut end_pos: text_pos = core::mem::zeroed ();

  end = Fnext_single_char_property_change (make_fixnum (CHARPOS! (start_pos)),
					   Qdisplay (), object, Qnil ());
  CHARPOS! (end_pos) = XFIXNAT (end);
  if STRINGP (object)
    { compute_string_pos (&raw mut end_pos, start_pos, (*it).string); }
  else
    { BYTEPOS! (end_pos) = CHAR_TO_BYTE (XFIXNAT (end)); }

  return end_pos;
}

/* Set up IT from a single `display' property specification SPEC.  OBJECT
   is the object in which the `display' property was found.  *POSITION
   is the position in OBJECT at which the `display' property was found.
   BUFPOS is the buffer position of OBJECT (different from POSITION if
   OBJECT is not a buffer).  DISPLAY_REPLACED non-zero means that we
   previously saw a display specification which already replaced text
   display with something else, for example an image; we ignore such
   properties after the first one has been processed.

   OVERLAY is the overlay this `display' property came from,
   or nil if it was a text property.

   If SPEC is a `space' or `image' specification, and in some other
   cases too, set *POSITION to the position where the `display'
   property ends.

   If IT is NULL, only examine the property specification in SPEC, but
   don't set up IT.  In that case, FRAME_WINDOW_P means SPEC
   is intended to be displayed in a window on a GUI frame.

   Enable evaluation of Lisp forms only if ENABLE_EVAL_P is true.

   Value is non-zero if something was found which replaces the display
   of buffer or string text.  */

unsafe fn handle_single_display_spec (it: *mut it, spec: Lisp_Object, mut object: Lisp_Object,
				      overlay: Lisp_Object, position: *mut text_pos,
				      bufpos: ptrdiff_t, display_replaced: c_int,
				      frame_window_p: bool, enable_eval_p: bool) -> c_int
{
  let mut form: Lisp_Object;
  let mut location: Lisp_Object;
  let mut value: Lisp_Object;
  let mut start_pos: text_pos = *position;
  let mut itdata: *mut c_void = core::ptr::null_mut ();
  // C rebinds its `spec` parameter (`spec = XCDR (spec)`); the port keeps the same name in
  // a mutable binding (the `textprop.rs` shape).
  let mut spec = spec;

  /* If SPEC is a list of the form `(when FORM . VALUE)', evaluate FORM.
     If the result is non-nil, use VALUE instead of SPEC.  */
  form = Qt ();
  if CONSP (spec) && EQ (XCAR (spec), Qwhen ())
    {
      spec = XCDR (spec);
      if !CONSP (spec)
	{ return 0; }
      form = XCAR (spec);
      spec = XCDR (spec);
    }

  if !NILP (form) && !EQ (form, Qt ()) && !enable_eval_p
    { form = Qnil (); }
  if !NILP (form) && !EQ (form, Qt ())
    {
      let count: specpdl_ref = SPECPDL_INDEX ();

      /* Bind `object' to the object having the `display' property, a
	 buffer or string.  Bind `position' to the position in the
	 object where the property was found, and `buffer-position'
	 to the current position in the buffer.  */

      if NILP (object)
	{ XSETBUFFER! (object, current_buffer); }
      specbind (Qobject (), object);
      specbind (Qposition (), make_fixnum (CHARPOS! (*position)));
      specbind (Qbuffer_position (), make_fixnum (bufpos));
      /* Save and restore the bidi cache, since FORM could be crazy
	 enough to re-enter redisplay, e.g., by calling 'message'.  */
      itdata = bidi_shelve_cache ();
      form = dsafe_eval (form);
      bidi_unshelve_cache (itdata, false);
      form = unbind_to (count, form);
    }

  if NILP (form)
    { return 0; }

  /* Handle `(height HEIGHT)' specifications.  */
  if CONSP (spec)
      && EQ (XCAR (spec), Qheight ())
      && CONSP (XCDR (spec))
    {
      if !it.is_null ()
	{
	  if !FRAME_WINDOW_P ((*it).f)
	    { return 0; }

	  (*it).font_height = XCAR (XCDR (spec));
	  if !NILP ((*it).font_height)
	    {
	      let mut new_height: c_int = -1;

	      if CONSP ((*it).font_height)
		  && (EQ (XCAR ((*it).font_height), Qplus ())
		      || EQ (XCAR ((*it).font_height), Qminus ()))
		  && CONSP (XCDR ((*it).font_height))
		  && RANGED_FIXNUMP (0, XCAR (XCDR ((*it).font_height)),
				     INT_MAX as EMACS_INT)
		{
		  /* `(+ N)' or `(- N)' where N is an integer.  */
		  let mut steps: c_int = XFIXNUM (XCAR (XCDR ((*it).font_height))) as c_int;
		  if EQ (XCAR ((*it).font_height), Qplus ())
		    { steps = - steps; }
		  (*it).face_id = smaller_face ((*it).f, (*it).face_id, steps);
		}
	      else if FUNCTIONP ((*it).font_height) && enable_eval_p
		{
		  /* Call function with current height as argument.
		     Value is the new height.  */
		  let face: *mut face = FACE_FROM_ID ((*it).f, (*it).face_id);
		  let mut height: Lisp_Object;
		  itdata = bidi_shelve_cache ();
		  height = dsafe_call1 ((*it).font_height,
					(*face).lface[LFACE_HEIGHT_INDEX as usize]);
		  bidi_unshelve_cache (itdata, false);
		  if NUMBERP (height)
		    { new_height = XFLOATINT (height) as c_int; }
		}
	      else if NUMBERP ((*it).font_height)
		{
		  /* Value is a multiple of the canonical char height.  */
		  let f: *mut face;

		  f = FACE_FROM_ID ((*it).f,
				    lookup_basic_face ((*it).w, (*it).f, DEFAULT_FACE_ID));
		  /* C's `double` product narrowed into the `int` `new_height` (the
		     `window.rs` `POINT_TO_PIXEL` cast shape).  */
		  new_height = (XFLOATINT ((*it).font_height)
				* XFIXNUM ((*f).lface[LFACE_HEIGHT_INDEX as usize]) as f64)
			       as c_int;
		}
	      else if enable_eval_p
		{
		  /* Evaluate IT->font_height with `height' bound to the
		     current specified height to get the new height.  */
		  let count: specpdl_ref = SPECPDL_INDEX ();
		  let face: *mut face = FACE_FROM_ID ((*it).f, (*it).face_id);

		  specbind (Qheight (), (*face).lface[LFACE_HEIGHT_INDEX as usize]);
		  itdata = bidi_shelve_cache ();
		  value = dsafe_eval ((*it).font_height);
		  bidi_unshelve_cache (itdata, false);
		  value = unbind_to (count, value);

		  if NUMBERP (value)
		    { new_height = XFLOATINT (value) as c_int; }
		}

	      if new_height > 0
		{ (*it).face_id = face_with_height ((*it).f, (*it).face_id, new_height); }
	    }
	}

      return 0;
    }

  /* Handle `(space-width WIDTH)'.  */
  if CONSP (spec)
      && EQ (XCAR (spec), Qspace_width ())
      && CONSP (XCDR (spec))
    {
      if !it.is_null ()
	{
	  if !FRAME_WINDOW_P ((*it).f)
	    { return 0; }

	  value = XCAR (XCDR (spec));
	  if NUMBERP (value) && XFLOATINT (value) > 0.0
	    { (*it).space_width = value; }
	}

      return 0;
    }

  /* Handle `(min-width (WIDTH))'.  */
  if CONSP (spec)
      && EQ (XCAR (spec), Qmin_width ())
      && CONSP (XCDR (spec))
      && CONSP (XCAR (XCDR (spec)))
    {
      if !it.is_null ()
	{
	  let mut pos: ptrdiff_t = bufpos;

	  if STRINGP (object)
	    { pos = IT_STRING_CHARPOS! (*it); }
	  display_min_width (it, pos, object, XCAR (XCDR (spec)));
	}
      return 0;
    }

  /* Handle `(slice X Y WIDTH HEIGHT)'.  */
  if CONSP (spec)
      && EQ (XCAR (spec), Qslice ())
    {
      let mut tem: Lisp_Object;

      if !it.is_null ()
	{
	  if !FRAME_WINDOW_P ((*it).f)
	    { return 0; }

	  if { tem = XCDR (spec); CONSP (tem) }
	    {
	      (*it).slice.x = XCAR (tem);
	      if { tem = XCDR (tem); CONSP (tem) }
		{
		  (*it).slice.y = XCAR (tem);
		  if { tem = XCDR (tem); CONSP (tem) }
		    {
		      (*it).slice.width = XCAR (tem);
		      if { tem = XCDR (tem); CONSP (tem) }
			{ (*it).slice.height = XCAR (tem); }
		    }
		}
	    }
	}

      return 0;
    }

  /* Handle `(raise FACTOR)'.  */
  if CONSP (spec)
      && EQ (XCAR (spec), Qraise ())
      && CONSP (XCDR (spec))
    {
      if !it.is_null ()
	{
	  if !FRAME_WINDOW_P ((*it).f)
	    { return 0; }

	  /* C 6320-6330's `#ifdef HAVE_WINDOW_SYSTEM` arm (the `value = XCAR (XCDR (spec))`
	     `NUMBERP`/`FACE_FROM_ID`/`normal_char_height`/`FRAME_LINE_HEIGHT` computation of
	     `it->voffset`) is dead on this target (P-08).  */
	}

      return 0;
    }

  /* Don't handle the other kinds of display specifications
     inside a string that we got from a `display' property.  */
  if !it.is_null () && it_bits! (string_from_display_prop_p, it)
    { return 0; }

  /* Characters having this form of property are not displayed, so
     we have to find the end of the property.  */
  if !it.is_null ()
    {
      start_pos = *position;
      /* If the display property comes from an overlay, always use the
	 end of that overlay as the end of the property.  Since
	 display_prop_end will happily find another 'display' property
	 coming from some other overlay or text property on buffer
	 positions before this overlay's end, or overlays not specific
	 to this window, we need to ignore them, or else we risk
	 displaying this overlay's display string/image twice, or fail
	 to display text that should be.  */
      if !NILP (overlay)
	{
	  let mut ovendpos: ptrdiff_t = OVERLAY_END (overlay);

	  /* Some borderline-sane Lisp might call us with the current
	     buffer narrowed so that overlay-end is outside the
	     POINT_MIN..POINT_MAX region, which will then cause
	     various assertion violations and crashes down the road,
	     starting with pop_it when it will attempt to use POSITION
	     set below.  Prevent that.  */
	  ovendpos = clip_to_bounds (BEGV! (), ovendpos, ZV! ());

	  SET_TEXT_POS! (*position, ovendpos, CHAR_TO_BYTE (ovendpos));
	}
      else
	{ *position = display_prop_end (it, object, start_pos); }
    }
  value = Qnil ();

  /* Stop the scan at that end position--we assume that all
     text properties change there.  */
  if !it.is_null ()
    { (*it).stop_charpos = (*position).charpos; }

  /* Handle `(left-fringe BITMAP [FACE])'
     and `(right-fringe BITMAP [FACE])'.  */
  if CONSP (spec)
      && (EQ (XCAR (spec), Qleft_fringe ())
	  || EQ (XCAR (spec), Qright_fringe ()))
      && CONSP (XCDR (spec))
    {
      if !it.is_null ()
	{
	  if !FRAME_WINDOW_P ((*it).f)
	    /* If we return here, POSITION has been advanced
	       across the text with this property.  */
	    {
	      /* Synchronize the bidi iterator with POSITION.  This is
		 needed because we are not going to push the iterator
		 on behalf of this display property, so there will be
		 no pop_it call to do this synchronization for us.  */
	      if it_bits! (bidi_p, it)
		{
		  (*it).position = *position;
		  iterate_out_of_display_property (it);
		  *position = (*it).position;
		}
	      return 1;
	    }
	}
      else if !frame_window_p
	{ return 1; }

      /* C 6407-6473's `#ifdef HAVE_WINDOW_SYSTEM` arm (the `lookup_fringe_bitmap`,
	 `lookup_derived_face`, `push_it` and `IT_IMAGE`/`GET_FROM_IMAGE`
	 `left_user_fringe_bitmap`/`right_user_fringe_bitmap` setup) is dead on this target
	 (P-08).  */
      return 1;
    }

  /* Prepare to handle `((margin left-margin) ...)',
     `((margin right-margin) ...)' and `((margin nil) ...)'
     prefixes for display specifications.  */
  location = Qunbound ();
  if CONSP (spec) && CONSP (XCAR (spec))
    {
      let mut tem: Lisp_Object;

      value = XCDR (spec);
      if CONSP (value)
	{ value = XCAR (value); }

      tem = XCAR (spec);
      if EQ (XCAR (tem), Qmargin ())
	  && { tem = XCDR (tem);
	       tem = if CONSP (tem) { XCAR (tem) } else { Qnil () };
	       (NILP (tem)
		|| EQ (tem, Qleft_margin ())
		|| EQ (tem, Qright_margin ())) }
	{ location = tem; }
    }

  if BASE_EQ (location, Qunbound ())
    {
      location = Qnil ();
      value = spec;
    }

  /* After this point, VALUE is the property after any
     margin prefix has been stripped.  It must be a string,
     an image specification, or `(space ...)'.

     LOCATION specifies where to display: `left-margin',
     `right-margin' or nil.  */

  let valid_p: bool = (STRINGP (value)
		       /* C 6513-6516's `#ifdef HAVE_WINDOW_SYSTEM` arm (the
			  `FRAME_WINDOW_P`/`valid_image_p` disjunct) is dead on this target
			  (P-08).  */
		       || (CONSP (value) && EQ (XCAR (value), Qspace ()))
		       || ((if !it.is_null () { FRAME_WINDOW_P ((*it).f) }
			    else { frame_window_p })
			   && valid_xwidget_spec_p (value)));

  if valid_p && display_replaced == 0
    {
      let mut retval: c_int = 1;

      if it.is_null ()
	{
	  /* Callers need to know whether the display spec is any kind
	     of `(space ...)' spec that is about to affect text-area
	     display.  */
	  if CONSP (value) && EQ (XCAR (value), Qspace ()) && NILP (location)
	    { retval = 2; }
	  return retval;
	}

      /* We want the string to inherit the paragraph direction of the
	 parent object, so we need to calculate that if not yet done.  */
      let eob: ptrdiff_t = if BUFFERP (object) { ZV! () } else { (*it).end_charpos };
      if bidi_it_bits! (first_elt, &raw mut (*it).bidi_it)
	  && (*it).bidi_it.charpos < eob
	{ bidi_paragraph_init ((*it).paragraph_embedding, &raw mut (*it).bidi_it, true); }

      /* Save current settings of IT so that we can restore them
	 when we are finished with the glyph property value.  */
      push_it (it, position);
      (*it).from_overlay = overlay;
      it_bits! (from_disp_prop_p, it, true);

      if NILP (location)
	{ (*it).area = TEXT_AREA; }
      else
	{
	  if EQ (location, Qleft_margin ())
	    { (*it).area = LEFT_MARGIN_AREA; }
	  else
	    { (*it).area = RIGHT_MARGIN_AREA; }
	  /* Use the 'margin' face for displaying text and images
	     in the margins.  */
	  (*it).face_id = if NILP (Vface_remapping_alist)
			  { MARGIN_FACE_ID }
			  else { lookup_basic_face ((*it).w, (*it).f, MARGIN_FACE_ID) };
	}

      if STRINGP (value)
	{
	  (*it).string = value;
	  it_bits! (multibyte_p, it, STRING_MULTIBYTE ((*it).string));
	  (*it).current.overlay_string_index = -1;
	  /* C's chain `IT_STRING_CHARPOS (*it) = IT_STRING_BYTEPOS (*it) = 0` becomes one
	     statement per member (the `init_iterator` precedent).  */
	  IT_STRING_CHARPOS! (*it) = 0;
	  IT_STRING_BYTEPOS! (*it) = 0;
	  /* C's chain `it->end_charpos = it->string_nchars = SCHARS (it->string)`, same.  */
	  (*it).end_charpos = SCHARS ((*it).string);
	  (*it).string_nchars = SCHARS ((*it).string);
	  (*it).method = GET_FROM_STRING;
	  (*it).stop_charpos = 0;
	  (*it).prev_stop = 0;
	  (*it).base_level_stop = 0;
	  it_bits! (string_from_display_prop_p, it, true);
	  (*it).cmp_it.id = -1;
	  /* Say that we haven't consumed the characters with
	     `display' property yet.  The call to pop_it in
	     set_iterator_to_next will clean this up.  */
	  if BUFFERP (object)
	    { *position = start_pos; }

	  /* Force paragraph direction to be that of the parent object.
	     If the parent object's paragraph direction is not yet
	     determined (which shouldn not happen, since we called
	     bidi_paragraph_init above), default to L2R.  */
	  if it_bits! (bidi_p, it) && (*it).bidi_it.paragraph_dir == R2L
	    { (*it).paragraph_embedding = (*it).bidi_it.paragraph_dir; }
	  else
	    { (*it).paragraph_embedding = L2R; }

	  /* Set up the bidi iterator for this display string.  */
	  if it_bits! (bidi_p, it)
	    {
	      (*it).bidi_it.string.lstring = (*it).string;
	      (*it).bidi_it.string.s = core::ptr::null ();
	      (*it).bidi_it.string.schars = (*it).end_charpos;
	      (*it).bidi_it.string.bufpos = bufpos;
	      bidi_string_bits! (from_disp_str, &raw mut (*it).bidi_it.string, true);
	      bidi_string_bits! (unibyte, &raw mut (*it).bidi_it.string,
				 !it_bits! (multibyte_p, it));
	      (*it).bidi_it.w = (*it).w;
	      bidi_init_it (0, 0, FRAME_WINDOW_P ((*it).f), &raw mut (*it).bidi_it);
	    }
	}
      else if CONSP (value) && EQ (XCAR (value), Qspace ())
	{
	  (*it).method = GET_FROM_STRETCH;
	  (*it).object = value;
	  /* C's chain `*position = it->position = start_pos`, same.  */
	  (*it).position = start_pos;
	  *position = start_pos;
	  retval = 1 + ((*it).area == TEXT_AREA) as c_int;
	}
      else if valid_xwidget_spec_p (value)
	{
	  (*it).what = IT_XWIDGET;
	  (*it).method = GET_FROM_XWIDGET;
	  (*it).position = start_pos;
	  (*it).object = if NILP (object) { (*(*it).w).contents } else { object };
	  *position = start_pos;
	  (*it).xwidget = lookup_xwidget (value);
	}
      /* C 6620-6641's `#ifdef HAVE_WINDOW_SYSTEM` `else` arm (the `lookup_image`
	 `/`IT_IMAGE`/`GET_FROM_IMAGE` setup) is dead on this target (P-08).  */

      return retval;
    }

  /* Invalid property or property not supported.  Restore
     POSITION to what it was before.  */
  *position = start_pos;
  return 0;
}

/* Check if PROP is a display property value whose text should be
   treated as intangible.  OVERLAY is the overlay from which PROP
   came, or nil if it came from a text property.  CHARPOS and BYTEPOS
   specify the buffer position covered by PROP.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn display_prop_intangible_p (prop: Lisp_Object, overlay: Lisp_Object,
						    charpos: ptrdiff_t,
						    bytepos: ptrdiff_t) -> bool
{
  let frame_window_p: bool = FRAME_WINDOW_P (XFRAME (selected_frame));
  /* C leaves `position` to `SET_TEXT_POS` below; Rust has no partial initialisation, so the
     port zero-initialises it first (PORTING.md 1.5, the `bidi.rs`/`window.rs` shape).  */
  let mut position: text_pos = core::mem::zeroed ();

  SET_TEXT_POS! (position, charpos, bytepos);
  return handle_display_spec (core::ptr::null_mut (), prop, Qnil (), overlay,
			      &raw mut position, charpos, frame_window_p)
    != 0;
}

/* Return true if PROP is a display sub-property value containing STRING.

   Implementation note: this and the following function are really
   special cases of handle_display_spec and
   handle_single_display_spec, and should ideally use the same code.
   Until they do, these two pairs must be consistent and must be
   modified in sync.  */

unsafe fn single_display_spec_string_p (prop: Lisp_Object, string: Lisp_Object) -> bool
{
  // C rebinds its `prop` parameter (`prop = XCDR (prop)`); the port keeps the same name in
  // a mutable binding (the `textprop.rs` shape).
  let mut prop = prop;

  if EQ (string, prop)
    { return true; }

  /* Skip over `when FORM'.  */
  if CONSP (prop) && EQ (XCAR (prop), Qwhen ())
    {
      prop = XCDR (prop);
      if !CONSP (prop)
	{ return false; }
      /* Actually, the condition following `when' should be eval'ed,
	 like handle_single_display_spec does, and we should return
	 false if it evaluates to nil.  However, this function is
	 called only when the buffer was already displayed and some
	 glyph in the glyph matrix was found to come from a display
	 string.  Therefore, the condition was already evaluated, and
	 the result was non-nil, otherwise the display string wouldn't
	 have been displayed and we would have never been called for
	 this property.  Thus, we can skip the evaluation and assume
	 its result is non-nil.  */
      prop = XCDR (prop);
    }

  if CONSP (prop)
    /* Skip over `margin LOCATION'.  */
    {
      if EQ (XCAR (prop), Qmargin ())
	{
	  prop = XCDR (prop);
	  if !CONSP (prop)
	    { return false; }

	  prop = XCDR (prop);
	  if !CONSP (prop)
	    { return false; }
	}
    }

  return EQ (prop, string) || (CONSP (prop) && EQ (XCAR (prop), string));
}

/* Return true if STRING appears in the `display' property PROP.  */

unsafe fn display_prop_string_p (prop: Lisp_Object, string: Lisp_Object) -> bool
{
  // C rebinds its `prop` parameter (`prop = XCDR (prop)`); the port keeps the same name in
  // a mutable binding (the `textprop.rs` shape).
  let mut prop = prop;

  if CONSP (prop)
      && !EQ (XCAR (prop), Qwhen ())
      && !(CONSP (XCAR (prop)) && EQ (Qmargin (), XCAR (XCAR (prop))))
    {
      /* A list of sub-properties.  */
      while CONSP (prop)
	{
	  if single_display_spec_string_p (XCAR (prop), string)
	    { return true; }
	  prop = XCDR (prop);
	}
    }
  else if VECTORP (prop)
    {
      /* A vector of sub-properties.  */
      let mut i: ptrdiff_t = 0;
      while i < ASIZE (prop)
	{
	  if single_display_spec_string_p (AREF (prop, i), string)
	    { return true; }
	  i += 1;
	}
    }
  else
    { return single_display_spec_string_p (prop, string); }

  return false;
}

/* Look for STRING in overlays and text properties in the current
   buffer, between character positions FROM and TO (excluding TO).
   BACK_P means look back (in this case, TO is supposed to be
   less than FROM).
   Value is the first character position where STRING was found, or
   zero if it wasn't found before hitting TO.

   This function may only use code that doesn't eval because it is
   called asynchronously from note_mouse_highlight.  */

unsafe fn string_buffer_position_lim (string: Lisp_Object,
				      from: ptrdiff_t, to: ptrdiff_t,
				      back_p: bool) -> ptrdiff_t
{
  let mut limit: Lisp_Object;
  let mut prop: Lisp_Object;
  let mut pos: Lisp_Object;
  let mut found: bool = false;

  pos = make_fixnum (max (from, BEGV! ()));

  if !back_p	/* looking forward */
    {
      limit = make_fixnum (min (to, ZV! ()));
      while !found && !EQ (pos, limit)
	{
	  prop = Fget_char_property (pos, Qdisplay (), Qnil ());
	  if !NILP (prop) && display_prop_string_p (prop, string)
	    { found = true; }
	  else
	    { pos = Fnext_single_char_property_change (pos, Qdisplay (), Qnil (),
						       limit); }
	}
    }
  else		/* looking back */
    {
      limit = make_fixnum (max (to, BEGV! ()));
      while !found && !EQ (pos, limit)
	{
	  prop = Fget_char_property (pos, Qdisplay (), Qnil ());
	  if !NILP (prop) && display_prop_string_p (prop, string)
	    { found = true; }
	  else
	    { pos = Fprevious_single_char_property_change (pos, Qdisplay (), Qnil (),
							   limit); }
	}
    }

  return if found { XFIXNUM (pos) } else { 0 };
}

/* Determine which buffer position in current buffer STRING comes from.
   AROUND_CHARPOS is an approximate position where it could come from.
   Value is the buffer position or 0 if it couldn't be determined.

   This function is necessary because we don't record buffer positions
   in glyphs generated from strings (to keep struct glyph small).
   This function may only use code that doesn't eval because it is
   called asynchronously from note_mouse_highlight.  */

unsafe fn string_buffer_position (string: Lisp_Object, around_charpos: ptrdiff_t) -> ptrdiff_t
{
  let MAX_DISTANCE: c_int = 1000;
  let forward_limit: ptrdiff_t = min (around_charpos + MAX_DISTANCE as ptrdiff_t, ZV! ());
  let mut found: ptrdiff_t = string_buffer_position_lim (string, around_charpos,
							 forward_limit, false);

  if found == 0
    {
      let backward_limit: ptrdiff_t = max (around_charpos - MAX_DISTANCE as ptrdiff_t, BEGV! ());
      found = string_buffer_position_lim (string, around_charpos,
					  backward_limit, true);
    }
  return found;
}


/***********************************************************************
			`composition' property
 ***********************************************************************/

/* Set up iterator IT from `composition' property at its current
   position.  Called from handle_stop.  */

unsafe extern "C" fn handle_composition_prop (it: *mut it) -> prop_handled
{
  /* C leaves `prop`, `start` and `end` to `find_composition`; Rust cannot prove that the
     call has written them before the `&&` chain reads them, so the port gives them the
     values that path implies (PORTING.md 1.5, deviation 1's rule).  */
  let mut prop: Lisp_Object = Qnil ();
  let mut string: Lisp_Object;
  let mut pos: ptrdiff_t;
  let mut pos_byte: ptrdiff_t;
  let mut start: ptrdiff_t = 0;
  let mut end: ptrdiff_t = 0;

  if STRINGP ((*it).string)
    {
      let mut s: *mut u8;

      pos = IT_STRING_CHARPOS! (*it);
      pos_byte = IT_STRING_BYTEPOS! (*it);
      string = (*it).string;
      s = SDATA (string).offset (pos_byte);
      if STRING_MULTIBYTE (string)
	{ (*it).c = STRING_CHAR (s); }
      else
	{ (*it).c = *s as c_int; }
    }
  else
    {
      pos = IT_CHARPOS! (*it);
      pos_byte = IT_BYTEPOS! (*it);
      string = Qnil ();
      (*it).c = FETCH_CHAR (pos_byte);
    }

  /* If there's a valid composition and point is not inside of the
     composition (in the case that the composition is from the current
     buffer), draw a glyph composed from the composition components.  */
  if find_composition (pos, -1, &raw mut start, &raw mut end, &raw mut prop, string)
      && composition_valid_p (start, end, prop)
      && (STRINGP ((*it).string) || (PT! () <= start || PT! () >= end))
    {
      if start < pos
	/* As we can't handle this situation (perhaps font-lock added
	   a new composition), we just return here hoping that next
	   redisplay will detect this composition much earlier.  */
	{ return HANDLED_NORMALLY; }
      if start != pos
	{
	  if STRINGP ((*it).string)
	    { pos_byte = string_char_to_byte ((*it).string, start); }
	  else
	    { pos_byte = CHAR_TO_BYTE (start); }
	}
      (*it).cmp_it.id = get_composition_id (start, pos_byte, end - start,
					    prop, string);

      if (*it).cmp_it.id >= 0
	{
	  (*it).cmp_it.ch = -1;
	  (*it).cmp_it.nchars = COMPOSITION_LENGTH (prop) as c_int;
	  (*it).cmp_it.nglyphs = -1;
	}
    }

  return HANDLED_NORMALLY;
}


/***********************************************************************
			   Overlay strings
 ***********************************************************************/

/* The following structure is used to record overlay strings for
   later sorting in load_overlay_strings.  */

#[repr(C)]
#[derive(Clone, Copy)]
pub struct overlay_entry
{
  pub overlay: Lisp_Object,
  pub string: Lisp_Object,
  pub priority: EMACS_INT,
  pub after_string_p: bool,
}
/* clang against the frozen target: sizeof 32, align 8, `overlay`@0, `string`@8,
   `priority`@16, `after_string_p`@24.  */
const _: () = assert! (core::mem::size_of::<overlay_entry> () == 32);
const _: () = assert! (core::mem::offset_of! (overlay_entry, overlay) == 0);
const _: () = assert! (core::mem::offset_of! (overlay_entry, string) == 8);
const _: () = assert! (core::mem::offset_of! (overlay_entry, priority) == 16);
const _: () = assert! (core::mem::offset_of! (overlay_entry, after_string_p) == 24);


/* Set up iterator IT from overlay strings at its current position.
   Called from handle_stop.  */

unsafe extern "C" fn handle_overlay_change (it: *mut it) -> prop_handled
{
  if !STRINGP ((*it).string) && get_overlay_strings (it, 0)
    { return HANDLED_RECOMPUTE_PROPS; }
  else
    { return HANDLED_NORMALLY; }
}


/* Set up the next overlay string for delivery by IT, if there is an
   overlay string to deliver.  Called by set_iterator_to_next when the
   end of the current overlay string is reached.  If there are more
   overlay strings to display, IT->string and
   IT->current.overlay_string_index are set appropriately here.
   Otherwise IT->string is set to nil.  */

unsafe fn next_overlay_string (it: *mut it)
{
  (*it).current.overlay_string_index += 1;
  if (*it).current.overlay_string_index == (*it).n_overlay_strings
    {
      /* No more overlay strings.  Restore IT's settings to what
	 they were before overlay strings were processed, and
	 continue to deliver from current_buffer.  */

      it_bits! (ellipsis_p, it,
		it_stack_bits! (display_ellipsis_p,
				&raw mut (*it).stack [((*it).sp - 1) as usize]));
      pop_it (it);
      eassert! ((*it).sp > 0
		|| (NILP ((*it).string)
		    && (*it).method == GET_FROM_BUFFER
		    && (*it).stop_charpos >= BEGV! ()
		    && (*it).stop_charpos <= (*it).end_charpos));
      (*it).current.overlay_string_index = -1;
      (*it).n_overlay_strings = 0;
      /* If there's an empty display string on the stack, pop the
	 stack, to resync the bidi iterator with IT's position.  Such
	 empty strings are pushed onto the stack in
	 get_overlay_strings_1.  */
      if (*it).sp > 0 && STRINGP ((*it).string) && SCHARS ((*it).string) == 0
	{ pop_it (it); }

      /* Since we've exhausted overlay strings at this buffer
	 position, set the flag to ignore overlays until we move to
	 another position.  (The flag will be reset in
	 next_element_from_buffer.)  But don't do that if the overlay
	 strings were loaded at position other than the current one,
	 which could happen if we called pop_it above, or if the
	 overlay strings were loaded by handle_invisible_prop at the
	 beginning of invisible text.  */
      if (*it).overlay_strings_charpos == IT_CHARPOS! (*it)
	{ it_bits! (ignore_overlay_strings_at_pos_p, it, true); }

      /* If we're at the end of the buffer, record that we have
	 processed the overlay strings there already, so that
	 next_element_from_buffer doesn't try it again.  */
      if NILP ((*it).string)
	  && IT_CHARPOS! (*it) >= (*it).end_charpos
	  && (*it).overlay_strings_charpos >= (*it).end_charpos
	{ it_bits! (overlay_strings_at_end_processed_p, it, true); }
      /* Note: we reset overlay_strings_charpos only here, to make
	 sure the just-processed overlays were indeed at EOB.
	 Otherwise, overlays on text with invisible text property,
	 which are processed with IT's position past the invisible
	 text, might fool us into thinking the overlays at EOB were
	 already processed (linum-mode can cause this, for
	 example).  */
      (*it).overlay_strings_charpos = -1;
    }
  else
    {
      /* There are more overlay strings to process.  If
	 IT->current.overlay_string_index has advanced to a position
	 where we must load IT->overlay_strings with more strings, do
	 it.  We must load at the IT->overlay_strings_charpos where
	 IT->n_overlay_strings was originally computed; when invisible
	 text is present, this might not be IT_CHARPOS (Bug#7016).  */
      let i: c_int = ((*it).current.overlay_string_index
		      % OVERLAY_STRING_CHUNK_SIZE as ptrdiff_t) as c_int;

      if (*it).current.overlay_string_index != 0 && i == 0
	{ load_overlay_strings (it, (*it).overlay_strings_charpos); }

      /* Initialize IT to deliver display elements from the overlay
	 string.  */
      (*it).string = (*it).overlay_strings[i as usize];
      it_bits! (multibyte_p, it, STRING_MULTIBYTE ((*it).string));
      SET_TEXT_POS! ((*it).current.string_pos, 0, 0);
      (*it).method = GET_FROM_STRING;
      (*it).stop_charpos = 0;
      (*it).end_charpos = SCHARS ((*it).string);
      if (*it).cmp_it.stop_pos >= 0
	{ (*it).cmp_it.stop_pos = 0; }
      (*it).prev_stop = 0;
      (*it).base_level_stop = 0;

      /* Set up the bidi iterator for this overlay string.  */
      if it_bits! (bidi_p, it)
	{
	  (*it).bidi_it.string.lstring = (*it).string;
	  (*it).bidi_it.string.s = core::ptr::null ();
	  (*it).bidi_it.string.schars = SCHARS ((*it).string);
	  (*it).bidi_it.string.bufpos = (*it).overlay_strings_charpos;
	  bidi_string_bits! (from_disp_str, &raw mut (*it).bidi_it.string,
			     it_bits! (string_from_display_prop_p, it));
	  bidi_string_bits! (unibyte, &raw mut (*it).bidi_it.string,
			     !it_bits! (multibyte_p, it));
	  (*it).bidi_it.w = (*it).w;
	  bidi_init_it (0, 0, FRAME_WINDOW_P ((*it).f), &raw mut (*it).bidi_it);
	}
    }

  CHECK_IT (it);
}


/* Compare two overlay_entry structures E1 and E2.  Used as a
   comparison function for qsort in load_overlay_strings.  Overlay
   strings for the same position are sorted so that

   1. All after-strings come in front of before-strings, except
   when they come from the same overlay.

   2. Within after-strings, strings are sorted so that overlay strings
   from overlays with higher priorities come first.

   2. Within before-strings, strings are sorted so that overlay
   strings from overlays with higher priorities come last.

   Value is analogous to strcmp.  */


unsafe extern "C" fn compare_overlay_entries (e1: *const c_void, e2: *const c_void) -> c_int
{
  let entry1: *const overlay_entry = e1 as *const overlay_entry;
  let entry2: *const overlay_entry = e2 as *const overlay_entry;
  let mut result: c_int;

  if (*entry1).after_string_p != (*entry2).after_string_p
    {
      /* Let after-strings appear in front of before-strings if
	 they come from different overlays.  */
      if EQ ((*entry1).overlay, (*entry2).overlay)
	{ result = if (*entry1).after_string_p { 1 } else { -1 }; }
      else
	{ result = if (*entry1).after_string_p { -1 } else { 1 }; }
    }
  else if (*entry1).priority != (*entry2).priority
    {
      if (*entry1).after_string_p
	/* After-strings sorted in order of decreasing priority.  */
	{ result = if (*entry2).priority < (*entry1).priority { -1 } else { 1 }; }
      else
	/* Before-strings sorted in order of increasing priority.  */
	{ result = if (*entry1).priority < (*entry2).priority { -1 } else { 1 }; }
    }
  else
    { result = 0; }

  return result;
}


/* Load the vector IT->overlay_strings with overlay strings from IT's
   current buffer position, or from CHARPOS if that is > 0.  Set
   IT->n_overlays to the total number of overlay strings found.

   Overlay strings are processed OVERLAY_STRING_CHUNK_SIZE strings at
   a time.  On entry into load_overlay_strings,
   IT->current.overlay_string_index gives the number of overlay
   strings that have already been loaded by previous calls to this
   function.

   IT->add_overlay_start contains an additional overlay start
   position to consider for taking overlay strings from, if non-zero.
   This position comes into play when the overlay has an `invisible'
   property, and both before and after-strings.  When we've skipped to
   the end of the overlay, because of its `invisible' property, we
   nevertheless want its before-string to appear.
   IT->add_overlay_start will contain the overlay start position
   in this case.

   Overlay strings are sorted so that after-string strings come in
   front of before-string strings.  Within before and after-strings,
   strings are sorted by overlay priority.  See also function
   compare_overlay_entries.  */

unsafe fn load_overlay_strings (it: *mut it, mut charpos: ptrdiff_t)
{
  let mut n: ptrdiff_t = 0;
  /* C leaves `entriesbuf` uninitialised; Rust has no partial initialisation, so the port
     zero-initialises it (PORTING.md 1.5, deviation 43's `values_here` shape).  */
  let mut entriesbuf: [overlay_entry; 20] = core::mem::zeroed ();
  let mut size: ptrdiff_t = countof! (entriesbuf) as ptrdiff_t;
  let mut entries: *mut overlay_entry = entriesbuf.as_mut_ptr ();
  let mut node: *mut itree_node;

  USE_SAFE_ALLOCA! (sa_avail, sa_count);

  if charpos <= 0
    { charpos = IT_CHARPOS! (*it); }

  /* Append the overlay string STRING of overlay OVERLAY to vector
     `entries' which has size `size' and currently contains `n'
     elements.  AFTER_P means STRING is an after-string of
     OVERLAY.  */
  /* C's `#define`d `do { … } while (false)` statement macro (C 7121-7148, `#undef`ed at
     C 7150) becomes this function-scoped `macro_rules!` (PORTING.md P-37); the three
     places it mutates — `entries`, `n`, `size` — are passed in because Rust's macro
     hygiene does not let the body see the caller's locals (the `SAFE_NALLOCA!` shape).  */
  macro_rules! RECORD_OVERLAY_STRING {
    ($overlay:expr, $string:expr, $after_p:expr, $entries:ident, $n:ident,
     $size:ident) => {{
      let mut priority: Lisp_Object;

      if $n == $size
	{
	  let old: *mut overlay_entry = $entries;
	  SAFE_NALLOCA! (sa_avail, $entries, 2, $size);
	  memcpy ($entries as *mut c_void, old as *const c_void,
		  ($size * core::mem::size_of::<overlay_entry> () as ptrdiff_t) as usize);
	  $size *= 2;
	}

      (*$entries.offset ($n)).string = $string;
      (*$entries.offset ($n)).overlay = $overlay;
      priority = Foverlay_get ($overlay, Qpriority ());
      (*$entries.offset ($n)).priority
	= if FIXNUMP (priority) { XFIXNUM (priority) } else { 0 };
      (*$entries.offset ($n)).after_string_p = $after_p;
      $n += 1;
    }};
  }

  /* Process overlays.  */
  /* C: `ITREE_FOREACH (node, current_buffer->overlays, charpos - 1, charpos + 1,
     DESCENDING)` — itree.h 166-176's expansion, written out at the use site (P-33).  */
  if (*current_buffer).overlays.is_null ()
    { }
  else
    {
      let mut itree_local_iter_: itree_iterator = core::mem::zeroed ();
      let itree_iter_: *mut itree_iterator
	= itree_iterator_start (&raw mut itree_local_iter_, (*current_buffer).overlays,
				charpos - 1, charpos + 1, ITREE_DESCENDING);
      while { node = itree_iterator_next (itree_iter_); !node.is_null () }
	{
	  let overlay: Lisp_Object = (*node).data;
	  eassert! (OVERLAYP (overlay));
	  let start: ptrdiff_t = (*node).begin;
	  let end: ptrdiff_t = (*node).end;

	  /* Skip this overlay if it doesn't start or end at IT's current
	     position.  */
	  if end != charpos && start != charpos
	    { continue; }

	  /* Skip this overlay if it doesn't apply to IT->w.  */
	  let window: Lisp_Object = Foverlay_get (overlay, Qwindow ());
	  if WINDOWP (window) && XWINDOW (window) != (*it).w
	    { continue; }

	  /* If the text ``under'' the overlay is invisible, both before-
	     and after-strings from this overlay are visible; start and
	     end position are indistinguishable.  */
	  let invisible: Lisp_Object = Foverlay_get (overlay, Qinvisible ());
	  let invis: c_int = TEXT_PROP_MEANS_INVISIBLE! (invisible);

	  /* If overlay has a non-empty before-string, record it.  */
	  let mut str: Lisp_Object;
	  if (start == charpos || (end == charpos && invis != 0))
	      && { str = Foverlay_get (overlay, Qbefore_string ()); STRINGP (str) }
	      && SCHARS (str) != 0
	    { RECORD_OVERLAY_STRING! (overlay, str, false, entries, n, size); }

	  /* If overlay has a non-empty after-string, record it.  */
	  if (end == charpos || (start == charpos && invis != 0))
	      && { str = Foverlay_get (overlay, Qafter_string ()); STRINGP (str) }
	      && SCHARS (str) != 0
	    { RECORD_OVERLAY_STRING! (overlay, str, true, entries, n, size); }
	}
    }

  /* Sort entries.  */
  if n > 1
    { qsort (entries as *mut c_void, n as usize,
	     core::mem::size_of::<overlay_entry> (), Some (compare_overlay_entries)); }

  /* Record number of overlay strings, and where we computed it.  */
  (*it).n_overlay_strings = n;
  (*it).overlay_strings_charpos = charpos;

  /* IT->current.overlay_string_index is the number of overlay strings
     that have already been consumed by IT.  Copy some of the
     remaining overlay strings to IT->overlay_strings.  */
  let mut j: ptrdiff_t = (*it).current.overlay_string_index;
  let mut i: ptrdiff_t = 0;
  while i < OVERLAY_STRING_CHUNK_SIZE as ptrdiff_t && j < n
    {
      (*it).overlay_strings[i as usize] = (*entries.offset (j)).string;
      (*it).string_overlays[i as usize] = (*entries.offset (j)).overlay;
      i += 1;
      j += 1;
    }

  CHECK_IT (it);
  SAFE_FREE! (sa_count);
}


/* Get the first chunk of overlay strings at IT's current buffer
   position, or at CHARPOS if that is > 0.  Value is true if at
   least one overlay string was found.  */

unsafe fn get_overlay_strings_1 (it: *mut it, charpos: ptrdiff_t,
				 compute_stop_p: bool) -> bool
{
  /* Get the first OVERLAY_STRING_CHUNK_SIZE overlay strings to
     process.  This fills IT->overlay_strings with strings, and sets
     IT->n_overlay_strings to the total number of strings to process.
     IT->pos.overlay_string_index has to be set temporarily to zero
     because load_overlay_strings needs this; it must be set to -1
     when no overlay strings are found because a zero value would
     indicate a position in the first overlay string.  */
  (*it).current.overlay_string_index = 0;
  load_overlay_strings (it, charpos);

  /* If we found overlay strings, set up IT to deliver display
     elements from the first one.  Otherwise set up IT to deliver
     from current_buffer.  */
  if (*it).n_overlay_strings != 0
    {
      /* Make sure we know settings in current_buffer, so that we can
	 restore meaningful values when we're done with the overlay
	 strings.  */
      if compute_stop_p
	{ compute_stop_pos (it); }
      eassert! ((*it).face_id >= 0);

      /* Save IT's settings.  They are restored after all overlay
	 strings have been processed.  */
      eassert! (!compute_stop_p || (*it).sp == 0);

      /* We want the string to inherit the paragraph direction of the
	 parent object, so we need to calculate that if not yet done.  */
      if bidi_it_bits! (first_elt, &raw mut (*it).bidi_it) && (*it).bidi_it.charpos < ZV! ()
	{ bidi_paragraph_init ((*it).paragraph_embedding, &raw mut (*it).bidi_it, true); }

      /* When called from handle_stop, there might be an empty display
	 string loaded.  In that case, don't bother saving it.  But
	 don't use this optimization with the bidi iterator, since we
	 need the corresponding pop_it call to resync the bidi
	 iterator's position with IT's position, after we are done
	 with the overlay strings.  (The corresponding call to pop_it
	 in case of an empty display string is in
	 next_overlay_string.)  */
      if !(!it_bits! (bidi_p, it)
	    && STRINGP ((*it).string) && SCHARS ((*it).string) == 0)
	{ push_it (it, core::ptr::null_mut ()); }

      /* Set up IT to deliver display elements from the first overlay
	 string.  */
      IT_STRING_CHARPOS! (*it) = 0;
      IT_STRING_BYTEPOS! (*it) = 0;
      (*it).string = (*it).overlay_strings[0];
      (*it).from_overlay = Qnil ();
      (*it).stop_charpos = 0;
      eassert! (STRINGP ((*it).string));
      (*it).end_charpos = SCHARS ((*it).string);
      (*it).prev_stop = 0;
      (*it).base_level_stop = 0;
      it_bits! (multibyte_p, it, STRING_MULTIBYTE ((*it).string));
      (*it).method = GET_FROM_STRING;
      it_bits! (from_disp_prop_p, it, false);
      (*it).cmp_it.id = -1;

      /* Force paragraph direction to be that of the parent
	 buffer.  */
      if it_bits! (bidi_p, it) && (*it).bidi_it.paragraph_dir == R2L
	{ (*it).paragraph_embedding = (*it).bidi_it.paragraph_dir; }
      else
	{ (*it).paragraph_embedding = L2R; }

      /* Set up the bidi iterator for this overlay string.  */
      if it_bits! (bidi_p, it)
	{
	  let pos: ptrdiff_t
	    = if charpos > 0 { charpos } else { IT_CHARPOS! (*it) };

	  (*it).bidi_it.string.lstring = (*it).string;
	  (*it).bidi_it.string.s = core::ptr::null ();
	  (*it).bidi_it.string.schars = SCHARS ((*it).string);
	  (*it).bidi_it.string.bufpos = pos;
	  bidi_string_bits! (from_disp_str, &raw mut (*it).bidi_it.string,
			     it_bits! (string_from_display_prop_p, it));
	  bidi_string_bits! (unibyte, &raw mut (*it).bidi_it.string,
			     !it_bits! (multibyte_p, it));
	  (*it).bidi_it.w = (*it).w;
	  bidi_init_it (0, 0, FRAME_WINDOW_P ((*it).f), &raw mut (*it).bidi_it);
	}
      return true;
    }

  (*it).current.overlay_string_index = -1;
  return false;
}

unsafe fn get_overlay_strings (it: *mut it, charpos: ptrdiff_t) -> bool
{
  (*it).string = Qnil ();
  (*it).method = GET_FROM_BUFFER;

  get_overlay_strings_1 (it, charpos, true);

  CHECK_IT (it);

  /* Value is true if we found at least one overlay string.  */
  return STRINGP ((*it).string);
}


/***********************************************************************
		      Saving and restoring state
 ***********************************************************************/

/* Save current settings of IT on IT->stack.  Called, for example,
   before setting up IT for an overlay string, to be able to restore
   IT's settings to what they were after the overlay string has been
   processed.  If POSITION is non-NULL, it is the position to save on
   the stack instead of IT->position.  */

unsafe fn push_it (it: *mut it, position: *mut text_pos)
{
  let p: *mut iterator_stack_entry;

  eassert! ((*it).sp < IT_STACK_SIZE);
  p = (*it).stack.as_mut_ptr ().offset ((*it).sp as isize);

  (*p).stop_charpos = (*it).stop_charpos;
  (*p).prev_stop = (*it).prev_stop;
  (*p).base_level_stop = (*it).base_level_stop;
  (*p).cmp_it = (*it).cmp_it;
  eassert! ((*it).face_id >= 0);
  (*p).face_id = (*it).face_id;
  (*p).string = (*it).string;
  (*p).method = (*it).method;
  (*p).from_overlay = (*it).from_overlay;
  match (*p).method
    {
      GET_FROM_IMAGE =>
	{
	  (*p).u.image.object = (*it).object;
	  (*p).u.image.image_id = (*it).image_id;
	  (*p).u.image.slice = (*it).slice;
	}
      GET_FROM_STRETCH =>
	{ (*p).u.stretch.object = (*it).object; }
      GET_FROM_XWIDGET =>
	{ (*p).u.xwidget.object = (*it).object; }
      GET_FROM_BUFFER | GET_FROM_DISPLAY_VECTOR | GET_FROM_STRING | GET_FROM_C_STRING =>
	{ }
      _ =>
	{ emacs_abort (); }
    }
  (*p).position = if !position.is_null () { *position } else { (*it).position };
  (*p).current = (*it).current;
  (*p).end_charpos = (*it).end_charpos;
  /* C narrows `it->string_nchars` (a `ptrdiff_t`) into the entry's `int string_nchars`.  */
  (*p).string_nchars = (*it).string_nchars as c_int;
  (*p).area = (*it).area;
  it_stack_bits! (multibyte_p, p, it_bits! (multibyte_p, it));
  it_stack_bits! (avoid_cursor_p, p, it_bits! (avoid_cursor_p, it));
  (*p).space_width = (*it).space_width;
  (*p).font_height = (*it).font_height;
  (*p).voffset = (*it).voffset;
  it_stack_bits! (string_from_display_prop_p, p,
		  it_bits! (string_from_display_prop_p, it));
  it_stack_bits! (string_from_prefix_prop_p, p,
		  it_bits! (string_from_prefix_prop_p, it));
  it_stack_bits! (display_ellipsis_p, p, false);
  (*p).line_wrap = (*it).line_wrap;
  it_stack_bits! (bidi_p, p, it_bits! (bidi_p, it));
  (*p).paragraph_embedding = (*it).paragraph_embedding;
  it_stack_bits! (from_disp_prop_p, p, it_bits! (from_disp_prop_p, it));
  (*it).sp += 1;

  /* Save the state of the bidi iterator as well. */
  if it_bits! (bidi_p, it)
    { bidi_push_it (&raw mut (*it).bidi_it); }
}

unsafe fn iterate_out_of_display_property (it: *mut it)
{
  let buffer_p: bool = !STRINGP ((*it).string);
  let eob: ptrdiff_t = if buffer_p { ZV! () } else { (*it).end_charpos };
  let bob: ptrdiff_t = if buffer_p { BEGV! () } else { 0 };

  eassert! (eob >= CHARPOS! ((*it).position) && CHARPOS! ((*it).position) >= bob);

  /* Maybe initialize paragraph direction.  If we are at the beginning
     of a new paragraph, next_element_from_buffer may not have had a
     chance to do that.  */
  if bidi_it_bits! (first_elt, &raw mut (*it).bidi_it) && (*it).bidi_it.charpos < eob
    { bidi_paragraph_init ((*it).paragraph_embedding, &raw mut (*it).bidi_it, true); }
  /* prev_stop can be zero, so check against BEGV as well.  */
  while (*it).bidi_it.charpos >= bob
	 && (*it).prev_stop <= (*it).bidi_it.charpos
	 && (*it).bidi_it.charpos < CHARPOS! ((*it).position)
	 && (*it).bidi_it.charpos < eob
    { bidi_move_to_visually_next (&raw mut (*it).bidi_it); }
  /* Record the stop_pos we just crossed, for when we cross it
     back, maybe.  */
  if (*it).bidi_it.charpos > CHARPOS! ((*it).position)
    { (*it).prev_stop = CHARPOS! ((*it).position); }
  /* If we ended up not where pop_it put us, resync IT's
     positional members with the bidi iterator. */
  if (*it).bidi_it.charpos != CHARPOS! ((*it).position)
    { SET_TEXT_POS! ((*it).position, (*it).bidi_it.charpos, (*it).bidi_it.bytepos); }
  if buffer_p
    { (*it).current.pos = (*it).position; }
  else
    { (*it).current.string_pos = (*it).position; }
}

/* Restore the IT->face_box_p flag, since it could have been
   overwritten by the face of the object that we just finished
   displaying.  Also, set the IT->start_of_box_run_p flag if the
   change in faces requires that.  */
unsafe fn restore_face_box_flags (it: *mut it, prev_face_id: c_int)
{
  let face: *mut face = FACE_FROM_ID_OR_NULL ((*it).f, (*it).face_id);

  if !face.is_null ()
    {
      let prev_face: *mut face = FACE_FROM_ID_OR_NULL ((*it).f, prev_face_id);

      if !(it_bits! (start_of_box_run_p, it)
	   && !prev_face.is_null ()
	   && face_bits! (box, prev_face) as face_box_type != FACE_NO_BOX)
	{ it_bits! (start_of_box_run_p, it,
		    face_bits! (box, face) as face_box_type != FACE_NO_BOX
		    && (prev_face.is_null ()
			|| face_bits! (box, prev_face) as face_box_type == FACE_NO_BOX)); }
      it_bits! (face_box_p, it, face_bits! (box, face) as face_box_type != FACE_NO_BOX);
    }
}

/* Restore IT's settings from IT->stack.  Called, for example, when no
   more overlay strings must be processed, and we return to delivering
   display elements from a buffer, or when the end of a string from a
   `display' property is reached and we return to delivering display
   elements from an overlay string, or from a buffer.  */

unsafe fn pop_it (it: *mut it)
{
  let p: *mut iterator_stack_entry;
  let from_display_prop: bool = it_bits! (from_disp_prop_p, it);
  let prev_pos: ptrdiff_t = IT_CHARPOS! (*it);
  let prev_face_id: c_int = (*it).face_id;

  eassert! ((*it).sp > 0);
  (*it).sp -= 1;
  p = (*it).stack.as_mut_ptr ().offset ((*it).sp as isize);
  (*it).stop_charpos = (*p).stop_charpos;
  (*it).prev_stop = (*p).prev_stop;
  (*it).base_level_stop = (*p).base_level_stop;
  (*it).cmp_it = (*p).cmp_it;
  (*it).face_id = (*p).face_id;
  (*it).current = (*p).current;
  (*it).position = (*p).position;
  (*it).string = (*p).string;
  (*it).from_overlay = (*p).from_overlay;
  if NILP ((*it).string)
    { SET_TEXT_POS! ((*it).current.string_pos, -1, -1); }
  (*it).method = (*p).method;
  match (*it).method
    {
      GET_FROM_IMAGE =>
	{
	  (*it).image_id = (*p).u.image.image_id;
	  (*it).object = (*p).u.image.object;
	  (*it).slice = (*p).u.image.slice;
	}
      GET_FROM_XWIDGET =>
	{ (*it).object = (*p).u.xwidget.object; }
      GET_FROM_STRETCH =>
	{ (*it).object = (*p).u.stretch.object; }
      GET_FROM_BUFFER =>
	{
	  restore_face_box_flags (it, prev_face_id);
	  (*it).object = (*(*it).w).contents;
	}
      GET_FROM_STRING =>
	{
	  restore_face_box_flags (it, prev_face_id);
	  (*it).object = (*it).string;
	}
      GET_FROM_DISPLAY_VECTOR =>
	{
	  if !(*it).s.is_null ()
	    { (*it).method = GET_FROM_C_STRING; }
	  else if STRINGP ((*it).string)
	    { (*it).method = GET_FROM_STRING; }
	  else
	    {
	      (*it).method = GET_FROM_BUFFER;
	      (*it).object = (*(*it).w).contents;
	    }
	}
      GET_FROM_C_STRING =>
	{ }
      _ =>
	{ emacs_abort (); }
    }
  (*it).end_charpos = (*p).end_charpos;
  (*it).string_nchars = (*p).string_nchars as ptrdiff_t;
  (*it).area = (*p).area;
  it_bits! (multibyte_p, it, it_stack_bits! (multibyte_p, p));
  it_bits! (avoid_cursor_p, it, it_stack_bits! (avoid_cursor_p, p));
  (*it).space_width = (*p).space_width;
  (*it).font_height = (*p).font_height;
  (*it).voffset = (*p).voffset;
  it_bits! (string_from_display_prop_p, it, it_stack_bits! (string_from_display_prop_p, p));
  it_bits! (string_from_prefix_prop_p, it, it_stack_bits! (string_from_prefix_prop_p, p));
  (*it).line_wrap = (*p).line_wrap;
  it_bits! (bidi_p, it, it_stack_bits! (bidi_p, p));
  (*it).paragraph_embedding = (*p).paragraph_embedding;
  it_bits! (from_disp_prop_p, it, it_stack_bits! (from_disp_prop_p, p));
  it_bits! (align_visually_p, it, false);
  if it_bits! (bidi_p, it)
    {
      bidi_pop_it (&raw mut (*it).bidi_it);
      /* Bidi-iterate until we get out of the portion of text, if any,
	 covered by a `display' text property or by an overlay with
	 `display' property.  (We cannot just jump there, because the
	 internal coherency of the bidi iterator state can not be
	 preserved across such jumps.)  We also must determine the
	 paragraph base direction if the overlay we just processed is
	 at the beginning of a new paragraph.  */
      if from_display_prop
	  && ((*it).method == GET_FROM_BUFFER || (*it).method == GET_FROM_STRING)
	{ iterate_out_of_display_property (it); }

      eassert! ((BUFFERP ((*it).object)
		 && IT_CHARPOS! (*it) == (*it).bidi_it.charpos
		 && IT_BYTEPOS! (*it) == (*it).bidi_it.bytepos)
		|| (STRINGP ((*it).object)
		    && IT_STRING_CHARPOS! (*it) == (*it).bidi_it.charpos
		    && IT_STRING_BYTEPOS! (*it) == (*it).bidi_it.bytepos)
		|| (CONSP ((*it).object) && (*it).method == GET_FROM_STRETCH)
		/* We could be in the middle of handling a list or a
		   vector of several 'display' properties, in which
		   case we should only verify the above conditions when
		   we pop the iterator stack the last time, because
		   higher stack levels cannot "iterate out of the
		   display property".  */
		|| (*it).sp > 0);
    }
  /* If we move the iterator over text covered by a display property
     to a new buffer position, any info about previously seen overlays
     is no longer valid.  */
  if from_display_prop && (*it).sp == 0 && CHARPOS! ((*it).position) != prev_pos
    { it_bits! (ignore_overlay_strings_at_pos_p, it, false); }
}

/***********************************************************************
			  Moving over lines
 ***********************************************************************/

/* Set IT's current position to the previous line start.  */

unsafe fn back_to_previous_line_start (it: *mut it)
{
  let mut cp: ptrdiff_t = IT_CHARPOS! (*it);
  let mut bp: ptrdiff_t = IT_BYTEPOS! (*it);

  dec_both (&raw mut cp, &raw mut bp);
  SET_WITH_NARROWED_BEGV! (it, IT_CHARPOS! (*it),
			   find_newline_no_quit (cp, bp, -1, &raw mut IT_BYTEPOS! (*it)),
			   get_small_narrowing_begv ((*it).w, IT_CHARPOS! (*it)));
}

/* Find in the current buffer the first display or overlay string
   between STARTPOS and ENDPOS that includes embedded newlines.
   Consider only overlays that apply to window W.
   Value is non-zero if such a display/overlay string is found.  */
unsafe fn strings_with_newlines (startpos: ptrdiff_t, endpos: ptrdiff_t,
				 w: *mut window) -> bool
{
  let mut node: *mut itree_node;
  /* Process overlays.  */
  /* C: `ITREE_FOREACH (node, current_buffer->overlays, startpos, endpos, DESCENDING)`
     — itree.h 166-176's expansion, written out at the use site (P-33).  */
  if (*current_buffer).overlays.is_null ()
    { }
  else
    {
      let mut itree_local_iter_: itree_iterator = core::mem::zeroed ();
      let itree_iter_: *mut itree_iterator
	= itree_iterator_start (&raw mut itree_local_iter_, (*current_buffer).overlays,
				startpos, endpos, ITREE_DESCENDING);
      while { node = itree_iterator_next (itree_iter_); !node.is_null () }
	{
	  let overlay: Lisp_Object = (*node).data;
	  eassert! (OVERLAYP (overlay));

	  /* Skip this overlay if it doesn't apply to our window.  */
	  let window: Lisp_Object = Foverlay_get (overlay, Qwindow ());
	  if WINDOWP (window) && XWINDOW (window) != w
	    { continue; }

	  let ostart: ptrdiff_t = (*node).begin;
	  let oend: ptrdiff_t = (*node).end;

	  /* Skip overlays that don't overlap the range.  */
	  if !((startpos < oend && ostart < endpos)
	       || (ostart == oend
		   && (startpos == oend || (endpos == ZV! () && oend == endpos))))
	    { continue; }

	  let mut str: Lisp_Object;
	  str = Foverlay_get (overlay, Qbefore_string ());
	  if STRINGP (str) && SCHARS (str) != 0
	      && !memchr (SDATA (str) as *const c_void, b'\n' as c_int,
			  SBYTES (str) as usize).is_null ()
	    { return true; }
	  str = Foverlay_get (overlay, Qafter_string ());
	  if STRINGP (str) && SCHARS (str) != 0
	      && !memchr (SDATA (str) as *const c_void, b'\n' as c_int,
			  SBYTES (str) as usize).is_null ()
	    { return true; }
	}
    }

  /* Check for 'display' properties whose values include strings.  */
  let mut cpos: Lisp_Object = make_fixnum (startpos);
  let limpos: Lisp_Object = make_fixnum (endpos);

  while { cpos = Fnext_single_property_change (cpos, Qdisplay (), Qnil (), limpos);
	  !(NILP (cpos) || XFIXNAT (cpos) >= endpos) }
    {
      let spec: Lisp_Object = Fget_char_property (cpos, Qdisplay (), Qnil ());
      let string: Lisp_Object = string_from_display_spec (spec);
      if STRINGP (string)
	  && !memchr (SDATA (string) as *const c_void, b'\n' as c_int,
		      SBYTES (string) as usize).is_null ()
	{ return true; }
    }

  return false;
}


/* Move IT to the next line start.

   Value is true if a newline was found.  Set *SKIPPED_P to true if
   we skipped over part of the text (as opposed to moving the iterator
   continuously over the text).  Otherwise, don't change the value
   of *SKIPPED_P.

   If BIDI_IT_PREV is non-NULL, store into it the state of the bidi
   iterator on the newline, if it was found.

   Newlines may come from buffer text, overlay strings, or strings
   displayed via the `display' property.  That's the reason we can't
   simply use find_newline_no_quit.

   Note that this function may not skip over invisible text that is so
   because of text properties and immediately follows a newline.  If
   it would, function reseat_at_next_visible_line_start, when called
   from set_iterator_to_next, would effectively make invisible
   characters following a newline part of the wrong glyph row, which
   leads to wrong cursor motion.  */

unsafe fn forward_to_next_line_start (it: *mut it, skipped_p: *mut bool,
				      bidi_it_prev: *mut bidi_it) -> bool
{
  let mut old_selective: ptrdiff_t;
  let mut newline_found_p: bool = false;
  let mut n: c_int;
  const MAX_NEWLINE_DISTANCE: c_int = 500;

  /* If already on a newline, just consume it to avoid unintended
     skipping over invisible text below.  */
  if (*it).what == IT_CHARACTER
      && (*it).c == b'\n' as c_int
      && CHARPOS! ((*it).position) == IT_CHARPOS! (*it)
    {
      if it_bits! (bidi_p, it) && !bidi_it_prev.is_null ()
	{ *bidi_it_prev = (*it).bidi_it; }
      set_iterator_to_next (it, false);
      (*it).c = 0;
      return true;
    }

  /* Don't handle selective display in the following.  It's (a)
     unnecessary because it's done by the caller, and (b) leads to an
     infinite recursion because next_element_from_ellipsis indirectly
     calls this function.  */
  old_selective = (*it).selective;
  (*it).selective = 0;

  /* Scan for a newline within MAX_NEWLINE_DISTANCE display elements
     from buffer text, or till the end of the string if iterating a
     string.  */
  n = 0;
  while !newline_found_p && n < MAX_NEWLINE_DISTANCE
    {
      if !get_next_display_element (it)
	{ return false; }
      newline_found_p = (*it).what == IT_CHARACTER && (*it).c == b'\n' as c_int;
      if newline_found_p && it_bits! (bidi_p, it) && !bidi_it_prev.is_null ()
	{ *bidi_it_prev = (*it).bidi_it; }
      set_iterator_to_next (it, false);
      /* C's `for` increment `n += !STRINGP (it->string)` adds the C boolean.  */
      n += (! STRINGP ((*it).string)) as c_int;
    }

  /* If we didn't find a newline near enough, see if we can use a
     short-cut.  */
  if !newline_found_p
    {
      /* C leaves `bytepos` to `find_newline_no_quit`; Rust cannot prove the call has
	 written it, so the port zero-initialises it (PORTING.md 1.5, deviation 1's rule).  */
      let mut bytepos: ptrdiff_t = 0;
      let start: ptrdiff_t = IT_CHARPOS! (*it);
      let limit: ptrdiff_t = find_newline_no_quit (start, IT_BYTEPOS! (*it),
						   1, &raw mut bytepos);
      eassert! (!STRINGP ((*it).string));

      /* it->stop_charpos >= limit means we already know there's no
	 stop position up until the newline at LIMIT, so there's no
	 need for any further checks.  */
      let mut no_strings_with_newlines: bool = (*it).stop_charpos >= limit;

      if !no_strings_with_newlines
	{
	  if !(buffer_bits! (long_line_optimizations_p, current_buffer)
	       && (*it).line_wrap == TRUNCATE)
	    {
	      /* Quick-and-dirty check: if there isn't any `display'
		 property in sight, and no overlays, we're done.  */
	      let pos: Lisp_Object
		= Fnext_single_property_change (make_fixnum (start),
						Qdisplay (), Qnil (),
						make_fixnum (limit));
	      no_strings_with_newlines =
		(NILP (pos) || XFIXNAT (pos) == limit) /* no 'display' props */
		&& next_overlay_change (start) == ZV! ();  /* no overlays */
	    }
	  else
	    {
	      /* For buffers with very long and truncated lines we try
		 harder, because it's worth our while to spend some
		 time looking into the overlays and 'display' properties
		 if we can then avoid iterating through all of them.  */
	      no_strings_with_newlines
		= !strings_with_newlines (start, limit, (*it).w);
	    }
	}

      /* If there's no display or overlay strings with embedded
	 newlines until the position of the newline in buffer text, we
	 can just use that position.  */
      if no_strings_with_newlines
	{
	  if !it_bits! (bidi_p, it) || bidi_it_prev.is_null ()
	    {
	      /* The optimal case: just jump there.  */
	      IT_CHARPOS! (*it) = limit;
	      IT_BYTEPOS! (*it) = bytepos;
	    }
	  else
	    {
	      /* The less optimal case: need to bidi-walk there, but
		 this is still cheaper that the full iteration using
		 get_next_display_element and set_iterator_to_next.  */
	      let mut bprev: bidi_it;

	      /* Help bidi.c avoid expensive searches for display
		 properties and overlays, by telling it that there are
		 none up to `limit'.  */
	      if (*it).bidi_it.disp_pos < limit
		{
		  (*it).bidi_it.disp_pos = limit;
		  (*it).bidi_it.disp_prop = 0;
		}
	      loop
		{
		  bprev = (*it).bidi_it;
		  bidi_move_to_visually_next (&raw mut (*it).bidi_it);
		  if !((*it).bidi_it.charpos != limit) { break; }
		}
	      IT_CHARPOS! (*it) = limit;
	      IT_BYTEPOS! (*it) = (*it).bidi_it.bytepos;
	      if !bidi_it_prev.is_null ()
		{ *bidi_it_prev = bprev; }
	    }
	  /* C's chain `*skipped_p = newline_found_p = true` (the `init_iterator` order).  */
	  *skipped_p = true;
	  newline_found_p = true;
	}
      else
	{
	  /* The slow case.  */
	  while !newline_found_p
	    {
	      if !get_next_display_element (it)
		{ break; }
	      newline_found_p = ITERATOR_AT_END_OF_LINE_P! (it);
	      if newline_found_p && it_bits! (bidi_p, it) && !bidi_it_prev.is_null ()
		{ *bidi_it_prev = (*it).bidi_it; }
	      set_iterator_to_next (it, false);
	    }
	}
    }

  (*it).selective = old_selective;
  return newline_found_p;
}


/* Set IT's current position to the previous visible line start.  Skip
   invisible text that is so either due to text properties or due to
   selective display.  Caution: this does not change IT->current_x and
   IT->hpos.  */

unsafe fn back_to_previous_visible_line_start (it: *mut it)
{
  /* C's `break` in the body belongs to this loop, and the two `goto replaced` jumps go to
     the label at the end of the inner block, so the port names the loop and the block
     (PORTING.md 2.2; the `read_filtered_event` shape).  */
  'next_line: while IT_CHARPOS! (*it) > BEGV! ()
    {
      back_to_previous_line_start (it);

      if IT_CHARPOS! (*it) <= BEGV! ()
	{ break; }

      /* If selective > 0, then lines indented more than its value are
	 invisible.  */
      if (*it).selective > 0
	  && indented_beyond_p (IT_CHARPOS! (*it), IT_BYTEPOS! (*it),
				(*it).selective)
	{ continue; }

      /* Check the newline before point for invisibility.  */
      {
	let prop: Lisp_Object;
	prop = Fget_char_property (make_fixnum (IT_CHARPOS! (*it) - 1),
				   Qinvisible (), (*it).window);
	if TEXT_PROP_MEANS_INVISIBLE! (prop) != 0
	  { continue; }
      }

      if IT_CHARPOS! (*it) <= BEGV! ()
	{ break; }

      {
	/* C leaves `it2`, `pos`, `beg`, `end`, `val` and `overlay` to the code below
	   (`SAVE_IT` fills `it2` in from `*it`); Rust has no partial initialisation, so the
	   port zero-initialises them (PORTING.md 1.5, deviation 1's rule).  */
	let mut it2: it = core::mem::zeroed ();
	let mut it2data: *mut c_void = core::ptr::null_mut ();
	let mut pos: ptrdiff_t = 0;
	let mut beg: ptrdiff_t = 0;
	let mut end: ptrdiff_t = 0;
	let mut val: Lisp_Object = Qnil ();
	let mut overlay: Lisp_Object = Qnil ();

	'replaced: {
	  SAVE_IT! (it2, *it, it2data);

	  /* If newline is part of a composition, continue from start of composition */
	  if find_composition (IT_CHARPOS! (*it), -1, &raw mut beg, &raw mut end,
			       &raw mut val, Qnil ())
	      && beg < IT_CHARPOS! (*it)
	    { break 'replaced; }

	  /* If newline is replaced by a display property, find start of overlay
	     or interval and continue search from that point.  */
	  /* C's pre-decrements in one statement: `pos = --IT_CHARPOS (it2)`.  */
	  pos = { IT_CHARPOS! (it2) -= 1; IT_CHARPOS! (it2) };
	  IT_BYTEPOS! (it2) -= 1;
	  it2.sp = 0;
	  bidi_unshelve_cache (core::ptr::null_mut (), false);
	  it_bits! (string_from_display_prop_p, &raw mut it2, false);
	  it_bits! (from_disp_prop_p, &raw mut it2, false);
	  if handle_display_prop (&raw mut it2) == HANDLED_RETURN
	      && !NILP ({ val = get_char_property_and_overlay
			  (make_fixnum (pos), Qdisplay (), Qnil (), &raw mut overlay);
			  val })
	      && (if OVERLAYP (overlay)
		  { beg = OVERLAY_START (overlay); beg != 0 }
		  else
		  { get_property_and_range (pos, Qdisplay (), &raw mut val,
					    &raw mut beg, &raw mut end, Qnil ()) })
	    {
	      RESTORE_IT! (it, it, it2data);
	      break 'replaced;
	    }

	  /* Newline is not replaced by anything -- so we are done.  */
	  RESTORE_IT! (it, it, it2data);
	  break 'next_line;
	}

	/* replaced: */
	if beg < BEGV! ()
	  { beg = BEGV! (); }
	IT_CHARPOS! (*it) = beg;
	IT_BYTEPOS! (*it) = buf_charpos_to_bytepos (current_buffer as *mut c_void, beg);
      }
    }

  (*it).continuation_lines_width = 0;

  eassert! (IT_CHARPOS! (*it) >= BEGV! ());
  eassert! ((*it).medium_narrowing_begv > 0 /* long-line optimizations: all bets off */
	    || IT_CHARPOS! (*it) == BEGV! ()
	    || FETCH_BYTE! (IT_BYTEPOS! (*it) - 1) == b'\n');
  CHECK_IT (it);
}


/* Reseat iterator IT at the previous visible line start.  Skip
   invisible text that is so either due to text properties or due to
   selective display.  At the end, update IT's overlay information,
   face information etc.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reseat_at_previous_visible_line_start (it: *mut it)
{
  back_to_previous_visible_line_start (it);
  reseat (it, (*it).current.pos, true);
  CHECK_IT (it);
}


/* Reseat iterator IT on the next visible line start in the current
   buffer.  ON_NEWLINE_P means position IT on the newline
   preceding the line start.  Skip over invisible text that is so
   because of selective display.  Compute faces, overlays etc at
   the new position.  Note that this function does not skip over text that
   is invisible because of text properties.  */

unsafe fn reseat_at_next_visible_line_start (it: *mut it, on_newline_p: bool)
{
  let mut skipped_p: bool = false;
  /* C leaves `bidi_it_prev` to `forward_to_next_line_start`; Rust has no partial
     initialisation, so the port zero-initialises it (PORTING.md 1.5, deviation 1's rule).  */
  let mut bidi_it_prev: bidi_it = core::mem::zeroed ();
  let mut newline_found_p: bool
    = forward_to_next_line_start (it, &raw mut skipped_p,
				  if on_newline_p { &raw mut bidi_it_prev }
				  else { core::ptr::null_mut () });

  /* Skip over lines that are invisible because they are indented
     more than the value of IT->selective.  */
  if (*it).selective > 0
    {
      while IT_CHARPOS! (*it) < ZV! ()
	    && indented_beyond_p (IT_CHARPOS! (*it), IT_BYTEPOS! (*it),
				  (*it).selective)
	{
	  eassert! (IT_BYTEPOS! (*it) == BEGV! ()
		    || FETCH_BYTE! (IT_BYTEPOS! (*it) - 1) == b'\n');
	  newline_found_p =
	    forward_to_next_line_start (it, &raw mut skipped_p,
					if on_newline_p { &raw mut bidi_it_prev }
					else { core::ptr::null_mut () });
	}
    }

  /* Position on the newline if that's what's requested.  */
  if on_newline_p && newline_found_p
    {
      if STRINGP ((*it).string)
	{
	  if IT_STRING_CHARPOS! (*it) > 0
	    {
	      if !it_bits! (bidi_p, it)
		{
		  IT_STRING_CHARPOS! (*it) -= 1;
		  IT_STRING_BYTEPOS! (*it) -= 1;
		}
	      else
		{
		  /* We need to restore the bidi iterator to the state
		     it had on the newline, and resync the IT's
		     position with that.  */
		  (*it).bidi_it = bidi_it_prev;
		  IT_STRING_CHARPOS! (*it) = (*it).bidi_it.charpos;
		  IT_STRING_BYTEPOS! (*it) = (*it).bidi_it.bytepos;
		}
	    }
	}
      else if IT_CHARPOS! (*it) > BEGV! ()
	{
	  if !it_bits! (bidi_p, it)
	    {
	      IT_CHARPOS! (*it) -= 1;
	      IT_BYTEPOS! (*it) -= 1;
	    }
	  else
	    {
	      /* We need to restore the bidi iterator to the state it
		 had on the newline and resync IT with that.  */
	      (*it).bidi_it = bidi_it_prev;
	      IT_CHARPOS! (*it) = (*it).bidi_it.charpos;
	      IT_BYTEPOS! (*it) = (*it).bidi_it.bytepos;
	    }
	  reseat (it, (*it).current.pos, false);
	}
    }
  else if skipped_p
    { reseat (it, (*it).current.pos, false); }

  CHECK_IT (it);
}


/***********************************************************************
		   Changing an iterator's position
***********************************************************************/

/* Change IT's current position to POS in current_buffer.
   If FORCE_P, always check for text properties at the new position.
   Otherwise, text properties are only looked up if POS >=
   IT->check_charpos of a property.  */

unsafe fn reseat (it: *mut it, pos: text_pos, force_p: bool)
{
  let original_pos: ptrdiff_t = IT_CHARPOS! (*it);

  reseat_1 (it, pos, false);

  if buffer_bits! (long_line_optimizations_p, current_buffer)
    {
      if (*it).medium_narrowing_begv == 0
	{
	  (*it).medium_narrowing_begv
	    = get_medium_narrowing_begv ((*it).w, window_point ((*it).w));
	  (*it).medium_narrowing_zv
	    = get_medium_narrowing_zv ((*it).w, window_point ((*it).w));
	  (*it).large_narrowing_begv
	    = get_large_narrowing_begv (window_point ((*it).w));
	  (*it).large_narrowing_zv
	    = get_large_narrowing_zv (window_point ((*it).w));
	}
      else if ((pos.charpos < (*it).medium_narrowing_begv
		|| pos.charpos > (*it).medium_narrowing_zv)
		&& (!redisplaying_p || (*it).line_wrap == TRUNCATE))
	{
	  (*it).medium_narrowing_begv
	    = get_medium_narrowing_begv ((*it).w, pos.charpos);
	  (*it).medium_narrowing_zv
	    = get_medium_narrowing_zv ((*it).w, pos.charpos);
	  (*it).large_narrowing_begv
	    = get_large_narrowing_begv (window_point ((*it).w));
	  (*it).large_narrowing_zv
	    = get_large_narrowing_zv (window_point ((*it).w));
	}
    }

  /* Determine where to check text properties.  Avoid doing it
     where possible because text property lookup is very expensive.  */
  if force_p
      || CHARPOS! (pos) > (*it).stop_charpos
      || CHARPOS! (pos) < original_pos
    {
      if it_bits! (bidi_p, it)
	{
	  /* For bidi iteration, we need to prime prev_stop and
	     base_level_stop with our best estimations.  */
	  /* Implementation note: Of course, POS is not necessarily a
	     stop position, so assigning prev_pos to it is a lie; we
	     should have called compute_stop_backwards.  However, if
	     the current buffer does not include any R2L characters,
	     that call would be a waste of cycles, because the
	     iterator will never move back, and thus never cross this
	     "fake" stop position.  So we delay that backward search
	     until the time we really need it, in next_element_from_buffer.  */
	  if CHARPOS! (pos) != (*it).prev_stop
	    { (*it).prev_stop = CHARPOS! (pos); }
	  if CHARPOS! (pos) < (*it).base_level_stop
	    { (*it).base_level_stop = 0; /* meaning it's unknown */ }
	  handle_stop (it);
	}
      else
	{
	  handle_stop (it);
	  (*it).prev_stop = 0;
	  (*it).base_level_stop = 0;
	}

    }

  CHECK_IT (it);
}


/* Change IT's buffer position to POS.  SET_STOP_P means set
   IT->stop_pos to POS, also.  */

unsafe fn reseat_1 (it: *mut it, pos: text_pos, set_stop_p: bool)
{
  /* Don't call this function when scanning a C string.  */
  eassert! ((*it).s.is_null ());

  /* POS must be a reasonable value.  */
  eassert! (CHARPOS! (pos) >= BEGV! () && CHARPOS! (pos) <= ZV! ());

  (*it).current.pos = pos;
  (*it).position = pos;
  (*it).end_charpos = ZV! ();
  (*it).dpvec = core::ptr::null_mut ();
  (*it).current.dpvec_index = -1;
  (*it).current.overlay_string_index = -1;
  IT_STRING_CHARPOS! (*it) = -1;
  IT_STRING_BYTEPOS! (*it) = -1;
  (*it).string = Qnil ();
  (*it).method = GET_FROM_BUFFER;
  (*it).object = (*(*it).w).contents;
  (*it).area = TEXT_AREA;
  it_bits! (multibyte_p, it,
	    !NILP (BVAR! (current_buffer, enable_multibyte_characters_)));
  (*it).sp = 0;
  it_bits! (string_from_display_prop_p, it, false);
  it_bits! (string_from_prefix_prop_p, it, false);

  it_bits! (from_disp_prop_p, it, false);
  it_bits! (face_before_selective_p, it, false);
  if it_bits! (bidi_p, it)
    {
      bidi_init_it (IT_CHARPOS! (*it), IT_BYTEPOS! (*it), FRAME_WINDOW_P ((*it).f),
		    &raw mut (*it).bidi_it);
      bidi_unshelve_cache (core::ptr::null_mut (), false);
      (*it).bidi_it.paragraph_dir = NEUTRAL_DIR;
      (*it).bidi_it.string.s = core::ptr::null ();
      (*it).bidi_it.string.lstring = Qnil ();
      (*it).bidi_it.string.bufpos = 0;
      bidi_string_bits! (from_disp_str, &raw mut (*it).bidi_it.string, false);
      bidi_string_bits! (unibyte, &raw mut (*it).bidi_it.string, false);
      (*it).bidi_it.w = (*it).w;
    }

  if set_stop_p
    {
      (*it).stop_charpos = CHARPOS! (pos);
      (*it).base_level_stop = CHARPOS! (pos);
    }
  /* This make the information stored in it->cmp_it invalidate.  */
  (*it).cmp_it.id = -1;
  (*it).min_width_property = Qnil ();
}


/* Set up IT for displaying a string, starting at CHARPOS in window W.
   If S is non-null, it is a C string to iterate over.  Otherwise,
   STRING gives a Lisp string to iterate over.

   If PRECISION > 0, don't return more then PRECISION number of
   characters from the string.

   If FIELD_WIDTH > 0, return padding spaces until FIELD_WIDTH
   characters have been returned.  FIELD_WIDTH < 0 means an infinite
   field width.

   MULTIBYTE = 0 means disable processing of multibyte characters,
   MULTIBYTE > 0 means enable it,
   MULTIBYTE < 0 means use IT->multibyte_p.

   IT must be initialized via a prior call to init_iterator before
   calling this function.  */

unsafe fn reseat_to_string (it: *mut it, s: *const c_char, string: Lisp_Object,
			    charpos: ptrdiff_t, precision: ptrdiff_t,
			    mut field_width: c_int, multibyte: c_int)
{
  /* No text property checks performed by default, but see below.  */
  (*it).stop_charpos = -1;

  /* Set iterator position and end position.  */
  memset (&raw mut (*it).current as *mut c_void, 0,
	  core::mem::size_of::<display_pos> ());
  (*it).current.overlay_string_index = -1;
  (*it).current.dpvec_index = -1;
  eassert! (charpos >= 0);

  /* If STRING is specified, use its multibyteness, otherwise use the
     setting of MULTIBYTE, if specified.  */
  if multibyte >= 0
    { it_bits! (multibyte_p, it, multibyte > 0); }

  /* Bidirectional reordering of strings is controlled by the default
     value of bidi-display-reordering.  Don't try to reorder while
     loading loadup.el, as the necessary character property tables are
     not yet available.  */
  it_bits! (bidi_p, it,
	    !redisplay__inhibit_bidi
	    && !NILP (BVAR! (&raw mut buffer_defaults, bidi_display_reordering_)));

  if s.is_null ()
    {
      eassert! (STRINGP (string));
      (*it).string = string;
      (*it).s = core::ptr::null ();
      (*it).end_charpos = SCHARS (string);
      (*it).string_nchars = SCHARS (string);
      (*it).method = GET_FROM_STRING;
      (*it).current.string_pos = string_pos (charpos, string);

      if it_bits! (bidi_p, it)
	{
	  (*it).bidi_it.string.lstring = string;
	  (*it).bidi_it.string.s = core::ptr::null ();
	  (*it).bidi_it.string.schars = (*it).end_charpos;
	  (*it).bidi_it.string.bufpos = 0;
	  bidi_string_bits! (from_disp_str, &raw mut (*it).bidi_it.string, false);
	  bidi_string_bits! (unibyte, &raw mut (*it).bidi_it.string,
			     !it_bits! (multibyte_p, it));
	  (*it).bidi_it.w = (*it).w;
	  bidi_init_it (charpos, IT_STRING_BYTEPOS! (*it),
			FRAME_WINDOW_P ((*it).f), &raw mut (*it).bidi_it);
	}
    }
  else
    {
      (*it).s = s as *const u8;
      (*it).string = Qnil ();

      /* Note that we use IT->current.pos, not it->current.string_pos,
	 for displaying C strings.  */
      IT_STRING_CHARPOS! (*it) = -1;
      IT_STRING_BYTEPOS! (*it) = -1;
      if it_bits! (multibyte_p, it)
	{
	  (*it).current.pos = c_string_pos (charpos, s, true);
	  (*it).end_charpos = number_of_chars (s, true);
	  (*it).string_nchars = number_of_chars (s, true);
	}
      else
	{
	  IT_CHARPOS! (*it) = charpos;
	  IT_BYTEPOS! (*it) = charpos;
	  (*it).end_charpos = strlen (s) as ptrdiff_t;
	  (*it).string_nchars = strlen (s) as ptrdiff_t;
	}

      if it_bits! (bidi_p, it)
	{
	  (*it).bidi_it.string.lstring = Qnil ();
	  (*it).bidi_it.string.s = s as *const u8;
	  (*it).bidi_it.string.schars = (*it).end_charpos;
	  (*it).bidi_it.string.bufpos = 0;
	  bidi_string_bits! (from_disp_str, &raw mut (*it).bidi_it.string, false);
	  bidi_string_bits! (unibyte, &raw mut (*it).bidi_it.string,
			     !it_bits! (multibyte_p, it));
	  (*it).bidi_it.w = (*it).w;
	  bidi_init_it (charpos, IT_BYTEPOS! (*it), FRAME_WINDOW_P ((*it).f),
			&raw mut (*it).bidi_it);
	}
      (*it).method = GET_FROM_C_STRING;
    }

  /* PRECISION > 0 means don't return more than PRECISION characters
     from the string.  */
  if precision > 0 && (*it).end_charpos - charpos > precision
    {
      (*it).end_charpos = charpos + precision;
      (*it).string_nchars = charpos + precision;
      if it_bits! (bidi_p, it)
	{ (*it).bidi_it.string.schars = (*it).end_charpos; }
    }

  /* FIELD_WIDTH > 0 means pad with spaces until FIELD_WIDTH
     characters have been returned.  FIELD_WIDTH == 0 means don't pad,
     FIELD_WIDTH < 0 means infinite field width.  This is useful for
     padding with `-' at the end of a mode line.  */
  if field_width < 0
    { field_width = DISP_INFINITY; }
  /* Implementation note: We deliberately don't enlarge
     it->bidi_it.string.schars here to fit it->end_charpos, because
     the bidi iterator cannot produce characters out of thin air.  */
  /* C compares an `int` with the `ptrdiff_t` difference (its usual arithmetic conversions).  */
  if (field_width as ptrdiff_t) > (*it).end_charpos - charpos
    { (*it).end_charpos = charpos + field_width as ptrdiff_t; }

  /* Use the standard display table for displaying strings.  */
  if DISP_TABLE_P (Vstandard_display_table)
    { (*it).dp = XCHAR_TABLE (Vstandard_display_table); }

  (*it).stop_charpos = charpos;
  (*it).prev_stop = charpos;
  (*it).base_level_stop = 0;
  if it_bits! (bidi_p, it)
    {
      bidi_it_bits! (first_elt, &raw mut (*it).bidi_it, true);
      (*it).bidi_it.paragraph_dir = NEUTRAL_DIR;
      (*it).bidi_it.disp_pos = -1;
    }
  if s.is_null () && it_bits! (multibyte_p, it)
    {
      let mut endpos: ptrdiff_t = SCHARS ((*it).string);
      if endpos > (*it).end_charpos
	{ endpos = (*it).end_charpos; }
      composition_compute_stop_pos (&raw mut (*it).cmp_it as *mut c_void,
				    charpos, -1, endpos, (*it).string, true);
    }
  CHECK_IT (it);
}


/***********************************************************************
			      Iteration
***********************************************************************/

/* Map enum it_method value to corresponding next_element_from_* function.  */

pub type next_element_function = Option<unsafe extern "C" fn (it: *mut it) -> bool>;

static get_next_element: [next_element_function; NUM_IT_METHODS as usize] =
[
  Some (next_element_from_buffer),
  Some (next_element_from_display_vector),
  Some (next_element_from_string),
  Some (next_element_from_c_string),
  Some (next_element_from_image),
  Some (next_element_from_stretch),
  Some (next_element_from_xwidget),
];

/* C: xdisp.c 8286: `#define GET_NEXT_DISPLAY_ELEMENT(it) (*get_next_element[(it)->method]) (it)`.
   The port's array holds `Option`s, so the call unwraps (the `it_props[]` handler shape).
   `get_next_element[]` takes the seven `next_element_from_*` addresses, so those are
   `static` in C but carry the C ABI here (and no `#[unsafe(no_mangle)]`, since the symbol
   is this unit's own — deviation 18's rule; the two the array does *not* name,
   `next_element_from_ellipsis` and `next_element_from_composition`, stay plain `unsafe fn`).  */
macro_rules! GET_NEXT_DISPLAY_ELEMENT {
  ($it:expr) => { (get_next_element[(*$it).method as usize].unwrap ()) ($it) };
}

/* Return true iff a character at CHARPOS (and BYTEPOS) is composed
   (possibly with the following characters).

  Note: we pass -1 as the "resolved bidi level" when the iterator
  doesn't have the bidi_p flag set, because in that case we really
  don't know what is the directionality of the text, so we leave it to
  the shaping engine to figure that out.  */

macro_rules! CHAR_COMPOSED_P {
  ($IT:expr, $CHARPOS:expr, $BYTEPOS:expr, $END_CHARPOS:expr) => {
    ((*$IT).cmp_it.id >= 0
     || ((*$IT).cmp_it.stop_pos == $CHARPOS
	 && composition_reseat_it (&raw mut (*$IT).cmp_it as *mut c_void,
				   $CHARPOS, $BYTEPOS, $END_CHARPOS,
				   (*$IT).w as *mut c_void,
				   if it_bits! (bidi_p, $IT)
				   { (*$IT).bidi_it.resolved_level }
				   else { -1 as c_char },
				   FACE_FROM_ID_OR_NULL ((*$IT).f, (*$IT).face_id) as *mut c_void,
				   (*$IT).string)))
  };
}


/* Lookup the char-table Vglyphless_char_display for character C (-1
   if we want information for no-font case), and return the display
   method symbol.  By side-effect, update it->what and
   it->glyphless_method.  This function is called from
   get_next_display_element for each character element, and from
   gui_produce_glyphs when no suitable font was found.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lookup_glyphless_char_display (c: c_int, it: *mut it) -> Lisp_Object
{
  let mut glyphless_method: Lisp_Object = Qnil ();

  if CHAR_TABLE_P (Vglyphless_char_display)
      && CHAR_TABLE_EXTRA_SLOTS (XCHAR_TABLE (Vglyphless_char_display)) >= 1
    {
      if c >= 0
	{ glyphless_method = CHAR_TABLE_REF (Vglyphless_char_display, c as isize); }
      else
	{ glyphless_method = FAM_ELEM! ((*XCHAR_TABLE (Vglyphless_char_display)).extras, 0); }

      if CONSP (glyphless_method)
	{ glyphless_method = if FRAME_WINDOW_P ((*it).f)
			     { XCAR (glyphless_method) }
			     else { XCDR (glyphless_method) }; }
    }

  /* C's `retry:` label with its backward `goto retry` becomes a labelled loop
     (PORTING.md 2.2, the `read_filtered_event` shape).  */
  'retry: loop
    {
      if NILP (glyphless_method)
	{
	  if c >= 0
	    /* The default is to display the character by a proper font.  */
	    { return Qnil (); }
	  /* The default for the no-font case is to display an empty box.  */
	  glyphless_method = Qempty_box ();
	}
      if EQ (glyphless_method, Qzero_width ())
	{
	  if c >= 0
	    { return glyphless_method; }
	  /* This method can't be used for the no-font case.  */
	  glyphless_method = Qempty_box ();
	}
      if EQ (glyphless_method, Qthin_space ())
	{ (*it).glyphless_method = GLYPHLESS_DISPLAY_THIN_SPACE; }
      else if EQ (glyphless_method, Qempty_box ())
	{ (*it).glyphless_method = GLYPHLESS_DISPLAY_EMPTY_BOX; }
      else if EQ (glyphless_method, Qhex_code ())
	{ (*it).glyphless_method = GLYPHLESS_DISPLAY_HEX_CODE; }
      else if STRINGP (glyphless_method)
	{ (*it).glyphless_method = GLYPHLESS_DISPLAY_ACRONYM; }
      else
	{
	  /* Invalid value.  We use the default method.  */
	  glyphless_method = Qnil ();
	  continue 'retry;
	}
      break;
    }
  (*it).what = IT_GLYPHLESS;
  return glyphless_method;
}

/* Merge escape glyph face and cache the result.  */

static mut last_escape_glyph_frame: *mut frame = core::ptr::null_mut ();
static mut last_escape_glyph_face_id: c_int = 1 << FACE_ID_BITS;
static mut last_escape_glyph_merged_face_id: c_int = 0;

unsafe fn merge_escape_glyph_face (it: *mut it) -> c_int
{
  let mut face_id: c_int;

  if (*it).f == last_escape_glyph_frame
      && (*it).face_id == last_escape_glyph_face_id
    { face_id = last_escape_glyph_merged_face_id; }
  else
    {
      /* Merge the `escape-glyph' face into the current face.  */
      face_id = merge_faces ((*it).w, Qescape_glyph (), 0, (*it).face_id);
      last_escape_glyph_frame = (*it).f;
      last_escape_glyph_face_id = (*it).face_id;
      last_escape_glyph_merged_face_id = face_id;
    }
  return face_id;
}

/* Likewise for glyphless glyph face.  */

static mut last_glyphless_glyph_frame: *mut frame = core::ptr::null_mut ();
static mut last_glyphless_glyph_face_id: c_int = 1 << FACE_ID_BITS;
static mut last_glyphless_glyph_merged_face_id: c_int = 0;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn merge_glyphless_glyph_face (it: *mut it) -> c_int
{
  let mut face_id: c_int;

  if (*it).f == last_glyphless_glyph_frame
      && (*it).face_id == last_glyphless_glyph_face_id
    { face_id = last_glyphless_glyph_merged_face_id; }
  else
    {
      /* Merge the `glyphless-char' face into the current face.  */
      face_id = merge_faces ((*it).w, Qglyphless_char (), 0, (*it).face_id);
      last_glyphless_glyph_frame = (*it).f;
      last_glyphless_glyph_face_id = (*it).face_id;
      last_glyphless_glyph_merged_face_id = face_id;
    }
  return face_id;
}

/* Forget the `escape-glyph' and `glyphless-char' faces.  This should
   be called before redisplaying windows, and when the frame's face
   cache is freed.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn forget_escape_and_glyphless_faces ()
{
  last_escape_glyph_frame = core::ptr::null_mut ();
  last_escape_glyph_face_id = 1 << FACE_ID_BITS;
  last_glyphless_glyph_frame = core::ptr::null_mut ();
  last_glyphless_glyph_face_id = 1 << FACE_ID_BITS;
}

/* Load IT's display element fields with information about the next
   display element from the current position of IT.  Value is false if
   end of buffer (or C string) is reached.  */

unsafe fn get_next_display_element (it: *mut it) -> bool
{
  /* True means that we found a display element.  False means that
     we hit the end of what we iterate over.  Performance note: the
     function pointer `method' used here turns out to be faster than
     using a sequence of if-statements.  */
  let mut success_p: bool;

  /* C's `get_next:` label with its three backward `goto get_next` becomes this labelled
     loop, and the two `goto done` sites leave it for the `done:` block below
     (PORTING.md 2.2).  */
  'get_next: loop
    {
      success_p = GET_NEXT_DISPLAY_ELEMENT! (it);

      if (*it).what == IT_CHARACTER
	{
	  /* UAX#9, L4: "A character is depicted by a mirrored glyph if
	     and only if (a) the resolved directionality of that character
	     is R..."  */
	  /* FIXME: Do we need an exception for characters from display
	     tables?  */
	  if it_bits! (bidi_p, it) && (*it).bidi_it.r#type == STRONG_R
	      && !inhibit_bidi_mirroring
	    { (*it).c = bidi_mirror_char ((*it).c); }
	  /* Map via display table or translate control characters.
	     IT->c, IT->len etc. have been set to the next character by
	     the function call above.  If we have a display table, and it
	     contains an entry for IT->c, translate it.  Don't do this if
	     IT->c itself comes from a display table, otherwise we could
	     end up in an infinite recursion.  (An alternative could be to
	     count the recursion depth of this function and signal an
	     error when a certain maximum depth is reached.)  Is it worth
	     it?  */
	  if success_p && (*it).dpvec.is_null ()
	    {
	      let mut dv: Lisp_Object;
	      let unibyte: *mut charset = CHARSET_FROM_ID! (charset_unibyte);
	      let mut nonascii_space_p: bool = false;
	      let mut nonascii_hyphen_p: bool = false;
	      let mut c: c_int = (*it).c;	/* This is the character to display.  */

	      if !it_bits! (multibyte_p, it) && !ASCII_CHAR_P (c as isize)
		{
		  eassert! (SINGLE_BYTE_CHAR_P (c as intmax_t));
		  if unibyte_display_via_language_environment
		    {
		      c = DECODE_CHAR! (unibyte, c as c_uint);
		      if c < 0
			{ c = BYTE8_TO_CHAR ((*it).c); }
		    }
		  else
		    { c = BYTE8_TO_CHAR ((*it).c); }
		}

	      if !(*it).dp.is_null ()
		  && { dv = DISP_CHAR_VECTOR ((*it).dp, c); VECTORP (dv) }
		{
		  let v: *mut Lisp_Vector = XVECTOR (dv);

		  /* Return the first character from the display table
		     entry, if not empty.  If empty, don't display the
		     current character.  */
		  if (*v).header.size != 0
		    {
		      (*it).dpvec_char_len = (*it).len;
		      (*it).dpvec = (*v).contents.as_mut_ptr ();
		      (*it).dpend = (*v).contents.as_mut_ptr ().offset ((*v).header.size);
		      (*it).current.dpvec_index = 0;
		      (*it).dpvec_face_id = -1;
		      (*it).saved_face_id = (*it).face_id;
		      (*it).method = GET_FROM_DISPLAY_VECTOR;
		      it_bits! (ellipsis_p, it, false);
		    }
		  else
		    {
		      set_iterator_to_next (it, false);
		    }
		  continue 'get_next;
		}

	      if !NILP (lookup_glyphless_char_display (c, it))
		{
		  if (*it).what == IT_GLYPHLESS
		    { break 'get_next; }
		  /* Don't display this character.  */
		  set_iterator_to_next (it, false);
		  continue 'get_next;
		}

	      /* If `nobreak-char-display' is non-nil, we display
		 non-ASCII spaces and hyphens specially.  */
	      if !ASCII_CHAR_P (c as isize) && !NILP (Vnobreak_char_display)
		{
		  if blankp (c as c_int)
		    { nonascii_space_p = true; }
		  else if c == SOFT_HYPHEN || c == HYPHEN
			  || c == NON_BREAKING_HYPHEN
		    { nonascii_hyphen_p = true; }
		}

	      /* Translate control characters into `\003' or `^C' form.
		 Control characters coming from a display table entry are
		 currently not translated because we use IT->dpvec to hold
		 the translation.  This could easily be changed but I
		 don't believe that it is worth doing.

		 The characters handled by `nobreak-char-display' must be
		 translated too.

		 Non-printable characters and raw-byte characters are also
		 translated to octal or hexadecimal form.  */
	      if (if c < ' ' as c_int || c == 127 /* ASCII control chars.  */
		  /* In mode line, treat \n, \t like other crl chars.  */
		  { (*it).area != TEXT_AREA
		    || (c != '\t' as c_int
			&& !(*it).glyph_row.is_null ()
			&& (glyph_row_bits! (mode_line_p, (*it).glyph_row)
			    || it_bits! (avoid_cursor_p, it)))
		    || (c != '\n' as c_int && c != '\t' as c_int) }
		  else
		  { nonascii_space_p
		    || nonascii_hyphen_p
		    || CHAR_BYTE8_P (c)
		    || !CHAR_PRINTABLE_P (c) })
		{
		  /* C is a control character, non-ASCII space/hyphen,
		     raw-byte, or a non-printable character which must be
		     displayed either as '\003' or as `^C' where the '\\'
		     and '^' can be defined in the display table.  Fill
		     IT->ctl_chars with glyphs for what we have to
		     display.  Then, set IT->dpvec to these glyphs.  */
		  let mut gc: Lisp_Object;
		  let mut ctl_len: c_int;
		  let mut face_id: c_int;
		  let mut lface_id: c_int = 0;
		  let mut escape_glyph: c_int;

		  /* C's `display_control:` label is the epilogue of this block: the four
		     `goto display_control` sites break out of the labelled part and reach it
		     (PORTING.md 2.2).  */
		  'display_control: {
		    /* Handle control characters with ^.  */

		    if ASCII_CHAR_P (c as isize) && it_bits! (ctl_arrow_p, it)
		      {
			let mut g: c_int;

			g = '^' as c_int;	     /* default glyph for Control */
			/* Set IT->ctl_chars[0] to the glyph for `^'.  */
			if !(*it).dp.is_null ()
			    && { gc = DISP_CTRL_GLYPH ((*it).dp); GLYPH_CODE_P (gc) }
			  {
			    g = GLYPH_CODE_CHAR (gc);
			    lface_id = GLYPH_CODE_FACE (gc);
			  }

			face_id = if lface_id != 0
				  { merge_faces ((*it).w, Qt (), lface_id, (*it).face_id) }
				  else { merge_escape_glyph_face (it) };

			XSETINT! ((*it).ctl_chars[0], g as EMACS_INT);
			XSETINT! ((*it).ctl_chars[1], (c ^ 0o100) as EMACS_INT);
			ctl_len = 2;
			break 'display_control;
		      }

		    /* Handle non-ascii space in the mode where it only gets
		       highlighting.  */

		    if nonascii_space_p && EQ (Vnobreak_char_display, Qt ())
		      {
			/* Merge `nobreak-space' into the current face.  */
			face_id = merge_faces ((*it).w, Qnobreak_space (), 0,
					       (*it).face_id);
			XSETINT! ((*it).ctl_chars[0],
				  (if nobreak_char_ascii_display { ' ' as c_int }
				   else { (*it).c }) as EMACS_INT);
			ctl_len = 1;
			break 'display_control;
		      }

		    /* Handle non-ascii hyphens in the mode where it only
		       gets highlighting.  */

		    if nonascii_hyphen_p && EQ (Vnobreak_char_display, Qt ())
		      {
			/* Merge `nobreak-space' into the current face.  */
			face_id = merge_faces ((*it).w, Qnobreak_hyphen (), 0,
					       (*it).face_id);
			XSETINT! ((*it).ctl_chars[0],
				  (if nobreak_char_ascii_display { '-' as c_int }
				   else { (*it).c }) as EMACS_INT);
			ctl_len = 1;
			break 'display_control;
		      }

		    /* Handle sequences that start with the "escape glyph".  */

		    /* the default escape glyph is \.  */
		    escape_glyph = '\\' as c_int;

		    if !(*it).dp.is_null ()
			&& { gc = DISP_ESCAPE_GLYPH ((*it).dp); GLYPH_CODE_P (gc) }
		      {
			escape_glyph = GLYPH_CODE_CHAR (gc);
			lface_id = GLYPH_CODE_FACE (gc);
		      }

		    face_id = if lface_id != 0
			      { merge_faces ((*it).w, Qt (), lface_id, (*it).face_id) }
			      else { merge_escape_glyph_face (it) };

		    /* Draw non-ASCII space/hyphen with escape glyph: */

		    if nonascii_space_p || nonascii_hyphen_p
		      {
			XSETINT! ((*it).ctl_chars[0], escape_glyph as EMACS_INT);
			XSETINT! ((*it).ctl_chars[1],
				  (if nonascii_space_p { ' ' as c_int }
				   else { '-' as c_int }) as EMACS_INT);
			ctl_len = 2;
			break 'display_control;
		      }

		    {
		      let mut str: [c_char; 10] = [0; 10];
		      let mut len: c_int;
		      let mut i: c_int;

		      if CHAR_BYTE8_P (c)
			/* Display \200 or \x80 instead of \17777600.  */
			{ c = CHAR_TO_BYTE8 (c); }
		      let format_string: *const c_char = if display_raw_bytes_as_hex
							 { c"x%02x".as_ptr () }
							 else { c"%03o".as_ptr () };
		      len = sprintf (str.as_mut_ptr (), format_string, c as c_uint);

		      XSETINT! ((*it).ctl_chars[0], escape_glyph as EMACS_INT);
		      i = 0;
		      while i < len
			{
			  XSETINT! ((*it).ctl_chars[(i + 1) as usize], str[i as usize] as c_int as EMACS_INT);
			  i += 1;
			}
		      ctl_len = len + 1;
		    }
		  }

		  /* display_control: */
		  /* Set up IT->dpvec and return first character from it.  */
		  (*it).dpvec_char_len = (*it).len;
		  (*it).dpvec = (*it).ctl_chars.as_mut_ptr ();
		  (*it).dpend = (*it).dpvec.offset (ctl_len as isize);
		  (*it).current.dpvec_index = 0;
		  (*it).dpvec_face_id = face_id;
		  (*it).saved_face_id = (*it).face_id;
		  (*it).method = GET_FROM_DISPLAY_VECTOR;
		  it_bits! (ellipsis_p, it, false);
		  continue 'get_next;
		}
	      (*it).char_to_display = c;
	    }
	  else if success_p
	    {
	      (*it).char_to_display = (*it).c;
	    }
	}

      /* C 8687-8736's `#ifdef HAVE_WINDOW_SYSTEM` arm (the multibyte face adjustment with
	 `composition_gstring_from_id`/`face_for_font`/`composition_table`, and its
	 `FACE_FROM_ID_OR_NULL` glyphless fallback) is dead on this target (P-08); recorded in
	 docs/ports/xdisp.md.  */
      break;
    }

  /* done: */
  /* Is this character the last one of a run of characters with
     box?  If yes, set IT->end_of_box_run_p to true.  */
  if it_bits! (face_box_p, it)
      && (*it).s.is_null ()
    {
      if (*it).method == GET_FROM_STRING && (*it).sp != 0
	{
	  let face_id: c_int = underlying_face_id (it);
	  let face: *mut face = FACE_FROM_ID_OR_NULL ((*it).f, face_id);

	  if !face.is_null ()
	    {
	      if face_bits! (box, face) as face_box_type == FACE_NO_BOX
		{
		  /* If the box comes from face properties in a
		     display string, check faces in that string.  */
		  let string_face_id: c_int = face_after_it_pos! (it);
		  if face_bits! (box, FACE_FROM_ID ((*it).f, string_face_id)) as face_box_type
		     == FACE_NO_BOX
		    { it_bits! (end_of_box_run_p, it, true); }
		}
	      /* Otherwise, the box comes from the underlying face.
		 If this is the last string character displayed, check
		 the next buffer location.  */
	      else if ((IT_STRING_CHARPOS! (*it) >= SCHARS ((*it).string) - 1)
		       /* For a composition, see if the string ends
			  at the last character included in the
			  composition.  */
		       || ((*it).what == IT_COMPOSITION
			   && (IT_STRING_CHARPOS! (*it) + (*it).cmp_it.nchars as ptrdiff_t
			       >= SCHARS ((*it).string)))
		       /* n_overlay_strings is unreliable unless
			  overlay_string_index is non-negative.  */
		       && (((*it).current.overlay_string_index >= 0
			    && ((*it).current.overlay_string_index
				== (*it).n_overlay_strings - 1))
			   /* A string from display property.  */
			   || it_bits! (from_disp_prop_p, it)))
		{
		  let mut ignore: ptrdiff_t = 0;
		  let mut next_face_id: c_int;
		  let mut text_from_string: bool = false;
		  /* Normally, the next buffer location is stored in
		     IT->current.pos...  */
		  let mut pos: text_pos = (*it).current.pos;

		  /* ...but for a string from a display property, the
		     next buffer position is stored in the 'position'
		     member of the iteration stack slot below the
		     current one, see handle_single_display_spec.  By
		     contrast, it->current.pos was not yet updated to
		     point to that buffer position; that will happen
		     in pop_it, after we finish displaying the current
		     string.  Note that we already checked above that
		     it->sp is positive, so subtracting one from it is
		     safe.  */
		  if it_bits! (from_disp_prop_p, it)
		    {
		      let mut stackp: c_int = (*it).sp - 1;

		      /* Find the stack level with data from buffer.  */
		      while stackp >= 0
			    && STRINGP ((*(*it).stack.as_mut_ptr ().offset (stackp as isize)).string)
			{ stackp -= 1; }
		      if stackp < 0
			{
			  /* If no stack slot was found for iterating
			     a buffer, we are displaying text from a
			     string, most probably the mode line or
			     the header line, and that string has a
			     display string on some of its
			     characters.  */
			  text_from_string = true;
			  pos = (*it).stack [((*it).sp - 1) as usize].position;
			}
		      else
			{ pos = (*(*it).stack.as_mut_ptr ().offset (stackp as isize)).position; }
		    }
		  else
		    { INC_TEXT_POS! (pos, it_bits! (multibyte_p, it)); }

		  if text_from_string
		    {
		      let base_string: Lisp_Object
			= (*it).stack [((*it).sp - 1) as usize].string;

		      if CHARPOS! (pos) >= SCHARS (base_string) - 1
			{ it_bits! (end_of_box_run_p, it, true); }
		      else
			{
			  next_face_id
			    = face_at_string_position ((*it).w, base_string,
						       CHARPOS! (pos), 0,
						       &raw mut ignore, face_id,
						       false, 0);
			  if face_bits! (box, FACE_FROM_ID ((*it).f, next_face_id)) as face_box_type
			     == FACE_NO_BOX
			    { it_bits! (end_of_box_run_p, it, true); }
			}
		    }
		  else if CHARPOS! (pos) >= ZV! ()
		    { it_bits! (end_of_box_run_p, it, true); }
		  else
		    {
		      next_face_id =
			face_at_buffer_position ((*it).w, CHARPOS! (pos),
						 &raw mut ignore,
						 CHARPOS! (pos)
						 + TEXT_PROP_DISTANCE_LIMIT as ptrdiff_t,
						 false, -1, 0);
		      if face_bits! (box, FACE_FROM_ID ((*it).f, next_face_id)) as face_box_type
			 == FACE_NO_BOX
			{ it_bits! (end_of_box_run_p, it, true); }
		    }
		}
	    }
	}
      /* next_element_from_display_vector sets this flag according to
	 faces of the display vector glyphs, see there.  */
      else if (*it).method != GET_FROM_DISPLAY_VECTOR
	{
	  let face_id: c_int = face_after_it_pos! (it);
	  if face_id != (*it).face_id
	      && face_bits! (box, FACE_FROM_ID ((*it).f, face_id)) as face_box_type
		 == FACE_NO_BOX
	    { it_bits! (end_of_box_run_p, it, true); }
	}
    }
  /* If we reached the end of the object we've been iterating (e.g., a
     display string or an overlay string), and there's something on
     IT->stack, proceed with what's on the stack.  It doesn't make
     sense to return false if there's unprocessed stuff on the stack,
     because otherwise that stuff will never be displayed.  */
  if !success_p && (*it).sp > 0
    {
      set_iterator_to_next (it, false);
      success_p = get_next_display_element (it);
    }

  /* Value is false if end of buffer or string reached.  */
  return success_p;
}


/* Move IT to the next display element.

   RESEAT_P means if called on a newline in buffer text,
   skip to the next visible line start.

   Functions get_next_display_element and set_iterator_to_next are
   separate because I find this arrangement easier to handle than a
   get_next_display_element function that also increments IT's
   position.  The way it is we can first look at an iterator's current
   display element, decide whether it fits on a line, and if it does,
   increment the iterator position.  The other way around we probably
   would either need a flag indicating whether the iterator has to be
   incremented the next time, or we would have to implement a
   decrement position function which would not be easy to write.  */

unsafe fn set_iterator_to_next (it: *mut it, reseat_p: bool)
{

  if max_redisplay_ticks > 0
    { update_redisplay_ticks (1, (*it).w); }

  /* C's `switch` becomes a `match` whose `break`s leave this block, and the two arms that
     `goto consider_string_end` fall through the match into the label's body at its end
     (PORTING.md 2.2).  */
  'consider_string_end: {
    match (*it).method
      {
	GET_FROM_BUFFER =>
	  {
	    /* The current display element of IT is a character from
	       current_buffer.  Advance in the buffer, and maybe skip over
	       invisible lines that are so because of selective display.  */
	    if ITERATOR_AT_END_OF_LINE_P! (it) && reseat_p
	      { reseat_at_next_visible_line_start (it, false); }
	    else if (*it).cmp_it.id >= 0
	      {
		/* We are currently getting glyphs from a composition.  */
		if !it_bits! (bidi_p, it)
		  {
		    IT_CHARPOS! (*it) += (*it).cmp_it.nchars as ptrdiff_t;
		    IT_BYTEPOS! (*it) += (*it).cmp_it.nbytes as ptrdiff_t;
		  }
		else
		  {
		    let mut i: c_int;

		    /* Update IT's char/byte positions to point to the first
		       character of the next grapheme cluster, or to the
		       character visually after the current composition.  */
		    i = 0;
		    while i < (*it).cmp_it.nchars
		      {
			bidi_move_to_visually_next (&raw mut (*it).bidi_it);
			i += 1;
		      }
		    IT_BYTEPOS! (*it) = (*it).bidi_it.bytepos;
		    IT_CHARPOS! (*it) = (*it).bidi_it.charpos;
		  }

		if (!it_bits! (bidi_p, it) || !(*it).cmp_it.reversed_p)
		    && (*it).cmp_it.to < (*it).cmp_it.nglyphs
		  {
		    /* Composition created while scanning forward.  Proceed
		       to the next grapheme cluster.  */
		    (*it).cmp_it.from = (*it).cmp_it.to;
		  }
		else if (it_bits! (bidi_p, it) && (*it).cmp_it.reversed_p)
		    && (*it).cmp_it.from > 0
		  {
		    /* Composition created while scanning backward.  Proceed
		       to the previous grapheme cluster.  */
		    (*it).cmp_it.to = (*it).cmp_it.from;
		  }
		else
		  {
		    /* No more grapheme clusters in this composition.
		       Find the next stop position.  */
		    let mut stop: ptrdiff_t = (*it).end_charpos;

		    if (*it).bidi_it.scan_dir < 0
		      /* Now we are scanning backward; figure out where to stop.  */
		      { stop = bidi_level_start ((*it).bidi_it.resolved_level as c_int) - 1; }
		    composition_compute_stop_pos (&raw mut (*it).cmp_it as *mut c_void,
						  IT_CHARPOS! (*it),
						  IT_BYTEPOS! (*it), stop, Qnil (), true);
		  }
	      }
	    else
	      {
		eassert! ((*it).len != 0);

		if !it_bits! (bidi_p, it)
		  {
		    IT_BYTEPOS! (*it) += (*it).len as ptrdiff_t;
		    IT_CHARPOS! (*it) += 1;
		  }
		else
		  {
		    let prev_scan_dir: c_int = (*it).bidi_it.scan_dir;
		    /* If this is a new paragraph, determine its base
		       direction (a.k.a. its base embedding level).  */
		    if bidi_it_bits! (new_paragraph, &raw mut (*it).bidi_it)
		      { bidi_paragraph_init ((*it).paragraph_embedding, &raw mut (*it).bidi_it,
					     false); }
		    bidi_move_to_visually_next (&raw mut (*it).bidi_it);
		    IT_BYTEPOS! (*it) = (*it).bidi_it.bytepos;
		    IT_CHARPOS! (*it) = (*it).bidi_it.charpos;
		    if prev_scan_dir != (*it).bidi_it.scan_dir
		      {
			/* As the scan direction was changed, we must
			   re-compute the stop position for composition.  */
			let mut stop: ptrdiff_t = (*it).end_charpos;
			if (*it).bidi_it.scan_dir < 0
			  { stop = bidi_level_start ((*it).bidi_it.resolved_level as c_int) - 1; }
			composition_compute_stop_pos (&raw mut (*it).cmp_it as *mut c_void,
						      IT_CHARPOS! (*it),
						      IT_BYTEPOS! (*it), stop, Qnil (),
						      true);
		      }
		  }
		eassert! (IT_BYTEPOS! (*it) == CHAR_TO_BYTE (IT_CHARPOS! (*it)));
	      }
	    break 'consider_string_end;
	  }

	GET_FROM_C_STRING =>
	  {
	    /* Current display element of IT is from a C string.  */
	    if !it_bits! (bidi_p, it)
		/* If the string position is beyond string's end, it means
		   next_element_from_c_string is padding the string with
		   blanks, in which case we bypass the bidi iterator,
		   because it cannot deal with such virtual characters.  */
		|| IT_CHARPOS! (*it) >= (*it).bidi_it.string.schars
	      {
		IT_BYTEPOS! (*it) += (*it).len as ptrdiff_t;
		IT_CHARPOS! (*it) += 1;
	      }
	    else
	      {
		bidi_move_to_visually_next (&raw mut (*it).bidi_it);
		IT_BYTEPOS! (*it) = (*it).bidi_it.bytepos;
		IT_CHARPOS! (*it) = (*it).bidi_it.charpos;
	      }
	    break 'consider_string_end;
	  }

	GET_FROM_DISPLAY_VECTOR =>
	  {
	    /* Current display element of IT is from a display table entry.
	       Advance in the display table definition.  Reset it to null if
	       end reached, and continue with characters from buffers/
	       strings.  */
	    (*it).current.dpvec_index += 1;

	    /* Restore face of the iterator to what they were before the
	       display vector entry (these entries may contain faces).  */
	    (*it).face_id = (*it).saved_face_id;

	    if (*it).dpvec.offset ((*it).current.dpvec_index as isize) >= (*it).dpend
	      {
		let recheck_faces: bool = it_bits! (ellipsis_p, it);

		if !(*it).s.is_null ()
		  { (*it).method = GET_FROM_C_STRING; }
		else if STRINGP ((*it).string)
		  { (*it).method = GET_FROM_STRING; }
		else
		  {
		    (*it).method = GET_FROM_BUFFER;
		    (*it).object = (*(*it).w).contents;
		  }

		(*it).dpvec = core::ptr::null_mut ();
		(*it).current.dpvec_index = -1;

		/* Skip over characters which were displayed via IT->dpvec.  */
		if (*it).dpvec_char_len < 0
		  { reseat_at_next_visible_line_start (it, true); }
		else if (*it).dpvec_char_len > 0
		  {
		    (*it).len = (*it).dpvec_char_len;
		    set_iterator_to_next (it, reseat_p);
		  }

		/* Maybe recheck faces after display vector.  */
		if recheck_faces
		  {
		    if (*it).method == GET_FROM_STRING
		      { (*it).stop_charpos = IT_STRING_CHARPOS! (*it); }
		    else
		      { (*it).stop_charpos = IT_CHARPOS! (*it); }
		  }
	      }
	    break 'consider_string_end;
	  }

	GET_FROM_STRING =>
	  {
	    /* Current display element is a character from a Lisp string.  */
	    eassert! ((*it).s.is_null () && STRINGP ((*it).string));
	    /* Don't advance past string end.  These conditions are true
	       when set_iterator_to_next is called at the end of
	       get_next_display_element, in which case the Lisp string is
	       already exhausted, and all we want is pop the iterator
	       stack.  */
	    /* C's two `goto consider_string_end` leave this arm early; the port wraps the code
	       they skip so that both they and the arm's natural end reach the label's body
	       below the match (PORTING.md 2.2).  */
	    'string_arm: {
	      if (*it).current.overlay_string_index >= 0
		{
		  /* This is an overlay string, so there's no padding with
		     spaces, and the number of characters in the string is
		     where the string ends.  */
		  if IT_STRING_CHARPOS! (*it) >= SCHARS ((*it).string)
		    { break 'string_arm; }
		}
	      else
		{
		  /* Not an overlay string.  There could be padding, so test
		     against it->end_charpos.  */
		  if IT_STRING_CHARPOS! (*it) >= (*it).end_charpos
		    { break 'string_arm; }
		}
	      if (*it).cmp_it.id >= 0
		{
		  /* We are delivering display elements from a composition.
		     Update the string position past the grapheme cluster
		     we've just processed.  */
		  if !it_bits! (bidi_p, it)
		    {
		      IT_STRING_CHARPOS! (*it) += (*it).cmp_it.nchars as ptrdiff_t;
		      IT_STRING_BYTEPOS! (*it) += (*it).cmp_it.nbytes as ptrdiff_t;
		    }
		  else
		    {
		      let mut i: c_int;

		      i = 0;
		      while i < (*it).cmp_it.nchars
			{
			  bidi_move_to_visually_next (&raw mut (*it).bidi_it);
			  i += 1;
			}
		      IT_STRING_BYTEPOS! (*it) = (*it).bidi_it.bytepos;
		      IT_STRING_CHARPOS! (*it) = (*it).bidi_it.charpos;
		    }

		  /* Did we exhaust all the grapheme clusters of this
		     composition?  */
		  if (!it_bits! (bidi_p, it) || !(*it).cmp_it.reversed_p)
		      && ((*it).cmp_it.to < (*it).cmp_it.nglyphs)
		    {
		      /* Not all the grapheme clusters were processed yet;
			 advance to the next cluster.  */
		      (*it).cmp_it.from = (*it).cmp_it.to;
		    }
		  else if (it_bits! (bidi_p, it) && (*it).cmp_it.reversed_p)
		      && (*it).cmp_it.from > 0
		    {
		      /* Likewise: advance to the next cluster, but going in
			 the reverse direction.  */
		      (*it).cmp_it.to = (*it).cmp_it.from;
		    }
		  else
		    {
		      /* This composition was fully processed; find the next
			 candidate place for checking for composed
			 characters.  */
		      /* Always limit string searches to the string length;
			 any padding spaces are not part of the string, and
			 there cannot be any compositions in that padding.  */
		      let mut stop: ptrdiff_t = SCHARS ((*it).string);

		      if it_bits! (bidi_p, it) && (*it).bidi_it.scan_dir < 0
			{ stop = -1; }
		      else if (*it).end_charpos < stop
			{
			  /* Cf. PRECISION in reseat_to_string: we might be
			     limited in how many of the string characters we
			     need to deliver.  */
			  stop = (*it).end_charpos;
			}
		      composition_compute_stop_pos (&raw mut (*it).cmp_it as *mut c_void,
						    IT_STRING_CHARPOS! (*it),
						    IT_STRING_BYTEPOS! (*it), stop,
						    (*it).string, true);
		    }
		}
	      else
		{
		  if !it_bits! (bidi_p, it)
		      /* If the string position is beyond string's end, it
			 means next_element_from_string is padding the string
			 with blanks, in which case we bypass the bidi
			 iterator, because it cannot deal with such virtual
			 characters.  */
		      || IT_STRING_CHARPOS! (*it) >= (*it).bidi_it.string.schars
		    {
		      IT_STRING_BYTEPOS! (*it) += (*it).len as ptrdiff_t;
		      IT_STRING_CHARPOS! (*it) += 1;
		    }
		  else
		    {
		      let prev_scan_dir: c_int = (*it).bidi_it.scan_dir;

		      bidi_move_to_visually_next (&raw mut (*it).bidi_it);
		      IT_STRING_BYTEPOS! (*it) = (*it).bidi_it.bytepos;
		      IT_STRING_CHARPOS! (*it) = (*it).bidi_it.charpos;
		      /* If the scan direction changes, we may need to update
			 the place where to check for composed characters.  */
		      if prev_scan_dir != (*it).bidi_it.scan_dir
			{
			  let mut stop: ptrdiff_t = SCHARS ((*it).string);

			  if (*it).bidi_it.scan_dir < 0
			    { stop = -1; }
			  else if (*it).end_charpos < stop
			    { stop = (*it).end_charpos; }

			  composition_compute_stop_pos (&raw mut (*it).cmp_it as *mut c_void,
							IT_STRING_CHARPOS! (*it),
							IT_STRING_BYTEPOS! (*it), stop,
							(*it).string, true);
			}
		    }
		}
	    }
	    /* consider_string_end (C's label, shared with the arm below).  */
	  }

	GET_FROM_IMAGE | GET_FROM_STRETCH | GET_FROM_XWIDGET =>
	  {
	    /* The position etc with which we have to proceed are on
	       the stack.  The position may be at the end of a string,
	       if the `display' property takes up the whole string.  */
	    eassert! ((*it).sp > 0);
	    pop_it (it);
	    if !((*it).method == GET_FROM_STRING)
	      { break 'consider_string_end; }
	    /* C's `goto consider_string_end` falls through to the label's body below.  */
	  }

	_ =>
	  {
	    /* There are no other methods defined, so this should be a bug.  */
	    emacs_abort ();
	  }
      }

    /* consider_string_end: the two `goto`s from the arms above enter this body by falling
       out of the match, and C's backward `goto consider_string_end` inside the body is the
       port's `continue` (PORTING.md 2.2).  */
    loop
      {
	if (*it).current.overlay_string_index >= 0
	  {
	    /* IT->string is an overlay string.  Advance to the
	       next, if there is one.  */
	    if IT_STRING_CHARPOS! (*it) >= SCHARS ((*it).string)
	      {
		/* Maybe add a stretch glyph if the string had 'min-width'
		   display spec.  */
		display_min_width (it, IT_STRING_CHARPOS! (*it), (*it).string,
				   Qnil ());
		it_bits! (ellipsis_p, it, false);
		next_overlay_string (it);
		if it_bits! (ellipsis_p, it)
		  { setup_for_ellipsis (it, 0); }
	      }
	  }
	else
	  {
	    /* IT->string is not an overlay string.  If we reached
	       its end, and there is something on IT->stack, proceed
	       with what is on the stack.  This can be either another
	       string, this time an overlay string, or a buffer.  */
	    if IT_STRING_CHARPOS! (*it) == SCHARS ((*it).string)
		&& (*it).sp > 0
	      {
		/* Maybe add a stretch glyph if the string had 'min-width'
		   display spec.  We only do this if it->sp > 0 because
		   mode-line strings are handled differently, see
		   display_min_width.  */
		display_min_width (it, IT_STRING_CHARPOS! (*it), (*it).string,
				   Qnil ());
		pop_it (it);
		if (*it).method == GET_FROM_STRING
		  { continue; }
	      }
	  }
	break;
      }
  }

  eassert! ((*it).method != GET_FROM_STRING
	    || (STRINGP ((*it).string)
		&& IT_STRING_CHARPOS! (*it) >= 0));
}

/* Load IT's display element fields with information about the next
   display element which comes from a display table entry or from the
   result of translating a control character to one of the forms `^C'
   or `\003'.

   IT->dpvec holds the glyphs to return as characters.
   IT->saved_face_id holds the face id before the display vector--it
   is restored into IT->face_id in set_iterator_to_next.  */

unsafe extern "C" fn next_element_from_display_vector (it: *mut it) -> bool
{
  let mut gc: Lisp_Object;
  let prev_face_id: c_int = (*it).face_id;
  let mut next_face_id: c_int;

  /* Precondition.  */
  eassert! (!(*it).dpvec.is_null () && (*it).current.dpvec_index >= 0);

  (*it).face_id = (*it).saved_face_id;

  /* KFS: This code used to check ip->dpvec[0] instead of the current element.
     That seemed totally bogus - so I changed it...  */
  if (*it).dpend.offset_from ((*it).dpvec) > 0	/* empty dpvec[] is invalid */
      && { gc = *(*it).dpvec.offset ((*it).current.dpvec_index as isize); GLYPH_CODE_P (gc) }
    {
      let this_face: *mut face;
      let prev_face: *mut face;
      let next_face: *mut face;

      (*it).c = GLYPH_CODE_CHAR (gc);
      (*it).len = CHAR_BYTES ((*it).c);
      /* The character code in the display vector could be non-ASCII, in
	 which case we must make the iterator multibyte, so that a
	 suitable font for the character is looked up.  But don't do
	 that if the original character came from a unibyte buffer.  */
      if !ASCII_CHAR_P ((*it).c as isize)
	  && !it_bits! (multibyte_p, it)
	  && !((((*it).sp == 0 && BUFFERP ((*it).object))
		|| ((*it).sp > 1
		    && !NILP ((*it).stack[0].string)))
	       && NILP (BVAR! (current_buffer, enable_multibyte_characters_)))
	{ it_bits! (multibyte_p, it, true); }

      /* The entry may contain a face id to use.  Such a face id is
	 the id of a Lisp face, not a realized face.  A face id of
	 zero means no face is specified.  */
      if (*it).dpvec_face_id >= 0
	{ (*it).face_id = (*it).dpvec_face_id; }
      else
	{
	  let lface_id: c_int = GLYPH_CODE_FACE (gc);
	  if lface_id > 0
	    { (*it).face_id = merge_faces ((*it).w, Qt (), lface_id,
					   (*it).saved_face_id); }
	}

      /* Glyphs in the display vector could have the box face, so we
	 need to set the related flags in the iterator, as
	 appropriate.  */
      this_face = FACE_FROM_ID_OR_NULL ((*it).f, (*it).face_id);
      prev_face = FACE_FROM_ID_OR_NULL ((*it).f, prev_face_id);

      /* Is this character the first character of a box-face run?  */
      it_bits! (start_of_box_run_p, it,
		!this_face.is_null ()
		&& face_bits! (box, this_face) as face_box_type != FACE_NO_BOX
		&& (prev_face.is_null ()
		    || face_bits! (box, prev_face) as face_box_type == FACE_NO_BOX));

      /* For the last character of the box-face run, we need to look
	 either at the next glyph from the display vector, or at the
	 face we saw before the display vector.  */
      next_face_id = (*it).saved_face_id;
      /* A line that starts with `<` would parse as generics, so the cast keeps its parentheses.  */
  if ((*it).current.dpvec_index as isize)
     < (*it).dpend.offset_from ((*it).dpvec) - 1
	{
	  if (*it).dpvec_face_id >= 0
	    { next_face_id = (*it).dpvec_face_id; }
	  else
	    {
	      let gc: Lisp_Object
		= *(*it).dpvec.offset (((*it).current.dpvec_index + 1) as isize);
	      let lface_id: c_int = if GLYPH_CODE_P (gc) { GLYPH_CODE_FACE (gc) } else { 0 };

	      if lface_id > 0
		{ next_face_id = merge_faces ((*it).w, Qt (), lface_id,
					      (*it).saved_face_id); }
	    }
	}
      next_face = FACE_FROM_ID_OR_NULL ((*it).f, next_face_id);
      if !this_face.is_null ()
	  && face_bits! (box, this_face) as face_box_type != FACE_NO_BOX
	  && (next_face.is_null ()
	      || face_bits! (box, next_face) as face_box_type == FACE_NO_BOX)
	{ it_bits! (end_of_box_run_p, it, true); }
      it_bits! (face_box_p, it,
		!this_face.is_null ()
		&& face_bits! (box, this_face) as face_box_type != FACE_NO_BOX);
    }
  else
    /* Display table entry is invalid.  Return a space.  */
    {
      (*it).c = ' ' as c_int;
      (*it).len = 1;
    }

  /* Don't change position and object of the iterator here.  They are
     still the values of the character that had this display table
     entry or was translated, and that's what we want.  */
  (*it).what = IT_CHARACTER;
  return true;
}

/* Get the first element of string/buffer in the visual order, after
   being reseated to a new position in a string or a buffer.  */
unsafe fn get_visually_first_element (it: *mut it)
{
  let string_p: bool = STRINGP ((*it).string) || !(*it).s.is_null ();
  let eob: ptrdiff_t = if string_p { (*it).bidi_it.string.schars } else { ZV! () };
  let mut bob: ptrdiff_t;
  let obegv: ptrdiff_t = BEGV! ();

  SET_WITH_NARROWED_BEGV! (it, bob,
			   if string_p { 0 }
			   else if IT_CHARPOS! (*it) < BEGV! () { obegv }
			   else { BEGV! () },
			   (*it).medium_narrowing_begv);

  if STRINGP ((*it).string)
    {
      (*it).bidi_it.charpos = IT_STRING_CHARPOS! (*it);
      (*it).bidi_it.bytepos = IT_STRING_BYTEPOS! (*it);
    }
  else
    {
      (*it).bidi_it.charpos = IT_CHARPOS! (*it);
      (*it).bidi_it.bytepos = IT_BYTEPOS! (*it);
    }

  if (*it).bidi_it.charpos == eob
    {
      /* Nothing to do, but reset the FIRST_ELT flag, like
	 bidi_paragraph_init does, because we are not going to
	 call it.  */
      bidi_it_bits! (first_elt, &raw mut (*it).bidi_it, false);
    }
  else if (*it).bidi_it.charpos == bob
	   || (!string_p
	       && (FETCH_BYTE! ((*it).bidi_it.bytepos - 1) == b'\n'
		   || FETCH_BYTE! ((*it).bidi_it.bytepos) == b'\n'))
    {
      /* If we are at the beginning of a line/string, we can produce
	 the next element right away.  */
      bidi_paragraph_init ((*it).paragraph_embedding, &raw mut (*it).bidi_it, true);
      bidi_move_to_visually_next (&raw mut (*it).bidi_it);
    }
  else
    {
      let orig_bytepos: ptrdiff_t = (*it).bidi_it.bytepos;

      /* We need to prime the bidi iterator starting at the line's or
	 string's beginning, before we will be able to produce the
	 next element.  */
      if string_p
	{
	  (*it).bidi_it.charpos = 0;
	  (*it).bidi_it.bytepos = 0;
	}
      else
	{ SET_WITH_NARROWED_BEGV! (it, (*it).bidi_it.charpos,
				   find_newline_no_quit (IT_CHARPOS! (*it),
							 IT_BYTEPOS! (*it), -1,
							 &raw mut (*it).bidi_it.bytepos),
				   (*it).medium_narrowing_begv); }
      bidi_paragraph_init ((*it).paragraph_embedding, &raw mut (*it).bidi_it, true);
      loop
	{
	  /* Now return to buffer/string position where we were asked
	     to get the next display element, and produce that.  */
	  bidi_move_to_visually_next (&raw mut (*it).bidi_it);
	  if !((*it).bidi_it.bytepos != orig_bytepos
	       && (*it).bidi_it.charpos < eob) { break; }
	}
    }

  /*  Adjust IT's position information to where we ended up.  */
  if STRINGP ((*it).string)
    {
      IT_STRING_CHARPOS! (*it) = (*it).bidi_it.charpos;
      IT_STRING_BYTEPOS! (*it) = (*it).bidi_it.bytepos;
    }
  else
    {
      IT_CHARPOS! (*it) = (*it).bidi_it.charpos;
      IT_BYTEPOS! (*it) = (*it).bidi_it.bytepos;
    }

  if STRINGP ((*it).string) || (*it).s.is_null ()
    {
      let mut stop: ptrdiff_t;
      let charpos: ptrdiff_t;
      let bytepos: ptrdiff_t;

      if STRINGP ((*it).string)
	{
	  eassert! ((*it).s.is_null ());
	  stop = SCHARS ((*it).string);
	  if stop > (*it).end_charpos
	    { stop = (*it).end_charpos; }
	  charpos = IT_STRING_CHARPOS! (*it);
	  bytepos = IT_STRING_BYTEPOS! (*it);
	}
      else
	{
	  stop = (*it).end_charpos;
	  charpos = IT_CHARPOS! (*it);
	  bytepos = IT_BYTEPOS! (*it);
	}
      if (*it).bidi_it.scan_dir < 0
	{ stop = if STRINGP ((*it).string)
		 { -1 }
		 else { bidi_level_start ((*it).bidi_it.resolved_level as c_int) - 1 }; }
      composition_compute_stop_pos (&raw mut (*it).cmp_it as *mut c_void,
				    charpos, bytepos, stop, (*it).string, true);
    }
}

/* Load IT with the next display element from Lisp string IT->string.
   IT->current.string_pos is the current position within the string.
   If IT->current.overlay_string_index >= 0, the Lisp string is an
   overlay string.  */

unsafe extern "C" fn next_element_from_string (it: *mut it) -> bool
{
  let mut position: text_pos;

  eassert! (STRINGP ((*it).string));
  eassert! (!it_bits! (bidi_p, it) || EQ ((*it).string, (*it).bidi_it.string.lstring));
  eassert! (IT_STRING_CHARPOS! (*it) >= 0);
  position = (*it).current.string_pos;

  /* With bidi reordering, the character to display might not be the
     character at IT_STRING_CHARPOS.  BIDI_IT.FIRST_ELT means
     that we were reseat()ed to a new string, whose paragraph
     direction is not known.  */
  if it_bits! (bidi_p, it) && bidi_it_bits! (first_elt, &raw mut (*it).bidi_it)
    {
      get_visually_first_element (it);
      SET_TEXT_POS! (position, IT_STRING_CHARPOS! (*it), IT_STRING_BYTEPOS! (*it));
    }

  /* Time to check for invisible text?  */
  if IT_STRING_CHARPOS! (*it) < (*it).end_charpos
    {
      if IT_STRING_CHARPOS! (*it) >= (*it).stop_charpos
	{
	  if !(!it_bits! (bidi_p, it)
	       || BIDI_AT_BASE_LEVEL! (&raw mut (*it).bidi_it)
	       || IT_STRING_CHARPOS! (*it) == (*it).stop_charpos)
	    {
	      /* With bidi non-linear iteration, we could find
		 ourselves far beyond the last computed stop_charpos,
		 with several other stop positions in between that we
		 missed.  Scan them all now, in buffer's logical
		 order, until we find and handle the last stop_charpos
		 that precedes our current position.  */
	      handle_stop_backwards (it, (*it).stop_charpos);
	      return GET_NEXT_DISPLAY_ELEMENT! (it);
	    }
	  else
	    {
	      if it_bits! (bidi_p, it)
		{
		  /* Take note of the stop position we just moved
		     across, for when we will move back across it.  */
		  (*it).prev_stop = (*it).stop_charpos;
		  /* If we are at base paragraph embedding level, take
		     note of the last stop position seen at this
		     level.  */
		  if BIDI_AT_BASE_LEVEL! (&raw mut (*it).bidi_it)
		    { (*it).base_level_stop = (*it).stop_charpos; }
		}
	      handle_stop (it);

	      /* Since a handler may have changed IT->method, we must
		 recurse here.  */
	      return GET_NEXT_DISPLAY_ELEMENT! (it);
	    }
	}
      else if it_bits! (bidi_p, it)
	       /* If we are before prev_stop, we may have overstepped
		  on our way backwards a stop_pos, and if so, we need
		  to handle that stop_pos.  */
	       && IT_STRING_CHARPOS! (*it) < (*it).prev_stop
	       /* We can sometimes back up for reasons that have nothing
		  to do with bidi reordering.  E.g., compositions.  The
		  code below is only needed when we are above the base
		  embedding level, so test for that explicitly.  */
	       && !BIDI_AT_BASE_LEVEL! (&raw mut (*it).bidi_it)
	{
	  /* If we lost track of base_level_stop, we have no better
	     place for handle_stop_backwards to start from than string
	     beginning.  This happens, e.g., when we were reseated to
	     the previous screenful of text by vertical-motion.  */
	  if (*it).base_level_stop <= 0
	      || IT_STRING_CHARPOS! (*it) < (*it).base_level_stop
	    { (*it).base_level_stop = 0; }
	  handle_stop_backwards (it, (*it).base_level_stop);
	  return GET_NEXT_DISPLAY_ELEMENT! (it);
	}
    }

  if (*it).current.overlay_string_index >= 0
    {
      /* Get the next character from an overlay string.  In overlay
	 strings, there is no field width or padding with spaces to
	 do.  */
      if IT_STRING_CHARPOS! (*it) >= SCHARS ((*it).string)
	{
	  (*it).what = IT_EOB;
	  return false;
	}
      else if CHAR_COMPOSED_P! (it, IT_STRING_CHARPOS! (*it),
				IT_STRING_BYTEPOS! (*it),
				if (*it).bidi_it.scan_dir < 0
				{ -1 }
				else { SCHARS ((*it).string) })
	       && next_element_from_composition (it)
	{
	  return true;
	}
      else if STRING_MULTIBYTE ((*it).string)
	{
	  let s: *const u8 = SDATA ((*it).string).offset (IT_STRING_BYTEPOS! (*it));
	  (*it).c = check_char_and_length (s, &raw mut (*it).len);
	}
      else
	{
	  (*it).c = SREF ((*it).string, IT_STRING_BYTEPOS! (*it) as usize) as c_int;
	  (*it).len = 1;
	}
    }
  else
    {
      /* Get the next character from a Lisp string that is not an
	 overlay string.  Such strings come from the mode line, for
	 example.  We may have to pad with spaces, or truncate the
	 string.  See also next_element_from_c_string.  */
      if IT_STRING_CHARPOS! (*it) >= (*it).end_charpos
	{
	  (*it).what = IT_EOB;
	  return false;
	}
      else if IT_STRING_CHARPOS! (*it) >= (*it).string_nchars
	{
	  /* Pad with spaces.  */
	  (*it).c = ' ' as c_int;
	  (*it).len = 1;
	  CHARPOS! (position) = -1;
	  BYTEPOS! (position) = -1;
	}
      else if CHAR_COMPOSED_P! (it, IT_STRING_CHARPOS! (*it),
				IT_STRING_BYTEPOS! (*it),
				if (*it).bidi_it.scan_dir < 0
				{ -1 }
				else { (*it).string_nchars })
	       && next_element_from_composition (it)
	{
	  return true;
	}
      else if STRING_MULTIBYTE ((*it).string)
	{
	  let s: *const u8 = SDATA ((*it).string).offset (IT_STRING_BYTEPOS! (*it));
	  (*it).c = check_char_and_length (s, &raw mut (*it).len);
	}
      else
	{
	  (*it).c = SREF ((*it).string, IT_STRING_BYTEPOS! (*it) as usize) as c_int;
	  (*it).len = 1;
	}
    }

  /* Record what we have and where it came from.  */
  (*it).what = IT_CHARACTER;
  (*it).object = (*it).string;
  (*it).position = position;
  return true;
}


/* Load IT with next display element from C string IT->s.
   IT->string_nchars is the maximum number of characters to return
   from the string.  IT->end_charpos may be greater than
   IT->string_nchars when this function is called, in which case we
   may have to return padding spaces.  Value is false if end of string
   reached, including padding spaces.  */

unsafe extern "C" fn next_element_from_c_string (it: *mut it) -> bool
{
  let mut success_p: bool = true;

  eassert! (!(*it).s.is_null ());
  eassert! (!it_bits! (bidi_p, it) || (*it).s == (*it).bidi_it.string.s);
  (*it).what = IT_CHARACTER;
  BYTEPOS! ((*it).position) = 0;
  CHARPOS! ((*it).position) = 0;
  (*it).object = make_fixnum (0);

  /* With bidi reordering, the character to display might not be the
     character at IT_CHARPOS.  BIDI_IT.FIRST_ELT means that
     we were reseated to a new string, whose paragraph direction is
     not known.  */
  if it_bits! (bidi_p, it) && bidi_it_bits! (first_elt, &raw mut (*it).bidi_it)
    { get_visually_first_element (it); }

  /* IT's position can be greater than IT->string_nchars in case a
     field width or precision has been specified when the iterator was
     initialized.  */
  if IT_CHARPOS! (*it) >= (*it).end_charpos
    {
      /* End of the game.  */
      (*it).what = IT_EOB;
      success_p = false;
    }
  else if IT_CHARPOS! (*it) >= (*it).string_nchars
    {
      /* Pad with spaces.  */
      (*it).c = ' ' as c_int;
      (*it).len = 1;
      BYTEPOS! ((*it).position) = -1;
      CHARPOS! ((*it).position) = -1;
    }
  else if it_bits! (multibyte_p, it)
    { (*it).c = check_char_and_length ((*it).s.offset (IT_BYTEPOS! (*it)),
				       &raw mut (*it).len); }
  else
    {
      (*it).c = *(*it).s.offset (IT_BYTEPOS! (*it)) as c_int;
      (*it).len = 1;
    }

  return success_p;
}


/* Set up IT to return characters from an ellipsis, if appropriate.
   The definition of the ellipsis glyphs may come from a display table
   entry.  This function fills IT with the first glyph from the
   ellipsis if an ellipsis is to be displayed.  */

unsafe fn next_element_from_ellipsis (it: *mut it) -> bool
{
  if it_bits! (selective_display_ellipsis_p, it)
    { setup_for_ellipsis (it, (*it).len); }
  else
    {
      /* The face at the current position may be different from the
	 face we find after the invisible text.  Remember what it
	 was in IT->saved_face_id, and signal that it's there by
	 setting face_before_selective_p.  */
      (*it).saved_face_id = (*it).face_id;
      (*it).method = GET_FROM_BUFFER;
      (*it).object = (*(*it).w).contents;
      reseat_at_next_visible_line_start (it, true);
      it_bits! (face_before_selective_p, it, true);
    }

  return GET_NEXT_DISPLAY_ELEMENT! (it);
}


/* Deliver an image display element.  The iterator IT is already
   filled with image information (done in handle_display_prop).  Value
   is always true.  */


unsafe extern "C" fn next_element_from_image (it: *mut it) -> bool
{
  (*it).what = IT_IMAGE;
  return true;
}

unsafe extern "C" fn next_element_from_xwidget (it: *mut it) -> bool
{
  (*it).what = IT_XWIDGET;
  return true;
}


/* Fill iterator IT with next display element from a stretch glyph
   property.  IT->object is the value of the text property.  Value is
   always true.  */

unsafe extern "C" fn next_element_from_stretch (it: *mut it) -> bool
{
  (*it).what = IT_STRETCH;
  return true;
}

/* Scan backwards from IT's current position until we find a stop
   position, or until BEGV.  This is called when we find ourself
   before both the last known prev_stop and base_level_stop while
   reordering bidirectional text.  */

unsafe fn compute_stop_pos_backwards (it: *mut it)
{
  let SCAN_BACK_LIMIT: c_int = 1000;
  /* C leaves `pos` to `SET_TEXT_POS`; Rust has no partial initialisation, so the port
     zero-initialises it (PORTING.md 1.5, deviation 1's rule).  */
  let mut pos: text_pos = core::mem::zeroed ();
  let save_current: display_pos = (*it).current;
  let save_position: text_pos = (*it).position;
  let mut charpos: ptrdiff_t = IT_CHARPOS! (*it);
  let where_we_are: ptrdiff_t = charpos;
  let save_stop_pos: ptrdiff_t = (*it).stop_charpos;
  let save_end_pos: ptrdiff_t = (*it).end_charpos;

  eassert! (NILP ((*it).string) && (*it).s.is_null ());
  eassert! (it_bits! (bidi_p, it));
  it_bits! (bidi_p, it, false);
  loop
    {
      (*it).end_charpos = min (charpos + 1, ZV! ());
      charpos = max (charpos - SCAN_BACK_LIMIT as ptrdiff_t, BEGV! ());
      SET_TEXT_POS! (pos, charpos, CHAR_TO_BYTE (charpos));
      reseat_1 (it, pos, false);
      compute_stop_pos (it);
      /* We must advance forward, right?  */
      if (*it).stop_charpos <= charpos
	{ emacs_abort (); }
      if !(charpos > BEGV! () && (*it).stop_charpos >= (*it).end_charpos) { break; }
    }

  if (*it).stop_charpos <= where_we_are
    { (*it).prev_stop = (*it).stop_charpos; }
  else
    { (*it).prev_stop = BEGV! (); }
  it_bits! (bidi_p, it, true);
  (*it).current = save_current;
  (*it).position = save_position;
  (*it).stop_charpos = save_stop_pos;
  (*it).end_charpos = save_end_pos;
}

/* Scan forward from CHARPOS in the current buffer/string, until we
   find a stop position > current IT's position.  Then handle the stop
   position before that.  This is called when we bump into a stop
   position while reordering bidirectional text.  CHARPOS should be
   the last previously processed stop_pos (or BEGV/0, if none were
   processed yet) whose position is less than IT's current
   position.  */

unsafe fn handle_stop_backwards (it: *mut it, mut charpos: ptrdiff_t)
{
  let bufp: bool = !STRINGP ((*it).string);
  let where_we_are: ptrdiff_t
    = if bufp { IT_CHARPOS! (*it) } else { IT_STRING_CHARPOS! (*it) };
  let save_current: display_pos = (*it).current;
  let save_position: text_pos = (*it).position;
  let save_cmp_it: composition_it = (*it).cmp_it;
  let save_sp: c_int = (*it).sp;
  /* C leaves `pos1` to `SET_TEXT_POS`; Rust has no partial initialisation, so the port
     zero-initialises it (PORTING.md 1.5, deviation 1's rule).  */
  let mut pos1: text_pos = core::mem::zeroed ();
  let mut next_stop: ptrdiff_t;

  /* Scan in strict logical order.  */
  eassert! (it_bits! (bidi_p, it));
  it_bits! (bidi_p, it, false);
  loop
    {
      (*it).prev_stop = charpos;
      if bufp
	{
	  SET_TEXT_POS! (pos1, charpos, CHAR_TO_BYTE (charpos));
	  reseat_1 (it, pos1, false);
	}
      else
	{ (*it).current.string_pos = string_pos (charpos, (*it).string); }
      compute_stop_pos (it);
      /* We must advance forward, right?  */
      if (*it).stop_charpos <= (*it).prev_stop
	{ emacs_abort (); }
      charpos = (*it).stop_charpos;
      if !(charpos <= where_we_are) { break; }
    }

  it_bits! (bidi_p, it, true);
  (*it).current = save_current;
  (*it).position = save_position;
  (*it).cmp_it = save_cmp_it;
  next_stop = (*it).stop_charpos;
  (*it).stop_charpos = (*it).prev_stop;
  handle_stop (it);
  if (*it).sp == save_sp
    { (*it).stop_charpos = next_stop; }
}

/* Load IT with the next display element from current_buffer.  Value
   is false if end of buffer reached.  IT->stop_charpos is the next
   position at which to stop and check for text properties or buffer
   end.  */

unsafe extern "C" fn next_element_from_buffer (it: *mut it) -> bool
{
  let mut success_p: bool = true;

  eassert! (IT_CHARPOS! (*it) >= BEGV! ());
  eassert! (NILP ((*it).string) && (*it).s.is_null ());
  eassert! (!it_bits! (bidi_p, it)
	    || (NILP ((*it).bidi_it.string.lstring)
		&& (*it).bidi_it.string.s.is_null ()));

  /* With bidi reordering, the character to display might not be the
     character at IT_CHARPOS.  BIDI_IT.FIRST_ELT means that
     we were reseat()ed to a new buffer position, which is potentially
     a different paragraph.  */
  if it_bits! (bidi_p, it) && bidi_it_bits! (first_elt, &raw mut (*it).bidi_it)
    {
      get_visually_first_element (it);
      SET_TEXT_POS! ((*it).position, IT_CHARPOS! (*it), IT_BYTEPOS! (*it));
    }

  if IT_CHARPOS! (*it) >= (*it).stop_charpos
    {
      if IT_CHARPOS! (*it) >= (*it).end_charpos
	{
	  let mut overlay_strings_follow_p: bool;

	  /* End of the game, except when overlay strings follow that
	     haven't been returned yet.  */
	  if it_bits! (overlay_strings_at_end_processed_p, it)
	    { overlay_strings_follow_p = false; }
	  else
	    {
	      it_bits! (overlay_strings_at_end_processed_p, it, true);
	      overlay_strings_follow_p = get_overlay_strings (it, 0);
	    }

	  if overlay_strings_follow_p
	    { success_p = GET_NEXT_DISPLAY_ELEMENT! (it); }
	  else
	    {
	      (*it).what = IT_EOB;
	      (*it).position = (*it).current.pos;
	      success_p = false;
	    }
	}
      else if !(!it_bits! (bidi_p, it)
		|| BIDI_AT_BASE_LEVEL! (&raw mut (*it).bidi_it)
		|| IT_CHARPOS! (*it) == (*it).stop_charpos)
	{
	  /* With bidi non-linear iteration, we could find ourselves
	     far beyond the last computed stop_charpos, with several
	     other stop positions in between that we missed.  Scan
	     them all now, in buffer's logical order, until we find
	     and handle the last stop_charpos that precedes our
	     current position.  */
	  handle_stop_backwards (it, (*it).stop_charpos);
	  it_bits! (ignore_overlay_strings_at_pos_p, it, false);
	  return GET_NEXT_DISPLAY_ELEMENT! (it);
	}
      else
	{
	  if it_bits! (bidi_p, it)
	    {
	      /* Take note of the stop position we just moved across,
		 for when we will move back across it.  */
	      (*it).prev_stop = (*it).stop_charpos;
	      /* If we are at base paragraph embedding level, take
		 note of the last stop position seen at this
		 level.  */
	      if BIDI_AT_BASE_LEVEL! (&raw mut (*it).bidi_it)
		{ (*it).base_level_stop = (*it).stop_charpos; }
	    }
	  handle_stop (it);
	  it_bits! (ignore_overlay_strings_at_pos_p, it, false);
	  return GET_NEXT_DISPLAY_ELEMENT! (it);
	}
    }
  else if it_bits! (bidi_p, it)
	   /* If we are before prev_stop, we may have overstepped on
	      our way backwards a stop_pos, and if so, we need to
	      handle that stop_pos.  */
	   && IT_CHARPOS! (*it) < (*it).prev_stop
	   /* We can sometimes back up for reasons that have nothing
	      to do with bidi reordering.  E.g., compositions.  The
	      code below is only needed when we are above the base
	      embedding level, so test for that explicitly.  */
	   && !BIDI_AT_BASE_LEVEL! (&raw mut (*it).bidi_it)
    {
      if (*it).base_level_stop <= 0
	  || IT_CHARPOS! (*it) < (*it).base_level_stop
	{
	  /* If we lost track of base_level_stop, we need to find
	     prev_stop by looking backwards.  This happens, e.g., when
	     we were reseated to the previous screenful of text by
	     vertical-motion.  */
	  (*it).base_level_stop = BEGV! ();
	  compute_stop_pos_backwards (it);
	  handle_stop_backwards (it, (*it).prev_stop);
	}
      else
	{ handle_stop_backwards (it, (*it).base_level_stop); }
      it_bits! (ignore_overlay_strings_at_pos_p, it, false);
      return GET_NEXT_DISPLAY_ELEMENT! (it);
    }
  else
    {
      /* No face changes, overlays etc. in sight, so just return a
	 character from current_buffer.  */
      let mut p: *mut u8;
      let mut stop: ptrdiff_t;

      /* We moved to the next buffer position, so any info about
	 previously seen overlays is no longer valid.  */
      it_bits! (ignore_overlay_strings_at_pos_p, it, false);

      if composition_break_at_point
	  && !NILP (BVAR! (current_buffer, enable_multibyte_characters_))
	  && !NILP (Vauto_composition_mode)
	{
	  /* Limit search for composable characters to point's position.  */
	  if (*it).bidi_it.scan_dir < 0
	    { stop = if PT! () <= IT_CHARPOS! (*it) { PT! () } else { -1 }; }
	  else
	    { stop = if IT_CHARPOS! (*it) < PT! ()
			&& PT! () < (*it).end_charpos
		     { PT! () }
		     else { (*it).end_charpos }; }
	}
      else
	{ stop = if (*it).bidi_it.scan_dir < 0
		 { bidi_level_start ((*it).bidi_it.resolved_level as c_int) - 1 }
		 else { (*it).end_charpos }; }
      if CHAR_COMPOSED_P! (it, IT_CHARPOS! (*it), IT_BYTEPOS! (*it), stop)
	  && next_element_from_composition (it)
	{
	  return true;
	}

      /* Get the next character, maybe multibyte.  */
      p = BYTE_POS_ADDR (IT_BYTEPOS! (*it));
      if it_bits! (multibyte_p, it) && !ASCII_CHAR_P (*p as isize)
	{ (*it).c = string_char_and_length (p, &raw mut (*it).len); }
      else
	{
	  (*it).c = *p as c_int;
	  (*it).len = 1;
	}

      /* Record what we have and where it came from.  */
      (*it).what = IT_CHARACTER;
      (*it).object = (*(*it).w).contents;
      (*it).position = (*it).current.pos;

      /* Normally we return the character found above, except when we
	 really want to return an ellipsis for selective display.  */
      if (*it).selective != 0
	{
	  if (*it).c == '\n' as c_int
	    {
	      /* A value of selective > 0 means hide lines indented more
		 than that number of columns.  */
	      if (*it).selective > 0
		  && IT_CHARPOS! (*it) + 1 < ZV! ()
		  && indented_beyond_p (IT_CHARPOS! (*it) + 1,
					IT_BYTEPOS! (*it) + 1,
					(*it).selective)
		{
		  success_p = next_element_from_ellipsis (it);
		  (*it).dpvec_char_len = -1;
		}
	    }
	  else if (*it).c == '\r' as c_int && (*it).selective == -1
	    {
	      /* A value of selective == -1 means that everything from the
		 CR to the end of the line is invisible, with maybe an
		 ellipsis displayed for it.  */
	      success_p = next_element_from_ellipsis (it);
	      (*it).dpvec_char_len = -1;
	    }
	}
    }

  /* Value is false if end of buffer reached.  */
  eassert! (!success_p || (*it).what != IT_CHARACTER || (*it).len > 0);
  return success_p;
}


/* Deliver a composition display element.  Unlike the other
   next_element_from_XXX, this function is not registered in the array
   get_next_element[].  It is called from next_element_from_buffer and
   next_element_from_string when necessary.  */

unsafe fn next_element_from_composition (it: *mut it) -> bool
{
  (*it).what = IT_COMPOSITION;
  (*it).len = (*it).cmp_it.nbytes;
  if STRINGP ((*it).string)
    {
      if (*it).c < 0
	{
	  IT_STRING_CHARPOS! (*it) += (*it).cmp_it.nchars as ptrdiff_t;
	  IT_STRING_BYTEPOS! (*it) += (*it).cmp_it.nbytes as ptrdiff_t;
	  return false;
	}
      (*it).position = (*it).current.string_pos;
      (*it).object = (*it).string;
      (*it).c = composition_update_it (&raw mut (*it).cmp_it as *mut c_void,
				       IT_STRING_CHARPOS! (*it),
				       IT_STRING_BYTEPOS! (*it), (*it).string);
    }
  else
    {
      if (*it).c < 0
	{
	  IT_CHARPOS! (*it) += (*it).cmp_it.nchars as ptrdiff_t;
	  IT_BYTEPOS! (*it) += (*it).cmp_it.nbytes as ptrdiff_t;
	  if it_bits! (bidi_p, it)
	    {
	      if bidi_it_bits! (new_paragraph, &raw mut (*it).bidi_it)
		{ bidi_paragraph_init ((*it).paragraph_embedding, &raw mut (*it).bidi_it,
				       false); }
	      /* Resync the bidi iterator with IT's new position.
		 FIXME: this doesn't support bidirectional text.  */
	      while (*it).bidi_it.charpos < IT_CHARPOS! (*it)
		{ bidi_move_to_visually_next (&raw mut (*it).bidi_it); }
	    }
	  return false;
	}
      (*it).position = (*it).current.pos;
      (*it).object = (*(*it).w).contents;
      (*it).c = composition_update_it (&raw mut (*it).cmp_it as *mut c_void,
				       IT_CHARPOS! (*it),
				       IT_BYTEPOS! (*it), Qnil ());
    }
  return true;
}

/***********************************************************************
	     Moving an iterator without producing glyphs
 ***********************************************************************/

/* Check if iterator is at a position corresponding to a valid buffer
   position after some move_it_ call.  */

macro_rules! IT_POS_VALID_AFTER_MOVE_P {
  ($it:expr) => { ((*$it).method != GET_FROM_STRING || IT_STRING_CHARPOS! (*$it) == 0) };
}


/* Move iterator IT to a specified buffer or X position within one
   line on the display without producing glyphs.

   OP should be a bit mask including some or all of these bits:
    MOVE_TO_X: Stop upon reaching x-position TO_X.
    MOVE_TO_POS: Stop upon reaching buffer or string position TO_CHARPOS.
   Regardless of OP's value, stop upon reaching the end of the display line.

   TO_X is normally a value 0 <= TO_X <= IT->last_visible_x.
   This means, in particular, that TO_X includes window's horizontal
   scroll amount.

   The return value has several possible values that
   say what condition caused the scan to stop:

   MOVE_POS_MATCH_OR_ZV
     - when TO_POS or ZV was reached.

   MOVE_X_REACHED
     -when TO_X was reached before TO_POS or ZV were reached.

   MOVE_LINE_CONTINUED
     - when we reached the end of the display area and the line must
     be continued.

   MOVE_LINE_TRUNCATED
     - when we reached the end of the display area and the line is
     truncated.

   MOVE_NEWLINE_OR_CR
     - when we stopped at a line end, i.e. a newline or a CR and selective
     display is on.  */

unsafe fn move_it_in_display_line_to (it: *mut it,
				       to_charpos: ptrdiff_t, to_x: c_int,
				       op: move_operation_enum) -> move_it_result
{
  let mut result: move_it_result = MOVE_UNDEFINED;
  let mut saved_glyph_row: *mut glyph_row;
  /* C leaves the four snapshots to `SAVE_IT`, and `closest_pos`/`saw_smaller_pos` are
     written below; Rust has no partial initialisation, so the port zero-initialises them
     (PORTING.md 1.5, deviation 1's rule).  `closest_pos`'s C initialiser is `UNINIT`, which
     the frozen configuration defines as `= {0,}`.  */
  let mut wrap_it: it = core::mem::zeroed ();
  let mut atpos_it: it = core::mem::zeroed ();
  let mut atx_it: it = core::mem::zeroed ();
  let mut ppos_it: it = core::mem::zeroed ();
  let mut wrap_data: *mut c_void = core::ptr::null_mut ();
  let mut atpos_data: *mut c_void = core::ptr::null_mut ();
  let mut atx_data: *mut c_void = core::ptr::null_mut ();
  let mut ppos_data: *mut c_void = core::ptr::null_mut ();
  let mut may_wrap: bool = false;
  let mut prev_method: it_method = (*it).method;
  let mut closest_pos: ptrdiff_t = 0;	/* UNINIT */
  let mut prev_pos: ptrdiff_t = IT_CHARPOS! (*it);
  let mut saw_smaller_pos: bool = prev_pos < to_charpos;
  let mut line_number_pending: bool = false;
  let mut this_line_subject_to_line_prefix: c_int = 0;
  let mut moved_to_next: bool = false;
  let mut saved_start_of_box_run: bool = it_bits! (start_of_box_run_p, it);

  /* C 10128-10132's `#ifdef GLYPH_DEBUG` block (the `atx_flag`/`atpos_flag`/`wrap_flag`
     triple, assigned in nine further dead blocks and never read) is dead on this target
     (P-08); recorded in docs/ports/xdisp.md.  */

  /* Don't produce glyphs in produce_glyphs.  */
  saved_glyph_row = (*it).glyph_row;
  (*it).glyph_row = core::ptr::null_mut ();

  /* Use wrap_it to save a copy of IT wherever a word wrap could
     occur.  Use atpos_it to save a copy of IT at the desired buffer
     position, if found, so that we can scan ahead and check if the
     word later overshoots the window edge.  Use atx_it similarly, for
     pixel positions.  */
  wrap_it.sp = -1;
  atpos_it.sp = -1;
  atx_it.sp = -1;

  /* Use ppos_it under bidi reordering to save a copy of IT for the
     initial position.  We restore that position in IT when we have
     scanned the entire display line without finding a match for
     TO_CHARPOS and all the character positions are greater than
     TO_CHARPOS.  We then restart the scan from the initial position,
     and stop at CLOSEST_POS, which is a position > TO_CHARPOS that is
     the closest to TO_CHARPOS.  */
  if it_bits! (bidi_p, it)
    {
      if (op & MOVE_TO_POS) != 0 && IT_CHARPOS! (*it) >= to_charpos
	{
	  SAVE_IT! (ppos_it, *it, ppos_data);
	  closest_pos = IT_CHARPOS! (*it);
	}
      else
	{ closest_pos = ZV! (); }
    }

  /* C defines this right here, before the loop; the port keeps a function-scoped
     `macro_rules!` in the same place (its body reads the enclosing `it`, `op` and
     `to_charpos`).  */
  macro_rules! BUFFER_POS_REACHED_P {
    () => {
      ((op & MOVE_TO_POS) != 0
       && BUFFERP ((*it).object)
       && (IT_CHARPOS! (*it) == to_charpos
	   || ((!it_bits! (bidi_p, it)
		|| BIDI_AT_BASE_LEVEL! (&raw mut (*it).bidi_it))
	       && IT_CHARPOS! (*it) > to_charpos)
	   || ((*it).what == IT_COMPOSITION
	       && ((IT_CHARPOS! (*it) > to_charpos
		    && to_charpos >= (*it).cmp_it.charpos)
		   || (IT_CHARPOS! (*it) < to_charpos
		       && to_charpos <= (*it).cmp_it.charpos))))
       && ((*it).method == GET_FROM_BUFFER
	   || ((*it).method == GET_FROM_DISPLAY_VECTOR
	       && (*it).dpvec.offset ((*it).current.dpvec_index as isize + 1)
		  >= (*it).dpend)))
    };
  }

  if (*it).hpos == 0
    {
      /* If line numbers are being displayed, produce a line number.
	 But don't do that if we are to reach first_visible_x, because
	 line numbers are not relevant to stuff that is not visible on
	 display.  */
      /* C 10187 writes `(op && MOVE_TO_X)` -- a *logical* AND of two non-zero tests, not
	 the bit test `op & MOVE_TO_X` every other site in this function uses.  With
	 `MOVE_TO_X` = 0x01 that operand's value is just `op != 0`; the port transcribes C's
	 truth value (Rust's `&&` takes `bool`, so the C spelling is not expressible).
	 Deviation recorded in docs/ports/xdisp.md.  */
      if !(op != 0 && MOVE_TO_X != 0 && to_x == (*it).first_visible_x)
	  && should_produce_line_number (it)
	{
	  if (*it).current_x == (*it).first_visible_x
	    { maybe_produce_line_number (it); }
	  else
	    { line_number_pending = true; }
	}
      /* If there's a line-/wrap-prefix, handle it, if we didn't already.  */
      if (*it).area == TEXT_AREA && !it_bits! (string_from_prefix_prop_p, it)
	{ handle_line_prefix (it); }

      /* Save whether this line has received a wrap prefix, as this
	 affects whether Emacs attempts to move glyphs into
	 continuation lines.  */
      this_line_subject_to_line_prefix = it_bits! (string_from_prefix_prop_p, it) as c_int;
    }

  if IT_CHARPOS! (*it) < CHARPOS! (this_line_min_pos)
    { SET_TEXT_POS! (this_line_min_pos, IT_CHARPOS! (*it), IT_BYTEPOS! (*it)); }

  saved_start_of_box_run = it_bits! (start_of_box_run_p, it);

  /* C's `goto done` (two sites inside the loop) skips the wrap/atpos/atx restore chain
     that follows it, so the port names that chain's block (PORTING.md 2.2).  */
  'done: {
  'scan: while true
    {
      let mut x: c_int;
      let mut i: c_int;
      let mut ascent: c_int = 0;
      let mut descent: c_int = 0;

      /* Utility macro to reset an iterator with x, ascent, and descent.  C defines it here,
	 inside the loop; the port hoists it to the top of the loop body because a Rust
	 `macro_rules!` is scoped to its block and the `buffer_pos_reached` body below (after
	 the labelled block) expands it too.  */
      macro_rules! IT_RESET_X_ASCENT_DESCENT {
	($IT:expr) => {
	  { (*$IT).current_x = x; (*$IT).max_ascent = ascent; (*$IT).max_descent = descent; }
	};
      }

      /* C's `buffer_pos_reached:` label is a leaf of the `if (it->nglyphs)` chain that
	 four forward and two backward `goto`s enter; the port wraps the loop body in this
	 labelled block, ends the block with `continue 'scan` (what C's fall-through does
	 anyway) and puts the label's body after it (PORTING.md 2.2).  */
      'buffer_pos_reached: {
      /* Stop if we move beyond TO_CHARPOS (after an image or a
	 display string or stretch glyph).  */
      if ((op & MOVE_TO_POS) != 0
	  && BUFFERP ((*it).object)
	  && (*it).method == GET_FROM_BUFFER
	  && (((!it_bits! (bidi_p, it)
		/* When the iterator is at base embedding level, we
		   are guaranteed that characters are delivered for
		   display in strictly increasing order of their
		   buffer positions.  */
		|| BIDI_AT_BASE_LEVEL! (&raw mut (*it).bidi_it))
	       && IT_CHARPOS! (*it) > to_charpos)
	      || (it_bits! (bidi_p, it)
		  && (prev_method == GET_FROM_IMAGE
		      || prev_method == GET_FROM_STRETCH
		      || prev_method == GET_FROM_STRING)
		  /* Passed TO_CHARPOS from left to right.  */
		  && ((prev_pos < to_charpos
		       && IT_CHARPOS! (*it) >= to_charpos)
		      /* Passed TO_CHARPOS from right to left.  */
		      || (prev_pos > to_charpos
			  && IT_CHARPOS! (*it) <= to_charpos)))))
	{
	  if (*it).line_wrap != WORD_WRAP || wrap_it.sp < 0
	    {
	      result = MOVE_POS_MATCH_OR_ZV;
	      break 'scan;
	    }
	  else if (*it).line_wrap == WORD_WRAP && atpos_it.sp < 0
	    {
	      /* If wrap_it is valid, the current position might be in
		 a word that is wrapped.  So, save the iterator in
		 atpos_it and continue to see if wrapping happens.  */
	      SAVE_IT! (atpos_it, *it, atpos_data);
	      /* C 10318-10320's `#ifdef GLYPH_DEBUG` block (`atpos_flag = …`) is dead.  */
	    }
	}

      /* Stop when ZV reached.
	 We used to stop here when TO_CHARPOS reached as well, but that is
	 too soon if this glyph does not fit on this line.  So we handle it
	 explicitly below.  */
      if !get_next_display_element (it)
	{
	  result = MOVE_POS_MATCH_OR_ZV;
	  break 'scan;
	}

      if (*it).line_wrap == TRUNCATE
	{
	  /* If it->pixel_width is zero, the last PRODUCE_GLYPHS call
	     produced something that doesn't consume any screen estate
	     in the text area, so we don't want to exit the loop at
	     TO_CHARPOS, before we produce the glyph for that buffer
	     position.  This happens, e.g., when there's an overlay at
	     TO_CHARPOS that draws a fringe bitmap.  */
	  if BUFFER_POS_REACHED_P! ()
	      && ((*it).pixel_width > 0
		  || IT_CHARPOS! (*it) > to_charpos
		  || (*it).area != TEXT_AREA)
	    {
	      result = MOVE_POS_MATCH_OR_ZV;
	      break 'scan;
	    }
	}
      else
	{
	  if (*it).line_wrap == WORD_WRAP && (*it).area == TEXT_AREA
	    {
	      let mut next_may_wrap: bool = may_wrap;
	      /* Can we wrap after this character?  */
	      if char_can_wrap_after (it)
		{ next_may_wrap = true; }
	      else
		{ next_may_wrap = false; }
	      /* Can we wrap here? */
	      if may_wrap && char_can_wrap_before (it)
		{
		  /* We have reached a glyph that follows one or more
		     whitespace characters or characters that allow
		     wrapping after them.  If this character allows
		     wrapping before it, save this position as a
		     wrapping point.  */
		  if atpos_it.sp >= 0
		    {
		      RESTORE_IT! (it, &raw mut atpos_it, atpos_data);
		      result = MOVE_POS_MATCH_OR_ZV;
		      break 'done;
		    }
		  if atx_it.sp >= 0
		    {
		      RESTORE_IT! (it, &raw mut atx_it, atx_data);
		      result = MOVE_X_REACHED;
		      break 'done;
		    }
		  /* Otherwise, we can wrap here.  */
		  SAVE_IT! (wrap_it, *it, wrap_data);
		  /* C 10404-10406's `#ifdef GLYPH_DEBUG` block (`wrap_flag = …`) is dead.  */
		}
	      /* Update may_wrap for the next iteration.  */
	      may_wrap = next_may_wrap;
	    }
	}

      /* Remember the line height for the current line, in case
	 the next element doesn't fit on the line.  */
      ascent = (*it).max_ascent;
      descent = (*it).max_descent;

      /* The call to produce_glyphs will get the metrics of the
	 display element IT is loaded with.  Record the x-position
	 before this display element, in case it doesn't fit on the
	 line.  */
      x = (*it).current_x;

      /* We will record the start_of_box_run_p flag to restore it before
	 exiting if we never move from this glyph.  */
      saved_start_of_box_run = it_bits! (start_of_box_run_p, it);
      PRODUCE_GLYPHS! (it);
      moved_to_next = false;

      if (*it).area != TEXT_AREA
	{
	  prev_method = (*it).method;
	  if (*it).method == GET_FROM_BUFFER
	    { prev_pos = IT_CHARPOS! (*it); }
	  set_iterator_to_next (it, true);
	  moved_to_next = true;
	  if IT_CHARPOS! (*it) < CHARPOS! (this_line_min_pos)
	    { SET_TEXT_POS! (this_line_min_pos,
			     IT_CHARPOS! (*it), IT_BYTEPOS! (*it)); }
	  if it_bits! (bidi_p, it)
	      && (op & MOVE_TO_POS) != 0
	      && IT_CHARPOS! (*it) > to_charpos
	      && IT_CHARPOS! (*it) < closest_pos
	    { closest_pos = IT_CHARPOS! (*it); }
	  continue 'scan;
	}

      /* The number of glyphs we get back in IT->nglyphs will normally
	 be 1 except when IT->c is (i) a TAB, or (ii) a multi-glyph
	 character on a terminal frame, or (iii) a line end.  For the
	 second case, IT->nglyphs - 1 padding glyphs will be present.
	 (On X frames, there is only one glyph produced for a
	 composite character.)

	 The behavior implemented below means, for continuation lines,
	 that as many spaces of a TAB as fit on the current line are
	 displayed there.  For terminal frames, as many glyphs of a
	 multi-glyph character are displayed in the current line, too.
	 This is what the old redisplay code did, and we keep it that
	 way.  Under X, the whole shape of a complex character must
	 fit on the line or it will be completely displayed in the
	 next line.

	 Note that both for tabs and padding glyphs, all glyphs have
	 the same width.  */
      if (*it).nglyphs != 0
	{
	  /* More than one glyph or glyph doesn't fit on line.  All
	     glyphs have the same width.  */
	  let single_glyph_width: c_int = (*it).pixel_width / (*it).nglyphs;
	  let mut new_x: c_int;
	  let x_before_this_char: c_int = x;
	  let hpos_before_this_char: c_int = (*it).hpos;

	  i = 0;
	  while i < (*it).nglyphs
	    {
	      new_x = x + single_glyph_width;

	      /* We want to leave anything reaching TO_X to the caller.  */
	      if (op & MOVE_TO_X) != 0 && new_x > to_x
		{
		  if BUFFER_POS_REACHED_P! ()
		    {
		      if (*it).line_wrap != WORD_WRAP || wrap_it.sp < 0
			{ break 'buffer_pos_reached; }
		      if atpos_it.sp < 0
			{
			  SAVE_IT! (atpos_it, *it, atpos_data);
			  IT_RESET_X_ASCENT_DESCENT! (&raw mut atpos_it);
			  /* C 10421-10423's `#ifdef GLYPH_DEBUG` block (`atpos_flag = …`) is
			     dead.  */
			}
		    }
		  else
		    {
		      if (*it).line_wrap != WORD_WRAP || wrap_it.sp < 0
			{
			  (*it).current_x = x;
			  result = MOVE_X_REACHED;
			  break;
			}
		      if atx_it.sp < 0
			{
			  SAVE_IT! (atx_it, *it, atx_data);
			  IT_RESET_X_ASCENT_DESCENT! (&raw mut atx_it);
			  /* C 10519-10521's `#ifdef GLYPH_DEBUG` block (`atx_flag = …`) is
			     dead.  */
			}
		    }
		}

	      if /* Lines are continued.  */
		 (*it).line_wrap != TRUNCATE
		 && (/* And glyph doesn't fit on the line.  */
		     new_x > (*it).last_visible_x
		     /* Or it fits exactly and we're on a window
			system frame.  */
		     || (new_x == (*it).last_visible_x
			 && FRAME_WINDOW_P ((*it).f)
			 && (if it_bits! (bidi_p, it)
				&& (*it).bidi_it.paragraph_dir == R2L
			     { WINDOW_LEFT_FRINGE_WIDTH! ((*it).w) }
			     else { WINDOW_RIGHT_FRINGE_WIDTH! ((*it).w) }) != 0))
		 /* There is no line prefix, next to which the
		    iterator _must_ produce a minimum of one actual
		    glyph.  */
		 && (!(this_line_subject_to_line_prefix != 0)
		     /* Or this is the second glyph to be produced
			beyond the confines of the line.  */
		     || (i != 0
			 && (x > (*it).last_visible_x
			     || (x == (*it).last_visible_x
				 && FRAME_WINDOW_P ((*it).f)
				 && (if it_bits! (bidi_p, it)
				     && (*it).bidi_it.paragraph_dir == R2L
				     { WINDOW_LEFT_FRINGE_WIDTH! ((*it).w) }
				     else { WINDOW_RIGHT_FRINGE_WIDTH! ((*it).w) }) != 0))))
		{
		  let mut moved_forward: bool = false;

		  if /* IT->hpos == 0 (modulo line-number width) means
			the very first glyph doesn't fit on the line,
			e.g., a wide image.  */
		     ((*it).hpos == 0 + if (*it).lnum_width != 0
				       { (*it).lnum_width + 2 } else { 0 }
		      || (new_x == (*it).last_visible_x
			  && FRAME_WINDOW_P ((*it).f)))
		    {
		      (*it).hpos += 1;
		      (*it).current_x = new_x;

		      /* The character's last glyph just barely fits
			 in this row.  */
		      if i == (*it).nglyphs - 1
			{
			  /* If this is the destination position,
			     return a position *before* it in this row,
			     now that we know it fits in this row.  */
			  if BUFFER_POS_REACHED_P! ()
			    {
			      let mut can_wrap: bool = true;

			      /* If the previous character says we can
				 wrap after it, but the current
				 character says we can't wrap before
				 it, then we can't wrap here.  */
			      if (*it).line_wrap == WORD_WRAP
				  && wrap_it.sp >= 0
				  && may_wrap
				  && IT_OVERFLOW_NEWLINE_INTO_FRINGE! (it)
				{
				  let mut tem_it: it = core::mem::zeroed ();
				  let mut tem_data: *mut c_void = core::ptr::null_mut ();

				  SAVE_IT! (tem_it, *it, tem_data);
				  set_iterator_to_next (it, true);
				  if get_next_display_element (it)
				      && !char_can_wrap_before (it)
				    { can_wrap = false; }
				  RESTORE_IT! (it, &raw mut tem_it, tem_data);
				}
			      if (*it).line_wrap != WORD_WRAP
				  || wrap_it.sp < 0
				  /* If we've just found whitespace
				     where we can wrap, effectively
				     ignore the previous wrap point --
				     it is no longer relevant, but we
				     won't have an opportunity to
				     update it, since we've reached
				     the edge of this screen line.  */
				  || (may_wrap && can_wrap
				      && IT_OVERFLOW_NEWLINE_INTO_FRINGE! (it))
				{
				  (*it).hpos = hpos_before_this_char;
				  (*it).current_x = x_before_this_char;
				  result = MOVE_POS_MATCH_OR_ZV;
				  break;
				}
			      if (*it).line_wrap == WORD_WRAP
				  && atpos_it.sp < 0
				{
				  SAVE_IT! (atpos_it, *it, atpos_data);
				  atpos_it.current_x = x_before_this_char;
				  atpos_it.hpos = hpos_before_this_char;
				  /* C 10631-10633's `#ifdef GLYPH_DEBUG` block
				     (`atpos_flag = …`) is dead.  */
				}
			    }

			  prev_method = (*it).method;
			  if (*it).method == GET_FROM_BUFFER
			    { prev_pos = IT_CHARPOS! (*it); }
			  set_iterator_to_next (it, true);
			  moved_to_next = true;
			  if IT_CHARPOS! (*it) < CHARPOS! (this_line_min_pos)
			    { SET_TEXT_POS! (this_line_min_pos,
					     IT_CHARPOS! (*it), IT_BYTEPOS! (*it)); }
			  /* On graphical terminals, newlines may
			     "overflow" into the fringe if
			     overflow-newline-into-fringe is non-nil.
			     On text terminals, and on graphical
			     terminals with no right margin, newlines
			     may overflow into the last glyph on the
			     display line.*/
			  if !FRAME_WINDOW_P ((*it).f)
			      || (if it_bits! (bidi_p, it)
				  && (*it).bidi_it.paragraph_dir == R2L
				  { WINDOW_LEFT_FRINGE_WIDTH! ((*it).w) }
				  else { WINDOW_RIGHT_FRINGE_WIDTH! ((*it).w) }) == 0
			      || IT_OVERFLOW_NEWLINE_INTO_FRINGE! (it)
			    {
			      if !get_next_display_element (it)
				{
				  result = MOVE_POS_MATCH_OR_ZV;
				  break;
				}
			      moved_forward = true;
			      if BUFFER_POS_REACHED_P! ()
				{
				  if ITERATOR_AT_END_OF_LINE_P! (it)
				    {
				      result = MOVE_POS_MATCH_OR_ZV;
				      break;
				    }
				  /* When word-wrapping IMAGES or
				     STRETCHES which just fit on a line,
				     do not return early, before the
				     wrap point can be restored */
				  else if !(prev_method == GET_FROM_STRETCH
					    || prev_method == GET_FROM_IMAGE)
					  || (*it).line_wrap != WORD_WRAP
				    {
				      result = MOVE_LINE_CONTINUED;
				      break;
				    }
				}
			      if ITERATOR_AT_END_OF_LINE_P! (it)
				  && ((*it).line_wrap != WORD_WRAP
				      || wrap_it.sp < 0
				      || IT_OVERFLOW_NEWLINE_INTO_FRINGE! (it))
				{
				  result = MOVE_NEWLINE_OR_CR;
				  break;
				}
			    }
			}
		    }
		  else
		    { IT_RESET_X_ASCENT_DESCENT! (it); }

		  /* If the screen line ends with whitespace (or
		     wrap-able character), and we are under word-wrap,
		     don't use wrap_it: it is no longer relevant, but
		     we won't have an opportunity to update it, since
		     we are done with this screen line.  */
		  if may_wrap && IT_OVERFLOW_NEWLINE_INTO_FRINGE! (it)
		      /* If the character after the one which set the
			 may_wrap flag says we can't wrap before it,
			 we can't wrap here.  Therefore, wrap_it
			 (previously found wrap-point) _is_ relevant
			 in that case.  */
		      && (!moved_forward || char_can_wrap_before (it))
		    {
		      /* If we've found TO_X, go back there, as we now
			 know the last word fits on this screen line.  */
		      if (op & MOVE_TO_X) != 0 && new_x == (*it).last_visible_x
			  && atx_it.sp >= 0
			{
			  RESTORE_IT! (it, &raw mut atx_it, atx_data);
			  atpos_it.sp = -1;
			  atx_it.sp = -1;
			  result = MOVE_X_REACHED;
			  break;
			}
		    }
		  else if wrap_it.sp >= 0
		    {
		      RESTORE_IT! (it, &raw mut wrap_it, wrap_data);
		      atpos_it.sp = -1;
		      atx_it.sp = -1;
		    }

		  move_trace (c"move_it_in: continued at %td\n".as_ptr (),
			      IT_CHARPOS! (*it));
		  result = MOVE_LINE_CONTINUED;
		  break;
		}

	      if BUFFER_POS_REACHED_P! ()
		{
		  if (*it).line_wrap != WORD_WRAP || wrap_it.sp < 0
		    { break 'buffer_pos_reached; }
		  if (*it).line_wrap == WORD_WRAP && atpos_it.sp < 0
		    {
		      SAVE_IT! (atpos_it, *it, atpos_data);
		      /* C 10706-10708's `#ifdef GLYPH_DEBUG`-style block is absent here; the
			 `atpos_flag` assignment C makes in this branch is inside its own dead
			 block.  */
		      IT_RESET_X_ASCENT_DESCENT! (&raw mut atpos_it);
		    }
		}

	      if new_x > (*it).first_visible_x
		{
		  /* If we have reached the visible portion of the
		     screen line, produce the line number if needed.  */
		  if line_number_pending
		    {
		      line_number_pending = false;
		      (*it).current_x = (*it).first_visible_x;
		      maybe_produce_line_number (it);
		      (*it).current_x += new_x - (*it).first_visible_x;
		    }
		  /* Glyph is visible.  Increment number of glyphs that
		     would be displayed.  */
		  (*it).hpos += 1;
		}
	      i += 1;
	      x = new_x;
	    }

	  /* C 10655-10656 is the tail of the `for` body (C 10389-10657), so its `break`
	     leaves the `for`, not the scan loop.  Every assignment to `result` inside that
	     body (C 10414, 10510, 10549, 10557, 10568, 10577, 10607) is immediately followed
	     by its own `break`, so the guard is never taken; the port keeps it inside the
	     `if (nglyphs)` arm, where breaking the scan loop is the same unreachable outcome.  */
	  if result != MOVE_UNDEFINED
	    { break 'scan; }
	}
      else if BUFFER_POS_REACHED_P! ()
	{
	  break 'buffer_pos_reached;
	}
      else if (op & MOVE_TO_X) != 0 && (*it).current_x >= to_x
	{
	  /* Stop when TO_X specified and reached.  This check is
	     necessary here because of lines consisting of a line end,
	     only.  The line end will not produce any glyphs and we
	     would never get MOVE_X_REACHED.  */
	  eassert! ((*it).nglyphs == 0);
	  result = MOVE_X_REACHED;
	  break 'scan;
	}

      /* Is this a line end?  If yes, we're done.  */
      if ITERATOR_AT_END_OF_LINE_P! (it)
	{
	  /* If we are past TO_CHARPOS, but never saw any character
	     positions smaller than TO_CHARPOS, return
	     MOVE_POS_MATCH_OR_ZV, like the unidirectional display
	     did.  */
	  if it_bits! (bidi_p, it) && (op & MOVE_TO_POS) != 0
	    {
	      if !saw_smaller_pos && IT_CHARPOS! (*it) > to_charpos
		{
		  if closest_pos < ZV! ()
		    {
		      RESTORE_IT! (it, &raw mut ppos_it, ppos_data);
		      /* Don't recurse if closest_pos is equal to
			 to_charpos, since we have just tried that.  */
		      if closest_pos != to_charpos
			{ move_it_in_display_line_to (it, closest_pos, -1, MOVE_TO_POS); }
		      result = MOVE_POS_MATCH_OR_ZV;
		    }
		  else
		    { break 'buffer_pos_reached; }
		}
	      else if (*it).line_wrap == WORD_WRAP && atpos_it.sp >= 0
		       && IT_CHARPOS! (*it) > to_charpos
		{ break 'buffer_pos_reached; }
	      else
		{ result = MOVE_NEWLINE_OR_CR; }
	    }
	  else
	    { result = MOVE_NEWLINE_OR_CR; }
	  /* If lines are truncated, and the line we moved across is
	     completely hscrolled out of view, reset the line metrics
	     to those of the newline we've just processed, so that
	     glyphs not on display don't affect the line's height.  */
	  if (*it).line_wrap == TRUNCATE
	      && (*it).current_x <= (*it).first_visible_x
	      && result == MOVE_NEWLINE_OR_CR
	      && (*it).char_to_display == '\n' as c_int
	    {
	      (*it).max_ascent = (*it).ascent;
	      (*it).max_descent = (*it).descent;
	    }
	  /* If we've processed the newline, make sure this flag is
	     reset, as it must only be set when the newline itself is
	     processed.  */
	  if result == MOVE_NEWLINE_OR_CR
	    { it_bits! (constrain_row_ascent_descent_p, it, false); }
	  break 'scan;
	}

      prev_method = (*it).method;
      if (*it).method == GET_FROM_BUFFER
	{ prev_pos = IT_CHARPOS! (*it); }

      /* The current display element has been consumed.  Advance to
	 the next.  */
      set_iterator_to_next (it, true);
      moved_to_next = true;

      /* If IT has just finished producing glyphs for the wrap prefix
	 and is proceeding to the next method, there might not be
	 sufficient space remaining in this line to accommodate its
	 glyphs, and one real glyph must be produced to prevent an
	 infinite loop.  Next, clear this flag if such a glyph has
	 already been produced.  */

      if this_line_subject_to_line_prefix == 1
	  && !it_bits! (string_from_prefix_prop_p, it)
	{ this_line_subject_to_line_prefix = 2; }
      else if this_line_subject_to_line_prefix == 2
	       && !it_bits! (string_from_prefix_prop_p, it)
	{ this_line_subject_to_line_prefix = 0; }

      if IT_CHARPOS! (*it) < CHARPOS! (this_line_min_pos)
	{ SET_TEXT_POS! (this_line_min_pos, IT_CHARPOS! (*it), IT_BYTEPOS! (*it)); }
      if IT_CHARPOS! (*it) < to_charpos
	{ saw_smaller_pos = true; }
      if it_bits! (bidi_p, it)
	  && (op & MOVE_TO_POS) != 0
	  && IT_CHARPOS! (*it) >= to_charpos
	  && IT_CHARPOS! (*it) < closest_pos
	{ closest_pos = IT_CHARPOS! (*it); }

      /* Stop if lines are truncated and IT's current x-position is
	 past the right edge of the window now.  */
      if (*it).line_wrap == TRUNCATE
	  && (*it).current_x >= (*it).last_visible_x
	{
	  if !FRAME_WINDOW_P ((*it).f)
	      || (if it_bits! (bidi_p, it) && (*it).bidi_it.paragraph_dir == R2L
		  { WINDOW_LEFT_FRINGE_WIDTH! ((*it).w) }
		  else { WINDOW_RIGHT_FRINGE_WIDTH! ((*it).w) }) == 0
	      || IT_OVERFLOW_NEWLINE_INTO_FRINGE! (it)
	    {
	      let mut at_eob_p: bool = false;

	      if { at_eob_p = !get_next_display_element (it); at_eob_p }
		  || BUFFER_POS_REACHED_P! ()
		  /* If we are past TO_CHARPOS, but never saw any
		     character positions smaller than TO_CHARPOS,
		     return MOVE_POS_MATCH_OR_ZV, like the
		     unidirectional display did.  */
		  || (it_bits! (bidi_p, it) && (op & MOVE_TO_POS) != 0
		      && !saw_smaller_pos
		      && IT_CHARPOS! (*it) > to_charpos)
		{
		  if it_bits! (bidi_p, it)
		      && !BUFFER_POS_REACHED_P! ()
		      && !at_eob_p && closest_pos < ZV! ()
		    {
		      RESTORE_IT! (it, &raw mut ppos_it, ppos_data);
		      if closest_pos != to_charpos
			{ move_it_in_display_line_to (it, closest_pos, -1, MOVE_TO_POS); }
		    }
		  result = MOVE_POS_MATCH_OR_ZV;
		  break 'scan;
		}
	      if ITERATOR_AT_END_OF_LINE_P! (it)
		{
		  result = MOVE_NEWLINE_OR_CR;
		  break 'scan;
		}
	    }
	  else if it_bits! (bidi_p, it) && (op & MOVE_TO_POS) != 0
		   && !saw_smaller_pos
		   && IT_CHARPOS! (*it) > to_charpos
	    {
	      if closest_pos < ZV! ()
		{
		  RESTORE_IT! (it, &raw mut ppos_it, ppos_data);
		  if closest_pos != to_charpos
		    { move_it_in_display_line_to (it, closest_pos, -1, MOVE_TO_POS); }
		}
	      result = MOVE_POS_MATCH_OR_ZV;
	      break 'scan;
	    }
	  result = MOVE_LINE_TRUNCATED;
	  break 'scan;
	}
      /* C 10819/10822's `#undef IT_RESET_X_ASCENT_DESCENT` / `#undef BUFFER_POS_REACHED_P`
	 have no Rust counterpart (the `macro_rules!` scopes end with their blocks).  */

      continue 'scan;
      }

      /* buffer_pos_reached: */
      IT_RESET_X_ASCENT_DESCENT! (it);
      result = MOVE_POS_MATCH_OR_ZV;
      break 'scan;
    }

  /* If we scanned beyond TO_POS, restore the saved iterator either to
     the wrap point (if found), or to atpos/atx location.  We decide which
     data to use to restore the saved iterator state by their X coordinates,
     since buffer positions might increase non-monotonically with screen
     coordinates due to bidi reordering.  */
  if result == MOVE_LINE_CONTINUED
      && (*it).line_wrap == WORD_WRAP
      && wrap_it.sp >= 0
      && ((atpos_it.sp >= 0 && wrap_it.current_x < atpos_it.current_x)
	  || (atx_it.sp >= 0 && wrap_it.current_x < atx_it.current_x))
    {
      /* C 10835-10837's `#ifdef GLYPH_DEBUG` block (`this_line_subject_to_line_prefix =
	 wrap_flag`) is dead.  */
      RESTORE_IT! (it, &raw mut wrap_it, wrap_data);
    }
  else if atpos_it.sp >= 0
    {
      /* C 10842-10844's `#ifdef GLYPH_DEBUG` block (`… = atpos_flag`) is dead.  */
      RESTORE_IT! (it, &raw mut atpos_it, atpos_data);
    }
  else if atx_it.sp >= 0
    {
      /* C 10849-10851's `#ifdef GLYPH_DEBUG` block (`… = atx_flag`) is dead.  */
      RESTORE_IT! (it, &raw mut atx_it, atx_data);
    }

  }

  /* done: */

  if !atpos_data.is_null ()
    { bidi_unshelve_cache (atpos_data, true); }
  if !atx_data.is_null ()
    { bidi_unshelve_cache (atx_data, true); }
  if !wrap_data.is_null ()
    { bidi_unshelve_cache (wrap_data, true); }
  if !ppos_data.is_null ()
    { bidi_unshelve_cache (ppos_data, true); }

  /* Restore the start_of_box_run_p flag, if we haven't moved from the
     position where it might have been set by get_next_display_element.
     This is so the following iteration picks up this flag instead of
     losing it (because PRODUCE_GLYPHS resets it).   */
  if !moved_to_next
    { it_bits! (start_of_box_run_p, it, saved_start_of_box_run); }

  /* Restore the iterator settings altered at the beginning of this
     function.  */
  (*it).glyph_row = saved_glyph_row;
  return result;
}
/* For external use.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn move_it_in_display_line (it: *mut it,
						   to_charpos: ptrdiff_t, to_x: c_int,
						   op: move_operation_enum)
{
  if (*it).line_wrap == WORD_WRAP
      && (op & MOVE_TO_X) != 0
    {
      let mut save_it: it = core::mem::zeroed ();
      let mut save_data: *mut c_void = core::ptr::null_mut ();
      let mut skip: c_int;

      SAVE_IT! (save_it, *it, save_data);
      skip = move_it_in_display_line_to (it, to_charpos, to_x, op);
      /* When word-wrap is on, TO_X may lie past the end
	 of a wrapped line.  Then it->current is the
	 character on the next line, so backtrack to the
	 space before the wrap point.  */
      if skip == MOVE_LINE_CONTINUED
	{
	  let prev_x: c_int = max ((*it).current_x as isize - 1, 0) as c_int;
	  RESTORE_IT! (it, &raw mut save_it, save_data);
	  move_it_in_display_line_to
	    (it, -1, prev_x, MOVE_TO_X);
	}
      else
	{ bidi_unshelve_cache (save_data, true); }
    }
  else
    { move_it_in_display_line_to (it, to_charpos, to_x, op); }
}

/* Move IT forward until it satisfies one or more of the criteria in
   TO_CHARPOS, TO_X, TO_Y, and TO_VPOS.

   OP is a bit-mask that specifies where to stop, and in particular,
   which of those four position arguments makes a difference.  See the
   description of enum move_operation_enum.

   If TO_CHARPOS is in invisible text, e.g. a truncated part of a
   screen line, this function will set IT to the next position that is
   displayed to the right of TO_CHARPOS on the screen.

   Return the maximum pixel length of any line scanned but never more
   than it.last_visible_x.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn move_it_to (it: *mut it, to_charpos: ptrdiff_t, to_x: c_int,
				     to_y: c_int, to_vpos: c_int, op: c_int) -> c_int
{
  let mut skip: move_it_result;
  let mut skip2: move_it_result = MOVE_X_REACHED;
  let mut line_height: c_int;
  let mut line_start_x: c_int = 0;
  let mut reached: c_int = 0;
  let mut max_current_x: c_int = 0;
  let mut backup_data: *mut c_void = core::ptr::null_mut ();

  /* C 11227's `out:` label is the first statement after the `for (;;)` body (C 11225 closes
     it), so C's `goto out` and its plain `break` land in the same place: the port writes
     `break` for both and no labelled block is needed (deviation 85).  */
  while true
    {
      if (op & MOVE_TO_VPOS) != 0
	{
	  /* If no TO_CHARPOS and no TO_X specified, stop at the
	     start of the line TO_VPOS.  */
	  if (op & (MOVE_TO_X | MOVE_TO_POS)) == 0
	    {
	      if (*it).vpos == to_vpos
		{
		  reached = 1;
		  break;
		}
	      else
		{ skip = move_it_in_display_line_to (it, -1, -1, 0); }
	    }
	  else
	    {
	      /* TO_VPOS >= 0 means stop at TO_X in the line at
		 TO_VPOS, or at TO_POS, whichever comes first.  */
	      if (*it).vpos == to_vpos
		{
		  reached = 2;
		  break;
		}

	      skip = move_it_in_display_line_to (it, to_charpos, to_x, op);

	      if skip == MOVE_POS_MATCH_OR_ZV || (*it).vpos == to_vpos
		{
		  reached = 3;
		  break;
		}
	      else if skip == MOVE_X_REACHED && (*it).vpos != to_vpos
		{
		  /* We have reached TO_X but not in the line we want.  */
		  skip = move_it_in_display_line_to (it, to_charpos,
						     -1, MOVE_TO_POS);
		  if skip == MOVE_POS_MATCH_OR_ZV
		    {
		      reached = 4;
		      break;
		    }
		}
	    }
	}
      else if (op & MOVE_TO_Y) != 0
	{
	  let mut it_backup: it = core::mem::zeroed ();

	  if (*it).line_wrap == WORD_WRAP
	    { SAVE_IT! (it_backup, *it, backup_data); }

	  /* TO_Y specified means stop at TO_X in the line containing
	     TO_Y---or at TO_CHARPOS if this is reached first.  The
	     problem is that we can't really tell whether the line
	     contains TO_Y before we have completely scanned it, and
	     this may skip past TO_X.  What we do is to first scan to
	     TO_X.

	     If TO_X is not specified, use a TO_X of zero.  The reason
	     is to make the outcome of this function more predictable.
	     If we didn't use TO_X == 0, we would stop at the end of
	     the line which is probably not what a caller would expect
	     to happen.  */
	  skip = move_it_in_display_line_to
	    (it, to_charpos, (if (op & MOVE_TO_X) != 0 { to_x } else { 0 }),
	     (MOVE_TO_X | (op & MOVE_TO_POS)));

	  /* If TO_CHARPOS is reached or ZV, we don't have to do more.  */
	  if skip == MOVE_POS_MATCH_OR_ZV
	    { reached = 5; }
	  else if skip == MOVE_X_REACHED
	    {
	      /* If TO_X was reached, we want to know whether TO_Y is
		 in the line.  We know this is the case if the already
		 scanned glyphs make the line tall enough.  Otherwise,
		 we must check by scanning the rest of the line.  */
	      line_height = (*it).max_ascent + (*it).max_descent;
	      if to_y >= (*it).current_y
		  && to_y < (*it).current_y + line_height
		{
		  reached = 6;
		  break;
		}
	      SAVE_IT! (it_backup, *it, backup_data);
	      move_trace (c"move_it: from %td\n".as_ptr (), IT_CHARPOS! (*it));
	      skip2 = move_it_in_display_line_to (it, to_charpos, -1,
						  op & MOVE_TO_POS);
	      move_trace (c"move_it: to %td\n".as_ptr (), IT_CHARPOS! (*it));
	      line_height = (*it).max_ascent + (*it).max_descent;
	      move_trace (c"move_it: line_height = %d\n".as_ptr (), line_height);

	      if to_y >= (*it).current_y
		  && to_y < (*it).current_y + line_height
		{
		  /* If TO_Y is in this line and TO_X was reached
		     above, we scanned too far.  We have to restore
		     IT's settings to the ones before skipping.  But
		     keep the more accurate values of max_ascent and
		     max_descent we've found while skipping the rest
		     of the line, for the sake of callers, such as
		     pos_visible_p, that need to know the line
		     height.  */
		  let max_ascent: c_int = (*it).max_ascent;
		  let max_descent: c_int = (*it).max_descent;

		  RESTORE_IT! (it, &raw mut it_backup, backup_data);
		  (*it).max_ascent = max_ascent;
		  (*it).max_descent = max_descent;
		  reached = 6;
		}
	      else
		{
		  skip = skip2;
		  if skip == MOVE_POS_MATCH_OR_ZV
		    {
		      reached = 7;
		      /* If the last move_it_in_display_line_to call
			 took us away from TO_CHARPOS, back up to the
			 previous position, as it is a better
			 approximation of TO_CHARPOS.  (Note that we
			 could have both positions after TO_CHARPOS or
			 both positions before it, due to bidi
			 reordering.)  */
		      if to_charpos > 0
			  && IT_CHARPOS! (*it) != to_charpos
			  && ((IT_CHARPOS! (it_backup) > to_charpos)
			      == (IT_CHARPOS! (*it) > to_charpos))
			{
			  let max_ascent: c_int = (*it).max_ascent;
			  let max_descent: c_int = (*it).max_descent;

			  RESTORE_IT! (it, &raw mut it_backup, backup_data);
			  (*it).max_ascent = max_ascent;
			  (*it).max_descent = max_descent;
			}
		    }
		}
	    }
	  else
	    {
	      /* Check whether TO_Y is in this line.  */
	      line_height = (*it).max_ascent + (*it).max_descent;
	      move_trace (c"move_it: line_height = %d\n".as_ptr (), line_height);

	      if to_y >= (*it).current_y
		  && to_y < (*it).current_y + line_height
		{
		  if to_y > (*it).current_y
		    { max_current_x = max ((*it).current_x as isize,
					   max_current_x as isize) as c_int; }

		  /* When word-wrap is on, TO_X may lie past the end
		     of a wrapped line.  Then it->current is the
		     character on the next line, so backtrack to the
		     space before the wrap point.  */
		  if skip == MOVE_LINE_CONTINUED
		      && (*it).line_wrap == WORD_WRAP
		    {
		      let prev_x: c_int = max ((*it).current_x as isize - 1, 0) as c_int;
		      RESTORE_IT! (it, &raw mut it_backup, backup_data);
		      skip = move_it_in_display_line_to
			(it, -1, prev_x, MOVE_TO_X);
		    }

		  reached = 6;
		}
	    }

	  if reached != 0
	    {
	      max_current_x = max ((*it).current_x as isize, max_current_x as isize) as c_int;
	      break;
	    }
	}
      else if BUFFERP ((*it).object)
	       && ((*it).method == GET_FROM_BUFFER
		   || (*it).method == GET_FROM_STRETCH)
	       && IT_CHARPOS! (*it) >= to_charpos
	       /* Under bidi iteration, a call to set_iterator_to_next
		  can scan far beyond to_charpos if the initial
		  portion of the next line needs to be reordered.  In
		  that case, give move_it_in_display_line_to another
		  chance below.  */
	       && !(it_bits! (bidi_p, it)
		    && (*it).bidi_it.scan_dir == -1)
	{ skip = MOVE_POS_MATCH_OR_ZV; }
      else
	{ skip = move_it_in_display_line_to (it, to_charpos, -1, MOVE_TO_POS); }

      match skip
	{
	  MOVE_POS_MATCH_OR_ZV =>
	    {
	      max_current_x = max ((*it).current_x as isize, max_current_x as isize) as c_int;
	      reached = 8;
	      break;
	    }

	  MOVE_NEWLINE_OR_CR =>
	    {
	      max_current_x = max ((*it).current_x as isize, max_current_x as isize) as c_int;
	      if !IT_OVERFLOW_NEWLINE_INTO_FRINGE! (it)
		{ (*it).override_ascent = -1; }
	      set_iterator_to_next (it, true);
	      (*it).continuation_lines_width = 0;
	    }

	  MOVE_LINE_TRUNCATED =>
	    {
	      max_current_x = (*it).last_visible_x;
	      (*it).continuation_lines_width = 0;
	      reseat_at_next_visible_line_start (it, false);
	      if (op & MOVE_TO_POS) != 0
		  && (IT_CHARPOS! (*it) > to_charpos
		      || (IT_CHARPOS! (*it) == to_charpos
			  /* Consider TO_CHARPOS as REACHED if we are at
			     EOB that ends in something other than a newline.  */
			  && to_charpos == ZV! ()
			  && (ZV_BYTE! () <= 1 || FETCH_BYTE! (ZV_BYTE! () - 1) != b'\n')
			  /* But if we have a display or an overlay string
			     at EOB, keep going until we exhaust all the
			     characters of the string(s).  */
			  && ((*it).sp == 0
			      || (STRINGP ((*it).string)
				  && ((*it).current.overlay_string_index < 0
				      || ((*it).current.overlay_string_index >= 0
					  && (*it).current.overlay_string_index
					     >= (*it).n_overlay_strings - 1))
				  && IT_STRING_CHARPOS! (*it) >= (*it).end_charpos))))
		{
		  reached = 9;
		  break;
		}
	    }

	  MOVE_LINE_CONTINUED =>
	    {
	      max_current_x = (*it).last_visible_x;
	      /* For continued lines ending in a tab, some of the glyphs
		 associated with the tab are displayed on the current
		 line.  Since it->current_x does not include these glyphs,
		 we use it->last_visible_x instead.  */
	      if (*it).c == '\t' as c_int
		{
		  (*it).continuation_lines_width += (*it).last_visible_x;
		  /* When moving by vpos, ensure that the iterator really
		     advances to the next line (bug#847, bug#969).  Fixme:
		     do we need to do this in other circumstances?  */
		  if (*it).current_x != (*it).last_visible_x
		      && (op & MOVE_TO_VPOS) != 0
		      && (op & (MOVE_TO_X | MOVE_TO_POS)) == 0
		    {
		      line_start_x = (*it).current_x + (*it).pixel_width
			- (*it).last_visible_x;
		      if FRAME_WINDOW_P ((*it).f)
			{
			  let face: *mut face = FACE_FROM_ID ((*it).f, (*it).face_id);
			  let face_font: *mut font = (*face).font;

			  /* When display_line produces a continued line
			     that ends in a TAB, it skips a tab stop that
			     is closer than the font's space character
			     width (see gui_produce_glyphs where it produces
			     the stretch glyph which represents a TAB).
			     We need to reproduce the same logic here.  */
			  eassert! (!face_font.is_null ());
			  if !face_font.is_null ()
			    {
			      if line_start_x < (*face_font).space_width
				{
				  line_start_x
				    += (*it).tab_width as c_int * (*face_font).space_width;
				}
			    }
			}
		      set_iterator_to_next (it, false);
		    }
		}
	      else
		{ (*it).continuation_lines_width += (*it).current_x; }
	    }

	  _ => { emacs_abort (); }
	}

      /* Reset/increment for the next run.  */
      (*it).current_x = line_start_x;
      (*it).wrap_prefix_width = 0;
      line_start_x = 0;
      (*it).hpos = 0;
      it_bits! (line_number_produced_p, it, false);
      (*it).current_y += (*it).max_ascent + (*it).max_descent;
      (*it).vpos += 1;
      last_height = (*it).max_ascent + (*it).max_descent;
      (*it).max_ascent = 0; (*it).max_descent = 0;
    }

  /* On text terminals, we may stop at the end of a line in the middle
     of a multi-character glyph.  If the glyph itself is continued,
     i.e. it is actually displayed on the next line, don't treat this
     stopping point as valid; move to the next line instead (unless
     that brings us offscreen).  */
  if !FRAME_WINDOW_P ((*it).f)
      && (op & MOVE_TO_POS) != 0
      && IT_CHARPOS! (*it) == to_charpos
      && (*it).what == IT_CHARACTER
      && (*it).nglyphs > 1
      && (*it).line_wrap == WINDOW_WRAP
      && (*it).current_x == (*it).last_visible_x - 1
      && (*it).c != '\n' as c_int
      && (*it).c != '\t' as c_int
      && window_bits! ((*it).w, window_end_valid)
      && (*it).vpos < (*(*it).w).window_end_vpos
    {
      (*it).continuation_lines_width += (*it).current_x;
      (*it).current_x = 0; (*it).hpos = 0; (*it).max_ascent = 0; (*it).max_descent = 0;
      (*it).wrap_prefix_width = 0;
      (*it).current_y += (*it).max_ascent + (*it).max_descent;
      (*it).vpos += 1;
      last_height = (*it).max_ascent + (*it).max_descent;
    }

  if !backup_data.is_null ()
    { bidi_unshelve_cache (backup_data, true); }

  move_trace (c"move_it_to: reached %d\n".as_ptr (), reached);

  return max_current_x;
}

/* Move iterator IT backward by a specified y-distance DY, DY >= 0.

   If DY > 0, move IT backward that many pixels.
   DY = 0 means move IT backward to the preceding line start or to BEGV.
   This function may move over less or more than DY pixels if
   IT->current_y - DY ends up in the middle of a line; in this case
   IT->current_y will be set to the top of the line either before or
   after the exact pixel coordinate.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn move_it_vertically_backward (it: *mut it, mut dy: c_int)
{
  let mut nlines: c_int;
  let mut h: c_int;
  let mut it2: it = core::mem::zeroed ();
  let mut it3: it = core::mem::zeroed ();
  let mut it2data: *mut c_void = core::ptr::null_mut ();
  let mut it3data: *mut c_void = core::ptr::null_mut ();
  let mut start_pos: ptrdiff_t;
  let nchars_per_row: c_int
    = ((*it).last_visible_x - (*it).first_visible_x) / FRAME_COLUMN_WIDTH ((*it).f);
  let mut pos_limit: ptrdiff_t;

  /* C 11283's `move_further_back:` label is jumped to once from inside the `else` arm
     (C 11416), so the port names the body `'move_further_back: loop` and writes
     `continue 'move_further_back` (PORTING.md 2.2, the `lookup_glyphless_char_display`
     `'retry` shape); the body's own end `break`s, which is where C's fall-through lands.  */
  'move_further_back: loop
    {
      eassert! (dy >= 0);

      start_pos = IT_CHARPOS! (*it);

      /* Estimate how many newlines we must move back.  */
      nlines = max (1, ((dy / default_line_pixel_height ((*it).w)) as isize)) as c_int;
      if (*it).line_wrap == TRUNCATE || nchars_per_row == 0
	{ pos_limit = BEGV! (); }
      else
	{
	  pos_limit = max ((start_pos - nlines as ptrdiff_t * nchars_per_row as ptrdiff_t)
			   as isize, BEGV! () as isize) as ptrdiff_t;
	}

      /* Set the iterator's position that many lines back.  But don't go
	 back more than NLINES full screen lines -- this wins a day with
	 buffers which have very long lines.  C writes `while (nlines-- && …)`, whose
	 post-decrement runs on every test including the one that ends the loop
	 (deviation 86), so the port spells the test out.  */
      loop
	{
	  let old: c_int = nlines;
	  nlines -= 1;
	  if old == 0 { break; }
	  if !(IT_CHARPOS! (*it) > pos_limit) { break; }
	  back_to_previous_visible_line_start (it);
	}

      /* Reseat the iterator here.  When moving backward, we don't want
	 reseat to skip forward over invisible text, set up the iterator
	 to deliver from overlay strings at the new position etc.  So,
	 use reseat_1 here.  */
      reseat_1 (it, (*it).current.pos, true);

      /* We are now surely at a line start.  */
      (*it).wrap_prefix_width = 0;
      (*it).current_x = 0; (*it).hpos = 0;	/* FIXME: this is incorrect when bidi
						   reordering is in effect.  */
      (*it).continuation_lines_width = 0;

      /* Move forward and see what y-distance we moved.  First move to the
	 start of the next line so that we get its height.  We need this
	 height to be able to tell whether we reached the specified
	 y-distance.  */
      SAVE_IT! (it2, *it, it2data);
      it2.max_ascent = 0; it2.max_descent = 0;
      let mut to_pos: ptrdiff_t = start_pos;
      loop
	{
	  move_it_to (&raw mut it2, to_pos, -1, -1, it2.vpos + 1,
		      (if to_pos > 0
			{ MOVE_TO_POS | MOVE_TO_VPOS }
		       else { MOVE_TO_VPOS }));
	  /* Avoid inflooping of there's a large display string with several
	     embedded newlines, which makes move_it_to stop after START_POS
	     but still inside a display or overlay string.  */
	  if IT_CHARPOS! (it2) >= start_pos
	    { to_pos = -1; }
	  /* C's `do … while (!(A || B))`: the port keeps C's negation and continues while
	     it holds.  */
	  if !(IT_POS_VALID_AFTER_MOVE_P! (&raw mut it2)
	       /* If we are in a display string which starts at START_POS,
		  and that display string includes a newline, and we are
		  right after that newline (i.e. at the beginning of a
		  display line), exit the loop, because otherwise we will
		  infloop, since move_it_to will see that it is already at
		  START_POS and will not move.  */
	       || (it2.method == GET_FROM_STRING
		   && IT_CHARPOS! (it2) == start_pos
		   && SREF (it2.string, IT_STRING_BYTEPOS! (it2) as usize - 1) == b'\n'))
	    { continue; }
	  break;
	}
      eassert! (IT_CHARPOS! (*it) >= BEGV! ());
      SAVE_IT! (it3, it2, it3data);

      move_it_to (&raw mut it2, start_pos, -1, -1, -1, MOVE_TO_POS);
      eassert! (IT_CHARPOS! (*it) >= BEGV! ());
      /* H is the actual vertical distance from the position in *IT
	 and the starting position.  */
      h = it2.current_y - (*it).current_y;
      /* NLINES is the distance in number of lines.  */
      nlines = it2.vpos - (*it).vpos;

      /* Correct IT's y and vpos position
	 so that they are relative to the starting point.  */
      (*it).vpos -= nlines;
      (*it).current_y -= h;

      if dy == 0
	{
	  /* DY == 0 means move to the start of the screen line.  The
	     value of nlines is > 0 if continuation lines were involved,
	     or if the original IT position was at start of a line.  */
	  RESTORE_IT! (it, it, it2data);
	  if nlines > 0
	    { move_it_by_lines (it, nlines as ptrdiff_t); }
	  /* The above code moves us to some position NLINES down,
	     usually to its first glyph (leftmost in an L2R line), but
	     that's not necessarily the start of the line, under bidi
	     reordering.  We want to get to the character position
	     that is immediately after the newline of the previous
	     line.  */
	  if it_bits! (bidi_p, it)
	      && (*it).continuation_lines_width == 0
	      && !STRINGP ((*it).string)
	      && IT_CHARPOS! (*it) > BEGV! ()
	      && FETCH_BYTE! (IT_BYTEPOS! (*it) - 1) != b'\n'
	    {
	      let mut cp: ptrdiff_t = IT_CHARPOS! (*it);
	      let mut bp: ptrdiff_t = IT_BYTEPOS! (*it);

	      dec_both (&raw mut cp, &raw mut bp);
	      SET_WITH_NARROWED_BEGV! (it, cp,
				      find_newline_no_quit (cp, bp, -1,
							    core::ptr::null_mut ()),
				      get_small_narrowing_begv ((*it).w, IT_CHARPOS! (*it)));
	      move_it_to (it, cp, -1, -1, -1, MOVE_TO_POS);
	    }
	  bidi_unshelve_cache (it3data, true);
	}
      else
	{
	  /* The y-position we try to reach, relative to *IT.
	     Note that H has been subtracted in front of the if-statement.  */
	  let target_y: c_int = (*it).current_y + h - dy;
	  let y0: c_int = it3.current_y;
	  let mut y1: c_int;
	  let mut line_height: c_int;

	  RESTORE_IT! (&raw mut it3, &raw mut it3, it3data);
	  last_height = 0;
	  y1 = line_bottom_y (&raw mut it3);
	  line_height = y1 - y0;
	  RESTORE_IT! (it, it, it2data);
	  /* If we did not reach target_y, try to move further backward if
	     we can.  If we moved too far backward, try to move forward.  */
	  if target_y < (*it).current_y
	      /* This is heuristic.  In a window that's 3 lines high, with
		 a line height of 13 pixels each, recentering with point
		 on the bottom line will try to move -39/2 = 19 pixels
		 backward.  Try to avoid moving into the first line.  */
	      && ((*it).current_y - target_y)
		 > min (window_box_height ((*it).w) as isize,
			(line_height * 2 / 3) as isize) as c_int
	      && IT_CHARPOS! (*it) > BEGV! ()
	    {
	      move_trace (c"  not far enough -> move_vert %d\n".as_ptr (),
			  target_y - (*it).current_y);
	      dy = (*it).current_y - target_y;
	      continue 'move_further_back;
	    }
	  else if target_y >= (*it).current_y + line_height
	       && IT_CHARPOS! (*it) < ZV! ()
	    {
	      /* Should move forward by at least one line, maybe more.

		 Note: Calling move_it_by_lines can be expensive on
		 terminal frames, where compute_motion is used (via
		 vmotion) to do the job, when there are very long lines
		 and truncate-lines is nil.  That's the reason for
		 treating terminal frames specially here.  */

	      if !FRAME_WINDOW_P ((*it).f)
		{ move_it_vertically (it, target_y - (*it).current_y); }
	      else
		{
		  let mut last_pos: text_pos = core::mem::zeroed ();
		  let mut last_y: c_int;
		  let mut last_vpos: c_int;
		  loop
		    {
		      last_pos = (*it).current.pos;
		      last_y = (*it).current_y;
		      last_vpos = (*it).vpos;
		      move_it_by_lines (it, 1);
		      /* C's `do … while (target_y > it->current_y && …)`: keep C's negation.  */
		      if !(target_y > (*it).current_y && IT_CHARPOS! (*it) < ZV! ())
			{ break; }
		    }
		  if (*it).current_y > target_y
		    {
		      reseat (it, last_pos, true);
		      (*it).current_y = last_y;
		      (*it).vpos = last_vpos;
		    }
		}
	    }
	}
      break;
    }
}

/* Move IT by a specified amount of pixel lines DY.  DY negative means
   move backwards.  DY = 0 means move to start of screen line.  At the
   end, IT will be on the start of a screen line.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn move_it_vertically (it: *mut it, dy: c_int)
{
  if dy <= 0
    { move_it_vertically_backward (it, -dy); }
  else
    {
      move_trace (c"move_it_v: from %td, %d\n".as_ptr (), IT_CHARPOS! (*it), dy);
      move_it_to (it, ZV! (), -1, (*it).current_y + dy, -1,
		  MOVE_TO_POS | MOVE_TO_Y);
      move_trace (c"move_it_v: to %td\n".as_ptr (), IT_CHARPOS! (*it));

      /* If buffer ends in ZV without a newline, move to the start of
	 the line to satisfy the post-condition.  */
      if IT_CHARPOS! (*it) == ZV! ()
	  && ZV! () > BEGV! ()
	  && FETCH_BYTE! (IT_BYTEPOS! (*it) - 1) != b'\n'
	{ move_it_by_lines (it, 0); }
    }
}

/* Move iterator IT past the end of the text line it is in.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn move_it_past_eol (it: *mut it)
{
  let mut rc: move_it_result;

  rc = move_it_in_display_line_to (it, Z! (), 0, MOVE_TO_POS);
  if rc == MOVE_NEWLINE_OR_CR
    { set_iterator_to_next (it, false); }
}

/* Move IT by a specified number DVPOS of screen lines down.  DVPOS
   negative means move up.  DVPOS == 0 means move to the start of the
   screen line.

   Optimization idea: If we would know that IT->f doesn't use
   a face with proportional font, we could be faster for
   truncate-lines nil.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn move_it_by_lines (it: *mut it, mut dvpos: ptrdiff_t)
{

  /* The commented-out optimization uses vmotion on terminals.  This
     gives bad results, because elements like it->what, on which
     callers such as pos_visible_p rely, aren't updated.  */
  /* struct position pos;
    if (!FRAME_WINDOW_P (it->f))
    {
      struct text_pos textpos;

      pos = *vmotion (IT_CHARPOS (*it), dvpos, it->w);
      SET_TEXT_POS (textpos, pos.bufpos, pos.bytepos);
      reseat (it, textpos, true);
      it->vpos += pos.vpos;
      it->current_y += pos.vpos;
    }
    else */

  if dvpos == 0
    {
      /* DVPOS == 0 means move to the start of the screen line.  */
      move_it_vertically_backward (it, 0);
      /* Let next call to line_bottom_y calculate real line height.  */
      last_height = 0;
    }
  else if dvpos > 0
    {
      move_it_to (it, -1, -1, -1, ((*it).vpos as ptrdiff_t + dvpos) as c_int,
		  MOVE_TO_VPOS);
      if !IT_POS_VALID_AFTER_MOVE_P! (it)
	{
	  /* Only move to the next buffer position if we ended up in a
	     string from display property, not in an overlay string
	     (before-string or after-string).  That is because the
	     latter don't conceal the underlying buffer position, so
	     we can ask to move the iterator to the exact position we
	     are interested in.  Note that, even if we are already at
	     IT_CHARPOS (*it), the call below is not a no-op, as it
	     will detect that we are at the end of the string, pop the
	     iterator, and compute it->current_x and it->hpos
	     correctly.  */
	  move_it_to (it, IT_CHARPOS! (*it)
			  + it_bits! (string_from_display_prop_p, it) as ptrdiff_t,
		      -1, -1, -1, MOVE_TO_POS);
	}
    }
  else
    {
      let mut it2: it = core::mem::zeroed ();
      let mut it2data: *mut c_void = core::ptr::null_mut ();
      let mut start_charpos: ptrdiff_t;
      let mut orig_charpos: ptrdiff_t;
      let mut i: ptrdiff_t;
      let nchars_per_row: c_int
	= ((*it).last_visible_x - (*it).first_visible_x) / FRAME_COLUMN_WIDTH ((*it).f);
      let mut hit_pos_limit: bool = false;
      let mut pos_limit: ptrdiff_t;

      /* Start at the beginning of the screen line containing IT's
	 position.  This may actually move vertically backwards,
	 in case of overlays, so adjust dvpos accordingly.  */
      dvpos += (*it).vpos as ptrdiff_t;
      orig_charpos = IT_CHARPOS! (*it);
      move_it_vertically_backward (it, 0);
      dvpos -= (*it).vpos as ptrdiff_t;

      /* Go back -DVPOS buffer lines, but no farther than -DVPOS full
	 screen lines, and reseat the iterator there.  */
      start_charpos = IT_CHARPOS! (*it);
      if (*it).line_wrap == TRUNCATE || nchars_per_row == 0
	{ pos_limit = BEGV! (); }
      else
	{
	  pos_limit = max ((start_charpos + dvpos * nchars_per_row as ptrdiff_t) as isize,
			   BEGV! () as isize) as ptrdiff_t;
	}

      i = -dvpos;
      while i > 0 && IT_CHARPOS! (*it) > pos_limit
	{
	  back_to_previous_visible_line_start (it);
	  i -= 1;
	}
      if i > 0 && IT_CHARPOS! (*it) <= pos_limit
	{ hit_pos_limit = true; }
      reseat (it, (*it).current.pos, true);

      /* Move further back if we end up in a string or an image.  */
      while !IT_POS_VALID_AFTER_MOVE_P! (it)
	{
	  /* First try to move to start of display line.  */
	  dvpos += (*it).vpos as ptrdiff_t;
	  move_it_vertically_backward (it, 0);
	  dvpos -= (*it).vpos as ptrdiff_t;
	  if IT_POS_VALID_AFTER_MOVE_P! (it)
	    { break; }
	  /* If start of line is still in string or image,
	     move further back.  */
	  back_to_previous_visible_line_start (it);
	  reseat (it, (*it).current.pos, true);
	  dvpos -= 1;
	}

      (*it).current_x = 0; (*it).hpos = 0; (*it).wrap_prefix_width = 0;

      /* Above call may have moved too far if continuation lines
	 are involved.  Scan forward and see if it did.  */
      SAVE_IT! (it2, *it, it2data);
      it2.vpos = 0; it2.current_y = 0;
      move_it_to (&raw mut it2, start_charpos, -1, -1, -1, MOVE_TO_POS);
      (*it).vpos -= it2.vpos;
      (*it).current_y -= it2.current_y;
      (*it).current_x = 0; (*it).hpos = 0; (*it).wrap_prefix_width = 0;

      /* If we moved too far back, move IT some lines forward.  */
      if (it2.vpos as ptrdiff_t) > -dvpos
	{
	  let delta: c_int = (it2.vpos as ptrdiff_t + dvpos) as c_int;

	  RESTORE_IT! (&raw mut it2, &raw mut it2, it2data);
	  SAVE_IT! (it2, *it, it2data);
	  move_it_to (it, -1, -1, -1, (*it).vpos + delta, MOVE_TO_VPOS);
	  /* Move back again if we got too far ahead,
	     or didn't move at all.  */
	  if (*it).vpos as ptrdiff_t - it2.vpos as ptrdiff_t > delta as ptrdiff_t
	      || IT_CHARPOS! (*it) == orig_charpos
	    { RESTORE_IT! (it, &raw mut it2, it2data); }
	  else
	    { bidi_unshelve_cache (it2data, true); }
	}
      else if hit_pos_limit && pos_limit > BEGV! ()
	       && dvpos < 0 && (it2.vpos as ptrdiff_t) < -dvpos
	{
	  /* If we hit the limit, but still didn't make it far enough
	     back, that means there's a display string with a newline
	     covering a large chunk of text, and that caused
	     back_to_previous_visible_line_start try to go too far.
	     Punish those who commit such atrocities by going back
	     until we've reached DVPOS, after lifting the limit, which
	     could make it slow for very long lines.  "If it hurts,
	     don't do that!"  */
	  dvpos += it2.vpos as ptrdiff_t;
	  RESTORE_IT! (it, it, it2data);
	  i = -dvpos;
	  while i > 0
	    {
	      back_to_previous_visible_line_start (it);
	      (*it).vpos -= 1;
	      i -= 1;
	    }
	  reseat_1 (it, (*it).current.pos, true);
	}
      else
	{ RESTORE_IT! (it, it, it2data); }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn partial_line_height (it_origin: *mut it) -> c_int
{
  /* In a buffer with very long and truncated lines, we ignore the
     possibly-partial height of the last line in the window: it is too
     expensive to compute that (since in most cases that involves
     going all the way to ZV), and the effect of ignoring it is
     relatively minor.  */
  if buffer_bits! (long_line_optimizations_p, XBUFFER ((*(*it_origin).w).contents))
      && (*it_origin).line_wrap == TRUNCATE
    { return 0; }

  let mut partial_height: c_int;
  let mut it_data: *mut c_void = core::ptr::null_mut ();
  let mut it: it = core::mem::zeroed ();
  SAVE_IT! (it, *it_origin, it_data);
  move_it_to (&raw mut it, ZV! (), -1, it.last_visible_y, -1,
	      MOVE_TO_POS | MOVE_TO_Y);
  if it.what == IT_EOB
    {
      let vis_height: c_int = it.last_visible_y - it.current_y;
      let height: c_int = it.ascent + it.descent;
      partial_height = if vis_height < height { vis_height } else { 0 };
    }
  else
    {
      let last_line_y: c_int = it.current_y;
      move_it_by_lines (&raw mut it, 1);
      partial_height = if it.current_y > it.last_visible_y
	{ it.last_visible_y - last_line_y } else { 0 };
    }
  RESTORE_IT! (&raw mut it, &raw mut it, it_data);
  return partial_height;
}

/* Approximate move_it_in_display_line_to for very long and truncated
   display lines, when moving horizontally.  This is used when the
   buffer's long_line_optimizations_p flag is set.  It ignores various
   complications, like different font sizes, invisible text, display
   and overlay strings, and, to some degree, bidirectional text.  So
   caveat emptor!

   Starting from IT's position, reseat IT after skipping NCHARS
   characters or to the next newline/ZV, whichever comes first.  Return
   what move_it_in_display_line_to would have returned in this case.  */

unsafe fn fast_move_it_horizontally (it: *mut it, nchars: ptrdiff_t) -> move_it_result
{
  let mut nl_bytepos: ptrdiff_t = 0;
  let mut nl_pos: ptrdiff_t = find_newline_no_quit (IT_CHARPOS! (*it), IT_BYTEPOS! (*it),
						1, &raw mut nl_bytepos);
  let mut new_pos: text_pos = core::mem::zeroed ();
  let move_result: move_it_result;

  if nl_pos - IT_CHARPOS! (*it) > nchars
    {
      SET_TEXT_POS! (new_pos,
		     IT_CHARPOS! (*it) + nchars,
		     CHAR_TO_BYTE (IT_CHARPOS! (*it) + nchars));
      move_result = MOVE_X_REACHED;
    }
  else
    {
      if nl_bytepos < ZV_BYTE! ()
	  || (nl_bytepos > BEGV_BYTE! ()
	      && FETCH_BYTE! (nl_bytepos - 1) == b'\n')
	{
	  nl_pos -= 1;
	  nl_bytepos -= 1;
	  move_result = MOVE_NEWLINE_OR_CR;
	}
      else
	{ move_result = MOVE_POS_MATCH_OR_ZV; }
      SET_TEXT_POS! (new_pos, nl_pos, nl_bytepos);
    }
  reseat (it, new_pos, false);
  return move_result;
}

/* Return true if IT points into the middle of a display vector.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn in_display_vector_p (it: *mut it) -> bool
{
  return ((*it).method == GET_FROM_DISPLAY_VECTOR
	  && (*it).current.dpvec_index > 0
	  && (*it).dpvec.offset ((*it).current.dpvec_index as isize) != (*it).dpend);
}

/* This is like Fwindow_text_pixel_size but assumes that WINDOW's buffer
   is the current buffer.  Fbuffer_text_pixel_size calls it after it has
   set WINDOW's buffer to the buffer specified by its BUFFER_OR_NAME
   argument.  */
unsafe fn window_text_pixel_size (window: Lisp_Object, from: Lisp_Object, to: Lisp_Object,
				  x_limit: Lisp_Object, y_limit: Lisp_Object,
				  mode_lines: Lisp_Object, ignore_line_at_end: Lisp_Object)
				  -> Lisp_Object
{
  let w: *mut window = decode_live_window (window);
  let mut it: it = core::mem::zeroed ();
  let mut start: ptrdiff_t;
  let mut end: ptrdiff_t;
  let mut bpos: ptrdiff_t;
  let mut startp: text_pos = core::mem::zeroed ();
  let mut itdata: *mut c_void = core::ptr::null_mut ();
  let mut c: c_int;
  let mut max_x: c_int = 0;
  let mut max_y: c_int = 0;
  let mut x: c_int = 0;
  let mut y: c_int = 0;
  let mut vertical_offset: c_int = 0;
  let mut doff: c_int = 0;

  if NILP (from)
    {
      start = BEGV! ();
      bpos = BEGV_BYTE! ();
    }
  else if EQ (from, Qt ())
    {
      start = BEGV! ();
      bpos = BEGV_BYTE! ();
      while bpos < ZV_BYTE! ()
	{
	  c = FETCH_BYTE! (bpos) as c_int;
	  if !(c == ' ' as c_int || c == '\t' as c_int || c == '\n' as c_int
	       || c == '\r' as c_int)
	    { break; }
	  inc_both (&raw mut start, &raw mut bpos);
	}
      while bpos > BEGV_BYTE! ()
	{
	  dec_both (&raw mut start, &raw mut bpos);
	  c = FETCH_BYTE! (bpos) as c_int;
	  if !(c == ' ' as c_int || c == '\t' as c_int)
	    { break; }
	}
    }
  else if CONSP (from)
    {
      start = clip_to_bounds (BEGV! (), fix_position (XCAR (from)), ZV! ());
      bpos = CHAR_TO_BYTE (start);
      CHECK_FIXNUM (XCDR (from));
      vertical_offset = XFIXNUM (XCDR (from)) as c_int;
    }
  else
    {
      start = clip_to_bounds (BEGV! (), fix_position (from), ZV! ());
      bpos = CHAR_TO_BYTE (start);
    }

  SET_TEXT_POS! (startp, start, bpos);

  if NILP (to)
    { end = ZV! (); }
  else if EQ (to, Qt ())
    {
      end = ZV! ();
      bpos = ZV_BYTE! ();
      while bpos > BEGV_BYTE! ()
	{
	  dec_both (&raw mut end, &raw mut bpos);
	  c = FETCH_BYTE! (bpos) as c_int;
	  if !(c == ' ' as c_int || c == '\t' as c_int || c == '\n' as c_int
	       || c == '\r' as c_int)
	    {
	      inc_both (&raw mut end, &raw mut bpos);
	      break;
	    }
	}
      while bpos < ZV_BYTE! ()
	{
	  c = fetch_char_advance (&raw mut end, &raw mut bpos);
	  if !(c == ' ' as c_int || c == '\t' as c_int)
	    { break; }
	}
    }
  else
    { end = clip_to_bounds (start, fix_position (to), ZV! ()); }

  if RANGED_FIXNUMP (0, x_limit, INT_MAX as EMACS_INT)
    { max_x = XFIXNUM (x_limit) as c_int; }
  else if !NILP (x_limit)
    { max_x = INT_MAX; }

  if NILP (y_limit)
    { max_y = INT_MAX; }
  else if RANGED_FIXNUMP (0, y_limit, INT_MAX as EMACS_INT)
    { max_y = XFIXNUM (y_limit) as c_int; }

  itdata = bidi_shelve_cache ();

  start_display (&raw mut it, w, startp);

  let mut start_y: c_int = it.current_y;

  /* It makes no sense to measure dimensions of region of text that
     crosses the point where bidi reordering changes scan direction.
     By using unidirectional movement here we at least support the use
     case of measuring regions of text that have a uniformly R2L
     directionality, and regions that begin and end in text of the
     same directionality.  */
  it_bits! (bidi_p, &raw mut it, false);

  let mut start_x: c_int;
  let mut start_bpos: ptrdiff_t = BYTEPOS! (startp);
  if vertical_offset != 0
    {
      let mut last_y: c_int;
      it.current_y = 0;

      move_it_by_lines (&raw mut it, 0);

      /* `move_it_vertically_backward' is called by move_it_vertically
	 to move by a negative value (upwards), but it is not always
	 guaranteed to leave the iterator at or above the position
	 given by the offset, which this loop ensures.  */
      if vertical_offset < 0
	{
	  while it.current_y > vertical_offset
	    {
	      last_y = it.current_y;
	      move_it_vertically_backward (&raw mut it,
					   ((vertical_offset as i32).abs ()
					    + it.current_y));

	      if it.current_y == last_y
		{ break; }
	    }
	}
      else
	{
	  move_it_vertically (&raw mut it, vertical_offset);
	}

      it.current_y = (WINDOW_TAB_LINE_HEIGHT! (w)
		      + WINDOW_HEADER_LINE_HEIGHT! (w));
      start = clip_to_bounds (BEGV! (), IT_CHARPOS! (it), ZV! ());
      start_bpos = CHAR_TO_BYTE (start);
      start_y = it.current_y;
      start_x = it.current_x;
    }
  else
    {
      /* Start at the beginning of the line containing FROM.  Otherwise
	 IT.current_x will be incorrectly set to zero at some arbitrary
	 non-zero X coordinate.  */
      move_it_by_lines (&raw mut it, 0);
      it.current_x = 0; it.hpos = 0; it.wrap_prefix_width = 0;
      if IT_CHARPOS! (it) != start
	{
	  let mut it1data: *mut c_void = core::ptr::null_mut ();
	  let mut it1: it = core::mem::zeroed ();

	  SAVE_IT! (it1, it, it1data);
	  move_it_to (&raw mut it, start, -1, -1, -1, MOVE_TO_POS);
	  /* We could have a display property at START, in which case
	     asking move_it_to to stop at START will overshoot and
	     stop at position after START.  So we try again, stopping
	     before START, and account for the width of the last
	     buffer position manually.  */
	  if IT_CHARPOS! (it) > start && start > BEGV! ()
	    {
	      let it1pos: ptrdiff_t = IT_CHARPOS! (it1);
	      let it1_x: c_int = it1.current_x;

	      RESTORE_IT! (&raw mut it, &raw mut it1, it1data);
	      /* If START - 1 is the beginning of screen line,
		 move_it_to will not move, so we need to use a
		 lower-level move_it_in_display_line subroutine, and
		 tell it to move just 1 pixel, so it stops at the next
		 display element.  */
	      if start - 1 > it1pos
		{ move_it_to (&raw mut it, start - 1, -1, -1, -1, MOVE_TO_POS); }
	      else
		{
		  move_it_in_display_line (&raw mut it, start, it1_x + 1,
					   MOVE_TO_POS | MOVE_TO_X);
		}
	      start_x = it.current_x;
	      /* If we didn't change our buffer position, the pixel
		 width of what's here was not yet accounted for; do it
		 manually.  */
	      if IT_CHARPOS! (it) == start - 1
		{ start_x += it.pixel_width; }
	    }
	  else
	    {
	      start_x = it.current_x;
	      bidi_unshelve_cache (it1data, true);
	    }
	}
      else
	{ start_x = it.current_x; }
    }

  /* Now move to TO.  */
  let mut move_op: c_int = MOVE_TO_POS | MOVE_TO_Y;
  let mut to_x: c_int = -1;
  it.current_y = start_y;
  /* If FROM is on a newline, pretend that we start at the beginning
     of the next line, because the newline takes no place on display.  */
  if FETCH_BYTE! (start_bpos) == b'\n'
    {
      it.current_x = 0;
      start_x = 0;	/* in case TO == FROM */
      it.wrap_prefix_width = 0;
    }
  if !NILP (x_limit)
    {
      it.last_visible_x = max_x;
      /* Actually, we never want move_it_to stop at to_x.  But to make
	 sure that move_it_in_display_line_to always moves far enough,
	 we set to_x to INT_MAX and specify MOVE_TO_X.  */
      move_op |= MOVE_TO_X;
      to_x = INT_MAX;
    }

  let mut it2data: *mut c_void = core::ptr::null_mut ();
  let mut it2: it = core::mem::zeroed ();
  SAVE_IT! (it2, it, it2data);

  x = move_it_to (&raw mut it, end, to_x, max_y, -1, move_op);

  /* We could have a display property at END, in which case asking
     move_it_to to stop at END will overshoot and stop at position
     after END.  So we try again, stopping before END, and account for
     the width of the last buffer position manually.  */
  if IT_CHARPOS! (it) > end
    {
      let end_y: c_int = it.current_y;

      end -= 1;
      RESTORE_IT! (&raw mut it, &raw mut it2, it2data);
      x = move_it_to (&raw mut it, end, to_x, max_y, -1, move_op);
      /* Add the width of the thing at TO, but only if we didn't
	 overshoot it; if we did, it is already accounted for.  Also,
	 account for the height of the thing at TO.  */
      if IT_CHARPOS! (it) == end
	{
	  x += it.pixel_width;

	  /* DTRT if ignore_line_at_end is t.  */
	  if !NILP (ignore_line_at_end)
	    {
	      /* If END-1 is on the previous screen line, we need to
		 account for the vertical dimensions of previous line.  */
	      if it.current_y < end_y
		{
		  doff = (max (it.max_ascent as isize, it.ascent as isize) as c_int
			  + max (it.max_descent as isize, it.descent as isize) as c_int);
		}
	    }
	  else
	    {
	      it.max_ascent = max (it.max_ascent as isize, it.ascent as isize) as c_int;
	      it.max_descent = max (it.max_descent as isize, it.descent as isize) as c_int;
	    }
	}
      else if IT_CHARPOS! (it) > end
	       && it.line_wrap == TRUNCATE
	       && it.current_x - it.first_visible_x >= it.last_visible_x
	{
	  /* If the display property at END is at the beginning of the
	     line, and the previous line was truncated, we are at END,
	     but it.current_y is not yet updated to reflect that.  */
	  it.current_y += max (it.max_ascent as isize, it.ascent as isize) as c_int
			  + max (it.max_descent as isize, it.descent as isize) as c_int;
	}
    }
  else
    { bidi_unshelve_cache (it2data, true); }

  if !NILP (x_limit)
    {
      /* Don't return more than X-LIMIT.  */
      if x > max_x
	{ x = max_x; }
    }

  /* If text spans more than one screen line, we don't need to adjust
     the x-span for start_x, since the second and subsequent lines
     will begin at zero X coordinate.  */
  if it.current_y > start_y
    { start_x = 0; }

  /* Subtract height of header-line and tab-line which was counted
     automatically by start_display.  */
  if !NILP (ignore_line_at_end)
    {
      y = (it.current_y + doff
	   - WINDOW_TAB_LINE_HEIGHT! (w)
	   - WINDOW_HEADER_LINE_HEIGHT! (w));
    }
  else
    {
      y = (it.current_y + it.max_ascent + it.max_descent + doff
	   - WINDOW_TAB_LINE_HEIGHT! (w) - WINDOW_HEADER_LINE_HEIGHT! (w));
    }

  /* Don't return more than Y-LIMIT.  */
  if y > max_y
    { y = max_y; }

  if (EQ (mode_lines, Qtab_line ()) || EQ (mode_lines, Qt ()))
      && window_wants_tab_line (w)
    /* Add height of tab-line as requested.  */
    {
      let window_tab_line_format: Lisp_Object
	= window_parameter (w, Qtab_line_format ());

      y = y + display_mode_line (w, CURRENT_TAB_LINE_ACTIVE_FACE_ID! (w),
				 if NILP (window_tab_line_format)
				 { BVAR! (current_buffer, tab_line_format_) }
				 else { window_tab_line_format });
    }

  if (EQ (mode_lines, Qheader_line ()) || EQ (mode_lines, Qt ()))
      && window_wants_header_line (w)
    {
      let window_header_line_format: Lisp_Object
	= window_parameter (w, Qheader_line_format ());

      y = y + display_mode_line (w, CURRENT_HEADER_LINE_ACTIVE_FACE_ID! (w),
				 if NILP (window_header_line_format)
				 { BVAR! (current_buffer, header_line_format_) }
				 else { window_header_line_format });
    }

  if (EQ (mode_lines, Qmode_line ()) || EQ (mode_lines, Qt ()))
      && window_wants_mode_line (w)
    {
      let window_mode_line_format: Lisp_Object
	= window_parameter (w, Qmode_line_format ());

      y = y + display_mode_line (w, CURRENT_MODE_LINE_ACTIVE_FACE_ID! (w),
				 if NILP (window_mode_line_format)
				 { BVAR! (current_buffer, mode_line_format_) }
				 else { window_mode_line_format });
    }

  bidi_unshelve_cache (itdata, false);

  return if vertical_offset == 0
	 { Fcons (make_fixnum ((x - start_x) as EMACS_INT), make_fixnum (y as EMACS_INT)) }
	 else { list3i ((x - start_x) as EMACS_INT, y as EMACS_INT, start as EMACS_INT) };
}

DEFUN! ("window-text-pixel-size", Fwindow_text_pixel_size, Swindow_text_pixel_size, 0, 7, 0,
       doc: "Return the dimensions of the text of WINDOW's buffer in pixels.\nWINDOW must be a live window and defaults to the selected one.  The\nreturn value is a cons of the maximum pixel-width of any text line and\nthe pixel-height of all the text lines in the accessible portion of\nbuffer text.\n\nIf FROM is a cons cell, the return value includes, in addition to the\ndimensions, also a third element that provides the buffer position\nfrom which measuring of the text dimensions was actually started.\n\nThis function exists to allow Lisp programs to adjust the dimensions\nof WINDOW to the buffer text it needs to display.\n\nThe optional argument FROM, if non-nil, specifies the first text\nposition to consider, and defaults to the minimum accessible position\nof the buffer.  If FROM is a cons, its car specifies a buffer\nposition, and its cdr specifies the vertical offset in pixels from\nthat position to the first screen line to be measured.  If FROM is t,\nit stands for the minimum accessible position that starts a non-empty\nline.  TO, if non-nil, specifies the last text position and defaults\nto the maximum accessible position of the buffer.  If TO is t, it\nstands for the maximum accessible position that ends a non-empty line.\n\nThe optional argument X-LIMIT, if non-nil, specifies the maximum X\ncoordinate beyond which the text should be ignored.  It is therefore\nalso the maximum width that the function can return.  X-LIMIT nil or\nomitted means to use the pixel-width of WINDOW's body.  This default\nmeans text of truncated lines wider than the window will be ignored;\nspecify a non-nil value for X-LIMIT if lines are truncated and you need\nto account for the truncated text.\n\nUse nil for X-LIMIT if you want to know how high WINDOW should become in\norder to fit all of its buffer's text with the width of WINDOW\nunaltered.  Use the maximum width WINDOW may assume if you intend to\nchange WINDOW's width.  Use t for the maximum possible value.  Since\ncalculating the width of long lines can take some time, it's always a\ngood idea to make this argument as small as possible; in particular, if\nthe buffer contains long lines that shall be truncated anyway.\n\nThe optional argument Y-LIMIT, if non-nil, specifies the maximum Y\ncoordinate beyond which the text is to be ignored; it is therefore\nalso the maximum height that the function can return (excluding the\nheight of the mode- or header-line, if any).  Y-LIMIT nil or omitted\nmeans consider all of the accessible portion of buffer text up to the\nposition specified by TO.  Since calculating the text height of a\nlarge buffer can take some time, it makes sense to specify this\nargument if the size of the buffer is large or unknown.\n\nOptional argument MODE-LINES nil or omitted means do not include the\nheight of the mode-, tab- or header-line of WINDOW in the return value.\nIf it is the symbol `mode-line', `tab-line' or `header-line', include\nonly the height of that line, if present, in the return value.  If t,\ninclude the height of any of these, if present, in the return value.\n\nIGNORE-LINE-AT-END, if non-nil, means to not add the height of the\nscreen line that includes TO to the returned height of the text.",
       (window: Lisp_Object, from: Lisp_Object, to: Lisp_Object, x_limit: Lisp_Object,
	y_limit: Lisp_Object, mode_lines: Lisp_Object, ignore_line_at_end: Lisp_Object),
       {
  let w: *mut window = decode_live_window (window);
  let b: *mut buffer = XBUFFER ((*w).contents);
  let mut old_b: *mut buffer = core::ptr::null_mut ();
  let value: Lisp_Object;

  if b != current_buffer
    {
      old_b = current_buffer;
      set_buffer_internal_1 (b);
    }

  value = window_text_pixel_size (window, from, to, x_limit, y_limit, mode_lines,
				  ignore_line_at_end);

  if !old_b.is_null ()
    { set_buffer_internal_1 (old_b); }

  return value;
});

DEFUN! ("buffer-text-pixel-size", Fbuffer_text_pixel_size, Sbuffer_text_pixel_size, 0, 6, 0,
       doc: "Return the dimensions of whole text of BUFFER-OR-NAME in WINDOW.\nBUFFER-OR-NAME must specify a live buffer or the name of a live buffer\nand defaults to the current buffer.  WINDOW must be a live window and\ndefaults to the selected one.  The return value is a cons of the maximum\npixel-width of any text line and the pixel-height of all the text lines\nof the buffer specified by BUFFER-OR-NAME.\n\nThe optional arguments X-LIMIT, Y-LIMIT, FROM and TO, have the same\nmeaning as with `window-text-pixel-size'.\n\nDo not use this function if the buffer specified by BUFFER-OR-NAME is\nalready displayed in WINDOW.  `window-text-pixel-size' is cheaper in\nthat case because it does not have to temporarily show that buffer in\nWINDOW.",
       (buffer_or_name: Lisp_Object, window: Lisp_Object, x_limit: Lisp_Object,
	y_limit: Lisp_Object, from: Lisp_Object, to: Lisp_Object),
       {
  let w: *mut window = decode_live_window (window);
  let b: *mut buffer = if NILP (buffer_or_name)
		       { current_buffer }
		       else { XBUFFER (Fget_buffer (buffer_or_name)) };
  let mut buffer: Lisp_Object = Qnil ();
  let value: Lisp_Object;
  let count: specpdl_ref = SPECPDL_INDEX ();

  XSETBUFFER! (buffer, b);

  /* The unwind form of with_echo_area_buffer is what we need here to
     make WINDOW temporarily show our buffer.  */
  /* FIXME: Can we move this into the `if (!EQ (buffer, w->contents))`?  */
  record_unwind_protect (Some (unwind_with_echo_area_buffer),
			 with_echo_area_buffer_unwind_data (w));

  set_buffer_internal_1 (b);

  let base_line_pos: ptrdiff_t = (*w).base_line_pos;
  let end_valid: c_int = if window_bits! (w, window_end_valid) { 1 } else { 0 };
  if !EQ (buffer, (*w).contents)
    {
      wset_buffer (w, buffer);
      set_marker_both ((*w).pointm, buffer, BEG as ptrdiff_t, BEG_BYTE as ptrdiff_t);
      set_marker_both ((*w).old_pointm, buffer, BEG as ptrdiff_t, BEG_BYTE as ptrdiff_t);
    }

  value = window_text_pixel_size (window, from, to, x_limit, y_limit, Qnil (), Qnil ());

  unbind_to (count, Qnil ());

  /* Restore original values.  This is important if this function is
     called from some ':eval' form in the middle of redisplay.  */
  (*w).base_line_pos = base_line_pos;
  window_bits! (w, window_end_valid, end_valid != 0);

  return value;
});

DEFUN! ("display--line-is-continued-p", Fdisplay__line_is_continued_p,
       Sdisplay__line_is_continued_p, 0, 0, 0,
       doc: "Return non-nil if the current screen line is continued on display.",
       (),
       {
  let oldb: *mut buffer = current_buffer;
  let w: *mut window = XWINDOW (selected_window);
  let mut rc: move_it_result = MOVE_POS_MATCH_OR_ZV;

  set_buffer_internal_1 (XBUFFER ((*w).contents));

  if PT! () < ZV! ()
    {
      let mut startpos: text_pos = core::mem::zeroed ();
      let mut it: it = core::mem::zeroed ();
      let mut itdata: *mut c_void;
      /* Use a marker, since vertical-motion enters redisplay, which can
	 trigger fontifications, which in turn could modify buffer text.  */
      let opoint: Lisp_Object = Fpoint_marker ();

      /* Make sure to start from the beginning of the current screen
	 line, so that move_it_in_display_line_to counts pixels correctly.  */
      Fvertical_motion (make_fixnum (0), selected_window, Qnil ());
      SET_TEXT_POS! (startpos, PT! (), PT_BYTE! ());
      itdata = bidi_shelve_cache ();
      start_display (&raw mut it, w, startpos);
      /* If lines are truncated, no line is continued.  */
      if it.line_wrap != TRUNCATE
	{
	  it.glyph_row = core::ptr::null_mut ();
	  rc = move_it_in_display_line_to (&raw mut it, ZV! (), -1, MOVE_TO_POS);
	}
      SET_PT_BOTH (marker_position (opoint), marker_byte_position (opoint));
      bidi_unshelve_cache (itdata, false);
    }
  set_buffer_internal_1 (oldb);

  return if rc == MOVE_LINE_CONTINUED { Qt () } else { Qnil () };
});

/***********************************************************************
			       Messages
 ***********************************************************************/

/* Return the number of arguments the format string FORMAT needs.  */

unsafe fn format_nargs (format: *const c_char) -> ptrdiff_t
{
  let mut nargs: ptrdiff_t = 0;
  let mut p: *const c_char = format;
  loop
    {
      p = strchr (p, '%' as c_int);
      if p.is_null () { break; }
      if *p.offset (1) == '%' as c_char
	{ p = p.offset (1); }
      else
	{ nargs += 1; }
      p = p.offset (1);
    }
  return nargs;
}

/* Add a message with format string FORMAT and formatted arguments
   to *Messages*.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn add_to_log (format: *const c_char, mut args: ...)
{
  let mut ap: VaList<'_> = args;
  vadd_to_log (format, ap);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vadd_to_log (format: *const c_char, mut ap: VaList<'_>)
{
  let form_nargs: ptrdiff_t = format_nargs (format);
  let nargs: ptrdiff_t = 1 + form_nargs;
  let mut args: [Lisp_Object; 10] = [Qnil (); 10];
  eassert! (nargs <= countof! (args) as ptrdiff_t);
  AUTO_STRING! (args0, format);
  args[0] = args0;
  let mut i: ptrdiff_t = 1;
  while i < nargs
    {
      args[i as usize] = ap.next_arg::<Lisp_Object> ();
      i += 1;
    }
  let mut msg: Lisp_Object = Qnil ();
  msg = Fformat_message (nargs, args.as_mut_ptr ());

  let len: ptrdiff_t = SBYTES (msg) + 1;
  USE_SAFE_ALLOCA! (sa_avail, sa_count);
  let buffer: *mut c_char = SAFE_ALLOCA! (sa_avail, len) as *mut c_char;
  memcpy (buffer as *mut c_void, SDATA (msg) as *const c_void, len as usize);

  message_dolog (buffer, len - 1, true, STRING_MULTIBYTE (msg));
  SAFE_FREE! (sa_count);
}

/* Output a newline in the *Messages* buffer if "needs" one.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn message_log_maybe_newline ()
{
  if message_log_need_newline
    { message_dolog (c"".as_ptr (), 0, true, false); }
}

/* Reset the message_log_need_newline flag, for when we split the
   message into two or more parts, and don't want a newline between
   them.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reset_message_log_need_newline ()
{
  message_log_need_newline = false;
}

/* Add a string M of length NBYTES to the message log, optionally
   terminated with a newline when NLFLAG is true.  MULTIBYTE, if
   true, means interpret the contents of M as multibyte.  This
   function calls low-level routines in order to bypass text property
   hooks, etc. which might not be safe to run.

   This may GC (insert may run before/after change hooks),
   so the buffer M must NOT point to a Lisp string.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn message_dolog (m: *const c_char, nbytes: ptrdiff_t,
					nlflag: bool, multibyte: bool)
{
  let msg: *const u8 = m as *const u8;

  if !NILP (Vmemory_full)
    { return; }

  if !NILP (Vmessage_log_max)
    {
      let mut oldbuf: *mut buffer;
      let mut oldpoint: Lisp_Object;
      let mut oldbegv: Lisp_Object;
      let mut oldzv: Lisp_Object;
      let old_windows_or_buffers_changed: c_int = windows_or_buffers_changed;
      let mut point_at_end: ptrdiff_t = 0;
      let mut zv_at_end: ptrdiff_t = 0;
      let mut old_deactivate_mark: Lisp_Object;

      old_deactivate_mark = Vdeactivate_mark;
      oldbuf = current_buffer;

      /* Sanity check, in case the variable has been set to something
	 invalid.  */
      if ! STRINGP (Vmessages_buffer_name)
	{ Vmessages_buffer_name = build_string (c"*Messages*".as_ptr ()); }
      /* Ensure the Messages buffer exists, and switch to it.
	 If we created it, set the major-mode.  */
      let newbuffer: bool = NILP (Fget_buffer (Vmessages_buffer_name));
      Fset_buffer (Fget_buffer_create (Vmessages_buffer_name, Qnil ()));
      if newbuffer
	  && !NILP (Ffboundp (Qmessages_buffer_mode ()))
	{ call0 (Qmessages_buffer_mode ()); }

      bset_undo_list (current_buffer, Qt ());
      bset_cache_long_scans (current_buffer, Qnil ());

      oldpoint = message_dolog_marker1;
      set_marker_restricted_both (oldpoint, Qnil (), PT! (), PT_BYTE! ());
      oldbegv = message_dolog_marker2;
      set_marker_restricted_both (oldbegv, Qnil (), BEGV! (), BEGV_BYTE! ());
      oldzv = message_dolog_marker3;
      set_marker_restricted_both (oldzv, Qnil (), ZV! (), ZV_BYTE! ());

      if PT! () == Z! ()
	{ point_at_end = 1; }
      if ZV! () == Z! ()
	{ zv_at_end = 1; }

      BEGV! () = BEG as ptrdiff_t;
      BEGV_BYTE! () = BEG_BYTE as ptrdiff_t;
      ZV! () = Z! ();
      ZV_BYTE! () = Z_BYTE! ();
      TEMP_SET_PT_BOTH (Z! (), Z_BYTE! ());

      /* Insert the string--maybe converting multibyte to single byte
	 or vice versa, so that all the text fits the buffer.  */
      if multibyte
	  && NILP (BVAR! (current_buffer, enable_multibyte_characters_))
	{
	  /* Convert a multibyte string to single-byte
	     for the *Message* buffer.  */
	  let mut i: ptrdiff_t = 0;
	  while i < nbytes
	    {
	      let mut char_bytes: c_int = 0;
	      let c: c_int = check_char_and_length (msg.offset (i as isize),
						    &raw mut char_bytes);
	      let work: c_char = CHAR_TO_BYTE8 (c) as c_char;
	      insert_1_both (&raw const work, 1, 1, true, false, false);
	      i += char_bytes as ptrdiff_t;
	    }
	}
      else if ! multibyte
	       && ! NILP (BVAR! (current_buffer, enable_multibyte_characters_))
	{
	  /* Convert a single-byte string to multibyte
	     for the *Message* buffer.  */
	  let mut i: ptrdiff_t = 0;
	  while i < nbytes
	    {
	      let c: c_int = make_char_multibyte (*msg.offset (i) as c_int);
	      let mut str_: [u8; MAX_MULTIBYTE_LENGTH as usize]
		= [0; MAX_MULTIBYTE_LENGTH as usize];
	      let char_bytes: c_int = CHAR_STRING (c, str_.as_mut_ptr ());
	      insert_1_both (str_.as_ptr () as *const c_char, 1,
			     char_bytes as ptrdiff_t, true, false, false);
	      i += 1;
	    }
	}
      else if nbytes != 0
	{
	  insert_1_both (m, chars_in_text (msg, nbytes), nbytes,
			 true, false, false);
	}

      if nlflag
	{
	  let mut this_bol: ptrdiff_t;
	  let mut this_bol_byte: ptrdiff_t;
	  let mut prev_bol: ptrdiff_t;
	  let mut prev_bol_byte: ptrdiff_t;
	  let mut dups: intmax_t;

	  /* Since we call del_range_both passing false for PREPARE,
	     we aren't prepared to run modification hooks (we could
	     end up calling modification hooks from another buffer and
	     only with AFTER=t, Bug#21824).  */
	  let count: specpdl_ref = SPECPDL_INDEX ();
	  specbind (Qinhibit_modification_hooks (), Qt ());

	  insert_1_both (c"\n".as_ptr (), 1, 1, true, false, false);

	  scan_newline (Z! (), Z_BYTE! (), BEG as ptrdiff_t, BEG_BYTE as ptrdiff_t, -2, false);
	  this_bol = PT! ();
	  this_bol_byte = PT_BYTE! ();

	  /* See if this line duplicates the previous one.
	     If so, combine duplicates.  */
	  if this_bol > BEG as ptrdiff_t
	    {
	      scan_newline (PT! (), PT_BYTE! (), BEG as ptrdiff_t, BEG_BYTE as ptrdiff_t, -2, false);
	      prev_bol = PT! ();
	      prev_bol_byte = PT_BYTE! ();

	      dups = message_log_check_duplicate (prev_bol_byte,
						  this_bol_byte);
	      if dups != 0
		{
		  del_range_both (prev_bol, prev_bol_byte,
				  this_bol, this_bol_byte, false);
		  if dups > 1
		    {
		      /* C: `char dupstr[sizeof " [ times]" + INT_STRLEN_BOUND (dups)];`
			 - clang on the frozen target measures 10 + 20 = 30
			 (deviation 93).  */
		      let mut dupstr: [c_char; 30] = [0; 30];

		      /* If you change this format, don't forget to also
			 change message_log_check_duplicate.  */
		      let duplen: c_int = sprintf (dupstr.as_mut_ptr (),
						   c" [jd times]".as_ptr (), dups);
		      TEMP_SET_PT_BOTH (Z! () - 1, Z_BYTE! () - 1);
		      insert_1_both (dupstr.as_ptr (), duplen as ptrdiff_t,
				     duplen as ptrdiff_t, true, false, true);
		    }
		}
	    }

	  /* If we have more than the desired maximum number of lines
	     in the *Messages* buffer now, delete the oldest ones.
	     This is safe because we don't have undo in this buffer.  */

	  if FIXNATP (Vmessage_log_max)
	    {
	      scan_newline (Z! (), Z_BYTE! (), BEG as ptrdiff_t, BEG_BYTE as ptrdiff_t,
			    -XFIXNAT (Vmessage_log_max) - 1, false);
	      del_range_both (BEG as ptrdiff_t, BEG_BYTE as ptrdiff_t, PT! (), PT_BYTE! (), false);
	    }

	  unbind_to (count, Qnil ());
	}
      BEGV! () = marker_position (oldbegv);
      BEGV_BYTE! () = marker_byte_position (oldbegv);

      if zv_at_end != 0
	{
	  ZV! () = Z! ();
	  ZV_BYTE! () = Z_BYTE! ();
	}
      else
	{
	  ZV! () = marker_position (oldzv);
	  ZV_BYTE! () = marker_byte_position (oldzv);
	}

      if point_at_end != 0
	{ TEMP_SET_PT_BOTH (Z! (), Z_BYTE! ()); }
      else
	/* We can't do Fgoto_char (oldpoint) because it will run some
	   Lisp code.  */
	{
	  TEMP_SET_PT_BOTH (marker_position (oldpoint),
			    marker_byte_position (oldpoint));
	}

      unchain_marker (XMARKER (oldpoint) as *mut c_void);
      unchain_marker (XMARKER (oldbegv) as *mut c_void);
      unchain_marker (XMARKER (oldzv) as *mut c_void);

      /* We called insert_1_both above with its 5th argument (PREPARE)
	 false, which prevents insert_1_both from calling
	 prepare_to_modify_buffer, which in turns prevents us from
	 incrementing windows_or_buffers_changed even if *Messages* is
	 shown in some window.  So we must manually set
	 windows_or_buffers_changed here to make up for that.  */
      windows_or_buffers_changed = old_windows_or_buffers_changed;
      bset_redisplay (current_buffer);

      set_buffer_internal (oldbuf);

      message_log_need_newline = !nlflag;
      Vdeactivate_mark = old_deactivate_mark;
    }
}
/* We are at the end of the buffer after just having inserted a newline.
   (Note: We depend on the fact we won't be crossing the gap.)
   Check to see if the most recent message looks a lot like the previous one.
   Return 0 if different, 1 if the new one should just replace it, or a
   value N > 1 if we should also append " [N times]".  */

unsafe fn message_log_check_duplicate (prev_bol_byte: ptrdiff_t,
				       this_bol_byte: ptrdiff_t) -> intmax_t
{
  let mut i: ptrdiff_t;
  let len: ptrdiff_t = Z_BYTE! () - 1 - this_bol_byte;
  let mut seen_dots: bool = false;
  let mut p1: *mut u8 = BUF_BYTE_ADDRESS (current_buffer, prev_bol_byte);
  let p2: *mut u8 = BUF_BYTE_ADDRESS (current_buffer, this_bol_byte);

  i = 0;
  while i < len
    {
      if i >= 3 && *p1.offset (i as isize - 3) == b'.'
	  && *p1.offset (i as isize - 2) == b'.'
	  && *p1.offset (i as isize - 1) == b'.'
	{ seen_dots = true; }
      if *p1.offset (i as isize) != *p2.offset (i as isize)
	{ return seen_dots as intmax_t; }
      i += 1;
    }
  p1 = p1.offset (len as isize);
  if *p1 == b'\n' as u8
    { return 2; }
  /* C's `if (*p1++ == ' ' && *p1++ == '[')`: each test's own post-increment runs whether or
     not that test's result is used, and `&&` still short-circuits the second (deviation 104).  */
  if { let v: bool = *p1 == b' ' as u8; p1 = p1.offset (1); v }
      && { let v: bool = *p1 == b'[' as u8; p1 = p1.offset (1); v }
    {
      let mut pend: *mut c_char = core::ptr::null_mut ();
      let n: intmax_t = strtoimax (p1 as *const c_char, &raw mut pend, 10);
      if 0 < n && n < INTMAX_MAX && strncmp (pend, c" times]\n".as_ptr (), 8) == 0
	{ return n + 1; }
    }
  return 0;
}

unsafe fn log_message (m: Lisp_Object)
{
  clear_message (true, true);
  cancel_echoing ();

  /* First flush out any partial line written with print.  */
  message_log_maybe_newline ();
  if STRINGP (m)
    {
      let nbytes: ptrdiff_t = SBYTES (m);
      let multibyte: bool = STRING_MULTIBYTE (m);
      let mut buffer: *mut c_char;
      USE_SAFE_ALLOCA! (sa_avail, sa_count);
      SAFE_ALLOCA_STRING! (sa_avail, buffer, m);
      message_dolog (buffer, nbytes, true, multibyte);
      SAFE_FREE! (sa_count);
    }
}

/* Display an echo area message M with a specified length of NBYTES
   bytes.  The string may include null characters.  If M is not a
   string, clear out any existing message, and let the mini-buffer
   text show through.

   This function cancels echoing.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn message3 (m: Lisp_Object)
{
  log_message (m);
  if !inhibit_message
    { message3_nolog (m); }
}

/* Display an echo area message M on frame F, which may not be the
   selected frame.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn message3_frame (m: Lisp_Object, f: *mut frame)
{
  log_message (m);
  if !inhibit_message
    { message3_frame_nolog (m, f); }
}

/* Log the message M to stderr.  Log an empty line if M is not a string.  */

unsafe fn message_to_stderr (m: Lisp_Object)
{
  if noninteractive_need_newline
    {
      noninteractive_need_newline = false;
      errputc ('\n' as c_int);
    }
  if STRINGP (m)
    {
      let mut coding_system: Lisp_Object = Vlocale_coding_system;
      let mut s: Lisp_Object;

      if !NILP (Vcoding_system_for_write)
	{ coding_system = Vcoding_system_for_write; }
      if !NILP (coding_system)
	{ s = code_convert_string_norecord (m, coding_system, true); }
      else
	{ s = m; }

      errwrite (SDATA (s) as *const c_void, SBYTES (s));
    }
  if STRINGP (m) || !cursor_in_echo_area
    { errputc ('\n' as c_int); }
}

/* The non-logging versions of message3 & message3_frame.
   This does not cancel echoing, because it is used for echoing.
   Perhaps we need to make a separate function for echoing
   and make this cancel echoing.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn message3_nolog (m: Lisp_Object)
{
  message3_frame_nolog (m, core::ptr::null_mut ());
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn message3_frame_nolog (m: Lisp_Object, f: *mut frame)
{
  let sf: *mut frame = SELECTED_FRAME ();
  let mut f = f;
  if f.is_null () { f = sf; }

  if FRAME_INITIAL_P (f)
    { message_to_stderr (m); }
  /* Error messages get reported properly by cmd_error, so this must be
     just an informative message; therefore if the frame hasn't really
     been initialized yet, just toss it.  */
  else if INTERACTIVE! () && frame_bits! (glyphs_initialized_p, f)
    {
      let mut frame: Lisp_Object = Qnil ();
      let mut mbf: *mut frame = core::ptr::null_mut ();
      let count: specpdl_ref = SPECPDL_INDEX ();

      if f == sf
	{
	  /* Get the frame containing the mini-buffer that the selected
	     frame is using.  */
	  let mini_window: Lisp_Object = FRAME_MINIBUF_WINDOW (f);
	  frame = (*(XWINDOW (mini_window))).frame;
	  mbf = XFRAME (frame);

	  if FRAME_VISIBLE_P (f) && !FRAME_VISIBLE_P (mbf)
	    { Fmake_frame_visible (frame); }
	}
      else
	{
	  /* We temporarily switch frame, show the message, and then
	     when we unwind the message will normally still be visible
	     in the other frame, at least for a few seconds.  */
	  record_unwind_protect
	    (Some (restore_selected_window), selected_window);
	  record_unwind_protect
	    (Some (restore_frame_selected_window), (*f).selected_window);
	  XSETFRAME! (frame, f);
	  selected_frame = frame;
	  selected_window = FRAME_SELECTED_WINDOW (f);
	}

      if STRINGP (m) && SCHARS (m) > 0
	{
	  set_message (m);
	  if minibuffer_auto_raise && !NILP (frame)
	    { Fraise_frame (frame); }
	  /* Assume we are not echoing.
	     (If we are, echo_now will override this.)  */
	  echo_message_buffer = Qnil ();
	}
      else
	{ clear_message (true, true); }

      do_pending_window_change (false);
      echo_area_display (true);
      do_pending_window_change (false);
      if !mbf.is_null ()
	  && !(*FRAME_TERMINAL! (mbf)).frame_up_to_date_hook.is_none ()
	{
	  (*FRAME_TERMINAL! (mbf)).frame_up_to_date_hook.unwrap () (mbf as *mut c_void);
	}

      unbind_to (count, Qnil ());
    }
}

/* Display a null-terminated echo area message M.  If M is 0, clear
   out any existing message, and let the mini-buffer text show through.

   The buffer M must continue to exist until after the echo area gets
   cleared or some other message gets displayed there.  Do not pass
   text that is stored in a Lisp string.  Do not pass text in a buffer
   that was alloca'd.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn message1 (m: *const c_char)
{
  message3 (if !m.is_null () { build_unibyte_string (m) } else { Qnil () });
}

/* The non-logging counterpart of message1.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn message1_nolog (m: *const c_char)
{
  message3_nolog (if !m.is_null () { build_unibyte_string (m) } else { Qnil () });
}

/* Display a message M which contains a single %s
   which gets replaced with STRING.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn message_with_string (m: *const c_char, string: Lisp_Object,
					      log: bool)
{
  CHECK_STRING (string);

  let need_message: bool;
  if noninteractive
    { need_message = !m.is_null (); }
  else if !INTERACTIVE! ()
    { need_message = false; }
  else
    {
      /* The frame whose minibuffer we're going to display the message on.
	 It may be larger than the selected frame, so we need
	 to use its buffer, not the selected frame's buffer.  */
      let mini_window: Lisp_Object;
      let f: *mut frame;
      let sf: *mut frame = SELECTED_FRAME ();

      /* Get the frame containing the minibuffer
	 that the selected frame is using.  */
      mini_window = FRAME_MINIBUF_WINDOW (sf);
      f = XFRAME (WINDOW_FRAME! (XWINDOW (mini_window)));

      /* Error messages get reported properly by cmd_error, so this must be
	 just an informative message; if the frame hasn't really been
	 initialized yet, just toss it.  */
      need_message = frame_bits! (glyphs_initialized_p, f);
    }

  if need_message
    {
      AUTO_STRING! (fmt, m);
      let msg: Lisp_Object = CALLN! (Fformat_message, fmt, string);

      if noninteractive
	{ message_to_stderr (msg); }
      else
	{
	  if log
	    { message3 (msg); }
	  else
	    { message3_nolog (msg); }

	  /* Print should start at the beginning of the message
	     buffer next time.  */
	  message_buf_print = false;
	}
    }
}


/* Dump an informative message to the minibuf.  If M is 0, clear out
   any existing message, and let the mini-buffer text show through.

   The message must be safe ASCII (because when Emacs is
   non-interactive the message is sent straight to stderr without
   encoding first) and the format must not contain ` or ' (because
   this function does not account for `text-quoting-style').  If your
   message and format do not fit into this category, convert your
   arguments to Lisp objects and use Fmessage instead.  */

/* C's `ATTRIBUTE_FORMAT_PRINTF (1, 0)` is a compiler attribute with no Rust counterpart
   (it only affects diagnostics).  */
unsafe fn vmessage (m: *const c_char, mut ap: VaList<'_>)
{
  if noninteractive
    {
      if !m.is_null ()
	{
	  if noninteractive_need_newline
	    { putc ('\n' as c_int, stderr); }
	  noninteractive_need_newline = false;
	  let va: VaList<'_> = ap;
	  vfprintf (stderr, m, va);
	  if !cursor_in_echo_area
	    { putc ('\n' as c_int, stderr); }
	  fflush (stderr);
	}
    }
  else if INTERACTIVE! ()
    {
      /* The frame whose mini-buffer we're going to display the message
	 on.  It may be larger than the selected frame, so we need to
	 use its buffer, not the selected frame's buffer.  */
      let mini_window: Lisp_Object;
      let f: *mut frame;
      let sf: *mut frame = SELECTED_FRAME ();

      /* Get the frame containing the mini-buffer
	 that the selected frame is using.  */
      mini_window = FRAME_MINIBUF_WINDOW (sf);
      f = XFRAME (WINDOW_FRAME! (XWINDOW (mini_window)));

      /* Error messages get reported properly by cmd_error, so this must be
	 just an informative message; if the frame hasn't really been
	 initialized yet, just toss it.  */
      if frame_bits! (glyphs_initialized_p, f)
	{
	  if !m.is_null ()
	    {
	      let mut len: ptrdiff_t;
	      let maxsize: ptrdiff_t = FRAME_MESSAGE_BUF_SIZE (f) as ptrdiff_t;
	      let message_bufsize: ptrdiff_t = maxsize + MAX_MULTIBYTE_LENGTH as ptrdiff_t;
	      USE_SAFE_ALLOCA! (sa_avail, sa_count);
	      let message_buf: *mut c_char = SAFE_ALLOCA! (sa_avail, message_bufsize) as *mut c_char;

	      len = doprnt (message_buf, message_bufsize, m, core::ptr::null (), ap);
	      /* doprnt returns the buffer size minus one when it
		 truncated a multibyte sequence.  Work around that by
		 truncating to the last valid multibyte head.  */
	      if 0 < maxsize && maxsize <= len
		{
		  len = maxsize - 1;
		  while 0 < len && !CHAR_HEAD_P (*message_buf.offset (len as isize) as c_int)
		    { len -= 1; }
		  *message_buf.offset (len as isize) = 0;
		}
	      message3 (make_string (message_buf, len));
	      SAFE_FREE! (sa_count);
	    }
	  else
	    { message1 (core::ptr::null ()); }

	  /* Print should start at the beginning of the message
	     buffer next time.  */
	  message_buf_print = false;
	}
    }
}

/* See vmessage for restrictions on the text of the message.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn message (m: *const c_char, mut args: ...)
{
  let mut ap: VaList<'_> = args;
  vmessage (m, ap);
}

/* Display the current message in the current mini-buffer.  This is
   only called from error handlers in process.c, and is not time
   critical.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn update_echo_area ()
{
  if !NILP (echo_area_buffer[0])
    {
      let string: Lisp_Object;
      string = Fcurrent_message ();
      message3 (string);
    }
}

/* Make sure echo area buffers in `echo_buffers' are live.
   If they aren't, make new ones.  */

unsafe fn ensure_echo_area_buffers ()
{
  let mut i: c_int = 0;
  while i < 2
    {
      if !BUFFERP (echo_buffer[i as usize])
	  || !BUFFER_LIVE_P (XBUFFER (echo_buffer[i as usize]))
	{
	  let old_buffer: Lisp_Object = echo_buffer[i as usize];
	  /* C: `static char const name_fmt[] = " *Echo Area %d*"; char name[sizeof name_fmt
	     + INT_STRLEN_BOUND (int)];` -- clang on the frozen target measures 16 + 11 = 27
	     (deviation 105).  */
	  let name_fmt: *const c_char = c" *Echo Area %d*".as_ptr ();
	  let mut name: [c_char; 27] = [0; 27];
	  AUTO_STRING_WITH_LEN! (lname, name.as_ptr (),
				 sprintf (name.as_mut_ptr (), name_fmt, i) as isize);
	  echo_buffer[i as usize] = Fget_buffer_create (lname, Qnil ());
	  bset_truncate_lines (XBUFFER (echo_buffer[i as usize]), Qnil ());
	  /* to force word wrap in echo area -
	     it was decided to postpone this*/
	  /* XBUFFER (echo_buffer[i])->word_wrap = Qt; */

	  let mut j: c_int = 0;
	  while j < 2
	    {
	      if EQ (old_buffer, echo_area_buffer[j as usize])
		{ echo_area_buffer[j as usize] = echo_buffer[i as usize]; }
	      j += 1;
	    }
	}
      i += 1;
    }
}


/* Call FN with args A1..A2 with either the current or last displayed
   echo_area_buffer as current buffer.

   WHICH zero means use the current message buffer
   echo_area_buffer[0].  If that is nil, choose a suitable buffer
   from echo_buffer[] and clear it.

   WHICH > 0 means use echo_area_buffer[1].  If that is nil, choose a
   suitable buffer from echo_buffer[] and clear it.

   If WHICH < 0, set echo_area_buffer[1] to echo_area_buffer[0], so
   that the current message becomes the last displayed one, choose a
   suitable buffer for echo_area_buffer[0], and clear it.

   Value is what FN returns.  */

unsafe fn with_echo_area_buffer (w: *mut window, which: c_int,
				 fn_: Option<unsafe extern "C" fn (*mut c_void,
								   Lisp_Object) -> bool>,
				 a1: *mut c_void, a2: Lisp_Object) -> bool
{
  let buffer: Lisp_Object;
  let this_one: bool;
  let the_other: bool;
  let mut clear_buffer_p: bool;
  let rc: bool;
  let count: specpdl_ref = SPECPDL_INDEX ();

  /* If buffers aren't live, make new ones.  */
  ensure_echo_area_buffers ();

  clear_buffer_p = false;

  if which == 0
    { this_one = false; the_other = true; }
  else if which > 0
    { this_one = true; the_other = false; }
  else
    {
      this_one = false; the_other = true;
      clear_buffer_p = true;

      /* We need a fresh one in case the current echo buffer equals
	 the one containing the last displayed echo area message.  */
      if !NILP (echo_area_buffer[this_one as usize])
	  && EQ (echo_area_buffer[this_one as usize], echo_area_buffer[the_other as usize])
	{ echo_area_buffer[this_one as usize] = Qnil (); }
    }

  /* Choose a suitable buffer from echo_buffer[] if we don't
     have one.  */
  if NILP (echo_area_buffer[this_one as usize])
    {
      echo_area_buffer[this_one as usize]
	= if EQ (echo_area_buffer[the_other as usize], echo_buffer[this_one as usize])
	  { echo_buffer[the_other as usize] }
	  else { echo_buffer[this_one as usize] };
      clear_buffer_p = true;
    }

  buffer = echo_area_buffer[this_one as usize];

  /* Don't get confused by reusing the buffer used for echoing
     for a different purpose.  */
  if echo_kboard.is_null () && EQ (buffer, echo_message_buffer)
    { cancel_echoing (); }

  record_unwind_protect (Some (unwind_with_echo_area_buffer),
			 with_echo_area_buffer_unwind_data (w));

  /* Make the echo area buffer current.  Note that for display
     purposes, it is not necessary that the displayed window's buffer
     == current_buffer, except for text property lookup.  So, let's
     only set that buffer temporarily here without doing a full
     Fset_window_buffer.  We must also change w->pointm, though,
     because otherwise an assertions in unshow_buffer fails, and Emacs
     aborts.  */
  set_buffer_internal_1 (XBUFFER (buffer));
  if !w.is_null ()
    {
      wset_buffer (w, buffer);
      set_marker_both ((*w).pointm, buffer, BEG as ptrdiff_t, BEG_BYTE as ptrdiff_t);
      set_marker_both ((*w).old_pointm, buffer, BEG as ptrdiff_t, BEG_BYTE as ptrdiff_t);
    }

  bset_undo_list (current_buffer, Qt ());
  bset_read_only (current_buffer, Qnil ());
  specbind (Qinhibit_read_only (), Qt ());
  specbind (Qinhibit_modification_hooks (), Qt ());

  if clear_buffer_p && Z! () > BEG as ptrdiff_t
    { del_range (BEG as ptrdiff_t, Z! ()); }

  eassert! (BEGV! () >= BEG as ptrdiff_t);
  eassert! (ZV! () <= Z! () && ZV! () >= BEGV! ());

  rc = (fn_.unwrap ()) (a1, a2);

  eassert! (BEGV! () >= BEG as ptrdiff_t);
  eassert! (ZV! () <= Z! () && ZV! () >= BEGV! ());

  unbind_to (count, Qnil ());
  return rc;
}

/* Save state that should be preserved around the call to the function
   FN called in with_echo_area_buffer.  */

unsafe fn with_echo_area_buffer_unwind_data (w: *mut window) -> Lisp_Object
{
  let mut i: c_int = 0;
  let mut vector: Lisp_Object;
  let mut tmp: Lisp_Object;

  /* Reduce consing by keeping one vector in
     Vwith_echo_area_save_vector.  */
  vector = Vwith_echo_area_save_vector;
  Vwith_echo_area_save_vector = Qnil ();

  if NILP (vector)
    { vector = make_nil_vector (11); }

  XSETBUFFER! (tmp, current_buffer); ASET (vector, i as isize, tmp); i += 1;
  ASET (vector, i as isize, Vdeactivate_mark); i += 1;
  ASET (vector, i as isize, make_fixnum (windows_or_buffers_changed as EMACS_INT)); i += 1;

  if !w.is_null ()
    {
      XSETWINDOW! (tmp, w); ASET (vector, i as isize, tmp); i += 1;
      ASET (vector, i as isize, (*w).contents); i += 1;
      ASET (vector, i as isize, make_fixnum (marker_position ((*w).pointm) as EMACS_INT)); i += 1;
      ASET (vector, i as isize, make_fixnum (marker_byte_position ((*w).pointm) as EMACS_INT)); i += 1;
      ASET (vector, i as isize, make_fixnum (marker_position ((*w).old_pointm) as EMACS_INT)); i += 1;
      ASET (vector, i as isize, make_fixnum (marker_byte_position ((*w).old_pointm) as EMACS_INT)); i += 1;
      ASET (vector, i as isize, make_fixnum (marker_position ((*w).start) as EMACS_INT)); i += 1;
      ASET (vector, i as isize, make_fixnum (marker_byte_position ((*w).start) as EMACS_INT)); i += 1;
    }
  else
    {
      let end: c_int = i + 8;
      while i < end
	{
	  ASET (vector, i as isize, Qnil ());
	  i += 1;
	}
    }

  eassert! (i as isize == ASIZE (vector));
  return vector;
}

/* Restore global state from VECTOR which was created by
   with_echo_area_buffer_unwind_data.  */

/* C's `static void` whose address goes to `record_unwind_protect` (deviation 18's rule):
   `unsafe extern "C" fn`, no `#[unsafe(no_mangle)]`.  */
unsafe extern "C" fn unwind_with_echo_area_buffer (vector: Lisp_Object)
{
  set_buffer_internal_1 (XBUFFER (AREF (vector, 0)));
  Vdeactivate_mark = AREF (vector, 1);
  windows_or_buffers_changed = XFIXNAT (AREF (vector, 2)) as c_int;

  if WINDOWP (AREF (vector, 3))
    {
      let w: *mut window;
      let buffer: Lisp_Object;

      w = XWINDOW (AREF (vector, 3));
      buffer = AREF (vector, 4);

      wset_buffer (w, buffer);
      set_marker_restricted_both ((*w).pointm, buffer,
				  XFIXNAT (AREF (vector, 5)),
				  XFIXNAT (AREF (vector, 6)));
      set_marker_restricted_both ((*w).old_pointm, buffer,
				  XFIXNAT (AREF (vector, 7)),
				  XFIXNAT (AREF (vector, 8)));
      set_marker_restricted_both ((*w).start, buffer,
				  XFIXNAT (AREF (vector, 9)),
				  XFIXNAT (AREF (vector, 10)));
    }

  Vwith_echo_area_save_vector = vector;
}

/* Set up the echo area for use by print functions.  MULTIBYTE_P
   means we will print multibyte.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn setup_echo_area_for_printing (multibyte_p: bool)
{
  /* If we can't find an echo area any more, exit.  */
  if ! FRAME_LIVE_P (XFRAME (selected_frame))
    { Fkill_emacs (Qnil (), Qnil ()); }

  ensure_echo_area_buffers ();

  if !message_buf_print
    {
      /* A message has been output since the last time we printed.
	 Choose a fresh echo area buffer.  */
      if EQ (echo_area_buffer[1], echo_buffer[0])
	{ echo_area_buffer[0] = echo_buffer[1]; }
      else
	{ echo_area_buffer[0] = echo_buffer[0]; }

      /* Switch to that buffer and clear it.  */
      set_buffer_internal (XBUFFER (echo_area_buffer[0]));
      bset_truncate_lines (current_buffer, Qnil ());

      if Z! () > BEG as ptrdiff_t
	{
	  let count: specpdl_ref = SPECPDL_INDEX ();
	  specbind (Qinhibit_read_only (), Qt ());
	  /* Note that undo recording is always disabled.  */
	  del_range (BEG as ptrdiff_t, Z! ());
	  unbind_to (count, Qnil ());
	}
      TEMP_SET_PT_BOTH (BEG as ptrdiff_t, BEG_BYTE as ptrdiff_t);

      /* Set up the buffer for the multibyteness we need.  We always
	 set it to be multibyte, except when
	 unibyte-display-via-language-environment is non-nil and the
	 buffer from which we are called is unibyte, because in that
	 case unibyte characters should not be displayed as octal
	 escapes.  */
      if unibyte_display_via_language_environment
	  && !multibyte_p
	  && !NILP (BVAR! (current_buffer, enable_multibyte_characters_))
	{ Fset_buffer_multibyte (Qnil ()); }
      else if NILP (BVAR! (current_buffer, enable_multibyte_characters_))
	{ Fset_buffer_multibyte (Qt ()); }

      /* Raise the frame containing the echo area.  */
      if minibuffer_auto_raise
	{
	  let sf: *mut frame = SELECTED_FRAME ();
	  let mini_window: Lisp_Object;
	  mini_window = FRAME_MINIBUF_WINDOW (sf);
	  Fraise_frame  (WINDOW_FRAME! (XWINDOW (mini_window)));
	}

      message_log_maybe_newline ();
      message_buf_print = true;
    }
  else
    {
      if NILP (echo_area_buffer[0])
	{
	  if EQ (echo_area_buffer[1], echo_buffer[0])
	    { echo_area_buffer[0] = echo_buffer[1]; }
	  else
	    { echo_area_buffer[0] = echo_buffer[0]; }
	}

      if current_buffer != XBUFFER (echo_area_buffer[0])
	{
	  /* Someone switched buffers between print requests.  */
	  set_buffer_internal (XBUFFER (echo_area_buffer[0]));
	  bset_truncate_lines (current_buffer, Qnil ());
	}
    }
}

/* Display an echo area message in window W.  Value is true if W's
   height is changed.  If display_last_displayed_message_p,
   display the message that was last displayed, otherwise
   display the current message.  */

unsafe fn display_echo_area (w: *mut window) -> bool
{
  let no_message_p: bool;
  let window_height_changed_p: bool;

  /* Temporarily disable garbage collections while displaying the echo
     area.  This is done because a GC can print a message itself.
     That message would modify the echo area buffer's contents while a
     redisplay of the buffer is going on, and seriously confuse
     redisplay.  */
  let count: specpdl_ref = inhibit_garbage_collection ();

  /* If there is no message, we must call display_echo_area_1
     nevertheless because it resizes the window.  But we will have to
     reset the echo_area_buffer in question to nil at the end because
     with_echo_area_buffer will set it to an empty buffer.  */
  let i: bool = display_last_displayed_message_p;
  /* According to the C standard, the integral value
     of a "bool" is always 0 or 1, so this array access is safe here,
     if oddly typed. */
  no_message_p = NILP (echo_area_buffer[i as usize]);

  window_height_changed_p
    = with_echo_area_buffer (w, if display_last_displayed_message_p { 1 } else { 0 },
			     Some (display_echo_area_1), w as *mut c_void, Qnil ());

  if no_message_p
    { echo_area_buffer[i as usize] = Qnil (); }

  unbind_to (count, Qnil ());
  return window_height_changed_p;
}

/* Helper for display_echo_area.  Display the current buffer which
   contains the current echo area message in window W, a mini-window,
   a pointer to which is passed in A1.  A2 is currently not used.
   Change the height of W so that all of the message is displayed.
   Value is true if height of W was changed.  */

/* C's `static bool` definition whose address goes to `with_echo_area_buffer`
   (deviation 18's rule): `unsafe extern "C" fn`, no `#[unsafe(no_mangle)]`.  */
unsafe extern "C" fn display_echo_area_1 (a1: *mut c_void, a2: Lisp_Object) -> bool
{
  let w: *mut window = a1 as *mut window;
  let window: Lisp_Object;
  let mut start: text_pos = core::mem::zeroed ();
  let _ = a2;

  /* We are about to enter redisplay without going through
     redisplay_internal, so we need to forget these faces by hand
     here.  */
  forget_escape_and_glyphless_faces ();

  /* Do this before displaying, so that we have a large enough glyph
     matrix for the display.  If we can't get enough space for the
     whole text, display the last N lines.  That works by setting w->start.  */
  let window_height_changed_p: bool = resize_mini_window (w, false);

  /* Use the starting position chosen by resize_mini_window.  */
  SET_TEXT_POS_FROM_MARKER! (start, (*w).start);

  /* Display.  */
  clear_glyph_matrix ((*w).desired_matrix as *mut glyph_matrix);
  XSETWINDOW! (window, w);
  let mut itdata: *mut c_void = bidi_shelve_cache ();
  try_window (window, start, 0);
  bidi_unshelve_cache (itdata, false);

  return window_height_changed_p;
}
/* Resize the echo area window to exactly the size needed for the
   currently displayed message, if there is one.  If a mini-buffer
   is active, don't shrink it.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn resize_echo_area_exactly ()
{
  if BUFFERP (echo_area_buffer[0])
      && WINDOWP (echo_area_window)
    {
      let w: *mut window = XWINDOW (echo_area_window);
      let resize_exactly: Lisp_Object = if minibuf_level == 0 { Qt () } else { Qnil () };
      let resized_p: bool = with_echo_area_buffer (w, 0, Some (resize_mini_window_1),
						   w as *mut c_void, resize_exactly);
      if resized_p
	{
	  windows_or_buffers_changed = 42;
	  update_mode_lines = 30;
	  redisplay_internal ();
	}
    }
}

/* Callback function for with_echo_area_buffer, when used from
   resize_echo_area_exactly.  A1 contains a pointer to the window to
   resize, EXACTLY non-nil means resize the mini-window exactly to the
   size of the text displayed.  Value is what resize_mini_window
   returns.  */

unsafe extern "C" fn resize_mini_window_1 (a1: *mut c_void, exactly: Lisp_Object) -> bool
{
  return resize_mini_window (a1 as *mut window, !NILP (exactly));
}

/* Resize mini-window W to fit the size of its contents.  EXACT_P
   means size the window exactly to the size needed.  Otherwise, it's
   only enlarged until W's buffer is empty.

   Set W->start to the right place to begin display.  If the whole
   contents fit, start at the beginning.  Otherwise, start so as
   to make the end of the contents appear.  This is particularly
   important for y-or-n-p, but seems desirable generally.

   Value is true if the window height has been changed.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn resize_mini_window (w: *mut window, exact_p: bool) -> bool
{
  let f: *mut frame = XFRAME ((*w).frame);
  let old_height: c_int = WINDOW_BOX_TEXT_HEIGHT! (w);

  eassert! (MINI_WINDOW_P (w));

  /* Don't resize windows while redisplaying a window; it would
     confuse redisplay functions when the size of the window they are
     displaying changes from under them.  Such a resizing can happen,
     for instance, when which-func prints a long message while
     we are running fontification-functions.  We're running these
     functions with dsafe_call which binds inhibit-redisplay to t.  */
  if !NILP (Vinhibit_redisplay)
    { return false; }

  /* By default, start display at the beginning.  */
  if redisplay_adhoc_scroll_in_resize_mini_windows
    {
      set_marker_both ((*w).start, (*w).contents,
		       BUF_BEGV (XBUFFER ((*w).contents)),
		       BUF_BEGV_BYTE (XBUFFER ((*w).contents)));
    }

  /* Nil means don't try to resize.  */
  if (NILP (Vresize_mini_windows)
       && (NILP (resize_mini_frames) || !FRAME_MINIBUF_ONLY_P (f)))
      || (FRAME_X_P (f) && (FRAME_OUTPUT_DATA! (f) as *mut c_void).is_null ())
    { return false; }

  if FRAME_MINIBUF_ONLY_P (f)
    {
      if !NILP (resize_mini_frames)
	{ dsafe_call1 (Qwindow__resize_mini_frame (), WINDOW_FRAME! (w)); }
    }
  else
    {
      let mut it: it = core::mem::zeroed ();
      let mut unit: c_int;
      let mut height: c_int;
      let mut max_height: c_int;
      let mut start: text_pos = core::mem::zeroed ();
      let mut old_current_buffer: *mut buffer = core::ptr::null_mut ();
      let windows_height: c_int = FRAME_INNER_HEIGHT (f);

      if current_buffer != XBUFFER ((*w).contents)
	{
	  old_current_buffer = current_buffer;
	  set_buffer_internal (XBUFFER ((*w).contents));
	}

      init_iterator (&raw mut it, w, BEGV! (), BEGV_BYTE! (), core::ptr::null_mut (),
		     DEFAULT_FACE_ID);

      /* Unit includes line spacing if line spacing is added above */
      unit = FRAME_LINE_HEIGHT (f) +
	  (if it.extra_line_spacing_above != 0 { it.extra_line_spacing } else { 0 });

      /* Compute the max. number of lines specified by the user.  */
      if FLOATP (Vmax_mini_window_height)
	{ max_height = (XFLOAT_DATA (Vmax_mini_window_height) * windows_height as f64) as c_int; }
      else if FIXNUMP (Vmax_mini_window_height)
	{ max_height = XFIXNUM (Vmax_mini_window_height) as c_int * unit; }
      else
	{ max_height = windows_height / 4; }

      /* Correct that max. height if it's bogus.  */
      max_height = clip_to_bounds (unit as isize, max_height as isize,
				   windows_height as isize) as c_int;

      /* Find out the height of the text in the window.  */
      last_height = 0;
      move_it_to (&raw mut it, ZV! (), -1, -1, -1, MOVE_TO_POS);
      /* If move_it_to moved to the next visible line after EOB,
	 account for the height of the last full line.  */
      if it.max_ascent == 0 && it.max_descent == 0
	{
	  height = it.current_y;
	  /* Don't add the last line's height if lines are truncated
	     and the text doesn't end in a newline.
	     FIXME: if the text ends in a newline from a display
	     property or an overlay string, they lose: the mini-window
	     might not show the last empty line.  */
	  if !(it.line_wrap == TRUNCATE
	       && it.current_x <= it.first_visible_x
	       && ZV_BYTE! () > 1
	       && FETCH_BYTE! (ZV_BYTE! () - 1) != b'\n')
	    { height += last_height; }
	}
      else
	{ height = it.current_y + it.max_ascent + it.max_descent; }

      /* Remove final line spacing in the mini-window */
      if it.extra_line_spacing_above == 0
	{ height -= min (it.extra_line_spacing as isize,
			 it.max_extra_line_spacing as isize) as c_int; }

      /* Compute a suitable window start.  */
      if height > max_height
	{
	  height = (max_height / unit) * unit;
	  if redisplay_adhoc_scroll_in_resize_mini_windows
	    {
	      init_iterator (&raw mut it, w, ZV! (), ZV_BYTE! (), core::ptr::null_mut (),
			     DEFAULT_FACE_ID);
	      move_it_vertically_backward (&raw mut it, height - unit);
	      /* The following move is usually a no-op when the stuff
		 displayed in the mini-window comes entirely from buffer
		 text, but it is needed when some of it comes from overlay
		 strings, especially when there's an after-string at ZV.
		 This happens with some completion packages, like
		 icomplete, ido-vertical, etc.  With those packages, if we
		 don't force w->start to be at the beginning of a screen
		 line, important parts of the stuff in the mini-window,
		 such as user prompt, will be hidden from view.  */
	      move_it_by_lines (&raw mut it, 0);
	      start = it.current.pos;
	      /* Prevent redisplay_window from recentering, and thus from
		 overriding the window-start point we computed here.  */
	      window_bits! (w, start_at_line_beg, false);
	      SET_MARKER_FROM_TEXT_POS! ((*w).start, start);
	    }
	}
      else
	{
	  SET_TEXT_POS! (start, BEGV! (), BEGV_BYTE! ());
	  SET_MARKER_FROM_TEXT_POS! ((*w).start, start);
	}

      if EQ (Vresize_mini_windows, Qgrow_only ())
	{
	  /* Let it grow only, until we display an empty message, in which
	     case the window shrinks again.  */
	  if height > old_height
	    { grow_mini_window (w, height - old_height, unit); }
	  else if height < old_height && (exact_p || BEGV! () == ZV! ())
	    { shrink_mini_window (w, unit); }
	}
      else if height != old_height
	/* Always resize to exact size needed.  */
	{ grow_mini_window (w, height - old_height, unit); }

      if !old_current_buffer.is_null ()
	{ set_buffer_internal (old_current_buffer); }
    }

  return WINDOW_BOX_TEXT_HEIGHT! (w) != old_height;
}


/* Value is the current message, a string, or nil if there is no
   current message.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn current_message () -> Lisp_Object
{
  /* C leaves MSG uninitialised here (the callback below is what stores it), which Rust has
     no equivalent for (PORTING.md 1.5).  */
  let mut msg: Lisp_Object = Qnil ();

  if !BUFFERP (echo_area_buffer[0])
    { msg = Qnil (); }
  else
    {
      with_echo_area_buffer (core::ptr::null_mut (), 0, Some (current_message_1),
			     &raw mut msg as *mut c_void, Qnil ());
      if NILP (msg)
	{ echo_area_buffer[0] = Qnil (); }
    }

  return msg;
}

unsafe extern "C" fn current_message_1 (a1: *mut c_void, a2: Lisp_Object) -> bool
{
  let msg: *mut Lisp_Object = a1 as *mut Lisp_Object;
  let _ = a2;

  if Z! () > BEG as ptrdiff_t
    { *msg = make_buffer_string (BEG as ptrdiff_t, Z! (), true); }
  else
    { *msg = Qnil (); }
  return false;
}

/* Push the current message on Vmessage_stack for later restoration
   by restore_message.  Value is true if the current message isn't
   empty.  This is a relatively infrequent operation, so it's not
   worth optimizing.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn push_message () -> bool
{
  let msg: Lisp_Object = current_message ();
  Vmessage_stack = Fcons (msg, Vmessage_stack);
  return STRINGP (msg);
}

/* Restore message display from the top of Vmessage_stack.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn restore_message ()
{
  eassert! (CONSP (Vmessage_stack));
  message3_nolog (XCAR (Vmessage_stack));
}

/* Handler for unwind-protect calling pop_message.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pop_message_unwind ()
{
  /* Pop the top-most entry off Vmessage_stack.  */
  eassert! (CONSP (Vmessage_stack));
  Vmessage_stack = XCDR (Vmessage_stack);
}

/* Check that Vmessage_stack is nil.  Called from emacs.c when Emacs
   exits.  If the stack is not empty, we have a missing
   pop_message_unwind somewhere.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_message_stack ()
{
  if !NILP (Vmessage_stack)
    { emacs_abort (); }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn clear_message_stack ()
{
  Vmessage_stack = Qnil ();
}

/* Truncate to NCHARS what will be displayed in the echo area the next
   time we display it---but don't redisplay it now.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn truncate_echo_area (nchars: ptrdiff_t)
{
  if nchars == 0
    { echo_area_buffer[0] = Qnil (); }
  else if !noninteractive
	   && INTERACTIVE! ()
	   && !NILP (echo_area_buffer[0])
    {
      let sf: *mut frame = SELECTED_FRAME ();
      /* Error messages get reported properly by cmd_error, so this must be
	 just an informative message; if the frame hasn't really been
	 initialized yet, just toss it.  */
      if frame_bits! (glyphs_initialized_p, sf)
	{
	  with_echo_area_buffer (core::ptr::null_mut (), 0, Some (truncate_message_1),
				 nchars as *mut c_void, Qnil ());
	}
    }
}

/* Helper function for truncate_echo_area.  Truncate the current
   message to at most NCHARS characters.  */

unsafe extern "C" fn truncate_message_1 (a1: *mut c_void, a2: Lisp_Object) -> bool
{
  let nchars: intptr_t = a1 as intptr_t;
  let _ = a2;
  if (BEG as ptrdiff_t + nchars as ptrdiff_t) < Z! ()
    { del_range (BEG as ptrdiff_t + nchars as ptrdiff_t, Z! ()); }
  if Z! () == BEG as ptrdiff_t
    { echo_area_buffer[0] = Qnil (); }
  return false;
}

/* Set the current message to STRING.  */

unsafe fn set_message (string: Lisp_Object)
{
  let mut message: Lisp_Object = Qnil ();
  let mut string = string;

  eassert! (STRINGP (string));

  if FUNCTIONP (Vset_message_function)
      /* FIXME: (bug#63253) We should really make the regexp engine re-entrant,
	 but in the mean time, let's ignore `set-message-function` when
	 called from `probably_quit`.  */
      && garbage_collection_inhibited == 0
    {
      let count: specpdl_ref = SPECPDL_INDEX ();
      specbind (Qinhibit_quit (), Qt ());
      message = dsafe_call1 (Vset_message_function, string);
      unbind_to (count, Qnil ());

      if STRINGP (message)
	{
	  string = message;
	  message = Qnil ();
	}
    }

  if NILP (message)
    {
      message_enable_multibyte = STRING_MULTIBYTE (string);

      with_echo_area_buffer (core::ptr::null_mut (), -1, Some (set_message_1),
			     core::ptr::null_mut (), string);
      message_buf_print = false;
      help_echo_showing_p = false;
    }

  if STRINGP (Vdebug_on_message)
      && STRINGP (string)
      && fast_string_match (Vdebug_on_message, string) >= 0
    { call_debugger (list2 (Qerror (), string)); }
}

/* Helper function for set_message.  First argument is ignored and second
   argument has the same meaning as for set_message.
   This function is called with the echo area buffer being current.  */

unsafe extern "C" fn set_message_1 (a1: *mut c_void, string: Lisp_Object) -> bool
{
  let _ = a1;
  eassert! (STRINGP (string));

  /* Change multibyteness of the echo buffer appropriately.  We always
     set it to be multibyte, except when
     unibyte-display-via-language-environment is non-nil and the
     string to display is unibyte, because in that case unibyte
     characters should not be displayed as octal escapes.  */
  if !message_enable_multibyte
      && unibyte_display_via_language_environment
      && !NILP (BVAR! (current_buffer, enable_multibyte_characters_))
    { Fset_buffer_multibyte (Qnil ()); }
  else if NILP (BVAR! (current_buffer, enable_multibyte_characters_))
    { Fset_buffer_multibyte (Qt ()); }

  bset_truncate_lines (current_buffer,
		       if message_truncate_lines { Qt () } else { Qnil () });
  if !NILP (BVAR! (current_buffer, bidi_display_reordering_))
    { bset_bidi_paragraph_direction (current_buffer, Qleft_to_right ()); }

  /* Insert new message at BEG.  */
  TEMP_SET_PT_BOTH (BEG as ptrdiff_t, BEG_BYTE as ptrdiff_t);

  /* This function takes care of single/multibyte conversion.
     We just have to ensure that the echo area buffer has the right
     setting of enable_multibyte_characters.  */
  insert_from_string (string, 0, 0, SCHARS (string), SBYTES (string), true);

  return false;
}

/* Clear messages.  CURRENT_P means clear the current message.
   LAST_DISPLAYED_P means clear the message last displayed.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn clear_message (current_p: bool, last_displayed_p: bool)
{
  let mut preserve: Lisp_Object = Qnil ();

  if current_p && !inhibit_message
    {
      if FUNCTIONP (Vclear_message_function)
	  /* FIXME: (bug#63253) Same as for `set-message-function` above.  */
	  && garbage_collection_inhibited == 0
	{
	  let count: specpdl_ref = SPECPDL_INDEX ();
	  specbind (Qinhibit_quit (), Qt ());
	  preserve = dsafe_calln! (false, Vclear_message_function);
	  unbind_to (count, Qnil ());
	}

      if !EQ (preserve, Qdont_clear_message ())
	{
	  echo_area_buffer[0] = Qnil ();
	  message_cleared_p = true;
	}
    }

  if last_displayed_p
    { echo_area_buffer[1] = Qnil (); }

  message_buf_print = false;
}


/* Clear garbaged frames.  Value is true if current matrices have been
   cleared on at least one tty frame.  This information is needed to
   determine if more than one window has to be updated on ttys, whose
   update requires building a frame matrix from window matrices.

   This function is used where the old redisplay called
   redraw_garbaged_frames which in turn called redraw_frame which in
   turn called clear_frame.  The call to clear_frame was a source of
   flickering.  I believe a clear_frame is not necessary.  It should
   suffice in the new redisplay to invalidate all current matrices,
   and ensure a complete redisplay of all windows. */

unsafe fn clear_garbaged_frames () -> bool
{
  let mut current_matrices_cleared: bool = false;
  if frame_garbaged
    {
      let mut tail: Lisp_Object = Qnil ();
      let mut frame: Lisp_Object = Qnil ();
      let sf: *mut frame = SELECTED_FRAME ();

      /* C's `FOR_EACH_FRAME (tail, frame)` walks `Vframe_list`.  */
      FOR_EACH_FRAME! (tail, frame, {
	  let f: *mut frame = XFRAME (frame);

	  if frame_redisplay_p (f) && FRAME_GARBAGED_P (f)
	    {
	      if frame_bits! (resized_p, f)
		  /* It makes no sense to redraw a non-selected TTY
		     frame, since that will actually clear the
		     selected frame, and might leave the selected
		     frame with corrupted display, if it happens not
		     to be marked garbaged.  */
		  && !(f != sf && is_tty_frame (f))
		{ redraw_frame (f); }
	      else
		{ clear_current_matrices (f); }
	      if is_tty_frame (f)
		{ current_matrices_cleared = true; }

	      /* C 13726-13730's `#ifdef HAVE_WINDOW_SYSTEM` block
		 (`FRAME_RIF (f)->clear_under_internal_border`) is dead.  */
	      fset_redisplay (f);
	      frame_bits! (garbaged, f, false);
	      frame_bits! (resized_p, f, false);
	    }
	});

      frame_garbaged = false;
    }

  return current_matrices_cleared;
}

/* Redisplay the echo area of the selected frame.  If UPDATE_FRAME_P, update
   selected_frame.  */

unsafe fn echo_area_display (update_frame_p: bool)
{
  let mut mini_window: Lisp_Object;
  let w: *mut window;
  let f: *mut frame;
  let mut window_height_changed_p: bool = false;
  let sf: *mut frame = SELECTED_FRAME ();

  mini_window = FRAME_MINIBUF_WINDOW (sf);
  if NILP (mini_window)
    { return; }

  w = XWINDOW (mini_window);
  f = XFRAME (WINDOW_FRAME! (w));

  /* Don't display if frame is invisible or not yet initialized or
     if redisplay is inhibited.  */
  if !frame_redisplay_p (f) || !frame_bits! (glyphs_initialized_p, f)
      || !NILP (Vinhibit_redisplay)
    { return; }

  /* C 13769-13775's `#ifdef HAVE_WINDOW_SYSTEM` block (the
     `FRAME_INITIAL_P (XFRAME (selected_frame))` early return) is dead.  */

  /* Redraw garbaged frames.  */
  clear_garbaged_frames ();

  if !NILP (echo_area_buffer[0]) || minibuf_level == 0
    {
      echo_area_window = mini_window;
      window_height_changed_p = display_echo_area (w);
      window_bits! (w, must_be_updated_p, true);

      /* Update the display, unless called from redisplay_internal.
	 Also don't update the screen during redisplay itself.  The
	 update will happen at the end of redisplay, and an update
	 here could cause confusion.  */
      if update_frame_p && !redisplaying_p
	{
	  if window_height_changed_p
	      /* Don't do this if Emacs is shutting down.  Redisplay
		 needs to run hooks.  */
	      && !NILP (Vrun_hooks)
	    {
	      /* Must update other windows.  Likewise as in other
		 cases, don't let this update be interrupted by
		 pending input.  */
	      fset_redisplay (f);
	      redisplay_internal ();
	    }
	  else if FRAME_WINDOW_P (f)
	    {
	      /* Window configuration is the same as before.
		 Can do with a display update of the echo area,
		 unless we displayed some mode lines.  */
	      update_single_window (w);
	      flush_frame (f);
	    }
	  else
	    {
	      update_frame (f, true);
	      if is_tty_frame (f)
		{ combine_updates_for_frame (f, true); }
	    }

	  /* If cursor is in the echo area, make sure that the next
	     redisplay displays the minibuffer, so that the cursor will
	     be replaced with what the minibuffer wants.  */
	  if cursor_in_echo_area
	    { wset_redisplay (XWINDOW (mini_window)); }
	}
    }
  else if !EQ (mini_window, selected_window)
    { wset_redisplay (XWINDOW (mini_window)); }

  /* Last displayed message is now the current message.  */
  echo_area_buffer[1] = echo_area_buffer[0];
  /* Inform read_char that we're not echoing.  */
  echo_message_buffer = Qnil ();

  /* Prevent redisplay optimization in redisplay_internal by resetting
     this_line_start_pos.  This is done because the mini-buffer now
     displays the message instead of its buffer text.  */
  if EQ (mini_window, selected_window)
    { CHARPOS! (this_line_start_pos) = 0; }

  if window_height_changed_p
    {
      fset_redisplay (f);

      /* If window configuration was changed, frames may have been
	 marked garbaged.  Clear them or we will experience
	 surprises wrt scrolling.
	 FIXME: How/why/when?  */
      clear_garbaged_frames ();
    }
}

/* True if W's buffer was changed but not saved.  */

unsafe fn window_buffer_changed (w: *mut window) -> bool
{
  let b: *mut buffer = XBUFFER ((*w).contents);

  eassert! (BUFFER_LIVE_P (b));

  return (BUF_SAVE_MODIFF! (b) < BUF_MODIFF! (b)) != window_bits! (w, last_had_star);
}

/* True if W has %c or %C in its mode line and mode line should be updated.  */

unsafe fn mode_line_update_needed (w: *mut window) -> bool
{
  return ((*w).column_number_displayed != -1
	  && !(PT! () == (*w).last_point && !window_outdated (w))
	  && ((*w).column_number_displayed != current_column ()));
}

/* True if window start of W is frozen and may not be changed during
   redisplay.  */

unsafe fn window_frozen_p (w: *mut window) -> bool
{
  if FRAME_WINDOWS_FROZEN (XFRAME (WINDOW_FRAME! (w)))
    {
      let window: Lisp_Object;

      XSETWINDOW! (window, w);
      if MINI_WINDOW_P (w)
	{ return false; }
      else if EQ (window, selected_window)
	{ return false; }
      else if MINI_WINDOW_P (XWINDOW (selected_window))
	       && EQ (window, Vminibuf_scroll_window)
	/* This special window can't be frozen too.  */
	{ return false; }
      else
	{ return true; }
    }
  return false;
}

/***********************************************************************
		     Mode Lines and Frame Titles
 ***********************************************************************/

/* A buffer for constructing non-propertized mode-line strings and
   frame titles in it; allocated from the heap in init_xdisp and
   resized as needed in store_mode_line_noprop_char.  */

static mut mode_line_noprop_buf: *mut c_char = core::ptr::null_mut ();

/* The buffer's end, and a current output position in it.  */

static mut mode_line_noprop_buf_end: *mut c_char = core::ptr::null_mut ();
static mut mode_line_noprop_ptr: *mut c_char = core::ptr::null_mut ();

/* C 13913-13914: `#define MODE_LINE_NOPROP_LEN(start) ((mode_line_noprop_ptr -
   mode_line_noprop_buf) - start)`.  */
macro_rules! MODE_LINE_NOPROP_LEN {
  ($start:expr) => { (mode_line_noprop_ptr.offset_from (mode_line_noprop_buf)
		     - ($start) as isize) };
}

/* C 13916-13921's anonymous `enum { MODE_LINE_DISPLAY = 0, MODE_LINE_TITLE,
   MODE_LINE_NOPROP, MODE_LINE_STRING } mode_line_target;` -- the port spells the enumerators
   as constants and the variable as the static below.  */
/* C: dispextern.h 3407-3431's `enum tab_bar_item_idx`; the frozen build's `TAB_BAR_ITEM_*`
   slots are positional (measured: CAPTION is the fourth enumerator, NSLOTS the seventh).  */
pub const TAB_BAR_ITEM_KEY: c_int = 0;
pub const TAB_BAR_ITEM_ENABLED_P: c_int = 1;
pub const TAB_BAR_ITEM_SELECTED_P: c_int = 2;
pub const TAB_BAR_ITEM_CAPTION: c_int = 3;
pub const TAB_BAR_ITEM_BINDING: c_int = 4;
pub const TAB_BAR_ITEM_HELP: c_int = 5;
pub const TAB_BAR_ITEM_NSLOTS: c_int = 6;

pub const MODE_LINE_DISPLAY: c_int = 0;
pub const MODE_LINE_TITLE: c_int = 1;
pub const MODE_LINE_NOPROP: c_int = 2;
pub const MODE_LINE_STRING: c_int = 3;
static mut mode_line_target: c_int = 0;

/* Alist that caches the results of :propertize.
   Each element is (PROPERTIZED-STRING . PROPERTY-LIST).  */
static mut mode_line_proptrans_alist: Lisp_Object = lisp_h_Qnil ();

/* List of strings making up the mode-line.  */
static mut mode_line_string_list: Lisp_Object = lisp_h_Qnil ();

/* Element number for the stored mode line string. */
static mut mode_line_elt_no: ptrdiff_t = 0;

/* Base face property when building propertized mode line string.  */
static mut mode_line_string_face: Lisp_Object = lisp_h_Qnil ();
static mut mode_line_string_face_prop: Lisp_Object = lisp_h_Qnil ();


/* Unwind data for mode line strings */

static mut Vmode_line_unwind_vector: Lisp_Object = lisp_h_Qnil ();

unsafe fn format_mode_line_unwind_data (target_frame: *mut frame,
					obuf: *mut buffer,
					owin: Lisp_Object,
					save_proptrans: bool) -> Lisp_Object
{
  let mut vector: Lisp_Object;
  let mut tmp: Lisp_Object;

  /* Reduce consing by keeping one vector in
     Vwith_echo_area_save_vector.  */
  vector = Vmode_line_unwind_vector;
  Vmode_line_unwind_vector = Qnil ();

  if NILP (vector)
    { vector = make_nil_vector (12); }

  ASET (vector, 0, make_fixnum (mode_line_target as EMACS_INT));
  ASET (vector, 1, make_fixnum (MODE_LINE_NOPROP_LEN! (0) as EMACS_INT));
  ASET (vector, 2, mode_line_string_list);
  ASET (vector, 3, if save_proptrans { mode_line_proptrans_alist } else { Qt () });
  ASET (vector, 4, mode_line_string_face);
  ASET (vector, 5, mode_line_string_face_prop);

  if !obuf.is_null ()
    { XSETBUFFER! (tmp, obuf); }
  else
    { tmp = Qnil (); }
  ASET (vector, 6, tmp);
  ASET (vector, 7, owin);
  if !target_frame.is_null ()
    {
      let buffer: Lisp_Object = (*(XWINDOW ((*target_frame).selected_window))).contents;
      let b: *mut buffer = XBUFFER (buffer);
      let cb: *mut buffer = current_buffer;

      /* Similarly to `with-selected-window', if the operation selects
	 a window on another frame, we must restore that frame's
	 selected window, and (for a tty) the top-frame.  */
      ASET (vector, 8, (*target_frame).selected_window);
      if FRAME_TERMCAP_P (target_frame)
	{ ASET (vector, 9, (*FRAME_TTY! (target_frame)).top_frame); }

      /* If we select a window on another frame, make sure that that
	 selection does not leave its buffer's point modified when
	 unwinding (Bug#32777).  */
      ASET (vector, 10, buffer);
      current_buffer = b;
      ASET (vector, 11, build_marker (current_buffer as *mut c_void, PT! (), PT_BYTE! ()));
      current_buffer = cb;
    }

  return vector;
}
unsafe extern "C" fn unwind_format_mode_line (vector: Lisp_Object)
{
  let old_window: Lisp_Object = AREF (vector, 7);
  let target_frame_window: Lisp_Object = AREF (vector, 8);
  let old_top_frame: Lisp_Object = AREF (vector, 9);

  mode_line_target = XFIXNUM (AREF (vector, 0)) as c_int;
  mode_line_noprop_ptr = mode_line_noprop_buf.offset (XFIXNUM (AREF (vector, 1)) as isize);
  mode_line_string_list = AREF (vector, 2);
  if ! EQ (AREF (vector, 3), Qt ())
    { mode_line_proptrans_alist = AREF (vector, 3); }
  mode_line_string_face = AREF (vector, 4);
  mode_line_string_face_prop = AREF (vector, 5);

  /* Select window before buffer, since it may change the buffer.  */
  if WINDOW_LIVE_P (old_window)
    {
      /* If the operation that we are unwinding had selected a window
	 on a different frame, reset its frame-selected-window.  For a
	 text terminal, reset its top-frame if necessary.  */
      if WINDOW_LIVE_P (target_frame_window)
	{
	  let frame: Lisp_Object
	    = WINDOW_FRAME! (XWINDOW (target_frame_window));

	  if !EQ (frame, WINDOW_FRAME! (XWINDOW (old_window)))
	    { Fselect_window (target_frame_window, Qt ()); }

	  if !NILP (old_top_frame) && !EQ (old_top_frame, frame)
	      /* This could've been destroyed during the formatting,
		 possibly because the terminal was deleted.  */
	      && FRAME_LIVE_P (XFRAME (old_top_frame))
	    { Fselect_frame (old_top_frame, Qt ()); }
	}

      Fselect_window (old_window, Qt ());

      /* Restore point of target_frame_window's buffer (Bug#32777).
	 But do this only after old_window has been reselected to
	 avoid that the window point of target_frame_window moves.  */
      if WINDOW_LIVE_P (target_frame_window)
	{
	  let buffer: Lisp_Object = AREF (vector, 10);

	  if BUFFER_LIVE_P (XBUFFER (buffer))
	    {
	      let cb: *mut buffer = current_buffer;

	      current_buffer = XBUFFER (buffer);
	      set_point_from_marker (AREF (vector, 11));
	      ASET (vector, 11, Qnil ());
	      current_buffer = cb;
	    }
	}
    }

  if !NILP (AREF (vector, 6))
    {
      set_buffer_internal_1 (XBUFFER (AREF (vector, 6)));
      ASET (vector, 6, Qnil ());
    }

  Vmode_line_unwind_vector = vector;

  if dont_resize_frames > 0
    { dont_resize_frames -= 1; }
}

/* Store a single character C for the frame title in mode_line_noprop_buf.
   Re-allocate mode_line_noprop_buf if necessary.  */

unsafe fn store_mode_line_noprop_char (c: c_char)
{
  /* If output position has reached the end of the allocated buffer,
     increase the buffer's size.  */
  if mode_line_noprop_ptr == mode_line_noprop_buf_end
    {
      let len: ptrdiff_t = MODE_LINE_NOPROP_LEN! (0);
      let mut size: ptrdiff_t = len;
      mode_line_noprop_buf =
	xpalloc (mode_line_noprop_buf as *mut c_void, &raw mut size, 1,
		 STRING_BYTES_BOUND, 1) as *mut c_char;
      mode_line_noprop_buf_end = mode_line_noprop_buf.offset (size as isize);
      mode_line_noprop_ptr = mode_line_noprop_buf.offset (len as isize);
    }

  *mode_line_noprop_ptr = c;
  mode_line_noprop_ptr = mode_line_noprop_ptr.offset (1);
}

/* Store part of a frame title in mode_line_noprop_buf, beginning at
   mode_line_noprop_ptr.  STRING is the string to store.  Do not copy
   characters that yield more columns than PRECISION; PRECISION <= 0
   means copy the whole string.  Pad with spaces until FIELD_WIDTH
   number of characters have been copied; FIELD_WIDTH <= 0 means don't
   pad.  Called from display_mode_element when it is used to build a
   frame title.  */

unsafe fn store_mode_line_noprop (string: *const c_char, field_width: c_int,
				  precision: c_int) -> c_int
{
  let mut str_: *const u8 = string as *const u8;
  let mut n: c_int = 0;
  let mut dummy: ptrdiff_t = 0;
  let mut nbytes: ptrdiff_t;

  /* Copy at most PRECISION chars from STR.  */
  nbytes = strlen (string) as ptrdiff_t;
  n += c_string_width (str_, nbytes, precision, &raw mut dummy, &raw mut nbytes) as c_int;
  /* C's `while (nbytes--)`: the post-decrement runs on every test including the one that
     ends the loop (deviation 86's rule).  */
  loop
    {
      let old: ptrdiff_t = nbytes;
      nbytes -= 1;
      if old == 0 { break; }
      store_mode_line_noprop_char (*str_ as c_char);
      str_ = str_.offset (1);
    }

  /* Fill up with spaces until FIELD_WIDTH reached.  */
  while field_width > 0
	&& n < field_width
    {
      store_mode_line_noprop_char (' ' as c_char);
      n += 1;
    }

  return n;
}

/* C 14235-14239: `#define REDISPLAY_SOME_P() ((windows_or_buffers_changed == 0 ||
   windows_or_buffers_changed == REDISPLAY_SOME) && (update_mode_lines == 0 ||
   update_mode_lines == REDISPLAY_SOME))`.  */
macro_rules! REDISPLAY_SOME_P {
  () => {
    ((windows_or_buffers_changed == 0
      || windows_or_buffers_changed == REDISPLAY_SOME)
     && (update_mode_lines == 0
	 || update_mode_lines == REDISPLAY_SOME))
  };
}

unsafe fn needs_no_redisplay (w: *mut window) -> bool
{
  let buffer: *mut buffer = XBUFFER ((*w).contents);
  let f: *mut frame = XFRAME ((*w).frame);
  return (REDISPLAY_SOME_P! ()
	  && !window_bits! (w, redisplay)
	  && !window_bits! (w, update_mode_line)
	  && !frame_bits! (face_change, f)
	  && !frame_bits! (redisplay, f)
	  && !buffer_text_bits! (text.redisplay, buffer)
	  && window_point (w) == (*w).last_point);
}


/* Prepare for redisplay by updating menu-bar item lists when
   appropriate.  This can call eval.  */

unsafe fn prepare_menu_bars ()
{
  let all_windows: bool = windows_or_buffers_changed != 0 || update_mode_lines != 0;
  let some_windows: bool = REDISPLAY_SOME_P! ();

  if FUNCTIONP (Vpre_redisplay_function)
    {
      let mut windows: Lisp_Object = if all_windows { Qt () } else { Qnil () };
      if all_windows && some_windows
	{
	  let mut ws: Lisp_Object = window_list ();
	  windows = Qnil ();
	  while CONSP (ws)
	    {
	      let this: Lisp_Object = XCAR (ws);
	      let w: *mut window = XWINDOW (this);
	      /* Cf. conditions for redisplaying a window at the
		 beginning of redisplay_window.  */
	      if !needs_no_redisplay (w)
		{ windows = Fcons (this, windows); }
	      ws = XCDR (ws);
	    }
	}
      dsafe_calln! (true, Vpre_redisplay_function, windows);
    }

  /* Update all frame titles based on their buffer names, etc.  We do
     this before the menu bars so that the buffer-menu will show the
     up-to-date frame titles.  */
  /* C 14286-14317's `#ifdef HAVE_WINDOW_SYSTEM` block (the frame-title walk that calls
     `gui_consider_frame_title`) is dead, as is the whole `gui_consider_frame_title`
     definition at C 14124-14227.  */

  /* Update the menu bar item lists, if appropriate.  This has to be
     done before any actual redisplay or generation of display lines.  */

  if all_windows
    {
      let mut tail: Lisp_Object = Qnil ();
      let mut frame: Lisp_Object = Qnil ();
      let count: specpdl_ref = SPECPDL_INDEX ();
      /* True means that update_menu_bar has run its hooks
	 so any further calls to update_menu_bar shouldn't do so again.  */
      let mut menu_bar_hooks_run: bool = false;
      let mut sw: *mut window = XWINDOW (selected_window);
      let mut sf: *mut frame = WINDOW_XFRAME! (sw);
      let mut rf: *mut frame = core::ptr::null_mut ();

      if !FRAME_PARENT_FRAME (sf).is_null () && !FRAME_WINDOW_P (sf)
	  && FRAME_MENU_BAR_LINES (sf) == 0
	  && FRAME_MENU_BAR_LINES ({ rf = root_frame (sf); rf }) != 0
	  && NILP (Fdefault_value (Qtty_menu_open_use_tmm ()))
	/* If the selected window's frame is a tty child frame without
	   menu bar, that frame's root frame has a menu bar and
	   'tty-menu-open-use-tmm' is nil, update the menu bar of the
	   root frame from the selected window.  */
	{ sf = rf; }
      else
	{
	  sf = core::ptr::null_mut ();
	  sw = core::ptr::null_mut ();
	}

      record_unwind_save_match_data ();

      FOR_EACH_FRAME! (tail, frame, {
	  let f: *mut frame = XFRAME (frame);
	  let w: *mut window
	    = if sf == f { sw } else { XWINDOW (FRAME_SELECTED_WINDOW (f)) };

	  /* Ignore tooltip frame.  */
	  if FRAME_TOOLTIP_P (f)
	    { tail = XCDR (tail); continue; }

	  if some_windows
	      && !frame_bits! (redisplay, f)
	      && !window_bits! (w, redisplay)
	      && !buffer_text_bits! (text.redisplay, XBUFFER ((*w).contents))
	    { tail = XCDR (tail); continue; }

	  menu_bar_hooks_run
	    = update_menu_bar (f, false, menu_bar_hooks_run, w);

	  update_tab_bar (f, false);
	  /* C 14370-14372's `#ifdef HAVE_WINDOW_SYSTEM` block (`update_tool_bar (f, false)`)
	     is dead.  */
	});

      unbind_to (count, Qnil ());
    }
  else
    {
      let sw: *mut window = XWINDOW (selected_window);
      let mut sf: *mut frame = WINDOW_XFRAME! (sw);
      let mut rf: *mut frame = core::ptr::null_mut ();

      if !FRAME_PARENT_FRAME (sf).is_null () && !FRAME_WINDOW_P (sf)
	  && FRAME_MENU_BAR_LINES (sf) == 0
	  && FRAME_MENU_BAR_LINES ({ rf = root_frame (sf); rf }) != 0
	  && NILP (Fdefault_value (Qtty_menu_open_use_tmm ()))
	/* If the selected window's frame is a tty child frame without
	   menu bar, that frame's root frame has a menu bar and
	   'tty-menu-open-use-tmm' is nil, update the menu bar of the
	   root frame from the selected window.  */
	{ sf = rf; }

      update_menu_bar (sf, true, false, sw);

      update_tab_bar (sf, true);
      /* C 14396-14398's `#ifdef HAVE_WINDOW_SYSTEM` block (`update_tool_bar (sf, true)`)
	 is dead.  */
    }
}

/* Update the menu bar item list for frame F.  This has to be done
   before we start to fill in any display lines, because it can call
   eval.

   If SAVE_MATCH_DATA, we must save and restore it here.

   If HOOKS_RUN, a previous call to update_menu_bar
   already ran the menu bar hooks for this redisplay, so there
   is no need to run them again.  The return value is the
   updated value of this flag, to pass to the next call.

   W, if set, denotes the window that should be considered as selected.
   For a tty child frame using F as surrogate menu bar frame, this
   specifies the child frame's selected window and its buffer shall be
   used for updating the menu bar of the root frame instead of the
   buffer of the root frame's selected window.  */

unsafe fn update_menu_bar (f: *mut frame, save_match_data: bool, mut hooks_run: bool,
			   w: *mut window) -> bool
{
  /* If called recursively during a menu update, do nothing.  This can
     happen when, for instance, an activate-menubar-hook causes a
     redisplay.  */
  if inhibit_menubar_update
    { return hooks_run; }

  /* C 14431-14435's `#ifdef HAVE_EXT_MENU_BAR` arm is dead (the manifest has no such macro),
     so C's ternary compiles as the `#else`: both arms test `FRAME_MENU_BAR_LINES`.  */
  if if FRAME_WINDOW_P (f)
     { FRAME_MENU_BAR_LINES (f) > 0 }
     else { FRAME_MENU_BAR_LINES (f) > 0 }
    {
      /* If the user has switched buffers or windows, we need to
	 recompute to reflect the new bindings.  But we'll
	 recompute when update_mode_lines is set too; that means
	 that people can use force-mode-line-update to request
	 that the menu bar be recomputed.  The adverse effect on
	 the rest of the redisplay algorithm is about the same as
	 windows_or_buffers_changed anyway.  */
      if windows_or_buffers_changed != 0
	  /* This used to test w->update_mode_line, but we believe
	     there is no need to recompute the menu in that case.  */
	  || update_mode_lines != 0
	  || window_buffer_changed (w)
	{
	  let prev: *mut buffer = current_buffer;
	  let count: specpdl_ref = SPECPDL_INDEX ();

	  specbind (Qinhibit_menubar_update (), Qt ());

	  set_buffer_internal_1 (XBUFFER ((*w).contents));
	  if save_match_data
	    { record_unwind_save_match_data (); }
	  if NILP (Voverriding_local_map_menu_flag)
	    {
	      specbind (Qoverriding_terminal_local_map (), Qnil ());
	      specbind (Qoverriding_local_map (), Qnil ());
	    }

	  if !hooks_run
	    {
	      /* Run the Lucid hook.  */
	      safe_run_hooks (Qactivate_menubar_hook ());

	      /* If it has changed current-menubar from previous value,
		 really recompute the menu-bar from the value.  */
	      safe_run_hooks (Qmenu_bar_update_hook ());

	      hooks_run = true;
	    }

	  XSETFRAME! (Vmenu_updating_frame, f);
	  fset_menu_bar_items (f, menu_bar_items (FRAME_MENU_BAR_ITEMS (f)));

	  /* Redisplay the menu bar in case we changed it.  */
	  /* C 14481-14499's `#ifdef HAVE_EXT_MENU_BAR` block is dead; the live arm is its
	     `#else`: the menu bar is an ordinary screen line on this target.  */
	  window_bits! (w, update_mode_line, true);

	  unbind_to (count, Qnil ());
	  set_buffer_internal_1 (prev);
	}
    }

  return hooks_run;
}


/* Restore WINDOW as the selected window and its frame as the selected
   frame.  If WINDOW is dead but the selected frame is live, make the
   latter's selected window the selected window.  If both, WINDOW and
   the selected frame, are dead, assign selected frame and window from
   some arbitrary live frame.  Abort if no such frame can be found.  */
unsafe extern "C" fn restore_selected_window (window: Lisp_Object)
{
  if WINDOW_LIVE_P (window)
    /* If WINDOW is live, make it the selected window and its frame's
       selected window and set the selected frame to its frame.  */
    {
      selected_window = window;
      selected_frame = (*XWINDOW (window)).frame;
      /* C's `FRAME_SELECTED_WINDOW (f)` is `(f)->selected_window`, an l-value here; the
	 port spells the store out (the `FRAME_WINDOW_CHANGE` note's shape).  */
      FRAME_SELECTED_WINDOW! (XFRAME (selected_frame)) = window;
    }
  else if FRAMEP (selected_frame) && FRAME_LIVE_P (XFRAME (selected_frame))
    /* If WINDOW is dead but the selected frame is still live, make the
       latter's selected window the selected one.  */
    { selected_window = FRAME_SELECTED_WINDOW (XFRAME (selected_frame)); }
  else
    /* If WINDOW and the selected frame are dead, choose some live,
       non-child and non-tooltip frame as the new selected frame and
       make its selected window the selected window.  */
    {
      let mut tail: Lisp_Object = Qnil ();
      /* C's `Lisp_Object frame UNINIT` (PORTING.md 1.5, deviation 1's rule).  */
      let mut frame: Lisp_Object = Qnil ();

      FOR_EACH_FRAME! (tail, frame, {
	  let f: *mut frame = XFRAME (frame);

	  if FRAME_PARENT_FRAME (f).is_null () && !FRAME_TOOLTIP_P (f)
	    {
	      selected_frame = frame;
	      selected_window = FRAME_SELECTED_WINDOW (f);

	      return;
	    }
	});

      /* Abort if we cannot find a live frame.  */
      emacs_abort ();
    }
}

/* Restore WINDOW, if live, as its frame's selected window.  */
unsafe extern "C" fn restore_frame_selected_window (window: Lisp_Object)
{
  if WINDOW_LIVE_P (window)
    /* If WINDOW is live, make it its frame's selected window.  If that
       frame is the selected frame, make WINDOW the selected window as
       well.  */
    {
      let frame: Lisp_Object = (*XWINDOW (window)).frame;

      FRAME_SELECTED_WINDOW! (XFRAME (frame)) = window;
      if EQ (frame, selected_frame)
	{ selected_window = window; }
    }
}

/* Update the tab-bar item list for frame F.  This has to be done
   before we start to fill in any display lines.  Called from
   prepare_menu_bars.  If SAVE_MATCH_DATA, we must save
   and restore it here.  */

unsafe fn update_tab_bar (f: *mut frame, save_match_data: bool)
{
  let mut do_update: bool = false;

  /* C 14588-14594's `#ifdef HAVE_WINDOW_SYSTEM` arm is dead, so the live test is its `else`
     (the tty tab-bar line count).  */
  if FRAME_TAB_BAR_LINES (f) > 0
    { do_update = true; }

  if do_update
    {
      let window: Lisp_Object;
      let w: *mut window;

      window = FRAME_SELECTED_WINDOW (f);
      w = XWINDOW (window);

      /* If the user has switched buffers or windows, we need to
	 recompute to reflect the new bindings.  But we'll
	 recompute when update_mode_lines is set too; that means
	 that people can use force-mode-line-update to request
	 that the menu bar be recomputed.  The adverse effect on
	 the rest of the redisplay algorithm is about the same as
	 windows_or_buffers_changed anyway.  */
      if windows_or_buffers_changed != 0
	  || window_bits! (w, update_mode_line)
	  || update_mode_lines != 0
	  || window_buffer_changed (w)
	{
	  let prev: *mut buffer = current_buffer;
	  let count: specpdl_ref = SPECPDL_INDEX ();
	  let new_tab_bar: Lisp_Object;
	  let mut new_n_tab_bar: c_int = 0;

	  /* Set current_buffer to the buffer of the selected
	     window of the frame, so that we get the right local
	     keymaps.  */
	  set_buffer_internal_1 (XBUFFER ((*w).contents));

	  /* Save match data, if we must.  */
	  if save_match_data
	    { record_unwind_save_match_data (); }

	  /* Make sure that we don't accidentally use bogus keymaps.  */
	  if NILP (Voverriding_local_map_menu_flag)
	    {
	      specbind (Qoverriding_terminal_local_map (), Qnil ());
	      specbind (Qoverriding_local_map (), Qnil ());
	    }

	  /* We must temporarily set the selected frame to this frame
	     before calling tab_bar_items, because the calculation of
	     the tab-bar keymap uses the selected frame (see
	     `tab-bar-make-keymap' in tab-bar.el).  */
	  eassert! (EQ (selected_window,
			/* Since we only explicitly preserve selected_frame,
			   check that selected_window would be redundant.  */
			(*XFRAME (selected_frame)).selected_window));

	  let frame: Lisp_Object;
	  record_unwind_protect (Some (restore_selected_window), selected_window);
	  XSETFRAME! (frame, f);
	  selected_frame = frame;
	  selected_window = FRAME_SELECTED_WINDOW (f);

	  /* Build desired tab-bar items from keymaps.  */
	  new_tab_bar
	    = tab_bar_items (Fcopy_sequence ((*f).tab_bar_items),
			     &raw mut new_n_tab_bar);

	  /* Redisplay the tab-bar if we changed it.  */
	  if new_n_tab_bar != (*f).n_tab_bar_items
	      /* Some features modify the appearance of the tab bar by
		 manipulating the text properties.  */
	      || NILP (Fequal_including_properties (new_tab_bar,
						    (*f).tab_bar_items))
	    {
	      /* Redisplay that happens asynchronously due to an expose event
		 may access f->tab_bar_items.  Make sure we update both
		 variables within BLOCK_INPUT so no such event interrupts.  */
	      block_input ();
	      fset_tab_bar_items (f, new_tab_bar);
	      (*f).n_tab_bar_items = new_n_tab_bar;
	      window_bits! (w, update_mode_line, true);
	      unblock_input ();
	    }

	  unbind_to (count, Qnil ());
	  set_buffer_internal_1 (prev);
	}
    }
}

/* Redisplay the tab bar in the frame for window W.

   The tab bar of X frames that don't have X toolkit support is
   displayed in a special window W->frame->tab_bar_window.

   The tab bar of terminal frames is treated specially as far as
   glyph matrices are concerned.  Tab bar lines are not part of
   windows, so the update is done directly on the frame matrix rows
   for the tab bar.  */

unsafe fn display_tab_bar (w: *mut window)
{
  let f: *mut frame = XFRAME (WINDOW_FRAME! (w));
  let mut it: it = core::mem::zeroed ();
  let items: Lisp_Object;
  let mut i: c_int;

  /* Don't do all this for graphical frames.  */
  /* C 14701-14735's `#ifdef HAVE_NTGUI`, `#if defined (USE_X_TOOLKIT) || defined (USE_GTK)`,
     `#ifdef HAVE_NS` and `#elif defined (HAVE_X_WINDOWS)` arms are all dead on this target;
     what follows is C's trailing `else` body (the TTY case).  */

  /* This is a TTY frame, i.e. character hpos/vpos are used as
     pixel x/y.  */
  init_iterator (&raw mut it, w, -1, -1,
		 (*((*f).desired_matrix as *mut glyph_matrix)).rows
		 .offset ((if FRAME_MENU_BAR_LINES (f) > 0 { 1 } else { 0 }) as isize),
		 TAB_BAR_FACE_ID);
  it.first_visible_x = 0;
  it.last_visible_x = FRAME_COLS (f);

  /* FIXME: This should be controlled by a user option.  See the
     comments in redisplay_tool_bar and display_mode_line about
     this.  */
  it.paragraph_embedding = L2R;

  /* Clear all rows of the tab bar.  */
  i = 0;
  while i < FRAME_TAB_BAR_LINES (f)
    {
      let row: *mut glyph_row = it.glyph_row.offset (i as isize);
      clear_glyph_row (row);
      glyph_row_bits! (enabled_p, row, true);
      glyph_row_bits! (full_width_p, row, true);
      glyph_row_bits! (reversed_p, row, false);
      i += 1;
    }

  /* Display all items of the tab bar.  */
  items = (*it.f).tab_bar_items;
  let mut j: c_int = 0;
  i = 0;
  while i < (*it.f).n_tab_bar_items
    {
      let string: Lisp_Object = AREF (items, (j + TAB_BAR_ITEM_CAPTION) as isize);

      /* Stop at nil string.  */
      if NILP (string)
	{ break; }

      if it.current_x < it.last_visible_x
	{
	  display_string (core::ptr::null (), string, Qnil (), 0, 0, &raw mut it,
			  SCHARS (string) as c_int, 0, 0, STRING_MULTIBYTE (string) as c_int);
	}
      i += 1;
      j += TAB_BAR_ITEM_NSLOTS;
    }

  /* Fill out the line with spaces.  */
  if it.current_x < it.last_visible_x
    {
      display_string (c"".as_ptr (), Qnil (), Qnil (), 0, 0, &raw mut it, -1, 0, 0, -1);
    }

  /* Compute the total height of the lines.  */
  compute_line_metrics (&raw mut it);
}
/* Find the tab-bar item at X coordinate and return its information.  */
unsafe fn tty_get_tab_bar_item (f: *mut frame, x: c_int, prop_idx: *mut c_int,
				close_p: *mut bool) -> Lisp_Object
{
  let mut clen: ptrdiff_t = 0;

  let mut i: c_int = 0;
  while i < (*f).n_tab_bar_items
    {
      let caption: Lisp_Object = AREF ((*f).tab_bar_items, (i * TAB_BAR_ITEM_NSLOTS
							   + TAB_BAR_ITEM_CAPTION) as isize);
      if NILP (caption)
	{ return Qnil (); }
      clen += SCHARS (caption);
      if (x as ptrdiff_t) < clen
	{
	  *prop_idx = i;
	  *close_p = !NILP (Fget_text_property (make_fixnum ((SCHARS (caption)
							      - (clen - x as ptrdiff_t))
							     as EMACS_INT),
						Qclose_tab (),
						caption));
	  return caption;
	}
      i += 1;
    }
  return Qnil ();
}

/* `tty_handle_tab_bar_click` (C 15615-15654) is NOT defined here: `include/termchar.rs`
   already carries termchar.h 238-239's `extern "C"` declaration of it, and a Rust
   definition beside that declaration is E0428 (the `set_buffer_internal_1` family, the
   ledger's integrator item 6).  The definition has to live wherever that declaration is
   resolved — reported to the integrator.  */

/* For all leaf windows in the window tree rooted at WINDOW, set their
   hscroll value so that PT is (i) visible in the window, and (ii) so
   that it is not within a certain margin at the window's left and
   right border.  Value is true if any window's hscroll has been
   changed.  */

unsafe fn hscroll_window_tree (window: Lisp_Object) -> bool
{
  let mut hscrolled_p: bool = false;
  let mut hscroll_relative_p: bool = FLOATP (Vhscroll_step);
  let mut hscroll_step_abs: c_int = 0;
  let mut hscroll_step_rel: f64 = 0.0;
  let mut window = window;

  if hscroll_relative_p
    {
      hscroll_step_rel = XFLOAT_DATA (Vhscroll_step);
      if hscroll_step_rel < 0.0
	{
	  hscroll_relative_p = false;
	  hscroll_step_abs = 0;
	}
    }
  else if TYPE_RANGED_FIXNUMP (INT_MIN as EMACS_INT, Vhscroll_step, INT_MAX as EMACS_INT)
    {
      hscroll_step_abs = XFIXNUM (Vhscroll_step) as c_int;
      if hscroll_step_abs < 0
	{ hscroll_step_abs = 0; }
    }
  else
    { hscroll_step_abs = 0; }

  while WINDOWP (window)
    {
      let w: *mut window = XWINDOW (window);

      if WINDOWP ((*w).contents)
	{ hscrolled_p |= hscroll_window_tree ((*w).contents); }
      else if (*w).cursor.vpos >= 0
	       /* Don't allow hscroll in mini-windows that display
		  echo-area messages.  This is because desired_matrix
		  of such windows was prepared while momentarily
		  switched to an echo-area buffer, which is different
		  from w->contents, and we simply cannot hscroll such
		  windows safely.  */
	       && !(w == XWINDOW (echo_area_window)
		    && !NILP (echo_area_buffer[0]))
	{
	  let mut h_margin: c_int;
	  let text_area_width: c_int;
	  let mut cursor_row: *mut glyph_row;
	  let mut bottom_row: *mut glyph_row;

	  bottom_row = MATRIX_BOTTOM_TEXT_ROW! ((*w).desired_matrix as *mut glyph_matrix, w);
	  if (*w).cursor.vpos < bottom_row.offset_from (
		(*((*w).desired_matrix as *mut glyph_matrix)).rows) as c_int
	    { cursor_row = MATRIX_ROW! ((*w).desired_matrix as *mut glyph_matrix,
					(*w).cursor.vpos); }
	  else
	    { cursor_row = bottom_row.offset (-1); }

	  if !glyph_row_bits! (enabled_p, cursor_row)
	    {
	      bottom_row = MATRIX_BOTTOM_TEXT_ROW! ((*w).current_matrix as *mut glyph_matrix, w);
	      if (*w).cursor.vpos < bottom_row.offset_from (
		    (*((*w).current_matrix as *mut glyph_matrix)).rows) as c_int
		{ cursor_row = MATRIX_ROW! ((*w).current_matrix as *mut glyph_matrix,
					    (*w).cursor.vpos); }
	      else
		{ cursor_row = bottom_row.offset (-1); }
	    }
	  let row_r2l_p: bool = glyph_row_bits! (reversed_p, cursor_row);
	  let hscl: bool = hscrolling_current_line_p (w);
	  let mut x_offset: c_int = 0;
	  /* When line numbers are displayed, we need to account for
	     the horizontal space they consume.  */
	  if !NILP (Vdisplay_line_numbers)
	    {
	      let mut g: *mut glyph;
	      if !row_r2l_p
		{
		  g = (*cursor_row).glyphs[TEXT_AREA as usize];
		  while g < (*cursor_row).glyphs[TEXT_AREA as usize]
			.offset ((*cursor_row).used[TEXT_AREA as usize] as isize)
		    {
		      if !(NILP ((*g).object) && (*g).charpos < 0)
			{ break; }
		      x_offset += (*g).pixel_width as c_int;
		      g = g.offset (1);
		    }
		}
	      else
		{
		  g = (*cursor_row).glyphs[TEXT_AREA as usize]
		      .offset ((*cursor_row).used[TEXT_AREA as usize] as isize);
		  while g > (*cursor_row).glyphs[TEXT_AREA as usize]
		    {
		      if !(NILP ((*g.offset (-1)).object)
			   && (*g.offset (-1)).charpos < 0)
			{ break; }
		      x_offset += (*g.offset (-1)).pixel_width as c_int;
		      g = g.offset (-1);
		    }
		}
	    }
	  if glyph_row_bits! (truncated_on_left_p, cursor_row)
	    /* On TTY frames, don't count the left truncation glyph.  */
	    { x_offset -= is_tty_frame (XFRAME (WINDOW_FRAME! (w))) as c_int; }

	  text_area_width = window_box_width (w, TEXT_AREA);

	  /* Scroll when cursor is inside this scroll margin.  */
	  h_margin = (clip_to_bounds (0, hscroll_margin as isize, 1000000) as c_int
		      * WINDOW_FRAME_COLUMN_WIDTH! (w));

	  /* If the position of this window's point has explicitly
	     changed, no more suspend auto hscrolling.  */
	  if window_bits! (w, suspend_auto_hscroll)
	      && NILP (Fequal (Fwindow_point (window),
			       Fwindow_old_point (window)))
	    {
	      window_bits! (w, suspend_auto_hscroll, false);
	      /* When hscrolling just the current line, and the rest
		 of lines were temporarily hscrolled, but no longer
		 are, force thorough redisplay of this window, to show
		 the effect of disabling hscroll suspension immediately.  */
	      if (*w).min_hscroll == 0 && (*w).hscroll > 0
		  && EQ (Fbuffer_local_value (Qauto_hscroll_mode (), (*w).contents),
			 Qcurrent_line ())
		{ SET_FRAME_GARBAGED! (XFRAME ((*w).frame)); }
	    }

	  /* Remember window point.  */
	  Fset_marker ((*w).old_pointm,
		       if w == XWINDOW (selected_window)
		       { make_fixnum (BUF_PT (XBUFFER ((*w).contents))) }
		       else { Fmarker_position ((*w).pointm) },
		       (*w).contents);

	  if !NILP (Fbuffer_local_value (Qauto_hscroll_mode (), (*w).contents))
	      && !window_bits! (w, suspend_auto_hscroll)
	      /* In some pathological cases, like restoring a window
		 configuration into a frame that is much smaller than
		 the one from which the configuration was saved, we
		 get glyph rows whose start and end have zero buffer
		 positions, which we cannot handle below.  Just skip
		 such windows.  */
	      && (CHARPOS! ((*cursor_row).start.pos)
		  >= BUF_BEG (XBUFFER ((*w).contents)))
	      /* For left-to-right rows, hscroll when cursor is either
		 (i) inside the right hscroll margin, or (ii) if it is
		 inside the left margin and the window is already
		 hscrolled.  */
	      && ((!row_r2l_p
		   && (((*w).hscroll != 0 && (*w).cursor.x <= h_margin + x_offset)
		       || (glyph_row_bits! (enabled_p, cursor_row)
			   && glyph_row_bits! (truncated_on_right_p, cursor_row)
			   && ((*w).cursor.x >= text_area_width - h_margin))))
		  /* For right-to-left rows, the logic is similar,
		     except that rules for scrolling to left and right
		     are reversed.  E.g., if cursor.x <= h_margin, we
		     need to hscroll "to the right" unconditionally,
		     and that will scroll the screen to the left so as
		     to reveal the next portion of the row.  */
		  || (row_r2l_p
		      && ((glyph_row_bits! (enabled_p, cursor_row)
			   /* FIXME: It is confusing to set the
			      truncated_on_right_p flag when R2L rows
			      are actually truncated on the left.  */
			   && glyph_row_bits! (truncated_on_right_p, cursor_row)
			   && (*w).cursor.x <= h_margin)
			  || ((*w).hscroll != 0
			      && ((*w).cursor.x >= (text_area_width - h_margin
						    - x_offset)))))
		  /* This last condition is needed when moving
		     vertically from an hscrolled line to a short line
		     that doesn't need to be hscrolled.  If we omit
		     this condition, the line from which we move will
		     remain hscrolled.  */
		  || (hscl
		      && (*w).hscroll != (*w).min_hscroll
		      && !glyph_row_bits! (truncated_on_left_p, cursor_row)))
	    {
	      let mut it: it = core::mem::zeroed ();
	      let mut hscroll: ptrdiff_t;
	      let saved_current_buffer: *mut buffer;
	      let mut pt: ptrdiff_t;
	      let mut wanted_x: c_int;

	      /* Find point in a display of infinite width.  */
	      saved_current_buffer = current_buffer;
	      current_buffer = XBUFFER ((*w).contents);

	      if w == XWINDOW (selected_window)
		{ pt = PT! (); }
	      else
		{ pt = clip_to_bounds (BEGV! (), marker_position ((*w).pointm), ZV! ()); }

	      /* Move iterator to pt starting at cursor_row->start in
		 a line with infinite width.  */
	      init_to_row_start (&raw mut it, w, cursor_row);
	      if hscl
		{
		  it.first_visible_x = window_hscroll_limited (w, it.f)
				       * FRAME_COLUMN_WIDTH (it.f);
		}
	      it.last_visible_x = DISP_INFINITY;

	      let nchars: ptrdiff_t = pt - IT_CHARPOS! (it);
	      if buffer_bits! (long_line_optimizations_p, current_buffer)
		  && nchars > large_hscroll_threshold
		{
		  /* Special optimization for very long and truncated
		     lines which need to be hscrolled far to the left:
		     jump directly to the (approximate) first position
		     that is visible, instead of slowly walking there.  */
		  fast_move_it_horizontally (&raw mut it, nchars);
		  it.current_x += nchars as c_int * FRAME_COLUMN_WIDTH (it.f);
		}
	      else
		{ move_it_in_display_line_to (&raw mut it, pt, -1, MOVE_TO_POS); }
	      /* If the line ends in an overlay string with a newline,
		 we might infloop, because displaying the window will
		 want to put the cursor after the overlay, i.e. at X
		 coordinate of zero on the next screen line.  So we
		 use the buffer position prior to the overlay string
		 instead.  */
	      if it.method == GET_FROM_STRING && pt > 1
		{
		  init_to_row_start (&raw mut it, w, cursor_row);
		  if hscl
		    {
		      it.first_visible_x = (window_hscroll_limited (w, it.f)
					    * FRAME_COLUMN_WIDTH (it.f));
		    }
		  if buffer_bits! (long_line_optimizations_p, current_buffer)
		      && nchars > large_hscroll_threshold
		    {
		      fast_move_it_horizontally (&raw mut it, nchars - 1);
		      it.current_x += (nchars - 1) as c_int * FRAME_COLUMN_WIDTH (it.f);
		    }
		  else
		    { move_it_in_display_line_to (&raw mut it, pt - 1, -1, MOVE_TO_POS); }
		}
	      current_buffer = saved_current_buffer;

	      /* Position cursor in window.  */
	      if !hscroll_relative_p && hscroll_step_abs == 0
		{
		  hscroll = max (0, (it.current_x
				     - (if ITERATOR_AT_END_OF_LINE_P! (&raw mut it)
					{ text_area_width - 4 * FRAME_COLUMN_WIDTH (it.f) }
					else { text_area_width / 2 })) as isize) as ptrdiff_t
			    / FRAME_COLUMN_WIDTH (it.f) as ptrdiff_t;
		}
	      else if (!row_r2l_p
			&& (*w).cursor.x >= text_area_width - h_margin)
		       || (row_r2l_p && (*w).cursor.x <= h_margin)
		{
		  if hscroll_relative_p
		    {
		      wanted_x = (text_area_width as f64 * (1.0 - hscroll_step_rel)
				  - h_margin as f64) as c_int;
		    }
		  else
		    {
		      wanted_x = text_area_width
			       - hscroll_step_abs * FRAME_COLUMN_WIDTH (it.f)
			       - h_margin;
		    }
		  hscroll
		    = max (0, (it.current_x - wanted_x) as isize) as ptrdiff_t
		      / FRAME_COLUMN_WIDTH (it.f) as ptrdiff_t;
		}
	      else
		{
		  if hscroll_relative_p
		    {
		      wanted_x =
			(text_area_width as f64 * hscroll_step_rel) as c_int
			+ h_margin + x_offset;
		    }
		  else
		    {
		      wanted_x =
			hscroll_step_abs * FRAME_COLUMN_WIDTH (it.f)
			+ h_margin + x_offset;
		    }
		  hscroll
		    = max (0, (it.current_x - wanted_x) as isize) as ptrdiff_t
		      / FRAME_COLUMN_WIDTH (it.f) as ptrdiff_t;
		}
	      hscroll = max (hscroll as isize, (*w).min_hscroll as isize) as ptrdiff_t;

	      /* Don't prevent redisplay optimizations if hscroll
		 hasn't changed, as it will unnecessarily slow down
		 redisplay.  */
	      if (*w).hscroll != hscroll
		  /* When hscrolling only the current line, we need to
		     report hscroll even if its value is equal to the
		     previous one, because the new line might need a
		     different value.  */
		  || (hscl && (*w).last_cursor_vpos != (*w).cursor.vpos)
		{
		  let b: *mut buffer = XBUFFER ((*w).contents);
		  buffer_bits! (prevent_redisplay_optimizations_p, b, true);
		  (*w).hscroll = hscroll;
		  hscrolled_p = true;
		}
	    }
	}

      window = (*w).next;
    }

  /* Value is true if hscroll of any leaf window has been changed.  */
  return hscrolled_p;
}

/* Set hscroll so that cursor is visible and not inside horizontal
   scroll margins for all windows in the tree rooted at WINDOW.  See
   also hscroll_window_tree above.  Value is true if any window's
   hscroll has been changed.  If it has, desired matrices on the frame
   of WINDOW are cleared.  */

unsafe fn hscroll_windows (window: Lisp_Object) -> bool
{
  let hscrolled_p: bool = hscroll_window_tree (window);
  if hscrolled_p
    { clear_desired_matrices (XFRAME (WINDOW_FRAME! (XWINDOW (window)))); }
  return hscrolled_p;
}
unsafe fn
text_outside_line_unchanged_p (w: *mut window,
			       start: ptrdiff_t, end: ptrdiff_t) -> bool
{
  let mut unchanged_p: bool = true;

  /* If text or overlays have changed, see where.  */
  if window_outdated (w)
    {
      /* Gap in the line?  */
      if GPT! () < start || Z! () - GPT! () < end
	{ unchanged_p = false; }

      /* Changes start in front of the line, or end after it?  */
      if unchanged_p
	  && (BEG_UNCHANGED! () < start - 1
	      || END_UNCHANGED! () < end)
	{ unchanged_p = false; }

      /* If selective display, can't optimize if changes start at the
	 beginning of the line.  */
      if unchanged_p
	  && FIXNUMP (BVAR! (current_buffer, selective_display_))
	  && XFIXNUM (BVAR! (current_buffer, selective_display_)) > 0
	  && (BEG_UNCHANGED! () < start || GPT! () <= start)
	{ unchanged_p = false; }

      /* If there are overlays at the start or end of the line, these
	 may have overlay strings with newlines in them.  A change at
	 START, for instance, may actually concern the display of such
	 overlay strings as well, and they are displayed on different
	 lines.  So, quickly rule out this case.  (For the future, it
	 might be desirable to implement something more telling than
	 just BEG/END_UNCHANGED.)  */
      if unchanged_p
	{
	  if BEG as ptrdiff_t + BEG_UNCHANGED! () == start
	      && overlay_touches_p (start)
	    { unchanged_p = false; }
	  if END_UNCHANGED! () == end
	      && overlay_touches_p (Z! () - end)
	    { unchanged_p = false; }
	}

      /* Under bidi reordering, adding or deleting a character in the
	 beginning of a paragraph, before the first strong directional
	 character, can change the base direction of the paragraph (unless
	 the buffer specifies a fixed paragraph direction), which will
	 require redisplaying the whole paragraph.  It might be worthwhile
	 to find the paragraph limits and widen the range of redisplayed
	 lines to that, but for now just give up this optimization.  */
      if !NILP (BVAR! (XBUFFER ((*w).contents), bidi_display_reordering_))
	  && NILP (BVAR! (XBUFFER ((*w).contents), bidi_paragraph_direction_))
	{ unchanged_p = false; }
    }

  return unchanged_p;
}


/* Do a frame update, taking possible shortcuts into account.  This is
   the main external entry point for redisplay.

   If the last redisplay displayed an echo area message and that message
   is no longer requested, we clear the echo area or bring back the
   mini-buffer if that is in use.  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn
redisplay ()
{
  redisplay_internal ();
}


unsafe fn
overlay_arrow_string_or_property (var: Lisp_Object) -> Lisp_Object
{
  let mut val: Lisp_Object;

  if { val = Fget (var, Qoverlay_arrow_string ()); STRINGP (val) }
    { return val; }

  return Voverlay_arrow_string;
}

/* Return true if there are any overlay-arrows in current_buffer.  */
unsafe fn
overlay_arrow_in_current_buffer_p () -> bool
{
  let mut vlist: Lisp_Object;

  vlist = Voverlay_arrow_variable_list;
  while CONSP (vlist)
    {
      let var: Lisp_Object = XCAR (vlist);
      let val: Lisp_Object;

      'body:
	{
	  if !SYMBOLP (var)
	    { break 'body; }
	  val = find_symbol_value (var);
	  if MARKERP (val)
	      && (*XMARKER (val)).buffer as *mut buffer == current_buffer
	    { return true; }
	}
      vlist = XCDR (vlist);
    }
  return false;
}


/* Return true if any overlay_arrows have moved or overlay-arrow-string
   has changed.
   If SET_REDISPLAY is true, additionally, set the `redisplay' bit in those
   buffers that are affected.  */

unsafe fn
overlay_arrows_changed_p (set_redisplay: bool) -> bool
{
  let mut vlist: Lisp_Object;
  let mut changed: bool = false;

  vlist = Voverlay_arrow_variable_list;
  while CONSP (vlist)
    {
      let var: Lisp_Object = XCAR (vlist);
      let mut val: Lisp_Object;
      let mut pstr: Lisp_Object;

      'body:
	{
	  if !SYMBOLP (var)
	    { break 'body; }
	  val = find_symbol_value (var);
	  if !MARKERP (val)
	    { break 'body; }
	  if !EQ (Fmarker_position (val),
                /* FIXME: Don't we have a problem, using such a global
                 * "last-position" if the variable is buffer-local?  */
		  Fget (var, Qlast_arrow_position ()))
	  || ! { pstr = overlay_arrow_string_or_property (var);
		 EQ (pstr, Fget (var, Qlast_arrow_string ())) }
	{
	  let buf: *mut buffer = (*XMARKER (val)).buffer as *mut buffer;

	  if set_redisplay
            {
              if !buf.is_null ()
	        { bset_redisplay (buf); }
              changed = true;
            }
	  else
	    { return true; }
	}
	}
      vlist = XCDR (vlist);
    }
  return changed;
}

/* Mark overlay arrows to be updated on next redisplay.  */

unsafe fn
update_overlay_arrows (up_to_date: c_int)
{
  let mut vlist: Lisp_Object;

  vlist = Voverlay_arrow_variable_list;
  while CONSP (vlist)
    {
      let var: Lisp_Object = XCAR (vlist);

      'body:
	{
	  if !SYMBOLP (var)
	    { break 'body; }

      if up_to_date > 0
	{
	  let val: Lisp_Object = find_symbol_value (var);
          if !MARKERP (val)
	    { break 'body; }
	  Fput (var, Qlast_arrow_position (), Fmarker_position (val));
	  Fput (var, Qlast_arrow_string (),
		  overlay_arrow_string_or_property (var));
	}
      else if up_to_date < 0
	       || !NILP (Fget (var, Qlast_arrow_position ()))
	{
	  Fput (var, Qlast_arrow_position (), Qt ());
	  Fput (var, Qlast_arrow_string (), Qt ());
	}
	}
      vlist = XCDR (vlist);
    }
}


/* Return overlay arrow string to display at row.
   Return integer (bitmap number) for arrow bitmap in left fringe.
   Return nil if no overlay arrow.  */

unsafe fn
overlay_arrow_at_row (it: *mut it, row: *mut glyph_row) -> Lisp_Object
{
  let mut vlist: Lisp_Object;

  vlist = Voverlay_arrow_variable_list;
  while CONSP (vlist)
    {
      let var: Lisp_Object = XCAR (vlist);
      let mut val: Lisp_Object;

      'body:
	{
	  if !SYMBOLP (var)
	    { break 'body; }

      val = find_symbol_value (var);

      let mut arrow_marker_pos: ptrdiff_t;
      if MARKERP (val)
	  && (*XMARKER (val)).buffer as *mut buffer == current_buffer
	  && { arrow_marker_pos = marker_position (val);
	      /* Normally, the marker position will be at the row's
                 start charpos.  But if the previous text lines are
                 invisible, the row's start charpos includes those
                 invisible lines, so we make a more general test that
                 the marker position is anywhere between the start and
                 the end character positions of this row.  */
	      (MATRIX_ROW_START_CHARPOS! (row) <= arrow_marker_pos
	       && arrow_marker_pos < MATRIX_ROW_END_CHARPOS! (row)) }
	{
	  if FRAME_WINDOW_P ((*it).f)
	      /* FIXME: if ROW->reversed_p is set, this should test
		 the right fringe, not the left one.  */
	      && WINDOW_LEFT_FRINGE_WIDTH! ((*it).w) > 0
	    {
/* C 17233-17240's `#ifdef HAVE_WINDOW_SYSTEM` arm is dead on this target (P-08):
	      the `Fget (var, Qoverlay_arrow_bitmap)` test, the `lookup_fringe_bitmap`
	      call and the `fringe_bitmap != 0` early return are recorded in
	      docs/ports/xdisp.md, not transcribed.  */
	      return make_fixnum (-1); /* Use default arrow bitmap.  */
	    }
	  return overlay_arrow_string_or_property (var);
	}
	}
      vlist = XCDR (vlist);
    }

  return Qnil ();
}

/* Return true if point moved out of or into a composition.  Otherwise
   return false.  PREV_BUF and PREV_PT are the last point buffer and
   position.  BUF and PT are the current point buffer and position.  */

unsafe fn
check_point_in_composition (prev_buf: *mut buffer, prev_pt: ptrdiff_t,
			    buf: *mut buffer, pt: ptrdiff_t) -> bool
{
  let mut start: ptrdiff_t = 0;
  let mut end: ptrdiff_t = 0;
  /* C leaves `start`, `end` and `prop` for the `find_composition` calls below to
     fill in; Rust has no partial initialisation, so the port zero-initialises them
     (PORTING.md 1.5, deviation 1's rule, the C 8849 shape).  */
  let mut prop: Lisp_Object = Qnil ();
  let mut buffer: Lisp_Object = Qnil ();

  XSETBUFFER! (buffer, buf);
  /* Check a composition at the last point if point moved within the
     same buffer.  */
  if prev_buf == buf
    {
      if prev_pt == pt
	/* Point didn't move.  */
	{ return false; }

      if prev_pt > BUF_BEGV (buf) && prev_pt < BUF_ZV (buf)
	  && find_composition (prev_pt, -1, &raw mut start, &raw mut end, &raw mut prop, buffer)
	  && composition_valid_p (start, end, prop)
	  && start < prev_pt && end > prev_pt
	/* The last point was within the composition.  Return true iff
            point moved out of the composition.  */
	{ return (pt <= start || pt >= end); }
    }

  /* Check a composition at the current point.  */
  return (pt > BUF_BEGV (buf) && pt < BUF_ZV (buf)
	  && find_composition (pt, -1, &raw mut start, &raw mut end, &raw mut prop, buffer)
	  && composition_valid_p (start, end, prop)
	  && start < pt && end > pt);
}

/* Reconsider the clip changes of buffer which is displayed in W.  */

unsafe fn
reconsider_clip_changes (w: *mut window)
{
  let b: *mut buffer = XBUFFER ((*w).contents);

  if buffer_bits! (clip_changed, b)
      && window_bits! (w, window_end_valid)
      && (*((*w).current_matrix as *mut glyph_matrix)).buffer as *mut buffer == b
      && (*((*w).current_matrix as *mut glyph_matrix)).zv == BUF_ZV (b)
      && (*((*w).current_matrix as *mut glyph_matrix)).begv == BUF_BEGV (b)
    { buffer_bits! (clip_changed, b, false); }

  /* If display wasn't paused, and W is not a tool bar window, see if
     point has been moved into or out of a composition.  In that case,
     set b->clip_changed to force updating the screen.  If
     b->clip_changed has already been set, skip this check.  */
  if !buffer_bits! (clip_changed, b) && window_bits! (w, window_end_valid)
    {
      let pt: ptrdiff_t = (if w == XWINDOW (selected_window)
		      { PT! () } else { marker_position ((*w).pointm) });

      if (((*((*w).current_matrix as *mut glyph_matrix)).buffer as *mut buffer != b
	  || pt != (*w).last_point)
	  && check_point_in_composition ((*((*w).current_matrix as *mut glyph_matrix)).buffer as *mut buffer,
					 (*w).last_point, b, pt))
	{ buffer_bits! (clip_changed, b, true); }
    }
}

unsafe fn
propagate_buffer_redisplay ()
{ /* Resetting b->text->redisplay is problematic!
     We can't just reset it in the case that some window that displays
     it has not been redisplayed; and such a window can stay
     unredisplayed for a long time if it's currently invisible.
     But we do want to reset it at the end of redisplay otherwise
     its displayed windows will keep being redisplayed over and over
     again.
     So we copy all b->text->redisplay flags up to their windows here,
     such that mark_window_display_accurate can safely reset
     b->text->redisplay.  */
  let mut ws: Lisp_Object = window_list ();
  while CONSP (ws)
    {
      let thisw: *mut window = XWINDOW (XCAR (ws));
      let thisb: *mut buffer = XBUFFER ((*thisw).contents);
      if buffer_text_bits! (text.redisplay, thisb)
	{ window_bits! (thisw, redisplay, true); }
      ws = XCDR (ws);
    }
}

/* C 17339-17341: `#define STOP_POLLING do { if (! polling_stopped_here) stop_polling ();
   polling_stopped_here = true; } while (false)`.  The C macro assigns a local of its
   caller, so the port passes that local in (the coding.rs `ONE_MORE_BYTE` shape).  */
macro_rules! STOP_POLLING {
  ($polling_stopped_here:ident) => {
    { if !$polling_stopped_here { stop_polling (); }
      $polling_stopped_here = true; }
  };
}

/* C 17343-17345: `#define RESUME_POLLING do { if (polling_stopped_here) start_polling ();
   polling_stopped_here = false; } while (false)`.  */
macro_rules! RESUME_POLLING {
  ($polling_stopped_here:ident) => {
    { if $polling_stopped_here { start_polling (); }
      $polling_stopped_here = false; }
  };
}

/* Perhaps in the future avoid recentering windows if it
   is not necessary; currently that causes some problems.  */
unsafe fn
redisplay_internal ()
{
  let mut w: *mut window = XWINDOW (selected_window);
  let mut sw: *mut window;
  let mut fr: *mut frame;
  let mut must_finish: bool = false;
  let mut match_p: bool;
  let mut tlbufpos: text_pos;
  let mut tlendpos: text_pos;
  let mut number_of_visible_frames: c_int;
  let mut sf: *mut frame;
  let mut polling_stopped_here: bool = false;
  let mut tail: Lisp_Object;
  let mut frame: Lisp_Object;

  /* Set a limit to the number of retries we perform due to horizontal
     scrolling, this avoids getting stuck in an uninterruptible
     infinite loop (Bug #24633).  */
  const MAX_HSCROLL_RETRIES: c_int = 16;
  let mut hscroll_retries: c_int = 0;

  /* Limit the number of retries for when frame(s) become garbaged as
     result of redisplaying them.  Some packages set various redisplay
     hooks, such as window-scroll-functions, to run Lisp that always
     calls APIs which cause the frame's garbaged flag to become set,
     so we loop indefinitely.  */
  const MAX_GARBAGED_FRAME_RETRIES: c_int = 2;
  let mut garbaged_frame_retries: c_int = 0;

  /* False means that only the selected_window needs to be updated.
     True means that other windows may need to be updated as well,
     so we need to consult `needs_no_update` for all windows.  */
  let mut consider_all_windows_p: bool;

  /* True means redisplay has to redisplay the miniwindow.  */
  let mut update_miniwindow_p: bool = false;

  redisplay_trace (c"redisplay_internal %d\n".as_ptr (), redisplaying_p as c_int);

  /* I don't think this happens but let's be paranoid.  In particular,
     this was observed happening when Emacs shuts down due to losing X
     connection, in which case accessing SELECTED_FRAME and the frame
     structure is likely to barf.  */
  if redisplaying_p
    { return; }

  /* No redisplay if running in batch mode or frame is not yet fully
     initialized, or redisplay is explicitly turned off by setting
     Vinhibit_redisplay.  */
  if ((FRAME_INITIAL_P (SELECTED_FRAME ())
       && redisplay_skip_initial_frame)
      || !NILP (Vinhibit_redisplay))
    { return; }

  /* Don't examine these until after testing Vinhibit_redisplay.
     When Emacs is shutting down, perhaps because its connection to
     X has dropped, we should not look at them at all.  */
  fr = XFRAME ((*w).frame);
  sf = SELECTED_FRAME ();

  if !frame_bits! (glyphs_initialized_p, fr)
    { return; }

/* C 17411-17419's two `#if` blocks are dead on this target (P-08):
     `defined (USE_X_TOOLKIT) || defined (USE_GTK) || defined (HAVE_NS)`'s
     `popup_activated ()` and `defined (HAVE_HAIKU)`'s `popup_activated_p` are both
     recorded in docs/ports/xdisp.md, not transcribed.  */

  redisplay_counter += 1;

  /* Record a function that clears redisplaying_p
     when we leave this function.  */
  let count: specpdl_ref = SPECPDL_INDEX ();
  record_unwind_protect_void (Some (unwind_redisplay));
  redisplaying_p = true;
  block_buffer_flips ();
  specbind (Qinhibit_free_realized_faces (), Qnil ());

  /* Record this function, so it appears on the profiler's backtraces.  */
  record_in_backtrace (Qredisplay_internal_xC_functionx (), core::ptr::null_mut (), 0);

  FOR_EACH_FRAME! (tail, frame, {
	  frame_bits! (already_hscrolled_p, XFRAME (frame), false); });

  reset_outermost_restrictions ();

 'retry: loop
    {
  /* Remember the currently selected window.  */
  sw = w;

  forget_escape_and_glyphless_faces ();

  inhibit_free_realized_faces = false;

  /* If face_change, init_iterator will free all realized faces, which
     includes the faces referenced from current matrices.  So, we
     can't reuse current matrices in this case.  */
  if face_change
    { windows_or_buffers_changed = 47; }

  let mut previous_frame: *mut frame;
  if is_tty_frame (sf)
      && { previous_frame = (*FRAME_TTY! (sf)).previous_frame;
     previous_frame != sf }
    {
      if previous_frame.is_null ()
	  || root_frame (previous_frame) != root_frame (sf)
	{
	  /* Since frames on a single terminal share the same display
	     area, displaying a different frame means redisplay the
	     whole thing.  */
	  SET_FRAME_GARBAGED! (sf);
	  set_tty_color_mode (FRAME_TTY! (sf), sf);
	}
      (*FRAME_TTY! (sf)).previous_frame = sf;
    }

  /* Set the visible flags for all frames.  Do this before checking for
     resized or garbaged frames; they want to know if their frames are
     visible.  See the comment in frame.h for FRAME_SAMPLE_VISIBILITY.  */
  number_of_visible_frames = 0;

  FOR_EACH_FRAME! (tail, frame, {
	{
	  let f: *mut frame = XFRAME (frame);

      /* frame_redisplay_p true basically means the frame is visible. */
	  if frame_redisplay_p (f)
	    {
	      number_of_visible_frames += 1;
	  /* Adjust matrices for visible frames only.  */
	      if frame_bits! (fonts_changed, f)
		{
		  adjust_frame_glyphs (f);
	      /* Disable all redisplay optimizations for this frame.
		 This is because adjust_frame_glyphs resets the
		 enabled_p flag for all glyph rows of all windows, so
		 many optimizations will fail anyway, and some might
		 fail to test that flag and do bogus things as
		 result.  */
		  SET_FRAME_GARBAGED! (f);
		  frame_bits! (fonts_changed, f, false);
		}
	  /* If cursor type has been changed on the frame
	     other than selected, consider all frames.  */
	      if f != sf && frame_bits! (cursor_type_changed, f)
		{ fset_redisplay (f); }
	    }
	  clear_desired_matrices (f);
    }
  });

  /* Notice any pending interrupt request to change frame size.  */
  do_pending_window_change (true);

  /* Clear frames marked as garbaged.  */
  let current_matrices_cleared: bool = clear_garbaged_frames ();

  /* Build menubar and tool-bar items.  */
  if NILP (Vmemory_full)
    { prepare_menu_bars (); }

  /* do_pending_window_change could change the selected_window due to
     frame resizing which makes the selected window too small.
     prepare_menu_bars may call lisp hooks and hence also change the
     selected_window.  */
  if WINDOWP (selected_window) && { w = XWINDOW (selected_window); w != sw }
    { sw = w; }

  reconsider_clip_changes (w);

  /* In most cases selected window displays current buffer.  */
  match_p = XBUFFER ((*w).contents) == current_buffer;
  if match_p
    {
      /* Detect case that we need to write or remove a star in the mode line.  */
      if (BUF_SAVE_MODIFF! (current_buffer) < BUF_MODIFF! (current_buffer)) != window_bits! (w, last_had_star)
	{ window_bits! (w, update_mode_line, true); }

      if mode_line_update_needed (w)
	{ window_bits! (w, update_mode_line, true); }

      /* If reconsider_clip_changes above decided that the narrowing
	 in the current buffer changed, make sure all other windows
	 showing that buffer will be redisplayed.  */
      if buffer_bits! (clip_changed, current_buffer)
	{ bset_update_mode_line (current_buffer); }
    }

  /* Normally the message* functions will have already displayed and
     updated the echo area, but the frame may have been trashed, or
     the update may have been preempted, so display the echo area
     again here.  Checking message_cleared_p captures the case that
     the echo area should be cleared.  */
  if (!NILP (echo_area_buffer[0]) && !display_last_displayed_message_p)
      || (!NILP (echo_area_buffer[1]) && display_last_displayed_message_p)
      || (message_cleared_p
	  && minibuf_level == 0
	  /* If the mini-window is currently selected, this means the
	     echo-area doesn't show through.  */
	  && !MINI_WINDOW_P (XWINDOW (selected_window)))
    {
      echo_area_display (false);

      if message_cleared_p
	{ update_miniwindow_p = true; }

      must_finish = true;

      /* If we don't display the current message, don't clear the
	 message_cleared_p flag, because, if we did, we wouldn't clear
	 the echo area in the next redisplay which doesn't preserve
	 the echo area.  */
      if !display_last_displayed_message_p
	{ message_cleared_p = false; }
    }
  else if EQ (selected_window, minibuf_window)
	   && (buffer_bits! (clip_changed, current_buffer) || window_outdated (w))
	   && resize_mini_window (w, false)
    {
      /* Resized active mini-window to fit the size of what it is
         showing if its contents might have changed.  */
      must_finish = true;

      /* If window configuration was changed, frames may have been
	 marked garbaged.  Clear them or we will experience
	 surprises wrt scrolling.  */
      clear_garbaged_frames ();
    }

  if !NILP (Vrun_hooks)
    { run_window_change_functions (); }

  if windows_or_buffers_changed != 0 && update_mode_lines == 0
    /* Code that sets windows_or_buffers_changed doesn't distinguish whether
       only the windows's contents needs to be refreshed, or whether the
       mode-lines also need a refresh.  */
    { update_mode_lines = (if windows_or_buffers_changed == REDISPLAY_SOME
			 { REDISPLAY_SOME } else { 32 }); }

  /* If specs for an arrow have changed, do thorough redisplay
     to ensure we remove any arrow that should no longer exist.  */
  /* Apparently, this is the only case where we update other windows,
     without updating other mode-lines.  */
  overlay_arrows_changed_p (true);

  consider_all_windows_p = (update_mode_lines != 0
			    || windows_or_buffers_changed != 0
			    || current_matrices_cleared);

/* C 17604-17609: `#define AINC(a,i) { Lisp_Object entry = Fgethash (make_fixnum (i), a,
   make_fixnum (0)); if (FIXNUMP (entry)) Fputhash (make_fixnum (i), make_fixnum (1 +
   XFIXNUM (entry)), a); }`.  It declares a local, so it is a `macro_rules!` taking the
   two arguments (the coding.rs `ONE_MORE_BYTE` shape).  */
macro_rules! AINC {
  ($a:expr, $i:expr) => {
    {
      let entry: Lisp_Object = Fgethash (make_fixnum ($i), $a, make_fixnum (0));
      if FIXNUMP (entry)
        { Fputhash (make_fixnum ($i), make_fixnum (1 + XFIXNUM (entry)), $a); }
    }
  };
}

  AINC! (Vredisplay__all_windows_cause, windows_or_buffers_changed as EMACS_INT);
  AINC! (Vredisplay__mode_lines_cause, update_mode_lines as EMACS_INT);

  /* Optimize the case that only the line containing the cursor in the
     selected window has changed.  Variables starting with this_ are
     set in display_line and record information about the line
     containing the cursor.  */
  tlbufpos = this_line_start_pos;
  tlendpos = this_line_end_pos;
  'frame_pass: {
    'update: {
  if !consider_all_windows_p
      && CHARPOS! (tlbufpos) > 0
      && !window_bits! (w, update_mode_line)
      && !buffer_bits! (clip_changed, current_buffer)
      && !buffer_bits! (prevent_redisplay_optimizations_p, current_buffer)
      && frame_redisplay_p (XFRAME ((*w).frame))
      && !frame_bits! (cursor_type_changed, XFRAME ((*w).frame))
      && !frame_bits! (face_change, XFRAME ((*w).frame))
      /* Make sure recorded data applies to current buffer, etc.  */
      && this_line_buffer == current_buffer
      && match_p
      && !window_bits! (w, force_start)
      && !window_bits! (w, optional_new_start)
      /* Point must be on the line that we have info recorded about.  */
      && PT! () >= CHARPOS! (tlbufpos)
      && PT! () <= Z! () - CHARPOS! (tlendpos)
      /* FIXME: The following condition is only needed when
	 significant parts of the buffer are hidden (e.g., under
	 hs-minor-mode), but there doesn't seem to be a simple way of
	 detecting that, so we always disable the one-line redisplay
	 optimizations whenever display-line-numbers-mode is turned on
	 in the buffer.  */
      && (NILP (Vdisplay_line_numbers)
	  || EQ (Vdisplay_line_numbers, Qvisual ()))
      /* All text outside that line, including its final newline,
	 must be unchanged.  */
      && text_outside_line_unchanged_p (w, CHARPOS! (tlbufpos),
					CHARPOS! (tlendpos))
      /* If this is a window on a tty root frame displaying a child frame,
	 the current matrix of W may contain glyphs of that child frame.
	 Don't try shortcuts that might use the current matrix in this case.  */
      && !is_tty_root_frame_with_visible_child (XFRAME ((*w).frame))
    {
      'cancel: {
      if CHARPOS! (tlbufpos) > BEGV! ()
	  && FETCH_BYTE! (BYTEPOS! (tlbufpos) - 1) != b'\n'
	  && (CHARPOS! (tlbufpos) == ZV! ()
	      || FETCH_BYTE! (BYTEPOS! (tlbufpos)) == b'\n')
	/* Former continuation line has disappeared by becoming empty.  */
	{ break 'cancel; }
      else if window_outdated (w) || MINI_WINDOW_P (w)
	{
	  /* We have to handle the case of continuation around a
	     wide-column character (see the comment in indent.c around
	     line 1340).

	     For instance, in the following case:

	     --------  Insert  --------
	     K_A_N_\\   `a'    K_A_N_a\		`X_' are wide-column chars.
	     J_I_       ==>    J_I_		`^^' are cursors.
	     ^^                ^^
	     --------          --------

	     As we have to redraw the line above, we cannot use this
	     optimization.  */

	  let mut it: it = core::mem::zeroed ();
	  let line_height_before: c_int = this_line_pixel_height;

	  /* Note that start_display will handle the case that the
	     line starting at tlbufpos is a continuation line.  */
	  start_display (&raw mut it, w, tlbufpos);

	  /* Implementation note: It this still necessary?  */
	  if it.current_x != this_line_start_x
	    { break 'cancel; }

	  /* Give up on this optimization if the line starts with a
	     string with display property that draws on the fringes,
	     as that might interfere with line-prefix display.  */
	  if it.sp > 1
	      && it.method == GET_FROM_IMAGE && it.image_id == -1
	    { break 'cancel; }
	  redisplay_trace (c"trying display optimization 1\n".as_ptr ());
	  (*w).cursor.vpos = -1;
	  overlay_arrow_seen = false;
	  it.vpos = this_line_vpos;
	  it.current_y = this_line_y;
	  it.glyph_row = MATRIX_ROW! ((*w).desired_matrix as *mut glyph_matrix, this_line_vpos);
	  display_line (&raw mut it, -1);

	  /* If line contains point, is not continued,
             and ends at same distance from eob as before, we win.  */
	  if (*w).cursor.vpos >= 0
              /* Line is not continued, otherwise this_line_start_pos
                 would have been set to 0 in display_line.  */
	      && CHARPOS! (this_line_start_pos) != 0
	      /* Line ends as before.  */
	      && CHARPOS! (this_line_end_pos) == CHARPOS! (tlendpos)
              /* Line has same height as before.  Otherwise other lines
                 would have to be shifted up or down.  */
	      && this_line_pixel_height == line_height_before
	      /* Cannot use this optimization if hscrolling current
		 line and this line is the current one, because
		 display_line above is not informed about the
		 current-line's vpos, and cannot DTRT in that case.  */
	      && !hscrolling_current_line_p (w)
	    {
 	      /* If this is not the window's last line, we must adjust
 		 the charstarts of the lines below.  */
 	      if it.current_y < it.last_visible_y
  		{
 		  let row: *mut glyph_row
 		    = MATRIX_ROW! ((*w).current_matrix as *mut glyph_matrix, this_line_vpos + 1);
  		  let mut delta: ptrdiff_t; let mut delta_bytes: ptrdiff_t;

		  /* We used to distinguish between two cases here,
		     conditioned by Z - CHARPOS (tlendpos) == ZV, for
		     when the line ends in a newline or the end of the
		     buffer's accessible portion.  But both cases did
		     the same, so they were collapsed.  */
		  delta = (Z! ()
			   - CHARPOS! (tlendpos)
			   - MATRIX_ROW_START_CHARPOS! (row));
		  delta_bytes = (Z_BYTE! ()
				 - BYTEPOS! (tlendpos)
				 - MATRIX_ROW_START_BYTEPOS! (row));

  		  increment_matrix_positions ((*w).current_matrix as *mut glyph_matrix,
					      this_line_vpos + 1,
					      (*((*w).current_matrix as *mut glyph_matrix)).nrows,
					      delta, delta_bytes);
		}

	      /* If this row displays text now but previously didn't,
		 or vice versa, w->window_end_vpos may have to be
		 adjusted.  */
	      if MATRIX_ROW_DISPLAYS_TEXT_P! (it.glyph_row.offset (-1))
		{
		  if (*w).window_end_vpos < this_line_vpos
		    { (*w).window_end_vpos = this_line_vpos; }
		}
	      else if (*w).window_end_vpos == this_line_vpos
		       && this_line_vpos > 0
		{ (*w).window_end_vpos = this_line_vpos - 1; }
	      window_bits! (w, window_end_valid, false);

	      /* Update hint: No need to try to scroll in update_window.  */
	      glyph_matrix_bits! (no_scrolling_p, (*w).desired_matrix as *mut glyph_matrix, true);

/* C 17760-17766's `#ifdef GLYPH_DEBUG` and `#ifdef HAVE_WINDOW_SYSTEM` blocks are dead
	      on this target (P-08): the `*w->desired_matrix->method = 0` / `debug_method_add
	      (w, "optimization 1")` pair and `update_window_fringes (w, false)` are recorded,
	      not transcribed.  */
	      break 'update;
	    }
	  else
	    { break 'cancel; }
	}
      else if /* Cursor position hasn't changed.  */
	       PT! () == (*w).last_point
	       /* Make sure the cursor was last displayed
		  in this window.  Otherwise we have to reposition it.  */

	       /* PXW: Must be converted to pixels, probably.  */
	       && 0 <= (*w).cursor.vpos
	       && (*w).cursor.vpos < WINDOW_TOTAL_LINES (w)
	{
	  if !must_finish
	    {
	      do_pending_window_change (true);
	      /* If selected_window changed, redisplay again.  */
	      if WINDOWP (selected_window)
		  && { w = XWINDOW (selected_window); w != sw }
		{ continue 'retry; }

		/* We used to always goto end_of_redisplay here, but this
		 isn't enough if we have a blinking cursor.  */
	      if window_bits! (w, cursor_off_p) == window_bits! (w, last_cursor_off_p)
		{ break 'retry; }
	    }
	  break 'update;
	}
      /* If highlighting the region, or if the cursor is in the echo area,
	 then we can't just move the cursor.  */
      else if NILP (Vshow_trailing_whitespace)
	       && !cursor_in_echo_area
	       && !composition_break_at_point
	{
	  let mut it: it = core::mem::zeroed ();
	  let mut row: *mut glyph_row;

	  /* Skip from tlbufpos to PT and see where it is.  Note that
	     PT may be in invisible text.  If so, we will end at the
	     next visible position.  */
	  init_iterator (&raw mut it, w, CHARPOS! (tlbufpos), BYTEPOS! (tlbufpos),
			 core::ptr::null_mut (), DEFAULT_FACE_ID);
	  it.current_x = this_line_start_x;
	  it.current_y = this_line_y;
	  it.wrap_prefix_width = 0;
	  it.vpos = this_line_vpos;

	  if buffer_bits! (long_line_optimizations_p, current_buffer)
	      && it.line_wrap == TRUNCATE
	      && PT! () - CHARPOS! (tlbufpos) > large_hscroll_threshold
	    {
	      /* When lines are very long and truncated, jumping to
		 the next visible line is much faster than slowly
		 iterating there.  */
	      reseat_at_next_visible_line_start (&raw mut it, false);
	      if IT_CHARPOS! (it) <= PT! () /* point moved off this line */
		{ it.vpos = this_line_vpos + 1; }
	    }
	  else
	    {
	      /* The call to move_it_to stops in front of PT, but
		 moves over before-strings.  */
	      move_it_to (&raw mut it, PT! (), -1, -1, -1, MOVE_TO_POS);
	    }

	  if it.vpos == this_line_vpos
	      && { row = MATRIX_ROW! ((*w).current_matrix as *mut glyph_matrix, this_line_vpos);
		  glyph_row_bits! (enabled_p, row) }
	    {
	      eassert! (this_line_vpos == it.vpos);
	      eassert! (this_line_y == it.current_y);
	      set_cursor_from_row (w, row, (*w).current_matrix as *mut glyph_matrix, 0, 0, 0, 0);
	      if cursor_row_fully_visible_p (w, false, true, false)
		{
/* C 17842-17845's `#ifdef GLYPH_DEBUG` block (`*w->desired_matrix->method = 0`
		  and `debug_method_add (w, "optimization 3")`) is dead (P-08).  */
		  { break 'update; }
		}
	      else
		{ break 'cancel; }
	    }
	  else
	    { break 'cancel; }
	}
      }

    /* cancel:  (C 17855 — the label has no Rust site; every `goto cancel` is the
       `break 'cancel` above.)  */
      /* Text changed drastically or point moved off of line.  */
      SET_MATRIX_ROW_ENABLED_P! ((*w).desired_matrix as *mut glyph_matrix, this_line_vpos, false);
    }

  CHARPOS! (this_line_start_pos) = 0;
  clear_face_cache_count += 1;
/* C 17862-17864's `#ifdef HAVE_WINDOW_SYSTEM` block (`++clear_image_cache_count`) is
     dead (C 2716 already records the pair).  */

  /* Build desired matrices, and update the display.  If
     consider_all_windows_p, do it for all windows on all frames that
     require redisplay, as specified by their 'redisplay' flag.
     Otherwise do it for selected_window, only.  */

  if consider_all_windows_p
    {
      FOR_EACH_FRAME! (tail, frame, {
	  frame_bits! (updated_p, XFRAME (frame), false); });

      propagate_buffer_redisplay ();

      let mut tty_root_frames: Lisp_Object = Qnil ();
      FOR_EACH_FRAME! (tail, frame, {
	{
	  'next_frame: {
	  let f: *mut frame = XFRAME (frame);

	  if is_tty_frame (f)
	    {
	      /* Ignore all invisible tty frames, children or root.  */
	      if !frame_redisplay_p (f)
		  /* Ignore frames not yet completely made, which we
		     cannot safely redisplay.  */
		  || !frame_bits! (after_make_frame, f)
		{ break 'next_frame; }

	      /* Remember tty root frames which we've seen.  */
	      if FRAME_PARENT_FRAME (f).is_null ()
		  && NILP (memq_no_quit (frame, tty_root_frames))
		{ tty_root_frames = Fcons (frame, tty_root_frames); }
	    }

	'retry_frame: loop
	  {
	    if FRAME_WINDOW_P (f) || is_tty_frame (f) || f == sf
	    {
	      /* Only GC scrollbars when we redisplay the whole frame.  */
	      let gcscrollbars: bool = frame_bits! (redisplay, f) || !REDISPLAY_SOME_P! ();
	      let f_redisplay_flag: bool = frame_bits! (redisplay, f);

	      /* The X error handler may have deleted that frame before
		 we went back to retry_frame.  Or this could be a TTY
		 frame that is not completely made, in which case we
		 cannot safely redisplay its windows.  This must come
		 before any accesses to f->terminal.  */
	      if !FRAME_LIVE_P (f)
		  || (FRAME_TERMCAP_P (f) && !frame_bits! (after_make_frame, f))
		{ break 'next_frame; }

	      /* Mark all the scroll bars to be removed; we'll redeem
		 the ones we want when we redisplay their windows.  */
	      if gcscrollbars && !(*FRAME_TERMINAL! (f)).condemn_scroll_bars_hook.is_none ()
		{ (*FRAME_TERMINAL! (f)).condemn_scroll_bars_hook.unwrap () (f as *mut c_void); }

	      if frame_redisplay_p (f)
		{
		  /* Don't allow freeing images and faces for this
		     frame as long as the frame's update wasn't
		     completed.  This prevents crashes when some Lisp
		     that runs from the various hooks or font-lock
		     decides to clear the frame's image cache and face
		     cache, when the images and faces in those caches
		     are referenced by the desired matrix.  */
		  frame_bits! (inhibit_clear_image_cache, f, true);
		  redisplay_windows (FRAME_ROOT_WINDOW (f));
		}
	      /* Remember that the invisible frames need to be redisplayed next
		 time they're visible.  */
	      else if !REDISPLAY_SOME_P! ()
		{ frame_bits! (redisplay, f, true); }

	      /* The X error handler may have deleted that frame.  */
	      if !FRAME_LIVE_P (f)
		{ break 'next_frame; }

	      /* Any scroll bars which redisplay_windows should have
		 nuked should now go away.  */
	      if gcscrollbars && !(*FRAME_TERMINAL! (f)).judge_scroll_bars_hook.is_none ()
		{ (*FRAME_TERMINAL! (f)).judge_scroll_bars_hook.unwrap () (f as *mut c_void); }

	      if frame_redisplay_p (f)
		{
		  /* If fonts changed on visible frame, display again.  */
		  if frame_bits! (fonts_changed, f)
		    {
		      adjust_frame_glyphs (f);
		      /* Disable all redisplay optimizations for this
			 frame.  For the reasons, see the comment near
			 the previous call to adjust_frame_glyphs above.  */
		      SET_FRAME_GARBAGED! (f);
		      frame_bits! (fonts_changed, f, false);
		      continue 'retry_frame;
		    }

		  /* See if we have to hscroll.  */
		  if !frame_bits! (already_hscrolled_p, f)
		    {
		      frame_bits! (already_hscrolled_p, f, true);
		      if hscroll_retries <= MAX_HSCROLL_RETRIES
			  && hscroll_windows ((*f).root_window)
                        {
			  hscroll_retries += 1;
			  continue 'retry_frame;
                        }
		    }

		  /* If the frame's redisplay flag was not set before
		     we went about redisplaying its windows, but it is
		     set now, that means we employed some redisplay
		     optimizations inside redisplay_windows, and
		     bypassed producing some screen lines.  But if
		     f->redisplay is now set, it might mean the old
		     faces are no longer valid (e.g., if redisplaying
		     some window called some Lisp which defined a new
		     face or redefined an existing face), so trying to
		     use them in update_frame will segfault.
		     Therefore, we must redisplay this frame.  */
		  if !f_redisplay_flag && frame_bits! (redisplay, f)
		    { continue 'retry_frame; }
                  /* In some case (e.g., window resize), we notice
                     only during window updating that the window
                     content changed unpredictably (e.g., a GTK
                     scrollbar moved, or some Lisp hook that winds up
                     calling adjust_frame_glyphs) and that our
                     previous estimation of the frame content was
                     garbage.  We have to start over.  These cases
                     should be rare, so going all the way back to the
                     top of redisplay should be good enough.  */
		  if FRAME_GARBAGED_P (f)
		      && { let old: c_int = garbaged_frame_retries; garbaged_frame_retries += 1; old } < MAX_GARBAGED_FRAME_RETRIES
		    { continue 'retry; }

/* C 17997-18001's `#ifdef HAVE_WINDOW_SYSTEM` block is dead on this target (P-08):
		     the `FRAME_WINDOW_P (f) && FRAME_RIF (f)->clear_under_internal_border (f)`
		     call is recorded, not transcribed.  */
		  /* Prevent various kinds of signals during display
		     update.  stdio is not robust about handling
		     signals, which can cause an apparent I/O error.  */
		  if interrupt_input
		    { unrequest_sigio (); }
		  STOP_POLLING! (polling_stopped_here);

		  update_frame (f, false);
		  /* On some platforms (at least MS-Windows), the
		     scroll_run_hook called from scrolling_window
		     called from update_frame could set the frame's
		     garbaged flag, in which case we need to redisplay
		     the frame.  Don't do that on TTY frames, since we
		     need to keep the garbaged flag in that case when
		     the frame has been resized.  */
		  if FRAME_GARBAGED_P (f)
		    {
		      fset_redisplay (f);
		      frame_bits! (garbaged, f, false);
		      continue 'retry_frame;
		    }
		  frame_bits! (cursor_type_changed, f, false);
		  frame_bits! (updated_p, f, true);
		  frame_bits! (inhibit_clear_image_cache, f, false);
		}
	    }
	      break 'retry_frame;
	  }
	  }
	}
	});

      if CONSP (tty_root_frames)
	{ combine_updates (tty_root_frames); }

      eassert! (EQ ((*XFRAME (selected_frame)).selected_window, selected_window));

      /* Do the mark_window_display_accurate after all windows have
	 been redisplayed because this call resets flags in buffers
	 which are needed for proper redisplay.  */
      FOR_EACH_FRAME! (tail, frame, {
	{
	  let f: *mut frame = XFRAME (frame);
	  if frame_bits! (updated_p, f)
	    {
	      frame_bits! (redisplay, f, false);
	      frame_bits! (garbaged, f, false);
	      mark_window_display_accurate ((*f).root_window, true);
	      if !(*FRAME_TERMINAL! (f)).frame_up_to_date_hook.is_none ()
		{ (*FRAME_TERMINAL! (f)).frame_up_to_date_hook.unwrap () (f as *mut c_void); }
	    }
	}
	});
      break 'frame_pass;   /* C falls out of the `if (consider_all_windows_p)` */
    }
  else if frame_redisplay_p (sf)
    {
      frame_bits! (inhibit_clear_image_cache, sf, true);
      displayed_buffer = XBUFFER ((*XWINDOW (selected_window)).contents);
      /* Use list_of_error, not Qerror, so that
	 we catch only errors and don't run the debugger.  */
      internal_condition_case_1 (Some (redisplay_window_1), selected_window,
				 list_of_error,
				 Some (redisplay_window_error));
      if update_miniwindow_p
	{
	  let mini_window: Lisp_Object = FRAME_MINIBUF_WINDOW (sf);

	  displayed_buffer = XBUFFER ((*XWINDOW (mini_window)).contents);
	  internal_condition_case_1 (Some (redisplay_window_1), mini_window,
				     list_of_error,
				     Some (redisplay_window_error));
	}
      }

      /* Compare desired and current matrices, perform output.  */

    /* update:  (C 18072 — the label has no Rust site; `break 'update` from the
       optimization block is C's `goto update`, and falling out of `'update` above
       is C's fall-through into the label.)  */
      /* If fonts changed, display again.  Likewise if redisplay_window_1
	 above caused some change (e.g., a change in faces) that requires
	 considering the entire frame again.  */
      if frame_bits! (fonts_changed, sf) || frame_bits! (redisplay, sf)
	{
	  if frame_bits! (redisplay, sf)
	    {
	      /* Set this to force a more thorough redisplay.
		 Otherwise, we might immediately loop back to the
		 above "else-if" clause (since all the conditions that
		 led here might still be true), and we will then
		 infloop, because the selected-frame's redisplay flag
		 is not (and cannot be) reset.  */
	      windows_or_buffers_changed = 50;
	    }
	  continue 'retry;
	}

      /* Prevent freeing of realized faces, since desired matrices are
	 pending that reference the faces we computed and cached.  */
      inhibit_free_realized_faces = true;

      /* Prevent various kinds of signals during display update.
	 stdio is not robust about handling signals,
	 which can cause an apparent I/O error.  */
      if interrupt_input
	{ unrequest_sigio (); }
      STOP_POLLING! (polling_stopped_here);

      if frame_redisplay_p (sf)
	{
          if hscroll_retries <= MAX_HSCROLL_RETRIES
              && hscroll_windows (selected_window)
            {
              hscroll_retries += 1;
              continue 'retry;
            }

	  window_bits! (XWINDOW (selected_window), must_be_updated_p, true);
	  update_frame (sf, false);

	  if is_tty_frame (sf)
	    { combine_updates_for_frame (sf, false); }

	  frame_bits! (cursor_type_changed, sf, false);
	  frame_bits! (inhibit_clear_image_cache, sf, false);
	}

      /* We may have called echo_area_display at the top of this
	 function.  If the echo area is on another frame, that may
	 have put text on a frame other than the selected one, so the
	 above call to update_frame would not have caught it.  Catch
	 it here.  */
      let mini_window: Lisp_Object = FRAME_MINIBUF_WINDOW (sf);
      let mini_frame: *mut frame = XFRAME (WINDOW_FRAME! (XWINDOW (mini_window)));

      if mini_frame != sf
	{
	  window_bits! (XWINDOW (mini_window), must_be_updated_p, true);
	  update_frame (mini_frame, false);
	  if is_tty_frame (mini_frame)
	    { combine_updates_for_frame (mini_frame, false); }
	  frame_bits! (cursor_type_changed, mini_frame, false);
          if hscroll_retries <= MAX_HSCROLL_RETRIES
              && hscroll_windows (mini_window)
            {
              hscroll_retries += 1;
              continue 'retry;
            }
	}
    }
  }

  if !consider_all_windows_p
    {
      /* This has already been done above if
	 consider_all_windows_p is set.  */
      if buffer_text_bits! (text.redisplay, XBUFFER ((*w).contents))
	  && buffer_window_count (XBUFFER ((*w).contents)) > 1
	/* This can happen if b->text->redisplay was set during
	   jit-lock.  */
	{ propagate_buffer_redisplay (); }
      mark_window_display_accurate_1 (w, true);

      /* Say overlay arrows are up to date.  */
      update_overlay_arrows (1);

      if !(*FRAME_TERMINAL! (sf)).frame_up_to_date_hook.is_none ()
	{ (*FRAME_TERMINAL! (sf)).frame_up_to_date_hook.unwrap () (sf as *mut c_void); }
    }

  update_mode_lines = 0;
  windows_or_buffers_changed = 0;

  /* Start SIGIO interrupts coming again.  Having them off during the
     code above makes it less likely one will discard output, but not
     impossible, since there might be stuff in the system buffer here.
     But it is much hairier to try to do anything about that.  */
  if interrupt_input
    { request_sigio (); }
  RESUME_POLLING! (polling_stopped_here);

  /* If a frame has become visible which was not before, redisplay
     again, so that we display it.  Expose events for such a frame
     (which it gets when becoming visible) don't call the parts of
     redisplay constructing glyphs, so simply exposing a frame won't
     display anything in this case.  So, we have to display these
     frames here explicitly.  */
  let mut new_count: c_int = 0;

  FOR_EACH_FRAME! (tail, frame, {
	{
	  if frame_redisplay_p (XFRAME (frame))
	    { new_count += 1; }
	}
	});

    if new_count != number_of_visible_frames
      { windows_or_buffers_changed = 52; }

  /* Change frame size now if a change is pending.  */
  do_pending_window_change (true);

  /* If we just did a pending size change, or have additional
     visible frames, or selected_window changed, redisplay again.  */
  if windows_or_buffers_changed != 0
      || (WINDOWP (selected_window)
	  && { w = XWINDOW (selected_window); w != sw })
    { continue 'retry; }

  /* Clear the face and image caches.

     We used to do this only if consider_all_windows_p.  But the cache
     needs to be cleared if a timer creates images in the current
     buffer (e.g. the test case in Bug#6230).  */

  if clear_face_cache_count > CLEAR_FACE_CACHE_COUNT
    {
      clear_face_cache (false);
      clear_face_cache_count = 0;
    }

/* C 18213-18219's `#ifdef HAVE_WINDOW_SYSTEM` block (`clear_image_cache_count >
     CLEAR_IMAGE_CACHE_COUNT` and `clear_image_caches (Qnil)`) is dead (C 2716 already
     records the pair).  */

      break 'retry;
    }
  /* end_of_redisplay:  (C 18221 — the label has no Rust site; C's fall-through out of
     the loop above and its `goto end_of_redisplay` are both the `break 'retry`.)  */
/* C 18222-18224's `#ifdef HAVE_NS` block (`ns_set_doc_edited ()`) is dead (P-08).  */
  if interrupt_input && interrupts_deferred
    { request_sigio (); }

  /* We're done with this redisplay cycle, so reset the tick count in
     preparation for the next redisplay cycle.  */
  if max_redisplay_ticks > 0
    { update_redisplay_ticks (0, core::ptr::null_mut ()); }

  unbind_to (count, Qnil ());
  RESUME_POLLING! (polling_stopped_here);
}
unsafe extern "C" fn
unwind_redisplay_preserve_echo_area ()
{
  unblock_buffer_flips ();
}

/* Redisplay, but leave alone any recent echo area message unless
   another message has been requested in its place.

   This is useful in situations where you need to redisplay but no
   user action has occurred, making it inappropriate for the message
   area to be cleared.  See tracking_off and
   wait_reading_process_output for examples of these situations.

   FROM_WHERE is an integer saying from where this function was
   called.  This is useful for debugging.  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn
redisplay_preserve_echo_area (from_where: c_int)
{
  redisplay_trace (c"redisplay_preserve_echo_area (%d)\n".as_ptr (), from_where);

  block_input ();
  let count: specpdl_ref = SPECPDL_INDEX ();
  record_unwind_protect_void (Some (unwind_redisplay_preserve_echo_area));
  block_buffer_flips ();
  unblock_input ();

  if !NILP (echo_area_buffer[1])
    {
      /* We have a previously displayed message, but no current
	 message.  Redisplay the previous message.  */
      display_last_displayed_message_p = true;
      redisplay_internal ();
      display_last_displayed_message_p = false;
    }
  else
    { redisplay_internal (); }

  flush_frame (SELECTED_FRAME ());
  unbind_to (count, Qnil ());
}


/* Function registered with record_unwind_protect in redisplay_internal.  */

unsafe extern "C" fn
unwind_redisplay ()
{
  redisplaying_p = false;
  unblock_buffer_flips ();
}

/* Function registered with record_unwind_protect before calling
   start_display outside of redisplay_internal.  */
#[unsafe(no_mangle)] pub unsafe extern "C" fn
unwind_display_working_on_window ()
{
  display_working_on_window_p = false;
}

/* Mark the display of leaf window W as accurate or inaccurate.
   If ACCURATE_P, mark display of W as accurate.
   If !ACCURATE_P, arrange for W to be redisplayed the next
   time redisplay_internal is called.  */

unsafe fn
mark_window_display_accurate_1 (w: *mut window, accurate_p: bool)
{
  let b: *mut buffer = XBUFFER ((*w).contents);
/* C 18307-18309's `#ifdef HAVE_TEXT_CONVERSION` declarations (`prev_point`,
     `prev_mark`) are dead on this target (P-08).  */

  (*w).last_modified = if accurate_p { BUF_MODIFF! (b) } else { 0 };
  (*w).last_overlay_modified = if accurate_p { BUF_OVERLAY_MODIFF! (b) } else { 0 };
  window_bits! (w, last_had_star, BUF_MODIFF! (b) > BUF_SAVE_MODIFF! (b));

  if accurate_p
    {
      buffer_bits! (clip_changed, b, false);
      buffer_bits! (prevent_redisplay_optimizations_p, b, false);
      eassert! (buffer_window_count (b) > 0);
      /* Resetting b->text->redisplay is problematic!
	 In order to make it safer to do it here, redisplay_internal must
	 have copied all b->text->redisplay to their respective windows.  */
      buffer_text_bits! (text.redisplay, b, false);

      BUF_UNCHANGED_MODIFIED! (b) = BUF_MODIFF! (b);
      BUF_OVERLAY_UNCHANGED_MODIFIED! (b) = BUF_OVERLAY_MODIFF! (b);
      BUF_BEG_UNCHANGED! (b) = BUF_GPT! (b) - BUF_BEG (b);
      BUF_END_UNCHANGED! (b) = BUF_Z! (b) - BUF_GPT! (b);

      (*((*w).current_matrix as *mut glyph_matrix)).buffer = b as *mut c_void;
      (*((*w).current_matrix as *mut glyph_matrix)).begv = BUF_BEGV (b);
      (*((*w).current_matrix as *mut glyph_matrix)).zv = BUF_ZV (b);
      glyph_matrix_bits! (header_line_p, (*w).current_matrix as *mut glyph_matrix, window_wants_header_line (w));
      glyph_matrix_bits! (tab_line_p, (*w).current_matrix as *mut glyph_matrix, window_wants_tab_line (w));

      (*w).last_cursor_vpos = (*w).cursor.vpos;
      window_bits! (w, last_cursor_off_p, window_bits! (w, cursor_off_p));

/* C 18339-18383's `#ifdef HAVE_TEXT_CONVERSION` block is dead on this target (P-08):
       the `prev_point`/`prev_mark` saves, the `ephemeral_last_point` copy and the
       `report_point_change` call (with its own long comment) are recorded, not
       transcribed.  */

      if w == XWINDOW (selected_window)
	{ (*w).last_point = BUF_PT (b); }
      else
	{ (*w).last_point = marker_position ((*w).pointm); }

      /* w->last_mark is recorded for text conversion purposes.
         Input methods aren't interested in the value of the mark
         if it is inactive, so set it to -1 if it's not.  */

      if (*XMARKER (BVAR! (b, mark_))).buffer as *mut buffer == b
	  && !NILP (BVAR! (b, mark_active_))
	{ (*w).last_mark = marker_position (BVAR! (b, mark_)); }
      else
	{ (*w).last_mark = -1; }


      let mut row: *mut glyph_row;
      /* These conditions should be consistent with CHECK_WINDOW_END.  */
      if (*w).window_end_vpos < (*((*w).current_matrix as *mut glyph_matrix)).nrows
	  && { row = MATRIX_ROW! ((*w).current_matrix as *mut glyph_matrix, (*w).window_end_vpos);
	       !glyph_row_bits! (enabled_p, row)
	       || MATRIX_ROW_DISPLAYS_TEXT_P! (row)
	       || MATRIX_ROW_VPOS! (row, (*w).current_matrix as *mut glyph_matrix) == 0 }
	{ window_bits! (w, window_end_valid, true); }
      window_bits! (w, update_mode_line, false);
      window_bits! (w, preserve_vscroll_p, false);
    }

  window_bits! (w, redisplay, !accurate_p);
}


/* Mark the display of windows in the window tree rooted at WINDOW as
   accurate or inaccurate.  If ACCURATE_P, mark display of
   windows as accurate.  If !ACCURATE_P, arrange for windows to
   be redisplayed the next time redisplay_internal is called.  */

#[unsafe(no_mangle)] pub unsafe extern "C" fn
mark_window_display_accurate (mut window: Lisp_Object, accurate_p: bool)
{
  let mut w: *mut window;

  while !NILP (window)
    {
      w = XWINDOW (window);
      if WINDOWP ((*w).contents)
	{ mark_window_display_accurate ((*w).contents, accurate_p); }
      else
	{ mark_window_display_accurate_1 (w, accurate_p); }
      window = (*w).next;
    }

  if accurate_p
    { update_overlay_arrows (1); }
  else
    /* Force a thorough redisplay the next time by setting
       last_arrow_position and last_arrow_string to t, which is
       unequal to any useful value of Voverlay_arrow_...  */
    { update_overlay_arrows (-1); }
}


/* Return value in display table DP (Lisp_Char_Table *) for character
   C.  Since a display table doesn't have any parent, we don't have to
   follow parent.  Do not call this function directly but use the
   macro DISP_CHAR_VECTOR.  */

/* disptab.h 55 declares `disp_char_vector` at its mirror top level, and a unit that
   defines it collides (E0428, the `tty_handle_tab_bar_click` family).  The definition
   therefore lives in this module and is glob-imported — the shape the mirrors use for a
   name a unit defines (`window_redisplay_decls`); the emitted symbol is unchanged.  */
mod disp_char_vector_decl {
  use super::*;

  #[unsafe(no_mangle)] pub unsafe extern "C" fn
  disp_char_vector (dp: *mut Lisp_Char_Table, c: c_int) -> Lisp_Object
  {
    let mut val: Lisp_Object;

    if ASCII_CHAR_P (c as isize)
      {
        val = (*dp).ascii;
        if SUB_CHAR_TABLE_P (val)
  	{ val = (*XSUB_CHAR_TABLE (val)).contents[c as usize]; }
      }
    else
      {
        let mut table: Lisp_Object;

        XSETCHAR_TABLE! (table, dp);
        val = char_table_ref (table, c);
      }
    if NILP (val)
      { val = (*dp).defalt; }
    return val;
  }
}
use disp_char_vector_decl::*;

static mut buffer_flip_blocked_depth: c_int = 0;

unsafe fn
block_buffer_flips ()
{
  eassert! (buffer_flip_blocked_depth >= 0);
  buffer_flip_blocked_depth += 1;
}

unsafe fn
unblock_buffer_flips ()
{
  eassert! (buffer_flip_blocked_depth > 0);
  buffer_flip_blocked_depth -= 1;
  if buffer_flip_blocked_depth == 0
    {
      let mut tail: Lisp_Object;
      let mut frame: Lisp_Object;
      block_input ();
      FOR_EACH_FRAME! (tail, frame, {
        {
	  let f: *mut frame = XFRAME (frame);
	  if !(*FRAME_TERMINAL! (f)).buffer_flipping_unblocked_hook.is_none ()
	    { (*FRAME_TERMINAL! (f)).buffer_flipping_unblocked_hook.unwrap () (f as *mut c_void); }
	}
	});
      unblock_input ();
    }
}

#[unsafe(no_mangle)] pub unsafe extern "C" fn
buffer_flipping_blocked_p () -> bool
{
  return buffer_flip_blocked_depth > 0;
}


/***********************************************************************
			   Window Redisplay
 ***********************************************************************/

/* Redisplay all leaf windows in the window tree rooted at WINDOW.  */

unsafe fn
redisplay_windows (mut window: Lisp_Object)
{
  while !NILP (window)
    {
      let w: *mut window = XWINDOW (window);

      if WINDOWP ((*w).contents)
	{ redisplay_windows ((*w).contents); }
      else if BUFFERP ((*w).contents)
	{
	  displayed_buffer = XBUFFER ((*w).contents);
	  /* Use list_of_error, not Qerror, so that
	     we catch only errors and don't run the debugger.  */
	  internal_condition_case_1 (Some (redisplay_window_0), window,
				     list_of_error,
				     Some (redisplay_window_error));
	}

      window = (*w).next;
    }
}

unsafe extern "C" fn
redisplay_window_error (error_data: Lisp_Object) -> Lisp_Object
{
  (*displayed_buffer).display_error_modiff = BUF_MODIFF! (displayed_buffer);

  /* When in redisplay, the error is captured and not shown.  Arrange
     for it to be shown later.  */
  if max_redisplay_ticks > 0
      && CONSP (error_data)
      && EQ (XCAR (error_data), Qerror ())
      && CONSP (XCDR (error_data))
      && STRINGP (XCAR (XCDR (error_data)))
    { Vdelayed_warnings_list = Fcons (list2 (XCAR (error_data),
					   XCAR (XCDR (error_data))),
				    Vdelayed_warnings_list); }
  return Qnil ();
}

unsafe extern "C" fn
redisplay_window_0 (window: Lisp_Object) -> Lisp_Object
{
  if (*displayed_buffer).display_error_modiff < BUF_MODIFF! (displayed_buffer)
    { redisplay_window (window, false); }
  return Qnil ();
}

unsafe extern "C" fn
redisplay_window_1 (window: Lisp_Object) -> Lisp_Object
{
  if (*displayed_buffer).display_error_modiff < BUF_MODIFF! (displayed_buffer)
    { redisplay_window (window, true); }
  return Qnil ();
}


/***********************************************************************
		      Aborting runaway redisplay
 ***********************************************************************/

/* Update the redisplay-tick count for window W, and signal an error
   if the tick count is above some threshold, indicating that
   redisplay of the window takes "too long".

   TICKS is the amount of ticks to add to the W's current count; zero
   means to initialize the tick count to zero.

   W can be NULL if TICKS is zero: that means unconditionally
   re-initialize the current tick count to zero.

   W can also be NULL if the caller doesn't know which window is being
   processed by the display code.  In that case, if TICKS is non-zero,
   we assume it's the last window that shows the current buffer.  */
#[unsafe(no_mangle)] pub unsafe extern "C" fn
update_redisplay_ticks (ticks: c_int, mut w: *mut window)
{
  /* This keeps track of the window on which redisplay is working.  */
  mod update_redisplay_ticks_caches {
    use super::*;

    pub static mut cwindow: *mut window = core::ptr::null_mut ();
    pub static mut window_ticks: EMACS_INT = 0;
  }
  use update_redisplay_ticks_caches::*;

  /* We only initialize the count if this is a different window or
     NULL.  Otherwise, this is a call from init_iterator for the same
     window we tracked before, and we should keep the count.  */
  if ticks == 0 && w != cwindow
    {
      cwindow = w;
      window_ticks = 0;
    }
  /* Some callers can be run in contexts unrelated to display code, so
     don't abort them and don't update the tick count in those cases.  */
  if (w.is_null () && !redisplaying_p && !display_working_on_window_p
      /* We never disable redisplay of a mini-window, since that is
	 absolutely essential for communicating with Emacs.  */
      || (!w.is_null () && MINI_WINDOW_P (w)))
    { return; }

  if ticks > 0
    { window_ticks += ticks as EMACS_INT; }
  if max_redisplay_ticks > 0 && window_ticks > max_redisplay_ticks as EMACS_INT
    {
      /* In addition to a buffer, this could be a window (for non-leaf
	 windows, not expected here) or nil (for pseudo-windows like
	 the one used for the native tool bar).  */
      let contents: Lisp_Object = if !w.is_null () { (*w).contents } else { Qnil () };
      let bufname: *const c_char
	= if NILP (contents)
	{ SSDATA (BVAR! (current_buffer, name_)) }
	else if BUFFERP (contents)
	   { SSDATA (BVAR! (XBUFFER (contents), name_)) }
	   else { c"<unknown>".as_ptr () };

      windows_or_buffers_changed = 177;
      /* scrolling_window depends too much on the glyph matrices being
	 correct, and we cannot guarantee that if we abort the
	 redisplay of this window.  */
      if !w.is_null () && !(*w).desired_matrix.is_null ()
	{ glyph_matrix_bits! (no_scrolling_p, (*w).desired_matrix as *mut glyph_matrix, true); }
      error (c"Window showing buffer %s takes too long to redisplay".as_ptr (), bufname);
    }
}

/* Set cursor position of W.  PT is assumed to be displayed in ROW.
   DELTA and DELTA_BYTES are the numbers of characters and bytes by
   which positions recorded in ROW differ from current buffer
   positions.

   Return true iff cursor is on this row.  */

unsafe fn
set_cursor_from_row (w: *mut window, row: *mut glyph_row,
		     matrix: *mut glyph_matrix,
		     delta: ptrdiff_t, delta_bytes: ptrdiff_t,
		     dy: c_int, dvpos: c_int) -> bool
{
  let mut glyph: *mut glyph = (*row).glyphs[TEXT_AREA as usize];
  let mut end: *mut glyph = glyph.offset ((*row).used[TEXT_AREA as usize] as isize);
  let mut cursor: *mut glyph = core::ptr::null_mut ();
  /* The last known character position in row.  */
  let last_pos: ptrdiff_t = MATRIX_ROW_START_CHARPOS! (row) + delta;
  let mut x: c_int = (*row).x;
  let pt_old: ptrdiff_t = PT! () - delta;
  let mut pos_before: ptrdiff_t = MATRIX_ROW_START_CHARPOS! (row) + delta;
  let mut pos_after: ptrdiff_t = MATRIX_ROW_END_CHARPOS! (row) + delta;
  let mut glyph_before: *mut glyph = glyph.offset (-1);
  let mut glyph_after: *mut glyph = end;
  /* A glyph beyond the edge of TEXT_AREA which we should never
     touch.  */
  let mut glyphs_end: *mut glyph = end;
  /* True means we've found a match for cursor position, but that
     glyph has the avoid_cursor_p flag set.  */
  let mut match_with_avoid_cursor: bool = false;
  /* True means we've seen at least one glyph that came from a
     display string.  */
  let mut string_seen: bool = false;
  /* Largest and smallest buffer positions seen so far during scan of
     glyph row.  */
  let mut bpos_max: ptrdiff_t = pos_before;
  let mut bpos_min: ptrdiff_t = pos_after;
  /* Last buffer position covered by an overlay string with an integer
     `cursor' property.  */
  let mut bpos_covered: ptrdiff_t = 0;
  /* True means the display string on which to display the cursor
     comes from a text property, not from an overlay.  */
  let mut string_from_text_prop: bool = false;

  /* Don't even try doing anything if called for a mode-line or
     header-line or tab-line row, since the rest of the code isn't
     prepared to deal with such calamities.  */
  eassert! (!glyph_row_bits! (mode_line_p, row));
  if glyph_row_bits! (mode_line_p, row)
    { return false; }

  /* Skip over glyphs not having an object at the start and the end of
     the row.  These are special glyphs like truncation marks on
     terminal frames.  */
  if MATRIX_ROW_DISPLAYS_TEXT_P! (row)
    {
      if !glyph_row_bits! (reversed_p, row)
	{
	  while glyph < end
		 && NILP ((*glyph).object)
		 && (*glyph).charpos < 0
	    {
	      x += (*glyph).pixel_width as c_int;
	      glyph = glyph.offset (1);
	    }
	  while end > glyph
		 && NILP ((*(end.offset (-1))).object)
		 /* CHARPOS is zero for blanks and stretch glyphs
		    inserted by extend_face_to_end_of_line.  */
		 && (*(end.offset (-1))).charpos <= 0
	    { end = end.offset (-1); }
	  glyph_before = glyph.offset (-1);
	  glyph_after = end;
	}
      else
	{
	  let mut g: *mut glyph;

	  /* If the glyph row is reversed, we need to process it from back
	     to front, so swap the edge pointers.  */
	  glyphs_end = end.offset (-1);
  end = end.offset (-1);
	  glyph = glyph.offset ((*row).used[TEXT_AREA as usize] as isize - 1);

	  while glyph > end.offset (1)
		 && NILP ((*glyph).object)
		 && (*glyph).charpos < 0
	    { glyph = glyph.offset (-1); }
	  if NILP ((*glyph).object) && (*glyph).charpos < 0
	    { glyph = glyph.offset (-1); }
	  /* By default, in reversed rows we put the cursor on the
	     rightmost (first in the reading order) glyph.  */
	  x = (*row).x;
  g = end.offset (1);
  while g < glyph
	    { x += (*g).pixel_width as c_int; g = g.offset (1); }
	  while end < glyph
		 && NILP ((*(end.offset (1))).object)
		 && (*(end.offset (1))).charpos <= 0
	    { end = end.offset (1); }
	  glyph_before = glyph.offset (1);
	  glyph_after = end;
	}
    }
  else if glyph_row_bits! (reversed_p, row)
    {
      /* In R2L rows that don't display text, put the cursor on the
	 rightmost glyph.  Case in point: an empty last line that is
	 part of an R2L paragraph.  */
      cursor = end.offset (-1);
      /* Avoid placing the cursor on the last glyph of the row, where
	 on terminal frames we hold the vertical border between
	 adjacent windows.  */
      if !FRAME_WINDOW_P (WINDOW_XFRAME! (w))
	  && !WINDOW_RIGHTMOST_P! (w)
	  && cursor == (*row).glyphs[LAST_AREA as usize].offset (-1)
	{ cursor = cursor.offset (-1); }
      x = -1;	/* will be computed below, at label compute_x */
    }

  /* Step 1: Try to find the glyph whose character position
     corresponds to point.  If that's not possible, find 2 glyphs
     whose character positions are the closest to point, one before
     point, the other after it.  */
  if !glyph_row_bits! (reversed_p, row)
    { while glyph < end
	   && !NILP ((*glyph).object)
	   /* glyph was not inserted by redisplay for internal purposes */
      {
	if BUFFERP ((*glyph).object)
	  {
	    let dpos: ptrdiff_t = (*glyph).charpos - pt_old;

	    if (*glyph).charpos > bpos_max
	      { bpos_max = (*glyph).charpos; }
	    if (*glyph).charpos < bpos_min
	      { bpos_min = (*glyph).charpos; }
	    if !glyph_bits! (avoid_cursor_p, *glyph)
	      {
		/* If we hit point, we've found the glyph on which to
		   display the cursor.  */
		if dpos == 0
		  {
		    match_with_avoid_cursor = false;
		    break;
		  }
		/* See if we've found a better approximation to
		   POS_BEFORE or to POS_AFTER.  */
		if 0 > dpos && dpos > pos_before - pt_old
		  {
		    pos_before = (*glyph).charpos;
		    glyph_before = glyph;
		  }
		else if 0 < dpos && dpos < pos_after - pt_old
		  {
		    pos_after = (*glyph).charpos;
		    glyph_after = glyph;
		  }
	      }
	    else if dpos == 0
	      { match_with_avoid_cursor = true; }
	  }
	else if STRINGP ((*glyph).object)
		 && !glyph_bits! (avoid_cursor_p, *glyph)
	  {
	    let chprop: Lisp_Object;
	    let glyph_pos: ptrdiff_t = (*glyph).charpos;

	    chprop = Fget_char_property (make_fixnum (glyph_pos), Qcursor (),
					 (*glyph).object);
	    if !NILP (chprop)
	      {
		/* If the string came from a `display' text property,
		   look up the buffer position of that property and
		   use that position to update bpos_max, as if we
		   actually saw such a position in one of the row's
		   glyphs.  This helps with supporting integer values
		   of `cursor' property on the display string in
		   situations where most or all of the row's buffer
		   text is completely covered by display properties,
		   so that no glyph with valid buffer positions is
		   ever seen in the row.  */
		let prop_pos: ptrdiff_t =
		  string_buffer_position_lim ((*glyph).object, pos_before,
					      pos_after, false);

		if prop_pos >= pos_before
		  { bpos_max = prop_pos; }
	      }
	    if FIXNUMP (chprop)
	      {
		bpos_covered = bpos_max + XFIXNUM (chprop);
		/* If the `cursor' property covers buffer positions up
		   to and including point, we should display cursor on
		   this glyph.  Note that, if a `cursor' property on one
		   of the string's characters has an integer value, we
		   will break out of the loop below _before_ we get to
		   the position match above.  IOW, integer values of
		   the `cursor' property override the "exact match for
		   point" strategy of positioning the cursor.  */
		/* Implementation note: bpos_max == pt_old when, e.g.,
		   we are in an empty line, where bpos_max is set to
		   MATRIX_ROW_START_CHARPOS, see above.  */
		if bpos_max <= pt_old && bpos_covered >= pt_old
		  {
		    cursor = glyph;
		    break;
		  }
	      }

	    string_seen = true;
	  }
	x += (*glyph).pixel_width as c_int;
	glyph = glyph.offset (1);
      }
  }
  else if glyph > end	/* row is reversed */
    { while !NILP ((*glyph).object)
      {
	if BUFFERP ((*glyph).object)
	  {
	    let dpos: ptrdiff_t = (*glyph).charpos - pt_old;

	    if (*glyph).charpos > bpos_max
	      { bpos_max = (*glyph).charpos; }
	    if (*glyph).charpos < bpos_min
	      { bpos_min = (*glyph).charpos; }
	    if !glyph_bits! (avoid_cursor_p, *glyph)
	      {
		if dpos == 0
		  {
		    match_with_avoid_cursor = false;
		    break;
		  }
		if 0 > dpos && dpos > pos_before - pt_old
		  {
		    pos_before = (*glyph).charpos;
		    glyph_before = glyph;
		  }
		else if 0 < dpos && dpos < pos_after - pt_old
		  {
		    pos_after = (*glyph).charpos;
		    glyph_after = glyph;
		  }
	      }
	    else if dpos == 0
	      { match_with_avoid_cursor = true; }
	  }
	else if STRINGP ((*glyph).object)
	  {
	    let chprop: Lisp_Object;
	    let glyph_pos: ptrdiff_t = (*glyph).charpos;

	    chprop = Fget_char_property (make_fixnum (glyph_pos), Qcursor (),
					 (*glyph).object);
	    if !NILP (chprop)
	      {
		let prop_pos: ptrdiff_t =
		  string_buffer_position_lim ((*glyph).object, pos_before,
					      pos_after, false);

		if prop_pos >= pos_before
		  { bpos_max = prop_pos; }
	      }
	    if FIXNUMP (chprop)
	      {
		bpos_covered = bpos_max + XFIXNUM (chprop);
		/* If the `cursor' property covers buffer positions up
		   to and including point, we should display cursor on
		   this glyph.  */
		if bpos_max <= pt_old && bpos_covered >= pt_old
		  {
		    cursor = glyph;
		    break;
		  }
	      }
	    string_seen = true;
	  }
	glyph = glyph.offset (-1);
	if glyph == glyphs_end /* don't dereference outside TEXT_AREA */
	  {
	    x -= 1;		/* can't use any pixel_width */
	    break;
	  }
	x -= (*glyph).pixel_width as c_int;
    }
  }

  /* Step 2: If we didn't find an exact match for point, we need to
     look for a proper place to put the cursor among glyphs between
     GLYPH_BEFORE and GLYPH_AFTER.  */
  'compute_x: {
  if !((if glyph_row_bits! (reversed_p, row) { glyph > glyphs_end } else { glyph < glyphs_end })
	&& BUFFERP ((*glyph).object) && (*glyph).charpos == pt_old)
      && !(bpos_max <= pt_old && pt_old <= bpos_covered)
    {
      /* An empty line has a single glyph whose OBJECT is nil and
	 whose CHARPOS is the position of a newline on that line.
	 Note that on a TTY, there are more glyphs after that, which
	 were produced by extend_face_to_end_of_line, but their
	 CHARPOS is zero or negative.  */
      let empty_line_p: bool =
	((if glyph_row_bits! (reversed_p, row) { glyph > glyphs_end } else { glyph < glyphs_end })
	 && NILP ((*glyph).object) && (*glyph).charpos > 0
	 /* On a TTY, continued and truncated rows also have a glyph at
	    their end whose OBJECT is nil and whose CHARPOS is
	    positive (the continuation and truncation glyphs), but such
	    rows are obviously not "empty".  */
	 && !(glyph_row_bits! (continued_p, row) || glyph_row_bits! (truncated_on_right_p, row)));

      if glyph_row_bits! (ends_in_ellipsis_p, row) && pos_after == last_pos
	{
	  let mut ellipsis_pos: ptrdiff_t;

	  /* Scan back over the ellipsis glyphs.  */
	  if !glyph_row_bits! (reversed_p, row)
	    {
	      ellipsis_pos = (*(glyph.offset (-1))).charpos;
	      while glyph > (*row).glyphs[TEXT_AREA as usize]
		     && (*(glyph.offset (-1))).charpos == ellipsis_pos
		{ glyph = glyph.offset (-1); x -= (*glyph).pixel_width as c_int; }
	      /* That loop always goes one position too far, including
		 the glyph before the ellipsis.  So scan forward over
		 that one.  */
	      x += (*glyph).pixel_width as c_int;
	      glyph = glyph.offset (1);
	    }
	  else	/* row is reversed */
	    {
	      ellipsis_pos = (*(glyph.offset (1))).charpos;
	      while glyph < (*row).glyphs[TEXT_AREA as usize].offset ((*row).used[TEXT_AREA as usize] as isize - 1)
		     && (*(glyph.offset (1))).charpos == ellipsis_pos
		{ glyph = glyph.offset (1); x += (*glyph).pixel_width as c_int; }
	      x -= (*glyph).pixel_width as c_int;
	      glyph = glyph.offset (-1);
	    }
	}
      else if match_with_avoid_cursor
	{
	  cursor = glyph_after;
	  x = -1;
	}
      else if string_seen
	{
	  let incr: isize = if glyph_row_bits! (reversed_p, row) { -1 } else { 1 };

	  /* Need to find the glyph that came out of a string which is
	     present at point.  That glyph is somewhere between
	     GLYPH_BEFORE and GLYPH_AFTER, and it came from a string
	     positioned between POS_BEFORE and POS_AFTER in the
	     buffer.  */
	  let mut start: *mut glyph;
  let mut stop: *mut glyph;
	  let mut pos: ptrdiff_t = pos_before;

	  x = -1;

	  /* If the row ends in a newline from a display string,
	     reordering could have moved the glyphs belonging to the
	     string out of the [GLYPH_BEFORE..GLYPH_AFTER] range.  So
	     in this case we extend the search to the last glyph in
	     the row that was not inserted by redisplay.  */
	  if glyph_row_bits! (ends_in_newline_from_string_p, row)
	    {
	      glyph_after = end;
	      pos_after = MATRIX_ROW_END_CHARPOS! (row) + delta;
	    }

	  /* GLYPH_BEFORE and GLYPH_AFTER are the glyphs that
	     correspond to POS_BEFORE and POS_AFTER, respectively.  We
	     need START and STOP in the order that corresponds to the
	     row's direction as given by its reversed_p flag.  If the
	     directionality of characters between POS_BEFORE and
	     POS_AFTER is the opposite of the row's base direction,
	     these characters will have been reordered for display,
	     and we need to reverse START and STOP.  */
	  if !glyph_row_bits! (reversed_p, row)
	    {
	      start = std::cmp::min (glyph_before, glyph_after);
	      stop = std::cmp::max (glyph_before, glyph_after);
	    }
	  else
	    {
	      start = std::cmp::max (glyph_before, glyph_after);
	      stop = std::cmp::min (glyph_before, glyph_after);
	    }
	  glyph = start.offset (incr);
	       while if glyph_row_bits! (reversed_p, row) { glyph > stop } else { glyph < stop }
	    {

	      /* Any glyphs that come from the buffer are here because
		 of bidi reordering.  Skip them, and only pay
		 attention to glyphs that came from some string.  */
	      if STRINGP ((*glyph).object)
		  && !glyph_bits! (avoid_cursor_p, *glyph)
		{
		  let str: Lisp_Object;
		  let mut tem: ptrdiff_t;
		  /* If the display property covers the newline, we
		     need to search for it one position farther.  */
		  let lim: ptrdiff_t = pos_after
		    + (if pos_after == MATRIX_ROW_END_CHARPOS! (row) + delta { 1 } else { 0 });

		  string_from_text_prop = false;
		  str = (*glyph).object;
		  tem = string_buffer_position_lim (str, pos, lim, false);
		  if tem == 0	/* from overlay */
		      || pos <= tem
		    {
		      /* If the string from which this glyph came is
			 found in the buffer at point, or at position
			 that is closer to point than pos_after, then
			 we've found the glyph we've been looking for.
			 If it comes from an overlay (tem == 0), and
			 it has the `cursor' property on one of its
			 glyphs, record that glyph as a candidate for
			 displaying the cursor.  (As in the
			 unidirectional version, we will display the
			 cursor on the last candidate we find.)  */
		      if tem == 0
			  || tem == pt_old
			  || (tem - pt_old > 0 && tem < pos_after)
			{
			  /* The glyphs from this string could have
			     been reordered.  Find the one with the
			     smallest string position.  Or there could
			     be a character in the string with the
			     `cursor' property, which means display
			     cursor on that character's glyph.  */
			  let mut strpos: ptrdiff_t = (*glyph).charpos;

			  if tem != 0
			    {
			      cursor = glyph;
			      string_from_text_prop = true;
			    }
			  while (if glyph_row_bits! (reversed_p, row) { glyph > stop } else { glyph < stop })
			       && EQ ((*glyph).object, str)
			    {
			      let cprop: Lisp_Object;
			      let gpos: ptrdiff_t = (*glyph).charpos;

			      cprop = Fget_char_property (make_fixnum (gpos),
							  Qcursor (),
							  (*glyph).object);
			      if !NILP (cprop)
				{
				  cursor = glyph;
				  break;
				}
			      if tem != 0 && (*glyph).charpos < strpos
				{
				  strpos = (*glyph).charpos;
				  cursor = glyph;
				}
			    glyph = glyph.offset (incr);
			    }

			  if tem == pt_old
			      || (tem - pt_old > 0 && tem < pos_after)
			    { break 'compute_x; }
			}
		      if tem != 0
			{ pos = tem + 1; } /* don't find previous instances */
		    }
		  /* This string is not what we want; skip all of the
		     glyphs that came from it.  */
		  while (if glyph_row_bits! (reversed_p, row) { glyph > stop } else { glyph < stop })
			 && EQ ((*glyph).object, str)
		    { glyph = glyph.offset (incr); }
		}
	      else
		{ glyph = glyph.offset (incr); }
	    }

	  /* If we reached the end of the line, and END was from a string,
	     the cursor is not on this line.  */
	  if cursor.is_null ()
	      && (if glyph_row_bits! (reversed_p, row) { glyph <= end } else { glyph >= end })
	      && (if glyph_row_bits! (reversed_p, row) { end > glyphs_end } else { end < glyphs_end })
	      && STRINGP ((*end).object)
	      && glyph_row_bits! (continued_p, row)
	    { return false; }
	}
      /* A truncated row may not include PT among its character positions.
	 Setting the cursor inside the scroll margin will trigger
	 recalculation of hscroll in hscroll_window_tree.  But if a
	 display string covers point, defer to the string-handling
	 code below to figure this out.  */
      else if glyph_row_bits! (truncated_on_left_p, row) && pt_old < bpos_min
	{
	  cursor = glyph_before;
	  x = -1;
	}
      else if (glyph_row_bits! (truncated_on_right_p, row) && pt_old > bpos_max)
	       /* Zero-width characters produce no glyphs.  */
	       || (!empty_line_p
		   && (if glyph_row_bits! (reversed_p, row)
		       { glyph_after > glyphs_end }
		       else { glyph_after < glyphs_end }))
	{
	  cursor = glyph_after;
	  x = -1;
	}
    }
  }

 /* compute_x:  (C 19123 — the label has no Rust site; `break 'compute_x` above is
     C's `goto compute_x`, and the fall-through out of the block lands here too.)  */
  if !cursor.is_null ()
    { glyph = cursor; }
  else if glyph == glyphs_end
	   && pos_before == pos_after
	   && STRINGP ((*(if glyph_row_bits! (reversed_p, row)
			{ (*row).glyphs[TEXT_AREA as usize].offset ((*row).used[TEXT_AREA as usize] as isize - 1) }
			else { (*row).glyphs[TEXT_AREA as usize] })).object)
    {
      /* If all the glyphs of this row came from strings, put the
	 cursor on the first glyph of the row.  This avoids having the
	 cursor outside of the text area in this very rare and hard
	 use case.  */
      glyph =
	if glyph_row_bits! (reversed_p, row)
	{ (*row).glyphs[TEXT_AREA as usize].offset ((*row).used[TEXT_AREA as usize] as isize - 1) }
	else { (*row).glyphs[TEXT_AREA as usize] };
    }
  if x < 0
    {
      let mut g: *mut glyph;

      /* Need to compute x that corresponds to GLYPH.  */
      g = (*row).glyphs[TEXT_AREA as usize];
  x = (*row).x;
  while g < glyph
	{
	  if g >= (*row).glyphs[TEXT_AREA as usize].offset ((*row).used[TEXT_AREA as usize] as isize)
	    { emacs_abort (); }
	  x += (*g).pixel_width as c_int;
	g = g.offset (1);
    }
    }

  /* ROW could be part of a continued line, which, under bidi
     reordering, might have other rows whose start and end charpos
     occlude point.  Only set w->cursor if we found a better
     approximation to the cursor position than we have from previously
     examined candidate rows belonging to the same continued line.  */
  if /* We already have a candidate row.  */
      (*w).cursor.vpos >= 0
      /* That candidate is not the row we are processing.  */
      && MATRIX_ROW! (matrix, (*w).cursor.vpos) != row
      /* Make sure cursor.vpos specifies a row whose start and end
	 charpos occlude point, and it is valid candidate for being a
	 cursor-row.  This is because some callers of this function
	 leave cursor.vpos at the row where the cursor was displayed
	 during the last redisplay cycle.  */
      && MATRIX_ROW_START_CHARPOS! (MATRIX_ROW! (matrix, (*w).cursor.vpos)) <= pt_old
      && pt_old <= MATRIX_ROW_END_CHARPOS! (MATRIX_ROW! (matrix, (*w).cursor.vpos))
      && cursor_row_p (MATRIX_ROW! (matrix, (*w).cursor.vpos))
    {
      let g1: *mut glyph
	= MATRIX_ROW_GLYPH_START! (matrix, (*w).cursor.vpos).offset ((*w).cursor.hpos as isize);

      /* Don't consider glyphs that are outside TEXT_AREA.  */
      if !(if glyph_row_bits! (reversed_p, row) { glyph > glyphs_end } else { glyph < glyphs_end })
	{ return false; }
      /* Keep the candidate whose buffer position is the closest to
	 point or has the `cursor' property.  */
      if /* Previous candidate is a glyph in TEXT_AREA of that row.  */
	  (*w).cursor.hpos >= 0
	  && (*w).cursor.hpos < MATRIX_ROW_USED! (matrix, (*w).cursor.vpos) as c_int
	  && ((BUFFERP ((*g1).object)
	       && ((*g1).charpos == pt_old /* An exact match always wins.  */
		   || (BUFFERP ((*glyph).object)
		       && eabs! ((*g1).charpos - pt_old)
		       < eabs! ((*glyph).charpos - pt_old))))
	      /* Previous candidate is a glyph from a string that has
		 a non-nil `cursor' property.  */
	      || (STRINGP ((*g1).object)
		  && (!NILP (Fget_char_property (make_fixnum ((*g1).charpos),
						Qcursor (), (*g1).object))
		      /* Previous candidate is from the same display
			 string as this one, and the display string
			 came from a text property.  */
		      || (EQ ((*g1).object, (*glyph).object)
			  && string_from_text_prop)
		      /* this candidate is from newline and its
			 position is not an exact match */
		      || (NILP ((*glyph).object)
			  && (*glyph).charpos != pt_old))))
	{ return false; }
      /* If this candidate gives an exact match, use that.  */
      if !((BUFFERP ((*glyph).object) && (*glyph).charpos == pt_old)
	    /* If this candidate is a glyph created for the
	       terminating newline of a line, and point is on that
	       newline, it wins because it's an exact match.  */
	    || (!glyph_row_bits! (continued_p, row)
		&& NILP ((*glyph).object)
		&& (*glyph).charpos == 0
		&& pt_old == MATRIX_ROW_END_CHARPOS! (row) - 1))
	  /* Otherwise, keep the candidate that comes from a row
	     spanning less buffer positions.  This may win when one or
	     both candidate positions are on glyphs that came from
	     display strings, for which we cannot compare buffer
	     positions.  */
	  && MATRIX_ROW_END_CHARPOS! (MATRIX_ROW! (matrix, (*w).cursor.vpos))
	     - MATRIX_ROW_START_CHARPOS! (MATRIX_ROW! (matrix, (*w).cursor.vpos))
	     < MATRIX_ROW_END_CHARPOS! (row) - MATRIX_ROW_START_CHARPOS! (row)
	{ return false; }
    }
  (*w).cursor.hpos = glyph.offset_from ((*row).glyphs[TEXT_AREA as usize]) as c_int;
  (*w).cursor.x = x;
  (*w).cursor.vpos = MATRIX_ROW_VPOS! (row, matrix) as c_int + dvpos;
  (*w).cursor.y = (*row).y + dy;

  if w == XWINDOW (selected_window)
    {
      if !glyph_row_bits! (continued_p, row)
	  && !MATRIX_ROW_CONTINUATION_LINE_P! (row)
	  && (*row).x == 0
	{
	  this_line_buffer = XBUFFER ((*w).contents);

	  CHARPOS! (this_line_start_pos)
	    = MATRIX_ROW_START_CHARPOS! (row) + delta;
	  BYTEPOS! (this_line_start_pos)
	    = MATRIX_ROW_START_BYTEPOS! (row) + delta_bytes;

	  CHARPOS! (this_line_end_pos)
	    = Z! () - (MATRIX_ROW_END_CHARPOS! (row) + delta);
	  BYTEPOS! (this_line_end_pos)
	    = Z_BYTE! () - (MATRIX_ROW_END_BYTEPOS! (row) + delta_bytes);

	  this_line_y = (*w).cursor.y;
	  this_line_pixel_height = (*row).height as c_int;
	  this_line_vpos = (*w).cursor.vpos;
	  this_line_start_x = (*row).x;
	}
      else
	{ CHARPOS! (this_line_start_pos) = 0; }
    }

  return true;
}


/* Run window scroll functions, if any, for WINDOW with new window
   start STARTP.  Sets the window start of WINDOW to that position.

   We assume that the window's buffer is really current.  */

unsafe fn
run_window_scroll_functions (window: Lisp_Object, mut startp: text_pos) -> text_pos
{
  let w: *mut window = XWINDOW (window);
  SET_MARKER_FROM_TEXT_POS! ((*w).start, startp);

  eassert! (current_buffer == XBUFFER ((*w).contents));

  if !NILP (Vwindow_scroll_functions)
    {
      safe_run_hooks_2
	(Qwindow_scroll_functions (), window, make_fixnum (CHARPOS! (startp)));
      SET_TEXT_POS_FROM_MARKER! (startp, (*w).start);
      /* In case the hook functions switch buffers.  */
      set_buffer_internal (XBUFFER ((*w).contents));
    }

  return startp;
}


/* Make sure the line containing the cursor is fully visible.
   A value of true means there is nothing to be done.
   (Either the line is fully visible, or it cannot be made so,
   or we cannot tell.)

   If FORCE_P, return false even if partial visible cursor row
   is higher than window.

   If CURRENT_MATRIX_P, use the information from the
   window's current glyph matrix; otherwise use the desired glyph
   matrix.

   If JUST_TEST_USER_PREFERENCE_P, just test what the value of
   make-cursor-row-fully-visible requires, don't test the actual
   cursor position.  The assumption is that in that case the caller
   performs the necessary testing of the cursor position.

   A value of false means the caller should do scrolling
   as if point had gone off the screen.  */

unsafe fn
cursor_row_fully_visible_p (w: *mut window, force_p: bool,
			    current_matrix_p: bool,
			    just_test_user_preference_p: bool) -> bool
{
  let mut matrix: *mut glyph_matrix;
  let mut row: *mut glyph_row;
  let mut window_height: c_int;
  let mut mclfv_p: Lisp_Object =
    buffer_local_value (Qmake_cursor_line_fully_visible (), (*w).contents);

  /* If no local binding, use the global value.  */
  if BASE_EQ (mclfv_p, Qunbound ())
    { mclfv_p = Vmake_cursor_line_fully_visible; }
  /* Follow mode sets the variable to a Lisp function in buffers that
     are under Follow mode.  */
  if FUNCTIONP (mclfv_p)
    {
      let window: Lisp_Object;
      XSETWINDOW! (window, w);
      /* Implementation note: if the function we call here signals an
	 error, we will NOT scroll when the cursor is partially-visible.  */
      let val: Lisp_Object = dsafe_call1 (mclfv_p, window);
      if NILP (val)
	{ return true; }
      else if just_test_user_preference_p
	{ return false; }
    }
  else if NILP (mclfv_p)
    { return true; }
  else if just_test_user_preference_p
    { return false; }

  /* It's not always possible to find the cursor, e.g, when a window
     is full of overlay strings.  Don't do anything in that case.  */
  if (*w).cursor.vpos < 0
    { return true; }

  matrix = if current_matrix_p { (*w).current_matrix as *mut glyph_matrix }
    else { (*w).desired_matrix as *mut glyph_matrix };
  row = MATRIX_ROW! (matrix, (*w).cursor.vpos);

  /* If the cursor row is not partially visible, there's nothing to do.  */
  if !MATRIX_ROW_PARTIALLY_VISIBLE_P! (w, row)
    { return true; }

  /* If the row the cursor is in is taller than the window's height,
     it's not clear what to do, so do nothing.  */
  window_height = window_box_height (w);
  if (*row).height >= window_height
    {
      if !force_p || MINI_WINDOW_P (w)
	  || (*w).vscroll != 0 || (*w).cursor.vpos == 0
	{ return true; }
    }
  return false;
}


/* Try scrolling PT into view in window WINDOW.  JUST_THIS_ONE_P
   means only WINDOW is redisplayed in redisplay_internal.
   TEMP_SCROLL_STEP has the same meaning as emacs_scroll_step, and is used
   in redisplay_window to bring a partially visible line into view in
   the case that only the cursor has moved.

   LAST_LINE_MISFIT should be true if we're scrolling because the
   last screen line's vertical height extends past the end of the screen.

   Value is

   1	if scrolling succeeded

   0	if scrolling didn't find point.

   -1	if new fonts have been loaded so that we must interrupt
   redisplay, adjust glyph matrices, and try again.  */

/* C 19380-19385: `enum { SCROLLING_SUCCESS = 1, SCROLLING_FAILED = 0,
   SCROLLING_NEED_LARGER_MATRICES = -1 };` — C's anonymous enum, the port's
   `const`s (the `GET_FROM_BUFFER` shape).  */
pub const SCROLLING_SUCCESS: c_int = 1;
pub const SCROLLING_FAILED: c_int = 0;
pub const SCROLLING_NEED_LARGER_MATRICES: c_int = -1;

/* If scroll-conservatively is more than this, never recenter.

   If you change this, don't forget to update the doc string of
   `scroll-conservatively' and the Emacs manual.  */
/* C 19391: `#define SCROLL_LIMIT 100` — the port's `const` (the
   `CLEAR_FACE_CACHE_COUNT` shape).  */
pub const SCROLL_LIMIT: c_int = 100;

/* The freshness of the w->base_line_number cache is only ensured at every
   redisplay cycle, so the cache can be used only if there's been
   no relevant changes to the buffer since the last redisplay.  */
/* C 19396-19399: `#define BASE_LINE_NUMBER_VALID_P(w)` — the C comma expression
   becomes a statement block in the port's macro; the `current_buffer->clip_changed`
   read is the unit's `buffer_bits!` arm.  */
macro_rules! BASE_LINE_NUMBER_VALID_P {
  ($w:expr) => {
    {
      eassert! (current_buffer == XBUFFER ((*$w).contents));
      !buffer_bits! (clip_changed, current_buffer)
      && BEG_UNCHANGED! () >= (*$w).base_line_pos
    }
  };
}

unsafe fn
try_scrolling (window: Lisp_Object, just_this_one_p: bool,
	       mut arg_scroll_conservatively: intmax_t, mut scroll_step: intmax_t,
	       temp_scroll_step: bool, last_line_misfit: bool) -> c_int
{
  let w: *mut window = XWINDOW (window);
  let mut pos: text_pos = core::mem::zeroed ();
  let mut startp: text_pos = core::mem::zeroed ();
  let mut it: it = core::mem::zeroed ();
  let mut this_scroll_margin: c_int;
  let mut scroll_max: c_int;
  let mut rc: c_int;
  let mut height: c_int;
  let mut dy: c_int = 0;
  let mut amount_to_scroll: c_int = 0;
  let mut scroll_down_p: bool = false;
  let mut extra_scroll_margin_lines: c_int = last_line_misfit as c_int;
  let mut aggressive: Lisp_Object;
  /* We will never try scrolling more than this number of lines.  */
  let scroll_limit: c_int = SCROLL_LIMIT;
  let frame_line_height: c_int = default_line_pixel_height (w);

/* C 19418-19420's `#ifdef GLYPH_DEBUG` block (`debug_method_add (w,
     "try_scrolling")`) is dead on this target (P-08).  */

  SET_TEXT_POS_FROM_MARKER! (startp, (*w).start);

  this_scroll_margin = window_scroll_margin (w, margin_unit::MARGIN_IN_PIXELS);

  /* Force arg_scroll_conservatively to have a reasonable value, to
     avoid scrolling too far away with slow move_it_* functions.  Note
     that the user can supply scroll-conservatively equal to
     `most-positive-fixnum', which can be larger than INT_MAX.  */
  if arg_scroll_conservatively > scroll_limit as intmax_t
    {
      arg_scroll_conservatively = scroll_limit as intmax_t + 1;
      scroll_max = scroll_limit * frame_line_height;
    }
  else if 0 < scroll_step || 0 < arg_scroll_conservatively || temp_scroll_step
    /* Compute how much we should try to scroll maximally to bring
       point into view.  */
    {
      let scroll_lines_max: intmax_t
	= max (scroll_step as isize, max (arg_scroll_conservatively as isize,
		 temp_scroll_step as isize)) as intmax_t;
      let scroll_lines: c_int = clip_to_bounds (0, scroll_lines_max as isize, 1000000) as c_int;
      scroll_max = scroll_lines * frame_line_height;
    }
  else if NUMBERP (BVAR! (current_buffer, scroll_down_aggressively_))
	   || NUMBERP (BVAR! (current_buffer, scroll_up_aggressively_))
    /* We're trying to scroll because of aggressive scrolling but no
       scroll_step is set.  Choose an arbitrary one.  */
    { scroll_max = 10 * frame_line_height; }
  else
    { scroll_max = 0; }

 'too_near_end: loop
	  {

  /* Decide whether to scroll down.  */
  if PT! () > CHARPOS! (startp)
    {
      let mut scroll_margin_y: c_int;

      /* Compute the pixel ypos of the scroll margin, then move IT to
	 either that ypos or PT, whichever comes first.  */
      start_display (&raw mut it, w, startp);
      scroll_margin_y = it.last_visible_y - partial_line_height (&raw mut it)
        - this_scroll_margin
	- frame_line_height * extra_scroll_margin_lines;
      move_it_to (&raw mut it, PT! (), -1, scroll_margin_y - 1, -1,
		  (MOVE_TO_POS | MOVE_TO_Y));

      if PT! () > CHARPOS! (it.current.pos)
	{
	  let y0: c_int = line_bottom_y (&raw mut it);
	  /* Compute how many pixels below window bottom to stop searching
	     for PT.  This avoids costly search for PT that is far away if
	     the user limited scrolling by a small number of lines, but
	     always finds PT if scroll_conservatively is set to a large
	     number, such as most-positive-fixnum.  */
	  let slack: c_int = max (scroll_max as isize, (10 * frame_line_height) as isize) as c_int;
	  let y_to_move: c_int = it.last_visible_y + slack;

	  /* Compute the distance from the scroll margin to PT or to
	     the scroll limit, whichever comes first.  This should
	     include the height of the cursor line, to make that line
	     fully visible.  */
	  move_it_to (&raw mut it, PT! (), -1, y_to_move,
	  	      -1, MOVE_TO_POS | MOVE_TO_Y);
	  dy = line_bottom_y (&raw mut it) - y0;

	  if dy > scroll_max
	    { return SCROLLING_FAILED; }

	  if dy > 0
	    { scroll_down_p = true; }
	}
      else if PT! () == IT_CHARPOS! (it)
	       && IT_CHARPOS! (it) < ZV! ()
	       && it.method == GET_FROM_STRING
	       && arg_scroll_conservatively > scroll_limit as intmax_t
	       && it.current_x == 0
	{
	  let mut skip: move_it_result;
	  let y1: c_int = it.current_y;
	  let mut vpos: c_int;

	  /* A before-string that includes newlines and is displayed
	     on the last visible screen line could fail us under
	     scroll-conservatively > 100, because we will be unable to
	     position the cursor on that last visible line.  Try to
	     recover by finding the first screen line that has some
	     glyphs coming from the buffer text.  */
	  loop {
	    skip = move_it_in_display_line_to (&raw mut it, ZV! (), -1, MOVE_TO_POS);
	    if skip != MOVE_NEWLINE_OR_CR
		|| IT_CHARPOS! (it) != PT! ()
		|| it.method == GET_FROM_BUFFER
	      { break; }
	    vpos = it.vpos;
	    move_it_to (&raw mut it, -1, -1, -1, vpos + 1, MOVE_TO_VPOS);
	  if !(it.vpos > vpos)
		    { break; }
		  }

	  dy = it.current_y - y1;

	  if dy > scroll_max
	    { return SCROLLING_FAILED; }

	  if dy > 0
	    { scroll_down_p = true; }
	}
      else if scroll_margin_y < 0
	{
	  /* A tall row (like tall image?) at window's bottom.  */
	  scroll_down_p = true;
	}
    }

  if scroll_down_p
    {
      /* Point is in or below the bottom scroll margin, so move the
	 window start down.  If scrolling conservatively, move it just
	 enough down to make point visible.  If scroll_step is set,
	 move it down by scroll_step.  */
      if arg_scroll_conservatively != 0
	  {
	amount_to_scroll
	  = min (max (dy as isize, frame_line_height as isize),
		 (frame_line_height as intmax_t * arg_scroll_conservatively) as isize) as c_int;
	  }
      else if scroll_step != 0 || temp_scroll_step
	{ amount_to_scroll = scroll_max; }
      else
	{
	  aggressive = BVAR! (current_buffer, scroll_up_aggressively_);
	  height = WINDOW_BOX_TEXT_HEIGHT! (w) as c_int;
	  if NUMBERP (aggressive)
	    {
	      let float_amount: c_double = XFLOATINT (aggressive) * height as c_double;
	      let mut aggressive_scroll: c_int = float_amount as c_int;
	      if aggressive_scroll == 0 && float_amount > 0.0
		{ aggressive_scroll = 1; }
	      /* Don't let point enter the scroll margin near top of
		 the window.  This could happen if the value of
		 scroll_up_aggressively is too large and there are
		 non-zero margins, because scroll_up_aggressively
		 means put point that fraction of window height
		 _from_the_bottom_margin_.  */
	      if aggressive_scroll + 2 * this_scroll_margin > height
		{ aggressive_scroll = height - 2 * this_scroll_margin; }
	      amount_to_scroll = dy + aggressive_scroll;
	    }
	}

      if amount_to_scroll <= 0
	{ return SCROLLING_FAILED; }

      start_display (&raw mut it, w, startp);
      if arg_scroll_conservatively <= scroll_limit as intmax_t
	{ move_it_vertically (&raw mut it, amount_to_scroll); }
      else
	{
	  /* Extra precision for users who set scroll-conservatively
	     to a large number: make sure the amount we scroll
	     the window start is never less than amount_to_scroll,
	     which was computed as distance from window bottom to
	     point.  This matters when lines at window top and lines
	     below window bottom have different height.  */
	  let mut it1: it = core::mem::zeroed ();
	  let mut it1data: *mut c_void = core::ptr::null_mut ();
	  /* We use a temporary it1 because line_bottom_y can modify
	     its argument, if it moves one line down; see there.  */
	  let mut start_y: c_int;

	  SAVE_IT! (it1, it, it1data);
	  start_y = line_bottom_y (&raw mut it1);
	  loop {
	    RESTORE_IT! (&raw mut it, &raw mut it, it1data);
	    move_it_by_lines (&raw mut it, 1);
	    SAVE_IT! (it1, it, it1data);
	  if !(IT_CHARPOS! (it) < ZV! ()
	     && line_bottom_y (&raw mut it1) - start_y < amount_to_scroll)
		   { break; }
	  }
	  bidi_unshelve_cache (it1data, true);
	}

      /* If STARTP is unchanged, move it down another screen line.  */
      if IT_CHARPOS! (it) == CHARPOS! (startp)
	{ move_it_by_lines (&raw mut it, 1); }
      startp = it.current.pos;
    }
  else
    {
      let mut scroll_margin_pos: text_pos = startp;
      let mut y_offset: c_int = 0;

      /* See if point is inside the scroll margin at the top of the
         window.  */
      if this_scroll_margin != 0
	{
	  let mut y_start: c_int;

	  start_display (&raw mut it, w, startp);
	  y_start = it.current_y;
	  move_it_vertically (&raw mut it, this_scroll_margin);
	  scroll_margin_pos = it.current.pos;
	  /* If we didn't move enough before hitting ZV, request
	     additional amount of scroll, to move point out of the
	     scroll margin.  */
	  if IT_CHARPOS! (it) == ZV! ()
	      && it.current_y - y_start < this_scroll_margin
	    { y_offset = this_scroll_margin - (it.current_y - y_start); }
	}

      if PT! () < CHARPOS! (scroll_margin_pos)
	{
	  /* Point is in the scroll margin at the top of the window or
	     above what is displayed in the window.  */
	  let mut y0: c_int;
  let mut y_to_move: c_int;

	  /* Compute the vertical distance from PT to the scroll
	     margin position.  Move as far as scroll_max allows, or
	     one screenful, or 10 screen lines, whichever is largest.
	     Give up if distance is greater than scroll_max or if we
	     didn't reach the scroll margin position.  */
	  SET_TEXT_POS! (pos, PT! (), PT_BYTE! ());
	  start_display (&raw mut it, w, pos);
	  y0 = it.current_y;
	  y_to_move = max (it.last_visible_y as isize,
			   max (scroll_max as isize, (10 * frame_line_height) as isize)) as c_int;
	  move_it_to (&raw mut it, CHARPOS! (scroll_margin_pos), 0,
		      y_to_move, -1,
		      MOVE_TO_POS | MOVE_TO_X | MOVE_TO_Y);
	  dy = it.current_y - y0;
	  if dy > scroll_max
	      || IT_CHARPOS! (it) < CHARPOS! (scroll_margin_pos)
	    { return SCROLLING_FAILED; }

	  /* Additional scroll for when ZV was too close to point.  */
	  dy += y_offset;

	  /* Compute new window start.  */
	  start_display (&raw mut it, w, startp);

	  if arg_scroll_conservatively != 0
	  {
	    amount_to_scroll
	      = min (max (dy as isize, frame_line_height as isize),
		     (frame_line_height as intmax_t * arg_scroll_conservatively) as isize) as c_int;
	  }
	  else if scroll_step != 0 || temp_scroll_step
	    { amount_to_scroll = scroll_max; }
	  else
	    {
	      aggressive = BVAR! (current_buffer, scroll_down_aggressively_);
	      height = WINDOW_BOX_TEXT_HEIGHT! (w) as c_int;
	      if NUMBERP (aggressive)
		{
		  let float_amount: c_double = XFLOATINT (aggressive) * height as c_double;
		  let mut aggressive_scroll: c_int = float_amount as c_int;
		  if aggressive_scroll == 0 && float_amount > 0.0
		    { aggressive_scroll = 1; }
		  /* Don't let point enter the scroll margin near
		     bottom of the window, if the value of
		     scroll_down_aggressively happens to be too
		     large.  */
		  if aggressive_scroll + 2 * this_scroll_margin > height
		    { aggressive_scroll = height - 2 * this_scroll_margin; }
		  amount_to_scroll = dy + aggressive_scroll;
		}
	    }

	  if amount_to_scroll <= 0
	    { return SCROLLING_FAILED; }

	  move_it_vertically_backward (&raw mut it, amount_to_scroll);
	  startp = it.current.pos;
	}
    }

  /* Run window scroll functions.  */
  startp = run_window_scroll_functions (window, startp);

  /* Display the window.  Give up if new fonts are loaded, or if point
     doesn't appear.  */
  if try_window (window, startp, 0) == 0
	    { rc = SCROLLING_NEED_LARGER_MATRICES; }
  else if (*w).cursor.vpos < 0
	    {
      clear_glyph_matrix ((*w).desired_matrix as *mut glyph_matrix);
      rc = SCROLLING_FAILED;
    }
  else
    {
      /* Maybe forget recorded base line for line number display.  */
      /* FIXME: Why do we need this?  `try_scrolling` can only be called from
         `redisplay_window` which should have flushed this cache already when
         needed.  */
      if !BASE_LINE_NUMBER_VALID_P! (w)
	{ (*w).base_line_number = 0; }

      /* If cursor ends up on a partially visible line,
	 treat that as being off the bottom of the screen.  */
      if !cursor_row_fully_visible_p (w, extra_scroll_margin_lines <= 1,
					false, false)
	  /* It's possible that the cursor is on the first line of the
	     buffer/window, which is partially obscured due to a vscroll
	     (Bug#7537, Bug#78766).  If so, just return, since the code
	     above is not prepared to deal with that case.  */
	  && !MATRIX_ROW_PARTIALLY_VISIBLE_AT_TOP_P!
		 (w, MATRIX_ROW! ((*w).desired_matrix as *mut glyph_matrix, (*w).cursor.vpos))
	{
	  clear_glyph_matrix ((*w).desired_matrix as *mut glyph_matrix);
	  extra_scroll_margin_lines += 1;
	  continue 'too_near_end;
	}
      rc = SCROLLING_SUCCESS;
    }
  break 'too_near_end;
	  }

  return rc;
}


/* Compute a suitable window start for window W if display of W starts
   on a continuation line.  Value is true if a new window start
   was computed.

   The new window start will be computed, based on W's width, starting
   from the start of the continued line.  It is the start of the
   screen line with the minimum distance from the old start W->start,
   which is still before point (otherwise point will definitely not
   be visible in the window).  */

unsafe fn
compute_window_start_on_continuation_line (w: *mut window) -> bool
{
  let mut pos: text_pos = core::mem::zeroed ();
  let mut start_pos: text_pos = core::mem::zeroed ();
  let mut pos_before_pt: text_pos = core::mem::zeroed ();
  let mut window_start_changed_p: bool = false;

  SET_TEXT_POS_FROM_MARKER! (start_pos, (*w).start);

  /* If window start is on a continuation line...  Window start may be
     < BEGV in case there's invisible text at the start of the
     buffer (M-x rmail, for example).  */
  if CHARPOS! (start_pos) > BEGV! ()
      && FETCH_BYTE! (BYTEPOS! (start_pos) - 1) != b'\n'
    {
      let mut it: it = core::mem::zeroed ();
      let mut row: *mut glyph_row;

      /* Handle the case that the window start is out of range.  */
      if CHARPOS! (start_pos) < BEGV! ()
	{ SET_TEXT_POS! (start_pos, BEGV! (), BEGV_BYTE! ()); }
      else if CHARPOS! (start_pos) > ZV! ()
	{ SET_TEXT_POS! (start_pos, ZV! (), ZV_BYTE! ()); }

      /* Find the start of the continued line.  This should be fast
	 because find_newline is fast (newline cache).  */
      row = (*((*w).desired_matrix as *mut glyph_matrix)).rows.offset (window_wants_tab_line (w) as isize
				    + window_wants_header_line (w) as isize);
      init_iterator (&raw mut it, w, CHARPOS! (start_pos), BYTEPOS! (start_pos),
		     row, DEFAULT_FACE_ID);
      reseat_at_previous_visible_line_start (&raw mut it);

      /* Give up (by not using the code in the block below) and say it
         takes too much time to compute a new window start, if the
         line start is "too far" away from the window start.  Also,
         give up if the line start is after point, as in that case
         point will not be visible with any window start we
         compute.  */
      if IT_CHARPOS! (it) <= PT! ()
	  && (CHARPOS! (start_pos) - IT_CHARPOS! (it)
	      /* PXW: Do we need upper bounds here?  */
	      < (WINDOW_TOTAL_LINES (w) * WINDOW_TOTAL_COLS (w)) as ptrdiff_t)
	{
	  let mut min_distance: c_int;
  let mut distance: c_int;

	  /* Move forward by display lines to find the new window
	     start.  If window width was enlarged, the new start can
	     be expected to be > the old start.  If window width was
	     decreased, the new window start will be < the old start.
	     So, we're looking for the display line start with the
	     minimum distance from the old window start.  */
	  pos_before_pt = it.current.pos;
  pos = pos_before_pt;
	  min_distance = DISP_INFINITY;
	  distance = eabs! (CHARPOS! (start_pos) - IT_CHARPOS! (it)) as c_int;
  while distance < min_distance
	    {
	      min_distance = distance;
	      if CHARPOS! (pos) <= PT! ()
		{ pos_before_pt = pos; }
	      pos = it.current.pos;
	      if it.line_wrap == WORD_WRAP
		{
		  /* Under WORD_WRAP, move_it_by_lines is likely to
		     overshoot and stop not at the first, but the
		     second character from the left margin.  So in
		     that case, we need a more tight control on the X
		     coordinate of the iterator than move_it_by_lines
		     promises in its contract.  The method is to first
		     go to the last (rightmost) visible character of a
		     line, then move to the leftmost character on the
		     next line in a separate call.  */
		  move_it_to (&raw mut it, ZV! (), it.last_visible_x, it.current_y, -1,
			      MOVE_TO_POS | MOVE_TO_X | MOVE_TO_Y);
		  move_it_to (&raw mut it, ZV! (), 0,
			      it.current_y + it.max_ascent + it.max_descent, -1,
			      MOVE_TO_POS | MOVE_TO_X | MOVE_TO_Y);
		}
	      else
		{ move_it_by_lines (&raw mut it, 1); }
	    distance = eabs! (CHARPOS! (start_pos) - IT_CHARPOS! (it)) as c_int;
		  }

	  /* It makes very little sense to make the new window start
	     after point, as point won't be visible.  If that's what
	     the loop above finds, fall back on the candidate before
	     or at point that is closest to the old window start.  */
	  if CHARPOS! (pos) > PT! ()
	    { pos = pos_before_pt; }

	  /* Set the window start there.  */
	  SET_MARKER_FROM_TEXT_POS! ((*w).start, pos);
	  window_start_changed_p = true;
	}
    }

  return window_start_changed_p;
}


/* Try cursor movement in case text has not changed in window WINDOW,
   with window start STARTP.  Value is

   CURSOR_MOVEMENT_SUCCESS if successful

   CURSOR_MOVEMENT_CANNOT_BE_USED if this method cannot be used

   CURSOR_MOVEMENT_MUST_SCROLL if we know we have to scroll the
   display.  *SCROLL_STEP is set to true, under certain circumstances, if
   we want to scroll as if scroll-step were set to 1.  See the code.

   CURSOR_MOVEMENT_NEED_LARGER_MATRICES if we need larger matrices, in
   which case we have to abort this redisplay, and adjust matrices
   first.  */

/* C 19856-19862: `enum { CURSOR_MOVEMENT_SUCCESS, CURSOR_MOVEMENT_CANNOT_BE_USED,
   CURSOR_MOVEMENT_MUST_SCROLL, CURSOR_MOVEMENT_NEED_LARGER_MATRICES };` — C's
   anonymous enum, the port's `const`s in C's order (0-3).  */
pub const CURSOR_MOVEMENT_SUCCESS: c_int = 0;
pub const CURSOR_MOVEMENT_CANNOT_BE_USED: c_int = 1;
pub const CURSOR_MOVEMENT_MUST_SCROLL: c_int = 2;
pub const CURSOR_MOVEMENT_NEED_LARGER_MATRICES: c_int = 3;

unsafe fn
try_cursor_movement (window: Lisp_Object, startp: text_pos,
		     scroll_step: *mut bool) -> c_int
{
  let w: *mut window = XWINDOW (window);
  let f: *mut frame = XFRAME ((*w).frame);
  let mut rc: c_int = CURSOR_MOVEMENT_CANNOT_BE_USED;


  /* If this is a window on a tty root frame displaying a child frame,
     the current matrix of W may contain glyphs of that child frame,
     so this method is not safe to use.  */
  if is_tty_root_frame_with_visible_child (f)
    { return rc; }

/* C 19879-19882's `#ifdef GLYPH_DEBUG` block (`inhibit_try_cursor_movement`) is dead
   on this target (P-08).  */

  /* Previously, there was a check for Lisp integer in the
     if-statement below. Now, this field is converted to
     ptrdiff_t, thus zero means invalid position in a buffer.  */
  eassert! ((*w).last_point > 0);
  /* Likewise there was a check whether window_end_vpos is nil or larger
     than the window.  Now window_end_vpos is int and so never nil, but
     let's leave eassert to check whether it fits in the window.  */
  eassert! (!window_bits! (w, window_end_valid)
	   || (*w).window_end_vpos < (*(*w).current_matrix).nrows);

  /* Handle case where text has not changed, only point, and it has
     not moved off the frame.  */
  if /* Point may be in this window.  */
      PT! () >= CHARPOS! (startp)
      /* Selective display hasn't changed.  */
      && !buffer_bits! (clip_changed, current_buffer)
      /* Function force-mode-line-update is used to force a thorough
	 redisplay.  It sets either windows_or_buffers_changed or
	 update_mode_lines.  So don't take a shortcut here for these
	 cases.  */
      && update_mode_lines == 0
      && windows_or_buffers_changed == 0
      && !frame_bits! (cursor_type_changed, f)
      && NILP (Vshow_trailing_whitespace)
      /* When display-line-numbers is in relative mode, moving point
	 requires redrawing the entire window.  */
      && !EQ (Vdisplay_line_numbers, Qrelative ())
      && !EQ (Vdisplay_line_numbers, Qvisual ())
      /* When the current line number should be displayed in a
	 distinct face, moving point cannot be handled in optimized
	 way as below.  */
      && !(!NILP (Vdisplay_line_numbers)
	   && NILP (Finternal_lisp_face_equal_p (Qline_number (),
						 Qline_number_current_line (),
						 (*w).frame, Qt ())))
      /* This code is not used for mini-buffer for the sake of the case
	 of redisplaying to replace an echo area message; since in
	 that case the mini-buffer contents per se are usually
	 unchanged.  This code is of no real use in the mini-buffer
	 since the handling of this_line_start_pos, etc., in redisplay
	 handles the same cases.  */
      && !EQ (window, minibuf_window)
      /* When overlay arrow is shown in current buffer, point movement
	 is no longer "simple", as it typically causes the overlay
	 arrow to move as well.  */
      && !overlay_arrow_in_current_buffer_p ()
    {
      let mut this_scroll_margin: c_int;
  let mut top_scroll_margin: c_int;
      let mut row: *mut glyph_row = core::ptr::null_mut ();

/* C 19934-19936's `#ifdef GLYPH_DEBUG` block (`debug_method_add (w,
     "cursor movement")`) is dead on this target (P-08).  */

      this_scroll_margin = window_scroll_margin (w, margin_unit::MARGIN_IN_PIXELS);

      top_scroll_margin = this_scroll_margin;
      if window_wants_tab_line (w)
	{ top_scroll_margin += CURRENT_TAB_LINE_HEIGHT! (w); }
      if window_wants_header_line (w)
	{ top_scroll_margin += CURRENT_HEADER_LINE_HEIGHT! (w); }

      /* Start with the row the cursor was displayed during the last
	 not paused redisplay.  Give up if that row is not valid.  */
      if (*w).last_cursor_vpos < 0
	  || (*w).last_cursor_vpos >= (*((*w).current_matrix as *mut glyph_matrix)).nrows
	{ rc = CURSOR_MOVEMENT_MUST_SCROLL; }
      else
	{
	  row = MATRIX_ROW! ((*w).current_matrix as *mut glyph_matrix, (*w).last_cursor_vpos);
	  /* Skip the tab-line and header-line rows, if any.  */
	  if glyph_row_bits! (tab_line_p, row)
	    { row = row.offset (1); }
	  if glyph_row_bits! (mode_line_p, row)
	    { row = row.offset (1); }
	  if !glyph_row_bits! (enabled_p, row)
	    { rc = CURSOR_MOVEMENT_MUST_SCROLL; }
	}

      if rc == CURSOR_MOVEMENT_CANNOT_BE_USED
	{
	  let mut scroll_p: bool = false;
  let mut must_scroll: bool = false;
	  let last_y: c_int = window_text_bottom_y (w) - this_scroll_margin;

	  if PT! () > (*w).last_point
	    {
	      /* Point has moved forward.  */
	      while MATRIX_ROW_END_CHARPOS! (row) < PT! ()
		     && MATRIX_ROW_BOTTOM_Y! (row) < last_y
		{
		  eassert! (glyph_row_bits! (enabled_p, row));
		  row = row.offset (1);
		}

	      /* If the end position of a row equals the start
		 position of the next row, and PT is at that position,
		 we would rather display cursor in the next line.  */
	      while MATRIX_ROW_BOTTOM_Y! (row) < last_y
		     && MATRIX_ROW_END_CHARPOS! (row) == PT! ()
		     && row < MATRIX_MODE_LINE_ROW! ((*w).current_matrix as *mut glyph_matrix)
		     && MATRIX_ROW_START_CHARPOS! (row.offset (1)) >= PT! ()
		     && !cursor_row_p (row)
		{ row = row.offset (1); }

	      /* If within the scroll margin, scroll.  Note that
		 MATRIX_ROW_BOTTOM_Y gives the pixel position at which
		 the next line would be drawn, and that
		 this_scroll_margin can be zero.  */
	      if MATRIX_ROW_BOTTOM_Y! (row) > last_y
		  || PT! () > MATRIX_ROW_END_CHARPOS! (row)
		  /* Line is completely visible last line in window
		     and PT is to be set in the next line.  */
		  || (MATRIX_ROW_BOTTOM_Y! (row) == last_y
		      && PT! () == MATRIX_ROW_END_CHARPOS! (row)
		      && !glyph_row_bits! (ends_at_zv_p, row)
		      && !MATRIX_ROW_ENDS_IN_MIDDLE_OF_CHAR_P! (row))
		{ scroll_p = true; }
	    }
	  else if PT! () < (*w).last_point
	    {
	      /* Cursor has to be moved backward.  Note that PT >=
		 CHARPOS (startp) because of the outer if-statement.  */
	      let mut row0: *mut glyph_row = row;

	      while !glyph_row_bits! (mode_line_p, row)
		     && (MATRIX_ROW_START_CHARPOS! (row) > PT! ()
			 || (MATRIX_ROW_START_CHARPOS! (row) == PT! ()
			     && (MATRIX_ROW_STARTS_IN_MIDDLE_OF_CHAR_P! (row)
				 || (/* STARTS_IN_MIDDLE_OF_STRING_P (row) */
				     row > (*((*w).current_matrix as *mut glyph_matrix)).rows
				     && glyph_row_bits! (ends_in_newline_from_string_p, row.offset (-1))))))
		     && ((*row).y > top_scroll_margin
			 || CHARPOS! (startp) == BEGV! ())
		{
		  eassert! (glyph_row_bits! (enabled_p, row));
		  row = row.offset (-1);
		}

	      /* With bidi-reordered rows we can have buffer positions
		 _decrease_ when going down by rows.  If we haven't
		 found our row in the loop above, give it another try
		 now going in the other direction from the original row.  */
	      if !(MATRIX_ROW_START_CHARPOS! (row) <= PT! ()
		    && PT! () <= MATRIX_ROW_END_CHARPOS! (row))
		  && glyph_row_bits! (continued_p, row0)
		{
		  row = row0;
		  while MATRIX_ROW_START_CHARPOS! (row) > PT! ()
			 && MATRIX_ROW_BOTTOM_Y! (row) < last_y
		    {
		      eassert! (glyph_row_bits! (enabled_p, row));
		      row = row.offset (1);
		    }
		}

	      /* Consider the following case: Window starts at BEGV,
		 there is invisible, intangible text at BEGV, so that
		 display starts at some point START > BEGV.  It can
		 happen that we are called with PT somewhere between
		 BEGV and START.  Try to handle that case.  */
	      if row < (*((*w).current_matrix as *mut glyph_matrix)).rows
		  || glyph_row_bits! (mode_line_p, row)
		{
		  row = (*((*w).current_matrix as *mut glyph_matrix)).rows;
		  /* Skip the tab-line and header-line rows, if any.  */
		  if glyph_row_bits! (tab_line_p, row)
		    { row = row.offset (1); }
		  if glyph_row_bits! (mode_line_p, row)
		    { row = row.offset (1); }
		}

	      /* Due to newlines in overlay strings, we may have to
		 skip forward over overlay strings.  */
	      while MATRIX_ROW_BOTTOM_Y! (row) < last_y
		     && MATRIX_ROW_END_CHARPOS! (row) == PT! ()
		     && !cursor_row_p (row)
		{ row = row.offset (1); }

	      /* If within the scroll margin, either the top one or
		 the bottom one, scroll.  */
	      if ((*row).y < top_scroll_margin
		   && CHARPOS! (startp) != BEGV! ())
		  || MATRIX_ROW_BOTTOM_Y! (row) > last_y
		  || PT! () > MATRIX_ROW_END_CHARPOS! (row)
		  || (MATRIX_ROW_BOTTOM_Y! (row) == last_y
		      && PT! () == MATRIX_ROW_END_CHARPOS! (row)
		      && !glyph_row_bits! (ends_at_zv_p, row)
		      && !MATRIX_ROW_ENDS_IN_MIDDLE_OF_CHAR_P! (row))
		{ scroll_p = true; }
	    }
	  else
	    {
	      /* Cursor did not move.  So don't scroll even if cursor line
		 is partially visible, as it was so before.  */
		 rc = CURSOR_MOVEMENT_SUCCESS;
	    }

	  if (PT! () < MATRIX_ROW_START_CHARPOS! (row)
	       && (row == MATRIX_FIRST_TEXT_ROW! ((*w).current_matrix as *mut glyph_matrix)
		   /* Don't give up if point is inside invisible text
		      at the beginning of its glyph row.  */
		   || (MATRIX_ROW_END_CHARPOS! (row.offset (-1))
		       == MATRIX_ROW_START_CHARPOS! (row))))
	      || PT! () > MATRIX_ROW_END_CHARPOS! (row)
	    {
	      /* if PT is not in the glyph row, give up.  */
	      rc = CURSOR_MOVEMENT_MUST_SCROLL;
	      must_scroll = true;
	    }
	  else if rc != CURSOR_MOVEMENT_SUCCESS
		   && !NILP (BVAR! (XBUFFER ((*w).contents), bidi_display_reordering_))
	    {
	      let mut row1: *mut glyph_row;

	      /* If rows are bidi-reordered and point moved, back up
		 until we find a row that does not belong to a
		 continuation line.  This is because we must consider
		 all rows of a continued line as candidates for the
		 new cursor positioning, since row start and end
		 positions change non-linearly with vertical position
		 in such rows.  */
	      /* FIXME: Revisit this when glyph ``spilling'' in
		 continuation lines' rows is implemented for
		 bidi-reordered rows.  */
	      row1 = MATRIX_FIRST_TEXT_ROW! ((*w).current_matrix as *mut glyph_matrix);
		   while MATRIX_ROW_CONTINUATION_LINE_P! (row)
		{
		  /* If we hit the beginning of the displayed portion
		     without finding the first row of a continued
		     line, give up.  */
		  if row <= row1
		    {
		      rc = CURSOR_MOVEMENT_MUST_SCROLL;
		      break;
		    }
		  eassert! (glyph_row_bits! (enabled_p, row));
		  row = row.offset (-1);
		}
	    }
	  if must_scroll
	    { }
	  else if rc != CURSOR_MOVEMENT_SUCCESS
	      && MATRIX_ROW_PARTIALLY_VISIBLE_P! (w, row)
	      /* Make sure this isn't a header line nor a tab-line by
		 any chance, since then MATRIX_ROW_PARTIALLY_VISIBLE_P
		 might yield true.  */
	      && !glyph_row_bits! (mode_line_p, row)
	      && !cursor_row_fully_visible_p (w, true, true, true)
	    {
	      if PT! () == MATRIX_ROW_END_CHARPOS! (row)
		  && !glyph_row_bits! (ends_at_zv_p, row)
		  && !MATRIX_ROW_ENDS_IN_MIDDLE_OF_CHAR_P! (row)
		{ rc = CURSOR_MOVEMENT_MUST_SCROLL; }
	      else if (*row).height > window_box_height (w)
		{
		  /* If we end up in a partially visible line, let's
		     make it fully visible, except when it's taller
		     than the window, in which case we can't do much
		     about it.  */
		  *scroll_step = true;
		  rc = CURSOR_MOVEMENT_MUST_SCROLL;
		}
	      else
		{
		  set_cursor_from_row (w, row, (*w).current_matrix as *mut glyph_matrix, 0, 0, 0, 0);
		  if !cursor_row_fully_visible_p (w, false, true, false)
		    { rc = CURSOR_MOVEMENT_MUST_SCROLL; }
		  else
		    { rc = CURSOR_MOVEMENT_SUCCESS; }
		}
	    }
	  else if scroll_p
	    { rc = CURSOR_MOVEMENT_MUST_SCROLL; }
	  else if rc != CURSOR_MOVEMENT_SUCCESS
		   && !NILP (BVAR! (XBUFFER ((*w).contents), bidi_display_reordering_))
	    {
	      /* With bidi-reordered rows, there could be more than
		 one candidate row whose start and end positions
		 occlude point.  We need to let set_cursor_from_row
		 find the best candidate.  */
	      /* FIXME: Revisit this when glyph ``spilling'' in
		 continuation lines' rows is implemented for
		 bidi-reordered rows.  */
	      let mut rv: bool = false;
	      let mut pt_invis: bool = false;
	      let val: Lisp_Object = get_char_property_and_overlay (make_fixnum (PT! ()),
							       Qinvisible (),
							       Qnil (), core::ptr::null_mut ());

	      if TEXT_PROP_MEANS_INVISIBLE! (val) != 0
		{ pt_invis = true; }

	      loop
		{
		  let mut at_zv_p: bool = false;
  let mut exact_match_p: bool = false;

		  /* If point is in invisible text, we cannot assume
		     it must be after row's start position, since the
		     row could have invisible text at its beginning
		     where point is located.  */
		  if ((pt_invis || MATRIX_ROW_START_CHARPOS! (row) <= PT! ())
		      && PT! () <= MATRIX_ROW_END_CHARPOS! (row)
		      && cursor_row_p (row))
		    { rv = set_cursor_from_row (w, row, (*w).current_matrix as *mut glyph_matrix,
					       0, 0, 0, 0) || rv; }
		  /* As soon as we've found the exact match for point,
		     or the first suitable row whose ends_at_zv_p flag
		     is set, we are done.  */
		  if rv
		    {
		      at_zv_p = glyph_row_bits! (ends_at_zv_p,
					    MATRIX_ROW! ((*w).current_matrix as *mut glyph_matrix, (*w).cursor.vpos));
		      if !at_zv_p
			  && (*w).cursor.hpos >= 0
			  && (*w).cursor.hpos < MATRIX_ROW_USED! ((*w).current_matrix as *mut glyph_matrix,
							       (*w).cursor.vpos) as c_int
			{
			  let candidate: *mut glyph_row =
			    MATRIX_ROW! ((*w).current_matrix as *mut glyph_matrix, (*w).cursor.vpos);
			  let g: *mut glyph =
			    (*candidate).glyphs[TEXT_AREA as usize].offset ((*w).cursor.hpos as isize);
			  let endpos: ptrdiff_t = MATRIX_ROW_END_CHARPOS! (candidate);

			  exact_match_p =
			    (BUFFERP ((*g).object) && (*g).charpos == PT! ())
			    || (NILP ((*g).object)
				&& ((*g).charpos == PT! ()
				    || ((*g).charpos == 0 && endpos - 1 == PT! ())));
			  /* Perhaps the point position is inside
			     invisible text?  In that case, we trust
			     'set_cursor_from_row' to do its job and
			     find the best position for the cursor.  */
			  if !exact_match_p && pt_invis
			    { exact_match_p = true; }
			}
		      if at_zv_p || exact_match_p
			{
			  rc = CURSOR_MOVEMENT_SUCCESS;
			  break;
			}
		    }
		  if MATRIX_ROW_BOTTOM_Y! (row) == last_y
		    { break; }
		  row = row.offset (1);
	      if !(((MATRIX_ROW_CONTINUATION_LINE_P! (row)
		       || glyph_row_bits! (continued_p, row))
		      && MATRIX_ROW_BOTTOM_Y! (row) <= last_y)
		     || (MATRIX_ROW_START_CHARPOS! (row) == PT! ()
			 && MATRIX_ROW_BOTTOM_Y! (row) < last_y))
	    { break; }
	  }
	      /* If we didn't find any candidate rows, or exited the
		 loop before all the candidates were examined, signal
		 to the caller that this method failed.  */
	      if rc != CURSOR_MOVEMENT_SUCCESS
		  && !(rv
		       && !MATRIX_ROW_CONTINUATION_LINE_P! (row)
		       && !glyph_row_bits! (continued_p, row))
		{ rc = CURSOR_MOVEMENT_MUST_SCROLL; }
	      else if rv
		{ rc = CURSOR_MOVEMENT_SUCCESS; }
	    }
	  else
	    {
	      loop
		{
		  if set_cursor_from_row (w, row, (*w).current_matrix as *mut glyph_matrix, 0, 0, 0, 0)
		    {
		      rc = CURSOR_MOVEMENT_SUCCESS;
		      break;
		    }
		  row = row.offset (1);
	      if !(MATRIX_ROW_BOTTOM_Y! (row) < last_y
		     && MATRIX_ROW_START_CHARPOS! (row) == PT! ()
		     && cursor_row_p (row))
	    { break; }
	  }
	    }
	}
    }

  return rc;
}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn
set_vertical_scroll_bar (w: *mut window)
{
  let mut start: ptrdiff_t;
  let mut end: ptrdiff_t;
  let mut whole: ptrdiff_t;

  /* Calculate the start and end positions for the current window.
     At some point, it would be nice to choose between scrollbars
     which reflect the whole buffer size, with special markers
     indicating narrowing, and scrollbars which reflect only the
     visible region.

     Note that mini-buffers sometimes aren't displaying any text.  */
  if !MINI_WINDOW_P (w)
      || (w == XWINDOW (minibuf_window)
	  && NILP (echo_area_buffer[0]))
    {
      let buf: *mut buffer = XBUFFER ((*w).contents);

      whole = BUF_ZV (buf) - BUF_BEGV (buf);
      start = marker_position ((*w).start) - BUF_BEGV (buf);
      end = BUF_Z! (buf) - (*w).window_end_pos - BUF_BEGV (buf);

      /* If w->window_end_pos cannot be trusted, recompute it "the
	 hard way".  But don't bother to be too accurate when
	 long-line shortcuts are in effect.  */
      if !window_bits! (w, window_end_valid) && !buffer_bits! (long_line_optimizations_p, buf)
	{
	  let mut it: it = core::mem::zeroed ();
	  let mut start_pos: text_pos = core::mem::zeroed ();
	  let obuf: *mut buffer = current_buffer;
	  /* When we display the scroll bar of a mini-window,
	     current_buffer is not guaranteed to be the mini-window's
	     buffer, see the beginning of redisplay_window.  */
	  set_buffer_internal_1 (XBUFFER ((*w).contents));
	  SET_TEXT_POS_FROM_MARKER! (start_pos, (*w).start);
	  start_display (&raw mut it, w, start_pos);
	  move_it_to (&raw mut it, -1, it.last_visible_x, window_box_height (w), -1,
		      MOVE_TO_X | MOVE_TO_Y);
	  end -= (BUF_Z! (buf) - IT_CHARPOS! (it)) - (*w).window_end_pos;
	  set_buffer_internal_1 (obuf);
	}

      if end < start
	{ end = start; }
      if whole < (end - start)
	{ whole = end - start; }
    }
  else
    { start = 0; end = 0; whole = 0; }

  /* Indicate what this scroll bar ought to be displaying now.  */
  if !(*FRAME_TERMINAL! (XFRAME ((*w).frame))).set_vertical_scroll_bar_hook.is_none ()
	{
    (*FRAME_TERMINAL! (XFRAME ((*w).frame))).set_vertical_scroll_bar_hook.unwrap ()
      (w as *mut c_void, (end - start) as c_int, whole as c_int, start as c_int);
	}
}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn
set_horizontal_scroll_bar (w: *mut window)
{
  let mut start: c_int;
  let mut end: c_int;
  let mut whole: c_int;
  let mut portion: c_int;

  if !MINI_WINDOW_P (w) || EQ ((*w).horizontal_scroll_bar_type, Qbottom ())
    {
      let b: *mut buffer = XBUFFER ((*w).contents);
      let mut old_buffer: *mut buffer = core::ptr::null_mut ();
      let mut it: it = core::mem::zeroed ();
      let mut startp: text_pos = core::mem::zeroed ();

      if b != current_buffer
	{
	  old_buffer = current_buffer;
	  set_buffer_internal (b);
	}

      SET_TEXT_POS_FROM_MARKER! (startp, (*w).start);
      start_display (&raw mut it, w, startp);
      it.last_visible_x = INT_MAX;
      whole = move_it_to (&raw mut it, -1, INT_MAX, window_box_height (w), -1,
			  MOVE_TO_X | MOVE_TO_Y);
      /* whole = move_it_to (&it, w->window_end_pos, INT_MAX,
			  window_box_height (w), -1,
			  MOVE_TO_POS | MOVE_TO_X | MOVE_TO_Y); */

      start = ((*w).hscroll * FRAME_COLUMN_WIDTH (WINDOW_XFRAME! (w)) as isize) as c_int;
      end = start + window_box_width (w, TEXT_AREA);
      portion = end - start;
      /* After enlarging a horizontally scrolled window such that it
	 gets at least as wide as the text it contains, make sure that
	 the thumb doesn't fill the entire scroll bar so we can still
	 drag it back to see the entire text.  */
      whole = max (whole as isize, end as isize) as c_int;

      if it_bits! (bidi_p, &raw mut it)
	{
	  let pdir: Lisp_Object;

	  pdir = Fcurrent_bidi_paragraph_direction (Qnil ());
	  if EQ (pdir, Qright_to_left ())
	    {
	      start = whole - end;
	      end = start + portion;
	    }
	}

      if !old_buffer.is_null ()
	{ set_buffer_internal (old_buffer); }
    }
  else
    { start = 0; end = 0; whole = 0; portion = 0; }

  (*w).hscroll_whole = whole as ptrdiff_t;

  /* Indicate what this scroll bar ought to be displaying now.  */
  if !(*FRAME_TERMINAL! (XFRAME ((*w).frame))).set_horizontal_scroll_bar_hook.is_none ()
	{
    (*FRAME_TERMINAL! (XFRAME ((*w).frame))).set_horizontal_scroll_bar_hook.unwrap ()
      (w as *mut c_void, portion, whole, start);
	}
}

/* Subroutine of redisplay_window, to determine whether a window-start
   point STARTP of WINDOW should be rejected.  */
unsafe fn
window_start_acceptable_p (window: Lisp_Object, startp: ptrdiff_t) -> bool
{
  if !make_window_start_visible
    { return true; }

  let w: *mut window = XWINDOW (window);
  let f: *mut frame = XFRAME ((*w).frame);
  let startpos: Lisp_Object = make_fixnum (startp);
  let mut invprop: Lisp_Object;
  let mut disp_spec: Lisp_Object;
  let mut ignored: text_pos = core::mem::zeroed ();

  /* Is STARTP in invisible text?  */
  invprop = Fget_char_property (startpos, Qinvisible (), window);
      if TEXT_PROP_MEANS_INVISIBLE! (invprop) != 0
    { return false; }

  /* Is STARTP covered by a replacing 'display' property?  */
  if !NILP ({ disp_spec = Fget_char_property (startpos, Qdisplay (), window); disp_spec })
      && handle_display_spec (core::ptr::null_mut (), disp_spec, Qnil (), Qnil (),
			      &raw mut ignored, startp,
			      FRAME_WINDOW_P (f)) > 0
    { return false; }

  return true;
}

DEFUN! ("long-line-optimizations-p", Flong_line_optimizations_p,
       Slong_line_optimizations_p, 0, 0, 0,
       doc: "Return non-nil if long-line optimizations are in effect in current buffer.\nSee `long-line-threshold' and `large-hscroll-threshold' for what these\noptimizations mean and when they are in effect.",
       (),
       {
  return if buffer_bits! (long_line_optimizations_p, current_buffer)
	 { Qt () } else { Qnil () };
});
/* ==================================================================== */
/* TEMPORARY SCAFFOLD — MUST BE DELETED BEFORE THIS UNIT IS ACCEPTED    */
/* ==================================================================== */
/* `src/xdisp.c` is written in C order, and its early text calls functions the unit defines
   further down (C's own forward declarations at C 1124-1209 are what makes that legal in C;
   Rust items are order-independent, but a *name* must still resolve).  Each declaration
   below stands in for a function this unit will define later; as soon as its definition
   lands the corresponding line is deleted.  Every entry keeps C's prototype in a `/// C:`
   line, so the acceptance round can `grep -c "TEMPORARY SCAFFOLD" femacs/src/xdisp.rs` = 0
   only when the list is empty.  */

unsafe extern "C" {
  /* C's `get_next_element[]` (C 8275-8284) takes the addresses of these seven, whose
     definitions land further down the unit (C 9830, 9258, 9463, 9629, 9703, 9722, 9710) —
     C reaches them through its own forward declarations at C 1189-1196 (PORTING.md
     0.1-12 drops those, so the names live here until the definitions arrive).  */
  /// C: xdisp.c 1240-1241: `static void produce_special_glyphs (struct it *, enum
  /// display_element_type, bidi_dir_t direction, bool left_edge_p);`
  fn produce_special_glyphs (it: *mut it, what: display_element_type,
			     direction: bidi_dir_t, left_edge_p: bool);
  /// C: xdisp.c 859-860: `static bool calc_pixel_width_or_height (double *, struct it *,
  /// Lisp_Object, struct font *, bool, int *, bool *);` (defined later in this unit; the
  /// `struct font *` is `include/font.rs`'s mirror of font.h 275-418, which this unit
  /// includes at C 496).
  fn calc_pixel_width_or_height (res: *mut c_double, it: *mut it,
				 prop: Lisp_Object, font: *mut font,
				 width_p: bool, align_to: *mut c_int,
				 lnum_added: *mut bool) -> bool;
  /// C: dispextern.h 3627: `extern void produce_stretch_glyph (struct it *);` (defined
  /// later in this unit; C's is non-`static`).
  fn produce_stretch_glyph (it: *mut it);
  /// C: xdisp.c 1170: `static int display_mode_line (struct window *, enum face_id,
  /// Lisp_Object);`
  fn display_mode_line (w: *mut window, face_id: face_id, format: Lisp_Object) -> c_int;
  /// C: xdisp.c 1132: `static void handle_line_prefix (struct it *);` (called by
  /// `move_it_in_display_line_to`; its definition lands later in this unit).
  fn handle_line_prefix (it: *mut it);
  /// C: xdisp.c 1165: `static void maybe_produce_line_number (struct it *);`
  fn maybe_produce_line_number (it: *mut it);
  /// C: xdisp.c 1166: `static bool should_produce_line_number (struct it *);`
  fn should_produce_line_number (it: *mut it) -> bool;
  /// C: dispextern.h 3573: `int try_window (Lisp_Object, struct text_pos, int);` (defined
  /// later in this unit; C's is non-`static`).
  fn try_window (window: Lisp_Object, pos: text_pos, flags: c_int) -> c_int;
  /// C: xdisp.c 1130/25062: `static bool cursor_row_p (struct glyph_row *);` (defined
  /// later in this unit).
  fn cursor_row_p (row: *mut glyph_row) -> bool;
  /// C: dispextern.h 3595: `int display_line (struct it *, int);` (xdisp.c defines it at
  /// C 25865).
  /// C: xdisp.c 1154/20475: `static void redisplay_window (Lisp_Object, bool);`
  /// (defined later in this unit).
  fn redisplay_window (window: Lisp_Object, force_p: bool);
  fn display_line (it: *mut it, cursor_vpos: c_int) -> c_int;
  /// C: xdisp.c 1183: `static void compute_line_metrics (struct it *);` (defined later).
  fn compute_line_metrics (it: *mut it);
  /// C: xdisp.c 1170-1171: `static int display_mode_element (struct it *, int, int, int,
  /// Lisp_Object, Lisp_Object, bool);` (defined later in this unit).
  fn display_mode_element (it: *mut it, depth: c_int, field_width: c_int, precision: c_int,
			   elt: Lisp_Object, props: Lisp_Object, risky: bool) -> c_int;
  /// C: xdisp.c 1180-1182: `static int display_string (const char *, Lisp_Object,
  /// Lisp_Object, ptrdiff_t, ptrdiff_t, struct it *, int, int, int, int);` (defined later).
  fn display_string (string: *const c_char, lisp_string: Lisp_Object, face_string: Lisp_Object,
		     face_string_pos: ptrdiff_t, start: ptrdiff_t, it: *mut it,
		     field_width: c_int, precision: c_int, max_x: c_int,
		     multibyte: c_int) -> c_int;
}
