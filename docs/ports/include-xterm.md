# emacs/src/xterm.h (2045 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 3
- [x] SELECTION_EVENT_DISPLAY  (line 1715)
- [x] x_make_truecolor_pixel  (line 1867)
- [x] x_mutable_colormap  (line 1887)

## Macros defined — 68
- [x] BLACK_PIX_DEFAULT  (line 137, body c7664084bf2b)
- [x] DEFER_SELECTIONS  (line 1851, body 24b9b2d53501)
- [x] FRAME_BASELINE_OFFSET  (line 1478, body dd98cb94d2d8)
- [x] FRAME_DISPLAY_INFO  (line 1481, body 0f2cf19a0d0e)
- [x] FRAME_FONT  (line 1467, body 2325950edbe3)
- [x] FRAME_FONTSET  (line 1468, body 34577d48893a)
- [x] FRAME_MENUBAR_HEIGHT  (line 1464, body 51f4e90e95dd)
- [x] FRAME_NATIVE_WINDOW  (line 1400, body 2cf58174594c)
- [x] FRAME_OUTER_WINDOW  (line 1457, body 2cf58174594c)
- [x] FRAME_OUTPUT_DATA  (line 1396, body 28d2d4b857b8)
- [x] FRAME_TOOLBAR_BOTTOM_HEIGHT  (line 1470, body 04e9cd73a808)
- [x] FRAME_TOOLBAR_HEIGHT  (line 1472, body 41fd73a11b31)
- [x] FRAME_TOOLBAR_LEFT_WIDTH  (line 1474, body d36829acedc7)
- [x] FRAME_TOOLBAR_RIGHT_WIDTH  (line 1475, body b3ce99443b6b)
- [x] FRAME_TOOLBAR_TOP_HEIGHT  (line 1469, body bcad76e82890)
- [x] FRAME_TOOLBAR_WIDTH  (line 1476, body b1cbc1bb0f62)
- [x] FRAME_XIC  (line 1529, body 9e4dfd50835f)
- [x] FRAME_XIC_FONTSET  (line 1533, body d7f2ac6c068f)
- [x] FRAME_XIC_STYLE  (line 1532, body c33c1ca52aab)
- [x] FRAME_X_COLORMAP  (line 1527, body 83d39ee06198)
- [x] FRAME_X_DISPLAY  (line 1484, body 986cf8910640)
- [x] FRAME_X_DRAWABLE  (line 1411, body 0a4030f35451)
- [x] FRAME_X_EMBEDDED_P  (line 2035, body 14b3568a5bf0)
- [x] FRAME_X_OUTPUT  (line 1395, body 7102c06ac864)
- [x] FRAME_X_RAW_DRAWABLE  (line 1403, body c26f2ef3c8bb)
- [x] FRAME_X_SCREEN  (line 1487, body a6e03de2dfeb)
- [x] FRAME_X_SCREEN_NUMBER  (line 1490, body 4ff144256fce)
- [x] FRAME_X_VISIBLE  (line 1500, body 43cc263c6c96)
- [x] FRAME_X_VISUAL  (line 1493, body 1918ba69fa41)
- [x] FRAME_X_VISUAL_INFO  (line 1496, body f7aaf658c0ba)
- [x] FRAME_X_WINDOW  (line 1399, body 90d0b9054736)
- [x] FRAME_X_XIM  (line 1530, body f73793c8fc5e)
- [x] FRAME_X_XIM_CODING  (line 1534, body b5fafacab5a9)
- [x] FRAME_X_XIM_STYLES  (line 1531, body 5d893a72a18b)
- [x] GTK_CHECK_VERSION  (line 74, body 7cb6efb98ba5)
- [x] HORIZONTAL_SCROLL_BAR_BOTTOM_BORDER  (line 1690, body 0902ac4af971)
- [x] HORIZONTAL_SCROLL_BAR_INSIDE_HEIGHT  (line 1650, body 52855ff9774e)
- [x] HORIZONTAL_SCROLL_BAR_INSIDE_WIDTH  (line 1667, body 3cdb9bf7412f)
- [x] HORIZONTAL_SCROLL_BAR_LEFT_BORDER  (line 1687, body 0902ac4af971)
- [x] HORIZONTAL_SCROLL_BAR_LEFT_RANGE  (line 1662, body 10d24a511ceb)
- [x] HORIZONTAL_SCROLL_BAR_MIN_HANDLE  (line 1694, body 28bd264a2903)
- [x] HORIZONTAL_SCROLL_BAR_RIGHT_BORDER  (line 1688, body 0902ac4af971)
- [x] HORIZONTAL_SCROLL_BAR_TOP_BORDER  (line 1689, body 0902ac4af971)
- [x] N_FAILABLE_REQUESTS  (line 95, body b4182bff4b3c)
- [x] SELECTION_EVENT_DPYINFO  (line 1719, body 126661311f2a)
- [x] SELECTION_EVENT_PROPERTY  (line 1728, body 01aaf797987f)
- [x] SELECTION_EVENT_REQUESTOR  (line 1722, body 304fe022a670)
- [x] SELECTION_EVENT_SELECTION  (line 1724, body fb2bd52434a9)
- [x] SELECTION_EVENT_TARGET  (line 1726, body 86cb8041714d)
- [x] SELECTION_EVENT_TIME  (line 1730, body 847ffccd26b8)
- [x] STANDARD_EVENT_SET  (line 146, body 63a73828dbda)
- [x] STORE_NATIVE_RECT  (line 2037, body ba4936021147)
- [x] VERTICAL_SCROLL_BAR_BOTTOM_BORDER  (line 1685, body 0902ac4af971)
- [x] VERTICAL_SCROLL_BAR_INSIDE_HEIGHT  (line 1645, body 36f98e3eabb8)
- [x] VERTICAL_SCROLL_BAR_INSIDE_WIDTH  (line 1628, body 643baf8bba30)
- [x] VERTICAL_SCROLL_BAR_LEFT_BORDER  (line 1682, body 0902ac4af971)
- [x] VERTICAL_SCROLL_BAR_MIN_HANDLE  (line 1693, body 28bd264a2903)
- [x] VERTICAL_SCROLL_BAR_RIGHT_BORDER  (line 1683, body 0902ac4af971)
- [x] VERTICAL_SCROLL_BAR_TOP_BORDER  (line 1684, body 0902ac4af971)
- [x] VERTICAL_SCROLL_BAR_TOP_RANGE  (line 1640, body 3d518bbbd4a8)
- [x] WHITE_PIX_DEFAULT  (line 139, body 3cc5eab2e5f5)
- [x] XSCROLL_BAR  (line 1605, body aa568fbbbc66)
- [x] XTERM_H  (line 21, body da39a3ee5e6b)
- [x] X_LONG_MAX  (line 1333, body 9a43d9d1386f)
- [x] X_LONG_MIN  (line 1334, body d857dfb69dcb)
- [x] X_SHRT_MAX  (line 1331, body 544606a93d58)
- [x] X_SHRT_MIN  (line 1332, body 3012d7890a5f)
- [x] X_ULONG_MAX  (line 1335, body 378d5d0c3b9b)

## Types and their fields — 7
- [x] color_name_cache_entry: struct color_name_cache_entry* next, XColor rgb, char* name, bool_bf valid : 1
- [x] scroll_bar: union vectorlike_header header, Lisp_Object window, Lisp_Object next, Lisp_Object prev, Window x_window, int top, int left, int width, int height, int start, int end, int dragging, bool horizontal
- [x] selection_input_event: ENUM_BF (event_kind) kind : EVENT_KIND_WIDTH, struct x_display_info* dpyinfo, Window requestor, Atom selection, Atom target, Atom property, Time time
- [x] x_bitmap_record: Pixmap pixmap, bool have_mask, Pixmap mask, char* file, int refcount, int height, int width, int depth
- [x] x_display_info: struct x_display_info* next, struct terminal* terminal, Display* display, int connection, Lisp_Object name_list_element, int reference_count, bool untrusted, Screen* screen, double resx, double resy, Visual* visual, XVisualInfo visual_info, Colormap cmap, int n_planes, int grabbed, ptrdiff_t icon_bitmap_id, Window root_window, Window client_leader_window, Cursor vertical_scroll_bar_cursor, Cursor horizontal_scroll_bar_cursor, Cursor invisible_cursor, XrmDatabase rdb, int smallest_char_width, int smallest_font_height, GC scratch_cursor_gc, Mouse_HLInfo mouse_highlight, unsigned x_id, char* x_id_name, int n_fonts, struct x_bitmap_record* bitmaps, ptrdiff_t bitmaps_size, ptrdiff_t bitmaps_last, unsigned int meta_mod_mask, unsigned int shift_lock_mask, unsigned alt_mod_mask, unsigned super_mod_mask, unsigned hyper_mod_mask, Atom Xatom_wm_protocols, Atom Xatom_wm_take_focus, Atom Xatom_wm_save_yourself, Atom Xatom_wm_delete_window, Atom Xatom_wm_change_state, Atom Xatom_wm_state, Atom Xatom_wm_configure_denied, Atom Xatom_wm_window_moved, Atom Xatom_wm_client_leader, Atom Xatom_wm_transient_for, Atom Xatom_editres, Atom Xatom_CLIPBOARD, Atom Xatom_TIMESTAMP, Atom Xatom_TEXT, Atom Xatom_DELETE, Atom Xatom_COMPOUND_TEXT, Atom Xatom_UTF8_STRING, Atom Xatom_MULTIPLE, Atom Xatom_INCR, Atom Xatom_EMACS_TMP, Atom Xatom_TARGETS, Atom Xatom_NULL, Atom Xatom_ATOM, Atom Xatom_ATOM_PAIR, Atom Xatom_CLIPBOARD_MANAGER, Atom Xatom_EMACS_SERVER_TIME_PROP, Atom Xatom_PIXEL_SIZE, Atom Xatom_AVERAGE_WIDTH, Atom Xatom_MULE_BASELINE_OFFSET, Atom Xatom_MULE_RELATIVE_COMPOSE, Atom Xatom_MULE_DEFAULT_ASCENT, Atom Xatom_DONE, Atom Xatom_PAGE, Atom Xatom_Scrollbar, Atom Xatom_Horizontal_Scrollbar, Atom Xatom_XEMBED, Atom Xatom_XEMBED_INFO, Atom Xatom_NET_WM_CM_Sn, Atom Xatom_MOTIF_WM_HINTS, Atom Xatom_MOTIF_DRAG_WINDOW, Atom Xatom_MOTIF_DRAG_TARGETS, Atom Xatom_MOTIF_DRAG_AND_DROP_MESSAGE, Atom Xatom_MOTIF_DRAG_INITIATOR_INFO, Atom Xatom_MOTIF_DRAG_RECEIVER_INFO, Atom Xatom_EMACS_DRAG_ATOM, Atom Xatom_XmTRANSFER_SUCCESS, Atom Xatom_XmTRANSFER_FAILURE, Atom Xatom_DndProtocol, Atom Xatom_DND_PROTOCOL, Atom Xatom_text_plain_charset_utf_8, Atom Xatom_LENGTH, Atom Xatom_FILE_NAME, Atom Xatom_CHARACTER_POSITION, Atom Xatom_LINE_NUMBER, Atom Xatom_COLUMN_NUMBER, Atom Xatom_OWNER_OS, Atom Xatom_HOST_NAME, Atom Xatom_USER, Atom Xatom_CLASS, Atom Xatom_NAME, Atom Xatom_SAVE_TARGETS, struct frame* x_focus_frame, struct frame* x_focus_event_frame, struct frame* highlight_frame, struct frame* x_pending_autoraise_frame, struct frame* last_mouse_frame, struct frame* last_mouse_glyph_frame, struct frame* last_mouse_motion_frame, struct scroll_bar* last_mouse_scroll_bar, Time last_user_time, int last_mouse_motion_x, int last_mouse_motion_y, XRectangle last_mouse_glyph, Time last_mouse_movement_time, bool last_mouse_movement_time_send_event, Pixmap gray, struct color_name_cache_entry** color_names, ptrdiff_t* color_names_length, int color_names_size, XColor* color_cells, int ncolor_cells, int red_bits, int blue_bits, int green_bits, int alpha_bits, int red_offset, int blue_offset, int green_offset, int alpha_offset, unsigned long alpha_mask
- [x] x_failable_request: unsigned long start, unsigned long end, unsigned int selection_serial
- [x] x_output: int toolbar_top_height, int toolbar_bottom_height, int toolbar_left_width, int toolbar_right_width, Pixmap border_tile, GC normal_gc, GC reverse_gc, GC cursor_gc, Window window_desc, Drawable draw_desc, Window icon_desc, Window parent_desc, Window user_time_window, ptrdiff_t icon_bitmap, struct font* font, int baseline_offset, int fontset, unsigned long cursor_pixel, unsigned long border_pixel, unsigned long mouse_pixel, unsigned long cursor_foreground_pixel, unsigned long scroll_bar_foreground_pixel, unsigned long scroll_bar_background_pixel, Cursor text_cursor, Cursor nontext_cursor, Cursor modeline_cursor, Cursor hand_cursor, Cursor hourglass_cursor, Cursor horizontal_drag_cursor, Cursor vertical_drag_cursor, Cursor current_cursor, Cursor left_edge_cursor, Cursor top_left_corner_cursor, Cursor top_edge_cursor, Cursor top_right_corner_cursor, Cursor right_edge_cursor, Cursor bottom_right_corner_cursor, Cursor bottom_edge_cursor, Cursor bottom_left_corner_cursor, Window hourglass_window, XWMHints wm_hints, struct x_display_info* display_info, XEvent* saved_menu_event, bool_bf hourglass_p : 1, bool_bf explicit_parent : 1, bool_bf asked_for_visible : 1, bool_bf has_been_visible : 1, bool_bf wait_for_wm : 1, bool_bf alpha_identical_p : 1, GC gc, unsigned long pixel, unsigned long relief_background, int focus_state, int move_offset_top, int move_offset_left, bool window_offset_certain_p, int root_x, int root_y, int visibility_state

## File-scope objects — 11
- [x] extern struct x_display_info x_display_list  (line 999)
- [x] extern Window tip_window  (line 1010)
- [x] extern Lisp_Object tip_dx  (line 1011)
- [x] extern Lisp_Object tip_dy  (line 1012)
- [x] extern Lisp_Object tip_frame  (line 1013)
- [x] extern frame_parm_handler x_frame_parm_handlers  (line 1735)
- [x] extern bool x_dnd_in_progress  (line 2015)
- [x] extern bool x_dnd_waiting_for_finish  (line 2016)
- [x] extern struct frame x_dnd_frame  (line 2017)
- [x] extern struct frame x_dnd_finish_frame  (line 2018)
- [x] extern int x_error_message_count  (line 2019)

## Decisions

### Gating: the port's narrowing, not C's condition

`struct x_display_info` (C 426-1013) and `struct x_output` (C 1050-...) are **ungated in C**:
they live inside `XTERM_H` and are compiled whenever the header is read, in every configuration.
This mirror carries both - and every other record whose members name X11 types - under
`#[cfg (feature = "have_x_windows")]`.  That gate is the **port's**, decided by *type
availability*: `Display`, `Screen`, `XVisualInfo`, `Atom`, `Window`, `GC`, `Pixmap`, `KeySym`,
`XModifierKeymap` and the rest have no definition in this tree, so a frozen-configuration layout
assertion for these records cannot be compiled, and `spikes/mirror-struct-coverage.py --arm
frozen` therefore reports them `unavailable` rather than complete.  The gated arm is what the
tool measures for them, and its reading is the port's remaining work list for that surface
(`--arm x11` was 98 gaps on 3.283, 0 after this round's fill; the fields, the hoisted
`struct relief` and the `X_WMTYPE_*` constants are recorded in `docs/ports/xterm.md` Decision 13).


- **`check-cond-eval` is VERIFIED for this file, so every arm is the frozen configuration's
  verdict.**  `python3 tools/check-cond-eval.py src/xterm.h` -> `112 conditions, 0 unknown, 0
  disagreements [VERIFIED]`; `HAVE_X_WINDOWS`, `USE_X_TOOLKIT`, `USE_GTK`, `HAVE_XAW3D`,
  `HAVE_XINPUT2`, `HAVE_XRENDER`, `HAVE_XFT`, `HAVE_XDBE`, `USE_CAIRO`, `USE_MOTIF`, `USE_LUCID`,
  `USE_XCB`, `HAVE_XSYNC`, `HAVE_GSETTINGS` and `HAVE_X11` are all `undef` in
  `docs/baseline/config/darwin-arm64.macros.tsv`.  The live inventory the tool computes is
  therefore real and this ledger's 89 boxes are the gate's own checklist: **3 INLINE definitions,
  68 macros, 7 records with 286 field names, 11 file-scope objects** (the map's all-branches view
  for the file is 99 macros / 12 constants / 323 fields / 14 objects, `docs/port-inventory.md`
  676).
- **The whole body carries `#[cfg (feature = "have_x_windows")]`.**  The file is unguarded X11
  text, but no unit includes it without `HAVE_X_WINDOWS`, so P-51's arm for the non-frozen side is
  the X11 target's (the same reading `include/w32term.md` records for the Windows header, one file
  over).  C's own names are kept inside it and **not** replaced by stand-ins: `Display`, `Screen`,
  `Visual`, `XVisualInfo`, `XColor`, `XRectangle`, `XWMHints`, `Colormap`, `Cursor`, `GC`,
  `Pixmap`, `Drawable`, `XEvent`, `XRenderPictFormat`, `XrmDatabase`, `Atom`, `Window`, `Time`,
  `XSyncCounter`, `XSyncFence`, `XSyncValue`, `XIC`, `XIM`, `XFontSet`, `XIMStyles`, `Widget`,
  `Picture`, `cairo_pattern_t`, `cairo_t`, `GtkWidget`, `GdkCursor`, `XIDeviceEvent` are the X,
  GTK and cairo headers' names, which that target supplies (the `include/macfont.rs` 9-18 reading
  of `CGFloat`/`CGContextRef`).  That includes the four records C embeds **by value**
  (`XColor rgb`, `XVisualInfo visual_info`, `XWMHints wm_hints`, `XRectangle last_mouse_glyph`):
  a `*mut c_void` or a byte-array stand-in for a by-value foreign record would be a layout lie, and
  no compiler on this target can measure it (`include/macfont.rs` 26-27) - so the name is kept and
  the C text is in the `// C:` line.  **No layout assertion is written** for any record.
- **The inner arms carry the feature their C macro lowercases to** (status 3.171's rule).  Already
  in `femacs/xtask/Cargo.toml`'s table: `use_x_toolkit`, `use_gtk`, `have_xinput2`,
  `have_xinput2_4`, `have_x_i18n`, `use_cairo`.  **Ten names are new to this tree** and are
  requested in this round's report: `have_gtk3`, `have_x11r6_xim`, `have_xdbe`, `have_xfixes`,
  `have_xinput2_1`, `have_xinput2_2`, `have_xinput2_3`, `have_xrender`, `have_xsync`,
  `have_xsynctriggerfence` - each is the C macro of the same name, lowercased, and each appears in
  real `#[cfg]` attributes (counts in the Evidence).
- **Compound guards are recorded in `// C guard` lines instead of being modelled as features**:
  `defined (USE_TOOLKIT_SCROLL_BARS) && defined (USE_LUCID)`,
  `defined (USE_TOOLKIT_SCROLL_BARS) && !defined (USE_GTK)`,
  `defined HAVE_XSYNC && !defined USE_GTK && defined HAVE_CLOCK_GETTIME`,
  `defined (USE_X_TOOLKIT) || defined (USE_GTK)` and
  `defined HAVE_XFIXES && XFIXES_VERSION >= 40000` - each names its members and is written above
  the member or item it wraps.
- **The three INLINE definitions are carried with C's statements**: `SELECTION_EVENT_DISPLAY`
  (C 1715, "Unlike macros below, this can't be used as an lvalue"), `x_make_truecolor_pixel`
  (C 1867, the RGB scaling/shifting) and `x_mutable_colormap` (C 1887).  They read
  `dpyinfo->…`/`visual->…`, which only resolves under the same feature - so they are `#[cfg
  (feature = "have_x_windows")]` functions with C's bodies, not stubs.
- **Bitfields are recorded, not packed**: `valid` (C 230), `kind : EVENT_KIND_WIDTH` (C 1705) and
  `struct x_output`'s seven (`hourglass_p`, `explicit_parent`, `asked_for_visible`,
  `has_been_visible`, `wait_for_wm`, `alpha_identical_p`, and the XDBE pair `need_buffer_flip`,
  `complete`) - PORTING.md 1.3 / P-80 require the real compiler's byte dump before a storage unit
  may stand for a bitfield, and none of these records can be compiled on this target.
- **Degraded names (P-55/P-72 rule 3).**  `frame_parm_handler` is `dispextern.h` 3063's typedef and
  `include/dispextern.rs` does not carry it yet, so `x_frame_parm_handlers[]`'s elements are
  `*mut c_void` for now (the `include/androidterm.rs` 506-511 reading of the same array);
  `struct input_event` (`xg_pending_quit_event`, C 2012) and `struct glyph_string` pointers keep
  P-55's untyped stand-in.  `struct frame`, `struct terminal`, `struct font`, `struct scroll_bar`,
  `struct x_bitmap_record` and `struct x_display_info` are named: mirrors in this tree own them.
- **The prototype block is recorded, not transcribed.**  C 750-2043 is roughly 150 `extern`
  declarations.  `tools/extract-abi.py`'s `functions` list is *definitions* only (this ledger's
  "Functions defined" section holds the three INLINE ones) and no gate compares a prototype, so
  the block is left in C's own text at the file's end (the `include/haiku_support.md` note, where
  the author transcribed 178 by hand; this block is three times that).  An X11 unit that needs one
  takes it from the mirror's text the way C takes it from the header.

### The five members carried on 2026-10-05, and the names this mirror still leaves bare

`struct x_display_info` gained five members, each C's own under C's own guard (`docs/ports/xterm.md`
§19①): `xcb_connection_t *xcb_connection` and `xcb_visualtype_t *xcb_visual` (C 842-843, inside
`#ifdef USE_XCB` 841-844) under `use_xcb`, and `bool composite_supported_p` / `int composite_major`
/ `int composite_minor` (C 922-924, inside `#ifdef HAVE_XCOMPOSITE` 921-925) under
`have_xcomposite`.  The two xcb pointee types come from the new xcb section of
`include/xlib.rs`; nothing else of C 823-910 is carried yet, and the missing list is written down in
§19① rather than implied.

**One thing this mirror itself gets wrong, found while measuring**: `STANDARD_EVENT_SET`
(C 144-155) is a constant whose body names eleven event masks bare - `KeyPressMask`,
`ExposureMask`, `ButtonPressMask`, `ButtonReleaseMask`, `PointerMotionMask`, `StructureNotifyMask`,
`FocusChangeMask`, `LeaveWindowMask`, `EnterWindowMask`, `PropertyChangeMask`,
`VisibilityChangeMask` - and all eleven are `E0425` under `have_x_windows`: two of them are in
`include/xlib.rs`'s `x11_constants` module (a module the mirror's own top level does not import
from) and nine have no home anywhere.  Fixing it is a decision, not a typo - the gate compares a
macro's body digest, so a path-qualified body needs a `docs/ports/token-aliases.tsv` row, and a
mirror-level declaration is the `True`/`False` second home (3.270).  Registered, not fixed.

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/xterm.h femacs/src/include/xterm.rs`
  -> `OK emacs/src/xterm.h -> femacs/src/include/xterm.rs` (no `- ` problem lines).
- `python3 tools/extract-abi.py emacs/src/xterm.h` -> this ledger: 0 subrs, **3 functions
  defined** (`SELECTION_EVENT_DISPLAY` 1715, `x_make_truecolor_pixel` 1867, `x_mutable_colormap`
  1887), **68 macros**, **7 types** (286 fields: `color_name_cache_entry` 4, `scroll_bar` 13,
  `selection_input_event` 7, `x_bitmap_record` 9, `x_display_info` 136, `x_failable_request` 3,
  `x_output` 114), **11 file-scope objects** - every box checked.
- `python3 tools/check-cond-eval.py src/xterm.h` -> `112 conditions, 0 unknown, 0 disagreements
  [VERIFIED]`.
- The ten new feature names and how often each appears in a real `#[cfg]` attribute:
  `have_gtk3` 4, `have_x11r6_xim` 1, `have_xdbe` 2, `have_xfixes` 3, `have_xinput2_1` 9,
  `have_xinput2_2` 12, `have_xinput2_3` 1, `have_xrender` 5, `have_xsync` 19,
  `have_xsynctriggerfence` 1 (`rg -c '#\[cfg \(feature = "<name>"\)\]' femacs/src/include/xterm.rs`).
- **Two measured Rust parse rules found by `layout-check` on this mirror** (the arm is inert for
  compilation, but `rustc` still *parses* it, so even cfg'd text has to be valid Rust):
  1. C's `int use;` member (`struct xi_device_t`, C 344) must be a raw identifier in Rust -
     `pub r#use` (PORTING.md 0.2), measured as
     `error: expected identifier, found keyword 'use'`;
  2. C's `(x >> n) as T << m` needs parentheses (`((x >> n) as T) << m`) because Rust's cast binds
     tighter than the shift and the compiler reads it as generic arguments for `T`, measured as
     `expected 'mut' or 'const' keyword in raw pointer type` + `'<<' is interpreted as a start of
     generic arguments for 'c_ulong', not a shift`.
- **A unit includes this mirror** (`rg -n 'include/xterm' femacs` ->
  `femacs/src/xterm.rs:655`, the port of C 636's `#include "xterm.h"`, carried since 3.249
  (`2241507`)): the arm is reached by `build-tu src/xterm.c` as well, so the flip arms in
  docs/ports/xterm.md §19's Evidence see it.  The "no hits" sentence this line replaces was stale,
  not wrong when it was first written.  `xtask layout-check`'s single crate still compiles it with
  the arm inert.
- The 2026-10-05 readings: `python3 tools/extract-abi.py --check emacs/src/xterm.h
  femacs/src/include/xterm.rs` -> `OK` (the five new members change nothing the pairing compares);
  `build-tu src/xterm.h` still stops in the mirror-as-unit class (the `eassert` row above); the
  plain X11 flip arm still reads `aborting due to 490 previous errors` and this mirror contributes
  201 of them - 180 `cannot find type`, 14 `cannot find value` (X11 names with no mirror) and the 7
  `E0133` of its INLINE bodies outside an unsafe block (Decision 13's recorded class).
- `CARGO_TARGET_DIR=/tmp/femacs-target-s188 cargo run -q -p xtask -- layout-check` ->
  `layout-check: 92 of 102 src mirror(s) compiled, 10 platform-exclusive skipped (...)`,
  `layout probe ok`, `layout-check passed` (this mirror is among the 92).
- `build-tu src/xterm.h` -> `error: cannot find macro 'eassert' in this scope` (then `XUNTAG`,
  `c_ulong`): the headers C 129-130 includes reach `lisp.h`'s macros through the *unit's* own
  includes (PORTING.md 0.1-9) - the class every header mirror shares (`src/charset.h`,
  `src/nsterm.h`, `src/w32term.h`, `src/textconv.h`, `src/xwidget.h`, `src/w32gdiplus.h`,
  `src/pgtkgui.h`, `src/widget.h`, `src/w32font.h`, `src/w32gui.h`, `src/nsxwidget.h` all measured
  the same way).  The compile evidence for this mirror is `layout-check`.
- Gates on the same worktree: `check-declarations` 13 sites (0 forbidden, 13 allowed literals) / 0
  failures; `check-target-names` 621 .rs files / 593 map targets / 0 findings; `check-mirror-dupes`
  1 duplicate declaration(s) / 1 ratchet row(s) / 0 failures.

# ----------------------------------------------------------------------
## `x_bitmap_stipple` moves into a declaration module (2026-10-05)

**C.**  C 214-216: `#ifdef USE_CAIRO / extern cairo_pattern_t *x_bitmap_stipple (struct frame *,
Pixmap); / #endif`.  It is a *declaration* in the header and a **definition** in `image.c` 416
(non-`static`), so the mirror's crate-root declaration collides with the unit's definition the
moment the unit is compiled with `have_x_windows` + `use_cairo`: `error[E0428]: the name
x_bitmap_stipple is defined multiple times` was one of the six sites
`docs/ports/image.md`'s Evidence 3 measured in the unit's own text.

**The move.**  The declaration now sits in `pub mod xterm_bitmap_decls` with
`use xterm_bitmap_decls::*;` - the `window.rs` 302-367 `window_fn_decls` and `androidterm.rs`
513-537 `androidterm_objs` shape (PORTING.md 0.1-10 / P-21): a glob import is *shadowed* by a
crate-root definition, so the unit's definition stays the live one and no ratchet row is needed
(`tools/check-mirror-dupes.py` counts a declaration with a body as a definition and never as a
copy).

**Evidence.**

- `python3 tools/extract-abi.py --check emacs/src/xterm.h femacs/src/include/xterm.rs` -> `OK
  emacs/src/xterm.h -> femacs/src/include/xterm.rs`.
- `python3 tools/check-mirror-dupes.py` -> `18 duplicate declaration(s), 18 ratchet row(s),
  0 failure(s)`.
- The flip the item exists for: `src/image.rs` with `--cfg 'feature="have_window_system"'
  --cfg 'feature="have_x_windows"' --cfg 'feature="use_cairo"'` -> the `E0428` is gone; the error
  lines move `233 -> 234` only because four *other* `E0428`s (the `dispextern.h` typedefs this
  batch landed and the unit's use-site copies of them) now report at the resolution stage, and
  because `E0428` stops rustc before type-checking, the arm's `Atom`/`Cursor`/`XGetPixel` list
  (`include/xlib.rs`'s gap, 124 + 19 + 13 + 13 of the 223 `E0425`s) is the reading that remains -
  a name-resolution reading, not a type-check one (P-86 7).

## `cairo` is a unit-side declaration, not a mirror (the P-88 judgement, 2026-10-05)

`docs/ports/image.md`'s `/// C:` list carries sixteen cairo entry points, six constants and two
records that clang measured there.  `ls femacs/src/include | grep -i cairo` is empty and no file in
the tree has a `/// C: #include <cairo…` section, because **`<cairo.h>` is an external library
header the reference tree does not contain** - exactly P-88 1's case.  The judgement is therefore:
**the declarations belong to the unit that includes the header, and no `femacs/src/include/` file
is created for them.**  Two facts make it a live question rather than a formality:

* the machine *does* have cairo (`/opt/homebrew/include/cairo`, 1.18.4), so a clang probe can
  measure its records - and `src/image.rs` 363-431 already carries the three opaque tags
  (`cairo_t`/`cairo_surface_t`/`cairo_pattern_t` as `pub enum … {}`), the ten prototypes, the
  constants and the two records with their `/// C:` lines and four layout assertions, so the arm
  compiles from the unit's own text;
* this mirror's `use_cairo` **record member** (`x_bitmap_record.stipple: *mut cairo_pattern_t`,
  C 186-188) is a *use* of the name, not a declaration of it - it resolves because the unit that
  sets `use_cairo` brings the tag (the "a macro body is resolved only where it is expanded" shape,
  applied to a record field).  A mirror must not declare it: P-88 1's rule keeps the reference
  tree's own headers in `femacs/src/include/`, and nothing there corresponds to `cairo.h`.

# ----------------------------------------------------------------------
## The eleven event masks get a home, and `xlib.rs` grows the X11 surface (2026-10-05)

**The masks.**  `STANDARD_EVENT_SET` (C 144-155) names eleven event masks bare, and every one was
`E0425` under `have_x_windows`: two were in `include/xlib.rs`'s `x11_constants` module (a module
this mirror's top level does not glob-import from - `None` would shadow the prelude's
`Option::None`) and nine had no home anywhere.  The nine others (`KeyPressMask`, `EnterWindowMask`,
`LeaveWindowMask`, `PointerMotionMask`, `ExposureMask`, `VisibilityChangeMask`,
`FocusChangeMask`, `ButtonPressMask`, `ButtonReleaseMask`) now sit in that module beside them, and
C 144-155's constant carries a targeted `use x11_constants::{…}` - the module's documented
consumption pattern, and what the `True`/`False` pair's 3.270 note anticipated.  The same `use`
carries `StaticColor`/`StaticGray`/`TrueColor` (C 1884-1891's `x_mutable_colormap`) and
`ZPixmap`/`BitmapSuccess`, which `src/image.rs` 1463/1922 writes bare through this header's
`TERM_HEADER` edge.

**`include/xlib.rs`.**  That mirror now carries the Xlib surface the X arms name - the scalar
typedefs (`XID`, `Mask`, `Atom`, `VisualID`, `Window`, `Drawable`, `Font`, `Pixmap`, `Cursor`,
`Colormap`, `GContext`, `KeySym`, `KeyCode`), the records `Visual`, `Depth`, `Screen`, `XExtData`,
`XImage` (with its `funcs` member), `XColor`, `XModifierKeymap`, `XWMHints`, `XVisualInfo`, the
opaque `Display`/`GC`, and ten procedures - with each layout **measured** on this machine's
libX11 1.8.13 / xorgproto 2025.1 by a clang `sizeof`/`offsetof` probe and asserted where it is
declared, plus the format/visual-class/bitmap-result values.  `XGetPixel`/`XPutPixel`/
`XDestroyImage` are *macros* in `<X11/Xlib.h>`, not entry points, so they are the port's inline
functions with C's bodies (a `macro_rules!` first draft was rejected: the call sites write C's
call shape, and `E0423` is what a macro answers there).  The whole section is a glob-imported
module (`xlib_surface`) because `src/image.rs` 232 declares `Pixmap` itself.  `XEvent`'s 33 member
records are the one gap left, with C's measured size (192 bytes) and the members recorded as owed.

### status 3.496 (the mirror round this file took part in)

- The edge/declaration changes are recorded in `docs/status.md` 3.496: `include/dispextern.rs`'s
  `include!("xlib.rs")` edge lost its `#[cfg]` (the `xtask` dedupe gives the first edge the slot),
  `include/xlib.rs` widened `XRectangle`/`Window`/`XrmDatabase`/`XID` to
  `any (have_x_windows, have_pgtk)`, `include/pgtkterm.rs` fixed a dangling `///` (a syntax error
  that made every `have_pgtk` TU unparsable), its `pgtk_im_set_cursor_location` prototype and its
  two bare `atimer` tags, `include/font.rs` gained the `harfbuzz` arm of the `font_driver` size
  assertion (clang-measured 208/224), `include/xterm.rs`'s three INLINE bodies became `unsafe fn`,
  `include/gtk.rs` gained `GtkWidgetClass.get_preferred_height`, and `include/emacsgtkfixed.rs`
  moved `emacs_fixed_new` into a `_decls` module.  The two arm flips that were blocked now read
  **exit 0**: `src/pgtkim.rs` (`have_pgtk` + `have_gtk3`, 14,083,208-byte rlib) and
  `src/hbfont.rs` (`harfbuzz`, 14,580,952-byte rlib); `layout-check` passes and
  `tools/check-all-units.sh` is **83/0 + 83/0**.

## 2026-10-05: the XRender members were missing from `x_display_info` (3.520)

C 898-902's `#ifdef HAVE_XRENDER` block (`xrender_supported_p`, `xrender_major`,
`xrender_minor`) was not in this mirror, so this file's *own* `FRAME_CHECK_XR_VERSION!` (C
1504-1510) read `E0609` in any `have_xrender` arm - five of `src/xfont.rs`'s.  They are carried
now, gated on the feature.  `pict_format` (C 464) also named `XRenderPictFormat`, a type whose
header has no mirror here: the member is the P-72 rule 3 untyped pointer, with a unit that has
the header's names casting at its use site.  Same flip: the XRender arm goes **14 -> 1**, and the
one that is left is `include/systime.rs`'s feature/config assertion (whose faithful arm is
`xtask dead-arms`, status 3.434/3.506).

## 2026-10-05: `x_mark_frame_dirty` (3.521)

C 1405 declares it and C 1412's `FRAME_X_DRAWABLE` calls it - and nothing in the tree declared it,
so `src/xfont.rs` carried its own declaration at the use site (its Owed 5, status 3.519).  It is
now in `xterm_bitmap_decls`, the module this mirror glob-imports for names whose *definition* is a
unit's (`xterm.c` defines it): a crate-root declaration would be `E0428` there, while a glob
import is shadowed by the definition.

## 2026-10-07: `x_error_message_count` 的缺席有了机器可读的登记（3.900）

C 2019 的 `extern int x_error_message_count;` 按 0.1-2 本该在这面镜像里，而它的**定义**是
`src/xterm.rs` 13702-13707 的（C 26252 的非 `static` 定义，`#[no_mangle] pub static mut`），
镜像再声明一次就是 `E0428`（P-90 7 的同一条，两次读数都记在本镜像 1644-1647 的注释里）。
此前这道缺席**只有注释**，`tools/extract-abi.py --check` 一直把它读成 `missing file-scope
object`（整树 13 份 failing 里最便宜的那一条）。现在它登记在
`docs/ports/excluded-objects.txt`（P-90 7 与 PORTING.md 0.1-2 的新例外），门的读数是
`OK` 并打印规模：`(1 file-scope object(s) recorded as not carried, with their reasons:
x_error_message_count)`；读到它的单元仍自己声明（`src/eval.rs` 1835-1836）。
