# emacs/src/androidgui.h (930 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 37
- [x] ANDROID_IS_MODIFIER_KEY  (line 333, body d27cd8452692)
- [x] AllValues  (line 241, body 652902e8f0f1)
- [x] CenterGravity  (line 229, body ac3478d69a3c)
- [x] EastGravity  (line 230, body c1dfd96eea8c)
- [x] ForgetGravity  (line 224, body b6589fc6ab0d)
- [x] HeightValue  (line 240, body 92df71781c5c)
- [x] NativeRectangle  (line 218, body 2725333df576)
- [x] NoValue  (line 236, body bd0d3415ccf6)
- [x] NorthEastGravity  (line 227, body 77de68daecd8)
- [x] NorthGravity  (line 226, body da4b9237bacc)
- [x] NorthWestGravity  (line 225, body 356a192b7913)
- [x] PAspect  (line 252, body f5a08b8b3015)
- [x] PBaseSize  (line 253, body 08b008cb3ba4)
- [x] PMaxSize  (line 250, body 2a0a8ece4dfd)
- [x] PMinSize  (line 249, body 7f2dac477a2d)
- [x] PPosition  (line 247, body b56fff76c324)
- [x] PResizeInc  (line 251, body b307cdcc94a2)
- [x] PSize  (line 248, body a33785b93413)
- [x] PWinGravity  (line 254, body 148030b1dc2b)
- [x] STORE_NATIVE_RECT  (line 220, body ba4936021147)
- [x] SouthEastGravity  (line 233, body 0ade7c2cf97f)
- [x] SouthGravity  (line 232, body fe5dbbcea5ce)
- [x] SouthWestGravity  (line 231, body 902ba3cda188)
- [x] StaticGravity  (line 234, body b1d5781111d8)
- [x] UI_MODE_NIGHT_MASK  (line 605, body a25faa96135b)
- [x] UI_MODE_NIGHT_NO  (line 606, body 40e556b78e97)
- [x] UI_MODE_NIGHT_UNDEFINED  (line 608, body 595760b278e9)
- [x] UI_MODE_NIGHT_YES  (line 607, body 12487289f045)
- [x] USPosition  (line 245, body 1e2ed944d274)
- [x] USSize  (line 246, body 72fd47b601c4)
- [x] WestGravity  (line 228, body 1b6453892473)
- [x] WidthValue  (line 239, body eeed498aee81)
- [x] XNegative  (line 242, body d73afeb43f0b)
- [x] XValue  (line 237, body f2b1649be8f5)
- [x] YNegative  (line 243, body 19fbaf699b05)
- [x] YValue  (line 238, body 5f399b2ba212)
- [x] _ANDROID_GUI_H_  (line 20, body da39a3ee5e6b)

## Types and their fields — 29
- [x] android_any_event: enum android_event_type type, unsigned long serial, android_window window
- [x] android_button_event: enum android_event_type type, unsigned long serial, android_window window, int x, int y, unsigned long time, unsigned int state, unsigned int button
- [x] android_char_struct: int rbearing, int lbearing, int width, int ascent, int descent
- [x] android_compose_status: unsigned int accent, int chars_matched
- [x] android_configuration_changed_event: enum android_event_type type, unsigned long serial, android_window window, enum android_configuration_change_type detail, double dpi_x, double dpi_y, double dpi_scaled, <aggregate> pixel_density, int ui_mode, <aggregate> u
- [x] android_configure_event: enum android_event_type type, unsigned long serial, android_window window, android_time time, int x, int y, int width, int height
- [x] android_crossing_event: enum android_event_type type, unsigned long serial, android_window window, int x, int y, unsigned long time
- [x] android_dnd_event: enum android_event_type type, unsigned long serial, android_window window, int x, int y, unsigned short* uri_or_string, size_t length
- [x] android_event: enum android_event_type type, struct android_any_event xany, struct android_key_event xkey, struct android_configure_event xconfigure, struct android_focus_event xfocus, struct android_window_action_event xaction, struct android_crossing_event xcrossing, struct android_motion_event xmotion, struct android_button_event xbutton, struct android_expose_event xexpose, struct android_touch_event touch, struct android_wheel_event wheel, struct android_iconify_event iconified, struct android_menu_event menu, struct android_ime_event ime, struct android_dnd_event dnd, struct android_notification_event notification, struct android_configuration_changed_event config
- [x] android_expose_event: enum android_event_type type, unsigned long serial, android_window window, int x, int y, int width, int height
- [x] android_focus_event: enum android_event_type type, unsigned long serial, android_window window, android_time time
- [x] android_gc: struct android_rectangle* clip_rects, int num_clip_rects, android_gcontext gcontext, unsigned long foreground, unsigned long background, enum android_gc_function function, enum android_fill_style fill_style, int clip_x_origin, int clip_y_origin, android_pixmap clip_mask, android_pixmap stipple, int ts_x_origin, int ts_y_origin, enum android_line_style line_style, int line_width, int dash_offset, int* dashes, int n_segments
- [x] android_gc_values: unsigned long foreground, unsigned long background, enum android_gc_function function, enum android_fill_style fill_style, int clip_x_origin, int clip_y_origin, android_pixmap clip_mask, android_pixmap stipple, int ts_x_origin, int ts_y_origin, enum android_line_style line_style, int line_width, int dash_offset, int dash
- [x] android_iconify_event: enum android_event_type type, unsigned long serial, android_window window
- [x] android_image: int width, int height, enum android_image_format format, char* data, int depth, int bytes_per_line, int bits_per_pixel
- [x] android_ime_event: enum android_event_type type, unsigned long serial, android_window window, enum android_ime_operation operation, ptrdiff_t start, ptrdiff_t end, ptrdiff_t position, size_t length, unsigned short* text, unsigned long counter
- [x] android_key_event: enum android_event_type type, unsigned long serial, android_window window, android_time time, unsigned int state, unsigned int keycode, unsigned int unicode_char, unsigned long counter
- [x] android_menu_event: enum android_event_type type, unsigned long serial, android_window window, int menu_event_id, int menu_event_serial
- [x] android_motion_event: enum android_event_type type, unsigned long serial, android_window window, int x, int y, unsigned long time
- [x] android_notification_event: enum android_event_type type, unsigned long serial, android_window window, char* tag, unsigned short* action, size_t length
- [x] android_point: int x, int y
- [x] android_rectangle: int x, int y, unsigned width, unsigned height
- [x] android_set_window_attributes: unsigned long background_pixel, bool override_redirect
- [x] android_swap_info: android_window swap_window, enum android_swap_action swap_action
- [x] android_touch_event: enum android_event_type type, unsigned long serial, android_window window, int x, int y, unsigned long time, unsigned int pointer_id, int flags
- [x] android_transform: float m1, float m2, float m3, float m4, float m5, float m6
- [x] android_wheel_event: enum android_event_type type, unsigned long serial, android_window window, int x, int y, unsigned long time, int state, double x_delta, double y_delta
- [x] android_window_action_event: enum android_event_type type, unsigned long serial, android_window window, unsigned int action
- [x] android_window_changes: android_window sibling, enum android_stack_mode stack_mode

## File-scope objects — 2
- [x] static const int ANDROID_NONE  (line 259, static)
- [x] static const int ANDROID_NO_SYMBOL  (line 259, static)

## Decisions

- **The whole file is carried; nothing here is dead text.**  `ANDROID_STUBIFY` is `undef` in the
  frozen macro table, so both `#ifndef ANDROID_STUBIFY` regions (C 256-867 and C 872-928) are live,
  and the header's other conditionals are the two guards (C 19-20, C 930).  `androidgui.h` is in
  `docs/baseline/config/cond-eval-verified.txt`, so `--check` compares exactly this text.
- **The X-compat block** (C 218-254) is a `macro_rules!`/`const` block for `NativeRectangle`,
  `STORE_NATIVE_RECT` and the 29 gravity/value/position macros, all with C's types: the plain
  decimals are `c_int`, and the ten `(1L << n)` members are `c_long` (PORTING.md 1.1).  Rust cannot
  spell `1L`, so the value is `1 << n` - token-identical after normalisation.
  `#define NativeRectangle Emacs_Rectangle` becomes a `macro_rules!` because C uses the name in
  *type* position and `macro_body_of` reads `macro_rules!`/`const`/`fn` only; `Emacs_Rectangle` is
  `dispextern.rs`'s `#ifndef HAVE_WINDOW_SYSTEM` typedef (the arm the frozen target takes), so the
  macro is usable wherever C's is, and the macro body's token is never resolved until it is used,
  as in C.
- **`STORE_NATIVE_RECT`/`NativeRectangle`/the gravity-value-position names repeat `haikugui.rs`'s**
  (and, for `Emacs_Cursor`/`Emacs_Rectangle`, `dispextern.rs`'s): C keeps the twins apart with its
  platform `#if`s, and `xtask layout-check` compiles every `src/include` mirror into one crate, so
  the pairs are an `E0428` until a platform mirror's scoping is decided.  Recorded (with the exact
  name list) at the end of `## Evidence`; no `cfg` or `pub mod` wrapper was invented here.
- **`static const int ANDROID_NONE, ANDROID_NO_SYMBOL;`** (C 259) is C's tentative definition with
  no initialiser, i.e. two internal-linkage ints of value 0.  The mirror uses `pub const …: c_int =
  0;` - the value C's compiler gives them, and no symbol (the C object is `static`).
- **The anonymous union and its nested anonymous struct** in
  `struct android_configuration_changed_event` (C 625-648) are named after their members per
  PORTING.md P-67: `android_configuration_changed_event_u` (the union `u`) and
  `android_configuration_changed_event_u_pixel_density` (the `pixel_density` struct inside it).
  The fields keep C's order and types, so `u` stays 24 bytes at its C offset.
- **Thirteen enums are `pub type NAME = c_int;` plus one `pub const` per member** (the shape
  `termhooks.rs`/`dispextern.rs` use), including the two anonymous ones (C 513-517, C 690-693),
  whose members become plain constants; `ANDROID_CURRENT_TIME = 0L` is `c_long`.
- **`typedef unsigned int android_time`** (C 44) is `c_uint`; `typedef intptr_t android_handle`
  (C 36) keeps `intptr_t`, which the port's `lisp.rs` declares as `isize` (`stdint.in.rs`
  deliberately leaves `intptr_t` to lisp.rs), so `android_handle` and its nine aliases resolve in
  any unit that includes lisp.rs - which every unit that carries this mirror does (PORTING.md
  0.1-9).  `size_t`/`ptrdiff_t` are used unqualified for the same reason.
- **`wchar_t`** (C 812's `android_wc_lookup_string`) reaches the port through
  `lib/include/wchar.rs`, the frozen target's `<wchar.h>` mirror.
- **The 63 prototypes are carried in full** in `pub mod androidgui_decls` + glob import
  (PORTING.md 0.1-10/P-17/P-21), although `extract-abi.py` does not inventory prototypes (its
  `functions` list is definitions only - the ledger's "Functions defined - 0").  Unnamed C
  parameters become `arg0`, `arg1`, … (the `pdumper.rs` 83 precedent) and `bool (*) (union
  android_event *, void *)` becomes `Option<unsafe extern "C" fn (*mut android_event, *mut c_void)
  -> bool>`.  Because no gate compares this block, the transcription was verified mechanically
  name-for-name against the reference (63 of 63).

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/androidgui.h femacs/src/include/androidgui.rs`
  -> `OK emacs/src/androidgui.h -> femacs/src/include/androidgui.rs` (no `- ` problem lines).
- Compile proof (no unit includes this mirror yet): a probe crate that `include!`s it, supplies
  `c_int`/`c_ushort`/…, the three lisp.h aliases a unit provides (`size_t`, `ptrdiff_t`,
  `intptr_t`) and the `Emacs_Rectangle` record the `NativeRectangle` macro names, compiles clean:
  `rustc --edition 2024 --crate-name mir_probe_androidgui -C panic=abort -o /tmp/mir-probe-androidgui /tmp/mir-probe-androidgui.rs` -> exit 0.
  `include/androidterm.rs` (the `androidterm.h` mirror) `include!`s this file, so
  `xtask layout-check` compiles it as part of that mirror and of `android.rs`.
- Consumer smoke: `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-mir-androidgui cargo run -q
  -p xtask -- build-tu src/androidterm.c` -> `built src/androidterm.rs -> ...` (the platform unit is
  the empty unit plus its note, docs/status.md 3.31).
- Gate readings (repo root): `check-declarations` 0 failures; `check-verbatim` 6 trees / 0;
  `check-mirror-dupes` 0 failures; `check-globals` 0.

- **The `layout-check` collision, measured** (`cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-mir-probe cargo run -q -p xtask -- layout-check` -> `layout probe failed to compile (exit status: 1)`, 35 x `E0428`).  This file is the *redefinition* site for all of them: 38 `XCharStruct`, 45 `Emacs_Pixmap`, 47 `Emacs_Window`, 51 `Drawable`, 53 `Emacs_Cursor` and the 29 X-compat constants at 262-292, against `haikugui.rs`; `Emacs_Cursor` also collides with `dispextern.rs` 62 (C's platform `#if`s keep those apart - `include-androidterm.md` records the same class for its own `Emacs_Cursor` note).  The two X-compat *macros* do not error (two `macro_rules!` of one name shadow each other).  The resolution is the platform probe recorded below (ruled 2026-10-04: no `cfg`, no `pub mod` wrapper, `layout-check`'s single-crate model left alone); these names are the reason the platform mirrors cannot share that probe.

- **This mirror's compile evidence is the Android platform probe, not `xtask layout-check`**
  (`spikes/android_inc_probe.rs`, the `femacs/nt/inc`/`f_inc_probe.rs` shape, docs/status.md 3.102).
  `layout-check` compiles *every* `src/include` mirror into one crate, which by construction cannot
  hold the platform-exclusive mirrors: `androidgui.h` and `haikugui.h` declare the same X-compat
  block (`NativeRectangle`, `STORE_NATIVE_RECT`, the gravity/value/position constants) and the same
  `XCharStruct`/`Emacs_Pixmap`/`Emacs_Window`/`Emacs_Cursor`/`Drawable` aliases, and C includes only
  one of the two behind its platform `#if` (`HAVE_ANDROID` vs `HAVE_HAIKU`).  The probe compiles
  `androidgui.rs` and `android.rs` (the latter pulls `androidgui.rs`, `lisp.rs`, `config.rs` and the
  lib mirrors) in one flat namespace, the way an Android translation unit sees them.  **This is a
  probe-scope record, not a defect of the mirror**: no `#[cfg (feature = …)]` and no `pub mod`
  wrapper were added, for the reasons `include-haikugui.md` records.  Whether `layout-check` learns
  to skip the platform-exclusive mirrors is the integrator's call.
  Evidence: `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-mir-probe cargo run -q -p xtask --
  probe ../spikes/android_inc_probe.rs ../build/probes/android_inc_probe.rmeta --crate-type=lib
  --emit=metadata` -> exit 0; and on the materialised root,
  `rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type=lib --emit=metadata -o
  /tmp/android_inc_probe.rmeta build/probes/android_inc_probe/_root.rs` -> exit 0.
