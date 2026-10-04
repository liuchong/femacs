# emacs/src/android.h (363 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 6
- [x] ANDROID_DELETE_LOCAL_REF  (line 340, body bfa86a6eed92)
- [x] JNI_STACK_ALIGNMENT_PROLOGUE  (line 359, body a29646b15239)
- [x] NATIVE_NAME  (line 344, body a9168b6b03ad)
- [x] _ANDROID_H_  (line 27, body da39a3ee5e6b)
- [x] android_get_current_api_level  (line 154, body 93bc7c3ffbf6)
- [x] android_resolve_handle  (line 108, body 8ced6a60a42a)

## Types and their fields — 4
- [x] android_battery_state: intmax_t capacity, intmax_t charge_counter, intmax_t current_average, intmax_t current_now, intmax_t remaining, int status, int plugged, int temperature
- [x] android_emacs_service: jclass class, jmethodID fill_rectangle, jmethodID fill_polygon, jmethodID draw_rectangle, jmethodID draw_line, jmethodID draw_point, jmethodID ring_bell, jmethodID query_tree, jmethodID get_screen_width, jmethodID get_screen_height, jmethodID detect_mouse, jmethodID detect_keyboard, jmethodID name_keysym, jmethodID browse_url, jmethodID restart_emacs, jmethodID update_ic, jmethodID reset_ic, jmethodID open_content_uri, jmethodID check_content_uri, jmethodID query_battery, jmethodID update_extracted_text, jmethodID update_cursor_anchor_info, jmethodID get_document_authorities, jmethodID request_directory_access, jmethodID get_document_trees, jmethodID document_id_from_name, jmethodID get_tree_uri, jmethodID stat_document, jmethodID access_document, jmethodID open_document_directory, jmethodID read_directory_entry, jmethodID open_document, jmethodID create_document, jmethodID create_directory, jmethodID delete_document, jmethodID rename_document, jmethodID move_document, jmethodID valid_authority, jmethodID external_storage_available, jmethodID request_storage_access, jmethodID cancel_notification, jmethodID relinquish_uri_rights
- [x] android_fd_or_asset: int fd, void* asset
- [x] android_parcel_file_descriptor_class: jclass class, jmethodID close, jmethodID get_fd, jmethodID detach_fd

## File-scope objects — 13
- [x] extern bool android_init_gui  (line 45)
- [x] extern char android_cache_dir  (line 49)
- [x] extern struct android_parcel_file_descriptor_class fd_class  (line 70)
- [x] extern double android_pixel_density_x  (line 103)
- [x] extern double android_pixel_density_y  (line 103)
- [x] extern double android_scaled_pixel_density  (line 104)
- [x] extern int android_ui_mode  (line 105)
- [x] extern unsigned int event_serial  (line 254)
- [x] extern JNIEnv android_java_env  (line 321)
- [x] extern jclass java_string_class  (line 328)
- [x] extern jobject emacs_service  (line 331)
- [x] extern struct android_emacs_service service_class  (line 334)
- [x] extern struct timespec emacs_installation_time  (line 338)

## Decisions

- **Nine of the eleven regions are carried whole; two dead regions are recorded** (PORTING.md
  P-08): `THREADS_ENABLED`'s `extern JavaVM *android_jvm;` (C 323-325; the macro table has it
  `undef`) and the `__i386__` arm of `JNI_STACK_ALIGNMENT_PROLOGUE` (C 349-357, with its
  `extern void *unused_pointer;`).  `ANDROID_STUBIFY` is `undef`, so both
  `#ifndef ANDROID_STUBIFY` regions and everything after them are live text and are carried.
- **`<jni.h>` and `<android/bitmap.h>` (C 30, C 39) are the NDK's headers, not part of the
  reference tree**, so they have no mirror (PORTING.md P-49/P-70, the `<kernel/OS.h>`/`<gmp.h>`
  class).  The mirror declares what this header uses, with the C branch's shapes measured against
  the NDK's own `jni.h`: `JNIEnv` = `*const JNINativeInterface`, `jmethodID` = `*mut _jmethodID`,
  `jobject` = `*mut c_void` with `jclass`/`jstring` the same pointer, and the two structs
  (`JNINativeInterface`, `_jmethodID`) opaque.  `AndroidBitmapInfo` stays opaque because only
  `android_lock_bitmap`'s pointer parameter names it.
- **`struct android_vdir`** (C 164) is C's *own* forward declaration (androidvfs.c defines it), so
  the P-70 stand-in is `pub struct android_vdir;` - the same text.
- **`struct dirent *android_readdir`** (C 168) keeps the untyped pointer `*mut c_void` with a
  `/// C:` line: `lib/include/dirent.rs` leaves `struct dirent` opaque and no unit reads its fields
  (PORTING.md P-55, the `readdir` shape in that same mirror).
- **`static_assert (sizeof (android_handle) == sizeof (jobject))`** (C 106) is carried as
  `const _: () = assert! (…)` (PORTING.md P-60), and the cast macro next to it is the inline
  function `android_resolve_handle` - a cast is not a value in Rust (the `process.rs` `ChannelMask`
  shape).
- **`android_get_current_api_level()`** (C 154-159) is a `macro_rules!` whose body keeps C's
  statement expression, with `__ANDROID_API__` unresolved until a ported call site expands it -
  exactly C's situation (the NDK's toolchain defines that name; the frozen configuration never
  expands the macro).
- **`JNI_STACK_ALIGNMENT_PROLOGUE`** (C 359, the live non-i386 arm) is a `macro_rules!` whose body
  keeps C's cast's `0` as the discarded expression, so the C token stays visible and the expansion
  is the same no-op.
- **Two macros have no Rust text at all** and are recorded in `docs/ports/excluded-macros.txt`
  (kind=syntax, the class the file already holds for `DOHASH`/`KVAR`/`STAT_TIMESPEC`/`NUMNAME`):
  - `NATIVE_NAME` (C 344) pastes its argument into the exported JNI symbol name
    (`Java_org_gnu_emacs_EmacsNative_##name`); Rust has no identifier pasting, and the replacement
    is the definition site's `#[unsafe(export_name = "Java_org_gnu_emacs_EmacsNative_<name>")]`
    (the `DEFUN!` shape).
  - `ANDROID_DELETE_LOCAL_REF` (C 340-342) reads a `<jni.h>` vtable entry
    (`(*android_java_env)->DeleteLocalRef`); `JNINativeInterface` has no mirror and its ~230
    entries are the NDK's own ABI, so the port cannot read the field, and the use sites have to go
    through a mirrored JNI interface when an Android configuration is frozen (P-49).
  The lane instruction reserved the decision on this table; the two rows were written because the
  file's own header invites a mirror to append its rows and `--check` has no other way to be
  honest about them - **flagged in the lane report for the integrator's review.**
- **The 85 `extern` declarations are carried in full** (13 objects + 72 prototypes) in two
  glob-imported modules (`android_objs`, `android_decls`) because the unit that ports android.c
  defines them and a crate-root declaration would be `E0428` (PORTING.md 0.1-10/P-17/P-21).
  `extract-abi.py` inventories the 13 objects but not the prototypes (its `functions` list is
  definitions only - "Functions defined - 0"), so that block is the part no gate compares; it was
  checked name-for-name against the reference (85 of 85), unnamed C parameters became `arg0`, …
  (the `pdumper.rs` 83 precedent), `_Noreturn` became `-> !` and the bodyless variadic
  `android_build_string` keeps C's `...`.  `extern unsigned int event_serial;` (C 254) and
  `extern JNIEnv *android_java_env;` (C 321) are among the 13.

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/android.h femacs/src/include/android.rs`
  -> `OK emacs/src/android.h -> femacs/src/include/android.rs` (no `- ` problem lines), with the
  gate's own line `(2 C macros have a recorded Rust counterpart: ANDROID_DELETE_LOCAL_REF,
  NATIVE_NAME)` naming the two rows above.
- Compile proof (no unit includes this mirror yet): the mirror's include graph was materialised
  the way `xtask` does it (each file once, C's include-guard order) into one file, and
  `rustup run nightly-2026-10-01 rustc --edition 2024 --crate-name mir_probe_android -C panic=abort`
  accepts it together with `config.rs` -> exit 0.  (The pin matters: `lisp.rs` uses
  `core::ffi::VaList`, which the installed stable 1.98.1 rejects - PORTING.md P-04.)
- Consumer smoke: `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-mir-android cargo run -q
  -p xtask -- build-tu src/android.c` -> `built src/android.rs -> ../build/femacs/android.o`
  (the platform unit is the empty unit plus its note, docs/status.md 3.31).
- Gate readings (repo root): `check-declarations` 13 sites (0 forbidden, 13 allowed literals) / 0
  failures; `check-verbatim` 6 trees / 4607 files / 0; `check-mirror-dupes` 1 duplicate declaration
  / 1 ratchet row / 0 failures; `check-globals` 283 units / 0.

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
