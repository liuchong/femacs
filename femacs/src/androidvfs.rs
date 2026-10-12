// femacs: port of src/androidvfs.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// struct, struct field, macro, file-scope object, local), the same control flow and the same error
// model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C
// `static` helpers stay plain `unsafe fn` except the sixteen whose address C takes (the thirteen
// `unix_vfs_ops` members and the three `android_vdir` ones), which are `unsafe extern "C" fn`
// items - a function-pointer field cannot hold a Rust-ABI item (the androidmenu.rs /
// androidselect.rs shape).
//
// **This file is P-86's largest unit (C 7,950 lines) and is being ported in slices.**  This is
// slice 1: C 1-995, from the file's first line through `android_unix_opendir`'s closing brace.
// The next live definition is C 1016's `android_extract_long`, preceded by C 1005-1011's three
// file-scope objects (`directory_tree`, `directory_tree_size`, `asset_manager`) and the
// asset-directory comment block at C 999-1002.  The ledger `docs/ports/androidvfs.md` carries the
// C line-range table, the `## Evidence` readings and the `## Deviations`.
//
// The frozen configuration compiles no `androidvfs.o`: `configure.ac` 2647-2648 adds it to
// `ANDROID_OBJ` only in the really-Android branch (`XCONFIGURE = android`), while 2597's list
// (`androidmenu.o`) is in both, and 2605-2606 defines `ANDROID_STUBIFY` in the *other* branch, so
// this file's build condition is C's `HAVE_ANDROID && !defined ANDROID_STUBIFY` -
// `docs/baseline/frozen-build-sources.txt` lists no `androidvfs.o` for this target and
// `extract-abi.py`'s checklist for the file is empty.  Every item of this slice therefore sits
// under the pair `all (feature = "have_android", feature = "not_android_stubify")`, as does C 20-55's
// include graph, and that is why the frozen configuration compiles only `include/config.rs` and
// the `#![allow]` attribute here.
//
// The gates, in C's order:
//
//   * the file gate is the pair above (`configure.ac` 2647-2648, the androidselect.rs shape);
//   * C 46's `#if __ANDROID_API__ >= 9` / C 50's `#else` is this file's only other `#if`.  Its two
//     arms include two different headers and define `OLD_ANDROID_ASSETS` as 0 or 1.  The port
//     gates them on `android_api_ge_9`, a feature name this tree has *not* registered in
//     `femacs/xtask/Cargo.toml` (the table carries `android_api_lt_21`, `android_api_le_11` and
//     `android_api_le_30` only) - a port lane does not add feature names to that manifest, so the
//     ledger names it as owed by the integrator and `#[cfg]` on an unregistered name is simply
//     false, which is what makes the `>= 9` arm the one a build gets today.
//
// Two constructor-level facts this slice records rather than invents:
//
//   * **`struct android_vdir` cannot come from its mirror.**  C 40's `#include "android.h"` has
//     the mirror `include/android.rs`, which carries C 164's forward declaration of that struct
//     as the P-70 zero-sized stand-in (`pub struct android_vdir;`), while C's *definition* of the
//     same struct is this file's (C 114-126).  One translation unit cannot hold both: a
//     crate-root item and that stand-in are one `E0428` (measured), and P-70 says the stand-in
//     goes in the same commit as the definition, in that mirror.  That file is another lane's, so
//     this unit carries the one name it takes from android.h itself - the
//     `android_parcel_file_descriptor_class` record C 308's `fd_class` is an object of - in an
//     in-unit section at C 40's line (PORTING.md P-88 ① / P-72 rule ③), and the ledger records
//     the conflict and the owed removal of the stand-in.
//   * `<assert.h>` (C 23) carries no item: its gnulib template (`lib/include/assert.in.rs`) holds
//     only preprocessor text, and the units that need C's `assert` mirror the macro themselves
//     (the `include/android-asset.rs` / `lib-src/ebrowse.rs` shape) - which this file does at C
//     23's line, with the frozen `NDEBUG` body.  Every layout assertion below therefore spells
//     `core::assert!`, because that `macro_rules!` shadows the prelude's.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");                        // C 20: `#include <config.h>` (ungated)

/* C 21-55: the file's own include list.  None of it is reached by a translation unit the frozen
   configuration builds, so the whole graph sits on the file gate (see the header).  C 33's
   `<systime.h>` and C 42's `"systime.h"` are the same header reached twice - the include guard
   makes the second a no-op, which is what `xtask`'s per-file dedupe reproduces - so the single
   edge stands at C 29's line, where C reaches it first.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/fcntl.in.rs");         // C 21: `#include <fcntl.h>` (open, AT_FDCWD)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/unistd.in.rs");        // C 22: `#include <unistd.h>` (access, readlink, …)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/dirent.rs");           // C 25: `#include <dirent.h>` (DIR, opendir, …)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/errno.rs");            // C 26: `#include <errno.h>` (errno, EXDEV)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/minmax.rs");           // C 27: `#include <minmax.h>`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/string.rs");           // C 28: `#include <string.h>` (memcpy, memchr, …)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/systime.rs");                 // C 33: `#include <systime.h>` (src/systime.h)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/sys_types.in.rs");     // `mode_t`/`ssize_t`/`off_t` (the two above)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/stdlib.rs");           // C's `<stdlib.h>` via lisp.h (no name used here)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/sys_stat.in.rs");      // C 32: `#include <sys/stat.h>`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/stat-time.rs");        // C 35: `#include <stat-time.h>`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/md5.rs");              // C 36: `#include <md5.h>`
/* C 41's edge stands after android.h's chain, which is where C puts it and where
   `lisp.rs`'s `macro_rules!` must already be in scope for `androidterm.rs` and `coding.rs`
   (PORTING.md 0.1-9: macro scope is textual).  */
/* C 40: `#include "android.h"`.  The mirror is `include/android.rs`; this unit does not include it
   because the mirror's C 164 forward declaration of `struct android_vdir` is a zero-sized stand-in
   that C's *definition* of the same struct (this file's C 114-126) cannot coexist with in one
   translation unit (measured: `E0428`).  P-70 puts the stand-in's removal in the same commit as
   the definition's landing, in that mirror; the mirror is another lane's file, so this unit
   carries the android.h name its own text uses and the ledger owes the move (P-88 ①/P-72 ③).
   The name is exactly one: the record `fd_class` is an object of.  */
/* C 40's two `include!` edges stand at C's own line, before C 41's and before C 46-53's: the
   materializer marks a file as included at its *first* textual reach regardless of the `#[cfg]`
   on that edge (PORTING.md P-89), and C 51's `android-asset.h` mirror includes `jni.rs` itself -
   so the gated edge below would otherwise consume `jni.rs`'s single inclusion and leave this arm
   without the JNI types.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/jni.rs");                     // C 40's chain: `android.h` -> `<jni.h>`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/lisp.rs");                    // C 40's chain: `android.h` -> `lisp.h`

// C 58-64.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_parcel_file_descriptor_class
{
  pub class: jclass,
  pub close: jmethodID,
  pub get_fd: jmethodID,
  pub detach_fd: jmethodID,
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::size_of::<android_parcel_file_descriptor_class> () == 32);

/* C 46-53's `#if __ANDROID_API__ >= 9` chain.  Both arms are carried: the real one includes the
   NDK's `<android/asset_manager.h>`/`<android/asset_manager_jni.h>` (no mirror in this tree, so
   the opaque `AAsset` its two declarations and `struct android_vops.open` name is declared here,
   P-88 ①) and the old one includes `"android-asset.h"` (whose mirror is
   `include/android-asset.rs`).  `OLD_ANDROID_ASSETS` is a two-valued macro; C's uses of its value
   are C 1046-1278's (slice 2), and this slice carries the two definitions C 49/C 52 write.  */
/* The `>= 9` arm (C 47-48) reaches the two NDK headers directly; they have no mirror, and their
   entries are declared at their use sites in the slices that call them (only `AAsset` is named by
   C 1-995, below).  The `#else` arm (C 51) reaches the reference header `android-asset.h`, whose
   mirror is `include/android-asset.rs`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg (not (feature = "android_api_ge_9"))]
include!("include/android-asset.rs");           // C 51: `#include "android-asset.h"`

/* C's `<assert.h>` is one header read once per translation unit, so the port must give the name
   exactly one definition per TU at the crate root (PORTING.md 0.1-5).  C 51's `android-asset.h`
   mirror mirrors the same macro (its own `<assert.h>` read), and a `macro_rules!` that arrives
   through an `include!` is "macro-expanded" for name resolution: with no crate-root definition
   *after* it, every sibling mirror's `assert!` is `E0659` (measured).  This definition is the
   unit's own C 23, its body is `android-asset.rs` 47's and `lib-src/ebrowse.rs` 93's, and it
   stands after the C 51 edge in C's include order so the same-level later definition wins.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! assert {
  ($cond:expr) => { () };
}

/* C 49's `#define OLD_ANDROID_ASSETS 0` and C 52's `= 1`: C's macro is defined twice, once per
   arm, so the port keeps C's name with the arm's value (PORTING.md P-08).  C's uses of the value
   are C 1046-1278's; this slice carries the two definitions.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg (feature = "android_api_ge_9")]
pub const OLD_ANDROID_ASSETS: c_int = 0;                                     // C 49
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg (not (feature = "android_api_ge_9"))]
pub const OLD_ANDROID_ASSETS: c_int = 1;                                     // C 52

/* `struct AAsset`, the NDK's own opaque type (`<android/asset_manager.h>` 70-77:
   `struct AAsset;` / `typedef struct AAsset AAsset;` - never defined, so it is only ever a
   pointer, which is how this file names it).  Measured with the NDK's clang against the
   installed header: `sizeof (AAsset *)` is 8 for `--target=aarch64-linux-android24`.  */
/// C: struct AAsset;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg (feature = "android_api_ge_9")]
pub struct AAsset;

/* The JNI ABI this unit's text names comes from the NDK's `<jni.h>`, which C reaches through
   android.h 30 (`#include <jni.h>`); that mirror (`include/jni.rs`, the measured NDK ABI) is
   included here for the same reason - the android.h edge above is the one that cannot be carried
   (P-72 rule 4: a unit that needs the names must have the mirror in its own list).  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/androidterm.rs");             // C 41: `#include "androidterm.h"`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/blockinput.rs");              // C 43: `#include "blockinput.h"`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/coding.rs");                  // C 44: `#include "coding.h"`

/* C 24 `<dlfcn.h>`, C 30 `<semaphore.h>`, C 33 `<sys/mman.h>`, C 38 `<linux/ashmem.h>` and C 55
   `<android/log.h>` have no mirror in this tree, and no item of theirs is named by C 1-995: the
   four `dlsym` entries and the two `RTLD_*` values are C 1295/C 1364/C 1471's (slice 2), the
   `sem_*` calls are C 6508's, `mmap`/`munmap` are C 1330-1360's, and `__android_log_print` is
   first called at C 1181.  Each is declared at its own use site when a later slice reaches it
   (PORTING.md P-88 ①), and this slice carries no text for them.  */

/* C 23: `<assert.h>`'s `assert`.  No mirror carries the macro, and the frozen configuration
   defines `NDEBUG` (`macros.tsv` 2892), so C's `assert (cond)` expands to `((void) 0)` and its
   operand is never evaluated; the port keeps C's name with the same body (the
   `include/android-asset.rs` 47 and `lib-src/ebrowse.rs` 93 precedent), which is also why every
   `const _` assertion below spells `core::assert!` - this macro shadows the prelude's.  C 340,
   371 and 404 are the three uses in this slice.  */



/* `<string.h>`: the gnulib entry `lib/include/string.rs` does not carry (`HAVE_MEMRCHR` is
   undefined for this configuration).  C 522 is the one caller in this slice.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: `void *memrchr (const void *, int, size_t);`
  pub fn memrchr (s: *const c_void, c: c_int, n: size_t) -> *mut c_void;
}

/* `<string.h>`: the same gnulib entry, not in the mirror.  C 724, 739 and 783 name it.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: `void *mempcpy (void *, const void *, size_t);`
  pub fn mempcpy (dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
}

/* `<string.h>`: the mirror carries only the dead `rpl_strtok_r` rename
   (`string.in.rs` 264-270), so the system entry is declared here.  C 1080 (slice 2) is its
   only caller in the file.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: `char *strtok_r (char *, const char *, char **);`
  pub fn strtok_r (s: *mut c_char, delim: *const c_char,
                                   save_ptr: *mut *mut c_char) -> *mut c_char;
}

/* `<stdlib.h>`: C reaches it through lisp.h's own chain (the edge above), and
   `lib/include/stdlib.rs` carries `malloc` and friends but not `abort`.  C 605 is its one use in
   this slice.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: `_Noreturn void abort (void);`
  pub fn abort () -> !;
}

/* `<stdio.h>`: `lib/include/stdio.rs` does not carry `rename` (the `sysdep.rs` 766 shape).
   C 896 names it.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: `int rename (const char *, const char *);`
  pub fn rename (old: *const c_char, new: *const c_char) -> c_int;
}

/* lisp.h 5339's `renameat_noreplace`: the declaration `lisp.rs` does not carry, and whose
   definition is `sysdep.rs`'s in an Android configuration (`sysdep.c` 2376-2378's
   `#ifndef HAVE_ANDROID` makes it external there).  C 894 names it.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: `int renameat_noreplace (int, char const *, int, char const *);`
  pub fn renameat_noreplace (srcfd: c_int, src: *const c_char,
                                             dstfd: c_int, dst: *const c_char) -> c_int;
}

/* Structure describing an array of VFS operations.  */

/* C 110-112: the comment above and C's forward declaration `struct android_vnode;`.  Rust needs no
   forward declaration for a type defined later in the same unit (PORTING.md 10.10).  */

// C 114-126.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_vdir
{
  /* Return a `struct dirent' describing the next file in this
     directory stream, or NULL if the stream has reached its end.  */
  /// C: struct dirent *(*readdir) (struct android_vdir *); - `struct dirent` has no mirror
  /// (`lib/include/dirent.rs` keeps it opaque: bionic's is 280 bytes, measured, and no ported unit
  /// reads its fields), so the pointer keeps the untyped form (PORTING.md P-55).
  pub readdir: Option<unsafe extern "C" fn (*mut android_vdir) -> *mut c_void>,

  /* Close and release all resources allocated for this directory
     stream.  */
  pub closedir: Option<unsafe extern "C" fn (*mut android_vdir)>,

  /* Return a ``file descriptor'' tied to this directory stream.  */
  pub dirfd: Option<unsafe extern "C" fn (*mut android_vdir) -> c_int>,
}
/* Measured with the NDK's clang (`-Xclang -fdump-record-layouts`, `--target=aarch64-linux-android24`).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::size_of::<android_vdir> () == 24);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_vdir, readdir) == 0);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_vdir, closedir) == 8);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_vdir, dirfd) == 16);

// C 128-231.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_vops
{
  /* Name a child of the given VFS node, which should be a
     directory.

     LENGTH should be the length of NAME, excluding that of any
     trailing NULL byte.

     NAME should be a normalized and NULL-terminated relative file
     name; it may contain a leading separator characters, but no
     consecutive ones.

     If NAME is empty, create another VFS node designating the same
     file instead.

     NAME should also be located within writable storage; it may be
     overwritten as the vnode sees fit.

     Value is a VFS node corresponding to the child, or NULL upon
     failure.

     A VFS node may be returned even if NAME does not exist, the
     expectation being that either a later filesystem operation will
     fail, or will create the file.  */
  pub name: Option<unsafe extern "C" fn (*mut android_vnode, *mut c_char, size_t)
                                        -> *mut android_vnode>,

  /* Open the specified VNODE, returning either a file descriptor or
     an asset file descriptor.

     FLAGS and MODE mean the same as they do to the Unix `open' system
     call.

     ASSET_P stipulates if an asset file descriptor may be returned;
     if true, *ASSET may be set to an asset file descriptor.

     If an asset file descriptor is unavailable or ASSET_P is false,
     *FD will be set to a file descriptor.

     If the vnode cannot be opened, value is -1 with errno set
     accordingly.  Otherwise, value is 0 if a file descriptor was
     returned, and 1 if an asset file descriptor was returned.  */
  pub open: Option<unsafe extern "C" fn (*mut android_vnode, c_int, mode_t, bool, *mut c_int,
                                         *mut *mut AAsset) -> c_int>,

  /* Close the specified VNODE, releasing all of its resources.
     Save errno before making system calls that may set it, and
     restore it to its original value before returning.

     This is unrelated to `android_close', which primarily releases on
     stat buffers linked to file or asset file descriptors.  */
  pub close: Option<unsafe extern "C" fn (*mut android_vnode)>,

  /* Unlink the file and the specified VNODE.  Value and errno are the
     same as Unix `unlink'.  */
  pub unlink: Option<unsafe extern "C" fn (*mut android_vnode) -> c_int>,

  /* Create a symlink from the specified VNODE to the target TARGET.
     Value and errno are the same as `symlink' on Linux (which notably
     means that errno is set to EPERM if VNODE doesn't support
     symlinks.)  */
  pub symlink: Option<unsafe extern "C" fn (*const c_char, *mut android_vnode) -> c_int>,

  /* Remove VNODE from its parent directory.  VNODE must be an empty
     directory.  Value and errno are the same as Unix `rmdir'.  */
  pub rmdir: Option<unsafe extern "C" fn (*mut android_vnode) -> c_int>,

  /* Move the file designated by SRC to DST, overwriting DST if
     KEEP_EXISTING is false.

     If KEEP_EXISTING is true and DST already exists, value is -1 with
     errno set to EEXIST.

     If VNODE does not natively support checking for a preexisting DST
     and KEEP_EXISTING is true, value is -1 with errno set to ENOSYS.

     Value is otherwise the same as `rename'.  */
  pub rename: Option<unsafe extern "C" fn (*mut android_vnode, *mut android_vnode, bool)
                                           -> c_int>,

  /* Return statistics for the specified VNODE, and FLAGS, as in a call
     to `fstatat'.  Value and errno are the same as with Unix
     `stat'.  */
  pub stat: Option<unsafe extern "C" fn (*mut android_vnode, *mut stat, c_int) -> c_int>,

  /* Return whether or not VNODE is accessible.
     Value, errno and MODE are the same as with Unix `access'.  */
  pub access: Option<unsafe extern "C" fn (*mut android_vnode, c_int) -> c_int>,

  /* Make a directory designated by VNODE, like Unix `mkdir'.  */
  pub mkdir: Option<unsafe extern "C" fn (*mut android_vnode, mode_t) -> c_int>,

  /* Change the access mode of the provided VNODE to MODE.  Value is
     the same as with `chmod'.  FLAGS is passed verbatim from the call
     to the delegating at-func, and is probably
     AT_SYMLINK_NOFOLLOW.  */
  pub chmod: Option<unsafe extern "C" fn (*mut android_vnode, mode_t, c_int) -> c_int>,

  /* Return the target of VNODE if it is a symbolic link, or -1.
     Value and errno are the same as with `readlink'.  */
  pub readlink: Option<unsafe extern "C" fn (*mut android_vnode, *mut c_char, size_t) -> ssize_t>,

  /* Open the specified VNODE as a directory.
     Value is a ``directory handle'', or NULL upon failure.  */
  pub opendir: Option<unsafe extern "C" fn (*mut android_vnode) -> *mut android_vdir>,
}
/* Measured with the NDK's clang (`-Xclang -fdump-record-layouts`, `--target=aarch64-linux-android24`).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::size_of::<android_vops> () == 104);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_vops, name) == 0);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_vops, open) == 8);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_vops, opendir) == 96);

// C 233-240.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_vnode
{
  /* Operations associated with this vnode.  */
  pub ops: *mut android_vops,

  /* Type of this vnode and its flags.  */
  pub r#type: c_short,
  pub flags: c_short,
}
/* Measured with the NDK's clang (`-Xclang -fdump-record-layouts`, `--target=aarch64-linux-android24`).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::size_of::<android_vnode> () == 16);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_vnode, ops) == 0);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_vnode, r#type) == 8);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_vnode, flags) == 10);

/* Structure describing a special named vnode relative to the root
   vnode, or another directory vnode.  */

// C 245-260.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_special_vnode
{
  /* The name of the special file.  */
  pub name: *const c_char,

  /* The length of that name.  */
  pub length: size_t,

  /* Function called to create the initial vnode from the rest of the
     component.  */
  pub initial: Option<unsafe extern "C" fn (*mut c_char, size_t) -> *mut android_vnode>,

  /* If non-nil, an encoding system into which file name buffers are to
     be re-encoded before being handed to VFS functions.  */
  pub special_coding_system: Lisp_Object,
}
/* Measured with the NDK's clang (`-Xclang -fdump-record-layouts`, `--target=aarch64-linux-android24`;
   `Lisp_Object` stands in for `lisp.h`'s pointer typedef, 8 bytes).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::size_of::<android_special_vnode> () == 32);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_special_vnode, name) == 0);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_special_vnode, length) == 8);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_special_vnode, initial) == 16);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_special_vnode, special_coding_system)
                                           == 24);

/* C 262: `static_assert (NIL_IS_ZERO); /* special_coding_system above.  */` - C's
   `static_assert` becomes `const _: () = core::assert! (…)` (PORTING.md P-60), and
   `NIL_IS_ZERO` is `lisp.rs`'s (lisp.h 1999).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (NIL_IS_ZERO); /* special_coding_system above.  */

/* C 264-275: `enum android_vnode_type`.  C's enum ABI is `int` (PORTING.md 1.3), so the port
   writes the alias plus C's members with their implicit values (0, 1, … 8), the
   `dispextern.rs` `draw_glyphs_face` shape.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub type android_vnode_type = c_int;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_VNODE_UNIX: android_vnode_type = 0;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_VNODE_AFS: android_vnode_type = 1;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_VNODE_CONTENT: android_vnode_type = 2;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_VNODE_CONTENT_AUTHORITY: android_vnode_type = 3;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_VNODE_CONTENT_AUTHORITY_NAMED: android_vnode_type = 4;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_VNODE_SAF_ROOT: android_vnode_type = 5;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_VNODE_SAF_TREE: android_vnode_type = 6;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_VNODE_SAF_FILE: android_vnode_type = 7;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_VNODE_SAF_NEW: android_vnode_type = 8;

/* Structure describing the android.database.Cursor class.  */

// C 281-285.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_cursor_class
{
  pub class: jclass,
  pub close: jmethodID,
}

/* Structure describing the EmacsDirectoryEntry class.  */

// C 289-294.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct emacs_directory_entry_class
{
  pub class: jclass,
  pub d_type: jfieldID,
  pub d_name: jfieldID,
}

/* Measured with the NDK's clang (`-Xclang -fdump-record-layouts`, `--target=aarch64-linux-android24`;
   `jclass`/`jmethodID`/`jfieldID` are `jni.h`'s pointers).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::size_of::<android_cursor_class> () == 16);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_cursor_class, class) == 0);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_cursor_class, close) == 8);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::size_of::<emacs_directory_entry_class> () == 24);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (emacs_directory_entry_class, class) == 0);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (emacs_directory_entry_class, d_type) == 8);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (emacs_directory_entry_class, d_name) == 16);

/* The java.lang.String class.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub static mut java_string_class: jclass = std::ptr::null_mut ();             // C 297

/* Fields and methods associated with the Cursor class.  C's `static struct` is a tentative
   definition, i.e. every member zero (the androidmenu.rs `menu_class` shape).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut cursor_class: android_cursor_class = unsafe { std::mem::zeroed () };         // C 300

/* Fields and methods associated with the EmacsDirectoryEntry
   class.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut entry_class: emacs_directory_entry_class
  = unsafe { std::mem::zeroed () };                                           // C 304

/* Fields and methods associated with the ParcelFileDescriptor
   class.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub static mut fd_class: android_parcel_file_descriptor_class
  = unsafe { std::mem::zeroed () };                                           // C 308

/* Global references to several exception classes.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut file_not_found_exception: jclass = std::ptr::null_mut ();         // C 311
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut security_exception: jclass = std::ptr::null_mut ();               // C 311
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut operation_canceled_exception: jclass = std::ptr::null_mut ();     // C 312
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut unsupported_operation_exception: jclass = std::ptr::null_mut ();  // C 313
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut out_of_memory_error: jclass = std::ptr::null_mut ();              // C 313

/* C 315-317: the comment `/* Initialize `cursor_class' using the given JNI environment ENV.
   Calling this function is not necessary on Android 4.4 and earlier.  */`.  */

/* C 319: `static void android_init_cursor_class (JNIEnv *env)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_init_cursor_class (env: *mut JNIEnv)
{
  let mut old: jclass;

  /* C leaves `old` uninitialised (PORTING.md 1.5).  */
  old = std::ptr::null_mut ();

  cursor_class.class
    = ((*(*env)).FindClass.unwrap ()) (env, c"android/database/Cursor".as_ptr ());
  eassert! (! cursor_class.class.is_null ());

  old = cursor_class.class;
  cursor_class.class
    = ((*(*env)).NewGlobalRef.unwrap ()) (env, old as jobject);
  ((*(*env)).DeleteLocalRef.unwrap ()) (env, old);

  if cursor_class.class.is_null ()
    { emacs_abort (); }

  /* C 336-340: `FIND_METHOD`, the macro this function defines and `#undef`s at C 342.  Rust's
     `macro_rules!` scope ends with the block, so the `#undef` has no counterpart (the
     androidselect.rs 213-223 / android.rs 1979-1990 shape), and the macro's parameter names are
     C's so the body's tokens survive.  C 340's `assert` is this file's macro above.  */
  macro_rules! FIND_METHOD {
    ($c_name:ident, $name:expr, $signature:expr) => {
      cursor_class.$c_name
	= ((*(*env)).GetMethodID.unwrap ()) (env, cursor_class.class,
					     $name, $signature);
      assert! (unsafe { cursor_class.$c_name });
    };
  }

  FIND_METHOD! (close, c"close".as_ptr (), c"()V".as_ptr ());
}

/* C 345-347: the comment `/* Initialize `entry_class' using the given JNI environment ENV.
   Calling this function is not necessary on Android 4.4 and earlier.  */`.  */

/* C 349: `static void android_init_entry_class (JNIEnv *env)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_init_entry_class (env: *mut JNIEnv)
{
  let mut old: jclass;

  /* C leaves `old` uninitialised (PORTING.md 1.5).  */
  old = std::ptr::null_mut ();

  entry_class.class
    = ((*(*env)).FindClass.unwrap ()) (env, c"org/femacs/EmacsDirectoryEntry".as_ptr ());
  eassert! (! entry_class.class.is_null ());

  old = entry_class.class;
  entry_class.class
    = ((*(*env)).NewGlobalRef.unwrap ()) (env, old as jobject);
  ((*(*env)).DeleteLocalRef.unwrap ()) (env, old);

  if entry_class.class.is_null ()
    { emacs_abort (); }

  entry_class.d_type = ((*(*env)).GetFieldID.unwrap ()) (env, entry_class.class,
							  c"d_type".as_ptr (),
							  c"I".as_ptr ());
  entry_class.d_name = ((*(*env)).GetFieldID.unwrap ()) (env, entry_class.class,
							  c"d_name".as_ptr (),
							  c"Ljava/lang/String;".as_ptr ());
  assert! (unsafe { entry_class.d_type } != std::ptr::null_mut ()
	   && unsafe { entry_class.d_name } != std::ptr::null_mut ());
}

/* C 374-377: the comment `/* Initialize `fd_class' using the given JNI environment ENV.  Called
   on API 12 (Android 3.1) and later by androidselect.c and on 5.0 and later in this file.  */`.  */

/* C 379: `void android_init_fd_class (JNIEnv *env)` - non-`static` in C, so the port exports C's
   symbol (PORTING.md 0.2; `android.h` 72 declares it and androidselect.rs 623 calls it).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_init_fd_class (env: *mut JNIEnv)
{
  let mut old: jclass;
  static mut fd_class_initialized: bool = false;

  if fd_class_initialized
    { return; }

  fd_class.class
    = ((*(*env)).FindClass.unwrap ()) (env, c"android/os/ParcelFileDescriptor".as_ptr ());
  eassert! (! fd_class.class.is_null ());

  old = fd_class.class;
  fd_class.class
    = ((*(*env)).NewGlobalRef.unwrap ()) (env, old as jobject);
  ((*(*env)).DeleteLocalRef.unwrap ()) (env, old);

  if fd_class.class.is_null ()
    { emacs_abort (); }

  /* C 400-408: the same `FIND_METHOD` macro with three uses (C 405-407).  */
  macro_rules! FIND_METHOD {
    ($c_name:ident, $name:expr, $signature:expr) => {
      fd_class.$c_name
	= ((*(*env)).GetMethodID.unwrap ()) (env, fd_class.class,
					     $name, $signature);
      assert! (unsafe { fd_class.$c_name });
    };
  }

  FIND_METHOD! (close, c"close".as_ptr (), c"()V".as_ptr ());
  FIND_METHOD! (get_fd, c"getFd".as_ptr (), c"()I".as_ptr ());
  FIND_METHOD! (detach_fd, c"detachFd".as_ptr (), c"()I".as_ptr ());

  fd_class_initialized = true;
}

/* Account for SAF file names two times as large as PATH_MAX; larger
   values are prohibitively slow, but smaller values can't face up to
   some long file names within several nested layers of directories.

   Buffers holding components or other similar file name constituents
   which don't represent SAF files must continue to use PATH_MAX, for
   that is the restriction imposed by the Unix file system.  */

/* C 423: `#define EMACS_PATH_MAX (PATH_MAX * 2)`.  `PATH_MAX` is the frozen target's
   (`lib/include/limits.in.rs` 131: 1024), reached through lisp.h's own `<limits.h>`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const EMACS_PATH_MAX: c_int = PATH_MAX as c_int * 2;                     // C 423

/* Delete redundant instances of `.' and `..' from NAME in-place.
   NAME must be *LENGTH long, excluding a mandatory trailing NULL
   byte.

   Transform each directory component in NAME to avoid instances
   of the `.' and `..' directories.  For example, turn:

     a/../b/c/.

   into

     b/c/

   and return NULL, writing the new length of NAME into *LENGTH.

   If there are more `..' components in NAME than there are normal
   file name components, return NAME incremented to the position after
   the first `..' component that cannot be transformed.  For example,
   if NAME is

     a/../../a

   value will be

     a

   If NAME is a directory separator and LENGTH is 1, return without
   modifying NAME.  In any other case, omit any leading directory
   separator when writing to NAME.  This is useful when a vnode that
   can only be opened as a directory is desired, as this status is
   made clear by suffixing the file name with a trailing
   directory separator.  */

/* C 458: `static char *android_vfs_canonicalize_name (char *name, size_t *length)`.  C's two
   `goto parent_vnode` sites (C 511 and C 579) and the `parent_vnode:` label at C 605 become one
   labelled block (PORTING.md 2.2, the androidselect.rs `'fail:` shape): `goto parent_vnode` is
   `break 'parent_vnode`, and the block's end is where C's label body starts.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_vfs_canonicalize_name (mut name: *mut c_char, length: *mut size_t)
  -> *mut c_char
{
  /* C's `size_t nellipsis` is compared against `-1` and `-2` (C 476's comment, C 554, C 559,
     C 583, C 589, C 597): converting a negative constant to `size_t` gives SIZE_MAX and
     SIZE_MAX - 1, which is what the port spells explicitly so the two comparisons compile.  */
  let mut nellipsis: size_t;
  let mut i: size_t;
  let mut last_component: *mut c_char;
  let mut prec_component: *mut c_char;
  let mut fill: *mut c_char;
  let mut orig_name: *mut c_char;
  let mut size: size_t;

  'parent_vnode: {
    /* Special case described in the last paragraph of the comment
       above.  */

    size = *length;
    orig_name = name;

    if *name == b'/' as c_char && size == 1
      { return std::ptr::null_mut (); }
    else if *name == b'/' as c_char
      { size -= 1; }

    nellipsis = 0; /* Number of ellipsis encountered within the current
		      file name component, or -1.  */
    prec_component = std::ptr::null_mut (); /* Pointer to the separator character of the
					       component immediately preceding the
					       component currently being written.  */
    last_component = name; /* Pointer to the separator character of
			      the component currently being read.  */
    fill = name; /* Pointer to the next character that will be written
		    within NAME.  */

    /* Adjust name to skip the leading directory separator.  But only
       after fill is set.  */
    if *name == b'/' as c_char
      { name = name.offset (1); }

    i = 0;
    while i < size
      {
	/* C 493-561's `switch (name[i])` with its two labels and its `default`.  Rust patterns
	   admit no cast, so the port matches C's own promoted value (`char` promotes to `int`,
	   sign-extending: 47 is `'/'`, 46 is `'.'`) and names each label in a comment - the
	   android.rs 1511-1519 shape for `switch (*name)`.  */
	match *name.offset (i as isize) as c_int
	  {
	    /* C 495: `case '/':` */
	    47 =>
	      {
		/* See if the previous component was `..' or `.'.

		   If it is .., and if no previous directory separator was
		   encountered, return or look up a vnode representing the
		   parent.  */

		if nellipsis == 2
		  {
		    /* .. */

		    if prec_component.is_null ()
		      {
			/* Return the content of the component, i.e. the text
			   _after_ this separator.  */
			i += 1;
			break 'parent_vnode;
		      }

		    /* Return to the last component.  */
		    fill = prec_component;

		    /* Restore last_component to prec_component, and
		       prec_component back to the component before that.  */
		    last_component = prec_component;

		    if last_component != orig_name
		      { prec_component
			  = memrchr (orig_name as *const c_void, b'/' as c_int,
				     (last_component.offset_from (orig_name) - 1) as size_t)
			    as *mut c_char; }
		    else
		      { prec_component = std::ptr::null_mut (); }

		    /* prec_component may now be NULL.  If last_component is
		       identical to the initial value of NAME, then fill has
		       really been returned to the beginning of the string, so
		       leave it be.  But if it's something else, then it must
		       be the first separator character in the string, so set
		       prec_component to this initial value itself.  */

		    if prec_component.is_null () && last_component != orig_name
		      { prec_component = orig_name; }
		  }
		else if nellipsis == 1
		  /* If it's ., return to this component.  */
		  { fill = last_component; }
		else
		  {
		    /* Record the position of the last directory separator,
		       so NAME can be overwritten from there onwards if `..'
		       or `.' are encountered.  */
		    prec_component = last_component;
		    last_component = fill;
		  }

		/* Allow tracking ellipses again.  */
		nellipsis = 0;
	      }

	    /* C 553: `case '.':` */
	    46 =>
	      {
		if nellipsis != size_t::MAX
		  { nellipsis += 1; }
	      }

	    /* C 558: `default:` */
	    _ =>
	      {
		nellipsis = size_t::MAX;
	      }
	  }

	/* Now copy this character over from NAME.  */
	*fill = *name.offset (i as isize);
	fill = fill.offset (1);

	i += 1;
      }

    /* See if the previous component was `..' or `.'.

       If it is .., and if no previous directory separator was
       encountered, return or look up a vnode representing the
       parent.  */

    if nellipsis == 2
      {
	/* .. */

	if prec_component.is_null ()
	  /* Look up the rest of the vnode in its parent.  */
	  { break 'parent_vnode; }

	/* Return to the last component.  */
	fill = prec_component;
	nellipsis = size_t::MAX - 1;
      }
    else if nellipsis == 1
      {
	/* If it's ., return to this component.  */
	fill = last_component;
	nellipsis = size_t::MAX - 1;
      }

    /* Now, if there's enough room and an ellipsis file name was the
       last component of END, append a trailing `/' before NULL
       terminating it, indicating that the file name must be a
       directory.  */

    if fill.offset (1) < name.offset (size as isize) && nellipsis == size_t::MAX - 1
      { *fill = b'/' as c_char; fill = fill.offset (1); }

    /* NULL terminate fill.  */
    *fill = 0;
    *length = fill.offset_from (orig_name) as size_t;
    return std::ptr::null_mut ();
  }

  /* parent_vnode: */
  /* .. was encountered and the parent couldn't be found through
     stripping off preceding components.

     Find the parent vnode and name the rest of NAME starting from
     there.  */
  return name.offset (i as isize);
}

/* Unix vnode implementation.  These VFS nodes directly wrap around
   the Unix filesystem, with the exception of the root vnode.  */

// C 619-629.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_unix_vnode
{
  /* The vnode data itself.  */
  pub vnode: android_vnode,

  /* Length of the name without a trailing null byte.  */
  pub name_length: size_t,

  /* Name of the vnode.  */
  pub name: *mut c_char,
}

// C 631-638.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_unix_vdir
{
  /* The directory function table.  */
  pub vdir: android_vdir,

  /* The directory stream.  */
  pub directory: *mut DIR,
}

/* Measured with the NDK's clang (`-Xclang -fdump-record-layouts`, `--target=aarch64-linux-android24`).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::size_of::<android_unix_vnode> () == 32);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_unix_vnode, vnode) == 0);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_unix_vnode, name_length) == 16);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_unix_vnode, name) == 24);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::size_of::<android_unix_vdir> () == 32);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_unix_vdir, vdir) == 0);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_unix_vdir, directory) == 24);

/* The vnode representing the root filesystem.  C's `static struct` is a tentative definition,
   i.e. every member zero (the android.rs `pixmap_class` shape).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut root_vnode: android_unix_vnode = unsafe { std::mem::zeroed () };  // C 641

/* C 643-659: C's eleven forward declarations of this file's own `static` functions
   (`android_unix_name` … `android_unix_opendir`).  Rust needs no forward declaration for a
   function defined later in the same unit (PORTING.md 10.10), so this block has no text.  */

/* Vector of VFS operations associated with Unix filesystem VFS
   nodes.  */

/* C 664-679: `static struct android_vfs_ops unix_vfs_ops = { … }`.  C's initialiser is
   positional; Rust spells the same members in C's order with their names (the androidselect.rs
   `clipboard_class` shape).  C's eleven prototypes above are what this definition needs in C;
   Rust's item order is free.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut unix_vfs_ops: android_vops = android_vops
  {
    name: Some (android_unix_name),
    open: Some (android_unix_open),
    close: Some (android_unix_close),
    unlink: Some (android_unix_unlink),
    symlink: Some (android_unix_symlink),
    rmdir: Some (android_unix_rmdir),
    rename: Some (android_unix_rename),
    stat: Some (android_unix_stat),
    access: Some (android_unix_access),
    mkdir: Some (android_unix_mkdir),
    chmod: Some (android_unix_chmod),
    readlink: Some (android_unix_readlink),
    opendir: Some (android_unix_opendir),
  };

/* C 681: `static struct android_vnode *android_unix_name (struct android_vnode *vnode, char
   *name, size_t length)`.  C's `goto parent_vnode` (C 695) and the `parent_vnode:` label at
   C 745 become one labelled block (PORTING.md 2.2): the jump is `break 'parent_vnode` and the
   block's end is where C's label body starts.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_unix_name (mut vnode: *mut android_vnode, name: *mut c_char,
					mut length: size_t) -> *mut android_vnode
{
  let mut vp: *mut android_unix_vnode;
  let mut input: *mut android_unix_vnode;
  let mut temp: android_unix_vnode;
  let mut fill: *mut c_char;
  let mut remainder: *mut c_char;
  let mut j: size_t;

  /* C leaves `vp` and `temp` uninitialised (PORTING.md 1.5); `input`, `fill` and `remainder` are
     assigned before any use on every path.  */
  vp = std::ptr::null_mut ();
  temp = unsafe { std::mem::zeroed () };
  input = std::ptr::null_mut ();
  fill = std::ptr::null_mut ();
  remainder = std::ptr::null_mut ();
  j = 0;

  'parent_vnode: {
    /* Canonicalize NAME.  */
    input = vnode as *mut android_unix_vnode;
    remainder = android_vfs_canonicalize_name (name, &raw mut length);

    /* If remainder is set, it's a name relative to the parent vnode.  */
    if ! remainder.is_null ()
      { break 'parent_vnode; }

    /* Create a new unix vnode.  */
    vp = xmalloc (std::mem::size_of::<android_unix_vnode> ()) as *mut android_unix_vnode;

    /* If name is empty, duplicate the current vnode, but reset its file
       operation vector to that for Unix vnodes.  */

    if length < 1
      {
	memcpy (vp as *mut c_void, vnode as *const c_void,
		std::mem::size_of::<android_unix_vnode> ());
	(*vp).vnode.ops = &raw mut unix_vfs_ops;
	(*vp).name = xstrdup ((*vp).name);
	return &raw mut (*vp).vnode;
      }

    /* Otherwise, fill in the vnode.  C assigns the enum member to a `short` field, which
       truncates the `int`; the port spells that conversion (PORTING.md 1.3).  */

    (*vp).vnode.ops = &raw mut unix_vfs_ops;
    (*vp).vnode.r#type = ANDROID_VNODE_UNIX as c_short;
    (*vp).vnode.flags = 0;

    /* Generate the new name of the vnode.  Remove any trailing slash
       from vp->name.  */

    (*vp).name_length = (*input).name_length + length;
    (*vp).name = xmalloc ((*vp).name_length + 2) as *mut c_char;

    /* Copy the parent name over.  */
    fill = mempcpy ((*vp).name as *mut c_void, (*input).name as *const c_void,
		    (*input).name_length) as *mut c_char;

    /* Check if it contains a trailing slash.  input->name cannot be
       empty, as the root vnode's name is `/'.  */

    if *fill.offset (-1) != b'/' as c_char && *name != b'/' as c_char
      /* If not, append a trailing slash and adjust vp->name_length
	 correspondingly.  */
      { *fill = b'/' as c_char; fill = fill.offset (1); (*vp).name_length += 1; }
    else if *fill.offset (-1) == b'/' as c_char && *name == b'/' as c_char
      /* If name has a leading slash and fill does too, move fill
	 backwards so that name's slash will override that of fill.  */
      { fill = fill.offset (-1); (*vp).name_length -= 1; }

    /* Now copy NAME.  */
    fill = mempcpy (fill as *mut c_void, name as *const c_void, length) as *mut c_char;

    /* And NULL terminate fill.  */
    *fill = 0;
    return &raw mut (*vp).vnode;
  }

  /* parent_vnode: */
  /* .. was encountered and the parent couldn't be found through
     stripping off preceding components.

     Find the parent vnode and name the rest of NAME starting from
     there.  */

  if (*input).name_length == 1
    /* This is the vnode representing the root directory; just look
       within itself... */
    { vnode = &raw mut root_vnode.vnode; }
  else
    {
      /* Create a temporary unix vnode within the parent and use it
	 instead.  First, establish the length of vp->name before its
	 last component.  */

      j = (*input).name_length - 1;
      while j != 0
	{
	  if *(*input).name.offset ((j - 1) as isize) == b'/' as c_char
	    { break; }
	  j -= 1;
	}

      /* There must be at least one leading directory separator in an
	 asset vnode's `name' field.  */

      if j == 0
	{ abort (); }

      /* j is now the length of the string minus the size of its last
	 component.  Create a temporary vnode with that as its
	 name.  */

      temp.vnode.ops = &raw mut unix_vfs_ops;
      temp.vnode.r#type = ANDROID_VNODE_UNIX as c_short;
      temp.vnode.flags = 0;
      temp.name_length = j;
      temp.name = xmalloc (j + 1) as *mut c_char;
      fill = mempcpy (temp.name as *mut c_void, (*input).name as *const c_void,
		      j) as *mut c_char;
      *fill = 0;

      /* Search for the remainder of NAME relative to its parent.  */
      vnode = android_unix_name (&raw mut temp.vnode, remainder,
				 strlen (remainder));
      xfree (temp.name as *mut c_void);
      return vnode;
    }

  /* Virtual directories must be ignored in accessing the root directory
     through a Unix subdirectory of the root, as, `/../' */
  return android_unix_name (vnode, remainder, strlen (remainder));
}

/* Create a Unix vnode representing the given file NAME.  Use this
   function to create vnodes that aren't rooted in the root VFS
   node.  */

/* C 802-814.  C calls it directly (C 6928), never through a pointer, so it stays a Rust-ABI
   `unsafe fn` (the android.rs `android_run_debug_thread` rule).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_unix_vnode (name: *const c_char) -> *mut android_vnode
{
  let mut vp: *mut android_unix_vnode;

  /* C leaves `vp` uninitialised (PORTING.md 1.5).  */
  vp = std::ptr::null_mut ();

  vp = xmalloc (std::mem::size_of::<android_unix_vnode> ()) as *mut android_unix_vnode;
  (*vp).vnode.ops = &raw mut unix_vfs_ops;
  (*vp).vnode.r#type = ANDROID_VNODE_UNIX as c_short;
  (*vp).vnode.flags = 0;
  (*vp).name_length = strlen (name);
  (*vp).name = xstrdup (name);
  return &raw mut (*vp).vnode;
}

/* C 816: `static int android_unix_open (struct android_vnode *vnode, int flags, mode_t mode,
   bool asset_p, int *fd, AAsset **asset)`.  ASSET_P and ASSET are unused by this implementation
   (C's own body never reads them) but the signature is `struct android_vops.open`'s.  Its
   address is taken at C 667, so it is an `unsafe extern "C" fn`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_unix_open (vnode: *mut android_vnode, flags: c_int,
					mode: mode_t, asset_p: bool, fd: *mut c_int,
					asset: *mut *mut AAsset) -> c_int
{
  let mut vp: *mut android_unix_vnode;
  let mut fds: c_int;

  /* C leaves `vp` and `fds` uninitialised (PORTING.md 1.5).  */
  vp = std::ptr::null_mut ();
  fds = 0;

  vp = vnode as *mut android_unix_vnode;
  /* C's `open` is variadic and its `mode` argument is `mode_t` (an `unsigned short`) promoted
     to `int` by the default argument promotions; Rust requires the promoted type there.  */
  fds = open ((*vp).name, flags, mode as c_int);

  if fds < 0
    { return -1; }

  *fd = fds;
  return 0;
}

/* C 834-845.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_unix_close (vnode: *mut android_vnode)
{
  let mut vp: *mut android_unix_vnode;
  let mut save_errno: c_int;

  /* C leaves both uninitialised (PORTING.md 1.5).  */
  vp = std::ptr::null_mut ();
  save_errno = 0;

  save_errno = *errno ();
  vp = vnode as *mut android_unix_vnode;
  xfree ((*vp).name as *mut c_void);
  xfree (vp as *mut c_void);
  *errno () = save_errno;
}

/* C 847-854.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_unix_unlink (vnode: *mut android_vnode) -> c_int
{
  let mut vp: *mut android_unix_vnode;

  /* C leaves `vp` uninitialised (PORTING.md 1.5).  */
  vp = std::ptr::null_mut ();

  vp = vnode as *mut android_unix_vnode;
  return unlink ((*vp).name);
}

/* C 856-863.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_unix_symlink (target: *const c_char,
					   vnode: *mut android_vnode) -> c_int
{
  let mut vp: *mut android_unix_vnode;

  /* C leaves `vp` uninitialised (PORTING.md 1.5).  */
  vp = std::ptr::null_mut ();

  vp = vnode as *mut android_unix_vnode;
  return symlink (target, (*vp).name);
}

/* C 865-872.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_unix_rmdir (vnode: *mut android_vnode) -> c_int
{
  let mut vp: *mut android_unix_vnode;

  /* C leaves `vp` uninitialised (PORTING.md 1.5).  */
  vp = std::ptr::null_mut ();

  vp = vnode as *mut android_unix_vnode;
  return rmdir ((*vp).name);
}

/* C 874-897.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_unix_rename (src: *mut android_vnode, dst: *mut android_vnode,
					  keep_existing: bool) -> c_int
{
  let mut vp: *mut android_unix_vnode;
  let mut dest: *mut android_unix_vnode;

  /* C leaves both uninitialised (PORTING.md 1.5).  */
  vp = std::ptr::null_mut ();
  dest = std::ptr::null_mut ();

  if (*src).r#type != (*dst).r#type
    {
      /* If the types of both vnodes differ, complain that they're on
	 two different filesystems (which is correct from a abstract
	 viewpoint.)  */
      *errno () = EXDEV;
      return -1;
    }

  vp = src as *mut android_unix_vnode;
  dest = dst as *mut android_unix_vnode;

  return
    if keep_existing
      { renameat_noreplace (AT_FDCWD, (*vp).name, AT_FDCWD, (*dest).name) }
    else
      { rename ((*vp).name, (*dest).name) };
}

/* C 899-907.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_unix_stat (vnode: *mut android_vnode, statb: *mut stat,
					flags: c_int) -> c_int
{
  let mut vp: *mut android_unix_vnode;

  /* C leaves `vp` uninitialised (PORTING.md 1.5).  */
  vp = std::ptr::null_mut ();

  vp = vnode as *mut android_unix_vnode;
  return fstatat (AT_FDCWD, (*vp).name, statb, flags);
}

/* C 909-916.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_unix_access (vnode: *mut android_vnode, mode: c_int) -> c_int
{
  let mut vp: *mut android_unix_vnode;

  /* C leaves `vp` uninitialised (PORTING.md 1.5).  */
  vp = std::ptr::null_mut ();

  vp = vnode as *mut android_unix_vnode;
  return access ((*vp).name, mode);
}

/* C 918-925.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_unix_mkdir (vnode: *mut android_vnode, mode: mode_t) -> c_int
{
  let mut vp: *mut android_unix_vnode;

  /* C leaves `vp` uninitialised (PORTING.md 1.5).  */
  vp = std::ptr::null_mut ();

  vp = vnode as *mut android_unix_vnode;
  return mkdir ((*vp).name, mode);
}

/* C 927-935.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_unix_chmod (vnode: *mut android_vnode, mode: mode_t,
					 flags: c_int) -> c_int
{
  let mut vp: *mut android_unix_vnode;

  /* C leaves `vp` uninitialised (PORTING.md 1.5).  */
  vp = std::ptr::null_mut ();

  vp = vnode as *mut android_unix_vnode;
  return fchmodat (AT_FDCWD, (*vp).name, mode, flags);
}

/* C 937-945.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_unix_readlink (vnode: *mut android_vnode, buffer: *mut c_char,
					    size: size_t) -> ssize_t
{
  let mut vp: *mut android_unix_vnode;

  /* C leaves `vp` uninitialised (PORTING.md 1.5).  */
  vp = std::ptr::null_mut ();

  vp = vnode as *mut android_unix_vnode;
  return readlink ((*vp).name, buffer, size);
}

/* C 947-954.  The return type is `*mut c_void`: `struct dirent` has no mirror (see the
   `android_vdir.readdir` field).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_unix_readdir (vdir: *mut android_vdir) -> *mut c_void
{
  let mut dir: *mut android_unix_vdir;

  /* C leaves `dir` uninitialised (PORTING.md 1.5).  */
  dir = std::ptr::null_mut ();

  dir = vdir as *mut android_unix_vdir;
  return readdir ((*dir).directory);
}

/* C 956-964.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_unix_closedir (vdir: *mut android_vdir)
{
  let mut dir: *mut android_unix_vdir;

  /* C leaves `dir` uninitialised (PORTING.md 1.5).  */
  dir = std::ptr::null_mut ();

  dir = vdir as *mut android_unix_vdir;
  closedir ((*dir).directory);
  xfree (vdir as *mut c_void);
}

/* C 966-973.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_unix_dirfd (vdir: *mut android_vdir) -> c_int
{
  let mut dir: *mut android_unix_vdir;

  /* C leaves `dir` uninitialised (PORTING.md 1.5).  */
  dir = std::ptr::null_mut ();

  dir = vdir as *mut android_unix_vdir;
  return dirfd ((*dir).directory);
}

/* C 975-995.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_unix_opendir (vnode: *mut android_vnode) -> *mut android_vdir
{
  let mut vp: *mut android_unix_vnode;
  let mut dir: *mut android_unix_vdir;
  let mut directory: *mut DIR;

  /* C leaves all three uninitialised (PORTING.md 1.5).  */
  vp = std::ptr::null_mut ();
  dir = std::ptr::null_mut ();
  directory = std::ptr::null_mut ();

  /* Try to opendir the vnode.  */
  vp = vnode as *mut android_unix_vnode;
  directory = opendir ((*vp).name);

  if directory.is_null ()
    { return std::ptr::null_mut (); }

  dir = xmalloc (std::mem::size_of::<android_unix_vdir> ()) as *mut android_unix_vdir;
  (*dir).vdir.readdir = Some (android_unix_readdir);
  (*dir).vdir.closedir = Some (android_unix_closedir);
  (*dir).vdir.dirfd = Some (android_unix_dirfd);
  (*dir).directory = directory;
  return &raw mut (*dir).vdir;
}

/* C 1874-7950: the remainder of the file, plus the three items slice 7 defers (C 1682-1699's
   `afs_vfs_ops`, and C 1708-1843's `android_afs_name`/`android_afs_initial` - the object names
   twelve functions C defines at C 1873-2900 and Rust cannot forward-declare, while two of those
   functions name the object, so the three land together in the round that reaches C 1873).  */

/* Asset directory handling functions.  ``directory-tree'' is a file in
   the root of the assets directory describing its contents.

   See lib-src/asset-directory-tool for more details.  */               // C 999-1002

/* The Android directory tree.  */                                       // C 1004
/* C 1005's `static const char *directory_tree;` is a **tentative definition**, and the object it
   declares is the one C's own header already defines: `android-asset.h` 26 carries
   `static const char *directory_tree;` and this unit includes that header (C 51).  So the mirror
   owns the object (`include/android-asset.rs` 61, whose own comment says exactly that) and the
   port's rule of one spelling per construct keeps this line out of the unit - measured: a second
   definition here is `E0428 the name \`directory_tree\` is defined multiple times`.  C's own
   `directory_tree_size` (C 1008) and `asset_manager` (C 1011) are *not* in the header, so they are
   the unit's.  */

/* The size of the directory tree.  */                                   // C 1007
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut directory_tree_size: size_t = 0;                             // C 1008

/* C 47's `<android/asset_manager.h>` 70-77 spells the manager's opaque tag, which the `>= 9` arm
   needs and the `#else` arm gets from `android-asset.h`'s mirror instead (`include/android-asset.rs`
   104 has that arm's `AAssetManager`; this arm's C 47-48 headers have no mirror at all).  Measured:
   without it the `>= 9` arm's reading is `E0425 cannot find type \`AAssetManager\`` here - which is
   the gap the round's two-arm probing found.  */
/// C: struct AAssetManager;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg (feature = "android_api_ge_9")]
pub enum AAssetManager {}

/* The asset manager being used.  */                                     // C 1010
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut asset_manager: *mut AAssetManager = std::ptr::null_mut ();    // C 1011

/* Read an unaligned (32-bit) long from the address POINTER.  */         // C 1013

/* C 1013's comment is the function's contract: the asset tree stores 32-bit lengths unaligned, so
   the port keeps C's own `memcpy` rather than a `ptr::read_unaligned` (PORTING.md 3.3 binds the
   libc call instead of replacing it, and this unit's C 1048 call is the same shape), with
   `sizeof number` as `std::mem::size_of_val (&number)` - the `c_uint` local is what fixes the
   read's width, as C's own `sizeof` operand does.  */
// C 1015-1022.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_extract_long (pointer: *const c_char) -> c_uint
{
  let mut number: c_uint = 0;                                           // C 1018 (PORTING.md 1.5:
									//  its address is handed
									//  to C 1020)

  memcpy (&raw mut number as *mut c_void, pointer as *const c_void,     // C 1020
	  std::mem::size_of_val (&number));
  return number;                                                        // C 1021
}

/* Scan to the file FILE in the asset directory tree.  Return a
   pointer to the end of that file (immediately before any children)
   in the directory tree, or NULL if that file does not exist.

   If returning non-NULL, also return the offset to the end of the
   last subdirectory or file in *LIMIT_RETURN.  LIMIT_RETURN may be
   NULL.

   FILE must have less than 11 levels of nesting.  If it ends with a
   trailing slash, then NULL will be returned if it is not actually a
   directory.  */                                                       // C 1024-1034

/* `<android/log.h>`'s one entry this file calls (C 1170) and the priority it passes: the header has
   no mirror in this tree (slice 1's `<dlfcn.h>`… comment names it), and the number is the NDK's
   measured enum (`src/android.rs` 135-143 measured `ANDROID_LOG_WARN == 5` with this machine's
   NDK clang), so both stand at their use site (P-88 ①).  */
/// C: `int __android_log_print (int prio, const char *tag, const char *fmt, ...);` (log.h 112)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  pub fn __android_log_print (prio: c_int, tag: *const c_char, fmt: *const c_char, ...)
			      -> c_int;
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_LOG_WARN: c_int = 5;

/* C 1005's tentative definition, **for the `>= 9` arm only**: that arm's C 47-48 includes the
   NDK's asset-manager headers instead of `android-asset.h`, so the object the `#else` arm gets
   from the mirror (`include/android-asset.rs` 61) has no other definition there - in C the two
   lines are one object, and this is C's own structure, one definition per arm.  (The `#else`
   arm must not carry it: that would be `E0428`, measured in slice 2.)  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg (feature = "android_api_ge_9")]
static mut directory_tree: *const c_char = std::ptr::null ();

// C 1036-1202.  C's three labels are the port's three jumps (PORTING.md 2.2): `fail` (C 1199) is
// the forward cleanup path, so the body is a labelled block whose end falls into C's label text;
// `again` (C 1106) is a *backward* jump, so it is a labelled `loop` that repeats; and
// `false_positive` (C 1176) is the tail of one token's iteration, reached by the no-match
// fall-through and by C 1126's jump, so the matched path sits in a labelled block that
// `break 'matched` leaves.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_scan_directory_tree (file: *const c_char, limit_return: *mut size_t)
				       -> *const c_char
{
  let mut token: *mut c_char;                                           // C 1039
  let mut saveptr: *mut c_char;
  let mut copy: *mut c_char;
  let mut start: *mut c_char;
  let mut max: *mut c_char;
  let mut limit: *mut c_char;
  let mut token_length: size_t = 0;                                     // C 1040
  let mut ntokens: size_t = 0;
  let mut i: size_t = 0;
  let mut len: size_t = 0;
  let mut tokens: [*mut c_char; 20] = [std::ptr::null_mut (); 20];      // C 1041

  USE_SAFE_ALLOCA! (sa_avail, sa_count);                                // C 1043

  'fail: {
  /* Skip past the 5 or 9 byte header.  */                              // C 1045
  #[cfg (feature = "android_api_ge_9")]                                 // C 1046: C's `#if !OLD_ANDROID_ASSETS`
  { start = (directory_tree as *mut c_char).offset (5); }               // C 1047
  #[cfg (not (feature = "android_api_ge_9"))]                           // C 1048: C's `#else`
  { start = (directory_tree as *mut c_char).offset (9); }               // C 1049

  /* Figure out the current limit.  */                                  // C 1052
  limit = (directory_tree as *mut c_char).offset (directory_tree_size as isize);   // C 1053

  /* Now, split `file' into tokens, with the delimiter being the file
     name separator.  Look for the file and seek past it.  Create a copy
     of FILE for the enjoyment of `strtok_r'.  */                       // C 1055-1057

  ntokens = 0;                                                          // C 1059
  saveptr = std::ptr::null_mut ();                                      // C 1060
  len = strlen (file) + 1;                                              // C 1061
  copy = SAFE_ALLOCA! (sa_avail, len) as *mut c_char;                   // C 1062
  memcpy (copy as *mut c_void, file as *const c_void, len);             // C 1063
  memset (&raw mut tokens as *mut c_void, 0, std::mem::size_of_val (&tokens));   // C 1064

  while { token = strtok_r (copy, c"/".as_ptr (), &raw mut saveptr);    // C 1066
	  ! token.is_null () }
    {
      copy = std::ptr::null_mut ();                                     // C 1068

      /* Make sure ntokens is within bounds.  */                        // C 1070
      if ntokens == countof! (tokens)                                   // C 1071
	{ break 'fail; }                                                // C 1072

      len = strlen (token) + 1;                                         // C 1074
      tokens[ntokens as usize] = SAFE_ALLOCA! (sa_avail, len) as *mut c_char;   // C 1075
      memcpy (tokens[ntokens as usize] as *mut c_void, token as *const c_void, len);   // C 1076
      ntokens += 1;                                                     // C 1077
    }

  /* If there are no tokens, just return the start of the directory
     tree.  */                                                          // C 1080-1081

  if ntokens == 0                                                       // C 1083
    {
      SAFE_FREE! (sa_count);                                            // C 1085

      /* Return the size of the directory tree as the limit.
         Do not subtract the initial header bytes, as the limit
         is an offset from the start of the file.  */                  // C 1087-1089

      if ! limit_return.is_null ()                                      // C 1091
	{ *limit_return = directory_tree_size; }                        // C 1092

      return start;                                                     // C 1094
    }

  /* Loop through tokens, indexing the directory tree each time.  */    // C 1097

  i = 0;                                                                // C 1099 (the `for` init)
  while i < ntokens
    {
      /* C's `continue` (C 1146) jumps to the `for`'s increment, which runs at the body's end, so
	 the body is a labelled block and `break 'continue_;` leaves it - the `src/androidterm.rs`
	 275-290 shape.  */
      'continue_: {
      token = tokens[i as usize];                                       // C 1101

      /* Figure out how many bytes to compare.  */                      // C 1103
      token_length = strlen (token);                                    // C 1104

      'again: loop                                                      // C 1106
	{
	  /* If this would be past the directory, return NULL.  */      // C 1108
	  if start.offset (token_length as isize) > limit               // C 1109
	    { break 'fail; }                                            // C 1110

	  /* Now compare the file name.  */                             // C 1112
	  if memcmp (start as *const c_void, token as *const c_void, token_length) == 0   // C 1113
	    {
	      /* They probably match.  Find the NULL byte.  It must be
		 either one byte past start + token_length, with the last
		 byte a trailing slash (indicating that it is a directory),
		 or just start + token_length.  Return 4 or 8 bytes past the
		 next NULL byte.  */                                    // C 1115-1119

	      'matched: {
	      max = memchr (start as *const c_void, 0, limit.offset_from (start) as usize) as *mut c_char;   // C 1121

	      if max != start.offset (token_length as isize)            // C 1123
		  && ! (max == start.offset (token_length as isize + 1)   // C 1124
			&& *max.offset (-1) == b'/' as c_char)          // C 1125
		{ break 'matched; }                                     // C 1126

	      /* Return it if it exists and is in range, and this is the
		 last token.  Otherwise, set it as start and the limit as
		 start + the offset and continue the loop.  */          // C 1128-1130

	      if ! max.is_null ()                                       // C 1132
		  && max.offset ((if OLD_ANDROID_ASSETS != 0 { 9 } else { 5 }) as isize) <= limit
		{
		  if i < ntokens - 1                                    // C 1134
		    {
		      start = max.offset ((if OLD_ANDROID_ASSETS != 0 { 9 } else { 5 }) as isize);   // C 1136
		      limit = ((directory_tree as *mut c_char)           // C 1137
			       .offset (android_extract_long (max.offset ((if OLD_ANDROID_ASSETS != 0 { 5 } else { 1 }) as isize)) as isize));   // C 1138-1139

		      /* Make sure limit is still in range.  */         // C 1141
		      if limit > (directory_tree as *mut c_char).offset (directory_tree_size as isize)   // C 1142
			  || start > (directory_tree as *mut c_char).offset (directory_tree_size as isize)   // C 1143
			{ break 'fail; }                                // C 1144

		      break 'continue_;                                 // C 1146
		    }

		  /* Now see if max is not a directory and file is.  If
		     file is a directory, then return NULL.  */         // C 1149-1150
		  if *max.offset (-1) != b'/' as c_char                 // C 1151
		      && *file.offset (strlen (file) as isize - 1) == b'/' as c_char
		    { max = std::ptr::null_mut (); }                    // C 1152
		  else
		    {
		      /* Figure out the limit.  */                      // C 1155
		      if ! limit_return.is_null ()                      // C 1156
			{
			  *limit_return                                 // C 1157
			    = android_extract_long (max.offset ((if OLD_ANDROID_ASSETS != 0 { 5 } else { 1 }) as isize)) as size_t;   // C 1158-1159
			}

		      /* Go to the end of this file.  */                // C 1161
		      max = max.offset ((if OLD_ANDROID_ASSETS != 0 { 9 } else { 5 }) as isize);   // C 1162
		    }

		  SAFE_FREE! (sa_count);                                // C 1165
		  return max;                                           // C 1166
		}

	      /* Return NULL otherwise.  */                             // C 1169
	      let __func__: *const c_char = c"android_scan_directory_tree".as_ptr ();   // C 1170
	      __android_log_print (ANDROID_LOG_WARN, __func__,          // C 1170
				   c"could not scan to end of directory tree".as_ptr (),
				   c": %s".as_ptr (), file);            // C 1171-1172
	      break 'fail;                                              // C 1173
	      }
	    }

	  /* No match was found.  Set start to the next sibling and try
	     again.  */                                                 // C 1178-1179

	  start = memchr (start as *const c_void, 0, limit.offset_from (start) as usize) as *mut c_char;   // C 1181

	  if start.is_null ()                                           // C 1183
	      || start.offset ((if OLD_ANDROID_ASSETS != 0 { 9 } else { 5 }) as isize) > limit
	    { break 'fail; }                                            // C 1184

	  start = (directory_tree as *mut c_char)                       // C 1186
	    .offset (android_extract_long (start.offset ((if OLD_ANDROID_ASSETS != 0 { 5 } else { 1 }) as isize)) as isize);   // C 1187-1188

	  /* Make sure start is still in bounds.  */                    // C 1190

	  if start > limit                                              // C 1192
	    { break 'fail; }                                            // C 1193

	  /* Continue the loop.  */                                     // C 1195
	  /* C 1196's `goto again` is this `loop`'s own repeat.  */
	}
      }
      i += 1;                                                           // C 1099's `++i`
    }
  }

  /* C 1199's `fail:` */
  SAFE_FREE! (sa_count);                                                // C 1200
  return std::ptr::null ();                                             // C 1201
}

/* Return whether or not the directory tree entry DIR is a
   directory.

   DIR should be a value returned by
   `android_scan_directory_tree'.  */                                   // C 1204-1208

// C 1210-1229.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_is_directory (dir: *const c_char) -> bool
{
  /* If the directory is the directory tree, then it is a
     directory.  */                                                     // C 1213-1214
  if dir == (directory_tree as *const c_char)                            // C 1215
	     .offset ((if OLD_ANDROID_ASSETS != 0 { 9 } else { 5 }) as isize)
    { return true; }                                                    // C 1216

  /* Otherwise, look 5 bytes behind.  If it is `/', then it is a
     directory.  */                                                     // C 1219-1220
  #[cfg (feature = "android_api_ge_9")]                                  // C 1218: C's `#if !OLD_ANDROID_ASSETS`
  return dir.offset (-6) >= directory_tree as *const c_char              // C 1221
	 && *dir.offset (-6) == b'/' as c_char;                          // C 1222
  /* Otherwise, look 9 bytes behind.  If it is `/', then it is a
     directory.  */                                                     // C 1224-1225
  #[cfg (not (feature = "android_api_ge_9"))]                            // C 1223: C's `#else`
  return dir.offset (-10) >= directory_tree as *const c_char             // C 1226
	 && *dir.offset (-10) == b'/' as c_char;                         // C 1227
}

/* Initialize asset retrieval.  ENV should be a JNI environment for
   the Emacs thread, and MANAGER should be a local reference to a Java
   asset manager object created for the Emacs service context.  */     // C 1231-1233

/* The five `<android/asset_manager.h>` entries this function calls, **for the `>= 9` arm only**:
   that arm reaches the NDK's header directly (C 47-48) and it has no mirror in this tree, while the
   `#else` arm gets all five from C 51's `android-asset.h` mirror (`include/android-asset.rs`
   136/138/257/565/587 - that arm's emulation *defines* the four functions, this arm's header
   declares them).  Measured: the arm that lacks them reads `E0425` at each call (the slice-3
   lesson, applied before writing this time).  */
/* The four's return types are the NDK header's own; `off_t` is already in scope from C 32's
   `sys_types.in.rs` edge (`femacs/lib/include/sys_types.in.rs`), so it is not re-declared here
   (measured: a local alias is `E0428 the name \`off_t\` is defined multiple times`).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg (feature = "android_api_ge_9")]
unsafe extern "C" {
  /// C: `AAssetManager *AAssetManager_fromJava (JNIEnv *env, jobject assetManager);`
  pub fn AAssetManager_fromJava (env: *mut JNIEnv, java_manager: jobject) -> *mut AAssetManager;
  /// C: `AAsset *AAssetManager_open (AAssetManager *mgr, const char *filename, int mode);`
  pub fn AAssetManager_open (manager: *mut AAssetManager, c_name: *const c_char, mode: c_int)
			     -> *mut AAsset;
  /// C: `const void *AAsset_getBuffer (AAsset *asset);`
  pub fn AAsset_getBuffer (asset: *mut AAsset) -> *const c_void;
  /// C: `off_t AAsset_getLength (AAsset *asset);`
  pub fn AAsset_getLength (asset: *mut AAsset) -> off_t;
}
/// C 171-176's anonymous enum in `<android/asset_manager.h>`: `AASSET_MODE_BUFFER == 1` (the
/// `#else` arm's `include/android-asset.rs` 136 carries the same value; measured with this
/// machine's NDK clang, the `src/android.rs` 135 table's reading).
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg (feature = "android_api_ge_9")]
pub const AASSET_MODE_BUFFER: c_int = 1;
/// C 1249's priority: `<android/log.h>`'s `ANDROID_LOG_FATAL == 7` (the measured value
/// `src/android.rs` 141-143 records).
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_LOG_FATAL: c_int = 7;

// C 1235-1287.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_init_assets (env: *mut JNIEnv, manager: jobject)
{
  let mut asset: *mut AAsset;                                           // C 1238

  /* Set the asset manager.  */                                         // C 1240
  asset_manager = AAssetManager_fromJava (env, manager);                // C 1241

  /* Initialize the directory tree.  */                                 // C 1243
  asset = AAssetManager_open (asset_manager, c"directory-tree".as_ptr (),   // C 1244
			      AASSET_MODE_BUFFER);                      // C 1245

  if asset.is_null ()                                                   // C 1247
    {
      let __func__: *const c_char = c"android_init_assets".as_ptr ();    // C 1249
      __android_log_print (ANDROID_LOG_FATAL, __func__,                 // C 1249
			   c"Failed to open directory tree".as_ptr ());  // C 1250
      emacs_abort ();                                                   // C 1251
    }

  directory_tree = AAsset_getBuffer (asset) as *const c_char;            // C 1254

  if directory_tree.is_null ()                                          // C 1256
    { emacs_abort (); }                                                 // C 1257

  /* Now figure out how big the directory tree is, and compare the
     first few bytes.  */                                              // C 1259-1260
  directory_tree_size = AAsset_getLength (asset) as size_t;             // C 1261
  #[cfg (feature = "android_api_ge_9")]                                  // C 1262: C's `#if !OLD_ANDROID_ASSETS`
  {
    if directory_tree_size < 5                                          // C 1263
	|| memcmp (directory_tree as *const c_void, c"EMACS".as_ptr () as *const c_void, 5) != 0   // C 1264
      {
	let __func__: *const c_char = c"android_init_assets".as_ptr ();  // C 1266
	__android_log_print (ANDROID_LOG_FATAL, __func__,                // C 1266
			     c"Directory tree has bad magic".as_ptr ());  // C 1267
	emacs_abort ();                                                  // C 1268
      }
  }
  #[cfg (not (feature = "android_api_ge_9"))]                            // C 1270: C's `#else`
  {
    if directory_tree_size < 9                                           // C 1271
	|| memcmp (directory_tree as *const c_void, c"EMACS____".as_ptr () as *const c_void, 9) != 0   // C 1272
      {
	let __func__: *const c_char = c"android_init_assets".as_ptr ();  // C 1274
	__android_log_print (ANDROID_LOG_FATAL, __func__,                // C 1274
			     c"Directory tree has bad magic".as_ptr ());  // C 1275
	emacs_abort ();                                                  // C 1276
      }
  }

  /* Hold a VM reference to the asset manager to prevent the native
     object from being deleted.  */                                    // C 1280-1281
  ((*(*env)).NewGlobalRef.unwrap ()) (env, manager);                     // C 1282

  /* Abort if there's no more memory for the global reference.  */       // C 1284
  if ((*(*env)).ExceptionCheck.unwrap ()) (env) != 0                     // C 1285 (C's implicit
									//  bool test: the JNI entry returns
									//  `jboolean`, a `u8`)
    { abort (); }                                                       // C 1286
}

/* Asset-to-file descriptor conversion.  */                            // C 1291

/* Pointer to the `ASharedMemory_create' function which is loaded
   dynamically.  */                                                    // C 1293-1294
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut asharedmemory_create: Option<unsafe extern "C" fn (name: *const c_char,
							      size: size_t) -> c_int> = None;   // C 1295

/* The `<sys/mman.h>` and `<dlfcn.h>` names this function and the next use, none of which has a
   mirror in this tree (slice 1's head lists both headers as mirrorless): the two calls keep
   `src/pdumper.rs` 104-112's declarations, the three constants are the frozen target's
   (`MAP_FAILED` as `(void *) -1`, `src/pdumper.rs` 186; `PROT_WRITE` 0x02, `src/gmalloc.rs` 573,
   here the Linux/Android value the NDK's own `<sys/mman.h>` gives).  `android_cache_dir` is C's
   `android.h` 49 object: `include/android.rs` 227 carries it, but that mirror is not in this
   unit's include graph (its own head says which `android.h` edges this file takes), so the
   declaration stands here (P-72 rule 4).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: `void *mmap (void *, size_t, int, int, int, off_t);`
  pub fn mmap (addr: *mut c_void, len: size_t, prot: c_int, flags: c_int, fd: c_int,
	       offset: off_t) -> *mut c_void;
  /// C: `int munmap (void *, size_t);`
  pub fn munmap (addr: *mut c_void, len: size_t) -> c_int;
}
/// C: `<sys/mman.h>`: `#define PROT_WRITE 0x2`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const PROT_WRITE: c_int = 0x2;
/// C: `<sys/mman.h>`: `#define MAP_SHARED 0x01`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const MAP_SHARED: c_int = 0x01;
/// C: `<sys/mman.h>`: `#define MAP_FAILED ((void *) -1)`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const MAP_FAILED: *mut c_void = usize::MAX as *mut c_void;
/// C: android.h 49: `extern char *android_cache_dir;`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  pub static mut android_cache_dir: *mut c_char;
}
/// C 1344's priority: `<android/log.h>`'s `ANDROID_LOG_ERROR == 6` - C 1263/1273's `WARN`/`FATAL`
/// neighbours are already declared above, and all four values are the NDK enum this machine's
/// headers give (measured: `NDK 25.2.9519653`'s `android/log.h` 73-90 numbers `INFO` 4, `WARN` 5,
/// `ERROR` 6, `FATAL` 7).
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_LOG_ERROR: c_int = 6;
/* C 1340's `AAsset_read`: the `#else` arm's mirror defines it (with the emulation's own body,
   `include/android-asset.rs` 613), the `>= 9` arm's NDK header declares it (asset_manager.h 131,
   measured) - so the declaration stands under that arm, the slice-4 rule.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg (feature = "android_api_ge_9")]
unsafe extern "C" {
  /// C: `int AAsset_read (AAsset *asset, void *buf, size_t count);`
  pub fn AAsset_read (asset: *mut AAsset, buffer: *mut c_void, count: size_t) -> c_int;
}

/* Do the same as android_hack_asset_fd, but use an unlinked temporary
   file to cater to old Android kernels where ashmem files are not
   readable.  */                                                       // C 1297-1299

// C 1301-1355.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_hack_asset_fd_fallback (asset: *mut AAsset) -> c_int
{
  let mut fd: c_int;                                                    // C 1304
  let mut filename: [c_char; PATH_MAX as usize] = [0; PATH_MAX as usize];   // C 1305
  let mut size: size_t;                                                 // C 1306
  let mut mem: *mut c_void;                                             // C 1307

  'fail: {

  /* Assets must be small enough to fit in size_t, if off_t is
     larger.  */                                                        // C 1309-1310
  size = AAsset_getLength (asset) as size_t;                            // C 1311 (the mirror takes
									// C's `off_t` back as `c_long`, the NDK's
									// as `off_t`; both are 64-bit here)

  /* Get an unlinked file descriptor from a file in the cache
     directory, which is guaranteed to only be written to by Emacs.
     Creating an ashmem file descriptor and reading from it doesn't
     work on these old Android versions.  */                            // C 1313-1316

  snprintf (&raw mut filename as *mut c_char, PATH_MAX as usize,          // C 1318
	    c"%s/temp~unlinked.%d".as_ptr (),                            // C 1318
	    android_cache_dir, getpid ());                               // C 1319
  fd = open (&raw mut filename as *const c_char,                         // C 1320
	     O_CREAT | O_RDWR | O_TRUNC,                                 // C 1320
	     (S_IRUSR | S_IWUSR) as c_int);                               // C 1321 (C's own variadic
									//  promotion of `mode_t` to
									//  `int`, the C 1190 call's spelling)

  if fd < 0                                                             // C 1323
    { return -1; }                                                      // C 1324

  if unlink (&raw mut filename as *const c_char) != 0 && *errno () != ENOENT   // C 1326
    { break 'fail; }                                                     // C 1327

  if ftruncate (fd, size as off_t) != 0                                  // C 1329
    { break 'fail; }                                                     // C 1330

  mem = mmap (std::ptr::null_mut (), size, PROT_WRITE, MAP_SHARED, fd, 0);   // C 1332
  if mem == MAP_FAILED                                                   // C 1333
    {
      let __func__: *const c_char = c"android_hack_asset_fd_fallback".as_ptr ();   // C 1335
      __android_log_print (ANDROID_LOG_ERROR, __func__,                  // C 1335
			   c"mmap: %s".as_ptr (),                        // C 1336
			   strerror (*errno ()));                        // C 1336
      break 'fail;                                                       // C 1337
    }

  if AAsset_read (asset, mem, size) as i64 != size as i64                // C 1340
    {
      /* Too little was read.  Close the file descriptor and
	 report an error.  */                                           // C 1342-1343
      let __func__: *const c_char = c"android_hack_asset_fd_fallback".as_ptr ();   // C 1344
      __android_log_print (ANDROID_LOG_ERROR, __func__,                  // C 1344
			   c"AAsset_read: %s".as_ptr (),                 // C 1345
			   strerror (*errno ()));                        // C 1345
      break 'fail;                                                       // C 1346
    }

  munmap (mem, size);                                                   // C 1349
  return fd;                                                            // C 1350
  }

  /* C 1352's `fail:` */
  close (fd);                                                           // C 1353
  return -1;                                                            // C 1354
}

/* C 1363-1365's function needs five names no mirror this unit includes carries: the `<dlfcn.h>`
   pair C 1413/1567 names (whose two `RTLD_*` values the head's note already lists as this file's),
   the `<sys/ioctl.h>` entry C 1387/1395/1509/1520 calls, the two `<linux/ashmem.h>` constants it
   passes, and android.h 148's `android_get_current_api_level`, which C 1377 and C 1499 test.
   Each stands at its use site (P-88 rule 1), the `femacs/exec/trace.rs` 503-513 and
   `femacs/src/inotify.rs` 222 shapes.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: `<dlfcn.h>`: `void *dlsym (void *, const char *);`
  pub fn dlsym (handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
  /// C: `<sys/ioctl.h>`: `int ioctl (int, unsigned long, ...);`
  pub fn ioctl (fd: c_int, request: c_ulong, ...) -> c_int;
  /// C: android.h 148: `extern int android_get_current_api_level (void) __attribute__ ((pure));`
  pub fn android_get_current_api_level () -> c_int;
}
/// C: `<dlfcn.h>`: `#define RTLD_DEFAULT ((void *) 0)` - measured 0 in bionic's header with this
/// machine's NDK (the `exec/trace.rs` 508 precedent).
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const RTLD_DEFAULT: *mut c_void = std::ptr::null_mut ();
/// C: `<linux/ashmem.h>` 22: `#define ASHMEM_SET_NAME _IOW (__ASHMEMIOC, 1, char[ASHMEM_NAME_LEN])` -
/// measured **0x41007701** with this machine's NDK 29.0.14206865 (its `clang`,
/// `--target=aarch64-linux-android24`: `__ASHMEMIOC` is 0x77, `ASHMEM_NAME_LEN` 256, and `_IOW`
/// packs `1U << 30 | sizeof << 16 | type << 8 | nr`).
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ASHMEM_SET_NAME: c_ulong = 0x41007701;
/// C: `<linux/ashmem.h>` 24: `#define ASHMEM_SET_SIZE _IOW (__ASHMEMIOC, 3, size_t)` - measured
/// **0x40087703** (the same measurement; `size_t` is 8 bytes on the target it expands for).
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ASHMEM_SET_SIZE: c_ulong = 0x40087703;

/* Return whether or not shared memory file descriptors can also be
   read from, and are thus suitable for creating asset files.

   This does not work on some ancient Android systems running old
   versions of the kernel.  */                                        // C 1357-1361

// C 1363-1465.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_detect_ashmem () -> bool
{
  let mut fd: c_int;                                                   // C 1366
  let mut rc: c_int;                                                   // C 1366
  let mut mem: *mut c_void;                                            // C 1367
  let mut test_buffer: [c_char; 10] = [0; 10];                         // C 1368

  memcpy (&raw mut test_buffer as *mut c_void,                         // C 1370
	  c"abcdefghi".as_ptr () as *const c_void, 10);                // C 1370

  /* Create the file descriptor to be used for the test.  */            // C 1372

  /* Android 28 and earlier let Emacs access /dev/ashmem directly, so
     prefer that over using ASharedMemory.  */                         // C 1374-1375

  if android_get_current_api_level () <= 28                            // C 1377
    {
      fd = open (c"/dev/ashmem".as_ptr (), O_RDWR);                     // C 1379

      if fd < 0                                                        // C 1381
	{ return false; }                                              // C 1382

      /* An empty name means the memory area will exist until the file
	 descriptor is closed, because no other process can
	 attach.  */                                                   // C 1384-1386
      rc = ioctl (fd, ASHMEM_SET_NAME, c"".as_ptr ());                  // C 1387

      if rc < 0                                                        // C 1389
	{
	  close (fd);                                                  // C 1391
	  return false;                                                // C 1392
	}

      rc = ioctl (fd, ASHMEM_SET_SIZE,                                 // C 1395
		  std::mem::size_of_val (&test_buffer));               // C 1395

      if rc < 0                                                        // C 1397
	{
	  close (fd);                                                  // C 1399
	  return false;                                                // C 1400
	}
    }
  else
    {
      /* On the other hand, SELinux restrictions on Android 29 and
	 later require that Emacs use a system service to obtain
	 shared memory.  Load this dynamically, as this service is not
	 available on all versions of the NDK.  */                     // C 1405-1408

      if asharedmemory_create.is_none ()                               // C 1410
	{
	  *((&raw mut asharedmemory_create) as *mut *mut c_void)       // C 1412
	    = dlsym (RTLD_DEFAULT, c"ASharedMemory_create".as_ptr ()); // C 1413

	  if asharedmemory_create.is_none ()                           // C 1415
	    {
	      let __func__: *const c_char = c"android_detect_ashmem".as_ptr ();   // C 1417
	      __android_log_print (ANDROID_LOG_FATAL, __func__,        // C 1417
				   c"dlsym: %s\n".as_ptr (),           // C 1418
				   strerror (*errno ()));              // C 1419
	      emacs_abort ();                                          // C 1420
	    }
	}

      fd = (asharedmemory_create.unwrap ()) (c"".as_ptr (),            // C 1424
					     std::mem::size_of_val (&test_buffer));   // C 1424

      if fd < 0                                                        // C 1426
	{ return false; }                                              // C 1427
    }

  /* Now map the resource and write the test contents.  */              // C 1430

  mem = mmap (std::ptr::null_mut (), std::mem::size_of_val (&test_buffer), PROT_WRITE,
	      MAP_SHARED, fd, 0);                                      // C 1432-1433
  if mem == MAP_FAILED                                                 // C 1434
    {
      close (fd);                                                      // C 1436
      return false;                                                    // C 1437
    }

  /* Copy over the test contents.  */                                   // C 1440
  memcpy (mem, &raw const test_buffer as *const c_void,                // C 1442
	  std::mem::size_of_val (&test_buffer));                       // C 1441

  /* Return anyway even if munmap fails.  */                            // C 1443
  munmap (mem, std::mem::size_of_val (&test_buffer));                  // C 1444

  /* Try to read the content back into test_buffer.  If this does not
     compare equal to the original string, or the read fails, then
     ashmem descriptors are not readable on this system.  */           // C 1446-1448

  if (read (fd, &raw mut test_buffer as *mut c_void,                   // C 1452
	    std::mem::size_of_val (&test_buffer)) as i64               // C 1450
      != std::mem::size_of_val (&test_buffer) as i64                   // C 1451
      || memcmp (&raw const test_buffer as *const c_void,              // C 1452
		 c"abcdefghi".as_ptr () as *const c_void,              // C 1452
		 std::mem::size_of_val (&test_buffer)) != 0)           // C 1452
    {
      let __func__: *const c_char = c"android_detect_ashmem".as_ptr ();   // C 1454
      __android_log_print (ANDROID_LOG_WARN, __func__,                 // C 1454
			   concat! ("/dev/ashmem does not produce real",
				    " temporary files on this system, so",
				    " Emacs will fall back to creating",
				    " unlinked temporary files.", "\0").as_ptr () as *const c_char);   // C 1455-1458
      close (fd);                                                      // C 1459
      return false;                                                    // C 1460
    }

  close (fd);                                                          // C 1463
  return true;                                                         // C 1464
}

/* Get a file descriptor backed by a temporary in-memory file for the
   given asset.  */                                                    // C 1467-1468

// C 1470-1611.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_hack_asset_fd (asset: *mut AAsset) -> c_int
{
  /* C's `static bool` pair is zero-initialized in C, and a block-scoped `static mut` is where
     those two objects live here (the `include/android-asset.rs` 61 tentative-definition rule's
     other half: these two are the function's own, not a header's).  */
  static mut ashmem_readable_p: bool = false;                          // C 1473
  static mut ashmem_initialized: bool = false;                         // C 1474
  let mut fd: c_int;                                                   // C 1475
  let mut rc: c_int;                                                   // C 1475
  let mut mem: *mut c_uchar;                                           // C 1476
  let mut size: size_t;                                                // C 1477

  /* The first time this function is called, try to determine whether
     or not ashmem file descriptors can be read from.  */              // C 1479-1480

  if !ashmem_initialized                                                // C 1482
    { ashmem_readable_p = android_detect_ashmem (); }                  // C 1483-1484
  ashmem_initialized = true;                                            // C 1485

  /* If it isn't, fall back.  */                                        // C 1487

  if !ashmem_readable_p                                                 // C 1489
    { return android_hack_asset_fd_fallback (asset); }                  // C 1490

  /* Assets must be small enough to fit in size_t, if off_t is
     larger.  */                                                        // C 1492-1493
  size = AAsset_getLength (asset) as size_t;                            // C 1494

  /* Android 28 and earlier let Emacs access /dev/ashmem directly, so
     prefer that over using ASharedMemory.  */                         // C 1496-1497

  if android_get_current_api_level () <= 28                            // C 1499
    {
      fd = open (c"/dev/ashmem".as_ptr (), O_RDWR);                     // C 1501

      if fd < 0                                                        // C 1503
	{ return -1; }                                                 // C 1504

      /* An empty name means the memory area will exist until the file
	 descriptor is closed, because no other process can
	 attach.  */                                                   // C 1506-1508
      rc = ioctl (fd, ASHMEM_SET_NAME, c"".as_ptr ());                  // C 1509

      if rc < 0                                                        // C 1511
	{
	  let __func__: *const c_char = c"android_hack_asset_fd".as_ptr ();   // C 1513
	  __android_log_print (ANDROID_LOG_ERROR, __func__,              // C 1513
			       c"ioctl ASHMEM_SET_NAME: %s".as_ptr (),   // C 1514
			       strerror (*errno ()));                    // C 1515
	  close (fd);                                                    // C 1516
	  return -1;                                                     // C 1517
	}

      rc = ioctl (fd, ASHMEM_SET_SIZE, size);                            // C 1520

      if rc < 0                                                        // C 1522
	{
	  let __func__: *const c_char = c"android_hack_asset_fd".as_ptr ();   // C 1524
	  __android_log_print (ANDROID_LOG_ERROR, __func__,              // C 1524
			       c"ioctl ASHMEM_SET_SIZE: %s".as_ptr (),   // C 1525
			       strerror (*errno ()));                    // C 1526
	  close (fd);                                                    // C 1527
	  return -1;                                                     // C 1528
	}

      if size == 0                                                       // C 1531
	{ return fd; }                                                   // C 1532

      /* Now map the resource.  */                                      // C 1534
      mem = mmap (std::ptr::null_mut (), size, PROT_WRITE,              // C 1535
		  MAP_SHARED, fd, 0) as *mut c_uchar;                    // C 1535
      if mem as *mut c_void == MAP_FAILED                                // C 1536
	{
	  let __func__: *const c_char = c"android_hack_asset_fd".as_ptr ();   // C 1538
	  __android_log_print (ANDROID_LOG_ERROR, __func__,              // C 1538
			       c"mmap: %s".as_ptr (),                    // C 1539
			       strerror (*errno ()));                    // C 1539
	  close (fd);                                                    // C 1540
	  return -1;                                                     // C 1541
	}

      if AAsset_read (asset, mem as *mut c_void, size) as i64 != size as i64   // C 1544
	{
	  /* Too little was read.  Close the file descriptor and
	     report an error.  */                                        // C 1546-1547
	  let __func__: *const c_char = c"android_hack_asset_fd".as_ptr ();   // C 1548
	  __android_log_print (ANDROID_LOG_ERROR, __func__,              // C 1548
			       c"AAsset_read: %s".as_ptr (),             // C 1549
			       strerror (*errno ()));                    // C 1549
	  close (fd);                                                    // C 1550
	  return -1;                                                     // C 1551
	}

      /* Return anyway even if munmap fails.  */                         // C 1553
      munmap (mem as *mut c_void, size);                                 // C 1554
      return fd;                                                         // C 1555
    }

  /* On the other hand, SELinux restrictions on Android 29 and later
     require that Emacs use a system service to obtain shared memory.
     Load this dynamically, as this service is not available on all
     versions of the NDK.  */                                           // C 1557-1561

  if asharedmemory_create.is_none ()                                    // C 1563
    {
      *((&raw mut asharedmemory_create) as *mut *mut c_void)            // C 1566
	= dlsym (RTLD_DEFAULT, c"ASharedMemory_create".as_ptr ());      // C 1567

      if asharedmemory_create.is_none ()                                // C 1569
	{
	  let __func__: *const c_char = c"android_hack_asset_fd".as_ptr ();   // C 1571
	  __android_log_print (ANDROID_LOG_FATAL, __func__,                  // C 1571
			       c"dlsym: %s\n".as_ptr (),                        // C 1572
			       strerror (*errno ()));                           // C 1573
	  emacs_abort ();                                                    // C 1574
	}
    }

  fd = (asharedmemory_create.unwrap ()) (c"".as_ptr (), size);           // C 1578

  if fd < 0                                                              // C 1580
    {
      let __func__: *const c_char = c"android_hack_asset_fd".as_ptr ();   // C 1582
      __android_log_print (ANDROID_LOG_ERROR, __func__,                  // C 1582
			   c"ASharedMemory_create: %s".as_ptr (),         // C 1583
			   strerror (*errno ()));                        // C 1583
      return -1;                                                         // C 1584
    }

  /* Now map the resource.  */                                           // C 1587
  mem = mmap (std::ptr::null_mut (), size, PROT_WRITE, MAP_SHARED, fd, 0) as *mut c_uchar;   // C 1588
  if mem as *mut c_void == MAP_FAILED                                    // C 1589
    {
      let __func__: *const c_char = c"android_hack_asset_fd".as_ptr ();   // C 1591
      __android_log_print (ANDROID_LOG_ERROR, __func__,                  // C 1591
			   c"mmap: %s".as_ptr (),                           // C 1592
			   strerror (*errno ()));                           // C 1592
      close (fd);                                                        // C 1593
      return -1;                                                         // C 1594
    }

  if AAsset_read (asset, mem as *mut c_void, size) as i64 != size as i64  // C 1597
    {
      /* Too little was read.  Close the file descriptor and
	 report an error.  */                                            // C 1599-1600
      let __func__: *const c_char = c"android_hack_asset_fd".as_ptr ();   // C 1601
      __android_log_print (ANDROID_LOG_ERROR, __func__,                  // C 1601
			   c"AAsset_read: %s".as_ptr (),                 // C 1602
			   strerror (*errno ()));                        // C 1602
      close (fd);                                                        // C 1603
      return -1;                                                         // C 1604
    }

  /* Return anyway even if munmap fails.  */                             // C 1606
  munmap (mem as *mut c_void, size);                                     // C 1607
  return fd;                                                             // C 1608
}

/* ``Asset file system'' vnode implementation.  These vnodes map to
   asset files within the application package, provided by the Android
   ``asset manager''.  */                                              // C 1615-1617

/* C 1619-1629: `struct android_afs_vnode'.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_afs_vnode
{
  /* The vnode data itself.  */
  pub vnode: android_vnode,                                            // C 1622

  /* Length of the name without a trailing null byte.  */
  pub name_length: size_t,                                             // C 1625

  /* Name of the vnode.  */
  pub name: *mut c_char,                                               // C 1628
}
/* Measured with the NDK's clang (`-Xclang -fdump-record-layouts-complete`,
   `--target=aarch64-linux-android24`).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::size_of::<android_afs_vnode> () == 32);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_afs_vnode, vnode) == 0);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_afs_vnode, name_length) == 16);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_afs_vnode, name) == 24);

/* C 1631-1650: `struct android_afs_vdir'.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_afs_vdir
{
  /* The directory function table.  */
  pub vdir: android_vdir,                                              // C 1634

  /* The next directory stream in `all_afs_vdirs'.  */
  pub next: *mut android_afs_vdir,                                     // C 1637

  /* Pointer to the directory in directory_tree.  */
  pub asset_dir: *mut c_char,                                          // C 1640

  /* And the end of the files in asset_dir.  */
  pub asset_limit: *mut c_char,                                        // C 1643

  /* Path to the directory relative to /.  */
  pub asset_file: *mut c_char,                                         // C 1646

  /* File descriptor representing this directory stream, or NULL.  */
  pub fd: c_int,                                                       // C 1649
}
/* Measured with the NDK's clang (the three above).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::size_of::<android_afs_vdir> () == 64);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_afs_vdir, vdir) == 0);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_afs_vdir, next) == 24);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_afs_vdir, asset_dir) == 32);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_afs_vdir, asset_limit) == 40);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_afs_vdir, asset_file) == 48);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_afs_vdir, fd) == 56);

/* C 1652-1662: `struct android_afs_open_fd'.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_afs_open_fd
{
  /* The next table entry.  */
  pub next: *mut android_afs_open_fd,                                  // C 1655

  /* The open file descriptor.  */
  pub fd: c_int,                                                       // C 1658

  /* The stat buffer associated with this entry.  */
  pub statb: stat,                                                     // C 1661
}
/* Measured with the NDK's clang (the three above).  This struct embeds the *mirror's* `stat`,
   which models the frozen target: the NDK's own `struct stat` is 128 bytes on
   `aarch64-linux-android24` (measured), while `lib/include/sys_stat.in.rs` 108 asserts 144 and the
   port has only that one type - so the Rust layout here is 16 + 144 = 160, and the C-side number
   this file's fields come from is recorded in this note (the C 1471-1611 `AAsset_getLength`
   situation's shape: the field offsets the reference's `struct stat` gives are unchanged for the
   three members before it).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::size_of::<android_afs_open_fd> () == 160);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_afs_open_fd, next) == 0);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_afs_open_fd, fd) == 8);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_afs_open_fd, statb) == 16);

/* C 1664-1680: C's twelve forward declarations of this file's own `static` functions
   (`android_afs_name` … `android_afs_opendir`).  Rust needs no forward declaration for a function
   defined later in the same unit (PORTING.md 10.10), so this block has no text.  */

/* C 1682-1699 - `static struct android_vops afs_vfs_ops = { … }`, the twelve-entry operation
   vector - is **deferred to the round that reaches C 1873**, and the deferral is not a choice:
   the object names twelve functions C defines at C 1873-2900, Rust has no forward declaration
   (PORTING.md 10.10, the P-91 class), and two of the twelve (`android_afs_name` at C 1740,
   `android_afs_initial` at C 1834) name the object in return - so the object and the twelve
   functions are one dependency clique that has to land in a single round, with the object's text
   moved to that round's foot (the haikuterm slice-22 out-of-order device, its Deviation 34).
   C 1708-1843 (`android_afs_name`/`android_afs_initial`) is deferred with it for that reason.
   The ledger's C range table carries the three deferred rows.  */

/* Chain consisting of all open asset directory streams.  */             // C 1701
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut all_afs_vdirs: *mut android_afs_vdir = std::ptr::null_mut ();   // C 1702

/* List linking open file descriptors to asset information.  This
   assumes Emacs does not use dup on regular files.  */                 // C 1704-1705
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut afs_file_descriptors: *mut android_afs_open_fd = std::ptr::null_mut ();   // C 1706

/* Make FD close-on-exec.  If any system call fails, do not abort, but
   log a warning to the system log.  */                                // C 1845-1846

/* C 1848-1870.  C calls it directly (C 1952/3391), never through a pointer, so it stays a
   Rust-ABI `unsafe fn` (the `android_unix_vnode` rule).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_close_on_exec (fd: c_int)
{
  let mut flags: c_int;                                                // C 1851
  let mut rc: c_int;                                                   // C 1851

  flags = fcntl (fd, F_GETFD);                                         // C 1853

  if flags < 0                                                         // C 1855
    {
      let __func__: *const c_char = c"android_close_on_exec".as_ptr ();   // C 1857
      __android_log_print (ANDROID_LOG_WARN, __func__,                 // C 1857
			   c"fcntl: %s".as_ptr (),                     // C 1858
			   strerror (*errno ()));                      // C 1858
      return;                                                          // C 1859
    }

  rc = fcntl (fd, F_SETFD, flags | O_CLOEXEC);                         // C 1862

  if rc < 0                                                            // C 1864
    {
      let __func__: *const c_char = c"android_close_on_exec".as_ptr ();   // C 1866
      __android_log_print (ANDROID_LOG_WARN, __func__,                 // C 1866
			   c"fcntl: %s".as_ptr (),                     // C 1867
			   strerror (*errno ()));                      // C 1867
      return;                                                          // C 1868
    }
}
