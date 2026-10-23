# emacs/src/xwidget.h (243 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 6
- [x] syms_of_xwidget  (line 233)
- [x] valid_xwidget_spec_p  (line 234)
- [x] xwidget_view_delete_all_in_window  (line 235)
- [x] x_draw_xwidget_glyph_string  (line 236)
- [x] xwidget_end_redisplay  (line 238)
- [x] kill_buffer_xwidgets  (line 239)

## Macros defined — 11
- [x] CHECK_LIVE_XWIDGET  (line 168, body 6f91961ef7e0)
- [x] CHECK_XWIDGET  (line 165, body 041252ef83f5)
- [x] CHECK_XWIDGET_VIEW  (line 178, body cf835e3c2bbd)
- [x] XG_XWIDGET  (line 181, body 9320cae87b04)
- [x] XG_XWIDGET_VIEW  (line 182, body 41381d414947)
- [x] XWIDGETP  (line 159, body f7c48f6d69b1)
- [x] XWIDGET_H_INCLUDED  (line 21, body da39a3ee5e6b)
- [x] XWIDGET_LIVE_P  (line 163, body 6905d677251a)
- [x] XWIDGET_VIEW_P  (line 174, body fe3910e8a67d)
- [x] XXWIDGET  (line 160, body 229be4342756)
- [x] XXWIDGET_VIEW  (line 175, body 62a7f13a5d86)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The header's live/dead split is `HAVE_XWIDGETS`.**  `HAVE_XWIDGETS` is `undef` in
  `docs/baseline/config/darwin-arm64.macros.tsv`, so C compiles the guard, the include, the five
  forward declarations, C 158-182's twelve accessors (outside both guards) and C 232-240's `#else`
  inline stubs; C 31-156 and C 184-230 are dead text and carry
  `#[cfg (feature = "have_xwidgets")]` - P-51's arm for the non-frozen side, the
  `include/macfont.rs` (3.170) / `include/w32font.rs` (3.176) shape.  `dispextern.rs` 355 and
  `print.rs` 2048-2065 name the same feature for the same record.
- **The dead arm keeps C's platform names.**  `GtkWidget`, `GdkWindow`, `guint`, `NSView`, `Window`,
  `Display`, `cairo_surface_t`, `cairo_t`, `Time`, `XEvent`, `XIGesturePinchEvent` are GTK's, X11's
  and AppKit's, and the platform target supplies them (the `include/ftfont.rs` 9-18 reading).  C's
  four inner conditions are nested `#[cfg]`s: `use_gtk`/`have_pgtk` for C 33-40 and 118-130,
  `ns_impl_cocoa` for the `#elif` arms, `have_x_windows` for C 208-230, `have_xinput2` and
  `have_xinput2_4` for the two nested `#ifdef`s inside it.  **The last two are new names in this
  tree** (the C macro's own spelling, lowercased - status 3.171's rule); they need a row in
  `xtask/Cargo.toml`'s `[features]` table like every other name there.
- **No layout assertion, and one bitfield is recorded rather than packed.**  The two records hold GTK
  and AppKit members, so no compiler on this target can measure them (`include/macfont.rs` 26-27)
  and the `GCALIGNED_STRUCT` spelling is `#[repr(C, align(8))]` as `include/comp.rs` 18-19 writes it.
  C 99's `bool_bf kill_without_query : 1` is recorded in a `//` line inside the record: PORTING.md
  1.3 / P-80 require the real compiler's byte dump before a storage unit may stand for a bitfield,
  and this record cannot be compiled here (the `include/emacsgtkfixed.rs` deferral).
- **`__OBJC__` halves are recorded, not mirrored.**  C 83 and 138's `#ifdef __OBJC__` members
  (`NSView *xwWidget`, `XwWindow *xwWindow`, `XvWindow *xvWindow`, `NSView *emacswindow`) are
  Objective-C's; this mirror is C text, so the arm's own `#else` half is what it carries, the
  `include/nsgui.rs` 16-27 reading of `__OBJC__`.
- **Degraded names (P-55/P-72 rule 3).**  `struct glyph_matrix`, `struct glyph_string` and
  `struct window` are dispextern.h's and window.h's and this mirror includes neither, so every
  pointer to one is `*mut c_void` with C's text in a `/// C:` line; `struct frame` is the same (the
  header does not include frame.h either).  `struct xwidget`/`struct xwidget_view` *are* this
  header's own records under the arm above, so the prototypes name them.
- **Four of the twelve accessors carry the record's feature.**  C defines `XXWIDGET`,
  `XWIDGET_LIVE_P`, `XXWIDGET_VIEW` and `CHECK_LIVE_XWIDGET` outside the guard, but their expansion
  reads `struct xwidget`, which C only completes under `HAVE_XWIDGETS`; as Rust functions they can
  only exist where the record does.  The tree's own consumer agrees with that shape: `print.rs`
  2050-2065, inside its `have_xwidgets` arm, calls `XXWIDGET (obj)` as a function and reads
  `.buffer`/`.xwidget_id`/`.widget_osr`/`.xwWidget`.
- **`lookup_xwidget`'s impossible return value is `emacs_abort ()` plus a null pointer.**  C's body
  is `eassume (0)` and falls off the end (UB); the port reports through `eassume!` and then takes the
  `include/frame.rs` 240 shape.  `unreachable! ()` is deliberately not used: the checker's stub rule
  (`STUB_RE`, `--check`) matches it at the end of a body, and `xdisp.rs` 1112 uses it for this same
  C line - see the note in this ledger's Evidence.
- **The mirror's seven `#else` stubs are the reference's own definitions.**  Each C body is empty or
  `eassume (0)`, and the port keeps them (with `let _ = …` for the empty ones, the
  `include/comp.rs` 48 idiom) so a unit that includes the mirror gets the same no-op C gives it.
  The units that today keep their own copies - `buffer.rs` 151-156, `window.rs` 5427-5432,
  `xdisp.rs` 1095-1112, `print.rs` 92, `emacs.rs` 316-317 (`syms_of_xwidget`) and `dispnew.rs`
  515-521 (`xwidget_end_redisplay`, as a macro) - do not include this mirror, so nothing changes for
  them; converging them is a separate call for their owners.

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/xwidget.h src/include/xwidget.rs`
  -> **FAIL** with these seven lines, and nothing else:
  `function syms_of_xwidget does not mention syms_of_xwidget`, `valid_xwidget_spec_p ...`,
  `xwidget_view_delete_all_in_window does not mention window, xwidget_view_delete_all_in_window`,
  `x_draw_xwidget_glyph_string does not mention 0, glyph_string, ...`, `xwidget_end_redisplay does
  not mention glyph_matrix, window, ...`, `kill_buffer_xwidgets does not mention
  kill_buffer_xwidgets`, `stub or empty body in fn syms_of_xwidget`.  The cause is measured: for
  C 233-239 the definitions are **one line each**, so `function_spans` reports `body_start ==
  body_end` and the compared "body" is the whole line - the check then asks the Rust *function body*
  for the function's own name and its parameter type names, which a Rust body cannot carry.  The
  committed `dispextern.rs` 113 has the identical verdict for `GLYPH_CHAR`/`GLYPH_FACE`
  (`functions GLYPH_CHAR does not mention glyph_char`), and its ledger still has unchecked boxes, so
  the tree had no passing shape for this before this file.
- With `docs/ports/macro-body-notes.tsv` extended by the seven rows below,
  `python3 tools/extract-abi.py --check emacs/src/xwidget.h src/include/xwidget.rs`
  -> `OK emacs/src/xwidget.h -> src/include/xwidget.rs`.  The rows are requested in this
  round's report (the shared table is written by the integrator):
  `syms_of_xwidget`, `valid_xwidget_spec_p`, `xwidget_view_delete_all_in_window`,
  `x_draw_xwidget_glyph_string`, `lookup_xwidget`, `xwidget_end_redisplay`, `kill_buffer_xwidgets`,
  each with `*` and the reason "xwidget.h 233-239: one-line INLINE definition - the tool's span is
  the whole line".
- `python3 tools/extract-abi.py emacs/src/xwidget.h` -> this ledger: 0 subrs, 6 functions defined,
  11 macros, 0 types, 0 file-scope objects (the two records are inside the dead guard).
- **No unit includes this mirror** (`rg -n 'include/xwidget' femacs` -> no hits); it is compiled by
  `xtask layout-check`'s single crate (both platform arms are inert there).
- `CARGO_TARGET_DIR=/tmp/femacs-target-s188 cargo run -q -p xtask -- layout-check` ->
  `layout-check: 87 of 98 mirror(s) compiled, 11 platform-exclusive skipped (...)`, `layout probe ok`,
  `layout-check passed`.
- `build-tu src/xwidget.h` -> `error[E0425]: cannot find value 'HAVE_PDUMPER' in this scope` - the
  same class as `src/textconv.h` and the committed `src/charset.h` (measured): a mirror whose
  include chain reaches `lisp.rs` wants the `config.rs` constants a unit includes first.  The
  compile evidence for this mirror is `layout-check`.
- Gates on the same worktree: `check-declarations` 13 sites (0 forbidden, 13 allowed literals) / 0
  failures; `check-target-names` 612 .rs files / 593 map targets / 0 findings; `check-mirror-dupes`
  1 duplicate declaration(s) / 1 ratchet row(s) / 0 failures.
