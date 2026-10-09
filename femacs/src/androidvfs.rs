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

/* C 1231-7950: the remainder of the file.  Slice 3 ends at C 1229 (`android_is_directory`'s
   closing brace); the next live item is C 1236's `android_init_assets`, and `extract-abi.py
   --check`'s remaining `missing …` list is exactly that text.  */

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
