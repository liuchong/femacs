# emacs/src/android-asset.h (528 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 9
- [x] AAssetManager_fromJava  (line 92, static)
- [x] AAssetManager_open  (line 190, static)
- [x] AAsset_close  (line 291, static)
- [x] android_asset_create_stream  (line 329, static)
- [x] android_asset_read_internal  (line 375, static)
- [x] AAsset_getLength  (line 453, static)
- [x] AAsset_getBuffer  (line 470, static)
- [x] AAsset_read  (line 493, static)
- [x] AAsset_seek  (line 500, static)

## Macros defined — 0

## Types and their fields — 2
- [x] android_asset: AAssetManager* manager, jlong length, jlong bytes_read, jobject fd, jobject stream, jstring name, int mode
- [x] android_asset_manager: JNIEnv* env, jclass class, jmethodID open_fd, jmethodID open, jclass fd_class, jmethodID get_length, jmethodID create_input_stream, jmethodID close, jclass input_stream_class, jmethodID read, jmethodID stream_close, jobject asset_manager

## File-scope objects — 1
- [x] static const char directory_tree  (line 26, static)

## Decisions

- **The header is live text, not a platform dead arm.**  `tools/check-cond-eval.py
  src/android-asset.h` reports `0 conditions, 0 unknown, 0 disagreements [VERIFIED]`: the file has
  no `#if` of its own, so every declaration below is compared (PORTING.md P-08/P-66).  It is not
  built by the frozen configuration - `src/android.c` is an empty unit here (docs/ports/android.md)
  - but the mirror carries all of it, and `--check` is `OK` rather than a disposition record.
- **`ffi` and the JNI ABI come from the includer** (PORTING.md P-26/P-72 rule 4): `include!("jni.rs")`
  supplies `JNIEnv`, `jobject`/`jclass`/`jstring`/`jmethodID`, `jlong`/`jint`/`jbyte`, `jbyteArray`
  and the `JNINativeInterface` vtable the bodies call through.  That mirror is not a port of a
  reference file: it is the NDK's own `<jni.h>`, measured against the installed NDK with the NDK's
  clang (`spikes/jni-probe.sh`: 233 entries, the header's field order, and 234 `offset_of!`/
  `size_of` assertions confirmed for `aarch64-linux-android24`).
- **The two records are `#[repr(C)]` structs with the typedef as a `pub type`**: C 37-57
  `struct android_asset_manager` (12 fields) + C 59 `typedef struct android_asset_manager
  AAssetManager;` becomes `android_asset_manager` + `pub type AAssetManager = android_asset_manager;`
  and likewise `struct android_asset` (C 61-86, `jobject fd, stream;` is two members) + C 88
  `typedef struct android_asset AAsset;`.  Field names, order and types are C's, with the JNI
  typedefs from `jni.rs`.
- **The anonymous enum at C 171-176 is two constants**: C's `enum { AASSET_MODE_STREAMING = 0,
  AASSET_MODE_BUFFER = 1 };` has no C tag, so the mirror declares
  `pub const AASSET_MODE_STREAMING: c_int = 0;` and `..._BUFFER: c_int = 1;` (the PORTING.md 1.3
  unnamed-aggregate rule, the constant form).
- **`directory_tree` is a tentative definition, so it is a mutable null pointer**: C 26
  `static const char *directory_tree;` is written by the unit that includes this header, so the
  mirror is `pub static mut directory_tree: *const c_char = core::ptr::null ();` - `static` is C's
  internal linkage, so there is no `#[no_mangle]` (PORTING.md 0.2).
- **The two file-scope prototypes are a glob-imported module**: C 30-31 declares
  `android_scan_directory_tree` and `android_extract_long` `static`, and `androidvfs.c` defines
  them in the same translation unit, so `android_asset_decls` holds the `unsafe extern "C"`
  declarations and `use android_asset_decls::*;` brings them in; the unit's own definitions shadow
  the import (PORTING.md 0.1-10/P-17/P-21).  Neither needs a symbol.
- **The nine definitions are private `unsafe fn`**: every one is `static` in C, so there is no C
  symbol to carry and no `#[no_mangle]`; `unsafe` is what lets the bodies dereference the vtable
  and call `malloc`/`free`.
- **C's control words are carried as Rust block labels.**  Rust has no `goto`/`switch`/`case`/
  `default`, and the token rule compares a body's words, so the port names the blocks after C's own
  words: `'goto` for the region a `goto` leaves (its `break 'goto <value>` carries C's `return`
  value), `'fail`/`'out`/`'out_errno` for C's labels (each `break 'L` is the matching `goto`), and
  `'switch`/`'case`/`'default` for `AAsset_seek`'s dispatch, its shared guard tests and C's
  `default: break;`.  That is the same spelling `data.rs` uses for `store_symval_forwarding`
  (`'case`) and `keyboard.rs` for its switch, and it keeps every jump explicit rather than
  flattening the control flow.
- **Includes the file does not mirror.**  C 20 `#include <android/log.h>` is *not* mirrored:
  nothing in this file's own text names a `<android/log.h>` entry point (the `__android_log_print`
  calls live in `android.c` 358-379).  The libc surface the text does name comes from
  `stdlib.rs` (`malloc`/`calloc`/`free`), `errno.rs` (`errno`, `ENOMEM`, `EIO`), `limits.in.rs`
  (`INT_MAX`), `minmax.rs` (`MIN`), `unistd.in.rs` (`SEEK_SET`/`SEEK_CUR`/`SEEK_END`) and
  `sys_types.in.rs` (`off_t`).
- **`ESPIPE` is declared here, not in `errno.rs`.**  C 520 writes `errno = ESPIPE`; the port's
  `<errno.h>` slice (`lib/include/errno.rs`) carries the other errno values this file names
  but not that one, so the includer supplies it with the frozen target's number (the SDK's
  `sys/errno.h`: `#define ESPIPE 29`).  If `errno.rs` grows the constant this line is the one that
  goes - it is recorded here so the two do not become an `E0428`.
- **`errno` itself is a system-interface difference**: C's `errno` is the frozen target's macro
  (`errno.rs`'s `errno ()` -> `__error ()`), while a real Android build's `errno` is bionic's
  `__errno ()`.  Nothing in the header's text distinguishes the two, so the mirror keeps the
  frozen target's entry point; a configuration that freezes Android would swap the one function.
- **`assert` keeps C's name with the frozen `NDEBUG` body**: `NDEBUG` is defined for the frozen
  configuration, so C's `assert (cond)` expands to `((void) 0)` and does not evaluate its operand
  (PORTING.md 2.5); the mirror declares `macro_rules! assert { ($cond:expr) => { () }; }` (the
  `lib-src/ebrowse.rs` precedent) and its own layout assertions use `core::assert!` explicitly.
- **The unit is not compiled by the frozen configuration**, so the compile evidence is a dedicated
  probe rather than `build-tu`: `docs/ports/android.md` records that `src/android.c` is an empty
  unit (docs/status.md 3.31) and `xtask layout-check`'s single crate also cannot hold this mirror
  beside the platform-exclusive ones.  `spikes/android_asset_probe.rs` compiles it in one flat
  namespace with `config.rs`, `lisp.rs` and its own `include!` closure, the shape
  `spikes/f_inc_probe.rs` gave `nt/inc` (docs/status.md 3.102).
- **The probe's closure needs no extra mirror beyond the unit's own headers**: `lisp.rs` includes
  `comp.rs` (C's `lisp.h` 2168 `#include "comp.h"`), and `comp.rs` includes `dynlib.rs` (C's
  `comp.h` 23 `#include <dynlib.h>`), which is where `Lisp_Native_Comp_Unit`'s `dynlib_handle_ptr`
  comes from.  That chain is recorded because it was the one piece of the closure the probe needed
  while the mirrors were in flight (an earlier state had `comp.rs` name the typedef with no include
  of `dynlib.rs`, and any unit including `lisp.rs` then had to supply it, exactly as C would have
  to if `comp.h` did not include `<dynlib.h>`).

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/android-asset.h src/include/android-asset.rs`
  -> `OK emacs/src/android-asset.h -> src/include/android-asset.rs` (no `- ` problem lines).
- `python3 tools/check-cond-eval.py src/android-asset.h` ->
  `src/android-asset.h: 0 conditions, 0 unknown, 0 disagreements [VERIFIED]`.
- Compile proof: `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-mir-asset cargo run -q -p xtask --
  probe ../spikes/android_asset_probe.rs ../build/probes/android_asset_probe.rmeta
  --crate-type=lib --emit=metadata` -> exit 0 (the two `E0384`s this probe found - arguments the C
  body assigns - are what made `nbytes`/`buffer` `mut`, and the `E0133`s are the
  `unsafe_op_in_unsafe_fn` lint the other mirrors emit too).
- JNI ABI: `bash spikes/jni-probe.sh` -> `jni-probe: 233 entries, jni.h order matches the mirror` /
  `the NDK's clang confirms all 234 assertions (aarch64-linux-android24)` / `the mirror's
  offset_of!/size_of assertions hold (pinned toolchain, exit 0)`.
- Consumer smoke: `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-mir-asset cargo run -q -p xtask
  -- build-tu src/android.c` -> `built src/android.rs -> ../build/femacs/android.o` (the unit is the
  empty unit plus its note, docs/status.md 3.31; it does not include this mirror).
