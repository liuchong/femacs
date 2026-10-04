# emacs/src/nsterm.h (1395 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **Every declaration in this file is behind `#ifdef HAVE_NS` (C 29) and - for C 30-785 alone -
  `#ifdef __OBJC__`.**  The file is Objective-C (`/* -*- objc -*- */`), the frozen configuration
  has `HAVE_NS`, `NS_IMPL_COCOA`, `NS_IMPL_GNUSTEP` and `__OBJC__` all `undef`
  (`docs/baseline/config/darwin-arm64.macros.tsv`), and `tools/check-cond-eval.py src/nsterm.h`
  -> `68 conditions, 0 unknown, 0 disagreements [VERIFIED]`, so every `#if` is the frozen
  configuration's verdict.  **The tool's live inventory for this file is therefore empty** - which
  is what this ledger prints (0 macros, 0 constants, 0 fields, 0 objects) - while the
  all-branches view the map uses counts **140 macros, 21 constants, 102 struct fields and 8
  objects** (`docs/port-inventory.md` 569; `tools/cinventory.py` on this file, without the macro
  table).  The mirror carries the map's view: the fidelity here is C's text, not the gate
  (`--check` requires nothing), the `include/w32gdiplus.md` situation at 1,395 lines.
- **The split follows `include/macfont.rs` (3.170) and `include/w32font.rs` (3.176).**  Everything
  from C 29 on carries `#[cfg (feature = "have_ns")]` - P-51's arm for the non-frozen side - and
  spells C's own platform names (`CGFloat`, `NSRect`, `NSString`, `NSColor`, `NSView`, `CGPoint`,
  `CGContextRef`, `id`, `NSUInteger`, `BOOL`): they arrive with the macOS/GNUstep target, whose
  bindings are that batch's job.  **No layout assertion is written** for any record, because every
  one of them holds AppKit/GNUstep/CoreGraphics members and no compiler on this target could
  measure them (the `include/macfont.rs` 26-27 reading); nothing is degraded to `*mut c_void`
  either, since the whole body is inert here.
- **The inner conditions.**  `NS_IMPL_COCOA` / `NS_IMPL_GNUSTEP` are the features of the same
  name.  The `MAC_OS_X_VERSION_MIN_REQUIRED`/`MAX_ALLOWED`/`MAC_OS_X_VERSION_10_*` tests are
  *SDK-version* tests, not target switches: the arm a Cocoa build takes is carried and the test is
  written in the `/// C:` line, while the five fallback blocks (C 1301-1391) - which a Cocoa build
  against a current SDK never takes - carry `ns_impl_gnustep`, the side that does take them.
  `#ifdef HAVE_PTHREAD` (C 1250) is **1** in the frozen configuration (macros.tsv 1502), so that
  declaration is live and carried unconditionally (the `include/syssignal.rs` 56 reading of the
  same macro); `#ifndef NSAppKitVersionNumber10_7/10_10` (C 1290-1296) are "define if the SDK
  lacks it" guards and are carried as constants (the `include/widget.rs` `XtNfont` reading).
- **What is recorded, not transcribed** (each region says so at its C line in the mirror): the 19
  `@interface`/`@class` blocks of C 350-785 with their `{ ... }` ivars and method declarations
  (Objective-C class syntax is not expressible as a Rust item at all); every `#ifdef __OBJC__`
  half (`EmacsImage *img` C 814, `NSFont *nsfont` C 839, `EmacsScroller *last_mouse_scroll_bar`
  C 925, the nine `ns_output` pointers C 937-957, the `id`/`NSColor **`/`NSPoint`/`EmacsToolbar *`
  prototypes C 1132-1178/1351, `typedef NSUInteger NSWindowStyleMask` C 1351-1353) - C's `#else`
  half is carried where there is one, the `include/nsgui.rs` 16-27 reading and this round's rule;
  the three macro bodies that are Objective-C message sends (`FRAME_NS_TITLEBAR_HEIGHT` C 1062,
  `FRAME_TOOLBAR_HEIGHT` C 1071, `NS_SCROLL_BAR_WIDTH_DEFAULT`/`HEIGHT_DEFAULT` C 1274-1275) and
  the two `@"..."` colour constants (C 1278-1279); the dead-by-construction `NSTRACE`
  implementation (C 146-295 is inside `#if NSTRACE_ENABLED`, which C 142-144 defines to 0 in this
  very file, and it uses `__attribute__((cleanup))`), whose names the mirror lists; and the
  `typedef id instancetype;` of C 347-349.
- **The 102 fields are C's, in C's order**, and the count is the all-branches one: each
  `#ifdef __OBJC__` member is counted twice by that view (`ns_bitmap_record` 7, `nsfont_info` 15,
  `ns_display_info` 29, `ns_output` 50, `x_output` 1), so the mirror's five records carry C's 6,
  14, 28, 41 and 1 members on the arm this target would take.
- **Degraded names (P-55/P-72 rule 3).**  `struct image`, `struct input_event` and
  `struct glyph_string` are pointed at but not defined by this header, so those parameters keep
  `*mut c_void` with C's prototype in the `/// C:` line; `ns_frame_parm_handlers[]`'s elements are
  `*mut c_void` until `include/dispextern.rs` carries `frame_parm_handler` (the
  `include/androidterm.rs` 506-511 reading of its own array); `sigset_t` is spelled as C spells it
  and no mirror in this tree declares it yet (`lib/include/` has `fd_set`, `timespec` and
  `signal`'s handlers - a system-mirror follow-up, not a stand-in guessed here).  `Window`, `Time`,
  `Mouse_HLInfo`, `Emacs_Cursor` and `fd_set` are other mirrors' names: `nsgui.h` 67 and
  `nsgui.h` 75-88 (reached through `dispextern.h` 142-148), `systime.h` (through `termhooks.h`),
  `dispextern.rs` and `lib/include/sys_select.rs`.

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/nsterm.h femacs/src/include/nsterm.rs`
  -> `OK emacs/src/nsterm.h -> femacs/src/include/nsterm.rs` (no `- ` problem lines).
- `python3 tools/extract-abi.py emacs/src/nsterm.h` -> this ledger: 0 subrs, 0 functions defined,
  0 macros, 0 constants, 0 types, 0 objects (the guards above).  The all-branches view, for the
  record: `python3 -c 'import importlib.util as u; from pathlib import Path;
  s=u.spec_from_file_location("ci","tools/cinventory.py"); ci=u.module_from_spec(s);
  s.loader.exec_module(ci); i=ci.inventory(Path("emacs/src/nsterm.h"));
  print(len(i["macros"]), len(i["constants"]), len(i["struct_fields"]), len(i["globals"]))'`
  -> `140 21 102 11` (run from the repository root, one line).
- `python3 tools/check-cond-eval.py src/nsterm.h` -> `68 conditions, 0 unknown, 0 disagreements
  [VERIFIED]`.
- **No unit includes this mirror** (`rg -n 'include/nsterm' femacs` -> no hits).  The C readers are
  `callproc.c`, `lread.c`, `systhread.c` and `thread.c` plus the nine `.m` files (`nsfns.m`,
  `nsfont.m`, `nsimage.m`, `nsmenu.m`, `nsselect.m`, `nsterm.m`, `nsxwidget.m`, `macfont.m`), none
  of which is ported yet.
- `CARGO_TARGET_DIR=/tmp/femacs-target-s188 cargo run -q -p xtask -- layout-check` ->
  `layout-check: 90 of 100 mirror(s) compiled, 10 platform-exclusive skipped (...)`,
  `layout probe ok`, `layout-check passed` (this mirror is among the 90; the `have_ns` arm is
  inert there).
- `build-tu src/nsterm.h` -> `error: cannot find macro 'eassert' in this scope` (then `XUNTAG`,
  `BASE_EQ`, `CHAR_TABLE_REF`, `emacs_abort`): the headers C 22-27 includes reach `lisp.h`'s macros
  and `config.rs`'s constants only through the *unit's* own includes (PORTING.md 0.1-9), which is
  the class every header mirror measured this round shares (`src/charset.h` fails the same way, as
  do `src/widget.h`, `src/w32font.h`, `src/w32gui.h`, `src/nsxwidget.h`, `src/textconv.h`,
  `src/xwidget.h`, `src/w32gdiplus.h` and `src/pgtkgui.h`).  The compile evidence for this mirror
  is `layout-check`.
- **Name-for-name check against the all-branches inventory** (the `include/haiku_support.md`
  reading): every one of the 140 macro names, 21 constant names, 11 object names and 102 field
  names is present in the mirror, and the command is re-runnable from the repository root:
  `python3 -c 'import importlib.util as u, re; from pathlib import Path;
  s=u.spec_from_file_location("ci","tools/cinventory.py"); ci=u.module_from_spec(s);
  s.loader.exec_module(ci); i=ci.inventory(Path("emacs/src/nsterm.h"));
  rs=Path("femacs/src/include/nsterm.rs").read_text();
  print([n for n in list(i["macros"])+list(i["constants"])+[g["name"] for g in i["globals"]]
  +[f["name"] for f in i["struct_fields"]] if not re.search(r"\\b"+re.escape(n)+r"\\b", rs)])'`
  -> `[]`.
- **One measured Rust rule, found by `layout-check` on the first run of this mirror**: an `extern`
  block cannot hold a `const` (`error: extern items cannot be 'const'` for
  `pub const NSAPP_DATA2_RUNASSCRIPT`), so C 1188/1192's two constants are written between two
  `unsafe extern "C"` blocks split at C's own lines - C's declaration order is kept rather than
  moving the constants out of the region.
- The mirror is **1,246 lines against the reference's 1,395**: the difference is the recorded
  Objective-C class region (C 350-785's `@interface` blocks and method lists) and the
  dead-by-construction `NSTRACE` implementation (C 146-295), both of which are described instead of
  transcribed.  `--check`'s 5% line rule does not fire for header mirrors (the
  `include/pgtkterm.rs` 30-of-680 precedent).
- Gates on the same worktree: `check-declarations` 13 sites (0 forbidden, 13 allowed literals) / 0
  failures; `check-target-names` 616 .rs files / 593 map targets / 0 findings; `check-mirror-dupes`
  1 duplicate declaration(s) / 1 ratchet row(s) / 0 failures.
