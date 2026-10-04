# emacs/src/nsmenu.m (2037 lines)

> **This file is not compiled by the frozen configuration** (`docs/baseline/frozen-build-sources.txt`,
> `docs/status.md` 3.31): the frozen target is darwin-arm64 with no window system, so `HAVE_NS`
> and `NS_IMPL_COCOA` are undefined and src/nsmenu.m belongs to the 77 src files `make -n` never
> reaches.  Batch F ports the Objective-C front end to Swift anyway (`docs/port-map.md`'s Swift
> table, `src/*.m` -> `femacs/platform/macos/*.swift`), so this ledger is the checklist of the
> **Cocoa/AppKit build** the file's `NS_IMPL_COCOA` arms describe, and the unit is
> `femacs/platform/macos/nsmenu.swift`.  Line numbers are the raw reference lines.

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 2
- [x] ns-reset-menu  (Fns_reset_menu, 0/0, doc e959e2321367, line 2008)
- [x] menu-or-popup-active-p  (Fmenu_or_popup_active_p, 0/0, doc 140765ec91aa, line 2017)

## Functions defined — 12
- [x] free_frame_menubar  (line 74)
- [x] popup_activated  (line 97)
- [x] ns_update_menubar  (line 111, static)
- [x] set_frame_menubar  (line 444)
- [x] prettify_key  (line 626, static, the `NS_IMPL_COCOA` arm; the GNUstep `#else` halves of
       the Cocoa tests around it - 553-554 in `addItemWithWidgetValue:` and 586-592 in
       `removeAllItems` - are recorded, not carried)
- [x] ns_menu_show  (line 885)
- [x] free_frame_tool_bar  (line 1096)
- [x] update_frame_tool_bar_1  (line 1117)
- [x] update_frame_tool_bar  (line 1240)
- [x] pop_down_menu  (line 1549, static)
- [x] ns_popup_dialog  (line 1567)
- [x] syms_of_nsmenu  (line 2031)

## Objective-C classes — 4 `@implementation`s + 1 view subclass
- [x] EmacsMenu  (`@implementation` 460-874, declared src/nsterm.h 540-553) -> Swift
      `class EmacsMenu: NSMenu, NSMenuDelegate`:
  - [x] ivar `BOOL needsUpdate;` (nsterm.h 542) -> `var needsUpdate: Bool`
  - [x] `-initWithTitle:` (463) -> `override init(title:)`; `-initWithCoder:` -> `required init(coder:)`
  - [x] `-menuNeedsUpdate:` (477) -> `@objc func menuNeedsUpdate(_:)`
  - [x] `-performKeyEquivalent:` (519) -> `override func performKeyEquivalent(with:)`
  - [x] `-addItemWithWidgetValue:attributes:` (528) -> `@objc func addItemWithWidgetValue(_:attributes:)`
  - [x] `-removeAllItems` (582) -> `override func removeAllItems()`
  - [x] `subst_t` + `key_symbols` + `prettify_key` (598-655, the `NS_IMPL_COCOA` arm) ->
        `EmacsMenu.subst_t` / `EmacsMenu.key_symbols` / `EmacsMenu.prettify_key`
  - [x] `-fillWithWidgetValue:` (657) ; `-addSubmenuWithTitle:` (736) ;
        `-runMenuAt:forFrame:keymaps:` (749) ; `-menu:willHighlight:` (782)
  - [x] the GNUstep-only class methods (821-872: `close`, `menu:updateItem:`,
        `menuHasKeyEquivalent:`, `numberOfItemsInMenu:`, `menuWillOpen:`, `menuDidClose:`,
        `confinementRectForMenu:`) - recorded, not carried
- [x] EmacsToolbar  (`@implementation` 1263-1409, declared src/nsterm.h 563-578) -> Swift
      `class EmacsToolbar: NSToolbar, NSToolbarDelegate`; ivars `emacsView`/`identifierToItem`/
      `activeIdentifiers`/`prevIdentifiers`/`enablement`/`prevEnablement`
  - [x] `-initForView:withIdentifier:` (1265) ; `-dealloc` (1281, ARC) ; `-clearActive` (1291) ;
        `-clearAll` (1302) ; `-changed` (1311) ; `-addDisplayItemWithImage:…` (1319) ;
        `-validateVisibleItems` (1360) ; the three `NSToolbarDelegate` methods (1368, 1378, 1387) ;
        the `setVisible:` override (1396-1401) recorded (Swift spells it `isVisible`)
- [x] EmacsTooltip  (`@implementation` 1421-1538, declared src/nsterm.h 632-653) -> Swift
      `class EmacsTooltip: NSObject, NSWindowDelegate`; ivars `win`/`textField`/`timer`
  - [x] `-init` (1423) ; `-dealloc` (1458) ; `-setText:` (1466) ; `-setBackgroundColor:` (1480) ;
        `-setForegroundColor:` (1485) ; `-showAtX:Y:for:` (1490) ; `-moveTo:` (1507) ;
        `-hide` (1516) ; `-isActive` (1528) ; `-frame` (1533)
- [x] FlippedView  (`@interface` 1633-1636, `@implementation` 1638-1643) -> Swift
      `class FlippedView: NSView` with `override var isFlipped: Bool { return true }`
- [x] EmacsDialogPanel  (`@implementation` 1645-1999, declared src/nsterm.h 602-610) -> Swift
      `class EmacsDialogPanel: NSPanel`; ivars `command`/`title`/`matrix`/`rows`/`cols`/
      `timer_fired`/`window_closed`/`dialog_return` (the C `title` ivar is renamed `title_`,
      see Decisions)
  - [x] `-initWithContentRect:styleMask:backing:defer:` (1652) ;
        `-windowShouldClose:` (1746) ; `-dealloc` (1753, ARC) ;
        `-processMenuItems:used:withErrorOutput:` (1758) ; `-addButton:value:row:` (1803) ;
        `-addString:row:` (1824) ; `-addSplit` (1840) ; `-clicked:` (1847) ;
        `-initWithTitle:isQuestion:` (1865) -> `convenience init(title:isQuestion:)` ;
        `-resizeBoundsPriorToDisplay` (1882) ; `-timeout_handler:` (1937) ; `-runDialogAt:` (1957)

## Macros defined — 6
- [x] NSMENUPROFILE  (line 42, body b6589fc6ab0d) - the file defines it as 0, so its four `#if`
      blocks are dead text; recorded
- [x] TOOLPROP  (line 1146, body e983e6fbfd78) - the function-like macro inside
      `update_frame_tool_bar_1`; the port keeps C's index expression as a nested
      `func TOOLPROP(_:)` (`#undef` at 1208)
- [x] SPACER  (line 1647, body db63fca62705) -> `fileprivate let SPACER: CGFloat = 8.0`
- [x] ICONSIZE  (line 1648, body aa78e36bb058) -> `fileprivate let ICONSIZE: CGFloat = 64.0`
- [x] TEXTHEIGHT  (line 1649, body b8fc90fd7ade) -> `fileprivate let TEXTHEIGHT: CGFloat = 20.0`
- [x] MINCELLWIDTH  (line 1650, body f73e84166931) -> `fileprivate let MINCELLWIDTH: CGFloat = 90.0`

## Types and their fields — 1
- [x] `<anon@599>` (`const char *from, const char *to`) -> `EmacsMenu.subst_t` (16 bytes, measured
      on both sides)

## File-scope objects — 3
- [x] `EmacsMenu *svcsMenu`  (line 50) -> `@_silgen_name("svcsMenu") var svcsMenu:
      UnsafeMutableRawPointer?` (an import - Swift cannot define a C data symbol; see (3))
- [x] `static int popup_activated_flag`  (line 52) -> `fileprivate var popup_activated_flag: Int32`
- [x] `static struct frame *last_menubar_frame`  (line 56) ->
      `fileprivate var last_menubar_frame: UnsafeMutableRawPointer?`

## Decisions

**Which arms are live (the lane's target is the Cocoa/AppKit build).**
- `NSMENUPROFILE` is defined as 0 by the file itself (C 42), so its four `#if` blocks (44-47,
  133-136, 165-168, 421-425) are dead text: recorded, not carried.
- The other implementation's arms are recorded, not carried: the `#ifdef NS_IMPL_GNUSTEP` blocks
  (113-120, 142-145, 295-297, 384-386, 409-413, 431-433, 487-515, 728-731, 821-872, 1347-1349)
  and the `#else` halves of the `NS_IMPL_COCOA` tests (553-554, 586-592, 1130-1140).
- `NSTRACE` is a no-op in the reference too (`NSTRACE_ENABLED` 0, src/nsterm.h 143 discards its
  arguments unread), so every call site keeps C's text as a `/* C: NSTRACE ("…"); */` comment
  instead of a call the C text does not have.
- The `#ifdef NS_IMPL_COCOA` arms are the ones the port carries: 395-398, 548-552, 584-585,
  598-655 (`subst_t`/`key_symbols`/`prettify_key`), 662-702 (the tab-stop attributes),
  770-773 (`allowsContextMenuPlugIns`), 1128-1129, 1211-1233, 1560-1562 (the `makeKeyWindow`
  spelling in `pop_down_menu`), plus the `NSMenuDelegate`/`NSToolbarDelegate`/`NSWindowDelegate`
  conformances declared in src/nsterm.h 540, 563, 632.

**`HAVE_MULTILINGUAL_MENU` is undefined in the reference, so the `#ifndef` arms are live.**
Nothing in `emacs/` defines it (`grep -rn "define HAVE_MULTILINGUAL_MENU" emacs/` is empty;
`emacs/admin/CPP-DEFINES` 227 lists the name only as a user option), and a clean clang probe of
`#include <config.h>` + `"lisp.h"` + the three spellings keeps the `#ifndef`/`#if !defined`
branches and drops the `#ifdef` one.  C 956-962, 1006-1018 and 1059-1062 are therefore compiled,
and the port calls `ENCODE_MENU_STRING` in them.  `check-cond-eval.py src/nsmenu.m` reports three
disagreements on exactly those lines; they are a defect of the tool's *probe*, not a mis-read arm
(Evidence, "the `-dM` + `@` artefact"; also filed in (3)).

**The two `DEFUN`s.** C's `DEFUN` (src/lisp.h 3406-3440) is two items: a file-static
`union Aligned_Lisp_Subr` in `__DATA,subrs`, and the `Lisp_Object F…` function.  Both are kept,
adjacently, with C's `DEFUN` text (docstring included) as the comment that introduces them:
the function is `@_cdecl("Fns_reset_menu")` / `@_cdecl("Fmenu_or_popup_active_p")` so the symbol
keeps C's name and signature; the record is a `fileprivate var` built by `make_aligned_subr`,
which spells C's initialiser (header word `PVEC_SUBR << PSEUDOVECTOR_AREA_BITS`, the function
slot, `min_args`/`max_args`, a `StaticString` symbol name whose bytes are NUL-terminated in the
binary, a null intspec, `command_modes` = `Qnil`, `doc` = 0).  `syms_of_nsmenu` registers them
with `defsubr` exactly as the reference does.  This is the `docs/ports/nsselect.md` shape; the
docstrings live in the comments, which is the DOC-scanner gap recorded in (3).

**Objective-C to Swift.**
- `[obj method: x]` becomes an optional-chained call (`obj?.method(x)`); C's nil-messaging is
  Swift's optional chaining.  `@implementation X : Y` becomes `class X: Y` with the same ivars
  and methods, and the `@interface` sections of src/nsterm.h are the class declarations the
  checklist above names.
- The `EmacsDialogPanel` ivar `NSTextField *title` (nsterm.h 605) is renamed `title_`: NSWindow
  already has a `title` property (`String`), and C's own `[self setTitle: @""]` (1689) is that
  one - the `prev_buffer_` shape of `docs/ports/nsimage.md`.
- Manual `release`/`dealloc` become ARC: `-dealloc` bodies that only release are `deinit {}`
  (`EmacsToolbar` 1281, `EmacsDialogPanel` 1753), and the ones with a side effect keep it
  (`EmacsTooltip` 1458's `[win close]`).
- C's `const char *` tables are `strdup`ed once (`key_symbols`, the two error strings,
  `empty_c_string`), because a Swift string literal's pointer only lives for the call and C's
  literals are static storage.
- `NSEvent.otherEvent (…)`/`mouseEvent (…)` take `context: nil`: C passes `[NSApp context]`, which
  is deprecated and absent from the SDK's Swift interface; `nil` is the "no graphics context"
  the event carries on a modern macOS.  Recorded here because it is a deliberate deviation from
  C's text (src/nsmenu.m 759, 1944).
- `[NSMenu setTitle:]`-style setters become property assignments where AppKit's Swift interface
  spells them that way (`NSWindow.makeKey()`, `NSMenu.popUpContextMenu(_:with:for:)`,
  `NSApplication.helpMenu`), and `[NSApp mainMenu]` is `NSApp.mainMenu`.

**Function-like macros and local constants.** `TOOLPROP (IDX)` (1146) is a nested
`func TOOLPROP(_ idx: Int32) -> Lisp_Object` keeping C's index expression; `MAX` is a tiny
`fileprivate func MAX`; the four `@implementation`-local constants are `fileprivate let`s next to
`EmacsDialogPanel`; `countof`/`strlen`/`memcpy`/`memmove`/`memmem`/`strcmp`/`strdup` are the
SDK's.

**`alloca` becomes heap.** `ns_menu_show`'s `USE_SAFE_ALLOCA` + `SAFE_ALLOCA (menu_items_used *
sizeof *submenu_stack)` is an `UnsafeMutablePointer<…>.allocate` freed by the matching
`deallocate` (C's `SAFE_FREE ()` at 1084), and `ns_update_menubar`'s `alloca
(previous_menu_items_used * sizeof *previous_items)` likewise.

**Imports, and five names that were wrong in the draft.** A `@_silgen_name` `var` imports
another unit's storage, which is what C's `extern` is; a `@_silgen_name` `func` with no body
imports a C function.  Five of the draft's imports did not match the reference and were corrected
before the evidence below was taken:
- `use_menu_items` does not exist (keyboard.h 385 declares only `unuse_menu_items`) - removed;
- `QFcar`/`QFcdr` do not exist here and nsmenu.m never uses them - removed;
- `CONSP` is never used by nsmenu.m - removed;
- C 189 tests `Voverriding_local_map_menu_flag`, a `DEFVAR_LISP` **variable** (src/keyboard.c
  14147), not a `Q…` symbol - the import was replaced by the data import
  `Voverriding_local_map_menu_flag`.
The checks that caught them are re-runnable: a `grep -w` of every imported name over
`emacs/{src,lib,lwlib}`, and a cross-check of every `/// C: … <file> <line>` citation in the file
against the line it cites (both are in Evidence).

**`svcsMenu`.** C 50 *defines* `EmacsMenu *svcsMenu` and src/nsterm.m 6088-6138 is its only user.
Swift cannot emit a C-named data symbol (`@_cdecl` refuses `var`; `@_silgen_name` on a `var` only
imports), so this file imports the name like the other storage it reads; where the storage itself
lives is recorded in (3).

## ABI

### (1) C symbols this file defines and exports (`@_cdecl`)

| C name | C prototype (reference) | Swift signature |
| --- | --- | --- |
| `free_frame_menubar` | `void (struct frame *)` (src/frame.h 1883) | `(UnsafeMutableRawPointer?) -> Void` |
| `popup_activated` | `int (void)` (src/dispextern.h 3879) | `() -> Int32` |
| `set_frame_menubar` | `void (struct frame *, bool)` (src/frame.h 1881) | `(UnsafeMutableRawPointer?, Bool) -> Void` |
| `ns_menu_show` | `Lisp_Object (struct frame *, int, int, int, Lisp_Object, const char **)` (src/menu.h 58) | `(UnsafeMutableRawPointer?, Int32, Int32, Int32, Lisp_Object, UnsafeMutablePointer<UnsafePointer<CChar>?>?) -> Lisp_Object` |
| `free_frame_tool_bar` | `void (struct frame *)` (src/nsterm.h 1180) | `(UnsafeMutableRawPointer?) -> Void` |
| `update_frame_tool_bar_1` | `void (struct frame *, EmacsToolbar *)` (src/nsterm.h 1177) | `(UnsafeMutableRawPointer?, UnsafeMutableRawPointer?) -> Void` |
| `update_frame_tool_bar` | `void (struct frame *)` (src/nsterm.h 1175) | `(UnsafeMutableRawPointer?) -> Void` |
| `ns_popup_dialog` | `Lisp_Object (struct frame *, Lisp_Object, Lisp_Object)` (src/nsterm.h 1184) | `(UnsafeMutableRawPointer?, Lisp_Object, Lisp_Object) -> Lisp_Object` |
| `Fns_reset_menu` | `Lisp_Object (void)` (DEFUN, src/lisp.h 3406) | `() -> Lisp_Object` |
| `Fmenu_or_popup_active_p` | `Lisp_Object (void)` (DEFUN) | `() -> Lisp_Object` |
| `syms_of_nsmenu` | `void (void)` (src/nsterm.h 1198) | `() -> Void` |

The two `S…` records C's `DEFUN` defines beside these functions are file-static in C and
`fileprivate` here.

### (2) C symbols this file calls (all `@_silgen_name`)

**a. Symbols the reference defines out of line** (the ported core defines them):

| C name | prototype | declared | defined |
| --- | --- | --- | --- |
| `menu_bar_items` | extern Lisp_Object menu_bar_items (Lisp_Object keymap) | src/lisp.h 5087 | src/keyboard.c 8641 |
| `parse_single_submenu` | extern bool parse_single_submenu (Lisp_Object key, Lisp_Object string, Lisp_Object maps) | src/menu.h 36 | src/menu.c 521 |
| `digest_single_submenu` | extern widget_value *digest_single_submenu (int start, int end, bool top_level_items) | src/menu.h 44 | src/menu.c 619 |
| `make_widget_value` | extern widget_value *make_widget_value (const char *name, char *value, bool enabled, Lisp_Object help) | src/menu.h 43 | src/menu.c 571 |
| `free_menubar_widget_value_tree` | extern void free_menubar_widget_value_tree (widget_value *) | src/menu.h 39 | src/menu.c 593 |
| `update_submenu_strings` | extern void update_submenu_strings (widget_value *) | src/menu.h 40 | src/menu.c 846 |
| `init_menu_items` | extern void init_menu_items (void) | src/menu.h 32 | src/menu.c 78 |
| `finish_menu_items` | - |  | src/menu.c 98 |
| `discard_menu_items` | - |  | src/menu.c 112 |
| `save_menu_items` | - |  | src/menu.c 145 |
| `list_of_panes` | extern void list_of_panes (Lisp_Object menu) | src/menu.h 37 | src/menu.c 496 |
| `inhibit_garbage_collection` | extern specpdl_ref inhibit_garbage_collection (void) | src/lisp.h 4654 | src/alloc.c 5458 |
| `record_unwind_save_match_data` | extern void record_unwind_save_match_data (void) | src/lisp.h 4998 | src/search.c 3157 |
| `specbind` | extern void specbind (Lisp_Object symbol, Lisp_Object value) | src/lisp.h 4817 | src/eval.c 3656 |
| `unbind_to` | extern Lisp_Object unbind_to (specpdl_ref count, Lisp_Object value) | src/lisp.h 4833 | src/eval.c 3940 |
| `record_unwind_protect_void` | extern void record_unwind_protect_void (void (*) (void)) | src/lisp.h 4825 | src/eval.c 3788 |
| `record_unwind_protect_ptr` | extern void record_unwind_protect_ptr (void (*) (void *), void *) | src/lisp.h 4820 | src/eval.c 3739 |
| `safe_run_hooks` | extern void safe_run_hooks (Lisp_Object hook) | src/lisp.h 5097 | src/keyboard.c 1970 |
| `set_buffer_internal_1` | extern void set_buffer_internal_1 (struct buffer *b) | src/buffer.h 1113 | src/buffer.c 2327 |
| `block_input` | INLINE void block_input (void) | src/blockinput.h 52 | - |
| `unblock_input` | - |  | src/keyboard.c 8405 |
| `Fcar` | extern Lisp_Object Fcar (Lisp_Object) | src/lisp.h 1588 | src/data.c 659 |
| `Fcdr` | - |  | src/data.c 677 |
| `Fcons` | - |  | src/alloc.c 2625 |
| `CHECK_STRING` | - |  | - |
| `check_window_system` | extern void check_window_system (struct frame *) | src/frame.h 1504 | src/frame.c 134 |
| `defsubr` | extern void defsubr (union Aligned_Lisp_Subr *) | src/lisp.h 3444 | src/lread.c 5188 |
| `FRAME_LINE_HEIGHT` | #define FRAME_LINE_HEIGHT(F) ((F)->line_height) | src/frame.h 1561 | - |
| `FRAME_COLUMN_WIDTH` | - |  | - |
| `build_string` | INLINE Lisp_Object build_string (const char *str) | src/lisp.h 4574 | - |
| `list1` | extern Lisp_Object list1 (Lisp_Object) | src/lisp.h 4486 | src/alloc.c 2665 |
| `list2` | - |  | src/alloc.c 2671 |
| `xmalloc` | extern void *xmalloc (size_t) | src/lisp.h 5535 | src/alloc.c 658 |
| `xfree` | - |  | src/alloc.c 713 |
| `error` | extern AVOID error (const char *, ...) | src/lisp.h 4835 | src/eval.c 2294 |
| `emacs_abort` | extern AVOID emacs_abort (void) NO_INLINE | src/lisp.h 5302 | src/sysdep.c 2402 |
| `quit` | extern Lisp_Object quit (void) | src/lisp.h 4786 | src/eval.c 1932 |
| `show_help_echo` | extern void show_help_echo (Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object) | src/keyboard.h 511 | src/keyboard.c 2259 |
| `menu_separator_name_p` | extern bool menu_separator_name_p (const char *) | src/keyboard.h 473 | src/keyboard.c 8608 |
| `unuse_menu_items` | extern void unuse_menu_items (void) | src/keyboard.h 385 | src/menu.c 103 |
| `timer_check` | extern struct timespec timer_check (void) | src/keyboard.h 521 | src/keyboard.c 4914 |
| `find_and_return_menu_selection` | extern Lisp_Object find_and_return_menu_selection (struct frame *f, bool keymaps, void *client_data) | src/nsterm.h 1181 | src/menu.c 971 |
| `valid_image_p` | extern bool valid_image_p (Lisp_Object) | src/dispextern.h 3755 | src/image.c 1395 |
| `prepare_image_for_display` | extern void prepare_image_for_display (struct frame *, struct image *) | src/dispextern.h 3756 | src/image.c 1860 |
| `lookup_image` | extern ptrdiff_t lookup_image (struct frame *, Lisp_Object, int) | src/dispextern.h 3757 | src/image.c 3556 |

**b. Header-only constructs** - macros, INLINE functions, enumerators, member accessors the port
names for itself, and generated-globals entries.  The reference has no symbol for any of them, so
the core side exports the C-ABI entry named here:

| C name | reference form / where |
| --- | --- |
| `EQ` | - |
| `NILP` | - |
| `STRINGP` | - |
| `VECTORP` | - |
| `AREF` | - |
| `ASET` | - |
| `ASIZE` | - |
| `aref_addr` | C: `INLINE Lisp_Object *aref_addr (Lisp_Object array, ptrdiff_t idx)` (src/lisp.h 1971). |
| `xvector_contents` | C: `INLINE void *xvector_contents (Lisp_Object a)` (src/lisp.h 3957). |
| `word_size` | C: `enum { header_size = …, bool_header_size = …, word_size = sizeof (Lisp_Object) }` (src/lisp.h 1830-1835) - |
| `STRING_MULTIBYTE` | C: `INLINE bool STRING_MULTIBYTE (Lisp_Object str)` (src/lisp.h 1601) and `#define ENCODE_MENU_STRING(str) ENC |
| `ENCODE_MENU_STRING` | - |
| `XSETFRAME` | C: `#define XSETFRAME(a, b) (XSETPSEUDOVECTOR (a, b, PVEC_FRAME))` (src/frame.h) - the macro assigns to its fi |
| `XBUFFER` | C: `INLINE struct buffer *XBUFFER (Lisp_Object a)` (src/buffer.h 1421) and `INLINE struct window *XWINDOW (Lis |
| `XWINDOW` | - |
| `IMAGE_FROM_ID` | C: `INLINE Lisp_Object image_spec_value (…)`-family accessors this file reads through `struct image`: `IMAGE_F |
| `image_load_failed_p` | - |
| `image_pixmap` | - |
| `FRAME_NS_VIEW` | C: `#define FRAME_NS_VIEW(f) ((f)->output_data.ns->view)` (src/nsterm.h 1045). |
| `SELECTED_FRAME` | C: `#define SELECTED_FRAME()` (src/frame.h 1526-1528). |
| `FRAME_LIVE_P` | C: `#define FRAME_LIVE_P(f)` (src/frame.h 1043), `FRAME_NS_P(f)` (src/frame.h 946-948), `FRAME_EXTERNAL_MENU_B |
| `FRAME_NS_P` | - |
| `FRAME_EXTERNAL_MENU_BAR` | - |
| `FRAME_SELECTED_WINDOW` | - |
| `FRAME_MENU_BAR_ITEMS` | - |
| `FRAME_COLS` | - |
| `FRAME_LINES` | - |
| `fset_menu_bar_items` | C: `INLINE void fset_menu_bar_items (struct frame *f, Lisp_Object val)` (src/frame.h 790) and `fset_menu_bar_v |
| `fset_menu_bar_vector` | - |
| `frame_menu_bar_items_used` | C: the frame members this file reads or writes directly and that no C macro or INLINE function names (`f->menu |
| `frame_set_menu_bar_items_used` | - |
| `frame_menu_bar_vector` | - |
| `frame_n_tool_bar_items` | - |
| `frame_tool_bar_items` | - |
| `frame_left_pos` | - |
| `frame_top_pos` | - |
| `ns_output_in_animation` | - |
| `ns_output_set_in_animation` | - |
| `timespec_valid_p` | C: `INLINE bool timespec_valid_p (struct timespec t)` (src/systime.h 68) and `timespectod` (lib/timespec.h 86, |
| `timespectod` | - |
| `SPECPDL_INDEX` | - |
| `MENU_ITEMS_PANE_NAME` | C: enumerators of `enum menu_item_idx` / `menu_item_pane_idx` (src/keyboard.h 360-378), `enum tool_bar_item_id |
| `MENU_ITEMS_PANE_PREFIX` | - |
| `MENU_ITEMS_PANE_LENGTH` | - |
| `MENU_ITEMS_ITEM_NAME` | - |
| `MENU_ITEMS_ITEM_ENABLE` | - |
| `MENU_ITEMS_ITEM_VALUE` | - |
| `MENU_ITEMS_ITEM_EQUIV_KEY` | - |
| `MENU_ITEMS_ITEM_DEFINITION` | - |
| `MENU_ITEMS_ITEM_TYPE` | - |
| `MENU_ITEMS_ITEM_SELECTED` | - |
| `MENU_ITEMS_ITEM_HELP` | - |
| `MENU_ITEMS_ITEM_LENGTH` | - |
| `MENU_KEYMAPS` | - |
| `TOOL_BAR_ITEM_ENABLED_P` | - |
| `TOOL_BAR_ITEM_CAPTION` | - |
| `TOOL_BAR_ITEM_IMAGES` | - |
| `TOOL_BAR_ITEM_TYPE` | - |
| `TOOL_BAR_ITEM_HELP` | - |
| `TOOL_BAR_ITEM_LABEL` | - |
| `TOOL_BAR_ITEM_NSLOTS` | - |
| `TOOL_BAR_IMAGE_ENABLED_SELECTED` | - |
| `PVEC_SUBR` | C: `PVEC_SUBR` (src/lisp.h 999). C: `PSEUDOVECTOR_AREA_BITS` (src/lisp.h 1039). |
| `PSEUDOVECTOR_AREA_BITS` | - |
| `Qnil` | C: `#define Qfoo builtin_lisp_symbol (iQfoo)`, one per symbol, generated into globals.h (PORTING.md P-17); cal |
| `Qt` | - |
| `Qinhibit_quit` | - |
| `Qdebug_on_next_call` | - |
| `Qoverriding_terminal_local_map` | - |
| `Qoverriding_local_map` | - |
| `Qactivate_menubar_hook` | - |
| `Qmenu_bar_update_hook` | - |
| `Qlambda` | - |
| `Qquote` | - |
| `QCtoggle` | - |
| `QCradio` | - |
| `Qundefined` | - |

**c. Constructs of this lane's headers that other lane files define** (the definition owner is
named; see (3)):

| C name | reference form | owner |
| --- | --- | --- |
| `createToolbar` | - (void)createToolbar: (struct frame *)f | src/nsterm.h 440 |
| `stringWithLispString` | +[NSString stringWithLispString:] | src/nsterm.h 368 |
| `lispString` | - |  |

**d. The core's variable storage this file imports** (a `@_silgen_name` `var` is an import of the
symbol's storage; C's `extern`):

| C name | reference |
| --- | --- |
| `menu_items` | `extern Lisp_Object menu_items` (src/keyboard.h 344) |
| `menu_items_allocated` | `extern int menu_items_allocated` (src/keyboard.h 351) |
| `menu_items_used` | `extern int menu_items_used` (src/keyboard.h 354) |
| `menu_items_n_panes` | `extern int menu_items_n_panes` (src/keyboard.h 358) |
| `ns_app_name` | `extern NSString *ns_app_name` (src/nsterm.h 775, defined src/nsterm.m) |
| `context_menu_value` | `extern int context_menu_value` (src/nsterm.h 933, defined src/nsterm.m) |
| `Vmenu_updating_frame` | `DEFVAR_LISP ("menu-updating-frame", …)` (src/xdisp.c) |
| `Voverriding_local_map_menu_flag` | `DEFVAR_LISP ("overriding-local-map-menu-flag", …)` (src/keyboard.c 14147) |
| `svcsMenu` | `extern EmacsMenu *svcsMenu` (src/nsterm.h 776) - **defined by src/nsmenu.m 50**, this file's own object |

**e. Spellings the port provides itself** (no ABI entry): `eassert` (src/lisp.h 181),
`lispVariadicArg` (the `va_arg (ap, Lisp_Object)` convention), `make_aligned_subr` (C's `DEFUN`
initialiser), `MAX`, `empty_c_string`, `menu_items_at`, `TOOLPROP`, `EmacsMenu.subst_t` /
`EmacsMenu.key_symbols` / `EmacsMenu.prettify_key`, and the `DEFSYM (Qdebug_on_next_call, …)`
call site (empty in the reference, src/lisp.h 2394).

### (3) What the lane still has to settle (flagged for the integrator)

1. **The four classes are instantiated outside this file.** `[[EmacsMenu alloc] initWithTitle:…]`
   at src/nsterm.m 6088-6093, `[[EmacsToolbar alloc] initForView:…]` at 9895 and `[EmacsTooltip
   class]` at 2534 are ObjC sends a stand-alone Swift file cannot receive: another file's
   `alloc`/`init` and its class object need a cross-file rule (the `docs/ports/nsselect.md` (3)
   item 3 family).
2. **The nine data imports of (2d) need a definition on the port's C-ABI side**, including
   `svcsMenu`, whose C definition is this very file (src/nsmenu.m 50): no Swift file can emit a
   C-named data symbol, so the storage for those nine has to come from the ported core (or from a
   small C-ABI shim, which is a lane-level decision).
3. **`check-cond-eval.py`'s `-dM` + `@` artefact.** The tool asks clang for a marker macro with
   `clang -dM -E`; once the translation unit contains an `@`, clang's `-dM` dump omits every macro
   defined after it, so the marker is reported missing and the condition is recorded as
   "clang False".  Minimal repro: `#include <config.h>` + `@` + `#define M1 1` -> `M1` absent from
   `-dM` (while `#ifdef M1` + `#error` proves it is defined).  Every `.m` unit's conditions after
   its first `@` are affected - for this file that is exactly the three `HAVE_MULTILINGUAL_MENU`
   arms.  Fixing the probe is the gate's owner's call, not this lane's.
4. The two `DEFUN` docstrings live in the file's comments; a future DOC scanner for the Swift lane
   should read them the way `extract-abi.py` reads the Rust units' docstring literals.
5. `tools/extract-abi.py` has no `.m -> .swift` pair, so `--check` cannot compare this unit; the
   checklist above was generated with it and the two doc digests were compared against the frozen
   `subrs.tsv`.

## Evidence

- `swiftc --version` ->
  `swift-driver version: 1.168.6 Apple Swift version 6.4 (swiftlang-6.4.0.34.1 clang-2100.3.34.1)`,
  `Target: arm64-apple-macosx26.0`.
- `cd /Users/liu/projects/src/github.com/liuchong/femacs && swiftc -parse-as-library -c -o
  /tmp/nm/nsmenu.o femacs/platform/macos/nsmenu.swift` -> **exit 0**, default language mode, 0
  errors, two warnings, both AppKit deprecations C's own text has:
  `'configuration' was deprecated in macOS 15.0` (nsmenu.swift 1431) and `'setConfiguration' was
  deprecated in macOS 15.0` (nsmenu.swift 1440) - C's `[toolbar configurationDictionary]` and
  `[toolbar setConfigurationFromDictionary:]` (src/nsmenu.m 1215, 1230).
- `nm -g /tmp/nm/nsmenu.o | grep ' T _'` -> the eleven exported C names, 11 of 11:
  `_Fns_reset_menu`, `_Fmenu_or_popup_active_p`, `_free_frame_menubar`, `_popup_activated`,
  `_set_frame_menubar`, `_ns_menu_show`, `_free_frame_tool_bar`, `_update_frame_tool_bar_1`,
  `_update_frame_tool_bar`, `_ns_popup_dialog`, `_syms_of_nsmenu`.
- `nm -u /tmp/nm/nsmenu.o` -> 540 undefined symbols, i.e. the whole C-ABI list of (2) plus the
  SDK's; the nine data imports appear as `_menu_items`, `_menu_items_allocated`,
  `_menu_items_used`, `_menu_items_n_panes`, `_ns_app_name`, `_context_menu_value`,
  `_Vmenu_updating_frame`, `_Voverriding_local_map_menu_flag`, `_svcsMenu` (the last is the (3)
  item 2 case).
- Layout, C side (`clang -std=gnu23 -I build/emacs-ref/src -I build/emacs-ref/lib -I emacs/src
  -I emacs/lib -I emacs/lwlib` on `/tmp/nm/probe3.c`, which includes `<config.h>`, `"lisp.h"` and
  `"menu.h"` and prints `sizeof`/`offsetof`) ->
  `vectorlike_header 8`; `Lisp_Subr 56 align 8` with offsets `header 0 function 8 min_args 16
  max_args 18 symbol_name 24 intspec 32 command_modes 40 doc 48`; `Aligned_Lisp_Subr 56 align 8`;
  `specpdl_ref 8`; `timespec 16 tv_sec 0 tv_nsec 8`; `widget_value 80 align 8 lname 0 name 8
  value 16 lkey 24 key 32 help 40 enabled 48 selected 49 edited 50 button_type 52 contents 56
  call_data 64 next 72`; `BUTTON_TYPE_NONE 0 TOGGLE 1 RADIO 2 MENU_KEYMAPS 1`.
- Layout, Swift side (`swiftc -o /tmp/nm/layoutbin femacs/platform/macos/nsmenu.swift
  /tmp/nm/layout/main.swift /tmp/nm/layout/stub.c -Xlinker -undefined -Xlinker dynamic_lookup`,
  where `main.swift` prints `MemoryLayout`/`offset(of:)` of the file's own types and `stub.c`
  defines the nine imported data symbols) -> `vectorlike_header 8`; `Lisp_Subr 56` with the same
  eight offsets; `Aligned_Lisp_Subr 56`; `specpdl_ref 8`; `timespec 16 tv_sec 0 tv_nsec 8`;
  `widget_value 80 align 8` with the same fourteen offsets; `subst_t 16 from 0 to 8`;
  `button_type 1 raw none 0 toggle 1 radio 2` - every number equals the C probe's.
- Enumerators and the ABI constants, C side (`clang -std=gnu23 -DHAVE_WINDOW_SYSTEM=1 -DHAVE_NS=1
  -DNS_IMPL_COCOA=1 -I build/emacs-ref/src -I build/emacs-ref/lib -I emacs/src -I emacs/lib -I
  emacs/lwlib` on `/tmp/nm/probe4.c`, which includes `<config.h>`, `"lisp.h"`, `"dispextern.h"`,
  `"keyboard.h"`, `"menu.h"`) -> `MENU_ITEMS_PANE_NAME/PREFIX/LENGTH 1/2/3`; `MENU_ITEMS_ITEM_*
  0 1 2 3 4 5 6 7 8`; `TOOL_BAR_ITEM_ENABLED_P 1 CAPTION 3 IMAGES 4 TYPE 6 HELP 7 LABEL 9
  NSLOTS 12`; `TOOL_BAR_IMAGE_ENABLED_SELECTED 0`; `MENU_KEYMAPS 1 MENU_FOR_CLICK 2`;
  `PVEC_SUBR 18 PSEUDOVECTOR_AREA_BITS 24 word_size 8`.  (`TOOL_BAR_ITEM_NSLOTS` is **12**, the
  measured sentinel at src/dispextern.h 3495, not 13.)
- `python3 tools/extract-abi.py emacs/src/nsmenu.m` -> the checklist above: 2 subrs (doc digests
  `e959e2321367`, `140765ec91aa`), 12 functions, 6 macros, 1 type, 3 file-scope objects.
- `python3 tools/check-swift-units.py` -> `femacs/platform/macos/nsmenu.swift: 12 C definition(s),
  0 unported, 0 row(s); 130 C symbol import(s)`, last line
  `140 new, 0 stale, 6 Swift unit(s) landed`.  The 140 `new` rows are all
  `femacs/platform/macos/nsterm.swift` (another lane's in-flight file, untracked); **zero new and
  zero stale rows are this unit**, so it needs no `swift-units.tsv` row.
- `python3 tools/check-cond-eval.py src/nsmenu.m` -> `src/nsmenu.m: 29 conditions, 0 unknown,
  3 disagreements [NOT VERIFIED]`, all three at `ifndef HAVE_MULTILINGUAL_MENU`
  (956, 1006, 1059) with `clang False`.  **The `False` is the tool's probe, not the arm**:
  - clean probe (`#include <config.h>` + `"lisp.h"` + `#ifdef`/`#ifndef`/`#if !defined
    (HAVE_MULTILINGUAL_MENU)` markers; `clang -E -dM`) -> only `#define D2 1` (the `#ifndef`) and
    `#define D3 1` (the `#if !defined`) survive, so the C arms are live and the port follows
    clang;
  - the artefact, reproduced minimally: `#include <config.h>` + `@` + `#define M1 1` ->
    `M1` **is not** in `clang -dM -E`'s dump, while `#include <config.h>` + `#define M1 1` + `@`
    keeps it, and `#ifdef M1` + `#error` proves the macro is defined in both.  The `@` of
    `@implementation EmacsMenu` (C 460) is the first one in the file, which is exactly why the
    three conditions after it (956, 1006, 1059) are the only ones recorded dead.
- `python3 tools/check-citations.py` -> the last line moves with the other lanes' in-flight edits
  (other files in this tree are being written concurrently); **zero rows name
  `femacs/platform/macos/nsmenu.swift`**.  An ad-hoc cross-check (not a repo tool: for each
  `/// C: <file> <line>` in the file, does the snippet the citation names appear within +-3 lines
  of the line it cites?) now reports 0 findings; it is what found the six citations that pointed
  at the wrong line - `safe_run_hooks` (keyboard.h 507 -> lisp.h 5097), `build_string` (lisp.h
  4552 -> 4574), `list1` (4470 -> 4486), `STRING_MULTIBYTE` (character.h 388 -> lisp.h 1601),
  `createToolbar` (nsterm.h 457 -> 440) and `record_unwind_protect_void` (lisp.h 4814 -> 4825) -
  all corrected above.
- `grep -w` of all 139 imported names over `emacs/{src,lib,lwlib}` -> the only names with no
  occurrence are the port's deliberately-renamed accessors (`frame_left_pos`, `frame_top_pos`,
  `frame_menu_bar_items_used`, `frame_set_menu_bar_items_used`, `frame_menu_bar_vector`,
  `frame_n_tool_bar_items`, `frame_tool_bar_items`, `ns_output_in_animation`,
  `ns_output_set_in_animation`, `set_waiting_for_input_flag`, `image_load_failed_p`,
  `image_pixmap`, `window_contents`) and the generated `Q…`/`QFcar`/`QFcdr` class, i.e. no
  invented name remains.
- `python3 tools/check-declarations.py` -> `13 declaration sites (0 forbidden, 13 allowed
  literals), 0 failures` (none of them this unit).
- `python3 tools/check-verbatim.py` -> `6 trees, 4607 files, 0 failures`.
- `python3 tools/check-mirror-dupes.py` -> `1 duplicate declaration(s), 1 ratchet row(s),
  0 failure(s)` (neither is this unit).
- `/tmp/nm` (this lane's probe directory) was deleted after these readings.
