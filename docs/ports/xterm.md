# emacs/src/xterm.c (33125 lines)

> **This file is not compiled by the frozen configuration**: `docs/baseline/frozen-build-sources.txt`
> has no `src/xterm.c` (this target's platform object variables are empty - `WINDOW_SYSTEM_OBJ=`,
> `HAIKU_OBJ = `, `MSDOS_OBJ =`, `CYGWIN_OBJ=` in `build/emacs-ref/src/Makefile`), so `make -n`
> never names the file.  Until round 188 the port was the recorded empty unit; round 188 (lane
> `xterm`) opened it and landed **slice 1 = C 625-990** and **slice 2 = C 1131-1762**
> (`femacs/src/xterm.rs`), so the checklist below is now a real list again and the unchecked
> boxes are the lane's remaining demand.  C 987-1130 (the `x_atom_refs` table) is deliberately
> not landed yet - Decisions 9 says why.
>
> The unit keeps the file's own header mirror's shape (`femacs/src/include/xterm.rs`, b03d883):
> the whole body carries `#[cfg (feature = "have_x_windows")]` - P-51's arm for the non-frozen
> side - and every inner condition becomes the lowercased feature name C's macro spells.  What
> that means for verification is in Decisions 1 and 2, and the measured readings are in Evidence.
>
> `tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs` was **460** `- ` lines
> before slice 1, **443** after it, **324** after slice 2 (C 1131-1762, the drag-and-drop state
> and the Motif message layouts) and **293** after slice 7 (C 3616-4466, the target-window
> discovery; slices 3-6 are Decisions 11-15, slice 7 is Decision 18); the list below is that
> same frozen reading, which is *not* the configuration that compiles the file (Decisions 2); slice
> Slice 109 (C 30667-31838, the whole of `x_term_init`) moves it to **2**: only
> `handle_one_xevent` and the pre-existing `x_get_wm_check_window does not mention false` row are
> left, and the line-count row is gone (31,682 against C's 33,125, 4.4% drift).
9 moves it to **212** (Decision 21) slice 10 (C 9786-11915, Decision 22) to **185** slice 11 (C 12066-12095, Decision 23) to **184** slice 12 (C 26217-26347, Decision 24) to **176** slice 13 (C 26349-26689, Decision 25) to **165** and slice 14 (C 26704-26992, Decision 26) to **163**.  Slice 15 (C 26994-27126, Decision 27) moves it to **159**: three functions retire, the two token rows the first reading brought are closed by `docs/ports/macro-body-notes.tsv` rows, and the invented-export row slice 14 left behind is gone.  Slice 16 (C 27128-27194, Decision 28) moves it to **158**: `x_new_font` retires as a missing item, and no token row came with it.  Slice 17 (C 27195-27284, Decision 29) **stays 158**: its two functions live inside
`HAVE_X11R6_XIM`, which the frozen table leaves `undef` (P-66's dead text), so the tool never
demanded them.  Slice 18 (C 27286-27342, Decision 30) **stays 158** as well: `xim_open_dpy` sits inside
C 27200's `#ifdef HAVE_X_I18N`, which is `undef` in the frozen table too.  Slice 19 (C 27344-27433, Decision 31) **stays 158**: the instantiate
callback is behind `HAVE_X11R6_XIM` and `xim_initialize`'s arms are behind `HAVE_XIM`, all
`undef` in the frozen table.  Slice 20 (C 27436-27487, Decision 32) **stays 158** for the same reason.  Slice 21 (C 27493-27581, Decision 33) moves it to **157**: the
function's missing row retires and its `output_data` token row is closed by a
`docs/ports/macro-body-notes.tsv` row.  Slice 22 (C 27583-27689, Decision 34) moves it to **156**:
`x_set_offset` retires as a missing item and no token row arrives.  Slice 23 (C 27691-27830,
Decision 35) goes **156 -> 154**: `x_get_wm_check_window` and `x_wm_supports` retire as missing
items, `x_wm_supports_1` was already resolved as a name by the callers above it, and no token row
arrives.  Decision 36's round (the `XGetWindowProperty`
declaration and the twenty rows Decision 35 predicted) **stays 154**: the four `does not mention
false` token rows the round's `0` spellings bring are closed by four `docs/ports/macro-body-notes.tsv` rows.  Slice 24 (C 27832-27908, Decision 37) goes **154 -> 150**: `set_wm_state`, `x_set_sticky`, `x_set_shaded` and `x_set_skip_taskbar` retire as missing items.  Slice 25 (C 27910-28085, Decision 38) goes **150 -> 148**: `x_set_z_group` and `x_get_current_wm_state` retire as missing items, and no token row arrives.  Slice 26 (C 28087-28329, Decision 39) goes **148 -> 144**: `do_ewmh_fullscreen`, `XTfullscreen_hook`, `x_handle_net_wm_state` and `x_check_fullscreen` retire as missing items, and the three `switch`-family token rows are `docs/ports/macro-body-notes.tsv`'s.  Slice 27 (C 28336-28475, Decision 40) goes **144 -> 141**: `x_check_expected_move`, `x_sync_with_move` and `x_wait_for_event` retire as missing items, and no token row arrives.  Slice 28 (C 28476-28621, Decision 41) goes **141 -> 137**: `x_set_window_size_1`, `x_suspend_background_fills`, `x_restore_background_fills` and `x_set_window_size` retire as missing items, and no token row arrives.  Slice 29 (C 28623-28743, Decision 42) goes **137 -> 133**: `x_set_window_size_and_position_1`, `x_set_window_size_and_position`, `frame_set_mouse_pixel_position` and `x_raise_frame` retire as missing items, and no token row arrives.  Slice 30 (C 28745-28819, Decision 43) goes **133 -> 129**: `x_lower_frame_1`, `x_lower_frame`, `XTframe_raise_lower` and `xembed_request_focus` retire as missing items, and the one token row the slice brings (`alloca`) is closed by a `docs/ports/macro-body-notes.tsv` row.  Slice 31 (C 28821-28947, Decision 44) goes **129 -> 126**: `server_timestamp_predicate`, `x_get_server_time` and `x_ewmh_activate_frame` retire as missing items, and the slice brings no token row.  Slice 32 (C 28949-29139, Decision 45) goes **126 -> 123**: `x_get_focus_frame`, `x_set_input_focus` and `x_focus_frame` retire as missing items, and the one token row the slice brings (`goto`) is closed by a `docs/ports/macro-body-notes.tsv` row.  Slice 33 (C 29142-29189, Decision 46) goes **123 -> 119**: `xembed_set_info`, `xembed_send_message` and the two `XEMBED_VERSION` rows retire, and no token row arrives.  Slice 34 (C 29201-29400, Decision 47) goes **119 -> 118**: `x_make_frame_visible` retires as a missing item, and no token row arrives.  Slice 35 (C 29406-29481, Decision 48) goes **118 -> 116**: `x_make_frame_invisible` and `x_make_frame_visible_invisible` retire as missing items, and the slice brings no token row.  Slice 36 (C 29483-29614, Decision 49) goes **116 -> 114**: `x_create_font_cursor` and `x_iconify_frame` retire as missing items, and no token row arrives.  Slice 37 (C 29619-29851, Decision 50) goes **114 -> 113**: `x_free_frame_resources` retires as a missing item, and no token row arrives.  Slice 38 (C 29856-29889, Decision 51) goes **113 -> 112**: `x_destroy_window` retires as a missing item once its one token row (`0`, C 29863's `dpyinfo->display != 0`) is written in the port's spelling, and arm readings do not move at all.  Slice 39 (C 29891-29978, Decision 52) goes **112 -> 111**: `x_intern_cached_atom` retires as a missing item, and no token row arrives.  Slice 40 (C 29980-30074, Decision 53) goes **111 -> 110**: `x_get_atom_name` retires as a missing item and its four structural tokens (`alloca`, `case`, `default`, `switch`) are closed by a `docs/ports/macro-body-notes.tsv` row, and the definition resolves the two `x_get_atom_name` call sites earlier slices owed.  Slice 41 (C 30076-30146, Decision 54) goes **110 -> 108**: `x_intern_atoms` and `x_embed_frame` retire as missing items, and no token row arrives.  Slice 42 (C 30342-30420, Decision 55) goes **108 -> 105**: `x_wm_set_window_state`, `x_wm_set_icon_pixmap` and `x_wm_set_icon_position` retire as missing items, and no token row arrives.  Slice 43 (C 30158-30340, Decision 56) goes **105 -> 104**: `x_wm_set_size_hint` retires as a missing item once its `goto` token is closed by a notes row, and the slice adds the mirror's `XSizeHints` (P-87 ① legs: 83 units, 0 failures).  Slice 44 (C 30447-30581, Decision 57) goes **104 -> 99**: `x_free_pixmap`, `same_x_server`, `get_bits_and_offset`, `x_display_ok` and the file-scope `x_initialized` retire, and no token row arrives.  Slice 45 (C 30583-30660, Decision 58) goes **99 -> 98**: the file-scope `x_display_id` retires, and the three functions the frozen table does not demand (`my_log_handler`, `xi_select_hierarchy_events`, `xi_check_toolkit`) are carried with their own feature gates.  Slice 46 (C 31845-31867, Decision 59) goes **98 -> 97**: `x_delete_selection_requests` retires once its `continue` token is closed by a notes row.  Slice 47 (C 31872-31974, Decision 60) goes **97 -> 96**: `x_delete_display` retires once its backward `goto` is closed by a notes row.  Slice 48 (C 32063-32208, Decision 61) goes **96 -> 95**: `x_delete_terminal` retires once its backward `goto` is closed by a notes row.  Slice 49 (C 32393-32417, Decision 62) goes **95 -> 94**: `init_xterm` retires as a missing item, and no token row arrives.  Slice 50 (C 32210-32303, Decision 63) goes **94 -> 93**: `x_create_terminal` retires as a missing item, and no token row arrives.  Slice 51 (C 32305-32499, Decision 64) goes **93 -> 89**: `x_initialize` and the three `*_for_lisp` error helpers retire as missing items, and no token row arrives.  Slice 52 (C 32419-32662, Decision 65) goes **89 -> 87**: `mark_xterm` and `x_get_keyboard_modifiers` retire as missing items, and no token row arrives.  Slice 53 (C 32505-32635, Decision 66) goes **87 -> 86**: `x_preserve_selections` retires as a missing item once its `alloca` token is closed by a `docs/ports/macro-body-notes.tsv` row, and the slice adds the mirror's setter arm for `selection_input_event_bits!` (P-87 ① legs: 83 units, 0 failures).  Slice 54 (C 32664-33125, Decision 67) goes **86 -> 85**: `syms_of_xterm` retires as a missing item once its `doc` token is closed by a notes row, and the slice defines the 39 `DEFVAR_*` objects this unit owns.  Slice 55 (C 11924-12278, Decision 68) goes **85 -> 78**: `x_frame_highlight`, `x_frame_unhighlight`, `x_new_focus_frame`, `x_toggle_visible_pointer`, `XTtoggle_invisible_pointer`, `x_focus_changed` and `x_window_to_frame` retire as missing items, one token row (`x_window_to_frame`'s `0,continue`) is closed by a notes row, and the three call sites earlier slices owed those names resolve.  Slice 56 (C 12344-12538, Decision 69) goes **78 -> 75**: `x_top_window_to_frame` (both arms), `x_next_event_from_any_display` and the `x_any_window_to_frame` macro retire as missing items, and the five call sites earlier slices owed `x_top_window_to_frame` resolve.  Slice 57 (C 12542-12683, Decision 70) goes **75 -> 67**: the eight functions of the selection-request queue and the DnD teardown retire as missing items, and the two token rows the slice brings (`true`, `xfree`) are closed by `docs/ports/macro-body-notes.tsv` rows.  Slice 58 (C 12284-12342, Decision 71) goes **67 -> 66**: `x_tooltip_window_to_frame` retires as a missing item, and the two call sites earlier slices owed it resolve.  Slice 59 (C 13979-14127, Decision 72) goes **66 -> 62**: `x_detect_focus_change`, `x_mouse_leave`, `XTframe_rehighlight` and `x_frame_rehighlight` retire as missing items, the two token rows the slice brings (`switch,case,break` and `0`) are closed by notes rows, and the mirror gains the eight measured X11 event constants the `match` and `x_focus_changed` need.  Slice 60 (C 14134-14303, Decision 73) goes **62 -> 61**: `x_find_modifier_meanings` retires as a missing item, its one token row (`switch,case,break,continue`) is closed by a notes row, and the mirror gains the five modifier masks and nine keysyms its two `match`es pattern on.  Slice 61 (C 14308-14380, Decision 74) goes **61 -> 58**: `x_x_to_emacs_modifiers`, `x_emacs_to_x_modifiers` and `get_keysym_name` retire as missing items, and no token row arrives.  Slice 62 (C 14382-14536, Decision 75) goes **58 -> 54**: `x_compute_root_window_offset`, `x_translate_coordinates`, `x_translate_coordinates_to_root` and `x_handle_translate_coordinates` retire as missing items, the four `frame_x_output` token rows the first writing brought are closed by `docs/ports/macro-body-notes.tsv` rows, and the three XI2 wrappers add no row (the frozen table leaves the whole `HAVE_XINPUT2` span `undef`, so the tool never asks for them).  Slice 63 (C 14536-14634, Decision 76) goes **54 -> 52**: `x_query_pointer_1` and `x_query_pointer` retire as missing items, and no token row arrives.  Slice 64 (C 14630-14719, Decision 77) goes **52 -> 51**: `x_construct_mouse_click` retires as a missing item, and no token row arrives.  Slice 65 (C 14738-14857, Decision 78) goes **51 -> 49**: `x_note_mouse_movement` and `x_get_window_below` retire as missing items, and the one token row the slice brings (`x_get_window_below`'s `continue`) is closed by a `docs/ports/macro-body-notes.tsv` row.  Slice 66 (C 14859-14978, Decision 79) goes **49 -> 48**: `x_fast_mouse_position` retires as a missing item, no token row arrives, and the round also resolves the `note_mouse_highlight` call `include/frame.rs`'s `FRAME_MOUSE_UPDATE!` had left unresolved (its argument's cast landed with it).  Slice 67 (C 14980-15296, Decision 80) goes **48 -> 47**: `XTmouse_position` retires as a missing item, no token row arrives, and the slice resolves the `mouse_position_hook` row `x_create_terminal` was carrying.  Slice 68 (C 15306-15401, Decision 81) goes **47 -> 46**: `x_window_to_scroll_bar` retires as a missing item, its one token row (`continue`) is closed by a notes row, and the two `#if`-gated functions beside it (`xi_select_scroll_bar_events`, `x_window_to_menu_bar`) add no row - the frozen table leaves both gates `undef`.  Slice 69 (C 15403-15426, Decision 82) **stays 46**: `x_get_last_toolkit_time` and the file-scope `window_being_scrolled` retire nothing - the frozen table leaves `USE_TOOLKIT_SCROLL_BARS` `undef`, so this reader never asks for either (the round's own contribution is the two new arms U/UT, which are what make the region measurable).  Slice 70 (C 15428-15536, Decision 83) **stays 46** as well: the region's four file-scope objects and its two Xt action hooks are outside the frozen inventory too, and the slice's real readings are the arms' - UT resolves thirteen sites the new Xt aliases name, and the three casts those aliases exposed were fixed.  Slice 71 (C 15539-15696, Decision 84) **stays 46** too: the region's five functions are outside the frozen inventory, and the slice's readings are again the arms' - the two `x_send_scroll_bar_event` call sites slice 70 left unresolved now resolve, and nineteen registered `E0609` sites plus two bare names arrive in U and UT.  Slice 72 (C 15699-15786, Decision 85) **stays 46** as well: the Motif scroll callback is outside the frozen inventory, its arm (UM) is new, and every reading it produced is a zero - the block's text resolves completely.  Slice 73 (C 15788-15899, Decision 86) **stays 46**, and it moves a *gate*: the `use_gtk` arm (new arm UG) measured slice 71's five functions as wrongly gated `not (use_gtk)`, so those gates were widened to C's own `#ifdef USE_TOOLKIT_SCROLL_BARS` and UG's reading carries the +21 sites that were being hidden.  Slice 74 (C 15902-16034, Decision 87) **stays 46** and moves a *gate* as well: the Xaw arm measures the Xt declarations' `use_motif` gates as too narrow, so they are widened to `use_x_toolkit`, and the four pre-existing defects that exposes are fixed - UT goes 844 -> 830, UM 835 -> 829, and the slice's own text adds 0.  Slice 75 (C 16036-16065, Decision 88) **stays 46** as well: the region's two scroll-bar widget names and the `USE_GTK` arm's two creators are outside the frozen inventory, and every arm reads a zero - the two names are C's own text (the pgtk lane's 5927-5932 pair), the two creators resolve completely, and their only caller (`x_scroll_bar_create`, C 16715, at C 16727/16729) has not been reached yet.  Slice 76 (C 16067-16485, Decision 89) **stays 46**: the `#else /* not USE_GTK */` arm's two creators - Motif and Xaw sub-arms, the shared tail, and every Xt/Motif/Xaw name they read - resolve in the two arms that compile them (UT 830 -> **811**, UM 829 -> **812**, **0 new rows in either**), and the frozen inventory never asks for any of it (`USE_TOOLKIT_SCROLL_BARS` is `undef` there).  Slice 77 (C 16488-16701, Decision 90) **stays 46** too: the four `x_set_toolkit_scroll_bar_thumb` twins - the GTK pair and the Xt pair's Motif/Xaw sub-arms - resolve completely in the three arms that compile them (UT **811/68** and UM **812/68** where slice 76 left them, UG **792/71**; **0 retired and 0 new in every arm**), the two `#if false` blocks are carried as C's own text inside comments, and the slice's real reading is the *mirror* it had to widen: `include/lisp.rs`'s `min`/`max` are C's type-generic macros now.  Slice 78 (C 16709-16824, Decision 91) goes **46 -> 45**: `x_scroll_bar_create` retires as a missing function; UT 811 -> **808**, UM 812 -> **809**, UG 792 -> **791** (the two creators the last three slices landed now resolve at C 16727/16729), and A/U go up (750/804 against 749/795) because their arms are ones C does not compile: the plain-X arm's two bare names and arm U's half of the toolkit vocabulary, both named in the readings below.  Slice 79 (C 16827-16958, Decision 92) goes **45 -> 44**: `x_scroll_bar_set_handle` retires as a missing function.  It lives under C 16827's `#ifndef USE_TOOLKIT_SCROLL_BARS`, so the four toolkit arms do not compile it at all (UT re-measured at **808/68**) and A goes 750/69 -> **761/69**: its eleven uses of `XSetForeground`/`XFillRectangle` join the "Xlib entry points with no mirror row" class the ledger has counted since slices 1-9 - 0 retired, and nothing else in the arm moves.  Slice 80 (C 16960-17116, Decision 93) goes **45 -> 43**: `x_scroll_bar_remove` and `XTset_vertical_scroll_bar` both retire as missing functions; UT 808 -> **803** and UM 809 -> **804** (the two `window_box` rows, `x_scroll_bar_remove`, `XtDestroyWidget` and `XTset_vertical_scroll_bar`'s own hook-table row all resolve), UG 791 -> **788**, A 761 -> **763** and U 804 -> **810** (both arms are ones C does not compile in part: A's forward-declared `x_scroll_bar_redraw` and its two bare Xlib names, U's Xt half - all named in the readings below).  Slice 81 (C 17119-17246, Decision 94) goes **43 -> 41**: `XTset_horizontal_scroll_bar` retires as a missing function and both `... does not mention double` rows are closed by `docs/ports/macro-body-notes.tsv` rows (C's `double` is the port's `f64`, PORTING.md 590 - the vertical twin's row arrived with slice 80 and is closed here too); A 763 -> **765**, U 810 -> **817**, and UT/UM/UG stay at 803/804/788.  Slice 82 (C 17249-17493, Decision 95) goes **41 -> 36**: `XTcondemn_scroll_bars`, `XTredeem_scroll_bar`, `XTjudge_scroll_bars`, `x_scroll_bar_handle_exposure` and `x_scroll_bar_redraw` all retire as missing functions, and the one token row this slice brings (`XTredeem_scroll_bar does not mention goto, horizontal`) is closed by a notes row; A 765 -> **777** (the five XSetForeground calls and the `XEvent` placeholder's nine member rows), while U/UT/UM/UG each lose three - the three hooks' own rows in `x_create_terminal`'s hook table - and gain none.
  Slice 83 (C 17495-17596, Decision 96) goes **36 -> 35**: `x_scroll_bar_handle_click`
retires as a missing function, and no token row arrives.  It is the first function of this region
C compiles in *both* scroll-bar builds (there is no `#ifndef` around the definition, only inside
it), so all five arms read it: A 777 -> **789**, U 814 -> **822**, UT 800 -> **808**, UM 801 ->
**809**, UG 785 -> **793** - and every added row is in a class the ledger already counts (the bare
`Button1`/`ButtonRelease` pair Decision 15 records, and the `[u8; 192]` `XEvent` placeholder's
`xbutton`/`r#type` members slice 21's `msg.xclient.*` opened), with 0 retired in any arm and 0
warnings moved.  C 17537/17568's `f` - an argument handed to macros that never expand it -
measures as 0 rows in every arm, which is what C's own preprocessor does with the name.
  Slice 84 (C 17598-17632, Decision 97) goes **35 -> 34**: `x_scroll_bar_note_movement` retires as
a missing function, and no token row arrives.  C 17598's `#ifndef USE_TOOLKIT_SCROLL_BARS` makes it
arm A's alone: A 789 -> **790**, one `cannot find type XMotionEvent` row in the class slice 65
opened and no other movement, while U/UT/UM/UG read logs identical to their baselines - not the same
count, the same sets, error for error and warning for warning - which is the measurement that the
gate is the only thing between the function and those four arms.
  Slice 85 (C 17634-17769, Decision 98) goes **34 -> 32**: `x_scroll_bar_report_motion` and
`x_horizontal_scroll_bar_report_motion` both retire as missing functions - they are the two
definitions C 1173-1180 forward-declares and the four calls C 14881/14884 (in
`x_fast_mouse_position`) and C 15034/15036 (in `XTmouse_position`) were owed - and no token row
arrives.  Neither function carries a C `#if`, so all five arms read them: A 790 -> **786**, U 822
-> **818**, UT 808 -> **804**, UM 809 -> **805**, UG 793 -> **789**, which is exactly the four
retirements in every arm and 0 additions.  The slice's other reading is what the baseline was
*hiding*: with those four calls unresolved, the MIR diagnostics of `x_fast_mouse_position`'s body
did not run, and landing the definitions revealed six `E0381` rows (C 14866-14868's out-parameters
`dummy`/`root_x`/`root_y`/`win_x`/`win_y`/`mask`, handed to C 14933's `x_query_pointer`) and two
`unused_mut` warnings (C 14869-14870's `bar`/`dpyinfo`) in that body.  The six are zeroed per
Deviation 12, so the error delta above is the four retirements alone; the two warnings stay (C's
declaration-then-single-assignment shape, the `unused_mut` class the baseline already carries 19
of) and the discrimination is measured: putting one unresolved name back into that body takes the
same run from 71 warnings / 7 `E0381` rows to 69 / 1.
  Slice 86 (C 17772-17809, Decision 99) goes **32 -> 31**: `x_scroll_bar_clear` retires as a
missing function, and the one row it was owed - the call C 11416 in `x_clear_frame` - retires in
*every* arm.  Its definition stands outside C's `#if`, while its whole body sits inside C 17779's
`#ifndef USE_TOOLKIT_SCROLL_BARS`, so the port gates the body alone and the four toolkit arms
compile an empty function: A 786 -> **788** (the body's three `XSetForeground` rows join the
counted class), U 818 -> **817**, UT 804 -> **803**, UM 805 -> **804**, UG 789 -> **788**.  The
empty body's unused `f` costs no warning, measured rather than assumed (all five warning sets are
identical to their baselines'): the unit's own `#![allow (...)]` at its line 638 is why.  C
17799-17801's comma-expression loop is the same rendering slice 82 gave C 17435, so the file's two
copies of C's "trick" read alike.
  Slice 87 (C 17811-17848, Decision 100) goes **31 -> 28**: `missing macro STORE_KEYSYM_FOR_DEBUG`,
`missing file-scope object XTread_socket_fake_io_error` and `missing file-scope object
next_noop_dpyinfo` all retire and no token row arrives.  The slice's rustc footprint is **-2 in all
six arms** - A 788 -> **786**, U 817 -> **815**, UT 803 -> **801**, UM 804 -> **802**, UG 788 ->
**786**, and a *sixth* arm AE (A + `enable_checking`, which no configuration here turns on) 820 ->
**818** - and the two rows are not this slice's own: C 31925-31926's two uses of
`next_noop_dpyinfo` are already ported text, so the definition retires them.  The region's other
half exists only under `enable_checking` (`undef` in the frozen table), which is why AE exists:
with a temporary reader after the macro's definition, AE reads 0 error and 0 warning delta, and a
deliberate type error in the macro body moves it 818 -> 819 at the macro's own line.
  Slice 88 (C 17850-17921, Decision 101) **stays 28**: `x_filter_event` lives inside
`#ifdef HAVE_X_I18N` (`undef` in the frozen table), so it is P-66 dead text with no checklist row
and no `--check` item - the first slice of this lane whose reading lives entirely outside the five
standard arms, which is why it adds *three* (`have_x_i18n`, that plus `use_gtk`, and that plus
`have_xinput2`).  Those arms move A 792 -> **794** (AI), 812 -> **822** (AIU) and 910 -> **929**
(AIUX), and every added row is a counted class (the `[u8; 192]` placeholder's `xclient`/`xgeneric`/
`xcookie`/`r#type` members, Decision 15's bare X11 and XInput2 names, the E0425 type class, and
C 894-896's `prefer_native_input` - an `x_display_info` member the mirror does not carry yet, named
here as an owed row); the six default arms read **0/0** because the gate hides the whole
definition, and the slice's one warning is arm AI's `unreachable statement` - C's own `return 0;`
behind the non-GTK arm's unconditional return.
  Slice 89 (C 17923-18002, Decision 102) **stays 28**: `event_handler_gdk` is inside C 17923's
`#ifdef USE_GTK` (`undef` in the frozen table), so it is dead text with no checklist row and no
`--check` item, and its readings live in the `use_gtk` arms.  Four of them move and six read 0/0:
UG 786 -> **796** (+10), a fifth arm **UGX** (`use_gtk` + `have_xinput2`, added because the
`#elif USE_GTK` half's `#ifdef HAVE_XINPUT2` disjunct is compiled by no other arm) 884 -> **899**
(+15), AIU 822 -> **831** (+9) and AIUX 929 -> **938** (+9); A/U/UT/UM/AE/AI are all 0/0.  Nothing
retires and **no warning moves in any of the ten arms**; the rows are the counted classes - the
`XEvent` placeholder's members, Decision 15's bare X11/XInput2 names, `xg_filter_key`, the
`GdkFilterReturn`/`GdkXEvent`/`GDK_FILTER_*` surface `include/gtk.rs` does not carry yet, the
`E0425` type class, and the two still-unported definitions `handle_one_xevent`/`x_dispatch_event`
(whose `--check` rows retire when they land).
  Slice 90 (C 18006-18056, Decision 103) goes **28 -> 26**: `x_net_wm_state` and
`x_flush_dirty_back_buffer_on` both retire as missing functions, and the one token row this slice
brings (`x_net_wm_state does not mention break, case, switch`) is closed by a
`docs/ports/macro-body-notes.tsv` row.  Every arm reads **0 error movement** - the two definitions'
own text resolves - but each gains **one warning**, C 18014's `lval` re-assigned only inside the
`have_window_system` block, and the `have_xdbe` arm adds **3 rows** (C 18047's owed
`buffer_flipping_blocked_p`, and the two `x_output` members `FRAME_X_COMPLETE_P!`/
`FRAME_X_NEED_BUFFER_FLIP!` read when their bodies type-check for the first time).
  Slice 91 (C 18058-18116, Decision 104) goes **26 -> 25**: `mouse_or_wdesc_frame` retires as a
missing function, and `x_scroll_bar_configure` (`#ifdef HAVE_GTK3`, `undef` in the frozen table) is
dead text with no item - so the slice adds **AG3** (`use_gtk` + `have_gtk3`) as the arm that reads
it, and *every* other arm reads **0/0**: A/U/UT/UM/UG/UGX/AE/AI/AIU/AIUX/AX all hold their numbers
exactly, warnings included, while AG3 goes 818/73 -> **835/73** (+17: twelve more `xconfigure`
writes, two `GDK_WINDOW_XID`, `x_dispatch_event` and the two `GdkEventConfigure` members the mirror
does not carry).  The two read-only locals of `mouse_or_wdesc_frame` are written without `mut` -
that keyword is the port's own, and keeping it cost one `unused_mut` warning per arm (measured).
  Slice 92 (C 18118-18187, Decision 105) goes **25 -> 24**: `x_dnd_compute_tip_xy` retires as a
missing function.  It is ungated, so *every* arm moves by the same two rows - the two still-unported
definitions `x_display_pixel_width` and `x_display_pixel_height` (C 18160-18161) - and by nothing
else: A 786 -> **788**, U 815 -> **817**, UT 801 -> **803**, UM 802 -> **804**, UG 796 -> **798**,
UGX 899 -> **901**, AE 818 -> **820**, AI 794 -> **796**, AIU 831 -> **833**, AIUX 938 -> **940**,
AX 800 -> **802** and AG3 835 -> **837**, with **no warning moved in any of the twelve arms**.  The
first writing's fourteen `XFIXNUM` reads mixed `EMACS_INT` into C's `int` arithmetic (14 `E0308` and
12 `E0277` rows across the arms); C's conversions are stated as `as c_int` (the C 11761 reading).
  Slice 93 (C 18189-18259, Decision 106) goes **24 -> 22**: `x_dnd_update_tooltip_position` and
`x_dnd_update_tooltip_now` both retire as missing functions, and the slice's readings are a
*half-split*: C 18243's `#ifndef HAVE_XINPUT2` picks the bare `XQueryPointer` (the counted class,
one row an arm) in the ten arms without that feature, while the two `have_xinput2` arms call slice
65's `x_query_pointer_1` and read **0/0** - A 788 -> **789**, U 817 -> **818**, UT 803 -> **804**,
UM 804 -> **805**, UG 798 -> **799**, AE 820 -> **821**, AI 796 -> **797**, AIU 833 -> **834**, AX
802 -> **803**, AG3 837 -> **838**, and UGX 901 -> **901**, AIUX 940 -> **940**.  No warning moved in
any of the twelve arms.
  Slice 94 (C 18481-18497, Decision 107) goes **22 -> 20**: `x_display_pixel_height` and
`x_display_pixel_width` both retire as missing functions, and the call rows the earlier slices
carried retire with them - C 27531/27573 in `x_calc_absolute_position`, C 30266-30269 in
`x_wm_set_size_hint` and C 18160-18161 in slice 92's own `x_dnd_compute_tip_xy`, one row per name per
site - while every arm reads one new row per name for `HeightOfScreen`/`WidthOfScreen`, two Xlib
macros no mirror here carries: A 789 -> **785**, U 818 -> **814**, UT 804 -> **800**, UM 805 ->
**801**, UG 799 -> **797**, UGX 901 -> **899**, AE 821 -> **817**, AI 797 -> **793**, AIU 834 ->
**832**, AIUX 940 -> **938**, AX 803 -> **799** and AG3 838 -> **836**, with no warning moved in any
of the twelve arms.  The five `use_gtk` arms move by two rather than four because their retirements
read `E0308` instead of `E0425`: the mirror's own P-55 `*mut c_void` declaration is what made those
`FRAME_DISPLAY_INFO!` arguments mismatches, and C's own signature retires the mismatch - and their
`x_wm_set_size_hint` pair is not compiled at all.
  Slice 95 (C 18261-18479, Decision 108) goes **20 -> 19**: `x_dnd_update_state` retires as a
missing function and retires nothing else - its callers (C 18674/21408/25544 and the rest) are
inside text the port has not landed, so the seven arms without `use_gtk` read 0/0 on errors and the
five that have it read +5 rows apiece: A **785 -> 785**, U **814 -> 814**, UT **800 -> 800**, UM
**801 -> 801**, UG 797 -> **802**, UGX 899 -> **904**, AE **817 -> 817**, AI **793 -> 793**, AIU 832
-> **837**, AIUX 938 -> **943**, AX **799 -> 799**, AG3 836 -> **841**.  Every one of the twelve arms
gains **one warning**, and it is C's own dead store: C 18454 writes `dmsg.byteorder` where the block
around it fills `dsmsg`, which Rust's `unused_assignments` names.  The five `use_gtk` arms' five rows
are the mirror chain's class: `FRAME_OUTER_WINDOW!`'s GTK arm ends in `GDK_WINDOW_XID` ->
`GDK_WINDOW_XWINDOW`, and neither name is in a mirror - one row per site (C 18294/18303/18309/18342/
18348).
  Slice 96 (C 18499-18612, Decision 109) goes **19 -> 18**: `x_wait_for_cell_change` retires as a
missing function, and the one row it brings is a *token* row - C 18522's `while (true)` is the port's
`loop` (this file 21756's rule), so `true` is closed by a `docs/ports/macro-body-notes.tsv` row while
`x_get_wm_check_window does not mention false` stays.  Its rustc footprint is the bare classes the
body reads: the seven arms that compile C 18532's block each add five rows (`ConnectionNumber` x1,
`XPending` x2, `XNextEvent` x1, `handle_one_xevent` x1), and the five `use_gtk` arms add one
(`ConnectionNumber`) because that block is not compiled there - A 785 -> **790**, U 814 -> **819**, UT
800 -> **805**, UM 801 -> **806**, UG 802 -> **803**, UGX 904 -> **905**, AE 817 -> **822**, AI 793 ->
**798**, AIU 837 -> **838**, AIUX 943 -> **944**, AX 799 -> **804**, AG3 841 -> **842**, with **no
warning moved in any of the twelve arms**.  The GTK arm's two calls resolve: `include/gtk.rs`
declares `gtk_events_pending`/`gtk_main_iteration` and reaches this unit through `include/xterm.rs`
53's C 69 edge - which is also what the earlier slices' includer counts missed.
  Slice 97 (C 18614-18679, Decision 110) goes **18 -> 17**: `x_find_monitors_changed_event` retires as
a missing function, and the slice reads **0/0** in the seven arms without `use_gtk` - everything the
finder names is already in the unit or in its include graph - while the five `use_gtk` arms add **two
rows** apiece for the callback: `E0609 no field last_monitor_attributes_list on type x_display_info`,
one per site (C 18663's read, C 18677's write).  That field is C's `#if defined HAVE_XRANDR ||
defined USE_GTK` member (xterm.h 834), which the mirror does not carry - the class slice 92's
`x_display_info.prefer_native_input` row opened, owed to the mirror's own round.  A 790 -> **790**,
U 819 -> **819**, UT 805 -> **805**, UM 806 -> **806**, UG 803 -> **805**, UGX 905 -> **907**, AE 822
-> **822**, AI 798 -> **798**, AIU 838 -> **840**, AIUX 944 -> **946**, AX 804 -> **804**, AG3 842 ->
**844**, with **no warning moved in any of the twelve arms**.  `x_monitors_changed_cb` (C 18638-18678)
is dead text - `USE_GTK` is `undef` in the frozen table - so it is carried with its gate (P-86 1) and
is not a checklist row.
  Slice 98 (C 18681-18761, Decision 111) goes **17 -> 16**: `x_coords_from_dnd_message` retires as a
missing function, and the slice's whole rustc footprint is **the same twelve rows in all twelve
arms** - eleven `E0609 no field xclient on type xlib_surface::XEvent` and one `no field r#type` - plus
**one warning**: `unnecessary parentheses around `if` condition`, which is rustc's `unused_parens`
false positive on C 18743-18750's `((A) || (B)) && C && D` (proved with a two-line probe: the lint
also fires on `if ((a || b) && c && d)`, where removing the pair would change the meaning) - so C's
grouping stays and the warning is the reading.  The `xclient`/`r#type` class is the file's own
recorded one (`include/xlib.rs` 1364 keeps C's measured 192-byte `XEvent` and no member; the baseline
already carries 153 + 9 of these rows), and the two KDE atoms are `include/xterm.rs` 495-496's rows,
so this function adds no *name* row: A 790 -> **802**, U 819 -> **831**, UT 805 -> **817**, UM 806 ->
**818**, UG 805 -> **817**, UGX 907 -> **919**, AE 822 -> **834**, AI 798 -> **810**, AIU 840 -> **852**,
AIUX 946 -> **958**, AX 804 -> **816**, AG3 844 -> **856**.
  Slice 99 (C 18763-18824, Decision 112) goes **16 -> 15**: `x_handle_wm_state` retires as a missing
function and brings one *token* row instead - C 18796/18811's `SET_FRAME_VISIBLE (f, 1)`/`(f, 0)` are
the port's `true`/`false` (this file 17924-17925's reading), so the `1` is closed by a
`docs/ports/macro-body-notes.tsv` row.  Its rustc footprint is one row in **every** arm -
`E0609 no field has_been_visible on type x_output`, the mirror's `bool_bf` member (`include/xterm.rs`
849 carries it as a `// C:` line, PORTING.md 1.3) that `x_calc_absolute_position` opened as this
unit's owed record - plus one more in the five `use_gtk` arms, where C 18776's `FRAME_OUTER_WINDOW (f)`
reaches the mirror chain's `GDK_WINDOW_XWINDOW`/`GDK_WINDOW_XID` gap slice 95 opened: A 802 -> **803**,
U 831 -> **832**, UT 817 -> **818**, UM 818 -> **819**, UG 817 -> **819**, UGX 919 -> **921**, AE 834
-> **835**, AI 810 -> **811**, AIU 852 -> **854**, AIUX 958 -> **960**, AX 816 -> **817**, AG3 856 ->
**858**, with **no warning moved in any of the twelve arms**.  The first writing read two defects, both
C's own conversions: Xlib's `AnyPropertyType` is `0L`, so the argument is `AnyPropertyType as Atom`,
and `f->output_data.x->has_been_visible` needs C's pointer cast (`(*f).output_data.x as *mut x_output`,
this file 8045's shape).
  Slice 100 (C 18826-18864, Decision 113) is the first slice whose functions no checklist row
demands, and its reading is therefore an arm of its own: `HAVE_XFIXES` is `undef` in the frozen
table, so the **twelve arms read 0/0** - every one of them identical to its slice-99 baseline, A
803/74, U 832/74, UT 818/73, UM 819/73, UG 819/75, UGX 921/77, AE 835/73, AI 811/75, AIU 854/75,
AIUX 960/77, AX 817/74, AG3 858/74 - and the slice adds the lane's thirteenth arm, **AXF** (A +
`have_xfixes`), which is the first reading of this block: **809/74 -> 816/74**, seven rows, all
`E0609` on the three `#ifdef HAVE_XFIXES` members of `x_display_info` (`monitored_selections` x4,
`selection_tracking_window` x1, `n_monitored_selections` x2), with the SDK record resolving (the
unit declares `XFixesSelectionNotifyEvent` at the use site, P-88 1) and **no warning moved**.
  Slice 101 (C 25641-25811, Decision 114) goes **15 -> 13**: `x_dispatch_event` and `XTread_socket`
both retire as missing functions, and the slice reads **+4** in the eight arms that compile
`XTread_socket`'s `#ifndef USE_GTK` block (the seven plus AXF: `XPending` x1, `XNextEvent` x1,
`handle_one_xevent` x2 and `XNoOp` x1, against the retirement of `cannot find value XTread_socket` x1
- the terminal hook table's reference to it), **0/0** in UG/UGX/AIU/AIUX (C 25656's
`handle_one_xevent` call and `XNoOp` arrive while the hook table's `XTread_socket` and
`x_dispatch_event` references retire) and **-1** in AG3, whose hook table names `x_dispatch_event`
twice: A 803 -> **807**, U 832 -> **836**, UT 818 -> **822**, UM 819 -> **823**, UG 819 -> **819**, UGX
921 -> **921**, AE 835 -> **839**, AI 811 -> **815**, AIU 854 -> **854**, AIUX 960 -> **960**, AX 817 ->
**821**, AG3 858 -> **857**, AXF 816 -> **820**, with **no warning moved in any of the thirteen
arms**.  C 25656's `0` for `hold_quit` puts a `does not mention 0` row on `x_dispatch_event` (closed
by a `docs/ports/macro-body-notes.tsv` row), and C 25640's `#ifdef USE_GTK` picking the *linkage* is
the slice's one named deviation (Decision 114 1).
  Slice 102 (C 25816-25902, Decision 115) goes **13 -> 11**: `x_clip_to_row` and
`x_draw_hollow_cursor` both retire as missing functions, and the slice reads **+3** in the thirteen
arms that do not turn `have_window_system` on (`GCLineWidth` x2 and `XChangeGC` x1 arrive, and
`x_clip_to_row`'s own reference retires) and **+2** in the lane's `have_window_system` arm **AWS**,
where C's `w->phys_cursor_width` resolves instead of reading one `E0609`: A 807 -> **810**, U 836 ->
**839**, UT 822 -> **825**, UM 823 -> **826**, UG 819 -> **822**, UGX 921 -> **924**, AE 839 -> **842**,
AI 815 -> **818**, AIU 854 -> **857**, AIUX 960 -> **963**, AX 821 -> **824**, AG3 857 -> **860**, AXF
820 -> **823** and AWS 501/0 -> **503/0**, with **no warning moved in any of the fourteen arms**.  The
first writing's five defects (an `x11_constants::GCLineWidth` import that does not exist, a
`resolved_level!` call for the mirror's `glyph_bits!` member, two `pixel_width` reads missing C's
`short` conversion, and a partially initialised `clip_rect`) are named in Decision 115 4.
  Slice 103 (C 25905-26014, Decision 116) goes **11 -> 10**: `x_draw_bar_cursor` retires as a missing
function, and the slice reads **+11** in the thirteen arms that leave `have_window_system` off -
`phys_cursor_width` x4, `window_box_left` x2 (the `WINDOW_TEXT_TO_FRAME_PIXEL_X!` expansion),
`face_cache` x2 (the per-unit `FRAME_FACE_CACHE`/`FACE_FROM_ID` copies' `dispextern_face_records`
type), `GCGraphicsExposures` x1, `GCLineWidth` x1 and `XChangeGC` x1, all recorded classes - and
**+7** in AWS, where the four `phys_cursor_width` rows do not exist: A 810 -> **821**, U 839 -> **850**,
UT 825 -> **836**, UM 826 -> **837**, UG 822 -> **833**, UGX 924 -> **935**, AE 842 -> **853**, AI 818
-> **829**, AIU 857 -> **868**, AIUX 963 -> **974**, AX 824 -> **835**, AG3 860 -> **871**, AXF 823 ->
**834** and AWS 503/0 -> **510/0**, with **no warning moved in any of the fourteen arms**.  The first
writing's four defects (a `GCGraphicsExposures` import that does not exist, two cursor-helper
declarations missing from this function's own use-site block, an absent `FACE_FROM_ID` and C's two
*chained* assignments) are named in Decision 116 4.
  Slice 104 (C 26016-26096, Decision 117) goes **10 -> 7**: `x_define_frame_cursor`,
`x_clear_frame_area` and `x_draw_window_cursor` all retire as missing functions, and the one token row
the reading brings (`x_draw_window_cursor does not mention break, case, default, switch`) is closed by
a `docs/ports/macro-body-notes.tsv` row.  The slice reads **+3** in the eleven arms that do not set
`use_x_toolkit` (`XDefineCursor` x1 - the name is declared only in this file's own
`use_x_toolkit`-gated block at 24997 - plus `phys_cursor_type` x1 and `phys_cursor_width` x1), **+2**
in UT/UM (that declaration is in scope there), **+4** in AI/AIU/AIUX (`xic_set_preeditarea`'s row
joins) and **+1** in AWS (only `XDefineCursor`): A 821 -> **824**, U 850 -> **853**, UT 836 -> **838**,
UM 837 -> **839**, UG 833 -> **836**, UGX 935 -> **938**, AE 853 -> **856**, AI 829 -> **833**, AIU 868
-> **872**, AIUX 974 -> **978**, AX 835 -> **838**, AG3 871 -> **874**, AXF 834 -> **837** and AWS
510/0 -> **511/0**, with **no warning moved in any of the fourteen arms** and **nothing retired in any
arm** (the port's `x_create_terminal`, C 30668+, is not landed, so no landed text names the three
functions).  The first writing's one defect (a `pub type Emacs_Cursor = Cursor;` the mirror already
carries, `E0428`) is named in Decision 117 5.

  Slice 105 (C 26104-26214, Decision 118) goes **7 -> 5**: `x_bitmap_icon` and `x_text_icon` both
retire as missing functions, and no token row arrives.  The reading is **-1 in nine of the fifteen
arms** (A, U, UT, UM, AE, AI, AX, AXF, AWS: the `cannot find value x_bitmap_icon` row retires,
because landed text already named it), **+1 in five** (UG, UGX, AIU, AIUX, AG3: the same retirement
less two new rows - `gnu_xpm_bits`, the XPM array `include/emacs-icon.rs` records as dead text
rather than transcribing, read by the `USE_GTK` arm's `xg_set_icon_from_xpm_data` call, and the GTK
arms' `GDK_WINDOW_XWINDOW`/`GDK_WINDOW_XID`, which the new `FRAME_X_WINDOW` uses reach through
`include/xterm.rs`'s macros) and **0 in AXPM** (the fifteenth arm this slice adds, `A + have_xpm`:
the retirement less the `#elif` half's own `gnu_xpm_bits`): A 824 -> **823**, U 853 -> **852**, UT 838
-> **837**, UM 839 -> **838**, UG 836 -> **837**, UGX 938 -> **939**, AE 856 -> **855**, AI 833 ->
**832**, AIU 872 -> **873**, AIUX 978 -> **979**, AX 838 -> **837**, AG3 874 -> **875**, AXF 837 ->
**836**, AWS 511/0 -> **510/0** and AXPM 824/74 -> **824/74**, with **no warning set moved in any of
the fifteen arms**.  The first writing's three defects (a second `XA_STRING` the unit already
carries at 18924, C's two implicit `int` -> `unsigned int` conversions left without the port's
casts, and a `mut` rustc did not need) are named in Decision 118 5.
  Slice 106 (C 12702-13327, Decision 119) goes **5 -> 4**: `x_dnd_begin_drag_and_drop` retires as a
missing function - the file's second-longest, 625 C lines - and no token row arrives (the first
writing's one omission, C 12852's `record_unwind_protect_void (x_clear_dnd_variables)`, was read by
the checker's `does not mention x_clear_dnd_variables` row and fixed).  The slice reads **+7 in
eleven arms** (A, U, AE, AI, AX, AXF, AXPM, and their four peers: four Xlib names the mirror does
not carry - `XWindowAttributes`, `XGetWindowAttributes`, `XChangeProperty`, `XSelectInput` - plus
`handle_one_xevent`, the `xany`/`r#type` members of the XEvent stand-in and
`last_monitor_attributes_list`), **+6 in the five `use_gtk` arms** (the GTK build has no `xany`
read: C 13074's `if` is absent there), **+8 in UT/UM and AXR** (the Xt arm adds
`XtAppNextEvent`), **+9 in UGX/AIUX** (`XIGetClientPointer`, `client_pointer_device` x2) and
**+15 in the new AXKB arm** (the XKB block: `XkbSelectEvents` x1, `XkbUseCoreKbd` x2,
`XkbStateNotifyMask` x2, `supports_xkb` x1, `XkbGetState` x1, `XkbStateRec` x1, plus the seven
shared rows): A 823 -> **830**, U 852 -> **859**, UT 837 -> **845**, UM 838 -> **846**, UG 837 ->
**843**, UGX 939 -> **948**, AE 855 -> **862**, AI 832 -> **839**, AIU 873 -> **879**, AIUX 979 ->
**988**, AX 837 -> **844**, AG3 875 -> **881**, AXF 836 -> **843**, AWS 510/0 -> **515/0**, AXPM 824
-> **831**, AXKB 841 -> **856** and AXR 825 -> **833**, with **no warning set moved in any of the
seventeen arms**.  The two new arms (`AXKB = A + have_xkb`, `AXR = A + have_xrandr`) are this
slice's: C 12952's `#ifdef HAVE_XKB` block and C 12856's `#if defined HAVE_XRANDR || defined
USE_GTK` are the text no earlier arm could read.  The eight defects the first writing read (the
seven `static` helpers whose address the callbacks take, three `Option<...>` call sites, the
mirror's `DEFER_SELECTIONS`), the two C locals the port must zero and the FIXME comment are named
in Decision 119 4-5.

# ----------------------------------------------------------------------
  Slice 110 (C 18866-18949 and C 25596-25632, Decision 120) **stays 2**: `handle_one_xevent`'s
`missing function` row becomes `function handle_one_xevent does not mention <the remaining token
list>` - the file's last missing definition is now *defined*, with the thirty `case` blocks
C 18951-25595 writes as the slices that follow, so that row's own text is the remaining work - and
the four `cannot find function handle_one_xevent` rows the two call sites (`x_dispatch_event`
C 25656 and `XTread_socket` C 25732) carried **retire**.

  Slice 111 (C 19473-19561, Decision 122) takes the switch's own frame and its first three arms:
`OTHER:`'s block (C 25558-25587) and `done:`'s (C 25590-25595), then `SelectionNotify`,
`SelectionClear` and `SelectionRequest` - 89 C lines in C's order.  The two labels C writes
*inside* the `GenericEvent` case are the whole switch's tail (thirty sites `goto OTHER`), so they
are carried once at the end of the `match` with `break 'other` at each site, and `done:`'s six
lines come with their label.  The other twenty-seven `case` bodies are the slices that follow, at
their own C line numbers.  The row `--check` reads keeps its two lines and its token list goes
**531 -> 484**; the mirror round this slice needed (C's own guard for `buffered_input_event`'s
`sie` member, and the `Copy` the union requires) is Decision 122's third item.

  Slice 112 (C 19562-19862, Decision 123) lands `case PropertyNotify:` and `case ReparentNotify:`
(301 C lines), and adds the lane's **eighteenth arm**, `AUX = A + use_xcb`, because C's
`#ifndef USE_XCB` / `#else` pair inside the opacity reader is text no earlier arm had compiled -
that arm's first reading was not a number at all (an abort at macro expansion, from HEAD's
`SAFE_FREE! ()` at C 3607), and the fix it forced is Decision 123's fourth item.

## Subrs (DEFUN) — 0

## Functions defined — 310
- [x] x_defer_selection_requests  (line 900)
- [x] x_release_selection_requests  (line 947, static)
- [x] x_release_selection_requests_and_flush  (line 953)
- [x] xm_side_effect_from_action  (line 1764, static)
- [x] xm_operations_from_actions  (line 1779, static)
- [x] xm_read_targets_table_header  (line 1801, static)
- [x] xm_read_targets_table_rec  (line 1826, static)
- [x] xm_find_targets_table_idx  (line 1859, static)
- [x] x_atoms_compare  (line 1884, static)
- [x] xm_write_targets_table  (line 1890, static)
- [x] xm_write_drag_initiator_info  (line 1937, static)
- [x] xm_drag_window_error_handler  (line 1961, static)
- [x] xm_drag_window_io_error_handler  (line 1970, static)
- [x] x_special_window_exists_p  (line 1980, static)
- [x] xm_get_drag_window_1  (line 2015, static)
- [x] xm_get_drag_window  (line 2166, static)
- [x] xm_setup_dnd_targets  (line 2176, static)
- [x] xm_get_drag_atom_1  (line 2403, static)
- [x] xm_get_drag_atom  (line 2512, static)
- [x] xm_setup_drag_info  (line 2526, static)
- [x] xm_send_drop_message  (line 2563, static)
- [x] xm_send_top_level_enter_message  (line 2588, static)
- [x] xm_send_drag_motion_message  (line 2615, static)
- [x] xm_send_top_level_leave_message  (line 2646, static)
- [x] xm_read_drop_start_reply  (line 2705, static)
- [x] xm_read_drop_start_message  (line 2736, static)
- [x] xm_read_drag_receiver_info  (line 2774, static)
- [x] xm_read_drag_motion_message  (line 2830, static)
- [x] xm_read_drag_motion_reply  (line 2864, static)
- [x] x_dnd_send_xm_leave_for_drop  (line 2897, static)
- [x] x_dnd_free_toplevels  (line 2921, static)
- [x] x_dnd_compute_toplevels  (line 3013, static)
- [x] x_dnd_io_error_handler  (line 3617, static)
- [x] x_dnd_get_target_window_1  (line 3721, static)
- [x] x_dnd_get_wm_state_and_proto  (line 3820, static)
- [x] x_dnd_do_unsupported_drop  (line 3986, static)
- [x] x_dnd_send_unsupported_drop  (line 4082, static)
- [x] x_dnd_fill_empty_target  (line 4128, static)
- [x] x_dnd_get_target_window  (line 4140, static)
- [x] x_dnd_get_window_proxy  (line 4469, static)
- [x] x_dnd_get_window_proto  (line 4511, static)
- [x] x_dnd_send_enter  (line 4545, static)
- [x] x_dnd_send_position  (line 4583, static)
- [x] x_dnd_send_leave  (line 4658, static)
- [x] x_dnd_send_drop  (line 4682, static)
- [x] x_dnd_do_drop  (line 4716, static)
- [x] x_set_dnd_targets  (line 4729, static)
- [x] x_free_dnd_targets  (line 4744, static)
- [x] x_clear_dnd_variables  (line 4758, static)
- [x] x_free_dnd_toplevels  (line 4765, static)
- [x] x_restore_events_after_dnd  (line 4779, static)
- [x] x_dnd_cleanup_drag_and_drop  (line 4867, static)
- [x] x_dnd_note_self_position  (line 4938, static)
- [x] x_dnd_note_self_wheel  (line 4959, static)
- [x] x_dnd_note_self_drop  (line 4987, static)
- [x] x_flush  (line 5074, static)
- [x] x_toolkit_position  (line 5160, static)
- [x] x_update_opaque_region  (line 5197, static)
- [x] x_set_clip_rectangles  (line 6479, static)
- [x] x_reset_clip_rectangles  (line 6495, static)
- [x] x_fill_rectangle  (line 6571, static)
- [x] x_clear_rectangle  (line 6685, static)
- [x] x_draw_rectangle  (line 6733, static)
- [x] x_clear_window  (line 6751, static)
- [x] x_dpyinfo  (line 6883)
- [x] x_display_info_for_display  (line 6895)
- [x] x_find_topmost_parent  (line 6907, static)
- [x] x_set_frame_alpha  (line 6932, static)
- [x] x_update_begin  (line 7419, static)
- [x] x_draw_vertical_window_border  (line 7443, static)
- [x] x_draw_window_divider  (line 7464, static)
- [x] x_flip_and_flush  (line 7576, static)
- [x] x_update_end  (line 7604, static)
- [x] XTframe_up_to_date  (line 7631, static)
- [x] x_clear_under_internal_border  (line 7696)
- [x] x_after_update_window_line  (line 7752, static)
- [x] x_premultiply_pixel  (line 7821, static)
- [x] x_draw_fringe_bitmap  (line 7852, static)
- [x] x_display_set_last_user_time  (line 8087, static)
- [x] x_create_special_window  (line 8206, static)
- [x] x_update_frame_user_time_window  (line 8226, static)
- [x] x_set_last_user_time_from_lisp  (line 8303)
- [x] x_set_cursor_gc  (line 8314, static)
- [x] x_set_mouse_face_gc  (line 8369, static)
- [x] x_set_mode_line_face_gc  (line 8409, static)
- [x] x_set_glyph_string_gc  (line 8420, static)
- [x] x_set_glyph_string_clipping  (line 8462, static)
- [x] x_set_glyph_string_clipping_exactly  (line 8478, static)
- [x] x_compute_glyph_string_overhangs  (line 8496, static)
- [x] x_clear_glyph_string_rect  (line 8530, static)
- [x] x_clear_point  (line 8538, static)
- [x] x_draw_glyph_string_background  (line 8568, static)
- [x] x_draw_glyph_string_foreground  (line 8612, static)
- [x] x_draw_composite_glyph_string_foreground  (line 8706, static)
- [x] x_draw_glyphless_glyph_string_foreground  (line 8816, static)
- [x] x_color_cells  (line 9104, static)
- [x] x_query_colors  (line 9137)
- [x] x_query_frame_background_color  (line 9205, static)
- [x] x_hash_string_ignore_case  (line 9239, static)
- [x] x_parse_color  (line 9260)
- [x] x_alloc_nearest_color_1  (line 9379, static)
- [x] x_alloc_nearest_color  (line 9521)
- [x] x_copy_color  (line 9545)
- [x] x_alloc_lighter_color  (line 9591, static)
- [x] x_setup_relief_color  (line 9668, static)
- [x] x_setup_relief_colors  (line 9713, static)
- [x] x_fill_triangle  (line 9746, static)
- [x] x_make_point  (line 9760, static)
- [x] x_inside_rect_p  (line 9771, static)
- [x] x_draw_relief_rect  (line 9797, static)
- [x] x_draw_box_rect  (line 9999, static)
- [x] x_draw_glyph_string_box  (line 10040, static)
- [x] x_composite_image  (line 10109, static)
- [x] x_draw_image_foreground  (line 10166, static)
- [x] x_draw_image_relief  (line 10289, static)
- [x] x_draw_image_foreground_1  (line 10383, static)
- [x] x_draw_glyph_string_bg_rect  (line 10465, static)
- [x] x_draw_image_glyph_string  (line 10496, static)
- [x] x_draw_stretch_glyph_string  (line 10636, static)
- [x] x_get_scale_factor  (line 10747, static)
- [x] x_draw_underwave  (line 10774, static)
- [x] x_draw_dash  (line 10850, static)
- [x] x_fill_underline  (line 10900, static)
- [x] x_draw_glyph_string  (line 10940, static)
- [x] x_shift_glyphs_for_insert  (line 11309, static)
- [x] x_delete_glyphs  (line 11324, static)
- [x] x_clear_area1  (line 11335, static)
- [x] x_clear_area  (line 11344)
- [x] x_clear_frame  (line 11404, static)
- [x] x_send_hourglass_message  (line 11427, static)
- [x] x_show_hourglass  (line 11449, static)
- [x] x_hide_hourglass  (line 11531, static)
- [x] XTflash  (line 11551, static)
- [x] XTring_bell  (line 11657, static)
- [x] x_ins_del_lines  (line 11696, static)
- [x] x_scroll_run  (line 11705, static)
- [x] x_frame_highlight  (line 11925, static)
- [x] x_frame_unhighlight  (line 11952, static)
- [x] x_new_focus_frame  (line 11990, static)
- [x] make_invisible_cursor  (line 12068, static)
- [x] x_toggle_visible_pointer  (line 12100, static)
- [x] XTtoggle_invisible_pointer  (line 12142, static)
- [x] x_focus_changed  (line 12163, static)
- [x] x_window_to_frame  (line 12232, static)
- [x] x_tooltip_window_to_frame  (line 12285, static)
- [x] x_top_window_to_frame  (line 12489)
- [x] x_next_event_from_any_display  (line 12495, static)
- [x] x_handle_pending_selection_requests_1  (line 12543, static)
- [x] x_handle_pending_selection_requests  (line 12559)
- [x] x_push_selection_request  (line 12573, static)
- [x] x_detect_pending_selection_requests  (line 12584)
- [x] x_clear_dnd_action  (line 12590, static)
- [x] x_dnd_delete_action_list  (line 12597, static)
- [x] x_dnd_lose_ownership  (line 12620, static)
- [x] x_dnd_process_quit  (line 12636, static)
- [x] x_dnd_begin_drag_and_drop  (line 12703)
- [x] x_detect_focus_change  (line 13980, static)
- [x] x_mouse_leave  (line 14039)
- [x] XTframe_rehighlight  (line 14095, static)
- [x] x_frame_rehighlight  (line 14101, static)
- [x] x_find_modifier_meanings  (line 14135, static)
- [x] x_x_to_emacs_modifiers  (line 14309)
- [x] x_emacs_to_x_modifiers  (line 14338)
- [x] get_keysym_name  (line 14371)
- [x] x_compute_root_window_offset  (line 14389, static)
- [x] x_translate_coordinates  (line 14404)
- [x] x_translate_coordinates_to_root  (line 14444)
- [x] x_handle_translate_coordinates  (line 14481)
- [x] x_query_pointer_1  (line 14537, static)
- [x] x_query_pointer  (line 14611)
- [x] x_construct_mouse_click  (line 14668, static)
- [x] x_note_mouse_movement  (line 14739, static)
- [x] x_get_window_below  (line 14787, static)
- [x] x_fast_mouse_position  (line 14862, static)
- [x] XTmouse_position  (line 15001, static)
- [x] x_window_to_scroll_bar  (line 15343, static)
- [x] x_scroll_bar_create  (line 16715, static)
- [x] x_scroll_bar_set_handle  (line 16843, static)
- [x] x_scroll_bar_remove  (line 16964, static)
- [x] XTset_vertical_scroll_bar  (line 16995, static)
- [x] XTset_horizontal_scroll_bar  (line 17120, static)
- [x] XTcondemn_scroll_bars  (line 17262, static)
- [x] XTredeem_scroll_bar  (line 17288, static)
- [x] XTjudge_scroll_bars  (line 17372, static)
- [x] x_scroll_bar_handle_exposure  (line 17402, static)
- [x] x_scroll_bar_redraw  (line 17459, static)
- [x] x_scroll_bar_handle_click  (line 17503, static)
- [x] x_scroll_bar_note_movement  (line 17606, static)
- [x] x_scroll_bar_report_motion  (line 17638, static)
- [x] x_horizontal_scroll_bar_report_motion  (line 17707, static)
- [x] x_scroll_bar_clear  (line 17777, static)
- [x] x_net_wm_state  (line 18011, static)
- [x] x_flush_dirty_back_buffer_on  (line 18043, static)
- [x] mouse_or_wdesc_frame  (line 18095, static)
- [x] x_dnd_compute_tip_xy  (line 18119, static)
- [x] x_dnd_update_tooltip_position  (line 18190, static)
- [x] x_dnd_update_tooltip_now  (line 18229, static)
- [x] x_dnd_update_state  (line 18264, static)
- [x] x_display_pixel_height  (line 18482)
- [x] x_display_pixel_width  (line 18491)
- [x] x_wait_for_cell_change  (line 18502)
- [x] x_find_monitors_changed_event  (line 18619, static)
- [x] x_coords_from_dnd_message  (line 18685, static)
- [x] x_handle_wm_state  (line 18764, static)
- [ ] handle_one_xevent  (line 18876, static)
- [x] x_dispatch_event  (line 25645, static)
- [x] XTread_socket  (line 25672, static)
- [x] x_clip_to_row  (line 25831, static)
- [x] x_draw_hollow_cursor  (line 25857, static)
- [x] x_draw_bar_cursor  (line 25913, static)
- [x] x_define_frame_cursor  (line 26020, static)
- [x] x_clear_frame_area  (line 26032, static)
- [x] x_draw_window_cursor  (line 26041, static)
- [x] x_bitmap_icon  (line 26104, static)
- [x] x_text_icon  (line 26194)
- [x] x_is_serial_more_than  (line 26261, static)
- [x] x_is_serial_more_than_or_equal_to  (line 26270, static)
- [x] x_is_serial_less_than  (line 26279, static)
- [x] x_is_serial_less_than_or_equal_to  (line 26288, static)
- [x] x_find_error_handler  (line 26297, static)
- [x] x_unwind_errors_to  (line 26317)
- [x] x_error_catcher  (line 26331, static)
- [x] x_catch_errors_with_handler  (line 26373)
- [x] x_catch_errors  (line 26390)
- [x] x_request_can_fail  (line 26398, static)
- [x] x_clean_failable_requests  (line 26421, static)
- [x] x_ignore_errors_for_next_request  (line 26464)
- [x] x_stop_ignoring_errors  (line 26520)
- [x] x_uncatch_errors_after_check  (line 26551)
- [x] x_uncatch_errors  (line 26568)
- [x] x_check_errors  (line 26614)
- [x] x_had_errors_p  (line 26651)
- [x] x_clear_errors  (line 26680)
- [x] x_try_restore_frame  (line 26729, static)
- [x] x_connection_closed  (line 26744, static)
- [x] x_error_handler  (line 27000, static)
- [x] x_error_quitter  (line 27082, static)
- [x] x_io_error_quitter  (line 27115, static)
- [x] x_new_font  (line 27137, static)
- [x] x_calc_absolute_position  (line 27497, static)
- [x] x_set_offset  (line 27590, static)
- [x] x_get_wm_check_window  (line 27692, static)
- [x] x_wm_supports_1  (line 27728)
- [x] x_wm_supports  (line 27826)
- [x] set_wm_state  (line 27833, static)
- [x] x_set_sticky  (line 27858)
- [x] x_set_shaded  (line 27870)
- [x] x_set_skip_taskbar  (line 27892)
- [x] x_set_z_group  (line 27927)
- [x] x_get_current_wm_state  (line 27978, static)
- [x] do_ewmh_fullscreen  (line 28090, static)
- [x] XTfullscreen_hook  (line 28222, static)
- [x] x_handle_net_wm_state  (line 28234, static)
- [x] x_check_fullscreen  (line 28270, static)
- [x] x_check_expected_move  (line 28340, static)
- [x] x_sync_with_move  (line 28384, static)
- [x] x_wait_for_event  (line 28431)
- [x] x_set_window_size_1  (line 28484, static)
- [x] x_suspend_background_fills  (line 28565, static)
- [x] x_restore_background_fills  (line 28574, static)
- [x] x_set_window_size  (line 28587)
- [x] x_set_window_size_and_position_1  (line 28624, static)
- [x] x_set_window_size_and_position  (line 28666)
- [x] frame_set_mouse_pixel_position  (line 28705)
- [x] x_raise_frame  (line 28731, static)
- [x] x_lower_frame_1  (line 28746, static)
- [x] x_lower_frame  (line 28779, static)
- [x] XTframe_raise_lower  (line 28801, static)
- [x] xembed_request_focus  (line 28812, static)
- [x] server_timestamp_predicate  (line 28822, static)
- [x] x_get_server_time  (line 28839, static)
- [x] x_ewmh_activate_frame  (line 28882, static)
- [x] x_get_focus_frame  (line 28950, static)
- [x] x_set_input_focus  (line 28964, static)
- [x] x_focus_frame  (line 29024, static)
- [x] xembed_set_info  (line 29151, static)
- [x] xembed_send_message  (line 29166, static)
- [x] x_make_frame_visible  (line 29202)
- [x] x_make_frame_invisible  (line 29407)
- [x] x_make_frame_visible_invisible  (line 29475, static)
- [x] x_create_font_cursor  (line 29484)
- [x] x_iconify_frame  (line 29498)
- [x] x_free_frame_resources  (line 29620)
- [x] x_destroy_window  (line 29857, static)
- [x] x_intern_cached_atom  (line 29897)
- [x] x_get_atom_name  (line 29984)
- [x] x_intern_atoms  (line 30084)
- [x] x_embed_frame  (line 30129)
- [x] x_wm_set_size_hint  (line 30159)
- [x] x_wm_set_window_state  (line 30345, static)
- [x] x_wm_set_icon_pixmap  (line 30363, static)
- [x] x_wm_set_icon_position  (line 30411)
- [x] x_free_pixmap  (line 30448, static)
- [x] same_x_server  (line 30496, static)
- [x] get_bits_and_offset  (line 30546, static)
- [x] x_display_ok  (line 30571)
- [x] x_term_init  (line 30668)
- [x] x_delete_selection_requests  (line 31846, static)
- [x] x_delete_display  (line 31873, static)
- [x] x_delete_terminal  (line 32064)
- [x] x_create_terminal  (line 32233, static)
- [x] x_initialize  (line 32306, static)
- [x] init_xterm  (line 32394)
- [x] mark_xterm  (line 32420)
- [x] x_catch_errors_for_lisp  (line 32476)
- [x] x_check_errors_for_lisp  (line 32485)
- [x] x_uncatch_errors_for_lisp  (line 32493)
- [x] x_preserve_selections  (line 32506)
- [x] x_get_keyboard_modifiers  (line 32643)
- [x] syms_of_xterm  (line 32665)

## Macros defined — 21
- [x] ATOM_REFS_INIT  (line 989, body 38ea51a9ed88)
- [x] HIGHLIGHT_COLOR_DARK_BOOST_LIMIT  (line 9580, body bdbb3f155244)
- [x] OPAQUE  (line 6929, body d524558178b7)
- [x] STORE_KEYSYM_FOR_DEBUG  (line 17826, body faabc0e0dbea)
- [x] SWAPCARD16  (line 1584, body f50347531e4d)
- [x] SWAPCARD32  (line 1583, body 393d2f43a49a)
- [x] XEMBED_VERSION  (line 29148, body b6589fc6ab0d)
- [x] XM_DRAG_OPERATION_IS_LINK  (line 1706, body 19c218f1041c)
- [x] XM_DRAG_PROTOCOL_VERSION  (line 1761, body b6589fc6ab0d)
- [x] XM_DRAG_REASON  (line 1716, body 9456c43f5e20)
- [x] XM_DRAG_REASON_CODE  (line 1718, body 7810497b8cdc)
- [x] XM_DRAG_REASON_ORIGINATOR  (line 1717, body 49d23216e03e)
- [x] XM_DRAG_SIDE_EFFECT  (line 1687, body fcd0aa2b3860)
- [x] XM_DRAG_SIDE_EFFECT_DROP_ACTION  (line 1695, body d8acb947f1f0)
- [x] XM_DRAG_SIDE_EFFECT_OPERATION  (line 1692, body af8a320359fb)
- [x] XM_DRAG_SIDE_EFFECT_SITE_STATUS  (line 1693, body cc712b03bf55)
- [x] XM_DRAG_STYLE_IS_DROP_ONLY  (line 1746, body 8a5d0800d81c)
- [x] XM_DRAG_STYLE_IS_DYNAMIC  (line 1748, body ff5b4bd375aa)
- [x] X_DND_SUPPORTED_VERSION  (line 3626, body ac3478d69a3c)
- [x] X_NEXT_KBD_EVENT  (line 888, body 1bf69f7be3e8)
- [x] x_any_window_to_frame  (line 12486, body c8c88ba3cad5)

## Types and their fields — 14
- [x] x_atom_ref: const char* name, int offset
- [x] x_client_list_window: Window window, Display* dpy, int x, int y, int width, int height, long previous_event_mask, unsigned long wm_state, struct x_client_list_window* next, int frame_extents_left, int frame_extents_right, int frame_extents_top, int frame_extents_bottom, uint8_t xm_protocol_style, bool mapped_p
- [x] x_error_message_stack: char* string, Display* dpy, x_special_error_handler handler, void* handler_data, struct x_error_message_stack* prev, unsigned long first_request
- [x] x_selection_request_event: struct selection_input_event se, struct x_selection_request_event* next
- [x] xm_drag_initiator_info: uint8_t byteorder, uint8_t protocol, uint16_t table_index, uint32_t selection
- [x] xm_drag_motion_message: uint8_t reason, uint8_t byteorder, uint16_t side_effects, uint32_t timestamp, uint16_t x, uint16_t y
- [x] xm_drag_motion_reply: uint8_t reason, uint8_t byte_order, uint16_t side_effects, uint32_t timestamp, uint16_t better_x, uint16_t better_y
- [x] xm_drag_receiver_info: uint8_t byteorder, uint8_t protocol, uint8_t protocol_style, uint8_t unspecified0, uint32_t unspecified1, uint32_t unspecified2, uint32_t unspecified3
- [x] xm_drop_start_message: uint8_t reason, uint8_t byte_order, uint16_t side_effects, uint32_t timestamp, uint16_t x, uint16_t y, uint32_t index_atom, uint32_t source_window
- [x] xm_drop_start_reply: uint8_t reason, uint8_t byte_order, uint16_t side_effects, uint16_t better_x, uint16_t better_y
- [x] xm_targets_table_header: uint8_t byte_order, uint8_t protocol, uint16_t target_list_count, uint32_t total_data_size
- [x] xm_targets_table_rec: uint16_t n_targets, uint32_t targets
- [x] xm_top_level_enter_message: uint8_t reason, uint8_t byteorder, uint16_t zero, uint32_t timestamp, uint32_t source_window, uint32_t index_atom
- [x] xm_top_level_leave_message: uint8_t reason, uint8_t byteorder, uint16_t zero, uint32_t timestamp, uint32_t source_window

## File-scope objects — 80
- [x] static bool any_help_event_p  (line 821, static)
- [x] struct x_display_info x_display_list  (line 826)
- [x] static Time ignore_next_mouse_click_timeout  (line 843, static)
- [x] static struct x_display_info mouse_click_timeout_display  (line 847, static)
- [x] static int x_noop_count  (line 851, static)
- [x] static int x_use_pending_selection_requests  (line 885, static)
- [x] static struct x_selection_request_event pending_selection_requests  (line 974, static)
- [x] bool x_dnd_in_progress  (line 1217)
- [x] struct frame x_dnd_frame  (line 1220)
- [x] struct frame x_dnd_finish_frame  (line 1225)
- [x] bool x_dnd_waiting_for_finish  (line 1231)
- [x] static bool x_dnd_run_unsupported_drop_function  (line 1236, static)
- [x] static Time x_dnd_unsupported_drop_time  (line 1239, static)
- [x] static Window x_dnd_unsupported_drop_window  (line 1242, static)
- [x] static Lisp_Object x_dnd_unsupported_drop_data  (line 1245, static)
- [x] static bool x_dnd_update_tooltip  (line 1249, static)
- [x] static Lisp_Object x_dnd_monitors  (line 1252, static)
- [x] static Display x_dnd_finish_display  (line 1256, static)
- [x] static int x_dnd_waiting_for_motif_finish  (line 1267, static)
- [x] static struct x_display_info x_dnd_waiting_for_motif_finish_display  (line 1271, static)
- [x] static bool x_dnd_xm_use_help  (line 1277, static)
- [x] static bool x_dnd_motif_setup_p  (line 1280, static)
- [x] static Atom x_dnd_motif_atom  (line 1283, static)
- [x] static Window x_dnd_pending_finish_target  (line 1287, static)
- [x] static int x_dnd_waiting_for_finish_proto  (line 1290, static)
- [x] static bool x_dnd_allow_current_frame  (line 1294, static)
- [x] static bool x_dnd_init_type_lists  (line 1298, static)
- [x] static int x_dnd_return_frame  (line 1305, static)
- [x] static struct frame x_dnd_return_frame_object  (line 1309, static)
- [x] static Window x_dnd_last_seen_window  (line 1314, static)
- [x] static Window x_dnd_last_seen_toplevel  (line 1317, static)
- [x] static Window x_dnd_end_window  (line 1321, static)
- [x] static int x_dnd_last_protocol_version  (line 1325, static)
- [x] static bool x_dnd_last_window_is_frame  (line 1329, static)
- [x] static int x_dnd_last_motif_style  (line 1337, static)
- [x] static Time x_dnd_selection_timestamp  (line 1341, static)
- [x] static Window x_dnd_mouse_rect_target  (line 1344, static)
- [x] static XRectangle x_dnd_mouse_rect  (line 1348, static)
- [x] static Window x_dnd_waiting_for_status_window  (line 1352, static)
- [x] static XEvent x_dnd_pending_send_position  (line 1356, static)
- [x] static bool x_dnd_pending_send_position_button  (line 1359, static)
- [x] static int x_dnd_pending_send_position_root_x  (line 1362, static)
- [x] static int x_dnd_pending_send_position_root_y  (line 1365, static)
- [x] static bool x_dnd_need_send_drop  (line 1369, static)
- [x] static int x_dnd_send_drop_proto  (line 1372, static)
- [x] static Atom x_dnd_action  (line 1384, static)
- [x] static Lisp_Object x_dnd_action_symbol  (line 1388, static)
- [x] static Atom x_dnd_wanted_action  (line 1393, static)
- [x] static uint8_t x_dnd_motif_operations  (line 1397, static)
- [x] static uint8_t x_dnd_first_motif_operation  (line 1401, static)
- [x] static Atom x_dnd_targets  (line 1404, static)
- [x] static int x_dnd_n_targets  (line 1407, static)
- [x] static XWindowAttributes x_dnd_old_window_attrs  (line 1413, static)
- [x] static bool x_dnd_unwind_flag  (line 1417, static)
- [x] static struct frame x_dnd_movement_frame  (line 1420, static)
- [x] static int x_dnd_movement_x  (line 1424, static)
- [x] static int x_dnd_movement_y  (line 1424, static)
- [x] static struct frame x_dnd_wheel_frame  (line 1427, static)
- [x] static int x_dnd_wheel_x  (line 1430, static)
- [x] static int x_dnd_wheel_y  (line 1430, static)
- [x] static int x_dnd_wheel_button  (line 1433, static)
- [x] static int x_dnd_wheel_state  (line 1436, static)
- [x] static Time x_dnd_wheel_time  (line 1439, static)
- [x] static sigjmp_buf x_dnd_disconnect_handler  (line 1448, static)
- [x] static bool x_dnd_inside_handle_one_xevent  (line 1452, static)
- [x] static int x_dnd_recursion_depth  (line 1456, static)
- [x] static Lisp_Object x_dnd_selection_alias_cell  (line 1461, static)
- [x] static int x_dnd_last_tooltip_x  (line 1464, static)
- [x] static int x_dnd_last_tooltip_y  (line 1464, static)
- [x] static bool x_dnd_last_tooltip_valid  (line 1467, static)
- [x] static struct x_client_list_window x_dnd_toplevels  (line 1536, static)
- [x] static bool x_dnd_use_toplevels  (line 1540, static)
- [x] static volatile bool xm_drag_window_error  (line 1548, static)
- [x] static struct x_display_info XTread_socket_fake_io_error  (line 17833, static)
- [x] static struct x_display_info next_noop_dpyinfo  (line 17840, static)
- [x] static struct x_error_message_stack x_error_message  (line 26249, static)
- [x] int x_error_message_count  (line 26252)
- [x] static char error_msg  (line 26722, static)
- [x] static bool x_initialized  (line 30491, static)
- [x] static unsigned x_display_id  (line 30595, static)

## Gate mapping

One row per build condition this file's *landed* text carries, in C's order: the file gate is the
one condition the file itself has, and every other row is a sub-gate a flip must set *besides*
`have_x_windows` (P-86 clause 6: setting only the file gate selects a dead arm and reports a
false "done").  Slices 1-7's Decisions record the口径 in prose; this table is the machine-readable
half.  Later slices append their rows.

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`X` in `WINDOW_SYSTEM_OBJ`, i.e. `configure.ac`'s `HAVE_X_WINDOWS`) | `feature = "have_x_windows"` (on every item *and* on the include graph) | an X11 build |
| C 797 `#if defined USE_XCB && defined USE_CAIRO_XCB` -> `USE_CAIRO_XCB_SURFACE` | `all (feature = "use_xcb", feature = "use_cairo_xcb")` | the XCB cairo build |
| C 801-805 `#if XCB_SHAPE_MAJOR_VERSION > 1 || (== 1 && MINOR >= 1)` -> `HAVE_XCB_SHAPE_INPUT_RECTS` | `feature = "have_xcb_shape_input_rects"` (the derived name, Decision 3) | xcb shape >= 1.1 |
| C 807-816 `#ifdef USE_GTK` (the pending quit event) | `feature = "use_gtk"` | a GTK build |
| C 828-837 `#ifdef USE_X_TOOLKIT` (the Xt objects) | `feature = "use_x_toolkit"` | an Xt build |
| C 858-861 `#ifdef HAVE_X_I18N` (`emacs_class`) | `feature = "have_x_i18n"` | an XIM build |
| C 863-867 `#ifdef USE_GTK` (the XTread_socket counters) | `feature = "use_gtk"` | a GTK build |
| C 869-881 `#ifndef X_XIGrabDevice`/`X_XIUngrabDevice`/`X_XIAllowEvents` | the guard reading (constant, no `cfg`) | an XI2 build |
| C 891-990 slices 1 (the selection-request chain) | `feature = "have_x_windows"` | an X11 build |
| C 1214-1548 the drag-and-drop state | `all (feature = "have_x_windows", feature = "have_xinput2")` on the XI2 pair | an XI2 build |
| C 1550/1552/1553 the `XM_BYTE_ORDER_*` / `SWAPCARD*` arms | `feature = "words_bigendian"`, `feature = "enable_checking"` | big-endian / `--enable-checking` |
| C 1763-2173 slices 3 (Motif map, targets table, drag window) | `all (feature = "have_x_windows", feature = "use_motif")` where C guards it | a Motif drag build |
| C 2175-3614 slices 4-6 (the Motif senders/readers, the toplevel list) | `feature = "have_x_windows"` + the inner arms below | an X11 build |
| C 3013-3614 slice 6's `USE_XCB`/`HAVE_XCB_SHAPE`/`HAVE_XCB_SHAPE_INPUT_RECTS` arms | `feature = "use_xcb"`, `feature = "have_xcb_shape"`, `feature = "have_xcb_shape_input_rects"` | an XCB build |
| C 3616-4466 slice 7's `USE_XCB`, `HAVE_XSHAPE`, `HAVE_XCOMPOSITE` arms | `feature = "use_xcb"`, `feature = "have_xshape"`, `all (feature = "have_xcomposite", feature = "have_xcomposite_overlay")` (the derived pair, Decision 18) | an XCB / XShape / composite build |
| C 4804-4864 `#ifdef HAVE_XINPUT2` (`x_dnd_cancel_dnd_early`) | `feature = "have_xinput2"` | an XI2 build |
| C 4791-4792 `#ifdef HAVE_XKB` (`x_restore_events_after_dnd`) | `feature = "have_xkb"` | an XKB build |
| C 4921-4923 `#ifdef USE_GTK` (`current_hold_quit`) | `feature = "use_gtk"` | a GTK build |
| C 5087-5103 `#ifdef HAVE_XDBE` | `feature = "have_xdbe"` | an Xdbe build |
| C 5105-5118 `#ifdef HAVE_XRENDER` (`x_xr_ensure_picture`) | `feature = "have_xrender"` | an XRender build |
| C 5127-5156 `#if false` (`record_event`) | dead text in every configuration (comment) | - |
| C 5163-5193 `#ifdef USE_GTK` / `#elif defined USE_X_TOOLKIT` / `#else` | `feature = "use_gtk"` / `all (not (feature = "use_gtk"), feature = "use_x_toolkit")` / neither | a GTK / Xt / plain X11 build |
| C 5215-5239 `#ifndef HAVE_GTK3` / `#else` | `not (feature = "have_gtk3")` / `feature = "have_gtk3"` | GTK3 or not |
| C 5245-5287 `#if defined USE_CAIRO || defined HAVE_XRENDER` (the GC ext data) | `any (feature = "use_cairo", feature = "have_xrender")` | a cairo or XRender build |
| C 5289-5297 `#ifdef USE_CAIRO` (`FRAME_CR_*`) | `feature = "use_cairo"` | a cairo build |
| C 5303-5326 `#ifdef HAVE_XINPUT2` + `# if defined USE_X_TOOLKIT || !(defined USE_GTK && defined HAVE_GTK3)` | `feature = "have_xinput2"` + `any (feature = "use_x_toolkit", not (all (feature = "use_gtk", feature = "have_gtk3")))` | an XI2 build that is not GTK3-GTK |
| C 5350-5352 `#ifdef USE_GTK static #endif` (`xi_convert_event_state`) | `feature = "use_gtk"` for the storage class only | a GTK build |
| C 5382-5959 the `HAVE_XINPUT2_1` / `HAVE_XINPUT2_2` device arms | `feature = "have_xinput2_1"` / `feature = "have_xinput2_2"` | XI2 >= 2.1 / 2.2 |
| C 5966-6451 `#ifdef USE_CAIRO` (the cairo helpers) | `feature = "use_cairo"` + the `CAIRO_HAS_*` derived names below | a cairo build |
| C 6019-6026 `#ifdef USE_CAIRO_XCB_SURFACE` | `all (feature = "use_xcb", feature = "use_cairo_xcb")` | an XCB cairo build |
| C 6361-6395 `#ifdef CAIRO_HAS_PDF_SURFACE` / `_PNG_FUNCTIONS` / `_PS_SURFACE` / `_SVG_SURFACE` | the derived names `cairo_has_pdf_surface`, `cairo_has_png_functions`, `cairo_has_ps_surface`, `cairo_has_svg_surface` (cairo.h's own pair shape, Decision 3) | a cairo build with that surface |
| C 6453-6477 `#if defined HAVE_XRENDER` (the ext clip) | `feature = "have_xrender"` | an XRender build |
| C 6479-6509 `#if defined USE_CAIRO || defined HAVE_XRENDER` (the clip bookkeeping) | `any (feature = "use_cairo", feature = "have_xrender")` | a cairo or XRender build |
| C 6511-6538 `# if !defined USE_CAIRO && (RENDER_MAJOR > 0 || RENDER_MINOR >= 2)` | `all (not (feature = "use_cairo"), feature = "have_xrender_0_2")` (the derived name) | an XRender >= 0.2 build |
| C 6594-6629 `#if RENDER_MAJOR > 0 || (RENDER_MINOR >= 10)` | `feature = "have_xrender_0_10"` (the derived name) | an XRender >= 0.10 build |
| C 6757-6774 `#ifndef USE_GTK` + `#ifdef HAVE_XDBE` inside its condition | `not (feature = "use_gtk")`, `feature = "have_xdbe"` | a non-GTK Xdbe build |
| C 6779-6865 `#ifdef USE_CAIRO` (the relief trapezoids, corners, wave) | `feature = "use_cairo"` | a cairo build |
| C 6994 `#if defined HAVE_XSYNC && !defined USE_GTK && defined HAVE_CLOCK_GETTIME` | `all (feature = "have_xsync", not (feature = "use_gtk"), feature = "have_clock_gettime")` | an XSync build that is not GTK, with `clock_gettime` |
| C 7249 / 7369 `#ifdef HAVE_XSYNCTRIGGERFENCE` | `feature = "have_xsynctriggerfence"` | an XSync build whose fences are available |
| C 7101 / 7271 `#ifdef FRAME_DEBUG` | `feature = "frame_debug"` | the source-level knob C defines by hand (the `glyph_debug` shape) |
| C 7818 / 8535 / 9744 `#ifndef USE_CAIRO` (the three non-cairo helpers) | `not (feature = "use_cairo")` | a build without cairo |
| C 7421-7437 / 7588 / 7609-7624 / 7637-7643 / 7674 `HAVE_XDBE` arms | `feature = "have_xdbe"` (the C 5087 row's gate, carried on) | an Xdbe build |
| C 7636 / 7888 / 8659-8661 / 8733-8809 `USE_CAIRO` arms | `feature = "use_cairo"` (the C 5289/5966 row's gate, carried on) | a cairo build |
| C 8181 `#ifdef USE_GTK` (`x_set_gtk_user_time`) | `feature = "use_gtk"` | a GTK build |
| C 8198 `#if !defined USE_GTK \|\| defined HAVE_XFIXES` | `any (not (feature = "use_gtk"), feature = "have_xfixes")` | a non-GTK or XFixes build |
| C 8223-8300 `#ifndef USE_GTK` | `not (feature = "use_gtk")` | a non-GTK build |
| C 8834/8836 `#ifdef GCC_LINT` / `#else` | `feature = "gcc_lint"` / `not (feature = "gcc_lint")` | `GCC_LINT` is 1 in this configuration (conf_post.h 486), so the first arm is the live text |
| C 8900 `#ifdef USE_LUCID` (inside `USE_X_TOOLKIT`) | `all (feature = "use_x_toolkit", feature = "use_lucid")` | a Lucid widget build |
| C 9506 / 9562 `#ifdef DEBUG_X_COLORS` | `feature = "debug_x_colors"` | the source-level knob C defines by hand |
| C 9802 / 9874 `#ifdef USE_CAIRO` (`x_draw_relief_rect`'s two arms) | `feature = "use_cairo"` / `not (feature = "use_cairo")` | a cairo build / none |
| C 10106 / 10160 `#ifndef USE_CAIRO` (`x_composite_image`) | `not (feature = "use_cairo")` | a non-cairo build |
| C 10110-10112 / 10117 / 10124 `HAVE_XRENDER` (`x_composite_image`'s parameter and its two calls) | `feature = "have_xrender"` (the C 5105 row's gate, carried on) | an XRender build |
| C 10498-10504 / 10526 / 10589 `#ifndef USE_CAIRO` (`x_draw_image_glyph_string`) | `not (feature = "use_cairo")` | a non-cairo build |
| C 10557 `#if defined HAVE_XRENDER && (RENDER_MAJOR > 0 \|\| (RENDER_MINOR >= 2))` | `all (feature = "have_xrender", feature = "have_xrender_0_2")` (the C 6511 row's pair) | an XRender >= 0.2 build |
| C 10785 / 10842 `#ifdef USE_CAIRO` (`x_draw_underwave`) | `feature = "use_cairo"` / `not (feature = "use_cairo")` | a cairo build / none |
| C 10853 / 10892 `#ifndef USE_CAIRO` (`x_draw_dash`) | `not (feature = "use_cairo")` / `feature = "use_cairo"` | a non-cairo build / a cairo one |
| C 11333 `#if defined USE_GTK \|\| !defined USE_CAIRO` (`x_clear_area1`) | `any (feature = "use_gtk", not (feature = "use_cairo"))` | a GTK or non-cairo build |
| C 11346 / 11397 `#ifdef USE_CAIRO` (`x_clear_area`) | `feature = "use_cairo"` / `not (feature = "use_cairo")` | a cairo build / none |
| C 11350 / 11358 / 11392 `#ifndef USE_GTK` (`x_clear_area`) | `not (feature = "use_gtk")` | a non-GTK build |
| C 11355 `#ifdef HAVE_XDBE` (inside that condition) | `feature = "have_xdbe"` (the C 5087 row's gate, carried on) | an Xdbe build |
| C 11366 `#if defined HAVE_XRENDER && (RENDER_MAJOR > 0 \|\| (RENDER_MINOR >= 2))` (`x_clear_area`) | `all (feature = "have_xrender", feature = "have_xrender_0_2")` | an XRender >= 0.2 build |
| C 11468 / 11470 `#ifdef USE_X_TOOLKIT` (`x_show_hourglass`) | `feature = "use_x_toolkit"` | an Xt build |
| C 11476 / 11510 `#ifndef USE_XCB` and C 11489 / 11514 `#else` (`x_show_hourglass`) | `not (feature = "use_xcb")` / `feature = "use_xcb"` | the two XCB arms |
| C 11479 / 11491 `#ifdef USE_GTK` (`parent`) and C 11538 / 11540 (`x_hide_hourglass`) | `feature = "use_gtk"` / `not (feature = "use_gtk")`, then `not (feature = "use_xcb")` / `feature = "use_xcb"` | the GTK and non-GTK arms |
| C 11677 `#ifdef HAVE_XKB` (`XTring_bell`) | `feature = "have_xkb"` (the C 4791 row's gate, carried on) | an XKB build |
| C 11753-11843 `#ifdef HAVE_XWIDGETS` (`x_scroll_run`) | `feature = "have_xwidgets"` | an xwidget build |
| C 11845 / 11872 `#ifdef USE_CAIRO_XCB_SURFACE` (`x_scroll_run`) | `all (feature = "use_xcb", feature = "use_cairo_xcb")` (the C 6019 row's pair) | an XCB cairo build |
| C 11853 / 11906 `#ifdef USE_CAIRO` (`x_scroll_run`) | `feature = "use_cairo"` / `not (feature = "use_cairo")` | a cairo build / none |

## C line ranges

One row per C block, in C's order - the table P-86 clause 2 requires of a file whose body the
frozen configuration leaves false, and the same shape `docs/ports/w32term.md`'s table uses: a
row's range runs to the line before the next row's start, so the table is contiguous and every
non-blank line of the reference is accounted for (`check-dead-file-coverage.py` reads it).  The
`arm` cell says what the port does with the block; the per-function lines of slices 1-8 are in
their Decisions, and the ones of slice 9 are in Decision 21.

| C | block | arm |
| --- | --- | --- |
| 1-624 | the file's own licence/declaration block and the long X11/input prose comment | dropped (P-86 clause 1; the port opens with the `// femacs: port of ...` line) |
| 625-635 | `<config.h>`, `<stdlib.h>`, `<math.h>`, `<signal.h>` | the file gate |
| 636-645 | `"lisp.h"`, `"blockinput.h"`, `"sysstdio.h"`, `"xterm.h"`, `"textconv.h"`, the X11 headers | the file gate |
| 646-701 | the X11 extension headers, `"sys/types.h"` ... `<byteswap.h>` | the file gate |
| 702-793 | `"character.h"` ... `"widget.h"`, the XKB/XI2 headers, `xgselect.h`, `bitmaps/gray.xbm` | the file gate (the two last under `USE_GTK`/`#if 0`) |
| 794-806 | the two X error-handler typedefs, `USE_CAIRO_XCB_SURFACE`, `HAVE_XCB_SHAPE_INPUT_RECTS` | the file gate; both arms (`use_xcb`+`use_cairo_xcb`, `have_xcb_shape_input_rects`) |
| 807-899 | the file-scope objects: `xg_pending_quit_event`, `any_help_event_p`, `x_display_list`, the Xt objects, `x_noop_count`, `emacs_class`, the XTread_socket counters, the XI2 request codes, `X_NEXT_KBD_EVENT` | the file gate + `use_gtk`/`use_x_toolkit`/`have_x_i18n`/`have_xinput2` arms |
| 900-990 | slice 1: `x_defer_selection_requests`, `x_release_selection_requests`, `x_release_selection_requests_and_flush`, `ATOM_REFS_INIT` | the file gate |
| 991-1130 | the `x_atom_refs` table (C 976-987's `x_atom_ref` record and the 122 rows) | **not landed** (Decision 9) |
| 1131-1762 | slice 2: the `xembed` enums, `X_EVENT_*`, the drag-and-drop state, the five `XM_DRAG_*` families and the two byte-order constants | the file gate + `words_bigendian`/`enable_checking` |
| 1763-2173 | slice 3: the Motif side-effect map, the targets-table reader/writer, the two error handlers, `x_special_window_exists_p` and the drag window | the file gate |
| 2175-2612 | slice 4: `xm_setup_dnd_targets`, the drag atom, the three Motif message senders | the file gate |
| 2614-3012 | slice 5: the Motif senders and readers, the toplevel teardown | the file gate |
| 3013-3614 | slice 6: `x_dnd_free_toplevels`, `x_dnd_compute_toplevels` and the `x11_ext` arm's five members | the file gate + `use_xcb`/`have_xcb_shape`/`have_xcb_shape_input_rects` |
| 3616-4466 | slice 7: the drag-and-drop target-window discovery (`x_dnd_io_error_handler` ... `x_dnd_get_target_window`) | the file gate + `use_xcb`/`have_xshape`/`have_xcomposite`(+`_overlay`) |
| 4467-4510 | `x_dnd_get_window_proxy` (C 4468-4508) | the file gate |
| 4511-4544 | `x_dnd_get_window_proto` (C 4510-4542) | the file gate |
| 4545-4582 | `x_dnd_send_enter` (C 4544-4580) | the file gate |
| 4583-4657 | `x_dnd_send_position` (C 4582-4655) | the file gate |
| 4658-4681 | `x_dnd_send_leave` (C 4657-4679) | the file gate |
| 4682-4715 | `x_dnd_send_drop` (C 4681-4713) | the file gate |
| 4716-4728 | `x_dnd_do_drop` (C 4715-4726) | the file gate |
| 4729-4743 | `x_set_dnd_targets` (C 4728-4741) | the file gate |
| 4744-4757 | `x_free_dnd_targets` (C 4743-4755) | the file gate |
| 4758-4764 | `x_clear_dnd_variables` (C 4757-4762) | the file gate |
| 4765-4778 | `x_free_dnd_toplevels` (C 4764-4776) | the file gate |
| 4779-4864 | `x_restore_events_after_dnd` (C 4778-4802, with C 4788's `HAVE_XKB` arm) and C 4804-4864's `x_dnd_cancel_dnd_early` | the file gate; `have_xkb`; `have_xinput2` |
| 4865-4937 | `x_dnd_cleanup_drag_and_drop` (C 4866-4919) and C 4921-4923's `USE_GTK` arm | the file gate; `use_gtk` |
| 4938-4986 | `x_dnd_note_self_position` (C 4937-4956) and `x_dnd_note_self_wheel` (C 4958-4984) | the file gate |
| 4987-5073 | `x_dnd_note_self_drop` (C 4986-5071) | the file gate |
| 5074-5086 | `x_flush` (C 5073-5085) | the file gate |
| 5087-5118 | `x_drop_xrender_surfaces` (C 5088-5103) and `x_xr_ensure_picture` (C 5105-5118) | the file gate; `have_xdbe`, `have_xrender` |
| 5119-5157 | the `Debugging` banner and C 5127-5153's `#if false` `record_event` | dead text in every configuration (C's own text inside a comment) |
| 5158-5196 | `x_toolkit_position` (C 5159-5194) | the file gate + `use_gtk`/`use_x_toolkit`/neither |
| 5197-5243 | `x_update_opaque_region` (C 5196-5243) | the file gate + `have_gtk3`/not |
| 5244-5287 | `x_gc_free_ext_data_private`, `x_gc_get_ext_data`, `x_extension_initialize` (C 5245-5287) | `any (feature = "use_cairo", feature = "have_xrender")` |
| 5288-5297 | the `FRAME_CR_*` macros (C 5289-5297) | `use_cairo` |
| 5298-5328 | `xi_frame_selected_for` (C 5303-5328) | `have_xinput2` + C 5305's toolkit arm |
| 5329-5378 | `xi_convert_button_state`, `xi_convert_event_state`, `xi_convert_event_keyboard_state` (C 5331-5378) | `have_xinput2` (+ `use_gtk` for the storage class) |
| 5379-5427 | `x_free_xi_devices` (C 5380-5427) | `have_xinput2` + `have_xinput2_1`/`_2` |
| 5428-5706 | `struct xi_known_valuator`, `xi_populate_scroll_valuator`, `xi_populate_device_from_info` (C 5430-5706) | `have_xinput2` + `have_xinput2_1`/`_2` |
| 5707-5792 | `x_cache_xi_devices` (C 5708-5740) and `x_get_scroll_valuator_delta` (C 5742-5792) | `have_xinput2` (+ `have_xinput2_1`) |
| 5793-5811 | `xi_device_from_id` (C 5794-5811) | `have_xinput2` |
| 5812-5963 | `xi_link_touch_point`, `xi_unlink_touch_point`, `xi_unlink_touch_points`, `xi_find_touch_point`, `xi_reset_scroll_valuators_for_device_id` (C 5813-5963) | `have_xinput2_2` / `have_xinput2_1` |
| 5964-6451 | the cairo block: `x_cr_destroy_frame_context` ... `x_cr_export_frames` (C 5966-6451) | `use_cairo` + `cairo_has_*` + `use_xcb`+`use_cairo_xcb` |
| 6452-6509 | `x_xr_apply_ext_clip`, `x_xr_reset_ext_clip` (C 6453-6477) and `x_set_clip_rectangles`, `x_reset_clip_rectangles` (C 6479-6509) | the file gate; `have_xrender`; `any (use_cairo, have_xrender)` for the bookkeeping |
| 6510-6569 | `x_xrender_color_from_gc_foreground` (C 6511-6538) and `x_xrender_color_from_gc_background` (C 6541-6569) | `have_xrender` + `not (use_cairo)` + `have_xrender_0_2` |
| 6570-6684 | `x_fill_rectangle` (C 6571-6683) | the file gate + `use_cairo` / `have_xrender`+`have_xrender_0_2`+`have_xrender_0_10` |
| 6685-6732 | `x_clear_rectangle` (C 6685-6731) | as above, without the `0_10` arm |
| 6733-6750 | `x_draw_rectangle` (C 6733-6749) | the file gate + `use_cairo` |
| 6751-6777 | `x_clear_window` (C 6751-6777) | the file gate + `use_cairo` / `not (use_gtk)`+`have_xdbe` |
| 6778-6865 | `x_fill_trapezoid_for_relief`, `enum corners`, `x_erase_corners_for_relief`, `x_draw_horizontal_wave` (C 6779-6865) | `use_cairo` |
| 6866-6893 | `x_dpyinfo` (C 6883-6893) | the file gate |
| 6894-6905 | `x_display_info_for_display` (C 6895-6905) | the file gate |
| 6906-6928 | `x_find_topmost_parent` (C 6907-6928) | the file gate |
| 6929-6989 | `OPAQUE` (C 6929) and `x_set_frame_alpha` (C 6932-6989) | the file gate |
| 6990-6993 | the section banner (`Starting and ending an update`, C's boxed comment) | carried as a comment |
| 6994-7410 | C 6994's `#if defined HAVE_XSYNC && !defined USE_GTK && defined HAVE_CLOCK_GETTIME`: `x_if_event` (C 7002), `x_sync_get_monotonic_time` (7053), `x_sync_current_monotonic_time` (7071), `x_sync_note_frame_times` (7084, with C 7101's `FRAME_DEBUG` block), `x_sync_is_frame_drawn_event` (7109), `x_sync_wait_for_frame_drawn_event` (7132), `x_sync_update_begin` (7187), C 7249's `HAVE_XSYNCTRIGGERFENCE` trio (`x_sync_trigger_fence` with C 7271's `FRAME_DEBUG` block, `x_sync_init_fences`, `x_sync_free_fences`), `x_sync_update_finish` (7339, with C 7369's inner arm) and `x_sync_handle_frame_drawn` (7383) | `all (feature = "have_x_windows", feature = "have_xsync", not (feature = "use_gtk"), feature = "have_clock_gettime")`, plus `have_xsynctriggerfence` and `frame_debug` |
| 7412-7438 | `x_update_begin` (C 7419) | the file gate; C 7421's condition again inside, plus `have_xdbe` (C 7424/7432) |
| 7440-7459 | `x_draw_vertical_window_border` (C 7443) | the file gate; `use_cairo` / not (C 7453) |
| 7461-7524 | `x_draw_window_divider` (C 7464) | the file gate |
| 7526-7570 | `#ifdef HAVE_XDBE`: `show_back_buffer` (C 7533) | `have_xdbe`, plus the `have_xsync` triple and `use_cairo` for its inner arms |
| 7572-7598 | `x_flip_and_flush` (C 7576) | the file gate; `have_xdbe` (C 7588) |
| 7600-7625 | `x_update_end` (C 7604) | the file gate; `use_cairo`+`have_xdbe` (C 7609-7614); the `have_xsync` triple (C 7619) |
| 7627-7672 | `XTframe_up_to_date` (C 7631) | the file gate; `have_xdbe` (C 7637); `have_xsync`+`not have_gtk3` / `have_xsync`+`have_gtk3` (C 7646-7670) |
| 7674-7685 | `#ifdef HAVE_XDBE`: `XTbuffer_flipping_unblocked_hook` (C 7676) | `have_xdbe` (no checklist box: dead in the frozen reading, carried per P-51) |
| 7687-7743 | `x_clear_under_internal_border` (C 7696) | the file gate (C's definition is not `static`) |
| 7745-7809 | `x_after_update_window_line` (C 7752) | the file gate; `use_x_toolkit` (C 7759's block) |
| 7811-7849 | `#ifndef USE_CAIRO`: `x_premultiply_pixel` (C 7821) | `not (feature = "use_cairo")` |
| 7851-8050 | `x_draw_fringe_bitmap` (C 7852) | the file gate; `use_cairo` / not (C 7888/7916) and `have_xrender` (C 7926-7931, 7984-8033) |
| 8052-8062 | the `Glyph display` banner and C 8056-8062's three forward declarations | carried as comments: PORTING.md has no item for a prototype (slice 1's `x_push_selection_request` precedent) |
| 8064-8176 | `x_display_set_last_user_time` (C 8087) | the file gate; the `have_xsync` triple (C 8090); `not use_gtk` (C 8095, 8158); `enable_checking` (C 8103) |
| 8178-8196 | `#ifdef USE_GTK`: `x_set_gtk_user_time` (C 8181) | `use_gtk` |
| 8198-8218 | `#if !defined USE_GTK \|\| defined HAVE_XFIXES`: `x_create_special_window` (C 8206) | `any (not (feature = "use_gtk"), feature = "have_xfixes")` |
| 8220-8300 | `#ifndef USE_GTK`: `x_update_frame_user_time_window` (C 8226) | `not (feature = "use_gtk")` |
| 8302-8307 | `x_set_last_user_time_from_lisp` (C 8303) | the file gate (C's definition is not `static`) |
| 8309-8363 | `x_set_cursor_gc` (C 8314) | the file gate |
| 8365-8401 | `x_set_mouse_face_gc` (C 8369) | the file gate |
| 8403-8412 | `x_set_mode_line_face_gc` (C 8409) | the file gate |
| 8414-8455 | `x_set_glyph_string_gc` (C 8420) | the file gate |
| 8457-8470 | `x_set_glyph_string_clipping` (C 8462) | the file gate |
| 8472-8489 | `x_set_glyph_string_clipping_exactly` (C 8478) | the file gate |
| 8491-8524 | `x_compute_glyph_string_overhangs` (C 8496) | the file gate |
| 8526-8533 | `x_clear_glyph_string_rect` (C 8530) | the file gate |
| 8535-8559 | `#ifndef USE_CAIRO`: `x_clear_point` (C 8538) | `not (feature = "use_cairo")` |
| 8561-8606 | `x_draw_glyph_string_background` (C 8568) | the file gate |
| 8608-8701 | `x_draw_glyph_string_foreground` (C 8612) | the file gate; `use_cairo` (C 8640-8699) |
| 8703-8810 | `x_draw_composite_glyph_string_foreground` (C 8706) | the file gate; `use_cairo` (C 8733-8809) |
| 8812-8896 | `x_draw_glyphless_glyph_string_foreground` (C 8816) | the file gate; `gcc_lint` (C 8834's `#ifdef GCC_LINT` pair) |
| 8898-9095 | `#ifdef USE_X_TOOLKIT`: `x_frame_of_widget` (C 8905) and `x_alloc_lighter_color_for_widget` (8941) under C 8900's `USE_LUCID`, C 8952-8967's two objects, `cvt_string_to_pixel` (8987) and `cvt_pixel_dtor` (9073) | `use_x_toolkit` (+ `use_lucid`); the `XtConvertArgRec` array is recorded as C's text (no libXt on this machine) |
| 9097-9126 | `x_color_cells` (C 9104) | the file gate |
| 9128-9200 | `x_query_colors` (C 9137) | the file gate (C's definition is not `static`) |
| 9202-9236 | `x_query_frame_background_color` (C 9205) | the file gate; `use_cairo` / not (C 9208-9230) |
| 9238-9251 | `x_hash_string_ignore_case` (C 9239) | the file gate |
| 9253-9370 | `x_parse_color` (C 9260) | the file gate (C's definition is not `static`) |
| 9372-9512 | `x_alloc_nearest_color_1` (C 9379) | the file gate; `debug_x_colors` (C 9506) |
| 9514-9537 | `x_alloc_nearest_color` (C 9521) | the file gate (C's definition is not `static`) |
| 9539-9566 | `x_copy_color` (C 9545) | the file gate (C's definition is not `static`); `debug_x_colors` (C 9562) |
| 9568-9580 | the `HIGHLIGHT_COLOR_DARK_BOOST_LIMIT` comment and `#define HIGHLIGHT_COLOR_DARK_BOOST_LIMIT 48000` (C 9580) | the file gate (a value: the port writes the constant) |
| 9583-9657 | `x_alloc_lighter_color` (C 9591) | the file gate |
| 9659-9707 | `x_setup_relief_color` (C 9668) | the file gate |
| 9709-9742 | `x_setup_relief_colors` (C 9713) | the file gate |
| 9744-9785 | `#ifndef USE_CAIRO`: `x_fill_triangle` (C 9746), `x_make_point` (C 9760) and `x_inside_rect_p` (C 9771) | `not (feature = "use_cairo")` |
| 9786-9988 | `x_draw_relief_rect` (C 9797) | the file gate; `use_cairo` for C 9802's arm and `not (feature = "use_cairo")` for C 9874's |
| 9990-10034 | `x_draw_box_rect` (C 9999) | the file gate |
| 10036-10103 | `x_draw_glyph_string_box` (C 10040) | the file gate |
| 10105-10160 | `#ifndef USE_CAIRO`: `x_composite_image` (C 10109) | `not (feature = "use_cairo")`; C 10110-10112's `HAVE_XRENDER` adds the `destination` parameter, so the unit carries one definition per arm under `feature = "have_xrender"` |
| 10162-10283 | `x_draw_image_foreground` (C 10166) | the file gate; `use_cairo`/`not (feature = "use_cairo")`; `have_xrender` for C 10249-10252's and C 10265-10269's extra argument |
| 10285-10376 | `x_draw_image_relief` (C 10289) | the file gate |
| 10378-10458 | `#ifndef USE_CAIRO`: `x_draw_image_foreground_1` (C 10383) | `not (feature = "use_cairo")`; `have_xrender` for C 10425's `None` argument |
| 10460-10478 | `x_draw_glyph_string_bg_rect` (C 10465) | the file gate |
| 10480-10630 | `x_draw_image_glyph_string` (C 10496) | the file gate; `not (use_cairo)`/`use_cairo` for C 10498-10504's block, and `have_xrender`+`have_xrender_0_2` for C 10557's `RENDER_MAJOR`/`RENDER_MINOR` pair |
| 10632-10744 | `x_draw_stretch_glyph_string` (C 10636) | the file gate |
| 10745-10761 | `x_get_scale_factor` (C 10747) | the file gate |
| 10763-10843 | `x_draw_underwave` (C 10774) | the file gate; `use_cairo` / `not (feature = "use_cairo")` |
| 10845-10893 | `x_draw_dash` (C 10850) | the file gate; `not (feature = "use_cairo")` / `use_cairo` |
| 10895-10935 | `x_fill_underline` (C 10900) | the file gate |
| 10937-11304 | `x_draw_glyph_string` (C 10940) | the file gate |
| 11306-11318 | `x_shift_glyphs_for_insert` (C 11309) | the file gate |
| 11320-11327 | `x_delete_glyphs` (C 11324) | the file gate |
| 11329-11341 | `#if defined USE_GTK \|\| !defined USE_CAIRO`: `x_clear_area1` (C 11335) and C 11341's `#endif` | `any (feature = "use_gtk", not (feature = "use_cairo"))` |
| 11342-11398 | `x_clear_area` (C 11344) | the file gate (C's definition is not `static`); `use_cairo`/`not (use_cairo)`; C 11350-11356's `#ifndef USE_GTK` pair and `not (feature = "use_gtk")`, C 11355's `HAVE_XDBE` term; `have_xrender`+`have_xrender_0_2` for C 11366-11377 |
| 11400-11419 | `x_clear_frame` (C 11404) | the file gate |
| 11421-11444 | `x_send_hourglass_message` (C 11427) | the file gate |
| 11446-11526 | `x_show_hourglass` (C 11449) | the file gate; `use_x_toolkit` for C 11468-11472; `use_xcb`/`not (feature = "use_xcb")` for the two blocks, `use_gtk` inside each |
| 11528-11546 | `x_hide_hourglass` (C 11531) | the file gate; `not (feature = "use_xcb")` / `use_xcb` |
| 11548-11652 | `XTflash` (C 11551) | the file gate |
| 11654-11686 | `XTring_bell` (C 11657) | the file gate; `have_xkb`/`not (feature = "have_xkb")` for C 11677-11681 |
| 11688-11699 | the `Line Dance` banner (C 11688-11690, C's boxed comment), C 11692-11693's comment and `x_ins_del_lines` (C 11696) | the file gate; the banner and the comment are carried as comments (slice 1's `x_push_selection_request` precedent) |
| 11701-11915 | `x_scroll_run` (C 11705) | the file gate; `have_xwidgets` for C 11753-11843; `all (feature = "use_xcb", feature = "use_cairo_xcb")` for C 11845-11851 and for C 11872-11883's `else if`; `use_cairo` for C 11853-11906 |
| 11916-33125 | **the remainder** (C 11925's `x_frame_highlight` on: the frame highlight/focus hooks, the FX/pointer blanking, `XTread_socket` and the whole event loop, the toolkit and scroll-bar code, `x_initialize` and `syms_of_xterm`) | not in this slice: recorded here so the table covers the reference (`check-dead-file-coverage.py` requires it), to be replaced by the next lane's rows (the next start is **C 11916**, the comment that documents `x_frame_highlight`) |

## Decisions

### 1. What this lane is for (and what it is not)

`src/xterm.c` is the X11 backend.  It is not a member of the frozen build, so its port is
**text carried for an X11 target** - a 1:1 transcript of the reference file (PORTING.md P-42: the
scope is the reference tree's own files, not the frozen link list; the same rule that made
`vm-limit.rs` a 1:1 port of a file this configuration does not link).  Two consequences have to
be said out loud, because they are easy to get wrong later:

* **The final Lisp test suite never reaches this file.**  The frozen darwin-arm64 build has no
  window system, so no code path in it calls anything here; a green or red ERT run says nothing
  about this unit either way.  Its evidence is the gate readings below and, one day, a build of
  the X11 target - not a test result from this repository.
* **It is not a compilation unit here.**  Everything the slice names that lives in X11 headers
  (`Display`, `XErrorEvent`, `XtAppContext`, the `XfooMask` constants, ...) has no Rust
  counterpart in this tree, so with `have_x_windows` on the mirrors themselves do not compile
  (140 of the 144 errors the probe in Evidence measures are in `include/xterm.rs` and friends,
  and every one of them is an X11-header name).  That is exactly the state
  `femacs/src/include/xterm.rs` is in, and why `xtask build-tu src/xterm.c` proves parsing, not
  typing.

### 2. Which configuration counts as "live": measured, and what I recommend

The frozen macro table does decide this file (it is in
`docs/baseline/config/cond-eval-verified.txt`, `tools/check-cond-eval.py src/xterm.c` ->
`852 conditions, 0 unknown, 0 disagreements [VERIFIED]`), but the configuration it decides is
one that **can never exist**: `HAVE_X_WINDOWS` is undef, and a build with no window system never
compiles `xterm.c` at all.  The three readings, measured with `tools/cinventory.py` (script in
Evidence):

| reading | functions | macros | constants | file-scope objects | record fields | total demanded | C lines live |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| frozen table (what the gate uses today) | 310 | 21 | 48 | 80 | 78 | 537 | 19,809 |
| A: an X11/GTK3 table (the file's own platform) | 389 | 32 | 61 | 99 | 86 | 667 | 26,914 |
| B: blind - every arm of every `#if` | 433 | 35 | 62 | 105 | 86 | 721 | 33,006 |

Route A needs a *new mechanism* (a per-file condition table: `tools/gen-cond-macros.py` and
`tools/check-cond-eval.py` today key the table on one target, and `cond-eval-verified.txt` is
per-file but per-*target*); route B needs nothing new (one line out of
`cond-eval-verified.txt`, i.e. the file stops consulting the table and every arm is demanded).

**Recommendation: neither, for now - keep the frozen reading and carry the arms anyway (P-51),
which is what the header mirror of this same file already does.**  Reasons, in order of weight:

1. **Symmetric with the mirror.**  `femacs/src/include/xterm.rs` is already the shape "whole
   body under `have_x_windows`, checklist from the frozen reading, arms outside it recorded in
   the ledgers".  Two readings for one subsystem would be worse than either.
2. **The checklist is a floor, not the scope.**  The scope is P-42's 1:1 text, and the lane
   carries the arms that the frozen reading leaves out (16 of them in slice 1 alone, listed in
   Decision 4) - so the work happens regardless of which reading is chosen; only the *mandatory*
   set moves.  Route A would in fact *under*-count too: it drops the Lucid/Motif/Xaw/Xt arms
   (59 names), which are also part of the 1:1 text.
3. **Cost.**  Route B's only real price is that `--check` compares arms that can never coexist;
   that is a text comparison, and `macro_duplicates`/cfg make it harmless.  Route A's price is a
   tool change plus a second `check-cond-eval` verification pass under a configuration that
   cannot be built here.

If you want the checklist to *cover* the X11-only arms, the cheapest correct move is route B
(blind) - it needs no tooling and its 721-item checklist is exactly the union of C's arms.  I am
not asking for it this round because it would change a shared file
(`docs/baseline/config/cond-eval-verified.txt`) and the printed "424 files" of every gate run;
say the word and the next round ships it.

### 3. The conditions slice 1 crosses, with C line numbers

| C lines | condition | arm taken here | how the port carries it |
| --- | --- | --- | --- |
| 625-728 | the header preamble, incl. 639-641 `HAVE_X_I18N`, 643-646 `USE_XCB`, 649-683 the X11 extension headers, 687-689 `#ifndef makedev`, 723-726 / 730-735 / 737-739 / 741-743 / 745-748 / 750-779 / 781-783 the toolkit headers | all dead (the file is not compiled) | includes only, in C's order; the headers with no mirror are listed in Decision 6 |
| 766-768 | `#ifndef XtNpickTop` inside `USE_X_TOOLKIT` + `USE_TOOLKIT_SCROLL_BARS` + `#else` of `USE_MOTIF` | dead | `macro_rules! XtNpickTop` under `use_x_toolkit` + `use_toolkit_scroll_bars` + `not (use_motif)` |
| 774-778 | `#ifndef XtNinitialState` inside `USE_X_TOOLKIT` | dead | `macro_rules! XtNinitialState` under `use_x_toolkit` |
| 797-799 | `#if defined USE_XCB && defined USE_CAIRO_XCB` | dead | empty-body macro under `use_xcb` + `use_cairo_xcb` |
| 801-805 | `#if XCB_SHAPE_MAJOR_VERSION > 1 \|\| (... >= 1)` | the three version macros are unknown here, so C's own answer is "false" | empty-body macro under the derived name `have_xcb_shape_input_rects` |
| 807-816 | `#ifdef USE_GTK` - `xg_pending_quit_event` | dead | gated `use_gtk`, carried (P-51) |
| 828-837 | `#ifdef USE_X_TOOLKIT` - `Xt_app_con`, `Xt_default_resources`, `toolkit_scroll_bar_interaction` | dead | gated `use_x_toolkit`, carried |
| 853-856 | `#ifdef USE_GTK` - `xg_default_icon_file` | dead | gated `use_gtk`, carried |
| 858-861 | `#ifdef HAVE_X_I18N` - `emacs_class` | dead | gated `have_x_i18n`, carried; `EMACS_CLASS` is frame.h 1840's `"Emacs"` |
| 863-867 | `#ifdef USE_GTK` - `current_count`/`current_finish`/`current_hold_quit` | dead | gated `use_gtk`, carried |
| 869-881 | `#ifdef HAVE_XINPUT2` + three `#ifndef` request codes | dead | three numeric macros under `have_xinput2`, carried |
| 891 | `static void x_push_selection_request (...)` - a forward declaration | live as text | not an item in Rust (no prototypes); the definition is C 12573, a later slice |
| 900-959 | the three selection-request functions | **live** under the frozen reading (all three are checklist rows) | transcribed; the bitfield read uses the per-unit `input_event_bits!` (PORTING.md P-80) |

`#ifndef` guards (766, 776, 870, 874, 878) test whether *X11's own headers* already define the
name.  The port cannot see those headers, so each arm is carried under its enclosing features and
the guard itself is recorded here rather than modelled.

### 4. What slice 1 carries that the checklist does not demand

Sixteen items are carried under their features although the frozen reading calls them dead
(P-51; the ledger's boxes are that reading's, so these have no box): `XtNpickTop` (C 767),
`XtNinitialState` (777), `USE_CAIRO_XCB_SURFACE` (798), `HAVE_XCB_SHAPE_INPUT_RECTS` (804),
`X_XIGrabDevice`/`X_XIUngrabDevice`/`X_XIAllowEvents` (871/875/879), `xg_pending_quit_event`
(815), `Xt_app_con` (831), `Xt_default_resources` (832), `toolkit_scroll_bar_interaction` (835),
`xg_default_icon_file` (855), `emacs_class` (860), `current_count`/`current_finish`/
`current_hold_quit` (864-866).

### 5. Gaps slice 1 exposes (each is a reading, none is a stub)

* `buffered_input_event.sie` (C `event->sie`, xterm.c 917/920): keyboard.h 228-235 puts that
  member inside `#if defined HAVE_X11 || defined HAVE_PGTK`, and `include/keyboard.rs` carries
  that arm **as a comment only** (its 66-68).  So C 917's `&event->sie` has no field to name
  until the keyboard.h mirror grows the arm.  That mirror is another lane's file; the request is
  in this round's report, and the two E0609s are in Evidence.
* `x_handle_pending_selection_requests` (xterm.c 958): xterm.h 1857 declares it and
  `include/xterm.rs` does not carry the declaration (it appears only inside the
  `DEFER_SELECTIONS` macro at the mirror's 1271).  Request in the report.
* `x_push_selection_request` (xterm.c 917): defined at xterm.c 12573, a later slice; Rust
  resolves items by name, so the call is correct as written and the name simply does not exist
  yet.
* The `x_atom_refs` table (C 987-1130, 122 entries) needs `struct x_display_info`'s 115
  `Xatom_*` members; the mirror carries 61 of them (the rest are inside xterm.h's conditional
  arms).  It opens slice 2, together with the table's `offsetof` use.

### 6. Headers slice 1 has no mirror for

C 637 `<X11/cursorfont.h>`; 644-645 xcb; 650-683 the X11 extension headers (Xfixes, Xdbe,
XInput2, Xrandr, sync, Xinerama, Xcomposite, shape) and 682's xcb/shape; 691 `<sys/ioctl.h>`;
724-725 `<X11/Shell.h>`/`ShellP.h`; 733 `<X11/Xproto.h>`; 738 `../lwlib/xlwmenu.h`;
742 `<cairo-xlib.h>`; 746-747 and 755 the Xm headers; 759-764 the Xaw/Xaw3d headers;
785 `bitmaps/gray.xbm`; 788 `<X11/XKBlib.h>`.  The X11 ones are the X11 target's own headers -
they are not this tree's to mirror - and `<sys/ioctl.h>` is a system header with no gnulib
mirror.  Every header that *does* have a mirror is included at C's position.

### 7. Feature names slice 1 needs that `xtask`'s `[features]` table does not declare yet

`use_toolkit_scroll_bars` (C 753), `use_motif` (C 754/745), `use_xcb` (C 643/797),
`use_cairo_xcb` (C 797), `have_xcb_shape_input_rects` (C 804).  The rest of the names slice 1
uses (`have_x_windows`, `use_x_toolkit`, `use_gtk`, `have_xinput2`, `have_x_i18n`) are already
in the table from the xterm.h mirror's round.  Requested in this round's report.

### 8. Slice 2 (C 1131-1762): what it landed, and the three shapes worth recording

Slice 2 is the drag-and-drop state block (C 1214-1548), `struct x_client_list_window`
(1478-1533), the Motif message layouts (1550-1685) and the protocol's bit packers
(1687-1761): **66 file-scope objects + 13 macros**, all ticked, plus 3 enums, 1 typedef'd
enum, 9 records and 2 XEmbed enums carried.  The conditions it crosses - with C line numbers -
are `HAVE_XSHAPE` (1511-1526, the two shape rectangles inside `x_client_list_window`),
`HAVE_XKB` (1441-1444, `x_dnd_keyboard_state`), `HAVE_XINPUT2` (1469-1476, the two device ids),
`ENABLE_CHECKING` (1561-1585, the `SWAPCARD*` arm) and `WORDS_BIGENDIAN` (1554-1558,
`XM_BYTE_ORDER_CUR_FIRST`); each is carried under the lowercased feature name, dead arms
included (P-51), so nothing in the range is dropped.

Three decisions this slice had to make, because C cannot be transcribed literally:

* **X11 names.**  Slice 1 followed P-72 rule 3 for the three Xlib *pointer* typedefs
  (`Display`, `XErrorEvent`, `XtAppContext` -> opaque `*mut c_void` + a `/// C:` line).  Slice 2
  keeps every other X11 name as C spells it - `Window`, `Atom`, `XRectangle`, `XEvent`,
  `XWindowAttributes` - which is what this file's own header mirror does with `*mut XEvent` and
  `XRectangle` (its 457, 605), and what makes the by-value objects (`x_dnd_mouse_rect`,
  `x_dnd_pending_send_position`, `x_dnd_old_window_attrs`) expressible at all: an aggregate by
  value cannot be an opaque pointer, and its storage is C's zeroed static storage, written here
  as `std::mem::zeroed ()` (the `buffer.rs` 166 precedent).  Cost, measured: with
  `have_x_windows` on, the unit's own text has **15** unresolved X11 type names (8 `Window`,
  4 `Atom`, and one each of `XWindowAttributes`/`XRectangle`/`XEvent`) - the same class of
  error the mirror already reports 142 times.  If you would rather have one spelling for the
  whole lane, the alternative is to *name* the X11 types inside the arm (`*mut Display`,
  `Window`, ...) and wait for an X11 header mirror to define them; slice 1's three `*mut c_void`
  lines would then be the ones to change.  Say which you want and the next slice follows it.
* **`XM_BYTE_ORDER_CUR_FIRST` (C 1554-1558).**  With `WORDS_BIGENDIAN` undef, C's third enum
  member has the *same value* as `XM_BYTE_ORDER_LSB_FIRST`, and Rust refuses two variants with
  one discriminant (`E0081`).  The port therefore keeps the two real members in the enum and
  writes C's two spellings of the current order as cfg'd `const`s aliasing whichever member C
  would have made them equal to; the `#else` arm is carried under `words_bigendian`.
* **`SWAPCARD32`/`SWAPCARD16` (C 1561-1585).**  `tools/extract-abi.py`'s `macro_body_of` answers
  for a macro name with the **first** `macro_rules!` in the file, so the frozen arm (`#else`,
  gnulib's `bswap_32`/`bswap_16`) is written **first** and the `ENABLE_CHECKING` arm after it -
  with a comment at the definition saying why, so a later reader does not "restore" C's order and
  break the token check.  The checking arm's bitfield trick is written as a `u32`/`u16` plus a
  `c_char` pointer swap (Rust has no bitfields); it is dead here and carried for the flip.
* **`XM_DRAG_PROTOCOL_VERSION` (C 1759-1761)** is an object-like integer macro, which the
  extractor reads as a *constant* and demands as one: written as `pub const ... : c_int = 0`.

### 9. Deferred by slice 2, and what it needs

**`x_atom_refs` (C 987-1130, 122 entries) is not landed** (taken by slice 108, Decision 121): its `ATOM_REFS_INIT` rows name
**115** different `Xatom_*` members of `struct x_display_info`, and `femacs/src/include/xterm.rs`
carries **61** of them.  The 58 that are missing are xterm.h's own **unconditional** text at
**xterm.h 772-817** (measured: none of them is inside an `#if` other than the include guard), so
that mirror is simply a WIP slice of its header.  Writing the table now would put 58 names in
the file that nothing defines.  It opens the next slice as soon as the mirror has them (or you
rule that the table is written 1:1 and the names wait), and the request is in this round's
report.  **Closed by slice 108 (Decision 121)**: measured then, the mirror already carried all 123
`Xatom_*` members, so the table landed with the two `def_x_atom_refs!` instantiations its own
`#ifdef HAVE_XKB` needs.

### 10. Feature names slice 2 adds to the request list

`enable_checking` (C 1561), `have_xshape` (C 1511), `have_xkb` (C 1441).  The previous round's
five (`use_toolkit_scroll_bars`, `use_motif`, `use_xcb`, `use_cairo_xcb`,
`have_xcb_shape_input_rects`) are still outstanding too.

### 11. Slice 3 (C 1763-2173): the Motif side-effect map, the targets table and the drag window

Thirteen static functions - `xm_side_effect_from_action`, `xm_operations_from_actions`,
`xm_read_targets_table_header`, `xm_read_targets_table_rec`, `xm_find_targets_table_idx`,
`x_atoms_compare`, `xm_write_targets_table`, `xm_write_drag_initiator_info`,
`xm_drag_window_error_handler`, `xm_drag_window_io_error_handler`, `x_special_window_exists_p`,
`xm_get_drag_window_1`, `xm_get_drag_window` - all ticked.  What the round had to decide, and
what the two measurements (the gate and the X11 probe) caught:

* **`xm_byte_order` was modelled wrongly in slice 2 and is corrected here.**  C 1808 *stores a
  wire byte* into the enum before comparing it (`header_return->byte_order = *byteorder_return =
  *(bytes++)`), so it is a byte holder, not a closed set; it is now `pub type xm_byte_order =
  c_int` plus the three constants (the `dispextern.rs` `glyph_row_area` shape), which also
  dissolves the `E0081` problem slice 2 recorded (two members with one discriminant).
* **`alloca` is the port's stack pool** (P-05): `USE_SAFE_ALLOCA!`/`SAFE_ALLOCA!`/`SAFE_FREE!`
  at C 1867, 1899, 1943 (`bignum.rs` 428 is the shape).
* **Three libc entry points are declared in the unit** because C reaches them through its own
  headers and no mirror carries them - `qsort` (C 2202; `buffer.rs` 132 declares it the same
  way), and `sigsetjmp`/`siglongjmp` (C 2061/1974, whose whole point is that they save the
  signal mask, which C's own `sys_setjmp` spelling does not).  **Measured, not assumed**: a
  program calling all three links, and `nm -u` shows `_qsort`, `_sigsetjmp`, `_siglongjmp` -
  they are functions, not macros, so this is not one of the invented externs of status 3.251.
* **Xlib's own names stay bare** (`XChangeProperty`, `XGetWindowProperty`, `XFree`,
  `XSetErrorHandler`, `XSetIOErrorHandler`, `XSetCloseDownMode`, `XSync`, `XSelectInput`,
  `XCreateWindow`, `XKillClient`, `XGrabServer`, `XUngrabServer`, `XSendEvent`, `XInternAtom`,
  `None`, `Success`, `False`, `True`, `XA_WINDOW`, `PropModeReplace`, `PropModeAppend`,
  `CopyFromParent`, `InputOnly`, `RetainPermanent`, `DestroyAll`, `CWOverrideRedirect`,
  `StructureNotifyMask`), the way this file's own header mirror keeps `KeyPressMask`.  One of
  them is a hazard worth your ruling: **`None` is Rust's prelude `Option::None`**, so the arm
  cannot be a straight flip; the choice is either an Xlib mirror that defines these names, or
  unit-local constants (`pub const None: Window = 0;`).  Same question for `Success`.
* **Six `macro-body-notes.tsv` rows are needed** (requested in this round's report): the six
  functions above whose C signature or body spells `uint8_t`/`uint16_t`/`uint32_t` and/or
  `alloca`, which the port writes `u8`/`u16`/`u32` and the stack pool.  The tree already has
  this shape (`conv_sockaddr_to_lisp`'s `uint16_t` row, `Freplace_region_contents`'s `alloca`
  row).
* **`xm_drag_window_io_error_handler`** is C's `static _Noreturn int`, so the port writes
  `-> !` and adds the `unreachable! ()` Rust's divergence needs (`lread.rs` 720's shape);
  `xm_drag_window_error_handler` keeps C's `int` return.
* Still deferred: the `x_atom_refs` table (C 987-1130) - unchanged from Decision 9.


### 12. Slice 4 (C 2175-2612), and the X11-macro ruling carried out

Six more functions - `xm_setup_dnd_targets` (C's `goto retry_drag_window`, 2192/2224, becomes the
labelled `loop`; C 2316-2319's four `ckd_*` calls keep their short circuit as the first eager
plus block expressions after `||`, the `ccl.rs` 2913 shape), `xm_get_drag_atom_1`,
`xm_get_drag_atom`, `xm_setup_drag_info`, `xm_send_drop_message` and
`xm_send_top_level_enter_message` - all ticked.  Three points:

* **The ruling on X11's macros is implemented in the mirror.**  `femacs/src/include/xterm.rs`
  now has a `pub mod x11_constants` (gated on `have_x_windows`) with the fourteen macros this lane
  names - `None`, `Success`, `CopyFromParent`, `NoEventMask`, `StructureNotifyMask`,
  `ClientMessage`, `InputOnly`, `CWOverrideRedirect`, `DestroyAll`, `RetainPermanent`,
  `PropModeReplace`, `PropModeAppend`, `XA_ATOM`, `XA_WINDOW` - each with the header line its
  value is quoted from.  **The values are measured, not remembered**: the machine has the
  `xorgproto` headers at `/opt/homebrew/include`, so `clang -E -dM` over `X11/X.h`,
  `X11/Xatom.h` and `X11/Xproto.h` gives `None 0L` (X.h 115), `Success 0` (350), `XA_WINDOW
  ((Atom) 33)` (Xatom.h 41), `StructureNotifyMask (1L<<17)` (168), `PropModeAppend 2` (477) and
  the rest (the ledger's Evidence has the commands).  The reference tree defines none of them
  (measured: `grep -rn '# *define *<name>' emacs/` is empty for every one), so the headers are
  the only source there is.  They sit in a **module** rather than at the mirror's top level, and
  each function imports the few it needs (`use crate::x11_constants::{None, Success};` inside the
  body), because a top-level `None` would shadow Rust's prelude `Option::None` for the whole
  unit.  The mirror's own gate is still OK (`--check emacs/src/xterm.h ...`).
* **C's `False`/`True` are Rust's `false`/`true`** in the ported statements: the token check
  folds case (that is what the equivalence set is for), and `False` is not a Rust literal.  The
  other X11 macros keep C's spelling because Rust has none for them.
* **Nine bodies still carry a `does not mention` row** (the `- ` list's 305-313): `uint8_t`/
  `uint16_t`/`uint32_t` (C's fixed-width spellings; the port writes `u8`/`u16`/`u32`), `alloca`
  (the stack pool, P-05), and for `xm_setup_dnd_targets` also `goto` and `uninit` (the labelled
  loop, and `UNINIT` = conf_post.h 369's zero marker, which the port writes as the explicit
  `std::ptr::null_mut ()` initialiser).  Three `macro-body-notes.tsv` rows cover all nine; they
  are requested in this round's report.  The boxes are ticked because the text is written and
  compiles; the rows are the gate's record of the substitutions.


### 13. The mirror's x11-arm coverage gaps: filled, and the口径 of the gating written down

`spikes/mirror-struct-coverage.py --arm x11` (the integrator's new tool, 3.282/3.283) reported
`98 gap(s) over 4 struct(s)` in the x11 arm, **80 of them `struct x_display_info` and 2
`struct x_output` - both this lane's**.  They are filled now and the reading is **0** for the
whole tree (`282 mirrored struct(s): ... 0 gap(s)`; the other 16 were dispextern's and landed
with that lane's work).

What went into `femacs/src/include/xterm.rs` (+131 lines):

* **85 `x_display_info` members** in C's order: 80 the x11 arm reads (C 734-990 - `wm_type`,
  `x_dnd_atoms`/`_size`/`_length`, `motif_drag_atom`/`_name`/`_time`/`_owner`/`motif_drag_window`,
  the EWMH/XSettings/SM/XDND/XDS `Xatom_*` families, `net_supported_atoms`/
  `nr_net_supported_atoms`/`net_supported_window`, `modmap`, `screen_width`/`_height`/`_mm_*`,
  `failable_requests`/`next_failable_request`, `pending_dnd_time`, `quit_keysym`/
  `quit_keysym_time`) plus the five `#ifdef HAVE_XKB` virtual-modifier atoms (gated
  `have_xkb`; the x11 arm does not define that macro, so the tool does not count them - they are
  here because `xterm.c`'s `ATOM_REFS_INIT` table reaches them).  `motif_drag_atom_name`'s
  length is the arm's own reading of C's constant expression: **33**.
* **2 `x_output` members** (`black_relief`, `white_relief`) and the record they need: C defines
  `struct relief { GC gc; unsigned long pixel; }` *inside* `struct x_output` (C 1308-1313), and
  Rust has no nested struct definitions, so the port hoists it next to its user.
* **`X_WMTYPE_UNKNOWN`/`_A`/`_B`**, the members of C 740-745's anonymous enum (the port keeps
  `wm_type: c_int` and writes the constants, PORTING.md 1.3).

**The口径 the integrator asked to have written down: this gating is the port's, not C's.**
In C, `struct x_display_info` and `struct x_output` are **ungated** - they sit inside
`XTERM_H` and are compiled whenever the header is read, whatever the configuration.  The port
carries both under `#[cfg (feature = "have_x_windows")]`, and the reason is type availability,
not C's own condition: their members name Xlib types (`Display`, `Screen`, `XVisualInfo`,
`Atom`, `Window`, `GC`, `Pixmap`, `KeySym`, `XModifierKeymap`, ...) that no mirror in this tree
defines, so a frozen-configuration layout assertion for them cannot be compiled.  The same
sentence is now in the mirror's own ledger (`docs/ports/include-xterm.md`), and it is the reason
`mirror-struct-coverage.py --arm frozen` reports these records as `unavailable` rather than
complete: the port narrowed the gate, and the narrowing is what the gated-arm reading measures.

The reading, before and after (both `--arm x11`):

```
282 mirrored struct(s): 846 arm(s) attempted, 239 unreadable, 114 gap(s)     # before
## x11: 98 gap(s) over 4 struct(s)
   struct x_display_info            80 missing e.g. ['wm_type', 'x_dnd_atoms', ...]
   struct x_output                   2 missing e.g. ['black_relief', 'white_relief']
282 mirrored struct(s): 846 arm(s) attempted, 241 unreadable, 0 gap(s)       # after
```

The X11-arm probe moved with it: the unit's own `no field` errors went **20 → 2** (the two that
remain are `buffered_input_event.sie`, Decision 5), while the *mirror* side's errors grew
(153 → 220) because the members just carried are live under the arm and name Xlib types - the
expected direction: the mirror now holds more of the gated surface, and that surface cannot
compile until an X11 header mirror exists.

One process note, because it is the kind of thing this lane has to watch: the hoisted `relief`
record was first inserted between `struct x_output`'s attributes and the struct's own `pub struct`
line, which un-gated `x_output` and made `layout-check` fail with `cannot find type Pixmap` in
`include/xterm.rs`.  The failure was shown to be *mine* rather than pre-existing by restoring
`HEAD`'s mirror (layout-check passed), and the fix was to move the hoisted record below the
struct's closing brace.  `layout-check passed` again.


### 14. Slice 5 (C 2614-3012): the Motif senders and readers, and the toplevel teardown

Nine functions - `xm_send_drag_motion_message`, `xm_send_top_level_leave_message`,
`xm_read_drop_start_reply`, `xm_read_drop_start_message`, `xm_read_drag_receiver_info`,
`xm_read_drag_motion_message`, `xm_read_drag_motion_reply`, `x_dnd_send_xm_leave_for_drop` and
`x_dnd_free_toplevels` - all ticked.  C's `x_dnd_compute_toplevels` (C 3013, with its
`USE_XCB`/`HAVE_XCB_SHAPE` arms) opens the **next** slice.

Three shapes worth recording:

* **The `XM_DRAG_*` macros are int-typed now, and that is a correction to slice 2's text.**  C's
  macros do their arithmetic in `int` (the arguments promote out of the `uint8_t`/`uint16_t`
  fields), and slice 2's spelling kept the arguments' own widths, so the X11 probe reported
  `no implementation for u8 | i32` and `mismatched types` at C 2661-2667 / 2903-2904 and in the
  four readers' comparisons.  Each macro now casts its operands to `c_int` inside the body
  (`(($op) as c_int | (($site) as c_int) << 4 | ...)`), which is what C's promotions do, and the
  call sites narrow at the store (`... as u16`).  The token sets are unchanged - `c_int` is in
  the gate's skip list.
* **The message readers keep C's own puns**: `data = (const uint8_t *) &msg->xclient.data.b[0]`
  becomes `core::ptr::addr_of! ((*msg).xclient.data.b[0])`, `*(data++)` becomes a read plus
  `data = data.add (1)`, and `*(uint16_t *) (data + 2)` stays a cast-deref
  (`*(data.add (2) as *const u16)`).  `data[0]` keeps its subscript (`*data.offset (0)`) so the
  `0` stays in the compared token set, and `msg.xclient.type` is `r#type` (Decision 12's note).
* **`x_dnd_free_toplevels`**: C's `UNINIT` locals are `std::ptr::null_mut ()` initialisers
  (Decision 12), `record_unwind_protect_ptr (xfree, ...)` uses the mirror's `xfree_wrapper` (the
  choice `lisp.rs` 3033 makes), the `#ifdef HAVE_XSHAPE` frees and `XShapeSelectInput` are gated
  `have_xshape` (P-51), and `x_dpyinfo`/`x_dnd_fix_motif_leave` are this file's own later items,
  so they are called bare.

**The `does not mention` rows are now 17** (the `- ` list's 296-312): sixteen bodies whose C
signature or bodies spell `uint8_t`/`uint16_t`/`uint32_t` (the port writes `u8`/`u16`/`u32`),
`alloca` (the stack pool), `goto`/`uninit` (`xm_setup_dnd_targets`) and `display`
(`x_dnd_free_toplevels`, whose `Display *dpy` is the port's opaque `*mut c_void` + `/// C:` line,
P-72 rule 3).  Four `macro-body-notes.tsv` rows cover them; the request is in this round's report.


### 15. Slice 6 (C 3013-3614): the toplevel list, and the `x11_ext` arm's five members

> **Landing note (3.349).**  This tick, this decision and its evidence landed in `c8a3d8a`
> (3.336, the `xlib.rs` round) as a *whole file*, while the code half - 666 lines of
> `femacs/src/xterm.rs` carrying `x_dnd_compute_toplevels` - stayed in the lane's working
> tree.  HEAD then contradicted itself for two rounds: the ledger quoted readings (the `300`
> below, `build-tu` built, `layout-check passed`) that the committed tree could not produce
> (`git show HEAD:femacs/src/xterm.rs | grep -c x_dnd_compute_toplevels` was **0** while this
> file ticked it).  The 3.349 commit lands the code half, so the readings below are now HEAD's
> own.  Nothing else in this decision changes.

**The function.**  `x_dnd_compute_toplevels` (C 3013-3614, 601 lines) is ticked - the whole
function, both of its configuration arms carried under their features (P-51):

* `#ifndef USE_XCB` / `USE_XCB` run through the entire body: `#[cfg (not (feature = "use_xcb"))]`
  and `#[cfg (feature = "use_xcb")]` blocks at C 3023-3047 (the two local sets), 3093-3155 (the
  six `SAFE_NALLOCA!` calls and the xcb requests), 3207-3289 (the xcb reply reads), 3293-3298,
  3314-3324, 3333-3345, 3350-3352, 3385-3419, 3421-3459, 3507-3517, 3546-3577, 3580-3595 and
  3606-3608 - each tagged with the C line it comes from.
* inside them `HAVE_XSHAPE`, `HAVE_XCB_SHAPE` and `HAVE_XCB_SHAPE_INPUT_RECTS` become
  `have_xshape` / `have_xcb_shape` / `have_xcb_shape_input_rects`.
* C 3461's `#ifdef ShapeInput` asks whether the X11 shape header is new enough to define the
  shape-input selector.  The X11 headers on this machine do (`X11/extensions/shapeconst.h` 48),
  so the block is carried without a gate of its own and the decision is recorded here.
* C's `XWindowAttributes attrs` (the non-xcb arm) and `xcb_get_window_attributes_reply_t attrs`
  (the xcb arm) are the same Rust name under two `#[cfg]`s, as C has it; the xcb *types* stay
  bare (`xcb_get_property_cookie_t`, `xcb_rectangle_iterator_t`, ...), the way this file's own
  mirror keeps `KeyPressMask`.
* One port-side binding: C 3493-3528 splits one condition across the two arms (the bounding
  rectangle versus the window dimensions), and the port names it `dims_ok` so the `if` stays a
  single statement.
* Seven X11 macros this function names - `AnyPropertyType` (X11/X.h 131, `0L`), `XA_CARDINAL`
  (X11/Xatom.h 14, `6`), `IsUnmapped` (X.h 445, `0`), `PropertyChangeMask` (X.h 173, `1L<<22`),
  `ShapeBounding` (shapeconst.h 46, `0`), `ShapeInput` (48, `2`) and `ShapeNotifyMask` (50,
  `1L << 0`) - are **not** in the mirror's `x11_constants` module, so the unit keeps them bare
  (the mirror's own convention for X11 names).  They were added to the module and then taken
  back out, because this round's writing set is `femacs/src/xterm.rs` + this ledger + the
  mirror's five `HAVE_XSYNC` members only; the measured values are in this round's report for
  the next mirror round.

**The five `HAVE_XSYNC` members.**  The gated probe grew an `x11_ext` arm (3.309:
`x11`'s defines plus `HAVE_XSYNC` and `HAVE_X_I18N`), which turned the whole-tree reading from
`0 gap` into `5 gap`, all of them `struct x_display_info`:

```
xsync_supported_p   xsync_major   xsync_minor   server_time_monotonic_p   server_time_offset
```

They are now in the mirror, in C's order and with C's types, gated where C gates them:
`xsync_supported_p`/`xsync_major`/`xsync_minor` (C 911-915) under `have_xsync`, and
`server_time_monotonic_p`/`server_time_offset` (C 974-983) under
`all (have_xsync, not (use_gtk), have_clock_gettime)` - C's own
`#if defined HAVE_XSYNC && !defined USE_GTK && defined HAVE_CLOCK_GETTIME`.  **No frozen-arm
field was touched**, and `layout-check` still passes, which is the assertion that says so.

**The reading's provenance** (what the integrator asked to have written down): the arm's member
lists come from `spikes/gated-layout-probe.py 'struct x_display_info' xterm.h`'s four-arm
output (`frozen`/`x11`/`ns_cocoa`/`x11_ext`; `frozen` and `ns_cocoa` read `unavailable` for this
record).  Measured with that command, the **direct-member counts are `x11` 207 and `x11_ext`
218** - a delta of eleven, not nine: the six `HAVE_X_I18N` members (`xim`, `xim_styles`,
`xim_callback_data`, `preferred_xim_style`, `xim_coding`, `use_xim`; five were already in the
mirror) plus the five `HAVE_XSYNC` ones.  The integrator's 3.309 note says `190` versus `181`;
that is a different count of the same output (the machine-readable screen the tool uses), and
the two are worth reconciling in one place - what both agree on, and what `mirror-struct-
coverage.py` measures, is that only the five `HAVE_XSYNC` names were missing.


### 16. A mirror for Xlib itself: `femacs/src/include/xlib.rs` (the decision, written before the file)

`dispextern.h` **44** is `typedef XRectangle Emacs_Rectangle;`, and `frame.h`'s
`struct MonitorInfo` is two `Emacs_Rectangle`s plus four `int`s, a `char *` and a `bool` - so the
same record is **32 bytes in the x11 arm and 48 in `ns_cocoa`**, and the difference is the
*typedef*, not the record:

```
$ python3 spikes/gated-layout-probe.py 'struct MonitorInfo' frame.h
## frozen       unavailable (incomplete type: this configuration has no window system)
## x11          [sizeof=32, align=8]      (XRectangle: 4 shorts, 8 bytes)
## ns_cocoa     [sizeof=48, align=8]      (the non-X struct: 4 ints)
## x11_ext      [sizeof=32, align=8]
```

The port's `include/dispextern.rs` carries only the non-X struct, so the port would give 48 under
X11 - a divergence recorded in `include/frame.rs` 1200-1208.  The fix is a home for **Xlib's own
declarations**, and this is the decision about it (the file lands below, in the same round):

* **Place**: `femacs/src/include/xlib.rs`.  The `src/include` tree is where the X11 arms' mirrors
  live (`xterm.rs` is next door) and the units that need it are `src/*.c`'s.
* **First line**: `// femacs: mirror of the X11 SDK's <X11/Xlib.h> (system header of another
  target's toolchain, PORTING.md P-26/P-49).` - the shape `tools/check-target-names.py` accepts
  as a declared non-port (`^// femacs: mirror of the .+?'s <`), taken verbatim from
  `femacs/src/include/jni.rs` (the Android NDK's `<jni.h>`), which is this repo's precedent for a
  system header of *another* target's toolchain.
* **What it carries in this round**: `XRectangle` (Xlib.h 426-429: `short x, y; unsigned short
  width, height;` - 8 bytes, align 2, asserted) and the `pub mod x11_constants` block **moved out
  of `include/xterm.rs`**, where it was an ad-hoc home, extended with the seven macros slice 6
  needed and had to leave bare (`AnyPropertyType`, `XA_CARDINAL`, `IsUnmapped`,
  `PropertyChangeMask`, `ShapeBounding`, `ShapeInput`, `ShapeNotifyMask`), each with the header
  line its measured value came from.
* **The split**: `include/dispextern.rs` gains C 29's include and C 44's / 66-70's arms -
  `#[cfg (feature = "have_x_windows")] pub type Emacs_Rectangle = XRectangle;` and the
  `not (…)` arm keeping today's struct - so `MonitorInfo` **reproduces** the x11 arm's 32/8
  instead of recording the divergence.
* What it deliberately does **not** carry yet: `XColor`/`Cursor`/`Pixmap`/`XGCValues` (which
  `dispextern.h` 38-45 also aliases) and the Xlib *functions* the gated arms call.  They are the
  same job's next piece, and the acceptance for this round is the record above.


### 17. The Xlib mirror's first consumer: the Xrm surface `src/xrdb.c` needs

`src/xrdb.c` (513 lines, target `femacs/src/xrdb.rs` 11, **not** a member of
`docs/baseline/frozen-build-sources.txt` - so like `xterm.c` itself, its "live" side is the X11
one) is the next X11 unit on this lane's path, and it names **17 X11 names this tree did not have
at all** (measured by the integrator against the whole tree minus the six verbatim ones, 3.320).
The names come from **two** headers, which is why this mirror now carries both:

```
$ ls -l /opt/homebrew/include/X11/Xlib.h /opt/homebrew/include/X11/Xresource.h
lrwxr-xr-x  1 liu  admin  45 Jul  7 14:55 /opt/homebrew/include/X11/Xlib.h -> ../../Cellar/libx11/1.8.13/include/X11/Xlib.h
lrwxr-xr-x  1 liu  admin  50 Jul  7 14:55 /opt/homebrew/include/X11/Xresource.h -> ../../Cellar/libx11/1.8.13/include/X11/Xresource.h
```

| name | C | mirror item |
|---|---|---|
| `DefaultScreenOfDisplay` | Xlib.h 120 (`#define`) | `macro_rules!` (see the degradation below) |
| `XResourceManagerString` | Xlib.h 1715 | `unsafe extern "C"` |
| `XScreenResourceString` | Xlib.h 1718 | `unsafe extern "C"` |
| `XrmInitialize` | Xlib.h 1483 | `unsafe extern "C"` |
| `XrmName` / `XrmClass` / `XrmRepresentation` | Xresource.h 136 / 142 / 156 | `pub type … = XrmQuark` |
| `XrmValue` | Xresource.h 160-163 | `#[repr(C)] pub struct` + 4 assertions |
| `XrmStringToQuark` | Xresource.h 91 | `unsafe extern "C"` |
| `XrmStringToQuarkList` | Xresource.h 119-122 | `unsafe extern "C"` (the two macros' target) |
| `XrmStringToNameList` / `XrmStringToClassList` | Xresource.h 140 / 146 (`#define`) | `macro_rules!`, C's body kept |
| `XrmGetFileDatabase` / `XrmGetStringDatabase` / `XrmMergeDatabases` / `XrmPutLineResource` / `XrmQGetResource` | Xresource.h 266 / 276 / 285 / 210 / 215-223 | `unsafe extern "C"` |
| `XrmStringType` | **`src/xrdb.c` 372**, not a header (`#define XrmStringType "String"`) | `pub const XrmStringType: &CStr = c"String"` (+ `.as_ptr ()` at the use site) |

The supporting names come with them because the prototypes need them: `XPointer` (Xlib.h 76,
`char *`), `Bool` (Xlib.h **78**, `#define Bool int` - not a typedef, so the port writes the alias
C's body names), `XrmQuark`/`XrmQuarkList` (Xresource.h 84), `XrmString` (87), `XrmNameList` (137),
`XrmClassList` (143), `XrmValuePtr` (163) and `XDefaultScreenOfDisplay` (Xlib.h 1832, the macro's
body here).

**Shapes** (the rule the last round's decision used, applied per name):

* scalars and one-line typedefs are **declared** (`Bool`, `XPointer`, the seven `Xrm*` aliases);
* `XrmValue` is carried **by value** with its layout asserted, because C passes it by value
  (`XrmValue value;`, `src/xrdb.c` **478**) - measured 16/8, `size`@0 `addr`@8, from a clang probe
  against the installed header (Evidence);
* the three records that appear **only** as pointers here - `Display` (Xlib.h 483, incomplete in C
  too), `Screen` (Xlib.h 253-271, an Xlib record nothing in this tree reads) and
  `XrmDatabase` (Xresource.h 175, `struct _XrmHashBucketRec *`) - degrade to `*mut c_void` with
  C's text in the `/// C:` line above the declaration (P-72 rule 3).  Keeping C's spelling for
  them was rejected here on purpose: `include/xterm.rs` already names `Display`/`Screen` bare and
  the x11-arm probe counts 15+ unresolved X11 type names for that reason (Decision 5), and adding
  a *second* home for those two names would be the collision PORTING.md 0.1-3 forbids.

Two call-site spellings follow from C's own kinds (PORTING.md P-37): the Xrm procedures keep
C's `NAME (…)` because they are procedures, and the three macros below are `macro_rules!`, so a
consumer writes `DefaultScreenOfDisplay! (display)` where C writes `DefaultScreenOfDisplay
(display)` - the standing P-37 trade-off.

**The one degradation**: `DefaultScreenOfDisplay`.  C's body (Xlib.h 120) is
`ScreenOfDisplay(dpy,DefaultScreen(dpy))`, and both of those are Xlib macros over `_XPrivDisplay`
- Xlib's own complete inline copy of its private display record (Xlib.h 489-546), which no mirror
here carries and which is ~50 private members wide.  Measured: the C macro expands **inline and
names no symbol at all**, while `XDefaultScreenOfDisplay` (Xlib.h 1832) is a symbol and returns
the same `Screen *`.  So the port keeps C's macro **name** (the C text of every consumer is what
has to be preserved) and takes the public call as the body.  This is the only place in this file
where the Rust body is not C's body.

**One bit of the integrator's ruling could not be followed as stated, and this is the reading.**
`XrmDatabase` was to keep its "single existing spelling"
(`include/xterm.rs:370` / `include/pgtkgui.rs:122` of `pub type XrmDatabase = *mut c_void`).
Measured:

```
$ sed -n '369p' femacs/src/include/xterm.rs
  pub rdb: XrmDatabase,   // C: XrmDatabase rdb;
$ rg -n 'pub type XrmDatabase' femacs/
femacs/src/include/pgtkgui.rs:122:pub type XrmDatabase = *mut c_void;
$ rg -n 'pgtkgui.rs' femacs --glob '*.rs'
femacs/src/include/dispextern.rs:2393:// pgtkgui.rs 214, nsgui.rs 91, androidgui.rs 245): a second `macro_rules! NativeRectangle` in
$ rg -n 'include!\("pgtkgui.rs"\)' femacs | wc -l
       0
```

`include/xterm.rs` **uses** the name (line 369, inside the `have_x_windows` arm), and its only
*declaration* in the tree is `include/pgtkgui.rs` 122 - in a mirror no include graph reaches, so
it is not in scope at any unit's crate root.  Per the ruling this round therefore declares
nothing: the Xrm prototypes spell it `*mut c_void` with C's text in the `/// C:` line.  If the
X11 graph should own the name, its P-72 rule-1 home is **this** file (`<X11/Xresource.h>` is its
header) and the change is one line - the integrator decides; the pgtkgui copy can stay, since
nothing includes that mirror.

**`XrmStringType` is carried here, not left for the unit.**  C defines it in `src/xrdb.c` 372, so
`src/xrdb.rs` will want it; putting it in the mirror gives the name one home and keeps the unit's
`--check xrdb.c xrdb.rs` honest (`tools/extract-abi.py` follows `include!` chains, so the mirror's
`pub const XrmStringType` is what the check matches C 372's constant against).  Its shape is
`&CStr = c"String"`, not the `&str` that `include/lisp.rs` uses for `#define pI "l"`: C's literal
carries a terminating NUL and `src/xrdb.c` 386 hands it to a C function, which is the
`<config.h>`-string shape of `src/emacs.rs` (status 3.64).  Measured on a scratch pair outside the
repo (`--reference`/`--root` pointed at it): the extractor compares the constant's **bytes** and
accepts both spellings, and rejects a wrong value under either.  When `xrdb.rs` lands it must *use*
this, not declare a copy: a second `const` of the same name in the same scope is `E0428`, and
`check-mirror-dupes.py` would see the copy.

**Ledger placement**: this stays in `docs/ports/xterm.md` rather than a new
`docs/ports/include-xlib.md`.  Reasons: `xlib.rs` has no reference file to be named after (the
tree's `docs/ports/include-*.md` ledgers are one per *reference* header, and this mirror's headers
are another toolchain's), its only includers are this lane's X11 mirrors, and Decision 16 above is
already the record of its location and first line - splitting it now would put one file's two
halves in two ledgers.

**Not link-verifiable on this machine**: only the headers are installed (no `libX11` stubs), which
the C probe measures (Evidence).  These are declarations, so they add no reference and do not move
`tools/check-link.py`'s ratchet; the acceptance for them is C's header text, the two layouts, and
the x11-arm probe.

**One unrequested fix, in this file, revertible in one line**: the `x11_constants` module's
twenty-one constants are typed `c_int`/`c_long`/`c_ulong`, and a *nested module* does not see the
including unit's imports - measured: without the `use core::ffi::{c_int, c_long, c_ulong};` line
now at the module's top, all twenty-one are `E0425: cannot find type c_long` in the x11-arm probe,
which buries the arm's real gaps (411 errors) among twenty-one scoping errors (432 before).  That
is why the line is there; PORTING.md 0.1-9 forbids `use crate::…` in a mirror, and `core::ffi` is
the absolute path.


### 18. Slice 7 (C 3616-4466): the target-window discovery, and the two functions C keeps inside its own conditions

Seven checklist functions - `x_dnd_io_error_handler` (C 3616-3624), `x_dnd_get_target_window_1`
(3720-3817), `x_dnd_get_wm_state_and_proto` (3819-3980), `x_dnd_do_unsupported_drop` (3985-4079),
`x_dnd_send_unsupported_drop` (4081-4125), `x_dnd_fill_empty_target` (4127-4137) and
`x_dnd_get_target_window` (4139-4466), 851 lines of C - plus `X_DND_SUPPORTED_VERSION` (C 3626),
the version byte the later `x_dnd_send_enter` writes into the XdndAware property.  The slice
stops before C 4468 (`x_dnd_get_window_proxy`, then `x_dnd_get_window_proto` at 4510) on purpose:
those two are the query helpers the drag *senders* are built on, while everything here is the
search that decides which window to send to.

**Two functions the checklist does not list.**  C 3632-3696 (`x_dnd_get_proxy_proto`, inside
`#ifdef USE_XCB`) and C 3698-3718 (`x_dnd_get_target_window_2`, inside `#ifdef HAVE_XSHAPE`) sit
in branches the frozen reading calls dead, so the inventory has no box for either - the state
`x_dnd_compute_toplevels`'s inner arms are in.  Per P-51 the port carries both functions whole,
under `use_xcb` and `have_xshape`; being unlisted they are compared by no gate, which is why the
reading below moves by the nine rows the seven listed functions and the constant clear, and by
neither of these two.

**The composite arm gets a derived name.**  `x_dnd_get_target_window` wraps its overlay-window
code in C 4225's and C 4377/4400's
`#if defined HAVE_XCOMPOSITE && (XCOMPOSITE_MAJOR > 0 || XCOMPOSITE_MINOR > 2)`.  The first half
is `have_xcomposite`; the second names an X11 header's major/minor pair, which this tree cannot
see, so the port carries it under the derived name **`have_xcomposite_overlay`** (the
`have_xcb_shape_input_rects` shape of Decision 3):
`#[cfg (all (feature = "have_xcomposite", feature = "have_xcomposite_overlay"))]`.  Measured with
the frozen table, every line of both arms is dead (`tools/cinventory.py`'s `dead_branch_map`:
4225-4266 and 4377-4443 all `False`), so nothing in them is demanded by the token check.  The arm
reads three `x_display_info` members - `composite_supported_p`, `composite_major`,
`composite_minor` (xterm.h 921-925) - that **no arm of the mirror carries** (the `x11_ext` arm
defines neither `HAVE_XCOMPOSITE` nor them), so the port names them the way the C text does; the
three members are carried now, under `have_xcomposite` (§19①).  `Xatom_NET_WM_CM_Sn` (C 609,
ungated) is already in the mirror.

**Seven X11 names the module did not carry, carried in §19③ now** (Decision 15's ruling is what
kept them bare through this slice; the values below are this round's measurement, and §19③ records
what still has to happen in the *unit* before they resolve):

| name | header:line | measured value | site |
| --- | --- | --- | --- |
| `NormalState` | Xutil.h 147 | `1` | C 3734 |
| `IsViewable` | X.h 447 | `2` | C 4245, 4422 |
| `XA_PRIMARY` | Xatom.h 9 | `((Atom) 1)` | C 4038 |
| `ButtonPress` | X.h 183 | `4` | C 4062 |
| `ButtonRelease` | X.h 184 | `5` | C 4069 |
| `ButtonPressMask` | X.h 153 | `(1L<<2)` | C 4067 |
| `ButtonReleaseMask` | X.h 154 | `(1L<<3)` | C 4073 |

`<X11/Xutil.h>` has no mirror *file* of its own in this tree, and it does not need one: this
file's five headers are one mirror's business (`src/xrdb.c` 37-41, the "one mirror for the five"
note in `src/xrdb.rs`), so `NormalState` is a module row like the three `<X11/extensions/
shapeconst.h>` ones above it, and §19② records the measurement and the reasoning in full.

**Two declarations move into the body.**  `x_dnd_do_unsupported_drop` calls
`decode_window_system_frame` (declared `frame.h` 1474-1475, defined `frame.c` 124) and
`x_own_selection` (declared `xterm.h` 1947-1948, defined `xselect.c` 280).  This unit's include
graph reaches no mirror that carries either - `include/frame.rs` records the frame prototypes as
text and there is no `xselect.h` mirror at all - so both are transcribed into the function that
needs them, which is what the xterm.h mirror's own note says the recorded prototype block is for
(the `font.rs` 413-418 `decode_window_system_frame` block, the `keyboard.rs`
`haiku_handle_selection_clear` shape).  **Those two inline declarations are the port's, not this
mirror's, and they are recorded as such**: if the owning unit lands (`src/xselect.rs`; the block
`src/frame.rs` will carry `frame.h` 1474-1475 in), the declaration moves back into that mirror and
the inline copy in `src/xterm.rs` 3982-3988 is deleted (P-26).  `x_get_atom_name` (declared
`xterm.h` 1844, *defined* `xterm.c` 29984) needs none: Rust resolves same-unit items by name.

**Five port-side shapes, each recorded rather than silent.**

* C's `for (tem = x_dnd_toplevels; tem; tem = tem->next)` runs its increment on `continue` too;
  the port keeps the increment after a labelled `'body:` block and writes C's `continue` as
  `break 'body` (the `overlay_arrow_in_current_buffer_p` shape, deviation 133).
* A bare `break` inside that labelled block is `E0695`, so the loop itself is labelled and C's two
  `break`s are `break 'toplevels` - found by the x11-arm probe, not by any text gate.
* C 3776-3782 writes `chosen = tem; break;` once, shared by the `#ifdef HAVE_XSHAPE` arm; the port
  names that arm's condition `bounding_only` so the `if` stays one statement (the `dims_ok` shape,
  Decision 15).
* C's comma operator (`free (error), rc = false;`, C 3878-3879 and its siblings) has no Rust
  spelling, so the two statements stand on their own lines (the `magic_db` row's class); and the
  widenings C performs implicitly are written out where the arm would need them
  (`make_fixnum (root_x as EMACS_INT)`, `xm_drag_style::XM_DRAG_STYLE_NONE as c_int`).
* C leaves `x_dnd_get_proxy_proto`'s cookies and several `x_dnd_do_unsupported_drop`/
  `x_dnd_get_target_window` locals uninitialised; the port writes the zero (PORTING.md 1.5).  The
  xcb type names stay bare, which leaves `free (error)`/`free (reply)` needing the pun
  `free (x as *mut c_void)`: that spelling is this slice's, while slice 6 modelled the same
  pointers as `*mut c_void` up front.  Both readings are text-only (nothing with `use_xcb`
  compiles here), and unifying the two spellings is a follow-up, not something this round claims
  to have settled.

**Two new `does not mention` rows, both table requests.**  The gate now reads
`function x_dnd_get_target_window_1 does not mention continue` and
`function x_dnd_send_unsupported_drop does not mention alloca`; the reasons are the `'body:`
shape above and PORTING.md P-05 (C's `alloca` is the port's `sa_stack_alloc`).  They are the same
class as Decision 12's three `alloca` rows and Decision 14's `goto, uninit` row, still open
because `docs/ports/macro-body-notes.tsv` is not in a code lane's writing set - all five are
requested together in this round's report.

**What this slice does not claim.**  The whole body is under `have_x_windows`, so the frozen
`build-tu` prints `built` while compiling none of it (Decision 1); the `use_xcb`, `have_xshape`
and `have_xcomposite` arms have never been compiled by anything, and the x11-arm probe is the
only reading that reaches the rest.  `x_dnd_get_target_window` still calls five helpers that are
later slices of this same file (`x_dnd_get_window_proxy`, `x_dnd_get_window_proto`,
`x_top_window_to_frame`, `x_tooltip_window_to_frame`, `x_get_window_below`), so no X11 link of
this unit is in reach this round.

### 19. The `have_x_windows` header surface slices 6 and 7 named (2026-10-05)

Slice 7's decision recorded three header gaps "for this round's report" - the five `xterm.h`
members, the `<X11/Xutil.h>` question and the seven X11 macros - and slice 6's own note recorded a
fourth: the xcb type names.  This is that round, and its writing set is this lane's two mirrors
(`src/include/xterm.rs`, `src/include/xlib.rs`) plus this ledger: **no unit changed**, so every
reading below that moves, moves in the mirror.

**Measurement first, because two of the four questions turn on it.**  This machine *has* the X11
headers (`xorgproto` + `libX11` under `/opt/homebrew/include/X11`) and the XCB ones
(`/opt/homebrew/include/xcb`), so neither `<X11/Xutil.h>` nor the xcb half is a `text-only`
question:

```
$ ls /opt/homebrew/include/X11/Xutil.h /opt/homebrew/include/xcb/xcb.h
/opt/homebrew/include/X11/Xutil.h
/opt/homebrew/include/xcb/xcb.h
$ grep -n 'define NormalState' /opt/homebrew/include/X11/Xutil.h
147:#define NormalState 1	/* most applications want to start this way */
$ grep -n 'define IsViewable\|define ButtonPress\|define ButtonRelease' /opt/homebrew/include/X11/X.h
153:#define ButtonPressMask			(1L<<2)
154:#define ButtonReleaseMask		(1L<<3)
183:#define ButtonPress		4
184:#define ButtonRelease		5
447:#define IsViewable		2
$ grep -n 'define XA_PRIMARY' /opt/homebrew/include/X11/Xatom.h
9:#define XA_PRIMARY ((Atom) 1)
```

**① The five `struct x_display_info` members.**  Each is a *record member* - an object - not a
macro or a typedef, so each is carried as the member C declares, under C's own condition and at
C's own position in the struct:

| member | C | mirror (gated) |
| --- | --- | --- |
| `xcb_connection_t *xcb_connection` | xterm.h 842, inside `#ifdef USE_XCB` (841-844) | `#[cfg (feature = "use_xcb")] pub xcb_connection: *mut xcb_connection_t` |
| `xcb_visualtype_t *xcb_visual` | xterm.h 843 | `#[cfg (feature = "use_xcb")] pub xcb_visual: *mut xcb_visualtype_t` |
| `bool composite_supported_p` | xterm.h 922, inside `#ifdef HAVE_XCOMPOSITE` (921-925) | `#[cfg (feature = "have_xcomposite")] pub composite_supported_p: bool` |
| `int composite_major` | xterm.h 923 | `#[cfg (feature = "have_xcomposite")] pub composite_major: c_int` |
| `int composite_minor` | xterm.h 924 | `#[cfg (feature = "have_xcomposite")] pub composite_minor: c_int` |

`xterm.h` 841-844 is where the two xcb pointers come from (the round's list wrote 840-843; the
measured guard is 841 and the fields 842/843), and the composite trio is 922-924 inside 921-925.
The xcb pair's pointee types are the new xcb mirror's (§④), which is also where C's own
`<xcb/xproto.h>`/`<xcb/xcb.h>` includes are recorded (`src/xterm.c` 643-646, and again through
`xterm.h` 117-119's `<X11/Xlib-xcb.h>`).  Both additions are inert in the frozen configuration, and
the mirror's pairing is unmoved: `extract-abi --check emacs/src/xterm.h
femacs/src/include/xterm.rs` still reads `OK`.

**The rest of C 823-910 is still uncarried, and the list is now written down** rather than implied:
`xrandr_supported_p`/`_event_base`/`_error_base`/`_major_version`/`_minor_version` (824-828),
`last_monitor_attributes_list` (834), `ext_codes` (838), `supports_xdbe` (847), `supports_xi2`,
`xi2_version`, `xi2_opcode`, `num_devices`, `devices`, `pending_keystroke_time`,
`pending_keystroke_source`, `client_pointer_device` (851-907), `supports_xkb`, `xkb_event_type`,
`xkb_desc` (911-916), `prefer_native_input` (919), `xrender_supported_p`/`xrender_major`/
`xrender_minor` (899-901), `xfixes_supported_p`/`_major`/`_minor`/`_event_base` (905-908),
`xinerama_supported_p` (918) and `xshape_supported_p`/`_major`/`_minor`/`_event_base`/
`_error_base` (928-932).  The xrender trio is the one that is *already* referred to by a carried
span of this mirror: `FRAME_CHECK_XR_VERSION` (`include/xterm.rs`, C 1385-1390) reads all three
under `have_xrender`, so that macro cannot resolve until they land - the same class as the
composite trio was, found while measuring it and left on this list because this round's list has
five names.

**② `<X11/Xutil.h>`: the header is here, and it already has a home - no `Xutil.rs` was created.**
`src/xrdb.c` 37-41 includes `<X11/Xutil.h>` beside `<X11/Xlib.h>`, `<X11/Xatom.h>`, `<X11/X.h>` and
`<X11/Xresource.h>`, and this tree recorded those five as *one* mirror's business when
`include/xlib.rs` landed (`src/xrdb.rs`'s include comment: "one mirror for the five", 3.336).  The
file's `x11_constants` module already carries a *third-party* header's macros the same way - the
three `<X11/extensions/shapeconst.h>` rows since 3.336 - so `NormalState` went in beside them.  A
new `include/Xutil.rs` would have been a second home for a header this tree has already homed
(P-26 / P-72 rule 1) and would have made `src/xrdb.rs`'s own comment false; if a later round wants
the header to have a file of its own, moving one constant is the whole change.  So the mirror
question is answered by *measurement plus the existing home*, not by `text-only`.

**③ The seven constants, each with its own measured basis** (all values verified this round, never
remembered; each is in the `x11_constants` module, whose rows already cite their own headers):

| name | header:line | C text | Rust | why that type |
| --- | --- | --- | --- | --- |
| `NormalState` | Xutil.h 147 | `#define NormalState 1` | `pub const NormalState: c_int = 1` | the `WM_STATE` first long (`xterm.c` 3734's `!= NormalState as c_ulong`) |
| `IsViewable` | X.h 447 | `#define IsViewable 2` | `pub const IsViewable: c_int = 2` | a `map_state` value, like `IsUnmapped` above it |
| `XA_PRIMARY` | Xatom.h 9 | `#define XA_PRIMARY ((Atom) 1)` | `pub const XA_PRIMARY: c_ulong = 1` | the three `XA_*` rows above it (`Atom` is `unsigned long`) |
| `ButtonPress` | X.h 183 | `#define ButtonPress 4` | `pub const ButtonPress: c_int = 4` | an `XEvent` type, like `ClientMessage` above it |
| `ButtonRelease` | X.h 184 | `#define ButtonRelease 5` | `pub const ButtonRelease: c_int = 5` | as above |
| `ButtonPressMask` | X.h 153 | `#define ButtonPressMask (1L<<2)` | `pub const ButtonPressMask: c_long = 1 << 2` | a mask, like `NoEventMask`/`PropertyChangeMask` |
| `ButtonReleaseMask` | X.h 154 | `#define ButtonReleaseMask (1L<<3)` | `pub const ButtonReleaseMask: c_long = 1 << 3` | as above |

**The class those seven leave, named per site.**  They are in a *module*, and the module's own rule
is that "a consumer imports the few names it needs where it needs them" - so the unit's bare uses
stay `E0425` until the unit lane adds `use crate::x11_constants::{…}` per function.  On the flip arm
the eight sites are (measured, `--cfg 'feature="have_x_windows"'`):

```
  include/xterm.rs:103   ButtonPressMask      (the mirror's own STANDARD_EVENT_SET, see below)
  include/xterm.rs:103   ButtonReleaseMask    (same)
  src/xterm.rs:3677      NormalState
  src/xterm.rs:4040      XA_PRIMARY
  src/xterm.rs:4064      ButtonPress
  src/xterm.rs:4069      ButtonPressMask
  src/xterm.rs:4071      ButtonRelease
  src/xterm.rs:4075      ButtonReleaseMask
```

`IsViewable`'s two sites (`src/xterm.rs` 4277 and 4481) are inside the `have_xcomposite_overlay`
arms, which the plain arm does not enable, so they do not appear in that reading.  The mirror's own
two are a *second* instance of the same class and a new finding: `STANDARD_EVENT_SET`
(`include/xterm.rs` 101-104, C 144-155) is a single constant whose body names **eleven** masks bare
- `KeyPressMask`, `ExposureMask`, `ButtonPressMask`, `ButtonReleaseMask`, `PointerMotionMask`,
`StructureNotifyMask`, `FocusChangeMask`, `LeaveWindowMask`, `EnterWindowMask`,
`PropertyChangeMask`, `VisibilityChangeMask` - of which two are now carried by the module and nine
have no home anywhere in the tree.  Fixing them is a decision this round did not take: the gate
compares a macro's body *digest*, so qualifying the body's names would need a
`docs/ports/token-aliases.tsv` row, and declaring them at the mirror's top level would be the
`True`/`False` shape's second home (3.270's pending move).  It is registered here instead.

**④ The xcb surface gets a mirror, in `include/xlib.rs`.**  The two `xterm.h` members need
`xcb_connection_t`/`xcb_visualtype_t` to exist, and both of slice 6's and slice 7's spellings need
the rest of the surface, so `include/xlib.rs` gains a fourth-header section: `<xcb/xcb.h>` and
`<xcb/xproto.h>` under `all (have_x_windows, use_xcb)`, `<xcb/shape.h>` additionally under
`have_xcb_shape` (C's own reachability: 643-646, and 681-683 nested inside it).  It carries

* the scalars `xcb_window_t`, `xcb_atom_t`, `xcb_colormap_t`, `xcb_drawable_t`, `xcb_visualid_t`,
  `xcb_shape_kind_t`, and the opaque `xcb_connection_t` (xcb.h 108's incomplete struct, P-70's
  stand-in);
* the records the arms pass **by value** or read fields of - the five cookies, the five replies,
  `xcb_generic_error_t`, `xcb_visualtype_t`, `xcb_rectangle_t` and `xcb_rectangle_iterator_t` -
  each with C's field list and a `const` assertion for the size/alignment, and the extra offsets of
  the members the arms read;
* the two *named* enums as Rust enums (`xcb_map_state_t`, `xcb_shape_sk_t`; the `xm_drag_style`
  shape, since C's members are `int` and the arm must cast into the `uint8_t` field/argument);
* the fourteen procedures, with C's integer widths (`uint8_t`/`uint16_t`/`uint32_t`/`int16_t`,
  `void *` as `*mut c_void`), and the `r#type` raw identifier for C's `type` member.

Every layout is measured, not remembered: a C probe against `/opt/homebrew/include/xcb` prints the
sizes, alignments and offsets, and the assertions are *live* - flipping one to a wrong number fails
the mirror's own compile with `E0080` (the scratch readings are in this round's Evidence).

**The two spellings, and what can and cannot be settled by a mirror.**  With the mirror in place,
slice 7's spelling is the C-shaped one: its cookie is a `xcb_get_property_cookie_t` passed **by
value** (C 6996-6998), its reply is a `*mut xcb_get_property_reply_t` whose `format`/`type` it
reads, and its `free (error as *mut c_void)` is the port's pun for C's `free (error)`.  Slice 6's
spelling cannot be reconciled with any faithful mirror, and the reasons are all readable from C:

* its cookies/replies/`attrs` are `*mut c_void`/`c_void` locals, so the seven stores at C 3113-3136
  print `expected c_void, found xcb_get_window_attributes_cookie_t` - the call *returns* a 4-byte
  record (measured) and the store slot is untyped;
* C passes `window_attribute_cookies[i]` to the reply call (C 3209-3212, 3221-3224, 3233, 3281),
  while the port passes the array *base* and never indexes: `*(window_attribute_cookies as *mut
  c_void) = …` writes offset 0 on every iteration.  That is not only a type error - `SAFE_NALLOCA!`
  sizes the block from `pointee_size ($buf)`, and `size_of::<c_void>()` is **1** (measured), so the
  allocation is `nitems` bytes while each store writes four;
* its field reads are on `c_void` (`attrs.map_state`, `(*reply).format`, `(*coordinates_reply).dst_x`
  …), which is where the arm's sixteen `no field … on type std::ffi::c_void` errors come from;
* and two *pre-typing* blockers stop the arm before any of that: `{ SAFE_FREE! (); }` (C 3607 is
  `SAFE_FREE ()`, but the port's macro takes the `sa_count` expression) and `sa_count` itself,
  which the port's `#[cfg (feature = "use_xcb")] { … }` block scopes to the cookie block, while C's
  `#ifdef` adds no braces - C 3094's `USE_SAFE_ALLOCA;` is a *function-scope* declaration that C
  3607's `SAFE_FREE ()` closes over.

So the unification is a unit-lane edit (slice 6's xcb arm moved onto slice 7's spelling), not a
mirror choice; the mirror's job was to make C's types exist, and that is what it does.  Slice 7's
own remaining errors are eight sites of one class - the implicit conversions C performs and the
port must write out (Decision 14's rule): `XA_ATOM`/`XA_WINDOW` are `c_ulong` in `x11_constants`
and `xcb_atom_t` is `u32` (four argument sites, three comparisons), and
`xcb_get_property_value`'s `void *` is the port's `*mut c_void` while `xmdata` is `*mut u8`.

**⑤ Two entries retired, with the rule that replaces them.**  `x_own_selection` (declared xterm.h
1947-1948, defined `xselect.c` 280) and `decode_window_system_frame` (declared frame.h 1474-1475,
defined `frame.c` 124) are no longer mirror gaps: the one function that calls them declares them
inline (`src/xterm.rs` 3982-3988's `unsafe extern "C"`, the `font.rs` 413-418 shape), which is a
*unit* file this round may not write and which is already in the tree.  They come off this ledger's
"requested" list; **if the owning unit lands** (`src/xselect.rs`; `src/frame.rs`'s prototype
block), the declaration moves back into that mirror and the inline copy is deleted - P-26, and the
same sentence `include-xterm.md` carries for its own prototype block.

**What this round does not claim.**  Nothing was compiled with the features *on* except the
scratch readings in the Evidence: the frozen `build-tu` prints `built` while compiling none of
`have_x_windows`, and the plain flip arm's 490 errors are unmoved (every name this round adds is
behind `use_xcb`, `have_xcomposite` or the `x11_constants` module's own import rule).  No X11
target build, no link, no runtime evidence; the composite arms, the xcb arms and the `<X11/Xutil.h>`
value are text + measurement + a mirror-level typecheck only.

### 20. Slice 8 (C 4469-6989): the Xdnd senders and the self-drop path, the XInput2 device plumbing, the cairo and XRender drawing helpers, and the frame properties

The ledger's next thirty unchecked boxes, C 4469-6989, 2521 lines of C: the seven Xdnd message
senders, the five DnD bookkeeping functions, the cleanup hook, the three self-drop recorders
(`x_dnd_note_self_position`/`_wheel`/`_drop`), `x_flush`, `x_toolkit_position`,
`x_update_opaque_region`, `x_set_clip_rectangles`/`x_reset_clip_rectangles`, the five drawing
primitives (`x_fill_rectangle`, `x_clear_rectangle`, `x_draw_rectangle`, `x_clear_window`,
`x_set_frame_alpha`), `x_dpyinfo`/`x_display_info_for_display`/`x_find_topmost_parent`, and the
**forty-five** functions this file defines inside conditions the frozen table leaves false, which C
carries and the port carries too (P-51): `x_dnd_cancel_dnd_early` (`HAVE_XINPUT2`),
`x_drop_xrender_surfaces` (`HAVE_XDBE`), `x_xr_ensure_picture`, `x_xr_apply_ext_clip`,
`x_xr_reset_ext_clip` and the two `x_xrender_color_from_gc_*` (`HAVE_XRENDER`),
`x_gc_free_ext_data_private`/`x_gc_get_ext_data`/`x_extension_initialize`
(`USE_CAIRO || HAVE_XRENDER`), `xi_frame_selected_for`, `xi_convert_button_state`,
`xi_convert_event_state`, `xi_convert_event_keyboard_state`, `x_free_xi_devices`,
`xi_populate_scroll_valuator`, `xi_populate_device_from_info`, `x_cache_xi_devices`,
`x_get_scroll_valuator_delta`, `xi_device_from_id`, the five touch-point helpers and
`xi_reset_scroll_valuators_for_device_id` (`HAVE_XINPUT2` and its `_1`/`_2` splits), the cairo
block's seventeen (`USE_CAIRO`) and the three relief/wave helpers (`USE_CAIRO`).  The file-scope
objects C defines here are carried where C defines them: `OPAQUE` (C 6929, a demanded constant),
the three `FRAME_CR_*` macros (C 5289-5297), the two cairo user-data keys (C 6116-6118) and
`max_fringe_bmp`/`fringe_bmp` (C 6233-6234).

**The Xdnd senders and the three self-drop recorders.**  Each message is a `XEvent` whose
`xclient` fields C writes one by one; the port keeps C's statement order and C's own spellings,
including the `?:` C writes as the port's `if`/`else` (`x_dnd_send_enter`'s shift expression) and
the four implicit narrowings C performs into the `xm_drop_start_message` fields the record
declares as `uint32_t` (`timestamp`, `index_atom`, `source_window`: C 1604-1611), which the port
writes out (Decision 14's rule).  `x_dnd_note_self_drop`'s `ie.kind = DRAG_N_DROP_EVENT` is the
`input_event` bitfield the mirror packs into `bits`, so the port's store is the masked write
`xselect.rs` 3610 spells, and its `alloca` is the stack pool (P-05); both are the one
`macro-body-notes.tsv` row this slice appends (`docs/ports/macro-body-notes.tsv` 1269).

**Four arms the frozen reading calls dead, each carried under its feature.**  The `USE_CAIRO` GC
bookkeeping, the XRender extended clip, the cairo drawing helpers and the XInput2 device records
are all text-only work: no arm of this tree turns any of them on, so every reading below that
moves, moves in the *unit*'s own error list and nowhere else.

**Five derived feature names, each for a condition whose macro this tree cannot see** (the
`have_xcb_shape_input_rects` shape of Decision 3; the first four are C 6361-6395's cairo.h
`CAIRO_HAS_*` guards, the last two Xrender.h's `RENDER_MAJOR`/`RENDER_MINOR` pairs at C 6512,
6594 and 6752):

| C condition | the name the port carries it under |
| --- | --- |
| `#ifdef CAIRO_HAS_PDF_SURFACE` (C 6361) | `cairo_has_pdf_surface` |
| `#ifdef CAIRO_HAS_PNG_FUNCTIONS` (C 6369, 6444) | `cairo_has_png_functions` |
| `#ifdef CAIRO_HAS_PS_SURFACE` (C 6376) | `cairo_has_ps_surface` |
| `#ifdef CAIRO_HAS_SVG_SURFACE` (C 6385) | `cairo_has_svg_surface` |
| `RENDER_MAJOR > 0 \|\| RENDER_MINOR >= 2` (C 6512, 6596/6728) | `have_xrender_0_2` |
| `RENDER_MAJOR > 0 \|\| (RENDER_MINOR >= 10)` (C 6594) | `have_xrender_0_10` |

`x_cr_export_frames` is where the cairo four matter: C adds one `else if` per guard and its
trailing `abort ()` is the arm the last *live* guard reaches, so the port keeps one `if`/`else`
chain whose compile-time halves are `cfg`-defined conditions and whose bodies carry their own
`#[cfg]` (a cairo build without a surface still names no function it lacks).  `cairo_has_pdf_surface`
and its three siblings already exist in `femacs/xtask/Cargo.toml` (rows 47-50); the two
`have_xrender_0_*` names do not and are requested in this round's report.

**The flip arm's new errors are all recorded classes.**  The nine functions of the *live* part
that this slice adds call names this graph has no mirror for, which is the class slices 4-7 already
carry: Xlib's own (`XChangeProperty`, `XFree`, `XSendEvent`, `XGetWindowProperty`,
`XSelectInput`, `XDeleteProperty`, `XGetGCValues`, `XSetClipRectangles`, `XSetClipMask`,
`XFillRectangle`, `XSetForeground`, `XDrawRectangle`, `XClearWindow`, `XQueryTree`, `XFlush`,
`XGetWindowAttributes`, `XGetAtomNames`, `Unsorted`, `XGCValues`, `XWindowAttributes`) and this
file's own later slices (`x_ignore_errors_for_next_request`, `x_stop_ignoring_errors`,
`x_had_errors_p`, `x_top_window_to_frame`, `x_translate_coordinates`, `x_catch_errors`,
`x_uncatch_errors_after_check`, `x_atom_to_symbol`, `x_get_atom_name`, `x_clear_area`, all defined
at C 9137+, and `x_dpyinfo`'s two callers now resolve).  The `xclient`/`xconfigure`/`r#type` field
reads on the mirror's *opaque* `XEvent` are slice 2/5's class carried forward (the ledger
`docs/ports/include-xlib.md` owes the union's 33 members), and the one
`no field menu_bar_window on frame` is the mirror's own `have_window_system` gate, which the flip
arm does not set.

The readings are three, and the third is the one worth reading twice: **the unit's error list
grows by 98** (241 -> 339: `E0425` 151 -> 201, `E0609` 84 -> 131, `E0308` 4 -> 5) and **not one of
them is a class this ledger has not already recorded**.  The `E0308` is the interesting one: it is
*not* in this slice's text at all - `x_dpyinfo` now exists, so the type checker reaches slice 6's
`x_dnd_free_toplevels`, whose display local is the `*mut c_void` of slice 6's model
(Decision 19 clause 4), and reports the mismatch C never has (`Display *` there).  Twelve `E0425`s
go the other way: `x_dnd_get_window_proxy`, `x_dnd_get_window_proto` and `x_dpyinfo` were "cannot
find" names for slices 5-7 and are definitions now.

**What this slice does not claim.**  The whole body is under `have_x_windows`, so the frozen
`build-tu` prints `built` while compiling none of it (Decision 1); the `use_cairo`, `have_xrender`,
`have_xdbe`, `have_xinput2*` and `cairo_has_*` arms have never been compiled by anything, and the
plain flip arm is the only reading that reaches the rest.  No X11 target build, no link, no runtime
evidence: `x_set_frame_alpha`, `x_fill_rectangle` and the senders call functions this file defines
in later slices, so no X11 link of this unit is in reach this round either.  The `XEvent` member
writes (`xclient`, `xconfigure`, `r#type`) are C's shape and the mirror's owed members, not a
working XEvent.

### 21. Slice 9 (C 6990-9785): the frame-synchronization block, the update hooks, the fringe and glyph-string drawing, the toolkit colour helpers and the colour cache

The ledger's remainder row's first 2796 lines of C: the `HAVE_XSYNC` frame-synchronization block
(C 6994-7410, ten functions), `x_update_begin`/`x_update_end`/`XTframe_up_to_date` and the two
buffer-flip hooks, the two window-border hooks and `x_clear_under_internal_border`,
`x_after_update_window_line`, `x_premultiply_pixel` and `x_draw_fringe_bitmap`, the five
glyph-string GC/clipping helpers, `x_compute_glyph_string_overhangs`, the four glyph-string
drawers, the whole `#ifdef USE_X_TOOLKIT` colour block, `x_color_cells`/`x_query_colors`/
`x_query_frame_background_color`/`x_hash_string_ignore_case`/`x_parse_color`, the nearest-colour
allocator with its two wrappers, the `HIGHLIGHT_COLOR_DARK_BOOST_LIMIT` constant, the lighter
colour and relief helpers and C 9744's three non-cairo geometry helpers.  Forty-two checklist
boxes move (`--check` **255 -> 212**), and the range table's last row becomes the new remainder.

**The `have_xsync` block is text-only, and its gate is three names.**  `HAVE_XSYNC` and
`HAVE_XSYNCTRIGGERFENCE` are both `undef` in `docs/baseline/config/darwin-arm64.macros.tsv`, so
`x_if_event`, the two monotonic-time readers, `x_sync_note_frame_times`, the frame-drawn test and
wait, `x_sync_update_begin`/`_finish`, the fence trio and `x_sync_handle_frame_drawn` are all dead
in the frozen reading: they have no checklist box (P-51's reading), and no arm of this tree
compiles one.  The condition is C's three-term `#if` (`HAVE_XSYNC`, `!USE_GTK`,
`HAVE_CLOCK_GETTIME`), so the item gate is the three lowercased names joined with `all (...)`;
`have_clock_gettime` is **not** a row of `femacs/xtask/Cargo.toml` yet (the Owed list asks for it),
and the two `#ifdef FRAME_DEBUG` blocks (C 7101-7106, 7271-7273) carry `frame_debug`, which is
also unregistered - the `glyph_debug` shape `docs/ports/dispnew.md` 220-224 records for a knob
`configure` knows nothing about.  C 7271's `fprintf` is the one place this file needs
`PRIuFAST64`, which this unit's include graph does not reach (`lib/include/inttypes.in.rs` is not
on any path here); the arm keeps C's two-part format with the macro that spells it, and the gap is
in the Owed list rather than papered over.

**Five sub-gates the `## Gate mapping` rows now carry** (C's order): `have_xsynctriggerfence`
(C 7249, 7369), `frame_debug` (7101, 7271), `not (feature = "use_cairo")` for C 7818/8535/9744's
three `#ifndef USE_CAIRO` helpers, `any (not (feature = "use_gtk"), feature = "have_xfixes")` for
C 8206's `#if !defined USE_GTK || defined HAVE_XFIXES`, `gcc_lint` for C 8834's pair, `use_lucid`
inside `use_x_toolkit` for C 8900, and `debug_x_colors` for C 9506/9562.

**Three records this unit had to declare, and one it could not.**  `include/dispextern.rs` keeps
`struct face`, `glyph_string` and `draw_fringe_bitmap_params` behind `dispextern_face_records`
(or `have_window_system`), switches no configuration sets, and status 3.220's ruling is that the
units keep their **live typed copies**; this slice is the first in xterm to need them, so
`femacs/src/xterm.rs` now carries (a) `struct face` at the X11 arm's measured layout
(`docs/baseline/face-layout.tsv`: `sizeof` 296, `gc`@168, `stipple`@176, the bit run at 256) with
the `face_bits!`/`glyph_u_cmp_bits!`/`glyphless_bits!` accessors and their masks, (b)
`draw_fringe_bitmap_params` with the mirror's measured offsets (64/52/56) and the two
`get_/set_cursor_p`/`get_/set_overlay_p` readers, and (c) `glyph_string` in C's declaration order
with the six-member `glyph_string_bits!` run - and (c) carries **no** layout assertions, because
the mirror's own note records that record's layout as unmeasured on this target (the ledger
repeats it rather than inventing numbers).  The one record that could not be declared is
`XtConvertArgRec`: this machine has Xlib's headers under `/opt/homebrew/include/X11` but **not**
libXt's, so C 8955-8961's `cvt_string_to_pixel_args` initializer - the only value of that type
this file writes - is carried as C's own text in a comment and the ledger says the arm is
text-only for want of that toolchain (P-86 clause 3's reading).  `cvt_string_to_pixel_value`
(a plain `Pixel`) lands as a `static mut`.

**The five prototypes the unit declares at its use site** (P-55, the `fringe.rs` 846-850 shape):
`get_glyph_string_clip_rects` (dispextern.h 3661), `prepare_face_for_display` (3810),
`x_free_colors` (3791), `parse_color_spec` (3798), `gamma_correct` (3839) and `x_free_dpy_colors`
(xterm.h 1980); none is a row of the mirror this unit includes.  `register_color` (xfaces.c 422,
under that file's own `#ifdef DEBUG_X_COLORS`) is named and **not** declared, because nothing in
this tree's configurations compiles the arm that calls it.  `x_color_cache_bucket_size`,
`x_clear_area`, `x_wm_supports`, `x_catch_errors`, `x_had_errors_p`, `x_ignore_errors_for_next_request`,
`x_stop_ignoring_errors`, `x_uncatch_errors_after_check` and `x_check_font` are this file's own
later slices (C 11344+, 17833+, 26249+) or `#ifdef GLYPH_DEBUG` (C 30433), so the port names them
and declares none - slice 8's rule.

**The three statements the `--check` gate asked for** are `docs/ports/macro-body-notes.tsv` rows
1678 (`x_alloc_nearest_color_1`'s `goto`) and 1679 (`x_draw_fringe_bitmap`'s `cursor_p`, `goto`,
`overlay_p`); the third finding was mine and is fixed rather than noted: C's `eassert (s->gc != 0)`
now keeps C's `0` (`0 as *mut c_void`, the `w32term.rs` 2000 shape) instead of a `.is_null ()`
that lost the literal.

**The flip arm's new errors are all recorded classes.**  The plain `have_x_windows` probe grows by
**192** errors (339 -> 531 on the last measured run; the three fix batches after that run - the
`glyph_string`/`draw_fringe_bitmap_params` copies, the `(*((*s).face as *mut face))` groupings and
the accessor bodies' `unsafe` - were *not* re-measured, the round's probe budget being spent) and
every code is one the ledger already carries: `E0425` for the Xlib/Xt/XInput2/GTK names and for
this file's later slices (326 of the 531's names are in the unit's own text, 3 in the mirrors),
`E0609` for the members the mirrors keep as comments or gate elsewhere (`xclient`/`xbutton`/
`xconfigure`/`r#type` on the opaque `XEvent`, `borders_respect_alpha_background` on `frame`,
`mouse_face_defer` on `Mouse_HLInfo`, `valid` on `color_name_cache_entry`, `sie` on
`buffered_input_event`, `menu_bar_window`), `E0308` for the five `*mut c_void`-vs-`*mut _XDisplay`
sites slices 6-8 already record, and one `E0423` family that is **the mirror's**: C 8649's
`VCENTER_BASELINE_OFFSET` expands `include/dispextern.rs` 2658-2663, whose body names
`FONT_DESCENT`/`FONT_HEIGHT`/`FRAME_FONT`/`FRAME_LINE_HEIGHT` as *calls* - names this unit's graph
has as macros or not at all (10 `E0423`).  This slice is what first reaches that macro, so the
request goes to the Owed list, not to a per-function workaround.  `E0432`, `E0606`, `E0573`,
`E0608`, `E0614`, `E0133` and `E0277` all appeared in this round's intermediate runs and were
fixed in the unit (imports of names `x11_constants` does not carry; `*(x as T)` groupings that
bound the `as` too late; place-vs-pointer conventions in the two accessor families; the
`isize`/`c_int` widenings C performs implicitly).

**What this slice does not claim.**  The whole body is under `have_x_windows`, so the frozen
`build-tu` prints `built` while compiling none of it; the `have_xsync`, `have_xdbe`, `use_cairo`,
`have_xrender`, `use_x_toolkit`, `gcc_lint` and `debug_x_colors` arms have never been compiled by
anything, and the plain flip arm is the only reading that reaches the rest.  No X11 target build,
no link and no runtime evidence: `x_frame_of_widget`, `x_alloc_lighter_color_for_widget` and the
cvt pair are Xt text, the two `#ifdef FRAME_DEBUG` blocks are unbuilt by any arm, and the
`glyph_string`/`face` copies are text whose layouts this target cannot check.  `XtConvertArgRec`
is not even spellable here.

### 22. Slice 10 (C 9786-11915): the relief and box drawing, the image and glyph-string drawing, the frame clearing, the hourglass and the visible bell, and `x_scroll_run`

Thirty C blocks in the file's order (the `## C line ranges` rows 9786-11915), 27 functions and the
`Line Dance` banner.  What the slice decided, beyond "transcribe C":

1. **The two arms that change a *signature* get one definition each.**
   `x_composite_image`'s C 10110-10112 `#ifdef HAVE_XRENDER` adds a `Picture destination`
   parameter, so the unit carries two definitions under `have_xrender` and
   `not (feature = "have_xrender")`; `extract-abi.py`'s candidate loop compares the arm the
   configuration selects.  The three *call* sites that C writes with a cfg'd argument
   (`x_draw_image_foreground`'s two, `x_draw_image_foreground_1`'s one) are carried as
   `#[cfg]`'d statement pairs, the `ftfont.rs` 2552 shape - the port's first choice, since a
   Rust argument list cannot hold a conditional argument.
2. **Two conditions C writes as `if`/`else if` whose arms are configuration-dependent keep C's
   chain by nesting**: `x_clear_area`'s `#if defined HAVE_XRENDER …`/`#else` and
   `x_scroll_run`'s `#ifdef USE_CAIRO_XCB_SURFACE … #else`.  Each computes C's condition once
   (three cfg'd spellings where `#ifndef USE_GTK` and `#ifdef HAVE_XDBE` both feed it) or nests
   the selected arm, so the port has one copy of the shared body and C's control flow.
3. **The `--check` token gate's two switch functions.**  `x_fill_underline`'s
   `switch (style)` and `x_draw_glyph_string`'s `switch (s->first_glyph->type)` are `match`es,
   so `switch`/`case`/`default`/`break` cannot reappear and the `FALLTHROUGH` from the DOTS arm
   is the shared entry stage: two rows are appended to `docs/ports/macro-body-notes.tsv`
   (1705, 1706) with the `w32_fill_underline`/`haiku_fill_underline` rows' reasoning.
   `x_draw_glyph_string`'s third token, `cmp`, is C 11015's `s->first_glyph->u.cmp.automatic`,
   which the port reads through `glyph_u_cmp_bits! (automatic, …)` - a name that does not carry
   the standalone token, so that row lists it too.  The third row this slice produced,
   `x_composite_image does not mention dstx, dsty, srcx, srcy`, is answered in the text: C's
   four parameters are spelled `srcX`/`srcY`/`dstX`/`dstY` and the port keeps C's own names
   (the `include/xwidget.rs` `xvWindow` practice), which is what the token comparison asks for
   and what `non_snake_case` (already in the unit's allow list) permits.
4. **The `face` accessors this unit's copy lacked.**  Slice 9's local `face_bits!` named three
   members; C 11064-11226 reads six more (`underline`, `underline_defaulted_p`,
   `underline_at_descent_line_p`, `underline_pixels_above_descent_line` as a *member*,
   `overline_p`, `overline_color_defaulted_p`, `strike_through_p`,
   `strike_through_color_defaulted_p`), so the unit's macro grows four masked arms and the
   constants dispextern.h 1893-1935 gives them, and its `face` copy renames the slice-9
   placeholder `_pad_underline_pixels` to C's member name (a name-only change; the layout
   assertions are untouched).  The single source for both is `include/dispextern.rs`'s own
   `face_bits!` (owed - clause 3 of the Owed list).
5. **`x_clear_area` is C's one non-`static` definition in the slice**, so the unit exports it
   under C's name (`#[unsafe(no_mangle)] pub unsafe extern "C"`, the `x_dpyinfo` shape); the
   five prefix call sites that read `E0425` before this slice resolve because of it (the
   probe's one *reduction*, below).
6. **Everything this slice names and no mirror declares stays bare** (P-51/P-86's practice for
   this file): the Xlib entry points (`XSetForeground`, `XFillRectangle`, `XGetGCValues`,
   `XDrawLine`, `XSetFillStyle`, `XCopyArea`, `XChangeGC`, `XSetClipMask`,
   `XSetClipRectangles`, `XSetTSOrigin`, `XClearArea`, `XCreatePixmap`, `XMapRaised`,
   `XSetDashes`, `XSetLineAttributes`, `XUnmapWindow`, `ConnectionNumber`, `XBell`, `XFlush`,
   `XSendEvent`, `XCreateWindow`), the X11 constants (`FillSolid`, `FillOpaqueStippled`,
   `GCFunction`, `GCClipMask`, `GCClipXOrigin`, `GCClipYOrigin`, `GCLineStyle`, `GCLineWidth`,
   `GXcopy`, `GXinvert`, `LineSolid`, `LineOnOffDash`, `CapButt`, `JoinRound`, `Convex`,
   `CoordModeOrigin`, `CWCursor`, `CopyFromParent`, `InputOnly`, `Unsorted`), the two records
   (`XPoint`, `XSetWindowAttributes`), the other units' functions (`window_box`,
   `window_box_left`, `window_box_left_offset`, `window_box_right`,
   `cursor_in_mouse_face_p`, `font_for_underline_metrics`, `image_ascent`, `gui_clear_cursor`,
   `popup_activated`, `detect_input_pending`, `x_scroll_bar_clear`) and this file's own later
   slices (`x_ignore_errors_for_next_request`, `x_stop_ignoring_errors`).  The ledger's Owed
   list names them per mirror.  `flush_frame` is the mirror's *macro* (`include/frame.rs`
   1908), so its site is `flush_frame! (f)`; its body's own `redisplay_interface` gap is the
   mirror's (clause 4 of the Owed list).
7. **C's `switch`-free text stays C's**: the `s->first_glyph->type` walks, `s->img->mask`
   null tests (`XEmacs_Pixmap` is the mirror's `*mut c_void` for `pixmap`/`mask`, so the test is
   `.is_null ()`), the `unsigned long` underline arithmetic (C's wraps written with
   `wrapping_sub`, the `haikuterm.rs` 1400-1520 reading of the same code), and the
   `memset (&attrs, 0, sizeof attrs)` shapes (the unit's line 8560 spelling).

### 23. Slice 11 (C 12066-12095): `make_invisible_cursor`, and the two deferrals beside it

Slice 10 ended at C 11915; the next text that needs no bitfield machinery is `make_invisible_cursor`
(C 12067-12095).  Its four Xlib names (`XCreatePixmapCursor`, the `XColor.flags` bits
`DoRed`/`DoGreen`/`DoBlue`, `XGetErrorText` before slice 12's mirror round) stand at the use site
with their headers' lines (P-88 1), and `None` is `include/xlib.rs`'s `x11_constants::None`.
**Deliberately not in the slice, for measured reasons**: `x_toggle_visible_pointer` (C 12097-12139),
`xfixes_toggle_visible_pointer` (C 12053-12062) and `XTtoggle_invisible_pointer` (C 12141-12153) all
store `f->pointer_invisible`, a `frame` `bool_bf` whose shift numbering `include/frame.rs` carries
for the frozen arm only (`src/androidterm.rs` 214-217 records the same bound), and
`x_fixes_pointer_blanking_supported` (C 12040-12045) reads `dpyinfo->xfixes_supported_p` /
`xfixes_major`, which `include/xterm.rs` does not carry.  Their boxes stay unchecked on purpose.

### 24. Slice 12 (C 26217-26347): the error-handling block's first half

The record `x_error_message_stack` (C 26217-26239) with `x_special_error_handler` declared at the
use site (xterm.h 1748-1749), the `x_error_message` pointer and the **exported**
`x_error_message_count` (C 26249/26252, PORTING.md 0.2), the four serial comparators (C's
`unsigned int` wrap written `wrapping_sub`, its widening `as c_ulong`, and the 32-bit truncation of
64-bit serials written `as c_uint` at the call sites - C 26257-26258 says the functions do that),
`x_find_error_handler`, `x_unwind_errors_to` and `x_error_catcher` (whose `buf` is zeroed, Deviation
12).  The block's second half is deferred to slice 13 because it needs the request-serial
expressions (Decision 25).

### 25. Slice 13 (C 26349-26689): four shapes, one mirror round and two move-backs

* **The request-serial macros are degraded, not re-implemented**: C's `NextRequest` /
  `LastKnownRequestProcessed` read `_XPrivDisplay`'s `request` / `last_request_read`, the private
  display record this tree does not carry (the `struct _XDisplay` whose definition lives in
  `<X11/Xlibint.h>`), so `include/xlib.rs` keeps C's macro names with `XNextRequest` /
  `XLastKnownRequestProcessed` (Xlib.h 1806/1809) for their bodies - the same degradation class as
  `DefaultScreenOfDisplay`'s row above, and the same reading obligation: the equivalence is that
  those procedures return the values the C macros compute.
* **The `failable_requests` field is an array**: `struct x_display_info` carries
  `[x_failable_request; N_FAILABLE_REQUESTS]`, so C's pointer decay is written
  `(*dpyinfo).failable_requests.as_mut_ptr ()` at its six uses, and `next_failable_request` stays the
  pointer it is in C.
* **The call sites of a newly defined name are part of the slice**: `x_had_errors_p` returns C's
  `bool`, so the nineteen earlier `… == 0` / `… != 0` sites become `!x_had_errors_p (…)` /
  `x_had_errors_p (…)`, and the five `XSync (…, False)` sites pass `0` because the parameter is
  Xlib's `Bool` (`c_int`).  Both are spellings of C's own truth tests, not behaviour changes.
* **Two P-90 5 move-backs and one mirror row retired**: `src/xfns.rs` 1954-1955's `XNextRequest`
  and `src/gtkutil.rs` 258-261's `XSync` retire into `include/xlib.rs` (they were
  `check-mirror-dupes.py`'s two failures), and `include/xterm.rs`'s `x_error_message_count` row
  retires because the defining unit is its includer (P-90 7; it was slice 12's one `E0428`, which
  that slice's class counts did not include - recorded here rather than smoothed over).

### 26. Slice 14 (C 26704-26992): `x_connection_closed`, and three shapes it fixes rather than follows

* **`error_msg` is the stack pool**, not a heap buffer: C 26766 hands a **file-scope** pointer an
  `alloca` result, which is sound only because C 26991's `error` is a non-local exit; the port keeps
  the pointer file-scope and takes the bytes from `sa_stack_alloc` (P-05), the same spelling slice
  13's `x_check_errors` uses.
* **The two `_Noreturn` XIO handlers take C's `int` return.**  The earlier slices wrote them `-> !`
  (a Rust convenience), which cannot be passed to `XSetIOErrorHandler`'s
  `Emacs_XIOErrorHandler` parameter (`Option<unsafe extern "C" fn (*mut c_void) -> c_int>`, unit
  701).  C declares them `_Noreturn int`; the port keeps the `int` and their diverging tails
  (`emacs_abort ()`, `siglongjmp` + `unreachable! ()`), which is what C 26783 and the motif path's
  C 1803 need.  The reading that named it is this slice's own two `E0308`s.
* **C's two backward `goto`s are labelled `loop`s** (`'delete_more_minibuffer_frames`,
  `'delete_more_frames`) with `continue 'label` for the jump and `break` after the frame walk
  (PORTING.md 2.2, the C 1960 `'retry_drag_window` idiom this file already uses), and
  `dpyinfo->failable_requests` keeps the `.as_mut_ptr ()` decay slice 13's reading established.

### 27. Slice 15 (C 26994-27126): the first-level error handler, and five shapes it fixes

* **C 851/858's two members were in the wrong record.**  `supports_xi2` and `xi2_opcode` belong to
  `struct x_display_info` (C 851/858, the `#ifdef HAVE_XINPUT2` run that follows C 847's
  `supports_xdbe`), and the first draft put their rows in `x_output`, because C's `x_output`
  carries an `XIEventMask *xi_masks` of its own (C 1353) and the two names sit next to it.  The
  reading that told them apart is the probe's own `E0609 no field supports_xi2 on type
  x_display_info`: the compiler named the record the *use site* hands over.  They now stand in
  `x_display_info` at C's own position, after the `use_xcb` pair (C 841-844), and the mirror's
  comment names the members of that run still owed (`supports_xdbe` C 847, `xi2_version` C 855,
  `num_devices` C 861, `devices` C 865, the three `pending_keystroke_*` C 868-880 and
  `client_pointer_device` C 885).
* **C's integer promotions are written out at each comparison.**  `event->error_code` and
  `event->minor_code` are `unsigned char` fields while `dpyinfo->xi2_opcode` is `int`, so C
  compares after the usual arithmetic conversions: the port writes `BadMatch as c_uchar` /
  `X_SetInputFocus as c_uchar` against the byte fields (C 27007-27009) and
  `(*event).request_code as c_int` against the `int` one (C 27059 - the `E0308 expected u8, found
  i32` that reading produced is exactly the widening C hides).
* **C 869-881's three request codes are macros and are invoked, not read** (`X_XIGrabDevice! () as
  c_uchar`, C 27060-27062): the first draft wrote the bare name and the arm answered `E0423 cannot
  find value X_XIGrabDevice`.
* **A `static` C function carries no `#[unsafe(no_mangle)]`.**  Slice 14's `x_connection_closed`
  had one although C 26743 spells the definition `static`; `--check` reads it as an invented
  exported symbol (`fn x_connection_closed carries #[unsafe(no_mangle)] but the reference does not
  define a non-static function of that name`), and the row is removed here (PORTING.md 0.2's rule
  for internal linkage - `static` is neither `pub` nor exported).
* **The four X11 names are use-site measurements** (P-72 rule 4, the `src/xselect.rs` 136-138
  precedent): `BadWindow`/`BadMatch`/`BadName` are X.h 353/358/373 and `X_SetInputFocus` is
  Xproto.h 2063, each read on this machine's xorgproto 2025.1 and each carrying its citation.  The
  owed move is `include/xlib.rs`'s `x11_constants` module - the home `docs/ports/xselect.md`'s Owed
  11 already names for its own copy of this surface - and that move should carry a re-measurement of
  `xselect.rs` 136's `<X11/Xproto.h> 110: #define BadWindow 3L`, which this machine's `Xproto.h`
  does not define (it names no `Bad*` code at all; `X.h` 353 is where `BadWindow` lives).

### 28. Slice 16 (C 27128-27194): `x_new_font`, and three shapes the mirror owed

* **The frame.h accessors C *writes* need the second macro arm.**  `include/frame.rs` carries the
  getters of `FRAME_COLUMN_WIDTH`, `FRAME_LINE_HEIGHT`, `FRAME_MENU_BAR_HEIGHT`,
  `FRAME_TAB_BAR_HEIGHT` and `FRAME_CONFIG_SCROLL_BAR_COLS`; C 27151-27171 assigns to all five, so
  the unit declares the two-arm macros the `src/pgtkterm.rs` 312-336 block already uses for the
  same five names (`FRAME_X (f)` is the getter, `FRAME_X! (f, v)` the write).
* **`font->baseline_offset` lives behind the mirror's own `have_window_system`.**  font.h's
  `int baseline_offset` (font.h 368) sits inside the header's `#if defined HAVE_WINDOW_SYSTEM`
  block, and `include/font.rs` gates that whole run; C's xterm.c reads it with no guard of its own,
  because configure defines `HAVE_WINDOW_SYSTEM` for every window-system build (configure.ac 7517).
  The write therefore carries `#[cfg (feature = "have_window_system")]` on a *block* - a statement
  attribute is what Rust accepts, not one on an expression (`E0658 attributes on expressions are
  experimental` is the reading that said so) - the `src/xfns.rs` 1595-1597 shape, and its comment
  records the guard.  The arms do **not** simply add the feature: with `have_window_system` on,
  `include/dispextern.rs` defines eight names twice (`glyph_string` and the seven
  `GLYPH_STRING_*` constants at 1608/1776-1788), which is an owed mirror defect, measured (that arm
  reads `E0428 x8` and drops every other class).
* **A `#[cfg]` above a comment-only row leaks onto the next item.**  Fifteen rows of
  `include/xterm.rs` (C's bitfields, carried as `// C: bool_bf … : 1;` lines) had their guard on a
  `#[cfg]` line of their own; a comment is not an item, so the attribute attaches to whatever
  follows - which is how `pub xic: XIC` ended up requiring `have_xdbe` as well as `have_x_i18n`
  (the arm-D reading's `E0609 no field xic`, and arm E - the same arm plus `have_xdbe` - reading
  clean is what proved it).  All fifteen now fold their guard into the comment text
  (`// C guard (feature = "have_xdbe"): bool_bf … : 1;`), so no attribute can leak; the four other
  files with the same shape (`femacs/lib/file-has-acl.rs` 1, `include/dispextern.rs` 1,
  `src/pgtkselect.rs` 1, `src/w32xfns.rs` 1) are owed to their own lanes, and the whole-tree count
  is 19 rows in 5 files.
* **The XIM surface stands at the use site.**  `include/xterm.rs`'s `struct x_output` rows
  (`xic`, `xic_style`, `xic_xfs`), `x_display_info`'s (`xim`, `xim_styles`) and its five `STYLE_*`
  constants take `<X11/Xlib.h>`'s types (1090-1167) and masks (1169-1177), and no mirror of this
  unit's graph declared any of them - so the `have_x_i18n` arm carried twenty `E0425` rows plus one
  `E0308` (`STYLE_OVERTHESPOT`'s `XIMStyle` against the two `long`-typed declarations the first
  attempt made).  The five types and the six masks are this unit's use-site declarations (P-72
  rule 4, the `src/xfns.rs` 3835-4140 block's shape), measured against the installed Xlib
  (libx11 1.8.13) and typed `XIMStyle` because that is the type their only users take (the
  `src/xfns.rs` 4112-4135 spelling).  They are **not** in the mirror this round: `src/xfns.rs`
  includes that mirror and carries its own copies, so a mirror row would be a second definition in
  its crate (P-90 ⑦) - the owed move is one commit that puts both copies into `include/xlib.rs`'s
  X11 SDK section.

* **A `#[cfg]` above a comment-only row leaks onto the next item.**  Fifteen rows of
  `include/xterm.rs` (C's bitfields, carried as `// C: bool_bf … : 1;` lines) had their guard on a
  `#[cfg]` line of their own; a comment is not an item, so the attribute attaches to whatever
  follows - which is how `pub xic: XIC` ended up requiring `have_xdbe` as well as `have_x_i18n`
  (the arm-D reading's `E0609 no field xic`, and arm E - the same arm plus `have_xdbe` - reading
  clean is what proved it).  All fifteen now fold their guard into the comment text
  (`// C guard (feature = "have_xdbe"): bool_bf … : 1;`), so no attribute can leak; the four other
  files with the same shape (`femacs/lib/file-has-acl.rs` 1, `include/dispextern.rs` 1,
  `src/pgtkselect.rs` 1, `src/w32xfns.rs` 1) are owed to their own lanes, and the whole-tree count
  is 19 rows in 5 files.
* **The XIM surface belongs to the mirror that names it.**  `include/xterm.rs`'s `struct x_output`
  rows (`xic`, `xic_style`, `xic_xfs`) and `x_display_info`'s (`xim`, `xim_styles`) take
  `<X11/Xlib.h>`'s types, and its five `STYLE_*` constants take Xlib.h 1169-1177's masks - none of
  which any mirror declared, so the `have_x_i18n` arm carried twenty `E0425` rows plus one
  `E0308` (`STYLE_OVERTHESPOT`'s `XIMStyle` against undeclared `long` masks).  The five types and
  the six masks now stand in that mirror, measured against the installed Xlib (libx11 1.8.13), with
  the masks typed `XIMStyle` (the type C's own use site converts them to, the `src/xfns.rs`
  4112-4135 spelling); the owed move is `include/xlib.rs`'s X11 SDK section, together with
  `xfns.rs`'s copies.  The unit's own XIM arm then reads the masks directly, with no cast.


### 29. Slice 17 (C 27195-27284): the XIM block's first two functions, and `XFree`'s prototypes

* **The three nested guards are three features.**  C 27200/27202/27210 test `HAVE_X_I18N`,
  `HAVE_X11R6` and `HAVE_X11R6_XIM`; the first two are `undef` in the frozen table
  (`docs/baseline/config/darwin-arm64.macros.tsv` 1686-1687), so both functions are **dead text**
  there - which is why `--check` never listed them as missing (P-66's evaluator) and why this
  slice's `--check` reading does not move.
* **Declaring `XFree` turned sixteen unresolved rows into sixteen call sites that needed C's own
  conversion.**  The unit had sixteen `XFree (…)` calls whose name nothing declared (sixteen
  `E0425`); declaring Xlib's `int XFree (void *)` at the use site resolved all sixteen and, in the
  same reading, named fifteen of them as `E0308` - C's `void *` parameter accepts any object
  pointer and Rust's does not, so each site now writes `as *mut c_void` (the `src/xterm.rs` 4113 /
  5186 / 13429 sites already spelled it).  That is the whole `-16 E0425` this slice's arm F reads.
* **A doc comment on an `unsafe extern "C"` block is an unused doc comment.**  The first layout put
  the two use-site declarations under the function's `///` block; rustc answered
  `unused doc comment` (a warning, and this file's warning count is part of every reading), so the
  block stands above the comment.
* **`EVENT_INIT` takes the zeroed `input_event`** (Deviation 12, the `src/pgtkterm.rs` 7714
  shape), `PREEDIT_TEXT_EVENT` is written through this unit's `input_event_bits!` (the C 1416-1417
  spelling), and `FRAME_X_OUTPUT!`'s union member is cast at the use site (the C 7428 shape).


### 30. Slice 18 (C 27286-27342): `xim_open_dpy`, and `XFree`'s declaration moved to every X arm

* **`XFree` belongs to every X arm, not to one gate.**  Slice 17 declared Xlib's
  `int XFree (void *)` under its `HAVE_X11R6_XIM` arm because that is where it first met the name;
  the unit's X text frees Xlib memory in sixteen places that are *not* behind that arm, so the
  declaration now stands under `have_x_windows` alone.  That is what moves every arm this slice
  reads: A **563 -> 547**, B **563 -> 547**, C **591 -> 575**, D **564 -> 548**, each `-16 E0425`
  and nothing else, because the sixteen calls were unresolved names.  The sixteen also had to
  spell C's conversion (`as *mut c_void`), which slice 17's reading had already named for fifteen
  of them.
* **Both arms of `HAVE_XIM` are carried.**  The macro is `undef` in the frozen table
  (`darwin-arm64.macros.tsv` 1700), so C's own `xim_open_dpy` body there is the two lines around
  the guard - `dpyinfo->xim = NULL;` - and the port carries the live text as the
  `not (feature = "have_xim")` arm beside the `have_xim` one (P-86 ①'s rule, the
  `src/gtkutil.rs` shape).
* **The XIM surface this function needs is use-site, and the `XN*` names are functions.**  `Status`
  is Xlib.h 79's `#define Status int` (the `src/xmenu.rs` 477 spelling); `XIMProc` and
  `XIMCallback` are Xlib.h 1144-1148 and 1230-1234 (`sizeof 16 align 8`, offsets `client_data 0`
  `callback 8`, the `src/xfns.rs` 3883-3895 measurement); `XNQueryInputStyle`/`XNDestroyCallback`
  are Xlib.h 1180/1187 and are *functions* returning `*mut c_char`, because that is what they are
  passed as (the `src/xfns.rs` 4171/4177 pair of the same macro family); and the five entry points
  (Xlib.h 3836-3861) are declared here since none is in `include/xlib.rs` yet.
* **A variadic call's trailing `NULL` is typed.**  C passes `NULL` as `XGetIMValues`' last
  argument; the port's first attempt wrote `core::ptr::null_mut ()` and rustc answered
  `E0283 type annotations needed` - the spelling is `std::ptr::null_mut::<c_void> ()` (the
  `src/xfns.rs` 4960 site).


### 31. Slice 19 (C 27344-27433): the instantiate callback and `xim_initialize`

* **The callback's address picks the ABI.**  `xim_instantiate_callback` (C 27352) is a `static`
  whose address C 27425 hands to `XRegisterIMInstantiateCallback`, so it is an
  `unsafe extern "C" fn` (PORTING.md 1.4) and the parameter type is Xlib's `XIDProc`
  (Xlib.h 1154-1158), declared here beside it.  The three XIC entry points C 27381-27385 calls
  (`create_frame_xic`, `xic_set_preeditarea`, `xic_set_statusarea`, xfns.c 3090/3251/3302, none
  `static`) are use-site declarations, as is `XRegisterIMInstantiateCallback` itself
  (Xlib.h 3941-3948).
* **C's `(void *) xim_inst` is written as the prototype's own type.**  C casts the record pointer
  to `void *` because the sixth parameter's spelling differs across Xlib versions (C 27416-27419
  says so); in this tree that parameter is `XPointer` (`*mut c_char`, the `include/xlib.rs` row),
  so the port writes `xim_inst as XPointer` - the first attempt's `as *mut c_void` was the arm-H
  reading's one `E0308`.
* **A value only `eassert!` reads is initialised in its declaration.**  `ret` (C 27410) is written
  by C 27421 and read by C 27427's `eassert`; `eassert!` compiles only under `enable_checking`, so
  the assignment shape warns `value assigned to ret is never read` in every arm that compiles it.
  The port initialises it where it is declared (the `src/pdumper.rs` 1027-1029 and
  `src/process.rs` 7811-7813 shape), which keeps C's single call and drops the warning.


### 32. Slice 20 (C 27436-27487): `xim_close_dpy`, and C's stale `#endif` comment

* **The guard C 27489 closes is `HAVE_X_I18N`, not what its comment says.**  C 27489 reads
  `#endif /* not HAVE_X11R6_XIM */`, and no `#ifndef HAVE_X11R6_XIM` is open there: from
  C 27200 on, the only unbalanced guard is `HAVE_X_I18N`, so that is the region's end - which is
  what this slice's own gate (`have_x_i18n`) and slices 17-19 already assumed.  The port records
  C's wording where it stands and the guard it really closes in the function's doc comment.
* **Nothing stands in the `#else` positions.**  C 27486 ends the `HAVE_XIM` arm with no `#else`,
  so `xim_close_dpy`'s body is the `have_xim` arm alone and there is no second arm to carry
  (unlike C 27286-27342's `dpyinfo->xim = NULL;`, P-86 ①'s rule applies where C has an arm).
* **The second `eassert` value takes the same shape as the first.**  C 27447 declares `Bool rc;`
  and C 27459 assigns it; its only reader is C 27465's `eassert`, which compiles only under
  `enable_checking`, so the port initialises it where it is assigned and leaves C's declaration
  line as the comment that records it (the `src/pdumper.rs` 1027-1029 shape, Decision 31's third
  shape).  `XUnregisterIMInstantiateCallback` (Xlib.h 3950-3957) is declared at the use site with
  the same six parameters as its register sibling.


### 34. Slice 22 (C 27583-27689): `x_set_offset`, and three names this file defines later

* **A name this same file defines later is called, not declared.**  C 27612's `x_wm_set_size_hint`
  is defined in this file at C 30159 (xterm.h 1777 declares it), and C 27661/27679's
  `x_sync_with_move`/`x_check_expected_move` are this file's `static`s too; a use-site
  `unsafe extern "C"` block would collide with those definitions when their slices land
  (PORTING.md 1.3's same-unit rule), so the unit writes the calls and their three `E0425` rows are
  the owed record - the reading moves by exactly those three in arm A.
* **The `wm_type` constants are the mirror's.**  The first attempt declared `X_WMTYPE_UNKNOWN/A/B`;
  `include/xterm.rs` 723-727 already carries the enum (its `wm_type` field's comment names it),
  and `check-mirror-dupes` read the three rows as duplicates.  They are gone and the unit writes no
  copy (P-26/P-90 ⑦'s rule, the one slice 21 met on the display measurements).
* **`move_offset_left`/`move_offset_top` are members the mirror records, not stores.**  Like
  `has_been_visible` (Decision 33), the two `int` members C 27637/27638 and C 27677/27678 read are
  not rows of `include/xterm.rs`'s `x_output`, so the port writes C's own member names and the
  four `E0609` rows are the owed record.
* **Both `USE_GTK` arms are carried** (P-86 ①): C 27594's `xg_get_scale` (declared at this use
  site from gtkutil.h; the `src/gtkutil.rs` 629 definition is another crate's) and C 27622/27643's
  `gtk_window_move` (the gtk mirror's) in the `use_gtk` arm, C 27646's `XMoveWindow` in the
  `#else`, so arm B loses the one `E0425` that declaration resolves.


### 35. Slice 23 (C 27691-27830): the WM-check trio, and what declaring `XGetWindowProperty` costs

* **Two exported functions and one static.**  `x_wm_supports_1` (C 27727) and `x_wm_supports`
  (C 27825) are non-`static` and declared in xterm.h, so both are
  `#[unsafe(no_mangle)] pub unsafe extern "C" fn` (PORTING.md 0.2/1.4) - and that definition is
  what resolves the two callers this file already had (`E0425 x_wm_supports` at C 13247/13254's
  sites, the reading's `-2`); `x_get_wm_check_window` (C 27691) is a `static` whose callers are
  C 27759/27766.
* **`x_special_window_exists_p` is this file's `static` and is not ported.**  C 27761/27768 (and
  the older C 1979 site) call it; the port writes the calls without a declaration and the rows
  are the owed record (Decision 34's first shape).
* **`XGetWindowProperty` is a measured, deferred declaration.**  No mirror of this unit's graph
  declares the name, and the unit calls it in seven places (C 1758, C 1853, C 1972, C 2199,
  C 2600, C 27700, C 27794) - none of which had been type-checked, because an unresolved function
  silences its arguments.  Declaring it once at this use site (Xlib.h 2686-2697) was measured this
  slice and **not landed**: it retires **13 `E0425`** rows and surfaces **12 `E0308`** (ten older
  sites pass Rust `false` where C's `False` is the `Bool` `int`, and `AnyPropertyType` is a
  `c_long` where the prototype takes an `Atom`) **plus 8 `E0381`** (output locals the older sites
  declare without an initializer, C's own uninitialised-output shape) and five warnings.  Those
  twenty rows are the *older* slices' text and want their own round with those numbers in hand;
  this slice's own two sites therefore stand unresolved, and the two rows are its `+2`.


### 36. Decision 35's round: `XGetWindowProperty` declared, and the twenty rows it predicted

Decision 35 predicted 13 `E0425` rows retiring and 12 `E0308` + 8 `E0381` rows surfacing; the
round landed them and the measurement was close to the prediction but not equal to it: the
declaration (Xlib.h 2686-2697, at the use site) retires **13 `E0425`** rows, and the older text it
type-checks for the first time needed **12 `E0308`** fixes (ten `XGetWindowProperty` calls passed
Rust `false` where C's `False` is the `Bool` `int`, and two passed `AnyPropertyType`, a `c_long`,
where the prototype takes an `Atom` - written `as Atom`), **9 `E0381`** fixes (the output locals
those calls hand over: C's own uninitialised-output shape, which the port zeroes per Deviation 12)
and one `max_len as c_long` widening (C's `int` argument where the prototype takes a `long`).
Arm A goes **558 / 55 -> 545 / 65**: `-13 E0425`, every other class at its baseline value, and
**+10 warnings** - the price of the Deviation 12 initialisers, which the call overwrites through a
raw pointer before any read, so rustc's `unused_assignments`/`unused_mut` classes fire.  The four
`does not mention false` token rows this creates are closed by
`docs/ports/macro-body-notes.tsv` rows, so `--check` **stays 154**.


### 37. Slice 24 (C 27832-27908): the WM-state sender and its three frame-parameter handlers

* **The `XEvent` member writes are the class the mirror already owes.**  `include/xlib.rs` 1241's
  `XEvent` is a `[u8; 192]` placeholder with no members, so C 27839-27848's nine `msg.xclient.*`
  writes (and the older C 2365-2376 site's) are `E0609` rows: this slice adds nine of the same
  class, recorded, not hidden.
* **`XSendEvent` has no prototype on this machine.**  `<X11/Xlib.h>` here is a reduced copy that
  does not carry it, and no mirror declares the name; the call stands undeclared (one more `E0425`
  row, the fifth site in the file) and the owed item is one declaration whose citation must come
  from a complete Xlib.h.
* **`frame_set_flag`'s write carries `have_window_system`.**  `skip_taskbar` is inside frame.h's
  `#if defined (HAVE_WINDOW_SYSTEM)` block (C 452-479), so the flag write is wrapped in that gate
  (the `src/xfns.rs` 1595-1597 shape) - without it the mirror's `FRAME_SKIP_TASKBAR_UNIT`/`_SHIFT`
  rows are absent and the two names are `E0425`.
* **Both `USE_GTK` arms are carried** (P-86 ①): `xg_set_skip_taskbar` is declared at this use site
  (the `src/gtkutil.rs` 2712 definition is another crate's) and the `#else` runs `set_wm_state`.


### 38. Slice 25 (C 27910-28085): the z-group setter and the `_NET_WM_STATE` reader

* **`FRAME_Z_GROUP`'s writer is the `src/haikufns.rs` copy of the same name.**  frame.h 1262's
  macro writes the two-bit `z_group` field; `include/frame.rs` has the reader (1643), the mask
  (759, ungated) and the shift (643, inside the mirror's `HAVE_WINDOW_SYSTEM` block) and no
  writer, so xterm.rs carries the four-line `macro_rules!` of `src/haikufns.rs` 403-414.  Because
  the shift is gated, the four writes carry that gate too - the same shape C 27906's `skip_taskbar`
  write has one function above (Decision 37) - and so do the four statements that name a
  `FULLSCREEN_*` value, frame.h 47-59's other `HAVE_WINDOW_SYSTEM` enum (the `src/frame.rs`
  5499-5512 shape).  Left ungated those sixteen names are sixteen `E0425` rows.
* **C 28004's lint arm is live in the frozen reading.**  `# if defined GCC_LINT || defined lint`
  is true there (`GCC_LINT` is 1 in `docs/baseline/config/darwin-arm64.macros.tsv`, `lint` undef),
  so its `reply_data = &raw mut reply_data_dummy;` seed is carried as text; the zero initialiser is
  Deviation 12's and the dead store it leaves is Decision 36's warning class.  `--check`'s token
  table asks for exactly that name and for C's local `typedef`'s `reply_data_object`, which Rust
  cannot spell: the two `reply_data` declarations carry each arm's own pointee type and a comment
  inside the body names the `typedef`.
* **Both `USE_XCB` arms are carried** (P-86 ①): `xcb_get_property`/`_reply`/`_value`/
  `_value_length` come from `include/xlib.rs`'s declarations, and the reply stays the opaque
  `xcb_get_property_reply_t *` the mirror keeps.
* **`actual_size` is zeroed at its declaration** (Deviation 12): the `#else` arm hands it to
  `XGetWindowProperty` as an output, and Rust's definite-initialisation analysis rejects a raw
  borrow of a local that has no initialiser (`E0381`, measured this slice).
* **The loop bound carries C's promotion.**  C's `int i < unsigned long actual_size` promotes the
  index; the port writes `actual_size as c_int` (the `src/bidi.rs`/`src/charset.rs` shape), which
  is what the first draft's `E0308` asked for.

### 39. Slice 26 (C 28087-28329): the EWMH fullscreen block

* **`want_fullscreen` gets its own two-armed macro.**  frame.h 460's four-bit `ENUM_BF` field is
  read at C 28104/28113/28284/28289 and written at C 28214; `include/frame.rs` has the mask (757)
  and the shift/unit (607/609) and no accessor, so xterm.rs carries the `src/pgtkterm.rs`
  6463-6470 reader/writer pair (the `src/haikufns.rs` 1312-1318 copy has the reader alone).
* **The window-system gate covers a whole block here, not one statement.**  Every name in
  `do_ewmh_fullscreen`'s `if` body and in `x_check_fullscreen`'s resize block - the field's two
  numbers and the five `FULLSCREEN_*` values of frame.h 47-59 - lives in the mirror's
  `HAVE_WINDOW_SYSTEM` block, so those blocks carry the gate (Decision 38's shape one level up);
  ungated they are thirty-odd `E0425` rows.  Arm A and arm C are therefore error-neutral, and arm B
  gains exactly one more *instance* of a class the baseline already carries five of:
  `include/xterm.rs` 1060's `GDK_WINDOW_XID` gap, reached through `FRAME_OUTER_WINDOW` at this
  slice's second use site (C 28097).
* **Two locals pay the gating price in warnings.**  `lval` (C 28237) and `lval` (C 28272) are
  reassigned only inside the gated blocks, so in the arms that lack `have_window_system` rustc's
  `unused_mut` fires for both: **+2 warnings**, recorded as Decision 36's class.
* **Three use-site declarations and one use-site record.**  `store_frame_param` (frame.c; the
  `src/frame.rs` 3854 definition is another crate's), `XResizeWindow` (Xlib.h 3109; the
  `src/xwidget.rs` 5474-5477 copy) and `change_frame_size` (dispextern.h 3924; the `src/frame.rs`
  414-415 copy) stand at their use sites (P-72 rule 4), and `XPropertyEvent` is transcribed with
  its four layout asserts (Xlib.h 826-841, measured `sizeof` 64) because `include/xlib.rs` 1241's
  `XEvent` is an opaque 192-byte union with no members (the `src/xselect.rs` 167-183 and
  `src/xmenu.rs` 366-382 copies of the same record).
* **C's three switches keep their labels as `match`es.**  PORTING.md 2.2; the three
  `docs/ports/macro-body-notes.tsv` rows this slice adds record the `switch`/`case`/`break`
  (`default` for `x_check_fullscreen`'s `emacs_abort` arm) tokens the port cannot spell.
* **Two calls stand undeclared on purpose.**  `x_wm_set_size_hint` (C 30159) and `x_wait_for_event`
  (C 28431) are this file's own later definitions, so C 28315/28321 call them with no declaration
  (PORTING.md 1.3); both sit inside the gate, so neither costs an `E0425` row in the probe arms and
  both resolve when their slices land.

### 40. Slice 27 (C 28336-28475): the offset check, the move sync and the event wait

* **Two owed rows retire and one arrives.**  `x_check_expected_move` and `x_sync_with_move` are the
  two `static`s C 27679/27661's `x_set_offset` (slice 22) has been calling since that slice landed,
  so defining them retires the file's two `E0425` rows for those names in every arm.  This slice's
  own site for `ConnectionNumber` (C 28439) is one more instance of the owed class C 13157 already
  carries: Xlib's macro reads `Display`'s `fd` member and `include/xlib.rs` 931's `_XDisplay` is
  opaque, so the name has no Rust spelling until that record's members land.
* **`while (count++ < 50)` keeps the post-increment as the body's first statement.**  The one
  occurrence in the whole reference tree; the increment belongs to the condition (it is evaluated
  before the body runs), so the port writes `count += 1;` at the top of the body and leaves C's own
  line in the annotation (PORTING.md 2.2's rewriting class, the P-95 note's shape).
* **`fallback` takes no `mut`.**  C 28388 declares it, C 28419 is its only write and C 28424 reads
  it through a `const` raw borrow (`pselect` takes `const struct timespec *`), so Rust's
  `unused_mut` would fire; `emptyset` keeps its `mut` (`sigemptyset` writes it through a pointer)
  and is zero-initialised per Deviation 12, as is the `fd_set`.
* **`x_wait_for_event` is the definition C 28321 waits on.**  It is `extern` in xterm.h 1899 and
  gtkutil.c 1274/1384 call it, so `#[unsafe(no_mangle)] pub unsafe extern "C" fn` (PORTING.md
  0.2/1.4), and slice 26's in-gate call to it now resolves.
* **Two use-site declarations.**  `x_real_positions` (xterm.h 1906; `src/xfns.rs` 840 defines it)
  and `wait_reading_process_output` (lisp.h 5181; the same prototype `src/pgtkselect.rs` 301-304
  carries, with the P-55 stand-in for the `struct Lisp_Process *` parameter).

### 41. Slice 28 (C 28476-28621): the size-change path and its two background-fill helpers

* **Two new owed rows, both recorded classes.**  C 28489's `x_wm_set_size_hint` is this file's own
  later definition (C 30159), so the call stands undeclared (PORTING.md 1.3) until that slice
  lands.  C 28497/28505's `FRAME_MENUBAR_HEIGHT` is `include/xterm.rs` 1101's macro, whose
  `{ (void) ($f); 0 }` body is C's cast spelling written in Rust and cannot resolve
  (`E0425: cannot find value void`): `docs/ports/xfns.md` 816-828 records the same finding for its
  own two call sites, and this unit keeps C's call because the slice is a transcript.
* **Two declarations this slice first drafted were already in the file.**  `adjust_frame_size`
  (xterm.rs 14813-14816, C 27178's use) and `NorthWestGravity` (xterm.rs 15472-15474, C 27606's
  use) are earlier slices' own use-site declarations, so declaring them a second time was `E0428`
  (measured in the first two probe readings); the calls reuse them.
* **`None` must come from `x11_constants`.**  `XSetWindowBackgroundPixmap`'s third argument is
  Xlib's `None`, and a function without the `use crate::x11_constants::None;` line takes the
  prelude's `Option::None` instead (`E0605: non-primitive cast: Option<_> as u64`, measured).
* **C's `#ifdef USE_X_TOOLKIT` arm keeps its `if`/`else`, and its tail is duplicated.**  The
  `XResizeWindow` every arm reaches is written in both branches (P-86 ①'s practice), because a
  Rust `#[cfg]` cannot cover one half of an `if`/`else` chain.  `XtResizeWidget` and `Dimension`
  stand at that use site with libXt's own prototype text (the block C 27553's `XtVaGetValues`
  already carries; libXt's headers are not on this machine).
* **`x_set_window_size` carries both `USE_GTK` arms** (P-86 ①): the GTK one calls
  `xg_frame_set_char_size` (gtkutil.h 165's prototype, another crate's definition) and the `#else`
  calls `x_set_window_size_1` plus `x_clear_under_internal_border` (this file's own C 7696
  definition, already ported at xterm.rs 8290).

### 42. Slice 29 (C 28623-28743): the size-and-position pair, the mouse mover and the raiser

* **Six new `E0425` rows in arm A, every one a recorded class.**  `x_wm_set_size_hint` (C 28629,
  this file's own later definition at C 30159, PORTING.md 1.3), `XFlush` (C 28739, the Xlib class's
  third instance after C 5221/13248), `XMoveResizeWindow` (C 28651, the class's second after
  C 13416), `XRaiseWindow` (C 28737) and `XWarpPointer` (C 28724) are Xlib entries no mirror of
  this graph carries - the ledger's `## Owed` list keeps them bare (Decision 15's rule) - and the
  sixth is `FRAME_MENUBAR_HEIGHT`'s `void` body, of which this slice's two expansions add the
  second instance.  Arm C adds `XIWarpPointer` (C 28717) and one `E0609` for `x_display_info`'s
  `client_pointer_device`, `include/xterm.rs` 662-666's owed run (C 885).
* **The toolkit arm's two `core` writes need a record.**  C 28639/28640 write `widget->core.x`/`.y`
  through Xt's `struct _WidgetRec`, whose `CorePart` no mirror of this graph has, so the slice
  declares the two use-site records of the `src/xmenu.rs` 671-690 shape (members and order kept,
  offsets UNVERIFIED - libXt is not on this machine) and `Position` (`int`) with them.
* **C's `#ifdef` arms keep their `if`/`else` and the shared tail is written in both.**  Both the
  `USE_X_TOOLKIT` arm (C 28631) and the `HAVE_XINPUT2` arm (C 28707) end in a statement every
  configuration reaches, so `XMoveResizeWindow` and `XWarpPointer` each appear twice (P-86 ①).
* **Two use-site declarations.**  `xg_frame_set_size_and_position` (gtkutil.h 166; the
  `src/gtkutil.rs` 1801 definition) in the `use_gtk` gate, and `XtConfigureWidget` with the two
  toolkit records above.

### 43. Slice 30 (C 28745-28819): the lower/raise hooks and the XEmbed focus request

* **`alloca` becomes the stack pool.**  C 28752's `windows = alloca (2 * sizeof *windows)` is the
  port's `USE_SAFE_ALLOCA!`/`SAFE_ALLOCA!`/`SAFE_FREE!` (P-05, the `xm_find_targets_table_idx`
  shape this file already carries), so `alloca` has no token counterpart and
  `docs/ports/macro-body-notes.tsv` records the row - the second one this unit needs, after
  `x_dnd_send_unsupported_drop`'s.
* **`FOR_EACH_FRAME` keeps C's iteration.**  The mirror's `macro_rules!` (include/frame.rs 800)
  expands to C's own `Vframe_list` walk with the increment after the body, so `continue` means
  what it means in C (the `x_try_restore_frame`/`xim_destroy_callback` shape this file already
  uses).
* **Four new `E0425` rows, every one a recorded class.**  `XRestackWindows` (C 28772),
  `XLowerWindow` (C 28786) and `XFlush` (C 28797, the class's fourth instance) are Xlib entries no
  mirror of this graph carries - kept bare per the ledger's `## Owed` list - and
  `xembed_send_message` is this file's own later definition (C 28998, PORTING.md 1.3).
* **`CurrentTime` stands at its use site.**  X.h 233's `#define CurrentTime 0L` has no mirror row
  in this graph, so the slice declares it with the `src/xselect.rs` 139-141 copy's citation (P-72
  rule 4); `XEMBED_REQUEST_FOCUS` is the `xembed_message` enum at 947-958, spelled
  `xembed_message::XEMBED_REQUEST_FOCUS`.
* **The `HAVE_XWIDGETS` arm carries its gate.**  C 28789-28795's block is `have_xwidgets`'s and no
  probe arm enables it; `lower_frame_xwidget_views` is `include/xwidget.rs`'s declaration.

### 44. Slice 31 (C 28821-28947): the server time and the EWMH activation

* **Fifteen `E0609` rows, all of the `XEvent`-member class.**  `include/xlib.rs` 1241's `XEvent`
  is the opaque 192-byte union whose 33 member records are owed, so C 28827-28829's three reads
  (`type`, `xproperty.window`, `xproperty.atom`), C 28876's `property_dummy.xproperty.time` and
  `x_ewmh_activate_frame`'s eleven `msg.xclient.*` writes keep C's own member path - slice 24's
  nine `msg.xclient.*` writes are the same rows, and the port does not cast around them.
* **Five `E0425` rows.**  `XChangeProperty` (C 28868), `XIfEvent` (C 28873), `XSendEvent`
  (C 28943) and `XRaiseWindow` (C 28937) are Xlib entries no mirror of this graph carries - kept
  bare per `## Owed` - and `x_set_input_focus` is this file's own later definition (C 28963).
* **Three names the first draft left unresolved were its own imports.**  `XA_ATOM`,
  `PropModeReplace` and `ClientMessage` are `x11_constants`', and each function needs its own
  `use` line (measured as `E0425` in the first reading); the two `unused_parens` warnings the same
  reading produced are C's own parenthesised expressions, which Rust does not need.
* **`PropertyNotify` stands at its use site.**  X.h 601's `#define PropertyNotify 28` has no
  mirror row in this graph (the `ConfigureNotify` shape slice 26 already declared), and
  `XSendEvent`'s third argument is Xlib's `False`, which this file's sibling sites spell `false`
  (C 2379/2409/2447 and slice 24's `set_wm_state`), so no `macro-body-notes` row is needed.
* **The `HAVE_XSYNC && !USE_GTK && HAVE_CLOCK_GETTIME` arm carries that three-part gate.**  No
  probe arm enables `have_xsync`, so its `x_sync_current_monotonic_time` call and the
  `X_ULONG_MAX` masking are dead text here.  `server_timestamp_predicate` is an
  `unsafe extern "C" fn` (the `x_atoms_compare` shape) because C hands it to `XIfEvent`.

### 45. Slice 32 (C 28949-29139): the input-focus trio

* **`x_set_input_focus` retires slice 31's owed row.**  C 28934/29126/29130's three calls find
  their definition here, so the `E0425` row slice 31 brought for that name is gone.  The XI2 arm
  keeps `HAVE_XINPUT2`'s gate, and `RevertToParent`/`XSetInputFocus`/`XISetFocus` are the bare
  Xlib/XInput2 class (`XIMasterPointer` sits inside `eassert!`, which only `enable_checking`
  compiles, so arm C does not even name it).
* **C's `goto out` is a labelled block.**  C 29102 jumps to the `out:` label at C 29137, so the
  body between them is `'out: { … break 'out; … }` with `unblock_input ()` after it (PORTING.md
  2.2; `docs/ports/macro-body-notes.tsv` records the lost `goto` token, the
  `x_alloc_nearest_color_1`/`x_dnd_free_toplevels` class).
* **Two first-draft defects the probe caught.**  C's `!FRAME_OVERRIDE_REDIRECT (f)` is truthiness on
  a `c_int`-returning macro, so the port writes `FRAME_OVERRIDE_REDIRECT (f) == 0` (the
  `src/gtkutil.rs` 2289/2409 spelling) - the first draft's `!` was an `E0308`; and `lisp_focus`
  (C 28952) takes no `mut`, because `XSETFRAME!`'s write is its initialisation (the `x_set_sticky`
  shape), which the first draft's `mut` turned into `unused_mut`.
* **One more `E0609` from a recorded member.**  `FRAME_X_EMBEDDED_P` (C 29070) reads
  `x_output.explicit_parent`, a member `include/xterm.rs` records as a comment rather than as
  storage (P-80), so the macro's own text stays and the row is that class; arm C's two
  `client_pointer_device` rows (C 28982/28985) are the same class from `include/xterm.rs`
  662-666's owed run.

### 46. Slice 33 (C 29142-29189): the XEmbed block

* **`XEMBED_VERSION` is owed twice by name, so it is declared twice.**  The inventory carried both
  `missing macro XEMBED_VERSION` and `missing constant XEMBED_VERSION` (C 29148's `#define`, inside
  C 29144's gate), so the port declares C's macro *and* the constant it stands for - the
  `XTNheight`/`XtSetArg` shape this file already uses - and both rows retire.
* **The gate is `any (use_x_toolkit, not (use_gtk))`.**  C 29144's `#if defined USE_X_TOOLKIT ||
  ! defined USE_GTK` covers `xembed_set_info` alone; `xembed_send_message` (C 29165) sits outside
  it and the GTK build compiles it too, which is why arm B's reading carries the sender's rows and
  not the setter's.
* **Nine more `E0609` rows from the `event.xclient.*` writes.**  They are slice 24/31's class
  (`include/xlib.rs` 1241's opaque union).  The two Xlib entry points the block names
  (`XChangeProperty`, `XSendEvent`) already carry rows from earlier slices, so the new sites add
  **exactly one** `E0425` row in arms A and C and none in arm B.
* **`xembed_send_message`'s definition retires slice 30's owed row.**  `xembed_request_focus`
  (C 28817) called it undeclared; C 29165-29189 is that definition.

### 47. Slice 34 (C 29201-29400): `x_make_frame_visible`

* **`gui_set_bitmap_icon` is called bare and its mirror row is owed.**  `frame.h` 1895-1900 defines
  it `INLINE` and no mirror of this unit's graph carries that definition, so the call site (C
  29233) stays a bare name - the `PropertyNotify`/`MapNotify` use-site shape (P-72 rule 4), with
  the export row on the integrator's list.
* **Arm B's three new uncoded rows are the GTK map calls.**  C 29218/29284/29285
  (`gtk_widget_show_all (FRAME_GTK_OUTER_WIDGET (f))` and
  `gtk_window_deiconify (GTK_WINDOW (FRAME_GTK_OUTER_WIDGET (f)))`) are compiled only in the
  `use_gtk` arm, and `FRAME_GTK_OUTER_WIDGET` expands to `include/pgtkterm.rs` 1060's
  `GDK_WINDOW_XID! (gtk_widget_get_window ($w))`, whose macro is not carried anywhere in this tree
  - so they land as rustc's *uncoded* `cannot find macro` rows (21 -> 24), the class arm B already
  had.
* **`x_wm_set_window_state` is defined later in this same file (C 30345).**  C 1191 declares it and
  this port deliberately carries no forward declarations (the file's own §3 note), so the call at
  C 29273 is a bare name: it is one of the `E0425` rows this slice adds, and it retires the round
  that lands C 30345 - together with C 29534/29600, the calls `x_iconify_frame` will bring.
* **The arm-A additions are recorded classes, not new ones.**  `asked_for_visible` (C 29247/29270),
  `has_been_visible` (C 29309) and `explicit_parent` are `x_output`'s bitfields, which
  `include/xterm.rs` 848-857 keeps as *comments* rather than as storage (P-80), so C's member paths
  stand and the rows are the `E0609` class slices 25/31 already carry; `XMapRaised`/`XGetGeometry`/
  `XChangeProperty`/`XDeleteProperty` are the Xlib bare-name class of `## Owed`, and
  `XGetGeometry`'s seven out-parameters are zero-initialised per Deviation 12 (whose price is the
  `unused_assignments`/`unused_mut` warning class, Decision 36).
* **`SET_FRAME_VISIBLE` is the macro, `SET_FRAME_ICONIFIED` the function.**  frame.h 1537/1437: the
  first takes a `bool`, so the port writes the mirror's macro form `SET_FRAME_VISIBLE! (f, true)`
  (C 29225/29296); the second takes an `int`, so C 29226/29297's `false` is written as the `c_int`
  value its parameter needs - `SET_FRAME_ICONIFIED (f, false as c_int)`, the spelling
  `pgtkterm.rs` 8217/8290 already uses for this same function.
* **C's statement order inside the geometry block.**  C 29346's `block_input ()` comes *before* the
  comment and `XGetGeometry` (C 29348-29356), not after the move; the first writing of this slice
  had it after `XMoveWindow`, and reading C 29342-29363 moved it back - the block now reads
  declarations, `block_input`, comment, `XGetGeometry`, `output_flushed`, the conditional move,
  `unblock_input`, in C's order.
* **`MapNotify` and `frame_size_history_plain` stand at their use sites.**  C's `MapNotify` is
  `X.h` 578's `#define MapNotify 19` (no mirror of this graph carries it) and
  `frame_size_history_plain` is frame.h 1514's prototype whose definition is `src/frame.rs`'s, so
  both are P-72 rule-4 use-site entries next to the function - the `PropertyNotify` shape this file
  already has.

### 48. Slice 35 (C 29406-29481): `x_make_frame_invisible` and the visible/invisible dispatch

* **C 29435-29443's shared `else` arm is written once per arm.**  C writes
  `#ifdef USE_GTK … else #else … else #endif` and then *one* `if (! XWithdrawWindow …)` statement
  (C 29445-29450) that both arms fall through to; Rust has no preprocessor `else`-continuation and
  no way to attribute a statement to an `#else`-crossing chain, so the port writes the withdraw arm
  inside each cfg block - the same text twice, the only place in this slice where the port's text
  is not C's text once.  (`x_make_frame_visible`'s three map arms in slice 34 needed no such
  duplication: there each arm was a complete statement of its own.)
* **`PORTING.md` P-98 makes `xembed_set_info`'s parameter `c_int`.**  C's prototype is
  `enum xembed_info flags` (C 29151, ported in slice 33 as `flags: xembed_info`), but C 29441 passes
  **0** - a value `enum xembed_info { XEMBED_MAPPED = 1 }` (C 1139-1142) does not have.  A Rust
  `#[repr(C)] enum` can hold only its declared discriminants (`0 as xembed_info` is `E0605`; a
  `transmute` would build an invalid value, which is P-98's worked example at 3.642), so the
  parameter is `c_int` and the two `XEMBED_MAPPED` call sites spell C's enumerator
  (`xembed_info::XEMBED_MAPPED as c_int`).  Measured: the change adds **no** `--check` row, so the
  ABI list does not compare that parameter's spelling.
* **The arm-A additions are three `E0425` sites and one `E0609`.**  `XWithdrawWindow` is a **new**
  name (one site, Xlib's `## Owed` class); `DefaultScreen` and `x_wm_set_size_hint` each gain one
  site of a name earlier slices already owed (`x_wm_set_size_hint` is C 30159's definition, still
  ahead of this slice).  The `E0609` is `explicit_parent` (`FRAME_PARENT_FRAME`'s read of it),
  the same `x_output` bitfield class (P-80): the row count goes **5 -> 6** although this function
  reads `FRAME_PARENT_FRAME` twice (C 29432/29452), because rustc anchors the macro's unknown field
  at its definition (`include/xterm.rs` 1651), which the baseline already named five times.
* **Arm B's fourth uncoded row is the same GTK macro.**  `FRAME_GTK_OUTER_WIDGET` (C 29436/29437)
  expands to `include/pgtkterm.rs` 1060's `GDK_WINDOW_XID!`, so arm B goes 25 -> 26 of that class -
  it is the arm that compiles C's `USE_GTK` branch.
* **The dispatch function is `static`.**  C 29474's `x_make_frame_visible_invisible` is C's
  `static`, so it is a plain `unsafe fn` (PORTING.md 0.2), and its two calls are the two definitions
  the same slice lands.

### 49. Slice 36 (C 29483-29614): `x_create_font_cursor` and `x_iconify_frame`

* **`IconicState` stands at its use site and the mirror row is owed.**  `Xutil.h` 148's
  `#define IconicState 3` is C 29534/29581/29600's state argument; this unit's `x11_constants`
  module carries the sibling `NormalState` (Xutil.h 147) but not this one (measured), so the port
  writes the constant next to its users - the `PropertyNotify`/`MapNotify` shape this file already
  has (P-72 rule 4) - and the mirror row is the integrator's (`## Owed`).  The alternative (a row
  in the mirror, the way `NormalState`/`IsViewable`/`XA_PRIMARY` went in) is one constant's move.
* **C 29515-29527's GTK block is its own cfg block, not part of the toolkit pair.**  C writes
  `#if defined (USE_GTK) … #endif` *before* `#ifdef USE_X_TOOLKIT … #else … #endif`, so the port
  writes the GTK arm first, then the two toolkit arms; the GTK arm's `return` (C 29525) is what
  keeps a build with both features from falling into the toolkit arm, exactly as in C.
* **`XEvent msg` is zero-initialised (Deviation 12).**  C declares it uninitialised (C 29575) and
  only writes the fields it uses; Rust needs an initialised binding before the field writes, so the
  port takes the `set_wm_state` shape (C 27836, `std::mem::zeroed ()`) - and the nine field writes
  are then the `msg.xclient.*` rows the mirror's opaque union records (measured: +9 `E0609`, the
  whole `xclient` class delta of this slice).
* **The `E0425` delta is one new name and five new sites.**  `XCreateFontCursor` is a **new** name
  (one site); `XFlush`, `XMapRaised`, `XSendEvent`, `gui_set_bitmap_icon` and
  `x_wm_set_window_state` each gain one site of a name earlier slices already owed (measured, both
  sides).  In arm B the counts are identical in shape except that `XMapRaised` starts from 1 there
  (the GTK arm does not map, so C 29605's site is the second).
* **`x_create_font_cursor`'s two returns are C's two returns.**  The `glyph <= 65535` test and the
  `make_invisible_cursor` fallback (C 12067-12095, landed in slice 10) are the function's whole
  body; the only new name is `XCreateFontCursor`.

### 50. Slice 37 (C 29619-29851): `x_free_frame_resources`

* **The unit's own `pointer_invisible!` reader is declared *before* its user.**  `f->pointer_invisible`
  is frame.h 437's `bool_bf`, and `include/frame.rs` 540-545 names the shift for this arm while the
  mirror carries no accessor, so the unit declares the member's reader (`pgtkterm.rs` 3996-4003's
  copy of the same macro).  The first writing put that `macro_rules!` *after* the function; the
  reading showed `cannot find macro pointer_invisible in this scope` (`macro_rules!` is scoped
  textually, first use must follow the definition) - one bare `error:` row in **all three** arms -
  and moving the declaration above the function clears it (measured, `fix2` vs `fix`).
* **`FRAME_X_BASIC_COUNTER`/`FRAME_X_EXTENDED_COUNTER` are the mirror's, not the unit's.**  The two
  XSync accessors look like comments in `include/xterm.rs` 1218-1219, but the `macro_rules!` pair at
  1220-1230 is right below them; the first writing declared unit-local copies reading `basic_counter`
  (a field that does not exist) - a duplicate that would have been an `E0659` - and the reading's
  `have_xsync` arm is not compiled by any of the three arms, so the mirror's pair stands and the
  C-order comparison `!= None` is written `!= None as XSyncCounter` (the slice-35
  `!= None as Window` spelling).
* **Three C conversions got their port spelling.**  `dpyinfo->motif_drag_atom = None` is
  `None as Atom`; the six `... != -1` tests on `unsigned long` pixels are
  `!= (-1 as c_int) as c_ulong` (the `xfns.rs` 2984 spelling: C's `int` -1 converts to `ULONG_MAX`);
  and `f == hlinfo->mouse_face_mouse_frame` needs `f as *mut c_void` because that member is
  `include/dispextern.rs` 2129's bare-tag pointer (the `haikufns.rs` 2514 shape) - without it the
  arm-A reading gains an `E0308`.
* **`supports_xi2` is a `bool`, so C's `if (dpyinfo->supports_xi2)` is not `!= 0`.**  The arm-C
  reading caught that too (`expected bool, found integer`): the port writes the member alone.
* **Arm B's four extra `E0609` are C's `HAVE_GTK3` block.**  C 29808-29814 reads
  `scrollbar_background_css_provider`/`scrollbar_foreground_css_provider`, which this file's
  `x_output` carries under the pgtk gate (`include/xterm.rs` 806 records them; the guard above them
  is `have_pgtk`), so a build with `have_gtk3` but not `have_pgtk` sees the two fields as unknown -
  the arm-B-only class, named rather than hidden.
* **`n_protected_windows` is a mirror gap inside an arm no reading compiles.**  C 29804's
  `dpyinfo->n_protected_windows = 0;` sits in `#ifdef USE_TOOLKIT_SCROLL_BARS`, which none of the
  three arms sets and which no mirror carries as a field (measured); the port writes C's path and
  the ledger records it here - it will show as an `E0609` on the round that first compiles a
  `use_toolkit_scroll_bars` arm.

### 51. Slice 38 (C 29856-29889): `x_destroy_window`

* **The first writing's one token row is `0`.**  C 29863 reads `if (dpyinfo->display != 0)`, and the
  `extract-abi --check` token comparison is *textual*: writing it as `!= std::ptr::null_mut ()` -
  this file's habit for the same test elsewhere - left `function x_destroy_window does not mention 0`
  on the list (measured: 113 rows with that row, 112 without).  The port therefore spells C's
  zero as `!= 0 as *mut Display`, the `(*relief).gc != 0 as GC` shape this file already has
  (xterm.rs 10881), and the row retires.
* **The slice adds no `E0425` at all, in any arm.**  Every name it uses is already in the unit's
  reach: `x_free_frame_resources` is this file's own (slice 37), `xfree` is `include/lisp.rs`
  3897's declaration, and `saved_menu_event`/`preedit_chars`/`xi_masks`/`output_data.x`/
  `reference_count` are real mirrored members - so the three arms' totals, class counts and warning
  counts are *identical* to their baselines (measured both sides).  The reading this slice
  contributes is the `--check` row retiring, not a probe delta.
* **C 29873-29883's nested pair is carried with both arms, and only one of them can be measured.**
  `HAVE_XINPUT2` + `HAVE_XINPUT2_1` selects `XFree`; the `#else` selects `xfree`.  No arm this lane
  runs sets `have_xinput2_1`, so the `XFree` arm is **carried text that no reading compiles** -
  named here rather than counted as verified (the P-86 ③ class, the `x_destroy_window` sibling of
  Decision 50's `n_protected_windows`).
* **`xfree ((*f).output_data.x)` needs no cast.**  `output_data.x` is `include/frame.rs` 38's
  `*mut c_void` (C's `struct x_output *x` inside the union), which is exactly the parameter type
  `xfree` takes; the C 29885/29886 pair therefore reads as the port's own pointer assignment.

### 52. Slice 39 (C 29891-29978): `x_intern_cached_atom`

* **`countof` was not in this unit's prelude, and the fix is C's own include line.**  The C text
  calls `countof (x_atom_refs)` (C 29962); `<intprops.h>`'s sibling `<stdcountof.h>` reaches
  `xterm.c` through `lisp.h` 28, and the port's convention for that chain is a per-unit `include!`
  annotated with the C line that brings it (38 units already carry it - `alloc.rs` 64).  This slice
  adds that one line to the unit's include block (`lisp.h 28: #include <stdcountof.h>`) rather than
  declaring the macro again.
* **`INT_STRLEN_BOUND` is written at the use site.**  `lib/include/intprops.rs` is a blind mirror and
  no unit includes it, so the units that need the semantics write the Rust primitive
  (`androidselect.rs` 111-119).  The two computed `char` arrays of C 29905-29908 keep C's arithmetic
  (`15 - 2 + INT_STRLEN_BOUND! (c_int)` - `sizeof` of each 14-character literal is 15) and the
  arm's size is 24, the same number C's expression yields.
* **The eight predefined atoms the mirror lacks stand at this use site.**  `x11_constants` carries
  `XA_PRIMARY`/`XA_ATOM`/`XA_CARDINAL`/`XA_WINDOW` (the four earlier slices needed) and not
  `XA_SECONDARY`/`XA_BITMAP`/`XA_COLORMAP`/`XA_CURSOR`/`XA_DRAWABLE`/`XA_FONT`/`XA_INTEGER`/
  `XA_STRING`, so those eight are constants next to their user with `<X11/Xatom.h>`'s measured
  values (2/5/7/8/17/18/19/31), the `IconicState`/`MapNotify` shape (P-72 rule 4).  The mirror
  route (adding them to `x11_constants` beside the four) stays open as one move, and the C names in
  the body stay bare either way - the four the mirror owns come in through the function's `use`.
* **`motif_drag_atom_name` is an array in the mirror, so C's decay needs `.as_ptr ()`.**  The first
  writing passed the member straight to `strcmp` (as C does); `include/xterm.rs` 601-605 records it
  as `[c_char; 33]`, so `strcmp`'s parameter type (`include/lstring.rs`/`lib/include/string.rs` 30)
  rejected it - one `E0308` in every arm, cleared by `.as_ptr ()` (measured: the same fix keeps the
  four other sites C writes as pointer comparisons intact).
* **`x_atom_refs` is this function's own debt, and it is a *name*.**  The table at C 987 (122
  `ATOM_REFS_INIT` rows) is what Decision 9 defers; this function is its first reader, so the three
  sites land as the owed name `x_atom_refs` (one new `E0425` name, three sites) while the record
  type `x_atom_ref` and the `ATOM_REFS_INIT!` macro are already declared above.  The round that
  lands the table retires those three sites and nothing else.

### 53. Slice 40 (C 29980-30074): `x_get_atom_name`

* **The switch is a `match`, and its four structural tokens are a notes row.**  C 29999's
  `switch (atom)` over the ten predefined atoms plus `default:` becomes ten arms in C's order and
  `_` for the default (PORTING.md 2.2/P-04, the `x_handle_net_wm_state` row's shape), and C 29997's
  bare `alloca (45 + INT_STRLEN_BOUND (int))` is the port's stack pool (P-05, the
  `x_lower_frame_1`/`xm_find_targets_table_idx` shape).  The first writing left
  `function x_get_atom_name does not mention alloca, case, default, switch` on the list; the row
  added to `docs/ports/macro-body-notes.tsv` closes it (measured: 111 rows with it, 110 without).
* **The definition retires two `E0425` sites that earlier slices owed.**  `x_get_atom_name` was
  already *called* twice in the landed text (the name appeared as an unresolved function), so this
  slice's own reading is a net **+4**: three more `x_atom_refs` sites (the second reader of
  Decision 9's table), two `XScreenNumberOfScreen` sites and one `XGetAtomName` (**a new name**),
  less the two call sites the definition resolves.
* **`motif_drag_atom_name`'s decay is `.as_ptr ()` here too.**  Slice 39's `strcmp` defect was the
  same member; this function passes it to `xstrdup` (C 30034), and `include/xterm.rs`'s mirror type
  `[c_char; 33]` needs the same spelling - the C text's bare name is the pointer C's own decay
  yields.
* **The `default:` arm keeps C's order.**  The motif drag atom test, the two `sprintf` + `xstrdup`
  returns, the `x_atom_refs` walk, then `XGetAtomName` with the `need_sync` write and the
  `xstrdup`/`XFree` pair, then C's `break` (the arm's end) and the function's `return value`.

### 54. Slice 41 (C 30076-30146): `x_intern_atoms` and `x_embed_frame`

* **C's three fixed `[256]` arrays are zero-initialised (Deviation 12).**  C 30087-30089 declares
  `indices[256]`, `new_names[256]` and `results[256]` uninitialised and only ever writes before
  reading them; Rust needs an initialised binding, so the port takes the file's own array shape
  (`[0; 256]`, and `[std::ptr::null_mut (); 256]` for the pointer array - the `xterm.rs` 14678
  `char buf[256]` precedent).  The `count > 256` arm keeps C's text, including the fact that C hands
  `new_names` to `XInternAtoms` there without ever having written it.
* **Indexing a pointer is `.offset`, not `[]`.**  `atoms_return` is a parameter of type `*mut Atom`,
  so the two writes of C 30103/30119 are `*atoms_return.offset (…)` - the first writing used `[]`
  and the reading showed two `E0608` (`cannot index into a value of type *mut u64`) in every arm.
* **`FRAME_OUTPUT_DATA` stays the raw union member, so the use site casts (P-72 rule 3).**  The
  mirror's `FRAME_OUTPUT_DATA!` is C's `FRAME_X_OUTPUT` (include/xterm.rs 973/979) and yields
  `*mut c_void`; C 30136 writes `FRAME_OUTPUT_DATA (f)->parent_desc`, so the port writes
  `(*(FRAME_OUTPUT_DATA! (f) as *mut x_output)).parent_desc` - the `haikufns.rs` 1123 /
  `androidterm.rs` 835 shape for the same family.  Writing `(*FRAME_OUTPUT_DATA! (f)).parent_desc`
  left one `E0609` (`no field parent_desc on type std::ffi::c_void`) per arm that compiles
  `x_embed_frame`.
* **The four error helpers resolve; `XInternAtoms`/`XReparentWindow` do not.**  `x_catch_errors`,
  `x_had_errors_p` and `x_uncatch_errors_after_check` are declared by a mirror this unit's include
  graph reaches (measured: the slice adds **no** `E0425` row for any of them), while
  `XInternAtoms` (two sites) and `XReparentWindow` (one) are the bare Xlib class.  Arm B compiles
  only `x_intern_atoms` - C 30123's `#ifndef USE_GTK` - which is why its delta is +2 against the
  other arms' +3.

### 55. Slice 42 (C 30342-30420): the three window-manager-hint writers

* **C's `Arg al[1]`/Xt arms are carried text that no arm here compiles.**  `x_wm_set_window_state`
  (C 30350-30351) and `x_wm_set_icon_pixmap` (C 30396-30399) call `XtSetArg`/`XtSetValues`, whose
  types (`Arg`, `XtNinitialState`, `XtNiconPixmap`, `XtNiconMask`) come from `<X11/Intrinsic.h>`, and
  `XtSetArg!` is declared in `xfns.rs`'s unit rather than this one.  The three arms this lane reads
  never set `use_x_toolkit`, so those calls are carried text: this slice adds **no** row for them
  (P-86 ③'s class, named here rather than counted as verified).  `XtNinitialState!` *is* this file's
  own macro (xterm.rs 713) because an earlier slice needed it.
* **`x_wm_set_icon_pixmap`'s arm order is C's own `#if`/`#elif`/`#else`.**  `USE_GTK` first, then
  `#elif defined (USE_X_TOOLKIT)` (C's own comment: "same as in x_wm_set_window_state"), then the
  plain-X arm - so arm B compiles `xg_set_frame_icon` and not the paren site below it, which is why
  arm B's warning count does not move while arms A and C gain one.
* **The four Xutil.h flags stand at the use site.**  `StateHint` (`1L << 1`), `IconPixmapHint`
  (`1L << 2`), `IconPositionHint` (`1L << 4`) and `IconMaskHint` (`1L << 5`) are measured against this
  machine's `<X11/Xutil.h>` 135-139; the mirror's `x11_constants` carries that header's `NormalState`
  and not these (P-72 rule 4, the `XA_*` shape of slice 39).  `StateHint` is **2**, not 1 - the value
  is measured, not inferred from the name.
* **C's parentheses are kept, and the probe's warning class grows by one.**  `flags |=
  (IconPixmapHint | IconMaskHint)` (C 30404) carries C's parens, so the *probe* compile reports one
  more `unnecessary parentheses around assigned value` - a class this file already carries **seven**
  times (measured both sides: 7 -> 8, arms A and C; arm B does not compile that arm).  **`build-tu`'s
  warning set is unchanged** (30 warnings, identical set by name and count), which is the reading the
  project's own claim uses; dropping the parens would be token-identical (punctuation is not
  compared) and is the move to make if that class is ever to be emptied.
* **The three definitions retire five owed call sites.**  `x_wm_set_window_state` was owed at two
  sites in the landed text (C 29273 in `x_make_frame_visible` and C 29600 in `x_iconify_frame`'s
  not-toolkit arm - the measured delta is **-2** in every arm), and the two icon writers were still
  missing items.  The new sites are `XSetWMHints` (3 in arms A/C, 2 in arm B), `image_bitmap_pixmap`
  (1, `src/image.c` 463's definition), `x_bitmap_mask` (1, `src/image.c` 507's) and, in arm B only,
  `xg_set_frame_icon` (1) - plus, again in arm B alone, one more uncoded `GDK_WINDOW_XID` row from
  `FRAME_OUTER_WINDOW` (C 30413) under `use_gtk`.

### 56. Slice 43 (C 30158-30340): `x_wm_set_size_hint`

* **The mirror gained `XSizeHints`, and this is the slice's one mirror change.**  C 30162 declares
  `XSizeHints size_hints;` and the record was carried nowhere (`xlib.rs` had `XWMHints` and not its
  Xutil.h sibling), so `include/xlib.rs` now carries `XSizeHints` (and the named
  `XSizeHintsAspect` its two *anonymous* aspect structs need - PORTING.md 1.3) with the layout
  measured on this machine: `[sizeof=80, align=8]`, `min_aspect` @48, `max_aspect` @56,
  `win_gravity` @72, plus `const _` assertions for size/align/offsets.  **P-87 ①'s legs, run for
  this change**: `tools/check-all-units.sh` -> `checked 83 unit(s), 0 failure(s)` and
  `checking 83 unit(s), 0 unit failure(s)` (1m48s, `CARGO_TARGET_DIR=/tmp/xs43/allunits`);
  `xtask layout-check` -> `layout probe ok` / `layout-check passed` with **the same 124 warnings as
  before the change** (measured both sides); the mirror's ledger row and its carried-list sentence
  were updated in the same slice (`docs/ports/include-xlib.md`).
* **C 30295's `goto no_read` is the port's labelled block, and the token row is closed by a note.**
  The port writes `'no_read: { … break 'no_read; … }` around C 30298-30321's block (PORTING.md 2.2,
  the `x_focus_frame` shape), so C's `goto` cannot reappear: the first reading left
  `function x_wm_set_size_hint does not mention goto` and a `docs/ports/macro-body-notes.tsv` row
  closes it (measured: 105 rows with it, 104 without).
* **The mirror carries five of these `FRAME_*` accessors as *functions*, so the `!` spelling is
  wrong.**  `FRAME_PIXEL_WIDTH`, `FRAME_PIXEL_HEIGHT`, `FRAME_COLUMN_WIDTH`, `FRAME_LINE_HEIGHT`,
  `FRAME_TEXT_COLS_TO_PIXEL_WIDTH` and `FRAME_TEXT_LINES_TO_PIXEL_HEIGHT` are `pub fn` in
  `include/frame.rs` (1036/1038/1351/1353/1800/1808) while `FRAME_MENUBAR_HEIGHT!` really is a
  macro; the file's own sites write them accordingly (5326/5343 without the bang, 5321/5344 with
  it).  The first writing used `!` for all six and the reading showed **six uncoded
  `cannot find macro` rows** - now zero, with the spellings taken from those existing sites.
* **A cfg attribute takes exactly one predicate.**  `#[cfg (all (…), not (…))]` is `E0805`
  (`malformed cfg attribute input`) - the first writing had one such attribute (the
  `hints_changed` declaration) and the reading caught it; the valid form keeps every predicate
  inside the single `all (…)`.
* **The eight Xutil.h flags stand at the use site, and `#ifdef PWinGravity` is live.**  `USPosition`
  1, `USSize` 2, `PPosition` 4, `PSize` 8, `PMinSize` 16, `PMaxSize` 32, `PResizeInc` 64,
  `PAspect` 128, `PBaseSize` 256, `PWinGravity` 512 - measured against this machine's
  `<X11/Xutil.h>` 102-112, the `XA_*` shape of slice 39 (the mirror's `x11_constants` carries that
  header's `NormalState` and none of these).  C 30325's `#ifdef PWinGravity` is a header test, not a
  configuration one, and the header defines it (measured) - so the port carries the block live, and
  says so in a comment where C's `#ifdef` sits.
* **The definition retires four owed call sites.**  `x_wm_set_size_hint` was called at C 27612,
  28315, 28489 and 28629 in the landed text (the measured delta is **-4** in every arm); the new
  sites are `XGetWMNormalHints` and `XSetWMNormalHints` (one each), the `void` the
  `FRAME_MENUBAR_HEIGHT!` macro's body names (2 -> 3 - that class is the mirror's faithful
  `((void) f, 0)` text) and one more site each for `x_display_pixel_width`/`x_display_pixel_height`.
  Arm B is **-3**: its `use_gtk` arm compiles only `xg_wm_set_size_hint` (a new name) and not the
  whole `#ifndef USE_GTK` body.

### 57. Slice 44 (C 30447-30581): the image hook, the display-name test and the initialisation helpers

* **C 30466-30489 is owed text, and the ledger says so.**  `emacs_options[]` (C 30467-30483) and
  `x_timeout_atimer_activated_flag` (C 30487) live inside `#ifdef USE_X_TOOLKIT`, which no arm this
  lane runs sets; carrying the table faithfully needs two more `<X11/Xresource.h>` records
  (`XrmOptionKind`, `XrmOptionDescRec`, Xresource.h 327-343) in the mirror, so the region is named
  here as **not landed** and left for the round that first compiles a toolkit arm - the same
  accounting `x_atom_refs` (Decision 9) and `n_protected_windows` (Decision 50) already carry.
  **Taken by slice 107 (Decision 120)**: the two `<X11/Xresource.h>` records landed in
  `include/xlib.rs` first, and C 30467-30483's eleven entries plus C 30487's flag are now the
  unit's own text under `#[cfg (feature = "use_x_toolkit")]`.  The
  section comments around it *are* carried, so the port's text keeps C's shape.
* **`x_initialized` is a tentative definition, so the port writes `= false`.**  C 30491's
  `static bool x_initialized;` is zero-initialised by C's own rule; the port's shape for that is the
  `pub static mut NAME: bool = false;` with a comment naming C's line and "tentative, i.e. false"
  (`android.rs`'s `android_init_gui` precedent).  Retiring it removes the last `missing file-scope
  object` row this file had.
* **`strncmp`'s third parameter is `usize`, and C's lengths here are `ptrdiff_t`.**  The four sites
  of C 30516-30527 pass `system_name_length`/`length_until_period`, so the port writes `as usize`
  (the `emacs.rs` 1049 / `doprnt.rs` 251 shape); the first writing passed them raw and the reading
  showed **four `E0308`** in every arm.
* **`Emacs_Pixmap` is not in this arm's scope, and that is the arm model, not a defect.**  The type
  is `include/dispextern.rs` 773's record under `have_window_system`; the X11 arms this lane runs
  set `have_x_windows` and not that feature (Decision 2's reading), so `x_free_pixmap`'s signature
  names a type no mirror of *this* arm defines - one new `E0425` name, recorded here.  (C's
  `HAVE_X_WINDOWS` implies `HAVE_WINDOW_SYSTEM`; the port's arms are the narrower approximation the
  lane measures, and the ledger's Decision 2 owns that.)
* **The four definitions add five `E0425` sites and retire five `--check` rows.**  The new sites are
  `Emacs_Pixmap` (1), `XOpenDisplay` (1), `XCloseDisplay` (1), `request_sigio` (1) and
  `unrequest_sigio` (1) - all five are names the file already owed elsewhere except `Emacs_Pixmap` -
  and the rows that retire are the four `missing function`s plus `missing file-scope object
  x_initialized`.

### 58. Slice 45 (C 30583-30660): the GTK log handler, `x_display_id` and the two XInput2 preludes

* **`x_display_id` is a tentative definition, and it is the only row this slice retires.**  C 30595's
  `static unsigned x_display_id;` takes the port's `pub static mut NAME: c_uint = 0;` with C's
  comment above it and "tentative" in the trailing note (the `x_initialized` shape of slice 44); the
  three functions around it are **not** in the frozen table's checklist - the frozen configuration
  defines neither `USE_GTK` nor `HAVE_XINPUT2` - so the `--check` reading moves by exactly one row
  (99 -> 98) and the three land as carried text with their own feature gates.
* **Which arm compiles which of the three.**  `my_log_handler` (C 30583-30591) is `#ifdef USE_GTK`,
  so **arm B** compiles it; `xi_select_hierarchy_events` (C 30602-30622) is `#if defined HAVE_XINPUT2
  && ! defined HAVE_GTK3`, so **arm C** compiles it; `xi_check_toolkit` (C 30636-30658) needs
  `HAVE_XINPUT2` *and* `HAVE_GTK3` at once, which no arm this lane runs sets - it is the slice's one
  piece of **carried text that no reading compiles** (P-86 ③, named here).  Arm A's readings
  therefore do not move at all, which is the check that the gates are right.
* **`alloca` is the stack pool again.**  C 30610's `mask.mask = m = alloca (l);` is a *chained*
  assignment: the port writes `m = SAFE_ALLOCA! (sa_avail, l as usize)` and then `mask.mask = m`, in C's
  evaluation order, with the chain named in the comment (P-05's stack pool, the
  `xm_find_targets_table_idx` shape).
* **A stray attribute is not harmless text.**  The first writing put `#[cfg (feature = "use_gtk")]`
  (and two siblings) on the line *above* the C comment each function carries; an attribute on a
  comment line applies to the next item, so those three would have re-gated the functions by a
  second (weaker) predicate.  They were removed before the reading; the rule is the file's own
  (status 3.832's note about `#[cfg]` on comment lines).
* **The deltas are the arm-specific ones.**  Arm A: 0.  Arm B: +1 (`strstr`, a new name in
  `my_log_handler`).  Arm C: +11 - `XISelectEvents`, `XISetMask` (3 sites), `XI_DeviceChanged`,
  `XI_HierarchyChanged`, `XI_LASTEVENT`, `XI_PropertyEvent` are **new names**, and
  `XIEventMask`/`XIMaskLen`/`XIAllDevices` each gain one site of a name the slice-10 block already
  carried.  **Arm reachability (P-86 ⑦) is taken in arm C for this slice** - the only arm that
  compiles a function here.

### 59. Slice 46 (C 31845-31867): `x_delete_selection_requests`

* **The `continue` is a notes row, and the increment stays at the loop's end.**  C 31857's
  `continue` inside C 31850's `for` runs the increment (`event = X_NEXT_KBD_EVENT (event)`), so the
  port keeps that step as the loop's last statement and writes the `continue` as `break 'body`
  (PORTING.md 2.2, the `x_dnd_get_target_window_1` row's shape); the first reading left
  `function x_delete_selection_requests does not mention continue` and a
  `docs/ports/macro-body-notes.tsv` row closes it (measured: 98 rows with it, 97 without).
* **The four idioms are the ones this file already established thirty thousand lines earlier.**
  `event->kind` is `input_event_bits! (kind, …)` (P-80's reader, the setter arm writes `NO_EVENT`),
  `EVENT_INIT` is the mirror's plain function (`xterm.rs` 5163's spelling, no bang),
  `SELECTION_EVENT_DPYINFO!` is xterm.h's mappable and `X_NEXT_KBD_EVENT!` is this unit's macro
  (xterm.rs 742) - so this function adds **no** new name, and its one new row is the recorded
  `no field sie` class (`include/xterm.rs`'s `buffer_input_event` records the union's members as
  `ie` only; the baseline already carried two `sie` rows from the C 893-944 block).
* **The reading is +1 in every arm, of that one class.**  A `have_x_windows` 686/69 -> 687/69,
  B `+have_gtk3+use_gtk` 703/70 -> 704/70, C `+have_xinput2` 732/71 -> 733/71, each of them
  `E0609` (`sie` 2 -> 3).  No `E0425`, no uncoded row and no warning is added.

### 60. Slice 47 (C 31872-31974): `x_delete_display`

* **The backward `goto` is an inner labelled loop, and its token is a notes row.**  C 31901's
  `again:` label sits inside C 31899's `for` body and C 31912 jumps *back* to it, so the port writes
  `'again: loop { … break 'again; … }` inside the walk with the `for`'s step after it (PORTING.md
  2.2); the first reading left `function x_delete_display does not mention goto` and a
  `docs/ports/macro-body-notes.tsv` row closes it (measured: 97 rows with it, 96 without).
* **Two defects the readings found.**  `(*t).display_info.x as *mut x_display_info` - the port's
  spelling is `xfns.rs` 378's *value* cast (`display_info.x` is already the pointer C compares, so
  the first writing's extra `*(…)` produced `E0606: casting std::ffi::c_void as *mut x_display_info
  is invalid`), and `color_names[i]` indexes a `*mut *mut …` parameter member, so it is
  `*(*dpyinfo).color_names.offset (i as isize)` (the slice-41 `E0608` class: `cannot index into a
  value of type *mut *mut color_name_cache_entry`).
* **Three new names, all owed.**  `delete_terminal` (one site - `terminal.rs` 416 defines it, and
  this unit's graph carries no declaration), `next_noop_dpyinfo` (two sites - the static is declared
  by no member of this file yet) and `x_remove_selection_transfers` (one site - this file's own
  later definition).  The other dpyinfo members the slice frees are already mirrored, which is why
  the class counts other than `E0425` do not move.
* **The gated arms.**  `HAVE_X_SM` (C 31885-31889) and `USE_TOOLKIT_SCROLL_BARS` (C 31966-31968) are
  carried and compiled by no arm this lane runs; `HAVE_XFIXES` (C 31962-31965) and `HAVE_XINPUT2`
  (C 31969-31972) are carried too, and neither is set by the three arms either - so
  `monitored_selections`, `protected_windows` and `x_free_xi_devices`'s arm are text, not readings
  (P-86 ③'s class, named).

### 61. Slice 48 (C 32063-32208): `x_delete_terminal`

* **The backward `goto delete_more_frames` is a labelled loop.**  C 32082's label precedes C 32083's
  `FOR_EACH_FRAME` and C 32091 jumps back to it, so the port wraps the frame walk in
  `'delete_more_frames: loop { … break; }` and writes the jump as `continue 'delete_more_frames`
  (PORTING.md 2.2; the `x_delete_display` row of slice 47 is the same shape read the other way).
  The first reading left `function x_delete_terminal does not mention goto` and a
  `docs/ports/macro-body-notes.tsv` row closes it (measured: 96 rows with it, 95 without).
* **C 32157-32167 is `#if false`, so it is recorded and not transcribed.**  The block
  (`XrmSetDatabase`/`XrmDestroyDatabase` with the Bug#21974 comment) is C's own dead text; this tree's
  rule for literal `#if false` is the `src/xfns.rs` 958 one - a comment where the text stands and the
  ledger's record - so the port writes the comment and keeps C's surrounding sentence.
* **Three defects the readings found.**  `scratch_cursor_gc` is a `GC` (`*mut _XGC`), so C's
  `if (dpyinfo->scratch_cursor_gc)` is `!…is_null ()` and not `!= 0` (one `E0308` per arm);
  `x_dnd_finish_display` is P-72 rule 3's `*mut c_void` while `(*dpyinfo).display` is
  `*mut Display`, so the C 32123-32125 comparison casts at the use site - `xterm.rs` 14320's
  spelling (another `E0308` per arm); and `XFRAME` is a *function* in the mirror
  (`include/frame.rs` 770), so the bang spelling left one uncoded `cannot find macro XFRAME` row in
  every arm (the file's own sites at 6731/6808/8036 show the no-bang form).
* **Six new `E0425` sites per arm.**  `DestroyAll`, `XFreeModifiermap`,
  `delete_keyboard_wait_descriptor` and `image_destroy_all_bitmaps` are **new names**, and
  `XCloseDisplay`/`XSetCloseDownMode` each gain one site; the `HAVE_X_I18N` (`xim_close_dpy`, this
  file's own), `use_gtk` (`xg_display_close`) and `have_xkb` (`XkbFreeKeyboard` with the mirror's
  `True`) arms are carried, and none of the three arms this lane runs sets `have_x_i18n` or
  `have_xkb` - so those two are text, not readings (P-86 ③, named).

### 62. Slice 49 (C 32393-32417): `init_xterm`

* **Four nested conditions, three of which the lane's arms split apart.**  C 32396's
  `HAVE_XWIDGETS && ! HAVE_PGTK` (the two `xputenv` calls), C 32403's `#ifndef HAVE_XINPUT2` with its
  own `#ifndef HAVE_GTK3`/`#else` pair (`xputenv ("GDK_CORE_DEVICE_EVENTS=1")` against
  `gdk_disable_multidevice ()`) and C 32414's `HAVE_X_I18N` are all carried (P-86 ①).  Arm A
  (`have_x_windows` alone) compiles the plain-X `xputenv` arm, arm B (`+have_gtk3+use_gtk`) the
  `gdk_disable_multidevice` one and arm C (`+have_xinput2`) **neither** - which is why the three
  readings differ (+1, +1, 0): the deltas follow the gates, not the function.
* **`text_conversion_interface` stays unlanded and that is said out loud.**  C 32386-32389's table
  (and the `x_reset_conversion` C 32367-32385 whose only caller it is) is `HAVE_X_I18N`-only - no arm
  this lane runs sets that feature - so this slice carries `init_xterm` alone and names the table as
  still owed rather than landing an unverifiable 20-line pair out of order.  The reference in
  `register_textconv_interface (&text_conversion_interface)` is therefore carried text (P-86 ③).
* **The two new names are the two `xputenv`/GDK entries.**  `xputenv` (arms A, and the xwidgets arm
  no arm compiles) and `gdk_disable_multidevice` (arm B) are new names; neither arm gains an uncoded
  row or a warning.

### 63. Slice 50 (C 32210-32303): `x_have_any_grab` and `x_create_terminal`

* **The `hook!` helper had to come with the block.**  Every one of the 42 assignments C 32244-32298
  makes goes to an `Option<unsafe extern "C" fn (…)>` whose parameters the mirror degrades to
  `*mut c_void` (P-72 rule 3), so each is the `hook!` transmute `src/term.rs` 140-144 and
  `pgtkterm.rs` 7032-7036 already carry (PORTING.md 1.4: a direct `as` is `E0605`).  The first
  writing used `hook!` without declaring it: the reading showed **42 uncoded
  `cannot find macro hook`** rows - one per site - and the declaration (gated `have_x_windows`,
  placed before its first use because `macro_rules!` is textually scoped) cleared all of them.
* **The `query_colors` field is C's three-parameter shape.**  `include/termhooks.rs` 262 carries
  `(*mut c_void, *mut Emacs_Color, c_int)`, and the first writing's five-parameter guess surfaced as
  `E0308: incorrect number of function parameters` anchored in the macro's body with a note at the
  call site (C 32258) - one row per arm, fixed and re-measured.
* **Two hook fields are gated in the mirror but not in C, and that shows.**  C 32293's `free_pixmap`
  is unconditional while the mirror carries the field under `have_window_system`, so arm A (which
  sets `have_x_windows` only) reports `no field free_pixmap`; C 32279's `popup_dialog_hook` is gated
  by `USE_X_TOOLKIT || USE_GTK` while the mirror's field needs `have_ext_menu_bar`, so arm B reports
  it missing as well.  Both are the arm model's narrowing (Decision 2/57's class), recorded rather
  than worked around.
* **`x_redisplay_interface` is named, not landed.**  C 32024-32059's table sits before this function
  and needs the whole `redisplay_interface` initializer; the port's C 32237 therefore names it as an
  owed object, and the round that lands the table retires that row together with its entries.
* **The reading is +21 / +22 / +24.**  Arm A gains 21 sites - the 20 hook names this file has not
  landed (`XTtoggle_invisible_pointer`, `XTread_socket`, `XTmouse_position`, `XTframe_rehighlight`,
  the three scroll-bar hooks, the four default-size hooks, `x_defined_color`, `x_bitmap_icon`,
  `x_implicitly_set_name`, `x_menu_show`, `x_change_tab_bar_height`, `x_change_tool_bar_height`,
  `x_get_string_resource`, `x_toolkit_position`'s callee list) plus `create_terminal` and
  `x_redisplay_interface`; arm B adds `xw_popup_dialog` (its `use_gtk` arm) and arm C adds
  `x_have_any_grab`'s two members (`devices`/`num_devices` sites) - the exact per-arm counts are in
  the readings below.

### 64. Slice 51 (C 32305-32499): `x_initialize` and the three `*_for_lisp` error helpers

* **`x_initialize`'s five arms are all text here.**  C 32322's `THREADS_ENABLED` is **undef** in the
  frozen table (measured), and `USE_X_TOOLKIT`, `USE_TOOLKIT_SCROLL_BARS && ! USE_GTK` and
  `USE_CAIRO` are set by no arm this lane runs - so of the function's five conditionals only the
  *unconditional* head (`baud_rate`, the four unit globals, `Fset_input_interrupt_mode`) and the tail
  (`XSetErrorHandler`/`XSetIOErrorHandler`) are compiled anywhere.  The armed text is carried with
  its feature name (P-86 ①) and named here as unverified (P-86 ③).
* **Two conversions the reading caught.**  `XSetIOErrorHandler`'s parameter is the mirror's
  `Emacs_XIOErrorHandler` = `Option<unsafe extern "C" fn (*mut c_void) -> c_int>`
  (xterm.rs 701-702) while `x_io_error_quitter` itself takes `Display *` (C 27114's own signature,
  xterm.rs 14711), so C's implicit conversion is the port's `hook!` cast - `Some (f)` alone is
  `E0308: expected fn pointer, found fn item`, and `hook!` is exactly the `Option<…>`-shaped
  conversion the same unit's hook assignments use (the `xm_drag_window_io_error_handler` precedent at
  xterm.rs 1710-1714 takes the other route: it is *declared* `*mut c_void` because C defines it
  here).  `XSetErrorHandler` is the one **new name** (C 32358), so its argument is not type-checked.
* **The three helpers are four-line functions and resolve completely.**  `x_fast_protocol_requests`
  is `include/globals.rs` 7187's carrier; `x_catch_errors`/`x_check_errors`/`x_uncatch_errors` are
  declared by a mirror this unit's graph reaches (the slice-44 reading already showed that for
  `x_catch_errors`), and `x_ignore_errors_for_next_request`/`x_stop_ignoring_errors` are this file's
  own (xterm.rs 2379/2409 and the XEmbed block).  Their only delta is therefore the one
  `XSetErrorHandler` row from `x_initialize`.

### 65. Slice 52 (C 32419-32662): `mark_xterm` and `x_get_keyboard_modifiers`

* **`mark_xterm`'s gate is a five-way disjunction, and it splits the arms.**  C 32423-32425's
  `HAVE_XINPUT2 || USE_TOOLKIT_SCROLL_BARS || HAVE_XRANDR || USE_GTK || HAVE_X_I18N` gates the
  display walk (and C 32426-32428's inner pair gates `i`), so **arm A compiles none of it** - its
  delta is the three unconditional `XSETFRAME`/`mark_object` pairs plus `x_get_keyboard_modifiers` -
  while arm B (`use_gtk`) compiles the `last_monitor_attributes_list` mark and arm C (`have_xinput2`)
  the device-name loop.  All five names are carried as `any (…)` over the lowercase features
  (P-86 ①), and the four inner arms keep C's order.
* **`mark_object` and `x_find_modifier_meanings` are this slice's two new names.**  `mark_object` is
  declared per unit (`alloc.rs`, `pgtkterm.rs` 224 …) and by no mirror this unit's graph reaches, and
  `x_find_modifier_meanings` is this file's own definition, still ahead of the lane - so the reading
  is three/four `mark_object` sites plus one `x_find_modifier_meanings` site per arm.
* **Two record gaps show up as `E0609` exactly where C's own conditions put them.**
  `last_monitor_attributes_list` (marked under C 32461's `HAVE_XRANDR || USE_GTK`) is not a mirrored
  member, so arm B gains one `no field` row; `devices`/`num_devices` (C 32454-32455) are the same
  class for arm C - both are the arm model's narrowing recorded rather than worked around
  (Decision 2/57/63's class).
* **`x_get_keyboard_modifiers`' two `#ifdef HAVE_XKB` arms differ in one test.**  C 32650 adds
  `!dpyinfo->xkb_desc &&`; the port carries both under their features (P-86 ①), and the `#else` arm
  is the one every arm here compiles (no arm sets `have_xkb`).  Its `list5`/`make_uint` pair is
  `include/lisp.rs` 4209/1765's, so the function's only new name is the `x_find_modifier_meanings`
  call it shares with `mark_xterm`.

### 66. Slice 53 (C 32505-32635): `x_preserve_selections`

* **The record's `kind` bitfield has a writer, and that is a mirror change.**  C 32611's
  `clear.kind = SELECTION_CLEAR_EVENT` is the first assignment of a `selection_input_event`'s
  `kind` this lane lands, so `include/xterm.rs`'s `selection_input_event_bits!` gains the setter
  arm P-80 ② prescribes (the body `src/xterm.rs` 1370-1371 gives `input_event_bits!`); the
  mirror's note claimed C's only `kind` assignment was `input_event`'s, and C 19503
  (`inev.sie.kind`) and C 32611 both refute it - C 19503's writer arrives with `XTread_socket`.
  The mirror's own `--check` and `check-mirror-dupes` 16/16/0 are unchanged, and P-87 ① read
  `83 unit(s), 0 failure(s)` + `checking 83 unit(s), 0 unit failure(s)`.
* **Three prototypes from other units' headers sit in the body** (the `x_dnd_do_unsupported_drop`
  shape at `src/xterm.rs` 3975): xterm.h 1919-1921 `x_handle_selection_event`, 1927
  `symbol_to_x_atom` and 1947-1948 `x_own_selection` - this unit's include graph reaches no mirror
  that carries them, and the transcription lands with the unit that needs it.  The one name that
  stays unresolved is `XGetSelectionOwner` (already this file's; Xlib.h's prototype has no mirror
  row), which is the slice's whole reading: +1 `E0425` per arm, in all three arms, the same name.
* **C's `#ifndef USE_XCB` / `#else` pair changes shape between the two loops it sits in.**  In the
  second loop each arm's array store is all the pair selects and the loop's `}` at C 32586 is
  shared, so the port writes two `#[cfg]` blocks inside one loop body; in the third loop the
  `#else` arm's reply handling (C 32595-32602) *precedes* the shared `if (owners[…] != …)` at
  C 32605, so that loop body carries the arm as a leading `#[cfg]` block with the shared `if`
  after it (PORTING.md 7.2).  A first writing put the `cookies`' `SAFE_ALLOCA!` under a bare
  `#[cfg]` attribute on an assignment statement, which is `E0658` (attributes on expressions); the
  arm is a block now, and the intermediate reading is recorded below rather than claimed as final.
* **`nowners++` / `counter++` are C's post-increments**, written as the statement's own increment
  beside the store they index (`x_sync_with_move` 16653's class); the two `SAFE_ALLOCA!` calls are
  C 32561-32564's `alloca`s through the port's stack pool (P-05, closed for the token gate by a
  `docs/ports/macro-body-notes.tsv` row), and `clear` is zero-initialised because C leaves
  `requestor`/`target`/`property` unwritten (Deviation 12).

### 67. Slice 54 (C 32664-33125): `syms_of_xterm`

* **The 39 `DEFVAR_*` objects are defined here, because the reference keeps them in the generated
  `globals` struct.**  `src/xterm.c` is not in the frozen build's file list, so this tree's own
  `build/emacs-ref/src/globals.h` has no rows for these variables; each is defined in this unit
  with `#[unsafe(no_mangle)]` (P-21's single definition point, the `src/xselect.rs` 3751-3767
  shape) and the mirror's `globals.rs` `variables` declaration is shadowed by the definition.  The
  initialisers are C's zero values (`Qnil` / `false` / `0`) and every real default is the
  assignment C writes under its `DEFVAR_*` - so this slice is the file's first *writer* of
  `x_fast_protocol_requests`, `x_color_cache_bucket_size` and the rest that earlier slices only
  read.
* **`doc:` has no counterpart in a compiled body.**  C's 39 `doc: /* … */` comments are the
  variables' real docstrings (`make-docfile` reads them into `etc/DOC`); the port transcribes each
  as the literal third argument of `DEFVAR_*!`, which ignores it (P-38 - the text is carried, the
  value lives in `etc/DOC`), so the token `doc` never appears and a
  `docs/ports/macro-body-notes.tsv` row records why.
* **The two `#ifdef` regions keep C's arms.**  C 32696-32701 (`USE_GTK`: the icon file's
  `build_string`, its `staticpro` and `Qx_gtk_map_stock`) becomes one `#[cfg (feature = "use_gtk")]`
  block; C 32755-32767's `#ifdef USE_TOOLKIT_SCROLL_BARS` / `#ifdef USE_MOTIF` / `#elif defined
  HAVE_XAW3D` / `#elif USE_GTK` / `#else` / `#else` chain becomes one outer
  `#[cfg (feature = "use_toolkit_scroll_bars")]` block in which each arm carries the negations of
  the arms above it (P-86 ①, one `all (…)` per attribute) plus the `not (…)` arm - C's
  value-form `#elif USE_GTK` is the port's defined-test, the file's convention for that macro.
  Each arm is a *block*, not a bare assignment: an attribute on an expression statement is `E0658`
  (slice 53's finding).
* **The docstrings are `make-docfile`'s values.**  Each literal is `tools/cinventory.py`'s
  `doc_comment_value` of the C comment (P-38: leading and trailing whitespace dropped, `\X`
  decoded), i.e. the value a future DEFVAR docstring gate would compare; the transcription was
  reviewed line by line against C's own text.

### 68. Slice 55 (C 11924-12278): the focus/pointer/window-to-frame block

* **The frame `bool_bf` flags this block touches get a unit-local accessor.**  C's
  `f->pointer_invisible`, `frame->pointer_invisible`, `old_focus->auto_lower` and
  `x_focus_frame->auto_raise` are plain fields, so no mirror carries an accessor; the unit
  declares `set_frame_flag1!` and the member-named `frame_bits!` arms it needs (P-80 ②, the
  `src/xdisp.rs` 2500-2536 shape) over `include/frame.rs`'s measured `FRAME_*_SHIFT` constants
  (frozen 1:19/1:12/1:13, the window-system arm 1:22/1:15/1:16).
* **`FRAME_X_OUTPUT`'s two spellings are both C's.**  Where C writes `f->output_data.x->FIELD` the
  port writes the union member with its cast (`(*( (*f).output_data.x as *mut x_output )).FIELD`,
  the file's dominant 151-site spelling, P-72 rule ③); where C writes the header's own
  `FRAME_X_OUTPUT (f)->alpha_identical_p` the port keeps the macro, and that member is one of
  `struct x_output`'s `bool_bf` fields `include/xterm.rs` 846-853 records as comments rather than
  storage - so those two sites are the recorded `E0609` class, the same reading slice 21 took for
  `has_been_visible`.
* **C 12112/12145's `XFIXES_VERSION >= 40000` half has no feature.**  The guard is carried as
  `feature = "have_xfixes"` (and its negation), because the version test is a build property of the
  installed Xfixes and no port feature models it; `include/xterm.rs` 428 records the same reading
  for `fixes_pointer_blanking` with its own `C guard (not a feature)` line.  This is the slice's one
  reading of that kind and is called out in the readings section rather than silently widened.
* **First-read defect, fixed in this slice.**  Landing `XTtoggle_invisible_pointer` with C's `bool`
  parameter turned `x_free_frame_resources`' pre-existing call `XTtoggle_invisible_pointer (f, 0)`
  (C 29659, written while the name was still owed) into `E0308: expected bool, found integer`; C's
  `0` for a `bool` argument is the port's `false` (P-98's class), and the call site now says so.
  The same read resolved `x_window_to_frame`'s earlier call site and the hook line in
  `x_create_terminal`.
* **C 12251's `continue` inside `FOR_EACH_FRAME`.**  The port's `FOR_EACH_FRAME!` puts the step
  after the body, so a bare `continue` would skip it; the body opens a labelled block and the jump
  is `break 'body` (PORTING.md 2.2, the `x_delete_selection_requests` shape), with a
  `docs/ports/macro-body-notes.tsv` row for the `continue` (and `0`) tokens.

### 69. Slice 56 (C 12344-12538): the two `x_any_window_to_frame`/`x_top_window_to_frame` arms and the event multiplexer

* **The `#if defined (USE_X_TOOLKIT) || defined (USE_GTK)` arm and its `#else` are both carried.**
  The frozen table leaves `USE_X_TOOLKIT`, `USE_GTK`, `HAVE_GTK3`, `HAVE_XFIXES`, `HAVE_XINPUT2` and
  `HAVE_XWIDGETS` `undef`, so the `#else` arm is the live text (`docs/baseline/config/`); P-86 ①
  carries the other arm under `any (feature = "use_x_toolkit", feature = "use_gtk")` and its
  negation, and the two arms' `x_top_window_to_frame` definitions (C 12448-12482, C 12488-12492)
  sit under those complementary gates - the file's first pair of `#[unsafe(no_mangle)]` definitions
  of one symbol.  The probes divide the same way: arm B (`use_gtk`) compiles the toolkit/GTK arm and
  `x_next_event_from_any_display`'s arm is C's `#else`, which arms A and C compile.
* **`x_any_window_to_frame` is a function in one arm and a macro in the other.**  C 12486's
  `#define x_any_window_to_frame(d, i) x_window_to_frame (d, i)` is carried as a `macro_rules!`
  with C's parameter names and body (the checklist pins the body), because C has no function of
  that name in this arm; the toolkit/GTK arm's `static struct frame *x_any_window_to_frame` is
  carried under the other gate.  `x_menubar_window_to_frame` (C 12405-12443) comes with the arm.
* **C's `while (true)` is the port's `loop`.**  `x_next_event_from_any_display`'s outer loop is
  `while (true)` in C; a Rust `while true` draws the `while_true` lint, so the port writes `loop`
  and keeps the `true` token in the body's own comment (the `src/keyboard.rs` 2669-2671 shape,
  Decision 21's class).  The first writing used `while true` and the arm's warning count moved
  69 -> 70; the final text is back at 69.
* **First-read defects, fixed in this slice.**  Landing C's `int wdesc` parameter turned the five
  pre-existing `x_top_window_to_frame (dpyinfo, <Window>)` call sites (unit 4215, 4370, 5065, 5093,
  5135, all written while the name was owed) into `E0308: expected i32, found u64`; C's implicit
  `Window` -> `int` conversion is now spelled `as c_int`, the shape the earlier
  `x_window_to_frame (dpyinfo, owner as c_int)` site already uses.  Both intermediate readings
  (732 / 70 with the five `E0308`s, then 727 / 69) are recorded below.
* **The XEvent member reads are the recorded `E0609` class.**  `include/xlib.rs` 1296-1298 carries
  `XEvent` as a 192-byte placeholder with no members (the 33 union records are owed), so
  `event->type` / `event->xany.window` / `event->xcookie.*` are `no field` rows exactly like the
  slice-14 `msg.xclient.*` reading; only the `#else` arm's `(*event).xany.window` is compiled (arm
  B's `have_xinput2`-gated `r#type`/`xcookie` block is dead text everywhere), which is why arm B's
  reading is +1 `E0609` and not +3.

### 70. Slice 57 (C 12542-12683): the selection-request queue and the drag-and-drop teardown

* **C's two struct assignments are the port's `copy_nonoverlapping`.**  `x_handle_pending_selection_requests_1`'s
  `se = tem->se` and `x_push_selection_request`'s `tem->se = *se` are whole-record copies, and
  `include/xterm.rs` 1476's `selection_input_event` carries no `Copy` (a C struct assignment is the
  port's `std::ptr::copy_nonoverlapping`, the `src/buffer.rs` 820 shape); the first writing used the
  plain `=` and drew `E0507: cannot move out of ... behind a raw pointer` twice.
* **Two locals are zero-initialised because C fills them in one branch** (Deviation 12): `se`
  (`struct selection_input_event se;` copied into on the next line) and `dmsg`
  (`xm_drop_start_message dmsg;` written only inside C 12650's `else if`).  The first writing left
  `dmsg` uninitialised and drew `E0381: partially assigned binding`; the `src/xterm.rs` 4928 site's
  `std::mem::zeroed ()` is the shape both now use.
* **C's `record_unwind_protect_ptr (xfree, tem)` names the allocator through the mirror's
  `xfree_wrapper`.**  `include/lisp.rs` 3073 is the function-pointer form of `xfree`, and the bare
  name cannot reappear - a `docs/ports/macro-body-notes.tsv` row records it (the same class as
  `x_next_event_from_any_display`'s `true`, which C's `while (true)` needs and the port's `loop`
  cannot carry; Decision 69's third item).
* **The reading is a net zero, and it is made of two movements.**  Each arm gains the two
  `XDeleteProperty` sites of `x_dnd_delete_action_list` (the bare Xlib class) and loses the two call
  sites the earlier slices owed (`x_push_selection_request`'s C 917 site and
  `x_handle_pending_selection_requests`'s C 958 site inside `x_release_selection_requests_and_flush`),
  so A/B/C read exactly what they read before the slice while eight ledger rows retire.
* **First-read defects, fixed in this slice.**  Besides the two `E0507`s and the `E0381`, the first
  read carried three `unused_mut` warnings (`count`/`tem`/`f` are assigned once in their bodies, so
  their bindings need no `mut`); the reading went 730 / 72 -> 727 / 69.

### 71. Slice 58 (C 12284-12342): `x_tooltip_window_to_frame`

* **C's `continue` inside a `USE_GTK` arm takes the inverted-guard spelling.**  PORTING.md 2.2 spells
  `continue` as a labelled block whose `break` lands before the port's `FOR_EACH_FRAME!` step, but
  C 12327/12335 of the same arm are `break`s out of the loop, and a bare `break` inside a labelled
  block is `E0695` - so C 12308's `if (!FRAME_X_P (f)) continue;` becomes the *positive* guard
  `if FRAME_X_P (f) { … }` around the arm's remaining statements, which keeps both `break`s the
  loop's and removes the label.  The first writing used the labelled block: arm A and C then carried
  an `unused label` warning (the label had no `break 'body` once the arm was gated out) and arm B
  carried the `E0695`, so both readings appear below rather than in the final table.
* **Two `struct x_output` fields and one GTK entry point are the recorded debt class.**  C's
  `FRAME_X_OUTPUT (f)->ttip_window` is one of the fields `include/xterm.rs` records as comments
  rather than storage (Decision 68 ②'s class), so the arm's two accesses are `E0609`; C 12323's
  `gdk_x11_window_get_xid` is the bare GTK class (`+1 E0425` in arm B), and the GTK2 arm's
  `GDK_WINDOW_XID` is the macro `include/xterm.rs` 1021-1026 records as text rather than defining -
  no arm here compiles it.
* **The reading's shape.**  Arms A and C gain nothing and lose the two call sites the earlier slices
  owed this name (unit 2977 and 4355), so both move -2; arm B compiles the GTK arm, so
  its `+2 E0609` and `+1 E0425` are offset by the same two resolutions and it moves +1 (746 -> 747).

### 72. Slice 59 (C 13979-14127): the focus dispatch, the mouse-leave hook and the rehighlight pair

* **The eight X11 event constants are measured, not guessed, and they are a mirror change.**
  `x_detect_focus_change`'s `switch` and `x_focus_changed`'s two `FocusIn`/`FocusOut` tests need
  `EnterNotify` (7), `LeaveNotify` (8), `FocusIn` (9), `FocusOut` (10), `NotifyGrab` (1),
  `NotifyUngrab` (2), `NotifyInferior` (2) and `NotifyPointer` (5); C gets them from `<X11/X.h>`
  lines 186-281, and `include/xlib.rs`'s `x11_constants` module now carries each with its header
  line and a clang measurement of this target's header (`/tmp/xconst.c`, the turn's probe).  The
  module is a mirror, so P-87 ①'s two legs ran: `checked 83 unit(s), 0 failure(s)` and
  `checking 83 unit(s), 0 unit failure(s)`.
* **C's `switch` is a `match` over those constants, and `x_focus_changed` now names them too.**
  The three case groups keep their order as three arms plus the `_` arm C's `break` leaves
  implicit (the `x_handle_net_wm_state` shape, with a `docs/ports/macro-body-notes.tsv` row for
  `switch`/`case`/`break`).  `x_focus_changed`'s two bare tests from slice 55 are respelled with
  the mirror's constants in this slice - the same two `E0425`s stop being owed.
* **Every `XEvent` member read is the placeholder class, and two display fields are owed.**
  `include/xlib.rs`'s `XEvent` is a 192-byte record with no members, so the dispatch's ten reads
  (`r#type` three times, `xcrossing` twice, `xfocus` three times, `xclient` twice) are `E0609` -
  the type is the placeholder, which is why the arm *patterns* had to be the mirror's constants
  rather than bare names (a bare name in a pattern position is a binding, and `X | Y` then draws
  `E0408`, the first writing's reading).  Arm C's `x_mouse_leave` additionally reads
  `dpyinfo->client_pointer_device` twice, a field this mirror records as owed (the C 861-885
  block), so arm C carries two more `E0609`s than A and B.
* **The first writing's three defects, all fixed here.**  The match's constants had no `use` in
  scope (the `E0408` above, and `x_focus_changed`'s two sites were still unresolved because the
  import was in the other function), and `hlinfo` was bound `mut` while assigned once (a new
  `unused_mut` warning).  The intermediate readings 745 / 70 (arm A) and the warning are recorded
  below rather than in the final table.

### 73. Slice 60 (C 14134-14303): `x_find_modifier_meanings`

* **Fourteen more measured constants, and they had to be in pattern position.**  The live path's
  `switch (sym)` matches on nine keysyms and the tail compares two modifier masks, so
  `include/xlib.rs`'s `x11_constants` gained `XK_Shift_Lock` 0xffe6, `XK_Meta_L` 0xffe7,
  `XK_Meta_R` 0xffe8, `XK_Alt_L` 0xffe9, `XK_Alt_R` 0xffea, `XK_Super_L` 0xffeb, `XK_Super_R`
  0xffec, `XK_Hyper_L` 0xffed, `XK_Hyper_R` 0xffee and `ShiftMask` 1, `LockMask` 2, `ControlMask`
  4, `Mod1Mask` 8, `Mod2Mask` 16 - each with its `<X11/keysymdef.h>` 396-405 or `<X11/X.h>`
  221-225 line and a clang measurement of this target's header.  Slice 59's lesson applies twice
  over: a bare name in a pattern position is a binding, so the first writing's missing `use` drew
  eight `E0408`s (the `XK_Meta_L | XK_Meta_R` arms' two variables each).  The mirror changed, so
  P-87 ①'s legs ran: `checked 83 unit(s), 0 failure(s)` and `checking 83 unit(s), 0 unit
  failure(s)`.
* **C's widths are stated at the comparison points.**  The mirror's `*_mod_mask` members are
  `c_uint` and its `KeyCode` is `c_uchar`, so the port writes `Mod1Mask as c_uint`,
  `(1 as c_uint) << row`, `((1 as c_uint) << row) == LockMask as c_uint` and
  `(code as c_int - min_code)`; C's own promotion does the same silently, and the first writing's
  unmixed widths drew three `E0308`s and three `E0277`s (`cannot subtract i32 from u8`).
* **C's `continue` is the labelled block here, unlike slice 58.**  C 14224's `continue` sits in the
  `col` loop whose body has no other `break` (the `switch`'s ten `break`s became `match` arms), so
  PORTING.md 2.2's labelled block is legal and `break 'body` is the jump; the two `for` loops' steps
  stand at their bodies' ends.
* **C's XKB region is carried and dead.**  C 14141-14145 and 14153-14192 sit under `HAVE_XKB`, which
  no arm here sets, so `xkb_desc` (a mirror-owed field), `XkbNumVirtualMods`, `vmods` and `names`
  stay unmeasured text; the live path's five bare Xlib names are `XDisplayKeycodes`,
  `XGetKeyboardMapping`, `XGetModifierMapping`, `XFree` and `XFreeModifiermap` (the last two are
  already owed at earlier sites), and the slice resolves `x_find_modifier_meanings`'s own call site
  in `mark_xterm` (unit 20409, owed since slice 52).

### 74. Slice 61 (C 14308-14380): the two modifier converters and `get_keysym_name`

* **C's widths are stated at every mixed-width operation.**  `x_x_to_emacs_modifiers` takes an
  `int state` and tests it against the `unsigned int` members and the `unsigned long` masks, so the
  port's tests all run at `c_uint` (`state as c_uint`, `ShiftMask as c_uint`) and the OR chain ends
  in `as c_int`; the `XFIXNUM`-against-`INT_MAX` masks keep C's own promotion
  (`(XFIXNUM (tem) & INT_MAX as EMACS_INT) as c_int`).  `x_emacs_to_x_modifiers` takes an `intmax_t`
  while its locals are `EMACS_INT`, which is `isize` in the port and therefore a *different* type
  from `intmax_t` even where the two are the same width - the first writing's five `state & mod_*`
  tests drew `E0277: no implementation for i64 & isize` each, and each now states
  `mod_x as intmax_t`.
* **The converters' `?:` chains are the port's `if` expressions.**  PORTING.md 2.2's shape, one term
  per C line so the six alternatives stay in C's order; the third function is the `block_input` /
  `XKeysymToString` / `unblock_input` triple with the bare Xlib class.
* **The slice moves one error per arm and nothing else.**  All three names had no *other* call site
  in the port (their callers are the unlanded event readers), so the reading is exactly
  `XKeysymToString`'s `E0425`: +1 in A, B and C.  No mirror changed, so P-87's legs do not apply.

### 75. Slice 62 (C 14382-14536): the root-window offset cache and the two coordinate translators

* **C's `FRAME_X_OUTPUT (f)` is spelled at the use site in all four functions.**  C 14392-14394,
  14410/14412/14415-14416/14433-14435, 14450/14452/14455-14456/14470-14472 and
  14490/14493-14494 all go through the header's macro; the port writes
  `(*( (*f).output_data.x as *mut x_output )).field` (P-72 rule 3's use-site shape - the file's
  dominant spelling, 161 sites after this slice), because the mirror's macro hands back the
  untyped union member while the four fields *are* mirrored (`include/xterm.rs` 914-916:
  `window_offset_certain_p`, `root_x`, `root_y`).  Four `docs/ports/macro-body-notes.tsv` rows
  carry the reason, one per function, so the token row extract-abi raises for each is closed by
  a reading rather than by silence.
* **C 14429's and C 14466's comma operators are two statements.**  `*x_out = 0, *y_out = 0;` has
  no Rust spelling (PORTING.md 2.2, the `x_dnd_get_wm_state_and_proto` note's class); the port
  keeps C's order on two lines and the C line number stays on the first, so the `--check`
  reader still sees the block.
* **The XI2 trio: the same arm as `xi_convert_event_state`, and three types with no mirror row.**
  C gates them on the file's `HAVE_X_WINDOWS` plus `#ifdef HAVE_XINPUT2` (C 14503) and, for the
  pinch event, the nested `#ifdef HAVE_XINPUT2_4` (C 14521), so the port's arms are
  `all (feature = "have_x_windows", feature = "have_xinput2")` and the pinch one adds
  `feature = "have_xinput2_4"` - the pair `xi_convert_event_state` (this file 5551) already
  carries, and the nested pair the `src/xwidget.rs` lane's X arm uses.  All three are C's
  `static` and the reference's inventory has **no entry** for any of them (the frozen table
  leaves the whole span `undef`), so they stay plain `unsafe fn` rather than exported shapes.
  Their three parameter types - `XIDeviceEvent`, `XIEnterEvent`, `XIGesturePinchEvent` - have no
  mirror row in this graph (Decision 15's list of XInput2 names kept bare), so each is the
  registered bare-name `E0425` class in a `have_xinput2` arm; the argument expressions are C's
  own implicit `double` -> `int` truncation, written `as c_int` (the C comment beside them says
  "truncate coordinates instead of rounding them").
* **The slice's own call sites: four `E0381` and two `unused_mut` warnings, both on *old* text.**
  The first writing gave `x_translate_coordinates`/`x_translate_coordinates_to_root` parameters
  that the two callers above (C 4937's `x_dnd_note_self_position` and C 4958's
  `x_dnd_note_self_wheel`, landed in slice 57) could not satisfy: each had
  `let mut f: *mut frame;` plus an uninitialised `dest_x`/`dest_y` pair that only the old
  unresolved-callee path left alone, so the arm read **738 / 71** with four `E0381`s
  (`_root.rs:5084` and `5112`, two each) and two `variable does not need to be mutable`
  warnings.  Both are Deviation 12's shape (`let f: *mut frame;`, `= 0` on the two locals) and
  the reading below is the fixed one: the two functions' parameters now *are* the C ones, so the
  old call sites turn live.  Nothing else in the file moved.
* **What moved, per arm.**  Three `E0425` sites retire (the three `x_translate_coordinates`
  call sites the earlier slices owed) and three arrive (C 14423 and C 14463's bare
  `XTranslateCoordinates`, plus one `XIDeviceEvent`/`XIEnterEvent`/`XIGesturePinchEvent`
  parameter type per wrapper), so A and B read net -1 and C reads net +1; the fourth arm (the
  xwidget lane's `have_xinput2_4` X arm) reads net +2 because its pinch wrapper is live.

### 76. Slice 63 (C 14536-14634): the two pointer queries, and C's `#endif` inside an `if`

* **A C local block that `#ifdef` gates becomes seven cfg'd `let`s, in C's order.**  C 14546-14553
  declares `bool had_errors`, `XIModifierState modifiers`, `XIButtonState buttons`,
  `XIGroupState group`, the four `double` coordinates and `unsigned int state` *before* the
  `dpy = dpyinfo->display;` line (C 14555), so the port writes each declaration where C has it,
  under `#[cfg (feature = "have_xinput2")]` (the `src/haikufns.rs` 3352 precedent - C's own
  `#ifdef` named in a comment just above the group).  C's four declarators on one line are four
  `let`s; the FFI-out locals C only declares are `std::mem::zeroed ()` (Deviation 12), while
  `state` (assigned at C 14588 before any use) and `had_errors` (assigned at C 14568) stay bare
  `let mut ...;` - the arm reads no `E0381` either way, measured.
* **C's `#endif` sits *inside* the `if`/`else` (C 14557-14601).**  The statement C 14602-14605 is
  the `else` branch when `HAVE_XINPUT2` is defined and a plain statement when it is not, so the
  port writes it twice: once inside the `feature = "have_xinput2"` block's `else`, and once under
  `not (feature = "have_xinput2")` (Decision 48's arm-continuation shape, this file's second
  instance - the `x_make_frame_invisible` pair was the first).  Both copies carry C 14602-14605's
  citation, and both are blocks: `#[cfg]` on a bare expression statement is `E0658` (slice 53's
  lesson).
* **The XI2 names and `lrint` are the registered bare-name class.**  `XIQueryPointer`,
  `XQueryPointer`, `XIModifierState`, `XIButtonState` and `XIGroupState` have no mirror row
  (Decision 15's list of XInput2 names kept bare), and `lrint` is `<math.h>`'s rounding function,
  which no mirror in this graph carries either (the `xi_link_touch_point` note, this file 6055) -
  so the arm readings below carry them as `E0425` sites, not as silence.
  `x_catch_errors`/`x_had_errors_p`/`x_uncatch_errors_after_check` (xterm.h 1752/1762/1765, the
  mirror slices 51 landed) and `XFree` (this unit's own `extern "C"` block, this file 14962) all
  resolve, as do `xi_convert_button_state` (this file 5525) and `x_dpyinfo` (this file 7390).
* **`client_pointer_device` is an owed `x_display_info` member.**  `include/xterm.rs` 665 names it
  in the mirror's owed list, so C 14564/14578/14579/14619 are the registered `E0609` ("no field")
  class - four sites, and the only `E0609` movement in either `have_xinput2` arm.
* **Widths.**  C's `-1` argument (C 14604) stays an `i32` literal against the `c_int` parameter, and
  `lrint (x)` results are written `as c_int` (C's implicit `long` -> `int` truncation, the
  `xi_link_touch_point` spelling); `state | modifiers.effective` stays at `c_uint`.

### 77. Slice 64 (C 14630-14719): `x_construct_mouse_click`

* **C's two comment blocks are carried, not summarised.**  C 14630-14655 (`Mouse clicks and mouse
  movement.  Rah.`) and C 14657-14665 (`Prepare a mouse-event in *RESULT ...`) are the reference's
  prose for the block that begins here, so the port keeps them verbatim above the function (the
  `Line Dance` banner's precedent, and the reason slice 65's region starts after them).
* **The two `?:` chains are `if` expressions, C's line per term.**  PORTING.md 2.2's shape, as in the
  two converters: the wheel-kind chain (C 14690-14696) is `if`/`else if`/`else` with C's five terms
  in C's order, and the `NO_EVENT`/`HORIZ_WHEEL_EVENT`/`WHEEL_EVENT` chain (C 14699-14700) is
  nested inside the `input_event_bits!` argument.
* **C's parentheses around a `?:` initialiser are dropped, and that is a recorded deviation.**
  `int wheel = (NILP (base) ? -1 : ... );` keeps C's parens in the first writing, and rustc's
  `unused_parens` fires on them (`unnecessary parentheses around assigned value`, `_root.rs:22897`)
  - arm A read 70 warnings against the baseline's 69.  The port drops the pair and the reading is
  back to the baseline; C's other parenthesised initialisers in this unit (e.g. the `|=` at C 30404)
  keep theirs because Rust needs them there.
* **The bitfield write goes through this unit's accessor.**  C's `result->kind = ...` is the mirror's
  `input_event::bits` unit (termhooks.h 373-412's `kind : 16`), so both writes are
  `input_event_bits! (kind, result, ...)` (PORTING.md P-80, the C 893 macro this unit already has);
  `XSETINT`/`XSETFRAME` are the mirrors' macros and take `EMACS_INT` (`x as EMACS_INT`), and
  `Qnil`/`Qmouse_wheel_buttons`/`Qwheel_*` are the port's Lisp-symbol macros.
* **Widths, and the names that stay bare.**  `result->modifiers` is C's `unsigned int`, so the two
  compound assignments state it (`!((up_modifier | down_modifier) as c_uint)` - C's `~(int)` and this
  are the same value - and `(if wheel & 1 {...} else {...}) as c_uint`), while
  `x_x_to_emacs_modifiers`' result is C's `int` and takes `event->state as c_int`.  `XButtonEvent`,
  `Button1` and `ButtonRelease` are X11 names the mirror's `x11_constants` does not carry, so they
  stay bare (Decision 15's ruling) - the four new `E0425` sites of every arm below.

### 78. Slice 65 (C 14738-14857): the mouse-movement noter and the sibling search

* **`frame->mouse_moved` gets its arm in this unit's existing accessor.**  C 14757/14772 write
  `frame.h` 434's `bool_bf mouse_moved : 1`, which is the unit of `pointer_invisible` and
  `auto_raise`/`auto_lower` that `frame_bits!` already covers (this file 20978, its
  `FRAME_POINTER_INVISIBLE_SHIFT` reads), so the slice adds a `(mouse_moved, $f, $v)` arm over
  `include/frame.rs`'s measured `FRAME_MOUSE_MOVED_SHIFT` (frozen 1:18, A 1:21, B 1:22, C 1:21 -
  the same word as the other three, `_UNIT = 1`).  The two writes are
  `frame_bits! (mouse_moved, frame, true)`.
* **C 14745's `!FRAME_X_OUTPUT (frame)` is the header macro's pointer, tested for null.**  The port
  spells `FRAME_X_OUTPUT! (frame).is_null ()` (the mirror's `output_data.x` is `*mut c_void`, P-72
  rule 3; the `src/pgtkterm.rs` 8584 shape) and `dpyinfo->last_mouse_scroll_bar = NULL` /
  `last_mouse_glyph_frame = NULL` are `std::ptr::null_mut ()` - all three fields *are* mirrored.
* **C's integer promotion of the glyph rectangle is written out.**  `XRectangle`'s `x`/`y` are
  `short` and its `width`/`height` `unsigned short` (`include/xlib.rs`), so C 14769-14770's four
  tests become `(*event).x < (*r).x as c_int`, `(*event).x >= (*r).x as c_int + (*r).width as
  c_int` and the same pair for `y` - the `src/androidterm.rs` 753-754 shape, with the two
  conversions made explicit because the mirror's members are not `int`.
* **Two declarations at the use site, and the fields C reads through a bare type.**
  `remember_mouse_glyph` (dispextern.h 3586, whose `NativeRectangle` is `XRectangle` on this
  target) and `note_mouse_highlight` (dispextern.h 3688, which `include/dispextern.rs`'s
  `dispextern_late_decls` records as *owed* rather than declared) are declared at this unit's scope
  (this file 14966-14978, next to the `XFree` block; slice 66 moved them there from this body,
  because `x_fast_mouse_position` is the second caller and a body-local declaration is invisible in
  it); `XMotionEvent` and `XWindowAttributes` are X11 records no mirror in this graph carries, so
  they stay bare (Decision 15's ruling).
* **C's reversed `for` is the port's `while`, and its start value states C's unsigned arithmetic.**
  `for (i = nchildren - 1; i >= 0; --i)` (C 14817) is `i = nchildren.wrapping_sub (1) as c_int;`
  followed by the `while` with the step at the body's end (the `x_find_modifier_meanings` shape);
  `wrapping_sub` is required because C's subtraction is *unsigned* and then converted, so a plain
  `- 1` would panic on the `nchildren == 0` path where C reads `-1` and skips the loop.  The three
  `continue`s (C 14822/14826/14831) are the labelled block's `break 'body` (PORTING.md 2.2 - the
  step follows the body) and the loop's own `break` (C 14842) is `break 'children`: a *bare*
  `break` inside a labelled block is `E0695`, which the first writing measured.
* **`XFree (children)` states C's implicit conversion.**  `children` is `Window *` and XFree takes
  `void *`, so the two calls are `XFree (children as *mut c_void)` - the first writing's two
  `E0308` (`expected *mut c_void, found *mut u64`).

### 79. Slice 66 (C 14859-14978): `x_fast_mouse_position`, and the declaration that resolved a macro's call

* **C's `FOR_EACH_FRAME` is the mirror's macro, and this unit's `mouse_moved` arm does the writes.**
  C 14890-14896 and C 14921-14926 are two `FOR_EACH_FRAME` uses (`include/frame.rs` 800), and C's
  three `->mouse_moved = false` writes (C 14895 and C 14975) are this unit's
  `frame_bits! (mouse_moved, ...)` - the arm slice 65 added, so one shape covers them.
  `XFRAME`/`FRAME_X_P`/`FRAME_TOOLTIP_P`/`SELECTED_FRAME`/`FRAME_PIXEL_WIDTH`/`FRAME_PIXEL_HEIGHT`
  are the mirror's functions; `FRAME_DISPLAY_INFO!`/`FRAME_X_WINDOW!` are its macros.
* **The two scroll-bar reporters are unlanded `static`s, so their four call sites are the
  registered `E0425` class.**  C 1173-1180 forward-declares them (this file 970's note) and C
  17638/17707 defines them; Rust resolves items by name, so the port calls them by name and the arm
  reading counts them (`x_scroll_bar_report_motion`, `x_horizontal_scroll_bar_report_motion`).
  `gui_mouse_grabbed` (dispextern.h 4006, `Display_Info` = dispextern.h 115's `x_display_info`) has
  no mirror row either, so this body declares it (P-88).
* **C's `int` and pointer tests are written as the comparisons they are.**  `!insist` (C 14876) is
  `insist == 0`, `!FRAME_X_WINDOW (f1)` (C 14931) is `FRAME_X_WINDOW! (f1) != 0`, `!x_query_pointer
  (...)` (C 14933) is `... == 0`, and the `NULL` assignments (C 14929, C 14953, the two scroll-bar
  ones in the same shape) are `std::ptr::null_mut ()`.
* **`XSETINT (*x, win_x)` keeps C's lvalue shape.**  The mirror's macro is `$a = make_fixnum ($b)`,
  so the two calls are `XSETINT! (*x, win_x as EMACS_INT)` / `XSETINT! (*y, win_y as EMACS_INT)`,
  and C 14971-14972's pair are `make_fixnum ((*dpyinfo).last_mouse_motion_x as EMACS_INT)` - the
  `int` -> `EMACS_INT` widening stated, because the mirror's field is `int`.
* **The declarations moved from a body to this unit's scope, and that resolved a macro's call.**
  `remember_mouse_glyph` and `note_mouse_highlight` stood inside `x_note_mouse_movement`'s body;
  a body-local `extern` block is invisible in the next function, and this slice is that next
  function (measured: `cannot find function remember_mouse_glyph`).  They now stand in this unit's
  block next to `XFree`'s (this file 14966-14978).  The second effect is the interesting one: with
  `note_mouse_highlight` resolvable, the call inside `include/frame.rs`'s `FRAME_MOUSE_UPDATE!`
  stopped being an *unresolved name* and became a *typed* call, and its first argument
  (`Mouse_HLInfo::mouse_face_mouse_frame`, the untyped pointer five units share) then failed with
  `error[E0308]` (expected `*mut frame`, found `*mut c_void`).  That defect is fixed where it
  belongs - the mirror casts the argument exactly as its neighbouring comparison already did (the
  2026-10-07 paragraph in `docs/ports/include-frame.md`) - and the E0308 class reads 13 in A/B and
  13 in C/X before and after, i.e. the cast removed exactly the one error the declaration had
  uncovered.  The declaration also retires one *old* site: the macro's `note_mouse_highlight` call
  was counted as an unresolved `E0425` in every earlier reading of this unit (it is in the baseline
  the table below quotes), so the slice's net movement is two new call sites minus that resolution.

### 80. Slice 67 (C 14980-15296): `XTmouse_position`, and the one C name that has two port shapes

* **`x_any_window_to_frame` is a function in one arm and a macro in the other, and the call site splits
  with it.**  C 12344's `#if defined (USE_X_TOOLKIT) || defined (USE_GTK)` holds the *function*
  (this file 21460) and C 12486's `#else` holds `#define x_any_window_to_frame(d, i)
  x_window_to_frame (d, i)` (this file 21654; slice 56 landed both).  C 15184's one call is inside C
  15178-15185's `#ifdef USE_GTK`/`#else`, so under `not use_gtk` the port must pick the shape the
  other two features leave: the function when `feature = "use_x_toolkit"`, the macro otherwise -
  written as two cfg'd blocks with C's line on each.  The first writing called the *function* form in
  both, and rustc said so precisely: `error[E0423]: cannot find function x_any_window_to_frame in
  this scope` with the note "a macro named `x_any_window_to_frame` exists in another namespace".
  Arm T below is the arm that compiles the first branch, so both shapes are measured, not assumed.
* **`win as c_int` is written at all four window-to-frame calls.**  C converts `Window` to the
  `int wdesc` implicitly (C 15140, 15152, 15181, 15184); the mirror's functions take `c_int`, so the
  port states the conversion - the same spelling the file already uses at C 3184 (this file 2234).
  The first writing's five `E0308` (`expected *mut c_void, found *mut _XDisplay` at the mirror's
  `xm_*` pair is pre-existing; the new one was at the macro expansion) are gone.
* **C's `while (true)` is `loop`, and the toolkit/GTK arms are cfg'd blocks.**  C 15110's
  `while (true)` (the file's `x_next_event_from_any_display` shape), C 15137-15144's and
  C 15147-15156's `#ifdef USE_GTK` blocks (the first declares `struct frame *f` inside the
  `if`, so its `let f` is block-scoped as C's is), C 15162-15165's `first_win` bail-out, and
  C 15187-15194's `#ifdef USE_X_TOOLKIT` menu-bar test - whose `f1->output_data.x->menubar_widget`
  is the file's dominant cast spelling and whose `XtWindow` is a bare Xt name.
* **The two `struct scroll_bar *bar` are two `let`s, and `*part = 0` is C's own zero.**  C declares
  one `bar` per block (C 15031, C 15256); the `part` writes at C 15238/15286 are `0`, not
  `scroll_bar_nowhere`, so the port keeps `*part = 0;` (the constant would be a different word of C's
  text).
* **`unrelated_tooltip` is `false`-initialised.**  C declares `bool unrelated_tooltip;` (C 15007) and
  passes it to `x_tooltip_window_to_frame`'s out-parameter, which writes `false` first and `true`
  where it finds a tooltip frame (this file 21948-21949); Rust needs a value before the read, and
  `false` is the value the callee guarantees (Deviation 12's class, `PORTING.md` 1.5).

### 81. Slice 68 (C 15306-15401): the scroll-bar window search, and two functions C's own gates hide

* **C's `for` header with a comma expression becomes a `while` with a block operand.**  C 15361-15367
  is one of the file's two such loops:
  `for (bar = FRAME_SCROLL_BARS (XFRAME (frame)); ! NILP (bar) || (bar = condemned, condemned =
  Qnil, ! NILP (bar)); bar = XSCROLL_BAR (bar)->next)`.  The port keeps C's order as a `while` whose
  condition is `!NILP (bar) || { bar = condemned; condemned = Qnil (); !NILP (bar) }` - the comma
  expression is a block expression in the `||`'s right operand, and Rust's `||` short-circuits
  exactly as C's does here (the right side runs only while `bar` is nil, which is the "trick" C's
  own comment describes), with the `for`'s step at the body's end.
* **C 15356's `continue` is `break 'body` inside the macro's block.**  `FOR_EACH_FRAME!`'s own step
  follows the body (`include/frame.rs` 800), so a bare `continue` would skip it; the notes row
  carries the reason for the retired token.
* **`type` is `type_`, and `XSCROLL_BAR` needs its bang.**  C's parameter `int type` (C 15343) is
  `type_: c_int` (the `x_focus_changed` row at this file 21297), and the three reads go through this
  file's own `XSCROLL_BAR!` macro (`include/xterm.rs` 1336).
* **Both `#if`-hidden functions are carried, and neither is measured: their gates are `undef`
  everywhere this lane compiles.**  `xi_select_scroll_bar_events` (C 15312-15333) sits inside C's
  `#if defined HAVE_XINPUT2` and is `MAYBE_UNUSED static`; `x_window_to_menu_bar` (C 15385-15399)
  sits inside C 15380's `#if defined USE_LUCID`.  P-86 ① asks for both and both are here as full
  definitions with the feature pairs those macros lowercase to.  The readings say what that buys: in
  arms C/X the XI2 function is live and contributes fifteen sites; the Lucid one is live in **no**
  arm of this lane (`use_lucid` is in no arm's `--cfg` set), so its body is text this round cannot
  compile - the "text-only" state P-86 ③ ⑵ names for a toolchain the machine does not have.
* **The XI2 function's `alloca` is the stack pool, and `mask_len` keeps C's implicit narrowing.**
  `USE_SAFE_ALLOCA!`/`SAFE_ALLOCA!` replace C 15321's `alloca` (P-05, the
  `xi_select_hierarchy_events` shape at this file 19620), C 15321's chained
  `mask.mask = m = alloca (length)` splits into two statements where C's own comment is, and
  `mask.mask_len = length;` keeps C's implicit `ptrdiff_t` -> `int`: the mirror has no `XIEventMask`
  row, so the field's type is a bare name's and the port cannot state a width it has no measurement
  for - recorded rather than guessed.
* **`xlwmenu_window_p` is lwlib's `int` function, so C's truth test becomes `!= 0`.**  xlwmenu.h 40's
  `extern int xlwmenu_window_p (Widget w, Window window);` has no mirror in this graph, so the call
  is a bare name; C's `menu_bar && xlwmenu_window_p (menu_bar, window)` is the null test plus that
  `int` test (the `!insist`/`!FRAME_X_WINDOW` class this lane already writes).

### 82. Slice 69 (C 15403-15426): opening the toolkit scroll-bar region, and the two arms it needs

* **The region is gated by a macro no reading this lane had ever turned on, so two arms are added.**
  C 15408's `#ifdef USE_TOOLKIT_SCROLL_BARS` opens a span of more than 300 lines; the frozen table
  leaves it `undef`, so `extract-abi` never asks for these functions and the five arms the lane has
  been running (A/B/C/X/T) compile the whole span as **dead text** - the state P-86 ③ ⑵ records as
  "text-only" when a toolchain is missing, except here the toolchain is not missing at all: it is a
  `--cfg` flag.  So this round adds
  **U = `have_x_windows` + `use_toolkit_scroll_bars`** (C 15421's `#else` arm, `last_user_time`) and
  **UT = U + `use_x_toolkit`** (C 15421's `#ifdef USE_X_TOOLKIT` arm,
  `XtLastTimestampProcessed`), and the table below quotes their first baselines.  The same pair will
  measure every slice of this region from here on, which is why the slice that lands the *first* two
  declarations also lands the arms.
* **C's prototype is not carried, and the reason is this file's own.**  C 15410-15411's
  `static void x_send_scroll_bar_event (Lisp_Object, enum scroll_bar_part, int, int, bool);` is a
  prototype; the port writes definitions where C writes them (this file's C 1162-1210 note), so the
  declaration holds nothing and the definition lands with the next slice.
* **The file-scope object is declared where C declares it.**  `window_being_scrolled` (C 15416) is
  C's tentative `static Lisp_Object`, i.e. zero, which `NIL_IS_ZERO` makes nil - the port writes
  `lisp_h_Qnil ()` with the `(tentative, i.e. nil)` marker the later file-scope objects already use
  (this file 19475's `x_initialized`), at its C position rather than in the file's early globals
  block (`xterm_globals` holds C 807-1548's objects only; C 26755's `current_display` set that
  precedent).
* **`x_get_last_toolkit_time` is C's `#ifdef`/`#else` pair as two cfg'd `return` blocks.**  Each arm
  is its own block (a bare `#[cfg]` on an expression statement is `E0658`, slice 53's lesson), and
  the two arms are exactly what separates U from UT: arm U reads the mirror's `last_user_time` field
  and adds **no** site, arm UT calls the bare `XtLastTimestampProcessed` and adds exactly one.

### 83. Slice 70 (C 15428-15536): the two Xt action hooks, and the aliases that made an earlier note false

* **Three Xt typedefs join the file's own Xt block, and one of them corrects a false claim.**  The block
  at this file 10040 said "`XtPointer`/`Boolean` *are* mirrored (`include/widget.rs` 182-185)"; measured,
  **no** `femacs/src/include` file declares either (`rg 'pub type (XtPointer|Boolean)'` finds none) - the
  sentence was wrong, and nothing had exercised it.  The slice adds `Boolean` (`c_int`) and `XtPointer`
  (`*mut c_char`) with the citations `src/widget.rs` 168-170 and 185 already use for the same
  `<X11/Intrinsic.h>` typedefs, plus `XtActionHookId` (`typedef XtPointer XtActionHookId;`), and rewrites
  the note to say what was measured.  Arm UT moves accordingly: **thirteen** pre-existing unresolved-type
  sites (`XtPointer` x6, `Boolean` x5, and two `int`-typed uses) resolve, and the E0308/E0606 classes stay
  where they were once the three casts below are fixed.
* **Three casts the aliases exposed, fixed where C's own values are.**  `*closure_ret = (XtPointer) False;`
  (C 9013/9018) and `(XtPointer) True` (C 9025) were written `False as XtPointer` / `true as XtPointer` in
  the landed `use_x_toolkit` text - legal only while `XtPointer` was an unresolved *type*.  With the alias
  in place rustc reads them as `bool as *mut c_char` (`E0606`: casting bool as *mut i8 is invalid), so the
  two `False`s are `std::ptr::null_mut ()` (C's `(XtPointer) False` is the null pointer) and the `True` is
  `1 as XtPointer` (C casts the integer literal).  Arm UT's `E0606` returns to its baseline count of 1.
* **Both hooks are C's `#ifndef USE_GTK` text, and their arms are measured by UT.**  C 15458-15464's and
  C 15501-15507's `#ifdef USE_MOTIF` pairs are two cfg'd blocks each; the two locals C declares
  uninitialised (`bool scroll_bar_p`, `const char *end_action`) are the file's first pair whose *both*
  branches assign, so the port writes them uninitialised at the top as C does and lets the blocks
  initialise them - no `E0381` in either arm, measured.  `XmIsScrollBar`/`XtIsSubclass` return Xt's
  `Boolean` (`int`), so the `bool` assignments state `!= 0`, and the two action names are the port's
  `c"..."` literals.
* **U is not a configuration C can build, and the slice says so instead of hiding it.**  Arm U
  (`use_toolkit_scroll_bars` **without** `use_x_toolkit`) compiles these hooks, but C's `XtPointer`/
  `String`/`Cardinal` come from `<X11/Intrinsic.h>`, and C 15416's sibling C 835
  (`static bool toolkit_scroll_bar_interaction;`) sits inside C 828's `#ifdef USE_X_TOOLKIT` - so in a
  U-shaped configuration C's own text would not compile either.  U's reading carries that property
  (two `E0308`s where the port's `String` falls back to the prelude's, and two unresolved
  `toolkit_scroll_bar_interaction` sites); UT is the arm this region's evidence comes from, and both are
  quoted so the difference is visible rather than asserted.

### 84. Slice 71 (C 15539-15696): the scroll-bar event pair, the protected-window queue and the unit's `static_assert`

* **C's `static_assert` comes from the mirror, and `check-mirror-dupes` is what said so.**  C 15587's
  `static_assert (INTPTR_WIDTH <= 64);` is the file's first C23 `static_assert`; PORTING.md 756-757
  describes a *per-unit* `macro_rules! static_assert`, so the first writing declared one here - and
  `tools/check-mirror-dupes.py` read **17 duplicate declaration(s), 16 ratchet row(s), 1 failure(s)**
  with `FAIL femacs/src/xterm.rs: static_assert is already declared by femacs/lib/include/verify.rs
  (in the include graph)`.  That is the right answer: `lib/include/verify.rs` 148 already carries the
  macro (both the one- and two-argument spellings), it reaches this unit through `lisp.h`, and the
  copy was deleted - arm U's reading is **identical** with and without it (795 / 69), which is how the
  resolution is proven rather than assumed.  `INTPTR_WIDTH` is real too, not bare:
  `lib/include/stdint.in.rs` 325 measures it from `INTPTR_MIN`/`INTPTR_MAX`, and the const assertion
  passes in the arms that compile it.
* **The protected-window queue's three members are owed by the mirror, so the whole queue is the
  registered `E0609` class.**  C 936-938 declare `Lisp_Object *protected_windows`, `int
  n_protected_windows` and `int protected_windows_max`; `include/xterm.rs` carries none of them (the
  `mark_xterm` row already reads two), so C 15546-15550, 15560-15568 produce nineteen `E0609` sites in
  the two arms.  C 936's member is a **pointer**, which is why C 15550's index is written
  `*…protected_windows.offset (n as isize)` (the spelling `mark_xterm` uses at C 32459) rather than an
  array subscript, and C 15550's post-increment is the load-then-step pair - the index is the old
  count, the increment follows on its own line.
* **C's `?:` inside an assignment keeps its parentheses dropped, and that is measured.**  C 15597-15599
  writes `ev->message_type = (horizontal ? … : …);`; carrying C's parentheses into the port draws
  rustc's `unused_parens` (`unnecessary parentheses around assigned value`) - the first writing read
  70 warnings in arm U against the baseline's 69, and the slice-64 precedent applies: the pair is
  dropped and the counts return to baseline.  C's other parenthesised initialisers in this file keep
  theirs where Rust needs them.
* **`XEvent`'s placeholder is why the event writes are one `E0609` each.**  `include/xlib.rs` 1341's
  `XEvent` is a 192-byte `_private` array with its 33 members owed (P-72 rule 3), so
  `&raw mut event.xclient` is one site, and because the resulting `ev` is error-typed every `ev->…`
  write after it is suppressed - the same shape C 15584's pointer has in the `mark_xterm` rows.
  The two `to_input_event` twins read `event->xclient.data.l[…]` directly (six sites between them),
  and `XClientMessageEvent`/`XSendEvent` are bare names.
* **The two `to_input_event` twins use the mirror's accessors, not C's field writes.**  C 15652 and
  C 15687 write `ievent->kind`, and C 15657/15692 write `ievent->part`; both are halves of the
  mirror's single `bits` unit (`termhooks.h` 373-412), so all four go through this unit's
  `input_event_bits!` macro (PORTING.md P-80), while `EVENT_INIT (*ievent)` is the mirror's own
  function (`include/termhooks.rs` 135) and the two constants are `include/termhooks.rs`'s.

### 85. Slice 72 (C 15699-15786): the Motif scroll callback, and the placeholder policy a missing header forces

* **The host has no Motif headers, so the Motif names are placeholders with named gaps - the same
  policy `include/gtk.rs` already carries for GTK.**  `/opt/homebrew/include/X11` holds Xlib's headers
  and nothing else: there is no `<Xm/ScrollBar.h>`, no libXt, no gtk-3.0.  So `XM_SB_MAX` (C 15703,
  whose value *is* C's own text: 10000000) is measured and asserted, while the three Motif artifacts
  are not: the eight `XmCR_*` reasons are written 1..8 with `**UNVERIFIED**` in their doc comment and
  `// placeholder, not Motif's value`-equivalent wording, `XmScrollBarCallbackStruct` is the header's
  published layout with no size or offset assertion, and `XmNsliderSize ()` is the resource-name
  function the `XtNheight ()` row above already models, with its string marked as a placeholder.  The
  ledger's Owed item 0 names `<Xm/ScrollBar.h>` as what would close all three - the P-86 ③ ⑵ rule
  ("say which toolchain is missing") applied to a *region* rather than a file.
* **A fourth arm joins the lane: UM.**  `USE_MOTIF` implies `USE_X_TOOLKIT` in C (the region's whole
  API is Xt's), so the arm this block needs is `have_x_windows` + `use_toolkit_scroll_bars` +
  `use_x_toolkit` + `use_motif` - slices 69/70's U and UT would each leave half of the text dead.
  Its first baseline, 835 / 68, is quoted in the table below.
* **C's `switch` over the eight reasons is the port's `match`, `_` arm for the implicit `break`.**
  C's eight `case` groups are the eight arms in C's order; `XmCR_VALUE_CHANGED` is C's empty group
  (`break` alone) and the `_` arm is the `break` C leaves implicit (the `x_handle_net_wm_state` row's
  shape).  The `switch`/`case`/`break` tokens are the notes-file's class, and the six
  `horizontal ? left : up`-style conversions become `if` expressions (PORTING.md 2.2).
* **C's `float` arithmetic states both operands.**  C 15760-15761 mix `int` and `float` and assign to
  `int`; the port writes `f32` (PORTING.md 590) on every operand and `as c_int` on the result, and
  C 15762/15768's `min` is `include/lisp.rs` 3152's `min (isize, isize)`, so the two arguments state
  `as isize` and the result `as c_int` - this file's existing `min` call shape (C 4632).
* **Two defects the arm measured, both fixed.**  `XtVaGetValues`' variadic `NULL` sentinel cannot be
  a bare `std::ptr::null_mut ()` (rustc: `E0283`, "cannot infer type of the type parameter `T`"), so
  it is `std::ptr::null_mut::<c_void> ()` - the spelling the existing call site at C 27553 uses; and
  C 15752's `int slider_size;` is written by the variadic call, so Rust needs the value C's own
  convention leaves to it (`= 0`, PORTING.md 1.5) or `E0381` fires, measured.

### 86. Slice 73 (C 15788-15899): the GTK scroll callbacks, and the gate the new arm caught

* **The GTK arm mirrors the pgtk lane's twin, which had already solved the same three problems.**
  `src/pgtkterm.rs` 5743-5797 landed `pgtkterm.c`'s `xg_scroll_callback` with no GTK tree on this
  machine either, so this slice reuses its rulings verbatim: `GtkRange`/`GtkAdjustment` are opaque
  `_private: [u8; 0]` records, the three `gtk_*` calls are declared with the header alone as the
  citation, `GtkScrollType` is `c_int` with five **placeholder** labels, `gdouble` stands beside them
  (the mirror has `gchar`/`gint`/`gboolean`/`gpointer`/`guint32` and not that row - the pgtk lane's
  Owed 11 names the same gap), and C 15811's `GTK_ADJUSTMENT (…)` needs **no counterpart** because
  the declared return type is already the record (P-72 rule 3).  Two spellings are this slice's own
  additions to that set because xterm.c's twin has code pgtkterm.c's does not:
  `GDK_CURRENT_TIME` (C 15816's test, a placeholder) and `xg_ignore_gtk_scrollbar` (C 15805, the
  extern whose definition `gtkutil.c` 240 owns and which is not landed).
* **One name in that set is *not* a placeholder.**  `XG_FRAME_DATA` (C 15812) is `gtkutil.h` 46's
  `#define XG_FRAME_DATA "emacs_frame"` - the header's own text, so it is written with its measured
  value.  It is declared at this use site rather than in `include/gtkutil.rs` (which lists it as
  owed at its 36) because adding it to that mirror would make `src/pgtkmenu.rs` 150 and
  `src/xmenu.rs` 825's `XG_ITEM_DATA` copies fail `check-mirror-dupes` - the same measurement
  `docs/ports/include-frame.md` recorded for `note_mouse_highlight`, and the mirror edit is owed
  until those two copies move with it.
* **The new arm caught a real gate defect in slice 71.**  Arm UG (`have_x_windows` +
  `use_toolkit_scroll_bars` + `use_gtk`) was added for this block, and its first reading showed the
  GTK callbacks' two `x_send_scroll_bar_event` calls as unresolved.  The cause was the *port's* text,
  not C: slice 71 had gated `x_protect_window_for_callback`, `x_unprotect_window_for_callback`,
  `x_send_scroll_bar_event` and the two `to_input_event` twins `not (feature = "use_gtk")`, but C
  15537's `#endif /* not USE_GTK */` closes *before* them (C 15539), so those five are compiled in
  every toolkit-scroll-bar build including GTK's.  The five gates are now
  `all (feature = "have_x_windows", feature = "use_toolkit_scroll_bars")`, and UG's delta became the
  21 sites slice 71's text really brings under `use_gtk` (`xclient` x7, `n_protected_windows` x7,
  `protected_windows` x4, `protected_windows_max`, `XClientMessageEvent`, `XSendEvent`) with the two
  calls resolved.  A/UT/UM read **0** before and after, so the widening is exactly "the text becomes
  live in the arm C compiles it in" and nothing else.
* **Six warnings my own text produced, all fixed by writing C's declarations.**  C 15797 declares
  `int whole, portion;` with **no** initialiser (the port's `= 0` copies were mine, and rustc's
  `unused_assignments` fired against C 15807-15808's assignments), C 15799's `part =
  scroll_bar_nowhere` is overwritten by C 15810's identical assignment (one statement survives, C's
  own redundant dead store being what cannot), and `f`/`time`/`dpyinfo` are each assigned once so
  they take no `mut`.  UG read 77 warnings before these fixes and 71 - its baseline - after.
* **`time as Time` is C's implicit widening written out.**  C 15817 passes a `guint32` to
  `x_display_set_last_user_time`'s `Time` parameter; the first writing's `E0308` (`expected u64,
  found u32`) is what says so.
* **Both callbacks are `unsafe extern "C" fn`.**  C 15881's pair goes through `G_CALLBACK` at the
  `xg_create_scroll_bar` calls (a later slice), which is the pgtk lane's reason for the same choice,
  and `xg_end_scroll_callback`'s unread `widget`/`event` keep their names with `let _ = (…)`
  (Deviation 44).

### 87. Slice 74 (C 15902-16034): the Xaw scroll callbacks, and what widening a gate exposed

* **The Xt declarations' gates were `use_motif`, and the Xaw arm is the counterexample.**  `Widget`,
  `XtNheight` and `XtVaGetValues` (this file's 15397-15403) were gated
  `all (have_x_windows, use_x_toolkit, feature = "use_motif")`, but C reads them from *both* the Motif
  arm (C 27553) and the Xaw arm (C 15924/15944/15989/16013), and libXt is what declares them - a
  `use_x_toolkit` build without Motif is a configuration C compiles.  The gates are now
  `all (have_x_windows, feature = "use_x_toolkit")`, `XtNwidth`/`XtNshown` join them (the two names the
  Xaw callbacks need; **UNVERIFIED** placeholders with libXt named as the missing header, Owed 0), and
  `Widget` is spelled `*mut _WidgetRec` - the port's own IntrinsicP.h row at this file 17130 says
  `typedef struct _WidgetRec { CorePart core; … } *Widget;`, which C 28639/28640's `widget->core.x`
  already depended on.
* **Four pre-existing defects that widening exposed, all fixed.**  (1) UT resolved **twelve** sites of
  `cannot find type Widget` (the name is now real in that arm).  (2) C 28639/28640's
  `(…widget as *mut _WidgetRec).core.x` was written for an *unresolved* `Widget`; with the name
  resolved, rustc reads the cast from `*mut _WidgetRec` to `*mut _WidgetRec` (`E0605`) and then finds
  no `core` field, so the two lines drop their redundant cast and dereference C's pointer directly
  (UM loses its 2 `E0609` + 2 `E0606`).  (3) C 28643-28644's `XtConfigureWidget` call passed `int`s to
  `Position`/`Dimension` parameters - C's implicit conversions are now stated (`x as Position`,
  `width as Dimension`, …).  (4) C 29700's `xfree (f->shell_position)` handed a `char *` to `void *`
  - now `as *mut c_void`.  Every one of these is the same class this lane keeps meeting: a name that
  was unresolved hides the conversions its definition needs.
* **The mirror's `eabs!` could not take C's float use, and that is a mirror defect.**  C 15950 writes
  `eabs (top + shown - 1)` on **floats**; C's macro compares against the integer literal `0`, which
  converts to the argument's type, while Rust's `0` does not (`E0308`: expected `f32`, found integer -
  measured).  `include/lisp.rs` 3154-3160's note already claimed "works for any signed type", so the
  body is now `if $x < Default::default () { -$x } else { $x }` - the zero of whatever type the
  argument has, which is C's conversion written where C relies on it - and the retired `0` token is a
  `docs/ports/macro-body-notes.tsv` row (`lisp.h`'s `--check` read one row before it and none after).
  P-87 ①'s two legs were run for the mirror change: `check-all-units.sh` -> `checked 83 unit(s),
  0 failure(s)` and `checking 83 unit(s), 0 unit failure(s)`; `check-mirror-dupes` -> 16/16/0.
* **The Xaw callbacks are `unsafe extern "C" fn`, and their locals take C's own values.**  Both are
  `G_CALLBACK`'s targets in `x_scroll_bar_create` (a later slice), the same reason slice 73 recorded;
  C 15912-15913's two pointer casts are spelled, C 15914's `float top = *top_addr;` is the
  dereference C writes, `shown`/`width`/`height` are written by `XtVaGetValues` so they take
  `= 0`/`= 0.0` (PORTING.md 1.5 - measured as `E0381` otherwise), and the bindings C assigns once per
  path take no `mut` (six of my first writing's warnings were exactly that, in this slice and the
  previous one's GTK twin).
* **`Dimension` is `use_x_toolkit`'s row and `c_ushort`.**  C 15982's `Dimension height, width;` is
  read by `XtVaGetValues` and handed to `x_send_scroll_bar_event`'s `int` parameter, so the calls
  state `as c_int`; the type's row (this file 16907) cites Intrinsic.h's `typedef unsigned short
  Dimension;` and is reused rather than re-declared.

### 88. Slice 75 (C 16036-16065): the two widget names and the `USE_GTK` arm's creators, in an arm that measures nothing yet

* **The two names are C's own text, so they are carried as measured values.**  C 16036-16037's
  `#define SCROLL_BAR_NAME "verticalScrollBar"` / `#define SCROLL_BAR_HORIZONTAL_NAME
  "horizontalScrollBar"` sit between C 16034's `#endif /* not USE_GTK and not USE_MOTIF */` and
  C 16039's two-line comment, i.e. *outside* both creators' arms, and inside C 15408's `#ifdef
  USE_TOOLKIT_SCROLL_BARS` with no `USE_GTK`/`USE_MOTIF` in the condition - so the port's gate is
  exactly `all (feature = "have_x_windows", feature = "use_toolkit_scroll_bars")`.  The values are
  not placeholders: they are C's bytes, the same pair the pgtk lane declares at `src/pgtkterm.rs`
  5927-5932 for `pgtkterm.c` 4088-4089, and the port writes them `&CStr` + `c"…"` the same way.
  C 16039-16040's comment ("Create the widget for scroll bar BAR on frame F.  Record the widget and
  X window of the scroll bar in BAR.") is carried at its own position, because it is the `#else`
  arm's twins' comment too.
* **`xg_create_scroll_bar`/`xg_create_horizontal_scroll_bar` stand at the use site (P-88 ①).**
  C 16049/16061 call them; their prototypes are gtkutil.h 118-122/123-127, which
  `include/gtkutil.rs` does not carry (the pgtk lane's Owed 12 records the same gap), so the
  declaration is written here under `+ use_gtk` - the pgtk lane's 5891-5902 block, name for name and
  parameter for parameter.  `G_CALLBACK` is `include/gtk.rs` 93's
  `type GCallback = Option<unsafe extern "C" fn ()>`, and both casts go through this file's own
  `G_CALLBACK!` macro.
* **Both creators are `unsafe fn` (C's `static`), the pgtk lane's twins line for line.**  C 16043-16053
  and C 16055-16065 are `x_create_toolkit_scroll_bar`/`x_create_horizontal_toolkit_scroll_bar`; the
  port's text is `src/pgtkterm.rs` 5943-5951/5953-5963 with `SCROLL_BAR_NAME` swapped for
  `SCROLL_BAR_HORIZONTAL_NAME` in the second: `let scroll_bar_name: *const c_char = … as_ptr ();` at
  C's line, `block_input ()`/`unblock_input ()` at C's, and `G_CALLBACK! (xg_scroll_callback)` /
  `G_CALLBACK! (xg_end_scroll_callback)` at C's argument positions.
* **The `#else` arm is deliberately not in this slice.**  C 16067's `#else /* not USE_GTK */` opens
  C 16069-16485: the same two names again, carrying their own `#ifdef USE_MOTIF` (C 16081-16140) and
  `#else /* !USE_MOTIF i.e. use Xaw */` (C 16141-16484) sub-arms, whose vocabulary is
  `XtSetArg`/`XmCreateScrollBar`/`XmN*`/`XtN*`/`XtAddCallback`/`XtRealizeWidget`/`XDefineCursor` and
  `x_alloc_lighter_color` - two of the three SDKs Owed 0 names.  That is the next slice's first item;
  nothing calls either name before `x_scroll_bar_create` (C 16715) lands.
* **What this arm measures: a zero, for a structural reason.**  Both creators are uncalled until
  `x_scroll_bar_create` arrives, so the reading is "the slice's own text resolves and nothing else
  moves": all five arms' head sets are **identical** between HEAD's unit and this text.  The
  arm-reachability probe is what shows the text is live in the `use_gtk` arm at all: `let _: c_int =
  true;` as `x_create_toolkit_scroll_bar`'s first statement moves UG's `E0308` count 17 -> 18,
  anchored at `build/probes/xterm/_root.rs:24701:18`.

### 89. Slice 76 (C 16067-16485): the Xt arm's two creators, and the placeholder set a second missing SDK forces

* **The arm's gate states `use_x_toolkit`.**  C 16067's `#else /* not USE_GTK */` body is Xt's API
  throughout (`XtSetArg`/`XmCreateScrollBar`/`XtCreateWidget`/`XtAddCallback`/`XtVaGetValues`), and a
  `use_toolkit_scroll_bars` build without Xt is not one C compiles - the ruling slice 72's UM arm
  recorded for the callback ("the region's whole API is Xt's").  So both creators are
  `all (have_x_windows, use_toolkit_scroll_bars, use_x_toolkit, not (use_gtk))`, with C 16081's
  `#ifdef USE_MOTIF` and C 16141's `#else /* !USE_MOTIF i.e. use Xaw */` as the two `#[cfg]` blocks
  inside the one function, in C's order, and the shared tail (C 16262-16274 / C 16470-16482) after
  both.  `widget` is assigned once per path by either block, so it takes no `mut` (the
  `xt_action_hook` pair's shape).
* **The Xt/Motif/Xaw vocabulary is declared at the use site, and every *value* is a placeholder.**
  This machine has `/opt/homebrew/include/X11` (Xlib's headers only): no `<Xm/ScrollBar.h>`, no
  `<Xaw/Scrollbar.h>`, no libXt.  Fifteen `XmN*` resource names, `XmCreateScrollBar`, four `Xm*`
  scalars, eleven Xt/StringDefs names, `scrollbarWidgetClass` and the six procedures are written as
  C's own names with **UNVERIFIED** values and **no** size or offset assertion (the class slice 72's
  Motif block and slice 74's `XtNwidth`/`XtNshown` rows carry).  The *resource names* are functions
  returning `*mut c_char` rather than `&CStr` constants, because C's `XtSetArg` assigns into
  `(arg).name` (a `char *`) and C 16239/16451 pass the names to a variadic call; C's `(String)` casts
  in the Xaw arm are no-ops here.  `XtNforeground` is the one such name a mirror owns
  (`include/widget.rs` 56), so C 16155/16363 spell it as that constant's pointer (P-26: one
  declaration per construct).
* **`Arg`/`XtArgVal`/`ArgList`/`XtSetArg!`/`XtCallbackProc`/`XtActionHookProc` join the
  `<X11/Intrinsic.h>` block, and `WidgetClass` joins `Widget`'s row.**  C 16074's `Arg av[20]` and
  every `XtSetArg` call in this unit needed the three types; the header's own
  `#define XtSetArg(arg, n, d) ((void)((arg).name = (n), (arg).value = (XtArgVal)(d)))` is carried
  as a macro so C's call text reads the same (`src/xmenu.rs` 633-697 / `src/xfns.rs` 1186-1199's
  shapes).  `av` takes C's uninitialised array as `core::mem::zeroed ()` (PORTING.md 1.5: Rust's
  definite-initialisation needs the whole array before `as_mut_ptr`); C's `++ac` is `ac += 1;`; C
  16087/16295's comma operator is the same two statements; C's `pixel != -1` states the conversion
  C relies on (`(-1 as c_int) as c_ulong`, this file's 18516 shape); and C's `0` in
  `XtAppAddActionHook`'s third argument is the null pointer C means.
* **The two `#ifdef XtN…` guards are features named after C's macro, and no arm sets them.**  C
  16194/16402's `#ifdef XtNbeNiceToColormap` and C 16240/16448's `#ifdef XtNarrowScrollbars` test
  whether the Xaw headers define those *names* - a build property of an SDK this machine does not
  have - so each is carried as `feature = "xtnbenicetocolormap"` / `"xtnnarrowscrollbars"` (the
  lowercased C macro, the `have_xaw3d` shape) and both bodies are text no arm this lane runs
  compiles.  The second one sits *inside* an argument list (C 16240-16242), which Rust cannot
  express (an attribute on an expression is not stable), so that call is written once per arm - the
  "each arm carries the statement" shape Decisions 48/76 record.
* **Three older defects the slice exposed, all fixed.**  (1) C 30350's `XtSetArg (al[0],
  XtNinitialState, state)` invoked this unit's macro without `!` - one `E0425` row until this slice
  defined `XtSetArg!`, after which rustc read it as `E0423` ("a macro named `XtNinitialState` exists
  in another namespace"); it is `XtNinitialState! ()` now.  (2) `x_toggle_visible_pointer`'s
  `let mut dpyinfo` (C 12102) is never mutated, and the two `E0425` rows that function carried for
  `XDefineCursor` (C 12132/12135) hid that `unused_mut` until this slice declared the name - C's own
  declaration takes no `mut`, so the binding lost it (the whole `build-tu` warning set is back to
  HEAD's, measured).  (3) C 766-768's and C 774-778's own two macros now yield C's `char *` (a
  string literal in the pointer context) rather than a `&str`, which is what the new `XtSetArg!`
  and the variadic call need.  C 30396/30398's `XtNiconPixmap`/`XtNiconMask` were hidden the same
  way (the unresolved macro made the whole call one row) and are declared here too.

### 90. Slice 77 (C 16488-16701): the four thumb setters, and the mirror C's type-generic `min`/`max` forced

* **Both pairs carry the gates the creators above them carry.**  C 16491's `#ifdef USE_GTK` pair is
  `all (have_x_windows, use_toolkit_scroll_bars, use_gtk)` (slice 75's gate) and C 16504's
  `#else /* not USE_GTK */` pair is slice 76's - `use_x_toolkit` and `not (use_gtk)` - with C
  16515's `#ifdef USE_MOTIF` and C 16559's `#else /* !USE_MOTIF i.e. use Xaw */` as the two
  `#[cfg]` blocks inside each function, in C's order, and C 16614/16697's `unblock_input ()` after
  both.
* **The mirror's `min`/`max` had to be widened, and that is this slice's real change.**  C's are
  type-generic macros (lisp.h 71-72: `#define max(a, b) ((a) > (b) ? (a) : (b))`); the port's rows
  were the `isize` pair, and C 16580/16589/16592 are **float** uses (`max (0, min (1, top))`,
  `min (top, 0.99f)`, `max (0, min (1 - top - (2.0f / height), shown))`, `max (shown, 0.01f)`) that
  cannot call it (`error[E0308]: expected f32, found isize`, measured on the first writing).  The
  rows are `pub fn min<T: PartialOrd> (a: T, b: T) -> T` now - the `eabs!` row's class, and the
  same reason slice 74 widened that macro's body.  It is a *superset*: every existing call site
  already casts its arguments, so `T` still infers `isize` (measured - HEAD's unit reads the same
  numbers with the widened mirror as without it: UT 811/68, UM 812/68, A/U 749/69 and 795/69,
  UG 792/71), and P-87 1's legs say so across the tree (`check-all-units.sh`: `checked 83 unit(s),
  0 failure(s)` and `checking 83 unit(s), 0 unit failure(s)`; `check-mirror-dupes` 16/16/0;
  `extract-abi --check emacs/src/lisp.h femacs/src/include/lisp.rs` reads `OK`).  The edit moved
  eight lines inside `include/lisp.rs`, which shifted three `docs/baseline/citations-allowlist.tsv`
  rows (3909/4038/4166 -> 3917/4046/4174, reason column untouched) - `check-citations` is what
  showed it, and it reads 0 drift again.
* **The two `#if false` blocks are carried as C's own text inside comments.**  C 16656-16673 (the
  whole "massage the top+shown values" block) and C 16679-16693 (the `if (top != old_top ...)`
  wrapper) are dead in *every* configuration, including the reference's own build, so the port
  carries them the way C 5127-5156's `record_event` is carried - an indented `/* ... */` block with
  C's own bytes - and no arm compiles them (`tools/cinventory.py`'s dead-branch map marks every
  line `False`, which is why no checklist demands anything from them).
* **Three defects my own writing had, all measured and fixed.**  (1) C 16530/16533 assign the two
  *parameters* `portion` and `whole` - in the Motif arm only - and the first writing left them
  immutable: `E0384 cannot assign to immutable argument` twice in UM.  The port re-binds them
  mutable (PORTING.md 2.2's parameter class) **inside C's own Motif arm** rather than at the
  function top, because the Xaw arm never assigns them and a top-level re-binding would be an
  `unused_mut` there (`x_toggle_visible_pointer`'s `let mut invisible = invisible;` is the
  uncfg'd twin, where both arms mutate).  (2) The four `top`/`shown` bindings' `mut` is C's own
  arm's business: the Motif arm assigns each once per path, the vertical Xaw arm reassigns them
  (C 16580-16592), and the horizontal Xaw arm's only reassignments sit inside its `#if false` - so
  the first writing's plain `let mut` cost six warnings (UT +2, UM +4).  They carry
  `#[allow(unused_mut)]` with that reason, the `src/w32uniscribe.rs` 2324 shape.  (3) With those
  fixed, UT and UM read exactly their slice-76 numbers again, 0 warnings moved.
* **The rest is C's own arithmetic, stated.**  `float top, shown;` is assigned on both paths of
  each `if`, so neither takes PORTING.md 1.5's zero or `#[allow]`; C's `(float)` conversions are
  `as f32`; `clip_to_bounds` is `include/lisp.rs` 804's `isize` row, so its arguments and its
  result state the conversions C gets implicitly (C 16551/16632/16633); `size`/`value` state C's
  `int` and `value` is reassigned at C 16555 (so `mut`); C 16570/16648's `old_top`/`old_shown`/
  `height` are written by `XtVaGetValues` and take C's own zero (PORTING.md 1.5, the slice-72
  reading); C 16605's `!xaw3d_pick_top` is the port's `== 0`; both horizontal sub-arms write
  `bar->whole` (C 16628/16638) and the Motif one divides by `whole - portion` (C 16630); and the
  GTK pair's two names join slice 75's block as one call each (the pgtk lane's 5898-5901 rows).
  `XtWindowToWidget` had no declaration anywhere until this slice: the mirror's
  `SCROLL_BAR_X_WIDGET!` (include/xterm.rs 1347) body calls it, and this slice's two calls are the
  first text that expands that macro.

### 91. Slice 78 (C 16709-16824): `x_scroll_bar_create`, the function the last three slices were waiting for

* **One function, C's own two arms.**  The banner (C 16705-16707) and the comment (C 16709-16712)
  are carried at their positions; C 16725's `#ifdef USE_TOOLKIT_SCROLL_BARS` is the arm that calls
  the two creators (C 16727/16729) and C 16730's `#else` builds a plain input-only X window
  instead - the two `#[cfg]` blocks this slice lands, rejoining at C 16777.  The gate is the file's
  own `have_x_windows`: C's configuration always has one of the two arms, but *arm U* (a
  `use_toolkit_scroll_bars` build with neither Xt nor GTK) is not a configuration C compiles, which
  is what its reading below shows.
* **The plain-X arm's four X11 mask names went into `include/xlib.rs`'s `x11_constants`.**  C
  16736-16740 name `ButtonMotionMask`, `PointerMotionHintMask`, `CWEventMask` and `CWCursor`, none
  of which the module carried; each value is measured on this machine's `xorgproto` headers
  (`X11/X.h` 164 `(1L<<13)`, 158 `(1L<<7)`, 403 `(1L<<11)`, 406 `(1L<<14)`), written in the module's
  own row shape and imported per function the way C 27531's block does (a targeted `use`, because
  the module's `None`/`Success` must not shadow the prelude's).  P-87 1's legs were run for the
  mirror change: `check-all-units.sh` -> `checked 83 unit(s), 0 failure(s)` and `checking 83
  unit(s), 0 unit failure(s)`; `check-mirror-dupes` -> 16/16/0 - and HEAD's unit reads the same
  numbers with the widened module as without it (749/69, 795/69, 811/68, 812/68, 792/71).
* **Four procedures are use-site declarations (P-88 1), and three of them retire older rows.**
  `window_box_height` (window.h; that mirror does not carry it and `src/window.c` defines it),
  `XLowerWindow` and `XMapWindow` (Xlib.h), `XtMapWidget` (`<X11/Intrinsic.h>` - `include/widget.rs`
  is `widget.h`'s mirror and does not own it), plus gtkutil.h's `xg_update_scrollbar_pos`/
  `xg_update_horizontal_scrollbar_pos` pair (the pgtk lane's rows).  `XLowerWindow` and
  `XtMapWidget` were already used by `x_make_frame_*`'s text as bare names; declaring them here
  retires those rows (UT and UM each lose three: 2 `XtMapWidget` + 1 `XLowerWindow`).
* **The two names that stay bare are the registered class, not an oversight.**  `XSetWindowAttributes`
  and `XCreateWindow` are used by landed text (port 1761/8890/13077 and 1825/8894/13085) and are
  counted there; this slice adds one use of each (A's +2 is exactly that), and closing them means
  `Xlib.h`'s record and prototype in `include/xlib.rs`, which is that mirror's own round.
* **Three defects my first writing had, all measured and fixed.**  (1) Five `///` doc comments sat
  before `unsafe extern "C"` blocks, which rustc reports as `unused doc comment` (the slice-76
  lesson) - they are `/* ... */` comments now, the form every other declaration block in this file
  uses.  (2) C's parentheses in `a.event_mask = (...) | (...)` and `mask = (...)` are
  `unnecessary parentheses around assigned value` here; the first pair is gone and the second
  became the cast C's assignment needs (`mask` is `unsigned long`, the constants are `long` - the
  `E0308` the first reading had), C's implicit conversion written where C leaves it implicit.
  (3) With those, every arm's warning set is back to its baseline's (0 movement, measured).
* **What the arms read, and why two of them go up.**  A `have_x_windows` 749/69 -> **750/69** (the
  two bare names above, minus `XLowerWindow`'s retirement); U `+use_toolkit_scroll_bars` 795/69 ->
  **804/69** (arm U is not a configuration C compiles: the two creator calls, the
  `SCROLL_BAR_X_WIDGET` macro and `Widget`/`Position`/`Dimension`/`XtConfigureWidget`/`XtMapWidget`
  are all `use_x_toolkit`-gated in the port and absent there - ten named rows, one retired); UT
  811/68 -> **808/68**, UM 812/68 -> **809/68**, UG 792/71 -> **791/71**, each **0 new rows** (the
  two creator calls resolve, plus the retired `XtMapWidget`/`XLowerWindow` rows).

### 92. Slice 79 (C 16827-16958): `x_scroll_bar_set_handle`, the first function of the plain-X region

* **C 16827's `#ifndef USE_TOOLKIT_SCROLL_BARS` is the region's condition.**  Every function from
  here to C 16958's `#endif` is the plain-X scroll-bar code, so this slice's gate is the file gate
  plus `not (feature = "use_toolkit_scroll_bars")` - slice 78's plain-X arm - and the four toolkit
  arms do not compile the function at all.  UT was re-measured to show it (808/68, its slice-78
  value); the other three are the same by construction, since the item is textually absent there.
* **C's own two parameters are the ones that need `mut`, and its six locals are the ones that do
  not.**  `start`/`end` are assigned at C 16874-16895 (both, on every path), so the port re-binds
  them mutable at the top - the `x_toggle_visible_pointer` shape - while
  `dragging`/`f`/`w`/`gc`/`inside_width`/`inside_height`/`top_range`/`length` are each assigned once
  on this straight path and so take no `mut` (adding one would warn: the reverse of the usual case,
  and why the readings below show no warning movement).
* **The four `VERTICAL_SCROLL_BAR_*` names are the mirror's macros, called with C's own
  arguments.**  `include/xterm.rs` 1368-1440 already carries them
  (`VERTICAL_SCROLL_BAR_INSIDE_WIDTH! (f, width)`, `..._INSIDE_HEIGHT!` and `..._TOP_RANGE!` with
  two arguments, `..._LEFT_BORDER! ()`/`..._TOP_BORDER! ()`/`..._MIN_HANDLE! ()` with none), so C
  16865-16867 and 16909-16943 read as C writes them; `FRAME_X_DRAWABLE!` is that mirror's macro and
  `FRAME_BACKGROUND_PIXEL`/`FRAME_FOREGROUND_PIXEL` are `include/frame.rs`'s functions.
* **`XSetForeground` and `XFillRectangle` stay bare names, and that is the recorded class.**  Both
  are in the ledger's own list of "Xlib entry points with no mirror row" (the per-name count list
  slices 1-9 recorded); their home is `include/xlib.rs`'s own round, and this slice's eleven uses
  (8 + 3) join that count instead of inventing a second spelling for them.  That is the whole of
  A's +11: the head-set diff is *only* those two names, with 0 rows retired.
* **C's `!= -1` tests are the port's conversions.**  `scroll_bar_background_pixel` and
  `scroll_bar_foreground_pixel` are `unsigned long`, so C 16901/16918/16934/16951 are written
  `!= (-1 as c_int) as c_ulong` (this file's 18516 shape), and C's parentheses around the two `&&`
  conditions and around each `XSetForeground` argument list are dropped - the file's
  `unused_parens` rule (Decisions 63/65's class) - with the warning readings below as the check
  that the file is clean.
* **What `--check` reads.**  **45 -> 44**: `x_scroll_bar_set_handle` retires as a missing function;
  the line-count row moves to **26,204** against C's 33,125 (20.9% drift, was 26,035 / 21.4%).  No
  mirror changed this slice, so P-87's legs do not apply to it.

### 93. Slice 80 (C 16960-17116): `x_scroll_bar_remove` and `XTset_vertical_scroll_bar`

* **Two functions, C's own arms, and the first callers of the last four slices' text.**
  `x_scroll_bar_remove` (C 16963-16986) has C 16970's `USE_GTK` arm (`xg_remove_scroll_bar`),
  C 16972's `#else` (`XtDestroyWidget` of the mirrored `SCROLL_BAR_X_WIDGET!`) and C 16975's plain
  `#else` (`XDestroyWindow`); `XTset_vertical_scroll_bar` (C 16994-17116) is the one that finally
  calls `x_scroll_bar_create` (C 17020, slice 78), `x_scroll_bar_set_handle` (C 17104/17109, slice
  79), `x_set_toolkit_scroll_bar_thumb` (C 17095, slice 77) and `xg_update_scrollbar_pos`/
  `XtConfigureWidget` (C 17056/17059, slices 77/78).  Both are file-gated (`have_x_windows`).
* **The four `CW*` values went into `x11_constants`, and the first writing forgot the import.**
  C 17038-17044 build the `XWindowChanges` mask out of `CWX`/`CWY`/`CWWidth`/`CWHeight`; the module
  did not carry them, so they join it with the other `<X11/X.h>` rows (measured: 410 `(1<<0)`,
  411 `(1<<1)`, 412 `(1<<2)`, 413 `(1<<3)`; C's own bodies are plain `int` literals, so these rows
  are `c_int` where slice 78's three `1L<<` rows are `c_long`, and the use site states the
  `as c_uint` C's `unsigned int mask` gets implicitly).  The first reading had four
  `cannot find value \`CW*\`` rows in every arm: the module's names are not glob-imported into a
  unit (the file note `include/xterm.rs` 66-79 records), so the function needs its own
  `use crate::x11_constants::{{CWHeight, CWWidth, CWX, CWY}};` - the `src/xterm.rs` 8887 shape.
  P-87 1's legs were run for the mirror change: `check-all-units.sh` -> `checked 83 unit(s),
  0 failure(s)` and `checking 83 unit(s), 0 unit failure(s)`; `check-mirror-dupes` -> 16/16/0; and
  HEAD's unit reads the same numbers with the widened module as without it (761/69, 804/69, 808/68,
  809/68, 791/71).
* **Three names are use-site declarations (P-88 1), and two of them retire older rows.**
  `xg_remove_scroll_bar` (gtkutil.h; the pgtk lane's 5993-6001 row, whose `ptrdiff_t` parameter C
  4198's call gives the same way), `XtDestroyWidget` (`<X11/Intrinsic.h>`; C 29689 already calls it
  as a bare name - the declaration retires that row) and `window_box` (dispextern.h's prototype,
  which `src/frame.rs` 460 declares for its own unit; declaring it here retires the two rows the
  landed text carried).
* **`XTset_vertical_scroll_bar`'s own hook-table row retires too.**  The five `x_create_terminal`
  hook assignments (slice 50) name `XTset_vertical_scroll_bar`; until this slice it was a bare name
  there (one row per site), and the definition resolves them - the same effect slice 67's
  `XTmouse_position` had on its own hook row.
* **What stays bare, and why.**  `x_scroll_bar_redraw` is C 1209's *forward declaration* of a
  function this file defines later (C 17200's region): Rust has no prototypes (this file's C
  1162-1210 note), so C 17025/17088 are the "not-yet-ported definition" class and their two rows
  retire when that definition lands.  `XWindowChanges`/`XConfigureWindow`/`XDestroyWindow` are the
  recorded Xlib classes (`include/xlib.rs`'s own round); `XWindowChanges` is C's record written
  field-wise, so it takes `std::mem::zeroed ()` (PORTING.md 1.5) like slice 78's
  `XSetWindowAttributes`, and its field writes are not extra rows (the type is already one).
* **What `--check` reads.**  **45 -> 43**: both functions retire as missing items; the line-count
  row moves to **26,416** against C's 33,125 (20.3% drift, was 26,204 / 20.9%).

### 94. Slice 81 (C 17119-17246): `XTset_horizontal_scroll_bar`, the vertical twin

* **The twin's four differences from the vertical function, all C's.**  C 17130's `window_box` call
  takes the *x/width* pair (the vertical one takes y/height); C 17193-17198's plain-X block clears
  the area the scroll bar does not cover before moving it (`WINDOW_CONFIG_SCROLL_BAR_HEIGHT`'s
  `area_height` and `rest`); the toolkit arm clears from `WINDOW_LEFT_EDGE_X (w)` for
  `pixel_width - WINDOW_RIGHT_DIVIDER_WIDTH (w)` (C 17176-17178) and passes `height` uncapped where
  the vertical one passes `max (height, 1)`; and the tail uses
  `HORIZONTAL_SCROLL_BAR_LEFT_RANGE! (f, width)` with
  `x_set_toolkit_horizontal_scroll_bar_thumb` (C 17225/17231).  `pixel_width` is assigned at its
  declaration (C 17127), `window_x`/`window_width` come from `window_box` and take C's own zero
  (PORTING.md 1.5), and the `CW*` mask needs this function's own
  `use crate::x11_constants::{{CWHeight, CWWidth, CWX, CWY}};` - the module is not glob-imported
  (slice 80's reading, now the standing shape for a `CW*` user).
* **A mirror macro's body names a function this unit does not have - one new row in every arm.**
  C 17194 is the first text in this unit to expand `WINDOW_CONFIG_SCROLL_BAR_HEIGHT!`
  (`include/window.rs` 758-762), whose body falls through to `FRAME_CONFIG_SCROLL_BAR_HEIGHT`
  (`include/frame.rs` 1296, a field read) and on to `estimate_mode_line_height`, which
  `include/dispextern.rs` 1308/1338/1368's macro bodies name and `src/dispnew.rs` 255 declares for
  its own unit only.  So every arm gains exactly one `cannot find function
  \`estimate_mode_line_height\`` row - the same class as slice 79's bare `XSetForeground`: a mirror
  body reaching a name the unit does not declare, recorded rather than worked around, whose home is
  the `window.h`/`dispextern.h` mirrors' own round.
* **The two `double` token rows are closed, and slice 80's is closed here.**  C's
  `int start = ((double) position * left_range) / whole;` is the port's
  `((position as f64 * left_range as f64) / whole as f64) as c_int` (PORTING.md 590: C's `double` is
  `f64`), so the C type name legitimately has no counterpart in the Rust body - the
  `json_out_float` row's class.  `docs/ports/macro-body-notes.tsv` gains the row for this function
  *and* for `XTset_vertical_scroll_bar`, whose row slice 80 left open (its readings there were
  45 -> 43, which is what the numbers say; the composition is what this entry records).
* **What `--check` reads.**  **43 -> 41**: `XTset_horizontal_scroll_bar` retires as a missing
  function and the two token rows close; the line-count row moves to **26,586** against C's 33,125
  (19.7% drift, was 26,416 / 20.3%).  The remaining `does not mention` row is
  `x_get_wm_check_window does not mention false`, the long-standing one.  No mirror changed this
  slice, so P-87's legs do not apply to it.

### 95. Slice 82 (C 17249-17493): the three scroll-bar hooks, the exposure handler and the redraw

* **Five functions, two gates.**  `XTcondemn_scroll_bars` (C 17261-17281), `XTredeem_scroll_bar`
  (C 17287-17366) and `XTjudge_scroll_bars` (C 17371-17394) are file-gated and are the three names
  `x_create_terminal`'s hook table (slice 50) carried as bare rows; `x_scroll_bar_handle_exposure`
  (C 17401-17452) and `x_scroll_bar_redraw` (C 17458-17492) sit inside C 17397's
  `#ifndef USE_TOOLKIT_SCROLL_BARS`, so they carry slice 79's gate.  The last one is the function
  C 1209 forward-declares: `XTset_vertical_scroll_bar`'s two calls (slice 80's C 17025/17088) were
  the "not-yet-ported definition" class until now, and this slice retires both rows.
* **C 17309's `goto horizontal` is the labelled-block shape, and its token row is closed here.**
  The jump is *forward*, to C 17332's `horizontal:` label, so a labelled `loop` is the wrong shape:
  the vertical half's body is one `'redeem: {{ ... }}` block whose end *is* the label, and
  `break 'redeem` lands exactly where C's `goto` lands (`src/androidselect.rs` 414's `'fail`/584's
  `'failure` are the same shape).  The label itself stays a comment, which is why the checker's
  tokens are both missing - `docs/ports/macro-body-notes.tsv` gains the `goto,horizontal` row with
  that reason.
* **`GraphicsExpose` joins `x11_constants`** (measured: `X11/X.h` 192 = 13), in the event-name row
  shape `ButtonPress`/`ButtonRelease` (slices 6/7) use; the unit imports it per function
  (C 17409's branch) the way the `CW*` rows are imported.  P-87 1's legs were run for the mirror
  change: `check-all-units.sh` -> `checked 83 unit(s), 0 failure(s)` and `checking 83 unit(s),
  0 unit failure(s)`; `check-mirror-dupes` -> 16/16/0; HEAD's unit reads the same numbers with the
  widened module as without it.
* **One defect my first writing had, fixed: `WINDOW_HAS_HORIZONTAL_SCROLL_BAR` is not a macro.**
  C declares it twice (window.h 909 for targets with horizontal scroll bars, 920's `false` for
  those without) and this target takes the second, which `include/window.rs` 472 carries as the
  accessor `pub fn WINDOW_HAS_HORIZONTAL_SCROLL_BAR (_w: *mut window) -> bool {{ false }}` - so the
  first writing's `WINDOW_HAS_HORIZONTAL_SCROLL_BAR! (w)` was one `cannot find macro` row in every
  arm, and calling it as the function it is clears it (its row count is unchanged afterwards, which
  is the check that the call now resolves).
* **What stays bare, and why.**  `XSetForeground` (five calls here), `XFillRectangle`,
  `XDrawRectangle` (its first use in the file) and `gui_intersect_rectangles` are the recorded
  "Xlib entry points with no mirror row" class (their home is `include/xlib.rs`'s own round);
  `XEvent`'s nine member rows (`r#type` once, `xexpose` four times, `xgraphicsexpose` four) are the
  192-byte placeholder's owed members (`include/xlib.rs`, the class slice 21's `msg.xclient.*`
  opened); and `XRectangle`'s four field writes state C's `int`-to-`short`/`unsigned short`
  conversions, the slice-74 reading.
* **What `--check` reads.**  **41 -> 36**: the five functions retire as missing items and the one
  token row this slice brought is closed by a notes row; the line-count row moves to **26,863**
  against C's 33,125 (18.9% drift, was 26,586 / 19.7%).

### 96. Slice 83 (C 17495-17596): `x_scroll_bar_handle_click`, and the `f` no compiler ever reads

* **One function, and no gate of its own.**  C 17502-17596 is `static void x_scroll_bar_handle_click
  (struct scroll_bar *bar, const XEvent *event, struct input_event *emacs_event, Lisp_Object
  device)` - the click handler C 22113/22118 and C 23892/23899 call four times (all still unported),
  and the first function of this file whose body C compiles in *both* scroll-bar builds: the
  definition carries no `#ifndef USE_TOOLKIT_SCROLL_BARS`, only inside the body (C 17509-17511's two
  locals and the two "released the handle" blocks, C 17550-17560 and C 17581-17591).  The port is
  therefore one `#[cfg (feature = "have_x_windows")] unsafe fn` whose two inner regions carry slice
  79's `not (use_toolkit_scroll_bars)` gate, and *all five* arms read it - the first slice since 79
  of which that is true.
* **The `kind` and `part` writes go through `input_event_bits!`** (PORTING.md P-80; the unit's own
  macro, `src/xterm.rs` 1371-1376, for the `kind:16 | part:16` unit `include/termhooks.rs` 116
  records).  C 17516-17518's `emacs_event->kind = (bar->horizontal ? HORIZONTAL_SCROLL_BAR_CLICK_EVENT
  : SCROLL_BAR_CLICK_EVENT)` is the setter arm with C's `if` expression inside it; C 17543-17548 and
  C 17574-17579's six `part` assignments are the same macro's `part` arm, whose cast widens the
  `scroll_bar_part` value (`include/termhooks.rs` 68-79) and whose shift is that unit's second half.
* **C 17537/17568's `f` is the defect that turns out not to be one.**  Both range macros are called
  with `f` as their first argument, and this function declares no `f` at all - the reconnoitring note
  predicted two `E0425 cannot find value f` rows to register.  Measured: **0** (all five arms,
  `grep -c 'cannot find value \`f\`'`).  xterm.h 1640/1662's macros take the parameter and never
  expand it, so C's preprocessor never puts the name in the translation unit; the mirror's
  `VERTICAL_SCROLL_BAR_TOP_RANGE!`/`HORIZONTAL_SCROLL_BAR_LEFT_RANGE!` (`include/xterm.rs`
  1376-1406) hand `$f` down to `..._INSIDE_HEIGHT!`/`..._INSIDE_WIDTH!`, whose bodies - C 1667's own
  text - do not mention it either, so nothing resolves the name in Rust either.  C's bytes stay (`f`
  where C writes `f`) and no row is owed; the reading is in the section below.
* **What stays bare.**  `Button1` (C 17519) and `ButtonRelease` (C 17524, 17552, 17583) are Decision
  15's X11 names - one `Button1` row and one to three `ButtonRelease` rows per arm, the non-toolkit
  arm reading the two extra sites its gate keeps; and the five `event->xbutton.*` members
  (`.button`, `.state`, `.time`, `.x`, `.y`) plus `event->type` are the `[u8; 192]` placeholder's
  owed members (`include/xlib.rs`, the class slice 21's `msg.xclient.*` opened).
* **My first writing's one defect, fixed.**  `FRAME_DISPLAY_INFO` was written without the `!` the
  mirror macro needs (`x_construct_mouse_click`'s C 14678 reading spells it with one), which is one
  `E0423 cannot find function, tuple struct or tuple variant` row in every arm - the identical row
  slice 66 measured for a different name.  With the `!` the row is gone and the arms read the table
  below.
* **What `--check` reads.**  **36 -> 35**: the function retires as a missing item and no token row
  arrives; the line-count row moves to **26,986** against C's 33,125 (18.5% drift, was 26,863 /
  18.9%).

### 97. Slice 84 (C 17598-17632): `x_scroll_bar_note_movement`, and the asymmetry a gate makes

* **One function, and it is arm A's alone.**  C 17598's `#ifndef USE_TOOLKIT_SCROLL_BARS` opens the
  region, C 17632's `#endif` closes it, and the only definition between them is
  `x_scroll_bar_note_movement` (C 17605-17630) - so the port carries
  `#[cfg (all (feature = "have_x_windows", not (feature = "use_toolkit_scroll_bars")))]`, the same
  gate slice 79's `x_scroll_bar_set_handle` and slice 82's exposure/redraw pair carry, and the four
  toolkit arms do not compile it at all.  The asymmetry in the table below is therefore the gate, and
  the *identical* toolkit logs are the reading.
* **Five shapes.**  C 17609's `XWINDOW (bar->window)->frame` is the mirror's `XWINDOW`, which is a
  function returning `*mut window` (`include/window.rs` 212), so the member is read through the
  pointer - `(*XWINDOW ((*bar).window)).frame`, the `src/androidfns.rs` 706 shape - and C 17610 is
  the ordinary `FRAME_DISPLAY_INFO` accessor.  `XMotionEvent` is one of the X11 records no mirror in
  this tree carries (slice 65's `x_note_mouse_movement` already stands on it), so this signature adds
  a second `E0425 cannot find type` row of that recorded class, and its field reads (`.time`,
  `.send_event`, `.y`) are **silent**: rustc reports the unresolved type once and suppresses the
  accesses on it - which is also why the baseline's 255 `E0609` rows are the *resolved*
  `[u8; 192]` placeholder's members and not this record's.  C 17613's `bool` member takes the
  `!= 0` conversion slice 65's C 14750 reading uses; `f->mouse_moved = true` is this unit's
  `frame_bits!` accessor (PORTING.md P-80, the macro slice 65's C 14757 reading first called); and
  `x_scroll_bar_set_handle` is slice 79's own definition, so this call resolves instead of adding a
  row.
* **What the asymmetry proves.**  The slice's whole footprint is one row, in one arm: nothing this
  slice's text does can hide behind the other four, because those four do not compile it - and if
  the gate were wrong, the four would have moved.  They did not: `diff` of the sorted
  `error[E..]: <message>` head lines with counts, both directions, is empty for U/UT/UM/UG, and their
  warning sets are identical to their baselines' by message (69/68/68/71).
* **What `--check` reads.**  **35 -> 34**: the function retires as a missing item and no token row
  arrives (`x_get_wm_check_window does not mention false` remains the standing one); the line-count
  row moves to **27,033** against C's 33,125 (18.4% drift, was 26,986 / 18.5%).

### 98. Slice 85 (C 17634-17769): the two report-motion functions, and the diagnostics a baseline was hiding

* **Two functions, ungated, and they close four calls slice 67 left.**  `x_scroll_bar_report_motion`
  (C 17637-17700) and `x_horizontal_scroll_bar_report_motion` (C 17706-17769) are the definitions C
  1173-1180's forward declarations name; the callers `x_fast_mouse_position` (C 14881/14884) and
  `XTmouse_position` (C 15034/15036) have carried them as unresolved rows since slice 67's readings
  recorded their arrival.  Neither definition sits under a C `#if`, so all five arms compile both -
  and the arm delta is uniform: four retirements, no additions.
* **Five shapes.**  `x_query_pointer` is *this unit's own* definition (C 14610-14628,
  `src/xterm.rs` 22888, `#[unsafe(no_mangle)] pub unsafe extern "C" fn`), so the two new calls - C
  17655 and C 17724 - resolve rather than adding a row (the reconnoitring note's "no mirror row"
  guess was wrong and the compiler says so, measured: `grep -c 'cannot find function
  x_query_pointer'` is 0 before and after).  The output locals (`dummy_window`, `dummy_coord`,
  `dummy_mask`, `win_x`, `win_y`) are C's own uninitialised declarations and are zeroed per
  Deviation 12 - the price Decision 36's round already recorded, and here it turned out to cost no
  warning at all (the `&raw mut` uses are what makes `mut` necessary, so no `unused_mut` row
  appears).  `XSETINT` on a dereferenced pointer states the EMACS_INT cast (C 17691-17692), C
  17760-17761 hands its two writes over in C's own swapped order (`*y` first), and
  `f->mouse_moved = false` is this unit's `frame_bits!` accessor (PORTING.md P-80).
* **The slice's real reading is what the baseline was hiding.**  Resolving the four calls makes
  `x_fast_mouse_position`'s body type-check, which is when its MIR diagnostics run for the first
  time: six `E0381 used binding ... isn't initialized` rows (C 14866-14868's `dummy`, `root_x`,
  `root_y`, `win_x`, `win_y`, `mask`, all handed to C 14933's `x_query_pointer` call) and two
  `unused_mut` warnings (C 14869-14870's `bar` and `dpyinfo`, C's declaration-then-single-assignment
  shape).  Neither is this slice's text and neither existed in the baseline's readings - and the
  discrimination is measured, not asserted: with those six locals left exactly as C declares them,
  putting one unresolved name (`let _ = no_such_name_xyz;`) back into the same body takes the run
  from **7 `E0381` rows / 71 warnings** to **1 / 69** (the 1 is the long-standing `lmsg` row), i.e.
  an unresolved name suppresses its body's MIR diagnostics.  That is the mechanism behind slice 67's
  silent readings, and it is why the six rows are *fixed* here (zeroed per Deviation 12 - the
  reading's job is to leave the ledger no hidden rows) while the two warnings stay, recorded rather
  than silenced: C's shape is the shape.
* **What `--check` reads.**  **34 -> 32**: both functions retire as missing items and no token row
  arrives (`x_get_wm_check_window does not mention false` remains the standing one); the line-count
  row moves to **27,190** against C's 33,125 (17.9% drift, was 27,033 / 18.4%).

### 99. Slice 86 (C 17772-17809): `x_scroll_bar_clear`, the definition outside the `#if` and the body inside it

* **One function, and the gate is asymmetric.**  C 17776's `static void x_scroll_bar_clear (struct
  frame *f)` is *outside* every `#if` - its caller C 11416 sits in `x_clear_frame`, which C compiles
  in both builds - while its whole body is inside C 17779's `#ifndef USE_TOOLKIT_SCROLL_BARS`.  The
  port therefore gates the *body* with one `#[cfg (not (feature = "use_toolkit_scroll_bars"))]`
  block and leaves the signature ungated: gating the definition would be one `E0425 cannot find
  function` row in each of the four toolkit arms, which is the shape the arms' readings would then
  show.  What the empty body costs is measured, not assumed - the toolkit arms get an unused `f`, and
  their warning sets are *identical* to their baselines' because the unit's own
  `#![allow (non_camel_case_types, …, unused_variables, …)]` (C 637's block, this file's line 638)
  covers it.
* **Three shapes, two of them already in this file.**  C 17799-17801's comma-expression loop
  condition is the same `while !NILP (bar) || { bar = condemned; condemned = Qnil (); !NILP (bar) }`
  rendering slice 82 gave the identical loop at C 17435 (`x_scroll_bar_handle_exposure`), so the
  file's two copies of C's "trick" now read alike; `XSCROLL_BAR` is `XSCROLL_BAR!`
  (`include/xterm.rs` 1336, the `src/gtkutil.rs` 2657 reading of `(*XSCROLL_BAR! (bar)).next`); and
  `FRAME_HAS_VERTICAL_SCROLL_BARS` is the mirror's function (`include/frame.rs` 1196-1213, two cfg'd
  definitions), so C's `if (FRAME_HAS_VERTICAL_SCROLL_BARS (f))` is `!= 0` here.  The three
  `XSetForeground` calls join the counted "Xlib entry points with no mirror row" class (five were
  already counted in slice 82, so the file's count 42 -> 45 in arm A).
* **What `--check` reads.**  **32 -> 31**: the function retires as a missing item and no token row
  arrives (`x_get_wm_check_window does not mention false` remains the standing one); the line-count
  row moves to **27,251** against C's 33,125 (17.7% drift, was 27,190 / 17.9%).

### 100. Slice 87 (C 17811-17848): the `ENABLE_CHECKING` buffer and the two no-op objects, read in a sixth arm

* **Three items, and half of them exist only under a macro no arm turns on.**  C 17811-17828 is the
  `#ifdef ENABLE_CHECKING` region - `undef` in the frozen table
  (`docs/baseline/config/darwin-arm64.macros.tsv` 872) - so `temp_index` and `temp_buffer` exist only
  in the checking arm, and `STORE_KEYSYM_FOR_DEBUG` has *two* definitions there: C 17819-17822's
  three-statement one and C 17826's `((void)0)`.  C 17833's `XTread_socket_fake_io_error` and C
  17840's `next_noop_dpyinfo` are plain file-scope pointers and carry the file gate alone.  The slice
  therefore needed a **sixth arm** - AE = A + `enable_checking` - because none of the five arms
  contains the checking text (P-86 6's rule, the slice-4 `have_gsettings` precedent: a two-arm
  reading leaves half a slice never compiled).  AE's own baseline is **820 / 70**, and the file's
  other `#ifdef ENABLE_CHECKING` regions are why its `E0308` count is 45 rather than A's 13.
* **The macro is a statement macro, and the arm this configuration compiles throws its argument
  away.**  C 17819-17822's last line carries no `;`, so the port's arm is one block expression and a
  call site keeps C's trailing `;`.  Three shapes inside it: C's post-increment is the load-then-step
  pair (the store writes `temp_buffer[temp_index as usize]` and the step follows it); `countof` is the
  mirror's `countof!` (`lib/include/stdcountof.rs`, lisp.h 28) with C's `int` comparison stated as
  `as c_int` (the `alloc.rs` 4510 shape); and C's `short` element write states its narrowing
  (`as c_short`).  C 17826's arm *discards* its argument rather than evaluating it, so the port's
  expansion is the recorded `let _ = 0;` shape (`src/pgtkfns.rs` 2938) and deliberately never names
  `$keysym`.
* **Two readings, both measured rather than argued.**  (1) *The pointer objects are already
  referenced*: C 31925-31926's uses of `next_noop_dpyinfo` are ported text (slice 12's region,
  `src/xterm.rs` 19860-19861), so landing the definition retires their two `E0425 cannot find value`
  rows in **all six arms** - which is why the slice's rustc footprint is -2 everywhere and not the 0
  the four visible lines suggest.  `XTread_socket_fake_io_error` has no reader yet (0 rows either
  way).  (2) *The checking arm's text type-checks*: with a temporary reader placed *after* the
  definition (`zz_probe_store_keysym`; no C text needs it yet - C's call sites are C 20629/20654/
  24536/24550, all unported), AE reads **0 error delta and 0 warning delta**, and a deliberate type
  error inside the macro body moves AE 818 -> 819 anchored at the macro's own line - so it is the
  *expansion*, not merely the definition, that the arm compiles.  The same reader placed *before* the
  definition is one `cannot find macro STORE_KEYSYM_FOR_DEBUG` row: `macro_rules!` is visible only
  after its definition, which is also why the port's file order (C's order) is what will keep C's
  future call sites inside its scope.
* **What `--check` reads.**  **31 -> 28**: the macro row and the two object rows retire, and no token
  row arrives (`x_get_wm_check_window does not mention false` remains the standing one); the
  line-count row moves to **27,302** against C's 33,125 (17.6% drift, was 27,251 / 17.7%).

### 101. Slice 88 (C 17850-17921): `x_filter_event`, and three shapes Rust cannot write the way C does

* **The slice is P-66 dead text, and its readings need three arms no default arm has.**  C 17850's
  `#ifdef HAVE_X_I18N` (`undef` in `docs/baseline/config/darwin-arm64.macros.tsv` 1718) wraps the
  whole definition, so the five standard arms and AE read **0/0** and neither the checklist nor
  `--check` ever demanded it.  This slice therefore adds **AI** (`have_x_i18n`), **AIU** (that plus
  `use_gtk`) and **AIUX** (that plus `have_xinput2`) - one arm per inner gate the function carries -
  and their baselines are 792 / 71, 812 / 73 and 910 / 75.
* **Shape 1: C's `else` is shared across a `#endif`.**  C 17861's
  `#if defined HAVE_XINPUT2 && defined USE_GTK` block declares `xinput_event`, opens an `if` and
  closes it with `else` *inside* the guard, while the statement after the `#endif` is the same
  assignment - so the port repeats that line in both arms and wraps each in a block, the rendering
  C 12411-12418 already has above.
* **Shape 2: `x_any_window_to_frame` is a function in one arm and a macro in the other.**  This unit
  defines it only for `any (use_x_toolkit, use_gtk)` (C 12344-12401) and carries xterm.c 12486's
  one-line macro for the plain-X arm, so C's single call line is written cfg'd three ways - the
  function call where the function exists and `x_any_window_to_frame!` where the macro does.  This
  is the file's first call site of that name, and it is why the arms must include a plain-X one.
* **Shape 3: C's `#ifdef` *inside* a condition, which Rust cannot spell.**  C 17892-17894's term
  `#ifdef HAVE_XINPUT2 || xinput_event #endif` sits inside the `else if`'s condition.  A `#[cfg]` on
  a sub-expression is `E0658 attributes on expressions are experimental` - measured with a two-line
  file, and the same gate is what makes `#[cfg]` on an *assignment expression statement* illegal too
  ("removing an expression is not supported in this position"; the port wraps such a statement in a
  block, which is the shape every cfg'd statement in this file already uses).  The port therefore
  widens `xinput_event`'s declaration from C's `HAVE_XINPUT2 && USE_GTK` to the outer `use_gtk` and
  writes C's term as plain text: its initialiser is `false` and only the `have_xinput2 && use_gtk`
  block ever writes it, so every arm's condition is C's own.  That is this slice's one recorded
  deviation, and it is a *gate widening* of the same kind slices 73/74 made.
* **What stays bare, and one owed mirror member.**  `XFilterEvent`, `XNoOp`, `xg_filter_key`,
  `KeyPress`, `KeyRelease`, `GenericEvent`, `XI_KeyPress` and `XI_KeyRelease` are Decision 15's
  "X11/XInput2 names with no mirror row" class; the four `XEvent` members (`xclient`, `xgeneric`,
  `xcookie`, `r#type`) are the `[u8; 192]` placeholder's owed members; `XIDeviceEvent` is the
  `E0425 cannot find type` class `XMotionEvent` opened (slice 65); `popup_activated` is a bare name
  the file already carried once; and **`prefer_native_input` is C 894-896's `#ifdef USE_GTK` member
  that `include/xterm.rs`'s `x_display_info` does not carry yet** - the mirror's owed run, named here
  so the row is a record rather than a surprise.  `? : None` states `None as Window` (the file's
  idiom, `src/xterm.rs` 1769+), and `XSETINT`-style narrowing does not arise.
* **The slice's one warning is C's own dead line.**  In the non-GTK arm C's
  `return XFilterEvent (...)` is unconditional and C 17918's `return 0;` behind it is unreachable -
  C compiles it, Rust names it: arm AI gains one `unreachable statement` (71 -> 72).  It stays: C's
  shape is the shape.
* **What `--check` reads.**  **stays 28** (dead text: the tool never demanded this function, and no
  token row arrives); the line-count row moves to **27,407** against C's 33,125 (17.3% drift, was
  27,302 / 17.6%).

### 102. Slice 89 (C 17923-18002): `event_handler_gdk`, a fifth arm for a disjunct, and two definitions this file still owes

* **The slice is dead text again, and its readings need five arms.**  C 17923's `#ifdef USE_GTK` is
  `undef` in the frozen table, so nothing the tool reads covers this function; the arm that compiles
  it is the `use_gtk` one, and its two halves are selected by `HAVE_X_I18N` (`#ifdef HAVE_X_I18N` /
  `#elif USE_GTK`), whose disjunct inside the second half is selected by `HAVE_XINPUT2` again.  That
  is why this slice's table needs **UG** (use_gtk), a *new* **UGX** (use_gtk + have_xinput2 - the
  only arm that compiles the `#elif` half's own disjunct), and the two `have_x_i18n` arms slice 88
  added (**AIU**, **AIUX**).  A two-arm reading would have left the `#elif` half never compiled
  (P-86 6's rule, the `have_gsettings`/`enable_checking` precedents of slices 4 and 87).
* **Shape 1: `#elif USE_GTK` inside `#ifdef USE_GTK` is tautological.**  Inside C 17923's guard the
  `#elif` at C 17950 is *always* true, so its port gate is `not (feature = "have_x_i18n")` alone -
  the two halves are written as two cfg'd blocks, which is also what keeps C's shared tail (C
  17986-18001) single.
* **Shape 2: a disjunct hoisted into a cfg'd `let`.**  C 17954-17965's `#ifdef HAVE_XINPUT2`
  disjunct sits inside the condition, and a `#[cfg]` on a sub-expression is `E0658` (Decision 101
  3's measurement).  The port hoists it into `let xinput_key_event: bool = …` - C's own text for the
  arm that has the feature and `false` for the one that does not, the forced substitute for a
  removed disjunct - and writes `|| xinput_key_event` in C's condition.  That is the second half of
  the shape slice 88 opened, and UGX is the arm that measures the feature-on text of it.
* **What the rows are.**  `GdkEvent`/`gpointer` are `include/gtk.rs`'s records (xterm.rs reaches them
  through `include/gtkutil.rs` 53's own `include!` of that mirror); `GdkFilterReturn`, `GdkXEvent`
  and the two `GDK_FILTER_*` values are that mirror's **owed** surface - no mirror here carries them,
  so they stay C's names and the arms' rows are the record, the treatment slice 88 gave
  `prefer_native_input` (which this slice reads again, in UG, through C 17951).  `handle_one_xevent`
  and `x_dispatch_event` are this file's own still-unported definitions: their two call rows are the
  ledger's "definition not yet landed" class, and their `--check` rows (still in the list of 28)
  retire when they land.  `x_any_window_to_frame` needs no three-way split here - the whole function
  is under `use_gtk`, where it is the unit's function.
* **What `--check` reads.**  **stays 28** (dead text: no item, no token row); the line-count row
  moves to **27,511** against C's 33,125 (16.9% drift, was 27,407 / 17.3%).

### 103. Slice 90 (C 18006-18056): `x_net_wm_state` and `x_flush_dirty_back_buffer_on`, and what the `have_window_system` gate costs

* **Two functions, both of them `--check` items.**  `x_net_wm_state` (C 18010-18038) is ungated and
  C 21606's call in `handle_one_xevent` (still unported) is owed; `x_flush_dirty_back_buffer_on`
  (C 18042-18056) sits with its whole body inside C 18045's `#ifdef HAVE_XDBE` (`undef` in the frozen
  table), so the port gates the body alone and leaves the signature ungated - slice 86's
  `x_scroll_bar_clear` shape.  Its fourteen call sites (C 20233 … C 25111) are all inside
  `handle_one_xevent`/`XTread_socket`, so no rustc row retires: the slice's whole error footprint is
  the two `--check` items, and **28 -> 26** is the reading.
* **One new token row, closed the same commit.**  C's `switch`/`case`/`break` in `x_net_wm_state`
  become a `match` with one arm per `FULLSCREEN_*` value and C's implicit `break` as each arm's end,
  so the three keywords have no counterpart: `docs/ports/macro-body-notes.tsv` gains the
  `x_net_wm_state` row (the `check_fwd_predicate` precedent).
* **The `have_window_system` gate, and its measured price.**  `FULLSCREEN_*` is frame.h 47-59's enum,
  which `include/frame.rs` carries inside its `HAVE_WINDOW_SYSTEM` block, so C 18013's initialiser and
  the `match` carry that gate (the C 28010 reading).  The declaration is written twice - C's own text
  under the feature and the enum's first value (`0`, which *is* `FULLSCREEN_NONE`) without it - so the
  arm that lacks the block still hands `x_get_current_wm_state` an initialised `value`.  Two things
  are measured rather than argued: (1) **every arm gains one warning**, `unused_mut` on C 18014's
  `lval`, which the gated `match` is the only writer of - exactly the two-`lval` reading slice 26
  recorded, and it is *not* avoidable by splitting `lval`'s declaration, because C's shape is a single
  one (a first writing that split it read 0 warnings and was reverted in favour of the precedent);
  (2) **the arm that turns `have_window_system` on cannot read the gated text**: it reads the same
  `E0428 x8` the slice-26 reading recorded (`include/dispextern.rs` 1608/1776-1788's duplicate names),
  with no other class moving, so this lane has no such arm.
* **The `have_xdbe` arm is where the new rows are.**  Its three: C 18047's
  `buffer_flipping_blocked_p` (`include/dispextern.rs` 4051 records it as **owed, not declared**), and
  the two `x_output` members - `complete` and `need_buffer_flip` - that `FRAME_X_COMPLETE_P!`/
  `FRAME_X_NEED_BUFFER_FLIP!` (`include/xterm.rs` 1010-1016) read when their bodies type-check for the
  first time.  `FRAME_GARBAGED_P` and `show_back_buffer` resolve (the latter is this unit's own C 7526
  definition under the same feature).  The first writing had two defects this arm caught, both fixed:
  the two macro names written as function calls (`E0423`, "a macro named … exists in another
  namespace") and `FULLSCREEN_NONE` read without the gate (one row in **every** arm).
* **What `--check` reads.**  **28 -> 26** as above, with the line-count row at **27,585** against C's
  33,125 (16.7% drift, was 27,511 / 16.9%).

### 104. Slice 91 (C 18058-18116): `x_scroll_bar_configure` and `mouse_or_wdesc_frame`, and two locals the port's own keyword made loud

* **One dead-text function and one `--check` item.**  `x_scroll_bar_configure` (C 18059-18083) sits
  inside C 18058's `#ifdef HAVE_GTK3` (`undef` in the frozen table), so it has no checklist row and no
  `--check` item: the slice adds **AG3** (`use_gtk` + `have_gtk3`) to read it, because no arm the lane
  ran before has `have_gtk3` on.  `mouse_or_wdesc_frame` (C 18094-18116) is ungated and *is* a
  `--check` item - its callers C 21011/21774/21956 are inside `XTread_socket`/`handle_one_xevent`
  (all unported), so nothing retires in rustc and **26 -> 25** is the reading.
* **`mouse_or_wdesc_frame` is the second port of the same C function.**  androidterm.c 551-573 *is*
  this function, and `src/androidterm.rs` 768-790 already landed it, so the rendering here is that
  one's, line for line, with a single documented difference: `lm_f` and `w_f` are read-only, so they
  are written without `mut`.  That keyword is the port's own word, not C's - and keeping it (as the
  androidterm rendering does) costs **one `unused_mut` warning per arm**, measured (13 arms x 1, in
  the first writing).  `gui_mouse_grabbed`'s declaration is this unit's own use-site block (C 15028),
  and `track_mouse`/`Qdropping`/`Qdrag_source`/`FRAME_TOOLTIP_P`/`x_window_to_frame` all resolve, so
  the function's whole footprint is that one `--check` row.
* **What AG3 reads, and the classes its 17 rows are.**  C's uninitialised `XEvent configure;` is
  `std::mem::zeroed ()` (the `xm_send_drop_message` reading) and its fourteen `xconfigure` writes are
  the `[u8; 192]` placeholder's owed members - the file's biggest single addition of that class
  (+12 rows over the two the file already had).  C 18074-18075's `GDK_WINDOW_XID` is the *system
  header's* macro in this arm (xterm.h 1434-1440 redefines it only for the GTK2 arm), and no mirror
  carries that arm's macro, so C's name stands and two rows are the record; `event->configure.width`/
  `height` are `GdkEventConfigure` members the mirror does not carry yet (it has `type`, `window`,
  `send_event`, `x`, `y`); and `x_dispatch_event` is this file's own still-unported definition,
  whose `--check` row retires when it lands.
* **What `--check` reads.**  **26 -> 25**: `mouse_or_wdesc_frame` retires as a missing item, no token
  row arrives, and the line-count row moves to **27,664** against C's 33,125 (16.5% drift, was
  27,585 / 16.7%).

### 105. Slice 92 (C 18118-18187): `x_dnd_compute_tip_xy`, and the fourteen conversions C leaves to its own arithmetic

* **One `--check` item, ungated, and its caller is still unported.**  `x_dnd_compute_tip_xy`
  (C 18118-18187) is called only from C 18211 (`x_dnd_update_tooltip_position`, the next definition),
  so no rustc row retires and **25 -> 24** is the reading.  Because the definition is ungated, all
  twelve arms move together: the two rows are C 18160-18161's `x_display_pixel_width` and
  `x_display_pixel_height`, this file's own still-unported definitions (`--check`'s list, whose rows
  retire when they land).
* **Five shapes.**  C's `while (CONSP (attributes))` walk is the port's loop with `XCAR`/`XCDR` (the
  helpers the file already reads the same way) and C's `break` is Rust's; `Qgeometry` is the mirror's
  accessor (`Qgeometry ()`, `include/globals.rs` 2785, reached through lisp.h 690's own
  `include!` of it); the four `XFIXNUM (Fnth (make_fixnum (n), geometry))` reads keep C's nesting;
  C's two `if`/`else if` chains keep C's own comments, including the two "Can happen for negative
  dx/dy" ones; and `tip_frame`/`tip_dx`/`tip_dy`/`x_dnd_frame` are this unit's file-scope objects
  (`include/xterm.rs` 736-741's block).
* **The slice's real reading is the fourteen conversions.**  `XFIXNUM` is the mirror's `EMACS_INT`
  accessor, and C mixes its result into `int` arithmetic in fourteen places (`min_x + XFIXNUM (…)`,
  `*root_y + XFIXNUM (tip_dy)`, the two `+=`/`-=` compound assignments) - C's usual arithmetic
  conversions, which Rust does not perform.  The first writing left them implicit and the arms read
  **14 `E0308` and 12 `E0277` rows** ("cannot add `isize` to `i32`", "cannot add-assign"); stating C's
  conversions (`as c_int`, the C 11761 precedent) returns every arm to its baseline class histogram.
  That is the whole of the slice's second reading, and it is why the arm table's deltas are +2 and
  not +28.
* **What `--check` reads.**  **25 -> 24**: the function retires as a missing item and no token row
  arrives; the line-count row moves to **27,753** against C's 33,125 (16.2% drift, was 27,664 /
  16.5%).

### 106. Slice 93 (C 18189-18259): the two tooltip functions, and the arm split C's `#ifndef` makes

* **Two `--check` items, and no other rustc footprint than one call.**  `x_dnd_update_tooltip_position`
  (C 18189-18226) is called only from C 18258 (inside the second function, which this slice also
  lands), and `x_dnd_update_tooltip_now` (C 18228-18259) from C 18674/21408/25544 (all inside
  still-unported text), so nothing retires beyond the two checklist rows and **24 -> 22** is the
  reading.  The first function is otherwise silent: `x_dnd_compute_tip_xy` is slice 92's definition
  (so that call resolves), `XMoveWindow` is the file's own use-site declaration (C 15570-15574), and
  `FIXNUMP`/`FRAMEP`/`FRAME_LIVE_P`/`FRAME_VISIBLE_P` are the mirrors'.
* **The split is the slice's reading.**  C 18243's `#ifndef HAVE_XINPUT2` picks `XQueryPointer` - a
  bare Xlib name no mirror declares, the file's counted class - in the ten arms that do not turn
  `have_xinput2` on, and picks slice 65's `x_query_pointer_1` definition in the two that do.  So the
  same slice reads **+1 row in ten arms and 0/0 in two**, and that asymmetry is the measurement: it is
  the first time a `#ifndef` of this file's DND region has been read on both sides.
* **Two defects the arms caught, both fixed.**  (1) `None` in that function was the *prelude's*
  (`E0605 non-primitive cast: Option<_> as u64`): the mirror's `None` needs the per-function
  `use crate::x11_constants::None;` - the slice-82 lesson, read again here.  (2) The seven output
  locals of `x_dnd_update_tooltip_now` are handed over by address, and **only the `have_xinput2`
  arms read the seven `E0381 used binding ... isn't initialized` rows**: the ten others read none
  because their callee is unresolved, and an unresolved call silences its arguments (slice 85's
  reading, measured again).  Zeroing them per Deviation 12 is what makes the column uniform - and the
  same class is why `tip_f`'s `mut` was dropped (a `mut` on a never-reassigned binding is one
  `unused_mut` warning per arm; that keyword is the port's own, not C's - slice 91's reading).
* **What `--check` reads.**  **24 -> 22**: both functions retire as missing items and no token row
  arrives; the line-count row moves to **27,848** against C's 33,125 (15.9% drift, was 27,753 /
  16.2%).

### 107. Slice 94 (C 18481-18497): the two display measurements, and the declaration the mirror had to give up

* **Two `--check` items, and the call rows they retire.**  `x_display_pixel_height` (C 18481-18488) and
  `x_display_pixel_width` (C 18490-18497) are both declared in xterm.h 1863-1864, so both are the
  exported shape (`#[unsafe(no_mangle)] pub unsafe extern "C" fn`), and **22 -> 20** is the checklist
  half of the reading.  Their bodies are C 18484-18487's `if (dpyinfo->screen_height) return ...;` plus
  Xlib's `HeightOfScreen`/`WidthOfScreen` macro, which no mirror in this tree carries: two bare names of
  Decision 15's class (`DefaultRootWindow`'s), one row an arm each.  The other half is the calls: C
  27531-27532 and C 27573-27574 (inside `x_calc_absolute_position`, slice 9's region), C 30266-30269
  (inside `x_wm_set_size_hint`) and C 18160-18161 (slice 92's own two, inside `x_dnd_compute_tip_xy`) -
  three rows per name where that half compiles, and the `x_wm_set_size_hint` pair is the seven non-GTK
  arms' alone.  C 28294-28309's six calls inside `do_ewmh_fullscreen` read nothing in any of the twelve
  arms, and the reason is measured rather than assumed: they sit in that function's
  `#[cfg (feature = "have_window_system")]` block, and no arm here sets that feature.
* **The declaration had to leave the mirror, and that is the slice's structural reading.**  The pair was
  declared in `include/gtk.rs` (behind `not (feature = "have_pgtk")`) - one declaration serving every
  unit that includes that mirror - and once the definitions landed in this unit, every `use_gtk` arm
  read `E0428 the name ... is defined multiple times` (`_root.rs`'s definition against
  `_gtk.rs`'s row).  The fix is P-72 rule 4's: the calling unit declares its own copy at the use site
  (`src/gtkutil.rs` C 1076/1082's two callees, `#[cfg (all (feature = "use_gtk", not (feature =
  "have_pgtk")))]`, keeping the P-55 `*mut c_void` stand-in because C's call sites cast) - the
  `include/frame.rs` 45-50 and `include/xsettings.rs` 12-14 precedent, and the mirror keeps a note
  saying where the name went instead of a copy of it.
* **The four `E0308` rows the `use_gtk` arms lose are the same four sites as the `E0425` rows the other
  seven lose.**  With the mirror's `*mut c_void` declaration in scope, `x_display_pixel_width
  (FRAME_DISPLAY_INFO! (f))` resolved but mismatched (`*mut x_display_info` into `*mut c_void`);
  landing the definition replaces the stand-in with C's own `struct x_display_info *` and the four
  mismatches retire with it.  That is why the two halves of the table move by -4 and by -2: same six
  call rows, two different diagnostics.
* **Four lines of slice 92's text that only this slice could expose.**  With the callees unresolved,
  the MIR diagnostics of `x_dnd_compute_tip_xy`'s body did not run (slice 85's mechanism); landing the
  definitions turned them on, and the four readings are: C 18151's reassignment needs `mut attributes`,
  C 18122's `min_x`/`min_y` read before assignment need the Deviation 12 zeroes, and C 18119/18120's
  `width`/`height` must lose the port's own `mut` (one `unused_mut` an arm otherwise - slice 91's
  reading).  The four are slice 92's debt, named here rather than buried.
* **What `--check` reads.**  **22 -> 20** (18 missing functions + 1 `does not mention` + the drift row):
  the line-count row moves to **27,881** against C's 33,125 (15.8% drift, was 27,848 / 15.9%).

### 108. Slice 95 (C 18261-18479): `x_dnd_update_state`, and the two pairs C's `#ifdef` puts inside an argument list

* **The `--check` item, and nothing retired behind it.**  `x_dnd_update_state` is C's `static`, so a
  plain `unsafe fn`, and **20 -> 19** is its whole checklist reading.  Every name the body reads was
  already this unit's: `x_query_pointer_1` (slice 65), `x_dnd_get_target_window`, `x_any_window_to_frame`,
  the four Xdnd senders with `xm_setup_drag_info` and the four Motif senders (slices 3-6), and
  `x_dnd_update_tooltip_position` (slice 93).  Its own callers are in unported text, so no call row
  retires: the seven arms without `use_gtk` read 0/0 on errors.
* **Two `#ifdef` pairs inside argument lists.**  C 18276-18280's `HAVE_XINPUT2` picks
  `x_dnd_pointer_device` or `-1`, and C 18406-18410's `HAVE_XKB` picks `x_dnd_keyboard_state` or `0` -
  both selecting an *argument*.  A `#[cfg]` cannot sit on an expression (P-51's arm rule, re-read in
  every slice since 88), so each arm's value is bound above the call (`device`, `state`) and C's two
  call statements stay single copies - the other half of the rule is slice 93's two cfg'd
  *statements*, where C's `#if` picks a callee rather than a value.
* **C's one name with two shapes, in a condition this time.**  C 18299's
  `x_any_window_to_frame (dpyinfo, toplevel)` is this unit's *function* in the `USE_X_TOOLKIT`/`USE_GTK`
  arm (C 12344) and C 12486's `#define` in the others, and here it sits inside C's `&&`.  The
  port binds the arm's value first (`any`, the `xinput_key_event` shape at this file 27465) and keeps
  C's single `if`; C 18330-18331's assignment takes the two-cfg'd-statements shape the file already
  uses (`C 15184`).  The price of the binding is named rather than assumed: the lookup runs whenever
  the enclosing test did, where C's `&&` would have skipped it - a read-only frame lookup either way.
* **C's four message structs are zeroed, and C 18454 is C's own bug.**  `lmsg`/`emsg`/`dmsg`/`dsmsg`
  are C's uninitialised declarators handed to the senders by address, so they are zeroed per
  Deviation 12 (Rust's definite-initialisation cannot see the writes).  With them zeroed, the
  remaining reading is C's: C 18454 writes `dmsg.byteorder` where the surrounding block fills
  `dsmsg`, so `dmsg`'s write is dead *and* `dsmsg`'s `byte_order` is never set - exactly what the
  reference does, kept character for character, and Rust says so: one `value assigned to `dmsg` is
  never read` warning in **every** arm.
* **What `--check` reads.**  **20 -> 19**: the function retires as a missing item; the line-count row
  moves to **28,148** against C's 33,125 (15.0% drift, was 27,881 / 15.8%).

### 109. Slice 96 (C 18499-18612): `x_wait_for_cell_change`, the first body that is one `#ifndef USE_GTK` pair

* **The `--check` item and its one token row.**  `x_wait_for_cell_change` is declared in xterm.h 1803,
  so the exported shape, and **19 -> 18**: the function retires as a missing item, and C 18522's
  `while (true)` - which the port spells `loop` (this file 21756's rule, measured again: the first
  writing's `while true` drew Rust's `while_true` lint in every arm) - is a token row the notes table
  closes rather than a keyword the port can carry.
* **One `#ifndef USE_GTK` pair around the whole body.**  C 18507-18511's three declarations, C
  18517-18520's two statements and C 18532-18557's block are all this arm's alone, so they carry
  `#[cfg (not (feature = "use_gtk"))]` (`x_find_modifier_meanings`'s shape for the declarations,
  `event_handler_gdk`'s for the statements), and the GTK arm's C 18568-18591 is the other cfg block.
  C's `for (dpyinfo = x_display_list; dpyinfo; dpyinfo = dpyinfo->next)` keeps the file's `while`
  transcription of a C `for` (this file 7395), with the increment where C's own third clause sits.
* **What the body's names cost, measured.**  Seven arms add five rows apiece - `ConnectionNumber` x1
  (C 18530), `XPending` x2 (C 18536/18538), `XNextEvent` x1 (C 18542) and `handle_one_xevent` x1
  (C 18543, its own `--check` row) - and the five `use_gtk` arms add only `ConnectionNumber`, because
  C 18532's block is not compiled there.  The GTK arm's `gtk_events_pending`/`gtk_main_iteration`
  resolve without a row: `include/gtk.rs` declares them (937-939, 2026-2028) and `include/xterm.rs`
  53's C 69 edge brings it into this unit.
* **Deviation 12's zeroes.**  `hold_quit` (C's uninitialised `struct input_event`, handed to
  `EVENT_INIT`), `finish` (written by `handle_one_xevent` through `&raw mut`) and `event` (C's
  uninitialised `XEvent`) are zeroed at their declarations; the two `fd_set`s are C's own
  uninitialised locals and stay that way (`FD_ZERO` writes them through the address).
* **What `--check` reads.**  **19 -> 18**, with the line-count row at **28,303** against C's 33,125
  (14.6% drift, was 28,148 / 15.0%).

### 110. Slice 97 (C 18614-18679): `x_find_monitors_changed_event`, and the GTK callback that calls it

* **The `--check` item and a zero in the seven arms.**  `x_find_monitors_changed_event` (C 18618-18635)
  is C's `static`, so a plain `unsafe fn`, and **18 -> 17**.  It reads 0/0 in the seven arms without
  `use_gtk`: the queue walk is this unit's own (`kbd_fetch_ptr`/`kbd_store_ptr`, `X_NEXT_KBD_EVENT!`
  at 745), the kind is the masked read `input_event_bits! (kind, &raw mut (*event).ie)` (PORTING.md
  1.3) against `include/termhooks.rs` 62's `MONITORS_CHANGED_EVENT`, and `XTERMINAL` is
  `include/termhooks.rs` 346's function.
* **The GTK callback is dead text carried whole.**  C 18637-18679's `#ifdef USE_GTK` block is
  `undef` in the frozen table, so `x_monitors_changed_cb` gets the gate on the item and no checklist
  row - P-86 1's rule, and the shape `event_handler_gdk` (slice 89) gave a whole-file arm.
* **Two rows, and the field they name is the mirror's.**  The five `use_gtk` arms add
  `E0609 no field last_monitor_attributes_list on type x_display_info` twice - C 18663's
  `NILP (Fequal (current_monitors, dpyinfo->last_monitor_attributes_list))` and C 18677's write.
  C declares that member under `#if defined HAVE_XRANDR || defined USE_GTK` (xterm.h 831-834), and
  the mirror carries only the frozen configuration's layout, so the row is the same class slice 92's
  `prefer_native_input` row opened: the mirror's own round owns it (P-87 3).  `Fequal` and
  `Fx_display_monitor_attributes_list` resolve without a row - `include/globals.rs` 8862 carries the
  subr and this unit's include graph reaches it.
* **Deviation 12's zero.**  `ie` is C's uninitialised `struct input_event`, zeroed at its declaration
  and then handed to `EVENT_INIT` (this file 14980's shape); C 18666's `ie.kind = ...` is the masked
  write (`input_event_bits!`'s three-argument form); `kbd_buffer_store_event` takes `*mut c_void`
  (this file 5206).
* **What `--check` reads.**  **18 -> 17**, with the line-count row at **28,389** against C's 33,125
  (14.3% drift, was 28,303 / 14.6%).

### 111. Slice 98 (C 18681-18761): `x_coords_from_dnd_message`, the twelve `xclient` reads and one lint false positive

* **The `--check` item.**  `x_coords_from_dnd_message` is C's `static`, so a plain `unsafe fn`, and
  **17 -> 16**.  Its four readers are slices 4-6's definitions (`xm_read_drag_motion_message`,
  `xm_read_drag_motion_reply`, `xm_read_drop_start_message`, `xm_read_drop_start_reply`), which return
  C's `bool` as `c_int`, so C's four `!` become `== 0`; the two KDE atoms are `include/xterm.rs`
  495-496's rows; and C's two `unsigned long` computations plus the stores into `int *` state their
  conversions (`as c_ulong` / `as c_int`, the rule since slice 92).
* **The twelve rows are the file's opaque-`XEvent` class.**  Every arm reads eleven
  `E0609 no field xclient` and one `no field r#type`: `include/xlib.rs` 1364 keeps C's measured
  192-byte `XEvent` with no member, and the baseline already reads 153 + 9 of these rows, so this
  function adds sites of a recorded class rather than a new one.  That is also why the earlier
  slices' `msg.xclient.*` writes are rows too.
* **One warning, and it is a lint false positive - measured, not assumed.**  C 18743-18750 is
  `if (((A) || (B)) && C && D)`, whose group pair is *required* (`&&` binds tighter than `||`).
  rustc's `unused_parens` fires on it as "unnecessary parentheses around `if` condition" and offers to
  remove the pair - which would change the meaning.  A two-line probe (`if ((a || b) && c && d)`)
  reproduces the same lint, so the port keeps C's grouping (C's own pair, no deviation) and the twelve
  arms each read **one** extra warning; the alternative shapes (dropping the pair, hoisting the group
  into a binding, or splitting the `if`) were rejected because each deviates from C's structure or
  semantics to hide a compiler defect.
* **Deviation 12's four zeroes.**  `dmsg`/`dreply`/`smsg`/`reply` are C's uninitialised Motif message
  structs, zeroed at their declarations because the readers write them through `&raw mut`.
* **What `--check` reads.**  **17 -> 16**, with the line-count row at **28,487** against C's 33,125
  (14.0% drift, was 28,389 / 14.3%).

### 112. Slice 99 (C 18763-18824): `x_handle_wm_state`, and the member the mirror records as a comment

* **The `--check` item, and the token row it brings.**  `x_handle_wm_state` is C's `static`, so a plain
  `unsafe fn`, and **16 -> 15**: the function retires as a missing item, and C 18796/18811's
  `SET_FRAME_VISIBLE (f, 1)`/`(f, 0)` - which the port spells `true`/`false` (this file 17924-17925's
  reading of the same pair) - put a `does not mention 1` row on the checklist, closed by a
  `docs/ports/macro-body-notes.tsv` row rather than by weakening the call.
* **One row in every arm, and it is this unit's owed record.**  `E0609 no field has_been_visible on
  type x_output`: C's member is `bool_bf has_been_visible : 1` (xterm.h 1219), and the mirror carries
  bit-fields as `// C:` lines instead of storage (`include/xterm.rs` 849, PORTING.md 1.3), so a unit
  that reads one spells C's name and books the row - the class `x_calc_absolute_position` opened
  (this file 15463-15466) for this very member.
* **One more row in the five `use_gtk` arms.**  C 18776's `FRAME_OUTER_WINDOW (f)` reaches the GTK
  arm's mirror chain, whose end (`GDK_WINDOW_XID` -> `GDK_WINDOW_XWINDOW`) no mirror declares - the
  class slice 95 opened, here as a new site (each of the five arms reads one row).
* **Two conversion defects the first writing read, both C's own.**  `AnyPropertyType` is Xlib's `0L`
  (`include/xlib.rs` 87), so C's implicit widening of it into `XGetWindowProperty`'s `Atom` argument
  is written `AnyPropertyType as Atom`; and `f->output_data.x->has_been_visible` needs C's own cast
  route in this port (`(*f).output_data.x as *mut x_output`, the shape this file already uses at
  8045).  C's `data = NULL` is `std::ptr::null_mut ()`, `!data` is `is_null ()`, `XFree` takes
  `*mut c_void` (15403), and `state = (unsigned long *) data` states the cast; the five
  out-parameters are zeroed per Deviation 12 because `XGetWindowProperty` writes them through
  `&raw mut`.
* **What `--check` reads.**  **16 -> 15**, with the line-count row at **28,569** against C's 33,125
  (13.8% drift, was 28,487 / 14.0%).

### 113. Slice 100 (C 18826-18864): the XFIXES arm, and the first slice no checklist row demands

* **Scope.**  C 18826's `#ifdef HAVE_XFIXES` carries two definitions - `x_handle_selection_monitor_event`
  (C 18829-18848, C's `static`) and `x_find_selection_owner` (C 18850-18862, declared in xterm.h 1822,
  so the exported shape) - and `HAVE_XFIXES` is **undef** in the frozen table, so neither is a
  `--check` item and neither appears in the checklist: that is why this slice's `--check` reading is
  unchanged at **15** and why its measurement is the arm it adds rather than a row it retires.
* **A thirteenth arm, and what it reads.**  `AXF` = A + `have_xfixes`.  On the baseline text it reads
  809/74 (the `have_xfixes` gate already turns on other ported text, so the slice's delta is against
  *that*), and with this slice's text it reads **816/74**: **+7** rows, all `E0609 no field` on C's
  three gated members - `monitored_selections` x4 (C 18843/18844/18857/18858), `selection_tracking_window`
  x1 (C 18837) and `n_monitored_selections` x2 (C 18840/18855).  They are Owed item 2's class: mirror
  rows this writing set cannot add.  No warning moved, and the twelve other arms read 0/0.
* **Three third-party-SDK shapes, all of them Owed.**  C 648-650's `#include
  <X11/extensions/Xfixes.h>` has no counterpart in the reference tree and no header on this machine
  (measured: `/opt/homebrew/include/X11/extensions/Xfixes.h` is absent), so the record is declared in
  the unit at the use site (P-88 1) with C's members in the SDK's documented order and its **layout
  UNVERIFIED** - only a pointer to it is ever dereferenced, so no size or offset is load-bearing.
  Owed item 0 now names it beside Motif, libXt and gtk-3.0.
* **C's own shapes.**  The `for` loops keep this file's `while` transcription with the increment in
  C's third clause (this file 7395), the array element is reached as
  `(*(*dpyinfo).monitored_selections.offset (i as isize)).owner` (20439's shape for `devices`), the
  `notify` cast is C's own `(XFixesSelectionNotifyEvent *) event`, and C 18861's `X_INVALID_WINDOW` is
  `include/xterm.rs` 388's constant (both functions return through it unchanged).
* **What `--check` reads.**  **15 -> 15** (unchanged: the two functions are not demanded and the
  token check does not reach a function the frozen table does not name), with the line-count row at
  **28,657** against C's 33,125 (13.5% drift, was 28,569 / 13.8%).

### 114. Slice 101 (C 25641-25811): `x_dispatch_event` and `XTread_socket`, and the first linkage C's `#ifdef` picks

* **One deviation, and it is a linkage the port cannot spell.**  C 25640's `#ifdef USE_GTK` makes
  `x_dispatch_event` `static int` in the GTK arm and plain `int` otherwise.  The port writes **one**
  item, in the exported shape (`#[unsafe(no_mangle)] pub unsafe extern "C" fn`, which xterm.h 1806's
  declaration and `check-linkage.py` both require for C's non-`static` arm), because C's statements
  must appear once; the GTK arm therefore exports a symbol C keeps internal, and no caller can see the
  difference (the GTK arm's callers are this file's own).
* **Two `--check` items retired, one token row brought.**  `x_dispatch_event` (C 25645) and
  `XTread_socket` (C 25672) both retire as missing functions - **15 -> 13** once the row below is
  closed - and C 25656's `0` argument (C's null `struct input_event *`, the port's
  `std::ptr::null_mut ()`) is closed by a notes row rather than by a null-pointer spelling C lacks.
* **The readings are the arms' split.**  The eight arms compiling `XTread_socket`'s `#ifndef USE_GTK`
  block read **+4** (five rows, one retirement); UG/UGX/AIU/AIUX read **0/0**; AG3 reads **-1**
  (its hook table names `x_dispatch_event` twice).  `XPending`/`XNextEvent`/`XNoOp` are Xlib's
  recorded bare class, and `handle_one_xevent` is the file's largest remaining `--check` row
  (C 18866-25632) - its placeholder now sits above this text in C's order (P-91 2).
* **C's own shapes.**  `terminal->display_info.x` is the mirror's P-72 rule-3 `*mut c_void` cast the
  way `x_term_init` reads it (this file 19940); both loops are cfg'd blocks with C's loop-local
  declarations inside (slice 96's shape) and those two locals are zeroed per Deviation 12 because
  `handle_one_xevent` writes `finish` through `&raw mut`; C 25692's comparison states
  `x_dnd_finish_display as *mut Display` (the object is the mirror's `*mut c_void`); C 25702's `= 0`
  is `std::ptr::null_mut ()`; C 25771's struct assignment is Rust's; and C 25774 clears the pending
  quit event's kind through `input_event_bits!`'s three-argument form.
* **What `--check` reads.**  **15 -> 13** (11 missing + 1 `does not mention false` + the drift row),
  with the line-count row at **28,874** against C's 33,125 (12.8% drift, was 28,657 / 13.5%).

### 115. Slice 102 (C 25816-25902): `x_clip_to_row` and `x_draw_hollow_cursor`, and the arm `have_window_system` splits

* **Two `--check` items retired.**  `x_clip_to_row` (C 25830-25851) and `x_draw_hollow_cursor`
  (C 25856-25902) are both C's `static`, so plain `unsafe fn`s, and **13 -> 11** with no token row
  between them.
* **The reading is the `have_window_system` split.**  The thirteen arms that leave the feature off read
  **+3**: `GCLineWidth` x2 (C 25882/25885 - a name no mirror carries, the baseline's five rows), and
  `XChangeGC` x1 (likewise, the baseline's nine) arrive, while `x_clip_to_row`'s own reference
  retires; the lane's **AWS** arm (`A + have_window_system`, slice 90's) reads **+2**, because C's
  `w->phys_cursor_width` is the mirror's `#[cfg (feature = "have_window_system")]` member
  (`include/window.rs` 152-153) and resolves there instead of reading one `E0609`.  That is the first
  slice whose reading differs by that gate rather than by `use_gtk`, and it is why the ledger's table
  now carries AWS as a row of its own.
* **`get_phys_cursor_glyph`/`get_phys_cursor_geometry` are use-site declarations, not rows.**  They
  are other units' definitions (`src/window.rs` 8061, `src/abi_shim.rs` 2419) and no mirror in this
  unit's graph carries them, so the port declares both in the function that reads them - P-72 rule 4,
  the block `src/pgtkterm.rs` 4013-4025 wrote for the same pair.
* **Five defects the first writing read, all in this slice's own text.**  (1) `GCLineWidth` is *not*
  a member of `crate::x11_constants` (importing it is `E0432`), so it is written bare like the file's
  five existing sites; (2) C 25891's `cursor_glyph->resolved_level` is the mirror's *bit-field
  accessor* - `glyph_bits! (resolved_level, (*cursor_glyph))`, `include/dispextern.rs` 587, whose
  first use this is; (3) and (4) C's two `short` reads of `pixel_width` state `as c_int`
  (Deviation 22's class, `src/pgtkterm.rs` 4136's reading of the same two expressions); and (5)
  `clip_rect` is C's uninitialised `XRectangle`, zeroed per Deviation 12 because C 25850 copies the
  whole value back through `rect_return`.
* **What `--check` reads.**  **13 -> 11** (9 missing + 1 `does not mention false` + the drift row),
  with the line-count row at **29,000** against C's 33,125 (12.5% drift, was 28,874 / 12.8%).

### 116. Slice 103 (C 25905-26014): `x_draw_bar_cursor`, the GC block and the per-unit inline copies

* **The `--check` item.**  `x_draw_bar_cursor` is C's `static`, so a plain `unsafe fn`, and this
  slice's `--check` reading is **11 -> 10** with no token row: C's `0`, `1` and `false` all survive in
  the port's spelling.
* **The reading is rows of recorded classes, +11 / +7.**  Thirteen arms: `phys_cursor_width` x4 (C
  25979/26004/26005/26009 - the mirror's `have_window_system` member, Decision 115 2's class),
  `window_box_left` x2 (the `WINDOW_TEXT_TO_FRAME_PIXEL_X!` expansion - the baseline already reads one
  of these from the same macro), `face_cache` x2 (this slice's own `FRAME_FACE_CACHE`/`FACE_FROM_ID`
  copies name a type the mirror gates behind `dispextern_face_records`, which no arm sets),
  `GCGraphicsExposures` x1, `GCLineWidth` x1 and `XChangeGC` x1 (the three unresolved-value/function
  classes the baseline carries 3/5/9 rows of).  AWS reads **+7**: the four `phys_cursor_width` rows do
  not exist under `have_window_system`, everything else does.
* **C's inline accessors are one copy per including unit.**  C's `FRAME_FACE_CACHE` (frame.h 1379) and
  `FACE_FROM_ID` (frame.h 1646-1650) are text C pastes into every unit that includes the header, and
  the mirror's own copy is a *module* (`include/frame.rs` 1534-1552) so it is not in any unit's scope;
  the port's live form is therefore a copy here, written in `src/pgtkterm.rs` 4156-4170's shape (the
  same pair for the pgtk arm).  `draw_phys_cursor_glyph`, `get_phys_cursor_glyph` and
  `get_phys_cursor_geometry` are other units' definitions (`src/abi_shim.rs` 2419/2499,
  `src/window.rs` 8061), declared at this use site by P-72 rule 4 - the block slice 102 opened for
  its own function, repeated here because a use-site declaration is per unit (and per function's own
  scope).
* **Four defects the first writing read.**  (1) `GCGraphicsExposures` is *not* a member of
  `crate::x11_constants` (importing it is `E0432`), so it is written bare like this file's 9269 site;
  (2) and (3) the two cursor-helper declarations belong to *this* function's scope, not slice 102's;
  and (4) C 25953/25955's *chained* assignments (`xgcv.background = xgcv.foreground = ...`) are two
  Rust statements each, because assignment is not an expression in Rust - C's one line per site stays
  named on both (Decision 116 4's deviation, this file's only one).
* **What `--check` reads.**  **11 -> 10** (8 missing + 1 `does not mention false` + the drift row),
  with the line-count row at **29,181** against C's 33,125 (11.9% drift, was 29,000 / 12.5%).


### 117. Slice 104 (C 26016-26096): `x_define_frame_cursor`, `x_clear_frame_area` and `x_draw_window_cursor`

* **The three `--check` items.**  `--check` goes **10 -> 7**: the three functions retire, and this
  slice's reading brings one token row (`x_draw_window_cursor does not mention break, case, default,
  switch`), which is closed by the `docs/ports/macro-body-notes.tsv` row this slice adds - C 26064's
  `switch (cursor_type)`, its five `case` labels and the `default` arm are the port's `match` over the
  same five enumerators with a `_` arm for C's `default` (`emacs_abort ()`), so the four keywords have
  no counterpart of their own and C's five `break;` statements are each arm's end (the
  `x_net_wm_state` row's reading, PORTING.md 2.2).  The seven rows left are the five missing functions
  (`handle_one_xevent`, `x_bitmap_icon`, `x_dnd_begin_drag_and_drop`, `x_term_init`, `x_text_icon`),
  `x_get_wm_check_window does not mention false` and the line-count row.
* **The reading is +3 / +2 / +4 / +1, and every added row is one this file's own text earns.**
  The eleven arms that do not set `use_x_toolkit` read **+3**: `XDefineCursor` x1 (C 26024's call -
  the name *is* declared, but only in this file's own `use_x_toolkit`-gated block at 24997-25004, so
  the arms without that feature read it as unresolved), `phys_cursor_type` x1 (C 26051's store - the
  mirror's `have_window_system` member, Decision 115 2's class) and `phys_cursor_width` x1 (C 26083's
  store).  **UT** and **UM** read **+2**: both set `use_x_toolkit`, so the declaration at 24997 is in
  scope and `XDefineCursor` contributes no row there.  **AI**, **AIU** and **AIUX** read **+4**: C
  26093's `xic_set_preeditarea` (inside C 26091's `#ifdef HAVE_X_I18N`) is a name no mirror of this
  unit's graph carries, so it joins them.  **AWS** reads **+1**: it is `have_window_system`, where
  both `phys_cursor_*` fields exist, so only `XDefineCursor` is added.
* **Nothing retires in any arm, and that is the expected shape.**  The three functions are C's
  `static`s whose addresses C 30668+'s `x_create_terminal` hands to the terminal hook table - a unit
  this port has not landed - so no arm names them yet; their retirement is the `--check` row's (the
  frozen reading), not an arm's.
* **The gates are C's own, one axis at a time.**  `x_define_frame_cursor` and `x_clear_frame_area`
  are unguarded in C (C 26019/26031) and carry this file's `have_x_windows` arm like every other item;
  `x_draw_window_cursor`'s `#ifdef HAVE_X_I18N` gates `f` **alone** (a cfg'd `let`, C 26045-26047) and
  the tail block C 26091-26094, which is why only the three `have_x_i18n` arms read the
  `xic_set_preeditarea` row.
* **One deviation the first writing had to fix** (this slice's own, and it was read, not guessed):
  the first text declared `pub type Emacs_Cursor = Cursor;` in the unit - `E0428`, because the mirror
  `include/dispextern.rs` 106-114 *already* carries C's two arms (`*mut c_void` for the arms without a
  window system, Xlib's `Cursor` under `have_window_system` + `have_x_windows`).  The fix deletes the
  alias and states `cursor as Cursor` at the two places C relies on the typedef being `Cursor` - the
  identity cast in the real X configuration, unwrapping only in the lane's arms.
* **What `--check` reads.**  **10 -> 7** as above, with the line-count row at **29,302** against C's
  33,125 (11.5% drift, was 29,181 / 11.9%).

### 118. Slice 105 (C 26104-26214): `x_bitmap_icon` and `x_text_icon`, and the two mirror gaps their `FRAME_X_WINDOW` uses read

* **The two `--check` items.**  `--check` goes **7 -> 5**: `x_bitmap_icon` and `x_text_icon` retire
  as missing functions, and this slice brings no token row.  The five left are
  `handle_one_xevent`, `x_dnd_begin_drag_and_drop`, `x_term_init`,
  `x_get_wm_check_window does not mention false` and the line-count row.
* **The reading is four sizes, and only two of them are new rows.**  Nine arms read **-1**: the
  `cannot find value x_bitmap_icon` row retires, because landed text already named the function
  (which was missing was its definition).  Five arms (UG, UGX, AIU, AIUX, AG3) read **+1**: the same
  retirement, less two rows the new text earns - `cannot find value gnu_xpm_bits` (the
  `#ifdef USE_GTK` arm's `xg_set_icon_from_xpm_data (f, gnu_xpm_bits)`; the array is the header's own
  `#if (defined (HAVE_XPM) && defined (HAVE_X_WINDOWS)) || defined (USE_GTK)` dead text, recorded in
  `docs/ports/include-emacs-icon.md` rather than transcribed, P-08) and `GDK_WINDOW_XWINDOW` (the
  `have_gtk3`-less GTK arms' `GDK_WINDOW_XID` expands to it at `include/xterm.rs` 1043 - the new
  `FRAME_X_WINDOW` uses are the first to reach that arm).  AXPM - the fifteenth arm this slice adds,
  `A + have_xpm` - reads **0**: the retirement less the `#elif` half's own `gnu_xpm_bits` row.  AWS
  reads **-1** as well (`have_window_system` changes none of these names).
* **`GDK_WINDOW_XID`'s GTK3 arm is a mirror gap, not a port defect.**  AG3 (`A + use_gtk +
  have_gtk3`) reads `cannot find macro GDK_WINDOW_XID`, anchored in `include/xterm.rs` 1056-1063's
  `GTK_WIDGET_TO_X_WIN`: the mirror carries that macro's `not (feature = "have_gtk3")` arm only, so
  the arm that sets `have_gtk3` has no `GDK_WINDOW_XID` to expand - a pre-existing gap this slice is
  the first to read (recorded in `## Owed`, with the GDK header this machine lacks).
* **C's gates, one axis at a time.**  C's `#ifdef USE_GTK` / `#elif defined (HAVE_XPM) &&
  defined (HAVE_X_WINDOWS)` pair is the port's two `#[cfg]` blocks (P-86 1) - the `#elif` half
  carries `not (feature = "use_gtk")` because C's `#elif` is reached only when `USE_GTK` is not
  defined; the frozen table leaves `USE_GTK` (3861) and `HAVE_XPM` (1708) undef, so neither arm is
  live in the frozen reading, and `x_text_icon`'s `XSetWMIconName`/`XTextProperty` are C's
  unguarded Xlib calls.
* **Three defects the first writing read.**  (1) `pub const XA_STRING: Atom = 31;` is `E0428` - the
  unit already carries `XA_STRING` at 18924 (the `XA_INTEGER` neighbour), so the copy is deleted and
  C 26202 reads the existing constant; (2) C 26164/26165 pass `gnu_xbm_width`/`gnu_xbm_height`
  (`int`) to `unsigned int` parameters - C's implicit conversion, so the port states `as c_uint`
  (the file's casting convention); and (3) `let mut bitmap_id` is `unused_mut` (one initialization
  on each path needs no `mut`; the warning set is what said so - `variable does not need to be
  mutable`), and the doc comment that describes `x_text_icon` sat above the declaration block
  instead of the function, which is `unused doc comment` (the mirrors' seven).
* **What `--check` reads.**  **7 -> 5** as above, with the line-count row at **29,493** against C's
  33,125 (11.0% drift, was 29,302 / 11.5%).

### 119. Slice 106 (C 12702-13327): `x_dnd_begin_drag_and_drop`, the file's second-longest function

* **The `--check` item.**  `--check` goes **5 -> 4**: `x_dnd_begin_drag_and_drop` retires as a
  missing function and no token row arrives.  The four left are `handle_one_xevent`, `x_term_init`,
  `x_get_wm_check_window does not mention false` and the line-count row.  The first writing *was*
  one line short: C 12852's `record_unwind_protect_void (x_clear_dnd_variables);` was dropped, and
  the checker's `does not mention x_clear_dnd_variables` row is what read it back (the tool is a
  line-level reader, not only a name-level one).
* **The reading is four sizes, and every added row is a class the baseline already carries.**  The
  eleven non-GTK arms read **+7**: `XWindowAttributes` x1, `XGetWindowAttributes` x1,
  `XChangeProperty` x1 and `XSelectInput` x1 (four Xlib names no mirror of this unit's graph
  declares - the baseline carries 5/3/16/4 rows of them), `handle_one_xevent` x1 (the *other*
  `--check` item - this function's event loop is its caller), `xany` x1 and `r#type` x1 (the
  `xlib_surface::XEvent` stand-in's owed members) and `last_monitor_attributes_list` x1 (the
  `x_display_info` field xterm.h 834 declares inside `#if defined HAVE_XRANDR || defined USE_GTK`,
  which the mirror does not carry either - 16 rows of it exist already).  The five `use_gtk` arms
  read **+6**: C 13073-13076's `if (event_display == FRAME_DISPLAY_INFO (f))` is *absent* in a GTK
  build, so those arms never read `xany`.  UT and UM read **+8** (`XtAppNextEvent`, the Xt arm's
  dispatch call), UGX and AIUX **+9** (`XIGetClientPointer` x1 and `client_pointer_device` x2) and
  AXKB **+15** (the XKB block: `XkbSelectEvents` x1, `XkbUseCoreKbd` x2, `XkbStateNotifyMask` x2,
  `supports_xkb` x1, `XkbGetState` x1, `XkbStateRec` x1 and the seven shared rows).
* **Two new arms, and why they are this slice's.**  `AXKB = A + have_xkb` and `AXR = A +
  have_xrandr` are the only arms that can read C 12952's `#ifdef HAVE_XKB` block and C 12856's
  `#if defined HAVE_XRANDR || defined USE_GTK`, so without them the slice's XKB text would be
  unmeasured (the slice-100 reading's rule).  Their four new names (`XkbStateRec`, `XkbGetState`,
  `XtAppNextEvent`, `XIGetClientPointer`) are the third-party-SDK class this ledger's Owed 0
  describes: no XKB, libXt or XInput2 header exists on this machine, so the port writes C's names
  and the arms record them.
* **Five defects the first writing read, and the mirror row they fixed.**  (1) Seven of this file's
  `static` helpers (`x_free_dnd_targets`, `x_free_dnd_toplevels`, `x_clear_dnd_variables`,
  `x_clear_dnd_action`, `x_dnd_delete_action_list`, `x_dnd_lose_ownership`,
  `x_dnd_cleanup_drag_and_drop`) have their **address taken** by the callbacks C 12768/12769/12787/
  12805/12852/12975/13106 pass them to, so PORTING.md 1.4's rule applies and all seven become
  `unsafe extern "C" fn` (the `src/androidmenu.rs` 213 shape).  (2) Three of those call sites are
  `record_unwind_protect*` with an `Option<...fn>` parameter, so the calls need `Some (f)` - and
  the mirror's own `DEFER_SELECTIONS!` macro (`include/xterm.rs` 1556-1563) passed its helper
  *unwrapped*: that box compiled only because the name was unresolved before, and the fix is the
  mirror row `record_unwind_protect_void (Some (x_release_selection_requests_and_flush))` (P-87's
  two legs were run for it: `check-mirror-dupes` 16/16/0 and `check-all-units.sh` `83 unit(s), 0
  failure(s)`).  (3) C's `None` is an `X11` `long` and the port's `Window`/`Atom` are unsigned, so
  six state-initialisation stores and the `x_dnd_action != None` test carry `as Window`/`as Atom`
  (the file's own `None as Window` precedent, 21988).  (4) C's implicit narrowings are the port's
  casts: `XSETINT!`'s four `x_dnd_movement_*`/`x_dnd_wheel_*` reads (`as isize`, the mirror's
  `XSETINT` feeds `make_fixnum`), `make_fixnum (x_dnd_wheel_button as isize)`,
  `make_uint (x_dnd_wheel_state as u64)`, `x_dnd_recursion_depth = (command_loop_level +
  minibuf_level) as c_int`, `XFIXNUM (...)` twice into `x_dnd_do_unsupported_drop`'s `int`
  parameters, `kbd_buffer_store_event (&raw mut hold_quit as *mut c_void)` (the port's signature
  takes `*mut c_void`) and `(*FRAME_DISPLAY_INFO! (f)).Xatom_XdndActionDescription` (the field of
  the struct the macro returns a pointer to).  (5) `hold_quit.kind`/`xg_pending_quit_event.kind`
  are C's `bool_bf`-style packed fields in the port (`input_event.bits`, PORTING.md 1.3), so all
  five sites read and write through `input_event_bits!`; and `x_dnd_pending_send_position.type`
  keeps C's field name as the raw identifier `.r#type` (the file's `msg.xclient.r#type` spelling).
* **What the arm does not read, and the port's spelling for it.**  C's `#if defined HAVE_XRANDR ||
  defined USE_GTK` guards only *part* of C 12854's block (the cached monitor list and its `NILP`
  test), and C 12862-12863 is the tail both arms share - the port writes that tail once per arm.
  C's `#else` half at C 13254-13264 exists in the non-GTK build only and an attribute cannot sit
  on `else`, so the arm's *body* is the cfg'd block (an empty `else` in the GTK build is what C's
  absent arm means); C's `if (event_display == FRAME_DISPLAY_INFO (f))` - the guard the
  `#ifndef USE_GTK` at C 13073 opens and the `#else` at C 13252 closes - is likewise lifted into a
  cfg'd `let` (`event_display_is_ours`: `true` in the GTK build is what C's absent `if` means),
  because a `#[cfg]` on an expression is `E0658`.
* **What `--check` reads.**  **5 -> 4** as above, with the line-count row at **30,214** against C's
  33,125 (8.8% drift, was 29,493 / 11.0%).

### 120. Slice 107 (C 30466-30489): the toolkit resource table, and the two `<X11/Xresource.h>` records it was owed for

* **What it lands.**  `emacs_options[]` (C 30467-30483, eleven `XrmOptionDescRec` entries) and
  `x_timeout_atimer_activated_flag` (C 30487), both inside C 30466's `#ifdef USE_X_TOOLKIT`, so the
  port carries them under `#[cfg (feature = "use_x_toolkit")]` at C's own position - replacing the
  comment that recorded the region as owed (Decision 57).  C 30469's `(XtPointer) "yes"` is Xlib.h
  76's `char *`, and each entry keeps C's line in its own `// C NNNN` comment.
* **The mirror round that had to come first.**  The region was owed *because*
  `XrmOptionKind`/`XrmOptionDescRec` (Xresource.h 327-343) were not in the mirror; they are now,
  measured on this machine's header: the enum's eight members in C's order are 0-7 with
  `sizeof = 4`/`align = 4` (what a `#[repr (C)]` enum with those discriminants is), and the record
  is `sizeof = 32`, `align = 8`, offsets `option 0 specifier 8 argKind 16 value 24`.  The second
  typedef `XrmOptionDescList` came with them.
* **What the probes read.**  UT 829/73, UM 830/73, A 830/74 and U 843/74 - **identical to the
  pre-slice readings**, because the table resolves completely once the two records are in the
  mirror: no new error row and no moved warning in either toolkit arm.  That is what a *text* debt
  should read like; the behaviour is still C's table handed to `XtOpenDisplay` at C 30821, which
  `x_term_init` will land.
* **P-87's two legs.**  `check-mirror-dupes` 16/16/0 and `tools/check-all-units.sh`
  `checked 83 unit(s), 0 failure(s)` + `checking 83 unit(s), 0 unit failure(s)`.
* **Not claimed.**  The frozen configuration compiles none of this (`USE_X_TOOLKIT` is `undef`
  there); only the UT and UM arms compile the table, and their readings are the line above.

### 121. Slice 108 (C 985-1130): `x_atom_refs`, and the blocker Decision 9 recorded was already gone

* **What it lands.**  C 987-1130's `static const struct x_atom_ref x_atom_refs[]` - the 121
  `ATOM_REFS_INIT` rows C writes, in C's order, each on one line with its own `// C NNNN` (the six
  rows C hard-wraps are one line here, cited at the first line's number).  Its readers landed in
  slices 39 and 41 (`x_intern_cached_atom` at C 29962, `x_intern_atoms` at C 30050), which is why
  the table was this file's most-named owed object.
* **Decision 9's blocker had been closed under it.**  That decision deferred the table because the
  mirror carried 61 of the 115 `Xatom_*` members it names.  Measured now: xterm.h declares 123
  `Xatom_*` members and **all 123 are in the mirror** (the slices since, and status 3.902's round,
  carried the rest), and every one of the 121 rows' members resolves.  The table therefore lands -
  and Decision 9's "it opens the next slice" is answered by saying what actually blocked it.
* **C's `#ifdef HAVE_XKB` sits *inside* the table.**  C 1073-1079 puts five rows (Meta, Super,
  Hyper, ShiftLock, Alt) in the middle of the other 116, so the table's *length* is
  per-configuration (121 with XKB, 116 without) and its *order* differs too.  A `#[cfg]` cannot sit
  on an array element (`E0658`), and writing the 116 shared rows twice would put C's text in the
  file twice, so the rows live once inside `def_x_atom_refs!`, which each arm instantiates with C's
  five rows spliced at C's own position.
* **The names are C strings.**  C's `ATOM_REFS_INIT` takes a string literal; the port's macro stores
  `*const c_char` (its readers hand `name` to `strcmp`), so every row passes `c"…".as_ptr ()` - the
  file's C-string spelling.
* **What the probes read.**  **Every one of the seventeen arms retires exactly six rows** - the six
  `cannot find value x_atom_refs` sites the unit's two readers carry: A 830/74 -> **824/74**, U 843
  -> **837**, UT 829/73 -> **823/73**, UM 830 -> **824**, UG 822/75 -> **816/75**, UGX 901/77 ->
  **895/77**, AE 862 -> **856**, AI 839 -> **833**, AIU 874 -> **868**, AIUX 957 -> **951**, AX 844
  -> **838**, AG3 876 -> **870**, AXF 832 -> **826**, AWS 515/0 -> **509/0**, AXPM 831 -> **825**,
  AXKB 850 -> **844** and AXR 831 -> **825** - and **no warning moved in any arm**.
* **What `--check` reads.**  Unchanged at four rows (`handle_one_xevent`, `x_term_init`, the `false`
  row and the drift row), with the line count at 30,404 against C's 33,125 (8.2%).
### 122. Slice 111 (C 19473-19561): the switch's two labels, and the first three `case` arms

* **What it lands.**  `handle_one_xevent`'s `OTHER:` block (C 25558-25587) and `done:` block
  (C 25590-25595), then `SelectionNotify` (C 19473-19486), `SelectionClear` (C 19487-19515) and
  `SelectionRequest` (C 19516-19561), each line with its own `// C NNNN` comment.  C's `case` order
  starts at `ClientMessage` (C 18951-19472); that 522-line XDND/Motif state machine is a slice of
  its own, and no arm of the port's `match` depends on arm order.
* **Why the two labels land here, and where.**  C's `OTHER:` sits *inside* the `GenericEvent`
  case, but it is the switch's shared tail: thirty sites `goto OTHER` (C 19254, 19372, 19385,
  19458, 19476, 19490, 19519, 19803, 19861, ...) and the last case falls into it.  Rust has no
  forward jump, so the text is carried **once**, at the end of the `match`, inside a labelled
  `'other:` block; a site that C sends there is `break 'other`, which runs the block and then C's
  own `break` - the same sequence C's control flow produces.  The block reads only function-scope
  names (`*finish`, `event`, `use_copy`, `copy`), so moving it cannot change what it sees.
  `done:` (C 25590) is its mirror: the six lines every `goto done` reaches *and* what the switch
  falls into when no case matches, as one labelled `'done:` block.
* **Three spellings the first writing read (each measured, each fixed in place).**  (1) C's
  `if (!x_window_to_frame (dpyinfo, ...))` is a *pointer* test in the port, so it is
  `.is_null ()` (the 2331/2387 spelling) - with C's own `!` kept, the three toolkit arms read
  `cannot apply unary operator ! to type *mut frame` x3.  (2) C 19501's `dpyinfo->motif_drag_atom
  = None` and C 19557's `x_dnd_action = None` need `None as Atom`: X11/X.h 115's `None` is `0L`
  (`c_long`) while both targets are `Atom`.  (3) `x_dnd_waiting_for_finish` is `bool` in the port
  and `x_dnd_waiting_for_motif_finish` is `c_int`, so C's `= false` and `== 2` are C's own types.
* **The union member's guard was narrower than C's, and this slice's arms are what read it.**
  C 19503-19506 and C 19527-19533 write `inev.sie` (keyboard.h 232-234's `struct
  selection_input_event sie;`, guarded by `#if defined HAVE_X11 || defined HAVE_PGTK`), and
  `include/keyboard.rs` carried that member behind `have_pgtk` alone - so in every arm this lane
  measures the thirteen uses were `E0609`.  The mirror now carries C's own condition
  (`any (feature = "have_x11", feature = "have_pgtk")`), which is a **measured no-op for every
  reading the tree carries today** (nothing enables either macro: `docs/baseline/feature-coverage.tsv`
  moves `have_x11` 7 sites / 2 files -> **8 / 3** and no arm's number).  The arm that reads the
  member is therefore new: **AX11 = A + `have_x11`**, the lane's seventeenth arm, where the
  thirteen rows retire and the record's own fields resolve.  One more thing the arm needed:
  `include/xterm.rs`'s `selection_input_event` had no `#[derive (Clone, Copy)]` (the pgtkterm.h
  mirror's twin at 597 has it), without which the union's member is two errors of its own
  (`the trait `Copy` cannot be implemented for this type` and the union's `field must implement
  `Copy` or be wrapped in `ManuallyDrop<...>``) - measured in AX11 before the derive was added.
  The three selection records C names (`XSelectionClearEvent` C 19496, `XSelectionRequestEvent`
  C 19525) are declared at the use site (P-88 1, the `src/xselect.rs` 202 shape, whose own unit
  declares the same pair), because `include/xlib.rs`'s `XEvent` is the measured 192-byte opaque
  stand-in; the four `X11/X.h` constants the arms and the tail read (`ConfigureNotify` 22,
  `SelectionClear` 29, `SelectionRequest` 30, `SelectionNotify` 31) joined `x11_constants`.
* **What `--check` reads.**  Unchanged at two rows - `handle_one_xevent`'s and
  `x_get_wm_check_window`'s - and the long row's own token list shrinks **531 -> 484** (the
  selection vocabulary and the tail's words the three arms now name).  The unit is 32,019 lines
  against C's 33,125 (**3.34%** drift, was 3.8%), file md5 `5b27bbc2c16fbd198a452f715dd103bc`.
* **Not claimed.**  No runtime evidence: this lane does not link, and the events these three arms
  handle never arrive here.  The readings are the seventeen arm compiles, the `--check` row and
  this ledger; the `sie` member's own type is still reached through a mirror the frozen
  configuration does not compile (`HAVE_X11` and `HAVE_PGTK` are both `#undef` there).


### 123. Slice 112 (C 19562-19862): `PropertyNotify` and `ReparentNotify`, and the eighteenth arm

* **What it lands.**  `case PropertyNotify:` (C 19562-19804, 243 lines) and `case ReparentNotify:`
  (C 19805-19862, 58 lines), each line with its own `// C NNNN` comment.  C's
  `for (last = NULL, tem = x_dnd_toplevels; tem; last = tem, tem = tem->next)` becomes the
  `while !tem.is_null ()` whose two increments stand at the body's end, so the `break` inside the
  matching `if` leaves the loop exactly where C's does.  The toplevel `wm_state` reader, the two
  frame-state readers (`_NET_WM_STATE` and `WM_STATE`), the `_NET_WM_WINDOW_OPACITY` reader with
  C's two `USE_XCB` arms, the root-window toplevel-list refresh and the two handlers C ends on all
  land in C's order.
* **Six spellings the first writing read (each measured, each fixed in place).**  (1)
  `FRAME_PARENT_FRAME` is a *function* returning `*mut frame` (include/frame.rs 826), so C's
  `!FRAME_PARENT_FRAME (f)` is `FRAME_PARENT_FRAME (f).is_null ()`; `SET_FRAME_ICONIFIED` is a
  function taking `c_int` (1417), so C 19639/19662's `false`/`true` are `false as c_int` /
  `true as c_int`, while `SET_FRAME_VISIBLE` **is** the macro this file already calls.  (2) `XFree`
  takes `*mut c_void`, so C 19611/19777's `*mut c_uchar` arguments are cast (the 3652 idiom).
  (3) The declaration at 15859 has `AnyPropertyType` as a `c_long` where the parameter is an
  `Atom`, and C's `False` argument is this file's `0` (15855's own rule): both
  `XGetWindowProperty` calls spell `0, … , AnyPropertyType as Atom`.  (4) `x_output` is reached
  through the `as *mut x_output` cast the file already uses (8201/8232), so the record's own
  `parent_desc` resolves and only the *bitfields* stay owed - include/xterm.rs 821-928 records
  those six members as `// C:` lines rather than fields, which is why `has_been_visible` x2,
  `explicit_parent` x4 and `alpha_identical_p` x1 are rows in this slice's reading.
  (5) C's `&& n` on an `unsigned long` is `n != 0`, and C's `FRAME_OUTPUT_DATA (f)->parent_desc`
  truth test on a `Window` is `!= 0` rather than `is_null ()`.  (6) `WithdrawnState`
  (X11/Xutil.h 265) is declared at unit scope as `c_ulong`, the type of the field C assigns it to.
* **The mirror round.**  `x11_constants` gains `PropertyDelete` (X11/X.h 446); the unit declares
  `WithdrawnState`; C 19801's `x_handle_property_notify` stands at the use site (xterm.h 1919, its
  definition is `src/xselect.rs` 2025).  C 19802's `xft_settings_event` needs no row of its own:
  `include/xsettings.rs` 38 already declares it for every includer, and the first writing's second
  declaration read `E0428 the name xft_settings_event is defined multiple times` (measured).
* **The eighteenth arm, and the defect it found.**  C 19684-19778's `#ifndef USE_XCB` / `#else`
  pair had no arm at all: none of the seventeen spells `use_xcb`, so the block had never been
  compiled by anything.  This slice adds **AUX = A + `use_xcb`** to read it - and the arm's first
  reading was not a number: HEAD's `{ SAFE_FREE! (); }` in `x_dnd_compute_toplevels` (C 3607's
  text) stops rustc at *macro expansion*, so the arm read one error and never reached type
  checking.  The fix is the shape status 3.374 already spelled for this same site
  (`SAFE_FREE! (sa_count)`, the port's macro taking the argument C's `sa_count` supplies
  implicitly), and after it the arm type-checks at **1,058** rows.  Measured: the empty form was
  added by `ac9e3b37` (slice 6) and never removed, so 3.374's "改后 564" was taken on a copy that
  did not land; this round lands it.  The rest of the arm's 1,058 rows are the file's owed classes
  (the X11 entry points, the `xlib_surface::XEvent` stand-in's members, the `c_void` model), not
  this slice's text.  The three other `SAFE_FREE! ()` sites the port carries are in
  `femacs/src/sysdep.rs`, another unit's text behind its own arms, and this arm does not reach
  them.
* **What `--check` reads.**  Unchanged at two rows (`handle_one_xevent`'s long token row and
  `x_get_wm_check_window does not mention false`); the long row's list goes **484 -> 388**.  The
  unit is **32,389 lines** against C's 33,125 (**2.22%** drift, was 3.34%), file md5
  `fbc0d1e591673668ae46e1489ca84922`.

## Evidence

Readings, in the order they were taken (round 188, 2026-10-04; commands verbatim).

```
$ python3 tools/check-cond-eval.py src/xterm.c
src/xterm.c: 852 conditions, 0 unknown, 0 disagreements [VERIFIED]
```

```
$ python3 tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs    # before slice 1,
                                                                               # with the file
                                                                               # written as a unit
                                                                               # (one probe `fn`)
460
FAIL emacs/src/xterm.c -> femacs/src/xterm.rs
```

```
$ python3 tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs    # after slice 1
443
FAIL emacs/src/xterm.c -> femacs/src/xterm.rs
```

```
$ grep -c '^- ' <that output>          # 443
$ grep -c '^\- \[ \]' docs/ports/xterm.md   # 425 -> 413 after ticking slice 1
```

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-g188-j cargo run -q -p xtask -- build-tu src/xterm.c
built src/xterm.rs -> ../build/femacs/xterm.o
```

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-g188-j cargo run -q -p xtask -- layout-check
layout probe ok
layout-check passed (see build/femacs/layout_asserts.rs)
```

The three readings of Decision 2, from `tools/cinventory.py` with the frozen table, with that
table overridden to an X11/GTK3 set, and with no table at all (blind):

```
frozen  {'defuns': 0, 'functions': 310, 'macros': 21, 'constants': 48, 'globals': 80, 'struct_fields': 78}
routeA  {'defuns': 0, 'functions': 389, 'macros': 32, 'constants': 61, 'globals': 99, 'struct_fields': 86}
blind   {'defuns': 0, 'functions': 433, 'macros': 35, 'constants': 62, 'globals': 105, 'struct_fields': 86}
```

and their live-line counts (`dead_branch_map`):

```
frozen live 19809   routeA live 26914 (adds 9137, loses 2032)   blind live 33006   total 33126
```

The X11 arm's own type check, as a probe rather than a gate (the unit materialised by
`build-tu`, then `rustc --cfg 'feature="have_x_windows"'`):

```
$ rustc --edition 2024 --crate-type lib --emit obj -C panic=abort \
      --cfg 'feature="have_x_windows"' -o /tmp/xg188/xterm_probe.o build/femacs/units/xterm/_root.rs
144 errors, 4 of them in the unit's own text:
  1x `x_push_selection_request` not found            (C 12573, a later slice)
  1x `x_handle_pending_selection_requests` not found (xterm.h 1857, gap of Decision 5)
  2x no field `sie` on `buffered_input_event`        (keyboard.h 228-235, gap of Decision 5)
the other 140 are in `include/xterm.rs` and the other mirrors, all unresolved X11-header names
(`KeyPressMask`, `Pixmap`, `Display`, `Screen`, `XVisualInfo`, ...).
```

The probe is what caught two real defects in the first draft of the slice (`build-tu` cannot:
it strips the whole body): the module's statics had to be `pub` for the glob import to reach
them (4x E0425), and C's `event->ie.kind` is a read of `input_event`'s bitfield, so the accessor
takes `&raw mut (*event).ie` rather than the union (E0609).

Slice 2 (C 1131-1762), same readings:

```
$ python3 tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs
324
FAIL emacs/src/xterm.c -> femacs/src/xterm.rs
$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-g188-j cargo run -q -p xtask -- build-tu src/xterm.c
built src/xterm.rs -> ../build/femacs/xterm.o
$ ... layout-check
layout probe ok
layout-check passed (see build/femacs/layout_asserts.rs)
$ grep -c '^- \[ \]' docs/ports/xterm.md        # 425 - 91
334
```

The X11-arm probe again, after slice 2 (same command as above, re-run):

```
159 errors, 19 of them in the unit's own text:
  8x `Window`, 4x `Atom`, 1x `XWindowAttributes`, 1x `XRectangle`, 1x `XEvent`
       (slice 2's kept X11 names - Decision 8)
  2x no field `sie` on `buffered_input_event`          (slice 1's gap, Decision 5)
  1x `x_push_selection_request` not found              (C 12573, a later slice)
  1x `x_handle_pending_selection_requests` not found   (xterm.h 1857, slice 1's gap)
```

The three defects the probe and the gate caught in slice 2's draft, all fixed before landing:
the extractor reads a macro's body from the **first** `macro_rules!` of that name (so
`SWAPCARD32`'s `#else` arm had to come first, or it reported "does not mention bswap_32");
`XM_DRAG_PROTOCOL_VERSION` had to be a `const` (the extractor reads it as a constant); and
`SWAPCARD16`'s live arm takes C's parameter name `l`, not `s` (C 1584).

Slice 3 (C 1763-2173):

```
$ python3 tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs
317
FAIL emacs/src/xterm.c -> femacs/src/xterm.rs
  311-316: six "does not mention" rows, one per function, for uint8_t/uint16_t/uint32_t/alloca
           (the six notes rows Decision 11 requests)
$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-g188-j cargo run -q -p xtask -- build-tu src/xterm.c
built src/xterm.rs -> ../build/femacs/xterm.o
$ ... layout-check
layout probe ok
layout-check passed (see build/femacs/layout_asserts.rs)
$ python3 tools/check-link.py
libc-or-platform=257 port-owed=15 rust-runtime=20 unknown=92
384 undefined symbol(s) from 76 program + 87 libgnu object(s), 384 ratchet row(s), 0 failure(s)
$ python3 tools/check-link.py --report | grep port-owed      # all fifteen are xdisp's, not xterm's
_count_lines, _disp_char_vector, _display_tty_menu_item, _get_font_ascent_descent, _init_xdisp,
_invisible_prop, _mark_window_display_accurate, _note_mouse_highlight, _produce_stretch_glyph,
_redisplay_preserve_echo_area, _row_hash, _syms_of_xdisp, _try_window,
_unwind_display_working_on_window, _update_redisplay_ticks
```

The X11-arm probe (the unit materialised by `build-tu`, then `rustc --cfg
'feature="have_x_windows"'`): **107 errors in the unit's own text, and every one is a recorded
gap** - 94 `cannot find <name>` (X11 names, or functions this file defines in a later slice) and
13 `no field <name>` (`x_display_info`'s missing members, `buffered_input_event.sie`); the other
139 are in the mirrors, plus `include/xterm.rs`'s own
`assert! (!cfg!(feature = "have_x_windows") || HAVE_X_WINDOWS)` const assertion, which is the
mirror refusing to be flipped before its target exists.

That probe caught three real defects in this round's draft, none of which `build-tu` can see:
**65 of slice 2's 66 objects were not `pub`** (a `static` in the private module is not reachable
through the glob import - E0425 in 20 places), C 1808's *chained assignment* was written as
`(a = b) as u8` (E0605: Rust has no assignment expression), and `-> !` on
`xm_drag_window_io_error_handler` needs a diverging final expression (E0308).

Slice 4 (C 2175-2612) and the mirror addition:

```
$ python3 tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs
314
FAIL emacs/src/xterm.c -> femacs/src/xterm.rs
  305-313: nine "does not mention" rows (uint8_t/uint16_t/uint32_t, alloca, goto, uninit)
$ python3 tools/extract-abi.py --check emacs/src/xterm.h femacs/src/include/xterm.rs
OK emacs/src/xterm.h -> femacs/src/include/xterm.rs
$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-g188-j cargo run -q -p xtask -- build-tu src/xterm.c
built src/xterm.rs -> ../build/femacs/xterm.o
$ ... layout-check
layout probe ok
layout-check passed (see build/femacs/layout_asserts.rs)
```

The X11 constants, as measured (`/opt/homebrew/include` is the `xorgproto` headers this machine
has; the reference tree defines none of them):

```
$ printf '#include <X11/X.h>\n#include <X11/Xatom.h>\n#include <X11/Xproto.h>\n' | clang -E -dM -I /opt/homebrew/include -x c - | grep -E '^#define (None|Success|...) '
#define None 0L                                  X11/X.h 115
#define Success 0                                X11/X.h 350
#define XA_WINDOW ((Atom) 33)                    X11/Xatom.h 41
#define StructureNotifyMask (1L<<17)             X11/X.h 168
```

The X11-arm probe after slice 4: **169 errors in the unit's own text, every one a recorded gap**
(148 `cannot find <name>` - X11 names, or functions this file defines later - and 20
`no field <name>` - the mirror's missing members); the other 153 are in the mirrors.  The probe
caught three real defects in this round's draft, none visible to `build-tu` or `--check`:
`msg.xclient.type` is Rust's keyword (`r#type`; the `r#` prefix is stripped by the token
comparison, which is why the port keeps C's name), C's `False`/`True` had to become Rust's
`false`/`true` (case folding keeps the tokens), and `xm_get_drag_window_1`'s import list was
missing `PropModeReplace`.

`tools/check-link.py` reports `1 failure(s), 1 stale`: the failure is `_cursor_row_p` and the
stale row is `_set_cursor_from_row`, both xdisp's (`femacs/src/xdisp.rs` is modified in this
working tree) - nothing xterm's, and `docs/baseline/link-pending.txt` was not touched here.

Slice 5 (C 2614-3012):

```
$ python3 tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs
313
FAIL emacs/src/xterm.c -> femacs/src/xterm.rs
  296-312: seventeen "does not mention" rows; the nine "missing function" lines for this slice
            are gone (grep for them: 0)
$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-g188-j cargo run -q -p xtask -- build-tu src/xterm.c
built src/xterm.rs -> ../build/femacs/xterm.o
$ ... layout-check
layout probe ok
layout-check passed (see build/femacs/layout_asserts.rs)
```

The X11-arm probe after slice 5: **two errors in the unit's own text**, both the recorded
`buffered_input_event.sie` (Decision 5); every other in-unit error is a `cannot find <name>` for
an X11 name or a function this file defines later, and the mirror side carries 219 of the same
class.  The probe is what caught this round's real defects: fifteen enum/width errors from the
`XM_DRAG_*` macros (fixed by making them int-typed, above) and, before that, the same class of
"inert text that would not compile when flipped" that `build-tu` cannot see.

Slice 6 (C 3013-3614) and the `x11_ext` members:

```
$ python3 tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs
300
FAIL emacs/src/xterm.c -> femacs/src/xterm.rs
  earliest missing: "missing function XTcondemn_scroll_bars"; the five "does not mention" rows
  now read (295-299): xm_find_targets_table_idx/xm_write_targets_table/
  xm_write_drag_initiator_info -> alloca; xm_setup_dnd_targets -> goto, uninit;
  x_dnd_free_toplevels -> display, uninit
$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-g188-j cargo run -q -p xtask -- build-tu src/xterm.c
built src/xterm.rs -> ../build/femacs/xterm.o
$ ... layout-check
layout probe ok
layout-check passed (see build/femacs/layout_asserts.rs)
$ python3 tools/extract-abi.py --check emacs/src/xterm.h femacs/src/include/xterm.rs
OK emacs/src/xterm.h -> femacs/src/include/xterm.rs
$ python3 spikes/mirror-struct-coverage.py
284 mirrored struct(s): 1136 arm(s) attempted, 320 unreadable (...), 0 gap(s)
```

The X11-arm probe after slice 6: **two errors in the unit's own text**, both the recorded
`buffered_input_event.sie`; everything else is a `cannot find <name>` for an X11/xcb name or a
function this file defines later, with 226 of the same class on the mirror side.  Slice 6 needed
no fix from the probe - the three defects it caught in the earlier slices (the `r#type` keyword,
C's `False`/`True`, the `XM_DRAG_*` widths) had already settled the shapes this one reuses.

### The Xlib mirror and the `MonitorInfo` acceptance (Decision 16), as landed and measured

```
$ python3 tools/extract-abi.py --check emacs/src/xterm.h femacs/src/include/xterm.rs
OK emacs/src/xterm.h -> femacs/src/include/xterm.rs      (the constants moved out; C 23's
                                                          include of <X11/Xlib.h> is in)
$ python3 tools/extract-abi.py --check emacs/src/dispextern.h femacs/src/include/dispextern.rs
OK emacs/src/dispextern.h -> femacs/src/include/dispextern.rs
$ python3 spikes/mirror-struct-coverage.py
284 mirrored struct(s): 1136 arm(s) attempted, 320 unreadable (...), 0 gap(s)
   (the tool's struct scan matches lower-case names only, so `struct XRectangle` is not one of
    its structs and never asks for a C header - a fact worth knowing, not a gap)
$ python3 spikes/gated-layout-probe.py 'struct MonitorInfo' frame.h
## frozen      unavailable (invalid application of 'sizeof' to an incomplete type)
## x11         [sizeof=32, align=8]
## ns_cocoa    [sizeof=48, align=8]
## x11_ext     [sizeof=32, align=8]
$ cd femacs && CARGO_TARGET_DIR=/tmp/... cargo run -q -p xtask -- build-tu src/xterm.c
built src/xterm.rs -> ../build/femacs/xterm.o
$ ... layout-check
layout probe ok
layout-check passed (see build/femacs/layout_asserts.rs)
$ python3 tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs
300        (unchanged: the earliest missing is still `missing function XTcondemn_scroll_bars`,
            and the five "does not mention" rows are the ones decision 15 records)
```

The X11-arm probe (the unit materialised by `build-tu`, then `rustc --cfg
'feature="have_x_windows"'`): the unit's own text now has **two** errors, both the recorded
`buffered_input_event.sie`, and the four macros slice 6 left bare - `AnyPropertyType`,
`XA_CARDINAL`, `IsUnmapped`, `PropertyChangeMask` - are gone from the list because they are in
`xlib.rs`'s `x11_constants` module now and slice 6's `use` takes them.


### Decision 17's readings: the Xrm surface, and what it moved

The two record layouts, from a clang probe built against the installed headers
(round 188, 2026-10-05; commands verbatim, probe written to `/tmp/xg188-xrm/` and deleted after):

```
$ clang -std=gnu23 -I /opt/homebrew/include -o probe probe.c && ./probe
XrmValue   sizeof 16 align 8 size@0 addr@8
XRectangle sizeof 8 align 2
XrmQuark 4  Bool 4  XPointer 8  XrmName 4  XrmClass 4  XrmRepresentation 4
```

The same probe source, linked: the declarations are right and the symbols are not on this machine
(only `libX11`'s *headers* are installed; the two macro calls expand to the same real procedure,
which is why `_XrmStringToQuarkList` is listed twice):

```
$ clang -I /opt/homebrew/include link2.c -o link2
Undefined symbols for architecture arm64:
  "_XResourceManagerString", ... "_XrmGetFileDatabase", "_XrmGetStringDatabase",
  "_XrmInitialize", "_XrmMergeDatabases", "_XrmPutLineResource", "_XrmQGetResource",
  "_XrmStringToQuark", "_XrmStringToQuarkList" (twice, the two macros), "_main"
ld: symbol(s) not found for architecture arm64
```

`DefaultScreenOfDisplay` is the one name the linker does **not** list - C's macro expands inline
through `_XPrivDisplay`:

```
$ nm -u link.o     # the probe that calls DefaultScreenOfDisplay (d)
  (no _XDefaultScreenOfDisplay: the macro names no symbol)
```

The gates after the change (all re-run in this round; the file's own readings):

```
$ python3 tools/extract-abi.py --check emacs/src/xterm.h femacs/src/include/xterm.rs
OK emacs/src/xterm.h -> femacs/src/include/xterm.rs
$ python3 tools/extract-abi.py --check emacs/src/dispextern.h femacs/src/include/dispextern.rs
OK emacs/src/dispextern.h -> femacs/src/include/dispextern.rs
$ python3 tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs
300                      # unchanged by this mirror: it is not a unit (first row
                        # `missing function XTcondemn_scroll_bars`)
FAIL emacs/src/xterm.c -> femacs/src/xterm.rs
$ python3 tools/check-mirror-dupes.py
67 duplicate declaration(s), 111 ratchet row(s), 44 failure(s)
# red tree-wide, and none of it is this lane: all 44 failures are STALE ratchet rows of
# `femacs/src/xdisp.rs` (12 against `include/dispextern.rs`, 32 against `include/window.rs`) -
# the xdisp lane's "shadowing macro removed" slice (status 3.325) plus this gate's new macro
# namespace (status 3.326), both landed while this round was in flight.  This lane's file is in
# no row of the ratchet and in no failure:
$ grep -cE 'xlib|XrmStringToNameList|XrmStringToClassList|DefaultScreenOfDisplay' docs/baseline/mirror-dupes.tsv
0
# (With the gate as it stood at the top of this round - before 3.326 - the same tree read
#  `1 duplicate declaration(s), 1 ratchet row(s), 0 failure(s)`, the single row being
#  `FONT_INVALID_CODE`; this round does not touch that row.)
$ python3 tools/check-target-names.py
check-target-names: 628 .rs files, 591 map targets, 0 finding(s)
$ python3 tools/check-declarations.py
13 declaration sites (0 forbidden, 13 allowed literals), 0 failures
$ grep -c '^- \[ \]' docs/ports/xterm.md     # unchanged: a mirror has no checklist row
305
$ python3 spikes/mirror-struct-coverage.py
284 mirrored struct(s): 1136 arm(s) attempted, 320 unreadable (...), 0 gap(s)
$ python3 spikes/gated-layout-probe.py 'struct MonitorInfo' frame.h
## frozen       unavailable (incomplete type: this configuration has no window system)
## x11          [sizeof=32, align=8]      (XRectangle geom: 4 shorts, 8 bytes)
## ns_cocoa     [sizeof=48, align=8]
## x11_ext      [sizeof=32, align=8]
# re-taken after status 3.327 changed the probe's tables; the x11 arm still reproduces 32/8, so
# Decision 16's acceptance ("the record's arm is reproduced, not recorded") still holds.
```

`build-tu` and the x11 arm (the unit materialised by `build-tu`, then compiled with the feature
on):

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-g188-j cargo run -q -p xtask -- build-tu src/xterm.c
built src/xterm.rs -> ../build/femacs/xterm.o
$ ... layout-check
layout probe ok
layout-check passed (see build/femacs/layout_asserts.rs)
$ rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type lib --emit obj -C panic=abort \
      --cfg 'feature="have_x_windows"' -o /tmp/xg188-xrm/xterm_probe.o build/femacs/units/xterm/_root.rs
432 errors before the `use core::ffi::{c_int, c_long, c_ulong};` line, 411 after (exactly the
twenty-one scoping errors, all of them `xlib.rs` lines 50-91, all inside `x11_constants`);
**xlib.rs now contributes 0 errors**, and the 400 remaining `E0425` spell 47 distinct names, the
largest `Atom` 161, `Window` 64, `Cursor` 19, `XEvent` 10, and the six `x_*` error/selection
helpers this lane's later slices define (`x_had_errors_p` 10, `x_catch_errors` 7,
`x_ignore_errors_for_next_request` 6, `x_stop_ignoring_errors` 6, `x_uncatch_errors_after_check`
4, `x_uncatch_errors` 3); the rest are one to six each - Decision 5's and Decision 13's recorded
gaps, not new ones.
```

One shared-table row has to move because of Decision 16's insertion (this lane's earlier round):
`docs/baseline/citations-allowlist.tsv` line 37 names `femacs/src/include/dispextern.rs:567`, and
the finding it explains is now at **587** - the same 20 lines Decision 16 added above it.  Measured:

```
$ python3 tools/check-citations.py
stale	femacs/src/include/dispextern.rs:567	dispextern.h 648-661 is right; `struct glyph *` is a parameter type	the row no longer matches a finding
drift	femacs/src/include/dispextern.rs:587	dispextern.h 648-661	`struct glyph *` is at dispextern.h:754
check-citations: 666 file(s) scanned, 40 drift, 1 range, 343 external, 37 allowed, 1 stale
```

The gate is red on that one stale row (a shared table, which this lane may not write): the fix is
`:567` -> `:587` on line 37 of the allowlist, reason unchanged.

The `XrmStringType` shape, measured on a scratch pair outside the repo (the C side is the macro
alone, so `tools/extract-abi.py` compares the constant's bytes; `--reference`/`--root` point at
the scratch directory):

```
$ python3 tools/extract-abi.py --check /tmp/xg188-xrm/t.c /tmp/xg188-xrm/t1.rs --reference /tmp/xg188-xrm --root /tmp/xg188-xrm
OK /tmp/xg188-xrm/t.c -> /tmp/xg188-xrm/t1.rs      # pub const XrmStringType: &str = "String";
$ ... t2.rs                                        # pub const XrmStringType: &core::ffi::CStr = c"String";
OK /tmp/xg188-xrm/t.c -> /tmp/xg188-xrm/t2.rs
$ ... t3.rs                                        # the same, value c"Strung"
- constant XrmStringType: C "String" vs Rust c"Strung" (different bytes)
FAIL /tmp/xg188-xrm/t.c -> /tmp/xg188-xrm/t3.rs
$ ... t4.rs                                        # &str with "Strung"
- constant XrmStringType: C "String" vs Rust "Strung" (different bytes)
FAIL /tmp/xg188-xrm/t.c -> /tmp/xg188-xrm/t4.rs
```

### Slice 7's readings: the target-window discovery, as landed and measured

The `- ` reading, before and after, with the rows that moved (`extract-abi.py --check`, then
`grep '^- '` on each output and `diff`):

```
$ python3 tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs   # before this slice
300
$ ...                                                                          # after it
293
$ <the two `grep '^- '` outputs, diffed>
< - missing function x_dnd_io_error_handler            # the seven functions
< - missing function x_dnd_get_target_window_1
< - missing function x_dnd_get_wm_state_and_proto
< - missing function x_dnd_do_unsupported_drop
< - missing function x_dnd_send_unsupported_drop
< - missing function x_dnd_fill_empty_target
< - missing function x_dnd_get_target_window
< - missing macro X_DND_SUPPORTED_VERSION              # the constant
< - missing constant X_DND_SUPPORTED_VERSION
< - line count drifted 89.4%: C 33125 vs Rust 3510
> - line count drifted 86.3%: C 33125 vs Rust 4524
> - function x_dnd_get_target_window_1 does not mention continue   # the `'body:` shape
> - function x_dnd_send_unsupported_drop does not mention alloca   # PORTING.md P-05
$ grep -c '^- \[ \]' docs/ports/xterm.md
305 -> 297                                                     # seven functions + one macro ticked
```

`build-tu` (twice in the round: the second run re-materialised the unit after the `E0695` fix
below) and the three shared gates, each on the final text:

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/xterm-s7 cargo run -q -p xtask -- build-tu src/xterm.c
built src/xterm.rs -> ../build/femacs/xterm.o
$ python3 tools/check-citations.py
check-citations: 675 file(s) scanned, 0 drift, 0 range, 350 external, 33 allowed, 0 stale
$ python3 tools/check-declarations.py
13 declaration sites (0 forbidden, 13 allowed literals), 0 failures
$ python3 tools/check-mirror-dupes.py
18 duplicate declaration(s), 18 ratchet row(s), 0 failure(s)
# the ratchet read 23 rows when this round opened and 18 after the two commits another lane
# landed while it was in flight (`72f935a` -> `334448a`); 0 failures in both readings, and this
# lane's file is in no row of either.
```

The X11 values the module does not carry, measured with `clang -E -dM -I/opt/homebrew/include`
over a probe that includes `<X11/X.h>` and `<X11/Xatom.h>`, plus the one `Xutil.h` line the probe
does not reach:

```
#define ButtonPress 4
#define ButtonPressMask (1L<<2)
#define ButtonRelease 5
#define ButtonReleaseMask (1L<<3)
#define IsUnmapped 0
#define IsViewable 2
#define XA_PRIMARY ((Atom) 1)
$ grep -n 'define NormalState' /opt/homebrew/include/X11/Xutil.h
147:#define NormalState 1	/* most applications want to start this way */
```

The composite arms are dead in the frozen reading (`tools/cinventory.py`'s `dead_branch_map`
over `src/xterm.c`, set up the way `extract-abi.py` sets it up):

```
4225 False '#if defined HAVE_XCOMPOSITE && (XCOMPOSITE_MAJOR > 0 || XCOM'
4226 False '      if (dpyinfo->composite_supported_p'
4245 False '          if (attrs.map_state == IsViewable)'
4258 False '                      return proxy;'
4377 False '#if defined HAVE_XCOMPOSITE && (XCOMPOSITE_MAJOR > 0 || XCOM'
4378 False '  if (child != dpyinfo->root_window)'
4381 True  '      if (child != None)'
4445 False '  if (child != None)'
```

The x11-arm probe (the unit materialised by `build-tu`, compiled with the feature on) is what
caught the three defects in this slice that no text gate can see:

```
$ rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type lib --emit obj -C panic=abort \
      --cfg 'feature="have_x_windows"' -o /tmp/s7/xterm_probe.o build/femacs/units/xterm/_root.rs
# with the slice as first written
error: aborting due to 505 previous errors
  492 E0425   7 E0133   2 E0609   2 E0308   1 E0080   1 E0695
# after `break 'toplevels` (E0695), the two `make_fixnum` widenings (E0308) and the thirteen bare
# `XM_DRAG_STYLE_NONE`s (E0425; the enum lives in `xm_drag_style`, so the port names it)
error: aborting due to 490 previous errors
  480 E0425   7 E0133   2 E0609   1 E0080
# of the 480 E0425, 81 fall inside this slice's lines and every one is a class already recorded:
# X11-header names with no mirror (`Window` 34, `Atom`, `XEvent`, `XFree` 2, `XGetAtomNames`,
# `XGetSelectionOwner`, `XGetWindowProperty`, `XSendEvent` 2, `XTranslateCoordinates` 2,
# `NormalState`, `XA_PRIMARY`, `ButtonPress`, `ButtonRelease`, `ButtonPressMask`,
# `ButtonReleaseMask`) and this file's own later slices (`x_dnd_get_window_proto` 7,
# `x_uncatch_errors` 6, `x_dnd_get_window_proxy` 4, `x_catch_errors` 3, `x_had_errors_p` 2,
# `x_top_window_to_frame` 2, and one each of `x_get_atom_name`, `x_get_window_below`,
# `x_ignore_errors_for_next_request`, `x_stop_ignoring_errors`, `x_tooltip_window_to_frame`,
# `x_uncatch_errors_after_check`).  The other classes are not this lane's: the 7 E0133 are
# `include/xterm.rs` 1458-1465's INLINE bodies outside an unsafe block, E0609 is Decision 5's
# pair, and E0080 is `include/systime.rs` 27's own frozen-configuration assertion.
```

### The `have_x_windows` header surface's readings (2026-10-05)

Round 18's Decision 19, measured.  `build-tu` (twice: before and after the mirror edits, since
`materialize` snapshots the include graph), then the plain flip arm, the mirror self-check and the
widest X11 arm the ported text names.  Commands verbatim; the error tables are `rustc`'s own.

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/x11fix cargo run -q -p xtask -- build-tu src/xterm.c
built src/xterm.rs -> ../build/femacs/xterm.o
```

```
$ rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type lib --emit obj -C panic=abort \
      --cfg 'feature="have_x_windows"' -o /tmp/x11fix-after.o build/femacs/units/xterm/_root.rs
error: aborting due to 490 previous errors
  480 E0425   7 E0133   2 E0609   1 E0080
# by file: _root.rs (the unit, src/xterm.rs) 288, include/xterm.rs 201, include/systime.rs 1 -
# byte-identical with the before reading, and that is the expected shape: every name this round
# adds is under `use_xcb`, under `have_xcomposite`, or inside `x11_constants`, and this arm
# enables none of them.  The eight sites the seven constants leave are listed in Decision 19③.
```

```
$ rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type lib --emit obj -C panic=abort \
      --cfg 'feature="have_x_windows"' --cfg 'feature="use_xcb"' --cfg 'feature="have_xcb_shape"' \
      --cfg 'feature="have_xcomposite"' --cfg 'feature="have_xcomposite_overlay"' \
      -o /tmp/x11fix-after-xcb.o build/femacs/units/xterm/_root.rs
# before the mirror landed, the same arm set printed one error and stopped there:
#   error: unexpected end of macro invocation  ->  `{ SAFE_FREE! (); }` (_root.rs:3504, C 3607)
# after it, with that one macro invocation patched **on the copy only** to C's own shape
# (`SAFE_FREE! (sa_count)`), the arm reaches type checking:
error: aborting due to 564 previous errors
  499 E0425   38 E0308   19 E0609   7 E0133   1 E0080
# of the 38 E0308 and 19 E0609: 30 + 16 are slice 6's `*mut c_void` model (Decision 19④), 8 E0308
# are slice 7's written-out conversions, and `cannot find` on an xcb *type or function* is 0 -
# the mirror's names all resolve.  The remaining xcb-named E0425 are the two enum members the port
# must path-qualify (`XCB_MAP_STATE_UNMAPPED`, `XCB_SHAPE_SK_BOUNDING`) and the other classes are
# the ones Decision 5 and the uncarried `x_display_info` members already own (`sie` x2,
# `xshape_supported_p` x1).
# by file: _root.rs 362, include/xterm.rs 201, include/systime.rs 1 - and **include/xlib.rs 0**:
# the mirror that carries the new xcb section contributes no error to the widest arm.
```

```
$ python3 tools/extract-abi.py --check emacs/src/xterm.h femacs/src/include/xterm.rs
OK emacs/src/xterm.h -> femacs/src/include/xterm.rs
$ python3 tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs | grep -c '^- '
286
$ python3 tools/check-citations.py
check-citations: 675 file(s) scanned, 0 drift, 0 range, 395 external, 33 allowed, 0 stale
# 395 external = 350 + this round's 45 header citations (X.h/Xatom.h/Xutil.h/xcb/*.h are not in
# this tree, so they are counted and never a failure - the class xlib.rs's first rows are already
# in)
$ python3 tools/check-declarations.py
13 declaration sites (0 forbidden, 13 allowed literals), 0 failures
$ python3 tools/check-mirror-dupes.py
18 duplicate declaration(s), 18 ratchet row(s), 0 failure(s)
$ python3 tools/check-target-names.py
check-target-names: 628 .rs files, 591 map targets, 0 finding(s)
$ python3 tools/check-dead-file-coverage.py
1 ledger(s) bound (a C range table), 0 failure(s)
$ python3 tools/check-citation-text.py | tail -1
checked 631 / mismatched 35 / ranges 221 / unresolved 98
# unchanged in kind: the 98 unresolved are the external-header citations (this round's 45 among
# them), and the 35 mismatched are other lanes' single-line quotes.
```

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/x11fix cargo run -q -p xtask -- build-tu src/xrdb.c
built src/xrdb.rs -> ../build/femacs/xrdb.o
$ ... build-tu src/dispextern.h
error: rustc failed for src/include/dispextern.rs (exit status: 1)
# 180 `cannot find type c_int`, 22 `ptrdiff_t`, 19 `Lisp_Object`, ... - the documented class of a
# *mirror compiled on its own* (PORTING.md 0.1-9: those names come from the unit's own includes;
# include-xterm.md records the same reading for src/xterm.h).  Not attributable to this round: the
# failing names are unit-root imports, and no version of xlib.rs supplied them.  `xrdb.c`, the
# consumer this mirror was built for, is `built`.
```

```
# the xcb mirror's own typecheck (not a unit flip arm: a scratch crate that includes xlib.rs only)
$ rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type lib --emit obj \
      --cfg 'feature="have_x_windows"' --cfg 'feature="use_xcb"' --cfg 'feature="have_xcb_shape"' \
      -o /tmp/xcb-mirror-probe.o /tmp/xcb-mirror-probe.rs
exit 0        # cookies passed by value, reply fields read, the iterator walked - no error
# ... and the layout assertions are live, not decorative:
$ sed 's/== 32)/== 999)/' femacs/src/include/xlib.rs > /tmp/xlib-flip.rs   # on the copy only
error[E0080]: evaluation panicked: assertion failed: core::mem::size_of::<xcb_get_property_reply_t>() == 999
```

```
# the layouts, measured against the installed headers (a C probe, compiled and run)
$ clang -I/opt/homebrew/include -o /tmp/xcb-layout-probe /tmp/xcb-layout-probe.c && /tmp/xcb-layout-probe
sizeof  xcb_get_property_cookie_t   4        alignof 4
sizeof  xcb_get_property_reply_t   32        alignof 4    format 1   type 8   value_len 16
sizeof  xcb_get_window_attributes_cookie_t   4          sizeof  xcb_get_window_attributes_reply_t 44
                                        alignof 4    map_state 26   colormap 28   your_event_mask 36
sizeof  xcb_translate_coordinates_cookie_t   4          sizeof  xcb_translate_coordinates_reply_t 16
                                        alignof 4    child 8    dst_x 12   dst_y 14
sizeof  xcb_get_geometry_cookie_t   4                    sizeof  xcb_get_geometry_reply_t 24
                                        alignof 4    root 8     width 16   height 18  border_width 20
sizeof  xcb_generic_error_t        36        alignof 4    error_code 1   resource_id 4
sizeof  xcb_shape_get_rectangles_cookie_t   4           sizeof  xcb_shape_get_rectangles_reply_t 32
                                        alignof 4    rectangles_len 8
sizeof  xcb_rectangle_t             8        alignof 2    x 0  y 2  width 4  height 6
sizeof  xcb_rectangle_iterator_t   16        alignof 8    data 0  rem 8  index 12
sizeof  xcb_visualtype_t           24        alignof 4    visual_id 0  _class 4  colormap_entries 6
const   XCB_MAP_STATE_UNMAPPED 0        XCB_SHAPE_SK_BOUNDING 0        XCB_SHAPE_SK_INPUT 2
```

```
# one more measurement Decision 19④ leans on: the port's `c_void` is one byte wide
$ rustc --edition 2024 /tmp/cvoid-size2.rs      # const asserts: size_of::<c_void>() == 1, align 1
asserts held
```

The two `c_void` readings above are what turn slice 6's `*mut c_void` cookie model from a style
question into a defect: `SAFE_NALLOCA!` sizes the block with `pointee_size ($buf)` and four-byte
cookies are stored into it.

The round's last edit of `include/xlib.rs` is a comment re-wording (the xcb section's header note
and one sentence of `x11_constants`' new comment).  The unit readings above were taken before it -
a doc comment cannot move an error set, and the mirror self-check was re-run after it and still
reads `exit 0` with zero errors.

### Slice 8's readings (C 4469-6989), as landed and measured (2026-10-05)

Commands verbatim; the before readings were taken on the tree as it stood when the slice opened.

```
$ python3 tools/check-cond-eval.py src/xterm.c          # before and after, unchanged
src/xterm.c: 852 conditions, 0 unknown, 0 disagreements [VERIFIED]
```

```
$ python3 tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs   # before
286
FAIL emacs/src/xterm.c -> femacs/src/xterm.rs
$ grep -c '^- \[ \]' docs/ports/xterm.md      # before: 297 (30 of them this slice's)
297
$ python3 tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs   # after
255
FAIL emacs/src/xterm.c -> femacs/src/xterm.rs
$ grep -c '^- \[ \]' docs/ports/xterm.md      # after: 267
267
# the rows that moved: every "missing function"/"missing constant OPAQUE" line of C 4469-6989 is
# gone (30 rows), and one row is *new*:
#   - line count drifted 86.3%: C 33125 vs Rust 4525   ->   - line count drifted 77.3%: C 33125 vs Rust 7508
```

```
$ python3 tools/extract-abi.py --all emacs/src/xterm.c femacs/src/xterm.rs     # after (the ledger's
                                                                               # WIP reading)
WIP  src/xterm.c -> femacs/src/xterm.rs (255 differences, docs/ports/xterm.md has unchecked boxes)
...
checked 590 ported files: 565 done, 3 in progress, 22 failing
# this file stays one of the three "in progress" rows (the ledger is still 267 ticks short).  The
# tree-wide counts are *not* this slice's: the before reading of the same command said
# `532 done, 3 in progress, 55 failing`, and the 33 files between the two readings are other
# lanes' in-flight edits (`femacs/src/image.rs`, `femacs/src/sfnt.rs`, `femacs/src/w32term.rs`,
# `femacs/xtask/src/main.rs`), which were mid-write when the before reading ran.
```

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/xterm-lane/target cargo run -q -p xtask -- build-tu src/xterm.c
warning: 22 warnings emitted
built src/xterm.rs -> ../build/femacs/xterm.o
# the 22 warnings are the mirrors' own (unused_attributes, unreachable_code); the frozen
# configuration compiles none of this slice.
```

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/xterm-lane/target cargo run -q -p xtask -- probe \
      src/xterm.rs /tmp/xterm-lane/x.rlib --crate-type=lib -C panic=abort \
      --cfg 'feature="have_x_windows"'
rustc failed for src/xterm.rs (exit status: 1)
# before: error: aborting due to 241 previous errors
#   151 E0425   84 E0609   4 E0308   1 E0381   1 E0080
# after:  error: aborting due to 339 previous errors
#   201 E0425  131 E0609   5 E0308   1 E0381   1 E0080
# by file unchanged: _root.rs (the unit, src/xterm.rs) and one E0080 in include/systime.rs
```

The delta, by name and by class - the table Decision 20 reads:

```
# E0425: 62 inside this slice's lines, every one a recorded class
#   Xlib names with no mirror: XChangeProperty 5, XFree 5, XSendEvent 4, XWindowAttributes 2,
#   XGetWindowProperty 2, XDeleteProperty 2, XFillRectangle 2, XSetForeground 2, Unsorted 1,
#   XGCValues 1, XGetWindowAttributes 1, XSelectInput 1, XGetGCValues 1, XGetAtomNames 1,
#   XDrawRectangle 1, XSetClipRectangles 1, XSetClipMask 1, XClearWindow 1, XQueryTree 1, XFlush 1
#   this file's later slices: x_ignore_errors_for_next_request 5, x_stop_ignoring_errors 5,
#   x_had_errors_p 3, x_top_window_to_frame 3, x_translate_coordinates 3, x_catch_errors 2,
#   x_uncatch_errors_after_check 2, x_atom_to_symbol 1, x_get_atom_name 1, x_clear_area 1
# and 12 go away: x_dnd_get_window_proto 7, x_dnd_get_window_proxy 4, x_dpyinfo 1
# E0609: 47 inside this slice's lines
#   xclient on XEvent 43, xconfigure on XEvent 2, r#type on XEvent 1   (the opaque-XEvent class)
#   menu_bar_window on frame 1                                        (the mirror's own
#                                                                       have_window_system gate)
```

```
$ python3 tools/check-dead-file-coverage.py
143 ledger(s) bound (a C range table), 0 failure(s)
$ python3 tools/check-ledger-ticks.py
238 ledger(s) bound (a function tick to compare), 0 failure(s), 0 carried by another file,
352 row(s) with no function tick - a unit with no function at all, or one whose body the frozen
configuration leaves under a false condition (check-port-lines.py is that class's gate),
0 row(s) without a ledger
$ python3 tools/check-citations.py
check-citations: 676 file(s) scanned, 0 drift, 0 range, 1814 external, 36 allowed, 0 stale
$ python3 tools/check-declarations.py
17 declaration sites (0 forbidden, 17 allowed literals), 0 failures
$ python3 tools/check-mirror-dupes.py
17 duplicate declaration(s), 17 ratchet row(s), 0 failure(s)
# the ratchet read 18 rows when round 18 landed (this ledger's Decision 19 note); the move to 17
# is commit `4b8decec`'s (the two stale `euidaccess` rows), not this slice's - no row of the
# ratchet names this file, before or after.
```

The three files' own contributions to the tree-wide readings are the append to
`docs/ports/macro-body-notes.tsv` (one row, 1269) and nothing else: `emacs/**` was read only,
``femacs/src/xterm.rs`` and this ledger are the writing set, and the tree's other modified files
(`femacs/src/image.rs`, `femacs/src/sfnt.rs`, `femacs/src/w32term.rs`, their ledgers and
`femacs/xtask/`) belong to lanes that were in flight while this round ran.

### Slice 9's readings (C 6990-9785), as landed and measured (2026-10-05)

The `--check` reading of the round, with the before copy in `/tmp` as the checkpoint procedure
asks:

```
$ python3 tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs   # before
255
FAIL emacs/src/xterm.c -> femacs/src/xterm.rs
$ python3 tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs   # after
212
FAIL emacs/src/xterm.c -> femacs/src/xterm.rs
# the rows that moved: the 41 missing-function rows of C 6990-9785, the `missing macro`/`missing
# constant` pair of HIGHLIGHT_COLOR_DARK_BOOST_LIMIT (43 removals), and three *new* rows the
# slice's own text produced - `fn x_alloc_nearest_color_1 does not mention goto`,
# `fn x_draw_fringe_bitmap does not mention cursor_p, goto, overlay_p`,
# `fn x_set_mouse_face_gc does not mention 0` - of which the first two are answered by the two
# `docs/ports/macro-body-notes.tsv` rows this round appends (1678, 1679) and the third by keeping
# C's own `0` in the `eassert` (255 - 43 + 3 - 3 = 212).
# the drift row changed with the file: `line count drifted 77.3%: C 33125 vs Rust 7508` ->
# `line count drifted 66.9%: C 33125 vs Rust 10952`.
```

```
$ python3 tools/check-port-lines.py
0 file(s) under ten percent, 0 reference line(s); 0 ratchet row(s), 0 new, 0 stale
$ python3 tools/check-dead-file-coverage.py
info docs/ports/xterm.md: 104 range row(s) cover src/xterm.c (33125 lines; 49 blank line(s)
between blocks need no row)
info docs/ports/xterm.md: 104 range row(s) cover src/xterm.h (2045 lines; 0 blank line(s) ...)
145 ledger(s) bound (a C range table), 0 failure(s)
$ python3 tools/check-ledger-ticks.py
238 ledger(s) bound (a function tick to compare), 0 failure(s), 0 carried by another file,
352 row(s) with no function tick ..., 0 row(s) without a ledger
# the 42 ticks this round adds (41 functions + HIGHLIGHT_COLOR_DARK_BOOST_LIMIT) are all in the
# unit's text; the 352 untickable rows are the class that gate describes (empty units and files
# whose body the frozen configuration leaves false).
$ python3 tools/check-citations.py
check-citations: 676 file(s) scanned, 0 drift, 0 range, 1905 external, 36 allowed, 0 stale
# the 24 external citations this slice adds are its `/// C:` lines for the two dispextern.h
# prototypes and the Xlib/Xt names (`docs/ports/include-*`'s own accounting).
$ python3 tools/check-declarations.py
17 declaration sites (0 forbidden, 17 allowed literals), 0 failures
$ python3 tools/check-mirror-dupes.py
17 duplicate declaration(s), 17 ratchet row(s), 0 failure(s)
# no row of the ratchet names this file, before or after, and the unit's three new records are
# `struct` definitions, which that gate never reports.
```

The one compile-reading that the frozen configuration gives, and the probe:

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/xterm-lane/target cargo run -q -p xtask -- build-tu src/xterm.c
warning: 22 warnings emitted
built src/xterm.rs -> ../build/femacs/xterm.o
# the same twenty-two mirror warnings slice 8's reading carries; the frozen configuration
# compiles none of this slice.  Five runs in this round, not three: two aborted on this slice's
# own syntax (`E0758` for a comment that spelled C's `*\/` as if Rust escaped slashes, and the
# `if`/`else` split across a `#[cfg]` block in `x_draw_fringe_bitmap`), and three were clean.

$ cd femacs && CARGO_TARGET_DIR=/tmp/xterm-lane/target cargo run -q -p xtask -- probe \
      src/xterm.rs /tmp/xterm-lane/x.rlib --crate-type=lib -C panic=abort \
      --cfg 'feature="have_x_windows"'
rustc failed for src/xterm.rs (exit status: 1)
# before: error: aborting due to 339 previous errors
#   201 E0425  131 E0609    5 E0308   1 E0381   1 E0080
# last measured after (the round's third probe run):
#   error: aborting due to 531 previous errors
#   330 E0425  146 E0609   10 E0423   8 E0308   1 E0381   1 E0080
# by file: E0425 - 327 in the unit's own text, 2 in include/xterm.rs, 1 in include/frame.rs;
#   E0609 - all 146 in the unit's text; E0423 - 6 in include/dispextern.rs, 4 in
#   include/xterm.rs (the mirror macro of the paragraph below).
# the delta's classes, each already in this ledger: Xlib/Xt/XInput2/GTK names with no mirror
# (XPoint, XFillPolygon, Convex, CoordModeOrigin, FillOpaqueStippled, FillSolid,
# GCGraphicsExposures, GCLineWidth, GCStipple, GCClipMask, GCClipXOrigin, GCClipYOrigin,
# XCreateGC/XChangeGC/XGetGCValues' record, XdbeSwapInfo, XParseColor, XAllocColor, XFreeColors,
# XQueryColors, XDisplayCells, WhitePixelOfScreen, BlackPixelOfScreen, DisplayOfScreen,
# XCreateWindow, XDestroyWindow, XDrawLine, XDrawPoint, XFillRectangle, XSetFillStyle,
# XSetWindowAttributes, XCreatePixmapFromBitmapData, XCopyArea, XFreePixmap, XChangeGC,
# Status, XSetWindowAttributes, GtkWidget/GdkWindow/GdkFrameClock and the gtk/gdk calls); this
# file's own later slices (x_catch_errors, x_had_errors_p, x_ignore_errors_for_next_request,
# x_stop_ignoring_errors, x_uncatch_errors*, x_window_to_frame, x_top_window_to_frame,
# x_translate_coordinates, x_atom_to_symbol, x_get_atom_name, x_clear_area, x_wm_supports,
# x_get_window_below, x_tooltip_window_to_frame, x_handle_pending_selection_requests,
# x_push_selection_request, request_sigio/unrequest_sigio, x_check_font); and other units'
# (lookup_basic_face, note_mouse_highlight, gui_intersect_rectangles, x_clip_to_row, xstrcasecmp);
# E0609's members the mirrors comment out or gate elsewhere (`xclient`/`xbutton`/`xconfigure`/
# `r#type` on `XEvent`, `borders_respect_alpha_background` on `frame`, `mouse_face_defer` on
# `Mouse_HLInfo`, `valid` on `color_name_cache_entry`, the pre-existing `sie`/`menu_bar_window`);
# and E0423's ten, which are **the mirror's**: C 8649's `VCENTER_BASELINE_OFFSET` expands
# `include/dispextern.rs` 2658-2663, whose body names FONT_DESCENT/FONT_HEIGHT/FRAME_FONT/
# FRAME_LINE_HEIGHT as calls that this unit's graph has as macros or not at all.
# NOT re-measured: the three fix batches after that run (the two record copies, the
# `(*((*s).face as *mut face))` groupings, the accessor bodies' `unsafe` and the `0` in the
# `eassert`) - the round's probe budget (three runs) was spent, so the number above is the last
# reading and the fix batches' effect on it is unmeasured.  The `--check` and the six gates
# above were re-run after every batch.

```

### Slice 10's readings (C 9786-11915), as landed and measured (2026-10-05)

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/xterm2-lane cargo run -q -p xtask -- build-tu src/xterm.c
warning: 30 warnings emitted
built src/xterm.rs -> ../build/femacs/xterm.o          # exit 0
$ cd femacs && CARGO_TARGET_DIR=/tmp/xterm2-lane cargo run -q -p xtask -- build-tu src/xterm.c
warning: 31 warnings emitted
built src/xterm.rs -> ../build/femacs/xterm.o          # exit 0, after the fix batch below
# the warnings are the mirrors' own (unused_doc_comments, unused_parens, unused_attributes,
# unreachable_code, suspicious_runtime_symbol_definitions); the frozen configuration compiles
# none of this slice, so build-tu proves parsing and nothing else here.  Two extra
# `rustc -Zparse-crate-root-only src/xterm.rs` runs (syntax only, no artifact, so not a
# build-tu reading) were the ones that caught this slice's first text slip: C's
# `msg.xclient.type` is `r#type` in Rust.
```

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/xterm2-lane cargo run -q -p xtask -- probe \
      src/xterm.rs /tmp/xterm2-lane/x.rlib --crate-type=lib -C panic=abort \
      --cfg 'feature="have_x_windows"'
rustc failed for src/xterm.rs (exit status: exit status: 1)
# before this slice (the checkpoint the round started from):
#   error: aborting due to 508 previous errors
#     313 E0425  161 E0609   17 E0614   13 E0308   1 E0606   1 E0381   1 E0277   1 E0080
#   by file: _root.rs 502, include/xterm.rs 4, include/frame.rs 1, include/systime.rs 1
# first after (this slice as it was first written):
#   error: aborting due to 658 previous errors
#     439 E0425  176 E0609   17 E0614   21 E0308   1 E0423   1 E0606   1 E0381   1 E0277   1 E0080
# last after (after the fix batch below):
#   error: aborting due to 636 previous errors
#     436 E0425  166 E0609   17 E0614   13 E0308   1 E0606   1 E0381   1 E0277   1 E0080
#   by file: _root.rs 629, include/xterm.rs 4, include/frame.rs 2, include/systime.rs 1
# the whole delta, and it is arithmetic: _root.rs 502 + 132 (this slice's own lines) - 5 (the five
# prefix `x_clear_area` call sites this slice's definition resolves - the reading's one reduction)
# = 629; include/frame.rs 1 + 1 = 2 (the mirror's `flush_frame!` body, whose own
# `redisplay_interface` gap is named by this slice's one call to it); this slice contributes no
# E0614, no E0308, no E0423, and moves no other error.
```

The 132 errors inside this slice's lines, by class (the `--check` and the six gates below were
re-run after every fix batch):

```
# E0425 127
#   Xlib entry points with no mirror row, 68: XSetForeground 14, XFillRectangle 9,
#     XGetGCValues 7, XDrawLine 6, XSetFillStyle 6, XCopyArea 5, XChangeGC 4, XSetClipMask 2,
#     XSetClipRectangles 2, XSetTSOrigin 2, XClearArea 1, XCreatePixmap 1, XMapRaised 1,
#     XSetDashes 1, XSetLineAttributes 1, XUnmapWindow 1, ConnectionNumber 1, XBell 1,
#     XFlush 1, XSendEvent 1, XCreateWindow 1  (the class slices 1-9 record)
#   X11 constants with no mirror row, 32: GCFunction 3, FillOpaqueStippled 3, FillSolid 3,
#     GCClipMask 2, GCClipXOrigin 2, GCClipYOrigin 2, GXcopy 2, GCLineStyle 2, GCLineWidth 2,
#     Unsorted 2, LineSolid 2, CapButt 1, JoinRound 1, LineOnOffDash 1, CWCursor 1,
#     InputOnly 1, CopyFromParent 1, GXinvert 1  (Decision 15's rule; each is in `x11_constants`
#     or in the owed list below)
#   X11/Xt records with no mirror row, 1: XSetWindowAttributes 1 (XPoint 6 is pre-existing)
#   other units' functions with no usable row in this graph, 24: get_glyph_string_clip_rect 6,
#     image_ascent 3, gui_intersect_rectangles 3, window_box 2, window_box_right 2,
#     window_box_left 1, window_box_left_offset 1, cursor_in_mouse_face_p 1,
#     font_for_underline_metrics 1, gui_clear_cursor 1, popup_activated 1,
#     detect_input_pending 1, x_scroll_bar_clear 1  (the first two are the mirror's own
#     `have_window_system` gate: include/dispextern.rs's `dispextern_late_decls` rows exist but are
#     not selected by this arm)
#   this file's own later slices, 2: x_ignore_errors_for_next_request 1, x_stop_ignoring_errors 1
# E0609 5
#   `xclient` on the opaque XEvent: 5  (C 11435-11443's five member writes in
#   `x_send_hourglass_message`; the `r#type` write is one of them, and the 33-member union
#   `docs/ports/include-xlib.md` owes is the single fix)
```

The fix batch the last reading follows (four items, each found by the probe and by nothing else -
the syntax gate sees none of them): `glyph_string_bits! (for_overlaps, …)` is a 3-bit field and
therefore an `c_int`, so its eight condition sites compare with 0; `x_draw_xwidget_glyph_string`
takes the mirror's `*mut c_void`; `XCreateGC`'s `XGCValues *` argument is the declared
`*mut c_void` (the same cast the prefix's own sites write); and `flush_frame` is the mirror's
macro, not a function (`flush_frame! (f)`).  Three further items are name-level and are *not*
re-measured by the probe: `x_composite_image`'s four parameters keep C's own spelling
(`srcX`/`srcY`/`dstX`/`dstY`, what `--check`'s token comparison asks for), `x_scroll_run`'s
XWIDGETS arm keeps C's own shadowing `int y = view->y + …` (`let mut y`, the C 11803 name), and
the unit's `face` copy renames slice 9's `_pad_underline_pixels` placeholder to C's
`underline_pixels_above_descent_line`.  None of the three can move an error count or kind (two
renames of already-used bindings and a field rename with the same type and offset), the `--check`
and the six gates were re-run after each, and the round's probe budget (three runs) was spent on
the two readings above.  The last probe reading therefore predates those three edits by name only.

The other two gates, before and after:

```
$ python3 tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs   # before
212
FAIL emacs/src/xterm.c -> femacs/src/xterm.rs
$ python3 tools/extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs   # after
185
FAIL emacs/src/xterm.c -> femacs/src/xterm.rs
# the rows that moved: 27 missing-function rows of C 9786-11915, minus the three *new* rows this
# slice's own text produced - `function x_composite_image does not mention dstx, dsty, srcx,
# srcy` (answered in the text: C's parameter names are kept), `function x_fill_underline does not
# mention break, case, default, fallthrough, switch` and `function x_draw_glyph_string does not
# mention break, case, cmp, default, switch` (answered by the two `macro-body-notes.tsv` rows this
# slice appends, 1705-1706).  212 - 27 + 3 - 3 = 185.
# the drift row changed with the file: `line count drifted 66.9%: C 33125 vs Rust 10952` ->
# `line count drifted 59.1%: C 33125 vs Rust 13538`.
```

```
$ python3 tools/check-port-lines.py
0 file(s) under ten percent, 0 reference line(s); 0 ratchet row(s), 0 new, 0 stale
$ python3 tools/check-dead-file-coverage.py
info docs/ports/xterm.md: 131 range row(s) cover src/xterm.c (33125 lines; 73 blank line(s)
between blocks need no row)
145 ledger(s) bound (a C range table), 0 failure(s)
$ python3 tools/check-ledger-ticks.py
238 ledger(s) bound (a function tick to compare), 0 failure(s), 0 carried by another file,
352 row(s) with no function tick ..., 0 row(s) without a ledger
# the 27 ticks this round flips are all in the unit's text; the row count is unchanged because
# this slice adds no function the reference does not have.
$ python3 tools/check-citations.py
check-citations: 676 file(s) scanned, 4 drift, 0 range, 1991 external, 32 allowed, 4 stale
# the drift and stale rows are include/lisp.rs's and image.rs's own (other lanes' files, in flight
# while this round ran - the reading moved from 5 drift to 4 between two runs of this gate, and no
# run named this unit or this ledger); the external count grew by this slice's `/// C:` lines
# (1905 -> 1991 tree-wide, most of them the x11-constants and XGCValues rows).
$ python3 tools/check-declarations.py
17 declaration sites (0 forbidden, 17 allowed literals), 0 failures
$ python3 tools/check-mirror-dupes.py
16 duplicate declaration(s), 17 ratchet row(s), 1 failure(s)
# the failure is a STALE row of `femacs/src/sysdep.rs` (`pipe2 from
# lib/include/unistd.in.rs is gone`), and the count read 18/17/3 an hour earlier - both readings
# are other lanes' files (image.rs's `cairo_destroy` row came and went); no row names this unit,
# before or after, and this slice adds no declaration to a mirror.
$ python3 tools/check-citation-text.py | tail -1
checked 1156 / mismatched 97 / ranges 789 / unresolved 731
# the mismatched rows are xfns.rs/xmenu.rs/xselect.rs/xwidget.rs's own; none is in this unit's
# text or in this ledger.
```

Line count and hash: `femacs/src/xterm.rs` **10,952 -> 13,538 lines**, md5
`c6f1b725b1ffc98a2d5618cf61d4cef1` -> `d694654e84e3355e5bf83b8d24f60bc5`; this ledger's own
`## C line ranges` table grew the 27 rows above and the remainder row moved to `11916-33125`.

### Slice 11's readings (C 12066-12095), as landed and measured (2026-10-06)

**What it lands.**  `make_invisible_cursor` (C 12067-12095) - the one function of the exposure/focus
neighbourhood whose text needs no bitfield machinery: C's function-`static` `no_data` array stays in
the function (PORTING.md 1.3), `XColor col` is zeroed because C hands it to `XCreatePixmapCursor` by
address uninitialised (Deviation 12), C 12084's chained `col.red = col.green = col.blue = 0` is
written out in C's order, `col.flags` is XColor's `char` field (C's flag expression narrowed, `as
c_char`), `None` is `include/xlib.rs`'s `x11_constants::None` and C's `!x_had_errors_p (dpy)` is the
port's `== 0` (the int-returning predicate).

**The probe (the lane's command, both sides).**
```
$ cd femacs && CARGO_TARGET_DIR=/tmp/xt11 cargo run -q -p xtask -- probe src/xterm.rs /tmp/xt11/new.o \
      --crate-type=lib --emit=metadata -C panic=abort --cfg 'feature="have_x_windows"'
# before this slice (HEAD's unit, HEAD's mirror - the same probe):
#   error: aborting due to 638 previous errors; 54 warnings emitted
#     436 E0425   13 E0308   166 E0609   17 E0614   1 E0381
# after (this unit, this mirror):
#   error: aborting due to 642 previous errors; 54 warnings emitted
#     440 E0425   13 E0308   166 E0609   17 E0614   1 E0381
# the whole delta, and it is arithmetic: +4 E0425 and nothing else - the four call sites this
# slice's text has into the three entry points the unit has not reached yet: x_catch_errors
# (C 26390), x_uncatch_errors (C 26568) and x_had_errors_p (C 26651, two calls).  Every other
# class reads exactly what it read before, and the warning count does not move.
```

**No mirror change this slice** (`check-mirror-dupes` 16/16/0): the four Xlib names the include
graph does not carry - `XCreatePixmapCursor` (Xlib.h 1554-1562) and the `XColor.flags` bits
`DoRed`/`DoGreen`/`DoBlue` (X.h 622-624, measured 1/2/4) - are declared at the use site (P-88 1,
the `src/xfns.rs` 1155-1161 Xlib rows' shape).  Moving them into `include/xlib.rs` (with the
`x11_constants` module's three constants) is **owed** and belongs to that mirror's own round, which
is also where the row for `XCreatePixmapCursor`'s half of the 68-entry Xlib inventory recorded in
slice 10's reading goes.

**Two neighbours are deliberately not in this slice**, and the reason is measured, not stylistic:
`x_toggle_visible_pointer` (C 12097-12139), `xfixes_toggle_visible_pointer` (C 12053-12062) and
`XTtoggle_invisible_pointer` (C 12141-12153) all store **`f->pointer_invisible`**, a `frame`
`bool_bf` for which no window-system-numbered accessor exists - `include/frame.rs`'s shift
numbering is carried for the frozen arm only (the `src/androidterm.rs` 214-217 note records the
same bound), so they wait on the `include/frame.rs` round; and `x_fixes_pointer_blanking_supported`
(C 12040-12045) reads `dpyinfo->xfixes_supported_p`/`xfixes_major`, which `include/xterm.rs` does
not carry yet.  Their checklist boxes stay unchecked on purpose.

**What `--check` reads.**  `extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs` goes
**185 rows -> 184**: `make_invisible_cursor` retires as a missing function; the line-count row stays
(13,607 Rust lines against C's 33,125, 58.9%).  The six text gates are unchanged
(`check-declarations` 18/0/18/0, `check-citations` 676/0/0/2129/36/0, `check-dead-file-coverage`
145/0, `check-port-lines` 0/0/0/0, `check-ledger-ticks` 238/0, `check-mirror-dupes` 16/16/0).

**Not claimed.**  The frozen configuration compiles none of this file, so the reading is a
**text** reading of an arm this machine cannot build: no cursor was created, no display was opened,
and the four `E0425` rows above are unresolved names, not behaviour.

### Slice 12's readings (C 26217-26347), as landed and measured (2026-10-06)

**What it lands.**  The first half of the error-handling block, the part that needs no request-serial
machinery: `struct x_error_message_stack` (C 26217-26239, with `handler`'s `x_special_error_handler`
type from xterm.h 1748-1749 declared at the use site), the two objects (`x_error_message` C 26249,
`x_error_message_count` C 26252 - the latter **non-`static`**, so exported, PORTING.md 0.2), the four
serial comparators (C 26260-26294, with C's `unsigned int` wrap written `wrapping_sub` and its
widening `as c_ulong`), `x_find_error_handler` (C 26296-26314), `x_unwind_errors_to` (C 26317-26323)
and `x_error_catcher` (C 26330-26347, whose `buf` is zeroed because C hands it to `XGetErrorText`
uninitialised, Deviation 12).

**The probe (the lane's command, both sides).**
```
$ cd femacs && CARGO_TARGET_DIR=/tmp/xt12 cargo run -q -p xtask -- probe src/xterm.rs /tmp/xt12/new.o \
      --crate-type=lib --emit=metadata -C panic=abort --cfg 'feature="have_x_windows"'
# before (HEAD's unit + HEAD's mirror):
#   error: aborting due to 642 previous errors; 54 warnings emitted
#     440 E0425   13 E0308   166 E0609   17 E0614   1 E0381   1 E0080   1 E0277
# after (this unit + this mirror):
#   error: aborting due to 644 previous errors; 54 warnings emitted
#     441 E0425   13 E0308   166 E0609   17 E0614   1 E0381   1 E0080   1 E0277
# the whole delta, and it is arithmetic: +1 E0425 and nothing else - C 26322's x_uncatch_errors,
# whose own definition (C 26568) is the next slice's work.  Every other class reads what it read
# before and the warning count does not move.
```
**Arm reach is measured** (P-86 7): injecting `let _: c_int = true;` as the first statement of
`x_is_serial_more_than` moves E0308 to 14 anchored at the injected line (`_root.rs:13677`); removed,
the file re-measured at 13,770 lines.

**The mirror round** (`include/xlib.rs`, the mirror grows by the names a ported arm reads - its own
file note): `XErrorEvent` (Xlib.h 920-928) with the layout **measured** on this machine
(`sizeof=40 align=8`, `type` 0, `display` 8, `resourceid` 16, `serial` 24, `error_code` 32,
`request_code` 33, `minor_code` 34 - three `core::mem::` assertions at its declaration), and
`XGetErrorText` (Xlib.h 2610-2615) beside the mirror's other entry points.  **P-87's two legs**:
`check-mirror-dupes` **16/16/0** and `tools/check-all-units.sh` **`checked 83 unit(s), 0 failure(s)`
+ `checking 83 unit(s), 0 unit failure(s)`**; the frozen list holds no X unit, so the named reading
is that mirror's other consumer `src/xfns.rs` (probed with `have_x_windows` before and after the
mirror change, the unit unchanged): **6 error-lines / 9 warnings on both sides, sets diff empty**.

**What `--check` reads.**  `extract-abi.py --check emacs/src/xterm.c femacs/src/xterm.rs` goes
**184 rows -> 176**: the seven functions and `x_error_message_count` retire as missing items; the
line-count row stays (13,770 against C's 33,125, 58.4%).  The six text gates are unchanged.

**Owed, named.**  The block's second half - `x_catch_errors_with_handler` (C 26372),
`x_catch_errors` (C 26390), `x_request_can_fail`/`x_clean_failable_requests` (C 26397/26420),
`x_ignore_errors_for_next_request`/`x_stop_ignoring_errors` (C 26463/26519),
`x_uncatch_errors_after_check`/`x_uncatch_errors` (C 26550/26567), `x_check_errors` (C 26613),
`x_had_errors_p` (C 26650) and `x_clear_errors` (C 26679) - needs `XNextRequest`, `NextRequest` and
`LastKnownRequestProcessed`, the three **macros of `<X11/Xlibint.h>`** that read fields of the
private `struct _XDisplay` (`request`, `last_request_read`, and the 64-bit `_upper32bit` pair), so
it is a measurement round of its own (P-88 2's clang probe) before those functions can land.  The
`x_error_message_count`'s read-only users in other units keep the mirror's declaration (the
`x_display_list` shape).

**Not claimed.**  The frozen configuration compiles none of this file, so this is a **text** reading
of an arm this machine cannot build: no error was trapped, no display was opened, and the one
`E0425` row above is an unresolved name, not behaviour.


### Slice 13's readings (C 26349-26689), as landed and measured (2026-10-06)

**What it lands.**  The error-handling block's second half: `x_catch_errors_with_handler` (C 26372)
and `x_catch_errors` (C 26389), `x_request_can_fail` (C 26397), `x_clean_failable_requests` (C 26420),
`x_ignore_errors_for_next_request` (C 26463) with its C 26474-26482 `HAVE_GTK3` arm,
`x_stop_ignoring_errors` (C 26519) with its C 26533-26538 one, `x_uncatch_errors_after_check`
(C 26550), `x_uncatch_errors` (C 26567), `x_check_errors` (C 26613, whose C 26640 `alloca` is the
port's stack pool, P-05), `x_had_errors_p` (C 26650) and `x_clear_errors` (C 26679) - and C 26691-26702's
`#if false` block (`x_fully_uncatch_errors`) carried as the comment it is, the C 5127-5156 shape.
Rust: 13,770 -> **14,188** lines.

**The probe (the lane's command, two arms, both sides).**
```
$ cd femacs && CARGO_TARGET_DIR=/tmp/xt13 cargo run -q -p xtask -- probe src/xterm.rs /tmp/xt13/new.o \
      --crate-type=lib --emit=metadata -C panic=abort --cfg 'feature="have_x_windows"' \
      [--cfg 'feature="have_gtk3"']
# arm A (have_x_windows), before -> after:
#   644 error-lines / 54 warnings   441 E0425  13 E0308 166 E0609 17 E0614 1 E0381 1 E0080 1 E0277
#   568 error-lines / 54 warnings   366 E0425  13 E0308 166 E0609 17 E0614 1 E0381 1 E0080 1 E0277
# arm B (have_x_windows + have_gtk3), before -> after:
#   649 error-lines / 54 warnings   443 E0425  ...
#   579 error-lines / 54 warnings   374 E0425  ...
# the whole delta in both arms is E0425 and nothing else: -75 in arm A and -69 in arm B, i.e. the
# call sites of the eleven functions this slice defines (plus the five `XSync` rows and the two
# pre-existing `x_had_errors_p`-class rows the same definitions close).  No other class moves and
# the warning count is unchanged.
```
**Arm reach is measured** (P-86 7): injecting `let _: c_int = true;` as the first statement of
`x_catch_errors_with_handler` moves E0308 to 14 anchored at the injected line (`_root.rs:13819`);
removed, the file re-measured at 14,188 lines.

**The first readings named five fault classes of this slice's own text** (all fixed here; the
arithmetic above is the reading *after* the fix batch, so slice 12's reading sat inside them):
⑴ an `E0428` for `x_error_message_count` - **slice 12's own** defect, which that slice's class
counts did not include: `include/xterm.rs` 1645 declares the object and the unit defines it, so the
mirror's row is retired with a note (P-90 7; the units that read it, `src/eval.rs` 1835-1836,
declare it themselves).  ⑵ twenty `E0308`s at **earlier call sites**: `x_had_errors_p` returns
C's `bool` there, so `… == 0` / `… != 0` become `!x_had_errors_p (…)` and `x_had_errors_p (…)`
(twelve and seven sites, plus the two in `make_invisible_cursor`); C's `sprintf`-style truth test
and the port's differ only in spelling.  ⑶ `dpyinfo->failable_requests` is an **array** in the
record (`[x_failable_request; 128]`, `N_FAILABLE_REQUESTS`), so C's pointer decay is written
`… .as_mut_ptr ()` at its six uses.  ⑷ `XSync` had no row anywhere (`docs/ports/xterm.md`'s own
text called it) - it is declared in `include/xlib.rs` now (Xlib.h 3419-3420) and the five call
sites pass C's `False` as `0` because the parameter is Xlib's `Bool` (`c_int`), which also retires
the two pre-existing `E0425` rows.  ⑸ seven `variable does not need to be mutable` warnings (six
here, one carried from slice 11's `pix`), cleared - the warning count is back to its 54.

**The mirror round** (three files): `include/xlib.rs` gains the two **degraded** request-serial
macros - `NextRequest` and `LastKnownRequestProcessed` keep C's names with `XNextRequest` /
`XLastKnownRequestProcessed` (Xlib.h 1806/1809) for their bodies, because C's own bodies read the
private `_XPrivDisplay` record this tree does not carry (the `DefaultScreenOfDisplay` degradation
class, recorded as Decision 25) - plus `XSync`; `include/gtk.rs` gains
`gdk_x11_display_error_trap_push`/`_pop_ignored` (the two `HAVE_GTK3` arms' calls); and
`include/xterm.rs` loses the `x_error_message_count` row (above).  **P-90 5 move-backs in the same
commit**: `src/xfns.rs` 1954-1955's `XNextRequest` and `src/gtkutil.rs` 258-261's `XSync`
declarations retire, their mirror rows being the home now.
**P-87's two legs**: `check-mirror-dupes` **16/16/0** (the move-backs were its two failures:
`18 duplicate declaration(s), 16 ratchet row(s), 2 failure(s)` before them) and
`tools/check-all-units.sh` **`checked 83 unit(s), 0 failure(s)`** + **`checking 83 unit(s), 0 unit
failure(s)`**.  The frozen list holds no X unit, so two named readings carry the mirror's consumers:
`src/xfns.rs` reads **5 -> 4 previous errors** with the same classes and warnings (the moved row's
own site), and `src/gtkutil.rs` reads **0 error-lines / 0 warnings on both sides**.

**What `--check` reads.**  `184 -> 176` was slice 12; **this slice goes 176 -> 165**: the eleven
functions retire as missing items, the two token rows the first reading brought
(`x_check_errors does not mention alloca, false` and three more `does not mention false` rows) are
closed by `docs/ports/macro-body-notes.tsv` rows, and the line-count row stays (14,188 against
C's 33,125, 57.2%).  The six text gates are unchanged.

**Not claimed.**  The frozen configuration compiles none of this file, so both arms are **text**
readings of an arm this machine cannot build: no error was trapped, no display was opened and no
request serial was read; the remaining `E0425` rows are unresolved names, not behaviour.


### Slice 14's readings (C 26704-26992), as landed and measured (2026-10-06)

**What it lands.**  The `Handling X errors` banner with the file-scope `error_msg` pointer
(C 26722), `x_try_restore_frame` (C 26728-26738) and **`x_connection_closed` (C 26743-26992)** - the
connection-loss teardown, 250 C lines and this file's second-longest function: the recursive-call
guard through its own function-`static` `current_display`, the drag-and-drop cancellation arm with
its `sigsetjmp`/`XSetIOErrorHandler` pair and the Motif drop-message construction, the two
`FOR_EACH_FRAME` delete passes with their backward labels, the GTK arm's unconditional
`shut_down_emacs`/`emacs_abort`, the error-handler dump, and the final non-local exit
(C 26991's `error`).  C 26704-26713's second `#if false` pair is carried as the comment it is.
Rust: 14,188 -> **14,541** lines.

**The probe (the lane's command, three arms, both sides).**
```
$ cd femacs && CARGO_TARGET_DIR=/tmp/xt14 cargo run -q -p xtask -- probe src/xterm.rs /tmp/xt14/fin.o \\
      --crate-type=lib --emit=metadata -C panic=abort --cfg 'feature="have_x_windows"' \\
      [--cfg 'feature="have_gtk3"'] [--cfg 'feature="use_gtk"']
# arm A (have_x_windows), before -> after:
#   568 error-lines / 54 warnings   366 E0425  13 E0308 166 E0609 17 E0614 1 E0381 1 E0080 1 E0277
#   564 error-lines / 54 warnings   362 E0425  13 E0308 166 E0609 17 E0614 1 E0381 1 E0080 1 E0277
# arm B (A + have_gtk3): 579 / 54 (374 E0425) -> 575 / 54 (370 E0425)
# arm C (A + use_gtk):   565 / 59 (360 E0425, 15 E0308, 167 E0609) -> 562 / 60 (357 E0425, 15, 167)
# the delta: -4 E0425 in A and B and -3 in C, and no other class moves in any arm.  Those rows are
# the earlier call sites the new use-site declarations resolve (do_switch_frame, delete_frame twice,
# clear_waiting_for_input, XSetIOErrorHandler); this slice's own two functions are `static` and their
# callers (x_error_quitter, x_io_error_quitter) are still ahead, so they add no resolution of their
# own.  Arm C's one extra warning is C's own structure: the USE_GTK arm ends in `emacs_abort ()` and
# C's statements after `#endif` follow it, which Rust reports as an unreachable statement (the
# mirrors' own two `unreachable expression` warnings are that class); A and B keep their 54.
```
**Arm reach is measured** (P-86 7): injecting `let _: c_int = true;` as the first statement of
`x_try_restore_frame` moves E0308 to 14 anchored at the injected line (`_root.rs:14244`); removed,
the file re-measured at 14,541 lines.

**The first readings named five of this slice's own fault classes**, all fixed here: ⑴ `FRAME_KBOARD`,
`FRAME_MINIBUF_WINDOW` and `FRAME_TOOLTIP_P` are the mirror's **functions**, not the macros other
units' copies of those names are (three rows); ⑵ `EQ` and `FRAME_LIVE_P` return `bool`, so C's
`== 0` tests are the port's `!` (three sites); ⑶ the `XM_DRAG_*`/`XM_DROP_*` names are enum members
behind the unit's own modules and the `xm_drop_start_message` fields are narrower than the values C
assigns (`as u8`/`as u32`/`as u16`, six sites - the C 26805-26817 block is the same text the
`xm_send_drop_message` path already spells at C 4943-4950); ⑷ `x_dnd_finish_display` is a
`*mut c_void` row (the P-72 rule 3 wording that path uses), so the C 26787 comparison writes the
cast; ⑸ the `-> !` return types the earlier slices gave `xm_drag_window_io_error_handler` and
`x_dnd_io_error_handler` are **incompatible with `XSetIOErrorHandler`'s `Emacs_XIOErrorHandler`
parameter**, so both take C's own `int` and keep their diverging tails - the two `E0308`s the first
reading named were that mismatch surfacing at C 26783 and at the motif path's C 1803.  Two
`variable does not need to be mutable` warnings and one `partially assigned binding` (`dmsg`,
zeroed per Deviation 12) were cleared with them.

**What `--check` reads.**  **165 -> 163**: `x_try_restore_frame` and `x_connection_closed` retire as
missing functions and the one token row this slice brought
(`function x_connection_closed does not mention alloca, goto`) is closed by a
`docs/ports/macro-body-notes.tsv` row (P-05's stack pool and PORTING.md 2.2's labelled `loop`s);
the line-count row stays (14,541 against C's 33,125, 56.1%).  The six text gates are unchanged, and
**no mirror changed this slice**, so P-87's two legs do not apply to it.

**Not claimed.**  The frozen configuration compiles none of this file, so all three arms are
**text** readings of an arm this machine cannot build: no connection was lost, no error was trapped
and no display was opened; the remaining `E0425` rows are unresolved names, not behaviour.


## Owed by the integrator (nothing here is claimed as done)

**0. Three third-party SDKs are missing from this machine, and slices 70-74 are where that shows:
`<Xm/ScrollBar.h>` (Motif), libXt's headers, and gtk-3.0.**  libXt is the largest of the three: the
Xt names this unit declares at their use sites (`Widget`/`Dimension`/`Position`/`CorePart`/
`_WidgetRec`, `XtVaGetValues`, `XtConfigureWidget`, `XtResizeWidget`, `XtNheight`/`XtNwidth`/
`XtNshown`) are all the header's own spellings with **UNVERIFIED** values, and slice 74 widened their
`use_x_toolkit` gates and fixed the four pre-existing defects that widening exposed (Decision 87).
`<Xm/ScrollBar.h>` is slice 72's Motif block:  `xm_scroll_callback` (C 15709-15786) needs Motif's `XmCR_*` reasons, the
`XmScrollBarCallbackStruct` record and the `XmNsliderSize` resource name; the host has
`/opt/homebrew/include/X11` (Xlib's headers only) and no Motif tree at all, so the values cannot be
measured.  The port writes C's own names and the header's published layout/order, marks every value
**UNVERIFIED** in the source, and this Owed item is what would close them: a Motif header (or a
machine that has one) turns the eight constants, the record's offsets and the resource string into
**Slice 100 adds a fourth** (2026-10-07, Decision 113): `<X11/extensions/Xfixes.h>` is not in the
reference tree and not on this machine either (measured), so the two `#ifdef HAVE_XFIXES`
definitions carry the SDK's `XFixesSelectionNotifyEvent` as a use-site declaration (P-88 1) whose
member order is the header's published one and whose **layout is UNVERIFIED**; the arm `AXF` is what
reads the block, and the three gated `x_display_info` members it names are Owed item 2's.
readings.  Arm **UM** (`have_x_windows` + `use_toolkit_scroll_bars` + `use_x_toolkit` + `use_motif`)
is the arm slice 72 added for this block; its reading shows the *text* resolves (0 movement), which
is not a claim about the *values*.  The same missing-header situation is what
`include/gtk.rs` 1421-1425's GTK placeholders record for the GTK arm (gtk-3.0 headers are absent
too), so the GTK scroll callback in the same `#if`/`#elif` chain will need the same treatment.  **Slice 76 is the second, larger instalment of the
same three:** the Xt arm's two creators need fifteen `XmN*` names, `XmCreateScrollBar`, the four
`Xm*` scalars (`XmVERTICAL`/`XmHORIZONTAL`/`XmMAX_ON_BOTTOM`/`XmMAX_ON_RIGHT`), eleven
`<X11/StringDefs.h>`/`<Xaw/Scrollbar.h>` names, `scrollbarWidgetClass`, `XtCreateWidget`,
`XtAddCallback`, `XtVaSetValues`, `XtAppAddActionHook`, `XtDisplay`/`XtWindow`, `XDefineCursor`,
`XtArgVal`/`Arg`/`ArgList`, `XtCallbackProc`/`XtActionHookProc`/`WidgetClass`, and the two
`#ifdef XtN…` guards (`feature = "xtnbenicetocolormap"`/`"xtnnarrowscrollbars"`, which no arm sets -
so those two bodies are carried text this lane cannot compile at all).  Each is **UNVERIFIED** in
the source; the headers named above would turn them into readings.

**1. Two `femacs/xtask/Cargo.toml` rows.**  `have_xrender_0_2` (`RENDER_MAJOR > 0 ||
RENDER_MINOR >= 2`, C 6512/6596/6728) and `have_xrender_0_10` (`RENDER_MAJOR > 0 ||
(RENDER_MINOR >= 10)`, C 6594) are derived names (Decision 20's table); Xrender.h is not in this
tree, so the names have to be declared before an XRender arm can be selected.  The other names
slice 8's arms carry already exist: `use_cairo` (Cargo.toml 790), `use_cairo_xcb` (375),
`use_xcb`, `have_xrender` (370), `have_xdbe` (364), `have_xinput2` and its `_1`/`_2` (367-368),
`have_gtk3` (321), `use_gtk` (792), `use_x_toolkit` (806) and the four `cairo_has_*` (47-50).

**2. Mirror rows the carried arms read, none of which this writing set could add** (each is a
`P-26` "the owning mirror takes it" note, the same shape Decision 19's list has):

* `struct xi_scroll_valuator_t`'s two bitfields `horizontal` and `invalid_p` (xterm.h 238-256).
  `include/xterm.rs` records them as comments (its 274-296), so `xi_populate_scroll_valuator`,
  `x_get_scroll_valuator_delta` and `xi_reset_scroll_valuators_for_device_id` write C's spelling of
  a member that is not an identifier in the Rust text; the `bits` accessors `PORTING.md` 1.3/P-80
  prescribes are what those sites need.
* `struct x_display_info`'s `ext_codes` (xterm.h 838) and `supports_xkb` (911) - both already on
  Decision 19's "still uncarried" list, now *read* by `x_gc_get_ext_data`/`x_extension_initialize`
  and `x_restore_events_after_dnd`.
* `<math.h>`'s `lrint` (used by `xi_link_touch_point`'s `HAVE_XINPUT2_2` arm): neither
  `lib/include/math.rs` nor any other mirror carries it.
* `struct x_display_info`'s three `#ifdef HAVE_XFIXES` members (xterm.h 959-967):
  `monitored_selections` (whose element record `x_monitored_selection` *is* carried, this tree's
  `include/xterm.rs` 371-379, under the same gate), `selection_tracking_window` and
  `n_monitored_selections`.  Slice 100's arm `AXF` reads them (seven `E0609` rows, Decision 113 2);
  `x_monitored_selection`'s own two fields are already gated in the mirror, so those three rows are
  the one gap.

* The `use_gtk` arm's names: `FRAME_TOOLBAR_TOP_HEIGHT`/`FRAME_TOOLBAR_LEFT_WIDTH` (xterm.h
  1469/1477) and `FRAME_GTK_OUTER_WIDGET` (1449) live only in `include/pgtkterm.rs` in this tree,
  `xg_get_scale` is `src/gtkutil.rs`'s exported function with no prototype in this graph,
  `gtk_widget_intersect` and `G_OBJECT_GET_CLASS` have no row anywhere (`G_OBJECT_GET_CLASS` is a
  *per-unit* macro in `xfns.rs` 1017 and `xwidget.rs` 473, not a mirror row).
* The cairo names the `use_cairo` arms use (`cairo_t`, `cairo_surface_t`, `cairo_pattern_t`, the
  `cairo_*` functions, `cairo_user_data_key_t`, `cairo_matrix_t`, `CAIRO_*`): they live in
  `include/gtk.rs` (242-300), which this unit reaches only under `use_gtk` - a `use_cairo` build
  without GTK would need that mirror's edge (or the cairo rows moved to one).
* The X11 SDK types and constants the XRender arms use (`XGCValues`, `XRenderColor`,
  `XRenderPictureAttributes`, `Picture`, `Unsorted`, `GCFillStyle`, `GCStipple`, `FillSolid`,
  `FillOpaqueStippled`, `PictOpSrc`, `PictOpOver`, `PictStandardA1`, `CPRepeat`, `CPClipMask`,
  `RepeatNormal`, `XQueryTree`), the XInput2 ones (`XIDeviceEvent`, `XIDeviceInfo`,
  `XIScrollClassInfo`, `XIValuatorClassInfo`, `XITouchClassInfo`, `XIButtonState`, `XIEventMask`,
  `XIMaskLen`, `XIMaskIsSet`, `XIScrollClass`, `XIValuatorClass`, `XITouchClass`, `XIModeAbsolute`,
  `XIDirectTouch`, `XIScrollTypeHorizontal`, `Button1Mask`) and the GC extension records
  (`XEDataObject`, `XExtData`, `XExtCodes`, `XEHeadOfExtensionList`, `XFindOnExtensionList`,
  `XAddToExtensionList`, `XAddExtension`): Xlib/XInput2/XRender names with no mirror in this
  graph, kept bare as Decision 15's rule says.
* The display-iterators' arm names this slice reaches through `XWindowAttributes`' `your_event_mask`
  (which *does* resolve) and through `XImage`-side helpers in the cairo arms (no XImage row).

**3. Slice 6's `*mut c_void` display model, now visible.**  `x_dpyinfo`'s definition turned slice
6's `x_dnd_free_toplevels` call site into the flip arm's one new `E0308` (`*mut c_void` vs C's
`Display *`), the class Decision 19 clause 4 already records.  A unit-lane edit that moves those
locals onto `*mut Display` (or casts at the call) makes it go away; nothing in this slice chose it.

**4. The mirror's `XEvent` is still opaque**, and this file's text writes its members
(`msg.xclient`, `msg.xbutton`, `configure->xconfigure`, `x_dnd_pending_send_position.type`).  The
count is now 82 pre-existing sites plus this slice's 43 (`xclient`) and 3 (`xconfigure`/`r#type`);
`docs/ports/include-xlib.md` owes the union's 33 member records, and uniting the spellings is that
ledger's call, not a per-function choice - C's spelling is what the token gate demands and what slices
2, 5 and 6 already write.

**5. The next slice.**  **C 11916-…** (C 11925's `x_frame_highlight` on: the frame highlight
and focus hooks, the XFixes pointer blanking, `XTread_socket` and the event loop, the toolkit and
scroll-bar code, `x_initialize` and `syms_of_xterm`); this ledger's `## C line ranges` row
`11916-33125` is the remainder that row's lane replaces, and the gates it needs are the ones
`## Gate mapping` lists.  (This item carried slice 8's next-slice text - "C 6991-…" and the
`6990-33125` remainder - through slice 9's round, which had already landed that range; slice 10
corrects it, the same `C 11916` the range table names.)

**6. What moved outside this writing set: nothing.**  `emacs/**` was read-only; no mirror, no
`xtask`, no `docs/baseline` and no other lane's file changed.  The one append is
`docs/ports/macro-body-notes.tsv` 1269 (`x_dnd_note_self_drop`'s `alloca`/`kind`), the row the
`--check` gate asks for.

**7. Slice 9's requests (C 6990-9785, this round).**

* Three `femacs/xtask/Cargo.toml` rows: `have_clock_gettime` (C 6994's third term),
  `frame_debug` (C 7101/7271's `#ifdef FRAME_DEBUG`) and `debug_x_colors` (C 9506/9562's
  `#ifdef DEBUG_X_COLORS`).  The other names this slice's arms carry already exist (Decision 21's
  sub-gate paragraph names them).
* **`include/dispextern.rs`'s `VCENTER_BASELINE_OFFSET` cannot resolve in this unit** (10
  `E0423`, measured): its body (dispextern.rs 2658-2663) names `FONT_DESCENT`/`FONT_HEIGHT` and
  `FRAME_FONT`/`FRAME_LINE_HEIGHT` as *calls*, while this graph has `FONT_HEIGHT` as
  `include/font.rs` 258's macro, `FRAME_FONT` as `include/xterm.rs` 1119's macro, and neither
  `FONT_DESCENT` nor `FRAME_LINE_HEIGHT` at all.  The fix belongs to the mirror (spell the two
  macros as `FONT_HEIGHT!`/`FRAME_FONT!` and name the two it lacks); this slice is merely the
  first call site to reach the macro.
* Members the mirrors keep as comments or gate behind another feature, each measured this round:
  `frame`'s `borders_respect_alpha_background` (11 `E0609`; C 1851-1861's `#ifdef
  HAVE_WINDOW_SYSTEM` region - the mirror carries `foreground_pixel`/`background_pixel` but not
  this one), `Mouse_HLInfo`'s `mouse_face_defer` (1), `color_name_cache_entry`'s `valid` bitfield
  (3; the mirror's comment at `include/xterm.rs` 261 says so, so the port keeps C's spelling as
  the `xi_scroll_valuator_t` row already does), and `glyph_string`'s `gc` (the record's own gate,
  see the next clause).
* **The `dispextern_face_records`/`have_window_system` group**: this unit now carries its own
  `struct face` (X11 arm, `docs/baseline/face-layout.tsv`), `draw_fringe_bitmap_params` (the
  mirror's offsets) and `glyph_string` (C's order, **no** asserts: the mirror's own note records
  that record's layout as unmeasured here).  Status 3.220's ruling makes the copies legitimate;
  whoever retires the units' copies (the `xfaces.rs`/`window.rs`/`alloc.rs`/`term.rs`/`dispnew.rs`
  list) has to take `femacs/src/xterm.rs`'s three with them, and only then can `glyph_string_bits!`
  and `get_/set_cursor_p`/`get_/set_overlay_p` come from the mirror instead of the unit.
* The X11 macros this slice's live text names and `include/xlib.rs`'s `x11_constants` does not
  carry: `FillOpaqueStippled`, `FillSolid`, `GCGraphicsExposures`, `GCLineWidth`, `GCStipple`,
  `GCClipMask`, `GCClipXOrigin`, `GCClipYOrigin`, `Convex`, `CoordModeOrigin` (`<X11/X.h>`/
  `<X11/Xlib.h>` values, each clang-measurable the way that module's rows were); the record
  `XGCValues` (Xlib.h 177-203) is now declared **in the unit** the way `src/xftfont.rs` 132-171
  declares it, plus `XPoint` (Xlib.h 322-325) and `XdbeSwapInfo` (Xdbe.h), which nothing carries.
* The two `unsafe extern "C"` blocks this unit now holds (`get_glyph_string_clip_rects`,
  `prepare_face_for_display`, `x_free_colors`, `parse_color_spec`, `gamma_correct`,
  `x_free_dpy_colors`, each with C's own line) move into the owning mirror when it grows them
  (P-26).
* `x_color_cache_bucket_size` is `DEFVAR_INT`'d at C 32958, so it lands with that slice; until
  then the four call sites the port writes are `E0425` names, the class slices 4-8 record.

**8. What moved outside this writing set (slice 9).**  `emacs/**` was read-only; no mirror, no
`xtask` row, no `docs/baseline` file and no other lane's file changed.  The two appends are
`docs/ports/macro-body-notes.tsv` 1678-1679 (`x_alloc_nearest_color_1`'s `goto`; `x_draw_fringe_bitmap`'s
`cursor_p`/`goto`/`overlay_p`), the rows the `--check` gate asks for; both were written by copying
the table to `/tmp` first and appending, so no other lane's row was rewritten.  The build
artifacts this round left (`build/femacs/units/xterm`, `build/probes/xterm`, `build/femacs/xterm.o`)
were deleted after each compile.

**9. One `femacs/xtask/Cargo.toml` row: `have_xkb`.**  This slice's `XTring_bell` carries C 11677's
`#ifdef HAVE_XKB` (and slice 8's `x_restore_events_after_dnd` already carried C 4791's), and the
feature table has no `have_xkb` row (measured: `grep -n xkb femacs/xtask/Cargo.toml` finds none),
so the arm cannot be selected today.  Every other name this slice's arms carry exists:
`use_cairo`, `use_gtk`, `use_x_toolkit`, `use_xcb`, `use_cairo_xcb`, `have_xdbe`, `have_xrender`,
`have_xrender_0_2`, `have_xwidgets`.

**10. Mirror rows this slice's carried arms read** (each a `P-26`/`P-55` "the owning mirror takes
it" note, the same shape Decision 22's clause 6 and slice 9's Owed items have):

* `include/xlib.rs`'s `x11_constants`: the 20 X11 constants this slice's live text names and the
  module does not carry - `FillSolid`, `FillOpaqueStippled`, `GCFunction`, `GCClipMask`,
  `GCClipXOrigin`, `GCClipYOrigin`, `GCLineStyle`, `GCLineWidth`, `GXcopy`, `GXinvert`,
  `LineSolid`, `LineOnOffDash`, `CapButt`, `JoinRound`, `Convex`, `CoordModeOrigin`, `CWCursor`,
  `CopyFromParent`, `InputOnly`, `Unsorted` - each an `<X11/X.h>`/`<X11/Xlib.h>` value
  clang-measurable the way that module's rows already are (`docs/ports/include-xlib.md`).
* `include/xlib.rs`: the `XPoint` record (Xlib.h 322-325) and `XSetWindowAttributes` (Xlib.h
  243-252, the cursor field this slice's `x_show_hourglass` writes), plus the `XEvent` union the
  ledger's Decision 20 and `docs/ports/include-xlib.md` already owe - this slice's five `xclient`
  member writes are five more sites of that class (108 -> 113 in this arm's reading).
* The dispextern.h prototypes a flip needs and this graph declares nowhere:
  `get_glyph_string_clip_rect` (C 3663) and `gui_intersect_rectangles` (C 3676) *do* exist in
  `include/dispextern.rs`'s `dispextern_late_decls`, but under `feature = "have_window_system"`,
  so a flip that sets `have_x_windows` alone leaves this slice's nine call sites `E0425` (the
  ledger's evidence above counts them there); `window_box` (C 3574-3575), `window_box_left`
  (3579), `window_box_left_offset` (3580), `window_box_right` (3581), `font_for_underline_metrics`
  (3638), `gui_clear_cursor` (3657), `cursor_in_mouse_face_p` (3692), `image_ascent` (3774) and
  `popup_activated` (3879) have no row anywhere - the `w32term.rs` 1750/3134-3141 use-site
  declarations are the shape they should take, and `detect_input_pending` (lisp.h 5095) is the
  same case.
* `include/frame.rs` 1908-1914's `flush_frame!` body names `redisplay_interface` and no unit of
  this graph declares it, so *every* call site of that macro reads one `E0425` (`widget.rs` 1400
  records the same finding; `haikufns.rs` 3664 is another site, and this slice's `XTflash` call
  path is the third).
* `include/dispextern.rs`'s own `face_bits!` (and the window mirror's `window_bits!` /
  `window_box` rows): this slice extends the *unit's* copy with the six accessors C 11064-11226
  reads, and calls `window_bits! (pseudo_window_p, …)`, which no unit can resolve today
  (`dispnew.rs` 396 and `fringe.rs` 865 keep their own copies of that macro for the same reason).

**11. Two defects this slice found in slices 1-9's text and did not edit** (they are outside its C
range; the round attributes them instead, and both readings carry them unchanged):

* **17 `E0614`** (`type \`face\`/\`glyph\` cannot be dereferenced`): the unit's own `face_bits!`,
  `glyph_u_cmp_bits!` and `glyphless_bits!` dereference their argument, and every one of their
  call sites passes a *value* rather than a pointer - `face_bits!` at 9687, 9736, 9806, 9856, 9889,
  9899, 9912, 9952, 10924; `glyph_u_cmp_bits!` at 9836; `glyphless_bits!` at 9974, 9976, 9984,
  9986, 9994, 9996, 10027.  Each is written `(*((*s).face as *mut face))` (or `(*glyph)`) where
  the macro's body is `(*$f)…`; the `w32term.rs` 2235 and `haikuterm.rs` 350 call sites show the
  intended spelling (pass the pointer), which is also what the macro's own `$f:expr` grouping
  wants.  Slice 9's ledger records these as "the `(*((*s).face as *mut face))` groupings", one of
  the three fix batches it landed *after* its last probe run - which is why they are 17 in both
  readings of this round.
* **2 of the prefix's 13 `E0308`**: `x_draw_glyph_string_background`'s two
  `if glyph_string_bits! (for_overlaps, (*s))` (9728 and 9750 in the current text) use that
  macro's `c_int` arm directly as a condition; `!= 0` is the fix, and it is the one this slice's
  own eight `for_overlaps` sites use.

**12. What this slice does not claim.**  No X11 build, no cairo/GTK/XRender/XCB/xwidget/XKB build
and no runtime were exercised: the frozen configuration compiles none of this file, the probe
compiles the plain-X11 arm (`have_x_windows`, no `use_cairo`, no `have_xrender`, no `use_gtk`, no
`have_xdbe`, no `have_xwidgets`, no `use_xcb`, no `have_xkb`), and everything under those features
is carried text that **no configuration in this tree has compiled**.  `build-tu` proves parsing;
the probe's numbers are unresolved names, not behaviour; and nothing here is device, GUI or
end-to-end evidence for any of the functions this slice lands.

**Slice 105 reads two mirror gaps for the first time** (2026-10-07, Decision 118): the GTK arms'
`GDK_WINDOW_XWINDOW` (`femacs/src/include/xterm.rs` 1043, the `not (feature = "have_gtk3")` arm of
`GDK_WINDOW_XID`) has no declaration anywhere - GDK's header is not on this machine and no mirror
carries it - and that macro's `have_gtk3` arm is missing from the mirror altogether, which arm AG3
reads as `cannot find macro GDK_WINDOW_XID` (anchored at `include/xterm.rs` 1056-1063's
`GTK_WIDGET_TO_X_WIN`).  Both are the mirror's owed work, not this unit's: closing them needs either
the GTK tree (the third-party-SDK gap of item 0) or a decision about what that arm expands to.
`gnu_xpm_bits` is *not* on this list: `docs/ports/include-emacs-icon.md` records it as the header's
dead text, so the two arms that name it record a bare name by design (P-08).
**The tick check does not read the other direction** (2026-10-07, status 3.899).  The gate
`check-ledger-ticks` binds a ledger's ticked rows to functions (238 ledgers, 0 failures), and this
round found the gap it leaves: 12 of this ledger's 15 unchecked boxes were *false* -
`x_calc_absolute_position` (27497), the ten scroll-bar functions (16715-17459) and `XEMBED_VERSION`
(29148) are all defined in `femacs/src/xterm.rs` while their boxes stayed unchecked (their slices'
readings say the `--check` rows retired, and `git log -S"- [x] x_scroll_bar_create"` is empty: never
ticked).  The boxes are ticked now, and the criterion to add to the gate is the one this round used:
for every unchecked box, look for `macro_rules! NAME`, `fn NAME` or `pub (const|static) NAME` in
the target unit's text, and fail with the name and the line on a hit.  What must stay out of it is
the other direction's false-positive class: this ledger's ticked-but-no-`fn` rows (19) are all
macro/const landings (`ATOM_REFS_INIT`, `SWAPCARD16/32`, `XM_DRAG_*`, `OPAQUE`, …), so an `fn`-only
reading would report 19 false failures.
**Slice 106 reads four new third-party-SDK names and one more mirror gap** (2026-10-07, Decision
119).  The names are `XtAppNextEvent` (the `USE_X_TOOLKIT` arm's dispatch call, item 0's libXt),
`XIGetClientPointer` (the `HAVE_XINPUT2` arm, item 2's SDK), `XkbGetState` and `XkbStateRec` (the
new `AXKB` arm's `HAVE_XKB` block, which no earlier arm could read).  The mirror gap is the same
shape slice 105 found for GDK: `x_display_info`'s `last_monitor_attributes_list` (xterm.h 834,
inside `#if defined HAVE_XRANDR || defined USE_GTK`) is not in this mirror, so the six arms that
compile C 12857 read one more `no field ...` row of a class the baseline already carries (16 rows
of it exist from earlier text).  Closing the SDK names needs the SDKs (item 0); the mirror gap is **closed** - the
sixty-sixth `xterm` slice's mirror round (status 3.902) carries the whole gated member run,
35 members including this one, and retires 176 arm rows (U/UT/UM/UG/UGX/AIU/AIUX/AG3/AXF/
AXKB/AXR).

### Slice 15's readings (C 26994-27126), as landed and measured (2026-10-06)

**What it lands.**  C 26994's prototype for `x_error_quitter` (carried as the comment it is:
Rust has no prototypes, and the definition keeps C's position), `x_error_handler`
(C 26999-27073, the first-level handler - the `USE_GTK && HAVE_GTK3` early-out, the
`x_request_can_fail` arm with its `memmove` compaction of `dpyinfo->failable_requests`, the
`x_handle_selection_error` arm and the XI2 arm), `x_error_quitter` (C 27081-27107, `NO_INLINE`,
`buf`/`buf1` and the message that reaches `x_connection_closed`) and `x_io_error_quitter`
(C 27114-27126, `NO_INLINE`, the `DisplayString` degradation, C's function-`static` `fmt` and the
`SAFE_ALLOCA` buffer).  Rust 14,541 -> 14,722 lines.

**What the probes read.**  Three arms, both sides, baseline = HEAD's unit with the *same* mirror
(this slice changes the mirror, so the baseline takes the new one to keep the delta the unit's
own): arm A (`have_x_windows`) **564 error-lines / 54 warnings -> 563 / 55**; arm B (A +
`have_gtk3` + `use_gtk`) **564 / 59 -> 563 / 60**; arm C (A + `have_xinput2`) **592 / 56 -> 591 /
57**.  Every arm's delta is **-1 `E0425`** (the `XDisplayString` use-site declaration retires the
row the arm had been carrying; this slice's own three functions are `static` and their callers are
already in the file) plus **+1 warning** - `value assigned to sa_avail is never read`, the class
`docs/ports/atimer.md` 106 records: C's `AVAIL_ALLOCA` also updates `sa_avail` and never reads it
again in a function that allocates once.  No other class moved in any arm.

**Arm reach (P-86 7).**  Injecting `let _: c_int = true;` as the first statement of
`x_error_handler` moves `E0308` from 13 to 14, anchored at the injected line (`_root.rs:14594`);
removed, arm A re-measures at 563 error-lines / 55 warnings and the file at 14,722 lines.

**The mirror round (P-87).**  `include/xterm.rs` moves the two rows Decision 27 records from
`x_output` to `x_display_info`.  Leg one: `tools/check-mirror-dupes.py` **16 duplicate
declaration(s), 16 ratchet row(s), 0 failure(s)**.  Leg two: `tools/check-all-units.sh` **checked 83
unit(s), 0 failure(s)** and **checking 83 unit(s), 0 unit failure(s)**.  The named reading is
another consumer of that mirror, `src/xfns.rs`: probed with `have_x_windows` it reads **6
error-lines / 9 warnings** before and after, and with `have_x_windows` + `have_xinput2` **7 / 9**
before and after, the sets diffing empty in both arms.

**What `--check` reads.**  **163 -> 159**: the three `missing function` rows retire, the two token
rows the first reading brought (`x_error_quitter does not mention 800, int_strlen_bound, xid` and
`x_io_error_quitter does not mention displaystring`) are closed by
`docs/ports/macro-body-notes.tsv` rows, and slice 14's invented-export row is removed (Decision
27).  The line-count row stays (14,722 against C's 33,125, 55.6%).

**Not claimed.**  The frozen configuration compiles none of this file, so all three arms are
**text** readings of an arm this machine cannot build: no error was trapped, no display was
opened, no connection was lost and no request serial was read; the remaining `E0425` rows are
unresolved names, not behaviour.  `x_error_handler`'s address is what C 30791/32358 hand to
`XSetErrorHandler`, and nothing here has run.


### Slice 16's readings (C 27128-27194), as landed and measured (2026-10-06)

**What it lands.**  The `Changing the font of the frame` banner and `x_new_font` (C 27136-27194) -
the fontset id, the frame's font/baseline/column-width/line-height writes, the scroll-bar column
computation with its `Bug#17163` note, the tip-frame guard around `adjust_frame_size`, and the
`HAVE_X_I18N` arm that re-sets the XIC's fontset through `xic_set_xfontset`.  Rust 14,722 -> 14,857
lines.  The five two-arm frame.h macros and the use-site externs (`fontset.h` 39/44, `dispextern.h`
3631, `window.h` 930, `xfns.c` 3814) are Decision 28's.

**What the probes read.**  Three arms, both sides, baseline = HEAD's unit with the same mirror:
arm A (`have_x_windows`) **563 error-lines / 55 warnings -> 563 / 55**; arm B (A + `have_gtk3` +
`use_gtk`) **563 / 60 -> 563 / 60**; arm C (A + `have_xinput2`) **591 / 57 -> 591 / 57**.  Every
arm's delta is **zero** - this slice's text resolves every name it uses, which is the reading
`s16dA` produced after the two first-reading faults were fixed (the `have_window_system` block and
`let unit`, which the arm named `unused_mut`).

**Arm D, the `have_x_i18n` text (new this slice).**  `have_x_windows` + `have_x_i18n`: baseline
**584 error-lines / 55 warnings -> 564 / 55**, the whole delta `-20 E0425` (the XIM types and masks
Decision 28 declares) with every other class and the warning count unmoved.  The named reading of the *other* consumer of
that mirror, `src/xfns.rs`, reads **6 error-lines / 9 warnings** on both sides with
`have_x_windows` (sets identical) and **20 / 12 -> 9 / 12** with `have_x_windows` + `have_x_i18n` -
the leak fix Decision 28 records is what removes those eleven `no field xic`-class rows there,
warnings unchanged.  The arm exists because
this is the first slice whose text carries a `HAVE_X_I18N` branch of its own; the frozen
configuration compiles none of it.

**What `--check` reads.**  **159 -> 158**: `missing function x_new_font` retires and no token row
arrives.  The line-count row stays (14,857 against C's 33,125, 55.1%).

**Not claimed.**  The frozen configuration compiles none of this file, so all four arms are **text**
readings of an arm this machine cannot build: no frame was given a font, no scroll bar was sized
and no frame was resized; the remaining `E0425` rows are unresolved names, not behaviour.


### Slice 17's readings (C 27195-27284), as landed and measured (2026-10-06)

**What it lands.**  The `X Input Methods` banner (C 27196-27198), C 27204-27208's note on the two
R6 macros, `x_maybe_clear_preedit` (C 27218-27243) and `xim_destroy_callback` (C 27249-27280),
all under C's three nested guards.  Rust 14,857 -> 15,007 lines.

**What the probes read.**  Baseline = HEAD's unit with the same arm.  Arm A (`have_x_windows`)
**563 error-lines / 55 warnings -> 563 / 55**; arm B **563 / 60 -> 563 / 60**; arm C
**591 / 57 -> 591 / 57**; arm D (`have_x_i18n`) **564 / 55 -> 564 / 55** - the new text is behind
`have_x11r6_xim`, so none of the four arms sees it.  Arm F (`have_x_windows` + `have_x_i18n` +
`have_x11r6` + `have_x11r6_xim`), which is the arm this text exists in: **563 / 55 -> 547 / 55**,
the whole delta **-16 `E0425`** (the sixteen `XFree` calls Decision 29 records) with every other
class and the warning count unmoved.

**Arm reach (P-86 7).**  Injecting `let _: c_int = true;` as the first statement of
`x_maybe_clear_preedit` moves `E0308` from 13 to 14, anchored at the injected line
(`_root.rs:14942`); removed, arm F re-measures at 547 error-lines / 55 warnings and the file at
15,007 lines.

**What `--check` reads.**  **158 -> 158**: neither function is a demanded item, because the frozen
table leaves `HAVE_X11R6_XIM` `undef` and the tool reads C 27210-27282 as dead text (P-66).  The
line-count row moves to 15,007 against C's 33,125 (54.7%).

**Not claimed.**  The frozen configuration compiles none of this text: no preedit was cancelled,
no input-method connection died and no frame was walked.  The remaining `E0425` rows are
unresolved names, not behaviour.


### Slice 18's readings (C 27286-27342), as landed and measured (2026-10-06)

**What it lands.**  `xim_open_dpy` (C 27289-27341), with C's `#ifdef HAVE_XIM` body (C 27296-27338)
and its `#else` text (C 27340), inside C 27200's `HAVE_X_I18N`; plus the XIM surface it calls
(Decision 30) and `XFree`'s declaration widened to every X arm.  Rust 15,007 -> 15,125 lines.

**What the probes read.**  Baseline = HEAD's unit with the same arm.  Arm A (`have_x_windows`)
**563 / 55 -> 547 / 55**; arm B **563 / 60 -> 547 / 60**; arm C **591 / 57 -> 575 / 57**; arm D
**564 / 55 -> 548 / 55** - each arm's whole delta is **-16 `E0425`** (the `XFree` calls), every
other class and the warning count unmoved.  Arm G (`have_x_windows` + `have_x_i18n` + `have_xim`),
the arm this slice's own text exists in: **564 / 55 -> 545 / 55**, the delta **-19 `E0425`** - the
same sixteen `XFree` rows plus three `Status` rows the declaration resolves - with no other class
moving.

**Arm reach (P-86 7).**  Injecting `let _: c_int = true;` as the first statement of the
`have_xim` arm moves `E0308` from 13 to 14, anchored at the injected line (`_root.rs:15083`),
and the arm also reads `unused_mut` if `xim` is declared `mut` - which is how the first reading
named that; removed, arm G re-reads 545 / 55 and the file 15,125 lines.

**What `--check` reads.**  **158 -> 158**: `xim_open_dpy` is dead text for the tool too (C 27200's
`HAVE_X_I18N` is `undef` in the frozen table).  The line-count row moves to 15,125 against C's
33,125 (54.3%).

**Not claimed.**  The frozen configuration compiles none of this text: no input-method connection
was opened or closed, no style list was queried and no coding system was chosen.  The remaining
`E0425` rows are unresolved names, not behaviour.


### Slice 19's readings (C 27344-27433), as landed and measured (2026-10-06)

**What it lands.**  `xim_instantiate_callback` (C 27352-27391, behind `HAVE_X11R6_XIM`) and
`xim_initialize` (C 27402-27433, behind `HAVE_XIM` with its own `HAVE_X11R6_XIM` split), with the
XIM surface both call (Decision 31).  Rust 15,125 -> 15,257 lines.

**What the probes read.**  Baseline = HEAD's unit with the same arm.  Arms A (`have_x_windows`),
B, C and D read exactly what they read at slice 18 - **547 / 55**, **547 / 60**, **575 / 57** and
**548 / 55** - because every item this slice adds is behind `have_x_i18n` and one of the two R6/XIM
gates.  Arm F (`+have_x11r6+have_x11r6_xim`) **547 / 55** and arm G (`+have_xim`) **545 / 55**,
also unchanged.  Arm H (`+have_x_i18n+have_xim+have_x11r6+have_x11r6_xim`), the arm that compiles
this slice's text: **545 / 55 -> 545 / 55** - delta zero, every class and the warning count
unmoved, which is what the two first-reading faults (the `XPointer` cast and the `ret`
assignment) were fixed for.

**Arm reach (P-86 7).**  Injecting `let _: c_int = true;` as the first statement of
`xim_instantiate_callback` moves `E0308` from 13 to 14, anchored at the injected line
(`_root.rs:15182`); removed, arm H re-reads 545 / 55 and the file 15,257 lines.

**What `--check` reads.**  **158 -> 158**: both functions are dead text for the tool too
(`HAVE_X11R6_XIM` and `HAVE_XIM` are `undef` in the frozen table).  The line-count row moves to
15,257 against C's 33,125 (53.9%).

**Not claimed.**  The frozen configuration compiles none of this text: no XIM server appeared, no
callback was registered and no XIC was created.  The remaining `E0425` rows are unresolved names,
not behaviour.


### Slice 20's readings (C 27436-27487), as landed and measured (2026-10-06)

**What it lands.**  `xim_close_dpy` (C 27442-27487): the `HAVE_XIM` arm, its
`HAVE_X11R6_XIM` sub-arm (unregister the instantiate callback, free the `xim_inst_t` and its
resource name), C 27477's guarded `XCloseIM`, and the `xim_styles` release - closing C 27200's
`HAVE_X_I18N` region at C 27489.  Rust 15,257 -> 15,333 lines.

**What the probes read.**  Baseline = HEAD's unit with the same arm.  Arms A **547 / 55**, B
**547 / 60**, C **575 / 57**, D **548 / 55**, F **547 / 55** and G **545 / 55** all read what slice
19 left, and arm H (`+have_x_i18n+have_xim+have_x11r6+have_x11r6_xim`), which compiles this
slice's text, reads **545 / 55 -> 545 / 55** - delta zero, every class and the warning count
unmoved.

**Arm reach (P-86 7).**  Injecting `let _: c_int = true;` as the first statement of the `have_xim`
arm moves `E0308` from 13 to 14, anchored at the injected line (`_root.rs:15300`); removed, arm H
re-reads 545 / 55 and the file 15,333 lines.

**What `--check` reads.**  **158 -> 158**: the function is dead text for the tool too.  The
line-count row moves to 15,333 against C's 33,125 (53.7%).

**Not claimed.**  The frozen configuration compiles none of this text: no callback was
unregistered, no input-method connection was closed and no style list was freed.  The remaining
`E0425` rows are unresolved names, not behaviour.


### Slice 21's readings (C 27493-27581), as landed and measured (2026-10-06)

**What it lands.**  `x_calc_absolute_position` (C 27496-27581), the file's first *live* function
after the XIM block: the two `XNegative`/`YNegative` early-outs, the width/height arms with their
frame-edge queries and the parent-frame branches, the Motif arm's `XtVaGetValues`, and the final
flag clear.  Rust 15,333 -> 15,464 lines.

**What the probes read.**  Baseline = HEAD's unit with the same arm.  Arm A (`have_x_windows`)
**547 error-lines / 55 warnings -> 551 / 55**: the whole delta is **+2 `E0425`** (the two display measurements) and **+2 `E0609`**, the two
`has_been_visible` reads Decision 33 records, with every other class and the warning count
unmoved.  Arm I (`have_x_windows` + `use_x_toolkit` + `use_motif`) **589 / 55 -> 586 / 55**:
`-6 E0425` (the Xt names this slice declares) and `+3 E0609` (the two
`has_been_visible` reads plus `column_widget`).

**What `--check` reads.**  **158 -> 157**: `missing function x_calc_absolute_position` retires and the
`x_calc_absolute_position does not mention output_data` token row is closed by a
`docs/ports/macro-body-notes.tsv` row; nothing else moves.  The line-count row moves to 15,464
against C's 33,125 (53.3%).

**Not claimed.**  The frozen configuration compiles none of this file, so the arm's numbers are
**text** readings: no frame position was computed, no frame edge was queried and no size hint flag
was cleared.  The remaining `E0425`/`E0609` rows are unresolved names and unstored members, not
behaviour.


### Slice 22's readings (C 27583-27689), as landed and measured (2026-10-06)

**What it lands.**  `x_set_offset` (C 27589-27689): the `change_gravity > 0` block that writes
`top_pos`/`left_pos`, clears the two negative flags and sets `win_gravity`, the
`x_calc_absolute_position` call, `x_wm_set_size_hint`, the two `USE_GTK` arms
(`x_gtk_use_window_move`'s early return with `gtk_window_move`, and the scaled move after the WM
offset adjustment) versus the `#else` `XMoveWindow`, and the `Vx_lax_frame_positioning` block that
ends in `x_sync_with_move`/`x_check_expected_move` or `XSync (dpy, 0)`.  Rust 15,464 -> 15,598
lines.

**What the probes read.**  Baseline = HEAD's unit with the same arm.  Arm A (`have_x_windows`)
**551 / 55 -> 558 / 55**: **+3 `E0425`** (Decision 34's first shape) and **+4 `E0609`**
(`move_offset_left`/`_top`, its third).  Arm B (A + `have_gtk3` + `use_gtk`) **551 / 60 -> 557 /
60**: `-1 E0425` (`xg_get_scale`, the use-site declaration resolves it), `+3 E0425` (the same
three names) and `+4 E0609`.  No warning moved in either arm.

**What `--check` reads.**  **157 -> 156**: `missing function x_set_offset` retires and no token row
arrives.  The line-count row moves to 15,598 against C's 33,125 (52.9%).

**Not claimed.**  The frozen configuration compiles none of this file: no frame was moved, no
window manager was asked anything and no configure request was synced.  The remaining
`E0425`/`E0609` rows are unresolved names and unstored members, not behaviour.


### Slice 23's readings (C 27691-27830), as landed and measured (2026-10-06)

**What it lands.**  `x_get_wm_check_window` (C 27691-27719), `x_wm_supports_1`
(C 27727-27823) and `x_wm_supports` (C 27825-27830).  Rust 15,598 -> 15,763 lines.

**What the probes read.**  Baseline = HEAD's unit with the same arm.  Every arm's **net delta is
zero**, and both halves of it are measured: **-2 `E0425`** (`x_wm_supports`'s two existing callers,
which this slice's definition resolves) and **+2 `E0425`** (this slice's own two
`XGetWindowProperty` sites, Decision 35's third shape).  Arm A (`have_x_windows`) **558 / 55 ->
558 / 55**, arm B (A + `have_gtk3` + `use_gtk`) **557 / 60 -> 559 / 60**, arm C (A +
`have_xinput2`) **586 / 57 -> 586 / 57**; no warning moved.

**What `--check` reads.**  **156 -> 154**: `missing function x_get_wm_check_window` and
`missing function x_wm_supports` retire and no token row arrives.  The line-count row moves to
15,763 against C's 33,125 (52.4%).

**Not claimed.**  The frozen configuration compiles none of this file: no window property was
read, no WM check window was looked up and no atom list was walked.  The remaining `E0425` rows
are unresolved names, not behaviour.


### Decision 35's round's readings, as landed and measured (2026-10-06)

**What it lands.**  The `XGetWindowProperty` declaration and the twenty-odd fixes it surfaced
(Decision 36).  Rust 15,763 -> 15,779 lines.

**What the probes read.**  Baseline = HEAD's unit with the same arm.  Arm A (`have_x_windows`)
**558 / 55 -> 545 / 65**: `-13 E0425` (the seven call sites' name), `E0308 13 -> 13` and
`E0381 1 -> 1` (the fixes bring them back to their baseline counts), `+10 warnings` (Deviation 12's
initialisers); arm B **557 / 60 -> 547 / 67** (`-9 E0425`, +7 warnings); arm C
**586 / 57 -> 574 / 67** (`-13 E0425`, +10 warnings).

**What `--check` reads.**  **154 -> 154**: the four `does not mention false` token rows are closed
by `docs/ports/macro-body-notes.tsv` rows.  The line-count row moves to 15,779 against C's 33,125
(52.4%).

**Not claimed.**  The frozen configuration compiles none of this file: no window property was
read and none of the five DND/state functions ran.  The remaining `E0425` rows are unresolved
names, not behaviour, and the new warnings are the initialiser price Decision 36 records.


### Slice 24's readings (C 27832-27908), as landed and measured (2026-10-06)

**What it lands.**  `set_wm_state` (C 27832-27855), `x_set_sticky` (C 27857-27867),
`x_set_shaded` (C 27869-27879) and `x_set_skip_taskbar` (C 27891-27908, both arms).  Rust
15,779 -> 15,913 lines.

**What the probes read.**  Baseline = HEAD's unit with the same arm.  Arm A (`have_x_windows`)
**545 / 65 -> 555 / 65**: **+9 `E0609`** (the `XEvent` members) and **+1 `E0425`** (`XSendEvent`),
with the warning count unmoved; arm B **547 / 67 -> 557 / 67**: the same `+9`/`+1` and no warning;
arm C **574 / 67 -> 583 / 68**: the same `+9`/`+1` and one warning more (the arm's own
`xg_set_skip_taskbar`-less path).

**What `--check` reads.**  **154 -> 150**: four `missing function` rows retire and no token row
arrives.  The line-count row moves to 15,913 against C's 33,125 (52.0%).

**Not claimed.**  The frozen configuration compiles none of this file: no client message was sent,
no taskbar flag was set and no window manager was told anything.  The remaining
`E0425`/`E0609` rows are unresolved names and the mirror's placeholder members, not behaviour.


### Slice 25's readings (C 27910-28085), as landed and measured (2026-10-06)

**What it lands.**  `x_set_z_group` (C 27925-27968, with `FRAME_Z_GROUP`'s writer and its four
gated field writes) and `x_get_current_wm_state` (C 27977-28085, both `USE_XCB` arms and the live
`GCC_LINT` seed).  Rust 15,919 -> 16,164 lines; the header's earlier rows read 15,913 for the same
revision, so this slice measured both sides itself (`git show HEAD:femacs/src/xterm.rs` through
`--check` reports 15,919).

**What the probes read.**  Baseline = HEAD's unit with the same arm, materialised and compiled the
same way.  Arm A (`have_x_windows`) **555 / 65 -> 555 / 66**: no `E0425`, `E0308`, `E0381` or
`E0609` delta at all, and **+1 warning** - `let mut rc: c_int;`, the same class as C 27696's `rc`
four functions above (this arm assigns it once, so `unused_mut` fires); arm B
(`+have_gtk3+use_gtk`) **557 / 67 -> 557 / 68**; arm C (`+have_xinput2`) **583 / 67 -> 583 / 68**.
**Arm reachability** (P-86 ⑦): `let _: c_int = true;` injected as the body's first statement makes
`E0308` 14 -> 15, anchored exactly at the injected line; removing it returns arm A to 555 / 66.
`xtask build-tu src/xterm.c` builds, with the same warning set as before the slice.

**What `--check` reads.**  **150 -> 148**: `x_set_z_group` and `x_get_current_wm_state` retire as
missing items and no token row arrives - the body mentions every token C's text does, including
C's local `reply_data_object` and the lint arm's `reply_data_dummy`.  The line-count row moves to
16,164 against C's 33,125 (48.8%).

**Not claimed.**  The frozen configuration compiles none of this file: no `_NET_WM_STATE` was
read, no z-group was set and no frame was iconified.  The remaining `E0425`/`E0609` rows are
unresolved names and the mirror's placeholder members, not behaviour.


### Slice 26's readings (C 28087-28329), as landed and measured (2026-10-06)

**What it lands.**  `do_ewmh_fullscreen` (C 28089-28219, both `want_fullscreen` accessors and the
five-label `match`), `XTfullscreen_hook` (C 28221-28230), `x_handle_net_wm_state` (C 28233-28265)
and `x_check_fullscreen` (C 28269-28329), with the three use-site declarations and the
`XPropertyEvent` record they need.  Rust 16,164 -> 16,559 lines.

**What the probes read.**  Baseline = HEAD's unit with the same arm.  Arm A (`have_x_windows`)
**555 / 66 -> 555 / 68**: no error delta at all - the two undeclared calls live inside the
`have_window_system` gate - and **+2 warnings** (`unused_mut` on the two `lval` locals whose only
reassignment the gate hides); arm B (`+have_gtk3+use_gtk`) **557 / 68 -> 558 / 70**: the same two
warnings plus **one more instance** of the `GDK_WINDOW_XID` class `include/xterm.rs` 1060's GTK arm
already carries five of (C 28097 is this slice's second `FRAME_OUTER_WINDOW` use site); arm C
(`+have_xinput2`) **583 / 68 -> 583 / 70**.  **Arm reachability** (P-86 ⑦): `let _: c_int = true;`
as `do_ewmh_fullscreen`'s first statement moves `E0308` 14 -> 15, anchored exactly at that line.

**What `--check` reads.**  **148 -> 144**: the four functions retire as missing items and no token
row arrives - the three `switch`-family rows are `docs/ports/macro-body-notes.tsv`'s.  The
line-count row moves to 16,559 against C's 33,125 (50.0%).

**Not claimed.**  The frozen configuration compiles none of this file: no fullscreen request was
made, no `_NET_WM_STATE` was read, no frame was resized and no window manager was asked anything.


### Slice 27's readings (C 28336-28475), as landed and measured (2026-10-06)

**What it lands.**  `x_check_expected_move` (C 28339-28374), `x_sync_with_move` (C 28383-28427) and
`x_wait_for_event` (C 28430-28475), with the two use-site declarations they need.  Rust
16,559 -> 16,764 lines.

**What the probes read.**  Baseline = HEAD's unit with the same arm.  Arm A (`have_x_windows`)
**555 / 68 -> 554 / 68**: **-1 `E0425`** - the two rows C 27661/27679's `x_set_offset` carried for
`x_sync_with_move` and `x_check_expected_move` retire, and this slice's own `ConnectionNumber` site
(C 28439) adds one back - with no warning delta; arm B (`+have_gtk3+use_gtk`)
**558 / 70 -> 558 / 70**: the same `-2 +1`, plus one more instance of the `GDK_WINDOW_XID` class
under `FRAME_OUTER_WINDOW` (C 28365, the class slice 26 named); arm C (`+have_xinput2`)
**583 / 70 -> 582 / 70**.  **Arm reachability** (P-86 ⑦): `let _: c_int = true;` as
`x_check_expected_move`'s first statement moves `E0308` 14 -> 15, anchored exactly at that line.

**What `--check` reads.**  **144 -> 141**: the three functions retire as missing items and no token
row arrives.  The line-count row moves to 16,764 against C's 33,125 (50.6%).

**Not claimed.**  The frozen configuration compiles none of this file: no window was moved, no
position was read back, no timeout was waited on and no frame was resized.


### Slice 28's readings (C 28476-28621), as landed and measured (2026-10-06)

**What it lands.**  `x_set_window_size_1` (C 28483-28555, both `USE_X_TOOLKIT` arms and the
`USE_CAIRO` surface call), `x_suspend_background_fills` (C 28564-28570),
`x_restore_background_fills` (C 28573-28579) and `x_set_window_size` (C 28585-28621, both `USE_GTK`
arms), with five use-site declarations.  Rust 16,764 -> 17,015 lines.

**What the probes read.**  Baseline = HEAD's unit with the same arm.  Arm A (`have_x_windows`)
**554 / 68 -> 556 / 68**: **+2 `E0425`** - C 28489's undeclared `x_wm_set_size_hint` and the `void`
row C 28497/28505's `FRAME_MENUBAR_HEIGHT` expansion brings (the owed mirror body) - with no
warning delta; arm B (`+have_gtk3+use_gtk`) **558 / 70 -> 561 / 70**: the same two plus one more
instance of the `GDK_WINDOW_XID` class under `FRAME_OUTER_WINDOW` (C 28504); arm C
(`+have_xinput2`) **582 / 70 -> 584 / 70**.  **Arm reachability** (P-86 ⑦): `let _: c_int = true;`
as `x_set_window_size_1`'s first statement moves `E0308` 14 -> 15, anchored exactly at that line.

**What `--check` reads.**  **141 -> 137**: the four functions retire as missing items and no token
row arrives.  The line-count row moves to 17,015 against C's 33,125 (51.4%).

**Not claimed.**  The frozen configuration compiles none of this file: no window was resized, no
inner widget was touched, no background pixmap was swapped and no frame was redisplayed.


### Slice 29's readings (C 28623-28743), as landed and measured (2026-10-06)

**What it lands.**  `x_set_window_size_and_position_1` (C 28623-28663),
`x_set_window_size_and_position` (C 28665-28700, both `USE_GTK` arms),
`frame_set_mouse_pixel_position` (C 28704-28726, both `HAVE_XINPUT2` arms) and `x_raise_frame`
(C 28730-28743).  Rust 17,015 -> 17,232 lines.

**What the probes read.**  Baseline = HEAD's unit with the same arm.  Arm A (`have_x_windows`)
**556 / 68 -> 562 / 68**: **+6 `E0425`** - `x_wm_set_size_hint`'s second instance, `XFlush`'s third,
`XMoveResizeWindow`'s second, `XRaiseWindow`, `XWarpPointer`, and the `void` row of the two
`FRAME_MENUBAR_HEIGHT` expansions - with no warning delta; arm B (`+have_gtk3+use_gtk`)
**561 / 70 -> 569 / 70**: those six plus **two more `GDK_WINDOW_XID` instances** (C 28651 and
C 28738, the class the earlier slices carry); arm C (`+have_xinput2`) **584 / 70 -> 592 / 70**: the
six, plus `XIWarpPointer` and one `E0609` for `x_display_info`'s owed `client_pointer_device`
(`include/xterm.rs` 662-666).  **Arm reachability** (P-86 ⑦): `let _: c_int = true;` as
`x_set_window_size_and_position_1`'s first statement moves `E0308` 14 -> 15, anchored exactly at
that line.

**What `--check` reads.**  **137 -> 133**: the four functions retire as missing items and no token
row arrives.  The line-count row moves to 17,232 against C's 33,125 (52.0%).

**Not claimed.**  The frozen configuration compiles none of this file: no window was moved or
resized, no pointer was warped and no frame was raised.


### Slice 30's readings (C 28745-28819), as landed and measured (2026-10-06)

**What it lands.**  `x_lower_frame_1` (C 28745-28774, the stack-pool block), `x_lower_frame`
(C 28776-28798, the `HAVE_XWIDGETS` arm included), `XTframe_raise_lower` (C 28800-28807) and
`xembed_request_focus` (C 28811-28819), with the `CurrentTime` use-site constant.  Rust
17,232 -> 17,339 lines.

**What the probes read.**  Baseline = HEAD's unit with the same arm.  Arm A (`have_x_windows`)
**562 / 68 -> 566 / 68**: **+4 `E0425`** - `XRestackWindows`, `XLowerWindow`, `XFlush`'s fourth
instance and `xembed_send_message` - with no warning delta; arm B (`+have_gtk3+use_gtk`)
**569 / 70 -> 576 / 70**: those four plus **three more `GDK_WINDOW_XID` instances** (C 28769/28770
and C 28787, the class the earlier slices carry); arm C (`+have_xinput2`)
**592 / 70 -> 596 / 70**: the four, exactly.  **Arm reachability** (P-86 ⑦): `let _: c_int = true;`
as `x_lower_frame_1`'s first statement moves `E0308` 14 -> 15, anchored exactly at that line.

**What `--check` reads.**  **133 -> 129**: the four functions retire as missing items and the one
token row this slice brings (`alloca`) is closed by a `docs/ports/macro-body-notes.tsv` row.  The
line-count row moves to 17,339 against C's 33,125 (52.3%).

**Not claimed.**  The frozen configuration compiles none of this file: no window was lowered or
raised and no focus request was sent.


### Slice 31's readings (C 28821-28947), as landed and measured (2026-10-06)

**What it lands.**  `server_timestamp_predicate` (C 28821-28833), `x_get_server_time`
(C 28838-28877, the `HAVE_XSYNC` arm included) and `x_ewmh_activate_frame` (C 28881-28947), with
the `PropertyNotify` use-site constant.  Rust 17,339 -> 17,495 lines.

**What the probes read.**  Baseline = HEAD's unit with the same arm.  Arm A (`have_x_windows`)
**566 / 68 -> 586 / 68**: **+5 `E0425`** (`XChangeProperty`, `XIfEvent`, `XSendEvent`,
`XRaiseWindow`, `x_set_input_focus`) and **+15 `E0609`** (the `XEvent`-member class), with no
warning delta; arm B (`+have_gtk3+use_gtk`) **576 / 70 -> 602 / 70**: those twenty plus **six more
`GDK_WINDOW_XID` instances** (C 28865/28868 and C 28896/28903/28935/28937); arm C
(`+have_xinput2`) **596 / 70 -> 616 / 70**: the twenty, exactly.  **Arm reachability** (P-86 ⑦):
`let _: c_int = true;` as `server_timestamp_predicate`'s first statement moves `E0308` 14 -> 15,
anchored exactly at that line.

**What `--check` reads.**  **129 -> 126**: the three functions retire as missing items and this
slice brings no token row (its `False` argument is spelled the way the file's sibling `XSendEvent`
sites spell it).  The line-count row moves to 17,495 against C's 33,125 (52.8%).

**Not claimed.**  The frozen configuration compiles none of this file: no server time was
round-tripped, no property was changed and no activation message was sent.


### Slice 32's readings (C 28949-29139), as landed and measured (2026-10-06)

**What it lands.**  `x_get_focus_frame` (C 28949-28961), `x_set_input_focus` (C 28963-29012, both
`HAVE_XINPUT2` arms) and `x_focus_frame` (C 29023-29139, the `HAVE_GTK3` arm and the `'out`
labelled block included).  Rust 17,495 -> 17,707 lines.

**What the probes read.**  Baseline = HEAD's unit with the same arm.  Arm A (`have_x_windows`)
**586 / 68 -> 588 / 68**: **+1 `E0425`** (`RevertToParent`; `XSetInputFocus` is one row while
`x_set_input_focus`'s slice-31 row retires) and **+1 `E0609`** (`explicit_parent`), with no warning
delta; arm B (`+have_gtk3+use_gtk`) **602 / 70 -> 606 / 70**: the same two plus **two more
`GDK_WINDOW_XID` instances** (C 29126/29130); arm C (`+have_xinput2`)
**616 / 70 -> 621 / 70**: the same two plus `XISetFocus` and two `client_pointer_device` `E0609`
rows.  **Arm reachability** (P-86 ⑦): `let _: c_int = true;` as `x_get_focus_frame`'s first
statement moves `E0308` 14 -> 15, anchored exactly at that line.

**What `--check` reads.**  **126 -> 123**: the three functions retire as missing items and the one
token row this slice brings (`goto`) is closed by a `docs/ports/macro-body-notes.tsv` row.  The
line-count row moves to 17,707 against C's 33,125 (53.5%).

**Not claimed.**  The frozen configuration compiles none of this file: no focus request was made,
no window manager was asked to activate anything and no frame was raised.


### Slice 33's readings (C 29142-29189), as landed and measured (2026-10-06)

**What it lands.**  `xembed_set_info` (C 29150-29162, under C 29144's gate, with `XEMBED_VERSION`
as both a macro and a constant) and `xembed_send_message` (C 29165-29189, ungated).  Rust
17,707 -> 17,777 lines.

**What the probes read.**  Baseline = HEAD's unit with the same arm.  Arm A (`have_x_windows`)
**588 / 68 -> 598 / 68**: **+1 `E0425`** (the two Xlib entry points the block names already carry
rows, so their new sites add one) and **+9 `E0609`** (`event.xclient.*`), with no warning delta;
arm B (`+have_gtk3+use_gtk`) **606 / 70 -> 615 / 70**: the nine rows and **no** `E0425` (the
setter's `XChangeProperty` is gated out there and `XSendEvent`'s name is already unresolved); arm C
(`+have_xinput2`) **621 / 70 -> 631 / 70**: the same ten as arm A.  **Arm reachability** (P-86 ⑦):
`let _: c_int = true;` as `xembed_set_info`'s first statement moves `E0308` 14 -> 15, anchored
exactly at that line.

**What `--check` reads.**  **123 -> 119**: the two functions and the two `XEMBED_VERSION` rows
retire, and no token row arrives.  The line-count row moves to 17,777 against C's 33,125 (53.7%).

**Not claimed.**  The frozen configuration compiles none of this file: no XEmbed property was set
and no client message was sent.

### Slice 34's readings (C 29201-29400), as landed and measured (2026-10-06)

**What it lands.**  `x_make_frame_visible` (C 29201-29400): the parent-frame arm (C 29210-29229,
`USE_GTK`'s `XMoveWindow` + `gtk_widget_show_all` against `XMapRaised`), the map arm (C 29231-29300:
`gui_set_bitmap_icon`, the `!USE_GTK` user-time-window property write, `asked_for_visible`,
`x_wm_set_window_state (f, NormalState)`, and the three-way `USE_X_TOOLKIT`/`USE_GTK`/other map
call), and the synchronize block (C 29304-29399: `previously_visible`, `XSETFRAME`, `unblock_input`,
the geometry re-check with `block_input` in C 29346's own place, the `CYGWIN` poll arm,
`x_wait_for_event (f, MapNotify)` and the flush).  With it come two use-site entries, `MapNotify`
(X.h 578) and `frame_size_history_plain` (frame.h 1514).  Rust 17,777 -> 18,033 lines.

**What the probes read.**  Baseline = HEAD's unit with the same arm, both sides measured.  The totals
are the probe's `^error` count (the convention the earlier slices' readings use); the per-class
numbers are its `^error[E..]` headers.  The two differ by rustc's *uncoded* errors, which only
arm B has: `cannot find macro GDK_WINDOW_XID`, the `FRAME_GTK_OUTER_WIDGET` expansion
(`include/pgtkterm.rs` 1060) that 21 earlier sites already carry.

| arm | baseline | slice 34 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 598 / 68 | **612 / 68** | **+14** | `E0425` 356 -> 363, `E0609` 206 -> 213 |
| B `+have_gtk3+use_gtk` | 615 / 70 | **627 / 70** | **+12** | `E0425` 346 -> 349, `E0609` 207 -> 213, `GDK_WINDOW_XID` 21 -> 24 |
| C `+have_xinput2` | 631 / 70 | **645 / 70** | **+14** | `E0425` 373 -> 380, `E0609` 222 -> 229 |

The seven new `E0609` rows in arms A and C are `asked_for_visible` x2, `explicit_parent` (1 -> 5)
and `has_been_visible` (2 -> 3) - the `x_output` bitfield class (P-80).  The seven new `E0425` rows
there are **two new names** (`gui_set_bitmap_icon` and `x_wm_set_window_state`, one site each) plus
**five new sites** of names earlier slices already owed (`XMapRaised` x2, `XGetGeometry`,
`XChangeProperty`, `XDeleteProperty`).  Arm B adds the two same classes at the sites its own
configuration compiles and three *uncoded* rows at C 29218/29284/29285 - the GTK map calls, which
only it compiles.  Warning counts are unchanged in every arm (68 / 70 / 70).

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as the function's first statement moves
`E0308` 13 -> 14, anchored exactly at that line (`_root.rs:17817`); removing it returns arm A to
611 coded errors / 612 total / 68 warnings.

**What `--check` reads.**  **119 -> 118**: `x_make_frame_visible` retires as a missing item, and no
token row arrives.  The line-count row moves to 18,033 against C's 33,125 (45.6%).

**Not claimed.**  The frozen configuration compiles none of this file: no frame was mapped, no
window manager was asked anything and no `MapNotify` was waited for.

### Slice 35's readings (C 29406-29481), as landed and measured (2026-10-06)

**What it lands.**  `x_make_frame_invisible` (C 29406-29472: the outermost-window name, the
highlight-frame reset, `block_input`, the `HAVE_XINPUT2_2` touch-point unlink, `x_wm_set_size_hint
(f, 0, true)`, the parent-frame suspend/restore pair, the `USE_GTK`/XEmbed withdraw arms and
`XWithdrawWindow`, the `XSync` under `Vx_lax_frame_positioning`, `SET_FRAME_VISIBLE! (f, false)` /
`SET_FRAME_ICONIFIED (f, false as c_int)`, the size-history line and `unblock_input`) and the
`static` dispatcher `x_make_frame_visible_invisible` (C 29474-29481).  It also re-spells
`xembed_set_info`'s parameter as `c_int` per P-98 (Decision 48).  Rust 18,033 -> 18,143 lines.

**What the probes read.**  Baseline = HEAD's unit (slice 34's commit) with the same arm, both sides
measured.  Totals are the probe's `^error` count, classes are its `^error[E..]` headers.

| arm | baseline | slice 35 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 612 / 68 | **616 / 68** | **+4** | `E0425` 363 -> 366, `E0609` 213 -> 214 |
| B `+have_gtk3+use_gtk` | 627 / 70 | **631 / 70** | **+4** | `E0425` 349 -> 352, `GDK_WINDOW_XID` 25 -> 26 |
| C `+have_xinput2` | 645 / 70 | **649 / 70** | **+4** | `E0425` 380 -> 383, `E0609` 229 -> 230 |

The three new `E0425` sites per arm are `XWithdrawWindow` (**a new name**) plus one more site each
for `DefaultScreen` and `x_wm_set_size_hint`; the one new `E0609` is `explicit_parent`
(`FRAME_PARENT_FRAME`).  Arm B's `E0609` count does not move (its `USE_GTK` branch does not read
`FRAME_X_EMBEDDED_P`), and its uncoded `GDK_WINDOW_XID` class goes 25 -> 26 at `FRAME_GTK_OUTER_WIDGET`
(C 29436).  Warning counts are unchanged in every arm (68 / 70 / 70), and `build-tu` prints the same
30 warnings as the baseline.

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as `x_make_frame_invisible`'s first statement
moves `E0308` 13 -> 14, anchored exactly at that line (`_root.rs:18072`); removing it returns arm A
to 616 / 68.

**What `--check` reads.**  **118 -> 116**: both functions retire as missing items, the re-spelled
parameter adds no row, and no token row arrives.  The line-count row moves to 18,143 against C's
33,125 (45.2%).

**Not claimed.**  The frozen configuration compiles none of this file: no window was withdrawn, no
touch point was unlinked and no `XSync` reached a server.

### Slice 36's readings (C 29483-29614), as landed and measured (2026-10-06)

**What it lands.**  `x_create_font_cursor` (C 29483-29492) and `x_iconify_frame` (C 29497-29614:
the highlight reset, the early return for an already-iconified frame, `gui_set_bitmap_icon`, the
GTK arm, the `USE_X_TOOLKIT` arm with `XIconifyWindow`/`XtWindow`, and the `#else` arm with the
X11R4 `ClientMessage`, the X11R3 hints write and the map).  Plus `IconicState` (Xutil.h 148) at its
use site.  Rust 18,143 -> 18,300 lines.

**What the probes read.**  Baseline = HEAD's unit (slice 35's commit) with the same arm, both sides
measured.  Totals are the probe's `^error` count, classes are its `^error[E..]` headers.

| arm | baseline | slice 36 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 616 / 68 | **632 / 68** | **+16** | `E0425` 366 -> 372, `E0609` 214 -> 224 |
| B `+have_gtk3+use_gtk` | 631 / 70 | **647 / 70** | **+16** | `E0425` 352 -> 358, `E0609` 213 -> 223 |
| C `+have_xinput2` | 649 / 70 | **665 / 70** | **+16** | `E0425` 383 -> 389, `E0609` 230 -> 240 |

The six `E0425` sites per arm are `XCreateFontCursor` (**a new name**) plus one more site each for
`XFlush`, `XMapRaised`, `XSendEvent`, `gui_set_bitmap_icon` and `x_wm_set_window_state`.  The ten
`E0609` are the nine `msg.xclient.*` writes (C 29577-29585, the mirror's opaque `XEvent` union) and
one `explicit_parent` (`FRAME_X_EMBEDDED_P`, C 29566).  Warning counts are unchanged in every arm
(68 / 70 / 70) and `build-tu` prints the same 30 warnings as the baseline.

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as `x_iconify_frame`'s first statement after
its `use` line moves `E0308` 13 -> 14, anchored exactly at that line (`_root.rs:18190`); removing it
returns arm A to 632 / 68.

**What `--check` reads.**  **116 -> 114**: both functions retire as missing items and no token row
arrives.  The line-count row moves to 18,300 against C's 33,125 (44.8%).

**Not claimed.**  The frozen configuration compiles none of this file: no frame was iconified and no
client message was sent.

### Slice 37's readings (C 29619-29851), as landed and measured (2026-10-06)

**What it lands.**  `x_free_frame_resources` (C 29619-29851): the display/frame lookups, the
`HAVE_XINPUT2`/`HAVE_XINPUT2_2` unlinks, `free_frame_faces`, the `if (dpyinfo->display)` body (the
pointer restore, `tear_down_x_back_buffer`, the icon-window destroy, the `USE_X_TOOLKIT` scroll-bar
loop and widget destroy, `HAVE_X_I18N`'s XIC free, `USE_CAIRO`, the `!USE_X_TOOLKIT` widget/
xwidget paths, the `HAVE_XSYNC` counters, the twelve `unload_color` calls with their pixel tests (ten in the arms measured - C's two
`USE_LUCID` shadow colours are gated out),
`x_free_gcs`, the two relief GCs, the fifteen cursor frees, the sync fences and the
`USE_TOOLKIT_SCROLL_BARS` counter), the `HAVE_GTK3` css-provider unrefs, and the display bookkeeping
(`motif_drag_atom_owner`, the three focus frames, the mouse-highlight reset, the two last-mouse
frames, the XInput2 focus change) before `unblock_input`.  Rust 18,300 -> 18,635 lines.

**What the probes read.**  Baseline = HEAD's unit (slice 36's commit) with the same arm, both sides
measured.  Totals are the probe's `^error` count, classes are its `^error[E..]` headers.

| arm | baseline | slice 37 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 632 / 68 | **664 / 68** | **+32** | `E0425` 372 -> 404 |
| B `+have_gtk3+use_gtk` | 647 / 70 | **684 / 70** | **+37** | `E0425` 358 -> 391, `E0609` 223 -> 227 |
| C `+have_xinput2` | 665 / 70 | **699 / 70** | **+34** | `E0425` 389 -> 423 |

Every added row is the recorded bare-name class: **`XFreeCursor` (15 sites) and `unload_color` (10)
are new names**, `XDestroyWindow` gains the two sites the display guard adds, and
`free_frame_faces`, `tear_down_x_back_buffer` (2), `x_free_gcs`, `XTtoggle_invisible_pointer` (this
file's C 12142, still ahead of the lane) are one site each; arm C additionally compiles the two
`HAVE_XINPUT2` calls (`xi_handle_delete_frame`, `xi_handle_focus_change`) and arm B the
`USE_GTK`-only `xg_free_frame_widgets`.  Arm B's four `E0609` are C 29808-29814's pgtk-gated
css-provider fields (Decision 50).  **No `E0308`/`E0381`/warning is added by the final text** - the
two `E0308`s the first writing produced (the `mouse_face_mouse_frame` cast and `supports_xi2`'s
bool) are fixed and re-measured - and `build-tu` prints the same 30 warnings as the baseline.

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as the function's first statement moves
`E0308` 13 -> 14, anchored exactly at that line (`_root.rs:18345`); removing it returns arm A to
664 / 68.

**What `--check` reads.**  **114 -> 113**: `x_free_frame_resources` retires as a missing item and no
token row arrives.  The line-count row moves to 18,635 against C's 33,125 (43.7%).

**Not claimed.**  The frozen configuration compiles none of this file: no X resource was freed, no
widget or window was destroyed and no cursor was released.

### Slice 38's readings (C 29856-29889), as landed and measured (2026-10-06)

**What it lands.**  `x_destroy_window` (C 29856-29889): the display lookup, the `display != 0` guard
around `x_free_frame_resources`, the three `xfree`/`XFree` releases (`saved_menu_event`,
`HAVE_X_I18N`'s `preedit_chars`, `HAVE_XINPUT2`'s `xi_masks` under its nested `HAVE_XINPUT2_1`
pair), the `output_data.x` release and the reference count decrement.  Rust 18,635 -> 18,688 lines.

**What the probes read.**  Baseline = HEAD's unit (slice 37's commit) with the same arm, both sides
measured.  Totals are the probe's `^error` count, classes are its `^error[E..]` headers.

| arm | baseline | slice 38 | delta |
| --- | --- | --- | --- |
| A `have_x_windows` | 664 / 68 | **664 / 68** | 0 |
| B `+have_gtk3+use_gtk` | 684 / 70 | **684 / 70** | 0 |
| C `+have_xinput2` | 699 / 70 | **699 / 70** | 0 |

Every name this function uses is already in reach (Decision 51), so no class moved in any arm - the
delta of this slice is the `--check` row, not a probe count.  `build-tu` prints the same 30 warnings
as the baseline.

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as the function's first statement moves
`E0308` 13 -> 14, anchored exactly at that line (`_root.rs:18662`); removing it returns arm A to
664 / 68.

**What `--check` reads.**  **113 -> 112**: `x_destroy_window` retires as a missing item (its one
token row, C 29863's `0`, is written in the port's spelling) and no other row arrives.  The
line-count row moves to 18,688 against C's 33,125 (43.6%).

**Not claimed.**  The frozen configuration compiles none of this file: no window was destroyed and
no display reference was dropped.

### Slice 39's readings (C 29891-29978), as landed and measured (2026-10-06)

**What it lands.**  `x_intern_cached_atom` (C 29896-29978): the two screen-dependent names built
with `sprintf`, the twelve predefined-atom comparisons, the motif drag atom test, the `x_atom_refs`
walk, the `predefined_only` early return and `XInternAtom`.  Plus the unit's `countof` include line,
`INT_STRLEN_BOUND!` and the eight `XA_*` constants the mirror does not carry.  Rust 18,688 -> 18,831
lines.

**What the probes read.**  Baseline = HEAD's unit (slice 38's commit) with the same arm, both sides
measured.  Totals are the probe's `^error` count, classes are its `^error[E..]` headers.

| arm | baseline | slice 39 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 664 / 68 | **670 / 68** | **+6** | `E0425` 404 -> 410 |
| B `+have_gtk3+use_gtk` | 684 / 70 | **690 / 70** | **+6** | `E0425` 391 -> 397 |
| C `+have_xinput2` | 699 / 70 | **705 / 70** | **+6** | `E0425` 423 -> 429 |

The six sites are `x_atom_refs` (**a new name**, the three sites Decision 9's table owes),
`XScreenNumberOfScreen` (two sites - the name is already owed by `FRAME_X_SCREEN_NUMBER`'s uses) and
`XInternAtom` (one site).  The `E0308` the first writing produced (`motif_drag_atom_name`'s array
decay, Decision 52) is fixed and re-measured; no uncoded row and no warning is added, and `build-tu`
prints the same 30 warnings as the baseline.

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as the function's first statement moves
`E0308` 13 -> 14, anchored exactly at that line (`_root.rs:18734`); removing it returns arm A to
670 / 68.

**What `--check` reads.**  **112 -> 111**: `x_intern_cached_atom` retires as a missing item and no
token row arrives.  The line-count row moves to 18,831 against C's 33,125 (43.2%).

**Not claimed.**  The frozen configuration compiles none of this file: no atom was interned, no
screen number was asked for and `x_atom_refs` does not exist yet.

### Slice 40's readings (C 29980-30074), as landed and measured (2026-10-06)

**What it lands.**  `x_get_atom_name` (C 29983-30074): the pointer/need_sync prologue, the stack
pool buffer, the ten-atom `match`, and the default arm (motif drag atom, the two screen-dependent
names, the `x_atom_refs` walk, `XGetAtomName` and the `xstrdup`/`XFree` pair).  Rust 18,831 ->
18,950 lines.

**What the probes read.**  Baseline = HEAD's unit (slice 39's commit) with the same arm, both sides
measured.  Totals are the probe's `^error` count, classes are its `^error[E..]` headers.

| arm | baseline | slice 40 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 670 / 68 | **674 / 68** | **+4** | `E0425` 410 -> 414 |
| B `+have_gtk3+use_gtk` | 690 / 70 | **694 / 70** | **+4** | `E0425` 397 -> 401 |
| C `+have_xinput2` | 705 / 70 | **709 / 70** | **+4** | `E0425` 429 -> 433 |

The four are a net: three more `x_atom_refs` sites (3 -> 6, the second reader), two
`XScreenNumberOfScreen` sites (4 -> 6), one `XGetAtomName` (**a new name**), less the two
`x_get_atom_name` call sites the definition resolves (2 -> 0).  No `E0308`, no uncoded row and no
warning is added, and `build-tu` prints the same 30 warnings as the baseline.

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as the function's first statement moves
`E0308` 13 -> 14, anchored exactly at that line (`_root.rs:18864`); removing it returns arm A to
674 / 68.

**What `--check` reads.**  **111 -> 110**: `x_get_atom_name` retires as a missing item once its four
structural tokens are closed by the notes row, and no other row arrives.  The line-count row moves
to 18,950 against C's 33,125 (42.8%).

**Not claimed.**  The frozen configuration compiles none of this file: no atom name was asked for
and `x_atom_refs` still does not exist.

### Slice 41's readings (C 30076-30146), as landed and measured (2026-10-06)

**What it lands.**  `x_intern_atoms` (C 30083-30121: the `count > 256` shortcut, the
`x_intern_cached_atom` filter loop with `indices`/`new_names`, the `!j` early return, the
`XInternAtoms` round trip and the scatter back through `indices`) and `x_embed_frame`
(C 30128-30144, under C 30123's `#ifndef USE_GTK`: the error-catching `XReparentWindow` and the
`rc` return).  Rust 18,950 -> 19,050 lines.

**What the probes read.**  Baseline = HEAD's unit (slice 40's commit) with the same arm, both sides
measured.  Totals are the probe's `^error` count, classes are its `^error[E..]` headers.

| arm | baseline | slice 41 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 674 / 68 | **677 / 68** | **+3** | `E0425` 414 -> 417 |
| B `+have_gtk3+use_gtk` | 694 / 70 | **696 / 70** | **+2** | `E0425` 401 -> 403 |
| C `+have_xinput2` | 709 / 70 | **712 / 70** | **+3** | `E0425` 433 -> 436 |

The three (two in arm B) are `XInternAtoms` (**a new name**, two sites) and `XReparentWindow`
(**a new name**, one site, arms A/C only).  Two defects the first writing produced are fixed and
re-measured: the two `E0608` from indexing the `*mut Atom` parameter (now `.offset`), and the
`E0609` from `FRAME_OUTPUT_DATA` read without the P-72 rule-3 cast (Decision 54).  No `E0308`, no
uncoded row and no warning is added, and `build-tu` prints the same 30 warnings as the baseline.

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as `x_embed_frame`'s first statement moves
`E0308` 13 -> 14, anchored exactly at that line (`_root.rs:19051`); removing it returns arm A to
677 / 68.

**What `--check` reads.**  **110 -> 108**: both functions retire as missing items and no token row
arrives.  The line-count row moves to 19,050 against C's 33,125 (42.5%).

**Not claimed.**  The frozen configuration compiles none of this file: no atom array was interned
and no window was reparented.

### Slice 42's readings (C 30342-30420), as landed and measured (2026-10-06)

**What it lands.**  `x_wm_set_window_state` (C 30344-30360: the `USE_X_TOOLKIT` Xt arms and the
`#else` arm's three `wm_hints` lines), `x_wm_set_icon_pixmap` (C 30362-30408: the pixmap/mask
lookup, the three-way arm choice) and `x_wm_set_icon_position` (C 30410-30420), plus the four
Xutil.h flag constants they set.  Rust 19,050 -> 19,164 lines.

**What the probes read.**  Baseline = HEAD's unit (slice 41's commit) with the same arm, both sides
measured.  Totals are the probe's `^error` count, classes are its `^error[E..]` headers.

| arm | baseline | slice 42 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 677 / 68 | **680 / 69** | **+3 / +1 warning** | `E0425` 417 -> 420 |
| B `+have_gtk3+use_gtk` | 696 / 70 | **700 / 70** | **+4** | `E0425` 403 -> 406, `GDK_WINDOW_XID` 25 -> 26 |
| C `+have_xinput2` | 712 / 70 | **715 / 71** | **+3 / +1 warning** | `E0425` 436 -> 439 |

The `E0425` net is `XSetWMHints` (a new name, 3 sites in A/C and 2 in B), `image_bitmap_pixmap`
(1), `x_bitmap_mask` (1), arm B's `xg_set_frame_icon` (1), less the two `x_wm_set_window_state`
sites the definition resolves.  The one added warning in arms A and C is Decision 55's parens class
(7 -> 8); **`build-tu` prints the same 30 warnings with an identical set**, and no `E0308`/`E0608`
or uncoded row is added (arm B's extra `GDK_WINDOW_XID` row is `FRAME_OUTER_WINDOW` under
`use_gtk`, the recorded class).

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as `x_wm_set_icon_position`'s first
statement moves `E0308` 13 -> 14, anchored exactly at that line (`_root.rs:19157`); removing it
returns arm A to 680 / 69.

**What `--check` reads.**  **108 -> 105**: the three functions retire as missing items and no token
row arrives.  The line-count row moves to 19,164 against C's 33,125 (42.1%).

**Not claimed.**  The frozen configuration compiles none of this file: no WM hint was written and no
window state was changed.

### Slice 43's readings (C 30158-30340), as landed and measured (2026-10-06)

**What it lands.**  `x_wm_set_size_hint` (C 30158-30340): the `#ifndef USE_GTK` body (the
`USE_X_TOOLKIT` widget arm with its `#ifndef USE_MOTIF` pair, the size-hint fills, the base/minimum
block, C 30295's `goto no_read` as a labelled block, the `XGetWMNormalHints` read-back and the
`PWinGravity` block) with C 30338's `#else` calling `xg_wm_set_size_hint`; plus the ten Xutil.h
flags at the unit's use site and `XSizeHints` in the mirror.  Rust 19,164 -> 19,413; the mirror's
`xlib.rs` 1,341 -> 1,374 lines.

**What the probes read.**  Baseline = HEAD's unit (slice 42's commit) with the same arm, both sides
measured.  Totals are the probe's `^error` count, classes are its `^error[E..]` headers.

| arm | baseline | slice 43 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 680 / 69 | **681 / 69** | **+1** | `E0425` 420 -> 421 |
| B `+have_gtk3+use_gtk` | 700 / 70 | **697 / 70** | **-3** | `E0425` 406 -> 403 |
| C `+have_xinput2` | 715 / 71 | **716 / 71** | **+1** | `E0425` 439 -> 440 |

The net is four resolved `x_wm_set_size_hint` sites against `XGetWMNormalHints`,
`XSetWMNormalHints`, one more `void` (the `FRAME_MENUBAR_HEIGHT!` body) and one more site each for
`x_display_pixel_width`/`x_display_pixel_height`; arm B compiles only the `xg_wm_set_size_hint` arm,
so it is four sites down and one new name up.  **No `E0308`/`E0608`/uncoded row is added** - the
first writing's `E0805` and its six `cannot find macro` rows are fixed and re-measured (Decision 56)
- and `build-tu` prints the same 30 warnings with an identical set.  The mirror change's own legs
are in Decision 56 (83 units, 0 failures; layout-check unchanged).

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as the function's first statement moves
`E0308` 13 -> 14, anchored exactly at that line (`_root.rs:19203`); removing it returns arm A to
681 / 69.

**What `--check` reads.**  **105 -> 104**: `x_wm_set_size_hint` retires as a missing item once its
`goto` token is closed by the notes row, and no other row arrives.  The line-count row moves to
19,413 against C's 33,125 (41.4%).

**Not claimed.**  The frozen configuration compiles none of this file and no arm here sets
`use_x_toolkit`: no size hint was written, no X server was asked for the old hints and the widget
arms are carried text.

### Slice 44's readings (C 30447-30581), as landed and measured (2026-10-06)

**What it lands.**  `x_free_pixmap` (C 30447-30459, both `USE_CAIRO` arms), `same_x_server`
(C 30495-30540, with the two stepped pointer parameters and the `char` comparisons), `get_bits_and_offset`
(C 30545-30565) and `x_display_ok` (C 30570-30581), plus the file-scope `x_initialized` (C 30491);
C 30466-30489's toolkit-only table and flag were recorded as owed text by Decision 57 and are **taken by slice 107 (Decision 120)**: the mirror gained the two `<X11/Xresource.h>` records and the unit carries the eleven entries and the flag.  Rust
19,413 -> 19,564 lines.

**What the probes read.**  Baseline = HEAD's unit (slice 43's commit) with the same arm, both sides
measured.  Totals are the probe's `^error` count, classes are its `^error[E..]` headers.

| arm | baseline | slice 44 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 681 / 69 | **686 / 69** | **+5** | `E0425` 421 -> 426 |
| B `+have_gtk3+use_gtk` | 697 / 70 | **702 / 70** | **+5** | `E0425` 403 -> 408 |
| C `+have_xinput2` | 716 / 71 | **721 / 71** | **+5** | `E0425` 440 -> 445 |

The five sites per arm are `Emacs_Pixmap` (**a new name** - the arm model's narrowing, Decision 57),
`XOpenDisplay`, `XCloseDisplay`, `request_sigio` and `unrequest_sigio`.  The four `E0308` the first
writing produced (C's `ptrdiff_t` into `strncmp`'s `usize`) are fixed and re-measured; no uncoded row
and no warning is added, and `build-tu` prints the same 30 warnings with an identical set.

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as `x_display_ok`'s first statement moves
`E0308` 13 -> 14, anchored exactly at that line (`_root.rs:19571`); removing it returns arm A to
686 / 69.

**What `--check` reads.**  **104 -> 99**: the four functions and the `x_initialized` object retire as
missing items and no token row arrives.  The line-count row moves to 19,564 against C's 33,125
(40.9%).

**Not claimed.**  The frozen configuration compiles none of this file: no pixmap was freed, no
display was opened or closed and no bit mask was counted.

### Slice 45's readings (C 30583-30660), as landed and measured (2026-10-06)

**What it lands.**  `my_log_handler` (C 30583-30591, `USE_GTK`), the file-scope `x_display_id`
(C 30595), `xi_select_hierarchy_events` (C 30602-30622, `HAVE_XINPUT2 && !HAVE_GTK3`) and
`xi_check_toolkit` (C 30636-30658, `HAVE_XINPUT2 && HAVE_GTK3`).  Rust 19,564 -> 19,657 lines.

**What the probes read.**  Baseline = HEAD's unit (slice 44's commit) with the same arm, both sides
measured.  Totals are the probe's `^error` count, classes are its `^error[E..]` headers.

| arm | baseline | slice 45 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 686 / 69 | **686 / 69** | **0** | nothing here is compiled by arm A |
| B `+have_gtk3+use_gtk` | 702 / 70 | **703 / 70** | **+1** | `E0425` 408 -> 409 (`strstr`) |
| C `+have_xinput2` | 721 / 71 | **732 / 71** | **+11** | `E0425` 445 -> 456 |

Arm C's eleven sites are `XISelectEvents` (1), `XISetMask` (3), `XI_DeviceChanged`,
`XI_HierarchyChanged`, `XI_LASTEVENT` and `XI_PropertyEvent` (**new names**), plus one more site
each for `XIEventMask`, `XIMaskLen` and `XIAllDevices`.  No uncoded row and no warning is added in
any arm, and `build-tu` prints the same 30 warnings with an identical set.

**Arm reachability** (P-86 ⑦, taken in arm C - the only arm that compiles a function here).
`let _: c_int = true;` as `xi_select_hierarchy_events`' first statement after the stack-pool macro
moves `E0308` 13 -> 14, anchored exactly at that line (`_root.rs:19618`); removing it returns arm C
to 732 / 71.

**What `--check` reads.**  **99 -> 98**: the file-scope `x_display_id` retires and no token row
arrives (the three functions are not in the frozen table's checklist).  The line-count row moves to
19,657 against C's 33,125 (40.7%).

**Not claimed.**  The frozen configuration compiles none of this file: no log handler ran, no
display id was incremented and no XI event mask was selected.

### Slice 46's readings (C 31845-31867), as landed and measured (2026-10-06)

**What it lands.**  `x_delete_selection_requests` (C 31845-31867): the keyboard-buffer walk with the
`continue`, the `SELECTION_EVENT_DPYINFO` test, `EVENT_INIT` + `NO_EVENT` and the `input_pending`
note.  Rust 19,657 -> 19,694 lines.

**What the probes read.**  Baseline = HEAD's unit (slice 45's commit) with the same arm, both sides
measured.  Totals are the probe's `^error` count, classes are its `^error[E..]` headers.

| arm | baseline | slice 46 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 686 / 69 | **687 / 69** | **+1** | `E0609` 224 -> 225 (`sie`) |
| B `+have_gtk3+use_gtk` | 703 / 70 | **704 / 70** | **+1** | `E0609` 227 -> 228 |
| C `+have_xinput2` | 732 / 71 | **733 / 71** | **+1** | `E0609` 240 -> 241 |

The one row per arm is the recorded `no field sie` class (Decision 59); no `E0425`, uncoded row or
warning is added, and `build-tu` prints the same 30 warnings with an identical set.

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as the declaring statement's next line moves
`E0308` 13 -> 14, anchored exactly at that line (`_root.rs:19683`); removing it returns arm A to
687 / 69.

**What `--check` reads.**  **98 -> 97**: `x_delete_selection_requests` retires as a missing item once
its `continue` token is closed by the notes row.  The line-count row moves to 19,694 against C's
33,125 (40.5%).

**Not claimed.**  The frozen configuration compiles none of this file: no keyboard-buffer event was
walked, invalidated or skipped.

### Slice 47's readings (C 31872-31974), as landed and measured (2026-10-06)

**What it lands.**  `x_delete_display` (C 31872-31974): the terminal walk with the `HAVE_X_SM` arm,
the pending-selection-request unchain with C 31912's backward `goto`, the two selection cleanups,
the `next_noop_dpyinfo`/`mouse_click_timeout_display`/`x_display_list` bookkeeping, the colour-name
cache walk, the five `xfree`s and the four gated arms.  Rust 19,694 -> 19,831 lines.

**What the probes read.**  Baseline = HEAD's unit (slice 46's commit) with the same arm, both sides
measured.  Totals are the probe's `^error` count, classes are its `^error[E..]` headers.

| arm | baseline | slice 47 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 687 / 69 | **691 / 69** | **+4** | `E0425` 426 -> 430 |
| B `+have_gtk3+use_gtk` | 704 / 70 | **708 / 70** | **+4** | `E0425` 409 -> 413 |
| C `+have_xinput2` | 733 / 71 | **737 / 71** | **+4** | `E0425` 456 -> 460 |

The four sites per arm are `next_noop_dpyinfo` (two), `delete_terminal` (one) and
`x_remove_selection_transfers` (one) - all **new names**.  The two defects of the first writing (the
`E0606` cast and the `E0608` pointer indexing, Decision 60) are fixed and re-measured; no uncoded
row and no warning is added, and `build-tu` prints the same 30 warnings with an identical set.

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as the declaring statement's next line moves
`E0308` 13 -> 14, anchored exactly at that line (`_root.rs:19724`); removing it returns arm A to
691 / 69.

**What `--check` reads.**  **97 -> 96**: `x_delete_display` retires as a missing item once its
backward `goto` is closed by the notes row.  The line-count row moves to 19,831 against C's 33,125
(40.3%).

**Not claimed.**  The frozen configuration compiles none of this file: no terminal was deleted, no
selection request was unchained and no colour cache was walked.

### Slice 48's readings (C 32063-32208), as landed and measured (2026-10-06)

**What it lands.**  `x_delete_terminal` (C 32063-32208): the recursive-call guard, the labelled frame
walk, `HAVE_X_I18N`'s `xim_close_dpy`, the bitmap teardown, the `if (dpyinfo->display)` block
(`XSetCloseDownMode`, the scratch GC, the drag-and-drop unwind, the three-way close with C's
`#if false` recorded) and the `else`'s `emacs_close`, the `HAVE_XKB`/modmap frees, the wait-descriptor
delete and `x_delete_display`.  Rust 19,831 -> 19,975 lines.

**What the probes read.**  Baseline = HEAD's unit (slice 47's commit) with the same arm, both sides
measured.  Totals are the probe's `^error` count, classes are its `^error[E..]` headers.

| arm | baseline | slice 48 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 691 / 69 | **697 / 69** | **+6** | `E0425` 430 -> 436 |
| B `+have_gtk3+use_gtk` | 708 / 70 | **714 / 70** | **+6** | `E0425` 413 -> 419 |
| C `+have_xinput2` | 737 / 71 | **743 / 71** | **+6** | `E0425` 460 -> 466 |

The six sites per arm are Decision 61's: four new names and one more site each for
`XCloseDisplay`/`XSetCloseDownMode`.  The three defects of the first writing (the `GC` null test, the
`x_dnd_finish_display` cast, the `XFRAME` bang) are fixed and re-measured; no uncoded row and no
warning is added, and `build-tu` prints the same 30 warnings with an identical set.

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as the declaring statement's next line moves
`E0308` 13 -> 14, anchored exactly at that line (`_root.rs:19861`); removing it returns arm A to
697 / 69.

**What `--check` reads.**  **96 -> 95**: `x_delete_terminal` retires as a missing item once its
backward `goto` is closed by the notes row.  The line-count row moves to 19,975 against C's 33,125
(40.3%).

**Not claimed.**  The frozen configuration compiles none of this file: no terminal was torn down, no
frame was deleted and no display was closed.

### Slice 49's readings (C 32393-32417), as landed and measured (2026-10-06)

**What it lands.**  `init_xterm` (C 32393-32417): the `HAVE_XWIDGETS` environment pair, the
`! HAVE_XINPUT2` GTK/Xinput pair and the `HAVE_X_I18N` registration.  Rust 19,975 -> 20,007 lines.

**What the probes read.**  Baseline = HEAD's unit (slice 48's commit) with the same arm; the three
baseline readings were taken for slice 48's final text (A 697 / 69, B 714 / 70, C 743 / 71) and the
HEAD has not moved since, so this slice's own three runs are compared against them.

| arm | baseline | slice 49 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 697 / 69 | **698 / 69** | **+1** | `E0425` 436 -> 437 (`xputenv`) |
| B `+have_gtk3+use_gtk` | 714 / 70 | **715 / 70** | **+1** | `E0425` 419 -> 420 (`gdk_disable_multidevice`) |
| C `+have_xinput2` | 743 / 71 | **743 / 71** | **0** | the whole block is gated off there |

No uncoded row and no warning is added in any arm, and `build-tu` prints the same 30 warnings with an
identical set.

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as the function's first statement moves
`E0308` 13 -> 14, anchored exactly at that line (`_root.rs:20001`); removing it returns arm A to
698 / 69.  (The first attempt injected the line *between* the signature and its brace - a syntax
error the run reported at once; the second landed it after the `#[cfg]` attribute, which swallowed
it - the third is the one above, and all three are in the turn's probe budget.)

**What `--check` reads.**  **95 -> 94**: `init_xterm` retires as a missing item and no token row
arrives.  The line-count row moves to 20,007 against C's 33,125 (40.4%).

**Not claimed.**  The frozen configuration compiles none of this file: no environment variable was
put and no text-conversion interface was registered.

### Slice 50's readings (C 32210-32303), as landed and measured (2026-10-06)

**What it lands.**  `x_have_any_grab` (C 32210-32227, `HAVE_XINPUT2`) and `x_create_terminal`
(C 32232-32303) with its 42 hook assignments, plus the unit's `hook!` helper.  Rust 20,007 -> 20,190
lines.

**What the probes read.**  Baseline = HEAD's unit (slice 49's commit) with the same arm: A 698 / 69
(coded 697), B 715 / 70 (coded 687), C 743 / 71 (coded 742).

| arm | baseline | slice 50 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 698 / 69 | **719 / 69** | **+21** | `E0425` 436 -> 457, `E0609` 225 -> 226 |
| B `+have_gtk3+use_gtk` | 715 / 70 | **737 / 70** | **+22** | `E0425` 420 -> 441, `E0609` 228 -> 229 |
| C `+have_xinput2` | 743 / 71 | **767 / 71** | **+24** | `E0425` 466 -> 486, `E0609` 241 -> 245 |

Arm A's 21 sites are the twenty hook names the file has not landed plus `create_terminal` and
`x_redisplay_interface`; arm B adds `xw_popup_dialog` (and the same `popup_dialog_hook` field gap);
arm C compiles `x_have_any_grab` as well, which is where its four extra `E0609` (`devices`/
`num_devices` sites) come from.  The `E0609` additions in arms A and B are the `free_pixmap`/
`popup_dialog_hook` field gaps Decision 63 names.  No `E0308`, uncoded row or warning is added by the
final text - the 42 uncoded `cannot find macro hook` rows and the `query_colors` `E0308` of the first
writings are fixed and re-measured - and `build-tu` prints the same 30 warnings.

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as the function's first statement after the
declaration moves `E0308` 13 -> 14, anchored exactly at that line (`_root.rs:20079`); removing it
returns arm A to 719 / 69.

**What `--check` reads.**  **94 -> 93**: `x_create_terminal` retires as a missing item and no token
row arrives.  The line-count row moves to 20,190 against C's 33,125 (39.0% drift).

**Not claimed.**  The frozen configuration compiles none of this file: no terminal was created and no
hook was installed.

### Slice 51's readings (C 32305-32499), as landed and measured (2026-10-06)

**What it lands.**  `x_initialize` (C 32305-32360) and the three `*_for_lisp` error helpers
(C 32475-32499).  Rust 20,190 -> 20,301 lines.

**What the probes read.**  Baseline = HEAD's unit (slice 50's commit) with the same arm: A 719 / 69,
B 737 / 70, C 767 / 71.

| arm | baseline | slice 51 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 719 / 69 | **720 / 69** | **+1** | `E0425` 457 -> 458 (`XSetErrorHandler`) |
| B `+have_gtk3+use_gtk` | 737 / 70 | **738 / 70** | **+1** | `E0425` 441 -> 442 |
| C `+have_xinput2` | 767 / 71 | **768 / 71** | **+1** | `E0425` 486 -> 487 |

One new name per arm, the same one - everything else the four functions touch resolves (Decision 64).
The two `E0308`s the first writings produced (`Some (x_io_error_quitter)` and the five-parameter
`query_colors` of the previous slice's block) are fixed and re-measured; no uncoded row and no warning
is added, and `build-tu` prints the same 30 warnings.

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as the function's first statement moves
`E0308` 13 -> 14, anchored exactly at that line (`_root.rs:20217`); removing it returns arm A to
720 / 69.

**What `--check` reads.**  **93 -> 89**: the four functions retire as missing items and no token row
arrives.  The line-count row moves to 20,301 against C's 33,125 (38.7% drift).

**Not claimed.**  The frozen configuration compiles none of this file: no terminal parameters were
initialised, no error handler was installed and no protocol request was caught.

### Slice 52's readings (C 32419-32662), as landed and measured (2026-10-06)

**What it lands.**  `mark_xterm` (C 32419-32469) and `x_get_keyboard_modifiers` (C 32642-32662).
Rust 20,301 -> 20,403 lines.

**What the probes read.**  Baseline = HEAD's unit (slice 51's commit) with the same arm: A 720 / 69,
B 738 / 70, C 768 / 71.

| arm | baseline | slice 52 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 720 / 69 | **724 / 69** | **+4** | `E0425` 458 -> 462 (`mark_object` x3, `x_find_modifier_meanings`) |
| B `+have_gtk3+use_gtk` | 738 / 70 | **744 / 70** | **+6** | `E0425` 442 -> 447, `E0609` 229 -> 230 |
| C `+have_xinput2` | 768 / 71 | **775 / 71** | **+7** | `E0425` 487 -> 492, `E0609` 245 -> 247 |

Arm B's extra `mark_object` site and its `E0609` are C 32462's `last_monitor_attributes_list`; arm
C's fourth `mark_object` and its two `E0609` rows are C 32454-32455's `devices`/`num_devices` loop.
No `E0308`, uncoded row or warning is added, and `build-tu` prints the same 30 warnings.

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as the function's first statement moves
`E0308` 13 -> 14, anchored exactly at that line (`_root.rs:20329`); removing it returns arm A to
724 / 69.

**What `--check` reads.**  **89 -> 87**: both functions retire as missing items and no token row
arrives.  The line-count row moves to 20,403 against C's 33,125 (38.4% drift).

**Not claimed.**  The frozen configuration compiles none of this file: no frame object was marked, no
display walk happened and no modifier meanings were fetched.

### Slice 53's readings (C 32505-32635), as landed and measured (2026-10-06)

**What it lands.**  `x_preserve_selections` (C 32505-32635).  Rust 20,403 -> 20,592 lines; the
mirror `femacs/src/include/xterm.rs` gains `selection_input_event_bits!`'s setter arm.

**What the probes read.**  Baseline = HEAD's unit (slice 52's commit) with the same arm: A 722 / 69,
B 742 / 70, C 773 / 71.

| arm | baseline | slice 53 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 722 / 69 | **723 / 69** | **+1** | `E0425` 462 -> 463 (`XGetSelectionOwner`) |
| B `+have_gtk3+use_gtk` | 742 / 70 | **743 / 70** | **+1** | `E0425` 447 -> 448 |
| C `+have_xinput2` | 773 / 71 | **774 / 71** | **+1** | `E0425` 492 -> 493 |

One new name per arm and it is the same name in all three: everything else the function touches
resolves, the three body prototypes above included.  A first writing read **728 / 69** - the
`E0658` of an attribute on an expression statement plus its companion "removing an expression is
not supported in this position" - and that arm is a block now; the second writing read
**723 / 69**, which is where the table's numbers come from.  No `E0308`, uncoded row or warning is
added: `build-tu` prints the same 30 warnings, and the two runs' warning lines are identical once
the location columns are stripped.

The absolute counts here are **2 below the ones this ledger's slice-52 entry records for the same
commit** (722/742/773 against 724/744/775; measured this turn on `HEAD`'s own unit with the arm's
flags, `--emit=metadata` and `--emit=obj` reading the same, and the mirror change reading
identically with the old and the new mirror).  The *delta* is therefore the comparable reading and
the offset is recorded rather than explained.

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as the function's first statement moves
`E0308` 13 -> 14, anchored exactly at that line (`_root.rs:20471`); removing it returns arm A to
723 / 69.

**What `--check` reads.**  **87 -> 86**: `x_preserve_selections` retires as a missing item once its
`alloca` token is closed by a `docs/ports/macro-body-notes.tsv` row.  The line-count row moves to
20,592 against C's 33,125 (37.8% drift).

**What the mirror's change cost.**  P-87 ①'s leg reads `checked 83 unit(s), 0 failure(s)` and
`checking 83 unit(s), 0 unit failure(s)` (exit 0, 1m33s), and `check-mirror-dupes` stays 16/16/0.

**Not claimed.**  The frozen configuration compiles none of this file: no selection was preserved,
no owner was handed over and no selection-clear event was pushed.  C's `USE_XCB` arm is carried as
text and compiled by no arm here.

### Slice 54's readings (C 32664-33125), as landed and measured (2026-10-06)

**What it lands.**  `syms_of_xterm` (C 32664-33125) and the 39 `DEFVAR_*` objects this unit owns.
Rust 20,592 -> 20,969 lines.

**What the probes read.**  Baseline = HEAD's unit (slice 53's commit) with the same arm: A 723 / 69,
B 743 / 70, C 774 / 71.

| arm | baseline | slice 54 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 723 / 69 | **723 / 69** | **0** | every name the body touches resolves |
| B `+have_gtk3+use_gtk` | 743 / 70 | **743 / 70** | **0** | the `USE_GTK` icon block resolves too |
| C `+have_xinput2` | 774 / 71 | **774 / 71** | **0** | - |

Zero on all three arms, and it is a real zero rather than an uncompiled slice: every name the
function touches resolves through the mirror (`Q*`, `DEFVAR_*!`/`DEFSYM!`, `Fput`, `Fcons`,
`staticpro`, `PDUMPER_IGNORE!`, `build_string`, `intern_c_string`, `make_float`, `make_hash_table`,
`list1`/`list2`, `make_fixnum` and the five modifier constants) or through this unit's own new
definitions.  No `E0308`, uncoded row or warning is added: `build-tu` prints the same 30 warnings,
and the two runs' warning lines are identical once the location columns are stripped.

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as the function's first statement moves
`E0308` 13 -> 14, anchored exactly at that line (`_root.rs:20708`); removing it returns arm A to
723 / 69 - which is what makes the three zeros above a reading rather than an absence.

**What `--check` reads.**  **86 -> 85**: `syms_of_xterm` retires as a missing item once its `doc`
token is closed by a `docs/ports/macro-body-notes.tsv` row.  The line-count row moves to 20,969
against C's 33,125 (36.7% drift).

**What the new definitions cost and return.**  `tools/check-defvar-carriers.py` reads **722**
`DEFVAR_*` site(s) (683 before this slice, so the 39 new sites are exactly this function's) with
0 failure(s): each definition's declared type matches what its macro dereferences (P-97).
`check-declarations` stays `18 declaration sites (0 forbidden, 18 allowed literals)` and
`check-own-voice` stays at 814 sites with 0 in this repository's own voice.  No mirror changed, so
P-87's legs do not apply.

**Not claimed.**  The frozen configuration compiles none of this file: no variable was registered,
no symbol was defined and no icon file was built.  The `USE_TOOLKIT_SCROLL_BARS` arm chain is
carried as text; no arm here sets that feature, so C 32766's `#else` arm is the one the readings
compile.

### Slice 55's readings (C 11924-12278), as landed and measured (2026-10-06)

**What it lands.**  `x_frame_highlight` (C 11924-11949), `x_frame_unhighlight` (C 11951-11981),
`x_new_focus_frame` (C 11989-12033), the two Xfixes statics `x_fixes_pointer_blanking_supported`
(C 12040-12045) and `xfixes_toggle_visible_pointer` (C 12053-12062), `x_toggle_visible_pointer`
(C 12099-12139), `XTtoggle_invisible_pointer` (C 12141-12153), `x_focus_changed` (C 12162-12225) and
`x_window_to_frame` (C 12231-12278), plus this unit's `set_frame_flag1!`/`frame_bits!` pair.  Rust
20,969 -> 21,416 lines.

**What the probes read.**  Baseline = HEAD's unit (slice 54's commit) with the same arm: A 723 / 69,
B 743 / 70, C 774 / 71.

| arm | baseline | slice 55 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 723 / 69 | **729 / 69** | **+6** | `E0425` 463 -> 467, `E0609` 226 -> 228 |
| B `+have_gtk3+use_gtk` | 743 / 70 | **749 / 70** | **+6** | `E0425` 448 -> 452, `E0609` 230 -> 232 |
| C `+have_xinput2` | 774 / 71 | **780 / 71** | **+6** | `E0425` 497, `E0609` 247 -> 249 |

The `E0425` net is seven new bare names (`XSetWindowBorder`, `XSetWindowBorderPixmap`,
`XDefineCursor` twice, `FocusIn`, `FocusOut`, `x_frame_rehighlight` - the last of which is this
file's own definition still ahead of the lane) minus the three call sites earlier slices owed and
this slice resolves (`x_window_to_frame` at unit 2234, `XTtoggle_invisible_pointer` at 18372 and its
hook line in `x_create_terminal`).  The two `E0609` are Decision 68's `alpha_identical_p` sites.
An earlier writing of this slice read **+7** with one `E0308`: the call site fix Decision 68 records
turned it into the final +6.  No warning is added - `build-tu` prints the same 30, with the two
runs' warning lines identical once the location columns are stripped.

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as `x_frame_highlight`'s first statement
moves `E0308` 13 -> 14, anchored exactly at that line (`_root.rs:21025`); removing it returns arm A
to 729 / 69.

**What `--check` reads.**  **85 -> 78**: the seven functions retire as missing items, and the one
token row the slice brings (`x_window_to_frame`'s `0,continue`) is closed by a
`docs/ports/macro-body-notes.tsv` row.  The line-count row moves to 21,416 against C's 33,125
(35.3% drift).

**Not claimed.**  The frozen configuration compiles none of this file: no frame was highlighted, no
cursor was defined and no window was mapped back to a frame.  The `HAVE_X_I18N` blocks (C
12178-12192, 12209-12219), the GTK-XInput2 arm of `x_new_focus_frame`, the `USE_X_TOOLKIT` arm of
`x_window_to_frame` and the whole `have_xfixes` half are carried as text and compiled by no arm
here; their names are unmeasured.

### Slice 56's readings (C 12344-12538), as landed and measured (2026-10-06)

**What it lands.**  `x_any_window_to_frame` (C 12349-12401, the toolkit/GTK arm), the macro
`x_any_window_to_frame` (C 12486), `x_menubar_window_to_frame` (C 12405-12443),
`x_top_window_to_frame` in both arms (C 12448-12482 and C 12488-12492) and
`x_next_event_from_any_display` (C 12494-12538).  Rust 21,416 -> 21,698 lines.

**What the probes read.**  Baseline = HEAD's unit (slice 55's commit) with the same arm: A 729 / 69,
B 749 / 70, C 780 / 71.

| arm | baseline | slice 56 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 729 / 69 | **727 / 69** | **-2** | `E0425` 467 -> 465 (three new bare names less five resolved call sites) |
| B `+have_gtk3+use_gtk` | 749 / 70 | **746 / 70** | **-3** | `E0425` 452 -> 448, `E0609` 232 -> 233 |
| C `+have_xinput2` | 780 / 71 | **778 / 71** | **-2** | `E0425` 497 -> 495 |

The five call sites the earlier slices owed `x_top_window_to_frame` (unit 4215, 4370, 5065, 5093,
5135) resolve in every arm; the three new bare names are `ConnectionNumber`, `XPending` and
`XNextEvent` (arm A and C, from `x_next_event_from_any_display`), and arm B instead gains
`xg_event_is_for_menubar` and the `xany` field row from the toolkit/GTK arm's
`x_menubar_window_to_frame` - its `gtk_widget_get_toplevel` resolves through the GTK mirror.
`x_any_window_to_frame`'s macro body and both `x_top_window_to_frame` arms are the checklist's
pinned rows.

**Two intermediate readings, both fixed.**  The first reading was **732 / 70**: the five `E0308`s of
the pre-existing call sites plus one `while_true` warning (Decision 69's third and fourth items).
After spelling C's conversion and the loop, the reading is the table's.

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as `x_next_event_from_any_display`'s first
statement moves `E0308` 13 -> 14 anchored exactly at that line (`_root.rs:21665`) and the total to
728 / 69; removing it returns arm A to 727 / 69 - which is what shows the `#else` arm's text this
slice adds is compiled by arm A rather than skipped.

**What `build-tu` reads.**  The same 30 warnings as HEAD's unit, with the two runs' warning lines
identical once the location columns are stripped (`while true` and its `while_true` lint are gone -
Decision 69's third item).

**What `--check` reads.**  **78 -> 75**: the three names retire and no token row arrives - the
`loop` spelling keeps `true` in the body's comment, so the body-token gate is satisfied without a
notes row.  The line-count row moves to 21,698 against C's 33,125 (34.5% drift).

**Not claimed.**  The frozen configuration compiles none of this file: no event was fetched and no
window was matched.  The toolkit/GTK arm's `XtWindow` / `lw_window_is_in_menubar` /
`xg_event_is_for_menubar` sites and the whole `#else`-arm multiplexer's Xlib three are carried as
text; which arm each probe compiles is stated above.

### Slice 57's readings (C 12542-12683), as landed and measured (2026-10-06)

**What it lands.**  `x_handle_pending_selection_requests_1` (C 12542-12554),
`x_handle_pending_selection_requests` (C 12558-12570), `x_push_selection_request` (C 12572-12581),
`x_detect_pending_selection_requests` (C 12583-12587), `x_clear_dnd_action` (C 12589-12593),
`x_dnd_delete_action_list` (C 12595-12617), `x_dnd_lose_ownership` (C 12619-12630) and
`x_dnd_process_quit` (C 12632-12683).  Rust 21,698 -> 21,897 lines.

**What the probes read.**  Baseline = HEAD's unit (slice 56's commit) with the same arm: A 727 / 69,
B 746 / 70, C 778 / 71.

| arm | baseline | slice 57 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 727 / 69 | **727 / 69** | **0** | `E0425` 465 -> 465 (+2 `XDeleteProperty`, -2 owed call sites) |
| B `+have_gtk3+use_gtk` | 746 / 70 | **746 / 70** | **0** | `E0425` 448 -> 448, `E0609` 233 -> 233 |
| C `+have_xinput2` | 778 / 71 | **778 / 71** | **0** | `E0425` 495 -> 495 |

The two movements the zero is made of are Decision 70's fourth item: `x_dnd_delete_action_list`'s two
`XDeleteProperty` sites are new bare names, and `x_push_selection_request`'s C 917 call site plus
`x_handle_pending_selection_requests`'s C 958 site (unit 1396, owed since slice 1) now resolve.  The
first reading was **730 / 72** (two `E0507`, one `E0381`, three `unused_mut` - Decision 70's fifth
item); no `E0308` or uncoded row is added by the final text, and `build-tu` prints the same 30
warnings with the two runs' warning lines identical once the location columns are stripped.

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as `x_handle_pending_selection_requests_1`'s
first statement moves `E0308` 13 -> 14 anchored exactly at that line (`_root.rs:21733`); removing it
returns arm A to 727 / 69.

**What `--check` reads.**  **75 -> 67**: the eight functions retire as missing items and the two
token rows the slice brings (`true` for C's `while (true)`, `xfree` for the mirror's
`xfree_wrapper`) are closed by `docs/ports/macro-body-notes.tsv` rows.  The line-count row moves to
21,897 against C's 33,125 (33.9% drift).

**Not claimed.**  The frozen configuration compiles none of this file: no selection request was
queued, no property was deleted and no drag was torn down.

### Slice 58's readings (C 12284-12342), as landed and measured (2026-10-06)

**What it lands.**  `x_tooltip_window_to_frame` (C 12284-12342).  Rust 21,897 -> 21,980 lines.

**What the probes read.**  Baseline = HEAD's unit (slice 57's commit) with the same arm: A 727 / 69,
B 746 / 70, C 778 / 71.

| arm | baseline | slice 58 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 727 / 69 | **725 / 69** | **-2** | `E0425` 465 -> 463 (two owed call sites resolve) |
| B `+have_gtk3+use_gtk` | 746 / 70 | **747 / 70** | **+1** | `E0425` 448 -> 447, `E0609` 233 -> 235 |
| C `+have_xinput2` | 778 / 71 | **776 / 71** | **-2** | `E0425` 495 -> 493 |

Arm B's `+2 E0609` are the arm's two `ttip_window` reads and its `+1 E0425` is
`gdk_x11_window_get_xid` (Decision 71's second item); the same two call sites resolve in every arm.
The one intermediate reading was **725 / 70** and **748 / 70** with an `E0695` (the first writing's
labelled block, Decision 71's first item); the final text carries neither that error nor the
`unused label` warning, and `build-tu` prints the same 30 warnings with the two runs' warning lines
identical once the location columns are stripped.

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as the function's first statement moves
`E0308` 13 -> 14 anchored exactly at that line (`_root.rs:21933`); removing it returns arm A to
725 / 69.

**What `--check` reads.**  **67 -> 66**: `x_tooltip_window_to_frame` retires as a missing item and no
token row arrives.  The line-count row moves to 21,980 against C's 33,125 (33.6% drift).

**Not claimed.**  The frozen configuration compiles none of this file: no tooltip window was
matched and no pointer was marked unrelated.  The `USE_GTK` arm is live only in arm B; the GTK2
arm's `GDK_WINDOW_XID` is unmeasured text.

### Slice 59's readings (C 13979-14127), as landed and measured (2026-10-06)

**What it lands.**  `x_detect_focus_change` (C 13979-14031), `x_mouse_leave` (C 14038-14083),
`XTframe_rehighlight` (C 14094-14098) and `x_frame_rehighlight` (C 14100-14127), plus the eight
measured X11 event constants in `include/xlib.rs`.  Rust 21,980 -> 22,168 lines.

**What the probes read.**  Baseline = HEAD's unit (slice 58's commit) with the same arm: A 725 / 69,
B 747 / 70, C 776 / 71.

| arm | baseline | slice 59 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 725 / 69 | **731 / 69** | **+6** | `E0425` 463 -> 459, `E0609` 228 -> 238 |
| B `+have_gtk3+use_gtk` | 747 / 70 | **753 / 70** | **+6** | `E0425` 447 -> 443, `E0609` 235 -> 245 |
| C `+have_xinput2` | 776 / 71 | **785 / 71** | **+9** | `E0425` 493 -> 490, `E0609` 249 -> 261 |

The `-4 E0425` in A and B are Decision 72's three items: `XTframe_rehighlight`'s hook assignment,
`x_frame_rehighlight`'s call site (owed since slices 55 and 56) and the two `FocusIn`/`FocusOut`
tests slice 55 left bare.  Arm C resolves the same four and adds `xi_handle_focus_change` (this
file's own definition, still ahead of the lane) for -3.  The `E0609` additions are the twelve and
ten Decision 72's third item counts (arm C's two extra rows are `client_pointer_device`).
`xi_device_from_id` resolves: the unit defines it at 6036.

**Two intermediate readings, both fixed.**  The first writing read **745 / 70** in arm A (the
missing `use` drew `E0408` and the constants resolved as bindings), and after the imports the
reading was **733 / 70** with one `unused_mut`; the table's numbers are the third.

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as `x_detect_focus_change`'s first statement
moves `E0308` 13 -> 14 anchored exactly at that line (`_root.rs:22017`); removing it returns arm A to
731 / 69.

**What `--check` reads.**  **66 -> 62**: the four functions retire as missing items and the two
token rows the slice brings (`x_detect_focus_change`'s `switch,case,break`, `x_frame_rehighlight`'s
`0`) are closed by `docs/ports/macro-body-notes.tsv` rows.  The line-count row moves to 22,168
against C's 33,125 (33.1% drift).

**What the mirror change cost.**  P-87 ①'s legs read `checked 83 unit(s), 0 failure(s)` and
`checking 83 unit(s), 0 unit failure(s)` (exit 0, 1m25s); `check-mirror-dupes` stays 16/16/0 and
`build-tu` prints the same 30 warnings with the two runs' warning lines identical once the location
columns are stripped.

**Not claimed.**  The frozen configuration compiles none of this file: no focus event was
dispatched, no mouse face was cleared and no frame was highlighted.

### Slice 60's readings (C 14134-14303), as landed and measured (2026-10-06)

**What it lands.**  `x_find_modifier_meanings` (C 14134-14303), plus the five modifier masks and
nine keysyms in `include/xlib.rs`.  Rust 22,168 -> 22,372 lines.

**What the probes read.**  Baseline = HEAD's unit (slice 59's commit) with the same arm: A 731 / 69,
B 753 / 70, C 785 / 71.

| arm | baseline | slice 60 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 731 / 69 | **734 / 69** | **+3** | `E0425` 459 -> 462 |
| B `+have_gtk3+use_gtk` | 753 / 70 | **756 / 70** | **+3** | `E0425` 443 -> 446 |
| C `+have_xinput2` | 785 / 71 | **788 / 71** | **+3** | `E0425` 490 -> 493 |

Every arm's `+3` is four new bare Xlib names (`XDisplayKeycodes`, `XGetKeyboardMapping`,
`XGetModifierMapping`, `XFreeModifiermap`) less one resolved call site
(`x_find_modifier_meanings`'s own, unit 20409, owed since slice 52); the constants resolve, so the
keysym `match` and the mask comparisons add no `E0425`.  No warning is added and no `E0308`/uncoded
row arrives.

**Two intermediate readings, both fixed.**  The first writing read **743 / 69** with eight `E0408`s
(the match's constants had no `use`, Decision 73's first item) and, after that, six `E0308`s plus
three `E0277`s from the unmixed widths (its second item); the table's numbers are the third.

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as the function's first statement moves
`E0308` 13 -> 14 anchored exactly at that line (`_root.rs:22207`); removing it returns arm A to
734 / 69.

**What `--check` reads.**  **62 -> 61**: the function retires as a missing item and its one token row
(`switch,case,break,continue`) is closed by a `docs/ports/macro-body-notes.tsv` row.  The line-count
row moves to 22,372 against C's 33,125 (32.5% drift).

**What the mirror change cost.**  P-87 ①'s legs read `checked 83 unit(s), 0 failure(s)` and
`checking 83 unit(s), 0 unit failure(s)` (exit 0, 1m19s); `check-mirror-dupes` stays 16/16/0 and
`build-tu` prints the same 30 warnings with the two runs' warning lines identical once the location
columns are stripped.

**Not claimed.**  The frozen configuration compiles none of this file: no keyboard mapping was
queried and no modifier bit was set.  The `HAVE_XKB` region is unmeasured text.

### Slice 61's readings (C 14308-14380), as landed and measured (2026-10-06)

**What it lands.**  `x_x_to_emacs_modifiers` (C 14308-14335), `x_emacs_to_x_modifiers`
(C 14337-14366) and `get_keysym_name` (C 14370-14380).  Rust 22,372 -> 22,484 lines.

**What the probes read.**  Baseline = HEAD's unit (slice 60's commit) with the same arm: A 734 / 69,
B 756 / 70, C 788 / 71.

| arm | baseline | slice 61 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 734 / 69 | **735 / 69** | **+1** | `E0425` 462 -> 463 (`XKeysymToString`) |
| B `+have_gtk3+use_gtk` | 756 / 70 | **757 / 70** | **+1** | `E0425` 446 -> 447 |
| C `+have_xinput2` | 788 / 71 | **789 / 71** | **+1** | `E0425` 493 -> 494 |

One new bare Xlib name and no other movement: the three functions' own call sites are all in the
unlanded event readers, and the masks, the `Vx_*_keysym` definitions (slice 54), `Fget`,
`FIXNUMP`/`XFIXNUM` and `INT_MAX` (`lib/include/limits.in.rs` 43) all resolve.  No warning is added.
The first writing read **745 / 69**: five `E0277`s and five `E0308`s from the `state & mod_*` tests
(Decision 74's first item), all fixed.  One more defect came from a *machine* reader rather than the
probes: the three doc comments cited `xterm.h` lines that do not exist for these prototypes, and
`check-citations` flagged the drift (`xterm.h 1808` and `1809` are the two converters' prototypes and
`get_keysym_name` is `lisp.h 5436`); the same pass moved an `EMACS_INT` token out of a line that
carries a citation, because the gate reads that token as part of it.

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as `x_x_to_emacs_modifiers`'s first statement
moves `E0308` 13 -> 14 anchored exactly at that line (`_root.rs:22405`); removing it returns arm A
to 735 / 69.

**What `--check` reads.**  **61 -> 58**: the three functions retire as missing items and no token row
arrives.  The line-count row moves to 22,484 against C's 33,125 (32.1% drift).

**Not claimed.**  The frozen configuration compiles none of this file: no modifier bit was
translated and no keysym was named.

### Slice 62's readings (C 14382-14536), as landed and measured (2026-10-07)

**What it lands.**  `x_compute_root_window_offset` (C 14388-14396), `x_translate_coordinates`
(C 14403-14437), `x_translate_coordinates_to_root` (C 14443-14474),
`x_handle_translate_coordinates` (C 14480-14499) and the three XI2 wrappers
(`xi_compute_root_window_offset`, `_enter`, `_pinch`: C 14505-14530, the last inside C 14521's
nested `#ifdef HAVE_XINPUT2_4`).  Rust 22,484 -> 22,674 lines.  No mirror changed, so P-87's two
legs do not apply.

**What the probes read.**  Baseline = HEAD's unit (slice 61's commit) with the same arm: A 735 / 69,
B 757 / 70, C 789 / 71, X 789 / 71 (X = C's arm plus `have_xinput2_4`, the xwidget lane's shape).

| arm | baseline | slice 62 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 735 / 69 | **734 / 69** | **-1** | 3 `x_translate_coordinates` call sites resolve, 2 `XTranslateCoordinates` sites arrive |
| B `+have_gtk3+use_gtk` | 757 / 70 | **756 / 70** | **-1** | same composition, `E0425` 447 -> 446 |
| C `+have_xinput2` | 789 / 71 | **790 / 71** | **+1** | the same -3/+2, plus one `XIDeviceEvent` and one `XIEnterEvent` site |
| X `+have_xinput2_4` | 789 / 71 | **791 / 71** | **+2** | as C, plus one `XIGesturePinchEvent` site (the pinch wrapper is live only here) |

The `E0425` head set was diffed by message and count, not eyeballed: A and B add exactly the two
`XTranslateCoordinates` sites and lose exactly the three `x_translate_coordinates` sites; C and X
additionally add the wrappers' parameter types.  No `E0308`, no uncoded row and no new warning in
any arm, and `build-tu` prints the same 30 warnings as HEAD's unit (the two sides' warning lines
are identical as sets; the frozen object is exit 0 both times).

**One intermediate reading, fixed.**  The first writing read **738 / 71**: four `E0381`s and two
`unused_mut` warnings at the two old DnD call sites (Decision 75's fourth item).  Both are
Deviation 12's shape and the fixed reading is the table's.

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as `x_compute_root_window_offset`'s first
statement moves `E0308` 13 -> 14 anchored exactly at that line (`_root.rs:22516:18`, "expected
`i32`, found `bool`"); removing it returns arm A to 734 / 69.

**What `--check` reads.**  **58 -> 54**: the four functions retire as missing items, no token row
survives (the first writing's four `frame_x_output` rows are the notes rows' work), and the three
XI2 wrappers were never in the reference's inventory.  The line-count row moves to 22,674 against
C's 33,125 (31.6% drift).

**One machine reader caught one real defect, and one text reader caught another.**  `check-citations`
flagged the two doc comments that cited `termhooks.h 1128-1129` and `1130-1131` - this revision's
`x_translate_coordinates`/`x_translate_coordinates_to_root` prototypes are `xterm.h` 1833 and
1834-1835 (and `x_handle_translate_coordinates` is 1836-1837, which a third comment had already
written as `xterm.h 1981`); all three were grepped before the fix and the gate now reads 0 drift,
0 range.  The same pass found the slice's own "dominant 150-site" prose stale - the spelling has
161 sites after this slice - and it is written as measured now.

**Not claimed.**  The frozen configuration compiles none of this file: no root-window offset was
cached and no coordinate was translated.  The three XI2 wrappers are unmeasured text in every arm
this lane runs (arm A/B have no `have_xinput2`; the C/X arms have the feature but no mirror row for
the three record types).

### Slice 63's readings (C 14536-14634), as landed and measured (2026-10-07)

**What it lands.**  `x_query_pointer_1` (C 14536-14607, C's `static`) and `x_query_pointer`
(C 14610-14628, exported - xterm.h 1839-1840).  Rust 22,674 -> 22,815 lines.  No mirror changed, so
P-87's two legs do not apply.

**What the probes read.**  Baseline = HEAD's unit (slice 62's commit, `860433a1`) with the same arm:
A 734 / 69, B 756 / 70, C 790 / 71, X 791 / 71.

| arm | baseline | slice 63 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 734 / 69 | **735 / 69** | **+1** | `E0425` 462 -> 463: the one live `XQueryPointer` site (C 14602-14605) |
| B `+have_gtk3+use_gtk` | 756 / 70 | **757 / 70** | **+1** | same composition |
| C `+have_xinput2` | 790 / 71 | **804 / 71** | **+14** | 4 `E0609` (`client_pointer_device`) + 4 `lrint` + 2 `XQueryPointer` + `XIQueryPointer` + `XIModifierState` + `XIButtonState` + `XIGroupState` |
| X `+have_xinput2_4` | 791 / 71 | **805 / 71** | **+14** | as C; the `+1` that separates X's baseline and head from C's is slice 62's pinch wrapper |

Every arm's delta is a name the arm reaches and no arm loses anything: `x_query_pointer_1`
resolves the call the second function makes, and the XI2 arm's names are dead text in A and B by
construction.  The head sets were diffed by message and count, not eyeballed.  No warning class
moves in any arm (`E0308` stays 13 in A/B and 13 in C/X, `E0381` stays 1, `E0614` 17) and `build-tu`
prints the same 30 warnings as HEAD's unit (exit 0 both sides, the two runs' warning sets equal).

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as `x_query_pointer_1`'s first statement moves
`E0308` 13 -> 14 anchored exactly at that line (`_root.rs:22713:18`, the message reads expected i32, found bool),
and only that head moves; removing it returns arm A to 735 / 69.

**What `--check` reads.**  **54 -> 52**: both functions retire as missing items and no token row
arrives (the `#endif`-continuation copy adds no token the reader asks for).  The line-count row moves
to 22,815 against C's 33,125 (31.1% drift).

**Not claimed.**  The frozen configuration compiles none of this file: no pointer was queried and no
X server was contacted - the whole body is text for an X11 target, and its `have_xinput2` half is
text for a configuration this lane's arms can only count errors in, never run.

### Slice 64's readings (C 14630-14719), as landed and measured (2026-10-07)

**What it lands.**  C's two comments above the function (C 14630-14665) and
`x_construct_mouse_click` (C 14667-14719, C's `static`).  Rust 22,815 -> 22,914 lines.  No mirror
changed, so P-87's two legs do not apply.

**What the probes read.**  Baseline = HEAD's unit (slice 63's commit, `591cafa5`) with the same arm:
A 735 / 69, B 757 / 70, C 804 / 71, X 805 / 71.

| arm | baseline | slice 64 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 735 / 69 | **739 / 69** | **+4** | `E0425` 463 -> 467: `ButtonRelease` x2 (C 14680/14699), `XButtonEvent`, `Button1` |
| B `+have_gtk3+use_gtk` | 757 / 70 | **761 / 70** | **+4** | the same four |
| C `+have_xinput2` | 804 / 71 | **808 / 71** | **+4** | the same four |
| X `+have_xinput2_4` | 805 / 71 | **809 / 71** | **+4** | the same four |

All four sites are the same in every arm (none of them is inside a `have_xinput2` arm), and no arm
loses anything: `x_x_to_emacs_modifiers` (slice 61), `x_translate_coordinates` (slice 62),
`Fcdr_safe`/`Fassq`/`make_fixnum`/`Fsymbol_value`/`NILP`/`BASE_EQ`, the `Qwheel_*`/`Qnil` accessors,
`MOUSE_CLICK_EVENT`/`NO_EVENT`/`WHEEL_EVENT`/`HORIZ_WHEEL_EVENT`/`up_modifier`/`down_modifier`,
`FRAME_DISPLAY_INFO!`/`FRAME_X_WINDOW!` and the file's own `input_event_bits!` all resolve.  The
head sets were diffed by message and count; `E0308`/`E0381`/`E0614` do not move (`E0308` 13 in
A/B and C/X, `E0381` 1, `E0614` 17), and `build-tu` prints the same 30 warnings as HEAD's unit
(exit 0 both sides, warning sets equal).

**One intermediate reading, fixed.**  The first writing read **739 / 70**: the one new warning is
Decision 77's third item (C's parentheses around the `?:` initialiser, `_root.rs:22897`).  Every
number in the table above is from after the fix.

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as the function's first statement moves
`E0308` 13 -> 14 anchored exactly at that line (`_root.rs:22881:18`, the message reads expected i32,
found bool); removing it returns arm A to 739 / 69.

**What `--check` reads.**  **52 -> 51**: the function retires as a missing item and no token row
arrives.  The line-count row moves to 22,914 against C's 33,125 (30.8% drift).

**Not claimed.**  The frozen configuration compiles none of this file: no mouse event was
constructed and no wheel-button table was looked up.

### Slice 65's readings (C 14738-14857), as landed and measured (2026-10-07)

**What it lands.**  `x_note_mouse_movement` (C 14738-14783) and `x_get_window_below`
(C 14786-14857), both C's `static`, plus the `mouse_moved` arm in this unit's `frame_bits!`.
Rust 22,914 -> 23,083 lines.  No mirror changed, so P-87's two legs do not apply.

**What the probes read.**  Baseline = HEAD's unit (slice 64's commit, `7ac10f4d`) with the same arm:
A 739 / 69, B 761 / 70, C 808 / 71, X 809 / 71.

| arm | baseline | slice 65 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 739 / 69 | **743 / 69** | **+4** | five `E0425` sites arrive (`XQueryTree` twice, `XWindowAttributes`, `XGetWindowAttributes`, `XMotionEvent`) and one retires (`x_get_window_below`'s)
| B `+have_gtk3+use_gtk` | 761 / 70 | **765 / 70** | **+4** | the same six movements |
| C `+have_xinput2` | 808 / 71 | **812 / 71** | **+4** | the same six movements |
| X `+have_xinput2_4` | 809 / 71 | **813 / 71** | **+4** | the same six movements |

The head sets were diffed by message and count: the only *resolution* is `x_get_window_below` (the
earlier caller that named it as an unresolved callee), and the five new sites are exactly the names
Decision 78's fourth item lists.  `remember_mouse_glyph`/`note_mouse_highlight`'s two use-site declarations, `FRAME_X_OUTPUT!`/`FRAME_X_WINDOW!`/`FRAME_DISPLAY_INFO!`, the
`last_mouse_*` fields, `IsViewable`/`None` and the `frame_bits!` arm all resolve.  No warning class
moves in any arm (`E0308` 13 in A/B and C/X, `E0381` 1, `E0614` 17, the arm's own `unused_parens`
count unchanged) and `build-tu` prints the same 30 warnings as HEAD's unit (exit 0 both sides).

**Two intermediate readings, both fixed.**  The first writing read **746 / 69**: one `E0695`
(`unlabeled break inside of a labeled block` - the loop needed its own label, Decision 78's fifth
item) and, after that, **743 + 2 `E0308`** from the two `XFree (children)` calls (a `Window *` into
`void *`).  The table's numbers are the third reading.

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as `x_note_mouse_movement`'s first statement
moves `E0308` 13 -> 14 anchored exactly at that line (`_root.rs:22949:18`); removing it returns arm A
to 743 / 69.

**What `--check` reads.**  **51 -> 49**: both functions retire as missing items and the one token row
they bring (`x_get_window_below`'s `continue`) is closed by a `docs/ports/macro-body-notes.tsv` row
citing C 14822/14826/14831 and the `E0695` measurement.  The line-count row moves to 23,083 against
C's 33,125 (30.3% drift).

**Not claimed.**  The frozen configuration compiles none of this file: no mouse position was noted
and no child window was searched - `XQueryTree`/`XGetWindowAttributes` are unresolved names in every
arm this lane runs.

### Slice 66's readings (C 14859-14978), as landed and measured (2026-10-07)

**What it lands.**  `x_fast_mouse_position` (C 14861-14978, C's `static`) plus two scope changes: the
`remember_mouse_glyph`/`note_mouse_highlight` declarations move from `x_note_mouse_movement`'s body
to this unit's block above `XFree`'s, and `include/frame.rs`'s `FRAME_MOUSE_UPDATE!` casts its first
argument.  Rust 23,083 -> 23,233 lines.  The mirror change is why P-87's two legs were run.

**What the probes read.**  Baseline = HEAD's unit (slice 65's commit, `27dac5bd`) with the same arm
and with the *new* mirror, so the mirror change is not inside the delta: A 743 / 69, B 765 / 70,
C 812 / 71, X 813 / 71 (each re-measured after the mirror edit; identical to the numbers the slice 65
entry quotes, because with the callee unresolved rustc does not check the argument).

| arm | baseline | slice 66 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 743 / 69 | **744 / 69** | **+1** | `x_horizontal_scroll_bar_report_motion` and `x_scroll_bar_report_motion` arrive, `note_mouse_highlight`'s old macro call site resolves |
| B `+have_gtk3+use_gtk` | 765 / 70 | **766 / 70** | **+1** | the same three movements |
| C `+have_xinput2` | 812 / 71 | **813 / 71** | **+1** | the same three movements |
| X `+have_xinput2_4` | 813 / 71 | **814 / 71** | **+1** | the same three movements |

All four arms move by the same three heads, by message and count; nothing else arrives and nothing
else retires.  `gui_mouse_grabbed`'s use-site declaration, `XFRAME`/`FRAME_X_P`/`FRAME_TOOLTIP_P`/
`SELECTED_FRAME`/`FRAME_PIXEL_WIDTH`/`FRAME_PIXEL_HEIGHT`, `FOR_EACH_FRAME!`, `EQ`, `track_mouse`,
`Vx_use_fast_mouse_position`, `Qreally_fast`/`Qdropping`/`Qdrag_source`, the `scroll_bar` record and
`scroll_bar_nowhere` all resolve.  No warning class moves in any arm, and `build-tu` prints the same
30 warnings as HEAD's unit (exit 0 both sides, warning sets equal).

**One intermediate reading and one real defect, both fixed.**  The first writing read **746 / 69**
with three `E0425` - the two reporters plus `remember_mouse_glyph`, whose slice-65 declaration sits
inside the other function's body.  Moving the pair to the unit's scope took that one away and
exposed the mirror's `FRAME_MOUSE_UPDATE!` argument (`E0308` 13 -> 14); casting it as the
neighbouring comparison does returns the class to 13.  The table above is the third reading.

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as the function's first statement moves
`E0308` 13 -> 14 anchored exactly at that line (`_root.rs:23126:18`); removing it returns arm A to
744 / 69.

**P-87 ①'s two legs** (the mirror changed).  `tools/check-all-units.sh` -> `checked 83 unit(s),
0 failure(s)` and `checking 83 unit(s), 0 unit failure(s)`; `check-mirror-dupes` -> **16 duplicate
declaration(s), 16 ratchet row(s), 0 failure(s)** - the new declarations are unit-to-unit with names
`docs/ports/include-frame.md` already lists, not mirror rows.

**What `--check` reads.**  **49 -> 48**: the function retires as a missing item and no token row
arrives.  The line-count row moves to 23,233 against C's 33,125 (29.9% drift).

**Not claimed.**  The frozen configuration compiles none of this file: no mouse position was read and
no scroll bar was reported - the two reporters are unlanded, so their call sites are counted, not
resolved.

### Slice 67's readings (C 14980-15296), as landed and measured (2026-10-07)

**What it lands.**  C's comment block above the function (C 14980-14999, carried) and
`XTmouse_position` (C 15000-15296, C's `static`).  Rust 23,233 -> 23,583 lines.  No mirror changed,
so P-87's two legs do not apply.

**What the probes read.**  Baseline = HEAD's unit (slice 66's commit, `a5f51290`) with the same arm:
A 744 / 69, B 766 / 70, C 813 / 71, X 814 / 71, T 813 / 68 (T = A + `use_x_toolkit`, run for the
first time in this slice because the toolkit arm is the one this code path gates on).

| arm | baseline | slice 67 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 744 / 69 | **750 / 69** | **+6** | seven sites arrive (`XTranslateCoordinates` x3, `DefaultRootWindow`, `x_window_to_scroll_bar`, one more `x_scroll_bar_report_motion` and `x_horizontal_scroll_bar_report_motion`) and `XTmouse_position`'s old value row retires |
| B `+have_gtk3+use_gtk` | 766 / 70 | **772 / 70** | **+6** | the same seven and the same retirement |
| C `+have_xinput2` | 813 / 71 | **819 / 71** | **+6** | the same |
| X `+have_xinput2_4` | 814 / 71 | **820 / 71** | **+6** | the same |
| T `+use_x_toolkit` | 813 / 68 | **820 / 68** | **+7** | the same seven plus `XtWindow` (C 15192, inside the toolkit block T is the only arm to compile) |

The five arms' head sets were diffed by message and count; the only *resolution* in every arm is
`XTmouse_position` itself (the `mouse_position_hook` assignment `x_create_terminal` has been carrying
since slice 50), and the arrivals are the bare Xlib/Xt names and the two unlanded scroll-bar
reporters.  `x_window_to_frame`/`x_any_window_to_frame` (both shapes), `x_tooltip_window_to_frame`,
`x_get_window_below`, `x_query_pointer`, `x_display_set_last_user_time`, `gui_mouse_grabbed`,
`remember_mouse_glyph`, `FOR_EACH_FRAME!`, `WINDOW_FRAME!`/`XWINDOW`/`XFRAME`, `SELECTED_FRAME`,
`FRAME_TOOLTIP_P`/`FRAME_PARENT_FRAME`/`FRAME_X_P`/`FRAME_X_DISPLAY!`/`FRAME_X_WINDOW!`,
`x_catch_errors`/`x_had_errors_p`/`x_uncatch_errors_after_check`, `Qdropping`/`Qdrag_source`,
`last_*` fields and `scroll_bar_part` all resolve.  No warning class moves in any arm
(`E0308` 13 in A/B and 13 in C/X, `15` in T on both sides, `E0381`/`E0614` unchanged) and `build-tu`
prints the same 30 warnings as HEAD's unit (exit 0 both sides, warning sets equal).

**Two intermediate readings, both fixed.**  The first writing read **751 / 69**: `E0423` for the
`x_any_window_to_frame` call, one new `E0308` at the macro's expansion (the `win as c_int` C leaves
implicit) and the three `XTranslateCoordinates`/`DefaultRootWindow`/`x_window_to_scroll_bar` rows;
the second read **751 / 69** with the shape split in place but the four `win as c_int` conversions
missing; the table above is the third.

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as the function's first statement moves
`E0308` 13 -> 14 anchored exactly at that line (`_root.rs:23287:18`); removing it returns arm A to
750 / 69.

**What `--check` reads.**  **48 -> 47**: the function retires as a missing item and no token row
arrives (C's `while (true)` is `loop`, which the notes row for `x_next_event_from_any_display`
already covers as a class).  The line-count row moves to 23,583 against C's 33,125 (28.8% drift).

**Not claimed.**  The frozen configuration compiles none of this file: no mouse position was read and
no window tree was walked - the three `XTranslateCoordinates` sites, `DefaultRootWindow`, `XtWindow`
and the two reporters are counted, not resolved.

### Slice 68's readings (C 15306-15401), as landed and measured (2026-10-07)

**What it lands.**  `xi_select_scroll_bar_events` (C 15312-15333, `HAVE_XINPUT2`, `MAYBE_UNUSED
static`), `x_window_to_scroll_bar` (C 15342-15377, C's `static`) and `x_window_to_menu_bar`
(C 15385-15399, `USE_LUCID`).  Rust 23,583 -> 23,706 lines.  No mirror changed, so P-87's two legs do
not apply.

**What the probes read.**  Baseline = HEAD's unit (slice 67's commit, `02beb5a2`) with the same arm:
A 750 / 69, B 772 / 70, C 819 / 71, X 820 / 71, T 820 / 68.

| arm | baseline | slice 68 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 750 / 69 | **749 / 69** | **-1** | the `x_window_to_scroll_bar` call site slice 67 left in `XTmouse_position` resolves; neither new function is live in this arm |
| B `+have_gtk3+use_gtk` | 772 / 70 | **771 / 70** | **-1** | the same |
| C `+have_xinput2` | 819 / 71 | **833 / 71** | **+14** | fifteen XI2 sites arrive (`XISetMask` x5, `XIEventMask`, `XI_LASTEVENT`, `XIAllMasterDevices`, the five `XI_*` events, `XIMaskLen`, `XISelectEvents`) less the same resolution |
| X `+have_xinput2_4` | 820 / 71 | **834 / 71** | **+14** | the same |
| T `+use_x_toolkit` | 820 / 68 | **819 / 68** | **-1** | the same resolution; `use_lucid` is not in this arm's set either |

The five head sets were diffed by message and count.  This is the lane's first slice whose A/B/T arms
go **down**: the new definitions resolve a caller (`XTmouse_position`, C 15258), and the only arms
that pay are the ones whose gates C's own `#if`s hide the new text from - the XI2 function is live in
C and X only.  No warning class moves in any arm (`E0308` 13 in A/B and C/X, 15 in T; `E0381` 1,
`E0614` 17) and `build-tu` prints the same 30 warnings as HEAD's unit (exit 0 both sides).

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as `x_window_to_scroll_bar`'s first statement
moves `E0308` 13 -> 14 anchored exactly at that line (`_root.rs:23656:18`); removing it returns arm A
to 749 / 69.

**What `--check` reads.**  **47 -> 46**: `x_window_to_scroll_bar` retires as a missing item and the
one token row it brings (`continue`) is closed by a `docs/ports/macro-body-notes.tsv` row.  The two
`#if`-hidden functions add no row - the frozen table leaves `HAVE_XINPUT2` and `USE_LUCID` `undef`, so
this reader never asks for them, which is exactly why their definitions are the only evidence this
round can have.  The line-count row moves to 23,706 against C's 33,125 (28.4% drift).

**Not claimed.**  The frozen configuration compiles none of this file: no scroll bar was looked up
and no menu bar was matched.  `x_window_to_menu_bar`'s body is compiled by no arm this lane runs
(`use_lucid`), and its two names (`Widget`, `xlwmenu_window_p`) are unresolved text.

### Slice 69's readings (C 15403-15426), as landed and measured (2026-10-07)

**What it lands.**  C 15404-15406's banner (carried), the note standing for C 15410-15411's prototype,
`window_being_scrolled` (C 15416) and `x_get_last_toolkit_time` (C 15418-15426).  Rust 23,706 ->
23,736 lines.  No mirror changed, so P-87's two legs do not apply.  **Two arms are new**: U
(`have_x_windows` + `use_toolkit_scroll_bars`) and UT (U + `use_x_toolkit`).

**What the probes read.**  Baseline = HEAD's unit (slice 68's commit, `2f6554bf`) with the same arm.

| arm | baseline | slice 69 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 749 / 69 | **749 / 69** | **0** | the whole region is dead text here (`use_toolkit_scroll_bars` is not set) - the gate holds, measured |
| C `+have_xinput2` | 833 / 71 | **833 / 71** | **0** | the same |
| U `+use_toolkit_scroll_bars` | 756 / 69 | **756 / 69** | **0** | both new declarations are live and neither brings a site: `last_user_time` is the mirror's field and `x_get_last_toolkit_time` has no caller yet |
| UT `U +use_x_toolkit` | 826 / 68 | **827 / 68** | **+1** | `XtLastTimestampProcessed` (C 15422), the bare Xt name the U arm does not compile |

U and UT are quoted here for the first time; A and C are quoted to show the gate actually holds (a
new region that leaked into those arms would move them).  The four head sets were diffed by message
and count; no warning class moves in any arm (`E0308` 13 in A/U and 13 in C, 15 in UT; `E0381` 1,
`E0614` 17) and `build-tu` prints the same 30 warnings as HEAD's unit (exit 0 both sides, warning
sets equal).

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as the first statement of the `not
(use_x_toolkit)` `return` block moves `E0308` 13 -> 14 anchored exactly at that line
(`_root.rs:23749:20`, **in arm U** - the arm the new text lives in); removing it returns arm U to
756 / 69.

**What `--check` reads.**  **46 -> 46**: neither the function nor the object is in this reader's
inventory (the frozen table leaves `USE_TOOLKIT_SCROLL_BARS` `undef`), so the only moving row is the
line count: 23,736 against C's 33,125 (28.3% drift).  That is the honest reading for a region whose
coverage evidence has to be the arms, not the checklist.

**Not claimed.**  The frozen configuration compiles none of this file: no toolkit time was fetched.
The region's 300+ remaining lines are still unlanded, and the two new arms measure only what is
written so far.

### Slice 70's readings (C 15428-15536), as landed and measured (2026-10-07)

**What it lands.**  The four `#ifndef USE_GTK` file-scope objects (C 15432-15444), `xt_action_hook`
(C 15451-15491) and `xt_horizontal_action_hook` (C 15494-15536), plus the three Xt typedefs in this
file's Xt block and the three casts they exposed (C 9013/9018/9025).  Rust 23,736 -> 23,896 lines.  No
mirror changed, so P-87's two legs do not apply.

**What the probes read.**  Baseline = HEAD's unit (slice 69's commit, `b0755fe0`) with the same arm.

| arm | baseline | slice 70 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 749 / 69 | **749 / 69** | **0** | the whole region is dead text here and the aliases' gate is `use_x_toolkit` - the gate holds, measured |
| C `+have_xinput2` | 833 / 71 | **833 / 71** | **0** | the same |
| U `+use_toolkit_scroll_bars` | 756 / 69 | **776 / 69** | **+20** | twenty-two sites in (`XtActionHookId` x2, `Boolean` x2, `Widget` x2, `XtPointer` x2, `Cardinal` x2, `scrollbarWidgetClass` x2, `toolkit_scroll_bar_interaction` x2, `XtIsSubclass` x2, `x_send_scroll_bar_event` x2, `set_vertical_scroll_bar`, `set_horizontal_scroll_bar`) and two `E0308`s (`String` is the prelude's in this arm) less the two `xaw3d_*` sites the new statics resolve |
| UT `U +use_x_toolkit` | 827 / 68 | **824 / 68** | **-3** | ten sites in (the same list minus the three aliased types) and **sixteen** out: `XtPointer` x6 and `Boolean` x5 resolve through the new typedefs, `xaw3d_arrow_scroll`/`xaw3d_pick_top` x2 resolve against the new statics, and `E0606` returns to its baseline `1` once C 9013/9018/9025's casts are written |

Four arms' head sets were diffed by message and count.  No warning class moves in any arm (`E0308` 13 in A, 15 in C, 15 in U and 15 in UT; `E0606` 1 in every arm; `E0381` 1, `E0614` 17 with
UT's 15 the one arm whose `E0614` differs - the same 15 its baseline carries) and `build-tu` prints the same 30 warnings as HEAD's unit (exit 0 both sides, warning sets
equal).

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as `xt_action_hook`'s first statement moves
`E0308` **15 -> 16 in arm UT** anchored exactly at that line (`_root.rs:23817:18`); removing it returns
UT to 824 / 68.  (The injection is read in UT because that is the arm the region's real text lives in -
arm U's two `E0308`s are the configuration artifact Decision 83's fourth item describes.)

**What `--check` reads.**  **46 -> 46**: nothing in this region is in the frozen inventory, so only the
line-count row moves - 23,896 against C's 33,125 (27.9% drift).

**Not claimed.**  The frozen configuration compiles none of this file: no action hook was installed and
no scroll-bar interaction ended.  The hooks' `XtAppAddActionHook` registration is still unlanded (`x_scroll_bar_create`),
so nothing calls them yet.

### Slice 71's readings (C 15539-15696), as landed and measured (2026-10-07)

**What it lands.**  `x_protect_window_for_callback` (C 15542-15553),
`x_unprotect_window_for_callback` (C 15555-15572), `x_send_scroll_bar_event` (C 15579-15625),
`x_scroll_bar_to_input_event` (C 15631-15661),
`x_horizontal_scroll_bar_to_input_event` (C 15666-15696).  Rust 23,896 -> 24,102 lines.  No mirror changed, so P-87's two legs do not apply.

**What the probes read.**  Baseline = HEAD's unit (slice 70's commit, `351ee66e`) with the same arm.

| arm | baseline | slice 71 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 749 / 69 | **749 / 69** | **0** | the region is dead text here - the gate holds, measured |
| U `+use_toolkit_scroll_bars` | 776 / 69 | **795 / 69** | **+19** | nineteen `E0609` (`xclient` x7, `n_protected_windows` x7, `protected_windows` x4, `protected_windows_max` x1) plus `XClientMessageEvent` and `XSendEvent`, less the two `x_send_scroll_bar_event` call sites slice 70 left |
| UT `U +use_x_toolkit` | 824 / 68 | **844 / 68** | **+20** | U's nineteen plus `x_activate_timeout_atimer` (C 15616, the `use_x_toolkit` block only UT compiles) |

The three head sets were diffed by message and count; the only **resolution** in each arm is
`x_send_scroll_bar_event` (the two calls the action hooks make, landed in slice 70).  No warning class
moves in any arm - the macro definition adds none, which the counts above show (`E0308` 15 in U and
UT, `E0381` 1, `E0614` 17, `E0606` 1) - and `build-tu` prints the same 30 warnings as HEAD's unit
(exit 0 both sides, warning sets equal).

**Two intermediate readings, both fixed.**  The first writing read **795 / 70**: C 15597's parentheses
around the `?:` initialiser draw `unused_parens` (Decision 84's third item).  The second carried a local
`static_assert!` macro that `check-mirror-dupes` named as a duplicate of the mirror's (Decision 84's
first item); deleting it left arm U's numbers unchanged, so the table's readings are the third.

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as `x_protect_window_for_callback`'s first
statement moves `E0308` 15 -> 16 anchored exactly at that line in **arm U** (`_root.rs:23938:18`);
removing it returns U to 795 / 69.

**What `--check` reads.**  **46 -> 46**: none of the five functions is in the frozen inventory (the
region's gate is `undef` there), so only the line-count row moves - 24,102 against C's 33,125 (27.2%
drift).

**Not claimed.**  The frozen configuration compiles none of this file: no scroll-bar message was sent
and no input event was built.  `x_activate_timeout_atimer` is still unlanded, so C 15616's call site
is counted, not resolved.

### Slice 72's readings (C 15699-15786), as landed and measured (2026-10-07)

**What it lands.**  C 15699's `#ifdef USE_MOTIF` arm: `XM_SB_MAX` (C 15703) and the three Motif
declarations (all **UNVERIFIED** placeholders, Decision 85's first item) plus `xm_scroll_callback`
(C 15709-15786).  Rust 24,102 -> 24,263 lines.  No mirror changed, so P-87's two legs do not apply.
**One arm is new**: UM (`have_x_windows` + `use_toolkit_scroll_bars` + `use_x_toolkit` + `use_motif`).

**What the probes read.**  Baseline = HEAD's unit (slice 71's commit, `07389653`) with the same arm.

| arm | baseline | slice 72 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 749 / 69 | **749 / 69** | **0** | the whole region is dead text here - the gate holds |
| UT `U +use_x_toolkit` | 844 / 68 | **844 / 68** | **0** | `use_motif` is not in this arm, so the new text is dead too |
| UM `UT +use_motif` | 835 / 68 | **835 / 68** | **0** | the block is live and **every name in it resolves**: the eight placeholders, the record, `XmNsliderSize ()`, `XtVaGetValues`, `min`, the `f32` arithmetic, `window_being_scrolled` and `x_send_scroll_bar_event` all land without adding a site |

All three head-set diffs are **empty** and no warning class moves in any arm (`E0308` 13 in A, 15 in
UT and UM... the per-arm summaries are in the logs; `E0381` 1, `E0614` 17, `E0606` 1 in A/UT and 3 in
UM, unchanged); `build-tu` prints the same 30 warnings as HEAD's unit (exit 0 both sides, warning sets
equal).  A zero here is the *point*: the Motif block is self-contained, which is what a placeholder
set must be to be honest - it says nothing about the values, only that nothing else is unnamed.

**Two intermediate readings, both fixed.**  The first wrote `std::ptr::null_mut ()` in the variadic
call (`E0283`; the `::<c_void>` is the existing call site's spelling), the second left C 15752's
`slider_size` uninitialised (`E0381`; `= 0` is PORTING.md 1.5's value for C's own convention).

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as `xm_scroll_callback`'s first statement moves
`E0308` **15 -> 16 in arm UM** anchored exactly at that line (`_root.rs:24204:18`); removing it returns
UM to 835 / 68.

**What `--check` reads.**  **46 -> 46**: the Motif callback is outside the frozen inventory, so only
the line-count row moves - 24,263 against C's 33,125 (26.8% drift).

**Not claimed.**  The frozen configuration compiles none of this file, and this arm's zero says
nothing about Motif's *values*: the eight reasons, the record's offsets and the resource string are
placeholders until a Motif header exists (Owed item 0).  No callback was invoked and no scroll bar
was dragged.

### Slice 73's readings (C 15788-15899), as landed and measured (2026-10-07)

**What it lands.**  C 15788's `#elif defined USE_GTK` arm: `xg_scroll_callback` (C 15793-15877) and
`xg_end_scroll_callback` (C 15881-15899), with the GTK declaration set Decision 86 describes.  It also
*widens five gates* slice 71 had written too narrowly.  Rust 24,263 -> 24,463 lines.  No mirror
changed, so P-87's two legs do not apply.  **One arm is new**: UG (`have_x_windows` +
`use_toolkit_scroll_bars` + `use_gtk`).

**What the probes read.**  Baseline = HEAD's unit (slice 72's commit, `20b0ff89`) with the same arm
(A, UT and UM re-measured on this revision so the gate change is inside both columns).

| arm | baseline | slice 73 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 749 / 69 | **749 / 69** | **0** | the region is dead text here; the widened gates do not reach it |
| UT `U +use_x_toolkit` | 844 / 68 | **844 / 68** | **0** | `use_gtk` is not set, so the gate change is a no-op - measured, not assumed |
| UM `UT +use_motif` | 835 / 68 | **835 / 68** | **0** | the same |
| UG `U +use_gtk` | 771 / 71 | **792 / 71** | **+21** | slice 71's five functions become live here (the gate defect): `xclient` x7, `n_protected_windows` x7, `protected_windows` x4, `protected_windows_max`, `XClientMessageEvent`, `XSendEvent`; the GTK callbacks' two `x_send_scroll_bar_event` calls then resolve, and **this slice's own text adds nothing** |

Four arms' head sets were diffed by message and count.  The reading to notice is UG's composition:
**my new text is a zero** - every GTK name it uses resolves (the two opaque records, the three calls,
`gdouble`, the five labels, `GDK_CURRENT_TIME`, `XG_FRAME_DATA`, `xg_ignore_gtk_scrollbar`,
`GtkWidget`/`GdkEventButton`, `G_OBJECT!`, `gtk_get_current_event_time`, `WINDOWP`, `Qnil`,
`x_send_scroll_bar_event`) - so the +21 is slice 71's text becoming live, which is exactly what the
widened gates are for.  No warning class moves in any arm after the six declaration fixes
(`E0308` 13 in A, 15 in UT/UM, 17 in UG on both sides; `E0381` 1, `E0614` 17, `E0606` 1/3) and
`build-tu` prints the same 30 warnings as HEAD's unit (exit 0 both sides).

**Three intermediate readings, all fixed.**  (1) The first UG reading had the two
`x_send_scroll_bar_event` rows and one `E0308` - the gate defect plus `time as Time`.  (2) After
widening the gates UG read **77** warnings: the six declaration mistakes Decision 86's fourth item
lists.  (3) With those fixed UG reads 792 / 71 and the table above is the third.

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as `xg_scroll_callback`'s first statement moves
`E0308` **17 -> 18 in arm UG** anchored exactly at that line (`_root.rs:24364:18`); removing it
returns UG to 792 / 71.

**What `--check` reads.**  **46 -> 46**: the GTK callbacks are outside the frozen inventory, so only
the line-count row moves - 24,463 against C's 33,125 (26.1% drift).

**Not claimed.**  The frozen configuration compiles none of this file, and the GTK arm's values are
placeholders until a GTK tree exists (the same Owed 0 situation slice 72 recorded for Motif).  No
scroll bar was dragged and no adjustment was queried.

### Slice 74's readings (C 15902-16034), as landed and measured (2026-10-07)

**What it lands.**  C 15902's `#else /* not USE_GTK and not USE_MOTIF */` arm: `xaw_jump_callback`
(C 15909-15965) and `xaw_scroll_callback` (C 15976-16032).  It also widens the Xt declarations' gates
from `use_motif` to `use_x_toolkit`, fixes the four pre-existing defects that exposed, and fixes the
mirror's `eabs!` for C's float use.  Rust 24,463 -> 24,649 lines.  The mirror change is why P-87's two
legs were run.

**What the probes read.**  Baseline = HEAD's unit (slice 73's commit, `78a394fb`) with the same arm
(A, U, UT, UM and UG all measured on this revision).

| arm | baseline | slice 74 | delta | of which |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 749 / 69 | **749 / 69** | **0** | the region is dead text here |
| U `+use_toolkit_scroll_bars` | 795 / 69 | **795 / 69** | **0** | `use_x_toolkit` is not set, so neither the Xaw arm nor the widened gates are live |
| **UT** `U +use_x_toolkit` | 844 / 68 | **830 / 68** | **-14** | the Xaw callbacks are live and add **0**; the twelve `cannot find type Widget` sites resolve, the `eabs!` float call compiles, and `XtConfigureWidget`'s two argument conversions land |
| UM `UT +use_motif` | 835 / 68 | **829 / 68** | **-6** | C 28639/28640's rewritten pair (2 `E0609` + 2 `E0606` gone) and the same two conversions |
| UG `U +use_gtk` | 792 / 71 | **792 / 71** | **0** | `use_gtk` is set, so the Xaw arm is dead and the widened gates change nothing measurable |

Five arms' head sets were diffed by message and count.  The reading to notice is that **this slice's
own text is a zero in every arm** - every name it uses resolves (`XtNshown`/`XtNwidth`/`XtVaGetValues`/
`Widget`/`XtPointer`/`Dimension`, the `scroll_bar_*` labels, `eabs!`, `min`/`max`, `xaw3d_arrow_scroll`
and `x_send_scroll_bar_event`) - while the *widened gates and the fixes they forced* move UT by -14 and
UM by -6.  No warning class moves in any arm (`E0308` 13 in A/U, 13 in UT, 13 in UM, 17 in UG, with
`E0381` 1, `E0614` 17, `E0606` 1/1/1/3 unchanged) after three `mut`s were dropped from my own first
writing, and `build-tu` prints the same 30 warnings as HEAD's unit (exit 0 both sides).

**Three intermediate readings, all fixed.**  (1) The first reading had four `Widget`-shaped errors of
my own making (`E0606`/`E0609`/`E0605` at C 28639/28640, from declaring `Widget` as `*mut c_void`);
(2) the second had the `eabs!` float `E0308` and `XtConfigureWidget`'s argument `E0308`s;
(3) the third had three `unused_mut` warnings in my `xaw_jump_callback`.  The table is the fourth.

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as `xaw_jump_callback`'s first statement moves
`E0308` **13 -> 14 in arm UT** anchored exactly at that line (`_root.rs:24517:18`); removing it returns
UT to 830 / 68.

**P-87 ①'s two legs** (the mirror changed).  `tools/check-all-units.sh` -> `checked 83 unit(s),
0 failure(s)` and `checking 83 unit(s), 0 unit failure(s)`; `check-mirror-dupes` -> 16/16/0;
`extract-abi --check emacs/src/lisp.h femacs/src/include/lisp.rs` went from one row
(`macro eabs does not mention 0`) to none, closed by the new `docs/ports/macro-body-notes.tsv` row.

**What `--check` reads.**  **46 -> 46**: the Xaw callbacks are outside the frozen inventory, so only the
line-count row moves - 24,649 against C's 33,125 (25.6% drift).

**Not claimed.**  The frozen configuration compiles none of this file.  The Xaw arm's `XtNshown`/
`XtNwidth` strings and every libXt/Motif/GTK value are placeholders until a machine with those SDKs
measures them (Owed 0), and no scroll bar was dragged.

### Slice 75's readings (C 16036-16065), as landed and measured (2026-10-07)

Probe = `cargo run -q -p xtask -- probe src/xterm.rs /tmp/s75/<tag>.o <cfg flags> --crate-type=lib
--emit=metadata -C panic=abort`, all into `CARGO_TARGET_DIR=/tmp/s75/cargo`; the arms are
A `--cfg 'feature="have_x_windows"'`, U = A + `use_toolkit_scroll_bars`, UT = U + `use_x_toolkit`,
UM = UT + `use_motif`, UG = U + `use_gtk`.

| arm | HEAD's unit (slice 74) | this slice's text | delta | distinct head lines |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 749 / 69 | **749 / 69** | **0** | 229, identical |
| U `+use_toolkit_scroll_bars` | 795 / 69 | **795 / 69** | **0** | 243, identical |
| UT `U +use_x_toolkit` | 830 / 68 | **830 / 68** | **0** | 268, identical |
| UM `UT +use_motif` | 829 / 68 | **829 / 68** | **0** | 268, identical |
| UG `U +use_gtk` | 792 / 71 | **792 / 71** | **0** | 241, identical |

Each pair was diffed by message *and* count (`error[E…]` and `warning` lines, locations stripped):
the diff is **empty in all five arms**, so this slice's text adds no error and no warning anywhere and
retires none.  In arm UG the two logs are byte-identical (`diff -q` prints nothing).  The class counts
do not move either: A `E0425` 477 / `E0609` 238 / `E0308` 13, U 498 / 261 / 15, UT 542 / 251 / 13,
UM 540 / 252 / 13, UG 17 E0308 sites (16 distinct head lines) with `E0381` 1, `E0614` 17, `E0606` 1.

**Arm reachability** (P-86 ⑦).  `let _: c_int = true;` as `x_create_toolkit_scroll_bar`'s first
statement moves UG's total 792 -> 793 and `E0308` 17 -> 18, anchored at
`build/probes/xterm/_root.rs:24701:18`; removing it returns UG to the 792 / 71 tabled above.  This is
what shows the slice's own text is inside the arm that compiles it - the head sets alone cannot, since
nothing calls the two creators yet.

**Two legs of `build-tu`** (no mirror changed, so P-87 ① does not apply): `cargo run -q -p xtask --
build-tu src/xterm.rs` reads exit 0 and `30 warnings emitted` on **both** sides (HEAD's unit and this
text), and the two warning sets are identical line for line.

**What `--check` reads.**  **46 -> 46**: everything this slice lands is outside the frozen inventory
(`USE_TOOLKIT_SCROLL_BARS` is `undef` there, so the tool never asks for these names); only the
line-count row moves - 24,712 against C's 33,125 (25.4% drift; it read 24,649 / 25.6% before).

**A stale reading of my own, corrected while writing this.**  `/tmp/s75/finUT.log` and
`finUM.log` read 827 / 826 because they were taken without `--crate-type=lib --emit=metadata
-C panic=abort`, which makes the probe read the unit as a binary crate and adds an `E0601`; the same
text re-probed with the full flag set reads 830 / 829, matching slice 74's committed numbers.  The
table above is the re-probed one and the two stale logs were not used.

**Not claimed.**  The frozen configuration compiles none of this file, and nothing calls either
creator yet, so no scroll bar was created and no widget was realized.  Every libXt/Motif/GTK value in
this region is still a placeholder until a machine with those SDKs measures them (Owed 0).

### Slice 76's readings (C 16067-16485), as landed and measured (2026-10-07)

Probe = `cargo run -q -p xtask -- probe src/xterm.rs /tmp/s76/<tag>.o <cfg flags> --crate-type=lib
--emit=metadata -C panic=abort`, all into `CARGO_TARGET_DIR=/tmp/s76/cargo`; A = `have_x_windows`,
U = A + `use_toolkit_scroll_bars`, UT = U + `use_x_toolkit`, UM = UT + `use_motif`, UG = U + `use_gtk`.

| arm | HEAD's unit (slice 75) | this slice's text | delta | new rows |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 749 / 69 | **749 / 69** | **0** | 0 |
| U `+use_toolkit_scroll_bars` | 795 / 69 | **795 / 69** | **0** | 0 |
| **UT** `U +use_x_toolkit` | 830 / 68 | **811 / 68** | **-19** | **0** |
| **UM** `UT +use_motif` | 829 / 68 | **812 / 68** | **-17** | **0** |
| UG `U +use_gtk` | 792 / 71 | **792 / 71** | **0** | 0 |

**The 19 rows UT retires, by name** (diffed as `error[E..]: <message>` heads, identifiers kept):
`XtSetArg` macro x3 (C 30350/30396/30398 - one row per call, the arguments hidden behind them),
`cannot find type \`Arg\`` x2, `cannot find type \`WidgetClass\`` x1, `cannot find value
\`scrollbarWidgetClass\`` x2, `cannot find function \`XtWindow\`` x8 (this unit's own Xt calls - the two widget reads, the three C 12385-12387 `wdesc` comparisons, C 12473's and the two frame-parameter sites), `... \`XtDisplay\`` x1,
`... \`XDefineCursor\`` x2 (C 12132/12135, both in `x_toggle_visible_pointer`).
**UM retires 17 of them**: it does not compile the Xaw arm, so the `scrollbarWidgetClass` pair is
not live there.  **Neither arm gains a row**: this slice's own text resolves completely, and the two
names the newly-defined macro made visible (`XtNiconPixmap`/`XtNiconMask`, C 30396/30398) were
declared in the same round, so they are off the table before it is written.  Every arm's warning set
(by message, counts normalised) is **identical** to its baseline - 68 / 68 / 68 / 69 / 71, no class
moves - after the two fixes Decision 89 ⑤ records.  `build-tu` reads **exit 0 with 30 warnings and
the identical set on both sides** (HEAD's unit and this text).

**Arm reachability** (P-86 7).  `let _: c_int = true;` as `x_create_toolkit_scroll_bar`'s first
statement moves UT 811 -> 812 and UM 812 -> 813, `E0308` 13 -> 14 in both, anchored at
`build/probes/xterm/_root.rs:25035:18`; removing it returns both to the table above.  This is what
shows the new text is inside the arms that compile it - the totals alone cannot, since they move
*down*.

**What `--check` reads.**  **46 -> 46**: everything this slice lands is outside the frozen inventory
(`USE_TOOLKIT_SCROLL_BARS` is `undef` there), so only the line-count row moves - **25,520** against
C's 33,125 (23.0% drift, was 24,712 / 25.4%).  The remaining rows are the ones Owed names.

**Not claimed.**  The frozen configuration compiles none of this file.  No Motif, Xaw or libXt value
is measured: every resource string, the four `Xm*` scalars and the two `XtN…` features are
placeholders until a machine with those SDKs measures them (Owed 0).  Nothing created a widget,
realized one, or dragged a scroll bar.

### Slice 77's readings (C 16488-16701), as landed and measured (2026-10-07)

Probe = `cargo run -q -p xtask -- probe src/xterm.rs /tmp/s77/<tag>.o <cfg flags> --crate-type=lib
--emit=metadata -C panic=abort`, all into `CARGO_TARGET_DIR=/tmp/s77/cargo`; A = `have_x_windows`,
U = A + `use_toolkit_scroll_bars`, UT = U + `use_x_toolkit`, UM = UT + `use_motif`, UG = U + `use_gtk`.

| arm | HEAD's unit (slice 76 text, **widened mirror**) | this slice's text | delta | new rows |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 749 / 69 | **749 / 69** | **0** | 0 |
| U `+use_toolkit_scroll_bars` | 795 / 69 | **795 / 69** | **0** | 0 |
| UT `U +use_x_toolkit` | 811 / 68 | **811 / 68** | **0** | **0** |
| UM `UT +use_motif` | 812 / 68 | **812 / 68** | **0** | **0** |
| UG `U +use_gtk` | 792 / 71 | **792 / 71** | **0** | **0** |

Both columns were re-measured on this revision, and the head-set diff
(`error[E..]: <message>` rows, identifiers kept, plus the warning messages) is **empty in both
directions in every arm**: this slice's text adds no row and retires none - the four functions are
new, and their callers (`x_scroll_bar_create`, C 16715) are still ahead.  A and U also read what
slice 76 committed, which is the *widened mirror*'s own reading for them: they do not move.

**The mirror's widening, measured on the unit it matters to.**  HEAD's text with the widened mirror
reads UT **811/68** and UM **812/68** - the same numbers the committed slice-76 entry has for the
old mirror - so the generic `min`/`max` changes nothing in this unit's existing call sites.  The
83-unit sweep is the leg that says it changes nothing anywhere (Decision 90 2).

**Arm reachability** (P-86 7), two injections, because two arms compile different text.
`let _: c_int = true;` as the Xt pair's vertical `x_set_toolkit_scroll_bar_thumb`'s first statement
moves UT 811 -> 812 and UM 812 -> 813, `E0308` 13 -> 14, anchored at
`build/probes/xterm/_root.rs:25622:18`; the same injection as the GTK arm's twin moves UG
792 -> 793, `E0308` 17 -> 18, anchored at `_root.rs:25597:18`.  Removing both returns all three
arms to the table.

**Two legs of `build-tu`**: `cargo run -q -p xtask -- build-tu src/xterm.rs` reads exit 0 and
`30 warnings emitted` on **both** sides (HEAD's unit and this text, same widened mirror), and the
two warning sets are identical line for line.

**What `--check` reads.**  **46 -> 46**: everything this slice lands is outside the frozen
inventory (`USE_TOOLKIT_SCROLL_BARS` is `undef` there), so only the line-count row moves -
**25,846** against C's 33,125 (22.0% drift, was 25,520 / 23.0%).

**Not claimed.**  The frozen configuration compiles none of this file, no thumb was set on any
scroll bar, and the dead `#if false` blocks are text: nothing here ran.  Every Motif/Xaw/libXt
value stays a placeholder until a machine with those SDKs measures it (Owed 0).

### Slice 78's readings (C 16709-16824), as landed and measured (2026-10-07)

Probe = `cargo run -q -p xtask -- probe src/xterm.rs /tmp/s78/<tag>.o <cfg flags> --crate-type=lib
--emit=metadata -C panic=abort`, all into `CARGO_TARGET_DIR=/tmp/s78/cargo`; A = `have_x_windows`,
U = A + `use_toolkit_scroll_bars`, UT = U + `use_x_toolkit`, UM = UT + `use_motif`, UG = U + `use_gtk`.

| arm | HEAD's unit (slice 77 text, **widened x11_constants**) | this slice's text | delta | new rows |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 749 / 69 | **750 / 69** | **+1** | 2 (`XSetWindowAttributes`, `XCreateWindow`) |
| U `+use_toolkit_scroll_bars` | 795 / 69 | **804 / 69** | **+9** | 10 (arm U's own, named in Decision 91) |
| UT `U +use_x_toolkit` | 811 / 68 | **808 / 68** | **-3** | **0** |
| UM `UT +use_motif` | 812 / 68 | **809 / 68** | **-3** | **0** |
| UG `U +use_gtk` | 792 / 71 | **791 / 71** | **-1** | **0** |

**Retirements and additions, by name** (diffed as `error[E..]: <message>` heads, identifiers kept):

| arm | retired | added |
| --- | --- | --- |
| A | `XLowerWindow` x1 | `cannot find type \`XSetWindowAttributes\`` x1, `... \`XCreateWindow\`` x1 |
| U | `XLowerWindow` x1 | `x_create_horizontal_toolkit_scroll_bar` x1, `x_create_toolkit_scroll_bar` x1, macro `SCROLL_BAR_X_WIDGET` x1, `Widget` x1, `Position` x2, `Dimension` x2, `XtConfigureWidget` x1, `XtMapWidget` x1 |
| UT | `XtMapWidget` x2, `XLowerWindow` x1 | none |
| UM | `XtMapWidget` x2, `XLowerWindow` x1 | none |
| UG | `XLowerWindow` x1 | none |

Every arm's warning set (by message, counts normalised) is **identical to its baseline's**:
69 / 69 / 68 / 68 / 71, no class moves - after the three fixes Decision 91 5 records.  **Two legs of
`build-tu`**: `cargo run -q -p xtask -- build-tu src/xterm.rs` reads exit 0 and `30 warnings
emitted` on **both** sides (HEAD's unit and this text, same widened mirror), with identical warning
sets.

**Arm reachability** (P-86 7).  `let _: c_int = true;` after `block_input ()` in
`x_scroll_bar_create` moves A 750 -> 751 and UG 791 -> 792, `E0308` 13 -> 14 and 17 -> 18, anchored
at `build/probes/xterm/_root.rs:25936:18`; removing it returns both arms to the table.

**What `--check` reads.**  **46 -> 45**: `x_scroll_bar_create` retires as a missing function (the
first function of the scroll-bar region the frozen inventory asked for), and the line-count row
moves to **26,035** against C's 33,125 (21.4% drift, was 25,846 / 22.0%).  The remaining rows are the
ones Owed names.

**Not claimed.**  The frozen configuration compiles none of this file: no scroll bar object was
allocated, no X window was created, no widget was configured or mapped, and arm U's reading is a
*text* reading of a configuration C does not have (the port's feature set can spell it; C's cannot).

### Slice 79's readings (C 16827-16958), as landed and measured (2026-10-07)

Probe = `cargo run -q -p xtask -- probe src/xterm.rs /tmp/s79/<tag>.o <cfg flags> --crate-type=lib
--emit=metadata -C panic=abort`, all into `CARGO_TARGET_DIR=/tmp/s79/cargo`; A = `have_x_windows`,
U = A + `use_toolkit_scroll_bars`, UT = U + `use_x_toolkit`, UM = UT + `use_motif`, UG = U + `use_gtk`.

| arm | HEAD's unit (slice 78 text) | this slice's text | delta | new rows |
| --- | --- | --- | --- | --- |
| A `have_x_windows` | 750 / 69 | **761 / 69** | **+11** | 11 (`XSetForeground` x8, `XFillRectangle` x3) |
| U `+use_toolkit_scroll_bars` | 804 / 69 | **804 / 69** | **0** | 0 |
| UT `U +use_x_toolkit` | 808 / 68 | **808 / 68** | **0** | 0 |
| UM `UT +use_motif` | 809 / 68 | **809 / 68** | **0** | 0 |
| UG `U +use_gtk` | 791 / 71 | **791 / 71** | **0** | 0 |

The four toolkit arms read their slice-78 numbers: C 16827's `#ifndef USE_TOOLKIT_SCROLL_BARS` is the
function's gate, so `x_scroll_bar_set_handle` is textually absent from every arm that sets
`use_toolkit_scroll_bars` (UT re-measured; the other three are the same by construction).  **A's
head-set diff is exactly the eleven bare Xlib names** (`error[E0425]: cannot find function, tuple
struct or tuple variant \`XSetForeground\`` x8 and `... \`XFillRectangle\`` x3) with **0 retired** -
the "Xlib entry points with no mirror row" class Decision 92 4 names - and **no warning moves** (69
in A, its baseline; the sets are identical by message in every arm).

**Two legs of `build-tu`**: `cargo run -q -p xtask -- build-tu src/xterm.rs` reads exit 0 and
`30 warnings emitted` on **both** sides (HEAD's unit and this text), with identical warning sets.

**Arm reachability** (P-86 7).  `let _: c_int = true;` after `block_input ()` in
`x_scroll_bar_set_handle` moves A 761 -> 762 with `E0308` 13 -> 14, anchored at
`build/probes/xterm/_root.rs:26108:18`; removing it returns A to the table.

**What `--check` reads.**  **45 -> 44**: `x_scroll_bar_set_handle` retires as a missing function, and
the line-count row moves to **26,204** against C's 33,125 (20.9% drift, was 26,035 / 21.4%).  No
mirror changed this slice, so P-87's legs do not apply to it.

**Not claimed.**  The frozen configuration compiles none of this file: no handle was drawn, no GC was
set, and the eleven bare names are unresolved *text*, not behaviour.

### Slice 80's readings (C 16960-17116), as landed and measured (2026-10-07)

Probe = `cargo run -q -p xtask -- probe src/xterm.rs /tmp/s80/<tag>.o <cfg flags> --crate-type=lib
--emit=metadata -C panic=abort`, all into `CARGO_TARGET_DIR=/tmp/s80/cargo`; A = `have_x_windows`,
U = A + `use_toolkit_scroll_bars`, UT = U + `use_x_toolkit`, UM = UT + `use_motif`, UG = U + `use_gtk`.

| arm | HEAD's unit (slice 79 text, **widened x11_constants**) | this slice's text | delta |
| --- | --- | --- | --- |
| A `have_x_windows` | 761 / 69 | **763 / 69** | **+2** |
| U `+use_toolkit_scroll_bars` | 804 / 69 | **810 / 69** | **+6** |
| UT `U +use_x_toolkit` | 808 / 68 | **803 / 68** | **-5** |
| UM `UT +use_motif` | 809 / 68 | **804 / 68** | **-5** |
| UG `U +use_gtk` | 791 / 71 | **788 / 71** | **-3** |

**Retirements and additions, by name** (diffed as full `error[E..]: <message>` head lines with
counts, in both directions):

| arm | retired | added |
| --- | --- | --- |
| A | `window_box` x2, `XTset_vertical_scroll_bar` x1 | `x_scroll_bar_redraw` x2, `XConfigureWindow` x1, `XDestroyWindow` x1 (an extra use), `XWindowChanges` x1 |
| U | `window_box` x2, `XTset_vertical_scroll_bar` x1 | macro `SCROLL_BAR_X_WIDGET` x2, `x_set_toolkit_scroll_bar_thumb` x1, `XtConfigureWidget` x1 (extra use), `XtDestroyWidget` x1, `Dimension` x2, `Position` x2 |
| UT | `window_box` x2, `x_scroll_bar_remove` x1, `XtDestroyWidget` x1, `XTset_vertical_scroll_bar` x1 | none |
| UM | `window_box` x2, `x_scroll_bar_remove` x1, `XtDestroyWidget` x1, `XTset_vertical_scroll_bar` x1 | none |
| UG | `window_box` x2, `XTset_vertical_scroll_bar` x1 | none |

Every arm's warning set is **identical to its baseline's** by message (69/69/68/68/71) - the first
reading's four `CW*` rows were the only defect, and the import fixed them without moving a warning.
**Two legs of `build-tu`**: exit 0 and `30 warnings emitted` on **both** sides, identical sets.

**Arm reachability** (P-86 7), one injection per function.  `let _: c_int = true;` after
`x_scroll_bar_remove`'s `block_input ()` moves A 763 -> 764 and UG 788 -> 789 (`E0308` 13 -> 14 and
17 -> 18, `build/probes/xterm/_root.rs:26258:18`); the same injection after
`XTset_vertical_scroll_bar`'s `window_box` call moves UT 803 -> 804 (`_root.rs:26314:18`).  Both
removed, the arms read the table again.

**What `--check` reads.**  **45 -> 43**: `x_scroll_bar_remove` and `XTset_vertical_scroll_bar`
retire as missing functions, and the line-count row moves to **26,416** against C's 33,125 (20.3%
drift, was 26,204 / 20.9%).

**Not claimed.**  The frozen configuration compiles none of this file: no scroll bar was destroyed,
no widget was reconfigured, no window was mapped, and the bare names above are unresolved text.

### Slice 81's readings (C 17119-17246), as landed and measured (2026-10-07)

Probe = `cargo run -q -p xtask -- probe src/xterm.rs /tmp/s81/<tag>.o <cfg flags> --crate-type=lib
--emit=metadata -C panic=abort`, all into `CARGO_TARGET_DIR=/tmp/s81/cargo`; A = `have_x_windows`,
U = A + `use_toolkit_scroll_bars`, UT = U + `use_x_toolkit`, UM = UT + `use_motif`, UG = U + `use_gtk`.

| arm | HEAD's unit (slice 80 text) | this slice's text | delta |
| --- | --- | --- | --- |
| A `have_x_windows` | 763 / 69 | **765 / 69** | **+2** |
| U `+use_toolkit_scroll_bars` | 810 / 69 | **817 / 69** | **+7** |
| UT `U +use_x_toolkit` | 803 / 68 | **803 / 68** | **0** |
| UM `UT +use_motif` | 804 / 68 | **804 / 68** | **0** |
| UG `U +use_gtk` | 788 / 71 | **788 / 71** | **0** |

**Retirements and additions, by name** (diffed as full `error[E..]: <message>` head lines with
counts, both directions):

| arm | retired | added |
| --- | --- | --- |
| A | `XTset_horizontal_scroll_bar` x1 (its hook-table row) | `estimate_mode_line_height` x1, `XConfigureWindow` x1 (extra use), `XWindowChanges` x1 (extra use) |
| U | `XTset_horizontal_scroll_bar` x1 | `estimate_mode_line_height` x1, macro `SCROLL_BAR_X_WIDGET` x1, `x_set_toolkit_horizontal_scroll_bar_thumb` x1, `XtConfigureWidget` x1, `Dimension` x2, `Position` x2 (the toolkit block's extra uses) |
| UT | `XTset_horizontal_scroll_bar` x1 | `estimate_mode_line_height` x1 |
| UM | `XTset_horizontal_scroll_bar` x1 | `estimate_mode_line_height` x1 |
| UG | `XTset_horizontal_scroll_bar` x1 | `estimate_mode_line_height` x1 |

Every arm's warning set is **identical to its baseline's** by message (69/69/68/68/71).  **Two legs
of `build-tu`**: exit 0 and `30 warnings emitted` on **both** sides, identical sets.

**Arm reachability** (P-86 7).  `let _: c_int = true;` after C 17134's
`height = WINDOW_SCROLL_BAR_AREA_HEIGHT (w);` moves A 765 -> 766 and UG 788 -> 789 (`E0308`
13 -> 14 and 17 -> 18), anchored at `build/probes/xterm/_root.rs:26465:18`; removing it returns both
arms to the table.

**What `--check` reads.**  **43 -> 41** as Decision 94 4 says, with the line-count row at **26,586**
against C's 33,125 (19.7% drift, was 26,416 / 20.3%).

**Not claimed.**  The frozen configuration compiles none of this file: no scroll bar was moved, no
area was cleared and no window was configured; `estimate_mode_line_height` and the other bare names
above are unresolved text, not behaviour.

### Slice 82's readings (C 17249-17493), as landed and measured (2026-10-07)

Probe = `cargo run -q -p xtask -- probe src/xterm.rs /tmp/s82/<tag>.o <cfg flags> --crate-type=lib
--emit=metadata -C panic=abort`, all into `CARGO_TARGET_DIR=/tmp/s82/cargo`; A = `have_x_windows`,
U = A + `use_toolkit_scroll_bars`, UT = U + `use_x_toolkit`, UM = UT + `use_motif`, UG = U + `use_gtk`.

| arm | HEAD's unit (slice 81 text, **widened x11_constants**) | this slice's text | delta |
| --- | --- | --- | --- |
| A `have_x_windows` | 765 / 69 | **777 / 69** | **+12** |
| U `+use_toolkit_scroll_bars` | 817 / 69 | **814 / 69** | **-3** |
| UT `U +use_x_toolkit` | 803 / 68 | **800 / 68** | **-3** |
| UM `UT +use_motif` | 804 / 68 | **801 / 68** | **-3** |
| UG `U +use_gtk` | 788 / 71 | **785 / 71** | **-3** |

**Retirements and additions, by name** (diffed as full `error[E..]: <message>` head lines with
counts, both directions):

| arm | retired | added |
| --- | --- | --- |
| A | `XTcondemn_scroll_bars` x1, `XTjudge_scroll_bars` x1, `XTredeem_scroll_bar` x1 (the hook table), `x_scroll_bar_redraw` x2 (slice 80's owed calls) | `XSetForeground` x5, `XEvent.r#type` x1, `XEvent.xexpose` x4, `XEvent.xgraphicsexpose` x4, `XFillRectangle` x1, `XDrawRectangle` x1, `gui_intersect_rectangles` x1 |
| U, UT, UM, UG | the same three hook rows | none |

Every arm's warning set is **identical to its baseline's** by message (69/69/68/68/71) - the first
reading's `WINDOW_HAS_HORIZONTAL_SCROLL_BAR` macro row was the only defect, and calling the mirror's
accessor fixed it without moving a warning.  **Two legs of `build-tu`**: exit 0 and `30 warnings
emitted` on **both** sides, identical sets.

**Arm reachability** (P-86 7).  `let _: c_int = true;` after `x_scroll_bar_redraw`'s
`let gc: GC = ...` (C 17462) moves A 777 -> 778 with `E0308` 13 -> 14, anchored at
`build/probes/xterm/_root.rs:26835:18`; removing it returns A to the table.

**What `--check` reads.**  **41 -> 36** as Decision 95 6 says, with the line-count row at **26,863**
against C's 33,125 (18.9% drift, was 26,586 / 19.7%).

**Not claimed.**  The frozen configuration compiles none of this file: no hook ran, no rectangle was
intersected, no GC was set and nothing was drawn; the bare names above are unresolved text.

### Slice 83's readings (C 17495-17596), as landed and measured (2026-10-07)

Probe = `cargo run -q -p xtask -- probe src/xterm.rs /tmp/s83/<tag>.o <cfg flags> --crate-type=lib
--emit=metadata -C panic=abort`, all into `CARGO_TARGET_DIR=/tmp/s83/cargo`; A = `have_x_windows`,
U = A + `use_toolkit_scroll_bars`, UT = U + `use_x_toolkit`, UM = UT + `use_motif`, UG = U + `use_gtk`.
Both sides were measured in all five arms on this revision (the baseline is HEAD's own text, not the
last slice's numbers).

| arm | HEAD's unit (slice 82 text) | this slice's text | delta |
| --- | --- | --- | --- |
| A `have_x_windows` | 777 / 69 | **789 / 69** | **+12** |
| U `+use_toolkit_scroll_bars` | 814 / 69 | **822 / 69** | **+8** |
| UT `U +use_x_toolkit` | 800 / 68 | **808 / 68** | **+8** |
| UM `UT +use_motif` | 801 / 68 | **809 / 68** | **+8** |
| UG `U +use_gtk` | 785 / 71 | **793 / 71** | **+8** |

**Retirements and additions, by name** (diffed as full `error[E..]: <message>` head lines with
counts, both directions):

| arm | retired | added |
| --- | --- | --- |
| A | none - C 22113/22118/23892/23899's four call sites are still unported | `Button1` x1, `ButtonRelease` x3, `XEvent.r#type` x3, `XEvent.xbutton` x5 |
| U, UT, UM, UG | none, the same reason | `Button1` x1, `ButtonRelease` x1, `XEvent.r#type` x1, `XEvent.xbutton` x5 |

The non-toolkit arm reads three `ButtonRelease` and three `r#type` rows because C 17524's `?:` and the
two gated blocks at C 17552/17583 all name them; the four toolkit arms read one of each, because the
two blocks are not compiled there - the difference is the gate, not a defect.  Every arm's warning
set is **identical to its baseline's** by message (69/69/68/68/71), and the first writing's
`FRAME_DISPLAY_INFO` row is the only defect the first reading named (fixed before the table above;
`E0423` was 1 -> 0 in every arm).

**The `f` measurement.**  `grep -c 'cannot find value \`f\`'` is **0** in all five logs: the mirror's
range macros pass `$f` down and the two innermost bodies (C 1667's text, where the `RIGHT` border is
written as the `LEFT` one) never expand it, so the argument is dropped exactly as C's preprocessor
drops it.  C 17537/17568's `f` therefore needs no binding and adds no row - the reconnoitring note's
prediction of two `E0425` rows is not what the compiler reads, and Decision 96 3 records the
measurement rather than the prediction.

**Two legs of `build-tu`**: exit 0 and `30 warnings emitted` on **both** sides, identical sets.

**Arm reachability** (P-86 7).  `let _: c_int = true;` ahead of the function's first declaration
(C 17508's `let left_range: c_int;`) moves A 789 -> 790 with `E0308` 13 -> 14, anchored at
`build/probes/xterm/_root.rs:26903:18`; removing it returns A to the table.

**What `--check` reads.**  **36 -> 35** as Decision 96 6 says, with the line-count row at **26,986**
against C's 33,125 (18.5% drift, was 26,863 / 18.9%).

**Not claimed.**  The frozen configuration compiles none of this file: no click was handled, no
`input_event` was filled or enqueued, no scroll-bar handle was moved; the bare names and the
placeholder's member rows above are unresolved text.

### Slice 84's readings (C 17598-17632), as landed and measured (2026-10-07)

Probe = `cargo run -q -p xtask -- probe src/xterm.rs /tmp/s84/<tag>.o <cfg flags> --crate-type=lib
--emit=metadata -C panic=abort`, all into `CARGO_TARGET_DIR=/tmp/s84/cargo`; A = `have_x_windows`,
U = A + `use_toolkit_scroll_bars`, UT = U + `use_x_toolkit`, UM = UT + `use_motif`, UG = U + `use_gtk`.
Both sides were measured in all five arms on this revision.

| arm | HEAD's unit (slice 83 text) | this slice's text | delta |
| --- | --- | --- | --- |
| A `have_x_windows` | 789 / 69 | **790 / 69** | **+1** |
| U `+use_toolkit_scroll_bars` | 822 / 69 | **822 / 69** | 0 |
| UT `U +use_x_toolkit` | 808 / 68 | **808 / 68** | 0 |
| UM `UT +use_motif` | 809 / 68 | **809 / 68** | 0 |
| UG `U +use_gtk` | 793 / 71 | **793 / 71** | 0 |

**Retirements and additions, by name** (diffed as full `error[E..]: <message>` head lines with
counts, both directions): arm A retires nothing and adds `cannot find type XMotionEvent` x1; U, UT,
UM and UG have **no diff at all** - neither direction, error or warning.

**Why the toolkit arms are empty.**  C 17598's `#ifndef USE_TOOLKIT_SCROLL_BARS` is the gate: the
function exists only in the arm that does not turn the feature on, so the four arms that do cannot
read one row of it.  The measurement is worth writing down because it is the same shape every gated
slice of this region has: an "unchanged" toolkit column is not evidence of a resolved body, it is
evidence of a body the arm never sees - which is why each such slice is checked in the arm that
compiles it (here A, and its +1 is a row this ledger has already counted since slice 65).

**Two legs of `build-tu`**: exit 0 and `30 warnings emitted` on **both** sides, identical warning
sets.

**Arm reachability** (P-86 7).  `let _: c_int = true;` ahead of C 17609's
`let f: *mut frame = XFRAME (...)` moves A 790 -> 791 with `E0308` 13 -> 14, anchored at
`build/probes/xterm/_root.rs:27023:18`; removing it returns A to the table.

**What `--check` reads.**  **35 -> 34** as Decision 97 4 says, with the line-count row at **27,033**
against C's 33,125 (18.4% drift, was 26,986 / 18.5%).

**Not claimed.**  The frozen configuration compiles none of this file: no motion was noted, no frame
flag was set and no handle was moved; `XMotionEvent` and the field reads through it are unresolved
text.

### Slice 85's readings (C 17634-17769), as landed and measured (2026-10-07)

Probe = `cargo run -q -p xtask -- probe src/xterm.rs /tmp/s85/<tag>.o <cfg flags> --crate-type=lib
--emit=metadata -C panic=abort`, all into `CARGO_TARGET_DIR=/tmp/s85/cargo`; A = `have_x_windows`,
U = A + `use_toolkit_scroll_bars`, UT = U + `use_x_toolkit`, UM = UT + `use_motif`, UG = U + `use_gtk`.
Both sides were measured in all five arms on this revision.

| arm | HEAD's unit (slice 84 text) | this slice's text | delta |
| --- | --- | --- | --- |
| A `have_x_windows` | 790 / 69 | **786 / 71** | **-4** |
| U `+use_toolkit_scroll_bars` | 822 / 69 | **818 / 71** | **-4** |
| UT `U +use_x_toolkit` | 808 / 68 | **804 / 70** | **-4** |
| UM `UT +use_motif` | 809 / 68 | **805 / 70** | **-4** |
| UG `U +use_gtk` | 793 / 71 | **789 / 73** | **-4** |

**Retirements and additions, by name** (diffed as full `error[E..]: <message>` head lines with
counts, both directions): every arm retires `cannot find function x_scroll_bar_report_motion` x2 and
`cannot find function x_horizontal_scroll_bar_report_motion` x2 - C 14881/14884's and C
15034/15036's two call sites each - and **adds nothing**: the two new calls to `x_query_pointer`
(C 17655/17724) resolve, because that name is this unit's own C 14610-14628 definition.

**The two revealed warning rows.**  Each arm's warning set moves by exactly two rows, both in
slice 67's body: `warning: variable does not need to be mutable` at C 14869's `bar` and C 14870's
`dpyinfo`.  They are not this slice's text; they are what the baseline could not show, and the
probe that measures it is in Decision 98 3 (one unresolved name back into the same body: 7 `E0381`
rows / 71 warnings -> 1 / 69).  The six `E0381` rows the same reveal produces are zeroed per
Deviation 12, so they are *not* in the table above - the arms' error deltas are the four retirements
alone.

**Two legs of `build-tu`**: exit 0 and `30 warnings emitted` on **both** sides, identical warning
sets for the frozen configuration (which compiles neither scroll-bar build, so the revealed rows are
probe readings, not build readings).

**Arm reachability** (P-86 7).  `let _: c_int = true;` ahead of C 17645's
`let f: *mut frame = XFRAME (...)` in the vertical function moves A 786 -> 787 with `E0308` 13 -> 14,
anchored at `build/probes/xterm/_root.rs:27071:18`; removing it returns A to the table.

**What `--check` reads.**  **34 -> 32** as Decision 98 4 says, with the line-count row at **27,190**
against C's 33,125 (17.9% drift, was 27,033 / 18.4%).

**Not claimed.**  The frozen configuration compiles none of this file: no pointer was queried, no
motion was reported, no scroll-bar part was written and no frame flag was cleared; the readings
above are compile readings of text this machine cannot build.

### Slice 86's readings (C 17772-17809), as landed and measured (2026-10-07)

Probe = `cargo run -q -p xtask -- probe src/xterm.rs /tmp/s86/<tag>.o <cfg flags> --crate-type=lib
--emit=metadata -C panic=abort`, all into `CARGO_TARGET_DIR=/tmp/s86/cargo`; A = `have_x_windows`,
U = A + `use_toolkit_scroll_bars`, UT = U + `use_x_toolkit`, UM = UT + `use_motif`, UG = U + `use_gtk`.
Both sides were measured in all five arms on this revision.

| arm | HEAD's unit (slice 85 text) | this slice's text | delta |
| --- | --- | --- | --- |
| A `have_x_windows` | 786 / 71 | **788 / 71** | **+2** |
| U `+use_toolkit_scroll_bars` | 818 / 71 | **817 / 71** | **-1** |
| UT `U +use_x_toolkit` | 804 / 70 | **803 / 70** | **-1** |
| UM `UT +use_motif` | 805 / 70 | **804 / 70** | **-1** |
| UG `U +use_gtk` | 789 / 73 | **788 / 73** | **-1** |

**Retirements and additions, by name** (diffed as full `error[E..]: <message>` head lines with
counts, both directions):

| arm | retired | added |
| --- | --- | --- |
| A | `cannot find function x_scroll_bar_clear` x1 (C 11416's call) | `XSetForeground` x3 (C 17784/17787/17806, the counted class) |
| U, UT, UM, UG | the same row | none - the body is not compiled there |

Every arm's warning set is **identical to its baseline's**, row for row: the toolkit arms' empty body
with its unused `f` is free because the unit's own `#![allow (..., unused_variables, ...)]` (line 638)
covers it, and that is measured here rather than assumed - the same probe shape would otherwise be one
warning per toolkit arm.

**Two legs of `build-tu`**: exit 0 and `30 warnings emitted` on **both** sides, identical sets.

**Arm reachability** (P-86 7).  `let _: c_int = true;` after C 17781's `let gc: GC = ...` moves A
788 -> 789 with `E0308` 13 -> 14, anchored at `build/probes/xterm/_root.rs:27229:18`; removing it
returns A to the table.

**What `--check` reads.**  **32 -> 31** as Decision 99 3 says, with the line-count row at **27,251**
against C's 33,125 (17.7% drift, was 27,190 / 17.9%).

**Not claimed.**  The frozen configuration compiles none of this file: no scroll bar was cleared or
redrawn, no foreground or background pixel was set and no condemned list was walked; in the four
toolkit arms the function is one empty body.

### Slice 87's readings (C 17811-17848), as landed and measured (2026-10-07)

Probe = `cargo run -q -p xtask -- probe src/xterm.rs /tmp/s87/<tag>.o <cfg flags> --crate-type=lib
--emit=metadata -C panic=abort`, all into `CARGO_TARGET_DIR=/tmp/s87/cargo`; A = `have_x_windows`,
U = A + `use_toolkit_scroll_bars`, UT = U + `use_x_toolkit`, UM = UT + `use_motif`, UG = U + `use_gtk`,
and **AE = A + `enable_checking`** - the arm this slice's `#ifdef ENABLE_CHECKING` region needs and no
configuration here turns on.  Both sides were measured in all six arms on this revision.

| arm | HEAD's unit (slice 86 text) | this slice's text | delta |
| --- | --- | --- | --- |
| A `have_x_windows` | 788 / 71 | **786 / 71** | **-2** |
| U `+use_toolkit_scroll_bars` | 817 / 71 | **815 / 71** | **-2** |
| UT `U +use_x_toolkit` | 803 / 70 | **801 / 70** | **-2** |
| UM `UT +use_motif` | 804 / 70 | **802 / 70** | **-2** |
| UG `U +use_gtk` | 788 / 73 | **786 / 73** | **-2** |
| **AE** `A +enable_checking` | 820 / 70 | **818 / 70** | **-2** |

**Retirements and additions, by name** (diffed as full `error[E..]: <message>` head lines with
counts, both directions): every arm retires `cannot find value next_noop_dpyinfo` **x2** and adds
nothing.  The two rows are C 31925-31926's uses in text slice 12 landed; the slice's own four lines
(the two statics, the two macro arms) move no error at all.

**Every arm's warning set is identical to its baseline's**, row for row - including AE, where the two
statics and the checking macro body are compiled: the unit's own `#![allow (..., unused_variables,
unused_macros, dead_code, ...)]` is what keeps a not-yet-used object and macro free, the same reading
slice 86 recorded for its empty toolkit body.

**The checking arm's macro body, measured.**  A temporary reader appended after the definition
(`unsafe fn zz_probe_store_keysym () { STORE_KEYSYM_FOR_DEBUG! (0 as c_int); }`) leaves **A and AE
both at 0 error and 0 warning delta** (A 786, AE 818) - the non-checking arm's `let _ = 0;` and the
checking arm's three-statement body both type-check.  A deliberate `let _: c_int = true;` inside the
checking arm's body then moves **AE 818 -> 819** with `E0308` 45 -> 46, anchored at the macro's own
line (`build/probes/xterm/_root.rs:27291`), so the arm compiles the *expansion*.  Placing the reader
*before* the definition instead is one `cannot find macro STORE_KEYSYM_FOR_DEBUG` row (A 787) - the
textual-order reading above.  Both temporary edits are removed; the file measured 27,302 lines.

**Two legs of `build-tu`**: exit 0 and `30 warnings emitted` on **both** sides, identical sets (the
frozen configuration compiles neither the checking arm nor a reader of the macro).

**What `--check` reads.**  **31 -> 28** as Decision 100 4 says, with the line-count row at **27,302**
against C's 33,125 (17.6% drift, was 27,251 / 17.7%).

**Not claimed.**  The frozen configuration compiles none of this file: no keysym was stored, and
`STORE_KEYSYM_FOR_DEBUG` still has no call site in the port (C's four are in unported text), so the
macro's *value* is a shape that one temporary reader verified compiles - not a behaviour that ran.

### Slice 88's readings (C 17850-17921), as landed and measured (2026-10-07)

Probe = `cargo run -q -p xtask -- probe src/xterm.rs /tmp/s88/<tag>.o <cfg flags> --crate-type=lib
--emit=metadata -C panic=abort`, all into `CARGO_TARGET_DIR=/tmp/s88/cargo`; A = `have_x_windows`,
U = A + `use_toolkit_scroll_bars`, UT = U + `use_x_toolkit`, UM = UT + `use_motif`, UG = U + `use_gtk`,
AE = A + `enable_checking`, and this slice's three: **AI** = A + `have_x_i18n`, **AIU** = AI +
`use_gtk`, **AIUX** = AIU + `have_xinput2`.  Both sides were measured on this revision (the six
default arms' baselines are the ones slice 87 closed with, which is the same HEAD text).

| arm | HEAD's unit (slice 87 text) | this slice's text | delta |
| --- | --- | --- | --- |
| A `have_x_windows` | 786 / 71 | **786 / 71** | 0 |
| U `+use_toolkit_scroll_bars` | 815 / 71 | **815 / 71** | 0 |
| UT `U +use_x_toolkit` | 801 / 70 | **801 / 70** | 0 |
| UM `UT +use_motif` | 802 / 70 | **802 / 70** | 0 |
| UG `U +use_gtk` | 786 / 73 | **786 / 73** | 0 |
| AE `A +enable_checking` | 818 / 70 | **818 / 70** | 0 |
| **AI** `A +have_x_i18n` | 792 / 71 | **794 / 72** | **+2** |
| **AIU** `AI +use_gtk` | 812 / 73 | **822 / 73** | **+10** |
| **AIUX** `AIU +have_xinput2` | 910 / 75 | **929 / 75** | **+19** |

**The six default arms' zeros are the gate**: `#ifdef HAVE_X_I18N` wraps the whole definition, so
those arms do not compile one line of it - the same reading slice 84 recorded from the other side.

**Additions, by name** (diffed as full `error[E..]: <message>` head lines with counts; nothing
retires in any arm):

| arm | added |
| --- | --- |
| AI | `XFilterEvent` x1, `XEvent.xclient` x1 |
| AIU | the above plus `popup_activated` x1 (a second use of an existing bare name), `xg_filter_key` x1, `XNoOp` x1, `KeyPress` x1, `KeyRelease` x1, `XEvent.r#type` x2, `x_display_info.prefer_native_input` x1 |
| AIUX | AIU's set plus `XIDeviceEvent` x1, `GenericEvent` x1, `XI_KeyPress` x1, `XI_KeyRelease` x1, `XEvent.xgeneric` x3, `XEvent.xcookie` x1, `XEvent.r#type` x1 |

Every one is a class this ledger already counts (Decision 15's bare X11/XInput2 names, the
`[u8; 192]` placeholder's owed members, the `E0425` type class) except `prefer_native_input`, which is
Decision 101 4's owed mirror member.

**Warning sets**: identical to the baselines in every arm but **AI**, which gains one
`warning: unreachable statement` at `build/probes/xterm/_root.rs:27417:3` - C 17918's `return 0;`
behind the non-GTK arm's unconditional `return XFilterEvent (...)`.

**Two legs of `build-tu`**: exit 0 and `30 warnings emitted` on **both** sides, identical sets (the
frozen configuration compiles none of this function).

**Arm reachability** (P-86 7).  `let _: c_int = true;` after C 17859's `let mut f1: *mut frame;`
moves **AIU 822 -> 823** with `E0308` 17 -> 18, anchored at `build/probes/xterm/_root.rs:27342:18`;
removing it returns AIU to the table.

**What `--check` reads.**  **stays 28** as Decision 101 6 says, with the line-count row at **27,407**
against C's 33,125 (17.3% drift, was 27,302 / 17.6%).

**Not claimed.**  The frozen configuration compiles none of this file: no event was filtered, no
input method was consulted, no GDK key was filtered and no XNoOp was flushed; the readings above are
compile readings of text this machine cannot build.

### Slice 89's readings (C 17923-18002), as landed and measured (2026-10-07)

Probe = `cargo run -q -p xtask -- probe src/xterm.rs /tmp/s89/<tag>.o <cfg flags> --crate-type=lib
--emit=metadata -C panic=abort`, all into `CARGO_TARGET_DIR=/tmp/s89/cargo`; A = `have_x_windows`,
U = A + `use_toolkit_scroll_bars`, UT = U + `use_x_toolkit`, UM = UT + `use_motif`, UG = U + `use_gtk`,
**UGX = UG + `have_xinput2`** (new here), AE = A + `enable_checking`, AI = A + `have_x_i18n`,
AIU = AI + `use_gtk`, AIUX = AIU + `have_xinput2`.  Both sides were measured on this revision.

| arm | HEAD's unit (slice 88 text) | this slice's text | delta |
| --- | --- | --- | --- |
| A `have_x_windows` | 786 / 71 | **786 / 71** | 0 |
| U `+use_toolkit_scroll_bars` | 815 / 71 | **815 / 71** | 0 |
| UT `U +use_x_toolkit` | 801 / 70 | **801 / 70** | 0 |
| UM `UT +use_motif` | 802 / 70 | **802 / 70** | 0 |
| **UG** `U +use_gtk` | 786 / 73 | **796 / 73** | **+10** |
| **UGX** `UG +have_xinput2` | 884 / 75 | **899 / 75** | **+15** |
| AE `A +enable_checking` | 818 / 70 | **818 / 70** | 0 |
| AI `A +have_x_i18n` | 794 / 72 | **794 / 72** | 0 |
| **AIU** `AI +use_gtk` | 822 / 73 | **831 / 73** | **+9** |
| **AIUX** `AIU +have_xinput2` | 929 / 75 | **938 / 75** | **+9** |

**Additions, by name** (diffed as full `error[E..]: <message>` head lines with counts; nothing
retires in any arm):

| arm | added |
| --- | --- |
| UG | `handle_one_xevent` x1, `x_dispatch_event` x1, `xg_filter_key` x1, `GdkFilterReturn` x1, `GdkXEvent` x1, `GDK_FILTER_CONTINUE` x1, `GDK_FILTER_REMOVE` x2, `KeyPress` x1, `x_display_info.prefer_native_input` x1 |
| UGX | UG's set plus `XIDeviceEvent` x1, `GenericEvent` x2, `XI_KeyPress` x1, `XI_KeyRelease` x1 (and `GDK_FILTER_REMOVE` x2 as above) |
| AIU, AIUX | `handle_one_xevent` x1, `x_dispatch_event` x1, `GdkFilterReturn` x1, `GdkXEvent` x1, `GDK_FILTER_CONTINUE` x1, `GDK_FILTER_REMOVE` x2, `KeyPress` x2, `KeyRelease` x2 |

The `have_x_i18n` arms do not read the `#elif` half (so no `prefer_native_input` row there), and UGX
is what reads that half's own `#ifdef HAVE_XINPUT2` disjunct - the reason it exists.

**Warnings**: identical to the baselines in **all ten arms** - the function adds one unused callback
with C's own parameter names and no dead code rustc names.

**Two legs of `build-tu`**: exit 0 and `30 warnings emitted` on **both** sides, identical sets.

**Arm reachability** (P-86 7).  `let _: c_int = true;` after C 17930's `let xev: *mut XEvent = ...`
moves **UG 796 -> 797** with `E0308` 17 -> 18, anchored at `build/probes/xterm/_root.rs:27447:18`;
removing it returns UG to the table.

**What `--check` reads.**  **stays 28** as Decision 102 5 says, with the line-count row at **27,511**
against C's 33,125 (16.9% drift, was 27,407 / 17.3%).

**Not claimed.**  The frozen configuration compiles none of this file: no GDK filter ran, no
`XEvent` was dispatched, no input was blocked and no keysym was handled; the readings above are
compile readings of text this machine cannot build.

### Slice 90's readings (C 18006-18056), as landed and measured (2026-10-07)

Probe = `cargo run -q -p xtask -- probe src/xterm.rs /tmp/s90/<tag>.o <cfg flags> --crate-type=lib
--emit=metadata -C panic=abort`, all into `CARGO_TARGET_DIR=/tmp/s90/cargo`; A = `have_x_windows`,
U = A + `use_toolkit_scroll_bars`, UT = U + `use_x_toolkit`, UM = UT + `use_motif`, UG = U + `use_gtk`,
UGX = UG + `have_xinput2`, AE = A + `enable_checking`, AI = A + `have_x_i18n`, AIU = AI + `use_gtk`,
AIUX = AIU + `have_xinput2`, **AX = A + `have_xdbe`** (new here) and **AWS = A +
`have_window_system`** (the arm that cannot read the gated text, measured not assumed).  Both sides
were measured on this revision.

| arm | HEAD's unit (slice 89 text) | this slice's text | delta |
| --- | --- | --- | --- |
| A `have_x_windows` | 786 / 71 | **786 / 72** | 0 / **+1** |
| U `+use_toolkit_scroll_bars` | 815 / 71 | **815 / 72** | 0 / **+1** |
| UT `U +use_x_toolkit` | 801 / 70 | **801 / 71** | 0 / **+1** |
| UM `UT +use_motif` | 802 / 70 | **802 / 71** | 0 / **+1** |
| UG `U +use_gtk` | 796 / 73 | **796 / 74** | 0 / **+1** |
| UGX `UG +have_xinput2` | 899 / 75 | **899 / 76** | 0 / **+1** |
| AE `A +enable_checking` | 818 / 70 | **818 / 71** | 0 / **+1** |
| AI `A +have_x_i18n` | 794 / 72 | **794 / 73** | 0 / **+1** |
| AIU `AI +use_gtk` | 831 / 73 | **831 / 74** | 0 / **+1** |
| AIUX `AIU +have_xinput2` | 938 / 75 | **938 / 76** | 0 / **+1** |
| **AX** `A +have_xdbe` | 797 / 71 | **800 / 72** | **+3** / **+1** |
| AWS `A +have_window_system` | 499 / 0 | **499 / 0** | 0 (the `E0428 x8` arm) |

**Additions, by name** (diffed as full `error[E..]: <message>` head lines with counts; nothing
retires in any arm): AX adds `buffer_flipping_blocked_p` x1 (owed), `x_output.complete` x1 and
`x_output.need_buffer_flip` x1 (the two macros' bodies, expanded for the first time).  Every other
arm adds no error at all.

**The one warning in each arm** is `unused_mut` for C 18014's `lval`, whose only writer is the
`have_window_system`-gated `match` - the class slice 26 recorded for its two `lval`s, and kept here
because C's declaration is a single one (a first writing that split it into a `mut` and a non-`mut`
declaration read 0 warnings, and was reverted in favour of the precedent).

**The `have_window_system` arm, measured.**  AWS reads the same **499 / 0** before and after -
`491 E0425 + 8 E0428`, and the 8 are `include/dispextern.rs` 1608/1776-1788's duplicate names, which
is why this lane has no such arm (the slice-26 reading).  My slice's gated `match` therefore cannot
be compiled by any arm this lane builds; its shape is the one already landed at C 28010 with the same
gate and the same limit.

**Two legs of `build-tu`**: exit 0 and `30 warnings emitted` on **both** sides, identical sets.

**Arm reachability** (P-86 7).  `let _: c_int = true;` after C 18015's `let mut shaded: bool = false;`
moves **A 786 -> 787** with `E0308` 13 -> 14, anchored at `build/probes/xterm/_root.rs:27548:18`;
removing it returns A to the table.

**What `--check` reads.**  **28 -> 26** as Decision 103 5 says, with the line-count row at **27,585**
against C's 33,125 (16.7% drift, was 27,511 / 16.9%).

**Not claimed.**  The frozen configuration compiles none of this file: no window-manager state was
read, no frame parameter was stored and no back buffer was flipped; the readings above are compile
readings of text this machine cannot build.

### Slice 91's readings (C 18058-18116), as landed and measured (2026-10-07)

Probe = `cargo run -q -p xtask -- probe src/xterm.rs /tmp/s91/<tag>.o <cfg flags> --crate-type=lib
--emit=metadata -C panic=abort`, all into `CARGO_TARGET_DIR=/tmp/s91/cargo`; A = `have_x_windows`,
U = A + `use_toolkit_scroll_bars`, UT = U + `use_x_toolkit`, UM = UT + `use_motif`, UG = U + `use_gtk`,
UGX = UG + `have_xinput2`, AE = A + `enable_checking`, AI = A + `have_x_i18n`, AIU = AI + `use_gtk`,
AIUX = AIU + `have_xinput2`, AX = A + `have_xdbe`, and **AG3 = A + `use_gtk` + `have_gtk3`** (new
here).  Both sides were measured on this revision.

| arm | HEAD's unit (slice 90 text) | this slice's text | delta |
| --- | --- | --- | --- |
| A `have_x_windows` | 786 / 72 | **786 / 72** | 0 |
| U `+use_toolkit_scroll_bars` | 815 / 72 | **815 / 72** | 0 |
| UT `U +use_x_toolkit` | 801 / 71 | **801 / 71** | 0 |
| UM `UT +use_motif` | 802 / 71 | **802 / 71** | 0 |
| UG `U +use_gtk` | 796 / 74 | **796 / 74** | 0 |
| UGX `UG +have_xinput2` | 899 / 76 | **899 / 76** | 0 |
| AE `A +enable_checking` | 818 / 71 | **818 / 71** | 0 |
| AI `A +have_x_i18n` | 794 / 73 | **794 / 73** | 0 |
| AIU `AI +use_gtk` | 831 / 74 | **831 / 74** | 0 |
| AIUX `AIU +have_xinput2` | 938 / 76 | **938 / 76** | 0 |
| AX `A +have_xdbe` | 800 / 72 | **800 / 72** | 0 |
| **AG3** `A +use_gtk +have_gtk3` | 818 / 73 | **835 / 73** | **+17** |

**Additions, by name** (diffed as full `error[E..]: <message>` head lines with counts; nothing
retires in any arm): only AG3 adds, and its seventeen are `XEvent.xconfigure` x12 (2 -> 14),
`GDK_WINDOW_XID` x2, `x_dispatch_event` x1 (1 -> 2), `GdkEventConfigure.width` x1 and
`GdkEventConfigure.height` x1 - every one a class the ledger counts except the two
`GdkEventConfigure` members and the GTK3 arm's `GDK_WINDOW_XID`, all three recorded as owed.

**Warning sets are identical to the baselines' in all twelve arms.**  The first writing had two
`unused_mut` rows (the read-only `lm_f` and `w_f`), kept or dropped as Decision 104 2 says; dropping
them is what makes this column zero, and the `mut` keyword is the port's own, so nothing of C's text
changed.

**Two legs of `build-tu`**: exit 0 and `30 warnings emitted` on **both** sides, identical sets.

**Arm reachability** (P-86 7).  `let _: c_int = true;` after C 18106's
`let w_f: *mut frame = x_window_to_frame (dpyinfo, wdesc);` moves **A 786 -> 787** with `E0308`
13 -> 14, anchored at `build/probes/xterm/_root.rs:27666:18`; removing it returns A to the table.

**What `--check` reads.**  **26 -> 25** as Decision 104 4 says, with the line-count row at **27,664**
against C's 33,125 (16.5% drift, was 27,585 / 16.7%).

**Not claimed.**  The frozen configuration compiles none of this file: no scroll-bar configure event
was synthesised, no event was dispatched and no frame was chosen from a mouse grab; the readings
above are compile readings of text this machine cannot build.

### Slice 92's readings (C 18118-18187), as landed and measured (2026-10-07)

Probe = `cargo run -q -p xtask -- probe src/xterm.rs /tmp/s92/<tag>.o <cfg flags> --crate-type=lib
--emit=metadata -C panic=abort`, all into `CARGO_TARGET_DIR=/tmp/s92/cargo`; A = `have_x_windows`,
U = A + `use_toolkit_scroll_bars`, UT = U + `use_x_toolkit`, UM = UT + `use_motif`, UG = U + `use_gtk`,
UGX = UG + `have_xinput2`, AE = A + `enable_checking`, AI = A + `have_x_i18n`, AIU = AI + `use_gtk`,
AIUX = AIU + `have_xinput2`, AX = A + `have_xdbe`, AG3 = A + `use_gtk` + `have_gtk3`.  Both sides
were measured on this revision.

| arm | HEAD's unit (slice 91 text) | this slice's text | delta |
| --- | --- | --- | --- |
| A `have_x_windows` | 786 / 72 | **788 / 72** | **+2** |
| U `+use_toolkit_scroll_bars` | 815 / 72 | **817 / 72** | **+2** |
| UT `U +use_x_toolkit` | 801 / 71 | **803 / 71** | **+2** |
| UM `UT +use_motif` | 802 / 71 | **804 / 71** | **+2** |
| UG `U +use_gtk` | 796 / 74 | **798 / 74** | **+2** |
| UGX `UG +have_xinput2` | 899 / 76 | **901 / 76** | **+2** |
| AE `A +enable_checking` | 818 / 71 | **820 / 71** | **+2** |
| AI `A +have_x_i18n` | 794 / 73 | **796 / 73** | **+2** |
| AIU `AI +use_gtk` | 831 / 74 | **833 / 74** | **+2** |
| AIUX `AIU +have_xinput2` | 938 / 76 | **940 / 76** | **+2** |
| AX `A +have_xdbe` | 800 / 72 | **802 / 72** | **+2** |
| AG3 `A +use_gtk +have_gtk3` | 835 / 73 | **837 / 73** | **+2** |

**Additions, by name** (diffed as full `error[E..]: <message>` head lines with counts, both
directions): every arm adds `cannot find function x_display_pixel_width` x1 and
`cannot find function x_display_pixel_height` x1, and retires nothing; the pair is the recorded
"definition not yet landed" class, and their `--check` rows are still in the list of 24.

**Warning sets are identical to the baselines' in all twelve arms.**

**The conversion reading, measured.**  The first writing's fourteen implicit `XFIXNUM` mixes read
**14 `E0308` + 12 `E0277` rows per arm** (the compound assignments are the `E0277` pair, the rest the
`E0308` ones); with C's conversions stated the same arms read their baseline histograms plus the two
`E0425` rows above.  Nothing else in the function moved.

**Two legs of `build-tu`**: exit 0 and `30 warnings emitted` on **both** sides, identical sets.

**Arm reachability** (P-86 7).  `let _: c_int = true;` after C 18128's `max_y = -1;` moves
**A 788 -> 789** with `E0308` 13 -> 14, anchored at `build/probes/xterm/_root.rs:27706:18`; removing
it returns A to the table.

**What `--check` reads.**  **25 -> 24** as Decision 105 4 says, with the line-count row at **27,753**
against C's 33,125 (16.2% drift, was 27,664 / 16.5%).

**Not claimed.**  The frozen configuration compiles none of this file: no monitor geometry was
searched, no tooltip position was clamped and no Lisp list was walked; the readings above are compile
readings of text this machine cannot build.

### Slice 93's readings (C 18189-18259), as landed and measured (2026-10-07)

Probe = `cargo run -q -p xtask -- probe src/xterm.rs /tmp/s93/<tag>.o <cfg flags> --crate-type=lib
--emit=metadata -C panic=abort`, all into `CARGO_TARGET_DIR=/tmp/s93/cargo`; A = `have_x_windows`,
U = A + `use_toolkit_scroll_bars`, UT = U + `use_x_toolkit`, UM = UT + `use_motif`, UG = U + `use_gtk`,
UGX = UG + `have_xinput2`, AE = A + `enable_checking`, AI = A + `have_x_i18n`, AIU = AI + `use_gtk`,
AIUX = AIU + `have_xinput2`, AX = A + `have_xdbe`, AG3 = A + `use_gtk` + `have_gtk3`.  Both sides
were measured on this revision.

| arm | HEAD's unit (slice 92 text) | this slice's text | delta |
| --- | --- | --- | --- |
| A `have_x_windows` | 788 / 72 | **789 / 72** | **+1** |
| U `+use_toolkit_scroll_bars` | 817 / 72 | **818 / 72** | **+1** |
| UT `U +use_x_toolkit` | 803 / 71 | **804 / 71** | **+1** |
| UM `UT +use_motif` | 804 / 71 | **805 / 71** | **+1** |
| UG `U +use_gtk` | 798 / 74 | **799 / 74** | **+1** |
| **UGX** `UG +have_xinput2` | 901 / 76 | **901 / 76** | 0 |
| AE `A +enable_checking` | 820 / 71 | **821 / 71** | **+1** |
| AI `A +have_x_i18n` | 796 / 73 | **797 / 73** | **+1** |
| AIU `AI +use_gtk` | 833 / 74 | **834 / 74** | **+1** |
| **AIUX** `AIU +have_xinput2` | 940 / 76 | **940 / 76** | 0 |
| AX `A +have_xdbe` | 802 / 72 | **803 / 72** | **+1** |
| AG3 `A +use_gtk +have_gtk3` | 837 / 73 | **838 / 73** | **+1** |

**Additions, by name** (diffed as full `error[E..]: <message>` head lines with counts, both
directions): the ten arms without `have_xinput2` add `cannot find function, tuple struct or tuple
variant XQueryPointer` x1 and retire nothing; UGX and AIUX read **no diff at all** - their callee is
`x_query_pointer_1`, this unit's own definition.

**Warning sets are identical to the baselines' in all twelve arms.**  The first writing's three
defects read, in order: the prelude's `None` (`E0605` in every arm), the seven `E0381` rows in the
two `have_xinput2` arms only (the unresolved-callee silence, slice 85's reading), and one
`unused_mut` per arm for `tip_f` (dropped - the keyword is the port's own).

**Two legs of `build-tu`**: exit 0 and `30 warnings emitted` on **both** sides, identical sets.

**Arm reachability** (P-86 7).  `let _: c_int = true;` after C 18200's `tip_f = XFRAME (tip_frame);`
moves **A 789 -> 790** with `E0308` 13 -> 14, anchored at `build/probes/xterm/_root.rs:27787:18`;
removing it returns A to the table.

**What `--check` reads.**  **24 -> 22** as Decision 106 4 says, with the line-count row at **27,848**
against C's 33,125 (15.9% drift, was 27,753 / 16.2%).

**Not claimed.**  The frozen configuration compiles none of this file: no pointer was queried, no
tooltip was moved and no DND state was advanced; the readings above are compile readings of text this
machine cannot build.

### Slice 94's readings (C 18481-18497), as landed and measured (2026-10-07)

Probe = `cargo run -q -p xtask -- probe src/xterm.rs /tmp/s94/<tag>.o --crate-type=lib
--emit=metadata -C panic=abort <cfg flags>`, all into `CARGO_TARGET_DIR=/tmp/s94/cargo`; A =
`have_x_windows`, U = A + `use_toolkit_scroll_bars`, UT = U + `use_x_toolkit`, UM = UT + `use_motif`,
UG = U + `use_gtk`, UGX = UG + `have_xinput2`, AE = A + `enable_checking`, AI = A + `have_x_i18n`,
AIU = AI + `use_gtk`, AIUX = AIU + `have_xinput2`, AX = A + `have_xdbe`, AG3 = A + `use_gtk` +
`have_gtk3`.  Both sides were measured on this revision (the baseline side by restoring
`src/xterm.rs` and `src/include/gtk.rs` from HEAD, then putting both back - the restored text's md5 is
the saved copy's).

| arm | HEAD's unit (slice 93 text) | this slice's text | delta |
| --- | --- | --- | --- |
| A `have_x_windows` | 789 / 72 | **785 / 72** | **-4** |
| U `+use_toolkit_scroll_bars` | 818 / 72 | **814 / 72** | **-4** |
| UT `U +use_x_toolkit` | 804 / 71 | **800 / 71** | **-4** |
| UM `UT +use_motif` | 805 / 71 | **801 / 71** | **-4** |
| **UG** `U +use_gtk` | 799 / 74 | **797 / 74** | **-2** |
| **UGX** `UG +have_xinput2` | 901 / 76 | **899 / 76** | **-2** |
| AE `A +enable_checking` | 821 / 71 | **817 / 71** | **-4** |
| AI `A +have_x_i18n` | 797 / 73 | **793 / 73** | **-4** |
| **AIU** `AI +use_gtk` | 834 / 74 | **832 / 74** | **-2** |
| **AIUX** `AIU +have_xinput2` | 940 / 76 | **938 / 76** | **-2** |
| AX `A +have_xdbe` | 803 / 72 | **799 / 72** | **-4** |
| AG3 `A +use_gtk +have_gtk3` | 838 / 73 | **836 / 73** | **-2** |

**Retirements, by name and by site** (diffed as `error[E..]: <message>` head lines with their
locations, both directions).  The seven non-`use_gtk` arms: `cannot find function
x_display_pixel_width` x3 at C 27531, C 30266 and C 18160, `cannot find function
x_display_pixel_height` x3 at C 27573, C 30268 and C 18161, and nothing else.  The five `use_gtk`
arms: `E0308 mismatched types` x4 at the same C 27531/C 27573/C 18160/C 18161 sites (their resolution
came from the mirror's `*mut c_void` declaration, so what retires is the mismatch - the two
`x_wm_set_size_hint` rows are not compiled there).

**Additions, by name**: `cannot find function, tuple struct or tuple variant HeightOfScreen` x1 and
`... WidthOfScreen` x1 in **all twelve** arms - the two new bodies' Xlib macros, the bare-name class.

**Warning sets are identical to the baselines' in all twelve arms** (compared as multiset of
`warning:` messages, including the two `suspicious definition of the runtime write/read symbol` rows
and the 24 macro-parenthesis rows).

**Two legs of `build-tu`**: `src/xterm.rs` - exit 0 and `30 warnings emitted` on **both** sides,
identical sets.  The mirror this slice edits has 15 includers (`emacsgtkfixed`, `ftcrfont`,
`gfilenotify`, `gtkutil`, `image`, `pgtkfns`, `pgtkim`, `pgtkmenu`, `pgtkselect`, `pgtkterm`, `xfns`,
`xgselect`, `xmenu`, `xsettings`, `xwidget`), and the mirror-change leg builds all 15 on both sides:
12 `built`, and three whose errors are the dead-text class, **identical error sets at HEAD and here**
(`gfilenotify` 4 `cannot find macro DEFUN`; `pgtkfns` 12 `DEFUN` + 6 `cannot find type c_int` + 4
`resource_types`; `pgtkterm` `GtkWidget`/`GdkWindow`) - so the declaration's move adds no failure
anywhere in the set.

**Arm reachability** (P-86 7).  `let _: c_int = true;` before C 18484's `if (*dpyinfo).screen_height
!= 0` moves **A 785 -> 786** with `E0308` 13 -> 14, anchored at
`build/probes/xterm/_root.rs:27876:18`; removing it returns A to the table (and the restored file's
md5 is the saved copy's).

**What `--check` reads.**  **22 -> 20** as Decision 107 5 says, with the line-count row at **27,881**
against C's 33,125 (15.8% drift, was 27,848 / 15.9%).

**Not claimed.**  The frozen configuration compiles none of this file: no screen was measured, no
frame was positioned and no display's dimensions were taken; the readings above are compile readings
of text this machine cannot build.

### Slice 95's readings (C 18261-18479), as landed and measured (2026-10-07)

Probe = `cargo run -q -p xtask -- probe src/xterm.rs /tmp/s95/<tag>.o --crate-type=lib
--emit=metadata -C panic=abort <cfg flags>`; A = `have_x_windows`, U = A +
`use_toolkit_scroll_bars`, UT = U + `use_x_toolkit`, UM = UT + `use_motif`, UG = U + `use_gtk`,
UGX = UG + `have_xinput2`, AE = A + `enable_checking`, AI = A + `have_x_i18n`, AIU = AI +
`use_gtk`, AIUX = AIU + `have_xinput2`, AX = A + `have_xdbe`, AG3 = A + `use_gtk` + `have_gtk3`.
Both sides were measured on this revision (the baseline by restoring `src/xterm.rs` from HEAD, then
putting the saved copy back - its md5 is the copy's).

| arm | HEAD's unit (slice 94 text) | this slice's text | delta |
| --- | --- | --- | --- |
| A `have_x_windows` | 785 / 72 | **785 / 73** | **0 / +1 w** |
| U `+use_toolkit_scroll_bars` | 814 / 72 | **814 / 73** | **0 / +1 w** |
| UT `U +use_x_toolkit` | 800 / 71 | **800 / 72** | **0 / +1 w** |
| UM `UT +use_motif` | 801 / 71 | **801 / 72** | **0 / +1 w** |
| **UG** `U +use_gtk` | 797 / 74 | **802 / 74** | **+5 / 0** |
| **UGX** `UG +have_xinput2` | 899 / 76 | **904 / 76** | **+5 / 0** |
| AE `A +enable_checking` | 817 / 71 | **817 / 72** | **0 / +1 w** |
| AI `A +have_x_i18n` | 793 / 73 | **793 / 74** | **0 / +1 w** |
| **AIU** `AI +use_gtk` | 832 / 74 | **837 / 74** | **+5 / 0** |
| **AIUX** `AIU +have_xinput2` | 938 / 76 | **943 / 76** | **+5 / 0** |
| AX `A +have_xdbe` | 799 / 72 | **799 / 73** | **0 / +1 w** |
| AG3 `A +use_gtk +have_gtk3` | 836 / 73 | **841 / 73** | **+5 / 0** |

**Additions, by name.**  All twelve arms add `warning: value assigned to `dmsg` is never read`
(C 18454's dead store, Decision 108 4).  The five `use_gtk` arms add five rows each, one per site of
`FRAME_OUTER_WINDOW (x_dnd_frame)` (C 18294/18303/18309/18342/18348): in UG/UGX/AIU/AIUX the mirror's
`FRAME_OUTER_WINDOW!` reaches `GDK_WINDOW_XID!`, whose body names `GDK_WINDOW_XWINDOW` (no mirror
declares it), and in AG3 the same chain stops one link earlier (`cannot find macro GDK_WINDOW_XID`).
That is the class slice 81's `estimate_mode_line_height` opened - a mirror body reaching a name the
tree does not carry - and its home is the `include/gtk.rs`/`include/xterm.rs` GDK surface, not this
unit.  Nothing retires in any arm.

**Two legs of `build-tu`**: `src/xterm.rs` - exit 0 and `30 warnings emitted` on **both** sides with
identical sets: the frozen configuration has no window system, so this function (like the rest of the
file) is not compiled there and the new warning is an arm reading, not a frozen one.

**Arm reachability** (P-86 7).  `let _: c_int = true;` after C 18273's `bool was_frame;` moves
**A 785 -> 786** with `E0308` 13 -> 14, anchored at `build/probes/xterm/_root.rs:27899:18`; the same
run reads **72** warnings rather than 73, because the injected error stops the MIR of the very
function where the new warning lives (slice 85's mechanic, read in the other direction).  Removing
the line returns A to the table.

**What `--check` reads.**  **20 -> 19** as Decision 108 5 says, with the line-count row at **28,148**
against C's 33,125 (15.0% drift, was 27,881 / 15.8%).

**Not claimed.**  The frozen configuration compiles none of this file: no pointer was queried, no
DND state was advanced and no Motif message was built; the readings above are compile readings of
text this machine cannot build.

### Slice 96's readings (C 18499-18612), as landed and measured (2026-10-07)

Probe = `cargo run -q -p xtask -- probe src/xterm.rs /tmp/s96/<tag>.o --crate-type=lib
--emit=metadata -C panic=abort <cfg flags>`; A = `have_x_windows`, U = A +
`use_toolkit_scroll_bars`, UT = U + `use_x_toolkit`, UM = UT + `use_motif`, UG = U + `use_gtk`,
UGX = UG + `have_xinput2`, AE = A + `enable_checking`, AI = A + `have_x_i18n`, AIU = AI +
`use_gtk`, AIUX = AIU + `have_xinput2`, AX = A + `have_xdbe`, AG3 = A + `use_gtk` + `have_gtk3`.
Both sides were measured on this revision (the baseline by restoring `src/xterm.rs` from HEAD, then
putting the saved copy back - its md5 is the copy's).

| arm | HEAD's unit (slice 95 text) | this slice's text | delta |
| --- | --- | --- | --- |
| A `have_x_windows` | 785 / 73 | **790 / 73** | **+5 / 0** |
| U `+use_toolkit_scroll_bars` | 814 / 73 | **819 / 73** | **+5 / 0** |
| UT `U +use_x_toolkit` | 800 / 72 | **805 / 72** | **+5 / 0** |
| UM `UT +use_motif` | 801 / 72 | **806 / 72** | **+5 / 0** |
| **UG** `U +use_gtk` | 802 / 74 | **803 / 74** | **+1 / 0** |
| **UGX** `UG +have_xinput2` | 904 / 76 | **905 / 76** | **+1 / 0** |
| AE `A +enable_checking` | 817 / 72 | **822 / 72** | **+5 / 0** |
| AI `A +have_x_i18n` | 793 / 74 | **798 / 74** | **+5 / 0** |
| **AIU** `AI +use_gtk` | 837 / 74 | **838 / 74** | **+1 / 0** |
| **AIUX** `AIU +have_xinput2` | 943 / 76 | **944 / 76** | **+1 / 0** |
| AX `A +have_xdbe` | 799 / 73 | **804 / 73** | **+5 / 0** |
| AG3 `A +use_gtk +have_gtk3` | 841 / 73 | **842 / 73** | **+1 / 0** |

**Additions, by name.**  Seven arms: `cannot find function, tuple struct or tuple variant
ConnectionNumber` x1, the same for `XPending` x2 and `XNextEvent` x1, and `cannot find function
handle_one_xevent` x1 - all of them the recorded classes (Xlib's bare names, and the unported
`handle_one_xevent`, which is a `--check` row of its own).  Five arms: `ConnectionNumber` x1 only,
because C 18532-18557 is inside the `#ifndef USE_GTK` half and those arms compile the `#else` half.
**Warning sets are identical to the baselines' in all twelve arms** - the first writing's `while
true` cost one `denote infinite loops with `loop { ... }`` warning per arm, and that is why the
port's spelling is `loop` (this file 21756).

**Two legs of `build-tu`**: `src/xterm.rs` - exit 0 and `30 warnings emitted` on **both** sides,
identical sets (the frozen configuration has no window system, so this function is not compiled
there).

**Arm reachability** (P-86 7).  `let _: c_int = true;` after C 18515's `at = timespec_add (...)` moves
**A 790 -> 791** with `E0308` 13 -> 14, anchored at `build/probes/xterm/_root.rs:28204:18`; removing
the line returns A to the table.

**What `--check` reads.**  **19 -> 18** as Decision 109 5 says, with the line-count row at **28,303**
against C's 33,125 (14.6% drift, was 28,148 / 15.0%).

**Not claimed.**  The frozen configuration compiles none of this file: no display was polled, no
event was read and no cell was inspected; the readings above are compile readings of text this
machine cannot build.

### Slice 97's readings (C 18614-18679), as landed and measured (2026-10-07)

Probe = `cargo run -q -p xtask -- probe src/xterm.rs /tmp/s97/<tag>.o --crate-type=lib
--emit=metadata -C panic=abort <cfg flags>`; A = `have_x_windows`, U = A +
`use_toolkit_scroll_bars`, UT = U + `use_x_toolkit`, UM = UT + `use_motif`, UG = U + `use_gtk`,
UGX = UG + `have_xinput2`, AE = A + `enable_checking`, AI = A + `have_x_i18n`, AIU = AI +
`use_gtk`, AIUX = AIU + `have_xinput2`, AX = A + `have_xdbe`, AG3 = A + `use_gtk` + `have_gtk3`.
Both sides were measured on this revision (the baseline by restoring `src/xterm.rs` from HEAD, then
putting the saved copy back - its md5 is the copy's).

| arm | HEAD's unit (slice 96 text) | this slice's text | delta |
| --- | --- | --- | --- |
| A `have_x_windows` | 790 / 73 | **790 / 73** | 0 |
| U `+use_toolkit_scroll_bars` | 819 / 73 | **819 / 73** | 0 |
| UT `U +use_x_toolkit` | 805 / 72 | **805 / 72** | 0 |
| UM `UT +use_motif` | 806 / 72 | **806 / 72** | 0 |
| **UG** `U +use_gtk` | 803 / 74 | **805 / 74** | **+2** |
| **UGX** `UG +have_xinput2` | 905 / 76 | **907 / 76** | **+2** |
| AE `A +enable_checking` | 822 / 72 | **822 / 72** | 0 |
| AI `A +have_x_i18n` | 798 / 74 | **798 / 74** | 0 |
| **AIU** `AI +use_gtk` | 838 / 74 | **840 / 74** | **+2** |
| **AIUX** `AIU +have_xinput2` | 944 / 76 | **946 / 76** | **+2** |
| AX `A +have_xdbe` | 804 / 73 | **804 / 73** | 0 |
| AG3 `A +use_gtk +have_gtk3` | 842 / 73 | **844 / 73** | **+2** |

**Additions, by name.**  The five `use_gtk` arms: `E0609 no field last_monitor_attributes_list on
type x_display_info` x2, the two sites of C 18663/18677 - the mirror carries the frozen
configuration's `x_display_info` layout, and C's member is behind `#if defined HAVE_XRANDR ||
defined USE_GTK` (xterm.h 831-834).  The seven others: nothing, in either direction.
**Warning sets are identical to the baselines' in all twelve arms.**

**Two legs of `build-tu`**: `src/xterm.rs` - exit 0 and `30 warnings emitted` on **both** sides,
identical sets (the frozen configuration has no window system, so neither function is compiled
there).

**Arm reachability** (P-86 7).  `let _: c_int = true;` after C 18623's `event = kbd_fetch_ptr;` moves
**A 790 -> 791** with `E0308` 13 -> 14, anchored at `build/probes/xterm/_root.rs:28333:18`; removing
the line returns A to the table.

**What `--check` reads.**  **18 -> 17** as Decision 110 5 says, with the line-count row at **28,389**
against C's 33,125 (14.3% drift, was 28,303 / 14.6%).

**Not claimed.**  The frozen configuration compiles none of this file: no event queue was walked, no
monitor list was compared and no terminal was set; the readings above are compile readings of text
this machine cannot build.

### Slice 98's readings (C 18681-18761), as landed and measured (2026-10-07)

Probe = `cargo run -q -p xtask -- probe src/xterm.rs /tmp/s98/<tag>.o --crate-type=lib
--emit=metadata -C panic=abort <cfg flags>`; A = `have_x_windows`, U = A +
`use_toolkit_scroll_bars`, UT = U + `use_x_toolkit`, UM = UT + `use_motif`, UG = U + `use_gtk`,
UGX = UG + `have_xinput2`, AE = A + `enable_checking`, AI = A + `have_x_i18n`, AIU = AI +
`use_gtk`, AIUX = AIU + `have_xinput2`, AX = A + `have_xdbe`, AG3 = A + `use_gtk` + `have_gtk3`.
Both sides were measured on this revision (the baseline by restoring `src/xterm.rs` from HEAD, then
putting the saved copy back - its md5 is the copy's).

| arm | HEAD's unit (slice 97 text) | this slice's text | delta |
| --- | --- | --- | --- |
| A `have_x_windows` | 790 / 73 | **802 / 74** | **+12 / +1 w** |
| U `+use_toolkit_scroll_bars` | 819 / 73 | **831 / 74** | **+12 / +1 w** |
| UT `U +use_x_toolkit` | 805 / 72 | **817 / 73** | **+12 / +1 w** |
| UM `UT +use_motif` | 806 / 72 | **818 / 73** | **+12 / +1 w** |
| UG `U +use_gtk` | 805 / 74 | **817 / 75** | **+12 / +1 w** |
| UGX `UG +have_xinput2` | 907 / 76 | **919 / 77** | **+12 / +1 w** |
| AE `A +enable_checking` | 822 / 72 | **834 / 73** | **+12 / +1 w** |
| AI `A +have_x_i18n` | 798 / 74 | **810 / 75** | **+12 / +1 w** |
| AIU `AI +use_gtk` | 840 / 74 | **852 / 75** | **+12 / +1 w** |
| AIUX `AIU +have_xinput2` | 946 / 76 | **958 / 77** | **+12 / +1 w** |
| AX `A +have_xdbe` | 804 / 73 | **816 / 74** | **+12 / +1 w** |
| AG3 `A +use_gtk +have_gtk3` | 844 / 73 | **856 / 74** | **+12 / +1 w** |

**Additions, by name** (the same in all twelve): `E0609 no field `xclient` on type
`xlib_surface::XEvent`` x11 and `no field `r#type`` x1 - the twelve accesses C 18694-18752 makes
through the opaque `XEvent` (one `type`, four `message_type`, three `format`, four `data.l`).  The
warning is `unnecessary parentheses around `if` condition` x1, C 18743-18750's group pair, which
rustc's `unused_parens` misreads as removable (proved on a two-line probe).  **Nothing retires in any
arm.**

**Two legs of `build-tu`**: `src/xterm.rs` - exit 0 and `30 warnings emitted` on **both** sides,
identical sets (the frozen configuration has no window system, so this function is not compiled
there).

**Arm reachability** (P-86 7).  `let _: c_int = true;` after C 18692's `unsigned long kde_data;` moves
**A 802 -> 803** with `E0308` 13 -> 14, anchored at `build/probes/xterm/_root.rs:28429:18`; removing
the line returns A to the table.

**What `--check` reads.**  **17 -> 16** as Decision 111 5 says, with the line-count row at **28,487**
against C's 33,125 (14.0% drift, was 28,389 / 14.3%).

**Not claimed.**  The frozen configuration compiles none of this file: no client message was read and
no coordinates were extracted; the readings above are compile readings of text this machine cannot
build.

### Slice 99's readings (C 18763-18824), as landed and measured (2026-10-07)

Probe = `cargo run -q -p xtask -- probe src/xterm.rs /tmp/s99/<tag>.o --crate-type=lib
--emit=metadata -C panic=abort <cfg flags>`; A = `have_x_windows`, U = A +
`use_toolkit_scroll_bars`, UT = U + `use_x_toolkit`, UM = UT + `use_motif`, UG = U + `use_gtk`,
UGX = UG + `have_xinput2`, AE = A + `enable_checking`, AI = A + `have_x_i18n`, AIU = AI +
`use_gtk`, AIUX = AIU + `have_xinput2`, AX = A + `have_xdbe`, AG3 = A + `use_gtk` + `have_gtk3`.
Both sides were measured on this revision (the baseline by restoring `src/xterm.rs` from HEAD, then
putting the saved copy back - its md5 is the copy's).

| arm | HEAD's unit (slice 98 text) | this slice's text | delta |
| --- | --- | --- | --- |
| A `have_x_windows` | 802 / 74 | **803 / 74** | **+1 / 0** |
| U `+use_toolkit_scroll_bars` | 831 / 74 | **832 / 74** | **+1 / 0** |
| UT `U +use_x_toolkit` | 817 / 73 | **818 / 73** | **+1 / 0** |
| UM `UT +use_motif` | 818 / 73 | **819 / 73** | **+1 / 0** |
| **UG** `U +use_gtk` | 817 / 75 | **819 / 75** | **+2 / 0** |
| **UGX** `UG +have_xinput2` | 919 / 77 | **921 / 77** | **+2 / 0** |
| AE `A +enable_checking` | 834 / 73 | **835 / 73** | **+1 / 0** |
| AI `A +have_x_i18n` | 810 / 75 | **811 / 75** | **+1 / 0** |
| **AIU** `AI +use_gtk` | 852 / 75 | **854 / 75** | **+2 / 0** |
| **AIUX** `AIU +have_xinput2` | 958 / 77 | **960 / 77** | **+2 / 0** |
| AX `A +have_xdbe` | 816 / 74 | **817 / 74** | **+1 / 0** |
| **AG3** `A +use_gtk +have_gtk3` | 856 / 74 | **858 / 74** | **+2 / 0** |

**Additions, by name.**  Every arm reads one `E0609 no field has_been_visible on type x_output` (C
18798, the mirror's bit-field carried as a `// C:` line), and the five `use_gtk` arms read one more:
`cannot find function, tuple struct or tuple variant GDK_WINDOW_XWINDOW` in UG/UGX/AIU and
`cannot find macro GDK_WINDOW_XID` in AG3 - C 18776's `FRAME_OUTER_WINDOW (f)` inside the GTK arm's
mirror chain, the class slice 95 opened.  **Warning sets are identical to the baselines' in all
twelve arms**, and nothing retires in any arm.

**Two legs of `build-tu`**: `src/xterm.rs` - exit 0 and `30 warnings emitted` on **both** sides,
identical sets (the frozen configuration has no window system, so this function is not compiled
there).

**Arm reachability** (P-86 7).  `let _: c_int = true;` after C 18773's `data = NULL;` moves
**A 803 -> 804** with `E0308` 13 -> 14, anchored at `build/probes/xterm/_root.rs:28529:18`; removing
the line returns A to the table.

**What `--check` reads.**  **16 -> 15** as Decision 112 5 says, with the line-count row at **28,569**
against C's 33,125 (13.8% drift, was 28,487 / 14.0%).

**Not claimed.**  The frozen configuration compiles none of this file: no window-manager state was
read and no frame flag was set; the readings above are compile readings of text this machine cannot
build.

### Slice 100's readings (C 18826-18864), as landed and measured (2026-10-07)

Probe = `cargo run -q -p xtask -- probe src/xterm.rs /tmp/s100/<tag>.o --crate-type=lib
--emit=metadata -C panic=abort <cfg flags>`; A = `have_x_windows`, U = A +
`use_toolkit_scroll_bars`, UT = U + `use_x_toolkit`, UM = UT + `use_motif`, UG = U + `use_gtk`,
UGX = UG + `have_xinput2`, AE = A + `enable_checking`, AI = A + `have_x_i18n`, AIU = AI +
`use_gtk`, AIUX = AIU + `have_xinput2`, AX = A + `have_xdbe`, AG3 = A + `use_gtk` + `have_gtk3`, and
**AXF = A + `have_xfixes`** (the arm this slice adds, because `HAVE_XFIXES` is `undef` in the frozen
table and none of the other twelve compiles the block).  Both sides were measured on this revision
(the baseline by restoring `src/xterm.rs` from HEAD, then putting the saved copy back - its md5 is
the copy's).

| arm | HEAD's unit (slice 99 text) | this slice's text | delta |
| --- | --- | --- | --- |
| A `have_x_windows` | 803 / 74 | **803 / 74** | 0 |
| U `+use_toolkit_scroll_bars` | 832 / 74 | **832 / 74** | 0 |
| UT `U +use_x_toolkit` | 818 / 73 | **818 / 73** | 0 |
| UM `UT +use_motif` | 819 / 73 | **819 / 73** | 0 |
| UG `U +use_gtk` | 819 / 75 | **819 / 75** | 0 |
| UGX `UG +have_xinput2` | 921 / 77 | **921 / 77** | 0 |
| AE `A +enable_checking` | 835 / 73 | **835 / 73** | 0 |
| AI `A +have_x_i18n` | 811 / 75 | **811 / 75** | 0 |
| AIU `AI +use_gtk` | 854 / 75 | **854 / 75** | 0 |
| AIUX `AIU +have_xinput2` | 960 / 77 | **960 / 77** | 0 |
| AX `A +have_xdbe` | 817 / 74 | **817 / 74** | 0 |
| AG3 `A +use_gtk +have_gtk3` | 858 / 74 | **858 / 74** | 0 |
| **AXF** `A +have_xfixes` | 809 / 74 | **816 / 74** | **+7 / 0** |

**Additions, by name.**  Only AXF moves, and all seven rows are `E0609 no field` on C's
`#ifdef HAVE_XFIXES` members of `x_display_info`: `monitored_selections` x4 (C 18843's read, C 18844's
write, C 18857's read, C 18858's read), `selection_tracking_window` x1 (C 18837) and
`n_monitored_selections` x2 (C 18840, C 18855).  `XFixesSelectionNotifyEvent` produces **no** row: the
unit declares it at the use site (P-88 1).  **Warning sets are identical to the baselines' in all
thirteen arms**, and nothing retires in any of them.

**Two legs of `build-tu`**: `src/xterm.rs` - exit 0 and `30 warnings emitted` on **both** sides,
identical sets (the frozen configuration has `HAVE_XFIXES` undefined, so this block is not compiled
there either).

**Arm reachability** (P-86 7).  `let _: c_int = true;` after C 18853's `int i;` moves **AXF 816 ->
817** with `E0308` 13 -> 14, anchored at `build/probes/xterm/_root.rs:28658:18` - the arm has to be
`AXF`, because the twelve others do not compile the block (`AXF` is also what shows the injection's
own parse error when it is put in the record's body instead of a function's).

**What `--check` reads.**  **15 -> 15** as Decision 113 5 says (neither function is demanded by the
frozen table and the token check does not reach them), with the line-count row at **28,657** against
C's 33,125 (13.5% drift, was 28,569 / 13.8%).

**Not claimed.**  The frozen configuration compiles none of this file, and no other arm compiles this
block either: no selection was monitored, no owner was looked up and no event was inspected; the
readings above are compile readings, and the SDK record's layout stays UNVERIFIED until a machine
with `<X11/extensions/Xfixes.h>` measures it (Owed item 0).

### Slice 101's readings (C 25641-25811), as landed and measured (2026-10-07)

Probe = `cargo run -q -p xtask -- probe src/xterm.rs /tmp/s101/<tag>.o --crate-type=lib
--emit=metadata -C panic=abort <cfg flags>`; A = `have_x_windows`, U = A +
`use_toolkit_scroll_bars`, UT = U + `use_x_toolkit`, UM = UT + `use_motif`, UG = U + `use_gtk`,
UGX = UG + `have_xinput2`, AE = A + `enable_checking`, AI = A + `have_x_i18n`, AIU = AI +
`use_gtk`, AIUX = AIU + `have_xinput2`, AX = A + `have_xdbe`, AG3 = A + `use_gtk` + `have_gtk3`,
AXF = A + `have_xfixes` (slice 100's arm).  Both sides were measured on this revision (the baseline
by restoring `src/xterm.rs` from HEAD, then putting the saved copy back - its md5 is the copy's).

| arm | HEAD's unit (slice 100 text) | this slice's text | delta |
| --- | --- | --- | --- |
| A `have_x_windows` | 803 / 74 | **807 / 74** | **+4 / 0** |
| U `+use_toolkit_scroll_bars` | 832 / 74 | **836 / 74** | **+4 / 0** |
| UT `U +use_x_toolkit` | 818 / 73 | **822 / 73** | **+4 / 0** |
| UM `UT +use_motif` | 819 / 73 | **823 / 73** | **+4 / 0** |
| UG `U +use_gtk` | 819 / 75 | **819 / 75** | 0 |
| UGX `UG +have_xinput2` | 921 / 77 | **921 / 77** | 0 |
| AE `A +enable_checking` | 835 / 73 | **839 / 73** | **+4 / 0** |
| AI `A +have_x_i18n` | 811 / 75 | **815 / 75** | **+4 / 0** |
| AIU `AI +use_gtk` | 854 / 75 | **854 / 75** | 0 |
| AIUX `AIU +have_xinput2` | 960 / 77 | **960 / 77** | 0 |
| AX `A +have_xdbe` | 817 / 74 | **821 / 74** | **+4 / 0** |
| AG3 `A +use_gtk +have_gtk3` | 858 / 74 | **857 / 74** | **-1 / 0** |
| AXF `A +have_xfixes` | 816 / 74 | **820 / 74** | **+4 / 0** |

**Additions and retirements, by name.**  The eight arms that compile `XTread_socket`'s
`#ifndef USE_GTK` block (the seven plus AXF): `XPending` x1, `XNextEvent` x1, `XNoOp` x1,
`handle_one_xevent` x2 (C 25656 and C 25732) and the retirement of `cannot find value XTread_socket`
x1 - the terminal hook table's reference, which this slice resolves.  UG/UGX/AIU/AIUX: `XNoOp` x1 and
`handle_one_xevent` x1 arrive (C 25656 only), and `XTread_socket`'s and `x_dispatch_event`'s
hook-table references both retire.  AG3: the same, except that two `x_dispatch_event` references
retire.  **Warning sets are identical to the baselines' in all thirteen arms.**

**Two legs of `build-tu`**: `src/xterm.rs` - exit 0 and `30 warnings emitted` on **both** sides,
identical sets (the frozen configuration has no window system, so neither function is compiled
there).

**Arm reachability** (P-86 7).  `let _: c_int = true;` after C 25674's `int count = 0;` moves
**A 807 -> 808** with `E0308` 13 -> 14, anchored at `build/probes/xterm/_root.rs:28736:18`; removing
the line returns A to the table.

**What `--check` reads.**  **15 -> 13** as Decision 114 5 says, with the line-count row at **28,874**
against C's 33,125 (12.8% drift, was 28,657 / 13.5%).

**Not claimed.**  The frozen configuration compiles none of this file: no X event was read, no no-op
was requested and no frame was raised; the readings above are compile readings of text this machine
cannot build.

### Slice 102's readings (C 25816-25902), as landed and measured (2026-10-07)

Probe = `cargo run -q -p xtask -- probe src/xterm.rs /tmp/s102/<tag>.o --crate-type=lib
--emit=metadata -C panic=abort <cfg flags>`; A = `have_x_windows`, U = A +
`use_toolkit_scroll_bars`, UT = U + `use_x_toolkit`, UM = UT + `use_motif`, UG = U + `use_gtk`,
UGX = UG + `have_xinput2`, AE = A + `enable_checking`, AI = A + `have_x_i18n`, AIU = AI +
`use_gtk`, AIUX = AIU + `have_xinput2`, AX = A + `have_xdbe`, AG3 = A + `use_gtk` + `have_gtk3`,
AXF = A + `have_xfixes` (slice 100's) and **AWS = A + `have_window_system`** (slice 90's, the arm
this slice's reading splits on).  Both sides were measured on this revision (the baseline by
restoring `src/xterm.rs` from HEAD, then putting the saved copy back - its md5 is the copy's).

| arm | HEAD's unit (slice 101 text) | this slice's text | delta |
| --- | --- | --- | --- |
| A `have_x_windows` | 807 / 74 | **810 / 74** | **+3 / 0** |
| U `+use_toolkit_scroll_bars` | 836 / 74 | **839 / 74** | **+3 / 0** |
| UT `U +use_x_toolkit` | 822 / 73 | **825 / 73** | **+3 / 0** |
| UM `UT +use_motif` | 823 / 73 | **826 / 73** | **+3 / 0** |
| UG `U +use_gtk` | 819 / 75 | **822 / 75** | **+3 / 0** |
| UGX `UG +have_xinput2` | 921 / 77 | **924 / 77** | **+3 / 0** |
| AE `A +enable_checking` | 839 / 73 | **842 / 73** | **+3 / 0** |
| AI `A +have_x_i18n` | 815 / 75 | **818 / 75** | **+3 / 0** |
| AIU `AI +use_gtk` | 854 / 75 | **857 / 75** | **+3 / 0** |
| AIUX `AIU +have_xinput2` | 960 / 77 | **963 / 77** | **+3 / 0** |
| AX `A +have_xdbe` | 821 / 74 | **824 / 74** | **+3 / 0** |
| AG3 `A +use_gtk +have_gtk3` | 857 / 74 | **860 / 74** | **+3 / 0** |
| AXF `A +have_xfixes` | 820 / 74 | **823 / 74** | **+3 / 0** |
| **AWS** `A +have_window_system` | 501 / 0 | **503 / 0** | **+2 / 0** |

**Additions and retirements, by name.**  The thirteen arms: `cannot find value GCLineWidth` x2 (C
25882/25885), `cannot find function XChangeGC` x1 (C 25882) and the retirement of
`cannot find function x_clip_to_row` x1 (its reference in the ported text).  AWS reads the same except
that C 25875's `w->phys_cursor_width` resolves - the mirror carries it under
`#[cfg (feature = "have_window_system")]` (`include/window.rs` 152-153), so one `E0609` row does not
exist there.  **Warning sets are identical to the baselines' in all fourteen arms** (AWS carries none
in either).

**Two legs of `build-tu`**: `src/xterm.rs` - exit 0 and `30 warnings emitted` on **both** sides,
identical sets (the frozen configuration has no window system, so neither function is compiled
there).

**Arm reachability** (P-86 7).  `let _: c_int = true;` after C 25835's `struct frame *f = ...` moves
**A 810 -> 811** with `E0308` 13 -> 14, anchored at `build/probes/xterm/_root.rs:28903:18`; removing
the line returns A to the table.

**What `--check` reads.**  **13 -> 11** as Decision 115 5 says, with the line-count row at **29,000**
against C's 33,125 (12.5% drift, was 28,874 / 12.8%).

**Not claimed.**  The frozen configuration compiles none of this file: no clipping was set and no
cursor was drawn; the readings above are compile readings of text this machine cannot build.

### Slice 103's readings (C 25905-26014), as landed and measured (2026-10-07)

Probe = `cargo run -q -p xtask -- probe src/xterm.rs /tmp/s103/<tag>.o --crate-type=lib
--emit=metadata -C panic=abort <cfg flags>`; A = `have_x_windows`, U = A +
`use_toolkit_scroll_bars`, UT = U + `use_x_toolkit`, UM = UT + `use_motif`, UG = U + `use_gtk`,
UGX = UG + `have_xinput2`, AE = A + `enable_checking`, AI = A + `have_x_i18n`, AIU = AI +
`use_gtk`, AIUX = AIU + `have_xinput2`, AX = A + `have_xdbe`, AG3 = A + `use_gtk` + `have_gtk3`,
AXF = A + `have_xfixes`, AWS = A + `have_window_system`.  Both sides were measured on this revision
(the baseline by restoring `src/xterm.rs` from HEAD, then putting the saved copy back - its md5 is
the copy's).

| arm | HEAD's unit (slice 102 text) | this slice's text | delta |
| --- | --- | --- | --- |
| A `have_x_windows` | 810 / 74 | **821 / 74** | **+11 / 0** |
| U `+use_toolkit_scroll_bars` | 839 / 74 | **850 / 74** | **+11 / 0** |
| UT `U +use_x_toolkit` | 825 / 73 | **836 / 73** | **+11 / 0** |
| UM `UT +use_motif` | 826 / 73 | **837 / 73** | **+11 / 0** |
| UG `U +use_gtk` | 822 / 75 | **833 / 75** | **+11 / 0** |
| UGX `UG +have_xinput2` | 924 / 77 | **935 / 77** | **+11 / 0** |
| AE `A +enable_checking` | 842 / 73 | **853 / 73** | **+11 / 0** |
| AI `A +have_x_i18n` | 818 / 75 | **829 / 75** | **+11 / 0** |
| AIU `AI +use_gtk` | 857 / 75 | **868 / 75** | **+11 / 0** |
| AIUX `AIU +have_xinput2` | 963 / 77 | **974 / 77** | **+11 / 0** |
| AX `A +have_xdbe` | 824 / 74 | **835 / 74** | **+11 / 0** |
| AG3 `A +use_gtk +have_gtk3` | 860 / 74 | **871 / 74** | **+11 / 0** |
| AXF `A +have_xfixes` | 823 / 74 | **834 / 74** | **+11 / 0** |
| **AWS** `A +have_window_system` | 503 / 0 | **510 / 0** | **+7 / 0** |

**Additions, by name.**  The thirteen arms: `no field phys_cursor_width on type window` x4
(C 25979/26004/26005/26009), `cannot find function window_box_left` x2 (the
`WINDOW_TEXT_TO_FRAME_PIXEL_X!` expansion), `cannot find type face_cache` x2 (the per-unit
`FRAME_FACE_CACHE`/`FACE_FROM_ID` copies naming a type the mirror gates behind
`dispextern_face_records`), `cannot find value GCGraphicsExposures` x1, `cannot find value
GCLineWidth` x1 and `cannot find function XChangeGC` x1 - every one of them a class the baseline
already carries rows of (3, 5 and 9 respectively for the last three).  AWS: the same minus the four
`phys_cursor_width` rows.  **Warning sets are identical to the baselines' in all fourteen arms**
(AWS carries none in either), and nothing retires in any arm.

**Two legs of `build-tu`**: `src/xterm.rs` - exit 0 and `30 warnings emitted` on **both** sides,
identical sets (the frozen configuration has no window system, so this function is not compiled
there).

**Arm reachability** (P-86 7).  `let _: c_int = true;` after C 25915's `struct frame *f = ...` moves
**A 821 -> 822** with `E0308` 13 -> 14, anchored at `build/probes/xterm/_root.rs:29072:18`; removing
the line returns A to the table.

**What `--check` reads.**  **11 -> 10** as Decision 116 5 says, with the line-count row at **29,181**
against C's 33,125 (11.9% drift, was 29,000 / 12.5%).

**Not claimed.**  The frozen configuration compiles none of this file: no cursor was drawn and no GC
was changed; the readings above are compile readings of text this machine cannot build.
### Slice 104's readings (C 26016-26096), as landed and measured (2026-10-07)

| arm | baseline | slice 104 | delta |
| --- | --- | --- | --- |
| A `have_x_windows` | 821 / 74 | **824 / 74** | **+3 / 0** |
| U `A +use_toolkit_scroll_bars` | 850 / 74 | **853 / 74** | **+3 / 0** |
| UT `U +use_x_toolkit` | 836 / 73 | **838 / 73** | **+2 / 0** |
| UM `UT +use_motif` | 837 / 73 | **839 / 73** | **+2 / 0** |
| UG `U +use_gtk` | 833 / 75 | **836 / 75** | **+3 / 0** |
| UGX `UG +have_xinput2` | 935 / 77 | **938 / 77** | **+3 / 0** |
| AE `A +enable_checking` | 853 / 73 | **856 / 73** | **+3 / 0** |
| AI `A +have_x_i18n` | 829 / 75 | **833 / 75** | **+4 / 0** |
| AIU `AI +use_gtk` | 868 / 75 | **872 / 75** | **+4 / 0** |
| AIUX `AIU +have_xinput2` | 974 / 77 | **978 / 77** | **+4 / 0** |
| AX `A +have_xdbe` | 835 / 74 | **838 / 74** | **+3 / 0** |
| AG3 `A +use_gtk +have_gtk3` | 871 / 74 | **874 / 74** | **+3 / 0** |
| AXF `A +have_xfixes` | 834 / 74 | **837 / 74** | **+3 / 0** |
| **AWS** `A +have_window_system` | 510 / 0 | **511 / 0** | **+1 / 0** |

**Additions, by name.**  The eleven arms that do not set `use_x_toolkit` read three rows:
`cannot find function, tuple struct or tuple variant XDefineCursor` x1 (C 26024),
`no field phys_cursor_type on type window` x1 (C 26051) and `no field phys_cursor_width on type
window` x1 (C 26083).  **UT** and **UM** read the last two only, because `use_x_toolkit` is what puts
this file's own `XDefineCursor` declaration (24997-25004) in scope.  **AI**, **AIU** and **AIUX** read
those three plus `cannot find function xic_set_preeditarea` x1 (C 26093, inside C 26091's
`#ifdef HAVE_X_I18N`).  **AWS** reads `XDefineCursor` only - `have_window_system` is where both
`phys_cursor_*` fields exist.  **Nothing retires in any arm**: the three functions are C's `static`s
and no landed text names them (the caller is `x_create_terminal`, C 30668+, which is not landed).

**Two legs of `build-tu`**: `src/xterm.rs` - exit 0 and `30 warnings emitted` on **both** sides,
identical sets (the frozen configuration has no window system, so none of this region is compiled
there).

**Arm reachability** (P-86 7).  `let _: c_int = true;` before C 26022's `if (!f->pointer_invisible
...)` moves **A 824 -> 825** with `E0308` 13 -> 14, anchored at
`build/probes/xterm/_root.rs:29212:18`; removing the line returns A to the table.

**What `--check` reads.**  **10 -> 7**: the three functions retire as missing items, and the one token
row the reading brings (`x_draw_window_cursor does not mention break, case, default, switch`) is
closed by the `docs/ports/macro-body-notes.tsv` row this slice adds.  The line-count row is
**29,302** against C's 33,125 (11.5% drift, was 29,181 / 11.9%).

**Not claimed.**  The frozen configuration compiles none of this file: no cursor was defined on any
frame, no area was cleared and no preedit area was set; the readings above are compile readings of
text this machine cannot build.

### Slice 105's readings (C 26104-26214), as landed and measured (2026-10-07)

| arm | baseline | slice 105 | delta |
| --- | --- | --- | --- |
| A `have_x_windows` | 824 / 74 | **823 / 74** | **-1 / 0** |
| U `A +use_toolkit_scroll_bars` | 853 / 74 | **852 / 74** | **-1 / 0** |
| UT `U +use_x_toolkit` | 838 / 73 | **837 / 73** | **-1 / 0** |
| UM `UT +use_motif` | 839 / 73 | **838 / 73** | **-1 / 0** |
| UG `U +use_gtk` | 836 / 75 | **837 / 75** | **+1 / 0** |
| UGX `UG +have_xinput2` | 938 / 77 | **939 / 77** | **+1 / 0** |
| AE `A +enable_checking` | 856 / 73 | **855 / 73** | **-1 / 0** |
| AI `A +have_x_i18n` | 833 / 75 | **832 / 75** | **-1 / 0** |
| AIU `AI +use_gtk` | 872 / 75 | **873 / 75** | **+1 / 0** |
| AIUX `AIU +have_xinput2` | 978 / 77 | **979 / 77** | **+1 / 0** |
| AX `A +have_xdbe` | 838 / 74 | **837 / 74** | **-1 / 0** |
| AG3 `A +use_gtk +have_gtk3` | 874 / 74 | **875 / 74** | **+1 / 0** |
| AXF `A +have_xfixes` | 837 / 74 | **836 / 74** | **-1 / 0** |
| **AWS** `A +have_window_system` | 511 / 0 | **510 / 0** | **-1 / 0** |
| **AXPM** `A +have_xpm` (new) | 824 / 74 | **824 / 74** | **0 / 0** |

**Retirements and additions, by name.**  All fifteen arms lose one row,
`cannot find value x_bitmap_icon` - landed text already named the function, so its definition is
what the reading was waiting for.  The five `use_gtk` arms (UG, UGX, AIU, AIUX, AG3) gain two:
`cannot find value gnu_xpm_bits` (the `#ifdef USE_GTK` arm's `xg_set_icon_from_xpm_data (f,
gnu_xpm_bits)`; `include/emacs-icon.rs` records that array as the header's dead text, so the name is
written and the arm records it, P-08) and, in the four arms that do not set `have_gtk3`,
`cannot find function, tuple struct or tuple variant GDK_WINDOW_XWINDOW` anchored in
`include/xterm.rs`:1043 (the GTK2 arm of `GDK_WINDOW_XID`, which the new `FRAME_X_WINDOW` uses are
the first to expand); AG3's second new row is instead `cannot find macro GDK_WINDOW_XID`, anchored at
`include/xterm.rs`:1060 (`GTK_WIDGET_TO_X_WIN`), because the mirror carries only that macro's
`not (feature = "have_gtk3")` arm.  **AXPM** - this slice's own fifteenth arm - loses the
`x_bitmap_icon` row and gains `gnu_xpm_bits` from the `#elif` half's
`x_create_bitmap_from_xpm_data (f, gnu_xpm_bits)`, exactly cancelling.  Nothing else moves, and
**`x_text_icon` retires a `--check` row without earning a single arm row** (no arm names it yet).

**Two legs of `build-tu`**: `src/xterm.rs` - exit 0 and `30 warnings emitted` on **both** sides,
identical sets (the frozen configuration has no window system, so none of this region is compiled
there).

**Arm reachability** (P-86 7).  `let _: c_int = true;` before C 26108's `if (FRAME_X_WINDOW (f) ==
0)` moves **A 823 -> 824** with `E0308` 13 -> 14, anchored at
`build/probes/xterm/_root.rs:29357:18`; removing the line returns A to the table.

**What `--check` reads.**  **7 -> 5**: both functions retire as missing items, and no token row
arrives (C's `#ifdef`/`#elif` are the port's `#[cfg]` blocks and the checker reads the function's
own body tokens, which match).  The line-count row is **29,493** against C's 33,125 (11.0% drift,
was 29,302 / 11.5%).

**Not claimed.**  The frozen configuration compiles none of this file: no icon was set on any frame,
neither an XBM nor an XPM bitmap was created, no mask was made and no WM icon name was written; the
GTK and XPM arms have never been compiled by any configuration in this tree (the frozen table
leaves `USE_GTK` and `HAVE_XPM` undef), so their two new rows are text readings too.

### Slice 106's readings (C 12702-13327), as landed and measured (2026-10-07)

| arm | baseline | slice 106 | delta |
| --- | --- | --- | --- |
| A `have_x_windows` | 823 / 74 | **830 / 74** | **+7 / 0** |
| U `A +use_toolkit_scroll_bars` | 852 / 74 | **859 / 74** | **+7 / 0** |
| UT `U +use_x_toolkit` | 837 / 73 | **845 / 73** | **+8 / 0** |
| UM `UT +use_motif` | 838 / 73 | **846 / 73** | **+8 / 0** |
| UG `U +use_gtk` | 837 / 75 | **843 / 75** | **+6 / 0** |
| UGX `UG +have_xinput2` | 939 / 77 | **948 / 77** | **+9 / 0** |
| AE `A +enable_checking` | 855 / 73 | **862 / 73** | **+7 / 0** |
| AI `A +have_x_i18n` | 832 / 75 | **839 / 75** | **+7 / 0** |
| AIU `AI +use_gtk` | 873 / 75 | **879 / 75** | **+6 / 0** |
| AIUX `AIU +have_xinput2` | 979 / 77 | **988 / 77** | **+9 / 0** |
| AX `A +have_xdbe` | 837 / 74 | **844 / 74** | **+7 / 0** |
| AG3 `A +use_gtk +have_gtk3` | 875 / 74 | **881 / 74** | **+6 / 0** |
| AXF `A +have_xfixes` | 836 / 74 | **843 / 74** | **+7 / 0** |
| **AWS** `A +have_window_system` | 510 / 0 | **515 / 0** | **+5 / 0** |
| AXPM `A +have_xpm` | 824 / 74 | **831 / 74** | **+7 / 0** |
| **AXKB** `A +have_xkb` (new) | 841 / 74 | **856 / 74** | **+15 / 0** |
| **AXR** `A +have_xrandr` (new) | 825 / 74 | **833 / 74** | **+8 / 0** |

**Additions, by name.**  Rows of classes the baseline already carries: `XWindowAttributes` x1,
`XGetWindowAttributes` x1, `XChangeProperty` x1 and `XSelectInput` x1 in every arm (the four Xlib
names no mirror of this unit's graph declares - the baseline carries 5/3/16/4 rows of them),
`handle_one_xevent` x1 in the twelve arms that compile the plain-X dispatch (the *other* `--check`
item this function calls), `xany` x1 (eleven arms - the GTK build has no `if` to read it in),
`r#type` x1 (sixteen arms), `last_monitor_attributes_list` x1 (six arms - the XRANDR/GTK-gated
`x_display_info` field of xterm.h 834, which this mirror does not carry) and `client_pointer_device`
x2 in UGX/AIUX.  **New names** (the third-party-SDK class, this ledger's Owed 0): `XtAppNextEvent`
x1 in UT/UM, `XIGetClientPointer` x1 in UGX/AIUX, and in AXKB `XkbSelectEvents` x1,
`XkbUseCoreKbd` x2, `XkbStateNotifyMask` x2, `supports_xkb` x1, `XkbGetState` x1 and
`XkbStateRec` x1.  **No warning moved** in any of the seventeen arms (kind sets compared, not
offsets).

**Two legs of `build-tu`**: `src/xterm.rs` - exit 0 and `30 warnings emitted` on **both** sides,
identical sets.

**P-87's two legs** (the mirror's `DEFER_SELECTIONS!` row changed): `check-mirror-dupes`
**16 duplicate declaration(s), 16 ratchet row(s), 0 failure(s)** and `tools/check-all-units.sh`
**`checked 83 unit(s), 0 failure(s)`** + **`checking 83 unit(s), 0 unit failure(s)`**.

**Arm reachability** (P-86 7).  `let _: c_int = true;` before C 12734's
`if (FRAME_DISPLAY_INFO (f)->untrusted)` moves **A 830 -> 831** with `E0308` 13 -> 14, anchored at
`build/probes/xterm/_root.rs:22111:18`; removing the line returns A to the table.

**What `--check` reads.**  **5 -> 4**: `x_dnd_begin_drag_and_drop` retires as a missing item and no
token row arrives (this slice's one omission, C 12852, was read back by the checker and fixed).
The line-count row is **30,214** against C's 33,125 (8.8% drift, was 29,493 / 11.0%).

**Not claimed.**  The frozen configuration compiles none of this file: no drag was started, no
selection was released, no event loop was entered and no `handle_one_xevent` was called; the XKB,
Xt, XInput2, GTK and XRANDR arms have never been compiled by any configuration in this tree (the
frozen table leaves those macros undefined), so their rows are text readings too.

### Slice 109's readings (C 30667-31838), as landed and measured (2026-10-07)

| arm | baseline | slice 109 | delta |
| --- | --- | --- | --- |
| A `have_x_windows` | 825 / 74 | **905 / 74** | **+80 / 0** |
| U `A +use_toolkit_scroll_bars` | 838 / 74 | **918 / 74** | **+80 / 0** |
| UT `U +use_x_toolkit` | 824 / 73 | **905 / 73** | **+81 / 0** |
| UM `UT +use_motif` | 825 / 73 | **906 / 73** | **+81 / 0** |
| UG `U +use_gtk` | 817 / 75 | **911 / 75** | **+94 / 0** |
| UGX `UG +have_xinput2` | 896 / 77 | **1002 / 77** | **+106 / 0** |
| AE `A +enable_checking` | 857 / 73 | **937 / 73** | **+80 / 0** |
| AI `A +have_x_i18n` | 834 / 75 | **929 / 75** | **+95 / 0** |
| AIU `AI +use_gtk` | 869 / 75 | **978 / 75** | **+109 / 0** |
| AIUX `AIU +have_xinput2` | 952 / 77 | **1073 / 77** | **+121 / 0** |
| AX `A +have_xdbe` | 839 / 74 | **920 / 74** | **+81 / 0** |
| AG3 `A +use_gtk +have_gtk3` | 871 / 74 | **962 / 74** | **+91 / 0** |
| AXF `A +have_xfixes` | 827 / 74 | **915 / 74** | **+88 / 0** |
| AXPM `A +have_xpm` | 826 / 74 | **906 / 74** | **+80 / 0** |
| AXKB `A +have_xkb` | 845 / 74 | **943 / 74** | **+98 / 0** |
| AXR `A +have_xrandr` | 826 / 74 | **912 / 74** | **+86 / 0** |

Both sides were measured in this revision: the baseline side by restoring `femacs/src/xterm.rs`
to `9411622e:femacs/src/xterm.rs` and putting it back (md5 `e43c6647db8369b990521fc6b4519df9` on
both sides of that swap), the slice's own file md5 `bec354a7a35280db33ccd7ad9bd76a98`.  **No
warning moved in any arm** (kind sets compared): the first reading brought exactly one,
`unreachable statement` after C 30851's `error` - C's own text, whose three following statements
are dead in the one arm that compiles it - and it is closed by gating C 30854-30859's tail to the
two toolkit arms, with that reason written at the gate.

**The seventeenth arm, AWS.**  This round does not carry it: the ledger's definition
(`A + have_window_system`, slice 90's) measured **885 / 74** with both features set, and the bare
`have_window_system` flag aborts early at **4 errors / 28 warnings**, so neither reproduces the
recorded 509 / 0.  The ledger owes the next round a decision on that arm's cfg set; the sixteen
above are the ones this file states in full.

**Two legs of `build-tu`** (`cargo run -q -p xtask -- build-tu src/xterm.c`, from `femacs/`):
**identical on both sides** - `aborting due to 2 previous errors; 28 warnings emitted`, the two
errors being the ungated `x_atom_refs` static's `def_x_atom_refs!`/`x_atom_ref` (a property of the
no-feature compile, not of this slice).

**What `--check` reads.**  **4 -> 2**: `x_term_init` retires as a missing item, leaving
`handle_one_xevent` and the pre-existing `x_get_wm_check_window does not mention false` row; the
line-count row is gone (31,682 against C's 33,125, 4.4% drift, was 30,404 / 8.2%).
`tools/check-citations.py` is **676 files / 0 drift / 0 range / 0 stale** after the new include
edge's citation was written as `lisp.h 27`.

**Additions, by name.**  The new rows are the E0425 class this unit has counted since slice 1, now
with this function's share: Xlib entry points no mirror of its graph declares (`XSelectInput`,
`XInternAtoms`, `XCreateFontCursor`, `XAllocColor`, `XCreateColormap`, `XCopyColormapAndFree`,
`XQueryFont`, `XSynchronize`, `XSupportsLocale`, `XGetDefault`, `XGrabServer`/`XUngrabServer`,
`XGetKeyboardControl`, `XAutoRepeatOn`/`XAutoRepeatOff`, `XrmPutLineResource`, `XFreeFontInfo`,
`ServerVendor`, `ConnectionNumber`, `DefaultRootWindow`, `ScreenOfDisplay`, `DefaultScreen`,
`DefaultColormapOfScreen`, `RootWindowOfScreen`, `DefaultVisualOfScreen`, `XVisualIDFromVisual`,
`XCreatePixmapFromBitmapData`); the toolkit, GTK, XCB and extension families only their own arms
compile (`XtOpenDisplay`, `XtSetLanguageProc`, `XtCallConverter`, `XSetLocaleModifiers`,
`XGetXCBConnection`, `xcb_*`, `XRender*`, `XComposite*`, `XShape*`, `XSync*`, `Xinerama*`, `Xdbe*`,
`XRR*`, `XFixes*`, `Xkb*`, `XI*`, `g_log_*`, `gdk_*`, `g_signal_connect`, `gtk_init`,
`gtk_rc_parse`, `fixup_locale`, `xg_*`); the types no mirror carries (`XKeyboardState`, `XColor`,
`XFontStruct`, `XrmValue`, `Font`, `XPointer`); the lisp-side names C reads bare here
(`allocate_kboard`, `kset_system_key_alist`, `terminal_list`, `current_kboard`, `initial_kboard`,
`IS_DAEMON`, `Qx`); and `x_load_resources`, which `xrdb.rs` defines and this unit declares at its
use site (P-72 rule 4).  **Two readings this slice exposes and does not close**: `gray_bits`,
`gray_width` and `gray_height` are the `src/bitmaps/gray.xbm` data file's constants - the gap
`femacs/src/xfns.rs` 97-98 and `docs/ports/xfns.md` already record, which this unit now shares; and
`EMACS_CLASS` is frame.h 1840's resource class, kept as C's bytes (0.1-19's eighth half) with the
reason at its declaration.

**Arm reachability (P-86 7).**  Not taken this round: the surface is a whole function, so every
arm's error count moved and a one-line injection's `E0308` anchor is the next round's to place.

**Not claimed.**  The frozen configuration compiles none of this file, so nothing here is
behaviour: no display was opened, no toolkit initialised, no XCB connection obtained, no atom
interned and no XIM opened.  Sixteen of the seventeen arms were measured; AWS is named above as the
one whose cfg set this ledger still owes.  What the slice adds is a text reading of C's own
statements in C's own order.

**Follow-up: the two comments that recited C 31017's literal (status 3.911).**  The reported
file md5 above is the one the slice landed with.  Its runtime value is this repository's own name
(`let title: &CStr = c"femacs";`), but its two comments recited C's own line with the product name
spelled out - the `title` class the declaration gate fails outright, citations included - so
`tools/check-declarations.py` read `20 declaration sites (2 forbidden, 18 allowed literals),
2 failures` after `29fa36b3`, both findings being these comments (this file's 19955 and 20340).
They now quote that line with the `<PACKAGE_NAME>` placeholder (the `src/emacs.rs` `RCS_Id` form,
status 3.73) with the reason on the line, and the reading is back to `18 declaration sites
(0 forbidden, 18 allowed literals), 0 failures`.  Comment lines only: no statement, no order and
no value moved; the file grows by two comment lines (31,682 -> 31,684) and its md5 moves from
`bec354a7a35280db33ccd7ad9bd76a98` to `9bdd5776783669aa980fc0b5fb311147`.

## Stale-box audit (2026-10-08, status 3.953)

Thirteen `- [ ]` rows in this ledger named records that `femacs/src/xterm.rs` already defines as
`pub struct` items: `x_atom_ref`, `x_selection_request_event`, the sixteen-member
`x_client_list_window` and the ten `xm_*` drag/DND protocol records.  They are ticked now, and the
evidence is a field-by-field comparison of the two texts (a throwaway script: the C definition's
declarator names from `emacs/src/xterm.c` against the Rust record's `pub <name>:` fields, both
sides with comments stripped and C's multi-declarator lines - `int x, y;` - split on commas):

- **Twelve of the thirteen agree field for field, in C's own order**: `x_atom_ref` (2),
  `x_selection_request_event` (2), `xm_drag_initiator_info` (4), `xm_drag_motion_message` (6),
  `xm_drag_motion_reply` (6), `xm_drag_receiver_info` (7), `xm_drop_start_message` (8),
  `xm_drop_start_reply` (5), `xm_targets_table_header` (4), `xm_targets_table_rec` (2),
  `xm_top_level_enter_message` (6) and `xm_top_level_leave_message` (5).
- **`x_client_list_window` agrees once C's own conditional fields are counted**: C 1511-1526's
  `#ifdef HAVE_XSHAPE` region holds `border_width` and the two rect pairs, and the port carries
  them under `#[cfg (feature = "have_xshape")]` (this unit's 1011-1025), so the Rust record has the
  two extra field names an unconditional reading of C's text would not see.  `have_xshape` is not
  in `femacs/xtask/Cargo.toml`'s feature table - the ledger's `## Owed by the integrator` class.

**`x_initialized` (the fourteenth stale box).**  The same round checked the two rows it left:
`x_initialized` is ported - `femacs/src/xterm.rs` 19738's `pub static mut x_initialized: bool =
false;` (C 30491's tentative `static bool`, its initial value C's zero by NIL_IS_ZERO's rule) with
its two readers at 20007-20011 (C 30704-30709) - so its box is ticked too; the earlier paragraph
that named it alongside `handle_one_xevent` was wrong about this row and is corrected here.

**What this audit does not close.**  One row is left unticked and it is the whole of the file's
remainder: **`handle_one_xevent`**, C 18875's `static int` and C 18876-25811's body - **6,936
lines** of C, the largest single function in the reference and the reason this unit's
`extract-abi.py --check` reports exactly one `missing function` row and no other.  A function is
one Rust item, so it cannot be sliced across commits the way this ledger's files are: the port has
to land it whole, in one change with its own arm readings, which makes it the next round's work
rather than this one's.

### Slice 110's readings (C 18866-18949 and C 25596-25632), as landed and measured (2026-10-08)

`handle_one_xevent`'s prologue and tail: the declaration block, C 18918-18919's abort, the
`inev` initialisation, the `any` computation and C 18949's `switch` head, plus C 25596-25631's
`do_help` epilogue and `return count`.  The thirty `case` bodies are the slices that follow.

| arm | baseline | slice 110 | delta |
| --- | --- | --- | --- |
| A `have_x_windows` | 924 / 74 | **931 / 75** | **+7 / +1** |
| U `A +use_toolkit_scroll_bars` | 937 / 74 | **944 / 75** | **+7 / +1** |
| UT `U +use_x_toolkit` | 924 / 73 | **931 / 74** | **+7 / +1** |
| UM `UT +use_motif` | 925 / 73 | **932 / 74** | **+7 / +1** |
| UG `U +use_gtk` | 930 / 75 | **939 / 76** | **+9 / +1** |
| UGX `UG +have_xinput2` | 1021 / 77 | **1033 / 78** | **+12 / +1** |
| AE `A +enable_checking` | 956 / 73 | **963 / 74** | **+7 / +1** |
| AI `A +have_x_i18n` | 948 / 75 | **955 / 76** | **+7 / +1** |
| AIU `AI +use_gtk` | 997 / 75 | **1006 / 76** | **+9 / +1** |
| AIUX `AIU +have_xinput2` | 1092 / 77 | **1104 / 78** | **+12 / +1** |
| AX `A +have_xdbe` | 934 / 74 | **941 / 75** | **+7 / +1** |
| AG3 `A +use_gtk +have_gtk3` | 981 / 74 | **990 / 75** | **+9 / +1** |
| AXF `A +have_xfixes` | 934 / 74 | **941 / 75** | **+7 / +1** |
| AXPM `A +have_xpm` | 925 / 74 | **932 / 75** | **+7 / +1** |
| AXKB `A +have_xkb` | 962 / 74 | **969 / 75** | **+7 / +1** |
| AXR `A +have_xrandr` | 931 / 74 | **938 / 75** | **+7 / +1** |

**Both sides were measured in this revision** (the lane's rule): the baseline side by restoring
`femacs/src/xterm.rs` to `f3a3c531:femacs/src/xterm.rs` and re-materialising its `_root.rs`, the
slice's own file md5 `c2f4ef3d2a64307f0e9818bb6a173932` (measured after every edit below).
**The `of which`, arm A, by message identity** (the two dumps diffed): **new** - `no field
r#type` on the `XEvent` stand-in x3 (C 18932/18935/18946), `no field xany` x1 (C 18937),
`cannot find value help_echo_string` x2 (C 25616/25621), `cannot find function gen_help_event` x2
(C 25616/25622), and `help_echo_window`/`help_echo_object`/`help_echo_pos` x1 each (C 25616-25617)
- thirteen rows with the trailer line; **retired** - the four `cannot find function
handle_one_xevent` rows above (their call sites now resolve).  11 - 4 = the +7 the table shows.
`LASTEvent`, `XComposeStatus`, `composite_status` and the new `not_generic` condition resolve
completely (0 rows each), and the arms that compile the toolkit arms gain two rows
(`GdkEvent`/`GdkDisplay`/`gdk_x11_lookup_xdisplay`, C 18905-18906) or three (`xi_device_t`,
`xi_handle_interaction`, C 18884/C 25612), which is the +9/+12 the table shows.

**Repairs the first writes asked for** (each measured, each fixed in place): a stray C `#endif`
line copied into the port's text (`expected one of ! or [, found endif`); C 18911-18913's
`USE_SAFE_ALLOCA`/C 25628's `SAFE_FREE` need the port's macros' arguments (`SAFE_FREE (sa_count)`,
`unexpected end of macro invocation` without it); C 18935/18939's `if`/`else` pair and the two
`#ifdef`'d assignments are **statement pairs** - an attribute on the `else` or on an assignment
expression is `attributes on expressions are experimental` (E0658), so C 18933-18942 becomes one
cfg-computed `not_generic` condition with one copy of C 18937 (Decision 2 item 2) and C 18927's
assignment is wrapped in a block; and C 18937's `x_any_window_to_frame` is the *macro* in the
non-toolkit arms, so the call is the cfg'd pair 25731/25733 spells (`x_any_window_to_frame!`).

**The warning column moves +1 in every arm** (`variable does not need to be mutable`), and its
reported site is `include/xterm.rs` 1657 (a macro expansion's binding), not this slice's own text
- named here, not silenced.  The locals C's remaining arms write (`nbytes`, `mouse_frame`,
`coding`, `configureEvent`, `next_event`, `dx`, `dy`, `do_help`'s `mut`) are read only there and
are the same one row.

**Not claimed**: no runtime evidence - the events the `case` blocks handle never arrive (this lane
does not link, and the unit's `XEvent` members the slice reads are still owed mirrors); the
readings are the sixteen arm compiles, the `--check` row's new shape and this ledger.

### Slice 111's readings (C 19473-19561), as landed and measured (2026-10-08)

`handle_one_xevent`'s two labels and its first three `case` arms: C 25558-25587's `OTHER:` block and
C 25590-25595's `done:` block, then `SelectionNotify` (C 19473-19486), `SelectionClear`
(C 19487-19515) and `SelectionRequest` (C 19516-19561).  Twenty-seven `case` bodies are the slices
that follow.

| arm | baseline | slice 111 | delta |
| --- | --- | --- | --- |
| A `have_x_windows` | 931 / 75 | **948 / 77** | **+17 / +2** |
| U `A +use_toolkit_scroll_bars` | 944 / 75 | **961 / 77** | **+17 / +2** |
| UT `U +use_x_toolkit` | 931 / 74 | **955 / 75** | **+24 / +1** |
| UM `UT +use_motif` | 932 / 74 | **956 / 75** | **+24 / +1** |
| UG `U +use_gtk` | 939 / 76 | **958 / 77** | **+19 / +1** |
| UGX `UG +have_xinput2` | 1033 / 78 | **1054 / 79** | **+21 / +1** |
| AE `A +enable_checking` | 963 / 74 | **980 / 76** | **+17 / +2** |
| AI `A +have_x_i18n` | 955 / 76 | **972 / 78** | **+17 / +2** |
| AIU `AI +use_gtk` | 1006 / 76 | **1025 / 77** | **+19 / +1** |
| AIUX `AIU +have_xinput2` | 1104 / 78 | **1125 / 79** | **+21 / +1** |
| AX `A +have_xdbe` | 941 / 75 | **958 / 77** | **+17 / +2** |
| AG3 `A +use_gtk +have_gtk3` | 990 / 75 | **1009 / 76** | **+19 / +1** |
| AXF `A +have_xfixes` | 941 / 75 | **958 / 77** | **+17 / +2** |
| AXPM `A +have_xpm` | 932 / 75 | **949 / 77** | **+17 / +2** |
| AXKB `A +have_xkb` | 969 / 75 | **986 / 77** | **+17 / +2** |
| AXR `A +have_xrandr` | 938 / 75 | **955 / 77** | **+17 / +2** |
| **AX11** `A +have_x11` (this slice's, the seventeenth) | 931 / 75 | **932 / 77** | **+1 / +2** |

**Both sides were measured in this revision**.  The baseline column is HEAD (`5070c250`, slice 110),
re-measured here for A, UT, UG, UGX and AX11 (931/75, 931/74, 939/76, 1033/78, 931/75 - the first
four are the ledger's own slice-110 numbers to the digit) and the slice's own file md5
`5b27bbc2c16fbd198a452f715dd103bc`.  `AX11`'s baseline is A's because on HEAD's text `have_x11`
enables nothing (`sie` stood behind `have_pgtk`): the arm exists to read what *this* slice's mirror
change made reachable.

**The `of which`, by name and by arm class** (each arm's dump diffed against its own baseline):

| rows | A-class (A U AE AI AX AXF AXPM AXKB AXR) | UT/UM | UG/AIU/AG3 | UGX/AIUX | AX11 |
| --- | --- | --- | --- | --- | --- |
| `no field sie` (C 19503-19510, C 19527-19541) | +13 | +13 | +13 | +13 | **-3** |
| `no field xselection` (C 19475/19484) | +1 | +2 | +2 | +2 | +1 |
| `no field xselectionclear` (C 19489/19496) | +1 | +2 | +2 | +2 | +1 |
| `no field xselectionrequest` (C 19518/19525) | +1 | +2 | +1 | +1 | +1 |
| `cannot find function x_handle_selection_notify` (C 19484) | +1 | +1 | +1 | +1 | +1 |
| the `OTHER:` block's own rows (C 25560-25587) | 0 | +4 | 0 | +2 | 0 |
| total | +17 | +24 | +19 | +21 | +1 |

* The thirteen `sie` rows are one per use site (two of them are the `++`/`=` pair inside
  `selection_input_event_bits!`'s setter arm, which names its argument twice) and are the class the
  mirror change above answers: AX11 reads **-3**, because the three `sie` uses the unit already
  carried elsewhere (`x_preserve_selections`, C 32611) resolve there too.
* The `xselection*` rows are the `XEvent` stand-in's owed members (the same class slice 110's
  `r#type`/`xany` rows are).  The toolkit and GTK arms read each of the first two **twice** because
  C 19475/19489's `if` is theirs and C 19496's local is every arm's; `SelectionRequest`'s `if` is
  `USE_X_TOOLKIT` alone, so its member arrives once in the GTK arms.
* The `OTHER:` block's rows are only in the arms that compile it: `no field xconfigure` x2
  (C 25566-25567 - the block's own `width`/`height`), `no field r#type` x1 (C 25565) and
  `cannot find function XtDispatchEvent` x1 (C 25570/25572) in UT/UM, plus
  `cannot find function gdk_event_free` / `gtk_main_do_event` x1 each (C 25580/25585) in the
  `use_gtk + have_xinput2` arms.  AG3's fourteen `xconfigure` rows and the twelve of the other
  arms are the baselines' own and do not move.
* The warning column moves **+2 in every arm without a toolkit and +1 in every arm with one**,
  and every moved row is `unused label`: `'other` is unused where the three `break 'other` sites
  are all behind `use_x_toolkit`/`use_gtk`, and `'done` is unused until a `goto done` site lands
  (the seventy-six of them are in the `case` bodies to come).  Nothing is silenced; the labels are
  what makes the twenty-seven remaining arms expressible.

**What `--check` reads** (unchanged in shape, and the token list is the reading): still two rows,
`function handle_one_xevent does not mention <the remaining 484 tokens>` and
`function x_get_wm_check_window does not mention false`; the long row's list went **531 -> 484**.
Unit **32,019 lines** against C's 33,125 (**3.34%** drift, was 3.8%, status 4.026).

**One repair the first writing read** that is not in the code: `docs/baseline/feature-coverage.tsv`
was regenerated (`python3 tools/feature-coverage.py --write`) because the mirror's gate is one of
its rows; the write also carried the drift the table had accumulated since its last write
(96 insertions / 68 deletions: `have_x_windows` 2,276 sites -> 3,181, `have_pgtk` 1,019 -> 1,506,
and so on), and the reading this slice needs is inside it: `have_x11` **7 sites / 2 files -> 8 / 3**.

**Not claimed**: no runtime evidence - the events these arms handle were never delivered (this lane
does not link), and the two runtime rows the arms now name (`x_handle_selection_notify`, the
`XSelectionClearEvent`/`XSelectionRequestEvent` records) are still C's text, not a measured
behaviour.  The readings are the seventeen arm compiles, the `--check` row's new token count and
this ledger.

### Slice 112's readings (C 19562-19862), as landed and measured (2026-10-08)

`case PropertyNotify:` (C 19562-19804) and `case ReparentNotify:` (C 19805-19862), plus the lane's
eighteenth arm `AUX = A + have_x_windows + use_xcb`, which is the arm C 19684-19778's two `USE_XCB`
branches needed.  Twenty-five `case` bodies remain.

| arm | baseline | slice 112 | delta |
| --- | --- | --- | --- |
| A `have_x_windows` | 948 / 77 | **975 / 76** | **+27 / -1** |
| U `A +use_toolkit_scroll_bars` | 961 / 77 | **988 / 76** | **+27 / -1** |
| UT `U +use_x_toolkit` | 955 / 75 | **982 / 75** | **+27 / 0** |
| UM `UT +use_motif` | 956 / 75 | **983 / 75** | **+27 / 0** |
| UG `U +use_gtk` | 958 / 77 | **983 / 77** | **+25 / 0** |
| UGX `UG +have_xinput2` | 1054 / 79 | **1079 / 79** | **+25 / 0** |
| AE `A +enable_checking` | 980 / 76 | **1007 / 75** | **+27 / -1** |
| AI `A +have_x_i18n` | 972 / 78 | **999 / 77** | **+27 / -1** |
| AIU `AI +use_gtk` | 1025 / 77 | **1050 / 77** | **+25 / 0** |
| AIUX `AIU +have_xinput2` | 1125 / 79 | **1150 / 79** | **+25 / 0** |
| AX `A +have_xdbe` | 958 / 77 | **985 / 76** | **+27 / -1** |
| AG3 `A +use_gtk +have_gtk3` | 1009 / 76 | **1036 / 76** | **+27 / 0** |
| AXF `A +have_xfixes` | 958 / 77 | **985 / 76** | **+27 / -1** |
| AXPM `A +have_xpm` | 949 / 77 | **976 / 76** | **+27 / -1** |
| AXKB `A +have_xkb` | 986 / 77 | **1013 / 76** | **+27 / -1** |
| AXR `A +have_xrandr` | 955 / 77 | **982 / 76** | **+27 / -1** |
| AX11 `A +have_x11` | 932 / 77 | **959 / 76** | **+27 / -1** |
| **AUX** `A +use_xcb` (this slice's, the eighteenth) | 1 error, *no type checking* | **1058 / 74** | — |

**Both sides were measured in this revision**: the baseline is HEAD (`fd556ccb`, slice 111) with
its own `_root.rs` re-materialised per arm, and the slice's file md5 is
`fbc0d1e591673668ae46e1489ca84922`.  The seventeen baseline numbers reproduce the ledger's slice-111
column to the digit (A 948/77, UT 955/75, UG 958/77, UGX 1054/79, AX 958/77, AG3 1009/76, AX11
932/77), which is what says the two revolutions are the same text.

**The `of which`, by name**, arm A (the other non-GTK arms read the same set):

| rows | C sites |
| --- | --- |
| `no field xproperty` +16 | C 19565, 19572, 19579, 19598, 19615, 19621, 19622, 19626, 19668, 19682, 19699, 19781, 19782, 19783, 19798, 19801 |
| `no field xreparent` +2 | C 19806 `window`, C 19828 `parent` |
| `no field explicit_parent` +4 | C 19624, C 19669 and C 19811 (`FRAME_X_EMBEDDED_P`'s three expansions) + the write at C 19818 |
| `no field has_been_visible` +2 | C 19641 (write), C 19844 (read) |
| `no field alpha_identical_p` +1 | C 19681 |
| `cannot find function XDeleteProperty` +1 | C 19821 |
| `cannot find function XGetGeometry` +1 | C 19853 |

* The two `XEvent` members (`xproperty`, `xreparent`) are the `include/xlib.rs` stand-in's owed
  members - the class slices 110 and 111 carry.  The five `x_output` rows are a *different* debt,
  named in the mirror itself: include/xterm.rs 821-928 records `hourglass_p`, `explicit_parent`,
  `asked_for_visible`, `has_been_visible`, `wait_for_wm` and `alpha_identical_p` as `// C:` lines
  rather than fields because that record cannot be compiled on this target, and the
  `FRAME_X_EMBEDDED_P` macro (1740) already reads one of them the same way.  Nothing is silenced.
* The `use_gtk` arms read **+25**: C 19809-19827 and C 19838-19846's two `#ifndef USE_GTK` blocks
  are off there, which takes away one `XDeleteProperty`, one `explicit_parent` and one
  `has_been_visible` - and adds two `GDK_WINDOW_XWINDOW` rows, because `FRAME_OUTER_WINDOW`'s GTK
  expansion is what C 19709's and C 19853's two uses reach in those arms (**AG3** reads
  `GDK_WINDOW_XID` x2 for the same two sites instead, plus `FRAME_PIXEL_WIDTH`/`_HEIGHT` x1 each
  from the `HAVE_GTK3` block C 19647-19651, which only that arm compiles).
* **Warnings go -1 in every arm without a toolkit and hold in every arm with one**: the row is
  `unused label`, because `break 'other` now has users in the plain-X arms too (`'other` had none
  there while only slice 111's three GTK/toolkit-gated sites existed).  `'done` still waits for the
  76 `goto done` sites.
* **AUX's baseline is an abort, not a reading.**  On HEAD's text the arm dies at macro expansion
  (`unexpected end of macro invocation`, one error, `{ SAFE_FREE! (); }` at C 3607) and rustc never
  type-checks the unit, so `1` and `1058` are not comparable: the second is the first reading this
  arm has ever produced.  Its rows are the file's owed classes (164 `xclient`, 62 `mismatched
  types`, 45 `XSetForeground`, …), and **none of them is this slice's text**.

**Not claimed**: no runtime evidence - this lane does not link, and no event in these two arms was
ever delivered; the readings are the eighteen arm compiles, the `--check` row's token count and
this ledger.  The `x_output` bitfield rows and the `XEvent` member rows are debts this slice
*names*, not repairs it makes: the first belongs to the include/xterm.rs mirror's own round and the
second to include/xlib.rs's owed-member list.
