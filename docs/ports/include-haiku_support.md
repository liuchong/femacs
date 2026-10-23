# emacs/src/haiku_support.h (784 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] _HAIKU_SUPPORT_H  (line 20, body da39a3ee5e6b)

## Types and their fields — 29
- [x] haiku_activation_event: void* window, int activated_p
- [x] haiku_app_quit_requested_event: char dummy
- [x] haiku_button_event: void* window, void* scroll_bar, int btn_no, int modifiers, int x, int y, bigtime_t time
- [x] haiku_clipboard_changed_event: char dummy
- [x] haiku_drag_and_drop_event: void* window, int x, int y, void* message
- [x] haiku_dummy_event: char dummy
- [x] haiku_expose_event: void* window, int x, int y, int width, int height
- [x] haiku_font_change_event: haiku_font_family_or_style new_family, haiku_font_family_or_style new_style, int new_size, enum haiku_what_font what
- [x] haiku_font_pattern: int specified, struct haiku_font_pattern* next, struct haiku_font_pattern* last, struct haiku_font_pattern* next_family, haiku_font_family_or_style family, haiku_font_family_or_style style, int mono_spacing_p, enum haiku_font_slant slant, enum haiku_font_width width, enum haiku_font_language language, enum haiku_font_weight weight, int* wanted_chars, int want_chars_len, int* need_one_of, int need_one_of_len, int family_index, int style_index, int oblique_seen_p, int use_antialiasing
- [x] haiku_iconification_event: void* window, int iconified_p
- [x] haiku_key_event: void* window, int modifiers, unsigned keysym, uint32_t multibyte_char, bigtime_t time
- [x] haiku_menu_bar_click_event: void* window, int x, int y
- [x] haiku_menu_bar_help_event: void* window, int mb_idx, void* data, bool highlight_p
- [x] haiku_menu_bar_left_event: void* window, int x, int y
- [x] haiku_menu_bar_resize_event: void* window, int width, int height
- [x] haiku_menu_bar_select_event: void* window, void* ptr
- [x] haiku_menu_bar_state_event: void* window
- [x] haiku_mouse_motion_event: void* window, bool just_exited_p, int x, int y, bigtime_t time, bool dnd_message
- [x] haiku_move_event: void* window, int x, int y, int decorator_width, int decorator_height
- [x] haiku_notification_click_event: intmax_t id
- [x] haiku_quit_requested_event: void* window
- [x] haiku_resize_event: void* window, float width, float height
- [x] haiku_screen_changed_event: bigtime_t when
- [x] haiku_scroll_bar_drag_event: void* scroll_bar, void* window, int dragging_p
- [x] haiku_scroll_bar_part_event: void* scroll_bar, void* window, enum haiku_scroll_bar_part part
- [x] haiku_scroll_bar_value_event: void* scroll_bar, void* window, int position
- [x] haiku_session_manager_reply: bool quit_reply
- [x] haiku_wheel_move_event: void* window, int modifiers, float delta_x, float delta_y
- [x] haiku_zoom_event: void* window, int fullscreen_mode

## File-scope objects — 3
- [x] extern port_id port_application_to_emacs  (line 514)
- [x] extern port_id port_popup_menu_to_emacs  (line 515)
- [x] extern port_id port_emacs_to_session_manager  (line 516)

## Decisions

- **The live text is carried whole; the four dead regions are recorded, not transcribed**
  (PORTING.md P-08, the `thread.rs`/`gnutls.rs` shape).  The frozen macro table decides every
  `#if` this header has (`tools/check-cond-eval.py src/haiku_support.h` agrees with clang on all of
  them), so the dead regions are: the `#ifdef __cplusplus` head (C 34-37's `extern "C" {`, C 467-500's
  `BE_RECT_HEIGHT`/`BE_RECT_WIDTH` plus their comment, C 507-512's `typedef void *haiku` and
  `haiku_put_pixel`/`haiku_get_pixel`), `#ifdef HAVE_FREETYPE`'s two includes (C 26-28),
  `#ifdef HAVE_NATIVE_IMAGE_API`'s three prototypes (C 705-709), `#ifdef USE_BE_CAIRO`'s four
  prototypes and their `cairo_t` (C 714-719) and the `#ifdef __cplusplus` tail (C 773-778: the
  closing brace, `_Noreturn gui_abort`, `find_appropriate_view_for_draw`).  Carrying the head's
  `typedef void *haiku` would be an `E0428` against `haikugui.rs`, which declares it unconditionally.
- **`<kernel/OS.h>` has no mirror** (a Haiku system header, like gnulib's `<gmp.h>` was for
  `bignum.rs`, PORTING.md P-49/P-70): the four names the live text uses are declared at the top of
  the mirror with Haiku's widths - `int32`/`status_t`/`team_id`/`port_id` (`int32_t`) and
  `bigtime_t` (`i64`).  `<stdint.h>` (C 22) reaches the port's own mirror
  (`lib/include/stdint.in.rs`, for `int32_t`/`uint32_t`), `ssize_t` comes from
  `lib/include/unistd.in.rs` (C's OS.h chain reaches `<sys/types.h>`; the port declares `ssize_t`
  there - measured: no `src/` mirror declares it) and `struct timespec` from
  `lib/include/time.in.rs` (the same chain reaches `<time.h>`).  `<math.h>` (C 30) is not mirrored
  into the include list: the only thing the header takes from it is `ceil`, inside the dead
  `BE_RECT_*` macros.
- **`size_t` and `ptrdiff_t` are used unqualified**, as every other `src/` mirror does
  (`buffer.rs`, `androidterm.rs`): C gets them from `<stddef.h>` through lisp.h, and the port's
  declarations live in `lisp.rs`, which no `src/` mirror includes (PORTING.md 0.1-9) - every unit
  that carries this mirror has lisp.rs in its own include list.  The standalone compile proof below
  therefore supplies those two aliases the way a unit does.
- **`intmax_t id`** in `struct haiku_notification_click_event` (C 470) is spelled `i64`:
  PORTING.md 1.1 maps `intmax_t` to `i64`, and the port's alias lives in `lisp.rs`, which this
  header does not include.
- **`typedef char haiku_font_family_or_style[64]`** (C 220) is `pub type haiku_font_family_or_style
  = [c_char; 64];` - the type *is* the array, so the struct fields embed it by value (C's
  `sizeof` 64) and the prototype uses stay `*mut haiku_font_family_or_style`.
- **The thirteen enums are `pub type NAME = c_int;` plus one `pub const` per member**, the shape
  `termhooks.rs`/`dispextern.rs`/`atimer.rs` use for an enum C treats as an integer.  Members with
  explicit values keep them (`CURSOR_ID_*` is not declaration order); `FSPEC_*` and
  `HAIKU_MODIFIER_*` keep C's `1 << n` expressions.
- **The 178 `extern` declarations are carried in full** (`pub mod haiku_support_decls` + glob
  import, PORTING.md 0.1-10/P-17/P-21: `haiku_support.cc`'s port is the defining unit), even though
  `extract-abi.py` does not inventory prototypes (`cinventory.functions` is definitions only, which
  is why this ledger says "Functions defined - 0").  Unnamed C parameters become `arg0`, `arg1`, …
  (the `pdumper.rs` 83 precedent); function-pointer parameters become
  `Option<unsafe extern "C" fn (…) -> …>`; `double (*)[3]` becomes `*mut [f64; 3]`.  Because no gate
  compares this block, the transcription was verified mechanically name-for-name against the
  reference text (178 of 178 declarations, no extras).
- **`MAYBE_UNUSED`-style attributes and the file's local-variables block**: the header's trailing
  `// Local Variables:` block (C 780-782) is carried verbatim as comments (PORTING.md 0.3).

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/haiku_support.h src/include/haiku_support.rs`
  -> `OK emacs/src/haiku_support.h -> src/include/haiku_support.rs` (no `- ` problem lines).
- `tools/check-cond-eval.py src/haiku_support.h` -> every condition decided, 0 disagreements.
- Compile proof (the mirror has no unit that includes it yet): a probe crate that `include!`s this
  mirror, supplies `c_int`/`c_char`/`c_ushort`/… and the two lisp.h aliases a unit provides
  (`size_t`, `ptrdiff_t`), and adds `#![allow(…, static_mut_refs, clashing_extern_declarations)]`,
  compiles clean:
  `rustc --edition 2024 --crate-name mir_probe_haiku_support -C panic=abort -o /tmp/mir-probe-haiku_support /tmp/mir-probe-haiku_support.rs` -> exit 0.
  It also runs as part of `xtask layout-check`, which compiles every `src/include` mirror into one
  crate.
- Consumer smoke: `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-mir-haiku_support cargo run -q
  -p xtask -- build-tu src/haikuterm.c` -> `built src/haikuterm.rs -> ...` (the platform unit is the
  empty unit plus its note, docs/status.md 3.31).
- Gate readings (repo root): `check-declarations` 0 failures; `check-verbatim` 6 trees / 0;
  `check-mirror-dupes` 0 failures; `check-globals` 0.

- **This mirror's compile evidence is the Haiku platform probe, not `xtask layout-check`**
  (`spikes/haiku_inc_probe.rs`, the `nt/inc`/`f_inc_probe.rs` shape, docs/status.md 3.102).
  `layout-check` compiles *every* `src/include` mirror into one crate, which by construction cannot
  hold the platform-exclusive mirrors: `haikugui.h` and `androidgui.h` declare the same X-compat
  block (`NativeRectangle`, `STORE_NATIVE_RECT`, the gravity/value/position constants) and the same
  `XCharStruct`/`Emacs_Pixmap`/`Emacs_Window`/`Emacs_Cursor`/`Drawable` aliases, and C includes only
  one of the two behind its platform `#if` (`HAVE_HAIKU` vs `HAVE_ANDROID`).  The probe compiles
  `haikugui.rs` and `haiku_support.rs` together with the names a Haiku translation unit supplies
  (`size_t`/`ptrdiff_t` from lisp.h, the `c_*` aliases from the unit's prelude).  **This is a
  probe-scope record, not a defect of the mirror**: no `#[cfg (feature = …)]` was added (a cfged
  item would lose every compiler, status 3.141 ④'s reasoning) and no `pub mod` wrapper (it would
  hide the `macro_rules!` names - they are textually scoped).  Whether `layout-check` learns to
  skip the platform-exclusive mirrors is the integrator's call.
  Evidence: `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-mir-probe cargo run -q -p xtask --
  probe ../spikes/haiku_inc_probe.rs ../build/probes/haiku_inc_probe.rmeta --crate-type=lib
  --emit=metadata` -> exit 0; and on the materialised root,
  `rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type=lib --emit=metadata -o
  /tmp/haiku_inc_probe.rmeta build/probes/haiku_inc_probe/_root.rs` -> exit 0.
- **The `#ifdef __cplusplus` pair this mirror records is live in `include/haikuterm.rs`**:
  `haiku_put_pixel`/`haiku_get_pixel` (C 507-512 here) are declared unconditionally by
  `haikuterm.h` 340-341, which is the declaration a Haiku unit gets, so nothing is lost by this
  header's arm being dead text on the frozen target.

